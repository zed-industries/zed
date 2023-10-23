use crate::{
    DocumentHighlight, Hover, HoverBlock, HoverBlockKind, InlayHint, InlayHintLabel,
    InlayHintLabelPart, InlayHintLabelPartTooltip, InlayHintTooltip, Location, LocationLink,
    MarkupContent, Project, ProjectTransaction, ResolveState,
};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use client2::proto::{self, PeerId};
use futures::future;
use gpui2::{AppContext, AsyncAppContext, Handle};
use language2::{
    language_settings::{language_settings, InlayHintKind},
    point_from_lsp, point_to_lsp,
    proto::{deserialize_anchor, deserialize_version, serialize_anchor, serialize_version},
    range_from_lsp, range_to_lsp, Anchor, Bias, Buffer, BufferSnapshot, CachedLspAdapter, CharKind,
    CodeAction, Completion, OffsetRangeExt, PointUtf16, ToOffset, ToPointUtf16, Transaction,
    Unclipped,
};
use lsp2::{
    CompletionListItemDefaultsEditRange, DocumentHighlightKind, LanguageServer, LanguageServerId,
    OneOf, ServerCapabilities,
};
use std::{cmp::Reverse, ops::Range, path::Path, sync::Arc};
use text::LineEnding;

pub fn lsp_formatting_options(tab_size: u32) -> lsp2::FormattingOptions {
    lsp2::FormattingOptions {
        tab_size,
        insert_spaces: true,
        insert_final_newline: Some(true),
        ..lsp2::FormattingOptions::default()
    }
}

#[async_trait(?Send)]
pub(crate) trait LspCommand: 'static + Sized {
    type Response: 'static + Default + Send;
    type LspRequest: 'static + Send + lsp2::request::Request;
    type ProtoRequest: 'static + Send + proto::RequestMessage;

    fn check_capabilities(&self, _: &lsp2::ServerCapabilities) -> bool {
        true
    }

    fn to_lsp(
        &self,
        path: &Path,
        buffer: &Buffer,
        language_server: &Arc<LanguageServer>,
        cx: &AppContext,
    ) -> <Self::LspRequest as lsp2::request::Request>::Params;

    async fn response_from_lsp(
        self,
        message: <Self::LspRequest as lsp2::request::Request>::Result,
        project: Handle<Project>,
        buffer: Handle<Buffer>,
        server_id: LanguageServerId,
        cx: AsyncAppContext,
    ) -> Result<Self::Response>;

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> Self::ProtoRequest;

    async fn from_proto(
        message: Self::ProtoRequest,
        project: Handle<Project>,
        buffer: Handle<Buffer>,
        cx: AsyncAppContext,
    ) -> Result<Self>;

    fn response_to_proto(
        response: Self::Response,
        project: &mut Project,
        peer_id: PeerId,
        buffer_version: &clock::Global,
        cx: &mut AppContext,
    ) -> <Self::ProtoRequest as proto::RequestMessage>::Response;

    async fn response_from_proto(
        self,
        message: <Self::ProtoRequest as proto::RequestMessage>::Response,
        project: Handle<Project>,
        buffer: Handle<Buffer>,
        cx: AsyncAppContext,
    ) -> Result<Self::Response>;

    fn buffer_id_from_proto(message: &Self::ProtoRequest) -> u64;
}

pub(crate) struct PrepareRename {
    pub position: PointUtf16,
}

pub(crate) struct PerformRename {
    pub position: PointUtf16,
    pub new_name: String,
    pub push_to_history: bool,
}

pub(crate) struct GetDefinition {
    pub position: PointUtf16,
}

pub(crate) struct GetTypeDefinition {
    pub position: PointUtf16,
}

pub(crate) struct GetReferences {
    pub position: PointUtf16,
}

pub(crate) struct GetDocumentHighlights {
    pub position: PointUtf16,
}

pub(crate) struct GetHover {
    pub position: PointUtf16,
}

pub(crate) struct GetCompletions {
    pub position: PointUtf16,
}

pub(crate) struct GetCodeActions {
    pub range: Range<Anchor>,
}

pub(crate) struct OnTypeFormatting {
    pub position: PointUtf16,
    pub trigger: String,
    pub options: FormattingOptions,
    pub push_to_history: bool,
}

pub(crate) struct InlayHints {
    pub range: Range<Anchor>,
}

pub(crate) struct FormattingOptions {
    tab_size: u32,
}

impl From<lsp2::FormattingOptions> for FormattingOptions {
    fn from(value: lsp2::FormattingOptions) -> Self {
        Self {
            tab_size: value.tab_size,
        }
    }
}

#[async_trait(?Send)]
impl LspCommand for PrepareRename {
    type Response = Option<Range<Anchor>>;
    type LspRequest = lsp2::request::PrepareRenameRequest;
    type ProtoRequest = proto::PrepareRename;

