//! Tools module - Native Zed tool integration
//!
//! This module provides the tool interface and built-in Zed tools
//! that give Claude native access to:
//! - File system operations
//! - Buffer/editor access
//! - LSP diagnostics
//! - Git operations
//! - Terminal/shell execution
//! - And more

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// A tool call from Claude
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Unique ID for this tool call
    pub id: String,
    /// Name of the tool to call
    pub name: String,
    /// Input parameters as JSON
    pub input: Value,
}

/// Result of a tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// ID of the tool call this is responding to
    pub tool_use_id: String,
    /// Result content
    pub content: ToolOutput,
    /// Whether this result is an error
    pub is_error: bool,
}

/// Output from a tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolOutput {
    /// Text output
    Text(String),
    /// Structured JSON output
    Json(Value),
    /// Multiple content blocks
    Blocks(Vec<ToolOutputBlock>),
}

/// A content block in tool output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ToolOutputBlock {
    /// Text content
    #[serde(rename = "text")]
    Text { text: String },
    /// Image content (base64)
    #[serde(rename = "image")]
    Image {
        source: ImageSource,
    },
}

/// Image source for tool output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub media_type: String,
    pub data: String,
}

/// Tool schema definition for Claude
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSchema {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// Input schema (JSON Schema)
    pub input_schema: Value,
}

/// Trait for implementing tools
#[async_trait]
pub trait Tool: Send + Sync {
    /// Get the tool name
    fn name(&self) -> &str;

    /// Get the tool description
    fn description(&self) -> &str;

    /// Get the input schema
    fn input_schema(&self) -> Value;

    /// Execute the tool with given input
    async fn execute(&self, input: &Value) -> Result<ToolOutput>;

    /// Get the full tool schema
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            input_schema: self.input_schema(),
        }
    }
}

// =============================================================================
// Built-in Zed Tools
// =============================================================================

/// Read file tool - reads content from files in the workspace
pub struct ReadFileTool;

#[async_trait]
impl Tool for ReadFileTool {
    fn name(&self) -> &str {
        "read_file"
    }

    fn description(&self) -> &str {
        "Read the contents of a file from the workspace. Returns the file content as text."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the file to read, relative to workspace root"
                },
                "start_line": {
                    "type": "integer",
                    "description": "Optional start line (1-indexed)"
                },
                "end_line": {
                    "type": "integer",
                    "description": "Optional end line (1-indexed)"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, input: &Value) -> Result<ToolOutput> {
        // TODO: Integrate with Zed's worktree/buffer system
        let path = input["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing path parameter"))?;

        // Placeholder - will be replaced with actual Zed integration
        Ok(ToolOutput::Text(format!(
            "TODO: Read file from Zed workspace: {}",
            path
        )))
    }
}

/// Write file tool - writes content to files
pub struct WriteFileTool;

#[async_trait]
impl Tool for WriteFileTool {
    fn name(&self) -> &str {
        "write_file"
    }

    fn description(&self) -> &str {
        "Write content to a file in the workspace. Creates the file if it doesn't exist."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the file to write"
                },
                "content": {
                    "type": "string",
                    "description": "The content to write to the file"
                }
            },
            "required": ["path", "content"]
        })
    }

    async fn execute(&self, input: &Value) -> Result<ToolOutput> {
        let path = input["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing path parameter"))?;
        let content = input["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing content parameter"))?;

        // TODO: Integrate with Zed's buffer system
        Ok(ToolOutput::Text(format!(
            "TODO: Write {} bytes to: {}",
            content.len(),
            path
        )))
    }
}

/// Edit file tool - makes targeted edits to files
pub struct EditFileTool;

#[async_trait]
impl Tool for EditFileTool {
    fn name(&self) -> &str {
        "edit_file"
    }

    fn description(&self) -> &str {
        "Make targeted edits to a file by replacing specific text patterns."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the file to edit"
                },
                "old_string": {
                    "type": "string",
                    "description": "The exact text to find and replace"
                },
                "new_string": {
                    "type": "string",
                    "description": "The text to replace with"
                }
            },
            "required": ["path", "old_string", "new_string"]
        })
    }

    async fn execute(&self, input: &Value) -> Result<ToolOutput> {
        // TODO: Integrate with Zed's edit operations
        Ok(ToolOutput::Text("TODO: Edit file via Zed".to_string()))
    }
}

