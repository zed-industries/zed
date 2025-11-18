# LLM Providers

To use AI in Zed, you need to have at least one large language model provider set up.

You can do that by either subscribing to [one of Zed's plans](./plans-and-usage.md), or by using API keys you already have for the supported providers.

## Use Your Own Keys {#use-your-own-keys}

If you already have an API key for an existing LLM provider, like Anthropic or OpenAI, you can add them to Zed and use the full power of the Agent Panel **_for free_**.

To add an existing API key to a given provider, go to the Agent Panel settings (`agent: open settings`), look for the desired provider, paste the key into the input, and hit enter.

> Note: API keys are _not_ stored as plain text in your `settings.json`, but rather in your OS's secure credential storage.

## Supported Providers

Zed offers an extensive list of "use your own key" LLM providers

- [Amazon Bedrock](#amazon-bedrock)
- [Anthropic](#anthropic)
- [DeepSeek](#deepseek)
- [GitHub Copilot Chat](#github-copilot-chat)
- [Google AI](#google-ai)
- [LM Studio](#lmstudio)
- [Mistral](#mistral)
- [Ollama](#ollama)
- [OpenAI](#openai)
- [OpenAI API Compatible](#openai-api-compatible)
- [OpenRouter](#openrouter)
- [Vercel](#vercel-v0)
- [xAI](#xai)

### Amazon Bedrock {#amazon-bedrock}

> Supports tool use with models that support streaming tool use.
> More details can be found in the [Amazon Bedrock's Tool Use documentation](https://docs.aws.amazon.com/bedrock/latest/userguide/conversation-inference-supported-models-features.html).

To use Amazon Bedrock's models, an AWS authentication is required.
Ensure your credentials have the following permissions set up:

- `bedrock:InvokeModelWithResponseStream`
- `bedrock:InvokeModel`

Your IAM policy should look similar to:

```json [settings]
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "bedrock:InvokeModel",
        "bedrock:InvokeModelWithResponseStream"
      ],
      "Resource": "*"
    }
  ]
}
```

With that done, choose one of the two authentication methods:

#### Authentication via Named Profile (Recommended)

1. Ensure you have the AWS CLI installed and configured with a named profile
2. Open your `settings.json` (`zed: open settings file`) and include the `bedrock` key under `language_models` with the following settings:
   ```json [settings]
   {
     "language_models": {
       "bedrock": {
         "authentication_method": "named_profile",
         "region": "your-aws-region",
         "profile": "your-profile-name"
       }
     }
   }
   ```

#### Authentication via Static Credentials

While it's possible to configure through the Agent Panel settings UI by entering your AWS access key and secret directly, we recommend using named profiles instead for better security practices.
To do this:

1. Create an IAM User that you can assume in the [IAM Console](https://us-east-1.console.aws.amazon.com/iam/home?region=us-east-1#/users).
2. Create security credentials for that User, save them and keep them secure.
3. Open the Agent Configuration with (`agent: open settings`) and go to the Amazon Bedrock section
4. Copy the credentials from Step 2 into the respective **Access Key ID**, **Secret Access Key**, and **Region** fields.

#### Cross-Region Inference

The Zed implementation of Amazon Bedrock uses [Cross-Region inference](https://docs.aws.amazon.com/bedrock/latest/userguide/cross-region-inference.html) for all the models and region combinations that support it.
With Cross-Region inference, you can distribute traffic across multiple AWS Regions, enabling higher throughput.

For example, if you use `Claude Sonnet 3.7 Thinking` from `us-east-1`, it may be processed across the US regions, namely: `us-east-1`, `us-east-2`, or `us-west-2`.
Cross-Region inference requests are kept within the AWS Regions that are part of the geography where the data originally resides.
For example, a request made within the US is kept within the AWS Regions in the US.

Although the data remains stored only in the source Region, your input prompts and output results might move outside of your source Region during cross-Region inference.
All data will be transmitted encrypted across Amazon's secure network.

We will support Cross-Region inference for each of the models on a best-effort basis, please refer to the [Cross-Region Inference method Code](https://github.com/zed-industries/zed/blob/main/crates/bedrock/src/models.rs#L297).

For the most up-to-date supported regions and models, refer to the [Supported Models and Regions for Cross Region inference](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html).

### Anthropic {#anthropic}

You can use Anthropic models by choosing them via the model dropdown in the Agent Panel.

1. Sign up for Anthropic and [create an API key](https://console.anthropic.com/settings/keys)
2. Make sure that your Anthropic account has credits
3. Open the settings view (`agent: open settings`) and go to the Anthropic section
4. Enter your Anthropic API key

Even if you pay for Claude Pro, you will still have to [pay for additional credits](https://console.anthropic.com/settings/plans) to use it via the API.

Zed will also use the `ANTHROPIC_API_KEY` environment variable if it's defined.

#### Custom Models {#anthropic-custom-models}

You can add custom models to the Anthropic provider by adding the following to your Zed `settings.json`:

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

You can configure a model to use [extended thinking](https://docs.anthropic.com/en/docs/about-claude/models/extended-thinking-models) (if it supports it) by changing the mode in your model's configuration to `thinking`, for example:

```json [settings]
{
  "name": "claude-sonnet-4-latest",
  "display_name": "claude-sonnet-4-thinking",
  "max_tokens": 200000,
  "mode": {
    "type": "thinking",
    "budget_tokens": 4096
  }
}
```

### DeepSeek {#deepseek}

1. Visit the DeepSeek platform and [create an API key](https://platform.deepseek.com/api_keys)
2. Open the settings view (`agent: open settings`) and go to the DeepSeek section
3. Enter your DeepSeek API key

The DeepSeek API key will be saved in your keychain.

Zed will also use the `DEEPSEEK_API_KEY` environment variable if it's defined.

#### Custom Models {#deepseek-custom-models}

The Zed agent comes pre-configured to use the latest version for common models (DeepSeek Chat, DeepSeek Reasoner).
If you wish to use alternate models or customize the API endpoint, you can do so by adding the following to your Zed `settings.json`:

```json [settings]
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

Custom models will be listed in the model dropdown in the Agent Panel.
You can also modify the `api_url` to use a custom endpoint if needed.

### GitHub Copilot Chat {#github-copilot-chat}

You can use GitHub Copilot Chat with the Zed agent by choosing it via the model dropdown in the Agent Panel.

1. Open the settings view (`agent: open settings`) and go to the GitHub Copilot Chat section
2. Click on `Sign in to use GitHub Copilot`, follow the steps shown in the modal.

Alternatively, you can provide an OAuth token via the `GH_COPILOT_TOKEN` environment variable.

> **Note**: If you don't see specific models in the dropdown, you may need to enable them in your [GitHub Copilot settings](https://github.com/settings/copilot/features).

To use Copilot Enterprise with Zed (for both agent and completions), you must configure your enterprise endpoint as described in [Configuring GitHub Copilot Enterprise](./edit-prediction.md#github-copilot-enterprise).

### Google AI {#google-ai}

You can use Gemini models with the Zed agent by choosing it via the model dropdown in the Agent Panel.

1. Go to the Google AI Studio site and [create an API key](https://aistudio.google.com/app/apikey).
2. Open the settings view (`agent: open settings`) and go to the Google AI section
3. Enter your Google AI API key and press enter.

The Google AI API key will be saved in your keychain.

Zed will also use the `GEMINI_API_KEY` environment variable if it's defined. See [Using Gemini API keys](https://ai.google.dev/gemini-api/docs/api-key) in the Gemini docs for more.

#### Custom Models {#google-ai-custom-models}

By default, Zed will use `stable` versions of models, but you can use specific versions of models, including [experimental models](https://ai.google.dev/gemini-api/docs/models/experimental-models). You can configure a model to use [thinking mode](https://ai.google.dev/gemini-api/docs/thinking) (if it supports it) by adding a `mode` configuration to your model. This is useful for controlling reasoning token usage and response speed. If not specified, Gemini will automatically choose the thinking budget.

Here is an example of a custom Google AI model you could add to your Zed `settings.json`:

```json [settings]
{
  "language_models": {
    "google": {
      "available_models": [
        {
          "name": "gemini-2.5-flash-preview-05-20",
          "display_name": "Gemini 2.5 Flash (Thinking)",
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

Custom models will be listed in the model dropdown in the Agent Panel.

### LM Studio {#lmstudio}

1. Download and install [the latest version of LM Studio](https://lmstudio.ai/download)
2. In the app press `cmd/ctrl-shift-m` and download at least one model (e.g., qwen2.5-coder-7b). Alternatively, you can get models via the LM Studio CLI:

   ```sh
   lms get qwen2.5-coder-7b
   ```

3. Make sure the LM Studio API server is running by executing:

   ```sh
   lms server start
   ```

Tip: Set [LM Studio as a login item](https://lmstudio.ai/docs/advanced/headless#run-the-llm-service-on-machine-login) to automate running the LM Studio server.

### Mistral {#mistral}

1. Visit the Mistral platform and [create an API key](https://console.mistral.ai/api-keys/)
2. Open the configuration view (`agent: open settings`) and navigate to the Mistral section
3. Enter your Mistral API key

The Mistral API key will be saved in your keychain.

Zed will also use the `MISTRAL_API_KEY` environment variable if it's defined.

#### Custom Models {#mistral-custom-models}

The Zed agent comes pre-configured with several Mistral models (codestral-latest, mistral-large-latest, mistral-medium-latest, mistral-small-latest, open-mistral-nemo, and open-codestral-mamba).
All the default models support tool use.
If you wish to use alternate models or customize their parameters, you can do so by adding the following to your Zed `settings.json`:

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

Custom models will be listed in the model dropdown in the Agent Panel.

### Ollama {#ollama}

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
Zed API requests to Ollama include this as the `num_ctx` parameter, but the default values do not exceed `16384` so users with ~16GB of RAM are able to use most models out of the box.

See [get_max_tokens in ollama.rs](https://github.com/zed-industries/zed/blob/main/crates/ollama/src/ollama.rs) for a complete set of defaults.

> **Note**: Token counts displayed in the Agent Panel are only estimates and will differ from the model's native tokenizer.

Depending on your hardware or use-case you may wish to limit or increase the context length for a specific model via settings.json:

```json [settings]
{
  "language_models": {
    "ollama": {
      "api_url": "http://localhost:11434",
      "available_models": [
        {
          "name": "qwen2.5-coder",
          "display_name": "qwen 2.5 coder 32K",
          "max_tokens": 32768,
          "supports_tools": true,
          "supports_thinking": true,
          "supports_images": true
        }
      ]
    }
  }
}
```

If you specify a context length that is too large for your hardware, Ollama will log an error.
You can watch these logs by running: `tail -f ~/.ollama/logs/ollama.log` (macOS) or `journalctl -u ollama -f` (Linux).
Depending on the memory available on your machine, you may need to adjust the context length to a smaller value.

You may also optionally specify a value for `keep_alive` for each available model.
This can be an integer (seconds) or alternatively a string duration like "5m", "10m", "1h", "1d", etc.
For example, `"keep_alive": "120s"` will allow the remote server to unload the model (freeing up GPU VRAM) after 120 seconds.

The `supports_tools` option controls whether the model will use additional tools.
If the model is tagged with `tools` in the Ollama catalog, this option should be supplied, and the built-in profiles `Ask` and `Write` can be used.
If the model is not tagged with `tools` in the Ollama catalog, this option can still be supplied with the value `true`; however, be aware that only the `Minimal` built-in profile will work.

The `supports_thinking` option controls whether the model will perform an explicit "thinking" (reasoning) pass before producing its final answer.
If the model is tagged with `thinking` in the Ollama catalog, set this option and you can use it in Zed.

The `supports_images` option enables the model's vision capabilities, allowing it to process images included in the conversation context.
If the model is tagged with `vision` in the Ollama catalog, set this option and you can use it in Zed.

#### Ollama Authentication

In addition to running Ollama on your own hardware, which generally does not require authentication, Zed also supports connecting to remote Ollama instances. API keys are required for authentication.

One such service is [Ollama Turbo])(https://ollama.com/turbo). To configure Zed to use Ollama turbo:

1. Sign in to your Ollama account and subscribe to Ollama Turbo
2. Visit [ollama.com/settings/keys](https://ollama.com/settings/keys) and create an API key
3. Open the settings view (`agent: open settings`) and go to the Ollama section
4. Paste your API key and press enter.
5. For the API URL enter `https://ollama.com`

Zed will also use the `OLLAMA_API_KEY` environment variables if defined.

### OpenAI {#openai}

1. Visit the OpenAI platform and [create an API key](https://platform.openai.com/account/api-keys)
2. Make sure that your OpenAI account has credits
3. Open the settings view (`agent: open settings`) and go to the OpenAI section
4. Enter your OpenAI API key

The OpenAI API key will be saved in your keychain.

Zed will also use the `OPENAI_API_KEY` environment variable if it's defined.

#### Custom Models {#openai-custom-models}

The Zed agent comes pre-configured to use the latest version for common models (GPT-5, GPT-5 mini, o4-mini, GPT-4.1, and others).
To use alternate models, perhaps a preview release, or if you wish to control the request parameters, you can do so by adding the following to your Zed `settings.json`:

```json [settings]
{
  "language_models": {
    "openai": {
      "available_models": [
        {
          "name": "gpt-5",
          "display_name": "gpt-5 high",
          "reasoning_effort": "high",
          "max_tokens": 272000,
          "max_completion_tokens": 20000
        },
        {
          "name": "gpt-4o-2024-08-06",
          "display_name": "GPT 4o Summer 2024",
          "max_tokens": 128000
        }
      ]
    }
  }
}
```

You must provide the model's context window in the `max_tokens` parameter; this can be found in the [OpenAI model documentation](https://platform.openai.com/docs/models).

OpenAI `o1` models should set `max_completion_tokens` as well to avoid incurring high reasoning token costs.
Custom models will be listed in the model dropdown in the Agent Panel.

### OpenAI API Compatible {#openai-api-compatible}

Zed supports using [OpenAI compatible APIs](https://platform.openai.com/docs/api-reference/chat) by specifying a custom `api_url` and `available_models` for the OpenAI provider.
This is useful for connecting to other hosted services (like Together AI, Anyscale, etc.) or local models.

You can add a custom, OpenAI-compatible model either via the UI or by editing your `settings.json`.

To do it via the UI, go to the Agent Panel settings (`agent: open settings`) and look for the "Add Provider" button to the right of the "LLM Providers" section title.
Then, fill up the input fields available in the modal.

To do it via your `settings.json`, add the following snippet under `language_models`:

```json [settings]
{
  "language_models": {
    "openai_compatible": {
      // Using Together AI as an example
      "Together AI": {
        "api_url": "https://api.together.xyz/v1",
        "available_models": [
          {
            "name": "mistralai/Mixtral-8x7B-Instruct-v0.1",
            "display_name": "Together Mixtral 8x7B",
            "max_tokens": 32768,
            "capabilities": {
              "tools": true,
              "images": false,
              "parallel_tool_calls": false,
              "prompt_cache_key": false
            }
          }
        ]
      }
    }
  }
}
```

By default, OpenAI-compatible models inherit the following capabilities:

- `tools`: true (supports tool/function calling)
- `images`: false (does not support image inputs)
- `parallel_tool_calls`: false (does not support `parallel_tool_calls` parameter)
- `prompt_cache_key`: false (does not support `prompt_cache_key` parameter)

Note that LLM API keys aren't stored in your settings file.
So, ensure you have it set in your environment variables (`<PROVIDER_NAME>_API_KEY=<your api key>`) so your settings can pick it up. In the example above, it would be `TOGETHER_AI_API_KEY=<your api key>`.

### OpenRouter {#openrouter}

OpenRouter provides access to multiple AI models through a single API. It supports tool use for compatible models.

1. Visit [OpenRouter](https://openrouter.ai) and create an account
2. Generate an API key from your [OpenRouter keys page](https://openrouter.ai/keys)
3. Open the settings view (`agent: open settings`) and go to the OpenRouter section
4. Enter your OpenRouter API key

The OpenRouter API key will be saved in your keychain.

Zed will also use the `OPENROUTER_API_KEY` environment variable if it's defined.

#### Custom Models {#openrouter-custom-models}

You can add custom models to the OpenRouter provider by adding the following to your Zed `settings.json`:

```json [settings]
{
  "language_models": {
    "open_router": {
      "api_url": "https://openrouter.ai/api/v1",
      "available_models": [
        {
          "name": "google/gemini-2.0-flash-thinking-exp",
          "display_name": "Gemini 2.0 Flash (Thinking)",
          "max_tokens": 200000,
          "max_output_tokens": 8192,
          "supports_tools": true,
          "supports_images": true,
          "mode": {
            "type": "thinking",
            "budget_tokens": 8000
          }
        }
      ]
    }
  }
}
```

The available configuration options for each model are:

- `name` (required): The model identifier used by OpenRouter
- `display_name` (optional): A human-readable name shown in the UI
- `max_tokens` (required): The model's context window size
- `max_output_tokens` (optional): Maximum tokens the model can generate
- `max_completion_tokens` (optional): Maximum completion tokens
- `supports_tools` (optional): Whether the model supports tool/function calling
- `supports_images` (optional): Whether the model supports image inputs
- `mode` (optional): Special mode configuration for thinking models

You can find available models and their specifications on the [OpenRouter models page](https://openrouter.ai/models).

Custom models will be listed in the model dropdown in the Agent Panel.

#### Provider Routing

You can optionally control how OpenRouter routes a given custom model request among underlying upstream providers via the `provider` object on each model entry.

Supported fields (all optional):

- `order`: Array of provider slugs to try first, in order (e.g. `["anthropic", "openai"]`)
- `allow_fallbacks` (default: `true`): Whether fallback providers may be used if preferred ones are unavailable
- `require_parameters` (default: `false`): Only use providers that support every parameter you supplied
- `data_collection` (default: `allow`): `"allow"` or `"disallow"` (controls use of providers that may store data)
- `only`: Whitelist of provider slugs allowed for this request
- `ignore`: Provider slugs to skip
- `quantizations`: Restrict to specific quantization variants (e.g. `["int4","int8"]`)
- `sort`: Sort strategy for candidate providers (e.g. `"price"` or `"throughput"`)

Example adding routing preferences to a model:

```json [settings]
{
  "language_models": {
    "open_router": {
      "api_url": "https://openrouter.ai/api/v1",
      "available_models": [
        {
          "name": "openrouter/auto",
          "display_name": "Auto Router (Tools Preferred)",
          "max_tokens": 2000000,
          "supports_tools": true,
          "provider": {
            "order": ["anthropic", "openai"],
            "allow_fallbacks": true,
            "require_parameters": true,
            "only": ["anthropic", "openai", "google"],
            "ignore": ["cohere"],
            "quantizations": ["int8"],
            "sort": "price",
            "data_collection": "allow"
          }
        }
      ]
    }
  }
}
```

These routing controls let you fine‑tune cost, capability, and reliability trade‑offs without changing the model name you select in the UI.

### Vercel v0 {#vercel-v0}

[Vercel v0](https://v0.app/docs/api/model) is an expert model for generating full-stack apps, with framework-aware completions optimized for modern stacks like Next.js and Vercel.
It supports text and image inputs and provides fast streaming responses.

The v0 models are [OpenAI-compatible models](/#openai-api-compatible), but Vercel is listed as first-class provider in the panel's settings view.

To start using it with Zed, ensure you have first created a [v0 API key](https://v0.dev/chat/settings/keys).
Once you have it, paste it directly into the Vercel provider section in the panel's settings view.

You should then find it as `v0-1.5-md` in the model dropdown in the Agent Panel.

### xAI {#xai}

Zed has first-class support for [xAI](https://x.ai/) models. You can use your own API key to access Grok models.

1. [Create an API key in the xAI Console](https://console.x.ai/team/default/api-keys)
2. Open the settings view (`agent: open settings`) and go to the **xAI** section
3. Enter your xAI API key

The xAI API key will be saved in your keychain. Zed will also use the `XAI_API_KEY` environment variable if it's defined.

> **Note:** While the xAI API is OpenAI-compatible, Zed has first-class support for it as a dedicated provider. For the best experience, we recommend using the dedicated `x_ai` provider configuration instead of the [OpenAI API Compatible](#openai-api-compatible) method.

#### Custom Models {#xai-custom-models}

The Zed agent comes pre-configured with common Grok models. If you wish to use alternate models or customize their parameters, you can do so by adding the following to your Zed `settings.json`:

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

## Custom Provider Endpoints {#custom-provider-endpoint}

You can use a custom API endpoint for different providers, as long as it's compatible with the provider's API structure.
To do so, add the following to your `settings.json`:

```json [settings]
{
  "language_models": {
    "some-provider": {
      "api_url": "http://localhost:11434"
    }
  }
}
```

Currently, `some-provider` can be any of the following values: `anthropic`, `google`, `ollama`, `openai`.

This is the same infrastructure that powers models that are, for example, [OpenAI-compatible](#openai-api-compatible).
