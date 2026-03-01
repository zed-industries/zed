# Codebase Structure

**Analysis Date:** 2026-03-01

## Directory Layout

```
zed/
├── crates/                    # 225+ Rust crates implementing all functionality
│   ├── zed/                   # Main application binary and top-level state
│   ├── gpui/                  # GPU-accelerated UI framework (core rendering)
│   ├── workspace/             # Window/pane/item layout and multi-workspace
│   ├── editor/                # Text editor implementation
│   ├── project/               # Project/worktree/LSP/debugger management
│   ├── language/              # Language parsing, syntax, buffers
│   ├── client/                # Network/RPC and authentication
│   ├── *_ui/                  # Feature-specific UI panels (project_panel, git_ui, terminal_view, etc.)
│   ├── *_host/                # Plugin/extension hosts (extension_host, language_model_host)
│   └── [200+ other specialized crates]
├── assets/                    # Static resources: icons, themes, fonts, sounds, keymaps
├── Cargo.toml                 # Workspace definition
├── Cargo.lock                 # Locked dependency versions
├── .rules                     # Agent guidelines for code generation
├── CLAUDE.md                  # Rust coding guidelines (symlink to .rules)
├── .github/workflows/         # CI/CD configuration
└── script/                    # Build and utility scripts
```

## Directory Purposes

**`crates/zed/`:**
- Purpose: Main application binary, initialization, and global UI state
- Contains: Entry point, workspace setup, menu system, quick action bar, telemetry
- Key files:
  - `src/main.rs` (179 lines) - Application bootstrap
  - `src/zed.rs` (2277+ lines) - Global state, action handlers, window management
  - `src/zed/app_menus.rs` - Menu definitions
  - `src/zed/quick_action_bar.rs` - Command palette implementation
  - `src/reliability.rs` - Panic/crash tracking

**`crates/gpui/`:**
- Purpose: GPU-accelerated immediate-mode UI framework
- Contains: Rendering engine, layout (flexbox), input handling, element primitives, action system
- Key files:
  - `src/gpui.rs` - Framework entry point
  - `src/app.rs` (93KB) - Application context, entity system, executor
  - `src/element.rs` - Element trait and composability
  - `src/platform.rs` - OS abstraction layer
  - `src/executor.rs` - Task scheduling and async runtime
  - `src/elements/` - Built-in elements (div, flex, text, button, etc.)

**`crates/workspace/`:**
- Purpose: Window state, multi-window coordination, pane layout
- Contains: Workspace entity, pane groups, item system, persistence, dock/sidebar
- Key files:
  - `src/workspace.rs` (504KB) - Main `Workspace` entity
  - `src/pane.rs` (320KB) - `Pane` entity for tab management
  - `src/pane_group.rs` (52KB) - Binary tree layout
  - `src/item.rs` (57KB) - `Item` trait definition
  - `src/multi_workspace.rs` (31KB) - Multi-window coordination
  - `src/persistence.rs` (164KB) - Layout and state serialization
  - `src/dock.rs` (39KB) - Dockable panels management

**`crates/editor/`:**
- Purpose: Text editor implementation
- Contains: Editor entity, display maps, syntax highlighting, completions, hovers, inlay hints
- Key files:
  - `src/editor.rs` (1.1MB) - Core `Editor` entity
  - `src/element.rs` (551KB) - Editor rendering
  - `src/editor_tests.rs` (1MB) - Comprehensive test suite
  - `src/display_map.rs` (152KB) - Display layout abstraction
  - `src/code_context_menus.rs` (63KB) - Context menu handlers
  - `src/hover_popover.rs` (76KB) - Hover information display
  - `src/inlays.rs` (83KB) - Inline hints and diagnostics

**`crates/project/`:**
- Purpose: Project/worktree management, LSP integration, debugging, git
- Contains: Project entity, language servers, debuggers, file watchers, task inventory
- Key files:
  - `src/project.rs` (218KB) - Core `Project` entity
  - `src/lsp_store.rs` (591KB) - Language server protocol handling
  - `src/buffer_store.rs` (65KB) - Collaborative buffer management
  - `src/git_store.rs` (267KB) - Git operations and blame
  - `src/worktree_store.rs` - File tree abstraction
  - `src/task_store.rs` - Task execution and terminals
  - `src/agent_server_store.rs` (65KB) - Agent/AI server integration
  - `crates/debugger/` subdirectory - DAP client implementation

