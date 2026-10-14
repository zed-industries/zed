use super::{
    AcpDebugLog, AcpDebugMessage, ClientContext, ForegroundWork, UnsupportedVersion,
    cancel_elicitations_after_disconnect, drain_foreground_queue, enqueue_notification,
    exited_load_error_after_drain, first_class_terminal_auth_task, map_acp_error, transport,
    with_v2_client_ingress,
};
use acp_thread::ElicitationStore;
use agent_client_protocol::{
    Agent, Client, ConnectTo, JsonRpcRequest, Lines, V2ConnectionTo,
    schema::{ProtocolVersion, v2 as acp},
};
use anyhow::{Context as _, Result, anyhow};
use collections::HashMap;
use futures::{FutureExt as _, StreamExt as _, channel::mpsc, future::Shared};
use gpui::{App, AppContext as _, AsyncApp, Entity, Task, WeakEntity};
use project::{AgentId, AgentServerStore, Project, agent_server_store::AgentServerCommand};
use std::{cell::RefCell, rc::Rc, time::Duration};
use task::SpawnInTerminal;
use util::{ResultExt as _, process::Child};

// Startup failures and dropped startup futures must also reap the spawned process.
struct OwnedChild(Child);

fn cancel_auth_elicitations(
    request_id: acp::RequestId,
    cx: &mut AsyncApp,
    context: &ClientContext,
) {
    context.request_elicitations.update(cx, |store, cx| {
        store.cancel_request(&request_id, cx);
    });
}

impl Drop for OwnedChild {
    fn drop(&mut self) {
        self.0.kill().log_err();
    }
}

/// An unselected v2 connection foundation, not yet an `AgentConnection` session backend.
pub struct AcpV2Connection {
    id: AgentId,
    connection: V2ConnectionTo<Agent>,
    initialize_request: acp::InitializeRequest,
    initialize_response: acp::InitializeResponse,
    agent_server_store: WeakEntity<AgentServerStore>,
    request_elicitations: Entity<ElicitationStore>,
    debug_log: AcpDebugLog,
    dispatch_tx: mpsc::UnboundedSender<ForegroundWork>,
    io_task: Option<Task<()>>,
    dispatch_task: Option<Task<()>>,
    shutdown: Option<futures::channel::oneshot::Sender<()>>,
    _stderr_task: Shared<Task<()>>,
    _wait_task: Option<Task<()>>,
    _child: Option<OwnedChild>,
}

impl Drop for AcpV2Connection {
    fn drop(&mut self) {
        // Let SDK shutdown finish its close callback before releasing the foreground owners.
        drop(self.shutdown.take());
        if let Some(task) = self.io_task.take() {
            task.detach();
        }
        if let Some(task) = self.dispatch_task.take() {
            task.detach();
        }
    }
}

