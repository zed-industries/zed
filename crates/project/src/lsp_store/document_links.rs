use std::ops::Range;
use std::str::FromStr as _;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use clock::Global;
use collections::HashMap;
use futures::FutureExt as _;
use futures::future::{Shared, join_all};
use gpui::{AppContext as _, AsyncApp, Context, Entity, SharedString, Task};
use language::{Buffer, point_to_lsp};
use lsp::LanguageServerId;
use lsp::request::DocumentLinkResolve;
use rpc::{TypedEnvelope, proto};
use settings::Settings as _;
use text::{Anchor, BufferId, ToPointUtf16 as _};
use util::ResultExt as _;

use crate::lsp_command::{GetDocumentLinks, LspCommand as _};
use crate::lsp_store::LspStore;
use crate::project_settings::ProjectSettings;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct DocumentLinkId(u64);

#[derive(Clone, Debug)]
pub struct LspDocumentLink {
    pub range: Range<Anchor>,
    pub target: Option<SharedString>,
    pub tooltip: Option<SharedString>,
    pub data: Option<serde_json::Value>,
    pub resolved: bool,
}

pub type BufferDocumentLinks = HashMap<LanguageServerId, HashMap<DocumentLinkId, LspDocumentLink>>;

pub(super) type DocumentLinksTask =
    Shared<Task<std::result::Result<Option<BufferDocumentLinks>, Arc<anyhow::Error>>>>;

pub type DocumentLinkResolveTask = Shared<Task<Option<(DocumentLinkId, LspDocumentLink)>>>;

#[derive(Debug, Default)]
pub(super) struct DocumentLinksData {
    pub(super) links: BufferDocumentLinks,
    pub(super) next_id: u64,
    links_update: Option<(Global, DocumentLinksTask)>,
    pub(super) link_resolves: HashMap<(LanguageServerId, DocumentLinkId), DocumentLinkResolveTask>,
}

impl DocumentLinksData {
    pub(super) fn remove_server_data(&mut self, server_id: LanguageServerId) {
        self.links.remove(&server_id);
        self.link_resolves
            .retain(|(resolved_server, _), _| *resolved_server != server_id);
    }
}

/// Mirror of [`crate::lsp_store::ResolvedHint`] for document links: callers
/// either get the resolved entry directly, an in-flight `Shared` task to await
/// (deduplicated across editors), or `None` when the cache no longer contains
/// a matching link.
pub enum ResolvedDocumentLink {
    Resolved(LspDocumentLink),
    Resolving(DocumentLinkResolveTask),
}

