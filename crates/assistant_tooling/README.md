# Assistant Tooling

Bringing Language Model tool calling to GPUI.

This unlocks:

- **Structured Extraction** of model responses
- **Validation** of model inputs
- **Execution** of chosen tools

## Overview

Language Models can produce structured outputs that are perfect for calling functions. The most famous of these is OpenAI's tool calling. When making a chat completion you can pass a list of tools available to the model. The model will choose `0..n` tools to help them complete a user's task. It's up to _you_ to create the tools that the model can call.

> **User**: "Hey I need help with implementing a collapsible panel in GPUI"
>
> **Assistant**: "Sure, I can help with that. Let me see what I can find."
>
> `tool_calls: ["name": "query_codebase", arguments: "{ 'query': 'GPUI collapsible panel' }"]`
>
> `result: "['crates/gpui/src/panel.rs:12: impl Panel { ... }', 'crates/gpui/src/panel.rs:20: impl Panel { ... }']"`
>
> **Assistant**: "Here are some excerpts from the GPUI codebase that might help you."

This library is designed to facilitate this interaction mode by allowing you to go from `struct` to `tool` with two simple traits, `LanguageModelTool` and `ToolView`.

## Using the Tool Registry

```rust
let mut tool_registry = ToolRegistry::new();
tool_registry
    .register(WeatherTool { api_client },
    })
    .unwrap(); // You can only register one tool per name

let completion = cx.update(|cx| {
    CompletionProvider::get(cx).complete(
        model_name,
        messages,
        Vec::new(),
        1.0,
        // The definitions get passed directly to OpenAI when you want
        // the model to be able to call your tool
        tool_registry.definitions(),
    )
});

let mut stream = completion?.await?;

let mut message = AssistantMessage::new();

while let Some(delta) = stream.next().await {
    // As messages stream in, you'll get both assistant content
    if let Some(content) = &delta.content {
        message
            .body
            .update(cx, |message, cx| message.append(&content, cx));
    }

    // And tool calls!
    for tool_call_delta in delta.tool_calls {
        let index = tool_call_delta.index as usize;
        if index >= message.tool_calls.len() {
            message.tool_calls.resize_with(index + 1, Default::default);
        }
        let tool_call = &mut message.tool_calls[index];

        // Build up an ID
        if let Some(id) = &tool_call_delta.id {
            tool_call.id.push_str(id);
        }

        tool_registry.update_tool_call(
            tool_call,
            tool_call_delta.name.as_deref(),
            tool_call_delta.arguments.as_deref(),
            cx,
        );
    }
}
```

Once the stream of tokens is complete, you can exexute the tool call by calling `tool_registry.execute_tool_call(tool_call, cx)`, which returns a `Task<Result<()>>`.

As the tokens stream in and tool calls are executed, your `ToolView` will get updates. Render each tool call by passing that `tool_call` in to `tool_registry.render_tool_call(tool_call, cx)`. The final message for the model can be pulled by calling `self.tool_registry.content_for_tool_call( tool_call, &mut project_context, cx, )`.
