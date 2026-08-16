//! ACP v2.0 Protocol Formalization
//! 
//! Standardized ACP (Agent Collaboration Protocol) between local AI agents,
//! collaborative servers (collab), and external orchestrators.
//! 
//! This protocol replaces ad-hoc JSON-RPC messaging with a typed, schema-driven
//! protocol featuring bidirectional streaming, checkpoint/rollback persistence,
//! and session serialization formats.

/// ACP v2.0 Request types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AcpRequest {
    /// Initialize a new agent session
    Initialize(InitializeRequest),
    /// Send a message/token to the agent
    SendToken(SendTokenRequest),
    /// Checkpoint the current agent state
    Checkpoint(CheckpointRequest),
    /// Rollback the agent to a previous state
    Rollback(RollbackRequest),
    /// Query agent capabilities
    QueryCapabilities(QueryCapabilitiesRequest),
    /// Execute a command in the agent context
    ExecuteCommand(ExecuteCommandRequest),
    /// End the agent session
    Shutdown(ShutdownRequest),
}

/// ACP v2.0 Response types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AcpResponse {
    /// Initialization success with session details
    Initialize(AcpInitializeResponse),
    /// Token acceptance/rejection result
    SendToken(AcpSendTokenResponse),
    /// Checkpoint confirmation with snapshot ID
    Checkpoint(AcpCheckpointResponse),
    /// Rollback confirmation
    Rollback(AcpRollbackResponse),
    /// Capabilities query result
    QueryCapabilities(AcpCapabilitiesResponse),
    /// Command execution result
    ExecuteCommand(AcpExecuteCommandResponse),
    /// Shutdown acknowledgment
    Shutdown(AcpShutdownResponse),
}

/// Initialize request - starts a new agent session
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct InitializeRequest {
    /// Unique agent identifier
    pub agent_id: String,
    /// Agent type/category (e.g., "editor", "search", "git")
    pub agent_type: String,
    /// Initial configuration state
    pub config: serde_json::Value,
    /// Capabilities the agent supports
    pub capabilities: Vec<String>,
}

/// Initialize response - confirms session creation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct AcpInitializeResponse {
    /// Session token for subsequent requests
    pub session_token: String,
    /// Assigned agent session ID
    pub session_id: u64,
    /// Agent workspace directory
    pub workspace: String,
    /// Effective capabilities after validation
    pub effective_capabilities: Vec<String>,
}

/// Send token request - sends LLM tokens or data to agent
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct SendTokenRequest {
    /// Sequence number for ordering
    pub sequence: u64,
    /// The token/data payload
    pub payload: serde_json::Value,
    /// Optional metadata about the token
    pub metadata: Option<serde_json::Value>,
}

/// Send token response
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct AcpSendTokenResponse {
    /// Whether the token was accepted
    pub accepted: bool,
    /// Processed result or interpretation
    pub result: Option<serde_json::Value>,
    /// Next sequence number expected
    pub next_sequence: u64,
}

/// Checkpoint request - snapshots agent state for rollback
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct CheckpointRequest {
    /// Checkpoint name/identifier
    pub name: String,
    /// Reason for checkpoint
    pub reason: String,
    /// Include full state or diff-only
    pub full_state: bool,
}

/// Checkpoint response
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct AcpCheckpointResponse {
    /// Unique checkpoint identifier
    pub checkpoint_id: String,
    /// Timestamp when checkpoint was created
    pub timestamp: u64,
    /// Whether rollback to this checkpoint is supported
    pub rollback_supported: bool,
}

/// Rollback request - restore agent to previous checkpoint
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct RollbackRequest {
    /// Checkpoint ID to rollback to
    pub checkpoint_id: String,
    /// Whether to preserve any uncommitted changes
    pub preserve_unsaved: bool,
}

/// Rollback response
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct AcpRollbackResponse {
    /// Whether rollback succeeded
    pub success: bool,
    /// New state after rollback
    pub new_state: Option<serde_json::Value>,
    /// Error message if rollback failed
    pub error: Option<String>,
}

/// Query capabilities request
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct QueryCapabilitiesRequest {
    /// Specific capability to query, or empty for all
    pub capability: Option<String>,
}

