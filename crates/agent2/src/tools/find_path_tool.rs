use agent_client_protocol as acp;
use anyhow::{anyhow, Result};
use gpui::{App, AppContext, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::{cmp, path::PathBuf, sync::Arc};
use util::paths::PathMatcher;

use crate::{AgentTool, ToolCallEventStream};

/// Fast file path pattern matching tool that works with any codebase size
///
/// - Supports glob patterns like "**/*.js" or "src/**/*.ts"
/// - Returns matching file paths sorted alphabetically
/// - Prefer the `grep` tool to this tool when searching for symbols unless you have specific information about paths.
/// - Use this tool when you need to find files by name patterns
/// - Results are paginated with 50 matches per page. Use the optional 'offset' parameter to request subsequent pages.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FindPathToolInput {
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
    pub offset: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct FindPathToolOutput {
    paths: Vec<PathBuf>,
}

const RESULTS_PER_PAGE: usize = 50;

pub struct FindPathTool {
    project: Entity<Project>,
}

impl FindPathTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for FindPathTool {
    type Input = FindPathToolInput;

    fn name(&self) -> SharedString {
        "find_path".into()
    }

    fn kind(&self) -> acp::ToolKind {
        acp::ToolKind::Search
    }

    fn needs_authorization(&self, _: Self::Input, _: &App) -> bool {
        false
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let search_paths_task = search_paths(&input.glob, self.project.clone(), cx);

        cx.background_spawn(async move {
            let matches = search_paths_task.await?;
            let paginated_matches: &[PathBuf] = &matches[cmp::min(input.offset, matches.len())
                ..cmp::min(input.offset + RESULTS_PER_PAGE, matches.len())];

            event_stream.send_update(acp::ToolCallUpdateFields {
                title: Some(if paginated_matches.len() == 0 {
                    "No matches".into()
                } else if paginated_matches.len() == 1 {
                    "1 match".into()
                } else {
                    format!("{} matches", paginated_matches.len())
                }),
                content: Some(
                    paginated_matches
                        .iter()
                        .map(|path| acp::ToolCallContent::Content {
                            content: acp::ContentBlock::ResourceLink(acp::ResourceLink {
                                uri: format!("file://{}", path.display()),
                                name: path.to_string_lossy().into(),
                                annotations: None,
                                description: None,
                                mime_type: None,
                                size: None,
                                title: None,
                            }),
                        })
                        .collect(),
                ),
                raw_output: Some(serde_json::json!({
                    "paths": &matches,
                })),
                ..Default::default()
            });

            if matches.is_empty() {
                Ok("No matches found".into())
            } else {
                let mut message = format!("Found {} total matches.", matches.len());
                if matches.len() > RESULTS_PER_PAGE {
                    write!(
                        &mut message,
                        "\nShowing results {}-{} (provide 'offset' parameter for more results):",
                        input.offset + 1,
                        input.offset + paginated_matches.len()
                    )
                    .unwrap();
                }

                for mat in matches.iter().skip(input.offset).take(RESULTS_PER_PAGE) {
                    write!(&mut message, "\n{}", mat.display()).unwrap();
                }

                Ok(message)
            }
        })
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
    let snapshots: Vec<_> = project
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
    async fn test_find_path_tool(cx: &mut TestAppContext) {
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
