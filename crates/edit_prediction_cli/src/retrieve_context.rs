use crate::{
    example::Example,
    headless::EpAppState,
    load_project::run_load_project,
    progress::{InfoStyle, Progress, Step, StepProgress},
};
use anyhow::Context as _;
use collections::HashSet;
use edit_prediction::{DebugEvent, EditPredictionStore};
use futures::{FutureExt as _, StreamExt as _, channel::mpsc};
use gpui::{AsyncApp, Entity};
use language::Buffer;
use project::Project;
use std::sync::Arc;
use std::time::Duration;

pub async fn run_context_retrieval(
    example: &mut Example,
    app_state: Arc<EpAppState>,
    mut cx: AsyncApp,
) -> anyhow::Result<()> {
    if example
        .prompt_inputs
        .as_ref()
        .is_some_and(|inputs| inputs.related_files.is_some())
    {
        return Ok(());
    }

    run_load_project(example, app_state.clone(), cx.clone()).await?;

    let step_progress: Arc<StepProgress> = Progress::global()
        .start(Step::Context, &example.spec.name)
        .into();

    let state = example.state.as_ref().unwrap();
    let project = state.project.clone();

    let _lsp_handle = project.update(&mut cx, |project, cx| {
        project.register_buffer_with_language_servers(&state.buffer, cx)
    });
    wait_for_language_servers_to_start(&project, &state.buffer, &step_progress, &mut cx).await?;

    let ep_store = cx
        .update(|cx| EditPredictionStore::try_global(cx))
        .context("EditPredictionStore not initialized")?;

    let mut events = ep_store.update(&mut cx, |store, cx| {
        store.register_buffer(&state.buffer, &project, cx);
        store.set_use_context(true);
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

    let context_files =
        ep_store.update(&mut cx, |store, cx| store.context_for_project(&project, cx));

    let excerpt_count: usize = context_files.iter().map(|f| f.excerpts.len()).sum();
    step_progress.set_info(format!("{} excerpts", excerpt_count), InfoStyle::Normal);

    if let Some(prompt_inputs) = example.prompt_inputs.as_mut() {
        prompt_inputs.related_files = Some(context_files);
    }
    Ok(())
}

async fn wait_for_language_servers_to_start(
    project: &Entity<Project>,
    buffer: &Entity<Buffer>,
    step_progress: &Arc<StepProgress>,
    cx: &mut AsyncApp,
) -> anyhow::Result<()> {
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());

    let (language_server_ids, mut starting_language_server_ids) =
        buffer.update(cx, |buffer, cx| {
            lsp_store.update(cx, |lsp_store, cx| {
                let ids = lsp_store.language_servers_for_local_buffer(buffer, cx);
                let starting_ids = ids
                    .iter()
                    .copied()
                    .filter(|id| !lsp_store.language_server_statuses.contains_key(&id))
                    .collect::<HashSet<_>>();
                (ids, starting_ids)
            })
        });

    step_progress.set_substatus(format!("waiting for {} LSPs", language_server_ids.len()));

    let timeout = cx
        .background_executor()
        .timer(Duration::from_secs(60 * 5))
        .shared();

    let (mut tx, mut rx) = mpsc::channel(language_server_ids.len());
    let added_subscription = cx.subscribe(project, {
        let step_progress = step_progress.clone();
        move |_, event, _| match event {
            project::Event::LanguageServerAdded(language_server_id, name, _) => {
                step_progress.set_substatus(format!("LSP started: {}", name));
                tx.try_send(*language_server_id).ok();
            }
            _ => {}
        }
    });

    while !starting_language_server_ids.is_empty() {
        futures::select! {
            language_server_id = rx.next() => {
                if let Some(id) = language_server_id {
                    starting_language_server_ids.remove(&id);
                }
            },
            _ = timeout.clone().fuse() => {
                return Err(anyhow::anyhow!("LSP wait timed out after 5 minutes"));
            }
        }
    }

    drop(added_subscription);

    let (mut tx, mut rx) = mpsc::channel(language_server_ids.len());
    let subscriptions = [
        cx.subscribe(&lsp_store, {
            let step_progress = step_progress.clone();
            move |_, event, _| {
                if let project::LspStoreEvent::LanguageServerUpdate {
                    message:
                        client::proto::update_language_server::Variant::WorkProgress(
                            client::proto::LspWorkProgress {
                                message: Some(message),
                                ..
                            },
                        ),
                    ..
                } = event
                {
                    step_progress.set_substatus(message.clone());
                }
            }
        }),
        cx.subscribe(project, {
            let step_progress = step_progress.clone();
            let lsp_store = lsp_store.clone();
            move |_, event, cx| match event {
                project::Event::DiskBasedDiagnosticsFinished { language_server_id } => {
                    let lsp_store = lsp_store.read(cx);
                    let name = lsp_store
                        .language_server_adapter_for_id(*language_server_id)
                        .unwrap()
                        .name();
                    step_progress.set_substatus(format!("LSP idle: {}", name));
                    tx.try_send(*language_server_id).ok();
                }
                _ => {}
            }
        }),
    ];

    project
        .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
        .await?;

    let mut pending_language_server_ids = lsp_store.read_with(cx, |lsp_store, _| {
        language_server_ids
            .iter()
            .copied()
            .filter(|id| {
                lsp_store
                    .language_server_statuses
                    .get(id)
                    .is_some_and(|status| status.has_pending_diagnostic_updates)
            })
            .collect::<HashSet<_>>()
    });
    while !pending_language_server_ids.is_empty() {
        futures::select! {
            language_server_id = rx.next() => {
                if let Some(id) = language_server_id {
                    pending_language_server_ids.remove(&id);
                }
            },
            _ = timeout.clone().fuse() => {
                return Err(anyhow::anyhow!("LSP wait timed out after 5 minutes"));
            }
        }
    }

    drop(subscriptions);
    step_progress.clear_substatus();
    Ok(())
}
