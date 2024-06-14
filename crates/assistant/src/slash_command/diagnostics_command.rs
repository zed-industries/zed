use super::{SlashCommand, SlashCommandOutput};
use anyhow::{anyhow, Result};
use assistant_slash_command::SlashCommandOutputSection;
use fuzzy::{PathMatch, StringMatchCandidate};
use gpui::{svg, AppContext, Model, RenderOnce, Task, View, WeakView};
use language::{
    Anchor, BufferSnapshot, DiagnosticEntry, DiagnosticSeverity, LspAdapterDelegate,
    OffsetRangeExt, ToOffset,
};
use project::{DiagnosticSummary, PathMatchCandidateSet, Project};
use rope::Point;
use std::fmt::Write;
use std::{
    ops::Range,
    sync::{atomic::AtomicBool, Arc},
};
use ui::{prelude::*, ButtonLike, ElevationIndex};
use util::paths::PathMatcher;
use util::ResultExt;
use workspace::Workspace;

pub(crate) struct DiagnosticsCommand;

impl DiagnosticsCommand {
    fn search_paths(
        &self,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        workspace: &View<Workspace>,
        cx: &mut AppContext,
    ) -> Task<Vec<PathMatch>> {
        if query.is_empty() {
            let workspace = workspace.read(cx);
            let entries = workspace.recent_navigation_history(Some(10), cx);
            let path_prefix: Arc<str> = "".into();
            Task::ready(
                entries
                    .into_iter()
                    .map(|(entry, _)| PathMatch {
                        score: 0.,
                        positions: Vec::new(),
                        worktree_id: entry.worktree_id.to_usize(),
                        path: entry.path.clone(),
                        path_prefix: path_prefix.clone(),
                        distance_to_relative_ancestor: 0,
                    })
                    .collect(),
            )
        } else {
            let worktrees = workspace.read(cx).visible_worktrees(cx).collect::<Vec<_>>();
            let candidate_sets = worktrees
                .into_iter()
                .map(|worktree| {
                    let worktree = worktree.read(cx);
                    PathMatchCandidateSet {
                        snapshot: worktree.snapshot(),
                        include_ignored: worktree
                            .root_entry()
                            .map_or(false, |entry| entry.is_ignored),
                        include_root_name: false,
                        candidates: project::Candidates::Entries,
                    }
                })
                .collect::<Vec<_>>();

            let executor = cx.background_executor().clone();
            cx.foreground_executor().spawn(async move {
                fuzzy::match_path_sets(
                    candidate_sets.as_slice(),
                    query.as_str(),
                    None,
                    false,
                    100,
                    &cancellation_flag,
                    executor,
                )
                .await
            })
        }
    }
}

impl SlashCommand for DiagnosticsCommand {
    fn name(&self) -> String {
        "diagnostics".into()
    }

    fn description(&self) -> String {
        "Insert diagnostics".into()
    }

    fn menu_text(&self) -> String {
        "Insert Diagnostics".into()
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn complete_argument(
        &self,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        let Some(workspace) = workspace.and_then(|workspace| workspace.upgrade()) else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };
        let query = query.split_whitespace().last().unwrap_or("").to_string();

        let paths = self.search_paths(query.clone(), cancellation_flag.clone(), &workspace, cx);
        let executor = cx.background_executor().clone();
        cx.background_executor().spawn(async move {
            let mut matches: Vec<String> = paths
                .await
                .into_iter()
                .map(|path_match| {
                    format!(
                        "{}{}",
                        path_match.path_prefix,
                        path_match.path.to_string_lossy()
                    )
                })
                .collect();

            matches.extend(
                fuzzy::match_strings(
                    &Options::match_candidates_for_args(),
                    &query,
                    false,
                    10,
                    &cancellation_flag,
                    executor,
                )
                .await
                .into_iter()
                .map(|candidate| candidate.string),
            );

            Ok(matches)
        })
    }

    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };

        let options = Options::parse(argument);

        let task = collect_diagnostics(workspace.read(cx).project().clone(), options, cx);
        cx.spawn(move |_| async move {
            let (text, sections) = task.await?;
            Ok(SlashCommandOutput {
                text,
                sections: sections
                    .into_iter()
                    .map(|(range, placeholder_type)| SlashCommandOutputSection {
                        range,
                        render_placeholder: Arc::new(move |id, unfold, _cx| {
                            DiagnosticsPlaceholder {
                                id,
                                unfold,
                                placeholder_type: placeholder_type.clone(),
                            }
                            .into_any_element()
                        }),
                    })
                    .collect(),
                run_commands_in_text: false,
            })
        })
    }
}

