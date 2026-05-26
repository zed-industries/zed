---
title: AI Quick Start - Zed
description: Choose the right Zed AI setup path for agents, models, subscriptions, local models, edit prediction, and privacy.
---

# AI Quick Start

Use this page to choose the right AI setup path in Zed. If you already know the company, subscription, model provider, agent, or CLI you want to use, start with [AI by Company](./by-company.md).

## I want Zed's built-in agent {#zed-agent}

Use [Zed Agent](./zed-agent.md) when you want Zed's native agent to read, edit, search, and run code in your project.

Zed Agent uses Zed-configured models from [LLM Providers](./llm-providers.md). It also uses Zed's built-in tools, [Agent Profiles](./agent-profiles.md), [Skills](./skills.md), [Instructions](./instructions.md), and [MCP servers](./mcp.md).

Start in the [Agent Panel](./agent-panel.md) to prompt the agent, add context, review changes, and manage threads.

## I want to use another coding agent in Zed {#agent-cli}

Use this path for Claude, Codex, OpenCode, Copilot, Cursor, Pi Coding Agent, Gemini CLI, or another coding agent.

| If the agent...                 | Use                                       |
| ------------------------------- | ----------------------------------------- |
| Integrates with Zed through ACP | [External Agents](./external-agents.md)   |
| Runs as a CLI or TUI            | [Terminal Threads](./terminal-threads.md) |

External Agents and Terminal Threads usually own their own auth, model configuration, subscriptions, tools, instructions, and MCP configuration.

## I want to choose which models Zed uses {#model-access}

| If you want to...                                                             | Use                                                               |
| ----------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| Use models billed through Zed                                                 | [Zed-Hosted Models](../account/zed-hosted-models.md)              |
| Bring your own provider API key, credits, top-ups, or usage billing           | [Use API Access](./use-api-access.md)                             |
| Use a subscription you already pay for                                        | [Use an Existing Subscription](./use-an-existing-subscription.md) |
| Use OpenRouter, Vercel AI Gateway, Amazon Bedrock, or another gateway         | [Use a Gateway](./use-a-gateway.md)                               |
| Use Ollama, LM Studio, local OpenAI-compatible servers, or self-hosted models | [Use a Local Model](./use-a-local-model.md)                       |

Provider keys saved through Zed are stored in the system keychain, not in `settings.json`.

## I want to change AI settings {#ai-settings}

| If you want to...                                                           | Go to                                                                    |
| --------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| Configure LLM providers, External Agents, or MCP servers                    | [Agent Settings](./agent-settings.md) with {#action agent::OpenSettings} |
| Disable AI, configure tool permissions, or set up edit prediction providers | Settings Editor with {#action zed::OpenSettings}                         |
| Edit advanced JSON-only settings                                            | Settings file with {#action zed::OpenSettingsFile}                       |

For general settings mechanics, see [Configuring Zed](../configuring-zed.md).

## I want a specific AI feature {#features}

| If you want to...                              | Use                                                                                                                                            |
| ---------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| Prompt agents, add context, and review changes | [Agent Panel](./agent-panel.md)                                                                                                                |
| Accept AI completions while typing             | [Edit Prediction](./edit-prediction.md)                                                                                                        |
| Rewrite selected code or terminal text         | [Inline Assistant](./inline-assistant.md)                                                                                                      |
| Run multiple AI tasks at once                  | [Parallel Agents](./parallel-agents.md)                                                                                                        |
| Generate commit messages                       | [Git commit generation](../git.md#ai-support-in-git)                                                                                           |
| Control tools and permissions                  | [Agent Profiles](./agent-profiles.md), [Tool Permissions](./tool-permissions.md), and [MCP](./mcp.md)                                          |
| Understand privacy and data controls           | [AI Privacy](./privacy-and-security.md) and [Feedback and Training Data](./ai-improvement.md)                                                  |
| Understand plans, usage, and billing           | [Plans & Pricing](../account/plans-and-pricing.md), [Zed-Hosted Models](../account/zed-hosted-models.md), and [Billing](../account/billing.md) |

## I want to turn AI off {#turn-ai-off}

Open the Settings Editor with {#action zed::OpenSettings}, search for `Disable AI`, and enable it.

You can also add this to your settings file:

```json [settings]
{
  "disable_ai": true
}
```
