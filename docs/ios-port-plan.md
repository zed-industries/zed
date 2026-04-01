# Zed for iPad — Thin Client Architecture & Engineering Plan

## Project Overview

This is a fork of zed-industries/zed with the goal of porting Zed to iPadOS as a remote thin
client. The iPad runs minimal local compute. All language servers, git operations, file I/O,
terminal PTYs, Node runtimes, and extension hosting run on a remote host (e.g. a Mac Studio)
over SSH. The iPad is a rendering and input shell that connects to a `zed --headless`
server instance. The **AI agent panel** initially runs locally on the iPad (LLM calls over
HTTPS, tool invocations proxy through the remote protocol). In a later phase, the agent
engine moves to the remote host so it can survive iOS backgrounding — iOS kills
backgrounded apps after ~30 seconds, which is incompatible with long-running agent turns.

This product definition is **non-negotiable** and must inform every architectural decision. If
you find yourself about to add a capability that requires spawning a subprocess, accessing
arbitrary filesystem paths, or running a language server locally on the iPad, **stop** — that work
belongs on the host, not the client.

---

## Repository Layout (Relevant to This Port)

```
crates/
  gpui/                  # GPU UI framework — PRIMARY WORK AREA for iOS
    src/platform/
      mac/               # Reference implementation (AppKit + Metal)
      linux/             # Reference implementation (Wayland/X11 + wgpu)
      windows/           # Reference implementation (Win32 + DirectWrite)
      ios/               # NEW — create this module (see Phase 1)
  zed_ios/               # NEW — iOS entry point crate (staticlib)
  remote/                # Existing remote dev protocol — reuse as-is
  remote_server/         # Host-side server — not built for iOS target
  workspace/             # Workspace/Pane/Panel UI — reuse as-is
  editor/                # Editor core — reuse as-is
  terminal/              # VT parser (reuse) + PTY spawn (exclude on iOS)
  agent_ui/              # AI agent panel — reuse; LLM calls from iPad, tools via remote proxy
  agent/                 # Agent conversation engine — INCLUDE in iOS binary
  agent_settings/        # Agent config — INCLUDE; add remote settings sync
  project/               # Project orchestration — used via remote proxy
  settings/              # SettingsStore — reuse, adjust paths for iOS
  theme/                 # Themes — reuse as-is
  lsp/                   # Local LSP spawn — EXCLUDE from iOS binary
  node_runtime/          # Node runtime — EXCLUDE from iOS binary
  task/                  # Local task runner — EXCLUDE from iOS binary
  dap/                   # Local debugger — EXCLUDE from iOS binary
  extension_host/        # Local extension host — EXCLUDE from iOS binary
  git/                   # git CLI wrapper — EXCLUDE from iOS binary

ios/                     # NEW — Xcode project lives here
  ZedApp.xcodeproj/
  ZedApp/
    AppDelegate.swift
    SceneDelegate.swift
    ZedApp.swift
    Supporting Files/
      Info.plist
      ZedApp-Bridging-Header.h
    entitlements/
      ZedApp.entitlements
```

---

## The Architecture

```
iPad (UI Shell)                         Mac Studio (Zed Server)
┌─────────────────────────────────┐     ┌──────────────────────────────┐
│ GPUI iOS platform               │     │ zed --headless               │
│ CAMetalLayer renderer           │     │ LspStore (rust-analyzer...)  │
│ UITextInput keyboard bridge     │◄───►│ WorktreeStore / FSEvents     │
│ UIPointerInteraction input      │SSH /│ GitStore (git CLI)           │
│ UIScene multi-window            │Zed  │ Node runtime / extensions    │
│ Workspace / Panes / Editor      │proto│ Terminal PTYs                │
│ Remote terminal view            │     │ DAP debugger                 │
│ Agent panel + Thread engine     │     │ Agent tool execution         │
│ LLM HTTP calls (HTTPS direct)   │     │ Agent settings + API keys    │
│ SSH connection manager          │     │   (synced to iPad at connect)│
│ Keychain SSH key storage        │     │                              │
└─────────────────────────────────┘     └──────────────────────────────┘
```

The existing `crates/remote/` protocol already implements the UI/compute split. The iPad
app is a new platform front-end for that existing protocol, not a new protocol.

**Agent architecture note:** The agent panel runs in the iPad client process — it makes
HTTPS calls directly to LLM providers (Anthropic, OpenAI, etc.). Agent *tool* invocations
(file edits, grep, terminal commands) flow through the `Project` abstraction, which
transparently proxies them to the remote host via protobuf RPC. The headless server has
**no agent, language model, or LLM provider infrastructure** — it only handles tool
execution. API keys are configured in iPad-local settings or via Zed Pro account auth
(see Phase 3.9). Remote settings sync is a Phase 3B enhancement (Phase 3.10).

---

## ⚠️ CRITICAL: SSH Transport Must Be Rewritten

**Zed does NOT use a Rust SSH library.** The SSH transport in
`crates/remote/src/transport/ssh.rs` shells out to the **system `ssh` binary** via
`std::process::Command("ssh")` and uses Unix-only ControlMaster for connection
multiplexing. This is the single largest technical blocker for iPadOS:

- iOS has **no system `ssh` binary**
- `std::process::Command` / `posix_spawn` are **kernel-level prohibited** on iOS
- App Store automated review rejects any binary linking these symbols

### Required: Embedded Rust SSH Library

Replace the system-ssh transport with an in-process SSH implementation. The
`RemoteConnection` trait in `crates/remote/` provides a clean abstraction point for this.

**Recommended library: `russh`** (v0.57.0+)
- Pure Rust, no C dependencies (uses `ring` or `aws-lc-rs` for crypto)
- Fully async (Tokio)
- Supports **in-memory key loading** via `russh_keys::decode_secret_key()`
- Enables the Keychain integration pattern: retrieve key bytes from iOS Keychain →
  pass to Rust via FFI → decode with russh_keys → authenticate

**Alternative: `ssh2` crate** — has `userauth_pubkey_memory()` for in-memory keys but
depends on libssh2 (C) and OpenSSL, making iOS cross-compilation significantly harder.

### What the new `IosSshTransport` must implement:
- Session establishment (TCP connect + SSH handshake, all in-process)
- Public key authentication from in-memory key bytes (Keychain-loaded)
- Channel multiplexing (replaces ControlMaster)
- Binary upload to remote host (for deploying `zed --headless` if needed)
- Port forwarding (if needed for future features)
- Keepalive and reconnection logic
- Interactive authentication flows (password prompt, 2FA)

**This is not a minor adaptation.** Budget 4–6 weeks of dedicated work and treat it as a
Phase 0.5 / Phase 2 prerequisite. The `RemoteConnection` trait provides the seam; the
implementation is substantial.

---

## ⚠️ Agent Panel: Local LLM Calls, Remote Tool Execution, Remote Config

### How the agent actually works in remote mode (current desktop Zed)

Zed's AI agent architecture has a **split execution model** that the original plan
did not account for:

- **LLM communication is 100% local.** The `Thread` engine in `crates/agent/src/thread.rs`
  calls `model.stream_completion()` directly, which makes HTTPS requests to LLM provider
  APIs (Anthropic, OpenAI, Google, etc.) via reqwest. The `HeadlessProject` on the remote
  server has **zero agent, language model, or LLM provider infrastructure.** The
  `remote_server` binary's `Cargo.toml` has no dependency on `agent`, `agent_ui`,
  `language_model`, or `language_models`.

- **Tool execution proxies through `Project`.** Agent tools (read_file, edit_file, grep,
  terminal, list_directory, find_path) receive an `Entity<Project>` handle and use standard
  `Project` methods. In remote mode, `Project` transparently routes these operations via
  protobuf RPC to the `HeadlessProject` on the server. No tool contains explicit
  `is_remote()` checks — the proxy is transparent.

- **Settings and API keys resolve from the LOCAL machine only.** This is the problem.
  On desktop Zed, `language_models` config and API keys (from OS keychain + environment
  variables like `ANTHROPIC_API_KEY`) are read from the local client. The remote host's
  settings and environment are ignored for AI features.

### Known bugs at the local/remote boundary

The transparent `Project` proxy works well for file operations but has produced real bugs
where the agent assumes local context:

