# Mini-LSM: Log-Structured Merge-Tree Implementation in Rust

## TL;DR

> **Quick Summary**: Build a production-quality LSM-tree storage engine in Rust, focusing on concurrent MemTable operations, block-based SSTable format with checksums, WAL for crash recovery, and leveled compaction. Educational unsafe code for zero-copy parsing.
> 
> **Deliverables**:
> - Complete `mini-lsm` crate with lib + CLI
> - MemTable with concurrent SkipList
> - SSTable reader/writer with block cache
> - WAL with crash recovery
> - Leveled compaction controller
> - Criterion benchmarks
> 
> **Estimated Effort**: Large (40-60 hours)
> **Parallel Execution**: YES - 4 waves
> **Critical Path**: Project Setup → MemTable → SSTable → Storage Engine → Compaction

---

## Context

### Original Request
Create a high-fidelity Mini-LSM implementation in Rust with:
- Memtable using crossbeam-skiplist
- SSTable format with blocks, bloom filter, index, checksums
- Read path: Memtable → Immutable Memtables → SSTables (L0 → Ln)
- Leveled compaction controller
- WAL for crash recovery
- Focus on Interior Mutability and Disk Format as learning points

### Interview Summary
**Key Discussions**:
- Test Strategy: TDD (Tests First) - correctness paramount for storage engines
- I/O Model: Sync I/O (std::fs) - keep simple, focus on LSM algorithms
- Benchmarks: Yes - criterion for write/read throughput measurement
- Unsafe Code: Educational unsafe for zero-copy block parsing, memory mapping

**Research Findings**:
- From `skyzh/mini-lsm`: Use `SkipMap<Bytes, Bytes>` with `AtomicUsize` for size tracking
- Use `ouroboros` crate for self-referential iterators (SkipMap iterator borrows from map)
- SSTable block layout: `[data | offsets[] | num_entries: u16 | crc32]`
- WAL format: `[key_len: u16 | key | value_len: u16 | value | checksum: u32]`
- Must use `fsync()` for true durability
- From `mini-redis`: Arc<Shared> patterns, bytes crate usage, module organization

### Gap Analysis (Self-Review)
**Identified Gaps Addressed**:
- Delete semantics: Using tombstones (empty value = deleted)
- Iterator invalidation: Using `ouroboros` for self-referential structs
- Block cache eviction: Using simple LRU with `lru` crate
- Compaction file selection: Following mini-lsm's leveled algorithm

---

## Work Objectives

### Core Objective
Build a complete, correct, and educational LSM-tree storage engine that demonstrates advanced Rust patterns (interior mutability, unsafe zero-copy parsing, concurrent data structures).

### Concrete Deliverables
- `/Users/byzantium/github/learn/rust/mini-lsm/` - Complete crate
- `src/lib.rs` - Public API exposing `LsmStorage`
- `src/memtable.rs` - Concurrent MemTable with SkipMap
- `src/wal.rs` - Write-Ahead Log with crash recovery
- `src/block.rs` - Block encoding/decoding with unsafe parsing
- `src/table/` - SSTable builder, reader, iterator
- `src/compact/` - Leveled compaction controller
- `src/cache.rs` - Block cache with LRU eviction
- `src/lsm_storage.rs` - Main storage engine orchestrator
- `benches/` - Criterion benchmarks
- `tests/` - Integration tests (crash recovery, compaction correctness)

### Definition of Done
- [ ] `cargo test` passes all unit and integration tests
- [ ] `cargo bench` runs without errors
- [ ] Write 1GB of data, restart process, verify all data readable
- [ ] Compaction reduces L0 file count when threshold exceeded
- [ ] `cargo clippy` reports no warnings

### Must Have
- CRC32 checksums on all blocks and WAL entries
- Bloom filter for SSTable (skip files that don't contain key)
- Concurrent read/write to MemTable
- Immutable memtable queue for flush pipeline
- fsync on WAL writes for durability

### Must NOT Have (Guardrails)
- NO async I/O (keep sync with std::fs)
- NO network layer (storage engine only)
- NO SQL or query language (raw key-value API)
- NO excessive abstraction (keep code educational)
- NO external database dependencies
- NO skipping checksums for "performance"

---

## Verification Strategy

> **UNIVERSAL RULE: ZERO HUMAN INTERVENTION**
>
> ALL tasks verifiable by agent-executed commands. No manual testing.

### Test Decision
- **Infrastructure exists**: NO (new project)
- **Automated tests**: TDD (Tests First)
- **Framework**: Rust built-in `#[test]` + `cargo test`

### TDD Workflow Per Task
Each implementation task follows RED-GREEN-REFACTOR:
1. **RED**: Write failing test first → `cargo test` fails
2. **GREEN**: Implement minimum code → `cargo test` passes  
3. **REFACTOR**: Clean up while green → `cargo test` still passes

### Agent-Executed QA Scenarios
Every task includes executable verification via:
- `cargo test` - Unit and integration tests
- `cargo bench` - Performance benchmarks
- `cargo clippy` - Lint checks
- Custom CLI commands for end-to-end verification

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately):
├── Task 1: Project Setup & Dependencies
└── Task 2: Error Types & Common Utilities

Wave 2 (After Wave 1):
├── Task 3: Block Encoding/Decoding (unsafe zero-copy)
├── Task 4: MemTable with SkipList
└── Task 5: WAL Implementation

Wave 3 (After Wave 2):
├── Task 6: SSTable Builder
├── Task 7: SSTable Reader & Iterator
├── Task 8: Bloom Filter
└── Task 9: Block Cache

Wave 4 (After Wave 3):
├── Task 10: LSM Storage Engine (Read/Write Path)
├── Task 11: Immutable MemTable Queue
└── Task 12: Leveled Compaction Controller

Wave 5 (After Wave 4):
├── Task 13: Crash Recovery Integration
├── Task 14: Benchmarks
└── Task 15: CLI Tool

