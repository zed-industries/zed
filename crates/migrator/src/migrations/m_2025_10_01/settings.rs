use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;

const FORMATTER_PATTERN: &str = r#"
    (document
        (object
            (pair
                key: (string (string_content) @formatter)
                value: (array
                    (object
                            (pair
                                key: (string (string_content) @code-actions-key)
                                value: (object
                                    (pair) @code-action
                                )
                            )
                        ) @code-actions-obj
                    )
                )
            )
        )
        (#eq? @formatter "formatter")
        (#eq? @code-actions-key "code_actions")
"#;

pub const SETTINGS_PATTERNS: MigrationPatterns =
    &[(FORMATTER_PATTERN, migrate_code_action_formatters)];

pub fn migrate_code_action_formatters(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    dbg!(query.capture_names());
    let code_actions_obj_ix = query.capture_index_for_name("code-actions-obj")?;
    let code_actions_obj_range = dbg!(
        mat.nodes_for_capture_index(code_actions_obj_ix)
            .next()?
            .byte_range()
    );

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

    let mut code_actions_str = String::new();
    for code_action in code_actions {
        code_actions_str.push_str(&format!(r#"{{ "code_action": "{}" }},\n"#, code_action));
    }
    code_actions_str.pop();
    code_actions_str.pop();
    code_actions_str.pop();

    Some((code_actions_obj_range, code_actions_str))
}
