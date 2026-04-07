use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

// 微信回调组件查询参数
#[derive(Debug, Clone, Deserialize)]
pub struct WxCallbackQuery {
    pub timestamp: Option<String>,
    pub nonce: Option<String>,
    pub msg_signature: Option<String>,
    pub encrypt_type: Option<String>,
}

// 微信加密消息的 XML 结构（外层）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptedXml {
    #[serde(rename = "xml")]
    pub xml: EncryptedXmlContent,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptedXmlContent {
    #[serde(rename = "Encrypt")]
    pub encrypt: String,
}

// 解密后的 XML 结构 - 组件验证票据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ComponentXml {
    #[serde(rename = "xml")]
    pub xml: ComponentXmlContent,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ComponentXmlContent {
    #[serde(rename = "AppId")]
    pub app_id: String,
    #[serde(rename = "CreateTime")]
    pub create_time: i64,
    #[serde(rename = "InfoType")]
    pub info_type: String,
    #[serde(rename = "ComponentVerifyTicket")]
    pub component_verify_ticket: Option<String>,
}

// 用于存储组件回调记录的请求数据
#[derive(Debug, Clone)]
pub struct ComponentRecordData {
    pub receive_time: DateTime<Utc>,
    pub info_type: String,
    pub post_body: String,
}

// 从 roxmltree 解析的组件 XML 内容
#[derive(Debug, Clone)]
pub struct ParsedComponentXml {
    pub app_id: String,
    pub create_time: i64,
    pub info_type: String,
    pub component_verify_ticket: Option<String>,
    pub raw_xml: String,
}

impl ParsedComponentXml {
    // 将 CreateTime 转换为 DateTime
    pub fn receive_time(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.create_time, 0)
            .single()
            .unwrap_or_else(Utc::now)
    }
}
