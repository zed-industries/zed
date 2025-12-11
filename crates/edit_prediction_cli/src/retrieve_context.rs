use crate::{
    example::{Example, ExampleContext},
    headless::EpAppState,
    load_project::run_load_project,
};
use anyhow::Result;
use collections::HashSet;
use edit_prediction::{DebugEvent, EditPredictionStore};
use futures::{FutureExt as _, StreamExt as _, channel::mpsc};
use gpui::{AsyncApp, Entity, Task};
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

    wait_for_language_server_to_start(example, &project, &state.buffer, &mut cx).await;

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

async fn wait_for_language_server_to_start(
    example: &Example,
    project: &Entity<Project>,
    buffer: &Entity<Buffer>,
    cx: &mut AsyncApp,
) {
    let Some(language_id) = buffer
        .read_with(cx, |buffer, _cx| {
            buffer.language().map(|language| language.id())
        })
        .unwrap()
    else {
        panic!("No language for {:?}", example.cursor_path);
    };

    let mut ready_languages = HashSet::default();
    let log_prefix = format!("{} | ", example.name);
    if !ready_languages.contains(&language_id) {
        wait_for_lang_server(&project, &buffer, log_prefix, cx)
            .await
            .unwrap();
        ready_languages.insert(language_id);
    }

    let lsp_store = project
        .read_with(cx, |project, _cx| project.lsp_store())
        .unwrap();

    // hacky wait for buffer to be registered with the language server
    for _ in 0..100 {
        if lsp_store
            .update(cx, |lsp_store, cx| {
                buffer.update(cx, |buffer, cx| {
                    lsp_store
                        .language_servers_for_local_buffer(&buffer, cx)
                        .next()
                        .map(|(_, language_server)| language_server.server_id())
                })
            })
            .unwrap()
            .is_some()
        {
            return;
        } else {
            cx.background_executor()
                .timer(Duration::from_millis(10))
                .await;
        }
    }

    panic!("No language server found for buffer");
}

pub fn wait_for_lang_server(
    project: &Entity<Project>,
    buffer: &Entity<Buffer>,
    log_prefix: String,
    cx: &mut AsyncApp,
) -> Task<Result<()>> {
    eprintln!("{}⏵ Waiting for language server", log_prefix);

    let (mut tx, mut rx) = mpsc::channel(1);

    let lsp_store = project
        .read_with(cx, |project, _| project.lsp_store())
        .unwrap();

    let has_lang_server = buffer
        .update(cx, |buffer, cx| {
            lsp_store.update(cx, |lsp_store, cx| {
                lsp_store
                    .language_servers_for_local_buffer(buffer, cx)
                    .next()
                    .is_some()
            })
        })
        .unwrap_or(false);

    if has_lang_server {
        project
            .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
            .unwrap()
            .detach();
    }
    let (mut added_tx, mut added_rx) = mpsc::channel(1);

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
            let buffer = buffer.clone();
            move |project, event, cx| match event {
                project::Event::LanguageServerAdded(_, _, _) => {
                    let buffer = buffer.clone();
                    project
                        .update(cx, |project, cx| project.save_buffer(buffer, cx))
                        .detach();
                    added_tx.try_send(()).ok();
                }
                project::Event::DiskBasedDiagnosticsFinished { .. } => {
                    tx.try_send(()).ok();
                }
                _ => {}
            }
        }),
    ];

    cx.spawn(async move |cx| {
        if !has_lang_server {
            // some buffers never have a language server, so this aborts quickly in that case.
            let timeout = cx.background_executor().timer(Duration::from_secs(500));
            futures::select! {
                _ = added_rx.next() => {},
                _ = timeout.fuse() => {
                    anyhow::bail!("Waiting for language server add timed out after 5 seconds");
                }
            };
        }
        let timeout = cx.background_executor().timer(Duration::from_secs(60 * 5));
        let result = futures::select! {
            _ = rx.next() => {
                eprintln!("{}⚑ Language server idle", log_prefix);
                anyhow::Ok(())
            },
            _ = timeout.fuse() => {
                anyhow::bail!("LSP wait timed out after 5 minutes");
            }
        };
        drop(subscriptions);
        result
    })
}
