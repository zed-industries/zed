# Model Temperature

Zed's settings allow you to specify a custom temperature for a provider and/or model:

```json
"model_parameters": [
      // To set parameters for all requests to OpenAI models:
      {
        "provider": "openai",
        "temperature": 0.5
      },
      // To set parameters for all requests in general:
      {
        "temperature": 0
      },
      // To set parameters for a specific provider and model:
      {
        "provider": "zed.dev",
        "model": "claude-3-7-sonnet-latest",
        "temperature": 1.0
      }
    ],
```