**`crates/language/`:**
- Purpose: Language support infrastructure
- Contains: Buffer abstraction, syntax trees, language definitions, diagnostics
- Key files:
  - `src/buffer.rs` - Text buffer with undo/redo
  - `src/language.rs` - Language registry
  - `src/language_settings.rs` - Per-language configuration

**`crates/client/`:**
- Purpose: Network communication and authentication
- Contains: RPC client, HTTP client, Zed Cloud integration, user store
- Key files:
  - `src/client.rs` - Main `Client` entity for RPC
  - `src/user_store.rs` - User authentication and profile data
  - `src/proto/` - Protocol buffer definitions

**`crates/settings/`:**
- Purpose: Settings management and persistence
- Contains: Settings schema, keymaps, defaults, watchers
- Key files:
  - `src/settings.rs` - Settings registry and store
  - `src/keymap_file.rs` - Keymap file parsing

**`crates/theme/`:**
- Purpose: Theme system and color management
- Contains: Theme registry, color tokens, appearance system
- Key files:
  - `src/theme.rs` - Theme provider
  - `src/color.rs` - Color definitions

**`crates/*_ui/` (UI Feature Crates):**
- `project_panel/` - File tree sidebar
- `git_ui/` - Git panel, commit view, diff visualization
- `terminal_view/` - Embedded terminal
- `debugger_ui/` - Debugging interface
- `assistant_slash_command/`, `copilot_ui/` - AI assistance panels
- `language_tools/` - LSP diagnostics, code actions
- `search/` - Find/replace, project search
- `outline_panel/` - Document symbols
- `repl/` - Interactive REPL
- Each implements `Render` for display and handles UI interactions

**`crates/extension/` and `crates/extension_host/`:**
- Purpose: Plugin/extension system
- Contains: WASM runtime, extension API, dynamic loading
- Key files:
  - `extension_host/src/extension_host.rs` - Runtime loader
  - `extension/src/extension.rs` - Extension trait

**`crates/db/`:**
- Purpose: Persistent storage
- Contains: SQLite-based key-value store and workspace database
- Key files:
  - `src/kvp.rs` - Global key-value store
  - `src/db.rs` - Database connection pooling

**`assets/`:**
- Purpose: Static resources
- Contains:
  - `themes/` - Built-in editor themes
  - `icons/` - UI icons (SVG)
  - `keymaps/` - Default keybindings
  - `fonts/` - Embedded fonts
  - `sounds/` - Audio resources
  - `settings/` - Default settings files

## Key File Locations

**Entry Points:**
- `crates/zed/src/main.rs` - Application bootstrap
- `crates/zed/src/zed.rs` - Global application state and main render function
- `crates/workspace/src/workspace.rs` - Window workspace entity

**Configuration:**
- `assets/settings/default_settings.json` - Default editor settings
- `assets/keymaps/` - Default keybindings by platform
- `Cargo.toml` - Workspace and dependency configuration

**Core Logic:**
- `crates/editor/src/editor.rs` - Text editor
- `crates/project/src/project.rs` - Project management
- `crates/project/src/lsp_store.rs` - Language server integration
- `crates/workspace/src/workspace.rs` - Multi-workspace coordination

**Testing:**
- Tests co-located with source files using `#[cfg(test)]` modules
- Key test files:
  - `crates/editor/src/editor_tests.rs` (1MB) - Editor behavior
  - `crates/project/tests/` - Project integration tests
  - `crates/language/tests/` - Language tests

## Naming Conventions

**Files:**
- `src/main.rs` - Binary entry point (in bin crates)
- `src/{crate_name}.rs` - Library root (e.g., `src/workspace.rs` in workspace crate)
- `src/{module_name}.rs` - Module implementation
- `src/{module_name}/` - Submodule directory (with internal `mod.rs` or direct children)
- `src/{name}_tests.rs` - Test module co-located with implementation
- `src/{name}_element.rs` - GPUI element rendering code (rendering-heavy files)

