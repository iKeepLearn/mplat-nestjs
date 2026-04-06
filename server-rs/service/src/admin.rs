use chrono::{TimeZone, Utc};
use database::Repository;
use error::Error;
use serde_json;
use structs::dto::admin::*;
use tracing::error;
use utils::date_time::format_datetime;

const BCRYPT_SALT_LENGTH: u32 = 10;

#[derive(Debug, Clone)]
pub struct AdminService {
    repository: Repository,
}

impl AdminService {
    pub fn new(repository: Repository) -> Self {
        AdminService { repository }
    }
}

impl AdminService {
    async fn hash_password(&self, password: &str) -> Result<String, Error> {
        bcrypt::hash(password, BCRYPT_SALT_LENGTH).map_err(|e| {
            error!("hash_password error: {:?}", e);
            Error::internal("密码加密失败")
        })
    }

    async fn verify_password(&self, password: &str, hash: &str) -> Result<bool, Error> {
        bcrypt::verify(password, hash).map_err(|e| {
            error!("verify_password error: {:?}", e);
            Error::internal("密码验证失败")
        })
    }

    pub async fn change_password(
        &self,
        detail: ChangePwdDto,
        username: String,
    ) -> Result<String, Error> {
        // 获取当前用户的旧密码进行验证
        let user = self
            .repository
            .user
            .find_by_username_with_password_only(&username)
            .await?;

        let Some(password_hash) = user else {
            return Err(Error::bad_request("用户不存在"));
        };

        let password_is_match = self.verify_password(&detail.old_password, &password_hash).await?;
        if !password_is_match {
            return Err(Error::bad_request("旧密码不正确"));
        }

        let encrypt_password = self.hash_password(&detail.password).await?;
        self.repository
            .admin
            .update_password(&username, &encrypt_password)
            .await?;

        Ok(username)
    }

    pub async fn change_username(
        &self,
        new_username: String,
        username: String,
    ) -> Result<String, Error> {
        match self
            .repository
            .admin
            .update_username(&username, &new_username)
            .await
        {
            Ok(_) => Ok(username),
            Err(Error::BadRequest(msg)) => Err(Error::BadRequest(msg)),
            Err(e) => Err(e),
        }
    }

    pub async fn change_secret(&self, secret: String) -> Result<(), Error> {
        self.repository.admin.upsert_secret(&secret).await?;
        Ok(())
    }

    pub async fn get_secret(&self) -> Result<SecretResponse, Error> {
        let result = self.repository.admin.get_secret().await?;
        let secret = result.ok_or_else(|| Error::not_found("secret not found"))?;
        Ok(SecretResponse { secret })
    }

    pub async fn add_component_info(&self, add_info: AddComponentInfoDto) -> Result<(), Error> {
        self.repository
            .admin
            .upsert_redirect_uri(&add_info.redirect_url)
            .await?;
        Ok(())
    }

    // 解析授权方信息
    fn parse_authorizer_info(&self, app_info: WxAuthorizerInfo) -> AuthorizerInfo {
        let func_info = app_info
            .authorization_info
            .func_info
            .iter()
            .filter_map(|item| {
                if let Some(confirm_info) = &item.confirm_info {
                    if confirm_info.already_confirm.unwrap_or(0) > 0 {
                        Some(item.funcscope_category.id)
                    } else {
                        Some(0)
                    }
                } else {
                    Some(item.funcscope_category.id)
                }
            })
            .filter(|&id| id > 0)
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        AuthorizerInfo {
            id: app_info.authorization_info.authorizer_appid.clone(),
            appid: app_info.authorization_info.authorizer_appid,
            user_name: app_info.authorizer_info.user_name,
            nick_name: app_info.authorizer_info.nick_name,
            app_type: if app_info.authorizer_info.mini_program_info.is_some() {
                0
            } else {
                1
            },
            service_type: app_info.authorizer_info.service_type_info.id,
            auth_time: app_info.authorization_info.auth_time,
            principal_name: app_info.authorizer_info.principal_name,
            register_type: app_info.authorizer_info.register_type.unwrap_or(0),
            account_status: app_info.authorizer_info.account_status.unwrap_or(0),
            basic_config: app_info.authorizer_info.basic_config,
            verify_info: app_info.authorizer_info.verify_type_info.id,
            refresh_token: app_info.authorization_info.authorizer_refresh_token,
            qrcode_url: app_info.authorizer_info.qrcode_url,
            head_img: app_info.authorizer_info.head_img,
            func_info,
            access_token: app_info.authorization_info.authorizer_access_token,
            expires_in: app_info.authorization_info.expires_in,
        }
    }

