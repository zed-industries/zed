use std::{collections::hash_map, iter::Peekable, slice::ChunksExact, sync::Arc};

use anyhow::Result;

use clock::Global;
use collections::HashMap;
use futures::{
    FutureExt as _,
    future::{Shared, join_all},
};
use gpui::{App, AppContext, AsyncApp, Context, Entity, SharedString, Task};
use language::Buffer;
use lsp::{AdapterServerCapabilities, LSP_REQUEST_TIMEOUT, LanguageServerId};
use rpc::{TypedEnvelope, proto};
use text::BufferId;
use util::ResultExt as _;

use crate::{
    LanguageServerToQuery, LspStore, LspStoreEvent,
    lsp_command::{
        LspCommand, SemanticTokensDelta, SemanticTokensEdit, SemanticTokensFull,
        SemanticTokensResponse,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct RefreshForServer {
    pub server_id: LanguageServerId,
    pub request_id: Option<usize>,
}

impl LspStore {
    pub fn current_semantic_tokens(&self, buffer: BufferId) -> Option<BufferSemanticTokens> {
        Some(
            self.lsp_data
                .get(&buffer)?
                .semantic_tokens
                .as_ref()?
                .buffer_tokens
                .clone(),
        )
    }

    pub fn semantic_tokens(
        &mut self,
        buffer: Entity<Buffer>,
        refresh: Option<RefreshForServer>,
        cx: &mut Context<Self>,
    ) -> SemanticTokensTask {
        let version_queried_for = buffer.read(cx).version();
        let latest_lsp_data = self.latest_lsp_data(&buffer, cx);
        let semantic_tokens_data = latest_lsp_data.semantic_tokens.get_or_insert_default();
        if let Some(refresh) = refresh {
            let mut invalidate_cache = true;
            match semantic_tokens_data
                .latest_invalidation_requests
                .entry(refresh.server_id)
            {
                hash_map::Entry::Occupied(mut o) => {
                    if refresh.request_id > *o.get() {
                        o.insert(refresh.request_id);
                    } else {
                        invalidate_cache = false;
                    }
                }
                hash_map::Entry::Vacant(v) => {
                    v.insert(refresh.request_id);
                }
            }

            if invalidate_cache {
                let old_data = std::mem::take(semantic_tokens_data);
                semantic_tokens_data.latest_invalidation_requests =
                    old_data.latest_invalidation_requests;
            }
        }

        if let Some((updating_for, task)) = &semantic_tokens_data.update
            && !version_queried_for.changed_since(updating_for)
        {
            return task.clone();
        }

        let new_tokens = self.fetch_semantic_tokens_for_buffer(
            &buffer,
            refresh.map(|refresh| refresh.server_id),
            cx,
        );

        let task_buffer = buffer.clone();
        let task_version_queried_for = version_queried_for.clone();
        let task: SemanticTokensTask = cx
            .spawn(async move |lsp_store, cx| {
                let buffer = task_buffer;
                let version_queried_for = task_version_queried_for;
                let new_tokens = new_tokens.await.unwrap_or_default();
                let buffer_tokens = lsp_store
                    .update(cx, |lsp_store, cx| {
                        let lsp_data = lsp_store.latest_lsp_data(&buffer, cx);
                        let semantic_tokens_data = lsp_data.semantic_tokens.get_or_insert_default();

                        if version_queried_for == lsp_data.buffer_version {
                            for (server_id, new_tokens_response) in new_tokens {
                                match new_tokens_response {
                                    SemanticTokensResponse::Full { data, result_id } => {
                                        semantic_tokens_data.buffer_tokens.servers.insert(
                                            server_id,
                                            ServerSemanticTokens::from_full(data, result_id),
                                        );
                                    }
                                    SemanticTokensResponse::Delta { edits, result_id } => {
                                        if let Some(tokens) = semantic_tokens_data
                                            .buffer_tokens
                                            .servers
                                            .get_mut(&server_id)
                                        {
                                            tokens.result_id = result_id;
                                            tokens.apply(&edits);
                                        }
                                    }
                                }
                            }
                        }
                        semantic_tokens_data.buffer_tokens.clone()
                    })
                    .map_err(Arc::new)?;
                Ok(buffer_tokens)
            })
            .shared();

        self.latest_lsp_data(&buffer, cx)
            .semantic_tokens
            .get_or_insert_default()
            .update = Some((version_queried_for, task.clone()));

        task
    }

    pub(crate) fn fetch_semantic_tokens_for_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        for_server: Option<LanguageServerId>,
        cx: &mut Context<Self>,
    ) -> Task<Option<HashMap<LanguageServerId, SemanticTokensResponse>>> {
        if let Some((client, upstream_project_id)) = self.upstream_client() {
            let request = SemanticTokensFull { for_server };
            if !self.is_capable_for_proto_request(buffer, &request, cx) {
                return Task::ready(None);
            }

            let request_task = client.request_lsp(
                upstream_project_id,
                None,
                LSP_REQUEST_TIMEOUT,
                cx.background_executor().clone(),
                request.to_proto(upstream_project_id, buffer.read(cx)),
            );
            let buffer = buffer.clone();
            cx.spawn(async move |weak_lsp_store, cx| {
                let lsp_store = weak_lsp_store.upgrade()?;
                let tokens = join_all(
                    request_task
                        .await
                        .log_err()
                        .flatten()
                        .map(|response| response.payload)
                        .unwrap_or_default()
                        .into_iter()
                        .map(|response| {
                            let server_id = LanguageServerId::from_proto(response.server_id);
                            let response = request.response_from_proto(
                                response.response,
                                lsp_store.clone(),
                                buffer.clone(),
                                cx.clone(),
                            );
                            async move {
                                match response.await {
                                    Ok(tokens) => Some((server_id, tokens)),
                                    Err(e) => {
                                        log::error!("Failed to query remote semantic tokens for server {server_id:?}: {e:#}");
                                        None
                                    }
                                }
                            }
                        }),
                )
                .await
                .into_iter()
                .flatten()
                .collect();
                Some(tokens)
            })
        } else {
            let token_tasks = self
                .local_lsp_servers_for_buffer(&buffer, cx)
                .into_iter()
                .filter(|&server_id| {
                    for_server.is_none_or(|for_server_id| for_server_id == server_id)
                })
                .filter_map(|server_id| {
                    let capabilities = AdapterServerCapabilities {
                        server_capabilities: self.lsp_server_capabilities.get(&server_id)?.clone(),
                        code_action_kinds: None,
                    };
                    let request_task = match self.semantic_tokens_result_id(server_id, buffer, cx) {
                        Some(result_id) => {
                            let delta_request = SemanticTokensDelta {
                                previous_result_id: result_id,
                            };
                            if !delta_request.check_capabilities(capabilities.clone()) {
                                let full_request = SemanticTokensFull {
                                    for_server: Some(server_id),
                                };
                                if !full_request.check_capabilities(capabilities) {
                                    return None;
                                }

                                self.request_lsp(
                                    buffer.clone(),
                                    LanguageServerToQuery::Other(server_id),
                                    full_request,
                                    cx,
                                )
                            } else {
                                self.request_lsp(
                                    buffer.clone(),
                                    LanguageServerToQuery::Other(server_id),
                                    delta_request,
                                    cx,
                                )
                            }
                        }
                        None => {
                            let request = SemanticTokensFull {
                                for_server: Some(server_id),
                            };
                            if !request.check_capabilities(capabilities) {
                                return None;
                            }
                            self.request_lsp(
                                buffer.clone(),
                                LanguageServerToQuery::Other(server_id),
                                request,
                                cx,
                            )
                        }
                    };
                    Some(async move { (server_id, request_task.await) })
                })
                .collect::<Vec<_>>();

            cx.background_spawn(async move {
                Some(
                    join_all(token_tasks)
                        .await
                        .into_iter()
                        .flat_map(|(server_id, response)| {
                            match response {
                                Ok(tokens) => Some((server_id, tokens)),
                                Err(e) => {
                                    log::error!("Failed to query remote semantic tokens for server {server_id:?}: {e:#}");
                                    None
                                }
                            }
                        })
                        .collect()
                )
            })
        }
    }

    pub(crate) async fn handle_refresh_semantic_tokens(
        lsp_store: Entity<Self>,
        envelope: TypedEnvelope<proto::RefreshSemanticTokens>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        lsp_store.update(&mut cx, |_, cx| {
            cx.emit(LspStoreEvent::RefreshSemanticTokens {
                server_id: LanguageServerId::from_proto(envelope.payload.server_id),
                request_id: envelope.payload.request_id.map(|id| id as usize),
            });
        });
        Ok(proto::Ack {})
    }

    fn semantic_tokens_result_id(
        &mut self,
        server_id: LanguageServerId,
        buffer: &Entity<Buffer>,
        cx: &mut App,
    ) -> Option<SharedString> {
        self.latest_lsp_data(buffer, cx)
            .semantic_tokens
            .as_ref()?
            .buffer_tokens
            .servers
            .get(&server_id)?
            .result_id
            .clone()
    }
}

pub type SemanticTokensTask =
    Shared<Task<std::result::Result<BufferSemanticTokens, Arc<anyhow::Error>>>>;

#[derive(Default, Debug)]
pub struct SemanticTokensData {
    pub(super) buffer_tokens: BufferSemanticTokens,
    pub(super) latest_invalidation_requests: HashMap<LanguageServerId, Option<usize>>,
    update: Option<(Global, SemanticTokensTask)>,
}

/// All the semantic token tokens for a buffer.
///
/// This aggregates semantic tokens from multiple language servers in a specific order.
/// Semantic tokens later in the list will override earlier ones in case of overlap.
#[derive(Default, Debug, Clone)]
pub struct BufferSemanticTokens {
    pub servers: HashMap<lsp::LanguageServerId, ServerSemanticTokens>,
}

struct BufferSemanticTokensIter<'a> {
    iters: Vec<(lsp::LanguageServerId, Peekable<SemanticTokensIter<'a>>)>,
}

/// All the semantic tokens for a buffer, from a single language server.
#[derive(Debug, Clone)]
pub struct ServerSemanticTokens {
    /// Each value is:
    /// data[5*i] - deltaLine: token line number, relative to the start of the previous token
    /// data[5*i+1] - deltaStart: token start character, relative to the start of the previous token (relative to 0 or the previous token’s start if they are on the same line)
    /// data[5*i+2] - length: the length of the token.
    /// data[5*i+3] - tokenType: will be looked up in SemanticTokensLegend.tokenTypes. We currently ask that tokenType < 65536.
    /// data[5*i+4] - tokenModifiers: each set bit will be looked up in SemanticTokensLegend.tokenModifiers
    ///
    /// See https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/ for more.
    data: Vec<u32>,

    pub(crate) result_id: Option<SharedString>,
}

pub struct SemanticTokensIter<'a> {
    prev: Option<(u32, u32)>,
    data: ChunksExact<'a, u32>,
}

