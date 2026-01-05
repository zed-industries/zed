# AsyncApp Result Removal Refactor - Execution Plan

## Process (FOLLOW THIS STRICTLY)

For each pattern, in each crate:

1. **Pre-flight check** - Identify WeakEntity usage (see below)
2. **Look** at specific examples in ONE crate
3. **Write/refine** an ast-grep rule for that pattern class
4. **Run** the rule on that crate with `-U` to apply changes
5. **Verify** with `cargo check -p <crate>` (must pass with NO WARNINGS OR ERRORS)
6. **If success**: Add to staging area and move to next crate
7. **If failure**: `git checkout crates/<crate>` to revert, fix the rule, try again

---

## ⚠️ CRITICAL: AsyncApp vs WeakEntity Distinction

**This is the #1 source of errors in this refactor.**

### Methods that NO LONGER return Result (safe to remove `?`):

These are `AsyncApp` methods where `cx` is the async context:
- `cx.update(|app| ...)` → returns `R` directly
- `cx.new(...)` → returns `Entity<T>` directly
- `cx.update_entity(&entity, ...)` → returns `R` directly
- `cx.read_entity(&entity, ...)` → returns `R` directly
- `cx.read_global(...)` → returns `R` directly
- `cx.update_global(...)` → returns `R` directly
- `cx.subscribe(...)` → returns `Subscription` directly
- `cx.refresh()` → returns `()` directly

### Methods that STILL return Result (DO NOT remove `?`):

**WeakEntity methods** - entity might be dropped:
- `weak_entity.update(cx, |this, cx| ...)?` - KEEP the `?`
- `weak_entity.read_with(cx, |this, app| ...)?` - KEEP the `?`
- `weak_entity.update_in(cx, |this, window, cx| ...)?` - KEEP the `?`

**Window methods** - window might be closed:
- `cx.update_window(handle, ...)?` - KEEP the `?`
- `cx.read_window(&handle, ...)?` - KEEP the `?`
- `window_handle.update(cx, ...)?` - KEEP the `?`
- `AsyncWindowContext::update(...)?` - KEEP the `?`

### How to Identify WeakEntity in Spawn Closures

In `cx.spawn(async move |this, cx| ...)` called from a `Context<T>`:
- `cx` is `AsyncApp` → its methods NO LONGER return Result
- `this` is `WeakEntity<T>` → its methods STILL return Result

**Example of CORRECT handling:**
```rust
cx.spawn(async move |this, cx| {
    // cx.update() - AsyncApp method, no Result
    let value = cx.update(|app| app.do_something());

    // this.update() - WeakEntity method, STILL returns Result
    this.update(cx, |this, cx| {
        this.value = value;
        cx.notify();
    })?;  // <-- Keep the ? here!

    Ok(())
})
```

---

## Pre-Flight Check (Run BEFORE applying patterns)

```bash
# Check for spawn closures with WeakEntity pattern
grep -n "cx.spawn.*|this.*cx|" crates/<CRATE>/src/*.rs

# Check for WeakEntity::update calls
grep -n "this.update(cx\|this.update(&mut cx\|this.read_with" crates/<CRATE>/src/*.rs

# If either returns matches, MANUAL REVIEW is required
# Do NOT blindly apply ast-grep patterns
```

---

## Current Target

**Crate**: Depth 2+ crates remaining
**Pattern**: Various patterns
**Status**: Depth 0 and Depth 1 partially complete, some crates need redo

---

## Pattern Classes (in order of application)

### Pattern 1: Remove `?` from `cx.update(...)?`

**⚠️ ONLY apply when `cx` is AsyncApp, NOT when receiver is WeakEntity**

```yaml
# ast-grep rule
id: remove-cx-update-question-mark
language: rust
rule:
  pattern: $CX.update($$$ARGS)?
fix: $CX.update($$$ARGS)
```

**Command:**
```bash
sg run --pattern '$CX.update($$$ARGS)?' --rewrite '$CX.update($$$ARGS)' --lang rust crates/<CRATE> -U
```

**After applying, manually check for WeakEntity calls that were incorrectly modified!**

### Pattern 2: Remove `.ok().flatten()` from `cx.update(...).ok().flatten()`

```yaml
id: remove-cx-update-ok-flatten
language: rust
rule:
  pattern: $CX.update($$$ARGS).ok().flatten()
fix: $CX.update($$$ARGS)
```

