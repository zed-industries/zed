# External Agents

Zed supports terminal-based agentic coding tools through the [Agent Client Protocol (ACP)](https://agentclientprotocol.com).

Currently, [Gemini CLI](https://github.com/google-gemini/gemini-cli) serves as the reference implementation, and you can [add custom ACP-compatible agents](#add-custom-agents) as well.

## Gemini CLI {#gemini-cli}

Zed provides the ability to run [Gemini CLI](https://github.com/google-gemini/gemini-cli) directly in the [agent panel](./agent-panel.md).

Under the hood we run Gemini CLI in the background, and talk to it over ACP.
This means that you're running the real Gemini CLI, with all of the advantages of that, but you can see and interact with files in your editor.

### Getting Started

As of Zed Stable v0.201.5 you should be able to use Gemini CLI directly from Zed. First open the agent panel with {#kb agent::ToggleFocus}, and then use the `+` button in the top right to start a New Gemini CLI thread.

If you'd like to bind this to a keyboard shortcut, you can do so by editing your keybindings file to include:

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

If you don't yet have Gemini CLI installed, then Zed will install a version for you. If you do, then we will use the version of Gemini CLI on your path.

You need to be running at least Gemini version `0.2.0`, and if your version of Gemini is too old you will see an
error message.

The instructions to upgrade Gemini depend on how you originally installed it, but typically, running `npm install -g @google/gemini-cli@latest` should work.

#### Authentication

After you have Gemini CLI running, you'll be prompted to choose your authentication method.

Most users should click the "Log in with Google". This will cause a browser window to pop-up and auth directly with Gemini CLI. Zed does not see your oauth or access tokens in this case.

You can also use the "Gemini API Key". If you select this, and have the `GEMINI_API_KEY` set, then we will use that. Otherwise Zed will prompt you for an API key which will be stored securely in your keychain, and used to start Gemini CLI from within Zed.

The "Vertex AI" option is for those who are using Vertex AI, and have already configured their environment correctly.

For more information, see the [Gemini CLI docs](https://github.com/google-gemini/gemini-cli/blob/main/docs/index.md).

### Usage

Similar to Zed's first-party agent, you can use Gemini CLI to do anything that you need.

You can @-mention files, recent threads, symbols, or fetch the web.

Note that some first-party agent features don't yet work with Gemini CLI: editing past messages, resuming threads from history, checkpointing, and using the agent in SSH projects.
We hope to add these features in the near future.

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

## Debugging Agents

When using external agents in Zed, you can access the debug view via with `dev: open acp logs` from the Command Palette. This lets you see the messages being sent and received between Zed and the agent.

![The debug view for ACP logs.](https://zed.dev/img/acp/acp-logs.webp)
