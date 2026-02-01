use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Json,
};
use chrono::Utc;
use sqlx::PgPool;
use std::env;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::payment_intent::{
    PaymentIntent, PaymentIntentStatus, StripeWebhookEvent,
};
use crate::utils::stripe::{cancel_payment_intent, capture_payment_intent, verify_stripe_webhook_signature};

/// Handle Stripe webhook for payment_intent.amount_capturable_updated event
/// This event fires when funds are authorized and ready to capture
pub async fn handle_stripe_webhook(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    body: String,
) -> Result<(StatusCode, Json<serde_json::Value>)> {
    info!("Received Stripe webhook");

    // Verify webhook signature
    let signature = headers
        .get("stripe-signature")
        .and_then(|hv| hv.to_str().ok())
        .ok_or_else(|| {
            error!("Missing Stripe-Signature header");
            AppError::Unauthorized
        })?;

    verify_stripe_webhook_signature(&body, signature)?;

    // Parse webhook payload
    let event: StripeWebhookEvent = serde_json::from_str(&body).map_err(|e| {
        error!("Failed to parse webhook payload: {}", e);
        AppError::Internal(anyhow::anyhow!("Invalid webhook payload"))
    })?;

    info!("Webhook event type: {}", event.event_type);

    // Only process payment_intent.amount_capturable_updated events
    // This event fires when funds are authorized and ready to capture
    // NOTE: Verify this event type exists in Stripe - may need to use payment_intent.requires_capture instead
    if event.event_type != "payment_intent.amount_capturable_updated" {
        info!("Ignoring event type: {}", event.event_type);
        return Ok((
            StatusCode::OK,
            Json(serde_json::json!({ "received": true })),
        ));
    }

    let payment_intent = &event.data.object;

    // Extract metadata
    let ticket_id = Uuid::parse_str(&payment_intent.metadata.ticket_id).map_err(|e| {
        error!("Invalid ticket_id in metadata: {}", e);
        AppError::Internal(anyhow::anyhow!("Invalid ticket_id in metadata"))
    })?;

    let buyer_id = Uuid::parse_str(&payment_intent.metadata.buyer_id).map_err(|e| {
        error!("Invalid buyer_id in metadata: {}", e);
        AppError::Internal(anyhow::anyhow!("Invalid buyer_id in metadata"))
    })?;

    info!(
        "Processing payment intent {} for ticket {} by buyer {}",
        payment_intent.id, ticket_id, buyer_id
    );

    // Store payment intent record (for idempotency)
    // Using ON CONFLICT DO NOTHING to handle duplicate webhook deliveries
    let result = sqlx::query_as::<_, PaymentIntent>(
        r#"
        INSERT INTO payment_intents (id, ticket_id, buyer_id, amount, currency, status)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (id) DO NOTHING
        RETURNING id, ticket_id, buyer_id, amount, currency, status, created_at, updated_at
        "#,
    )
    .bind(&payment_intent.id)
    .bind(&ticket_id)
    .bind(&buyer_id)
    .bind(&(payment_intent.amount as i32))
    .bind(&payment_intent.currency)
    .bind(&PaymentIntentStatus::Capturable)
    .fetch_optional(&pool)
    .await?;

    match result {
        Some(_) => {
            // First time processing this payment intent
            info!(
                "Payment intent {} stored successfully for ticket {}",
                payment_intent.id, ticket_id
            );
            
            // Stage 4: Gatekeeper Check (reserved → paid)
            perform_gatekeeper_check(
                &pool,
                &payment_intent.id,
                ticket_id,
                buyer_id,
            ).await?;
            
            Ok((
                StatusCode::OK,
                Json(serde_json::json!({
                    "received": true,
                    "payment_intent_id": payment_intent.id
                })),
            ))
        }
        None => {
            // Duplicate webhook - already processed
            info!(
                "Payment intent {} already processed (idempotent)",
                payment_intent.id
            );
            Ok((
                StatusCode::OK,
                Json(serde_json::json!({ "received": true, "duplicate": true })),
            ))
        }
    }
}

