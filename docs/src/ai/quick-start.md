---
title: AI Quick Start - Zed
description: Choose the right Zed AI setup path for agents, providers, subscriptions, local models, edit prediction, and privacy.
---

# AI Quick Start

Use this page to route to the right setup path. Each section links to the canonical page for the details.

## I want Zed's built-in agent {#zed-agent}

Use [Zed Agent](./zed-agent.md) when you want Zed's native agent to read, edit, search, and run code in your project.

- Configure model access with [LLM Providers](./llm-providers.md).
- Use the [Agent Panel](./agent-panel.md) to prompt and review changes.
- Use [Agent Profiles](./agent-profiles.md) to choose which tools and MCP tools are available.

## I want Claude, Codex, Gemini, or another agent CLI in Zed {#agent-cli}

Use [External Agents](./external-agents.md) when the agent is available through ACP. Use [Terminal Threads](./terminal-threads.md) when you want the native CLI or TUI running directly in a terminal-backed thread.

External agents and terminal threads usually own their own auth, model configuration, subscriptions, tools, instructions, and MCP configuration.

## I want to use Zed-hosted models {#zed-hosted-models}

Use [Zed-Hosted Models](../account/zed-hosted-models.md) when you want model access through a Zed plan.

- See [Plans & Pricing](../account/plans-and-pricing.md) to compare plans.
- Use [Billing](../account/billing.md) for invoices, payment, and spend limits.

## I want to use an LLM subscription I already pay for {#existing-subscription}

Use [Use an Existing Subscription](./use-an-existing-subscription.md) when you already pay for ChatGPT, Claude, Copilot, Cursor, OpenCode, or another AI product.

Some subscriptions work as Zed model providers. Others are used through an external agent or terminal CLI.

## I want to bring my own API key {#api-access}

Use [Use API Access](./use-api-access.md) when a provider gives you an API key, API credits, top-ups, or usage billing.

Provider keys saved through Zed are stored in the system keychain, not in `settings.json`.

## I want to use a gateway {#gateway}

Use [Use a Gateway](./use-a-gateway.md) for OpenRouter, Vercel AI Gateway, Amazon Bedrock, or a similar platform.

## I want to use a local model {#local-model}

Use [Use a Local Model](./use-a-local-model.md) for Ollama, LM Studio, local OpenAI-compatible servers, or local/self-hosted edit prediction.

## I want AI autocomplete, not an agent {#edit-prediction}

Use [Edit Prediction](./edit-prediction.md). Edit prediction has its own provider setup and is separate from LLM providers used by the Zed Agent and Inline Assistant.

## I want inline code edits {#inline-edits}

Use [Inline Assistant](./inline-assistant.md) when you want to rewrite selected code, terminal text, or other text in place.

## I want to run multiple AI tasks at once {#parallel-agents}

Use [Parallel Agents](./parallel-agents.md) to run multiple threads across projects and worktrees.

## I want AI help with Git {#git}

Zed can generate commit messages from the Git panel using your configured model. See [LLM Providers](./llm-providers.md) for model access and [Git](../git.md) for Git workflows.

## I want to control what the agent can do {#control-tools}

- Use [Agent Profiles](./agent-profiles.md) to choose which tools are available.
- Use [Tool Permissions](./tool-permissions.md) to control allow, deny, and confirm behavior.
- Use [MCP](./mcp.md) to add external tools.

## I want to understand privacy before enabling AI {#privacy}

Read [Privacy & Security](./privacy-and-security.md). For model-improvement controls, see [AI Improvement](./ai-improvement.md).

## I want to understand AI plans, usage, and billing {#billing}

Use [Plans & Pricing](../account/plans-and-pricing.md), [Zed-Hosted Models](../account/zed-hosted-models.md), and [Billing](../account/billing.md).

## I want to turn AI off {#turn-ai-off}

Open the Settings Editor with {#action zed::OpenSettings}, search for `Disable AI`, and enable it.

You can also add this to your settings file:

```json [settings]
{
  "disable_ai": true
}
```
