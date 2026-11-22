# Rust coding guidelines

* Prioritize code correctness and clarity. Speed and efficiency are secondary priorities unless otherwise specified.
* Do not write organizational or comments that summarize the code. Comments should only be written in order to explain "why" the code is written in some way in the case there is a reason that is tricky / non-obvious.
* Prefer implementing functionality in existing files unless it is a new logical component. Avoid creating many small files.
* Avoid using functions that panic like `unwrap()`, instead use mechanisms like `?` to propagate errors.
* Be careful with operations like indexing which may panic if the indexes are out of bounds.
* Never silently discard errors with `let _ =` on fallible operations. Always handle errors appropriately:
  - Propagate errors with `?` when the calling function should handle them
  - Use `.log_err()` or similar when you need to ignore errors but want visibility
  - Use explicit error handling with `match` or `if let Err(...)` when you need custom logic
  - Example: avoid `let _ = client.request(...).await?;` - use `client.request(...).await?;` instead
* When implementing async operations that may fail, ensure errors propagate to the UI layer so users get meaningful feedback.
* Never create files with `mod.rs` paths - prefer `src/some_module.rs` instead of `src/some_module/mod.rs`.
* When creating new crates, prefer specifying the library root path in `Cargo.toml` using `[lib] path = "...rs"` instead of the default `lib.rs`, to maintain consistent and descriptive naming (e.g., `gpui.rs` or `main.rs`).
* Avoid creative additions unless explicitly requested
* Use full words for variable names (no abbreviations like "q" for "queue")
* Use variable shadowing to scope clones in async contexts for clarity, minimizing the lifetime of borrowed references.
  Example:
  ```rust
  executor.spawn({
      let task_ran = task_ran.clone();
      async move {
          *task_ran.borrow_mut() = true;
      }
  });
  ```

# Timers in tests

* In GPUI tests, prefer GPUI executor timers over `smol::Timer::after(...)` when you need timeouts, delays, or to drive `run_until_parked()`:
  - Use `cx.background_executor().timer(duration).await` (or `cx.background_executor.timer(duration).await` in `TestAppContext`) so the work is scheduled on GPUI's dispatcher.
  - Avoid `smol::Timer::after(...)` for test timeouts when you rely on `run_until_parked()`, because it may not be tracked by GPUI's scheduler and can lead to "nothing left to run" when pumping.

# GPUI

GPUI is a UI framework which also provides primitives for state and concurrency management.

## Context

Context types allow interaction with global state, windows, entities, and system services. They are typically passed to functions as the argument named `cx`. When a function takes callbacks they come after the `cx` parameter.

* `App` is the root context type, providing access to global state and read and update of entities.
* `Context<T>` is provided when updating an `Entity<T>`. This context dereferences into `App`, so functions which take `&App` can also take `&Context<T>`.
* `AsyncApp` and `AsyncWindowContext` are provided by `cx.spawn` and `cx.spawn_in`. These can be held across await points.

## `Window`

`Window` provides access to the state of an application window. It is passed to functions as an argument named `window` and comes before `cx` when present. It is used for managing focus, dispatching actions, directly drawing, getting user input state, etc.

## Entities

An `Entity<T>` is a handle to state of type `T`. With `thing: Entity<T>`:

* `thing.entity_id()` returns `EntityId`
* `thing.downgrade()` returns `WeakEntity<T>`
* `thing.read(cx: &App)` returns `&T`.
* `thing.read_with(cx, |thing: &T, cx: &App| ...)` returns the closure's return value.
* `thing.update(cx, |thing: &mut T, cx: &mut Context<T>| ...)` allows the closure to mutate the state, and provides a `Context<T>` for interacting with the entity. It returns the closure's return value.
* `thing.update_in(cx, |thing: &mut T, window: &mut Window, cx: &mut Context<T>| ...)` takes a `AsyncWindowContext` or `VisualTestContext`. It's the same as `update` while also providing the `Window`.

Within the closures, the inner `cx` provided to the closure must be used instead of the outer `cx` to avoid issues with multiple borrows.

Trying to update an entity while it's already being updated must be avoided as this will cause a panic.

