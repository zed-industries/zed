use crate::{schema::json_schema_for, ui::ToolCallCardHeader};
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolCard, ToolResult, ToolUseStatus};
use editor::Editor;
use futures::channel::oneshot::{self, Receiver};
use gpui::{
    AnyWindowHandle, App, AppContext, Context, Entity, IntoElement, Task, WeakEntity, Window,
};
use language;
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::{cmp, path::PathBuf, sync::Arc};
use ui::{Disclosure, Tooltip, prelude::*};
use util::{ResultExt, paths::PathMatcher};
use workspace::Workspace;

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

const RESULTS_PER_PAGE: usize = 50;

pub struct FindPathTool;

impl Tool for FindPathTool {
    fn name(&self) -> String {
        "find_path".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./find_path_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::SearchCode
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<FindPathToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<FindPathToolInput>(input.clone()) {
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
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let (offset, glob) = match serde_json::from_value::<FindPathToolInput>(input) {
            Ok(input) => (input.offset, input.glob),
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        let (sender, receiver) = oneshot::channel();

        let card = cx.new(|cx| FindPathToolCard::new(glob.clone(), receiver, cx));

        let search_paths_task = search_paths(&glob, project, cx);

        let task = cx.background_spawn(async move {
            let matches = search_paths_task.await?;
            let paginated_matches: &[PathBuf] = &matches[cmp::min(offset, matches.len())
                ..cmp::min(offset + RESULTS_PER_PAGE, matches.len())];

            sender.send(paginated_matches.to_vec()).log_err();

            if matches.is_empty() {
                Ok("No matches found".to_string().into())
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
                Ok(message.into())
            }
        });

        ToolResult {
            output: task,
            card: Some(card.into()),
        }
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

struct FindPathToolCard {
    paths: Vec<PathBuf>,
    expanded: bool,
    glob: String,
    _receiver_task: Option<Task<Result<()>>>,
}

impl FindPathToolCard {
    fn new(glob: String, receiver: Receiver<Vec<PathBuf>>, cx: &mut Context<Self>) -> Self {
        let _receiver_task = cx.spawn(async move |this, cx| {
            let paths = receiver.await?;

            this.update(cx, |this, _cx| {
                this.paths = paths;
            })
            .log_err();

            Ok(())
        });

        Self {
            paths: Vec::new(),
            expanded: false,
            glob,
            _receiver_task: Some(_receiver_task),
        }
    }
}

impl ToolCard for FindPathToolCard {
    fn render(
        &mut self,
        _status: &ToolUseStatus,
        _window: &mut Window,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let matches_label: SharedString = if self.paths.len() == 0 {
            "No matches".into()
        } else if self.paths.len() == 1 {
            "1 match".into()
        } else {
            format!("{} matches", self.paths.len()).into()
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
                        let button_label = path.to_string_lossy().to_string();

                        Button::new(("path", index), button_label)
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
                            .on_click(cx.listener(move |this, _, _, _cx| {
                                this.expanded = !this.expanded;
                            })),
                    ),
            )
            .children(content)
    }
}

impl Component for FindPathTool {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn sort_name() -> &'static str {
        "FindPathTool"
    }

    fn preview(window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let successful_card = cx.new(|_| FindPathToolCard {
            paths: vec![
                PathBuf::from("src/main.rs"),
                PathBuf::from("src/lib.rs"),
                PathBuf::from("tests/test.rs"),
            ],
            expanded: true,
            glob: "*.rs".to_string(),
            _receiver_task: None,
        });

        let empty_card = cx.new(|_| FindPathToolCard {
            paths: Vec::new(),
            expanded: false,
            glob: "*.nonexistent".to_string(),
            _receiver_task: None,
        });

        Some(
            v_flex()
                .gap_6()
                .children(vec![example_group(vec![
                    single_example(
                        "With Paths",
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
                        "No Paths",
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
