//! MCP server construction, direct serving, and optional ACP attachment.

use std::{marker::PhantomData, sync::Arc};

use futures::{StreamExt, channel::mpsc};

use crate::{
    ConnectTo, Dispatch, DynConnectTo, Role,
    jsonrpc::run::{NullRun, RunWithConnectionTo},
    mcp_server::{McpConnectionContext, McpConnectionTo, McpServerConnect},
    role,
};

#[cfg(feature = "unstable_mcp_over_acp")]
use uuid::Uuid;

#[cfg(feature = "unstable_mcp_over_acp")]
use crate::{
    Agent, Client, ConnectionTo, HandleDispatchFrom, Handled,
    jsonrpc::DynamicHandlerGuard,
    mcp_server::active_session::McpActiveSession,
    schema::v1::{
        LoadSessionRequest, McpServer as SchemaMcpServer, McpServerAcp, McpServerAcpId,
        NewSessionRequest, ResumeSessionRequest,
    },
    util::MatchDispatchFrom,
};

#[cfg(feature = "unstable_mcp_over_acp")]
use crate::role::HasPeer;

#[cfg(all(feature = "unstable_mcp_over_acp", feature = "unstable_session_fork"))]
use crate::schema::v1::ForkSessionRequest;

/// A runtime-agnostic MCP server.
///
/// `McpServer` wraps an [`McpServerConnect`](`super::McpServerConnect`)
/// implementation. A server whose counterpart is [`role::mcp::Client`] can be
/// connected directly as a standalone MCP component. With the
/// `unstable_mcp_over_acp` feature, servers can instead be attached to ACP
/// session setup through `Builder::with_mcp_server` or
/// `SessionBuilder::with_mcp_server`.
///
/// # Creating an MCP Server
///
/// The `agent-client-protocol-rmcp` crate provides builder APIs for MCP tools
/// backed by the `rmcp` crate.
///
/// Or implement [`McpServerConnect`](`super::McpServerConnect`) for custom server behavior:
///
/// ```rust,ignore
/// let server = McpServer::new(MyCustomServerConnect, NullRun);
/// ```
pub struct McpServer<Counterpart: Role, Run = NullRun> {
    /// The host role that is serving up this MCP server
    phantom: PhantomData<Counterpart>,

    /// The "connect" instance
    connect: Arc<dyn McpServerConnect<Counterpart>>,

    /// The runner is a task that should be run alongside the message handler.
    /// Some futures direct messages back through channels to this future which actually
    /// handles responding to the client.
    ///
    /// Some connector implementations use this to run support tasks alongside
    /// the message handler.
    runner: Run,
}

impl<Counterpart: Role + std::fmt::Debug, Run: std::fmt::Debug> std::fmt::Debug
    for McpServer<Counterpart, Run>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("McpServer")
            .field("phantom", &self.phantom)
            .field("runner", &self.runner)
            .finish_non_exhaustive()
    }
}

impl<Counterpart: Role, Run> McpServer<Counterpart, Run>
where
    Run: RunWithConnectionTo<Counterpart>,
{
    /// Create an MCP server from something that implements the [`McpServerConnect`](`super::McpServerConnect`) trait.
    ///
    /// # See also
    ///
    /// See `agent-client-protocol-rmcp` to construct MCP servers from Rust code
    /// with `rmcp`.
    pub fn new(c: impl McpServerConnect<Counterpart>, runner: Run) -> Self {
        McpServer {
            phantom: PhantomData,
            connect: Arc::new(c),
            runner,
        }
    }

    /// Split this MCP server into the message handler and a future that must be run while the handler is active.
    #[cfg(feature = "unstable_mcp_over_acp")]
    pub(crate) fn into_handler_and_runner(self) -> (McpSessionHandler<Counterpart>, Run)
    where
        Counterpart: HasPeer<Agent>,
    {
        let Self {
            phantom: _,
            connect,
            runner,
        } = self;
        let server_id = McpServerAcpId::new(format!("mcp-server:{}", Uuid::new_v4()));
        (McpSessionHandler::new(server_id, connect), runner)
    }
}

