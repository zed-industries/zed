# Assistant Tooling

Bringing OpenAI compatible tool calling to GPUI.

This unlocks:

- **Structured Extraction** of model responses
- **Validation** of model inputs
- **Execution** of chosen toolsn

## Overview

Language Models can produce structured outputs that are perfect for calling functions. The most famous of these is OpenAI's tool calling. When make a chat completion you can pass a list of tools available to the model. The model will choose `0..n` tools to help them complete a user's task. It's up to _you_ to create the tools that the model can call.

> **User**: "Hey I need help with implementing a collapsible panel in GPUI"
>
> **Assistant**: "Sure, I can help with that. Let me see what I can find."
>
> `tool_calls: ["name": "query_codebase", arguments: "{ 'query': 'GPUI collapsible panel' }"]`
>
> `result: "['crates/gpui/src/panel.rs:12: impl Panel { ... }', 'crates/gpui/src/panel.rs:20: impl Panel { ... }']"`
>
> **Assistant**: "Here are some excerpts from the GPUI codebase that might help you."

This library is designed to facilitate this interaction mode by allowing you to go from `struct` to `tool` with a simple trait, `LanguageModelTool`.

## Example

Let's expose querying a semantic index directly by the model. First, we'll set up some _necessary_ imports

```rust
use anyhow::Result;
use assistant_tooling::{LanguageModelTool, ToolRegistry};
use gpui::{App, AppContext, Task};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::json;
```

Then we'll define the query structure the model must fill in. This _must_ derive `Deserialize` from `serde` and `JsonSchema` from the `schemars` crate.

```rust
#[derive(Deserialize, JsonSchema)]
struct CodebaseQuery {
    query: String,
}
```

After that we can define our tool, with the expectation that it will need a `ProjectIndex` to search against. For this example, the index uses the same interface as `semantic_index::ProjectIndex`.

```rust
struct ProjectIndex {}

impl ProjectIndex {
    fn new() -> Self {
        ProjectIndex {}
    }

    fn search(&self, _query: &str, _limit: usize, _cx: &AppContext) -> Task<Result<Vec<String>>> {
        // Instead of hooking up a real index, we're going to fake it
        if _query.contains("gpui") {
            return Task::ready(Ok(vec![r#"// crates/gpui/src/gpui.rs
    //! # Welcome to GPUI!
    //!
    //! GPUI is a hybrid immediate and retained mode, GPU accelerated, UI framework
    //! for Rust, designed to support a wide variety of applications
    "#
            .to_string()]));
        }
        return Task::ready(Ok(vec![]));
    }
}

struct ProjectIndexTool {
    project_index: ProjectIndex,
}
```

Now we can implement the `LanguageModelTool` trait for our tool by:

- Defining the `Input` from the model, which is `CodebaseQuery`
- Defining the `Output`
- Implementing the `name` and `description` functions to provide the model information when it's choosing a tool
- Implementing the `execute` function to run the tool

```rust
impl LanguageModelTool for ProjectIndexTool {
    type Input = CodebaseQuery;
    type Output = String;

    fn name(&self) -> String {
        "query_codebase".to_string()
    }

    fn description(&self) -> String {
        "Executes a query against the codebase, returning excerpts related to the query".to_string()
    }

    fn execute(&self, query: Self::Input, cx: &AppContext) -> Task<Result<Self::Output>> {
        let results = self.project_index.search(query.query.as_str(), 10, cx);

        cx.spawn(|_cx| async move {
            let results = results.await?;

            if !results.is_empty() {
                Ok(results.join("\n"))
            } else {
                Ok("No results".to_string())
            }
        })
    }
}
```

For the sake of this example, let's look at the types that OpenAI will be passing to us

```rust
// OpenAI definitions, shown here for demonstration
#[derive(Deserialize)]
struct FunctionCall {
    name: String,
    args: String,
}

#[derive(Deserialize, Eq, PartialEq)]
enum ToolCallType {
    #[serde(rename = "function")]
    Function,
    Other,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct ToolCallId(String);

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ToolCall {
    Function {
        #[allow(dead_code)]
        id: ToolCallId,
        function: FunctionCall,
    },
    Other {
        #[allow(dead_code)]
        id: ToolCallId,
    },
}

#[derive(Deserialize)]
struct AssistantMessage {
    role: String,
    content: Option<String>,
    tool_calls: Option<Vec<ToolCall>>,
}
```

When the model wants to call tools, it will pass a list of `ToolCall`s. When those are `function`s that we can handle, we'll pass them to our `ToolRegistry` to get a future that we can await.

```rust
// Inside `fn main()`
App::new().run(|cx: &mut AppContext| {
    let tool = ProjectIndexTool {
        project_index: ProjectIndex::new(),
    };

    let mut registry = ToolRegistry::new();
    let registered = registry.register(tool);
    assert!(registered.is_ok());
```

Let's pretend the model sent us back a message requesting

```rust
let model_response = json!({
    "role": "assistant",
    "tool_calls": [
        {
            "id": "call_1",
            "function": {
                "name": "query_codebase",
                "args": r#"{"query":"GPUI Task background_executor"}"#
            },
            "type": "function"
        }
    ]
});

let message: AssistantMessage = serde_json::from_value(model_response).unwrap();

// We know there's a tool call, so let's skip straight to it for this example
let tool_calls = message.tool_calls.as_ref().unwrap();
let tool_call = tool_calls.get(0).unwrap();
```

We can now use our registry to call the tool.

```rust
let task = registry.call(
    tool_call.name,
    tool_call.args,
);

cx.spawn(|_cx| async move {
    let result = task.await?;
    println!("{}", result.unwrap());
    Ok(())
})
```
