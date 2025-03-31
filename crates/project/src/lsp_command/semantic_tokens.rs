use crate::SemanticToken;

use super::*;

#[derive(Debug)]
pub struct SemanticTokensRange {
    pub range: Range<Anchor>,
}

#[async_trait(?Send)]
impl LspCommand for SemanticTokensRange {
    type Response = Vec<SemanticToken>;
    type LspRequest = lsp::request::SemanticTokensRangeRequest;
    type ProtoRequest = proto::SemanticTokensRangeRequest;

    fn display_name(&self) -> &str {
        "Semantic tokens range"
    }

    fn check_capabilities(&self, capabilities: AdapterServerCapabilities) -> bool {
        capabilities
            .server_capabilities
            .semantic_tokens_provider
            .is_some()
    }

    fn to_lsp(
        &self,
        path: &Path,
        buffer: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::SemanticTokensRangeParams> {
        Ok(lsp::SemanticTokensRangeParams {
            range: range_to_lsp(self.range.to_point_utf16(&buffer.snapshot()))?,
            text_document: make_text_document_identifier(path)?,
            partial_result_params: lsp::PartialResultParams {
                partial_result_token: None,
            },
            work_done_progress_params: lsp::WorkDoneProgressParams::default(),
        })
    }

    async fn response_from_lsp(
        mut self,
        message: <Self::LspRequest as lsp::request::Request>::Result,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncApp,
    ) -> Result<Self::Response> {
        let snapshot = buffer.update(&mut cx, |buffer, _| buffer.snapshot())?;
        let legend = legend(lsp_store, server_id, &cx)?;
        let tokens = match message {
            Some(lsp::SemanticTokensRangeResult::Partial(tokens)) => tokens.data,
            Some(lsp::SemanticTokensRangeResult::Tokens(tokens)) => tokens.data,
            None => vec![],
        };
        let mut current_line = 0;
        let mut current_char = 0;
        let new_tokens = tokens.into_iter().map(|token| {
            current_line += token.delta_line;

            // Restore absolute character position
            // If delta_line is 0, subtract delta_start from previous character position
            // Otherwise, use delta_start as the new character position
            if token.delta_line == 0 {
                current_char += token.delta_start;
            } else {
                current_char = token.delta_start;
            }

            // Assuming that we have one token lines because the
            // line of the token never gets "spoofed" by this. and this is actually
            // the current limitation of the semantic highlight support here.
            let start = snapshot.clip_point_utf16(
                Unclipped(PointUtf16::new(current_line, current_char)),
                Bias::Left,
            );
            let end = snapshot.clip_point_utf16(
                Unclipped(PointUtf16::new(current_line, current_char + token.length)),
                Bias::Left,
            );
            SemanticToken {
                range: snapshot.anchor_before(start)..snapshot.anchor_after(end),
                modifiers: active_modifiers(token.token_modifiers_bitset, &legend.token_modifiers),
                r#type: legend
                    .token_types
                    .get(token.token_type as usize)
                    .cloned()
                    .expect("cant find the token in the legend"),
            }
        });
        Ok(new_tokens.collect())
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> Self::ProtoRequest {
        proto::SemanticTokensRangeRequest {
            project_id,
            buffer_id: buffer.remote_id().to_proto(),
            version: serialize_version(&buffer.version()),
            start: Some(serialize_anchor(&self.range.start)),
            end: Some(serialize_anchor(&self.range.end)),
        }
    }

    async fn from_proto(
        message: proto::SemanticTokensRangeRequest,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        let start = message
            .start
            .and_then(language::proto::deserialize_anchor)
            .context("invalid start position")?;
        let end = message
            .end
            .and_then(language::proto::deserialize_anchor)
            .context("invalid end position")?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;

        Ok(Self { range: start..end })
    }

    fn response_to_proto(
        response: Self::Response,
        _: &mut LspStore,
        _: PeerId,
        buffer_version: &clock::Global,
        _: &mut App,
    ) -> proto::SemanticTokensResponse {
        proto::SemanticTokensResponse {
            tokens: response
                .into_iter()
                .map(serialize_semantic_token)
                .collect_vec(),
            version: serialize_version(buffer_version),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::SemanticTokensResponse,
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: AsyncApp,
    ) -> Result<Self::Response> {
        message
            .tokens
            .into_iter()
            .map(proto_to_semantic_token)
            .collect::<Result<Vec<_>>>()
    }

    fn buffer_id_from_proto(message: &proto::SemanticTokensRangeRequest) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

#[derive(Debug)]
pub struct SemanticTokensFull;

#[async_trait(?Send)]
impl LspCommand for SemanticTokensFull {
    type Response = Vec<SemanticToken>;
    type LspRequest = lsp::request::SemanticTokensFullRequest;
    type ProtoRequest = proto::SemanticTokensFullRequest;

    fn display_name(&self) -> &str {
        "Semantic tokens full"
    }

    fn check_capabilities(&self, capabilities: AdapterServerCapabilities) -> bool {
        capabilities
            .server_capabilities
            .semantic_tokens_provider
            .is_some()
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::SemanticTokensParams> {
        Ok(lsp::SemanticTokensParams {
            text_document: make_text_document_identifier(path)?,
            partial_result_params: lsp::PartialResultParams {
                partial_result_token: None,
            },
            work_done_progress_params: lsp::WorkDoneProgressParams::default(),
        })
    }

    async fn response_from_lsp(
        mut self,
        message: <Self::LspRequest as lsp::request::Request>::Result,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncApp,
    ) -> Result<Self::Response> {
        let snapshot = buffer.update(&mut cx, |buffer, _| buffer.snapshot())?;
        let legend = legend(lsp_store, server_id, &cx)?;
        let tokens = match message {
            Some(lsp::SemanticTokensResult::Partial(tokens)) => tokens.data,
            Some(lsp::SemanticTokensResult::Tokens(tokens)) => tokens.data,
            None => vec![],
        };
        let mut current_line = 0;
        let mut current_char = 0;
        let new_tokens = tokens.into_iter().map(|token| {
            current_line += token.delta_line;

            // Restore absolute character position
            // If delta_line is 0, subtract delta_start from previous character position
            // Otherwise, use delta_start as the new character position
            if token.delta_line == 0 {
                current_char += token.delta_start;
            } else {
                current_char = token.delta_start;
            }

            // Assuming that we have one token lines because the
            // line of the token never gets "spoofed" by this. and this is actually
            // the current limitation of the semantic highlight support here.
            let start = snapshot.clip_point_utf16(
                Unclipped(PointUtf16::new(current_line, current_char)),
                Bias::Left,
            );
            let end = snapshot.clip_point_utf16(
                Unclipped(PointUtf16::new(current_line, current_char + token.length)),
                Bias::Left,
            );
            SemanticToken {
                range: snapshot.anchor_before(start)..snapshot.anchor_after(end),
                modifiers: active_modifiers(token.token_modifiers_bitset, &legend.token_modifiers),
                r#type: legend
                    .token_types
                    .get(token.token_type as usize)
                    .cloned()
                    .expect("cant find the token in the legend"),
            }
        });
        Ok(new_tokens.collect())
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> Self::ProtoRequest {
        proto::SemanticTokensFullRequest {
            project_id,
            buffer_id: buffer.remote_id().to_proto(),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::SemanticTokensFullRequest,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;

        Ok(Self)
    }

    fn response_to_proto(
        response: Self::Response,
        _: &mut LspStore,
        _: PeerId,
        buffer_version: &clock::Global,
        _: &mut App,
    ) -> proto::SemanticTokensResponse {
        proto::SemanticTokensResponse {
            tokens: response
                .into_iter()
                .map(serialize_semantic_token)
                .collect_vec(),
            version: serialize_version(buffer_version),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::SemanticTokensResponse,
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: AsyncApp,
    ) -> Result<Self::Response> {
        message
            .tokens
            .into_iter()
            .map(proto_to_semantic_token)
            .collect::<Result<Vec<_>>>()
    }

    fn buffer_id_from_proto(message: &proto::SemanticTokensFullRequest) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

fn active_modifiers(
    modifiers_bitset: u32,
    legend: &[lsp::SemanticTokenModifier],
) -> Vec<lsp::SemanticTokenModifier> {
    legend
        .iter()
        .enumerate()
        .filter_map(move |(idx, modifier)| {
            if (modifiers_bitset & (1 << idx)) != 0 {
                Some(modifier.clone())
            } else {
                None
            }
        })
        .collect_vec()
}

fn legend(
    lsp_store: Entity<LspStore>,
    server_id: LanguageServerId,
    cx: &AsyncApp,
) -> Result<lsp::SemanticTokensLegend> {
    let language_server = cx.update(|cx| {
        lsp_store
            .read(cx)
            .language_server_for_id(server_id)
            .with_context(|| {
                format!("Missing the language server that just returned a response {server_id}")
            })
    })??;
    let server_capabilities = language_server.capabilities();
    let legend = match server_capabilities.semantic_tokens_provider {
        Some(lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(options)) => {
            options.legend
        }
        Some(lsp::SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(options)) => {
            options.semantic_tokens_options.legend
        }
        None => anyhow::bail!("Missing semantic tokens provider in the server"),
    };
    Ok(legend)
}

fn serialize_semantic_token(token: SemanticToken) -> proto::SemanticToken {
    proto::SemanticToken {
        start: Some(language::proto::serialize_anchor(&token.range.start)),
        end: Some(language::proto::serialize_anchor(&token.range.end)),
        token: token.r#type.as_str().into(),
        modifiers: token
            .modifiers
            .into_iter()
            .map(|r#mod| r#mod.as_str().into())
            .collect_vec(),
    }
}

fn proto_to_semantic_token(proto: proto::SemanticToken) -> anyhow::Result<SemanticToken> {
    let start = proto
        .start
        .and_then(language::proto::deserialize_anchor)
        .context("invalid start position")?;
    let end = proto
        .end
        .and_then(language::proto::deserialize_anchor)
        .context("invalid end position")?;
    Ok(SemanticToken {
        range: start..end,
        modifiers: vec![],
        r#type: lsp::SemanticTokenType::from(proto.token),
    })
}
