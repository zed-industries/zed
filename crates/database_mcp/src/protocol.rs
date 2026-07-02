use serde::{Deserialize, Serialize};

use crate::tools::ToolHost;

/// Protocol version this server implements. If a client advertises an older
/// version during `initialize`, we echo the client's version back.
pub const PROTOCOL_VERSION: &str = "2025-06-18";

pub const PARSE_ERROR: i32 = -32700;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;

/// A single JSON-RPC 2.0 request or notification. A missing `id` marks a
/// notification, to which no response is sent.
#[derive(Deserialize)]
pub struct RpcRequest {
    #[serde(default)]
    pub id: Option<serde_json::Value>,
    pub method: String,
    #[serde(default)]
    pub params: serde_json::Value,
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
    let id = request.id.clone();
    // Notifications carry no id and never receive a response.
    let Some(id) = id else {
        return None;
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
            Box::new(|_config, _database| {
                Arc::new(FakeDatabaseClient::new()) as Arc<dyn DatabaseClient>
            }),
            Box::new(|_config| Ok("pw".to_string())),
        )
    }

    fn request(
        id: Option<serde_json::Value>,
        method: &str,
        params: serde_json::Value,
    ) -> RpcRequest {
        RpcRequest {
            id,
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
    async fn tools_list_returns_four_tools() {
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
                "run_query"
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
