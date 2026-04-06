use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utils::format_date_time;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CommonKv {
    pub id: i32,
    #[serde(with = "format_date_time")]
    pub create_time: DateTime<Utc>,
    #[serde(with = "format_date_time")]
    pub update_time: DateTime<Utc>,
    pub key: String,
    pub value: String,
}

// Authorizer 授权账号
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Authorizer {
    pub id: i32,
    pub appid: String,
    pub app_type: i32,
    pub service_type: i32,
    pub nickname: String,
    pub username: String,
    pub headimg: String,
    pub qrcodeurl: String,
    pub principalname: String,
    pub refreshtoken: String,
    pub funcinfo: String,
    pub verifyinfo: i32,
    #[serde(with = "format_date_time")]
    pub auth_time: DateTime<Utc>,
}

// WxCallbackComponentRecord 第三方授权事件的记录
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct WxCallbackComponentRecord {
    pub id: i32,
    #[serde(with = "format_date_time")]
    pub receive_time: DateTime<Utc>,
    #[serde(with = "format_date_time")]
    pub create_time: DateTime<Utc>,
    pub info_type: String,
    pub post_body: String,
}

// WxCallbackBizRecord 小程序授权事件记录
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct WxCallbackBizRecord {
    pub id: i32,
    #[serde(with = "format_date_time")]
    pub receive_time: DateTime<Utc>,
    #[serde(with = "format_date_time")]
    pub create_time: DateTime<Utc>,
    pub appid: String,
    pub to_user_name: String,
    pub msg_type: String,
    pub event: String,
    pub info_type: String,
    pub post_body: String,
}

// WxCallbackRule 回调消息转发规则
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct WxCallbackRule {
    pub id: i32,
    #[serde(with = "format_date_time")]
    pub update_time: DateTime<Utc>,
    #[serde(with = "format_date_time")]
    pub create_time: DateTime<Utc>,
    pub name: String,
    pub r#type: i32, // 使用 raw identifier 因为 type 是 Rust 关键字
    pub msg_type: String,
    pub event: String,
    pub info_type: String,
    pub info: String,
    pub open: i32,
    pub post_body: String,
}

// HttpProxyConfig http转发配置
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct HttpProxyConfig {
    pub id: i32,
    pub port: i32,
    pub path: String,
}

// UserRecord 用户信息
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: i32,
    pub username: String,
    pub password: String,
    #[serde(with = "format_date_time")]
    pub create_time: DateTime<Utc>,
    #[serde(with = "format_date_time")]
    pub update_time: DateTime<Utc>,
}

// WxToken 微信相关的token
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct WxToken {
    pub id: i32,
    #[serde(with = "format_date_time")]
    pub create_time: DateTime<Utc>,
    #[serde(with = "format_date_time")]
    pub update_time: DateTime<Utc>,
    pub r#type: String,
    pub appid: String,
    pub token: String,
    #[serde(with = "format_date_time")]
    pub expire_time: DateTime<Utc>,
}

// CommKv 通用的kv
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CommKv {
    pub id: i32,
    #[serde(with = "format_date_time")]
    pub create_time: DateTime<Utc>,
    #[serde(with = "format_date_time")]
    pub update_time: DateTime<Utc>,
    pub key: String,
    pub value: String,
}

// Counter 计数器
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Counter {
    pub id: i32,
    #[serde(with = "format_date_time")]
    pub create_time: DateTime<Utc>,
    #[serde(with = "format_date_time")]
    pub update_time: DateTime<Utc>,
    pub key: String,
    pub value: i32,
}
