use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};
use sqlx::PgPool;
use std::env;
use tracing::{error, info};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::ticket::{
    ClaimTicketRequest, ClaimTicketResponse, CreateTicketRequest, ListTicketsResponse,
    MyListingsQuery, ReserveTicketResponse, Ticket, TicketStatus, TicketStatusResponse,
};
use crate::utils::auth::{acquire_bot_permit, validate_bot_key};
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
            "verifying" => TicketStatus::Verifying,
            "verified" => TicketStatus::Verified,
            "reserved" => TicketStatus::Reserved,
            "paid" => TicketStatus::Paid,
            "sold" => TicketStatus::Sold,
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

/// Bot claim ticket (unverified → verifying)
pub async fn claim_ticket(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(req): Json<ClaimTicketRequest>,
) -> Result<Json<ClaimTicketResponse>> {
    validate_bot_key(&headers)?;
    let _permit = acquire_bot_permit().await?;

    let result = sqlx::query_as::<_, ClaimTicketResponse>(
        r#"
        UPDATE tickets t
        SET status = 'verifying',
            updated_at = NOW()
        FROM (
            SELECT id
            FROM tickets
            WHERE status = 'unverified'
              AND event_name = $1
              AND seat_section = $2
              AND seat_row = $3
              AND seat_number = $4
              AND transfer_deadline > NOW()
            ORDER BY created_at ASC
            FOR UPDATE SKIP LOCKED
            LIMIT 1
        ) candidate
        WHERE t.id = candidate.id
        RETURNING
            t.id AS ticket_id,
            t.seller_id,
            t.event_name,
            t.seat_section,
            t.seat_row,
            t.seat_number,
            t.status
        "#,
    )
    .bind(&req.event_name)
    .bind(&req.seat_section)
    .bind(&req.seat_row)
    .bind(&req.seat_number)
    .fetch_optional(&pool)
    .await?;

    match result {
        Some(resp) => {
            info!(
                "Ticket {} claimed by bot for event {} seat {}-{}-{}",
                resp.ticket_id, resp.event_name, resp.seat_section, resp.seat_row, resp.seat_number
            );
            Ok(Json(resp))
        }
        None => {
            info!(
                "No unverified ticket available for event {} seat {}-{}-{}",
                req.event_name, req.seat_section, req.seat_row, req.seat_number
            );
            Err(AppError::NotFound("No matching unverified ticket found".to_string()))
        }
    }
}

/// Bot verify ticket (verifying → verified)
pub async fn verify_ticket(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(ticket_id): Path<Uuid>,
) -> Result<Json<TicketStatusResponse>> {
    validate_bot_key(&headers)?;
    let _permit = acquire_bot_permit().await?;

    let result = sqlx::query_as::<_, TicketStatusResponse>(
        r#"
        UPDATE tickets
        SET status = 'verified',
            updated_at = NOW()
        WHERE id = $1
          AND status = 'verifying'
        RETURNING id AS ticket_id, status
        "#,
    )
    .bind(&ticket_id)
    .fetch_optional(&pool)
    .await?;

    match result {
        Some(resp) => {
            info!("Ticket {} moved to verified", ticket_id);
            Ok(Json(resp))
        }
        None => {
            info!("Ticket {} not in verifying state for verification", ticket_id);
            Err(AppError::Conflict("Ticket not in verifying state".to_string()))
        }
    }
}

/// Bot unclaim (verifying → unverified)
pub async fn unclaim_ticket(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(ticket_id): Path<Uuid>,
) -> Result<Json<TicketStatusResponse>> {
    validate_bot_key(&headers)?;
    let _permit = acquire_bot_permit().await?;

    let result = sqlx::query_as::<_, TicketStatusResponse>(
        r#"
        UPDATE tickets
        SET status = 'unverified',
            updated_at = NOW()
        WHERE id = $1
          AND status = 'verifying'
        RETURNING id AS ticket_id, status
        "#,
    )
    .bind(&ticket_id)
    .fetch_optional(&pool)
    .await?;

    match result {
        Some(resp) => {
            info!("Ticket {} rolled back to unverified", ticket_id);
            Ok(Json(resp))
        }
        None => {
            info!("Ticket {} not in verifying state for rollback", ticket_id);
            Err(AppError::Conflict("Ticket not in verifying state".to_string()))
        }
    }
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

    // Check reservation limit per user
    let max_reservations: i64 = env::var("MAX_RESERVATIONS_PER_USER")
        .unwrap_or_else(|_| "3".to_string())
        .parse()
        .unwrap_or(3);

    let active_count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM tickets
        WHERE reserved_by = $1
          AND status = 'reserved'
          AND reserved_at > $2
        "#,
    )
    .bind(&buyer_id)
    .bind(&expiry_time)
    .fetch_one(&pool)
    .await?;

    if active_count.0 >= max_reservations {
        info!(
            "User {} has {} active reservations, limit is {}",
            buyer_id, active_count.0, max_reservations
        );
        return Err(AppError::Conflict(format!(
            "Maximum {} concurrent reservations allowed",
            max_reservations
        )));
    }

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

