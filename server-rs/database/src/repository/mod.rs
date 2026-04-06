mod user;

use sqlx::{Pool, Postgres};
use std::sync::Arc;
use user::UserRepository;

#[derive(Debug)]
pub struct Repository(Arc<RepositoryInner>);

#[derive(Debug, Clone)]
pub struct RepositoryInner {
    pub user: UserRepository,
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
        };

        Repository(Arc::new(inner))
    }
}
