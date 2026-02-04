use bytes::Bytes;

#[derive(Debug, PartialEq, Clone)]
pub enum Frame<'a> {
    SimpleString(&'a str),
    Error(&'a str),
    Integer(i64),
    BulkString(Option<&'a [u8]>),
    Array(Option<Vec<Frame<'a>>>),
}

#[derive(Debug, Clone)]
pub enum FrameOwned {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<Bytes>),
    Array(Option<Vec<FrameOwned>>),
}

impl<'a> From<Frame<'a>> for FrameOwned {
    fn from(f: Frame<'a>) -> Self {
        match f {
            Frame::SimpleString(s) => FrameOwned::SimpleString(s.to_string()),
            Frame::Error(s) => FrameOwned::Error(s.to_string()),
            Frame::Integer(i) => FrameOwned::Integer(i),
            Frame::BulkString(opt) => FrameOwned::BulkString(opt.map(Bytes::copy_from_slice)),
            Frame::Array(opt) => {
                FrameOwned::Array(opt.map(|vec| vec.into_iter().map(FrameOwned::from).collect()))
            }
        }
    }
}
