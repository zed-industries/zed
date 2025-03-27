# Completions

Zed supports two sources for completions:

1. "Code Completions" provided by Language Servers (LSPs) automatically installed by Zed or via [Zed Language Extensions](languages.md).
2. "Edit Predictions" provided by Zed's own Zeta model or by external providers like [GitHub Copilot](#github-copilot) or [Supermaven](#supermaven).

## Language Server Code Completions {#code-completions}

When there is an appropriate language server available, Zed will provide completions of variable names, functions, and other symbols in the current file. You can disable these by adding the following to your Zed `settings.json` file:

```json
"show_completions_on_input": false
```

You can manually trigger completions with `ctrl-space` or by triggering the `editor::ShowCompletions` action from the command palette.

For more information, see:

- [Configuring Supported Languages](./configuring-languages.md)
- [List of Zed Supported Languages](./languages.md)

## Edit Predictions {#edit-predictions}

Zed has built-in support for predicting multiple edits at a time [via Zeta](https://huggingface.co/zed-industries/zeta), Zed's open-source and open-data model.
Edit predictions appear as you type, and most of the time, you can accept them by pressing `tab`.

### Configuring Zeta

Zed's Edit Prediction was initially introduced via a banner on the title bar.
Clicking on it would take you to a modal with a button ("Enable Edit Prediction") that sets `zed` as your `edit_prediction_provider`.

![Onboarding banner and modal](https://zed.dev/img/edit-prediction/docs.webp)

But, if you haven't come across the banner, Zed's Edit Prediction is the default edit prediction provider and you should see it right away in your status bar.

### Switching Modes {#switching-modes}

Zed's Edit Prediction comes with two different display modes:

1. `eager` (default): predictions are displayed inline as long as it doesn't conflict with language server completions
2. `subtle`: predictions only appear inline when holding a modifier key (`alt` by default)

Toggle between them via the `mode` key:

```json
"edit_predictions": {
  "mode": "eager" | "subtle"
},
```

Or directly via the UI through the status bar menu:

![Edit Prediction status bar menu, with the modes toggle.](https://zed.dev/img/edit-prediction/status-bar-menu.webp)

### Conflict With Other `tab` Actions {#edit-predictions-conflict}

By default, when `tab` would normally perform a different action, Zed requires a modifier key to accept predictions:

1. When the language server completions menu is visible.
2. When your cursor isn't at the right indentation level.

In these cases, `alt-tab` is used instead to accept the prediction. When the language server completions menu is open, holding `alt` first will cause it to temporarily disappear in order to preview the prediction within the buffer.

On Linux, `alt-tab` is often used by the window manager for switching windows, so `alt-l` is provided as the default binding for accepting predictions. `tab` and `alt-tab` also work, but aren't displayed by default.

{#action editor::AcceptPartialEditPrediction} ({#kb editor::AcceptPartialEditPrediction}) can be used to accept the current edit prediction up to the next word boundary.

See the [Configuring GitHub Copilot](#github-copilot) and [Configuring Supermaven](#supermaven) sections below for configuration of other providers. Only text insertions at the current cursor are supported for these providers, whereas the Zeta model provides multiple predictions including deletions.

## Configuring Edit Prediction Keybindings {#edit-predictions-keybinding}

By default, `tab` is used to accept edit predictions. You can use another keybinding by inserting this in your keymap:

```json
{
  "context": "Editor && edit_prediction",
  "bindings": {
    // Here we also allow `alt-enter` to accept the prediction
    "alt-enter": "editor::AcceptEditPrediction"
  }
}
```

When there's a [conflict with the `tab` key](#edit-predictions-conflict), Zed uses a different context to accept keybindings (`edit_prediction_conflict`). If you want to use a different one, you can insert this in your keymap:

```json
{
  "context": "Editor && edit_prediction_conflict",
  "bindings": {
    "ctrl-enter": "editor::AcceptEditPrediction" // Example of a modified keybinding
  }
}
```

If your keybinding contains a modifier (`ctrl` in the example above), it will also be used to preview the edit prediction and temporarily hide the language server completion menu.

You can also bind this action to keybind without a modifier. In that case, Zed will use the default modifier (`alt`) to preview the edit prediction.

```json
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

```json
{
  "context": "Editor && edit_prediction_conflict && !showing_completions",
  "bindings": {
    // Here we don't require a modifier unless there's a language server completion
    "tab": "editor::AcceptEditPrediction"
  }
}
```

### Keybinding Example: Always Use Alt-Tab

The keybinding example below causes `alt-tab` to always be used instead of sometimes using `tab`. You might want this in order to have just one keybinding to use for accepting edit predictions, since the behavior of `tab` varies based on context.

```json
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

If `"vim_mode": true` is set within `settings.json`, then additional bindings are needed after the above to return `tab` to its original behavior:

```json
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

While `tab` and `alt-tab` are supported on Linux, `alt-l` is displayed instead. If your window manager does not reserve `alt-tab`, and you would prefer to use `tab` and `alt-tab`, include these bindings in `keymap.json`:

```json
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

```json
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

```json
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

```json
{
  "show_edit_predictions": false
}
```

This hides every indication that there is a prediction available, regardless of [the display mode](#switching-modes) you're in (valid only if you have Zed as your provider).
Still, you can trigger edit predictions manually by executing {#action editor::ShowEditPrediction} or hitting {#kb editor::ShowEditPrediction}.

### For Specific Languages

To not have predictions appear automatically as you type when working with a specific language, set this within `settings.json`:

```json
{
  "language": {
    "python": {
      "show_edit_predictions": false
    }
  }
}
```

### Turning Off Completely

To completely turn off edit prediction across all providers, explicitly set the settings to `none`, like so:

```json
"features": {
  "edit_prediction_provider": "none"
},
```

## Configuring GitHub Copilot {#github-copilot}

To use GitHub Copilot as your provider, set this within `settings.json`:

```json
{
  "features": {
    "edit_prediction_provider": "copilot"
  }
}
```

You should be able to sign-in to GitHub Copilot by clicking on the Copilot icon in the status bar and following the setup instructions.

Copilot can provide multiple completion alternatives, and these can be navigated with the following actions:

- {#action editor::NextEditPrediction} ({#kb editor::NextEditPrediction}): To cycle to the next edit prediction
- {#action editor::PreviousEditPrediction} ({#kb editor::PreviousEditPrediction}): To cycle to the previous edit prediction

## Configuring Supermaven {#supermaven}

To use Supermaven as your provider, set this within `settings.json`:

```json
{
  "features": {
    "edit_prediction_provider": "supermaven"
  }
}
```

You should be able to sign-in to Supermaven by clicking on the Supermaven icon in the status bar and following the setup instructions.

## See also

You may also use the Assistant Panel or the Inline Assistant to interact with language models, see [the assistant documentation](assistant/assistant.md) for more information.