`WeakEntity<T>` is a weak handle. It has `read_with`, `update`, and `update_in` methods that work the same, but always return an `anyhow::Result` so that they can fail if the entity no longer exists. This can be useful to avoid memory leaks - if entities have mutually recursive handles to each other they will never be dropped.

## Concurrency

All use of entities and UI rendering occurs on a single foreground thread.

`cx.spawn(async move |cx| ...)` runs an async closure on the foreground thread. Within the closure, `cx` is `&mut AsyncApp`.

When the outer cx is a `Context<T>`, the use of `spawn` instead looks like `cx.spawn(async move |this, cx| ...)`, where `this: WeakEntity<T>` and `cx: &mut AsyncApp`.

To do work on other threads, `cx.background_spawn(async move { ... })` is used. Often this background task is awaited on by a foreground task which uses the results to update state.

Both `cx.spawn` and `cx.background_spawn` return a `Task<R>`, which is a future that can be awaited upon. If this task is dropped, then its work is cancelled. To prevent this one of the following must be done:

* Awaiting the task in some other async context.
* Detaching the task via `task.detach()` or `task.detach_and_log_err(cx)`, allowing it to run indefinitely.
* Storing the task in a field, if the work should be halted when the struct is dropped.

A task which doesn't do anything but provide a value can be created with `Task::ready(value)`.

## Elements

The `Render` trait is used to render some state into an element tree that is laid out using flexbox layout. An `Entity<T>` where `T` implements `Render` is sometimes called a "view".

Example:

```
struct TextWithBorder(SharedString);

impl Render for TextWithBorder {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().border_1().child(self.0.clone())
    }
}
```

Since `impl IntoElement for SharedString` exists, it can be used as an argument to `child`. `SharedString` is used to avoid copying strings, and is either an `&'static str` or `Arc<str>`.

UI components that are constructed just to be turned into elements can instead implement the `RenderOnce` trait, which is similar to `Render`, but its `render` method takes ownership of `self` and receives `&mut App` instead of `&mut Context<Self>`. Types that implement this trait can use `#[derive(IntoElement)]` to use them directly as children.

The style methods on elements are similar to those used by Tailwind CSS.

If some attributes or children of an element tree are conditional, `.when(condition, |this| ...)` can be used to run the closure only when `condition` is true. Similarly, `.when_some(option, |this, value| ...)` runs the closure when the `Option` has a value.

## Input events

Input event handlers can be registered on an element via methods like `.on_click(|event, window, cx: &mut App| ...)`.

Often event handlers will want to update the entity that's in the current `Context<T>`. The `cx.listener` method provides this - its use looks like `.on_click(cx.listener(|this: &mut T, event, window, cx: &mut Context<T>| ...)`.

## Actions

Actions are dispatched via user keyboard interaction or in code via `window.dispatch_action(SomeAction.boxed_clone(), cx)` or `focus_handle.dispatch_action(&SomeAction, window, cx)`.

Actions with no data are defined with the `actions!(some_namespace, [SomeAction, AnotherAction])` macro call. Otherwise the `Action` derive macro is used. Doc comments on actions are displayed to the user.

Action handlers can be registered on an element via the event handler `.on_action(|action, window, cx| ...)`. Like other event handlers, this is often used with `cx.listener`.

## Notify

When a view's state has changed in a way that may affect its rendering, it should call `cx.notify()`. This will cause the view to be rerendered. It will also cause any observe callbacks registered for the entity with `cx.observe` to be called.

## Entity events

While updating an entity (`cx: Context<T>`), it can emit an event using `cx.emit(event)`. Entities register which events they can emit by declaring `impl EventEmitter<EventType> for EntityType {}`.

Other entities can then register a callback to handle these events by doing `cx.subscribe(other_entity, |this, other_entity, event, cx| ...)`. This will return a `Subscription` which deregisters the callback when dropped.  Typically `cx.subscribe` happens when creating a new entity and the subscriptions are stored in a `_subscriptions: Vec<Subscription>` field.

## Build guidelines

- Use `./script/clippy` instead of `cargo clippy`

# Pull request hygiene

When an agent opens or updates a pull request, it must:

