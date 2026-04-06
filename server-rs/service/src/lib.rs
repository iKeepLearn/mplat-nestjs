mod user;
mod admin;
use user::Service as UserService;

pub use user::{AuthenticatedUser, TokenPlayload};
pub use admin::AdminService;

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
