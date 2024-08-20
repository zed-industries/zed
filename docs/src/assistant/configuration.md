# Configuring the Assistant

## Providers {#providers}

The following providers are supported:

- [Zed AI (Configured by default when signed in)](#zed-ai)
- [Anthropic](#anthropic)
- [GitHub Copilot Chat](#github-copilot-chat) [^1]
- [Google Gemini](#google-gemini) [^1]
- [Ollama](#ollama)
- [OpenAI](#openai)

To configure different providers, run `assistant: show configuration` in the command palette, or click on the hamburger menu at the top-right of the assistant panel and select "Configure".

[^1]: This provider does not support [`/workflow`](./commands#workflow-not-generally-available) command.

To further customize providers, you can use `settings.json` to do that as follows:

- [Configuring endpoints](#custom-endpoint)
- [Configuring timeouts](#provider-timeout)
- [Configuring default model](#default-model)

### Zed AI {#zed-ai}

A hosted service providing convenient and performant support for AI-enabled coding in Zed, powered by Anthropic's Claude 3.5 Sonnet and accessible just by signing in.

### Anthropic {#anthropic}

You can use Claude 3.5 Sonnet via [Zed AI](#zed-ai) for free. To use other Anthropic models you will need to configure it by providing your own API key.

1. You can obtain an API key [here](https://console.anthropic.com/settings/keys).
2. Make sure that your Anthropic account has credits
3. Open the configuration view (`assistant: show configuration`) and navigate to the Anthropic section
4. Enter your Anthropic API key

Even if you pay for Claude Pro, you will still have to [pay for additional credits](https://console.anthropic.com/settings/plans) to use it via the API.

#### Anthropic Custom Models {#anthropic-custom-models}

You can add custom models to the Anthropic provider, by adding the following to your Zed `settings.json`:

```json
{
  "language_models": {
    "anthropic": {
      "available_models": [
        {
          "name": "some-model",
          "display_name": "some-model",
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

### GitHub Copilot Chat {#github-copilot-chat}

You can use GitHub Copilot chat with the Zed assistant by choosing it via the model dropdown in the assistant panel.

### Google Gemini {#google-gemini}

You can use Gemini 1.5 Pro/Flash with the Zed assistant by choosing it via the model dropdown in the assistant panel.

You can obtain an API key [here](https://aistudio.google.com/app/apikey).

#### Google Gemini Custom Models {#google-custom-models}

You can add custom models to the OpenAI provider, by adding the following to your Zed `settings.json`:

```json
{
  "language_models": {
    "google": {
      "available_models": [
        {
          "name": "custom-model",
          "max_tokens": 128000
        }
      ]
    }
  }
}
```

Custom models will be listed in the model dropdown in the assistant panel.

### Ollama {#ollama}

Download and install Ollama from [ollama.com/download](https://ollama.com/download) (Linux or macOS) and ensure it's running with `ollama --version`.

You can use Ollama with the Zed assistant by making Ollama appear as an OpenAPI endpoint.

1. Download, for example, the `mistral` model with Ollama:

   ```sh
   ollama pull mistral
   ```

2. Make sure that the Ollama server is running. You can start it either via running the Ollama app, or launching:

   ```sh
   ollama serve
   ```

3. In the assistant panel, select one of the Ollama models using the model dropdown.
4. (Optional) If you want to change the default URL that is used to access the Ollama server, you can do so by adding the following settings:

```json
{
  "language_models": {
    "ollama": {
      "api_url": "http://localhost:11434"
    }
  }
}
```

### OpenAI {#openai}

1. Create an [OpenAI API key](https://platform.openai.com/account/api-keys)
2. Make sure that your OpenAI account has credits
3. Open the configuration view (`assistant: show configuration`) and navigate to the OpenAI section
4. Enter your OpenAI API key

The OpenAI API key will be saved in your keychain.

Zed will also use the `OPENAI_API_KEY` environment variable if it's defined.

#### OpenAI Custom Models {#openai-custom-models}

You can add custom models to the OpenAI provider, by adding the following to your Zed `settings.json`:

```json
{
  "language_models": {
    "openai": {
      "version": "1",
      "available_models": [
        {
          "name": "custom-model",
          "max_tokens": 128000
        }
      ]
    }
  }
}
```

Custom models will be listed in the model dropdown in the assistant panel.

### Advanced configuration {#advanced-configuration}

#### Example Configuration

```json
{
  "assistant": {
    "enabled": true,
    "default_model": {
      "provider": "zed.dev",
      "model": "claude-3-5-sonnet"
    },
    "version": "2",
    "button": true,
    "default_width": 480,
    "dock": "right"
  }
}
```

#### Custom endpoints {#custom-endpoint}

You can use a custom API endpoint for different providers, as long as it's compatible with the API structure.

To do so, add the following to your Zed `settings.json`:

```json
{
  "language_models": {
    "some-provider": {
      "api_url": "http://localhost:11434/v1"
    }
  }
}
```

Where `some-provider` can be any of the following values: `anthropic`, `google`, `ollama`, `openai`.

#### Custom timeout {#provider-timeout}

You can customize the timeout that's used for LLM requests, by adding the following to your Zed `settings.json`:

```json
{
  "language_models": {
    "some-provider": {
      "low_speed_timeout_in_seconds": 10
    }
  }
}
```

Where `some-provider` can be any of the following values: `anthropic`, `copilot_chat`, `google`, `ollama`, `openai`.

#### Configuring the default model {#default-model}

The default model can be changed by clicking on the model dropdown (top-right) in the assistant panel.
Picking a model will save it as the default model. You can still change the default model manually, by editing the `default_model` object in the settings. The `default_model` object can contain the following keys:

```json
{
  "assistant": {
    "version": "2",
    "default_model": {
      "provider": "zed.dev",
      "model": "claude-3-5-sonnet"
    }
  }
}
```

#### Common Panel Settings

| key            | type    | default | description                                                                           |
| -------------- | ------- | ------- | ------------------------------------------------------------------------------------- |
| enabled        | boolean | true    | Disabling this will completely disable the assistant                                  |
| button         | boolean | true    | Show the assistant icon                                                               |
| dock           | string  | "right" | The default dock position for the assistant panel. Can be ["left", "right", "bottom"] |
| default_height | string  | null    | The pixel height of the assistant panel when docked to the bottom                     |
| default_width  | string  | null    | The pixel width of the assistant panel when docked to the left or right               |
