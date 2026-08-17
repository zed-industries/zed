//! Tests for messages that nobody is waiting for or that no handler claims.
//!
//! Everything in this file holds **regardless of feature flags** — these are
//! baseline guarantees of the dispatch loop:
//!
//! - Unhandled notifications are ignored instead of rejected, so peers that
//!   use optional protocol-level or vendor extensions interoperate with
//!   components that do not support them.
//! - A `_proxy/successor` envelope that cannot be peeled still reaches the
//!   handler chain unchanged.
//! - A response routed to a request handle that was already dropped is
//!   discarded without disturbing the connection.
//! - `forward_response_to` answers the incoming request with an error when
//!   the pending response is dropped without ever being delivered, instead of
//!   leaving the peer waiting forever.
//!
//! Like the other JSON-RPC tests, these avoid sleeps: messages are delivered
//! in order and each side's dispatch loop processes them sequentially, so a
//! request/response round trip acts as a barrier.

use std::sync::{Arc, Mutex};

use agent_client_protocol::{
    Channel, ConnectionTo, Dispatch, Handled, JsonRpcMessage, JsonRpcRequest, JsonRpcResponse,
    Responder, SentRequest, role::UntypedRole,
};
use expect_test::expect;
use futures::StreamExt as _;
use futures::channel::mpsc;
use serde::{Deserialize, Serialize};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

/// Await the next item on `rx`, panicking instead of hanging if it never
/// arrives.
async fn next_with_timeout<T>(rx: &mut mpsc::UnboundedReceiver<T>) -> T {
    tokio::time::timeout(tokio::time::Duration::from_secs(10), rx.next())
        .await
        .expect("timed out waiting for channel event")
        .expect("channel closed before expected event")
}

