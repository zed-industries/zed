use super::{create_label_for_command, SlashCommand, SlashCommandOutput};
use anyhow::{anyhow, Result};
use assistant_slash_command::{ArgumentCompletion, SlashCommandOutputSection};
use fuzzy::{PathMatch, StringMatchCandidate};
use gpui::{AppContext, Model, Task, View, WeakView};
use language::{
    Anchor, BufferSnapshot, DiagnosticEntry, DiagnosticSeverity, LspAdapterDelegate,
    OffsetRangeExt, ToOffset,
};
use project::{DiagnosticSummary, PathMatchCandidateSet, Project};
use rope::Point;
use std::{
    fmt::Write,
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc},
};
use ui::prelude::*;
use util::paths::PathMatcher;
use util::ResultExt;
use workspace::Workspace;

pub(crate) struct DiagnosticsSlashCommand;

impl DiagnosticsSlashCommand {
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
            let path_prefix: Arc<str> = Arc::default();
            Task::ready(
                entries
                    .into_iter()
                    .map(|(entry, _)| PathMatch {
                        score: 0.,
                        positions: Vec::new(),
                        worktree_id: entry.worktree_id.to_usize(),
                        path: entry.path.clone(),
                        path_prefix: path_prefix.clone(),
                        is_dir: false, // Diagnostics can't be produced for directories
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
                        include_root_name: true,
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

impl SlashCommand for DiagnosticsSlashCommand {
    fn name(&self) -> String {
        "diagnostics".into()
    }

    fn label(&self, cx: &AppContext) -> language::CodeLabel {
        create_label_for_command("diagnostics", &[INCLUDE_WARNINGS_ARGUMENT], cx)
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

    fn accepts_arguments(&self) -> bool {
        true
    }

    fn complete_argument(
        self: Arc<Self>,
        arguments: &[String],
        cancellation_flag: Arc<AtomicBool>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        let Some(workspace) = workspace.and_then(|workspace| workspace.upgrade()) else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };
        let query = arguments.last().cloned().unwrap_or_default();

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

            Ok(matches
                .into_iter()
                .map(|completion| ArgumentCompletion {
                    label: completion.clone().into(),
                    new_text: completion,
                    after_completion: assistant_slash_command::AfterCompletion::Run,
                    replace_previous_arguments: false,
                })
                .collect())
        })
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };

        let options = Options::parse(arguments);

        let task = collect_diagnostics(workspace.read(cx).project().clone(), options, cx);

        cx.spawn(move |_| async move { task.await?.ok_or_else(|| anyhow!("No diagnostics found")) })
    }
}

#[derive(Default)]
struct Options {
    include_warnings: bool,
    path_matcher: Option<PathMatcher>,
}

const INCLUDE_WARNINGS_ARGUMENT: &str = "--include-warnings";

impl Options {
    fn parse(arguments: &[String]) -> Self {
        let mut include_warnings = false;
        let mut path_matcher = None;
        for arg in arguments {
            if arg == INCLUDE_WARNINGS_ARGUMENT {
                include_warnings = true;
            } else {
                path_matcher = PathMatcher::new(&[arg.to_owned()]).log_err();
            }
        }
        Self {
            include_warnings,
            path_matcher,
        }
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
) -> Task<Result<Option<SlashCommandOutput>>> {
    let error_source = if let Some(path_matcher) = &options.path_matcher {
        debug_assert_eq!(path_matcher.sources().len(), 1);
        Some(path_matcher.sources().first().cloned().unwrap_or_default())
    } else {
        None
    };

    let glob_is_exact_file_match = if let Some(path) = options
        .path_matcher
        .as_ref()
        .and_then(|pm| pm.sources().first())
    {
        PathBuf::try_from(path)
            .ok()
            .and_then(|path| {
                project.read(cx).worktrees(cx).find_map(|worktree| {
                    let worktree = worktree.read(cx);
                    let worktree_root_path = Path::new(worktree.root_name());
                    let relative_path = path.strip_prefix(worktree_root_path).ok()?;
                    worktree.absolutize(&relative_path).ok()
                })
            })
            .is_some()
    } else {
        false
    };

    let project_handle = project.downgrade();
    let diagnostic_summaries: Vec<_> = project
        .read(cx)
        .diagnostic_summaries(false, cx)
        .flat_map(|(path, _, summary)| {
            let worktree = project.read(cx).worktree_for_id(path.worktree_id, cx)?;
            let mut path_buf = PathBuf::from(worktree.read(cx).root_name());
            path_buf.push(&path.path);
            Some((path, path_buf, summary))
        })
        .collect();

    cx.spawn(|mut cx| async move {
        let mut output = SlashCommandOutput::default();

        if let Some(error_source) = error_source.as_ref() {
            writeln!(output.text, "diagnostics: {}", error_source).unwrap();
        } else {
            writeln!(output.text, "diagnostics").unwrap();
        }

        let mut project_summary = DiagnosticSummary::default();
        for (project_path, path, summary) in diagnostic_summaries {
            if let Some(path_matcher) = &options.path_matcher {
                if !path_matcher.is_match(&path) {
                    continue;
                }
            }

            project_summary.error_count += summary.error_count;
            if options.include_warnings {
                project_summary.warning_count += summary.warning_count;
            } else if summary.error_count == 0 {
                continue;
            }

            let last_end = output.text.len();
            let file_path = path.to_string_lossy().to_string();
            if !glob_is_exact_file_match {
                writeln!(&mut output.text, "{file_path}").unwrap();
            }

            if let Some(buffer) = project_handle
                .update(&mut cx, |project, cx| project.open_buffer(project_path, cx))?
                .await
                .log_err()
            {
                let snapshot = cx.read_model(&buffer, |buffer, _| buffer.snapshot())?;
                collect_buffer_diagnostics(&mut output, &snapshot, options.include_warnings);
            }

            if !glob_is_exact_file_match {
                output.sections.push(SlashCommandOutputSection {
                    range: last_end..output.text.len().saturating_sub(1),
                    icon: IconName::File,
                    label: file_path.into(),
                    metadata: None,
                });
            }
        }

        // No diagnostics found
        if output.sections.is_empty() {
            return Ok(None);
        }

        let mut label = String::new();
        label.push_str("Diagnostics");
        if let Some(source) = error_source {
            write!(label, " ({})", source).unwrap();
        }

        if project_summary.error_count > 0 || project_summary.warning_count > 0 {
            label.push(':');

            if project_summary.error_count > 0 {
                write!(label, " {} errors", project_summary.error_count).unwrap();
                if project_summary.warning_count > 0 {
                    label.push_str(",");
                }
            }

            if project_summary.warning_count > 0 {
                write!(label, " {} warnings", project_summary.warning_count).unwrap();
            }
        }

        output.sections.insert(
            0,
            SlashCommandOutputSection {
                range: 0..output.text.len(),
                icon: IconName::Warning,
                label: label.into(),
                metadata: None,
            },
        );

        Ok(Some(output))
    })
}

pub fn collect_buffer_diagnostics(
    output: &mut SlashCommandOutput,
    snapshot: &BufferSnapshot,
    include_warnings: bool,
) {
    for (_, group) in snapshot.diagnostic_groups(None) {
        let entry = &group.entries[group.primary_ix];
        collect_diagnostic(output, entry, &snapshot, include_warnings)
    }
}

fn collect_diagnostic(
    output: &mut SlashCommandOutput,
    entry: &DiagnosticEntry<Anchor>,
    snapshot: &BufferSnapshot,
    include_warnings: bool,
) {
    const EXCERPT_EXPANSION_SIZE: u32 = 2;
    const MAX_MESSAGE_LENGTH: usize = 2000;

    let (ty, icon) = match entry.diagnostic.severity {
        DiagnosticSeverity::WARNING => {
            if !include_warnings {
                return;
            }
            ("warning", IconName::Warning)
        }
        DiagnosticSeverity::ERROR => ("error", IconName::XCircle),
        _ => return,
    };
    let prev_len = output.text.len();

    let range = entry.range.to_point(snapshot);
    let diagnostic_row_number = range.start.row + 1;

    let start_row = range.start.row.saturating_sub(EXCERPT_EXPANSION_SIZE);
    let end_row = (range.end.row + EXCERPT_EXPANSION_SIZE).min(snapshot.max_point().row) + 1;
    let excerpt_range =
        Point::new(start_row, 0).to_offset(&snapshot)..Point::new(end_row, 0).to_offset(&snapshot);

    output.text.push_str("```");
    if let Some(language_name) = snapshot.language().map(|l| l.code_fence_block_name()) {
        output.text.push_str(&language_name);
    }
    output.text.push('\n');

    let mut buffer_text = String::new();
    for chunk in snapshot.text_for_range(excerpt_range) {
        buffer_text.push_str(chunk);
    }

    for (i, line) in buffer_text.lines().enumerate() {
        let line_number = start_row + i as u32 + 1;
        writeln!(output.text, "{}", line).unwrap();

        if line_number == diagnostic_row_number {
            output.text.push_str("//");
            let prev_len = output.text.len();
            write!(output.text, " {}: ", ty).unwrap();
            let padding = output.text.len() - prev_len;

            let message = util::truncate(&entry.diagnostic.message, MAX_MESSAGE_LENGTH)
                .replace('\n', format!("\n//{:padding$}", "").as_str());

            writeln!(output.text, "{message}").unwrap();
        }
    }

    writeln!(output.text, "```").unwrap();
    output.sections.push(SlashCommandOutputSection {
        range: prev_len..output.text.len().saturating_sub(1),
        icon,
        label: entry.diagnostic.message.clone().into(),
        metadata: None,
    });
}
