use std::{collections::hash_map, ops::Range, slice::ChunksExact, sync::Arc};

use anyhow::Result;

use clock::Global;
use collections::HashMap;
use futures::{
    FutureExt as _,
    future::{Shared, join_all},
};
use gpui::{App, AppContext, AsyncApp, Context, Entity, ReadGlobal as _, SharedString, Task};
use itertools::Itertools;
use language::{Buffer, LanguageName, language_settings::all_language_settings};
use lsp::{AdapterServerCapabilities, LanguageServerId};
use rpc::{TypedEnvelope, proto};
use settings::{SemanticTokenRule, SemanticTokenRules, Settings as _, SettingsStore};
use smol::future::yield_now;
use text::{Anchor, Bias, OffsetUtf16, PointUtf16, Unclipped};
use util::ResultExt as _;

use crate::{
    LanguageServerToQuery, LspStore, LspStoreEvent,
    lsp_command::{
        LspCommand, SemanticTokensDelta, SemanticTokensEdit, SemanticTokensFull,
        SemanticTokensResponse,
    },
    project_settings::ProjectSettings,
};

pub(super) struct SemanticTokenConfig {
    stylizers: HashMap<(LanguageServerId, Option<LanguageName>), SemanticTokenStylizer>,
    rules: SemanticTokenRules,
    global_mode: settings::SemanticTokens,
}

impl SemanticTokenConfig {
    pub(super) fn new(cx: &App) -> Self {
        Self {
            stylizers: HashMap::default(),
            rules: ProjectSettings::get_global(cx)
                .global_lsp_settings
                .semantic_token_rules
                .clone(),
            global_mode: all_language_settings(None, cx).defaults.semantic_tokens,
        }
    }

    pub(super) fn remove_server_data(&mut self, server_id: LanguageServerId) {
        self.stylizers.retain(|&(id, _), _| id != server_id);
    }

    pub(super) fn update_rules(&mut self, new_rules: SemanticTokenRules) -> bool {
        if new_rules != self.rules {
            self.rules = new_rules;
            self.stylizers.clear();
            true
        } else {
            false
        }
    }

