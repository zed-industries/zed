use assistant_tooling::{FunctionCall, FunctionCallTask};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
struct CodebaseQuery {
    query: String,
}

impl FunctionCall for CodebaseQuery {
    type Output = String;

    fn name() -> &'static str {
        "query_codebase"
    }

    fn description() -> &'static str {
        "Executes a query against the codebase, returning structured information."
    }

    fn execute(&self) -> FunctionCallTask<String> {
        let query = self.query.clone();

        Box::pin(async move {
            // Placeholder until semantic index hooked up
            Ok(format!("Results for query: '{}'", query))
        })
    }
}

fn main() {
    let query = CodebaseQuery {
        query: "how do i GPUI".to_string(),
    };

    let task = query.execute();
    let result = futures::executor::block_on(task);
    println!("{}", result.unwrap());
}
