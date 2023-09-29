use ai::function_calling::OpenAIFunction;
use anyhow::anyhow;
use gpui::{AppContext, AsyncAppContext, ModelHandle};
use project::Project;
use serde::{Serialize, Serializer};
use serde_json::json;
use std::fmt::Write;

use crate::SemanticIndex;

#[derive(Clone)]
pub struct RepositoryContextRetriever {
    index: ModelHandle<SemanticIndex>,
    project: ModelHandle<Project>,
}

impl RepositoryContextRetriever {
    pub fn load(index: ModelHandle<SemanticIndex>, project: ModelHandle<Project>) -> Self {
        Self { index, project }
    }
    pub async fn complete_test(
        &self,
        arguments: serde_json::Value,
        cx: &mut AsyncAppContext,
    ) -> anyhow::Result<String> {
        let queries = arguments.get("queries").unwrap().as_array().unwrap();
        let mut prompt = String::new();
        let query = queries
            .iter()
            .map(|query| query.to_string())
            .collect::<Vec<String>>()
            .join(";");
        let project = self.project.clone();
        let results = self
            .index
            .update(cx, |this, cx| {
                this.search_project(project, query, 10, vec![], vec![], cx)
            })
            .await?;

        for result in results {
            result.buffer.read_with(cx, |buffer, cx| {
                let text = buffer.text_for_range(result.range).collect::<String>();
                let file_path = buffer.file().unwrap().path().to_string_lossy();
                let language = buffer.language();

                writeln!(
                    prompt,
                    "The following is a relevant snippet from file ({}):",
                    file_path
                )
                .unwrap();
                if let Some(language) = language {
                    writeln!(prompt, "```{}\n{text}\n```", language.name().to_lowercase()).unwrap();
                } else {
                    writeln!(prompt, "```\n{text}\n```").unwrap();
                }
            });
        }

        Ok(prompt)
    }
}

impl OpenAIFunction for RepositoryContextRetriever {
    fn name(&self) -> String {
        "retrieve_context_from_repository".to_string()
    }
    fn description(&self) -> String {
        "Retrieve relevant content from repository with natural language".to_string()
    }
    fn system_prompt(&self) -> String {
        "'retrieve_context_from_repository'
                If more information is needed from the repository, to complete the users prompt reliably, pass up to 3 queries describing pieces of code or text you would like additional context upon.
                Do not make these queries general about programming, include very specific lexical references to the pieces of code you need more information on.
                We are passing these into a semantic similarity retrieval engine, with all the information in the current codebase included.
                As such, these should be phrased as descriptions of code of interest as opposed to questions".to_string()
    }
    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "queries": {
                    "title": "queries",
                    "type": "array",
                    "items": {"type": "string"}
                }
            },
            "required": ["queries"]
        })
    }
    fn complete(&self, arguments: serde_json::Value) -> anyhow::Result<String> {
        todo!();
    }
}
impl Serialize for RepositoryContextRetriever {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        json!({"name": self.name(),
            "description": self.description(),
            "parameters": self.parameters()})
        .serialize(serializer)
    }
}
