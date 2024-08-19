
## Setup Instructions

### OpenAI

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

#### OpenAI Custom Endpoint

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

### Ollama

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

### Anthropic

You can use Claude 3.5 Sonnet with the Zed assistant by choosing it via the model dropdown in the assistant panel.

You can obtain an API key [here](https://console.anthropic.com/settings/keys).

Even if you pay for Claude Pro, you will still have to [pay for additional credits](https://console.anthropic.com/settings/plans) to use it via the API.

### Google Gemini

You can use Gemini 1.5 Pro/Flash with the Zed assistant by choosing it via the model dropdown in the assistant panel.

You can obtain an API key [here](https://aistudio.google.com/app/apikey).

### GitHub Copilot Chat

You can use GitHub Copilot chat with the Zed assistant by choosing it via the model dropdown in the assistant panel.

Previous: [Assistant](assistant.md) | Next: [Introducing Contexts](contexts.md)
