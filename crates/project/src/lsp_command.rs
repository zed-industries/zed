use crate::{
    DocumentHighlight, Hover, HoverBlock, Location, LocationLink, Project, ProjectTransaction,
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use client::proto::{self, PeerId};
use fs::LineEnding;
use gpui::{AppContext, AsyncAppContext, ModelHandle};
use language::{
    point_from_lsp, point_to_lsp,
    proto::{deserialize_anchor, deserialize_version, serialize_anchor, serialize_version},
    range_from_lsp, range_to_lsp, Anchor, Bias, Buffer, CachedLspAdapter, CharKind, CodeAction,
    Completion, OffsetRangeExt, PointUtf16, ToOffset, ToPointUtf16, Unclipped,
};
use lsp::{DocumentHighlightKind, LanguageServer, ServerCapabilities};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use std::{cmp::Reverse, ops::Range, path::Path, sync::Arc};

#[async_trait(?Send)]
pub(crate) trait LspCommand: 'static + Sized {
    type Response: 'static + Default + Send;
    type LspRequest: 'static + Send + lsp::request::Request;
    type ProtoRequest: 'static + Send + proto::RequestMessage;

    fn check_capabilities(&self, _: &lsp::ServerCapabilities) -> bool {
        true
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
        project: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        cx: AsyncAppContext,
    ) -> Result<Self::Response>;

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> Self::ProtoRequest;
    async fn from_proto(
        message: Self::ProtoRequest,
        project: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
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
        project: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
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

#[async_trait(?Send)]
impl LspCommand for PrepareRename {
    type Response = Option<Range<Anchor>>;
    type LspRequest = lsp::request::PrepareRenameRequest;
    type ProtoRequest = proto::PrepareRename;

    fn check_capabilities(&self, capabilities: &ServerCapabilities) -> bool {
        if let Some(lsp::OneOf::Right(rename)) = &capabilities.rename_provider {
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
        _: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        cx: AsyncAppContext,
    ) -> Result<Option<Range<Anchor>>> {
        buffer.read_with(&cx, |buffer, _| {
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
        })
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::PrepareRename {
        proto::PrepareRename {
            project_id,
            buffer_id: buffer.remote_id(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::PrepareRename,
        _: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })
            .await?;

        Ok(Self {
            position: buffer.read_with(&cx, |buffer, _| position.to_point_utf16(buffer)),
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
        _: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Option<Range<Anchor>>> {
        if message.can_rename {
            buffer
                .update(&mut cx, |buffer, _| {
                    buffer.wait_for_version(deserialize_version(&message.version))
                })
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
        project: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<ProjectTransaction> {
        if let Some(edit) = message {
            let (lsp_adapter, lsp_server) = language_server_for_buffer(&project, &buffer, &mut cx)?;
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
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            new_name: self.new_name.clone(),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::PerformRename,
        _: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })
            .await?;
        Ok(Self {
            position: buffer.read_with(&cx, |buffer, _| position.to_point_utf16(buffer)),
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
        project: ModelHandle<Project>,
        _: ModelHandle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<ProjectTransaction> {
        let message = message
            .transaction
            .ok_or_else(|| anyhow!("missing transaction"))?;
        project
            .update(&mut cx, |project, cx| {
                project.deserialize_project_transaction(message, self.push_to_history, cx)
            })
            .await
    }

    fn buffer_id_from_proto(message: &proto::PerformRename) -> u64 {
        message.buffer_id
    }
}

#[async_trait(?Send)]
impl LspCommand for GetDefinition {
    type Response = Vec<LocationLink>;
    type LspRequest = lsp::request::GotoDefinition;
    type ProtoRequest = proto::GetDefinition;

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
        project: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        cx: AsyncAppContext,
    ) -> Result<Vec<LocationLink>> {
        location_links_from_lsp(message, project, buffer, cx).await
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetDefinition {
        proto::GetDefinition {
            project_id,
            buffer_id: buffer.remote_id(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetDefinition,
        _: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })
            .await?;
        Ok(Self {
            position: buffer.read_with(&cx, |buffer, _| position.to_point_utf16(buffer)),
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
        project: ModelHandle<Project>,
        _: ModelHandle<Buffer>,
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
    type LspRequest = lsp::request::GotoTypeDefinition;
    type ProtoRequest = proto::GetTypeDefinition;

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
        project: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        cx: AsyncAppContext,
    ) -> Result<Vec<LocationLink>> {
        location_links_from_lsp(message, project, buffer, cx).await
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetTypeDefinition {
        proto::GetTypeDefinition {
            project_id,
            buffer_id: buffer.remote_id(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetTypeDefinition,
        _: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })
            .await?;
        Ok(Self {
            position: buffer.read_with(&cx, |buffer, _| position.to_point_utf16(buffer)),
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
        project: ModelHandle<Project>,
        _: ModelHandle<Buffer>,
        cx: AsyncAppContext,
    ) -> Result<Vec<LocationLink>> {
        location_links_from_proto(message.links, project, cx).await
    }

    fn buffer_id_from_proto(message: &proto::GetTypeDefinition) -> u64 {
        message.buffer_id
    }
}

fn language_server_for_buffer(
    project: &ModelHandle<Project>,
    buffer: &ModelHandle<Buffer>,
    cx: &mut AsyncAppContext,
) -> Result<(Arc<CachedLspAdapter>, Arc<LanguageServer>)> {
    project
        .read_with(cx, |project, cx| {
            project
                .language_server_for_buffer(buffer.read(cx), cx)
                .map(|(adapter, server)| (adapter.clone(), server.clone()))
        })
        .ok_or_else(|| anyhow!("no language server found for buffer"))
}

async fn location_links_from_proto(
    proto_links: Vec<proto::LocationLink>,
    project: ModelHandle<Project>,
    mut cx: AsyncAppContext,
) -> Result<Vec<LocationLink>> {
    let mut links = Vec::new();

    for link in proto_links {
        let origin = match link.origin {
            Some(origin) => {
                let buffer = project
                    .update(&mut cx, |this, cx| {
                        this.wait_for_remote_buffer(origin.buffer_id, cx)
                    })
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
                    .update(&mut cx, |buffer, _| buffer.wait_for_anchors([&start, &end]))
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
            })
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
            .update(&mut cx, |buffer, _| buffer.wait_for_anchors([&start, &end]))
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
    project: ModelHandle<Project>,
    buffer: ModelHandle<Buffer>,
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

    let (lsp_adapter, language_server) = language_server_for_buffer(&project, &buffer, &mut cx)?;
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
            })
            .await?;

        cx.read(|cx| {
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
        });
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
    type LspRequest = lsp::request::References;
    type ProtoRequest = proto::GetReferences;

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
        project: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Vec<Location>> {
        let mut references = Vec::new();
        let (lsp_adapter, language_server) =
            language_server_for_buffer(&project, &buffer, &mut cx)?;

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
                    })
                    .await?;

                cx.read(|cx| {
                    let target_buffer = target_buffer_handle.read(cx);
                    let target_start = target_buffer
                        .clip_point_utf16(point_from_lsp(lsp_location.range.start), Bias::Left);
                    let target_end = target_buffer
                        .clip_point_utf16(point_from_lsp(lsp_location.range.end), Bias::Left);
                    references.push(Location {
                        buffer: target_buffer_handle,
                        range: target_buffer.anchor_after(target_start)
                            ..target_buffer.anchor_before(target_end),
                    });
                });
            }
        }

        Ok(references)
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetReferences {
        proto::GetReferences {
            project_id,
            buffer_id: buffer.remote_id(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetReferences,
        _: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })
            .await?;
        Ok(Self {
            position: buffer.read_with(&cx, |buffer, _| position.to_point_utf16(buffer)),
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
        project: ModelHandle<Project>,
        _: ModelHandle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Vec<Location>> {
        let mut locations = Vec::new();
        for location in message.locations {
            let target_buffer = project
                .update(&mut cx, |this, cx| {
                    this.wait_for_remote_buffer(location.buffer_id, cx)
                })
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
                .update(&mut cx, |buffer, _| buffer.wait_for_anchors([&start, &end]))
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
    type LspRequest = lsp::request::DocumentHighlightRequest;
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
        _: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        cx: AsyncAppContext,
    ) -> Result<Vec<DocumentHighlight>> {
        buffer.read_with(&cx, |buffer, _| {
            let mut lsp_highlights = lsp_highlights.unwrap_or_default();
            lsp_highlights.sort_unstable_by_key(|h| (h.range.start, Reverse(h.range.end)));
            Ok(lsp_highlights
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
                .collect())
        })
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetDocumentHighlights {
        proto::GetDocumentHighlights {
            project_id,
            buffer_id: buffer.remote_id(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetDocumentHighlights,
        _: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })
            .await?;
        Ok(Self {
            position: buffer.read_with(&cx, |buffer, _| position.to_point_utf16(buffer)),
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
        _: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
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
                .update(&mut cx, |buffer, _| buffer.wait_for_anchors([&start, &end]))
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
    type LspRequest = lsp::request::HoverRequest;
    type ProtoRequest = proto::GetHover;

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
        _: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        cx: AsyncAppContext,
    ) -> Result<Self::Response> {
        Ok(message.and_then(|hover| {
            let range = hover.range.map(|range| {
                cx.read(|cx| {
                    let buffer = buffer.read(cx);
                    let token_start =
                        buffer.clip_point_utf16(point_from_lsp(range.start), Bias::Left);
                    let token_end = buffer.clip_point_utf16(point_from_lsp(range.end), Bias::Left);
                    buffer.anchor_after(token_start)..buffer.anchor_before(token_end)
                })
            });

            let contents = cx.read(|_| match hover.contents {
                lsp::HoverContents::Scalar(marked_string) => {
                    HoverBlock::try_new(marked_string).map(|contents| vec![contents])
                }
                lsp::HoverContents::Array(marked_strings) => {
                    let content: Vec<HoverBlock> = marked_strings
                        .into_iter()
                        .filter_map(HoverBlock::try_new)
                        .collect();
                    if content.is_empty() {
                        None
                    } else {
                        Some(content)
                    }
                }
                lsp::HoverContents::Markup(markup_content) => {
                    let mut contents = Vec::new();
                    let mut language = None;
                    let mut current_text = String::new();
                    for event in Parser::new_ext(&markup_content.value, Options::all()) {
                        match event {
                            Event::SoftBreak => {
                                current_text.push(' ');
                            }
                            Event::Text(text) | Event::Code(text) => {
                                current_text.push_str(&text.to_string());
                            }
                            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(new_language))) => {
                                if !current_text.is_empty() {
                                    let text = std::mem::take(&mut current_text).trim().to_string();
                                    contents.push(HoverBlock { text, language });
                                }

                                language = if new_language.is_empty() {
                                    None
                                } else {
                                    Some(new_language.to_string())
                                };
                            }
                            Event::End(Tag::CodeBlock(_))
                            | Event::End(Tag::Paragraph)
                            | Event::End(Tag::Heading(_, _, _))
                            | Event::End(Tag::BlockQuote)
                            | Event::HardBreak => {
                                if !current_text.is_empty() {
                                    let text = std::mem::take(&mut current_text).trim().to_string();
                                    contents.push(HoverBlock { text, language });
                                }
                                language = None;
                            }
                            _ => {}
                        }
                    }

                    if !current_text.trim().is_empty() {
                        contents.push(HoverBlock {
                            text: current_text,
                            language,
                        });
                    }

                    if contents.is_empty() {
                        None
                    } else {
                        Some(contents)
                    }
                }
            });

            contents.map(|contents| Hover { contents, range })
        }))
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> Self::ProtoRequest {
        proto::GetHover {
            project_id,
            buffer_id: buffer.remote_id(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            version: serialize_version(&buffer.version),
        }
    }

    async fn from_proto(
        message: Self::ProtoRequest,
        _: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })
            .await?;
        Ok(Self {
            position: buffer.read_with(&cx, |buffer, _| position.to_point_utf16(buffer)),
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
                    language: block.language,
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
        _: ModelHandle<Project>,
        _: ModelHandle<Buffer>,
        _: AsyncAppContext,
    ) -> Result<Self::Response> {
        let range = if let (Some(start), Some(end)) = (message.start, message.end) {
            language::proto::deserialize_anchor(start)
                .and_then(|start| language::proto::deserialize_anchor(end).map(|end| start..end))
        } else {
            None
        };

        let contents: Vec<_> = message
            .contents
            .into_iter()
            .map(|block| HoverBlock {
                text: block.text,
                language: block.language,
            })
            .collect();

        Ok(if contents.is_empty() {
            None
        } else {
            Some(Hover { contents, range })
        })
    }

    fn buffer_id_from_proto(message: &Self::ProtoRequest) -> u64 {
        message.buffer_id
    }
}

#[async_trait(?Send)]
impl LspCommand for GetCompletions {
    type Response = Vec<Completion>;
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
            context: Default::default(),
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        }
    }

    async fn response_from_lsp(
        self,
        completions: Option<lsp::CompletionResponse>,
        _: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        cx: AsyncAppContext,
    ) -> Result<Vec<Completion>> {
        let completions = if let Some(completions) = completions {
            match completions {
                lsp::CompletionResponse::Array(completions) => completions,
                lsp::CompletionResponse::List(list) => list.items,
            }
        } else {
            Default::default()
        };

        let completions = buffer.read_with(&cx, |buffer, _| {
            let language = buffer.language().cloned();
            let snapshot = buffer.snapshot();
            let clipped_position = buffer.clip_point_utf16(Unclipped(self.position), Bias::Left);
            let mut range_for_token = None;
            completions
                .into_iter()
                .filter_map(move |mut lsp_completion| {
                    // For now, we can only handle additional edits if they are returned
                    // when resolving the completion, not if they are present initially.
                    if lsp_completion
                        .additional_text_edits
                        .as_ref()
                        .map_or(false, |edits| !edits.is_empty())
                    {
                        return None;
                    }

                    let (old_range, mut new_text) = match lsp_completion.text_edit.as_ref() {
                        // If the language server provides a range to overwrite, then
                        // check that the range is valid.
                        Some(lsp::CompletionTextEdit::Edit(edit)) => {
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
                            let Range { start, end } = range_for_token
                                .get_or_insert_with(|| {
                                    let offset = self.position.to_offset(&snapshot);
                                    let (range, kind) = snapshot.surrounding_word(offset);
                                    if kind == Some(CharKind::Word) {
                                        range
                                    } else {
                                        offset..offset
                                    }
                                })
                                .clone();
                            let text = lsp_completion
                                .insert_text
                                .as_ref()
                                .unwrap_or(&lsp_completion.label)
                                .clone();
                            (
                                snapshot.anchor_before(start)..snapshot.anchor_after(end),
                                text,
                            )
                        }
                        Some(lsp::CompletionTextEdit::InsertAndReplace(_)) => {
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
                                language::CodeLabel::plain(
                                    lsp_completion.label.clone(),
                                    lsp_completion.filter_text.as_deref(),
                                )
                            }),
                            lsp_completion,
                        }
                    })
                })
        });

        Ok(futures::future::join_all(completions).await)
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::GetCompletions {
        let anchor = buffer.anchor_after(self.position);
        proto::GetCompletions {
            project_id,
            buffer_id: buffer.remote_id(),
            position: Some(language::proto::serialize_anchor(&anchor)),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetCompletions,
        _: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Self> {
        let version = deserialize_version(&message.version);
        buffer
            .update(&mut cx, |buffer, _| buffer.wait_for_version(version))
            .await?;
        let position = message
            .position
            .and_then(language::proto::deserialize_anchor)
            .map(|p| {
                buffer.read_with(&cx, |buffer, _| {
                    buffer.clip_point_utf16(Unclipped(p.to_point_utf16(buffer)), Bias::Left)
                })
            })
            .ok_or_else(|| anyhow!("invalid position"))?;
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
                .map(language::proto::serialize_completion)
                .collect(),
            version: serialize_version(&buffer_version),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::GetCompletionsResponse,
        _: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Vec<Completion>> {
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })
            .await?;

        let language = buffer.read_with(&cx, |buffer, _| buffer.language().cloned());
        let completions = message.completions.into_iter().map(|completion| {
            language::proto::deserialize_completion(completion, language.clone())
        });
        futures::future::try_join_all(completions).await
    }

    fn buffer_id_from_proto(message: &proto::GetCompletions) -> u64 {
        message.buffer_id
    }
}

#[async_trait(?Send)]
impl LspCommand for GetCodeActions {
    type Response = Vec<CodeAction>;
    type LspRequest = lsp::request::CodeActionRequest;
    type ProtoRequest = proto::GetCodeActions;

    fn check_capabilities(&self, capabilities: &ServerCapabilities) -> bool {
        capabilities.code_action_provider.is_some()
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
            .diagnostics_in_range::<_, usize>(self.range.clone(), false)
            .map(|entry| entry.to_lsp_diagnostic_stub())
            .collect();
        lsp::CodeActionParams {
            text_document: lsp::TextDocumentIdentifier::new(
                lsp::Url::from_file_path(path).unwrap(),
            ),
            range: range_to_lsp(self.range.to_point_utf16(buffer)),
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: lsp::CodeActionContext {
                diagnostics: relevant_diagnostics,
                only: language_server.code_action_kinds(),
            },
        }
    }

    async fn response_from_lsp(
        self,
        actions: Option<lsp::CodeActionResponse>,
        _: ModelHandle<Project>,
        _: ModelHandle<Buffer>,
        _: AsyncAppContext,
    ) -> Result<Vec<CodeAction>> {
        Ok(actions
            .unwrap_or_default()
            .into_iter()
            .filter_map(|entry| {
                if let lsp::CodeActionOrCommand::CodeAction(lsp_action) = entry {
                    Some(CodeAction {
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
            start: Some(language::proto::serialize_anchor(&self.range.start)),
            end: Some(language::proto::serialize_anchor(&self.range.end)),
            version: serialize_version(&buffer.version()),
        }
    }

    async fn from_proto(
        message: proto::GetCodeActions,
        _: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
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
            })
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
                .map(language::proto::serialize_code_action)
                .collect(),
            version: serialize_version(&buffer_version),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::GetCodeActionsResponse,
        _: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Vec<CodeAction>> {
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&message.version))
            })
            .await?;
        message
            .actions
            .into_iter()
            .map(language::proto::deserialize_code_action)
            .collect()
    }

    fn buffer_id_from_proto(message: &proto::GetCodeActions) -> u64 {
        message.buffer_id
    }
}
