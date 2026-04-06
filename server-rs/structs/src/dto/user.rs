use error::Error;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct AuthDto {
    #[validate(length(min = 1, message = "请确认username是否正确"))]
    pub username: String,
    #[validate(length(min = 1, message = "请确认password是否正确"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub jwt: String,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub username: String,
}
