# Inline Assistant

## Using the Inline Assistant

You can use `ctrl-enter` to open the Inline Assistant nearly anywhere you can enter text: editors, the agent panel, the prompt library, channel notes, and even within the terminal panel.

The Inline Assistant allows you to send the current selection (or the current line) to a language model and modify the selection with the language model's response.

You can use `ctrl-enter` to open the inline assistant nearly anywhere you can write text: editors, the Agent Panel, the Rules Library, channel notes, and even within the terminal panel.

You can also perform multiple generation requests in parallel by pressing `ctrl-enter` with multiple cursors, or by pressing `ctrl-enter` with a selection that spans multiple excerpts in a multibuffer.

## Context

You can give the Inline Assistant context the same way you can in the agent panel, allowing you to provide additional instructions or rules for code transformations with @-mentions.

A useful pattern here is to create a thread in the [Agent Panel](./agent-panel.md), and then use the `@thread` command in the Inline Assistant to include the thread as context for the Inline Assistant transformation.

## Prefilling Prompts

To create a custom keybinding that prefills a prompt, you can add the following format in your keymap:

```json
[
  {
    "context": "Editor && mode == full",
    "bindings": {
      "ctrl-shift-enter": [
        "assistant::InlineAssist",
        { "prompt": "Build a snake game" }
      ]
    }
  }
]
```
