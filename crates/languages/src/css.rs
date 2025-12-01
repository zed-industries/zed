use anyhow::Result;
use async_trait::async_trait;
use gpui::AsyncApp;
use language::{LspAdapter, LspAdapterDelegate, LspInstaller, Toolchain};
use lsp::{LanguageServerBinary, LanguageServerName, Uri};
use node_runtime::{NodeRuntime, VersionStrategy};
use project::lsp_store::language_server_settings;
use serde_json::json;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{ResultExt, maybe, merge_json_value_into};

const SERVER_PATH: &str =
    "node_modules/vscode-langservers-extracted/bin/vscode-css-language-server";

fn server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct CssLspAdapter {
    node: NodeRuntime,
}

impl CssLspAdapter {
    const PACKAGE_NAME: &str = "vscode-langservers-extracted";
    pub fn new(node: NodeRuntime) -> Self {
        CssLspAdapter { node }
    }
}

impl LspInstaller for CssLspAdapter {
    type BinaryVersion = String;

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
        _: bool,
        _: &mut AsyncApp,
    ) -> Result<String> {
        self.node
            .npm_package_latest_version("vscode-langservers-extracted")
            .await
    }

    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        _: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        let path = delegate
            .which("vscode-css-language-server".as_ref())
            .await?;
        let env = delegate.shell_env().await;

        Some(LanguageServerBinary {
            path,
            env: Some(env),
            arguments: vec!["--stdio".into()],
        })
    }

    async fn fetch_server_binary(
        &self,
        latest_version: String,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let server_path = container_dir.join(SERVER_PATH);

        self.node
            .npm_install_packages(
                &container_dir,
                &[(Self::PACKAGE_NAME, latest_version.as_str())],
            )
            .await?;

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            env: None,
            arguments: server_binary_arguments(&server_path),
        })
    }

    async fn check_if_version_installed(
        &self,
        version: &String,
        container_dir: &PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let server_path = container_dir.join(SERVER_PATH);

        let should_install_language_server = self
            .node
            .should_install_npm_package(
                Self::PACKAGE_NAME,
                &server_path,
                container_dir,
                VersionStrategy::Latest(version),
            )
            .await;

        if should_install_language_server {
            None
        } else {
            Some(LanguageServerBinary {
                path: self.node.binary_path().await.ok()?,
                env: None,
                arguments: server_binary_arguments(&server_path),
            })
        }
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir, &self.node).await
    }
}

#[async_trait(?Send)]
impl LspAdapter for CssLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("vscode-css-language-server".into())
    }

    async fn initialization_options(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(json!({
            "provideFormatter": true
        })))
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Option<Toolchain>,
        _: Option<Uri>,
        cx: &mut AsyncApp,
    ) -> Result<serde_json::Value> {
        let mut default_config = json!({
            "css": {
                "lint": {}
            },
            "less": {
                "lint": {}
            },
            "scss": {
                "lint": {}
            }
        });

        let project_options = cx.update(|cx| {
            language_server_settings(delegate.as_ref(), &self.name(), cx)
                .and_then(|s| s.settings.clone())
        })?;

        if let Some(override_options) = project_options {
            merge_json_value_into(override_options, &mut default_config);
        }

        Ok(default_config)
    }
}

async fn get_cached_server_binary(
    container_dir: PathBuf,
    node: &NodeRuntime,
) -> Option<LanguageServerBinary> {
    maybe!(async {
        let server_path = container_dir.join(SERVER_PATH);
        anyhow::ensure!(
            server_path.exists(),
            "missing executable in directory {server_path:?}"
        );
        Ok(LanguageServerBinary {
            path: node.binary_path().await?,
            env: None,
            arguments: server_binary_arguments(&server_path),
        })
    })
    .await
    .log_err()
}

#[cfg(test)]
mod tests {
    use gpui::{AppContext as _, TestAppContext};
    use unindent::Unindent;

    #[gpui::test]
    async fn test_outline(cx: &mut TestAppContext) {
        let language = crate::language("css", tree_sitter_css::LANGUAGE.into());

        let text = r#"
            /* Import statement */
            @import './fonts.css';

            /* multiline list of selectors with nesting */
            .test-class,
            div {
                .nested-class {
                    color: red;
                }
            }

            /* descendant selectors */
            .test .descendant {}

            /* pseudo */
            .test:not(:hover) {}

            /* media queries */
            @media screen and (min-width: 3000px) {
                .desktop-class {}
            }
        "#
        .unindent();

        let buffer = cx.new(|cx| language::Buffer::local(text, cx).with_language(language, cx));
        let outline = buffer.read_with(cx, |buffer, _| buffer.snapshot().outline(None));
        assert_eq!(
            outline
                .items
                .iter()
                .map(|item| (item.text.as_str(), item.depth))
                .collect::<Vec<_>>(),
            &[
                ("@import './fonts.css'", 0),
                (".test-class, div", 0),
                (".nested-class", 1),
                (".test .descendant", 0),
                (".test:not(:hover)", 0),
                ("@media screen and (min-width: 3000px)", 0),
                (".desktop-class", 1),
            ]
        );
    }
}
