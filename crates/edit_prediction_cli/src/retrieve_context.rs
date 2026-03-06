use crate::{
    example::Example,
    headless::EpAppState,
    load_project::run_load_project,
    progress::{ExampleProgress, InfoStyle, Step, StepProgress},
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
    example_progress: &ExampleProgress,
    mut cx: AsyncApp,
) -> anyhow::Result<()> {
    if example.prompt_inputs.is_some() {
        if example.spec.repository_url.is_empty() {
            return Ok(());
        }

        if example
            .prompt_inputs
            .as_ref()
            .is_some_and(|inputs| !inputs.related_files.is_empty())
        {
            return Ok(());
        }
    }

    run_load_project(example, app_state.clone(), example_progress, cx.clone()).await?;

    let step_progress: Arc<StepProgress> = example_progress.start(Step::Context).into();

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
        prompt_inputs.related_files = context_files;
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
