# Draft: Rust Web Server REST API

## Requirements (confirmed)
- Framework: Axum 0.8.x
- Database: SQLite with SQLx 0.8
- Location: /Users/byzantium/github/learn/rust/web-server/
- Domain: Todo CRUD API

## Technical Decisions
- WAL mode for SQLite concurrency
- Embedded migrations with sqlx::migrate!
- Compile-time checked SQL with query!/query_as! macros
- Centralized error handling with custom AppError
- TraceLayer for HTTP logging
- tower-http for middleware

## Project Structure (confirmed)
```
web-server/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   └── todos.rs
│   ├── models/
│   │   ├── mod.rs
│   │   └── todo.rs
│   ├── error.rs
│   └── db.rs
└── migrations/
    └── 001_create_todos.sql
```

## API Endpoints (confirmed)
- GET /health - health check
- GET /todos - list all
- GET /todos/:id - get single
- POST /todos - create
- PUT /todos/:id - update
- DELETE /todos/:id - delete

## Test Strategy Decision
- **Infrastructure exists**: NO (first project with tests)
- **Automated tests**: YES (tests-after)
- **Framework**: Rust built-in #[tokio::test] with reqwest for integration tests
- **Agent-Executed QA**: ALWAYS (curl verification during development)

## Scope Boundaries
- INCLUDE: Full CRUD, error handling, logging, migrations
- EXCLUDE: Authentication, Docker, CI/CD, rate limiting
