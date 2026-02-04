# Rust REST API Web Server with Axum + SQLite

## TL;DR

> **Quick Summary**: Build a production-quality Todo CRUD REST API using Axum 0.8.x and SQLite with SQLx, establishing multi-module patterns for the learning repository.
> 
> **Deliverables**:
> - Complete Rust web server at `/web-server/`
> - 6 REST endpoints (health + full CRUD)
> - SQLite database with WAL mode and embedded migrations
> - Centralized error handling with JSON responses
> - Request logging with tracing
> - Integration tests for all endpoints
> 
> **Estimated Effort**: Medium (8-10 tasks)
> **Parallel Execution**: YES - 3 waves
> **Critical Path**: Task 1 → Task 2 → Task 3 → Tasks 4-6 (parallel) → Task 7 → Task 8 → Task 9

---

## Context

### Original Request
Create a complete REST API Web Server in Rust with Axum 0.8.x, SQLite/SQLx, JSON serialization, error handling, and logging. This is a learning project to be created at `/Users/byzantium/github/learn/rust/web-server/`.

### Interview Summary
**Key Discussions**:
- Framework: Axum 0.8.x (confirmed with specific patterns)
- Database: SQLite with SQLx 0.8, WAL mode, embedded migrations
- Test Strategy: Tests-after (implementation first, integration tests at end)

**Research Findings**:
- Existing projects use Rust Edition 2024
- First multi-module project in this learning repo
- No existing patterns for error handling crates (will use thiserror)
- Projects are standalone directories (no workspace)

### Gap Analysis (Self-Review)
**Identified Gaps** (addressed):
- Migration file naming: Using `YYYYMMDD_HHMMSS_name.sql` format per SQLx convention
- Port number: Default to 3000 as specified in requirements
- Database file location: `./data/todos.db` (inside project, gitignored)
- Test dependencies: Added `reqwest` and `tokio-test` for integration tests

---

## Work Objectives

### Core Objective
Build a fully functional Todo CRUD REST API that demonstrates Axum 0.8.x patterns, SQLx database integration, and production-quality error handling.

### Concrete Deliverables
- `/web-server/Cargo.toml` - Project configuration with all dependencies
- `/web-server/src/main.rs` - Entry point with server setup
- `/web-server/src/db.rs` - Database connection and pool setup
- `/web-server/src/error.rs` - Centralized AppError type
- `/web-server/src/routes/mod.rs` - Route module organization
- `/web-server/src/routes/todos.rs` - Todo CRUD handlers
- `/web-server/src/models/mod.rs` - Model module organization
- `/web-server/src/models/todo.rs` - Todo struct and DTOs
- `/web-server/migrations/*.sql` - Database schema
- `/web-server/tests/api_tests.rs` - Integration tests

### Definition of Done
- [ ] `cargo build` succeeds with no errors
- [ ] `cargo run` starts server on localhost:3000
- [ ] All 6 endpoints respond correctly (verified via curl)
- [ ] Database persists data across server restarts
- [ ] `cargo test` passes all integration tests

### Must Have
- JSON request/response with proper Content-Type headers
- SQLite with WAL mode for concurrency
- Compile-time checked SQL with query_as! macros
- Consistent JSON error responses with appropriate HTTP status codes
- Request logging via tracing/tower-http

### Must NOT Have (Guardrails)
- NO authentication/authorization (out of scope for learning project)
- NO Docker or containerization
- NO CI/CD configuration
- NO rate limiting or advanced middleware
- NO pagination (keep CRUD simple for learning)
- NO over-engineering - this is a learning project, keep it straightforward

---

## Verification Strategy

> **UNIVERSAL RULE: ZERO HUMAN INTERVENTION**
> ALL verification is executed by the agent using curl commands.

### Test Decision
- **Infrastructure exists**: NO (first project with tests)
- **Automated tests**: YES (tests-after)
- **Framework**: Rust built-in `#[tokio::test]` with `reqwest` for HTTP assertions
- **Agent-Executed QA**: curl verification during development, integration tests at end

### Agent-Executed QA Approach

Every endpoint task includes curl-based verification scenarios. The agent will:
1. Start the server with `cargo run &`
2. Execute curl commands against endpoints
3. Verify response status codes and JSON bodies
4. Capture evidence in terminal output
5. Stop the server after verification

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Foundation - Sequential):
└── Task 1: Project scaffolding + Cargo.toml

