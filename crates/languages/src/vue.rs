use anyhow::{anyhow, ensure, Result};
use async_trait::async_trait;
use futures::StreamExt;
pub use language::*;
use lsp::{CodeActionKind, LanguageServerBinary};
use node_runtime::NodeRuntime;
use parking_lot::Mutex;
use smol::fs::{self};
use std::{
    any::Any,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{maybe, ResultExt};

pub struct VueLspVersion {
    vue_version: String,
    ts_version: String,
}

pub struct VueLspAdapter {
    node: Arc<dyn NodeRuntime>,
    typescript_install_path: Mutex<Option<PathBuf>>,
}

impl VueLspAdapter {
    const SERVER_PATH: &'static str =
        "node_modules/@vue/language-server/bin/vue-language-server.js";
    // TODO: this can't be hardcoded, yet we have to figure out how to pass it in initialization_options.
    const TYPESCRIPT_PATH: &'static str = "node_modules/typescript/lib";
    pub fn new(node: Arc<dyn NodeRuntime>) -> Self {
        let typescript_install_path = Mutex::new(None);
        Self {
            node,
            typescript_install_path,
        }
    }
}
#[async_trait(?Send)]
impl super::LspAdapter for VueLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("vue-language-server".into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(VueLspVersion {
            // We hardcode the version to 1.8 since we do not support @vue/language-server 2.0 yet.
            vue_version: "1.8".to_string(),
            ts_version: self.node.npm_package_latest_version("typescript").await?,
        }) as Box<_>)
    }
    async fn initialization_options(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        let typescript_sdk_path = self.typescript_install_path.lock();
        let typescript_sdk_path = typescript_sdk_path
            .as_ref()
            .expect("initialization_options called without a container_dir for typescript");

        Ok(Some(serde_json::json!({
            "typescript": {
                "tsdk": typescript_sdk_path
            }
        })))
    }
    fn code_action_kinds(&self) -> Option<Vec<CodeActionKind>> {
        // REFACTOR is explicitly disabled, as vue-lsp does not adhere to LSP protocol for code actions with these - it
        // sends back a CodeAction with neither `command` nor `edits` fields set, which is against the spec.
        Some(vec![
            CodeActionKind::EMPTY,
            CodeActionKind::QUICKFIX,
            CodeActionKind::REFACTOR_REWRITE,
        ])
    }
    async fn fetch_server_binary(
        &self,
        latest_version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let latest_version = latest_version.downcast::<VueLspVersion>().unwrap();
        let server_path = container_dir.join(Self::SERVER_PATH);
        let ts_path = container_dir.join(Self::TYPESCRIPT_PATH);

        let vue_package_name = "@vue/language-server";
        let should_install_vue_language_server = self
            .node
            .should_install_npm_package(
                vue_package_name,
                &server_path,
                &container_dir,
                &latest_version.vue_version,
            )
            .await;

        if should_install_vue_language_server {
            self.node
                .npm_install_packages(
                    &container_dir,
                    &[(vue_package_name, latest_version.vue_version.as_str())],
                )
                .await?;
        }
        ensure!(
            fs::metadata(&server_path).await.is_ok(),
            "@vue/language-server package installation failed"
        );

        let ts_package_name = "typescript";
        let should_install_ts_language_server = self
            .node
            .should_install_npm_package(
                ts_package_name,
                &server_path,
                &container_dir,
                &latest_version.ts_version,
            )
            .await;

        if should_install_ts_language_server {
            self.node
                .npm_install_packages(
                    &container_dir,
                    &[(ts_package_name, latest_version.ts_version.as_str())],
                )
                .await?;
        }

        ensure!(
            fs::metadata(&ts_path).await.is_ok(),
            "typescript for Vue package installation failed"
        );
        *self.typescript_install_path.lock() = Some(ts_path);
        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            env: None,
            arguments: vue_server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let (server, ts_path) = get_cached_server_binary(container_dir, self.node.clone()).await?;
        *self.typescript_install_path.lock() = Some(ts_path);
        Some(server)
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        let (server, ts_path) = get_cached_server_binary(container_dir, self.node.clone())
            .await
            .map(|(mut binary, ts_path)| {
                binary.arguments = vec!["--help".into()];
                (binary, ts_path)
            })?;
        *self.typescript_install_path.lock() = Some(ts_path);
        Some(server)
    }

    async fn label_for_completion(
        &self,
        item: &lsp::CompletionItem,
        language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        use lsp::CompletionItemKind as Kind;
        let len = item.label.len();
        let grammar = language.grammar()?;
        let highlight_id = match item.kind? {
            Kind::CLASS | Kind::INTERFACE => grammar.highlight_id_for_name("type"),
            Kind::CONSTRUCTOR => grammar.highlight_id_for_name("type"),
            Kind::CONSTANT => grammar.highlight_id_for_name("constant"),
            Kind::FUNCTION | Kind::METHOD => grammar.highlight_id_for_name("function"),
            Kind::PROPERTY | Kind::FIELD => grammar.highlight_id_for_name("tag"),
            Kind::VARIABLE => grammar.highlight_id_for_name("type"),
            Kind::KEYWORD => grammar.highlight_id_for_name("keyword"),
            Kind::VALUE => grammar.highlight_id_for_name("tag"),
            _ => None,
        }?;

        let text = match &item.detail {
            Some(detail) => format!("{} {}", item.label, detail),
            None => item.label.clone(),
        };

        Some(language::CodeLabel {
            text,
            runs: vec![(0..len, highlight_id)],
            filter_range: 0..len,
        })
    }
}

fn vue_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

type TypescriptPath = PathBuf;
async fn get_cached_server_binary(
    container_dir: PathBuf,
    node: Arc<dyn NodeRuntime>,
) -> Option<(LanguageServerBinary, TypescriptPath)> {
    maybe!(async {
        let mut last_version_dir = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            let entry = entry?;
            if entry.file_type().await?.is_dir() {
                last_version_dir = Some(entry.path());
            }
        }
        let last_version_dir = last_version_dir.ok_or_else(|| anyhow!("no cached binary"))?;
        let server_path = last_version_dir.join(VueLspAdapter::SERVER_PATH);
        let typescript_path = last_version_dir.join(VueLspAdapter::TYPESCRIPT_PATH);
        if server_path.exists() && typescript_path.exists() {
            Ok((
                LanguageServerBinary {
                    path: node.binary_path().await?,
                    env: None,
                    arguments: vue_server_binary_arguments(&server_path),
                },
                typescript_path,
            ))
        } else {
            Err(anyhow!(
                "missing executable in directory {:?}",
                last_version_dir
            ))
        }
    })
    .await
    .log_err()
}