impl LspStore {
    /// `Some(..)` means the underlying state was actually refreshed; `None`
    /// means the fetch was skipped or failed, and the caller should keep its
    /// previous data.
    pub fn fetch_document_links(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Option<BufferDocumentLinks>> {
        let version_queried_for = buffer.read(cx).version();
        let buffer_id = buffer.read(cx).remote_id();

        let current_language_servers = self.as_local().map(|local| {
            local
                .buffers_opened_in_servers
                .get(&buffer_id)
                .cloned()
                .unwrap_or_default()
        });

        if let Some(lsp_data) = self.current_lsp_data(buffer_id)
            && let Some(cached) = &lsp_data.document_links
            && !version_queried_for.changed_since(&lsp_data.buffer_version)
        {
            let has_different_servers =
                current_language_servers.is_some_and(|current_language_servers| {
                    current_language_servers != cached.links.keys().copied().collect()
                });
            if !has_different_servers {
                return Task::ready(Some(cached.links.clone()));
            }
        }

        let links_lsp_data = self
            .latest_lsp_data(buffer, cx)
            .document_links
            .get_or_insert_default();
        if let Some((updating_for, running_update)) = &links_lsp_data.links_update
            && !version_queried_for.changed_since(updating_for)
        {
            let running = running_update.clone();
            return cx.background_spawn(async move { running.await.ok().flatten() });
        }

        let buffer = buffer.clone();
        let query_version = version_queried_for.clone();
        let new_task = cx
            .spawn(async move |lsp_store, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(30))
                    .await;

                let fetched = lsp_store
                    .update(cx, |lsp_store, cx| {
                        lsp_store.fetch_document_links_for_buffer(&buffer, cx)
                    })
                    .map_err(Arc::new)?
                    .await
                    .context("fetching document links")
                    .map_err(Arc::new);

                let fetched = match fetched {
                    Ok(fetched) => fetched,
                    Err(e) => {
                        lsp_store
                            .update(cx, |lsp_store, _| {
                                if let Some(lsp_data) = lsp_store.lsp_data.get_mut(&buffer_id)
                                    && let Some(document_links) = &mut lsp_data.document_links
                                {
                                    document_links.links_update = None;
                                }
                            })
                            .ok();
                        return Err(e);
                    }
                };

                lsp_store
                    .update(cx, |lsp_store, cx| {
                        let lsp_data = lsp_store.latest_lsp_data(&buffer, cx);
                        let links_data = lsp_data.document_links.get_or_insert_default();
                        links_data.links_update = None;

                        let Some(fetched_links) = fetched else {
                            return None;
                        };

                        let mut tagged = BufferDocumentLinks::default();
                        for (server_id, server_links) in fetched_links {
                            let mut by_id = HashMap::default();
                            by_id.reserve(server_links.len());
                            for link in server_links {
                                let id = DocumentLinkId(links_data.next_id);
                                links_data.next_id += 1;
                                by_id.insert(id, link);
                            }
                            tagged.insert(server_id, by_id);
                        }

                        if lsp_data.buffer_version == query_version {
                            for (server_id, new_links) in &tagged {
                                links_data.links.insert(*server_id, new_links.clone());
                            }
                            // The newly inserted links are unresolved by definition; drop any
                            // pending resolves that were keyed against the prior entries for
                            // those servers so callers re-issue against the fresh ids.
                            links_data.link_resolves.clear();
                            Some(links_data.links.clone())
                        } else if !lsp_data.buffer_version.changed_since(&query_version) {
                            lsp_data.buffer_version = query_version;
                            links_data.links = tagged;
                            links_data.link_resolves.clear();
                            Some(links_data.links.clone())
                        } else {
                            None
                        }
                    })
                    .map_err(Arc::new)
            })
            .shared();

        links_lsp_data.links_update = Some((version_queried_for, new_task.clone()));

        cx.background_spawn(async move { new_task.await.ok().flatten() })
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn document_links_for_buffer(&self, buffer_id: BufferId) -> Option<BufferDocumentLinks> {
        let data = self.lsp_data.get(&buffer_id)?;
        let document_links = data.document_links.as_ref()?;
        Some(document_links.links.clone())
    }

    fn fetch_document_links_for_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Option<HashMap<LanguageServerId, Vec<LspDocumentLink>>>>> {
        if let Some((client, project_id)) = self.upstream_client() {
            let request = GetDocumentLinks;
            if !self.is_capable_for_proto_request(buffer, &request, cx) {
                return Task::ready(Ok(None));
            }

            let request_timeout = ProjectSettings::get_global(cx)
                .global_lsp_settings
                .get_request_timeout();
            let request_task = client.request_lsp(
                project_id,
                None,
                request_timeout,
                cx.background_executor().clone(),
                request.to_proto(project_id, buffer.read(cx)),
            );
            let buffer = buffer.clone();
            cx.spawn(async move |weak_lsp_store, cx| {
                let Some(lsp_store) = weak_lsp_store.upgrade() else {
                    return Ok(None);
                };
                let Some(responses) = request_task.await? else {
                    return Ok(None);
                };

                let document_links = join_all(responses.payload.into_iter().map(|response| {
                    let lsp_store = lsp_store.clone();
                    let buffer = buffer.clone();
                    let cx = cx.clone();
                    async move {
                        let server_id = LanguageServerId::from_proto(response.server_id);
                        let links = GetDocumentLinks
                            .response_from_proto(response.response, lsp_store, buffer, cx)
                            .await;
                        (server_id, links)
                    }
                }))
                .await;

                let mut has_errors = false;
                let result = document_links
                    .into_iter()
                    .filter_map(|(server_id, links)| match links {
                        Ok(links) => Some((server_id, links)),
                        Err(e) => {
                            has_errors = true;
                            log::error!(
                                "Failed to fetch document links for server {server_id}: {e:#}"
                            );
                            None
                        }
                    })
                    .collect::<HashMap<_, _>>();
                anyhow::ensure!(
                    !has_errors || !result.is_empty(),
                    "Failed to fetch document links"
                );
                Ok(Some(result))
            })
        } else {
            let links_task =
                self.request_multiple_lsp_locally(buffer, None::<usize>, GetDocumentLinks, cx);
            cx.background_spawn(async move { Ok(Some(links_task.await.into_iter().collect())) })
        }
    }

