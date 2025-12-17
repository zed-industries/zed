# Environment Variables

This page lists environment variables that affect Zed's behavior.

## Where Zed Gets Environment Variables

| Launch Method | Environment Source |
|---------------|-------------------|
| CLI (`zed .`) | Inherits from the shell session |
| Dock / Launcher / Window Manager | Spawns a login shell in home directory |
| Per-project | Spawns a login shell in the project directory |

When launched via the CLI, Zed inherits environment variables from your shell. When launched from the Dock or a launcher, Zed spawns a login shell to obtain a base environment.

For project-specific environments (e.g., when using `direnv`, `asdf`, or `mise`), Zed spawns an additional login shell in the project directory and stores those variables for use in tasks, terminals, and language servers.

## Environment Variable Reference

| Variable | Purpose |
|----------|---------|
| `XDG_CONFIG_HOME` | On Linux, overrides the default config location (`~/.config/zed`) |
| `ZED_LOG_LEVEL` | Sets the log verbosity (`debug`, `info`, `warn`, `error`) |
| `ZED_BUNDLE` | When set, indicates Zed is running as a bundled app |
| `ZED_STATELESS` | When set, Zed won't persist window state between sessions |
| `ZED_ALWAYS_ACTIVE` | When set, Zed behaves as if it's always the active application |
| `http_proxy` / `HTTP_PROXY` | HTTP proxy URL for network requests |
| `https_proxy` / `HTTPS_PROXY` | HTTPS proxy URL for network requests |
| `no_proxy` / `NO_PROXY` | Comma-separated list of hosts to bypass proxy |

## Variables Set by Zed

Zed sets these variables in spawned processes (tasks, terminals, language servers):

| Variable | Value |
|----------|-------|
| `ZED_TERM` | Set in the integrated terminal |
| `TERM` | Set to a terminal type in the integrated terminal |
| `EDITOR` | Set to `zed --wait` when appropriate |
| `VISUAL` | Set to `zed --wait` when appropriate |

## Usage in Tasks and Terminals

Environment variables from your project are available in:

- **Tasks**: All task commands have access to the project environment
- **Terminal**: The integrated terminal inherits project-specific variables
- **Language servers**: Servers spawned for a project use that project's environment

```json
{
  "tasks": [
    {
      "label": "echo env",
      "command": "echo $MY_PROJECT_VAR"
    }
  ]
}
```

> **Note:** Environment handling changed in Zed 0.152.0. The CLI now always passes its environment to Zed, even when an instance is already running.
