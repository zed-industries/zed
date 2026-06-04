use crate::{
    example::Example,
    headless::EpAppState,
    load_project::run_load_project,
    progress::{ExampleProgress, InfoStyle, Step, StepProgress},
};
use anyhow::Context as _;
use clap::ValueEnum;
use collections::HashSet;
use edit_prediction::{DebugEvent, EditPredictionStore, udiff::refresh_worktree_entries};
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
        let oracle_paths = if editable_context_sources.contains(&ContextSource::OracleFile) {
            let oracle_paths = oracle_paths_from_expected_patches(example);
            refresh_paths(&project, &oracle_paths, &mut cx).await?;
            oracle_paths
        } else {
            Vec::new()
        };

        let editable_context = ep_store
            .update(&mut cx, |store, cx| {
                store.collect_editable_context(
                    project.clone(),
                    state.buffer.clone(),
                    state.cursor_position,
                    oracle_paths,
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
            existing_file
                .excerpts
                .sort_by_key(|excerpt| (excerpt.order, excerpt.row_range.start));
            existing_file.in_open_source_repo =
                existing_file.in_open_source_repo && new_file.in_open_source_repo;
        } else {
            context_files.push(new_file);
        }
    }
}

fn oracle_paths_from_expected_patches(example: &Example) -> Vec<Arc<Path>> {
    let mut seen_paths = HashSet::default();
    let mut paths = Vec::new();

    for patch in &example.spec.expected_patches {
        for path in paths_from_diff(patch) {
            if seen_paths.insert(path.clone()) {
                paths.push(path.into());
            }
        }
    }

    paths
}

fn paths_from_diff(diff: &str) -> Vec<PathBuf> {
    diff.lines()
        .filter_map(|line| match DiffLine::parse(line) {
            DiffLine::OldPath { path } | DiffLine::NewPath { path }
                if path.as_ref() != "/dev/null" =>
            {
                Some(Path::new(path.as_ref()).to_path_buf())
            }
            _ => None,
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
