# External Agents

Through the Agent Client Protocol (ACP), Zed can expose external agents that run as a subprocess.

Read the ACP documentation to learn how to add your agent to Zed.

At the moment, Zed supports [Gemini CLI](https://github.com/google-gemini/gemini-cli) as a reference implementation of an external agent speaking ACP.

## Gemini CLI

Zed provides the ability to run [Gemini CLI](https://github.com/google-gemini/gemini-cli) directly in the [agent panel](./agent-panel.md).

Under the hood we run Gemini CLI in the background, and talk to it over the [Agent Client Protocol (ACP)](https://agentclientprotocol.com).
This means that you're running the real Gemini CLI, with all of the advantages of that, but you can see and interact with files in your editor.

### Getting Started

The Gemini integration should be enabled by default.
To access it, run `agent: new gemini cli thread`.

#### Installation

If you don't yet have Gemini CLI installed, then Zed will install a version for you.
If you do, then we will use the version of Gemini CLI on your path.

If the version you haven installed is too old, you will see an error message, and you will have to upgrade Gemini CLI.
The instructions depend on how you originally installed it, but typically, running `npm install -g gemini-cli@preview` should work to fix it.

#### Authentication

After you have Gemini CLI running, you'll be prompted to choose your authentication method.
Most users should click the "Log in with Google" button that will show up in the UI, but if you have an API key already you can also click "Use Gemini API Key".

The "Vertex AI" option is for those who are using Vertex AI, and have already configured their environment correctly.

For more information, see the [Gemini CLI docs](https://github.com/google-gemini/gemini-cli/blob/main/docs/index.md).

### Usage

Similar to the built-in agent in [the agent panel](./agent-panel.md), you can use Gemini CLI to do anything that you need.
You can @-mention files, recent conversations, symbols, or fetch the web.

In this initial version, some features supported by the first-party agent are not avaialble for Gemini CLI.
Capacities such as editing previous messages, checkpoints, profile selection, and others should hopefully be added in the future.
