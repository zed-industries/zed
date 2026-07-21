---
title: LLM Providers - Zed
description: Choose how Zed gets language models: Zed-hosted models, API access, subscriptions, gateways, or local models.
---

# LLM Providers

Use this page to choose which models power [the Zed Agent](./zed-agent.md) and
other Zed-owned AI features, including [Inline Assistant](./inline-assistant.md),
Git commit generation, thread summaries, and similar model-backed features.

Model access paths do not configure [External Agents](./external-agents.md) or
[Terminal Threads](./terminal-threads.md). External Agents and Terminal Threads
usually own their own model access, auth, and configuration.

## Choose a Model Access Path {#choose-a-model-access-path}

| Model access path                                                 | Best when                                                             | Source of truth                       |
| ----------------------------------------------------------------- | --------------------------------------------------------------------- | ------------------------------------- |
| [Use Zed-Hosted Models](../account/zed-hosted-models.md)          | You want models billed through Zed                                    | Account & Billing > Zed-Hosted Models |
| [Use API Access](./use-api-access.md)                             | You have provider API access, credits, or usage billing               | Use API Access                        |
| [Use an Existing Subscription](./use-an-existing-subscription.md) | You already pay for ChatGPT, Claude, Copilot, or another subscription | Use an Existing Subscription          |
| [Use a Gateway](./use-a-gateway.md)                               | You route through OpenRouter, Bedrock, Vercel, or a similar platform  | Use a Gateway                         |
| [Use a Local Model](./use-a-local-model.md)                       | You run models locally or self-hosted                                 | Use a Local Model                     |

Use the setup pages for provider-specific details. See [Agents](./agents.md) for
the difference between the Zed Agent, External Agents, and Terminal Threads.

## Edit Prediction {#edit-prediction}

[Edit Prediction](./edit-prediction.md) has its own provider setup under `edit_predictions`. LLM providers on this page apply to model-backed Zed AI features such as Zed Agent, Inline Assistant, Git commit generation, and thread summaries.

## Anthropic-Compatible Providers {#anthropic-api-compatible}

Anthropic-compatible provider setup has moved to [Use API Access](./use-api-access.md#anthropic-compatible).

## OpenAI-Compatible Providers {#openai-api-compatible}

OpenAI-compatible provider setup has moved to [Use API Access](./use-api-access.md#openai-compatible).