    pub async fn get_authorizer_list(
        &self,
        params: AuthorizerListQuery,
    ) -> Result<AuthorizerListResponse, Error> {
        // TODO: 这里需要调用微信 API 获取授权方列表
        // 暂时返回空结果
        let _offset = params.offset.unwrap_or(0);
        let _limit = params.limit.unwrap_or(15);

        // 模拟返回空列表
        Ok(AuthorizerListResponse {
            records: vec![],
            total: 0,
        })
    }

    pub async fn get_authorizer_access_token(&self, appid: String) -> Result<String, Error> {
        let result = self
            .repository
            .admin
            .get_authorizer_access_token(&appid)
            .await?;
        result.ok_or_else(|| Error::bad_request("该appid没有返回有效token"))
    }

    pub async fn get_dev_weapp_list(
        &self,
        params: DevWeappListQuery,
    ) -> Result<DevWeappListResponse, Error> {
        let offset = params.offset.unwrap_or(0);
        let limit = params.limit.unwrap_or(15);

        let result = self
            .repository
            .admin
            .get_dev_weapp_list(offset, limit)
            .await?;

        Ok(DevWeappListResponse {
            total: result.len(),
            records: result,
        })
    }

    pub async fn get_component_verify_ticket(&self) -> Result<TicketResponse, Error> {
        let result = self.repository.admin.get_component_verify_ticket().await?;
        let ticket = result.ok_or_else(|| Error::not_found("ticket not found"))?;
        Ok(TicketResponse { ticket })
    }

    pub async fn get_component_access_token(&self) -> Result<ComponentAccessTokenResponse, Error> {
        let result = self.repository.admin.get_component_access_token().await?;
        let token = result.ok_or_else(|| Error::not_found("token not found"))?;
        Ok(ComponentAccessTokenResponse { token })
    }

    pub async fn get_wx_component_records(
        &self,
        params: WxRecordsQuery,
    ) -> Result<WxRecordsResponse<WxComponentRecord>, Error> {
        let limit = params.limit.unwrap_or(15);
        let offset = params.offset.unwrap_or(0);
        let start_time = params.start_time.map(|t| {
            Utc.timestamp_opt(t, 0)
                .single()
                .unwrap_or_else(|| Utc.with_ymd_and_hms(2022, 11, 24, 0, 0, 0).unwrap())
        });
        let end_time = params.end_time.map(|t| {
            Utc.timestamp_opt(t, 0)
                .single()
                .unwrap_or_else(Utc::now)
        });

        let result = self
            .repository
            .admin
            .get_wx_component_records(limit, offset, start_time, end_time, params.info_type)
            .await?;

        let records: Vec<_> = result
            .iter()
            .map(|item| WxComponentRecord {
                receive_time: format_datetime(&item.receive_time, true),
                info_type: item.info_type.clone(),
                post_body: item.post_body.clone(),
            })
            .collect();

        Ok(WxRecordsResponse {
            total: records.len(),
            records,
        })
    }

