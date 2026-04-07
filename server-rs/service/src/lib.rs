mod admin;
mod user;
use user::Service as UserService;

pub use admin::AdminService;
pub use user::{AuthenticatedUser, TokenPlayload};

#[derive(Debug, Clone)]
pub struct Service {
    pub user: UserService,
    pub admin: AdminService,
}

impl Service {
    pub fn new(repo: database::Repository) -> Self {
        Service {
            user: UserService::new(repo.clone()),
            admin: AdminService::new(repo),
        }
    }
}
