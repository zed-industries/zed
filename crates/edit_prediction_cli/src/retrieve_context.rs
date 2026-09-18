use crate::{
    example::Example,
    format_prompt::line_start_offset,
    headless::EpAppState,
    load_project::run_load_project,
    progress::{ExampleProgress, InfoStyle, Step, StepProgress},
};
use anyhow::Context as _;
use clap::ValueEnum;
use collections::{HashMap, HashSet};
use edit_prediction::{DebugEvent, EditPredictionStore, udiff::refresh_worktree_entries};
use edit_prediction_context::OracleTarget;
use futures::{FutureExt as _, StreamExt as _, channel::mpsc};
use gpui::{AsyncApp, Entity};
use language::Buffer;
use project::Project;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use zeta_prompt::{ContextSource, udiff::DiffLine};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum)]
pub enum ContextRetrievalType {
    Lsp,
    Editable,
    CurrentFile,
    EditHistory,
    EditHistoryFile,
    GitLog,
    Bm25,
    OracleFile,
    OracleSnippet,
    #[default]
    All,
    None,
}

impl std::fmt::Display for ContextRetrievalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextRetrievalType::Lsp => write!(f, "lsp"),
            ContextRetrievalType::Editable => write!(f, "editable"),
            ContextRetrievalType::CurrentFile => write!(f, "current-file"),
            ContextRetrievalType::EditHistory => write!(f, "edit-history"),
            ContextRetrievalType::EditHistoryFile => write!(f, "edit-history-file"),
            ContextRetrievalType::GitLog => write!(f, "git-log"),
            ContextRetrievalType::Bm25 => write!(f, "bm25"),
            ContextRetrievalType::OracleFile => write!(f, "oracle-file"),
            ContextRetrievalType::OracleSnippet => write!(f, "oracle-snippet"),
            ContextRetrievalType::All => write!(f, "all"),
            ContextRetrievalType::None => write!(f, "none"),
        }
    }
}

impl ContextRetrievalType {
    pub fn context_sources(self) -> Vec<ContextSource> {
        match self {
            ContextRetrievalType::Lsp => vec![ContextSource::Lsp],
            ContextRetrievalType::Editable => editable_context_sources(),
            ContextRetrievalType::CurrentFile => vec![ContextSource::CurrentFile],
            ContextRetrievalType::EditHistory => vec![ContextSource::EditHistory],
            ContextRetrievalType::EditHistoryFile => vec![ContextSource::EditHistoryFile],
            ContextRetrievalType::GitLog => vec![ContextSource::GitLog],
            ContextRetrievalType::Bm25 => vec![ContextSource::Bm25],
            ContextRetrievalType::OracleFile => vec![ContextSource::OracleFile],
            ContextRetrievalType::OracleSnippet => vec![ContextSource::OracleSnippet],
            ContextRetrievalType::All => {
                let mut sources = vec![ContextSource::Lsp];
                sources.extend(editable_context_sources());
                sources
            }
            ContextRetrievalType::None => Vec::new(),
        }
    }
}

pub fn context_sources_for_types(context_types: &[ContextRetrievalType]) -> Vec<ContextSource> {
    let mut context_sources = Vec::new();
    for context_type in context_types {
        for context_source in context_type.context_sources() {
            if !context_sources.contains(&context_source) {
                context_sources.push(context_source);
            }
        }
    }
    context_sources
}

fn editable_context_sources() -> Vec<ContextSource> {
    vec![
        ContextSource::CursorExcerpt,
        ContextSource::CurrentFile,
        ContextSource::EditHistory,
        ContextSource::EditHistoryFile,
        ContextSource::GitLog,
        ContextSource::Bm25,
    ]
}

