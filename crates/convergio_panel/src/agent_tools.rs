//! Agent Tools Module
//!
//! Defines and executes tools that Convergio agents can use
//! to interact with the filesystem, run commands, and search code.

use anyhow::{anyhow, Result};
use anthropic::{RequestContent, Tool, ToolResultContent};
use serde_json::{json, Value};
use std::path::Path;
use std::process::Command;

/// Maximum file size to read (1MB)
const MAX_FILE_SIZE: u64 = 1_048_576;

/// Maximum output length for command execution
const MAX_OUTPUT_LENGTH: usize = 65536;

/// Maximum number of files to list
const MAX_LIST_FILES: usize = 500;

/// Get all available tools for agents
pub fn get_agent_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "read_file".to_string(),
            description: "Read the contents of a file at the given path. Returns the file content as text.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "The absolute or relative path to the file to read"
                    }
                },
                "required": ["path"]
            }),
        },
        Tool {
            name: "write_file".to_string(),
            description: "Write content to a file at the given path. Creates the file if it doesn't exist, overwrites if it does.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "The absolute or relative path to the file to write"
                    },
                    "content": {
                        "type": "string",
                        "description": "The content to write to the file"
                    }
                },
                "required": ["path", "content"]
            }),
        },
        Tool {
            name: "list_directory".to_string(),
            description: "List all files and directories in the given path. Returns file names with type indicators (/ for directories).".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "The absolute or relative path to the directory to list"
                    },
                    "recursive": {
                        "type": "boolean",
                        "description": "If true, list files recursively. Default is false."
                    }
                },
                "required": ["path"]
            }),
        },
        Tool {
            name: "run_command".to_string(),
            description: "Execute a shell command and return its output. Use for git operations, build commands, tests, etc.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The shell command to execute"
                    },
                    "working_directory": {
                        "type": "string",
                        "description": "The directory to run the command in. Optional."
                    }
                },
                "required": ["command"]
            }),
        },
        Tool {
            name: "search_files".to_string(),
            description: "Search for a pattern in files using ripgrep. Returns matching lines with file paths and line numbers.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "pattern": {
                        "type": "string",
                        "description": "The regex pattern to search for"
                    },
                    "path": {
                        "type": "string",
                        "description": "The directory to search in. Defaults to current directory."
                    },
                    "file_pattern": {
                        "type": "string",
                        "description": "Glob pattern to filter files (e.g., '*.rs', '*.ts'). Optional."
                    }
                },
                "required": ["pattern"]
            }),
        },
    ]
}

/// Execute a tool and return the result
pub fn execute_tool(
    tool_name: &str,
    tool_input: &Value,
    workspace_root: Option<&Path>,
) -> Result<ToolResultContent> {
    let result = match tool_name {
        "read_file" => execute_read_file(tool_input, workspace_root),
        "write_file" => execute_write_file(tool_input, workspace_root),
        "list_directory" => execute_list_directory(tool_input, workspace_root),
        "run_command" => execute_run_command(tool_input, workspace_root),
        "search_files" => execute_search_files(tool_input, workspace_root),
        _ => Err(anyhow!("Unknown tool: {}", tool_name)),
    };

    match result {
        Ok(output) => Ok(ToolResultContent::Plain(output)),
        Err(e) => Ok(ToolResultContent::Plain(format!("Error: {}", e))),
    }
}

/// Create a tool_result RequestContent
pub fn create_tool_result(
    tool_use_id: &str,
    content: ToolResultContent,
    is_error: bool,
) -> RequestContent {
    RequestContent::ToolResult {
        tool_use_id: tool_use_id.to_string(),
        is_error,
        content,
        cache_control: None,
    }
}

/// Resolve path relative to workspace root
fn resolve_path(path_str: &str, workspace_root: Option<&Path>) -> std::path::PathBuf {
    let path = Path::new(path_str);
    if path.is_absolute() {
        path.to_path_buf()
    } else if let Some(root) = workspace_root {
        root.join(path)
    } else {
        path.to_path_buf()
    }
}

fn execute_read_file(input: &Value, workspace_root: Option<&Path>) -> Result<String> {
    let path_str = input
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Missing 'path' parameter"))?;

    let path = resolve_path(path_str, workspace_root);

    // Check file size before reading
    let metadata = std::fs::metadata(&path)
        .map_err(|e| anyhow!("Cannot access file '{}': {}", path.display(), e))?;

    if metadata.len() > MAX_FILE_SIZE {
        return Err(anyhow!(
            "File too large ({} bytes). Maximum is {} bytes.",
            metadata.len(),
            MAX_FILE_SIZE
        ));
    }

    std::fs::read_to_string(&path)
        .map_err(|e| anyhow!("Failed to read file '{}': {}", path.display(), e))
}

fn execute_write_file(input: &Value, workspace_root: Option<&Path>) -> Result<String> {
    let path_str = input
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Missing 'path' parameter"))?;

    let content = input
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Missing 'content' parameter"))?;

    let path = resolve_path(path_str, workspace_root);

    // Create parent directories if needed
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| anyhow!("Failed to create directories: {}", e))?;
    }

    std::fs::write(&path, content)
        .map_err(|e| anyhow!("Failed to write file '{}': {}", path.display(), e))?;

    Ok(format!("Successfully wrote {} bytes to '{}'", content.len(), path.display()))
}

