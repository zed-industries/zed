# THIS FILE SHOULD NEVER BE COMMITTED TO SOURCE CONTROL!

# Implementation Plan: Addressing Review Findings for User-Defined Slash Commands

## Overview

This plan addresses the findings from the code review of the user-defined slash commands feature. The feature allows users to create custom slash commands as Markdown files that expand into templated text in Zed's agent panel.

**Target Audience:** This plan is written for developers unfamiliar with the Zed codebase.

---

## Background: Key Concepts

### GPUI Framework
Zed uses a custom UI framework called GPUI. Key concepts:
- **Entities:** State containers managed by GPUI, similar to React components with state. Created with `cx.new(|cx| ...)`.
- **Context (`cx`):** Passed to most functions, provides access to app state and async primitives.
- **`cx.spawn`:** Spawns async tasks on the foreground thread.
- **`cx.notify()`:** Tells GPUI to re-render when state changes.
- **`TestAppContext`:** Test harness that provides deterministic execution.
- **`cx.run_until_parked()`:** In tests, advances the executor until all pending work is complete. This is how we make async tests deterministic.

### FakeFs
Zed has a `FakeFs` implementation for testing filesystem operations without touching the real filesystem. It supports:
- Creating directory trees with `fs.insert_tree(path, json!({...}))`
- Creating symlinks with `fs.create_symlink()`
- Simulating file changes for watcher tests

### File Watching Pattern
File watchers in Zed return a stream of events. In tests with `FakeFs`, you can simulate file changes and then call `cx.run_until_parked()` to process the resulting events deterministically.

**IMPORTANT:** All tests must be completely deterministic. Never use real timers, sleeps, or wall-clock time. Always use GPUI's executor and `run_until_parked()` to advance simulated time.

---

## Task 1: Remove Redundant Tests

### 1.1 Remove `test_very_long_template`

**File:** `crates/agent_ui/src/user_slash_command.rs`

**Location:** Lines 1083-1091

**Rationale:** This test creates a 100,000 character string and verifies it's handled correctly. However, this doesn't test any logic specific to our implementation—it just tests that Rust's standard library handles large strings, which is already well-tested. The `expand_template` function uses standard `String` operations with no custom memory management.

**Action:** Delete the entire test function:

```rust
#[test]
fn test_very_long_template() {
    // Test that large templates work correctly
    let long_content = "x".repeat(100_000);
    let template = format!("Start: $1 {}", long_content);
    let args = vec![Cow::Borrowed("value")];
    let result = expand_template(&template, &args, "value").unwrap();
    assert!(result.starts_with("Start: value x"));
    assert_eq!(result.len(), 100_000 + 13); // "Start: " (7) + "value" (5) + " " (1) + long_content
}
```

### 1.2 Remove `test_command_load_error_includes_path_info`

**File:** `crates/agent_ui/src/user_slash_command.rs`

**Location:** Lines 1697-1707

**Rationale:** This test only verifies that the `Display` implementation for `CommandLoadError` includes the path and message. This is trivial code (a format string) that doesn't warrant its own test. The error formatting is already implicitly tested by other tests that check error messages contain specific content.

**Action:** Delete the entire test function:

```rust
#[gpui::test]
async fn test_command_load_error_includes_path_info(_cx: &mut TestAppContext) {
    // Test that CommandLoadError properly includes path information
    let error = CommandLoadError {
        path: PathBuf::from("/path/to/problematic/command.md"),
        message: "Could not read file".to_string(),
    };

    let display = error.to_string();
    assert!(display.contains("/path/to/problematic/command.md"));
    assert!(display.contains("Could not read file"));
}
```

---

## Task 2: Add Missing File Watcher Test

### 2.1 Add `test_registry_reloads_on_file_change`

**File:** `crates/agent_ui/src/user_slash_command.rs`

**Location:** Add in the "Async File Loading Tests" section, after `test_registry_updates_worktree_roots`

**Rationale:** The `SlashCommandRegistry` watches for file changes and reloads commands automatically. This behavior is currently untested.

