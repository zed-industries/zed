# Shell Command Parsing Security Fix - Implementation Plan

## Executive Summary

This document outlines a plan to fix a security vulnerability in Zed's AI agent tool permission system. The vulnerability allows shell command injection to bypass the `always_allow` pattern matching in the terminal tool. For example, if a user has configured `^ls` as an always-allow pattern, an attacker could craft a command like `ls && rm -rf /` which would be auto-approved because the regex only matches the beginning of the command string, ignoring the dangerous secondary command.

## Problem Statement

### Current Behavior (Vulnerable)

The `decide_permission` function in `zed/crates/agent/src/tool_permissions.rs` matches regex patterns against the entire command string as a single unit. This is problematic because shell commands can contain multiple sub-commands connected by operators like:

- `&&` (AND - run second if first succeeds)
- `||` (OR - run second if first fails)
- `;` (sequential execution)
- `&` (background execution)
- `|` (pipe stdout)
- `|&` (pipe stdout and stderr)
- `\n` (newline as command separator)
- `` `cmd` `` (backtick command substitution)
- `$(cmd)` (dollar-paren command substitution)
- `<(cmd)` / `>(cmd)` (process substitution)

### Example Exploit

```
User configuration:
  always_allow: ["^ls"]

Command submitted by AI:
  "ls && rm -rf /"

Current behavior:
  - Pattern "^ls" matches "ls && rm -rf /" ✓
  - Result: ALLOW (dangerous!)

Expected behavior:
  - Parse into sub-commands: ["ls", "rm -rf /"]
  - Pattern "^ls" matches "ls" ✓
  - Pattern "^ls" does NOT match "rm -rf /" ✗
  - Result: CONFIRM (safe - user must approve)
```

### Desired Behavior

For the `always_allow` rules to approve a command, **ALL** sub-commands extracted from the shell command must match at least one allow pattern.

For `always_deny` rules, if **ANY** sub-command matches a deny pattern, the command is denied.

For `always_confirm` rules, if **ANY** sub-command matches a confirm pattern, confirmation is required.

**Precedence order remains:** deny > confirm > allow

## Proposed Solution

### Overview

1. Add the `brush-parser` crate as a dependency (MIT licensed, POSIX/bash shell parser)
2. Create a new module `shell_parser.rs` that extracts all sub-commands from a shell command string
3. Modify `decide_permission` to evaluate patterns against each extracted sub-command
4. Update the matching logic:
   - `always_allow`: ALL sub-commands must match at least one pattern
   - `always_deny`: ANY sub-command matching triggers denial
   - `always_confirm`: ANY sub-command matching triggers confirmation

### Recommended Crate: `brush-parser`

**Why `brush-parser`?**
- MIT licensed
- Actively maintained (part of the `brush-shell` project)
- Produces a proper AST with types for:
  - `AndOrList` - sequences connected by `&&` and `||`
  - `Pipeline` - commands connected by `|`
  - `CompoundCommand` - subshells, brace groups, loops
  - Command substitution parsing
- Handles POSIX shell syntax (not bash-specific)

**Crate location:** https://crates.io/crates/brush-parser

## Implementation Steps

### Step 1: Add Dependency

Add `brush-parser` to `zed/crates/agent/Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies ...
brush-parser.workspace = true
```

Also add to root `Cargo.toml` workspace dependencies:

```toml
[workspace.dependencies]
brush-parser = "0.3"
```

### Step 2: Create Shell Parser Module

Create new file `zed/crates/agent/src/shell_parser.rs`:

```rust
//! Utilities for parsing shell commands to extract sub-commands for permission checking.
//!
//! This module provides functionality to parse a shell command string and extract
//! all individual commands that would be executed. This is used by the permission
//! system to ensure that ALL commands in a compound command are checked against
//! permission rules, preventing shell injection attacks.

use brush_parser::{Parser, ast};

/// Extracts all executable command strings from a shell command.
///
/// This function parses the input as a POSIX shell command and recursively
/// extracts all simple commands that would be executed. This includes commands
/// connected by:
/// - `&&` and `||` (boolean operators)
/// - `;` and `&` (sequential/background execution)
/// - `|` (pipes)
/// - Command substitution (`$()` and backticks)
/// - Process substitution (`<()` and `>()`)
///
/// # Arguments
/// * `command` - The shell command string to parse
///
/// # Returns
/// * `Ok(Vec<String>)` - A list of individual command strings
/// * `Err(...)` - If parsing fails
///
/// # Example
/// ```
/// let commands = extract_commands("ls && rm -rf /")?;
/// assert_eq!(commands, vec!["ls", "rm -rf /"]);
/// ```
pub fn extract_commands(command: &str) -> Result<Vec<String>, ShellParseError> {
    // Implementation details in Step 2
}

