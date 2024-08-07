pub mod auth;
mod conn;
mod extension;
mod llm;
mod notification;
mod peer;
pub mod proto;

pub use conn::Connection;
pub use extension::*;
pub use llm::*;
pub use notification::*;
pub use peer::*;
pub use proto::{error::*, Receipt, TypedEnvelope};
mod macros;

pub const PROTOCOL_VERSION: u32 = 68;
