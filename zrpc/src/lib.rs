pub mod auth;
mod conn;
mod peer;
pub mod proto;
pub use conn::Connection;
pub use peer::*;

pub const VERSION: u32 = 0;
