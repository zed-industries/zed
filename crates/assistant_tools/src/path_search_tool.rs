use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use gpui::{App, AppContext, Entity, Task};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{cmp, fmt::Write as _, path::PathBuf, sync::Arc};
use ui::IconName;
use util::paths::PathMatcher;
use worktree::Snapshot;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PathSearchToolInput {
    /// The glob to match against every path in the project.
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
        let offset = offset as usize;
        let task = search_paths(&glob, project, cx);
        cx.background_spawn(async move {
            let matches = task.await?;
            let paginated_matches = &matches[cmp::min(offset, matches.len())
                ..cmp::min(offset + RESULTS_PER_PAGE, matches.len())];

            if matches.is_empty() {
                Ok("No matches found".to_string())
            } else {
                let mut message = format!("Found {} total matches.", matches.len());
                if matches.len() > RESULTS_PER_PAGE {
                    write!(
                        &mut message,
                        "\nShowing results {}-{} (provide 'offset' parameter for more results):",
                        offset + 1,
                        offset + paginated_matches.len()
                    )
                    .unwrap();
                }
                for mat in matches.into_iter().skip(offset).take(RESULTS_PER_PAGE) {
                    write!(&mut message, "\n{}", mat.display()).unwrap();
                }
                Ok(message)
            }
        })
        .into()
    }
}

fn search_paths(glob: &str, project: Entity<Project>, cx: &mut App) -> Task<Result<Vec<PathBuf>>> {
    let path_matcher = match PathMatcher::new([
        // Sometimes models try to search for "". In this case, return all paths in the project.
        if glob.is_empty() { "*" } else { glob },
    ]) {
        Ok(matcher) => matcher,
        Err(err) => return Task::ready(Err(anyhow!("Invalid glob: {err}"))),
    };
    let snapshots: Vec<Snapshot> = project
        .read(cx)
        .worktrees(cx)
        .map(|worktree| worktree.read(cx).snapshot())
        .collect();

    cx.background_spawn(async move {
        Ok(snapshots
            .iter()
            .flat_map(|snapshot| {
                let root_name = PathBuf::from(snapshot.root_name());
                snapshot
                    .entries(false, 0)
                    .map(move |entry| root_name.join(&entry.path))
                    .filter(|path| path_matcher.is_match(&path))
            })
            .collect())
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use gpui::TestAppContext;
    use project::{FakeFs, Project};
    use settings::SettingsStore;
    use util::path;

    #[gpui::test]
    async fn test_path_search_tool(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            serde_json::json!({
                "apple": {
                    "banana": {
                        "carrot": "1",
                    },
                    "bandana": {
                        "carbonara": "2",
                    },
                    "endive": "3"
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;

        let matches = cx
            .update(|cx| search_paths("root/**/car*", project.clone(), cx))
            .await
            .unwrap();
        assert_eq!(
            matches,
            &[
                PathBuf::from("root/apple/banana/carrot"),
                PathBuf::from("root/apple/bandana/carbonara")
            ]
        );

        let matches = cx
            .update(|cx| search_paths("**/car*", project.clone(), cx))
            .await
            .unwrap();
        assert_eq!(
            matches,
            &[
                PathBuf::from("root/apple/banana/carrot"),
                PathBuf::from("root/apple/bandana/carbonara")
            ]
        );
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
        });
    }
}
