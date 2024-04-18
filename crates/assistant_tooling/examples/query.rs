use anyhow::Result;
use assistant_tooling::LanguageModelTool;
use futures::Future;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
struct CodebaseQuery {
    query: String,
}

struct ProjectIndexTool {
    // semantic_index: SemanticIndex
}

impl LanguageModelTool for ProjectIndexTool {
    type Input = CodebaseQuery;
    type Output = String;

    fn name(&self) -> String {
        "query_codebase".to_string()
    }

    fn description(&self) -> String {
        "Executes a query against the codebase, returning structured information.".to_string()
    }

    fn execute(&self, query: Self::Input) -> impl 'static + Future<Output = Result<Self::Output>> {
        let query = query.query.clone();

        async move {
            // Placeholder until semantic index hooked up
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
