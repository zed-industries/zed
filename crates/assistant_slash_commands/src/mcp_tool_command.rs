use anyhow::{Context as _, Result, anyhow};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandResult,
};
use context_server::{ContextServerId, types::Tool};
use gpui::{App, Entity, Task, WeakEntity, Window};
use language::{BufferSnapshot, CodeLabel, LspAdapterDelegate};
use project::context_server_store::ContextServerStore;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use ui::{IconName, SharedString};
use workspace::Workspace;

use crate::create_label_for_command;

pub struct McpToolSlashCommand {
    store: Entity<ContextServerStore>,
    server_id: ContextServerId,
    tool: Tool,
}

impl McpToolSlashCommand {
    pub fn new(store: Entity<ContextServerStore>, server_id: ContextServerId, tool: Tool) -> Self {
        Self {
            store,
            server_id,
            tool,
        }
    }
}

impl SlashCommand for McpToolSlashCommand {
    fn name(&self) -> String {
        let server_name = clean_server_name(&self.server_id.0);
        format!("{}-{}", server_name, self.tool.name.replace('_', "-"))
    }

    fn label(&self, cx: &App) -> CodeLabel {
        create_label_for_command(&self.name(), &[], cx)
    }

    fn description(&self) -> String {
        match &self.tool.description {
            Some(desc) => desc.clone(),
            None => format!("Run MCP tool '{}' from {}", self.tool.name, self.server_id),
        }
    }

    fn menu_text(&self) -> String {
        let server_name = clean_server_name(&self.server_id.0);
        match &self.tool.description {
            Some(desc) => format!("{} Tool: {}", server_name, desc),
            None => format!("{} Tool: '{}'", server_name, self.tool.name),
        }
    }

    fn requires_argument(&self) -> bool {
        // Check if the tool's input schema has required properties
        if let Some(schema) = self.tool.input_schema.as_object() {
            if let Some(properties) = schema.get("properties").and_then(|p| p.as_object()) {
                if let Some(required) = schema.get("required").and_then(|r| r.as_array()) {
                    return !required.is_empty() && !properties.is_empty();
                }
                return !properties.is_empty();
            }
        }
        false
    }

    fn accepts_arguments(&self) -> bool {
        // Check if the tool has any input schema properties
        if let Some(schema) = self.tool.input_schema.as_object() {
            if let Some(properties) = schema.get("properties").and_then(|p| p.as_object()) {
                return !properties.is_empty();
            }
        }
        false
    }

