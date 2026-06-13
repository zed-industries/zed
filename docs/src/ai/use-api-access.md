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
  - [Custom Anthropic Models](#anthropic-custom-models)
- [OpenAI API](#openai)
  - [Custom OpenAI Models](#openai-custom-models)
- [Google AI](#google-ai)
  - [Custom Google AI Models](#google-ai-custom-models)
- [Mistral](#mistral)
  - [Custom Mistral Models](#mistral-custom-models)
- [DeepSeek](#deepseek)
  - [Custom DeepSeek Models](#deepseek-custom-models)
- [xAI](#xai)
  - [Custom xAI Models](#xai-custom-models)
- [OpenCode API](#opencode)
  - [Custom OpenCode Models](#opencode-custom-models)
- [Anthropic-compatible endpoints](#anthropic-compatible)
- [OpenAI-compatible endpoints](#openai-compatible)

## What API Access Applies To {#support}

Use API access for the Zed Agent, Inline Assistant, Git commit generation,
thread summaries, and similar Zed-owned AI features.

External Agents and Terminal Threads usually configure model access in the
agent or CLI itself. See [Agents](./agents.md) for the difference between
agent paths and model access paths.

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

## Custom Headers {#custom-headers}

You can attach extra HTTP headers to every request Zed makes to supported HTTP-based providers. This is useful in corporate environments or for observability tooling.

Configure them with `language_models.<provider>.custom_headers`:

```json [settings]
{
  "language_models": {
    "openai": {
      "custom_headers": {
        "Fancy-Auth": "Bearer <your-fancy-key>",
        "X-My-Tag": "zed"
      }
    }
  }
}
```

`custom_headers` is supported by Amazon Bedrock, Anthropic, DeepSeek, Google AI, LM Studio, Mistral, Ollama, OpenAI, OpenAI-compatible providers, OpenCode, OpenRouter, Vercel AI Gateway, and xAI.

Headers managed by Zed for each provider, such as `Authorization`, `Content-Type`, `Accept`, and provider-specific authentication headers, are ignored with a warning if you try to override them.

## Remote Projects {#remote-projects}

Zed LLM providers for Zed AI features are initialized in the local Zed app. In SSH, dev container, and other remote projects, API keys saved in Zed are read from the local system keychain, and provider environment variables are read from the local Zed process environment.

External Agents and Terminal Threads may run their own processes and use their own remote or local environment. See [External Agents](./external-agents.md) and [Terminal Threads](./terminal-threads.md).

## Provider Notes {#provider-notes}

### Anthropic {#anthropic}

Use Anthropic API access when you have an Anthropic API key or API credits. Claude Pro and Max subscriptions are separate; see [Use an Existing Subscription](./use-an-existing-subscription.md#claude).

1. Sign up for Anthropic and [create an API key](https://console.anthropic.com/settings/keys).
2. Make sure your Anthropic account has API credits.
3. Open Agent Settings with {#action agent::OpenSettings} and go to the Anthropic section.
4. Enter your Anthropic API key.

Zed also reads `ANTHROPIC_API_KEY` from the local Zed process environment.

#### Custom Anthropic Models {#anthropic-custom-models}

Add custom Anthropic models in settings when you need an alternate model ID,
display name, context window, output limit, tool override, or thinking mode.

```json [settings]
{
  "language_models": {
    "anthropic": {
      "available_models": [
        {
          "name": "claude-3-5-sonnet-20240620",
          "display_name": "Sonnet 2024-June",
          "max_tokens": 128000,
          "max_output_tokens": 2560,
          "tool_override": "some-model-that-supports-toolcalling"
        }
      ]
    }
  }
}
```

For Anthropic models that support extended thinking, add a `mode` configuration:

```json [settings]
{
  "language_models": {
    "anthropic": {
      "available_models": [
        {
          "name": "claude-sonnet-4-latest",
          "display_name": "claude-sonnet-4-thinking",
          "max_tokens": 200000,
          "mode": {
            "type": "thinking",
            "budget_tokens": 4096
          }
        }
      ]
    }
  }
}
```

### OpenAI API {#openai}

Use OpenAI API access when you have an OpenAI API key or API billing. ChatGPT Plus and Pro subscriptions use a different setup path; see [Use an Existing Subscription](./use-an-existing-subscription.md#chatgpt).

1. Visit the OpenAI platform and [create an API key](https://platform.openai.com/account/api-keys).
2. Make sure your OpenAI account has credits or billing enabled.
3. Open Agent Settings with {#action agent::OpenSettings} and go to the OpenAI section.
4. Enter your OpenAI API key.

Zed also reads `OPENAI_API_KEY` from the local Zed process environment.

#### Custom OpenAI Models {#openai-custom-models}

Add custom OpenAI models in your settings file when you need alternate model IDs, preview releases, or custom request parameters.

```json [settings]
{
  "language_models": {
    "openai": {
      "available_models": [
        {
          "name": "gpt-5.2",
          "display_name": "gpt-5.2 high",
          "reasoning_effort": "high",
          "max_tokens": 272000,
          "max_completion_tokens": 20000
        }
      ]
    }
  }
}
```

You must provide the model's context window in `max_tokens`. For reasoning-focused models, set `max_completion_tokens` to avoid high reasoning-token costs.

### Google AI {#google-ai}

Use Google AI API access when you have a Gemini API key.

1. Go to Google AI Studio and [create an API key](https://aistudio.google.com/app/apikey).
2. Open Agent Settings with {#action agent::OpenSettings} and go to the Google AI section.
3. Enter your Google AI API key.

Zed reads `GEMINI_API_KEY`, falling back to `GOOGLE_AI_API_KEY`, from the local Zed process environment.

#### Custom Google AI Models {#google-ai-custom-models}

Add custom Google AI models when you need a specific Gemini model version,
including experimental models, or a thinking-mode configuration.

```json [settings]
{
  "language_models": {
    "google": {
      "available_models": [
        {
          "name": "gemini-3.1-pro-preview",
          "display_name": "Gemini 3.1 Pro",
          "max_tokens": 1000000,
          "mode": {
            "type": "thinking",
            "budget_tokens": 24000
          }
        },
        {
          "name": "gemini-3-flash-preview",
          "display_name": "Gemini 3 Flash (Thinking)",
          "max_tokens": 1000000,
          "mode": {
            "type": "thinking",
            "budget_tokens": 24000
          }
        }
      ]
    }
  }
}
```

### Mistral {#mistral}

Use Mistral API access when you have a Mistral API key.

1. Visit the Mistral platform and [create an API key](https://console.mistral.ai/api-keys/).
2. Open Agent Settings with {#action agent::OpenSettings} and go to the Mistral section.
3. Enter your Mistral API key.

Zed also reads `MISTRAL_API_KEY` from the local Zed process environment.

#### Custom Mistral Models {#mistral-custom-models}

Add custom Mistral models when you need alternate model IDs, custom limits, tool
support, image support, or a custom endpoint.

```json [settings]
{
  "language_models": {
    "mistral": {
      "api_url": "https://api.mistral.ai/v1",
      "available_models": [
        {
          "name": "mistral-tiny-latest",
          "display_name": "Mistral Tiny",
          "max_tokens": 32000,
          "max_output_tokens": 4096,
          "max_completion_tokens": 1024,
          "supports_tools": true,
          "supports_images": false
        }
      ]
    }
  }
}
```

### DeepSeek {#deepseek}

Use DeepSeek API access when you have paid API usage, top-ups, or an API key. In Zed, DeepSeek is API access, not subscription sign-in.

1. Visit the DeepSeek platform and [create an API key](https://platform.deepseek.com/api_keys).
2. Open Agent Settings with {#action agent::OpenSettings} and go to the DeepSeek section.
3. Enter your DeepSeek API key.

Zed also reads `DEEPSEEK_API_KEY` from the local Zed process environment.

#### Custom DeepSeek Models {#deepseek-custom-models}

Add custom DeepSeek models when you need alternate model IDs, custom token
limits, or a custom endpoint.

```json [settings]
{
  "language_models": {
    "deepseek": {
      "api_url": "https://api.deepseek.com/v1",
      "available_models": [
        {
          "name": "deepseek-v4-flash",
          "display_name": "DeepSeek V4 Flash",
          "max_tokens": 1000000,
          "max_output_tokens": 384000
        },
        {
          "name": "deepseek-v4-pro",
          "display_name": "DeepSeek V4 Pro",
          "max_tokens": 1000000,
          "max_output_tokens": 384000
        }
      ]
    }
  }
}
```

### xAI {#xai}

Use xAI API access when you have an xAI API key.

1. [Create an API key in the xAI Console](https://console.x.ai/team/default/api-keys).
2. Open Agent Settings with {#action agent::OpenSettings} and go to the xAI section.
3. Enter your xAI API key.

Zed also reads `XAI_API_KEY` from the local Zed process environment.

#### Custom xAI Models {#xai-custom-models}

Add custom xAI models when you need alternate Grok model IDs, custom limits,
image support, or a custom endpoint.

```json [settings]
{
  "language_models": {
    "x_ai": {
      "api_url": "https://api.x.ai/v1",
      "available_models": [
        {
          "name": "grok-1.5",
          "display_name": "Grok 1.5",
          "max_tokens": 131072,
          "max_output_tokens": 8192
        },
        {
          "name": "grok-1.5v",
          "display_name": "Grok 1.5V (Vision)",
          "max_tokens": 131072,
          "max_output_tokens": 8192,
          "supports_images": true
        }
      ]
    }
  }
}
```

### OpenCode API {#opencode}

Use OpenCode API access when you have an OpenCode API key. OpenCode Zen and Go affect which OpenCode models are available.

Zed does not sign in to OpenCode with OAuth or detect your OpenCode subscription; it uses an OpenCode API key saved in the system keychain or `OPENCODE_API_KEY`.

1. Visit [OpenCode Console](https://opencode.ai/auth) and create an account.
2. Free models are available without payment. To use Zen or Go models, make sure you have enough credits or an active subscription.
3. Generate an API key from the API Keys section in the OpenCode Console.
4. Open Agent Settings with {#action agent::OpenSettings} and go to the OpenCode section.
5. Enter your OpenCode API key.

Zed also reads `OPENCODE_API_KEY` from the local Zed process environment.

By default, models from all OpenCode subscription types are shown. You can hide subscriptions that are not relevant to you in the provider UI or in settings:

```json [settings]
{
  "language_models": {
    "opencode": {
      "show_zen_models": true,
      "show_go_models": false,
      "show_free_models": false
    }
  }
}
```

**Note:** Zed only bundles configuration for long-term OpenCode Free models. Free models that are available for a limited time are not included in Zed. To use those models, add a custom OpenCode model with configuration from [the OpenCode website](https://opencode.ai/docs/zen#pricing) and [models.dev](https://github.com/anomalyco/models.dev/tree/dev/providers/opencode/models).

#### Custom OpenCode Models {#opencode-custom-models}

The Zed Agent comes preconfigured with OpenCode models. Add custom OpenCode models when you need newer models, limited-time Free models, or models with custom endpoints.

Add custom models in your settings file:

```json [settings]
{
  "language_models": {
    "opencode": {
      "available_models": [
        {
          "name": "my-custom-model",
          "display_name": "My Custom Model",
          "max_tokens": 123456,
          "max_output_tokens": 98765,
          "protocol": "openai_chat",
          "reasoning_effort_levels": ["low", "medium", "high"],
          "interleaved_reasoning": false,
          "subscription": "go",
          "custom_model_api_url": "https://example.com/zen"
        }
      ]
    }
  }
}
```

The available configuration options for custom OpenCode models are:

- `name` (required): model ID used by OpenCode, such as `glm-9000`
- `display_name` (optional): human-readable model name shown in the UI, such as `Custom GLM 9000`
- `max_tokens` (required): maximum model context window size, such as `1000000`
- `max_output_tokens` (optional): maximum tokens the model can generate, such as `64000`
- `protocol` (required): model API protocol, one of `"anthropic"`, `"openai_responses"`, `"openai_chat"`, or `"google"`
- `reasoning_effort_levels` (optional): list of supported reasoning effort levels, such as `["low", "medium", "high"]`. The last value in the list is used as the default
- `interleaved_reasoning` (optional, default `false`): whether thinking tokens are sent as a dedicated `reasoning_content` field. Applies only when using the `openai_chat` protocol
- `subscription` (optional): `"zen"`, `"go"`, or `"free"`; defaults to `"zen"`
- `custom_model_api_url` (optional): custom API base URL to use instead of the default OpenCode API

Custom OpenCode models are listed in the model dropdown in the Agent Panel.

### Anthropic-Compatible Endpoints {#anthropic-compatible}

Use an Anthropic-compatible endpoint when a service implements Anthropic's [Messages API](https://docs.anthropic.com/en/api/messages) (`/v1/messages`) and gives you a custom base URL, model ID, and API key.

You can add a custom Anthropic-compatible provider from Agent Settings with {#action agent::OpenSettings}. Look for `Add Provider` in the LLM Providers section, choose `Anthropic`, and fill in the provider name, API URL, model ID, and context window.

You can also configure the provider in your settings file:

```json [settings]
{
  "language_models": {
    "anthropic_compatible": {
      "Some Provider": {
        "api_url": "https://api.someprovider.com",
        "custom_headers": {
          "X-Some-Header": "some-value"
        },
        "available_models": [
          {
            "name": "some-model",
            "display_name": "Some Model",
            "max_tokens": 200000,
            "max_output_tokens": 32000,
            "capabilities": {
              "tools": true,
              "images": false,
              "prompt_caching": false
            }
          }
        ]
      }
    }
  }
}
```

By default, Anthropic-compatible models inherit these capabilities:

- `tools`: `true`
- `images`: `false`
- `prompt_caching`: `false`

Enable `prompt_caching` to send explicit `cache_control` breakpoints for [prompt caching](https://docs.anthropic.com/en/docs/build-with-claude/prompt-caching); leave it disabled if the provider rejects requests containing them.

The optional `custom_headers` map adds extra headers to every request, which some providers require. Headers managed by Zed (such as `X-Api-Key` and `Anthropic-Version`) cannot be overridden.

Models also support the optional `default_temperature`, `extra_beta_headers` (sent as `anthropic-beta` headers), `mode`, and `tool_override` fields, which behave the same as in [Custom Anthropic Models](#anthropic-custom-models).

Enter the API key in the provider settings UI or set the generated environment variable (`<PROVIDER_NAME>_API_KEY`; in the example above, `SOME_PROVIDER_API_KEY`). Do not put API keys in `settings.json`.

### OpenAI-Compatible Endpoints {#openai-compatible}

Use an OpenAI-compatible endpoint when you have a custom base URL, model ID, and API key.

You can add a custom OpenAI-compatible provider from Agent Settings with {#action agent::OpenSettings}. Look for `Add Provider` in the LLM Providers section and fill in the provider name, API URL, model ID, and context window.

You can also configure the provider in your settings file:

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

By default, OpenAI-compatible models inherit these capabilities:

- `tools`: `true`
- `images`: `false`
- `parallel_tool_calls`: `false`
- `prompt_cache_key`: `false`
- `chat_completions`: `true`
- `interleaved_reasoning`: `false`

If a model only works with the Responses API, set `capabilities.chat_completions` to `false`. Zed will use the Responses endpoint for that model.

Enter the API key in the provider settings UI or set the generated environment variable. Do not put API keys in `settings.json`.
