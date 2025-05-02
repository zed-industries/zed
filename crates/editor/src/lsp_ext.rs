use std::sync::Arc;

use crate::Editor;
use collections::HashMap;
use futures::stream::FuturesUnordered;
use gpui::AsyncApp;
use gpui::{App, AppContext as _, Entity, Task};
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
use smol::stream::StreamExt;
use task::ResolvedTask;
use task::TaskContext;
use text::BufferId;
use util::ResultExt as _;

pub(crate) fn find_specific_language_server_in_selection<F>(
    editor: &Editor,
    cx: &mut App,
    filter_language: F,
    language_server_name: &str,
) -> Task<Option<(Anchor, Arc<Language>, LanguageServerId, Entity<Buffer>)>>
where
    F: Fn(&Language) -> bool,
{
    let Some(project) = &editor.project else {
        return Task::ready(None);
    };

    let applicable_buffers = editor
        .selections
        .disjoint_anchors()
        .iter()
        .filter(|selection| selection.start == selection.end)
        .filter_map(|selection| Some((selection.start, selection.start.buffer_id?)))
        .filter_map(|(trigger_anchor, buffer_id)| {
            let buffer = editor.buffer().read(cx).buffer(buffer_id)?;
            let language = buffer.read(cx).language_at(trigger_anchor.text_anchor)?;
            if filter_language(&language) {
                Some((trigger_anchor, buffer, language))
            } else {
                None
            }
        })
        .unique_by(|(_, buffer, _)| buffer.read(cx).remote_id())
        .collect::<Vec<_>>();

    let applicable_buffer_tasks = applicable_buffers
        .into_iter()
        .map(|(trigger_anchor, buffer, language)| {
            let task = buffer.update(cx, |buffer, cx| {
                project.update(cx, |project, cx| {
                    project.language_server_id_for_name(buffer, language_server_name, cx)
                })
            });
            (trigger_anchor, buffer, language, task)
        })
        .collect::<Vec<_>>();
    cx.background_spawn(async move {
        for (trigger_anchor, buffer, language, task) in applicable_buffer_tasks {
            if let Some(server_id) = task.await {
                return Some((trigger_anchor, language, server_id, buffer));
            }
        }

        None
    })
}

async fn lsp_task_context(
    project: &Entity<Project>,
    buffer: &Entity<Buffer>,
    cx: &mut AsyncApp,
) -> Option<TaskContext> {
    let worktree_store = project
        .update(cx, |project, _| project.worktree_store())
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
            project.buffer_environment(&buffer, &worktree_store, cx)
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
    let mut lsp_task_sources = task_sources
        .iter()
        .map(|(name, buffer_ids)| {
            let buffers = buffer_ids
                .iter()
                .filter(|&&buffer_id| match for_position {
                    Some(for_position) => for_position.buffer_id == Some(buffer_id),
                    None => true,
                })
                .filter_map(|&buffer_id| project.read(cx).buffer_for_id(buffer_id, cx))
                .collect::<Vec<_>>();
            language_server_for_buffers(project.clone(), name.clone(), buffers, cx)
        })
        .collect::<FuturesUnordered<_>>();

    cx.spawn(async move |cx| {
        let mut lsp_tasks = Vec::new();
        while let Some(server_to_query) = lsp_task_sources.next().await {
            if let Some((server_id, buffers)) = server_to_query {
                let source_kind = TaskSourceKind::Lsp(server_id);
                let id_base = source_kind.to_id_base();
                let mut new_lsp_tasks = Vec::new();
                for buffer in buffers {
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
                    }) {
                        if let Some(new_runnables) = runnables_task.await.log_err() {
                            new_lsp_tasks.extend(new_runnables.runnables.into_iter().filter_map(
                                |(location, runnable)| {
                                    let resolved_task =
                                        runnable.resolve_task(&id_base, &lsp_buffer_context)?;
                                    Some((location, resolved_task))
                                },
                            ));
                        }
                    }
                }
                lsp_tasks.push((source_kind, new_lsp_tasks));
            }
        }
        lsp_tasks
    })
}

fn language_server_for_buffers(
    project: Entity<Project>,
    name: LanguageServerName,
    candidates: Vec<Entity<Buffer>>,
    cx: &mut App,
) -> Task<Option<(LanguageServerId, Vec<Entity<Buffer>>)>> {
    cx.spawn(async move |cx| {
        for buffer in &candidates {
            let server_id = buffer
                .update(cx, |buffer, cx| {
                    project.update(cx, |project, cx| {
                        project.language_server_id_for_name(buffer, &name.0, cx)
                    })
                })
                .ok()?
                .await;
            if let Some(server_id) = server_id {
                return Some((server_id, candidates));
            }
        }
        None
    })
}
