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

#[cfg(feature = "gpui")]
mod proto_client;
#[cfg(feature = "gpui")]
pub use proto_client::*;

pub const PROTOCOL_VERSION: u32 = 68;
