use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;

pub const SETTINGS_PATTERNS: MigrationPatterns = &[(
    SETTINGS_CONTEXT_SERVER_PATTERN,
    flatten_context_server_command,
)];

const SETTINGS_CONTEXT_SERVER_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @context-servers)
            value: (object
                (pair
                    key: (string (string_content) @server-name)
                    value: (object
                        (pair
                            key: (string (string_content) @source-key)
                            value: (string (string_content) @source-value)
                        )
                        (pair
                            key: (string (string_content) @command-key)
                            value: (object) @command-object
                        ) @command-pair
                    ) @server-settings
                )
            )
        )
    )
    (#eq? @context-servers "context_servers")
    (#eq? @source-key "source")
    (#eq? @source-value "custom")
    (#eq? @command-key "command")
)"#;

fn flatten_context_server_command(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let command_pair_index = query.capture_index_for_name("command-pair")?;
    let command_pair = mat.nodes_for_capture_index(command_pair_index).next()?;

    let command_object_index = query.capture_index_for_name("command-object")?;
    let command_object = mat.nodes_for_capture_index(command_object_index).next()?;

    let server_settings_index = query.capture_index_for_name("server-settings")?;
    let _server_settings = mat.nodes_for_capture_index(server_settings_index).next()?;

    // Parse the command object to extract path, args, and env
    let mut path_value = None;
    let mut args_value = None;
    let mut env_value = None;

    let mut cursor = command_object.walk();
    for child in command_object.children(&mut cursor) {
        if child.kind() == "pair"
            && let Some(key_node) = child.child_by_field_name("key")
            && let Some(string_content) = key_node.child(1)
        {
            let key = &contents[string_content.byte_range()];
            if let Some(value_node) = child.child_by_field_name("value") {
                let value_range = value_node.byte_range();
                match key {
                    "path" => path_value = Some(&contents[value_range]),
                    "args" => args_value = Some(&contents[value_range]),
                    "env" => env_value = Some(&contents[value_range]),
                    _ => {}
                }
            }
        }
    }

    let path = path_value?;

    // Get the proper indentation from the command pair
    let command_pair_start = command_pair.start_byte();
    let line_start = contents[..command_pair_start]
        .rfind('\n')
        .map(|pos| pos + 1)
        .unwrap_or(0);
    let indent = &contents[line_start..command_pair_start];

    // Build the replacement string
    let mut replacement = format!("\"command\": {}", path);

    // Add args if present - need to reduce indentation
    if let Some(args) = args_value {
        replacement.push_str(",\n");
        replacement.push_str(indent);
        replacement.push_str("\"args\": ");
        let reduced_args = reduce_indentation(args, 4);
        replacement.push_str(&reduced_args);
    }

    // Add env if present - need to reduce indentation
    if let Some(env) = env_value {
        replacement.push_str(",\n");
        replacement.push_str(indent);
        replacement.push_str("\"env\": ");
        replacement.push_str(&reduce_indentation(env, 4));
    }

    let range_to_replace = command_pair.byte_range();
    Some((range_to_replace, replacement))
}

fn reduce_indentation(text: &str, spaces: usize) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut result = String::new();

    for (i, line) in lines.iter().enumerate() {
        if i > 0 {
            result.push('\n');
        }

        // Count leading spaces
        let leading_spaces = line.chars().take_while(|&c| c == ' ').count();

        if leading_spaces >= spaces {
            // Reduce indentation
            result.push_str(&line[spaces..]);
        } else {
            // Keep line as is if it doesn't have enough indentation
            result.push_str(line);
        }
    }

    result
}