    /// Returns the resolved state for a cached document link, deduplicating
    /// in-flight `documentLink/resolve` requests across editors via a `Shared`
    /// task stored on `DocumentLinksData`.
    ///
    /// `link_id` is the [`DocumentLinkId`] stamped on the cached link by
    /// [`Self::fetch_document_links`]; sibling links sharing the same buffer
    /// range are disambiguated by it. `None` is returned when the cache no
    /// longer holds a matching link (likely a version bump in between).
    pub fn resolved_document_link(
        &mut self,
        buffer: &Entity<Buffer>,
        server_id: LanguageServerId,
        link_id: DocumentLinkId,
        cx: &mut Context<Self>,
    ) -> Option<ResolvedDocumentLink> {
        let buffer_id = buffer.read(cx).remote_id();

        let document_links = self.lsp_data.get(&buffer_id)?.document_links.as_ref()?;
        let cached_link = document_links.links.get(&server_id)?.get(&link_id)?.clone();

        if cached_link.resolved {
            return Some(ResolvedDocumentLink::Resolved(cached_link));
        }

        let key = (server_id, link_id);
        if let Some(running) = document_links.link_resolves.get(&key) {
            return Some(ResolvedDocumentLink::Resolving(running.clone()));
        }

        let resolve_task = self.resolve_document_link_request(buffer, server_id, &cached_link, cx);
        let query_version = self.lsp_data.get(&buffer_id)?.buffer_version.clone();
        let resolve_task = cx
            .spawn(async move |lsp_store, cx| {
                let resolved = resolve_task.await;
                lsp_store
                    .update(cx, |lsp_store, _| {
                        let lsp_data = lsp_store.lsp_data.get_mut(&buffer_id)?;
                        if lsp_data.buffer_version != query_version {
                            return None;
                        }
                        let links_data = lsp_data.document_links.as_mut()?;
                        links_data.link_resolves.remove(&key);
                        let updated = match resolved {
                            Some(resolved) => lsp_store
                                .cache_resolved_link(buffer_id, server_id, link_id, &resolved)?,
                            None => {
                                // No further resolution is possible (no capability,
                                // missing server, or LSP error); mark as resolved so we
                                // do not keep retrying on every hover, and yield the
                                // entry as-is so awaiters can still surface it.
                                let links_data = lsp_data.document_links.as_mut()?;
                                let link =
                                    links_data.links.get_mut(&server_id)?.get_mut(&link_id)?;
                                link.resolved = true;
                                link.clone()
                            }
                        };
                        Some((link_id, updated))
                    })
                    .ok()
                    .flatten()
            })
            .shared();

        let document_links = self.lsp_data.get_mut(&buffer_id)?.document_links.as_mut()?;
        document_links
            .link_resolves
            .insert(key, resolve_task.clone());
        Some(ResolvedDocumentLink::Resolving(resolve_task))
    }

