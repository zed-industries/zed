use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_tool::Tool;
use gpui::{Task, WeakView, WindowContext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CodeEditsToolInput {
    /// A high-level description of the code changes. This should be as short as possible, possibly using common abbreviations.
    pub title: String,
    /// An array of edits to be applied.
    pub edits: Vec<Edit>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Edit {
    /// The path to the file that this edit will change.
    pub path: String,
    /// An arbitrarily-long comment that describes the purpose of this edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An excerpt from the file's current contents that uniquely identifies a range within the file where the edit should occur.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_text: Option<String>,
    /// The new text to insert into the file.
    pub new_text: String,
    /// The type of change that should occur at the given range of the file.
    pub operation: Operation,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    /// Replaces the entire range with the new text.
    Update,
    /// Inserts the new text before the range.
    InsertBefore,
    /// Inserts new text after the range.
    InsertAfter,
    /// Creates a new file with the given path and the new text.
    Create,
    /// Deletes the specified range from the file.
    Delete,
}

pub struct CodeEditsTool;

impl CodeEditsTool {
    pub const TOOL_NAME: &str = "zed_code_edits";
}

impl Tool for CodeEditsTool {
    fn name(&self) -> String {
        Self::TOOL_NAME.to_string()
    }

    fn description(&self) -> String {
        // Anthropic's best practices for tool descriptions:
        // https://docs.anthropic.com/en/docs/build-with-claude/tool-use#best-practices-for-tool-definitions
        include_str!("edit_tool_description.txt").to_string()
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(CodeEditsToolInput);

        serde_json::to_value(&schema).unwrap()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _workspace: WeakView<workspace::Workspace>,
        _cx: &mut WindowContext,
    ) -> Task<Result<String>> {
        let input: CodeEditsToolInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        let text = format!("The tool returned {:?}.", input);

        Task::ready(Ok(text))
    }
}
