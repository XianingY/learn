use crate::{
    db::Db, 
    error::MiniRedisError, 
    Result, 
    frame::{Frame, FrameOwned}, 
    connection::Connection
};
use bytes::Bytes;
use std::time::Duration;

#[derive(Debug)]
pub enum Command {
    Get(String),
    Set(String, Bytes, Option<Duration>),
    Publish(String, Bytes),
    Subscribe(Vec<String>),
    Ping(Option<String>),
}

impl Command {
    pub fn from_frame(frame: Frame<'_>) -> Result<Self> {
        match frame {
            Frame::Array(Some(frames)) => {
                let mut it = frames.into_iter();
                let cmd_name = match it.next() {
                    Some(Frame::BulkString(Some(bytes))) => {
                        std::str::from_utf8(bytes).map_err(|_| MiniRedisError::Protocol("invalid command name".into()))?.to_lowercase()
                    }
                    _ => return Err(MiniRedisError::Protocol("invalid command format".into())),
                };

                match cmd_name.as_str() {
                    "get" => {
                        let key = it.next().and_then(|f| match f {
                            Frame::BulkString(Some(b)) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                            _ => None,
                        }).ok_or_else(|| MiniRedisError::Protocol("GET missing key".into()))?;
                        Ok(Command::Get(key))
                    }
                    "set" => {
                        let key = it.next().and_then(|f| match f {
                            Frame::BulkString(Some(b)) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                            _ => None,
                        }).ok_or_else(|| MiniRedisError::Protocol("SET missing key".into()))?;
                        let val = it.next().and_then(|f| match f {
                            Frame::BulkString(Some(b)) => Some(Bytes::copy_from_slice(b)),
                            _ => None,
                        }).ok_or_else(|| MiniRedisError::Protocol("SET missing value".into()))?;
                        
                        let mut expire = None;
                        if let Some(Frame::BulkString(Some(opt))) = it.next() {
                            if std::str::from_utf8(opt).ok().map(|s| s.to_lowercase()) == Some("ex".into()) {
                                if let Some(Frame::BulkString(Some(secs_bytes))) = it.next() {
                                    if let Ok(secs) = std::str::from_utf8(secs_bytes).unwrap_or("").parse::<u64>() {
                                        expire = Some(Duration::from_secs(secs));
                                    }
                                }
                            }
                        }

                        Ok(Command::Set(key, val, expire))
                    }
                    "publish" => {
                        let channel = it.next().and_then(|f| match f {
                            Frame::BulkString(Some(b)) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                            _ => None,
                        }).ok_or_else(|| MiniRedisError::Protocol("PUBLISH missing channel".into()))?;
                        let msg = it.next().and_then(|f| match f {
                            Frame::BulkString(Some(b)) => Some(Bytes::copy_from_slice(b)),
                            _ => None,
                        }).ok_or_else(|| MiniRedisError::Protocol("PUBLISH missing message".into()))?;
                        Ok(Command::Publish(channel, msg))
                    }
                    "subscribe" => {
                        let mut channels = Vec::new();
                        for frame in it {
                            if let Frame::BulkString(Some(b)) = frame {
                                channels.push(std::str::from_utf8(b).map_err(|_| MiniRedisError::Protocol("invalid channel name".into()))?.to_string());
                            }
                        }
                        if channels.is_empty() {
                            return Err(MiniRedisError::Protocol("SUBSCRIBE missing channels".into()));
                        }
                        Ok(Command::Subscribe(channels))
                    }
                    "ping" => {
                        let msg = it.next().and_then(|f| match f {
                            Frame::BulkString(Some(b)) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                            Frame::SimpleString(s) => Some(s.to_string()),
                            _ => None,
                        });
                        Ok(Command::Ping(msg))
                    }
                    _ => Err(MiniRedisError::Protocol(format!("unknown command: {}", cmd_name))),
                }
            }
            _ => Err(MiniRedisError::Protocol("expected array for command".into())),
        }
    }

