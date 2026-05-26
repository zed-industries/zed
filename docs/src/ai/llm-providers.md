---
title: LLM Providers - Zed
description: Choose how Zed gets language models: Zed-hosted models, API access, subscriptions, gateways, or local models.
---

# LLM Providers

Use this page to choose which models power Zed-owned AI features, including [Zed Agent](./zed-agent.md), [Inline Assistant](./inline-assistant.md), Git commit generation, thread summaries, and similar model-backed features.

[External Agents](./external-agents.md) and [Terminal Threads](./terminal-threads.md) are different: Zed hosts the thread, but the External Agent or CLI usually owns its own model setup.

## Choose a Model Access Path {#choose-a-model-access-path}

| Model access path                                                 | Best when                                                             | Source of truth                       |
| ----------------------------------------------------------------- | --------------------------------------------------------------------- | ------------------------------------- |
| [Use Zed-Hosted Models](../account/zed-hosted-models.md)          | You want models billed through Zed                                    | Account & Billing > Zed-Hosted Models |
| [Use API Access](./use-api-access.md)                             | You have provider API access, credits, or usage billing               | Use API Access                        |
| [Use an Existing Subscription](./use-an-existing-subscription.md) | You already pay for ChatGPT, Claude, Copilot, or another subscription | Use an Existing Subscription          |
| [Use a Gateway](./use-a-gateway.md)                               | You route through OpenRouter, Bedrock, Vercel, or a similar platform  | Use a Gateway                         |
| [Use a Local Model](./use-a-local-model.md)                       | You run models locally or self-hosted                                 | Use a Local Model                     |

## Zed AI Features {#zed-ai-features}

Zed AI features are Zed-owned features that use a selected language model, including Zed Agent, Inline Assistant, Git commit generation, thread summaries, and similar model-backed features.

| Model access path     | Zed AI features | External Agents   | Terminal Threads |
| --------------------- | --------------- | ----------------- | ---------------- |
| Zed-hosted models     | Yes             | No                | No               |
| API access            | Yes             | Separate config   | Separate config  |
| Existing subscription | Some            | Often agent-owned | Often CLI-owned  |
| Gateway               | Yes             | Separate config   | Separate config  |
| Local model           | Yes             | Separate config   | Separate config  |

Use the child pages for provider-specific details and setup steps.

## Agent Path Boundaries {#agent-path-boundaries}

| Agent path                                | Model configuration                          |
| ----------------------------------------- | -------------------------------------------- |
| [Zed Agent](./zed-agent.md)               | Uses LLM providers configured in Zed         |
| [External Agents](./external-agents.md)   | Usually owned by the External Agent          |
| [Terminal Threads](./terminal-threads.md) | Owned by the CLI/TUI running in the terminal |

## Edit Prediction {#edit-prediction}

[Edit Prediction](./edit-prediction.md) has its own provider setup under `edit_predictions`. LLM providers on this page apply to model-backed Zed AI features such as Zed Agent, Inline Assistant, Git commit generation, and thread summaries.

## OpenAI-Compatible Providers {#openai-api-compatible}

OpenAI-compatible provider setup has moved to [Use API Access](./use-api-access.md#openai-compatible).
