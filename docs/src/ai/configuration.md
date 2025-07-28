# Configuration

When using AI in Zed, you can customize several aspects:

1. Which [LLM providers](./llm-providers.md) you can use
2. [Model parameters and usage](./agent-settings.md#model-settings)
3. [Interactions with the Agent Panel](./agent-settings.md#agent-panel-settings)

## Turning AI Off Entirely

We want to respect users who want to use Zed without interacting with AI whatsoever.
To do that, add the following key to your `settings.json`:

```json
{
  "disable_ai": true
}
```

Read [the following blog post](https://zed.dev/blog/disable-ai-features) to learn more about our motivation to promote this, as much as we also encourage users to explore AI-assisted programming.