**Command:**
```bash
sg run --pattern '$CX.update($$$ARGS).ok().flatten()' --rewrite '$CX.update($$$ARGS)' --lang rust crates/<CRATE> -U
```

### Pattern 3: Remove `.unwrap()` from `cx.update(...).unwrap()`

```yaml
id: remove-cx-update-unwrap
language: rust
rule:
  pattern: $CX.update($$$ARGS).unwrap()
fix: $CX.update($$$ARGS)
```

**Command:**
```bash
sg run --pattern '$CX.update($$$ARGS).unwrap()' --rewrite '$CX.update($$$ARGS)' --lang rust crates/<CRATE> -U
```

### Pattern 4: Replace `.ok()` with `Some(...)`

**⚠️ MANUAL REVIEW REQUIRED** - Sometimes you want `Some()`, sometimes just the value.

```bash
# Search only, do NOT auto-apply
sg run --pattern '$CX.update($$$ARGS).ok()' --lang rust crates/<CRATE>
```

### Pattern 5: Other AsyncApp methods

Apply same patterns to these methods:
- `cx.new($$$)?` → `cx.new($$$)`
- `cx.update_entity($$$)?` → `cx.update_entity($$$)`
- `cx.read_entity($$$)?` → `cx.read_entity($$$)`
- `cx.read_global($$$)?` → `cx.read_global($$$)`
- `cx.update_global($$$)?` → `cx.update_global($$$)`
- `cx.subscribe($$$)?` → `cx.subscribe($$$)`
- `cx.refresh()?` → `cx.refresh()`

### Pattern 6: `C::Result<T>` return types

**⚠️ MANUAL FIX REQUIRED** - Change return types from `C::Result<T>` to `T`.

```bash
# Search
grep -rn "::Result<" --include="*.rs" crates/<CRATE> | grep -v "anyhow::Result"
```

---

## Crate Processing Order

Work through crates in **dependency-graph order** (leaf crates first).
This ensures foundational crates are fixed before their dependents.

### Depth 0 (No gpui-dependent deps)
- [x] `gpui_tokio` - Pattern 6 ✅
- [x] `askpass` - No changes needed ✅
- [x] `assets` - No changes needed ✅
- [x] `eval_utils` - No changes needed ✅
- [x] `feature_flags` - No changes needed ✅
- [x] `fuzzy` - No changes needed ✅
- [x] `gpui_macros` - Removed Self::Result<T> from derive macros ✅
- [x] `menu` - No changes needed ✅
- [x] `release_channel` - No changes needed ✅
- [x] `reqwest_client` - No changes needed ✅
- [x] `rope` - No changes needed ✅
- [x] `rpc` - No changes needed ✅
- [x] `story` - No changes needed ✅
- [x] `watch` - No changes needed ✅
- [x] `web_search` - No changes needed ✅
- [x] `zed_actions` - No changes needed ✅
- [x] `zed_env_vars` - No changes needed ✅

### Depth 1
- [x] `credentials_provider` - Pattern 1 ✅
- [x] `session` - Pattern 2 ✅
- [x] `audio` - Fixed try_read_default_global → read_default_global ✅
- [x] `cloud_api_client` - No changes needed ✅
- [x] `command_palette_hooks` - No changes needed ✅
- [x] `component` - No changes needed ✅
- [x] `file_icons` - No changes needed ✅
- [x] `fs_benchmarks` - No changes needed ✅
- [x] `lsp` - No changes needed ✅
- [x] `task` - No changes needed ✅
- [x] `text` - No changes needed ✅
- [x] `theme_importer` - No changes needed ✅
- [x] `zlog_settings` - No changes needed ✅

### Depth 2
- [x] `context_server` - Removed ? from cx.update, fixed handler call ✅
- [x] `db` - No changes needed ✅
- [x] `miniprofiler_ui` - No changes needed (uses WeakEntity::update which still returns Result) ✅
- [x] `system_specs` - No changes needed ✅

