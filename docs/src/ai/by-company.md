---
title: AI by Company - Zed
description: Find the right Zed setup path for OpenAI, ChatGPT, Codex, Claude, Gemini, Copilot, Cursor, OpenCode, Pi, OpenRouter, Bedrock, local models, and other AI tools.
---

# AI by Company

Use this page when you know the company, subscription, provider, agent, or CLI you want to use in Zed.

For detailed setup, follow the links in the `Setup` column. This page answers routing questions; it does not replace the setup pages.

## Zed {#zed}

| Path                 | Support level  | What you get                      | Account / billing       | Setup                                                |
| -------------------- | -------------- | --------------------------------- | ----------------------- | ---------------------------------------------------- |
| Zed-hosted models    | Built into Zed | Hosted models for Zed AI features | Billed through Zed      | [Zed-Hosted Models](../account/zed-hosted-models.md) |
| Zeta edit prediction | Built into Zed | Edit predictions while you type   | Included by plan limits | [Edit Prediction](./edit-prediction.md)              |

## OpenAI / ChatGPT / Codex {#openai-chatgpt-codex}

| Path                 | Support level     | What you get                                          | Account / billing     | Setup                                                                     |
| -------------------- | ----------------- | ----------------------------------------------------- | --------------------- | ------------------------------------------------------------------------- |
| ChatGPT Subscription | Configured in Zed | Subscription-backed OpenAI models for Zed AI features | ChatGPT Plus or Pro   | [Use an Existing Subscription](./use-an-existing-subscription.md#chatgpt) |
| OpenAI API           | Configured in Zed | OpenAI models through API access                      | OpenAI API billing    | [Use API Access](./use-api-access.md#openai)                              |
| Codex via ACP        | Hosted in Zed     | Codex in an External Agent thread                     | Owned by Codex/OpenAI | [External Agents](./external-agents.md#codex-cli)                         |
| Codex CLI            | Run in terminal   | Native Codex CLI experience in a Terminal Thread      | Owned by Codex/OpenAI | [Terminal Threads](./terminal-threads.md)                                 |

## Anthropic / Claude / Claude Code {#anthropic-claude}

| Path                 | Support level     | What you get                                       | Account / billing                       | Setup                                                |
| -------------------- | ----------------- | -------------------------------------------------- | --------------------------------------- | ---------------------------------------------------- |
| Anthropic API        | Configured in Zed | Claude models through API access                   | Anthropic API billing                   | [Use API Access](./use-api-access.md#anthropic)      |
| Claude Agent via ACP | Hosted in Zed     | Claude in an External Agent thread                 | Owned by Claude/Anthropic               | [External Agents](./external-agents.md#claude-agent) |
| Claude Code CLI      | Run in terminal   | Native Claude Code experience in a Terminal Thread | Claude subscription or Claude Code auth | [Terminal Threads](./terminal-threads.md)            |

Claude Pro and Max subscriptions are separate from Anthropic API credits. If you want Claude subscription-limit behavior, use Claude Agent or Claude Code where supported. See [Use an Existing Subscription](./use-an-existing-subscription.md#claude).

## Google / Gemini / Gemini CLI {#google-gemini}

| Path          | Support level                    | What you get                                      | Account / billing     | Setup                                                                                         |
| ------------- | -------------------------------- | ------------------------------------------------- | --------------------- | --------------------------------------------------------------------------------------------- |
| Google AI API | Configured in Zed                | Gemini models through API access                  | Google AI API billing | [Use API Access](./use-api-access.md#google-ai)                                               |
| Gemini CLI    | Hosted in Zed or run in terminal | Gemini CLI as an External Agent or native CLI/TUI | Owned by Gemini CLI   | [External Agents](./external-agents.md#gemini-cli), [Terminal Threads](./terminal-threads.md) |

## GitHub / Copilot {#github-copilot}

| Path                    | Support level     | What you get                                         | Account / billing           | Setup                                                                            |
| ----------------------- | ----------------- | ---------------------------------------------------- | --------------------------- | -------------------------------------------------------------------------------- |
| GitHub Copilot Chat     | Configured in Zed | Copilot Chat models for Zed AI features              | GitHub Copilot/Copilot Chat | [Use an Existing Subscription](./use-an-existing-subscription.md#github-copilot) |
| Copilot edit prediction | Built into Zed    | Edit prediction provider option                      | GitHub Copilot              | [Edit Prediction](./edit-prediction.md)                                          |
| Copilot External Agent  | Hosted in Zed     | Copilot in an External Agent thread, where available | Owned by Copilot            | [External Agents](./external-agents.md#copilot)                                  |
| Copilot CLI             | Run in terminal   | Native CLI experience, where available               | Owned by Copilot            | [Terminal Threads](./terminal-threads.md)                                        |

## OpenCode / Zen / Go {#opencode}

| Path                    | Support level     | What you get                                          | Account / billing                                    | Setup                                            |
| ----------------------- | ----------------- | ----------------------------------------------------- | ---------------------------------------------------- | ------------------------------------------------ |
| OpenCode provider       | Configured in Zed | OpenCode models for Zed AI features                   | OpenCode API key; Zen or Go affects available models | [Use API Access](./use-api-access.md#opencode)   |
| OpenCode External Agent | Hosted in Zed     | OpenCode in an External Agent thread, where available | Owned by OpenCode                                    | [External Agents](./external-agents.md#opencode) |
| `opencode` CLI          | Run in terminal   | Native OpenCode CLI experience                        | Owned by OpenCode                                    | [Terminal Threads](./terminal-threads.md)        |

## Cursor {#cursor}

| Path                  | Support level   | What you get                                           | Account / billing           | Setup                                          |
| --------------------- | --------------- | ------------------------------------------------------ | --------------------------- | ---------------------------------------------- |
| Cursor External Agent | Hosted in Zed   | Cursor in an External Agent thread, where available    | Cursor account/subscription | [External Agents](./external-agents.md#cursor) |
| Cursor CLI/TUI        | Run in terminal | Native Cursor command-line experience, where available | Cursor account/subscription | [Terminal Threads](./terminal-threads.md)      |

Cursor subscriptions do not configure Zed's LLM provider settings. If you want to use a work Cursor subscription in Zed, use the Cursor External Agent or a Terminal Threads workflow where available.

## Pi Coding Agent {#pi}

| Path            | Support level   | What you get                                       | Account / billing | Setup                                      |
| --------------- | --------------- | -------------------------------------------------- | ----------------- | ------------------------------------------ |
| Pi Coding Agent | Hosted in Zed   | Pi in an External Agent thread, where available    | Owned by Pi       | [External Agents](./external-agents.md#pi) |
| Pi CLI/TUI      | Run in terminal | Native Pi command-line experience, where available | Owned by Pi       | [Terminal Threads](./terminal-threads.md)  |

Pi is an agent harness, not a Zed LLM subscription. Pi may support provider auth such as ChatGPT, Claude, or Copilot through its own setup flow.

## DeepSeek {#deepseek}

| Path         | Support level     | What you get                        | Account / billing                               | Setup                                          |
| ------------ | ----------------- | ----------------------------------- | ----------------------------------------------- | ---------------------------------------------- |
| DeepSeek API | Configured in Zed | DeepSeek models for Zed AI features | DeepSeek API credits, top-ups, or usage billing | [Use API Access](./use-api-access.md#deepseek) |

Paid DeepSeek usage is API access in Zed, not subscription sign-in.

## Gateways and Cloud Platforms {#gateways}

| Provider          | Support level     | What you get                         | Account / billing  | Setup                                                 |
| ----------------- | ----------------- | ------------------------------------ | ------------------ | ----------------------------------------------------- |
| OpenRouter        | Configured in Zed | Gateway access to multiple providers | OpenRouter billing | [Use a Gateway](./use-a-gateway.md#openrouter)        |
| Vercel AI Gateway | Configured in Zed | Gateway access through Vercel        | Vercel billing     | [Use a Gateway](./use-a-gateway.md#vercel-ai-gateway) |
| Amazon Bedrock    | Configured in Zed | AWS-hosted model access              | AWS billing        | [Use a Gateway](./use-a-gateway.md#amazon-bedrock)    |

## Local Models {#local-models}

| Tool                              | Support level     | What you get                           | Account / billing | Setup                                                         |
| --------------------------------- | ----------------- | -------------------------------------- | ----------------- | ------------------------------------------------------------- |
| Ollama                            | Configured in Zed | Local models for Zed AI features       | Local/self-hosted | [Use a Local Model](./use-a-local-model.md#ollama)            |
| LM Studio                         | Configured in Zed | Local models for Zed AI features       | Local/self-hosted | [Use a Local Model](./use-a-local-model.md#lm-studio)         |
| Local OpenAI-compatible server    | Configured in Zed | Local or self-hosted model endpoint    | Local/self-hosted | [Use a Local Model](./use-a-local-model.md#openai-compatible) |
| Local/self-hosted edit prediction | Configured in Zed | Edit predictions from a local provider | Local/self-hosted | [Edit Prediction](./edit-prediction.md)                       |

## Other API Providers {#other-api-providers}

For Mistral, xAI, and OpenAI-compatible endpoints that are not listed above, see [Use API Access](./use-api-access.md).
