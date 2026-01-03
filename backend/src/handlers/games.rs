use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};
use chrono::{Duration, Utc};
use sqlx::PgPool;
use std::env;
use tracing::{info, error};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::game::{CreateGameRequest, Game, ListGamesResponse, SportType};

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

/// Parse sport type string to SportType enum
fn parse_sport_type(s: &str) -> Result<SportType> {
    match s.to_lowercase().as_str() {
        "football" => Ok(SportType::Football),
        "basketball" => Ok(SportType::Basketball),
        "hockey" => Ok(SportType::Hockey),
        _ => Err(AppError::InvalidSportType),
    }
}

/// Create a new game (admin endpoint)
pub async fn create_game(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(req): Json<CreateGameRequest>,
) -> Result<(StatusCode, Json<Game>)> {
    info!("Received create game request: sport_type={}, name={}", req.sport_type, req.name);
    
    // Validate admin API key
    validate_admin_key(&headers)?;
    info!("Admin key validated");

    // Validate inputs
    if req.name.trim().is_empty() {
        error!("Game name is empty");
        return Err(AppError::Internal(anyhow::anyhow!("Game name cannot be empty")));
    }

    // Validate game_time is in the future
    if req.game_time <= Utc::now() {
        error!("Game time is in the past: {}", req.game_time);
        return Err(AppError::Internal(anyhow::anyhow!("Game time must be in the future")));
    }

    // Parse sport type
    let sport_type = parse_sport_type(&req.sport_type)?;
    info!("Sport type parsed: {:?}", sport_type);

    // Calculate cutoff_time
    let cutoff_minutes = env::var("LISTING_CUTOFF_MINUTES")
        .map_err(|e| {
            error!("LISTING_CUTOFF_MINUTES env var error: {}", e);
            AppError::Internal(anyhow::anyhow!("LISTING_CUTOFF_MINUTES environment variable must be set"))
        })?
        .parse::<i64>()
        .map_err(|e| {
            error!("LISTING_CUTOFF_MINUTES parse error: {}", e);
            AppError::Internal(anyhow::anyhow!("LISTING_CUTOFF_MINUTES must be a valid number"))
        })?;

    let cutoff_time = req.game_time
        .checked_sub_signed(Duration::minutes(cutoff_minutes))
        .ok_or_else(|| {
            error!("Invalid cutoff time calculation");
            AppError::Internal(anyhow::anyhow!("Invalid cutoff time calculation"))
        })?;
    
    info!("Cutoff time calculated: {}", cutoff_time);

    // Insert game into database
    let game = sqlx::query_as::<_, Game>(
        r#"
        INSERT INTO games (sport_type, name, game_time, cutoff_time)
        VALUES ($1, $2, $3, $4)
        RETURNING id, sport_type, name, game_time, cutoff_time
        "#,
    )
    .bind(&sport_type)
    .bind(&req.name)
    .bind(&req.game_time)
    .bind(&cutoff_time)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!("Failed to create game: {}", e);
        e
    })?;

    info!("Game created: {} ({})", game.name, game.id);

    Ok((StatusCode::CREATED, Json(game)))
}

/// Delete a game by ID (admin endpoint)
pub async fn delete_game(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(game_id): Path<Uuid>,
) -> Result<StatusCode> {
    // Validate admin API key
    validate_admin_key(&headers)?;

    // Check if game exists
    let game = sqlx::query_scalar::<_, Option<Uuid>>(
        "SELECT id FROM games WHERE id = $1",
    )
    .bind(game_id)
    .fetch_optional(&pool)
    .await?;

    if game.is_none() {
        return Err(AppError::Internal(anyhow::anyhow!("Game not found")));
    }

    // Delete the game
    sqlx::query("DELETE FROM games WHERE id = $1")
        .bind(game_id)
        .execute(&pool)
        .await?;

    info!("Game deleted: {}", game_id);

    Ok(StatusCode::NO_CONTENT)
}

/// List upcoming games (public endpoint)
pub async fn list_games(
    State(pool): State<PgPool>,
) -> Result<Json<ListGamesResponse>> {
    // Get all games where cutoff_time > NOW() (still open for trading)
    let games = sqlx::query_as::<_, Game>(
        r#"
        SELECT id, sport_type, name, game_time, cutoff_time
        FROM games
        WHERE cutoff_time > NOW()
        ORDER BY game_time ASC
        "#,
    )
    .fetch_all(&pool)
    .await?;

    info!("Listed {} upcoming games", games.len());

    Ok(Json(ListGamesResponse { games }))
}

