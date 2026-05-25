---
title: Use API Access - Zed
description: Configure provider API access, API keys, API credits, usage billing, and OpenAI-compatible endpoints for Zed AI features.
---

# Use API Access

Use API access when a provider gives you an API key, API credits, top-ups, or usage billing.

Paid API credits, usage billing, and top-ups are API access, even when you pay the provider directly. Use this path when the provider gives you an API key.

## Supported API Providers {#providers}

Zed supports these first-class API providers for model-backed Zed AI features:

- [Anthropic](#anthropic)
- [OpenAI API](#openai)
- [Google AI](#google-ai)
- [Mistral](#mistral)
- [DeepSeek](#deepseek)
- [xAI](#xai)
- [OpenCode API](#opencode)
- [OpenAI-compatible endpoints](#openai-compatible)

## What API Access Applies To {#support}

| Access path                       | Zed AI features | External agents | Terminal threads | Notes                                                                                                        |
| --------------------------------- | --------------- | --------------- | ---------------- | ------------------------------------------------------------------------------------------------------------ |
| Provider API key                  | Yes             | Separate config | Separate config  | Zed Agent, Inline Assistant, Git commit generation, and similar Zed-owned features can use configured models |
| API key used by an external agent | No              | Agent-owned     | Separate config  | Configure it in the external agent                                                                           |
| API key used by a CLI/TUI         | No              | Separate config | CLI-owned        | Configure it in the terminal environment or CLI config                                                       |

## API Keys and Environment Variables {#api-keys}

Most API-access providers can be configured in Zed's Agent Settings panel with {#action agent::OpenSettings}. Keys saved through Zed are stored in the system keychain, not in `settings.json`.

Zed also reads provider-specific environment variables. Non-empty environment variables take precedence over keychain values. If a key comes from an environment variable, unset the variable and restart Zed to stop using it.

| Provider          | Environment variable                                  |
| ----------------- | ----------------------------------------------------- |
| Anthropic         | `ANTHROPIC_API_KEY`                                   |
| OpenAI            | `OPENAI_API_KEY`                                      |
| Google AI         | `GEMINI_API_KEY`, falling back to `GOOGLE_AI_API_KEY` |
| Mistral           | `MISTRAL_API_KEY`                                     |
| DeepSeek          | `DEEPSEEK_API_KEY`                                    |
| xAI               | `XAI_API_KEY`                                         |
| OpenCode          | `OPENCODE_API_KEY`                                    |
| OpenRouter        | `OPENROUTER_API_KEY`                                  |
| Vercel AI Gateway | `VERCEL_AI_GATEWAY_API_KEY`                           |
| Ollama            | `OLLAMA_API_KEY`                                      |
| LM Studio         | `LMSTUDIO_API_KEY`                                    |

OpenAI-compatible provider environment variables are generated from the configured provider ID as upper snake case plus `_API_KEY`. For example, provider ID `my-gateway` uses `MY_GATEWAY_API_KEY`.

## Remote Projects {#remote-projects}

Zed LLM providers for Zed AI features are initialized in the local Zed app. In SSH, dev container, and other remote projects, API keys saved in Zed are read from the local system keychain, and provider environment variables are read from the local Zed process environment.

External agents and terminal threads may run their own processes and use their own remote or local environment. See [External Agents](./external-agents.md) and [Terminal Threads](./terminal-threads.md).

## Provider Notes {#provider-notes}

### Anthropic {#anthropic}

Use Anthropic API access when you have an Anthropic API key or API credits. Claude Pro and Max subscriptions are separate; see [Use an Existing Subscription](./use-an-existing-subscription.md#claude).

### OpenAI API {#openai}

Use OpenAI API access when you have an OpenAI API key or API billing. ChatGPT Plus and Pro subscriptions use a different setup path; see [Use an Existing Subscription](./use-an-existing-subscription.md#chatgpt).

### Google AI {#google-ai}

Use Google AI API access when you have a Gemini API key.

### Mistral {#mistral}

Use Mistral API access when you have a Mistral API key.

### DeepSeek {#deepseek}

Use DeepSeek API access when you have paid API usage, top-ups, or an API key. In Zed, DeepSeek is API access, not subscription sign-in.

### xAI {#xai}

Use xAI API access when you have an xAI API key.

### OpenCode API {#opencode}

Use OpenCode API access when you have an OpenCode API key. OpenCode Zen and Go affect which OpenCode models are available.

### OpenAI-Compatible Endpoints {#openai-compatible}

Use an OpenAI-compatible endpoint when you have a custom base URL, model ID, and API key.

```json [settings]
{
  "language_models": {
    "openai_compatible": {
      "my-provider": {
        "api_url": "https://example.com/v1",
        "available_models": [
          {
            "name": "my-model",
            "display_name": "My Model",
            "max_tokens": 128000
          }
        ]
      }
    }
  }
}
```

Enter the API key in the provider settings UI or set the generated environment variable. Do not put API keys in `settings.json`.
