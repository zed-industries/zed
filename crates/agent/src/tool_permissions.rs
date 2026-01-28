use crate::AgentTool;
use crate::shell_parser::extract_commands;
use crate::tools::TerminalTool;
use agent_settings::{AgentSettings, ToolPermissions, ToolRules};
use settings::ToolPermissionMode;
use util::shell::ShellKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolPermissionDecision {
    Allow,
    Deny(String),
    Confirm,
}

impl ToolPermissionDecision {
    /// Determines the permission decision for a tool invocation based on configured rules.
    ///
    /// # Precedence Order (highest to lowest)
    ///
    /// 1. **`always_allow_tool_actions`** - When enabled, allows all tool actions except those
    ///    blocked by `always_deny` patterns. This global setting takes precedence over
    ///    `always_confirm` patterns and `default_mode`.
    /// 2. **`always_deny`** - If any deny pattern matches, the tool call is blocked immediately.
    ///    This takes precedence over all other rules for security (including `always_allow_tool_actions`).
    /// 3. **`always_confirm`** - If any confirm pattern matches (and no deny matched),
    ///    the user is prompted for confirmation (unless `always_allow_tool_actions` is enabled).
    /// 4. **`always_allow`** - If any allow pattern matches (and no deny/confirm matched),
    ///    the tool call proceeds without prompting.
    /// 5. **`default_mode`** - If no patterns match, falls back to the tool's default mode.
    ///
    /// # Shell Compatibility (Terminal Tool Only)
    ///
    /// For the terminal tool, commands are parsed to extract sub-commands for security.
    /// This parsing only works for shells with POSIX-like `&&` / `||` / `;` / `|` syntax:
    ///
    /// **Compatible shells:** Posix (sh, bash, dash, zsh), Fish 3.0+, PowerShell 7+/Pwsh,
    /// Cmd, Xonsh, Csh, Tcsh
    ///
    /// **Incompatible shells:** Nushell, Elvish, Rc (Plan 9)
    ///
    /// For incompatible shells, `always_allow` patterns are disabled for safety.
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
    pub fn from_input(
        tool_name: &str,
        input: &str,
        permissions: &ToolPermissions,
        always_allow_tool_actions: bool,
        shell_kind: ShellKind,
    ) -> ToolPermissionDecision {
        // If always_allow_tool_actions is enabled, bypass all permission checks.
        if always_allow_tool_actions {
            return ToolPermissionDecision::Allow;
        }

        let rules = match permissions.tools.get(tool_name) {
            Some(rules) => rules,
            None => {
                return ToolPermissionDecision::Confirm;
            }
        };

        // Check for invalid regex patterns before evaluating rules.
        // If any patterns failed to compile, block the tool call entirely.
        if let Some(error) = check_invalid_patterns(tool_name, rules) {
            return ToolPermissionDecision::Deny(error);
        }

        // For the terminal tool, parse the command to extract all sub-commands.
        // This prevents shell injection attacks where a user configures an allow
        // pattern like "^ls" and an attacker crafts "ls && rm -rf /".
        //
        // If parsing fails or the shell syntax is unsupported, always_allow is
        // disabled for this command (we set allow_enabled to false to signal this).
        if tool_name == TerminalTool::name() {
            // Our shell parser (brush-parser) only supports POSIX-like shell syntax.
            // See the doc comment above for the list of compatible/incompatible shells.
            if !shell_kind.supports_posix_chaining() {
                // For shells with incompatible syntax, we can't reliably parse
                // the command to extract sub-commands.
                if !rules.always_allow.is_empty() {
                    // If the user has configured always_allow patterns, we must deny
                    // because we can't safely verify the command doesn't contain
                    // hidden sub-commands that bypass the allow patterns.
                    return ToolPermissionDecision::Deny(format!(
                        "The {} shell does not support \"always allow\" patterns for the terminal \
                         tool because Zed cannot parse its command chaining syntax. Please remove \
                         the always_allow patterns from your tool_permissions settings, or switch \
                         to a supported shell (bash, zsh, fish, powershell, etc.).",
                        shell_kind
                    ));
                }
                // No always_allow rules, so we can still check deny/confirm patterns.
                return check_commands(std::iter::once(input.to_string()), rules, tool_name, false);
            }

            match extract_commands(input) {
                Some(commands) => check_commands(commands, rules, tool_name, true),
                None => {
                    // The command failed to parse, so we check to see if we should auto-deny
                    // or auto-confirm; if neither auto-deny nor auto-confirm applies here,
                    // fall back on the default (based on the user's settings, which is Confirm
                    // if not specified otherwise). Ignore "always allow" when it failed to parse.
                    check_commands(std::iter::once(input.to_string()), rules, tool_name, false)
                }
            }
        } else {
            check_commands(std::iter::once(input.to_string()), rules, tool_name, true)
        }
    }
}

