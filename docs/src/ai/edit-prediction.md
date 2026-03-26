---
title: AI Code Completion in Zed - Zeta, Copilot, Codestral, Mercury Coder
description: Set up AI code completions in Zed with Zeta (built-in), GitHub Copilot, Codestral, or Mercury Coder. Multi-line predictions on every keystroke.
---

# Edit Prediction

Edit Prediction is how Zed's AI code completions work: an LLM predicts the code you want to write.
Each keystroke sends a new request to the edit prediction provider, which returns individual or multi-line suggestions you accept by pressing `tab`.

The default provider is [Zeta, a proprietary open source and open dataset model](https://huggingface.co/zed-industries/zeta), but you can also use [other providers](#other-providers) like GitHub Copilot, Mercury Coder, and Codestral.

## Configuring Zeta

To use Zeta, [sign in](../authentication.md#what-features-require-signing-in).
Once signed in, predictions appear as you type.

You can confirm that Zeta is properly configured by opening the [Settings Editor](zed://settings/edit_predictions.providers) (`Cmd+,` on macOS or `Ctrl+,` on Linux/Windows) and searching for `edit_predictions`. The `provider` field should be set to `Zed AI`.

Or verify this in your settings.json:

```json [settings]
{
  "edit_predictions": {
    "provider": "zed"
  }
}
```

The Z icon in the status bar also indicates Zeta is active.

### Pricing and Plans

The free plan includes 2,000 Zeta predictions per month. The [Pro plan](../ai/plans-and-usage.md) removes this limit. See [Zed's pricing page](https://zed.dev/pricing) for details.

### Switching Modes {#switching-modes}

Edit Prediction has two display modes:

1. `eager` (default): predictions are displayed inline as long as it doesn't conflict with language server completions
2. `subtle`: predictions only appear inline when holding a modifier key (`alt` by default)

Toggle between them via the `mode` key:

```json [settings]
"edit_predictions": {
  "mode": "eager" // or "subtle"
},
```

Or directly via the UI through the status bar menu:

![Edit Prediction status bar menu, with the modes toggle.](https://zed.dev/img/edit-prediction/status-bar-menu.webp)

> Note that edit prediction modes work with any prediction provider.

## Default Key Bindings

On macOS and Windows, you can accept edit predictions with `alt-tab`. On Linux, `alt-tab` is often used by the window manager for switching windows, so `alt-l` is the default key binding for edit predictions.

In `eager` mode, you can also use the `tab` key to accept edit predictions, unless the completion menu is open, in which case `tab` accepts LSP completions. To use `tab` to insert whitespace, you need to dismiss the prediction with {#kb editor::Cancel} before hitting `tab`.

{#action editor::AcceptNextWordEditPrediction} ({#kb editor::AcceptNextWordEditPrediction}) can be used to accept the current edit prediction up to the next word boundary.
{#action editor::AcceptNextLineEditPrediction} ({#kb editor::AcceptNextLineEditPrediction}) can be used to accept the current edit prediction up to the new line boundary.

## Configuring Edit Prediction Keybindings {#edit-predictions-keybinding}

### Keybinding Example: Always Use Tab

To always use `tab` for accepting edit predictions, regardless of whether the LSP completions menu is open, you can add the following to your keymap:

Open the keymap editor with {#action zed::OpenKeymap} ({#kb zed::OpenKeymap}), search for `AcceptEditPrediction`, right click on the binding for `tab` and hit `edit`. Then change the context the binding is active in to just `Editor && edit_prediction` and save it.

Alternatively, you can put the following in your `keymap.json`:

```json [keymap]
[
  {
    "context": "Editor && edit_prediction",
    "bindings": {
      "tab": "editor::AcceptEditPrediction"
    }
  }
]
```

After that, {#kb editor::ComposeCompletion} remains available for accepting LSP completions.

### Keybinding Example: Always Use Alt-Tab

To stop using `tab` for accepting edit predictions and always use `alt-tab` instead, unbind the default `tab` binding in the eager edit prediction context:

Open the keymap editor with {#action zed::OpenKeymap} ({#kb zed::OpenKeymap}), search for `AcceptEditPrediction`, right click on the binding for `tab` and delete it.

Alternatively, you can put the following in your `keymap.json`:

```json [keymap]
[
  {
    "context": "Editor && edit_prediction",
    "unbind": {
      "tab": "editor::AcceptEditPrediction"
    }
  }
]
```

After that, `alt-tab` remains available for accepting edit predictions, and on Linux `alt-l` does too unless you unbind it.

### Keybinding Example: Rebind Both Tab and Alt-Tab

To move both default accept bindings to something else, unbind them and add your replacement:

Open the keymap editor with {#action zed::OpenKeymap} ({#kb zed::OpenKeymap}), search for `AcceptEditPrediction`, right click on the binding for `tab` and delete it. Then right click on the binding for `alt-tab`, select "Edit", and record your desired keystrokes before hitting saving.

Alternatively, you can put the following in your `keymap.json`:

```json [keymap]
[
  {
    "context": "Editor && edit_prediction",
    "unbind": {
      "alt-tab": "editor::AcceptEditPrediction",
      // Add this as well on Windows/Linux
      // "alt-l": "editor::AcceptEditPrediction",
      "tab": "editor::AcceptEditPrediction"
    },
    "bindings": {
      "ctrl-enter": "editor::AcceptEditPrediction"
    }
  }
]
```

In this case, because the binding contains the modifier `ctrl`, it will be used to preview the prediction in subtle mode, or when the completions menu is open.

### Cleaning Up Older Keymap Entries

If you configured edit prediction keybindings before Zed `v0.229.0`, your `keymap.json` may have entries that are now redundant.

**Old tab workaround**: Before `unbind` existed, the only way to prevent `tab` from accepting edit predictions was to copy all the default non-edit-prediction `tab` bindings into your keymap alongside a custom `AcceptEditPrediction` binding. If your keymap still contains those copy-pasted entries, delete them and use a single `"unbind"` entry as shown in the examples above.

**Renamed context**: The `edit_prediction_conflict` context has been replaced by `edit_prediction && (showing_completions || in_leading_whitespace)`. Zed automatically migrates any bindings that used `edit_prediction_conflict`, so no changes are required on your end.

## Disabling Automatic Edit Prediction

You can disable edit predictions at several levels, or turn them off entirely.

Alternatively, if you have Zed set as your provider, consider [using Subtle Mode](#switching-modes).

### On Buffers

To not have predictions appear automatically as you type, set this in your settings file ([how to edit](../configuring-zed.md#settings-files)):

```json [settings]
{
  "show_edit_predictions": false
}
```

This hides every indication that there is a prediction available, regardless of [the display mode](#switching-modes) you're in (valid only if you have Zed as your provider).
Still, you can trigger edit predictions manually by executing {#action editor::ShowEditPrediction} or hitting {#kb editor::ShowEditPrediction}.

### For Specific Languages

To not have predictions appear automatically as you type when working with a specific language, set this in your settings file ([how to edit](../configuring-zed.md#settings-files)):

```json [settings]
{
  "languages": {
    "Python": {
      "show_edit_predictions": false
    }
  }
}
```

### In Specific Directories

To disable edit predictions for specific directories or files, set this in your settings file ([how to edit](../configuring-zed.md#settings-files)):

```json [settings]
{
  "edit_predictions": {
    "disabled_globs": ["~/.config/zed/settings.json"]
  }
}
```

### Turning Off Completely

To completely turn off edit prediction across all providers, explicitly set the settings to `none`, like so:

```json [settings]
{
  "edit_predictions": {
    "provider": "none"
  }
}
```

## Configuring Other Providers {#other-providers}

Edit Prediction also works with other providers.

### GitHub Copilot {#github-copilot}

To use GitHub Copilot as your provider, set this in your settings file ([how to edit](../configuring-zed.md#settings-files)):

```json [settings]
{
  "edit_predictions": {
    "provider": "copilot"
  }
}
```

To sign in to GitHub Copilot, click on the Copilot icon in the status bar. A popup window appears displaying a device code. Click the copy button to copy the code, then click "Connect to GitHub" to open the GitHub verification page in your browser. Paste the code when prompted. The popup window closes automatically after successful authorization.

#### Using GitHub Copilot Enterprise

If your organization uses GitHub Copilot Enterprise, you can configure Zed to use your enterprise instance by specifying the enterprise URI in your settings file ([how to edit](../configuring-zed.md#settings-files)):

```json [settings]
{
  "edit_predictions": {
    "copilot": {
      "enterprise_uri": "https://your.enterprise.domain"
    }
  }
}
```

Replace `"https://your.enterprise.domain"` with the URL provided by your GitHub Enterprise administrator (e.g., `https://foo.ghe.com`).

Once set, Zed routes Copilot requests through your enterprise endpoint.
When you sign in by clicking the Copilot icon in the status bar, you are redirected to your configured enterprise URL to complete authentication.
All other Copilot features and usage remain the same.

Copilot can provide multiple completion alternatives, and these can be navigated with the following actions:

- {#action editor::NextEditPrediction} ({#kb editor::NextEditPrediction}): To cycle to the next edit prediction
- {#action editor::PreviousEditPrediction} ({#kb editor::PreviousEditPrediction}): To cycle to the previous edit prediction

### Mercury Coder {#mercury-coder}

To use [Mercury Coder](https://www.inceptionlabs.ai/) by Inception Labs as your provider:

1. Open the Settings Editor ({#kb zed::OpenSettings})
2. Search for "Edit Predictions" and click **Configure Providers**
3. Find the Mercury section and enter your API key from the
   [Inception Labs dashboard](https://platform.inceptionlabs.ai/dashboard/api-keys)

Alternatively, click the edit prediction icon in the status bar and select
**Configure Providers** from the menu.

After adding your API key, Mercury Coder will appear in the provider dropdown in the status bar menu, where you can select it. You can also set it directly in your settings file:

```json [settings]
{
  "edit_predictions": {
    "provider": "mercury"
  }
}
```

### Codestral {#codestral}

To use Mistral's Codestral as your provider:

1. Open the Settings Editor (`Cmd+,` on macOS, `Ctrl+,` on Linux/Windows)
2. Search for "Edit Predictions" and click **Configure Providers**
3. Find the Codestral section and enter your API key from the
   [Codestral dashboard](https://console.mistral.ai/codestral)

Alternatively, click the edit prediction icon in the status bar and select
**Configure Providers** from the menu.

After adding your API key, Codestral will appear in the provider dropdown in the status bar menu, where you can select it. You can also set it directly in your settings file:

```json [settings]
{
  "edit_predictions": {
    "provider": "codestral"
  }
}
```

### Self-Hosted OpenAI-compatible servers

You can use any self-hosted server that implements the OpenAI completion API format. This works with vLLM, llama.cpp server, LocalAI, and other compatible servers.

#### Configuration

Set `open_ai_compatible_api` as your provider and configure the API endpoint:

```json [settings]
{
  "edit_predictions": {
    "provider": "open_ai_compatible_api",
    "open_ai_compatible_api": {
      "api_url": "http://localhost:8080/v1/completions",
      "model": "deepseek-coder-6.7b-base",
      "prompt_format": "deepseek_coder",
      "max_output_tokens": 64
    }
  }
}
```

The `prompt_format` setting controls how code context is formatted for the model. Use `"infer"` to detect the format from the model name, or specify one explicitly:

- `code_llama` - CodeLlama format: `<PRE> prefix <SUF> suffix <MID>`
- `star_coder` - StarCoder format: `<fim_prefix>prefix<fim_suffix>suffix<fim_middle>`
- `deepseek_coder` - DeepSeek format with special unicode markers
- `qwen` - Qwen/CodeGemma format: `<|fim_prefix|>prefix<|fim_suffix|>suffix<|fim_middle|>`
- `codestral` - Codestral format: `[SUFFIX]suffix[PREFIX]prefix`
- `glm` - GLM-4 format with code markers
- `infer` - Auto-detect from model name (default)

Your server must implement the OpenAI `/v1/completions` endpoint. Edit predictions will send POST requests with this format:

```json
{
  "model": "your-model-name",
  "prompt": "formatted-code-context",
  "max_tokens": 256,
  "temperature": 0.2,
  "stop": ["<|endoftext|>", ...]
}
```

## See also

- [Agent Panel](./agent-panel.md): Agentic editing with file read/write and terminal access
- [Inline Assistant](./inline-assistant.md): Prompt-driven transformations on selected code
