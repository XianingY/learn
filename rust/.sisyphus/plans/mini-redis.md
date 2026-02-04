# Mini-Redis: Production-Quality Redis Implementation in Rust

## TL;DR

> **Quick Summary**: Build a fully functional Mini-Redis server in Rust using Tokio, focusing on zero-copy RESP parsing with lifetimes to master Rust's ownership system. Implements GET, SET (with TTL), PUBLISH, and SUBSCRIBE commands.
> 
> **Deliverables**:
> - Zero-copy RESP parser with `Frame<'a>` enum demonstrating lifetime concepts
> - Frame-based connection layer with BytesMut buffering
> - Concurrent key-value store with TTL expiration (DashMap + BTreeSet)
> - Pub/Sub system using Tokio broadcast channels
> - Server binary compatible with `redis-cli`
> - Client library for programmatic access
> 
> **Estimated Effort**: Large (8-12 focused sessions)
> **Parallel Execution**: YES - 3 waves
> **Critical Path**: Task 1 (Project Setup) → Task 2 (RESP Parser) → Task 4 (Connection) → Task 6 (Commands) → Task 9 (Server)

---

## Context

### Original Request
Create a high-fidelity, production-quality implementation of Mini-Redis in Rust with:
- RESP parser using zero-copy (lifetimes)
- Connection layer with frame-based interface
- GET, SET, PUBLISH, SUBSCRIBE commands
- Concurrent state with TTL support
- Client/Server binaries

### User Profile
- **Experience**: Intermediate Rustacean
- **Prior Work**: Built Axum + SQLite web server (demonstrates Tokio, async, error handling)
- **Learning Goal**: Master lifetimes and low-level buffer management
- **Existing Code**: `/Users/byzantium/github/learn/rust/web-server/` shows modular organization patterns

### Research Findings

**RESP Protocol** (from librarian):
- Type-prefixed: `+` Simple String, `-` Error, `:` Integer, `$` Bulk String, `*` Array
- CRLF (`\r\n`) terminated
- Bulk strings are length-prefixed: `$6\r\nfoobar\r\n`
- Arrays: `*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n`

**Zero-Copy Patterns** (from redis-protocol.rs):
```rust
pub enum BorrowedFrame<'a> {
    SimpleString(&'a [u8]),
    Error(&'a str),
    Integer(i64),
    BulkString(&'a [u8]),
    Array(&'a [BorrowedFrame<'a>]),
    Null,
}
```

**Tokio Production Patterns** (from mini-redis reference):
- Graceful shutdown: `broadcast::channel` + `mpsc::channel` for coordination
- Buffering: `BytesMut` with `AsyncReadExt::read_buf`
- Two-phase parsing: `check()` validates, `parse()` constructs
- TTL: `BTreeSet<(Instant, Key)>` + `Notify` for efficient expiration

---

## Work Objectives

### Core Objective
Build a Redis-compatible server that teaches lifetime concepts through hands-on zero-copy parsing, while being production-quality enough to handle real workloads.

### Concrete Deliverables
1. `mini-redis/src/frame.rs` - Zero-copy `Frame<'a>` enum with RESP parsing
2. `mini-redis/src/connection.rs` - TcpStream wrapper with frame read/write
3. `mini-redis/src/cmd/` - Command implementations (GET, SET, PUBLISH, SUBSCRIBE)
4. `mini-redis/src/db.rs` - Concurrent KV store with TTL
5. `mini-redis/src/server.rs` - Main server loop with graceful shutdown
6. `mini-redis/src/client.rs` - Client library
7. `mini-redis/src/bin/server.rs` - Server binary
8. `mini-redis/src/bin/cli.rs` - Simple CLI client

### Definition of Done
- [ ] `redis-cli ping` returns `PONG`
- [ ] `redis-cli set foo bar` + `redis-cli get foo` returns `bar`
- [ ] `redis-cli setex temp 1 value` expires after 1 second
- [ ] Two `redis-cli` instances can pub/sub on same channel
- [ ] `cargo test` passes all unit and integration tests
- [ ] `cargo clippy` reports no warnings
- [ ] Server handles 1000+ concurrent connections without panicking

### Must Have
- Zero-copy parser using `Frame<'a>` with explicit lifetime annotations
- Comprehensive error handling with custom error types
- Tracing instrumentation for debugging
- Graceful shutdown on SIGINT/SIGTERM
- Thread-safe state management

### Must NOT Have (Guardrails)
- **NO persistence** (no RDB/AOF - this is in-memory only)
- **NO clustering/replication** (single-node only)
- **NO Lua scripting** (out of scope)
- **NO full Redis command set** (only GET, SET, SETEX, DEL, PING, PUBLISH, SUBSCRIBE, UNSUBSCRIBE)
- **NO premature optimization** - clarity over performance tricks
- **NO unsafe code** - safe Rust only for learning
- **NO external parser libraries** (nom, winnow) - manual parsing to learn lifetimes

---

## Verification Strategy

### Test Decision
- **Infrastructure exists**: NO (new project)
- **Automated tests**: YES (TDD) - critical for learning lifetimes
- **Framework**: Rust's built-in `#[test]` + `#[tokio::test]`

### Test Infrastructure Setup (Task 1)
The project setup task includes test configuration:
- Unit tests in `src/*.rs` files with `#[cfg(test)]` modules
- Integration tests in `tests/` directory
- `cargo test` as the single verification command

