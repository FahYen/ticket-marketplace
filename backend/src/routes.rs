use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use crate::handlers::{auth, games, tickets};

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(crate::health_check))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/verify-email", post(auth::verify_email))
        .route("/api/auth/login", post(auth::login))
        .route("/api/games", get(games::list_games).post(games::create_game))
        .route("/api/games/:id", delete(games::delete_game))
        .route("/api/tickets", get(tickets::list_tickets).post(tickets::create_ticket))
        .route("/api/tickets/my-listings", get(tickets::my_listings))
        .route("/api/tickets/:id/verify", post(tickets::verify_ticket))
        .layer(CorsLayer::permissive())
        .with_state(pool)
}

