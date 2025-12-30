use sqlx::{PgPool, Connection, pool::PoolOptions};
use std::env;
use tracing::info;

/// Initialize and return a PostgreSQL connection pool
pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let max_connections = match env::var("DB_POOL_MAX_CONNECTIONS") {
        Ok(val) => val.parse().unwrap_or(10),
        Err(_) => 10,
    };
    
    let min_connections = match env::var("DB_POOL_MIN_CONNECTIONS") {
        Ok(val) => val.parse().unwrap_or(2),
        Err(_) => 2,
    };

    let pool = PoolOptions::<sqlx::Postgres>::new()
        .max_connections(max_connections)
        .min_connections(min_connections)
        .connect(&database_url)
        .await?;

    // Test the connection by pinging the database
    let mut conn = pool.acquire().await?;
    conn.ping().await?;
    
    info!("Database connection pool created successfully");

    Ok(pool)
}

/// Run database migrations automatically on startup
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    
    info!("Database migrations completed successfully");
    
    Ok(())
}

