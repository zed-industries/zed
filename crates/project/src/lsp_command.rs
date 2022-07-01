use crate::{
    DocumentHighlight, Hover, HoverBlock, Location, LocationLink, Project, ProjectTransaction,
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use client::{proto, PeerId};
use gpui::{AppContext, AsyncAppContext, ModelHandle};
use language::{
    point_from_lsp, point_to_lsp,
    proto::{deserialize_anchor, deserialize_version, serialize_anchor, serialize_version},
    range_from_lsp, Anchor, Bias, Buffer, PointUtf16, ToPointUtf16,
};
use lsp::{DocumentHighlightKind, ServerCapabilities};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use std::{cmp::Reverse, ops::Range, path::Path};

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
        cx: &AppContext,
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

pub(crate) struct GetReferences {
    pub position: PointUtf16,
}

pub(crate) struct GetDocumentHighlights {
    pub position: PointUtf16,
}

pub(crate) struct GetHover {
    pub position: PointUtf16,
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

    fn to_lsp(&self, path: &Path, _: &AppContext) -> lsp::TextDocumentPositionParams {
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
                if buffer.clip_point_utf16(start, Bias::Left) == start
                    && buffer.clip_point_utf16(end, Bias::Left) == end
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
                buffer.wait_for_version(deserialize_version(message.version))
            })
            .await;

        Ok(Self {
            position: buffer.read_with(&cx, |buffer, _| position.to_point_utf16(buffer)),
        })
    }

    fn response_to_proto(
        range: Option<Range<Anchor>>,
        _: &mut Project,
        _: PeerId,
        buffer_version: &clock::Global,
        _: &AppContext,
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
                    buffer.wait_for_version(deserialize_version(message.version))
                })
                .await;
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

    fn to_lsp(&self, path: &Path, _: &AppContext) -> lsp::RenameParams {
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
            let (lsp_adapter, lsp_server) = project
                .read_with(&cx, |project, cx| {
                    project
                        .language_server_for_buffer(buffer.read(cx), cx)
                        .map(|(adapter, server)| (adapter.clone(), server.clone()))
                })
                .ok_or_else(|| anyhow!("no language server found for buffer"))?;
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
                buffer.wait_for_version(deserialize_version(message.version))
            })
            .await;
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
        cx: &AppContext,
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

    fn to_lsp(&self, path: &Path, _: &AppContext) -> lsp::GotoDefinitionParams {
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
        mut cx: AsyncAppContext,
    ) -> Result<Vec<LocationLink>> {
        let mut definitions = Vec::new();
        let (lsp_adapter, language_server) = project
            .read_with(&cx, |project, cx| {
                project
                    .language_server_for_buffer(buffer.read(cx), cx)
                    .map(|(adapter, server)| (adapter.clone(), server.clone()))
            })
            .ok_or_else(|| anyhow!("no language server found for buffer"))?;

        if let Some(message) = message {
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

            for (origin_range, target_uri, target_range) in unresolved_links {
                let target_buffer_handle = project
                    .update(&mut cx, |this, cx| {
                        this.open_local_buffer_via_lsp(
                            target_uri,
                            language_server.server_id(),
                            lsp_adapter.name(),
                            cx,
                        )
                    })
                    .await?;

                cx.read(|cx| {
                    let origin_location = origin_range.map(|origin_range| {
                        let origin_buffer = buffer.read(cx);
                        let origin_start = origin_buffer
                            .clip_point_utf16(point_from_lsp(origin_range.start), Bias::Left);
                        let origin_end = origin_buffer
                            .clip_point_utf16(point_from_lsp(origin_range.end), Bias::Left);
                        Location {
                            buffer: buffer.clone(),
                            range: origin_buffer.anchor_after(origin_start)
                                ..origin_buffer.anchor_before(origin_end),
                        }
                    });

                    let target_buffer = target_buffer_handle.read(cx);
                    let target_start = target_buffer
                        .clip_point_utf16(point_from_lsp(target_range.start), Bias::Left);
                    let target_end = target_buffer
                        .clip_point_utf16(point_from_lsp(target_range.end), Bias::Left);
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
        }

        Ok(definitions)
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
                buffer.wait_for_version(deserialize_version(message.version))
            })
            .await;
        Ok(Self {
            position: buffer.read_with(&cx, |buffer, _| position.to_point_utf16(buffer)),
        })
    }

    fn response_to_proto(
        response: Vec<LocationLink>,
        project: &mut Project,
        peer_id: PeerId,
        _: &clock::Global,
        cx: &AppContext,
    ) -> proto::GetDefinitionResponse {
        let links = response
            .into_iter()
            .map(|definition| {
                let origin = definition.origin.map(|origin| {
                    let buffer = project.serialize_buffer_for_peer(&origin.buffer, peer_id, cx);
                    proto::Location {
                        start: Some(serialize_anchor(&origin.range.start)),
                        end: Some(serialize_anchor(&origin.range.end)),
                        buffer: Some(buffer),
                    }
                });

                let buffer =
                    project.serialize_buffer_for_peer(&definition.target.buffer, peer_id, cx);
                let target = proto::Location {
                    start: Some(serialize_anchor(&definition.target.range.start)),
                    end: Some(serialize_anchor(&definition.target.range.end)),
                    buffer: Some(buffer),
                };

                proto::LocationLink {
                    origin,
                    target: Some(target),
                }
            })
            .collect();
        proto::GetDefinitionResponse { links }
    }

    async fn response_from_proto(
        self,
        message: proto::GetDefinitionResponse,
        project: ModelHandle<Project>,
        _: ModelHandle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Vec<LocationLink>> {
        let mut links = Vec::new();
        for link in message.links {
            let origin = match link.origin {
                Some(origin) => {
                    let buffer = origin
                        .buffer
                        .ok_or_else(|| anyhow!("missing origin buffer"))?;
                    let buffer = project
                        .update(&mut cx, |this, cx| this.deserialize_buffer(buffer, cx))
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
                        .await;
                    Some(Location {
                        buffer,
                        range: start..end,
                    })
                }
                None => None,
            };

            let target = link.target.ok_or_else(|| anyhow!("missing target"))?;
            let buffer = target.buffer.ok_or_else(|| anyhow!("missing buffer"))?;
            let buffer = project
                .update(&mut cx, |this, cx| this.deserialize_buffer(buffer, cx))
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
                .await;
            let target = Location {
                buffer,
                range: start..end,
            };

            links.push(LocationLink { origin, target })
        }
        Ok(links)
    }

    fn buffer_id_from_proto(message: &proto::GetDefinition) -> u64 {
        message.buffer_id
    }
}

#[async_trait(?Send)]
impl LspCommand for GetReferences {
    type Response = Vec<Location>;
    type LspRequest = lsp::request::References;
    type ProtoRequest = proto::GetReferences;

    fn to_lsp(&self, path: &Path, _: &AppContext) -> lsp::ReferenceParams {
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
        let (lsp_adapter, language_server) = project
            .read_with(&cx, |project, cx| {
                project
                    .language_server_for_buffer(buffer.read(cx), cx)
                    .map(|(adapter, server)| (adapter.clone(), server.clone()))
            })
            .ok_or_else(|| anyhow!("no language server found for buffer"))?;

        if let Some(locations) = locations {
            for lsp_location in locations {
                let target_buffer_handle = project
                    .update(&mut cx, |this, cx| {
                        this.open_local_buffer_via_lsp(
                            lsp_location.uri,
                            language_server.server_id(),
                            lsp_adapter.name(),
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
                buffer.wait_for_version(deserialize_version(message.version))
            })
            .await;
        Ok(Self {
            position: buffer.read_with(&cx, |buffer, _| position.to_point_utf16(buffer)),
        })
    }

    fn response_to_proto(
        response: Vec<Location>,
        project: &mut Project,
        peer_id: PeerId,
        _: &clock::Global,
        cx: &AppContext,
    ) -> proto::GetReferencesResponse {
        let locations = response
            .into_iter()
            .map(|definition| {
                let buffer = project.serialize_buffer_for_peer(&definition.buffer, peer_id, cx);
                proto::Location {
                    start: Some(serialize_anchor(&definition.range.start)),
                    end: Some(serialize_anchor(&definition.range.end)),
                    buffer: Some(buffer),
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
            let buffer = location.buffer.ok_or_else(|| anyhow!("missing buffer"))?;
            let target_buffer = project
                .update(&mut cx, |this, cx| this.deserialize_buffer(buffer, cx))
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
                .await;
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

    fn to_lsp(&self, path: &Path, _: &AppContext) -> lsp::DocumentHighlightParams {
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
                buffer.wait_for_version(deserialize_version(message.version))
            })
            .await;
        Ok(Self {
            position: buffer.read_with(&cx, |buffer, _| position.to_point_utf16(buffer)),
        })
    }

    fn response_to_proto(
        response: Vec<DocumentHighlight>,
        _: &mut Project,
        _: PeerId,
        _: &clock::Global,
        _: &AppContext,
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
                .await;
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

    fn to_lsp(&self, path: &Path, _: &AppContext) -> lsp::HoverParams {
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
        mut cx: AsyncAppContext,
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
                        .filter_map(|marked_string| HoverBlock::try_new(marked_string))
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
                                    let text = std::mem::replace(&mut current_text, String::new())
                                        .trim()
                                        .to_string();
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
                                    let text = std::mem::replace(&mut current_text, String::new())
                                        .trim()
                                        .to_string();
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
                buffer.wait_for_version(deserialize_version(message.version))
            })
            .await;
        Ok(Self {
            position: buffer.read_with(&cx, |buffer, _| position.to_point_utf16(buffer)),
        })
    }

    fn response_to_proto(
        response: Self::Response,
        _: &mut Project,
        _: PeerId,
        _: &clock::Global,
        _: &AppContext,
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
