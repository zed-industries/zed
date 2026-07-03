use serde::{Deserialize, Serialize};

use crate::tools::ToolHost;

/// Protocol version this server implements. If a client advertises an older
/// version during `initialize`, we echo the client's version back.
pub const PROTOCOL_VERSION: &str = "2025-06-18";

pub const PARSE_ERROR: i32 = -32700;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;

/// A single JSON-RPC 2.0 request or notification. A missing `id` marks a
/// notification, to which no response is sent. Per JSON-RPC 2.0, an *explicit*
/// `"id": null` is still a request and must receive a response, so the absent
/// and null cases are tracked separately via [`RequestId`].
#[derive(Deserialize)]
pub struct RpcRequest {
    #[serde(default)]
    pub id: RequestId,
    pub method: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

/// Distinguishes an absent `id` (a notification) from an explicit `"id": null`.
///
/// `serde` collapses both an omitted field and an explicit JSON `null` into
/// `Option::None` when the field type is `Option<T>`, so a plain
/// `Option<serde_json::Value>` cannot tell them apart. Because `#[serde(default)]`
/// only fills in a value when the key is *absent*, defaulting to
/// [`RequestId::Absent`] while deserializing present values (including null)
/// into [`RequestId::Present`] recovers the distinction JSON-RPC 2.0 requires.
#[derive(Clone, Default)]
pub enum RequestId {
    /// The `id` key was not present: this message is a notification.
    #[default]
    Absent,
    /// The `id` key was present (possibly with a `null` value): this message is
    /// a request and must receive a response echoing this id.
    Present(serde_json::Value),
}

impl<'de> Deserialize<'de> for RequestId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Reached only when the key is present (serde uses the `Default` impl
        // otherwise), so any value here — including `null` — is an explicit id.
        serde_json::Value::deserialize(deserializer).map(RequestId::Present)
    }
}

#[derive(Serialize)]
pub struct RpcResponse {
    pub jsonrpc: &'static str,
    pub id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
}

#[derive(Serialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}

impl RpcResponse {
    fn result(id: serde_json::Value, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0",
            id,
            result: Some(result),
            error: None,
        }
    }

    fn error(id: serde_json::Value, code: i32, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: "2.0",
            id,
            result: None,
            error: Some(RpcError {
                code,
                message: message.into(),
            }),
        }
    }
}

/// Builds the response to a parse failure of a raw stdin line. The spec assigns
/// a null id to messages that could not be parsed.
pub fn parse_error(message: impl Into<String>) -> RpcResponse {
    RpcResponse::error(serde_json::Value::Null, PARSE_ERROR, message)
}

/// Dispatches a single request against the host. Returns `None` for
/// notifications (requests without an `id`), which receive no reply.
///
/// This is pure with respect to stdio so it can be unit tested directly.
pub async fn handle_request(request: RpcRequest, host: &mut ToolHost) -> Option<RpcResponse> {
    // Notifications carry no id and never receive a response. An explicit
    // `"id": null` is a request and is answered with a null-id response.
    let id = match request.id.clone() {
        RequestId::Absent => return None,
        RequestId::Present(id) => id,
    };

    let response = match request.method.as_str() {
        "initialize" => RpcResponse::result(id, initialize_result(&request.params)),
        "ping" => RpcResponse::result(id, serde_json::json!({})),
        "tools/list" => RpcResponse::result(
            id,
            serde_json::json!({ "tools": ToolHost::tool_definitions() }),
        ),
        "tools/call" => tools_call(id, &request.params, host).await,
        other => RpcResponse::error(id, METHOD_NOT_FOUND, format!("unknown method: {other}")),
    };
    Some(response)
}

fn initialize_result(params: &serde_json::Value) -> serde_json::Value {
    // Echo the client's protocol version when it is older than ours; otherwise
    // advertise our own. We simply mirror whatever the client requested if
    // present, which keeps older clients happy without negotiation logic.
    let protocol_version = params
        .get("protocolVersion")
        .and_then(|value| value.as_str())
        .filter(|version| *version <= PROTOCOL_VERSION)
        .unwrap_or(PROTOCOL_VERSION);

    serde_json::json!({
        "protocolVersion": protocol_version,
        "capabilities": { "tools": {} },
        "serverInfo": { "name": "zed-database-mcp", "version": "0.1.0" },
    })
}