pub async fn run_context_retrieval(
    example: &mut Example,
    app_state: Arc<EpAppState>,
    example_progress: &ExampleProgress,
    context_types: Vec<ContextRetrievalType>,
    force: bool,
    mut cx: AsyncApp,
) -> anyhow::Result<()> {
    if (!force
        && example
            .prompt_inputs
            .as_ref()
            .is_some_and(|inputs| inputs.related_files.is_some()))
        || example.spec.repository_url.is_empty()
    {
        return Ok(());
    }

    run_load_project(example, app_state.clone(), example_progress, cx.clone()).await?;

    let step_progress: Arc<StepProgress> = example_progress.start(Step::Context).into();

    let state = example.state.as_ref().unwrap();
    let project = state.project.clone();

    let ep_store = cx
        .update(|cx| EditPredictionStore::try_global(cx))
        .context("EditPredictionStore not initialized")?;

    let mut context_files = Vec::new();
    let context_sources = context_sources_for_types(&context_types);

    if context_sources.contains(&ContextSource::Lsp) {
        let _lsp_handle = project.update(&mut cx, |project, cx| {
            project.register_buffer_with_language_servers(&state.buffer, cx)
        });
        wait_for_language_servers_to_start(&project, &state.buffer, &step_progress, &mut cx)
            .await?;

        let mut events = ep_store.update(&mut cx, |store, cx| {
            store.register_buffer(&state.buffer, &project, cx);
            store.refresh_context(&project, &state.buffer, state.cursor_position, cx);
            store.debug_info(&project, cx)
        });

        while let Some(event) = events.next().await {
            match event {
                DebugEvent::ContextRetrievalFinished(_) => {
                    break;
                }
                _ => {}
            }
        }

        context_files
            .extend(ep_store.update(&mut cx, |store, cx| store.context_for_project(&project, cx)));
    }

    let editable_context_sources = context_sources
        .into_iter()
        .filter(|context_source| *context_source != ContextSource::Lsp)
        .collect::<Vec<_>>();
    if !editable_context_sources.is_empty() {
        let oracle_targets = if editable_context_sources.contains(&ContextSource::OracleFile)
            || editable_context_sources.contains(&ContextSource::OracleSnippet)
        {
            let oracle_targets = oracle_targets_from_expected_patches(example);
            let oracle_paths = oracle_targets
                .iter()
                .map(|target| target.path.clone())
                .collect::<Vec<_>>();
            refresh_paths(&project, &oracle_paths, &mut cx).await?;
            oracle_targets
        } else {
            Vec::new()
        };

        let editable_context = ep_store
            .update(&mut cx, |store, cx| {
                store.collect_editable_context(
                    project.clone(),
                    state.buffer.clone(),
                    state.cursor_position,
                    oracle_targets,
                    editable_context_sources,
                    cx,
                )
            })
            .await?;
        merge_context_files(&mut context_files, editable_context);
    }

    let excerpt_count: usize = context_files.iter().map(|f| f.excerpts.len()).sum();
    step_progress.set_info(format!("{} excerpts", excerpt_count), InfoStyle::Normal);

    if let Some(prompt_inputs) = example.prompt_inputs.as_mut() {
        prompt_inputs.related_files = Some(context_files);
    }
    Ok(())
}

fn merge_context_files(
    context_files: &mut Vec<zeta_prompt::RelatedFile>,
    new_files: Vec<zeta_prompt::RelatedFile>,
) {
    for mut new_file in new_files {
        if let Some(existing_file) = context_files
            .iter_mut()
            .find(|existing_file| existing_file.path == new_file.path)
        {
            existing_file.max_row = existing_file.max_row.max(new_file.max_row);
            existing_file.excerpts.append(&mut new_file.excerpts);
            existing_file.in_open_source_repo =
                existing_file.in_open_source_repo && new_file.in_open_source_repo;
        } else {
            context_files.push(new_file);
        }
    }
    for file in context_files.iter_mut() {
        coalesce_touching_excerpts(&mut file.excerpts);
    }
}

