mod signature_help;

use crate::{
    CodeAction, CompletionSource, CoreCompletion, DocumentHighlight, DocumentSymbol, Hover,
    HoverBlock, HoverBlockKind, InlayHint, InlayHintLabel, InlayHintLabelPart,
    InlayHintLabelPartTooltip, InlayHintTooltip, Location, LocationLink, LspAction, MarkupContent,
    PrepareRenameResponse, ProjectTransaction, ResolveState,
    lsp_store::{LocalLspStore, LspStore},
};
use anyhow::{Context as _, Result, anyhow};
use async_trait::async_trait;
use client::proto::{self, PeerId};
use clock::Global;
use collections::HashSet;
use futures::future;
use gpui::{App, AsyncApp, Entity, Task};
use language::{
    Anchor, Bias, Buffer, BufferSnapshot, CachedLspAdapter, CharKind, OffsetRangeExt, PointUtf16,
    ToOffset, ToPointUtf16, Transaction, Unclipped,
    language_settings::{InlayHintKind, LanguageSettings, language_settings},
    point_from_lsp, point_to_lsp,
    proto::{deserialize_anchor, deserialize_version, serialize_anchor, serialize_version},
    range_from_lsp, range_to_lsp,
};
use lsp::{
    AdapterServerCapabilities, CodeActionKind, CodeActionOptions, CodeDescription,
    CompletionContext, CompletionListItemDefaultsEditRange, CompletionTriggerKind,
    DocumentHighlightKind, LanguageServer, LanguageServerId, LinkedEditingRangeServerCapabilities,
    OneOf, RenameOptions, ServerCapabilities,
};
use serde_json::Value;
use signature_help::{lsp_to_proto_signature, proto_to_lsp_signature};
use std::{cmp::Reverse, mem, ops::Range, path::Path, sync::Arc};
use text::{BufferId, LineEnding};

pub use signature_help::SignatureHelp;

pub fn lsp_formatting_options(settings: &LanguageSettings) -> lsp::FormattingOptions {
    lsp::FormattingOptions {
        tab_size: settings.tab_size.into(),
        insert_spaces: !settings.hard_tabs,
        trim_trailing_whitespace: Some(settings.remove_trailing_whitespace_on_save),
        trim_final_newlines: Some(settings.ensure_final_newline_on_save),
        insert_final_newline: Some(settings.ensure_final_newline_on_save),
        ..lsp::FormattingOptions::default()
    }
}

pub(crate) fn file_path_to_lsp_url(path: &Path) -> Result<lsp::Url> {
    match lsp::Url::from_file_path(path) {
        Ok(url) => Ok(url),
        Err(()) => Err(anyhow!(
            "Invalid file path provided to LSP request: {path:?}"
        )),
    }
}

pub(crate) fn make_text_document_identifier(path: &Path) -> Result<lsp::TextDocumentIdentifier> {
    Ok(lsp::TextDocumentIdentifier {
        uri: file_path_to_lsp_url(path)?,
    })
}

pub(crate) fn make_lsp_text_document_position(
    path: &Path,
    position: PointUtf16,
) -> Result<lsp::TextDocumentPositionParams> {
    Ok(lsp::TextDocumentPositionParams {
        text_document: make_text_document_identifier(path)?,
        position: point_to_lsp(position),
    })
}

#[async_trait(?Send)]
pub trait LspCommand: 'static + Sized + Send + std::fmt::Debug {
    type Response: 'static + Default + Send + std::fmt::Debug;
    type LspRequest: 'static + Send + lsp::request::Request;
    type ProtoRequest: 'static + Send + proto::RequestMessage;

    fn display_name(&self) -> &str;

    fn status(&self) -> Option<String> {
        None
    }

    fn to_lsp_params_or_response(
        &self,
        path: &Path,
        buffer: &Buffer,
        language_server: &Arc<LanguageServer>,
        cx: &App,
    ) -> Result<
        LspParamsOrResponse<<Self::LspRequest as lsp::request::Request>::Params, Self::Response>,
    > {
        if self.check_capabilities(language_server.adapter_server_capabilities()) {
            Ok(LspParamsOrResponse::Params(self.to_lsp(
                path,
                buffer,
                language_server,
                cx,
            )?))
        } else {
            Ok(LspParamsOrResponse::Response(Default::default()))
        }
    }

    /// When false, `to_lsp_params_or_response` default implementation will return the default response.
    fn check_capabilities(&self, _: AdapterServerCapabilities) -> bool {
        true
    }

    fn to_lsp(
        &self,
        path: &Path,
        buffer: &Buffer,
        language_server: &Arc<LanguageServer>,
        cx: &App,
    ) -> Result<<Self::LspRequest as lsp::request::Request>::Params>;

    async fn response_from_lsp(
        self,
        message: <Self::LspRequest as lsp::request::Request>::Result,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        cx: AsyncApp,
    ) -> Result<Self::Response>;

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> Self::ProtoRequest;

    async fn from_proto(
        message: Self::ProtoRequest,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        cx: AsyncApp,
    ) -> Result<Self>;

    fn response_to_proto(
        response: Self::Response,
        lsp_store: &mut LspStore,
        peer_id: PeerId,
        buffer_version: &clock::Global,
        cx: &mut App,
    ) -> <Self::ProtoRequest as proto::RequestMessage>::Response;

    async fn response_from_proto(
        self,
        message: <Self::ProtoRequest as proto::RequestMessage>::Response,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        cx: AsyncApp,
    ) -> Result<Self::Response>;

    fn buffer_id_from_proto(message: &Self::ProtoRequest) -> Result<BufferId>;
}

pub enum LspParamsOrResponse<P, R> {
    Params(P),
    Response(R),
}

#[derive(Debug)]
pub(crate) struct PrepareRename {
    pub position: PointUtf16,
}

#[derive(Debug)]
pub(crate) struct PerformRename {
    pub position: PointUtf16,
    pub new_name: String,
    pub push_to_history: bool,
}

#[derive(Debug)]
pub struct GetDefinition {
    pub position: PointUtf16,
}

#[derive(Debug)]
pub(crate) struct GetDeclaration {
    pub position: PointUtf16,
}

#[derive(Debug)]
pub(crate) struct GetTypeDefinition {
    pub position: PointUtf16,
}

#[derive(Debug)]
pub(crate) struct GetImplementation {
    pub position: PointUtf16,
}

#[derive(Debug)]
pub(crate) struct GetReferences {
    pub position: PointUtf16,
}

#[derive(Debug)]
pub(crate) struct GetDocumentHighlights {
    pub position: PointUtf16,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct GetDocumentSymbols;

#[derive(Clone, Debug)]
pub(crate) struct GetSignatureHelp {
    pub position: PointUtf16,
}

#[derive(Clone, Debug)]
pub(crate) struct GetHover {
    pub position: PointUtf16,
}

#[derive(Debug)]
pub(crate) struct GetCompletions {
    pub position: PointUtf16,
    pub context: CompletionContext,
}

#[derive(Clone, Debug)]
pub(crate) struct GetCodeActions {
    pub range: Range<Anchor>,
    pub kinds: Option<Vec<lsp::CodeActionKind>>,
}

#[derive(Debug)]
pub(crate) struct OnTypeFormatting {
    pub position: PointUtf16,
    pub trigger: String,
    pub options: lsp::FormattingOptions,
    pub push_to_history: bool,
}

#[derive(Debug)]
pub(crate) struct InlayHints {
    pub range: Range<Anchor>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct GetCodeLens;

impl GetCodeLens {
    pub(crate) fn can_resolve_lens(capabilities: &ServerCapabilities) -> bool {
        capabilities
            .code_lens_provider
            .as_ref()
            .and_then(|code_lens_options| code_lens_options.resolve_provider)
            .unwrap_or(false)
    }
}

#[derive(Debug)]
pub(crate) struct LinkedEditingRange {
    pub position: Anchor,
}

#[derive(Clone, Debug)]
pub(crate) struct GetDocumentDiagnostics {}

#[async_trait(?Send)]
impl LspCommand for PrepareRename {
    type Response = PrepareRenameResponse;
    type LspRequest = lsp::request::PrepareRenameRequest;
    type ProtoRequest = proto::PrepareRename;

    fn display_name(&self) -> &str {
        "Prepare rename"
    }

