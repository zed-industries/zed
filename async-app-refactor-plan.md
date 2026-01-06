# AsyncApp Result Removal Refactor

## The Change

`AsyncApp` methods no longer return `Result` - task cancellation now happens at the executor level.

## What Changed

| Method | Before | After |
|--------|--------|-------|
| `cx.update(\|app\| ...)` | `Result<T>` | `T` |
| `cx.new(...)` | `Result<Entity<T>>` | `Entity<T>` |
| `cx.update_entity(&entity, ...)` | `Result<T>` | `T` |
| `cx.read_entity(&entity, ...)` | `Result<T>` | `T` |
| `cx.read_global(...)` | `Result<T>` | `T` |
| `cx.update_global(...)` | `Result<T>` | `T` |
| `Entity<T>.update(&mut cx, ...)` | `Result<T>` | `T` |
| `Entity<T>.read_with(&cx, ...)` | `Result<T>` | `T` |

## What Still Returns Result

| Method | Reason |
|--------|--------|
| `WeakEntity::update(cx, ...)` | Entity might be dropped |
| `WeakEntity::read_with(cx, ...)` | Entity might be dropped |
| `cx.update_window(...)` | Window might be closed |
| `oneshot::Receiver<T>.await` | Channel might be closed |

---

## Deterministic Pattern: `&mut cx` vs `cx`

**Key insight**: The calling convention distinguishes Entity from WeakEntity:

| Pattern | Context | Entity Type | Transform? |
|---------|---------|-------------|------------|
| `this.update(&mut cx, ...)` | Handler functions | `Entity<T>` | ✅ Remove `?` |
| `this.update(cx, ...)` | Spawn closures | `WeakEntity<T>` | ❌ Keep `?` |
| `this.read_with(&cx, ...)` | Handler functions | `Entity<T>` | ✅ Remove `?` |
| `this.read_with(cx, ...)` | Spawn closures | `WeakEntity<T>` | ❌ Keep `?` |

**Why?** Handler functions receive `mut cx: AsyncApp` and call `.update(&mut cx, ...)`.
Spawn closures receive `cx: AsyncApp` (no mut) and call `.update(cx, ...)`.

---

## ast-grep Rules (Safe & Deterministic)

### Phase 1: Direct AsyncApp calls

```bash
# cx.update(...)?  →  cx.update(...)
sg run --pattern 'cx.update($$$ARGS)?' --rewrite 'cx.update($$$ARGS)' --lang rust crates/project -U

# cx.new(...)?  →  cx.new(...)
sg run --pattern 'cx.new($$$ARGS)?' --rewrite 'cx.new($$$ARGS)' --lang rust crates/project -U

# cx.read_global(...)?  →  cx.read_global(...)
sg run --pattern 'cx.read_global($$$ARGS)?' --rewrite 'cx.read_global($$$ARGS)' --lang rust crates/project -U

# cx.update_global(...)?  →  cx.update_global(...)
sg run --pattern 'cx.update_global($$$ARGS)?' --rewrite 'cx.update_global($$$ARGS)' --lang rust crates/project -U
```

### Phase 2: Entity calls (with &mut cx - handler pattern)

```bash
# this.update(&mut cx, ...)?  →  this.update(&mut cx, ...)
sg run --pattern 'this.update(&mut cx, $$$ARGS)?' --rewrite 'this.update(&mut cx, $$$ARGS)' --lang rust crates/project -U

# this.read_with(&cx, ...)?  →  this.read_with(&cx, ...)
sg run --pattern 'this.read_with(&cx, $$$ARGS)?' --rewrite 'this.read_with(&cx, $$$ARGS)' --lang rust crates/project -U
```

### Phase 3: Double-? patterns (closure returns Result)

```bash
# })??  →  })?   (for Entity::update where closure returns Result)
sg run --pattern 'this.update(&mut cx, $$$ARGS)??' --rewrite 'this.update(&mut cx, $$$ARGS)?' --lang rust crates/project -U

# Same for read_with
sg run --pattern 'this.read_with(&cx, $$$ARGS)??' --rewrite 'this.read_with(&cx, $$$ARGS)?' --lang rust crates/project -U
```

### Phase 4: Remove .ok()/.log_err()/.unwrap() wrappers

```bash
# cx.update(...).unwrap()  →  cx.update(...)
sg run --pattern 'cx.update($$$ARGS).unwrap()' --rewrite 'cx.update($$$ARGS)' --lang rust crates/project -U

# this.update(&mut cx, ...).ok()  →  this.update(&mut cx, ...)
sg run --pattern 'this.update(&mut cx, $$$ARGS).ok()' --rewrite 'this.update(&mut cx, $$$ARGS)' --lang rust crates/project -U
```

