use chrono::{DateTime, Utc};
use error::Error;
use sqlx::{Pool, Postgres};
use structs::sqlx::{CommKv, WxCallbackComponentRecord};
use tracing::error;

#[derive(Debug, Clone)]
pub struct WxCallbackRepository {
    pub pool: Pool<Postgres>,
}

impl WxCallbackRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        WxCallbackRepository { pool }
    }

    // 创建组件回调记录
    pub async fn create_component_record(
        &self,
        receive_time: DateTime<Utc>,
        info_type: &str,
        post_body: &str,
    ) -> Result<WxCallbackComponentRecord, Error> {
        const QUERY: &str = r#"
        INSERT INTO "wx_callback_component_record"
        (receive_time, info_type, post_body, create_time)
        VALUES ($1, $2, $3, $4)
        RETURNING *;
        "#;

        let now = Utc::now();
        match sqlx::query_as::<_, WxCallbackComponentRecord>(QUERY)
            .bind(receive_time)
            .bind(info_type)
            .bind(post_body)
            .bind(now)
            .fetch_one(&self.pool)
            .await
        {
            Err(err) => {
                error!("create_component_record error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    // 更新或创建 ticket
    pub async fn upsert_ticket(&self, ticket: &str) -> Result<CommKv, Error> {
        const QUERY: &str = r#"
        INSERT INTO "comm_kv" (key, value, create_time, update_time)
        VALUES ('ticket', $1, $2, $3)
        ON CONFLICT (key) DO UPDATE SET value = $1, update_time = $3
        RETURNING *;
        "#;

        let now = Utc::now();
        match sqlx::query_as::<_, CommKv>(QUERY)
            .bind(ticket)
            .bind(now)
            .bind(now)
            .fetch_one(&self.pool)
            .await
        {
            Err(err) => {
                error!("upsert_ticket error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
