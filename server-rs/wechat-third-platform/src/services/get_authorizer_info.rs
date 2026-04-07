use crate::error::Error;
use crate::service::WxService;
use crate::{WxResponse, WxStorage};
use serde::{Deserialize, Serialize};

impl<S: WxStorage> WxService<S> {
    /// 获取授权账号详情
    pub async fn get_authorizer_info(
        &self,
        target_id: &str,
    ) -> Result<GetAuthorizerInfoResponse, Error> {
        let token = self.get_component_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/component/api_get_authorizer_info?access_token={}",
            token
        );

        let res: WxResponse<GetAuthorizerInfoResponse> = self
            .http_client
            .post(&url)
            .json(&serde_json::json!({
                "component_appid": self.config.app_id,
                "authorizer_appid": target_id
            }))
            .send()
            .await?
            .json()
            .await?;

        res.extract()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthorizerInfoResponse {
    pub authorizer_info: AuthorizerInfo,
    pub authorization_info: AuthorizationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizerInfo {
    pub nick_name: String,
    pub head_img: String,
    pub service_type_info: ServiceTypeInfo,
    pub verify_type_info: VerifyTypeInfo,
    pub user_name: String,
    pub alias: Option<String>,
    pub qrcode_url: String,
    pub business_info: BusinessInfo,
    pub idc: Option<u32>,
    pub principal_name: String,
    pub signature: String,
    #[serde(rename = "MiniProgramInfo")]
    pub mini_program_info: Option<MiniProgramInfo>,
    pub register_type: Option<u32>,
    pub account_status: Option<u32>,
    pub basic_config: Option<BasicConfig>,
    pub channels_info: Option<ChannelsInfo>,
    pub store_info: Option<StoreInfo>,
    pub talent_info: Option<TalentInfo>,
    pub supplier_info: Option<SupplierInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationInfo {
    pub authorizer_appid: String,
    pub authorizer_refresh_token: String,
    pub func_info: Vec<FuncInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceTypeInfo {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyTypeInfo {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessInfo {
    pub open_pay: u32,
    pub open_shake: u32,
    pub open_scan: u32,
    pub open_card: u32,
    pub open_store: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramInfo {
    pub network: Network,
    pub categories: Vec<Category>,
    pub visit_status: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    #[serde(rename = "RequestDomain")]
    pub request_domain: Vec<String>,
    #[serde(rename = "WsRequestDomain")]
    pub ws_request_domain: Vec<String>,
    #[serde(rename = "UploadDomain")]
    pub upload_domain: Vec<String>,
    #[serde(rename = "DownloadDomain")]
    pub download_domain: Vec<String>,
    #[serde(rename = "UDPDomain")]
    pub udp_domain: Vec<String>,
    #[serde(rename = "TCPDomain")]
    pub tcp_domain: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub first: String,
    pub second: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicConfig {
    pub is_phone_configured: bool,
    pub is_email_configured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelsInfo {
    pub id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreInfo {
    pub id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentInfo {
    pub id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierInfo {
    pub id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuncInfo {
    pub funcscope_category: FuncscopeCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuncscopeCategory {
    pub id: u32,
}