#[derive(Default)]
struct Options {
    include_warnings: bool,
    path_matcher: Option<PathMatcher>,
}

const INCLUDE_WARNINGS_ARGUMENT: &str = "--include-warnings";

impl Options {
    pub fn parse(arguments_line: Option<&str>) -> Self {
        arguments_line
            .map(|arguments_line| {
                let args = arguments_line.split_whitespace().collect::<Vec<_>>();
                let mut include_warnings = false;
                let mut path_matcher = None;
                for arg in args {
                    if arg == INCLUDE_WARNINGS_ARGUMENT {
                        include_warnings = true;
                    } else {
                        path_matcher = PathMatcher::new(arg).log_err();
                    }
                }
                Self {
                    include_warnings,
                    path_matcher,
                }
            })
            .unwrap_or_default()
    }

    fn match_candidates_for_args() -> [StringMatchCandidate; 1] {
        [StringMatchCandidate::new(
            0,
            INCLUDE_WARNINGS_ARGUMENT.to_string(),
        )]
    }
}

fn collect_diagnostics(
    project: Model<Project>,
    options: Options,
    cx: &mut AppContext,
) -> Task<Result<(String, Vec<(Range<usize>, PlaceholderType)>)>> {
    let header = if let Some(path_matcher) = &options.path_matcher {
        format!("diagnostics: {}", path_matcher.source())
    } else {
        "diagnostics".to_string()
    };

    let project_handle = project.downgrade();
    let diagnostic_summaries: Vec<_> = project.read(cx).diagnostic_summaries(false, cx).collect();

    cx.spawn(|mut cx| async move {
        let mut text = String::new();
        writeln!(text, "{}", &header).unwrap();
        let mut sections: Vec<(Range<usize>, PlaceholderType)> = Vec::new();

        let mut project_summary = DiagnosticSummary::default();
        for (project_path, _, summary) in diagnostic_summaries {
            if let Some(path_matcher) = &options.path_matcher {
                if !path_matcher.is_match(&project_path.path) {
                    continue;
                }
            }

            project_summary.error_count += summary.error_count;
            if options.include_warnings {
                project_summary.warning_count += summary.warning_count;
            } else if summary.error_count == 0 {
                continue;
            }

            let last_end = text.len();
            let file_path = project_path.path.to_string_lossy().to_string();
            writeln!(&mut text, "{file_path}").unwrap();

            if let Some(buffer) = project_handle
                .update(&mut cx, |project, cx| project.open_buffer(project_path, cx))?
                .await
                .log_err()
            {
                collect_buffer_diagnostics(
                    &mut text,
                    &mut sections,
                    cx.read_model(&buffer, |buffer, _| buffer.snapshot())?,
                    options.include_warnings,
                );
            }

            sections.push((
                last_end..text.len().saturating_sub(1),
                PlaceholderType::File(file_path),
            ))
        }
        sections.push((
            0..text.len(),
            PlaceholderType::Root(project_summary, header),
        ));

        Ok((text, sections))
    })
}

fn collect_buffer_diagnostics(
    text: &mut String,
    sections: &mut Vec<(Range<usize>, PlaceholderType)>,
    snapshot: BufferSnapshot,
    include_warnings: bool,
) {
    for (_, group) in snapshot.diagnostic_groups(None) {
        let entry = &group.entries[group.primary_ix];
        collect_diagnostic(text, sections, entry, &snapshot, include_warnings)
    }
}

