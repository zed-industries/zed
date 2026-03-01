# Architecture

**Analysis Date:** 2026-03-01

## Pattern Overview

**Overall:** Modular, layered desktop application with a GPU-accelerated immediate-mode UI framework (GPUI) as the foundation.

**Key Characteristics:**
- **GPUI-based:** All UI rendering uses GPUI, a custom GPU-accelerated UI framework with immediate-mode rendering
- **Entity-Component Model:** Application state is managed through GPUI's `Entity<T>` handles and a global `App` context
- **Single-threaded UI Foreground:** All UI rendering and entity updates occur on the foreground thread
- **Async-aware:** Extensive use of async/await with `AsyncApp` and `AsyncWindowContext` for non-blocking operations
- **Plugin Architecture:** Extensions loaded at runtime via the extension host system
- **Modular Crate Structure:** 225+ specialized crates with clear separation of concerns

## Layers

**Platform Layer:**
- Purpose: Operating system abstraction and window management
- Location: `crates/gpui_platform`, `crates/gpui_macos`, `crates/gpui_windows`, `crates/gpui_linux`
- Contains: Platform-specific window creation, input handling, rendering APIs (Metal/D3D11/Wgpu)
- Depends on: Native OS APIs, graphics libraries
- Used by: GPUI core, application initialization

**Rendering Layer:**
- Purpose: GPU-accelerated drawing and layout using flexbox
- Location: `crates/gpui`, `crates/gpui_wgpu`, `crates/gpui_macros`
- Contains: Element trait, layout engine, rendering primitives, input handling
- Depends on: Platform layer, geometry utilities
- Used by: All UI components

**Framework Layer (GPUI):**
- Purpose: Application context, state management, entity system, concurrency primitives
- Location: `crates/gpui/src/app.rs`, `crates/gpui/src/executor.rs`
- Contains: `App`, `Context<T>`, `AsyncApp`, `Entity<T>`, `WeakEntity<T>`, action dispatching, task scheduling
- Depends on: Rendering layer, platform layer
- Used by: All application code, views, modal handlers

**Core Business Logic Layer:**
- Purpose: Editor, project management, language servers, debugging, version control
- Location: `crates/editor`, `crates/project`, `crates/language`, `crates/git`, `crates/dap`
- Contains: Buffer management, LSP client integration, debugger DAP implementation, git operations
- Depends on: GPUI framework, external services (LSP, debugger protocols)
- Used by: Workspace, UI panels, views

**Workspace/Layout Layer:**
- Purpose: Window state, pane management, multi-workspace coordination
- Location: `crates/workspace`
- Contains: `Workspace`, `Pane`, `PaneGroup`, `Dock`, item system, persistence
- Depends on: Core business logic, GPUI framework
- Used by: Main application, window management

**UI Components Layer:**
- Purpose: Feature-specific views and interactive components
- Location: `crates/*_ui` (e.g., `crates/editor` element rendering, `crates/project_panel`, `crates/git_ui`, `crates/terminal_view`)
- Contains: Panels, dialogs, inline editors, search views, theme previews, debugger UI
- Depends on: Workspace/Layout, Core Business Logic, GPUI
- Used by: Workspace for docking, action handlers

**Application Layer:**
- Purpose: App initialization, global state, menu setup, action routing
- Location: `crates/zed/src/main.rs`, `crates/zed/src/zed.rs`, `crates/zed/src/zed/app_menus.rs`
- Contains: Global application setup, multi-workspace coordination, menu definitions, action handlers, settings initialization
- Depends on: All other layers
- Used by: OS

**Extension/Plugin Layer:**
- Purpose: Runtime-loaded extensions (themes, languages, commands)
- Location: `crates/extension`, `crates/extension_host`, `crates/extension_cli`
- Contains: Extension loading, WASM runtime, extension API, theme registry
- Depends on: Core business logic, settings
- Used by: Application layer for dynamic feature loading

**Collaboration Layer:**
- Purpose: Real-time collaboration features (when available)
- Location: `crates/collab`, `crates/collab_ui`, `crates/session`, `crates/channel`, `crates/call`
- Contains: Multi-user editing synchronization, channel management, presence, voice calling
- Depends on: Core business logic, network client
- Used by: Workspace items, authentication

**Infrastructure Layer:**
- Purpose: External services, authentication, network communication
- Location: `crates/client`, `crates/cloud_api_client`, `crates/http_client`, `crates/db`
- Contains: RPC client, API clients, database access, telemetry, credential management
- Depends on: Standard library, external crates (reqwest, sqlite)
- Used by: All business logic that needs external data

