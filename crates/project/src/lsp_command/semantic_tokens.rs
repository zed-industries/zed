use crate::SemanticToken;

use super::*;

#[derive(Debug, Clone)]
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
        modifiers: proto.modifiers.into_iter().map(Into::into).collect_vec(),
        r#type: lsp::SemanticTokenType::from(proto.token),
    })
}
#[cfg(test)]
mod tests {
    use crate::Project;

    use super::*;
    use fs::FakeFs;
    use gpui::{SemanticVersion, TestAppContext};
    use language::Anchor;
    use lsp::{SemanticTokenModifier, SemanticTokenType};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    #[test]
    fn test_active_modifiers_empty() {
        let modifiers_bitset = 0;
        let legend = vec![];
        let result = active_modifiers(modifiers_bitset, &legend);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_active_modifiers_single_bit() {
        let modifiers_bitset = 1; // 0001 - first bit set
        let legend = vec![
            SemanticTokenModifier::READONLY,
            SemanticTokenModifier::STATIC,
            SemanticTokenModifier::ABSTRACT,
        ];
        let result = active_modifiers(modifiers_bitset, &legend);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], SemanticTokenModifier::READONLY);
    }

    #[test]
    fn test_active_modifiers_multiple_bits() {
        let modifiers_bitset = 6; // 0110 - second and third bits set
        let legend = vec![
            SemanticTokenModifier::READONLY,
            SemanticTokenModifier::STATIC,
            SemanticTokenModifier::ABSTRACT,
            SemanticTokenModifier::DEPRECATED,
        ];
        let result = active_modifiers(modifiers_bitset, &legend);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], SemanticTokenModifier::STATIC);
        assert_eq!(result[1], SemanticTokenModifier::ABSTRACT);
    }

    #[test]
    fn test_active_modifiers_all_bits() {
        let modifiers_bitset = 0b111; // All three bits set
        let legend = vec![
            SemanticTokenModifier::READONLY,
            SemanticTokenModifier::STATIC,
            SemanticTokenModifier::ABSTRACT,
        ];
        let result = active_modifiers(modifiers_bitset, &legend);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], SemanticTokenModifier::READONLY);
        assert_eq!(result[1], SemanticTokenModifier::STATIC);
        assert_eq!(result[2], SemanticTokenModifier::ABSTRACT);
    }

    #[gpui::test]
    async fn test_from_proto_semantic_tokens_range(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() { let x = 42; } // and some long comment to ensure tokens are not trimmed out",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();

        // Create start and end anchors
        let start_anchor = buffer.update(cx, |buffer, _| {
            buffer.anchor_before(buffer.offset_to_point(0))
        });
        let end_anchor = buffer.update(cx, |buffer, _| {
            buffer.anchor_after(buffer.offset_to_point(20))
        });

        // Create a proto request
        let proto_request = cx.update(|app| proto::SemanticTokensRangeRequest {
            project_id: 100,
            buffer_id: buffer.read(app).remote_id().to_proto(),
            version: serialize_version(&buffer.read(app).version()),
            start: Some(language::proto::serialize_anchor(&start_anchor)),
            end: Some(language::proto::serialize_anchor(&end_anchor)),
        });

        // Call from_proto
        let result = cx
            .update(|app| {
                SemanticTokensRange::from_proto(
                    proto_request,
                    project.read(app).lsp_store(),
                    buffer,
                    app.to_async(),
                )
            })
            .await
            .unwrap();

        // Verify the result
        assert_eq!(result.range.start, start_anchor);
        assert_eq!(result.range.end, end_anchor);
    }

    #[gpui::test]
    async fn test_from_proto_semantic_tokens_full(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() { let x = 42; } // and some long comment to ensure tokens are not trimmed out",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();

        // Create a proto request
        let proto_request = cx.update(|app| proto::SemanticTokensFullRequest {
            project_id: 100,
            buffer_id: buffer.read(app).remote_id().to_proto(),
            version: serialize_version(&buffer.read(app).version()),
        });

        // Call from_proto
        let result = cx
            .update(|app| {
                SemanticTokensFull::from_proto(
                    proto_request,
                    project.read(app).lsp_store(),
                    buffer,
                    app.to_async(),
                )
            })
            .await
            .unwrap();

        // Just verify it doesn't fail - the result doesn't have any fields to check
        assert!(matches!(result, SemanticTokensFull));
    }

    #[test]
    fn test_proto_to_semantic_token_valid() {
        // Create a start and end anchor
        let start_anchor = Anchor::MIN;
        let end_anchor = Anchor::MAX;

        // Create a proto token
        let proto_token = proto::SemanticToken {
            start: Some(language::proto::serialize_anchor(&start_anchor)),
            end: Some(language::proto::serialize_anchor(&end_anchor)),
            token: "function".into(),
            modifiers: vec!["readonly".into()],
        };

        // Call proto_to_semantic_token
        let result = proto_to_semantic_token(proto_token).unwrap();

        // Verify the result
        assert_eq!(result.r#type, SemanticTokenType::FUNCTION);
        assert_eq!(result.modifiers, vec![SemanticTokenModifier::READONLY])
    }

    #[test]
    fn test_proto_to_semantic_token_invalid() {
        // Create a proto token with missing start
        let proto_token = proto::SemanticToken {
            start: None,
            end: Some(language::proto::serialize_anchor(&Anchor::MIN)),
            token: "function".into(),
            modifiers: vec!["readonly".into()],
        };

        // Call proto_to_semantic_token
        let result = proto_to_semantic_token(proto_token);

        // Verify it returns an error
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid start position")
        );
    }

    #[test]
    fn test_serialize_semantic_token() {
        // Create a semantic token
        let token = SemanticToken {
            range: Anchor::MIN..Anchor::MAX,
            modifiers: vec![
                SemanticTokenModifier::READONLY,
                SemanticTokenModifier::STATIC,
            ],
            r#type: SemanticTokenType::FUNCTION,
        };

        // Call serialize_semantic_token
        let proto_token = serialize_semantic_token(token);

        // Verify the result
        assert_eq!(proto_token.token, "function");
        assert_eq!(proto_token.modifiers.len(), 2);
        assert_eq!(proto_token.modifiers[0], "readonly");
        assert_eq!(proto_token.modifiers[1], "static");

        // Verify anchors were serialized
        assert!(proto_token.start.is_some());
        assert!(proto_token.end.is_some());
    }

    pub fn init_test(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            release_channel::init(SemanticVersion::default(), cx);
            language::init(cx);
            Project::init_settings(cx);
        });
    }
}
