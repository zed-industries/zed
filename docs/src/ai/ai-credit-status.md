# AI Credit Status

Zed can show how much AI credit or quota you have used for your **active LLM provider** directly in the status bar.

## Status bar indicator

When enabled, a compact progress bar appears on the right side of the status bar. It reflects usage from 0% to 100% with color bands:

- 0–25%: green
- 25–50%: yellow
- 50–75%: orange
- 75–100%: red

Hover for details, or click to open the provider billing page when available.

## Settings

Configure under `ai_credit_status` in your settings file:

```json
"ai_credit_status": {
  "enabled": true,
  "refresh_seconds": 60,
  "monthly_budget_usd": null
}
```

- `enabled`: show or hide the indicator
- `refresh_seconds`: polling interval (minimum 15 seconds)
- `monthly_budget_usd`: optional budget used as the 100% mark for OpenAI, Anthropic, and Mistral when the provider API does not expose remaining credits

You can hide the indicator from the status bar context menu via **Hide Button**.

## Supported providers

| Provider | Data source |
| --- | --- |
| Zed Pro (hosted models) | Zed account token spend (when exposed by the cloud API) |
| GitHub Copilot Chat | GitHub Copilot internal usage API |
| OpenRouter | OpenRouter `/api/v1/key` |
| OpenAI | OpenAI usage API + optional `monthly_budget_usd` |
| Anthropic | Validates API key; configure `monthly_budget_usd` and check the Anthropic console for spend |
| Mistral | Validates API key; configure `monthly_budget_usd` and check the Mistral console for spend |

The indicator follows whichever provider is selected as the default model in Agent settings.

## Related discussions

- [Surfacing Token Count and API Billing info](https://github.com/zed-industries/zed/discussions/56161)
- [Token Spend progress bar on account page](https://github.com/zed-industries/zed/discussions/41148)
