# Inline Assistant

## Usage Overview

Use `ctrl-enter` to open the Inline Assistant nearly anywhere you can enter text: editors, text threads, the rules library, channel notes, and even within the terminal panel.

The Inline Assistant allows you to send the current selection (or the current line) to a language model and modify the selection with the language model's response.

You can also perform multiple generation requests in parallel by pressing `ctrl-enter` with multiple cursors, or by pressing the same binding with a selection that spans multiple excerpts in a multibuffer.

## Context

Give the Inline Assistant context the same way you can in [the Agent Panel](./agent-panel.md), allowing you to provide additional instructions or rules for code transformations with @-mentions.

A useful pattern here is to create a thread in the Agent Panel, and then use the mention that thread with `@thread` in the Inline Assistant to include it as context.

> The Inline Assistant is limited to normal mode context windows ([see Models](./models.md) for more).

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
