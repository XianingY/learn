# Draft: Mini-LSM Implementation

## Requirements (confirmed)
- Core components: Memtable, SSTable, Block Cache, WAL, Compaction
- Key Crates: bytes, crossbeam-skiplist, parking_lot, serde
- Architecture: L0 (Tiered) + L1..Ln (Leveled) compaction
- User Profile: Intermediate/Advanced (completed Mini-Redis)

## Technical Decisions
- Project structure: Standalone crate at `/Users/byzantium/github/learn/rust/mini-lsm`
- Follow mini-redis patterns (Arc<Shared>, module organization)
- Use crossbeam-skiplist for MemTable (already familiar from mini-redis)

## Research Findings
### From Codebase Exploration:
- mini-redis uses tokio, bytes, dashmap, crossbeam-skiplist, serde
- Arc<Shared> pattern for concurrent state management
- Zero-copy parsing patterns in frame.rs
- Integration tests in /tests directory

### From LSM Research (skyzh/mini-lsm, tikv/agatedb):
- MemTable: `SkipMap<Bytes, Bytes>` with `AtomicUsize` for size tracking
- Use `ouroboros` crate for self-referential iterators
- SSTable block layout: `[data | offsets[] | num_entries: u16 | crc32]`
- SSTable file layout: `[Data Blocks] [Index Block] [Bloom Filter] [Footer]`
- WAL format: `[key_len: u16 | key | value_len: u16 | value | checksum: u32]`
- Must use `fsync()` for true durability, not just `write_all()`
- Leveled compaction: L0 overlapping, L1+ non-overlapping key ranges
- Each level ~10x larger than previous
- Block cache needs sharding to avoid contention

## Deliverables (from user request)
1. **Memtable**: Concurrent SkipList using crossbeam-skiplist
2. **SSTable Format**: 
   - [Data Block 1] ... [Data Block N] [Index Block] [Bloom Filter] [Footer]
   - Block encoding with CRC32 checksums
3. **Read Path**: Memtable -> Immutable Memtables -> SSTables (L0 -> Ln)
4. **Compaction**: Simple Leveled Compaction controller
5. **WAL**: Crash recovery support

## Key Learning Points (user specified)
- Interior Mutability patterns
- Disk Format design

## Decisions Made
- **Test Strategy**: TDD (Tests First) - Correctness paramount for storage engines
- **I/O Model**: Sync I/O (std::fs) - Keep simple, focus on LSM algorithms
- **Benchmarks**: Yes - criterion for write/read throughput
- **Unsafe Code**: Educational Unsafe - zero-copy block parsing, memory mapping for learning

## Scope Boundaries
- INCLUDE: Full LSM implementation with all 5 components
- EXCLUDE: Network layer (this is storage engine only)
