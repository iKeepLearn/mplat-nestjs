mod admin;
mod auth_page;
mod user;
mod wxcallback;

pub mod middleware;

use crate::middleware::rate_limit::RateLimit;
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse, Responder, http::header::ContentType};
use admin::configure_admin_routes;
use auth_page::configure_auth_page_routes;
use chrono::{TimeZone, Utc};
use config::AppConfig;
use database::Repository;
use serde::Serialize;
use service::Service;
use sqlx::{Pool, Postgres};
use structs::sqlx::{CommKv, WxToken};
use tracing::error;
use user::configure_auth_routes;
use wechat_third_platform::WxStorage;
use wechat_third_platform::error::Error as WtpError;
use wechat_third_platform::service::WxService;
use wxcallback::configure_wxcallback_routes;

#[derive(Clone)]
pub struct AppState {
    pub service: Service,
    pub jwt_token: String,
    pub wx_service: WxService<PostgresqlDb>,
}

#[derive(Debug, Clone)]
pub struct PostgresqlDb {
    pub db: Pool<Postgres>,
}

impl PostgresqlDb {
    fn new(db: Pool<Postgres>) -> Self {
        PostgresqlDb { db }
    }
}

#[async_trait::async_trait]
impl WxStorage for PostgresqlDb {
    async fn get_ticket(&self) -> Result<String, WtpError> {
        const QUERY: &str = r#"SELECT * FROM "comm_kv" WHERE key = $1;"#;
        match sqlx::query_as::<_, CommKv>(QUERY)
            .bind("ticket")
            .fetch_one(&self.db)
            .await
        {
            Err(err) => {
                error!("get ticket error: {:?}", &err);
                Err(WtpError::Config("no ticket".to_string()))
            }
            Ok(res) => Ok(res.value),
        }
    }

    async fn get_component_token(&self, appid: &str) -> Result<Option<(String, u64)>, WtpError> {
        const QUERY: &str = "SELECT * FROM wx_token WHERE appid = $1 AND type = $2";

        match sqlx::query_as::<_, WxToken>(QUERY)
            .bind(appid)
            .bind("component_access_token")
            .fetch_optional(&self.db)
            .await
        {
            Ok(res) => {
                let token = res.map(|r| (r.token, r.expire_time.timestamp() as u64));
                Ok(token)
            }
            Err(e) => {
                error!("get component token err:{:?}", e);
                Err(WtpError::Config("no component token".to_string()))
            }
        }
    }

    async fn save_component_token(
        &self,
        appid: &str,
        token: &str,
        expire_time: u64,
    ) -> Result<(), WtpError> {
        const QUERY: &str = "INSERT INTO wx_token (appid, type, token, expire_time) 
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (appid, type) 
             DO UPDATE SET token = $3, expire_time = $4";
        let time = Utc
            .timestamp_opt(expire_time as i64, 0)
            .single()
            .unwrap_or_default();
        match sqlx::query(QUERY)
            .bind(appid)
            .bind("component_access_token")
            .bind(token)
            .bind(time)
            .execute(&self.db)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("save component token err:{:?}", e);
                Err(WtpError::Config("save component token failed".to_string()))
            }
        }
    }
}

impl AppState {
    pub fn new(db: Pool<Postgres>, config: AppConfig) -> AppState {
        let repository = Repository::new(db.clone());
        let service = Service::new(repository);
        let pg = PostgresqlDb::new(db);
        let wx_service = WxService::new(config.wx_service, pg);

        AppState {
            service,
            jwt_token: config.jwt_token,
            wx_service,
        }
    }
}

// 使用 untagged 宏，序列化出来的 JSON 不会带有 "Success" 或 "Error" 的外壳
#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success {
        data: T,
        code: i32,
        #[serde(rename = "errorMsg")]
        error_msg: String,
    },
    Error {
        data: Option<String>,
        code: i32,
        #[serde(rename = "errorMsg")]
        error_msg: String,
    },
}

impl<T> ApiResponse<T> {
    // 快捷构造成功响应
    pub fn success(data: T) -> Self {
        ApiResponse::Success {
            data,
            code: 0,
            error_msg: "请求成功".to_string(),
        }
    }

    // 快捷构造失败响应 (完美还原: code != 0 时, result.data = message)
    pub fn error(code: i32, message: &str) -> Self {
        ApiResponse::Error {
            data: None,
            code,
            error_msg: message.to_string(),
        }
    }
}

impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        // 直接序列化为 JSON 字符串
        match serde_json::to_string(&self) {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_) => HttpResponse::InternalServerError().body("JSON Serialization Error"),
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig, rate_limit: &RateLimit) {
    configure_admin_routes(cfg, rate_limit);
    configure_wxcallback_routes(cfg);
    configure_auth_page_routes(cfg, rate_limit);
    configure_auth_routes(cfg, rate_limit);
}
