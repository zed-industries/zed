use agent_settings::{AgentSettings, ToolPermissions};
use settings::ToolPermissionMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolPermissionDecision {
    Allow,
    Deny(String),
    Confirm,
}

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
    use agent_settings::{CompiledRegex, ToolRules};
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
            },
        );
        ToolPermissions { tools }
    }

    fn terminal_rules_with_confirm(patterns: &[&str]) -> ToolPermissions {
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Allow,
                always_allow: vec![],
                always_deny: vec![],
                always_confirm: patterns
                    .iter()
                    .filter_map(|p| CompiledRegex::new(p, false))
                    .collect(),
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
            },
        );
        let permissions = ToolPermissions { tools };

        let decision = decide_permission("terminal", "do risky thing", &permissions, true);
        assert_eq!(decision, ToolPermissionDecision::Confirm);
    }

    #[test]
    fn test_allow_rule_matches() {
        let permissions = terminal_rules_with_allow(&["^cargo\\s"]);
        let decision = decide_permission("terminal", "cargo build", &permissions, false);
        assert_eq!(decision, ToolPermissionDecision::Allow);
    }

    #[test]
    fn test_deny_rule_matches() {
        let permissions = terminal_rules_with_deny(&["rm\\s+-rf"]);
        let decision = decide_permission("terminal", "rm -rf /", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));
    }

    #[test]
    fn test_confirm_rule_matches() {
        let permissions = terminal_rules_with_confirm(&["rm\\s"]);
        let decision = decide_permission("terminal", "rm file.txt", &permissions, true);
        assert_eq!(decision, ToolPermissionDecision::Confirm);
    }

    #[test]
    fn test_no_rules_configured_uses_always_allow_setting() {
        let permissions = empty_permissions();

        let decision = decide_permission("terminal", "any command", &permissions, true);
        assert_eq!(decision, ToolPermissionDecision::Allow);

        let decision = decide_permission("terminal", "any command", &permissions, false);
        assert_eq!(decision, ToolPermissionDecision::Confirm);
    }

    #[test]
    fn test_default_mode_deny() {
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Deny,
                always_allow: vec![],
                always_deny: vec![],
                always_confirm: vec![],
            },
        );
        let permissions = ToolPermissions { tools };

        let decision = decide_permission("terminal", "any command", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));
    }

    #[test]
    fn test_default_mode_allow() {
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Allow,
                always_allow: vec![],
                always_deny: vec![],
                always_confirm: vec![],
            },
        );
        let permissions = ToolPermissions { tools };

        let decision = decide_permission("terminal", "any command", &permissions, false);
        assert_eq!(decision, ToolPermissionDecision::Allow);
    }

    #[test]
    fn test_default_mode_confirm_respects_always_allow_tool_actions() {
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Confirm,
                always_allow: vec![],
                always_deny: vec![],
                always_confirm: vec![],
            },
        );
        let permissions = ToolPermissions { tools };

        let decision = decide_permission("terminal", "any command", &permissions, true);
        assert_eq!(decision, ToolPermissionDecision::Allow);

        let decision = decide_permission("terminal", "any command", &permissions, false);
        assert_eq!(decision, ToolPermissionDecision::Confirm);
    }

    #[test]
    fn test_regex_matches_anywhere_in_string() {
        let permissions = terminal_rules_with_deny(&["rm\\s+-rf", "/etc/passwd"]);

        let decision =
            decide_permission("terminal", "echo hello && rm -rf /", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));

        let decision = decide_permission("terminal", "cat /etc/passwd | grep root", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));
    }

    #[test]
    fn test_case_insensitive_matching() {
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Confirm,
                always_allow: vec![],
                always_deny: vec![CompiledRegex::new("DELETE", false).unwrap()],
                always_confirm: vec![],
            },
        );
        let permissions = ToolPermissions { tools };

        let decision = decide_permission("terminal", "delete file.txt", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));
    }

    #[test]
    fn test_case_sensitive_matching() {
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Confirm,
                always_allow: vec![],
                always_deny: vec![CompiledRegex::new("DELETE", true).unwrap()],
                always_confirm: vec![],
            },
        );
        let permissions = ToolPermissions { tools };

        let decision = decide_permission("terminal", "delete file.txt", &permissions, false);
        assert_eq!(decision, ToolPermissionDecision::Confirm);

        let decision = decide_permission("terminal", "DELETE file.txt", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));
    }

    #[test]
    fn test_unknown_tool_uses_fallback() {
        let permissions = empty_permissions();

        let decision = decide_permission("unknown_tool", "some input", &permissions, false);
        assert_eq!(decision, ToolPermissionDecision::Confirm);

        let decision = decide_permission("unknown_tool", "some input", &permissions, true);
        assert_eq!(decision, ToolPermissionDecision::Allow);
    }

    #[test]
    fn test_fork_bomb_blocked() {
        let permissions = terminal_rules_with_deny(&[r":\(\)\{\s*:\|:&\s*\};:"]);

        let decision = decide_permission("terminal", ":(){ :|:& };:", &permissions, true);
        assert!(matches!(decision, ToolPermissionDecision::Deny(_)));
    }
}
