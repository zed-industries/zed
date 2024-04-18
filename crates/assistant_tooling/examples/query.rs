use anyhow::Result;
use assistant_tooling::{LanguageModelTool, ToolRegistry};
use futures::Future;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
struct CodebaseQuery {
    query: String,
}

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
                Ok("No results".to_string())
            }
        }
    }
}

fn main() {
    let tool = ProjectIndexTool {
        project_index: ProjectIndex::new(),
    };

    let mut registry = ToolRegistry::new();
    let registered = registry.register(tool);
    assert!(registered.is_ok());

    // This is where OpenAI would request a tool_call using a name and arguments
    let task = registry.call(
        "query_codebase",
        r#"{"query":"GPUI Task background_executor"}"#,
    );

    let result = futures::executor::block_on(task);
    println!("{}", result.unwrap());
}
