use bcrypt::{hash, verify, DEFAULT_COST};
use crate::error::{AppError, Result};

const MIN_PASSWORD_LENGTH: usize = 8;

/// Validates password meets minimum requirements
pub fn validate_password(password: &str) -> Result<()> {
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(AppError::PasswordTooShort);
    }
    Ok(())
}

/// Hashes a password using bcrypt
pub fn hash_password(password: &str) -> Result<String> {
    hash(password, DEFAULT_COST)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Password hashing failed: {}", e)))
}

/// Verifies a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    verify(password, hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Password verification failed: {}", e)))
}

