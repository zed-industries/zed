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

        let path_matcher = match PathMatcher::new([
            // Sometimes models try to search for "". In this case, return all paths in the project.
            if glob.is_empty() { "*" } else { &glob },
        ]) {
            Ok(matcher) => matcher,
            Err(err) => return Task::ready(Err(anyhow!("Invalid glob: {err}"))).into(),
        };

        let project_handle = project.read(cx);
        let worktrees: Vec<_> = project_handle.worktrees(cx).collect();

        let mut matches = Vec::new();
        for worktree_handle in &worktrees {
            let worktree = worktree_handle.read(cx);
            let snapshot = worktree.snapshot();
            let root_name = snapshot.root_name();

            for entry in snapshot.entries(false, 0) {
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

        matches.sort();
        let total_matches = matches.len();

        let card = if !matches.is_empty() {
            let matches_for_card: Vec<_> = matches
                .iter()
                .skip(offset as usize)
                .take(RESULTS_PER_PAGE)
                .cloned()
                .collect();

            Some(
                cx.new(|cx| PathSearchToolCard::new(total_matches, matches_for_card, cx))
                    .into(),
            )
        } else {
            Some(
                cx.new(|cx| PathSearchToolCard::new(0, Vec::new(), cx))
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

struct PathSearchToolCard {
    total_matches: usize,
    paths: Vec<String>,
    expanded: bool,
}

impl PathSearchToolCard {
    fn new(total_matches: usize, paths: Vec<String>, _cx: &mut Context<Self>) -> Self {
        Self {
            total_matches,
            paths,
            expanded: false,
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
            "1 match for".into()
        } else {
            format!("{} matches for", self.total_matches).into()
        };

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
                ToolCallCardHeader::new(IconName::SearchCode, matches_label).disclosure_slot(
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
        });

        let empty_card = cx.new(|_| PathSearchToolCard {
            total_matches: 0,
            paths: Vec::new(),
            expanded: false,
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
