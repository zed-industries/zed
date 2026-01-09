# Tool Permission System: Test Coverage Improvement Plan

## Overview

This document outlines the work needed to complete test coverage and fix implementation issues in Zed's tool permission system. The permission system allows users to configure rules that control when AI agent tools require confirmation, are automatically allowed, or are blocked entirely.

## Background

### What is the Tool Permission System?

The tool permission system lets users define rules in their settings to control agent tool behavior:

- **`always_allow`**: Patterns that, when matched, allow the tool to run without confirmation
- **`always_deny`**: Patterns that, when matched, block the tool entirely
- **`always_confirm`**: Patterns that, when matched, always require user confirmation
- **`default_mode`**: The fallback behavior when no patterns match (`Allow`, `Deny`, or `Confirm`)

The system also respects a global `always_allow_tool_actions` setting as a final fallback.

### Relevant Files

| File | Purpose |
|------|---------|
| `crates/agent/src/tool_permissions.rs` | Core permission decision logic and unit tests |
| `crates/agent/src/tests/mod.rs` | Integration tests for agent tools |
| `crates/agent/src/tools/*.rs` | Individual tool implementations |
| `crates/agent_settings/src/agent_settings.rs` | Settings schema and parsing |

### How Permission Checks Work

Each tool calls `decide_permission_from_settings(tool_name, input, settings)` which returns one of:

- `ToolPermissionDecision::Allow` - Proceed without confirmation
- `ToolPermissionDecision::Deny(reason)` - Block with error message
- `ToolPermissionDecision::Confirm` - Prompt user for confirmation

Tools then handle this decision:
1. `Allow` → Set `authorize = None`, skip confirmation
2. `Deny` → Return `Task::ready(Err(...))`immediately
3. `Confirm` → Set `authorize = Some(event_stream.authorize(...))`, await before proceeding

---

## Issues to Address

### Issue 1: Dead Code in `edit_file_tool.rs`

**Location**: `crates/agent/src/tools/edit_file_tool.rs`, approximately lines 168-170

**Problem**: After matching on the permission decision, there's a secondary check:

```rust
match decision {
    ToolPermissionDecision::Allow => return Task::ready(Ok(())),
    ToolPermissionDecision::Deny(reason) => {
        return Task::ready(Err(anyhow!("{}", reason)));
    }
    ToolPermissionDecision::Confirm => {}
}

// This check is unreachable/redundant:
if settings.always_allow_tool_actions {
    return Task::ready(Ok(()));
}
```

The secondary `always_allow_tool_actions` check is dead code because:
- `decide_permission` already incorporates `always_allow_tool_actions` into its decision
- When `Confirm` is returned, it means `always_allow_tool_actions` was already `false` OR an explicit `always_confirm` pattern matched

**Fix**: Remove lines 168-170 (the `if settings.always_allow_tool_actions` block).

---

### Issue 2: Missing Permission Checks in `copy_path_tool.rs`

**Location**: `crates/agent/src/tools/copy_path_tool.rs`

**Problem**: The `move_path_tool` was updated to check permissions for both source and destination paths, but `copy_path_tool` was not updated. These tools have identical security implications - both can write to arbitrary locations.

**Fix**: Add permission checks to `copy_path_tool.rs` following the same pattern as `move_path_tool.rs`:

1. Add imports:
   ```rust
   use crate::{
       AgentTool, ToolCallEventStream, ToolPermissionDecision, decide_permission_from_settings,
   };
   use agent_settings::AgentSettings;
   use settings::Settings;
   ```

2. In the `run` method, check both paths:
   ```rust
   let settings = AgentSettings::get_global(cx);
   
   let source_decision = decide_permission_from_settings("copy_path", &input.source_path, settings);
   if let ToolPermissionDecision::Deny(reason) = source_decision {
       return Task::ready(Err(anyhow!("{}", reason)));
   }
   
   let dest_decision = decide_permission_from_settings("copy_path", &input.destination_path, settings);
   if let ToolPermissionDecision::Deny(reason) = dest_decision {
       return Task::ready(Err(anyhow!("{}", reason)));
   }
   
   let needs_confirmation = matches!(source_decision, ToolPermissionDecision::Confirm)
       || matches!(dest_decision, ToolPermissionDecision::Confirm);
   
   let authorize = if needs_confirmation {
       let src = MarkdownInlineCode(&input.source_path);
       let dest = MarkdownInlineCode(&input.destination_path);
       Some(event_stream.authorize(format!("Copy {src} to {dest}"), cx))
   } else {
       None
   };
   ```

3. Await authorization in the async block before proceeding.

---

### Issue 3: No Integration Tests for Non-Terminal Tools

**Problem**: Integration tests only exist for the `terminal` tool, but 7 tools have permission checks:
- `terminal` ✅ (has tests)
- `create_directory` ❌
- `delete_path` ❌
- `edit_file` ❌
- `fetch` ❌
- `move_path` ❌
- `save_file` ❌
- `web_search` ❌