---

## DO NOT Transform (WeakEntity patterns)

These patterns must KEEP their `?` or `.ok()`:

```bash
# ❌ DO NOT transform - WeakEntity in spawn closures
this.update(cx, ...)?      # Note: cx without &mut
this.read_with(cx, ...)?   # Note: cx without &

# ❌ DO NOT use generic patterns
$VAR.update($$$)?          # Matches both Entity and WeakEntity!
```

---

## Manual Fixes Required After ast-grep

### 1. Function returns `Result<()>` but update now returns `()`

```rust
// Error: expected Result<()>, found ()
async fn handler(...) -> Result<()> {
    this.update(&mut cx, |this, cx| { ... })  // Returns ()
}

// Fix: Wrap in Ok()
async fn handler(...) -> Result<()> {
    Ok(this.update(&mut cx, |this, cx| { ... }))
}
```

### 2. Task returns need `.await`

```rust
// Error: ? cannot be applied to Task<Result<...>>
this.update(&mut cx, |this, cx| this.save_buffer(buffer, cx))?

// Fix: Add .await
this.update(&mut cx, |this, cx| this.save_buffer(buffer, cx)).await?
```

### 3. Type annotations after removing `?`

```rust
// Error: type annotations needed
let (a, b) = this.read_with(&cx, |this, _| { ... });

// Fix: Add explicit types
let (a, b): (Entity<Buffer>, u64) = this.read_with(&cx, |this, _| { ... });
```

### 4. Keep `??` for oneshot::Receiver

```rust
// These return Receiver<Result<T>>, need BOTH ? operators
this.update(&mut cx, |this, cx| this.fetch(cx)).await??  // KEEP ??
```

---

## Execution Order

1. Reset any partial changes: `git checkout crates/project`
2. Run Phase 1-4 ast-grep rules
3. Run `cargo check -p project 2>&1 | head -100` to see remaining errors
4. Fix manual patterns (Result<()> wrappers, .await, type annotations)
5. Verify with `cargo check -p project`

---

## Crate Verification Status

Legend: ✅ = check passes, 🧪 = tests pass, ⏳ = pending, ❌ = failing

### Depth 0 (zed binary)

| Crate | Check | Tests |
|-------|-------|-------|
| zed | ✅ | ⏳ |

### Depth 1 (direct dependencies of zed)

| Crate | Check | Tests |
|-------|-------|-------|
| activity_indicator | ✅ | ⏳ |
| agent_ui | ✅ | ⏳ |
| askpass | ✅ | 🧪 |
| assets | ✅ | 🧪 |
| audio | ❌ (livekit-protocol dep issue, unrelated) | ⏳ |
| auto_update | ✅ | ⏳ |
| breadcrumbs | ✅ | ⏳ |
| channel | ✅ | ⏳ |
| cli | ✅ | 🧪 |
| client | ✅ | ⏳ |
| collab_ui | ✅ | ⏳ |
| collections | ✅ | 🧪 |
| command_palette | ✅ | ⏳ |
| component | ✅ | 🧪 |
| copilot | ✅ | ⏳ |
| dap | ✅ | ⏳ |
| dap_adapters | ✅ | ⏳ |
| db | ✅ | 🧪 |
| debugger_ui | ✅ | ⏳ |
| diagnostics | ✅ | ⏳ |
| edit_prediction | ✅ | ⏳ |
| editor | ✅ | ⏳ |
| extension | ✅ | ⏳ |
| extension_host | ✅ | ⏳ |
| feature_flags | ✅ | 🧪 |
| file_finder | ✅ | ⏳ |
| fs | ✅ | 🧪 |
| git | ✅ | 🧪 |
| git_hosting_providers | ✅ | 🧪 |
| git_ui | ✅ | ⏳ |
| go_to_line | ✅ | ⏳ |
| gpui | ✅ | 🧪 |
| gpui_tokio | ✅ | 🧪 |
| http_client | ✅ | 🧪 |
| language | ✅ | 🧪 |
| language_model | ✅ | ⏳ |
| language_models | ✅ | ⏳ |
| markdown | ✅ | ⏳ |
| markdown_preview | ✅ | ⏳ |
| menu | ✅ | 🧪 |
| migrator | ✅ | 🧪 |
| node_runtime | ✅ | 🧪 |
| notifications | ✅ | ⏳ |
| outline_panel | ✅ | ⏳ |
| paths | ✅ | 🧪 |
| picker | ✅ | ⏳ |
| project | ✅ | 🧪 |
| prompt_store | ✅ (fixed: .and_then → .map) | ⏳ |
| proto | ✅ | 🧪 |
| recent_projects | ✅ | ⏳ |
| release_channel | ✅ | 🧪 |
| remote | ✅ | 🧪 |
| repl | ✅ | ⏳ |
| reqwest_client | ✅ | 🧪 |
| rope | ✅ | 🧪 |
| search | ✅ | ⏳ |
| session | ✅ | 🧪 |
| settings | ✅ | 🧪 |
| snippet_provider | ✅ | ⏳ |
| supermaven | ✅ | ⏳ |
| task | ✅ | 🧪 |
| tasks_ui | ✅ | ⏳ |
| telemetry | ✅ | 🧪 |
| terminal_view | ✅ | ⏳ |
| theme | ✅ | 🧪 |
| title_bar | ✅ | ⏳ |
| ui | ✅ | ⏳ |
| util | ✅ | 🧪 |
| vim | ✅ | ⏳ |
| vim_mode_setting | ✅ | 🧪 |
| watch | ✅ | 🧪 |
| web_search | ✅ | 🧪 |
| workspace | ✅ | ⏳ |
| zed_actions | ✅ | 🧪 |
| zed_env_vars | ✅ | 🧪 |
| zlog | ✅ | 🧪 |
| zlog_settings | ✅ | 🧪 |
| ztracing | ✅ | 🧪 |