/// Query capabilities response
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct AcpCapabilitiesResponse {
    /// Whether the capability is supported
    pub supported: bool,
    /// Detailed capability information
    pub details: Option<String>,
}

/// Execute command request
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ExecuteCommandRequest {
    /// Command to execute
    pub command: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Working directory for command
    pub working_dir: Option<String>,
    /// Timeout in seconds
    pub timeout: Option<u64>,
}

/// Execute command response
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct AcpExecuteCommandResponse {
    /// Whether execution succeeded
    pub success: bool,
    /// Command output/stdout
    pub output: Option<String>,
    /// Command stderr
    pub stderr: Option<String>,
    /// Execution exit code
    pub exit_code: Option<i32>,
}

/// Shutdown request - gracefully terminate agent session
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ShutdownRequest {
    /// Reason for shutdown
    pub reason: String,
    /// Whether to preserve state for future restart
    pub preserve_state: bool,
}

/// Shutdown response
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct AcpShutdownResponse {
    /// Whether shutdown succeeded
    pub success: bool,
    /// Session state that was preserved
    pub preserved_state: Option<String>,
}

/// Event types for ACP v2.0 bidirectional streaming
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AcpEvent {
    /// Agent is thinking/reasoning
    Thinking(ThinkingEvent),
    /// Agent has completed a task
    TaskCompleted(TaskCompletedEvent),
    /// Agent encountered an error
    Error(AcpErrorEvent),
    /// Capabilities have changed
    CapabilitiesChanged(CapabilitiesChangedEvent),
    /// Checkpoint was created
    CheckpointCreated(AcpCheckpointCreatedEvent),
}

/// Thinking event - agent is processing
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ThinkingEvent {
    /// Current thought step
    pub step: u64,
    /// Thinking progress (0.0 to 1.0)
    pub progress: f32,
    /// Optional thinking text/rationale
    pub rationale: Option<String>,
}

/// Task completed event
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct TaskCompletedEvent {
    /// Task identifier
    pub task_id: String,
    /// Task result summary
    pub result: Option<serde_json::Value>,
    /// Whether task succeeded
    pub success: bool,
    /// Execution duration in milliseconds
    pub duration_ms: u64,
}

/// Error event - agent encountered an issue
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct AcpErrorEvent {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Recoverable flag
    pub recoverable: bool,
}

/// Capabilities changed event
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct CapabilitiesChangedEvent {
    /// Previous capabilities
    pub previous: Vec<String>,
    /// New capabilities
    pub current: Vec<String>,
}

/// Checkpoint created event (broadcast)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct AcpCheckpointCreatedEvent {
    /// Checkpoint ID
    pub checkpoint_id: String,
    /// Timestamp
    pub timestamp: u64,
}

/// Session persistence format for ACP v2.0
/// Serializes complete agent session state for storage/transmission
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct AcpSessionSnapshot {
    /// Session identifier
    pub session_id: u64,
    /// Agent ID
    pub agent_id: String,
    /// Current checkpoint ID
    pub current_checkpoint: Option<String>,
    /// All checkpoints in the session
    pub checkpoints: Vec<AcpCheckpointSnapshot>,
    /// Current execution state
    pub current_state: serde_json::Value,
    /// Pending requests/operations
    pub pending: Vec<serde_json::Value>,
    /// Timestamp when snapshot was created
    pub timestamp: u64,
}

/// Individual checkpoint snapshot within a session
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct AcpCheckpointSnapshot {
    /// Checkpoint ID
    pub checkpoint_id: String,
    /// Timestamp when checkpoint was created
    pub timestamp: u64,
    /// Agent state at checkpoint
    pub state: serde_json::Value,
    /// Which operations were included
    pub included_operations: Vec<String>,
}

