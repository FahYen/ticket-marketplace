use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Payment Intent status enum
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_intent_status", rename_all = "lowercase")]
pub enum PaymentIntentStatus {
    Created,
    Capturable,
    Captured,
    Cancelled,
}

/// Payment Intent model from database
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PaymentIntent {
    pub id: String, // Stripe payment intent ID (e.g., "pi_xxx")
    pub ticket_id: Uuid,
    pub buyer_id: Uuid,
    pub amount: i32,
    pub currency: String, // ISO currency code (e.g., "usd")
    pub status: PaymentIntentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Stripe webhook event payload structure
#[derive(Debug, Deserialize)]
pub struct StripeWebhookEvent {
    pub id: Option<String>, // Event ID (evt_xxx) - optional for deserialization
    #[serde(rename = "type")]
    pub event_type: String,
    pub data: StripeWebhookData,
}

#[derive(Debug, Deserialize)]
pub struct StripeWebhookData {
    pub object: StripePaymentIntent,
}

#[derive(Debug, Deserialize)]
pub struct StripePaymentIntent {
    pub id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub metadata: StripeMetadata,
}

#[derive(Debug, Deserialize)]
pub struct StripeMetadata {
    pub ticket_id: String,
    pub buyer_id: String,
    pub reserved_at: String,
}

