use std::{future::Future, marker::PhantomData, path::Path};

use futures::channel::{mpsc, oneshot};

use crate::{
    Agent, Client, ConnectionTo, Dispatch, HandleDispatchFrom, Handled, Responder, Role,
    jsonrpc::{
        DynamicHandlerGuard,
        run::{NullRun, RunWithConnectionTo},
    },
    role::{HasPeer, acp::ProxySessionMessages},
    schema::v1::{
        ContentBlock, ContentChunk, NewSessionRequest, NewSessionResponse, PromptRequest,
        PromptResponse, SessionId, SessionModeState, SessionNotification, SessionUpdate,
        StopReason,
    },
    util::{MatchDispatch, MatchDispatchFrom, run_until},
};

#[cfg(feature = "unstable_mcp_over_acp")]
use crate::{jsonrpc::run::ChainRun, mcp_server::McpServer};

/// Marker type indicating the session builder will block the current task.
#[derive(Debug)]
pub struct Blocking;
impl SessionBlockState for Blocking {}

/// Marker type indicating the session builder will not block the current task.
#[derive(Debug)]
pub struct NonBlocking;
impl SessionBlockState for NonBlocking {}

/// Trait for marker types that indicate blocking vs non-blocking API.
/// See [`SessionBuilder::block_task`].
pub trait SessionBlockState: Send + 'static + Sync + std::fmt::Debug {}

impl<Counterpart: Role> ConnectionTo<Counterpart>
where
    Counterpart: HasPeer<Agent>,
{
    /// Session builder for a new session request.
    pub fn build_session(&self, cwd: impl AsRef<Path>) -> SessionBuilder<Counterpart, NullRun> {
        SessionBuilder::new(self, NewSessionRequest::new(cwd.as_ref()))
    }

    /// Session builder using the current working directory.
    ///
    /// This is a convenience wrapper around [`build_session`](Self::build_session)
    /// that uses [`std::env::current_dir`] to get the working directory.
    ///
    /// Returns an error if the current directory cannot be determined.
    pub fn build_session_cwd(&self) -> Result<SessionBuilder<Counterpart, NullRun>, crate::Error> {
        let cwd = std::env::current_dir().map_err(|e| {
            crate::Error::internal_error().data(format!("cannot get current directory: {e}"))
        })?;
        Ok(self.build_session(cwd))
    }

    /// Session builder starting from an existing request.
    ///
    /// Use this when you've intercepted a `session.new` request and want to
    /// modify it (e.g., inject MCP servers) before forwarding.
    pub fn build_session_from(
        &self,
        request: NewSessionRequest,
    ) -> SessionBuilder<Counterpart, NullRun> {
        SessionBuilder::new(self, request)
    }

    /// Given a session response received from the agent,
    /// attach a handler to process messages related to this session
    /// and let you access them.
    ///
    /// Normally you would not use this method directly but would
    /// instead use [`Self::build_session`] and then [`SessionBuilder::start_session`].
    ///
    /// The vector `dynamic_handler_registrations` contains any dynamic
    /// handle registrations associated with this session (e.g., from MCP servers).
    /// You can simply pass `Default::default()` if not applicable.
    pub(crate) fn attach_session<'runner>(
        &self,
        response: NewSessionResponse,
        mcp_handler_registrations: Vec<DynamicHandlerGuard<Counterpart>>,
    ) -> Result<ActiveSession<'runner, Counterpart>, crate::Error> {
        let NewSessionResponse {
            session_id,
            modes,
            meta,
            ..
        } = response;

        let (update_tx, update_rx) = mpsc::unbounded();
        let handler = ActiveSessionHandler::new(session_id.clone(), update_tx.clone());
        let session_handler_registration = self.add_dynamic_handler(handler)?;

        Ok(ActiveSession {
            session_id,
            modes,
            meta,
            update_rx,
            update_tx,
            connection: self.clone(),
            session_handler_registration,
            mcp_handler_registrations,
            _runner: PhantomData,
        })
    }
}

/// Session builder for a new session request.
/// Allows you to add MCP servers or set other details for this session.
///
/// The `BlockState` type parameter tracks whether blocking methods are available:
/// - `NonBlocking` (default): Only [`on_session_start`](Self::on_session_start) is available
/// - `Blocking` (after calling [`block_task`](Self::block_task)):
///   [`run_until`](Self::run_until) and [`start_session`](Self::start_session) become available
#[must_use = "use `start_session`, `run_until`, or `on_session_start` to start the session"]
#[derive(Debug)]
pub struct SessionBuilder<
    Counterpart,
    Run: RunWithConnectionTo<Counterpart> = NullRun,
    BlockState: SessionBlockState = NonBlocking,