/// Sort a file's excerpts by position and merge those whose row ranges touch
/// or overlap. Touching excerpts render seamlessly in prompts (no `...`
/// separator), so keeping them separate splits edit addressing across a
/// boundary the model can't see and wastes adjacent marker tags (see
/// `TeacherJumpsPrompt::parse`).
///
/// A merged excerpt keeps the minimum `order` (and that excerpt's context
/// source), so the highest-priority content still survives budget-based
/// selection downstream; the tradeoff is that lower-priority touching
/// content is now selected together with it.
fn coalesce_touching_excerpts(excerpts: &mut Vec<zeta_prompt::RelatedExcerpt>) {
    excerpts.sort_by_key(|excerpt| (excerpt.row_range.start, excerpt.row_range.end));
    let mut coalesced: Vec<zeta_prompt::RelatedExcerpt> = Vec::with_capacity(excerpts.len());
    for excerpt in excerpts.drain(..) {
        let Some(last) = coalesced.last_mut() else {
            coalesced.push(excerpt);
            continue;
        };
        if excerpt.row_range.start > last.row_range.end {
            coalesced.push(excerpt);
            continue;
        }
        if excerpt.row_range.end > last.row_range.end {
            // Touching or overlapping. Shared rows come from the same buffer
            // snapshot, so drop the duplicated prefix of the new excerpt and
            // append the rest.
            let mut overlap_rows = (last.row_range.end - excerpt.row_range.start) as usize;
            if !last.text.ends_with('\n') && !last.text.is_empty() {
                // An unterminated final line means `last` includes the
                // content of its `row_range.end` row itself.
                overlap_rows += 1;
            }
            let Some(tail_start) = line_start_offset(&excerpt.text, overlap_rows) else {
                // The excerpt has fewer lines than its row range claims;
                // leave it unmerged rather than corrupt the text.
                coalesced.push(excerpt);
                continue;
            };
            let mut text =
                String::with_capacity(last.text.len() + excerpt.text.len() - tail_start + 1);
            text.push_str(&last.text);
            if !text.is_empty() && !text.ends_with('\n') {
                text.push('\n');
            }
            text.push_str(&excerpt.text[tail_start..]);
            last.text = text.into();
            last.row_range.end = excerpt.row_range.end;
        }
        if excerpt.order < last.order {
            last.order = excerpt.order;
            last.context_source = excerpt.context_source;
        }
    }
    *excerpts = coalesced;
}

fn oracle_targets_from_expected_patches(example: &Example) -> Vec<OracleTarget> {
    let mut target_indices: HashMap<PathBuf, usize> = HashMap::default();
    let mut targets: Vec<(PathBuf, Vec<std::ops::Range<u32>>)> = Vec::new();

    let mut target_index = |path: &str, targets: &mut Vec<(PathBuf, Vec<std::ops::Range<u32>>)>| {
        let path = Path::new(path).to_path_buf();
        *target_indices.entry(path.clone()).or_insert_with(|| {
            targets.push((path, Vec::new()));
            targets.len() - 1
        })
    };

    for patch in &example.spec.expected_patches {
        // Index of the target whose old-side rows the current hunk headers
        // refer to. Hunk old rows refer to the file's current state only for
        // the first expected patch; later patches drift, but the snippet
        // padding absorbs small offsets.
        let mut current_target: Option<usize> = None;
        for line in patch.lines() {
            match DiffLine::parse(line) {
                DiffLine::OldPath { path } => {
                    current_target = (path.as_ref() != "/dev/null")
                        .then(|| target_index(path.as_ref(), &mut targets));
                }
                DiffLine::NewPath { path } => {
                    if path.as_ref() != "/dev/null" {
                        let index = target_index(path.as_ref(), &mut targets);
                        if current_target.is_none() {
                            current_target = Some(index);
                        }
                    }
                }
                DiffLine::HunkHeader(Some(location)) => {
                    if let Some(index) = current_target {
                        let start = location.start_line_old;
                        targets[index].1.push(start..start + location.count_old);
                    }
                }
                _ => {}
            }
        }
    }

    targets
        .into_iter()
        .map(|(path, row_ranges)| OracleTarget {
            path: path.into(),
            row_ranges,
        })
        .collect()
}

async fn refresh_paths(
    project: &Entity<Project>,
    paths: &[Arc<Path>],
    cx: &mut AsyncApp,
) -> anyhow::Result<()> {
    if paths.is_empty() {
        return Ok(());
    }

    let Some(worktree) = project.read_with(cx, |project, cx| project.visible_worktrees(cx).next())
    else {
        return Ok(());
    };

    refresh_worktree_entries(&worktree, paths.iter().map(|path| path.as_ref()), cx).await
}