## Data Flow

**Application Startup:**

1. `main()` in `crates/zed/src/main.rs` initializes system resources
2. Parse CLI arguments, create `Application` with GPUI platform
3. Initialize file system, logging, crash handling, session tracking
4. Build global state: `Client`, `LanguageRegistry`, `ThemeRegistry`, `SettingsStore`
5. Create `MultiWorkspace` entity
6. Open initial window(s) with `build_window_options()` from `crates/zed/src/zed.rs`
7. Restore persisted workspace layout from `crates/db`
8. Start listening for file changes, language server updates, user input

**User Input to Action:**

1. User presses key → GPUI platform layer captures input
2. Key event dispatched through `KeyDispatch` to focused element
3. Element or action handler registered via `.on_action()` or `.on_key_down()`
4. Action closure mutates state via `cx.update()` or spawns async task
5. State change → `cx.notify()` triggers view rerender
6. Modified view emits element tree → rendered to GPU

**File Edit Flow:**

1. User edits text in `Editor` entity (in `crates/editor/src/editor.rs`)
2. Edit creates `Transaction` in `MultiBuffer`
3. If collaborative: Transaction sent via `Client` to remote peers
4. If LSP available: Diagnostic updates triggered, completion cache invalidated
5. Undo/redo states updated in `Editor`
6. View re-renders via GPUI's layout and painting pipeline

**Language Server Operations:**

1. `LspStore` in `crates/project/src/lsp_store.rs` manages language server instances
2. File change detected → send `textDocument/didChange` to LSP
3. LSP responds with diagnostics, code actions, completions
4. Updates are merged into project's diagnostic/completion caches
5. Editor re-renders with inline diagnostics, hints, colors

**Project Opening:**

1. `Project` entity created in `crates/project/src/project.rs`
2. Worktrees enumerated via `Fs` abstraction (`crates/fs`)
3. Git state initialized via `GitStore`
4. Language detection for all files
5. First LSP servers spawned (if configured)
6. Project panel populates with file tree
7. User can open buffers → Editor views created

**State Management:**

- **Global State:** Managed via `App` context singletons (e.g., `UserStore`, `ThemeRegistry`)
- **Window State:** Stored in `Workspace` entity, accessed via `WindowHandle`
- **Entity State:** Individual `Entity<T>` holds state, updated via `entity.update(cx, |this, cx| { ... })`
- **Local Component State:** Captured in struct fields during render, re-derived on each frame
- **Persistence:** Key-value store in `crates/db/src/kvp.rs`, workspace layout in `crates/workspace/src/persistence.rs`

## Key Abstractions

**Entity<T> (GPUI Handle System):**
- Purpose: Reference counting and lifetime management for stateful objects
- Examples: `Entity<Workspace>`, `Entity<Editor>`, `Entity<Project>`, `Entity<Buffer>`
- Pattern: Create with `cx.new()`, read with `.read(cx)`, update with `.update(cx, |this, cx| { ... })`
- Key methods: `read()`, `read_with()`, `update()`, `update_in()`, `downgrade()`, `entity_id()`

**Render/RenderOnce Trait:**
- Purpose: Convert entities and components into element trees for GPU rendering
- Examples: `impl Render for Workspace`, `impl Render for Editor`
- Pattern: Return `impl IntoElement` with div, flex, button, text elements
- File locations: Every view struct implements `Render` in same file or neighboring `_element.rs`

**Item Trait:**
- Purpose: Polymorphic workspace items (editors, images, terminals, REPLs, etc.)
- Examples: `Editor`, `ImageViewer`, `TerminalPanel`, `MarkdownPreview`
- Pattern: Implement `Item` trait from `crates/workspace/src/item.rs`, register with pane
- Key methods: `try_open()`, `entry_id()`, `project_path()`, `is_dirty()`, `clone_on_split()`

**Pane and PaneGroup:**
- Purpose: Hierarchical layout of items in a workspace
- Location: `crates/workspace/src/pane.rs`, `crates/workspace/src/pane_group.rs`
- Pattern: `PaneGroup` contains binary tree of panes/groups; each `Pane` holds items
- Used for: Tab management, split handling, focus tracking

**Action System:**
- Purpose: Keybinding-driven operations and global command routing
- Location: `crates/gpui/src/action.rs`, `crates/zed/src/zed.rs`
- Pattern: Define via `actions!(namespace, [ActionName])` or `#[derive(Action)]`; dispatch with `window.dispatch_action()`
- Handlers: Registered via `.on_action(|action, window, cx| { ... })`

