use chrono::{TimeZone, Utc};
use database::Repository;
use error::Error;
use structs::dto::wxcallback::*;
use tracing::{error, info};
use wechat_third_platform::WxStorage;
use wechat_third_platform::service::WxService;
use wechat_third_platform::xml_parser::{parse_component_xml, parse_encrypted_xml};

#[derive(Debug, Clone)]
pub struct WxCallbackService {
    repository: Repository,
}

impl WxCallbackService {
    pub fn new(repository: Repository) -> Self {
        WxCallbackService { repository }
    }
}

impl WxCallbackService {
    /// 处理组件验证票据回调
    pub async fn handle_component_callback(
        &self,
        body: &str,
        query: WxCallbackQuery,
        wx_service: &WxService<impl WxStorage>,
    ) -> Result<String, Error> {
        info!("Received component callback");

        // 1. 解析外层 XML 获取 Encrypt
        let encrypt = parse_encrypted_xml(body)?;
        info!("Parsed encrypted XML");

        // 2. 验证签名
        if let (Some(timestamp), Some(nonce), Some(msg_signature)) =
            (&query.timestamp, &query.nonce, &query.msg_signature)
        {
            let signature = wx_service.gen_sign(timestamp, nonce, &encrypt);
            if signature != *msg_signature {
                error!("Signature mismatch");
                return Err(Error::bad_request("签名验证失败"));
            }
            info!("Signature verified");
        } else {
            error!("Missing query parameters for signature verification");
            return Err(Error::bad_request("缺少必要的查询参数"));
        }

        // 3. 解密消息
        let decrypted_xml = wx_service.decode(&encrypt)?;
        info!("Decrypted XML: {}", decrypted_xml);

        // 4. 解析解密后的 XML
        let parsed = parse_component_xml(&decrypted_xml)?;
        info!("Parsed component XML: info_type={}", parsed.info_type);

        // 5. 保存回调记录
        let post_body = serde_json::json!({
            "AppId": parsed.app_id,
            "CreateTime": parsed.create_time,
            "InfoType": parsed.info_type,
            "ComponentVerifyTicket": parsed.component_verify_ticket,
        })
        .to_string();

        let receive_time = Utc
            .timestamp_opt(parsed.create_time, 0)
            .single()
            .unwrap_or_else(Utc::now);
        self.repository
            .wxcallback
            .create_component_record(receive_time, &parsed.info_type, &post_body)
            .await?;

        // 6. 如果是 component_verify_ticket，则更新 ticket
        if parsed.info_type == "component_verify_ticket"
            && let Some(ticket) = parsed.component_verify_ticket
        {
            info!("Updating component verify ticket");
            self.repository.wxcallback.upsert_ticket(&ticket).await?;
        }

        // 返回 success
        Ok("success".to_string())
    }

    /// 处理业务回调（授权方消息）
    pub async fn handle_biz_callback(
        &self,
        _body: &str,
        _query: WxCallbackQuery,
        _appid: &str,
    ) -> Result<String, Error> {
        // TODO: 实现业务回调处理
        info!("Received biz callback for appid: {}", _appid);
        Ok("success".to_string())
    }
}
