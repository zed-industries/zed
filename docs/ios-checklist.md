# iPad Port — Working Checklist

Day-to-day reference for what's done and what's next. See `docs/ios-port-plan.md` for
full architectural details.

---

## Phase 0 — Build Infrastructure ✅
- [x] Xcode project, Swift host app, cargo build script
- [x] `force-embed-assets` feature for iOS debug builds
- [x] CI-ready build pipeline

## Phase 1 — GPUI iOS Platform Layer ✅
- [x] Metal renderer (runtime shaders, MSAA, instance buffering)
- [x] CoreText text system (font loading, shaping, glyph rasterization)
- [x] GCD dispatcher (foreground/background, timers)
- [x] Display (UIScreen, scale factor), CADisplayLink vsync
- [x] Touch input (tap, drag, two-finger scroll, momentum scrolling)
- [x] Hardware keyboard (pressesBegan/pressesEnded, HID codes, modifiers)
- [x] Software keyboard (inputView override, tap-gated show/hide)
- [x] Trackpad scroll, hover, right-click, secondary click
- [x] Clipboard, dark mode, prompt dialogs, open_url stubs

## Phase 1.5 — Headless Boot ✅
- [x] Full workspace UI renders with embedded fonts
- [x] Syntax highlighting, command palette, theme selector, file finder
- [x] Vim mode with partial-failure tolerance
- [x] Default iOS keymap (`assets/keymaps/default-ios.json`)
- [x] Settings watcher, globe key fix
- [x] iPadOS menu bar, momentum scrolling, secondary click

## Phase 1.8 — Connection Landing Screen ✅
- [x] GPUI-based connection manager
- [x] Saved hosts list with add/remove, edit mode
- [x] Per-host project paths with add/edit/remove
- [x] Tab/Shift-Tab navigation, focus indicators
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
- [ ] Bottom dock resize (flex value not passed — shared bug, needs investigation)
- [ ] Workspace database: persist/restore panel state across navigation

---

## Phase 3A — Stock Server (no protocol changes)

These items work with the standard `zed --headless` binary.

### 3.1 — Editor Interaction
- [ ] Verify completion menu, hover, diagnostics on iPad screen sizes
- [ ] Long-tap → right-click context menu (go to definition, etc.)
- [ ] Scrolling performance profiling on device
- [ ] Memory management: `os_proc_available_memory()` monitoring

### 3.2 — Settings Path Adjustment
- [ ] iOS config_dir() → Application Support/zed/
- [ ] iOS log/cache/temp directory routing

### 3.3 — Debug Panel
- [ ] Add `debugger_ui`, `debugger_tools`, `dap_adapters` to zed-ios
- [ ] Init debugger_ui, debugger_tools, dap_adapters
- [ ] Verify RemoteDapStore works over SSH connection
- [ ] cfg-gate local dap transport if needed for iOS compilation

### 3.4 — Git Branch Picker & AI Worktree Selection
- [ ] Verify `git_ui` branch/worktree picker works over remote proxy
- [ ] Test AI worktree selection in agent panel

### 3.5 — Collab Panel
- [ ] Feature-gate `livekit_client`, `audio`, screen-capture in `call` crate
- [ ] Implement `ASWebAuthenticationSession` for Zed account OAuth
- [ ] Add `collab_ui` (channel list, contacts, chat — no voice/video)
- [ ] Handle `title_bar` exclusion on iPad

