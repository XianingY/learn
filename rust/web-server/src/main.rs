mod db;
mod error;
mod models;
mod routes;

use axum::Router;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = db::init_db().await?;

    let app = Router::new()
        .route("/health", axum::routing::get(routes::health::health_check))
        .nest("/todos", routes::todos::todo_routes())
        .with_state(pool)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Server running on http://localhost:3000");
    axum::serve(listener, app).await?;

    Ok(())
}
