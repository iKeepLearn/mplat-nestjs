use chrono::{Duration, TimeZone, Utc};
use error::Error;
use sqlx::{Pool, Postgres};
use structs::sqlx::{
    Authorizer, CommKv, WxCallbackBizRecord, WxCallbackComponentRecord, WxCallbackRule,
};
use tracing::error;

#[derive(Debug, Clone)]
pub struct AdminRepository {
    pub pool: Pool<Postgres>,
}

impl AdminRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        AdminRepository { pool }
    }

    // 更新用户密码
    pub async fn update_password(
        &self,
        username: &str,
        new_password: &str,
    ) -> Result<(), Error> {
        const QUERY: &str = r#"UPDATE "user_record" SET password = $1, update_time = $2 WHERE username = $3;"#;
        let now = Utc::now();
        match sqlx::query(QUERY)
            .bind(new_password)
            .bind(now)
            .bind(username)
            .execute(&self.pool)
            .await
        {
            Err(err) => {
                error!("update_password error: {:?}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }

    // 更新用户名
    pub async fn update_username(
        &self,
        old_username: &str,
        new_username: &str,
    ) -> Result<(), Error> {
        const QUERY: &str = r#"UPDATE "user_record" SET username = $1, update_time = $2 WHERE username = $3;"#;
        let now = Utc::now();
        match sqlx::query(QUERY)
            .bind(new_username)
            .bind(now)
            .bind(old_username)
            .execute(&self.pool)
            .await
        {
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                Err(Error::bad_request("用户名已存在"))
            }
            Err(err) => {
                error!("update_username error: {:?}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }

    // 更新或创建 secret
    pub async fn upsert_secret(&self, secret: &str) -> Result<CommKv, Error> {
        const QUERY: &str = r#"
        INSERT INTO "comm_kv" (key, value, create_time, update_time)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (key) DO UPDATE SET value = $2, update_time = $4
        RETURNING *;
        "#;
        let now = Utc::now();
        match sqlx::query_as::<_, CommKv>(QUERY)
            .bind("secret")
            .bind(secret)
            .bind(now)
            .bind(now)
            .fetch_one(&self.pool)
            .await
        {
            Err(err) => {
                error!("upsert_secret error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    // 获取 secret
    pub async fn get_secret(&self) -> Result<Option<String>, Error> {
        const QUERY: &str = r#"SELECT value FROM "comm_kv" WHERE key = $1;"#;
        match sqlx::query_scalar::<_, String>(QUERY)
            .bind("secret")
            .fetch_optional(&self.pool)
            .await
        {
            Err(err) => {
                error!("get_secret error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    // 更新或创建 redirect_uri
    pub async fn upsert_redirect_uri(&self, redirect_uri: &str) -> Result<CommKv, Error> {
        const QUERY: &str = r#"
        INSERT INTO "comm_kv" (key, value, create_time, update_time)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (key) DO UPDATE SET value = $2, update_time = $4
        RETURNING *;
        "#;
        let now = Utc::now();
        match sqlx::query_as::<_, CommKv>(QUERY)
            .bind("redirect_uri")
            .bind(redirect_uri)
            .bind(now)
            .bind(now)
            .fetch_one(&self.pool)
            .await
        {
            Err(err) => {
                error!("upsert_redirect_uri error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    // 获取授权方访问 token
    pub async fn get_authorizer_access_token(&self, appid: &str) -> Result<Option<String>, Error> {
        const QUERY: &str = r#"
        SELECT token FROM "wx_token"
        WHERE appid = $1 AND type = 'authorizer_access_token';
        "#;
        match sqlx::query_scalar::<_, String>(QUERY)
            .bind(appid)
            .fetch_optional(&self.pool)
            .await
        {
            Err(err) => {
                error!("get_authorizer_access_token error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    // 获取开发小程序列表
    pub async fn get_dev_weapp_list(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Authorizer>, Error> {
        const QUERY: &str = r#"
        SELECT * FROM "authorizer"
        WHERE app_type = 0 AND funcinfo LIKE '%18%'
        LIMIT $1 OFFSET $2;
        "#;
        match sqlx::query_as::<_, Authorizer>(QUERY)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
        {
            Err(err) => {
                error!("get_dev_weapp_list error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    // 获取 component_verify_ticket
    pub async fn get_component_verify_ticket(&self) -> Result<Option<String>, Error> {
        const QUERY: &str = r#"SELECT value FROM "comm_kv" WHERE key = 'ticket';"#;
        match sqlx::query_scalar::<_, String>(QUERY)
            .fetch_optional(&self.pool)
            .await
        {
            Err(err) => {
                error!("get_component_verify_ticket error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    // 获取 component_access_token
    pub async fn get_component_access_token(&self) -> Result<Option<String>, Error> {
        const QUERY: &str = r#"SELECT token FROM "wx_token" WHERE type = 'component_access_token';"#;
        match sqlx::query_scalar::<_, String>(QUERY)
            .fetch_optional(&self.pool)
            .await
        {
            Err(err) => {
                error!("get_component_access_token error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    // 获取微信组件回调记录
    pub async fn get_wx_component_records(
        &self,
        limit: i64,
        offset: i64,
        start_time: Option<chrono::DateTime<Utc>>,
        end_time: Option<chrono::DateTime<Utc>>,
        info_type: Option<String>,
    ) -> Result<Vec<WxCallbackComponentRecord>, Error> {
        let (start, end) = match (start_time, end_time) {
            (Some(s), Some(e)) => (s, e),
            (Some(s), None) => (s, Utc::now()),
            (None, Some(e)) => (
                Utc.with_ymd_and_hms(2022, 11, 24, 0, 0, 0)
                    .single()
                    .unwrap(),
                e,
            ),
            (None, None) => (
                Utc.with_ymd_and_hms(2022, 11, 24, 0, 0, 0)
                    .single()
                    .unwrap(),
                Utc::now(),
            ),
        };

        let (query, bind_info_type) = if let Some(itype) = info_type {
            (
                r#"
                SELECT * FROM "wx_callback_component_record"
                WHERE receive_time >= $1 AND receive_time <= $2 AND info_type = $3
                ORDER BY receive_time DESC
                LIMIT $4 OFFSET $5;
                "#,
                Some(itype),
            )
        } else {
            (
                r#"
                SELECT * FROM "wx_callback_component_record"
                WHERE receive_time >= $1 AND receive_time <= $2
                ORDER BY receive_time DESC
                LIMIT $3 OFFSET $4;
                "#,
                None,
            )
        };

        let mut q = sqlx::query_as::<_, WxCallbackComponentRecord>(query)
            .bind(start)
            .bind(end);

        if let Some(itype) = bind_info_type {
            q = q.bind(itype);
        }

        q = q.bind(limit).bind(offset);

        match q.fetch_all(&self.pool).await {
            Err(err) => {
                error!("get_wx_component_records error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    // 获取微信业务回调记录
    pub async fn get_wx_biz_records(
        &self,
        limit: i64,
        offset: i64,
        start_time: Option<chrono::DateTime<Utc>>,
        end_time: Option<chrono::DateTime<Utc>>,
        appid: Option<String>,
        event: Option<String>,
        msg_type: Option<String>,
    ) -> Result<Vec<WxCallbackBizRecord>, Error> {
        let (start, end) = match (start_time, end_time) {
            (Some(s), Some(e)) => (s, e),
            (Some(s), None) => (s, Utc::now()),
            (None, Some(e)) => (
                Utc.with_ymd_and_hms(2022, 11, 24, 0, 0, 0)
                    .single()
                    .unwrap(),
                e,
            ),
            (None, None) => (
                Utc.with_ymd_and_hms(2022, 11, 24, 0, 0, 0)
                    .single()
                    .unwrap(),
                Utc::now(),
            ),
        };

        // 构建动态查询条件
        let mut conditions = vec!["receive_time >= $1".to_string(), "receive_time <= $2".to_string()];
        let mut binds: Vec<String> = Vec::new();
        let mut param_index = 3;

        if let Some(a) = &appid {
            conditions.push(format!("appid = ${}", param_index));
            binds.push(a.clone());
            param_index += 1;
        }
        if let Some(e) = &event {
            conditions.push(format!("event = ${}", param_index));
            binds.push(e.clone());
            param_index += 1;
        }
        if let Some(m) = &msg_type {
            conditions.push(format!("msg_type = ${}", param_index));
            binds.push(m.clone());
            param_index += 1;
        }

        let query = format!(
            r#"
            SELECT * FROM "wx_callback_biz_record"
            WHERE {}
            ORDER BY receive_time DESC
            LIMIT ${} OFFSET ${};
            "#,
            conditions.join(" AND "),
            param_index,
            param_index + 1
        );

        let mut q = sqlx::query_as::<_, WxCallbackBizRecord>(&query)
            .bind(start)
            .bind(end);

        for bind in binds {
            q = q.bind(bind);
        }

        q = q.bind(limit).bind(offset);

        match q.fetch_all(&self.pool).await {
            Err(err) => {
                error!("get_wx_biz_records error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    // 获取代理配置
    pub async fn get_proxy_config(&self) -> Result<Vec<CommKv>, Error> {
        const QUERY: &str = r#"SELECT key, value FROM "comm_kv" WHERE key IN ('proxy_state', 'proxy_port');"#;
        match sqlx::query_as::<_, CommKv>(QUERY)
            .fetch_all(&self.pool)
            .await
        {
            Err(err) => {
                error!("get_proxy_config error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    // 更新代理配置
    pub async fn update_proxy_config(
        &self,
        open: &str,
        port: &str,
    ) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        const QUERY_STATE: &str = r#"
        INSERT INTO "comm_kv" (key, value, create_time, update_time)
        VALUES ('proxy_state', $1, $2, $3)
        ON CONFLICT (key) DO UPDATE SET value = $1, update_time = $3;
        "#;

        const QUERY_PORT: &str = r#"
        INSERT INTO "comm_kv" (key, value, create_time, update_time)
        VALUES ('proxy_port', $1, $2, $3)
        ON CONFLICT (key) DO UPDATE SET value = $1, update_time = $3;
        "#;

        let now = Utc::now();
        sqlx::query(QUERY_STATE)
            .bind(open)
            .bind(now)
            .bind(now)
            .execute(&mut *tx)
            .await?;

        sqlx::query(QUERY_PORT)
            .bind(port)
            .bind(now)
            .bind(now)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    // 获取回调代理规则列表
    pub async fn get_callback_proxy_rule_list(
        &self,
        r#type: i32,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<WxCallbackRule>, Error> {
        const QUERY: &str = r#"
        SELECT * FROM "wx_callback_rule"
        WHERE type = $1
        LIMIT $2 OFFSET $3;
        "#;
        match sqlx::query_as::<_, WxCallbackRule>(QUERY)
            .bind(r#type)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
        {
            Err(err) => {
                error!("get_callback_proxy_rule_list error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    // 添加回调代理规则
    pub async fn add_callback_proxy_rule(
        &self,
        name: &str,
        r#type: i32,
        event: &str,
        msg_type: &str,
        info_type: &str,
        info: &str,
        open: i32,
        post_body: &str,
    ) -> Result<WxCallbackRule, Error> {
        const QUERY: &str = r#"
        INSERT INTO "wx_callback_rule"
        (name, type, msg_type, event, info_type, info, open, post_body, create_time, update_time)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING *;
        "#;
        let now = Utc::now();
        match sqlx::query_as::<_, WxCallbackRule>(QUERY)
            .bind(name)
            .bind(r#type)
            .bind(msg_type)
            .bind(event)
            .bind(info_type)
            .bind(info)
            .bind(open)
            .bind(post_body)
            .bind(now)
            .bind(now)
            .fetch_one(&self.pool)
            .await
        {
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                Err(Error::bad_request("已存在相同规则"))
            }
            Err(err) => {
                error!("add_callback_proxy_rule error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    // 更新回调代理规则
    pub async fn update_callback_proxy_rule(
        &self,
        id: i32,
        data: serde_json::Value,
    ) -> Result<WxCallbackRule, Error> {
        // 先获取现有记录
        let existing = self.get_callback_proxy_rule_by_id(id).await?;

        // 构建更新字段
        let mut name = existing.name;
        let mut r#type = existing.r#type;
        let mut msg_type = existing.msg_type;
        let mut event = existing.event;
        let mut info_type = existing.info_type;
        let mut info = existing.info;
        let mut open = existing.open;
        let mut post_body = existing.post_body;

        if let Some(obj) = data.as_object() {
            for (key, value) in obj {
                match key.as_str() {
                    "name" => {
                        if let Some(s) = value.as_str() {
                            name = s.to_string();
                        }
                    }
                    "type" => {
                        if let Some(i) = value.as_i64() {
                            r#type = i as i32;
                        }
                    }
                    "msgType" => {
                        if let Some(s) = value.as_str() {
                            msg_type = s.to_string();
                        }
                    }
                    "event" => {
                        if let Some(s) = value.as_str() {
                            event = s.to_string();
                        }
                    }
                    "infoType" => {
                        if let Some(s) = value.as_str() {
                            info_type = s.to_string();
                        }
                    }
                    "info" => {
                        if let Some(s) = value.as_str() {
                            info = s.to_string();
                        }
                    }
                    "open" => {
                        if let Some(b) = value.as_bool() {
                            open = if b { 1 } else { 0 };
                        } else if let Some(i) = value.as_i64() {
                            open = i as i32;
                        }
                    }
                    "data" => {
                        post_body = value.to_string();
                    }
                    _ => {}
                }
            }
        }

        const QUERY: &str = r#"
        UPDATE "wx_callback_rule"
        SET name = $1, type = $2, msg_type = $3, event = $4, info_type = $5,
            info = $6, open = $7, post_body = $8, update_time = $9
        WHERE id = $10
        RETURNING *;
        "#;

        match sqlx::query_as::<_, WxCallbackRule>(QUERY)
            .bind(name)
            .bind(r#type)
            .bind(msg_type)
            .bind(event)
            .bind(info_type)
            .bind(info)
            .bind(open)
            .bind(post_body)
            .bind(Utc::now())
            .bind(id)
            .fetch_one(&self.pool)
            .await
        {
            Err(err) => {
                error!("update_callback_proxy_rule error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    // 根据 ID 获取回调代理规则
    async fn get_callback_proxy_rule_by_id(&self, id: i32) -> Result<WxCallbackRule, Error> {
        const QUERY: &str = r#"SELECT * FROM "wx_callback_rule" WHERE id = $1;"#;
        match sqlx::query_as::<_, WxCallbackRule>(QUERY)
            .bind(id)
            .fetch_one(&self.pool)
            .await
        {
            Err(err) => {
                error!("get_callback_proxy_rule_by_id error: {:?}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    // 删除回调代理规则
    pub async fn delete_callback_proxy_rule(&self, id: i32) -> Result<(), Error> {
        const QUERY: &str = r#"DELETE FROM "wx_callback_rule" WHERE id = $1;"#;
        match sqlx::query(QUERY)
            .bind(id)
            .execute(&self.pool)
            .await
        {
            Err(err) => {
                error!("delete_callback_proxy_rule error: {:?}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }

    // 批量添加或更新授权方信息和 token (用于事务)
    pub async fn upsert_authorizer_and_token(
        &self,
        records: &[structs::dto::admin::AuthorizerInfo],
    ) -> Result<(), Error> {
        let now = Utc::now();

        for record in records {
            // 插入或更新 authorizer
            const AUTHORIZER_QUERY: &str = r#"
            INSERT INTO "authorizer"
            (appid, app_type, service_type, nickname, username, headimg, qrcodeurl, principalname, refreshtoken, funcinfo, verifyinfo, auth_time)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (appid) DO UPDATE SET
                app_type = $2, service_type = $3, nickname = $4, username = $5,
                headimg = $6, qrcodeurl = $7, principalname = $8, refreshtoken = $9,
                funcinfo = $10, verifyinfo = $11, auth_time = $12
            RETURNING *;
            "#;

            sqlx::query(AUTHORIZER_QUERY)
                .bind(&record.appid)
                .bind(record.app_type)
                .bind(record.service_type)
                .bind(&record.nick_name)
                .bind(&record.user_name)
                .bind(&record.head_img)
                .bind(&record.qrcode_url)
                .bind(&record.principal_name)
                .bind(&record.refresh_token)
                .bind(&record.func_info)
                .bind(record.verify_info)
                .bind(now)
                .execute(&self.pool)
                .await?;

            // 如果有 token，插入或更新 token
            if let Some(token) = &record.access_token {
                let expires_in = record.expires_in.unwrap_or(7200);
                let expire_time = now + Duration::seconds(expires_in);

                const TOKEN_QUERY: &str = r#"
                INSERT INTO "wx_token"
                (appid, type, token, expire_time, create_time, update_time)
                VALUES ($1, 'authorizer_access_token', $2, $3, $4, $5)
                ON CONFLICT (appid, type) DO UPDATE SET
                    token = $2, expire_time = $3, update_time = $5;
                "#;

                sqlx::query(TOKEN_QUERY)
                    .bind(&record.appid)
                    .bind(token)
                    .bind(expire_time)
                    .bind(now)
                    .bind(now)
                    .execute(&self.pool)
                    .await?;
            }
        }

        Ok(())
    }
}
