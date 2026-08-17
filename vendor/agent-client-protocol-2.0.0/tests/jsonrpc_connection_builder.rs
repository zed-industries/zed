//! Integration tests for JSON-RPC connection builder behavior.
//!
//! These tests verify that multiple handlers can be registered on a connection builder
//! and that requests/notifications are routed correctly based on which
//! handler claims them.

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use agent_client_protocol::{
    ConnectTo, ConnectionTo, JsonRpcMessage, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse,
    Responder, SentRequest, role::UntypedRole, util::run_until,
};
use serde::{Deserialize, Serialize};
use tokio_util::compat::{TokioAsyncReadCompatExt as _, TokioAsyncWriteCompatExt as _};

/// Test helper to block and wait for a JSON-RPC response.
async fn recv<T: JsonRpcResponse + Send>(
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

// ============================================================================
// Test 1: Multiple handlers with different methods
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FooRequest {
    value: String,
}

impl JsonRpcMessage for FooRequest {
    fn matches_method(method: &str) -> bool {
        method == "foo"
    }

    fn method(&self) -> &'static str {
        "foo"
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

impl JsonRpcRequest for FooRequest {
    type Response = FooResponse;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FooResponse {
    result: String,
}

impl JsonRpcResponse for FooResponse {
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
struct BarRequest {
    value: String,
}

impl JsonRpcMessage for BarRequest {
    fn matches_method(method: &str) -> bool {
        method == "bar"
    }

    fn method(&self) -> &'static str {
        "bar"
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

impl JsonRpcRequest for BarRequest {
    type Response = BarResponse;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BarResponse {
    result: String,
}

impl JsonRpcResponse for BarResponse {
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
async fn test_multiple_handlers_different_methods() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (client_writer, server_reader) = tokio::io::duplex(1024);
            let (server_writer, client_reader) = tokio::io::duplex(1024);

            let server_reader = server_reader.compat();
            let server_writer = server_writer.compat_write();
            let client_reader = client_reader.compat();
            let client_writer = client_writer.compat_write();

            // Chain both handlers
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole
                .builder()
                .on_receive_request(
                    async |request: FooRequest,
                           responder: Responder<FooResponse>,
                           _connection: ConnectionTo<UntypedRole>| {
                        responder.respond(FooResponse {
                            result: format!("foo: {}", request.value),
                        })
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_request(
                    async |request: BarRequest,
                           responder: Responder<BarResponse>,
                           _connection: ConnectionTo<UntypedRole>| {
                        responder.respond(BarResponse {
                            result: format!("bar: {}", request.value),
                        })
                    },
                    agent_client_protocol::on_receive_request!(),
                );
            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let client = UntypedRole.builder();

            tokio::task::spawn_local(async move {
                if let Err(e) = server.connect_to(server_transport).await {
                    eprintln!("Server error: {e:?}");
                }
            });

            let result = client
                .connect_with(
                    client_transport,
                    async |cx| -> std::result::Result<(), agent_client_protocol::Error> {
                        // Test foo request
                        let foo_response = recv(cx.send_request(FooRequest {
                            value: "test1".to_string(),
                        }))
                        .await
                        .map_err(
                            |e| -> agent_client_protocol::Error {
                                agent_client_protocol::util::internal_error(format!(
                                    "Foo request failed: {e:?}"
                                ))
                            },
                        )?;
                        assert_eq!(foo_response.result, "foo: test1");

                        // Test bar request
                        let bar_response = recv(cx.send_request(BarRequest {
                            value: "test2".to_string(),
                        }))
                        .await
                        .map_err(
                            |e| -> agent_client_protocol::Error {
                                agent_client_protocol::util::internal_error(format!(
                                    "Bar request failed: {e:?}"
                                ))
                            },
                        )?;
                        assert_eq!(bar_response.result, "bar: test2");

                        Ok(())
                    },
                )
                .await;

            assert!(result.is_ok(), "Test failed: {result:?}");
        })
        .await;
}

// ============================================================================
// Test 2: Handler priority/ordering (first handler gets first chance)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TrackRequest {
    value: String,
}

impl JsonRpcMessage for TrackRequest {
    fn matches_method(method: &str) -> bool {
        method == "track"
    }

    fn method(&self) -> &'static str {
        "track"
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

impl JsonRpcRequest for TrackRequest {
    type Response = FooResponse;
}

#[tokio::test(flavor = "current_thread")]
async fn test_handler_priority_ordering() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let handled = Arc::new(Mutex::new(Vec::new()));

            let (client_writer, server_reader) = tokio::io::duplex(1024);
            let (server_writer, client_reader) = tokio::io::duplex(1024);

            let server_reader = server_reader.compat();
            let server_writer = server_writer.compat_write();
            let client_reader = client_reader.compat();
            let client_writer = client_writer.compat_write();

            // First handler in chain should get first chance
            let handled_clone1 = handled.clone();
            let handled_clone2 = handled.clone();
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole
                .builder()
                .on_receive_request(
                    async move |request: TrackRequest,
                                responder: Responder<FooResponse>,
                                _connection: ConnectionTo<UntypedRole>| {
                        handled_clone1.lock().unwrap().push("handler1".to_string());
                        responder.respond(FooResponse {
                            result: format!("handler1: {}", request.value),
                        })
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_request(
                    async move |request: TrackRequest,
                                responder: Responder<FooResponse>,
                                _connection: ConnectionTo<UntypedRole>| {
                        handled_clone2.lock().unwrap().push("handler2".to_string());
                        responder.respond(FooResponse {
                            result: format!("handler2: {}", request.value),
                        })
                    },
                    agent_client_protocol::on_receive_request!(),
                );
            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let client = UntypedRole.builder();

            tokio::task::spawn_local(async move {
                if let Err(e) = server.connect_to(server_transport).await {
                    eprintln!("Server error: {e:?}");
                }
            });

            let result = client
                .connect_with(
                    client_transport,
                    async |cx| -> std::result::Result<(), agent_client_protocol::Error> {
                        let response = recv(cx.send_request(TrackRequest {
                            value: "test".to_string(),
                        }))
                        .await
                        .map_err(|e| {
                            agent_client_protocol::util::internal_error(format!(
                                "Track request failed: {e:?}"
                            ))
                        })?;

                        // First handler should have handled it
                        assert_eq!(response.result, "handler1: test");

                        Ok(())
                    },
                )
                .await;

            assert!(result.is_ok(), "Test failed: {result:?}");

            // Verify only handler1 was invoked
            let handled_by = handled.lock().unwrap();
            assert_eq!(handled_by.len(), 1);
            assert_eq!(handled_by[0], "handler1");
        })
        .await;
}

// ============================================================================
// Test 3: Fallthrough behavior (handler passes to next)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Method1Request {
    value: String,
}

impl JsonRpcMessage for Method1Request {
    fn matches_method(method: &str) -> bool {
        method == "method1"
    }

    fn method(&self) -> &'static str {
        "method1"
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

impl JsonRpcRequest for Method1Request {
    type Response = FooResponse;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Method2Request {
    value: String,
}

impl JsonRpcMessage for Method2Request {
    fn matches_method(method: &str) -> bool {
        method == "method2"
    }

    fn method(&self) -> &'static str {
        "method2"
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

impl JsonRpcRequest for Method2Request {
    type Response = FooResponse;
}

#[tokio::test(flavor = "current_thread")]
async fn test_fallthrough_behavior() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let handled = Arc::new(Mutex::new(Vec::new()));

            let (client_writer, server_reader) = tokio::io::duplex(1024);
            let (server_writer, client_reader) = tokio::io::duplex(1024);

            let server_reader = server_reader.compat();
            let server_writer = server_writer.compat_write();
            let client_reader = client_reader.compat();
            let client_writer = client_writer.compat_write();

            // Handler1 only handles "method1", Handler2 only handles "method2"
            let handled_clone1 = handled.clone();
            let handled_clone2 = handled.clone();
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole
                .builder()
                .on_receive_request(
                    async move |request: Method1Request,
                                responder: Responder<FooResponse>,
                                _connection: ConnectionTo<UntypedRole>| {
                        handled_clone1.lock().unwrap().push("method1".to_string());
                        responder.respond(FooResponse {
                            result: format!("method1: {}", request.value),
                        })
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_request(
                    async move |request: Method2Request,
                                responder: Responder<FooResponse>,
                                _connection: ConnectionTo<UntypedRole>| {
                        handled_clone2.lock().unwrap().push("method2".to_string());
                        responder.respond(FooResponse {
                            result: format!("method2: {}", request.value),
                        })
                    },
                    agent_client_protocol::on_receive_request!(),
                );
            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let client = UntypedRole.builder();

            tokio::task::spawn_local(async move {
                if let Err(e) = server.connect_to(server_transport).await {
                    eprintln!("Server error: {e:?}");
                }
            });

            let result = client
                .connect_with(
                    client_transport,
                    async |cx| -> std::result::Result<(), agent_client_protocol::Error> {
                        // Send method2 - should fallthrough handler1 to handler2
                        let response = recv(cx.send_request(Method2Request {
                            value: "fallthrough".to_string(),
                        }))
                        .await
                        .map_err(|e| {
                            agent_client_protocol::util::internal_error(format!(
                                "Method2 request failed: {e:?}"
                            ))
                        })?;

                        assert_eq!(response.result, "method2: fallthrough");

                        Ok(())
                    },
                )
                .await;

            assert!(result.is_ok(), "Test failed: {result:?}");

            // Verify only method2 was handled (handler1 passed through)
            let handled_methods = handled.lock().unwrap();
            assert_eq!(handled_methods.len(), 1);
            assert_eq!(handled_methods[0], "method2");
        })
        .await;
}

// ============================================================================
// Test 4: No handler claims request
// ============================================================================

#[tokio::test(flavor = "current_thread")]
async fn test_no_handler_claims() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (client_writer, server_reader) = tokio::io::duplex(1024);
            let (server_writer, client_reader) = tokio::io::duplex(1024);

            let server_reader = server_reader.compat();
            let server_writer = server_writer.compat_write();
            let client_reader = client_reader.compat();
            let client_writer = client_writer.compat_write();

            // Handler that only handles "foo"
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole.builder().on_receive_request(
                async |request: FooRequest,
                       responder: Responder<FooResponse>,
                       _connection: ConnectionTo<UntypedRole>| {
                    responder.respond(FooResponse {
                        result: format!("foo: {}", request.value),
                    })
                },
                agent_client_protocol::on_receive_request!(),
            );
            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let client = UntypedRole.builder();

            tokio::task::spawn_local(async move {
                if let Err(e) = server.connect_to(server_transport).await {
                    eprintln!("Server error: {e:?}");
                }
            });

            let result = client
                .connect_with(
                    client_transport,
                    async |cx| -> std::result::Result<(), agent_client_protocol::Error> {
                        // Send "bar" request which no handler claims
                        let response_result = recv(cx.send_request(BarRequest {
                            value: "unclaimed".to_string(),
                        }))
                        .await;

                        // Should get an error (method not found)
                        assert!(response_result.is_err());

                        Ok(())
                    },
                )
                .await;

            assert!(result.is_ok(), "Test failed: {result:?}");
        })
        .await;
}

// ============================================================================
// Test 5: Handler can claim notifications
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EventNotification {
    event: String,
}

impl JsonRpcMessage for EventNotification {
    fn matches_method(method: &str) -> bool {
        method == "event"
    }

    fn method(&self) -> &'static str {
        "event"
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

impl JsonRpcNotification for EventNotification {}

#[tokio::test(flavor = "current_thread")]
async fn test_handler_claims_notification() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let events = Arc::new(Mutex::new(Vec::new()));

            let (client_writer, server_reader) = tokio::io::duplex(1024);
            let (server_writer, client_reader) = tokio::io::duplex(1024);

            let server_reader = server_reader.compat();
            let server_writer = server_writer.compat_write();
            let client_reader = client_reader.compat();
            let client_writer = client_writer.compat_write();

            // EventHandler claims notifications
            let events_clone = events.clone();
            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole.builder().on_receive_notification(
                async move |notification: EventNotification,
                            _connection: ConnectionTo<UntypedRole>| {
                    events_clone.lock().unwrap().push(notification.event);
                    Ok(())
                },
                agent_client_protocol::on_receive_notification!(),
            );
            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let client = UntypedRole.builder();

            tokio::task::spawn_local(async move {
                if let Err(e) = server.connect_to(server_transport).await {
                    eprintln!("Server error: {e:?}");
                }
            });

            let result = client
                .connect_with(
                    client_transport,
                    async |cx| -> std::result::Result<(), agent_client_protocol::Error> {
                        cx.send_notification(EventNotification {
                            event: "test_event".to_string(),
                        })
                        .map_err(|e| {
                            agent_client_protocol::util::internal_error(format!(
                                "Failed to send notification: {e:?}"
                            ))
                        })?;

                        // Give server time to process
                        tokio::time::sleep(Duration::from_millis(100)).await;

                        Ok(())
                    },
                )
                .await;

            assert!(result.is_ok(), "Test failed: {result:?}");

            let received_events = events.lock().unwrap();
            assert_eq!(received_events.len(), 1);
            assert_eq!(received_events[0], "test_event");
        })
        .await;
}

// ============================================================================
// Test 6: Builder implements Component
// ============================================================================

#[tokio::test]
async fn test_connection_builder_as_component() -> Result<(), agent_client_protocol::Error> {
    // Create duplex streams
    let (server_stream, client_stream) = tokio::io::duplex(8192);
    let (server_read, server_write) = tokio::io::split(server_stream);
    let (client_read, client_write) = tokio::io::split(client_stream);

    // Create a connection builder (server side)
    let server_builder = UntypedRole.builder().on_receive_request(
        async |request: FooRequest,
               responder: Responder<FooResponse>,
               _cx: ConnectionTo<UntypedRole>| {
            responder.respond(FooResponse {
                result: format!("component: {}", request.value),
            })
        },
        agent_client_protocol::on_receive_request!(),
    );

    // Create ByteStreams for both sides
    let server_transport =
        agent_client_protocol::ByteStreams::new(server_write.compat_write(), server_read.compat());
    let client_transport =
        agent_client_protocol::ByteStreams::new(client_write.compat_write(), client_read.compat());

    // Use Builder as a Component via run_until
    run_until(
        // This uses Component::serve on Builder
        ConnectTo::<UntypedRole>::connect_to(server_builder, server_transport),
        async move {
            // Client side
            UntypedRole
                .builder()
                .connect_with(client_transport, async |cx| {
                    let response = recv(cx.send_request(FooRequest {
                        value: "test".to_string(),
                    }))
                    .await?;

                    assert_eq!(response.result, "component: test");
                    Ok(())
                })
                .await
        },
    )
    .await
}
