# Inline Assistant

## Using the Inline Assistant

You can use `ctrl-enter` to open the inline assistant nearly anywhere you can enter text: Editors, the assistant panel, the prompt library, channel notes, and even within the terminal panel.

The inline assistant allows you to send the current selection (or the current line) to a language model and modify the selection with the language model's response.

You can also perform multiple generation requests in parallel by pressing `ctrl-enter` with multiple cursors, or by pressing `ctrl-enter` with a selection that spans multiple excerpts in a multibuffer.

The inline assistant pulls its context from the assistant panel, allowing you to provide additional instructions or rules for code transformations.

> **Note**: The inline assistant sees the entire active context from the assistant panel. This means the assistant panel's context editor becomes one of the most powerful tools for shaping the results of the inline assistant.

## Using Prompts & Commands

While you can't directly use slash commands (and by extension, the `/prompt` command to include prompts) in the inline assistant, you can use them in the active context in the assistant panel.

A common workflow when using the inline assistant is to create a context in the assistant panel, add the desired context through text, prompts and commands, and then use the inline assistant to generate and apply transformations.

### Example Recipe - Fixing Errors with the Inline Assistant

1. Create a new chat in the assistant panel.
2. Use the `/diagnostic` command to add current diagnostics to the context.
3. OR use the `/terminal` command to add the current terminal output to the context (maybe a panic, error, or log?)
4. Use the inline assistant to generate a fix for the error.

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
