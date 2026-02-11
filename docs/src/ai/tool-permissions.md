# Tool Permissions

Configure which agent actions run automatically and which require your approval.

> **Note:** In Zed v0.224.0 and above, this page documents the granular `agent.tool_permissions` system.
>
> **Note:** Before Zed v0.224.0, tool approval was controlled by the `agent.always_allow_tool_actions` boolean (default `false`). Set it to `true` to auto-approve tool actions, or leave it `false` to require confirmation.

## Quick Start

You can use Zed's Settings UI to configure tool permissions, or add rules directly to your `settings.json`:

```json [settings]
{
  "agent": {
    "tool_permissions": {
      "default": "allow",
      "tools": {
        "terminal": {
          "default": "confirm",
          "always_allow": [
            { "pattern": "^cargo\\s+(build|test|check)" },
            { "pattern": "^npm\\s+(install|test|run)" }
          ],
          "always_confirm": [{ "pattern": "sudo\\s+/" }]
        }
      }
    }
  }
}
```

This example auto-approves `cargo` and `npm` commands in the terminal tool, while requiring manual confirmation on a case-by-case basis for `sudo` commands. Non-terminal commands follow the global `"default": "allow"` setting, but tool-specific defaults and `always_confirm` rules can still prompt.

## How It Works

The `tool_permissions` setting lets you customize tool permissions by specifying regex patterns that:

- **Auto-approve** actions you trust
- **Auto-deny** dangerous actions (blocked even when `tool_permissions.default` is set to `"allow"`)
- **Always confirm** sensitive actions regardless of other settings

## Supported Tools

| Tool                     | Input Matched Against        |
| ------------------------ | ---------------------------- |
| `terminal`               | The shell command string     |
| `edit_file`              | The file path                |
| `delete_path`            | The path being deleted       |
| `move_path`              | Source and destination paths |
| `copy_path`              | Source and destination paths |
| `create_directory`       | The directory path           |
| `restore_file_from_disk` | The file paths               |
| `save_file`              | The file paths               |
| `fetch`                  | The URL                      |
| `web_search`             | The search query             |

For MCP tools, use the format `mcp:<server>:<tool_name>`. For example, a tool called `create_issue` on a server called `github` would be `mcp:github:create_issue`.

## Configuration

```json [settings]
{
  "agent": {
    "tool_permissions": {
      "default": "confirm",
      "tools": {
        "<tool_name>": {
          "default": "confirm",
          "always_allow": [{ "pattern": "...", "case_sensitive": false }],
          "always_deny": [{ "pattern": "...", "case_sensitive": false }],
          "always_confirm": [{ "pattern": "...", "case_sensitive": false }]
        }
      }
    }
  }
}
```

### Options

| Option           | Description                                                                    |
| ---------------- | ------------------------------------------------------------------------------ |
| `default`        | Fallback when no patterns match: `"confirm"` (default), `"allow"`, or `"deny"` |
| `always_allow`   | Patterns that auto-approve (unless deny or confirm also matches)               |
| `always_deny`    | Patterns that block immediately—highest priority, cannot be overridden         |
| `always_confirm` | Patterns that always prompt, even when `tool_permissions.default` is `"allow"` |

### Pattern Syntax

```json [settings]
{
  "pattern": "your-regex-here",
  "case_sensitive": false
}
```

Patterns use Rust regex syntax. Matching is case-insensitive by default.

## Rule Precedence

From highest to lowest priority:

1. **Built-in security rules** — Hardcoded protections (e.g., `rm -rf /`). Cannot be overridden.
2. **`always_deny`** — Blocks matching actions
3. **`always_confirm`** — Requires confirmation for matching actions
4. **`always_allow`** — Auto-approves matching actions
5. **Tool-specific `default`** — Per-tool fallback when no patterns match (e.g., `tools.terminal.default`)
6. **Global `default`** — Falls back to `tool_permissions.default` when no tool-specific default is set

## Examples

### Terminal: Auto-Approve Build Commands

```json [settings]
{
  "agent": {
    "tool_permissions": {
      "tools": {
        "terminal": {
          "default": "confirm",
          "always_allow": [
            { "pattern": "^cargo\\s+(build|test|check|clippy|fmt)" },
            { "pattern": "^npm\\s+(install|test|run|build)" },
            { "pattern": "^git\\s+(status|log|diff|branch)" },
            { "pattern": "^ls\\b" },
            { "pattern": "^cat\\s" }
          ],
          "always_deny": [
            { "pattern": "rm\\s+-rf\\s+(/|~)" },
            { "pattern": "sudo\\s+rm" }
          ],
          "always_confirm": [
            { "pattern": "sudo\\s" },
            { "pattern": "git\\s+push" }
          ]
        }
      }
    }
  }
}
```

