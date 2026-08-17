//! Edge case tests for JSON-RPC layer
//!
//! Tests various edge cases and boundary conditions:
//! - Empty requests
//! - Null parameters
//! - Server shutdown scenarios
//! - Client disconnect handling

use agent_client_protocol::{
    ConnectionTo, JsonRpcMessage, JsonRpcRequest, JsonRpcResponse, RawJsonRpcMessage, Responder,
    SentRequest, role::UntypedRole,
};
use futures::{AsyncRead, AsyncWrite};
use serde::{Deserialize, Serialize};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

#[test]
fn raw_jsonrpc_message_rejects_scalar_params() {
    assert!(
        RawJsonRpcMessage::request(
            "scalar_params".into(),
            serde_json::json!(1),
            agent_client_protocol::schema::v1::RequestId::Number(1),
        )
        .is_err()
    );

    assert!(
        RawJsonRpcMessage::notification("scalar_params".into(), serde_json::json!("bad")).is_err()
    );

    assert!(
        serde_json::from_value::<RawJsonRpcMessage>(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "scalar_params",
            "params": 1
        }))
        .is_err()
    );

    assert!(
        serde_json::from_value::<RawJsonRpcMessage>(serde_json::json!({
            "jsonrpc": "2.0",
            "method": "scalar_params",
            "params": true
        }))
        .is_err()
    );

    let response = RawJsonRpcMessage::response(
        agent_client_protocol::schema::v1::RequestId::Number(1),
        Ok(serde_json::json!(1)),
    );
    assert_eq!(
        serde_json::to_value(response).unwrap(),
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": 1
        })
    );
}

#[test]
fn raw_jsonrpc_message_rejects_conflicting_envelope_fields() {
    for message in [
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "request_with_result",
            "result": {}
        }),
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {},
            "error": { "code": -32603, "message": "Internal error" }
        }),
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "notification_with_error",
            "error": { "code": -32603, "message": "Internal error" }
        }),
    ] {
        assert!(serde_json::from_value::<RawJsonRpcMessage>(message).is_err());
    }
}

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

/// Helper to set up test streams.
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EmptyRequest;

impl JsonRpcMessage for EmptyRequest {
    fn matches_method(method: &str) -> bool {
        method == "empty_method"
    }

    fn method(&self) -> &'static str {
        "empty_method"
    }

    fn to_untyped_message(
        &self,
    ) -> Result<agent_client_protocol::UntypedMessage, agent_client_protocol::Error> {
        agent_client_protocol::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(
        method: &str,
        _params: &impl serde::Serialize,
    ) -> Result<Self, agent_client_protocol::Error> {
        if !Self::matches_method(method) {
            return Err(agent_client_protocol::Error::method_not_found());
        }
        Ok(EmptyRequest)
    }
}

impl JsonRpcRequest for EmptyRequest {
    type Response = SimpleResponse;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OptionalParamsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}

impl JsonRpcMessage for OptionalParamsRequest {
    fn matches_method(method: &str) -> bool {
        method == "optional_params_method"
    }

    fn method(&self) -> &'static str {
        "optional_params_method"
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

impl JsonRpcRequest for OptionalParamsRequest {
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

// ============================================================================
// Test 1: Empty request (no parameters)
// ============================================================================

#[tokio::test(flavor = "current_thread")]
async fn test_empty_request() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();

            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole.builder().on_receive_request(
                async |_request: EmptyRequest,
                       responder: Responder<SimpleResponse>,
                       _connection: ConnectionTo<UntypedRole>| {
                    responder.respond(SimpleResponse {
                        result: "Got empty request".to_string(),
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
                        let request = EmptyRequest;

                        let result: Result<SimpleResponse, _> =
                            recv(cx.send_request(request)).await;

                        // Should succeed
                        assert!(result.is_ok());
                        if let Ok(response) = result {
                            assert_eq!(response.result, "Got empty request");
                        }
                        Ok(())
                    },
                )
                .await;

            assert!(result.is_ok(), "Test failed: {result:?}");
        })
        .await;
}

// ============================================================================
// Test 2: Null parameters
// ============================================================================

#[tokio::test(flavor = "current_thread")]
async fn test_null_params() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();

            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole.builder().on_receive_request(
                async |_request: OptionalParamsRequest,
                       responder: Responder<SimpleResponse>,
                       _connection: ConnectionTo<UntypedRole>| {
                    responder.respond(SimpleResponse {
                        result: "Has params: true".to_string(),
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
                        let request = OptionalParamsRequest { value: None };

                        let result: Result<SimpleResponse, _> =
                            recv(cx.send_request(request)).await;

                        // Should succeed - handler should handle null/missing params
                        assert!(result.is_ok());
                        Ok(())
                    },
                )
                .await;

            assert!(result.is_ok(), "Test failed: {result:?}");
        })
        .await;
}

// ============================================================================
// Test 3: Server shutdown with pending requests
// ============================================================================

#[tokio::test(flavor = "current_thread")]
async fn test_server_shutdown() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();

            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole.builder().on_receive_request(
                async |_request: EmptyRequest,
                       responder: Responder<SimpleResponse>,
                       _connection: ConnectionTo<UntypedRole>| {
                    responder.respond(SimpleResponse {
                        result: "Got empty request".to_string(),
                    })
                },
                agent_client_protocol::on_receive_request!(),
            );

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let client = UntypedRole.builder();

            let server_handle = tokio::task::spawn_local(async move {
                server.connect_to(server_transport).await.ok();
            });

            let client_result = tokio::task::spawn_local(async move {
                client
                    .connect_with(
                        client_transport,
                        async |cx| -> Result<(), agent_client_protocol::Error> {
                            let request = EmptyRequest;

                            // Send request and get future for response
                            let response_future = recv(cx.send_request(request));

                            // Give the request time to be sent over the wire
                            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

                            // Try to get response (server should still be running briefly)
                            let _result: Result<SimpleResponse, _> = response_future.await;

                            // Could succeed or fail depending on timing
                            // The important thing is that it doesn't hang
                            Ok(())
                        },
                    )
                    .await
            });

            // Let the client send its request
            tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;

            // Abort the server
            server_handle.abort();

            // Wait for client to finish
            let result = client_result.await;
            assert!(result.is_ok(), "Test failed: {result:?}");
        })
        .await;
}

// ============================================================================
// Test 4: Client disconnect mid-request
// ============================================================================

#[tokio::test(flavor = "current_thread")]
async fn test_client_disconnect() {
    use tokio::io::AsyncWriteExt;
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (mut client_writer, server_reader) = tokio::io::duplex(1024);
            let (server_writer, _client_reader) = tokio::io::duplex(1024);

            let server_reader = server_reader.compat();
            let server_writer = server_writer.compat_write();

            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole.builder().on_receive_request(
                async |_request: EmptyRequest,
                       responder: Responder<SimpleResponse>,
                       _connection: ConnectionTo<UntypedRole>| {
                    responder.respond(SimpleResponse {
                        result: "Got empty request".to_string(),
                    })
                },
                agent_client_protocol::on_receive_request!(),
            );

            tokio::task::spawn_local(async move {
                drop(server.connect_to(server_transport).await);
            });

            // Send partial request and then disconnect
            let partial_request = b"{\"jsonrpc\":\"2.0\",\"method\":\"empty_method\",\"id\":1";
            client_writer.write_all(partial_request).await.unwrap();
            client_writer.flush().await.unwrap();

            // Drop the writer to disconnect
            drop(client_writer);

            // Give server time to process the disconnect
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            // Server should handle this gracefully and terminate
            // (We can't really assert much here except that the test completes)
        })
        .await;
}
