pub mod auth;
mod conn;
mod notification;
mod peer;
pub mod proto;

pub use conn::Connection;
pub use peer::*;
pub use notification::*;
mod macros;

pub const PROTOCOL_VERSION: u32 = 64;
