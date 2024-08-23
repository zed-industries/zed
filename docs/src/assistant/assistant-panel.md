# Assistant Panel

The assistant panel provides you with a way to interact with large language models. The assistant is useful for various tasks, such as generating code, asking questions about existing code, and even writing plaintext, such as emails and documentation.

To open the assistant panel, toggle the right dock by using the {#action workspace::ToggleRightDock} action in the command palette or by using the
{#kb workspace::ToggleRightDock} shortcut.

> **Note**: A custom [key binding](../key-bindings.md) can be set to toggle the right dock.

Once you have [configured a provider](./configuration.md#providers), you can interact with the provider's language models in a context editor.

![](https://private-user-images.githubusercontent.com/1714999/359287532-abd8f918-e65f-44ce-a853-1e90f852e206.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MjQxMDk2OTIsIm5iZiI6MTcyNDEwOTM5MiwicGF0aCI6Ii8xNzE0OTk5LzM1OTI4NzUzMi1hYmQ4ZjkxOC1lNjVmLTQ0Y2UtYTg1My0xZTkwZjg1MmUyMDYucG5nP1gtQW16LUFsZ29yaXRobT1BV1M0LUhNQUMtU0hBMjU2JlgtQW16LUNyZWRlbnRpYWw9QUtJQVZDT0RZTFNBNTNQUUs0WkElMkYyMDI0MDgxOSUyRnVzLWVhc3QtMSUyRnMzJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNDA4MTlUMjMxNjMyWiZYLUFtei1FeHBpcmVzPTMwMCZYLUFtei1TaWduYXR1cmU9MDhlMjZhMjI0NjM3M2JiZmEzMWU5ZWIwYWRjZjhkNTI3NTkyM2JlNmNjODcyMjg3YjkxNjIxNmI5ZTk1ZWRjZCZYLUFtei1TaWduZWRIZWFkZXJzPWhvc3QmYWN0b3JfaWQ9MCZrZXlfaWQ9MCZyZXBvX2lkPTAifQ.NiiQkF65VvBKCJs_zNxmjpyvKGK6Hw1aIWA3Xc87XRs)

To create a new context editor, press {#kb workspace::NewFile} or use the menu in the top right of the assistant panel and select the `New Context` option.

In the context editor, select a model from one of the configured providers, type a message in the `You` block, and submit with {#kb assistant::Assist}.

### Interacting with the Assistant

The context editor in Zed functions similarly to any other editor. You can use custom key bindings and work with multiple cursors, allowing for seamless transitions between coding and engaging in discussions with the language models.

However, the assistant editor differs with the inclusion of message blocks. These blocks serve as containers for text that correspond to different roles within the context. These roles include:

- `You`
- `Assistant`
- `System`

To begin, select a model and type a message in a `You` block.

![Asking a question](https://zed.dev/img/assistant/ask-a-question.png)

As you type, the remaining tokens count for the selected model is updated.

Inserting text from an editor is as simple as highlighting the text and running `assistant: quote selection` ({#kb assistant::QuoteSelection}); Zed will wrap it in a fenced code block if it is code.

![Quoting a selection](https://zed.dev/img/assistant/quoting-a-selection.png)

To submit a message, use {#kb assistant::Assist}(`assistant: assist`). Unlike typical chat applications where pressing <kbd>enter</kbd> would submit the message, in the assistant editor, our goal was to make it feel as close to a regular editor as possible. So, pressing {#kb editor::Newline} simply inserts a new line.

After submitting a message, the assistant's response will be streamed below, in an `Assistant` message block.

![Receiving an answer](https://zed.dev/img/assistant/receiving-an-answer.png)

The stream can be canceled at any point with <kbd>escape</kbd>. This is useful if you realize early on that the response is not what you were looking for.

If you want to start a new conversation at any time, you can hit <kbd>cmd-n|ctrl-n</kbd> or use the `New Context` menu option in the hamburger menu at the top left of the panel.

Simple back-and-forth conversations work well with the assistant. However, there may come a time when you want to modify the previous text in the conversation and steer it in a different direction.

### Editing a Context

> **Note**: Wondering about Context vs Conversation? [Read more here](./contexts.md).

The assistant gives you the flexibility to have control over the context. You can freely edit any previous text, including the responses from the assistant. If you want to remove a message block entirely, simply place your cursor at the beginning of the block and use the `delete` key. A typical workflow might involve making edits and adjustments throughout the context to refine your inquiry or provide additional information. Here's an example:

1. Write text in a `You` block.
2. Submit the message with {#kb assistant::Assist}.
3. Receive an `Assistant` response that doesn't meet your expectations.
4. Cancel the response with <kbd>escape</kbd>.
5. Erase the content of the `Assistant` message block and remove the block entirely.
6. Add additional context to your original message.
7. Submit the message with {#kb assistant::Assist}.

Being able to edit previous messages gives you control over how tokens are used. You don't need to start up a new context to correct a mistake or to add additional information, and you don't have to waste tokens by submitting follow-up corrections.

> **Note**: The act of editing past messages is often referred to as "Rewriting History" in the context of the language models.

Some additional points to keep in mind:

- You are free to change the model type at any point in the conversation.
- You can cycle the role of a message block by clicking on the role, which is useful when you receive a response in an `Assistant` block that you want to edit and send back up as a `You` block.