    /// Builds the LSP/proto request task for a single unresolved link. Returns
    /// a task that yields `None` when the resolve request cannot be issued
    /// (no upstream capability, no local server, or no `resolveProvider`).
    fn resolve_document_link_request(
        &self,
        buffer: &Entity<Buffer>,
        server_id: LanguageServerId,
        cached_link: &LspDocumentLink,
        cx: &mut Context<Self>,
    ) -> Task<Option<lsp::DocumentLink>> {
        let snapshot = buffer.read(cx).snapshot();
        let buffer_id = buffer.read(cx).remote_id();
        let lsp_link = lsp::DocumentLink {
            range: lsp::Range {
                start: point_to_lsp(cached_link.range.start.to_point_utf16(&snapshot)),
                end: point_to_lsp(cached_link.range.end.to_point_utf16(&snapshot)),
            },
            target: cached_link
                .target
                .as_ref()
                .and_then(|s| lsp::Uri::from_str(s).ok()),
            tooltip: cached_link.tooltip.as_deref().map(str::to_string),
            data: cached_link.data.clone(),
        };

        if let Some((upstream_client, project_id)) = self.upstream_client() {
            if !self.check_if_capable_for_proto_request(buffer, can_resolve_link, cx) {
                return Task::ready(None);
            }
            let request = proto::ResolveDocumentLink {
                project_id,
                buffer_id: buffer_id.into(),
                language_server_id: server_id.0 as u64,
                lsp_link: serde_json::to_vec(&lsp_link).unwrap_or_default(),
            };
            cx.background_spawn(async move {
                let response = upstream_client.request(request).await.log_err()?;
                serde_json::from_slice::<lsp::DocumentLink>(&response.lsp_link).log_err()
            })
        } else {
            let Some(server) = self.language_server_for_id(server_id) else {
                return Task::ready(None);
            };
            if !can_resolve_link(&server.capabilities()) {
                return Task::ready(None);
            }
            let request_timeout = ProjectSettings::get_global(cx)
                .global_lsp_settings
                .get_request_timeout();
            cx.background_spawn(async move {
                server
                    .request::<DocumentLinkResolve>(lsp_link, request_timeout)
                    .await
                    .into_response()
                    .log_err()
            })
        }
    }

    fn cache_resolved_link(
        &mut self,
        buffer_id: BufferId,
        server_id: LanguageServerId,
        link_id: DocumentLinkId,
        resolved: &lsp::DocumentLink,
    ) -> Option<LspDocumentLink> {
        let document_links = self.lsp_data.get_mut(&buffer_id)?.document_links.as_mut()?;
        let link = document_links
            .links
            .get_mut(&server_id)?
            .get_mut(&link_id)?;
        link.target = resolved.target.as_ref().map(|u| u.to_string().into());
        if let Some(tooltip) = &resolved.tooltip {
            link.tooltip = Some(tooltip.clone().into());
        }
        link.data = resolved.data.clone();
        link.resolved = true;
        Some(link.clone())
    }

    pub(super) async fn handle_resolve_document_link(
        lsp_store: Entity<Self>,
        envelope: TypedEnvelope<proto::ResolveDocumentLink>,
        mut cx: AsyncApp,
    ) -> anyhow::Result<proto::ResolveDocumentLinkResponse> {
        let lsp_link: lsp::DocumentLink = serde_json::from_slice(&envelope.payload.lsp_link)
            .context("deserializing document link to resolve")?;
        let server_id = LanguageServerId::from_proto(envelope.payload.language_server_id);

        let resolve_task = lsp_store.update(&mut cx, |lsp_store, cx| {
            let server = lsp_store
                .language_server_for_id(server_id)
                .with_context(|| format!("No language server {server_id}"))?;
            let timeout = ProjectSettings::get_global(cx)
                .global_lsp_settings
                .get_request_timeout();
            anyhow::Ok(server.request::<DocumentLinkResolve>(lsp_link, timeout))
        })?;
        let resolved = resolve_task.await.into_response()?;

        Ok(proto::ResolveDocumentLinkResponse {
            lsp_link: serde_json::to_vec(&resolved)
                .context("serializing resolved document link")?,
        })
    }
}

fn can_resolve_link(capabilities: &lsp::ServerCapabilities) -> bool {
    capabilities
        .document_link_provider
        .as_ref()
        .and_then(|opts| opts.resolve_provider)
        .unwrap_or(false)
}
