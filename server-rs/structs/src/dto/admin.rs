use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::sqlx::Authorizer;

#[derive(Deserialize, Validate)]
pub struct ChangePwdDto {
    #[validate(length(min = 1, message = "请确认旧密码是否正确"))]
    pub old_password: String,
    #[validate(length(min = 1, message = "请确认密码是否正确"))]
    pub password: String,
}

#[derive(Deserialize)]
pub struct ChangeUsernameDto {
    pub username: String,
}

#[derive(Deserialize)]
pub struct ChangeSecretDto {
    pub secret: String,
}

#[derive(Serialize)]
pub struct SecretResponse {
    pub secret: String,
}

#[derive(Deserialize)]
pub struct AddComponentInfoDto {
    pub redirect_url: String,
}

#[derive(Deserialize)]
pub struct AuthorizerListQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct AuthorizerListResponse {
    pub records: Vec<AuthorizerInfo>,
    pub total: i64,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Deserialize)]
pub struct AppIdQuery {
    pub appid: String,
}

#[derive(Deserialize)]
pub struct DevWeappListQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct DevWeappListResponse {
    pub records: Vec<Authorizer>,
    pub total: usize,
}

#[derive(Serialize)]
pub struct TicketResponse {
    pub ticket: String,
}

#[derive(Serialize)]
pub struct ComponentAccessTokenResponse {
    pub token: String,
}

#[derive(Deserialize)]
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

#[derive(Serialize)]
pub struct WxComponentRecord {
    pub receive_time: String,
    pub info_type: String,
    pub post_body: String,
}

#[derive(Serialize)]
pub struct WxRecordsResponse<T> {
    pub records: Vec<T>,
    pub total: usize,
}

#[derive(Serialize)]
pub struct WxBizRecord {
    pub receive_time: String,
    pub event: String,
    pub msg_type: String,
    pub appid: String,
    pub info_type: String,
    pub post_body: String,
}

#[derive(Serialize)]
pub struct ProxyConfigResponse {
    pub open: bool,
    pub port: String,
}

#[derive(Deserialize)]
pub struct UpdateProxyConfigDto {
    pub open: bool,
    pub port: String,
}

#[derive(Deserialize)]
pub struct CallbackProxyRuleListQuery {
    pub r#type: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct CallbackProxyRuleListResponse {
    pub rules: Vec<CallbackProxyRule>,
}

#[derive(Deserialize)]
pub struct AddCallbackProxyRuleDto {
    pub name: String,
    pub r#type: i32,
    pub event: Option<String>,
    pub msg_type: Option<String>,
    pub info: Option<String>,
    pub open: i32,
    pub data: serde_json::Value,
}

#[derive(Deserialize)]
pub struct UpdateCallbackProxyRuleDto {
    pub id: i32,
    pub data: serde_json::Value,
}

#[derive(Deserialize)]
pub struct DeleteCallbackProxyRuleQuery {
    pub id: i32,
}

// 用于从微信 API 获取授权方列表的临时结构
#[derive(Deserialize)]
pub struct WxAuthorizerList {
    pub list: Option<Vec<WxAuthorizerItem>>,
    pub total_count: Option<i64>,
}

#[derive(Deserialize)]
pub struct WxAuthorizerItem {
    pub authorizer_appid: String,
}

// 用于解析微信授权方信息
#[derive(Deserialize)]
pub struct WxAuthorizerInfo {
    pub authorization_info: WxAuthorizationInfo,
    pub authorizer_info: WxAuthorizerBaseInfo,
}

#[derive(Deserialize)]
pub struct WxAuthorizationInfo {
    pub authorizer_appid: String,
    pub func_info: Vec<WxFuncInfo>,
    pub authorizer_refresh_token: String,
    pub authorizer_access_token: Option<String>,
    pub expires_in: Option<i64>,
    pub auth_time: Option<i64>,
}

#[derive(Deserialize)]
pub struct WxFuncInfo {
    pub confirm_info: Option<WxConfirmInfo>,
    pub funcscope_category: WxFuncscopeCategory,
}

#[derive(Deserialize)]
pub struct WxConfirmInfo {
    pub already_confirm: Option<i32>,
}

#[derive(Deserialize)]
pub struct WxFuncscopeCategory {
    pub id: i32,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct WxServiceTypeInfo {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct WxVerifyTypeInfo {
    pub id: i32,
}
