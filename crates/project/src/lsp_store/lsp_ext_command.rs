use crate::{
    lsp_command::{location_links_from_proto, location_links_to_proto, LspCommand},
    lsp_store::LspStore,
    make_text_document_identifier, LocationLink,
};
use anyhow::{Context as _, Result};
use async_trait::async_trait;
use gpui::{App, AsyncApp, Entity};
use language::{point_to_lsp, proto::deserialize_anchor, Buffer};
use lsp::{LanguageServer, LanguageServerId};
use rpc::proto::{self, PeerId};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path, sync::Arc};
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

#[derive(Debug)]
pub struct ExpandMacro {
    pub position: PointUtf16,
}

pub enum LspFindRunnables {}

impl lsp::request::Request for LspFindRunnables {
    type Params = FindRunnablesParams;
    type Result = Vec<Runnable2>;
    const METHOD: &'static str = "experimental/runnables";
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Runnable2 {
    pub label: String,
    /// If this Runnable is associated with a specific function/module, etc., the location of this item
    pub location: Option<lsp::LocationLink>,
    /// Running things is necessary technology specific, `kind` needs to be advertised via server capabilities,
    /// the type of `args` is specific to `kind`. The actual running is handled by the client.
    pub kind: String,
    pub args: RunableArgs,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum RunableArgs {
    Cargo {
        /// Environment variables to set before running the command.
        environment: Option<HashMap<String, String>>,
        cwd: String,
        /// The workspace root directory of the cargo project.
        #[serde(rename = "workspaceRoot")]
        workspace_root: Option<String>,
        /// The cargo command to run.
        #[serde(rename = "cargoArgs")]
        cargo_args: Vec<String>,
        /// Arguments to pass to the executable, these will be passed to the command after a `--` argument.
        #[serde(rename = "executableArgs")]
        executable_args: Vec<String>,
        /// Command to execute instead of `cargo`.
        #[serde(rename = "overrideCargo")]
        override_cargo: Option<String>,
    },
    Shell(RunnableCommand),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RunnableCommand {
    pub kind: String,
    pub program: String,
    pub args: Vec<String>,
    /// The working directory to run the command in.
    pub cwd: String,
    /// Environment variables to set before running the command.
    pub environment: Option<HashMap<String, String>>,
}

impl From<RunableArgs> for RunnableCommand {
    fn from(value: RunableArgs) -> Self {
        match value {
            RunableArgs::Cargo {
                environment,
                cwd,
                cargo_args,
                executable_args,
                override_cargo,
                ..
            } => RunnableCommand {
                kind: "cargo".to_string(),
                program: override_cargo
                    .clone()
                    .unwrap_or_else(|| "cargo".to_string()),
                args: {
                    let mut args = cargo_args;
                    if !executable_args.is_empty() {
                        args.push("--".to_string());
                        args.extend(executable_args);
                    }
                    args
                },
                cwd,
                environment,
            },
            RunableArgs::Shell(v) => v,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FindRunnablesParams {
    pub text_document: lsp::TextDocumentIdentifier,
    /// If null, compute runnables for the whole file.
    pub position: Option<lsp::Position>,
}

#[derive(Debug)]
pub struct FindRunnables {
    pub position: Option<PointUtf16>,
}

#[derive(Debug)]
pub struct Runnable {
    pub label: String,
    pub location: Option<LocationLink>,
    pub kind: String,
    pub command: RunnableCommand,
}

#[async_trait(?Send)]
impl LspCommand for FindRunnables {
    type Response = Vec<Runnable>;
    type LspRequest = LspFindRunnables;
    type ProtoRequest = proto::LspExtFindRunnable;

    fn display_name(&self) -> &str {
        "Find runnables"
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<FindRunnablesParams> {
        Ok(FindRunnablesParams {
            text_document: make_text_document_identifier(path)?,
            position: self.position.map(point_to_lsp),
        })
    }

    async fn response_from_lsp(
        self,
        message: Vec<Runnable2>,
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: LanguageServerId,
        _: AsyncApp,
    ) -> anyhow::Result<Vec<Runnable>> {
        Ok(message
            .into_iter()
            .map(|v| Runnable {
                label: v.label,
                //TODO: location, args
                location: None,
                kind: v.kind,
                command: v.args.into(),
            })
            .collect())
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::LspExtFindRunnable {
        proto::LspExtFindRunnable {
            project_id,
            buffer_id: buffer.remote_id().into(),
            position: self
                .position
                .map(|pos| language::proto::serialize_anchor(&buffer.anchor_before(pos))),
        }
    }

    async fn from_proto(
        _: Self::ProtoRequest,
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: AsyncApp,
    ) -> anyhow::Result<Self> {
        //TODO:
        Ok(Self { position: None })
    }

    fn response_to_proto(
        response: Vec<Runnable>,
        lsp_store: &mut LspStore,
        peer_id: PeerId,
        _: &clock::Global,
        cx: &mut App,
    ) -> proto::LspExtFindRunnableResponse {
        let mut runnables = vec![];
        let mut locations = vec![];
        let mut links = vec![];
        for runnable in response {
            locations.push(runnable.location.is_some());
            if let Some(location) = runnable.location {
                links.push(location);
            }
            runnables.push(proto::Runnable {
                label: runnable.label,
                location: None,
                kind: runnable.kind,
                command: Some(proto::RunnableCommand {
                    kind: runnable.command.kind,
                    program: runnable.command.program,
                    args: runnable.command.args,
                    cwd: runnable.command.cwd,
                    environment: runnable
                        .command
                        .environment
                        .unwrap_or_default()
                        .into_iter()
                        .map(|(k, v)| proto::EnviromentArg { key: k, value: v })
                        .collect(),
                }),
            });
        }
        let mut links = location_links_to_proto(links, lsp_store, peer_id, cx).into_iter();
        for (i, runnable) in runnables.iter_mut().enumerate() {
            if locations[i] {
                runnable.location = Some(links.next().unwrap());
            }
        }
        proto::LspExtFindRunnableResponse { runnables }
    }

    async fn response_from_proto(
        self,
        message: proto::LspExtFindRunnableResponse,
        lsp_store: Entity<LspStore>,
        _: Entity<Buffer>,
        cx: AsyncApp,
    ) -> anyhow::Result<Vec<Runnable>> {
        let mut out = vec![];
        let mut locations = vec![];
        let mut proto_links = vec![];
        for runnable in message.runnables {
            locations.push(runnable.location.is_some());
            if let Some(location) = runnable.location {
                proto_links.push(location);
            }
            out.push(Runnable {
                label: runnable.label,
                location: None,
                kind: runnable.kind,
                command: {
                    let command = runnable.command.unwrap();
                    RunnableCommand {
                        kind: command.kind,
                        program: command.program,
                        args: command.args,
                        cwd: command.cwd,
                        environment: match command.environment.is_empty() {
                            true => None,
                            false => Some(
                                command
                                    .environment
                                    .into_iter()
                                    .map(|arg| (arg.key, arg.value))
                                    .collect(),
                            ),
                        },
                    }
                },
            });
        }
        let mut links = location_links_from_proto(proto_links, lsp_store, cx)
            .await?
            .into_iter();
        for (i, runnable) in out.iter_mut().enumerate() {
            if locations[i] {
                runnable.location = Some(links.next().unwrap());
            }
        }
        Ok(out)
    }

    fn buffer_id_from_proto(message: &proto::LspExtFindRunnable) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

#[async_trait(?Send)]
impl LspCommand for ExpandMacro {
    type Response = ExpandedMacro;
    type LspRequest = LspExpandMacro;
    type ProtoRequest = proto::LspExtExpandMacro;

    fn display_name(&self) -> &str {
        "Expand macro"
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<ExpandMacroParams> {
        Ok(ExpandMacroParams {
            text_document: make_text_document_identifier(path)?,
            position: point_to_lsp(self.position),
        })
    }

    async fn response_from_lsp(
        self,
        message: Option<ExpandedMacro>,
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: LanguageServerId,
        _: AsyncApp,
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
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
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
        _: &mut App,
    ) -> proto::LspExtExpandMacroResponse {
        proto::LspExtExpandMacroResponse {
            name: response.name,
            expansion: response.expansion,
        }
    }

    async fn response_from_proto(
        self,
        message: proto::LspExtExpandMacroResponse,
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: AsyncApp,
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

pub enum LspOpenDocs {}

impl lsp::request::Request for LspOpenDocs {
    type Params = OpenDocsParams;
    type Result = Option<DocsUrls>;
    const METHOD: &'static str = "experimental/externalDocs";
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenDocsParams {
    pub text_document: lsp::TextDocumentIdentifier,
    pub position: lsp::Position,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DocsUrls {
    pub web: Option<String>,
    pub local: Option<String>,
}

impl DocsUrls {
    pub fn is_empty(&self) -> bool {
        self.web.is_none() && self.local.is_none()
    }
}

#[derive(Debug)]
pub struct OpenDocs {
    pub position: PointUtf16,
}

#[async_trait(?Send)]
impl LspCommand for OpenDocs {
    type Response = DocsUrls;
    type LspRequest = LspOpenDocs;
    type ProtoRequest = proto::LspExtOpenDocs;

    fn display_name(&self) -> &str {
        "Open docs"
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<OpenDocsParams> {
        Ok(OpenDocsParams {
            text_document: lsp::TextDocumentIdentifier {
                uri: lsp::Url::from_file_path(path).unwrap(),
            },
            position: point_to_lsp(self.position),
        })
    }

    async fn response_from_lsp(
        self,
        message: Option<DocsUrls>,
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: LanguageServerId,
        _: AsyncApp,
    ) -> anyhow::Result<DocsUrls> {
        Ok(message
            .map(|message| DocsUrls {
                web: message.web,
                local: message.local,
            })
            .unwrap_or_default())
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::LspExtOpenDocs {
        proto::LspExtOpenDocs {
            project_id,
            buffer_id: buffer.remote_id().into(),
            position: Some(language::proto::serialize_anchor(
                &buffer.anchor_before(self.position),
            )),
        }
    }

    async fn from_proto(
        message: Self::ProtoRequest,
        _: Entity<LspStore>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
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
        response: DocsUrls,
        _: &mut LspStore,
        _: PeerId,
        _: &clock::Global,
        _: &mut App,
    ) -> proto::LspExtOpenDocsResponse {
        proto::LspExtOpenDocsResponse {
            web: response.web,
            local: response.local,
        }
    }

    async fn response_from_proto(
        self,
        message: proto::LspExtOpenDocsResponse,
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: AsyncApp,
    ) -> anyhow::Result<DocsUrls> {
        Ok(DocsUrls {
            web: message.web,
            local: message.local,
        })
    }

    fn buffer_id_from_proto(message: &proto::LspExtOpenDocs) -> Result<BufferId> {
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

    fn display_name(&self) -> &str {
        "Switch source header"
    }

    fn to_lsp(
        &self,
        path: &Path,
        _: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<SwitchSourceHeaderParams> {
        Ok(SwitchSourceHeaderParams(make_text_document_identifier(
            path,
        )?))
    }

    async fn response_from_lsp(
        self,
        message: Option<SwitchSourceHeaderResult>,
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: LanguageServerId,
        _: AsyncApp,
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
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: AsyncApp,
    ) -> anyhow::Result<Self> {
        Ok(Self {})
    }

    fn response_to_proto(
        response: SwitchSourceHeaderResult,
        _: &mut LspStore,
        _: PeerId,
        _: &clock::Global,
        _: &mut App,
    ) -> proto::LspExtSwitchSourceHeaderResponse {
        proto::LspExtSwitchSourceHeaderResponse {
            target_file: response.0,
        }
    }

    async fn response_from_proto(
        self,
        message: proto::LspExtSwitchSourceHeaderResponse,
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: AsyncApp,
    ) -> anyhow::Result<SwitchSourceHeaderResult> {
        Ok(SwitchSourceHeaderResult(message.target_file))
    }

    fn buffer_id_from_proto(message: &proto::LspExtSwitchSourceHeader) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}
