use gpui::SharedString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use util::schemars::{AllowTrailingCommas, DefaultDenyUnknownFields};

use crate::TaskTemplate;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum WorktreeTaskDefinition {
    ByName(SharedString),
    Template {
        #[serde(flatten)]
        task_template: TaskTemplate,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct WorktreeTasks {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub setup: Vec<WorktreeTaskDefinition>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub teardown: Vec<WorktreeTaskDefinition>,
}

impl WorktreeTasks {
    pub fn is_empty(&self) -> bool {
        self.setup.is_empty() && self.teardown.is_empty()
    }

    pub fn generate_json_schema() -> serde_json::Value {
        let schema = schemars::generate::SchemaSettings::draft2019_09()
            .with_transform(DefaultDenyUnknownFields)
            .with_transform(AllowTrailingCommas)
            .into_generator()
            .root_schema_for::<Self>();

        serde_json::to_value(schema).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{WorktreeTaskDefinition, WorktreeTasks};

    #[test]
    fn test_worktree_task_definition_by_name() {
        let definition: WorktreeTaskDefinition = serde_json::from_str(r#""bootstrap""#).unwrap();

        match definition {
            WorktreeTaskDefinition::ByName(name) => assert_eq!(name.as_ref(), "bootstrap"),
            WorktreeTaskDefinition::Template { .. } => panic!("expected task name"),
        }
    }

    #[test]
    fn test_worktree_task_definition_template() {
        let definition: WorktreeTaskDefinition = serde_json::from_str(
            r#"{
                "label": "Install dependencies",
                "command": "pnpm",
                "args": ["install"]
            }"#,
        )
        .unwrap();

        match definition {
            WorktreeTaskDefinition::Template { task_template } => {
                assert_eq!(task_template.label, "Install dependencies");
                assert_eq!(task_template.command, "pnpm");
                assert_eq!(task_template.args, vec!["install"]);
            }
            WorktreeTaskDefinition::ByName(_) => panic!("expected inline task"),
        }
    }

    #[test]
    fn test_worktree_tasks_deserialization() {
        let tasks: WorktreeTasks = serde_json::from_str(
            r#"{
                "setup": [
                    "bootstrap",
                    {
                        "label": "Install dependencies",
                        "command": "pnpm",
                        "args": ["install"]
                    }
                ],
                "teardown": ["cleanup"]
            }"#,
        )
        .unwrap();

        assert_eq!(tasks.setup.len(), 2);
        assert_eq!(tasks.teardown.len(), 1);

        match &tasks.setup[0] {
            WorktreeTaskDefinition::ByName(name) => assert_eq!(name.as_ref(), "bootstrap"),
            WorktreeTaskDefinition::Template { .. } => panic!("expected task name"),
        }

        match &tasks.setup[1] {
            WorktreeTaskDefinition::Template { task_template } => {
                assert_eq!(task_template.label, "Install dependencies");
                assert_eq!(task_template.command, "pnpm");
                assert_eq!(task_template.args, vec!["install"]);
            }
            WorktreeTaskDefinition::ByName(_) => panic!("expected inline task"),
        }

        match &tasks.teardown[0] {
            WorktreeTaskDefinition::ByName(name) => assert_eq!(name.as_ref(), "cleanup"),
            WorktreeTaskDefinition::Template { .. } => panic!("expected task name"),
        }
    }

    #[test]
    fn test_worktree_tasks_default_to_empty_sections() {
        let tasks: WorktreeTasks = serde_json::from_str("{}").unwrap();

        assert!(tasks.setup.is_empty());
        assert!(tasks.teardown.is_empty());
        assert!(tasks.is_empty());
    }

    #[test]
    fn test_worktree_tasks_omit_empty_sections_when_serializing() {
        let tasks = WorktreeTasks::default();

        assert_eq!(serde_json::to_value(tasks).unwrap(), json!({}));
    }
}