    fn complete_argument(
        self: Arc<Self>,
        _arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakEntity<Workspace>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        // For now, we don't provide completions for MCP tool arguments
        // This could be enhanced in the future by analyzing the input schema
        Task::ready(Ok(vec![]))
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        _workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<SlashCommandResult> {
        let server_id = self.server_id.clone();
        let tool_name = self.tool.name.clone();
        let store = self.store.clone();

        // Parse arguments into JSON based on the tool's input schema
        let input_args = if arguments.is_empty() {
            serde_json::Value::Object(serde_json::Map::new())
        } else {
            // Try different argument formats
            let args_str = arguments.join(" ");

            // First try JSON format
            if args_str.starts_with('{') && args_str.ends_with('}') {
                match serde_json::from_str(&args_str) {
                    Ok(value) => value,
                    Err(_) => {
                        // If JSON parsing fails, create a simple object with the raw string
                        serde_json::json!({ "input": args_str })
                    }
                }
            } else {
                // Try key:value or key=value format
                let mut parsed_args = serde_json::Map::new();
                let mut has_key_value = false;

                for arg in arguments {
                    if let Some((key, value)) = parse_key_value_argument(arg) {
                        parsed_args.insert(key, value);
                        has_key_value = true;
                    }
                }

                if has_key_value {
                    serde_json::Value::Object(parsed_args)
                } else {
                    // For simple string input, try to map to the first property in the schema
                    if let Some(schema) = self.tool.input_schema.as_object() {
                        if let Some(properties) =
                            schema.get("properties").and_then(|p| p.as_object())
                        {
                            if let Some(first_prop) = properties.keys().next() {
                                serde_json::json!({ first_prop: args_str })
                            } else {
                                serde_json::json!({ "input": args_str })
                            }
                        } else {
                            serde_json::json!({ "input": args_str })
                        }
                    } else {
                        serde_json::json!({ "input": args_str })
                    }
                }
            }
        };

        if let Some(server) = store.read(cx).get_running_server(&server_id) {
            cx.foreground_executor().spawn(async move {
                let protocol = server.client().context("Context server not initialized")?;

                let arguments = if let serde_json::Value::Object(map) = input_args {
                    Some(map.into_iter().collect())
                } else {
                    None
                };

                // Tool execution starting

                let response = protocol
                    .request::<context_server::types::requests::CallTool>(
                        context_server::types::CallToolParams {
                            name: tool_name.clone(),
                            arguments,
                            meta: None,
                        },
                    )
                    .await
                    .with_context(|| {
                        format!(
                            "Failed to execute tool '{}' from server '{}'",
                            tool_name, server_id.0
                        )
                    })?;

                let mut result = String::new();
                for (_i, content) in response.content.iter().enumerate() {
                    match content {
                        context_server::types::ToolResponseContent::Text { text } => {
                            if !result.is_empty() {
                                result.push('\n');
                            }
                            // Try to format JSON as readable text
                            let formatted_text = format_tool_output_text(text);
                            result.push_str(&formatted_text);
                        }
                        context_server::types::ToolResponseContent::Image { .. } => {
                            if !result.is_empty() {
                                result.push('\n');
                            }
                            result.push_str("[Image content not supported in slash commands]");
                        }
                        context_server::types::ToolResponseContent::Audio { .. } => {
                            if !result.is_empty() {
                                result.push('\n');
                            }
                            result.push_str("[Audio content not supported in slash commands]");
                        }
                        context_server::types::ToolResponseContent::Resource { .. } => {
                            if !result.is_empty() {
                                result.push('\n');
                            }
                            result.push_str("[Resource content not supported in slash commands]");
                        }
                    }
                }

                // Always ensure we have visible output
                if result.is_empty() {
                    result = format!(
                        "✓ Tool '{}' from {} executed successfully.\n\nNo output was returned by the tool.",
                        tool_name,
                        clean_server_name(&server_id.0)
                    );
                } else {
                    // Ensure output is properly formatted with context
                    result = format!(
                        "Output from {} tool '{}':\n\n{}",
                        clean_server_name(&server_id.0),
                        tool_name,
                        result
                    );
                }

                let server_name = clean_server_name(&server_id.0);

                let output = SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: 0..(result.len()),
                        icon: IconName::ToolHammer,
                        label: SharedString::from(format!("{} Tool: {}", server_name, tool_name)),
                        metadata: None,
                    }],
                    text: result,
                    run_commands_in_text: false,
                };

                // Output created successfully

                Ok(output.to_event_stream())
            })
        } else {
            Task::ready(Err(anyhow!(
                "Context server '{}' not found or not running",
                server_id.0
            )))
        }
    }
}

/// Parse key:value or key=value format arguments
fn parse_key_value_argument(arg: &str) -> Option<(String, serde_json::Value)> {
    // Try key:value format first
    if let Some(colon_pos) = arg.find(':') {
        let key = arg[..colon_pos].trim();
        let value = arg[colon_pos + 1..].trim();
        if !key.is_empty() {
            return Some((key.to_string(), parse_value_type(value)));
        }
    }

    // Try key=value format
    if let Some(equals_pos) = arg.find('=') {
        let key = arg[..equals_pos].trim();
        let value = arg[equals_pos + 1..].trim();
        if !key.is_empty() {
            return Some((key.to_string(), parse_value_type(value)));
        }
    }

    None
}

/// Parse a string value into the appropriate JSON type
fn parse_value_type(value: &str) -> serde_json::Value {
    let trimmed = value.trim();

    // Remove quotes if present
    let unquoted = if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        &trimmed[1..trimmed.len() - 1]
    } else {
        trimmed
    };

    // Try to parse as number first
    if let Ok(int_val) = unquoted.parse::<i64>() {
        return serde_json::Value::Number(serde_json::Number::from(int_val));
    }

    if let Ok(float_val) = unquoted.parse::<f64>() {
        if let Some(num) = serde_json::Number::from_f64(float_val) {
            return serde_json::Value::Number(num);
        }
    }

    // Try to parse as boolean
    match unquoted.to_lowercase().as_str() {
        "true" => return serde_json::Value::Bool(true),
        "false" => return serde_json::Value::Bool(false),
        "null" => return serde_json::Value::Null,
        _ => {}
    }

    // Default to string
    serde_json::Value::String(unquoted.to_string())
}

/// Format tool output text for better readability in text threads
fn format_tool_output_text(text: &str) -> String {
    // Try to parse as JSON and format it nicely
    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(text) {
        format_json_value(&json_value, 0)
    } else {
        // If not JSON, return as-is
        text.to_string()
    }
}