    pub fn from_frame_owned(frame: FrameOwned) -> Result<Self> {
        match frame {
            FrameOwned::Array(Some(frames)) => {
                let mut it = frames.into_iter();
                let cmd_name = match it.next() {
                    Some(FrameOwned::BulkString(Some(bytes))) => {
                        std::str::from_utf8(&bytes).map_err(|_| MiniRedisError::Protocol("invalid command name".into()))?.to_lowercase()
                    }
                    _ => return Err(MiniRedisError::Protocol("invalid command format".into())),
                };

                match cmd_name.as_str() {
                    "get" => {
                        let key = it.next().and_then(|f| match f {
                            FrameOwned::BulkString(Some(b)) => std::str::from_utf8(&b).ok().map(|s| s.to_string()),
                            _ => None,
                        }).ok_or_else(|| MiniRedisError::Protocol("GET missing key".into()))?;
                        Ok(Command::Get(key))
                    }
                    "set" => {
                        let key = it.next().and_then(|f| match f {
                            FrameOwned::BulkString(Some(b)) => std::str::from_utf8(&b).ok().map(|s| s.to_string()),
                            _ => None,
                        }).ok_or_else(|| MiniRedisError::Protocol("SET missing key".into()))?;
                        let val = it.next().and_then(|f| match f {
                            FrameOwned::BulkString(Some(b)) => Some(b),
                            _ => None,
                        }).ok_or_else(|| MiniRedisError::Protocol("SET missing value".into()))?;
                        
                        let mut expire = None;
                        if let Some(FrameOwned::BulkString(Some(opt))) = it.next() {
                            if std::str::from_utf8(&opt).ok().map(|s| s.to_lowercase()) == Some("ex".into()) {
                                if let Some(FrameOwned::BulkString(Some(secs_bytes))) = it.next() {
                                    if let Ok(secs) = std::str::from_utf8(&secs_bytes).unwrap_or("").parse::<u64>() {
                                        expire = Some(Duration::from_secs(secs));
                                    }
                                }
                            }
                        }

                        Ok(Command::Set(key, val, expire))
                    }
                    "publish" => {
                        let channel = it.next().and_then(|f| match f {
                            FrameOwned::BulkString(Some(b)) => std::str::from_utf8(&b).ok().map(|s| s.to_string()),
                            _ => None,
                        }).ok_or_else(|| MiniRedisError::Protocol("PUBLISH missing channel".into()))?;
                        let msg = it.next().and_then(|f| match f {
                            FrameOwned::BulkString(Some(b)) => Some(b),
                            _ => None,
                        }).ok_or_else(|| MiniRedisError::Protocol("PUBLISH missing message".into()))?;
                        Ok(Command::Publish(channel, msg))
                    }
                    "subscribe" => {
                        let mut channels = Vec::new();
                        for frame in it {
                            if let FrameOwned::BulkString(Some(b)) = frame {
                                channels.push(std::str::from_utf8(&b).map_err(|_| MiniRedisError::Protocol("invalid channel name".into()))?.to_string());
                            }
                        }
                        if channels.is_empty() {
                            return Err(MiniRedisError::Protocol("SUBSCRIBE missing channels".into()));
                        }
                        Ok(Command::Subscribe(channels))
                    }
                    "ping" => {
                        let msg = it.next().and_then(|f| match f {
                            FrameOwned::BulkString(Some(b)) => std::str::from_utf8(&b).ok().map(|s| s.to_string()),
                            FrameOwned::SimpleString(s) => Some(s),
                            _ => None,
                        });
                        Ok(Command::Ping(msg))
                    }
                    _ => Err(MiniRedisError::Protocol(format!("unknown command: {}", cmd_name))),
                }
            }
            _ => Err(MiniRedisError::Protocol("expected array for command".into())),
        }
    }

    pub async fn apply(self, db: &Db, conn: &mut Connection) -> Result<()> {
        match self {
            Command::Get(key) => {
                let response = match db.get(&key) {
                    Some(val) => FrameOwned::BulkString(Some(val)),
                    None => FrameOwned::BulkString(None),
                };
                conn.write_frame(&response).await?;
            }
            Command::Set(key, val, expire) => {
                db.set(key, val, expire);
                let response = FrameOwned::SimpleString("OK".to_string());
                conn.write_frame(&response).await?;
            }
            Command::Publish(channel, msg) => {
                let count = db.publish(&channel, msg);
                let response = FrameOwned::Integer(count as i64);
                conn.write_frame(&response).await?;
            }
            Command::Subscribe(channels) => {
                let mut receivers = Vec::with_capacity(channels.len());
                for channel in channels.clone() {
                    receivers.push((channel.clone(), db.subscribe(channel)));
                }

                // Initial confirmation
                let mut response = Vec::new();
                for channel in channels {
                    response.push(FrameOwned::Array(Some(vec![
                        FrameOwned::SimpleString("subscribe".into()),
                        FrameOwned::BulkString(Some(Bytes::from(channel))),
                        FrameOwned::Integer(receivers.len() as i64),
                    ])));
                }
                
                for resp in response {
                    conn.write_frame(&resp).await?;
                }

                // Loop to wait for messages
                loop {
                    // This is a bit simplified, ideally uses tokio::select
                    for (name, rx) in &mut receivers {
                        if let Ok(msg) = rx.try_recv() {
                             let msg_frame = FrameOwned::Array(Some(vec![
                                FrameOwned::SimpleString("message".into()),
                                FrameOwned::BulkString(Some(Bytes::from(name.clone()))),
                                FrameOwned::BulkString(Some(msg)),
                            ]));
                            conn.write_frame(&msg_frame).await?;
                        }
                    }
                    tokio::task::yield_now().await;
                }
            }
            Command::Ping(msg) => {
                let response = match msg {
                    Some(m) => FrameOwned::BulkString(Some(Bytes::from(m))),
                    None => FrameOwned::SimpleString("PONG".to_string()),
                };
                conn.write_frame(&response).await?;
            }
        }
        Ok(())
    }
}
