use crate::error::Error;
use crate::service::WxService;
use crate::{WxResponse, WxStorage};
use serde::{Deserialize, Serialize};

/// 获取授权账号列表的响应体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizerListResponse {
    /// 授权的账号总数
    pub total_count: u32,
    /// 当前查询的帐号基本信息列表
    pub list: Vec<AuthorizerInfo>,
}

/// 授权账号基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizerInfo {
    /// 已授权账号的 appid
    pub authorizer_appid: String,
    /// 刷新令牌 authorizer_refresh_token
    pub refresh_token: String,
    /// 授权的时间（时间戳，单位：秒）
    pub auth_time: u64,
}

impl<S: WxStorage> WxService<S> {
    /// 获取授权账号列表
    pub async fn get_authorizer_list(
        &self,
        count: u32,
        offset: u32,
    ) -> Result<AuthorizerListResponse, Error> {
        let token = self.get_component_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/component/api_get_authorizer_list?access_token={}",
            token
        );

        let res: WxResponse<AuthorizerListResponse> = self
            .http_client
            .post(&url)
            .json(&serde_json::json!({
                "component_appid": self.config.app_id,
                "count": count,
                "offset": offset
            }))
            .send()
            .await?
            .json()
            .await?;

        res.extract()
    }
}