// A single item from `data`.
struct SemanticTokenValue {
    delta_line: u32,
    delta_start: u32,
    length: u32,
    token_type: u32,
    token_modifiers: u32,
}

/// A semantic token, independent of its position.
#[derive(Debug, PartialEq, Eq)]
pub struct SemanticToken {
    pub line: u32,
    pub start: u32,
    pub length: u32,
    pub token_type: u32,
    pub token_modifiers: u32,
}

impl BufferSemanticTokens {
    pub fn all_tokens(&self) -> impl Iterator<Item = (lsp::LanguageServerId, SemanticToken)> {
        let iters = self
            .servers
            .iter()
            .map(|(server_id, tokens)| (*server_id, tokens.tokens().peekable()))
            .collect();

        BufferSemanticTokensIter { iters }
    }
}

impl ServerSemanticTokens {
    pub fn from_full(data: Vec<u32>, result_id: Option<SharedString>) -> Self {
        ServerSemanticTokens { data, result_id }
    }

    pub(crate) fn apply(&mut self, edits: &[SemanticTokensEdit]) {
        for edit in edits {
            let start = edit.start as usize;
            let end = start + edit.delete_count as usize;
            self.data.splice(start..end, edit.data.iter().copied());
        }
    }

    pub fn tokens(&self) -> SemanticTokensIter<'_> {
        SemanticTokensIter {
            prev: None,
            data: self.data.chunks_exact(5),
        }
    }
}

