# Coding Conventions

**Analysis Date:** 2026-03-01

## Naming Patterns

**Files:**
- Snake case with single underscore separators: `activity_indicator.rs`, `sidebar.rs`, `editor.rs`
- Test files suffixed with `_test.rs`, `_tests.rs`, or placed in `tests/` directory: `edit_prediction_context_tests.rs`, `tests/integration/project_tests.rs`
- Library crate entry points named after crate: `journal.rs`, `lmstudio.rs` (configured in `Cargo.toml` with `[lib] path = "src/..."` rather than `mod.rs`)
- Internal module files use snake case without `mod.rs`: `src/some_module.rs` instead of `src/some_module/mod.rs`

**Functions:**
- Snake case: `test_handle_output_event`, `show_error_message`, `active_debug_session_panel`
- Test functions prefixed with `test_`: `test_initially_disabled`, `test_neovim`, `test_toggle_through_settings`
- Helper functions in tests use descriptive names: `init_test`, `init_test_workspace`, `start_debug_session`

**Variables:**
- Snake case: `server_status`, `worktree_label`, `active_workspace_index`
- Full words, never abbreviations: `queue` not `q`, `workspace` not `ws`
- Context parameters always named `cx` (can be `&App`, `&Context<T>`, `&AsyncApp`, `&mut TestAppContext`, etc.)
- Window parameters always named `window`
- This reference in closures uses natural shadowing: `async move |this, cx| ...` for entity updates

**Types:**
- PascalCase for struct names: `ActivityIndicator`, `VimTestContext`, `WorkspaceThreadEntry`
- PascalCase for enum variants: `ShowErrorMessage`, `Separator`, `WorkspaceThread`
- Trait names: `Render`, `RenderOnce`, `EventEmitter`, `StatusItemView`
- Generic type parameters single uppercase: `T`, `R` (not abbreviated descriptors)

## Code Style

**Formatting:**
- Rust Edition 2024 with style_edition 2024 (`rustfmt.toml`)
- Enforced via `rustfmt` using Rust 1.93 toolchain
- Maximum line length: 120 characters for Prettier (`.prettierrc`); apply rustfmt defaults for Rust code
- No trailing semicolons on module declarations

**Linting:**
- Clippy enabled via `./script/clippy` (not `cargo clippy` directly)
- Disallowed methods configured in `clippy.toml`:
  - `std::process::Command` methods → use `smol::process::Command` instead (non-blocking)
  - `serde_json::from_reader` → use `serde_json::from_slice` (faster for buffers)
  - `cocoa::foundation::NSString::alloc` → use `ns_string()` helper (autorelease safety)
  - `smol::Timer::after` in tests → use `gpui::BackgroundExecutor::timer` (determinism)

## Import Organization

**Order:**
1. Standard library (`use std::...`)
2. External crates (alphabetical by crate name)
3. Internal crates (alphabetical by crate name)
4. Module declarations and reexports (`pub use`, `use crate::...`)
5. Prelude-style imports at the end: `use prelude::*;`

**Pattern:**
```rust
use std::sync::Arc;
use std::collections::HashMap;

use futures::StreamExt;
use gpui::{App, Context, Entity};
use serde_json::json;

use editor::Editor;
use project::Project;
use workspace::Workspace;

use crate::{
    types::Something,
    utils::helper_fn,
};

use ui::prelude::*;
```

**Path Aliases:**
- Relative imports from workspace members use bare names: `use editor::Editor;`, `use workspace::Workspace;`
- Standard library imports use full `std::` prefix
- Prelude imports: crates often expose `prelude::*` containing common types

## Error Handling

**Patterns:**
- Never use `let _ = ...` to silently discard errors on fallible operations
- Propagate with `?` operator when calling function can handle: `some_fallible_call()?;`
- Use `.detach()` or `.detach_and_log_err(cx)` for spawned tasks that should run indefinitely
- Use `.log_err()` for operations where error should be logged but not block execution
- Use explicit `match` or `if let Err(...)` for custom error handling logic
- Always propagate errors to UI layer for user feedback on async operations
- Use `anyhow::Result<T>` for fallible operations; `anyhow::Ok(())` for explicit success in async blocks
- `Option::unwrap_or()` is acceptable when providing a sensible default
- `write!()` macro accepts unwrap (infallible for string writing)