- **Terminal shell detection (Issue #35603):** Used `which::which("bash")` locally to
  select the shell, then ran that shell on the remote — wrong shell on cross-platform
- **File existence checks (Issue #30106):** `edit_file` tool hit "remote worktrees can't
  yet check file existence" — missing RemoteWorktree capability
- **Proxy env leakage (Issue #38392):** Local `HTTP_PROXY`/`HTTPS_PROXY` were applied
  to ACP agent servers on the remote where the network context differs
- **External agents on remote (Issue #47910, fixed):** ACP Registry agents weren't
  discovered on the remote server
- **Claude Code on remote (Issue #47362, open):** Claude Code external agent fails to
  initialize in remote sessions

### Architecture decision for iPad

**Zed agent (built-in) — works with stock server (Phase 3A):**

The iPad runs the Thread engine locally, identical to desktop. LLM calls go directly from
iPad to providers over HTTPS (reqwest, no platform deps). Tool invocations proxy through
`Project` → remote host (existing protocol). API keys come from iPad-local settings or
Zed Pro account auth (`ASWebAuthenticationSession` for OAuth/PKCE).

The agent code doesn't know or care that it's on an iPad — it gets settings from
`SettingsStore` and credentials from the credential store as it always does.

Limitation: iOS kills backgrounded apps after ~30s. Agent turns that outlast the user's
attention span get killed. Acceptable for v1 with a warning.

**External agents (Claude Code, Gemini CLI) — via SSH channels (Phase 3A):**

External agents are discovered on the remote host and advertised to the client via
`ExternalAgentsUpdated` proto. On desktop, the client spawns the binary locally. On iPad,
we open an SSH channel and exec the agent binary on the remote host — same technique as
the terminal (Phase 3.7). ACP messages flow over the channel's stdio. No server protocol
changes needed.

**Remote Zed agent execution — future enhancement (Phase 3B):**

Move `Thread.run_turn()` and LLM streaming to the headless server so agent turns survive
iOS backgrounding. Requires new proto messages and agent initialization in
`HeadlessProject` (which currently has none). Also requires settings sync so the server
has API keys.

### What NOT to use: Authentication Services Framework

The plan does **not** need `AuthenticationServices.framework` for general agent auth.
That framework handles Sign in with Apple, passkeys, and `ASWebAuthenticationSession`
for web OAuth. The actual frameworks needed:

| Need | Framework | Notes |
|---|---|---|
| SSH key storage on iPad | `Security.framework` (Keychain Services) | Already in plan |
| LLM API keys | None — configured in iPad-local settings or via Zed Pro | Zed Pro avoids key management |
| Zed Pro OAuth (if supported) | `AuthenticationServices` (`ASWebAuthenticationSession`) | Standard OAuth/PKCE |
| LLM provider OAuth (e.g. Google/Gemini) | `AuthenticationServices` (`ASWebAuthenticationSession`) | Narrow case, v2 |

---

## Work Items & Phase Plan

### Phase 0 — Build Infrastructure

**Goal:** `cargo build --target aarch64-apple-ios` succeeds; thin Swift host app compiles
and launches on device/simulator showing a black screen without crashing.

**Tasks:**

1. Create `crates/zed_ios/` with `crate-type = ["staticlib"]` and a single
   `pub extern "C" fn zed_ios_main()` entry point.

2. Audit all `build.rs` scripts across the workspace for macOS-isms (hardcoded `xcrun
   metal`, macOS SDK paths, CoreFoundation macOS-only linkage). Gate with
   `#[cfg(target_os = "macos")]` or generalize to `target_vendor = "apple"`.

   **Specific known issues in `crates/gpui/build.rs`:**
   - Metal shader compilation hardcodes `-sdk macosx`. Must become
     `-sdk iphoneos` or `-sdk iphonesimulator` based on target.
   - The `runtime_shaders` feature flag bypasses offline compilation entirely
     (uses `device.new_library_with_source()` at runtime) — **use this as an
     interim workaround** during early iOS development.
   - GCD bindgen from `src/platform/mac/dispatch.h` links `framework=System` —
     verify this works with iOS SDK.

3. Add `[target.'cfg(target_os = "ios")'.dependencies]` blocks in relevant Cargo.toml
   files to exclude `node_runtime`, `lsp` (local spawn path), `task`, `dap`,
   `extension_host`, and the `git` CLI wrapper.

   **Recommended Cargo approach:** Feature flags with `default = ["desktop"]`:
   ```toml
   [features]
   default = ["desktop"]
   desktop = ["dep:terminal", "dep:lsp", "dep:node_runtime", ...]
   ios = ["dep:objc2-ui-kit", "dep:russh"]
   ```
   Build with: `cargo build --target aarch64-apple-ios --no-default-features --features ios`

4. Create the Xcode project at `ios/ZedApp.xcodeproj` with:
   - Deployment target: **iPadOS 17.0** minimum
   - `UIApplicationSceneManifest` in Info.plist with
     `UIApplicationSupportsMultipleScenes = YES`
   - Build phase that runs `cargo build --target aarch64-apple-ios --release` and
     links the resulting `.a`
   - `ZedApp-Bridging-Header.h` importing the C header generated from zed_ios
   - **Bundle required monospace fonts** (Fira Code, JetBrains Mono, etc.) and
     register via `UIAppFonts` in Info.plist — iOS does not have
     `/Library/Fonts/` and has far fewer built-in monospace fonts than macOS
     (only SF Mono, Menlo, Courier New are guaranteed)

5. Add a macOS CI job (GitHub Actions, `macos-latest` runner, Xcode 16+) that builds
   the iOS static lib and runs `xcodebuild` for the simulator target.
   - Pin Xcode version via `maxim-lobanov/setup-xcode` action
   - Note: macOS runners cost ~10x more per-minute than Linux runners
   - Consider `cargo-dinghy` for running tests on iOS simulators

**Success criterion:** `xcodebuild -scheme ZedApp -destination 'platform=iOS
Simulator,name=iPad Pro 13-inch'` exits 0.

---

### Phase 1 — GPUI iOS Platform Layer

**Goal:** GPUI renders the Zed UI on an iPad screen at 120fps. Software keyboard works.
Touch and trackpad input reaches the editor.

#### 1.1 — Create `crates/gpui/src/platform/ios/`

Module files to create (model directly on `platform/mac/`):

```
platform/ios/
  mod.rs
  platform.rs        — IosPlatform implementing the Platform trait (~40+ methods)
  window.rs          — IosWindow implementing PlatformWindow (~37+ methods)
  display.rs         — IosDisplay implementing PlatformDisplay (4 methods)
  dispatcher.rs      — GCD-backed foreground/background executor
  text_system.rs     — CoreText font loading + CTTypesetter shaping
  events.rs          — UIEvent / UITouch / UIKey → PlatformInput
  metal_renderer.rs  — thin wrapper reusing mac/metal_renderer.rs logic
```

**Prior art:** The `gpui-mobile` project (github.com/itsbalamurali/gpui-mobile) has
implemented `gpui::Platform` for iOS using wgpu/Metal + CoreText, with touch input,
momentum scrolling, safe area insets, and dark mode support. This is not official Zed code
but demonstrates the porting path is viable and can serve as reference.

**IosPlatform** (`platform.rs`):
- Replaces `NSApplication` with `UIApplication` — do NOT link AppKit on iOS
- The `Platform` trait requires ~40+ method implementations: application lifecycle
  (`run`, `quit`, `restart`, `hide`), window management (`open_window`, `displays`,
  `window_appearance`), clipboard, credentials/keychain, file dialogs, menus, URL
  handling, executors, and text system access
- Platform injection happens via `Application::with_platform()` — add an `ios` case
  to `gpui_platform::current_platform()`
- Run loop integration: call our async executor via `DispatchQueue.main.async` blocks
  rather than `CFRunLoop` sources
- `open_url` → `UIApplication.shared.open(_:)`
- `reveal_path` → not applicable on iOS; no-op or show a "file is on remote host" message
- `prompt_for_paths` → `UIDocumentPickerViewController` (used only for SSH key import,
  not project opening)
- All `NSPasteboard` calls → `UIPasteboard`

**IosWindow** (`window.rs`):
- Backed by a `UIWindow` containing a custom `UIView` subclass (`ZedMetalView`)
- `ZedMetalView` overrides `+layerClass` to return `CAMetalLayer.self`
- **UITextInput — RECOMMENDED: full protocol implementation for v1** (not the
  hidden UITextField trampoline). Rationale:
  - The trampoline pattern (used by SDL2) has documented bugs with hardware keyboards
    on iPadOS and poor IME/CJK composition support
  - Flutter implements full `UITextInput` and gets correct marked text, autocorrect
    disable, smart quote disable, and `UITextInteraction` selection handles
  - A code editor benefits enormously from proper `UITextInput`: correct cursor
    positioning, selection rects for find-and-replace highlights, and stable
    interaction with iPadOS text editing menus
  - The protocol requires ~30+ methods but most have trivial implementations for a
    code editor (e.g., return `nil` for `textStylingAtPosition`, empty rect arrays
    for `selectionRectsForRange`)
  - If full protocol proves too costly for v1, fall back to the trampoline with the
    understanding that CJK input will be broken
- Implements `pressesBegan:withEvent:` and `pressesEnded:withEvent:` to capture
  hardware keyboard events — this catches keys `UIKeyCommand` misses, including
  Escape, modifier-only presses, and bare character keys without modifiers
  - **Note:** System-reserved shortcuts that CANNOT be captured on iPadOS:
    Cmd+Tab, Cmd+H, Cmd+Space, Globe+key combinations, screenshot shortcuts
  - **Good news:** Ctrl-based shortcuts ARE fully available on iPadOS (unlike macOS
    where many are intercepted by the system)
- Responds to `UIWindowScene` geometry change notifications to update window bounds
  (Stage Manager resize)

**IosDispatcher** (`dispatcher.rs`):
- `ForegroundExecutor`: schedules closures via `DispatchQueue.main.async`
- `BackgroundExecutor`: schedules closures via `DispatchQueue.global(qos:
  .userInitiated).async`
- Nearly identical in shape to `platform/mac/dispatcher.rs`; the Mac version uses
  `CFRunLoop` integration which is unnecessary here

**IosTextSystem** (`text_system.rs`):
- Font loading: `CTFontCollectionCreateFromAvailableFonts` — same CoreText API as
  macOS; the font discovery path is the main delta (no `/Library/Fonts` on iOS)
- Text shaping: `CTTypesetter` — identical API to macOS
- Glyph rasterization: `CGContext` with `CGFont` — identical API to macOS
- **Coordinate system note:** iOS UIKit uses top-left origin; macOS uses bottom-left.
  Apply `CGAffineTransform` flip when rendering CoreText output to Metal textures.
- The `core-text = "21"` Rust crate and Zed's `font-kit` fork use CoreText's loader;
  minor patches needed for iOS target detection
- Expected to be ~85–90% shared code with `platform/mac/text_system.rs`; extract
  shared logic into `platform/apple/text_system_shared.rs` if duplication is excessive

**events.rs:**
- `UITouch` → `MouseDown` / `MouseUp` / `MouseMove` (single finger = left button)
- `UIPanGestureRecognizer` (two finger) → `ScrollWheel`
- `UIPointerInteraction` hover → `MouseMoved` (for trackpad/mouse pointer)
  - Use `UIPointerStyle.verticalBeam` for the text I-beam cursor in the editor area
  - Define `UIPointerRegion` per view area for different cursor behaviors
- `UIPointerInteraction` click → `MouseDown` / `MouseUp`
- Hardware key via `pressesBegan:` → `KeyDown` with modifier flags extracted from
  `UIKey.modifierFlags`
- Long press → right-click context menu (maps to `MouseDown` with
  `button: MouseButton::Right`)

**metal_renderer.rs:**
- `CAMetalLayer` drawable acquisition is identical on iOS and macOS
- Reuse the shader compilation and pipeline setup from
  `platform/mac/metal_renderer.rs` — **shaders are 100% portable** (verified:
  standard MSL only, no macOS-specific Metal features used)
- **Key deltas from the macOS renderer:**
  - **CVDisplayLink → CADisplayLink**: macOS uses Core Video's `CVDisplayLink` for
    frame timing; iOS uses `CADisplayLink` exclusively. `CADisplayLink` is actually
    simpler — it fires on every display refresh and handles ProMotion (24–120Hz)
    adaptively via `preferredFrameRateRange`
  - **presentsWithTransaction**: Zed uses `presentsWithTransaction = true` during
    macOS window resize for compositor synchronization. Verify behavior on iOS; the
    pattern works but timing characteristics differ under ProMotion
  - **Texture memory**: iOS has no swap space. Metal textures count toward Jetsam
    memory footprint. Implement aggressive atlas eviction when backgrounded.
  - On macOS, shaders are compiled at build time via `xcrun metal`; on iOS the same
    `.metal` files compile via Xcode build system or via `xcrun -sdk iphoneos metal`.
    Use the `runtime_shaders` feature during development to bypass build-time
    compilation entirely.

#### 1.2 — Conditional Compilation Gating

Add `#[cfg(target_os = "ios")]` to `crates/gpui/src/platform/mod.rs` to select the iOS
platform implementation, following the exact same pattern as the existing
`#[cfg(target_os = "macos")]` / `#[cfg(target_os = "linux")]` / `#[cfg(target_os = "windows")]`
/ `#[cfg(target_family = "wasm")]` branches.

**Note:** The WASM platform (merged February 2026 via PR #50228) is actually the most
recently written backend and demonstrates the minimum viable `Platform` implementation.
Study it for lessons about async architecture constraints (no blocking `Mutex`/`RwLock` on
the main thread) that are equally relevant to iOS.

**Success criterion:** A GPUI test window displaying the Zed theme renders on the iPad
simulator at the correct pixel density with no artifacts. `UIPointerInteraction` moves the
cursor. Hardware keyboard input reaches the editor.

---

### Phase 2 — Connection & Session Management

**Goal:** User can enter a hostname, authenticate via SSH key, and open a remote project. The
project panel populates with the remote filesystem. Files open in the editor.

#### 2.0 — SSH Transport Replacement (CRITICAL PATH)

Before any connection UI work, the system-ssh transport must be replaced with an
embedded implementation. See the "⚠️ CRITICAL: SSH Transport Must Be Rewritten"
section above for full details.

New file: `crates/remote/src/transport/ios_ssh.rs` (or a new `crates/ssh-transport/` crate)

The `RemoteConnection` trait provides the abstraction point. Implement `IosSshConnection`
that:
- Establishes TCP connections and performs SSH handshake using `russh`
- Authenticates via in-memory key bytes loaded from iOS Keychain
- Opens channels for the Zed remote protocol (stdin/stdout of `zed --headless`)
- Multiplexes multiple channels over a single SSH connection
- Handles keepalive, timeout, and reconnection

**This transport should also work on macOS** as an alternative to the system-ssh path,
enabling shared testing and a migration path for desktop Zed.

#### 2.1 — Connection Manager UI

New GPUI-based screens (pure Rust, no UIKit — these render via GPUI like all other Zed UI):

**HostListView** — root view when no session is active:
- Loads saved hosts from SettingsStore (stored in
  `Application Support/zed/remote_hosts.json`)
- Shows hostname, last-connected timestamp, connection status badge
- "+" button → `AddHostView`
- Tap host → initiate connection

**AddHostView:**
- Fields: hostname/IP, port (default 22), username, key selector (dropdown of Keychain
  labels), optional display name
- "Test Connection" button
- "Save & Connect"

**ConnectionStatusView** (toolbar component, always visible during active session):
- Shows: host display name, latency (ping the host every 10s), connection health icon
- Tap → dropdown with "Disconnect", "Reconnect", "Host Info"

**Success criterion:** Connect to a `zed --headless` instance on a Mac Studio. Project panel
shows remote files. Tap a file → it opens in the editor. Edit and save. See LSP
completions and git diff decorations (from remote). Auto-connect to a previously saved
host on launch.

---

### Phase 3 — Editor, Terminal, Agent & Debug

**Goal:** Full editing session with all major panels. LSP completions (from remote), git
decorations (from remote), remote terminal, agent panel with LLM integration, debug panel,
edit predictions, collab panel, and git branch/worktree management.

Phase 3 is split at a **protocol cut line**. Phase 3A items work with a stock
`zed --headless` binary (the same one desktop Zed installs). Phase 3B items require
running a modified server with new protocol messages — which means setting up a dev
workflow for building and deploying a custom `remote_server` binary to the remote host.

#### ═══ Phase 3A — Stock Server (no protocol changes) ═══

#### 3.1 — Editor Interaction

The `editor` crate itself needs no changes — it operates on buffer state proxied from the
remote host exactly as it does in Zed's existing SSH remote mode. The `Project` struct's
`ProjectClientState::Remote` variant transparently proxies all operations. The work here is
validation and polish:

- Verify that `CompletionMenu`, `HoverPopover`, `DiagnosticIndicator`, and
  `CodeActionMenu` render correctly on iPad screen sizes
- Ensure that long-tap on a token triggers the right-click context menu (go to definition,
  find references, etc.) — this requires the long-press → right-click input mapping from
  Phase 1 to be wired correctly
- Scrolling performance: profile the Metal render path on device; the editor's `DisplayMap`
  pipeline is CPU-intensive for large files; ensure it stays off the main thread
- **Memory management:** Monitor `os_proc_available_memory()` (iOS 13+) to detect
  approaching Jetsam limits. Large files may need buffer eviction strategies that desktop
  Zed never needed.

#### 3.2 — Settings Path Adjustment

In `crates/paths/src/lib.rs` (or wherever `config_dir()`, `data_dir()`, `cache_dir()` are
defined), add an iOS branch:

```rust
#[cfg(target_os = "ios")]
pub fn config_dir() -> PathBuf {
    // [[NSFileManager defaultManager]
    //   URLsForDirectory:NSApplicationSupportDirectory
    //   inDomains:NSUserDomainMask]
    ios_support_dir().join("zed")
}
```

The iOS sandbox directories to use:
- Config/settings: `Application Support/zed/`
- Workspace DB: `Application Support/zed/db/`
- Logs: `Library/Logs/zed/`
- Caches: `Library/Caches/zed/`
- Temp: `tmp/`

#### 3.3 — Debug Panel

The debug panel (`debugger_ui`) supports remote debugging out of the box via `DapStore`'s
`RemoteDapStore` mode, which proxies DAP (Debug Adapter Protocol) operations through the
remote connection to the headless server. The headless server manages debug adapter
processes (CodeLLDB, debugpy, etc.) — the iPad never spawns debugger subprocesses.

**Crates to add to zed_ios:**
- `debugger_ui` — panel UI, session views, breakpoint visualization
- `debugger_tools` — debug logging (`dap_log`)
- `dap_adapters` — built-in adapter definitions (the iPad doesn't run them, but needs
  the type definitions for remote adapter discovery)

**Initialization:**
```rust
debugger_ui::init(cx);
debugger_tools::init(cx);
dap_adapters::init(cx);
```

Plus in the workspace observer:
```rust
project::debugger::breakpoint_store::BreakpointStore::init(...)
project::debugger::dap_store::DapStore::init(...)
```

**iOS-specific concerns:**
- The `dap` crate's `StdioTransport` and `TcpTransport` use `std::process::Command` to
  spawn debug adapter binaries. These code paths are only exercised in `LocalDapStore`
  mode — the iPad uses `RemoteDapStore` exclusively. May need `#[cfg(not(target_os = "ios"))]`
  gates on the local transport if the crate fails to compile for iOS.
- `dap_adapters` references adapter binaries and download URLs — these are used by the
  remote host, not the iPad. Should compile fine as data definitions.
- `debugger_ui` depends on `terminal_view` for the debug console — this must be wired up
  as part of the terminal panel work (Phase 3.7).

**Keybindings:**
- `F5` or `cmd-shift-d` → start/continue debugging
- `F9` → toggle breakpoint
- `F10` → step over, `F11` → step into

#### 3.4 — Git Branch Picker & AI Worktree Selection

The git branch picker and worktree picker already support remote connections via the
existing protocol (`proto::GitGetBranches`, `proto::GitGetWorktrees`). Both live in
`git_ui` and use `Project`/`Repository` abstractions that transparently proxy to the
remote host.

**Components:**
- `git_ui::branch_picker::BranchList` — modal/popover for switching branches
- `git_ui::worktree_picker::WorktreeList` — picker for git worktrees
- `git_ui::git_picker::GitPicker` — unified tabbed picker (Branches / Worktrees / Stash)
- All registered via `git_ui::init(cx)` which is already called in `init_zed()`

**AI worktree selection** (in `agent_ui`):
- `StartThreadIn` enum: `LocalProject` vs `NewWorktree`
- When `NewWorktree` is selected, the agent creates a new git worktree for its changes
- The worktree picker in the agent panel uses the same `git_ui` worktree infrastructure
- Requires a git repository in the workspace (validated in `set_start_thread_in()`)
- Gated behind `AgentV2FeatureFlag` on desktop

**iPad status: Works with stock server** — all git operations proxy through existing
remote protocol messages. No additional crates needed beyond `git_ui` (already included).

**Keybindings:**
- `cmd-shift-b` → open branch picker

#### 3.5 — Collab Panel

The collab panel (`collab_ui::collab_panel`) manages channels, contacts, and calls. It
connects to **Zed's collab server** (zed.dev), not to the SSH remote host — so it's
independent of the `zed --headless` connection.

**Dependency challenge:**

`collab_ui` pulls in `call`, which depends on:
- `livekit_client` — native audio/video codec bindings (NOT iOS-compatible as-is)
- `audio` — platform-specific audio backend
- `gpui` with `screen-capture` feature — desktop-only

The `title_bar` crate (also initialized by `collab_ui::init`) depends on `livekit_client`
and `platform_title_bar`.

**Approach for iPad:**

Feature-gate the audio/video/screen-capture dependencies. The iPad doesn't need voice
calls or screen sharing for v1 — it needs the channel list, contacts, and chat:

1. Make `livekit_client`, `audio`, and `screen-capture` optional in `call` crate via
   a `voice` or `media` feature flag
2. Gate `call_stats_modal` and call-related UI on that feature
3. The collab panel's channel list, contact list, and channel notes work without audio
4. `title_bar` needs similar treatment or can be excluded entirely on iPad (we have our
   own iOS titlebar)

**Requires Zed account auth:** The collab panel needs authentication with zed.dev. This
means implementing `ASWebAuthenticationSession` for OAuth/PKCE on iOS — same work
as Zed Pro LLM access (see Phase 3.9).

**Crates to add:**
- `collab_ui` (with feature-gated media deps)
- `channel` — channel store, channel buffers
- `call` (with feature-gated `livekit_client`, `audio`)
- `notifications` — already included

**Graceful degradation:** The panel shows "Connect to view notifications" when
disconnected from zed.dev. Channel data is cached in memory across reconnections.

#### 3.6 — Edit Prediction (Inline Completions)

Zed's edit prediction system has multiple providers. Most are HTTP-based and will work
on iPad without modification. The Copilot provider is the exception — it spawns a Node.js
LSP server.

**Provider compatibility on iPad:**

| Provider | Transport | iPad Status |
|---|---|---|
| Zed (Zeta) | HTTPS to Zed cloud API | Works |
| Zed (Mercury) | HTTPS to Zed cloud API | Works |
| Codestral | HTTPS to Mistral API | Works |
| Ollama | HTTP to local/remote server | Works (if server reachable) |
| OpenAI-compatible | HTTPS | Works |
| Copilot | Node.js LSP subprocess | **Blocked** — requires local process |

**Crates to add to zed_ios:**
- `edit_prediction` — core engine, Zed/Codestral/OpenAI delegates
- `edit_prediction_types` — trait definitions
- `edit_prediction_context` — context analysis for predictions
- `edit_prediction_ui` — UI components (inline ghost text, accept/reject)
- `codestral` — Codestral (Mistral) provider (HTTPS only)
- `cloud_llm_client` — Zed cloud prediction API client

**Exclude from iOS:**
- `copilot` — spawns Node.js subprocess (depends on `node_runtime`, `lsp`)
- `copilot_ui` — Copilot auth UI (depends on `copilot`)

**Initialization:**
```rust
edit_prediction_ui::init(cx);
// edit_prediction_registry::init() — need to check if this can be
// adapted to skip the Copilot provider path on iOS
edit_prediction::init(cx);
```

**iOS-specific work:**
- The `edit_prediction_registry` module (in `crates/zed/src/zed/`) assigns providers to
  editors based on settings. This logic lives in the desktop `zed` crate, not a library
  crate. Need to either extract it to a shared crate or duplicate the non-Copilot
  registration logic in `zed_ios`.
- Settings: users who have `"edit_predictions": { "provider": "copilot" }` will get no
  completions on iPad. Consider a settings overlay that maps `copilot` → `zed` on iOS,
  or show a notification suggesting they switch providers.

**Keybindings:**
- `Tab` → accept prediction
- `Escape` → dismiss prediction
- `Alt+]` / `Alt+[` → cycle through predictions

#### 3.7 — Remote Terminal

The terminal panel on desktop shells out to `ssh` for each terminal (`build_command()` in
`subprocess_ssh.rs`). On iPad we can't spawn subprocesses, but we don't need to — **SSH's
native channel multiplexing** gives us everything we need without any server protocol
changes.

Our `RusshRemoteConnection` stores the SSH handle as
`Arc<tokio::sync::Mutex<SessionHandle>>`. The existing Zed protocol runs on one channel
(opened by `start_proxy()`). We can open additional session channels on the same
connection for terminal sessions — this is core SSH protocol, handled entirely by the
remote host's SSH daemon.

**Implementation:**

```rust
// Open a new session channel (alongside the Zed protocol channel)
let handle = self.session.lock().await;
let channel = handle.channel_open_session().await?;
drop(handle); // Release lock — channel is independent

// Allocate PTY and start shell
channel.request_pty("xterm-256color", cols, rows, 0, 0, &[]).await?;
channel.request_shell().await?;

// Bidirectional I/O via split (same pattern as start_proxy)
let (read_half, write_half) = channel.split();

// Terminal resize
channel.window_change(new_cols, new_rows, 0, 0).await?;
```

**Client-side rendering:**
- VT100/VT220 parsing via `alacritty_terminal::Term<ZedListener>` using `vte` crate
  (Paul Williams' state machine, ~2.9K SLoC, `no_std`, zero platform deps)
- `TerminalBuilder::new_display_only()` already creates a terminal without a PTY — used
  by the REPL subsystem. iPad uses this mode with I/O from the SSH channel.
- Terminal rendering via GPUI's text system — works as-is

**Integration point:** `RusshRemoteConnection` needs a public method to open terminal
channels. Something like:
```rust
impl RusshRemoteConnection {
    pub async fn open_terminal(
        &self,
        working_directory: Option<&str>,
        cols: u32,
        rows: u32,
        env: &[(String, String)],
    ) -> Result<TerminalChannel> { ... }
}
```

Where `TerminalChannel` wraps the russh channel split halves and exposes `write()`,
`read()`, and `resize()`. The `project::terminals` module routes here on iPad instead of
calling `build_command()`.

**Touch keyboard accessory bar** (new component):
- A GPUI element rendered above the software keyboard
- Buttons: `Esc`, `Tab`, `Ctrl`, `↑`, `↓`, `←`, `→`, `~`, `/`, `|`, `-`
- Each button emits the appropriate escape sequence / control character
- Shown when terminal panel is focused and software keyboard is visible

#### 3.8 — Extensions & Tree-sitter Grammar Support

Zed's extensions provide Tree-sitter grammars (for syntax highlighting), language
configurations, themes, and optional WASM logic. On desktop, grammars are compiled to
WASM and loaded via **wasmtime** (JIT). iOS prohibits JIT compilation everywhere — except
inside **WKWebView's JavaScriptCore**, which has a system-level exemption.

**What works on iOS without changes:**
- Extension downloading from `extensions.zed.dev` (HTTP + tar.gz unpack to sandbox)
- Extension metadata/index scanning (`ExtensionStore::rebuild_extension_index()`)
- Language configuration loading (JSON — file matchers, indent rules, brackets)
- Theme loading (JSON/TOML — pure data, no WASM)

**What's blocked on iOS:**
- wasmtime WASM engine (JIT prohibited) — affects grammar loading and extension logic
- Extension subprocess spawning (`process:exec`, `npm:install` capabilities)
- Language server binary downloads from extensions

The codebase already has `#[cfg(not(target_os = "ios"))]` on the WASM store setup in
`crates/language/src/language.rs` (lines 88–137), with a fallback to native-only grammars.
But this only covers built-in grammars — extension grammars need WASM.

**WKWebView WASM runtime for grammars:**

Use a hidden `WKWebView` as a JIT-enabled WASM execution environment for Tree-sitter
grammars. This is the standard iOS approach for apps that need WASM JIT (used by
iSH, a]Shell, Pyto, etc.).

Architecture:
```
Extension installed → grammar.wasm on disk
    ↓
iOS grammar loader reads WASM bytes
    ↓
Hidden WKWebView loads web-tree-sitter JS + grammar WASM
    ↓
Native code sends source text to WKWebView via WKScriptMessageHandler
    ↓
web-tree-sitter parses in JavaScriptCore (JIT enabled)
    ↓
Parse tree (node types + ranges) returned to native via postMessage/evaluateJavaScript
    ↓
Native Tree-sitter highlight queries applied to parse tree
    ↓
Editor renders syntax colors
```

**Implementation components:**

1. **`crates/gpui_ios/src/wasm_grammar_host.rs`** — manages a hidden `WKWebView`
   - Loads `web-tree-sitter` JavaScript module (bundled in app)
   - Accepts grammar WASM bytes, creates parser instances
   - Exposes async `parse(source: &str) -> ParseTree` interface
   - Uses `WKScriptMessageHandler` for native↔JS message passing

2. **Grammar bridge trait** — abstract over native vs WKWebView parsing
   ```rust
   #[cfg(target_os = "ios")]
   trait GrammarRuntime {
       fn load_grammar(&self, name: &str, wasm_bytes: &[u8]) -> Result<GrammarHandle>;
       fn parse(&self, grammar: &GrammarHandle, source: &str) -> Result<Tree>;
   }
   ```

3. **Language registry integration** — `LanguageRegistry::get_or_load_grammar()` in
   `crates/language/src/language_registry.rs` already loads WASM bytes from disk.
   On iOS, route these bytes to the WKWebView host instead of wasmtime.

**Performance considerations:**
- JavaScriptCore JIT is fast but slower than native wasmtime (~2-5x overhead)
- Parse tree serialization across the JS↔native boundary adds latency
- Mitigate with: incremental parsing (Tree-sitter's edit API), background parsing,
  and coalescing rapid edits (reuse the 4ms batching from terminal)
- Consider keeping a pool of WKWebView instances for parallel grammar loading

**Extension logic (non-grammar WASM):**
Full extension WASM logic (slash commands, context servers, language server management)
also runs on wasmtime. For v1, **skip extension logic on iPad** — only load grammars,
language configs, and themes. Extensions that need WASM logic (e.g., custom formatters)
show as "not supported on iPad" in the extension panel. This can be revisited if the
WKWebView WASM host proves performant enough for general extension execution.

**Crates involved:**
- `extension` — manifest parsing, extension builder (skip WASM compilation on iOS)
- `extension_host` — extension store, download, index management
- `language` — grammar loading, parser setup (iOS conditional already exists)

**Exclude from iOS (for now):**
- `extension_host` WASM host subsystem (wasmtime engine, WIT bindings)
- Extensions declaring `process:exec` or `npm:install` capabilities

#### 3.9 — Agent Panel (Zed Agent)

The **Zed agent** (built-in Thread engine) works with the stock server:
- `Thread` engine (`crates/agent/src/thread.rs`) runs `run_turn()` locally on iPad
- LLM API calls (HTTPS) go directly from iPad to providers — no server involvement
- Tool invocations go through `Project` → remote proxy → headless server — same path
  desktop SSH remote already uses
- API keys: user configures in iPad-local settings, or signs into Zed Pro
  (`ASWebAuthenticationSession` for OAuth/PKCE — LLM calls route through `api.zed.dev`)

**What to wire up:**
- Compile agent crates into the iPad binary (`agent`, `agent_ui`, `agent_settings`,
  `language_model`, `language_models`, `anthropic`, `open_ai`, `acp_thread`)
- Initialize the agent panel in `init_zed()` — same as desktop

**External agents (Claude Code, Gemini CLI) via SSH channels:**

The headless server discovers installed agents and advertises them via
`ExternalAgentsUpdated` proto. When selected, the client gets the command via
`GetAgentServerCommand` proto. On desktop, the client spawns the binary locally
(`Child::spawn(path)` in `acp.rs:218`). On iPad, we instead **open an SSH channel and
exec the agent binary on the remote host** — same technique as the terminal (Phase 3.7).

The agent binary's stdio carries ACP (Agent Communication Protocol) messages. An SSH
channel gives us bidirectional byte streams, which is exactly what ACP needs. The flow:

```rust
// Reuse the same SSH handle as terminal channels
let handle = self.session.lock().await;
let channel = handle.channel_open_session().await?;
drop(handle);

// Exec the agent binary (command came from GetAgentServerCommand)
channel.exec(true, "/usr/local/bin/claude --mcp".into()).await?;

// Split for bidirectional ACP I/O
let (read_half, write_half) = channel.split();
// Pipe ACP JSON-RPC messages over read_half / write_half
```

On iPad, replace `Child::spawn()` in `acp.rs` with this SSH channel path. The agent
panel code and ACP protocol handling remain unchanged — only the transport layer differs.

**Limitation — iOS background execution:**
iOS kills backgrounded apps after ~30 seconds. Agent turns can take minutes. If the user
switches apps mid-turn, the turn is killed. This is acceptable for v1 with a clear
warning: "Keep Zed in the foreground while the agent is working." The remote Zed agent
enhancement (Phase 3B) solves this by moving the Thread engine to the server.

**Crates included in iOS build:**
- `agent` — Thread engine, tool definitions
- `agent_ui` — agent panel UI
- `agent_settings` — configuration resolution
- `language_model` — model trait + registry
- `language_models` — provider implementations (HTTPS)
- `anthropic`, `open_ai`, etc. — HTTP client crates
- `acp_thread` — ACP protocol bridge for conversation UI

#### ═══ Phase 3B — Modified Server (new protocol messages) ═══

Phase 3B items require building and deploying a custom `remote_server` binary to the
remote host. This means adding new protobuf messages to the remote protocol and running a
version of `zed --headless` that includes them. Getting this dev workflow set up (build
the server, deploy it, ensure the iPad client and server versions match) is a prerequisite
for all 3B work.

#### 3.10 — Remote Zed Agent Execution (survives backgrounding)

Move the Thread engine to the remote host so Zed agent turns survive iOS backgrounding.
The iPad becomes a thin UI shell — it sends prompts and renders streamed responses, but
the actual LLM calls and tool invocations happen on the server.

New protocol messages needed:
- `proto::AgentStartThread` — create a thread on the server
- `proto::AgentSendMessage` — send a user message to a server-side thread
- `proto::AgentStreamEvent` — server→client streaming of assistant responses, tool use
  status, file diffs, terminal output
- `proto::AgentListThreads` — enumerate saved threads on the server
- `proto::AgentResumeThread` — reconnect to an in-progress thread after app relaunch

Server-side changes:
- `HeadlessProject` hosts a `Thread` instance and runs `run_turn()` locally
- LLM API calls happen from the server, which means the server needs API keys

**Agent settings sync (required for remote Zed agent):**

The remote host needs API keys and agent configuration. Two approaches, not mutually
exclusive:

*Approach A — Forward settings from host:*
The user configures `~/.config/zed/settings.json` and environment variables on their Mac
Studio. At connection time, the iPad reads `agent_settings` and LLM provider API keys
from the remote host via new proto messages (`proto::GetAgentSettings`,
`proto::GetEnvironmentVariables`). The iPad's `SettingsStore` applies these as an overlay.

*Approach B — Zed Pro account auth:*
LLM calls already go through Zed's proxy when using Zed Pro (from Phase 3.9). For remote
execution, the server can use the same Zed Pro credentials to make LLM calls.

For remote execution, Approach A is essential — the server must have the API keys to
make LLM calls. Approach B complements it for users who prefer Zed Pro.

#### 3.11 — Notifications

Zed has three notification layers, each with different iOS considerations:

**Layer 1: Toast notifications (ephemeral, in-app)**
- Status toasts for git operations, build results, agent actions, settings changes
- Rendered via GPUI `StatusToast` component in the workspace toast layer
- Duration: ~10 seconds, dismissible
- **iPad status: Works as-is** — no platform dependencies, pure GPUI rendering
- Already included via the `notifications` crate (dependency of `workspace`)

**Layer 2: Workspace notifications (inline prompts)**
- Error message prompts, language server prompts, custom notifications
- Rendered inline in the workspace area
- **iPad status: Works as-is** — part of the `workspace` crate

**Layer 3: Collaboration notification panel (server-backed)**
- Contact requests, channel invitations, project sharing notifications
- Requires connection to Zed's collab server (separate from the SSH remote connection)
- Shows "Connect to view notifications" when disconnected
- **iPad status: Requires collab server authentication** — defer unless/until Zed account
  auth is implemented (see Phase 3.9)

**Crates involved:**
- `notifications` — `NotificationStore`, `StatusToast` (already a transitive dependency)
- `collab_ui` — `NotificationPanel`, `IncomingCallNotification`,
  `ProjectSharedNotification` (heavy dependency chain: `channel`, `client`, `call`)

**iOS push notifications — brainstorm:**

iPadOS supports `UNUserNotificationCenter` for local notifications and APNs for push.
Potential uses:

| Scenario | Type | Utility | Recommendation |
|---|---|---|---|
| SSH connection lost | Local | Medium — user may have backgrounded the app | **Yes for v1** — schedule a local notification when the app enters background with an active session and the connection drops |
| SSH reconnected | Local | Low — app will show reconnection banner when foregrounded | Skip |
| Long-running agent task complete | Local | High — agent may run for minutes; user switches apps | **Yes for v1** — fire when agent thread completes a turn while app is backgrounded |
| Collab invite / channel message | Push (APNs) | High — real-time collaboration | **Defer to v2** — requires server-side APNs integration |
| Remote build complete | Local | Medium — could monitor a build command in terminal | **Defer** — needs terminal output pattern matching |
| Jetsam memory warning | Local | Low — if we're about to be killed, notification won't help | Skip |
| Remote server went offline | Push (APNs) | High — server monitoring use case | **Defer to v2** — requires server-side health reporting |

**Implementation for v1 local notifications:**

```rust
// In crates/zed_ios/src/local_notifications.rs
// Wrap UNUserNotificationCenter via objc2-user-notifications

fn schedule_disconnect_notification() {
    // "Zed: Connection to {host} lost"
    // Trigger: 5 seconds after connection drops while backgrounded
    // Auto-cancel if connection restores before trigger
}

fn schedule_agent_complete_notification() {
    // "Zed: Agent finished editing {n} files"
    // Trigger: immediately when agent turn completes while backgrounded
}
```

Request notification permission at first SSH connection (not at launch — Apple rejects
apps that ask for notification permission without clear context).

**Success criterion:** Open a file on the remote host, edit it, save it (save goes to host via
remote protocol), see git diff decorations in the gutter. Open the terminal panel, run `ls`,
see output. Disconnect WiFi mid-session, reconnect, continue editing. Open the agent panel,
send a prompt, see completions stream, verify tool invocations execute on the remote host.
Start a debug session, hit a breakpoint, inspect variables. See edit prediction ghost text
while typing.

---

### Phase 4 — iPad UX Polish

**Goal:** Feels like a native iPad app. Ships on the App Store.

#### 4.1 — Stage Manager Multi-Window

- Each `UIWindowScene` = one Zed workspace connection (can be same host, different
  directory, or different host entirely)
- `SceneDelegate.swift` calls `zed_ios_open_window(scene_id: *const c_char)` on new
  scene activation
- `SceneDelegate` receives `NSUserActivity` for state restoration (reconnect to the right
  host/directory on app relaunch)
- Window sizing hints via `UIWindowSceneGeometryPreferencesIOS`: default to
  `CGSize(1100, 820)` on first open
- **iPadOS 26 note** (WWDC 2025): Stage Manager now available on all supported iPad
  models with macOS-style traffic light window controls

#### 4.2 — UIKeyCommand Discoverability HUD

Register `UIKeyCommand` instances on the root `UIViewController` for all major Zed actions
so they appear in the system HUD when the user holds ⌘:

```swift
override var keyCommands: [UIKeyCommand]? {
    return [
        UIKeyCommand(title: "Command Palette", action: #selector(commandPalette),
                     input: "p", modifierFlags: [.command, .shift]),
        UIKeyCommand(title: "Go to File", action: #selector(goToFile),
                     input: "p", modifierFlags: .command),
        // ... etc
    ]
}
```

These route through a thin Swift → Rust FFI bridge into GPUI's existing action dispatch
system.

#### 4.3 — Layout Adaptation

Breakpoints for layout adaptation (Stage Manager gives us arbitrary window widths):

| Window width | Layout |
|---|---|
| < 600pt | Sidebar hidden, toolbar-accessible; single pane |
| 600–900pt | Sidebar collapsed by default, toggleable |
| > 900pt | Full Zed layout: sidebar + editor + optional panel |

Implement via GPUI layout logic in `workspace.rs` — check `WindowContext::bounds()` at
render time and select the appropriate layout. No UIKit layout constraints involved; GPUI
handles this entirely.

#### 4.4 — App Store Entitlements

Required entitlements in `ZedApp.entitlements`:

```xml
<key>keychain-access-groups</key>             <!-- SSH key storage -->
<array>
    <string>$(AppIdentifierPrefix)com.zed.ZedApp</string>
</array>
```

**Note on network entitlements:** `com.apple.security.network.client` is a **macOS App
Sandbox entitlement only**. iOS apps can make outgoing network connections (including
raw TCP sockets for SSH) by default without any special entitlement.

Explicitly **NOT** requested (would cause App Review rejection):
- `com.apple.security.cs.allow-jit` (not needed for our use case)
- Any entitlement implying subprocess spawning
- `com.apple.developer.kernel.increased-memory-limit` (request only if profiling shows
  need — but note this entitlement IS available on iPad and may be worth requesting
  given Metal texture atlas memory usage)

#### 4.5 — Per-Project Settings Profiles for rust-analyzer

Settings profiles (`profiles` key in `settings.json`) currently only work in user settings
(`~/.config/zed/settings.json`), not in project settings (`.zed/settings.json`). This is a
Zed-wide limitation, not iOS-specific, but it directly impacts the iPad development workflow.

**The problem:** When working on Zed for iPad, rust-analyzer needs different configuration
(target `aarch64-apple-ios`, `--no-default-features`, `-p zed_ios`) than when working on
desktop Zed. Today, developers must configure an "iOS" profile in their personal user
settings. This can't be shared via `.zed/settings.json` in the repo.

**Why profiles are user-only today:**

The settings system has two content types:
- `UserSettingsContent` — has `profiles: IndexMap<String, SettingsContent>`, plus
  `release_channel_overrides` and `platform_overrides`
- `ProjectSettingsContent` — focused subset (LSP, terminal, DAP, language settings),
  **no profiles field**

The `SettingsStore::recompute_values()` merge order applies user profiles at step 7,
before project settings at step 9. Project settings always win over user profiles, which
is the correct precedence.

**What needs to change:**

1. Add `profiles: IndexMap<String, SettingsContent>` to `ProjectSettingsContent`
   (in `crates/settings_content/src/project.rs`)

2. Update `SettingsStore::set_local_settings()` to parse and store project-level profiles

3. Update `SettingsStore::recompute_values()` to apply project profiles after user
   profiles but before the final local settings merge — so project profiles can override
   user settings but project-level non-profile settings still win

4. Update `configured_settings_profiles()` to include project-level profiles in the
   selector, distinguishing them from user profiles (e.g., "iOS (project)" vs
   "Streaming (user)")

5. Decide on profile scope: should the active profile be global (current behavior) or
   per-workspace? Per-workspace would allow having an "iOS" profile active for the Zed
   repo while another workspace uses the default.

**Example `.zed/settings.json` with profiles:**
```json
{
  "profiles": {
    "iOS": {
      "lsp": {
        "rust-analyzer": {
          "initialization_options": {
            "cargo": {
              "target": "aarch64-apple-ios",
              "noDefaultFeatures": true,
              "buildScripts": {
                "overrideCommand": [
                  "cargo", "check", "--message-format=json",
                  "--target", "aarch64-apple-ios",
                  "--no-default-features", "-p", "zed_ios"
                ]
              }
            },
            "check": {
              "overrideCommand": [
                "cargo", "check", "--message-format=json",
                "--target", "aarch64-apple-ios",
                "--no-default-features", "-p", "zed_ios"
              ]
            }
          }
        }
      }
    }
  }
}
```

This would let any Zed contributor working on the iPad port activate the "iOS" profile
from the command palette without any per-user setup.

#### 4.6 — SSH Key Management (Keychain)

New file: `crates/zed_ios/src/keychain.rs`

- `store_ssh_key(label: &str, pem_bytes: &[u8]) -> Result<()>` — calls `SecItemAdd`
  with `kSecClassKey` (or `kSecClassGenericPassword` for simpler storage),
  `kSecAttrAccessible = kSecAttrAccessibleWhenUnlockedThisDeviceOnly`
- `load_ssh_key(label: &str) -> Result<Vec<u8>>` — calls `SecItemCopyMatching`
- `delete_ssh_key(label: &str) -> Result<()>` — calls `SecItemDelete`
- `list_ssh_key_labels() -> Result<Vec<String>>` — for the connection UI

SSH key import UI flow (Swift side):
1. Tap "Import SSH Key" → present `UIDocumentPickerViewController`
2. User selects file → read bytes → call `store_ssh_key` → confirm
3. Key never written to disk; raw bytes only exist in memory during import

#### 4.7 — Network Resilience

New file: `crates/zed_ios/src/network_monitor.rs`

- Wraps `NWPathMonitor` (Network.framework) via `objc2` crate bindings
- Emits `NetworkEvent::Available` / `NetworkEvent::Lost` into GPUI's event system
- `ConnectionManager` listens for `NetworkEvent::Available` and triggers reconnection
- Reconnection strategy: exponential backoff (1s, 2s, 4s, 8s, 16s, cap at 30s)
- During reconnection, editor is read-only with a banner ("Reconnecting to host…");
  existing buffer content remains visible because CRDT state is local

#### 4.8 — Server Host Key Verification

Currently `check_server_key` in `russh_ssh.rs` returns `true` for all keys. Before
shipping, implement proper host key verification:

- Consult a known_hosts file stored in the app sandbox
- On first connection to a host, prompt the user to accept the fingerprint
- Store accepted fingerprints in the known_hosts file
- On subsequent connections, verify the key matches; warn on mismatch

---

## Crates Excluded from iOS Build

These crates must not be linked into the iOS static lib. Enforce via Cargo.toml feature flags
or `[target.'cfg(target_os = "ios")'.dependencies]` exclusions:

| Crate | Reason |
|---|---|
| `node_runtime` | Spawns Node.js subprocess |
| `lsp` (local spawn path) | Spawns language server subprocesses |
| `task` | Spawns local task runner |
| `dap` | Spawns local debugger |
| `extension_host` (WASM host subsystem) | wasmtime engine requires JIT — use WKWebView for grammars |
| `git` (CLI wrapper path) | Spawns git binary |
| Terminal PTY spawn code | Spawns shell subprocess |

The remote proxy equivalents of all these (in `crates/remote/`) ARE included — that's how
the iPad communicates with the host that runs them.

**Crates INCLUDED in iOS build (not obvious):**

| Crate | Reason |
|---|---|
| `agent` | Thread engine, tool definitions — LLM calls work over HTTPS from iPad |
| `agent_ui` | Agent panel UI — renders via GPUI like all other panels |
| `agent_settings` | Agent config resolution — add remote settings sync overlay |
| `language_model` | Model trait + registry — needed for provider dispatch |
| `language_models` | Provider implementations (anthropic, openai, etc.) — HTTPS only |
| `anthropic`, `open_ai`, etc. | HTTP client crates — pure reqwest, no platform deps |
| `acp_thread` | ACP protocol bridge — needed for conversation UI |
| `edit_prediction` | Core edit prediction engine — Zed/Codestral/OpenAI delegates |
| `edit_prediction_types` | Trait definitions for edit prediction providers |
| `edit_prediction_context` | Context analysis for predictions |
| `edit_prediction_ui` | Inline ghost text UI, accept/reject actions |
| `codestral` | Codestral (Mistral) provider — pure HTTPS |
| `cloud_llm_client` | Zed cloud prediction API client |
| `debugger_ui` | Debug panel UI — uses `RemoteDapStore` on iPad |
| `debugger_tools` | Debug logging |
| `dap_adapters` | Adapter type definitions (iPad doesn't run adapters locally) |
| `notifications` | `NotificationStore`, `StatusToast` — already transitive dep |
| `extension` | Manifest parsing, extension metadata — no WASM |
| `extension_host` (store/download) | Extension download, index management — HTTP + filesystem only |
| `language` | Grammar loading, parser setup — iOS uses WKWebView WASM host |

**Exclude from iOS:**
- `extension_host` WASM host — wasmtime engine, WIT bindings (JIT prohibited on iOS)
- Extensions with `process:exec` or `npm:install` capabilities (subprocess spawning)
- `copilot` — spawns Node.js LSP server for Copilot completions
- `copilot_ui` — Copilot auth UI (depends on `copilot`)
- `subprocess_ssh` — system-ssh transport (replaced by `russh_ssh` on iOS)
- `collab_ui` — notification panel + call UI (heavy deps, defer to v2)

---

## Key Constraints & Guardrails

**Never do these on the iOS target:**
- Call `posix_spawn`, `fork`, `execve`, or `Process()` / `NSTask` — iOS will silently kill
  your app or reject it from the App Store (kernel-level enforcement, not just policy)
- Call `std::process::Command` — this is the Rust wrapper around `posix_spawn`
- Access paths outside the app sandbox container without a security-scoped bookmark
- Write SSH key material to disk — Keychain only
- Assume `~/.ssh/` exists or is accessible
- Use `NSApplication`, `AppKit`, or any AppKit-only API — iOS uses UIKit
- Use `CVDisplayLink` — iOS uses `CADisplayLink`
- Hold blocking `Mutex`/`RwLock` on the main thread (learned from WASM port)

**Always do these on the iOS target:**
- Use `#[cfg(target_os = "ios")]` (not `#[cfg(unix)]`) for iOS-specific branches
- Check `WindowContext::bounds()` at render time for layout decisions — window size is
  dynamic under Stage Manager
- Handle `UISceneActivationConditions` and scene lifecycle
  (foreground/background/disconnect)
- Use `Security.framework` for any credential storage
- Respond to memory pressure notifications
  (`UIApplicationDidReceiveMemoryWarningNotification`) — mobile has stricter memory
  limits than desktop; **iOS has no swap space — Jetsam kills via SIGKILL**
- Monitor `os_proc_available_memory()` (iOS 13+) to detect approaching termination
- Release Metal texture caches and non-visible buffers when entering background
- Use `CADisplayLink` with `preferredFrameRateRange` for ProMotion frame timing

---

## Build Commands

```bash
# Add iOS targets to rustup
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim

# Build the iOS static lib (device)
cargo build -p zed_ios --target aarch64-apple-ios --release \
    --no-default-features --features ios

# Build the iOS static lib (simulator, Apple Silicon Mac)
cargo build -p zed_ios --target aarch64-apple-ios-sim --release \
    --no-default-features --features ios

# Build and run on simulator via Xcode
xcodebuild \
    -project ios/ZedApp.xcodeproj \
    -scheme ZedApp \
    -destination 'platform=iOS Simulator,name=iPad Pro 13-inch (M4)' \
    build

# Run tests (those that compile for iOS — platform-agnostic crates only)
cargo test -p gpui --target aarch64-apple-ios-sim
```

**Note:** `aarch64-apple-ios` and `aarch64-apple-ios-sim` are **Tier 2** Rust targets —
guaranteed to build with pre-built std libraries available via `rustup target add`.
Cross-compilation requires macOS with Xcode installed. Set `IPHONEOS_DEPLOYMENT_TARGET=17.0`
environment variable for minimum version.

---

## Rust ↔ iOS FFI Architecture

The staticlib pattern is production-proven at scale:

- **Mozilla (Firefox):** Uses UniFFI for auto-generated Swift bindings from Rust, compiled
  to `.a` static libraries combined into XCFrameworks — powers bookmarks sync, history,
  telemetry for hundreds of millions of users
- **Signal:** Uses cbindgen for C header generation from Rust

Our architecture:
```
Pure Rust core (GPUI, editor, remote, workspace)
    ↓
FFI wrapper crate (crates/zed_ios/, crate-type = ["staticlib"])
    #[no_mangle] pub extern "C" fn zed_ios_main()
    #[no_mangle] pub extern "C" fn zed_ios_open_window()
    ...
    ↓
ZedApp-Bridging-Header.h (auto-generated C header)
    ↓
Swift host app (AppDelegate, SceneDelegate)
```

**objc2 crate ecosystem** for calling UIKit from Rust:
- `objc2-ui-kit` — UIView, UIViewController, UIWindow, UIApplication, etc.
- `objc2-foundation` — NSString, NSURL, NSFileManager, etc.
- `objc2-metal` — MTLDevice, MTLCommandQueue, etc.
- `block2` — Objective-C block support
- `dispatch2` — GCD bindings
- **Caveat:** Bindings are currently generated in a macOS-centric manner and may
  reference AppKit types on iOS (objc2 Issue #637). The maintainer (@madsmtm) is
  also an official Rust iOS target maintainer.

---

## Reference Material

- Zed remote development architecture: `crates/remote/README.md` (if it exists),
  otherwise read `crates/remote/src/lib.rs` and `crates/remote_server/src/main.rs`
- **SSH transport (current):** `crates/remote/src/transport/ssh.rs` — understand the
  `RemoteConnection` trait before implementing `IosSshTransport`
- GPUI platform abstraction: `crates/gpui/src/platform.rs` (the `Platform` and
  `PlatformWindow` traits)
- macOS platform reference implementation: `crates/gpui/src/platform/mac/`
- Linux platform reference implementation: `crates/gpui/src/platform/linux/`
- **WASM platform (newest):** `crates/gpui/src/platform/wasm/` — study for minimum
  viable `Platform` implementation and async architecture lessons
- **gpui-mobile (third-party):** github.com/itsbalamurali/gpui-mobile — iOS platform
  implementation using wgpu/Metal, useful reference but not official
- Rust iOS targets: https://doc.rust-lang.org/rustc/platform-support/apple-ios.html
- Apple UITextInput protocol: https://developer.apple.com/documentation/uikit/uitextinput
- Apple Security framework (Keychain):
  https://developer.apple.com/documentation/security/keychain_services
- Network.framework path monitor:
  https://developer.apple.com/documentation/network/nwpathmonitor
- UIScene lifecycle:
  https://developer.apple.com/documentation/uikit/app_and_environment/scenes
- **russh crate:** https://crates.io/crates/russh
- **russh_keys crate:** https://crates.io/crates/russh-keys
- **Agent architecture:** `crates/agent/src/thread.rs` (Thread engine, `run_turn()` loop),
  `crates/agent_ui/src/agent_panel.rs` (panel UI),
  `crates/agent_settings/` (settings schema)
- **Agent tool definitions:** `crates/assistant_tools/` — all tools use `Project` handle,
  no explicit remote checks
- **Remote server (no agent):** `crates/remote_server/src/headless_project.rs` — confirm
  zero agent initialization; understand what the headless server does and doesn't do
- **Settings sync reference:** GitHub Discussion #47058 — Coder team requesting remote
  host settings/API key resolution (our Phase 3.10 addresses this)
- **Known agent+remote bugs:** Issues #35603, #30106, #38392, #47362, #47910
- ASWebAuthenticationSession:
  https://developer.apple.com/documentation/authenticationservices/aswebauthenticationsession

---

## Questions to Answer Before Starting Each Phase

**Before Phase 0:**
- Which version of `zed --headless` on the Mac Studio side are we targeting? Pin to
  current main or a specific release tag?
- What is the minimum iOS deployment target? (Recommendation: iPadOS 17.0 for Stage
  Manager reliability; iPadOS 16.1 is the Stage Manager minimum but has rough edges.)
- **Do we need to bundle `zed --headless` binary for auto-deploy to remote hosts, or
  assume users install it manually?** (Decision: download from URL rather than embed in
  the iOS binary — embedding would bloat the app with server binaries for every arch.)

**Before Phase 1:**
- Do we implement the full UITextInput protocol or use the hidden UITextField
  trampoline? (Updated recommendation: **full protocol for v1** based on code editor
  requirements for cursor positioning, selection rects, and IME. See Phase 1 notes.)
- Is Apple Pencil input in scope for v1? (Recommendation: no.)
- **Which fonts to bundle?** (Recommendation: Fira Code, JetBrains Mono, SF Mono is
  system-provided. Keep bundle size under 5MB for fonts.)

**Before Phase 2:**
- Do we support password authentication in addition to key-based SSH?
  (Recommendation: keys only for v1; passwords require careful Keychain UX.)
- **`russh` vs `ssh2` for the embedded SSH library?** (Recommendation: `russh` — pure
  Rust, no C dependencies, async-native, simpler cross-compilation.)

**Before Phase 3A:**
- What languages get in-process syntax highlighting (Tree-sitter grammars) as a fallback
  when disconnected? All of them? Or a curated set?
- **What is the Jetsam memory budget we're targeting?** Profile on the lowest-RAM
  target iPad (likely 4GB models) to establish baseline.
- **Debug panel: does the `dap` crate compile for iOS?** The local transport code uses
  `std::process::Command` — may need cfg gates even though iPad only uses `RemoteDapStore`.
- **Edit prediction: how to handle the provider registry?** The `edit_prediction_registry`
  module lives in `crates/zed/src/zed/` (the desktop binary crate). Extract to a shared
  library crate, or duplicate the non-Copilot logic in `zed_ios`?
- **Edit prediction: what happens for Copilot users?** Silently fall back to Zed provider?
  Show a notification? Respect the setting and show no predictions?

**Before Phase 3B:**
- **Remote terminal: how to design the PTY multiplexing protocol?** Should terminal I/O
  messages be length-prefixed binary (efficient) or protobuf-wrapped (consistent with
  the rest of the protocol)?
- **Agent panel: implement settings sync (Approach A) first, or Zed Pro auth
  (Approach B)?** Approach A is more self-contained; Approach B serves more users.
  Both may be needed.
- **External agents (Claude Code, Gemini CLI) — do we show them on iPad?** They run
  as subprocesses on the host. (Recommendation: show them if the host has them
  configured; the iPad displays conversation events, the host runs the process. Verify
  Issues #47362 and #47910 are resolved first.)
- **Notifications: request permission at first connection or defer?** Apple rejects apps
  that request notification permission without clear user context.

**Before Phase 4:**
- **Per-project profiles: should profile selection be global or per-workspace?**
  Per-workspace is more useful but requires scoping `ActiveSettingsProfileName` to a
  workspace entity rather than a global.
