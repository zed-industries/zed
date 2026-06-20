# RULES.md

Task-specific rules for agents working in the Zed codebase. Read sections relevant to your current task only.

## §0: Agent SOP

The plan → delegate → review loop for code changes.

### Step 1: Analyze & Plan (jcodemunch)

1. `plan_turn(repo="zed", query="<task>")` — opening move. Returns confidence + recommended symbols.
2. `search_symbols` / `get_file_outline` — locate exact symbols and API surfaces.
3. `get_blast_radius(symbol="...", depth=2)` — understand downstream impact before planning.
4. `get_hotspots` / `find_dead_code` — identify risk areas.
5. Break into smallest incremental steps.

### Step 2: Delegate

Use `spawn_agent` for all code modifications. Every delegation prompt MUST include:

1. **Repo identifier**: `"zed"`
2. **Target symbol_ids** — the subagent retrieves these via `get_symbol_source`, not full-file reads
3. **jcodemunch mandate**: subagent must use `get_file_outline` before reading, `search_symbols` / `get_symbol_source` instead of reading whole files
4. **Token budget** when using `get_ranked_context` or `get_context_bundle` (default 4000)
5. **Full context** — `spawn_agent` is stateless; include everything the subagent needs

Delegation preamble template:

```
You are working in repo "zed" (indexed via jcodemunch-mcp).
Mandatory: use jcodemunch tools for ALL code lookup. Never read a full file.
- get_file_outline before pulling source
- search_symbols / get_symbol_source for targeted retrieval
- Batch with symbol_ids[] instead of repeated calls
- get_ranked_context(query="...", token_budget=4000) for task-driven context

Target symbols: <list symbol_ids>
```

Delegate ONE step per `spawn_agent` call. Never bundle multiple steps. If a step can be parallelized, include "fan out subagents" in the prompt so the subagent can decompose further.

If you are the subagent (receiving a delegated task): do the work directly. Do not recursively spawn unless explicitly told to "fan out".

### Step 3: Review

After the subagent returns, verify with jcodemunch:

- `get_blast_radius(symbol="...", include_source=true)` — impact matches expectations?
- `find_references(identifier="...")` — no call site broken?
- `get_call_hierarchy(symbol_id="...", direction="callers")` — upstream dependents intact?
- `get_symbol_source(symbol_id="...", verify=true)` — indexed source matches what was written?
- `register_edit(file_path="...", reindex=true)` — refresh the index after edits
- Run the relevant test: `cargo test -p <crate>`
- Run `./script/clippy` if touching public APIs

### Step 4: Iterate

- **Approved**: move to the next step (return to Step 2)
- **Revision needed**: call `spawn_agent` again with repo id, the affected symbol_ids, and corrective feedback. Instruct the subagent to `get_symbol_source` to re-read current state before fixing.

## §1: Rust/GPUI Coding Guidelines

### Error handling

- Never `unwrap()` in production code — use `?` to propagate
- Never `let _ =` on fallible operations — use `?`, `.log_err()`, or explicit `match`/`if let Err`
- Async operations that fail must propagate errors to the UI layer
- Be careful with indexing operations that may panic on out-of-bounds

### Variable naming and structure

- Full words for variable names (no abbreviations like "q" for "queue")
- Prefer adding functionality to existing files over creating new ones
- Avoid creative additions unless explicitly requested
- Use `src/module_name.rs` not `src/module_name/mod.rs`
- New crates: specify `[lib] path = "...rs"` in Cargo.toml (e.g., `gpui.rs` not `lib.rs`)
- Comments: explain "why", not "what"

### Variable shadowing for async contexts

Clone before entering async closures to minimize the lifetime of borrowed references:

```rust
executor.spawn({
    let task_ran = task_ran.clone();
    async move {
        *task_ran.borrow_mut() = true;
    }
});
```

### GPUI Context types

- `App` — root context, global state and entity access
- `Context<T>` — provided when updating `Entity<T>`; derefs to `App`
- `AsyncApp` / `AsyncWindowContext` — provided by `cx.spawn` / `cx.spawn_in`, held across await points
- Callbacks come after the `cx` parameter in function signatures

### Window

Passed as `window` before `cx`. Used for focus, action dispatch, drawing, input state.

### Entities

- `Entity<T>`: handles to state. `read(cx)` → `&T`, `update(cx, ...)` → `&mut T` + `Context<T>`
- `WeakEntity<T>`: weak handle, returns `Result` to handle dangling references
- Inside closures, use the inner `cx` — not the outer one — to avoid double-borrow
- Never update an entity while it's already being updated (panics)
- Use `WeakEntity` for mutually-recursive references to prevent memory leaks

### Concurrency

