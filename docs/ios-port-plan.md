# Zed for iPad — Thin Client Architecture & Engineering Plan

## Project Overview

This is a fork of zed-industries/zed with the goal of porting Zed to iPadOS as a remote thin
client. The iPad runs minimal local compute. All language servers, git operations, file I/O,
terminal PTYs, Node runtimes, and extension hosting run on a remote host (e.g. a Mac Studio)
over SSH. The iPad is a rendering and input shell that connects to a `zed --headless`
server instance. The one exception is the **AI agent panel**: LLM API calls (HTTPS) execute
from the iPad, while agent tool invocations (file edits, terminal commands) proxy through
the remote protocol to execute on the host. Agent settings and API keys are synced from
the remote host at connection time — the user configures AI on the Mac Studio, not the iPad.

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
  zed-ios/               # NEW — iOS entry point crate (staticlib)
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
execution. Settings and API keys sync from the remote host at connection time (see
Phase 2.4).

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

### Architecture decision for iPad: Option A (v1) + Option B (v2)

**v1 — LLM calls from iPad, config synced from remote host:**

The iPad makes HTTPS calls to LLM providers directly — this works fine from iOS, it's
just reqwest over HTTPS with no platform dependencies. The key change is **settings
resolution**: instead of reading `language_models` config and API keys from the iPad's
local environment (which has neither), we sync them from the remote host at connection
time.

This requires:
1. A new `SyncLanguageModelSettings` protobuf message sent by the headless server at
   session start, containing the host's `language_models` settings block and
   environment-variable API keys (`ANTHROPIC_API_KEY`, `OPENAI_API_KEY`, etc.)
2. A settings overlay in `SettingsStore` that applies remote agent config on the iPad
3. A credential forwarding path: API keys from the host's OS keychain or environment
   → protobuf → iPad's in-memory credential store (never written to iPad disk/keychain)
4. For Zed Pro / hosted LLM access via Zed's proxy: `ASWebAuthenticationSession` on
   iOS for the OAuth/PKCE flow through Zed's web auth endpoint — this is the **only**
   place Authentication Services Framework is needed

What stays unchanged: `Thread`, `AcpThread`, `NativeAgent`, `AgentPanel`, all tool
definitions, the entire `crates/agent/` and `crates/agent_ui/` crate. The agent code
doesn't know or care that it's on an iPad — it gets settings from `SettingsStore` and
credentials from the credential store as it always does.

**v2 (future) — Full server-side agent execution:**

Move `Thread.run_turn()` and LLM streaming to the headless server. The iPad sends
user messages over a new agent protocol and receives streamed conversation events.
This is architecturally cleaner (true thin client) but requires:
- New protobuf message types for agent conversation lifecycle
- Agent initialization in `HeadlessProject` (currently has none)
- Tool confirmation UI flowing back to the iPad client
- Multi-month effort — defer to v2

### What NOT to use: Authentication Services Framework

The plan does **not** need `AuthenticationServices.framework` for general agent auth.
That framework handles Sign in with Apple, passkeys, and `ASWebAuthenticationSession`
for web OAuth. The actual frameworks needed:

| Need | Framework | Notes |
|---|---|---|
| SSH key storage on iPad | `Security.framework` (Keychain Services) | Already in plan |
| LLM API keys | None — synced from remote host, held in memory only | More secure than desktop |
| Zed Pro OAuth (if supported) | `AuthenticationServices` (`ASWebAuthenticationSession`) | Standard OAuth/PKCE |
| LLM provider OAuth (e.g. Google/Gemini) | `AuthenticationServices` (`ASWebAuthenticationSession`) | Narrow case, v2 |

---

## Work Items & Phase Plan

### Phase 0 — Build Infrastructure

**Goal:** `cargo build --target aarch64-apple-ios` succeeds; thin Swift host app compiles
and launches on device/simulator showing a black screen without crashing.

**Tasks:**

1. Create `crates/zed-ios/` with `crate-type = ["staticlib"]` and a single
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
   - `ZedApp-Bridging-Header.h` importing the C header generated from zed-ios
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

#### 2.1 — SSH Key Management (Keychain)

New file: `crates/zed-ios/src/keychain.rs`

- `store_ssh_key(label: &str, pem_bytes: &[u8]) -> Result<()>` — calls `SecItemAdd`
  with `kSecClassKey` (or `kSecClassGenericPassword` for simpler storage),
  `kSecAttrKeyType`, `kSecAttrApplicationLabel`, `kSecValueData`,
  and `kSecAttrAccessible = kSecAttrAccessibleWhenUnlockedThisDeviceOnly`
