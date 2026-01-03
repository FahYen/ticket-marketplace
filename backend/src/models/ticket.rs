use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Database ticket_status enum mapping
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ticket_status")]
pub enum TicketStatus {
    Unverified,
    Verified,
    Reserved,
    Sold,
    Cancelled,
}

/// Ticket model from database
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Ticket {
    pub id: Uuid,
    pub seller_id: Uuid,
    pub game_id: Uuid,
    pub event_name: String,
    pub event_date: DateTime<Utc>,
    pub level: String,
    pub seat_section: String,
    pub seat_row: String,
    pub seat_number: String,
    pub price: i32,
    pub status: TicketStatus,
    #[serde(skip_serializing)]
    pub reserved_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing)]
    pub reserved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub updated_at: DateTime<Utc>,
}

/// Request to create a ticket listing
#[derive(Debug, Deserialize)]
pub struct CreateTicketRequest {
    pub game_id: Uuid,
    pub level: String,
    pub seat_section: String,
    pub seat_row: String,
    pub seat_number: String,
    pub price: i32,
}

/// Request to update ticket status (verify, cancel, etc.)
#[derive(Debug, Deserialize)]
pub struct UpdateTicketRequest {
    pub status: Option<String>,
}

