use crate::{schema::json_schema_for, ui::ToolCallCardHeader};
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolCard, ToolResult, ToolUseStatus};
use editor::Editor;
use gpui::{
    AnyWindowHandle, App, AppContext, Context, Entity, IntoElement, Task, WeakEntity, Window,
};
use language;
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use ui::{Disclosure, Tooltip, prelude::*};
use util::{ResultExt, paths::PathMatcher};
use workspace::Workspace;

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

#[derive(RegisterComponent)]
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
            Ok(input) => format!("Find paths matching \"`{}`\"", input.glob),
            Err(_) => "Search paths".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let (offset, glob) = match serde_json::from_value::<PathSearchToolInput>(input.clone()) {
            Ok(input) => (input.offset, input.glob),
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        let matches = match search_paths(&glob, project, cx) {
            Ok(matches) => matches,
            Err(err) => return Task::ready(Err(err)).into(),
        };

        let total_matches = matches.len();

        let card = if !matches.is_empty() {
            let matches_for_card: Vec<_> = matches
                .iter()
                .skip(offset as usize)
                .take(RESULTS_PER_PAGE)
                .cloned()
                .collect();

            Some(
                cx.new(|cx| {
                    PathSearchToolCard::new(total_matches, matches_for_card, glob.clone(), cx)
                })
                .into(),
            )
        } else {
            Some(
                cx.new(|cx| PathSearchToolCard::new(0, Vec::new(), glob.clone(), cx))
                    .into(),
            )
        };

        let result = if matches.is_empty() {
            format!("No paths in the project matched the glob {glob:?}")
        } else {
            let paginated_matches: Vec<_> = matches
                .into_iter()
                .skip(offset as usize)
                .take(RESULTS_PER_PAGE)
                .collect();

            if total_matches > RESULTS_PER_PAGE + offset as usize {
                format!(
                    "Found {} total matches. Showing results {}-{} (provide 'offset' parameter for more results):\n\n{}",
                    total_matches,
                    offset + 1,
                    offset as usize + paginated_matches.len(),
                    paginated_matches.join("\n")
                )
            } else {
                paginated_matches.join("\n")
            }
        };

        ToolResult {
            output: Task::ready(Ok(result)),
            card,
        }
    }
}

fn search_paths(glob: &str, project: Entity<Project>, cx: &mut App) -> Result<Vec<String>> {
    let path_matcher = match PathMatcher::new([if glob.is_empty() { "*" } else { glob }]) {
        Ok(matcher) => matcher,
        Err(err) => return Err(anyhow!("Invalid glob: {err}")),
    };

    let project_handle = project.read(cx);
    let worktrees: Vec<_> = project_handle.worktrees(cx).collect();

    let mut matches = Vec::new();
    for worktree_handle in &worktrees {
        let worktree = worktree_handle.read(cx);
        let snapshot = worktree.snapshot();
        let root_name = snapshot.root_name();

        for entry in snapshot.entries(false, 0) {
            let full_path = PathBuf::from(root_name).join(&entry.path);
            let full_path_str = full_path.to_string_lossy().to_string();

            if path_matcher.is_match(&entry.path) || path_matcher.is_match(&full_path) {
                matches.push(full_path_str);
            }
        }
    }

    matches.sort();
    Ok(matches)
}

struct PathSearchToolCard {
    total_matches: usize,
    paths: Vec<String>,
    expanded: bool,
    glob: String,
}

impl PathSearchToolCard {
    fn new(
        total_matches: usize,
        paths: Vec<String>,
        glob: String,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            total_matches,
            paths,
            expanded: false,
            glob,
        }
    }
}

