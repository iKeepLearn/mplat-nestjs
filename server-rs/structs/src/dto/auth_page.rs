use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetComponentInfoResponse {
    pub appid: String,
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetPreauthCodeResponse {
    #[serde(rename = "preAuthCode")]
    pub pre_auth_code: String,
}
