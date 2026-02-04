use bytes::{BufMut, Bytes};

/// A simple Bloom Filter implementation.
pub struct Bloom {
    /// data of filter in bits
    filter: Bytes,
    /// number of hash functions
    k: u8,
}

impl Bloom {
    /// Create a new Bloom Filter from keys.
    /// bits_per_key: normally 10, which gives ~1% false positive rate.
    pub fn build_from_keys(keys: &[Bytes], bits_per_key: usize) -> Self {
        let k = (bits_per_key as f64 * 0.69) as u8;
        let k = k.clamp(1, 30);
        let nbits = (keys.len() * bits_per_key).max(64);
        let nbytes = (nbits + 7) / 8;
        let nbits = nbytes * 8;

        let mut filter = vec![0u8; nbytes];

        for key in keys {
            let mut h = farmhash::fingerprint32(key);
            let delta = (h >> 17) | (h << 15);
            for _ in 0..k {
                let bit_pos = (h as usize) % nbits;
                filter[bit_pos / 8] |= 1 << (bit_pos % 8);
                h = h.wrapping_add(delta);
            }
        }

        Self {
            filter: Bytes::from(filter),
            k,
        }
    }

    /// Decode a Bloom Filter from a byte buffer.
    pub fn decode(buf: &[u8]) -> crate::Result<Self> {
        let filter = &buf[..buf.len() - 1];
        let k = buf[buf.len() - 1];
        Ok(Self {
            filter: Bytes::copy_from_slice(filter),
            k,
        })
    }

    /// Encode the Bloom Filter to a byte buffer.
    pub fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.filter);
        buf.put_u8(self.k);
    }

    /// Check if the key may exist in the Bloom Filter.
    pub fn may_contain(&self, key: &[u8]) -> bool {
        let nbits = self.filter.len() * 8;
        if nbits == 0 {
            return false;
        }

        let mut h = farmhash::fingerprint32(key);
        let delta = (h >> 17) | (h << 15);

        for _ in 0..self.k {
            let bit_pos = (h as usize) % nbits;
            if (self.filter[bit_pos / 8] & (1 << (bit_pos % 8))) == 0 {
                return false;
            }
            h = h.wrapping_add(delta);
        }

        true
    }
}
