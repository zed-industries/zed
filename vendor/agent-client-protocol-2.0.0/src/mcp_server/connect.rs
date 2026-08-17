use std::sync::Arc;

use crate::{
    DynConnectTo,
    mcp_server::McpConnectionTo,
    role::{self, Role},
};

/// Trait for types that can create MCP server connections.
///
/// Implement this trait to create custom MCP servers. Each call to [`connect`](Self::connect)
/// should return a new [`ConnectTo`](crate::ConnectTo) that serves MCP requests for a single
/// connection.
///
/// # Example
///
/// ```rust,ignore
/// use agent_client_protocol::mcp_server::{McpServerConnect, McpConnectionTo};
/// use agent_client_protocol::{DynConnectTo, role::Role};
///
/// struct MyMcpServer {
///     name: String,
/// }
///
/// impl<R: Role> McpServerConnect<R> for MyMcpServer {
///     fn name(&self) -> String {
///         self.name.clone()
///     }
///
///     fn connect(&self, cx: McpConnectionTo<R>) -> DynConnectTo<role::mcp::Client> {
///         // Create and return a component that handles MCP requests
///         DynConnectTo::new(MyMcpComponent::new(cx))
///     }
/// }
/// ```
pub trait McpServerConnect<Counterpart: Role>: Send + Sync + 'static {
    /// The name of the MCP server, used in ACP declarations when attached.
    fn name(&self) -> String;

    /// Create a component to service a new MCP connection.
    ///
    /// This is called each time an MCP client connects to this server. The returned
    /// component will handle MCP protocol messages for that connection.
    ///
    /// [`McpConnectionTo`] distinguishes a direct MCP connection from an
    /// ACP-attached connection and provides the corresponding host connection.
    fn connect(&self, cx: McpConnectionTo<Counterpart>) -> DynConnectTo<role::mcp::Client>;
}

impl<Counterpart: Role, S: ?Sized + McpServerConnect<Counterpart>> McpServerConnect<Counterpart>
    for Box<S>
{
    fn name(&self) -> String {
        S::name(self)
    }

    fn connect(&self, cx: McpConnectionTo<Counterpart>) -> DynConnectTo<role::mcp::Client> {
        S::connect(self, cx)
    }
}

impl<Counterpart: Role, S: ?Sized + McpServerConnect<Counterpart>> McpServerConnect<Counterpart>
    for Arc<S>
{
    fn name(&self) -> String {
        S::name(self)
    }

    fn connect(&self, cx: McpConnectionTo<Counterpart>) -> DynConnectTo<role::mcp::Client> {
        S::connect(self, cx)
    }
}
