use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
use std::env;
use tracing::{error, info};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::ticket::{CreateTicketRequest, ListTicketsResponse, MyListingsQuery, ReserveTicketResponse, Ticket, TicketStatus, UpdateTicketRequest};
use crate::utils::auth::validate_admin_key;
use crate::utils::jwt::extract_user_id;
use chrono::Utc;

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

    // Get transfer deadline hours from environment variable (default: 24 hours)
    let transfer_deadline_hours: i64 = env::var("TRANSFER_DEADLINE_HOURS")
        .unwrap_or_else(|_| "24".to_string())
        .parse()
        .unwrap_or(24);

    // Insert ticket with status='unverified' and calculate transfer_deadline
    let ticket = sqlx::query_as::<_, Ticket>(
        r#"
        INSERT INTO tickets (
            seller_id, game_id, event_name, event_date,
            level, seat_section, seat_row, seat_number, price, status,
            transfer_deadline
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW() + INTERVAL '1 hour' * $11)
        RETURNING id, seller_id, game_id, event_name, event_date,
                  level, seat_section, seat_row, seat_number, price, status,
                  transfer_deadline, price_at_reservation, reserved_at, reserved_by, created_at, updated_at
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
    .bind(&transfer_deadline_hours)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!("Failed to create ticket: {}", e);
        e
    })?;

    info!("Ticket created: {} for game {}", ticket.id, ticket.game_id);

    Ok((StatusCode::CREATED, Json(ticket)))
}

/// List all verified tickets (public endpoint, no authentication required)
pub async fn list_tickets(
    State(pool): State<PgPool>,
) -> Result<Json<ListTicketsResponse>> {
    // Get all tickets with status='verified' (available for sale)
    let tickets = sqlx::query_as::<_, Ticket>(
        r#"
        SELECT id, seller_id, game_id, event_name, event_date,
               level, seat_section, seat_row, seat_number, price, status,
               transfer_deadline, price_at_reservation, reserved_at, reserved_by, created_at, updated_at
        FROM tickets
        WHERE status = $1
        ORDER BY event_date ASC, created_at ASC
        "#,
    )
    .bind(&TicketStatus::Verified)
    .fetch_all(&pool)
    .await?;

    info!("Listed {} verified tickets", tickets.len());

    Ok(Json(ListTicketsResponse { tickets }))
}

/// List user's own tickets (authenticated endpoint)
pub async fn my_listings(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Query(params): Query<MyListingsQuery>,
) -> Result<Json<ListTicketsResponse>> {
    // Extract user_id from JWT token
    let user_id = extract_user_id(&headers)?;
    info!("Listing tickets for seller_id: {}", user_id);

    // Build query based on optional status filter
    let tickets = if let Some(status_str) = params.status {
        // Parse status string to enum
        let status = match status_str.to_lowercase().as_str() {
            "unverified" => TicketStatus::Unverified,
            "verified" => TicketStatus::Verified,
            "reserved" => TicketStatus::Reserved,
            "paid" => TicketStatus::Paid,
            "sold" => TicketStatus::Sold,
            "refunding" => TicketStatus::Refunding,
            "cancelled" => TicketStatus::Cancelled,
            _ => {
                error!("Invalid status filter: {}", status_str);
                return Err(AppError::Internal(anyhow::anyhow!("Invalid status filter")));
            }
        };

        sqlx::query_as::<_, Ticket>(
            r#"
            SELECT id, seller_id, game_id, event_name, event_date,
                   level, seat_section, seat_row, seat_number, price, status,
                   transfer_deadline, price_at_reservation, reserved_at, reserved_by, created_at, updated_at
            FROM tickets
            WHERE seller_id = $1 AND status = $2
            ORDER BY created_at DESC
            "#,
        )
        .bind(&user_id)
        .bind(&status)
        .fetch_all(&pool)
        .await?
    } else {
        // No status filter - return all tickets for this user
        sqlx::query_as::<_, Ticket>(
            r#"
            SELECT id, seller_id, game_id, event_name, event_date,
                   level, seat_section, seat_row, seat_number, price, status,
                   transfer_deadline, price_at_reservation, reserved_at, reserved_by, created_at, updated_at
            FROM tickets
            WHERE seller_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(&user_id)
        .fetch_all(&pool)
        .await?
    };

    info!("Listed {} tickets for seller {}", tickets.len(), user_id);

    Ok(Json(ListTicketsResponse { tickets }))
}

/// Reserve a ticket (verified → reserved)
pub async fn reserve_ticket(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(ticket_id): Path<Uuid>,
) -> Result<Json<ReserveTicketResponse>> {
    // Extract buyer_id from JWT token
    let buyer_id = extract_user_id(&headers)?;
    info!("Reserve request for ticket {} by buyer {}", ticket_id, buyer_id);

    // Get total reservation window minutes from environment variable (default: 7 minutes)
    let total_reservation_window_minutes: i64 = env::var("TOTAL_RESERVATION_WINDOW_MINUTES")
        .unwrap_or_else(|_| "7".to_string())
        .parse()
        .unwrap_or(7);

    // Calculate expiry time: NOW() - INTERVAL '1 minute' * TOTAL_RESERVATION_WINDOW_MINUTES
    // Tickets with reserved_at older than this are considered expired
    let expiry_time = Utc::now() - chrono::Duration::minutes(total_reservation_window_minutes);

    // Atomic reservation query
    // This updates the ticket to reserved status only if:
    // - status is 'verified', OR
    // - status is 'reserved' AND reserved_at < expiry_time (expired reservation)
    let result = sqlx::query_as::<_, (Uuid, i32, chrono::DateTime<Utc>)>(
        r#"
        UPDATE tickets
        SET status = 'reserved',
            reserved_at = NOW(),
            reserved_by = $1,
            price_at_reservation = price,
            updated_at = NOW()
        WHERE id = $2
          AND (
            status = 'verified'
            OR (
              status = 'reserved'
              AND reserved_at < $3
            )
          )
        RETURNING id, price_at_reservation, reserved_at
        "#,
    )
    .bind(&buyer_id)
    .bind(&ticket_id)
    .bind(&expiry_time)
    .fetch_optional(&pool)
    .await?;

    match result {
        Some((ticket_id, price_at_reservation, reserved_at)) => {
            info!(
                "Ticket {} reserved by buyer {} at price {}",
                ticket_id, buyer_id, price_at_reservation
            );

            Ok(Json(ReserveTicketResponse {
                ticket_id,
                status: TicketStatus::Reserved,
                price_at_reservation,
                reserved_at,
            }))
        }
        None => {
            info!("Ticket {} is not available for reservation", ticket_id);
            Err(AppError::Conflict("Ticket is no longer available".to_string()))
        }
    }
}