> where
    Counterpart: HasPeer<Agent>,
{
    connection: ConnectionTo<Counterpart>,
    request: NewSessionRequest,
    dynamic_handler_registrations: Vec<DynamicHandlerGuard<Counterpart>>,
    run: Run,
    block_state: PhantomData<BlockState>,
}

impl<Counterpart> SessionBuilder<Counterpart, NullRun, NonBlocking>
where
    Counterpart: HasPeer<Agent>,
{
    fn new(connection: &ConnectionTo<Counterpart>, request: NewSessionRequest) -> Self {
        SessionBuilder {
            connection: connection.clone(),
            request,
            dynamic_handler_registrations: Vec::default(),
            run: NullRun,
            block_state: PhantomData,
        }
    }
}

impl<Counterpart, R, BlockState> SessionBuilder<Counterpart, R, BlockState>
where
    Counterpart: HasPeer<Agent>,
    R: RunWithConnectionTo<Counterpart>,
    BlockState: SessionBlockState,
{
    /// Attach an MCP server to this new session.
    #[cfg(feature = "unstable_mcp_over_acp")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable_mcp_over_acp")))]
    pub fn with_mcp_server<McpRun>(
        mut self,
        mcp_server: McpServer<Counterpart, McpRun>,
    ) -> Result<SessionBuilder<Counterpart, ChainRun<R, McpRun>, BlockState>, crate::Error>
    where
        McpRun: RunWithConnectionTo<Counterpart>,
    {
        let (handler, mcp_run) = mcp_server.into_handler_and_runner();
        self.dynamic_handler_registrations
            .push(handler.into_dynamic_handler(&mut self.request, &self.connection)?);
        Ok(SessionBuilder {
            connection: self.connection,
            request: self.request,
            dynamic_handler_registrations: self.dynamic_handler_registrations,
            run: ChainRun::new(self.run, mcp_run),
            block_state: self.block_state,
        })
    }

    /// Spawn a task that runs the provided closure once the session starts.
    ///
    /// Unlike [`start_session`](Self::start_session), this method returns immediately
    /// without blocking the current task. The session handshake and closure execution
    /// happen in a spawned background task.
    ///
    /// The closure receives an `ActiveSession<'static, _>` and runs in a
    /// spawned task. If it returns an error, the error propagates to the
    /// connection's task handling.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use agent_client_protocol::{Client, Agent, ConnectTo};
    /// # use agent_client_protocol::mcp_server::McpServer;
    /// # use agent_client_protocol_rmcp::McpServerExt;
    /// # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
    /// # Client.builder().connect_with(transport, async |cx| {
    /// # let mcp = McpServer::<Agent, _>::builder("tools").build();
    /// cx.build_session_cwd()?
    ///     .with_mcp_server(mcp)?
    ///     .on_session_start(async |mut session| {
    ///         // Do something with the session
    ///         session.send_prompt("Hello")?;
    ///         let response = session.read_to_string().await?;
    ///         Ok(())
    ///     })?;
    /// // Returns immediately, session runs in background
    /// # Ok(())
    /// # }).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Ordering
    ///
    /// Session runners are scheduled and routing setup is installed before the
    /// dispatch loop processes the next message when the session response is
    /// routed during its original dispatch. No user callback code runs under
    /// that ordering guarantee: the callback is invoked in a spawned task, so
    /// it may wait for later session traffic without deadlocking the connection.
    /// A response interceptor that retains the response and routes it later
    /// cannot retroactively order session setup before messages the dispatch
    /// loop has already processed.
    pub fn on_session_start<F, Fut>(self, op: F) -> Result<(), crate::Error>
    where
        R: 'static,
        F: FnOnce(ActiveSession<'static, Counterpart>) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), crate::Error>> + Send,
    {
        let Self {
            connection,
            request,
            dynamic_handler_registrations,
            run,
            block_state: _,
        } = self;

        connection
            .send_request_to(Agent, request)
            .on_receiving_result({
                let connection = connection.clone();
                async move |result| {
                    let response = result?;

                    connection.spawn(run.run_with_connection_to(connection.clone()))?;

                    let active_session =
                        connection.attach_session(response, dynamic_handler_registrations)?;

                    connection.spawn(async move { op(active_session).await })
                }
            })
    }

    /// Spawn a proxy session and run a closure with the session ID.
    ///
    /// A **proxy session** starts the session with the agent and then automatically
    /// proxies all session updates (prompts, tool calls, etc.) from the agent back
    /// to the client. You don't need to handle any messages yourself - the proxy
    /// takes care of forwarding everything. This is useful when you want to inject
    /// and/or filter prompts coming from the client but otherwise not be involved
    /// in the session.
    ///
    /// Unlike [`start_session_proxy`](Self::start_session_proxy), this method returns
    /// immediately without blocking the current task. The session handshake, client
    /// response, and proxy setup all happen in a spawned background task.
    ///
    /// The closure receives the `SessionId` once the session is established. Use it for logging
    /// or eventual tracking; it runs concurrently with later connection traffic. Register
    /// ID-independent state that later handlers must observe before calling this helper. For
    /// ID-keyed bookkeeping, install a gate or placeholder first, make later handlers await it,
    /// and populate it from the closure.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use agent_client_protocol::{Proxy, Client, Conductor, ConnectTo};
    /// # use agent_client_protocol::schema::v1::NewSessionRequest;
    /// # use agent_client_protocol::mcp_server::McpServer;
    /// # use agent_client_protocol_rmcp::McpServerExt;
    /// # async fn example(transport: impl ConnectTo<Proxy>) -> Result<(), agent_client_protocol::Error> {
    /// Proxy.builder()
    ///     .on_receive_request_from(Client, async |request: NewSessionRequest, responder, cx| {
    ///         let mcp = McpServer::<Conductor, _>::builder("tools").build();
    ///         cx.build_session_from(request)
    ///             .with_mcp_server(mcp)?
    ///             .on_proxy_session_start(responder, async |session_id| {
    ///                 // Session started
    ///                 Ok(())
    ///             })
    ///     }, agent_client_protocol::on_receive_request!())
    ///     .connect_to(transport)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Ordering
    ///
    /// The client response is queued, proxy routing is installed, and session runners are
    /// scheduled before the dispatch loop processes the next message when the session response
    /// is routed during its original dispatch. This is a local ordering guarantee, not a
    /// guarantee that the response reaches the client before later wire traffic. No user callback
    /// code runs under the barrier: the callback is invoked in a spawned task, so it may wait for
    /// later connection traffic. A response interceptor that retains the response and routes it
    /// later cannot retroactively order this setup before messages the loop already processed.
    pub fn on_proxy_session_start<F, Fut>(
        self,
        responder: Responder<NewSessionResponse>,
        op: F,
    ) -> Result<(), crate::Error>
    where
        F: FnOnce(SessionId) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), crate::Error>> + Send,
        Counterpart: HasPeer<Client>,
        R: 'static,
    {
        let Self {
            connection,
            request,
            dynamic_handler_registrations,
            run,
            block_state: _,
        } = self;

        // Send the "new session" request to the agent.
        let sent = connection.send_request_to(Agent, request);
        let sent = sent.forward_cancellation_from(responder.cancellation());

        sent.on_receiving_ok_result(responder, {
            let connection = connection.clone();
            async move |response, responder| {
                // Extract the session-id from the response and forward
                // the response back to the client
                let session_id = response.session_id.clone();
                responder.respond(response)?;

                // Install a dynamic handler to proxy messages from this session
                connection
                    .add_dynamic_handler(ProxySessionMessages::new(session_id.clone()))?
                    .detach();

                // Spawn off the run and dynamic handlers to run indefinitely
                connection.spawn(run.run_with_connection_to(connection.clone()))?;
                dynamic_handler_registrations
                    .into_iter()
                    .for_each(DynamicHandlerGuard::detach);

                connection.spawn(async move { op(session_id).await })
            }
        })
    }
}

