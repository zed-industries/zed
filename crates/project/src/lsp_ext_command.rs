use crate::{lsp_command::LspCommand, lsp_store::LspStore};
use anyhow::{Context, Result};
use async_trait::async_trait;
use gpui::{AppContext, AsyncAppContext, Model};
use language::{point_to_lsp, proto::deserialize_anchor, Buffer};
use lsp::{LanguageServer, LanguageServerId};
use rpc::proto::{self, PeerId};
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc};
use text::{BufferId, PointUtf16, ToPointUtf16};

pub enum LspExpandMacro {}

impl lsp::request::Request for LspExpandMacro {
    type Params = ExpandMacroParams;
    type Result = Option<ExpandedMacro>;
    const METHOD: &'static str = "rust-analyzer/expandMacro";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExpandMacroParams {
    pub text_document: lsp::TextDocumentIdentifier,
    pub position: lsp::Position,
}

#[derive(Default, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExpandedMacro {
    pub name: String,
    pub expansion: String,
}

impl ExpandedMacro {
    pub fn is_empty(&self) -> bool {
        self.name.is_empty() && self.expansion.is_empty()
    }
}

pub struct ExpandMacro {
    pub position: PointUtf16,
}

#[async_trait(?Send)]
impl LspCommand for ExpandMacro {
    type Response = ExpandedMacro;
    type LspRequest = LspExpandMacro;
    type ProtoRequest = proto::LspExtExpandMacro;

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &AppContext,
    ) -> ExpandMacroParams {
        ExpandMacroParams {
            text_document: lsp::TextDocumentIdentifier {
                uri: lsp::Url::from_file_path(path).unwrap(),
            },
            position: point_to_lsp(self.position),
        }
    }

    async fn response_from_lsp(
        self,
        message: Option<ExpandedMacro>,
        _: Model<LspStore>,
        _: Model<Buffer>,
        _: LanguageServerId,
        _: AsyncAppContext,
    ) -> anyhow::Result<ExpandedMacro> {
        Ok(message
            .map(|message| ExpandedMacro {
                name: message.name,
                expansion: message.expansion,
            })
            .unwrap_or_default())
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::LspExtExpandMacro {
        proto::LspExtExpandMacro {
            project_id,
            buffer_id: buffer.remote_id().into(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
        }
    }

    async fn from_proto(
        message: Self::ProtoRequest,
        _: Model<LspStore>,
        buffer: Model<Buffer>,
        mut cx: AsyncAppContext,
    ) -> anyhow::Result<Self> {
        let position = message
            .position
            .and_then(deserialize_anchor)
            .context("invalid position")?;
        Ok(Self {
            position: buffer.update(&mut cx, |buffer, _| position.to_point_utf16(buffer))?,
        })
    }

    fn response_to_proto(
        response: ExpandedMacro,
        _: &mut LspStore,
        _: PeerId,
        _: &clock::Global,
        _: &mut AppContext,
    ) -> proto::LspExtExpandMacroResponse {
        proto::LspExtExpandMacroResponse {
            name: response.name,
            expansion: response.expansion,
        }
    }

    async fn response_from_proto(
        self,
        message: proto::LspExtExpandMacroResponse,
        _: Model<LspStore>,
        _: Model<Buffer>,
        _: AsyncAppContext,
    ) -> anyhow::Result<ExpandedMacro> {
        Ok(ExpandedMacro {
            name: message.name,
            expansion: message.expansion,
        })
    }

    fn buffer_id_from_proto(message: &proto::LspExtExpandMacro) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

pub enum LspSwitchSourceHeader {}

impl lsp::request::Request for LspSwitchSourceHeader {
    type Params = SwitchSourceHeaderParams;
    type Result = Option<SwitchSourceHeaderResult>;
    const METHOD: &'static str = "textDocument/switchSourceHeader";
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwitchSourceHeaderParams(lsp::TextDocumentIdentifier);

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SwitchSourceHeaderResult(pub String);

#[derive(Default, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwitchSourceHeader;

#[async_trait(?Send)]
impl LspCommand for SwitchSourceHeader {
    type Response = SwitchSourceHeaderResult;
    type LspRequest = LspSwitchSourceHeader;
    type ProtoRequest = proto::LspExtSwitchSourceHeader;

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &AppContext,
    ) -> SwitchSourceHeaderParams {
        SwitchSourceHeaderParams(lsp::TextDocumentIdentifier {
            uri: lsp::Url::from_file_path(path).unwrap(),
        })
    }

    async fn response_from_lsp(
        self,
        message: Option<SwitchSourceHeaderResult>,
        _: Model<LspStore>,
        _: Model<Buffer>,
        _: LanguageServerId,
        _: AsyncAppContext,
    ) -> anyhow::Result<SwitchSourceHeaderResult> {
        Ok(message
            .map(|message| SwitchSourceHeaderResult(message.0))
            .unwrap_or_default())
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::LspExtSwitchSourceHeader {
        proto::LspExtSwitchSourceHeader {
            project_id,
            buffer_id: buffer.remote_id().into(),
        }
    }

    async fn from_proto(
        _: Self::ProtoRequest,
        _: Model<LspStore>,
        _: Model<Buffer>,
        _: AsyncAppContext,
    ) -> anyhow::Result<Self> {
        Ok(Self {})
    }

    fn response_to_proto(
        response: SwitchSourceHeaderResult,
        _: &mut LspStore,
        _: PeerId,
        _: &clock::Global,
        _: &mut AppContext,
    ) -> proto::LspExtSwitchSourceHeaderResponse {
        proto::LspExtSwitchSourceHeaderResponse {
            target_file: response.0,
        }
    }

    async fn response_from_proto(
        self,
        message: proto::LspExtSwitchSourceHeaderResponse,
        _: Model<LspStore>,
        _: Model<Buffer>,
        _: AsyncAppContext,
    ) -> anyhow::Result<SwitchSourceHeaderResult> {
        Ok(SwitchSourceHeaderResult(message.target_file))
    }

    fn buffer_id_from_proto(message: &proto::LspExtSwitchSourceHeader) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}
