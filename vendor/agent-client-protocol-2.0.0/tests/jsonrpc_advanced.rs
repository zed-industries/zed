//! Advanced feature tests for JSON-RPC layer
//!
//! Tests advanced JSON-RPC capabilities:
//! - Bidirectional communication (both sides can be client+server)
//! - Request ID tracking and matching
//! - Out-of-order response handling

use agent_client_protocol::{
    Channel, ConnectionTo, Dispatch, HandleDispatchFrom, Handled, JsonRpcMessage,
    JsonRpcNotification, JsonRpcRequest, JsonRpcResponse, RawJsonRpcMessage, Responder,
    SentRequest, TransportBatch, TransportFrame, role::UntypedRole,
};
use futures::channel::{mpsc, oneshot};
use futures::{AsyncRead, AsyncWrite, StreamExt as _};
use serde::{Deserialize, Serialize};
use std::{marker::PhantomData, time::Duration};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

/// Test helper to block and wait for a JSON-RPC response.
async fn recv<T: Send + 'static>(
    response: SentRequest<T>,
) -> Result<T, agent_client_protocol::Error> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    response.on_receiving_result(async move |result| {
        tx.send(result)
            .map_err(|_| agent_client_protocol::Error::internal_error())
    })?;
    rx.await
        .map_err(|_| agent_client_protocol::Error::internal_error())?
}

/// Helper to set up test streams for testing.
fn setup_test_streams() -> (
    impl AsyncRead,
    impl AsyncWrite,
    impl AsyncRead,
    impl AsyncWrite,
) {
    let (client_writer, server_reader) = tokio::io::duplex(1024);
    let (server_writer, client_reader) = tokio::io::duplex(1024);

    let server_reader = server_reader.compat();
    let server_writer = server_writer.compat_write();
    let client_reader = client_reader.compat();
    let client_writer = client_writer.compat_write();

    (server_reader, server_writer, client_reader, client_writer)
}

// ============================================================================
// Test types
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PingRequest {
    value: u32,
}

impl JsonRpcMessage for PingRequest {
    fn matches_method(method: &str) -> bool {
        method == "ping"
    }

    fn method(&self) -> &'static str {
        "ping"
    }

    fn to_untyped_message(
        &self,
    ) -> Result<agent_client_protocol::UntypedMessage, agent_client_protocol::Error> {
        agent_client_protocol::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(
        method: &str,
        params: &impl serde::Serialize,
    ) -> Result<Self, agent_client_protocol::Error> {
        if !Self::matches_method(method) {
            return Err(agent_client_protocol::Error::method_not_found());
        }
        agent_client_protocol::util::json_cast(params)
    }
}

