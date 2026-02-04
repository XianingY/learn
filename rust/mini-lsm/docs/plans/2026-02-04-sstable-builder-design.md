# SsTable and SsTableBuilder design

## Goal
Define an SSTable writer (builder) and reader layout that uses the existing
`BlockBuilder` format for data blocks and a separate index block, plus a bloom
filter and footer. The on-disk layout is:

[Block 1] ... [Block N] [Index Block] [Bloom Filter] [Footer]

## Assumptions
- Keys are non-empty byte slices and sorted when added to the builder.
- `BlockBuilder` remains the only block encoding used for data and index.
- Bloom filter is optional for correctness but stored for fast negative lookups.
- Footer includes file size and checksums for data, index, and bloom sections.

## Options
1. **Index-as-Block (recommended)**
   - Encode index entries with `BlockBuilder`, using `first_key` as the key and
     a 4-byte little-endian offset as the value. Index block encoding matches
     data blocks, so reading can reuse `Block::decode` and `get_entry`.
   - Pros: reuse existing encoding logic, minimal custom parsing.
   - Cons: offset is stored as value bytes rather than a typed field.

2. Custom index format
   - Encode entries as `[key_len u16][key][offset u32]` in a custom vector.
   - Pros: minimal overhead and explicit types.
   - Cons: duplicate parsing logic and no reuse of `BlockBuilder`.

Recommendation: Option 1. It aligns with the reuse constraint and keeps the
index parsing aligned with data block code.

## Data encoding
### Data blocks
Same as `BlockBuilder` / `Block` encoding:

u16 key_len | key bytes | u16 value_len | value bytes

### Index block
Each entry uses the same block encoding. Value bytes are a 4-byte little-endian
offset of the corresponding data block from the file start.

### Bloom filter
Implement a simple bitset bloom filter:

- `m` bits stored as `Vec<u8>`
- `k` hash functions derived from `farmhash::hash64` with different salts

Encoding:

[u32 num_bits][u8 num_hashes][bitset bytes]

### Footer
Footer fields are little-endian and fixed size:

[u64 file_size][u32 data_checksum][u32 index_checksum][u32 bloom_checksum]

Checksums use `crc32fast` and are computed over the raw bytes of each section.

## Reader behavior
`SsTable` loads:

- `block_meta`: offset + first_key for each data block (from index block)
- bloom filter bytes for `may_contain`
- footer for verification and file sizing

`SsTable::get(key)` (optional in this phase) uses bloom first, then binary search
over `block_meta` to locate a data block, and then scans entries in that block.

## Tests
Unit tests should verify:

- Builder splits blocks on size boundary and records offsets.
- Index block stores first key and correct offsets.
- Bloom filter encodes/decodes and `may_contain` behavior for inserted keys.
- Footer file size and checksums match computed values.