**Pattern for Deterministic Testing:**
1. Create the registry with initial files
2. Call `cx.run_until_parked()` to let initial load complete
3. Use `FakeFs` to modify files (add/remove/change)
4. Call `cx.run_until_parked()` to let the watcher process events
5. Verify the registry state updated

**Implementation:**

```rust
#[gpui::test]
async fn test_registry_reloads_on_file_change(cx: &mut TestAppContext) {
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/project"),
        json!({
            ".zed": {
                "commands": {
                    "original.md": "Original command"
                }
            }
        }),
    )
    .await;
    let fs: Arc<dyn Fs> = fs.clone();

    let registry = cx.new(|cx| {
        SlashCommandRegistry::new(fs.clone(), vec![PathBuf::from(path!("/project"))], cx)
    });

    // Wait for initial load
    cx.run_until_parked();

    registry.read_with(cx, |registry, _cx| {
        assert_eq!(registry.commands().len(), 1);
        assert!(registry.commands().contains_key("original"));
    });

    // Add a new command file
    fs.insert_file(path!("/project/.zed/commands/new.md"), "New command".into())
        .await;

    // Wait for watcher to process the change
    cx.run_until_parked();

    registry.read_with(cx, |registry, _cx| {
        assert_eq!(registry.commands().len(), 2);
        assert!(registry.commands().contains_key("original"));
        assert!(registry.commands().contains_key("new"));
    });

    // Remove a command file
    fs.remove_file(Path::new(path!("/project/.zed/commands/original.md")), Default::default())
        .await
        .unwrap();

    // Wait for watcher to process the change
    cx.run_until_parked();

    registry.read_with(cx, |registry, _cx| {
        assert_eq!(registry.commands().len(), 1);
        assert!(!registry.commands().contains_key("original"));
        assert!(registry.commands().contains_key("new"));
    });

    // Modify an existing command
    fs.insert_file(path!("/project/.zed/commands/new.md"), "Updated content".into())
        .await;

    cx.run_until_parked();

    registry.read_with(cx, |registry, _cx| {
        let cmd = registry.commands().get("new").unwrap();
        assert_eq!(cmd.template.as_ref(), "Updated content");
    });
}
```

**Note:** You may need to check if `FakeFs` has `insert_file` and `remove_file` methods. If not, look for similar patterns in other tests in the codebase (search for `FakeFs` usage). The exact API might be:
- `fs.save(path, content, ...)` for creating/modifying files
- `fs.remove_file(path, ...)` for deletion

Search the codebase with: `grep -r "FakeFs" --include="*.rs" | grep -E "(save|remove|insert)"` to find the correct method names.

---

## Task 3: Add Missing Edge Case Tests

### 3.1 Add `test_deeply_nested_namespace`

**File:** `crates/agent_ui/src/user_slash_command.rs`

**Location:** Add in the "Async File Loading Tests" section

**Rationale:** The current tests only go 2 levels deep (`tools/git`). We should verify deeply nested namespaces work correctly.

```rust
#[gpui::test]
async fn test_deeply_nested_namespace(cx: &mut TestAppContext) {
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/commands"),
        json!({
            "a": {
                "b": {
                    "c": {
                        "d": {
                            "e": {
                                "deep.md": "Very deep command"
                            }
                        }
                    }
                }
            }
        }),
    )
    .await;
    let fs: Arc<dyn Fs> = fs;

    let result =
        load_commands_from_path_async(&fs, Path::new(path!("/commands")), CommandScope::User)
            .await;

    assert!(result.errors.is_empty());
    assert_eq!(result.commands.len(), 1);
    let cmd = &result.commands[0];
    assert_eq!(cmd.name.as_ref(), "a:b:c:d:e:deep");
    assert_eq!(
        cmd.namespace.as_ref().map(|s| s.as_ref()),
        Some("a/b/c/d/e")
    );
}
```

### 3.2 Add `test_command_name_with_emoji`

**File:** `crates/agent_ui/src/user_slash_command.rs`

**Location:** Add in the "Edge Case Tests" section

