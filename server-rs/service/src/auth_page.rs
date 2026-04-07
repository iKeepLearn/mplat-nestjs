use database::Repository;
use error::Error;
use structs::dto::auth_page::{GetComponentInfoResponse, GetPreauthCodeResponse};
use tokio::try_join;
use wechat_third_platform::WxStorage;
use wechat_third_platform::service::WxService;
#[derive(Debug, Clone)]
pub struct AuthPageService {
    repository: Repository,
}

impl AuthPageService {
    pub fn new(repository: Repository) -> Self {
        AuthPageService { repository }
    }
}

impl AuthPageService {
    pub async fn get_component_info(&self) -> Result<GetComponentInfoResponse, Error> {
        let (appid, redirect_uri) = try_join!(
            self.repository.comm_kv.get_comm_kv("appid"),
            self.repository.comm_kv.get_comm_kv("redirect_uri"),
        )?;

        Ok(GetComponentInfoResponse {
            appid: appid.value,
            redirect_url: redirect_uri.value,
        })
    }

    /// 处理业务回调（授权方消息）
    pub async fn get_preauth_code(
        &self,
        wx_service: &WxService<impl WxStorage>,
    ) -> Result<GetPreauthCodeResponse, Error> {
        let code = wx_service.get_preauth_code().await?;
        Ok(GetPreauthCodeResponse {
            pre_auth_code: code.pre_auth_code,
        })
    }
}
