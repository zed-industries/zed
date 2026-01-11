use agent_settings::{AgentSettings, ToolPermissions, ToolRules};
use settings::ToolPermissionMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolPermissionDecision {
    Allow,
    Deny(String),
    Confirm,
}

/// Determines the permission decision for a tool invocation based on configured rules.
///
/// # Precedence Order (highest to lowest)
///
/// 1. **`always_deny`** - If any deny pattern matches, the tool call is blocked immediately.
///    This takes precedence over all other rules for security.
/// 2. **`always_confirm`** - If any confirm pattern matches (and no deny matched),
///    the user is prompted for confirmation regardless of other settings.
/// 3. **`always_allow`** - If any allow pattern matches (and no deny/confirm matched),
///    the tool call proceeds without prompting.
/// 4. **`default_mode`** - If no patterns match, falls back to the tool's default mode.
/// 5. **`always_allow_tool_actions`** - Global setting used as fallback when no tool-specific
///    rules are configured, or when `default_mode` is `Confirm`.
///
/// # Pattern Matching Tips
///
/// Patterns are matched as regular expressions against the tool input (e.g., the command
/// string for the terminal tool). Some tips for writing effective patterns:
///
/// - Use word boundaries (`\b`) to avoid partial matches. For example, pattern `rm` will
///   match "storm" and "arms", but `\brm\b` will only match the standalone word "rm".
///   This is important for security rules where you want to block specific commands
///   without accidentally blocking unrelated commands that happen to contain the same
///   substring.
/// - Patterns are case-insensitive by default. Set `case_sensitive: true` for exact matching.
/// - Use `^` and `$` anchors to match the start/end of the input.
pub fn decide_permission(
    tool_name: &str,
    input: &str,
    permissions: &ToolPermissions,
    always_allow_tool_actions: bool,
) -> ToolPermissionDecision {
    let rules = permissions.tools.get(tool_name);

    let rules = match rules {
        Some(rules) => rules,
        None => {
            return if always_allow_tool_actions {
                ToolPermissionDecision::Allow
            } else {
                ToolPermissionDecision::Confirm
            };
        }
    };

    // Check for invalid regex patterns before evaluating rules.
    // If any patterns failed to compile, block the tool call entirely.
    if let Some(error) = check_invalid_patterns(tool_name, rules) {
        return ToolPermissionDecision::Deny(error);
    }

    if rules.always_deny.iter().any(|r| r.is_match(input)) {
        return ToolPermissionDecision::Deny(format!(
            "Command blocked by security rule for {} tool",
            tool_name
        ));
    }

    if rules.always_confirm.iter().any(|r| r.is_match(input)) {
        return ToolPermissionDecision::Confirm;
    }

    if rules.always_allow.iter().any(|r| r.is_match(input)) {
        return ToolPermissionDecision::Allow;
    }

    match rules.default_mode {
        ToolPermissionMode::Deny => {
            ToolPermissionDecision::Deny(format!("{} tool is disabled", tool_name))
        }
        ToolPermissionMode::Allow => ToolPermissionDecision::Allow,
        ToolPermissionMode::Confirm => {
            if always_allow_tool_actions {
                ToolPermissionDecision::Allow
            } else {
                ToolPermissionDecision::Confirm
            }
        }
    }
}

/// Checks if the tool rules contain any invalid regex patterns.
/// Returns an error message if invalid patterns are found.
fn check_invalid_patterns(tool_name: &str, rules: &ToolRules) -> Option<String> {
    if rules.invalid_patterns.is_empty() {
        return None;
    }

    let count = rules.invalid_patterns.len();
    let pattern_word = if count == 1 { "pattern" } else { "patterns" };

    Some(format!(
        "The {} tool cannot run because {} regex {} failed to compile. \
         Please fix the invalid patterns in your tool_permissions settings.",
        tool_name, count, pattern_word
    ))
}