**Rationale:** Unicode is tested but emoji specifically may have different handling (multi-codepoint characters).

```rust
#[test]
fn test_command_name_with_emoji() {
    // Emoji can be multi-codepoint, test they're handled correctly
    let result = try_parse_user_command("/🚀deploy fast");
    assert!(result.is_some());
    let parsed = result.unwrap();
    assert_eq!(parsed.name, "🚀deploy");
    assert_eq!(parsed.raw_arguments, "fast");

    // Emoji in arguments
    let args = parse_arguments("🎉 \"🎊 party\"").unwrap();
    assert_eq!(args, vec!["🎉", "🎊 party"]);
}
```

### 3.3 Add `test_circular_symlink_handling`

**File:** `crates/agent_ui/src/user_slash_command.rs`

**Location:** Add in the "Symlink Handling Tests" section

**Rationale:** Circular symlinks could cause infinite loops. We need to verify the code handles them gracefully.

**Note:** First check if `FakeFs` supports creating circular symlinks. If not, this test may need to be skipped or implemented differently. Search for existing circular symlink tests in the codebase.

```rust
#[gpui::test]
async fn test_circular_symlink_handling(cx: &mut TestAppContext) {
    let fs = FakeFs::new(cx.executor());
    
    // Create a directory structure
    fs.insert_tree(
        path!("/commands"),
        json!({
            "valid.md": "Valid command"
        }),
    )
    .await;

    // Create a circular symlink: /commands/loop -> /commands
    // Note: Check if FakeFs supports this. If not, the test should verify
    // that the code doesn't hang even without explicit circular symlink handling.
    let symlink_result = fs
        .create_symlink(
            Path::new(path!("/commands/loop")),
            PathBuf::from(path!("/commands")),
        )
        .await;

    // If FakeFs doesn't support circular symlinks, that's OK - skip the symlink part
    if symlink_result.is_ok() {
        let fs: Arc<dyn Fs> = fs;

        // This should complete without hanging, even if it produces errors
        let result = load_commands_from_path_async(
            &fs,
            Path::new(path!("/commands")),
            CommandScope::User,
        )
        .await;

        // Should have loaded the valid command without hanging
        // May or may not have errors depending on implementation
        assert!(result.commands.iter().any(|c| c.name.as_ref() == "valid"));
    }
}
```

---

## Task 4: Clean Up Dead Code

### 4.1 Remove or Document `CommandSource::UserDefined`

**File:** `crates/agent_ui/src/completion_provider.rs`

**Location:** Around line 185-192

**Current Code:**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum CommandSource {
    Server,
    UserDefined { template: Arc<str> },
}
```

The `UserDefined` variant is defined but never matched against. The `source` field on `AvailableCommand` is marked `#[allow(dead_code)]`.

**Options:**

**Option A (Recommended): Add a TODO comment explaining intended use**

If this is infrastructure for planned future work (e.g., showing different icons for user vs server commands), add documentation:

```rust
/// The source of the command, used to differentiate UI behavior.
#[derive(Debug, Clone, PartialEq)]
pub enum CommandSource {
    /// Command provided by the ACP server
    Server,
    /// User-defined command from a markdown file
    /// TODO: Use this to show a different icon/style in the completion menu
    UserDefined { template: Arc<str> },
}
```

And update the `#[allow(dead_code)]` with a comment:

```rust
/// The source of the command - kept for future UI differentiation
/// TODO: Use this to show different styling for user vs server commands
#[allow(dead_code)]
pub source: CommandSource,
```

**Option B: Remove if not needed**

If there are no plans to use this, remove the `source` field entirely and simplify. However, given the implementation effort, Option A is likely better.

---

## Task 5: Refactor `MessageEditor::contents` for Clarity

### 5.1 Extract User Command Resolution Logic

**File:** `crates/agent_ui/src/acp/message_editor.rs`

**Location:** The `contents` method (around line 407-490)

**Problem:** The method has grown complex with async loading fallback logic for user commands.

**Solution:** Extract the user command loading into a separate helper function.