/// Stage 4: Gatekeeper Check
/// Determines if the "freeze" becomes a "charge" or a "release"
/// 
/// Happy Path: Ticket is still reserved by this buyer within the reservation window
///   - Updates ticket status to 'paid'
///   - Captures payment from buyer
/// 
/// Late Path: Reservation expired
///   - Cancels payment intent (releases authorization hold)
///   - Ticket status remains unchanged (will be reset to 'verified' by cleanup job)
async fn perform_gatekeeper_check(
    pool: &PgPool,
    payment_intent_id: &str,
    ticket_id: Uuid,
    buyer_id: Uuid,
) -> Result<()> {
    info!(
        "Performing gatekeeper check for payment intent {} on ticket {}",
        payment_intent_id, ticket_id
    );

    // Get total reservation window minutes from environment variable (default: 7 minutes)
    let total_reservation_window_minutes: i64 = env::var("TOTAL_RESERVATION_WINDOW_MINUTES")
        .unwrap_or_else(|_| "7".to_string())
        .parse()
        .unwrap_or(7);

    // Calculate expiry time: NOW() - INTERVAL '1 minute' * TOTAL_RESERVATION_WINDOW_MINUTES
    // Tickets with reserved_at older than this are considered expired
    let expiry_time = Utc::now() - chrono::Duration::minutes(total_reservation_window_minutes);

    let update_result = sqlx::query_as::<_, (Uuid, i32)>(
        r#"
        UPDATE tickets
        SET status = 'paid',
            updated_at = NOW()
        WHERE id = $1
          AND status = 'reserved'
          AND reserved_by = $2
          AND reserved_at > $3
        RETURNING id, price_at_reservation
        "#,
    )
    .bind(&ticket_id)
    .bind(&buyer_id)
    .bind(&expiry_time)
    .fetch_optional(pool)
    .await?;

    match update_result {
        Some((_, _price_at_reservation)) => {
            // Branch A: Happy Path - Reservation is still valid
            info!(
                "Gatekeeper check passed: Ticket {} reserved by buyer {} within {} minutes",
                ticket_id, buyer_id, total_reservation_window_minutes
            );

            // Update payment intent status to 'captured'
            sqlx::query(
                r#"
                UPDATE payment_intents
                SET status = 'captured',
                    updated_at = NOW()
                WHERE id = $1
                "#,
            )
            .bind(payment_intent_id)
            .execute(pool)
            .await?;

            info!(
                "Updated payment intent {} status to 'captured'",
                payment_intent_id
            );

            // Call Stripe API to capture funds
            capture_payment_intent(payment_intent_id).await.map_err(|e| {
                error!(
                    "Failed to capture payment intent {} for ticket {}: {}",
                    payment_intent_id, ticket_id, e
                );
                // Ticket status is already 'paid' - this requires manual intervention
                // Log error but don't fail the webhook (return 200 to Stripe)
                // TODO: Implement retry mechanism or alert system
                e
            })?;

            info!(
                "Successfully captured payment for ticket {} (payment intent {})",
                ticket_id, payment_intent_id
            );

            Ok(())
        }
        None => {
            // Branch B: Late Path - Reservation expired
            warn!(
                "Gatekeeper check failed: Ticket {} reservation expired or invalid (buyer {}, reservation window: {} minutes)",
                ticket_id, buyer_id, total_reservation_window_minutes
            );

            // Update payment intent status to 'cancelled'
            sqlx::query(
                r#"
                UPDATE payment_intents
                SET status = 'cancelled',
                    updated_at = NOW()
                WHERE id = $1
                "#,
            )
            .bind(payment_intent_id)
            .execute(pool)
            .await?;

            info!(
                "Updated payment intent {} status to 'cancelled'",
                payment_intent_id
            );

            // Call Stripe API to cancel authorization (release hold)
            cancel_payment_intent(payment_intent_id).await.map_err(|e| {
                error!(
                    "Failed to cancel payment intent {} for ticket {}: {}",
                    payment_intent_id, ticket_id, e
                );
                // Log error but don't fail the webhook (return 200 to Stripe)
                // May need manual intervention to release hold
                e
            })?;

            info!(
                "Successfully cancelled payment intent {} for expired reservation on ticket {}",
                payment_intent_id, ticket_id
            );

            Ok(())
        }
    }
}