fn execute_list_directory(input: &Value, workspace_root: Option<&Path>) -> Result<String> {
    let path_str = input
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Missing 'path' parameter"))?;

    let recursive = input
        .get("recursive")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let path = resolve_path(path_str, workspace_root);

    if !path.is_dir() {
        return Err(anyhow!("'{}' is not a directory", path.display()));
    }

    let mut entries = Vec::new();
    collect_entries(&path, &path, recursive, &mut entries, 0)?;

    if entries.is_empty() {
        Ok("Directory is empty".to_string())
    } else {
        Ok(entries.join("\n"))
    }
}

fn collect_entries(
    base: &Path,
    current: &Path,
    recursive: bool,
    entries: &mut Vec<String>,
    depth: usize,
) -> Result<()> {
    if entries.len() >= MAX_LIST_FILES {
        return Ok(());
    }

    let read_dir = std::fs::read_dir(current)
        .map_err(|e| anyhow!("Cannot read directory '{}': {}", current.display(), e))?;

    let mut items: Vec<_> = read_dir.filter_map(|e| e.ok()).collect();
    items.sort_by_key(|e| e.file_name());

    for entry in items {
        if entries.len() >= MAX_LIST_FILES {
            entries.push(format!("... (truncated at {} entries)", MAX_LIST_FILES));
            return Ok(());
        }

        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Skip hidden files at root level
        if depth == 0 && file_name_str.starts_with('.') {
            continue;
        }

        let relative_path = entry
            .path()
            .strip_prefix(base)
            .unwrap_or(&entry.path())
            .to_string_lossy()
            .to_string();

        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

        if is_dir {
            entries.push(format!("{}/", relative_path));
            if recursive {
                collect_entries(base, &entry.path(), true, entries, depth + 1)?;
            }
        } else {
            entries.push(relative_path);
        }
    }

    Ok(())
}

fn execute_run_command(input: &Value, workspace_root: Option<&Path>) -> Result<String> {
    let command_str = input
        .get("command")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Missing 'command' parameter"))?;

    let working_dir = input
        .get("working_directory")
        .and_then(|v| v.as_str())
        .map(|p| resolve_path(p, workspace_root))
        .or_else(|| workspace_root.map(|p| p.to_path_buf()));

    // Use shell to execute command
    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.args(["/C", command_str]);
        c
    } else {
        let mut c = Command::new("sh");
        c.args(["-c", command_str]);
        c
    };

    if let Some(dir) = working_dir {
        cmd.current_dir(dir);
    }

    let output = cmd.output()
        .map_err(|e| anyhow!("Failed to execute command: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let mut result = String::new();

    if !stdout.is_empty() {
        result.push_str(&stdout);
    }

    if !stderr.is_empty() {
        if !result.is_empty() {
            result.push_str("\n--- stderr ---\n");
        }
        result.push_str(&stderr);
    }

    if result.is_empty() {
        result = if output.status.success() {
            "Command completed successfully with no output".to_string()
        } else {
            format!("Command failed with exit code: {:?}", output.status.code())
        };
    }

    // Truncate if too long
    if result.len() > MAX_OUTPUT_LENGTH {
        result.truncate(MAX_OUTPUT_LENGTH);
        result.push_str("\n... (output truncated)");
    }

    if !output.status.success() {
        result = format!("[Exit code: {:?}]\n{}", output.status.code(), result);
    }

    Ok(result)
}

fn execute_search_files(input: &Value, workspace_root: Option<&Path>) -> Result<String> {
    let pattern = input
        .get("pattern")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Missing 'pattern' parameter"))?;

    let search_path = input
        .get("path")
        .and_then(|v| v.as_str())
        .map(|p| resolve_path(p, workspace_root))
        .or_else(|| workspace_root.map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    let file_pattern = input
        .get("file_pattern")
        .and_then(|v| v.as_str());

    // Build ripgrep command
    let mut cmd = Command::new("rg");
    cmd.args(["--line-number", "--no-heading", "--color=never"]);

    if let Some(fp) = file_pattern {
        cmd.args(["--glob", fp]);
    }

    cmd.arg(pattern);
    cmd.arg(&search_path);

    let output = cmd.output()
        .map_err(|e| anyhow!("Failed to run ripgrep: {}. Is ripgrep installed?", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    if stdout.is_empty() {
        if output.status.code() == Some(1) {
            Ok("No matches found".to_string())
        } else if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Search failed: {}", stderr))
        } else {
            Ok("No matches found".to_string())
        }
    } else {
        let mut result = stdout.to_string();
        if result.len() > MAX_OUTPUT_LENGTH {
            result.truncate(MAX_OUTPUT_LENGTH);
            result.push_str("\n... (results truncated)");
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_agent_tools_returns_5_tools() {
        let tools = get_agent_tools();
        assert_eq!(tools.len(), 5);
        assert!(tools.iter().any(|t| t.name == "read_file"));
        assert!(tools.iter().any(|t| t.name == "write_file"));
        assert!(tools.iter().any(|t| t.name == "list_directory"));
        assert!(tools.iter().any(|t| t.name == "run_command"));
        assert!(tools.iter().any(|t| t.name == "search_files"));
    }

    #[test]
    fn test_resolve_path_absolute() {
        let path = resolve_path("/absolute/path", Some(Path::new("/workspace")));
        assert_eq!(path.to_str().unwrap(), "/absolute/path");
    }

    #[test]
    fn test_resolve_path_relative() {
        let path = resolve_path("relative/path", Some(Path::new("/workspace")));
        assert_eq!(path.to_str().unwrap(), "/workspace/relative/path");
    }
}
