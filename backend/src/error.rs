use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Invalid email format")]
    InvalidEmail,

    #[error("Email must be an MSU email (must end with msu.edu)")]
    NotSchoolEmail,

    #[error("Password must be at least 8 characters")]
    PasswordTooShort,

    #[error("Invalid verification code")]
    InvalidVerificationCode,

    #[error("Verification code expired")]
    VerificationCodeExpired,

    #[error("Invalid email or password")]
    InvalidCredentials,

    #[error("Email not verified")]
    EmailNotVerified,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Invalid sport type")]
    InvalidSportType,

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::EmailAlreadyExists => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::InvalidEmail => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::NotSchoolEmail => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::PasswordTooShort => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::InvalidVerificationCode => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::VerificationCodeExpired => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::EmailNotVerified => (StatusCode::FORBIDDEN, self.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::InvalidSportType => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ),
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