/// Format JSON value as readable text with proper indentation
fn format_json_value(value: &serde_json::Value, indent_level: usize) -> String {
    let _indent = "  ".repeat(indent_level);
    let next_indent = "  ".repeat(indent_level + 1);

    match value {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Array(arr) => {
            if arr.is_empty() {
                "[]".to_string()
            } else {
                let items: Vec<String> = arr
                    .iter()
                    .map(|item| {
                        format!(
                            "{}- {}",
                            next_indent,
                            format_json_value(item, indent_level + 1)
                        )
                    })
                    .collect();
                format!("\n{}", items.join("\n"))
            }
        }
        serde_json::Value::Object(obj) => {
            if obj.is_empty() {
                "{}".to_string()
            } else {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(key, val)| {
                        let formatted_value = format_json_value(val, indent_level + 1);
                        if formatted_value.starts_with('\n') {
                            format!("{}{}:{}", next_indent, key, formatted_value)
                        } else {
                            format!("{}{}: {}", next_indent, key, formatted_value)
                        }
                    })
                    .collect();
                format!("\n{}", items.join("\n"))
            }
        }
    }
}

/// Clean server name by removing common prefixes and converting to kebab-case
pub fn clean_server_name(server_name: &str) -> String {
    let cleaned = server_name
        .strip_prefix("mcp-server-")
        .or_else(|| server_name.strip_prefix("mcp-"))
        .unwrap_or(server_name);

    cleaned.replace('_', "-")
}

#[cfg(test)]
mod tests {
    use super::*;
    use context_server::types::Tool;

    #[test]
    fn test_mcp_tool_name_formatting() {
        let server_id = ContextServerId("test-server".into());

        let tool = Tool {
            name: "complex_tool_name_with_underscores".to_string(),
            description: None,
            input_schema: serde_json::json!({}),
            annotations: None,
            output_schema: None,
        };

        // Test name formatting without needing complex setup
        let expected_name = "test-server-complex-tool-name-with-underscores";
        let server_name = clean_server_name(&server_id.0);
        let actual_name = format!("{}-{}", server_name, tool.name.replace('_', "-"));
        assert_eq!(actual_name, expected_name);
    }

