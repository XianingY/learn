use crate::{error::MiniRedisError, frame::Frame, Result};
use std::io::Cursor;

pub fn check(src: &[u8]) -> Result<()> {
    let mut parser = Parser::new(src);
    parser.check_frame()
}

pub fn parse<'a>(src: &'a [u8]) -> Result<(Frame<'a>, usize)> {
    let mut parser = Parser::new(src);
    let frame = parser.parse_frame()?;
    Ok((frame, parser.cursor.position() as usize))
}

struct Parser<'a> {
    cursor: Cursor<&'a [u8]>,
}

enum Length {
    Null,
    Len(usize),
}

impl<'a> Parser<'a> {
    fn new(src: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(src),
        }
    }

    fn parse_frame(&mut self) -> Result<Frame<'a>> {
        match self.read_u8()? {
            b'+' => Ok(Frame::SimpleString(self.read_str_line()?)),
            b'-' => Ok(Frame::Error(self.read_str_line()?)),
            b':' => Ok(Frame::Integer(self.read_number()?)),
            b'$' => self.parse_bulk_string(),
            b'*' => self.parse_array(),
            byte => Err(MiniRedisError::Protocol(format!(
                "invalid frame type: {}",
                byte as char
            ))),
        }
    }

    fn check_frame(&mut self) -> Result<()> {
        match self.read_u8()? {
            b'+' | b'-' => {
                self.read_line()?;
                Ok(())
            }
            b':' => {
                self.read_number()?;
                Ok(())
            }
            b'$' => match self.read_length()? {
                Length::Null => Ok(()),
                Length::Len(len) => self.skip_bulk_bytes(len),
            },
            b'*' => match self.read_length()? {
                Length::Null => Ok(()),
                Length::Len(len) => {
                    for _ in 0..len {
                        self.check_frame()?;
                    }
                    Ok(())
                }
            },
            byte => Err(MiniRedisError::Protocol(format!(
                "invalid frame type: {}",
                byte as char
            ))),
        }
    }

    fn parse_bulk_string(&mut self) -> Result<Frame<'a>> {
        match self.read_length()? {
            Length::Null => Ok(Frame::BulkString(None)),
            Length::Len(len) => {
                let data = self.read_bulk_bytes(len)?;
                Ok(Frame::BulkString(Some(data)))
            }
        }
    }

    fn parse_array(&mut self) -> Result<Frame<'a>> {
        match self.read_length()? {
            Length::Null => Ok(Frame::Array(None)),
            Length::Len(len) => {
                let mut frames = Vec::with_capacity(len);
                for _ in 0..len {
                    frames.push(self.parse_frame()?);
                }
                Ok(Frame::Array(Some(frames)))
            }
        }
    }

    fn read_u8(&mut self) -> Result<u8> {
        let pos = self.cursor.position() as usize;
        let next = pos + 1;
        let byte = {
            let buf = self.cursor.get_ref();
            if pos >= buf.len() {
                return Err(MiniRedisError::Incomplete);
            }
            buf[pos]
        };
        self.cursor.set_position(next as u64);
        Ok(byte)
    }

    fn read_line(&mut self) -> Result<&'a [u8]> {
        let start = self.cursor.position() as usize;
        let end = {
            let buf = self.cursor.get_ref();
            if start >= buf.len() {
                return Err(MiniRedisError::Incomplete);
            }

            let mut idx = start;
            let mut end = None;
            while idx + 1 < buf.len() {
                if buf[idx] == b'\r' && buf[idx + 1] == b'\n' {
                    end = Some(idx);
                    break;
                }
                idx += 1;
            }

            end.ok_or(MiniRedisError::Incomplete)?
        };

        self.cursor.set_position((end + 2) as u64);
        let buf = self.cursor.get_ref();
        Ok(&buf[start..end])
    }

    fn read_str_line(&mut self) -> Result<&'a str> {
        let line = self.read_line()?;
        std::str::from_utf8(line).map_err(|_| MiniRedisError::Parse("invalid utf-8 in line".into()))
    }

    fn read_number(&mut self) -> Result<i64> {
        let line = self.read_line()?;
        let text = std::str::from_utf8(line)
            .map_err(|_| MiniRedisError::Parse("invalid utf-8 in number".into()))?;
        text.parse::<i64>()
            .map_err(|_| MiniRedisError::Parse(format!("invalid number: {}", text)))
    }

    fn read_length(&mut self) -> Result<Length> {
        let len = self.read_number()?;
        if len == -1 {
            return Ok(Length::Null);
        }
        if len < -1 {
            return Err(MiniRedisError::Parse("invalid length".into()));
        }

        let len = usize::try_from(len)
            .map_err(|_| MiniRedisError::Parse("length out of range".into()))?;
        Ok(Length::Len(len))
    }

    fn read_bulk_bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        let start = self.cursor.position() as usize;
        let end = start
            .checked_add(len)
            .ok_or_else(|| MiniRedisError::Parse("bulk length overflow".into()))?;
        let end_with_terminator = end
            .checked_add(2)
            .ok_or_else(|| MiniRedisError::Parse("bulk length overflow".into()))?;

        {
            let buf = self.cursor.get_ref();
            if buf.len() < end_with_terminator {
                return Err(MiniRedisError::Incomplete);
            }
            if buf[end] != b'\r' || buf[end + 1] != b'\n' {
                return Err(MiniRedisError::Parse(
                    "bulk string missing terminator".into(),
                ));
            }
        }

        self.cursor.set_position(end_with_terminator as u64);
        let buf = self.cursor.get_ref();
        Ok(&buf[start..end])
    }

    fn skip_bulk_bytes(&mut self, len: usize) -> Result<()> {
        let _ = self.read_bulk_bytes(len)?;
        Ok(())
    }
}
