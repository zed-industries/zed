# External Agents

Zed has support for integrating with existing terminal-based agentic coding tools through the [Agent Client Protocol (ACP)](https://agentclientprotocol.com).

At the moment, Zed supports [Gemini CLI](https://github.com/google-gemini/gemini-cli) as a reference implementation of an external agent speaking ACP.

You can also [configure your own](#custom-agents) if you'd like to add ACP support to an existing tool.

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

You need to be running at least Gemini version `0.2.0-preview`, and if your version of Gemini is too old you will see an
error message.

The instructions to upgrade Gemini depend on how you originally installed it, but typically, running `npm install -g gemini-cli@preview` should work.

#### Authentication

After you have Gemini CLI running, you'll be prompted to choose your authentication method.

Most users should click the "Log in with Google". This will cause a browser window to pop-up and auth directly with Gemini CLI. Zed does not see your oauth or access tokens in this case.

You can also use the "Gemini API Key". If you select this, and have the `GEMINI_API_KEY` set, then we will use that. Otherwise Zed will prompt you for an API key which will be stored securely in your keychain, and used to start Gemini CLI from within Zed.

The "Vertex AI" option is for those who are using Vertex AI, and have already configured their environment correctly.

For more information, see the [Gemini CLI docs](https://github.com/google-gemini/gemini-cli/blob/main/docs/index.md).

### Usage

Similar to the built-in agent in [the agent panel](./agent-panel.md), you can use Gemini CLI to do anything that you need.

You can @-mention files, recent conversations, symbols, or fetch the web.

There are two features that don't yet work with Gemini CLI: editing past messages, which we hope to add support for soon; and resuming a conversation from history.

## Custom Agents {#custom-agents}

If you have written (or are writing) a tool that speaks ACP, and you'd like to test it with Zed, you can add it to your settings:

```json
{
  "agent_servers": {
    "Claude Code": {
      "command": "node",
      "args": [
        "/Users/conrad/projects/claude-code-acp/index.js",
        "--acp"
      ]
    }
  }
}
```

Zed has some support for debugging acp connections, and you can open the debug view with `dev: open acp logs` from the command line.