**Why This Matters**: Each tool has slightly different integration code. A bug in one tool's permission handling wouldn't be caught by terminal tests.

**Fix**: Add integration tests for at least one tool from each category:

#### 3a. File Modification Tool Test (`edit_file` or `save_file`)

Add to `crates/agent/src/tests/mod.rs`:

```rust
#[gpui::test]
async fn test_edit_file_tool_deny_rule_blocks_edit(cx: &mut TestAppContext) {
    // Setup: Configure deny rule for paths containing "sensitive"
    // Action: Try to edit "sensitive_config.txt"
    // Assert: Tool returns error with "blocked" message
}

#[gpui::test]
async fn test_edit_file_tool_allow_rule_skips_confirmation(cx: &mut TestAppContext) {
    // Setup: Configure allow rule for "*.md" files, always_allow_tool_actions=false
    // Action: Edit "README.md"
    // Assert: No authorization request, edit proceeds
}
```

#### 3b. Path Operation Tool Test (`delete_path` or `move_path`)

```rust
#[gpui::test]
async fn test_delete_path_tool_deny_rule_blocks_deletion(cx: &mut TestAppContext) {
    // Setup: Configure deny rule for paths containing "important"
    // Action: Try to delete "important_data.txt"
    // Assert: Tool returns error with "blocked" message
}
```

#### 3c. Network Tool Test (`fetch` or `web_search`)

```rust
#[gpui::test]
async fn test_fetch_tool_deny_rule_blocks_url(cx: &mut TestAppContext) {
    // Setup: Configure deny rule for URLs containing "internal.company.com"
    // Action: Try to fetch "https://internal.company.com/api"
    // Assert: Tool returns error with "blocked" message
}

#[gpui::test]
async fn test_fetch_tool_allow_rule_skips_confirmation(cx: &mut TestAppContext) {
    // Setup: Configure allow rule for "docs.rs", always_allow_tool_actions=false
    // Action: Fetch "https://docs.rs/some-crate"
    // Assert: No authorization request (unlike old behavior which always confirmed)
}
```

---

### Issue 4: No Tests for Multi-Path Tool Behavior

**Problem**: `move_path_tool` and `save_file_tool` check multiple paths, but no tests verify edge cases:
- What if source is allowed but destination is denied?
- What if one of multiple save paths is denied?

**Fix**: Add specific multi-path tests:

```rust
#[gpui::test]
async fn test_move_path_tool_denies_if_destination_denied(cx: &mut TestAppContext) {
    // Setup: Allow all source paths, deny paths containing "protected"
    // Action: Move "safe.txt" to "protected/safe.txt"
    // Assert: Tool returns error (destination denied)
}

#[gpui::test]
async fn test_move_path_tool_denies_if_source_denied(cx: &mut TestAppContext) {
    // Setup: Deny paths containing "secret", allow all destinations
    // Action: Move "secret.txt" to "public/not_secret.txt"
    // Assert: Tool returns error (source denied)
}

#[gpui::test]
async fn test_save_file_tool_denies_if_any_path_denied(cx: &mut TestAppContext) {
    // Setup: Deny paths containing "readonly"
    // Action: Save ["normal.txt", "readonly/config.txt"]
    // Assert: Tool returns error before saving anything
}
```

---

### Issue 5: `save_file_tool` Tests Bypass Permission System

**Location**: `crates/agent/src/tools/save_file_tool.rs`, test setup

**Problem**: The test setup sets `always_allow_tool_actions = true`, which means existing tests don't verify permission checks work at all. If someone breaks the permission integration, tests still pass.

**Fix**: Add a dedicated test that verifies permissions ARE checked:

```rust
#[gpui::test]
async fn test_save_file_tool_respects_deny_rules(cx: &mut TestAppContext) {
    // DO NOT set always_allow_tool_actions = true
    init_test(cx);
    
    // Setup deny rule
    cx.update(|cx| {
        let mut settings = AgentSettings::get_global(cx).clone();
        settings.always_allow_tool_actions = false;
        settings.tool_permissions.tools.insert(
            "save_file".into(),
            ToolRules {
                default_mode: ToolPermissionMode::Allow,
                always_deny: vec![CompiledRegex::new(r"\.secret$", false).unwrap()],
                ..Default::default()
            },
        );
        AgentSettings::override_global(settings, cx);
    });
    
    // Try to save a .secret file
    let result = /* run tool with path "config.secret" */;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("blocked"));
}
```

---

### Issue 6: Inconsistent Authorization Messages

**Problem**: Different tools format their authorization messages differently:

| Tool | Message Format |
|------|----------------|
| `create_directory` | `"Create directory {path}"` with `MarkdownInlineCode` |
| `delete_path` | `"Delete {path}"` with `MarkdownInlineCode` |
| `move_path` | `"Move {src} to {dest}"` with `MarkdownInlineCode` |
| `fetch` | Raw URL string |
| `web_search` | `"Searching the Web"` (no query shown!) |
| `save_file` | `"Save file"` or `"Save N files"` (no paths shown!) |

