# Completions

CodeOrbit supports two sources for completions:

1. "Code Completions" provided by Language Servers (LSPs) automatically installed by CodeOrbit or via [CodeOrbit Language Extensions](languages.md).
2. "Edit Predictions" provided by CodeOrbit's own Zeta model or by external providers like [GitHub Copilot](#github-copilot) or [Supermaven](#supermaven).

## Language Server Code Completions {#code-completions}

When there is an appropriate language server available, CodeOrbit will provide completions of variable names, functions, and other symbols in the current file. You can disable these by adding the following to your CodeOrbit `settings.json` file:

```json
"show_completions_on_input": false
```

You can manually trigger completions with `ctrl-space` or by triggering the `editor::ShowCompletions` action from the command palette.

For more information, see:

- [Configuring Supported Languages](./configuring-languages.md)
- [List of CodeOrbit Supported Languages](./languages.md)

## Edit Predictions {#edit-predictions}

CodeOrbit has built-in support for predicting multiple edits at a time [via Zeta](https://huggingface.co/CodeOrbit-industries/zeta), CodeOrbit's open-source and open-data model.
Edit predictions appear as you type, and most of the time, you can accept them by pressing `tab`.

See the [edit predictions documentation](./ai/edit-prediction.md) for more information on how to setup and configure CodeOrbit's edit predictions.
