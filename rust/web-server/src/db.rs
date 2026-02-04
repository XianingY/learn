use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    // Ensure data directory exists
    std::fs::create_dir_all("./data").ok();

    let connection_options = SqliteConnectOptions::from_str("sqlite://./data/todos.db")?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await?;

    // Run embedded migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