impl<Counterpart, R> SessionBuilder<Counterpart, R, NonBlocking>
where
    Counterpart: HasPeer<Agent>,
    R: RunWithConnectionTo<Counterpart>,
{
    /// Mark this session builder as being able to block the current task.
    ///
    /// After calling this, you can use [`run_until`](Self::run_until) or
    /// [`start_session`](Self::start_session) which block the current task.
    ///
    /// This should not be used from inside a message handler like
    /// [`Builder::on_receive_request`](`crate::Builder::on_receive_request`) or [`HandleDispatchFrom`]
    /// implementations.
    pub fn block_task(self) -> SessionBuilder<Counterpart, R, Blocking> {
        SessionBuilder {
            connection: self.connection,
            request: self.request,
            dynamic_handler_registrations: self.dynamic_handler_registrations,
            run: self.run,
            block_state: PhantomData,
        }
    }
}

impl<Counterpart, R> SessionBuilder<Counterpart, R, Blocking>
where
    Counterpart: HasPeer<Agent>,
    R: RunWithConnectionTo<Counterpart>,
{
    /// Run this session synchronously. The current task will be blocked
    /// and `op` will be executed with the active session information.
    /// This is useful when you have MCP servers that are borrowed from your local
    /// stack frame.
    ///
    /// The `ActiveSession` passed to `op` has a non-`'static` lifetime, which
    /// prevents calling [`ActiveSession::proxy_remaining_messages`] (since the
    /// session's background runners would terminate when `op` returns).
    ///
    /// Requires calling [`block_task`](Self::block_task) first.
    pub async fn run_until<T>(
        self,
        op: impl for<'runner> AsyncFnOnce(
            ActiveSession<'runner, Counterpart>,
        ) -> Result<T, crate::Error>,
    ) -> Result<T, crate::Error> {
        let Self {
            connection,
            request,
            dynamic_handler_registrations,
            run,
            block_state: _,
        } = self;

        let response = connection
            .send_request_to(Agent, request)
            .block_task()
            .await?;

        let active_session = connection.attach_session(response, dynamic_handler_registrations)?;

        run_until(
            run.run_with_connection_to(connection.clone()),
            op(active_session),
        )
        .await
    }

    /// Send the request to create the session and return a handle.
    /// This is an alternative to [`Self::run_until`] that avoids rightward
    /// drift but at the cost of requiring MCP servers that are `Send` and
    /// don't access data from the surrounding scope.
    ///
    /// Returns an `ActiveSession<'static, _>` because the session's runners are spawned into
    /// background tasks that live for the connection lifetime.
    ///
    /// Requires calling [`block_task`](Self::block_task) first.
    pub async fn start_session(self) -> Result<ActiveSession<'static, Counterpart>, crate::Error>
    where
        R: 'static,
    {
        let Self {
            connection,
            request,
            dynamic_handler_registrations,
            run,
            block_state: _,
        } = self;

        let (active_session_tx, active_session_rx) = oneshot::channel();

        connection.clone().spawn(async move {
            let response = connection
                .send_request_to(Agent, request)
                .block_task()
                .await?;

            connection.spawn(run.run_with_connection_to(connection.clone()))?;

            let active_session =
                connection.attach_session(response, dynamic_handler_registrations)?;

            active_session_tx
                .send(active_session)
                .map_err(|_| crate::Error::internal_error())?;

            Ok(())
        })?;

        active_session_rx
            .await
            .map_err(|_| crate::Error::internal_error())
    }

    /// Start a proxy session that forwards all messages between client and agent.
    ///
    /// A **proxy session** starts the session with the agent and then automatically
    /// proxies all session updates (prompts, tool calls, etc.) from the agent back
    /// to the client. You don't need to handle any messages yourself - the proxy
    /// takes care of forwarding everything. This is useful when you want to inject
    /// and/or filter prompts coming from the client but otherwise not be involved
    /// in the session.
    ///
    /// This is a convenience method that combines [`start_session`](Self::start_session),
    /// responding to the client, and [`ActiveSession::proxy_remaining_messages`].
    ///
    /// For more control (e.g., to send some messages before proxying), use
    /// [`start_session`](Self::start_session) instead and call
    /// [`proxy_remaining_messages`](ActiveSession::proxy_remaining_messages) manually.
    ///
    /// Requires calling [`block_task`](Self::block_task) first.
    pub async fn start_session_proxy(
        self,
        responder: Responder<NewSessionResponse>,
    ) -> Result<SessionId, crate::Error>
    where
        Counterpart: HasPeer<Client>,
        R: 'static,
    {
        let active_session = self.start_session().await?;
        let session_id = active_session.session_id().clone();
        responder.respond(active_session.response())?;
        active_session.proxy_remaining_messages()?;
        Ok(session_id)
    }
}

