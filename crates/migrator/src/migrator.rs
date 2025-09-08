//! ## When to create a migration and why?
//! A migration is necessary when keymap actions or settings are renamed or transformed (e.g., from an array to a string, a string to an array, a boolean to an enum, etc.).
//!
//! This ensures that users with outdated settings are automatically updated to use the corresponding new settings internally.
//! It also provides a quick way to migrate their existing settings to the latest state using button in UI.
//!
//! ## How to create a migration?
//! Migrations use Tree-sitter to query commonly used patterns, such as actions with a string or actions with an array where the second argument is an object, etc.
//! Once queried, *you can filter out the modified items* and write the replacement logic.
//!
//! You *must not* modify previous migrations; always create new ones instead.
//! This is important because if a user is in an intermediate state, they can smoothly transition to the latest state.
//! Modifying existing migrations means they will only work for users upgrading from version x-1 to x, but not from x-2 to x, and so on, where x is the latest version.
//!
//! You only need to write replacement logic for x-1 to x because you can be certain that, internally, every user will be at x-1, regardless of their on disk state.

use anyhow::{Context as _, Result};
use std::{cmp::Reverse, ops::Range, sync::LazyLock};
use streaming_iterator::StreamingIterator;
use tree_sitter::{Query, QueryMatch};

use patterns::SETTINGS_NESTED_KEY_VALUE_PATTERN;

mod migrations;
mod patterns;

fn migrate(text: &str, patterns: MigrationPatterns, query: &Query) -> Result<Option<String>> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_json::LANGUAGE.into())?;
    let syntax_tree = parser
        .parse(text, None)
        .context("failed to parse settings")?;

    let mut cursor = tree_sitter::QueryCursor::new();
    let mut matches = cursor.matches(query, syntax_tree.root_node(), text.as_bytes());

    let mut edits = vec![];
    while let Some(mat) = matches.next() {
        if let Some((_, callback)) = patterns.get(mat.pattern_index) {
            edits.extend(callback(text, mat, query));
        }
    }

    edits.sort_by_key(|(range, _)| (range.start, Reverse(range.end)));
    edits.dedup_by(|(range_b, _), (range_a, _)| {
        range_a.contains(&range_b.start) || range_a.contains(&range_b.end)
    });

    if edits.is_empty() {
        Ok(None)
    } else {
        let mut new_text = text.to_string();
        for (range, replacement) in edits.iter().rev() {
            new_text.replace_range(range.clone(), replacement);
        }
        if new_text == text {
            log::error!(
                "Edits computed for configuration migration do not cause a change: {:?}",
                edits
            );
            Ok(None)
        } else {
            Ok(Some(new_text))
        }
    }
}

fn run_migrations(
    text: &str,
    migrations: &[(MigrationPatterns, &Query)],
) -> Result<Option<String>> {
    let mut current_text = text.to_string();
    let mut result: Option<String> = None;
    for (patterns, query) in migrations.iter() {
        if let Some(migrated_text) = migrate(&current_text, patterns, query)? {
            current_text = migrated_text.clone();
            result = Some(migrated_text);
        }
    }
    Ok(result.filter(|new_text| text != new_text))
}

pub fn migrate_keymap(text: &str) -> Result<Option<String>> {
    let migrations: &[(MigrationPatterns, &Query)] = &[
        (
            migrations::m_2025_01_29::KEYMAP_PATTERNS,
            &KEYMAP_QUERY_2025_01_29,
        ),
        (
            migrations::m_2025_01_30::KEYMAP_PATTERNS,
            &KEYMAP_QUERY_2025_01_30,
        ),
        (
            migrations::m_2025_03_03::KEYMAP_PATTERNS,
            &KEYMAP_QUERY_2025_03_03,
        ),
        (
            migrations::m_2025_03_06::KEYMAP_PATTERNS,
            &KEYMAP_QUERY_2025_03_06,
        ),
        (
            migrations::m_2025_04_15::KEYMAP_PATTERNS,
            &KEYMAP_QUERY_2025_04_15,
        ),
    ];
    run_migrations(text, migrations)
}

