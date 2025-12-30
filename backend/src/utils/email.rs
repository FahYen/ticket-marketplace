use crate::error::{AppError, Result};
use rand::Rng;

/// Validates if an email is msu.edu
pub fn validate_school_email(email: &str) -> Result<()> {
    // Basic email format validation
    if !email.contains('@') || email.split('@').count() != 2 {
        return Err(AppError::InvalidEmail);
    }

    let domain = email.split('@').nth(1).unwrap();

    // Check if it ends with msu.edu
    if !domain.ends_with("msu.edu") {
        return Err(AppError::NotSchoolEmail);
    }

    Ok(())
}

/// Generates a 6-digit verification code
pub fn generate_verification_code() -> String {
    let mut rng = rand::thread_rng();
    format!("{:06}", rng.gen_range(100000..=999999))
}

