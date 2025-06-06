pub mod auth;
mod conn;
mod extension;
mod message_stream;
mod notification;
mod peer;

pub use conn::Connection;
pub use extension::*;
pub use notification::*;
pub use peer::*;
pub use proto;
pub use proto::{Receipt, TypedEnvelope, error::*};
mod macros;

#[cfg(feature = "gpui")]
mod proto_client;
#[cfg(feature = "gpui")]
pub use proto_client::*;

pub const PROTOCOL_VERSION: u32 = 68;