/// Schema reflection endpoint - provides JSON Schema for all ACP methods
/// This enables typed client generation and validation
pub fn schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "ACP v2.0 Protocol Schema",
        "type": "object",
        "properties": {
            "request": {
                "type": "object",
                "properties": {
                    "type": {
                        "type": "string",
                        "enum": [
                            "Initialize",
                            "SendToken",
                            "Checkpoint",
                            "Rollback",
                            "QueryCapabilities",
                            "ExecuteCommand",
                            "Shutdown"
                        ]
                    },
                    "agent_id": {"type": "string"},
                    "session_token": {"type": "string"},
                    "sequence": {"type": "integer"},
                    "payload": {"type": "object"},
                    "checkpoint_id": {"type": "string"},
                    "name": {"type": "string"},
                    "reason": {"type": "string"},
                    "full_state": {"type": "boolean"},
                    "checkpoint_supported": {"type": "boolean"},
                    "preserve_unsaved": {"type": "boolean"},
                    "capability": {"type": "string"},
                    "supported": {"type": "boolean"},
                    "details": {"type": "string"},
                    "command": {"type": "string"},
                    "args": {
                        "type": "array",
                        "items": {"type": "string"}
                    },
                    "working_dir": {"type": "string"},
                    "timeout": {"type": "integer"},
                    "success": {"type": "boolean"},
                    "preserve_state": {"type": "boolean"},
                    "reason": {"type": "string"},
                },
                "required": ["type"],
                "additionalProperties": false
            },
            "response": {
                "type": "object",
                "properties": {
                    "type": {
                        "type": "string",
                        "enum": [
                            "Initialize",
                            "SendToken",
                            "Checkpoint",
                            "Rollback",
                            "QueryCapabilities",
                            "ExecuteCommand",
                            "Shutdown"
                        ]
                    },
                    "session_token": {"type": "string"},
                    "session_id": {"type": "integer"},
                    "workspace": {"type": "string"},
                    "effective_capabilities": {
                        "type": "array",
                        "items": {"type": "string"}
                    },
                    "accepted": {"type": "boolean"},
                    "result": {"type": "object"},
                    "next_sequence": {"type": "integer"},
                    "checkpoint_id": {"type": "string"},
                    "timestamp": {"type": "integer"},
                    "success": {"type": "boolean"},
                    "new_state": {"type": "object"},
                    "error": {"type": "string"},
                    "output": {"type": "string"},
                    "exit_code": {"type": "integer"},
                    "preserved_state": {"type": "string"},
                    "success": {"type": "boolean"},
                },
                "required": ["type"],
                "additionalProperties": false
            },
            "event": {
                "type": "object",
                "properties": {
                    "type": {
                        "type": "string",
                        "enum": [
                            "Thinking",
                            "TaskCompleted",
                            "Error",
                            "CapabilitiesChanged",
                            "CheckpointCreated"
                        ]
                    },
                    "step": {"type": "integer"},
                    "progress": {"type": "number"},
                    "rationale": {"type": "string"},
                    "task_id": {"type": "string"},
                    "result": {"type": "object"},
                    "success": {"type": "boolean"},
                    "duration_ms": {"type": "integer"},
                    "code": {"type": "integer"},
                    "message": {"type": "string"},
                    "recoverable": {"type": "boolean"},
                    "previous": {
                        "type": "array",
                        "items": {"type": "string"}
                    },
                    "current": {
                        "type": "array",
                        "items": {"type": "string"}
                    },
                    "checkpoint_id": {"type": "string"},
                    "timestamp": {"type": "integer"},
                },
                "required": ["type"],
                "additionalProperties": false
            }
        },
        "required": ["request", "response", "event"],
        "additionalProperties": false
    })
}

