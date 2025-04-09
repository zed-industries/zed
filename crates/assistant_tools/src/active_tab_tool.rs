use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::IconName;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ActiveTabToolInput {}

pub struct ActiveTabTool;

impl Tool for ActiveTabTool {
    fn name(&self) -> String {
        "active-tab".into()
    }

    fn needs_confirmation(&self) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./active_tab_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::Eye
    }

    fn input_schema(&self, _format: LanguageModelToolSchemaFormat) -> serde_json::Value {
        let schema = schemars::schema_for!(ActiveTabToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn ui_text(&self, _input: &serde_json::Value) -> String {
        "Get active tabs".to_string()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        _project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        _cx: &mut App,
    ) -> Task<Result<String>> {
        let _input = match serde_json::from_value::<ActiveTabToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        Task::ready(Ok("Active tab information would be shown here".to_string()))
    }
}
