use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::ticket::{CreateTicketRequest, ListTicketsResponse, MyListingsQuery, Ticket, TicketStatus, UpdateTicketRequest};
use crate::utils::auth::validate_admin_key;
use crate::utils::jwt::extract_user_id;

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

/// List all verified tickets (public endpoint, no authentication required)
pub async fn list_tickets(
    State(pool): State<PgPool>,
) -> Result<Json<ListTicketsResponse>> {
    // Get all tickets with status='verified' (available for sale)
    let tickets = sqlx::query_as::<_, Ticket>(
        r#"
        SELECT id, seller_id, game_id, event_name, event_date,
               level, seat_section, seat_row, seat_number, price, status,
               reserved_at, reserved_by, created_at, updated_at
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
                   reserved_at, reserved_by, created_at, updated_at
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
                   reserved_at, reserved_by, created_at, updated_at
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

/// Update a ticket (admin verification, user cancellation, or price update)
/// Admin (ADMIN_API_KEY): can verify tickets (status: "verified")
/// User (JWT): can cancel tickets (status: "cancelled") or update price
pub async fn update_ticket(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(ticket_id): Path<Uuid>,
    Json(req): Json<UpdateTicketRequest>,
) -> Result<Json<Ticket>> {
    info!("Received update ticket request for ticket_id: {}", ticket_id);

    // Ensure at least one field is being updated
    if req.status.is_none() && req.price.is_none() {
        return Err(AppError::Internal(anyhow::anyhow!("At least one field (status or price) must be provided for update.")));
    }

    // Fetch the ticket
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

    // Check authentication and determine operation type
    let is_admin = validate_admin_key(&headers).is_ok();
    
    let mut new_status = ticket.status;
    let mut new_price = ticket.price;

    if is_admin {
        // Admin operations: verification only
        info!("Admin key validated for ticket update");
        
        if req.price.is_some() {
            error!("Admin cannot update ticket price");
            return Err(AppError::Internal(anyhow::anyhow!("Admin can only verify tickets, not update price")));
        }

        if let Some(status_str) = req.status {
            match status_str.to_lowercase().as_str() {
                "verified" => {
                    // Validate ticket is in unverified state
                    if !matches!(ticket.status, TicketStatus::Unverified) {
                        error!("Ticket {} is not unverified, current status: {:?}", ticket_id, ticket.status);
                        return Err(AppError::Internal(anyhow::anyhow!(
                            "Ticket must be in unverified state to be verified"
                        )));
                    }
                    new_status = TicketStatus::Verified;
                    info!("Admin verifying ticket {}", ticket_id);
                }
                _ => {
                    error!("Invalid status for admin update: {}", status_str);
                    return Err(AppError::Internal(anyhow::anyhow!("Admin can only set status to 'verified'")));
                }
            }
        } else {
            error!("Admin must provide status field for verification");
            return Err(AppError::Internal(anyhow::anyhow!("Admin must provide status field for verification")));
        }
    } else {
        // User operations: cancellation and price updates (requires ownership)
        let user_id = extract_user_id(&headers)?;
        info!("User ID: {}", user_id);

        // Ownership check
        if ticket.seller_id != user_id {
            error!("User {} attempted to update ticket {} owned by {}", user_id, ticket_id, ticket.seller_id);
            return Err(AppError::Forbidden);
        }

        // Validate that ticket is in a state that allows updates (unverified or verified only)
        let can_update = matches!(ticket.status, TicketStatus::Unverified | TicketStatus::Verified);
        if !can_update {
            error!("Cannot update ticket {} with status: {:?}", ticket_id, ticket.status);
            return Err(AppError::Internal(anyhow::anyhow!(
                "Cannot update ticket. Only unverified or verified tickets can be updated."
            )));
        }

        // Process status update (cancel)
        if let Some(status_str) = req.status {
            match status_str.to_lowercase().as_str() {
                "cancelled" => {
                    new_status = TicketStatus::Cancelled;
                    info!("Cancelling ticket {}", ticket_id);
                }
                _ => {
                    error!("Invalid status for user update: {}", status_str);
                    return Err(AppError::Internal(anyhow::anyhow!("Users can only set status to 'cancelled'")));
                }
            }
        }

        // Process price update
        if let Some(price) = req.price {
            if price < 0 {
                error!("Invalid price: {}", price);
                return Err(AppError::Internal(anyhow::anyhow!("Price must be >= 0")));
            }
            new_price = price;
            info!("Updating price for ticket {} to {}", ticket_id, price);
        }
    }

    // Update the ticket
    let updated_ticket = sqlx::query_as::<_, Ticket>(
        r#"
        UPDATE tickets
        SET status = $1, price = $2, updated_at = NOW()
        WHERE id = $3
        RETURNING id, seller_id, game_id, event_name, event_date,
                  level, seat_section, seat_row, seat_number, price, status,
                  reserved_at, reserved_by, created_at, updated_at
        "#,
    )
    .bind(&new_status)
    .bind(&new_price)
    .bind(&ticket_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!("Failed to update ticket {}: {}", ticket_id, e);
        e
    })?;

    info!("Ticket updated: {} - status: {:?}, price: {}", ticket_id, new_status, new_price);

    Ok(Json(updated_ticket))
}

