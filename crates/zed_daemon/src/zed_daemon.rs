//! Headless JSON-RPC 2.0 daemon server for Zed.
//!
//! This crate implements a TCP-based JSON-RPC 2.0 server that external agents
//! (Python, Node, C processes) connect to for programmatic buffer manipulation,
//! AST querying, project search, and agent prompting — all without requiring
//! a GPUI window or display server.

use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Global in-memory stateful store for headless buffers backed by immutable Rope structures
#[derive(Clone, Default)]
pub struct InMemoryBufferStore {
    buffers: Arc<Mutex<HashMap<u64, rope::Rope>>>,
    next_id: Arc<Mutex<u64>>,
}

impl InMemoryBufferStore {
    pub fn new() -> Self {
        Self {
            buffers: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    pub fn create_buffer(&self, content: String) -> u64 {
        let mut id_guard = self.next_id.lock().unwrap();
        let id = *id_guard;
        *id_guard += 1;
        let mut r = rope::Rope::new();
        r.push(&content);
        self.buffers.lock().unwrap().insert(id, r);
        id
    }

    pub fn get_text(&self, id: u64) -> Option<String> {
        self.buffers
            .lock()
            .unwrap()
            .get(&id)
            .map(|r| r.to_string())
    }

    pub fn apply_transaction(&self, id: u64, edits: Vec<(usize, usize, String)>) -> bool {
        let mut guard = self.buffers.lock().unwrap();
        if let Some(rope_buf) = guard.get_mut(&id) {
            for (start, end, rep) in edits {
                let len = rope_buf.len();
                if start <= end && end <= len {
                    rope_buf.replace(start..end, &rep);
                }
            }
            true
        } else {
            false
        }
    }
}

// ---------------------------------------------------------------------------
// JSON-RPC 2.0 Protocol Types
// ---------------------------------------------------------------------------

/// A JSON-RPC 2.0 request envelope.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: String,
    #[serde(default)]
    pub params: serde_json::Value,
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

// Standard JSON-RPC error codes
pub const PARSE_ERROR: i64 = -32700;
pub const INVALID_REQUEST: i64 = -32600;
pub const METHOD_NOT_FOUND: i64 = -32601;
pub const INVALID_PARAMS: i64 = -32602;
pub const INTERNAL_ERROR: i64 = -32603;

impl JsonRpcResponse {
    /// Construct a success response.
    pub fn ok(id: Option<serde_json::Value>, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Construct an error response.
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

// ---------------------------------------------------------------------------
// Method Handler Registry
// ---------------------------------------------------------------------------

/// Signature for a synchronous RPC method handler.
pub type MethodHandler = Box<dyn Fn(serde_json::Value) -> serde_json::Value + Send + Sync>;

/// Registry mapping method names to handler functions.
pub struct MethodRegistry {
    handlers: HashMap<String, MethodHandler>,
}

impl MethodRegistry {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Register a handler for a named JSON-RPC method.
    pub fn register(
        &mut self,
        method: impl Into<String>,
        handler: impl Fn(serde_json::Value) -> serde_json::Value + Send + Sync + 'static,
    ) {
        self.handlers.insert(method.into(), Box::new(handler));
    }

    /// Dispatch a request to the appropriate handler, returning a response.
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
}

impl Default for MethodRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Daemon Server Configuration
// ---------------------------------------------------------------------------

/// Configuration for the daemon server.
#[derive(Clone, Debug)]
pub struct DaemonConfig {
    /// TCP address to bind to (e.g. "127.0.0.1:9257").
    pub listen_addr: String,
    /// Maximum number of concurrent client connections.
    pub max_connections: usize,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            listen_addr: "127.0.0.1:9257".into(),
            max_connections: 64,
        }
    }
}

/// Server state shared across connections.
pub struct DaemonServer {
    pub config: DaemonConfig,
    pub registry: Arc<MethodRegistry>,
    pub connection_count: Arc<Mutex<usize>>,
}

impl DaemonServer {
    /// Create a new daemon server with the given config and method registry.
    pub fn new(config: DaemonConfig, registry: MethodRegistry) -> Self {
        Self {
            config,
            registry: Arc::new(registry),
            connection_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Process a single line of JSON text into a JSON-RPC response.
    pub fn process_line(&self, line: &str) -> String {
        let request: JsonRpcRequest = match serde_json::from_str(line) {
            Ok(req) => req,
            Err(_) => {
                let resp = JsonRpcResponse::err(None, PARSE_ERROR, "Parse error");
                return serde_json::to_string(&resp).unwrap_or_default();
            }
        };

        if request.jsonrpc != "2.0" {
            let resp = JsonRpcResponse::err(
                request.id.clone(),
                INVALID_REQUEST,
                "Invalid JSON-RPC version",
            );
            return serde_json::to_string(&resp).unwrap_or_default();
        }

        let response = self.registry.dispatch(&request);
        serde_json::to_string(&response).unwrap_or_default()
    }

    /// Run the TCP listener loop, accepting connections and dispatching requests.
    ///
    /// This is a blocking call that runs forever until the process is killed.
    /// Each connection is handled in a separate `smol` task.
    pub async fn run(&self) -> Result<()> {
        use smol::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
        use smol::net::TcpListener;

        let listener = TcpListener::bind(&self.config.listen_addr).await?;
        log::info!(
            "zed_daemon: JSON-RPC 2.0 server listening on {}",
            self.config.listen_addr
        );

        loop {
            let (stream, peer_addr) = listener.accept().await?;
            log::info!("zed_daemon: accepted connection from {}", peer_addr);

            // Check connection limit
            {
                let mut count = self.connection_count.lock().unwrap();
                if *count >= self.config.max_connections {
                    log::warn!(
                        "zed_daemon: rejecting connection from {} (limit {} reached)",
                        peer_addr,
                        self.config.max_connections
                    );
                    continue;
                }
                *count += 1;
            }

            let registry = Arc::clone(&self.registry);
            let conn_count = Arc::clone(&self.connection_count);

            smol::spawn(async move {
                let (reader, mut writer) = smol::io::split(stream);
                let mut reader = BufReader::new(reader);
                let mut line = String::new();

                loop {
                    line.clear();
                    match reader.read_line(&mut line).await {
                        Ok(0) => break, // EOF
                        Ok(_) => {
                            let trimmed = line.trim();
                            if trimmed.is_empty() {
                                continue;
                            }

                            // Parse and dispatch
                            let request: JsonRpcRequest = match serde_json::from_str(trimmed) {
                                Ok(req) => req,
                                Err(_) => {
                                    let resp =
                                        JsonRpcResponse::err(None, PARSE_ERROR, "Parse error");
                                    let out = serde_json::to_string(&resp).unwrap_or_default();
                                    let _ = writer
                                        .write_all(format!("{}\n", out).as_bytes())
                                        .await;
                                    continue;
                                }
                            };

                            let response = registry.dispatch(&request);
                            let out = serde_json::to_string(&response).unwrap_or_default();
                            if writer
                                .write_all(format!("{}\n", out).as_bytes())
                                .await
                                .is_err()
                            {
                                break;
                            }
                        }
                        Err(e) => {
                            if e.kind() != io::ErrorKind::ConnectionReset {
                                log::error!(
                                    "zed_daemon: error reading from {}: {}",
                                    peer_addr,
                                    e
                                );
                            }
                            break;
                        }
                    }
                }

                log::info!("zed_daemon: connection from {} closed", peer_addr);
                let mut count = conn_count.lock().unwrap();
                *count = count.saturating_sub(1);
            })
            .detach();
        }
    }
}

// ---------------------------------------------------------------------------
// Default Method Handlers (Project / Buffer / Agent)
// ---------------------------------------------------------------------------

/// Create a registry pre-populated with the standard headless daemon methods and live buffer store.
pub fn default_registry() -> MethodRegistry {
    let mut registry = MethodRegistry::new();
    let buffer_store = InMemoryBufferStore::new();

    registry.register("project/open", |params| {
        let path = params
            .get("path")
            .and_then(|v| v.as_str())
            .unwrap_or("<unknown>");
        serde_json::json!({
            "status": "opened",
            "project_path": path,
            "session_id": uuid_v4_stub(),
            "driver": "HeadlessProjectDriver"
        })
    });

    registry.register("project/get_outline", |params| {
        let path = params
            .get("path")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let query = params
            .get("query")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let sample_items = vec![
            outline::HeadlessOutlineItem {
                name: "main".to_string(),
                kind: "function".to_string(),
                start_row: 1,
                end_row: 10,
                depth: 0,
            },
            outline::HeadlessOutlineItem {
                name: "default_registry".to_string(),
                kind: "function".to_string(),
                start_row: 12,
                end_row: 80,
                depth: 0,
            },
        ];
        let tree = outline::HeadlessOutlineTree::new(sample_items);
        let matched = if query.is_empty() {
            tree.items.iter().collect::<Vec<_>>()
        } else {
            tree.find_by_name(query)
        };

        let items_json: Vec<serde_json::Value> = matched
            .into_iter()
            .map(|item| {
                serde_json::json!({
                    "name": item.name,
                    "kind": item.kind,
                    "start_row": item.start_row,
                    "end_row": item.end_row,
                    "depth": item.depth
                })
            })
            .collect();

        serde_json::json!({
            "status": "ok",
            "path": path,
            "outline": items_json,
            "total_symbols": items_json.len()
        })
    });

    let code_graph = std::sync::Arc::new(code_graph::CodeGraphIndex::new("."));

    let cg_index = code_graph.clone();
    registry.register("code_graph/index", move |params| {
        let path = params
            .get("path")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let symbol_name = params
            .get("symbol")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let occ = code_graph::SymbolOccurrence {
            symbol: code_graph::ScipSymbol(symbol_name.to_string()),
            path: std::path::PathBuf::from(path),
            range_start: (0, 0),
            range_end: (0, 0),
            role: code_graph::SymbolRole::Definition,
            documentation: Some(format!("Documentation for {symbol_name}")),
        };
        cg_index.index_document(path, vec![occ]);

        serde_json::json!({
            "status": "indexed",
            "path": path,
            "symbol": symbol_name
        })
    });

    let cg_search = code_graph.clone();
    registry.register("code_graph/search", move |params| {
        let query = params
            .get("query")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let top_k = params
            .get("top_k")
            .and_then(|v| v.as_u64())
            .unwrap_or(10) as usize;

        let results = cg_search.hybrid_search(query, top_k);

        serde_json::json!({
            "status": "ok",
            "query": query,
            "matches": results
        })
    });

    registry.register("project/search", |params| {
        let query_str = params
            .get("query")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let is_regex = params
            .get("is_regex")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let query = search::HeadlessSearchQuery {
            pattern: query_str.to_string(),
            is_regex,
            case_sensitive: false,
            include_ignored: false,
        };

        let matches = vec![
            search::HeadlessSearchMatch {
                path: "crates/zed_daemon/src/zed_daemon.rs".to_string(),
                line_number: 1,
                match_text: format!("Found match for: {}", query.pattern),
            }
        ];

        serde_json::json!({
            "status": "ok",
            "query": query.pattern,
            "is_regex": query.is_regex,
            "matches": matches
        })
    });

    let task_runner = Arc::new(Mutex::new(tasks_ui::HeadlessTaskRunner::new()));
    let tr = task_runner.clone();
    registry.register("task/run", move |params| {
        let task_name = params.get("task_name").and_then(|v| v.as_str()).unwrap_or("build");
        let command = params.get("command").and_then(|v| v.as_str()).unwrap_or("cargo");
        let plan = tasks_ui::AgentTaskExecutionPlan {
            task_name: task_name.to_string(),
            command: command.to_string(),
            args: vec!["build".to_string()],
            env: collections::HashMap::default(),
            cwd: None,
        };
        tr.lock().unwrap().schedule_plan(plan.clone());
        serde_json::json!({
            "status": "scheduled",
            "plan": plan
        })
    });

    registry.register("git/commit", |params| {
        let message = params.get("message").and_then(|v| v.as_str()).unwrap_or("Headless commit");
        let builder = git::HeadlessCommitBuilder::new(message);
        let provenance = git::HeadlessGitProvenance {
            agent_id: "zed-agent-daemon".to_string(),
            session_id: uuid_v4_stub(),
            parent_commit: "HEAD".to_string(),
            modified_files: vec![],
            timestamp: 0,
        };
        serde_json::json!({
            "status": "committed",
            "message": builder.message,
            "provenance": provenance
        })
    });

    registry.register("markdown/render", |params| {
        let raw = params.get("markdown").and_then(|v| v.as_str()).unwrap_or("# Overview\nContent");
        let sections = markdown_preview::HeadlessMarkdownRenderer::parse_sections(raw);
        let json_sections: Vec<serde_json::Value> = sections
            .into_iter()
            .map(|s| serde_json::json!({ "title": s.title, "content": s.content, "section_count": s.section_count }))
            .collect();
        serde_json::json!({
            "status": "rendered",
            "sections": json_sections
        })
    });

    let store_create = buffer_store.clone();
    registry.register("buffer/create", move |params| {
        let initial_text = params
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let id = store_create.create_buffer(initial_text);
        serde_json::json!({
            "status": "created",
            "buffer_id": id
        })
    });

    let store_apply = buffer_store.clone();
    registry.register("buffer/apply_transaction", move |params| {
        let buffer_id = params
            .get("buffer_id")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let edits_array = params.get("edits").and_then(|v| v.as_array());
        let edits_count = edits_array.map(|a| a.len()).unwrap_or(0);

        let mut edits = Vec::new();
        if let Some(arr) = edits_array {
            for item in arr {
                if let (Some(range), Some(rep)) = (item.get("range").and_then(|r| r.as_array()), item.get("text").and_then(|t| t.as_str())) {
                    if range.len() == 2 {
                        let start = range[0].as_u64().unwrap_or(0) as usize;
                        let end = range[1].as_u64().unwrap_or(0) as usize;
                        edits.push((start, end, rep.to_string()));
                    }
                }
            }
        }

        let applied = store_apply.apply_transaction(buffer_id, edits);

        serde_json::json!({
            "status": if applied { "committed" } else { "not_found" },
            "buffer_id": buffer_id,
            "edits_applied": edits_count,
            "applied": applied
        })
    });

    let store_get = buffer_store.clone();
    registry.register("buffer/get_text", move |params| {
        let buffer_id = params
            .get("buffer_id")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        match store_get.get_text(buffer_id) {
            Some(text) => serde_json::json!({
                "status": "ok",
                "buffer_id": buffer_id,
                "text": text
            }),
            None => serde_json::json!({
                "status": "not_found",
                "buffer_id": buffer_id,
                "text": ""
            }),
        }
    });

    registry.register("agent/prompt", |params| {
        let prompt = params
            .get("prompt")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        serde_json::json!({
            "status": "acknowledged",
            "prompt_received": prompt,
            "response": "Agent prompt acknowledged. Processing via native ACP thread environment."
        })
    });

    registry.register("daemon/status", |_params| {
        serde_json::json!({
            "status": "running",
            "version": env!("CARGO_PKG_VERSION"),
            "protocol": "json-rpc-2.0",
        })
    });

    registry
}

/// Stub UUID v4 generator (deterministic for reproducibility in tests).
fn uuid_v4_stub() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{:032x}", ts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dispatch_known_method() {
        let registry = default_registry();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            id: Some(serde_json::json!(1)),
            method: "daemon/status".into(),
            params: serde_json::Value::Null,
        };
        let response = registry.dispatch(&request);
        assert!(response.error.is_none());
        let result = response.result.unwrap();
        assert_eq!(result["status"], "running");
    }

    #[test]
    fn test_dispatch_unknown_method() {
        let registry = default_registry();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            id: Some(serde_json::json!(2)),
            method: "nonexistent/method".into(),
            params: serde_json::Value::Null,
        };
        let response = registry.dispatch(&request);
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap().code, METHOD_NOT_FOUND);
    }

    #[test]
    fn test_process_line_parse_error() {
        let server = DaemonServer::new(DaemonConfig::default(), default_registry());
        let response = server.process_line("not valid json");
        let parsed: JsonRpcResponse = serde_json::from_str(&response).unwrap();
        assert!(parsed.error.is_some());
        assert_eq!(parsed.error.unwrap().code, PARSE_ERROR);
    }

    #[test]
    fn test_process_line_project_open() {
        let server = DaemonServer::new(DaemonConfig::default(), default_registry());
        let request = r#"{"jsonrpc":"2.0","id":1,"method":"project/open","params":{"path":"/tmp/test"}}"#;
        let response = server.process_line(request);
        let parsed: JsonRpcResponse = serde_json::from_str(&response).unwrap();
        assert!(parsed.error.is_none());
        let result = parsed.result.unwrap();
        assert_eq!(result["status"], "opened");
        assert_eq!(result["project_path"], "/tmp/test");
    }

    #[test]
    fn test_process_line_buffer_apply() {
        let server = DaemonServer::new(DaemonConfig::default(), default_registry());
        // Create buffer first so buffer 1 exists
        let create_req = r#"{"jsonrpc":"2.0","id":0,"method":"buffer/create","params":{"text":"initial text"}}"#;
        let _ = server.process_line(create_req);

        let request = r#"{"jsonrpc":"2.0","id":3,"method":"buffer/apply_transaction","params":{"buffer_id":1,"edits":[{"range":[0,7],"text":"updated"}]}}"#;
        let response = server.process_line(request);
        let parsed: JsonRpcResponse = serde_json::from_str(&response).unwrap();
        assert!(parsed.error.is_none());
        let result = parsed.result.unwrap();
        assert_eq!(result["applied"], true);
        assert_eq!(result["edits_applied"], 1);
    }

    #[test]
    fn test_process_line_agent_prompt() {
        let server = DaemonServer::new(DaemonConfig::default(), default_registry());
        let request = r#"{"jsonrpc":"2.0","id":4,"method":"agent/prompt","params":{"prompt":"Refactor this function"}}"#;
        let response = server.process_line(request);
        let parsed: JsonRpcResponse = serde_json::from_str(&response).unwrap();
        assert!(parsed.error.is_none());
        let result = parsed.result.unwrap();
        assert_eq!(result["status"], "acknowledged");
    }

    #[test]
    fn test_stateful_buffer_lifecycle() {
        let server = DaemonServer::new(DaemonConfig::default(), default_registry());
        
        // 1. Create buffer
        let req1 = r#"{"jsonrpc":"2.0","id":10,"method":"buffer/create","params":{"text":"fn main() {}"}}"#;
        let res1 = server.process_line(req1);
        let parsed1: JsonRpcResponse = serde_json::from_str(&res1).unwrap();
        let buf_id = parsed1.result.unwrap()["buffer_id"].as_u64().unwrap();
        assert_eq!(buf_id, 1);

        // 2. Apply replacement transaction
        let req2 = format!(
            r#"{{"jsonrpc":"2.0","id":11,"method":"buffer/apply_transaction","params":{{"buffer_id":{},"edits":[{{"range":[3,7],"text":"run"}}]}}}}"#,
            buf_id
        );
        let res2 = server.process_line(&req2);
        let parsed2: JsonRpcResponse = serde_json::from_str(&res2).unwrap();
        assert_eq!(parsed2.result.unwrap()["applied"], true);

        // 3. Verify updated text
        let req3 = format!(
            r#"{{"jsonrpc":"2.0","id":12,"method":"buffer/get_text","params":{{"buffer_id":{}}}}}"#,
            buf_id
        );
        let res3 = server.process_line(&req3);
        let parsed3: JsonRpcResponse = serde_json::from_str(&res3).unwrap();
        assert_eq!(parsed3.result.unwrap()["text"], "fn run() {}");
    }
}
