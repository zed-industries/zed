//! WebSocket transport and JSON-RPC/MCP message routing for the Claude Code IDE
//! integration.
//!
//! The Claude Code CLI connects to us as a WebSocket client and speaks a
//! WebSocket variant of MCP (Model Context Protocol): newline-free JSON-RPC 2.0
//! objects, one per WebSocket text frame. This module owns the wire protocol; it
//! knows nothing about Zed's editor state. Everything that needs to read or act
//! on the workspace is delegated through the [`Dispatcher`] trait, which the
//! GPUI-aware layer implements.

use anyhow::{Context as _, Result};
use async_tungstenite::tungstenite::{
    Message,
    handshake::server::{ErrorResponse, Request, Response},
    http,
};
use futures::{AsyncRead, AsyncWrite, StreamExt as _};
use serde::Serialize;
use serde_json::{Value, json};

/// The MCP protocol revision the official extensions speak. Sent back from
/// `initialize`; the CLI checks it for compatibility.
pub const MCP_PROTOCOL_VERSION: &str = "2024-11-05";

const SERVER_NAME: &str = "zed";
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The HTTP header the CLI must send, carrying the token from the lock file.
pub const AUTH_HEADER: &str = "x-claude-code-ide-authorization";

/// JSON-RPC 2.0 standard error codes (see the spec, section 5.1).
pub mod error_codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INTERNAL_ERROR: i32 = -32603;
}

/// An error a method handler can return; serialized into the JSON-RPC `error`
/// field of the response.
#[derive(Debug)]
pub struct ProtocolError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

impl ProtocolError {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self { code, message: message.into(), data: None }
    }

    pub fn method_not_found(method: &str) -> Self {
        Self::new(error_codes::METHOD_NOT_FOUND, format!("Method not found: {method}"))
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(error_codes::INTERNAL_ERROR, message)
    }
}

/// One MCP tool as advertised by `tools/list`. Serializes to
/// `{"name": ..., "description": ..., "inputSchema": {...}}`.
#[derive(Debug, Clone, Serialize)]
pub struct ToolDescriptor {
    pub name: &'static str,
    pub description: &'static str,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

/// The seam between the wire protocol (this module) and Zed's editor state.
///
/// Implementors live in the GPUI layer: `call_tool` reaches into the workspace
/// to satisfy a `tools/call` request. The protocol layer only ever sees JSON.
// The dispatcher runs on GPUI's single-threaded foreground executor, so its
// futures are intentionally `!Send`; we don't need the `Send` bound the lint
// asks us to consider.
#[allow(async_fn_in_trait)]
pub trait Dispatcher {
    /// The tools advertised in response to `tools/list`.
    fn tools(&self) -> Vec<ToolDescriptor>;

