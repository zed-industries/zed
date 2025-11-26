// TODO kb move impl LspStore here all semantic token-related methods

use std::{iter::Peekable, slice::ChunksExact, sync::Arc};

use anyhow::{Context as _, Result};

use clock::Global;
use collections::{HashMap, IndexMap};
use futures::{
    FutureExt as _,
    future::{Shared, join_all},
};
use gpui::{AsyncApp, Context, Entity, Task};
use language::Buffer;
use lsp::{AdapterServerCapabilities, LanguageServerId};
use rpc::{TypedEnvelope, proto};
use text::BufferId;

use crate::{
    LanguageServerToQuery, LspStore, LspStoreEvent,
    lsp_command::{
        LspCommand, SemanticTokensDelta, SemanticTokensDeltaResponse, SemanticTokensEdit,
        SemanticTokensFull,
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
        let buffer_id = buffer.read(cx).remote_id();
        // TODO kb this won't work on remote, cannot make accents on server_id
        let server_ids = self.local_lsp_servers_for_buffer(&buffer, cx);

        // If there are no servers yet, don't try and debounce. This makes startup quicker.
        if server_ids.is_empty() {
            return Task::ready(Ok(Default::default())).shared();
        }

        let latest_lsp_data = self.latest_lsp_data(&buffer, cx);
        if refresh.is_some() {
            latest_lsp_data.semantic_tokens = None;
        }
        let semantic_tokens_data = latest_lsp_data.semantic_tokens.get_or_insert_default();

        if let Some((updating_for, task)) = &semantic_tokens_data.update
            && !version_queried_for.changed_since(updating_for)
        {
            return task.clone();
        }

        let request_ids = semantic_tokens_data
            .buffer_tokens
            .servers
            .iter()
            .filter_map(|(&server_id, data)| Some((server_id, data.result_id.as_ref()?.clone())))
            .collect::<HashMap<_, _>>();

        let tasks = join_all(server_ids.into_iter().map(|server_id| {
            if let Some(result_id) = request_ids.get(&server_id).cloned() {
                let request = SemanticTokensDelta {
                    previous_result_id: result_id,
                };

                if self
                    .lsp_server_capabilities
                    .get(&server_id)
                    .is_some_and(|caps| {
                        request.check_capabilities(AdapterServerCapabilities {
                            server_capabilities: caps.clone(),
                            code_action_kinds: None,
                        })
                    })
                {
                    return self.fetch_semantic_tokens_delta(
                        buffer.clone(),
                        server_id,
                        request,
                        cx,
                    );
                }
            }

            self.fetch_semantic_tokens_full(buffer.clone(), server_id, cx)
        }));

        let task: SemanticTokensTask = cx
            .spawn(async move |lsp_store, cx| {
                tasks
                    .await
                    .into_iter()
                    .collect::<anyhow::Result<()>>()
                    .map_err(Arc::new)?;

                lsp_store
                    .update(cx, |lsp_store, _| {
                        if let Some(lsp_data) = lsp_store.current_lsp_data(buffer_id) {
                            lsp_data
                                .semantic_tokens
                                .as_ref()
                                .unwrap()
                                .buffer_tokens
                                .clone()
                        } else {
                            Default::default()
                        }
                    })
                    .map_err(Arc::new)
            })
            .shared();

        let semantic_tokens_data = self
            .latest_lsp_data(&buffer, cx)
            .semantic_tokens
            .get_or_insert_default();
        semantic_tokens_data.update = Some((version_queried_for, task.clone()));

        task
    }

    fn fetch_semantic_tokens_full(
        &mut self,
        buffer: Entity<Buffer>,
        server: LanguageServerId,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let buffer_id = buffer.read(cx).remote_id();

        self.send_semantic_tokens_request(
            buffer,
            cx,
            server,
            SemanticTokensFull,
            move |response, store| {
                // TODO kb here and below: this is racy, as the document version could have changed already
                if let Some(lsp_data) = store.current_lsp_data(buffer_id) {
                    let semantic_tokens_data = lsp_data.semantic_tokens.get_or_insert_default();

                    let semantic_tokens =
                        ServerSemanticTokens::from_full(response.data, response.id);

                    semantic_tokens_data
                        .buffer_tokens
                        .servers
                        .insert(server, semantic_tokens);
                }
            },
        )
    }

    fn fetch_semantic_tokens_delta(
        &mut self,
        buffer: Entity<Buffer>,
        server: LanguageServerId,
        request: SemanticTokensDelta,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let buffer_id = buffer.read(cx).remote_id();

        self.send_semantic_tokens_request(buffer, cx, server, request, move |response, store| {
            if let Some(lsp_data) = store.current_lsp_data(buffer_id) {
                let semantic_tokens_data = lsp_data.semantic_tokens.get_or_insert_default();

                match response {
                    SemanticTokensDeltaResponse::Full { data, id } => {
                        semantic_tokens_data
                            .buffer_tokens
                            .servers
                            .insert(server, ServerSemanticTokens::from_full(data, id));
                    }
                    SemanticTokensDeltaResponse::Delta { edits, id } => {
                        // If we don't have tokens for this server, we shouldn't have sent the request
                        // in the first place.
                        if let Some(tokens) =
                            semantic_tokens_data.buffer_tokens.servers.get_mut(&server)
                        {
                            tokens.result_id = id;
                            tokens.apply(&edits);
                        }
                    }
                }
            }
        })
    }

    pub(crate) fn send_semantic_tokens_request<R: LspCommand>(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
        server: LanguageServerId,
        request: R,
        handle_response: impl FnOnce(<R as LspCommand>::Response, &mut LspStore) + 'static,
    ) -> Task<anyhow::Result<()>> {
        if self.upstream_client().is_some() {
            // TODO kb Semantic tokens on remote servers.
            return Task::ready(Ok(()));
        } else {
            let lsp_request_task =
                self.request_lsp(buffer, LanguageServerToQuery::Other(server), request, cx);
            cx.spawn(async move |store, cx| {
                let response = lsp_request_task
                    .await
                    .context("semantic tokens LSP request")?;

                store.upgrade().unwrap().update(cx, move |store, _| {
                    handle_response(response, store);
                })
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
        })?;
        Ok(proto::Ack {})
    }
}

pub type SemanticTokensTask =
    Shared<Task<std::result::Result<BufferSemanticTokens, Arc<anyhow::Error>>>>;

#[derive(Default, Debug)]
pub struct SemanticTokensData {
    pub(super) buffer_tokens: BufferSemanticTokens,
    update: Option<(Global, SemanticTokensTask)>,
}

/// All the semantic token tokens for a buffer.
///
/// This aggregates semantic tokens from multiple language servers in a specific order.
/// Semantic tokens later in the list will override earlier ones in case of overlap.
#[derive(Default, Debug, Clone)]
pub struct BufferSemanticTokens {
    // TODO kb why index map is needed?
    pub servers: IndexMap<lsp::LanguageServerId, ServerSemanticTokens>,
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

    pub(crate) result_id: Option<String>,
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
    pub fn from_full(data: Vec<u32>, result_id: Option<String>) -> Self {
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
            servers: IndexMap::from_iter([
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
