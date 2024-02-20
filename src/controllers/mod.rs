pub mod message;
pub mod mw_auth;
pub mod user;

pub const AUTH_TOKEN: &str = "Authorization";
pub const BEARER: &str = "Bearer ";
pub const JWT_SECRET: &[u8] = b"secret";