/// Error type for shell parsing failures
#[derive(Debug, Clone)]
pub enum ShellParseError {
    /// The command could not be parsed as valid shell syntax
    ParseError(String),
}
```

The implementation should:

1. Use `brush_parser::Parser` to parse the command string
2. Walk the AST recursively to find all `SimpleCommand` nodes
3. Also recurse into:
   - `AndOrList.additional` for `&&` and `||` chained commands
   - `Pipeline` sequences for piped commands
   - `CompoundCommand::Subshell` for subshell commands
   - Command substitution within words
   - Process substitution

### Step 3: Modify Permission Decision Logic

Update `zed/crates/agent/src/tool_permissions.rs`:

```rust
use crate::shell_parser::{extract_commands, ShellParseError};

pub fn decide_permission(
    tool_name: &str,
    input: &str,
    permissions: &ToolPermissions,
    always_allow_tool_actions: bool,
) -> ToolPermissionDecision {
    // ... existing early returns for missing rules ...

    // For terminal tool, parse the command to extract sub-commands
    let commands_to_check = if tool_name == "terminal" {
        match extract_commands(input) {
            Ok(commands) if !commands.is_empty() => commands,
            Ok(_) => vec![input.to_string()], // Empty parse result, use original
            Err(_) => vec![input.to_string()], // Parse error, fall back to original
        }
    } else {
        vec![input.to_string()]
    };

    // Check for invalid regex patterns (existing logic)
    if let Some(error) = check_invalid_patterns(tool_name, rules) {
        return ToolPermissionDecision::Deny(error);
    }

    // DENY: If ANY command matches a deny pattern, deny the entire command
    for cmd in &commands_to_check {
        if rules.always_deny.iter().any(|r| r.is_match(cmd)) {
            return ToolPermissionDecision::Deny(format!(
                "Command blocked by security rule for {} tool",
                tool_name
            ));
        }
    }

    // CONFIRM: If ANY command matches a confirm pattern, require confirmation
    // (unless always_allow_tool_actions is true)
    if !always_allow_tool_actions {
        for cmd in &commands_to_check {
            if rules.always_confirm.iter().any(|r| r.is_match(cmd)) {
                return ToolPermissionDecision::Confirm;
            }
        }
    }

    // ALLOW: ALL commands must match at least one allow pattern
    let all_allowed = commands_to_check.iter().all(|cmd| {
        rules.always_allow.iter().any(|r| r.is_match(cmd))
    });

    if all_allowed && !commands_to_check.is_empty() {
        return ToolPermissionDecision::Allow;
    }

    // Fall through to default behavior
    if always_allow_tool_actions {
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
```

### Step 4: Register the New Module

Add to `zed/crates/agent/src/agent.rs`:

```rust
mod shell_parser;
```

### Step 5: Handle Edge Cases

The implementation should handle these edge cases gracefully:

1. **Parse failures**: If `brush-parser` fails to parse a command, fall back to treating the entire string as a single command (current behavior). This ensures we don't break existing functionality.

2. **Empty commands**: If parsing returns no commands, treat the original input as a single command.

3. **Non-terminal tools**: Only apply shell parsing to the `terminal` tool. Other tools (edit_file, fetch, etc.) should continue using the original single-string matching.

4. **Nested command substitution**: Ensure recursive extraction captures commands like:
   ```bash
   echo "$(cat $(whoami).txt)"
   # Should extract: echo, cat, whoami
   ```

5. **Here documents**: Handle heredocs appropriately (the content is data, not commands).

## Testing Strategy

### Existing Tests That Should Pass

All existing tests in `zed/crates/agent/src/tool_permissions.rs` should continue to pass, as they test single commands without shell operators.

### New Tests That Should Now Pass

The shell injection tests added in the test file should pass after the fix:

```rust
#[test]
fn shell_injection_via_double_ampersand_not_allowed() {
    t("ls && rm -rf /").allow(&["^ls"]).is_confirm();
}

#[test]
fn shell_injection_via_semicolon_not_allowed() {
    t("ls; rm -rf /").allow(&["^ls"]).is_confirm();
}

#[test]
fn shell_injection_via_pipe_not_allowed() {
    t("ls | xargs rm -rf").allow(&["^ls"]).is_confirm();
}

// ... and 12 more shell injection tests
```

### Additional Tests to Add

```rust
// Test that ALL commands must match for allow
#[test]
fn allow_requires_all_commands_to_match() {
    // Both "ls" and "echo" must be allowed for this to pass
    t("ls && echo hello")
        .allow(&["^ls", "^echo"])
        .is_allow();
}

// Test deny on any command
#[test]
fn deny_triggers_on_any_matching_command() {
    // Even though "ls" is allowed, "rm" is denied, so entire command is denied
    t("ls && rm file")
        .allow(&["^ls"])
        .deny(&["^rm"])
        .is_deny();
}

// Test confirm on any command
#[test]
fn confirm_triggers_on_any_matching_command() {
    // "ls" is allowed but "sudo" requires confirm
    t("ls && sudo reboot")
        .allow(&["^ls"])
        .confirm(&["^sudo"])
        .is_confirm();
}

// Test nested command substitution
#[test]
fn nested_command_substitution_all_checked() {
    // All three commands (echo, cat, whoami) must be allowed
    t("echo $(cat $(whoami).txt)")
        .allow(&["^echo", "^cat", "^whoami"])
        .is_allow();
}

// Test that parse failures fall back safely
#[test]
fn parse_failure_falls_back_to_confirm() {
    // Invalid syntax should not auto-allow
    t("ls &&").allow(&["^ls"]).is_confirm();
}
```

## File Changes Summary

| File | Action | Description |
|------|--------|-------------|
| `Cargo.toml` (root) | Modify | Add `brush-parser` to workspace dependencies |
| `crates/agent/Cargo.toml` | Modify | Add `brush-parser` dependency |
| `crates/agent/src/shell_parser.rs` | Create | New module for shell command parsing |
| `crates/agent/src/agent.rs` | Modify | Register `shell_parser` module |
| `crates/agent/src/tool_permissions.rs` | Modify | Update permission logic to use shell parsing |

## Rollout Considerations

1. **Backward Compatibility**: Users with existing `always_allow` patterns for compound commands (e.g., `^git pull && git push`) will need to update their patterns to allow each sub-command individually. This is a security improvement, not a regression.

2. **Performance**: Shell parsing adds overhead. However, this only happens for terminal tool commands, and `brush-parser` is designed to be fast. The security benefit far outweighs the minimal performance cost.

3. **Documentation**: Update any documentation about `tool_permissions` to explain that patterns are now matched against individual sub-commands, not the entire command string.

## Success Criteria

1. All 15 shell injection tests pass
2. All existing permission tests continue to pass
3. The terminal tool correctly denies/confirms compound commands where any sub-command doesn't match the expected rules
4. Parse failures gracefully fall back to safe behavior (confirm)

## Appendix: Shell Operators Reference

| Operator | Name | Behavior |
|----------|------|----------|
| `&&` | AND | Run second command if first succeeds |
| `\|\|` | OR | Run second command if first fails |
| `;` | Sequential | Run commands in sequence |
| `&` | Background | Run first command in background, then second |
| `\|` | Pipe | Pipe stdout of first to stdin of second |
| `\|&` | Pipe all | Pipe stdout and stderr (bash-specific) |
| `\n` | Newline | Command separator |
| `` `cmd` `` | Backtick | Command substitution |
| `$(cmd)` | Dollar-paren | Command substitution |
| `<(cmd)` | Process sub (in) | Process substitution for input |
| `>(cmd)` | Process sub (out) | Process substitution for output |