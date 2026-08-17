//! JSON-RPC implementations for the unstable native MCP-over-ACP transport.

use crate::schema::v1::{
    ConnectMcpRequest, ConnectMcpResponse, DisconnectMcpRequest, DisconnectMcpResponse,
    MessageMcpNotification, MessageMcpRequest, MessageMcpResponse,
};

impl_jsonrpc_request!(ConnectMcpRequest, ConnectMcpResponse, "mcp/connect");
impl_jsonrpc_request!(MessageMcpRequest, MessageMcpResponse, "mcp/message");
impl_jsonrpc_notification!(MessageMcpNotification, "mcp/message");
impl_jsonrpc_request!(
    DisconnectMcpRequest,
    DisconnectMcpResponse,
    "mcp/disconnect"
);
