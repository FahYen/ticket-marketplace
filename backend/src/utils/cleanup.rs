use sqlx::PgPool;
use std::env;
use tokio::time::{interval, Duration};
use tracing::{error, info};

async fn cleanup_expired_unverified(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM tickets
        WHERE id IN (
            SELECT id
            FROM tickets
            WHERE status = 'unverified'
              AND transfer_deadline <= NOW()
            FOR UPDATE SKIP LOCKED
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

async fn cleanup_stuck_verifying(pool: &PgPool, verifying_timeout_minutes: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE tickets
        SET status = 'unverified',
            updated_at = NOW()
        WHERE status = 'verifying'
          AND updated_at < NOW() - INTERVAL '1 minute' * $1
        "#,
    )
    .bind(&verifying_timeout_minutes)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

async fn cleanup_expired_reservations(pool: &PgPool, total_reservation_window_minutes: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE tickets
        SET status = 'verified',
            reserved_at = NULL,
            reserved_by = NULL,
            price_at_reservation = NULL,
            updated_at = NOW()
        WHERE status = 'reserved'
          AND reserved_at < NOW() - INTERVAL '1 minute' * $1
        "#,
    )
    .bind(&total_reservation_window_minutes)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub fn start_cleanup_tasks(pool: PgPool) {
    // Cleanup expired unverified tickets
    {
        let pool = pool.clone();
        let interval_hours = env::var("TRANSFER_DEADLINE_CLEANUP_INTERVAL_HOURS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(1);
        let mut ticker = interval(Duration::from_secs(interval_hours * 3600));

        tokio::spawn(async move {
            loop {
                ticker.tick().await;
                match cleanup_expired_unverified(&pool).await {
                    Ok(affected) => {
                        if affected > 0 {
                            info!("Expired unverified cleanup removed {} tickets", affected);
                        }
                    }
                    Err(e) => error!("Expired unverified cleanup failed: {}", e),
                }
            }
        });
    }

    // Cleanup stuck verifying tickets
    {
        let pool = pool.clone();
        let interval_seconds = env::var("VERIFYING_CLEANUP_INTERVAL_SECONDS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(60);
        let verifying_timeout_minutes = env::var("VERIFYING_TIMEOUT_MINUTES")
            .ok()
            .and_then(|v| v.parse::<i64>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(10);
        let mut ticker = interval(Duration::from_secs(interval_seconds));

        tokio::spawn(async move {
            loop {
                ticker.tick().await;
                match cleanup_stuck_verifying(&pool, verifying_timeout_minutes).await {
                    Ok(affected) => {
                        if affected > 0 {
                            info!("Stuck verifying cleanup reset {} tickets", affected);
                        }
                    }
                    Err(e) => error!("Stuck verifying cleanup failed: {}", e),
                }
            }
        });
    }

    // Cleanup expired reservations
    {
        let pool = pool.clone();
        let interval_seconds = env::var("RESERVATION_CLEANUP_INTERVAL_SECONDS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(60);
        let total_reservation_window_minutes = env::var("TOTAL_RESERVATION_WINDOW_MINUTES")
            .ok()
            .and_then(|v| v.parse::<i64>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(7);
        let mut ticker = interval(Duration::from_secs(interval_seconds));

        tokio::spawn(async move {
            loop {
                ticker.tick().await;
                match cleanup_expired_reservations(&pool, total_reservation_window_minutes).await {
                    Ok(affected) => {
                        if affected > 0 {
                            info!("Reservation cleanup reset {} tickets", affected);
                        }
                    }
                    Err(e) => error!("Reservation cleanup failed: {}", e),
                }
            }
        });
    }
}
