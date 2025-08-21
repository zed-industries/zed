# Gemini CLI

Zed provides the ability to run Gemini CLI directly in our [Agent Panel](./agent-panel.md).

Under the hood we run Gemini CLI in the background, and talk to it over [ACP](TODO TODO). This means that you're running the real Gemini CLI, with all of the advantages of that, but you can see and interact with files in your editor.

## Getting Started

The Gemini integration should be enabled by default, run `agent: new gemini cli thread`.

If you don't yet have Gemini CLI installed, then Zed will install a version for you. If you do, then we will use the version of Gemini on your path.

If it is too old, you will see an error message, and you will have to upgrade Gemini. The instructions depend on how you originally installed it, but typically `npm install -g gemini-cli@preview` should work.

After you have Gemini running, you'll be prompted to choose your authentication method. Most users should click "Log in with Google", but if you have an API key already you can also click "Use Gemini API Key". The "Vertex AI" option is for those who are using Vertex AI, and have already configured their environment correctly.

For more information, see the [Gemini docs](TODO TODO)

## Usage

Similar to the built in agent, you can use Gemini CLI to do anything that you need. You can @-mention files, recent conversations, symbols, or fetch the web.

In the initial version, Gemini CLI does not support editing previous messages. We hope to add this ability soon.