**Directories:**
- `crates/{crate_name}/` - Each crate in separate directory
- `src/` - Rust source code
- `tests/` - Integration tests (optional)

**Crate Naming:**
- `{feature}_ui` - UI panels and views for a feature
- `{feature}_host` - Plugin/host system for a feature
- `{feature}_settings` - Settings management for a feature
- `{service}_client` - Client library for external service
- `{tool}_tools` - Utility functions for a tool

## Where to Add New Code

**New Feature (Editor Enhancement):**
- Primary code: `crates/editor/src/editor.rs` or new `src/{feature}.rs` in editor crate
- Tests: Co-located test module or `src/{feature}_tests.rs`
- UI components: `crates/editor/src/{feature}_element.rs` or inline in editor.rs
- Settings: Add to `crates/settings/src/editor_settings.rs`
- Example: Bracket matching is in `crates/editor/src/bracket_colorization.rs`

**New UI Panel:**
- Create new crate: `crates/{panel_name}/`
- Structure:
  - `Cargo.toml` with dependencies on workspace, gpui, project, etc.
  - `src/{panel_name}.rs` - Main entity implementing `Render`
  - Register with dock system in `crates/workspace/src/dock.rs`
- Example: `crates/outline_panel/` for document symbols

**New Editor Action:**
- Add action macro in relevant file: `crates/editor/src/actions.rs`
- Implement handler: `.on_action(cx.listener(|this, action, window, cx| { ... }))`
- Bind keymap: `assets/keymaps/{platform}.json`
- Example: Search actions in `crates/search/src/search.rs`

**New Language Server Integration:**
- Add to `crates/project/src/lsp_store.rs` - LSP request/response handlers
- Register language selector in `assets/settings/default_settings.json`
- Implement custom behavior if needed: `crates/language_tools/` for code actions, hovers, etc.

**New Utility/Helper:**
- Shared helpers: `crates/util/src/`
- Collection utilities: `crates/collections/src/`
- Path utilities: `crates/paths/src/`

**Settings/Config:**
- Editor settings: `crates/settings/src/settings.rs`
- Add to JSON schema in `assets/settings/`
- Make observable: `let settings = cx.observe_config(|_| {})`

## Special Directories

**`.planning/codebase/`:**
- Purpose: AI-readable codebase documentation
- Generated: Yes (by GSD agents)
- Committed: Yes
- Contents: ARCHITECTURE.md, STRUCTURE.md, CONVENTIONS.md, TESTING.md, CONCERNS.md, INTEGRATIONS.md, STACK.md

**`assets/`:**
- Purpose: Static runtime resources
- Generated: No (manually maintained)
- Committed: Yes
- Contents: Icons, fonts, themes, sounds, default settings

**`crates/`:**
- Purpose: All Rust implementation
- Generated: No
- Committed: Yes
- Contents: 225+ crates with clear semantic boundaries

**`.github/workflows/`:**
- Purpose: CI/CD pipelines
- Generated: No
- Committed: Yes
- Contents: Testing, linting, release workflows

**`target/`:**
- Purpose: Build artifacts
- Generated: Yes (by Cargo)
- Committed: No (in .gitignore)

**`.cargo/`:**
- Purpose: Cargo configuration
- Generated: Partially (config.toml checked in)
- Committed: Yes

## Module Organization Pattern

Example from `crates/workspace/`:
```
workspace/
├── Cargo.toml
└── src/
    ├── workspace.rs           # Public library root - re-exports all public types
    ├── dock.rs                # Panel docking system
    ├── pane.rs                # Pane entity (large file)
    ├── pane_group.rs          # Pane tree layout
    ├── item.rs                # Item trait definition
    ├── multi_workspace.rs     # Multi-window coordination
    ├── persistence.rs         # State serialization/restoration
    ├── notifications.rs       # Notification system
    └── persistence/           # Persistence submodule
        ├── mod.rs             # Re-export
        └── model.rs           # Database schema
```

Pattern: Large entities get their own files; shared concerns like persistence are submodules; library root (`workspace.rs`) re-exports public API.

---

*Structure analysis: 2026-03-01*
