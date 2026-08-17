#![cfg(feature = "unstable_protocol_v2")]

use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};

use agent_client_protocol::schema::{ProtocolVersion, v1, v2};
use agent_client_protocol::{
    Agent, AgentProtocolRouter, Builder, ByteStreams, Client, ClientProtocolConnector, ConnectTo,
    Error, JsonRpcMessage, JsonRpcRequest, JsonRpcResponse, NullHandler, RawJsonRpcMessage, Role,
    TransportFrame, UntypedRole,
};
use agent_client_protocol_test::testy::Testy;
use futures::StreamExt as _;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

#[derive(Debug, Clone, Serialize, Deserialize, JsonRpcRequest)]
#[request(method = "initialize", response = ForeignInitializeResponse)]
struct ForeignInitializeRequest {
    #[serde(rename = "protocolVersion")]
    protocol_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonRpcResponse)]
struct ForeignInitializeResponse {
    #[serde(rename = "protocolVersion")]
    protocol_version: String,
}

struct ForeignPeer;

impl ConnectTo<UntypedRole> for ForeignPeer {
    async fn connect_to(self, client: impl ConnectTo<UntypedRole>) -> Result<(), Error> {
        UntypedRole
            .builder()
            .on_receive_request(
                async |request: ForeignInitializeRequest, responder, _cx| {
                    assert_eq!(request.protocol_version, "2025-06-18");
                    responder.respond(ForeignInitializeResponse {
                        protocol_version: request.protocol_version,
                    })
                },
                agent_client_protocol::on_receive_request!(),
            )
            .connect_to(client)
            .await
    }
}

fn cwd() -> Result<PathBuf, Error> {
    std::env::current_dir().map_err(Error::into_internal_error)
}

fn v2_implementation() -> v2::Implementation {
    v2::Implementation::new("agent-client-protocol-test", env!("CARGO_PKG_VERSION"))
}

fn v1_implementation() -> v1::Implementation {
    v1::Implementation::new("agent-client-protocol-test", env!("CARGO_PKG_VERSION"))
}

fn v1_initialize_request(protocol_version: ProtocolVersion) -> v1::InitializeRequest {
    v1::InitializeRequest::new(protocol_version).client_info(v1_implementation())
}

fn v2_initialize_request(protocol_version: ProtocolVersion) -> v2::InitializeRequest {
    v2::InitializeRequest::new(protocol_version, v2_implementation())
}

fn v2_initialize_response_with_session(
    protocol_version: ProtocolVersion,
) -> v2::InitializeResponse {
    v2::InitializeResponse::new(protocol_version, v2_implementation())
        .capabilities(v2::AgentCapabilities::new().session(v2::SessionCapabilities::new()))
}

fn json_value(value: impl Serialize) -> Result<Value, Error> {
    serde_json::to_value(value).map_err(Error::into_internal_error)
}

async fn write_wire_json(
    writer: &mut (impl tokio::io::AsyncWrite + Unpin),
    value: &Value,
) -> Result<(), Error> {
    use tokio::io::AsyncWriteExt as _;

    let mut bytes = serde_json::to_vec(value).map_err(Error::into_internal_error)?;
    bytes.push(b'\n');
    writer
        .write_all(&bytes)
        .await
        .map_err(Error::into_internal_error)?;
    writer.flush().await.map_err(Error::into_internal_error)
}

async fn read_wire_json(
    reader: &mut (impl tokio::io::AsyncBufRead + Unpin),
) -> Result<Value, Error> {
    use tokio::io::AsyncBufReadExt as _;

    let mut line = String::new();
    let bytes_read = tokio::time::timeout(
        tokio::time::Duration::from_secs(10),
        reader.read_line(&mut line),
    )
    .await
    .map_err(Error::into_internal_error)?
    .map_err(Error::into_internal_error)?;
    if bytes_read == 0 {
        return Err(Error::internal_error().data("wire stream closed before the next JSON value"));
    }
    serde_json::from_str(line.trim()).map_err(Error::into_internal_error)
}

fn runtime_flag_protocol_router(enable_protocol_v2: bool) -> AgentProtocolRouter {
    let v1_agent = Agent.builder().on_receive_request(
        async |initialize: v1::InitializeRequest, responder, _cx| {
            assert_eq!(initialize.protocol_version, ProtocolVersion::V1);
            responder.respond(v1::InitializeResponse::new(initialize.protocol_version))
        },
        agent_client_protocol::on_receive_request!(),
    );

    let agent = Agent.protocol_router().with_v1(v1_agent);

    if enable_protocol_v2 {
        let v2_agent = Agent.v2().on_receive_request(
            async |initialize: v2::InitializeRequest, responder, _cx| {
                assert_eq!(initialize.protocol_version, ProtocolVersion::V2);
                responder.respond(v2_initialize_response_with_session(
                    initialize.protocol_version,
                ))
            },
            agent_client_protocol::on_receive_request!(),
        );

        agent.with_v2(v2_agent)
    } else {
        agent
    }
}

fn runtime_flag_client_protocol_connector(enable_protocol_v2: bool) -> ClientProtocolConnector {
    let client = Client
        .protocol_connector()
        .with_v1(|| InitializingV1Client::new("v1-client-connector-session"));

    if enable_protocol_v2 {
        client.with_v2(|| InitializingV2Client::new("v2-client-connector-session"))
    } else {
        client
    }
}

struct InitializingV1Client {
    expected_session_id: &'static str,
    implementation_name: &'static str,
    client_capabilities: Option<v1::ClientCapabilities>,
    previous_clients_dropped: Vec<Arc<AtomicBool>>,
}

impl InitializingV1Client {
    fn new(expected_session_id: &'static str) -> Self {
        Self {
            expected_session_id,
            implementation_name: "agent-client-protocol-test",
            client_capabilities: None,
            previous_clients_dropped: Vec::new(),
        }
    }

    fn with_implementation_name(
        expected_session_id: &'static str,
        implementation_name: &'static str,
    ) -> Self {
        Self {
            expected_session_id,
            implementation_name,
            client_capabilities: None,
            previous_clients_dropped: Vec::new(),
        }
    }

    fn with_client_capabilities(mut self, client_capabilities: v1::ClientCapabilities) -> Self {
        self.client_capabilities = Some(client_capabilities);
        self
    }

    fn expect_previous_client_dropped(mut self, dropped: Arc<AtomicBool>) -> Self {
        self.previous_clients_dropped.push(dropped);
        self
    }
}

impl ConnectTo<Agent> for InitializingV1Client {
    async fn connect_to(self, agent: impl ConnectTo<Client>) -> Result<(), Error> {
        let expected_session_id = self.expected_session_id;
        let implementation_name = self.implementation_name;
        let client_capabilities = self.client_capabilities;
        let previous_clients_dropped = self.previous_clients_dropped;
        Client
            .builder()
            .connect_with(agent, async move |cx| {
                let mut request = v1::InitializeRequest::new(ProtocolVersion::V1).client_info(
                    v1::Implementation::new(implementation_name, env!("CARGO_PKG_VERSION")),
                );
                if let Some(client_capabilities) = client_capabilities {
                    request = request.client_capabilities(client_capabilities);
                }

                let initialize = cx.send_request(request).block_task().await?;
                assert_eq!(initialize.protocol_version, ProtocolVersion::V1);
                for dropped in previous_clients_dropped {
                    assert!(
                        dropped.load(Ordering::SeqCst),
                        "each abandoned protocol client must be dropped before v1 continues"
                    );
                }

                let session = cx
                    .send_request(v1::NewSessionRequest::new(cwd()?))
                    .block_task()
                    .await?;
                assert_eq!(session.session_id.0.as_ref(), expected_session_id);
                Ok(())
            })
            .await
    }
}

struct RejectingV1Client;

impl ConnectTo<Agent> for RejectingV1Client {
    async fn connect_to(self, _agent: impl ConnectTo<Client>) -> Result<(), Error> {
        Err(Error::internal_error().data("v1 client fallback should not run"))
    }
}

struct ExpectingV2InitializeErrorClient;

