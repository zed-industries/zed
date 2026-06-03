---
title: Use an Existing Subscription - Zed
description: Use ChatGPT, Claude, Copilot, OpenCode, Cursor, and other existing AI subscriptions in Zed.
---

# Use an Existing Subscription

Use this page when you already pay for an AI product and want to know how it fits into Zed.

Some subscriptions work as Zed model providers. Others are used through an External Agent or terminal CLI.

| Subscription                  | Zed AI features                                      | External Agent via ACP                | Terminal Thread                | Notes                                                            |
| ----------------------------- | ---------------------------------------------------- | ------------------------------------- | ------------------------------ | ---------------------------------------------------------------- |
| Zed Pro, Business, or Student | [Zed-hosted models](../account/zed-hosted-models.md) | No                                    | No                             | Billed through Zed                                               |
| ChatGPT Plus / Pro            | ChatGPT Subscription                                 | Codex where supported                 | Codex CLI                      | Sign in with OpenAI in Zed; separate from OpenAI API keys        |
| Claude Pro / Max              | No direct Zed LLM provider path                      | Claude Agent                          | Claude Code                    | Separate from Anthropic API keys                                 |
| GitHub Copilot                | GitHub Copilot Chat; Copilot edit prediction         | Copilot agent where available         | CLI where available            | Requires Copilot/Copilot Chat auth                               |
| OpenCode Zen / Go             | OpenCode provider                                    | OpenCode agent where available        | `opencode` CLI                 | Requires OpenCode API key; subscription affects available models |
| Cursor subscription           | No Zed LLM provider path                             | Cursor External Agent where available | Cursor CLI/TUI where available | Use agent/CLI paths instead of Zed LLM provider settings         |

## ChatGPT Plus / Pro {#chatgpt}

ChatGPT Plus and Pro can be used through Zed's ChatGPT Subscription provider. Sign in with OpenAI in Zed; no separate OpenAI API key is required.

OpenAI API access is separate. If you have OpenAI API credits or API billing, use [Use API Access](./use-api-access.md#openai).

## Claude Pro / Max {#claude}

Claude Pro and Max subscriptions are separate from Anthropic API credits. Use Claude Agent or Claude Code where supported if you want subscription-backed Claude behavior.

For Anthropic API access, use [Use API Access](./use-api-access.md#anthropic).

See [What Anthropic's New Claude Billing Means for Zed Users](https://zed.dev/blog/anthropic-subscription-changes) for more context.

## GitHub Copilot {#github-copilot}

GitHub Copilot can be used as a Copilot Chat model provider for Zed AI features where supported. Copilot can also be used for [Edit Prediction](./edit-prediction.md).

If you use a Copilot agent or CLI, that setup is owned by Copilot. See [External Agents](./external-agents.md) and [Terminal Threads](./terminal-threads.md).

## OpenCode Zen / Go {#opencode}

OpenCode is a first-class language model provider in Zed. If you think of Zen or Go as your OpenCode subscription, the Zed setup path is still [Use API Access](./use-api-access.md#opencode): enter an OpenCode API key, then choose which OpenCode models to show. Zed does not sign in to OpenCode with OAuth or detect your subscription directly.

## Cursor {#cursor}

Cursor subscriptions do not configure Zed's LLM provider settings. Use a Cursor External Agent or Cursor CLI/TUI where available.

## Subscriptions Used Through Agent Harnesses {#agent-harnesses}

Some harnesses, CLIs, and External Agents can authenticate to ChatGPT, Claude, Copilot, or other subscriptions through their own flows. In those cases, Zed hosts the External Agent or Terminal Thread, but the harness owns auth and model behavior.

Pi Coding Agent is an example: Pi is a harness, not the subscription. Configure provider auth in Pi.

## DeepSeek {#deepseek}

DeepSeek paid usage, top-ups, and API billing are API access in Zed, not subscription sign-in. Use [Use API Access](./use-api-access.md#deepseek).
