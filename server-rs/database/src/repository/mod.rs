mod admin;
mod comm_kv;
mod user;
mod wxcallback;

use admin::AdminRepository;
use comm_kv::CommKvRepository;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use user::UserRepository;
use wxcallback::WxCallbackRepository;

#[derive(Debug)]
pub struct Repository(Arc<RepositoryInner>);

#[derive(Debug, Clone)]
pub struct RepositoryInner {
    pub user: UserRepository,
    pub admin: AdminRepository,
    pub wxcallback: WxCallbackRepository,
    pub comm_kv: CommKvRepository,
}

impl Clone for Repository {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl std::ops::Deref for Repository {
    type Target = RepositoryInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Repository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        let inner = RepositoryInner {
            user: UserRepository::new(pool.clone()),
            admin: AdminRepository::new(pool.clone()),
            wxcallback: WxCallbackRepository::new(pool.clone()),
            comm_kv: CommKvRepository::new(pool),
        };

        Repository(Arc::new(inner))
    }
}