**Before (current structure):**
```rust
pub fn contents(
    &self,
    full_mention_content: bool,
    cached_user_commands: Option<...>,
    cx: &mut Context<Self>,
) -> Task<Result<(Vec<acp::ContentBlock>, Vec<Entity<Buffer>>)>> {
    // ... validation
    // ... complex logic to get fs and worktree_roots
    // ... spawn async task
    //     ... inside async: load commands if not cached
    //     ... expand user commands
    //     ... validate slash commands
    //     ... process mentions
}
```

**After (proposed structure):**

Add a new helper method:

```rust
/// Resolves user commands, either from cache or by loading from disk.
/// Returns a tuple of (user_commands_map, Option<fs>, worktree_roots) for async loading.
fn prepare_user_command_resolution(
    &self,
    cached_user_commands: Option<HashMap<String, UserSlashCommand>>,
    cx: &App,
) -> (
    Option<HashMap<String, UserSlashCommand>>,
    Option<Arc<dyn Fs>>,
    Vec<PathBuf>,
) {
    if let Some(cached) = cached_user_commands {
        return (Some(cached), None, Vec::new());
    }

    if !cx.has_flag::<UserSlashCommandsFeatureFlag>() {
        return (Some(HashMap::default()), None, Vec::new());
    }

    let workspace = self.workspace.upgrade();
    let fs = workspace
        .as_ref()
        .map(|w| w.read(cx).project().read(cx).fs().clone());
    let roots: Vec<PathBuf> = workspace
        .map(|workspace| {
            workspace
                .read(cx)
                .visible_worktrees(cx)
                .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
                .collect()
        })
        .unwrap_or_default();

    (None, fs, roots)
}
```

Then simplify `contents`:

```rust
pub fn contents(
    &self,
    full_mention_content: bool,
    cached_user_commands: Option<HashMap<String, UserSlashCommand>>,
    cx: &mut Context<Self>,
) -> Task<Result<(Vec<acp::ContentBlock>, Vec<Entity<Buffer>>)>> {
    let text = self.editor.read(cx).text(cx);
    let available_commands = self.available_commands.borrow().clone();
    let agent_name = self.agent_name.clone();

    let (cached_commands, fs, worktree_roots) =
        self.prepare_user_command_resolution(cached_user_commands, cx);

    let contents = self.mention_set.contents(full_mention_content, cx);
    let supports_embedded_context = self.prompt_capabilities.borrow().embedded_context;

    cx.spawn(async move |_, cx| {
        // Resolve user commands
        let user_commands = match cached_commands {
            Some(cached) => cached,
            None => {
                let Some(fs) = fs else {
                    HashMap::default()
                };
                let load_result =
                    user_slash_command::load_all_commands_async(&fs, &worktree_roots).await;
                for error in &load_result.errors {
                    log::warn!("Failed to load slash command: {}", error);
                }
                user_slash_command::commands_to_map(&load_result.commands)
            }
        };

        // Try to expand user command
        match user_slash_command::try_expand_from_commands(&text, &user_commands) {
            Ok(Some(expanded)) => return Ok((vec![expanded.into()], Vec::new())),
            Err(err) => return Err(err),
            Ok(None) => {}
        }

        // Validate server commands
        Self::validate_slash_commands(&text, &available_commands, &user_commands, &agent_name)?;

        // Process mentions (rest of existing logic)
        // ...
    })
}
```

---

## Task 6: Add Concurrent Loading Test

### 6.1 Add `test_concurrent_command_loading`

**File:** `crates/agent_ui/src/user_slash_command.rs`

**Location:** Add in the "Async File Loading Tests" section

**Rationale:** Verify that multiple simultaneous load requests don't cause issues.

