---
title: AI Code Editor Documentation - Zed
description: Docs for AI in Zed, the open-source AI code editor. Agentic coding, inline edits, AI code completion, and multi-model support.
---

# AI

Zed is an open-source AI code editor. AI runs throughout the editing experience: agents that read and write your code, inline transformations, code completions on every keystroke, and conversations with models in any buffer.

## How Zed approaches AI

Zed's AI features run inside a native, GPU-accelerated application built in Rust. There is no Electron layer between you and the model output.

- **Open source.** The editor and all AI features are [open source](https://github.com/zed-industries/zed). You can read how AI is implemented, how data flows to providers, and how tool calls execute.
- **Multi-model.** Use Zed's hosted models or [bring your own API keys](./llm-providers.md) from Anthropic, OpenAI, Google, Ollama, and 8+ other providers. Run local models, connect to cloud APIs, or mix both. Switch models per task.
- **External agents.** Run Claude Code, Gemini CLI, Codex, and other CLI-based agents directly in Zed through the [Agent Client Protocol](https://zed.dev/acp). See [External Agents](./external-agents.md).
- **Privacy by default.** AI data sharing is opt-in. When you use your own API keys, Zed maintains zero-data retention agreements with providers. See [Privacy and Security](./privacy-and-security.md).

## Agentic editing

The [Agent Panel](./agent-panel.md) is where you work with AI agents. Agents can read files, edit code, run terminal commands, search the web, and access diagnostics through [built-in tools](./tools.md).

You can extend agents with additional tools through [MCP servers](./mcp.md), control what they can access with [tool permissions](./tool-permissions.md), and shape their behavior with [rules](./rules.md).

The [Inline Assistant](./inline-assistant.md) works differently: select code or a terminal command, describe what you want, and the model rewrites the selection in place. It works with multiple cursors.

## Code completions

[Edit Prediction](./edit-prediction.md) provides AI code completions on every keystroke. Each keypress sends a request to the prediction provider, which returns single or multi-line suggestions you accept with `tab`.

The default provider is Zeta, Zed's open-source model trained on open data. You can also use GitHub Copilot, Supermaven, or Codestral.

## Text threads

[Text Threads](./text-threads.md) are conversations with models inside any buffer. They work like a regular editor with your keybindings, multiple cursors, and standard editing features. Content is organized into message blocks with roles (You, Assistant, System).

## Getting started

- [Configuration](./configuration.md): Connect to Anthropic, OpenAI, Ollama, Google AI, or other LLM providers.
- [External Agents](./external-agents.md): Run Claude Code, Codex, Aider, or other external agents inside Zed.
- [Subscription](./subscription.md): Zed's hosted models and billing.
- [Privacy and Security](./privacy-and-security.md): How Zed handles data when using AI features.

New to Zed? Start with [Getting Started](../getting-started.md), then come back here to set up AI. For a higher-level overview, see [zed.dev/ai](https://zed.dev/ai).