impl AcpV2Connection {
    pub async fn stdio(
        agent_id: AgentId,
        project: Entity<Project>,
        command: AgentServerCommand,
        agent_server_store: WeakEntity<AgentServerStore>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let transport::StdioProcess {
            child,
            incoming,
            outgoing,
            stderr,
            debug_log,
        } = transport::spawn_stdio(&project, command, cx)?;
        let mut child = OwnedChild(child);
        let stderr_task = cx
            .background_spawn(async move {
                stderr.await.log_err();
            })
            .shared();
        let info = cx.update(|cx| {
            acp::Implementation::new("zed", release_channel::AppVersion::global(cx).to_string())
                .title(
                    release_channel::ReleaseChannel::try_global(cx)
                        .map(|channel| channel.display_name().to_owned()),
                )
        });
        let initialized = {
            let mut cx = cx.clone();
            let debug_log = debug_log.clone();
            let stderr_task = stderr_task.clone();
            async move {
                Self::connect(
                    agent_id,
                    agent_server_store,
                    Lines::new(outgoing, incoming),
                    info,
                    debug_log,
                    stderr_task,
                    &mut cx,
                )
                .await
            }
        }
        .boxed_local();
        let status = child
            .0
            .status()
            .map(|result| {
                result.map_err(|error| anyhow!("failed to wait for agent server exit: {error}"))
            })
            .boxed_local();
        let (mut connection, status) = match futures::future::select(initialized, status).await {
            futures::future::Either::Left((Ok(connection), status)) => (connection, status),
            futures::future::Either::Left((Err(error), status)) => {
                let timeout = cx
                    .background_executor()
                    .timer(Duration::from_millis(250))
                    .boxed_local();
                if let futures::future::Either::Left((status, _)) =
                    futures::future::select(status, timeout).await
                {
                    return Err(exited_load_error_after_drain(
                        status?,
                        stderr_task,
                        &debug_log,
                        cx,
                    )
                    .await
                    .into());
                }
                return Err(error);
            }
            futures::future::Either::Right((status, _)) => {
                return Err(
                    exited_load_error_after_drain(status?, stderr_task, &debug_log, cx)
                        .await
                        .into(),
                );
            }
        };
        let drained = {
            let sdk_connection = connection.connection.clone();
            let dispatch_tx = connection.dispatch_tx.clone();
            async move {
                sdk_connection.incoming_closed().await;
                drain_foreground_queue(&dispatch_tx).await.log_err();
                stderr_task.await;
            }
        };
        connection._wait_task = Some(cx.spawn(async move |cx| match status.await {
            Ok(status) => {
                let error = exited_load_error_after_drain(status, drained, &debug_log, cx).await;
                log::warn!("ACP v2 agent exited: {error}");
            }
            Err(error) => log::error!("ACP v2 process wait failed: {error}"),
        }));
        connection._child = Some(child);
        Ok(connection)
    }

    async fn connect(
        id: AgentId,
        agent_server_store: WeakEntity<AgentServerStore>,
        transport: impl ConnectTo<Client> + 'static,
        info: acp::Implementation,
        debug_log: AcpDebugLog,
        stderr_task: Shared<Task<()>>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let request_elicitations = cx.new(|_| ElicitationStore::default());
        let context = ClientContext {
            sessions: Rc::new(RefCell::new(HashMap::default())),
            session_list: Rc::new(RefCell::new(None)),
            request_elicitations: request_elicitations.clone(),
        };
        let (dispatch_tx, mut dispatch_rx) = mpsc::unbounded::<ForegroundWork>();
        let dispatch_task = cx.spawn(async move |cx| {
            while let Some(work) = dispatch_rx.next().await {
                work.run(cx, &context);
            }
        });
        let (sender, received) = futures::channel::oneshot::channel();
        let (shutdown, shutdown_received) = futures::channel::oneshot::channel();
        let io_task = cx.background_spawn({
            let dispatch_tx = dispatch_tx.clone();
            async move {
                let cleanup = cancel_elicitations_after_disconnect(dispatch_tx.clone())
                    .boxed()
                    .shared();
                let result = with_v2_client_ingress(Client.v2().name("zed-v2"), dispatch_tx)
                    .on_close({
                        let cleanup = cleanup.clone();
                        async move |_connection: V2ConnectionTo<Agent>| cleanup.await
                    })
                    .connect_with(transport, async move |connection| {
                        if sender.send(connection).is_err() {
                            return Err(agent_client_protocol::Error::internal_error()
                                .data("ACP v2 connection receiver dropped"));
                        }
                        if shutdown_received.await.is_err() {
                            log::debug!("ACP v2 connection owner dropped");
                        }
                        Ok(())
                    })
                    .await;
                if let Err(error) = cleanup.await {
                    log::warn!("ACP v2 disconnect cleanup failed: {error}");
                }
                result.log_err();
            }
        });
        let connection = received
            .await
            .context("Failed to receive ACP v2 connection handle")?;
        let supports_terminal_auth = agent_server_store.upgrade().is_some_and(|store| {
            store.update(cx, |store, _| store.get_external_agent(&id).is_some())
        });
        let mut capabilities = acp::ClientCapabilities::new().elicitation(
            acp::ElicitationCapabilities::new()
                .form(acp::ElicitationFormCapabilities::new())
                .url(acp::ElicitationUrlCapabilities::new()),
        );
        if supports_terminal_auth {
            capabilities = capabilities
                .auth(acp::AuthCapabilities::new().terminal(acp::TerminalAuthCapabilities::new()));
        }
        let initialize_request =
            acp::InitializeRequest::new(ProtocolVersion::V2, info).capabilities(capabilities);
        let initialize_response = connection
            .send_request(initialize_request.clone())
            .block_task()
            .await
            .map_err(map_acp_error)?;
        if initialize_response.protocol_version != ProtocolVersion::V2 {
            return Err(UnsupportedVersion.into());
        }
        Ok(Self {
            id,
            connection,
            initialize_request,
            initialize_response,
            agent_server_store,
            request_elicitations,
            debug_log,
            dispatch_tx,
            io_task: Some(io_task),
            dispatch_task: Some(dispatch_task),
            shutdown: Some(shutdown),
            _stderr_task: stderr_task,
            _wait_task: None,
            _child: None,
        })
    }

