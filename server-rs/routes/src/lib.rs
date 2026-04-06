mod user;

pub mod middleware;

use crate::middleware::rate_limit::RateLimit;
use actix_web::web;
use actix_web::{http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use config::AppConfig;
use database::Repository;
use service::Service;
use sqlx::{Pool, Postgres};
use user::configure_auth_routes;

#[derive(Clone)]
pub struct AppState {
    pub service: Service,
    pub jwt_token: String,
}

impl AppState {
    pub fn new(db: Pool<Postgres>, config: AppConfig) -> AppState {
        let repository = Repository::new(db);
        let service = Service::new(repository);

        AppState {
            service,
            jwt_token: config.jwt_token,
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig, rate_limit: &RateLimit) {
    configure_auth_routes(cfg, rate_limit);
}




// 使用 untagged 宏，序列化出来的 JSON 不会带有 "Success" 或 "Error" 的外壳
#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success {
        data: T,
        code: i32,
        message: String,
    },
    Error {
        data: String,
        code: i32,
        message: String,
    },
}

impl<T> ApiResponse<T> {
    // 快捷构造成功响应
    pub fn success(data: T) -> Self {
        ApiResponse::Success {
            data,
            code: 0,
            message: "请求成功".to_string(),
        }
    }

    // 快捷构造失败响应 (完美还原: code != 0 时, result.data = message)
    pub fn error(code: i32, message: &str) -> Self {
        ApiResponse::Error {
            data: message.to_string(),
            code,
            message: message.to_string(),
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
            Err(_) => HttpResponse::InternalServerError()
                .body("JSON Serialization Error"),
        }
    }
}