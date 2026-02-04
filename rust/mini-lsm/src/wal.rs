use bytes::BufMut;
use crc32fast::Hasher;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct Wal {
    file: Arc<Mutex<BufWriter<File>>>,
}

impl Wal {
    pub fn create(path: impl AsRef<Path>) -> crate::Result<Self> {
        let file = File::create(path)?;
        Ok(Self {
            file: Arc::new(Mutex::new(BufWriter::new(file))),
        })
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> crate::Result<()> {
        let mut file = self.file.lock().unwrap();
        let mut buf: Vec<u8> = Vec::with_capacity(key.len() + value.len() + 8);

        // Format: [key_len: u16] [key] [val_len: u16] [value] [checksum: u32]
        buf.put_u16_le(key.len() as u16);
        buf.put(key);
        buf.put_u16_le(value.len() as u16);
        buf.put(value);

        let mut hasher = Hasher::new();
        hasher.update(&buf);
        let checksum = hasher.finalize();
        buf.put_u32_le(checksum);

        file.write_all(&buf)?;
        // Ideally we fsync here or periodically
        // file.get_mut().sync_all()?;
        Ok(())
    }
}
