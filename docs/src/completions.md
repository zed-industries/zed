# Completions

Zed supports supports two sources for completions:

1. "Code Completions" provided by Language Servers (LSPs) automatically installed by Zed or via [Zed Language Extensions](languages.md).
2. "Edit Predictions" provided by [Zed AI](https://zed.dev/ai), or by third-party services like [GitHub Copilot](#github-copilot) or [Supermaven](#supermaven).

## Code Completions

When there is an appropriate language server available, Zed will by-default provide completions of variable names, functions, and other symbols in the current file. You can disable these by adding the following to your zed settings.json file:

```json
"show_completions_on_input": false
```

You can manually trigger completions with `ctrl-space` or by triggering the `editor::ShowCompletions` action from the command palette.

For more information, see:

- [Configuring Supported Languages](./configuring-languages.md)
- [List of Zed Supported Languages](./languages.md).

## Configuring Edit Prediction

### GitHub Copilot

To use GitHub Copilot (enabled by default), add the following to your `settings.json`:

```json
{
  "features": {
    "inline_completion_provider": "copilot"
  }
}
```

You should be able to sign-in to GitHub Copilot by clicking on the Copilot icon in the status bar and following the setup instructions.

### Supermaven

To use Supermaven, add the following to your `settings.json`:

```json
{
  "features": {
    "inline_completion_provider": "supermaven"
  }
}
```

You should be able to sign-in to Supermaven by clicking on the Supermaven icon in the status bar and following the setup instructions.

## Using Edit Prediction

Once you have configured an Edit Prediction provider, you can start using edit predictions in your code. Edit predictions will appear as you type, and you can accept them by pressing `tab` or `enter` or hide them by pressing `esc`.

There a number of actions/shortcuts available to interact with edit predictions:

- `editor: accept edit prediction` (`tab`): To accept the current edit prediction
- `editor: accept partial inline completion` (`cmd-right`): To accept the current inline completion up to the next word boundary
- `editor: show inline completion` (`alt-\`): Trigger a inline completion request manually
- `editor: next inline completion` (`alt-]`): To cycle to the next inline completion
- `editor: previous inline completion` (`alt-[`): To cycle to the previous inline completion

### Disabling Inline-Completions

To disable completions that appear automatically as you type, add the following to your `settings.json`:

```json
{
  "show_inline_completions": false
}
```

You can trigger edit predictions manually by executing `editor: show edit prediction` (`alt-\\`).

You can also add this as a language-specific setting in your `settings.json` to disable edit predictions for a specific language:

```json
{
  "language": {
    "python": {
      "show_inline_completions": false
    }
  }
}
```

## See also

You may also use the Assistant Panel or the Inline Assistant to interact with language models, see the [assistant](assistant/assistant.md) documentation for more information.