/// Message handler created from a [`McpServer`].
#[cfg(feature = "unstable_mcp_over_acp")]
pub(crate) struct McpSessionHandler<Counterpart: Role>
where
    Counterpart: HasPeer<Agent>,
{
    server_id: McpServerAcpId,
    connect: Arc<dyn McpServerConnect<Counterpart>>,
    active_session: McpActiveSession<Counterpart>,
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl<Counterpart: Role> McpSessionHandler<Counterpart>
where
    Counterpart: HasPeer<Agent>,
{
    pub fn new(server_id: McpServerAcpId, connect: Arc<dyn McpServerConnect<Counterpart>>) -> Self {
        Self {
            active_session: McpActiveSession::new(server_id.clone(), connect.clone()),
            server_id,
            connect,
        }
    }

    /// Append this MCP server's native ACP declaration to a session setup request.
    fn append_declaration(&self, mcp_servers: &mut Vec<SchemaMcpServer>) {
        mcp_servers.push(SchemaMcpServer::Acp(McpServerAcp::new(
            self.connect.name(),
            self.server_id.clone(),
        )));
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl<Counterpart: Role> McpSessionHandler<Counterpart>
where
    Counterpart: HasPeer<Agent>,
{
    /// Attach this server to the new session, spawning off a dynamic handler that will
    /// manage requests coming from this session.
    ///
    /// # Return value
    ///
    /// Returns a [`DynamicHandlerGuard`] for the handler that intercepts messages
    /// related to this MCP server. Once the value is dropped, the MCP server messages
    /// will no longer be received, so you need to keep this value alive as long as the session
    /// is in use. You can also invoke [`DynamicHandlerGuard::detach`] if you
    /// want to keep the handler registered for the life of the connection.
    pub fn into_dynamic_handler(
        self,
        request: &mut NewSessionRequest,
        cx: &ConnectionTo<Counterpart>,
    ) -> Result<DynamicHandlerGuard<Counterpart>, crate::Error>
    where
        Counterpart: HasPeer<Agent>,
    {
        self.append_declaration(&mut request.mcp_servers);
        cx.add_dynamic_handler(self.active_session)
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl<Counterpart: Role> HandleDispatchFrom<Counterpart> for McpSessionHandler<Counterpart>
where
    Counterpart: HasPeer<Client> + HasPeer<Agent>,
{
    async fn handle_dispatch_from(
        &mut self,
        message: Dispatch,
        cx: ConnectionTo<Counterpart>,
    ) -> Result<Handled<Dispatch>, crate::Error> {
        let matcher = MatchDispatchFrom::new(message, &cx)
            .if_request_from(Client, async |mut request: NewSessionRequest, responder| {
                self.append_declaration(&mut request.mcp_servers);
                Ok(Handled::No {
                    message: (request, responder),
                    retry: false,
                })
            })
            .await
            .if_request_from(
                Client,
                async |mut request: LoadSessionRequest, responder| {
                    self.append_declaration(&mut request.mcp_servers);
                    Ok(Handled::No {
                        message: (request, responder),
                        retry: false,
                    })
                },
            )
            .await
            .if_request_from(
                Client,
                async |mut request: ResumeSessionRequest, responder| {
                    self.append_declaration(&mut request.mcp_servers);
                    Ok(Handled::No {
                        message: (request, responder),
                        retry: false,
                    })
                },
            )
            .await;

        #[cfg(feature = "unstable_session_fork")]
        let matcher = matcher
            .if_request_from(
                Client,
                async |mut request: ForkSessionRequest, responder| {
                    self.append_declaration(&mut request.mcp_servers);
                    Ok(Handled::No {
                        message: (request, responder),
                        retry: false,
                    })
                },
            )
            .await;

        matcher.otherwise_delegate(&mut self.active_session).await
    }

    fn describe_chain(&self) -> impl std::fmt::Debug {
        format!("McpServer({})", self.connect.name())
    }
}

impl<Run> ConnectTo<role::mcp::Client> for McpServer<role::mcp::Client, Run>
where
    Run: RunWithConnectionTo<role::mcp::Client> + 'static,
{
    async fn connect_to(
        self,
        client: impl ConnectTo<role::mcp::Server>,
    ) -> Result<(), crate::Error> {
        let Self {
            connect,
            runner,
            phantom: _,
        } = self;

        let (tx, mut rx) = mpsc::unbounded();

        role::mcp::Server
            .builder()
            .with_runner(runner)
            .on_receive_dispatch(
                async |message_from_client: Dispatch, _cx| {
                    tx.unbounded_send(message_from_client)
                        .map_err(|_| crate::util::internal_error("nobody listening to mcp server"))
                },
                crate::on_receive_dispatch!(),
            )
            .with_spawned(async move |connection_to_client| {
                let spawned_server: DynConnectTo<role::mcp::Client> =
                    connect.connect(McpConnectionTo {
                        context: McpConnectionContext::Standalone,
                        connection: connection_to_client.clone(),
                    });

                role::mcp::Client
                    .builder()
                    .on_receive_dispatch(
                        async |message_from_server: Dispatch, _| {
                            // when we receive a message from the server, fwd to the client
                            connection_to_client.send_proxied_message(message_from_server)
                        },
                        crate::on_receive_dispatch!(),
                    )
                    .connect_with(spawned_server, async |connection_to_server| {
                        while let Some(message_from_client) = rx.next().await {
                            connection_to_server.send_proxied_message(message_from_client)?;
                        }
                        Ok(())
                    })
                    .await
            })
            .connect_to(client)
            .await
    }
}
