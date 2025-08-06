pub mod auth;
mod conn;
mod extension;
mod message_stream;
mod notification;
mod peer;
mod websocket_yawc;

pub use conn::{Connection, YawcConnection};
pub use extension::*;
pub use notification::*;
pub use peer::*;
pub use proto;
pub use proto::{Receipt, TypedEnvelope, error::*};

// Export tungstenite types for existing /rpc endpoint
pub use async_tungstenite::tungstenite::Message as WebSocketMessage;

// Export yawc types for new /cloud endpoint  
pub use websocket_yawc::{
    Message as YawcMessage, 
    WebSocketAdapter as YawcWebSocketAdapter,
    build_websocket_request as build_yawc_websocket_request,
    CloseFrame,
};
pub use yawc::close::CloseCode;

mod macros;

#[cfg(feature = "gpui")]
mod proto_client;
#[cfg(feature = "gpui")]
pub use proto_client::*;

pub const PROTOCOL_VERSION: u32 = 68;