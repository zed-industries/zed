use anyhow::Result;
use assistant_tooling::{LanguageModelTool, ToolRegistry};
use gpui::{App, AppContext, Task};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, JsonSchema)]
struct CodebaseQuery {
    query: String,
}

struct ProjectIndex {}

impl ProjectIndex {
    fn new() -> Self {
        ProjectIndex {}
    }

    fn search(&self, _query: &str, _limit: usize, cx: &AppContext) -> Task<Result<Vec<String>>> {
        cx.spawn(|_cx| async {
            Ok(vec![r#"// crates/gpui/src/gpui.rs
    //! # Welcome to GPUI!
    //!
    //! GPUI is a hybrid immediate and retained mode, GPU accelerated, UI framework
    //! for Rust, designed to support a wide variety of applications
    "#
            .to_string()])
        })
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

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let tool = ProjectIndexTool {
            project_index: ProjectIndex::new(),
        };

        let mut registry = ToolRegistry::new();
        let registered = registry.register(tool);
        assert!(registered.is_ok());

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

        // We know there are tool_calls, so let's skip straight to it for this example
        let tool_calls = message.tool_calls.as_ref().unwrap();

        // We need to create a group of tasks that we can join on
        let mut tasks = Vec::new();

        for tool_call in tool_calls.iter() {
            match tool_call {
                ToolCall::Function { function, .. } => {
                    let task = registry.call(&function.name, &function.args, cx);

                    tasks.push(task);
                }
                _ => {}
            }
        }

        cx.spawn(|_cx| async move {
            let results = futures::future::join_all(tasks).await;

            for result in results {
                match result {
                    Ok(result) => {
                        println!("{}", result);
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                    }
                }
            }

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        cx.quit();
    });
}