pub fn migrate_settings(text: &str) -> Result<Option<String>> {
    let migrations: &[(MigrationPatterns, &Query)] = &[
        (
            migrations::m_2025_01_02::SETTINGS_PATTERNS,
            &SETTINGS_QUERY_2025_01_02,
        ),
        (
            migrations::m_2025_01_29::SETTINGS_PATTERNS,
            &SETTINGS_QUERY_2025_01_29,
        ),
        (
            migrations::m_2025_01_30::SETTINGS_PATTERNS,
            &SETTINGS_QUERY_2025_01_30,
        ),
        (
            migrations::m_2025_03_29::SETTINGS_PATTERNS,
            &SETTINGS_QUERY_2025_03_29,
        ),
        (
            migrations::m_2025_04_15::SETTINGS_PATTERNS,
            &SETTINGS_QUERY_2025_04_15,
        ),
        (
            migrations::m_2025_04_21::SETTINGS_PATTERNS,
            &SETTINGS_QUERY_2025_04_21,
        ),
        (
            migrations::m_2025_04_23::SETTINGS_PATTERNS,
            &SETTINGS_QUERY_2025_04_23,
        ),
        (
            migrations::m_2025_05_05::SETTINGS_PATTERNS,
            &SETTINGS_QUERY_2025_05_05,
        ),
        (
            migrations::m_2025_05_08::SETTINGS_PATTERNS,
            &SETTINGS_QUERY_2025_05_08,
        ),
        (
            migrations::m_2025_05_29::SETTINGS_PATTERNS,
            &SETTINGS_QUERY_2025_05_29,
        ),
        (
            migrations::m_2025_06_16::SETTINGS_PATTERNS,
            &SETTINGS_QUERY_2025_06_16,
        ),
        (
            migrations::m_2025_06_25::SETTINGS_PATTERNS,
            &SETTINGS_QUERY_2025_06_25,
        ),
        (
            migrations::m_2025_06_27::SETTINGS_PATTERNS,
            &SETTINGS_QUERY_2025_06_27,
        ),
        (
            migrations::m_2025_07_08::SETTINGS_PATTERNS,
            &SETTINGS_QUERY_2025_07_08,
        ),
    ];
    run_migrations(text, migrations)
}

pub fn migrate_edit_prediction_provider_settings(text: &str) -> Result<Option<String>> {
    migrate(
        text,
        &[(
            SETTINGS_NESTED_KEY_VALUE_PATTERN,
            migrations::m_2025_01_29::replace_edit_prediction_provider_setting,
        )],
        &EDIT_PREDICTION_SETTINGS_MIGRATION_QUERY,
    )
}

pub type MigrationPatterns = &'static [(
    &'static str,
    fn(&str, &QueryMatch, &Query) -> Option<(Range<usize>, String)>,
)];

macro_rules! define_query {
    ($var_name:ident, $patterns_path:path) => {
        static $var_name: LazyLock<Query> = LazyLock::new(|| {
            Query::new(
                &tree_sitter_json::LANGUAGE.into(),
                &$patterns_path
                    .iter()
                    .map(|pattern| pattern.0)
                    .collect::<String>(),
            )
            .unwrap()
        });
    };
}

// keymap
define_query!(
    KEYMAP_QUERY_2025_01_29,
    migrations::m_2025_01_29::KEYMAP_PATTERNS
);
define_query!(
    KEYMAP_QUERY_2025_01_30,
    migrations::m_2025_01_30::KEYMAP_PATTERNS
);
define_query!(
    KEYMAP_QUERY_2025_03_03,
    migrations::m_2025_03_03::KEYMAP_PATTERNS
);
define_query!(
    KEYMAP_QUERY_2025_03_06,
    migrations::m_2025_03_06::KEYMAP_PATTERNS
);
define_query!(
    KEYMAP_QUERY_2025_04_15,
    migrations::m_2025_04_15::KEYMAP_PATTERNS
);

