use crate::errors::DbError;
use sqlx::postgres::PgPoolOptions;
use sqlx::{ PgPool};

pub async fn create_pool(database_url: &str) -> Result<PgPool, DbError> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}