### 3.6 — Edit Prediction
- [ ] Add edit_prediction crates to zed-ios
- [ ] Extract or duplicate provider registry (currently in desktop `zed` crate)
- [ ] Verify Zed/Codestral/OpenAI providers work on iPad
- [ ] Handle Copilot fallback (skip or map to Zed provider)
- [ ] Keybindings: Tab accept, Escape dismiss, Alt+]/[ cycle

### 3.7 — Remote Terminal (via SSH channels)
- [ ] `RusshRemoteConnection::open_terminal()` — opens new SSH session channel
- [ ] PTY allocation via `channel.request_pty()` + `channel.request_shell()`
- [ ] Bidirectional I/O via `channel.split()` (same pattern as `start_proxy`)
- [ ] Terminal resize via `channel.window_change()`
- [ ] Wire into `project::terminals` as iPad alternative to `build_command()`
- [ ] Client-side: display-only terminal via `TerminalBuilder::new_display_only()`
- [ ] Terminal rendering via GPUI text system
- [ ] Touch keyboard accessory bar (Esc, Tab, Ctrl, arrows, etc.)

### 3.8 — Extensions & Tree-sitter Grammar Support
- [ ] Extension downloading from extensions.zed.dev (HTTP + tar.gz unpack)
- [ ] Extension index scanning and metadata loading
- [ ] Language config + theme loading (JSON/TOML — no WASM needed)
- [ ] WKWebView WASM host for Tree-sitter grammars (JIT-enabled JavaScriptCore)
- [ ] Bundle web-tree-sitter JS module in app
- [ ] Grammar bridge: async parse() interface over WKScriptMessageHandler
- [ ] Language registry integration: route WASM bytes to WKWebView on iOS
- [ ] Extension panel UI (show installed extensions, install new ones)
- [ ] Mark extensions with subprocess capabilities as "not supported on iPad"

### 3.9 — Agent Panel (Zed Agent + External Agents)
- [ ] Add agent crates to zed-ios (agent, agent_ui, agent_settings, language_model, etc.)
- [ ] Initialize agent panel in `init_zed()`
- [ ] Zed agent: verify LLM calls work directly from iPad (HTTPS to providers)
- [ ] Zed agent: verify tool invocations proxy through Project → remote host
- [ ] Zed Pro auth via ASWebAuthenticationSession (OAuth/PKCE)
- [ ] External agents: replace `Child::spawn()` with SSH channel exec on iPad
- [ ] External agents: pipe ACP JSON-RPC over SSH channel stdio
- [ ] Add warning: "Keep Zed in foreground while agent is working"

---

## Phase 3B — Modified Server (new protocol messages)

Prerequisite: dev workflow for building and deploying custom `remote_server` binary.

### 3.10 — Remote Zed Agent Execution (survives backgrounding)
- [ ] `proto::AgentStartThread`, `AgentSendMessage`, `AgentStreamEvent`
- [ ] `proto::AgentListThreads`, `AgentResumeThread` for reconnection
- [ ] Server-side Thread engine in HeadlessProject
- [ ] Agent settings sync from host (new proto: GetAgentSettings, GetEnvironmentVariables)

### 3.11 — Notifications
- [ ] Toast notifications — already work (pure GPUI)
- [ ] Workspace notifications — already work
- [ ] Local notification: "Connection to {host} lost" (backgrounded)
- [ ] Local notification: "Agent finished editing {n} files" (backgrounded)
- [ ] `UNUserNotificationCenter` permission request at first SSH connection
- [ ] Collab notification panel (requires Zed account auth — see 3.5/3.9)
- [ ] Push notifications via APNs (defer to v2)

---

## Phase 4 — iPad UX Polish

### 4.1 — Stage Manager Multi-Window
- [ ] Each UIWindowScene = one workspace connection
- [ ] SceneDelegate with NSUserActivity state restoration
- [ ] Window sizing hints via UIWindowSceneGeometryPreferencesIOS

### 4.2 — UIKeyCommand Discoverability HUD
- [ ] Register UIKeyCommand instances for major actions
- [ ] Swift → Rust FFI bridge for action dispatch

### 4.3 — Layout Adaptation
- [ ] Breakpoints: <600pt / 600-900pt / >900pt
- [ ] Sidebar show/hide logic based on window width

### 4.4 �� App Store Entitlements
- [ ] Keychain access groups
- [ ] Increased memory limit (if profiling shows need)
- [ ] Final entitlements review

### 4.5 — Per-Project Settings Profiles
- [ ] Add `profiles` to `ProjectSettingsContent`
- [ ] Update `SettingsStore::set_local_settings()` for project profiles
- [ ] Update `recompute_values()` merge order
- [ ] Update profile selector UI (distinguish project vs user profiles)
- [ ] Decide: global vs per-workspace profile scope

### 4.6 — SSH Key Management (Keychain)
- [ ] `store_ssh_key`, `load_ssh_key`, `delete_ssh_key`, `list_ssh_key_labels`
- [ ] Integration with russh auth flow
- [ ] SSH key import UI via UIDocumentPicker

### 4.7 — Network Resilience
- [ ] NWPathMonitor wrapper (network_monitor.rs)
- [ ] NetworkEvent::Available / Lost into GPUI event system
- [ ] Exponential backoff reconnection strategy
- [ ] Read-only mode with "Reconnecting…" banner

### 4.8 — Server Host Key Verification
- [ ] Known_hosts file in app sandbox
- [ ] First-connection fingerprint prompt
- [ ] Key mismatch warning

---

## Deferred / Follow-up
- [ ] Feature-gate libgit2 out of iOS build (currently requires -lz, -liconv)
- [ ] IME position (update_ime_position) — editor polish
- [ ] Cursor styling (UIPointerInteraction) — trackpad polish
- [ ] File dialogs (UIDocumentPicker) — SSH key import
- [ ] ssh-agent forwarding
- [ ] upload_directory over russh
