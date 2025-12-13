# Edit Prediction

Edit Prediction is Zed's LLM mechanism for predicting the code you want to write.
Each keystroke sends a new request to the edit prediction provider, which returns individual or multi-line suggestions that can be quickly accepted by pressing `tab`.

The default provider is [Zeta, a proprietary open source and open dataset model](https://huggingface.co/zed-industries/zeta), but you can also use [other providers](#other-providers) like GitHub Copilot, Supermaven, and Codestral.

## Configuring Zeta

To use Zeta, the only thing you need to do is [to sign in](../authentication.md#what-features-require-signing-in).
After doing that, you should already see predictions as you type on your files.

You can confirm that Zeta is properly configured either by verifying whether you have the following code in your `settings.json`:

```json [settings]
"features": {
  "edit_prediction_provider": "zed"
},
```

Or you can also look for a little Z icon in the right of your status bar at the bottom.

### Pricing and Plans

From just signing in, while in Zed's free plan, you get 2,000 Zeta-powered edit predictions per month.
But you can get _**unlimited edit predictions**_ by upgrading to [the Pro plan](../ai/plans-and-usage.md).
More information can be found in [Zed's pricing page](https://zed.dev/pricing).

### Switching Modes {#switching-modes}

Zed's Edit Prediction comes with two different display modes:

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

### Conflict With Other `tab` Actions {#edit-predictions-conflict}

By default, when `tab` would normally perform a different action, Zed requires a modifier key to accept predictions:

1. When the language server completions menu is visible.
2. When your cursor isn't at the right indentation level.

In these cases, `alt-tab` is used instead to accept the prediction. When the language server completions menu is open, holding `alt` first will cause it to temporarily disappear in order to preview the prediction within the buffer.

On Linux, `alt-tab` is often used by the window manager for switching windows, so `alt-l` is provided as the default binding for accepting predictions. `tab` and `alt-tab` also work, but aren't displayed by default.

{#action editor::AcceptNextWordEditPrediction} ({#kb editor::AcceptNextWordEditPrediction}) can be used to accept the current edit prediction up to the next word boundary.
{#action editor::AcceptNextLineEditPrediction} ({#kb editor::AcceptNextLineEditPrediction}) can be used to accept the current edit prediction up to the new line boundary.

## Configuring Edit Prediction Keybindings {#edit-predictions-keybinding}

By default, `tab` is used to accept edit predictions. You can use another keybinding by inserting this in your keymap:

```json [settings]
{
  "context": "Editor && edit_prediction",
  "bindings": {
    // Here we also allow `alt-enter` to accept the prediction
    "alt-enter": "editor::AcceptEditPrediction"
  }
}
```

When there's a [conflict with the `tab` key](#edit-predictions-conflict), Zed uses a different key context to accept keybindings (`edit_prediction_conflict`).
If you want to use a different one, you can insert this in your keymap:

```json [settings]
{
  "context": "Editor && edit_prediction_conflict",
  "bindings": {
    "ctrl-enter": "editor::AcceptEditPrediction" // Example of a modified keybinding
  }
}
```

If your keybinding contains a modifier (`ctrl` in the example above), it will also be used to preview the edit prediction and temporarily hide the language server completion menu.

You can also bind this action to keybind without a modifier.
In that case, Zed will use the default modifier (`alt`) to preview the edit prediction.

```json [settings]
{
  "context": "Editor && edit_prediction_conflict",
  "bindings": {
    // Here we bind tab to accept even when there's a language server completion
    // or the cursor isn't at the correct indentation level
    "tab": "editor::AcceptEditPrediction"
  }
}
```

To maintain the use of the modifier key for accepting predictions when there is a language server completions menu, but allow `tab` to accept predictions regardless of cursor position, you can specify the context further with `showing_completions`:

```json [settings]
{
  "context": "Editor && edit_prediction_conflict && !showing_completions",
  "bindings": {
    // Here we don't require a modifier unless there's a language server completion
    "tab": "editor::AcceptEditPrediction"
  }
}
```

### Keybinding Example: Always Use Tab

If you want to use `tab` to always accept edit predictions, you can use the following keybinding:

```json [keymap]
{
  "context": "Editor && edit_prediction_conflict && showing_completions",
  "bindings": {
    "tab": "editor::AcceptEditPrediction"
  }
}
```

This will make `tab` work to accept edit predictions _even when_ you're also seeing language server completions.
That means that you need to rely on `enter` for accepting the latter.

### Keybinding Example: Always Use Alt-Tab

The keybinding example below causes `alt-tab` to always be used instead of sometimes using `tab`.
You might want this in order to have just one (alternative) keybinding to use for accepting edit predictions, since the behavior of `tab` varies based on context.

```json [keymap]
  {
    "context": "Editor && edit_prediction",
    "bindings": {
      "alt-tab": "editor::AcceptEditPrediction"
    }
  },
  // Bind `tab` back to its original behavior.
  {
    "context": "Editor",
    "bindings": {
      "tab": "editor::Tab"
    }
  },
  {
    "context": "Editor && showing_completions",
    "bindings": {
      "tab": "editor::ComposeCompletion"
    }
  },
```

If you are using [Vim mode](../vim.md), then additional bindings are needed after the above to return `tab` to its original behavior:

```json [keymap]
  {
    "context": "(VimControl && !menu) || vim_mode == replace || vim_mode == waiting",
    "bindings": {
      "tab": "vim::Tab"
    }
  },
  {
    "context": "vim_mode == literal",
    "bindings": {
      "tab": ["vim::Literal", ["tab", "\u0009"]]
    }
  },
```

### Keybinding Example: Displaying Tab and Alt-Tab on Linux

While `tab` and `alt-tab` are supported on Linux, `alt-l` is displayed instead.
If your window manager does not reserve `alt-tab`, and you would prefer to use `tab` and `alt-tab`, include these bindings in `keymap.json`:

```json [keymap]
  {
    "context": "Editor && edit_prediction",
    "bindings": {
      "tab": "editor::AcceptEditPrediction",
      // Optional: This makes the default `alt-l` binding do nothing.
      "alt-l": null
    }
  },
  {
    "context": "Editor && edit_prediction_conflict",
    "bindings": {
      "alt-tab": "editor::AcceptEditPrediction",
      // Optional: This makes the default `alt-l` binding do nothing.
      "alt-l": null
    }
  },
```

### Missing keybind {#edit-predictions-missing-keybinding}

Zed requires at least one keybinding for the {#action editor::AcceptEditPrediction} action in both the `Editor && edit_prediction` and `Editor && edit_prediction_conflict` contexts ([learn more above](#edit-predictions-keybinding)).

If you have previously bound the default keybindings to different actions in the global context, you will not be able to preview or accept edit predictions. For example:

```json [keymap]
[
  // Your keymap
  {
    "bindings": {
      // Binds `alt-tab` to a different action globally
      "alt-tab": "menu::SelectNext"
    }
  }
]
```

To fix this, you can specify your own keybinding for accepting edit predictions:

```json [keymap]
[
  // ...
  {
    "context": "Editor && edit_prediction_conflict",
    "bindings": {
      "alt-l": "editor::AcceptEditPrediction"
    }
  }
]
```

If you would like to use the default keybinding, you can free it up by either moving yours to a more specific context or changing it to something else.

## Disabling Automatic Edit Prediction

There are different levels in which you can disable edit predictions to be displayed, including not having it turned on at all.

Alternatively, if you have Zed set as your provider, consider [using Subtle Mode](#switching-modes).

### On Buffers

To not have predictions appear automatically as you type, set this within `settings.json`:

```json [settings]
{
  "show_edit_predictions": false
}
```

This hides every indication that there is a prediction available, regardless of [the display mode](#switching-modes) you're in (valid only if you have Zed as your provider).
Still, you can trigger edit predictions manually by executing {#action editor::ShowEditPrediction} or hitting {#kb editor::ShowEditPrediction}.

### For Specific Languages

To not have predictions appear automatically as you type when working with a specific language, set this within `settings.json`:

```json [settings]
{
  "language": {
    "python": {
      "show_edit_predictions": false
    }
  }
}
```

### In Specific Directories

To disable edit predictions for specific directories or files, set this within `settings.json`:

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
"features": {
  "edit_prediction_provider": "none"
},
```

## Configuring Other Providers {#other-providers}

Zed's Edit Prediction also work with other completion model providers aside from Zeta.
Learn about the available ones below.

### GitHub Copilot {#github-copilot}

To use GitHub Copilot as your provider, set this within `settings.json`:

```json [settings]
{
  "features": {
    "edit_prediction_provider": "copilot"
  }
}
```

You should be able to sign-in to GitHub Copilot by clicking on the Copilot icon in the status bar and following the setup instructions.

#### Using GitHub Copilot Enterprise

If your organization uses GitHub Copilot Enterprise, you can configure Zed to use your enterprise instance by specifying the enterprise URI in your `settings.json`:

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

Once set, Zed will route Copilot requests through your enterprise endpoint.
When you sign in by clicking the Copilot icon in the status bar, you will be redirected to your configured enterprise URL to complete authentication.
All other Copilot features and usage remain the same.

Copilot can provide multiple completion alternatives, and these can be navigated with the following actions:

- {#action editor::NextEditPrediction} ({#kb editor::NextEditPrediction}): To cycle to the next edit prediction
- {#action editor::PreviousEditPrediction} ({#kb editor::PreviousEditPrediction}): To cycle to the previous edit prediction

### Supermaven {#supermaven}

To use Supermaven as your provider, set this within `settings.json`:

```json [settings]
{
  "features": {
    "edit_prediction_provider": "supermaven"
  }
}
```

You should be able to sign-in to Supermaven by clicking on the Supermaven icon in the status bar and following the setup instructions.

### Codestral {#codestral}

To use Mistral's Codestral as your provider, start by going to the Agent Panel settings view by running the {#action agent::OpenSettings} action.
Look for the Mistral item and add a Codestral API key in the corresponding text input.

After that, you should be able to switch your provider to it in your `settings.json` file:

```json [settings]
{
  "features": {
    "edit_prediction_provider": "codestral"
  }
}
```

## See also

To learn about other ways to interact with AI in Zed, you may also want to see more about the [Agent Panel](./agent-panel.md) or the [Inline Assistant](./inline-assistant.md) feature.