Wave 2 (Core Infrastructure - Sequential):
├── Task 2: Database setup (db.rs + migrations)
└── Task 3: Error handling (error.rs) [after Task 2]

Wave 3 (Models + Routes - Parallel after Wave 2):
├── Task 4: Todo model (models/)
├── Task 5: Health endpoint
└── Task 6: Todo CRUD routes (routes/)

Wave 4 (Integration - Sequential):
└── Task 7: Main.rs - wire everything together

Wave 5 (Verification - Sequential):
├── Task 8: Full API verification with curl
└── Task 9: Integration tests

Critical Path: 1 → 2 → 3 → 6 → 7 → 8 → 9
Parallel Speedup: Tasks 4, 5, 6 can run in parallel after Task 3
```

### Dependency Matrix

| Task | Depends On | Blocks | Can Parallelize With |
|------|------------|--------|---------------------|
| 1 | None | 2, 3, 4, 5, 6 | None |
| 2 | 1 | 3, 6, 7 | None |
| 3 | 2 | 4, 5, 6 | None |
| 4 | 3 | 6, 7 | 5 |
| 5 | 3 | 7 | 4, 6 |
| 6 | 3, 4 | 7 | 5 |
| 7 | 4, 5, 6 | 8 | None |
| 8 | 7 | 9 | None |
| 9 | 8 | None | None |

### Agent Dispatch Summary

| Wave | Tasks | Recommended Category |
|------|-------|---------------------|
| 1 | 1 | quick |
| 2 | 2, 3 | quick (sequential) |
| 3 | 4, 5, 6 | quick (can parallelize) |
| 4 | 7 | quick |
| 5 | 8, 9 | quick |

---

## TODOs

- [ ] 1. Project Scaffolding and Cargo.toml

  **What to do**:
  - Create `/web-server/` directory
  - Initialize with `cargo init`
  - Configure `Cargo.toml` with all dependencies:
    ```toml
    [package]
    name = "web-server"
    version = "0.1.0"
    edition = "2024"

    [dependencies]
    axum = "0.8"
    tokio = { version = "1.0", features = ["full"] }
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
    sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
    tower-http = { version = "0.6", features = ["trace", "cors"] }
    tracing = "0.1"
    tracing-subscriber = { version = "0.3", features = ["env-filter"] }
    thiserror = "1.0"

    [dev-dependencies]
    reqwest = { version = "0.12", features = ["json"] }
    tokio-test = "0.4"
    ```
  - Create directory structure:
    - `src/routes/`
    - `src/models/`
    - `migrations/`
    - `data/` (for SQLite database)
  - Add `data/` to `.gitignore`

  **Must NOT do**:
  - Do not create a workspace
  - Do not add unnecessary dependencies

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple file creation and cargo init
  - **Skills**: None needed
    - File operations are straightforward

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 1 (solo)
  - **Blocks**: Tasks 2, 3, 4, 5, 6
  - **Blocked By**: None

  **References**:
  - User-provided Cargo.toml dependencies in technical context
  - Explore agent finding: Use `edition = "2024"` for consistency

  **Acceptance Criteria**:
  - [ ] `/web-server/Cargo.toml` exists with all dependencies
  - [ ] Directory structure created: `src/routes/`, `src/models/`, `migrations/`, `data/`
  - [ ] `.gitignore` includes `data/` directory
  - [ ] `cargo check` runs without dependency resolution errors

  **Agent-Executed QA Scenarios**:

  ```
  Scenario: Project compiles with all dependencies
    Tool: Bash
    Preconditions: None
    Steps:
      1. cd /Users/byzantium/github/learn/rust/web-server
      2. cargo check 2>&1
      3. Assert: exit code is 0
      4. Assert: output contains "Compiling" or "Finished"
    Expected Result: All dependencies resolve and compile
    Evidence: Terminal output captured
  ```

  **Commit**: YES
  - Message: `feat(web-server): initialize project with dependencies`
  - Files: `web-server/Cargo.toml`, `web-server/src/main.rs`, `web-server/.gitignore`

---

- [ ] 2. Database Setup (db.rs + migrations)

  **What to do**:
  - Create `src/db.rs` with:
    - `init_db()` function returning `SqlitePool`
    - SQLite connection with WAL mode enabled
    - `create_if_missing(true)` for auto-creation
    - `foreign_keys(true)` for referential integrity
    - `max_connections(5)` for SQLite
    - Embedded migrations with `sqlx::migrate!()`
  - Create migration file `migrations/20240101_000001_create_todos.sql`:
    ```sql
    CREATE TABLE IF NOT EXISTS todos (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        description TEXT,
        completed INTEGER NOT NULL DEFAULT 0,
        created_at TEXT NOT NULL DEFAULT (datetime('now')),
        updated_at TEXT NOT NULL DEFAULT (datetime('now'))
    );
    ```

  **Must NOT do**:
  - Do not use runtime migrations (use embedded)
  - Do not create multiple database files
  - Do not use complex connection pooling

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Standard SQLx setup pattern
  - **Skills**: None needed
    - SQLx patterns are well-documented in technical context

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2 (sequential with Task 3)
  - **Blocks**: Task 3, 6, 7
  - **Blocked By**: Task 1

  **References**:
  - User-provided SQLx patterns: WAL mode, create_if_missing, foreign_keys
  - SQLx 0.8 documentation for SqliteConnectOptions

  **Acceptance Criteria**:
  - [ ] `src/db.rs` exists with `init_db()` function
  - [ ] `migrations/20240101_000001_create_todos.sql` exists
  - [ ] Migration creates todos table with all columns
  - [ ] `cargo check` passes with db.rs included

  **Agent-Executed QA Scenarios**:

  ```
  Scenario: Database module compiles
    Tool: Bash
    Preconditions: Task 1 complete
    Steps:
      1. cd /Users/byzantium/github/learn/rust/web-server
      2. Add "mod db;" to main.rs temporarily
      3. cargo check 2>&1
      4. Assert: exit code is 0
    Expected Result: db.rs compiles without errors
    Evidence: Terminal output captured
  ```

  **Commit**: YES
  - Message: `feat(web-server): add database setup with migrations`
  - Files: `web-server/src/db.rs`, `web-server/migrations/*.sql`

---

- [ ] 3. Error Handling (error.rs)

  **What to do**:
  - Create `src/error.rs` with:
    - `AppError` enum using `thiserror` derive:
      - `NotFound(String)` - 404 errors
      - `BadRequest(String)` - 400 errors
      - `InternalError(String)` - 500 errors
      - `DatabaseError(sqlx::Error)` - database errors
    - Implement `IntoResponse` for AppError:
      - Return JSON body: `{"error": "message"}`
      - Set appropriate HTTP status codes
    - Implement `From<sqlx::Error>` for AppError

  **Must NOT do**:
  - Do not create overly complex error hierarchies
  - Do not add authentication-related errors (out of scope)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Standard error handling pattern
  - **Skills**: None needed
    - thiserror + IntoResponse is straightforward

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2 (after Task 2)
  - **Blocks**: Tasks 4, 5, 6
  - **Blocked By**: Task 2 (needs sqlx::Error type)

  **References**:
  - User-provided: Custom AppError enum implementing IntoResponse
  - Axum 0.8 documentation for IntoResponse trait

  **Acceptance Criteria**:
  - [ ] `src/error.rs` exists with AppError enum
  - [ ] AppError implements IntoResponse
  - [ ] AppError implements From<sqlx::Error>
  - [ ] JSON error format: `{"error": "message"}`
  - [ ] `cargo check` passes

  **Agent-Executed QA Scenarios**:

  ```
  Scenario: Error module compiles with correct trait implementations
    Tool: Bash
    Preconditions: Task 2 complete
    Steps:
      1. cd /Users/byzantium/github/learn/rust/web-server
      2. Add "mod error;" to main.rs
      3. cargo check 2>&1
      4. Assert: exit code is 0
      5. Assert: no "trait bound" errors in output
    Expected Result: Error types compile with all trait implementations
    Evidence: Terminal output captured
  ```

  **Commit**: YES
  - Message: `feat(web-server): add centralized error handling`
  - Files: `web-server/src/error.rs`

---

- [ ] 4. Todo Model (models/)

  **What to do**:
  - Create `src/models/mod.rs` with `pub mod todo;`
  - Create `src/models/todo.rs` with:
    - `Todo` struct (database row):
      ```rust
      #[derive(Debug, Serialize, sqlx::FromRow)]
      pub struct Todo {
          pub id: i64,
          pub title: String,
          pub description: Option<String>,
          pub completed: bool,
          pub created_at: String,
          pub updated_at: String,
      }
      ```
    - `CreateTodo` DTO (request body):
      ```rust
      #[derive(Debug, Deserialize)]
      pub struct CreateTodo {
          pub title: String,
          pub description: Option<String>,
      }
      ```
    - `UpdateTodo` DTO (request body):
      ```rust
      #[derive(Debug, Deserialize)]
      pub struct UpdateTodo {
          pub title: Option<String>,
          pub description: Option<String>,
          pub completed: Option<bool>,
      }
      ```

  **Must NOT do**:
  - Do not add validation logic (keep models simple)
  - Do not add pagination structs

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple struct definitions
  - **Skills**: None needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 5, 6)
  - **Blocks**: Task 6, 7
  - **Blocked By**: Task 3

  **References**:
  - Migration schema from Task 2 for field types
  - Serde derive patterns for JSON serialization

  **Acceptance Criteria**:
  - [ ] `src/models/mod.rs` exists
  - [ ] `src/models/todo.rs` exists with Todo, CreateTodo, UpdateTodo
  - [ ] Todo derives FromRow for SQLx
  - [ ] DTOs derive Serialize/Deserialize as appropriate
  - [ ] `cargo check` passes

  **Agent-Executed QA Scenarios**:

  ```
  Scenario: Model structs compile with correct derives
    Tool: Bash
    Preconditions: Task 3 complete
    Steps:
      1. cd /Users/byzantium/github/learn/rust/web-server
      2. Add "mod models;" to main.rs
      3. cargo check 2>&1
      4. Assert: exit code is 0
    Expected Result: Models compile with Serialize, Deserialize, FromRow
    Evidence: Terminal output captured
  ```

  **Commit**: NO (groups with Task 6)

---

- [ ] 5. Health Endpoint

  **What to do**:
  - Create simple health check in `src/routes/health.rs`:
    ```rust
    use axum::{Json, http::StatusCode};
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct HealthResponse {
        pub status: String,
    }

    pub async fn health_check() -> Json<HealthResponse> {
        Json(HealthResponse {
            status: "ok".to_string(),
        })
    }
    ```
  - This is a simple, independent endpoint for testing server liveness

  **Must NOT do**:
  - Do not add database health checks (keep simple)
  - Do not add version info

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single simple handler
  - **Skills**: None needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 4, 6)
  - **Blocks**: Task 7
  - **Blocked By**: Task 3

  **References**:
  - Axum 0.8 Json response pattern

  **Acceptance Criteria**:
  - [ ] `src/routes/health.rs` exists
  - [ ] Returns JSON `{"status": "ok"}`
  - [ ] `cargo check` passes

  **Agent-Executed QA Scenarios**:

  ```
  Scenario: Health module compiles
    Tool: Bash
    Preconditions: Task 3 complete
    Steps:
      1. cd /Users/byzantium/github/learn/rust/web-server
      2. cargo check 2>&1
      3. Assert: exit code is 0
    Expected Result: Health endpoint compiles
    Evidence: Terminal output captured
  ```

  **Commit**: NO (groups with Task 6)

---

- [ ] 6. Todo CRUD Routes (routes/)

  **What to do**:
  - Create `src/routes/mod.rs`:
    ```rust
    pub mod health;
    pub mod todos;
    ```
  - Create `src/routes/todos.rs` with handlers:
    - `list_todos` - GET /todos → returns Vec<Todo>
    - `get_todo` - GET /todos/:id → returns Todo or 404
    - `create_todo` - POST /todos → creates and returns Todo with 201
    - `update_todo` - PUT /todos/:id → updates and returns Todo or 404
    - `delete_todo` - DELETE /todos/:id → returns 204 or 404
  - Create `todo_routes()` function returning Router:
    ```rust
    pub fn todo_routes() -> Router<AppState> {
        Router::new()
            .route("/", get(list_todos).post(create_todo))
            .route("/{id}", get(get_todo).put(update_todo).delete(delete_todo))
    }
    ```
  - Use `State<AppState>` extractor for database pool
  - Use `Path<i64>` extractor for :id parameter
  - Use `Json<CreateTodo>` and `Json<UpdateTodo>` for request bodies
  - Use SQLx `query_as!` macro for compile-time checked queries

  **Must NOT do**:
  - Do not add pagination
  - Do not add filtering/sorting
  - Do not add bulk operations

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Standard CRUD patterns
  - **Skills**: None needed
    - Axum patterns provided in technical context

  **Parallelization**:
  - **Can Run In Parallel**: YES (partially)
  - **Parallel Group**: Wave 3 (with Tasks 4, 5, but needs Task 4 models)
  - **Blocks**: Task 7
  - **Blocked By**: Tasks 3, 4

  **References**:
  - User-provided Axum patterns: Router::nest, State extractor, Json extractor
  - Todo model from Task 4
  - AppError from Task 3

  **Acceptance Criteria**:
  - [ ] `src/routes/mod.rs` exists
  - [ ] `src/routes/todos.rs` exists with 5 handlers
  - [ ] `todo_routes()` returns configured Router
  - [ ] All handlers use compile-time checked SQL
  - [ ] Proper error handling with AppError
  - [ ] `cargo check` passes

  **Agent-Executed QA Scenarios**:

  ```
  Scenario: Routes module compiles with all handlers
    Tool: Bash
    Preconditions: Tasks 3, 4 complete
    Steps:
      1. cd /Users/byzantium/github/learn/rust/web-server
      2. Add "mod routes;" to main.rs
      3. cargo check 2>&1
      4. Assert: exit code is 0
    Expected Result: All route handlers compile
    Evidence: Terminal output captured
  ```

  **Commit**: YES
  - Message: `feat(web-server): add Todo CRUD routes and models`
  - Files: `web-server/src/routes/*.rs`, `web-server/src/models/*.rs`

---

- [ ] 7. Main Entry Point (main.rs)

  **What to do**:
  - Wire everything together in `src/main.rs`:
    ```rust
    mod db;
    mod error;
    mod models;
    mod routes;

    use axum::Router;
    use sqlx::SqlitePool;
    use std::sync::Arc;
    use tower_http::trace::TraceLayer;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    #[derive(Clone)]
    pub struct AppState {
        pub db: SqlitePool,
    }

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Initialize tracing
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
            ))
            .with(tracing_subscriber::fmt::layer())
            .init();

        // Initialize database
        let pool = db::init_db().await?;
        let state = AppState { db: pool };

        // Build router
        let app = Router::new()
            .route("/health", axum::routing::get(routes::health::health_check))
            .nest("/todos", routes::todos::todo_routes())
            .with_state(state)
            .layer(TraceLayer::new_for_http());

        // Start server
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
        tracing::info!("Server running on http://localhost:3000");
        axum::serve(listener, app).await?;

        Ok(())
    }
    ```

  **Must NOT do**:
  - Do not add graceful shutdown (keep simple)
  - Do not add configuration files (use env vars)
  - Do not add CORS middleware (out of scope)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Wiring existing modules together
  - **Skills**: None needed

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4 (solo)
  - **Blocks**: Task 8
  - **Blocked By**: Tasks 4, 5, 6

  **References**:
  - All previous task outputs (db.rs, error.rs, routes/, models/)
  - User-provided: TraceLayer for HTTP logging

  **Acceptance Criteria**:
  - [ ] `src/main.rs` imports all modules
  - [ ] AppState struct with database pool
  - [ ] Router configured with all routes
  - [ ] TraceLayer added for logging
  - [ ] Server binds to 0.0.0.0:3000
  - [ ] `cargo build` succeeds
  - [ ] `cargo run` starts server

  **Agent-Executed QA Scenarios**:

  ```
  Scenario: Server compiles and builds successfully
    Tool: Bash
    Preconditions: Tasks 4, 5, 6 complete
    Steps:
      1. cd /Users/byzantium/github/learn/rust/web-server
      2. cargo build 2>&1
      3. Assert: exit code is 0
      4. Assert: output contains "Finished"
    Expected Result: Full project builds without errors
    Evidence: Terminal output captured

  Scenario: Server starts and listens on port 3000
    Tool: Bash
    Preconditions: Build succeeds
    Steps:
      1. cd /Users/byzantium/github/learn/rust/web-server
      2. timeout 5 cargo run 2>&1 || true
      3. Assert: output contains "Server running on http://localhost:3000"
    Expected Result: Server announces it's listening
    Evidence: Terminal output captured
  ```

  **Commit**: YES
  - Message: `feat(web-server): wire up main entry point`
  - Files: `web-server/src/main.rs`

---

- [ ] 8. Full API Verification with curl

  **What to do**:
  - Start the server in background
  - Test all endpoints with curl commands
  - Verify correct HTTP status codes and JSON responses
  - Test error cases (404 for missing todos)
  - Document any issues found

  **Must NOT do**:
  - Do not skip any endpoint
  - Do not ignore error responses

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: curl commands for verification
  - **Skills**: None needed

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 5 (sequential)
  - **Blocks**: Task 9
  - **Blocked By**: Task 7

  **References**:
  - All endpoint definitions from Task 6

  **Acceptance Criteria**:
  - [ ] Health endpoint returns 200 with `{"status": "ok"}`
  - [ ] POST /todos creates todo, returns 201 with created todo
  - [ ] GET /todos returns 200 with array of todos
  - [ ] GET /todos/:id returns 200 with single todo
  - [ ] PUT /todos/:id returns 200 with updated todo
  - [ ] DELETE /todos/:id returns 204
  - [ ] GET /todos/:nonexistent returns 404 with error JSON
  - [ ] Database persists across server restart

  **Agent-Executed QA Scenarios**:

  ```
  Scenario: Health endpoint returns OK
    Tool: Bash
    Preconditions: Server running on localhost:3000
    Steps:
      1. curl -s -w "\n%{http_code}" http://localhost:3000/health
      2. Assert: HTTP status is 200
      3. Assert: response body contains "ok"
    Expected Result: {"status":"ok"} with 200
    Evidence: curl output captured

  Scenario: Create todo returns 201 with created todo
    Tool: Bash
    Preconditions: Server running
    Steps:
      1. curl -s -w "\n%{http_code}" -X POST http://localhost:3000/todos \
           -H "Content-Type: application/json" \
           -d '{"title":"Test Todo","description":"Test description"}'
      2. Assert: HTTP status is 201
      3. Assert: response contains "id"
      4. Assert: response contains "Test Todo"
    Expected Result: Created todo with id, title, description
    Evidence: curl output captured

  Scenario: List todos returns created todo
    Tool: Bash
    Preconditions: Todo created in previous scenario
    Steps:
      1. curl -s -w "\n%{http_code}" http://localhost:3000/todos
      2. Assert: HTTP status is 200
      3. Assert: response is array
      4. Assert: array contains todo with "Test Todo"
    Expected Result: Array with at least one todo
    Evidence: curl output captured

  Scenario: Get single todo by ID
    Tool: Bash
    Preconditions: Todo with ID 1 exists
    Steps:
      1. curl -s -w "\n%{http_code}" http://localhost:3000/todos/1
      2. Assert: HTTP status is 200
      3. Assert: response contains "id":1
    Expected Result: Single todo object
    Evidence: curl output captured

  Scenario: Update todo
    Tool: Bash
    Preconditions: Todo with ID 1 exists
    Steps:
      1. curl -s -w "\n%{http_code}" -X PUT http://localhost:3000/todos/1 \
           -H "Content-Type: application/json" \
           -d '{"completed":true}'
      2. Assert: HTTP status is 200
      3. Assert: response contains "completed":true
    Expected Result: Updated todo with completed=true
    Evidence: curl output captured

  Scenario: Delete todo
    Tool: Bash
    Preconditions: Todo with ID 1 exists
    Steps:
      1. curl -s -w "\n%{http_code}" -X DELETE http://localhost:3000/todos/1
      2. Assert: HTTP status is 204
      3. curl -s -w "\n%{http_code}" http://localhost:3000/todos/1
      4. Assert: HTTP status is 404
    Expected Result: 204 on delete, 404 on subsequent get
    Evidence: curl output captured

  Scenario: Get non-existent todo returns 404
    Tool: Bash
    Preconditions: Server running, no todo with ID 999
    Steps:
      1. curl -s -w "\n%{http_code}" http://localhost:3000/todos/999
      2. Assert: HTTP status is 404
      3. Assert: response contains "error"
    Expected Result: {"error":"..."} with 404
    Evidence: curl output captured

  Scenario: Data persists across restart
    Tool: Bash
    Preconditions: Server was running with todos created
    Steps:
      1. Create a todo with distinctive title "Persistence Test"
      2. Stop the server
      3. Restart the server
      4. GET /todos
      5. Assert: response contains "Persistence Test"
    Expected Result: Todo survives server restart
    Evidence: curl output before and after restart
  ```

  **Commit**: NO (verification only)

---

- [ ] 9. Integration Tests

  **What to do**:
  - Create `tests/api_tests.rs` with integration tests:
    - Test setup: spawn server on random port, return client
    - Test health endpoint
    - Test full CRUD cycle
    - Test 404 error handling
  - Use `reqwest` for HTTP requests
  - Use `tokio::spawn` to run server in background

  **Must NOT do**:
  - Do not test internal implementation details
  - Do not create excessive test cases (focus on happy path + key errors)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Standard Rust test patterns
  - **Skills**: None needed

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 5 (after Task 8)
  - **Blocks**: None (final task)
  - **Blocked By**: Task 8

  **References**:
  - Task 8 curl scenarios (convert to reqwest assertions)
  - Rust test patterns

  **Acceptance Criteria**:
  - [ ] `tests/api_tests.rs` exists
  - [ ] Tests cover: health, create, read, update, delete, 404
  - [ ] `cargo test` passes all tests
  - [ ] Tests are independent (don't rely on shared state)

  **Agent-Executed QA Scenarios**:

  ```
  Scenario: All integration tests pass
    Tool: Bash
    Preconditions: All previous tasks complete
    Steps:
      1. cd /Users/byzantium/github/learn/rust/web-server
      2. cargo test 2>&1
      3. Assert: exit code is 0
      4. Assert: output contains "test result: ok"
      5. Assert: output shows multiple tests passed
    Expected Result: All tests green
    Evidence: Test output captured

  Scenario: Tests provide meaningful coverage
    Tool: Bash
    Preconditions: Tests exist
    Steps:
      1. grep -c "#\[tokio::test\]" tests/api_tests.rs
      2. Assert: count >= 5 (health + 4 CRUD operations minimum)
    Expected Result: At least 5 test functions
    Evidence: grep output captured
  ```

  **Commit**: YES
  - Message: `test(web-server): add integration tests for API endpoints`
  - Files: `web-server/tests/api_tests.rs`

---

## Commit Strategy

| After Task | Message | Files | Pre-commit Verification |
|------------|---------|-------|------------------------|
| 1 | `feat(web-server): initialize project with dependencies` | Cargo.toml, src/main.rs, .gitignore | `cargo check` |
| 2 | `feat(web-server): add database setup with migrations` | src/db.rs, migrations/*.sql | `cargo check` |
| 3 | `feat(web-server): add centralized error handling` | src/error.rs | `cargo check` |
| 6 | `feat(web-server): add Todo CRUD routes and models` | src/routes/*.rs, src/models/*.rs | `cargo check` |
| 7 | `feat(web-server): wire up main entry point` | src/main.rs | `cargo build` |
| 9 | `test(web-server): add integration tests for API endpoints` | tests/api_tests.rs | `cargo test` |

---

## Success Criteria

### Verification Commands
```bash
# Build succeeds
cd /Users/byzantium/github/learn/rust/web-server && cargo build
# Expected: Finished release [optimized] target(s)

# Server starts
cargo run &
# Expected: Server running on http://localhost:3000

# Health check
curl http://localhost:3000/health
# Expected: {"status":"ok"}

# CRUD works
curl -X POST http://localhost:3000/todos -H "Content-Type: application/json" -d '{"title":"Test"}'
# Expected: {"id":1,"title":"Test",...}

# Tests pass
cargo test
# Expected: test result: ok. X passed; 0 failed
```

### Final Checklist
- [ ] All "Must Have" features implemented
- [ ] All "Must NOT Have" guardrails respected
- [ ] All 6 endpoints functional
- [ ] Database persists data
- [ ] All tests pass
- [ ] Clean git history with meaningful commits
