//! Extracts structured observations from completed thread messages.
//!
//! Walks the message list looking for:
//! - Terminal commands (from tool_call content / tool_result content)
//! - File edits (edit_file tool calls)
//! - Error messages
//! - Tool names used

use std::sync::Arc;

use crate::curator::Observation;
use crate::Message;

pub struct PatternExtractor;

impl PatternExtractor {
    /// Extract a single observation from a completed thread's messages.
    /// Returns `None` if the thread had no meaningful agent activity.
    pub fn extract(&self, messages: &[Arc<Message>]) -> Option<Observation> {
        let mut commands: Vec<String> = Vec::new();
        let mut edits: Vec<String> = Vec::new();
        let mut tools: Vec<String> = Vec::new();
        let mut error_fragment: Option<String> = None;

        for msg in messages {
            match msg.as_ref() {
                Message::Agent(agent_msg) => {
                    for block in &agent_msg.content {
                        match block {
                            crate::AgentMessageContent::ToolCall(tc) => {
                                let name = &tc.name;
                                tools.push(name.clone());

                                match name.as_str() {
                                    "terminal" | "terminal_sandbox" | "sandboxed_terminal" => {
                                        // Extract the command argument if present
                                        if let Some(args) = tc.arguments.as_object() {
                                            if let Some(cmd) = args.get("command").and_then(|v| v.as_str()) {
                                                let summary = summarize_command(cmd);
                                                if !commands.contains(&summary) {
                                                    commands.push(summary);
                                                }
                                            }
                                        }
                                    }
                                    "edit_file" | "write_file" => {
                                        if let Some(args) = tc.arguments.as_object() {
                                            if let Some(path) = args.get("path").and_then(|v| v.as_str()) {
                                                if let Some(ext) = std::path::Path::new(path)
                                                    .extension()
                                                    .and_then(|e| e.to_str())
                                                {
                                                    if !edits.contains(&ext.to_string()) {
                                                        edits.push(ext.to_string());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            crate::AgentMessageContent::ToolResult(tr) => {
                                // Look for error messages in tool results
                                if let Some(content) = &tr.content {
                                    let text = match content {
                                        crate::ToolResultContent::Text(t) => t.clone(),
                                        _ => String::new(),
                                    };
                                    if let Some(err) = extract_error(&text) {
                                        if error_fragment.is_none() {
                                            error_fragment = Some(err);
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if tools.is_empty() {
            return None; // No agent activity to learn from
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Some(Observation {
            command_signature: commands.join(","),
            edit_signature: edits.join(","),
            tool_signature: tools.join(","),
            error_fragment,
            observed_at: now,
        })
    }
}

/// Shorten a command to its essence for pattern matching.
/// "cargo test --lib -- --nocapture" → "cargo test"
/// "npm run build" → "npm run"
fn summarize_command(cmd: &str) -> String {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.len() >= 2 {
        format!("{} {}", parts[0], parts[1])
    } else {
        parts.first().copied().unwrap_or("").to_string()
    }
}

/// Extract the first error-like line from text.
fn extract_error(text: &str) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let lower = trimmed.to_lowercase();
        if lower.starts_with("error")
            || lower.starts_with("failed")
            || lower.starts_with("fatal")
            || lower.starts_with("panic")
            || lower.starts_with("thread '")
        {
            let fragment = if trimmed.len() > 120 {
                format!("{}...", &trimmed[..120])
            } else {
                trimmed.to_string()
            };
            return Some(fragment);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summarize_command() {
        assert_eq!(summarize_command("cargo test"), "cargo test");
        assert_eq!(summarize_command("cargo test --lib -- --nocapture"), "cargo test");
        assert_eq!(summarize_command("npm run build --prod"), "npm run");
        assert_eq!(summarize_command("ls"), "ls");
    }

    #[test]
    fn test_extract_error() {
        let text = "Compiling foo.rs\nerror[E0308]: mismatched types\n --> src/main.rs:10:5";
        let result = extract_error(text);
        assert!(result.is_some());
        assert!(result.unwrap().contains("error[E0308]"));

        assert!(extract_error("Everything worked fine").is_none());
    }

    #[test]
    fn test_extract_fatal() {
        let result = extract_error("fatal: not a git repository");
        assert!(result.is_some());
        assert!(result.unwrap().contains("fatal:"));
    }
}
