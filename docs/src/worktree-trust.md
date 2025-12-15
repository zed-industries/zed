# Zed and trusted worktrees

A worktree in Zed is either a directory or a single file that Zed opens as a standalone "project".
Zed opens a worktree every time `zed some/path` is invoked, on drag and dropping a file or directory into Zed, on opening user settings.json, etc.

Every worktree opened may contain a `.zed/settings.json` file with extra configuration options that may require installing and spawning language servers or MCP servers.
Note that the Zed workspace itself may also perform user-configured MCP server installation and spawning, even if no worktrees are open.

In order to provide users the opportunity to make their own choices according to their unique threat model and risk tolerance, the workspace and all worktrees will be started in Restricted mode, which prevents download and execution of any related items. Until configured to trust the workspace and/or worktrees, Zed will not perform any untrusted actions and will wait for user confirmation. This gives users a chance to review and understand any pre-configured settings, MCP servers, or language servers associated with a project.

If a worktree is not trusted, Zed will indicate this with an exclamation mark icon in the title bar and a message in the Agent panel. Clicking this icon or using `workspace::ToggleWorktreeSecurity` action will bring up the security modal that allows the user to trust the worktree.

Trusting any worktree will persist this information between restarts. It's possible to clear all trusted worktrees with `workspace::ClearTrustedWorktrees` command.
This command will restart Zed, to ensure no untrusted settings, language servers or MCP servers persist.

This feature works locally and on SSH and WSL remote hosts. Zed tracks trust information per host in these cases.

## What is restricted

Restricted Mode prevents:

- Project settings (`.zed/settings.json`) from being parsed and applied
- Language servers from being installed and spawned
- MCP servers from being installed and spawned

## Configuring broad worktree trust

By default, Zed won't trust any new worktrees and users will be required to trust each new worktree. Though not recommended, users may elect to trust all worktrees and the current workspace for a given session by configuring the following setting:

```json [settings]
"session": {
  "trust_all_worktrees": true
}
```

Note that auto trusted worktrees are not persisted between restarts, only manually trusted worktrees are. This ensures that new trust decisions must be made if a users elects to disable the `trust_all_worktrees` setting.

## Trust hierarchy

These are mostly internal details and may change in the future, but are helpful to understand how multiple different trust requests can be approved at once.
Zed has multiple layers of trust, based on the requests, from the least to most trusted level:

- "single file worktree"

After opening an empty Zed it's possible to open just a file, same as after opening a directory in Zed it's possible to open a file outside of this directory.
A typical scenario where a directory might be open and a single file is subsequently opened is opening Zed's settings.json file via `zed: open settings file` command: that starts a language server for a new file open, which originates from a newly created, single file worktree.

Spawning a language server presents a risk should the language server experience a supply-chain attack; therefore, Zed restricts that by default. Each single file worktree requires a separate trust grant, unless the directory containing it is trusted or all worktrees are trusted.

- "workspace"

Even an empty Zed workspace with no files or directories open presents a risk if new MCP servers are locally configured by the user without review. For instance, opening an Assistant Panel and creating a new external agent thread might require installing and running new user-configured [Model Context Protocol servers](./ai/mcp.md). By default, zed will restrict a new MCP server until the user elects to trust the local workspace. Users may also disable the entire Agent panel if preferred; see [AI Configuration](./ai/configuration.md) for more details.

Workspace trust, permitted by trusting Zed with no worktrees open, allows locally configured resources to be downloaded and executed. Workspace trust is per host and also trusts all single file worktrees from the same host in order to permit all local user-configured MCP and language servers to start.

- "directory worktree"

If a directory is open in Zed, it's a full worktree which may spawn multiple language servers associated with it or spawn MCP servers if contained in a project settings file.Therefore, each directory worktree requires a separate trust grant unless a parent directory worktree trust is granted (see below).

When a directory worktree is trusted, language and MCP servers are permitted to be downloaded and started, hence we also enable workspace trust for the host in question automatically when this occurs.

- "parent directory worktree"

To permit trust decisions for multiple directory worktrees at once, it's possible to trust all subdirectories of a given parent directory worktree opened in Zed by checking the appropriate checkbox. This will grant trust to all its subdirectories, including all current and potential directory worktrees.

This also automatically enables workspace trust to permit the newly trusted resources to download and start.
