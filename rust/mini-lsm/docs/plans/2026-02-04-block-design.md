# Block and BlockBuilder design

## Goal
Define an in-memory block representation and a builder that serializes to the
on-disk layout used by SSTables. The block layout is:

[data bytes...] [offsets u16...] [num_entries u16]

Offsets are u16 positions into the data area. All integer fields are encoded
little-endian to match `bytes::Buf` access via `get_u16_le`.

## Data encoding
Each entry in the data area is length-prefixed:

u16 key_len | key bytes | u16 value_len | value bytes

Offsets point to the start of each entry within the data area. This format keeps
decoding simple without requiring external schema knowledge.

## Block
`Block` stores the data region and the entry offsets:

- `data: Bytes` for the concatenated entry bytes.
- `offsets: Vec<u16>` for entry starts.

`encode()` returns a `Vec<u8>` consisting of `data`, offsets in order, and the
final `num_entries` u16. `decode()` takes a `Bytes` (or byte slice) and parses
the trailer to recover `num_entries`, then reads `num_entries` offsets from the
end, and treats the remaining prefix as `data`.

## BlockBuilder
`BlockBuilder` collects entries until a target size (e.g. 4096 bytes). It tracks:

- `data: Vec<u8>`
- `offsets: Vec<u16>`
- `target_size: usize`

`add(key, value)` computes the additional size for the entry and the final
offset/count trailer: `data_len + entry_len + (offsets_len + 2) <= target_size`.
On success, it appends the current data length as the next offset, then writes
the entry bytes. It returns `true` when added, `false` otherwise.

`new(target_size)` creates an empty builder. A `build()` method can return a
`Block` directly or callers can call `encode()` on `Block` built from internal
state.

## Tests
Unit tests should verify:

- The encoded layout matches the specification (data prefix, offsets, count).
- Round-trip `encode` then `decode` yields identical `data` and `offsets`.
- `add` respects target size boundary and offset placement.
