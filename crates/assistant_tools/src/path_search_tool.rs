use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use gpui::{App, AppContext, Entity, Task};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use ui::IconName;
use util::paths::PathMatcher;
use worktree::Snapshot;

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

    /// Optional starting position for paginated results (0-based).
    /// When not provided, starts from the beginning.
    #[serde(default)]
    pub offset: u32,
}

const RESULTS_PER_PAGE: usize = 50;

pub struct PathSearchTool;

impl Tool for PathSearchTool {
    fn name(&self) -> String {
        "path_search".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./path_search_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::SearchCode
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<PathSearchToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<PathSearchToolInput>(input.clone()) {
            Ok(input) => format!("Find paths matching “`{}`”", input.glob),
            Err(_) => "Search paths".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> ToolResult {
        let (offset, glob) = match serde_json::from_value::<PathSearchToolInput>(input) {
            Ok(input) => (input.offset, input.glob),
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        let path_matcher = match PathMatcher::new([
            // Sometimes models try to search for "". In this case, return all paths in the project.
            if glob.is_empty() { "*" } else { &glob },
        ]) {
            Ok(matcher) => matcher,
            Err(err) => return Task::ready(Err(anyhow!("Invalid glob: {err}"))).into(),
        };
        let snapshots: Vec<Snapshot> = project
            .read(cx)
            .worktrees(cx)
            .map(|worktree| worktree.read(cx).snapshot())
            .collect();

        cx.background_spawn(async move {
            let mut matches = Vec::new();

            for worktree in snapshots {
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
                Ok(format!("No paths in the project matched the glob {glob:?}"))
            } else {
                // Sort to group entries in the same directory together.
                matches.sort();

                let total_matches = matches.len();
                let response = if total_matches > RESULTS_PER_PAGE + offset as usize {
                let paginated_matches: Vec<_> = matches
                      .into_iter()
                      .skip(offset as usize)
                      .take(RESULTS_PER_PAGE)
                      .collect();

                    format!(
                        "Found {} total matches. Showing results {}-{} (provide 'offset' parameter for more results):\n\n{}",
                        total_matches,
                        offset + 1,
                        offset as usize + paginated_matches.len(),
                        paginated_matches.join("\n")
                    )
                } else {
                    matches.join("\n")
                };

                Ok(response)
            }
        }).into()
    }
}
