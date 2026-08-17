//! Integration tests for `$/cancel_request` support.
//!
//! These tests avoid sleeps by relying on two ordering guarantees:
//!
//! - Messages are delivered in the order they were sent, and each side's
//!   dispatch loop processes incoming messages sequentially. A request/response
//!   round trip therefore acts as a barrier: by the time the response arrives,
//!   every message sent before the request (including any `$/cancel_request`)
//!   has been fully processed by the peer.
//! - Test handlers report observed cancellations through in-process channels,
//!   which the test awaits (with a timeout) instead of sleeping.

use std::sync::{Arc, Mutex};

use agent_client_protocol::{
    Channel, ConnectionTo, Dispatch, HandleDispatchFrom, Handled, JsonRpcMessage, JsonRpcRequest,
    JsonRpcResponse, Responder, Role, RoleId, SentRequest,
    role::UntypedRole,
    schema::v1::{CancelRequestNotification, ProtocolLevelNotification, RequestId},
};
use expect_test::expect;
use futures::channel::mpsc;
use futures::{AsyncRead, AsyncWrite, StreamExt as _};
use serde::{Deserialize, Serialize};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

fn setup_test_streams() -> (
    impl AsyncRead,
    impl AsyncWrite,
    impl AsyncRead,
    impl AsyncWrite,
) {
    let (client_writer, server_reader) = tokio::io::duplex(4096);
    let (server_writer, client_reader) = tokio::io::duplex(4096);

    let server_reader = server_reader.compat();
    let server_writer = server_writer.compat_write();
    let client_reader = client_reader.compat();
    let client_writer = client_writer.compat_write();

    (server_reader, server_writer, client_reader, client_writer)
}

/// Await the next item on `rx`, panicking instead of hanging if it never
/// arrives.
async fn next_with_timeout<T>(rx: &mut mpsc::UnboundedReceiver<T>) -> T {
    tokio::time::timeout(tokio::time::Duration::from_secs(10), rx.next())
        .await
        .expect("timed out waiting for channel event")
        .expect("channel closed before expected event")
}