- `load_ssh_key(label: &str) -> Result<Vec<u8>>` — calls `SecItemCopyMatching`
- `delete_ssh_key(label: &str) -> Result<()>` — calls `SecItemDelete`
- `list_ssh_key_labels() -> Result<Vec<String>>` — for the host connection UI to show
  available keys
- Expose these via the C FFI header so they're callable from Swift if needed for the key
  import flow

**Integration with russh:**
```rust
let pem_bytes = keychain::load_ssh_key("my-server-key")?;
let pem_str = std::str::from_utf8(&pem_bytes)?;
let key_pair = russh_keys::decode_secret_key(pem_str, passphrase)?;
// Use key_pair for authentication
```

SSH key import UI flow (Swift side):
1. Tap "Import SSH Key" → present `UIDocumentPickerViewController` filtering for
   `.pem`, `.p8`, `.key` extensions
2. User selects file → read bytes → call `store_ssh_key` → confirm to user
3. Key never written to disk; raw bytes only exist in memory during import

#### 2.2 — Connection Manager UI

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

#### 2.3 — Network Resilience

New file: `crates/zed-ios/src/network_monitor.rs`

- Wraps `NWPathMonitor` (Network.framework) via Rust FFI or `objc2` crate bindings
- Emits `NetworkEvent::Available` / `NetworkEvent::Lost` into GPUI's event system
- `ConnectionManager` listens for `NetworkEvent::Available` and triggers reconnection
  if a session was previously active
- Reconnection strategy: immediate retry, then 1s, 2s, 4s, 8s, 16s, cap at 30s
  (exponential backoff)
- During reconnection, editor is read-only with a banner ("Reconnecting to host…");
  existing buffer content remains visible because CRDT state is local

#### 2.4 — Agent Settings Sync from Remote Host

The agent panel runs on the iPad but its configuration lives on the remote host. At
connection time, the iPad must receive the host's agent settings and API keys.

**New protobuf messages** (add to `crates/proto/proto/zed.proto`):

```protobuf
message SyncLanguageModelSettings {
    string language_models_json = 1;   // serialized "language_models" settings block
    repeated ApiKeyEntry api_keys = 2; // keys from host environment/keychain
}

message ApiKeyEntry {
    string provider = 1;   // "anthropic", "openai", "google", etc.
    string api_key = 2;    // key value (transmitted over encrypted SSH tunnel)
}
```

**Host side** (`crates/remote_server/`):
- On session initialization, read the host's `language_models` settings from
  `~/.config/zed/settings.json` and collect API keys from environment variables
  (`ANTHROPIC_API_KEY`, `OPENAI_API_KEY`, `GOOGLE_AI_API_KEY`, etc.) and/or the
  host's OS keychain
- Send `SyncLanguageModelSettings` to the client immediately after session handshake

**iPad side** (`crates/zed-ios/` + `crates/settings/`):
- `SettingsStore` receives the remote settings and applies them as a **remote overlay**
  layer — higher priority than iPad-local settings, lower than project `.zed/settings.json`
- API keys are stored in an **in-memory credential store** only — never written to the
  iPad's Keychain or filesystem. Keys exist in memory for the duration of the session
  and are cleared on disconnect.
- The `LanguageModelRegistry` picks up the settings transparently — no changes needed
  to `crates/agent/` or `crates/agent_ui/`

**Zed Pro / hosted LLM authentication** (if supported):
- Zed's own proxy service uses account-based auth, not API keys
- Implement the OAuth/PKCE flow using `ASWebAuthenticationSession` on iOS
- Store the Zed account token in the iPad's Keychain (this IS iPad-local, unlike
  LLM API keys which come from the remote host)

**External agents** (Claude Code, Gemini CLI, etc.):
- External agents spawn as subprocesses — **this is blocked on iOS** regardless
- External agents that users configure on the Mac Studio will run there; the iPad
  receives conversation events via the existing ACP protocol over the remote connection
