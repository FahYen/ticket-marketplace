use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use crate::handlers::auth;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(crate::health_check))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/verify-email", post(auth::verify_email))
        .layer(CorsLayer::permissive())
        .with_state(pool)
}

