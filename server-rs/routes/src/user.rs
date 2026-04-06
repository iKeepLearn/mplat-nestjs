use super::AppState;
use crate::{ApiResponse, middleware::rate_limit::RateLimit};
use actix_web::{Responder, web};
use error::Error;
use structs::dto::user::AuthDto;
use validator::Validate;

pub async fn signin(
    state: web::Data<AppState>,
    query: web::Json<AuthDto>,
) -> Result<impl Responder, Error> {
    query.validate()?;
    let response = state
        .service
        .user
        .signin(query.into_inner(), &state.jwt_token)
        .await?;
    Ok(ApiResponse::success(response))
}

pub async fn signup(
    state: web::Data<AppState>,
    query: web::Json<AuthDto>,
) -> Result<impl Responder, Error> {
    query.validate()?;
    let response = state.service.user.signup(query.into_inner()).await?;
    Ok(web::Json(response))
}

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig, rate_limit: &RateLimit) {
    cfg.service(
        web::scope("/api")
            .wrap(rate_limit.login())
            .route("/auth", web::put().to(signin))
            .route("/signup", web::post().to(signup)),
    );
}
