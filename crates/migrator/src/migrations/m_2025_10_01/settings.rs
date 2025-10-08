use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;

pub const SETTINGS_PATTERNS: MigrationPatterns =
    &[(FORMATTER_PATTERN, migrate_code_action_formatters)];

const FORMATTER_PATTERN: &str = r#"
        (object
            (pair
                key: (string (string_content) @formatter) (#any-of? @formatter "formatter" "format_on_save")
                value: [
                    (array
                        (object
                            (pair
                                key: (string (string_content) @code-actions-key) (#eq? @code-actions-key "code_actions")
                                value: (object
                                    ((pair) @code-action ","?)*
                                )
                            )
                        ) @code-actions-obj
                    ) @formatter-array
                    (object
                        (pair
                            key: (string (string_content) @code-actions-key) (#eq? @code-actions-key "code_actions")
                            value: (object
                                ((pair) @code-action ","?)*
                            )
                        )
                    ) @code-actions-obj
                ]
            )
        )
"#;

pub fn migrate_code_action_formatters(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let code_actions_obj_ix = query.capture_index_for_name("code-actions-obj")?;
    let code_actions_obj_node = mat.nodes_for_capture_index(code_actions_obj_ix).next()?;

    let mut code_actions = vec![];

    let code_actions_ix = query.capture_index_for_name("code-action")?;
    for code_action_node in mat.nodes_for_capture_index(code_actions_ix) {
        let Some(enabled) = code_action_node
            .child_by_field_name("value")
            .map(|n| n.kind() != "false")
        else {
            continue;
        };
        if !enabled {
            continue;
        }
        let Some(name) = code_action_node
            .child_by_field_name("key")
            .and_then(|n| n.child(1))
            .map(|n| &contents[n.byte_range()])
        else {
            continue;
        };
        code_actions.push(name);
    }

    let indent = query
        .capture_index_for_name("formatter")
        .and_then(|ix| mat.nodes_for_capture_index(ix).next())
        .map(|node| node.start_position().column + 1)
        .unwrap_or(2);

    let mut code_actions_str = code_actions
        .into_iter()
        .map(|code_action| format!(r#"{{ "code_action": "{}" }}"#, code_action))
        .collect::<Vec<_>>()
        .join(&format!(",\n{}", " ".repeat(indent)));
    let is_array = query
        .capture_index_for_name("formatter-array")
        .map(|ix| mat.nodes_for_capture_index(ix).count() > 0)
        .unwrap_or(false);
    if !is_array {
        code_actions_str.insert_str(0, &" ".repeat(indent));
        code_actions_str.insert_str(0, "[\n");
        code_actions_str.push('\n');
        code_actions_str.push_str(&" ".repeat(indent.saturating_sub(2)));
        code_actions_str.push_str("]");
    }
    let mut replace_range = code_actions_obj_node.byte_range();
    if is_array && code_actions_str.is_empty() {
        let mut cursor = code_actions_obj_node.parent().unwrap().walk();
        cursor.goto_first_child();
        while cursor.node().id() != code_actions_obj_node.id() && cursor.goto_next_sibling() {}
        while cursor.goto_next_sibling()
            && (cursor.node().is_extra()
                || cursor.node().is_missing()
                || cursor.node().kind() == "comment")
        {}
        if cursor.node().kind() == "," {
            // found comma, delete up to next node
            while cursor.goto_next_sibling()
                && (cursor.node().is_extra() || cursor.node().is_missing())
            {}
            replace_range.end = cursor.node().range().start_byte;
        }
    }
    Some((replace_range, code_actions_str))
}
