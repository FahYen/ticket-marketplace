use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use crate::handlers::{auth, games, tickets, webhooks};
use crate::utils::rate_limit::RateLimitLayer;

pub fn create_router(pool: PgPool) -> Router {
    // Create rate-limited reservation route
    let reservation_routes = Router::new()
        .route("/api/tickets/:id/reserve", post(tickets::reserve_ticket))
        .layer(RateLimitLayer::new());

    Router::new()
        .route("/health", get(crate::health_check))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/verify-email", post(auth::verify_email))
        .route("/api/auth/login", post(auth::login))
        .route("/api/games", get(games::list_games).post(games::create_game))
        .route("/api/games/:id", delete(games::delete_game))
        .route("/api/tickets", get(tickets::list_tickets).post(tickets::create_ticket))
        .route("/api/tickets/claim", post(tickets::claim_ticket))
        .route("/api/tickets/:id/verify", patch(tickets::verify_ticket))
        .route("/api/tickets/:id/unclaim", delete(tickets::unclaim_ticket))
        .route("/api/tickets/my-listings", get(tickets::my_listings))
        .route("/api/webhooks/stripe", post(webhooks::handle_stripe_webhook))
        .merge(reservation_routes)
        .layer(CorsLayer::permissive())
        .with_state(pool)
}

