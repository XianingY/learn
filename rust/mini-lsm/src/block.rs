use bytes::{Buf, BufMut};

pub const SIZEOF_U16: usize = std::mem::size_of::<u16>();

/// A data block in an SSTable.
pub struct Block {
    data: Vec<u8>,
    pub offsets: Vec<u16>,
}

impl Block {
    /// Decode a byte vector into a Block.
    pub fn decode(data: &[u8]) -> Block {
        let num_of_elements = (&data[data.len() - SIZEOF_U16..]).get_u16_le() as usize;
        let data_end = data.len() - SIZEOF_U16 - num_of_elements * SIZEOF_U16;
        let offsets_raw = &data[data_end..data.len() - SIZEOF_U16];
        let offsets = offsets_raw
            .chunks(SIZEOF_U16)
            .map(|mut x| x.get_u16_le())
            .collect();
        let data = data[0..data_end].to_vec();
        Block { data, offsets }
    }

    /// Encode the block data for storage.
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = self.data.clone();
        for offset in &self.offsets {
            buf.put_u16_le(*offset);
        }
        buf.put_u16_le(self.offsets.len() as u16);
        buf
    }

    /// Get the key and value at the given index.
    pub fn get_entry(&self, idx: usize) -> (&[u8], &[u8]) {
        let start = self.offsets[idx] as usize;
        let end = if idx + 1 < self.offsets.len() {
            self.offsets[idx + 1] as usize
        } else {
            self.data.len()
        };
        let entry = &self.data[start..end];
        let mut entry_mut = entry;

        let key_len = entry_mut.get_u16_le() as usize;
        let key = &entry[2..2 + key_len];
        let value_len = (&entry[2 + key_len..]).get_u16_le() as usize;
        let value = &entry[2 + key_len + 2..2 + key_len + 2 + value_len];

        (key, value)
    }
}

/// Builds a block with a target size.
pub struct BlockBuilder {
    offsets: Vec<u16>,
    data: Vec<u8>,
    block_size: usize,
}

impl BlockBuilder {
    pub fn new(block_size: usize) -> Self {
        Self {
            offsets: Vec::new(),
            data: Vec::new(),
            block_size,
        }
    }

    /// Adds a key-value pair to the block. Returns false if the block is full.
    pub fn add(&mut self, key: &[u8], value: &[u8]) -> bool {
        assert!(!key.is_empty(), "key must not be empty");
        // Check if adding this entry would exceed the target block size
        // 2 (key_len) + key_len + 2 (val_len) + val_len
        let entry_size = SIZEOF_U16 + key.len() + SIZEOF_U16 + value.len();
        // SIZEOF_U16 (offset) + SIZEOF_U16 (num_entries)
        let metadata_increase = SIZEOF_U16;

        // Total size if we add this entry = current_data + current_offsets + new_entry + new_offset + num_entries_field
        let total_size_after = self.data.len()
            + (self.offsets.len() * SIZEOF_U16)
            + entry_size
            + metadata_increase
            + SIZEOF_U16;

        if total_size_after > self.block_size && !self.is_empty() {
            return false;
        }

        self.offsets.push(self.data.len() as u16);
        self.data.put_u16_le(key.len() as u16);
        self.data.put(key);
        self.data.put_u16_le(value.len() as u16);
        self.data.put(value);

        true
    }

    pub fn is_empty(&self) -> bool {
        self.offsets.is_empty()
    }

    pub fn build(self) -> Block {
        Block {
            data: self.data,
            offsets: self.offsets,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_encode_decode() {
        let mut builder = BlockBuilder::new(4096);
        builder.add(b"key1", b"value1");
        builder.add(b"key2", b"value2");
        let block = builder.build();
        let encoded = block.encode();
        let decoded = Block::decode(&encoded);

        assert_eq!(decoded.offsets.len(), 2);
        assert_eq!(decoded.get_entry(0), (&b"key1"[..], &b"value1"[..]));
        assert_eq!(decoded.get_entry(1), (&b"key2"[..], &b"value2"[..]));
    }
}
