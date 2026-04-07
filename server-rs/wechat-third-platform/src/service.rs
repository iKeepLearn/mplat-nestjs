use crate::error::Error;
use crate::{Config, WxResponse, WxStorage, util};
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit, block_padding::NoPadding};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STD};
use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

pub type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
pub type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

/// 微信服务核心类
#[derive(Debug, Clone)]
pub struct WxService<S: WxStorage> {
    pub config: Config,
    pub storage: S,
    pub http_client: Client,
    pub refresh_lock: Arc<Mutex<()>>,
}

impl<S: WxStorage> WxService<S> {
    pub fn new(config: Config, storage: S) -> Self {
        Self {
            config,
            storage,
            http_client: Client::new(),
            refresh_lock: Arc::new(Mutex::new(())),
        }
    }

    /// 加密消息
    pub fn encode(&self, msg: &str) -> Result<String, Error> {
        let mut random_bytes = [0u8; 16];
        rand::rng().fill_bytes(&mut random_bytes);

        let msg_bytes = msg.as_bytes();
        let msg_len = (msg_bytes.len() as u32).to_be_bytes();
        let app_id_bytes = self.config.app_id.as_bytes();

        let mut total_buf = Vec::new();
        total_buf.extend_from_slice(&random_bytes);
        total_buf.extend_from_slice(&msg_len);
        total_buf.extend_from_slice(msg_bytes);
        total_buf.extend_from_slice(app_id_bytes);

        // 自定义 PKCS#7 填充 (按 32 字节块)
        let total_buf = util::pkcs7_encode(total_buf);

        // AES-256-CBC 加密
        let mut cipher_buf = total_buf;
        let pt_len = cipher_buf.len();

        let encryptor = Aes256CbcEnc::new_from_slices(&self.config.key, &self.config.iv)
            .map_err(|e| Error::Config(format!("Cipher init error: {:?}", e)))?;

        let encrypted_data = encryptor
            .encrypt_padded_mut::<NoPadding>(&mut cipher_buf, pt_len)
            .map_err(|e| Error::Config(format!("Encryption error: {:?}", e)))?;

        Ok(BASE64_STD.encode(encrypted_data))
    }

    /// 解密消息
    pub fn decode(&self, encrypted_msg: &str) -> Result<String, Error> {
        let encrypted_bytes = BASE64_STD.decode(encrypted_msg)?;
        let mut dec_buf = encrypted_bytes;

        let decryptor = Aes256CbcDec::new_from_slices(&self.config.key, &self.config.iv)
            .map_err(|e| Error::Config(format!("Cipher init error: {:?}", e)))?;

        let decrypted_data = decryptor
            .decrypt_padded_mut::<NoPadding>(&mut dec_buf)
            .map_err(|e| Error::Config(format!("Decryption error: {:?}", e)))?;

        // 移除 PKCS#7 填充
        let unpadded = util::pkcs7_decode(decrypted_data)?;

        // 读取消息长度和内容
        if unpadded.len() < 20 {
            return Err(Error::Config("Invalid decrypted data length".to_string()));
        }

        let mut len_bytes = [0u8; 4];
        len_bytes.copy_from_slice(&unpadded[16..20]);
        let msg_size = u32::from_be_bytes(len_bytes) as usize;

        let msg_start = 20;
        let msg_end = msg_start + msg_size;

        if unpadded.len() < msg_end {
            return Err(Error::Config("Message length out of bounds".to_string()));
        }

        let msg_buf = &unpadded[msg_start..msg_end];
        let app_id_buf = &unpadded[msg_end..];
        let decoded_app_id = String::from_utf8(app_id_buf.to_vec())
            .map_err(|_| Error::Config("Invalid utf8 in app_id".to_string()))?;

        if decoded_app_id != self.config.app_id {
            return Err(Error::Config(
                "AppID mismatch in decrypted message".to_string(),
            ));
        }
        Ok(String::from_utf8(msg_buf.to_vec())?)
    }

    /// 生成签名
    pub fn gen_sign(&self, timestamp: &str, nonce: &str, encrypt: &str) -> String {
        let mut params = [self.config.token.as_str(), timestamp, nonce, encrypt];
        params.sort();
        let raw_str = params.join("");

        let mut hasher = Sha1::new();
        hasher.update(raw_str.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub async fn refresh_component_token(&self) -> Result<(String, u64), Error> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let ticket = self.storage.get_ticket().await?;

        let req_body = serde_json::json!({
            "component_appid": self.config.app_id,
            "component_appsecret": self.config.secret,
            "component_verify_ticket": ticket,
        });

        let res: WxResponse<ComponentTokenResponse> = self
            .http_client
            .post("https://api.weixin.qq.com/cgi-bin/component/api_component_token")
            .json(&req_body)
            .send()
            .await?
            .json()
            .await?;

        let token = res.extract()?;

        let expire_time = now + token.expires_in;

        // 4. 保存新 Token 到存储
        self.storage
            .save_component_token(
                &self.config.app_id,
                &token.component_access_token,
                expire_time,
            )
            .await?;
        Ok((token.component_access_token, expire_time))
    }

    /// 获取或刷新 component_access_token
    pub async fn get_component_token(&self) -> Result<String, Error> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let safe_now = now + 20; // 提前 20 秒认为过期，留出容错窗口

        // 1. 从存储中检查是否有未过期的 Token
        if let Some((token, expire_time)) = self
            .storage
            .get_component_token(&self.config.app_id)
            .await?
            && expire_time > safe_now
        {
            return Ok(token);
        }

        let _lock = self.refresh_lock.lock().await;

        // 刷新完成后重新从数据库读取最新的 Token
        if let Some((token, expire_time)) = self
            .storage
            .get_component_token(&self.config.app_id)
            .await?
            && expire_time > safe_now
        {
            return Ok(token);
        }

        let (token, _) = self.refresh_component_token().await?;
        Ok(token)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ComponentTokenResponse {
    component_access_token: String,
    expires_in: u64,
}
