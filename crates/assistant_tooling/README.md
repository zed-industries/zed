# Tooling

Bringing OpenAI compatible tool calling to Rust.

- Structured Extraction
- Validation and Healing
- Execution

## Overview

The main way to think about this package is in _patterns_. Models absolutely will fail to meet the schema passed to them, so we want to be able to give that feedback to the model directly, to allow it to "self-heal".

Let's start with an example using a semantic index. This has a few parts that are fairly hairy.

We're working on a semantic index that we want to expose to a model.

We want the model to be able to query a semantic index directly.

```rust
use anyhow::Result;
use assistant_tooling::LanguageModelTool;
use futures::Future;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
struct CodebaseQuery {
    query: String,
}

#[derive(Serialize)]
struct CodebaseQueryResult {
    excerpts: Vec<String>,
}

struct ProjectIndexTool {
    project_index: ProjectIndex
}

impl LanguageModelTool for ProjectIndexTool {
    type Input = CodebaseQuery;
    type Output = CodebaseQueryResult;

    fn name(&self) -> String {
        "query_codebase".to_string()
    }

    fn description(&self) -> String {
        "Executes a query against the codebase, returning excerpts related to the query".to_string()
    }

    fn execute(&self, query: Self::Input) -> impl 'static + Future<Output = Result<Self::Output>> {
        let query = query.query.clone();

        async move {
            let results = self.project_index.search(query, 10).await
            Ok(format!("No results for query: '{}'", query))
        }
    }
}

fn main() {
    let tool = ProjectIndexTool {};

    let query = CodebaseQuery {
        query: "how do i GPUI".to_string(),
    };

    let task = tool.execute(query);
    let result = futures::executor::block_on(task);
    println!("{}", result.unwrap());
}
```
