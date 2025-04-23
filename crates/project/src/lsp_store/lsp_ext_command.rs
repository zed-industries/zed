use crate::{
    LocationLink,
    lsp_command::{
        LspCommand, location_link_from_lsp, location_link_from_proto, location_link_to_proto,
    },
    lsp_store::LspStore,
    make_text_document_identifier,
};
use anyhow::{Context as _, Result};
use async_trait::async_trait;
use collections::HashMap;
use gpui::{App, AsyncApp, Entity};
use language::{
    Buffer, point_to_lsp,
    proto::{deserialize_anchor, serialize_anchor},
};
use lsp::{LanguageServer, LanguageServerId};
use rpc::proto::{self, PeerId};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use task::TaskTemplate;
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

// https://rust-analyzer.github.io/book/contributing/lsp-extensions.html#runnables
// Taken from https://github.com/rust-lang/rust-analyzer/blob/a73a37a757a58b43a796d3eb86a1f7dfd0036659/crates/rust-analyzer/src/lsp/ext.rs#L425-L489
pub enum Runnables {}

impl lsp::request::Request for Runnables {
    type Params = RunnablesParams;
    type Result = Vec<Runnable>;
    const METHOD: &'static str = "experimental/runnables";
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RunnablesParams {
    pub text_document: lsp::TextDocumentIdentifier,
    #[serde(default)]
    pub position: Option<lsp::Position>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Runnable {
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<lsp::LocationLink>,
    pub kind: RunnableKind,
    pub args: RunnableArgs,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum RunnableArgs {
    Cargo(CargoRunnableArgs),
    Shell(ShellRunnableArgs),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RunnableKind {
    Cargo,
    Shell,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CargoRunnableArgs {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub environment: HashMap<String, String>,
    pub cwd: PathBuf,
    /// Command to be executed instead of cargo
    #[serde(default)]
    pub override_cargo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_root: Option<PathBuf>,
    // command, --package and --lib stuff
    #[serde(default)]
    pub cargo_args: Vec<String>,
    // stuff after --
    #[serde(default)]
    pub executable_args: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShellRunnableArgs {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub environment: HashMap<String, String>,
    pub cwd: PathBuf,
    pub program: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug)]
pub struct GetLspRunnables {
    pub buffer_id: BufferId,
    pub position: Option<text::Anchor>,
}

#[derive(Debug, Default)]
pub struct LspRunnables {
    pub runnables: Vec<(Option<LocationLink>, TaskTemplate)>,
}

#[async_trait(?Send)]
impl LspCommand for GetLspRunnables {
    type Response = LspRunnables;
    type LspRequest = Runnables;
    type ProtoRequest = proto::LspExtRunnables;

    fn display_name(&self) -> &str {
        "LSP Runnables"
    }

    fn to_lsp(
        &self,
        path: &Path,
        buffer: &Buffer,
        _: &Arc<LanguageServer>,
        _: &App,
    ) -> Result<RunnablesParams> {
        let url = match lsp::Url::from_file_path(path) {
            Ok(url) => url,
            Err(()) => anyhow::bail!("Failed to parse path {path:?} as lsp::Url"),
        };
        Ok(RunnablesParams {
            text_document: lsp::TextDocumentIdentifier::new(url),
            position: self
                .position
                .map(|anchor| point_to_lsp(anchor.to_point_utf16(&buffer.snapshot()))),
        })
    }

    async fn response_from_lsp(
        self,
        lsp_runnables: Vec<Runnable>,
        lsp_store: Entity<LspStore>,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        mut cx: AsyncApp,
    ) -> Result<LspRunnables> {
        let mut runnables = Vec::with_capacity(lsp_runnables.len());

        for runnable in lsp_runnables {
            let location = match runnable.location {
                Some(location) => Some(
                    location_link_from_lsp(location, &lsp_store, &buffer, server_id, &mut cx)
                        .await?,
                ),
                None => None,
            };
            let mut task_template = TaskTemplate::default();
            task_template.label = runnable.label;
            match runnable.args {
                RunnableArgs::Cargo(cargo) => {
                    match cargo.override_cargo {
                        Some(override_cargo) => {
                            let mut override_parts =
                                override_cargo.split(" ").map(|s| s.to_string());
                            task_template.command = override_parts
                                .next()
                                .unwrap_or_else(|| override_cargo.clone());
                            task_template.args.extend(override_parts);
                        }
                        None => task_template.command = "cargo".to_string(),
                    };
                    task_template.env = cargo.environment;
                    task_template.cwd = Some(
                        cargo
                            .workspace_root
                            .unwrap_or(cargo.cwd)
                            .to_string_lossy()
                            .to_string(),
                    );
                    task_template.args.extend(cargo.cargo_args);
                    if !cargo.executable_args.is_empty() {
                        task_template.args.push("--".to_string());
                        task_template.args.extend(
                            cargo
                                .executable_args
                                .into_iter()
                                // rust-analyzer's doctest data may be smth. like
                                // ```
                                // command: "cargo",
                                // args: [
                                //     "test",
                                //     "--doc",
                                //     "--package",
                                //     "cargo-output-parser",
                                //     "--",
                                //     "X<T>::new",
                                //     "--show-output",
                                // ],
                                // ```
                                // and `X<T>::new` will cause troubles if not escaped properly, as later
                                // the task runs as `$SHELL -i -c "cargo test ..."`.
                                //
                                // We cannot escape all shell arguments unconditionally, as we use this for ssh commands, which may involve paths starting with `~`.
                                // That bit is not auto-expanded when using single quotes.
                                // Escape extra cargo args unconditionally as those are unlikely to contain `~`.
                                .map(|extra_arg| format!("'{extra_arg}'")),
                        );
                    }
                }
                RunnableArgs::Shell(shell) => {
                    task_template.command = shell.program;
                    task_template.args = shell.args;
                    task_template.env = shell.environment;
                    task_template.cwd = Some(shell.cwd.to_string_lossy().to_string());
                }
            }

            runnables.push((location, task_template));
        }

        Ok(LspRunnables { runnables })
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::LspExtRunnables {
        proto::LspExtRunnables {
            project_id,
            buffer_id: buffer.remote_id().to_proto(),
            position: self.position.as_ref().map(serialize_anchor),
        }
    }

    async fn from_proto(
        message: proto::LspExtRunnables,
        _: Entity<LspStore>,
        _: Entity<Buffer>,
        _: AsyncApp,
    ) -> Result<Self> {
        let buffer_id = Self::buffer_id_from_proto(&message)?;
        let position = message.position.and_then(deserialize_anchor);
        Ok(Self {
            buffer_id,
            position,
        })
    }

    fn response_to_proto(
        response: LspRunnables,
        lsp_store: &mut LspStore,
        peer_id: PeerId,
        _: &clock::Global,
        cx: &mut App,
    ) -> proto::LspExtRunnablesResponse {
        proto::LspExtRunnablesResponse {
            runnables: response
                .runnables
                .into_iter()
                .map(|(location, task_template)| proto::LspRunnable {
                    location: location
                        .map(|location| location_link_to_proto(location, lsp_store, peer_id, cx)),
                    task_template: serde_json::to_vec(&task_template).unwrap(),
                })
                .collect(),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::LspExtRunnablesResponse,
        lsp_store: Entity<LspStore>,
        _: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<LspRunnables> {
        let mut runnables = LspRunnables {
            runnables: Vec::new(),
        };

        for lsp_runnable in message.runnables {
            let location = match lsp_runnable.location {
                Some(location) => {
                    Some(location_link_from_proto(location, &lsp_store, &mut cx).await?)
                }
                None => None,
            };
            let task_template = serde_json::from_slice(&lsp_runnable.task_template)
                .context("deserializing task template from proto")?;
            runnables.runnables.push((location, task_template));
        }

        Ok(runnables)
    }

    fn buffer_id_from_proto(message: &proto::LspExtRunnables) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}
