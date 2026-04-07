use error::Error;
use sqlx::{Pool, Postgres};
use structs::sqlx::CommKv;
use tracing::error;

#[derive(Debug, Clone)]
pub struct CommKvRepository {
    pub pool: Pool<Postgres>,
}

impl CommKvRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        CommKvRepository { pool }
    }

    pub async fn get_comm_kv(&self, key: &str) -> Result<CommKv, Error> {
        const QUERY: &str = r#"SELECT * FROM "comm_kv" WHERE key = $1;"#;

        match sqlx::query_as::<_, CommKv>(QUERY)
            .bind(key)
            .fetch_one(&self.pool)
            .await
        {
            Err(err) => {
                error!("get_comm_kv error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
