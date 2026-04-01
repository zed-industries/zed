# iPad Port — Working Checklist

Day-to-day reference for what's done and what's next. See `docs/ios-port-plan.md` for
full architectural details.

---

## Phase 0 — Build Infrastructure ✅
- [x] Xcode project, Swift host app, cargo build script
- [x] `force-embed-assets` feature for iOS debug builds
- [x] CI-ready build pipeline

## Phase 1 — GPUI iOS Platform Layer & Headless Boot ✅
- [x] Metal renderer (runtime shaders, MSAA, instance buffering)
- [x] CoreText text system (font loading, shaping, glyph rasterization)
- [x] GCD dispatcher (foreground/background, timers)
- [x] Display (UIScreen, scale factor), CADisplayLink vsync
- [x] Touch input (tap, drag, two-finger scroll, momentum scrolling)
- [x] Hardware keyboard (pressesBegan/pressesEnded, HID codes, modifiers)
- [x] Software keyboard (inputView override, tap-gated show/hide)
- [x] Trackpad scroll, hover, right-click, secondary click
- [x] Clipboard, dark mode, prompt dialogs, open_url stubs

- [x] Full workspace UI renders with embedded fonts
- [x] Syntax highlighting, command palette, theme selector, file finder
- [x] Vim mode with partial-failure tolerance
- [x] Default iOS keymap (`assets/keymaps/default-ios.json`)
- [x] Settings watcher, globe key fix
- [x] iPadOS menu bar, momentum scrolling, secondary click
- [x] GPUI-based connection landing screen
- [x] Saved hosts list with add/remove, edit mode
- [x] Per-host project paths with add/edit/remove
- [x] Hosts persisted as JSON in app sandbox

## Phase 2 — SSH Transport & Connection ✅
- [x] russh-based SSH transport (`russh_ssh.rs`)
- [x] TCP connect + SSH handshake
- [x] Key auth (ed25519, ecdsa, rsa)
- [x] Password auth (saved or interactive prompt)
- [x] Remote platform/shell probing
- [x] Remote binary resolution
- [x] Proxy start_proxy() with length-prefixed protobuf
- [x] Stderr handling (JSON log records)
- [x] Module split: `ssh.rs` / `subprocess_ssh.rs` / `russh_ssh.rs`
- [x] Module-level cfg gates (no per-item attributes)
- [x] SSH reconnection via russh on iPad
- [x] Multi-workspace support with shared SSH connections
- [x] Full-bleed safe area, iOS titlebar with back button
- [x] Project panel opens automatically on connection
- [x] Workspace panels: project, outline, git, search, diagnostics
- [x] Status bar items wired up
- [x] Post-rebase fixes (AppDatabase, theme_settings, force-embed-assets)
- [x] Auto-connect on launch (KVP persistence, eager per-host reconnect)
- [x] Session persistence on backgrounding (applicationWillResignActive FFI)
- [x] Landing screen: real-time SSH status, themed indicators (spinner/dot)
- [x] Connection error states: retry icon, dismiss button, error detail modal
- [x] Workspace switcher: all saved paths, open unconnected inline
- [x] UIPointerInteraction: custom resize cursors (double-chevron CGPath)
- [x] Sidebar toggle hidden on iOS (sidebar crate not initialized)
- [x] Bottom dock resize (flex value not passed — shared bug, fixed)
- [x] Workspace database: persist/restore dock state (panel sizes, visibility)
- [x] Workspace database: restore open files (deserialize pane items on relaunch)
- [x] SSH-backed terminal panel with full keybindings
- [x] Terminal session restore (initial working directory)
- [x] Trackpad scroll targeting (hover position, momentum fix)
- [x] Settings profile selector + per-workspace persistence
- [x] Project-level profiles (`project_profiles` in `.zed/settings.json`)
- [x] Active profile sync to remote server (UpdateUserSettings proto change)
- [x] Status bar prefix/suffix survives SSH reconnection
- [x] Dev remote server build and deployment workflow
- [x] Debug panel (debugger_ui, debugger_tools, dap_adapters)
- [ ] Debug panel: test full debug workflow end-to-end

---

## Phase 3 — Features

### 3.1 — Edit Prediction ✅
- [x] Provider registry, keybindings, Copilot graceful skip
- [ ] Verify providers work end-to-end (needs Zed Cloud auth)

### 3.2 — Remote Terminal (via SSH channels) ✅
- [x] `open_shell_channel()` / `open_command_channel()` on RemoteConnection
- [x] `TerminalType::Ssh`, iOS terminal task support, exit status fix
- [ ] Touch keyboard accessory bar (Esc, Tab, Ctrl, arrows, etc.)

