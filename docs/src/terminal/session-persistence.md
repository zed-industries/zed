# Terminal Session Persistence with ZED_TAB_ID

Zed injects a `ZED_TAB_ID` environment variable into every terminal session — a stable UUID that persists across workspace restores and SSH reconnects. You can use it to automatically attach to a named tmux or zellij session, so closing and reopening Zed (or reconnecting after an SSH drop) resumes exactly where you left off.

## tmux

### bash / zsh

Add to `~/.bashrc` or `~/.zshrc`:

```sh
if [ -n "$ZED_TAB_ID" ]; then
  if [ -z "$TMUX" ]; then
    exec tmux new-session -A -s "$ZED_TAB_ID"
  fi
fi
```

### fish

Add to `~/.config/fish/config.fish`:

```fish
if set -q ZED_TAB_ID; and not set -q TMUX
    exec tmux new-session -A -s "$ZED_TAB_ID"
end
```

## zellij

Zellij works best inside Zed with a minimal config that starts your shell directly, locks the UI (so keybindings don't conflict with Zed), and hides the default bars.

Create `~/.config/zellij/config.kdl` (or add to your existing config):

```kdl
// Start in locked mode so zellij keybindings don't conflict with Zed
default_mode "locked"

// Use fish (or your preferred shell)
default_shell "fish"

// Minimal UI — Zed already provides tabs, scrollback, etc.
default_layout "compact"
pane_frames false
simplified_ui true
```

### bash / zsh

Add to `~/.bashrc` or `~/.zshrc`:

```sh
if [ -n "$ZED_TAB_ID" ]; then
  if [ -z "$ZELLIJ" ]; then
    exec zellij attach --create "$ZED_TAB_ID"
  fi
fi
```

### fish

Add to `~/.config/fish/config.fish`:

```fish
if set -q ZED_TAB_ID; and not set -q ZELLIJ
    exec zellij attach --create "$ZED_TAB_ID"
end
```

## How it works

- `ZED_TAB_ID` is set to a UUID like `a1b2c3d4-e5f6-...` before the shell starts.
- `tmux new-session -A -s` / `zellij attach --create` attaches to an existing session with that name, or creates one if it doesn't exist.
- The `exec` replaces the shell process so that exiting tmux/zellij closes the terminal tab cleanly.
- The `TMUX` / `ZELLIJ` guard prevents nesting when you're already inside a session.
- The UUID is persisted to Zed's database, so reopening the workspace reuses the same ID and reattaches to the same session.
