use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolCard, ToolResult, ToolUseStatus};
use gpui::{App, AppContext, Context, Entity, IntoElement, Task, Window};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use ui::{Disclosure, prelude::*};
use util::paths::PathMatcher;

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

        let glob_for_card = glob.clone();
        let card = if !matches.is_empty() {
            let matches_for_card: Vec<_> = matches
                .iter()
                .skip(offset as usize)
                .take(RESULTS_PER_PAGE)
                .cloned()
                .collect();

            Some(
                cx.new(|cx| {
                    PathSearchToolCard::new(glob_for_card, total_matches, matches_for_card, cx)
                })
                .into(),
            )
        } else {
            Some(
                cx.new(|cx| PathSearchToolCard::new(glob_for_card, 0, Vec::new(), cx))
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
    glob: String,
    total_matches: usize,
    paths: Vec<String>,
    expanded: bool,
}

impl PathSearchToolCard {
    fn new(
        glob: String,
        total_matches: usize,
        paths: Vec<String>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            glob,
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
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let matches_label: SharedString = if self.total_matches == 0 {
            "No matches for".into()
        } else if self.total_matches == 1 {
            "1 match for".into()
        } else {
            format!("{} matches for", self.total_matches).into()
        };

        let header = h_flex()
            .id("tool-label-container")
            .group("tool-label-container")
            .gap_1p5()
            .max_w_full()
            .justify_between()
            .child({
                h_flex()
                    .gap_1p5()
                    .child(
                        Icon::new(IconName::SearchCode)
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(Label::new(matches_label).size(LabelSize::Small))
                    .child(
                        Label::new(&self.glob)
                            .size(LabelSize::Small)
                            .buffer_font(cx),
                    )
                    .into_any_element()
            })
            .child(
                div().visible_on_hover("tool-label-container").child(
                    Disclosure::new("path-search-disclosure", self.expanded)
                        .opened_icon(IconName::ChevronUp)
                        .closed_icon(IconName::ChevronDown)
                        .disabled(self.paths.is_empty())
                        .on_click(cx.listener(move |this, _event, _window, _cx| {
                            this.expanded = !this.expanded;
                        })),
                ),
            )
            .into_any();

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
                        Button::new(("path", index), path.clone())
                            .icon(IconName::ArrowUpRight)
                            .icon_size(IconSize::XSmall)
                            .icon_position(IconPosition::End)
                            .label_size(LabelSize::Small)
                            .color(Color::Muted)
                    }))
                    .into_any(),
            )
        } else {
            None
        };

        v_flex().my_2().gap_1().child(header).children(content)
    }
}