    pub fn initialize_request(&self) -> &acp::InitializeRequest {
        &self.initialize_request
    }

    pub fn initialize_response(&self) -> &acp::InitializeResponse {
        &self.initialize_response
    }

    pub fn request_elicitations(&self) -> Entity<ElicitationStore> {
        self.request_elicitations.clone()
    }

    pub fn subscribe_debug_messages(
        &self,
    ) -> (
        Vec<AcpDebugMessage>,
        async_channel::Receiver<AcpDebugMessage>,
    ) {
        self.debug_log.subscribe()
    }

    pub fn terminal_auth_task(
        &self,
        method_id: &acp::AuthMethodId,
        cx: &App,
    ) -> Option<Task<Result<SpawnInTerminal>>> {
        self.initialize_request
            .capabilities
            .auth
            .as_ref()?
            .terminal
            .as_ref()?;
        let method = self
            .initialize_response
            .auth_methods
            .iter()
            .find(|method| method.method_id() == method_id)?;
        match method {
            acp::AuthMethod::Terminal(terminal) => Some(first_class_terminal_auth_task(
                self.id.clone(),
                self.agent_server_store.clone(),
                terminal.clone(),
                cx,
            )),
            _ => None,
        }
    }

    pub fn login(
        &self,
        request: acp::LoginAuthRequest,
        cx: &mut App,
    ) -> Task<Result<acp::LoginAuthResponse>> {
        if !self.initialize_response.auth_methods.iter().any(|method| {
            matches!(method, acp::AuthMethod::Agent(agent) if agent.method_id == request.method_id)
        }) {
            return Task::ready(Err(anyhow!(
                "Authentication method is not supported for agent-managed login"
            )));
        }
        self.auth_request(request, cx)
    }

    pub fn supports_logout(&self) -> bool {
        !self.initialize_response.auth_methods.is_empty()
    }

    pub fn logout(
        &self,
        request: acp::LogoutAuthRequest,
        cx: &mut App,
    ) -> Task<Result<acp::LogoutAuthResponse>> {
        if !self.supports_logout() {
            return Task::ready(Err(anyhow!("Logout is not advertised by this agent")));
        }
        self.auth_request(request, cx)
    }

