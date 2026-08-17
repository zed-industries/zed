//! Wire-level regressions for receiving JSON-RPC batch arrays.
//!
//! Batch parsing is a transport concern: the public channel boundary carries
//! batch-aware transport frames. Calls received in one batch are answered with
//! one consolidated response array, while requests and notifications initiated
//! by the SDK remain individual messages.

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::time::Duration;

use agent_client_protocol::{
    Agent, ByteStreams, Channel, ConnectTo, ConnectionTo, Dispatch, Error, HandleDispatchFrom,
    Handled, JsonRpcMessage, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse,
    RawJsonRpcMessage, Responder, TransportBatch, TransportBatchEntry, TransportFrame,
    role::{Role, UntypedRole},
    schema::ProtocolVersion,
    schema::v1,
};
use futures::StreamExt as _;
use futures::channel::mpsc;
use futures::future::join;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt as _, BufReader, DuplexStream};
use tokio::task::JoinHandle;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

const TIMEOUT: Duration = Duration::from_secs(10);

struct DelegatingTransport<T>(T);

impl<R, T> ConnectTo<R> for DelegatingTransport<T>
where
    R: Role,
    T: ConnectTo<R>,
{
    async fn connect_to(self, client: impl ConnectTo<R::Counterpart>) -> Result<(), Error> {
        self.0.connect_to(client).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestRequest {
    message: String,
}

impl JsonRpcMessage for TestRequest {
    fn matches_method(method: &str) -> bool {
        method == "test/echo"
    }

    fn method(&self) -> &'static str {
        "test/echo"
    }

    fn to_untyped_message(
        &self,
    ) -> Result<agent_client_protocol::UntypedMessage, agent_client_protocol::Error> {
        agent_client_protocol::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(
        method: &str,
        params: &impl Serialize,
    ) -> Result<Self, agent_client_protocol::Error> {
        if !Self::matches_method(method) {
            return Err(agent_client_protocol::Error::method_not_found());
        }
        agent_client_protocol::util::json_cast_params(params)
    }
}

impl JsonRpcRequest for TestRequest {
    type Response = TestResponse;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestResponse {
    result: String,
}

impl JsonRpcResponse for TestResponse {
    fn into_json(self, _method: &str) -> Result<Value, agent_client_protocol::Error> {
        serde_json::to_value(self).map_err(agent_client_protocol::Error::into_internal_error)
    }

    fn from_value(_method: &str, value: Value) -> Result<Self, agent_client_protocol::Error> {
        agent_client_protocol::util::json_cast(&value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestNotification {
    message: String,
}

impl JsonRpcMessage for TestNotification {
    fn matches_method(method: &str) -> bool {
        method == "test/notify"
    }

    fn method(&self) -> &'static str {
        "test/notify"
    }

    fn to_untyped_message(
        &self,
    ) -> Result<agent_client_protocol::UntypedMessage, agent_client_protocol::Error> {
        agent_client_protocol::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(
        method: &str,
        params: &impl Serialize,
    ) -> Result<Self, agent_client_protocol::Error> {
        if !Self::matches_method(method) {
            return Err(agent_client_protocol::Error::method_not_found());
        }
        agent_client_protocol::util::json_cast_params(params)
    }
}

impl JsonRpcNotification for TestNotification {}

struct RetryErrorHandler;

impl HandleDispatchFrom<UntypedRole> for RetryErrorHandler {
    async fn handle_dispatch_from(
        &mut self,
        _message: Dispatch,
        _connection: ConnectionTo<UntypedRole>,
    ) -> Result<Handled<Dispatch>, Error> {
        Err(Error::internal_error().data("retry handler error won"))
    }

    fn describe_chain(&self) -> impl std::fmt::Debug {
        "RetryErrorHandler"
    }
}

fn start_server(
    notification_tx: mpsc::UnboundedSender<String>,
) -> (
    DuplexStream,
    BufReader<DuplexStream>,
    JoinHandle<Result<(), agent_client_protocol::Error>>,
) {
    let (peer_writer, sdk_reader) = tokio::io::duplex(8192);
    let (sdk_writer, peer_reader) = tokio::io::duplex(8192);
    let transport = ByteStreams::new(sdk_writer.compat_write(), sdk_reader.compat());

    let server = UntypedRole
        .builder()
        .on_receive_request(
            async |request: TestRequest,
                   responder: Responder<TestResponse>,
                   _connection: ConnectionTo<UntypedRole>| {
                if request.message == "handler error" {
                    return Err(
                        agent_client_protocol::Error::internal_error().data("handler error won")
                    );
                }
                if request.message == "respond then error" {
                    responder.respond(TestResponse {
                        result: "first response wins".into(),
                    })?;
                    return Err(agent_client_protocol::Error::internal_error());
                }
                if request.message == "drop responder" {
                    drop(responder);
                    return Ok(());
                }
                responder.respond(TestResponse {
                    result: format!("echo: {}", request.message),
                })
            },
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_notification(
            async move |notification: TestNotification, _connection: ConnectionTo<UntypedRole>| {
                if notification.message == "handler error" {
                    return Err(agent_client_protocol::Error::internal_error());
                }
                notification_tx
                    .unbounded_send(notification.message)
                    .map_err(agent_client_protocol::Error::into_internal_error)
            },
            agent_client_protocol::on_receive_notification!(),
        );

    let server_task = tokio::task::spawn_local(server.connect_to(transport));
    (peer_writer, BufReader::new(peer_reader), server_task)
}

async fn write_json_line(writer: &mut DuplexStream, value: &Value) {
    let mut bytes = serde_json::to_vec(value).expect("test JSON should serialize");
    bytes.push(b'\n');
    tokio::time::timeout(TIMEOUT, writer.write_all(&bytes))
        .await
        .expect("timed out writing JSON-RPC line")
        .expect("failed to write JSON-RPC line");
    tokio::time::timeout(TIMEOUT, writer.flush())
        .await
        .expect("timed out flushing JSON-RPC line")
        .expect("failed to flush JSON-RPC line");
}

async fn read_json_line(reader: &mut BufReader<DuplexStream>) -> Value {
    let mut line = String::new();
    let bytes_read = tokio::time::timeout(TIMEOUT, reader.read_line(&mut line))
        .await
        .expect("timed out waiting for JSON-RPC line")
        .expect("failed to read JSON-RPC line");
    assert_ne!(bytes_read, 0, "stream closed before expected JSON-RPC line");
    serde_json::from_str(line.trim()).expect("response should be valid JSON")
}

async fn next_notification(rx: &mut mpsc::UnboundedReceiver<String>) -> String {
    tokio::time::timeout(TIMEOUT, rx.next())
        .await
        .expect("timed out waiting for notification handler")
        .expect("notification channel closed unexpectedly")
}

async fn next_deferred_response(
    rx: &mut mpsc::UnboundedReceiver<(String, Responder<TestResponse>)>,
) -> (String, Responder<TestResponse>) {
    tokio::time::timeout(TIMEOUT, rx.next())
        .await
        .expect("timed out waiting for deferred responder")
        .expect("deferred responder channel closed unexpectedly")
}

fn start_deferred_server(
    responder_tx: mpsc::UnboundedSender<(String, Responder<TestResponse>)>,
) -> (
    DuplexStream,
    BufReader<DuplexStream>,
    JoinHandle<Result<(), agent_client_protocol::Error>>,
) {
    let (peer_writer, sdk_reader) = tokio::io::duplex(8192);
    let (sdk_writer, peer_reader) = tokio::io::duplex(8192);
    let transport = ByteStreams::new(sdk_writer.compat_write(), sdk_reader.compat());

    let server = UntypedRole.builder().on_receive_request(
        async move |request: TestRequest,
                    responder: Responder<TestResponse>,
                    _connection: ConnectionTo<UntypedRole>| {
            if request.message.starts_with("deferred") {
                responder_tx
                    .unbounded_send((request.message, responder))
                    .map_err(agent_client_protocol::Error::into_internal_error)
            } else {
                responder.respond(TestResponse {
                    result: format!("echo: {}", request.message),
                })
            }
        },
        agent_client_protocol::on_receive_request!(),
    );

    let server_task = tokio::task::spawn_local(server.connect_to(transport));
    (peer_writer, BufReader::new(peer_reader), server_task)
}

async fn finish_server(
    peer_writer: DuplexStream,
    server_task: JoinHandle<Result<(), agent_client_protocol::Error>>,
) {
    drop(peer_writer);
    tokio::time::timeout(TIMEOUT, server_task)
        .await
        .expect("server did not stop after incoming EOF")
        .expect("server task panicked")
        .expect("server connection failed");
}

#[tokio::test(flavor = "current_thread")]
async fn mixed_batch_returns_one_array_with_each_response_bearing_entry() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let (notification_tx, mut notification_rx) = mpsc::unbounded();
            let (mut peer_writer, mut peer_reader, server_task) = start_server(notification_tx);

            write_json_line(
                &mut peer_writer,
                &json!([
                    {
                        "jsonrpc": "2.0",
                        "id": 1,
                        "method": "test/echo",
                        "params": { "message": "request sibling" }
                    },
                    {
                        "jsonrpc": "2.0",
                        "method": "test/notify",
                        "params": { "message": "notification sibling" }
                    },
                    17,
                    {
                        "jsonrpc": "2.0",
                        "id": 2,
                        "method": "test/echo",
                        "params": { "wrong_field": "invalid params" }
                    },
                    {
                        "jsonrpc": "2.0",
                        "id": 3,
                        "method": "test/echo",
                        "params": { "message": "handler error" }
                    },
                    {
                        "jsonrpc": "2.0",
                        "id": 99,
                        "result": null,
                        "error": { "code": -32603, "message": "Internal error" }
                    },
                    {
                        "jsonrpc": "2.0",
                        "method": "test/notify",
                        "params": { "wrong_field": "invalid params" }
                    },
                    {
                        "jsonrpc": "2.0",
                        "method": "test/notify",
                        "params": { "message": "handler error" }
                    }
                ]),
            )
            .await;

            assert_eq!(
                next_notification(&mut notification_rx).await,
                "notification sibling"
            );

            let response = read_json_line(&mut peer_reader).await;
            let responses = response
                .as_array()
                .expect("batch call should receive one response array");
            assert_eq!(responses.len(), 4);

            let success = responses
                .iter()
                .find(|response| response["id"] == json!(1))
                .expect("request sibling should receive a response");
            assert_eq!(
                success["result"],
                json!({ "result": "echo: request sibling" })
            );

            let invalid = responses
                .iter()
                .find(|response| response["id"].is_null() && response.get("error").is_some())
                .expect("invalid sibling should receive an error");
            assert_eq!(invalid["error"]["code"], json!(-32600));

            let invalid_params = responses
                .iter()
                .find(|response| response["id"] == json!(2))
                .expect("invalid request params should receive an error");
            assert_eq!(invalid_params["error"]["code"], json!(-32602));

            let handler_error = responses
                .iter()
                .find(|response| response["id"] == json!(3))
                .expect("failing request handler should receive an error");
            assert_eq!(handler_error["error"]["code"], json!(-32603));
            assert_eq!(handler_error["error"]["data"], json!("handler error won"));

            write_json_line(
                &mut peer_writer,
                &json!({
                    "jsonrpc": "2.0",
                    "id": 4,
                    "method": "test/echo",
                    "params": { "message": "after mixed batch" }
                }),
            )
            .await;
            let standalone = read_json_line(&mut peer_reader).await;
            assert!(standalone.is_object());
            assert_eq!(standalone["id"], json!(4));
            assert_eq!(
                standalone["result"],
                json!({ "result": "echo: after mixed batch" })
            );

            finish_server(peer_writer, server_task).await;
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn batch_response_waits_for_trailing_notification_handler() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let entered = Arc::new(tokio::sync::Notify::new());
            let release = Arc::new(tokio::sync::Notify::new());
            let handler_entered = Arc::clone(&entered);
            let handler_release = Arc::clone(&release);

            let (mut peer_writer, sdk_reader) = tokio::io::duplex(8192);
            let (sdk_writer, peer_reader) = tokio::io::duplex(8192);
            let transport = ByteStreams::new(sdk_writer.compat_write(), sdk_reader.compat());
            let server = UntypedRole
                .builder()
                .on_receive_request(
                    async |request: TestRequest,
                           responder: Responder<TestResponse>,
                           _connection: ConnectionTo<UntypedRole>| {
                        responder.respond(TestResponse {
                            result: format!("echo: {}", request.message),
                        })
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_notification(
                    async move |_notification: TestNotification,
                                _connection: ConnectionTo<UntypedRole>| {
                        handler_entered.notify_one();
                        handler_release.notified().await;
                        Ok(())
                    },
                    agent_client_protocol::on_receive_notification!(),
                );
            let server_task = tokio::task::spawn_local(server.connect_to(transport));
            let mut peer_reader = BufReader::new(peer_reader);

            let wait_for_handler = entered.notified();
            write_json_line(
                &mut peer_writer,
                &json!([
                    {
                        "jsonrpc": "2.0",
                        "id": 7,
                        "method": "test/echo",
                        "params": { "message": "before trailing notification" }
                    },
                    {
                        "jsonrpc": "2.0",
                        "method": "test/notify",
                        "params": { "message": "held trailing notification" }
                    }
                ]),
            )
            .await;
            tokio::time::timeout(TIMEOUT, wait_for_handler)
                .await
                .expect("trailing notification handler was not entered");

            let mut line = String::new();
            {
                let read_response = peer_reader.read_line(&mut line);
                tokio::pin!(read_response);
                assert!(
                    tokio::time::timeout(Duration::from_millis(100), &mut read_response)
                        .await
                        .is_err(),
                    "batch response was emitted before the trailing notification completed"
                );

                release.notify_one();
                tokio::time::timeout(TIMEOUT, &mut read_response)
                    .await
                    .expect("timed out waiting for completed batch response")
                    .expect("failed to read completed batch response");
            }

            let response: Value =
                serde_json::from_str(line.trim()).expect("response should be valid JSON");
            let responses = response
                .as_array()
                .expect("batch call should receive one response array");
            assert_eq!(responses.len(), 1);
            assert_eq!(responses[0]["id"], json!(7));

            finish_server(peer_writer, server_task).await;
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn incomplete_batch_withholds_partial_responses_without_blocking_standalone_calls() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let (responder_tx, mut responder_rx) = mpsc::unbounded();
            let (mut peer_writer, mut peer_reader, server_task) =
                start_deferred_server(responder_tx);

            write_json_line(
                &mut peer_writer,
                &json!([
                    {
                        "jsonrpc": "2.0",
                        "id": 11,
                        "method": "test/echo",
                        "params": { "message": "deferred first" }
                    },
                    {
                        "jsonrpc": "2.0",
                        "id": 12,
                        "method": "test/echo",
                        "params": { "message": "immediate second" }
                    }
                ]),
            )
            .await;
            write_json_line(
                &mut peer_writer,
                &json!({
                    "jsonrpc": "2.0",
                    "id": 13,
                    "method": "test/echo",
                    "params": { "message": "standalone while batch pending" }
                }),
            )
            .await;

            let (message, responder) = next_deferred_response(&mut responder_rx).await;
            assert_eq!(message, "deferred first");

            // The completed batch sibling must remain buffered, while the
            // independent call remains usable and receives a normal response.
            let standalone = read_json_line(&mut peer_reader).await;
            assert!(standalone.is_object());
            assert_eq!(standalone["id"], json!(13));

            responder
                .respond(TestResponse {
                    result: "echo: deferred first".into(),
                })
                .expect("deferred response should be accepted");

            let response = read_json_line(&mut peer_reader).await;
            let responses = response
                .as_array()
                .expect("completed batch should emit one response array");
            assert_eq!(responses.len(), 2);
            let deferred = responses
                .iter()
                .find(|response| response["id"] == json!(11))
                .expect("deferred response should be present");
            assert_eq!(
                deferred["result"],
                json!({ "result": "echo: deferred first" })
            );
            let immediate = responses
                .iter()
                .find(|response| response["id"] == json!(12))
                .expect("immediate response should be present");
            assert_eq!(
                immediate["result"],
                json!({ "result": "echo: immediate second" })
            );

            finish_server(peer_writer, server_task).await;
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn dropped_batch_responder_gets_internal_error_without_stranding_siblings() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let (notification_tx, _notification_rx) = mpsc::unbounded();
            let (mut peer_writer, mut peer_reader, server_task) = start_server(notification_tx);

            write_json_line(
                &mut peer_writer,
                &json!([
                    {
                        "jsonrpc": "2.0",
                        "id": 14,
                        "method": "test/echo",
                        "params": { "message": "drop responder" }
                    },
                    {
                        "jsonrpc": "2.0",
                        "id": 15,
                        "method": "test/echo",
                        "params": { "message": "healthy sibling" }
                    }
                ]),
            )
            .await;

            let response = read_json_line(&mut peer_reader).await;
            let responses = response
                .as_array()
                .expect("both batch slots should complete in one response array");
            assert_eq!(responses.len(), 2);

            let abandoned = responses
                .iter()
                .find(|response| response["id"] == json!(14))
                .expect("abandoned request should receive a fallback response");
            assert_eq!(abandoned["error"]["code"], json!(-32603));
            assert!(
                abandoned["error"]["data"]
                    .as_str()
                    .is_some_and(|data| data.contains("dropped its responder"))
            );

            let healthy = responses
                .iter()
                .find(|response| response["id"] == json!(15))
                .expect("healthy sibling response should not be stranded");
            assert_eq!(
                healthy["result"],
                json!({ "result": "echo: healthy sibling" })
            );

            finish_server(peer_writer, server_task).await;
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn retried_batch_handler_error_wins_over_dropped_responder_fallback() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let (peer_writer, sdk_reader) = tokio::io::duplex(8192);
            let (sdk_writer, peer_reader) = tokio::io::duplex(8192);
            let transport = ByteStreams::new(sdk_writer.compat_write(), sdk_reader.compat());

            let server = UntypedRole.builder().on_receive_request(
                async |request: TestRequest,
                       responder: Responder<TestResponse>,
                       connection: ConnectionTo<UntypedRole>| {
                    match request.message.as_str() {
                        "retry later" => Ok(Handled::No {
                            message: (request, responder),
                            retry: true,
                        }),
                        "register retry error" => {
                            connection.add_dynamic_handler(RetryErrorHandler)?.detach();
                            responder.respond(TestResponse {
                                result: "retry handler registered".into(),
                            })?;
                            Ok(Handled::Yes)
                        }
                        message => {
                            responder.respond(TestResponse {
                                result: format!("echo: {message}"),
                            })?;
                            Ok(Handled::Yes)
                        }
                    }
                },
                agent_client_protocol::on_receive_request!(),
            );
            let server_task = tokio::task::spawn_local(server.connect_to(transport));
            let mut peer_writer = peer_writer;
            let mut peer_reader = BufReader::new(peer_reader);

            write_json_line(
                &mut peer_writer,
                &json!([
                    {
                        "jsonrpc": "2.0",
                        "id": 16,
                        "method": "test/echo",
                        "params": { "message": "retry later" }
                    },
                    {
                        "jsonrpc": "2.0",
                        "id": 17,
                        "method": "test/echo",
                        "params": { "message": "healthy sibling" }
                    }
                ]),
            )
            .await;

            // This standalone round trip is ordered after the original batch
            // dispatch barrier, proving the retry happens after that barrier.
            write_json_line(
                &mut peer_writer,
                &json!({
                    "jsonrpc": "2.0",
                    "id": 18,
                    "method": "test/echo",
                    "params": { "message": "barrier" }
                }),
            )
            .await;
            let barrier = read_json_line(&mut peer_reader).await;
            assert_eq!(barrier["id"], json!(18));
            assert_eq!(barrier["result"], json!({ "result": "echo: barrier" }));

            write_json_line(
                &mut peer_writer,
                &json!({
                    "jsonrpc": "2.0",
                    "id": 19,
                    "method": "test/echo",
                    "params": { "message": "register retry error" }
                }),
            )
            .await;

            let first = read_json_line(&mut peer_reader).await;
            let second = read_json_line(&mut peer_reader).await;
            let (registration, batch) = if first.is_array() {
                (second, first)
            } else {
                (first, second)
            };
            assert_eq!(registration["id"], json!(19));

            let responses = batch
                .as_array()
                .expect("retried request and its sibling should remain grouped");
            assert_eq!(responses.len(), 2);
            let retry_error = responses
                .iter()
                .find(|response| response["id"] == json!(16))
                .expect("retried request should receive an error");
            assert_eq!(retry_error["error"]["code"], json!(-32603));
            assert_eq!(
                retry_error["error"]["data"],
                json!("retry handler error won")
            );

            let sibling = responses
                .iter()
                .find(|response| response["id"] == json!(17))
                .expect("healthy sibling response should be preserved");
            assert_eq!(
                sibling["result"],
                json!({ "result": "echo: healthy sibling" })
            );

            finish_server(peer_writer, server_task).await;
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn all_invalid_nonempty_batch_returns_an_error_array_and_connection_stays_usable() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let (notification_tx, _notification_rx) = mpsc::unbounded();
            let (mut peer_writer, mut peer_reader, server_task) = start_server(notification_tx);

            write_json_line(&mut peer_writer, &json!([17, true, null])).await;
            let response = read_json_line(&mut peer_reader).await;
            let responses = response
                .as_array()
                .expect("nonempty invalid batch should receive an error array");
            assert_eq!(responses.len(), 3);
            assert!(responses.iter().all(|response| {
                response["id"].is_null() && response["error"]["code"] == json!(-32600)
            }));

            write_json_line(
                &mut peer_writer,
                &json!({
                    "jsonrpc": "2.0",
                    "id": 14,
                    "method": "test/echo",
                    "params": { "message": "after invalid batch" }
                }),
            )
            .await;
            let standalone = read_json_line(&mut peer_reader).await;
            assert!(standalone.is_object());
            assert_eq!(standalone["id"], json!(14));

            finish_server(peer_writer, server_task).await;
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn invalid_scalar_beside_malformed_response_gets_only_one_error() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let (notification_tx, _notification_rx) = mpsc::unbounded();
            let (mut peer_writer, mut peer_reader, server_task) = start_server(notification_tx);

            write_json_line(
                &mut peer_writer,
                &json!([
                    17,
                    {
                        "jsonrpc": "2.0",
                        "id": 91,
                        "result": null,
                        "error": { "code": -32603, "message": "Internal error" }
                    }
                ]),
            )
            .await;

            let response = read_json_line(&mut peer_reader).await;
            let responses = response
                .as_array()
                .expect("the invalid call-shaped entry should receive one response array");
            assert_eq!(responses.len(), 1);
            assert!(responses[0]["id"].is_null());
            assert_eq!(responses[0]["error"]["code"], json!(-32600));

            write_json_line(
                &mut peer_writer,
                &json!({
                    "jsonrpc": "2.0",
                    "id": 92,
                    "method": "test/echo",
                    "params": { "message": "after malformed response" }
                }),
            )
            .await;
            let standalone = read_json_line(&mut peer_reader).await;
            assert!(standalone.is_object());
            assert_eq!(standalone["id"], json!(92));

            finish_server(peer_writer, server_task).await;
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn protocol_actor_ignores_response_shaped_malformed_public_frame_entries() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let (server_transport, mut peer) = Channel::duplex();
            let server = UntypedRole.builder().on_receive_request(
                async |request: TestRequest,
                       responder: Responder<TestResponse>,
                       _connection: ConnectionTo<UntypedRole>| {
                    responder.respond(TestResponse {
                        result: format!("echo: {}", request.message),
                    })
                },
                agent_client_protocol::on_receive_request!(),
            );
            let server_task = tokio::task::spawn_local(server.connect_to(server_transport));

            let request = serde_json::from_value::<RawJsonRpcMessage>(json!({
                "jsonrpc": "2.0",
                "id": 94,
                "method": "test/echo",
                "params": { "message": "after retained response" }
            }))
            .expect("test request should be valid JSON-RPC");
            let batch = TransportBatch::from_entries([
                TransportBatchEntry::malformed(
                    json!({
                        "jsonrpc": "2.0",
                        "id": 93,
                        "result": null,
                        "error": { "code": -32603, "message": "Internal error" }
                    }),
                    Error::invalid_request(),
                ),
                TransportBatchEntry::message(request),
            ])
            .expect("test batch is non-empty");
            peer.tx
                .unbounded_send(TransportFrame::Batch(batch))
                .expect("server should accept the test batch");

            let frame = tokio::time::timeout(TIMEOUT, peer.rx.next())
                .await
                .expect("timed out waiting for the batch response")
                .expect("server channel closed before responding");
            let TransportFrame::Batch(batch) = frame else {
                panic!("request sibling should receive one grouped batch response");
            };
            let response = serde_json::to_value(batch).expect("batch response should serialize");
            let responses = response.as_array().expect("response should be an array");
            assert_eq!(responses.len(), 1);
            assert_eq!(responses[0]["id"], json!(94));

            drop(peer);
            tokio::time::timeout(TIMEOUT, server_task)
                .await
                .expect("server did not stop after channel close")
                .expect("server task panicked")
                .expect("server connection failed");
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn duplicate_request_ids_keep_distinct_batch_response_slots() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let (notification_tx, _notification_rx) = mpsc::unbounded();
            let (mut peer_writer, mut peer_reader, server_task) = start_server(notification_tx);

            write_json_line(
                &mut peer_writer,
                &json!([
                    {
                        "jsonrpc": "2.0",
                        "id": 21,
                        "method": "test/echo",
                        "params": { "message": "duplicate first" }
                    },
                    {
                        "jsonrpc": "2.0",
                        "id": 21,
                        "method": "test/echo",
                        "params": { "message": "duplicate second" }
                    }
                ]),
            )
            .await;

            let response = read_json_line(&mut peer_reader).await;
            let responses = response
                .as_array()
                .expect("duplicate IDs should still receive one response array");
            assert_eq!(responses.len(), 2);
            assert!(responses.iter().all(|response| response["id"] == json!(21)));
            assert!(responses.iter().any(|response| {
                response["result"] == json!({ "result": "echo: duplicate first" })
            }));
            assert!(responses.iter().any(|response| {
                response["result"] == json!({ "result": "echo: duplicate second" })
            }));

            finish_server(peer_writer, server_task).await;
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn duplicate_completion_after_batch_flush_is_ignored() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let (notification_tx, _notification_rx) = mpsc::unbounded();
            let (mut peer_writer, mut peer_reader, server_task) = start_server(notification_tx);

            write_json_line(
                &mut peer_writer,
                &json!([{
                    "jsonrpc": "2.0",
                    "id": 25,
                    "method": "test/echo",
                    "params": { "message": "respond then error" }
                }]),
            )
            .await;

            let response = read_json_line(&mut peer_reader).await;
            let responses = response
                .as_array()
                .expect("batch should emit one response array");
            assert_eq!(responses.len(), 1);
            assert_eq!(responses[0]["id"], json!(25));
            assert_eq!(
                responses[0]["result"],
                json!({ "result": "first response wins" })
            );

            write_json_line(
                &mut peer_writer,
                &json!({
                    "jsonrpc": "2.0",
                    "id": 26,
                    "method": "test/echo",
                    "params": { "message": "after duplicate completion" }
                }),
            )
            .await;
            let standalone = read_json_line(&mut peer_reader).await;
            assert!(standalone.is_object());
            assert_eq!(standalone["id"], json!(26));

            finish_server(peer_writer, server_task).await;
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn duplicate_completion_of_individual_request_is_ignored() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let (notification_tx, _notification_rx) = mpsc::unbounded();
            let (mut peer_writer, mut peer_reader, server_task) = start_server(notification_tx);

            write_json_line(
                &mut peer_writer,
                &json!({
                    "jsonrpc": "2.0",
                    "id": 27,
                    "method": "test/echo",
                    "params": { "message": "respond then error" }
                }),
            )
            .await;
            write_json_line(
                &mut peer_writer,
                &json!({
                    "jsonrpc": "2.0",
                    "id": 28,
                    "method": "test/echo",
                    "params": { "message": "after duplicate completion" }
                }),
            )
            .await;

            let first = read_json_line(&mut peer_reader).await;
            assert_eq!(first["id"], json!(27));
            assert_eq!(first["result"], json!({ "result": "first response wins" }));

            let barrier = read_json_line(&mut peer_reader).await;
            assert_eq!(
                barrier["id"],
                json!(28),
                "a duplicate handler-error response must not precede the next request"
            );
            assert_eq!(
                barrier["result"],
                json!({ "result": "echo: after duplicate completion" })
            );

            finish_server(peer_writer, server_task).await;
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn overlapping_batches_may_reuse_ids_without_cross_contamination() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let (responder_tx, mut responder_rx) = mpsc::unbounded();
            let (mut peer_writer, mut peer_reader, server_task) =
                start_deferred_server(responder_tx);

            for message in ["deferred batch a", "deferred batch b"] {
                write_json_line(
                    &mut peer_writer,
                    &json!([{
                        "jsonrpc": "2.0",
                        "id": 31,
                        "method": "test/echo",
                        "params": { "message": message }
                    }]),
                )
                .await;
            }

            let (message_a, responder_a) = next_deferred_response(&mut responder_rx).await;
            let (message_b, responder_b) = next_deferred_response(&mut responder_rx).await;
            assert_eq!(message_a, "deferred batch a");
            assert_eq!(message_b, "deferred batch b");

            // Complete the later batch first to ensure grouping is based on a
            // batch-local slot rather than the reused JSON-RPC id.
            responder_b
                .respond(TestResponse {
                    result: "echo: deferred batch b".into(),
                })
                .expect("second batch response should be accepted");
            responder_a
                .respond(TestResponse {
                    result: "echo: deferred batch a".into(),
                })
                .expect("first batch response should be accepted");

            let wire_responses = [
                read_json_line(&mut peer_reader).await,
                read_json_line(&mut peer_reader).await,
            ];
            let mut results = wire_responses
                .iter()
                .map(|response| {
                    let responses = response
                        .as_array()
                        .expect("each inbound batch should receive its own array");
                    assert_eq!(responses.len(), 1);
                    assert_eq!(responses[0]["id"], json!(31));
                    responses[0]["result"]["result"]
                        .as_str()
                        .expect("test response should be a string")
                        .to_string()
                })
                .collect::<Vec<_>>();
            results.sort();
            assert_eq!(
                results,
                [
                    "echo: deferred batch a".to_string(),
                    "echo: deferred batch b".to_string()
                ]
            );

            finish_server(peer_writer, server_task).await;
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn empty_batch_returns_one_invalid_request_and_connection_stays_usable() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let (notification_tx, _notification_rx) = mpsc::unbounded();
            let (mut peer_writer, mut peer_reader, server_task) = start_server(notification_tx);

            write_json_line(&mut peer_writer, &json!([])).await;
            let invalid = read_json_line(&mut peer_reader).await;
            assert!(
                invalid.is_object(),
                "empty batch response must be a single object"
            );
            assert!(invalid["id"].is_null());
            assert_eq!(invalid["error"]["code"], json!(-32600));

            write_json_line(
                &mut peer_writer,
                &json!({
                    "jsonrpc": "2.0",
                    "id": 2,
                    "method": "test/echo",
                    "params": { "message": "after empty batch" }
                }),
            )
            .await;
            let success = read_json_line(&mut peer_reader).await;
            assert!(success.is_object());
            assert_eq!(success["id"], json!(2));
            assert_eq!(
                success["result"],
                json!({ "result": "echo: after empty batch" })
            );

            finish_server(peer_writer, server_task).await;
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn notification_only_batch_emits_nothing_before_following_barrier_response() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let (notification_tx, mut notification_rx) = mpsc::unbounded();
            let (mut peer_writer, mut peer_reader, server_task) = start_server(notification_tx);

            write_json_line(
                &mut peer_writer,
                &json!([
                    {
                        "jsonrpc": "2.0",
                        "method": "test/notify",
                        "params": { "message": "first" }
                    },
                    {
                        "jsonrpc": "2.0",
                        "method": "test/notify",
                        "params": { "wrong_field": "invalid params" }
                    },
                    {
                        "jsonrpc": "2.0",
                        "method": "test/notify",
                        "params": { "message": "handler error" }
                    },
                    {
                        "jsonrpc": "2.0",
                        "method": "unknown/notification",
                        "params": {}
                    },
                    {
                        "jsonrpc": "2.0",
                        "method": "test/notify",
                        "params": { "message": "second" }
                    }
                ]),
            )
            .await;
            write_json_line(
                &mut peer_writer,
                &json!({
                    "jsonrpc": "2.0",
                    "id": 9,
                    "method": "test/echo",
                    "params": { "message": "barrier" }
                }),
            )
            .await;

            // Incoming entries and outgoing responses are queued in order. If
            // any notification, including either failing notification,
            // produced a response, it would precede this barrier response and
            // make this assertion fail without a sleep.
            let barrier = read_json_line(&mut peer_reader).await;
            assert!(barrier.is_object());
            assert_eq!(barrier["id"], json!(9));
            assert_eq!(barrier["result"], json!({ "result": "echo: barrier" }));
            assert_eq!(next_notification(&mut notification_rx).await, "first");
            assert_eq!(next_notification(&mut notification_rx).await, "second");

            finish_server(peer_writer, server_task).await;
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn response_batch_routes_two_pending_requests_by_id() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let (mut peer_writer, sdk_reader) = tokio::io::duplex(8192);
            let (sdk_writer, peer_reader) = tokio::io::duplex(8192);
            let transport = ByteStreams::new(sdk_writer.compat_write(), sdk_reader.compat());

            let client = UntypedRole
                .builder()
                .connect_with(transport, async |connection| {
                    connection.send_notification(TestNotification {
                        message: "outbound notification".into(),
                    })?;
                    let first = connection
                        .send_request(TestRequest {
                            message: "first".into(),
                        })
                        .block_task();
                    let second = connection
                        .send_request(TestRequest {
                            message: "second".into(),
                        })
                        .block_task();
                    let (first, second) = join(first, second).await;
                    let barrier = connection
                        .send_request(TestRequest {
                            message: "barrier".into(),
                        })
                        .block_task()
                        .await?;
                    Ok::<_, agent_client_protocol::Error>((
                        first?.result,
                        second?.result,
                        barrier.result,
                    ))
                });

            let peer = async move {
                let mut peer_reader = BufReader::new(peer_reader);
                let notification = read_json_line(&mut peer_reader).await;
                assert!(notification.is_object());
                assert_eq!(notification["method"], json!("test/notify"));
                assert_eq!(
                    notification["params"]["message"],
                    json!("outbound notification")
                );
                assert!(notification.get("id").is_none());

                let requests = [
                    read_json_line(&mut peer_reader).await,
                    read_json_line(&mut peer_reader).await,
                ];
                assert!(
                    requests.iter().all(Value::is_object),
                    "SDK should continue sending individual request lines"
                );

                let id_for = |message: &str| {
                    requests
                        .iter()
                        .find(|request| request["params"]["message"] == json!(message))
                        .unwrap_or_else(|| panic!("missing outbound request for {message}"))["id"]
                        .clone()
                };
                let first_id = id_for("first");
                let second_id = id_for("second");

                // Reverse the responses to prove routing uses JSON-RPC ids, not
                // the order of entries in the batch.
                write_json_line(
                    &mut peer_writer,
                    &json!([
                        17,
                        {
                            "jsonrpc": "2.0",
                            "id": second_id,
                            "result": { "result": "second response" }
                        },
                        {
                            "jsonrpc": "2.0",
                            "id": first_id,
                            "result": { "result": "first response" }
                        }
                    ]),
                )
                .await;

                // The scalar is still an invalid call-shaped batch member and
                // receives an Invalid Request response. Valid response siblings
                // are routed to their local awaiters rather than answered.
                let invalid = read_json_line(&mut peer_reader).await;
                let invalid = invalid
                    .as_array()
                    .expect("invalid sibling should receive one response array");
                assert_eq!(invalid.len(), 1);
                assert!(invalid[0]["id"].is_null());
                assert_eq!(invalid[0]["error"]["code"], json!(-32600));

                // The next outbound message is the request sent after both
                // valid responses resolve.
                let barrier = read_json_line(&mut peer_reader).await;
                assert_eq!(barrier["method"], json!("test/echo"));
                assert_eq!(barrier["params"]["message"], json!("barrier"));
                write_json_line(
                    &mut peer_writer,
                    &json!({
                        "jsonrpc": "2.0",
                        "id": barrier["id"].clone(),
                        "result": { "result": "barrier response" }
                    }),
                )
                .await;
            };

            let (client_result, ()) = tokio::time::timeout(TIMEOUT, join(client, peer))
                .await
                .expect("batch responses did not resolve pending requests");
            assert_eq!(
                client_result.expect("client connection should succeed"),
                (
                    "first response".to_string(),
                    "second response".to_string(),
                    "barrier response".to_string()
                )
            );
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn v1_agent_accepts_inbound_batch_through_default_component_adapter() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let notifications = Arc::new(AtomicUsize::new(0));
            let received_notifications = Arc::clone(&notifications);
            let agent = Agent
                .builder()
                .on_receive_request(
                    async |initialize: v1::InitializeRequest, responder, _cx| {
                        responder.respond(v1::InitializeResponse::new(initialize.protocol_version))
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_notification(
                    async move |notification: v1::CancelNotification, _cx| {
                        assert_eq!(notification.session_id.0.as_ref(), "batch-session");
                        received_notifications.fetch_add(1, Ordering::SeqCst);
                        Ok(())
                    },
                    agent_client_protocol::on_receive_notification!(),
                )
                .on_receive_request(
                    async |_request: v1::ListSessionsRequest, responder, _cx| {
                        responder.respond(v1::ListSessionsResponse::new(Vec::new()))
                    },
                    agent_client_protocol::on_receive_request!(),
                );

            let (mut peer_writer, sdk_reader) = tokio::io::duplex(8192);
            let (sdk_writer, peer_reader) = tokio::io::duplex(8192);
            let transport = DelegatingTransport(ByteStreams::new(
                sdk_writer.compat_write(),
                sdk_reader.compat(),
            ));
            let agent_task = tokio::task::spawn_local(agent.connect_to(transport));
            let mut peer_reader = BufReader::new(peer_reader);

            write_json_line(
                &mut peer_writer,
                &json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "initialize",
                    "params": serde_json::to_value(v1::InitializeRequest::new(
                        ProtocolVersion::V1
                    ))
                    .expect("v1 initialize request should serialize")
                }),
            )
            .await;
            let initialize = read_json_line(&mut peer_reader).await;
            assert_eq!(initialize["id"], json!(1));
            assert_eq!(
                initialize["result"]["protocolVersion"],
                serde_json::to_value(ProtocolVersion::V1)
                    .expect("protocol version should serialize")
            );

            write_json_line(
                &mut peer_writer,
                &json!([
                    {
                        "jsonrpc": "2.0",
                        "method": "session/cancel",
                        "params": { "sessionId": "batch-session" }
                    },
                    {
                        "jsonrpc": "2.0",
                        "id": 2,
                        "method": "session/list",
                        "params": {}
                    }
                ]),
            )
            .await;

            let batch_response = read_json_line(&mut peer_reader).await;
            let responses = batch_response
                .as_array()
                .expect("v1 batch call should receive one response array");
            assert_eq!(responses.len(), 1);
            assert_eq!(responses[0]["id"], json!(2));
            assert_eq!(responses[0]["result"]["sessions"], json!([]));
            assert_eq!(notifications.load(Ordering::SeqCst), 1);

            write_json_line(
                &mut peer_writer,
                &json!({
                    "jsonrpc": "2.0",
                    "id": 3,
                    "method": "session/list",
                    "params": {}
                }),
            )
            .await;
            let standalone = read_json_line(&mut peer_reader).await;
            assert!(standalone.is_object());
            assert_eq!(standalone["id"], json!(3));
            assert_eq!(standalone["result"]["sessions"], json!([]));

            finish_server(peer_writer, agent_task).await;
        })
        .await;
}
