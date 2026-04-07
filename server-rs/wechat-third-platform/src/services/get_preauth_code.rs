use crate::error::Error;
use crate::service::WxService;
use crate::{WxResponse, WxStorage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreAuthCodeResponse {
    pub pre_auth_code: String,
    pub expires_in: u32,
}

impl<S: WxStorage> WxService<S> {
    /// 获取预授权码
    pub async fn get_preauth_code(&self) -> Result<PreAuthCodeResponse, Error> {
        let token = self.get_component_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/component/api_create_preauthcode?component_access_token={}",
            token
        );

        let res: WxResponse<PreAuthCodeResponse> = self
            .http_client
            .post(&url)
            .json(&serde_json::json!({ "component_appid": self.config.app_id }))
            .send()
            .await?
            .json()
            .await?;

        res.extract()
    }
}