    fn auth_request<Request: JsonRpcRequest + 'static>(
        &self,
        request: Request,
        cx: &mut App,
    ) -> Task<Result<Request::Response>> {
        let connection = self.connection.clone();
        let dispatch_tx = self.dispatch_tx.clone();
        cx.foreground_executor().spawn(async move {
            let request = connection.prepare_request(request);
            let request_id = request.id().clone();
            let cancellation = request.cancellation_handle();
            let response = request.block_task();
            // Publication must precede caller-drop cancellation; early SDK cancellation is inert.
            let guard = util::defer(move || {
                cancellation.cancel().log_err();
                enqueue_notification(&dispatch_tx, request_id, cancel_auth_elicitations);
            });
            let response = response.await.map_err(map_acp_error)?;
            guard.abort();
            Ok(response)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use acp_thread::ElicitationStatus;
    use agent_client_protocol::{
        Responder,
        schema::v1::{self as acp_v1, ErrorCode},
    };
    use gpui::TestAppContext;
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    struct Harness {
        client: AcpV2Connection,
        agent: V2ConnectionTo<Client>,
        logins: Arc<Mutex<Vec<(acp::LoginAuthRequest, acp::RequestId)>>>,
        logouts: Arc<Mutex<Vec<acp::LogoutAuthRequest>>>,
        _agent_task: Task<Result<(), agent_client_protocol::Error>>,
    }

    fn meta(scope: &str) -> acp::Meta {
        acp::Meta::from_iter([("scope".to_owned(), json!(scope))])
    }

    async fn bounded<T>(future: impl std::future::Future<Output = T>, cx: &TestAppContext) -> T {
        let timeout = cx.background_executor.timer(Duration::from_secs(5));
        futures::pin_mut!(future, timeout);
        match futures::future::select(future, timeout).await {
            futures::future::Either::Left((result, _)) => result,
            futures::future::Either::Right(_) => panic!("timed out during v2 auth exchange"),
        }
    }

    impl Harness {
        async fn new(response: acp::InitializeResponse, cx: &mut TestAppContext) -> Result<Self> {
            let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();
            let logins = Arc::new(Mutex::new(Vec::new()));
            let logouts = Arc::new(Mutex::new(Vec::new()));
            let (sender, received) = futures::channel::oneshot::channel();
            let agent_task = cx.background_spawn(
                Agent
                    .v2()
                    .on_receive_request(
                        async move |request: acp::InitializeRequest,
                                    responder: Responder<acp::InitializeResponse>,
                                    _| {
                            assert_eq!(request.protocol_version, ProtocolVersion::V2);
                            assert!(request.capabilities.auth.is_none());
                            let elicitation = request
                                .capabilities
                                .elicitation
                                .expect("elicitation capability");
                            assert!(elicitation.form.is_some() && elicitation.url.is_some());
                            responder.respond(response.clone())
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        {
                            let logins = logins.clone();
                            async move |request: acp::LoginAuthRequest,
                                        responder: Responder<acp::LoginAuthResponse>,
                                        agent: V2ConnectionTo<Client>| {
                                let request_id = responder.id().clone();
                                logins
                                    .lock()
                                    .expect("login log")
                                    .push((request, request_id.clone()));
                                let elicitation = acp::CreateElicitationRequest::new(
                                    acp::ElicitationFormMode::new(
                                        acp::ElicitationRequestScope::new(request_id),
                                        acp::ElicitationSchema::new().string("name", true),
                                    ),
                                    "Auth input",
                                )
                                .meta(meta("elicitation"));
                                // Caller cleanup must work even when the peer ignores auth cancellation.
                                agent.prepare_request(elicitation).on_receiving_result(
                                    async move |result| match result {
                                        Ok(response) => {
                                            assert!(matches!(
                                                response.action,
                                                acp::ElicitationAction::Accept(_)
                                                    | acp::ElicitationAction::Cancel
                                            ));
                                            if matches!(
                                                response.action,
                                                acp::ElicitationAction::Accept(_)
                                            ) {
                                                assert_eq!(
                                                    response.meta,
                                                    Some(meta("elicitation-response"))
                                                );
                                            }
                                            responder.respond(
                                                acp::LoginAuthResponse::new()
                                                    .meta(meta("login-response")),
                                            )
                                        }
                                        Err(error) => responder.respond_with_error(error),
                                    },
                                )?;
                                Ok(())
                            }
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        {
                            let logouts = logouts.clone();
                            async move |request: acp::LogoutAuthRequest,
                                        responder: Responder<acp::LogoutAuthResponse>,
                                        _| {
                                logouts.lock().expect("logout log").push(request);
                                responder.respond(
                                    acp::LogoutAuthResponse::new().meta(meta("logout-response")),
                                )
                            }
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .connect_with(agent_transport, async move |agent| {
                        assert!(sender.send(agent).is_ok());
                        futures::future::pending::<Result<(), agent_client_protocol::Error>>().await
                    }),
            );
            let client = bounded(
                AcpV2Connection::connect(
                    AgentId::new("auth-test"),
                    WeakEntity::new_invalid(),
                    client_transport,
                    acp::Implementation::new("client-test", "1"),
                    AcpDebugLog::default(),
                    Task::ready(()).shared(),
                    &mut cx.to_async(),
                ),
                cx,
            )
            .await?;
            let agent = received.await.expect("agent handle");
            Ok(Self {
                client,
                agent,
                logins,
                logouts,
                _agent_task: agent_task,
            })
        }

        async fn registered(&self, cx: &mut TestAppContext) {
            let store = self.client.request_elicitations();
            let timeout = cx.background_executor.timer(Duration::from_secs(5));
            let update = store.next_event::<acp_thread::ElicitationStoreEvent>(cx);
            futures::pin_mut!(update, timeout);
            match futures::future::select(update, timeout).await {
                futures::future::Either::Left(_) => {}
                futures::future::Either::Right(_) => {
                    panic!("timed out registering auth elicitation")
                }
            }
        }
    }

    #[gpui::test]
    async fn v2_auth_sdk_preserves_offers_and_serves_elicitation_before_sessions(
        cx: &mut TestAppContext,
    ) {
        let offered: acp::InitializeResponse = serde_json::from_value(json!({
            "protocolVersion": 2, "info": {"name": "auth-agent", "version": "1",
                "_meta": {"scope": "info"}},
            "capabilities": {"session": {"_meta": {"scope": "session"}}, "_meta": {"scope": "caps"}},
            "authMethods": [
                {"type": "agent", "methodId": "login", "name": "Login",
                    "_meta": {"scope": "method", "terminal-auth": {"command": "not-executable"}}},
                {"type": "terminal", "methodId": "terminal", "name": "Terminal", "args": ["auth"]},
                {"type": "_future", "methodId": "future", "name": "Future",
                    "payload": {"retain": [null, 7]}}
            ], "_meta": {"scope": "initialize"}
        })).expect("offered initialization");
        let harness = Harness::new(offered.clone(), cx).await.expect("connect v2");
        assert_eq!(harness.client.initialize_response(), &offered);
        assert!(
            harness.client.supports_logout(),
            "auth methods, not capabilities.auth, enable logout"
        );
        assert!(
            cx.read(|cx| harness.client.terminal_auth_task(&"login".into(), cx))
                .is_none()
        );
        let login = cx.update(|cx| {
            harness.client.login(
                acp::LoginAuthRequest::new("login").meta(meta("login-request")),
                cx,
            )
        });
        harness.registered(cx).await;
        let store = harness.client.request_elicitations();
        let entry_id = store.read_with(cx, |store, _| {
            let [elicitation] = store.elicitations() else {
                panic!("one auth elicitation");
            };
            assert_eq!(elicitation.request.meta, Some(meta("elicitation")));
            let acp::ElicitationScope::Request(scope) = elicitation.request.scope() else {
                panic!("request scope without a session");
            };
            let logins = harness.logins.lock().expect("login log");
            assert_eq!(scope.request_id, logins.first().expect("login").1);
            assert_eq!(
                logins.first().expect("login").0.meta,
                Some(meta("login-request"))
            );
            elicitation.id.clone()
        });
        // Another RPC must be handled while login is waiting for foreground user input.
        let logout = cx.update(|cx| {
            harness.client.logout(
                acp::LogoutAuthRequest::new().meta(meta("logout-request")),
                cx,
            )
        });
        assert_eq!(
            bounded(logout, cx).await.expect("logout").meta,
            Some(meta("logout-response"))
        );
        store.update(cx, |store, cx| {
            store.respond_to_elicitation(
                &entry_id,
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Accept(
                    acp::ElicitationAcceptAction::new().content(std::collections::BTreeMap::from(
                        [("name".to_owned(), acp::ElicitationContentValue::from("Ada"))],
                    )),
                ))
                .meta(meta("elicitation-response")),
                cx,
            )
        });
        assert_eq!(
            bounded(login, cx).await.expect("login").meta,
            Some(meta("login-response"))
        );
        for method_id in ["terminal", "future", "missing"] {
            let login = cx.update(|cx| {
                harness
                    .client
                    .login(acp::LoginAuthRequest::new(method_id), cx)
            });
            assert!(bounded(login, cx).await.is_err());
        }
        assert_eq!(harness.logins.lock().expect("login log").len(), 1);
        assert_eq!(
            harness
                .logouts
                .lock()
                .expect("logout log")
                .first()
                .expect("logout")
                .meta,
            Some(meta("logout-request"))
        );
    }

    #[gpui::test]
    async fn v2_auth_sdk_caller_drop_and_eof_cleanup_only_their_owned_entries(
        cx: &mut TestAppContext,
    ) {
        let offered = acp::InitializeResponse::new(
            ProtocolVersion::V2,
            acp::Implementation::new("agent", "1"),
        )
        .auth_methods(vec![acp::AuthMethod::Agent(acp::AuthMethodAgent::new(
            "login", "Login",
        ))]);
        let harness = Harness::new(offered, cx).await.expect("connect");
        let first = cx.update(|cx| {
            harness
                .client
                .login(acp::LoginAuthRequest::new("login"), cx)
        });
        harness.registered(cx).await;
        let second = cx.update(|cx| {
            harness
                .client
                .login(acp::LoginAuthRequest::new("login"), cx)
        });
        harness.registered(cx).await;
        let store = harness.client.request_elicitations();
        let unrelated = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationRequestScope::new(acp::RequestId::Number(999)),
                            acp::ElicitationSchema::new(),
                        ),
                        "No active auth waiter",
                    ),
                    cx,
                )
                .expect("unrelated pending elicitation")
        });
        let (first_id, second_id) = store.read_with(cx, |store, _| {
            let [first, second, _] = store.elicitations() else {
                panic!("two auth entries and independent feedback");
            };
            (first.id.clone(), second.id.clone())
        });
        drop(first);
        cx.run_until_parked();
        store.read_with(cx, |store, _| {
            assert!(matches!(
                store.elicitation(&first_id).expect("first").1.status,
                ElicitationStatus::Canceled
            ));
            assert!(matches!(
                store.elicitation(&second_id).expect("second").1.status,
                ElicitationStatus::Pending { .. }
            ));
        });
        // The server goes away while the client and its displayed store are retained.
        drop(harness._agent_task);
        drop(harness.agent);
        let result = bounded(second, cx).await;
        assert!(result.is_err());
        bounded(harness.client.connection.incoming_closed(), cx).await;
        bounded(drain_foreground_queue(&harness.client.dispatch_tx), cx)
            .await
            .expect("foreground cleanup barrier");
        assert_eq!(
            bounded(unrelated, cx).await.action,
            acp::ElicitationAction::Cancel
        );
        store.read_with(cx, |store, _| {
            assert!(
                store
                    .elicitations()
                    .iter()
                    .all(|entry| matches!(entry.status, ElicitationStatus::Canceled))
            );
        });
    }

    #[gpui::test]
    async fn v2_auth_sdk_missing_methods_do_not_enable_auth_and_v1_is_rejected(
        cx: &mut TestAppContext,
    ) {
        let offered = acp::InitializeResponse::new(
            ProtocolVersion::V2,
            acp::Implementation::new("agent", "1"),
        )
        .capabilities(acp::AgentCapabilities::new().auth(acp::AgentAuthCapabilities::new()));
        let harness = Harness::new(offered, cx).await.expect("connect");
        assert!(!harness.client.supports_logout());
        let login = cx.update(|cx| {
            harness
                .client
                .login(acp::LoginAuthRequest::new("missing"), cx)
        });
        let logout = cx.update(|cx| harness.client.logout(acp::LogoutAuthRequest::new(), cx));
        assert!(bounded(login, cx).await.is_err());
        assert!(bounded(logout, cx).await.is_err());
        assert!(harness.logins.lock().expect("login log").is_empty());
        assert!(harness.logouts.lock().expect("logout log").is_empty());
        let store = harness.client.request_elicitations();
        let pending = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationRequestScope::new(acp::RequestId::Number(999)),
                            acp::ElicitationSchema::new(),
                        ),
                        "Retained across connection destruction",
                    ),
                    cx,
                )
                .expect("retained pending entry")
        });
        drop(harness.client);
        assert_eq!(
            bounded(pending, cx).await.action,
            acp::ElicitationAction::Cancel
        );
        store.read_with(cx, |store, _| {
            assert!(
                store
                    .elicitations()
                    .iter()
                    .all(|entry| matches!(entry.status, ElicitationStatus::Canceled))
            );
        });
        let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();
        // A v2 SDK server rewrites replies to its negotiated version; use a real v1 peer.
        let _agent_task = cx.background_spawn(
            Agent
                .builder()
                .on_receive_request(
                    async move |_request: acp_v1::InitializeRequest,
                                responder: Responder<acp_v1::InitializeResponse>,
                                _| {
                        responder.respond(acp_v1::InitializeResponse::new(ProtocolVersion::V1))
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .connect_with(agent_transport, async move |_agent| {
                    futures::future::pending::<Result<(), agent_client_protocol::Error>>().await
                }),
        );
        let error = bounded(
            AcpV2Connection::connect(
                AgentId::new("v1-only"),
                WeakEntity::new_invalid(),
                client_transport,
                acp::Implementation::new("client", "1"),
                AcpDebugLog::default(),
                Task::ready(()).shared(),
                &mut cx.to_async(),
            ),
            cx,
        )
        .await
        .err()
        .expect("reject unimplemented fallback");
        assert!(error.to_string().contains("protocol"), "{error}");
    }

    #[gpui::test]
    async fn v2_auth_sdk_terminal_methods_reuse_the_configured_invocation(cx: &mut TestAppContext) {
        use project::agent_server_store::AllAgentServersSettings;
        use settings::Settings as _;
        cx.update(|cx| {
            let settings = settings::SettingsStore::test(cx);
            cx.set_global(settings);
            AllAgentServersSettings::override_global(
                AllAgentServersSettings(HashMap::from_iter([(
                    "terminal-test".to_owned(),
                    settings::CustomAgentServerSettings::Custom {
                        path: "/bin/sh".into(),
                        args: vec!["--base".into()],
                        env: HashMap::from_iter([("BASE".into(), "1".into())]),
                        default_mode: None,
                        default_config_options: HashMap::default(),
                        favorite_config_option_values: HashMap::default(),
                    }
                    .into(),
                )])),
                cx,
            );
        });
        let fs = fs::FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let store = project.read_with(cx, |project, _| project.agent_server_store().downgrade());
        let method: acp::AuthMethod = serde_json::from_value(json!({
            "type": "terminal", "methodId": "terminal", "name": "Interactive login",
            "args": ["auth", ""], "env": [{"name": "EXTRA", "value": "2",
                "_meta": {"retain": [null, 7]}}], "_meta": {"scope": "terminal"}
        }))
        .expect("terminal method");
        let expected = method.clone();
        let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();
        let _agent_task = cx.background_spawn(
            Agent
                .v2()
                .on_receive_request(
                    async move |request: acp::InitializeRequest,
                                responder: Responder<acp::InitializeResponse>,
                                _| {
                        assert!(
                            request
                                .capabilities
                                .auth
                                .as_ref()
                                .and_then(|auth| auth.terminal.as_ref())
                                .is_some()
                        );
                        responder.respond(
                            acp::InitializeResponse::new(
                                ProtocolVersion::V2,
                                acp::Implementation::new("terminal-agent", "1"),
                            )
                            .auth_methods(vec![method.clone()]),
                        )
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .connect_with(agent_transport, async move |_agent| {
                    futures::future::pending::<Result<(), agent_client_protocol::Error>>().await
                }),
        );
        let client = bounded(
            AcpV2Connection::connect(
                AgentId::new("terminal-test"),
                store,
                client_transport,
                acp::Implementation::new("test", "1"),
                AcpDebugLog::default(),
                Task::ready(()).shared(),
                &mut cx.to_async(),
            ),
            cx,
        )
        .await
        .expect("connect terminal-capable client");
        assert_eq!(client.initialize_response.auth_methods, [expected]);
        assert!(client.supports_logout());
        let login = cx.update(|cx| client.login(acp::LoginAuthRequest::new("terminal"), cx));
        assert!(bounded(login, cx).await.is_err());
        let task = cx
            .read(|cx| client.terminal_auth_task(&"terminal".into(), cx))
            .expect("separate terminal route");
        let task = bounded(task, cx)
            .await
            .expect("configured terminal invocation");
        assert_eq!(task.command.as_deref(), Some("/bin/sh"));
        assert_eq!(task.args, ["--base", "auth", ""]);
        assert_eq!(task.env.get("BASE").map(String::as_str), Some("1"));
        assert_eq!(task.env.get("EXTRA").map(String::as_str), Some("2"));
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn v2_auth_stdio_uses_one_launcher_and_preserves_errors_and_exit_stderr(
        cx: &mut TestAppContext,
    ) {
        cx.update(|cx| {
            let settings = settings::SettingsStore::test(cx);
            cx.set_global(settings);
            release_channel::init("0.0.0".parse().expect("version"), cx);
        });
        cx.executor().allow_parking();
        let directory = tempfile::tempdir().expect("working directory");
        let project = Project::example([directory.path()], &mut cx.to_async()).await;
        let command = AgentServerCommand {
            path: "/bin/sh".into(),
            args: vec!["-c".into(), r#"
while IFS= read -r request; do
    id=${request#*\"id\":}
    id=${id%%,*}
    case "$request" in
        *'"method":"initialize"'*)
            case "$request" in *'"protocolVersion":2'*) ;; *) exit 42 ;; esac
            printf '{"jsonrpc":"2.0","id":%s,"result":{"protocolVersion":2,"info":{"name":"stdio-agent","version":"1"},"authMethods":[{"type":"agent","methodId":"login","name":"Login"}]}}\n' "$id"
            ;;
        *'"method":"auth/login"'*)
            printf '{"jsonrpc":"2.0","id":%s,"error":{"code":-32602,"message":"login rejected","data":{"scope":"wire-error"}}}\n' "$id"
            ;;
        *'"method":"auth/logout"'*)
            printf '{"jsonrpc":"2.0","id":%s,"result":{"_meta":{"scope":"logout-response"}}}\n' "$id"
            printf 'foundation final stderr\n' >&2
            exit 0
            ;;
        *) exit 43 ;;
    esac
done
"#.into()],
            env: None,
        };
        let client = bounded(
            AcpV2Connection::stdio(
                AgentId::new("stdio-test"),
                project.clone(),
                command,
                WeakEntity::new_invalid(),
                &mut cx.to_async(),
            ),
            cx,
        )
        .await
        .expect("initialize scripted stdio");
        assert_eq!(client.initialize_response.info.name, "stdio-agent");
        let login = cx.update(|cx| client.login(acp::LoginAuthRequest::new("login"), cx));
        let error = bounded(login, cx).await.expect_err("wire login rejection");
        let error = error
            .downcast_ref::<agent_client_protocol::Error>()
            .expect("runtime error");
        assert_eq!(error.code, ErrorCode::InvalidParams);
        assert_eq!(error.data, Some(json!({"scope": "wire-error"})));
        let logout = cx.update(|cx| client.logout(acp::LogoutAuthRequest::new(), cx));
        assert_eq!(
            bounded(logout, cx).await.expect("logout").meta,
            Some(meta("logout-response"))
        );
        bounded(client.connection.incoming_closed(), cx).await;
        let (messages, _receiver) = client.subscribe_debug_messages();
        for method_name in ["initialize", "auth/login", "auth/logout"] {
            assert_eq!(messages.iter().filter(|message| {
                matches!(&message.message, super::super::AcpDebugMessageContent::Request { method, .. }
                    if method.as_ref() == method_name)
            }).count(), 1, "one wire request per method");
        }
        drop(client);

        let command = AgentServerCommand {
            path: "/bin/sh".into(),
            args: vec![
                "-c".into(),
                "printf 'initialize exit stderr\\n' >&2; exit 7".into(),
            ],
            env: None,
        };
        let error = bounded(
            AcpV2Connection::stdio(
                AgentId::new("early-exit"),
                project,
                command,
                WeakEntity::new_invalid(),
                &mut cx.to_async(),
            ),
            cx,
        )
        .await
        .err()
        .expect("initialization EOF");
        let acp_thread::LoadError::Exited { status, stderr } = error
            .downcast_ref::<acp_thread::LoadError>()
            .expect("process exit error")
        else {
            panic!("exited error");
        };
        assert_eq!(status.code(), Some(7));
        assert_eq!(stderr.as_deref(), Some("initialize exit stderr"));
    }
}