    pub async fn get_wx_biz_records(
        &self,
        params: WxRecordsQuery,
    ) -> Result<WxRecordsResponse<WxBizRecord>, Error> {
        let limit = params.limit.unwrap_or(15);
        let offset = params.offset.unwrap_or(0);
        let start_time = params.start_time.map(|t| {
            Utc.timestamp_opt(t, 0)
                .single()
                .unwrap_or_else(|| Utc.with_ymd_and_hms(2022, 11, 24, 0, 0, 0).unwrap())
        });
        let end_time = params.end_time.map(|t| {
            Utc.timestamp_opt(t, 0)
                .single()
                .unwrap_or_else(Utc::now)
        });

        let result = self
            .repository
            .admin
            .get_wx_biz_records(
                limit,
                offset,
                start_time,
                end_time,
                params.appid,
                params.event,
                params.msg_type,
            )
            .await?;

        let records: Vec<_> = result
            .iter()
            .map(|item| WxBizRecord {
                receive_time: format_datetime(&item.receive_time, true),
                event: item.event.clone(),
                msg_type: item.msg_type.clone(),
                appid: item.appid.clone(),
                info_type: item.info_type.clone(),
                post_body: item.post_body.clone(),
            })
            .collect();

        Ok(WxRecordsResponse {
            total: records.len(),
            records,
        })
    }

    pub async fn get_proxy_config(&self) -> Result<ProxyConfigResponse, Error> {
        let result = self.repository.admin.get_proxy_config().await?;
        let open = result
            .iter()
            .find(|item| item.key == "proxy_state")
            .map(|item| item.value == "open")
            .unwrap_or(false);
        let port = result
            .iter()
            .find(|item| item.key == "proxy_port")
            .map(|item| item.value.clone())
            .unwrap_or_else(|| "8082".to_string());

        Ok(ProxyConfigResponse { open, port })
    }

    pub async fn update_proxy_config(&self, open: bool, port: String) -> Result<(), Error> {
        let open_str = if open { "open" } else { "close" };
        self.repository
            .admin
            .update_proxy_config(open_str, &port)
            .await?;
        Ok(())
    }

    pub async fn get_callback_proxy_rule_list(
        &self,
        r#type: Option<String>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<CallbackProxyRuleListResponse, Error> {
        let type_int = r#type
            .and_then(|t| t.parse::<i32>().ok())
            .unwrap_or(0);
        let offset = offset.unwrap_or(0);
        let limit = limit.unwrap_or(15);

        let result = self
            .repository
            .admin
            .get_callback_proxy_rule_list(type_int, offset, limit)
            .await?;

        let rules = result
            .iter()
            .filter_map(|item| {
                let data = match serde_json::from_str(&item.post_body) {
                    Ok(d) => d,
                    Err(_) => return None,
                };
                Some(CallbackProxyRule {
                    name: item.name.clone(),
                    msg_type: item.msg_type.clone(),
                    event: item.event.clone(),
                    open: item.open,
                    update_time: format_datetime(&item.update_time, true),
                    info_type: item.info_type.clone(),
                    r#type: item.r#type,
                    data,
                    id: item.id,
                })
            })
            .collect();

        Ok(CallbackProxyRuleListResponse { rules })
    }

    pub async fn add_callback_proxy_rule(
        &self,
        params: AddCallbackProxyRuleDto,
    ) -> Result<(), Error> {
        let event = params.event.unwrap_or_default();
        let msg_type = params.msg_type.unwrap_or_default();
        let info = params.info.unwrap_or_default();
        let post_body = serde_json::to_string(&params.data).map_err(|e| {
            error!("serialize post_body error: {:?}", e);
            Error::internal("序列化数据失败")
        })?;

        self.repository
            .admin
            .add_callback_proxy_rule(
                &params.name,
                params.r#type,
                &event,
                &msg_type,
                &info,
                &info,
                params.open,
                &post_body,
            )
            .await?;

        Ok(())
    }

    pub async fn update_callback_proxy_rule(
        &self,
        id: i32,
        data: serde_json::Value,
    ) -> Result<(), Error> {
        self.repository
            .admin
            .update_callback_proxy_rule(id, data)
            .await?;
        Ok(())
    }

    pub async fn delete_callback_proxy_rule(&self, id: i32) -> Result<(), Error> {
        self.repository.admin.delete_callback_proxy_rule(id).await?;
        Ok(())
    }
}