    pub(super) fn update_global_mode(&mut self, new_mode: settings::SemanticTokens) -> bool {
        if new_mode != self.global_mode {
            self.global_mode = new_mode;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RefreshForServer {
    pub server_id: LanguageServerId,
    pub request_id: Option<usize>,
}

impl LspStore {
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
                let SemanticTokensData {
                    raw_tokens,
                    latest_invalidation_requests: _,
                    update,
                } = semantic_tokens_data;
                *update = None;
                raw_tokens.servers.clear();
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
        let task = cx
            .spawn(async move |lsp_store, cx| {
                let buffer = task_buffer;
                let version_queried_for = task_version_queried_for;
                let res = if let Some(new_tokens) = new_tokens.await {
                    let (raw_tokens, buffer_snapshot) = lsp_store
                        .update(cx, |lsp_store, cx| {
                            let lsp_data = lsp_store.latest_lsp_data(&buffer, cx);
                            let semantic_tokens_data =
                                lsp_data.semantic_tokens.get_or_insert_default();

                            if version_queried_for == lsp_data.buffer_version {
                                for (server_id, new_tokens_response) in new_tokens {
                                    match new_tokens_response {
                                        SemanticTokensResponse::Full { data, result_id } => {
                                            semantic_tokens_data.raw_tokens.servers.insert(
                                                server_id,
                                                Arc::new(ServerSemanticTokens::from_full(
                                                    data, result_id,
                                                )),
                                            );
                                        }
                                        SemanticTokensResponse::Delta { edits, result_id } => {
                                            if let Some(tokens) = semantic_tokens_data
                                                .raw_tokens
                                                .servers
                                                .get_mut(&server_id)
                                            {
                                                let tokens = Arc::make_mut(tokens);
                                                tokens.result_id = result_id;
                                                tokens.apply(&edits);
                                            }
                                        }
                                    }
                                }
                            }
                            let buffer_snapshot =
                                buffer.read_with(cx, |buffer, _| buffer.snapshot());
                            (semantic_tokens_data.raw_tokens.clone(), buffer_snapshot)
                        })
                        .map_err(Arc::new)?;
                    Some(raw_to_buffer_semantic_tokens(raw_tokens, &buffer_snapshot).await)
                } else {
                    lsp_store.update(cx, |lsp_store, cx| {
                        if let Some(current_lsp_data) =
                            lsp_store.current_lsp_data(buffer.read(cx).remote_id())
                        {
                            if current_lsp_data.buffer_version == version_queried_for {
                                current_lsp_data.semantic_tokens = None;
                            }
                        }
                    })?;
                    None
                };
                Ok(BufferSemanticTokens { tokens: res })
            })
            .shared();

        self.latest_lsp_data(&buffer, cx)
            .semantic_tokens
            .get_or_insert_default()
            .update = Some((version_queried_for, task.clone()));

        task
    }

    pub(super) fn fetch_semantic_tokens_for_buffer(
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

            let request_timeout = ProjectSettings::get_global(cx)
                .global_lsp_settings
                .get_request_timeout();
            let request_task = client.request_lsp(
                upstream_project_id,
                None,
                request_timeout,
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
            if token_tasks.is_empty() {
                return Task::ready(None);
            }

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
            .raw_tokens
            .servers
            .get(&server_id)?
            .result_id
            .clone()
    }

    pub fn get_or_create_token_stylizer(
        &mut self,
        server_id: LanguageServerId,
        language: Option<&LanguageName>,
        cx: &mut App,
    ) -> Option<&SemanticTokenStylizer> {
        let stylizer = match self
            .semantic_token_config
            .stylizers
            .entry((server_id, language.cloned()))
        {
            hash_map::Entry::Occupied(o) => o.into_mut(),
            hash_map::Entry::Vacant(v) => {
                let tokens_provider = self
                    .lsp_server_capabilities
                    .get(&server_id)?
                    .semantic_tokens_provider
                    .as_ref()?;
                let legend = match tokens_provider {
                    lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(opts) => {
                        &opts.legend
                    }
                    lsp::SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
                        opts,
                    ) => &opts.semantic_tokens_options.legend,
                };
                let language_rules = language.and_then(|language| {
                    SettingsStore::global(cx).language_semantic_token_rules(language.as_ref())
                });
                let stylizer = SemanticTokenStylizer::new(server_id, legend, language_rules, cx);
                v.insert(stylizer)
            }
        };
        Some(stylizer)
    }
}

pub type SemanticTokensTask =
    Shared<Task<std::result::Result<BufferSemanticTokens, Arc<anyhow::Error>>>>;

#[derive(Debug, Default, Clone)]
pub struct BufferSemanticTokens {
    pub tokens: Option<HashMap<LanguageServerId, Arc<[BufferSemanticToken]>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenType(pub u32);

#[derive(Debug, Clone)]
pub struct BufferSemanticToken {
    pub range: Range<Anchor>,
    pub token_type: TokenType,
    pub token_modifiers: u32,
}

pub struct SemanticTokenStylizer {
    server_id: LanguageServerId,
    rules_by_token_type: HashMap<TokenType, Vec<SemanticTokenRule>>,
    token_type_names: HashMap<TokenType, SharedString>,
    modifier_mask: HashMap<SharedString, u32>,
}

impl SemanticTokenStylizer {
    pub fn new(
        server_id: LanguageServerId,
        legend: &lsp::SemanticTokensLegend,
        language_rules: Option<&SemanticTokenRules>,
        cx: &App,
    ) -> Self {
        let token_types: HashMap<TokenType, SharedString> = legend
            .token_types
            .iter()
            .enumerate()
            .map(|(i, token_type)| {
                (
                    TokenType(i as u32),
                    SharedString::from(token_type.as_str().to_string()),
                )
            })
            .collect();
        let modifier_mask: HashMap<SharedString, u32> = legend
            .token_modifiers
            .iter()
            .enumerate()
            .map(|(i, modifier)| (SharedString::from(modifier.as_str().to_string()), 1 << i))
            .collect();

        let global_rules = &ProjectSettings::get_global(cx)
            .global_lsp_settings
            .semantic_token_rules;

        let rules_by_token_type = token_types
            .iter()
            .map(|(index, token_type_name)| {
                let filter = |rule: &&SemanticTokenRule| {
                    rule.token_type
                        .as_ref()
                        .is_none_or(|rule_token_type| rule_token_type == token_type_name.as_ref())
                };
                let matching_rules: Vec<SemanticTokenRule> = global_rules
                    .rules
                    .iter()
                    .chain(language_rules.into_iter().flat_map(|lr| &lr.rules))
                    .rev()
                    .filter(filter)
                    .cloned()
                    .collect();
                (*index, matching_rules)
            })
            .collect();

        SemanticTokenStylizer {
            server_id,
            rules_by_token_type,
            token_type_names: token_types,
            modifier_mask,
        }
    }

    pub fn server_id(&self) -> LanguageServerId {
        self.server_id
    }

    pub fn token_type_name(&self, token_type: TokenType) -> Option<&SharedString> {
        self.token_type_names.get(&token_type)
    }

    pub fn has_modifier(&self, token_modifiers: u32, modifier: &str) -> bool {
        let Some(mask) = self.modifier_mask.get(modifier) else {
            return false;
        };
        (token_modifiers & mask) != 0
    }

    pub fn token_modifiers(&self, token_modifiers: u32) -> Option<String> {
        let modifiers: Vec<&str> = self
            .modifier_mask
            .iter()
            .filter(|(_, mask)| (token_modifiers & *mask) != 0)
            .map(|(name, _)| name.as_ref())
            .collect();
        if modifiers.is_empty() {
            None
        } else {
            Some(modifiers.join(", "))
        }
    }

    pub fn rules_for_token(&self, token_type: TokenType) -> Option<&[SemanticTokenRule]> {
        self.rules_by_token_type
            .get(&token_type)
            .map(|v| v.as_slice())
    }
}

async fn raw_to_buffer_semantic_tokens(
    raw_tokens: RawSemanticTokens,
    buffer_snapshot: &text::BufferSnapshot,
) -> HashMap<LanguageServerId, Arc<[BufferSemanticToken]>> {
    let mut res = HashMap::default();
    for (&server_id, server_tokens) in &raw_tokens.servers {
        // We don't do `collect` here due to the filter map not pre-allocating
        // we'd rather over allocate here than not since we have to re-allocate into an arc slice anyways
        let mut buffer_tokens = Vec::with_capacity(server_tokens.data.len() / 5);
        // 5000 was chosen by profiling, on a decent machine this will take about 1ms per chunk
        // This is to avoid blocking the main thread for hundreds of milliseconds at a time for very big files
        // If we every change the below code to not query the underlying rope 6 times per token we can bump this up
        for chunk in server_tokens.tokens().chunks(5000).into_iter() {
            buffer_tokens.extend(chunk.filter_map(|token| {
                let start = Unclipped(PointUtf16::new(token.line, token.start));
                let clipped_start = buffer_snapshot.clip_point_utf16(start, Bias::Left);
                let start_offset = buffer_snapshot
                    .as_rope()
                    .point_utf16_to_offset_utf16(clipped_start);
                let end_offset = start_offset + OffsetUtf16(token.length as usize);

                let start = buffer_snapshot
                    .as_rope()
                    .offset_utf16_to_offset(start_offset);
                let end = buffer_snapshot.as_rope().offset_utf16_to_offset(end_offset);

                if start == end {
                    return None;
                }

                Some(BufferSemanticToken {
                    range: buffer_snapshot.anchor_before(start)..buffer_snapshot.anchor_after(end),
                    token_type: token.token_type,
                    token_modifiers: token.token_modifiers,
                })
            }));
            yield_now().await;
        }

        res.insert(server_id, buffer_tokens.into());
        yield_now().await;
    }
    res
}

#[derive(Default, Debug)]
pub struct SemanticTokensData {
    pub(super) raw_tokens: RawSemanticTokens,
    pub(super) latest_invalidation_requests: HashMap<LanguageServerId, Option<usize>>,
    update: Option<(Global, SemanticTokensTask)>,
}

/// All the semantic token tokens for a buffer.
///
/// This aggregates semantic tokens from multiple language servers in a specific order.
/// Semantic tokens later in the list will override earlier ones in case of overlap.
#[derive(Default, Debug, Clone)]
pub(super) struct RawSemanticTokens {
    pub servers: HashMap<lsp::LanguageServerId, Arc<ServerSemanticTokens>>,
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
    token_type: TokenType,
    token_modifiers: u32,
}

/// A semantic token, independent of its position.
#[derive(Debug, PartialEq, Eq)]
pub struct SemanticToken {
    pub line: u32,
    pub start: u32,
    pub length: u32,
    pub token_type: TokenType,
    pub token_modifiers: u32,
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

impl Iterator for SemanticTokensIter<'_> {
    type Item = SemanticToken;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.data.next()?;
        let token = SemanticTokenValue {
            delta_line: chunk[0],
            delta_start: chunk[1],
            length: chunk[2],
            token_type: TokenType(chunk[3]),
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
    use crate::lsp_command::SemanticTokensEdit;
    use lsp::SEMANTIC_TOKEN_MODIFIERS;

    fn modifier_names(bits: u32) -> String {
        if bits == 0 {
            return "-".to_string();
        }
        let names: Vec<&str> = SEMANTIC_TOKEN_MODIFIERS
            .iter()
            .enumerate()
            .filter(|(i, _)| bits & (1 << i) != 0)
            .map(|(_, m)| m.as_str())
            .collect();

        // Check for unknown bits
        let known_bits = (1u32 << SEMANTIC_TOKEN_MODIFIERS.len()) - 1;
        let unknown = bits & !known_bits;

        if unknown != 0 {
            let mut result = names.join("+");
            if !result.is_empty() {
                result.push('+');
            }
            result.push_str(&format!("?0x{:x}", unknown));
            result
        } else {
            names.join("+")
        }
    }

    /// Debug tool: parses semantic token JSON from LSP and prints human-readable output.
    ///
    /// Usage: Paste JSON into `json_input`, then run:
    ///   cargo test -p project debug_parse_tokens -- --nocapture --ignored
    ///
    /// Accepts either:
    /// - Full LSP response: `{"jsonrpc":"2.0","id":1,"result":{"data":[...]}}`
    /// - Just the data array: `[0,0,5,1,0,...]`
    ///
    /// For delta responses, paste multiple JSON messages (one per line) and they
    /// will be applied in sequence.
    ///
    /// Token encoding (5 values per token):
    ///   [deltaLine, deltaStart, length, tokenType, tokenModifiers]
    #[test]
    #[ignore] // Run with: cargo test -p project debug_parse_tokens -- --nocapture --ignored
    fn debug_parse_tokens() {
        // ============================================================
        // PASTE YOUR JSON HERE (one message per line for sequences)
        // Comments starting with // are ignored
        // ============================================================
        let json_input = r#"
// === EXAMPLE 1: Full response (LSP spec example) ===
// 3 tokens: property at line 2, type at line 2, class at line 5
{"jsonrpc":"2.0","id":1,"result":{"resultId":"1","data":[2,5,3,9,3,0,5,4,6,0,3,2,7,1,0]}}

// === EXAMPLE 2: Delta response ===
// User added empty line at start of file, so all tokens shift down by 1 line.
// This changes first token's deltaLine from 2 to 3 (edit at index 0).
{"jsonrpc":"2.0","id":2,"result":{"resultId":"2","edits":[{"start":0,"deleteCount":1,"data":[3]}]}}

// === EXAMPLE 3: Another delta ===
// User added a new token. Insert 5 values at position 5 (after first token).
// New token: same line as token 1, 2 chars after it ends, len 5, type=function(12), mods=definition(2)
{"jsonrpc":"2.0","id":3,"result":{"resultId":"3","edits":[{"start":5,"deleteCount":0,"data":[0,2,5,12,2]}]}}
        "#;
        // Accepted formats:
        // - Full response: {"result":{"data":[...]}}
        // - Delta response: {"result":{"edits":[{"start":N,"deleteCount":N,"data":[...]}]}}
        // - Just array: [0,0,5,1,0,...]

        // ============================================================
        // PROCESSING
        // ============================================================
        let mut current_data: Vec<u32> = Vec::new();
        let mut result_id: Option<String> = None;

        for line in json_input.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            let parsed: serde_json::Value =
                serde_json::from_str(line).expect("Failed to parse JSON");

            // Try to extract data from various JSON shapes
            let (data, edits, new_result_id) = extract_semantic_tokens(&parsed);

            if let Some(new_id) = new_result_id {
                result_id = Some(new_id);
            }

            if let Some(full_data) = data {
                println!("\n{}", "=".repeat(70));
                println!("FULL RESPONSE (resultId: {:?})", result_id);
                current_data = full_data;
            } else if let Some(delta_edits) = edits {
                println!("\n{}", "=".repeat(70));
                println!(
                    "DELTA RESPONSE: {} edit(s) (resultId: {:?})",
                    delta_edits.len(),
                    result_id
                );
                for (i, edit) in delta_edits.iter().enumerate() {
                    println!(
                        "  [{}] start={}, delete={}, insert {} values",
                        i,
                        edit.start,
                        edit.delete_count,
                        edit.data.len()
                    );
                }
                let mut tokens = ServerSemanticTokens::from_full(current_data.clone(), None);
                tokens.apply(&delta_edits);
                current_data = tokens.data;
            }
        }

        // Print parsed tokens
        println!(
            "\nDATA: {} values = {} tokens",
            current_data.len(),
            current_data.len() / 5
        );
        println!("\nPARSED TOKENS:");
        println!("{:-<100}", "");
        println!(
            "{:>5} {:>6} {:>4}  {:<15} {}",
            "LINE", "START", "LEN", "TYPE", "MODIFIERS"
        );
        println!("{:-<100}", "");

        let tokens = ServerSemanticTokens::from_full(current_data, None);
        for token in tokens.tokens() {
            println!(
                "{:>5} {:>6} {:>4}  {:<15} {}",
                token.line,
                token.start,
                token.length,
                token.token_type.0,
                modifier_names(token.token_modifiers),
            );
        }
        println!("{:-<100}", "");
        println!("{}\n", "=".repeat(100));
    }

    fn extract_semantic_tokens(
        value: &serde_json::Value,
    ) -> (
        Option<Vec<u32>>,
        Option<Vec<SemanticTokensEdit>>,
        Option<String>,
    ) {
        // Try as array directly: [1,2,3,...]
        if let Some(arr) = value.as_array() {
            let data: Vec<u32> = arr
                .iter()
                .filter_map(|v| v.as_u64().map(|n| n as u32))
                .collect();
            return (Some(data), None, None);
        }

        // Try as LSP response: {"result": {"data": [...]} } or {"result": {"edits": [...]}}
        let result = value.get("result").unwrap_or(value);
        let result_id = result
            .get("resultId")
            .and_then(|v| v.as_str())
            .map(String::from);

        // Full response with data
        if let Some(data_arr) = result.get("data").and_then(|v| v.as_array()) {
            let data: Vec<u32> = data_arr
                .iter()
                .filter_map(|v| v.as_u64().map(|n| n as u32))
                .collect();
            return (Some(data), None, result_id);
        }

        // Delta response with edits
        if let Some(edits_arr) = result.get("edits").and_then(|v| v.as_array()) {
            let edits: Vec<SemanticTokensEdit> = edits_arr
                .iter()
                .filter_map(|e| {
                    Some(SemanticTokensEdit {
                        start: e.get("start")?.as_u64()? as u32,
                        delete_count: e.get("deleteCount")?.as_u64()? as u32,
                        data: e
                            .get("data")
                            .and_then(|d| d.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|v| v.as_u64().map(|n| n as u32))
                                    .collect()
                            })
                            .unwrap_or_default(),
                    })
                })
                .collect();
            return (None, Some(edits), result_id);
        }

        (None, None, result_id)
    }

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
                    token_type: TokenType(0),
                    token_modifiers: 3
                },
                SemanticToken {
                    line: 2,
                    start: 10,
                    length: 4,
                    token_type: TokenType(1),
                    token_modifiers: 0
                },
                SemanticToken {
                    line: 5,
                    start: 2,
                    length: 7,
                    token_type: TokenType(2),
                    token_modifiers: 0
                }
            ]
        );
    }

    #[test]
    fn applies_delta_edit() {
        // Example from the spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_semanticTokens
        // After a user types a new empty line at the beginning of the file,
        // the tokens shift down by one line. The delta edit transforms
        // [2,5,3,0,3, 0,5,4,1,0, 3,2,7,2,0] into [3,5,3,0,3, 0,5,4,1,0, 3,2,7,2,0]
        // by replacing the first element (deltaLine of first token) from 2 to 3.

        let mut tokens = ServerSemanticTokens::from_full(
            vec![2, 5, 3, 0, 3, 0, 5, 4, 1, 0, 3, 2, 7, 2, 0],
            None,
        );

        tokens.apply(&[SemanticTokensEdit {
            start: 0,
            delete_count: 1,
            data: vec![3],
        }]);

        let result = tokens.tokens().collect::<Vec<SemanticToken>>();

        assert_eq!(
            result,
            &[
                SemanticToken {
                    line: 3,
                    start: 5,
                    length: 3,
                    token_type: TokenType(0),
                    token_modifiers: 3
                },
                SemanticToken {
                    line: 3,
                    start: 10,
                    length: 4,
                    token_type: TokenType(1),
                    token_modifiers: 0
                },
                SemanticToken {
                    line: 6,
                    start: 2,
                    length: 7,
                    token_type: TokenType(2),
                    token_modifiers: 0
                }
            ]
        );
    }
}
