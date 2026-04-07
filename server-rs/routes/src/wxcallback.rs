use super::AppState;
use actix_web::{Responder, web};
use error::Error;
use structs::dto::wxcallback::WxCallbackQuery;
use tracing::info;

/// 处理微信第三方平台组件回调
pub async fn component_callback(
    state: web::Data<AppState>,
    query: web::Query<WxCallbackQuery>,
    body: String,
) -> Result<impl Responder, Error> {
    info!("Received component callback");

    let response = state
        .service
        .wxcallback
        .handle_component_callback(&body, query.into_inner(), &state.wx_service)
        .await?;

    Ok(response)
}

/// 处理授权方业务消息回调
pub async fn biz_callback(
    state: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<WxCallbackQuery>,
    body: String,
) -> Result<impl Responder, Error> {
    let appid = path.into_inner();
    info!("Received biz callback for appid: {}", appid);

    let response = state
        .service
        .wxcallback
        .handle_biz_callback(&body, query.into_inner(), &appid)
        .await?;

    Ok(response)
}

pub fn configure_wxcallback_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/wxcallback")
            .route("/component", web::post().to(component_callback))
            .route("/biz/{appid}", web::post().to(biz_callback)),
    );
}