/// Assert that no item is currently buffered on `rx`.
///
/// Callers must first establish an ordering barrier (such as a
/// request/response round trip) that guarantees any erroneously sent
/// notification would already have been observed.
fn assert_no_event<T: std::fmt::Debug>(rx: &mut mpsc::UnboundedReceiver<T>) {
    if let Ok(event) = rx.try_recv() {
        panic!("unexpected event: {event:?}");
    }
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

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct WrappedHost;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct WrappedCounterpart;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct WrappedSuccessor;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct WrappedSuccessorCounterpart;

impl Role for WrappedHost {
    type Counterpart = WrappedCounterpart;

    fn role_id(&self) -> RoleId {
        RoleId::from_singleton(self)
    }

    async fn default_handle_dispatch_from(
        &self,
        message: Dispatch,
        _connection: ConnectionTo<Self>,
    ) -> Result<Handled<Dispatch>, agent_client_protocol::Error> {
        Ok(Handled::No {
            message,
            retry: false,
        })
    }

    fn counterpart(&self) -> Self::Counterpart {
        WrappedCounterpart
    }
}

impl Role for WrappedCounterpart {
    type Counterpart = WrappedHost;

    fn role_id(&self) -> RoleId {
        RoleId::from_singleton(self)
    }

    async fn default_handle_dispatch_from(
        &self,
        message: Dispatch,
        _connection: ConnectionTo<Self>,
    ) -> Result<Handled<Dispatch>, agent_client_protocol::Error> {
        Ok(Handled::No {
            message,
            retry: false,
        })
    }

    fn counterpart(&self) -> Self::Counterpart {
        WrappedHost
    }
}

impl Role for WrappedSuccessor {
    type Counterpart = WrappedSuccessorCounterpart;

    fn role_id(&self) -> RoleId {
        RoleId::from_singleton(self)
    }

    async fn default_handle_dispatch_from(
        &self,
        message: Dispatch,
        _connection: ConnectionTo<Self>,
    ) -> Result<Handled<Dispatch>, agent_client_protocol::Error> {
        Ok(Handled::No {
            message,
            retry: false,
        })
    }

    fn counterpart(&self) -> Self::Counterpart {
        WrappedSuccessorCounterpart
    }
}

impl Role for WrappedSuccessorCounterpart {
    type Counterpart = WrappedSuccessor;

    fn role_id(&self) -> RoleId {
        RoleId::from_singleton(self)
    }

    async fn default_handle_dispatch_from(
        &self,
        message: Dispatch,
        _connection: ConnectionTo<Self>,
    ) -> Result<Handled<Dispatch>, agent_client_protocol::Error> {
        Ok(Handled::No {
            message,
            retry: false,
        })
    }

    fn counterpart(&self) -> Self::Counterpart {
        WrappedSuccessor
    }
}

impl agent_client_protocol::role::HasPeer<WrappedCounterpart> for WrappedCounterpart {
    fn remote_style(&self, _peer: WrappedCounterpart) -> agent_client_protocol::role::RemoteStyle {
        agent_client_protocol::role::RemoteStyle::Counterpart
    }
}

impl agent_client_protocol::role::HasPeer<WrappedSuccessor> for WrappedCounterpart {
    fn remote_style(&self, _peer: WrappedSuccessor) -> agent_client_protocol::role::RemoteStyle {
        agent_client_protocol::role::RemoteStyle::Successor
    }
}

impl agent_client_protocol::role::HasPeer<WrappedSuccessor> for WrappedHost {
    fn remote_style(&self, _peer: WrappedSuccessor) -> agent_client_protocol::role::RemoteStyle {
        agent_client_protocol::role::RemoteStyle::Successor
    }
}

impl agent_client_protocol::role::HasPeer<WrappedHost> for WrappedHost {
    fn remote_style(&self, _peer: WrappedHost) -> agent_client_protocol::role::RemoteStyle {
        agent_client_protocol::role::RemoteStyle::Counterpart
    }
}

#[tokio::test(flavor = "current_thread")]
async fn unhandled_wrapped_protocol_level_notifications_are_ignored() {
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
            let server = WrappedHost
                .builder()
                .on_receive_notification_from(
                    WrappedSuccessor,
                    async |cancel: CancelRequestNotification,
                           cx: ConnectionTo<WrappedCounterpart>| {
                        Ok::<_, agent_client_protocol::Error>(Handled::No {
                            message: (cancel, cx),
                            retry: false,
                        })
                    },
                    agent_client_protocol::on_receive_notification!(),
                )
                .on_receive_request(
                    async |request: SimpleRequest,
                           responder: Responder<SimpleResponse>,
                           _connection: ConnectionTo<WrappedCounterpart>| {
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
                    br#"{"jsonrpc":"2.0","method":"_proxy/successor","params":{"method":"$/cancel_request","params":{"requestId":"req-1"}}}
"#,
                )
                .await
                .unwrap();
            client_writer.flush().await.unwrap();

            client_writer
                .write_all(
                    br#"{"jsonrpc":"2.0","id":2,"method":"simple_method","params":{"message":"after wrapped cancel"}}
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
                    "result": "echo: after wrapped cancel"
                  }
                }"#]]
            .assert_eq(&serde_json::to_string_pretty(&response).unwrap());
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn wrapped_cancel_request_cancels_wrapped_request() {
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
            let server = WrappedHost.builder().on_receive_request_from(
                WrappedSuccessor,
                async |_request: SimpleRequest,
                       responder: Responder<SimpleResponse>,
                       cx: ConnectionTo<WrappedCounterpart>| {
                    let cancellation = responder.cancellation();
                    cx.spawn(async move {
                        let response = cancellation
                            .run_until_cancelled(futures::future::pending::<
                                Result<SimpleResponse, agent_client_protocol::Error>,
                            >())
                            .await;
                        responder.respond_with_result(response)
                    })?;
                    Ok(())
                },
                agent_client_protocol::on_receive_request!(),
            );

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_transport).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            let mut client_reader = BufReader::new(client_reader);

            // A request wrapped in a successor envelope is registered under
            // its outer JSON-RPC id, so a wrapped `$/cancel_request` for that
            // outer id must cancel it.
            client_writer
                .write_all(
                    br#"{"jsonrpc":"2.0","id":7,"method":"_proxy/successor","params":{"method":"simple_method","params":{"message":"wrapped"}}}
"#,
                )
                .await
                .unwrap();
            client_writer.flush().await.unwrap();

            client_writer
                .write_all(
                    br#"{"jsonrpc":"2.0","method":"_proxy/successor","params":{"method":"$/cancel_request","params":{"requestId":7}}}
"#,
                )
                .await
                .unwrap();
            client_writer.flush().await.unwrap();

            let response = read_jsonrpc_response_line(&mut client_reader).await;
            expect![[r#"
                {
                  "jsonrpc": "2.0",
                  "id": 7,
                  "error": {
                    "code": -32800,
                    "message": "Request cancelled"
                  }
                }"#]]
            .assert_eq(&serde_json::to_string_pretty(&response).unwrap());
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn cancelling_request_sent_to_successor_peer_sends_wrapped_cancel() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (wrapped_cancel_tx, mut wrapped_cancel_rx) = mpsc::unbounded();
            let (plain_cancel_tx, mut plain_cancel_rx) = mpsc::unbounded();

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = WrappedHost
                .builder()
                .on_receive_request_from(
                    WrappedSuccessor,
                    async |_request: SimpleRequest,
                           responder: Responder<SimpleResponse>,
                           cx: ConnectionTo<WrappedCounterpart>| {
                        let cancellation = responder.cancellation();
                        cx.spawn(async move {
                            let response = cancellation
                                .run_until_cancelled(futures::future::pending::<
                                    Result<SimpleResponse, agent_client_protocol::Error>,
                                >())
                                .await;
                            responder.respond_with_result(response)
                        })?;
                        Ok(())
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                // Matches only a `$/cancel_request` wrapped in a
                // `_proxy/successor` envelope: observing it here proves the
                // client wrapped the outgoing cancellation the same way as
                // the request it refers to.
                .on_receive_notification_from(
                    WrappedSuccessor,
                    async move |cancel: CancelRequestNotification,
                                _cx: ConnectionTo<WrappedCounterpart>| {
                        wrapped_cancel_tx.unbounded_send(cancel.request_id).unwrap();
                        Ok(())
                    },
                    agent_client_protocol::on_receive_notification!(),
                )
                // Matches only an *unwrapped* `$/cancel_request`; the client
                // must never send one for a successor-wrapped request.
                .on_receive_notification(
                    async move |cancel: CancelRequestNotification,
                                _cx: ConnectionTo<WrappedCounterpart>| {
                        plain_cancel_tx.unbounded_send(cancel.request_id).unwrap();
                        Ok(())
                    },
                    agent_client_protocol::on_receive_notification!(),
                );

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_transport).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let (expected_id, error) = WrappedCounterpart
                .builder()
                .connect_with(client_transport, async |cx| {
                    let request: SentRequest<SimpleResponse> = cx.send_request_to(
                        WrappedSuccessor,
                        SimpleRequest {
                            message: "wrapped cancel".into(),
                        },
                    );
                    let expected_id = request.id().clone();
                    request.cancel()?;
                    let error = request
                        .block_task()
                        .await
                        .expect_err("request should be cancelled");
                    Ok((expected_id, error))
                })
                .await
                .unwrap();

            assert_eq!(i32::from(error.code), -32800);

            // The cancellation arrived wrapped, for the wrapped request's
            // outer JSON-RPC id, and never in unwrapped form.
            let received = next_with_timeout(&mut wrapped_cancel_rx).await;
            assert_eq!(received, expected_id);
            assert_no_event(&mut wrapped_cancel_rx);
            assert_no_event(&mut plain_cancel_rx);
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn cancel_request_notification_can_be_sent_and_handled() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (cancel_tx, mut cancel_rx) = mpsc::unbounded();

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole.builder().on_receive_notification(
                async move |notification: CancelRequestNotification,
                            _connection: ConnectionTo<UntypedRole>| {
                    cancel_tx.unbounded_send(notification.request_id).unwrap();
                    Ok(())
                },
                agent_client_protocol::on_receive_notification!(),
            );

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_transport).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let received = UntypedRole
                .builder()
                .connect_with(client_transport, async |cx| {
                    cx.send_cancel_request("request-42".to_string())?;
                    Ok(next_with_timeout(&mut cancel_rx).await)
                })
                .await
                .unwrap();

            assert_eq!(received, RequestId::Str("request-42".into()));
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn sent_request_can_send_cancellation_for_its_id() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (cancel_tx, mut cancel_rx) = mpsc::unbounded();

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole
                .builder()
                .on_receive_request(
                    async |request: SimpleRequest,
                           responder: Responder<SimpleResponse>,
                           _connection: ConnectionTo<UntypedRole>| {
                        if request.message == "barrier" {
                            return responder.respond(SimpleResponse {
                                result: format!("echo: {}", request.message),
                            });
                        }
                        // Park other requests (by dropping the responder) so
                        // the cancelled request is never answered and the
                        // client handle stays unconsumed.
                        Ok(())
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_notification(
                    async move |notification: CancelRequestNotification,
                                _connection: ConnectionTo<UntypedRole>| {
                        cancel_tx.unbounded_send(notification.request_id).unwrap();
                        Ok(())
                    },
                    agent_client_protocol::on_receive_notification!(),
                );

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_transport).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let (expected_id, received) = UntypedRole
                .builder()
                .connect_with(client_transport, async |cx| {
                    let request: SentRequest<SimpleResponse> = cx.send_request(SimpleRequest {
                        message: "slow".into(),
                    });
                    let expected_id = request.id().clone();
                    request.cancel()?;
                    let received = next_with_timeout(&mut cancel_rx).await;

                    // Dropping the handle after an explicit cancel must not
                    // send a second `$/cancel_request`.
                    drop(request);

                    // Barrier round trip: a duplicate cancel sent by the drop
                    // above would reach the server before this request.
                    let barrier = cx
                        .send_request(SimpleRequest {
                            message: "barrier".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(barrier.result, "echo: barrier");

                    Ok((expected_id, received))
                })
                .await
                .unwrap();

            assert_eq!(received, expected_id);
            assert_no_event(&mut cancel_rx);
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn dropped_sent_request_sends_cancellation_for_its_id() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (cancel_tx, mut cancel_rx) = mpsc::unbounded();

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole
                .builder()
                .on_receive_request(
                    async |_request: SimpleRequest,
                           _responder: Responder<SimpleResponse>,
                           _connection: ConnectionTo<UntypedRole>| { Ok(()) },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_notification(
                    async move |notification: CancelRequestNotification,
                                _connection: ConnectionTo<UntypedRole>| {
                        cancel_tx.unbounded_send(notification.request_id).unwrap();
                        Ok(())
                    },
                    agent_client_protocol::on_receive_notification!(),
                );

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_transport).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let (expected_id, received) = UntypedRole
                .builder()
                .connect_with(client_transport, async |cx| {
                    let request: SentRequest<SimpleResponse> = cx.send_request(SimpleRequest {
                        message: "abandoned".into(),
                    });
                    let expected_id = request.id().clone();
                    drop(request);
                    let received = next_with_timeout(&mut cancel_rx).await;
                    Ok((expected_id, received))
                })
                .await
                .unwrap();

            assert_eq!(received, expected_id);
            assert_no_event(&mut cancel_rx);
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn detached_sent_request_does_not_send_cancellation() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (cancel_tx, mut cancel_rx) = mpsc::unbounded();

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole
                .builder()
                .on_receive_request(
                    async |request: SimpleRequest,
                           responder: Responder<SimpleResponse>,
                           _connection: ConnectionTo<UntypedRole>| {
                        if request.message == "barrier" {
                            return responder.respond(SimpleResponse {
                                result: format!("echo: {}", request.message),
                            });
                        }

                        Ok(())
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_notification(
                    async move |notification: CancelRequestNotification,
                                _connection: ConnectionTo<UntypedRole>| {
                        cancel_tx.unbounded_send(notification.request_id).unwrap();
                        Ok(())
                    },
                    agent_client_protocol::on_receive_notification!(),
                );

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_transport).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            UntypedRole
                .builder()
                .connect_with(client_transport, async |cx| {
                    cx.send_request(SimpleRequest {
                        message: "detached".into(),
                    })
                    .detach();

                    // Barrier round trip: a cancellation sent by dropping the
                    // detached handle would reach the server before this
                    // request.
                    let barrier = cx
                        .send_request(SimpleRequest {
                            message: "barrier".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(barrier.result, "echo: barrier");

                    Ok(())
                })
                .await
                .unwrap();

            assert_no_event(&mut cancel_rx);
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn late_response_after_dropped_sent_request_does_not_close_connection() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (cancel_tx, mut cancel_rx) = mpsc::unbounded();
            // The responder for the abandoned request, held by the server
            // until the cancellation notification arrives.
            let pending_responder: Arc<Mutex<Option<Responder<SimpleResponse>>>> =
                Arc::new(Mutex::new(None));

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole
                .builder()
                .on_receive_request(
                    {
                        let pending_responder = pending_responder.clone();
                        async move |request: SimpleRequest,
                                    responder: Responder<SimpleResponse>,
                                    _connection: ConnectionTo<UntypedRole>| {
                            if request.message == "late" {
                                *pending_responder.lock().unwrap() = Some(responder);
                                return Ok(());
                            }

                            responder.respond(SimpleResponse {
                                result: format!("echo: {}", request.message),
                            })
                        }
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_notification(
                    {
                        let pending_responder = pending_responder.clone();
                        async move |notification: CancelRequestNotification,
                                    _connection: ConnectionTo<UntypedRole>| {
                            // Ignore the cancellation and answer the abandoned
                            // request anyway: the client must tolerate this.
                            if let Some(responder) = pending_responder.lock().unwrap().take() {
                                responder.respond(SimpleResponse {
                                    result: "late response".into(),
                                })?;
                            }
                            cancel_tx.unbounded_send(notification.request_id).unwrap();
                            Ok(())
                        }
                    },
                    agent_client_protocol::on_receive_notification!(),
                );

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_transport).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let (expected_id, received, response) = UntypedRole
                .builder()
                .connect_with(client_transport, async |cx| {
                    let request: SentRequest<SimpleResponse> = cx.send_request(SimpleRequest {
                        message: "late".into(),
                    });
                    let expected_id = request.id().clone();
                    drop(request);

                    let received = next_with_timeout(&mut cancel_rx).await;

                    // The server sent the late response before answering this
                    // follow-up, so a successful round trip proves the late
                    // response for the dropped request was routed without
                    // closing the connection.
                    let response = cx
                        .send_request(SimpleRequest {
                            message: "after late".into(),
                        })
                        .block_task()
                        .await?;
                    Ok((expected_id, received, response))
                })
                .await
                .unwrap();

            assert_eq!(response.result, "echo: after late");
            assert_eq!(received, expected_id);
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn response_buffered_before_drop_disarms_auto_cancellation() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (cancel_tx, mut cancel_rx) = mpsc::unbounded();

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole
                .builder()
                .on_receive_request(
                    async |request: SimpleRequest,
                           responder: Responder<SimpleResponse>,
                           _connection: ConnectionTo<UntypedRole>| {
                        responder.respond(SimpleResponse {
                            result: format!("echo: {}", request.message),
                        })
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_notification(
                    async move |notification: CancelRequestNotification,
                                _connection: ConnectionTo<UntypedRole>| {
                        cancel_tx.unbounded_send(notification.request_id).unwrap();
                        Ok(())
                    },
                    agent_client_protocol::on_receive_notification!(),
                );

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_transport).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let response = UntypedRole
                .builder()
                .connect_with(client_transport, async |cx| {
                    let request: SentRequest<SimpleResponse> = cx.send_request(SimpleRequest {
                        message: "buffered".into(),
                    });

                    // The server answers requests in order, so once this round
                    // trip completes, the response to `buffered` has already
                    // been routed into the unconsumed request handle above,
                    // disarming its auto-cancellation.
                    let barrier = cx
                        .send_request(SimpleRequest {
                            message: "barrier".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(barrier.result, "echo: barrier");

                    drop(request);

                    // Another round trip: any cancellation sent by the drop
                    // above would reach the server before this request.
                    cx.send_request(SimpleRequest {
                        message: "after buffered".into(),
                    })
                    .block_task()
                    .await
                })
                .await
                .unwrap();

            assert_eq!(response.result, "echo: after buffered");
            assert_no_event(&mut cancel_rx);
        })
        .await;
}

/// A dispatch handler may claim a `Dispatch::Response` and drop the router
/// without invoking it. Routing the response settles the request all the
/// same, so dropping the (never-delivered-to) request handle afterwards must
/// not ask the peer to cancel a request it has already answered.
#[tokio::test(flavor = "current_thread")]
async fn response_claimed_by_dispatch_handler_disarms_auto_cancellation() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (cancel_tx, mut cancel_rx) = mpsc::unbounded();

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole
                .builder()
                .on_receive_request(
                    async |request: SimpleRequest,
                           responder: Responder<SimpleResponse>,
                           _connection: ConnectionTo<UntypedRole>| {
                        responder.respond(SimpleResponse {
                            result: format!("echo: {}", request.message),
                        })
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_notification(
                    async move |notification: CancelRequestNotification,
                                _connection: ConnectionTo<UntypedRole>| {
                        cancel_tx.unbounded_send(notification.request_id).unwrap();
                        Ok(())
                    },
                    agent_client_protocol::on_receive_notification!(),
                );

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_transport).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            // The JSON-RPC id whose response the dispatch handler below
            // claims (and discards) without ever invoking the router.
            let claimed_id: Arc<Mutex<Option<RequestId>>> = Arc::new(Mutex::new(None));

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            UntypedRole
                .builder()
                .on_receive_dispatch(
                    {
                        let claimed_id = claimed_id.clone();
                        async move |dispatch: Dispatch, _connection: ConnectionTo<UntypedRole>| {
                            if let Dispatch::Response(_, router) = &dispatch
                                && claimed_id.lock().unwrap().as_ref() == Some(router.id())
                            {
                                // Claim the response; the router is dropped
                                // without responding.
                                return Ok(Handled::Yes);
                            }
                            Ok(Handled::No {
                                message: dispatch,
                                retry: false,
                            })
                        }
                    },
                    agent_client_protocol::on_receive_dispatch!(),
                )
                .connect_with(client_transport, async |cx| {
                    let request: SentRequest<SimpleResponse> = cx.send_request(SimpleRequest {
                        message: "claimed".into(),
                    });
                    *claimed_id.lock().unwrap() = Some(request.id().clone());

                    // The server answers requests in order, so once this
                    // round trip completes, the response to `claimed` has
                    // been routed and discarded by the dispatch handler.
                    let barrier = cx
                        .send_request(SimpleRequest {
                            message: "barrier".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(barrier.result, "echo: barrier");

                    drop(request);

                    // Another round trip: any cancellation sent by the drop
                    // above would reach the server before this request.
                    let after = cx
                        .send_request(SimpleRequest {
                            message: "after claimed".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(after.result, "echo: after claimed");
                    Ok(())
                })
                .await
                .unwrap();

            assert_no_event(&mut cancel_rx);
        })
        .await;
}

/// A dispatch handler may keep the `ResponseRouter` alive after the peer has
/// answered. The original `SentRequest` is settled as soon as the response is
/// routed into the handler, so dropping it must not ask the peer to cancel.
#[tokio::test(flavor = "current_thread")]
async fn response_retained_by_dispatch_handler_disarms_auto_cancellation() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (cancel_tx, mut cancel_rx) = mpsc::unbounded();

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole
                .builder()
                .on_receive_request(
                    async |request: SimpleRequest,
                           responder: Responder<SimpleResponse>,
                           _connection: ConnectionTo<UntypedRole>| {
                        responder.respond(SimpleResponse {
                            result: format!("echo: {}", request.message),
                        })
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_notification(
                    async move |notification: CancelRequestNotification,
                                _connection: ConnectionTo<UntypedRole>| {
                        cancel_tx.unbounded_send(notification.request_id).unwrap();
                        Ok(())
                    },
                    agent_client_protocol::on_receive_notification!(),
                );

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_transport).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            let claimed_id: Arc<Mutex<Option<RequestId>>> = Arc::new(Mutex::new(None));
            let retained_response: Arc<Mutex<Option<Dispatch>>> = Arc::new(Mutex::new(None));

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            UntypedRole
                .builder()
                .on_receive_dispatch(
                    {
                        let claimed_id = claimed_id.clone();
                        let retained_response = retained_response.clone();
                        async move |dispatch: Dispatch, _connection: ConnectionTo<UntypedRole>| {
                            let should_claim = match &dispatch {
                                Dispatch::Response(_, router) => {
                                    claimed_id.lock().unwrap().as_ref() == Some(router.id())
                                }
                                Dispatch::Request(_, _) | Dispatch::Notification(_) => false,
                            };

                            if should_claim {
                                *retained_response.lock().unwrap() = Some(dispatch);
                                return Ok(Handled::Yes);
                            }

                            Ok(Handled::No {
                                message: dispatch,
                                retry: false,
                            })
                        }
                    },
                    agent_client_protocol::on_receive_dispatch!(),
                )
                .connect_with(client_transport, async |cx| {
                    let request: SentRequest<SimpleResponse> = cx.send_request(SimpleRequest {
                        message: "retained".into(),
                    });
                    *claimed_id.lock().unwrap() = Some(request.id().clone());

                    // This proves the earlier response was routed into the
                    // handler and is still retained rather than dropped.
                    let barrier = cx
                        .send_request(SimpleRequest {
                            message: "barrier".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(barrier.result, "echo: barrier");
                    assert!(retained_response.lock().unwrap().is_some());

                    drop(request);

                    // Any auto-cancel from dropping the request would be
                    // delivered before this follow-up request.
                    let after = cx
                        .send_request(SimpleRequest {
                            message: "after retained".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(after.result, "echo: after retained");
                    Ok(())
                })
                .await
                .unwrap();

            assert_no_event(&mut cancel_rx);
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn completed_sent_request_does_not_send_cancellation_on_drop() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (cancel_tx, mut cancel_rx) = mpsc::unbounded();

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole
                .builder()
                .on_receive_request(
                    async |request: SimpleRequest,
                           responder: Responder<SimpleResponse>,
                           _connection: ConnectionTo<UntypedRole>| {
                        responder.respond(SimpleResponse {
                            result: format!("echo: {}", request.message),
                        })
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_notification(
                    async move |notification: CancelRequestNotification,
                                _connection: ConnectionTo<UntypedRole>| {
                        cancel_tx.unbounded_send(notification.request_id).unwrap();
                        Ok(())
                    },
                    agent_client_protocol::on_receive_notification!(),
                );

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_transport).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let response = UntypedRole
                .builder()
                .connect_with(client_transport, async |cx| {
                    let response = cx
                        .send_request(SimpleRequest {
                            message: "complete".into(),
                        })
                        .block_task()
                        .await?;

                    // Barrier round trip: any cancellation erroneously sent
                    // when the completed request handle was dropped would
                    // reach the server before this request.
                    let barrier = cx
                        .send_request(SimpleRequest {
                            message: "barrier".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(barrier.result, "echo: barrier");

                    Ok(response)
                })
                .await
                .unwrap();

            assert_eq!(response.result, "echo: complete");
            assert_no_event(&mut cancel_rx);
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn forward_response_to_propagates_cancellation_to_downstream_request() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (backend_cancel_tx, mut backend_cancel_rx) = mpsc::unbounded();
            // The responder for the cancelled request, parked by the backend
            // until the forwarded cancellation arrives.
            let pending_responder: Arc<Mutex<Option<Responder<SimpleResponse>>>> =
                Arc::new(Mutex::new(None));

            let (backend_for_proxy, backend_for_server) = Channel::duplex();
            let (backend_connection_tx, backend_connection_rx) =
                futures::channel::oneshot::channel();

            tokio::task::spawn_local(async move {
                let result = UntypedRole
                    .builder()
                    .connect_with(backend_for_proxy, async |connection| {
                        drop(backend_connection_tx.send(connection.clone()));
                        std::future::pending::<Result<(), agent_client_protocol::Error>>().await
                    })
                    .await;
                if let Err(error) = result {
                    panic!("proxy-to-backend connection should stay alive: {error:?}");
                }
            });

            let backend_server = UntypedRole
                .builder()
                .on_receive_request(
                    {
                        let pending_responder = pending_responder.clone();
                        async move |request: SimpleRequest,
                                    responder: Responder<SimpleResponse>,
                                    _connection: ConnectionTo<UntypedRole>| {
                            if request.message == "cancel downstream" {
                                *pending_responder.lock().unwrap() = Some(responder);
                                return Ok(());
                            }

                            responder.respond(SimpleResponse {
                                result: format!("echo: {}", request.message),
                            })
                        }
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_notification(
                    {
                        let pending_responder = pending_responder.clone();
                        async move |notification: CancelRequestNotification,
                                    _connection: ConnectionTo<UntypedRole>| {
                            // Honor the forwarded cancellation: answer the
                            // parked request with the cancellation error.
                            if let Some(responder) = pending_responder.lock().unwrap().take() {
                                responder.respond_with_result(Err(
                                    agent_client_protocol::Error::request_cancelled(),
                                ))?;
                            }
                            backend_cancel_tx
                                .unbounded_send(notification.request_id)
                                .unwrap();
                            Ok(())
                        }
                    },
                    agent_client_protocol::on_receive_notification!(),
                );

            tokio::task::spawn_local(async move {
                if let Err(error) = backend_server.connect_to(backend_for_server).await {
                    panic!("backend server should stay alive: {error:?}");
                }
            });

            let backend_connection = backend_connection_rx
                .await
                .expect("backend connection should start");

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let proxy_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let proxy = UntypedRole.builder().on_receive_request(
                {
                    let backend_connection = backend_connection.clone();
                    async move |request: SimpleRequest,
                                responder: Responder<SimpleResponse>,
                                _connection: ConnectionTo<UntypedRole>| {
                        backend_connection
                            .send_request(request)
                            .forward_response_to(responder)?;
                        Ok(())
                    }
                },
                agent_client_protocol::on_receive_request!(),
            );

            tokio::task::spawn_local(async move {
                if let Err(error) = proxy.connect_to(proxy_transport).await {
                    panic!("proxy should stay alive: {error:?}");
                }
            });

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            UntypedRole
                .builder()
                .connect_with(client_transport, async |connection| {
                    let request: SentRequest<SimpleResponse> =
                        connection.send_request(SimpleRequest {
                            message: "cancel downstream".into(),
                        });
                    request.cancel()?;

                    // The backend answers the parked request only once the
                    // proxy has forwarded the cancellation to it, and the
                    // proxy forwards the backend's cancellation error back
                    // upstream as the response.
                    let error = request
                        .block_task()
                        .await
                        .expect_err("request should be cancelled");
                    assert_eq!(i32::from(error.code), -32800);
                    next_with_timeout(&mut backend_cancel_rx).await;

                    // Barrier: this round trip traverses both hops after the
                    // cancellation, so a duplicate `$/cancel_request` would
                    // already have been recorded by the backend.
                    let barrier = connection
                        .send_request(SimpleRequest {
                            message: "barrier".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(barrier.result, "echo: barrier");
                    Ok(())
                })
                .await
                .unwrap();

            assert_no_event(&mut backend_cancel_rx);
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn send_proxied_message_does_not_tunnel_cancel_notifications() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (backend_cancel_tx, mut backend_cancel_rx) = mpsc::unbounded();
            // The downstream JSON-RPC id of the parked request, as seen by
            // the backend.
            let (parked_id_tx, mut parked_id_rx) = mpsc::unbounded();
            // The responder for the cancelled request, parked by the backend
            // until the forwarded cancellation arrives.
            let pending_responder: Arc<Mutex<Option<Responder<SimpleResponse>>>> =
                Arc::new(Mutex::new(None));

            let (backend_for_proxy, backend_for_server) = Channel::duplex();
            let (backend_connection_tx, backend_connection_rx) =
                futures::channel::oneshot::channel();

            tokio::task::spawn_local(async move {
                let result = UntypedRole
                    .builder()
                    .connect_with(backend_for_proxy, async |connection| {
                        drop(backend_connection_tx.send(connection.clone()));
                        std::future::pending::<Result<(), agent_client_protocol::Error>>().await
                    })
                    .await;
                if let Err(error) = result {
                    panic!("proxy-to-backend connection should stay alive: {error:?}");
                }
            });

            let backend_server = UntypedRole
                .builder()
                .on_receive_request(
                    {
                        let pending_responder = pending_responder.clone();
                        async move |request: SimpleRequest,
                                    responder: Responder<SimpleResponse>,
                                    _connection: ConnectionTo<UntypedRole>| {
                            if request.message == "park" {
                                parked_id_tx.unbounded_send(responder.id().clone()).unwrap();
                                *pending_responder.lock().unwrap() = Some(responder);
                                return Ok(());
                            }

                            responder.respond(SimpleResponse {
                                result: format!("echo: {}", request.message),
                            })
                        }
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_notification(
                    {
                        let pending_responder = pending_responder.clone();
                        async move |notification: CancelRequestNotification,
                                    _connection: ConnectionTo<UntypedRole>| {
                            // Honor the cancellation: answer the parked
                            // request with the cancellation error.
                            if let Some(responder) = pending_responder.lock().unwrap().take() {
                                responder.respond_with_result(Err(
                                    agent_client_protocol::Error::request_cancelled(),
                                ))?;
                            }
                            backend_cancel_tx
                                .unbounded_send(notification.request_id)
                                .unwrap();
                            Ok(())
                        }
                    },
                    agent_client_protocol::on_receive_notification!(),
                );

            tokio::task::spawn_local(async move {
                if let Err(error) = backend_server.connect_to(backend_for_server).await {
                    panic!("backend server should stay alive: {error:?}");
                }
            });

            let backend_connection = backend_connection_rx
                .await
                .expect("backend connection should start");

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let proxy_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            // The proxy forwards *every* incoming dispatch with
            // `send_proxied_message`. Without the hop-scoped filter, the
            // client's raw `$/cancel_request` (whose request ID only means
            // something on the client-to-proxy connection) would be tunneled
            // to the backend verbatim, alongside the cancellation that
            // `forward_response_to` re-issues with the downstream ID.
            let proxy = UntypedRole.builder().on_receive_dispatch(
                {
                    let backend_connection = backend_connection.clone();
                    async move |dispatch: Dispatch, _connection: ConnectionTo<UntypedRole>| {
                        backend_connection.send_proxied_message(dispatch)
                    }
                },
                agent_client_protocol::on_receive_dispatch!(),
            );

            tokio::task::spawn_local(async move {
                if let Err(error) = proxy.connect_to(proxy_transport).await {
                    panic!("proxy should stay alive: {error:?}");
                }
            });

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let client_request_id = UntypedRole
                .builder()
                .connect_with(client_transport, async |connection| {
                    let request: SentRequest<SimpleResponse> =
                        connection.send_request(SimpleRequest {
                            message: "park".into(),
                        });
                    let client_request_id = request.id().clone();
                    request.cancel()?;

                    let error = request
                        .block_task()
                        .await
                        .expect_err("request should be cancelled");
                    assert_eq!(i32::from(error.code), -32800);

                    // Barrier: this round trip traverses both hops after the
                    // cancellation, so a tunneled raw `$/cancel_request`
                    // would already have been recorded by the backend.
                    let barrier = connection
                        .send_request(SimpleRequest {
                            message: "barrier".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(barrier.result, "echo: barrier");
                    Ok(client_request_id)
                })
                .await
                .unwrap();

            // The backend saw exactly one `$/cancel_request`: the one
            // re-issued for the downstream request, not the client's raw
            // notification with its hop-local request ID.
            let parked_id = next_with_timeout(&mut parked_id_rx).await;
            assert_ne!(
                parked_id, client_request_id,
                "the proxy must re-issue the request under its own ID"
            );
            let observed = next_with_timeout(&mut backend_cancel_rx).await;
            assert_eq!(observed, parked_id);
            assert_no_event(&mut backend_cancel_rx);
        })
        .await;
}

/// A proxy that forwards raw dispatches with `send_proxied_message` can see a
/// `$/cancel_request` that is still wrapped in a `_proxy/successor` envelope:
/// raw dispatch handlers run before any peer-specific unwrapping. The
/// hop-scoped filter must peel the envelope and drop the notification rather
/// than tunnel it to the next peer.
#[tokio::test(flavor = "current_thread")]
async fn send_proxied_message_does_not_tunnel_wrapped_cancel_notifications() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            // Every notification method the backend observes.
            let (backend_notification_tx, mut backend_notification_rx) = mpsc::unbounded();

            let (backend_for_proxy, backend_for_server) = Channel::duplex();
            let (backend_connection_tx, backend_connection_rx) =
                futures::channel::oneshot::channel();

            tokio::task::spawn_local(async move {
                let result = UntypedRole
                    .builder()
                    .connect_with(backend_for_proxy, async |connection| {
                        drop(backend_connection_tx.send(connection.clone()));
                        std::future::pending::<Result<(), agent_client_protocol::Error>>().await
                    })
                    .await;
                if let Err(error) = result {
                    panic!("proxy-to-backend connection should stay alive: {error:?}");
                }
            });

            let backend_server = UntypedRole
                .builder()
                .on_receive_request(
                    async |request: SimpleRequest,
                           responder: Responder<SimpleResponse>,
                           _connection: ConnectionTo<UntypedRole>| {
                        responder.respond(SimpleResponse {
                            result: format!("echo: {}", request.message),
                        })
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_notification(
                    async move |notification: agent_client_protocol::UntypedMessage,
                                _connection: ConnectionTo<UntypedRole>| {
                        backend_notification_tx
                            .unbounded_send(notification.method)
                            .unwrap();
                        Ok(())
                    },
                    agent_client_protocol::on_receive_notification!(),
                );

            tokio::task::spawn_local(async move {
                if let Err(error) = backend_server.connect_to(backend_for_server).await {
                    panic!("backend server should stay alive: {error:?}");
                }
            });

            let backend_connection = backend_connection_rx
                .await
                .expect("backend connection should start");

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let proxy_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            // The raw dispatch handler receives successor-addressed messages
            // still wrapped in their envelope and forwards them verbatim.
            let proxy = WrappedHost.builder().on_receive_dispatch(
                {
                    let backend_connection = backend_connection.clone();
                    async move |dispatch: Dispatch,
                                _connection: ConnectionTo<WrappedCounterpart>| {
                        backend_connection.send_proxied_message(dispatch)
                    }
                },
                agent_client_protocol::on_receive_dispatch!(),
            );

            tokio::task::spawn_local(async move {
                if let Err(error) = proxy.connect_to(proxy_transport).await {
                    panic!("proxy should stay alive: {error:?}");
                }
            });

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            WrappedCounterpart
                .builder()
                .connect_with(client_transport, async |cx| {
                    // A successor-wrapped `$/cancel_request`, exactly as
                    // produced when cancelling a request sent to a successor
                    // peer.
                    cx.send_cancel_request_to(WrappedSuccessor, "req-1".to_string())?;

                    // Barrier: both hops have processed the notification by
                    // the time this completes, so a tunneled wrapped cancel
                    // would already have been recorded by the backend.
                    let barrier = cx
                        .send_request(SimpleRequest {
                            message: "barrier".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(barrier.result, "echo: barrier");
                    Ok(())
                })
                .await
                .unwrap();

            // The backend saw no notification at all: the wrapped cancel was
            // dropped at the proxy hop instead of being tunneled.
            assert_no_event(&mut backend_notification_rx);
        })
        .await;
}

/// Spawn a backend whose `park` requests wait until the cancel observer
/// releases them, reporting parked request ids and observed cancellations.
///
/// Returns the proxy-side connection to the backend.
async fn spawn_parking_backend(
    honor_cancellations: bool,
    parked_id_tx: mpsc::UnboundedSender<RequestId>,
    backend_cancel_tx: mpsc::UnboundedSender<RequestId>,
) -> ConnectionTo<UntypedRole> {
    let pending_responder: Arc<Mutex<Option<Responder<SimpleResponse>>>> =
        Arc::new(Mutex::new(None));

    let (backend_for_proxy, backend_for_server) = Channel::duplex();
    let (backend_connection_tx, backend_connection_rx) = futures::channel::oneshot::channel();

    tokio::task::spawn_local(async move {
        let result = UntypedRole
            .builder()
            .connect_with(backend_for_proxy, async |connection| {
                drop(backend_connection_tx.send(connection.clone()));
                std::future::pending::<Result<(), agent_client_protocol::Error>>().await
            })
            .await;
        if let Err(error) = result {
            panic!("proxy-to-backend connection should stay alive: {error:?}");
        }
    });

    let backend_server = UntypedRole
        .builder()
        .on_receive_request(
            {
                let pending_responder = pending_responder.clone();
                async move |request: SimpleRequest,
                            responder: Responder<SimpleResponse>,
                            _connection: ConnectionTo<UntypedRole>| {
                    match request.message.as_str() {
                        "park" => {
                            parked_id_tx.unbounded_send(responder.id().clone()).unwrap();
                            *pending_responder.lock().unwrap() = Some(responder);
                            Ok(())
                        }
                        "release" => {
                            if let Some(parked) = pending_responder.lock().unwrap().take() {
                                parked.respond(SimpleResponse {
                                    result: "released".into(),
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
        )
        .on_receive_notification(
            {
                let pending_responder = pending_responder.clone();
                async move |notification: CancelRequestNotification,
                            _connection: ConnectionTo<UntypedRole>| {
                    if honor_cancellations
                        && let Some(responder) = pending_responder.lock().unwrap().take()
                    {
                        responder.respond_with_result(Err(
                            agent_client_protocol::Error::request_cancelled(),
                        ))?;
                    }
                    backend_cancel_tx
                        .unbounded_send(notification.request_id)
                        .unwrap();
                    Ok(())
                }
            },
            agent_client_protocol::on_receive_notification!(),
        );

    tokio::task::spawn_local(async move {
        if let Err(error) = backend_server.connect_to(backend_for_server).await {
            panic!("backend server should stay alive: {error:?}");
        }
    });

    backend_connection_rx
        .await
        .expect("backend connection should start")
}

#[tokio::test(flavor = "current_thread")]
async fn custom_forwarding_propagates_cancellation_when_opted_in() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (backend_cancel_tx, mut backend_cancel_rx) = mpsc::unbounded();
            let (parked_id_tx, mut parked_id_rx) = mpsc::unbounded();

            let backend_connection =
                spawn_parking_backend(true, parked_id_tx, backend_cancel_tx).await;

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let proxy_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            // A proxy with a *custom* method handler: it forwards with
            // `on_receiving_result` (so it could post-process the result) and
            // opts into cancellation propagation explicitly.
            let proxy = UntypedRole.builder().on_receive_request(
                {
                    let backend_connection = backend_connection.clone();
                    async move |request: SimpleRequest,
                                responder: Responder<SimpleResponse>,
                                _connection: ConnectionTo<UntypedRole>| {
                        backend_connection
                            .send_request(request)
                            .forward_cancellation_from(responder.cancellation())
                            .on_receiving_result(async move |result| {
                                responder.respond_with_result(result)
                            })
                    }
                },
                agent_client_protocol::on_receive_request!(),
            );

            tokio::task::spawn_local(async move {
                if let Err(error) = proxy.connect_to(proxy_transport).await {
                    panic!("proxy should stay alive: {error:?}");
                }
            });

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let client_request_id = UntypedRole
                .builder()
                .connect_with(client_transport, async |connection| {
                    let request: SentRequest<SimpleResponse> =
                        connection.send_request(SimpleRequest {
                            message: "park".into(),
                        });
                    let client_request_id = request.id().clone();
                    request.cancel()?;

                    let error = request
                        .block_task()
                        .await
                        .expect_err("request should be cancelled");
                    assert_eq!(i32::from(error.code), -32800);

                    let barrier = connection
                        .send_request(SimpleRequest {
                            message: "barrier".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(barrier.result, "echo: barrier");
                    Ok(client_request_id)
                })
                .await
                .unwrap();

            // Exactly one cancellation reached the backend, re-issued under
            // the proxy's downstream request ID.
            let parked_id = next_with_timeout(&mut parked_id_rx).await;
            assert_ne!(parked_id, client_request_id);
            let observed = next_with_timeout(&mut backend_cancel_rx).await;
            assert_eq!(observed, parked_id);
            assert_no_event(&mut backend_cancel_rx);
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn custom_forwarding_absorbs_cancellation_by_default() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (backend_cancel_tx, mut backend_cancel_rx) = mpsc::unbounded();
            let (parked_id_tx, mut parked_id_rx) = mpsc::unbounded();

            let backend_connection =
                spawn_parking_backend(false, parked_id_tx, backend_cancel_tx).await;

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let proxy_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            // The same custom forwarding *without* opting into propagation:
            // the implementor decided cancellation stops at this hop.
            let proxy = UntypedRole.builder().on_receive_request(
                {
                    let backend_connection = backend_connection.clone();
                    async move |request: SimpleRequest,
                                responder: Responder<SimpleResponse>,
                                _connection: ConnectionTo<UntypedRole>| {
                        backend_connection
                            .send_request(request)
                            .on_receiving_result(async move |result| {
                                responder.respond_with_result(result)
                            })
                    }
                },
                agent_client_protocol::on_receive_request!(),
            );

            tokio::task::spawn_local(async move {
                if let Err(error) = proxy.connect_to(proxy_transport).await {
                    panic!("proxy should stay alive: {error:?}");
                }
            });

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            UntypedRole
                .builder()
                .connect_with(client_transport, async |connection| {
                    let request: SentRequest<SimpleResponse> =
                        connection.send_request(SimpleRequest {
                            message: "park".into(),
                        });
                    request.cancel()?;

                    // Barrier: the cancellation has now been processed by the
                    // proxy (and would have been processed by the backend if
                    // it had been forwarded).
                    let barrier = connection
                        .send_request(SimpleRequest {
                            message: "barrier".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(barrier.result, "echo: barrier");
                    assert_no_event(&mut backend_cancel_rx);

                    // Release the parked request: the cancelled request still
                    // completes with normal data, because the proxy absorbed
                    // the cancellation.
                    let release = connection
                        .send_request(SimpleRequest {
                            message: "release".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(release.result, "echo: release");

                    let response = request
                        .block_task()
                        .await
                        .expect("absorbed cancellation must not fail the request");
                    assert_eq!(response.result, "released");
                    Ok(())
                })
                .await
                .unwrap();

            // The backend never saw any `$/cancel_request`.
            let _parked_id = next_with_timeout(&mut parked_id_rx).await;
            assert_no_event(&mut backend_cancel_rx);
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn cancellation_marker_requested_after_cancel_is_already_cancelled() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            // The responder is parked here by the request handler *without*
            // requesting a cancellation marker; the marker is only created
            // after the cancellation has already been recorded.
            let pending_responder: Arc<Mutex<Option<Responder<SimpleResponse>>>> =
                Arc::new(Mutex::new(None));

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole
                .builder()
                .on_receive_request(
                    {
                        let pending_responder = pending_responder.clone();
                        async move |_request: SimpleRequest,
                                    responder: Responder<SimpleResponse>,
                                    _connection: ConnectionTo<UntypedRole>| {
                            *pending_responder.lock().unwrap() = Some(responder);
                            Ok(())
                        }
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_notification(
                    {
                        let pending_responder = pending_responder.clone();
                        async move |_cancel: CancelRequestNotification,
                                    _connection: ConnectionTo<UntypedRole>| {
                            // The registry recorded the cancellation before
                            // this handler ran, so markers created only now
                            // must already report it.
                            let responder = pending_responder
                                .lock()
                                .unwrap()
                                .take()
                                .expect("request should have arrived before its cancellation");
                            let marker = responder.cancellation();
                            let second_marker = responder.cancellation();
                            if marker.is_cancelled() && second_marker.is_cancelled() {
                                responder.respond_with_result(Err(
                                    agent_client_protocol::Error::request_cancelled(),
                                ))
                            } else {
                                responder.respond(SimpleResponse {
                                    result: "marker not cancelled".into(),
                                })
                            }
                        }
                    },
                    agent_client_protocol::on_receive_notification!(),
                );

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_transport).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let error = UntypedRole
                .builder()
                .connect_with(client_transport, async |cx| {
                    let request: SentRequest<SimpleResponse> = cx.send_request(SimpleRequest {
                        message: "cancel before marker".into(),
                    });
                    request.cancel()?;
                    Ok(request
                        .block_task()
                        .await
                        .expect_err("request should be cancelled"))
                })
                .await
                .unwrap();

            assert_eq!(i32::from(error.code), -32800);
            assert_eq!(error.message, "Request cancelled");
        })
        .await;
}

/// A dynamic handler that claims `$/cancel_request` notifications and reports
/// them on a channel.
struct CancelCollector {
    tx: mpsc::UnboundedSender<RequestId>,
}

impl HandleDispatchFrom<UntypedRole> for CancelCollector {
    async fn handle_dispatch_from(
        &mut self,
        message: Dispatch,
        _connection: ConnectionTo<UntypedRole>,
    ) -> Result<Handled<Dispatch>, agent_client_protocol::Error> {
        if let Dispatch::Notification(notification) = &message
            && CancelRequestNotification::matches_method(&notification.method)
        {
            let cancel = CancelRequestNotification::parse_message(
                &notification.method,
                &notification.params,
            )?;
            self.tx.unbounded_send(cancel.request_id).unwrap();
            return Ok(Handled::Yes);
        }

        Ok(Handled::No {
            message,
            retry: false,
        })
    }

    fn describe_chain(&self) -> impl std::fmt::Debug {
        "CancelCollector"
    }
}

#[tokio::test(flavor = "current_thread")]
async fn retried_protocol_level_notification_reaches_later_dynamic_handler() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (collector_tx, mut collector_rx) = mpsc::unbounded();

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole
                .builder()
                .on_receive_notification(
                    // Decline the notification but ask for a retry: this must
                    // take precedence over the "ignore unhandled
                    // notifications" fallback.
                    async |cancel: CancelRequestNotification, cx: ConnectionTo<UntypedRole>| {
                        Ok::<_, agent_client_protocol::Error>(Handled::No {
                            message: (cancel, cx),
                            retry: true,
                        })
                    },
                    agent_client_protocol::on_receive_notification!(),
                )
                .on_receive_request(
                    {
                        let collector_tx = collector_tx.clone();
                        async move |request: SimpleRequest,
                                    responder: Responder<SimpleResponse>,
                                    connection: ConnectionTo<UntypedRole>| {
                            if request.message == "register" {
                                connection
                                    .add_dynamic_handler(CancelCollector {
                                        tx: collector_tx.clone(),
                                    })?
                                    .detach();
                            }
                            responder.respond(SimpleResponse {
                                result: format!("echo: {}", request.message),
                            })
                        }
                    },
                    agent_client_protocol::on_receive_request!(),
                );

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_transport).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let received = UntypedRole
                .builder()
                .connect_with(client_transport, async |cx| {
                    cx.send_cancel_request("req-1".to_string())?;

                    // Barrier: the notification has now been declined and
                    // queued for retry, and no dynamic handler has seen it.
                    let barrier = cx
                        .send_request(SimpleRequest {
                            message: "barrier".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(barrier.result, "echo: barrier");
                    assert_no_event(&mut collector_rx);

                    // Registering the dynamic handler replays the queued
                    // notification to it.
                    let register = cx
                        .send_request(SimpleRequest {
                            message: "register".into(),
                        })
                        .block_task()
                        .await?;
                    assert_eq!(register.result, "echo: register");

                    Ok(next_with_timeout(&mut collector_rx).await)
                })
                .await
                .unwrap();

            assert_eq!(received, RequestId::Str("req-1".into()));
            assert_no_event(&mut collector_rx);
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn request_handler_can_observe_cancellation_from_responder() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole.builder().on_receive_request(
                async |_request: SimpleRequest,
                       responder: Responder<SimpleResponse>,
                       connection: ConnectionTo<UntypedRole>| {
                    let cancellation = responder.cancellation();
                    assert!(!cancellation.is_cancelled());

                    connection.spawn(async move {
                        let response = cancellation
                            .run_until_cancelled(futures::future::pending::<
                                Result<SimpleResponse, agent_client_protocol::Error>,
                            >())
                            .await;
                        assert!(cancellation.is_cancelled());
                        responder.respond_with_result(response)
                    })?;

                    Ok(())
                },
                agent_client_protocol::on_receive_request!(),
            );

            tokio::task::spawn_local(async move {
                if let Err(error) = server.connect_to(server_transport).await {
                    panic!("server should stay alive: {error:?}");
                }
            });

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let error = UntypedRole
                .builder()
                .connect_with(client_transport, async |cx| {
                    let request: SentRequest<SimpleResponse> = cx.send_request(SimpleRequest {
                        message: "cancel me".into(),
                    });
                    request.cancel()?;
                    Ok(request
                        .block_task()
                        .await
                        .expect_err("request should be cancelled"))
                })
                .await
                .unwrap();

            assert_eq!(i32::from(error.code), -32800);
            assert_eq!(error.message, "Request cancelled");
        })
        .await;
}

#[test]
fn protocol_level_notification_and_cancelled_error_code_are_typed() {
    let notification = ProtocolLevelNotification::parse_message(
        "$/cancel_request",
        &serde_json::json!({ "requestId": "req-1" }),
    )
    .unwrap();
    assert_eq!(notification.method(), "$/cancel_request");

    let error = agent_client_protocol::Error::request_cancelled();
    assert_eq!(i32::from(error.code), -32800);
    assert_eq!(error.message, "Request cancelled");
}
