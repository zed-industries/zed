---
title: AI Agents in Zed
description: Compare Zed Agent, external agents, and terminal threads.
---

# Agents

Zed supports three agent paths. Choose the path based on how you want agentic work to run.

| Agent path                                | Runs in                         | Uses                                                                  | Best when                                                                              |
| ----------------------------------------- | ------------------------------- | --------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| [Zed Agent](./zed-agent.md)               | Agent Panel and Threads Sidebar | Zed-configured LLM providers, native tools, skills, instructions, MCP | You want Zed's native agent integration                                                |
| [External Agents](./external-agents.md)   | Agent Panel and Threads Sidebar | ACP agent process and its own auth/config                             | You want Claude, Codex, OpenCode, Copilot, Cursor, Pi, or another ACP-integrated agent |
| [Terminal Threads](./terminal-threads.md) | Threads Sidebar and terminal    | Native CLI/TUI auth/config                                            | You want the tool's command-line experience organized in Zed                           |

An agent path is sometimes called a harness: it is the way agentic work is started, displayed, configured, and controlled. It is separate from the LLM provider that supplies a model.

## Agent Path vs. LLM Provider {#agent-path-vs-llm-provider}

| Concept      | What it answers              | Examples                                                             |
| ------------ | ---------------------------- | -------------------------------------------------------------------- |
| Agent path   | How agentic work runs in Zed | Zed Agent, external agents, terminal threads                         |
| LLM provider | Where models come from       | Zed-hosted models, API access, subscriptions, gateways, local models |

Zed Agent uses [LLM Providers](./llm-providers.md) configured in Zed. External agents and terminal threads may use their own provider configuration.

## Thread Types {#thread-types}

Threads are the units shown in the [Threads Sidebar](./parallel-agents.md#threads-sidebar). Thread types include:

- Zed Agent threads
- External agent threads
- Terminal threads

Use [Parallel Agents](./parallel-agents.md) to run and manage multiple threads at once.
