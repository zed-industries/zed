---
title: AI Code Editor Documentation - Zed
description: Docs for AI in Zed, the open-source AI code editor. Agentic coding, inline edits, AI code completion, and multi-model support.
---

# AI

Zed integrates AI throughout the editor: agentic coding, inline transformations, edit prediction, and direct model conversations.

## Setting Up AI

- [Configuration](./configuration.md): Connect to Anthropic, OpenAI, Ollama, Google AI, or other LLM providers.

- [External Agents](./external-agents.md): Run Claude Code, Codex, Aider, or other external agents inside Zed.

- [Subscription](./subscription.md): Zed's hosted models and billing.

- [Privacy and Security](./privacy-and-security.md): How Zed handles data when using AI features.

## Agentic Editing

- [Agent Panel](./agent-panel.md): Chat with AI agents that can read, write, and run code in your project.

- [Rules](./rules.md): Define specific instructions for AI behavior.

- [Tools](./tools.md): The built-in capabilities agents use: file operations, terminal commands, web search.

- [Tool Permissions](./tool-permissions.md): Configure granular permission rules for agent tool actions.

- [Model Context Protocol](./mcp.md): Extend agents with custom tools via MCP servers.

- [Inline Assistant](./inline-assistant.md): Transform selected code or terminal output with `ctrl-enter`.

## Edit Prediction

- [Edit Prediction](./edit-prediction.md): AI-powered autocomplete that predicts multi-line edits as you type.

## Text Threads

- [Text Threads](./text-threads.md): Lightweight conversations with models inside any buffer.
