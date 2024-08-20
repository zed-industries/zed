# Configuring the Assistant

## Settings

| key           | type   | default | description                   |
| ------------- | ------ | ------- | ----------------------------- |
| version       | string | "2"     | The version of the assistant. |
| default_model | object | {}      | The default model to use.     |

### Configuring the default model

The `default_model` object can contain the following keys:

```json
// settings.json
{
  "assistant": {
    "default_model": {
      "provider": "zed.dev",
      "model": "claude-3-5-sonnet"
    }
  }
}
```

## Common Panel Settings

| key            | type    | default | description                                                                           |
| -------------- | ------- | ------- | ------------------------------------------------------------------------------------- |
| enabled        | boolean | true    | Disabling this will completely disable the assistant                                  |
| button         | boolean | true    | Show the assistant icon                                                               |
| dock           | string  | "right" | The default dock position for the assistant panel. Can be ["left", "right", "bottom"] |
| default_height | string  | null    | The pixel height of the assistant panel when docked to the bottom                     |
| default_width  | string  | null    | The pixel width of the assistant panel when docked to the left or right               |

## Example Configuration

```json
// settings.json
{
  "assistant": {
    "default_model": {
      "provider": "zed.dev",
      "model": "claude-3-5-sonnet-20240620"
    },
    "version": "2",
    "button": true,
    "default_width": 480,
    "dock": "right",
    "enabled": true
  }
}
```

## Providers {#providers}

The following providers are supported:

- Zed AI (Configured by default when signed in)
- [Anthropic](#anthropic)
- [GitHub Copilot Chat](#github-copilot-chat)
- [Google Gemini](#google-gemini)
- [Ollama](#ollama)
- [OpenAI](#openai)
- [OpenAI Custom Endpoint](#openai-custom-endpoint)

### Zed AI {#zed-ai}

A hosted service providing convenient and performant support for AI-enabled coding in Zed, powered by Anthropic's Claude 3.5 Sonnet and accessible just by signing in.

### Anthropic {#anthropic}

You can use Claude 3.5 Sonnet via [Zed AI](#zed-ai) for free. To use other Anthropic models you will need to configure it by providing your own API key.

You can obtain an API key [here](https://console.anthropic.com/settings/keys).

Even if you pay for Claude Pro, you will still have to [pay for additional credits](https://console.anthropic.com/settings/plans) to use it via the API.

### GitHub Copilot Chat {#github-copilot-chat}

You can use GitHub Copilot chat with the Zed assistant by choosing it via the model dropdown in the assistant panel.

### Google Gemini {#google-gemini}

You can use Gemini 1.5 Pro/Flash with the Zed assistant by choosing it via the model dropdown in the assistant panel.

You can obtain an API key [here](https://aistudio.google.com/app/apikey).

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

<!--
TBD: OpenAI Setup flow: Review/Correct/Simplify
-->

1. Create an [OpenAI API key](https://platform.openai.com/account/api-keys)
2. Make sure that your OpenAI account has credits
3. Open the assistant panel, using either the `assistant: toggle focus` or the `workspace: toggle right dock` action in the command palette (`cmd-shift-p`).
4. Make sure the assistant panel is focused:

   ![The focused assistant panel](https://zed.dev/img/assistant/assistant-focused.png)

The OpenAI API key will be saved in your keychain.

Zed will also use the `OPENAI_API_KEY` environment variable if it's defined.

#### OpenAI Custom Endpoint {#openai-custom-endpoint}

You can use a custom API endpoint for OpenAI, as long as it's compatible with the OpenAI API structure.

To do so, add the following to your Zed `settings.json`:

```json
{
  "language_models": {
    "openai": {
      "api_url": "http://localhost:11434/v1"
    }
  }
}
```

The custom URL here is `http://localhost:11434/v1`.
