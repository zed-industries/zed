---
title: Use Claude Agent, Gemini CLI, and Codex in Zed
description: Run Claude Agent, Gemini CLI, Codex, and other AI coding agents directly in Zed via the Agent Client Protocol (ACP).
---

# External Agents

Zed supports many external agents, including CLI-based ones, through the [Agent Client Protocol (ACP)](https://agentclientprotocol.com).

Zed supports [Gemini CLI](https://github.com/google-gemini/gemini-cli) (the reference ACP implementation), [Claude Agent](https://platform.claude.com/docs/en/agent-sdk/overview), [Codex](https://developers.openai.com/codex), [GitHub Copilot](https://github.com/github/copilot-language-server-release), and [additional agents](#add-more-agents) you can configure.

> Note that Zed's interaction with external agents is strictly UI-based; the billing, legal, and terms arrangement is directly between you and the agent provider.
> Zed does not charge for use of external agents, and our [zero-data retention agreements/privacy guarantees](./ai-improvement.md) are **_only_** applicable for Zed's hosted models.

## Gemini CLI {#gemini-cli}

Zed provides the ability to run [Gemini CLI](https://github.com/google-gemini/gemini-cli) directly in the [agent panel](./agent-panel.md).
Under the hood we run Gemini CLI in the background, and talk to it over ACP.

### Getting Started

First open the agent panel with {#kb agent::ToggleFocus}, and then use the `+` button in the top right to start a new Gemini CLI thread.

If you'd like to bind this to a keyboard shortcut, you can do so by editing your `keymap.json` file via the `zed: open keymap file` command to include:

```json [keymap]
[
  {
    "bindings": {
      "cmd-alt-g": [
        "agent::NewExternalAgentThread",
        { "agent": { "custom": { "name": "gemini" } } }
      ]
    }
  }
]
```

#### Installation

The first time you create a Gemini CLI thread, Zed will install [@google/gemini-cli](https://github.com/google-gemini/gemini-cli).
This installation is only available to Zed and is kept up to date as you use the agent.

#### Authentication

After you have Gemini CLI running, you'll be prompted to authenticate.

Click the "Login" button to open the Gemini CLI interactively, where you can log in with your Google account or [Vertex AI](https://cloud.google.com/vertex-ai) credentials.
Zed does not see your OAuth or access tokens in this case.

If the `GEMINI_API_KEY` environment variable (or `GOOGLE_AI_API_KEY`) is already set, or you have configured a Google AI API key in Zed's [language model provider settings](./llm-providers.md#google-ai), it will be passed to Gemini CLI automatically.

For more information, see the [Gemini CLI docs](https://github.com/google-gemini/gemini-cli/blob/main/docs/index.md).

### Usage

Gemini CLI supports the same workflows as Zed's first-party agent: code generation, refactoring, debugging, and Q&A. Add context by @-mentioning files, recent threads, or symbols.

> Some agent panel features are not yet available with Gemini CLI: editing past messages, resuming threads from history, and checkpointing.

## Claude Agent

Similar to Gemini CLI, you can also run [Claude Agent](https://platform.claude.com/docs/en/agent-sdk/overview) directly via Zed's [agent panel](./agent-panel.md).
Under the hood, Zed runs the Claude Agent SDK, which runs Claude Code under the hood, and communicates to it over ACP, through [a dedicated adapter](https://github.com/zed-industries/claude-agent-acp).

### Getting Started

Open the agent panel with {#kb agent::ToggleFocus}, and then use the `+` button in the top right to start a new Claude Agent thread.

If you'd like to bind this to a keyboard shortcut, you can do so by editing your `keymap.json` file via the `zed: open keymap file` command to include:

```json [keymap]
[
  {
    "bindings": {
      "cmd-alt-c": [
        "agent::NewExternalAgentThread",
        { "agent": { "custom": { "name": "claude-acp" } } }
      ]
    }
  }
]
```

### Authentication

As of version `0.202.7`, authentication to Zed's Claude Agent installation is decoupled entirely from Zed's agent.
That is to say, an Anthropic API key added via the [Zed Agent's settings](./llm-providers.md#anthropic) will _not_ be utilized by Claude Agent for authentication and billing.

To ensure you're using your billing method of choice, [open a new Claude Agent thread](./agent-panel.md#new-thread).
Then, run `/login`, and authenticate either via API key, or via `Log in with Claude Code` to use a Claude Pro/Max subscription.

#### Installation

The first time you create a Claude Agent thread, Zed will install [@zed-industries/claude-agent-acp](https://github.com/zed-industries/claude-agent-acp).
This installation is only available to Zed and is kept up to date as you use the agent.

Zed will always use this managed version of the Claude Agent adapter, which includes a vendored version of the Claude Code CLI, even if you have it installed globally.

If you want to override the executable used by the adapter, you can set the `CLAUDE_CODE_EXECUTABLE` environment variable in your settings to the path of your preferred executable.

```json
{
  "agent_servers": {
    "claude-acp": {
      "type": "registry",
      "env": {
        "CLAUDE_CODE_EXECUTABLE": "/path/to/alternate-claude-code-executable"
      }
    }
  }
}
```

### Usage

Claude Agent supports the same workflows as Zed's first-party agent. Add context by @-mentioning files, recent threads, diagnostics, or symbols.

In complement to talking to it [over ACP](https://agentclientprotocol.com), Zed relies on the [Claude Agent SDK](https://platform.claude.com/docs/en/agent-sdk/overview) to support some of its specific features.
However, the SDK doesn't yet expose everything needed to fully support all of them:

- Slash Commands: [Custom slash commands](https://code.claude.com/docs/en/slash-commands#custom-slash-commands) are fully supported, and have been merged into skills. A subset of [built-in commands](https://code.claude.com/docs/en/slash-commands#built-in-slash-commands) are supported.
- [Subagents](https://code.claude.com/docs/en/sub-agents) are supported.
- [Agent teams](https://code.claude.com/docs/en/agent-teams) are currently _not_ supported.
- [Hooks](https://code.claude.com/docs/en/hooks-guide) are currently _not_ supported.

> Some [agent panel](./agent-panel.md) features are not yet available with Claude Agent: editing past messages, resuming threads from history, and checkpointing.

#### CLAUDE.md

Claude Agent in Zed will automatically use any `CLAUDE.md` file found in your project root, project subdirectories, or root `.claude` directory.

If you don't have a `CLAUDE.md` file, you can ask Claude Agent to create one for you through the `init` slash command.

## Codex CLI

You can also run [Codex CLI](https://github.com/openai/codex) directly via Zed's [agent panel](./agent-panel.md).
Under the hood, Zed runs Codex CLI and communicates to it over ACP, through [a dedicated adapter](https://github.com/zed-industries/codex-acp).

### Getting Started

As of version `0.208`, you should be able to use Codex directly from Zed.
Open the agent panel with {#kb agent::ToggleFocus}, and then use the `+` button in the top right to start a new Codex thread.

If you'd like to bind this to a keyboard shortcut, you can do so by editing your `keymap.json` file via the `zed: open keymap file` command to include:

```json
[
  {
    "bindings": {
      "cmd-alt-c": [
        "agent::NewExternalAgentThread",
        { "agent": { "custom": { "name": "codex-acp" } } }
      ]
    }
  }
]
```

### Authentication

Authentication to Zed's Codex installation is decoupled entirely from Zed's agent.
That is to say, an OpenAI API key added via the [Zed Agent's settings](./llm-providers.md#openai) will _not_ be utilized by Codex for authentication and billing.

To ensure you're using your billing method of choice, [open a new Codex thread](./agent-panel.md#new-thread).
The first time you will be prompted to authenticate with one of three methods:

1. Login with ChatGPT - allows you to use your existing, paid ChatGPT subscription. _Note: This method isn't currently supported in remote projects_
2. `CODEX_API_KEY` - uses an API key you have set in your environment under the variable `CODEX_API_KEY`.
3. `OPENAI_API_KEY` - uses an API key you have set in your environment under the variable `OPENAI_API_KEY`.

If you are already logged in and want to change your authentication method, type `/logout` in the thread and authenticate again.

If you want to use a third-party provider with Codex, you can configure that with your [Codex config.toml](https://github.com/openai/codex/blob/main/docs/config.md#model-selection) or pass extra [args/env variables](https://github.com/openai/codex/blob/main/docs/config.md#model-selection) to your Codex agent servers settings.

#### Installation

The first time you create a Codex thread, Zed will install [codex-acp](https://github.com/zed-industries/codex-acp).
This installation is only available to Zed and is kept up to date as you use the agent.

Zed will always use this managed version of Codex even if you have it installed globally.

### Usage

Codex supports the same workflows as Zed's first-party agent. Add context by @-mentioning files or symbols.

> Some agent panel features are not yet available with Codex: editing past messages, resuming threads from history, and checkpointing.

## Add More Agents {#add-more-agents}

### Via Agent Server Extensions

<div class="warning">

Starting from `v0.221.x`, [the ACP Registry](https://agentclientprotocol.com/registry) is the preferred way to install external agents in Zed.
Learn more about it in [the release blog post](https://zed.dev/blog/acp-registry).
At some point in the near future, Agent Server extensions will be deprecated.

</div>

Add more external agents to Zed by installing [Agent Server extensions](../extensions/agent-servers.md).

See what agents are available by filtering for "Agent Servers" in the extensions page, which you can access via the command palette with `zed: extensions`, or the [Zed website](https://zed.dev/extensions?filter=agent-servers).

### Via The ACP Registry

#### Overview

As mentioned above, the Agent Server extensions will be deprecated in the near future to give room to the ACP Registry.

[The ACP Registry](https://github.com/agentclientprotocol/registry) lets developers distribute ACP-compatible agents to any client that implements the protocol. Agents installed from the registry update automatically.

At the moment, the registry is a curated set of agents, including only the ones that [support authentication](https://agentclientprotocol.com/rfds/auth-methods).

#### Using it in Zed

Use the `zed: acp registry` command to quickly go to the ACP Registry page.
There's also a button ("Add Agent") that takes you there in the agent panel's configuration view.

From there, you can click to install your preferred agent and it will become available right away in the `+` icon button in the agent panel.

> If you installed the same agent through both the extension and the registry, the registry version takes precedence.

### Custom Agents

You can also add agents through your settings file ([how to edit](../configuring-zed.md#settings-files)) by specifying certain fields under `agent_servers`, like so:

```json [settings]
{
  "agent_servers": {
    "My Custom Agent": {
      "type": "custom",
      "command": "node",
      "args": ["~/projects/agent/index.js", "--acp"],
      "env": {}
    }
  }
}
```

This can be useful if you're in the middle of developing a new agent that speaks the protocol and you want to debug it.

It's also possible to customize environment variables for registry-installed agents like Claude Agent, Codex, and Gemini CLI by using their registry names (`claude-acp`, `codex-acp`, `gemini`) with `"type": "registry"` in your settings.

## Debugging Agents

When using external agents in Zed, you can access the debug view via with `dev: open acp logs` from the Command Palette.
This lets you see the messages being sent and received between Zed and the agent.

![The debug view for ACP logs.](https://zed.dev/img/acp/acp-logs.webp)

It's helpful to attach data from this view if you're opening issues about problems with external agents like Claude Agent, Codex, OpenCode, etc.

## MCP Servers

Note that for external agents, access to MCP servers [installed from Zed](./mcp.md) may vary depending on the ACP implementation.
For example, Claude Agent and Codex both support it, but Gemini CLI does not yet.