### TDD Workflow (Per Task)
Each implementation task follows RED-GREEN-REFACTOR:
1. **RED**: Write failing test that specifies expected behavior
2. **GREEN**: Implement minimum code to pass
3. **REFACTOR**: Clean up while maintaining green

### Agent-Executed QA Scenarios (MANDATORY)
Every task includes redis-cli or cargo-based verification that the agent executes directly.

**Verification Tools:**
| Type | Tool | How Agent Verifies |
|------|------|-------------------|
| Unit Tests | Bash (`cargo test`) | Run tests, assert pass count |
| Integration | Bash (`redis-cli`) | Connect to server, execute commands |
| Server | interactive_bash (tmux) | Start server, verify listening |
| Pub/Sub | Bash (multiple processes) | Publisher + subscriber coordination |

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately):
├── Task 1: Project Setup & Dependencies
└── Task 3: Error Types & Custom Error Handling

Wave 2 (After Wave 1):
├── Task 2: RESP Parser with Lifetimes (depends: 1, 3)
├── Task 5: Database Layer - KV Store (depends: 1, 3)
└── Task 7: Pub/Sub State Management (depends: 1, 3)

Wave 3 (After Wave 2):
├── Task 4: Connection Layer (depends: 2)
├── Task 6: Command Engine (depends: 2, 5)
└── Task 8: TTL & Expiration System (depends: 5)

Wave 4 (After Wave 3):
├── Task 9: Server Binary (depends: 4, 6, 7, 8)
└── Task 10: Client Library (depends: 4)

Wave 5 (After Wave 4):
└── Task 11: Integration Tests & Polish (depends: 9, 10)