/// Active session struct that lets you send prompts and receive updates.
///
/// The `'runner` lifetime represents the span during which session support runners
/// (such as MCP servers) are active. When created via [`SessionBuilder::start_session`],
/// this is `'static` because the runners are spawned into background tasks.
/// When created via [`SessionBuilder::run_until`], this is tied to the
/// closure scope, preventing [`Self::proxy_remaining_messages`] from being called
/// (since the runners would stop when the closure returns).
#[derive(Debug)]
pub struct ActiveSession<'runner, Link>
where
    Link: HasPeer<Agent>,
{
    session_id: SessionId,
    update_rx: mpsc::UnboundedReceiver<SessionMessage>,
    update_tx: mpsc::UnboundedSender<SessionMessage>,
    modes: Option<SessionModeState>,
    meta: Option<serde_json::Map<String, serde_json::Value>>,
    connection: ConnectionTo<Link>,

    /// Registration for the handler that routes session messages to `update_rx`.
    /// This is separate from MCP handlers so it can be dropped independently
    /// when switching to proxy mode.
    session_handler_registration: DynamicHandlerGuard<Link>,

    /// Registrations for MCP server handlers.
    /// These will be dropped once the active-session struct is dropped
    /// which will cause them to be deregistered.
    mcp_handler_registrations: Vec<DynamicHandlerGuard<Link>>,

    /// Phantom lifetime representing the session-runner lifetime.
    _runner: PhantomData<&'runner ()>,
}