### 3.3 — Agent Panel (Zed Agent + External Agents) 🔧
- [x] Agent crates initialized, external agents via ACP over SSH
- [x] Keychain unlock auth flow for macOS remotes
- [x] AcpThread keybindings (enter to send, cmd-enter follow)
- [ ] Zed agent: verify LLM calls + tool invocations (needs Zed Cloud auth)
- [ ] Keychain auth: Linux credential file support
- [ ] Add warning: "Keep Zed in foreground while agent is working"

### 3.4 — Additional crates (no platform blockers)
- [ ] `notifications` — toast/workspace notifications (already transitive dep, just init)
- [ ] `sidebar` — sidebar toggle (just init, currently hidden)
- [ ] `outline` — document symbol list (pure UI)
- [ ] `tab_switcher` — ctrl-tab switcher (pure UI)
- [ ] `markdown`, `markdown_preview` — markdown rendering/preview
- [ ] `image_viewer` — image file preview
- [ ] `git_graph` — commit graph visualization
- [ ] `encoding_selector`, `line_ending_selector` — status bar selectors
- [ ] `web_search`, `web_search_providers` — web search from agent panel
- [ ] `snippet_provider`, `snippets_ui` — snippet management
- [ ] `toolchain_selector` — Python/Node toolchain picker
- [ ] `project_symbols` — workspace-wide symbol search
- [ ] `csv_preview`, `svg_preview` — file type previews
- [ ] `settings_ui` — settings GUI
- [ ] `which_key` — key binding hints overlay
- [ ] `journal` — daily journal files

### 3.5 — Collab Panel (blocked on Zed Cloud auth)
- [ ] `ASWebAuthenticationSession` for Zed account OAuth — blocked on redirect URI
- [ ] `collab_ui` — channel list, contacts, chat
- [ ] `call` — voice/video (already transitive dep via git_ui, WebRTC linked)
- [ ] `channel` — channel store, channel buffers
- [ ] `title_bar` — user menu, sharing controls

### 3.6 — Extensions & Tree-sitter (blocked on JIT)
- [ ] Extension downloading/indexing (HTTP + tar.gz — no WASM needed)
- [ ] Language config + theme loading (JSON/TOML — no WASM needed)
- [ ] `extensions_ui` — extension panel UI
- [ ] WKWebView WASM host for Tree-sitter grammars (JIT via JavaScriptCore)
- [ ] `language_extension`, `theme_extension` — extension-provided content
- [ ] Mark extensions with subprocess capabilities as "not supported on iPad"

### 3.7 — Blocked on remote execution proxy
- [ ] `task`, `tasks_ui` — local task runner (needs remote proxy, no local spawn)
- [ ] `repl` — notebook/REPL (needs remote kernel proxy)
- [ ] `dev_container` — dev container support

---

### 3.8 — Remote Zed Agent Execution (survives backgrounding)
- [ ] Server-side Thread engine in HeadlessProject
- [ ] New proto messages for thread lifecycle and LLM streaming
- [ ] Agent settings sync from host

### 3.9 — Notifications
- [ ] Local notification: "Connection lost" / "Agent finished" (backgrounded)
- [ ] `UNUserNotificationCenter` permission request
- [ ] Collab notification panel (requires Zed account auth)

---

## Phase 4 — iPad UX Polish

- [ ] Stage Manager multi-window (UIWindowScene per workspace)
- [ ] UIKeyCommand discoverability HUD (hold ⌘ overlay)
- [ ] Layout adaptation (breakpoints for sidebar/panel visibility)
- [ ] SSH key management via iOS Keychain + UIDocumentPicker import
- [ ] Network resilience (NWPathMonitor, exponential backoff, read-only mode)
- [ ] Server host key verification (known_hosts, fingerprint prompt)
- [ ] App Store entitlements review

---

## Known Bugs
- [ ] Rope panic: UTF-16 point beyond end of line in iOS text input (see `plans/ios-rope-panic.md`)
- [ ] Agent panel input lag (conversation re-rendering on every keystroke — needs profiling)
- [ ] `ExternalAgentsUpdated` arrives before handler registered (race on first connect — benign)

## Deferred / Follow-up
- [ ] Feature-gate libgit2 out of iOS build (currently requires -lz, -liconv)
- [ ] Full UITextInput protocol (selectedTextRange, textInRange, etc. — currently UIKeyInput only)
- [ ] IME / CJK composition support
- [ ] `auto_update` — N/A for App Store distribution
- [ ] `copilot_ui`, `copilot_chat` — blocked on Node.js (no local process)
- [ ] ssh-agent forwarding
- [ ] upload_directory over russh
