use chrono::{Duration, Utc};
use database::Repository;
use error::Error;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use structs::dto::user::{AuthDto, AuthResponse, SignupResponse};
use tracing::error;

#[derive(Debug, Clone)]
pub struct Service {
    repository: Repository,
}

impl Service {
    pub fn new(repository: Repository) -> Self {
        Service { repository }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPlayload {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub username: String,
}

const BCRYPT_SALT_LENGTH: u32 = 10;

impl Service {
    pub async fn hash_password(&self, password: &str) -> Result<String, Error> {
        bcrypt::hash(password, BCRYPT_SALT_LENGTH).map_err(|e| {
            error!("hash_password error: {:?}", e);
            Error::internal("密码加密失败")
        })
    }

    pub async fn verify_password(&self, password: &str, hash: &str) -> Result<bool, Error> {
        bcrypt::verify(password, hash).map_err(|e| {
            error!("verify_password error: {:?}", e);
            Error::internal("密码验证失败")
        })
    }

    pub async fn signin(&self, dto: AuthDto, jwt_secret: &str) -> Result<AuthResponse, Error> {
        let user = self
            .repository
            .user
            .find_by_username_with_password_only(&dto.username)
            .await?;

        let Some(password_hash) = user else {
            return Err(Error::bad_request("未注册"));
        };

        let password_is_match = self.verify_password(&dto.password, &password_hash).await?;

        if !password_is_match {
            return Err(Error::bad_request("密码不正确"));
        }

        let access_token = self.sign_token(&dto.username, jwt_secret)?;

        Ok(AuthResponse { jwt: access_token })
    }

    fn sign_token(&self, username: &str, jwt_secret: &str) -> Result<String, Error> {
        let exp = Utc::now()
            .checked_add_signed(Duration::days(1))
            .expect("valid timestamp")
            .timestamp() as usize;

        let payload = TokenPlayload {
            sub: username.to_string(),
            exp,
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        )
        .map_err(|e| {
            error!("sign_token error: {:?}", e);
            Error::internal("Token生成失败")
        })
    }

    pub async fn signup(&self, dto: AuthDto) -> Result<SignupResponse, Error> {
        let encrypt_password = self.hash_password(&dto.password).await?;

        match self
            .repository
            .user
            .create_user(&dto.username, &encrypt_password)
            .await
        {
            Ok(_) => Ok(SignupResponse {
                username: dto.username,
            }),
            Err(Error::Internal(_)) => Err(Error::bad_request("该用户名已存在")),
            Err(e) => Err(e),
        }
    }

    pub async fn valid_login_token(&self, username: &str) -> Option<AuthenticatedUser> {
        let user = self.repository.user.find_by_username(username).await;

        match user {
            Ok(Some(_)) => Some(AuthenticatedUser {
                username: username.to_string(),
            }),
            _ => None,
        }
    }
}
