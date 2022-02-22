use crate::{Definition, Project, ProjectTransaction};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use client::{proto, PeerId};
use gpui::{AppContext, AsyncAppContext, ModelHandle};
use language::{
    point_from_lsp,
    proto::{deserialize_anchor, serialize_anchor},
    range_from_lsp, Anchor, Bias, Buffer, PointUtf16, ToLspPosition, ToPointUtf16,
};
use std::{ops::Range, path::Path};

#[async_trait(?Send)]
pub(crate) trait LspCommand: 'static + Sized {
    type Response: 'static + Default + Send;
    type LspRequest: 'static + Send + lsp::request::Request;
    type ProtoRequest: 'static + Send + proto::RequestMessage;

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
    fn from_proto(
        message: Self::ProtoRequest,
        project: &mut Project,
        buffer: &Buffer,
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

#[async_trait(?Send)]
impl LspCommand for PrepareRename {
    type Response = Option<Range<Anchor>>;
    type LspRequest = lsp::request::PrepareRenameRequest;
    type ProtoRequest = proto::PrepareRename;

    fn to_lsp(&self, path: &Path, _: &AppContext) -> lsp::TextDocumentPositionParams {
        lsp::TextDocumentPositionParams {
            text_document: lsp::TextDocumentIdentifier {
                uri: lsp::Url::from_file_path(path).unwrap(),
            },
            position: self.position.to_lsp_position(),
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
        }
    }

    fn from_proto(message: proto::PrepareRename, _: &mut Project, buffer: &Buffer) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        if !buffer.can_resolve(&position) {
            Err(anyhow!("cannot resolve position"))?;
        }
        Ok(Self {
            position: position.to_point_utf16(buffer),
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
            version: buffer_version.into(),
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
                    buffer.wait_for_version(message.version.into())
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
                position: self.position.to_lsp_position(),
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
            let (language_name, language_server) = buffer.read_with(&cx, |buffer, _| {
                let language = buffer
                    .language()
                    .ok_or_else(|| anyhow!("buffer's language was removed"))?;
                let language_server = buffer
                    .language_server()
                    .cloned()
                    .ok_or_else(|| anyhow!("buffer's language server was removed"))?;
                Ok::<_, anyhow::Error>((language.name().to_string(), language_server))
            })?;
            Project::deserialize_workspace_edit(
                project,
                edit,
                self.push_to_history,
                language_name,
                language_server,
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
        }
    }

    fn from_proto(message: proto::PerformRename, _: &mut Project, buffer: &Buffer) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        if !buffer.can_resolve(&position) {
            Err(anyhow!("cannot resolve position"))?;
        }
        Ok(Self {
            position: position.to_point_utf16(buffer),
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
    type Response = Vec<Definition>;
    type LspRequest = lsp::request::GotoDefinition;
    type ProtoRequest = proto::GetDefinition;

    fn to_lsp(&self, path: &Path, _: &AppContext) -> lsp::GotoDefinitionParams {
        lsp::GotoDefinitionParams {
            text_document_position_params: lsp::TextDocumentPositionParams {
                text_document: lsp::TextDocumentIdentifier {
                    uri: lsp::Url::from_file_path(path).unwrap(),
                },
                position: self.position.to_lsp_position(),
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
    ) -> Result<Vec<Definition>> {
        let mut definitions = Vec::new();
        let (language, language_server) = buffer
            .read_with(&cx, |buffer, _| {
                buffer
                    .language()
                    .cloned()
                    .zip(buffer.language_server().cloned())
            })
            .ok_or_else(|| anyhow!("buffer no longer has language server"))?;

        if let Some(message) = message {
            let mut unresolved_locations = Vec::new();
            match message {
                lsp::GotoDefinitionResponse::Scalar(loc) => {
                    unresolved_locations.push((loc.uri, loc.range));
                }
                lsp::GotoDefinitionResponse::Array(locs) => {
                    unresolved_locations.extend(locs.into_iter().map(|l| (l.uri, l.range)));
                }
                lsp::GotoDefinitionResponse::Link(links) => {
                    unresolved_locations.extend(
                        links
                            .into_iter()
                            .map(|l| (l.target_uri, l.target_selection_range)),
                    );
                }
            }

            for (target_uri, target_range) in unresolved_locations {
                let target_buffer_handle = project
                    .update(&mut cx, |this, cx| {
                        this.open_local_buffer_via_lsp(
                            target_uri,
                            language.name().to_string(),
                            language_server.clone(),
                            cx,
                        )
                    })
                    .await?;

                cx.read(|cx| {
                    let target_buffer = target_buffer_handle.read(cx);
                    let target_start = target_buffer
                        .clip_point_utf16(point_from_lsp(target_range.start), Bias::Left);
                    let target_end = target_buffer
                        .clip_point_utf16(point_from_lsp(target_range.end), Bias::Left);
                    definitions.push(Definition {
                        target_buffer: target_buffer_handle,
                        target_range: target_buffer.anchor_after(target_start)
                            ..target_buffer.anchor_before(target_end),
                    });
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
        }
    }

    fn from_proto(message: proto::GetDefinition, _: &mut Project, buffer: &Buffer) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        if !buffer.can_resolve(&position) {
            Err(anyhow!("cannot resolve position"))?;
        }
        Ok(Self {
            position: position.to_point_utf16(buffer),
        })
    }

    fn response_to_proto(
        response: Vec<Definition>,
        project: &mut Project,
        peer_id: PeerId,
        _: &clock::Global,
        cx: &AppContext,
    ) -> proto::GetDefinitionResponse {
        let definitions = response
            .into_iter()
            .map(|definition| {
                let buffer =
                    project.serialize_buffer_for_peer(&definition.target_buffer, peer_id, cx);
                proto::Definition {
                    target_start: Some(serialize_anchor(&definition.target_range.start)),
                    target_end: Some(serialize_anchor(&definition.target_range.end)),
                    buffer: Some(buffer),
                }
            })
            .collect();
        proto::GetDefinitionResponse { definitions }
    }

    async fn response_from_proto(
        self,
        message: proto::GetDefinitionResponse,
        project: ModelHandle<Project>,
        _: ModelHandle<Buffer>,
        mut cx: AsyncAppContext,
    ) -> Result<Vec<Definition>> {
        let mut definitions = Vec::new();
        for definition in message.definitions {
            let buffer = definition.buffer.ok_or_else(|| anyhow!("missing buffer"))?;
            let target_buffer = project
                .update(&mut cx, |this, cx| this.deserialize_buffer(buffer, cx))
                .await?;
            let target_start = definition
                .target_start
                .and_then(deserialize_anchor)
                .ok_or_else(|| anyhow!("missing target start"))?;
            let target_end = definition
                .target_end
                .and_then(deserialize_anchor)
                .ok_or_else(|| anyhow!("missing target end"))?;
            definitions.push(Definition {
                target_buffer,
                target_range: target_start..target_end,
            })
        }
        Ok(definitions)
    }

    fn buffer_id_from_proto(message: &proto::GetDefinition) -> u64 {
        message.buffer_id
    }
}
