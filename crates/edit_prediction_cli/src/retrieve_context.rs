use crate::{
    example::{Example, ExampleContext},
    headless::EpAppState,
    load_project::run_load_project,
};
use collections::HashSet;
use edit_prediction::{DebugEvent, EditPredictionStore};
use futures::{FutureExt as _, StreamExt as _, channel::mpsc};
use gpui::{AsyncApp, Entity};
use language::Buffer;
use project::Project;
use std::{sync::Arc, time::Duration};

pub async fn run_context_retrieval(
    example: &mut Example,
    app_state: Arc<EpAppState>,
    mut cx: AsyncApp,
) {
    if example.context.is_some() {
        return;
    }

    run_load_project(example, app_state.clone(), cx.clone()).await;

    let state = example.state.as_ref().unwrap();
    let project = state.project.clone();

    let _lsp_handle = project
        .update(&mut cx, |project, cx| {
            project.register_buffer_with_language_servers(&state.buffer, cx)
        })
        .unwrap();
    wait_for_language_servers_to_start(example, &project, &state.buffer, &mut cx).await;

    let ep_store = cx
        .update(|cx| EditPredictionStore::try_global(cx).unwrap())
        .unwrap();

    let mut events = ep_store
        .update(&mut cx, |store, cx| {
            store.register_buffer(&state.buffer, &project, cx);
            store.set_use_context(true);
            store.refresh_context(&project, &state.buffer, state.cursor_position, cx);
            store.debug_info(&project, cx)
        })
        .unwrap();

    while let Some(event) = events.next().await {
        match event {
            DebugEvent::ContextRetrievalFinished(_) => {
                break;
            }
            _ => {}
        }
    }

    let context_files = ep_store
        .update(&mut cx, |store, cx| store.context_for_project(&project, cx))
        .unwrap();

    example.context = Some(ExampleContext {
        files: context_files,
    });
}

async fn wait_for_language_servers_to_start(
    example: &Example,
    project: &Entity<Project>,
    buffer: &Entity<Buffer>,
    cx: &mut AsyncApp,
) {
    let log_prefix = format!("{} | ", example.name);

    let lsp_store = project
        .read_with(cx, |project, _| project.lsp_store())
        .unwrap();

    let (language_server_ids, mut starting_language_server_ids) = buffer
        .update(cx, |buffer, cx| {
            lsp_store.update(cx, |lsp_store, cx| {
                let ids = lsp_store.language_servers_for_local_buffer(buffer, cx);
                let starting_ids = ids
                    .iter()
                    .copied()
                    .filter(|id| !lsp_store.language_server_statuses.contains_key(&id))
                    .collect::<HashSet<_>>();
                (ids, starting_ids)
            })
        })
        .unwrap_or_default();

    eprintln!(
        "{}⏵ Waiting for {} language servers",
        log_prefix,
        language_server_ids.len()
    );

    let timeout = cx
        .background_executor()
        .timer(Duration::from_secs(60 * 5))
        .shared();

    let (mut tx, mut rx) = mpsc::channel(language_server_ids.len());
    let added_subscription = cx.subscribe(project, {
        let log_prefix = log_prefix.clone();
        move |_, event, _| match event {
            project::Event::LanguageServerAdded(language_server_id, name, _) => {
                eprintln!("{}+ Language server started: {}", log_prefix, name);
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
                panic!("LSP wait timed out after 5 minutes");
            }
        }
    }

    drop(added_subscription);

    if !language_server_ids.is_empty() {
        project
            .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
            .unwrap()
            .detach();
    }

    let (mut tx, mut rx) = mpsc::channel(language_server_ids.len());
    let subscriptions = [
        cx.subscribe(&lsp_store, {
            let log_prefix = log_prefix.clone();
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
                    eprintln!("{}⟲ {message}", log_prefix)
                }
            }
        }),
        cx.subscribe(project, {
            let log_prefix = log_prefix.clone();
            move |_, event, cx| match event {
                project::Event::DiskBasedDiagnosticsFinished { language_server_id } => {
                    let lsp_store = lsp_store.read(cx);
                    let name = lsp_store
                        .language_server_adapter_for_id(*language_server_id)
                        .unwrap()
                        .name();
                    eprintln!("{}⚑ Language server idle: {}", log_prefix, name);
                    tx.try_send(*language_server_id).ok();
                }
                _ => {}
            }
        }),
    ];

    project
        .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
        .unwrap()
        .await
        .unwrap();

    let mut pending_language_server_ids = HashSet::from_iter(language_server_ids.into_iter());
    while !pending_language_server_ids.is_empty() {
        futures::select! {
            language_server_id = rx.next() => {
                if let Some(id) = language_server_id {
                    pending_language_server_ids.remove(&id);
                }
            },
            _ = timeout.clone().fuse() => {
                panic!("LSP wait timed out after 5 minutes");
            }
        }
    }

    drop(subscriptions);
}