    /// Execute a `tools/call`. `Ok` is the MCP result object (typically
    /// `{"content": [{"type": "text", "text": "<json>"}]}`).
    async fn call_tool(&self, name: &str, arguments: Value) -> Result<Value, ProtocolError>;
}

/// Binds a fresh WebSocket listener on the loopback interface, letting the OS
/// pick a free port. Returns the listener and the port we got, which the caller
/// writes into the lock file and injects into the terminal environment.
pub async fn bind() -> Result<(smol::net::TcpListener, u16)> {
    let listener = smol::net::TcpListener::bind("127.0.0.1:0")
        .await
        .context("binding Claude Code IDE WebSocket listener")?;
    let port = listener.local_addr().context("reading bound port")?.port();
    Ok((listener, port))
}

/// Performs the WebSocket handshake (validating the auth header against
/// `auth_token`), then serves JSON-RPC requests until the client disconnects.
pub async fn serve_connection<S, D>(stream: S, auth_token: String, dispatcher: D) -> Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
    D: Dispatcher,
{
    // The handshake callback runs during the HTTP upgrade. We reject the
    // connection unless it presents the exact token we wrote into the lock file,
    // so only the CLI that read our lock file (same user) can connect.
    let authorize = move |request: &Request, response: Response| -> Result<Response, ErrorResponse> {
        let presented = request.headers().get(AUTH_HEADER).map(|value| value.as_bytes());
        if presented == Some(auth_token.as_bytes()) {
            Ok(response)
        } else {
            let denied = http::Response::builder()
                .status(http::StatusCode::UNAUTHORIZED)
                .body(Some("invalid or missing authorization token".to_string()))
                .expect("static unauthorized response is valid");
            Err(denied)
        }
    };

    let websocket = async_tungstenite::accept_hdr_async(stream, authorize)
        .await
        .context("websocket handshake failed")?;

    let (mut outgoing, mut incoming) = websocket.split();

    while let Some(message) = incoming.next().await {
        match message.context("reading websocket frame")? {
            Message::Text(text) => {
                if let Some(response) = handle_message(text.as_str(), &dispatcher).await {
                    outgoing
                        .send(Message::Text(response.into()))
                        .await
                        .context("sending response")?;
                }
            }
            // Reply to keepalive pings so the CLI doesn't consider us dead.
            Message::Ping(payload) => {
                outgoing.send(Message::Pong(payload)).await.context("sending pong")?;
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    Ok(())
}

/// Routes a single incoming JSON-RPC message. Returns `Some(json)` to send back
/// for requests (those with an `id`), or `None` for notifications and for
/// messages we intentionally don't answer.
///
/// Kept free of sockets and GPUI so it is unit-testable against a mock
/// [`Dispatcher`].
pub async fn handle_message<D: Dispatcher>(raw: &str, dispatcher: &D) -> Option<String> {
    let parsed: Value = match serde_json::from_str(raw) {
        Ok(value) => value,
        Err(_) => {
            return Some(error_response(
                Value::Null,
                error_codes::PARSE_ERROR,
                "Parse error",
            ));
        }
    };

    if parsed.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
        let id = parsed.get("id").cloned().unwrap_or(Value::Null);
        return Some(error_response(id, error_codes::INVALID_REQUEST, "Invalid Request"));
    }

    let method = parsed.get("method").and_then(Value::as_str).unwrap_or_default();
    let params = parsed.get("params").cloned().unwrap_or(Value::Null);
    let id = parsed.get("id").cloned();

    match id {
        // A request: the client expects a response carrying the same `id`.
        Some(id) => Some(match route_request(method, params, dispatcher).await {
            Ok(result) => success_response(id, result),
            Err(error) => {
                error_response_with_data(id, error.code, &error.message, error.data)
            }
        }),
        // A notification: fire-and-forget, no response.
        None => {
            // `notifications/initialized` and friends need no action yet.
            None
        }
    }
}

async fn route_request<D: Dispatcher>(
    method: &str,
    params: Value,
    dispatcher: &D,
) -> Result<Value, ProtocolError> {
    match method {
        "initialize" => Ok(json!({
            "protocolVersion": MCP_PROTOCOL_VERSION,
            "capabilities": {
                "logging": {},
                "prompts": { "listChanged": true },
                "resources": { "subscribe": true, "listChanged": true },
                "tools": { "listChanged": true },
            },
            "serverInfo": { "name": SERVER_NAME, "version": SERVER_VERSION },
        })),
        "prompts/list" => Ok(json!({ "prompts": [] })),
        "resources/list" => Ok(json!({ "resources": [] })),
        "tools/list" => Ok(json!({ "tools": dispatcher.tools() })),
        "tools/call" => {
            let name = params
                .get("name")
                .and_then(Value::as_str)
                .ok_or_else(|| ProtocolError::new(error_codes::INVALID_REQUEST, "missing tool name"))?;
            let arguments = params.get("arguments").cloned().unwrap_or(json!({}));
            dispatcher.call_tool(name, arguments).await
        }
        other => Err(ProtocolError::method_not_found(other)),
    }
}

fn success_response(id: Value, result: Value) -> String {
    json!({ "jsonrpc": "2.0", "id": id, "result": result }).to_string()
}

fn error_response(id: Value, code: i32, message: &str) -> String {
    error_response_with_data(id, code, message, None)
}

fn error_response_with_data(id: Value, code: i32, message: &str, data: Option<Value>) -> String {
    let mut error = json!({ "code": code, "message": message });
    if let Some(data) = data {
        error["data"] = data;
    }
    json!({ "jsonrpc": "2.0", "id": id, "error": error }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    struct MockDispatcher;

    impl Dispatcher for MockDispatcher {
        fn tools(&self) -> Vec<ToolDescriptor> {
            vec![ToolDescriptor {
                name: "getCurrentSelection",
                description: "Get the current selection",
                input_schema: json!({ "type": "object" }),
            }]
        }

        async fn call_tool(&self, name: &str, arguments: Value) -> Result<Value, ProtocolError> {
            if name == "getCurrentSelection" {
                Ok(json!({ "content": [{ "type": "text", "text": "ok" }] }))
            } else {
                Err(ProtocolError::method_not_found(name))
            }
            .map(|result| {
                // Thread `arguments` through so the test exercises the path.
                let _ = &arguments;
                result
            })
        }
    }

    fn response(raw: &str) -> Value {
        let json = block_on(handle_message(raw, &MockDispatcher)).expect("expected a response");
        serde_json::from_str(&json).unwrap()
    }

    #[test]
    fn initialize_reports_protocol_version() {
        let value = response(r#"{"jsonrpc":"2.0","id":1,"method":"initialize"}"#);
        assert_eq!(value["id"], 1);
        assert_eq!(value["result"]["protocolVersion"], MCP_PROTOCOL_VERSION);
        assert_eq!(value["result"]["serverInfo"]["name"], "zed");
    }

    #[test]
    fn tools_list_returns_advertised_tools() {
        let value = response(r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#);
        assert_eq!(value["result"]["tools"][0]["name"], "getCurrentSelection");
        assert_eq!(value["result"]["tools"][0]["inputSchema"]["type"], "object");
    }

    #[test]
    fn tools_call_wraps_result_in_mcp_content() {
        let value = response(
            r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"getCurrentSelection","arguments":{}}}"#,
        );
        assert_eq!(value["result"]["content"][0]["type"], "text");
        assert_eq!(value["result"]["content"][0]["text"], "ok");
    }

    #[test]
    fn unknown_tool_is_a_method_not_found_error() {
        let value = response(
            r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"nope","arguments":{}}}"#,
        );
        assert_eq!(value["error"]["code"], error_codes::METHOD_NOT_FOUND);
    }

    #[test]
    fn unknown_method_is_a_method_not_found_error() {
        let value = response(r#"{"jsonrpc":"2.0","id":5,"method":"frobnicate"}"#);
        assert_eq!(value["error"]["code"], error_codes::METHOD_NOT_FOUND);
    }

    #[test]
    fn notification_yields_no_response() {
        let result = block_on(handle_message(
            r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
            &MockDispatcher,
        ));
        assert!(result.is_none());
    }

    #[test]
    fn malformed_json_is_a_parse_error() {
        let value = response("not json at all");
        assert_eq!(value["error"]["code"], error_codes::PARSE_ERROR);
        assert_eq!(value["id"], Value::Null);
    }
}
