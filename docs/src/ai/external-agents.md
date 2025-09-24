# External Agents

Zed supports terminal-based agents through the [Agent Client Protocol (ACP)](https://agentclientprotocol.com).

Currently, [Gemini CLI](https://github.com/google-gemini/gemini-cli) serves as the reference implementation.
[Claude Code](https://www.anthropic.com/claude-code) is also included by default, and you can [add custom ACP-compatible agents](#add-custom-agents) as well.

Zed's affordance for external agents is strictly UI-based; the billing and legal/terms arrangement is directly between you and the agent provider. Zed does not charge for use of external agents, and our [zero-data retention agreements/privacy guarantees](./ai-improvement.md) are **_only_** applicable for Zed's hosted models.

## Gemini CLI {#gemini-cli}

Zed provides the ability to run [Gemini CLI](https://github.com/google-gemini/gemini-cli) directly in the [agent panel](./agent-panel.md).

Under the hood we run Gemini CLI in the background, and talk to it over ACP.
This means that you're running the real Gemini CLI, with all of the advantages of that, but you can see and interact with files in your editor.

### Getting Started

As of [Zed Stable v0.201.5](https://zed.dev/releases/stable/0.201.5) you should be able to use Gemini CLI directly from Zed. First open the agent panel with {#kb agent::ToggleFocus}, and then use the `+` button in the top right to start a new Gemini CLI thread.

If you'd like to bind this to a keyboard shortcut, you can do so by editing your `keymap.json` file via the `zed: open keymap` command to include:

```json
[
  {
    "bindings": {
      "cmd-alt-g": ["agent::NewExternalAgentThread", { "agent": "gemini" }]
    }
  }
]
```

#### Installation

The first time you create a Gemini CLI thread, Zed will install [@google/gemini-cli](https://github.com/zed-industries/claude-code-acp). This installation is only available to Zed and is kept up to date as you use the agent.

By default, Zed will use this managed version of Gemini CLI even if you have it installed globally. However, you can configure it to use a version in your `PATH` by adding this to your settings:

```json
{
  "agent_servers": {
    "gemini": {
      "ignore_system_version": false
    }
  }
}
```

#### Authentication

After you have Gemini CLI running, you'll be prompted to choose your authentication method.

Most users should click the "Log in with Google". This will cause a browser window to pop-up and auth directly with Gemini CLI. Zed does not see your OAuth or access tokens in this case.

You can also use the "Gemini API Key". If you select this, and have the `GEMINI_API_KEY` set, then we will use that. Otherwise Zed will prompt you for an API key which will be stored securely in your keychain, and used to start Gemini CLI from within Zed.

The "Vertex AI" option is for those who are using [Vertex AI](https://cloud.google.com/vertex-ai), and have already configured their environment correctly.

For more information, see the [Gemini CLI docs](https://github.com/google-gemini/gemini-cli/blob/main/docs/index.md).

### Usage

Similar to Zed's first-party agent, you can use Gemini CLI to do anything that you need.
And to give it context, you can @-mention files, recent threads, symbols, or fetch the web.

> Note that some first-party agent features don't yet work with Gemini CLI: editing past messages, resuming threads from history, checkpointing, and using the agent in SSH projects.
> We hope to add these features in the near future.

## Claude Code

Similar to Gemini CLI, you can also run [Claude Code](https://www.anthropic.com/claude-code) directly via Zed's [agent panel](./agent-panel.md).
Under the hood, Zed runs Claude Code and communicate to it over ACP, through [a dedicated adapter](https://github.com/zed-industries/claude-code-acp).

### Getting Started

Open the agent panel with {#kb agent::ToggleFocus}, and then use the `+` button in the top right to start a new Claude Code thread.

If you'd like to bind this to a keyboard shortcut, you can do so by editing your `keymap.json` file via the `zed: open keymap` command to include:

```json
[
  {
    "bindings": {
      "cmd-alt-c": ["agent::NewExternalAgentThread", { "agent": "claude_code" }]
    }
  }
]
```

### Authentication

As of version `0.202.7` (stable) and `0.203.2` (preview), authentication to Zed's Claude Code installation is decoupled entirely from Zed's agent. That is to say, an Anthropic API key added via the [Zed Agent's settings](./llm-providers.md#anthropic) will _not_ be utilized by Claude Code for authentication and billing.

To ensure you're using your billing method of choice, [open a new Claude Code thread](./agent-panel.md#new-thread). Then, run `/login`, and authenticate either via API key, or via `Log in with Claude Code` to use a Claude Pro/Max subscription.

#### Installation

The first time you create a Claude Code thread, Zed will install [@zed-industries/claude-code-acp](https://github.com/zed-industries/claude-code-acp). This installation is only available to Zed and is kept up to date as you use the agent.

Zed will always use this managed version of Claude Code even if you have it installed globally.

### Usage

Similar to Zed's first-party agent, you can use Claude Code to do anything that you need.
And to give it context, you can @-mention files, recent threads, symbols, or fetch the web.

In complement to talking to it [over ACP](https://agentclientprotocol.com), Zed relies on the [Claude Code SDK](https://docs.anthropic.com/en/docs/claude-code/sdk/sdk-overview) to support some of its specific features.
However, the SDK doesn't yet expose everything needed to fully support all of them:

- Slash Commands: A subset of [built-in commands](https://docs.anthropic.com/en/docs/claude-code/slash-commands#built-in-slash-commands) are supported, while [custom slash commands](https://docs.anthropic.com/en/docs/claude-code/slash-commands#custom-slash-commands) are fully supported.
- [Subagents](https://docs.anthropic.com/en/docs/claude-code/sub-agents) are supported.
- [Hooks](https://docs.anthropic.com/en/docs/claude-code/hooks-guide) are currently _not_ supported.

> Also note that some [first-party agent](./agent-panel.md) features don't yet work with Claude Code: editing past messages, resuming threads from history, checkpointing, and using the agent in SSH projects.
> We hope to add these features in the near future.

#### CLAUDE.md

Claude Code in Zed will automatically use any `CLAUDE.md` file found in your project root, project subdirectories, or root `.claude` directory.

If you don't have a `CLAUDE.md` file, you can ask Claude Code to create one for you through the `init` slash command.

## Add Custom Agents {#add-custom-agents}

You can run any agent speaking ACP in Zed by changing your settings as follows:

```json
{
  "agent_servers": {
    "Custom Agent": {
      "command": "node",
      "args": ["~/projects/agent/index.js", "--acp"],
      "env": {}
    }
  }
}
```

This can also be useful if you're in the middle of developing a new agent that speaks the protocol and you want to debug it.

You can also specify a custom path, arguments, or environment for the builtin integrations by using the `claude` and `gemini` names.

## Debugging Agents

When using external agents in Zed, you can access the debug view via with `dev: open acp logs` from the Command Palette. This lets you see the messages being sent and received between Zed and the agent.

![The debug view for ACP logs.](https://zed.dev/img/acp/acp-logs.webp)