- All entity updates and UI rendering happen on a single foreground thread
- `cx.spawn(async move |cx| ...)` — foreground async; `cx` is `&mut AsyncApp`
- `cx.spawn(async move |this, cx| ...)` — when outer cx is `Context<T>`; `this` is `WeakEntity<T>`
- `cx.background_spawn(async move { ... })` — work on other threads
- Tasks are cancelled when dropped. Prevent cancellation: `await`, `detach()` / `detach_and_log_err(cx)`, or store in a field

### Elements

- `Render` trait: renders entity state into flexbox element tree. `Entity<T>` where `T: Render` is a "view"
- `RenderOnce`: takes ownership of `self`, receives `&mut App` instead of `Context<Self>`. Use `#[derive(IntoElement)]`
- Conditional attributes: `.when(condition, |this| ...)`, `.when_some(option, |this, value| ...)`
- Style methods mirror Tailwind CSS

### Input events & Actions

- Event handlers: `.on_click(|event, window, cx| ...)`
- Entity-bound handlers: `.on_click(cx.listener(|this, event, window, cx| ...))`
- Actions: `actions!(namespace, [ActionName])` macro or `Action` derive macro
- Dispatch: `window.dispatch_action(action.boxed_clone(), cx)` or `focus_handle.dispatch_action(&Action, window, cx)`
- Doc comments on actions are shown to users

### Notify & Entity events

- `cx.notify()` — re-renders a view after state changes
- `cx.emit(event)` — emit typed event; register with `impl EventEmitter<EventType> for EntityType {}`
- `cx.subscribe(entity, |this, entity, event, cx| ...)` — returns `Subscription`; store in `_subscriptions: Vec<Subscription>` field

### Disallowed methods (enforced by clippy.toml)

| Don't use | Use instead | Reason |
|-----------|-------------|--------|
| `std::process::Command::spawn` | `smol::process::Command::spawn` | std blocks async runtime |
| `std::process::Command::output` | `smol::process::Command::output` | same |
| `std::process::Command::status` | `smol::process::Command::status` | same |
| `smol::Timer::after` | `gpui::BackgroundExecutor::timer` | non-determinism in tests |
| `serde_json::from_reader` | `serde_json::from_slice` | from_reader is much slower |
| `cocoa::foundation::NSString::alloc` | `ns_string()` helper | must autorelease to avoid leaks |

## §2: Testing Patterns

### GPUI test timers

Always use GPUI executor timers in tests, not `smol::Timer::after`:

- `cx.background_executor().timer(duration).await` (in `Context<T>`)
- `cx.background_executor.timer(duration).await` (in `TestAppContext`)
- `smol::Timer::after` is not tracked by GPUI's scheduler and causes "nothing left to run" when using `run_until_parked()`

### GPUI test structure

See `.agents/skills/gpui-test/SKILL.md` for full details on:
- `gpui::test` arguments and `TestAppContext` parameters
- Scheduler seeds and ITERATIONS/SEED reproduction
- Parking failures and pending task traces

### Running tests

- Single crate: `cargo test -p <crate_name>`
- All workspace tests: `cargo test --workspace`
- Doc tests: `cargo test --workspace --doc --no-fail-fast`
- Visual regression tests: `cargo build -p zed --bin zed_visual_test_runner --features visual-tests`

## §3: PR and Release Notes Format

- Title: clear, imperative, correctly capitalized. No conventional commits prefixes (`fix:`, `feat:`, etc.)
- No trailing punctuation in titles
- Optional crate prefix for scoped changes: `git_ui: Add history view`
- Body must end with `Release Notes:` section:

```markdown
Release Notes:

- Added ... | - Fixed ... | - Improved ...
```

For non-user-facing changes:

```markdown
Release Notes:

- N/A
```

## §4: Crash Investigation

- Investigation prompts: `.factory/prompts/crash/investigate.md`
- Fix prompts: `.factory/prompts/crash/fix.md`
- Fetch crash reports: `script/sentry-fetch <issue-id>`
- Generate investigation prompt: `script/crash-to-prompt <issue-id>`

## §5: Git and Branching

### Release branches

See `.agents/skills/zed-cherry-pick/SKILL.md` for the full cherry-pick workflow to `preview` or `stable` release branches.

### Rules hygiene

These `.rules` files are read every session. Keep them high-signal:

- New rules must be: (1) non-obvious, (2) repeatedly encountered, (3) specific enough to act on
- Crate-specific rules go in that crate's `.rules`, not the repo root
- Don't put architectural descriptions in `.rules` — they go stale; the agent can read the code
- Don't edit `.rules` inline during normal work. Include "Suggested .rules additions" in your PR description. Reviewers decide what gets merged.
