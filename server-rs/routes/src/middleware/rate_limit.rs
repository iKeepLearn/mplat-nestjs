use actix_extensible_rate_limit::{
    RateLimiter,
    backend::{
        SimpleInput, SimpleInputFuture, SimpleOutput, memory::InMemoryBackend, string_ip_key,
    },
};
use actix_web::dev::ServiceRequest;
use enum_map::{EnumMap, enum_map};
use std::{
    future::ready,
    sync::{Arc, RwLock},
    time::Duration,
};
use strum::{AsRefStr, Display};

#[derive(Debug, enum_map::Enum, Copy, Clone, Display, AsRefStr)]
pub enum ActionType {
    Common,
    Login,
    Register,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct BucketConfig {
    pub max_requests: u32,
    pub interval: u32,
}

#[derive(Clone)]
pub struct RateLimit {
    configs: Arc<RwLock<EnumMap<ActionType, BucketConfig>>>,
    backends: EnumMap<ActionType, InMemoryBackend<String>>,
}

impl RateLimit {
    pub fn new(configs: EnumMap<ActionType, BucketConfig>) -> Self {
        Self {
            configs: Arc::new(RwLock::new(configs)),
            backends: EnumMap::from_fn(|_| InMemoryBackend::<String>::builder().build()),
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(enum_map! {
          ActionType::Common => BucketConfig {
            max_requests: 180,
            interval: 60,
          },
          ActionType::Login => BucketConfig {
            max_requests: 60,
            interval: 300,
          },
          ActionType::Register => BucketConfig {
            max_requests: 3,
            interval: 3600,
          },

        })
    }

    #[allow(clippy::expect_used)]
    pub fn set_config(&self, configs: EnumMap<ActionType, BucketConfig>) {
        *self.configs.write().expect("write rwlock") = configs;
    }

    fn build_rate_limiter(
        &self,
        action_type: ActionType,
    ) -> RateLimiter<
        InMemoryBackend<String>,
        SimpleOutput,
        impl Fn(&ServiceRequest) -> SimpleInputFuture<String> + 'static,
    > {
        let input = new_input(action_type, self.configs.clone());

        RateLimiter::builder(self.backends[action_type].clone(), input)
            .add_headers()
            // rollback rate limit on any error 500
            .rollback_server_errors()
            .build()
    }

    pub fn common(
        &self,
    ) -> RateLimiter<
        InMemoryBackend<String>,
        SimpleOutput,
        impl Fn(&ServiceRequest) -> SimpleInputFuture<String> + 'static,
    > {
        self.build_rate_limiter(ActionType::Common)
    }

    pub fn login(
        &self,
    ) -> RateLimiter<
        InMemoryBackend<String>,
        SimpleOutput,
        impl Fn(&ServiceRequest) -> SimpleInputFuture<String> + 'static,
    > {
        self.build_rate_limiter(ActionType::Login)
    }
    pub fn register(
        &self,
    ) -> RateLimiter<
        InMemoryBackend<String>,
        SimpleOutput,
        impl Fn(&ServiceRequest) -> SimpleInputFuture<String> + 'static,
    > {
        self.build_rate_limiter(ActionType::Register)
    }
}

fn new_input(
    action_type: ActionType,
    configs: Arc<RwLock<EnumMap<ActionType, BucketConfig>>>,
) -> impl Fn(&ServiceRequest) -> SimpleInputFuture<String> + 'static {
    move |req| {
        ready({
            let key = extract_identifier(req);

            #[allow(clippy::expect_used)]
            let config = configs.read().expect("read rwlock")[action_type];

            let interval = Duration::from_secs(config.interval.into());
            let max_requests = config.max_requests.into();
            Ok(SimpleInput {
                interval,
                max_requests,
                key,
            })
        })
    }
}

fn extract_identifier(req: &ServiceRequest) -> String {
    if let Some(auth_header) = req.headers().get(actix_web::http::header::AUTHORIZATION)
        && let Ok(auth_str) = auth_header.to_str()
    {
        if let Some(token) = auth_str.strip_prefix("Bearer ") {
            return token.to_string();
        }
        return auth_str.to_string();
    }

    string_ip_key(req.connection_info().realip_remote_addr())
}
