use error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use structs::sqlx::CommKv;
use tokio::try_join;
use tracing::error;
use wechat_third_platform::Config;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub jwt_token: String,
    pub wx_service: Config,
}

pub const ENV_DATABASE_URL: &str = "DATABASE_URL";
pub const ENV_PORT: &str = "PORT";

pub const DEFAULT_PORT: u16 = 9999;
pub const DEFAULT_DB_URL: &str = "postgresql://postgres:123@localhost:5432/mplat";

impl AppConfig {
    pub async fn load(db: &Pool<Postgres>) -> Result<AppConfig, Error> {
        let (jwt_secret, appid, aes_key, token, secret) = try_join!(
            get_setting_by_name(db, "jwt_secret"),
            get_setting_by_name(db, "appid"),
            get_setting_by_name(db, "encodingAESKey"),
            get_setting_by_name(db, "token"),
            get_setting_by_name(db, "secret"),
        )?;
        let wx_service = Config::new(&appid.value, &token.value, &secret.value, &aes_key.value)?;
        Ok(AppConfig {
            jwt_token: jwt_secret.value,
            wx_service,
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
