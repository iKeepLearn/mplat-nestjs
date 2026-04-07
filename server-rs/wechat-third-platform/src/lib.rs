mod services;

pub mod error;
pub mod service;
pub mod util;

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STD};
use error::Error;
use serde::{Deserialize, Serialize};
use tracing::error;

/// 微信第三方服务配置
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub app_id: String,
    pub token: String,
    pub secret: String,
    pub key: Vec<u8>,
    pub iv: Vec<u8>,
}

impl Config {
    pub fn new(
        app_id: &str,
        token: &str,
        secret: &str,
        encoding_aes_key: &str,
    ) -> Result<Self, Error> {
        let key_base64 = format!("{}=", encoding_aes_key);
        let key = BASE64_STD.decode(key_base64)?;
        if key.len() < 16 {
            return Err(Error::Config("Invalid encodingAESKey length".to_string()));
        }
        let iv = key[0..16].to_vec();

        Ok(Self {
            app_id: app_id.to_string(),
            token: token.to_string(),
            secret: secret.to_string(),
            key,
            iv,
        })
    }
}

#[async_trait::async_trait]
pub trait WxStorage: Send + Sync {
    /// 获取全局票据 ticket
    async fn get_ticket(&self) -> Result<String, Error>;
    /// 获取 component_access_token 及其过期时间戳(秒)
    async fn get_component_token(&self, appid: &str) -> Result<Option<(String, u64)>, Error>;
    /// 保存 component_access_token
    async fn save_component_token(
        &self,
        appid: &str,
        token: &str,
        expire_time: u64,
    ) -> Result<(), Error>;
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum WxResponse<T> {
    Success {
        #[serde(flatten)]
        data: T,
    },
    Error {
        errcode: i64,
        errmsg: String,
    },
}

impl<T> WxResponse<T> {
    /// 获取微信第三方平台返回的数据
    pub(crate) fn extract(self) -> Result<T, Error> {
        match self {
            Self::Success { data } => Ok(data),
            Self::Error { errcode, errmsg } => {
                error!(
                    "微信第三方平台返回错误: code={}, message={}",
                    errcode, errmsg
                );

                Err(Error::Network(errmsg))
            }
        }
    }
}
