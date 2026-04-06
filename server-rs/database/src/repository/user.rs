use chrono::Utc;
use error::Error;
use sqlx::{Pool, Postgres};
use structs::sqlx::UserRecord;
use tracing::error;

#[derive(Debug, Clone)]
pub struct UserRepository {
    pub pool: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        UserRepository { pool }
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<UserRecord>, Error> {
        const QUERY: &str = r#"SELECT * FROM "user_record" WHERE username = $1;"#;
        match sqlx::query_as::<_, UserRecord>(QUERY)
            .bind(username)
            .fetch_optional(&self.pool)
            .await
        {
            Err(err) => {
                error!("find_by_username error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    pub async fn find_by_username_with_password_only(
        &self,
        username: &str,
    ) -> Result<Option<String>, Error> {
        const QUERY: &str = r#"SELECT password FROM "user_record" WHERE username = $1;"#;
        match sqlx::query_scalar::<_, String>(QUERY)
            .bind(username)
            .fetch_optional(&self.pool)
            .await
        {
            Err(err) => {
                error!("find_by_username_with_password_only error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    pub async fn create_user(&self, username: &str, password: &str) -> Result<UserRecord, Error> {
        const QUERY: &str = r#"
        INSERT INTO "user_record" (username, password, create_time, update_time)
        VALUES ($1, $2, $3, $4)
        RETURNING *;
        "#;

        let now = Utc::now();
        match sqlx::query_as::<_, UserRecord>(QUERY)
            .bind(username)
            .bind(password)
            .bind(now)
            .bind(now)
            .fetch_one(&self.pool)
            .await
        {
            Err(err) => {
                error!("create_user error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