    fn to_lsp_params_or_response(
        &self,
        path: &Path,
        buffer: &Buffer,
        language_server: &Arc<LanguageServer>,
        cx: &App,
    ) -> Result<LspParamsOrResponse<lsp::TextDocumentPositionParams, PrepareRenameResponse>> {
        let rename_provider = language_server
            .adapter_server_capabilities()
            .server_capabilities
            .rename_provider;
        match rename_provider {
            Some(lsp::OneOf::Right(RenameOptions {
                prepare_provider: Some(true),
                ..
            })) => Ok(LspParamsOrResponse::Params(self.to_lsp(
                path,
                buffer,
                language_server,
                cx,
            )?)),
            Some(lsp::OneOf::Right(_)) => Ok(LspParamsOrResponse::Response(
                PrepareRenameResponse::OnlyUnpreparedRenameSupported,
            )),
            Some(lsp::OneOf::Left(true)) => Ok(LspParamsOrResponse::Response(
                PrepareRenameResponse::OnlyUnpreparedRenameSupported,
            )),
            _ => Err(anyhow!("Rename not supported")),
        }
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::TextDocumentPositionParams> {
        make_lsp_text_document_position(path, self.position)
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::PrepareRenameResponse>,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        _: LanguageServerId,
        mut cx: AsyncApp,
    ) -> Result<PrepareRenameResponse> {
        buffer.update(&mut cx, |buffer, _| match message {
            Some(lsp::PrepareRenameResponse::Range(range))
            | Some(lsp::PrepareRenameResponse::RangeWithPlaceholder { range, .. }) => {
                let Range { start, end } = range_from_lsp(range);
                if buffer.clip_point_utf16(start, Bias::Left) == start.0
                    && buffer.clip_point_utf16(end, Bias::Left) == end.0
                {
                    Ok(PrepareRenameResponse::Success(
                        buffer.anchor_after(start)..buffer.anchor_before(end),
                    ))
                } else {
                    Ok(PrepareRenameResponse::InvalidPosition)
                }
            }
            Some(lsp::PrepareRenameResponse::DefaultBehavior { .. }) => {
                let snapshot = buffer.snapshot();
                let (range, _) = snapshot.surrounding_word(self.position);
                let range = snapshot.anchor_after(range.start)..snapshot.anchor_before(range.end);
                Ok(PrepareRenameResponse::Success(range))
            }
            None => Ok(PrepareRenameResponse::InvalidPosition),
        })?
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::PrepareRename {
        proto::PrepareRename {
            project_id,
            buffer_id: buffer.remote_id().into(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::PrepareRename,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;

        Ok(Self {
            position: buffer.update(&mut cx, |buffer, _| position.to_point_utf16(buffer))?,
        })
    }

    fn response_to_proto(
        response: PrepareRenameResponse,
        _: &mut LspStore,
        _: PeerId,
        buffer_version: &clock::Global,
        _: &mut App,
    ) -> proto::PrepareRenameResponse {
        match response {
            PrepareRenameResponse::Success(range) => proto::PrepareRenameResponse {
                can_rename: true,
                only_unprepared_rename_supported: false,
                start: Some(language::proto::serialize_anchor(&range.start)),
                end: Some(language::proto::serialize_anchor(&range.end)),
                version: serialize_version(buffer_version),
            },
            PrepareRenameResponse::OnlyUnpreparedRenameSupported => proto::PrepareRenameResponse {
                can_rename: false,
                only_unprepared_rename_supported: true,
                start: None,
                end: None,
                version: vec![],
            },
            PrepareRenameResponse::InvalidPosition => proto::PrepareRenameResponse {
                can_rename: false,
                only_unprepared_rename_supported: false,
                start: None,
                end: None,
                version: vec![],
            },
        }
    }

    async fn response_from_proto(
        self,
        message: proto::PrepareRenameResponse,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<PrepareRenameResponse> {
        if message.can_rename {
            buffer
                .update(&mut cx, |buffer, _| {
                    buffer.wait_for_version(deserialize_version(&message.version))
                })?
                .await?;
            if let (Some(start), Some(end)) = (
                message.start.and_then(deserialize_anchor),
                message.end.and_then(deserialize_anchor),
            ) {
                Ok(PrepareRenameResponse::Success(start..end))
            } else {
                Err(anyhow!(
                    "Missing start or end position in remote project PrepareRenameResponse"
                ))
            }
        } else if message.only_unprepared_rename_supported {
            Ok(PrepareRenameResponse::OnlyUnpreparedRenameSupported)
        } else {
            Ok(PrepareRenameResponse::InvalidPosition)
        }
    }

    fn buffer_id_from_proto(message: &proto::PrepareRename) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

#[async_trait(?Send)]
impl LspCommand for PerformRename {
    type Response = ProjectTransaction;
    type LspRequest = lsp::request::Rename;
    type ProtoRequest = proto::PerformRename;

    fn display_name(&self) -> &str {
        "Rename"
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::RenameParams> {
        Ok(lsp::RenameParams {
            text_document_position: make_lsp_text_document_position(path, self.position)?,
            new_name: self.new_name.clone(),
            work_done_progress_params: Default::default(),
        })
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::WorkspaceEdit>,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncApp,
    ) -> Result<ProjectTransaction> {
        if let Some(edit) = message {
            let (lsp_adapter, lsp_server) =
                language_server_for_buffer(&lsp_store, &buffer, server_id, &mut cx)?;
            LocalLspStore::deserialize_workspace_edit(
                lsp_store,
                edit,
                self.push_to_history,
                lsp_adapter,
                lsp_server,
                &mut cx,
            )
            .await
        } else {
            Ok(ProjectTransaction::default())
        }
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::PerformRename {
        proto::PerformRename {
            project_id,
            buffer_id: buffer.remote_id().into(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            new_name: self.new_name.clone(),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::PerformRename,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;
        Ok(Self {
            position: buffer.update(&mut cx, |buffer, _| position.to_point_utf16(buffer))?,
            new_name: message.new_name,
            push_to_history: false,
        })
    }

    fn response_to_proto(
        response: ProjectTransaction,
        lsp_store: &mut LspStore,
        peer_id: PeerId,
        _: &clock::Global,
        cx: &mut App,
    ) -> proto::PerformRenameResponse {
        let transaction = lsp_store.buffer_store().update(cx, |buffer_store, cx| {
            buffer_store.serialize_project_transaction_for_peer(response, peer_id, cx)
        });
        proto::PerformRenameResponse {
            transaction: Some(transaction),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::PerformRenameResponse,
        lsp_store: Entity<LspStore>,
        _: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<ProjectTransaction> {
        let message = message
            .transaction
            .ok_or_else(|| anyhow!("missing transaction"))?;
        lsp_store
            .update(&mut cx, |lsp_store, cx| {
                lsp_store.buffer_store().update(cx, |buffer_store, cx| {
                    buffer_store.deserialize_project_transaction(message, self.push_to_history, cx)
                })
            })?
            .await
    }

    fn buffer_id_from_proto(message: &proto::PerformRename) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

#[async_trait(?Send)]
impl LspCommand for GetDefinition {
    type Response = Vec<LocationLink>;
    type LspRequest = lsp::request::GotoDefinition;
    type ProtoRequest = proto::GetDefinition;

    fn display_name(&self) -> &str {
        "Get definition"
    }

    fn check_capabilities(&self, capabilities: AdapterServerCapabilities) -> bool {
        capabilities
            .server_capabilities
            .definition_provider
            .is_some()
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::GotoDefinitionParams> {
        Ok(lsp::GotoDefinitionParams {
            text_document_position_params: make_lsp_text_document_position(path, self.position)?,
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        })
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::GotoDefinitionResponse>,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        cx: AsyncApp,
    ) -> Result<Vec<LocationLink>> {
        location_links_from_lsp(message, lsp_store, buffer, server_id, cx).await
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetDefinition {
        proto::GetDefinition {
            project_id,
            buffer_id: buffer.remote_id().into(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetDefinition,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;
        Ok(Self {
            position: buffer.update(&mut cx, |buffer, _| position.to_point_utf16(buffer))?,
        })
    }

    fn response_to_proto(
        response: Vec<LocationLink>,
        lsp_store: &mut LspStore,
        peer_id: PeerId,
        _: &clock::Global,
        cx: &mut App,
    ) -> proto::GetDefinitionResponse {
        let links = location_links_to_proto(response, lsp_store, peer_id, cx);
        proto::GetDefinitionResponse { links }
    }

    async fn response_from_proto(
        self,
        message: proto::GetDefinitionResponse,
        lsp_store: Entity<LspStore>,
        _: Entity<Buffer>,
        cx: AsyncApp,
    ) -> Result<Vec<LocationLink>> {
        location_links_from_proto(message.links, lsp_store, cx).await
    }

    fn buffer_id_from_proto(message: &proto::GetDefinition) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

#[async_trait(?Send)]
impl LspCommand for GetDeclaration {
    type Response = Vec<LocationLink>;
    type LspRequest = lsp::request::GotoDeclaration;
    type ProtoRequest = proto::GetDeclaration;

    fn display_name(&self) -> &str {
        "Get declaration"
    }

    fn check_capabilities(&self, capabilities: AdapterServerCapabilities) -> bool {
        capabilities
            .server_capabilities
            .declaration_provider
            .is_some()
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::GotoDeclarationParams> {
        Ok(lsp::GotoDeclarationParams {
            text_document_position_params: make_lsp_text_document_position(path, self.position)?,
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        })
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::GotoDeclarationResponse>,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        cx: AsyncApp,
    ) -> Result<Vec<LocationLink>> {
        location_links_from_lsp(message, lsp_store, buffer, server_id, cx).await
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetDeclaration {
        proto::GetDeclaration {
            project_id,
            buffer_id: buffer.remote_id().into(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetDeclaration,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;
        Ok(Self {
            position: buffer.update(&mut cx, |buffer, _| position.to_point_utf16(buffer))?,
        })
    }

    fn response_to_proto(
        response: Vec<LocationLink>,
        lsp_store: &mut LspStore,
        peer_id: PeerId,
        _: &clock::Global,
        cx: &mut App,
    ) -> proto::GetDeclarationResponse {
        let links = location_links_to_proto(response, lsp_store, peer_id, cx);
        proto::GetDeclarationResponse { links }
    }

    async fn response_from_proto(
        self,
        message: proto::GetDeclarationResponse,
        lsp_store: Entity<LspStore>,
        _: Entity<Buffer>,
        cx: AsyncApp,
    ) -> Result<Vec<LocationLink>> {
        location_links_from_proto(message.links, lsp_store, cx).await
    }

    fn buffer_id_from_proto(message: &proto::GetDeclaration) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

#[async_trait(?Send)]
impl LspCommand for GetImplementation {
    type Response = Vec<LocationLink>;
    type LspRequest = lsp::request::GotoImplementation;
    type ProtoRequest = proto::GetImplementation;

    fn display_name(&self) -> &str {
        "Get implementation"
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::GotoImplementationParams> {
        Ok(lsp::GotoImplementationParams {
            text_document_position_params: make_lsp_text_document_position(path, self.position)?,
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        })
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::GotoImplementationResponse>,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        cx: AsyncApp,
    ) -> Result<Vec<LocationLink>> {
        location_links_from_lsp(message, lsp_store, buffer, server_id, cx).await
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetImplementation {
        proto::GetImplementation {
            project_id,
            buffer_id: buffer.remote_id().into(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetImplementation,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;
        Ok(Self {
            position: buffer.update(&mut cx, |buffer, _| position.to_point_utf16(buffer))?,
        })
    }

    fn response_to_proto(
        response: Vec<LocationLink>,
        lsp_store: &mut LspStore,
        peer_id: PeerId,
        _: &clock::Global,
        cx: &mut App,
    ) -> proto::GetImplementationResponse {
        let links = location_links_to_proto(response, lsp_store, peer_id, cx);
        proto::GetImplementationResponse { links }
    }

    async fn response_from_proto(
        self,
        message: proto::GetImplementationResponse,
        project: Entity<LspStore>,
        _: Entity<Buffer>,
        cx: AsyncApp,
    ) -> Result<Vec<LocationLink>> {
        location_links_from_proto(message.links, project, cx).await
    }

    fn buffer_id_from_proto(message: &proto::GetImplementation) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

#[async_trait(?Send)]
impl LspCommand for GetTypeDefinition {
    type Response = Vec<LocationLink>;
    type LspRequest = lsp::request::GotoTypeDefinition;
    type ProtoRequest = proto::GetTypeDefinition;

    fn display_name(&self) -> &str {
        "Get type definition"
    }

    fn check_capabilities(&self, capabilities: AdapterServerCapabilities) -> bool {
        !matches!(
            &capabilities.server_capabilities.type_definition_provider,
            None | Some(lsp::TypeDefinitionProviderCapability::Simple(false))
        )
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::GotoTypeDefinitionParams> {
        Ok(lsp::GotoTypeDefinitionParams {
            text_document_position_params: make_lsp_text_document_position(path, self.position)?,
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        })
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::GotoTypeDefinitionResponse>,
        project: Entity<LspStore>,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        cx: AsyncApp,
    ) -> Result<Vec<LocationLink>> {
        location_links_from_lsp(message, project, buffer, server_id, cx).await
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetTypeDefinition {
        proto::GetTypeDefinition {
            project_id,
            buffer_id: buffer.remote_id().into(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetTypeDefinition,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;
        Ok(Self {
            position: buffer.update(&mut cx, |buffer, _| position.to_point_utf16(buffer))?,
        })
    }

    fn response_to_proto(
        response: Vec<LocationLink>,
        lsp_store: &mut LspStore,
        peer_id: PeerId,
        _: &clock::Global,
        cx: &mut App,
    ) -> proto::GetTypeDefinitionResponse {
        let links = location_links_to_proto(response, lsp_store, peer_id, cx);
        proto::GetTypeDefinitionResponse { links }
    }

    async fn response_from_proto(
        self,
        message: proto::GetTypeDefinitionResponse,
        project: Entity<LspStore>,
        _: Entity<Buffer>,
        cx: AsyncApp,
    ) -> Result<Vec<LocationLink>> {
        location_links_from_proto(message.links, project, cx).await
    }

    fn buffer_id_from_proto(message: &proto::GetTypeDefinition) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

fn language_server_for_buffer(
    lsp_store: &Entity<LspStore>,
    buffer: &Entity<Buffer>,
    server_id: LanguageServerId,
    cx: &mut AsyncApp,
) -> Result<(Arc<CachedLspAdapter>, Arc<LanguageServer>)> {
    lsp_store
        .update(cx, |lsp_store, cx| {
            buffer.update(cx, |buffer, cx| {
                lsp_store
                    .language_server_for_local_buffer(buffer, server_id, cx)
                    .map(|(adapter, server)| (adapter.clone(), server.clone()))
            })
        })?
        .ok_or_else(|| anyhow!("no language server found for buffer"))
}

pub async fn location_links_from_proto(
    proto_links: Vec<proto::LocationLink>,
    lsp_store: Entity<LspStore>,
    mut cx: AsyncApp,
) -> Result<Vec<LocationLink>> {
    let mut links = Vec::new();

    for link in proto_links {
        links.push(location_link_from_proto(link, lsp_store.clone(), &mut cx).await?)
    }

    Ok(links)
}

pub fn location_link_from_proto(
    link: proto::LocationLink,
    lsp_store: Entity<LspStore>,
    cx: &mut AsyncApp,
) -> Task<Result<LocationLink>> {
    cx.spawn(async move |cx| {
        let origin = match link.origin {
            Some(origin) => {
                let buffer_id = BufferId::new(origin.buffer_id)?;
                let buffer = lsp_store
                    .update(cx, |lsp_store, cx| {
                        lsp_store.wait_for_remote_buffer(buffer_id, cx)
                    })?
                    .await?;
                let start = origin
                    .start
                    .and_then(deserialize_anchor)
                    .ok_or_else(|| anyhow!("missing origin start"))?;
                let end = origin
                    .end
                    .and_then(deserialize_anchor)
                    .ok_or_else(|| anyhow!("missing origin end"))?;
                buffer
                    .update(cx, |buffer, _| buffer.wait_for_anchors([start, end]))?
                    .await?;
                Some(Location {
                    buffer,
                    range: start..end,
                })
            }
            None => None,
        };

        let target = link.target.ok_or_else(|| anyhow!("missing target"))?;
        let buffer_id = BufferId::new(target.buffer_id)?;
        let buffer = lsp_store
            .update(cx, |lsp_store, cx| {
                lsp_store.wait_for_remote_buffer(buffer_id, cx)
            })?
            .await?;
        let start = target
            .start
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("missing target start"))?;
        let end = target
            .end
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("missing target end"))?;
        buffer
            .update(cx, |buffer, _| buffer.wait_for_anchors([start, end]))?
            .await?;
        let target = Location {
            buffer,
            range: start..end,
        };
        Ok(LocationLink { origin, target })
    })
}

pub async fn location_links_from_lsp(
    message: Option<lsp::GotoDefinitionResponse>,
    lsp_store: Entity<LspStore>,
    buffer: Entity<Buffer>,
    server_id: LanguageServerId,
    mut cx: AsyncApp,
) -> Result<Vec<LocationLink>> {
    let message = match message {
        Some(message) => message,
        None => return Ok(Vec::new()),
    };

    let mut unresolved_links = Vec::new();
    match message {
        lsp::GotoDefinitionResponse::Scalar(loc) => {
            unresolved_links.push((None, loc.uri, loc.range));
        }

        lsp::GotoDefinitionResponse::Array(locs) => {
            unresolved_links.extend(locs.into_iter().map(|l| (None, l.uri, l.range)));
        }

        lsp::GotoDefinitionResponse::Link(links) => {
            unresolved_links.extend(links.into_iter().map(|l| {
                (
                    l.origin_selection_range,
                    l.target_uri,
                    l.target_selection_range,
                )
            }));
        }
    }

    let (lsp_adapter, language_server) =
        language_server_for_buffer(&lsp_store, &buffer, server_id, &mut cx)?;
    let mut definitions = Vec::new();
    for (origin_range, target_uri, target_range) in unresolved_links {
        let target_buffer_handle = lsp_store
            .update(&mut cx, |this, cx| {
                this.open_local_buffer_via_lsp(
                    target_uri,
                    language_server.server_id(),
                    lsp_adapter.name.clone(),
                    cx,
                )
            })?
            .await?;

        cx.update(|cx| {
            let origin_location = origin_range.map(|origin_range| {
                let origin_buffer = buffer.read(cx);
                let origin_start =
                    origin_buffer.clip_point_utf16(point_from_lsp(origin_range.start), Bias::Left);
                let origin_end =
                    origin_buffer.clip_point_utf16(point_from_lsp(origin_range.end), Bias::Left);
                Location {
                    buffer: buffer.clone(),
                    range: origin_buffer.anchor_after(origin_start)
                        ..origin_buffer.anchor_before(origin_end),
                }
            });

            let target_buffer = target_buffer_handle.read(cx);
            let target_start =
                target_buffer.clip_point_utf16(point_from_lsp(target_range.start), Bias::Left);
            let target_end =
                target_buffer.clip_point_utf16(point_from_lsp(target_range.end), Bias::Left);
            let target_location = Location {
                buffer: target_buffer_handle,
                range: target_buffer.anchor_after(target_start)
                    ..target_buffer.anchor_before(target_end),
            };

            definitions.push(LocationLink {
                origin: origin_location,
                target: target_location,
            })
        })?;
    }
    Ok(definitions)
}

pub async fn location_link_from_lsp(
    link: lsp::LocationLink,
    lsp_store: &Entity<LspStore>,
    buffer: &Entity<Buffer>,
    server_id: LanguageServerId,
    cx: &mut AsyncApp,
) -> Result<LocationLink> {
    let (lsp_adapter, language_server) =
        language_server_for_buffer(&lsp_store, &buffer, server_id, cx)?;

    let (origin_range, target_uri, target_range) = (
        link.origin_selection_range,
        link.target_uri,
        link.target_selection_range,
    );

    let target_buffer_handle = lsp_store
        .update(cx, |lsp_store, cx| {
            lsp_store.open_local_buffer_via_lsp(
                target_uri,
                language_server.server_id(),
                lsp_adapter.name.clone(),
                cx,
            )
        })?
        .await?;

    cx.update(|cx| {
        let origin_location = origin_range.map(|origin_range| {
            let origin_buffer = buffer.read(cx);
            let origin_start =
                origin_buffer.clip_point_utf16(point_from_lsp(origin_range.start), Bias::Left);
            let origin_end =
                origin_buffer.clip_point_utf16(point_from_lsp(origin_range.end), Bias::Left);
            Location {
                buffer: buffer.clone(),
                range: origin_buffer.anchor_after(origin_start)
                    ..origin_buffer.anchor_before(origin_end),
            }
        });

        let target_buffer = target_buffer_handle.read(cx);
        let target_start =
            target_buffer.clip_point_utf16(point_from_lsp(target_range.start), Bias::Left);
        let target_end =
            target_buffer.clip_point_utf16(point_from_lsp(target_range.end), Bias::Left);
        let target_location = Location {
            buffer: target_buffer_handle,
            range: target_buffer.anchor_after(target_start)
                ..target_buffer.anchor_before(target_end),
        };

        LocationLink {
            origin: origin_location,
            target: target_location,
        }
    })
}

pub fn location_links_to_proto(
    links: Vec<LocationLink>,
    lsp_store: &mut LspStore,
    peer_id: PeerId,
    cx: &mut App,
) -> Vec<proto::LocationLink> {
    links
        .into_iter()
        .map(|definition| location_link_to_proto(definition, lsp_store, peer_id, cx))
        .collect()
}

pub fn location_link_to_proto(
    location: LocationLink,
    lsp_store: &mut LspStore,
    peer_id: PeerId,
    cx: &mut App,
) -> proto::LocationLink {
    let origin = location.origin.map(|origin| {
        lsp_store
            .buffer_store()
            .update(cx, |buffer_store, cx| {
                buffer_store.create_buffer_for_peer(&origin.buffer, peer_id, cx)
            })
            .detach_and_log_err(cx);

        let buffer_id = origin.buffer.read(cx).remote_id().into();
        proto::Location {
            start: Some(serialize_anchor(&origin.range.start)),
            end: Some(serialize_anchor(&origin.range.end)),
            buffer_id,
        }
    });

    lsp_store
        .buffer_store()
        .update(cx, |buffer_store, cx| {
            buffer_store.create_buffer_for_peer(&location.target.buffer, peer_id, cx)
        })
        .detach_and_log_err(cx);

    let buffer_id = location.target.buffer.read(cx).remote_id().into();
    let target = proto::Location {
        start: Some(serialize_anchor(&location.target.range.start)),
        end: Some(serialize_anchor(&location.target.range.end)),
        buffer_id,
    };

    proto::LocationLink {
        origin,
        target: Some(target),
    }
}

#[async_trait(?Send)]
impl LspCommand for GetReferences {
    type Response = Vec<Location>;
    type LspRequest = lsp::request::References;
    type ProtoRequest = proto::GetReferences;

    fn display_name(&self) -> &str {
        "Find all references"
    }

    fn status(&self) -> Option<String> {
        Some("Finding references...".to_owned())
    }

    fn check_capabilities(&self, capabilities: AdapterServerCapabilities) -> bool {
        match &capabilities.server_capabilities.references_provider {
            Some(OneOf::Left(has_support)) => *has_support,
            Some(OneOf::Right(_)) => true,
            None => false,
        }
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::ReferenceParams> {
        Ok(lsp::ReferenceParams {
            text_document_position: make_lsp_text_document_position(path, self.position)?,
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: lsp::ReferenceContext {
                include_declaration: true,
            },
        })
    }

    async fn response_from_lsp(
        self,
        locations: Option<Vec<lsp::Location>>,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncApp,
    ) -> Result<Vec<Location>> {
        let mut references = Vec::new();
        let (lsp_adapter, language_server) =
            language_server_for_buffer(&lsp_store, &buffer, server_id, &mut cx)?;

        if let Some(locations) = locations {
            for lsp_location in locations {
                let target_buffer_handle = lsp_store
                    .update(&mut cx, |lsp_store, cx| {
                        lsp_store.open_local_buffer_via_lsp(
                            lsp_location.uri,
                            language_server.server_id(),
                            lsp_adapter.name.clone(),
                            cx,
                        )
                    })?
                    .await?;

                target_buffer_handle
                    .clone()
                    .update(&mut cx, |target_buffer, _| {
                        let target_start = target_buffer
                            .clip_point_utf16(point_from_lsp(lsp_location.range.start), Bias::Left);
                        let target_end = target_buffer
                            .clip_point_utf16(point_from_lsp(lsp_location.range.end), Bias::Left);
                        references.push(Location {
                            buffer: target_buffer_handle,
                            range: target_buffer.anchor_after(target_start)
                                ..target_buffer.anchor_before(target_end),
                        });
                    })?;
            }
        }

        Ok(references)
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetReferences {
        proto::GetReferences {
            project_id,
            buffer_id: buffer.remote_id().into(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetReferences,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;
        Ok(Self {
            position: buffer.update(&mut cx, |buffer, _| position.to_point_utf16(buffer))?,
        })
    }

    fn response_to_proto(
        response: Vec<Location>,
        lsp_store: &mut LspStore,
        peer_id: PeerId,
        _: &clock::Global,
        cx: &mut App,
    ) -> proto::GetReferencesResponse {
        let locations = response
            .into_iter()
            .map(|definition| {
                lsp_store
                    .buffer_store()
                    .update(cx, |buffer_store, cx| {
                        buffer_store.create_buffer_for_peer(&definition.buffer, peer_id, cx)
                    })
                    .detach_and_log_err(cx);
                let buffer_id = definition.buffer.read(cx).remote_id();
                proto::Location {
                    start: Some(serialize_anchor(&definition.range.start)),
                    end: Some(serialize_anchor(&definition.range.end)),
                    buffer_id: buffer_id.into(),
                }
            })
            .collect();
        proto::GetReferencesResponse { locations }
    }

    async fn response_from_proto(
        self,
        message: proto::GetReferencesResponse,
        project: Entity<LspStore>,
        _: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Vec<Location>> {
        let mut locations = Vec::new();
        for location in message.locations {
            let buffer_id = BufferId::new(location.buffer_id)?;
            let target_buffer = project
                .update(&mut cx, |this, cx| {
                    this.wait_for_remote_buffer(buffer_id, cx)
                })?
                .await?;
            let start = location
                .start
                .and_then(deserialize_anchor)
                .ok_or_else(|| anyhow!("missing target start"))?;
            let end = location
                .end
                .and_then(deserialize_anchor)
                .ok_or_else(|| anyhow!("missing target end"))?;
            target_buffer
                .update(&mut cx, |buffer, _| buffer.wait_for_anchors([start, end]))?
                .await?;
            locations.push(Location {
                buffer: target_buffer,
                range: start..end,
            })
        }
        Ok(locations)
    }

    fn buffer_id_from_proto(message: &proto::GetReferences) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

#[async_trait(?Send)]
impl LspCommand for GetDocumentHighlights {
    type Response = Vec<DocumentHighlight>;
    type LspRequest = lsp::request::DocumentHighlightRequest;
    type ProtoRequest = proto::GetDocumentHighlights;

    fn display_name(&self) -> &str {
        "Get document highlights"
    }

    fn check_capabilities(&self, capabilities: AdapterServerCapabilities) -> bool {
        capabilities
            .server_capabilities
            .document_highlight_provider
            .is_some()
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::DocumentHighlightParams> {
        Ok(lsp::DocumentHighlightParams {
            text_document_position_params: make_lsp_text_document_position(path, self.position)?,
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        })
    }

    async fn response_from_lsp(
        self,
        lsp_highlights: Option<Vec<lsp::DocumentHighlight>>,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        _: LanguageServerId,
        mut cx: AsyncApp,
    ) -> Result<Vec<DocumentHighlight>> {
        buffer.update(&mut cx, |buffer, _| {
            let mut lsp_highlights = lsp_highlights.unwrap_or_default();
            lsp_highlights.sort_unstable_by_key(|h| (h.range.start, Reverse(h.range.end)));
            lsp_highlights
                .into_iter()
                .map(|lsp_highlight| {
                    let start = buffer
                        .clip_point_utf16(point_from_lsp(lsp_highlight.range.start), Bias::Left);
                    let end = buffer
                        .clip_point_utf16(point_from_lsp(lsp_highlight.range.end), Bias::Left);
                    DocumentHighlight {
                        range: buffer.anchor_after(start)..buffer.anchor_before(end),
                        kind: lsp_highlight
                            .kind
                            .unwrap_or(lsp::DocumentHighlightKind::READ),
                    }
                })
                .collect()
        })
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetDocumentHighlights {
        proto::GetDocumentHighlights {
            project_id,
            buffer_id: buffer.remote_id().into(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetDocumentHighlights,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;
        Ok(Self {
            position: buffer.update(&mut cx, |buffer, _| position.to_point_utf16(buffer))?,
        })
    }

    fn response_to_proto(
        response: Vec<DocumentHighlight>,
        _: &mut LspStore,
        _: PeerId,
        _: &clock::Global,
        _: &mut App,
    ) -> proto::GetDocumentHighlightsResponse {
        let highlights = response
            .into_iter()
            .map(|highlight| proto::DocumentHighlight {
                start: Some(serialize_anchor(&highlight.range.start)),
                end: Some(serialize_anchor(&highlight.range.end)),
                kind: match highlight.kind {
                    DocumentHighlightKind::TEXT => proto::document_highlight::Kind::Text.into(),
                    DocumentHighlightKind::WRITE => proto::document_highlight::Kind::Write.into(),
                    DocumentHighlightKind::READ => proto::document_highlight::Kind::Read.into(),
                    _ => proto::document_highlight::Kind::Text.into(),
                },
            })
            .collect();
        proto::GetDocumentHighlightsResponse { highlights }
    }

    async fn response_from_proto(
        self,
        message: proto::GetDocumentHighlightsResponse,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Vec<DocumentHighlight>> {
        let mut highlights = Vec::new();
        for highlight in message.highlights {
            let start = highlight
                .start
                .and_then(deserialize_anchor)
                .ok_or_else(|| anyhow!("missing target start"))?;
            let end = highlight
                .end
                .and_then(deserialize_anchor)
                .ok_or_else(|| anyhow!("missing target end"))?;
            buffer
                .update(&mut cx, |buffer, _| buffer.wait_for_anchors([start, end]))?
                .await?;
            let kind = match proto::document_highlight::Kind::from_i32(highlight.kind) {
                Some(proto::document_highlight::Kind::Text) => DocumentHighlightKind::TEXT,
                Some(proto::document_highlight::Kind::Read) => DocumentHighlightKind::READ,
                Some(proto::document_highlight::Kind::Write) => DocumentHighlightKind::WRITE,
                None => DocumentHighlightKind::TEXT,
            };
            highlights.push(DocumentHighlight {
                range: start..end,
                kind,
            });
        }
        Ok(highlights)
    }

    fn buffer_id_from_proto(message: &proto::GetDocumentHighlights) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

#[async_trait(?Send)]
impl LspCommand for GetDocumentSymbols {
    type Response = Vec<DocumentSymbol>;
    type LspRequest = lsp::request::DocumentSymbolRequest;
    type ProtoRequest = proto::GetDocumentSymbols;

    fn display_name(&self) -> &str {
        "Get document symbols"
    }

    fn check_capabilities(&self, capabilities: AdapterServerCapabilities) -> bool {
        capabilities
            .server_capabilities
            .document_symbol_provider
            .is_some()
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::DocumentSymbolParams> {
        Ok(lsp::DocumentSymbolParams {
            text_document: make_text_document_identifier(path)?,
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        })
    }

    async fn response_from_lsp(
        self,
        lsp_symbols: Option<lsp::DocumentSymbolResponse>,
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: LanguageServerId,
        _: AsyncApp,
    ) -> Result<Vec<DocumentSymbol>> {
        let Some(lsp_symbols) = lsp_symbols else {
            return Ok(Vec::new());
        };

        let symbols: Vec<_> = match lsp_symbols {
            lsp::DocumentSymbolResponse::Flat(symbol_information) => symbol_information
                .into_iter()
                .map(|lsp_symbol| DocumentSymbol {
                    name: lsp_symbol.name,
                    kind: lsp_symbol.kind,
                    range: range_from_lsp(lsp_symbol.location.range),
                    selection_range: range_from_lsp(lsp_symbol.location.range),
                    children: Vec::new(),
                })
                .collect(),
            lsp::DocumentSymbolResponse::Nested(nested_responses) => {
                fn convert_symbol(lsp_symbol: lsp::DocumentSymbol) -> DocumentSymbol {
                    DocumentSymbol {
                        name: lsp_symbol.name,
                        kind: lsp_symbol.kind,
                        range: range_from_lsp(lsp_symbol.range),
                        selection_range: range_from_lsp(lsp_symbol.selection_range),
                        children: lsp_symbol
                            .children
                            .map(|children| {
                                children.into_iter().map(convert_symbol).collect::<Vec<_>>()
                            })
                            .unwrap_or_default(),
                    }
                }
                nested_responses.into_iter().map(convert_symbol).collect()
            }
        };
        Ok(symbols)
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetDocumentSymbols {
        proto::GetDocumentSymbols {
            project_id,
            buffer_id: buffer.remote_id().into(),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetDocumentSymbols,
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
        response: Vec<DocumentSymbol>,
        _: &mut LspStore,
        _: PeerId,
        _: &clock::Global,
        _: &mut App,
    ) -> proto::GetDocumentSymbolsResponse {
        let symbols = response
            .into_iter()
            .map(|symbol| {
                fn convert_symbol_to_proto(symbol: DocumentSymbol) -> proto::DocumentSymbol {
                    proto::DocumentSymbol {
                        name: symbol.name.clone(),
                        kind: unsafe { mem::transmute::<lsp::SymbolKind, i32>(symbol.kind) },
                        start: Some(proto::PointUtf16 {
                            row: symbol.range.start.0.row,
                            column: symbol.range.start.0.column,
                        }),
                        end: Some(proto::PointUtf16 {
                            row: symbol.range.end.0.row,
                            column: symbol.range.end.0.column,
                        }),
                        selection_start: Some(proto::PointUtf16 {
                            row: symbol.selection_range.start.0.row,
                            column: symbol.selection_range.start.0.column,
                        }),
                        selection_end: Some(proto::PointUtf16 {
                            row: symbol.selection_range.end.0.row,
                            column: symbol.selection_range.end.0.column,
                        }),
                        children: symbol
                            .children
                            .into_iter()
                            .map(convert_symbol_to_proto)
                            .collect(),
                    }
                }
                convert_symbol_to_proto(symbol)
            })
            .collect::<Vec<_>>();

        proto::GetDocumentSymbolsResponse { symbols }
    }

    async fn response_from_proto(
        self,
        message: proto::GetDocumentSymbolsResponse,
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: AsyncApp,
    ) -> Result<Vec<DocumentSymbol>> {
        let mut symbols = Vec::with_capacity(message.symbols.len());
        for serialized_symbol in message.symbols {
            fn deserialize_symbol_with_children(
                serialized_symbol: proto::DocumentSymbol,
            ) -> Result<DocumentSymbol> {
                let kind =
                    unsafe { mem::transmute::<i32, lsp::SymbolKind>(serialized_symbol.kind) };

                let start = serialized_symbol
                    .start
                    .ok_or_else(|| anyhow!("invalid start"))?;
                let end = serialized_symbol
                    .end
                    .ok_or_else(|| anyhow!("invalid end"))?;

                let selection_start = serialized_symbol
                    .selection_start
                    .ok_or_else(|| anyhow!("invalid selection start"))?;
                let selection_end = serialized_symbol
                    .selection_end
                    .ok_or_else(|| anyhow!("invalid selection end"))?;

                Ok(DocumentSymbol {
                    name: serialized_symbol.name,
                    kind,
                    range: Unclipped(PointUtf16::new(start.row, start.column))
                        ..Unclipped(PointUtf16::new(end.row, end.column)),
                    selection_range: Unclipped(PointUtf16::new(
                        selection_start.row,
                        selection_start.column,
                    ))
                        ..Unclipped(PointUtf16::new(selection_end.row, selection_end.column)),
                    children: serialized_symbol
                        .children
                        .into_iter()
                        .filter_map(|symbol| deserialize_symbol_with_children(symbol).ok())
                        .collect::<Vec<_>>(),
                })
            }

            symbols.push(deserialize_symbol_with_children(serialized_symbol)?);
        }

        Ok(symbols)
    }

    fn buffer_id_from_proto(message: &proto::GetDocumentSymbols) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

#[async_trait(?Send)]
impl LspCommand for GetSignatureHelp {
    type Response = Option<SignatureHelp>;
    type LspRequest = lsp::SignatureHelpRequest;
    type ProtoRequest = proto::GetSignatureHelp;

    fn display_name(&self) -> &str {
        "Get signature help"
    }

    fn check_capabilities(&self, capabilities: AdapterServerCapabilities) -> bool {
        capabilities
            .server_capabilities
            .signature_help_provider
            .is_some()
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _cx: &App,
    ) -> Result<lsp::SignatureHelpParams> {
        Ok(lsp::SignatureHelpParams {
            text_document_position_params: make_lsp_text_document_position(path, self.position)?,
            context: None,
            work_done_progress_params: Default::default(),
        })
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::SignatureHelp>,
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: LanguageServerId,
        _: AsyncApp,
    ) -> Result<Self::Response> {
        Ok(message.and_then(SignatureHelp::new))
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> Self::ProtoRequest {
        let offset = buffer.point_utf16_to_offset(self.position);
        proto::GetSignatureHelp {
            project_id,
            buffer_id: buffer.remote_id().to_proto(),
            position: Some(serialize_anchor(&buffer.anchor_after(offset))),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        payload: Self::ProtoRequest,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&payload.version))
            })?
            .await
            .with_context(|| format!("waiting for version for buffer {}", buffer.entity_id()))?;
        let buffer_snapshot = buffer.update(&mut cx, |buffer, _| buffer.snapshot())?;
        Ok(Self {
            position: payload
                .position
                .and_then(deserialize_anchor)
                .context("invalid position")?
                .to_point_utf16(&buffer_snapshot),
        })
    }

    fn response_to_proto(
        response: Self::Response,
        _: &mut LspStore,
        _: PeerId,
        _: &Global,
        _: &mut App,
    ) -> proto::GetSignatureHelpResponse {
        proto::GetSignatureHelpResponse {
            signature_help: response
                .map(|signature_help| lsp_to_proto_signature(signature_help.original_data)),
        }
    }

    async fn response_from_proto(
        self,
        response: proto::GetSignatureHelpResponse,
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: AsyncApp,
    ) -> Result<Self::Response> {
        Ok(response
            .signature_help
            .map(proto_to_lsp_signature)
            .and_then(SignatureHelp::new))
    }

    fn buffer_id_from_proto(message: &Self::ProtoRequest) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

#[async_trait(?Send)]
impl LspCommand for GetHover {
    type Response = Option<Hover>;
    type LspRequest = lsp::request::HoverRequest;
    type ProtoRequest = proto::GetHover;

    fn display_name(&self) -> &str {
        "Get hover"
    }

    fn check_capabilities(&self, capabilities: AdapterServerCapabilities) -> bool {
        match capabilities.server_capabilities.hover_provider {
            Some(lsp::HoverProviderCapability::Simple(enabled)) => enabled,
            Some(lsp::HoverProviderCapability::Options(_)) => true,
            None => false,
        }
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::HoverParams> {
        Ok(lsp::HoverParams {
            text_document_position_params: make_lsp_text_document_position(path, self.position)?,
            work_done_progress_params: Default::default(),
        })
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::Hover>,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        _: LanguageServerId,
        mut cx: AsyncApp,
    ) -> Result<Self::Response> {
        let Some(hover) = message else {
            return Ok(None);
        };

        let (language, range) = buffer.update(&mut cx, |buffer, _| {
            (
                buffer.language().cloned(),
                hover.range.map(|range| {
                    let token_start =
                        buffer.clip_point_utf16(point_from_lsp(range.start), Bias::Left);
                    let token_end = buffer.clip_point_utf16(point_from_lsp(range.end), Bias::Left);
                    buffer.anchor_after(token_start)..buffer.anchor_before(token_end)
                }),
            )
        })?;

        fn hover_blocks_from_marked_string(marked_string: lsp::MarkedString) -> Option<HoverBlock> {
            let block = match marked_string {
                lsp::MarkedString::String(content) => HoverBlock {
                    text: content,
                    kind: HoverBlockKind::Markdown,
                },
                lsp::MarkedString::LanguageString(lsp::LanguageString { language, value }) => {
                    HoverBlock {
                        text: value,
                        kind: HoverBlockKind::Code { language },
                    }
                }
            };
            if block.text.is_empty() {
                None
            } else {
                Some(block)
            }
        }

        let contents = match hover.contents {
            lsp::HoverContents::Scalar(marked_string) => {
                hover_blocks_from_marked_string(marked_string)
                    .into_iter()
                    .collect()
            }
            lsp::HoverContents::Array(marked_strings) => marked_strings
                .into_iter()
                .filter_map(hover_blocks_from_marked_string)
                .collect(),
            lsp::HoverContents::Markup(markup_content) => vec![HoverBlock {
                text: markup_content.value,
                kind: if markup_content.kind == lsp::MarkupKind::Markdown {
                    HoverBlockKind::Markdown
                } else {
                    HoverBlockKind::PlainText
                },
            }],
        };

        Ok(Some(Hover {
            contents,
            range,
            language,
        }))
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> Self::ProtoRequest {
        proto::GetHover {
            project_id,
            buffer_id: buffer.remote_id().into(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version),
        }
    }

    async fn from_proto(
        message: Self::ProtoRequest,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;
        Ok(Self {
            position: buffer.update(&mut cx, |buffer, _| position.to_point_utf16(buffer))?,
        })
    }

    fn response_to_proto(
        response: Self::Response,
        _: &mut LspStore,
        _: PeerId,
        _: &clock::Global,
        _: &mut App,
    ) -> proto::GetHoverResponse {
        if let Some(response) = response {
            let (start, end) = if let Some(range) = response.range {
                (
                    Some(language::proto::serialize_anchor(&range.start)),
                    Some(language::proto::serialize_anchor(&range.end)),
                )
            } else {
                (None, None)
            };

            let contents = response
                .contents
                .into_iter()
                .map(|block| proto::HoverBlock {
                    text: block.text,
                    is_markdown: block.kind == HoverBlockKind::Markdown,
                    language: if let HoverBlockKind::Code { language } = block.kind {
                        Some(language)
                    } else {
                        None
                    },
                })
                .collect();

            proto::GetHoverResponse {
                start,
                end,
                contents,
            }
        } else {
            proto::GetHoverResponse {
                start: None,
                end: None,
                contents: Vec::new(),
            }
        }
    }

    async fn response_from_proto(
        self,
        message: proto::GetHoverResponse,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self::Response> {
        let contents: Vec<_> = message
            .contents
            .into_iter()
            .map(|block| HoverBlock {
                text: block.text,
                kind: if let Some(language) = block.language {
                    HoverBlockKind::Code { language }
                } else if block.is_markdown {
                    HoverBlockKind::Markdown
                } else {
                    HoverBlockKind::PlainText
                },
            })
            .collect();
        if contents.is_empty() {
            return Ok(None);
        }

        let language = buffer.update(&mut cx, |buffer, _| buffer.language().cloned())?;
        let range = if let (Some(start), Some(end)) = (message.start, message.end) {
            language::proto::deserialize_anchor(start)
                .and_then(|start| language::proto::deserialize_anchor(end).map(|end| start..end))
        } else {
            None
        };
        if let Some(range) = range.as_ref() {
            buffer
                .update(&mut cx, |buffer, _| {
                    buffer.wait_for_anchors([range.start, range.end])
                })?
                .await?;
        }

        Ok(Some(Hover {
            contents,
            range,
            language,
        }))
    }

    fn buffer_id_from_proto(message: &Self::ProtoRequest) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

#[async_trait(?Send)]
impl LspCommand for GetCompletions {
    type Response = Vec<CoreCompletion>;
    type LspRequest = lsp::request::Completion;
    type ProtoRequest = proto::GetCompletions;

    fn display_name(&self) -> &str {
        "Get completion"
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::CompletionParams> {
        Ok(lsp::CompletionParams {
            text_document_position: make_lsp_text_document_position(path, self.position)?,
            context: Some(self.context.clone()),
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        })
    }

    async fn response_from_lsp(
        self,
        completions: Option<lsp::CompletionResponse>,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncApp,
    ) -> Result<Self::Response> {
        let mut response_list = None;
        let mut completions = if let Some(completions) = completions {
            match completions {
                lsp::CompletionResponse::Array(completions) => completions,
                lsp::CompletionResponse::List(mut list) => {
                    let items = std::mem::take(&mut list.items);
                    response_list = Some(list);
                    items
                }
            }
        } else {
            Vec::new()
        };

        let language_server_adapter = lsp_store
            .update(&mut cx, |lsp_store, _| {
                lsp_store.language_server_adapter_for_id(server_id)
            })?
            .with_context(|| format!("no language server with id {server_id}"))?;

        let lsp_defaults = response_list
            .as_ref()
            .and_then(|list| list.item_defaults.clone())
            .map(Arc::new);

        let mut completion_edits = Vec::new();
        buffer.update(&mut cx, |buffer, _cx| {
            let snapshot = buffer.snapshot();
            let clipped_position = buffer.clip_point_utf16(Unclipped(self.position), Bias::Left);

            let mut range_for_token = None;
            completions.retain(|lsp_completion| {
                let lsp_edit = lsp_completion.text_edit.clone().or_else(|| {
                    let default_text_edit = lsp_defaults.as_deref()?.edit_range.as_ref()?;
                    let new_text = lsp_completion
                        .insert_text
                        .as_ref()
                        .unwrap_or(&lsp_completion.label)
                        .clone();
                    match default_text_edit {
                        CompletionListItemDefaultsEditRange::Range(range) => {
                            Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                                range: *range,
                                new_text,
                            }))
                        }
                        CompletionListItemDefaultsEditRange::InsertAndReplace {
                            insert,
                            replace,
                        } => Some(lsp::CompletionTextEdit::InsertAndReplace(
                            lsp::InsertReplaceEdit {
                                new_text,
                                insert: *insert,
                                replace: *replace,
                            },
                        )),
                    }
                });

                let edit = match lsp_edit {
                    // If the language server provides a range to overwrite, then
                    // check that the range is valid.
                    Some(completion_text_edit) => {
                        match parse_completion_text_edit(&completion_text_edit, &snapshot) {
                            Some(edit) => edit,
                            None => return false,
                        }
                    }
                    // If the language server does not provide a range, then infer
                    // the range based on the syntax tree.
                    None => {
                        if self.position != clipped_position {
                            log::info!("completion out of expected range");
                            return false;
                        }

                        let default_edit_range = lsp_defaults.as_ref().and_then(|lsp_defaults| {
                            lsp_defaults
                                .edit_range
                                .as_ref()
                                .and_then(|range| match range {
                                    CompletionListItemDefaultsEditRange::Range(r) => Some(r),
                                    _ => None,
                                })
                        });

                        let range = if let Some(range) = default_edit_range {
                            let range = range_from_lsp(*range);
                            let start = snapshot.clip_point_utf16(range.start, Bias::Left);
                            let end = snapshot.clip_point_utf16(range.end, Bias::Left);
                            if start != range.start.0 || end != range.end.0 {
                                log::info!("completion out of expected range");
                                return false;
                            }

                            snapshot.anchor_before(start)..snapshot.anchor_after(end)
                        } else {
                            range_for_token
                                .get_or_insert_with(|| {
                                    let offset = self.position.to_offset(&snapshot);
                                    let (range, kind) = snapshot.surrounding_word(offset);
                                    let range = if kind == Some(CharKind::Word) {
                                        range
                                    } else {
                                        offset..offset
                                    };

                                    snapshot.anchor_before(range.start)
                                        ..snapshot.anchor_after(range.end)
                                })
                                .clone()
                        };

                        // We already know text_edit is None here
                        let text = lsp_completion
                            .insert_text
                            .as_ref()
                            .unwrap_or(&lsp_completion.label)
                            .clone();

                        ParsedCompletionEdit {
                            replace_range: range,
                            insert_range: None,
                            new_text: text,
                        }
                    }
                };

                completion_edits.push(edit);
                true
            });
        })?;

        language_server_adapter
            .process_completions(&mut completions)
            .await;

        Ok(completions
            .into_iter()
            .zip(completion_edits)
            .map(|(mut lsp_completion, mut edit)| {
                LineEnding::normalize(&mut edit.new_text);
                if lsp_completion.data.is_none() {
                    if let Some(default_data) = lsp_defaults
                        .as_ref()
                        .and_then(|item_defaults| item_defaults.data.clone())
                    {
                        // Servers (e.g. JDTLS) prefer unchanged completions, when resolving the items later,
                        // so we do not insert the defaults here, but `data` is needed for resolving, so this is an exception.
                        lsp_completion.data = Some(default_data);
                    }
                }
                CoreCompletion {
                    replace_range: edit.replace_range,
                    new_text: edit.new_text,
                    source: CompletionSource::Lsp {
                        insert_range: edit.insert_range,
                        server_id,
                        lsp_completion: Box::new(lsp_completion),
                        lsp_defaults: lsp_defaults.clone(),
                        resolved: false,
                    },
                }
            })
            .collect())
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetCompletions {
        let anchor = buffer.anchor_after(self.position);
        proto::GetCompletions {
            project_id,
            buffer_id: buffer.remote_id().into(),
            position: Some(language::proto::serialize_anchor(&anchor)),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetCompletions,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        let version = deserialize_version(&message.version);
        buffer
            .update(&mut cx, |buffer, _| buffer.wait_for_version(version))?
            .await?;
        let position = message
            .position
            .and_then(language::proto::deserialize_anchor)
            .map(|p| {
                buffer.update(&mut cx, |buffer, _| {
                    buffer.clip_point_utf16(Unclipped(p.to_point_utf16(buffer)), Bias::Left)
                })
            })
            .ok_or_else(|| anyhow!("invalid position"))??;
        Ok(Self {
            position,
            context: CompletionContext {
                trigger_kind: CompletionTriggerKind::INVOKED,
                trigger_character: None,
            },
        })
    }

    fn response_to_proto(
        completions: Vec<CoreCompletion>,
        _: &mut LspStore,
        _: PeerId,
        buffer_version: &clock::Global,
        _: &mut App,
    ) -> proto::GetCompletionsResponse {
        proto::GetCompletionsResponse {
            completions: completions
                .iter()
                .map(LspStore::serialize_completion)
                .collect(),
            version: serialize_version(buffer_version),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::GetCompletionsResponse,
        _project: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self::Response> {
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;

        message
            .completions
            .into_iter()
            .map(LspStore::deserialize_completion)
            .collect()
    }

    fn buffer_id_from_proto(message: &proto::GetCompletions) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

pub struct ParsedCompletionEdit {
    pub replace_range: Range<Anchor>,
    pub insert_range: Option<Range<Anchor>>,
    pub new_text: String,
}

pub(crate) fn parse_completion_text_edit(
    edit: &lsp::CompletionTextEdit,
    snapshot: &BufferSnapshot,
) -> Option<ParsedCompletionEdit> {
    let (replace_range, insert_range, new_text) = match edit {
        lsp::CompletionTextEdit::Edit(edit) => (edit.range, None, &edit.new_text),
        lsp::CompletionTextEdit::InsertAndReplace(edit) => {
            (edit.replace, Some(edit.insert), &edit.new_text)
        }
    };

    let replace_range = {
        let range = range_from_lsp(replace_range);
        let start = snapshot.clip_point_utf16(range.start, Bias::Left);
        let end = snapshot.clip_point_utf16(range.end, Bias::Left);
        if start != range.start.0 || end != range.end.0 {
            log::info!("completion out of expected range");
            return None;
        }
        snapshot.anchor_before(start)..snapshot.anchor_after(end)
    };

    let insert_range = match insert_range {
        None => None,
        Some(insert_range) => {
            let range = range_from_lsp(insert_range);
            let start = snapshot.clip_point_utf16(range.start, Bias::Left);
            let end = snapshot.clip_point_utf16(range.end, Bias::Left);
            if start != range.start.0 || end != range.end.0 {
                log::info!("completion (insert) out of expected range");
                return None;
            }
            Some(snapshot.anchor_before(start)..snapshot.anchor_after(end))
        }
    };

    Some(ParsedCompletionEdit {
        insert_range: insert_range,
        replace_range: replace_range,
        new_text: new_text.clone(),
    })
}

#[async_trait(?Send)]
impl LspCommand for GetCodeActions {
    type Response = Vec<CodeAction>;
    type LspRequest = lsp::request::CodeActionRequest;
    type ProtoRequest = proto::GetCodeActions;

    fn display_name(&self) -> &str {
        "Get code actions"
    }

    fn check_capabilities(&self, capabilities: AdapterServerCapabilities) -> bool {
        match &capabilities.server_capabilities.code_action_provider {
            None => false,
            Some(lsp::CodeActionProviderCapability::Simple(false)) => false,
            _ => {
                // If we do know that we want specific code actions AND we know that
                // the server only supports specific code actions, then we want to filter
                // down to the ones that are supported.
                if let Some((requested, supported)) = self
                    .kinds
                    .as_ref()
                    .zip(Self::supported_code_action_kinds(capabilities))
                {
                    let server_supported = supported.into_iter().collect::<HashSet<_>>();
                    requested.iter().any(|kind| server_supported.contains(kind))
                } else {
                    true
                }
            }
        }
    }

    fn to_lsp(
        &self,
        path: &Path,
        buffer: &Buffer,
        language_server: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::CodeActionParams> {
        let mut relevant_diagnostics = Vec::new();
        for entry in buffer
            .snapshot()
            .diagnostics_in_range::<_, language::PointUtf16>(self.range.clone(), false)
        {
            relevant_diagnostics.push(entry.to_lsp_diagnostic_stub()?);
        }

        let supported =
            Self::supported_code_action_kinds(language_server.adapter_server_capabilities());

        let only = if let Some(requested) = &self.kinds {
            if let Some(supported_kinds) = supported {
                let server_supported = supported_kinds.into_iter().collect::<HashSet<_>>();

                let filtered = requested
                    .iter()
                    .filter(|kind| server_supported.contains(kind))
                    .cloned()
                    .collect();
                Some(filtered)
            } else {
                Some(requested.clone())
            }
        } else {
            supported
        };

        Ok(lsp::CodeActionParams {
            text_document: make_text_document_identifier(path)?,
            range: range_to_lsp(self.range.to_point_utf16(buffer))?,
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: lsp::CodeActionContext {
                diagnostics: relevant_diagnostics,
                only,
                ..lsp::CodeActionContext::default()
            },
        })
    }

    async fn response_from_lsp(
        self,
        actions: Option<lsp::CodeActionResponse>,
        lsp_store: Entity<LspStore>,
        _: Entity<Buffer>,
        server_id: LanguageServerId,
        cx: AsyncApp,
    ) -> Result<Vec<CodeAction>> {
        let requested_kinds_set = if let Some(kinds) = self.kinds {
            Some(kinds.into_iter().collect::<HashSet<_>>())
        } else {
            None
        };

        let language_server = cx.update(|cx| {
            lsp_store
                .read(cx)
                .language_server_for_id(server_id)
                .with_context(|| {
                    format!("Missing the language server that just returned a response {server_id}")
                })
        })??;

        let server_capabilities = language_server.capabilities();
        let available_commands = server_capabilities
            .execute_command_provider
            .as_ref()
            .map(|options| options.commands.as_slice())
            .unwrap_or_default();
        Ok(actions
            .unwrap_or_default()
            .into_iter()
            .filter_map(|entry| {
                let (lsp_action, resolved) = match entry {
                    lsp::CodeActionOrCommand::CodeAction(lsp_action) => {
                        if let Some(command) = lsp_action.command.as_ref() {
                            if !available_commands.contains(&command.command) {
                                return None;
                            }
                        }
                        (LspAction::Action(Box::new(lsp_action)), false)
                    }
                    lsp::CodeActionOrCommand::Command(command) => {
                        if available_commands.contains(&command.command) {
                            (LspAction::Command(command), true)
                        } else {
                            return None;
                        }
                    }
                };

                if let Some((requested_kinds, kind)) =
                    requested_kinds_set.as_ref().zip(lsp_action.action_kind())
                {
                    if !requested_kinds.contains(&kind) {
                        return None;
                    }
                }

                Some(CodeAction {
                    server_id,
                    range: self.range.clone(),
                    lsp_action,
                    resolved,
                })
            })
            .collect())
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetCodeActions {
        proto::GetCodeActions {
            project_id,
            buffer_id: buffer.remote_id().into(),
            start: Some(language::proto::serialize_anchor(&self.range.start)),
            end: Some(language::proto::serialize_anchor(&self.range.end)),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetCodeActions,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        let start = message
            .start
            .and_then(language::proto::deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid start"))?;
        let end = message
            .end
            .and_then(language::proto::deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid end"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;

        Ok(Self {
            range: start..end,
            kinds: None,
        })
    }

    fn response_to_proto(
        code_actions: Vec<CodeAction>,
        _: &mut LspStore,
        _: PeerId,
        buffer_version: &clock::Global,
        _: &mut App,
    ) -> proto::GetCodeActionsResponse {
        proto::GetCodeActionsResponse {
            actions: code_actions
                .iter()
                .map(LspStore::serialize_code_action)
                .collect(),
            version: serialize_version(buffer_version),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::GetCodeActionsResponse,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Vec<CodeAction>> {
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;
        message
            .actions
            .into_iter()
            .map(LspStore::deserialize_code_action)
            .collect()
    }

    fn buffer_id_from_proto(message: &proto::GetCodeActions) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

impl GetCodeActions {
    fn supported_code_action_kinds(
        capabilities: AdapterServerCapabilities,
    ) -> Option<Vec<CodeActionKind>> {
        match capabilities.server_capabilities.code_action_provider {
            Some(lsp::CodeActionProviderCapability::Options(CodeActionOptions {
                code_action_kinds: Some(supported_action_kinds),
                ..
            })) => Some(supported_action_kinds.clone()),
            _ => capabilities.code_action_kinds,
        }
    }

    pub fn can_resolve_actions(capabilities: &ServerCapabilities) -> bool {
        capabilities
            .code_action_provider
            .as_ref()
            .and_then(|options| match options {
                lsp::CodeActionProviderCapability::Simple(_is_supported) => None,
                lsp::CodeActionProviderCapability::Options(options) => options.resolve_provider,
            })
            .unwrap_or(false)
    }
}

#[async_trait(?Send)]
impl LspCommand for OnTypeFormatting {
    type Response = Option<Transaction>;
    type LspRequest = lsp::request::OnTypeFormatting;
    type ProtoRequest = proto::OnTypeFormatting;

    fn display_name(&self) -> &str {
        "Formatting on typing"
    }

    fn check_capabilities(&self, capabilities: AdapterServerCapabilities) -> bool {
        let Some(on_type_formatting_options) = &capabilities
            .server_capabilities
            .document_on_type_formatting_provider
        else {
            return false;
        };
        on_type_formatting_options
            .first_trigger_character
            .contains(&self.trigger)
            || on_type_formatting_options
                .more_trigger_character
                .iter()
                .flatten()
                .any(|chars| chars.contains(&self.trigger))
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::DocumentOnTypeFormattingParams> {
        Ok(lsp::DocumentOnTypeFormattingParams {
            text_document_position: make_lsp_text_document_position(path, self.position)?,
            ch: self.trigger.clone(),
            options: self.options.clone(),
        })
    }

    async fn response_from_lsp(
        self,
        message: Option<Vec<lsp::TextEdit>>,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncApp,
    ) -> Result<Option<Transaction>> {
        if let Some(edits) = message {
            let (lsp_adapter, lsp_server) =
                language_server_for_buffer(&lsp_store, &buffer, server_id, &mut cx)?;
            LocalLspStore::deserialize_text_edits(
                lsp_store,
                buffer,
                edits,
                self.push_to_history,
                lsp_adapter,
                lsp_server,
                &mut cx,
            )
            .await
        } else {
            Ok(None)
        }
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::OnTypeFormatting {
        proto::OnTypeFormatting {
            project_id,
            buffer_id: buffer.remote_id().into(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            trigger: self.trigger.clone(),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::OnTypeFormatting,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;

        let options = buffer.update(&mut cx, |buffer, cx| {
            lsp_formatting_options(
                language_settings(buffer.language().map(|l| l.name()), buffer.file(), cx).as_ref(),
            )
        })?;

        Ok(Self {
            position: buffer.update(&mut cx, |buffer, _| position.to_point_utf16(buffer))?,
            trigger: message.trigger.clone(),
            options,
            push_to_history: false,
        })
    }

    fn response_to_proto(
        response: Option<Transaction>,
        _: &mut LspStore,
        _: PeerId,
        _: &clock::Global,
        _: &mut App,
    ) -> proto::OnTypeFormattingResponse {
        proto::OnTypeFormattingResponse {
            transaction: response
                .map(|transaction| language::proto::serialize_transaction(&transaction)),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::OnTypeFormattingResponse,
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: AsyncApp,
    ) -> Result<Option<Transaction>> {
        let Some(transaction) = message.transaction else {
            return Ok(None);
        };
        Ok(Some(language::proto::deserialize_transaction(transaction)?))
    }

    fn buffer_id_from_proto(message: &proto::OnTypeFormatting) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

impl InlayHints {
    pub async fn lsp_to_project_hint(
        lsp_hint: lsp::InlayHint,
        buffer_handle: &Entity<Buffer>,
        server_id: LanguageServerId,
        resolve_state: ResolveState,
        force_no_type_left_padding: bool,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<InlayHint> {
        let kind = lsp_hint.kind.and_then(|kind| match kind {
            lsp::InlayHintKind::TYPE => Some(InlayHintKind::Type),
            lsp::InlayHintKind::PARAMETER => Some(InlayHintKind::Parameter),
            _ => None,
        });

        let position = buffer_handle.update(cx, |buffer, _| {
            let position = buffer.clip_point_utf16(point_from_lsp(lsp_hint.position), Bias::Left);
            if kind == Some(InlayHintKind::Parameter) {
                buffer.anchor_before(position)
            } else {
                buffer.anchor_after(position)
            }
        })?;
        let label = Self::lsp_inlay_label_to_project(lsp_hint.label, server_id)
            .await
            .context("lsp to project inlay hint conversion")?;
        let padding_left = if force_no_type_left_padding && kind == Some(InlayHintKind::Type) {
            false
        } else {
            lsp_hint.padding_left.unwrap_or(false)
        };

        Ok(InlayHint {
            position,
            padding_left,
            padding_right: lsp_hint.padding_right.unwrap_or(false),
            label,
            kind,
            tooltip: lsp_hint.tooltip.map(|tooltip| match tooltip {
                lsp::InlayHintTooltip::String(s) => InlayHintTooltip::String(s),
                lsp::InlayHintTooltip::MarkupContent(markup_content) => {
                    InlayHintTooltip::MarkupContent(MarkupContent {
                        kind: match markup_content.kind {
                            lsp::MarkupKind::PlainText => HoverBlockKind::PlainText,
                            lsp::MarkupKind::Markdown => HoverBlockKind::Markdown,
                        },
                        value: markup_content.value,
                    })
                }
            }),
            resolve_state,
        })
    }

    async fn lsp_inlay_label_to_project(
        lsp_label: lsp::InlayHintLabel,
        server_id: LanguageServerId,
    ) -> anyhow::Result<InlayHintLabel> {
        let label = match lsp_label {
            lsp::InlayHintLabel::String(s) => InlayHintLabel::String(s),
            lsp::InlayHintLabel::LabelParts(lsp_parts) => {
                let mut parts = Vec::with_capacity(lsp_parts.len());
                for lsp_part in lsp_parts {
                    parts.push(InlayHintLabelPart {
                        value: lsp_part.value,
                        tooltip: lsp_part.tooltip.map(|tooltip| match tooltip {
                            lsp::InlayHintLabelPartTooltip::String(s) => {
                                InlayHintLabelPartTooltip::String(s)
                            }
                            lsp::InlayHintLabelPartTooltip::MarkupContent(markup_content) => {
                                InlayHintLabelPartTooltip::MarkupContent(MarkupContent {
                                    kind: match markup_content.kind {
                                        lsp::MarkupKind::PlainText => HoverBlockKind::PlainText,
                                        lsp::MarkupKind::Markdown => HoverBlockKind::Markdown,
                                    },
                                    value: markup_content.value,
                                })
                            }
                        }),
                        location: Some(server_id).zip(lsp_part.location),
                    });
                }
                InlayHintLabel::LabelParts(parts)
            }
        };

        Ok(label)
    }

    pub fn project_to_proto_hint(response_hint: InlayHint) -> proto::InlayHint {
        let (state, lsp_resolve_state) = match response_hint.resolve_state {
            ResolveState::Resolved => (0, None),
            ResolveState::CanResolve(server_id, resolve_data) => (
                1,
                Some(proto::resolve_state::LspResolveState {
                    server_id: server_id.0 as u64,
                    value: resolve_data.map(|json_data| {
                        serde_json::to_string(&json_data)
                            .expect("failed to serialize resolve json data")
                    }),
                }),
            ),
            ResolveState::Resolving => (2, None),
        };
        let resolve_state = Some(proto::ResolveState {
            state,
            lsp_resolve_state,
        });
        proto::InlayHint {
            position: Some(language::proto::serialize_anchor(&response_hint.position)),
            padding_left: response_hint.padding_left,
            padding_right: response_hint.padding_right,
            label: Some(proto::InlayHintLabel {
                label: Some(match response_hint.label {
                    InlayHintLabel::String(s) => proto::inlay_hint_label::Label::Value(s),
                    InlayHintLabel::LabelParts(label_parts) => {
                        proto::inlay_hint_label::Label::LabelParts(proto::InlayHintLabelParts {
                            parts: label_parts.into_iter().map(|label_part| {
                                let location_url = label_part.location.as_ref().map(|(_, location)| location.uri.to_string());
                                let location_range_start = label_part.location.as_ref().map(|(_, location)| point_from_lsp(location.range.start).0).map(|point| proto::PointUtf16 { row: point.row, column: point.column });
                                let location_range_end = label_part.location.as_ref().map(|(_, location)| point_from_lsp(location.range.end).0).map(|point| proto::PointUtf16 { row: point.row, column: point.column });
                                proto::InlayHintLabelPart {
                                value: label_part.value,
                                tooltip: label_part.tooltip.map(|tooltip| {
                                    let proto_tooltip = match tooltip {
                                        InlayHintLabelPartTooltip::String(s) => proto::inlay_hint_label_part_tooltip::Content::Value(s),
                                        InlayHintLabelPartTooltip::MarkupContent(markup_content) => proto::inlay_hint_label_part_tooltip::Content::MarkupContent(proto::MarkupContent {
                                            is_markdown: markup_content.kind == HoverBlockKind::Markdown,
                                            value: markup_content.value,
                                        }),
                                    };
                                    proto::InlayHintLabelPartTooltip {content: Some(proto_tooltip)}
                                }),
                                location_url,
                                location_range_start,
                                location_range_end,
                                language_server_id: label_part.location.as_ref().map(|(server_id, _)| server_id.0 as u64),
                            }}).collect()
                        })
                    }
                }),
            }),
            kind: response_hint.kind.map(|kind| kind.name().to_string()),
            tooltip: response_hint.tooltip.map(|response_tooltip| {
                let proto_tooltip = match response_tooltip {
                    InlayHintTooltip::String(s) => proto::inlay_hint_tooltip::Content::Value(s),
                    InlayHintTooltip::MarkupContent(markup_content) => {
                        proto::inlay_hint_tooltip::Content::MarkupContent(proto::MarkupContent {
                            is_markdown: markup_content.kind == HoverBlockKind::Markdown,
                            value: markup_content.value,
                        })
                    }
                };
                proto::InlayHintTooltip {
                    content: Some(proto_tooltip),
                }
            }),
            resolve_state,
        }
    }

    pub fn proto_to_project_hint(message_hint: proto::InlayHint) -> anyhow::Result<InlayHint> {
        let resolve_state = message_hint.resolve_state.as_ref().unwrap_or_else(|| {
            panic!("incorrect proto inlay hint message: no resolve state in hint {message_hint:?}",)
        });
        let resolve_state_data = resolve_state
            .lsp_resolve_state.as_ref()
            .map(|lsp_resolve_state| {
                let value = lsp_resolve_state.value.as_deref().map(|value| {
                    serde_json::from_str::<Option<lsp::LSPAny>>(value)
                        .with_context(|| format!("incorrect proto inlay hint message: non-json resolve state {lsp_resolve_state:?}"))
                }).transpose()?.flatten();
                anyhow::Ok((LanguageServerId(lsp_resolve_state.server_id as usize), value))
            })
            .transpose()?;
        let resolve_state = match resolve_state.state {
            0 => ResolveState::Resolved,
            1 => {
                let (server_id, lsp_resolve_state) = resolve_state_data.with_context(|| {
                    format!(
                        "No lsp resolve data for the hint that can be resolved: {message_hint:?}"
                    )
                })?;
                ResolveState::CanResolve(server_id, lsp_resolve_state)
            }
            2 => ResolveState::Resolving,
            invalid => {
                anyhow::bail!("Unexpected resolve state {invalid} for hint {message_hint:?}")
            }
        };
        Ok(InlayHint {
            position: message_hint
                .position
                .and_then(language::proto::deserialize_anchor)
                .context("invalid position")?,
            label: match message_hint
                .label
                .and_then(|label| label.label)
                .context("missing label")?
            {
                proto::inlay_hint_label::Label::Value(s) => InlayHintLabel::String(s),
                proto::inlay_hint_label::Label::LabelParts(parts) => {
                    let mut label_parts = Vec::new();
                    for part in parts.parts {
                        label_parts.push(InlayHintLabelPart {
                            value: part.value,
                            tooltip: part.tooltip.map(|tooltip| match tooltip.content {
                                Some(proto::inlay_hint_label_part_tooltip::Content::Value(s)) => {
                                    InlayHintLabelPartTooltip::String(s)
                                }
                                Some(
                                    proto::inlay_hint_label_part_tooltip::Content::MarkupContent(
                                        markup_content,
                                    ),
                                ) => InlayHintLabelPartTooltip::MarkupContent(MarkupContent {
                                    kind: if markup_content.is_markdown {
                                        HoverBlockKind::Markdown
                                    } else {
                                        HoverBlockKind::PlainText
                                    },
                                    value: markup_content.value,
                                }),
                                None => InlayHintLabelPartTooltip::String(String::new()),
                            }),
                            location: {
                                match part
                                    .location_url
                                    .zip(
                                        part.location_range_start.and_then(|start| {
                                            Some(start..part.location_range_end?)
                                        }),
                                    )
                                    .zip(part.language_server_id)
                                {
                                    Some(((uri, range), server_id)) => Some((
                                        LanguageServerId(server_id as usize),
                                        lsp::Location {
                                            uri: lsp::Url::parse(&uri)
                                                .context("invalid uri in hint part {part:?}")?,
                                            range: lsp::Range::new(
                                                point_to_lsp(PointUtf16::new(
                                                    range.start.row,
                                                    range.start.column,
                                                )),
                                                point_to_lsp(PointUtf16::new(
                                                    range.end.row,
                                                    range.end.column,
                                                )),
                                            ),
                                        },
                                    )),
                                    None => None,
                                }
                            },
                        });
                    }

                    InlayHintLabel::LabelParts(label_parts)
                }
            },
            padding_left: message_hint.padding_left,
            padding_right: message_hint.padding_right,
            kind: message_hint
                .kind
                .as_deref()
                .and_then(InlayHintKind::from_name),
            tooltip: message_hint.tooltip.and_then(|tooltip| {
                Some(match tooltip.content? {
                    proto::inlay_hint_tooltip::Content::Value(s) => InlayHintTooltip::String(s),
                    proto::inlay_hint_tooltip::Content::MarkupContent(markup_content) => {
                        InlayHintTooltip::MarkupContent(MarkupContent {
                            kind: if markup_content.is_markdown {
                                HoverBlockKind::Markdown
                            } else {
                                HoverBlockKind::PlainText
                            },
                            value: markup_content.value,
                        })
                    }
                })
            }),
            resolve_state,
        })
    }

    pub fn project_to_lsp_hint(hint: InlayHint, snapshot: &BufferSnapshot) -> lsp::InlayHint {
        lsp::InlayHint {
            position: point_to_lsp(hint.position.to_point_utf16(snapshot)),
            kind: hint.kind.map(|kind| match kind {
                InlayHintKind::Type => lsp::InlayHintKind::TYPE,
                InlayHintKind::Parameter => lsp::InlayHintKind::PARAMETER,
            }),
            text_edits: None,
            tooltip: hint.tooltip.and_then(|tooltip| {
                Some(match tooltip {
                    InlayHintTooltip::String(s) => lsp::InlayHintTooltip::String(s),
                    InlayHintTooltip::MarkupContent(markup_content) => {
                        lsp::InlayHintTooltip::MarkupContent(lsp::MarkupContent {
                            kind: match markup_content.kind {
                                HoverBlockKind::PlainText => lsp::MarkupKind::PlainText,
                                HoverBlockKind::Markdown => lsp::MarkupKind::Markdown,
                                HoverBlockKind::Code { .. } => return None,
                            },
                            value: markup_content.value,
                        })
                    }
                })
            }),
            label: match hint.label {
                InlayHintLabel::String(s) => lsp::InlayHintLabel::String(s),
                InlayHintLabel::LabelParts(label_parts) => lsp::InlayHintLabel::LabelParts(
                    label_parts
                        .into_iter()
                        .map(|part| lsp::InlayHintLabelPart {
                            value: part.value,
                            tooltip: part.tooltip.and_then(|tooltip| {
                                Some(match tooltip {
                                    InlayHintLabelPartTooltip::String(s) => {
                                        lsp::InlayHintLabelPartTooltip::String(s)
                                    }
                                    InlayHintLabelPartTooltip::MarkupContent(markup_content) => {
                                        lsp::InlayHintLabelPartTooltip::MarkupContent(
                                            lsp::MarkupContent {
                                                kind: match markup_content.kind {
                                                    HoverBlockKind::PlainText => {
                                                        lsp::MarkupKind::PlainText
                                                    }
                                                    HoverBlockKind::Markdown => {
                                                        lsp::MarkupKind::Markdown
                                                    }
                                                    HoverBlockKind::Code { .. } => return None,
                                                },
                                                value: markup_content.value,
                                            },
                                        )
                                    }
                                })
                            }),
                            location: part.location.map(|(_, location)| location),
                            command: None,
                        })
                        .collect(),
                ),
            },
            padding_left: Some(hint.padding_left),
            padding_right: Some(hint.padding_right),
            data: match hint.resolve_state {
                ResolveState::CanResolve(_, data) => data,
                ResolveState::Resolving | ResolveState::Resolved => None,
            },
        }
    }

    pub fn can_resolve_inlays(capabilities: &ServerCapabilities) -> bool {
        capabilities
            .inlay_hint_provider
            .as_ref()
            .and_then(|options| match options {
                OneOf::Left(_is_supported) => None,
                OneOf::Right(capabilities) => match capabilities {
                    lsp::InlayHintServerCapabilities::Options(o) => o.resolve_provider,
                    lsp::InlayHintServerCapabilities::RegistrationOptions(o) => {
                        o.inlay_hint_options.resolve_provider
                    }
                },
            })
            .unwrap_or(false)
    }
}

#[async_trait(?Send)]
impl LspCommand for InlayHints {
    type Response = Vec<InlayHint>;
    type LspRequest = lsp::InlayHintRequest;
    type ProtoRequest = proto::InlayHints;

    fn display_name(&self) -> &str {
        "Inlay hints"
    }

    fn check_capabilities(&self, capabilities: AdapterServerCapabilities) -> bool {
        let Some(inlay_hint_provider) = &capabilities.server_capabilities.inlay_hint_provider
        else {
            return false;
        };
        match inlay_hint_provider {
            lsp::OneOf::Left(enabled) => *enabled,
            lsp::OneOf::Right(inlay_hint_capabilities) => match inlay_hint_capabilities {
                lsp::InlayHintServerCapabilities::Options(_) => true,
                lsp::InlayHintServerCapabilities::RegistrationOptions(_) => false,
            },
        }
    }

    fn to_lsp(
        &self,
        path: &Path,
        buffer: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::InlayHintParams> {
        Ok(lsp::InlayHintParams {
            text_document: lsp::TextDocumentIdentifier {
                uri: file_path_to_lsp_url(path)?,
            },
            range: range_to_lsp(self.range.to_point_utf16(buffer))?,
            work_done_progress_params: Default::default(),
        })
    }

    async fn response_from_lsp(
        self,
        message: Option<Vec<lsp::InlayHint>>,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncApp,
    ) -> anyhow::Result<Vec<InlayHint>> {
        let (lsp_adapter, lsp_server) =
            language_server_for_buffer(&lsp_store, &buffer, server_id, &mut cx)?;
        // `typescript-language-server` adds padding to the left for type hints, turning
        // `const foo: boolean` into `const foo : boolean` which looks odd.
        // `rust-analyzer` does not have the padding for this case, and we have to accommodate both.
        //
        // We could trim the whole string, but being pessimistic on par with the situation above,
        // there might be a hint with multiple whitespaces at the end(s) which we need to display properly.
        // Hence let's use a heuristic first to handle the most awkward case and look for more.
        let force_no_type_left_padding =
            lsp_adapter.name.0.as_ref() == "typescript-language-server";

        let hints = message.unwrap_or_default().into_iter().map(|lsp_hint| {
            let resolve_state = if InlayHints::can_resolve_inlays(&lsp_server.capabilities()) {
                ResolveState::CanResolve(lsp_server.server_id(), lsp_hint.data.clone())
            } else {
                ResolveState::Resolved
            };

            let buffer = buffer.clone();
            cx.spawn(async move |cx| {
                InlayHints::lsp_to_project_hint(
                    lsp_hint,
                    &buffer,
                    server_id,
                    resolve_state,
                    force_no_type_left_padding,
                    cx,
                )
                .await
            })
        });
        future::join_all(hints)
            .await
            .into_iter()
            .collect::<anyhow::Result<_>>()
            .context("lsp to project inlay hints conversion")
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::InlayHints {
        proto::InlayHints {
            project_id,
            buffer_id: buffer.remote_id().into(),
            start: Some(language::proto::serialize_anchor(&self.range.start)),
            end: Some(language::proto::serialize_anchor(&self.range.end)),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::InlayHints,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        let start = message
            .start
            .and_then(language::proto::deserialize_anchor)
            .context("invalid start")?;
        let end = message
            .end
            .and_then(language::proto::deserialize_anchor)
            .context("invalid end")?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;

        Ok(Self { range: start..end })
    }

    fn response_to_proto(
        response: Vec<InlayHint>,
        _: &mut LspStore,
        _: PeerId,
        buffer_version: &clock::Global,
        _: &mut App,
    ) -> proto::InlayHintsResponse {
        proto::InlayHintsResponse {
            hints: response
                .into_iter()
                .map(InlayHints::project_to_proto_hint)
                .collect(),
            version: serialize_version(buffer_version),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::InlayHintsResponse,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> anyhow::Result<Vec<InlayHint>> {
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;

        let mut hints = Vec::new();
        for message_hint in message.hints {
            hints.push(InlayHints::proto_to_project_hint(message_hint)?);
        }

        Ok(hints)
    }

    fn buffer_id_from_proto(message: &proto::InlayHints) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

#[async_trait(?Send)]
impl LspCommand for GetCodeLens {
    type Response = Vec<CodeAction>;
    type LspRequest = lsp::CodeLensRequest;
    type ProtoRequest = proto::GetCodeLens;

    fn display_name(&self) -> &str {
        "Code Lens"
    }

    fn check_capabilities(&self, capabilities: AdapterServerCapabilities) -> bool {
        capabilities
            .server_capabilities
            .code_lens_provider
            .as_ref()
            .map_or(false, |code_lens_options| {
                code_lens_options.resolve_provider.unwrap_or(false)
            })
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::CodeLensParams> {
        Ok(lsp::CodeLensParams {
            text_document: lsp::TextDocumentIdentifier {
                uri: file_path_to_lsp_url(path)?,
            },
            work_done_progress_params: lsp::WorkDoneProgressParams::default(),
            partial_result_params: lsp::PartialResultParams::default(),
        })
    }

    async fn response_from_lsp(
        self,
        message: Option<Vec<lsp::CodeLens>>,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncApp,
    ) -> anyhow::Result<Vec<CodeAction>> {
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
        let available_commands = server_capabilities
            .execute_command_provider
            .as_ref()
            .map(|options| options.commands.as_slice())
            .unwrap_or_default();
        Ok(message
            .unwrap_or_default()
            .into_iter()
            .filter(|code_lens| {
                code_lens
                    .command
                    .as_ref()
                    .is_none_or(|command| available_commands.contains(&command.command))
            })
            .map(|code_lens| {
                let code_lens_range = range_from_lsp(code_lens.range);
                let start = snapshot.clip_point_utf16(code_lens_range.start, Bias::Left);
                let end = snapshot.clip_point_utf16(code_lens_range.end, Bias::Right);
                let range = snapshot.anchor_before(start)..snapshot.anchor_after(end);
                CodeAction {
                    server_id,
                    range,
                    lsp_action: LspAction::CodeLens(code_lens),
                    resolved: false,
                }
            })
            .collect())
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetCodeLens {
        proto::GetCodeLens {
            project_id,
            buffer_id: buffer.remote_id().into(),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetCodeLens,
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
        response: Vec<CodeAction>,
        _: &mut LspStore,
        _: PeerId,
        buffer_version: &clock::Global,
        _: &mut App,
    ) -> proto::GetCodeLensResponse {
        proto::GetCodeLensResponse {
            lens_actions: response
                .iter()
                .map(LspStore::serialize_code_action)
                .collect(),
            version: serialize_version(buffer_version),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::GetCodeLensResponse,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> anyhow::Result<Vec<CodeAction>> {
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;
        message
            .lens_actions
            .into_iter()
            .map(LspStore::deserialize_code_action)
            .collect::<Result<Vec<_>>>()
            .context("deserializing proto code lens response")
    }

    fn buffer_id_from_proto(message: &proto::GetCodeLens) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

#[async_trait(?Send)]
impl LspCommand for LinkedEditingRange {
    type Response = Vec<Range<Anchor>>;
    type LspRequest = lsp::request::LinkedEditingRange;
    type ProtoRequest = proto::LinkedEditingRange;

    fn display_name(&self) -> &str {
        "Linked editing range"
    }

    fn check_capabilities(&self, capabilities: AdapterServerCapabilities) -> bool {
        let Some(linked_editing_options) = &capabilities
            .server_capabilities
            .linked_editing_range_provider
        else {
            return false;
        };
        if let LinkedEditingRangeServerCapabilities::Simple(false) = linked_editing_options {
            return false;
        }
        true
    }

    fn to_lsp(
        &self,
        path: &Path,
        buffer: &Buffer,
        _server: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::LinkedEditingRangeParams> {
        let position = self.position.to_point_utf16(&buffer.snapshot());
        Ok(lsp::LinkedEditingRangeParams {
            text_document_position_params: make_lsp_text_document_position(path, position)?,
            work_done_progress_params: Default::default(),
        })
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::LinkedEditingRanges>,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        _server_id: LanguageServerId,
        cx: AsyncApp,
    ) -> Result<Vec<Range<Anchor>>> {
        if let Some(lsp::LinkedEditingRanges { mut ranges, .. }) = message {
            ranges.sort_by_key(|range| range.start);

            buffer.read_with(&cx, |buffer, _| {
                ranges
                    .into_iter()
                    .map(|range| {
                        let start =
                            buffer.clip_point_utf16(point_from_lsp(range.start), Bias::Left);
                        let end = buffer.clip_point_utf16(point_from_lsp(range.end), Bias::Left);
                        buffer.anchor_before(start)..buffer.anchor_after(end)
                    })
                    .collect()
            })
        } else {
            Ok(vec![])
        }
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::LinkedEditingRange {
        proto::LinkedEditingRange {
            project_id,
            buffer_id: buffer.remote_id().to_proto(),
            position: Some(serialize_anchor(&self.position)),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::LinkedEditingRange,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        let position = message
            .position
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;
        let position = deserialize_anchor(position).ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| buffer.wait_for_anchors([position]))?
            .await?;
        Ok(Self { position })
    }

    fn response_to_proto(
        response: Vec<Range<Anchor>>,
        _: &mut LspStore,
        _: PeerId,
        buffer_version: &clock::Global,
        _: &mut App,
    ) -> proto::LinkedEditingRangeResponse {
        proto::LinkedEditingRangeResponse {
            items: response
                .into_iter()
                .map(|range| proto::AnchorRange {
                    start: Some(serialize_anchor(&range.start)),
                    end: Some(serialize_anchor(&range.end)),
                })
                .collect(),
            version: serialize_version(buffer_version),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::LinkedEditingRangeResponse,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Vec<Range<Anchor>>> {
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;
        let items: Vec<Range<Anchor>> = message
            .items
            .into_iter()
            .filter_map(|range| {
                let start = deserialize_anchor(range.start?)?;
                let end = deserialize_anchor(range.end?)?;
                Some(start..end)
            })
            .collect();
        for range in &items {
            buffer
                .update(&mut cx, |buffer, _| {
                    buffer.wait_for_anchors([range.start, range.end])
                })?
                .await?;
        }
        Ok(items)
    }

    fn buffer_id_from_proto(message: &proto::LinkedEditingRange) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

impl GetDocumentDiagnostics {
    fn deserialize_lsp_diagnostic(diagnostic: proto::LspDiagnostic) -> Result<lsp::Diagnostic> {
        let start = diagnostic
            .start
            .ok_or_else(|| anyhow!("invalid start range"))?;
        let end = diagnostic.end.ok_or_else(|| anyhow!("invalid end range"))?;

        let range = Range::<PointUtf16> {
            start: PointUtf16 {
                row: start.row,
                column: start.column,
            },
            end: PointUtf16 {
                row: end.row,
                column: end.column,
            },
        };

        let data = diagnostic.data.and_then(|data| Value::from_str(&data).ok());
        let code = diagnostic.code.map(lsp::NumberOrString::String);

        let related_information = diagnostic
            .related_information
            .into_iter()
            .map(|info| {
                let start = info.location_range_start.unwrap();
                let end = info.location_range_end.unwrap();

                lsp::DiagnosticRelatedInformation {
                    location: lsp::Location {
                        range: lsp::Range {
                            start: point_to_lsp(PointUtf16::new(start.row, start.column)),
                            end: point_to_lsp(PointUtf16::new(end.row, end.column)),
                        },
                        uri: lsp::Url::parse(&info.location_url.unwrap()).unwrap(),
                    },
                    message: info.message.clone(),
                }
            })
            .collect::<Vec<_>>();

        let tags = diagnostic
            .tags
            .into_iter()
            .filter_map(|tag| match proto::LspDiagnosticTag::from_i32(tag) {
                Some(proto::LspDiagnosticTag::Unnecessary) => Some(lsp::DiagnosticTag::UNNECESSARY),
                Some(proto::LspDiagnosticTag::Deprecated) => Some(lsp::DiagnosticTag::DEPRECATED),
                _ => None,
            })
            .collect::<Vec<_>>();

        Ok(lsp::Diagnostic {
            range: language::range_to_lsp(range)?,
            severity: match proto::lsp_diagnostic::Severity::from_i32(diagnostic.severity).unwrap()
            {
                proto::lsp_diagnostic::Severity::Error => Some(lsp::DiagnosticSeverity::ERROR),
                proto::lsp_diagnostic::Severity::Warning => Some(lsp::DiagnosticSeverity::WARNING),
                proto::lsp_diagnostic::Severity::Information => {
                    Some(lsp::DiagnosticSeverity::INFORMATION)
                }
                proto::lsp_diagnostic::Severity::Hint => Some(lsp::DiagnosticSeverity::HINT),
                _ => None,
            },
            code,
            code_description: match diagnostic.code_description {
                Some(code_description) => Some(CodeDescription {
                    href: lsp::Url::parse(&code_description).unwrap(),
                }),
                None => None,
            },
            related_information: Some(related_information),
            tags: Some(tags),
            source: diagnostic.source.clone(),
            message: diagnostic.message,
            data,
        })
    }

    fn serialize_lsp_diagnostic(diagnostic: lsp::Diagnostic) -> Result<proto::LspDiagnostic> {
        let range = language::range_from_lsp(diagnostic.range);
        let related_information = diagnostic
            .related_information
            .unwrap_or_default()
            .into_iter()
            .map(|related_information| {
                let location_range_start =
                    point_from_lsp(related_information.location.range.start).0;
                let location_range_end = point_from_lsp(related_information.location.range.end).0;

                Ok(proto::LspDiagnosticRelatedInformation {
                    location_url: Some(related_information.location.uri.to_string()),
                    location_range_start: Some(proto::PointUtf16 {
                        row: location_range_start.row,
                        column: location_range_start.column,
                    }),
                    location_range_end: Some(proto::PointUtf16 {
                        row: location_range_end.row,
                        column: location_range_end.column,
                    }),
                    message: related_information.message,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let tags = diagnostic
            .tags
            .unwrap_or_default()
            .into_iter()
            .map(|tag| match tag {
                lsp::DiagnosticTag::UNNECESSARY => proto::LspDiagnosticTag::Unnecessary,
                lsp::DiagnosticTag::DEPRECATED => proto::LspDiagnosticTag::Deprecated,
                _ => proto::LspDiagnosticTag::None,
            } as i32)
            .collect();

        Ok(proto::LspDiagnostic {
            start: Some(proto::PointUtf16 {
                row: range.start.0.row,
                column: range.start.0.column,
            }),
            end: Some(proto::PointUtf16 {
                row: range.end.0.row,
                column: range.end.0.column,
            }),
            severity: match diagnostic.severity {
                Some(lsp::DiagnosticSeverity::ERROR) => proto::lsp_diagnostic::Severity::Error,
                Some(lsp::DiagnosticSeverity::WARNING) => proto::lsp_diagnostic::Severity::Warning,
                Some(lsp::DiagnosticSeverity::INFORMATION) => {
                    proto::lsp_diagnostic::Severity::Information
                }
                Some(lsp::DiagnosticSeverity::HINT) => proto::lsp_diagnostic::Severity::Hint,
                _ => proto::lsp_diagnostic::Severity::None,
            } as i32,
            code: diagnostic.code.as_ref().map(|code| match code {
                lsp::NumberOrString::Number(code) => code.to_string(),
                lsp::NumberOrString::String(code) => code.clone(),
            }),
            source: diagnostic.source.clone(),
            related_information,
            tags,
            code_description: diagnostic
                .code_description
                .map(|desc| desc.href.to_string()),
            message: diagnostic.message,
            data: diagnostic.data.as_ref().map(|data| data.to_string()),
        })
    }
}

#[async_trait(?Send)]
impl LspCommand for GetDocumentDiagnostics {
    type Response = Option<LspDiagnostics>;
    type LspRequest = lsp::request::DocumentDiagnosticRequest;
    type ProtoRequest = proto::GetDocumentDiagnostics;

    fn display_name(&self) -> &str {
        "Get diagnostics"
    }

    fn check_capabilities(&self, server_capabilities: AdapterServerCapabilities) -> bool {
        server_capabilities
            .server_capabilities
            .diagnostic_provider
            .is_some()
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        language_server: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<lsp::DocumentDiagnosticParams> {
        let identifier = match language_server.capabilities().diagnostic_provider {
            Some(lsp::DiagnosticServerCapabilities::Options(options)) => options.identifier.clone(),
            Some(lsp::DiagnosticServerCapabilities::RegistrationOptions(options)) => {
                options.diagnostic_options.identifier.clone()
            }
            None => None,
        };

        Ok(lsp::DocumentDiagnosticParams {
            text_document: lsp::TextDocumentIdentifier {
                uri: lsp::Url::from_file_path(path)
                    .map_err(|_| anyhow::anyhow!("Invalid file path"))?,
            },
            identifier,
            previous_result_id: None,
            partial_result_params: Default::default(),
            work_done_progress_params: Default::default(),
        })
    }

    async fn response_from_lsp(
        self,
        message: lsp::DocumentDiagnosticReportResult,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncApp,
    ) -> Result<Self::Response> {
        let uri = buffer.read_with(&cx, |buffer, cx| {
            buffer
                .file()
                .and_then(|file| file.as_local())
                .and_then(|file| lsp::Url::from_file_path(file.abs_path(cx)).ok())
        })?;

        let Some(uri) = uri else {
            return Ok(None);
        };

        let language_server_adapter = lsp_store
            .update(&mut cx, |lsp_store, _| {
                lsp_store.language_server_adapter_for_id(server_id)
            })?
            .ok_or_else(|| anyhow!("no such language server"))?;

        match message {
            lsp::DocumentDiagnosticReportResult::Report(report) => match report {
                lsp::DocumentDiagnosticReport::Full(report) => {
                    lsp_store
                        .update(&mut cx, |store, cx| {
                            report
                                .related_documents
                                .into_iter()
                                .flatten()
                                .filter_map(|(uri, report)| {
                                    if let lsp::DocumentDiagnosticReportKind::Full(full_report) =
                                        report
                                    {
                                        Some(store.update_diagnostics(
                                            server_id,
                                            lsp::PublishDiagnosticsParams {
                                                diagnostics: full_report.items.clone(),
                                                uri,
                                                version: None,
                                            },
                                            &language_server_adapter.disk_based_diagnostic_sources,
                                            cx,
                                        ))
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>()
                        })
                        .with_context(|| "Failed to update diagnostics for related documents")?;

                    Ok(Some(LspDiagnostics {
                        server_id,
                        uri: Some(uri),
                        diagnostics: Some(report.full_document_diagnostic_report.items.clone()),
                    }))
                }
                lsp::DocumentDiagnosticReport::Unchanged(_) => Ok(Some(LspDiagnostics {
                    server_id,
                    uri: Some(uri),
                    diagnostics: None,
                })),
            },
            lsp::DocumentDiagnosticReportResult::Partial(_) => Ok(Some(LspDiagnostics {
                server_id,
                uri: Some(uri),
                diagnostics: None,
            })),
        }
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetDocumentDiagnostics {
        proto::GetDocumentDiagnostics {
            project_id,
            buffer_id: buffer.remote_id().into(),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetDocumentDiagnostics,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Self> {
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;
        Ok(Self {})
    }

    fn response_to_proto(
        response: Self::Response,
        _: &mut LspStore,
        _: PeerId,
        _: &clock::Global,
        _: &mut App,
    ) -> proto::GetDocumentDiagnosticsResponse {
        if let Some(response) = response {
            let diagnostics = response
                .diagnostics
                .map(|diagnostics| {
                    diagnostics
                        .into_iter()
                        .filter_map(|diagnostic| {
                            match GetDocumentDiagnostics::serialize_lsp_diagnostic(diagnostic) {
                                Ok(diagnostic) => Some(diagnostic),
                                Err(error) => {
                                    log::error!("Failed to serialize diagnostic: {}", error);
                                    None
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            proto::GetDocumentDiagnosticsResponse {
                server_id: LanguageServerId::to_proto(response.server_id),
                uri: response.uri.unwrap().to_string(),
                diagnostics,
            }
        } else {
            proto::GetDocumentDiagnosticsResponse {
                server_id: 0,
                uri: Default::default(),
                diagnostics: Vec::new(),
            }
        }
    }

    async fn response_from_proto(
        self,
        response: proto::GetDocumentDiagnosticsResponse,
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: AsyncApp,
    ) -> Result<Self::Response> {
        let uri = lsp::Url::from_str(response.uri.as_str())
            .with_context(|| format!("Failed to parse URI: {}", response.uri))?;

        let diagnostics = response
            .diagnostics
            .into_iter()
            .filter_map(|diagnostic| {
                match GetDocumentDiagnostics::deserialize_lsp_diagnostic(diagnostic) {
                    Ok(diagnostic) => Some(diagnostic),
                    Err(error) => {
                        log::error!("Failed to deserialize diagnostic: {}", error);
                        None
                    }
                }
            })
            .collect();

        Ok(Some(LspDiagnostics {
            server_id: LanguageServerId::from_proto(response.server_id),
            uri: Some(uri),
            diagnostics: Some(diagnostics),
        }))
    }

    fn buffer_id_from_proto(message: &proto::GetDocumentDiagnostics) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lsp::{DiagnosticSeverity, DiagnosticTag};
    use serde_json::json;

    #[test]
    fn test_serialize_lsp_diagnostic() {
        let lsp_diagnostic = lsp::Diagnostic {
            range: lsp::Range {
                start: lsp::Position::new(0, 1),
                end: lsp::Position::new(2, 3),
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(lsp::NumberOrString::String("E001".to_string())),
            source: Some("test-source".to_string()),
            message: "Test error message".to_string(),
            related_information: None,
            tags: Some(vec![DiagnosticTag::DEPRECATED]),
            code_description: None,
            data: Some(json!({"detail": "test detail"})),
        };

        let proto_diagnostic =
            GetDocumentDiagnostics::serialize_lsp_diagnostic(lsp_diagnostic.clone())
                .expect("Failed to serialize diagnostic");

        let start = proto_diagnostic.start.unwrap();
        let end = proto_diagnostic.end.unwrap();
        assert_eq!(start.row, 0);
        assert_eq!(start.column, 1);
        assert_eq!(end.row, 2);
        assert_eq!(end.column, 3);
        assert_eq!(
            proto_diagnostic.severity,
            proto::lsp_diagnostic::Severity::Error as i32
        );
        assert_eq!(proto_diagnostic.code, Some("E001".to_string()));
        assert_eq!(proto_diagnostic.source, Some("test-source".to_string()));
        assert_eq!(proto_diagnostic.message, "Test error message");
    }

    #[test]
    fn test_deserialize_lsp_diagnostic() {
        let proto_diagnostic = proto::LspDiagnostic {
            start: Some(proto::PointUtf16 { row: 0, column: 1 }),
            end: Some(proto::PointUtf16 { row: 2, column: 3 }),
            severity: proto::lsp_diagnostic::Severity::Warning as i32,
            code: Some("ERR".to_string()),
            source: Some("Prism".to_string()),
            message: "assigned but unused variable - a".to_string(),
            related_information: vec![],
            tags: vec![],
            code_description: None,
            data: None,
        };

        let lsp_diagnostic = GetDocumentDiagnostics::deserialize_lsp_diagnostic(proto_diagnostic)
            .expect("Failed to deserialize diagnostic");

        assert_eq!(lsp_diagnostic.range.start.line, 0);
        assert_eq!(lsp_diagnostic.range.start.character, 1);
        assert_eq!(lsp_diagnostic.range.end.line, 2);
        assert_eq!(lsp_diagnostic.range.end.character, 3);
        assert_eq!(lsp_diagnostic.severity, Some(DiagnosticSeverity::WARNING));
        assert_eq!(
            lsp_diagnostic.code,
            Some(lsp::NumberOrString::String("ERR".to_string()))
        );
        assert_eq!(lsp_diagnostic.source, Some("Prism".to_string()));
        assert_eq!(lsp_diagnostic.message, "assigned but unused variable - a");
    }

    #[test]
    fn test_related_information() {
        let related_info = lsp::DiagnosticRelatedInformation {
            location: lsp::Location {
                uri: lsp::Url::parse("file:///test.rs").unwrap(),
                range: lsp::Range {
                    start: lsp::Position::new(1, 1),
                    end: lsp::Position::new(1, 5),
                },
            },
            message: "Related info message".to_string(),
        };

        let lsp_diagnostic = lsp::Diagnostic {
            range: lsp::Range {
                start: lsp::Position::new(0, 0),
                end: lsp::Position::new(0, 1),
            },
            severity: Some(DiagnosticSeverity::INFORMATION),
            code: None,
            source: Some("Prism".to_string()),
            message: "assigned but unused variable - a".to_string(),
            related_information: Some(vec![related_info]),
            tags: None,
            code_description: None,
            data: None,
        };

        let proto_diagnostic = GetDocumentDiagnostics::serialize_lsp_diagnostic(lsp_diagnostic)
            .expect("Failed to serialize diagnostic");

        assert_eq!(proto_diagnostic.related_information.len(), 1);
        let related = &proto_diagnostic.related_information[0];
        assert_eq!(related.location_url, Some("file:///test.rs".to_string()));
        assert_eq!(related.message, "Related info message");
    }

    #[test]
    fn test_invalid_ranges() {
        let proto_diagnostic = proto::LspDiagnostic {
            start: None,
            end: Some(proto::PointUtf16 { row: 2, column: 3 }),
            severity: proto::lsp_diagnostic::Severity::Error as i32,
            code: None,
            source: None,
            message: "Test message".to_string(),
            related_information: vec![],
            tags: vec![],
            code_description: None,
            data: None,
        };

        let result = GetDocumentDiagnostics::deserialize_lsp_diagnostic(proto_diagnostic);
        assert!(result.is_err());
    }
}