```rust
#[gpui::test]
async fn test_concurrent_command_loading(cx: &mut TestAppContext) {
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/project"),
        json!({
            ".zed": {
                "commands": {
                    "cmd1.md": "Command 1",
                    "cmd2.md": "Command 2",
                    "cmd3.md": "Command 3"
                }
            }
        }),
    )
    .await;
    let fs: Arc<dyn Fs> = fs;
    let worktree_roots = vec![PathBuf::from(path!("/project"))];

    // Spawn multiple load tasks concurrently
    let fs1 = fs.clone();
    let roots1 = worktree_roots.clone();
    let task1 = cx.executor().spawn(async move {
        load_all_commands_async(&fs1, &roots1).await
    });

    let fs2 = fs.clone();
    let roots2 = worktree_roots.clone();
    let task2 = cx.executor().spawn(async move {
        load_all_commands_async(&fs2, &roots2).await
    });

    let fs3 = fs.clone();
    let roots3 = worktree_roots.clone();
    let task3 = cx.executor().spawn(async move {
        load_all_commands_async(&fs3, &roots3).await
    });

    // Wait for all tasks to complete
    let (result1, result2, result3) = futures::join!(task1, task2, task3);

    // All should succeed with the same results
    assert!(result1.errors.is_empty());
    assert!(result2.errors.is_empty());
    assert!(result3.errors.is_empty());

    assert_eq!(result1.commands.len(), 3);
    assert_eq!(result2.commands.len(), 3);
    assert_eq!(result3.commands.len(), 3);
}
```

**Note:** Check how other tests in the codebase spawn background tasks. The pattern might be `cx.background_executor().spawn(...)` instead. Search for similar patterns.

---

## Task 7: Fix Minor Code Issues

### 7.1 Update Error Dismissal to Clear Errors on Reload

**File:** `crates/agent_ui/src/acp/thread_view.rs`

**Location:** The `clear_command_load_errors` method and the subscription handler

**Problem:** When errors are dismissed, they're just hidden. If commands are reloaded and succeed, old errors might still be "dismissed" even though there are no errors.

**Solution:** Reset the dismissed flag when new errors arrive:

```rust
// In the subscription handler (around line 520-530):
cx.subscribe(&registry, move |this, registry, event, cx| match event {
    SlashCommandRegistryEvent::CommandsChanged => {
        this.command_load_errors = registry.read(cx).errors().to_vec();
        // Reset dismissed state when errors change
        this.command_load_errors_dismissed = false;
        *cached_user_commands_for_subscription.borrow_mut() =
            registry.read(cx).commands().clone();
        cx.notify();
    }
})
.detach();
```

This ensures that if new errors appear after a reload, they'll be shown even if the user previously dismissed errors.

---

## Summary Checklist

| Task | Description | Priority |
|------|-------------|----------|
| 1.1 | Remove `test_very_long_template` | Low |
| 1.2 | Remove `test_command_load_error_includes_path_info` | Low |
| 2.1 | Add `test_registry_reloads_on_file_change` | High |
| 3.1 | Add `test_deeply_nested_namespace` | Medium |
| 3.2 | Add `test_command_name_with_emoji` | Medium |
| 3.3 | Add `test_circular_symlink_handling` | Medium |
| 4.1 | Document or remove `CommandSource::UserDefined` | Low |
| 5.1 | Refactor `MessageEditor::contents` | Medium |
| 6.1 | Add `test_concurrent_command_loading` | Medium |
| 7.1 | Fix error dismissal on reload | Low |

**Estimated Total Effort:** 2-4 hours for an engineer familiar with the codebase, 4-8 hours for someone new.

---

## Testing Your Changes

After making changes, run the tests with:

```bash
cargo test -p agent_ui user_slash_command
```

For the full test suite:

```bash
cargo test -p agent_ui
```

Use the project's clippy script for linting:

```bash
./script/clippy
```

---

## Key Principles for Tests

1. **All tests must be deterministic.** Never use real timers, `sleep`, or wall-clock time.
2. **Use `cx.run_until_parked()`** to advance the GPUI executor until all pending work completes.
3. **Use `FakeFs`** for all filesystem operations in tests.
4. **Use `cx.executor()` timers** (e.g., `cx.background_executor().timer(duration).await`) instead of `smol::Timer::after(...)` when you need delays in tests.
5. **Follow existing patterns** in the codebase - search for similar tests when unsure about APIs.