async fn read_jsonrpc_response_line(
    reader: &mut tokio::io::BufReader<tokio::io::DuplexStream>,
) -> serde_json::Value {
    use tokio::io::AsyncBufReadExt as _;

    let mut line = String::new();
    match tokio::time::timeout(
        tokio::time::Duration::from_secs(10),
        reader.read_line(&mut line),
    )
    .await
    {
        Ok(Ok(0)) | Err(_) => panic!("timed out waiting for JSON-RPC response"),
        Ok(Ok(_)) => serde_json::from_str(line.trim()).expect("response should be valid JSON"),
        Ok(Err(error)) => panic!("failed to read JSON-RPC response line: {error}"),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimpleRequest {
    message: String,
}

impl JsonRpcMessage for SimpleRequest {
    fn matches_method(method: &str) -> bool {
        method == "simple_method"
    }

    fn method(&self) -> &'static str {
        "simple_method"
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

impl JsonRpcRequest for SimpleRequest {
    type Response = SimpleResponse;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimpleResponse {
    result: String,
}

impl JsonRpcResponse for SimpleResponse {
    fn into_json(self, _method: &str) -> Result<serde_json::Value, agent_client_protocol::Error> {
        serde_json::to_value(self).map_err(agent_client_protocol::Error::into_internal_error)
    }

    fn from_value(
        _method: &str,
        value: serde_json::Value,
    ) -> Result<Self, agent_client_protocol::Error> {
        agent_client_protocol::util::json_cast(&value)
    }
}

#[tokio::test(flavor = "current_thread")]
async fn unhandled_notifications_are_ignored() {
    use tokio::io::{AsyncWriteExt, BufReader};
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (mut client_writer, server_reader) = tokio::io::duplex(4096);
            let (server_writer, client_reader) = tokio::io::duplex(4096);

            let server_transport = agent_client_protocol::ByteStreams::new(
                server_writer.compat_write(),
                server_reader.compat(),
            );
            let server = UntypedRole.builder().on_receive_request(
                async |request: SimpleRequest,
                       responder: Responder<SimpleResponse>,
                       _connection: ConnectionTo<UntypedRole>| {
                    responder.respond(SimpleResponse {
                        result: format!("echo: {}", request.message),
                    })
                },
                agent_client_protocol::on_receive_request!(),
            );

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_transport).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            let mut client_reader = BufReader::new(client_reader);

            client_writer
                .write_all(
                    br#"{"jsonrpc":"2.0","method":"$/cancel_request","params":{"requestId":"req-1"}}
"#,
                )
                .await
                .unwrap();
            client_writer
                .write_all(
                    br#"{"jsonrpc":"2.0","method":"$/cancel_request","params":{"requestId":true}}
"#,
                )
                .await
                .unwrap();
            client_writer
                .write_all(
                    br#"{"jsonrpc":"2.0","method":"_x.ai/settings/update","params":{"theme":"auto"}}
"#,
                )
                .await
                .unwrap();
            client_writer
                .write_all(
                    br#"{"jsonrpc":"2.0","method":"unknown/notification","params":{"value":1}}
"#,
                )
                .await
                .unwrap();
            client_writer.flush().await.unwrap();

            // The server processes messages in order: a response to this
            // request proves the unknown and malformed notifications sent
            // before it were ignored without replying or closing the connection.
            client_writer
                .write_all(
                    br#"{"jsonrpc":"2.0","id":2,"method":"simple_method","params":{"message":"after notifications"}}
"#,
                )
                .await
                .unwrap();
            client_writer.flush().await.unwrap();

            let response = read_jsonrpc_response_line(&mut client_reader).await;
            expect![[r#"
                {
                  "jsonrpc": "2.0",
                  "id": 2,
                  "result": {
                    "result": "echo: after notifications"
                  }
                }"#]]
            .assert_eq(&serde_json::to_string_pretty(&response).unwrap());
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn unhandled_requests_are_rejected_with_method_not_found() {
    use tokio::io::{AsyncWriteExt, BufReader};
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (mut client_writer, server_reader) = tokio::io::duplex(4096);
            let (server_writer, client_reader) = tokio::io::duplex(4096);

            let server_transport = agent_client_protocol::ByteStreams::new(
                server_writer.compat_write(),
                server_reader.compat(),
            );
            let server = UntypedRole.builder();

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_transport).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            let mut client_reader = BufReader::new(client_reader);

            client_writer
                .write_all(
                    br#"{"jsonrpc":"2.0","id":1,"method":"unknown/request","params":{"value":1}}
"#,
                )
                .await
                .unwrap();
            client_writer.flush().await.unwrap();

            let response = read_jsonrpc_response_line(&mut client_reader).await;
            expect![[r#"
                {
                  "jsonrpc": "2.0",
                  "id": 1,
                  "error": {
                    "code": -32601,
                    "message": "Method not found",
                    "data": "unknown/request"
                  }
                }"#]]
            .assert_eq(&serde_json::to_string_pretty(&response).unwrap());
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn malformed_successor_envelope_still_reaches_handlers() {
    use tokio::io::AsyncWriteExt;
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (notification_tx, mut notification_rx) = mpsc::unbounded();

            let (mut client_writer, server_reader) = tokio::io::duplex(4096);
            let (server_writer, _client_reader) = tokio::io::duplex(4096);

            let server_transport = agent_client_protocol::ByteStreams::new(
                server_writer.compat_write(),
                server_reader.compat(),
            );
            // A catch-all notification handler: a successor envelope whose
            // params cannot be peeled (no inner `method`) must not be
            // mistaken for a protocol-level notification and ignored; it must
            // flow through the handler chain like any other notification.
            let server = UntypedRole.builder().on_receive_notification(
                async move |notification: agent_client_protocol::UntypedMessage,
                            _connection: ConnectionTo<UntypedRole>| {
                    notification_tx
                        .unbounded_send((notification.method, notification.params))
                        .unwrap();
                    Ok(())
                },
                agent_client_protocol::on_receive_notification!(),
            );

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_transport).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            client_writer
                .write_all(
                    br#"{"jsonrpc":"2.0","method":"_proxy/successor","params":{"bogus":true}}
"#,
                )
                .await
                .unwrap();
            client_writer.flush().await.unwrap();

            let (method, params) = next_with_timeout(&mut notification_rx).await;
            assert_eq!(method, "_proxy/successor");
            assert_eq!(params, serde_json::json!({ "bogus": true }));
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn late_response_to_dropped_request_is_discarded_without_closing_connection() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            // The responder for the abandoned request, parked by the server
            // until the client asks for its release.
            let pending_responder: Arc<Mutex<Option<Responder<SimpleResponse>>>> =
                Arc::new(Mutex::new(None));

            let (client_end, server_end) = Channel::duplex();

            let server = UntypedRole.builder().on_receive_request(
                {
                    let pending_responder = pending_responder.clone();
                    async move |request: SimpleRequest,
                                responder: Responder<SimpleResponse>,
                                _connection: ConnectionTo<UntypedRole>| {
                        match request.message.as_str() {
                            "late" => {
                                *pending_responder.lock().unwrap() = Some(responder);
                                Ok(())
                            }
                            "release" => {
                                // Answer the abandoned request first, then the
                                // release request: the late response is routed
                                // by the client before the release response.
                                if let Some(parked) = pending_responder.lock().unwrap().take() {
                                    parked.respond(SimpleResponse {
                                        result: "late response".into(),
                                    })?;
                                }
                                responder.respond(SimpleResponse {
                                    result: "echo: release".into(),
                                })
                            }
                            other => responder.respond(SimpleResponse {
                                result: format!("echo: {other}"),
                            }),
                        }
                    }
                },
                agent_client_protocol::on_receive_request!(),
            );

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_end).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            UntypedRole
                .builder()
                .connect_with(client_end, async |cx| {
                    let request: SentRequest<SimpleResponse> = cx.send_request(SimpleRequest {
                        message: "late".into(),
                    });
                    drop(request);

                    // By the time this round trip completes, the late response
                    // has already been routed into the dropped handle above.
                    let release = cx
                        .send_request(SimpleRequest {
                            message: "release".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(release.result, "echo: release");

                    // The connection survived the unroutable response.
                    let after = cx
                        .send_request(SimpleRequest {
                            message: "after".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(after.result, "echo: after");

                    Ok(())
                })
                .await
                .unwrap();
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn forward_response_to_answers_upstream_when_response_is_never_delivered() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (backend_for_proxy, backend_for_server) = Channel::duplex();
            let (backend_connection_tx, backend_connection_rx) =
                futures::channel::oneshot::channel();

            // The proxy's connection to the backend swallows every response:
            // the `ResponseRouter` (and with it the pending response sender)
            // is dropped without ever delivering the response, as also
            // happens when a downstream connection closes mid-request.
            tokio::task::spawn_local(async move {
                let result = UntypedRole
                    .builder()
                    .on_receive_dispatch(
                        async |dispatch: Dispatch, _connection: ConnectionTo<UntypedRole>| {
                            if matches!(dispatch, Dispatch::Response(..)) {
                                return Ok(Handled::Yes);
                            }
                            Ok(Handled::No {
                                message: dispatch,
                                retry: false,
                            })
                        },
                        agent_client_protocol::on_receive_dispatch!(),
                    )
                    .connect_with(backend_for_proxy, async |connection| {
                        drop(backend_connection_tx.send(connection.clone()));
                        std::future::pending::<Result<(), agent_client_protocol::Error>>().await
                    })
                    .await;
                if let Err(error) = result {
                    panic!("proxy-to-backend connection should stay alive: {error:?}");
                }
            });

            // The backend itself answers promptly; its response is then
            // swallowed on the proxy side.
            let backend_server = UntypedRole.builder().on_receive_request(
                async |request: SimpleRequest,
                       responder: Responder<SimpleResponse>,
                       _connection: ConnectionTo<UntypedRole>| {
                    responder.respond(SimpleResponse {
                        result: format!("echo: {}", request.message),
                    })
                },
                agent_client_protocol::on_receive_request!(),
            );

            tokio::task::spawn_local(async move {
                if let Err(error) = backend_server.connect_to(backend_for_server).await {
                    panic!("backend server should stay alive: {error:?}");
                }
            });

            let backend_connection = backend_connection_rx
                .await
                .expect("backend connection should start");

            let (client_end, proxy_end) = Channel::duplex();
            let proxy = UntypedRole.builder().on_receive_request(
                {
                    let backend_connection = backend_connection.clone();
                    async move |request: SimpleRequest,
                                responder: Responder<SimpleResponse>,
                                _connection: ConnectionTo<UntypedRole>| {
                        if request.message == "forward" {
                            return backend_connection
                                .send_request(request)
                                .forward_response_to(responder);
                        }
                        responder.respond(SimpleResponse {
                            result: format!("local: {}", request.message),
                        })
                    }
                },
                agent_client_protocol::on_receive_request!(),
            );

            tokio::task::spawn_local(async move {
                if let Err(error) = proxy.connect_to(proxy_end).await {
                    panic!("proxy should stay alive: {error:?}");
                }
            });

            UntypedRole
                .builder()
                .connect_with(client_end, async |cx| {
                    // The forwarded request must not be left unanswered when
                    // its response is dropped downstream.
                    let error = cx
                        .send_request(SimpleRequest {
                            message: "forward".into(),
                        })
                        .block_task()
                        .await
                        .expect_err("the response was dropped downstream");
                    let detail = error
                        .data
                        .as_ref()
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or_default();
                    assert!(
                        detail.contains("never received"),
                        "unexpected error: {error:?}"
                    );

                    // The proxy and its connection to the client still work.
                    let local = cx
                        .send_request(SimpleRequest {
                            message: "ping".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(local.result, "local: ping");

                    Ok(())
                })
                .await
                .unwrap();
        })
        .await;
}
