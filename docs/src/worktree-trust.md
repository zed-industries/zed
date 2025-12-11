# Zed and trusted worktrees

A worktree in Zed is either a directory or a single file that Zed opens as a standalone "project".
Zed opens a worktree every time `zed some/path` is invoked, on drag and dropping a file or directory into Zed, on opening user settings.json, etc.

Every worktree may contain a `.zed/settings.json` file with extra configuration options, may require installing and spawning language server(s).
Zed itself may perform MCP server installation and spawning, even if no worktrees are open.
All those actions (and potentially, more will be added later) may be trusted or untrusted, and blindly trusting them may lead to security issues.

Until configured to automatically trust worktrees, Zed will not perform any untrusted actions and will wait for user conformation.
If any worktree trust is pending, Zed will indicate this with an exclamation mark icon in the title bar.
Clicking this icon or using `workspace::ToggleWorktreeSecurity` action will bring up the security modal that allows to trust the worktree.

Trusting any worktree will persist this information between restarts, it's possible to clear all trusted worktrees with `workspace::ClearTrustedWorktrees` command.
This command will restart Zed, to ensure no unnecessary settings are mixed and propagated into the application.

The mechanism works locally and on ssh and wsl remote hosts.
Zed tracks trust information per host.

## What is restricted

Restricted Mode prevents:
 - Project settings (`.zed/settings.json`) from being parsed and applied
 - Language servers from being installed and spawned
 - MCP Server integrations from being installed and spawned

## Configuring worktree trust

By default, Zed won't trust any new worktrees, this is possible to alter with

```json [settings]
"session": {
    "trust_all_worktrees": true
}
```

settings.

The auto trusted worktrees are not persisted between restarts, only the manually trusted worktrees are.

## Trust hierarchy

These are mostly internal details and may change in the future, but are helpful to understand how multiple different trust requests can be approved at once.
Zed has multiple layers of trust, based on the requests, from the least to the most trust level:

* "single file worktree"

After opening an empty Zed it's possible to open just a file, same as after opening a directory in Zed it's possible to open a file outside of this directory.
Usual scenario for both cases is opening Zed's settings.json file via `zed: open settings file` command: that starts a language server for a new file open, which originates from a newly created, single file worktree.

Spawning a language server is potentially dangerous, and Zed needs to restrict that by default.
Each single file worktree requires a separate trust permission, unless a more global level is trusted.

* "global"

Even an empty Zed instance with no files or directories open is potentially dangerous: opening an Assistant Panel and creating new external agent thread might require installing and running [Model Context Protocol servers](./ai/mcp.md).

Disabling the entire panel is possible, see [AI Configuration](./ai/configuration.md) for more details.
Yet when it's enabled, it's still reasonably safe to use remote AI agents and control their permissions in the Assistant Panel.

Unlike that, MCP servers are similar to language servers and may require fetching, installing and running packages or binaries.
Given that those servers are not tied to any particular worktree, this level of trust is required to operate any MCP server.

Global level of trust assumes all single file worktrees are trusted too, for the same host: if we allow global MCP server-related functionality, we can already allow spawning language servers for single file worktrees as well.

* "directory worktree"

If a directory is open in Zed, it's a full worktree which may spawn multiple language servers associated with it.
Each such worktree requires a separate trust permission, so each separate directory worktree has to be trusted separately, unless a more global level is trusted.

When a directory worktree is trusted and language servers are allowed to be downloaded and started, hence we also allow "global" level of trust (hence, "single file worktree" level of trust also).

* "path override"

To ease trusting multiple directory worktrees at once, it's possible to trust a parent directory of a certain directory worktree opened in Zed.
Trusting a directory means trusting all its subdirectories as well, including all current and potential directory worktrees.

If we trust multiple projects to install and spawn various language server processes, we can also allow global trust requests for MCP servers installation and spawning.