fn check_commands(
    commands: impl IntoIterator<Item = String>,
    rules: &ToolRules,
    tool_name: &str,
    allow_enabled: bool,
) -> ToolPermissionDecision {
    // Single pass through all commands:
    // - DENY: If ANY command matches a deny pattern, deny immediately (short-circuit)
    // - CONFIRM: Track if ANY command matches a confirm pattern
    // - ALLOW: Track if ALL commands match at least one allow pattern
    let mut any_matched_confirm = false;
    let mut all_matched_allow = true;
    let mut had_any_commands = false;

    for command in commands {
        had_any_commands = true;

        // DENY: immediate return if any command matches a deny pattern
        if rules.always_deny.iter().any(|r| r.is_match(&command)) {
            return ToolPermissionDecision::Deny(format!(
                "Command blocked by security rule for {} tool",
                tool_name
            ));
        }

        // CONFIRM: remember if any command matches a confirm pattern
        if rules.always_confirm.iter().any(|r| r.is_match(&command)) {
            any_matched_confirm = true;
        }

        // ALLOW: track if all commands match at least one allow pattern
        if !rules.always_allow.iter().any(|r| r.is_match(&command)) {
            all_matched_allow = false;
        }
    }

    // After processing all commands, check accumulated state
    if any_matched_confirm {
        return ToolPermissionDecision::Confirm;
    }

    if allow_enabled && all_matched_allow && had_any_commands {
        return ToolPermissionDecision::Allow;
    }

    match rules.default_mode {
        ToolPermissionMode::Deny => {
            ToolPermissionDecision::Deny(format!("{} tool is disabled", tool_name))
        }
        ToolPermissionMode::Allow => ToolPermissionDecision::Allow,
        ToolPermissionMode::Confirm => ToolPermissionDecision::Confirm,
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
/// delegates to [`ToolPermissionDecision::from_input`], using the system shell.
pub fn decide_permission_from_settings(
    tool_name: &str,
    input: &str,
    settings: &AgentSettings,
) -> ToolPermissionDecision {
    ToolPermissionDecision::from_input(
        tool_name,
        input,
        &settings.tool_permissions,
        settings.always_allow_tool_actions,
        ShellKind::system(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use agent_settings::{CompiledRegex, InvalidRegexPattern, ToolRules};
    use std::sync::Arc;

    struct PermTest {
        tool: &'static str,
        input: &'static str,
        mode: ToolPermissionMode,
        allow: Vec<&'static str>,
        deny: Vec<&'static str>,
        confirm: Vec<&'static str>,
        global: bool,
    }

    impl PermTest {
        fn new(input: &'static str) -> Self {
            Self {
                tool: "terminal",
                input,
                mode: ToolPermissionMode::Confirm,
                allow: vec![],
                deny: vec![],
                confirm: vec![],
                global: false,
            }
        }

        fn tool(mut self, t: &'static str) -> Self {
            self.tool = t;
            self
        }
        fn mode(mut self, m: ToolPermissionMode) -> Self {
            self.mode = m;
            self
        }
        fn allow(mut self, p: &[&'static str]) -> Self {
            self.allow = p.to_vec();
            self
        }
        fn deny(mut self, p: &[&'static str]) -> Self {
            self.deny = p.to_vec();
            self
        }
        fn confirm(mut self, p: &[&'static str]) -> Self {
            self.confirm = p.to_vec();
            self
        }
        fn global(mut self, g: bool) -> Self {
            self.global = g;
            self
        }

        fn is_allow(self) {
            assert_eq!(
                self.run(),
                ToolPermissionDecision::Allow,
                "expected Allow for '{}'",
                self.input
            );
        }
        fn is_deny(self) {
            assert!(
                matches!(self.run(), ToolPermissionDecision::Deny(_)),
                "expected Deny for '{}'",
                self.input
            );
        }
        fn is_confirm(self) {
            assert_eq!(
                self.run(),
                ToolPermissionDecision::Confirm,
                "expected Confirm for '{}'",
                self.input
            );
        }

        fn run(&self) -> ToolPermissionDecision {
            let mut tools = collections::HashMap::default();
            tools.insert(
                Arc::from(self.tool),
                ToolRules {
                    default_mode: self.mode,
                    always_allow: self
                        .allow
                        .iter()
                        .filter_map(|p| CompiledRegex::new(p, false))
                        .collect(),
                    always_deny: self
                        .deny
                        .iter()
                        .filter_map(|p| CompiledRegex::new(p, false))
                        .collect(),
                    always_confirm: self
                        .confirm
                        .iter()
                        .filter_map(|p| CompiledRegex::new(p, false))
                        .collect(),
                    invalid_patterns: vec![],
                },
            );
            ToolPermissionDecision::from_input(
                self.tool,
                self.input,
                &ToolPermissions { tools },
                self.global,
                ShellKind::Posix,
            )
        }
    }

    fn t(input: &'static str) -> PermTest {
        PermTest::new(input)
    }

    fn no_rules(input: &str, global: bool) -> ToolPermissionDecision {
        ToolPermissionDecision::from_input(
            "terminal",
            input,
            &ToolPermissions {
                tools: collections::HashMap::default(),
            },
            global,
            ShellKind::Posix,
        )
    }

    // allow pattern matches
    #[test]
    fn allow_exact_match() {
        t("cargo test").allow(&["^cargo\\s"]).is_allow();
    }
    #[test]
    fn allow_with_args() {
        t("cargo build --release").allow(&["^cargo\\s"]).is_allow();
    }
    #[test]
    fn allow_one_of_many() {
        t("npm install").allow(&["^cargo\\s", "^npm\\s"]).is_allow();
    }
    #[test]
    fn allow_middle_pattern() {
        t("run cargo now").allow(&["cargo"]).is_allow();
    }
    #[test]
    fn allow_anchor_prevents_middle() {
        t("run cargo now").allow(&["^cargo"]).is_confirm();
    }

    // allow pattern doesn't match -> falls through
    #[test]
    fn allow_no_match_confirms() {
        t("python x.py").allow(&["^cargo\\s"]).is_confirm();
    }
    #[test]
    fn allow_no_match_global_allows() {
        t("python x.py")
            .allow(&["^cargo\\s"])
            .global(true)
            .is_allow();
    }

    // deny pattern matches
    #[test]
    fn deny_blocks() {
        t("rm -rf /").deny(&["rm\\s+-rf"]).is_deny();
    }
    #[test]
    fn global_bypasses_deny() {
        // always_allow_tool_actions bypasses ALL checks, including deny
        t("rm -rf /").deny(&["rm\\s+-rf"]).global(true).is_allow();
    }
    #[test]
    fn deny_blocks_with_mode_allow() {
        t("rm -rf /")
            .deny(&["rm\\s+-rf"])
            .mode(ToolPermissionMode::Allow)
            .is_deny();
    }
    #[test]
    fn deny_middle_match() {
        t("echo rm -rf x").deny(&["rm\\s+-rf"]).is_deny();
    }
    #[test]
    fn deny_no_match_allows() {
        t("ls -la").deny(&["rm\\s+-rf"]).global(true).is_allow();
    }

    // confirm pattern matches
    #[test]
    fn confirm_requires_confirm() {
        t("sudo apt install").confirm(&["sudo\\s"]).is_confirm();
    }
    #[test]
    fn global_overrides_confirm() {
        t("sudo reboot")
            .confirm(&["sudo\\s"])
            .global(true)
            .is_allow();
    }
    #[test]
    fn confirm_overrides_mode_allow() {
        t("sudo x")
            .confirm(&["sudo"])
            .mode(ToolPermissionMode::Allow)
            .is_confirm();
    }

    // confirm beats allow
    #[test]
    fn confirm_beats_allow() {
        t("git push --force")
            .allow(&["^git\\s"])
            .confirm(&["--force"])
            .is_confirm();
    }
    #[test]
    fn confirm_beats_allow_overlap() {
        t("deploy prod")
            .allow(&["deploy"])
            .confirm(&["prod"])
            .is_confirm();
    }
    #[test]
    fn allow_when_confirm_no_match() {
        t("git status")
            .allow(&["^git\\s"])
            .confirm(&["--force"])
            .is_allow();
    }

    // deny beats allow
    #[test]
    fn deny_beats_allow() {
        t("rm -rf /tmp/x")
            .allow(&["/tmp/"])
            .deny(&["rm\\s+-rf"])
            .is_deny();
    }
    #[test]
    fn deny_beats_allow_diff() {
        t("bad deploy").allow(&["deploy"]).deny(&["bad"]).is_deny();
    }

    // deny beats confirm
    #[test]
    fn deny_beats_confirm() {
        t("sudo rm -rf /")
            .confirm(&["sudo"])
            .deny(&["rm\\s+-rf"])
            .is_deny();
    }

    // deny beats everything
    #[test]
    fn deny_beats_all() {
        t("bad cmd")
            .allow(&["cmd"])
            .confirm(&["cmd"])
            .deny(&["bad"])
            .is_deny();
    }

    // no patterns -> default_mode
    #[test]
    fn default_confirm() {
        t("python x.py")
            .mode(ToolPermissionMode::Confirm)
            .is_confirm();
    }
    #[test]
    fn default_allow() {
        t("python x.py").mode(ToolPermissionMode::Allow).is_allow();
    }
    #[test]
    fn default_deny() {
        t("python x.py").mode(ToolPermissionMode::Deny).is_deny();
    }
    #[test]
    fn default_deny_global_true() {
        t("python x.py")
            .mode(ToolPermissionMode::Deny)
            .global(true)
            .is_allow();
    }

    // default_mode confirm + global
    #[test]
    fn default_confirm_global_false() {
        t("x")
            .mode(ToolPermissionMode::Confirm)
            .global(false)
            .is_confirm();
    }
    #[test]
    fn default_confirm_global_true() {
        t("x")
            .mode(ToolPermissionMode::Confirm)
            .global(true)
            .is_allow();
    }

    // no rules at all -> global setting
    #[test]
    fn no_rules_global_false() {
        assert_eq!(no_rules("x", false), ToolPermissionDecision::Confirm);
    }
    #[test]
    fn no_rules_global_true() {
        assert_eq!(no_rules("x", true), ToolPermissionDecision::Allow);
    }

    // empty input
    #[test]
    fn empty_input_no_match() {
        t("").deny(&["rm"]).is_confirm();
    }
    #[test]
    fn empty_input_global() {
        t("").deny(&["rm"]).global(true).is_allow();
    }

    // multiple patterns - any match
    #[test]
    fn multi_deny_first() {
        t("rm x").deny(&["rm", "del", "drop"]).is_deny();
    }
    #[test]
    fn multi_deny_last() {
        t("drop x").deny(&["rm", "del", "drop"]).is_deny();
    }
    #[test]
    fn multi_allow_first() {
        t("cargo x").allow(&["^cargo", "^npm", "^git"]).is_allow();
    }
    #[test]
    fn multi_allow_last() {
        t("git x").allow(&["^cargo", "^npm", "^git"]).is_allow();
    }
    #[test]
    fn multi_none_match() {
        t("python x")
            .allow(&["^cargo", "^npm"])
            .deny(&["rm"])
            .is_confirm();
    }

    // tool isolation
    #[test]
    fn other_tool_not_affected() {
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
        let p = ToolPermissions { tools };
        // With always_allow_tool_actions=true, even default_mode: Deny is overridden
        assert_eq!(
            ToolPermissionDecision::from_input("terminal", "x", &p, true, ShellKind::Posix),
            ToolPermissionDecision::Allow
        );
        // With always_allow_tool_actions=false, default_mode: Deny is respected
        assert!(matches!(
            ToolPermissionDecision::from_input("terminal", "x", &p, false, ShellKind::Posix),
            ToolPermissionDecision::Deny(_)
        ));
        assert_eq!(
            ToolPermissionDecision::from_input("edit_file", "x", &p, false, ShellKind::Posix),
            ToolPermissionDecision::Allow
        );
    }

    #[test]
    fn partial_tool_name_no_match() {
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
        let p = ToolPermissions { tools };
        assert_eq!(
            ToolPermissionDecision::from_input("terminal", "x", &p, true, ShellKind::Posix),
            ToolPermissionDecision::Allow
        );
    }

    // invalid patterns block the tool (but global bypasses all checks)
    #[test]
    fn invalid_pattern_blocks() {
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from("terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Allow,
                always_allow: vec![CompiledRegex::new("echo", false).unwrap()],
                always_deny: vec![],
                always_confirm: vec![],
                invalid_patterns: vec![InvalidRegexPattern {
                    pattern: "[bad".into(),
                    rule_type: "always_deny".into(),
                    error: "err".into(),
                }],
            },
        );
        let p = ToolPermissions {
            tools: tools.clone(),
        };
        // With global=true, all checks are bypassed including invalid pattern check
        assert!(matches!(
            ToolPermissionDecision::from_input("terminal", "echo hi", &p, true, ShellKind::Posix),
            ToolPermissionDecision::Allow
        ));
        // With global=false, invalid patterns block the tool
        assert!(matches!(
            ToolPermissionDecision::from_input("terminal", "echo hi", &p, false, ShellKind::Posix),
            ToolPermissionDecision::Deny(_)
        ));
    }

    // user scenario: only echo allowed, git should confirm
    #[test]
    fn user_scenario_only_echo() {
        t("echo hello").allow(&["^echo\\s"]).is_allow();
    }
    #[test]
    fn user_scenario_git_confirms() {
        t("git status").allow(&["^echo\\s"]).is_confirm();
    }
    #[test]
    fn user_scenario_rm_confirms() {
        t("rm -rf /").allow(&["^echo\\s"]).is_confirm();
    }

    // shell injection: && in command should NOT be auto-approved just because
    // the first part matches an allow pattern
    #[test]
    fn shell_injection_via_double_ampersand_not_allowed() {
        // If "ls" is in always_allow, a command like "ls && rm -rf /" should NOT
        // be auto-approved because it contains a dangerous secondary command.
        // This test should FAIL with the current implementation (demonstrating the vulnerability)
        // and PASS once the fix is in place.
        t("ls && rm -rf /").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_via_semicolon_not_allowed() {
        // Similarly, "ls; rm -rf /" should not be auto-approved
        t("ls; rm -rf /").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_via_pipe_not_allowed() {
        // "ls | xargs rm -rf" should not be auto-approved just because "ls" is allowed
        t("ls | xargs rm -rf").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_via_backticks_not_allowed() {
        // "echo `rm -rf /`" should not be auto-approved just because "echo" is allowed
        t("echo `rm -rf /`").allow(&["^echo\\s"]).is_confirm();
    }

    #[test]
    fn shell_injection_via_dollar_parens_not_allowed() {
        // "echo $(rm -rf /)" should not be auto-approved just because "echo" is allowed
        t("echo $(rm -rf /)").allow(&["^echo\\s"]).is_confirm();
    }

    #[test]
    fn shell_injection_via_or_operator_not_allowed() {
        // "ls || rm -rf /" should not be auto-approved (OR operator runs second if first fails)
        t("ls || rm -rf /").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_via_background_operator_not_allowed() {
        // "ls & rm -rf /" should not be auto-approved (background operator)
        t("ls & rm -rf /").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_via_newline_not_allowed() {
        // "ls\nrm -rf /" should not be auto-approved (newline is a command separator)
        t("ls\nrm -rf /").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_via_process_substitution_input_not_allowed() {
        // "cat <(rm -rf /)" should not be auto-approved (process substitution)
        t("cat <(rm -rf /)").allow(&["^cat"]).is_confirm();
    }

    #[test]
    fn shell_injection_via_process_substitution_output_not_allowed() {
        // "ls >(rm -rf /)" should not be auto-approved (process substitution for output)
        t("ls >(rm -rf /)").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_without_spaces_not_allowed() {
        // "ls&&rm -rf /" (no spaces around &&) should not be auto-approved
        t("ls&&rm -rf /").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_semicolon_no_space_not_allowed() {
        // "ls;rm -rf /" (no space after semicolon) should not be auto-approved
        t("ls;rm -rf /").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_multiple_chained_operators_not_allowed() {
        // Multiple chained commands should not be auto-approved
        t("ls && echo hello && rm -rf /")
            .allow(&["^ls"])
            .is_confirm();
    }

    #[test]
    fn shell_injection_mixed_operators_not_allowed() {
        // Mixed operators should not be auto-approved
        t("ls; echo hello && rm -rf /").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_pipe_stderr_not_allowed() {
        // "|&" pipes both stdout and stderr (bash-specific)
        t("ls |& rm -rf /").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn allow_requires_all_commands_to_match() {
        // Both "ls" and "echo" must be allowed for this to pass
        t("ls && echo hello").allow(&["^ls", "^echo"]).is_allow();
    }

    #[test]
    fn deny_triggers_on_any_matching_command() {
        // Even though "ls" is allowed, "rm" is denied, so entire command is denied
        t("ls && rm file").allow(&["^ls"]).deny(&["^rm"]).is_deny();
    }

    #[test]
    fn confirm_triggers_on_any_matching_command() {
        // "ls" is allowed but "sudo" requires confirm
        t("ls && sudo reboot")
            .allow(&["^ls"])
            .confirm(&["^sudo"])
            .is_confirm();
    }

    #[test]
    fn nested_command_substitution_all_checked() {
        // All three commands (echo, cat, whoami) must be allowed
        t("echo $(cat $(whoami).txt)")
            .allow(&["^echo", "^cat", "^whoami"])
            .is_allow();
    }

    #[test]
    fn parse_failure_falls_back_to_confirm() {
        // Invalid syntax should not auto-allow, falls back to original string matching
        // Since "ls &&" doesn't match "^ls$" exactly, it should confirm
        t("ls &&").allow(&["^ls$"]).is_confirm();
    }

    // mcp tools
    #[test]
    fn mcp_allow() {
        t("")
            .tool("mcp:fs:read")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
    }
    #[test]
    fn mcp_deny() {
        t("")
            .tool("mcp:bad:del")
            .mode(ToolPermissionMode::Deny)
            .is_deny();
    }
    #[test]
    fn mcp_confirm() {
        t("")
            .tool("mcp:gh:issue")
            .mode(ToolPermissionMode::Confirm)
            .is_confirm();
    }
    #[test]
    fn mcp_confirm_global() {
        t("")
            .tool("mcp:gh:issue")
            .mode(ToolPermissionMode::Confirm)
            .global(true)
            .is_allow();
    }

    // mcp vs builtin isolation
    #[test]
    fn mcp_doesnt_collide_with_builtin() {
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
        tools.insert(
            Arc::from("mcp:srv:terminal"),
            ToolRules {
                default_mode: ToolPermissionMode::Allow,
                always_allow: vec![],
                always_deny: vec![],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        let p = ToolPermissions { tools };
        assert!(matches!(
            ToolPermissionDecision::from_input("terminal", "x", &p, false, ShellKind::Posix),
            ToolPermissionDecision::Deny(_)
        ));
        assert_eq!(
            ToolPermissionDecision::from_input(
                "mcp:srv:terminal",
                "x",
                &p,
                false,
                ShellKind::Posix
            ),
            ToolPermissionDecision::Allow
        );
    }
}