**Examples from codebase:**
```rust
// Correct: propagate error
cx.spawn(async move |this, cx| {
    while let Some(job_event) = job_events.next().await {
        this.update(cx, |this: &mut ActivityIndicator, cx| {
            match job_event {
                fs::JobEvent::Started { info } => { ... }
                fs::JobEvent::Completed { id } => { ... }
            }
            cx.notify();
        })?;
    }
    anyhow::Ok(())
})
.detach();

// Correct: use unwrap_or with default
let mut message = progress.title.clone().unwrap_or(progress_token.to_string());
write!(&mut message, " ({}%)", percentage).unwrap(); // write! is infallible for String
```

## Logging

**Framework:** Built-in with `zlog` (Zed's logging system), initialized with `zlog::init_test()` in tests

**Patterns:**
- Avoid `.log_err()` calls except where error visibility is needed
- Prefer propagating errors with `?` so UI layer can handle and display
- In tests, initialize logging with `#[ctor::ctor] fn init_logger() { zlog::init_test(); }`

## Comments

**When to Comment:**
- Only explain "why", never summarize "what"
- Do not write organizational comments
- Explain non-obvious reasoning or context that isn't clear from code

**JSDoc/TSDoc:**
- Rust doc comments with `///` for public items
- Include doc comments on actions for UI display
- Example from codebase:
```rust
actions!(
    activity_indicator,
    [
        /// Displays error messages from language servers in the status bar.
        ShowErrorMessage
    ]
);
```

## Function Design

**Size:** Keep functions focused on single responsibility; use closures in GPUI for state updates

**Parameters:**
- Context parameters (`cx`) come first or after window
- Callbacks come after context: `.on_click(|event, window, cx| ...)`
- For entity closures: `|this: &mut T, cx: &mut Context<T>| ...` where `this` is the current entity
- Test functions accept context type appropriate to their use: `cx: &mut TestAppContext` or `cx: &mut VisualTestContext`

**Return Values:**
- Return `anyhow::Result<T>` for fallible operations
- Return `Entity<T>` for created entities
- Return `Option<T>` for optional lookups
- Async functions spawned via `cx.spawn()` implicitly wrap return value in `anyhow::Result`

## Module Design

**Exports:**
- Use `pub use` to reexport common types at crate root
- Keep internal implementation details private with `mod` declarations
- Use `#[cfg(test)]` to conditionally compile test modules

**Barrel Files:**
- Root-level modules declare submodules: `pub mod editor_lsp_test_context; pub mod editor_test_context;`
- Reexport important items: `pub use crate::rust_analyzer_ext::expand_macro_recursively;`
- Test utility modules public within test feature: `pub fn test_font() -> Font { ... }`

## Entity and Context Patterns

**Entity Creation:**
- Create with `cx.new(|cx| EntityType { ... })`
- Return `Entity<T>` handle from constructor

**Entity Updates:**
- Read: `entity.read(cx)` returns `&T`
- Read with closure: `entity.read_with(cx, |item, cx| ...)`
- Update: `entity.update(cx, |item, cx| { ... })` for mutable access
- Update in window: `entity.update_in(cx, |item, window, cx| { ... })`
- Weak references: `entity.downgrade()` returns `WeakEntity<T>` (returns `Result` on operations)

**Notification:**
- Call `cx.notify()` when state changes affect rendering

## Task and Async Patterns

**Spawning:**
- Foreground: `cx.spawn(async move |cx| ...)` returns `Task<R>`
- Background: `cx.background_spawn(async move { ... })` for non-UI work
- Task cleanup: store in fields or use `.detach()` / `.detach_and_log_err(cx)`

**Subscriptions:**
- `cx.subscribe(entity, |this, other, event, cx| { ... })` returns `Subscription`
- Store subscriptions in `_subscriptions: Vec<Subscription>` field to maintain lifetime

---

*Convention analysis: 2026-03-01*