impl ToolCard for PathSearchToolCard {
    fn render(
        &mut self,
        _status: &ToolUseStatus,
        _window: &mut Window,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let matches_label: SharedString = if self.total_matches == 0 {
            "No matches".into()
        } else if self.total_matches == 1 {
            "1 match".into()
        } else {
            format!("{} matches", self.total_matches).into()
        };

        let glob_label = self.glob.to_string();

        let content = if !self.paths.is_empty() && self.expanded {
            Some(
                v_flex()
                    .relative()
                    .ml_1p5()
                    .px_1p5()
                    .gap_0p5()
                    .border_l_1()
                    .border_color(cx.theme().colors().border_variant)
                    .children(self.paths.iter().enumerate().map(|(index, path)| {
                        let path_clone = path.clone();
                        let workspace_clone = workspace.clone();

                        Button::new(("path", index), path.clone())
                            .icon(IconName::ArrowUpRight)
                            .icon_size(IconSize::XSmall)
                            .icon_position(IconPosition::End)
                            .label_size(LabelSize::Small)
                            .color(Color::Muted)
                            .tooltip(Tooltip::text("Jump to File"))
                            .on_click(move |_, window, cx| {
                                workspace_clone
                                    .update(cx, |workspace, cx| {
                                        let path = PathBuf::from(&path_clone);
                                        let Some(project_path) = workspace
                                            .project()
                                            .read(cx)
                                            .find_project_path(&path, cx)
                                        else {
                                            return;
                                        };
                                        let open_task = workspace.open_path(
                                            project_path,
                                            None,
                                            true,
                                            window,
                                            cx,
                                        );
                                        window
                                            .spawn(cx, async move |cx| {
                                                let item = open_task.await?;
                                                if let Some(active_editor) =
                                                    item.downcast::<Editor>()
                                                {
                                                    active_editor
                                                        .update_in(cx, |editor, window, cx| {
                                                            editor.go_to_singleton_buffer_point(
                                                                language::Point::new(0, 0),
                                                                window,
                                                                cx,
                                                            );
                                                        })
                                                        .log_err();
                                                }
                                                anyhow::Ok(())
                                            })
                                            .detach_and_log_err(cx);
                                    })
                                    .ok();
                            })
                    }))
                    .into_any(),
            )
        } else {
            None
        };

        v_flex()
            .mb_2()
            .gap_1()
            .child(
                ToolCallCardHeader::new(IconName::SearchCode, matches_label)
                    .with_code_path(glob_label)
                    .disclosure_slot(
                        Disclosure::new("path-search-disclosure", self.expanded)
                            .opened_icon(IconName::ChevronUp)
                            .closed_icon(IconName::ChevronDown)
                            .disabled(self.paths.is_empty())
                            .on_click(cx.listener(move |this, _event, _window, _cx| {
                                this.expanded = !this.expanded;
                            })),
                    ),
            )
            .children(content)
    }
}

impl Component for PathSearchTool {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn sort_name() -> &'static str {
        "ToolPathSearch"
    }

    fn preview(window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let successful_card = cx.new(|_| PathSearchToolCard {
            total_matches: 3,
            paths: vec![
                "src/main.rs".to_string(),
                "src/lib.rs".to_string(),
                "tests/test.rs".to_string(),
            ],
            expanded: true,
            glob: "*.rs".to_string(),
        });

        let empty_card = cx.new(|_| PathSearchToolCard {
            total_matches: 0,
            paths: Vec::new(),
            expanded: false,
            glob: "*.nonexistent".to_string(),
        });

        Some(
            v_flex()
                .gap_6()
                .children(vec![example_group(vec![
                    single_example(
                        "With Results",
                        div()
                            .size_full()
                            .child(successful_card.update(cx, |tool, cx| {
                                tool.render(
                                    &ToolUseStatus::Finished("".into()),
                                    window,
                                    WeakEntity::new_invalid(),
                                    cx,
                                )
                                .into_any_element()
                            }))
                            .into_any_element(),
                    ),
                    single_example(
                        "No Results",
                        div()
                            .size_full()
                            .child(empty_card.update(cx, |tool, cx| {
                                tool.render(
                                    &ToolUseStatus::Finished("".into()),
                                    window,
                                    WeakEntity::new_invalid(),
                                    cx,
                                )
                                .into_any_element()
                            }))
                            .into_any_element(),
                    ),
                ])])
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gpui::TestAppContext;
    use project::{FakeFs, Project};
    use settings::SettingsStore;
    use std::path::MAIN_SEPARATOR_STR;
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

        let expected_paths = [
            ["root", "apple", "banana", "carrot"].join(MAIN_SEPARATOR_STR),
            ["root", "apple", "bandana", "carbonara"].join(MAIN_SEPARATOR_STR),
        ];

        let matches = cx
            .update(|cx| search_paths("root/**/car*", project.clone(), cx))
            .unwrap();
        assert_eq!(matches, &expected_paths);

        let matches = cx
            .update(|cx| search_paths("**/car*", project.clone(), cx))
            .unwrap();
        assert_eq!(matches, &expected_paths);
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
