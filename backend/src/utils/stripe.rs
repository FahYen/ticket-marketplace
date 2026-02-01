use stripe::{Client, PaymentIntent, PaymentIntentId, CapturePaymentIntent, CancelPaymentIntent};
use std::str::FromStr;
use std::env;
use tracing::{error, info};

use crate::error::{AppError, Result};

/// Get Stripe client instance
fn get_stripe_client() -> Result<Client> {
    let secret_key = env::var("STRIPE_SECRET_KEY")
        .map_err(|_| AppError::Internal(anyhow::anyhow!("STRIPE_SECRET_KEY environment variable not set")))?;
    
    Ok(Client::new(secret_key))
}

/// Verify Stripe webhook signature
/// 
/// Returns Ok(()) if signature is valid, Err if invalid
pub fn verify_stripe_webhook_signature(
    body: &str,
    signature_header: &str,
) -> Result<()> {
    let webhook_secret = env::var("STRIPE_WEBHOOK_SECRET")
        .map_err(|_| AppError::Internal(anyhow::anyhow!("STRIPE_WEBHOOK_SECRET environment variable not set")))?;

    stripe::Webhook::construct_event(body, signature_header, &webhook_secret)
        .map_err(|_| AppError::Unauthorized)?;

    Ok(())
}

/// Capture a payment intent (charge the buyer)
pub async fn capture_payment_intent(payment_intent_id: &str) -> Result<()> {
    let client = get_stripe_client()?;
    let payment_intent_id = PaymentIntentId::from_str(payment_intent_id)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid payment intent ID: {}", e)))?;

    info!("Capturing payment intent: {}", payment_intent_id);

    PaymentIntent::capture(&client, &payment_intent_id, CapturePaymentIntent::default())
        .await
        .map_err(|e| {
            error!("Failed to capture payment intent {}: {:?}", payment_intent_id, e);
            AppError::Internal(anyhow::anyhow!("Failed to capture payment intent: {}", e))
        })?;

    info!("Successfully captured payment intent: {}", payment_intent_id);
    Ok(())
}

/// Cancel a payment intent (release the authorization hold)
pub async fn cancel_payment_intent(payment_intent_id: &str) -> Result<()> {
    let client = get_stripe_client()?;
    let payment_intent_id = PaymentIntentId::from_str(payment_intent_id)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid payment intent ID: {}", e)))?;

    info!("Cancelling payment intent: {}", payment_intent_id);

    PaymentIntent::cancel(&client, &payment_intent_id, CancelPaymentIntent { cancellation_reason: None })
        .await
        .map_err(|e| {
            error!("Failed to cancel payment intent {}: {:?}", payment_intent_id, e);
            AppError::Internal(anyhow::anyhow!("Failed to cancel payment intent: {}", e))
        })?;

    info!("Successfully cancelled payment intent: {}", payment_intent_id);
    Ok(())
}

