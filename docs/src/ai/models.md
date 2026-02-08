---
title: AI Models and Pricing - Zed
description: AI models available via Zed Pro including Claude, GPT-5, Gemini, and Grok. Pricing, context windows, and tool call support.
---

# Models

Zed's plans offer hosted versions of major LLMs with higher rate limits than direct API access. Model availability is updated regularly.

| Model                  | Provider  | Token Type          | Provider Price per 1M tokens | Zed Price per 1M tokens |
| ---------------------- | --------- | ------------------- | ---------------------------- | ----------------------- |
| Claude Opus 4.5        | Anthropic | Input               | $5.00                        | $5.50                   |
|                        | Anthropic | Output              | $25.00                       | $27.50                  |
|                        | Anthropic | Input - Cache Write | $6.25                        | $6.875                  |
|                        | Anthropic | Input - Cache Read  | $0.50                        | $0.55                   |
| Claude Opus 4.1        | Anthropic | Input               | $15.00                       | $16.50                  |
|                        | Anthropic | Output              | $75.00                       | $82.50                  |
|                        | Anthropic | Input - Cache Write | $18.75                       | $20.625                 |
|                        | Anthropic | Input - Cache Read  | $1.50                        | $1.65                   |
| Claude Sonnet 4.5      | Anthropic | Input               | $3.00                        | $3.30                   |
|                        | Anthropic | Output              | $15.00                       | $16.50                  |
|                        | Anthropic | Input - Cache Write | $3.75                        | $4.125                  |
|                        | Anthropic | Input - Cache Read  | $0.30                        | $0.33                   |
| Claude Sonnet 4        | Anthropic | Input               | $3.00                        | $3.30                   |
|                        | Anthropic | Output              | $15.00                       | $16.50                  |
|                        | Anthropic | Input - Cache Write | $3.75                        | $4.125                  |
|                        | Anthropic | Input - Cache Read  | $0.30                        | $0.33                   |
| Claude Sonnet 3.7      | Anthropic | Input               | $3.00                        | $3.30                   |
|                        | Anthropic | Output              | $15.00                       | $16.50                  |
|                        | Anthropic | Input - Cache Write | $3.75                        | $4.125                  |
|                        | Anthropic | Input - Cache Read  | $0.30                        | $0.33                   |
| Claude Haiku 4.5       | Anthropic | Input               | $1.00                        | $1.10                   |
|                        | Anthropic | Output              | $5.00                        | $5.50                   |
|                        | Anthropic | Input - Cache Write | $1.25                        | $1.375                  |
|                        | Anthropic | Input - Cache Read  | $0.10                        | $0.11                   |
| GPT-5                  | OpenAI    | Input               | $1.25                        | $1.375                  |
|                        | OpenAI    | Output              | $10.00                       | $11.00                  |
|                        | OpenAI    | Cached Input        | $0.125                       | $0.1375                 |
| GPT-5 mini             | OpenAI    | Input               | $0.25                        | $0.275                  |
|                        | OpenAI    | Output              | $2.00                        | $2.20                   |
|                        | OpenAI    | Cached Input        | $0.025                       | $0.0275                 |
| GPT-5 nano             | OpenAI    | Input               | $0.05                        | $0.055                  |
|                        | OpenAI    | Output              | $0.40                        | $0.44                   |
|                        | OpenAI    | Cached Input        | $0.005                       | $0.0055                 |
| Gemini 3.0 Pro         | Google    | Input               | $2.00                        | $2.20                   |
|                        | Google    | Output              | $12.00                       | $13.20                  |
| Gemini 2.5 Pro         | Google    | Input               | $1.25                        | $1.375                  |
|                        | Google    | Output              | $10.00                       | $11.00                  |
| Gemini 2.5 Flash       | Google    | Input               | $0.30                        | $0.33                   |
|                        | Google    | Output              | $2.50                        | $2.75                   |
| Grok 4                 | X.ai      | Input               | $3.00                        | $3.30                   |
|                        | X.ai      | Output              | $15.00                       | $16.5                   |
|                        | X.ai      | Cached Input        | $0.75                        | $0.825                  |
| Grok 4 Fast            | X.ai      | Input               | $0.20                        | $0.22                   |
|                        | X.ai      | Output              | $0.50                        | $0.55                   |
|                        | X.ai      | Cached Input        | $0.05                        | $0.055                  |
| Grok 4 (Non-Reasoning) | X.ai      | Input               | $0.20                        | $0.22                   |
|                        | X.ai      | Output              | $0.50                        | $0.55                   |
|                        | X.ai      | Cached Input        | $0.05                        | $0.055                  |
| Grok Code Fast 1       | X.ai      | Input               | $0.20                        | $0.22                   |
|                        | X.ai      | Output              | $1.50                        | $1.65                   |
|                        | X.ai      | Cached Input        | $0.02                        | $0.022                  |

## Usage {#usage}

Any usage of a Zed-hosted model will be billed at the Zed Price (rightmost column above). See [Plans and Usage](./plans-and-usage.md) for details on Zed's plans and limits for use of hosted models.

> LLMs can enter unproductive loops that require user intervention. Monitor longer-running tasks and interrupt if needed.

## Context Windows {#context-windows}

A context window is the maximum span of text and code an LLM can consider at once, including both the input prompt and output generated by the model.

| Model             | Provider  | Zed-Hosted Context Window |
| ----------------- | --------- | ------------------------- |
| Claude Opus 4.5   | Anthropic | 200k                      |
| Claude Opus 4.1   | Anthropic | 200k                      |
| Claude Sonnet 4   | Anthropic | 200k                      |
| Claude Sonnet 3.7 | Anthropic | 200k                      |
| Claude Haiku 4.5  | Anthropic | 200k                      |
| GPT-5             | OpenAI    | 400k                      |
| GPT-5 mini        | OpenAI    | 400k                      |
| GPT-5 nano        | OpenAI    | 400k                      |
| Gemini 2.5 Pro    | Google    | 200k                      |
| Gemini 2.5 Flash  | Google    | 200k                      |
| Gemini 3.0 Pro    | Google    | 200k                      |

> Context window limits for hosted Sonnet 4 and Gemini 2.5 Pro/Flash may increase in future releases.

Each Agent thread and text thread in Zed maintains its own context window.
The more prompts, attached files, and responses included in a session, the larger the context window grows.

Start a new thread for each distinct task to keep context focused.

## Tool Calls {#tool-calls}

Models can use [tools](./tools.md) to interface with your code, search the web, and perform other useful functions.
