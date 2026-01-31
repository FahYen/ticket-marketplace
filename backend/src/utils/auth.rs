use axum::http::HeaderMap;
use std::env;
use std::sync::{Arc, OnceLock};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
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

/// Validate bot API key from Authorization header
pub fn validate_bot_key(headers: &HeaderMap) -> Result<()> {
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let expected_key = env::var("BOT_API_KEY")
        .map_err(|_| AppError::Internal(anyhow::anyhow!("BOT_API_KEY environment variable must be set")))?;

    if auth_header != expected_key {
        return Err(AppError::Unauthorized);
    }

    Ok(())
}

fn bot_semaphore() -> &'static Arc<Semaphore> {
    static BOT_SEMAPHORE: OnceLock<Arc<Semaphore>> = OnceLock::new();

    BOT_SEMAPHORE.get_or_init(|| {
        let limit = env::var("BOT_CONCURRENCY_LIMIT")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(5);
        Arc::new(Semaphore::new(limit))
    })
}

/// Limit concurrent bot operations to avoid overload or accidental floods
pub async fn acquire_bot_permit() -> Result<OwnedSemaphorePermit> {
    bot_semaphore()
        .clone()
        .acquire_owned()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Bot semaphore closed: {}", e)))
}