Critical Path: 1 → 2 → 4 → 9 → 11
Parallel Speedup: ~35% faster than sequential
```

### Dependency Matrix

| Task | Depends On | Blocks | Can Parallelize With |
|------|------------|--------|---------------------|
| 1 | None | 2, 3, 5, 7 | 3 |
| 2 | 1, 3 | 4, 6 | 5, 7 |
| 3 | None | 2, 5, 6, 7 | 1 |
| 4 | 2 | 9, 10 | 6, 8 |
| 5 | 1, 3 | 6, 8 | 2, 7 |
| 6 | 2, 5 | 9 | 4, 8 |
| 7 | 1, 3 | 9 | 2, 5 |
| 8 | 5 | 9 | 4, 6 |
| 9 | 4, 6, 7, 8 | 11 | 10 |
| 10 | 4 | 11 | 9 |
| 11 | 9, 10 | None | None (final) |

---

## TODOs

### Wave 1: Foundation

- [ ] 1. Project Setup & Dependencies

  **What to do**:
  - Create new Cargo package at `mini-redis/`
  - Configure `Cargo.toml` with exact dependency versions
  - Set up module structure matching mini-redis reference
  - Create lib.rs with module declarations
  - Verify `cargo check` passes

  **Dependencies to add**:
  ```toml
  [dependencies]
  tokio = { version = "1", features = ["full"] }
  bytes = "1"
  tracing = "0.1"
  tracing-subscriber = { version = "0.3", features = ["env-filter"] }
  dashmap = "6"
  thiserror = "2"
  
  [dev-dependencies]
  tokio-test = "0.4"
  ```

  **Must NOT do**:
  - Do not add unnecessary dependencies
  - Do not create any implementation yet (just structure)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Straightforward file creation and configuration
  - **Skills**: [`git-master`]
    - `git-master`: For initial commit after setup

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Task 3)
  - **Blocks**: Tasks 2, 5, 7
  - **Blocked By**: None

  **References**:
  - Pattern: `/Users/byzantium/github/learn/rust/web-server/Cargo.toml` - Existing Tokio project configuration
  - Pattern: `/Users/byzantium/github/learn/rust/web-server/src/main.rs` - Module organization pattern
  - External: `https://github.com/tokio-rs/mini-redis/blob/master/Cargo.toml` - Reference dependencies

  **Acceptance Criteria**:

  **TDD (structure verification):**
  - [ ] `cargo check` in `mini-redis/` exits with code 0
  - [ ] `cargo test` runs (0 tests is OK for setup)

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Project compiles successfully
    Tool: Bash
    Preconditions: mini-redis/ directory exists with Cargo.toml
    Steps:
      1. cd mini-redis && cargo check
      2. Assert: exit code 0
      3. Assert: stdout contains "Finished"
    Expected Result: Project compiles with no errors
    Evidence: Command output captured

  Scenario: All dependencies resolve
    Tool: Bash
    Preconditions: Cargo.toml has all dependencies listed
    Steps:
      1. cd mini-redis && cargo fetch
      2. Assert: exit code 0
      3. Assert: no "failed to fetch" in stderr
    Expected Result: All crates downloaded successfully
    Evidence: Command output captured
  ```

  **Commit**: YES
  - Message: `feat(mini-redis): initialize project with tokio and bytes dependencies`
  - Files: `mini-redis/Cargo.toml`, `mini-redis/src/lib.rs`
  - Pre-commit: `cargo check`

---

- [ ] 3. Error Types & Custom Error Handling

  **What to do**:
  - Create `src/error.rs` with custom error enum
  - Implement `std::error::Error` and `std::fmt::Display`
  - Use `thiserror` for derive macros
  - Create type alias `pub type Result<T> = std::result::Result<T, Error>`
  - Write tests for error conversion

  **Error variants needed**:
  ```rust
  #[derive(Debug, thiserror::Error)]
  pub enum Error {
      #[error("connection reset by peer")]
      ConnectionReset,
      
      #[error("protocol error: {0}")]
      Protocol(String),
      
      #[error("invalid frame: {0}")]
      InvalidFrame(String),
      
      #[error("incomplete frame")]
      Incomplete,
      
      #[error("unknown command: {0}")]
      UnknownCommand(String),
      
      #[error(transparent)]
      Io(#[from] std::io::Error),
  }
  ```

  **Must NOT do**:
  - Do not use `anyhow` (we want explicit error types for learning)
  - Do not add errors that aren't needed yet

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Small, well-defined error module
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Task 1)
  - **Blocks**: Tasks 2, 5, 6, 7
  - **Blocked By**: None (can use minimal lib.rs)

  **References**:
  - Pattern: `/Users/byzantium/github/learn/rust/web-server/src/error.rs` - Your existing error handling pattern
  - External: `https://docs.rs/thiserror/latest/thiserror/` - thiserror documentation

  **Acceptance Criteria**:

  **TDD:**
  - [ ] Test file: `mini-redis/src/error.rs` (inline tests)
  - [ ] Test: `io::Error` converts to `Error::Io`
  - [ ] Test: `Error::Protocol` displays message
  - [ ] `cargo test error` → PASS

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Error types compile and convert correctly
    Tool: Bash
    Preconditions: error.rs created with all variants
    Steps:
      1. cd mini-redis && cargo test error
      2. Assert: exit code 0
      3. Assert: "test result: ok" in output
    Expected Result: All error tests pass
    Evidence: Test output captured

  Scenario: Error implements std::error::Error
    Tool: Bash
    Preconditions: Error enum defined
    Steps:
      1. Create test that uses `Box<dyn std::error::Error>`
      2. cargo test error_trait
      3. Assert: compiles and passes
    Expected Result: Error is object-safe
    Evidence: Test output captured
  ```

  **Commit**: YES
  - Message: `feat(mini-redis): add error types with thiserror`
  - Files: `mini-redis/src/error.rs`, `mini-redis/src/lib.rs`
  - Pre-commit: `cargo test error`

---

### Wave 2: Core Components

- [ ] 2. RESP Parser with Zero-Copy Lifetimes

  **What to do**:
  - Create `src/frame.rs` with `Frame<'a>` enum
  - Implement two-phase parsing: `check()` then `parse()`
  - Use `std::io::Cursor` for position tracking
  - Write comprehensive tests for each RESP type
  - Document lifetime relationships with comments

  **Frame enum (THE LEARNING GOAL)**:
  ```rust
  /// A RESP frame that borrows data from the input buffer.
  /// The lifetime `'a` ensures the Frame cannot outlive the buffer.
  #[derive(Debug, Clone, PartialEq)]
  pub enum Frame<'a> {
      /// +OK\r\n -> SimpleString("OK")
      SimpleString(&'a str),
      /// -Error message\r\n -> Error("Error message")  
      Error(&'a str),
      /// :1000\r\n -> Integer(1000)
      Integer(i64),
      /// $6\r\nfoobar\r\n -> BulkString(b"foobar")
      BulkString(&'a [u8]),
      /// Null bulk string: $-1\r\n
      Null,
      /// *2\r\n... -> Array(vec![...])
      Array(Vec<Frame<'a>>),
  }
  ```

  **Parsing methods**:
  ```rust
  impl<'a> Frame<'a> {
      /// Check if buffer contains a complete frame WITHOUT parsing.
      /// Returns Ok(()) if complete, Err(Incomplete) if more data needed.
      pub fn check(src: &mut Cursor<&[u8]>) -> Result<()>;
      
      /// Parse a frame from the buffer. Caller must ensure check() passed.
      /// The returned Frame borrows from the original buffer.
      pub fn parse(src: &'a [u8]) -> Result<(Frame<'a>, usize)>;
  }
  ```

  **Must NOT do**:
  - Do NOT use `nom`, `winnow`, or other parser combinator libraries
  - Do NOT allocate Strings - use `&'a str` slices
  - Do NOT use `unsafe` code
  - Do NOT skip writing tests first (TDD!)

  **Recommended Agent Profile**:
  - **Category**: `ultrabrain`
    - Reason: Complex lifetime annotations, careful memory reasoning required
  - **Skills**: []
    - No special skills, but executor should understand Rust lifetimes deeply

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 7)
  - **Blocks**: Tasks 4, 6
  - **Blocked By**: Tasks 1, 3

  **References**:
  - Pattern: `https://github.com/aembke/redis-protocol.rs/blob/master/src/resp2/types.rs#L106-L120` - BorrowedFrame<'a> pattern
  - External: `https://redis.io/docs/latest/develop/reference/protocol-spec/` - RESP specification
  - Learning: `/Users/byzantium/github/learn/rust/borrowing/src/main.rs` - Your borrowing examples

  **Acceptance Criteria**:

  **TDD (write these tests FIRST):**
  - [ ] Test file: `mini-redis/src/frame.rs` (inline `#[cfg(test)]` module)
  - [ ] Test: `parse_simple_string` - `+OK\r\n` → `Frame::SimpleString("OK")`
  - [ ] Test: `parse_error` - `-ERR unknown\r\n` → `Frame::Error("ERR unknown")`
  - [ ] Test: `parse_integer` - `:42\r\n` → `Frame::Integer(42)`
  - [ ] Test: `parse_integer_negative` - `:-1\r\n` → `Frame::Integer(-1)`
  - [ ] Test: `parse_bulk_string` - `$6\r\nfoobar\r\n` → `Frame::BulkString(b"foobar")`
  - [ ] Test: `parse_bulk_string_empty` - `$0\r\n\r\n` → `Frame::BulkString(b"")`
  - [ ] Test: `parse_null` - `$-1\r\n` → `Frame::Null`
  - [ ] Test: `parse_array` - `*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n` → nested Frame
  - [ ] Test: `parse_empty_array` - `*0\r\n` → `Frame::Array(vec![])`
  - [ ] Test: `check_incomplete` - partial data returns `Err(Error::Incomplete)`
  - [ ] Test: `lifetime_safety` - frame cannot outlive buffer (compile-time check)
  - [ ] `cargo test frame` → PASS (12+ tests)

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: All RESP types parse correctly
    Tool: Bash
    Preconditions: frame.rs with Frame<'a> and parse()
    Steps:
      1. cd mini-redis && cargo test frame
      2. Assert: exit code 0
      3. Assert: "12 passed" or higher in output
    Expected Result: All frame parsing tests pass
    Evidence: Test output with pass count

  Scenario: Lifetime annotation prevents use-after-free
    Tool: Bash
    Preconditions: frame.rs exists
    Steps:
      1. Create test that tries to return Frame from dropped buffer
      2. cargo test --no-run 2>&1 | grep -i "borrow\|lifetime"
      3. Assert: compilation error about lifetime
    Expected Result: Compiler prevents unsafe lifetime escape
    Evidence: Compiler error message captured

  Scenario: Zero-copy verification - no allocations in parse
    Tool: Bash
    Preconditions: frame.rs complete
    Steps:
      1. cargo test frame -- --nocapture 2>&1
      2. Search for any String::from or .to_string() in frame.rs
      3. Assert: grep finds no heap allocations in parse path
    Expected Result: Parse path uses only borrowed slices
    Evidence: grep output showing no allocations
  ```

  **Commit**: YES
  - Message: `feat(mini-redis): implement zero-copy RESP parser with Frame<'a>`
  - Files: `mini-redis/src/frame.rs`, `mini-redis/src/lib.rs`
  - Pre-commit: `cargo test frame`

---

- [ ] 5. Database Layer - Concurrent KV Store

  **What to do**:
  - Create `src/db.rs` with `Db` struct wrapping `DashMap`
  - Implement GET, SET, DEL operations
  - Store values as `Bytes` (owned, cloneable)
  - Add entry metadata for TTL (expiration instant)
  - Write tests for concurrent access

  **Structure**:
  ```rust
  use bytes::Bytes;
  use dashmap::DashMap;
  use std::time::Instant;

  #[derive(Debug)]
  pub struct Entry {
      pub data: Bytes,
      pub expires_at: Option<Instant>,
  }

  #[derive(Debug, Clone)]
  pub struct Db {
      entries: Arc<DashMap<String, Entry>>,
  }

  impl Db {
      pub fn new() -> Self;
      pub fn get(&self, key: &str) -> Option<Bytes>;
      pub fn set(&self, key: String, value: Bytes, expire: Option<Duration>);
      pub fn del(&self, key: &str) -> bool;
  }
  ```

  **Must NOT do**:
  - Do NOT implement TTL cleanup yet (Task 8)
  - Do NOT implement pub/sub storage yet (Task 7)
  - Do NOT use `std::sync::Mutex` (DashMap handles locking)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
    - Reason: Straightforward concurrent data structure usage
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 2, 7)
  - **Blocks**: Tasks 6, 8
  - **Blocked By**: Tasks 1, 3

  **References**:
  - External: `https://docs.rs/dashmap/latest/dashmap/` - DashMap API
  - Pattern: `https://github.com/tokio-rs/mini-redis/blob/master/src/db.rs` - Reference implementation

  **Acceptance Criteria**:

  **TDD:**
  - [ ] Test: `get_nonexistent` - returns None
  - [ ] Test: `set_then_get` - returns stored value
  - [ ] Test: `set_overwrites` - second set replaces first
  - [ ] Test: `del_existing` - returns true, removes entry
  - [ ] Test: `del_nonexistent` - returns false
  - [ ] Test: `concurrent_access` - multiple threads read/write safely
  - [ ] `cargo test db` → PASS

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Basic CRUD operations work
    Tool: Bash
    Preconditions: db.rs with Db struct
    Steps:
      1. cd mini-redis && cargo test db
      2. Assert: exit code 0
      3. Assert: "6 passed" or higher
    Expected Result: All database tests pass
    Evidence: Test output captured

  Scenario: Concurrent access is thread-safe
    Tool: Bash
    Preconditions: db.rs complete
    Steps:
      1. cargo test concurrent -- --test-threads=4
      2. Assert: no panics, no data races
      3. Assert: test passes
    Expected Result: DashMap handles concurrent access
    Evidence: Test output showing concurrent test pass
  ```

  **Commit**: YES
  - Message: `feat(mini-redis): add concurrent KV store with DashMap`
  - Files: `mini-redis/src/db.rs`, `mini-redis/src/lib.rs`
  - Pre-commit: `cargo test db`

---

- [ ] 7. Pub/Sub State Management

  **What to do**:
  - Extend `src/db.rs` or create `src/pubsub.rs`
  - Store channel subscriptions using `broadcast::Sender`
  - Implement subscribe/publish/unsubscribe operations
  - Handle subscriber creation and message delivery
  - Write tests for pub/sub flow

  **Structure**:
  ```rust
  use tokio::sync::broadcast;
  use bytes::Bytes;

  pub struct PubSub {
      channels: DashMap<String, broadcast::Sender<Bytes>>,
  }

  impl PubSub {
      pub fn subscribe(&self, channel: &str) -> broadcast::Receiver<Bytes>;
      pub fn publish(&self, channel: &str, message: Bytes) -> usize; // returns subscriber count
  }
  ```

  **Must NOT do**:
  - Do NOT implement pattern matching (PSUBSCRIBE)
  - Do NOT handle message history (subscribers miss old messages)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
    - Reason: Straightforward broadcast channel usage
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 2, 5)
  - **Blocks**: Task 9
  - **Blocked By**: Tasks 1, 3

  **References**:
  - Pattern: `https://github.com/tokio-rs/mini-redis/blob/master/src/db.rs#L225-L252` - subscribe/publish implementation
  - External: `https://docs.rs/tokio/latest/tokio/sync/broadcast/` - broadcast channel docs

  **Acceptance Criteria**:

  **TDD:**
  - [ ] Test: `subscribe_creates_channel` - first subscriber creates channel
  - [ ] Test: `publish_to_subscribers` - message reaches all subscribers
  - [ ] Test: `publish_no_subscribers` - returns 0 count
  - [ ] Test: `multiple_channels` - isolated message delivery
  - [ ] `cargo test pubsub` → PASS

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Pub/Sub message delivery
    Tool: Bash
    Preconditions: pubsub.rs with PubSub struct
    Steps:
      1. cd mini-redis && cargo test pubsub
      2. Assert: exit code 0
      3. Assert: all pubsub tests pass
    Expected Result: Message broadcast works correctly
    Evidence: Test output captured
  ```

  **Commit**: YES
  - Message: `feat(mini-redis): add pub/sub with broadcast channels`
  - Files: `mini-redis/src/pubsub.rs`, `mini-redis/src/lib.rs`
  - Pre-commit: `cargo test pubsub`

---

### Wave 3: Integration Layer

- [ ] 4. Connection Layer

  **What to do**:
  - Create `src/connection.rs` with `Connection` struct
  - Wrap `TcpStream` with `BufWriter` and `BytesMut` read buffer
  - Implement `read_frame()` using check-then-parse pattern
  - Implement `write_frame()` for serialization
  - Handle partial reads and connection errors

  **Structure**:
  ```rust
  use bytes::BytesMut;
  use tokio::io::{AsyncRead, AsyncWrite, BufWriter};
  use tokio::net::TcpStream;

  pub struct Connection {
      stream: BufWriter<TcpStream>,
      buffer: BytesMut,
  }

  impl Connection {
      pub fn new(stream: TcpStream) -> Self;
      
      /// Read a complete frame from the connection.
      /// Returns None on clean EOF.
      pub async fn read_frame(&mut self) -> Result<Option<Frame>>;
      
      /// Write a frame to the connection.
      pub async fn write_frame(&mut self, frame: &Frame) -> Result<()>;
  }
  ```

  **Challenge**: Converting `Frame<'a>` (borrowed) to owned `Frame` for async boundaries.
  - Option A: Create `OwnedFrame` enum with `String`/`Bytes` instead of slices
  - Option B: Parse into owned types immediately after read
  - Recommendation: Use owned `Frame` for Connection (educational note: explain why borrowing doesn't work across await)

  **Must NOT do**:
  - Do NOT handle multiple commands per read (one frame at a time)
  - Do NOT implement pipelining

  **Recommended Agent Profile**:
  - **Category**: `ultrabrain`
    - Reason: Complex async I/O with careful buffer management
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 6, 8)
  - **Blocks**: Tasks 9, 10
  - **Blocked By**: Task 2

  **References**:
  - Pattern: `https://github.com/tokio-rs/mini-redis/blob/master/src/connection.rs` - Connection implementation
  - Research Finding: Two-phase parse with Cursor, BytesMut buffering

  **Acceptance Criteria**:

  **TDD:**
  - [ ] Test: `read_complete_frame` - mock stream returns valid frame
  - [ ] Test: `read_partial_then_complete` - handles chunked data
  - [ ] Test: `write_frame_serializes` - frame written correctly
  - [ ] Test: `eof_returns_none` - clean disconnect handling
  - [ ] `cargo test connection` → PASS

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Frame round-trip through connection
    Tool: Bash
    Preconditions: connection.rs complete
    Steps:
      1. cd mini-redis && cargo test connection
      2. Assert: exit code 0
    Expected Result: Read/write tests pass
    Evidence: Test output captured
  ```

  **Commit**: YES
  - Message: `feat(mini-redis): add frame-based connection layer`
  - Files: `mini-redis/src/connection.rs`, `mini-redis/src/lib.rs`
  - Pre-commit: `cargo test connection`

---

- [ ] 6. Command Engine

  **What to do**:
  - Create `src/cmd/mod.rs` with command parsing
  - Implement `Get`, `Set`, `Del`, `Ping`, `Publish`, `Subscribe` commands
  - Parse from Frame arrays to command structs
  - Execute commands against Db and return response Frames
  - Write tests for each command

  **Structure**:
  ```rust
  // src/cmd/mod.rs
  pub mod get;
  pub mod set;
  pub mod ping;
  pub mod publish;
  pub mod subscribe;

  pub enum Command {
      Get(Get),
      Set(Set),
      Del(Del),
      Ping,
      Publish(Publish),
      Subscribe(Subscribe),
  }

  impl Command {
      pub fn from_frame(frame: Frame) -> Result<Command>;
      pub async fn execute(self, db: &Db, conn: &mut Connection) -> Result<()>;
  }
  ```

  **Must NOT do**:
  - Do NOT implement full Redis command set
  - Do NOT add command flags/options beyond basic ones
  - Do NOT implement AUTH/SELECT

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Multiple command implementations, pattern matching, integration
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 4, 8)
  - **Blocks**: Task 9
  - **Blocked By**: Tasks 2, 5

  **References**:
  - Pattern: `https://github.com/tokio-rs/mini-redis/tree/master/src/cmd` - Command structure
  - External: `https://redis.io/docs/latest/commands/` - Command specifications

  **Acceptance Criteria**:

  **TDD:**
  - [ ] Test: `parse_ping` - `*1\r\n$4\r\nPING\r\n` → `Command::Ping`
  - [ ] Test: `parse_get` - extracts key correctly
  - [ ] Test: `parse_set` - extracts key, value, optional expiry
  - [ ] Test: `execute_ping` - returns `PONG`
  - [ ] Test: `execute_get_missing` - returns null
  - [ ] Test: `execute_set_then_get` - round-trip works
  - [ ] `cargo test cmd` → PASS

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: All commands parse and execute
    Tool: Bash
    Preconditions: cmd/ module complete
    Steps:
      1. cd mini-redis && cargo test cmd
      2. Assert: exit code 0
    Expected Result: Command tests pass
    Evidence: Test output captured
  ```

  **Commit**: YES
  - Message: `feat(mini-redis): implement command parsing and execution`
  - Files: `mini-redis/src/cmd/*.rs`, `mini-redis/src/lib.rs`
  - Pre-commit: `cargo test cmd`

---

- [ ] 8. TTL & Expiration System

  **What to do**:
  - Add expiration tracking to Db using `BTreeSet<(Instant, String)>`
  - Implement background purge task with `tokio::time::sleep_until`
  - Use `Notify` to wake purge task when new TTL added
  - Integrate with SET command's EX/PX options
  - Write tests for expiration behavior

  **Structure**:
  ```rust
  // Addition to db.rs
  struct Shared {
      entries: DashMap<String, Entry>,
      expirations: Mutex<BTreeSet<(Instant, String)>>,
      background_task: Notify,
      shutdown: AtomicBool,
  }

  impl Db {
      pub fn set_with_expiry(&self, key: String, value: Bytes, expiry: Duration);
      
      // Called by background task
      fn purge_expired_keys(&self) -> Option<Instant>;
  }

  // Background task
  async fn purge_expired_task(shared: Arc<Shared>) {
      while !shared.shutdown.load(Ordering::Relaxed) {
          if let Some(when) = shared.purge_expired_keys() {
              tokio::select! {
                  _ = tokio::time::sleep_until(when.into()) => {}
                  _ = shared.background_task.notified() => {}
              }
          } else {
              shared.background_task.notified().await;
          }
      }
  }
  ```

  **Must NOT do**:
  - Do NOT implement lazy expiration only (we need active purge)
  - Do NOT use per-key timers (too expensive)

  **Recommended Agent Profile**:
  - **Category**: `ultrabrain`
    - Reason: Complex async coordination, careful timing logic
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 4, 6)
  - **Blocks**: Task 9
  - **Blocked By**: Task 5

  **References**:
  - Pattern: `https://github.com/tokio-rs/mini-redis/blob/master/src/db.rs#L346-L369` - Purge task
  - Research Finding: BTreeSet<(Instant, String)> + Notify pattern

  **Acceptance Criteria**:

  **TDD:**
  - [ ] Test: `set_with_expiry_expires` - key gone after TTL
  - [ ] Test: `set_updates_expiry` - new TTL replaces old
  - [ ] Test: `purge_removes_expired` - background task cleans up
  - [ ] `cargo test expiry` → PASS

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Keys expire after TTL
    Tool: Bash
    Preconditions: TTL implementation complete
    Steps:
      1. cd mini-redis && cargo test expiry -- --test-threads=1
      2. Assert: exit code 0
    Expected Result: Expiration tests pass
    Evidence: Test output captured
  ```

  **Commit**: YES
  - Message: `feat(mini-redis): add TTL expiration with background purge`
  - Files: `mini-redis/src/db.rs`
  - Pre-commit: `cargo test expiry`

---

### Wave 4: Server & Client

- [ ] 9. Server Binary

  **What to do**:
  - Create `src/server.rs` with main server logic
  - Implement `TcpListener` accept loop
  - Spawn task per connection with shared Db
  - Implement graceful shutdown on SIGINT
  - Add tracing instrumentation
  - Create `src/bin/server.rs` binary entry point

  **Structure**:
  ```rust
  // src/server.rs
  pub struct Server {
      listener: TcpListener,
      db: Db,
      shutdown: broadcast::Sender<()>,
  }

  impl Server {
      pub async fn run(self) -> Result<()>;
  }

  // src/bin/server.rs
  #[tokio::main]
  async fn main() -> mini_redis::Result<()> {
      // Init tracing
      // Bind listener
      // Create server
      // Run with graceful shutdown
  }
  ```

  **Must NOT do**:
  - Do NOT add CLI argument parsing (hardcode port 6379)
  - Do NOT implement TLS
  - Do NOT implement connection limits

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Integration of all components, async coordination
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Task 10)
  - **Blocks**: Task 11
  - **Blocked By**: Tasks 4, 6, 7, 8

  **References**:
  - Pattern: `https://github.com/tokio-rs/mini-redis/blob/master/src/server.rs` - Server implementation
  - Pattern: `/Users/byzantium/github/learn/rust/web-server/src/main.rs` - Your Tokio server pattern
  - Research Finding: broadcast + mpsc for graceful shutdown

  **Acceptance Criteria**:

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Server starts and accepts connections
    Tool: interactive_bash (tmux)
    Preconditions: server binary built
    Steps:
      1. tmux new-session -d -s redis-server "cd mini-redis && cargo run --bin server"
      2. Wait 3 seconds for startup
      3. Assert: "Listening on" in tmux output
      4. redis-cli -p 6379 PING
      5. Assert: stdout is "PONG"
      6. tmux send-keys -t redis-server C-c
    Expected Result: Server responds to PING
    Evidence: .sisyphus/evidence/task-9-server-ping.txt

  Scenario: GET and SET work via redis-cli
    Tool: Bash + interactive_bash
    Preconditions: server running on 6379
    Steps:
      1. Start server in tmux
      2. redis-cli SET mykey "hello world"
      3. Assert: stdout is "OK"
      4. redis-cli GET mykey
      5. Assert: stdout is "hello world"
      6. Stop server
    Expected Result: Basic KV operations work
    Evidence: Command outputs captured

  Scenario: SETEX expires keys
    Tool: Bash + interactive_bash
    Preconditions: server running
    Steps:
      1. redis-cli SETEX tempkey 1 "temporary"
      2. redis-cli GET tempkey → "temporary"
      3. sleep 2
      4. redis-cli GET tempkey → (nil)
    Expected Result: Key expires after TTL
    Evidence: Command outputs captured

  Scenario: Graceful shutdown on SIGINT
    Tool: interactive_bash (tmux)
    Preconditions: server running
    Steps:
      1. Send SIGINT to server (Ctrl+C)
      2. Assert: "Shutting down" in logs
      3. Assert: process exits cleanly (code 0)
    Expected Result: Clean shutdown without panic
    Evidence: Exit code captured
  ```

  **Commit**: YES
  - Message: `feat(mini-redis): implement server with graceful shutdown`
  - Files: `mini-redis/src/server.rs`, `mini-redis/src/bin/server.rs`, `mini-redis/src/lib.rs`
  - Pre-commit: `cargo build --bin server`

---

- [ ] 10. Client Library

  **What to do**:
  - Create `src/client.rs` with `Client` struct
  - Implement connection to server
  - Provide typed methods: `get()`, `set()`, `publish()`, `subscribe()`
  - Handle response parsing
  - Write unit tests with mock server

  **Structure**:
  ```rust
  pub struct Client {
      connection: Connection,
  }

  impl Client {
      pub async fn connect(addr: impl ToSocketAddrs) -> Result<Client>;
      pub async fn ping(&mut self) -> Result<()>;
      pub async fn get(&mut self, key: &str) -> Result<Option<Bytes>>;
      pub async fn set(&mut self, key: &str, value: Bytes) -> Result<()>;
      pub async fn set_ex(&mut self, key: &str, value: Bytes, seconds: u64) -> Result<()>;
      pub async fn publish(&mut self, channel: &str, message: Bytes) -> Result<u64>;
  }

  pub struct Subscriber {
      // ...
  }
  ```

  **Must NOT do**:
  - Do NOT implement connection pooling
  - Do NOT implement automatic reconnection
  - Do NOT implement pipelining

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
    - Reason: Straightforward client wrapper around Connection
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Task 9)
  - **Blocks**: Task 11
  - **Blocked By**: Task 4

  **References**:
  - Pattern: `https://github.com/tokio-rs/mini-redis/blob/master/src/client.rs` - Client implementation

  **Acceptance Criteria**:

  **TDD:**
  - [ ] Test: `client_ping` - ping returns OK
  - [ ] Test: `client_get_set` - round-trip works
  - [ ] `cargo test client` → PASS

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Client connects and operates
    Tool: Bash
    Preconditions: client.rs complete
    Steps:
      1. cargo test client
      2. Assert: tests pass
    Expected Result: Client unit tests pass
    Evidence: Test output captured
  ```

  **Commit**: YES
  - Message: `feat(mini-redis): add client library`
  - Files: `mini-redis/src/client.rs`, `mini-redis/src/lib.rs`
  - Pre-commit: `cargo test client`

---

### Wave 5: Integration & Polish

- [ ] 11. Integration Tests & Polish

  **What to do**:
  - Create `tests/integration.rs` with full E2E tests
  - Test client-server interaction
  - Test pub/sub with multiple clients
  - Test concurrent operations
  - Run `cargo clippy` and fix warnings
  - Add module-level documentation
  - Create simple CLI binary for manual testing

  **Integration tests**:
  ```rust
  #[tokio::test]
  async fn test_ping_pong() {
      let server = spawn_server().await;
      let mut client = Client::connect(server.addr()).await.unwrap();
      client.ping().await.unwrap();
  }

  #[tokio::test]
  async fn test_pubsub_broadcast() {
      // Start server
      // Create publisher client
      // Create 2 subscriber clients
      // Publish message
      // Assert both subscribers receive it
  }
  ```

  **Must NOT do**:
  - Do NOT add features not in scope
  - Do NOT over-document (brief, useful comments only)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Integration testing, multiple components, final polish
  - **Skills**: [`git-master`]
    - `git-master`: For final commit with all tests passing

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 5 (final, sequential)
  - **Blocks**: None (final task)
  - **Blocked By**: Tasks 9, 10

  **References**:
  - Pattern: `/Users/byzantium/github/learn/rust/web-server/tests/api_tests.rs` - Your integration test pattern
  - Pattern: `https://github.com/tokio-rs/mini-redis/tree/master/tests` - Reference integration tests

  **Acceptance Criteria**:

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Full test suite passes
    Tool: Bash
    Preconditions: All prior tasks complete
    Steps:
      1. cd mini-redis && cargo test
      2. Assert: exit code 0
      3. Assert: no test failures
    Expected Result: All unit and integration tests pass
    Evidence: .sisyphus/evidence/task-11-test-output.txt

  Scenario: Clippy finds no warnings
    Tool: Bash
    Preconditions: All code complete
    Steps:
      1. cd mini-redis && cargo clippy -- -D warnings
      2. Assert: exit code 0
    Expected Result: No clippy warnings
    Evidence: Command output captured

  Scenario: Pub/Sub works with redis-cli
    Tool: Bash + interactive_bash
    Preconditions: server running
    Steps:
      1. Start server in tmux
      2. In tmux pane 2: redis-cli SUBSCRIBE mychannel
      3. In shell: redis-cli PUBLISH mychannel "hello"
      4. Assert: pane 2 shows "hello" message
      5. Stop server
    Expected Result: Pub/sub broadcast works
    Evidence: tmux output captured

  Scenario: Concurrent clients work correctly
    Tool: Bash
    Preconditions: server running
    Steps:
      1. Start server
      2. Run integration test with 10 concurrent clients
      3. Assert: all operations succeed
      4. Assert: no panics or data races
    Expected Result: Server handles concurrency
    Evidence: Test output captured
  ```

  **Commit**: YES
  - Message: `feat(mini-redis): add integration tests and polish`
  - Files: `mini-redis/tests/integration.rs`, various doc comments
  - Pre-commit: `cargo test && cargo clippy -- -D warnings`

---

## Commit Strategy

| After Task | Message | Files | Verification |
|------------|---------|-------|--------------|
| 1 | `feat(mini-redis): initialize project` | Cargo.toml, lib.rs | cargo check |
| 3 | `feat(mini-redis): add error types` | error.rs | cargo test error |
| 2 | `feat(mini-redis): implement RESP parser` | frame.rs | cargo test frame |
| 5 | `feat(mini-redis): add concurrent KV store` | db.rs | cargo test db |
| 7 | `feat(mini-redis): add pub/sub` | pubsub.rs | cargo test pubsub |
| 4 | `feat(mini-redis): add connection layer` | connection.rs | cargo test connection |
| 6 | `feat(mini-redis): implement commands` | cmd/*.rs | cargo test cmd |
| 8 | `feat(mini-redis): add TTL expiration` | db.rs | cargo test expiry |
| 9 | `feat(mini-redis): implement server` | server.rs, bin/server.rs | redis-cli PING |
| 10 | `feat(mini-redis): add client library` | client.rs | cargo test client |
| 11 | `feat(mini-redis): integration tests` | tests/*.rs | cargo test |

---

## Success Criteria

### Verification Commands
```bash
# All tests pass
cd mini-redis && cargo test
# Expected: test result: ok. X passed; 0 failed

# No warnings
cargo clippy -- -D warnings
# Expected: exit 0

# Server accepts redis-cli
cargo run --bin server &
redis-cli PING
# Expected: PONG

redis-cli SET foo bar && redis-cli GET foo
# Expected: OK, then bar

redis-cli SETEX temp 1 value && sleep 2 && redis-cli GET temp
# Expected: OK, then (nil)
```

### Final Checklist
- [ ] All "Must Have" present:
  - [ ] Zero-copy parser with Frame<'a>
  - [ ] GET, SET, SETEX, DEL, PING commands
  - [ ] PUBLISH, SUBSCRIBE commands
  - [ ] TTL expiration
  - [ ] Graceful shutdown
  - [ ] Tracing instrumentation
- [ ] All "Must NOT Have" absent:
  - [ ] No persistence
  - [ ] No clustering
  - [ ] No Lua scripting
  - [ ] No unsafe code
  - [ ] No parser libraries (nom, winnow)
- [ ] All tests pass: `cargo test`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] redis-cli compatible

---

## Learning Outcomes

After completing this project, you will have mastered:

1. **Lifetimes**: Understanding `'a` annotations, why Frame<'a> can't cross await boundaries, and when to use owned vs borrowed types
2. **Zero-Copy Parsing**: Using `&[u8]` slices instead of allocating, Cursor for position tracking
3. **Async I/O**: TcpListener/TcpStream patterns, BytesMut buffering, frame-based protocols
4. **Concurrency**: DashMap for lock-free concurrent access, broadcast channels for pub/sub
5. **Tokio Patterns**: Graceful shutdown, background tasks, select! macro, Notify for coordination
6. **Production Rust**: Error handling with thiserror, tracing for observability, testing best practices
