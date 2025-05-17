# Configuration

There are various aspects about the Agent Panel that you can customize.
All of them can be seen by either visiting [the Configuring Zed page](/configuring-zed.md#agent) or by running the `zed: open default settings` action and searching for `"agent"`.
Alternatively, you can also visit the panel's Settings view by running the `agent: open configuration` action or going to the top-right menu and hitting "Settings".

## LLM Providers

Zed supports multiple large language model providers.
Here's an overview of the supported providers and tool call support:

| Provider                                        | Tool Use Supported |
| ----------------------------------------------- | ------------------ |
| [Anthropic](#anthropic)                         | ✅                 |
| [GitHub Copilot Chat](#github-copilot-chat)     | In Some Cases      |
| [Google AI](#google-ai)                         | ✅                 |
| [Mistral](#mistral)                             | ✅                 |
| [Ollama](#ollama)                               | ✅                 |
| [OpenAI](#openai)                               | ✅                 |
| [DeepSeek](#deepseek)                           | 🚫                 |
| [OpenAI API Compatible](#openai-api-compatible) | 🚫                 |
| [LM Studio](#lmstudio)                          | 🚫                 |

## Use Your Own Keys {#use-your-own-keys}

While Zed offers hosted versions of models through [our various plans](/ai/plans-and-usage), we're always happy to support users wanting to supply their own API keys for LLM providers. Below, you can learn how to do that for each provider.

> Using your own API keys is _free_—you do not need to subscribe to a Zed plan to use our AI features with your own keys.

### Anthropic {#anthropic}

> ✅ Supports tool use

You can use Anthropic models by choosing it via the model dropdown in the Agent Panel.

1. Sign up for Anthropic and [create an API key](https://console.anthropic.com/settings/keys)
2. Make sure that your Anthropic account has credits
3. Open the settings view (`agent: open configuration`) and go to the Anthropic section
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

Custom models will be listed in the model dropdown in the Agent Panel.

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

> ✅ Supports tool use in some cases.
> Visit [the Copilot Chat code](https://github.com/zed-industries/zed/blob/9e0330ba7d848755c9734bf456c716bddf0973f3/crates/language_models/src/provider/copilot_chat.rs#L189-L198) for the supported subset.

You can use GitHub Copilot chat with the Zed assistant by choosing it via the model dropdown in the Agent Panel.

### Google AI {#google-ai}

> ✅ Supports tool use

You can use Gemini 1.5 Pro/Flash with the Zed assistant by choosing it via the model dropdown in the Agent Panel.

1. Go the Google AI Studio site and [create an API key](https://aistudio.google.com/app/apikey).
2. Open the settings view (`agent: open configuration`) and go to the Google AI section
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

Custom models will be listed in the model dropdown in the Agent Panel.

### Mistral {#mistral}

> 🔨Supports tool use

1. Visit the Mistral platform and [create an API key](https://console.mistral.ai/api-keys/)
2. Open the configuration view (`assistant: show configuration`) and navigate to the Mistral section
3. Enter your Mistral API key

The Mistral API key will be saved in your keychain.

Zed will also use the `MISTRAL_API_KEY` environment variable if it's defined.

#### Mistral Custom Models {#mistral-custom-models}

The Zed Assistant comes pre-configured with several Mistral models (codestral-latest, mistral-large-latest, mistral-medium-latest, mistral-small-latest, open-mistral-nemo, and open-codestral-mamba). All the default models support tool use. If you wish to use alternate models or customize their parameters, you can do so by adding the following to your Zed `settings.json`:

```json
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
          "supports_tools": true
        }
      ]
    }
  }
}
```

Custom models will be listed in the model dropdown in the assistant panel.

### Ollama {#ollama}

> ✅ Supports tool use

Download and install Ollama from [ollama.com/download](https://ollama.com/download) (Linux or macOS) and ensure it's running with `ollama --version`.

1. Download one of the [available models](https://ollama.com/models), for example, for `mistral`:

   ```sh
   ollama pull mistral
   ```

2. Make sure that the Ollama server is running. You can start it either via running Ollama.app (macOS) or launching:

   ```sh
   ollama serve
   ```

3. In the Agent Panel, select one of the Ollama models using the model dropdown.

#### Ollama Context Length {#ollama-context}

Zed has pre-configured maximum context lengths (`max_tokens`) to match the capabilities of common models.
Zed API requests to Ollama include this as `num_ctx` parameter, but the default values do not exceed `16384` so users with ~16GB of ram are able to use most models out of the box.
See [get_max_tokens in ollama.rs](https://github.com/zed-industries/zed/blob/main/crates/ollama/src/ollama.rs) for a complete set of defaults.

> **Note**: Tokens counts displayed in the Agent Panel are only estimates and will differ from the models native tokenizer.

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
          "supports_tools": true
        }
      ]
    }
  }
}
```

If you specify a context length that is too large for your hardware, Ollama will log an error. You can watch these logs by running: `tail -f ~/.ollama/logs/ollama.log` (MacOS) or `journalctl -u ollama -f` (Linux). Depending on the memory available on your machine, you may need to adjust the context length to a smaller value.

You may also optionally specify a value for `keep_alive` for each available model. This can be an integer (seconds) or alternately a string duration like "5m", "10m", "1h", "1d", etc., For example `"keep_alive": "120s"` will allow the remote server to unload the model (freeing up GPU VRAM) after 120seconds.

The `supports_tools` option controls whether or not the model will use additional tools.
If the model is tagged with `tools` in the Ollama catalog this option should be supplied, and built in profiles `Ask` and `Write` can be used.
If the model is not tagged with `tools` in the Ollama catalog, this
option can still be supplied with value `true`; however be aware that only the
`Minimal` built in profile will work.

### OpenAI {#openai}

> ✅ Supports tool use

1. Visit the OpenAI platform and [create an API key](https://platform.openai.com/account/api-keys)
2. Make sure that your OpenAI account has credits
3. Open the settings view (`agent: open configuration`) and go to the OpenAI section
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
      ],
      "version": "1"
    }
  }
}
```

You must provide the model's Context Window in the `max_tokens` parameter, this can be found [OpenAI Model Docs](https://platform.openai.com/docs/models). OpenAI `o1` models should set `max_completion_tokens` as well to avoid incurring high reasoning token costs. Custom models will be listed in the model dropdown in the Agent Panel.

### DeepSeek {#deepseek}

> 🚫 Does not support tool use

1. Visit the DeepSeek platform and [create an API key](https://platform.deepseek.com/api_keys)
2. Open the settings view (`agent: open configuration`) and go to the DeepSeek section
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

Custom models will be listed in the model dropdown in the Agent Panel. You can also modify the `api_url` to use a custom endpoint if needed.

### OpenAI API Compatible{#openai-api-compatible}

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

> 🚫 Does not support tool use

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

### Custom Provider Endpoints {#custom-provider-endpoint}

You can use a custom API endpoint for different providers, as long as it's compatible with the providers API structure.
To do so, add the following to your `settings.json`:

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

### Default Model {#default-model}

Zed's hosted LLM service sets `claude-3-7-sonnet-latest` as the default model.
However, you can change it either via the model dropdown in the Agent Panel's bottom-right corner or by manually editing the `default_model` object in your settings:

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

### Feature-specific Models {#feature-specific-models}

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

### Alternative Models for Inline Assists {#alternative-assists}

You can configure additional models that will be used to perform inline assists in parallel.
When you do this, the inline assist UI will surface controls to cycle between the alternatives generated by each model.

The models you specify here are always used in _addition_ to your [default model](#default-model).
For example, the following configuration will generate two outputs for every assist.
One with Claude 3.7 Sonnet, and one with GPT-4o.

```json
{
  "assistant": {
    "default_model": {
      "provider": "zed.dev",
      "model": "claude-3-7-sonnet"
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
