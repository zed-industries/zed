//! Integration test for basic JSON-RPC communication.
//!
//! This test sets up two JSON-RPC connections and verifies they can
//! exchange simple "hello world" messages.

use agent_client_protocol::role::UntypedRole;
use agent_client_protocol::{
    ConnectionTo, JsonRpcMessage, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse, Responder,
    SentRequest,
};
use futures::{AsyncRead, AsyncWrite};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

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

/// Helper to set up a client-server pair for testing.
/// Returns (server_reader, server_writer, client_reader, client_writer) for manual setup.
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

/// A simple "ping" request.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PingRequest {
    message: String,
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

/// A simple "pong" response.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PongResponse {
    echo: String,
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

#[tokio::test(flavor = "current_thread")]
async fn test_hello_world() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();

            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole.builder().on_receive_request(
                async move |request: PingRequest,
                            responder: Responder<PongResponse>,
                            _connection: ConnectionTo<UntypedRole>| {
                    let pong = PongResponse {
                        echo: format!("pong: {}", request.message),
                    };
                    responder.respond(pong)
                },
                agent_client_protocol::on_receive_request!(),
            );

            let client_transport =
                agent_client_protocol::ByteStreams::new(client_writer, client_reader);
            let client = UntypedRole.builder();

            // Spawn the server in the background
            tokio::task::spawn_local(async move {
                if let Err(e) = server.connect_to(server_transport).await {
                    eprintln!("Server error: {e:?}");
                }
            });

            // Use the client to send a ping and wait for a pong
            let result = client
                .connect_with(
                    client_transport,
                    async |cx| -> std::result::Result<(), agent_client_protocol::Error> {
                        let request = PingRequest {
                            message: "hello world".to_string(),
                        };

                        let response = recv(cx.send_request(request)).await.map_err(|e| {
                            agent_client_protocol::util::internal_error(format!(
                                "Request failed: {e:?}"
                            ))
                        })?;

                        assert_eq!(response.echo, "pong: hello world");

                        Ok(())
                    },
                )
                .await;

            assert!(result.is_ok(), "Test failed: {result:?}");
        })
        .await;
}

/// A simple notification message
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LogNotification {
    message: String,
}

impl JsonRpcMessage for LogNotification {
    fn matches_method(method: &str) -> bool {
        method == "log"
    }

    fn method(&self) -> &'static str {
        "log"
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

impl JsonRpcNotification for LogNotification {}

#[tokio::test(flavor = "current_thread")]
async fn test_notification() {
    use tokio::task::LocalSet;

    let local = LocalSet::new();

    local
        .run_until(async {
            let logs = Arc::new(Mutex::new(Vec::new()));
            let logs_clone = logs.clone();

            let (server_reader, server_writer, client_reader, client_writer) = setup_test_streams();

            let server_transport =
                agent_client_protocol::ByteStreams::new(server_writer, server_reader);
            let server = UntypedRole.builder().on_receive_notification(
                {
                    let logs = logs_clone.clone();
                    async move |notification: LogNotification, _cx: ConnectionTo<UntypedRole>| {
                        logs.lock().unwrap().push(notification.message);
                        Ok(())
                    }
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
                        // Send a notification (no response expected)
                        cx.send_notification(LogNotification {
                            message: "test log 1".to_string(),
                        })
                        .map_err(|e| {
                            agent_client_protocol::util::internal_error(format!(
                                "Failed to send notification: {e:?}"
                            ))
                        })?;

                        cx.send_notification(LogNotification {
                            message: "test log 2".to_string(),
                        })
                        .map_err(|e| {
                            agent_client_protocol::util::internal_error(format!(
                                "Failed to send notification: {e:?}"
                            ))
                        })?;

                        // Give the server time to process notifications
                        tokio::time::sleep(Duration::from_millis(100)).await;

                        Ok(())
                    },
                )
                .await;

            assert!(result.is_ok(), "Test failed: {result:?}");

            let received_logs = logs.lock().unwrap();
            assert_eq!(received_logs.len(), 2);
            assert_eq!(received_logs[0], "test log 1");
            assert_eq!(received_logs[1], "test log 2");
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn test_multiple_sequential_requests() {
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
                    let pong = PongResponse {
                        echo: format!("pong: {}", request.message),
                    };
                    responder.respond(pong)
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
                        // Send multiple requests sequentially
                        for i in 1..=5 {
                            let request = PingRequest {
                                message: format!("message {i}"),
                            };

                            let response = recv(cx.send_request(request)).await.map_err(|e| {
                                agent_client_protocol::util::internal_error(format!(
                                    "Request {i} failed: {e:?}"
                                ))
                            })?;

                            assert_eq!(response.echo, format!("pong: message {i}"));
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
async fn test_concurrent_requests() {
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
                    let pong = PongResponse {
                        echo: format!("pong: {}", request.message),
                    };
                    responder.respond(pong)
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
                        // Send multiple requests concurrently
                        let mut responses = Vec::new();

                        for i in 1..=5 {
                            let request = PingRequest {
                                message: format!("concurrent message {i}"),
                            };

                            // Start all requests without awaiting
                            responses.push((i, cx.send_request(request)));
                        }

                        // Now await all responses
                        for (i, response_future) in responses {
                            let response = recv(response_future).await.map_err(|e| {
                                agent_client_protocol::util::internal_error(format!(
                                    "Request {i} failed: {e:?}"
                                ))
                            })?;

                            assert_eq!(response.echo, format!("pong: concurrent message {i}"));
                        }

                        Ok(())
                    },
                )
                .await;

            assert!(result.is_ok(), "Test failed: {result:?}");
        })
        .await;
}