async fn wait_for_language_servers_to_start(
    project: &Entity<Project>,
    buffer: &Entity<Buffer>,
    step_progress: &Arc<StepProgress>,
    cx: &mut AsyncApp,
) -> anyhow::Result<()> {
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());

    // Determine which servers exist for this buffer, and which are still starting.
    let mut servers_pending_start = HashSet::default();
    let mut servers_pending_diagnostics = HashSet::default();
    buffer.update(cx, |buffer, cx| {
        lsp_store.update(cx, |lsp_store, cx| {
            let ids = lsp_store.language_servers_for_local_buffer(buffer, cx);
            for &id in &ids {
                match lsp_store.language_server_statuses.get(&id) {
                    None => {
                        servers_pending_start.insert(id);
                        servers_pending_diagnostics.insert(id);
                    }
                    Some(status) if status.has_pending_diagnostic_updates => {
                        servers_pending_diagnostics.insert(id);
                    }
                    Some(_) => {}
                }
            }
        });
    });

    step_progress.set_substatus(format!(
        "waiting for {} LSPs",
        servers_pending_diagnostics.len()
    ));

    let timeout_duration = if servers_pending_start.is_empty() {
        Duration::from_secs(30)
    } else {
        Duration::from_secs(60 * 5)
    };
    let timeout = cx.background_executor().timer(timeout_duration).shared();

    let (mut started_tx, mut started_rx) = mpsc::channel(servers_pending_start.len().max(1));
    let (mut diag_tx, mut diag_rx) = mpsc::channel(servers_pending_diagnostics.len().max(1));
    let subscriptions = [cx.subscribe(&lsp_store, {
        let step_progress = step_progress.clone();
        move |lsp_store, event, cx| match event {
            project::LspStoreEvent::LanguageServerAdded(id, name, _) => {
                step_progress.set_substatus(format!("LSP started: {}", name));
                started_tx.try_send(*id).ok();
            }
            project::LspStoreEvent::DiskBasedDiagnosticsFinished { language_server_id } => {
                let name = lsp_store
                    .read(cx)
                    .language_server_adapter_for_id(*language_server_id)
                    .unwrap()
                    .name();
                step_progress.set_substatus(format!("LSP idle: {}", name));
                diag_tx.try_send(*language_server_id).ok();
            }
            project::LspStoreEvent::LanguageServerUpdate {
                message:
                    client::proto::update_language_server::Variant::WorkProgress(
                        client::proto::LspWorkProgress {
                            message: Some(message),
                            ..
                        },
                    ),
                ..
            } => {
                step_progress.set_substatus(message.clone());
            }
            _ => {}
        }
    })];

    // Phase 1: wait for all servers to start.
    while !servers_pending_start.is_empty() {
        futures::select! {
            id = started_rx.next() => {
                if let Some(id) = id {
                    servers_pending_start.remove(&id);
                }
            },
            _ = timeout.clone().fuse() => {
                return Err(anyhow::anyhow!("LSP wait timed out after {} minutes", timeout_duration.as_secs() / 60));
            }
        }
    }

    // Save the buffer so the server sees the current content and kicks off diagnostics.
    project
        .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
        .await?;

    // Phase 2: wait for all servers to finish their diagnostic pass.
    while !servers_pending_diagnostics.is_empty() {
        futures::select! {
            id = diag_rx.next() => {
                if let Some(id) = id {
                    servers_pending_diagnostics.remove(&id);
                }
            },
            _ = timeout.clone().fuse() => {
                return Err(anyhow::anyhow!("LSP wait timed out after {} minutes", timeout_duration.as_secs() / 60));
            }
        }
    }

    drop(subscriptions);
    step_progress.clear_substatus();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use zeta_prompt::{ContextSource, RelatedExcerpt, RelatedFile};

    fn excerpt(
        row_range: std::ops::Range<u32>,
        text: &str,
        order: usize,
        context_source: ContextSource,
    ) -> RelatedExcerpt {
        RelatedExcerpt {
            row_range,
            text: Arc::from(text),
            order,
            context_source,
        }
    }

    #[test]
    fn test_coalesce_touching_excerpts() {
        let mut excerpts = vec![
            excerpt(2..4, "line2\nline3\n", 1, ContextSource::EditHistory),
            excerpt(0..2, "line0\nline1\n", 3, ContextSource::Bm25),
            excerpt(4..6, "line4\nline5\n", 2, ContextSource::OracleSnippet),
            excerpt(10..12, "line10\nline11\n", 0, ContextSource::Bm25),
        ];
        coalesce_touching_excerpts(&mut excerpts);

        assert_eq!(excerpts.len(), 2);
        assert_eq!(excerpts[0].row_range, 0..6);
        assert_eq!(
            excerpts[0].text.as_ref(),
            "line0\nline1\nline2\nline3\nline4\nline5\n"
        );
        // The merged excerpt keeps the highest priority (minimum order) and
        // its context source.
        assert_eq!(excerpts[0].order, 1);
        assert_eq!(excerpts[0].context_source, ContextSource::EditHistory);
        // The gapped excerpt stays separate.
        assert_eq!(excerpts[1].row_range, 10..12);
    }

    #[test]
    fn test_coalesce_overlapping_excerpts_drops_duplicated_rows() {
        let mut excerpts = vec![
            excerpt(0..3, "line0\nline1\nline2\n", 0, ContextSource::Bm25),
            excerpt(
                2..5,
                "line2\nline3\nline4\n",
                1,
                ContextSource::OracleSnippet,
            ),
        ];
        coalesce_touching_excerpts(&mut excerpts);

        assert_eq!(excerpts.len(), 1);
        assert_eq!(excerpts[0].row_range, 0..5);
        assert_eq!(
            excerpts[0].text.as_ref(),
            "line0\nline1\nline2\nline3\nline4\n"
        );
        assert_eq!(excerpts[0].order, 0);
        assert_eq!(excerpts[0].context_source, ContextSource::Bm25);
    }

    #[test]
    fn test_coalesce_contained_excerpt_upgrades_order() {
        let mut excerpts = vec![
            excerpt(0..4, "line0\nline1\nline2\nline3\n", 5, ContextSource::Bm25),
            excerpt(1..3, "line1\nline2\n", 2, ContextSource::OracleSnippet),
        ];
        coalesce_touching_excerpts(&mut excerpts);

        assert_eq!(excerpts.len(), 1);
        assert_eq!(excerpts[0].row_range, 0..4);
        assert_eq!(excerpts[0].text.as_ref(), "line0\nline1\nline2\nline3\n");
        assert_eq!(excerpts[0].order, 2);
        assert_eq!(excerpts[0].context_source, ContextSource::OracleSnippet);
    }

    #[test]
    fn test_coalesce_handles_unterminated_final_line() {
        // An excerpt ending without a newline includes the content of its
        // `row_range.end` row (e.g. git-log excerpts ending at EOF), so a
        // touching excerpt's first row duplicates it.
        let mut excerpts = vec![
            excerpt(0..2, "line0\nline1\nline2", 0, ContextSource::GitLog),
            excerpt(2..4, "line2\nline3\n", 1, ContextSource::Bm25),
        ];
        coalesce_touching_excerpts(&mut excerpts);

        assert_eq!(excerpts.len(), 1);
        assert_eq!(excerpts[0].row_range, 0..4);
        assert_eq!(excerpts[0].text.as_ref(), "line0\nline1\nline2\nline3\n");
    }

    #[test]
    fn test_merge_context_files_coalesces_across_sources() {
        let path: Arc<Path> = Path::new("root/src/lib.rs").into();
        let mut context_files = vec![RelatedFile {
            path: path.clone(),
            max_row: 100,
            excerpts: vec![excerpt(
                0..2,
                "line0\nline1\n",
                0,
                ContextSource::CurrentFile,
            )],
            in_open_source_repo: false,
        }];
        let new_files = vec![RelatedFile {
            path,
            max_row: 100,
            excerpts: vec![excerpt(2..4, "line2\nline3\n", 1, ContextSource::Bm25)],
            in_open_source_repo: false,
        }];
        merge_context_files(&mut context_files, new_files);

        assert_eq!(context_files.len(), 1);
        assert_eq!(context_files[0].excerpts.len(), 1);
        assert_eq!(context_files[0].excerpts[0].row_range, 0..4);
        assert_eq!(
            context_files[0].excerpts[0].text.as_ref(),
            "line0\nline1\nline2\nline3\n"
        );
    }
}
