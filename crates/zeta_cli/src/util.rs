use anyhow::{Result, anyhow};
use futures::channel::mpsc;
use futures::{FutureExt as _, StreamExt as _};
use gpui::{AsyncApp, Entity, Task};
use language::{Buffer, LanguageId, LanguageServerId, ParseStatus};
use project::{Project, ProjectPath, Worktree};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use util::rel_path::RelPath;

pub fn open_buffer(
    project: Entity<Project>,
    worktree: Entity<Worktree>,
    path: Arc<RelPath>,
    cx: &AsyncApp,
) -> Task<Result<Entity<Buffer>>> {
    cx.spawn(async move |cx| {
        let project_path = worktree.read_with(cx, |worktree, _cx| ProjectPath {
            worktree_id: worktree.id(),
            path,
        })?;

        let buffer = project
            .update(cx, |project, cx| project.open_buffer(project_path, cx))?
            .await?;

        let mut parse_status = buffer.read_with(cx, |buffer, _cx| buffer.parse_status())?;
        while *parse_status.borrow() != ParseStatus::Idle {
            parse_status.changed().await?;
        }

        Ok(buffer)
    })
}

pub async fn open_buffer_with_language_server(
    project: Entity<Project>,
    worktree: Entity<Worktree>,
    path: Arc<RelPath>,
    ready_languages: &mut HashSet<LanguageId>,
    cx: &mut AsyncApp,
) -> Result<(Entity<Entity<Buffer>>, LanguageServerId, Entity<Buffer>)> {
    let buffer = open_buffer(project.clone(), worktree, path.clone(), cx).await?;

    let (lsp_open_handle, path_style) = project.update(cx, |project, cx| {
        (
            project.register_buffer_with_language_servers(&buffer, cx),
            project.path_style(cx),
        )
    })?;

    let Some(language_id) = buffer.read_with(cx, |buffer, _cx| {
        buffer.language().map(|language| language.id())
    })?
    else {
        return Err(anyhow!("No language for {}", path.display(path_style)));
    };

    let log_prefix = path.display(path_style);
    if !ready_languages.contains(&language_id) {
        wait_for_lang_server(&project, &buffer, log_prefix.into_owned(), cx).await?;
        ready_languages.insert(language_id);
    }

    let lsp_store = project.read_with(cx, |project, _cx| project.lsp_store())?;

    // hacky wait for buffer to be registered with the language server
    for _ in 0..100 {
        let Some(language_server_id) = lsp_store.update(cx, |lsp_store, cx| {
            buffer.update(cx, |buffer, cx| {
                lsp_store
                    .language_servers_for_local_buffer(&buffer, cx)
                    .next()
                    .map(|(_, language_server)| language_server.server_id())
            })
        })?
        else {
            cx.background_executor()
                .timer(Duration::from_millis(10))
                .await;
            continue;
        };

        return Ok((lsp_open_handle, language_server_id, buffer));
    }

    return Err(anyhow!("No language server found for buffer"));
}

// TODO: Dedupe with similar function in crates/eval/src/instance.rs
pub fn wait_for_lang_server(
    project: &Entity<Project>,
    buffer: &Entity<Buffer>,
    log_prefix: String,
    cx: &mut AsyncApp,
) -> Task<Result<()>> {
    println!("{}⏵ Waiting for language server", log_prefix);

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
                    println!("{}⟲ {message}", log_prefix)
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
            let timeout = cx.background_executor().timer(Duration::from_secs(5));
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
                println!("{}⚑ Language server idle", log_prefix);
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
