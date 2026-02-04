use bytes::Bytes;
use crossbeam_skiplist::SkipMap;
use ouroboros::self_referencing;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A basic MemTable implementation.
pub struct MemTable {
    map: SkipMap<Bytes, Bytes>,
    wal: Option<crate::wal::Wal>,
    id: usize,
    approximate_size: AtomicUsize,
}

impl MemTable {
    /// Create a new MemTable.
    pub fn new(id: usize) -> Self {
        Self {
            map: SkipMap::new(),
            wal: None, // WAL implemented later
            id,
            approximate_size: AtomicUsize::new(0),
        }
    }

    pub fn create_with_wal(id: usize, path: impl AsRef<std::path::Path>) -> crate::Result<Self> {
        let wal = crate::wal::Wal::create(path)?;
        Ok(Self {
            map: SkipMap::new(),
            wal: Some(wal),
            id,
            approximate_size: AtomicUsize::new(0),
        })
    }

    /// Get a value by key.
    pub fn get(&self, key: &[u8]) -> Option<Bytes> {
        self.map.get(key).map(|e| e.value().clone())
    }

    /// Put a key-value pair into the MemTable.
    pub fn put(&self, key: &[u8], value: &[u8]) -> crate::Result<()> {
        let estimated_size = key.len() + value.len();
        self.map
            .insert(Bytes::copy_from_slice(key), Bytes::copy_from_slice(value));
        self.approximate_size
            .fetch_add(estimated_size, Ordering::Relaxed);
        if let Some(ref wal) = self.wal {
            wal.put(key, value)?;
        }
        Ok(())
    }

    pub fn scan(
        &self,
        _lower: std::ops::Bound<&[u8]>,
        _upper: std::ops::Bound<&[u8]>,
    ) -> MemTableIterator {
        unimplemented!("scan")
    }

    pub fn flush(&self, _builder: &mut crate::sstable::SsTableBuilder) -> crate::Result<()> {
        unimplemented!("flush to sstable")
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn approximate_size(&self) -> usize {
        self.approximate_size.load(Ordering::Relaxed)
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

#[self_referencing]
pub struct MemTableIterator {
    map: SkipMap<Bytes, Bytes>,
    #[borrows(map)]
    #[not_covariant]
    iter: crossbeam_skiplist::map::Iter<'this, Bytes, Bytes>,
    item: (Bytes, Bytes),
}

impl MemTableIterator {
    pub fn next(&mut self) -> Option<(Bytes, Bytes)> {
        // Simple iterator implementation for now
        let entry =
            self.with_iter_mut(|iter| iter.next().map(|e| (e.key().clone(), e.value().clone())));
        entry
    }
}
