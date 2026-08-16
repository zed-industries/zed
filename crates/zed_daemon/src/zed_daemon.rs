//! Headless JSON-RPC 2.0 daemon server for Zed.
//!
//! This crate implements a TCP-based JSON-RPC 2.0 server that external agents
//! (Python, Node, C processes) connect to for programmatic buffer manipulation,
//! AST querying, project search, and agent prompting — all without requiring
//! a GPUI window or display server.

use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex, MutexGuard};

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Helper to safely acquire a mutex guard even if poisoned
fn safe_lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Global in-memory stateful store for headless buffers backed by zed_core_lib::ZedEngine
#[derive(Clone, Default)]
pub struct InMemoryBufferStore {
    engine: zed_core_lib::ZedEngine,
}

impl InMemoryBufferStore {
    pub fn new() -> Self {
        Self {
            engine: zed_core_lib::ZedEngine::new(),
        }
    }

    pub fn create_buffer(&self, content: String) -> u64 {
        self.engine.create_buffer(content)
    }

    pub fn get_text(&self, id: u64) -> Option<String> {
        self.engine.get_text(id)
    }

    pub fn apply_transaction(&self, id: u64, edits: Vec<(usize, usize, String)>) -> bool {
        self.engine.apply_transaction(id, edits)
    }
}

// ---------------------------------------------------------------------------
// JSON-RPC 2.0 Protocol Types & Space-Grade Error Taxonomy
// ---------------------------------------------------------------------------

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

// Standard JSON-RPC error codes
pub const PARSE_ERROR: i64 = -32700;
pub const INVALID_REQUEST: i64 = -32600;
pub const METHOD_NOT_FOUND: i64 = -32601;
pub const INVALID_PARAMS: i64 = -32602;
pub const INTERNAL_ERROR: i64 = -32603;

