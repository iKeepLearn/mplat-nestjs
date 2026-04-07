mod admin;
mod user;
mod wxcallback;
use user::Service as UserService;

pub use admin::AdminService;
pub use user::{AuthenticatedUser, TokenPlayload};
pub use wxcallback::WxCallbackService;

#[derive(Debug, Clone)]
pub struct Service {
    pub user: UserService,
    pub admin: AdminService,
    pub wxcallback: WxCallbackService,
}

impl Service {
    pub fn new(repo: database::Repository) -> Self {
        Service {
            user: UserService::new(repo.clone()),
            admin: AdminService::new(repo.clone()),
            wxcallback: WxCallbackService::new(repo),
        }
    }
}