    fn check_capabilities(&self, capabilities: &ServerCapabilities) -> bool {
        if let Some(lsp2::OneOf::Right(rename)) = &capabilities.rename_provider {
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
    ) -> lsp2::TextDocumentPositionParams {
        lsp2::TextDocumentPositionParams {
            text_document: lsp2::TextDocumentIdentifier {
                uri: lsp2::Url::from_file_path(path).unwrap(),
            },
            position: point_to_lsp(self.position),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp2::PrepareRenameResponse>,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
        _: LanguageServerId,
        cx: AsyncAppContext,
    ) -> Result<Option<Range<Anchor>>> {
        buffer.update(&mut cx, |buffer, _| {
            if let Some(
                lsp2::PrepareRenameResponse::Range(range)
                | lsp2::PrepareRenameResponse::RangeWithPlaceholder { range, .. },
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
            buffer_id: buffer.remote_id(),
            position: Some(language2::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::PrepareRename,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
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
        _: &mut Project,
        _: PeerId,
        buffer_version: &clock::Global,
        _: &mut AppContext,
    ) -> proto::PrepareRenameResponse {
        proto::PrepareRenameResponse {
            can_rename: range.is_some(),
            start: range
                .as_ref()
                .map(|range| language2::proto::serialize_anchor(&range.start)),
            end: range
                .as_ref()
                .map(|range| language2::proto::serialize_anchor(&range.end)),
            version: serialize_version(buffer_version),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::PrepareRenameResponse,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
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

    fn buffer_id_from_proto(message: &proto::PrepareRename) -> u64 {
        message.buffer_id
    }
}

#[async_trait(?Send)]
impl LspCommand for PerformRename {
    type Response = ProjectTransaction;
    type LspRequest = lsp2::request::Rename;
    type ProtoRequest = proto::PerformRename;

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &AppContext,
    ) -> lsp2::RenameParams {
        lsp2::RenameParams {
            text_document_position: lsp2::TextDocumentPositionParams {
                text_document: lsp2::TextDocumentIdentifier {
                    uri: lsp2::Url::from_file_path(path).unwrap(),
                },
                position: point_to_lsp(self.position),
            },
            new_name: self.new_name.clone(),
            work_done_progress_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp2::WorkspaceEdit>,
        project: Handle<Project>,
        buffer: Handle<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncAppContext,
    ) -> Result<ProjectTransaction> {
        if let Some(edit) = message {
            let (lsp_adapter, lsp_server) =
                language_server_for_buffer(&project, &buffer, server_id, &mut cx)?;
            Project::deserialize_workspace_edit(
                project,
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
            buffer_id: buffer.remote_id(),
            position: Some(language2::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            new_name: self.new_name.clone(),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::PerformRename,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
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
        project: &mut Project,
        peer_id: PeerId,
        _: &clock::Global,
        cx: &mut AppContext,
    ) -> proto::PerformRenameResponse {
        let transaction = project.serialize_project_transaction_for_peer(response, peer_id, cx);
        proto::PerformRenameResponse {
            transaction: Some(transaction),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::PerformRenameResponse,
        project: Handle<Project>,
        _: Handle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<ProjectTransaction> {
        let message = message
            .transaction
            .ok_or_else(|| anyhow!("missing transaction"))?;
        project
            .update(&mut cx, |project, cx| {
                project.deserialize_project_transaction(message, self.push_to_history, cx)
            })?
            .await
    }

    fn buffer_id_from_proto(message: &proto::PerformRename) -> u64 {
        message.buffer_id
    }
}

#[async_trait(?Send)]
impl LspCommand for GetDefinition {
    type Response = Vec<LocationLink>;
    type LspRequest = lsp2::request::GotoDefinition;
    type ProtoRequest = proto::GetDefinition;

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &AppContext,
    ) -> lsp2::GotoDefinitionParams {
        lsp2::GotoDefinitionParams {
            text_document_position_params: lsp2::TextDocumentPositionParams {
                text_document: lsp2::TextDocumentIdentifier {
                    uri: lsp2::Url::from_file_path(path).unwrap(),
                },
                position: point_to_lsp(self.position),
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp2::GotoDefinitionResponse>,
        project: Handle<Project>,
        buffer: Handle<Buffer>,
        server_id: LanguageServerId,
        cx: AsyncAppContext,
    ) -> Result<Vec<LocationLink>> {
        location_links_from_lsp(message, project, buffer, server_id, cx).await
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetDefinition {
        proto::GetDefinition {
            project_id,
            buffer_id: buffer.remote_id(),
            position: Some(language2::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetDefinition,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
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
        project: &mut Project,
        peer_id: PeerId,
        _: &clock::Global,
        cx: &mut AppContext,
    ) -> proto::GetDefinitionResponse {
        let links = location_links_to_proto(response, project, peer_id, cx);
        proto::GetDefinitionResponse { links }
    }

    async fn response_from_proto(
        self,
        message: proto::GetDefinitionResponse,
        project: Handle<Project>,
        _: Handle<Buffer>,
        cx: AsyncAppContext,
    ) -> Result<Vec<LocationLink>> {
        location_links_from_proto(message.links, project, cx).await
    }

    fn buffer_id_from_proto(message: &proto::GetDefinition) -> u64 {
        message.buffer_id
    }
}

#[async_trait(?Send)]
impl LspCommand for GetTypeDefinition {
    type Response = Vec<LocationLink>;
    type LspRequest = lsp2::request::GotoTypeDefinition;
    type ProtoRequest = proto::GetTypeDefinition;

    fn check_capabilities(&self, capabilities: &ServerCapabilities) -> bool {
        match &capabilities.type_definition_provider {
            None => false,
            Some(lsp2::TypeDefinitionProviderCapability::Simple(false)) => false,
            _ => true,
        }
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &AppContext,
    ) -> lsp2::GotoTypeDefinitionParams {
        lsp2::GotoTypeDefinitionParams {
            text_document_position_params: lsp2::TextDocumentPositionParams {
                text_document: lsp2::TextDocumentIdentifier {
                    uri: lsp2::Url::from_file_path(path).unwrap(),
                },
                position: point_to_lsp(self.position),
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp2::GotoTypeDefinitionResponse>,
        project: Handle<Project>,
        buffer: Handle<Buffer>,
        server_id: LanguageServerId,
        cx: AsyncAppContext,
    ) -> Result<Vec<LocationLink>> {
        location_links_from_lsp(message, project, buffer, server_id, cx).await
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetTypeDefinition {
        proto::GetTypeDefinition {
            project_id,
            buffer_id: buffer.remote_id(),
            position: Some(language2::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetTypeDefinition,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
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
        project: &mut Project,
        peer_id: PeerId,
        _: &clock::Global,
        cx: &mut AppContext,
    ) -> proto::GetTypeDefinitionResponse {
        let links = location_links_to_proto(response, project, peer_id, cx);
        proto::GetTypeDefinitionResponse { links }
    }

    async fn response_from_proto(
        self,
        message: proto::GetTypeDefinitionResponse,
        project: Handle<Project>,
        _: Handle<Buffer>,
        cx: AsyncAppContext,
    ) -> Result<Vec<LocationLink>> {
        location_links_from_proto(message.links, project, cx).await
    }

    fn buffer_id_from_proto(message: &proto::GetTypeDefinition) -> u64 {
        message.buffer_id
    }
}

fn language_server_for_buffer(
    project: &Handle<Project>,
    buffer: &Handle<Buffer>,
    server_id: LanguageServerId,
    cx: &mut AsyncAppContext,
) -> Result<(Arc<CachedLspAdapter>, Arc<LanguageServer>)> {
    project
        .update(cx, |project, cx| {
            project
                .language_server_for_buffer(buffer.read(cx), server_id, cx)
                .map(|(adapter, server)| (adapter.clone(), server.clone()))
        })?
        .ok_or_else(|| anyhow!("no language server found for buffer"))
}

async fn location_links_from_proto(
    proto_links: Vec<proto::LocationLink>,
    project: Handle<Project>,
    mut cx: AsyncAppContext,
) -> Result<Vec<LocationLink>> {
    let mut links = Vec::new();

    for link in proto_links {
        let origin = match link.origin {
            Some(origin) => {
                let buffer = project
                    .update(&mut cx, |this, cx| {
                        this.wait_for_remote_buffer(origin.buffer_id, cx)
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
        let buffer = project
            .update(&mut cx, |this, cx| {
                this.wait_for_remote_buffer(target.buffer_id, cx)
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
    message: Option<lsp2::GotoDefinitionResponse>,
    project: Handle<Project>,
    buffer: Handle<Buffer>,
    server_id: LanguageServerId,
    mut cx: AsyncAppContext,
) -> Result<Vec<LocationLink>> {
    let message = match message {
        Some(message) => message,
        None => return Ok(Vec::new()),
    };

    let mut unresolved_links = Vec::new();
    match message {
        lsp2::GotoDefinitionResponse::Scalar(loc) => {
            unresolved_links.push((None, loc.uri, loc.range));
        }

        lsp2::GotoDefinitionResponse::Array(locs) => {
            unresolved_links.extend(locs.into_iter().map(|l| (None, l.uri, l.range)));
        }

        lsp2::GotoDefinitionResponse::Link(links) => {
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
        language_server_for_buffer(&project, &buffer, server_id, &mut cx)?;
    let mut definitions = Vec::new();
    for (origin_range, target_uri, target_range) in unresolved_links {
        let target_buffer_handle = project
            .update(&mut cx, |this, cx| {
                this.open_local_buffer_via_lsp(
                    target_uri,
                    language_server.server_id(),
                    lsp_adapter.name.clone(),
                    cx,
                )
            })?
            .await?;

        buffer.update(&mut cx, |origin_buffer, cx| {
            let origin_location = origin_range.map(|origin_range| {
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
    project: &mut Project,
    peer_id: PeerId,
    cx: &mut AppContext,
) -> Vec<proto::LocationLink> {
    links
        .into_iter()
        .map(|definition| {
            let origin = definition.origin.map(|origin| {
                let buffer_id = project.create_buffer_for_peer(&origin.buffer, peer_id, cx);
                proto::Location {
                    start: Some(serialize_anchor(&origin.range.start)),
                    end: Some(serialize_anchor(&origin.range.end)),
                    buffer_id,
                }
            });

            let buffer_id = project.create_buffer_for_peer(&definition.target.buffer, peer_id, cx);
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
    type LspRequest = lsp2::request::References;
    type ProtoRequest = proto::GetReferences;

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &AppContext,
    ) -> lsp2::ReferenceParams {
        lsp2::ReferenceParams {
            text_document_position: lsp2::TextDocumentPositionParams {
                text_document: lsp2::TextDocumentIdentifier {
                    uri: lsp2::Url::from_file_path(path).unwrap(),
                },
                position: point_to_lsp(self.position),
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: lsp2::ReferenceContext {
                include_declaration: true,
            },
        }
    }

    async fn response_from_lsp(
        self,
        locations: Option<Vec<lsp2::Location>>,
        project: Handle<Project>,
        buffer: Handle<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncAppContext,
    ) -> Result<Vec<Location>> {
        let mut references = Vec::new();
        let (lsp_adapter, language_server) =
            language_server_for_buffer(&project, &buffer, server_id, &mut cx)?;

        if let Some(locations) = locations {
            for lsp_location in locations {
                let target_buffer_handle = project
                    .update(&mut cx, |this, cx| {
                        this.open_local_buffer_via_lsp(
                            lsp_location.uri,
                            language_server.server_id(),
                            lsp_adapter.name.clone(),
                            cx,
                        )
                    })?
                    .await?;

                target_buffer_handle.update(&mut cx, |target_buffer, cx| {
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
            buffer_id: buffer.remote_id(),
            position: Some(language2::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetReferences,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
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
        project: &mut Project,
        peer_id: PeerId,
        _: &clock::Global,
        cx: &mut AppContext,
    ) -> proto::GetReferencesResponse {
        let locations = response
            .into_iter()
            .map(|definition| {
                let buffer_id = project.create_buffer_for_peer(&definition.buffer, peer_id, cx);
                proto::Location {
                    start: Some(serialize_anchor(&definition.range.start)),
                    end: Some(serialize_anchor(&definition.range.end)),
                    buffer_id,
                }
            })
            .collect();
        proto::GetReferencesResponse { locations }
    }

    async fn response_from_proto(
        self,
        message: proto::GetReferencesResponse,
        project: Handle<Project>,
        _: Handle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Vec<Location>> {
        let mut locations = Vec::new();
        for location in message.locations {
            let target_buffer = project
                .update(&mut cx, |this, cx| {
                    this.wait_for_remote_buffer(location.buffer_id, cx)
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

    fn buffer_id_from_proto(message: &proto::GetReferences) -> u64 {
        message.buffer_id
    }
}

#[async_trait(?Send)]
impl LspCommand for GetDocumentHighlights {
    type Response = Vec<DocumentHighlight>;
    type LspRequest = lsp2::request::DocumentHighlightRequest;
    type ProtoRequest = proto::GetDocumentHighlights;

    fn check_capabilities(&self, capabilities: &ServerCapabilities) -> bool {
        capabilities.document_highlight_provider.is_some()
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &AppContext,
    ) -> lsp2::DocumentHighlightParams {
        lsp2::DocumentHighlightParams {
            text_document_position_params: lsp2::TextDocumentPositionParams {
                text_document: lsp2::TextDocumentIdentifier {
                    uri: lsp2::Url::from_file_path(path).unwrap(),
                },
                position: point_to_lsp(self.position),
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        lsp_highlights: Option<Vec<lsp2::DocumentHighlight>>,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
        _: LanguageServerId,
        cx: AsyncAppContext,
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
                            .unwrap_or(lsp2::DocumentHighlightKind::READ),
                    }
                })
                .collect()
        })
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetDocumentHighlights {
        proto::GetDocumentHighlights {
            project_id,
            buffer_id: buffer.remote_id(),
            position: Some(language2::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetDocumentHighlights,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
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
        _: &mut Project,
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
        _: Handle<Project>,
        buffer: Handle<Buffer>,
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

    fn buffer_id_from_proto(message: &proto::GetDocumentHighlights) -> u64 {
        message.buffer_id
    }
}

#[async_trait(?Send)]
impl LspCommand for GetHover {
    type Response = Option<Hover>;
    type LspRequest = lsp2::request::HoverRequest;
    type ProtoRequest = proto::GetHover;

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &AppContext,
    ) -> lsp2::HoverParams {
        lsp2::HoverParams {
            text_document_position_params: lsp2::TextDocumentPositionParams {
                text_document: lsp2::TextDocumentIdentifier {
                    uri: lsp2::Url::from_file_path(path).unwrap(),
                },
                position: point_to_lsp(self.position),
            },
            work_done_progress_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<lsp2::Hover>,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
        _: LanguageServerId,
        mut cx: AsyncAppContext,
    ) -> Result<Self::Response> {
        let Some(hover) = message else {
            return Ok(None);
        };

        let (language, range) = buffer.update(&mut cx, |buffer, cx| {
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

        fn hover_blocks_from_marked_string(
            marked_string: lsp2::MarkedString,
        ) -> Option<HoverBlock> {
            let block = match marked_string {
                lsp2::MarkedString::String(content) => HoverBlock {
                    text: content,
                    kind: HoverBlockKind::Markdown,
                },
                lsp2::MarkedString::LanguageString(lsp2::LanguageString { language, value }) => {
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
            lsp2::HoverContents::Scalar(marked_string) => {
                hover_blocks_from_marked_string(marked_string)
                    .into_iter()
                    .collect()
            }
            lsp2::HoverContents::Array(marked_strings) => marked_strings
                .into_iter()
                .filter_map(hover_blocks_from_marked_string)
                .collect(),
            lsp2::HoverContents::Markup(markup_content) => vec![HoverBlock {
                text: markup_content.value,
                kind: if markup_content.kind == lsp2::MarkupKind::Markdown {
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
            buffer_id: buffer.remote_id(),
            position: Some(language2::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version),
        }
    }

    async fn from_proto(
        message: Self::ProtoRequest,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
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
        _: &mut Project,
        _: PeerId,
        _: &clock::Global,
        _: &mut AppContext,
    ) -> proto::GetHoverResponse {
        if let Some(response) = response {
            let (start, end) = if let Some(range) = response.range {
                (
                    Some(language2::proto::serialize_anchor(&range.start)),
                    Some(language2::proto::serialize_anchor(&range.end)),
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
        _: Handle<Project>,
        buffer: Handle<Buffer>,
        cx: AsyncAppContext,
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
            language2::proto::deserialize_anchor(start)
                .and_then(|start| language2::proto::deserialize_anchor(end).map(|end| start..end))
        } else {
            None
        };

        Ok(Some(Hover {
            contents,
            range,
            language,
        }))
    }

    fn buffer_id_from_proto(message: &Self::ProtoRequest) -> u64 {
        message.buffer_id
    }
}

#[async_trait(?Send)]
impl LspCommand for GetCompletions {
    type Response = Vec<Completion>;
    type LspRequest = lsp2::request::Completion;
    type ProtoRequest = proto::GetCompletions;

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &AppContext,
    ) -> lsp2::CompletionParams {
        lsp2::CompletionParams {
            text_document_position: lsp2::TextDocumentPositionParams::new(
                lsp2::TextDocumentIdentifier::new(lsp2::Url::from_file_path(path).unwrap()),
                point_to_lsp(self.position),
            ),
            context: Default::default(),
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        completions: Option<lsp2::CompletionResponse>,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
        server_id: LanguageServerId,
        cx: AsyncAppContext,
    ) -> Result<Vec<Completion>> {
        let mut response_list = None;
        let completions = if let Some(completions) = completions {
            match completions {
                lsp2::CompletionResponse::Array(completions) => completions,

                lsp2::CompletionResponse::List(mut list) => {
                    let items = std::mem::take(&mut list.items);
                    response_list = Some(list);
                    items
                }
            }
        } else {
            Default::default()
        };

        let completions = buffer.update(&mut cx, |buffer, _| {
            let language = buffer.language().cloned();
            let snapshot = buffer.snapshot();
            let clipped_position = buffer.clip_point_utf16(Unclipped(self.position), Bias::Left);

            let mut range_for_token = None;
            completions
                .into_iter()
                .filter_map(move |mut lsp_completion| {
                    let (old_range, mut new_text) = match lsp_completion.text_edit.as_ref() {
                        // If the language server provides a range to overwrite, then
                        // check that the range is valid.
                        Some(lsp2::CompletionTextEdit::Edit(edit)) => {
                            let range = range_from_lsp(edit.range);
                            let start = snapshot.clip_point_utf16(range.start, Bias::Left);
                            let end = snapshot.clip_point_utf16(range.end, Bias::Left);
                            if start != range.start.0 || end != range.end.0 {
                                log::info!("completion out of expected range");
                                return None;
                            }
                            (
                                snapshot.anchor_before(start)..snapshot.anchor_after(end),
                                edit.new_text.clone(),
                            )
                        }

                        // If the language server does not provide a range, then infer
                        // the range based on the syntax tree.
                        None => {
                            if self.position != clipped_position {
                                log::info!("completion out of expected range");
                                return None;
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
                                let range = range_from_lsp(range.clone());
                                let start = snapshot.clip_point_utf16(range.start, Bias::Left);
                                let end = snapshot.clip_point_utf16(range.end, Bias::Left);
                                if start != range.start.0 || end != range.end.0 {
                                    log::info!("completion out of expected range");
                                    return None;
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

                        Some(lsp2::CompletionTextEdit::InsertAndReplace(_)) => {
                            log::info!("unsupported insert/replace completion");
                            return None;
                        }
                    };

                    let language = language.clone();
                    LineEnding::normalize(&mut new_text);
                    Some(async move {
                        let mut label = None;
                        if let Some(language) = language {
                            language.process_completion(&mut lsp_completion).await;
                            label = language.label_for_completion(&lsp_completion).await;
                        }
                        Completion {
                            old_range,
                            new_text,
                            label: label.unwrap_or_else(|| {
                                language2::CodeLabel::plain(
                                    lsp_completion.label.clone(),
                                    lsp_completion.filter_text.as_deref(),
                                )
                            }),
                            server_id,
                            lsp_completion,
                        }
                    })
                })
        })?;

        Ok(future::join_all(completions).await)
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetCompletions {
        let anchor = buffer.anchor_after(self.position);
        proto::GetCompletions {
            project_id,
            buffer_id: buffer.remote_id(),
            position: Some(language2::proto::serialize_anchor(&anchor)),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetCompletions,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Self> {
        let version = deserialize_version(&message.version);
        buffer
            .update(&mut cx, |buffer, _| buffer.wait_for_version(version))?
            .await?;
        let position = message
            .position
            .and_then(language2::proto::deserialize_anchor)
            .map(|p| {
                buffer.update(&mut cx, |buffer, _| {
                    buffer.clip_point_utf16(Unclipped(p.to_point_utf16(buffer)), Bias::Left)
                })
            })
            .ok_or_else(|| anyhow!("invalid position"))??;
        Ok(Self { position })
    }

    fn response_to_proto(
        completions: Vec<Completion>,
        _: &mut Project,
        _: PeerId,
        buffer_version: &clock::Global,
        _: &mut AppContext,
    ) -> proto::GetCompletionsResponse {
        proto::GetCompletionsResponse {
            completions: completions
                .iter()
                .map(language2::proto::serialize_completion)
                .collect(),
            version: serialize_version(&buffer_version),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::GetCompletionsResponse,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Vec<Completion>> {
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;

        let language = buffer.update(&mut cx, |buffer, _| buffer.language().cloned())?;
        let completions = message.completions.into_iter().map(|completion| {
            language2::proto::deserialize_completion(completion, language.clone())
        });
        future::try_join_all(completions).await
    }

    fn buffer_id_from_proto(message: &proto::GetCompletions) -> u64 {
        message.buffer_id
    }
}

#[async_trait(?Send)]
impl LspCommand for GetCodeActions {
    type Response = Vec<CodeAction>;
    type LspRequest = lsp2::request::CodeActionRequest;
    type ProtoRequest = proto::GetCodeActions;

    fn check_capabilities(&self, capabilities: &ServerCapabilities) -> bool {
        match &capabilities.code_action_provider {
            None => false,
            Some(lsp2::CodeActionProviderCapability::Simple(false)) => false,
            _ => true,
        }
    }

    fn to_lsp(
        &self,
        path: &Path,
        buffer: &Buffer,
        language_server: &Arc<LanguageServer>,
        _: &AppContext,
    ) -> lsp2::CodeActionParams {
        let relevant_diagnostics = buffer
            .snapshot()
            .diagnostics_in_range::<_, usize>(self.range.clone(), false)
            .map(|entry| entry.to_lsp_diagnostic_stub())
            .collect();
        lsp2::CodeActionParams {
            text_document: lsp2::TextDocumentIdentifier::new(
                lsp2::Url::from_file_path(path).unwrap(),
            ),
            range: range_to_lsp(self.range.to_point_utf16(buffer)),
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: lsp2::CodeActionContext {
                diagnostics: relevant_diagnostics,
                only: language_server.code_action_kinds(),
                ..lsp2::CodeActionContext::default()
            },
        }
    }

    async fn response_from_lsp(
        self,
        actions: Option<lsp2::CodeActionResponse>,
        _: Handle<Project>,
        _: Handle<Buffer>,
        server_id: LanguageServerId,
        _: AsyncAppContext,
    ) -> Result<Vec<CodeAction>> {
        Ok(actions
            .unwrap_or_default()
            .into_iter()
            .filter_map(|entry| {
                if let lsp2::CodeActionOrCommand::CodeAction(lsp_action) = entry {
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
            buffer_id: buffer.remote_id(),
            start: Some(language2::proto::serialize_anchor(&self.range.start)),
            end: Some(language2::proto::serialize_anchor(&self.range.end)),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetCodeActions,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Self> {
        let start = message
            .start
            .and_then(language2::proto::deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid start"))?;
        let end = message
            .end
            .and_then(language2::proto::deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid end"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })?
            .await?;

        Ok(Self { range: start..end })
    }

    fn response_to_proto(
        code_actions: Vec<CodeAction>,
        _: &mut Project,
        _: PeerId,
        buffer_version: &clock::Global,
        _: &mut AppContext,
    ) -> proto::GetCodeActionsResponse {
        proto::GetCodeActionsResponse {
            actions: code_actions
                .iter()
                .map(language2::proto::serialize_code_action)
                .collect(),
            version: serialize_version(&buffer_version),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::GetCodeActionsResponse,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
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
            .map(language2::proto::deserialize_code_action)
            .collect()
    }

    fn buffer_id_from_proto(message: &proto::GetCodeActions) -> u64 {
        message.buffer_id
    }
}

#[async_trait(?Send)]
impl LspCommand for OnTypeFormatting {
    type Response = Option<Transaction>;
    type LspRequest = lsp2::request::OnTypeFormatting;
    type ProtoRequest = proto::OnTypeFormatting;

    fn check_capabilities(&self, server_capabilities: &lsp2::ServerCapabilities) -> bool {
        let Some(on_type_formatting_options) =
            &server_capabilities.document_on_type_formatting_provider
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
    ) -> lsp2::DocumentOnTypeFormattingParams {
        lsp2::DocumentOnTypeFormattingParams {
            text_document_position: lsp2::TextDocumentPositionParams::new(
                lsp2::TextDocumentIdentifier::new(lsp2::Url::from_file_path(path).unwrap()),
                point_to_lsp(self.position),
            ),
            ch: self.trigger.clone(),
            options: lsp_formatting_options(self.options.tab_size),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<Vec<lsp2::TextEdit>>,
        project: Handle<Project>,
        buffer: Handle<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncAppContext,
    ) -> Result<Option<Transaction>> {
        if let Some(edits) = message {
            let (lsp_adapter, lsp_server) =
                language_server_for_buffer(&project, &buffer, server_id, &mut cx)?;
            Project::deserialize_edits(
                project,
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
            buffer_id: buffer.remote_id(),
            position: Some(language2::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            trigger: self.trigger.clone(),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::OnTypeFormatting,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
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

        let tab_size = buffer.update(&mut cx, |buffer, cx| {
            language_settings(buffer.language(), buffer.file(), cx).tab_size
        })?;

        Ok(Self {
            position: buffer.update(&mut cx, |buffer, _| position.to_point_utf16(buffer))?,
            trigger: message.trigger.clone(),
            options: lsp_formatting_options(tab_size.get()).into(),
            push_to_history: false,
        })
    }

    fn response_to_proto(
        response: Option<Transaction>,
        _: &mut Project,
        _: PeerId,
        _: &clock::Global,
        _: &mut AppContext,
    ) -> proto::OnTypeFormattingResponse {
        proto::OnTypeFormattingResponse {
            transaction: response
                .map(|transaction| language2::proto::serialize_transaction(&transaction)),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::OnTypeFormattingResponse,
        _: Handle<Project>,
        _: Handle<Buffer>,
        _: AsyncAppContext,
    ) -> Result<Option<Transaction>> {
        let Some(transaction) = message.transaction else {
            return Ok(None);
        };
        Ok(Some(language2::proto::deserialize_transaction(
            transaction,
        )?))
    }

    fn buffer_id_from_proto(message: &proto::OnTypeFormatting) -> u64 {
        message.buffer_id
    }
}

impl InlayHints {
    pub async fn lsp_to_project_hint(
        lsp_hint: lsp2::InlayHint,
        buffer_handle: &Handle<Buffer>,
        server_id: LanguageServerId,
        resolve_state: ResolveState,
        force_no_type_left_padding: bool,
        cx: &mut AsyncAppContext,
    ) -> anyhow::Result<InlayHint> {
        let kind = lsp_hint.kind.and_then(|kind| match kind {
            lsp2::InlayHintKind::TYPE => Some(InlayHintKind::Type),
            lsp2::InlayHintKind::PARAMETER => Some(InlayHintKind::Parameter),
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
                lsp2::InlayHintTooltip::String(s) => InlayHintTooltip::String(s),
                lsp2::InlayHintTooltip::MarkupContent(markup_content) => {
                    InlayHintTooltip::MarkupContent(MarkupContent {
                        kind: match markup_content.kind {
                            lsp2::MarkupKind::PlainText => HoverBlockKind::PlainText,
                            lsp2::MarkupKind::Markdown => HoverBlockKind::Markdown,
                        },
                        value: markup_content.value,
                    })
                }
            }),
            resolve_state,
        })
    }

    async fn lsp_inlay_label_to_project(
        lsp_label: lsp2::InlayHintLabel,
        server_id: LanguageServerId,
    ) -> anyhow::Result<InlayHintLabel> {
        let label = match lsp_label {
            lsp2::InlayHintLabel::String(s) => InlayHintLabel::String(s),
            lsp2::InlayHintLabel::LabelParts(lsp_parts) => {
                let mut parts = Vec::with_capacity(lsp_parts.len());
                for lsp_part in lsp_parts {
                    parts.push(InlayHintLabelPart {
                        value: lsp_part.value,
                        tooltip: lsp_part.tooltip.map(|tooltip| match tooltip {
                            lsp2::InlayHintLabelPartTooltip::String(s) => {
                                InlayHintLabelPartTooltip::String(s)
                            }
                            lsp2::InlayHintLabelPartTooltip::MarkupContent(markup_content) => {
                                InlayHintLabelPartTooltip::MarkupContent(MarkupContent {
                                    kind: match markup_content.kind {
                                        lsp2::MarkupKind::PlainText => HoverBlockKind::PlainText,
                                        lsp2::MarkupKind::Markdown => HoverBlockKind::Markdown,
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
            position: Some(language2::proto::serialize_anchor(&response_hint.position)),
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
                serde_json::from_str::<Option<lsp2::LSPAny>>(&lsp_resolve_state.value)
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
                .and_then(language2::proto::deserialize_anchor)
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
                                        lsp2::Location {
                                            uri: lsp2::Url::parse(&uri)
                                                .context("invalid uri in hint part {part:?}")?,
                                            range: lsp2::Range::new(
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

    pub fn project_to_lsp_hint(hint: InlayHint, snapshot: &BufferSnapshot) -> lsp2::InlayHint {
        lsp2::InlayHint {
            position: point_to_lsp(hint.position.to_point_utf16(snapshot)),
            kind: hint.kind.map(|kind| match kind {
                InlayHintKind::Type => lsp2::InlayHintKind::TYPE,
                InlayHintKind::Parameter => lsp2::InlayHintKind::PARAMETER,
            }),
            text_edits: None,
            tooltip: hint.tooltip.and_then(|tooltip| {
                Some(match tooltip {
                    InlayHintTooltip::String(s) => lsp2::InlayHintTooltip::String(s),
                    InlayHintTooltip::MarkupContent(markup_content) => {
                        lsp2::InlayHintTooltip::MarkupContent(lsp2::MarkupContent {
                            kind: match markup_content.kind {
                                HoverBlockKind::PlainText => lsp2::MarkupKind::PlainText,
                                HoverBlockKind::Markdown => lsp2::MarkupKind::Markdown,
                                HoverBlockKind::Code { .. } => return None,
                            },
                            value: markup_content.value,
                        })
                    }
                })
            }),
            label: match hint.label {
                InlayHintLabel::String(s) => lsp2::InlayHintLabel::String(s),
                InlayHintLabel::LabelParts(label_parts) => lsp2::InlayHintLabel::LabelParts(
                    label_parts
                        .into_iter()
                        .map(|part| lsp2::InlayHintLabelPart {
                            value: part.value,
                            tooltip: part.tooltip.and_then(|tooltip| {
                                Some(match tooltip {
                                    InlayHintLabelPartTooltip::String(s) => {
                                        lsp2::InlayHintLabelPartTooltip::String(s)
                                    }
                                    InlayHintLabelPartTooltip::MarkupContent(markup_content) => {
                                        lsp2::InlayHintLabelPartTooltip::MarkupContent(
                                            lsp2::MarkupContent {
                                                kind: match markup_content.kind {
                                                    HoverBlockKind::PlainText => {
                                                        lsp2::MarkupKind::PlainText
                                                    }
                                                    HoverBlockKind::Markdown => {
                                                        lsp2::MarkupKind::Markdown
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
                    lsp2::InlayHintServerCapabilities::Options(o) => o.resolve_provider,
                    lsp2::InlayHintServerCapabilities::RegistrationOptions(o) => {
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
    type LspRequest = lsp2::InlayHintRequest;
    type ProtoRequest = proto::InlayHints;

    fn check_capabilities(&self, server_capabilities: &lsp2::ServerCapabilities) -> bool {
        let Some(inlay_hint_provider) = &server_capabilities.inlay_hint_provider else {
            return false;
        };
        match inlay_hint_provider {
            lsp2::OneOf::Left(enabled) => *enabled,
            lsp2::OneOf::Right(inlay_hint_capabilities) => match inlay_hint_capabilities {
                lsp2::InlayHintServerCapabilities::Options(_) => true,
                lsp2::InlayHintServerCapabilities::RegistrationOptions(_) => false,
            },
        }
    }

    fn to_lsp(
        &self,
        path: &Path,
        buffer: &Buffer,
        _: &Arc<LanguageServer>,
        _: &AppContext,
    ) -> lsp2::InlayHintParams {
        lsp2::InlayHintParams {
            text_document: lsp2::TextDocumentIdentifier {
                uri: lsp2::Url::from_file_path(path).unwrap(),
            },
            range: range_to_lsp(self.range.to_point_utf16(buffer)),
            work_done_progress_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<Vec<lsp2::InlayHint>>,
        project: Handle<Project>,
        buffer: Handle<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncAppContext,
    ) -> anyhow::Result<Vec<InlayHint>> {
        let (lsp_adapter, lsp_server) =
            language_server_for_buffer(&project, &buffer, server_id, &mut cx)?;
        // `typescript-language-server` adds padding to the left for type hints, turning
        // `const foo: boolean` into `const foo : boolean` which looks odd.
        // `rust-analyzer` does not have the padding for this case, and we have to accomodate both.
        //
        // We could trim the whole string, but being pessimistic on par with the situation above,
        // there might be a hint with multiple whitespaces at the end(s) which we need to display properly.
        // Hence let's use a heuristic first to handle the most awkward case and look for more.
        let force_no_type_left_padding =
            lsp_adapter.name.0.as_ref() == "typescript-language-server";

        let hints = message.unwrap_or_default().into_iter().map(|lsp_hint| {
            let resolve_state = if InlayHints::can_resolve_inlays(lsp_server.capabilities()) {
                ResolveState::CanResolve(lsp_server.server_id(), lsp_hint.data.clone())
            } else {
                ResolveState::Resolved
            };

            let buffer = buffer.clone();
            cx.spawn(|mut cx| async move {
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
            buffer_id: buffer.remote_id(),
            start: Some(language2::proto::serialize_anchor(&self.range.start)),
            end: Some(language2::proto::serialize_anchor(&self.range.end)),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::InlayHints,
        _: Handle<Project>,
        buffer: Handle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Self> {
        let start = message
            .start
            .and_then(language2::proto::deserialize_anchor)
            .context("invalid start")?;
        let end = message
            .end
            .and_then(language2::proto::deserialize_anchor)
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
        _: &mut Project,
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
        _: Handle<Project>,
        buffer: Handle<Buffer>,
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

    fn buffer_id_from_proto(message: &proto::InlayHints) -> u64 {
        message.buffer_id
    }
}
