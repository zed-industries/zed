# Zed AI Improvement

<!-- todo - complete agent panel data collection -->

## Agent Panel

### Opt-In

When using the Zed Agent Panel, whether through Zed's hosted AI service or via connecting a non-Zed AI service via API key, Zed does not persistently store user content or use user content to evaluate and/or improve our AI features, unless it is explicitly shared with Zed. Each share is opt-in, and sharing once will not cause future content or data to be shared again.

> Note that rating responses will send your data related to that response to Zed's servers.
> **_If you don't want data persisted on Zed's servers, don't rate_**. We will not collect data for improving our Agentic offering without you explicitly rating responses.

When using upstream services through Zed AI, we require assurances from our service providers that your user content won't be used for training models. For example, usage of Anthropic Claude 3.5 via Zed AI in the Assistant is governed by the [Anthropic Commercial Terms](https://www.anthropic.com/legal/commercial-terms) which includes the following:

> "Anthropic may not train models on Customer Content from paid Services."

When you directly connect the Zed Assistant with a non Zed AI service (e.g. via API key) Zed does not have control over how your data is used by that service provider. You should reference your agreement with each service provider to understand what terms and conditions apply.

### Data we collect

For prompts you have explicitly shared with us, Zed may store copies of those prompts and other data about the specific use of the Agent Panel.

This data includes:

- The prompt given to the Agent
- Any commentary you include
- Product telemetry about the agentic thread
- Metadata about your Zed installation

### Data Handling

Collected data is stored in Snowflake, a private database where we track other metrics. We periodically review this data to improve our overall agentic approach and refine the product via our system prompt, tool use, etc. We ensure any included data is anonymized and contains no sensitive information (access tokens, user IDs, email addresses, etc).

## Edit Predictions

By default, when using Zed Edit Predictions, Zed does not persistently store user content or use user content for training of its models.

### Opt-in

Users who are working on open source licensed projects may optionally opt-in to providing model improvement feedback. This opt-in occurs on a per-project basis. If you work on multiple open source projects and wish to provide model improvement feedback you will have to opt-in for each individual project.

When working on other projects where you haven't opted-in, Zed will not persistently store user content or use user content for training of its models.

You can see exactly how Zed detects open source licenses in: [license_detection.rs](https://github.com/zed-industries/zed/blob/main/crates/zeta/src/license_detection.rs).

### Exclusions

Zed will intentionally exclude certain files from Predictive Edits entirely, even when you have opted-in to model improvement feedback.

You can inspect this exclusion list by opening `zed: open default settings` from the command palette:

```json
{
  "edit_predictions": {
    // A list of globs representing files that edit predictions should be disabled for.
    // There's a sensible default list of globs already included.
    // Any addition to this list will be merged with the default list.
    "disabled_globs": [
      "**/.env*",
      "**/*.pem",
      "**/*.key",
      "**/*.cert",
      "**/*.crt",
      "**/secrets.yml"
    ]
  }
}
```

Users may explicitly exclude additional paths and/or file extensions by adding them to [`edit_predictions.disabled_globs`](https://zed.dev/docs/configuring-zed#edit-predictions) in their Zed settings.json:

```json
{
  "edit_predictions": {
    "disabled_globs": ["secret_dir/*", "**/*.log"]
  }
}
```

### Data we collect

For open source projects where you have opted-in, Zed may store copies of requests and responses to the Zed AI Prediction service.

This data includes:

- the edit prediction
- a portion of the buffer content around the cursor
- a few recent edits
- the current buffer outline
- diagnostics (errors, warnings, etc) from language servers

### Data Handling

Collected data is stored in Snowflake, a private database where we track other metrics. We periodically review this data to select training samples for inclusion in our model training dataset. We ensure any included data is anonymized and contains no sensitive information (access tokens, user IDs, email addresses, etc). This training dataset is publicly available at: [huggingface.co/datasets/zed-industries/zeta](https://huggingface.co/datasets/zed-industries/zeta).

### Model Output

We then use this training dataset to fine-tune [Qwen2.5-Coder-7B](https://huggingface.co/Qwen/Qwen2.5-Coder-7B) and make the resulting model available at [huggingface.co/zed-industries/zeta](https://huggingface.co/zed-industries/zeta).

## Applicable terms

Please see the [Zed Terms of Service](https://zed.dev/terms-of-service) for more.
