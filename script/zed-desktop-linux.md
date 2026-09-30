# Local Linux Zed-Desktop build

This is a side-by-side local fork installation, not an official Zed release.
Run these commands from the repository root after installing the normal Linux
build prerequisites yourself (see `docs/src/development/linux.md`). Python 3.9+
is also required. The scripts do not install dependencies or use sudo.

```sh
python3 script/zed-desktop-linux build
# Only when you explicitly want to install the completed build:
python3 script/zed-desktop-linux install --prefix "$HOME/.local"
```

The build uses the host Rust target and a dedicated `target/zed-desktop-linux`
directory. Cargo may fetch dependencies. When available, it selects `clang` and
`clang++` for WebRTC compatibility, while preserving explicit `CC`/`CXX`
overrides. It builds the `zed` and `cli` packages, not remote servers, and does
not upload symbols, publish artifacts, or install anything. This is a native
local build using host libraries, not a portable Linux distribution archive. The
install command accepts `--bundle PATH` for an already-built directory containing
the executable `zed-desktop` and `cli` files.

Installation writes these fork-named paths. The first four are under the explicit prefix:

- `bin/zed-desktop`: launcher, passing all arguments to the fork CLI with an
  explicit `--zed PREFIX/lib/zed-desktop/zed-desktop` editor path.
- `lib/zed-desktop/{zed-desktop,cli-bin,cli}`: private editor, CLI binary and
  CLI wrapper. The private wrapper also passes `--zed` for editor features that
  locate the CLI via `util::get_zed_cli_path()`.
- `share/applications/io.github.piumal.ZedDesktop.desktop`: application entry.
- `share/icons/hicolor/512x512/apps/io.github.piumal.ZedDesktop.png`: existing
  editor artwork under the fork's own icon name (not new branding artwork).

By default, the CLI `install` command also writes the following XDG autostart
entry outside the prefix:

- `${XDG_CONFIG_HOME:-$HOME/.config}/autostart/io.github.piumal.ZedDesktop.mobile-gateway.desktop`

Put `PREFIX/bin` on your PATH. Use a standard desktop-visible prefix such as
`~/.local` for application-menu discovery. No MIME defaults, URL handlers,
`zed` executables, upstream desktop entries, or upstream icons are changed.
Symlink destinations/ancestors are rejected to avoid overwriting another install.
To uninstall, remove only the four fork-owned locations under the prefix and,
if it was installed, the Mobile Gateway autostart entry shown above. User data
is deliberately retained.

## Mobile Gateway autostart

A normal CLI install enables this entry by default. It runs the fork's private
editor binary with the hidden `--mobile-gateway-host` argument at desktop login,
without opening a workspace window. To opt out during installation, use:

```sh
python3 script/zed-desktop-linux install --prefix "$HOME/.local" --no-autostart
```

`--no-autostart` does not change the Mobile Gateway setting and does not remove
an entry from an earlier install. Remove an existing startup trigger explicitly:

```sh
rm -f "${XDG_CONFIG_HOME:-$HOME/.config}/autostart/io.github.piumal.ZedDesktop.mobile-gateway.desktop"
```

Autostart does not enable Mobile Gateway. If Mobile Gateway is disabled (the
default), the host logs a refusal and exits immediately. If it is enabled, the
host remains resident with no workspace window. Launch the normal editor UI
later with `zed-desktop` or the `Zed-Desktop` application-menu entry; a normal
open request can create a window in the resident process. Closing a UI window
does not stop an enabled resident host—use the application's explicit **Quit**
command to stop both the process and Mobile Gateway host.

## Identity and isolation

`paths::APP_NAME` is `Zed-Desktop`; its derived lower-case name is `zed-desktop`.
On Linux the defaults are:

| Purpose | Default |
| --- | --- |
| Configuration | `~/.config/zed-desktop` |
| Data, extensions, databases and CLI socket | `~/.local/share/zed-desktop` |
| Cache and temporary application files | `~/.cache/zed-desktop` |
| State | `~/.local/state/zed-desktop` |

The corresponding XDG overrides (and existing Flatpak XDG overrides) still
apply. Do not deliberately point `--user-data-dir` at official Zed's directory;
explicit user overrides bypass default isolation. Project `.zed` directories,
internal Rust package names, release channel names and remote server protocols
are unchanged. Changing `APP_NAME` also changes default user paths on other
platforms; their packaging is not supported by this local Linux workflow.

All Linux channels use display name `Zed-Desktop` and Wayland/X11 app ID
`io.github.piumal.ZedDesktop`. The CLI socket retains its channel-specific file
name but lives under the separate fork data directory. Fork channels share the
same app identity and user directories; this is fork-versus-upstream isolation,
not per-channel isolation.

The desktop entry accepts files (`%F`) and advertises no URL scheme or MIME
association. Linux GPUI currently returns an unimplemented error for in-app
URL-scheme registration. Official `zed://` links therefore remain with the
existing handler. Internal `zed://` parsing is unchanged. Features requiring an
external official URL callback are not redirected to this fork.

## Updates

`ReleaseChannel::poll_for_updates()` returns false on Linux for every channel.
Both `auto_update::init` (including its settings observer) and manual
`auto_update::check` consult that gate, so neither can download an official
replacement. The build also embeds `ZED_UPDATE_EXPLANATION`, and the launcher
sets it, to explain manual updates. Rebuild and explicitly reinstall to update.
This is not a general offline mode: services, extensions, language servers and
remote-server downloads retain their existing behavior.

No `main.rs` updater insertion is required: keep `auto_update::init` so the
existing UI globals/release-note actions remain available. This conclusion
assumes future update entry points continue using the release-channel gate.

## Parent-agent integration

In `crates/zed/Cargo.toml`, change only the primary binary target:

```toml
[[bin]]
name = "zed-desktop"
path = "src/main.rs"
```

Keep `[package] name = "zed"` and all other binary targets/internal crates
unchanged. The existing `main.rs` compile-time assertion already compares
`paths::APP_NAME_LOWERCASE` to `CARGO_BIN_NAME`, and needs no edit. Until the
binary target is renamed, the fork build cannot succeed.

Upstream `script/bundle-linux`, `script/install-linux`, `script/install.sh`,
Flatpak/Snap and other platform packaging have deliberately not been rewritten.
They assume official binary names and/or register official URL handlers. Do not
use them for this fork. Any future distribution packaging needs its own identity
review. The dedicated launcher uses CLI `--zed` explicitly, avoiding upstream
CLI executable auto-detection paths without changing the CLI crate.

## Tests

```sh
python3 script/test-zed-desktop-linux
cargo test -p paths
cargo test -p release_channel
```

Installer tests use fake executables and explicit XDG configuration directories
inside a temporary test tree. They verify argument forwarding, reinstall
behavior, official-file preservation, symlink rejection, desktop-entry quoting,
and Mobile Gateway autostart opt-in/default/opt-out behavior. When available,
`desktop-file-validate` validates both desktop entries. They never install into
your home or system directories. Rust tests cover user-directory isolation,
unchanged project settings, Linux app identity and update policy for every
channel.