/// Incoming message from the agent
#[non_exhaustive]
#[derive(Debug)]
#[allow(
    clippy::large_enum_variant,
    reason = "Dispatch messages vastly outnumber StopReason; boxing would add a heap allocation"
)]
pub enum SessionMessage {
    /// Periodic updates with new content, tool requests, etc.
    /// Use [`MatchDispatch`] to match on the message type.
    SessionMessage(Dispatch),

    /// When a prompt completes, the stop reason.
    StopReason(StopReason),
}

impl<Link> ActiveSession<'_, Link>
where
    Link: HasPeer<Agent>,
{
    /// Access the session ID.
    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    /// Access modes available in this session.
    pub fn modes(&self) -> Option<&SessionModeState> {
        self.modes.as_ref()
    }

    /// Access meta data from session response.
    pub fn meta(&self) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.meta.as_ref()
    }

    /// Build a `NewSessionResponse` from the session information.
    ///
    /// Useful when you need to forward the session response to a client
    /// after doing some processing.
    pub fn response(&self) -> NewSessionResponse {
        NewSessionResponse::new(self.session_id.clone())
            .modes(self.modes.clone())
            .meta(self.meta.clone())
    }

    /// Access the underlying connection context used to communicate with the agent.
    pub fn connection(&self) -> &ConnectionTo<Link> {
        &self.connection
    }

    /// Send a prompt to the agent. You can then read messages sent in response.
    pub fn send_prompt(&mut self, prompt: impl ToString) -> Result<(), crate::Error> {
        let update_tx = self.update_tx.clone();
        self.connection
            .send_request_to(
                Agent,
                PromptRequest::new(self.session_id.clone(), vec![prompt.to_string().into()]),
            )
            .on_receiving_result(async move |result| {
                let PromptResponse { stop_reason, .. } = result?;

                update_tx
                    .unbounded_send(SessionMessage::StopReason(stop_reason))
                    .map_err(crate::util::internal_error)?;

                Ok(())
            })
    }

    /// Read an update from the agent in response to the prompt.
    pub async fn read_update(&mut self) -> Result<SessionMessage, crate::Error> {
        use futures::StreamExt;
        let message =
            self.update_rx.next().await.ok_or_else(|| {
                crate::util::internal_error("session channel closed unexpectedly")
            })?;

        Ok(message)
    }

    /// Read all updates until the end of the turn and create a string.
    /// Ignores non-text updates.
    pub async fn read_to_string(&mut self) -> Result<String, crate::Error> {
        let mut output = String::new();
        loop {
            let update = self.read_update().await?;
            tracing::trace!(?update, "read_to_string update");
            match update {
                SessionMessage::SessionMessage(dispatch) => MatchDispatch::new(dispatch)
                    .if_notification(async |notif: SessionNotification| match notif.update {
                        SessionUpdate::AgentMessageChunk(ContentChunk {
                            content: ContentBlock::Text(text),
                            ..
                        }) => {
                            output.push_str(&text.text);
                            Ok(())
                        }
                        _ => Ok(()),
                    })
                    .await
                    .otherwise_ignore()?,
                SessionMessage::StopReason(_stop_reason) => break,
            }
        }
        Ok(output)
    }
}

