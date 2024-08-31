mod signature_help;

use crate::{
    buffer_store::BufferStore, lsp_store::LspStore, CodeAction, CoreCompletion, DocumentHighlight,
    Hover, HoverBlock, HoverBlockKind, InlayHint, InlayHintLabel, InlayHintLabelPart,
    InlayHintLabelPartTooltip, InlayHintTooltip, Location, LocationLink, MarkupContent,
    ProjectTransaction, ResolveState,
};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use client::proto::{self, PeerId};
use clock::Global;
use collections::HashSet;
use futures::future;
use gpui::{AppContext, AsyncAppContext, Entity, Model};
use language::{
    language_settings::{language_settings, InlayHintKind, LanguageSettings},
    point_from_lsp, point_to_lsp,
    proto::{deserialize_anchor, deserialize_version, serialize_anchor, serialize_version},
    range_from_lsp, range_to_lsp, Anchor, Bias, Buffer, BufferSnapshot, CachedLspAdapter, CharKind,
    OffsetRangeExt, PointUtf16, ToOffset, ToPointUtf16, Transaction, Unclipped,
};
use lsp::{
    AdapterServerCapabilities, CodeActionKind, CodeActionOptions, CompletionContext,
    CompletionListItemDefaultsEditRange, CompletionTriggerKind, DocumentHighlightKind,
    LanguageServer, LanguageServerId, LinkedEditingRangeServerCapabilities, OneOf,
    ServerCapabilities,
};
use signature_help::{lsp_to_proto_signature, proto_to_lsp_signature};
use std::{cmp::Reverse, ops::Range, path::Path, sync::Arc};
use text::{BufferId, LineEnding};

pub use signature_help::{
    SignatureHelp, SIGNATURE_HELP_HIGHLIGHT_CURRENT, SIGNATURE_HELP_HIGHLIGHT_OVERLOAD,
};

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

#[async_trait(?Send)]
pub trait LspCommand: 'static + Sized + Send {
    type Response: 'static + Default + Send;
    type LspRequest: 'static + Send + lsp::request::Request;
    type ProtoRequest: 'static + Send + proto::RequestMessage;

    fn check_capabilities(&self, _: AdapterServerCapabilities) -> bool {
        true
    }

    fn status(&self) -> Option<String> {
        None
    }

    fn to_lsp(
        &self,
        path: &Path,
        buffer: &Buffer,
        language_server: &Arc<LanguageServer>,
        cx: &AppContext,
    ) -> <Self::LspRequest as lsp::request::Request>::Params;

    async fn response_from_lsp(
        self,
        message: <Self::LspRequest as lsp::request::Request>::Result,
        lsp_store: Model<LspStore>,
        buffer: Model<Buffer>,
        server_id: LanguageServerId,
        cx: AsyncAppContext,
    ) -> Result<Self::Response>;

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> Self::ProtoRequest;

    async fn from_proto(
        message: Self::ProtoRequest,
        lsp_store: Model<LspStore>,
        buffer: Model<Buffer>,
        cx: AsyncAppContext,
    ) -> Result<Self>;

    fn response_to_proto(
        response: Self::Response,
        lsp_store: &mut LspStore,
        peer_id: PeerId,
        buffer_version: &clock::Global,
        cx: &mut AppContext,
    ) -> <Self::ProtoRequest as proto::RequestMessage>::Response;

    async fn response_from_proto(
        self,
        message: <Self::ProtoRequest as proto::RequestMessage>::Response,
        lsp_store: Model<LspStore>,
        buffer: Model<Buffer>,
        cx: AsyncAppContext,
    ) -> Result<Self::Response>;

    fn buffer_id_from_proto(message: &Self::ProtoRequest) -> Result<BufferId>;
}

pub(crate) struct PrepareRename {
    pub position: PointUtf16,
}

pub(crate) struct PerformRename {
    pub position: PointUtf16,
    pub new_name: String,
    pub push_to_history: bool,
}

pub struct GetDefinition {
    pub position: PointUtf16,
}

pub(crate) struct GetDeclaration {
    pub position: PointUtf16,
}

pub(crate) struct GetTypeDefinition {
    pub position: PointUtf16,
}

pub(crate) struct GetImplementation {
    pub position: PointUtf16,
}

pub(crate) struct GetReferences {
    pub position: PointUtf16,
}

