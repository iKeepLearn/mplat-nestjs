use super::AppState;
use crate::{ApiResponse, middleware::rate_limit::RateLimit};
use actix_web::{HttpMessage, Responder, web};
use error::Error;
use service::AuthenticatedUser;
use structs::dto::admin::*;
use validator::Validate;

// 获取当前用户的辅助函数
fn get_username(req: &actix_web::HttpRequest) -> Result<String, Error> {
    let ext = req.extensions();
    let user = ext
        .get::<AuthenticatedUser>()
        .ok_or_else(|| Error::InvalidAuth)?;
    Ok(user.username.clone())
}

pub async fn change_password(
    state: web::Data<AppState>,
    query: web::Json<ChangePwdDto>,
    req: actix_web::HttpRequest,
) -> Result<impl Responder, Error> {
    query.validate()?;
    let username = get_username(&req)?;
    let response = state
        .service
        .admin
        .change_password(query.into_inner(), username)
        .await?;
    Ok(ApiResponse::success(response))
}

pub async fn change_username(
    state: web::Data<AppState>,
    query: web::Json<ChangeUsernameDto>,
    req: actix_web::HttpRequest,
) -> Result<impl Responder, Error> {
    let username = get_username(&req)?;
    let response = state
        .service
        .admin
        .change_username(query.username.clone(), username)
        .await?;
    Ok(ApiResponse::success(response))
}

pub async fn change_secret(
    state: web::Data<AppState>,
    query: web::Json<ChangeSecretDto>,
) -> Result<impl Responder, Error> {
    state.service.admin.change_secret(query.secret.clone()).await?;
    Ok(ApiResponse::success(()))
}

pub async fn get_secret(state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let response = state.service.admin.get_secret().await?;
    Ok(ApiResponse::success(response))
}

pub async fn get_authorizer_list(
    state: web::Data<AppState>,
    query: web::Query<AuthorizerListQuery>,
) -> Result<impl Responder, Error> {
    let response = state
        .service
        .admin
        .get_authorizer_list(query.into_inner())
        .await?;
    Ok(ApiResponse::success(response))
}

pub async fn add_component_info(
    state: web::Data<AppState>,
    query: web::Json<AddComponentInfoDto>,
) -> Result<impl Responder, Error> {
    state
        .service
        .admin
        .add_component_info(query.into_inner())
        .await?;
    Ok(ApiResponse::success(()))
}

pub async fn get_authorizer_access_token(
    state: web::Data<AppState>,
    query: web::Query<AppIdQuery>,
) -> Result<impl Responder, Error> {
    let response = state
        .service
        .admin
        .get_authorizer_access_token(query.appid.clone())
        .await?;
    Ok(ApiResponse::success(response))
}

pub async fn get_dev_weapp_list(
    state: web::Data<AppState>,
    query: web::Query<DevWeappListQuery>,
) -> Result<impl Responder, Error> {
    let response = state
        .service
        .admin
        .get_dev_weapp_list(query.into_inner())
        .await?;
    Ok(ApiResponse::success(response))
}

pub async fn get_component_verify_ticket(
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let response = state.service.admin.get_component_verify_ticket().await?;
    Ok(ApiResponse::success(response))
}

pub async fn get_component_access_token(
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let response = state.service.admin.get_component_access_token().await?;
    Ok(ApiResponse::success(response))
}

pub async fn get_wx_component_records(
    state: web::Data<AppState>,
    query: web::Query<WxRecordsQuery>,
) -> Result<impl Responder, Error> {
    let response = state
        .service
        .admin
        .get_wx_component_records(query.into_inner())
        .await?;
    Ok(ApiResponse::success(response))
}

pub async fn get_wx_biz_records(
    state: web::Data<AppState>,
    query: web::Query<WxRecordsQuery>,
) -> Result<impl Responder, Error> {
    let response = state
        .service
        .admin
        .get_wx_biz_records(query.into_inner())
        .await?;
    Ok(ApiResponse::success(response))
}

pub async fn get_proxy_config(state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let response = state.service.admin.get_proxy_config().await?;
    Ok(ApiResponse::success(response))
}

pub async fn update_proxy_config(
    state: web::Data<AppState>,
    query: web::Json<UpdateProxyConfigDto>,
) -> Result<impl Responder, Error> {
    state
        .service
        .admin
        .update_proxy_config(query.open, query.port.clone())
        .await?;
    Ok(ApiResponse::success(()))
}

pub async fn get_callback_proxy_rule_list(
    state: web::Data<AppState>,
    query: web::Query<CallbackProxyRuleListQuery>,
) -> Result<impl Responder, Error> {
    let response = state
        .service
        .admin
        .get_callback_proxy_rule_list(query.r#type.clone(), query.offset, query.limit)
        .await?;
    Ok(ApiResponse::success(response))
}

pub async fn add_callback_proxy_rule(
    state: web::Data<AppState>,
    query: web::Json<AddCallbackProxyRuleDto>,
) -> Result<impl Responder, Error> {
    state
        .service
        .admin
        .add_callback_proxy_rule(query.into_inner())
        .await?;
    Ok(ApiResponse::success(()))
}

pub async fn update_callback_proxy_rule(
    state: web::Data<AppState>,
    query: web::Json<UpdateCallbackProxyRuleDto>,
) -> Result<impl Responder, Error> {
    state
        .service
        .admin
        .update_callback_proxy_rule(query.id, query.data.clone())
        .await?;
    Ok(ApiResponse::success(()))
}

pub async fn delete_callback_proxy_rule(
    state: web::Data<AppState>,
    query: web::Query<DeleteCallbackProxyRuleQuery>,
) -> Result<impl Responder, Error> {
    state
        .service
        .admin
        .delete_callback_proxy_rule(query.id)
        .await?;
    Ok(ApiResponse::success(()))
}

pub fn configure_admin_routes(cfg: &mut web::ServiceConfig, _rate_limit: &RateLimit) {
    cfg.service(
        web::scope("/api/admin")
            .route("/userpwd", web::post().to(change_password))
            .route("/username", web::post().to(change_username))
            .route("/secret", web::post().to(change_secret))
            .route("/secret", web::get().to(get_secret))
            .route("/authorizer-list", web::get().to(get_authorizer_list))
            .route("/componentinfo", web::post().to(add_component_info))
            .route(
                "/authorizer-access-token",
                web::get().to(get_authorizer_access_token),
            )
            .route("/dev-weapp-list", web::get().to(get_dev_weapp_list))
            .route("/ticket", web::get().to(get_component_verify_ticket))
            .route(
                "/component-access-token",
                web::get().to(get_component_access_token),
            )
            .route(
                "/wx-component-records",
                web::get().to(get_wx_component_records),
            )
            .route("/wx-biz-records", web::get().to(get_wx_biz_records))
            .route("/proxy", web::get().to(get_proxy_config))
            .route("/proxy", web::post().to(update_proxy_config))
            .route(
                "/callback-proxy-rule-list",
                web::get().to(get_callback_proxy_rule_list),
            )
            .route(
                "/callback-proxy-rule",
                web::put().to(add_callback_proxy_rule),
            )
            .route(
                "/callback-proxy-rule",
                web::post().to(update_callback_proxy_rule),
            )
            .route(
                "/callback-proxy-rule",
                web::delete().to(delete_callback_proxy_rule),
            ),
    );
}
