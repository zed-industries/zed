---
title: Terminal Threads - Zed
description: Run agent CLIs and TUIs directly in terminal-backed threads in Zed.
---

# Terminal Threads

Terminal Threads are terminal-backed threads in the [Threads Sidebar](./parallel-agents.md#threads-sidebar). Use them when you want to run an agent CLI or TUI directly in Zed.

Terminal Threads are different from [External Agents](./external-agents.md). External Agents integrate with Zed through ACP and render as agent threads. Terminal Threads run the native command-line tool in a terminal that Zed organizes as a thread.

## What Zed Owns {#what-zed-owns}

Zed owns the thread surface:

- the terminal-backed thread in the Threads Sidebar
- thread grouping by project
- switching and organizing the terminal session alongside other threads

## What the CLI Owns {#what-the-cli-owns}

The CLI or TUI running inside the terminal owns its own:

- authentication
- model/provider configuration
- subscriptions or API keys
- tool configuration
- skills and instruction files
- MCP configuration

Zed Agent profiles, Zed Agent tool permissions, Zed Skills, and Zed Agent MCP settings do not automatically apply to Terminal Threads.

## Credentials and Remote Projects {#credentials-and-remote-projects}

Credentials come from the terminal session and the CLI/TUI running inside it.

In remote projects, the CLI may read the remote shell environment and remote config files. In local Terminal Threads, it reads the local shell environment and local config files. Zed does not copy API keys from LLM provider settings into Terminal Threads.

## When to Use Terminal Threads {#when-to-use-terminal-threads}

Use Terminal Threads when:

- you want the tool's native CLI/TUI experience
- no ACP integration exists
- you want subscription behavior owned by the CLI
- you want the CLI to use its own native config files

For ACP-integrated agents, see [External Agents](./external-agents.md).
