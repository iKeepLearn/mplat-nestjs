use super::AppState;
use crate::{ApiResponse, middleware::rate_limit::RateLimit};
use actix_web::{Responder, web};
use error::Error;

pub async fn get_component_info(state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let response = state.service.auth_page.get_component_info().await?;
    Ok(ApiResponse::success(response))
}

pub async fn get_preauth_code(state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let response = state
        .service
        .auth_page
        .get_preauth_code(&state.wx_service)
        .await?;
    Ok(ApiResponse::success(response))
}

pub fn configure_auth_page_routes(cfg: &mut web::ServiceConfig, _rate_limit: &RateLimit) {
    cfg.service(
        web::scope("/api/authpage")
            .route("/componentinfo", web::post().to(get_component_info))
            .route("/preauthcode", web::post().to(get_preauth_code)),
    );
}
