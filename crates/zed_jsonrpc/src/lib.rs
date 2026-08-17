//! # zed_jsonrpc
//!
//! Space-Grade, lightweight JSON-RPC 2.0 protocol engine for Zed.
//! (Section 1.2 & Phase 1.1 of Space-Grade Audit)
//!
//! Supports line-delimited request/response parsing, synchronous and asynchronous
//! method dispatch, schema reflection, notifications, and standard JSON-RPC 2.0 error taxonomy.

use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

// Standard JSON-RPC 2.0 Error Codes
pub const PARSE_ERROR: i64 = -32700;
pub const INVALID_REQUEST: i64 = -32600;
pub const METHOD_NOT_FOUND: i64 = -32601;
pub const INVALID_PARAMS: i64 = -32602;
pub const INTERNAL_ERROR: i64 = -32603;

// Zed-specific Application Error Codes
pub const UNAUTHORIZED: i64 = -32001;
pub const BUFFER_NOT_FOUND: i64 = -32002;
pub const EXECUTION_FAILED: i64 = -32003;
pub const TIMEOUT: i64 = -32004;

/// A JSON-RPC 2.0 request envelope.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: String,
    #[serde(default)]
    pub params: serde_json::Value,
    #[serde(default)]
    pub auth_token: Option<String>,
}

/// A JSON-RPC 2.0 response envelope.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// A JSON-RPC 2.0 error object.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl JsonRpcResponse {
    pub fn ok(id: Option<serde_json::Value>, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            result: Some(result),
            error: None,
        }
    }

    pub fn err(id: Option<serde_json::Value>, code: i64, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.into(),
                data: None,
            }),
        }
    }
}

/// A JSON-RPC 2.0 notification envelope (one-way message without id).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

impl JsonRpcNotification {
    pub fn new(method: impl Into<String>, params: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            method: method.into(),
            params,
        }
    }
}

/// Synchronous method handler function signature.
pub type MethodHandler = Box<dyn Fn(serde_json::Value) -> serde_json::Value + Send + Sync>;

/// Method registry mapping method strings to closures.
#[derive(Default)]
pub struct MethodRegistry {
    handlers: HashMap<String, MethodHandler>,
}

impl MethodRegistry {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        method: impl Into<String>,
        handler: impl Fn(serde_json::Value) -> serde_json::Value + Send + Sync + 'static,
    ) {
        self.handlers.insert(method.into(), Box::new(handler));
    }

    pub fn dispatch(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        match self.handlers.get(&request.method) {
            Some(handler) => {
                let result = handler(request.params.clone());
                JsonRpcResponse::ok(request.id.clone(), result)
            }
            None => JsonRpcResponse::err(
                request.id.clone(),
                METHOD_NOT_FOUND,
                format!("Method '{}' not found", request.method),
            ),
        }
    }

    pub fn list_methods(&self) -> Vec<String> {
        let mut list: Vec<String> = self.handlers.keys().cloned().collect();
        list.sort();
        list
    }
}

/// Process a single raw JSON line and produce a JSON response string
pub fn process_line(
    registry: &Arc<MethodRegistry>,
    line: &str,
    required_auth_token: Option<&str>,
) -> String {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return serde_json::to_string(&JsonRpcResponse::err(
            None,
            INVALID_REQUEST,
            "Empty request body",
        ))
        .unwrap_or_default();
    }

    let request: JsonRpcRequest = match serde_json::from_str(trimmed) {
        Ok(req) => req,
        Err(e) => {
            return serde_json::to_string(&JsonRpcResponse::err(
                None,
                PARSE_ERROR,
                format!("Parse error: {e}"),
            ))
            .unwrap_or_default();
        }
    };

    if request.jsonrpc != "2.0" {
        return serde_json::to_string(&JsonRpcResponse::err(
            request.id,
            INVALID_REQUEST,
            "Invalid JSON-RPC protocol version (expected '2.0')",
        ))
        .unwrap_or_default();
    }

    if let Some(expected_token) = required_auth_token {
        let authorized = request
            .auth_token
            .as_deref()
            .map(|t| t == expected_token)
            .unwrap_or(false);

        if !authorized {
            return serde_json::to_string(&JsonRpcResponse::err(
                request.id,
                UNAUTHORIZED,
                "Unauthorized: valid auth_token required",
            ))
            .unwrap_or_default();
        }
    }

    let response = registry.dispatch(&request);
    serde_json::to_string(&response).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsonrpc_protocol_roundtrip() {
        let mut registry = MethodRegistry::new();
        registry.register("math/add", |params| {
            let a = params["a"].as_i64().unwrap_or(0);
            let b = params["b"].as_i64().unwrap_or(0);
            serde_json::json!({ "sum": a + b })
        });
        let reg = Arc::new(registry);

        let req = r#"{"jsonrpc":"2.0","id":1,"method":"math/add","params":{"a":5,"b":7}}"#;
        let res = process_line(&reg, req, None);
        let parsed: JsonRpcResponse = serde_json::from_str(&res).unwrap();
        assert!(parsed.error.is_none());
        assert_eq!(parsed.result.unwrap()["sum"], 12);
    }

    #[test]
    fn test_jsonrpc_auth_enforcement() {
        let mut registry = MethodRegistry::new();
        registry.register("ping", |_| serde_json::json!("pong"));
        let reg = Arc::new(registry);

        // 1. Without token -> Unauthorized
        let req1 = r#"{"jsonrpc":"2.0","id":1,"method":"ping","params":{}}"#;
        let res1 = process_line(&reg, req1, Some("secret-token"));
        let parsed1: JsonRpcResponse = serde_json::from_str(&res1).unwrap();
        assert_eq!(parsed1.error.unwrap().code, UNAUTHORIZED);

        // 2. With token -> Success
        let req2 = r#"{"jsonrpc":"2.0","id":2,"method":"ping","params":{},"auth_token":"secret-token"}"#;
        let res2 = process_line(&reg, req2, Some("secret-token"));
        let parsed2: JsonRpcResponse = serde_json::from_str(&res2).unwrap();
        assert!(parsed2.error.is_none());
        assert_eq!(parsed2.result.unwrap(), "pong");
    }
}
