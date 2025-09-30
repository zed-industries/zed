use std::sync::Arc;
use std::time::Duration;

use crate::Editor;
use collections::HashMap;
use gpui::AsyncApp;
use gpui::{App, Entity, Task};
use itertools::Itertools;
use language::Buffer;
use language::Language;
use lsp::LanguageServerId;
use lsp::LanguageServerName;
use multi_buffer::Anchor;
use project::LanguageServerToQuery;
use project::LocationLink;
use project::Project;
use project::TaskSourceKind;
use project::lsp_store::lsp_ext_command::GetLspRunnables;
use smol::future::FutureExt as _;
use task::ResolvedTask;
use task::TaskContext;
use text::BufferId;
use ui::SharedString;
use util::ResultExt as _;

pub(crate) fn find_specific_language_server_in_selection<F>(
    editor: &Editor,
    cx: &mut App,
    filter_language: F,
    language_server_name: LanguageServerName,
) -> Option<(Anchor, Arc<Language>, LanguageServerId, Entity<Buffer>)>
where
    F: Fn(&Language) -> bool,
{
    let project = editor.project.clone()?;
    editor
        .selections
        .disjoint_anchors_arc()
        .iter()
        .filter_map(|selection| Some((selection.head(), selection.head().buffer_id?)))
        .unique_by(|(_, buffer_id)| *buffer_id)
        .find_map(|(trigger_anchor, buffer_id)| {
            let buffer = editor.buffer().read(cx).buffer(buffer_id)?;
            let language = buffer.read(cx).language_at(trigger_anchor.text_anchor)?;
            if filter_language(&language) {
                let server_id = buffer.update(cx, |buffer, cx| {
                    project
                        .read(cx)
                        .language_server_id_for_name(buffer, &language_server_name, cx)
                })?;
                Some((trigger_anchor, language, server_id, buffer))
            } else {
                None
            }
        })
}

async fn lsp_task_context(
    project: &Entity<Project>,
    buffer: &Entity<Buffer>,
    cx: &mut AsyncApp,
) -> Option<TaskContext> {
    let worktree_store = project
        .read_with(cx, |project, _| project.worktree_store())
        .ok()?;

    let worktree_abs_path = cx
        .update(|cx| {
            let worktree_id = buffer.read(cx).file().map(|f| f.worktree_id(cx));

            worktree_id
                .and_then(|worktree_id| worktree_store.read(cx).worktree_for_id(worktree_id, cx))
                .and_then(|worktree| worktree.read(cx).root_dir())
        })
        .ok()?;

    let project_env = project
        .update(cx, |project, cx| {
            project.buffer_environment(buffer, &worktree_store, cx)
        })
        .ok()?
        .await;

    Some(TaskContext {
        cwd: worktree_abs_path.map(|p| p.to_path_buf()),
        project_env: project_env.unwrap_or_default(),
        ..TaskContext::default()
    })
}

pub fn lsp_tasks(
    project: Entity<Project>,
    task_sources: &HashMap<LanguageServerName, Vec<BufferId>>,
    for_position: Option<text::Anchor>,
    cx: &mut App,
) -> Task<Vec<(TaskSourceKind, Vec<(Option<LocationLink>, ResolvedTask)>)>> {
    let lsp_task_sources = task_sources
        .iter()
        .filter_map(|(name, buffer_ids)| {
            let buffers = buffer_ids
                .iter()
                .filter(|&&buffer_id| match for_position {
                    Some(for_position) => for_position.buffer_id == Some(buffer_id),
                    None => true,
                })
                .filter_map(|&buffer_id| project.read(cx).buffer_for_id(buffer_id, cx))
                .collect::<Vec<_>>();

            let server_id = buffers.iter().find_map(|buffer| {
                project.read_with(cx, |project, cx| {
                    project.language_server_id_for_name(buffer.read(cx), name, cx)
                })
            });
            server_id.zip(Some(buffers))
        })
        .collect::<Vec<_>>();

    cx.spawn(async move |cx| {
        cx.spawn(async move |cx| {
            let mut lsp_tasks = HashMap::default();
            for (server_id, buffers) in lsp_task_sources {
                let mut new_lsp_tasks = Vec::new();
                for buffer in buffers {
                    let source_kind = match buffer.update(cx, |buffer, _| {
                        buffer.language().map(|language| language.name())
                    }) {
                        Ok(Some(language_name)) => TaskSourceKind::Lsp {
                            server: server_id,
                            language_name: SharedString::from(language_name),
                        },
                        Ok(None) => continue,
                        Err(_) => return Vec::new(),
                    };
                    let id_base = source_kind.to_id_base();
                    let lsp_buffer_context = lsp_task_context(&project, &buffer, cx)
                        .await
                        .unwrap_or_default();

                    if let Ok(runnables_task) = project.update(cx, |project, cx| {
                        let buffer_id = buffer.read(cx).remote_id();
                        project.request_lsp(
                            buffer,
                            LanguageServerToQuery::Other(server_id),
                            GetLspRunnables {
                                buffer_id,
                                position: for_position,
                            },
                            cx,
                        )
                    }) && let Some(new_runnables) = runnables_task.await.log_err()
                    {
                        new_lsp_tasks.extend(new_runnables.runnables.into_iter().filter_map(
                            |(location, runnable)| {
                                let resolved_task =
                                    runnable.resolve_task(&id_base, &lsp_buffer_context)?;
                                Some((location, resolved_task))
                            },
                        ));
                    }
                    lsp_tasks
                        .entry(source_kind)
                        .or_insert_with(Vec::new)
                        .append(&mut new_lsp_tasks);
                }
            }
            lsp_tasks.into_iter().collect()
        })
        .race({
            // `lsp::LSP_REQUEST_TIMEOUT` is larger than we want for the modal to open fast
            let timer = cx.background_executor().timer(Duration::from_millis(200));
            async move {
                timer.await;
                log::info!("Timed out waiting for LSP tasks");
                Vec::new()
            }
        })
        .await
    })
}
