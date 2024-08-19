## Inline Assistant

You can use `ctrl-enter` to open the inline assistant in both a normal editor, within the assistant panel, and even within the terminal panel.

The inline assistant allows you to send the current selection (or the current line) to a language model and modify the selection with the language model's response. You can also perform multiple generation requests in parallel by pressing `ctrl-enter` with multiple cursors, or by pressing `ctrl-enter` with a selection that spans multiple excerpts in a multibuffer.

The inline assistant pulls its context from the assistant panel, allowing you to provide additional instructions or rules for code transformations.

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