impl JsonRpcRequest for PingRequest {
    type Response = PongResponse;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PongResponse {
    value: u32,
}

impl JsonRpcResponse for PongResponse {
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

#[derive(Debug)]
struct LifetimeTaggedResponse<'a> {
    value: u32,
    _scope: PhantomData<&'a ()>,
}

#[allow(clippy::elidable_lifetime_names)]
fn map_response_with_scope(
    request: SentRequest<PongResponse>,
    _scope: &(),
) -> SentRequest<LifetimeTaggedResponse<'_>> {
    request.map(|response| {
        Ok(LifetimeTaggedResponse {
            value: response.value,
            _scope: PhantomData,
        })
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SlowRequest {
    delay_ms: u64,
    id: u32,
}

impl JsonRpcMessage for SlowRequest {
    fn matches_method(method: &str) -> bool {
        method == "slow"
    }

    fn method(&self) -> &'static str {
        "slow"
    }

    fn to_untyped_message(
        &self,
    ) -> Result<agent_client_protocol::UntypedMessage, agent_client_protocol::Error> {
        agent_client_protocol::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(
        method: &str,
        params: &impl serde::Serialize,
    ) -> Result<Self, agent_client_protocol::Error> {
        if !Self::matches_method(method) {
            return Err(agent_client_protocol::Error::method_not_found());
        }
        agent_client_protocol::util::json_cast(params)
    }
}

impl JsonRpcRequest for SlowRequest {
    type Response = SlowResponse;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SlowResponse {
    id: u32,
}

impl JsonRpcResponse for SlowResponse {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AfterResponseNotification {
    value: u32,
}

impl JsonRpcMessage for AfterResponseNotification {
    fn matches_method(method: &str) -> bool {
        method == "after_response"
    }

    fn method(&self) -> &'static str {
        "after_response"
    }

    fn to_untyped_message(
        &self,
    ) -> Result<agent_client_protocol::UntypedMessage, agent_client_protocol::Error> {
        agent_client_protocol::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(
        method: &str,
        params: &impl serde::Serialize,
    ) -> Result<Self, agent_client_protocol::Error> {
        if !Self::matches_method(method) {
            return Err(agent_client_protocol::Error::method_not_found());
        }
        agent_client_protocol::util::json_cast(params)
    }
}

impl JsonRpcNotification for AfterResponseNotification {}

struct AfterResponseCollector {
    notification_tx: mpsc::UnboundedSender<u32>,
}

impl HandleDispatchFrom<UntypedRole> for AfterResponseCollector {
    async fn handle_dispatch_from(
        &mut self,
        message: Dispatch,
        _connection: ConnectionTo<UntypedRole>,
    ) -> Result<Handled<Dispatch>, agent_client_protocol::Error> {
        if let Dispatch::Notification(notification) = &message
            && AfterResponseNotification::matches_method(&notification.method)
        {
            let notification = AfterResponseNotification::parse_message(
                &notification.method,
                &notification.params,
            )?;
            self.notification_tx
                .unbounded_send(notification.value)
                .map_err(agent_client_protocol::Error::into_internal_error)?;
            return Ok(Handled::Yes);
        }

        Ok(Handled::No {
            message,
            retry: false,
        })
    }

    fn describe_chain(&self) -> impl std::fmt::Debug {
        "AfterResponseCollector"
    }
}

// ============================================================================
// Test 1: Bidirectional communication
// ============================================================================

#[tokio::test(flavor = "current_thread")]
async fn test_bidirectional_communication() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            // Set up two connections that are symmetric - both can send and receive
            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();

            let side_a_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let side_a = UntypedRole.builder().on_receive_request(
                async |request: PingRequest,
                       responder: Responder<PongResponse>,
                       _connection: ConnectionTo<UntypedRole>| {
                    responder.respond(PongResponse {
                        value: request.value + 1,
                    })
                },
                agent_client_protocol::on_receive_request!(),
            );

            let side_b_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);

            // Spawn side_a as server
            tokio::task::spawn_local(async move {
                side_a.connect_to(side_a_transport).await.ok();
            });

            // Use side_b as client
            let result = UntypedRole
                .builder()
                .connect_with(
                    side_b_transport,
                    async |cx| -> Result<(), agent_client_protocol::Error> {
                        let request = PingRequest { value: 10 };
                        let response_future = recv(cx.send_request(request));
                        let response: Result<PongResponse, _> = response_future.await;

                        assert!(response.is_ok());
                        if let Ok(resp) = response {
                            assert_eq!(resp.value, 11);
                        }
                        Ok(())
                    },
                )
                .await;

            assert!(result.is_ok(), "Test failed: {result:?}");
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn callback_consumes_response_before_following_message_is_dispatched() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let (callback_started_tx, callback_started_rx) = oneshot::channel();
            let (release_callback_tx, release_callback_rx) = oneshot::channel();
            let (notification_tx, mut notification_rx) = mpsc::unbounded();

            let server = UntypedRole.builder().on_receive_request(
                async |request: PingRequest,
                       responder: Responder<PongResponse>,
                       connection: ConnectionTo<UntypedRole>| {
                    responder.respond(PongResponse {
                        value: request.value + 1,
                    })?;
                    connection.send_notification(AfterResponseNotification {
                        value: request.value,
                    })
                },
                agent_client_protocol::on_receive_request!(),
            );
            let client = UntypedRole.builder().on_receive_notification(
                async move |notification: AfterResponseNotification,
                            _connection: ConnectionTo<UntypedRole>| {
                    notification_tx
                        .unbounded_send(notification.value)
                        .map_err(agent_client_protocol::Error::into_internal_error)
                },
                agent_client_protocol::on_receive_notification!(),
            );

            let result = client
                .connect_with(server, async move |connection| {
                    connection
                        .send_request(PingRequest { value: 41 })
                        .on_receiving_result(async move |response| {
                            assert_eq!(response?.value, 42);
                            callback_started_tx
                                .send(())
                                .map_err(|()| agent_client_protocol::Error::internal_error())?;
                            release_callback_rx
                                .await
                                .map_err(|_| agent_client_protocol::Error::internal_error())
                        })?;

                    callback_started_rx
                        .await
                        .map_err(|_| agent_client_protocol::Error::internal_error())?;
                    assert!(
                        tokio::time::timeout(Duration::from_millis(100), notification_rx.next())
                            .await
                            .is_err(),
                        "the next message was dispatched before the response callback completed"
                    );

                    release_callback_tx
                        .send(())
                        .map_err(|()| agent_client_protocol::Error::internal_error())?;
                    let notification =
                        tokio::time::timeout(Duration::from_secs(10), notification_rx.next())
                            .await
                            .map_err(agent_client_protocol::Error::into_internal_error)?
                            .ok_or_else(agent_client_protocol::Error::internal_error)?;
                    assert_eq!(notification, 41);
                    Ok(())
                })
                .await;

            assert!(result.is_ok(), "Test failed: {result:?}");
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn unconsumed_response_does_not_block_following_message() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let (notification_tx, mut notification_rx) = mpsc::unbounded();
            let server = UntypedRole.builder().on_receive_request(
                async |request: PingRequest,
                       responder: Responder<PongResponse>,
                       connection: ConnectionTo<UntypedRole>| {
                    responder.respond(PongResponse {
                        value: request.value + 1,
                    })?;
                    connection.send_notification(AfterResponseNotification {
                        value: request.value,
                    })
                },
                agent_client_protocol::on_receive_request!(),
            );
            let client = UntypedRole.builder().on_receive_notification(
                async move |notification: AfterResponseNotification,
                            _connection: ConnectionTo<UntypedRole>| {
                    notification_tx
                        .unbounded_send(notification.value)
                        .map_err(agent_client_protocol::Error::into_internal_error)
                },
                agent_client_protocol::on_receive_notification!(),
            );

            let result = client
                .connect_with(server, async move |connection| {
                    let response = connection.send_request(PingRequest { value: 7 });
                    let notification =
                        tokio::time::timeout(Duration::from_secs(10), notification_rx.next())
                            .await
                            .map_err(agent_client_protocol::Error::into_internal_error)?
                            .ok_or_else(agent_client_protocol::Error::internal_error)?;
                    assert_eq!(notification, 7);
                    assert_eq!(response.block_task().await?.value, 8);
                    Ok(())
                })
                .await;

            assert!(result.is_ok(), "Test failed: {result:?}");
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn ordered_callback_installs_dynamic_handler_before_later_batch_entry() {
    let (transport, mut peer) = Channel::duplex();
    let (notification_tx, mut notification_rx) = mpsc::unbounded();

    let client = UntypedRole
        .builder()
        .connect_with(transport, async move |connection| {
            connection
                .send_request(PingRequest { value: 41 })
                .on_receiving_result({
                    let connection = connection.clone();
                    async move |response| {
                        assert_eq!(response?.value, 42);
                        connection
                            .add_dynamic_handler(AfterResponseCollector { notification_tx })?
                            .detach();
                        Ok(())
                    }
                })?;

            let notification = notification_rx
                .next()
                .await
                .ok_or_else(agent_client_protocol::Error::internal_error)?;
            assert_eq!(notification, 41);
            Ok(())
        });

    let peer = async move {
        let Some(TransportFrame::Single(RawJsonRpcMessage::Request(request))) =
            peer.rx.next().await
        else {
            panic!("expected a ping request");
        };
        assert_eq!(request.method.as_ref(), "ping");

        let response = RawJsonRpcMessage::response(
            request.id,
            Ok(serde_json::to_value(PongResponse { value: 42 })
                .expect("ping response should serialize")),
        );
        let notification = RawJsonRpcMessage::notification(
            "after_response".into(),
            serde_json::to_value(AfterResponseNotification { value: 41 })
                .expect("notification should serialize"),
        )
        .expect("notification should form valid JSON-RPC parameters");
        let batch = TransportBatch::from_messages([response, notification])
            .expect("test batch should be non-empty");
        peer.tx
            .unbounded_send(TransportFrame::Batch(batch))
            .expect("client should accept the response batch");

        while peer.rx.next().await.is_some() {}
        Ok::<(), agent_client_protocol::Error>(())
    };

    tokio::time::timeout(Duration::from_secs(10), async {
        futures::try_join!(client, peer)
    })
    .await
    .expect("dynamic handler was not installed before the later batch entry")
    .expect("connection failed");
}

#[tokio::test(flavor = "current_thread")]
async fn test_map_response_to_application_types() {
    use std::rc::Rc;
    use tokio::task::LocalSet;

    LocalSet::new()
        .run_until(async {
            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();

            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole.builder().on_receive_request(
                async |request: PingRequest,
                       responder: Responder<PongResponse>,
                       _connection: ConnectionTo<UntypedRole>| {
                    responder.respond(PongResponse {
                        value: request.value + 1,
                    })
                },
                agent_client_protocol::on_receive_request!(),
            );

            tokio::task::spawn_local(async move {
                server.connect_to(server_transport).await.ok();
            });

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let mapper_state = String::from("consumed by the mapper");

            let result = UntypedRole
                .builder()
                .connect_with(
                    client_transport,
                    async |cx| -> Result<(), agent_client_protocol::Error> {
                        let response = recv(cx.send_request(PingRequest { value: 10 }).map(
                            move |response| {
                                let mapper_state = mapper_state;
                                assert_eq!(mapper_state, "consumed by the mapper");
                                Ok(response.value)
                            },
                        ))
                        .await?;

                        assert_eq!(response, 11_u32);
                        let non_send_response = cx
                            .send_request(PingRequest { value: 20 })
                            .map(|response| Ok(Rc::new(response.value)))
                            .block_task()
                            .await?;

                        assert_eq!(*non_send_response, 21_u32);

                        let scope = ();
                        let scoped_response = map_response_with_scope(
                            cx.send_request(PingRequest { value: 30 }),
                            &scope,
                        );
                        assert_eq!(scoped_response.method(), "ping");
                        assert!(matches!(
                            scoped_response.id(),
                            agent_client_protocol::schema::v1::RequestId::Str(id)
                                if !id.is_empty()
                        ));
                        let scoped_response = scoped_response.block_task().await?;

                        assert_eq!(scoped_response.value, 31_u32);
                        Ok(())
                    },
                )
                .await;

            assert!(result.is_ok(), "Test failed: {result:?}");
        })
        .await;
}

// ============================================================================
// Test 2: Request IDs are properly tracked
// ============================================================================

#[tokio::test(flavor = "current_thread")]
async fn test_request_ids() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();

            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole.builder().on_receive_request(
                async |request: PingRequest,
                       responder: Responder<PongResponse>,
                       _connection: ConnectionTo<UntypedRole>| {
                    responder.respond(PongResponse {
                        value: request.value + 1,
                    })
                },
                agent_client_protocol::on_receive_request!(),
            );

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let client = UntypedRole.builder();

            tokio::task::spawn_local(async move {
                server.connect_to(server_transport).await.ok();
            });

            let result = client
                .connect_with(
                    client_transport,
                    async |cx| -> Result<(), agent_client_protocol::Error> {
                        // Send multiple requests and verify responses match
                        let req1 = PingRequest { value: 1 };
                        let req2 = PingRequest { value: 2 };
                        let req3 = PingRequest { value: 3 };

                        let resp1_future = recv(cx.send_request(req1));
                        let resp2_future = recv(cx.send_request(req2));
                        let resp3_future = recv(cx.send_request(req3));

                        let resp1: Result<PongResponse, _> = resp1_future.await;
                        let resp2: Result<PongResponse, _> = resp2_future.await;
                        let resp3: Result<PongResponse, _> = resp3_future.await;

                        // Verify each response corresponds to its request
                        assert_eq!(resp1.unwrap().value, 2); // 1 + 1
                        assert_eq!(resp2.unwrap().value, 3); // 2 + 1
                        assert_eq!(resp3.unwrap().value, 4); // 3 + 1

                        Ok(())
                    },
                )
                .await;

            assert!(result.is_ok(), "Test failed: {result:?}");
        })
        .await;
}

// ============================================================================
// Test 3: Out-of-order responses
// ============================================================================

#[tokio::test(flavor = "current_thread")]
async fn test_out_of_order_responses() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();

            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole.builder().on_receive_request(
                async |request: SlowRequest,
                       responder: Responder<SlowResponse>,
                       _connection: ConnectionTo<UntypedRole>| {
                    // Simulate delay
                    tokio::time::sleep(tokio::time::Duration::from_millis(request.delay_ms)).await;
                    responder.respond(SlowResponse { id: request.id })
                },
                agent_client_protocol::on_receive_request!(),
            );

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let client = UntypedRole.builder();

