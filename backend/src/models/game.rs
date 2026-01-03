use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Database sport_type enum mapping
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "sport_type", rename_all = "lowercase")]
pub enum SportType {
    Football,
    Basketball,
    Hockey,
}

/// Game model from database
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Game {
    pub id: Uuid,
    pub sport_type: SportType,
    pub name: String,
    pub game_time: DateTime<Utc>,
    pub cutoff_time: DateTime<Utc>,
}

/// Request to create a game (admin endpoint)
#[derive(Debug, Deserialize)]
pub struct CreateGameRequest {
    pub sport_type: String,
    pub name: String,
    pub game_time: DateTime<Utc>,
}

/// Response for list games endpoint
#[derive(Debug, Serialize)]
pub struct ListGamesResponse {
    pub games: Vec<Game>,
}
