//! JSON-RPC trait implementations for protocol-level (`$/`-prefixed) messages.

use crate::schema::v1::{CancelRequestNotification, ProtocolLevelNotification};

impl_jsonrpc_notification!(CancelRequestNotification, "$/cancel_request");

impl_jsonrpc_protocol_level_notification_enum!(ProtocolLevelNotification {
    CancelRequestNotification => "$/cancel_request",
});