### Depth 3
- [x] `settings` - Wrapped read_global calls in Ok() ✅
- [x] `debug_adapter_extension` - No changes needed ✅
- [x] `edit_prediction_types` - No changes needed ✅
- [x] `git_hosting_providers` - No changes needed ✅
- [x] `install_cli` - Removed ? from cx.update calls ✅
- [x] `rich_text` - No changes needed ✅
- [x] `snippet_provider` - No changes needed (uses WeakEntity::update which still returns Result) ✅
- [x] `theme_extension` - No changes needed ✅
- [x] `web_search_providers` - No changes needed ✅
- [x] `worktree_benchmarks` - Removed .unwrap() from Entity::update and read_with calls ✅

### Depth 4
- [x] `git` - Pattern 2 ✅
- [ ] `feedback`
- [ ] `fs`
- [ ] `journal`
- [ ] `language_model`
- [x] `livekit_client` - Removed ? from Tokio::spawn calls ✅
- [x] `theme` - Removed Result handling from cx.update ✅

### Depth 5
- [ ] `ai_onboarding`
- [ ] `auto_update`
- [ ] `codestral`
- [ ] `debugger_tools`
- [ ] `language_extension`
- [ ] `language_onboarding`
- [ ] `panel`
- [x] `prettier` - Removed ? from cx.update calls ✅
- [ ] `project_benchmarks`
- [ ] `svg_preview`
- [ ] `terminal`
- [ ] `which_key`

### Depth 6
- [x] `remote` - Removed ? from cx.update, cx.update_global, cx.new ✅
- [ ] `agent_settings`
- [ ] `assistant_slash_command`
- [ ] `auto_update_ui`
- [x] `buffer_diff` - Removed ? from cx.update and this.read_with, removed .log_err() ✅
- [ ] `extension`
- [ ] `extension_cli`
- [ ] `line_ending_selector`
- [ ] `picker`
- [ ] `prompt_store`
- [ ] `ui`
- [ ] `ui_input`
- [ ] `ui_prompt`

### Depth 7-10
- [ ] `acp_tools`
- [x] `dap` - Removed .with_context().unwrap_or() from cx.update calls ✅
- [ ] `markdown`
- [ ] `breadcrumbs`
- [ ] `dap_adapters`
- [ ] `edit_prediction_context`
- [ ] `json_schema_store`
- [ ] `snippets_ui`
- [ ] `theme_selector`
- [ ] `channel`
- [ ] `markdown_preview`
- [ ] `toolchain_selector`
- [ ] `worktree`
- [ ] `action_log`
- [ ] `activity_indicator`
- [ ] `image_viewer`
- [ ] `inspector_ui`

### Depth 11-15
- [ ] `language_selector`
- [ ] `multi_buffer`
- [ ] `notifications`
- [ ] `supermaven`
- [ ] `tab_switcher`
- [x] `client`
- [x] `worktree`
- [ ] `language`
- [ ] `languages`
- [ ] `storybook`
- [ ] `call`
- [ ] `go_to_line`
- [ ] `language_tools`
- [ ] `rules_library`
- [ ] `search`
- [ ] `settings_profile_selector`
- [ ] `component_preview`
- [ ] `onboarding`
- [ ] `outline`
- [ ] `project_symbols`
- [ ] `tasks_ui`
- [ ] `outline_panel`

### Depth 16-25
- [ ] `acp_thread`
- [ ] `assistant_slash_commands`
- [ ] `extensions_ui`
- [ ] `file_finder`
- [ ] `agent_ui_v2`
- [ ] `command_palette`
- [ ] `diagnostics`
- [ ] `terminal_view`
- [ ] `assistant_text_thread`
- [ ] `edit_prediction_cli`
- [ ] `extension_host`
- [ ] `language_models`
- [ ] `copilot`
- [ ] `agent_servers`
- [ ] `keymap_editor`
- [ ] `project_panel`
- [ ] `title_bar`
- [ ] `eval`
- [ ] `edit_prediction`
- [ ] `repl`
- [ ] `settings_ui`

### Depth 26-40
- [ ] `collab_ui`
- [ ] `workspace`
- [ ] `recent_projects`
- [ ] `edit_prediction_ui`
- [ ] `vim`
- [ ] `debugger_ui`
- [ ] `git_ui`
- [ ] `editor`
- [ ] `project`
- [ ] `remote_server`
- [ ] `agent`

### Depth 41+
- [ ] `collab`
- [ ] `agent_ui`
- [ ] `zed`

### Skipped (No changes needed)
- (add crates here as you verify they have no matching patterns)

---

## Workflow Commands