**Buffer and MultiBuffer:**
- Purpose: Text document state and collaborative editing
- Location: `crates/language/src/buffer.rs`, `crates/editor/src/editor.rs`
- Pattern: `Buffer` = single file; `MultiBuffer` = union of buffers in editor (for multi-file search/replace)
- Features: Undo/redo, blame/diff tracking, syntax coloring, folding ranges

**LanguageServer/LspStore:**
- Purpose: LSP protocol handling
- Location: `crates/project/src/lsp_store.rs`
- Pattern: Manage connections, dispatch LSP requests/notifications, cache responses
- Integrations: Diagnostics, completions, hovers, renames, refactors, code actions

**Workspace Persistence:**
- Purpose: Save/restore editor state, open files, pane layout
- Location: `crates/workspace/src/persistence.rs`, `crates/db`
- Pattern: `SerializedWorkspace` → `WorkspaceDb` schema → recoverable on restart
- Includes: Pane structure, active items, scroll positions, search history

## Entry Points

**Application Entry Point:**
- Location: `crates/zed/src/main.rs` line 179 (`fn main()`)
- Triggers: System launch, user double-click
- Responsibilities: CLI parsing, file system init, crash handler setup, GPUI app creation, window opening, multi-workspace restoration

**Window Render Loop:**
- Location: GPUI's `crates/gpui/src/app.rs` (internal)
- Triggers: Frame timer (typically 60 FPS), input event, state change
- Responsibilities: Call `Render::render()` on root view, layout elements, dispatch paint commands to GPU

**Action Dispatch:**
- Location: `crates/gpui/src/key_dispatch.rs`, element `.on_action()` handlers
- Triggers: User keystroke matching bound action, programmatic `window.dispatch_action()`
- Responsibilities: Route action to focused element/window, execute handler closure

**Extension Loading:**
- Location: `crates/extension_host/src/extension_host.rs`
- Triggers: Application startup, extension install/reload
- Responsibilities: Load WASM, parse manifest, register themes/languages/commands

**Language Server Startup:**
- Location: `crates/project/src/lsp_store.rs` (search for `spawn_language_server`)
- Triggers: File opened matching language server selector, manual LSP install
- Responsibilities: Fork language server binary, establish JSON-RPC connection, send `initialize` request

## Error Handling

**Strategy:** Propagate errors through `Result<T>` types; avoid panics; use `?` operator extensively; log errors; surface critical errors to user via notifications

**Patterns:**

**Propagate with `?`:**
```rust
// Recommended: error propagates to caller
let client = Client::new(url).await?;
let result = client.request(params).await?;
```

**Explicit logging when ignoring:**
```rust
// Not: let _ = some_operation().await?;
// Yes: Log the error for visibility
fs.write_file(path, content).await.log_err();
```

**Custom error handling with `match`:**
```rust
match result {
    Ok(value) => { /* use value */ }
    Err(err) => {
        // Handle specific error cases
        cx.show_notification(err);
    }
}
```

**User-facing errors:**
```rust
// Surface errors as notifications
Toast::error("Failed to save file", cx);
// Or in dialogs
window.prompt(..., "Error: ...", cx)
```

## Cross-Cutting Concerns

**Logging:**
- Framework: `log` crate via `zlog` initialization in `main.rs`
- Usage: `log::info!()`, `log::warn!()`, `log::error()`
- Output: File (`.local/share/zed/logs/`) or stdout depending on TTY detection

**Validation:**
- Settings: `crates/settings` validates keymaps, themes, language configs via schema files
- Project paths: `crates/project` validates file paths before operations
- User input: Modal dialogs validate text input before dispatch

**Authentication:**
- Provider: Zed Cloud account via `crates/client`
- Storage: OAuth tokens in platform keychain via `crates/credentials_provider`
- Flow: Intercepted failed request → sign-in modal → token refresh

**Telemetry:**
- Framework: `crates/telemetry` with event collection
- Data: Anonymous usage metrics (feature flags, performance, errors)
- Opt-out: User settings in `crates/settings`

**Theme/Appearance:**
- Framework: `crates/theme` registry with runtime theme loading
- Sources: Built-in themes + user extensions
- Application: `crates/gpui` renders elements with theme colors from `ThemeSettings`

---

*Architecture analysis: 2026-03-01*
