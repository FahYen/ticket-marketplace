use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use sqlx::PgPool;
use tracing::{info, warn};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::user::{
    LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, UserInfo,
    VerifyEmailRequest, VerifyEmailResponse,
};
use crate::utils::email::{generate_verification_code, validate_school_email};
use crate::utils::jwt::generate_token;
use crate::utils::password::{hash_password, validate_password, verify_password};

/// Register a new user - creates account with unverified email and sends verification code
pub async fn register(
    State(pool): State<PgPool>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegisterResponse>)> {
    // Validate email is an MSU email
    validate_school_email(&req.email)?;

    // Validate password
    validate_password(&req.password)?;

    // Check if email already exists
    let existing_user = sqlx::query_as::<_, (Uuid,)>("SELECT id FROM users WHERE email = $1")
        .bind(&req.email)
        .fetch_optional(&pool)
        .await?;

    if existing_user.is_some() {
        return Err(AppError::EmailAlreadyExists);
    }

    // Hash password
    let password_hash = hash_password(&req.password)?;

    // Generate verification code
    let verification_code = generate_verification_code();

    // Create user record with email_verified = false
    let user_id = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO users (email, password_hash, email_verified, verification_code)
        VALUES ($1, $2, false, $3)
        RETURNING id
        "#,
    )
    .bind(&req.email)
    .bind(&password_hash)
    .bind(&verification_code)
    .fetch_one(&pool)
    .await?;

    info!("User registered: {} (ID: {})", req.email, user_id);

    // TODO: Send verification code via email service
    // For now, log it (in production, remove this logging and send email)
    info!("Verification code for {}: {}", req.email, verification_code);

    // In development, we'll return the code in the response
    // In production, remove this and only send via email
    let response = RegisterResponse {
        message: "Registration successful. Please check your email for verification code.".to_string(),
        verification_code: Some(verification_code), // TODO: Remove in production
    };

    Ok((StatusCode::CREATED, Json(response)))
}



/// Verify email with verification code and activate account
pub async fn verify_email(
    State(pool): State<PgPool>,
    Json(req): Json<VerifyEmailRequest>,
) -> Result<(StatusCode, Json<VerifyEmailResponse>)> {
    // Find user by email and verification code
    let user_result = sqlx::query_as::<_, (Uuid,)>(
        r#"
        SELECT id FROM users
        WHERE email = $1 AND verification_code = $2 AND email_verified = false
        "#,
    )
    .bind(&req.email)
    .bind(&req.code)
    .fetch_optional(&pool)
    .await?;

    let user_id = match user_result {
        Some((id,)) => id,
        None => {
            warn!("Invalid verification code for email: {}", req.email);
            return Err(AppError::InvalidVerificationCode);
        }
    };

    // Update user to mark email as verified and clear verification code
    sqlx::query(
        r#"
        UPDATE users
        SET email_verified = true, verification_code = NULL
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .execute(&pool)
    .await?;

    info!("Email verified for user: {} (ID: {})", req.email, user_id);

    let response = VerifyEmailResponse {
        message: "Email verified successfully. Your account is now active.".to_string(),
        user_id,
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Login user - validates credentials and returns JWT token
pub async fn login(
    State(pool): State<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>)> {
    // Find user by email
    let user = sqlx::query_as::<_, (Uuid, String, bool, String)>(
        r#"
        SELECT id, email, email_verified, password_hash
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(&req.email)
    .fetch_optional(&pool)
    .await?;

    let (user_id, email, email_verified, password_hash) = match user {
        Some(user) => user,
        None => {
            warn!("Login attempt with invalid email: {}", req.email);
            return Err(AppError::InvalidCredentials);
        }
    };

    // Verify password
    let password_valid = verify_password(&req.password, &password_hash)?;
    if !password_valid {
        warn!("Login attempt with invalid password for email: {}", req.email);
        return Err(AppError::InvalidCredentials);
    }

    // Check if email is verified
    if !email_verified {
        warn!("Login attempt with unverified email: {}", req.email);
        return Err(AppError::EmailNotVerified);
    }

    // Generate JWT token
    let token = generate_token(&user_id.to_string(), &email)?;

    info!("User logged in: {} (ID: {})", email, user_id);

    let response = LoginResponse {
        token,
        user: UserInfo {
            id: user_id,
            email,
            email_verified,
        },
    };

    Ok((StatusCode::OK, Json(response)))
}

