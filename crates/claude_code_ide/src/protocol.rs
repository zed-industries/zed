//! Wire types for the Claude Code IDE bridge: JSON-RPC 2.0 framing plus the MCP
//! payload shapes Claude Code expects.
//!
//! `context_server` already models MCP, but its request/response structs are
//! `pub(crate)` and shaped for its Unix-socket + stdio transport, so we define the
//! small subset we need here rather than reach into its internals.
//!
//! Only shapes that are deserialised, or built in more than one place, are structs.
//! One-shot responses are built with `json!` at the point of use, which keeps the
//! wire text next to the code that decides it.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use serde_json::value::RawValue;

pub const JSON_RPC_VERSION: &str = "2.0";

/// An inbound JSON-RPC message. A request carries an `id`, a notification does not.
/// `params` stays raw so each method can deserialize its own shape.
#[derive(Debug, Deserialize)]
pub struct IncomingMessage {
    #[allow(dead_code)]
    pub jsonrpc: Option<String>,
    #[serde(default)]
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Option<Box<RawValue>>,
}

impl IncomingMessage {
    pub fn is_notification(&self) -> bool {
        self.id.is_none()
    }
}

pub mod error_code {
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    #[allow(dead_code)]
    pub const INTERNAL_ERROR: i32 = -32603;
}

pub fn ok_response<T: Serialize>(id: Value, result: T) -> Value {
    json!({ "jsonrpc": JSON_RPC_VERSION, "id": id, "result": result })
}

pub fn err_response(id: Value, code: i32, message: impl Into<String>) -> Value {
    let message: String = message.into();
    json!({
        "jsonrpc": JSON_RPC_VERSION,
        "id": id,
        "error": { "code": code, "message": message },
    })
}

pub fn notification<T: Serialize>(method: &str, params: T) -> Value {
    json!({ "jsonrpc": JSON_RPC_VERSION, "method": method, "params": params })
}

/// Parameters of a `tools/call` request.
#[derive(Debug, Deserialize)]
pub struct CallToolParams {
    pub name: String,
    #[serde(default)]
    pub arguments: Value,
}

/// A tool result. Claude Code reads `content[].text`, so every result we produce is
/// a single text block whose body is the JSON the CLI parses.
pub fn tool_ok(text: impl Into<String>) -> Value {
    let text: String = text.into();
    json!({ "content": [{ "type": "text", "text": text }] })
}

/// A tool-level error, distinct from a JSON-RPC transport error.
pub fn tool_error(text: impl Into<String>) -> Value {
    let text: String = text.into();
    json!({ "content": [{ "type": "text", "text": text }], "isError": true })
}

/// A zero-based line/character position, matching the protocol's coordinates.
#[derive(Debug, Serialize, Clone, Copy)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Serialize, Clone, Copy)]
pub struct SelectionRange {
    pub start: Position,
    pub end: Position,
}

/// Payload for the selection tools and for `selection_changed`.
#[derive(Debug, Serialize, Clone)]
pub struct SelectionPayload {
    pub text: String,
    #[serde(rename = "filePath")]
    pub file_path: String,
    #[serde(rename = "fileUrl")]
    pub file_url: String,
    pub selection: SelectionRange,
}

/// A `selection_changed` payload saying there is no selection any more.
///
/// A `Value` rather than a [`SelectionPayload`] because that struct's `selection` is
/// not optional, and making it optional would add a `None` case to every read path
/// that currently cannot have one.
pub fn cleared_selection() -> Value {
    json!({ "text": "", "filePath": "", "fileUrl": "", "selection": Value::Null })
}

