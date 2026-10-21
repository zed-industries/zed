---
title: Agent Settings - Zed
description: Map the AI settings pages to Zed AI setup for LLM providers, External Agents, MCP servers, and related settings.
---

# Agent Settings

Agent Settings live in the **AI** section of the Settings Editor, which configures model providers, External Agents, and MCP servers.
Open it with {#action agent::OpenSettings} (also available from the top-right menu in the [Agent Panel](./agent-panel.md)), which takes you straight to the AI page.
You can also reach the same page with {#action zed::OpenSettings} and selecting **AI** in the sidebar.

Within the AI page, LLM Providers, External Agents, and MCP Servers each open as their own sub-page under the **General** section.

| Surface         | Opens with                      | Use it for                                                                  |
| --------------- | ------------------------------- | --------------------------------------------------------------------------- |
| Settings Editor | {#action agent::OpenSettings}   | AI settings: LLM providers, External Agents, MCP servers, and related pages |
| Settings file   | {#action zed::OpenSettingsFile} | Direct JSON edits and settings not exposed in UI                            |

For general settings mechanics, see [Configuring Zed](../configuring-zed.md).

## LLM Providers {#llm-providers}

The `LLM Providers` section configures model providers for Zed AI features, including Zed Agent, Inline Assistant, Git commit generation, thread summaries, and similar model-backed features.

Use this section to:

- sign in to supported subscription-backed providers
- enter provider API keys
- add OpenAI-compatible providers
- remove providers

For the model-access paths and provider-specific setup, see [LLM Providers](./llm-providers.md).

## Feature-Specific Settings {#feature-specific-settings}

Some Zed AI features have their own model or prompt settings in `settings.json`, including:

- `agent.inline_assistant_model`
- `agent.commit_message_model`
- `agent.thread_summary_model`
- `agent.compaction_model`
- `agent.subagent_model`
- `agent.commit_message_instructions`
- `agent.inline_alternatives`

Use `agent.commit_message_instructions` for instructions that apply only to generated Git commit messages:

```json [settings]
{
  "agent": {
    "commit_message_instructions": "Use the Conventional Commits format: <type>(<scope>): <description>."
  }
}
```

For feature-specific model examples, see [Feature-specific Models](#feature-specific-models).

## Automatic Compaction {#automatic-compaction}

Zed Agent can automatically compact long threads before they reach the selected model's context window. Compaction summarizes earlier messages and keeps the conversation usable without starting a new thread.

Automatic compaction is enabled by default and runs when the thread reaches `90%` of the model's context window. You can change the threshold or disable automatic compaction in `settings.json`:

```json [settings]
{
  "agent": {
    "auto_compact": {
      "enabled": true,
      "threshold": "90%"
    }
  }
}
```

The `threshold` value can be one of:

| Value                           | Meaning                                                                        |
| ------------------------------- | ------------------------------------------------------------------------------ |
| Percentage string, like `90%`   | Compact when the thread uses that percentage of the model's context window.    |
| Positive integer, like `100000` | Compact after that many tokens have been used.                                 |
| Negative integer, like `-20000` | Compact once fewer than that many tokens remain in the model's context window. |

`0` is not a valid threshold. If the threshold is invalid, Zed falls back to `90%`.

You can compact a Zed Agent thread manually at any time by typing `/compact` in the Agent Panel message editor. For more on thread token usage and compaction behavior, see [Token Usage and Compaction](./agent-panel.md#token-usage).

## Compaction Model {#compaction-model}

By default, context compaction (both `/compact` and auto-compaction) uses the thread's currently selected model. Set `agent.compaction_model` to use a different model:

```json [settings]
{
  "agent": {
    "default_model": {
      "provider": "anthropic",
      "model": "claude-opus-4-6"
    },
    "compaction_model": {
      "provider": "anthropic",
      "model": "claude-sonnet-4-5"
    }
  }
}
```

**Notes:**

- The configured model should have a context window at least as large as the thread's primary model for predictable behavior.

## External Agents {#external-agents}

The External Agents section configures ACP-integrated agents.

Use `Add Agent` to:

- `Install from Registry`
- `Add Custom Agent`

For setup details and support boundaries, see [External Agents](./external-agents.md).

## MCP Servers {#mcp-servers}

The `MCP Servers` page configures Model Context Protocol servers connected to Zed.

Use `Add Server` (in the page header) to:

- `Add Local Server`
- `Add Remote Server`
- `Install from Extensions`

Each configured server row exposes:

- a `Configure` (gear) button to edit the server
- an `Uninstall` (trash) button to remove it
- a toggle to enable or disable it

For MCP setup, auth, server status, and agent-path boundaries, see [MCP](./mcp.md).

## Related Configuration {#related-configuration}

Some AI settings are not configured on the AI settings pages:

| Task                                                         | Go to                                          |
| ------------------------------------------------------------ | ---------------------------------------------- |
| Choose which tools are available in a Zed Agent thread       | [Agent Profiles](./agent-profiles.md)          |
| Control whether tool calls are allowed, denied, or confirmed | [Tool Permissions](./tool-permissions.md)      |
| Configure reusable task instructions                         | [Skills](./skills.md)                          |
| Configure always-on personal or project instructions         | [Instructions](./instructions.md)              |
| Configure edit prediction providers                          | [Edit Prediction](./edit-prediction.md)        |
| Turn AI off                                                  | [AI Quick Start](./quick-start.md#turn-ai-off) |
| Edit raw settings JSON                                       | [Configuring Zed](../configuring-zed.md)       |

## Feature-Specific Models {#feature-specific-models}

Zed supports feature-specific model settings for Inline Assistant, Git commit generation, thread summaries, and subagents. Configure these in settings when you need a different model for a specific workflow.

See [LLM Providers](./llm-providers.md) for model access, and [All Settings](../reference/all-settings.md) for the complete settings reference.

### Subagent Models {#subagent-models}

By default, Zed Agent's subagents inherit the parent thread's model, thinking setting, reasoning effort, and speed. Set `agent.subagent_model` in `settings.json` to choose separate model, reasoning, and speed settings. You can override the model by asking the agent to use a specific model for a subagent. When that model matches the provider and model in `agent.subagent_model`, its `enable_thinking`, `effort`, and `speed` settings are preserved. This setting does not configure [External Agents](./external-agents.md).

Configure this setting with a `provider` and `model`. To choose a reasoning effort for a model that supports it, also set `enable_thinking` and `effort`:

```json [settings]
{
  "agent": {
    "subagent_model": {
      "effort": "low",
      "enable_thinking": true,
      "model": "claude-opus-4-6",
      "provider": "anthropic"
    }
  }
}
```

Choose an `effort` value supported by the selected model. Omitting `effort` from this object does not inherit the parent's effort. It leaves effort to the selected provider and model's default behavior. Omitting `enable_thinking` defaults to `false`, although models that cannot disable thinking still use it.

To find supported effort values:

1. Choose the target provider and model from the model selector in a native Zed Agent thread, even if it is already selected, to save it as `agent.default_model`.
2. Enable thinking if it is off, then open the thinking effort menu. The "Change Thinking Effort" menu lists the model's supported levels. If the model offers no effort choices, omit `effort`.
3. Choose a level and read `agent.default_model.effort` in `settings.json` for the exact value to use in `agent.subagent_model.effort`.

These controls change the current thread and saved default model settings, and can update a matching favorite model. Restore your previous selections afterward if needed.

The optional `speed` field accepts `"fast"` or `"standard"`. It applies only to models that support fast mode. Omitting it from `agent.subagent_model` leaves speed unspecified rather than inheriting the parent's speed.

| Model Selection                                                                         | Settings Applied                                                                                                                                                            |
| --------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| No specific model requested, and `agent.subagent_model` is unset                        | Inherits the parent thread's current thinking, effort, and speed settings.                                                                                                  |
| No specific model requested, and `agent.subagent_model` is configured                   | Uses `enable_thinking`, `effort`, and `speed` from `agent.subagent_model`.                                                                                                  |
| Requested model matches the provider and model in `agent.subagent_model`                | Uses `enable_thinking`, `effort`, and `speed` from `agent.subagent_model`.                                                                                                  |
| Requested model differs from `agent.subagent_model`, or `agent.subagent_model` is unset | Uses matching `agent.default_model` settings. Without a match, enables thinking if supported, uses the model's default effort when available, and leaves speed unspecified. |

When using matching `agent.default_model` settings, unsupported or omitted `effort` falls back to the selected model's default effort, when available.

If the configured model is unavailable, subagents use the parent thread's model and its thinking, effort, and speed settings.

The agent can only request a model for a subagent, not a separate effort or speed. Requesting the parent's own model is not the same as requesting no specific model: the table's rules for requested models apply instead of inheriting the parent's current settings. Subagents with a requested or configured model do not follow later changes to the parent's model, reasoning, or speed settings. Resuming a subagent keeps its existing model, reasoning, and speed settings.

## Model Temperature {#model-temperature}

Most Zed AI features use the selected model's default generation behavior.
Use `agent.model_parameters` when you need to set a temperature for a provider,
a model, or a specific provider/model pair.

```json [settings]
{
  "agent": {
    "model_parameters": [
      {
        "provider": "anthropic",
        "model": "claude-sonnet-4-5",
        "temperature": 0.2
      }
    ]
  }
}
```

Zed checks matching entries from last to first. An entry can omit `provider` or
`model` to apply more broadly. For provider-specific model configuration such as
custom model entries, context windows, or gateway routing, see
[LLM Providers](./llm-providers.md) and the provider setup pages.

## Rules, Skills, and Instructions {#rules-skills-instructions}

Reusable Rules have been replaced by [Skills](./skills.md). Always-on Rules have moved to [Instructions](./instructions.md), including personal `AGENTS.md` and project instruction files.

Older builds or transitional UI may still refer to `Rules`. Use [Skills](./skills.md) for reusable task instructions and [Instructions](./instructions.md) for always-on context.
