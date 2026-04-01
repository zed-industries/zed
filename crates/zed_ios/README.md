# crates/zed_ios/ — iOS Entry Point Crate

This crate produces a static library (`.a`) linked by the Swift host app in `ios/`.
It is the Rust-side entry point for everything iOS-specific that doesn't belong in GPUI.

## Current state

- `ios_main()` initializes GPUI with the iOS platform + embedded assets, then boots the
  real Zed workspace (settings, themes, fonts, client, workspace) via `init_zed()`.
- `Session::new` is async, so workspace window opening is deferred to a `cx.spawn`.
- `connection_landing.rs` — connection manager UI with saved hosts list, auto-reconnect,
  session persistence, workspace switcher. Hosts persisted as JSON in App Support.
- Terminal panel with SSH-backed terminals (PTY over SSH channel via russh).
- Agent panel with external agent support (Claude Code via ACP over SSH channels).
- Edit prediction with provider registry (copied from desktop, Copilot skipped).
- Keychain unlock auth flow for external agents on macOS remotes.
- Workspace state persisted to database (dock layout, open files, active profile).
- Settings profile selector for project-level profiles.

## Responsibilities

- C FFI entry points (`zed_ios_main`, `zed_ios_open_window`, `zed_ios_close_window`,
  `zed_ios_will_resign_active`, `zed_ios_build_menus`)
- Full Zed app initialization (settings, theme, fonts, client, workspace, terminal,
  agent panel, edit prediction, profile selector, panels)
- `edit_prediction_registry.rs` — provider assignment (adapted from desktop `zed` crate)
- HTTP client configured with `proxy_and_user_agent()` for platform TLS verifier

## Build

The primary build path is Xcode — open `ios/Zed.xcodeproj` and hit Run. The Xcode
build phase handles `cargo build` automatically.

For quick Rust-only error checking:
```bash
# Simulator:
cargo check -p zed_ios --target aarch64-apple-ios-sim --no-default-features
# Device:
cargo check -p zed_ios --target aarch64-apple-ios --no-default-features
```

Do not pass `--features ios` — there is no such feature flag; iOS-specific code
is gated by `cfg(target_os = "ios")` automatically when targeting `aarch64-apple-ios*`.

## Dev Remote Server

The iPad connects to a remote `zed --headless` server. For development with features
on this branch (e.g. settings profile sync), build and deploy the server binary:

```bash
# Build (release recommended for ongoing use, debug for iteration):
cargo build -p remote_server --release

# Deploy — copy to ~/.zed_server/ with a name matching the glob pattern:
cp target/release/remote_server ~/.zed_server/zed-remote-server-dev-ipad

# Kill running server so next connection picks up the new binary:
pkill -9 -f "zed-remote-server"
```

The iPad's russh transport resolves the server binary via `ls -t ~/.zed_server/zed-remote-server-*`
(newest by modification time). The dev binary must be newer than the stable release binary.

Features requiring an updated server:
- Settings profile sync (active profile sent via `UpdateUserSettings` proto message)

## This crate must NEVER

- Use `std::process::Command` or any subprocess spawning
- Access filesystem paths outside the app sandbox
- Depend on `node_runtime`, `lsp` (local), `task`, `dap`, `extension_host`, or `git` (CLI)
- Link AppKit or any macOS-only framework

## Keybindings

iPad has its own standalone keymap at `assets/keymaps/default-ios.json`. It is **not** an
overlay on the macOS keymap — it is a self-contained file that only references actions
registered by crates initialized in `init_zed()`.

When adding a new feature or action to the iPad build:
1. Register / init the crate in `init_zed()` in `lib.rs`.
2. Add keybindings for the new actions to `assets/keymaps/default-ios.json`.
3. Only reference actions whose crates are initialized — unregistered actions cause a
   load failure. Check `init_zed()` to see what's available.
4. Touch `crates/assets/src/assets.rs` to force RustEmbed to pick up keymap changes.

The vim keymap (`keymaps/vim.json`) is loaded separately with partial-failure tolerance
since it may reference actions from crates not yet ported to iPad.

The iOS keymap includes `AcpThread > Editor` contexts for agent panel input (enter to
send, cmd-enter follow mode) — these are separate from the `AgentPanel` context bindings.

## FFI pattern

All public functions use `#[no_mangle] pub extern "C" fn` with C-compatible types.
The Swift side imports them via `Zed-Bridging-Header.h`.
See `ios/port-plan.md` for full architecture details.
