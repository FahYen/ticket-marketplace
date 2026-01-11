use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Database ticket_status enum mapping
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ticket_status", rename_all = "lowercase")]
pub enum TicketStatus {
    Unverified,
    Verified,
    Reserved,
    Paid,
    Sold,
    Refunding,
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
    pub transfer_deadline: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_at_reservation: Option<i32>,
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

/// Request to update ticket (price and/or status)
#[derive(Debug, Deserialize)]
pub struct UpdateTicketRequest {
    pub status: Option<String>,
    pub price: Option<i32>,
}

/// Response for list tickets endpoint
#[derive(Debug, Serialize)]
pub struct ListTicketsResponse {
    pub tickets: Vec<Ticket>,
}

/// Query parameters for my-listings endpoint
#[derive(Debug, Deserialize)]
pub struct MyListingsQuery {
    pub status: Option<String>,
}

/// Response for reserve ticket endpoint
#[derive(Debug, Serialize)]
pub struct ReserveTicketResponse {
    pub ticket_id: Uuid,
    pub status: TicketStatus,
    pub price_at_reservation: i32,
    pub reserved_at: DateTime<Utc>,
}

