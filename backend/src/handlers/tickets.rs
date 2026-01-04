use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};
use sqlx::PgPool;
use std::env;
use tracing::{error, info};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::ticket::{CreateTicketRequest, Ticket, TicketStatus};
use crate::utils::jwt::validate_token;

/// Extract and validate JWT token from Authorization header
fn extract_user_id(headers: &HeaderMap) -> Result<Uuid> {
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let claims = validate_token(auth_header)?;
    let user_id = Uuid::parse_str(&claims.id)
        .map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid user ID in token")))?;

    Ok(user_id)
}

/// Create a new ticket listing
pub async fn create_ticket(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(req): Json<CreateTicketRequest>,
) -> Result<(StatusCode, Json<Ticket>)> {
    info!("Received create ticket request for game_id: {}", req.game_id);

    // Extract user_id from JWT token
    let seller_id = extract_user_id(&headers)?;
    info!("Seller ID: {}", seller_id);

    // Validate price
    if req.price < 0 {
        error!("Invalid price: {}", req.price);
        return Err(AppError::Internal(anyhow::anyhow!("Price must be >= 0")));
    }

    // Validate seat details are not empty
    if req.level.trim().is_empty()
        || req.seat_section.trim().is_empty()
        || req.seat_row.trim().is_empty()
        || req.seat_number.trim().is_empty()
    {
        error!("Empty seat details provided");
        return Err(AppError::Internal(anyhow::anyhow!("Seat details cannot be empty")));
    }

    // Fetch game info to populate event_name and event_date
    let game_result = sqlx::query_as::<_, (String, chrono::DateTime<chrono::Utc>)>(
        "SELECT name, game_time FROM games WHERE id = $1",
    )
    .bind(&req.game_id)
    .fetch_optional(&pool)
    .await?;

    let (event_name, event_date) = game_result.ok_or_else(|| {
        error!("Game not found: {}", req.game_id);
        AppError::Internal(anyhow::anyhow!("Game not found"))
    })?;

    info!("Found game: {} at {}", event_name, event_date);

    // Insert ticket with status='unverified'
    let ticket = sqlx::query_as::<_, Ticket>(
        r#"
        INSERT INTO tickets (
            seller_id, game_id, event_name, event_date,
            level, seat_section, seat_row, seat_number, price, status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id, seller_id, game_id, event_name, event_date,
                  level, seat_section, seat_row, seat_number, price, status,
                  reserved_at, reserved_by, created_at, updated_at
        "#,
    )
    .bind(&seller_id)
    .bind(&req.game_id)
    .bind(&event_name)
    .bind(&event_date)
    .bind(&req.level)
    .bind(&req.seat_section)
    .bind(&req.seat_row)
    .bind(&req.seat_number)
    .bind(&req.price)
    .bind(&TicketStatus::Unverified)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!("Failed to create ticket: {}", e);
        e
    })?;

    info!("Ticket created: {} for game {}", ticket.id, ticket.game_id);

    Ok((StatusCode::CREATED, Json(ticket)))
}

/// Extract and validate admin API key from Authorization header
fn validate_admin_key(headers: &HeaderMap) -> Result<()> {
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let expected_key = env::var("ADMIN_API_KEY")
        .map_err(|_| AppError::Internal(anyhow::anyhow!("ADMIN_API_KEY environment variable must be set")))?;

    if auth_header != expected_key {
        return Err(AppError::Unauthorized);
    }

    Ok(())
}

/// Verify a ticket (admin/bot endpoint)
/// Changes status from 'unverified' to 'verified'
pub async fn verify_ticket(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(ticket_id): Path<Uuid>,
) -> Result<Json<Ticket>> {
    info!("Received verify ticket request for ticket_id: {}", ticket_id);

    // Validate admin API key
    validate_admin_key(&headers)?;
    info!("Admin key validated for verify ticket");

    // Check if ticket exists and is unverified
    let ticket = sqlx::query_as::<_, Ticket>(
        r#"
        SELECT id, seller_id, game_id, event_name, event_date,
               level, seat_section, seat_row, seat_number, price, status,
               reserved_at, reserved_by, created_at, updated_at
        FROM tickets
        WHERE id = $1
        "#,
    )
    .bind(&ticket_id)
    .fetch_optional(&pool)
    .await?;

    let ticket = ticket.ok_or_else(|| {
        error!("Ticket not found: {}", ticket_id);
        AppError::Internal(anyhow::anyhow!("Ticket not found"))
    })?;

    // Validate ticket is in unverified state
    if !matches!(ticket.status, TicketStatus::Unverified) {
        error!("Ticket {} is not unverified, current status: {:?}", ticket_id, ticket.status);
        return Err(AppError::Internal(anyhow::anyhow!(
            "Ticket must be in unverified state to be verified"
        )));
    }

    // Update ticket status to verified
    let verified_ticket = sqlx::query_as::<_, Ticket>(
        r#"
        UPDATE tickets
        SET status = $1
        WHERE id = $2
        RETURNING id, seller_id, game_id, event_name, event_date,
                  level, seat_section, seat_row, seat_number, price, status,
                  reserved_at, reserved_by, created_at, updated_at
        "#,
    )
    .bind(&TicketStatus::Verified)
    .bind(&ticket_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!("Failed to verify ticket {}: {}", ticket_id, e);
        e
    })?;

    info!("Ticket verified: {}", ticket_id);

    Ok(Json(verified_ticket))
}

