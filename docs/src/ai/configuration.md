# Configuration

When using AI in Zed, you can configure multiple dimensions:

1. Which LLM providers you can use
   - Zed's hosted models, which require [authentication](../authentication.md) and [subscription](./subscription.md)
   - [Using your own API keys](./llm-providers.md), which do not
   - Using [external agents like Claude Code](./external-agents.md), which do not
2. [Model parameters and usage](./agent-settings.md#model-settings)
3. [Interactions with the Agent Panel](./agent-settings.md#agent-panel-settings)

## Turning AI Off Entirely

To disable all AI features, add the following to your `settings.json`:

```json [settings]
{
  "disable_ai": true
}
```

See [this blog post](https://zed.dev/blog/disable-ai-features) for background on this option.