### Depth 2

| Crate | Check | Tests |
|-------|-------|-------|
| acp_thread | ✅ | ⏳ |
| action_log | ✅ | ⏳ |
| agent | ✅ | ⏳ |
| agent_servers | ✅ | ⏳ |
| anthropic | ✅ | 🧪 |
| assistant_slash_command | ✅ | ⏳ |
| assistant_slash_commands | ✅ | ⏳ |
| assistant_text_thread | ✅ | ⏳ |
| aws_http_client | ✅ | 🧪 |
| buffer_diff | ✅ | 🧪 |
| clock | ✅ | 🧪 (needs --features test-support) |
| cloud_api_client | ✅ | 🧪 |
| cloud_api_types | ✅ | 🧪 |
| cloud_llm_client | ✅ | 🧪 |
| command_palette_hooks | ✅ | ⏳ |
| context_server | ✅ | ⏳ |
| credentials_provider | ✅ | 🧪 |
| denoise | ✅ | 🧪 |
| edit_prediction_types | ✅ | ⏳ |
| eval_utils | ✅ | 🧪 |
| file_icons | ✅ | 🧪 |
| fsevent | ✅ | 🧪 |
| fuzzy | ✅ | 🧪 |
| google_ai | ✅ | 🧪 |
| gpui_macros | ✅ | 🧪 |
| html_to_markdown | ✅ | 🧪 |
| http_client_tls | ✅ | 🧪 |
| icons | ✅ | 🧪 |
| livekit_client | ✅ | ⏳ |
| lsp | ✅ | 🧪 |
| media | ✅ | 🧪 |
| mistral | ✅ | 🧪 |
| multi_buffer | ✅ | ⏳ |
| net | ✅ | 🧪 |
| ollama | ✅ | 🧪 |
| open_ai | ✅ | 🧪 |
| open_router | ✅ | 🧪 |
| perf | ✅ | 🧪 |
| prettier | ✅ | 🧪 |
| refineable | ✅ | 🧪 |
| rpc | ✅ | 🧪 |
| settings_json | ✅ | 🧪 |
| settings_macros | ✅ | 🧪 |
| snippet | ✅ | 🧪 |
| sqlez | ✅ | 🧪 |
| sqlez_macros | ✅ | 🧪 |
| story | ✅ | 🧪 |
| streaming_diff | ✅ | 🧪 |
| sum_tree | ✅ | 🧪 |
| supermaven_api | ✅ | 🧪 |
| telemetry_events | ✅ | 🧪 |
| terminal | ✅ | 🧪 |
| text | ✅ | 🧪 |
| time_format | ✅ | 🧪 |
| ui_macros | ✅ | 🧪 |
| util_macros | ✅ | 🧪 |
| worktree | ✅ | ⏳ |
| ztracing_macro | ✅ | 🧪 |

### Depth 3

| Crate | Check | Tests |
|-------|-------|-------|
| derive_refineable | ✅ | 🧪 |
| livekit_api | ✅ | ⏳ |
