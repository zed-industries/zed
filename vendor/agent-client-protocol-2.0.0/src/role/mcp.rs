//! MCP (Model Context Protocol) role types.
//!
//! These roles are used for MCP connections, which are separate from ACP but
//! use the same underlying connection infrastructure.

use crate::{
    Handled, RoleId,
    jsonrpc::{Builder, handlers::NullHandler, run::NullRun},
    role::{HasPeer, RemoteStyle, Role},
};

/// The MCP client role - connects to MCP servers to access tools and resources.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Client;

impl Role for Client {
    type Counterpart = Server;

    fn role_id(&self) -> RoleId {
        RoleId::from_singleton(self)
    }

    fn counterpart(&self) -> Self::Counterpart {
        Server
    }

    async fn default_handle_dispatch_from(
        &self,
        message: crate::Dispatch,
        _connection: crate::ConnectionTo<Self>,
    ) -> Result<crate::Handled<crate::Dispatch>, crate::Error> {
        Ok(Handled::No {
            message,
            retry: false,
        })
    }
}

impl Client {
    /// Create a connection builder for an MCP client.
    pub fn builder(self) -> Builder<Client, NullHandler, NullRun> {
        Builder::new(self)
    }
}

impl HasPeer<Client> for Client {
    fn remote_style(&self, _peer: Client) -> RemoteStyle {
        RemoteStyle::Counterpart
    }
}

/// The MCP server role - provides tools and resources to MCP clients.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Server;

impl Role for Server {
    type Counterpart = Client;

    fn role_id(&self) -> RoleId {
        RoleId::from_singleton(self)
    }

    fn counterpart(&self) -> Self::Counterpart {
        Client
    }

    async fn default_handle_dispatch_from(
        &self,
        message: crate::Dispatch,
        _connection: crate::ConnectionTo<Self>,
    ) -> Result<crate::Handled<crate::Dispatch>, crate::Error> {
        Ok(Handled::No {
            message,
            retry: false,
        })
    }
}

impl Server {
    /// Create a connection builder for an MCP server.
    pub fn builder(self) -> Builder<Server, NullHandler, NullRun> {
        Builder::new(self)
    }
}

impl HasPeer<Server> for Server {
    fn remote_style(&self, _peer: Server) -> RemoteStyle {
        RemoteStyle::Counterpart
    }
}