Critical Path: 1 → 3 → 6 → 10 → 12 → 13
Parallel Speedup: ~45% faster than sequential
```

### Dependency Matrix

| Task | Depends On | Blocks | Can Parallelize With |
|------|------------|--------|---------------------|
| 1 | None | 2,3,4,5 | None |
| 2 | 1 | 3,4,5,6,7 | None |
| 3 | 2 | 6,7 | 4, 5 |
| 4 | 2 | 10,11 | 3, 5 |
| 5 | 2 | 10,13 | 3, 4 |
| 6 | 3 | 7,10 | 8, 9 |
| 7 | 3,6 | 10 | 8, 9 |
| 8 | 2 | 6 | 9 |
| 9 | 3 | 10 | 6, 7, 8 |
| 10 | 4,5,7,9 | 11,12,13 | None |
| 11 | 4,10 | 12 | None |
| 12 | 10,11 | 13 | None |
| 13 | 5,10,12 | 14,15 | None |
| 14 | 10 | None | 15 |
| 15 | 10 | None | 14 |

---

## TODOs

### Wave 1: Foundation

- [ ] 1. Project Setup & Dependencies

  **What to do**:
  - Create `mini-lsm/` directory with `cargo init --lib`
  - Configure Cargo.toml with all dependencies
  - Set up module structure in `src/lib.rs`
  - Create basic directory structure

  **Dependencies to add**:
  ```toml
  [dependencies]
  bytes = "1.5"
  crossbeam-skiplist = "0.1"
  parking_lot = "0.12"
  crc32fast = "1.3"
  thiserror = "2.0"
  tracing = "0.1"
  lru = "0.12"
  ouroboros = "0.18"
  farmhash = "1.1"
  
  [dev-dependencies]
  tempfile = "3.10"
  criterion = { version = "0.5", features = ["html_reports"] }
  rand = "0.8"
  
  [[bench]]
  name = "lsm_bench"
  harness = false
  ```

  **Must NOT do**:
  - Don't add async runtime (tokio)
  - Don't create complex workspace structure

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]
    - `git-master`: Initial commit after setup

  **Parallelization**:
  - **Can Run In Parallel**: NO (foundation for all)
  - **Parallel Group**: Wave 1 (alone)
  - **Blocks**: Tasks 2, 3, 4, 5
  - **Blocked By**: None

  **References**:
  - `mini-redis/Cargo.toml` - Dependency patterns for bytes, crossbeam-skiplist
  - `mini-redis/src/lib.rs` - Module organization pattern
  - https://github.com/skyzh/mini-lsm/blob/main/mini-lsm/Cargo.toml - Reference dependencies

  **Acceptance Criteria**:
  - [ ] `cargo check` succeeds with no errors
  - [ ] Directory structure exists: `src/`, `tests/`, `benches/`
  - [ ] `src/lib.rs` declares all planned modules (even if empty)

  **Agent-Executed QA Scenarios**:
  ```
  Scenario: Project compiles successfully
    Tool: Bash
    Preconditions: None
    Steps:
      1. cd /Users/byzantium/github/learn/rust/mini-lsm
      2. cargo check 2>&1
      3. Assert: exit code 0
      4. Assert: output contains "Finished"
    Expected Result: Project compiles with all dependencies resolved
    Evidence: cargo check output captured

  Scenario: Module structure declared
    Tool: Bash
    Preconditions: Task 1 complete
    Steps:
      1. grep -c "pub mod" src/lib.rs
      2. Assert: count >= 6 (memtable, wal, block, table, compact, cache)
    Expected Result: All major modules declared
    Evidence: grep output
  ```

  **Commit**: YES
  - Message: `feat(mini-lsm): initialize project with dependencies`
  - Files: `mini-lsm/`
  - Pre-commit: `cargo check`

---

- [ ] 2. Error Types & Common Utilities

  **What to do**:
  - Create `src/error.rs` with `LsmError` enum using `thiserror`
  - Create `src/key.rs` for key encoding utilities
  - Add `Result<T>` type alias
  - Write tests for error types

  **TDD Sequence**:
  1. RED: Write test for `LsmError::Io` wrapping std::io::Error
  2. GREEN: Implement error enum
  3. RED: Write test for key comparison
  4. GREEN: Implement key utilities

  **Must NOT do**:
  - Don't create overly complex error hierarchies
  - Don't add key versioning yet (MVCC is out of scope)

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 1 (sequential after Task 1)
  - **Blocks**: Tasks 3, 4, 5, 6, 7
  - **Blocked By**: Task 1

  **References**:
  - `mini-redis/src/error.rs` - thiserror pattern
  - https://github.com/skyzh/mini-lsm/blob/main/mini-lsm/src/lib.rs - Error definitions

  **Acceptance Criteria**:
  - [ ] `cargo test error` passes
  - [ ] `LsmError` has variants: Io, Corruption, NotFound, InvalidData
  - [ ] `Result<T>` type alias exported from lib.rs

  **Agent-Executed QA Scenarios**:
  ```
  Scenario: Error types compile and test
    Tool: Bash
    Preconditions: Task 1 complete
    Steps:
      1. cd /Users/byzantium/github/learn/rust/mini-lsm
      2. cargo test error --lib 2>&1
      3. Assert: exit code 0
      4. Assert: output contains "test result: ok"
    Expected Result: All error tests pass
    Evidence: cargo test output
  ```

  **Commit**: YES
  - Message: `feat(mini-lsm): add error types and key utilities`
  - Files: `src/error.rs`, `src/key.rs`, `src/lib.rs`
  - Pre-commit: `cargo test`

---

### Wave 2: Core Components (Parallel)

- [ ] 3. Block Encoding/Decoding with Unsafe Zero-Copy

  **What to do**:
  - Create `src/block.rs` with Block struct
  - Implement block encoding: `[kv_data | offsets[] | num_entries: u16]`
  - Implement BlockBuilder for constructing blocks
  - Use unsafe for zero-copy parsing of block data
  - Add CRC32 checksum validation
  - Create BlockIterator for scanning entries

  **TDD Sequence**:
  1. RED: Test encoding single key-value pair
  2. GREEN: Implement BlockBuilder.add()
  3. RED: Test decoding retrieves same data
  4. GREEN: Implement Block::decode() with unsafe pointer arithmetic
  5. RED: Test checksum validation catches corruption
  6. GREEN: Implement CRC32 check
  7. RED: Test BlockIterator traversal
  8. GREEN: Implement iterator

  **Interior Mutability Learning Point**:
  - BlockBuilder uses internal Vec that grows
  - Block is immutable after construction (zero-copy reads)

  **Unsafe Learning Point**:
  ```rust
  // Zero-copy access to offset array at end of block
  unsafe fn get_offset(&self, idx: usize) -> u16 {
      let offset_ptr = self.data.as_ptr()
          .add(self.data.len() - 2 - (self.num_entries - idx) * 2);
      u16::from_le_bytes(*(offset_ptr as *const [u8; 2]))
  }
  ```

  **Must NOT do**:
  - Don't use compression (out of scope)
  - Don't skip CRC validation

  **Recommended Agent Profile**:
  - **Category**: `ultrabrain`
    - Reason: Unsafe code requires careful memory reasoning
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4, 5)
  - **Blocks**: Tasks 6, 7, 9
  - **Blocked By**: Task 2

  **References**:
  - https://github.com/skyzh/mini-lsm/blob/main/mini-lsm/src/block.rs - Block structure
  - https://github.com/skyzh/mini-lsm/blob/main/mini-lsm/src/block/builder.rs - Builder pattern
  - Rust unsafe guidelines: https://doc.rust-lang.org/nomicon/

  **Acceptance Criteria**:
  - [ ] `cargo test block` passes (5+ tests)
  - [ ] Block encoding/decoding roundtrip preserves data
  - [ ] CRC32 mismatch returns `LsmError::Corruption`
  - [ ] BlockIterator yields entries in sorted order
  - [ ] Contains at least one `unsafe` block with safety comment

  **Agent-Executed QA Scenarios**:
  ```
  Scenario: Block roundtrip test
    Tool: Bash
    Preconditions: Task 2 complete
    Steps:
      1. cargo test block::tests::test_block_roundtrip --lib 2>&1
      2. Assert: exit code 0
      3. Assert: output contains "test block::tests::test_block_roundtrip ... ok"
    Expected Result: Block encode/decode preserves data
    Evidence: cargo test output

  Scenario: Corruption detection test
    Tool: Bash
    Preconditions: Block implementation complete
    Steps:
      1. cargo test block::tests::test_corruption_detected --lib 2>&1
      2. Assert: exit code 0
    Expected Result: Corrupted blocks are rejected
    Evidence: cargo test output

  Scenario: Unsafe code documented
    Tool: Bash
    Preconditions: Block implementation complete
    Steps:
      1. grep -n "unsafe" src/block.rs | head -5
      2. grep -B2 "unsafe" src/block.rs | grep -c "SAFETY"
      3. Assert: SAFETY comment count >= 1
    Expected Result: Unsafe blocks have safety comments
    Evidence: grep output
  ```

  **Commit**: YES
  - Message: `feat(mini-lsm): implement block encoding with unsafe zero-copy parsing`
  - Files: `src/block.rs`
  - Pre-commit: `cargo test block`

---

- [ ] 4. MemTable with Concurrent SkipList

  **What to do**:
  - Create `src/memtable.rs` with MemTable struct
  - Use `crossbeam_skiplist::SkipMap<Bytes, Bytes>`
  - Track approximate size with `AtomicUsize`
  - Implement get/put/delete (delete = tombstone)
  - Create MemTableIterator using `ouroboros` for self-referential struct
  - Support scan (range query)

  **TDD Sequence**:
  1. RED: Test put and get single key
  2. GREEN: Implement basic get/put
  3. RED: Test concurrent puts from multiple threads
  4. GREEN: Verify SkipMap handles concurrency
  5. RED: Test delete creates tombstone
  6. GREEN: Implement delete as put(key, empty)
  7. RED: Test approximate_size increases
  8. GREEN: Implement AtomicUsize tracking
  9. RED: Test iterator yields sorted order
  10. GREEN: Implement MemTableIterator with ouroboros

  **Interior Mutability Learning Point**:
  ```rust
  pub struct MemTable {
      map: Arc<SkipMap<Bytes, Bytes>>,
      approximate_size: Arc<AtomicUsize>,  // Interior mutability!
      id: usize,
  }
  // SkipMap allows concurrent mutation without &mut self
  ```

  **Must NOT do**:
  - Don't implement WAL integration here (separate task)
  - Don't add MVCC/versioning

  **Recommended Agent Profile**:
  - **Category**: `ultrabrain`
    - Reason: Concurrent data structures and self-referential types
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 3, 5)
  - **Blocks**: Tasks 10, 11
  - **Blocked By**: Task 2

  **References**:
  - `mini-redis/src/db.rs:19` - SkipMap usage pattern
  - https://github.com/skyzh/mini-lsm/blob/main/mini-lsm/src/mem_table.rs - MemTable structure
  - https://docs.rs/ouroboros - Self-referential struct patterns

  **Acceptance Criteria**:
  - [ ] `cargo test memtable` passes (6+ tests)
  - [ ] Concurrent writes from 4 threads succeed
  - [ ] Delete returns None on subsequent get
  - [ ] Iterator yields keys in lexicographic order
  - [ ] approximate_size within 10% of actual

  **Agent-Executed QA Scenarios**:
  ```
  Scenario: Concurrent write test
    Tool: Bash
    Preconditions: Task 2 complete
    Steps:
      1. cargo test memtable::tests::test_concurrent_writes --lib 2>&1
      2. Assert: exit code 0
    Expected Result: No data races, all writes visible
    Evidence: cargo test output

  Scenario: Iterator ordering test
    Tool: Bash
    Preconditions: MemTable complete
    Steps:
      1. cargo test memtable::tests::test_iterator_order --lib 2>&1
      2. Assert: exit code 0
    Expected Result: Keys yielded in sorted order
    Evidence: cargo test output
  ```

  **Commit**: YES
  - Message: `feat(mini-lsm): implement concurrent memtable with skiplist`
  - Files: `src/memtable.rs`
  - Pre-commit: `cargo test memtable`

---

- [ ] 5. Write-Ahead Log (WAL)

  **What to do**:
  - Create `src/wal.rs` with Wal struct
  - Format: `[key_len: u16 | key | value_len: u16 | value | crc32: u32]`
  - Use `BufWriter<File>` for batching writes
  - Implement `sync()` with `file.sync_all()` for durability
  - Implement recovery: read WAL, verify checksums, rebuild MemTable
  - Handle partial writes (truncate at corruption)

  **TDD Sequence**:
  1. RED: Test write single entry and read back
  2. GREEN: Implement write/read
  3. RED: Test sync persists to disk (reopen file, verify)
  4. GREEN: Implement sync_all()
  5. RED: Test recovery rebuilds memtable
  6. GREEN: Implement recover()
  7. RED: Test corrupted entry stops recovery at that point
  8. GREEN: Implement truncation on corruption

  **Disk Format Learning Point**:
  - Fixed header layout for seeking
  - CRC covers key_len + key + value_len + value
  - Recovery reads until EOF or first corruption

  **Must NOT do**:
  - Don't batch multiple WAL files (single file per memtable)
  - Don't compress WAL entries

  **Recommended Agent Profile**:
  - **Category**: `ultrabrain`
    - Reason: File I/O correctness critical for durability
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 3, 4)
  - **Blocks**: Tasks 10, 13
  - **Blocked By**: Task 2

  **References**:
  - https://github.com/skyzh/mini-lsm/blob/main/mini-lsm/src/wal.rs - WAL implementation
  - LevelDB WAL format documentation

  **Acceptance Criteria**:
  - [ ] `cargo test wal` passes (5+ tests)
  - [ ] Write + sync + reopen preserves all entries
  - [ ] Recovery stops at first corrupted entry
  - [ ] CRC mismatch logged and entry skipped
  - [ ] Uses `sync_all()` not just `flush()`

  **Agent-Executed QA Scenarios**:
  ```
  Scenario: WAL durability test
    Tool: Bash
    Preconditions: Task 2 complete
    Steps:
      1. cargo test wal::tests::test_wal_durability --lib 2>&1
      2. Assert: exit code 0
    Expected Result: Data persists across file reopen
    Evidence: cargo test output

  Scenario: WAL recovery test
    Tool: Bash
    Preconditions: WAL complete
    Steps:
      1. cargo test wal::tests::test_recovery --lib 2>&1
      2. Assert: exit code 0
    Expected Result: MemTable rebuilt from WAL
    Evidence: cargo test output

  Scenario: Corruption handling test
    Tool: Bash
    Preconditions: WAL complete
    Steps:
      1. cargo test wal::tests::test_corruption_truncates --lib 2>&1
      2. Assert: exit code 0
    Expected Result: Recovery stops at corruption, keeps valid prefix
    Evidence: cargo test output
  ```

  **Commit**: YES
  - Message: `feat(mini-lsm): implement WAL with crash recovery`
  - Files: `src/wal.rs`
  - Pre-commit: `cargo test wal`

---

### Wave 3: SSTable Layer (Parallel)

- [ ] 6. SSTable Builder

  **What to do**:
  - Create `src/table/mod.rs` and `src/table/builder.rs`
  - SsTableBuilder accumulates key-value pairs
  - Writes blocks when size threshold reached (4KB default)
  - Builds index block with first_key per data block
  - Generates bloom filter from all keys
  - Writes footer with offsets to index and bloom

  **File Format**:
  ```
  [Data Block 0][Data Block 1]...[Data Block N]
  [Index Block: (first_key, offset, len) per block]
  [Bloom Filter Block]
  [Footer: index_offset: u32 | bloom_offset: u32 | magic: u32]
  ```

  **TDD Sequence**:
  1. RED: Test builder accepts key-value pairs
  2. GREEN: Implement add()
  3. RED: Test build creates valid file
  4. GREEN: Implement build() with block flushing
  5. RED: Test index block contains first keys
  6. GREEN: Implement index generation
  7. RED: Test footer has correct offsets
  8. GREEN: Implement footer writing

  **Must NOT do**:
  - Don't implement compression
  - Don't add block cache here (separate task)

  **Recommended Agent Profile**:
  - **Category**: `ultrabrain`
    - Reason: Complex file format layout
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 7, 8, 9)
  - **Blocks**: Task 7, 10
  - **Blocked By**: Tasks 3, 8

  **References**:
  - Task 3 `src/block.rs` - Block encoding
  - https://github.com/skyzh/mini-lsm/blob/main/mini-lsm/src/table/builder.rs - Builder pattern
  - LevelDB table format documentation

  **Acceptance Criteria**:
  - [ ] `cargo test table::builder` passes (4+ tests)
  - [ ] Output file has correct format
  - [ ] Index block parseable
  - [ ] Footer magic number validates file type

  **Agent-Executed QA Scenarios**:
  ```
  Scenario: SSTable file format test
    Tool: Bash
    Preconditions: Task 3 complete
    Steps:
      1. cargo test table::builder::tests::test_build_sstable --lib 2>&1
      2. Assert: exit code 0
    Expected Result: Valid SSTable file created
    Evidence: cargo test output
  ```

  **Commit**: YES
  - Message: `feat(mini-lsm): implement SSTable builder`
  - Files: `src/table/mod.rs`, `src/table/builder.rs`
  - Pre-commit: `cargo test table::builder`

---

- [ ] 7. SSTable Reader & Iterator

  **What to do**:
  - Create `src/table/sstable.rs` with SsTable struct
  - Open and parse SSTable file (validate footer magic)
  - Load index block into memory
  - Implement `get(key)` with index binary search + block read
  - Implement SsTableIterator for full scans
  - Use bloom filter to skip files

  **TDD Sequence**:
  1. RED: Test open validates footer
  2. GREEN: Implement open()
  3. RED: Test get finds existing key
  4. GREEN: Implement get with index lookup
  5. RED: Test get returns None for missing key
  6. GREEN: Implement bloom filter check
  7. RED: Test iterator yields all entries
  8. GREEN: Implement SsTableIterator

  **Must NOT do**:
  - Don't cache blocks here (separate cache task)
  - Don't implement range delete

  **Recommended Agent Profile**:
  - **Category**: `ultrabrain`
    - Reason: Complex read path with multiple components
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 6, 8, 9)
  - **Blocks**: Task 10
  - **Blocked By**: Tasks 3, 6

  **References**:
  - Task 6 `src/table/builder.rs` - File format
  - Task 3 `src/block.rs` - Block decoding
  - https://github.com/skyzh/mini-lsm/blob/main/mini-lsm/src/table.rs - SsTable reader

  **Acceptance Criteria**:
  - [ ] `cargo test table::sstable` passes (5+ tests)
  - [ ] get() returns correct value
  - [ ] Bloom filter rejects definitely-missing keys
  - [ ] Iterator yields entries in sorted order

  **Agent-Executed QA Scenarios**:
  ```
  Scenario: SSTable read test
    Tool: Bash
    Preconditions: Task 6 complete
    Steps:
      1. cargo test table::sstable::tests::test_get --lib 2>&1
      2. Assert: exit code 0
    Expected Result: Keys retrieved correctly
    Evidence: cargo test output

  Scenario: Bloom filter effectiveness
    Tool: Bash
    Preconditions: Task 7 complete
    Steps:
      1. cargo test table::sstable::tests::test_bloom_filter --lib 2>&1
      2. Assert: exit code 0
    Expected Result: Missing keys rejected without block read
    Evidence: cargo test output
  ```

  **Commit**: YES
  - Message: `feat(mini-lsm): implement SSTable reader and iterator`
  - Files: `src/table/sstable.rs`
  - Pre-commit: `cargo test table::sstable`

---

- [ ] 8. Bloom Filter

  **What to do**:
  - Create `src/bloom.rs` with Bloom struct
  - Use farmhash for fast hashing
  - Implement add(key) and may_contain(key)
  - Serialize/deserialize for SSTable storage
  - Configure bits_per_key (10 bits = ~1% false positive)

  **TDD Sequence**:
  1. RED: Test add + may_contain returns true
  2. GREEN: Implement basic bloom
  3. RED: Test definitely-missing key returns false
  4. GREEN: Verify hash distribution
  5. RED: Test serialization roundtrip
  6. GREEN: Implement encode/decode

  **Must NOT do**:
  - Don't implement counting bloom filter
  - Don't use cryptographic hashes (too slow)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Well-defined algorithm, straightforward
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 6, 7, 9)
  - **Blocks**: Task 6
  - **Blocked By**: Task 2

  **References**:
  - https://github.com/skyzh/mini-lsm/blob/main/mini-lsm/src/bloom.rs - Bloom implementation
  - https://docs.rs/farmhash - Hash function docs

  **Acceptance Criteria**:
  - [ ] `cargo test bloom` passes (4+ tests)
  - [ ] False positive rate < 2% with 10 bits/key
  - [ ] Serialization preserves filter state

  **Agent-Executed QA Scenarios**:
  ```
  Scenario: Bloom filter accuracy
    Tool: Bash
    Preconditions: Task 2 complete
    Steps:
      1. cargo test bloom::tests::test_false_positive_rate --lib 2>&1
      2. Assert: exit code 0
    Expected Result: False positive rate acceptable
    Evidence: cargo test output
  ```

  **Commit**: YES
  - Message: `feat(mini-lsm): implement bloom filter`
  - Files: `src/bloom.rs`
  - Pre-commit: `cargo test bloom`

---

- [ ] 9. Block Cache (LRU)

  **What to do**:
  - Create `src/cache.rs` with BlockCache struct
  - Use `lru` crate with `parking_lot::Mutex` for thread safety
  - Cache key: (sst_id, block_offset)
  - Implement get_or_load() pattern
  - Configurable capacity (default 64MB)

  **TDD Sequence**:
  1. RED: Test get returns None for uncached
  2. GREEN: Implement basic cache
  3. RED: Test put + get returns cached block
  4. GREEN: Implement put/get
  5. RED: Test eviction when capacity exceeded
  6. GREEN: Configure LRU eviction
  7. RED: Test concurrent access
  8. GREEN: Add Mutex wrapper

  **Interior Mutability Learning Point**:
  ```rust
  pub struct BlockCache {
      cache: Mutex<LruCache<(usize, u64), Arc<Block>>>,
      capacity: usize,
  }
  // Mutex allows mutation through shared reference
  ```

  **Must NOT do**:
  - Don't implement sharded cache (optimization for later)
  - Don't use async locks

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Straightforward caching pattern
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 6, 7, 8)
  - **Blocks**: Task 10
  - **Blocked By**: Task 3

  **References**:
  - https://docs.rs/lru - LRU cache API
  - Task 3 `src/block.rs` - Block type to cache

  **Acceptance Criteria**:
  - [ ] `cargo test cache` passes (4+ tests)
  - [ ] Cache hit returns same Arc<Block>
  - [ ] Eviction occurs at capacity
  - [ ] No deadlocks under concurrent access

  **Agent-Executed QA Scenarios**:
  ```
  Scenario: Cache eviction test
    Tool: Bash
    Preconditions: Task 3 complete
    Steps:
      1. cargo test cache::tests::test_eviction --lib 2>&1
      2. Assert: exit code 0
    Expected Result: Oldest entries evicted at capacity
    Evidence: cargo test output
  ```

  **Commit**: YES
  - Message: `feat(mini-lsm): implement LRU block cache`
  - Files: `src/cache.rs`
  - Pre-commit: `cargo test cache`

---

### Wave 4: Storage Engine

- [ ] 10. LSM Storage Engine (Read/Write Path)

  **What to do**:
  - Create `src/lsm_storage.rs` with LsmStorage struct
  - Manage: current memtable, immutable memtables, L0 SSTables, L1+ SSTables
  - Implement put(key, value) → write to WAL + memtable
  - Implement get(key) → search memtable → immutables → L0 → L1+
  - Implement delete(key) → put tombstone
  - Use `Arc<RwLock<LsmStorageInner>>` for state

  **Read Path Order**:
  1. Current memtable
  2. Immutable memtables (newest first)
  3. L0 SSTables (newest first, all must be checked - overlapping)
  4. L1+ SSTables (binary search by key range, non-overlapping)

  **TDD Sequence**:
  1. RED: Test put + get single key
  2. GREEN: Implement basic put/get
  3. RED: Test delete makes key invisible
  4. GREEN: Implement tombstone handling
  5. RED: Test read path order (newer data shadows older)
  6. GREEN: Implement proper read ordering

  **Interior Mutability Learning Point**:
  ```rust
  pub struct LsmStorage {
      inner: Arc<RwLock<LsmStorageInner>>,
  }
  
  struct LsmStorageInner {
      memtable: Arc<MemTable>,
      imm_memtables: Vec<Arc<MemTable>>,
      l0_sstables: Vec<Arc<SsTable>>,
      levels: Vec<Vec<Arc<SsTable>>>,  // L1+
  }
  ```

  **Must NOT do**:
  - Don't implement compaction here (separate task)
  - Don't implement transactions

  **Recommended Agent Profile**:
  - **Category**: `ultrabrain`
    - Reason: Complex state management, critical correctness
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4 (sequential)
  - **Blocks**: Tasks 11, 12, 13, 14, 15
  - **Blocked By**: Tasks 4, 5, 7, 9

  **References**:
  - Task 4 `src/memtable.rs` - MemTable API
  - Task 5 `src/wal.rs` - WAL API  
  - Task 7 `src/table/sstable.rs` - SSTable API
  - Task 9 `src/cache.rs` - Block cache
  - https://github.com/skyzh/mini-lsm/blob/main/mini-lsm/src/lsm_storage.rs - Storage engine

  **Acceptance Criteria**:
  - [ ] `cargo test lsm_storage` passes (6+ tests)
  - [ ] put + get roundtrip works
  - [ ] delete makes key return None
  - [ ] Newer writes shadow older writes
  - [ ] Read path checks all levels in order

  **Agent-Executed QA Scenarios**:
  ```
  Scenario: Basic CRUD operations
    Tool: Bash
    Preconditions: Tasks 4, 5, 7, 9 complete
    Steps:
      1. cargo test lsm_storage::tests::test_crud --lib 2>&1
      2. Assert: exit code 0
    Expected Result: Put, get, delete all work
    Evidence: cargo test output

  Scenario: Read path ordering
    Tool: Bash
    Preconditions: LSM storage complete
    Steps:
      1. cargo test lsm_storage::tests::test_read_path_order --lib 2>&1
      2. Assert: exit code 0
    Expected Result: Newer data shadows older
    Evidence: cargo test output
  ```

  **Commit**: YES
  - Message: `feat(mini-lsm): implement LSM storage engine core`
  - Files: `src/lsm_storage.rs`, `src/lib.rs`
  - Pre-commit: `cargo test lsm_storage`

---

- [ ] 11. Immutable MemTable Queue & Flush

  **What to do**:
  - Implement memtable rotation when size threshold reached
  - Move current memtable to immutable queue
  - Create new empty memtable with new WAL
  - Implement flush: write immutable memtable to L0 SSTable
  - Delete WAL after successful flush

  **TDD Sequence**:
  1. RED: Test memtable rotates at threshold
  2. GREEN: Implement rotation
  3. RED: Test immutable queue ordered newest-first
  4. GREEN: Implement queue ordering
  5. RED: Test flush creates SSTable
  6. GREEN: Implement flush
  7. RED: Test WAL deleted after flush
  8. GREEN: Implement WAL cleanup

  **Must NOT do**:
  - Don't implement background flush thread (manual trigger for now)
  - Don't implement flush parallelism

  **Recommended Agent Profile**:
  - **Category**: `ultrabrain`
    - Reason: State transitions, durability correctness
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4 (after Task 10)
  - **Blocks**: Task 12
  - **Blocked By**: Tasks 4, 10

  **References**:
  - Task 10 `src/lsm_storage.rs` - Storage state
  - Task 6 `src/table/builder.rs` - SSTable creation

  **Acceptance Criteria**:
  - [ ] `cargo test lsm_storage::flush` passes (4+ tests)
  - [ ] Rotation occurs at size threshold
  - [ ] Flush creates valid L0 SSTable
  - [ ] Old WAL cleaned up

  **Agent-Executed QA Scenarios**:
  ```
  Scenario: Memtable rotation test
    Tool: Bash
    Preconditions: Task 10 complete
    Steps:
      1. cargo test lsm_storage::tests::test_memtable_rotation --lib 2>&1
      2. Assert: exit code 0
    Expected Result: Memtable rotates at threshold
    Evidence: cargo test output
  ```

  **Commit**: YES
  - Message: `feat(mini-lsm): implement memtable rotation and flush`
  - Files: `src/lsm_storage.rs`
  - Pre-commit: `cargo test lsm_storage`

---

- [ ] 12. Leveled Compaction Controller

  **What to do**:
  - Create `src/compact/mod.rs` and `src/compact/leveled.rs`
  - Implement compaction trigger logic:
    - L0 → L1 when L0 file count > 4
    - Li → Li+1 when level size > target size
  - Implement file selection: oldest L0 + overlapping L1 files
  - Implement merge-sort iterator for compaction
  - Write new non-overlapping SSTables to target level
  - Update manifest and delete old files

  **TDD Sequence**:
  1. RED: Test L0 trigger at file count
  2. GREEN: Implement trigger check
  3. RED: Test file selection finds overlapping
  4. GREEN: Implement overlap detection
  5. RED: Test merge produces sorted output
  6. GREEN: Implement MergeIterator
  7. RED: Test old files removed
  8. GREEN: Implement cleanup

  **Compaction Algorithm**:
  ```
  1. Select oldest SSTable from source level
  2. Find all overlapping SSTables in target level
  3. Merge-sort all selected tables
  4. Write new SSTables (split at size threshold)
  5. Atomically update manifest
  6. Delete old SSTable files
  ```

  **Must NOT do**:
  - Don't implement background compaction thread
  - Don't implement tiered compaction (leveled only)

  **Recommended Agent Profile**:
  - **Category**: `ultrabrain`
    - Reason: Complex algorithm, correctness critical
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4 (after Task 11)
  - **Blocks**: Task 13
  - **Blocked By**: Tasks 10, 11

  **References**:
  - https://github.com/skyzh/mini-lsm/blob/main/mini-lsm/src/compact/leveled.rs - Leveled compaction
  - LevelDB compaction documentation
  - Task 10 `src/lsm_storage.rs` - Storage state

  **Acceptance Criteria**:
  - [ ] `cargo test compact` passes (5+ tests)
  - [ ] L0 compaction triggers at threshold
  - [ ] Overlapping file detection correct
  - [ ] Merge produces sorted, deduplicated output
  - [ ] Old files deleted after compaction

  **Agent-Executed QA Scenarios**:
  ```
  Scenario: Compaction trigger test
    Tool: Bash
    Preconditions: Task 11 complete
    Steps:
      1. cargo test compact::tests::test_l0_trigger --lib 2>&1
      2. Assert: exit code 0
    Expected Result: Compaction triggers at L0 threshold
    Evidence: cargo test output

  Scenario: Merge correctness test
    Tool: Bash
    Preconditions: Compaction complete
    Steps:
      1. cargo test compact::tests::test_merge_iterator --lib 2>&1
      2. Assert: exit code 0
    Expected Result: Merged output sorted and deduplicated
    Evidence: cargo test output
  ```

  **Commit**: YES
  - Message: `feat(mini-lsm): implement leveled compaction controller`
  - Files: `src/compact/mod.rs`, `src/compact/leveled.rs`
  - Pre-commit: `cargo test compact`

---

### Wave 5: Integration & Polish

- [ ] 13. Crash Recovery Integration Test

  **What to do**:
  - Create `tests/crash_recovery.rs` integration test
  - Test: write data → simulate crash → recover → verify data
  - Test: partial WAL write → recover → verify prefix recovered
  - Test: write 1GB data → restart → verify consistency
  - Ensure all WAL files replayed on startup

  **TDD Sequence**:
  1. RED: Write integration test for basic recovery
  2. GREEN: Verify recovery works
  3. RED: Write test for 1GB data consistency
  4. GREEN: Verify large dataset recovery
  5. RED: Write test for partial WAL
  6. GREEN: Verify truncation handling

  **Must NOT do**:
  - Don't test network failures (no network layer)
  - Don't test disk full (out of scope)

  **Recommended Agent Profile**:
  - **Category**: `ultrabrain`
    - Reason: Integration complexity, correctness verification
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 5 (critical path)
  - **Blocks**: Tasks 14, 15
  - **Blocked By**: Tasks 5, 10, 12

  **References**:
  - Task 5 `src/wal.rs` - WAL recovery
  - Task 10 `src/lsm_storage.rs` - Storage open/recovery
  - `mini-redis/tests/integration.rs` - Integration test patterns

  **Acceptance Criteria**:
  - [ ] `cargo test --test crash_recovery` passes
  - [ ] 1GB write + restart + verify succeeds
  - [ ] Partial WAL recovered correctly
  - [ ] All memtable data survives restart

  **Agent-Executed QA Scenarios**:
  ```
  Scenario: 1GB crash recovery test
    Tool: Bash
    Preconditions: Tasks 5, 10, 12 complete
    Steps:
      1. cargo test --test crash_recovery test_1gb_recovery -- --nocapture 2>&1
      2. Assert: exit code 0
      3. Assert: output contains "1GB data verified"
    Expected Result: All data recoverable after restart
    Evidence: cargo test output

  Scenario: Partial WAL recovery
    Tool: Bash
    Preconditions: Crash recovery tests written
    Steps:
      1. cargo test --test crash_recovery test_partial_wal -- --nocapture 2>&1
      2. Assert: exit code 0
    Expected Result: Valid prefix of WAL recovered
    Evidence: cargo test output
  ```

  **Commit**: YES
  - Message: `test(mini-lsm): add crash recovery integration tests`
  - Files: `tests/crash_recovery.rs`
  - Pre-commit: `cargo test --test crash_recovery`

---

- [ ] 14. Criterion Benchmarks

  **What to do**:
  - Create `benches/lsm_bench.rs` with criterion benchmarks
  - Benchmark: sequential write throughput (ops/sec, MB/sec)
  - Benchmark: random read latency (p50, p99)
  - Benchmark: scan throughput
  - Benchmark: compaction speed

  **Benchmarks**:
  ```rust
  // Sequential writes
  fn bench_seq_write(c: &mut Criterion) {
      c.bench_function("seq_write_1m", |b| {
          b.iter(|| {
              // Write 1M 100-byte values
          });
      });
  }
  
  // Random reads
  fn bench_random_read(c: &mut Criterion) {
      // Pre-populate, then random reads
  }
  ```

  **Must NOT do**:
  - Don't optimize prematurely based on benchmarks
  - Don't add complex profiling

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Straightforward benchmark setup
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 5 (with Task 15)
  - **Blocks**: None
  - **Blocked By**: Task 10

  **References**:
  - https://docs.rs/criterion - Criterion API
  - Task 10 `src/lsm_storage.rs` - Storage API to benchmark

  **Acceptance Criteria**:
  - [ ] `cargo bench` runs without errors
  - [ ] Reports ops/sec for write benchmark
  - [ ] Reports latency percentiles for read benchmark
  - [ ] HTML report generated in `target/criterion/`

  **Agent-Executed QA Scenarios**:
  ```
  Scenario: Benchmarks run successfully
    Tool: Bash
    Preconditions: Task 10 complete
    Steps:
      1. cd /Users/byzantium/github/learn/rust/mini-lsm
      2. cargo bench --bench lsm_bench -- --quick 2>&1
      3. Assert: exit code 0
      4. Assert: output contains "seq_write"
    Expected Result: Benchmarks complete with metrics
    Evidence: cargo bench output

  Scenario: HTML report generated
    Tool: Bash
    Preconditions: Benchmarks complete
    Steps:
      1. ls target/criterion/*/report/index.html 2>/dev/null | wc -l
      2. Assert: count >= 1
    Expected Result: Criterion HTML reports exist
    Evidence: file listing
  ```

  **Commit**: YES
  - Message: `perf(mini-lsm): add criterion benchmarks`
  - Files: `benches/lsm_bench.rs`
  - Pre-commit: `cargo bench -- --quick`

---

- [ ] 15. CLI Tool

  **What to do**:
  - Create `src/bin/mini-lsm-cli.rs`
  - Commands: put <key> <value>, get <key>, delete <key>, scan, compact, stats
  - Use simple argument parsing (no clap, keep deps minimal)
  - Display storage statistics (memtable size, SSTable count, etc.)

  **TDD Sequence**:
  1. RED: Test CLI parses put command
  2. GREEN: Implement argument parsing
  3. RED: Test get returns stored value
  4. GREEN: Wire up to LsmStorage
  5. RED: Test stats shows info
  6. GREEN: Implement stats display

  **Must NOT do**:
  - Don't add REPL mode (batch commands only)
  - Don't add complex formatting

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple CLI wrapper
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 5 (with Task 14)
  - **Blocks**: None
  - **Blocked By**: Task 10

  **References**:
  - `mini-redis/src/main.rs` - CLI patterns
  - Task 10 `src/lsm_storage.rs` - Storage API

  **Acceptance Criteria**:
  - [ ] `cargo build --bin mini-lsm-cli` succeeds
  - [ ] `./mini-lsm-cli put foo bar` works
  - [ ] `./mini-lsm-cli get foo` returns "bar"
  - [ ] `./mini-lsm-cli stats` shows storage info

  **Agent-Executed QA Scenarios**:
  ```
  Scenario: CLI put and get
    Tool: Bash
    Preconditions: Task 10 complete
    Steps:
      1. cd /Users/byzantium/github/learn/rust/mini-lsm
      2. cargo build --bin mini-lsm-cli 2>&1
      3. ./target/debug/mini-lsm-cli put testkey testvalue 2>&1
      4. ./target/debug/mini-lsm-cli get testkey 2>&1
      5. Assert: output contains "testvalue"
    Expected Result: CLI CRUD works
    Evidence: CLI output

  Scenario: CLI stats command
    Tool: Bash
    Preconditions: CLI built
    Steps:
      1. ./target/debug/mini-lsm-cli stats 2>&1
      2. Assert: output contains "memtable" or "sstable"
    Expected Result: Stats displayed
    Evidence: CLI output
  ```

  **Commit**: YES
  - Message: `feat(mini-lsm): add CLI tool`
  - Files: `src/bin/mini-lsm-cli.rs`
  - Pre-commit: `cargo build --bin mini-lsm-cli`

---

## Commit Strategy

| After Task | Message | Files | Verification |
|------------|---------|-------|--------------|
| 1 | `feat(mini-lsm): initialize project with dependencies` | `mini-lsm/` | `cargo check` |
| 2 | `feat(mini-lsm): add error types and key utilities` | `src/error.rs`, `src/key.rs` | `cargo test` |
| 3 | `feat(mini-lsm): implement block encoding with unsafe zero-copy` | `src/block.rs` | `cargo test block` |
| 4 | `feat(mini-lsm): implement concurrent memtable` | `src/memtable.rs` | `cargo test memtable` |
| 5 | `feat(mini-lsm): implement WAL with crash recovery` | `src/wal.rs` | `cargo test wal` |
| 6 | `feat(mini-lsm): implement SSTable builder` | `src/table/` | `cargo test table::builder` |
| 7 | `feat(mini-lsm): implement SSTable reader` | `src/table/sstable.rs` | `cargo test table::sstable` |
| 8 | `feat(mini-lsm): implement bloom filter` | `src/bloom.rs` | `cargo test bloom` |
| 9 | `feat(mini-lsm): implement LRU block cache` | `src/cache.rs` | `cargo test cache` |
| 10 | `feat(mini-lsm): implement LSM storage engine core` | `src/lsm_storage.rs` | `cargo test lsm_storage` |
| 11 | `feat(mini-lsm): implement memtable rotation and flush` | `src/lsm_storage.rs` | `cargo test lsm_storage` |
| 12 | `feat(mini-lsm): implement leveled compaction` | `src/compact/` | `cargo test compact` |
| 13 | `test(mini-lsm): add crash recovery integration tests` | `tests/` | `cargo test --test crash_recovery` |
| 14 | `perf(mini-lsm): add criterion benchmarks` | `benches/` | `cargo bench -- --quick` |
| 15 | `feat(mini-lsm): add CLI tool` | `src/bin/` | `cargo build --bin mini-lsm-cli` |

---

## Success Criteria

### Verification Commands
```bash
# All tests pass
cargo test

# Benchmarks run
cargo bench -- --quick

# No clippy warnings
cargo clippy -- -D warnings

# Build release binary
cargo build --release --bin mini-lsm-cli

# 1GB crash recovery (run manually for final validation)
cargo test --test crash_recovery test_1gb_recovery --release -- --nocapture
```

### Final Checklist
- [ ] All 15 tasks completed with passing tests
- [ ] `cargo test` passes (50+ tests expected)
- [ ] `cargo bench` produces performance reports
- [ ] 1GB data survives process restart
- [ ] Compaction reduces L0 file count
- [ ] Unsafe code has SAFETY comments
- [ ] No clippy warnings

### Key Learning Outcomes Verified
- [ ] Interior Mutability: `Arc<AtomicUsize>`, `Arc<RwLock<T>>`, `Mutex<LruCache>`
- [ ] Disk Format: SSTable layout, block encoding, WAL format
- [ ] Unsafe Code: Zero-copy block parsing, pointer arithmetic
- [ ] Concurrent Data Structures: SkipMap, self-referential iterators