impl<Link> ActiveSession<'static, Link>
where
    Link: HasPeer<Agent>,
{
    /// Proxy all remaining messages for this session between client and agent.
    ///
    /// Use this when you want to inject MCP servers into a session but don't need
    /// to actively interact with it after setup. The session messages will be proxied
    /// between client and agent automatically.
    ///
    /// This consumes the `ActiveSession` since you're giving up active control.
    ///
    /// This method is only available on `ActiveSession<'static, _>` (from
    /// [`SessionBuilder::start_session`]) because it requires the session's runners to outlive
    /// the method call.
    ///
    /// # Message Ordering Guarantees
    ///
    /// This method ensures proper handoff from active session mode to proxy mode
    /// without losing or reordering messages:
    ///
    /// 1. **Stop the session handler** - Drop the registration that routes messages
    ///    to `update_rx`. After this, no new messages will be queued.
    /// 2. **Close the channel** - Drop `update_tx` so we can detect when the channel
    ///    is fully drained.
    /// 3. **Drain queued messages** - Forward any messages that were already queued
    ///    in `update_rx` to the client, preserving order.
    /// 4. **Install proxy handler** - Now that all queued messages are forwarded,
    ///    install the proxy handler to handle future messages.
    ///
    /// This sequence prevents the race condition where messages could be delivered
    /// out of order or lost during the transition.
    pub fn proxy_remaining_messages(self) -> Result<(), crate::Error>
    where
        Link: HasPeer<Client>,
    {
        // Destructure self to get ownership of all fields
        let ActiveSession {
            session_id,
            mut update_rx,
            update_tx,
            connection,
            session_handler_registration,
            mcp_handler_registrations,
            // These fields are not needed for proxying
            modes: _,
            meta: _,
            _runner,
        } = self;

        // Step 1: Drop the session handler registration.
        // This unregisters the handler that was routing messages to update_rx.
        // After this point, no new messages will be added to the channel.
        drop(session_handler_registration);

        // Step 2: Drop the sender side of the channel.
        // This allows us to detect when the channel is fully drained
        // (recv will return None when empty and sender is dropped).
        drop(update_tx);

        // Step 3: Drain any messages that were already queued and forward to client.
        // These messages arrived before we dropped the handler but haven't been
        // consumed yet. We must forward them to maintain message ordering.
        while let Ok(message) = update_rx.try_recv() {
            match message {
                SessionMessage::SessionMessage(dispatch) => {
                    // Forward the message to the client
                    connection.send_proxied_message_to(Client, dispatch)?;
                }
                SessionMessage::StopReason(_) => {
                    // StopReason is internal bookkeeping, not forwarded
                }
            }
        }

        // Step 4: Install the proxy handler for future messages.
        // Now that all queued messages have been forwarded, the proxy handler
        // can take over. Any new messages will go directly through the proxy.
        connection
            .add_dynamic_handler(ProxySessionMessages::new(session_id))?
            .detach();

        // Keep MCP server handlers alive for the lifetime of the proxy
        for registration in mcp_handler_registrations {
            registration.detach();
        }

        Ok(())
    }
}

struct ActiveSessionHandler {
    session_id: SessionId,
    update_tx: mpsc::UnboundedSender<SessionMessage>,
}

impl ActiveSessionHandler {
    pub fn new(session_id: SessionId, update_tx: mpsc::UnboundedSender<SessionMessage>) -> Self {
        Self {
            session_id,
            update_tx,
        }
    }
}

impl<Counterpart: Role> HandleDispatchFrom<Counterpart> for ActiveSessionHandler
where
    Counterpart: HasPeer<Agent>,
{
    async fn handle_dispatch_from(
        &mut self,
        message: Dispatch,
        cx: ConnectionTo<Counterpart>,
    ) -> Result<Handled<Dispatch>, crate::Error> {
        // If this is a message for our session, grab it.
        tracing::trace!(
            ?message,
            handler_session_id = ?self.session_id,
            "ActiveSessionHandler::handle_dispatch"
        );
        MatchDispatchFrom::new(message, &cx)
            .if_dispatch_from(Agent, async |message| {
                if let Some(session_id) = message.get_session_id()? {
                    tracing::trace!(
                        message_session_id = ?session_id,
                        handler_session_id = ?self.session_id,
                        "ActiveSessionHandler::handle_dispatch"
                    );
                    if session_id == self.session_id {
                        self.update_tx
                            .unbounded_send(SessionMessage::SessionMessage(message))
                            .map_err(crate::util::internal_error)?;
                        return Ok(Handled::Yes);
                    }
                }

                // Otherwise, pass it through.
                Ok(Handled::No {
                    message,
                    retry: false,
                })
            })
            .await
            .done()
    }

    fn describe_chain(&self) -> impl std::fmt::Debug {
        format!("ActiveSessionHandler({})", self.session_id)
    }
}