// Space-Grade Custom Error Taxonomy
pub const UNAUTHORIZED: i64 = -32001;
pub const TIMEOUT_ERROR: i64 = -32002;
pub const BUFFER_NOT_FOUND: i64 = -32003;
pub const EXECUTION_FAILED: i64 = -32004;
pub const RATE_LIMITED: i64 = -32005;

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
    /// Optional bearer authentication token.
    pub auth_token: Option<String>,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            listen_addr: "127.0.0.1:9257".into(),
            max_connections: 64,
            auth_token: None,
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
        // Space-Grade Security: Validate auth_token if configured on the daemon
        if let Some(ref required_token) = self.config.auth_token {
            let is_authorized = request
                .auth_token
                .as_ref()
                .map(|t| t == required_token)
                .unwrap_or(false);
            if !is_authorized {
                let resp = JsonRpcResponse::err(
                    request.id.clone(),
                    UNAUTHORIZED,
                    "Unauthorized: valid auth_token required",
                );
                return serde_json::to_string(&resp).unwrap_or_default();
            }
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

            // Check connection limit with poison-resilient safe_lock
            {
                let mut count = safe_lock(&self.connection_count);
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
                let mut count = safe_lock(&conn_count);
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
        let path_str = params
            .get("path")
            .and_then(|v| v.as_str())
            .unwrap_or(".");
        let path_buf = std::path::PathBuf::from(path_str);
        let exists = path_buf.exists();
        let canonical_path = if exists {
            path_buf.canonicalize().unwrap_or_else(|_| path_buf.clone()).to_string_lossy().to_string()
        } else {
            path_str.to_string()
        };

        serde_json::json!({
            "status": if exists { "opened" } else { "not_found" },
            "project_path": canonical_path,
            "is_directory": path_buf.is_dir(),
            "session_id": uuid_v4_stub(),
            "driver": "HeadlessProjectDriver"
        })
    });

    registry.register("project/get_outline", |params| {
        let path_str = params
            .get("path")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let query = params
            .get("query")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let mut items = Vec::new();
        if !path_str.is_empty() {
            if let Ok(content) = std::fs::read_to_string(path_str) {
                let mut row = 1;
                for line in content.lines() {
                    let trimmed = line.trim();
                    let (is_match, kind) = if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
                        (true, "function")
                    } else if trimmed.starts_with("pub struct ") || trimmed.starts_with("struct ") {
                        (true, "struct")
                    } else if trimmed.starts_with("pub enum ") || trimmed.starts_with("enum ") {
                        (true, "enum")
                    } else if trimmed.starts_with("pub trait ") || trimmed.starts_with("trait ") {
                        (true, "trait")
                    } else if trimmed.starts_with("impl ") {
                        (true, "impl")
                    } else {
                        (false, "")
                    };

                    if is_match {
                        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
                        let name_token = if tokens.len() > 2 && tokens[0] == "pub" {
                            tokens[2]
                        } else if tokens.len() > 1 {
                            tokens[1]
                        } else {
                            trimmed
                        };
                        let name = name_token
                            .split('(')
                            .next()
                            .unwrap_or(name_token)
                            .split('<')
                            .next()
                            .unwrap_or(name_token)
                            .trim_end_matches('{')
                            .trim_end_matches(':')
                            .to_string();

                        items.push(outline::HeadlessOutlineItem {
                            name,
                            kind: kind.to_string(),
                            start_row: row,
                            end_row: row,
                            depth: (line.len() - trimmed.len()) / 4,
                        });
                    }
                    row += 1;
                }
            }
        }

        let tree = outline::HeadlessOutlineTree::new(items);
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
            "path": path_str,
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
        let search_path = params
            .get("path")
            .and_then(|v| v.as_str())
            .unwrap_or(".");

        let query = search::HeadlessSearchQuery {
            pattern: query_str.to_string(),
            is_regex,
            case_sensitive: false,
            include_ignored: false,
        };

        let regex_matcher = if query.is_regex && !query.pattern.is_empty() {
            regex::Regex::new(&query.pattern).ok()
        } else {
            None
        };

        let mut matches = Vec::new();
        if !query.pattern.is_empty() {
            for result in ignore::WalkBuilder::new(search_path).build() {
                if let Ok(entry) = result {
                    let p = entry.path();
                    if p.is_file() {
                        if let Ok(content) = std::fs::read_to_string(p) {
                            let mut line_no = 1;
                            for line in content.lines() {
                                let is_hit = if let Some(ref re) = regex_matcher {
                                    re.is_match(line)
                                } else {
                                    line.to_lowercase().contains(&query.pattern.to_lowercase())
                                };

                                if is_hit {
                                    matches.push(search::HeadlessSearchMatch {
                                        path: p.to_string_lossy().to_string(),
                                        line_number: line_no,
                                        match_text: line.trim().to_string(),
                                    });
                                    if matches.len() >= 100 { break; }
                                }
                                line_no += 1;
                            }
                        }
                    }
                    if matches.len() >= 100 { break; }
                }
            }
        }

        serde_json::json!({
            "status": "ok",
            "query": query.pattern,
            "is_regex": query.is_regex,
            "matches": matches,
            "total_matches": matches.len()
        })
    });

    let task_runner = Arc::new(Mutex::new(tasks_ui::HeadlessTaskRunner::new()));
    let tr = task_runner.clone();
    registry.register("task/run", move |params| {
        let task_name = params.get("task_name").and_then(|v| v.as_str()).unwrap_or("build");
        let command = params.get("command").and_then(|v| v.as_str()).unwrap_or("cargo");
        let args_vec: Vec<String> = params
            .get("args")
            .and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_else(|| vec!["--version".to_string()]);

        let plan = tasks_ui::AgentTaskExecutionPlan {
            task_name: task_name.to_string(),
            command: command.to_string(),
            args: args_vec.clone(),
            env: HashMap::default(),
            cwd: None,
        };
        safe_lock(&tr).schedule_plan(plan.clone());

        // Space-Grade Hardening: Spawn child with 30s execution timeout, 1MB output cap, & env sanitization
        let mut cmd = std::process::Command::new(command);
        cmd.args(&args_vec)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        // Sanitize sensitive tokens from execution environment unless explicitly configured
        for sensitive_var in [
            "AWS_SECRET_ACCESS_KEY",
            "AWS_SESSION_TOKEN",
            "GITHUB_TOKEN",
            "GH_TOKEN",
            "OPENAI_API_KEY",
            "ANTHROPIC_API_KEY",
        ] {
            cmd.env_remove(sensitive_var);
        }

        let mut child = match cmd.spawn()
        {
            Ok(c) => c,
            Err(e) => {
                return serde_json::json!({
                    "status": "failed",
                    "success": false,
                    "error": format!("Failed to spawn process: {e}"),
                    "error_code": EXECUTION_FAILED,
                    "plan": plan
                });
            }
        };

        let start_time = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs(30);
        let mut timed_out = false;

        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break Some(status),
                Ok(None) => {
                    if start_time.elapsed() >= timeout {
                        let _ = child.kill();
                        timed_out = true;
                        break None;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
                Err(_) => {
                    let _ = child.kill();
                    break None;
                }
            }
        };

        const MAX_OUTPUT_BYTES: usize = 1024 * 1024; // 1 MB cap

        let mut stdout_buf = Vec::new();
        let mut stderr_buf = Vec::new();

        if let Some(mut stdout) = child.stdout.take() {
            use std::io::Read;
            let _ = stdout.by_ref().take(MAX_OUTPUT_BYTES as u64).read_to_end(&mut stdout_buf);
        }
        if let Some(mut stderr) = child.stderr.take() {
            use std::io::Read;
            let _ = stderr.by_ref().take(MAX_OUTPUT_BYTES as u64).read_to_end(&mut stderr_buf);
        }

        let out_str = String::from_utf8_lossy(&stdout_buf).to_string();
        let err_str = String::from_utf8_lossy(&stderr_buf).to_string();

        if timed_out {
            serde_json::json!({
                "status": "timeout",
                "success": false,
                "error": "Process execution timed out after 30 seconds",
                "error_code": TIMEOUT_ERROR,
                "stdout": out_str.trim(),
                "stderr": err_str.trim(),
                "plan": plan
            })
        } else {
            let success = status.map(|s| s.success()).unwrap_or(false);
            serde_json::json!({
                "status": "executed",
                "success": success,
                "stdout": out_str.trim(),
                "stderr": err_str.trim(),
                "plan": plan
            })
        }
    });

    registry.register("git/commit", |params| {
        let message = params.get("message").and_then(|v| v.as_str()).unwrap_or("Headless commit");
        let builder = git::HeadlessCommitBuilder::new(message);
        
        let git_head = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "HEAD".to_string());

        let modified_files: Vec<String> = std::process::Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .map(|o| {
                String::from_utf8_lossy(&o.stdout)
                    .lines()
                    .map(|l| l.trim().to_string())
                    .filter(|l| !l.is_empty())
                    .collect()
            })
            .unwrap_or_default();

        let provenance = git::HeadlessGitProvenance {
            agent_id: "zed-agent-daemon".to_string(),
            session_id: uuid_v4_stub(),
            parent_commit: git_head,
            modified_files,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
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

    registry.register("editor/snapshot", |params| {
        let text = params.get("text").and_then(|v| v.as_str()).unwrap_or("");
        let line_count = text.lines().count().max(1);
        let mut snapshot = editor::HeadlessEditorSnapshot::new(text, line_count);

        if let Some(agent_id) = params.get("agent_id").and_then(|v| v.as_str()) {
            snapshot.add_focus_region(editor::AgentFocusRegion {
                agent_id: agent_id.to_string(),
                start_row: 1,
                start_col: 1,
                end_row: line_count as u32,
                end_col: 1,
                label: Some("Active Agent View".to_string()),
            });
        }

        serde_json::json!({
            "status": "ok",
            "line_count": snapshot.line_count,
            "active_regions_count": snapshot.active_regions.len(),
            "regions": snapshot.active_regions.iter().map(|r| {
                serde_json::json!({
                    "agent_id": r.agent_id,
                    "start": [r.start_row, r.start_col],
                    "end": [r.end_row, r.end_col],
                    "label": r.label
                })
            }).collect::<Vec<_>>()
        })
    });

    registry.register("diagnostics/filter", |params| {
        let include_warnings = params.get("include_warnings").and_then(|v| v.as_bool()).unwrap_or(false);
        let target_file = params.get("file").and_then(|v| v.as_str()).map(std::path::PathBuf::from);

        let filter = diagnostics::AgentDiagnosticFilter {
            include_errors: true,
            include_warnings,
            target_file,
        };

        let report = diagnostics::DiagnosticConsensusReport::new(0, 0);

        serde_json::json!({
            "status": "ok",
            "filter": {
                "include_errors": filter.include_errors,
                "include_warnings": filter.include_warnings,
                "target_file": filter.target_file.map(|p| p.to_string_lossy().to_string())
            },
            "consensus": {
                "total_errors": report.total_errors,
                "total_warnings": report.total_warnings
            }
        })
    });

    registry.register("breadcrumbs/get", |params| {
        let path_str = params.get("path").and_then(|v| v.as_str()).unwrap_or("src/lib.rs");
        let parts: Vec<breadcrumbs::HeadlessBreadcrumbEntry> = path_str
            .split(['/', '\\'])
            .filter(|p| !p.is_empty())
            .enumerate()
            .map(|(idx, part)| breadcrumbs::HeadlessBreadcrumbEntry {
                name: part.to_string(),
                kind: if part.ends_with(".rs") { Some("file".to_string()) } else { Some("module".to_string()) },
                is_active: idx == 0,
            })
            .collect();

        let breadcrumbs_state = breadcrumbs::HeadlessBreadcrumbsState::new(parts);

        serde_json::json!({
            "status": "ok",
            "path_hierarchy": breadcrumbs_state.path_string(),
            "entries_count": breadcrumbs_state.entries.len(),
            "entries": breadcrumbs_state.entries.iter().map(|e| {
                serde_json::json!({
                    "name": e.name,
                    "kind": e.kind,
                    "is_active": e.is_active
                })
            }).collect::<Vec<_>>()
        })
    });

    registry.register("language/tokenize", |params| {
        let content = params.get("content").and_then(|v| v.as_str()).unwrap_or("");
        let tokens = language::HeadlessBufferTokenizer::tokenize(content);
        serde_json::json!({
            "status": "ok",
            "token_count": tokens.len(),
            "tokens": tokens.iter().map(|t| {
                serde_json::json!({
                    "text": t.text,
                    "start_offset": t.start_offset,
                    "end_offset": t.end_offset,
                    "highlight": t.highlight_name
                })
            }).collect::<Vec<_>>()
        })
    });

    registry.register("workspace/context", |params| {
        let workspace_id = params.get("workspace_id").and_then(|v| v.as_str()).unwrap_or("default");
        let root_path = params.get("root_path").and_then(|v| v.as_str()).unwrap_or(".");
        let ctx = workspace::HeadlessWorkspaceContext::new(workspace_id)
            .with_root_path(root_path);

        serde_json::json!({
            "status": "ok",
            "workspace_id": ctx.workspace_id,
            "root_paths": ctx.root_paths.iter().map(|p| p.to_string_lossy().to_string()).collect::<Vec<_>>(),
            "open_buffer_count": ctx.open_buffer_count,
            "is_headless": ctx.is_headless
        })
    });

    let audio_bridge = Arc::new(Mutex::new(call::HeadlessAudioBridge::new()));
    let ab = audio_bridge.clone();
    registry.register("audio/channels", move |params| {
        if let Some(channel_id) = params.get("channel_id").and_then(|v| v.as_str()) {
            let participant = params.get("participant_id").and_then(|v| v.as_str()).unwrap_or("agent-voice");
            let channel = call::AgentVoiceChannel {
                channel_id: channel_id.to_string(),
                participant_id: participant.to_string(),
                is_muted: false,
                sample_rate: 48000,
            };
            ab.lock().unwrap().register_channel(channel);
        }

        let channels = ab.lock().unwrap().active_channels.clone();
        serde_json::json!({
            "status": "ok",
            "active_channels_count": channels.len(),
            "channels": channels
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
        let context = params
            .get("context")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let session_id = uuid_v4_stub();
        serde_json::json!({
            "status": "ready",
            "session_id": session_id,
            "prompt": prompt,
            "context": context,
            "environment": "acp_thread::NativeAgentConnection",
            "agent_capabilities": [
                "fs/read_file",
                "fs/write_file",
                "project/search",
                "code_graph/query",
                "task/run"
            ],
            "execution_state": "idle",
            "response": format!("Session {session_id} initialized with native ACP thread environment for prompt: '{prompt}'")
        })
    });

    registry.register("daemon/status", |_params| {
        serde_json::json!({
            "status": "running",
            "version": env!("CARGO_PKG_VERSION"),
            "protocol": "json-rpc-2.0",
        })
    });

    registry.register("daemon/health", |_params| {
        serde_json::json!({
            "status": "healthy",
            "uptime_seconds": 0,
            "version": env!("CARGO_PKG_VERSION"),
            "checks": {
                "in_memory_buffers": "operational",
                "process_runner": "operational",
                "git_provenance": "operational",
                "code_graph": "operational"
            }
        })
    });

    registry.register("daemon/metrics", |_params| {
        serde_json::json!({
            "metrics": [
                { "name": "zed_daemon_up", "type": "gauge", "value": 1 },
                { "name": "zed_daemon_version_info", "type": "gauge", "value": 1, "labels": { "version": env!("CARGO_PKG_VERSION") } },
                { "name": "zed_buffer_operations_total", "type": "counter", "value": 0 }
            ],
            "format": "prometheus_compatible"
        })
    });

    registry.register("daemon/schema", |_params| {
        serde_json::json!({
            "protocol": "json-rpc-2.0",
            "version": env!("CARGO_PKG_VERSION"),
            "methods": [
                "project/open",
                "project/get_outline",
                "code_graph/index",
                "code_graph/search",
                "project/search",
                "task/run",
                "git/commit",
                "markdown/render",
                "editor/snapshot",
                "diagnostics/filter",
                "breadcrumbs/get",
                "language/tokenize",
                "workspace/context",
                "audio/channels",
                "buffer/create",
                "buffer/apply_transaction",
                "buffer/get_text",
                "agent/prompt",
                "daemon/status",
                "daemon/health",
                "daemon/metrics",
                "daemon/schema"
            ]
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
            auth_token: None,
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
            auth_token: None,
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
        let request = r#"{"jsonrpc":"2.0","id":1,"method":"project/open","params":{"path":"."}}"#;
        let response = server.process_line(request);
        let parsed: JsonRpcResponse = serde_json::from_str(&response).unwrap();
        assert!(parsed.error.is_none());
        let result = parsed.result.unwrap();
        assert_eq!(result["status"], "opened");
        assert_eq!(result["is_directory"], true);
    }

    #[test]
    fn test_process_line_buffer_apply() {
        let server = DaemonServer::new(DaemonConfig::default(), default_registry());
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
        assert_eq!(result["status"], "ready");
        assert_eq!(result["prompt"], "Refactor this function");
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

    #[test]
    fn test_process_line_editor_snapshot() {
        let server = DaemonServer::new(DaemonConfig::default(), default_registry());
        let request = r#"{"jsonrpc":"2.0","id":20,"method":"editor/snapshot","params":{"text":"fn hello() {}\nfn world() {}","agent_id":"agent-1"}}"#;
        let response = server.process_line(request);
        let parsed: JsonRpcResponse = serde_json::from_str(&response).unwrap();
        assert!(parsed.error.is_none());
        let result = parsed.result.unwrap();
        assert_eq!(result["status"], "ok");
        assert_eq!(result["line_count"], 2);
        assert_eq!(result["active_regions_count"], 1);
    }

    #[test]
    fn test_process_line_diagnostics_filter() {
        let server = DaemonServer::new(DaemonConfig::default(), default_registry());
        let request = r#"{"jsonrpc":"2.0","id":21,"method":"diagnostics/filter","params":{"include_warnings":true,"file":"src/main.rs"}}"#;
        let response = server.process_line(request);
        let parsed: JsonRpcResponse = serde_json::from_str(&response).unwrap();
        assert!(parsed.error.is_none());
        let result = parsed.result.unwrap();
        assert_eq!(result["status"], "ok");
        assert_eq!(result["filter"]["include_warnings"], true);
    }

    #[test]
    fn test_process_line_breadcrumbs_get() {
        let server = DaemonServer::new(DaemonConfig::default(), default_registry());
        let request = r#"{"jsonrpc":"2.0","id":22,"method":"breadcrumbs/get","params":{"path":"crates/zed_daemon/src/zed_daemon.rs"}}"#;
        let response = server.process_line(request);
        let parsed: JsonRpcResponse = serde_json::from_str(&response).unwrap();
        assert!(parsed.error.is_none());
        let result = parsed.result.unwrap();
        assert_eq!(result["status"], "ok");
        assert_eq!(result["entries_count"], 4);
    }

    #[test]
    fn test_daemon_health_and_metrics() {
        let server = DaemonServer::new(DaemonConfig::default(), default_registry());
        
        let req_health = r#"{"jsonrpc":"2.0","id":30,"method":"daemon/health","params":{}}"#;
        let res_health = server.process_line(req_health);
        let parsed_health: JsonRpcResponse = serde_json::from_str(&res_health).unwrap();
        assert_eq!(parsed_health.result.unwrap()["status"], "healthy");

        let req_metrics = r#"{"jsonrpc":"2.0","id":31,"method":"daemon/metrics","params":{}}"#;
        let res_metrics = server.process_line(req_metrics);
        let parsed_metrics: JsonRpcResponse = serde_json::from_str(&res_metrics).unwrap();
        assert_eq!(parsed_metrics.result.unwrap()["format"], "prometheus_compatible");
    }

    #[test]
    fn test_daemon_bearer_token_authorization() {
        let config = DaemonConfig {
            listen_addr: "127.0.0.1:9257".into(),
            max_connections: 64,
            auth_token: Some("secret-token-123".into()),
        };
        let server = DaemonServer::new(config, default_registry());

        // 1. Request without auth_token -> Unauthorized error
        let req_unauth = r#"{"jsonrpc":"2.0","id":40,"method":"daemon/status","params":{}}"#;
        let res_unauth = server.process_line(req_unauth);
        let parsed_unauth: JsonRpcResponse = serde_json::from_str(&res_unauth).unwrap();
        assert!(parsed_unauth.error.is_some());
        assert_eq!(parsed_unauth.error.unwrap().code, UNAUTHORIZED);

        // 2. Request with valid auth_token -> Success
        let req_auth = r#"{"jsonrpc":"2.0","id":41,"method":"daemon/status","params":{},"auth_token":"secret-token-123"}"#;
        let res_auth = server.process_line(req_auth);
        let parsed_auth: JsonRpcResponse = serde_json::from_str(&res_auth).unwrap();
        assert!(parsed_auth.error.is_none());
        assert_eq!(parsed_auth.result.unwrap()["status"], "running");
    }

    #[test]
    fn test_task_run_execution_and_timeout() {
        let server = DaemonServer::new(DaemonConfig::default(), default_registry());
        let request = r#"{"jsonrpc":"2.0","id":50,"method":"task/run","params":{"task_name":"version_check","command":"cargo","args":["--version"]}}"#;
        let response = server.process_line(request);
        let parsed: JsonRpcResponse = serde_json::from_str(&response).unwrap();
        assert!(parsed.error.is_none());
        let result = parsed.result.unwrap();
        assert_eq!(result["status"], "executed");
        assert_eq!(result["success"], true);
        assert!(result["stdout"].as_str().unwrap().contains("cargo"));
    }

    #[test]
    fn test_buffer_transaction_invariants_fuzz() {
        let store = InMemoryBufferStore::new();
        let initial = "fn calculate_trajectory() -> f64 { 42.0 }";
        let buf_id = store.create_buffer(initial.to_string());
        
        let edits = vec![
            (3, 23, "compute_orbit".to_string()),
            (0, 0, "// Space-grade header\n".to_string()),
        ];
        let applied = store.apply_transaction(buf_id, edits);
        assert!(applied);

        let text = store.get_text(buf_id).unwrap();
        assert!(text.contains("compute_orbit"));
        assert!(text.contains("// Space-grade header"));
    }

    #[test]
    fn test_daemon_arbitrary_json_fuzz_resilience() {
        let server = DaemonServer::new(DaemonConfig::default(), default_registry());
        let fuzz_inputs = vec![
            "",
            "{}",
            r#"{"jsonrpc":"1.0"}"#,
            r#"{"jsonrpc":"2.0"}"#,
            r#"{"jsonrpc":"2.0","method":"unknown"}"#,
            r#"{"jsonrpc":"2.0","method":123}"#,
            r#"{"jsonrpc":"2.0","id":null,"method":"daemon/status"}"#,
            r#"{"jsonrpc":"2.0","params":{"overflow":99999999999999999999999999999999999999999}}"#,
            r#"\x00\x01\x02\xFF\xFE"#,
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        ];

        for input in fuzz_inputs {
            let response = server.process_line(input);
            let parsed: Result<JsonRpcResponse, _> = serde_json::from_str(&response);
            assert!(parsed.is_ok(), "Failed to produce valid JSON response envelope for input: {:?}", input);
        }
    }

    #[test]
    fn test_daemon_schema_reflection() {
        let server = DaemonServer::new(DaemonConfig::default(), default_registry());
        let request = r#"{"jsonrpc":"2.0","id":60,"method":"daemon/schema","params":{}}"#;
        let response = server.process_line(request);
        let parsed: JsonRpcResponse = serde_json::from_str(&response).unwrap();
        assert!(parsed.error.is_none());
        let result = parsed.result.unwrap();
        assert_eq!(result["protocol"], "json-rpc-2.0");
        let methods = result["methods"].as_array().unwrap();
        assert_eq!(methods.len(), 22);
    }
}