### Step 0: Pre-flight check
```bash
# Check for WeakEntity patterns that need manual handling
grep -rn "\.spawn.*|this.*cx|" crates/<CRATE>/src/
grep -rn "this\.update(cx\|this\.update(&mut cx\|this\.read_with" crates/<CRATE>/src/
```

### Step 1: Search for patterns in a crate
```bash
# Pattern 1: ?
sg run --pattern '$CX.update($$$ARGS)?' --lang rust crates/<CRATE>

# Pattern 2: .ok().flatten()
sg run --pattern '$CX.update($$$ARGS).ok().flatten()' --lang rust crates/<CRATE>

# Pattern 3: .unwrap()
sg run --pattern '$CX.update($$$ARGS).unwrap()' --lang rust crates/<CRATE>

# Pattern 4: .ok()
sg run --pattern '$CX.update($$$ARGS).ok()' --lang rust crates/<CRATE>
```

### Step 2: Apply a fix
```bash
sg run --pattern '<PATTERN>' --rewrite '<REWRITE>' --lang rust crates/<CRATE> -U
```

### Step 3: Verify (MUST PASS WITH NO WARNINGS)
```bash
cargo check -p <CRATE> 2>&1 | grep -E "(^error|^warning)"
```

### Step 4a: If success, commit
```bash
git add crates/<CRATE>
git commit -m "refactor(<CRATE>): remove Result from AsyncApp methods"
```

### Step 4b: If failure, revert and retry
```bash
git checkout crates/<CRATE>
# Fix the rule or handle manually, then try again
```

---

## Common Mistakes to Avoid

### Mistake 1: Removing `?` from WeakEntity::update
```rust
// WRONG - this.update returns Result because this is WeakEntity
this.update(cx, |this, cx| { ... });  // Missing ?

// CORRECT
this.update(cx, |this, cx| { ... })?;
```

### Mistake 2: Removing `.await` from closures returning futures
```rust
// WRONG - clear_contacts() returns a Future
this.update(cx, |this, cx| {
    this.clear_contacts()
}).ok();

// CORRECT
this.update(cx, |this, cx| {
    this.clear_contacts()
})?.await;
```

### Mistake 3: Double `?` confusion
```rust
// Old code (when AsyncApp::update returned Result):
cx.update(|cx| {
    this.update(cx, |this, cx| { ... })  // WeakEntity returns Result
})??;  // Two ? because both returned Result

// New code (AsyncApp::update returns R directly):
cx.update(|cx| {
    this.update(cx, |this, cx| { ... })?  // Still need ? for WeakEntity
});  // No outer ? because cx.update no longer returns Result
```

---

## Reference: What Still Returns Result (DO NOT CHANGE)

- `WeakEntity::update(cx, ...)` - entity might be dropped
- `WeakEntity::read_with(cx, ...)` - entity might be dropped
- `WeakEntity::update_in(cx, ...)` - entity might be dropped
- `AsyncWindowContext::update(...)` - window might be closed
- `AsyncWindowContext::update_root(...)` - window might be closed
- `cx.update_window(...)` - window might be closed
- `cx.read_window(...)` - window might be closed
- `window_handle.update(cx, ...)` - window might be closed
- `cx.open_window(...)` - platform error possible

---

## Reference: What No Longer Returns Result

- `cx.update(|app| ...)` → `T` (was `Result<T>`)
- `cx.new(...)` → `Entity<T>` (was `Result<Entity<T>>`)
- `cx.reserve_entity()` → `Reservation<T>` (was `Result<Reservation<T>>`)
- `cx.insert_entity(...)` → `Entity<T>` (was `Result<Entity<T>>`)
- `cx.update_entity(&entity, ...)` → `T` (was `Result<T>`)
- `cx.read_entity(&entity, ...)` → `T` (was `Result<T>`)
- `cx.read_global(...)` → `T` (was `Result<T>`)
- `cx.update_global(...)` → `T` (was `Result<T>`)
- `cx.has_global::<T>()` → `bool` (was `Result<bool>`)
- `cx.subscribe(...)` → `Subscription` (was `Result<Subscription>`)
- `cx.refresh()` → `()` (was `Result<()>`)

---

## Branch Info

- **Branch**: `actually-remove-the-app`
- **Clean state command**: `git status` (should show clean before starting each crate)