- Use a clear, correctly capitalized, imperative PR title (for example, `Fix crash in project panel`).
- Avoid conventional commit prefixes in PR titles (`fix:`, `feat:`, `docs:`, etc.).
- Avoid trailing punctuation in PR titles.
- Optionally prefix the title with a crate name when one crate is the clear scope (for example, `git_ui: Add history view`).
- Include a `Release Notes:` section as the final section in the PR body.
- Use one bullet under `Release Notes:`:
  - `- Added ...`, `- Fixed ...`, or `- Improved ...` for user-facing changes, or
  - `- N/A` for docs-only and other non-user-facing changes.
- Changes to `gpui` (including its benchmark and test harnesses) must have a bullet prefixed with `[GPUI]` describing the improvement even when nothing is user-facing in Zed itself, since GPUI is consumed as a library (CI enforces this), for example `- [GPUI] Improved benchmarks to report retired CPU instructions alongside wall time on Linux`.
- Format release notes exactly with a blank line after the heading, for example:

```
Release Notes:

- N/A
```

# Crash Investigation

## Sentry Integration
- Crash investigation prompts: `.factory/prompts/crash/investigate.md`
- Crash fix prompts: `.factory/prompts/crash/fix.md`
- Fetch crash reports: `script/sentry-fetch <issue-id>`
- Generate investigation prompt from crash: `script/crash-to-prompt <issue-id>`

# Rules Hygiene

These `AGENTS.md` files are read by every agent session. Keep them high-signal.

## After any agentic session
If you discover a non-obvious pattern that would help future sessions, include a **"Suggested AGENTS.md additions"** heading in your PR description with the proposed text. Do **not** edit `AGENTS.md` inline during normal feature/fix work. Reviewers decide what gets merged.

## High bar for new rules
Editing or clarifying existing rules is always welcome. New rules must meet **all three** criteria:
1. **Non-obvious** — someone familiar with the codebase would still get it wrong without the rule.
2. **Repeatedly encountered** — it came up more than once (multiple hits in one session counts).
3. **Specific enough to act on** — a concrete instruction, not a vague principle.

Rules that apply to a single crate belong in that crate's own `AGENTS.md` file, not the repo root.

## What NOT to put in `AGENTS.md`
Avoid architectural descriptions of a crate (module layout, data flow, key types). These go stale fast and the agent can gather them by reading the code. Rules should be **traps to avoid**, not **maps to follow**.

## No drive-by additions
Rules emerge from validated patterns, not one-off observations. The workflow is:
1. Agent notes a pattern during a session.
2. Team validates the pattern in code review.
3. A dedicated commit adds the rule with context on *why* it exists.

# Fork personalizations (Belizário Ribeiro)

