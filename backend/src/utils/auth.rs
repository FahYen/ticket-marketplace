use axum::http::HeaderMap;
use std::env;
use crate::error::{AppError, Result};

/// Validate admin API key from Authorization header
pub fn validate_admin_key(headers: &HeaderMap) -> Result<()> {
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