// settings
define_query!(
    SETTINGS_QUERY_2025_01_02,
    migrations::m_2025_01_02::SETTINGS_PATTERNS
);
define_query!(
    SETTINGS_QUERY_2025_01_29,
    migrations::m_2025_01_29::SETTINGS_PATTERNS
);
define_query!(
    SETTINGS_QUERY_2025_01_30,
    migrations::m_2025_01_30::SETTINGS_PATTERNS
);
define_query!(
    SETTINGS_QUERY_2025_03_29,
    migrations::m_2025_03_29::SETTINGS_PATTERNS
);
define_query!(
    SETTINGS_QUERY_2025_04_15,
    migrations::m_2025_04_15::SETTINGS_PATTERNS
);
define_query!(
    SETTINGS_QUERY_2025_04_21,
    migrations::m_2025_04_21::SETTINGS_PATTERNS
);
define_query!(
    SETTINGS_QUERY_2025_04_23,
    migrations::m_2025_04_23::SETTINGS_PATTERNS
);
define_query!(
    SETTINGS_QUERY_2025_05_05,
    migrations::m_2025_05_05::SETTINGS_PATTERNS
);
define_query!(
    SETTINGS_QUERY_2025_05_08,
    migrations::m_2025_05_08::SETTINGS_PATTERNS
);
define_query!(
    SETTINGS_QUERY_2025_05_29,
    migrations::m_2025_05_29::SETTINGS_PATTERNS
);
define_query!(
    SETTINGS_QUERY_2025_06_16,
    migrations::m_2025_06_16::SETTINGS_PATTERNS
);
define_query!(
    SETTINGS_QUERY_2025_06_25,
    migrations::m_2025_06_25::SETTINGS_PATTERNS
);
define_query!(
    SETTINGS_QUERY_2025_06_27,
    migrations::m_2025_06_27::SETTINGS_PATTERNS
);
define_query!(
    SETTINGS_QUERY_2025_07_08,
    migrations::m_2025_07_08::SETTINGS_PATTERNS
);

