use bytes::{Buf, BufMut, Bytes};
use crc32fast::Hasher;
use parking_lot::Mutex;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::block::BlockBuilder;
use crate::bloom::Bloom;
use crate::error::{LsmError, Result};

const FOOTER_SIZE: usize = 8 + 4 + 4 + 4;

pub(crate) struct Footer {
    file_size: u64,
    data_checksum: u32,
    index_checksum: u32,
    bloom_checksum: u32,
}

impl Footer {
    fn new(file_size: u64, data_checksum: u32, index_checksum: u32, bloom_checksum: u32) -> Self {
        Self {
            file_size,
            data_checksum,
            index_checksum,
            bloom_checksum,
        }
    }

    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(FOOTER_SIZE);
        buf.put_u64_le(self.file_size);
        buf.put_u32_le(self.data_checksum);
        buf.put_u32_le(self.index_checksum);
        buf.put_u32_le(self.bloom_checksum);
        buf
    }

    #[allow(dead_code)]
    fn decode(mut buf: &[u8]) -> Result<Self> {
        if buf.len() != FOOTER_SIZE {
            return Err(LsmError::Format("invalid footer length".to_string()));
        }
        Ok(Self {
            file_size: buf.get_u64_le(),
            data_checksum: buf.get_u32_le(),
            index_checksum: buf.get_u32_le(),
            bloom_checksum: buf.get_u32_le(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct BlockMeta {
    pub offset: u32,
    pub first_key: Vec<u8>,
}

pub type BlockCache = lru::LruCache<u32, Bytes>;

pub struct SsTable {
    pub id: usize,
    pub file_path: PathBuf,
    pub block_meta: Vec<BlockMeta>,
    pub index_offset: u64,
    pub index_len: u64,
    pub bloom_offset: u64,
    pub bloom_len: u64,
    #[allow(dead_code)]
    pub(crate) footer: Footer,
    pub block_cache: Option<Arc<Mutex<BlockCache>>>,
}

pub struct SsTableBuilder {
    block_size: usize,
    current_block: BlockBuilder,
    current_first_key: Option<Vec<u8>>,
    block_meta: Vec<BlockMeta>,
    data: Vec<u8>,
    keys: Vec<Bytes>,
}

impl SsTableBuilder {
    pub fn new(block_size: usize) -> Self {
        Self {
            block_size,
            current_block: BlockBuilder::new(block_size),
            current_first_key: None,
            block_meta: Vec::new(),
            data: Vec::new(),
            keys: Vec::new(),
        }
    }

    pub fn add(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        if self.current_block.is_empty() {
            self.current_first_key = Some(key.to_vec());
        }
        if !self.current_block.add(key, value) {
            self.flush_current_block()?;
            self.current_first_key = Some(key.to_vec());
            let added = self.current_block.add(key, value);
            if !added {
                return Err(LsmError::Format("entry too large for block".to_string()));
            }
        }
        self.keys.push(Bytes::copy_from_slice(key));
        Ok(())
    }

    pub fn build(
        mut self,
        id: usize,
        path: impl AsRef<Path>,
        cache: Option<Arc<Mutex<BlockCache>>>,
    ) -> Result<SsTable> {
        self.finish_data_blocks()?;

        let mut data_hasher = Hasher::new();
        data_hasher.update(&self.data);
        let data_checksum = data_hasher.finalize();

        let index_bytes = self.build_index_block()?;
        let mut index_hasher = Hasher::new();
        index_hasher.update(&index_bytes);
        let index_checksum = index_hasher.finalize();

        let bloom = Bloom::build_from_keys(&self.keys, 10);
        let mut bloom_bytes = Vec::new();
        bloom.encode(&mut bloom_bytes);
        let mut bloom_hasher = Hasher::new();
        bloom_hasher.update(&bloom_bytes);
        let bloom_checksum = bloom_hasher.finalize();

        let index_offset = self.data.len() as u64;
        let index_len = index_bytes.len() as u64;
        let bloom_offset = index_offset + index_len;
        let bloom_len = bloom_bytes.len() as u64;
        let footer_offset = bloom_offset + bloom_len;
        let footer = Footer::new(
            footer_offset + FOOTER_SIZE as u64,
            data_checksum,
            index_checksum,
            bloom_checksum,
        );

        let path = path.as_ref();
        let mut writer = BufWriter::new(File::create(path)?);
        writer.write_all(&self.data)?;
        writer.write_all(&index_bytes)?;
        writer.write_all(&bloom_bytes)?;
        writer.write_all(&footer.encode())?;
        writer.flush()?;

        Ok(SsTable {
            id,
            file_path: path.to_path_buf(),
            block_meta: self.block_meta,
            index_offset,
            index_len,
            bloom_offset,
            bloom_len,
            footer,
            block_cache: cache,
        })
    }

    fn finish_data_blocks(&mut self) -> Result<()> {
        if !self.current_block.is_empty() {
            self.flush_current_block()?;
        }
        Ok(())
    }

    fn flush_current_block(&mut self) -> Result<()> {
        if self.current_block.is_empty() {
            return Ok(());
        }
        let first_key = self
            .current_first_key
            .take()
            .ok_or_else(|| LsmError::Format("missing block first key".to_string()))?;
        let block =
            std::mem::replace(&mut self.current_block, BlockBuilder::new(self.block_size)).build();
        let encoded = block.encode();
        let offset = self.data.len() as u32;
        self.data.extend_from_slice(&encoded);
        self.block_meta.push(BlockMeta { offset, first_key });
        Ok(())
    }

    fn build_index_block(&self) -> Result<Vec<u8>> {
        let mut index_builder = BlockBuilder::new(self.block_size);
        for meta in &self.block_meta {
            let mut offset_bytes = [0u8; 4];
            offset_bytes.as_mut().put_u32_le(meta.offset);
            if !index_builder.add(&meta.first_key, &offset_bytes) {
                return Err(LsmError::Format("index block too large".to_string()));
            }
        }
        let block = index_builder.build();
        Ok(block.encode())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Block;
    use std::io::{Read, Seek, SeekFrom};
    use tempfile::tempdir;

    #[test]
    fn test_sstable_footer_round_trip() {
        let footer = Footer::new(123, 1, 2, 3);
        let encoded = footer.encode();
        let decoded = Footer::decode(&encoded).expect("footer decode");
        assert_eq!(decoded.file_size, 123);
        assert_eq!(decoded.data_checksum, 1);
        assert_eq!(decoded.index_checksum, 2);
        assert_eq!(decoded.bloom_checksum, 3);
    }

    #[test]
    fn test_bloom_filter_round_trip() {
        let keys = vec![Bytes::from("alpha"), Bytes::from("beta")];
        let bloom = Bloom::build_from_keys(&keys, 10);
        let mut encoded = Vec::new();
        bloom.encode(&mut encoded);
        let decoded = Bloom::decode(&encoded).expect("bloom decode");
        assert!(decoded.may_contain(b"alpha"));
        assert!(decoded.may_contain(b"beta"));
    }

    #[test]
    fn test_builder_writes_index_and_footer() {
        let mut builder = SsTableBuilder::new(64);
        builder.add(b"apple", b"1").expect("add apple");
        builder.add(b"banana", b"2").expect("add banana");

        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("1.sst");
        let table = builder.build(1, &path, None).expect("build sstable");

        assert_eq!(table.block_meta.len(), 1);
        assert_eq!(table.block_meta[0].first_key, b"apple".to_vec());

        let mut file = File::open(&path).expect("open sstable");
        let file_size = file.metadata().expect("metadata").len();
        let mut footer_bytes = vec![0u8; FOOTER_SIZE];
        file.seek(SeekFrom::End(-(FOOTER_SIZE as i64)))
            .expect("seek footer");
        file.read_exact(&mut footer_bytes).expect("read footer");
        let footer = Footer::decode(&footer_bytes).expect("decode footer");
        assert_eq!(footer.file_size, file_size);

        let mut index_bytes = vec![0u8; table.index_len as usize];
        file.seek(SeekFrom::Start(table.index_offset))
            .expect("seek index");
        file.read_exact(&mut index_bytes).expect("read index");
        let index_block = Block::decode(&index_bytes);
        let (first_key, offset_bytes) = index_block.get_entry(0);
        assert_eq!(first_key, b"apple");
        assert_eq!(offset_bytes.len(), 4);

        let mut data_bytes = vec![0u8; table.index_offset as usize];
        file.seek(SeekFrom::Start(0)).expect("seek data");
        file.read_exact(&mut data_bytes).expect("read data");
        let mut data_hasher = Hasher::new();
        data_hasher.update(&data_bytes);
        assert_eq!(footer.data_checksum, data_hasher.finalize());

        let mut bloom_bytes = vec![0u8; table.bloom_len as usize];
        file.seek(SeekFrom::Start(table.bloom_offset))
            .expect("seek bloom");
        file.read_exact(&mut bloom_bytes).expect("read bloom");
        let mut index_hasher = Hasher::new();
        index_hasher.update(&index_bytes);
        assert_eq!(footer.index_checksum, index_hasher.finalize());
        let mut bloom_hasher = Hasher::new();
        bloom_hasher.update(&bloom_bytes);
        assert_eq!(footer.bloom_checksum, bloom_hasher.finalize());
    }
}