- Note: External agent discovery on remote is a known pain point (Issue #47910, #47362);
  verify these are resolved before shipping

**Success criterion:** Connect to a `zed --headless` instance on a Mac Studio. Project panel
shows remote files. Tap a file → it opens in the editor. Open the agent panel → it shows
the models configured on the Mac Studio. Send a prompt → completions stream. Agent
edits a file → edit happens on the remote host. Kill WiFi for 5 seconds and reconnect
→ session automatically restores.

---

### Phase 3 — Editor & Terminal

**Goal:** Full editing session end-to-end. LSP completions (from remote), git decorations
(from remote), remote terminal with touch keyboard accessory bar.

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

#### 3.2 — Remote Terminal

The terminal panel in thin-client mode:

- VT100/VT220 parsing lives in `crates/terminal/` — wraps
  `alacritty_terminal::Term<ZedListener>` using the `vte` crate (Paul Williams' state
  machine, ~2.9K SLoC, `no_std` compatible, **zero platform dependencies**)
- **Key proof point:** `TerminalBuilder::new_display_only()` already creates a terminal
  **without any PTY** — used by the REPL subsystem. This proves the VT state machine
  operates independently of PTY spawning. The iPad only needs this display-only mode
  with input forwarded over the SSH channel.
- PTY management lives on the host — the iPad receives a byte stream over the remote
  connection and writes keystrokes back
- Terminal rendering via GPUI's text rendering — works as-is
- Terminal events bridge via `ZedListener` implementing alacritty's `EventListener` trait,
  using an unbounded channel with **4ms batching** for efficiency

**Touch keyboard accessory bar** (new component:
`crates/terminal/src/ios_accessory_bar.rs`):
- A `UIInputAccessoryView`-equivalent implemented as a GPUI element rendered above
  the software keyboard
- Buttons: `Esc`, `Tab`, `Ctrl`, `↑`, `↓`, `←`, `→`, `~`, `/`, `|`, `-`
- Each button emits the appropriate escape sequence / control character into the terminal
  input stream
- Visibility: shown when the terminal panel is focused and software keyboard is visible;
  hidden otherwise

#### 3.3 — Settings Path Adjustment

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

**Success criterion:** Open a file on the remote host, edit it, save it (save goes to host via
remote protocol), see git diff decorations in the gutter. Open the terminal panel, run `ls`,
see output. Disconnect WiFi mid-session, reconnect, continue editing.

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
| `extension_host` | Manages local WASM extensions |
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

**Exclude from iOS:** `extension_host` (which manages external ACP agent server
processes like Claude Code) — these spawn subprocesses. External agents configured on
the Mac Studio run there; the iPad receives their events via the remote protocol.

**Also exclude the system-ssh transport** (`crates/remote/src/transport/ssh.rs` in its
current form) — replace with the `russh`-based `IosSshTransport`.

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
cargo build -p zed-ios --target aarch64-apple-ios --release \
    --no-default-features --features ios

# Build the iOS static lib (simulator, Apple Silicon Mac)
cargo build -p zed-ios --target aarch64-apple-ios-sim --release \
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
FFI wrapper crate (crates/zed-ios/, crate-type = ["staticlib"])
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
  host settings/API key resolution (our Phase 2.4 addresses this)
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
  assume users install it manually?** (Impacts binary upload code in SSH transport.)

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
- What is the reconnection UX during a long network outage — show error and require
  manual reconnect, or keep retrying indefinitely?
- **`russh` vs `ssh2` for the embedded SSH library?** (Recommendation: `russh` — pure
  Rust, no C dependencies, async-native, simpler cross-compilation.)
- **Agent settings sync: which API keys does the host forward?** All environment
  variables matching known provider patterns? Only keys for providers configured in
  settings.json? (Recommendation: forward keys for all configured providers only.)
- **Do we support Zed Pro account auth on iPad?** If yes, need
  `ASWebAuthenticationSession` for OAuth/PKCE. (Recommendation: yes for v1 — many
  users rely on Zed's hosted LLM proxy and won't have their own API keys.)
- **External agents (Claude Code, Gemini CLI) — do we show them on iPad?** They run
  as subprocesses on the host. (Recommendation: show them if the host has them
  configured; the iPad displays conversation events, the host runs the process. Verify
  Issues #47362 and #47910 are resolved first.)

**Before Phase 3:**
- What languages get in-process syntax highlighting (Tree-sitter grammars) as a fallback
  when disconnected? All of them? Or a curated set?
- Does the terminal panel show at all when disconnected, or is it hidden?
- **What is the Jetsam memory budget we're targeting?** Profile on the lowest-RAM
  target iPad (likely 4GB models) to establish baseline.