/// Converts an ACP request to JSON-RPC 2.0 format for daemon compatibility
pub fn to_jsonrpc(request: &AcpRequest) -> serde_json::Value {
    use AcpRequest::*;
    match request {
        Initialize(req) => serde_json::json!({
            "jsonrpc": "2.0",
            "method": "agent/initialize",
            "params": {
                "agent_id": req.agent_id,
                "agent_type": req.agent_type,
                "config": req.config,
                "capabilities": req.capabilities
            }
        }),
        SendToken(req) => serde_json::json!({
            "jsonrpc": "2.0",
            "method": "agent/send_token",
            "params": {
                "sequence": req.sequence,
                "payload": req.payload,
                "metadata": req.metadata
            }
        }),
        Checkpoint(req) => serde_json::json!({
            "jsonrpc": "2.0",
            "method": "agent/checkpoint",
            "params": {
                "name": req.name,
                "reason": req.reason,
                "full_state": req.full_state
            }
        }),
        Rollback(req) => serde_json::json!({
            "jsonrpc": "2.0",
            "method": "agent/rollback",
            "params": {
                "checkpoint_id": req.checkpoint_id,
                "preserve_unsaved": req.preserve_unsaved
            }
        }),
        QueryCapabilities(req) => serde_json::json!({
            "jsonrpc": "2.0",
            "method": "agent/query_capabilities",
            "params": {
                "capability": req.capability
            }
        }),
        ExecuteCommand(req) => serde_json::json!({
            "jsonrpc": "2.0",
            "method": "agent/execute_command",
            "params": {
                "command": req.command,
                "args": req.args,
                "working_dir": req.working_dir,
                "timeout": req.timeout
            }
        }),
        Shutdown(req) => serde_json::json!({
            "jsonrpc": "2.0",
            "method": "agent/shutdown",
            "params": {
                "reason": req.reason,
                "preserve_state": req.preserve_state
            }
        }),
    }
}

/// Converts a JSON-RPC 2.0 ACP response into AcpResponse enum
pub fn from_jsonrpc(response: &serde_json::Value) -> Option<AcpResponse> {
    use AcpResponse::*;
    let r#type = response.get("result")?.get("type")?.as_str()?;
    Some(match r#type {
        "Initialize" => Initialize(Initialize(
            response.get("result")?.get("session_token")?.as_str()?.unwrap_or("").to_string(),
        )),
        "SendToken" => SendToken(SendToken(
            response.get("result")?.get("accepted")?.as_bool()?unwrap_or(false),
        )),
        "Checkpoint" => Checkpoint(Checkpoint(
            response.get("result")?.get("checkpoint_id")?.as_str()?.unwrap_or("").to_string(),
        )),
        "Rollback" => Rollback(Rollback(
            response.get("result")?.get("success")?.as_bool()?unwrap_or(false),
        )),
        "QueryCapabilities" => QueryCapabilities(QueryCapabilities(
            response.get("result")?.get("supported")?.as_bool()?unwrap_or(false),
        )),
        "ExecuteCommand" => ExecuteCommand(ExecuteCommand(
            response.get("result")?.get("success")?.as_bool()?unwrap_or(false),
        )),
        "Shutdown" => Shutdown(Shutdown(
            response.get("result")?.get("success")?.as_bool()?unwrap_or(false),
        )),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acp_schema_is_valid() {
        let s = schema();
        assert!(s.get("request").is_some());
        assert!(s.get("response").is_some());
        assert!(s.get("event").is_some());
    }

    #[test]
    fn test_to_from_jsonrpc_roundtrip() {
        let req = AcpRequest::Initialize(Initialize {
            agent_id: "test-agent".to_string(),
            agent_type: "editor".to_string(),
            config: serde_json::json!({}),
            capabilities: vec!["edit".to_string()],
        });
        let json = to_jsonrpc(&req);
        assert_eq!(json.get("method").unwrap_as<String>(), "agent/initialize");
        assert_eq!(json.get("params").unwrap().get("agent_id").unwrap_as<String>(), "test-agent");

        // Round-trip through JSON
        let back = from_jsonrpc(&json);
        match back {
            Some(AcpResponse::Initialize(resp)) => {
                assert_eq!(resp.session_token, "");
            }
            _ => panic!("Expected Initialize response"),
        }
    }

    #[test]
    fn test_session_snapshot_serialization() {
        let snapshot = AcpSessionSnapshot {
            session_id: 42,
            agent_id: "test-agent".to_string(),
            current_checkpoint: Some("cp1".to_string()),
            checkpoints: vec![AcpCheckpointSnapshot {
                checkpoint_id: "cp1".to_string(),
                timestamp: 1000,
                state: serde_json::json!({}),
                included_operations: vec!["edit".to_string()],
            }],
            current_state: serde_json::json!({}),
            pending: vec![],
            timestamp: 1000,
        };

        let serialized = serde_json::to_string(&snapshot).unwrap();
        let deserialized: AcpSessionSnapshot = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.session_id, 42);
        assert_eq!(deserialized.agent_id, "test-agent");
    }
}