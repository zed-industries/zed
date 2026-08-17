use crate::{ConnectionTo, role::Role};

#[cfg(feature = "unstable_mcp_over_acp")]
use crate::schema::v1::{McpConnectionId, McpServerAcpId};

/// Describes how an MCP server connection was established.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum McpConnectionContext {
    /// The MCP server was connected directly, without an ACP transport.
    Standalone,

    /// The MCP server was attached to an ACP session.
    #[cfg(feature = "unstable_mcp_over_acp")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable_mcp_over_acp")))]
    Acp {
        /// The identifier advertised in the session's `McpServer::Acp` declaration.
        server_id: McpServerAcpId,

        /// The identifier for this active `mcp/connect` connection.
        connection_id: McpConnectionId,
    },
}

impl McpConnectionContext {
    /// Whether this MCP connection was established without an ACP transport.
    #[must_use]
    pub fn is_standalone(&self) -> bool {
        matches!(self, Self::Standalone)
    }

    /// The identifier advertised in the session's `McpServer::Acp` declaration.
    ///
    /// Returns `None` for a standalone MCP connection.
    #[cfg(feature = "unstable_mcp_over_acp")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable_mcp_over_acp")))]
    #[must_use]
    pub fn server_id(&self) -> Option<&McpServerAcpId> {
        match self {
            Self::Standalone => None,
            Self::Acp { server_id, .. } => Some(server_id),
        }
    }

    /// The identifier for the active `mcp/connect` connection.
    ///
    /// Returns `None` for a standalone MCP connection.
    #[cfg(feature = "unstable_mcp_over_acp")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable_mcp_over_acp")))]
    #[must_use]
    pub fn connection_id(&self) -> Option<&McpConnectionId> {
        match self {
            Self::Standalone => None,
            Self::Acp { connection_id, .. } => Some(connection_id),
        }
    }
}

/// Connection information available to an MCP server.
#[derive(Clone, Debug)]
pub struct McpConnectionTo<Counterpart: Role> {
    pub(super) context: McpConnectionContext,
    pub(super) connection: ConnectionTo<Counterpart>,
}

impl<Counterpart: Role> McpConnectionTo<Counterpart> {
    /// Describes whether this is a standalone or ACP-attached MCP connection.
    #[must_use]
    pub fn context(&self) -> &McpConnectionContext {
        &self.context
    }

    /// The identifier advertised in the session's `McpServer::Acp` declaration.
    ///
    /// Returns `None` for a standalone MCP connection.
    #[cfg(feature = "unstable_mcp_over_acp")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable_mcp_over_acp")))]
    #[must_use]
    pub fn server_id(&self) -> Option<&McpServerAcpId> {
        self.context.server_id()
    }

    /// The identifier for the active `mcp/connect` connection.
    ///
    /// Returns `None` for a standalone MCP connection.
    #[cfg(feature = "unstable_mcp_over_acp")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable_mcp_over_acp")))]
    #[must_use]
    pub fn connection_id(&self) -> Option<&McpConnectionId> {
        self.context.connection_id()
    }

    /// Borrow the host protocol connection.
    ///
    /// For an ACP-attached server, this is its host ACP connection. For a
    /// standalone server, this is the direct MCP client connection.
    #[must_use]
    pub fn connection(&self) -> &ConnectionTo<Counterpart> {
        &self.connection
    }
}

#[cfg(test)]
mod tests {
    use super::McpConnectionContext;

    #[test]
    fn standalone_context_is_explicit() {
        let context = McpConnectionContext::Standalone;

        assert!(context.is_standalone());

        #[cfg(feature = "unstable_mcp_over_acp")]
        {
            assert_eq!(context.server_id(), None);
            assert_eq!(context.connection_id(), None);
        }
    }

    #[cfg(feature = "unstable_mcp_over_acp")]
    #[test]
    fn acp_context_exposes_server_and_connection_ids() {
        use crate::schema::v1::{McpConnectionId, McpServerAcpId};

        let server_id = McpServerAcpId::new("server-id");
        let connection_id = McpConnectionId::new("connection-id");
        let context = McpConnectionContext::Acp {
            server_id: server_id.clone(),
            connection_id: connection_id.clone(),
        };

        assert!(!context.is_standalone());
        assert_eq!(context.server_id(), Some(&server_id));
        assert_eq!(context.connection_id(), Some(&connection_id));
    }
}
