use error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use structs::sqlx::CommKv;
use tracing::error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub jwt_token: String,
}

pub const ENV_DATABASE_URL: &str = "DATABASE_URL";
pub const ENV_PORT: &str = "PORT";

pub const DEFAULT_PORT: u16 = 9999;
pub const DEFAULT_DB_URL: &str = "postgresql://postgres:123@localhost:5432/mplat";

impl AppConfig {
    pub async fn load(db: &Pool<Postgres>) -> Result<AppConfig, Error> {
        let jwt_secret = get_setting_by_name(db, "jwt_secret").await?;

        Ok(AppConfig {
            jwt_token: jwt_secret.value,
        })
    }
}

async fn get_setting_by_name(db: &Pool<Postgres>, name: &str) -> Result<CommKv, Error> {
    const QUERY: &str = r#"SELECT * FROM "comm_kv" WHERE key = $1;"#;
    match sqlx::query_as::<_, CommKv>(QUERY)
        .bind(name)
        .fetch_one(db)
        .await
    {
        Err(err) => {
            error!("find_setting_by_name error: {:?}", &err);
            Err(err.into())
        }
        Ok(res) => Ok(res),
    }
}
