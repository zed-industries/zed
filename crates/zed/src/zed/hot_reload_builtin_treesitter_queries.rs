use anyhow::Result;
use fs::Fs;
use futures::{StreamExt, channel::mpsc};
use gpui::{App, AppContext, AsyncApp, Entity};
use language::{Buffer, LanguageRegistry};
use languages::{HOT_RELOAD_BUILTIN_TREE_SITTER_QUERIES, TREE_SITTER_QUERIES_SOURCE_PATH};
use node_runtime::NodeRuntime;
use std::{env, path::PathBuf, sync::Arc, time::Duration};
use util::ResultExt as _;
use workspace::Workspace;

pub(crate) fn init(languages: Arc<LanguageRegistry>, node: NodeRuntime, cx: &mut App) {
    let Ok(zed_repo_path) = env::var(HOT_RELOAD_BUILTIN_TREE_SITTER_QUERIES) else {
        return;
    };
    let zed_repo_path = PathBuf::from(zed_repo_path);
    let fs = <dyn Fs>::global(cx);
    let (tx, mut rx) = mpsc::unbounded();
    cx.spawn(
        async move |cx| match setup_watches(zed_repo_path, tx, fs, cx).await {
            Ok(()) => {
                while let Some(()) = rx.next().await {
                    cx.update(|cx| reload_treesitter_queries(languages.clone(), node.clone(), cx))
                        .ok();
                }
            }
            Err(err) => log::error!(
                "Failed to setup watches to handle {}: {}",
                err,
                HOT_RELOAD_BUILTIN_TREE_SITTER_QUERIES
            ),
        },
    )
    .detach();
}

async fn setup_watches(
    zed_repo_path: PathBuf,
    tx: mpsc::UnboundedSender<()>,
    fs: Arc<dyn Fs>,
    cx: &mut AsyncApp,
) -> Result<()> {
    let mut queries_dir = zed_repo_path;
    queries_dir.push(TREE_SITTER_QUERIES_SOURCE_PATH);
    let mut paths = fs.read_dir(&queries_dir).await?;
    while let Some(path) = paths.next().await {
        let path = path?;
        if fs.is_dir(&path).await {
            let tx = tx.clone();
            let fs = fs.clone();
            cx.background_spawn(async move {
                let (events, _) = fs.watch(&path, Duration::from_millis(100)).await;
                futures::pin_mut!(events);
                while let Some(event_batch) = events.next().await {
                    let query_modified = event_batch
                        .into_iter()
                        .any(|event| event.path.extension().is_some_and(|ext| ext == "scm"));
                    if query_modified {
                        tx.unbounded_send(()).log_err();
                    }
                }
            })
            .detach();
        }
    }
    log::info!(
        "Watching {} for changes due to use of {}",
        queries_dir.display(),
        HOT_RELOAD_BUILTIN_TREE_SITTER_QUERIES
    );
    Ok(())
}

fn reload_treesitter_queries(languages: Arc<LanguageRegistry>, node: NodeRuntime, cx: &mut App) {
    languages::init(languages.clone(), node, cx);
    for window in cx.windows() {
        if let Some(workspace) = window.downcast::<Workspace>()
            && let Ok(workspace) = workspace.entity(cx)
        {
            let buffers = workspace
                .read(cx)
                .project()
                .read(cx)
                .buffer_store()
                .read(cx)
                .buffers()
                .collect::<Vec<_>>();
            for buffer in buffers {
                reset_buffer_language(buffer, languages.clone(), cx);
            }
        }
    }
}

fn reset_buffer_language(buffer: Entity<Buffer>, languages: Arc<LanguageRegistry>, cx: &App) {
    if let Some(old_language) = buffer.read(cx).language() {
        let old_language_name = old_language.name().to_string();
        let new_language = cx.background_spawn({
            let old_language_name = old_language_name.clone();
            async move { languages.language_for_name(&old_language_name).await }
        });
        cx.spawn(async move |cx_wat| match new_language.await {
            Ok(new_language) => {
                buffer
                    .update(cx_wat, |buffer, cx| {
                        buffer.set_language(Some(new_language), cx);
                    })
                    .ok();
            }
            Err(err) => {
                log::warn!(
                    "While handling {}, failed to update buffer with language \"{}\" \
                    (probably just not a built-in language): {}",
                    HOT_RELOAD_BUILTIN_TREE_SITTER_QUERIES,
                    old_language_name,
                    err
                );
            }
        })
        .detach();
    }
}
