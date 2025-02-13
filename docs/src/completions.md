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
- [List of Zed Supported Languages](./languages.md).

## Edit Predictions {#edit-predictions}

Zed has built-in support for predicting multiple edits at a time via its Zeta model. Clicking "Introducing: Edit Prediction" on the top right will open a brief prompt setting up this feature.

Edit predictions appear as you type, and you can accept them by pressing `tab`. The `tab` key is already used for accepting language server completions and for indenting. In these cases, `alt-tab` is used instead to accept the prediction. When the completions menu is open, holding `alt` will cause it to temporarily disappear in order to view the prediction within the buffer.

On Linux, `alt-tab` is often used by the window manager for switching windows, so `alt-l` is provided as the default binding for accepting predictions. `tab` and `alt-tab` also work, but aren't displayed by default.

{#action editor::AcceptPartialEditPrediction} ({#kb editor::AcceptPartialEditPrediction}) can be used to accept the current edit prediction up to the next word boundary.

See the [Configuring GitHub Copilot](#github-copilot) and [Configuring Supermaven](#supermaven) sections below for configuration of other providers. Only text insertions at the current cursor are supported for these providers, whereas the Zeta model provides multiple predictions including deletions.

## Configuring Edit Prediction Keybindings

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

When you have both a language server completion and an edit prediction on screen at the same time, Zed uses a different context to accept keybindings (`edit_prediction_conflict`). If you want to use a different keybinding, you can insert this in your keymap:

```json
{
  "context": "Editor && edit_prediction_conflict",
  "bindings": {
    "ctrl-enter": "editor::AcceptEditPrediction"
  }
}
```

If your keybinding contains a modifier (`ctrl` in the example), it will be used to preview the edit prediction and temporarily hide the language server completion menu.

You can also bind a keystroke without a modifier. In that case, Zed will use the default modifier (`alt`) to preview the edit prediction.

```json
{
  "context": "Editor && edit_prediction_conflict",
  "bindings": {
    // Here we bind tab to accept even when there's a language server completion
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

## Disabling Automatic Edit Prediction

To disable predictions that appear automatically as you type, set this within `settings.json`:

```json
{
  "show_edit_predictions": false
}
```

You can trigger edit predictions manually by executing {#action editor::ShowEditPrediction} ({#kb editor::ShowEditPrediction}).

You can also add this as a language-specific setting in your `settings.json` to disable edit predictions for a specific language:

```json
{
  "language": {
    "python": {
      "show_edit_predictions": false
    }
  }
}
```

## Configuring GitHub Copilot {#github-copilot}

To use GitHub Copilot, set this within `settings.json`:

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

To use Supermaven, set this within `settings.json`:

```json
{
  "features": {
    "edit_prediction_provider": "supermaven"
  }
}
```

You should be able to sign-in to Supermaven by clicking on the Supermaven icon in the status bar and following the setup instructions.

## See also

You may also use the Assistant Panel or the Inline Assistant to interact with language models, see the [assistant](assistant/assistant.md) documentation for more information.
