use crate::{Project, ProjectTransaction};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use client::{proto, PeerId};
use gpui::{AppContext, AsyncAppContext, ModelContext, ModelHandle};
use language::{
    proto::deserialize_anchor, range_from_lsp, Anchor, Bias, Buffer, PointUtf16, ToLspPosition,
    ToPointUtf16,
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

    fn to_proto(
        &self,
        project_id: u64,
        buffer: &ModelHandle<Buffer>,
        cx: &AppContext,
    ) -> Self::ProtoRequest;
    fn from_proto(
        message: Self::ProtoRequest,
        project: &mut Project,
        buffer: &ModelHandle<Buffer>,
        cx: &mut ModelContext<Project>,
    ) -> Result<Self>;
    fn buffer_id_from_proto(message: &Self::ProtoRequest) -> u64;

    fn response_to_proto(
        response: Self::Response,
        project: &mut Project,
        peer_id: PeerId,
        buffer_version: &clock::Global,
        cx: &mut ModelContext<Project>,
    ) -> <Self::ProtoRequest as proto::RequestMessage>::Response;
    async fn response_from_proto(
        self,
        message: <Self::ProtoRequest as proto::RequestMessage>::Response,
        project: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        cx: AsyncAppContext,
    ) -> Result<Self::Response>;
}

pub(crate) struct PrepareRename {
    pub position: PointUtf16,
}

pub(crate) struct PerformRename {
    pub position: PointUtf16,
    pub new_name: String,
    pub push_to_history: bool,
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

    fn to_proto(
        &self,
        project_id: u64,
        buffer: &ModelHandle<Buffer>,
        cx: &AppContext,
    ) -> proto::PrepareRename {
        proto::PrepareRename {
            project_id,
            buffer_id: buffer.read(cx).remote_id(),
            position: Some(language::proto::serialize_anchor(
                &buffer.read(cx).anchor_before(self.position),
            )),
        }
    }

    fn buffer_id_from_proto(message: &proto::PrepareRename) -> u64 {
        message.buffer_id
    }

    fn from_proto(
        message: proto::PrepareRename,
        _: &mut Project,
        buffer: &ModelHandle<Buffer>,
        cx: &mut ModelContext<Project>,
    ) -> Result<Self> {
        let buffer = buffer.read(cx);
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
        _: &mut ModelContext<Project>,
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

    fn to_proto(
        &self,
        project_id: u64,
        buffer: &ModelHandle<Buffer>,
        cx: &AppContext,
    ) -> proto::PerformRename {
        let buffer = buffer.read(cx);
        let buffer_id = buffer.remote_id();
        proto::PerformRename {
            project_id,
            buffer_id,
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
            new_name: self.new_name.clone(),
        }
    }

    fn buffer_id_from_proto(message: &proto::PerformRename) -> u64 {
        message.buffer_id
    }

    fn from_proto(
        message: proto::PerformRename,
        _: &mut Project,
        buffer: &ModelHandle<Buffer>,
        cx: &mut ModelContext<Project>,
    ) -> Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        let buffer = buffer.read(cx);
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
        cx: &mut ModelContext<Project>,
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
}
