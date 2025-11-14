use anyhow::{Result, ensure};
use async_trait::async_trait;
use gpui::AsyncApp;
use language::{LanguageName, LspAdapter, LspAdapterDelegate, LspInstaller, Toolchain};
use lsp::{LanguageServerBinary, LanguageServerName};
use node_runtime::{NodeRuntime, VersionStrategy};
use project::lsp_store::language_server_settings;
use serde_json::{Value, json};
use smol::{fs, io::AsyncWriteExt};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{ResultExt, maybe, merge_json_value_into};

const SERVER_SCRIPT_NAME: &str = "markdown-lsp-server.cjs";
const SERVER_SCRIPT_SOURCE: &str = include_str!("markdown/markdown-lsp-server.cjs");

fn server_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), OsString::from("--stdio")]
}

pub struct MarkdownLspAdapter {
    node: NodeRuntime,
}

impl MarkdownLspAdapter {
    pub fn new(node: NodeRuntime) -> Self {
        Self { node }
    }
    const SERVER_NAME: LanguageServerName =
        LanguageServerName::new_static("vscode-markdown-languageservice");
    const PACKAGE_NAME: &'static str = "vscode-markdown-languageservice";

    fn script_path(container_dir: &Path) -> PathBuf {
        container_dir.join(SERVER_SCRIPT_NAME)
    }
}

impl LspInstaller for MarkdownLspAdapter {
    type BinaryVersion = String;

    async fn fetch_latest_server_version(
        &self,
        _delegate: &dyn LspAdapterDelegate,
        _allow_pre: bool,
        _cx: &mut AsyncApp,
    ) -> Result<String> {
        self.node
            .npm_package_latest_version(Self::PACKAGE_NAME)
            .await
    }

    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        _toolchain: Option<Toolchain>,
        _cx: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        // No user-provided binary for this server; rely on managed install.
        let _ = delegate; // silence unused warning
        None
    }

    async fn fetch_server_binary(
        &self,
        latest_version: String,
        container_dir: PathBuf,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        // Ensure directory exists and write server script
        fs::create_dir_all(&container_dir).await.ok();

        self.node
            .npm_install_packages(
                &container_dir,
                &[
                    (Self::PACKAGE_NAME, latest_version.as_str()),
                    ("vscode-languageserver", "latest"),
                    ("vscode-languageserver-textdocument", "latest"),
                    ("vscode-uri", "latest"),
                    ("markdown-it", "latest"),
                ],
            )
            .await?;

        let script_path = Self::script_path(&container_dir);
        let mut file = fs::File::create(&script_path).await?;
        file.write_all(SERVER_SCRIPT_SOURCE.as_bytes()).await?;
        // Not strictly needed for Node execution as argument, but try to chmod +x.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path).await?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms).await?;
        }

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            env: None,
            arguments: server_arguments(&script_path),
        })
    }

    async fn check_if_version_installed(
        &self,
        version: &String,
        container_dir: &PathBuf,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let script_path = Self::script_path(container_dir);

        let should_install = self
            .node
            .should_install_npm_package(
                Self::PACKAGE_NAME,
                &script_path,
                container_dir,
                VersionStrategy::Latest(version),
            )
            .await;

        if should_install {
            None
        } else {
            Some(LanguageServerBinary {
                path: self.node.binary_path().await.ok()?,
                env: None,
                arguments: server_arguments(&script_path),
            })
        }
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        maybe!(async {
            let script_path = Self::script_path(&container_dir);
            ensure!(
                script_path.exists(),
                "missing executable in directory {:?}",
                container_dir
            );
            Ok(LanguageServerBinary {
                path: self.node.binary_path().await?,
                env: None,
                arguments: server_arguments(&script_path),
            })
        })
        .await
        .log_err()
    }
}

#[async_trait(?Send)]
impl LspAdapter for MarkdownLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
    }

    async fn initialization_options(
        self: Arc<Self>,
        delegate: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        let _ = delegate; // not used here
        // Default options passed to vscode-markdown-languageservice
        let options = json!({
          // Enable useful defaults
          "includeWorkspaceHeaderCompletions": true,
          "diagnostics": { "validateFileLinks": true }
        });

        Ok(Some(options))
    }

    fn language_ids(&self) -> collections::HashMap<LanguageName, String> {
        [(LanguageName::new("Markdown"), "markdown".to_string())]
            .into_iter()
            .collect()
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _toolchain: Option<Toolchain>,
        cx: &mut AsyncApp,
    ) -> Result<Value> {
        // Defaults (same shape as initializationOptions), can be overridden by user settings
        let mut options = json!({
          "includeWorkspaceHeaderCompletions": true,
          "diagnostics": { "validateFileLinks": true }
        });

        let user_overrides = cx.update(|cx| {
            language_server_settings(delegate.as_ref(), &Self::SERVER_NAME, cx)
                .and_then(|s| s.settings.clone())
        })?;

        if let Some(override_options) = user_overrides {
            merge_json_value_into(override_options, &mut options);
        }

        Ok(options)
    }
}
