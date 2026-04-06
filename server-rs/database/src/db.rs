use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;
use tracing::error;

pub async fn connect(database_url: &str) -> Result<Pool<Postgres>, error::Error> {
    PgPoolOptions::new()
        .max_connections(20)
        .max_lifetime(Duration::from_secs(30 * 60)) // 30 mins
        .connect(database_url)
        .await
        .map_err(|err| {
            error!("db: connecting to DB: {}", err);
            err.into()
        })
}
