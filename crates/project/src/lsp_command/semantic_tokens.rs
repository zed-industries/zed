use crate::SemanticToken;

use super::*;

#[derive(Debug)]
pub struct SemanticTokensFull;

#[async_trait(?Send)]
impl LspCommand for SemanticTokensFull {
    type Response = Vec<SemanticToken>;
    type LspRequest = lsp::request::SemanticTokensFullRequest;
    type ProtoRequest = proto::SemanticTokensFullRequest;

    fn display_name(&self) -> &str {
        "Semantic Tokens"
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::SemanticTokensParams> {
        Ok(lsp::SemanticTokensParams {
            text_document: lsp::TextDocumentIdentifier {
                uri: file_path_to_lsp_url(path)?,
            },
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
            Some(lsp::SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
                options,
            )) => options.semantic_tokens_options.legend,
            None => anyhow::bail!("Missing semantic tokens provider in the server"),
        };
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
                Unclipped(PointUtf16::new(
                    current_line, //
                    current_char + token.length,
                )),
                Bias::Right,
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
        todo!()
    }

    async fn from_proto(
        message: Self::ProtoRequest,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        cx: AsyncApp,
    ) -> Result<Self> {
        todo!()
    }

    fn response_to_proto(
        response: Self::Response,
        lsp_store: &mut LspStore,
        peer_id: PeerId,
        buffer_version: &clock::Global,
        cx: &mut App,
    ) -> <Self::ProtoRequest as proto::RequestMessage>::Response {
        todo!()
    }

    async fn response_from_proto(
        self,
        message: <Self::ProtoRequest as proto::RequestMessage>::Response,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        cx: AsyncApp,
    ) -> Result<Self::Response> {
        todo!()
    }

    fn buffer_id_from_proto(message: &Self::ProtoRequest) -> Result<BufferId> {
        todo!()
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
