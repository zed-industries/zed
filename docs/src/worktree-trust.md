---
title: Zed and trusted worktrees
description: "Configure which folders Zed trusts for running code and extensions."
---

# Zed and trusted worktrees

A worktree in Zed is either a directory or a single file that Zed opens as a standalone "project".
Zed opens a worktree each time you run `zed some/path`, drag a file or directory into Zed, or open your user settings file.

Every worktree opened may contain a `.zed/settings.json` file with extra configuration options that may require installing and spawning language servers or MCP servers.
To let users choose based on their own threat model and risk tolerance, all worktrees start in Restricted Mode. Restricted Mode prevents downloading and running related items from `.zed/settings.json`. Until a worktree is trusted, Zed does not run related untrusted actions and waits for user confirmation. This gives users a chance to review project settings, MCP servers, and language servers.

Zed still trusts tools it installs globally. Global MCP servers and global language servers such as Prettier and Copilot are installed and started as usual, independent of worktree trust.

If a worktree is not trusted, Zed will indicate this with an exclamation mark icon in the title bar. Clicking this icon or using `workspace::ToggleWorktreeSecurity` action will bring up the security modal that allows the user to trust the worktree.

Trusting a worktree persists that decision between restarts. You can clear all trusted worktrees with the `workspace::ClearTrustedWorktrees` command.
This command will restart Zed, to ensure no untrusted settings, language servers or MCP servers persist.

This feature works locally and on SSH and WSL remote hosts. Zed tracks trust information per host in these cases.

## What is restricted

Restricted Mode prevents:

- Project settings (`.zed/settings.json`) from being parsed and applied
- Language servers from being installed and spawned
- MCP servers from being installed and spawned

## Configuring broad worktree trust

By default, Zed does not trust new worktrees. Users must trust each new worktree individually. Though not recommended, users can trust all worktrees with this setting ([how to edit](./configuring-zed.md#settings-files)):

```json [settings]
"session": {
  "trust_all_worktrees": true
}
```

Auto-trusted worktrees are not persisted between restarts; only manually trusted worktrees are. This ensures users make new trust decisions if they later disable `trust_all_worktrees`.

## Trust hierarchy

These are mostly internal details and may change, but they help explain how multiple trust requests can be approved at once.
Zed has multiple layers of trust, based on the requests, from the least to most trusted level:

- "single file worktree"

After opening an empty Zed window, you can open a single file. You can also open a file outside the current directory after opening a directory.
A common example is `zed: open settings file`, which may start a language server for that file and create a new single-file worktree.

Spawning a language server presents a risk should the language server experience a supply-chain attack; therefore, Zed restricts that by default. Each single file worktree requires a separate trust grant, unless the directory containing it is trusted or all worktrees are trusted.

- "directory worktree"

If a directory is open in Zed, it is a full worktree. It may spawn multiple language servers and MCP servers defined in project settings. Each directory worktree therefore requires a separate trust grant unless a parent-directory trust grant exists (see below).

When a directory worktree is trusted, language and MCP servers are permitted to be downloaded and started, hence we also enable single file worktree trust for the host in question automatically when this occurs: this helps when opening single files when using language server features in the trusted directory worktree.

- "parent directory worktree"

To permit trust decisions for multiple directory worktrees at once, it's possible to trust all subdirectories of a given parent directory worktree opened in Zed by checking the appropriate checkbox. This will grant trust to all its subdirectories, including all current and potential directory worktrees.
