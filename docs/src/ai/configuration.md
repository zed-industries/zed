# Configuring the Assistant

Here's a bird's-eye view of all the configuration options available in Zed's Assistant:

- Configure LLM Providers
  - [Zed AI (Configured by default when signed in)](#zed-ai)
  - [Anthropic](#anthropic)
  - [GitHub Copilot Chat](#github-copilot-chat)
  - [Google AI](#google-ai)
  - [Ollama](#ollama)
  - [OpenAI](#openai)
  - [DeepSeek](#deepseek)
  - [LM Studio](#lmstudio)
- Advanced configuration options
  - [Configuring Endpoints](#custom-endpoint)
  - [Configuring Timeouts](#provider-timeout)
  - [Configuring Models](#default-model)
  - [Configuring Feature-specific Models](#feature-specific-models)
  - [Configuring Alternative Models for Inline Assists](#alternative-assists)
- [Common Panel Settings](#common-panel-settings)
- [General Configuration Example](#general-example)

## Providers {#providers}

To access the Assistant configuration view, run `agent: open configuration` in the command palette, or click on the hamburger menu at the top-right of the Assistant Panel and select "Configure".

Below you can find all the supported providers available so far.

### Zed AI {#zed-ai}

A hosted service providing convenient and performant support for AI-enabled coding in Zed, powered by Anthropic's Claude 3.5 Sonnet and accessible just by signing in.

### Anthropic {#anthropic}

You can use Claude 3.5 Sonnet via [Zed AI](#zed-ai) for free. To use other Anthropic models you will need to configure it by providing your own API key.

1. Sign up for Anthropic and [create an API key](https://console.anthropic.com/settings/keys)
2. Make sure that your Anthropic account has credits
3. Open the configuration view (`agent: open configuration`) and navigate to the Anthropic section
4. Enter your Anthropic API key

Even if you pay for Claude Pro, you will still have to [pay for additional credits](https://console.anthropic.com/settings/plans) to use it via the API.

Zed will also use the `ANTHROPIC_API_KEY` environment variable if it's defined.

#### Anthropic Custom Models {#anthropic-custom-models}

You can add custom models to the Anthropic provider by adding the following to your Zed `settings.json`:

```json
{
  "language_models": {
    "anthropic": {
      "available_models": [
        {
          "name": "claude-3-5-sonnet-20240620",
          "display_name": "Sonnet 2024-June",
          "max_tokens": 128000,
          "max_output_tokens": 2560,
          "cache_configuration": {
            "max_cache_anchors": 10,
            "min_total_token": 10000,
            "should_speculate": false
          },
          "tool_override": "some-model-that-supports-toolcalling"
        }
      ]
    }
  }
}
```

Custom models will be listed in the model dropdown in the assistant panel.

You can configure a model to use [extended thinking](https://docs.anthropic.com/en/docs/about-claude/models/extended-thinking-models) (if it supports it),
by changing the mode in of your models configuration to `thinking`, for example:

```json
{
  "name": "claude-3-7-sonnet-latest",
  "display_name": "claude-3-7-sonnet-thinking",
  "max_tokens": 200000,
  "mode": {
    "type": "thinking",
    "budget_tokens": 4_096
  }
}
```

### GitHub Copilot Chat {#github-copilot-chat}

You can use GitHub Copilot chat with the Zed assistant by choosing it via the model dropdown in the assistant panel.

### Google AI {#google-ai}

You can use Gemini 1.5 Pro/Flash with the Zed assistant by choosing it via the model dropdown in the assistant panel.

1. Go the Google AI Studio site and [create an API key](https://aistudio.google.com/app/apikey).
2. Open the configuration view (`agent: open configuration`) and navigate to the Google AI section
3. Enter your Google AI API key and press enter.

The Google AI API key will be saved in your keychain.

Zed will also use the `GOOGLE_AI_API_KEY` environment variable if it's defined.

#### Google AI custom models {#google-ai-custom-models}

By default Zed will use `stable` versions of models, but you can use specific versions of models, including [experimental models](https://ai.google.dev/gemini-api/docs/models/experimental-models) with the Google AI provider by adding the following to your Zed `settings.json`:

```json
{
  "language_models": {
    "google": {
      "available_models": [
        {
          "name": "gemini-1.5-flash-latest",
          "display_name": "Gemini 1.5 Flash (Latest)",
          "max_tokens": 1000000
        }
      ]
    }
  }
}
```

Custom models will be listed in the model dropdown in the assistant panel.

### Ollama {#ollama}

Download and install Ollama from [ollama.com/download](https://ollama.com/download) (Linux or macOS) and ensure it's running with `ollama --version`.

1. Download one of the [available models](https://ollama.com/models), for example, for `mistral`:

   ```sh
   ollama pull mistral
   ```

2. Make sure that the Ollama server is running. You can start it either via running Ollama.app (MacOS) or launching:

   ```sh
   ollama serve
   ```

3. In the assistant panel, select one of the Ollama models using the model dropdown.

#### Ollama Context Length {#ollama-context}

Zed has pre-configured maximum context lengths (`max_tokens`) to match the capabilities of common models. Zed API requests to Ollama include this as `num_ctx` parameter, but the default values do not exceed `16384` so users with ~16GB of ram are able to use most models out of the box. See [get_max_tokens in ollama.rs](https://github.com/zed-industries/zed/blob/main/crates/ollama/src/ollama.rs) for a complete set of defaults.

**Note**: Tokens counts displayed in the assistant panel are only estimates and will differ from the models native tokenizer.

Depending on your hardware or use-case you may wish to limit or increase the context length for a specific model via settings.json:

```json
{
  "language_models": {
    "ollama": {
      "api_url": "http://localhost:11434",
      "available_models": [
        {
          "name": "qwen2.5-coder",
          "display_name": "qwen 2.5 coder 32K",
          "max_tokens": 32768
        }
      ]
    }
  }
}
```

If you specify a context length that is too large for your hardware, Ollama will log an error. You can watch these logs by running: `tail -f ~/.ollama/logs/ollama.log` (MacOS) or `journalctl -u ollama -f` (Linux). Depending on the memory available on your machine, you may need to adjust the context length to a smaller value.

You may also optionally specify a value for `keep_alive` for each available model. This can be an integer (seconds) or alternately a string duration like "5m", "10m", "1h", "1d", etc., For example `"keep_alive": "120s"` will allow the remote server to unload the model (freeing up GPU VRAM) after 120seconds.

### OpenAI {#openai}

1. Visit the OpenAI platform and [create an API key](https://platform.openai.com/account/api-keys)
2. Make sure that your OpenAI account has credits
3. Open the configuration view (`agent: open configuration`) and navigate to the OpenAI section
4. Enter your OpenAI API key

The OpenAI API key will be saved in your keychain.

Zed will also use the `OPENAI_API_KEY` environment variable if it's defined.

#### OpenAI Custom Models {#openai-custom-models}

The Zed Assistant comes pre-configured to use the latest version for common models (GPT-3.5 Turbo, GPT-4, GPT-4 Turbo, GPT-4o, GPT-4o mini). If you wish to use alternate models, perhaps a preview release or a dated model release or you wish to control the request parameters you can do so by adding the following to your Zed `settings.json`:

```json
{
  "language_models": {
    "openai": {
      "available_models": [
        {
          "name": "gpt-4o-2024-08-06",
          "display_name": "GPT 4o Summer 2024",
          "max_tokens": 128000
        },
        {
          "name": "o1-mini",
          "display_name": "o1-mini",
          "max_tokens": 128000,
          "max_completion_tokens": 20000
        }
      ]
      "version": "1"
    },
  }
}
```

You must provide the model's Context Window in the `max_tokens` parameter, this can be found [OpenAI Model Docs](https://platform.openai.com/docs/models). OpenAI `o1` models should set `max_completion_tokens` as well to avoid incurring high reasoning token costs. Custom models will be listed in the model dropdown in the assistant panel.

### DeepSeek {#deepseek}

1. Visit the DeepSeek platform and [create an API key](https://platform.deepseek.com/api_keys)
2. Open the configuration view (`agent: open configuration`) and navigate to the DeepSeek section
3. Enter your DeepSeek API key

The DeepSeek API key will be saved in your keychain.

Zed will also use the `DEEPSEEK_API_KEY` environment variable if it's defined.

#### DeepSeek Custom Models {#deepseek-custom-models}

The Zed Assistant comes pre-configured to use the latest version for common models (DeepSeek Chat, DeepSeek Reasoner). If you wish to use alternate models or customize the API endpoint, you can do so by adding the following to your Zed `settings.json`:

```json
{
  "language_models": {
    "deepseek": {
      "api_url": "https://api.deepseek.com",
      "available_models": [
        {
          "name": "deepseek-chat",
          "display_name": "DeepSeek Chat",
          "max_tokens": 64000
        },
        {
          "name": "deepseek-reasoner",
          "display_name": "DeepSeek Reasoner",
          "max_tokens": 64000,
          "max_output_tokens": 4096
        }
      ]
    }
  }
}
```

Custom models will be listed in the model dropdown in the assistant panel. You can also modify the `api_url` to use a custom endpoint if needed.

### OpenAI API Compatible

Zed supports using OpenAI compatible APIs by specifying a custom `endpoint` and `available_models` for the OpenAI provider.

#### X.ai Grok

Example configuration for using X.ai Grok with Zed:

```json
  "language_models": {
    "openai": {
      "api_url": "https://api.x.ai/v1",
      "available_models": [
        {
          "name": "grok-beta",
          "display_name": "X.ai Grok (Beta)",
          "max_tokens": 131072
        }
      ],
      "version": "1"
    },
  }
```

### LM Studio {#lmstudio}

1. Download and install the latest version of LM Studio from https://lmstudio.ai/download
2. In the app press ⌘/Ctrl + Shift + M and download at least one model, e.g. qwen2.5-coder-7b

   You can also get models via the LM Studio CLI:

   ```sh
   lms get qwen2.5-coder-7b
   ```

3. Make sure the LM Studio API server by running:

   ```sh
   lms server start
   ```

Tip: Set [LM Studio as a login item](https://lmstudio.ai/docs/advanced/headless#run-the-llm-service-on-machine-login) to automate running the LM Studio server.

## Advanced Configuration {#advanced-configuration}

### Custom Endpoints {#custom-endpoint}

You can use a custom API endpoint for different providers, as long as it's compatible with the providers API structure.

To do so, add the following to your Zed `settings.json`:

```json
{
  "language_models": {
    "some-provider": {
      "api_url": "http://localhost:11434"
    }
  }
}
```

Where `some-provider` can be any of the following values: `anthropic`, `google`, `ollama`, `openai`.

### Configuring Models {#default-model}

Zed's hosted LLM service sets `claude-3-7-sonnet-latest` as the default model.
However, you can change it either via the model dropdown in the Assistant Panel's bottom-left corner or by manually editing the `default_model` object in your settings:

```json
{
  "assistant": {
    "version": "2",
    "default_model": {
      "provider": "zed.dev",
      "model": "gpt-4o"
    }
  }
}
```

#### Feature-specific Models {#feature-specific-models}

> Currently only available in [Preview](https://zed.dev/releases/preview).

Zed allows you to configure different models for specific features.
This provides flexibility to use more powerful models for certain tasks while using faster or more efficient models for others.

If a feature-specific model is not set, it will fall back to using the default model, which is the one you set on the Agent Panel.

You can configure the following feature-specific models:

- Thread summary model: Used for generating thread summaries
- Inline assistant model: Used for the inline assistant feature
- Commit message model: Used for generating Git commit messages

Example configuration:

```json
{
  "assistant": {
    "version": "2",
    "default_model": {
      "provider": "zed.dev",
      "model": "claude-3-7-sonnet"
    },
    "inline_assistant_model": {
      "provider": "anthropic",
      "model": "claude-3-5-sonnet"
    },
    "commit_message_model": {
      "provider": "openai",
      "model": "gpt-4o-mini"
    },
    "thread_summary_model": {
      "provider": "google",
      "model": "gemini-2.0-flash"
    }
  }
}
```

### Configuring Alternative Models for Inline Assists {#alternative-assists}

You can configure additional models that will be used to perform inline assists in parallel. When you do this,
the inline assist UI will surface controls to cycle between the alternatives generated by each model. The models
you specify here are always used in _addition_ to your default model. For example, the following configuration
will generate two outputs for every assist. One with Claude 3.5 Sonnet, and one with GPT-4o.

```json
{
  "assistant": {
    "default_model": {
      "provider": "zed.dev",
      "model": "claude-3-5-sonnet"
    },
    "inline_alternatives": [
      {
        "provider": "zed.dev",
        "model": "gpt-4o"
      }
    ],
    "version": "2"
  }
}
```

## Common Panel Settings {#common-panel-settings}

| key            | type    | default | description                                                                           |
| -------------- | ------- | ------- | ------------------------------------------------------------------------------------- |
| enabled        | boolean | true    | Setting this to `false` will completely disable the assistant                         |
| button         | boolean | true    | Show the assistant icon in the status bar                                             |
| dock           | string  | "right" | The default dock position for the assistant panel. Can be ["left", "right", "bottom"] |
| default_height | string  | null    | The pixel height of the assistant panel when docked to the bottom                     |
| default_width  | string  | null    | The pixel width of the assistant panel when docked to the left or right               |

## General Configuration Example {#general-example}

```json
{
  "assistant": {
    "enabled": true,
    "default_model": {
      "provider": "zed.dev",
      "model": "claude-3-7-sonnet"
    },
    "editor_model": {
      "provider": "openai",
      "model": "gpt-4o"
    },
    "inline_assistant_model": {
      "provider": "anthropic",
      "model": "claude-3-5-sonnet"
    },
    "commit_message_model": {
      "provider": "openai",
      "model": "gpt-4o-mini"
    },
    "thread_summary_model": {
      "provider": "google",
      "model": "gemini-1.5-flash"
    },
    "version": "2",
    "button": true,
    "default_width": 480,
    "dock": "right"
  }
}
```