impl ConnectTo<Agent> for ExpectingV2InitializeErrorClient {
    async fn connect_to(self, agent: impl ConnectTo<Client>) -> Result<(), Error> {
        Client
            .v2()
            .connect_with(agent, async |cx| {
                let error = cx
                    .send_request(v2_initialize_request(ProtocolVersion::V1))
                    .block_task()
                    .await
                    .expect_err("v2 initialize rejection should be surfaced");
                let data = error
                    .data
                    .as_ref()
                    .and_then(|data| data.as_str())
                    .unwrap_or_default();
                assert!(
                    data.contains("only supports ACP protocol version 1"),
                    "{error:?}"
                );
                Ok(())
            })
            .await
    }
}

struct InitializingV2Client {
    expected_session_id: &'static str,
}

struct DropTrackedClient<C> {
    client: C,
    dropped: Arc<AtomicBool>,
}

impl<C> DropTrackedClient<C> {
    fn new(client: C, dropped: Arc<AtomicBool>) -> Self {
        Self { client, dropped }
    }
}

struct DropFlag(Arc<AtomicBool>);

impl Drop for DropFlag {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

impl<C: ConnectTo<Agent>> ConnectTo<Agent> for DropTrackedClient<C> {
    async fn connect_to(self, agent: impl ConnectTo<Client>) -> Result<(), Error> {
        let Self { client, dropped } = self;
        let _drop_flag = DropFlag(dropped);
        client.connect_to(agent).await
    }
}

impl InitializingV2Client {
    fn new(expected_session_id: &'static str) -> Self {
        Self {
            expected_session_id,
        }
    }
}

impl ConnectTo<Agent> for InitializingV2Client {
    async fn connect_to(self, agent: impl ConnectTo<Client>) -> Result<(), Error> {
        let expected_session_id = self.expected_session_id;
        Client
            .v2()
            .connect_with(agent, async move |cx| {
                let initialize = cx
                    .send_request(v2_initialize_request(ProtocolVersion::V1))
                    .block_task()
                    .await?;
                assert_eq!(initialize.protocol_version, ProtocolVersion::V2);

                let session = cx
                    .send_request(v2::NewSessionRequest::new(cwd()?))
                    .block_task()
                    .await?;
                assert_eq!(session.session_id.0.as_ref(), expected_session_id);
                Ok(())
            })
            .await
    }
}

fn v1_agent_with_session(session_id: &'static str) -> impl ConnectTo<Client> {
    Agent
        .builder()
        .on_receive_request(
            async |initialize: v1::InitializeRequest, responder, _cx| {
                assert_eq!(initialize.protocol_version, ProtocolVersion::V1);
                responder.respond(v1::InitializeResponse::new(initialize.protocol_version))
            },
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            async move |request: v1::NewSessionRequest, responder, _cx| {
                assert!(request.cwd.is_absolute());
                responder.respond(v1::NewSessionResponse::new(v1::SessionId::new(session_id)))
            },
            agent_client_protocol::on_receive_request!(),
        )
}

fn v2_agent_with_session(session_id: &'static str) -> impl ConnectTo<Client> {
    Agent
        .v2()
        .on_receive_request(
            async |initialize: v2::InitializeRequest, responder, _cx| {
                assert_eq!(initialize.protocol_version, ProtocolVersion::V2);
                responder.respond(v2_initialize_response_with_session(
                    initialize.protocol_version,
                ))
            },
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            async move |request: v2::NewSessionRequest, responder, _cx| {
                assert!(AsRef::<std::path::Path>::as_ref(&request.cwd).is_absolute());
                responder.respond(v2::NewSessionResponse::new(v2::SessionId::new(session_id)))
            },
            agent_client_protocol::on_receive_request!(),
        )
}

async fn assert_malformed_initialize_rejected(params: Map<String, Value>) -> Result<(), Error> {
    let agent = Agent.v2().on_receive_request(
        async |_initialize: v2::InitializeRequest, responder, _cx| {
            responder.respond_with_internal_error("handler should not run")
        },
        agent_client_protocol::on_receive_request!(),
    );
    let (mut channel, agent_future) = ConnectTo::<Client>::into_channel_and_future(agent);
    let agent_task = tokio::spawn(agent_future);

    channel
        .tx
        .unbounded_send(TransportFrame::Single(RawJsonRpcMessage::request(
            "initialize".into(),
            Value::Object(params),
            v1::RequestId::Number(1),
        )?))
        .map_err(Error::into_internal_error)?;

    while let Some(message) = channel.rx.next().await {
        let TransportFrame::Single(message) = message else {
            continue;
        };
        let RawJsonRpcMessage::Response(response) = message else {
            continue;
        };
        let v1::Response::Error { error, .. } = response else {
            panic!("malformed initialize should fail");
        };
        assert_eq!(error.code, agent_client_protocol::ErrorCode::InvalidParams);
        let data = error
            .data
            .as_ref()
            .and_then(|data| data.as_str())
            .unwrap_or_default();
        assert!(data.contains("protocolVersion"), "{error:?}");
        agent_task.abort();
        return Ok(());
    }

    agent_task.abort();
    Err(agent_client_protocol::util::internal_error(
        "agent did not respond to malformed initialize",
    ))
}

async fn assert_v2_client_rejected_by_v1_agent(agent: impl ConnectTo<Client>) -> Result<(), Error> {
    Client
        .v2()
        .connect_with(agent, async |cx| {
            let error = cx
                .send_request(v2_initialize_request(ProtocolVersion::V2))
                .block_task()
                .await
                .expect_err("v1 agent protocol mode should reject v2 clients");
            let data = error
                .data
                .as_ref()
                .and_then(|data| data.as_str())
                .unwrap_or_default();
            assert!(
                data.contains("only supports ACP protocol version 1"),
                "{error:?}"
            );
            Ok(())
        })
        .await
}

#[tokio::test(flavor = "current_thread")]
async fn non_acp_initialize_is_not_rewritten() -> Result<(), Error> {
    UntypedRole
        .builder()
        .connect_with(ForeignPeer, async |cx| {
            let response = cx
                .send_request(ForeignInitializeRequest {
                    protocol_version: "2025-06-18".into(),
                })
                .block_task()
                .await?;

            assert_eq!(response.protocol_version, "2025-06-18");
            Ok(())
        })
        .await
}

#[tokio::test(flavor = "current_thread")]
async fn v2_agent_rejects_initialize_without_protocol_version() -> Result<(), Error> {
    assert_malformed_initialize_rejected(Map::new()).await
}

#[tokio::test(flavor = "current_thread")]
async fn v2_agent_rejects_initialize_with_malformed_protocol_version() -> Result<(), Error> {
    let mut params = Map::new();
    params.insert("protocolVersion".into(), serde_json::json!(100_000));

    assert_malformed_initialize_rejected(params).await
}

#[tokio::test(flavor = "current_thread")]
async fn role_builder_v1_agent_rejects_v2_client_negotiation() -> Result<(), Error> {
    let agent = <Agent as Role>::builder(Agent).on_receive_request(
        async |initialize: v1::InitializeRequest, responder, _cx| {
            assert_eq!(initialize.protocol_version, ProtocolVersion::V1);
            responder.respond(v1::InitializeResponse::new(initialize.protocol_version))
        },
        agent_client_protocol::on_receive_request!(),
    );

    assert_v2_client_rejected_by_v1_agent(agent).await
}

#[tokio::test(flavor = "current_thread")]
async fn builder_new_v1_agent_rejects_v2_client_negotiation() -> Result<(), Error> {
    let agent = Builder::new(Agent).on_receive_request(
        async |initialize: v1::InitializeRequest, responder, _cx| {
            assert_eq!(initialize.protocol_version, ProtocolVersion::V1);
            responder.respond(v1::InitializeResponse::new(initialize.protocol_version))
        },
        agent_client_protocol::on_receive_request!(),
    );

    assert_v2_client_rejected_by_v1_agent(agent).await
}

#[tokio::test(flavor = "current_thread")]
async fn builder_new_with_v1_agent_rejects_v2_client_negotiation() -> Result<(), Error> {
    let agent = Builder::new_with(Agent, NullHandler).on_receive_request(
        async |initialize: v1::InitializeRequest, responder, _cx| {
            assert_eq!(initialize.protocol_version, ProtocolVersion::V1);
            responder.respond(v1::InitializeResponse::new(initialize.protocol_version))
        },
        agent_client_protocol::on_receive_request!(),
    );

    assert_v2_client_rejected_by_v1_agent(agent).await
}

#[tokio::test(flavor = "current_thread")]
async fn role_builder_v1_client_is_rejected_by_v2_agent() -> Result<(), Error> {
    let agent = Agent.v2().on_receive_request(
        async |_initialize: v2::InitializeRequest, responder, _cx| {
            responder.respond_with_internal_error("handler should not run")
        },
        agent_client_protocol::on_receive_request!(),
    );

    <Client as Role>::builder(Client)
        .connect_with(agent, async |cx| {
            let error = cx
                .send_request(v1_initialize_request(ProtocolVersion::V1))
                .block_task()
                .await
                .expect_err("v2 agents require a v2 client implementation");
            let data = error
                .data
                .as_ref()
                .and_then(|data| data.as_str())
                .unwrap_or_default();
            assert!(
                data.contains("only supports ACP protocol version 2"),
                "{error:?}"
            );
            Ok(())
        })
        .await
}

#[test]
fn v2_extension_enum_parsing_preserves_method_prefix() -> Result<(), Error> {
    let params = serde_json::json!({ "payload": true });

    let request = v2::ClientRequest::parse_message("_vendor/request", &params)?;
    assert_eq!(request.method(), "_vendor/request");
    let untyped_request = request.to_untyped_message()?;
    assert_eq!(untyped_request.method(), "_vendor/request");
    assert_eq!(untyped_request.params(), &params);

    let notification = v2::AgentNotification::parse_message("_vendor/notify", &params)?;
    assert_eq!(notification.method(), "_vendor/notify");
    let untyped_notification = notification.to_untyped_message()?;
    assert_eq!(untyped_notification.method(), "_vendor/notify");
    assert_eq!(untyped_notification.params(), &params);

    Ok(())
}

#[test]
fn v2_schema_1_4_method_names_are_jsonrpc_mapped() -> Result<(), Error> {
    fn assert_request<Req: JsonRpcRequest>() {}
    fn assert_notification<Notif: agent_client_protocol::JsonRpcNotification>() {}

    assert_request::<v2::LoginAuthRequest>();
    assert_request::<v2::LogoutAuthRequest>();
    assert_notification::<v2::CancelRequestNotification>();
    assert_notification::<v2::CancelSessionNotification>();
    assert_notification::<v2::UpdateSessionNotification>();

    let login_params = serde_json::json!({ "methodId": "browser" });
    let login = v2::LoginAuthRequest::parse_message("auth/login", &login_params)?;
    assert_eq!(login.method(), "auth/login");
    let client_request = v2::ClientRequest::parse_message("auth/login", &login_params)?;
    assert!(matches!(
        client_request,
        v2::ClientRequest::LoginAuthRequest(_)
    ));
    let login_response = v2::AgentResponse::from_value("auth/login", serde_json::json!({}))?;
    assert!(matches!(
        login_response,
        v2::AgentResponse::LoginAuthResponse(_)
    ));

    let logout = v2::LogoutAuthRequest::parse_message("auth/logout", &serde_json::json!({}))?;
    assert_eq!(logout.method(), "auth/logout");
    let client_request = v2::ClientRequest::parse_message("auth/logout", &serde_json::json!({}))?;
    assert!(matches!(
        client_request,
        v2::ClientRequest::LogoutAuthRequest(_)
    ));
    let logout_response = v2::AgentResponse::from_value("auth/logout", serde_json::json!({}))?;
    assert!(matches!(
        logout_response,
        v2::AgentResponse::LogoutAuthResponse(_)
    ));

    let cancel_params = serde_json::json!({ "requestId": "req-1" });
    let cancel = v2::CancelRequestNotification::parse_message("$/cancel_request", &cancel_params)?;
    assert_eq!(cancel.method(), "$/cancel_request");
    let protocol_notification =
        v2::ProtocolLevelNotification::parse_message("$/cancel_request", &cancel_params)?;
    assert!(matches!(
        protocol_notification,
        v2::ProtocolLevelNotification::CancelRequestNotification(_)
    ));

    let session_cancel_params = serde_json::json!({ "sessionId": "session-1" });
    let session_cancel =
        v2::CancelSessionNotification::parse_message("session/cancel", &session_cancel_params)?;
    assert_eq!(session_cancel.method(), "session/cancel");
    let client_notification =
        v2::ClientNotification::parse_message("session/cancel", &session_cancel_params)?;
    assert!(matches!(
        client_notification,
        v2::ClientNotification::CancelSessionNotification(_)
    ));

    let update_params = serde_json::json!({
        "sessionId": "session-1",
        "update": { "sessionUpdate": "_custom" }
    });
    let update = v2::UpdateSessionNotification::parse_message("session/update", &update_params)?;
    assert_eq!(update.method(), "session/update");
    let agent_notification =
        v2::AgentNotification::parse_message("session/update", &update_params)?;
    assert!(matches!(
        agent_notification,
        v2::AgentNotification::UpdateSessionNotification(_)
    ));

    Ok(())
}

#[cfg(feature = "unstable_mcp_over_acp")]
#[test]
fn mcp_over_acp_variants_are_jsonrpc_mapped() -> Result<(), Error> {
    fn assert_request<Req: JsonRpcRequest>() {}
    fn assert_notification<Notif: agent_client_protocol::JsonRpcNotification>() {}

    macro_rules! assert_message_mapping {
        ($ty:ty, $method:literal, $params:expr, $pattern:pat) => {{
            let message = <$ty as JsonRpcMessage>::parse_message($method, &$params)?;
            assert_eq!(message.method(), $method);
            assert_eq!(message.to_untyped_message()?.method(), $method);
            assert!(matches!(message, $pattern));
        }};
    }

    macro_rules! assert_response_mapping {
        ($ty:ty, $method:literal, $value:expr, $pattern:pat) => {{
            let response = <$ty as JsonRpcResponse>::from_value($method, $value)?;
            assert!(matches!(response, $pattern));
        }};
    }

    assert_request::<v2::ConnectMcpRequest>();
    assert_request::<v2::MessageMcpRequest>();
    assert_request::<v2::DisconnectMcpRequest>();
    assert_notification::<v2::MessageMcpNotification>();

    assert_message_mapping!(
        v1::ClientRequest,
        "mcp/message",
        json_value(v1::MessageMcpRequest::new("conn-1", "tools/list"))?,
        v1::ClientRequest::MessageMcpRequest(_)
    );
    assert_response_mapping!(
        v1::AgentResponse,
        "mcp/message",
        serde_json::json!({ "tools": [] }),
        v1::AgentResponse::MessageMcpResponse(_)
    );
    assert_message_mapping!(
        v1::ClientNotification,
        "mcp/message",
        json_value(v1::MessageMcpNotification::new(
            "conn-1",
            "notifications/tools/list"
        ))?,
        v1::ClientNotification::MessageMcpNotification(_)
    );
    assert_message_mapping!(
        v1::AgentRequest,
        "mcp/connect",
        json_value(v1::ConnectMcpRequest::new("server-1"))?,
        v1::AgentRequest::ConnectMcpRequest(_)
    );
    assert_message_mapping!(
        v1::AgentRequest,
        "mcp/message",
        json_value(v1::MessageMcpRequest::new("conn-1", "tools/list"))?,
        v1::AgentRequest::MessageMcpRequest(_)
    );
    assert_message_mapping!(
        v1::AgentRequest,
        "mcp/disconnect",
        json_value(v1::DisconnectMcpRequest::new("conn-1"))?,
        v1::AgentRequest::DisconnectMcpRequest(_)
    );
    assert_response_mapping!(
        v1::ClientResponse,
        "mcp/connect",
        json_value(v1::ConnectMcpResponse::new("conn-1"))?,
        v1::ClientResponse::ConnectMcpResponse(_)
    );
    assert_response_mapping!(
        v1::ClientResponse,
        "mcp/message",
        serde_json::json!({ "tools": [] }),
        v1::ClientResponse::MessageMcpResponse(_)
    );
    assert_response_mapping!(
        v1::ClientResponse,
        "mcp/disconnect",
        serde_json::json!({}),
        v1::ClientResponse::DisconnectMcpResponse(_)
    );
    assert_message_mapping!(
        v1::AgentNotification,
        "mcp/message",
        json_value(v1::MessageMcpNotification::new(
            "conn-1",
            "notifications/tools/list"
        ))?,
        v1::AgentNotification::MessageMcpNotification(_)
    );

    assert_message_mapping!(
        v2::MessageMcpRequest,
        "mcp/message",
        json_value(v2::MessageMcpRequest::new("conn-1", "tools/list"))?,
        v2::MessageMcpRequest { .. }
    );
    assert_message_mapping!(
        v2::MessageMcpNotification,
        "mcp/message",
        json_value(v2::MessageMcpNotification::new(
            "conn-1",
            "notifications/tools/list"
        ))?,
        v2::MessageMcpNotification { .. }
    );
    assert_message_mapping!(
        v2::ConnectMcpRequest,
        "mcp/connect",
        json_value(v2::ConnectMcpRequest::new("server-1"))?,
        v2::ConnectMcpRequest { .. }
    );
    assert_message_mapping!(
        v2::DisconnectMcpRequest,
        "mcp/disconnect",
        json_value(v2::DisconnectMcpRequest::new("conn-1"))?,
        v2::DisconnectMcpRequest { .. }
    );

    assert_message_mapping!(
        v2::ClientRequest,
        "mcp/message",
        json_value(v2::MessageMcpRequest::new("conn-1", "tools/list"))?,
        v2::ClientRequest::MessageMcpRequest(_)
    );
    assert_response_mapping!(
        v2::AgentResponse,
        "mcp/message",
        serde_json::json!({ "tools": [] }),
        v2::AgentResponse::MessageMcpResponse(_)
    );
    assert_message_mapping!(
        v2::ClientNotification,
        "mcp/message",
        json_value(v2::MessageMcpNotification::new(
            "conn-1",
            "notifications/tools/list"
        ))?,
        v2::ClientNotification::MessageMcpNotification(_)
    );
    assert_message_mapping!(
        v2::AgentRequest,
        "mcp/connect",
        json_value(v2::ConnectMcpRequest::new("server-1"))?,
        v2::AgentRequest::ConnectMcpRequest(_)
    );
    assert_message_mapping!(
        v2::AgentRequest,
        "mcp/message",
        json_value(v2::MessageMcpRequest::new("conn-1", "tools/list"))?,
        v2::AgentRequest::MessageMcpRequest(_)
    );
    assert_message_mapping!(
        v2::AgentRequest,
        "mcp/disconnect",
        json_value(v2::DisconnectMcpRequest::new("conn-1"))?,
        v2::AgentRequest::DisconnectMcpRequest(_)
    );
    assert_response_mapping!(
        v2::ClientResponse,
        "mcp/connect",
        json_value(v2::ConnectMcpResponse::new("conn-1"))?,
        v2::ClientResponse::ConnectMcpResponse(_)
    );
    assert_response_mapping!(
        v2::ClientResponse,
        "mcp/message",
        serde_json::json!({ "tools": [] }),
        v2::ClientResponse::MessageMcpResponse(_)
    );
    assert_response_mapping!(
        v2::ClientResponse,
        "mcp/disconnect",
        serde_json::json!({}),
        v2::ClientResponse::DisconnectMcpResponse(_)
    );
    assert_message_mapping!(
        v2::AgentNotification,
        "mcp/message",
        json_value(v2::MessageMcpNotification::new(
            "conn-1",
            "notifications/tools/list"
        ))?,
        v2::AgentNotification::MessageMcpNotification(_)
    );

    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn v2_client_rejects_v1_agent() -> Result<(), Error> {
    Client
        .v2()
        .connect_with(Testy::new(), async |cx| {
            let error = cx
                .send_request(v2_initialize_request(ProtocolVersion::V1))
                .block_task()
                .await
                .expect_err("v2 clients require a v2 agent");
            let data = error
                .data
                .as_ref()
                .and_then(|data| data.as_str())
                .unwrap_or_default();
            assert!(
                data.contains("only supports ACP protocol version 1"),
                "{error:?}"
            );
            Ok(())
        })
        .await
}

#[tokio::test(flavor = "current_thread")]
async fn v2_client_and_agent_negotiate_v2() -> Result<(), Error> {
    let agent = Agent
        .v2()
        .on_receive_request(
            async |initialize: v2::InitializeRequest, responder, _cx| {
                assert_eq!(initialize.protocol_version, ProtocolVersion::V2);
                responder.respond(v2_initialize_response_with_session(
                    initialize.protocol_version,
                ))
            },
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            async |request: v2::NewSessionRequest, responder, _cx| {
                assert!(AsRef::<std::path::Path>::as_ref(&request.cwd).is_absolute());
                responder.respond(v2::NewSessionResponse::new(v2::SessionId::new(
                    "v2-native-session",
                )))
            },
            agent_client_protocol::on_receive_request!(),
        );

    Client
        .v2()
        .connect_with(agent, async |cx| {
            let initialize = cx
                .send_request(v2_initialize_request(ProtocolVersion::V1))
                .block_task()
                .await?;
            assert_eq!(initialize.protocol_version, ProtocolVersion::V2);

            let session = cx
                .send_request(v2::NewSessionRequest::new(cwd()?))
                .block_task()
                .await?;
            assert_eq!(session.session_id.0.as_ref(), "v2-native-session");
            Ok(())
        })
        .await
}

#[tokio::test(flavor = "current_thread")]
async fn client_protocol_connector_routes_to_v2_client_for_v2_agent() -> Result<(), Error> {
    Client
        .protocol_connector()
        .with_v1(|| InitializingV1Client::new("v1-client-connector-session"))
        .with_v2(|| InitializingV2Client::new("v2-client-connector-session"))
        .connect_to(|| v2_agent_with_session("v2-client-connector-session"))
        .await
}

#[tokio::test(flavor = "current_thread")]
async fn client_protocol_connector_does_not_retry_after_v2_initialize_rejection()
-> Result<(), Error> {
    Client
        .protocol_connector()
        .with_v1(|| RejectingV1Client)
        .with_v2(|| ExpectingV2InitializeErrorClient)
        .connect_to(|| v1_agent_with_session("v1-client-connector-session"))
        .await
}

#[tokio::test(flavor = "current_thread")]
async fn client_protocol_connector_falls_back_to_v1_when_agent_router_negotiates_v1()
-> Result<(), Error> {
    Client
        .protocol_connector()
        .with_v1(|| InitializingV1Client::new("v1-client-connector-session"))
        .with_v2(|| InitializingV2Client::new("v2-client-connector-session"))
        .connect_to(|| {
            Agent
                .protocol_router()
                .with_v1(v1_agent_with_session("v1-client-connector-session"))
        })
        .await
}

#[tokio::test(flavor = "current_thread")]
async fn client_protocol_connector_reuses_matching_connection_before_v1_fallback()
-> Result<(), Error> {
    let connections = Arc::new(AtomicUsize::new(0));
    let agent_connections = Arc::clone(&connections);
    let v2_client_dropped = Arc::new(AtomicBool::new(false));
    let v1_drop_observer = Arc::clone(&v2_client_dropped);
    let v2_drop_observer = Arc::clone(&v2_client_dropped);

    Client
        .protocol_connector()
        .with_v1(move || {
            InitializingV1Client::new("v1-reused-session")
                .with_client_capabilities(
                    v1::ClientCapabilities::new().session(
                        v1::ClientSessionCapabilities::new().config_options(
                            v1::SessionConfigOptionsCapabilities::new()
                                .boolean(v1::BooleanConfigOptionCapabilities::new()),
                        ),
                    ),
                )
                .expect_previous_client_dropped(Arc::clone(&v1_drop_observer))
        })
        .with_v2(move || {
            DropTrackedClient::new(
                InitializingV2Client::new("v2-client-should-not-continue"),
                Arc::clone(&v2_drop_observer),
            )
        })
        .connect_to(move || {
            let connection_number = agent_connections.fetch_add(1, Ordering::SeqCst) + 1;
            let initialize_connection_number = connection_number;
            let session_connection_number = connection_number;

            Agent.protocol_router().with_v1(
                Agent
                    .builder()
                    .on_receive_request(
                        async move |initialize: v1::InitializeRequest, responder, _cx| {
                            assert_eq!(initialize_connection_number, 1);
                            assert_eq!(initialize.protocol_version, ProtocolVersion::V1);
                            let info = initialize
                                .client_info
                                .as_ref()
                                .expect("initialize should include client info");
                            assert_eq!(&*info.name, "agent-client-protocol-test");

                            responder
                                .respond(v1::InitializeResponse::new(initialize.protocol_version))
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        async move |request: v1::NewSessionRequest, responder, _cx| {
                            assert_eq!(session_connection_number, 1);
                            assert!(request.cwd.is_absolute());
                            responder.respond(v1::NewSessionResponse::new(v1::SessionId::new(
                                "v1-reused-session",
                            )))
                        },
                        agent_client_protocol::on_receive_request!(),
                    ),
            )
        })
        .await?;

    assert_eq!(connections.load(Ordering::SeqCst), 1);
    assert!(v2_client_dropped.load(Ordering::SeqCst));
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn client_protocol_connector_reconnects_before_v1_fallback() -> Result<(), Error> {
    let connections = Arc::new(AtomicUsize::new(0));
    let agent_connections = Arc::clone(&connections);
    let v1_factory_calls = Arc::new(AtomicUsize::new(0));
    let v1_factory_call_counter = Arc::clone(&v1_factory_calls);
    let v2_client_dropped = Arc::new(AtomicBool::new(false));
    let v2_drop_tracker = Arc::clone(&v2_client_dropped);
    let v2_drop_observer = Arc::clone(&v2_client_dropped);
    let v1_probe_dropped = Arc::new(AtomicBool::new(false));
    let v1_probe_drop_tracker = Arc::clone(&v1_probe_dropped);
    let v1_probe_drop_observer = Arc::clone(&v1_probe_dropped);

    Client
        .protocol_connector()
        .with_v1(move || {
            let factory_call = v1_factory_call_counter.fetch_add(1, Ordering::SeqCst);
            let mut client = InitializingV1Client::with_implementation_name(
                "v1-reconnected-session",
                "v1-reconnected-client",
            );
            if factory_call == 1 {
                client = client
                    .expect_previous_client_dropped(Arc::clone(&v2_drop_observer))
                    .expect_previous_client_dropped(Arc::clone(&v1_probe_drop_observer));
            } else {
                assert_eq!(factory_call, 0, "v1 factory should be called exactly twice");
            }

            DropTrackedClient::new(client, Arc::clone(&v1_probe_drop_tracker))
        })
        .with_v2(move || {
            DropTrackedClient::new(
                InitializingV2Client::new("v2-client-should-not-continue"),
                Arc::clone(&v2_drop_tracker),
            )
        })
        .connect_to(move || {
            let connection_number = agent_connections.fetch_add(1, Ordering::SeqCst) + 1;
            let initialize_connection_number = connection_number;
            let session_connection_number = connection_number;

            Agent.protocol_router().with_v1(
                Agent
                    .builder()
                    .on_receive_request(
                        async move |initialize: v1::InitializeRequest, responder, _cx| {
                            assert_eq!(initialize.protocol_version, ProtocolVersion::V1);
                            let info = initialize
                                .client_info
                                .as_ref()
                                .expect("initialize should include client info");

                            if initialize_connection_number == 1 {
                                assert_eq!(&*info.name, "agent-client-protocol-test");
                            } else {
                                assert_eq!(initialize_connection_number, 2);
                                assert_eq!(&*info.name, "v1-reconnected-client");
                            }

                            responder
                                .respond(v1::InitializeResponse::new(initialize.protocol_version))
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        async move |request: v1::NewSessionRequest, responder, _cx| {
                            assert_eq!(session_connection_number, 2);
                            assert!(request.cwd.is_absolute());
                            responder.respond(v1::NewSessionResponse::new(v1::SessionId::new(
                                "v1-reconnected-session",
                            )))
                        },
                        agent_client_protocol::on_receive_request!(),
                    ),
            )
        })
        .await?;

    assert_eq!(connections.load(Ordering::SeqCst), 2);
    assert_eq!(v1_factory_calls.load(Ordering::SeqCst), 2);
    assert!(v2_client_dropped.load(Ordering::SeqCst));
    assert!(v1_probe_dropped.load(Ordering::SeqCst));
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn client_protocol_connector_supports_runtime_v2_registration_flag() -> Result<(), Error> {
    runtime_flag_client_protocol_connector(false)
        .connect_to(|| v1_agent_with_session("v1-client-connector-session"))
        .await?;

    runtime_flag_client_protocol_connector(true)
        .connect_to(|| v2_agent_with_session("v2-client-connector-session"))
        .await
}

#[tokio::test(flavor = "current_thread")]
async fn protocol_router_routes_v1_client_to_v1_implementation() -> Result<(), Error> {
    let agent = Agent
        .protocol_router()
        .with_v1(
            Agent
                .builder()
                .on_receive_request(
                    async |initialize: v1::InitializeRequest, responder, _cx| {
                        assert_eq!(initialize.protocol_version, ProtocolVersion::V1);
                        responder.respond(v1::InitializeResponse::new(initialize.protocol_version))
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_request(
                    async |request: v1::NewSessionRequest, responder, _cx| {
                        assert!(request.cwd.is_absolute());
                        responder.respond(v1::NewSessionResponse::new(v1::SessionId::new(
                            "v1-protocol-router-session",
                        )))
                    },
                    agent_client_protocol::on_receive_request!(),
                ),
        )
        .with_v2(Agent.v2().on_receive_request(
            async |_initialize: v2::InitializeRequest, responder, _cx| {
                responder.respond_with_internal_error("v2 implementation should not run")
            },
            agent_client_protocol::on_receive_request!(),
        ));

    Client
        .builder()
        .connect_with(agent, async |cx| {
            let initialize = cx
                .send_request(v1_initialize_request(ProtocolVersion::V1))
                .block_task()
                .await?;
            assert_eq!(initialize.protocol_version, ProtocolVersion::V1);

            let session = cx
                .send_request(v1::NewSessionRequest::new(cwd()?))
                .block_task()
                .await?;
            assert_eq!(session.session_id.0.as_ref(), "v1-protocol-router-session");
            Ok(())
        })
        .await
}

#[tokio::test(flavor = "current_thread")]
async fn protocol_router_routes_v2_client_to_v2_implementation() -> Result<(), Error> {
    let agent = Agent
        .protocol_router()
        .with_v1(Agent.builder().on_receive_request(
            async |_initialize: v1::InitializeRequest, responder, _cx| {
                responder.respond_with_internal_error("v1 implementation should not run")
            },
            agent_client_protocol::on_receive_request!(),
        ))
        .with_v2(
            Agent
                .v2()
                .on_receive_request(
                    async |initialize: v2::InitializeRequest, responder, _cx| {
                        assert_eq!(initialize.protocol_version, ProtocolVersion::V2);
                        responder.respond(v2_initialize_response_with_session(
                            initialize.protocol_version,
                        ))
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_request(
                    async |request: v2::NewSessionRequest, responder, _cx| {
                        assert!(AsRef::<std::path::Path>::as_ref(&request.cwd).is_absolute());
                        responder.respond(v2::NewSessionResponse::new(v2::SessionId::new(
                            "v2-protocol-router-session",
                        )))
                    },
                    agent_client_protocol::on_receive_request!(),
                ),
        );

    Client
        .v2()
        .connect_with(agent, async |cx| {
            let initialize = cx
                .send_request(v2_initialize_request(ProtocolVersion::V1))
                .block_task()
                .await?;
            assert_eq!(initialize.protocol_version, ProtocolVersion::V2);

            let session = cx
                .send_request(v2::NewSessionRequest::new(cwd()?))
                .block_task()
                .await?;
            assert_eq!(session.session_id.0.as_ref(), "v2-protocol-router-session");
            Ok(())
        })
        .await
}

#[tokio::test(flavor = "current_thread")]
async fn protocol_router_can_route_only_v1() -> Result<(), Error> {
    let agent = Agent
        .protocol_router()
        .with_v1(Agent.builder().on_receive_request(
            async |initialize: v1::InitializeRequest, responder, _cx| {
                assert_eq!(initialize.protocol_version, ProtocolVersion::V1);
                responder.respond(v1::InitializeResponse::new(initialize.protocol_version))
            },
            agent_client_protocol::on_receive_request!(),
        ));

    Client
        .builder()
        .connect_with(agent, async |cx| {
            let initialize = cx
                .send_request(v1_initialize_request(ProtocolVersion::V1))
                .block_task()
                .await?;
            assert_eq!(initialize.protocol_version, ProtocolVersion::V1);
            Ok(())
        })
        .await
}

#[tokio::test(flavor = "current_thread")]
async fn protocol_router_can_route_only_v2() -> Result<(), Error> {
    let agent = Agent
        .protocol_router()
        .with_v2(Agent.v2().on_receive_request(
            async |initialize: v2::InitializeRequest, responder, _cx| {
                assert_eq!(initialize.protocol_version, ProtocolVersion::V2);
                responder.respond(v2_initialize_response_with_session(
                    initialize.protocol_version,
                ))
            },
            agent_client_protocol::on_receive_request!(),
        ));

    Client
        .v2()
        .connect_with(agent, async |cx| {
            let initialize = cx
                .send_request(v2_initialize_request(ProtocolVersion::V1))
                .block_task()
                .await?;
            assert_eq!(initialize.protocol_version, ProtocolVersion::V2);
            Ok(())
        })
        .await
}

#[tokio::test(flavor = "current_thread")]
async fn protocol_router_supports_runtime_v2_registration_flag() -> Result<(), Error> {
    Client
        .v2()
        .connect_with(runtime_flag_protocol_router(false), async |cx| {
            let error = cx
                .send_request(v2_initialize_request(ProtocolVersion::V2))
                .block_task()
                .await
                .expect_err("runtime-disabled v2 should route to v1 and fail v2 negotiation");
            let data = error
                .data
                .as_ref()
                .and_then(|data| data.as_str())
                .unwrap_or_default();
            assert!(data.contains("peer negotiated 1"), "{error:?}");
            Ok(())
        })
        .await?;

    Client
        .v2()
        .connect_with(runtime_flag_protocol_router(true), async |cx| {
            let initialize = cx
                .send_request(v2_initialize_request(ProtocolVersion::V1))
                .block_task()
                .await?;
            assert_eq!(initialize.protocol_version, ProtocolVersion::V2);
            Ok(())
        })
        .await
}

#[tokio::test(flavor = "current_thread")]
async fn protocol_router_downgrades_v2_initialize_metadata_to_v1() -> Result<(), Error> {
    let agent = Agent
        .protocol_router()
        .with_v1(Agent.builder().on_receive_request(
            async |initialize: v1::InitializeRequest, responder, _cx| {
                assert_eq!(initialize.protocol_version, ProtocolVersion::V1);
                let client_info = initialize
                    .client_info
                    .as_ref()
                    .expect("v2 info should become v1 clientInfo");
                assert_eq!(client_info.name, "v2-metadata-client");
                assert_eq!(client_info.title.as_deref(), Some("V2 Metadata Client"));
                assert!(
                    initialize.client_capabilities.session.is_some(),
                    "{:?}",
                    initialize.client_capabilities
                );
                responder.respond(v1::InitializeResponse::new(initialize.protocol_version))
            },
            agent_client_protocol::on_receive_request!(),
        ));

    Client
        .v2()
        .connect_with(agent, async |cx| {
            let request = v2::InitializeRequest::new(
                ProtocolVersion::V2,
                v2::Implementation::new("v2-metadata-client", "9.9.9").title("V2 Metadata Client"),
            );
            let error = cx
                .send_request(request)
                .block_task()
                .await
                .expect_err("v2 client should reject the downgraded v1 initialize response");
            let data = error
                .data
                .as_ref()
                .and_then(|data| data.as_str())
                .unwrap_or_default();
            assert!(data.contains("peer negotiated 1"), "{error:?}");
            Ok(())
        })
        .await
}

#[tokio::test(flavor = "current_thread")]
async fn protocol_router_v2_only_rejects_v1_client() -> Result<(), Error> {
    let agent = Agent
        .protocol_router()
        .with_v2(Agent.v2().on_receive_request(
            async |_initialize: v2::InitializeRequest, responder, _cx| {
                responder.respond_with_internal_error("v2 implementation should not run")
            },
            agent_client_protocol::on_receive_request!(),
        ));

    let (mut channel, agent_future) = ConnectTo::<Client>::into_channel_and_future(agent);
    let agent_task = tokio::spawn(agent_future);

    channel
        .tx
        .unbounded_send(TransportFrame::Single(RawJsonRpcMessage::request(
            "initialize".into(),
            json_value(v1_initialize_request(ProtocolVersion::V1))?,
            v1::RequestId::Number(1),
        )?))
        .map_err(Error::into_internal_error)?;

    while let Some(message) = channel.rx.next().await {
        let TransportFrame::Single(message) = message else {
            continue;
        };
        let RawJsonRpcMessage::Response(v1::Response::Error { error, .. }) = message else {
            continue;
        };
        let data = error
            .data
            .as_ref()
            .and_then(|data| data.as_str())
            .unwrap_or_default();
        assert!(
            data.contains("supports ACP protocol version 2"),
            "{error:?}"
        );
        agent_task.abort();
        return Ok(());
    }

    agent_task.abort();
    Err(agent_client_protocol::util::internal_error(
        "protocol router did not reject v1 initialize",
    ))
}

#[tokio::test(flavor = "current_thread")]
async fn protocol_router_rejection_is_initialize_request_error() -> Result<(), Error> {
    let agent = Agent
        .protocol_router()
        .with_v2(Agent.v2().on_receive_request(
            async |_initialize: v2::InitializeRequest, responder, _cx| {
                responder.respond_with_internal_error("v2 implementation should not run")
            },
            agent_client_protocol::on_receive_request!(),
        ));

    Client
        .builder()
        .connect_with(agent, async |cx| {
            let error = cx
                .send_request(v1_initialize_request(ProtocolVersion::V1))
                .block_task()
                .await
                .expect_err("v1 initialize should be rejected by the v2-only router");
            let data = error
                .data
                .as_ref()
                .and_then(|data| data.as_str())
                .unwrap_or_default();
            assert!(
                data.contains("supports ACP protocol version 2"),
                "{error:?}"
            );
            Ok(())
        })
        .await
}

#[tokio::test(flavor = "current_thread")]
async fn protocol_router_rejection_flushes_over_byte_streams() -> Result<(), Error> {
    let agent = Agent
        .protocol_router()
        .with_v2(Agent.v2().on_receive_request(
            async |_initialize: v2::InitializeRequest, responder, _cx| {
                responder.respond_with_internal_error("v2 implementation should not run")
            },
            agent_client_protocol::on_receive_request!(),
        ));

    let (client_writer, server_reader) = tokio::io::duplex(1024);
    let (server_writer, client_reader) = tokio::io::duplex(1024);
    let server_transport = ByteStreams::new(server_writer.compat_write(), server_reader.compat());
    let client_transport = ByteStreams::new(client_writer.compat_write(), client_reader.compat());

    let agent_task = tokio::spawn(agent.connect_to(server_transport));

    Client
        .builder()
        .connect_with(client_transport, async |cx| {
            let error = cx
                .send_request(v1_initialize_request(ProtocolVersion::V1))
                .block_task()
                .await
                .expect_err("v1 initialize should be rejected by the v2-only router");
            let data = error
                .data
                .as_ref()
                .and_then(|data| data.as_str())
                .unwrap_or_default();
            assert!(
                data.contains("supports ACP protocol version 2"),
                "{error:?}"
            );
            Ok(())
        })
        .await?;

    agent_task
        .await
        .map_err(agent_client_protocol::util::internal_error)??;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn protocol_router_rejection_flushes_with_trailing_malformed_input() -> Result<(), Error> {
    use tokio::io::{AsyncWriteExt as _, BufReader};

    let agent = Agent
        .protocol_router()
        .with_v2(Agent.v2().on_receive_request(
            async |_initialize: v2::InitializeRequest, responder, _cx| {
                responder.respond_with_internal_error("v2 implementation should not run")
            },
            agent_client_protocol::on_receive_request!(),
        ));

    let (mut client_writer, server_reader) = tokio::io::duplex(4096);
    // Keep the outbound pipe full until the malformed input is already queued.
    let (server_writer, client_reader) = tokio::io::duplex(1);
    let server_transport = ByteStreams::new(server_writer.compat_write(), server_reader.compat());
    let agent_task = tokio::spawn(agent.connect_to(server_transport));
    let mut client_reader = BufReader::new(client_reader);

    write_wire_json(
        &mut client_writer,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": json_value(v1_initialize_request(ProtocolVersion::V1))?,
        }),
    )
    .await?;
    client_writer
        .write_all(b"{not json\n")
        .await
        .map_err(Error::into_internal_error)?;
    client_writer
        .shutdown()
        .await
        .map_err(Error::into_internal_error)?;

    let response = read_wire_json(&mut client_reader).await?;
    assert_eq!(response["id"], 1);
    assert!(
        response["error"]["data"]
            .as_str()
            .is_some_and(|data| data.contains("supports ACP protocol version 2")),
        "{response:?}"
    );

    agent_task
        .await
        .map_err(agent_client_protocol::util::internal_error)??;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn protocol_router_ignores_retained_response_batch_before_initialize() -> Result<(), Error> {
    use tokio::io::{AsyncWriteExt as _, BufReader};

    let v1_agent = Agent.builder().on_receive_request(
        async |initialize: v1::InitializeRequest, responder, _cx| {
            responder.respond(v1::InitializeResponse::new(initialize.protocol_version))
        },
        agent_client_protocol::on_receive_request!(),
    );
    let agent = Agent.protocol_router().with_v1(v1_agent);

    let (mut client_writer, server_reader) = tokio::io::duplex(4096);
    let (server_writer, client_reader) = tokio::io::duplex(4096);
    let server_transport = ByteStreams::new(server_writer.compat_write(), server_reader.compat());
    let agent_task = tokio::spawn(agent.connect_to(server_transport));
    let mut client_reader = BufReader::new(client_reader);

    write_wire_json(
        &mut client_writer,
        &serde_json::json!([{
            "jsonrpc": "2.0",
            "id": 99,
            "result": null,
            "error": { "code": -32603, "message": "Internal error" },
        }]),
    )
    .await?;
    write_wire_json(
        &mut client_writer,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": json_value(v1_initialize_request(ProtocolVersion::V1))?,
        }),
    )
    .await?;

    let response = read_wire_json(&mut client_reader).await?;
    assert_eq!(response["id"], 1);
    assert!(response.get("result").is_some(), "{response:?}");

    client_writer
        .shutdown()
        .await
        .map_err(Error::into_internal_error)?;
    agent_task
        .await
        .map_err(agent_client_protocol::util::internal_error)??;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn protocol_router_ignores_retained_standalone_response_before_initialize()
-> Result<(), Error> {
    use tokio::io::{AsyncWriteExt as _, BufReader};

    let v1_agent = Agent.builder().on_receive_request(
        async |initialize: v1::InitializeRequest, responder, _cx| {
            responder.respond(v1::InitializeResponse::new(initialize.protocol_version))
        },
        agent_client_protocol::on_receive_request!(),
    );
    let agent = Agent.protocol_router().with_v1(v1_agent);

    let (mut client_writer, server_reader) = tokio::io::duplex(4096);
    let (server_writer, client_reader) = tokio::io::duplex(4096);
    let server_transport = ByteStreams::new(server_writer.compat_write(), server_reader.compat());
    let agent_task = tokio::spawn(agent.connect_to(server_transport));
    let mut client_reader = BufReader::new(client_reader);

    write_wire_json(
        &mut client_writer,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "id": 99,
            "result": null,
            "error": { "code": -32603, "message": "Internal error" },
        }),
    )
    .await?;
    write_wire_json(
        &mut client_writer,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": json_value(v1_initialize_request(ProtocolVersion::V1))?,
        }),
    )
    .await?;

    let response = read_wire_json(&mut client_reader).await?;
    assert_eq!(response["id"], 1);
    assert!(response.get("result").is_some(), "{response:?}");

    client_writer
        .shutdown()
        .await
        .map_err(Error::into_internal_error)?;
    agent_task
        .await
        .map_err(agent_client_protocol::util::internal_error)??;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn protocol_router_rejection_does_not_answer_malformed_response_entries() -> Result<(), Error>
{
    use tokio::io::{AsyncWriteExt as _, BufReader};

    let agent = Agent.protocol_router().with_v1(Agent.builder());
    let (mut client_writer, server_reader) = tokio::io::duplex(4096);
    let (server_writer, client_reader) = tokio::io::duplex(4096);
    let server_transport = ByteStreams::new(server_writer.compat_write(), server_reader.compat());
    let agent_task = tokio::spawn(agent.connect_to(server_transport));
    let mut client_reader = BufReader::new(client_reader);

    write_wire_json(
        &mut client_writer,
        &serde_json::json!([
            17,
            {
                "jsonrpc": "2.0",
                "id": 99,
                "result": null,
                "error": { "code": -32603, "message": "Internal error" },
            }
        ]),
    )
    .await?;

    let response = read_wire_json(&mut client_reader).await?;
    let responses = response
        .as_array()
        .expect("invalid batch entry should receive a response array");
    assert_eq!(responses.len(), 1);
    assert!(responses[0]["id"].is_null());
    assert_eq!(responses[0]["error"]["code"], -32600);

    client_writer
        .shutdown()
        .await
        .map_err(Error::into_internal_error)?;
    agent_task
        .await
        .map_err(agent_client_protocol::util::internal_error)??;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn protocol_router_relays_invalid_batch_members_to_external_v1_agent() -> Result<(), Error> {
    use tokio::io::{AsyncWriteExt as _, BufReader};

    let (mut client_writer, router_reader) = tokio::io::duplex(4096);
    let (router_writer, client_reader) = tokio::io::duplex(4096);
    let router_transport = ByteStreams::new(router_writer.compat_write(), router_reader.compat());

    let (router_to_agent_writer, agent_reader) = tokio::io::duplex(4096);
    let (mut agent_writer, agent_to_router_reader) = tokio::io::duplex(4096);
    let external_agent_transport = ByteStreams::new(
        router_to_agent_writer.compat_write(),
        agent_to_router_reader.compat(),
    );
    let router = Agent.protocol_router().with_v1(external_agent_transport);
    let router_task = tokio::spawn(router.connect_to(router_transport));

    let mut client_reader = BufReader::new(client_reader);
    let mut agent_reader = BufReader::new(agent_reader);

    let initialize = serde_json::json!([
        {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": json_value(v1_initialize_request(ProtocolVersion::V1))?,
        },
        17,
    ]);
    write_wire_json(&mut client_writer, &initialize).await?;
    assert_eq!(read_wire_json(&mut agent_reader).await?, initialize);

    let initialize_response = serde_json::json!([
        {
            "jsonrpc": "2.0",
            "id": 1,
            "result": {},
        },
        {
            "jsonrpc": "2.0",
            "id": null,
            "error": { "code": -32600, "message": "Invalid Request" },
        },
    ]);
    write_wire_json(&mut agent_writer, &initialize_response).await?;
    assert_eq!(
        read_wire_json(&mut client_reader).await?,
        initialize_response
    );

    let mixed_batch = serde_json::json!([
        {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "session/list",
            "params": {},
        },
        17,
    ]);
    write_wire_json(&mut client_writer, &mixed_batch).await?;
    assert_eq!(read_wire_json(&mut agent_reader).await?, mixed_batch);

    let batch_response = serde_json::json!([
        {
            "jsonrpc": "2.0",
            "id": 2,
            "result": { "sessions": [] },
        },
        {
            "jsonrpc": "2.0",
            "id": null,
            "error": { "code": -32600, "message": "Invalid Request" },
        },
    ]);
    write_wire_json(&mut agent_writer, &batch_response).await?;
    assert_eq!(read_wire_json(&mut client_reader).await?, batch_response);

    let invalid_standalone = serde_json::json!(17);
    write_wire_json(&mut client_writer, &invalid_standalone).await?;
    assert_eq!(read_wire_json(&mut agent_reader).await?, invalid_standalone);

    let invalid_standalone_response = serde_json::json!({
        "jsonrpc": "2.0",
        "id": null,
        "error": { "code": -32600, "message": "Invalid Request" },
    });
    write_wire_json(&mut agent_writer, &invalid_standalone_response).await?;
    assert_eq!(
        read_wire_json(&mut client_reader).await?,
        invalid_standalone_response
    );

    client_writer
        .shutdown()
        .await
        .map_err(Error::into_internal_error)?;
    agent_writer
        .shutdown()
        .await
        .map_err(Error::into_internal_error)?;
    router_task
        .await
        .map_err(agent_client_protocol::util::internal_error)??;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn v2_protocol_router_accepts_inbound_json_rpc_batches() -> Result<(), Error> {
    use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt as _, BufReader};

    let notifications = Arc::new(AtomicUsize::new(0));
    let received_notifications = Arc::clone(&notifications);
    let v2_agent = Agent
        .v2()
        .on_receive_request(
            async |initialize: v2::InitializeRequest, responder, _cx| {
                responder.respond(v2_initialize_response_with_session(
                    initialize.protocol_version,
                ))
            },
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_notification(
            async move |notification: v2::CancelSessionNotification, _cx| {
                assert_eq!(notification.session_id.0.as_ref(), "batch-session");
                received_notifications.fetch_add(1, Ordering::SeqCst);
                Ok(())
            },
            agent_client_protocol::on_receive_notification!(),
        )
        .on_receive_request(
            async |_request: v2::ListSessionsRequest, responder, _cx| {
                responder.respond(v2::ListSessionsResponse::new(Vec::new()))
            },
            agent_client_protocol::on_receive_request!(),
        );
    let agent = Agent.protocol_router().with_v2(v2_agent);

    let (mut client_writer, server_reader) = tokio::io::duplex(4096);
    let (server_writer, client_reader) = tokio::io::duplex(4096);
    let server_transport = ByteStreams::new(server_writer.compat_write(), server_reader.compat());
    let agent_task = tokio::spawn(agent.connect_to(server_transport));
    let mut client_reader = BufReader::new(client_reader);

    let initialize = serde_json::json!([{
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": json_value(v2_initialize_request(ProtocolVersion::V2))?,
    }]);
    client_writer
        .write_all(format!("{initialize}\n").as_bytes())
        .await
        .map_err(Error::into_internal_error)?;
    client_writer
        .flush()
        .await
        .map_err(Error::into_internal_error)?;

    let mut line = String::new();
    tokio::time::timeout(
        tokio::time::Duration::from_secs(10),
        client_reader.read_line(&mut line),
    )
    .await
    .map_err(Error::into_internal_error)?
    .map_err(Error::into_internal_error)?;
    let initialize_response: Value =
        serde_json::from_str(line.trim()).map_err(Error::into_internal_error)?;
    let initialize_responses = initialize_response
        .as_array()
        .ok_or_else(|| Error::internal_error().data("expected initialize response array"))?;
    assert_eq!(initialize_responses.len(), 1);
    assert_eq!(initialize_responses[0]["id"], 1);
    assert!(initialize_responses[0].get("result").is_some());

    let batch = serde_json::json!([
        {
            "jsonrpc": "2.0",
            "method": "session/cancel",
            "params": { "sessionId": "batch-session" },
        },
        {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "session/list",
            "params": {},
        },
    ]);
    client_writer
        .write_all(format!("{batch}\n").as_bytes())
        .await
        .map_err(Error::into_internal_error)?;
    client_writer
        .flush()
        .await
        .map_err(Error::into_internal_error)?;

    line.clear();
    tokio::time::timeout(
        tokio::time::Duration::from_secs(10),
        client_reader.read_line(&mut line),
    )
    .await
    .map_err(Error::into_internal_error)?
    .map_err(Error::into_internal_error)?;
    let batch_response: Value =
        serde_json::from_str(line.trim()).map_err(Error::into_internal_error)?;
    let responses = batch_response
        .as_array()
        .ok_or_else(|| Error::internal_error().data("expected v2 batch response array"))?;
    assert_eq!(responses.len(), 1);
    assert_eq!(responses[0]["id"], 2);
    assert_eq!(responses[0]["result"]["sessions"], serde_json::json!([]));
    assert_eq!(notifications.load(Ordering::SeqCst), 1);

    let standalone = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "session/list",
        "params": {},
    });
    client_writer
        .write_all(format!("{standalone}\n").as_bytes())
        .await
        .map_err(Error::into_internal_error)?;
    client_writer
        .flush()
        .await
        .map_err(Error::into_internal_error)?;

    line.clear();
    tokio::time::timeout(
        tokio::time::Duration::from_secs(10),
        client_reader.read_line(&mut line),
    )
    .await
    .map_err(Error::into_internal_error)?
    .map_err(Error::into_internal_error)?;
    let standalone_response: Value =
        serde_json::from_str(line.trim()).map_err(Error::into_internal_error)?;
    assert!(standalone_response.is_object());
    assert_eq!(standalone_response["id"], 3);
    assert_eq!(
        standalone_response["result"]["sessions"],
        serde_json::json!([])
    );

    client_writer
        .shutdown()
        .await
        .map_err(Error::into_internal_error)?;
    agent_task
        .await
        .map_err(agent_client_protocol::util::internal_error)??;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn protocol_router_routes_future_protocol_version_to_v2() -> Result<(), Error> {
    let agent = Agent
        .protocol_router()
        .with_v1(Agent.builder().on_receive_request(
            async |_initialize: v1::InitializeRequest, responder, _cx| {
                responder.respond_with_internal_error("v1 implementation should not run")
            },
            agent_client_protocol::on_receive_request!(),
        ))
        .with_v2(Agent.v2().on_receive_request(
            async |initialize: v2::InitializeRequest, responder, _cx| {
                assert_eq!(initialize.protocol_version, ProtocolVersion::V2);
                responder.respond(v2_initialize_response_with_session(
                    initialize.protocol_version,
                ))
            },
            agent_client_protocol::on_receive_request!(),
        ));

    let (mut channel, agent_future) = ConnectTo::<Client>::into_channel_and_future(agent);
    let agent_task = tokio::spawn(agent_future);

    channel
        .tx
        .unbounded_send(TransportFrame::Single(RawJsonRpcMessage::request(
            "initialize".into(),
            json_value(v2_initialize_request(ProtocolVersion::from(3_u16)))?,
            v1::RequestId::Number(1),
        )?))
        .map_err(Error::into_internal_error)?;

    while let Some(message) = channel.rx.next().await {
        let TransportFrame::Single(message) = message else {
            continue;
        };
        let RawJsonRpcMessage::Response(v1::Response::Result { result, .. }) = message else {
            continue;
        };
        let initialize = v2::InitializeResponse::from_value("initialize", result)?;
        assert_eq!(initialize.protocol_version, ProtocolVersion::V2);
        agent_task.abort();
        return Ok(());
    }

    agent_task.abort();
    Err(agent_client_protocol::util::internal_error(
        "protocol router did not respond to initialize",
    ))
}

/// A v2 agent whose `session/new` handler only responds once the peer cancels
/// the request via `$/cancel_request`.
fn v2_agent_with_cancellable_new_session()
-> Builder<Agent, impl agent_client_protocol::HandleDispatchFrom<Client>> {
    Agent
        .v2()
        .on_receive_request(
            async |initialize: v2::InitializeRequest, responder, _cx| {
                responder.respond(v2_initialize_response_with_session(
                    initialize.protocol_version,
                ))
            },
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            async |_request: v2::NewSessionRequest, responder, cx| {
                let cancellation = responder.cancellation();
                cx.spawn(async move {
                    let response = cancellation
                        .run_until_cancelled(std::future::pending::<
                            Result<v2::NewSessionResponse, Error>,
                        >())
                        .await;
                    responder.respond_with_result(response)
                })?;
                Ok(())
            },
            agent_client_protocol::on_receive_request!(),
        )
}

#[tokio::test(flavor = "current_thread")]
async fn v2_client_can_cancel_request_to_v2_agent() -> Result<(), Error> {
    Client
        .v2()
        .connect_with(v2_agent_with_cancellable_new_session(), async |cx| {
            let initialize = cx
                .send_request(v2_initialize_request(ProtocolVersion::V2))
                .block_task()
                .await?;
            assert_eq!(initialize.protocol_version, ProtocolVersion::V2);

            let request = cx.send_request(v2::NewSessionRequest::new(cwd()?));
            request.cancel()?;
            let error = request
                .block_task()
                .await
                .expect_err("request should be cancelled");
            assert_eq!(i32::from(error.code), -32800);
            Ok(())
        })
        .await
}
