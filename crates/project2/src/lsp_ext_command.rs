use std::{path::Path, sync::Arc};

use async_trait::async_trait;
use gpui::{AppContext, AsyncAppContext, Model};
use language::Buffer;
use lsp::{LanguageServer, LanguageServerId};
use rpc::proto::{self, PeerId};
use serde::{Deserialize, Serialize};

use crate::{lsp_command::LspCommand, Project};

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

pub struct ExpandMacro {}

// TODO kb
#[async_trait(?Send)]
impl LspCommand for ExpandMacro {
    type Response = ExpandedMacro;
    type LspRequest = LspExpandMacro;
    type ProtoRequest = proto::LspExtExpandMacro;

    fn to_lsp(
        &self,
        path: &Path,
        buffer: &Buffer,
        language_server: &Arc<LanguageServer>,
        cx: &AppContext,
    ) -> ExpandMacroParams {
        todo!()
    }

    async fn response_from_lsp(
        self,
        message: Option<ExpandedMacro>,
        project: Model<Project>,
        buffer: Model<Buffer>,
        server_id: LanguageServerId,
        cx: AsyncAppContext,
    ) -> anyhow::Result<ExpandedMacro> {
        anyhow::bail!("TODO kb")
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::LspExtExpandMacro {
        todo!()
    }

    async fn from_proto(
        message: Self::ProtoRequest,
        project: Model<Project>,
        buffer: Model<Buffer>,
        cx: AsyncAppContext,
    ) -> anyhow::Result<Self> {
        todo!()
    }

    fn response_to_proto(
        response: ExpandedMacro,
        project: &mut Project,
        peer_id: PeerId,
        buffer_version: &clock::Global,
        cx: &mut AppContext,
    ) -> proto::LspExtExpandMacroResponse {
        todo!()
    }

    async fn response_from_proto(
        self,
        message: proto::LspExtExpandMacroResponse,
        project: Model<Project>,
        buffer: Model<Buffer>,
        cx: AsyncAppContext,
    ) -> anyhow::Result<ExpandedMacro> {
        todo!()
    }

    fn buffer_id_from_proto(message: &proto::LspExtExpandMacro) -> u64 {
        message.buffer_id
    }
}