            tokio::task::spawn_local(async move {
                server.connect_to(server_transport).await.ok();
            });

            let result = client
                .connect_with(
                    client_transport,
                    async |cx| -> Result<(), agent_client_protocol::Error> {
                        // Send requests with different delays
                        // Request 1: 100ms delay
                        // Request 2: 50ms delay
                        // Request 3: 10ms delay
                        // Responses should arrive in order: 3, 2, 1

                        let req1 = SlowRequest {
                            delay_ms: 100,
                            id: 1,
                        };
                        let req2 = SlowRequest {
                            delay_ms: 50,
                            id: 2,
                        };
                        let req3 = SlowRequest {
                            delay_ms: 10,
                            id: 3,
                        };

                        let resp1_future = recv(cx.send_request(req1));
                        let resp2_future = recv(cx.send_request(req2));
                        let resp3_future = recv(cx.send_request(req3));

                        // Wait for all responses
                        let resp1: Result<SlowResponse, _> = resp1_future.await;
                        let resp2: Result<SlowResponse, _> = resp2_future.await;
                        let resp3: Result<SlowResponse, _> = resp3_future.await;

                        // Verify each future got the correct response despite out-of-order arrival
                        assert_eq!(resp1.unwrap().id, 1);
                        assert_eq!(resp2.unwrap().id, 2);
                        assert_eq!(resp3.unwrap().id, 3);

                        Ok(())
                    },
                )
                .await;

            assert!(result.is_ok(), "Test failed: {result:?}");
        })
        .await;
}
