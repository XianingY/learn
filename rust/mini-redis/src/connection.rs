use crate::{Result, MiniRedisError, frame::FrameOwned, parse};
use bytes::{BytesMut, Buf};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4096),
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<FrameOwned>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            if self.stream.read_buf(&mut self.buffer).await? == 0 {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(MiniRedisError::ConnectionReset);
                }
            }
        }
    }

    fn parse_frame(&mut self) -> Result<Option<FrameOwned>> {
        let (frame, consumed) = {
            let buf = &self.buffer[..];
            match parse::check(buf) {
                Ok(_) => {
                    let (frame, consumed) = parse::parse(buf)?;
                    (Some(FrameOwned::from(frame)), consumed)
                }
                Err(MiniRedisError::Incomplete) => (None, 0),
                Err(e) => return Err(e),
            }
        };

        if let Some(frame) = frame {
            self.buffer.advance(consumed);
            return Ok(Some(frame));
        }

        Ok(None)
    }

    pub async fn write_frame(&mut self, frame: &FrameOwned) -> Result<()> {
        match frame {
            FrameOwned::SimpleString(s) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(s.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            FrameOwned::Error(s) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(s.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            FrameOwned::Integer(i) => {
                self.stream.write_u8(b':').await?;
                self.stream.write_all(i.to_string().as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            FrameOwned::BulkString(None) => {
                self.stream.write_all(b"$-1\r\n").await?;
            }
            FrameOwned::BulkString(Some(data)) => {
                self.stream.write_u8(b'$').await?;
                self.stream.write_all(data.len().to_string().as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
                self.stream.write_all(data).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            FrameOwned::Array(None) => {
                self.stream.write_all(b"*-1\r\n").await?;
            }
            FrameOwned::Array(Some(frames)) => {
                self.stream.write_u8(b'*').await?;
                self.stream.write_all(frames.len().to_string().as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
                for frame in frames {
                    Box::pin(self.write_frame(frame)).await?; 
                }
            }
        }
        self.stream.flush().await?;
        Ok(())
    }
}