// TODO kb why is it necessary?
// Preform the data in the task instead?
impl Iterator for BufferSemanticTokensIter<'_> {
    type Item = (lsp::LanguageServerId, SemanticToken);

    fn next(&mut self) -> Option<Self::Item> {
        let (i, _) = self
            .iters
            // TODO kb can we avoid re-iterating each time?
            .iter_mut()
            .enumerate()
            .filter_map(|(i, (_, iter))| iter.peek().map(|peeked| (i, peeked)))
            .min_by_key(|(_, tok)| (tok.line, tok.start))?;

        let (id, iter) = &mut self.iters[i];
        Some((*id, iter.next()?))
    }
}

impl Iterator for SemanticTokensIter<'_> {
    type Item = SemanticToken;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.data.next()?;
        let token = SemanticTokenValue {
            delta_line: chunk[0],
            delta_start: chunk[1],
            length: chunk[2],
            token_type: chunk[3],
            token_modifiers: chunk[4],
        };

        let (line, start) = if let Some((last_line, last_start)) = self.prev {
            let line = last_line + token.delta_line;
            let start = if token.delta_line == 0 {
                last_start + token.delta_start
            } else {
                token.delta_start
            };
            (line, start)
        } else {
            (token.delta_line, token.delta_start)
        };

        self.prev = Some((line, start));

        Some(SemanticToken {
            line,
            start,
            length: token.length,
            token_type: token.token_type,
            token_modifiers: token.token_modifiers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sample_tokens() {
        // Example from the spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_semanticTokens
        let tokens = ServerSemanticTokens::from_full(
            vec![2, 5, 3, 0, 3, 0, 5, 4, 1, 0, 3, 2, 7, 2, 0],
            None,
        )
        .tokens()
        .collect::<Vec<SemanticToken>>();

        // The spec uses 1-based line numbers, and 0-based character numbers. This test uses 0-based for both.
        assert_eq!(
            tokens,
            &[
                SemanticToken {
                    line: 2,
                    start: 5,
                    length: 3,
                    token_type: 0,
                    token_modifiers: 3
                },
                SemanticToken {
                    line: 2,
                    start: 10,
                    length: 4,
                    token_type: 1,
                    token_modifiers: 0
                },
                SemanticToken {
                    line: 5,
                    start: 2,
                    length: 7,
                    token_type: 2,
                    token_modifiers: 0
                }
            ]
        );
    }

    #[test]
    fn iterate_all_tokens() {
        // A token at 0,0 and at 1,0
        let tokens_1 = ServerSemanticTokens::from_full(vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0], None);
        // A token at 0,5 and at 2,10
        let tokens_2 = ServerSemanticTokens::from_full(vec![0, 5, 0, 0, 0, 2, 10, 0, 0, 0], None);

        let buffer_tokens = BufferSemanticTokens {
            servers: HashMap::from_iter([
                (lsp::LanguageServerId(1), tokens_1),
                (lsp::LanguageServerId(2), tokens_2),
            ]),
        };

        let all_tokens = buffer_tokens
            .all_tokens()
            .map(|(server, tok)| (server, tok.line, tok.start))
            .collect::<Vec<_>>();
        assert_eq!(
            all_tokens,
            [
                (lsp::LanguageServerId(1), 0, 0),
                (lsp::LanguageServerId(2), 0, 5),
                (lsp::LanguageServerId(1), 1, 0),
                (lsp::LanguageServerId(2), 2, 10),
            ]
        )
    }
}
