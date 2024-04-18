# Tooling

Bringing OpenAI compatible tool calling to Rust.

- Structured Extraction
- Validation and Healing
- Execution

## Overview

Language Models can produce structured outputs that are perfect for calling functions. The most famous of these is OpenAI's tool calling. When you call out to make a chat completion you can pass a list of tools available to the model. The model will choose `0..n` tools to help them complete a user's task.

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
use futures::Future;
use schemars::JsonSchema;
use serde::Deserialize;

use assistant_tooling::LanguageModelTool;
```

Then we'll define the query structure the model must fill in. This _must_ derive `Deserialize` from `serde` and `JsonSchema` from the `schemars` crate.

```rust
#[derive(Deserialize, Serialize, JsonSchema)]
struct CodebaseQuery {
    query: String,
}
```

After that we can define our tool, its input type (`CodebaseQuery`) and its output. We'll make a fake `ProjectIndex` for now just to make an illustrative example.

```rust
struct ProjectIndex {}

impl ProjectIndex {
    fn new() -> Self {
        ProjectIndex {}
    }

    fn search(&self, _query: &str, _limit: usize) -> impl Future<Output = Result<Vec<String>>> {
        async move { Ok(vec![]) }
    }
}


struct ProjectIndexTool {
    project_index: ProjectIndex,
}

impl LanguageModelTool for ProjectIndexTool {
    type Input = CodebaseQuery;
    type Output = String;

    fn name(&self) -> String {
        "query_codebase".to_string()
    }

    fn description(&self) -> String {
        "Executes a query against the codebase, returning excerpts related to the query".to_string()
    }

    fn execute(&self, query: Self::Input) -> impl 'static + Future<Output = Result<Self::Output>> {
        let results = self.project_index.search(query.query.as_str(), 10);

        async move {
            let results = results.await?;

            if !results.is_empty() {
                Ok(results.join("\n"))
            } else {
                Ok(format!("No results"))
            }
        }
    }
}
```

The model will use the `name` and `description` to determine which tool, amongst potentially several, to use to help complete tasks. Once that's determined, it's up to us to run the code based on the `Input` passed in, using that `execute`.

As an example, we'll set up a `ToolRegistry` and register our new tool.

```rust
let tool = ProjectIndexTool {
    project_index: ProjectIndex::new(),
};

let mut registry = ToolRegistry::new();

let registered = registry.register(tool);
assert!(registered.is_ok());
```

When OpenAI says it wants to use one of our tools it will pass us an object with `name` and `arguments`. We pass those into `registry.call` which gives us a future.

```rust
let task = registry.call(
    "query_codebase",
    r#"{"query":"GPUI Task background_executor"}"#,
);
```

Given that we're targeting GPUI, I wanted to leave this open to different kinds of executors. Here's it being consumed by the `futures::executor`:

```rust
let result = futures::executor::block_on(task);
println!("{}", result.unwrap())
```
