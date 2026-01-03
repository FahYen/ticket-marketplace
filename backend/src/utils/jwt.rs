use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use chrono::{Duration, Utc};
use crate::error::{AppError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user ID (subject)
    pub email: String,
    pub exp: usize, // expiration time
}

const DEFAULT_EXPIRY_HOURS: i64 = 24;

/// Generate a JWT token for a user
pub fn generate_token(user_id: &str, email: &str) -> Result<String> {
    let secret = env::var("JWT_SECRET")
        .map_err(|_| AppError::Internal(anyhow::anyhow!("JWT_SECRET environment variable must be set")))?;

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(DEFAULT_EXPIRY_HOURS))
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Invalid expiration timestamp: date calculation overflow")))?
        .timestamp()
        .try_into()
        .map_err(|_| {
            AppError::Internal(anyhow::anyhow!("Token expiration timestamp out of range: cannot convert to usize"))
        })?;

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Token generation failed: {}", e)))
}

/// Validate a JWT token and return the claims
pub fn validate_token(token: &str) -> Result<Claims> {
    let secret = env::var("JWT_SECRET")
        .map_err(|_| AppError::Internal(anyhow::anyhow!("JWT_SECRET environment variable must be set")))?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid token")))?;

    Ok(token_data.claims)
}

