use bytes::Bytes;
use parking_lot::{Mutex, RwLock};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::error::Result;
use crate::memtable::MemTable;
use crate::sstable::{BlockCache, SsTable};

pub struct LsmStorageState {
    pub memtable: Arc<MemTable>,
    pub imm_memtables: Vec<Arc<MemTable>>,
    pub l0_sstables: Vec<usize>,
    pub levels: Vec<(usize, Vec<usize>)>, // (level_id, sst_ids)
    pub sstables: HashMap<usize, Arc<SsTable>>,
}

impl LsmStorageState {
    fn create(memtable: Arc<MemTable>) -> Self {
        Self {
            memtable,
            imm_memtables: Vec::new(),
            l0_sstables: Vec::new(),
            levels: Vec::new(),
            sstables: HashMap::new(),
        }
    }
}

pub struct LsmStorage {
    state: Arc<RwLock<LsmStorageState>>,
    path: PathBuf,
    block_cache: Arc<Mutex<BlockCache>>,
    next_sst_id: AtomicUsize,
}

impl LsmStorage {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        std::fs::create_dir_all(&path)?;

        // TODO: Load manifest/recovery
        let memtable = Arc::new(MemTable::create_with_wal(0, path.join("mem.wal"))?);
        let state = Arc::new(RwLock::new(LsmStorageState::create(memtable)));
        let block_cache = Arc::new(Mutex::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(1024).unwrap(),
        )));

        Ok(Self {
            state,
            path,
            block_cache,
            next_sst_id: AtomicUsize::new(1),
        })
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Bytes>> {
        let snapshot = {
            let guard = self.state.read();
            guard.memtable.clone()
        }; // Cheap clone Arc

        // 1. Search MemTable
        if let Some(value) = snapshot.get(key) {
            if value.is_empty() {
                return Ok(None);
            } // Tombstone
            return Ok(Some(value));
        }

        // 2. Search Immutable MemTables
        // TODO: Add search logic

        // 3. Search L0 SSTables
        // TODO: Add search logic

        Ok(None)
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        let size;
        {
            let guard = self.state.read();
            guard.memtable.put(key, value)?;
            size = guard.memtable.approximate_size();
        }

        if size > 1 << 20 {
            // 1MB threshold
            self.force_freeze_memtable()?;
        }

        Ok(())
    }

    fn force_freeze_memtable(&self) -> Result<()> {
        let mut guard = self.state.write();
        if guard.memtable.approximate_size() <= 1 << 20 {
            return Ok(());
        }

        let old_memtable = guard.memtable.clone();
        let new_id = old_memtable.id() + 1;
        let new_memtable = Arc::new(MemTable::create_with_wal(
            new_id,
            self.path.join(format!("{:05}.wal", new_id)),
        )?);

        guard.imm_memtables.insert(0, old_memtable);
        guard.memtable = new_memtable;

        // Trigger flush task here
        Ok(())
    }
}
