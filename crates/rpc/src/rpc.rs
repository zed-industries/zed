pub mod auth;
mod conn;
mod peer;
pub mod proto;
pub use conn::Connection;
pub use peer::*;
mod macros;

pub const PROTOCOL_VERSION: u32 = 16;
