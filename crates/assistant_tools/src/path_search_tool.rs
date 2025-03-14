use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use util::paths::PathMatcher;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PathSearchToolInput {
    /// The glob to search all project paths for.
    ///
    /// <example>
    /// If the project has the following root directories:
    ///
    /// - directory1/a/something.txt
    /// - directory2/a/things.txt
    /// - directory3/a/other.txt
    ///
    /// You can get back the first two paths by providing a glob of "*thing*.txt"
    /// </example>
    pub glob: String,
}

pub struct PathSearchTool;

impl Tool for PathSearchTool {
    fn name(&self) -> String {
        "path-search".into()
    }

    fn description(&self) -> String {
        include_str!("./path_search_tool/description.md").into()
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(PathSearchToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let glob = match serde_json::from_value::<PathSearchToolInput>(input) {
            Ok(input) => input.glob,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };
        let path_matcher = match PathMatcher::new(&[glob.clone()]) {
            Ok(matcher) => matcher,
            Err(err) => return Task::ready(Err(anyhow!("Invalid glob: {}", err))),
        };

        let mut matches = Vec::new();

        for worktree_handle in project.read(cx).worktrees(cx) {
            let worktree = worktree_handle.read(cx);
            let root_name = worktree.root_name();

            // Don't consider ignored entries.
            for entry in worktree.entries(false, 0) {
                if path_matcher.is_match(&entry.path) {
                    matches.push(
                        PathBuf::from(root_name)
                            .join(&entry.path)
                            .to_string_lossy()
                            .to_string(),
                    );
                }
            }
        }

        if matches.is_empty() {
            Task::ready(Ok(format!(
                "No paths in the project matched the glob {glob:?}"
            )))
        } else {
            // Sort to group entries in the same directory together.
            matches.sort();
            Task::ready(Ok(matches.join("\n")))
        }
    }
}
