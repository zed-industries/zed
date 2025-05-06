# Inline Assistant

## Overview

The inline assistant allows you to send the current selection (or the current line) to a language model and modify the selection with the language model's response.

You can use `ctrl-enter` to open the inline assistant nearly anywhere you can write text: editors, the Agent Panel, the Rules Library, channel notes, and even within the terminal panel.

You can also perform multiple generation requests in parallel by pressing `ctrl-enter` with multiple cursors, or by pressing `ctrl-enter` with a selection that spans multiple excerpts in a multibuffer.

## Context

You can give the inline assistant context the same way you can in the Agent Panel, via @-mentioning files or adding them via the context picker.
That allows you to provide additional instructions or rules for code transformations.

A useful pattern here is to create a thread in the Agent Panel, and then mention it by typing `@thread`.
That includes the thread as context, and it stays there until you remove it.

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
