use crate::sqlx::Authorizer;
use chrono::{DateTime, TimeZone, Utc};
use error::Error;
use serde::{Deserialize, Serialize};
use tracing::error;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct ChangePwdDto {
    #[validate(length(min = 1, message = "请确认旧密码是否正确"))]
    pub old_password: String,
    #[validate(length(min = 1, message = "请确认密码是否正确"))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangeUsernameDto {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangeSecretDto {
    pub secret: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecretResponse {
    pub secret: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddComponentInfoDto {
    pub redirect_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorizerListQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorizerListResponse {
    pub records: Vec<AuthorizerInfo>,
    pub total: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorizerInfo {
    pub id: String,
    pub appid: String,
    pub user_name: String,
    pub nick_name: String,
    pub app_type: i32,
    pub service_type: i32,
    pub auth_time: Option<i64>,
    pub principal_name: String,
    pub register_type: i32,
    pub account_status: i32,
    pub basic_config: Option<i32>,
    pub verify_info: i32,
    pub refresh_token: String,
    pub qrcode_url: String,
    pub head_img: String,
    pub func_info: String,
    pub access_token: Option<String>,
    pub expires_in: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppIdQuery {
    pub appid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DevWeappListQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DevWeappListResponse {
    pub records: Vec<Authorizer>,
    pub total: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TicketResponse {
    pub ticket: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComponentAccessTokenResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WxRecordsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub info_type: Option<String>,
    pub appid: Option<String>,
    pub event: Option<String>,
    pub msg_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WxBizRecordsQuery {
    pub limit: i64,
    pub offset: i64,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub appid: Option<String>,
    pub event: Option<String>,
    pub msg_type: Option<String>,
}

impl From<WxRecordsQuery> for WxBizRecordsQuery {
    fn from(value: WxRecordsQuery) -> Self {
        let default_datetime = Utc
            .with_ymd_and_hms(2022, 11, 24, 0, 0, 0)
            .single()
            .expect("Invalid default date");
        let limit = value.limit.unwrap_or(15);
        let offset = value.offset.unwrap_or(0);
        let start_time = value
            .start_time
            .and_then(|ts| Utc.timestamp_opt(ts, 0).single())
            .unwrap_or(default_datetime);
        let end_time = value
            .end_time
            .and_then(|ts| Utc.timestamp_opt(ts, 0).single())
            .unwrap_or(default_datetime);

        WxBizRecordsQuery {
            limit,
            offset,
            start_time,
            end_time,
            appid: value.appid,
            event: value.event,
            msg_type: value.msg_type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WxComponentRecord {
    pub receive_time: String,
    pub info_type: String,
    pub post_body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WxRecordsResponse<T> {
    pub records: Vec<T>,
    pub total: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WxBizRecord {
    pub receive_time: String,
    pub event: String,
    pub msg_type: String,
    pub appid: String,
    pub info_type: String,
    pub post_body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProxyConfigResponse {
    pub open: bool,
    pub port: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProxyConfigDto {
    pub open: bool,
    pub port: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CallbackProxyRuleListQuery {
    pub r#type: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CallbackProxyRule {
    pub name: String,
    pub msg_type: String,
    pub event: String,
    pub open: i32,
    pub update_time: String,
    pub info_type: String,
    pub r#type: i32,
    pub data: serde_json::Value,
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CallbackProxyRuleListResponse {
    pub rules: Vec<CallbackProxyRule>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddCallbackProxyRuleDto {
    pub name: String,
    pub r#type: i32,
    pub event: Option<String>,
    pub msg_type: Option<String>,
    pub info: Option<String>,
    pub open: i32,
    pub data: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CallbackProxyRuleAdd {
    pub name: String,
    pub r#type: i32,
    pub event: String,
    pub msg_type: String,
    pub info_type: String,
    pub info: String,
    pub open: i32,
    pub post_body: String,
}

impl TryFrom<AddCallbackProxyRuleDto> for CallbackProxyRuleAdd {
    type Error = Error;
    fn try_from(value: AddCallbackProxyRuleDto) -> Result<Self, Error> {
        let event = value.event.unwrap_or_default();
        let msg_type = value.msg_type.unwrap_or_default();
        let info = value.info.unwrap_or_default();
        let post_body = serde_json::to_string(&value.data).map_err(|e| {
            error!("serialize post_body error: {:?}", e);
            Error::internal("序列化数据失败")
        })?;
        Ok(CallbackProxyRuleAdd {
            name: value.name,
            r#type: value.r#type,
            event,
            msg_type,
            info_type: info.clone(),
            info,
            open: value.open,
            post_body,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCallbackProxyRuleDto {
    pub id: i32,
    pub data: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteCallbackProxyRuleQuery {
    pub id: i32,
}

// 用于从微信 API 获取授权方列表的临时结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WxAuthorizerList {
    pub list: Option<Vec<WxAuthorizerItem>>,
    pub total_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WxAuthorizerItem {
    pub authorizer_appid: String,
}

// 用于解析微信授权方信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WxAuthorizerInfo {
    pub authorization_info: WxAuthorizationInfo,
    pub authorizer_info: WxAuthorizerBaseInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WxAuthorizationInfo {
    pub authorizer_appid: String,
    pub func_info: Vec<WxFuncInfo>,
    pub authorizer_refresh_token: String,
    pub authorizer_access_token: Option<String>,
    pub expires_in: Option<i64>,
    pub auth_time: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WxFuncInfo {
    pub confirm_info: Option<WxConfirmInfo>,
    pub funcscope_category: WxFuncscopeCategory,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WxConfirmInfo {
    pub already_confirm: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WxFuncscopeCategory {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WxAuthorizerBaseInfo {
    pub user_name: String,
    pub nick_name: String,
    #[serde(rename = "MiniProgramInfo")]
    pub mini_program_info: Option<serde_json::Value>,
    pub service_type_info: WxServiceTypeInfo,
    pub principal_name: String,
    pub register_type: Option<i32>,
    pub account_status: Option<i32>,
    pub basic_config: Option<i32>,
    pub verify_type_info: WxVerifyTypeInfo,
    pub qrcode_url: String,
    pub head_img: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WxServiceTypeInfo {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WxVerifyTypeInfo {
    pub id: i32,
}
