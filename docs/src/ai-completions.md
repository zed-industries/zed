# AI (LLM) Code Completions

TBD: Document AI code completions. Mention how to they differ from [LSP completions](./language-settings.md).

## Configuration

To disable completions that appear automatically as you type, add the following to your `settings.json`:

```json
{
  "show_inline_completions": false
}
```

You can trigger inline completions manually by executing `editor: show inline completion` (`alt-\\`).

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

## Inline completions

There a number of actions/shortcuts available to interact with inline completions:

- `editor: accept inline completion` (`tab`): To accept the current inline completion
- `editor: accept partial inline completion` (`cmd-right`): To accept the current inline completion up to the next word boundary
- `editor: show inline completion` (`alt-\\`): Trigger a inline completion request manually
- `editor: next inline completion` (`alt-]`): To cycle to the next inline completion
- `editor: previous inline completion` (`alt-[`): To cycle to the previous inline completion

## Assistant Panel

You may also use the Assistant Panel or the Inline Assistant to interact with language models, see [Language Model Integration](language-model-integration.md) documentation for more information.
