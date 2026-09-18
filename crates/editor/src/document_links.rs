use collections::HashMap;
use futures::future::join_all;
use gpui::{App, Entity, Task};
use itertools::Itertools;
use language::{Buffer, BufferSnapshot};
use lsp::LanguageServerId;
use project::lsp_store::{BufferDocumentLinks, LspDocumentLink, ResolvedDocumentLink};
use settings::Settings;
use text::BufferId;
use ui::Context;

use crate::{Editor, LSP_REQUEST_DEBOUNCE_TIMEOUT, editor_settings::EditorSettings};

pub(super) struct LspDocumentLinks {
    pub(super) enabled: bool,
    pub(super) per_buffer: HashMap<BufferId, BufferDocumentLinks>,
    pub(super) refresh_task: Task<()>,
}

impl LspDocumentLinks {
    pub(super) fn new(cx: &App) -> Self {
        Self {
            enabled: EditorSettings::get_global(cx).lsp_document_links,
            per_buffer: HashMap::default(),
            refresh_task: Task::ready(()),
        }
    }
}

impl Editor {
    pub(super) fn refresh_document_links(
        &mut self,
        for_buffer: Option<BufferId>,
        cx: &mut Context<Self>,
    ) {
        if !self.lsp_data_enabled() || !self.lsp_document_links.enabled {
            return;
        }
        let Some(project) = self.project.as_ref().map(|p| p.downgrade()) else {
            return;
        };

        let buffers_to_query = self
            .visible_buffers(cx)
            .into_iter()
            .filter(|buffer| self.is_lsp_relevant(buffer.read(cx).file(), cx))
            .chain(for_buffer.and_then(|id| self.buffer.read(cx).buffer(id)))
            .filter(|buffer| {
                let id = buffer.read(cx).remote_id();
                for_buffer.is_none_or(|target| target == id)
                    && self.registered_buffers.contains_key(&id)
            })
            .unique_by(|buffer| buffer.read(cx).remote_id())
            .collect::<Vec<_>>();
        if buffers_to_query.is_empty() {
            self.lsp_document_links.refresh_task = Task::ready(());
            return;
        }

        self.lsp_document_links.refresh_task = cx.spawn(async move |editor, cx| {
            cx.background_executor()
                .timer(LSP_REQUEST_DEBOUNCE_TIMEOUT)
                .await;

            let Some(tasks_for_buffers) = project
                .update(cx, |project, cx| {
                    project.lsp_store().update(cx, |lsp_store, cx| {
                        buffers_to_query
                            .into_iter()
                            .map(|buffer| {
                                let buffer_id = buffer.read(cx).remote_id();
                                let task = lsp_store.fetch_document_links(&buffer, cx);
                                async move { (buffer_id, task.await) }
                            })
                            .collect::<Vec<_>>()
                    })
                })
                .ok()
            else {
                return;
            };

            let new_links_for_buffers = join_all(tasks_for_buffers).await;
            editor
                .update(cx, |editor, _| {
                    for (buffer_id, links) in new_links_for_buffers {
                        let Some(links) = links else {
                            continue;
                        };
                        if links.is_empty() {
                            editor.lsp_document_links.per_buffer.remove(&buffer_id);
                        } else {
                            editor
                                .lsp_document_links
                                .per_buffer
                                .insert(buffer_id, links);
                        }
                    }
                })
                .ok();
        });
    }

    /// Returns a task yielding the resolved document links covering `position`
    /// in `buffer`, paired with the language server that owns each link.
    /// Resolution is deduplicated through `LspStore`'s per-`(server_id,
    /// link_id)` `Shared` task; the editor's mirror is updated when the
    /// resolves complete so subsequent renders/hovers find resolved data
    /// without re-issuing requests.
    ///
    /// Returns `None` when nothing is cached at `position` so callers can skip
    /// spawning anything.
    pub fn document_links_at(
        &mut self,
        buffer: Entity<Buffer>,
        position: text::Anchor,
        cx: &mut Context<Self>,
    ) -> Option<Task<Vec<(LanguageServerId, LspDocumentLink)>>> {
        let buffer_id = buffer.read(cx).remote_id();
        let snapshot = buffer.read(cx).snapshot();
        let matches = self
            .lsp_document_links
            .per_buffer
            .get(&buffer_id)?
            .iter()
            .flat_map(|(server_id, per_server)| {
                per_server
                    .iter()
                    .map(move |(link_id, link)| (*server_id, *link_id, link))
            })
            .filter(|(_, _, link)| link_contains(link, &position, &snapshot))
            .map(|(server_id, link_id, link)| (server_id, link_id, link.clone()))
            .collect::<Vec<_>>();
        if matches.is_empty() {
            return None;
        }

        let project = self.project.clone()?;
        let mut resolved_links = Vec::with_capacity(matches.len());
        let mut pending = Vec::new();
        project.update(cx, |project, cx| {
            project.lsp_store().update(cx, |lsp_store, cx| {
                for (server_id, link_id, _link) in matches {
                    match lsp_store.resolved_document_link(&buffer, server_id, link_id, cx) {
                        Some(ResolvedDocumentLink::Resolved(resolved)) => {
                            resolved_links.push((server_id, link_id, resolved));
                        }
                        Some(ResolvedDocumentLink::Resolving(task)) => {
                            pending.push((server_id, task));
                        }
                        None => {
                            // Cache no longer holds the link (likely a version
                            // bump between the mirror snapshot and now); skip.
                        }
                    }
                }
            })
        });

        if pending.is_empty() {
            return Some(Task::ready(
                resolved_links
                    .into_iter()
                    .map(|(server_id, _, link)| (server_id, link))
                    .collect(),
            ));
        }

        Some(cx.spawn(async move |editor, cx| {
            let pending_results =
                join_all(pending.into_iter().map(|(server_id, task)| async move {
                    task.await.map(|(link_id, link)| (server_id, link_id, link))
                }))
                .await;
            resolved_links.extend(pending_results.into_iter().flatten());
            editor
                .update(cx, |editor, cx| {
                    if let Some(by_server) =
                        editor.lsp_document_links.per_buffer.get_mut(&buffer_id)
                    {
                        for (server_id, link_id, resolved) in &resolved_links {
                            if let Some(slot) = by_server
                                .get_mut(server_id)
                                .and_then(|links| links.get_mut(link_id))
                            {
                                *slot = resolved.clone();
                            }
                        }
                    }
                    cx.notify();
                })
                .ok();

            resolved_links
                .into_iter()
                .map(|(server_id, _, link)| (server_id, link))
                .collect()
        }))
    }
}

fn link_contains(
    link: &LspDocumentLink,
    position: &text::Anchor,
    snapshot: &BufferSnapshot,
) -> bool {
    link.range.start.cmp(position, snapshot).is_le()
        && link.range.end.cmp(position, snapshot).is_ge()
}
