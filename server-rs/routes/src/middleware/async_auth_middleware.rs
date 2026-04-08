use crate::AppState;
use actix_web::{
    Error, HttpMessage, HttpResponse,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::InternalError,
    middleware::Next,
    web::Data,
};
use actix_web::{
    http::{Method, StatusCode},
    web,
};
use error::ErrorResponse;
use jsonwebtoken::{DecodingKey, TokenData, Validation};
use service::{AuthenticatedUser, TokenPlayload};
use utils::constants;

pub fn decode_token(
    token: &str,
    key: &str,
) -> jsonwebtoken::errors::Result<TokenData<TokenPlayload>> {
    jsonwebtoken::decode::<TokenPlayload>(
        token,
        &DecodingKey::from_secret(key.as_bytes()),
        &Validation::default(),
    )
}

pub async fn verify_token(
    token_data: &TokenData<TokenPlayload>,
    state: &web::Data<AppState>,
) -> Option<AuthenticatedUser> {
    let username = &token_data.claims.sub;
    state.service.user.valid_login_token(username).await
}

fn invalid_auth() -> Error {
    let error_response = ErrorResponse::new(
        StatusCode::UNAUTHORIZED.as_u16(),
        constants::MESSAGE_UNAUTHORIZED,
    );
    InternalError::from_response(
        constants::MESSAGE_UNAUTHORIZED,
        HttpResponse::Unauthorized().json(error_response),
    )
    .into()
}

pub async fn async_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let req_path = req.path();
    if req.method() == Method::OPTIONS {
        return next.call(req).await;
    }
    let should_check = constants::IGNORE_ROUTES
        .iter()
        .all(|route| !req_path.starts_with(*route));

    if should_check {
        let auth_header = extract_token(&req);

        match auth_header {
            Some(header) => {
                let state = req.app_data::<Data<AppState>>();
                if state.is_none() {
                    return Err(invalid_auth());
                }

                let token_data =
                    decode_token(&header, &state.expect("state should valid").jwt_token);

                if token_data.is_err() {
                    return Err(invalid_auth());
                }

                let verify_token = verify_token(
                    &token_data.expect("token data should valid"),
                    state.expect("state should valid"),
                )
                .await;

                if verify_token.is_none() {
                    return Err(invalid_auth());
                }
                let token_value = verify_token.expect("verify token should valid");

                req.extensions_mut().insert(token_value);
            }
            None => return Err(invalid_auth()),
        };
    }

    let res = next.call(req).await?;

    Ok(res)
}

fn extract_token(req: &ServiceRequest) -> Option<String> {
    if let Some(auth_header) = req.headers().get("token")
        && let Some(token) = auth_header.to_str().ok()
    {
        return Some(token.to_string());
    }
    // 1. 尝试从 Header 获取 (例如 "Authorization: Bearer <token>")
    if let Some(auth_header) = req.headers().get(actix_web::http::header::AUTHORIZATION)
        && let Some(token) = auth_header.to_str().ok()
    {
        let t = token.strip_prefix("Bearer ").unwrap_or_default();
        return Some(t.to_string());
    }

    None
}