/// List files tool - lists files in the workspace
pub struct ListFilesTool;

#[async_trait]
impl Tool for ListFilesTool {
    fn name(&self) -> &str {
        "list_files"
    }

    fn description(&self) -> &str {
        "List files and directories in the workspace."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The directory path to list (defaults to workspace root)"
                },
                "pattern": {
                    "type": "string",
                    "description": "Optional glob pattern to filter files"
                },
                "recursive": {
                    "type": "boolean",
                    "description": "Whether to list recursively"
                }
            }
        })
    }

    async fn execute(&self, input: &Value) -> Result<ToolOutput> {
        // TODO: Integrate with Zed's worktree
        Ok(ToolOutput::Text("TODO: List files via Zed worktree".to_string()))
    }
}

/// Search tool - searches for text/patterns in the workspace
pub struct SearchTool;

#[async_trait]
impl Tool for SearchTool {
    fn name(&self) -> &str {
        "search"
    }

    fn description(&self) -> &str {
        "Search for text or patterns in the workspace using ripgrep."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "The search pattern (regex supported)"
                },
                "path": {
                    "type": "string",
                    "description": "Optional path to limit search scope"
                },
                "case_sensitive": {
                    "type": "boolean",
                    "description": "Whether search is case sensitive"
                },
                "whole_word": {
                    "type": "boolean",
                    "description": "Match whole words only"
                }
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, input: &Value) -> Result<ToolOutput> {
        // TODO: Integrate with Zed's project search
        Ok(ToolOutput::Text("TODO: Search via Zed project search".to_string()))
    }
}

/// LSP diagnostics tool - gets diagnostics from language servers
pub struct DiagnosticsTool;

#[async_trait]
impl Tool for DiagnosticsTool {
    fn name(&self) -> &str {
        "get_diagnostics"
    }

    fn description(&self) -> &str {
        "Get LSP diagnostics (errors, warnings) for a file or the entire workspace."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Optional file path to get diagnostics for"
                },
                "severity": {
                    "type": "string",
                    "enum": ["error", "warning", "info", "hint"],
                    "description": "Filter by severity level"
                }
            }
        })
    }

    async fn execute(&self, input: &Value) -> Result<ToolOutput> {
        // TODO: Integrate with Zed's diagnostic system
        Ok(ToolOutput::Text("TODO: Get diagnostics from Zed LSP".to_string()))
    }
}

/// Terminal tool - executes shell commands
pub struct TerminalTool;

#[async_trait]
impl Tool for TerminalTool {
    fn name(&self) -> &str {
        "terminal"
    }

    fn description(&self) -> &str {
        "Execute a shell command in the integrated terminal."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "The command to execute"
                },
                "cwd": {
                    "type": "string",
                    "description": "Working directory for the command"
                },
                "timeout_ms": {
                    "type": "integer",
                    "description": "Timeout in milliseconds (default: 30000)"
                }
            },
            "required": ["command"]
        })
    }

    async fn execute(&self, input: &Value) -> Result<ToolOutput> {
        // TODO: Integrate with Zed's terminal
        Ok(ToolOutput::Text("TODO: Execute via Zed terminal".to_string()))
    }
}

/// Git tool - git operations
pub struct GitTool;

#[async_trait]
impl Tool for GitTool {
    fn name(&self) -> &str {
        "git"
    }

    fn description(&self) -> &str {
        "Perform git operations like status, diff, commit, etc."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["status", "diff", "log", "blame", "show"],
                    "description": "The git operation to perform"
                },
                "path": {
                    "type": "string",
                    "description": "Optional file path for the operation"
                },
                "args": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Additional arguments"
                }
            },
            "required": ["operation"]
        })
    }

    async fn execute(&self, input: &Value) -> Result<ToolOutput> {
        // TODO: Integrate with Zed's git module
        Ok(ToolOutput::Text("TODO: Git operation via Zed".to_string()))
    }
}

/// Create all default Zed tools
pub fn create_default_tools() -> Vec<Box<dyn Tool>> {
    vec![
        Box::new(ReadFileTool),
        Box::new(WriteFileTool),
        Box::new(EditFileTool),
        Box::new(ListFilesTool),
        Box::new(SearchTool),
        Box::new(DiagnosticsTool),
        Box::new(TerminalTool),
        Box::new(GitTool),
    ]
}
