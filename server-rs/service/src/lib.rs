mod user;
use user::Service as UserService;

pub use user::{AuthenticatedUser, TokenPlayload};

#[derive(Debug, Clone)]
pub struct Service {
    pub user: UserService,
}

impl Service {
    pub fn new(repo: database::Repository) -> Self {
        Service {
            user: UserService::new(repo),
        }
    }
}