    #[test]
    fn test_requires_argument_logic() {
        // Test tool with required properties
        let tool_with_required = Tool {
            name: "test_tool".to_string(),
            description: Some("A test tool".to_string()),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The query to process"
                    }
                },
                "required": ["query"]
            }),
            annotations: None,
            output_schema: None,
        };

        // Test tool with no properties
        let tool_no_args = Tool {
            name: "simple_tool".to_string(),
            description: Some("A simple tool with no args".to_string()),
            input_schema: serde_json::json!({}),
            annotations: None,
            output_schema: None,
        };

        // Test the logic directly
        let has_required = if let Some(schema) = tool_with_required.input_schema.as_object() {
            if let Some(properties) = schema.get("properties").and_then(|p| p.as_object()) {
                if let Some(required) = schema.get("required").and_then(|r| r.as_array()) {
                    !required.is_empty() && !properties.is_empty()
                } else {
                    !properties.is_empty()
                }
            } else {
                false
            }
        } else {
            false
        };

        let has_no_args = if let Some(schema) = tool_no_args.input_schema.as_object() {
            if let Some(properties) = schema.get("properties").and_then(|p| p.as_object()) {
                !properties.is_empty()
            } else {
                false
            }
        } else {
            false
        };

        assert!(has_required);
        assert!(!has_no_args);
    }

    #[test]
    fn test_parse_key_value_arguments() {
        // Test key:value format with string
        assert_eq!(
            parse_key_value_argument("title:Bug report"),
            Some((
                "title".to_string(),
                serde_json::Value::String("Bug report".to_string())
            ))
        );

        // Test key=value format with string
        assert_eq!(
            parse_key_value_argument("priority=high"),
            Some((
                "priority".to_string(),
                serde_json::Value::String("high".to_string())
            ))
        );

        // Test with numbers
        assert_eq!(
            parse_key_value_argument("count=42"),
            Some((
                "count".to_string(),
                serde_json::Value::Number(serde_json::Number::from(42))
            ))
        );

        // Test with float
        assert_eq!(
            parse_key_value_argument("price=19.99"),
            Some((
                "price".to_string(),
                serde_json::Value::Number(serde_json::Number::from_f64(19.99).unwrap())
            ))
        );

        // Test with boolean
        assert_eq!(
            parse_key_value_argument("active=true"),
            Some(("active".to_string(), serde_json::Value::Bool(true)))
        );

        // Test with quoted string
        assert_eq!(
            parse_key_value_argument("name=\"John Doe\""),
            Some((
                "name".to_string(),
                serde_json::Value::String("John Doe".to_string())
            ))
        );

        // Test invalid formats
        assert_eq!(parse_key_value_argument("just_text"), None);
        assert_eq!(parse_key_value_argument(":no_key"), None);
        assert_eq!(parse_key_value_argument("=no_key"), None);
        assert_eq!(parse_key_value_argument(""), None);

        // Test edge cases
        assert_eq!(
            parse_key_value_argument("key:"),
            Some(("key".to_string(), serde_json::Value::String("".to_string())))
        );
        assert_eq!(
            parse_key_value_argument("key="),
            Some(("key".to_string(), serde_json::Value::String("".to_string())))
        );
    }

    #[test]
    fn test_parse_value_type() {
        // Test integers
        assert_eq!(
            parse_value_type("42"),
            serde_json::Value::Number(serde_json::Number::from(42))
        );
        assert_eq!(
            parse_value_type("-123"),
            serde_json::Value::Number(serde_json::Number::from(-123))
        );

        // Test floats
        assert_eq!(
            parse_value_type("3.14"),
            serde_json::Value::Number(serde_json::Number::from_f64(3.14).unwrap())
        );
        assert_eq!(
            parse_value_type("-0.5"),
            serde_json::Value::Number(serde_json::Number::from_f64(-0.5).unwrap())
        );

        // Test booleans
        assert_eq!(parse_value_type("true"), serde_json::Value::Bool(true));
        assert_eq!(parse_value_type("false"), serde_json::Value::Bool(false));
        assert_eq!(parse_value_type("TRUE"), serde_json::Value::Bool(true));
        assert_eq!(parse_value_type("False"), serde_json::Value::Bool(false));

        // Test null
        assert_eq!(parse_value_type("null"), serde_json::Value::Null);
        assert_eq!(parse_value_type("NULL"), serde_json::Value::Null);

        // Test quoted strings
        assert_eq!(
            parse_value_type("\"hello\""),
            serde_json::Value::String("hello".to_string())
        );
        assert_eq!(
            parse_value_type("'world'"),
            serde_json::Value::String("world".to_string())
        );

        // Test regular strings
        assert_eq!(
            parse_value_type("hello"),
            serde_json::Value::String("hello".to_string())
        );
        assert_eq!(
            parse_value_type("not_a_number"),
            serde_json::Value::String("not_a_number".to_string())
        );
    }

    #[test]
    fn test_format_json_value() {
        // Test simple JSON object
        let json = serde_json::json!({
            "name": "John Doe",
            "age": 30,
            "active": true
        });
        let formatted = format_json_value(&json, 0);
        assert!(formatted.contains("name: John Doe"));
        assert!(formatted.contains("age: 30"));
        assert!(formatted.contains("active: true"));

        // Test JSON array
        let json = serde_json::json!(["item1", "item2", "item3"]);
        let formatted = format_json_value(&json, 0);
        assert!(formatted.contains("- item1"));
        assert!(formatted.contains("- item2"));
        assert!(formatted.contains("- item3"));

        // Test nested object
        let json = serde_json::json!({
            "user": {
                "name": "John",
                "details": {
                    "email": "john@example.com"
                }
            }
        });
        let formatted = format_json_value(&json, 0);
        assert!(formatted.contains("user:"));
        assert!(formatted.contains("name: John"));
        assert!(formatted.contains("email: john@example.com"));
    }

    #[test]
    fn test_format_tool_output_text() {
        // Test JSON formatting
        let json_text = r#"{"name": "John", "age": 30}"#;
        let formatted = format_tool_output_text(json_text);
        assert!(formatted.contains("name: John"));
        assert!(formatted.contains("age: 30"));

        // Test non-JSON text
        let plain_text = "This is just plain text";
        let formatted = format_tool_output_text(plain_text);
        assert_eq!(formatted, plain_text);
    }

    #[test]
    fn test_clean_server_name() {
        // Test removing mcp-server- prefix
        assert_eq!(clean_server_name("mcp-server-github"), "github");
        assert_eq!(clean_server_name("mcp-server-slack-bot"), "slack-bot");

        // Test removing mcp- prefix
        assert_eq!(clean_server_name("mcp-github"), "github");
        assert_eq!(clean_server_name("mcp-custom"), "custom");

        // Test underscore to hyphen conversion
        assert_eq!(clean_server_name("my_custom_server"), "my-custom-server");
        assert_eq!(clean_server_name("mcp-server-my_app"), "my-app");

        // Test no prefix removal needed
        assert_eq!(clean_server_name("github"), "github");
        assert_eq!(clean_server_name("slack"), "slack");
        assert_eq!(clean_server_name("custom-tool"), "custom-tool");

        // Test edge cases
        assert_eq!(clean_server_name("mcp"), "mcp");
        assert_eq!(clean_server_name("mcp-"), "");
        assert_eq!(clean_server_name("mcp-server-"), "");
    }
}