This fork (`belizariogr/zed`) carries personal source customizations on top of upstream Zed (and syncs such as from Zelle's fork).

**HARD RULE:** Whenever this fork is updated from upstream (or any other remote), you **must** keep the personalization commits below working. Do not drop them, overwrite them silently, or leave them broken after a merge/rebase. After every sync:

1. Ensure each personalization still exists in the tree (or is cleanly re-applied if rewritten).
2. Resolve conflicts in favor of preserving the personalization behavior.
3. Verify the related features still compile and behave as described.
4. When adding a new personalization, append it to this list in the same commit (or immediately after).

Author filter: `Belizário Ribeiro <belizariogr@gmail.com>`.

## Personalization commits (on `main`)

Newest first. Hashes may change if history is rewritten; match by message/intent if needed.

| Commit | Summary | What to preserve |
| --- | --- | --- |
| Match by summary | Update from this fork's GitHub releases | `auto_update` reads `https://github.com/belizariogr/zed/releases/latest/download/release.json` (repository overridable with `ZED_FORK_RELEASE_REPOSITORY` at build time) instead of zed.dev and downloads `Zed-{arch}.dmg`, `zed-linux-{arch}.tar.gz` or `Zed-{arch}.exe` from it; SSH/Docker/WSL remote server downloads (`get_remote_server_release_url` and `download_remote_server_release`) also resolve `zed-remote-server-{os}-{arch}.{gz,zip}` from the fork's GitHub releases (`fork-v{major}.{minor}.{patch}` with `latest` fallback for custom tags) instead of zed.dev; a rebuild of the same version updates when its run number (second build metadata field) is higher, while builds without one (local) only update to a higher version; Linux system installs (`$PREFIX/lib/zed/libexec/zed-editor`) update by running the archive's `install-linux-release` with `SUDO=sudo` and `SUDO_ASKPASS` pointing at an `askpass` session, so `sudo -A` gets the password from the git askpass modal (`git_ui_core` `AskPassModal`, registered by `auto_update_ui` through `set_install_password_prompt`) without polkit, and require sudo instead of rsync; `script/install-macos`/`install-linux-system` builds keep `ZED_UPDATE_EXPLANATION` and do not update. README documents per-platform installation with `releases/latest/download` links. |
| Match by summary | Identify the build as a repack in the About window | The About window shows "Repack of Zed by Belizário Ribeiro" under the version headline, includes it in Copy details, and is tall enough to fit it (`zed` `zed.rs` about). |
| Match by summary | Build signed macOS DMGs, Linux release archives and Windows installers on GitHub Actions | `.github/workflows/fork_release.yml` (manual run or `fork-v*` tag) builds the stable channel: macOS aarch64/x86_64 DMGs through `script/bundle-mac`, signed with the Developer ID of team 3Z7C3Q7NMT and notarized with an Apple ID (secrets `MACOS_P12_BASE64`, `MACOS_P12_PASSWORD`, `MACOS_NOTARY_APPLE_ID`, `MACOS_NOTARY_PASSWORD`); `bundle-mac` accepts `MACOS_CODESIGN_IDENTITY`, `APPLE_NOTARIZATION_TEAM`, `APPLE_NOTARIZATION_APPLE_ID`/`APPLE_NOTARIZATION_PASSWORD`, adds its keychain to the search list, and skips Zed's provisioning profile for other teams; Linux x86_64/aarch64 archives from `script/bundle-linux` plus the musl remote server, its version label and `install-linux-release` (used by the auto-updater); unsigned Windows x86_64/aarch64 Inno Setup installers from `script/bundle-windows.ps1` on `windows-2022` (which finds Visual Studio with vswhere instead of assuming Community); also uploads each platform's `zed-remote-server-{macos,linux,windows}-{x86_64,aarch64}.{gz,zip}` archive; publishes the release `fork-v{version}` titled `Zed {version}` (tag pushes keep their tag), deleting and recreating it when it already exists, with whichever platforms built (including the remote servers) and a `release.json` manifest (`version` with `+stable.{run}.{sha}`, `tag`, `assets`) (a failed platform must not block the others; the notes list what is missing) and `script/install-linux-release` (as `install-zed-linux.sh`), which installs the bundle to `/usr/lib/zed` (dropping bundled libraries the system already provides, and keeping the bundled `libstdc++` only when it is newer, since an older one loaded through the rpath stops Mesa's Vulkan driver from loading), links `/usr/bin/zed`, and installs the remote server like `install-linux-system`; all privileged steps run in one `$SUDO` call (with `-A` when `SUDO_ASKPASS` is set) so the password is asked once. |
| Match by summary | Match VS Code's indentation when pressing Enter | `newline` ports VS Code's `getEnterAction`/`getIndentForEnter` (onEnterRules, indentationRules, bracket indent/outdent) from `[on_enter]` in the TypeScript, JavaScript, TSX (with JSX tag rules), JSON, JSONC and Rust `config.toml`; tree-sitter string/comment scopes stand in for TextMate tokens; `{\|}` splits into three lines; Enter inside indentation keeps the cursor column; Zed's `//` and doc-comment continuation run first; only with syntax-aware `auto_indent` (`language_core` language_config, `language` on_enter, `editor` input, `grammars` configs). |
| Match by summary | Classify JS/TS identifiers with language server semantic tokens | JavaScript, TypeScript, and TSX default to `"semantic_tokens": "combined"`, so vtsls (tsserver) decides whether an identifier is a class, interface, type, enum, namespace, or variable, as in VS Code; `typescript/semantic_token_rules.json` styles `readonly` variables and properties (`const` bindings) and enum members as `constant`. Do not reintroduce the generic PascalCase `@type.class` tree-sitter rule from `11174d394d`: it colored PascalCase objects such as `const Config = {...}` as classes. Tree-sitter keeps only upstream's syntactic class captures (`new X`, `class X`, `extends X`) as fallback (`assets/settings/default.json`, `grammars` typescript, `languages` typescript/lib). |
| Match by summary | Resolve VS Code variables and `.env` values in launch.json | `.vscode/launch.json` and `tasks.json` map VS Code predefined variables (`userHome`, `pathSeparator`/`/`, `file*`, `relativeFileDirname`, `lineNumber`, `columnNumber`, `selectedText`) to Zed equivalents; debug configs from a nested `.vscode/launch.json` resolve `${workspaceFolder}` to the folder containing that `.vscode`, and resolve `${env:NAME}` from the `.env` beside it (e.g. `frontend` and `frontend/.env` for `frontend/.vscode/launch.json`), falling back to the project environment (`task`, `project` task_inventory, `debugger_ui` running). |
| Match by summary | Show debug controls beside the title bar account menu | On all platforms, show compact Continue/Pause, Step Over, Step In, Step Out, Restart, and red Stop controls beside Sign In or the account menu only during an active debug session; follow the selected session and thread, work with the debugger panel closed, and disappear when the session ends (`debugger_ui`, `title_bar`, `workspace`). |
| Match by summary | Keep the debugger console where it was dragged | Dragging the debugger console into the terminal panel or the editor keeps that pane. Later debug sessions, including restarts, show the new console there. With `debugger.auto_close_on_session_end`, stopping debug hides that console and the debugger panel; the next session shows the console in the same pane (`debugger_ui`). |
| Match by summary | Use a red rounded-square debug stop icon | The debugger panel Stop button uses `IconName::Stop` colored `Color::Error` (a filled rounded square) instead of the power symbol; disabled sessions still grey out the control (`debugger_ui`). |
| Match by summary | Use a red rounded-square debug stop icon | The debugger panel Stop button uses `IconName::Stop` colored `Color::Error` (a filled rounded square) instead of the power symbol; disabled sessions still grey out the control (`debugger_ui`). |
| Match by summary | Add automatic panel visibility settings | `project_panel.auto_open_on_project_open` shows the file panel for new and reopened projects despite saved visibility; `debugger.auto_close_on_session_end` hides only the debugger panel when the last session ends, preserving output and watches, reveals the project panel if its auto-open option is enabled, and excludes debugger visibility and zoom from saved layouts (including ignoring legacy open states on restore). With auto-close disabled, debugger visibility still persists. Both options are available in Settings > Panels (`project_panel`, `debugger_ui`, `workspace`, settings/settings_ui). |
| Match by summary | Install the remote server with Linux system builds | `script/install-linux-system` also compiles `remote_server` in a separate release build, installs it to `~/.zed_server/zed-remote-server-{channel}-{version}` matching the editor version so SSH clients do not download a missing GitHub artifact, and stops/restarts a running remote server daemon; requests sudo before building and refreshes it in the background until the script exits. |
| Match by summary | Show project tree controls while hovering the file panel | Right-aligned, borderless New File (file with plus), New Folder, and Collapse All (overlapping squares with minus) buttons use the original classic VS Code Codicons at 16 px and the theme's muted text color at 75% opacity, appearing on project roots while hovering anywhere in the project panel; toolbar creation uses the selected directory, the selected file's parent, or the button's project root when nothing is selected, and respects read-only projects. Esc restores the previous selection (including no selection) so creation can be restarted immediately. Project path tooltips appear only over file/folder name text, excluding empty row space and controls while preserving button tooltips (`project_panel`). |
| Match by summary | Add Close Panel button to the terminal panel | Always-visible Close Panel icon in the terminal tab bar (divider only when New/Split/Zoom are shown), emitting `PanelEvent::Close` so the dock closes regardless of left/bottom/right position; preserve existing New/Split/Zoom focus-only controls (`terminal_view` terminal_panel). |
| Match by summary | Add debugger console clearing | Right-click Clear Console and `console::ClearConsole`, clearing output and ANSI highlights without replaying pending output; preserve the debug session, watches, query input/history, and subsequent output (`debugger_ui` console). |
| Match by summary | Preview individual file diffs from the Git panel | Default `file_diff` single-click preview replaces the previous preview; double-click and edits keep the tab open; Single Click Action in Panels > Git Panel selects Uncommitted Changes, File Diff, or Open File; preserve explicit click/preview settings, unsaved edits in shared buffers, and deduplication during rapid/concurrent opens (`git_ui` panel/solo_diff_view, settings/settings_ui). |
| Match by summary | Add Delphi-style numbered bookmarks | Toggle/jump 0–9, list, clear, gutter badges to the left of breakpoints, workspace persistence, optional `.vscode/numbered-bookmarks.json`, and disabling macOS symbolic hotkeys while focused so cmd-shift-N is not stolen by screenshots (`editor`, `project` bookmark_store, `gpui_macos`, settings/keymaps). |
| Match by summary | Navigate browser launches after the page debugger is configured | Open the URL from `launch.json` after binding and configuring the browser page's child session, avoiding a cached service worker deadlock while preserving breakpoints, URL filters, browser profiles, and lifecycle cleanup; keep the real `url` and set `skipNavigateForTest` rather than replacing `url` with `about:blank`, since js-debug's own post-launch `Page.navigate` would otherwise race Zed's navigation and sometimes leave the tab on about:blank (`dap_adapters` javascript, `project` debugger session). |
| Match by summary | Allow adding watch expressions without an active debug session | Always enable add-watch controls, show Variables before starting debug, retain watch expressions in the project's DAP store across sessions, and evaluate them when a stack frame becomes available (`debugger_ui`, `project` debugger session/dap_store). |
| Match by summary | Add a macOS build and install script | `script/install-macos` builds the fork in release mode as stable by default, preserves macOS bundle icons and document associations, and installs the app and CLI. |
| Match by summary | Overlay the stack frame arrow on gutter breakpoints | VS Code-style outlined stack frame arrow drawn above (and around) a smaller breakpoint circle without blocking clicks, so breakpoints can be removed/re-added on the stopped line; `assets` build script so newly added icons get embedded in release builds (`editor`, `assets/icons/debug_stack_frame.svg`, `crates/assets/build.rs`). |
| Match by summary | Close debugger-launched browsers when their Zed window closes | Track the original browser endpoint until its owning Zed window closes or the app quits, even after the debug tab/session ends; preserve browsers owned by other Zed windows. Before a new browser launch (including restarts), close the tracked browser with the same profile (`--user-data-dir`/`userDataDir`) and wait for its endpoint and `SingletonLock` to go away: on macOS closing the window leaves Chromium running, so the next launch would hand its about:blank window to it and the old session would reconnect there (`project` debugger browser/dap_store/session, `debugger_ui` panel). |
| `75d684dfb5` | Show active stack frame indicator in the editor gutter | Dedicated gutter icon for the current debug line; soften breakpoint accents when no debug session is running (`editor`, `icons`, `assets/icons/debug_stack_frame.svg`). |
| `b6f480a59f` | Improve watch expression UI and undefined handling | Inline add-watch input in Variables; keep watches when evaluation fails or returns undefined (`debugger_ui` variable list, `project` debugger session). |
| `9160b4f4bb` | Move add-watch button into the Variables tab bar | Add-watch control lives beside Console/Variables tabs (`debugger_ui` running session UI). |
| `a468d10f12` | Add a + button to create debugger watch expressions | Ability to add watches from the variables list without selecting an existing variable first. |
| `6f02e45fb0` | Fix browser debugger lifecycle and platform launch overrides | Disconnect ended/failed sessions without terminating the debuggee; reconnectable Chromium TCP launches; apply `launch.json` OS overrides before converting configs (`dap_adapters` javascript, `project` dap_store/session, `task` vscode_debug_format). |
| `5ddf1bbc37` | Add Bun debugger and activate session windows on breakpoints | Bun DAP adapter/inspector, Bun debug locator, docs, and bringing session windows forward on breakpoints. |
| `c4de2623fa` | Install Linux system builds as stable with reliable icons | `script/install-linux-system` defaults to stable channel and absolute desktop icon path. |
| `c9f5940cd8` | Add system-wide Linux install script | System-wide Linux install path: CLI in `/usr/bin`, editor under `/usr/lib/zed` (`script/install-linux-system`). |
| `96bf5e1fe5` | Enhance Project Panel UI with Root State Handling | Project panel `is_root` handling and root-specific padding/collapse-all behavior; control visibility follows the panel hover personalization above (`project_panel`). |

Note: `5925f8fbfc` merged the PascalCase highlighting branch (`11174d394d`) into `main`; that heuristic was replaced by the semantic token classification above.
