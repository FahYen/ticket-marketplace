use axum::{
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use sqlx::PgPool;

mod db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::new(std::env::var("RUST_LOG").expect("RUST_LOG must be set"))
        )
        .init();

    // Create database connection pool
    let pool = db::create_pool().await?;

    // Run database migrations
    db::run_migrations(&pool).await?;

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(pool) // Share database pool with all routes
        .layer(CorsLayer::permissive());

    // Run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "healthy",
            "service": "ticket-marketplace-backend",
            "version": env!("CARGO_PKG_VERSION")
        })),
    )
}