/// Convenience wrapper that extracts permission settings from `AgentSettings`.
///
/// This is the primary entry point for tools to check permissions. It extracts
/// `tool_permissions` and `always_allow_tool_actions` from the settings and
/// delegates to [`decide_permission`].
pub fn decide_permission_from_settings(
    tool_name: &str,
    input: &str,
    settings: &AgentSettings,
) -> ToolPermissionDecision {
    decide_permission(
        tool_name,
        input,
        &settings.tool_permissions,
        settings.always_allow_tool_actions,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use agent_settings::{CompiledRegex, InvalidRegexPattern, ToolRules};
    use std::sync::Arc;

    fn empty_permissions() -> ToolPermissions {
        ToolPermissions {
            tools: collections::HashMap::default(),
        }
    }

    fn terminal_rules_with_deny(patterns: &[&str]) -> ToolPermissions {
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Confirm,
                always_allow: vec![],
                always_deny: patterns
                    .iter()
                    .filter_map(|p| CompiledRegex::new(p, false))
                    .collect(),
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        ToolPermissions { tools }
    }

    fn terminal_rules_with_allow(patterns: &[&str]) -> ToolPermissions {
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Confirm,
                always_allow: patterns
                    .iter()
                    .filter_map(|p| CompiledRegex::new(p, false))
                    .collect(),
                always_deny: vec![],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        ToolPermissions { tools }
    }

    #[test]
    fn test_deny_takes_precedence_over_allow() {
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Allow,
                always_allow: vec![CompiledRegex::new("dangerous", false).unwrap()],
                always_deny: vec![CompiledRegex::new("dangerous", false).unwrap()],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        let permissions = ToolPermissions { tools };

        let decision = decide_permission("terminal", "run dangerous command", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));
    }

    #[test]
    fn test_deny_takes_precedence_over_confirm() {
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Allow,
                always_allow: vec![],
                always_deny: vec![CompiledRegex::new("dangerous", false).unwrap()],
                always_confirm: vec![CompiledRegex::new("dangerous", false).unwrap()],
                invalid_patterns: vec![],
            },
        );
        let permissions = ToolPermissions { tools };

        let decision = decide_permission("terminal", "run dangerous command", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));
    }

    #[test]
    fn test_confirm_takes_precedence_over_allow() {
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Allow,
                always_allow: vec![CompiledRegex::new("risky", false).unwrap()],
                always_deny: vec![],
                always_confirm: vec![CompiledRegex::new("risky", false).unwrap()],
                invalid_patterns: vec![],
            },
        );
        let permissions = ToolPermissions { tools };

        let decision = decide_permission("terminal", "do risky thing", &permissions, true);
        assert_eq!(decision, ToolPermissionDecision::Confirm);
    }

    #[test]
    fn test_no_tool_rules_uses_global_setting() {
        let permissions = empty_permissions();

        let decision = decide_permission("terminal", "any command", &permissions, false);
        assert_eq!(decision, ToolPermissionDecision::Confirm);

        let decision = decide_permission("terminal", "any command", &permissions, true);
        assert_eq!(decision, ToolPermissionDecision::Allow);
    }

    #[test]
    fn test_default_mode_fallthrough() {
        // default_mode: Allow - should allow regardless of global setting
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Allow,
                always_allow: vec![],
                always_deny: vec![],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        let permissions = ToolPermissions { tools };
        let decision = decide_permission("terminal", "any command", &permissions, false);
        assert_eq!(decision, ToolPermissionDecision::Allow);

        // default_mode: Deny - should deny regardless of global setting
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Deny,
                always_allow: vec![],
                always_deny: vec![],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        let permissions = ToolPermissions { tools };
        let decision = decide_permission("terminal", "any command", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));

        // default_mode: Confirm - respects global always_allow_tool_actions
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Confirm,
                always_allow: vec![],
                always_deny: vec![],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        let permissions = ToolPermissions { tools };
        let decision = decide_permission("terminal", "any command", &permissions, false);
        assert_eq!(decision, ToolPermissionDecision::Confirm);
        let decision = decide_permission("terminal", "any command", &permissions, true);
        assert_eq!(decision, ToolPermissionDecision::Allow);
    }

    #[test]
    fn test_empty_input() {
        let permissions = terminal_rules_with_deny(&["rm"]);

        // Empty input doesn't match the deny pattern, so falls through to default_mode (Confirm)
        let decision = decide_permission("terminal", "", &permissions, false);
        assert_eq!(decision, ToolPermissionDecision::Confirm);

        // With always_allow_tool_actions=true and default_mode=Confirm, it returns Allow
        let decision = decide_permission("terminal", "", &permissions, true);
        assert_eq!(decision, ToolPermissionDecision::Allow);
    }

    #[test]
    fn test_multiple_patterns_any_match() {
        // Multiple deny patterns - any match should deny
        let permissions = terminal_rules_with_deny(&["rm", "dangerous", "delete"]);

        let decision = decide_permission("terminal", "run dangerous command", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));

        let decision = decide_permission("terminal", "delete file", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));

        // Multiple allow patterns - any match should allow
        let permissions = terminal_rules_with_allow(&["^cargo", "^npm", "^git"]);

        let decision = decide_permission("terminal", "cargo build", &permissions, false);
        assert_eq!(decision, ToolPermissionDecision::Allow);

        let decision = decide_permission("terminal", "npm install", &permissions, false);
        assert_eq!(decision, ToolPermissionDecision::Allow);

        // No pattern matches - falls through to default
        let decision = decide_permission("terminal", "rm file", &permissions, false);
        assert_eq!(decision, ToolPermissionDecision::Confirm);
    }

    #[test]
    fn test_case_insensitive_matching() {
        // Case-insensitive by default (case_sensitive: false)
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Confirm,
                always_allow: vec![],
                always_deny: vec![CompiledRegex::new(r"\brm\b", false).unwrap()],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        let permissions = ToolPermissions { tools };

        // Should match regardless of case
        let decision = decide_permission("terminal", "RM file.txt", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));

        let decision = decide_permission("terminal", "Rm file.txt", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));

        let decision = decide_permission("terminal", "rm file.txt", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));
    }

    #[test]
    fn test_case_sensitive_matching() {
        // Case-sensitive matching when explicitly enabled
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Confirm,
                always_allow: vec![],
                always_deny: vec![CompiledRegex::new("DROP TABLE", true).unwrap()],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        let permissions = ToolPermissions { tools };

        // Should only match exact case
        let decision = decide_permission("terminal", "DROP TABLE users", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));

        // Should NOT match different case
        let decision = decide_permission("terminal", "drop table users", &permissions, true);
        assert_eq!(decision, ToolPermissionDecision::Allow);
    }

    #[test]
    fn test_multi_tool_isolation() {
        // Rules for terminal should not affect edit_file
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Deny,
                always_allow: vec![],
                always_deny: vec![CompiledRegex::new("dangerous", false).unwrap()],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        tools.insert(
            Arc::from("edit_file"),
            ToolRules {
                default_mode: ToolPermissionMode::Allow,
                always_allow: vec![],
                always_deny: vec![],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        let permissions = ToolPermissions { tools };

        // Terminal with "dangerous" should be denied
        let decision = decide_permission("terminal", "run dangerous command", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));

        // edit_file with "dangerous" should be allowed (no deny rules for edit_file)
        let decision = decide_permission("edit_file", "dangerous_file.txt", &permissions, true);
        assert_eq!(decision, ToolPermissionDecision::Allow);

        // Terminal without "dangerous" should still be denied due to default_mode: Deny
        let decision = decide_permission("terminal", "safe command", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));
    }

    #[test]
    fn test_invalid_patterns_block_tool() {
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Allow,
                always_allow: vec![CompiledRegex::new("echo", false).unwrap()],
                always_deny: vec![],
                always_confirm: vec![],
                invalid_patterns: vec![InvalidRegexPattern {
                    pattern: "[invalid(regex".to_string(),
                    rule_type: "always_deny".to_string(),
                    error: "unclosed character class".to_string(),
                }],
            },
        );
        let permissions = ToolPermissions { tools };

        // Even though "echo" matches always_allow, the tool should be blocked
        // because there are invalid patterns
        let decision = decide_permission("terminal", "echo hello", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));

        if let ToolPermissionDecision::Deny(msg) = decision {
            assert!(
                msg.contains("regex"),
                "error message should mention regex: {}",
                msg
            );
            assert!(
                msg.contains("settings"),
                "error message should mention settings: {}",
                msg
            );
            assert!(
                msg.contains("terminal"),
                "error message should mention the tool name: {}",
                msg
            );
        }
    }

    #[test]
    fn test_same_pattern_in_deny_and_allow_deny_wins() {
        // When the same pattern appears in both deny and allow lists, deny should win
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Allow,
                always_allow: vec![CompiledRegex::new("deploy", false).unwrap()],
                always_deny: vec![CompiledRegex::new("deploy", false).unwrap()],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        let permissions = ToolPermissions { tools };

        let decision = decide_permission("terminal", "deploy production", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));
    }

    #[test]
    fn test_same_pattern_in_confirm_and_allow_confirm_wins() {
        // When the same pattern appears in both confirm and allow lists, confirm should win
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Allow,
                always_allow: vec![CompiledRegex::new("deploy", false).unwrap()],
                always_deny: vec![],
                always_confirm: vec![CompiledRegex::new("deploy", false).unwrap()],
                invalid_patterns: vec![],
            },
        );
        let permissions = ToolPermissions { tools };

        let decision = decide_permission("terminal", "deploy production", &permissions, true);
        assert_eq!(decision, ToolPermissionDecision::Confirm);
    }

    #[test]
    fn test_partial_tool_name_does_not_match() {
        // Rules for "term" should not affect "terminal"
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("term"),
            ToolRules {
                default_mode: ToolPermissionMode::Deny,
                always_allow: vec![],
                always_deny: vec![],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        let permissions = ToolPermissions { tools };

        // "terminal" should not be affected by "term" rules, falls back to global setting
        let decision = decide_permission("terminal", "echo hello", &permissions, true);
        assert_eq!(decision, ToolPermissionDecision::Allow);

        let decision = decide_permission("terminal", "echo hello", &permissions, false);
        assert_eq!(decision, ToolPermissionDecision::Confirm);
    }

    #[test]
    fn test_very_long_input() {
        // Test that very long inputs are handled correctly
        let permissions = terminal_rules_with_deny(&[r"\brm\b"]);

        // Long input without the pattern should not match
        let long_safe_input = "echo ".to_string() + &"a".repeat(100_000);
        let decision = decide_permission("terminal", &long_safe_input, &permissions, true);
        assert_eq!(decision, ToolPermissionDecision::Allow);

        // Long input with the pattern should match
        let long_dangerous_input = "a".repeat(50_000) + " rm " + &"b".repeat(50_000);
        let decision = decide_permission("terminal", &long_dangerous_input, &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));
    }
}