// custom query
static EDIT_PREDICTION_SETTINGS_MIGRATION_QUERY: LazyLock<Query> = LazyLock::new(|| {
    Query::new(
        &tree_sitter_json::LANGUAGE.into(),
        SETTINGS_NESTED_KEY_VALUE_PATTERN,
    )
    .unwrap()
});

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_migrate_keymap(input: &str, output: Option<&str>) {
        let migrated = migrate_keymap(input).unwrap();
        pretty_assertions::assert_eq!(migrated.as_deref(), output);
    }

    fn assert_migrate_settings(input: &str, output: Option<&str>) {
        let migrated = migrate_settings(input).unwrap();
        pretty_assertions::assert_eq!(migrated.as_deref(), output);
    }

    fn assert_migrate_settings_with_migrations(
        migrations: &[(MigrationPatterns, &Query)],
        input: &str,
        output: Option<&str>,
    ) {
        let migrated = run_migrations(input, migrations).unwrap();
        pretty_assertions::assert_eq!(migrated.as_deref(), output);
    }

    #[test]
    fn test_replace_array_with_single_string() {
        assert_migrate_keymap(
            r#"
            [
                {
                    "bindings": {
                        "cmd-1": ["workspace::ActivatePaneInDirection", "Up"]
                    }
                }
            ]
            "#,
            Some(
                r#"
            [
                {
                    "bindings": {
                        "cmd-1": "workspace::ActivatePaneUp"
                    }
                }
            ]
            "#,
            ),
        )
    }

    #[test]
    fn test_replace_action_argument_object_with_single_value() {
        assert_migrate_keymap(
            r#"
            [
                {
                    "bindings": {
                        "cmd-1": ["editor::FoldAtLevel", { "level": 1 }]
                    }
                }
            ]
            "#,
            Some(
                r#"
            [
                {
                    "bindings": {
                        "cmd-1": ["editor::FoldAtLevel", 1]
                    }
                }
            ]
            "#,
            ),
        )
    }

    #[test]
    fn test_replace_action_argument_object_with_single_value_2() {
        assert_migrate_keymap(
            r#"
            [
                {
                    "bindings": {
                        "cmd-1": ["vim::PushOperator", { "Object": { "some" : "value" } }]
                    }
                }
            ]
            "#,
            Some(
                r#"
            [
                {
                    "bindings": {
                        "cmd-1": ["vim::PushObject", { "some" : "value" }]
                    }
                }
            ]
            "#,
            ),
        )
    }

    #[test]
    fn test_rename_string_action() {
        assert_migrate_keymap(
            r#"
                [
                    {
                        "bindings": {
                            "cmd-1": "inline_completion::ToggleMenu"
                        }
                    }
                ]
            "#,
            Some(
                r#"
                [
                    {
                        "bindings": {
                            "cmd-1": "edit_prediction::ToggleMenu"
                        }
                    }
                ]
            "#,
            ),
        )
    }

    #[test]
    fn test_rename_context_key() {
        assert_migrate_keymap(
            r#"
                [
                    {
                        "context": "Editor && inline_completion && !showing_completions"
                    }
                ]
            "#,
            Some(
                r#"
                [
                    {
                        "context": "Editor && edit_prediction && !showing_completions"
                    }
                ]
            "#,
            ),
        )
    }

    #[test]
    fn test_incremental_migrations() {
        // Here string transforms to array internally. Then, that array transforms back to string.
        assert_migrate_keymap(
            r#"
                [
                    {
                        "bindings": {
                            "ctrl-q": "editor::GoToHunk", // should remain same
                            "ctrl-w": "editor::GoToPrevHunk", // should rename
                            "ctrl-q": ["editor::GoToHunk", { "center_cursor": true }], // should transform
                            "ctrl-w": ["editor::GoToPreviousHunk", { "center_cursor": true }] // should transform
                        }
                    }
                ]
            "#,
            Some(
                r#"
                [
                    {
                        "bindings": {
                            "ctrl-q": "editor::GoToHunk", // should remain same
                            "ctrl-w": "editor::GoToPreviousHunk", // should rename
                            "ctrl-q": "editor::GoToHunk", // should transform
                            "ctrl-w": "editor::GoToPreviousHunk" // should transform
                        }
                    }
                ]
            "#,
            ),
        )
    }

    #[test]
    fn test_action_argument_snake_case() {
        // First performs transformations, then replacements
        assert_migrate_keymap(
            r#"
            [
                {
                    "bindings": {
                        "cmd-1": ["vim::PushOperator", { "Object": { "around": false } }],
                        "cmd-3": ["pane::CloseActiveItem", { "saveIntent": "saveAll" }],
                        "cmd-2": ["vim::NextWordStart", { "ignorePunctuation": true }],
                        "cmd-4": ["task::Spawn", { "task_name": "a b" }] // should remain as it is
                    }
                }
            ]
            "#,
            Some(
                r#"
            [
                {
                    "bindings": {
                        "cmd-1": ["vim::PushObject", { "around": false }],
                        "cmd-3": ["pane::CloseActiveItem", { "save_intent": "save_all" }],
                        "cmd-2": ["vim::NextWordStart", { "ignore_punctuation": true }],
                        "cmd-4": ["task::Spawn", { "task_name": "a b" }] // should remain as it is
                    }
                }
            ]
            "#,
            ),
        )
    }

    #[test]
    fn test_replace_setting_name() {
        assert_migrate_settings(
            r#"
                {
                    "show_inline_completions_in_menu": true,
                    "show_inline_completions": true,
                    "inline_completions_disabled_in": ["string"],
                    "inline_completions": { "some" : "value" }
                }
            "#,
            Some(
                r#"
                {
                    "show_edit_predictions_in_menu": true,
                    "show_edit_predictions": true,
                    "edit_predictions_disabled_in": ["string"],
                    "edit_predictions": { "some" : "value" }
                }
            "#,
            ),
        )
    }

    #[test]
    fn test_nested_string_replace_for_settings() {
        assert_migrate_settings(
            r#"
                {
                    "features": {
                        "inline_completion_provider": "zed"
                    },
                }
            "#,
            Some(
                r#"
                {
                    "features": {
                        "edit_prediction_provider": "zed"
                    },
                }
            "#,
            ),
        )
    }

    #[test]
    fn test_replace_settings_in_languages() {
        assert_migrate_settings(
            r#"
                {
                    "languages": {
                        "Astro": {
                            "show_inline_completions": true
                        }
                    }
                }
            "#,
            Some(
                r#"
                {
                    "languages": {
                        "Astro": {
                            "show_edit_predictions": true
                        }
                    }
                }
            "#,
            ),
        )
    }

    #[test]
    fn test_replace_settings_value() {
        assert_migrate_settings(
            r#"
                {
                    "scrollbar": {
                        "diagnostics": true
                    },
                    "chat_panel": {
                        "button": true
                    }
                }
            "#,
            Some(
                r#"
                {
                    "scrollbar": {
                        "diagnostics": "all"
                    },
                    "chat_panel": {
                        "button": "always"
                    }
                }
            "#,
            ),
        )
    }

    #[test]
    fn test_replace_settings_name_and_value() {
        assert_migrate_settings(
            r#"
                {
                    "tabs": {
                        "always_show_close_button": true
                    }
                }
            "#,
            Some(
                r#"
                {
                    "tabs": {
                        "show_close_button": "always"
                    }
                }
            "#,
            ),
        )
    }

    #[test]
    fn test_replace_bash_with_terminal_in_profiles() {
        assert_migrate_settings(
            r#"
                {
                    "assistant": {
                        "profiles": {
                            "custom": {
                                "name": "Custom",
                                "tools": {
                                    "bash": true,
                                    "diagnostics": true
                                }
                            }
                        }
                    }
                }
            "#,
            Some(
                r#"
                {
                    "agent": {
                        "profiles": {
                            "custom": {
                                "name": "Custom",
                                "tools": {
                                    "terminal": true,
                                    "diagnostics": true
                                }
                            }
                        }
                    }
                }
            "#,
            ),
        )
    }

    #[test]
    fn test_replace_bash_false_with_terminal_in_profiles() {
        assert_migrate_settings(
            r#"
                {
                    "assistant": {
                        "profiles": {
                            "custom": {
                                "name": "Custom",
                                "tools": {
                                    "bash": false,
                                    "diagnostics": true
                                }
                            }
                        }
                    }
                }
            "#,
            Some(
                r#"
                {
                    "agent": {
                        "profiles": {
                            "custom": {
                                "name": "Custom",
                                "tools": {
                                    "terminal": false,
                                    "diagnostics": true
                                }
                            }
                        }
                    }
                }
            "#,
            ),
        )
    }

    #[test]
    fn test_no_bash_in_profiles() {
        assert_migrate_settings(
            r#"
                {
                    "assistant": {
                        "profiles": {
                            "custom": {
                                "name": "Custom",
                                "tools": {
                                    "diagnostics": true,
                                    "find_path": true,
                                    "read_file": true
                                }
                            }
                        }
                    }
                }
            "#,
            Some(
                r#"
                {
                    "agent": {
                        "profiles": {
                            "custom": {
                                "name": "Custom",
                                "tools": {
                                    "diagnostics": true,
                                    "find_path": true,
                                    "read_file": true
                                }
                            }
                        }
                    }
                }
            "#,
            ),
        )
    }

    #[test]
    fn test_rename_path_search_to_find_path() {
        assert_migrate_settings(
            r#"
                {
                    "assistant": {
                        "profiles": {
                            "default": {
                                "tools": {
                                    "path_search": true,
                                    "read_file": true
                                }
                            }
                        }
                    }
                }
            "#,
            Some(
                r#"
                {
                    "agent": {
                        "profiles": {
                            "default": {
                                "tools": {
                                    "find_path": true,
                                    "read_file": true
                                }
                            }
                        }
                    }
                }
            "#,
            ),
        );
    }

    #[test]
    fn test_rename_assistant() {
        assert_migrate_settings(
            r#"{
                "assistant": {
                    "foo": "bar"
                },
                "edit_predictions": {
                    "enabled_in_assistant": false,
                }
            }"#,
            Some(
                r#"{
                "agent": {
                    "foo": "bar"
                },
                "edit_predictions": {
                    "enabled_in_text_threads": false,
                }
            }"#,
            ),
        );
    }

    #[test]
    fn test_comment_duplicated_agent() {
        assert_migrate_settings(
            r#"{
                "agent": {
                    "name": "assistant-1",
                "model": "gpt-4", // weird formatting
                    "utf8": "привіт"
                },
                "something": "else",
                "agent": {
                    "name": "assistant-2",
                    "model": "gemini-pro"
                }
            }
        "#,
            Some(
                r#"{
                /* Duplicated key auto-commented: "agent": {
                    "name": "assistant-1",
                "model": "gpt-4", // weird formatting
                    "utf8": "привіт"
                }, */
                "something": "else",
                "agent": {
                    "name": "assistant-2",
                    "model": "gemini-pro"
                }
            }
        "#,
            ),
        );
    }

    #[test]
    fn test_preferred_completion_mode_migration() {
        assert_migrate_settings(
            r#"{
                "agent": {
                    "preferred_completion_mode": "max",
                    "enabled": true
                }
            }"#,
            Some(
                r#"{
                "agent": {
                    "preferred_completion_mode": "burn",
                    "enabled": true
                }
            }"#,
            ),
        );

        assert_migrate_settings(
            r#"{
                "agent": {
                    "preferred_completion_mode": "normal",
                    "enabled": true
                }
            }"#,
            None,
        );

        assert_migrate_settings(
            r#"{
                "agent": {
                    "preferred_completion_mode": "burn",
                    "enabled": true
                }
            }"#,
            None,
        );

        assert_migrate_settings(
            r#"{
                "other_section": {
                    "preferred_completion_mode": "max"
                },
                "agent": {
                    "preferred_completion_mode": "max"
                }
            }"#,
            Some(
                r#"{
                "other_section": {
                    "preferred_completion_mode": "max"
                },
                "agent": {
                    "preferred_completion_mode": "burn"
                }
            }"#,
            ),
        );
    }

    #[test]
    fn test_mcp_settings_migration() {
        assert_migrate_settings_with_migrations(
            &[(
                migrations::m_2025_06_16::SETTINGS_PATTERNS,
                &SETTINGS_QUERY_2025_06_16,
            )],
            r#"{
    "context_servers": {
        "empty_server": {},
        "extension_server": {
            "settings": {
                "foo": "bar"
            }
        },
        "custom_server": {
            "command": {
                "path": "foo",
                "args": ["bar"],
                "env": {
                    "FOO": "BAR"
                }
            }
        },
        "invalid_server": {
            "command": {
                "path": "foo",
                "args": ["bar"],
                "env": {
                    "FOO": "BAR"
                }
            },
            "settings": {
                "foo": "bar"
            }
        },
        "empty_server2": {},
        "extension_server2": {
            "foo": "bar",
            "settings": {
                "foo": "bar"
            },
            "bar": "foo"
        },
        "custom_server2": {
            "foo": "bar",
            "command": {
                "path": "foo",
                "args": ["bar"],
                "env": {
                    "FOO": "BAR"
                }
            },
            "bar": "foo"
        },
        "invalid_server2": {
            "foo": "bar",
            "command": {
                "path": "foo",
                "args": ["bar"],
                "env": {
                    "FOO": "BAR"
                }
            },
            "bar": "foo",
            "settings": {
                "foo": "bar"
            }
        }
    }
}"#,
            Some(
                r#"{
    "context_servers": {
        "empty_server": {
            "source": "extension",
            "settings": {}
        },
        "extension_server": {
            "source": "extension",
            "settings": {
                "foo": "bar"
            }
        },
        "custom_server": {
            "source": "custom",
            "command": {
                "path": "foo",
                "args": ["bar"],
                "env": {
                    "FOO": "BAR"
                }
            }
        },
        "invalid_server": {
            "source": "custom",
            "command": {
                "path": "foo",
                "args": ["bar"],
                "env": {
                    "FOO": "BAR"
                }
            },
            "settings": {
                "foo": "bar"
            }
        },
        "empty_server2": {
            "source": "extension",
            "settings": {}
        },
        "extension_server2": {
            "source": "extension",
            "foo": "bar",
            "settings": {
                "foo": "bar"
            },
            "bar": "foo"
        },
        "custom_server2": {
            "source": "custom",
            "foo": "bar",
            "command": {
                "path": "foo",
                "args": ["bar"],
                "env": {
                    "FOO": "BAR"
                }
            },
            "bar": "foo"
        },
        "invalid_server2": {
            "source": "custom",
            "foo": "bar",
            "command": {
                "path": "foo",
                "args": ["bar"],
                "env": {
                    "FOO": "BAR"
                }
            },
            "bar": "foo",
            "settings": {
                "foo": "bar"
            }
        }
    }
}"#,
            ),
        );
    }

    #[test]
    fn test_mcp_settings_migration_doesnt_change_valid_settings() {
        let settings = r#"{
    "context_servers": {
        "empty_server": {
            "source": "extension",
            "settings": {}
        },
        "extension_server": {
            "source": "extension",
            "settings": {
                "foo": "bar"
            }
        },
        "custom_server": {
            "source": "custom",
            "command": {
                "path": "foo",
                "args": ["bar"],
                "env": {
                    "FOO": "BAR"
                }
            }
        },
        "invalid_server": {
            "source": "custom",
            "command": {
                "path": "foo",
                "args": ["bar"],
                "env": {
                    "FOO": "BAR"
                }
            },
            "settings": {
                "foo": "bar"
            }
        }
    }
}"#;
        assert_migrate_settings_with_migrations(
            &[(
                migrations::m_2025_06_16::SETTINGS_PATTERNS,
                &SETTINGS_QUERY_2025_06_16,
            )],
            settings,
            None,
        );
    }

    #[test]
    fn test_remove_version_fields() {
        assert_migrate_settings(
            r#"{
    "language_models": {
        "anthropic": {
            "version": "1",
            "api_url": "https://api.anthropic.com"
        },
        "openai": {
            "version": "1",
            "api_url": "https://api.openai.com/v1"
        }
    },
    "agent": {
        "version": "2",
        "enabled": true,
        "preferred_completion_mode": "normal",
        "button": true,
        "dock": "right",
        "default_width": 640,
        "default_height": 320,
        "default_model": {
            "provider": "zed.dev",
            "model": "claude-sonnet-4"
        }
    }
}"#,
            Some(
                r#"{
    "language_models": {
        "anthropic": {
            "api_url": "https://api.anthropic.com"
        },
        "openai": {
            "api_url": "https://api.openai.com/v1"
        }
    },
    "agent": {
        "enabled": true,
        "preferred_completion_mode": "normal",
        "button": true,
        "dock": "right",
        "default_width": 640,
        "default_height": 320,
        "default_model": {
            "provider": "zed.dev",
            "model": "claude-sonnet-4"
        }
    }
}"#,
            ),
        );

        // Test that version fields in other contexts are not removed
        assert_migrate_settings(
            r#"{
    "language_models": {
        "other_provider": {
            "version": "1",
            "api_url": "https://api.example.com"
        }
    },
    "other_section": {
        "version": "1"
    }
}"#,
            None,
        );
    }

    #[test]
    fn test_flatten_context_server_command() {
        assert_migrate_settings(
            r#"{
    "context_servers": {
        "some-mcp-server": {
            "source": "custom",
            "command": {
                "path": "npx",
                "args": [
                    "-y",
                    "@supabase/mcp-server-supabase@latest",
                    "--read-only",
                    "--project-ref=<project-ref>"
                ],
                "env": {
                    "SUPABASE_ACCESS_TOKEN": "<personal-access-token>"
                }
            }
        }
    }
}"#,
            Some(
                r#"{
    "context_servers": {
        "some-mcp-server": {
            "source": "custom",
            "command": "npx",
            "args": [
                "-y",
                "@supabase/mcp-server-supabase@latest",
                "--read-only",
                "--project-ref=<project-ref>"
            ],
            "env": {
                "SUPABASE_ACCESS_TOKEN": "<personal-access-token>"
            }
        }
    }
}"#,
            ),
        );

        // Test with additional keys in server object
        assert_migrate_settings(
            r#"{
    "context_servers": {
        "server-with-extras": {
            "source": "custom",
            "command": {
                "path": "/usr/bin/node",
                "args": ["server.js"]
            },
            "settings": {}
        }
    }
}"#,
            Some(
                r#"{
    "context_servers": {
        "server-with-extras": {
            "source": "custom",
            "command": "/usr/bin/node",
            "args": ["server.js"],
            "settings": {}
        }
    }
}"#,
            ),
        );

        // Test command without args or env
        assert_migrate_settings(
            r#"{
    "context_servers": {
        "simple-server": {
            "source": "custom",
            "command": {
                "path": "simple-mcp-server"
            }
        }
    }
}"#,
            Some(
                r#"{
    "context_servers": {
        "simple-server": {
            "source": "custom",
            "command": "simple-mcp-server"
        }
    }
}"#,
            ),
        );
    }
}