async fn tools_call(
    id: serde_json::Value,
    params: &serde_json::Value,
    host: &mut ToolHost,
) -> RpcResponse {
    let Some(name) = params.get("name").and_then(|value| value.as_str()) else {
        return RpcResponse::error(id, INVALID_PARAMS, "missing tool name");
    };
    let arguments = params
        .get("arguments")
        .cloned()
        .unwrap_or(serde_json::Value::Null);

    match host.call(name, &arguments).await {
        Ok(result) => {
            let text = serde_json::to_string_pretty(&result)
                .unwrap_or_else(|error| format!("failed to serialize result: {error}"));
            RpcResponse::result(
                id,
                serde_json::json!({
                    "content": [{ "type": "text", "text": text }],
                    "isError": false,
                }),
            )
        }
        Err(error) => RpcResponse::result(
            id,
            serde_json::json!({
                // Alternate formatting includes the full `anyhow` context chain
                // so the agent sees the underlying cause, not just the outer
                // wrapper.
                "content": [{ "type": "text", "text": format!("{error:#}") }],
                "isError": true,
            }),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::ToolHost;
    use database_client::DatabaseClient;
    use database_client::fake::FakeDatabaseClient;
    use std::sync::Arc;

    fn empty_host() -> ToolHost {
        ToolHost::new(
            Vec::new(),
            200,
            Box::new(|_config, _database, _password| {
                Arc::new(FakeDatabaseClient::new()) as Arc<dyn DatabaseClient>
            }),
            Box::new(|_config, _database, _password| {
                Arc::new(FakeDatabaseClient::new()) as Arc<dyn DatabaseClient>
            }),
            Box::new(|_config| Ok("pw".to_string())),
            std::collections::HashSet::new(),
            crate::token_store::TokenStore::new(std::time::Duration::from_secs(300)),
        )
    }

    fn request(
        id: Option<serde_json::Value>,
        method: &str,
        params: serde_json::Value,
    ) -> RpcRequest {
        RpcRequest {
            id: match id {
                Some(id) => RequestId::Present(id),
                None => RequestId::Absent,
            },
            method: method.to_string(),
            params,
        }
    }

    #[tokio::test]
    async fn initialize_returns_server_info() {
        let mut host = empty_host();
        let response = handle_request(
            request(
                Some(serde_json::json!(1)),
                "initialize",
                serde_json::json!({ "protocolVersion": PROTOCOL_VERSION }),
            ),
            &mut host,
        )
        .await
        .expect("initialize must respond");
        let result = response.result.expect("initialize returns a result");
        assert_eq!(result["protocolVersion"], PROTOCOL_VERSION);
        assert_eq!(result["serverInfo"]["name"], "zed-database-mcp");
        assert_eq!(result["serverInfo"]["version"], "0.1.0");
        assert!(result["capabilities"]["tools"].is_object());
    }

    #[tokio::test]
    async fn initialize_echoes_older_client_version() {
        let mut host = empty_host();
        let response = handle_request(
            request(
                Some(serde_json::json!(1)),
                "initialize",
                serde_json::json!({ "protocolVersion": "2024-11-05" }),
            ),
            &mut host,
        )
        .await
        .expect("initialize must respond");
        let result = response.result.unwrap();
        assert_eq!(result["protocolVersion"], "2024-11-05");
    }

    #[tokio::test]
    async fn tools_list_returns_six_tools() {
        let mut host = empty_host();
        let response = handle_request(
            request(
                Some(serde_json::json!(2)),
                "tools/list",
                serde_json::Value::Null,
            ),
            &mut host,
        )
        .await
        .expect("tools/list must respond");
        let tools = response.result.unwrap()["tools"]
            .as_array()
            .unwrap()
            .clone();
        let names: Vec<&str> = tools
            .iter()
            .filter_map(|tool| tool["name"].as_str())
            .collect();
        assert_eq!(
            names,
            vec![
                "list_connections",
                "list_tables",
                "describe_table",
                "run_query",
                "propose_write",
                "apply_write"
            ]
        );
    }

    #[tokio::test]
    async fn unknown_method_returns_method_not_found() {
        let mut host = empty_host();
        let response = handle_request(
            request(
                Some(serde_json::json!(3)),
                "does/not/exist",
                serde_json::Value::Null,
            ),
            &mut host,
        )
        .await
        .expect("a request with an id always responds");
        let error = response.error.expect("unknown method is an error");
        assert_eq!(error.code, METHOD_NOT_FOUND);
    }

    #[tokio::test]
    async fn notification_receives_no_response() {
        let mut host = empty_host();
        let response = handle_request(
            request(None, "notifications/initialized", serde_json::Value::Null),
            &mut host,
        )
        .await;
        assert!(response.is_none());
    }

    #[tokio::test]
    async fn tools_call_unknown_tool_is_tool_error_not_rpc_error() {
        let mut host = empty_host();
        let response = handle_request(
            request(
                Some(serde_json::json!(4)),
                "tools/call",
                serde_json::json!({ "name": "bogus", "arguments": {} }),
            ),
            &mut host,
        )
        .await
        .expect("tools/call responds");
        // Unknown tool must be an isError result, not a JSON-RPC error.
        assert!(response.error.is_none());
        let result = response.result.unwrap();
        assert_eq!(result["isError"], true);
        let text = result["content"][0]["text"].as_str().unwrap();
        assert!(text.contains("unknown tool"));
    }

    #[tokio::test]
    async fn explicit_null_id_request_receives_response() {
        // Parse from raw JSON to exercise the real deserialization path: an
        // explicit `"id": null` is a request (not a notification) and must be
        // answered with a null-id response per JSON-RPC 2.0.
        let mut host = empty_host();
        let request: RpcRequest =
            serde_json::from_str(r#"{"jsonrpc":"2.0","id":null,"method":"ping"}"#).unwrap();
        let response = handle_request(request, &mut host)
            .await
            .expect("explicit null id is a request and must respond");
        assert_eq!(response.id, serde_json::Value::Null);
        assert_eq!(response.result.unwrap(), serde_json::json!({}));
    }

    #[tokio::test]
    async fn absent_id_is_notification() {
        // A message with no `id` key at all is a notification and gets no reply,
        // even when deserialized from raw JSON.
        let mut host = empty_host();
        let request: RpcRequest =
            serde_json::from_str(r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#)
                .unwrap();
        let response = handle_request(request, &mut host).await;
        assert!(response.is_none());
    }

    #[tokio::test]
    async fn ping_returns_empty_object() {
        let mut host = empty_host();
        let response = handle_request(
            request(Some(serde_json::json!(5)), "ping", serde_json::Value::Null),
            &mut host,
        )
        .await
        .expect("ping responds");
        assert_eq!(response.result.unwrap(), serde_json::json!({}));
    }
}