**Fix**: Standardize messages to include relevant context:

1. **`web_search_tool.rs`**: Change from `"Searching the Web"` to include query:
   ```rust
   Some(event_stream.authorize(
       format!("Search the web for {}", MarkdownInlineCode(&input.query)),
       cx,
   ))
   ```

2. **`save_file_tool.rs`**: Include file paths in message:
   ```rust
   let title = if input.paths.len() == 1 {
       format!("Save {}", MarkdownInlineCode(&input.paths[0].to_string_lossy()))
   } else {
       let paths: Vec<_> = input.paths.iter()
           .take(3)
           .map(|p| p.to_string_lossy().to_string())
           .collect();
       if input.paths.len() > 3 {
           format!("Save {} and {} more", paths.join(", "), input.paths.len() - 3)
       } else {
           format!("Save {}", paths.join(", "))
       }
   };
   ```

3. **`fetch_tool.rs`**: Wrap URL in `MarkdownInlineCode` for consistency:
   ```rust
   Some(event_stream.authorize(
       format!("Fetch {}", MarkdownInlineCode(&input.url)),
       cx,
   ))
   ```

---

## How to Run Tests

```bash
# Run all agent tests
cargo test -p agent -q

# Run specific permission-related tests
cargo test -p agent -q tool_permission
cargo test -p agent -q terminal_tool_deny
cargo test -p agent -q terminal_tool_allow

# Run with output to see test progress
cargo test -p agent -- --nocapture
```

---

## Checklist

**Instructions**: Check off each item as you complete it. The project is only complete when ALL items are checked.

### Implementation Fixes

- [x] **1.1** Remove dead code in `edit_file_tool.rs` (the redundant `always_allow_tool_actions` check after the match)
- [x] **1.2** Add permission checks to `copy_path_tool.rs` following the `move_path_tool` pattern
- [x] **1.3** Update `web_search_tool.rs` authorization message to include the search query
- [x] **1.4** Update `save_file_tool.rs` authorization message to include file paths
- [x] **1.5** Update `fetch_tool.rs` authorization message to use `MarkdownInlineCode`

### New Integration Tests

- [x] **2.1** Add `test_edit_file_tool_deny_rule_blocks_edit` test
- [x] **2.2** Add `test_edit_file_tool_allow_rule_skips_confirmation` test
- [x] **2.3** Add `test_delete_path_tool_deny_rule_blocks_deletion` test
- [x] **2.4** Add `test_fetch_tool_deny_rule_blocks_url` test
- [x] **2.5** Add `test_fetch_tool_allow_rule_skips_confirmation` test
- [x] **2.6** Add `test_move_path_tool_denies_if_destination_denied` test
- [x] **2.7** Add `test_move_path_tool_denies_if_source_denied` test
- [x] **2.8** Add `test_save_file_tool_denies_if_any_path_denied` test
- [x] **2.9** Add `test_save_file_tool_respects_deny_rules` test (without `always_allow_tool_actions`)
- [x] **2.10** Add `test_copy_path_tool_deny_rule_blocks_copy` test (after implementing 1.2)
- [x] **2.11** Add `test_web_search_tool_deny_rule_blocks_search` test

### Verification

- [x] **3.1** Run `./script/clippy` and fix any warnings
- [x] **3.2** Run `cargo test -p agent -q` and verify all tests pass
- [ ] **3.3** Manually verify authorization messages look correct in the UI for at least 2 tools

---

## Notes for Implementer

1. **Test Patterns**: Look at the existing terminal tool tests in `crates/agent/src/tests/mod.rs` for the pattern to follow. They use `FakeTerminalHandle` and `FakeThreadEnvironment` - you'll need similar fakes or mocks for other tools.

2. **Settings Setup**: Each test needs to configure `AgentSettings` with appropriate `tool_permissions`. See how terminal tests do this:
   ```rust
   cx.update(|cx| {
       let mut settings = AgentSettings::get_global(cx).clone();
       settings.tool_permissions.tools.insert(
           "tool_name".into(),
           ToolRules { /* ... */ },
       );
       AgentSettings::override_global(settings, cx);
   });
   ```

3. **Testing Deny vs Allow vs Confirm**:
   - **Deny**: Task completes with `Err`, error message contains "blocked" or "disabled"
   - **Allow**: No authorization event in the stream, task succeeds
   - **Confirm**: Authorization event appears in the stream

4. **Import Requirements**: For tests, you'll likely need:
   ```rust
   use agent_settings::{AgentSettings, CompiledRegex, ToolRules};
   use settings::ToolPermissionMode;
   ```

5. **Build Commands**: Always use `-q` flag with cargo as per project guidelines:
   ```bash
   cargo build -p agent -q
   cargo test -p agent -q
   ```
