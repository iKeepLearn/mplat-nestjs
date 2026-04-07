// Messages
pub const MESSAGE_UNAUTHORIZED: &str = "Unauthorized";
pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";
pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

// Headers
pub const AUTHORIZATION: &str = "Authorization";

// Misc
pub const EMPTY: &str = "";

// ignore routes - no auth required for these endpoints
pub const IGNORE_ROUTES: [&str; 3] = ["/api/auth", "/api/auth/signup", "/api/wxcallback"];

pub const EMPTY_STR: &str = "";