fn collect_diagnostic(
    text: &mut String,
    sections: &mut Vec<(Range<usize>, PlaceholderType)>,
    entry: &DiagnosticEntry<Anchor>,
    snapshot: &BufferSnapshot,
    include_warnings: bool,
) {
    const EXCERPT_EXPANSION_SIZE: u32 = 2;
    const MAX_MESSAGE_LENGTH: usize = 2000;

    let ty = match entry.diagnostic.severity {
        DiagnosticSeverity::WARNING => {
            if !include_warnings {
                return;
            }
            DiagnosticType::Warning
        }
        DiagnosticSeverity::ERROR => DiagnosticType::Error,
        _ => return,
    };
    let prev_len = text.len();

    let range = entry.range.to_point(snapshot);
    let diagnostic_row_number = range.start.row + 1;

    let start_row = range.start.row.saturating_sub(EXCERPT_EXPANSION_SIZE);
    let end_row = (range.end.row + EXCERPT_EXPANSION_SIZE).min(snapshot.max_point().row) + 1;
    let excerpt_range =
        Point::new(start_row, 0).to_offset(&snapshot)..Point::new(end_row, 0).to_offset(&snapshot);

    text.push_str("```");
    if let Some(language_name) = snapshot.language().map(|l| l.code_fence_block_name()) {
        text.push_str(&language_name);
    }
    text.push('\n');

    let mut buffer_text = String::new();
    for chunk in snapshot.text_for_range(excerpt_range) {
        buffer_text.push_str(chunk);
    }

    for (i, line) in buffer_text.lines().enumerate() {
        let line_number = start_row + i as u32 + 1;
        writeln!(text, "{}", line).unwrap();

        if line_number == diagnostic_row_number {
            text.push_str("//");
            let prev_len = text.len();
            write!(text, " {}: ", ty.as_str()).unwrap();
            let padding = text.len() - prev_len;

            let message = util::truncate(&entry.diagnostic.message, MAX_MESSAGE_LENGTH)
                .replace('\n', format!("\n//{:padding$}", "").as_str());

            writeln!(text, "{message}").unwrap();
        }
    }

    writeln!(text, "```").unwrap();
    sections.push((
        prev_len..text.len().saturating_sub(1),
        PlaceholderType::Diagnostic(ty, entry.diagnostic.message.clone()),
    ))
}

#[derive(Clone)]
pub enum PlaceholderType {
    Root(DiagnosticSummary, String),
    File(String),
    Diagnostic(DiagnosticType, String),
}

#[derive(Copy, Clone, IntoElement)]
pub enum DiagnosticType {
    Warning,
    Error,
}

impl DiagnosticType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DiagnosticType::Warning => "warning",
            DiagnosticType::Error => "error",
        }
    }
}

#[derive(IntoElement)]
pub struct DiagnosticsPlaceholder {
    pub id: ElementId,
    pub placeholder_type: PlaceholderType,
    pub unfold: Arc<dyn Fn(&mut WindowContext)>,
}

impl RenderOnce for DiagnosticsPlaceholder {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let unfold = self.unfold;
        let (icon, content) = match self.placeholder_type {
            PlaceholderType::Root(summary, title) => (
                h_flex()
                    .w_full()
                    .gap_0p5()
                    .when(summary.error_count > 0, |this| {
                        this.child(DiagnosticType::Error)
                            .child(Label::new(summary.error_count.to_string()))
                    })
                    .when(summary.warning_count > 0, |this| {
                        this.child(DiagnosticType::Warning)
                            .child(Label::new(summary.warning_count.to_string()))
                    })
                    .into_any_element(),
                Label::new(title),
            ),
            PlaceholderType::File(file) => (
                Icon::new(IconName::File).into_any_element(),
                Label::new(file),
            ),
            PlaceholderType::Diagnostic(diagnostic_type, message) => (
                diagnostic_type.into_any_element(),
                Label::new(message).single_line(),
            ),
        };

        ButtonLike::new(self.id)
            .style(ButtonStyle::Filled)
            .layer(ElevationIndex::ElevatedSurface)
            .child(icon)
            .child(content)
            .on_click(move |_, cx| unfold(cx))
    }
}

impl RenderOnce for DiagnosticType {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        svg()
            .size(cx.text_style().font_size)
            .flex_none()
            .map(|icon| match self {
                DiagnosticType::Error => icon
                    .path(IconName::XCircle.path())
                    .text_color(Color::Error.color(cx)),
                DiagnosticType::Warning => icon
                    .path(IconName::ExclamationTriangle.path())
                    .text_color(Color::Warning.color(cx)),
            })
    }
}
