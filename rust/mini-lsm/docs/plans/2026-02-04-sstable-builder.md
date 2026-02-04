# SsTableBuilder Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement SSTable builder and reader with block/index/bloom/footer layout using `BlockBuilder`.

**Architecture:** `SsTableBuilder` writes data blocks via `BlockBuilder`, tracks block metadata, then emits an index block and bloom filter before a fixed-size footer with checksums. `SsTable` reads the footer, index, and bloom filter for lookups.

**Tech Stack:** Rust 2024, `bytes`, `crc32fast`, `farmhash`, standard I/O.

---

### Task 1: Define SSTable data structures

**Files:**
- Modify: `src/sstable.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn test_sstable_footer_round_trip() {
    let footer = Footer::new(123, 1, 2, 3);
    let encoded = footer.encode();
    let decoded = Footer::decode(&encoded).unwrap();
    assert_eq!(decoded.file_size, 123);
    assert_eq!(decoded.data_checksum, 1);
    assert_eq!(decoded.index_checksum, 2);
    assert_eq!(decoded.bloom_checksum, 3);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_sstable_footer_round_trip -v`
Expected: FAIL with "Footer not found" or missing symbol.

**Step 3: Write minimal implementation**

```rust
struct Footer { file_size: u64, data_checksum: u32, index_checksum: u32, bloom_checksum: u32 }
impl Footer {
    fn new(file_size: u64, data_checksum: u32, index_checksum: u32, bloom_checksum: u32) -> Self { Self { file_size, data_checksum, index_checksum, bloom_checksum } }
    fn encode(&self) -> Vec<u8> { /* fixed-size LE */ }
    fn decode(buf: &[u8]) -> Result<Self> { /* parse LE */ }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_sstable_footer_round_trip -v`
Expected: PASS

**Step 5: Commit**

```bash
git add src/sstable.rs
git commit -m "Add SSTable footer encoding"
```

### Task 2: Implement bloom filter encode/decode

**Files:**
- Modify: `src/sstable.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn test_bloom_filter_round_trip() {
    let mut bloom = BloomFilter::new(128, 3);
    bloom.add(b"alpha");
    bloom.add(b"beta");
    let encoded = bloom.encode();
    let decoded = BloomFilter::decode(&encoded).unwrap();
    assert!(decoded.may_contain(b"alpha"));
    assert!(decoded.may_contain(b"beta"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_bloom_filter_round_trip -v`
Expected: FAIL with missing symbols.

**Step 3: Write minimal implementation**

```rust
struct BloomFilter { num_bits: u32, num_hashes: u8, bits: Vec<u8> }
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_bloom_filter_round_trip -v`
Expected: PASS

**Step 5: Commit**

```bash
git add src/sstable.rs
git commit -m "Add bloom filter encoding"
```

### Task 3: Build data blocks and index block

**Files:**
- Modify: `src/sstable.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn test_builder_splits_blocks_and_writes_index() {
    let mut builder = SsTableBuilder::new(64);
    builder.add(b"a", b"1");
    builder.add(b"b", b"2");
    builder.add(b"c", b"3");
    let (data, sstable) = builder.build_to_bytes(1).unwrap();
    assert!(!data.is_empty());
    assert_eq!(sstable.block_meta.len(), 1);
    assert_eq!(sstable.block_meta[0].first_key, b"a");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_builder_splits_blocks_and_writes_index -v`
Expected: FAIL with missing builder API.

**Step 3: Write minimal implementation**

```rust
struct BlockMeta { offset: u32, first_key: Vec<u8> }
struct SsTableBuilder { block_size: usize, current: BlockBuilder, block_meta: Vec<BlockMeta> }
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_builder_splits_blocks_and_writes_index -v`
Expected: PASS

**Step 5: Commit**

```bash
git add src/sstable.rs
git commit -m "Add SsTableBuilder block and index logic"
```

### Task 4: Final file layout and reader construction

**Files:**
- Modify: `src/sstable.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn test_builder_writes_footer_checksums() {
    let mut builder = SsTableBuilder::new(128);
    builder.add(b"k1", b"v1");
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("1.sst");
    let sstable = builder.build(1, &path, None).unwrap();
    let footer = Footer::read_from_path(&path).unwrap();
    assert_eq!(footer.file_size as usize, std::fs::metadata(&path).unwrap().len() as usize);
    assert_eq!(footer.index_checksum, sstable.footer.index_checksum);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_builder_writes_footer_checksums -v`
Expected: FAIL with missing APIs.

**Step 3: Write minimal implementation**

```rust
impl SsTableBuilder {
    pub fn build(self, id: usize, path: impl AsRef<Path>, cache: Option<Arc<BlockCache>>) -> Result<SsTable> { /* write file layout */ }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_builder_writes_footer_checksums -v`
Expected: PASS

**Step 5: Commit**

```bash
git add src/sstable.rs
git commit -m "Write SSTable file layout with footer"
```

### Task 5: Cleanup and verification

**Files:**
- Modify: `src/sstable.rs`

**Step 1: Run lints/tests**

Run: `cargo test`
Expected: PASS

**Step 2: Run LSP diagnostics**

Check: `lsp_diagnostics` on `src/sstable.rs`
Expected: no errors

**Step 3: Commit**

```bash
git add src/sstable.rs
git commit -m "Refine SSTable builder tests"
```
