---
title: Set Up AI in Zed
description: Choose setup paths for Zed AI, including model access, agent paths, tools, instructions, and disabling AI.
---

# Set Up AI

Use this page when you need to configure AI in Zed. If you know what you want to do, start with [AI Quick Start](./quick-start.md).

## Choose Model Access {#model-access}

| I want to                             | Go to                                                             | Notes                                                               |
| ------------------------------------- | ----------------------------------------------------------------- | ------------------------------------------------------------------- |
| Use models billed through Zed         | [Zed-Hosted Models](../account/zed-hosted-models.md)              | Requires a Zed plan with hosted model access                        |
| Use provider API access               | [Use API Access](./use-api-access.md)                             | For API keys, credits, top-ups, and usage billing                   |
| Use a subscription I already pay for  | [Use an Existing Subscription](./use-an-existing-subscription.md) | Some subscriptions work in Zed; others route through agents or CLIs |
| Use a gateway or cloud model platform | [Use a Gateway](./use-a-gateway.md)                               | OpenRouter, Vercel AI Gateway, Bedrock, and similar platforms       |
| Use local or self-hosted models       | [Use a Local Model](./use-a-local-model.md)                       | Ollama, LM Studio, local OpenAI-compatible servers                  |

## Choose an Agent Path {#agent-path}

| I want to                   | Go to                                     | Notes                                                                         |
| --------------------------- | ----------------------------------------- | ----------------------------------------------------------------------------- |
| Use Zed's native agent      | [Zed Agent](./zed-agent.md)               | Uses Zed-configured providers, profiles, tools, skills, instructions, and MCP |
| Use an ACP-integrated agent | [External Agents](./external-agents.md)   | The external agent usually owns auth and model configuration                  |
| Run a CLI or TUI directly   | [Terminal Threads](./terminal-threads.md) | The CLI owns auth, config, tools, instructions, and MCP                       |

## Configuration Surfaces {#configuration-surfaces}

| Surface              | Opens with                      | Use it for                                                                           |
| -------------------- | ------------------------------- | ------------------------------------------------------------------------------------ |
| Agent Settings panel | {#action agent::OpenSettings}   | LLM providers, external agents, MCP servers                                          |
| Settings Editor      | {#action zed::OpenSettings}     | General Zed settings, `disable_ai`, tool permissions, edit prediction provider setup |
| Settings file        | {#action zed::OpenSettingsFile} | Direct JSON edits and settings not exposed in UI                                     |

For general settings mechanics, see [Configuring Zed](../configuring-zed.md).

## Configure Tools and Instructions {#tools-and-instructions}

- Use [Tools](./tools.md) to understand built-in Zed Agent tools.
- Use [Agent Profiles](./agent-profiles.md) to choose which tools and MCP tools are available.
- Use [Tool Permissions](./tool-permissions.md) to control whether permission-gated tool calls are allowed, denied, or confirmed.
- Use [Skills](./skills.md) for reusable task instructions.
- Use [Instructions](./instructions.md) for always-on personal and project instructions.
- Use [MCP](./mcp.md) to add external tools and context servers.

## Turn AI Off {#turn-ai-off}

Open the Settings Editor with {#action zed::OpenSettings}, search for `Disable AI`, and enable it.

You can also add this to your settings file:

```json [settings]
{
  "disable_ai": true
}
```

See [Privacy & Security](./privacy-and-security.md) for more context on AI data handling.
