use std::env;
use std::sync::Arc;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn init_db() -> Result<Arc<PgPool>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Run migrations (if you use sqlx-cli with migrations folder)
    // sqlx::migrate!().run(&pool).await?;

    Ok(Arc::new(pool))
}