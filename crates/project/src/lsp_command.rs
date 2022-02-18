use crate::{Project, ProjectTransaction};
use anyhow::{anyhow, Result};
use client::proto;
use futures::{future::LocalBoxFuture, FutureExt};
use gpui::{AppContext, AsyncAppContext, ModelHandle};
use language::{
    proto::deserialize_anchor, range_from_lsp, Anchor, Bias, Buffer, PointUtf16, ToLspPosition,
};
use std::{ops::Range, path::Path};

pub(crate) trait LspCommand: 'static {
    type Response: 'static + Default + Send;
    type LspRequest: 'static + Send + lsp::request::Request;
    type ProtoRequest: 'static + Send + proto::RequestMessage;

    fn to_lsp(
        &self,
        path: &Path,
        cx: &AppContext,
    ) -> <Self::LspRequest as lsp::request::Request>::Params;
    fn to_proto(&self, project_id: u64, cx: &AppContext) -> Self::ProtoRequest;
    fn response_from_lsp(
        self,
        message: <Self::LspRequest as lsp::request::Request>::Result,
        project: ModelHandle<Project>,
        cx: AsyncAppContext,
    ) -> LocalBoxFuture<'static, Result<Self::Response>>;
    fn response_from_proto(
        self,
        message: <Self::ProtoRequest as proto::RequestMessage>::Response,
        project: ModelHandle<Project>,
        cx: AsyncAppContext,
    ) -> LocalBoxFuture<'static, Result<Self::Response>>;
}

pub(crate) struct PrepareRename {
    pub buffer: ModelHandle<Buffer>,
    pub position: PointUtf16,
}

#[derive(Debug)]
pub(crate) struct PerformRename {
    pub buffer: ModelHandle<Buffer>,
    pub position: PointUtf16,
    pub new_name: String,
    pub push_to_history: bool,
}

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

    fn to_proto(&self, project_id: u64, cx: &AppContext) -> proto::PrepareRename {
        let buffer = &self.buffer.read(cx);
        let buffer_id = buffer.remote_id();
        proto::PrepareRename {
            project_id,
            buffer_id,
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
        }
    }

    fn response_from_lsp(
        self,
        message: Option<lsp::PrepareRenameResponse>,
        _: ModelHandle<Project>,
        cx: AsyncAppContext,
    ) -> LocalBoxFuture<'static, Result<Option<Range<Anchor>>>> {
        async move {
            Ok(message.and_then(|result| match result {
                lsp::PrepareRenameResponse::Range(range)
                | lsp::PrepareRenameResponse::RangeWithPlaceholder { range, .. } => {
                    self.buffer.read_with(&cx, |buffer, _| {
                        let range = range_from_lsp(range);
                        if buffer.clip_point_utf16(range.start, Bias::Left) == range.start
                            && buffer.clip_point_utf16(range.end, Bias::Left) == range.end
                        {
                            Some(buffer.anchor_after(range.start)..buffer.anchor_before(range.end))
                        } else {
                            None
                        }
                    })
                }
                _ => None,
            }))
        }
        .boxed_local()
    }

    fn response_from_proto(
        self,
        message: proto::PrepareRenameResponse,
        _: ModelHandle<Project>,
        mut cx: AsyncAppContext,
    ) -> LocalBoxFuture<'static, Result<Option<Range<Anchor>>>> {
        async move {
            if message.can_rename {
                self.buffer
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
        .boxed_local()
    }
}

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

    fn to_proto(&self, project_id: u64, cx: &AppContext) -> proto::PerformRename {
        let buffer = &self.buffer.read(cx);
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

    fn response_from_lsp(
        self,
        message: Option<lsp::WorkspaceEdit>,
        project: ModelHandle<Project>,
        mut cx: AsyncAppContext,
    ) -> LocalBoxFuture<'static, Result<ProjectTransaction>> {
        async move {
            if let Some(edit) = message {
                let (language_name, language_server) =
                    self.buffer.read_with(&cx, |buffer, _| {
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
        .boxed_local()
    }

    fn response_from_proto(
        self,
        message: proto::PerformRenameResponse,
        project: ModelHandle<Project>,
        mut cx: AsyncAppContext,
    ) -> LocalBoxFuture<'static, Result<ProjectTransaction>> {
        async move {
            let message = message
                .transaction
                .ok_or_else(|| anyhow!("missing transaction"))?;
            project
                .update(&mut cx, |project, cx| {
                    project.deserialize_project_transaction(message, self.push_to_history, cx)
                })
                .await
        }
        .boxed_local()
    }
}