### File Editing: Protect Sensitive Files

```json [settings]
{
  "agent": {
    "tool_permissions": {
      "tools": {
        "edit_file": {
          "default": "confirm",
          "always_allow": [
            { "pattern": "\\.(md|txt|json)$" },
            { "pattern": "^src/" }
          ],
          "always_deny": [
            { "pattern": "\\.env" },
            { "pattern": "secrets?/" },
            { "pattern": "\\.(pem|key)$" }
          ]
        }
      }
    }
  }
}
```

### Path Deletion: Block Critical Directories

```json [settings]
{
  "agent": {
    "tool_permissions": {
      "tools": {
        "delete_path": {
          "default": "confirm",
          "always_deny": [
            { "pattern": "^/etc" },
            { "pattern": "^/usr" },
            { "pattern": "\\.git/?$" },
            { "pattern": "node_modules/?$" }
          ]
        }
      }
    }
  }
}
```

### URL Fetching: Control External Access

```json [settings]
{
  "agent": {
    "tool_permissions": {
      "tools": {
        "fetch": {
          "default": "confirm",
          "always_allow": [
            { "pattern": "docs\\.rs" },
            { "pattern": "github\\.com" }
          ],
          "always_deny": [{ "pattern": "internal\\.company\\.com" }]
        }
      }
    }
  }
}
```

### MCP Tools

```json [settings]
{
  "agent": {
    "tool_permissions": {
      "tools": {
        "mcp:github:create_issue": {
          "default": "confirm"
        },
        "mcp:github:create_pull_request": {
          "default": "confirm"
        }
      }
    }
  }
}
```

## Global Auto-Approve

To auto-approve all tool actions:

```json [settings]
{
  "agent": {
    "tool_permissions": {
      "default": "allow"
    }
  }
}
```

This bypasses confirmation prompts for most actions, but `always_deny`, `always_confirm`, built-in security rules, and paths inside Zed settings directories still prompt or block.

## Shell Compatibility

For the `terminal` tool, Zed parses chained commands (e.g., `echo hello && rm file`) to check each sub-command against your patterns.

All supported shells work with tool permission patterns, including sh, bash, zsh, dash, fish, PowerShell 7+, pwsh, cmd, xonsh, csh, tcsh, Nushell, Elvish, and rc (Plan 9).

## Writing Patterns

- Use `\b` for word boundaries: `\brm\b` matches "rm" but not "storm"
- Use `^` and `$` to anchor patterns to start/end of input
- Escape special characters: `\.` for literal dot, `\\` for backslash
- Test carefully—a typo in a deny pattern blocks legitimate actions

## Built-in Security Rules

Zed includes a small set of hardcoded security rules that **cannot be overridden** by any setting. These only apply to the **terminal** tool and block recursive deletion of critical directories:

- `rm -rf /` and `rm -rf /*` — filesystem root
- `rm -rf ~` and `rm -rf ~/*` — home directory
- `rm -rf $HOME` / `rm -rf ${HOME}` (and `$HOME/*`) — home directory via environment variable
- `rm -rf .` and `rm -rf ./*` — current directory
- `rm -rf ..` and `rm -rf ../*` — parent directory

These patterns catch any flag combination (e.g., `-fr`, `-rfv`, `-r -f`, `--recursive --force`) and are case-insensitive. They are checked against both the raw command and each parsed sub-command in chained commands (e.g., `ls && rm -rf /`).

There are no other built-in rules. The default settings file ({#action zed::OpenDefaultSettings}) includes commented-out examples for protecting `.env` files, secrets directories, and private keys — you can uncomment or adapt these to suit your needs.

## UI Options

When the agent requests permission, the dialog includes:

- **Allow once** / **Deny once** — One-time decision
- **Always for <tool>** — Sets a tool-level default to allow or deny
- **Always for <pattern>** — Adds an `always_allow` or `always_deny` pattern (when a safe pattern can be extracted)

Selecting "Always for <tool>" sets `tools.<tool>.default` to allow or deny. When a pattern can be safely extracted, selecting "Always for <pattern>" adds an `always_allow` or `always_deny` rule for that input. MCP tools only support the tool-level option.