pub(crate) struct GetDocumentHighlights {
    pub position: PointUtf16,
}

#[derive(Clone)]
pub(crate) struct GetSignatureHelp {
    pub position: PointUtf16,
}

#[derive(Clone)]
pub(crate) struct GetHover {
    pub position: PointUtf16,
}

pub(crate) struct GetCompletions {
    pub position: PointUtf16,
    pub context: CompletionContext,
}

#[derive(Clone)]
pub(crate) struct GetCodeActions {
    pub range: Range<Anchor>,
    pub kinds: Option<Vec<lsp::CodeActionKind>>,
}

pub(crate) struct OnTypeFormatting {
    pub position: PointUtf16,
    pub trigger: String,
    pub options: lsp::FormattingOptions,
    pub push_to_history: bool,
}

pub(crate) struct InlayHints {
    pub range: Range<Anchor>,
}

pub(crate) struct LinkedEditingRange {
    pub position: Anchor,
}

#[async_trait(?Send)]
impl LspCommand for PrepareRename {
    type Response = Option<Range<Anchor>>;
    type LspRequest = lsp::request::PrepareRenameRequest;
    type ProtoRequest = proto::PrepareRename;

    fn check_capabilities(&self, capabilities: AdapterServerCapabilities) -> bool {
        if let Some(lsp::OneOf::Right(rename)) = &capabilities.server_capabilities.rename_provider {
            rename.prepare_provider == Some(true)
        } else {
            false
        }
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &AppContext,
    ) -> lsp::TextDocumentPositionParams {
        lsp::TextDocumentPositionParams {
            text_document: lsp::TextDocumentIdentifier {
                uri: lsp::Url::from_file_path(path).unwrap(),
            },
            position: point_to_lsp(self.position),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::PrepareRenameResponse>,
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        _: LanguageServerId,
        mut cx: AsyncAppContext,
    ) -> Result<Option<Range<Anchor>>> {
        buffer.update(&mut cx, |buffer, _| {
            if let Some(
                lsp::PrepareRenameResponse::Range(range)
                | lsp::PrepareRenameResponse::RangeWithPlaceholder { range, .. },
            ) = message
            {
                let Range { start, end } = range_from_lsp(range);
                if buffer.clip_point_utf16(start, Bias::Left) == start.0
                    && buffer.clip_point_utf16(end, Bias::Left) == end.0
                {
                    return Ok(Some(buffer.anchor_after(start)..buffer.anchor_before(end)));
                }
            }
            Ok(None)
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
        range: Option<Range<Anchor>>,
        _: &mut LspStore,
        _: PeerId,
        buffer_version: &clock::Global,
        _: &mut AppContext,
    ) -> proto::PrepareRenameResponse {
        proto::PrepareRenameResponse {
            can_rename: range.is_some(),
            start: range
                .as_ref()
                .map(|range| language::proto::serialize_anchor(&range.start)),
            end: range
                .as_ref()
                .map(|range| language::proto::serialize_anchor(&range.end)),
            version: serialize_version(buffer_version),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::PrepareRenameResponse,
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Option<Range<Anchor>>> {
        if message.can_rename {
            buffer
                .update(&mut cx, |buffer, _| {
                    buffer.wait_for_version(deserialize_version(&message.version))
                })?
                .await?;
            let start = message.start.and_then(deserialize_anchor);
            let end = message.end.and_then(deserialize_anchor);
            Ok(start.zip(end).map(|(start, end)| start..end))
        } else {
            Ok(None)
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

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &AppContext,
    ) -> lsp::RenameParams {
        lsp::RenameParams {
            text_document_position: lsp::TextDocumentPositionParams {
                text_document: lsp::TextDocumentIdentifier {
                    uri: lsp::Url::from_file_path(path).unwrap(),
                },
                position: point_to_lsp(self.position),
            },
            new_name: self.new_name.clone(),
            work_done_progress_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::WorkspaceEdit>,
        lsp_store: Model<LspStore>,
        buffer: Model<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncAppContext,
    ) -> Result<ProjectTransaction> {
        if let Some(edit) = message {
            let (lsp_adapter, lsp_server) =
                language_server_for_buffer(&lsp_store, &buffer, server_id, &mut cx)?;
            LspStore::deserialize_workspace_edit(
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
        cx: &mut AppContext,
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
        lsp_store: Model<LspStore>,
        _: Model<Buffer>,
        cx: AsyncAppContext,
    ) -> Result<ProjectTransaction> {
        let message = message
            .transaction
            .ok_or_else(|| anyhow!("missing transaction"))?;
        BufferStore::deserialize_project_transaction(
            lsp_store.read_with(&cx, |lsp_store, _| lsp_store.buffer_store().downgrade())?,
            message,
            self.push_to_history,
            cx,
        )
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
        _: &AppContext,
    ) -> lsp::GotoDefinitionParams {
        lsp::GotoDefinitionParams {
            text_document_position_params: lsp::TextDocumentPositionParams {
                text_document: lsp::TextDocumentIdentifier {
                    uri: lsp::Url::from_file_path(path).unwrap(),
                },
                position: point_to_lsp(self.position),
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::GotoDefinitionResponse>,
        lsp_store: Model<LspStore>,
        buffer: Model<Buffer>,
        server_id: LanguageServerId,
        cx: AsyncAppContext,
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
        cx: &mut AppContext,
    ) -> proto::GetDefinitionResponse {
        let links = location_links_to_proto(response, lsp_store, peer_id, cx);
        proto::GetDefinitionResponse { links }
    }

    async fn response_from_proto(
        self,
        message: proto::GetDefinitionResponse,
        lsp_store: Model<LspStore>,
        _: Model<Buffer>,
        cx: AsyncAppContext,
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
        _: &AppContext,
    ) -> lsp::GotoDeclarationParams {
        lsp::GotoDeclarationParams {
            text_document_position_params: lsp::TextDocumentPositionParams {
                text_document: lsp::TextDocumentIdentifier {
                    uri: lsp::Url::from_file_path(path).unwrap(),
                },
                position: point_to_lsp(self.position),
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::GotoDeclarationResponse>,
        lsp_store: Model<LspStore>,
        buffer: Model<Buffer>,
        server_id: LanguageServerId,
        cx: AsyncAppContext,
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
        cx: &mut AppContext,
    ) -> proto::GetDeclarationResponse {
        let links = location_links_to_proto(response, lsp_store, peer_id, cx);
        proto::GetDeclarationResponse { links }
    }

    async fn response_from_proto(
        self,
        message: proto::GetDeclarationResponse,
        lsp_store: Model<LspStore>,
        _: Model<Buffer>,
        cx: AsyncAppContext,
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

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &AppContext,
    ) -> lsp::GotoImplementationParams {
        lsp::GotoImplementationParams {
            text_document_position_params: lsp::TextDocumentPositionParams {
                text_document: lsp::TextDocumentIdentifier {
                    uri: lsp::Url::from_file_path(path).unwrap(),
                },
                position: point_to_lsp(self.position),
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::GotoImplementationResponse>,
        lsp_store: Model<LspStore>,
        buffer: Model<Buffer>,
        server_id: LanguageServerId,
        cx: AsyncAppContext,
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
        cx: &mut AppContext,
    ) -> proto::GetImplementationResponse {
        let links = location_links_to_proto(response, lsp_store, peer_id, cx);
        proto::GetImplementationResponse { links }
    }

    async fn response_from_proto(
        self,
        message: proto::GetImplementationResponse,
        project: Model<LspStore>,
        _: Model<Buffer>,
        cx: AsyncAppContext,
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

    fn check_capabilities(&self, capabilities: AdapterServerCapabilities) -> bool {
        match &capabilities.server_capabilities.type_definition_provider {
            None => false,
            Some(lsp::TypeDefinitionProviderCapability::Simple(false)) => false,
            _ => true,
        }
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &AppContext,
    ) -> lsp::GotoTypeDefinitionParams {
        lsp::GotoTypeDefinitionParams {
            text_document_position_params: lsp::TextDocumentPositionParams {
                text_document: lsp::TextDocumentIdentifier {
                    uri: lsp::Url::from_file_path(path).unwrap(),
                },
                position: point_to_lsp(self.position),
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::GotoTypeDefinitionResponse>,
        project: Model<LspStore>,
        buffer: Model<Buffer>,
        server_id: LanguageServerId,
        cx: AsyncAppContext,
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
        cx: &mut AppContext,
    ) -> proto::GetTypeDefinitionResponse {
        let links = location_links_to_proto(response, lsp_store, peer_id, cx);
        proto::GetTypeDefinitionResponse { links }
    }

    async fn response_from_proto(
        self,
        message: proto::GetTypeDefinitionResponse,
        project: Model<LspStore>,
        _: Model<Buffer>,
        cx: AsyncAppContext,
    ) -> Result<Vec<LocationLink>> {
        location_links_from_proto(message.links, project, cx).await
    }

    fn buffer_id_from_proto(message: &proto::GetTypeDefinition) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

fn language_server_for_buffer(
    lsp_store: &Model<LspStore>,
    buffer: &Model<Buffer>,
    server_id: LanguageServerId,
    cx: &mut AsyncAppContext,
) -> Result<(Arc<CachedLspAdapter>, Arc<LanguageServer>)> {
    lsp_store
        .update(cx, |lsp_store, cx| {
            lsp_store
                .language_server_for_buffer(buffer.read(cx), server_id, cx)
                .map(|(adapter, server)| (adapter.clone(), server.clone()))
        })?
        .ok_or_else(|| anyhow!("no language server found for buffer"))
}

async fn location_links_from_proto(
    proto_links: Vec<proto::LocationLink>,
    lsp_store: Model<LspStore>,
    mut cx: AsyncAppContext,
) -> Result<Vec<LocationLink>> {
    let mut links = Vec::new();

    for link in proto_links {
        let origin = match link.origin {
            Some(origin) => {
                let buffer_id = BufferId::new(origin.buffer_id)?;
                let buffer = lsp_store
                    .update(&mut cx, |lsp_store, cx| {
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
                    .update(&mut cx, |buffer, _| buffer.wait_for_anchors([start, end]))?
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
            .update(&mut cx, |lsp_store, cx| {
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
            .update(&mut cx, |buffer, _| buffer.wait_for_anchors([start, end]))?
            .await?;
        let target = Location {
            buffer,
            range: start..end,
        };

        links.push(LocationLink { origin, target })
    }

    Ok(links)
}

async fn location_links_from_lsp(
    message: Option<lsp::GotoDefinitionResponse>,
    lsp_store: Model<LspStore>,
    buffer: Model<Buffer>,
    server_id: LanguageServerId,
    mut cx: AsyncAppContext,
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

fn location_links_to_proto(
    links: Vec<LocationLink>,
    lsp_store: &mut LspStore,
    peer_id: PeerId,
    cx: &mut AppContext,
) -> Vec<proto::LocationLink> {
    links
        .into_iter()
        .map(|definition| {
            let origin = definition.origin.map(|origin| {
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
                    buffer_store.create_buffer_for_peer(&definition.target.buffer, peer_id, cx)
                })
                .detach_and_log_err(cx);

            let buffer_id = definition.target.buffer.read(cx).remote_id().into();
            let target = proto::Location {
                start: Some(serialize_anchor(&definition.target.range.start)),
                end: Some(serialize_anchor(&definition.target.range.end)),
                buffer_id,
            };

            proto::LocationLink {
                origin,
                target: Some(target),
            }
        })
        .collect()
}

#[async_trait(?Send)]
impl LspCommand for GetReferences {
    type Response = Vec<Location>;
    type LspRequest = lsp::request::References;
    type ProtoRequest = proto::GetReferences;

    fn status(&self) -> Option<String> {
        return Some("Finding references...".to_owned());
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
        _: &AppContext,
    ) -> lsp::ReferenceParams {
        lsp::ReferenceParams {
            text_document_position: lsp::TextDocumentPositionParams {
                text_document: lsp::TextDocumentIdentifier {
                    uri: lsp::Url::from_file_path(path).unwrap(),
                },
                position: point_to_lsp(self.position),
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: lsp::ReferenceContext {
                include_declaration: true,
            },
        }
    }

    async fn response_from_lsp(
        self,
        locations: Option<Vec<lsp::Location>>,
        lsp_store: Model<LspStore>,
        buffer: Model<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncAppContext,
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
        cx: &mut AppContext,
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
        project: Model<LspStore>,
        _: Model<Buffer>,
        mut cx: AsyncAppContext,
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
        _: &AppContext,
    ) -> lsp::DocumentHighlightParams {
        lsp::DocumentHighlightParams {
            text_document_position_params: lsp::TextDocumentPositionParams {
                text_document: lsp::TextDocumentIdentifier {
                    uri: lsp::Url::from_file_path(path).unwrap(),
                },
                position: point_to_lsp(self.position),
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        lsp_highlights: Option<Vec<lsp::DocumentHighlight>>,
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        _: LanguageServerId,
        mut cx: AsyncAppContext,
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
        _: &mut AppContext,
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
impl LspCommand for GetSignatureHelp {
    type Response = Option<SignatureHelp>;
    type LspRequest = lsp::SignatureHelpRequest;
    type ProtoRequest = proto::GetSignatureHelp;

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
        _cx: &AppContext,
    ) -> lsp::SignatureHelpParams {
        let url_result = lsp::Url::from_file_path(path);
        if url_result.is_err() {
            log::error!("an invalid file path has been specified");
        }

        lsp::SignatureHelpParams {
            text_document_position_params: lsp::TextDocumentPositionParams {
                text_document: lsp::TextDocumentIdentifier {
                    uri: url_result.expect("invalid file path"),
                },
                position: point_to_lsp(self.position),
            },
            context: None,
            work_done_progress_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::SignatureHelp>,
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        _: LanguageServerId,
        mut cx: AsyncAppContext,
    ) -> Result<Self::Response> {
        let language = buffer.update(&mut cx, |buffer, _| buffer.language().cloned())?;
        Ok(message.and_then(|message| SignatureHelp::new(message, language)))
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
        _: &mut AppContext,
    ) -> proto::GetSignatureHelpResponse {
        proto::GetSignatureHelpResponse {
            signature_help: response
                .map(|signature_help| lsp_to_proto_signature(signature_help.original_data)),
        }
    }

    async fn response_from_proto(
        self,
        response: proto::GetSignatureHelpResponse,
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Self::Response> {
        let language = buffer.update(&mut cx, |buffer, _| buffer.language().cloned())?;
        Ok(response
            .signature_help
            .map(|proto_help| proto_to_lsp_signature(proto_help))
            .and_then(|lsp_help| SignatureHelp::new(lsp_help, language)))
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
        _: &AppContext,
    ) -> lsp::HoverParams {
        lsp::HoverParams {
            text_document_position_params: lsp::TextDocumentPositionParams {
                text_document: lsp::TextDocumentIdentifier {
                    uri: lsp::Url::from_file_path(path).unwrap(),
                },
                position: point_to_lsp(self.position),
            },
            work_done_progress_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::Hover>,
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        _: LanguageServerId,
        mut cx: AsyncAppContext,
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
        _: &mut AppContext,
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &AppContext,
    ) -> lsp::CompletionParams {
        lsp::CompletionParams {
            text_document_position: lsp::TextDocumentPositionParams::new(
                lsp::TextDocumentIdentifier::new(lsp::Url::from_file_path(path).unwrap()),
                point_to_lsp(self.position),
            ),
            context: Some(self.context.clone()),
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        completions: Option<lsp::CompletionResponse>,
        lsp_store: Model<LspStore>,
        buffer: Model<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncAppContext,
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
            Default::default()
        };

        let language_server_adapter = lsp_store
            .update(&mut cx, |lsp_store, _| {
                lsp_store.language_server_adapter_for_id(server_id)
            })?
            .ok_or_else(|| anyhow!("no such language server"))?;

        let item_defaults = response_list
            .as_ref()
            .and_then(|list| list.item_defaults.as_ref());

        if let Some(item_defaults) = item_defaults {
            let default_data = item_defaults.data.as_ref();
            let default_commit_characters = item_defaults.commit_characters.as_ref();
            let default_insert_text_mode = item_defaults.insert_text_mode.as_ref();

            if default_data.is_some()
                || default_commit_characters.is_some()
                || default_insert_text_mode.is_some()
            {
                for item in completions.iter_mut() {
                    if let Some(data) = default_data {
                        item.data = Some(data.clone())
                    }
                    if let Some(characters) = default_commit_characters {
                        item.commit_characters = Some(characters.clone())
                    }
                    if let Some(text_mode) = default_insert_text_mode {
                        item.insert_text_mode = Some(*text_mode)
                    }
                }
            }
        }

        let mut completion_edits = Vec::new();
        buffer.update(&mut cx, |buffer, _cx| {
            let snapshot = buffer.snapshot();
            let clipped_position = buffer.clip_point_utf16(Unclipped(self.position), Bias::Left);

            let mut range_for_token = None;
            completions.retain_mut(|lsp_completion| {
                let edit = match lsp_completion.text_edit.as_ref() {
                    // If the language server provides a range to overwrite, then
                    // check that the range is valid.
                    Some(completion_text_edit) => {
                        match parse_completion_text_edit(completion_text_edit, &snapshot) {
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

                        let default_edit_range = response_list
                            .as_ref()
                            .and_then(|list| list.item_defaults.as_ref())
                            .and_then(|defaults| defaults.edit_range.as_ref())
                            .and_then(|range| match range {
                                CompletionListItemDefaultsEditRange::Range(r) => Some(r),
                                _ => None,
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

                        let text = lsp_completion
                            .insert_text
                            .as_ref()
                            .unwrap_or(&lsp_completion.label)
                            .clone();
                        (range, text)
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
            .map(|(lsp_completion, (old_range, mut new_text))| {
                LineEnding::normalize(&mut new_text);
                CoreCompletion {
                    old_range,
                    new_text,
                    server_id,
                    lsp_completion,
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
        _: &mut AppContext,
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
        _project: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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

pub(crate) fn parse_completion_text_edit(
    edit: &lsp::CompletionTextEdit,
    snapshot: &BufferSnapshot,
) -> Option<(Range<Anchor>, String)> {
    match edit {
        lsp::CompletionTextEdit::Edit(edit) => {
            let range = range_from_lsp(edit.range);
            let start = snapshot.clip_point_utf16(range.start, Bias::Left);
            let end = snapshot.clip_point_utf16(range.end, Bias::Left);
            if start != range.start.0 || end != range.end.0 {
                log::info!("completion out of expected range");
                None
            } else {
                Some((
                    snapshot.anchor_before(start)..snapshot.anchor_after(end),
                    edit.new_text.clone(),
                ))
            }
        }

        lsp::CompletionTextEdit::InsertAndReplace(edit) => {
            let range = range_from_lsp(edit.insert);

            let start = snapshot.clip_point_utf16(range.start, Bias::Left);
            let end = snapshot.clip_point_utf16(range.end, Bias::Left);
            if start != range.start.0 || end != range.end.0 {
                log::info!("completion out of expected range");
                None
            } else {
                Some((
                    snapshot.anchor_before(start)..snapshot.anchor_after(end),
                    edit.new_text.clone(),
                ))
            }
        }
    }
}

#[async_trait(?Send)]
impl LspCommand for GetCodeActions {
    type Response = Vec<CodeAction>;
    type LspRequest = lsp::request::CodeActionRequest;
    type ProtoRequest = proto::GetCodeActions;

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
        _: &AppContext,
    ) -> lsp::CodeActionParams {
        let relevant_diagnostics = buffer
            .snapshot()
            .diagnostics_in_range::<_, language::PointUtf16>(self.range.clone(), false)
            .map(|entry| entry.to_lsp_diagnostic_stub())
            .collect::<Vec<_>>();

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

        lsp::CodeActionParams {
            text_document: lsp::TextDocumentIdentifier::new(
                lsp::Url::from_file_path(path).unwrap(),
            ),
            range: range_to_lsp(self.range.to_point_utf16(buffer)),
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: lsp::CodeActionContext {
                diagnostics: relevant_diagnostics,
                only,
                ..lsp::CodeActionContext::default()
            },
        }
    }

    async fn response_from_lsp(
        self,
        actions: Option<lsp::CodeActionResponse>,
        _: Model<LspStore>,
        _: Model<Buffer>,
        server_id: LanguageServerId,
        _: AsyncAppContext,
    ) -> Result<Vec<CodeAction>> {
        Ok(actions
            .unwrap_or_default()
            .into_iter()
            .filter_map(|entry| {
                if let lsp::CodeActionOrCommand::CodeAction(lsp_action) = entry {
                    Some(CodeAction {
                        server_id,
                        range: self.range.clone(),
                        lsp_action,
                    })
                } else {
                    None
                }
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
        _: &mut AppContext,
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
        _: &AppContext,
    ) -> lsp::DocumentOnTypeFormattingParams {
        lsp::DocumentOnTypeFormattingParams {
            text_document_position: lsp::TextDocumentPositionParams::new(
                lsp::TextDocumentIdentifier::new(lsp::Url::from_file_path(path).unwrap()),
                point_to_lsp(self.position),
            ),
            ch: self.trigger.clone(),
            options: self.options.clone(),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<Vec<lsp::TextEdit>>,
        lsp_store: Model<LspStore>,
        buffer: Model<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncAppContext,
    ) -> Result<Option<Transaction>> {
        if let Some(edits) = message {
            let (lsp_adapter, lsp_server) =
                language_server_for_buffer(&lsp_store, &buffer, server_id, &mut cx)?;
            LspStore::deserialize_text_edits(
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
            lsp_formatting_options(language_settings(buffer.language(), buffer.file(), cx))
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
        _: &mut AppContext,
    ) -> proto::OnTypeFormattingResponse {
        proto::OnTypeFormattingResponse {
            transaction: response
                .map(|transaction| language::proto::serialize_transaction(&transaction)),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::OnTypeFormattingResponse,
        _: Model<LspStore>,
        _: Model<Buffer>,
        _: AsyncAppContext,
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
        buffer_handle: &Model<Buffer>,
        server_id: LanguageServerId,
        resolve_state: ResolveState,
        force_no_type_left_padding: bool,
        cx: &mut AsyncAppContext,
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
                resolve_data
                    .map(|json_data| {
                        serde_json::to_string(&json_data)
                            .expect("failed to serialize resolve json data")
                    })
                    .map(|value| proto::resolve_state::LspResolveState {
                        server_id: server_id.0 as u64,
                        value,
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
                serde_json::from_str::<Option<lsp::LSPAny>>(&lsp_resolve_state.value)
                    .with_context(|| format!("incorrect proto inlay hint message: non-json resolve state {lsp_resolve_state:?}"))
                    .map(|state| (LanguageServerId(lsp_resolve_state.server_id as usize), state))
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
        _: &AppContext,
    ) -> lsp::InlayHintParams {
        lsp::InlayHintParams {
            text_document: lsp::TextDocumentIdentifier {
                uri: lsp::Url::from_file_path(path).unwrap(),
            },
            range: range_to_lsp(self.range.to_point_utf16(buffer)),
            work_done_progress_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<Vec<lsp::InlayHint>>,
        lsp_store: Model<LspStore>,
        buffer: Model<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncAppContext,
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
            cx.spawn(move |mut cx| async move {
                InlayHints::lsp_to_project_hint(
                    lsp_hint,
                    &buffer,
                    server_id,
                    resolve_state,
                    force_no_type_left_padding,
                    &mut cx,
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
        _: &mut AppContext,
    ) -> proto::InlayHintsResponse {
        proto::InlayHintsResponse {
            hints: response
                .into_iter()
                .map(|response_hint| InlayHints::project_to_proto_hint(response_hint))
                .collect(),
            version: serialize_version(buffer_version),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::InlayHintsResponse,
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
impl LspCommand for LinkedEditingRange {
    type Response = Vec<Range<Anchor>>;
    type LspRequest = lsp::request::LinkedEditingRange;
    type ProtoRequest = proto::LinkedEditingRange;

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
        return true;
    }

    fn to_lsp(
        &self,
        path: &Path,
        buffer: &Buffer,
        _server: &Arc<LanguageServer>,
        _: &AppContext,
    ) -> lsp::LinkedEditingRangeParams {
        let position = self.position.to_point_utf16(&buffer.snapshot());
        lsp::LinkedEditingRangeParams {
            text_document_position_params: lsp::TextDocumentPositionParams::new(
                lsp::TextDocumentIdentifier::new(lsp::Url::from_file_path(path).unwrap()),
                point_to_lsp(position),
            ),
            work_done_progress_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp::LinkedEditingRanges>,
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        _server_id: LanguageServerId,
        cx: AsyncAppContext,
    ) -> Result<Vec<Range<Anchor>>> {
        if let Some(lsp::LinkedEditingRanges { mut ranges, .. }) = message {
            ranges.sort_by_key(|range| range.start);
            let ranges = buffer.read_with(&cx, |buffer, _| {
                ranges
                    .into_iter()
                    .map(|range| {
                        let start =
                            buffer.clip_point_utf16(point_from_lsp(range.start), Bias::Left);
                        let end = buffer.clip_point_utf16(point_from_lsp(range.end), Bias::Left);
                        buffer.anchor_before(start)..buffer.anchor_after(end)
                    })
                    .collect()
            });

            ranges
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
        _: &mut AppContext,
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
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
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
