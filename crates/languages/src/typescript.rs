use anyhow::{anyhow, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use collections::HashMap;
use gpui::AppContext;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::{CodeActionKind, LanguageServerBinary};
use node_runtime::NodeRuntime;
use project::project_settings::ProjectSettings;
use serde_json::{json, Value};
use settings::Settings;
use smol::{fs, io::BufReader, stream::StreamExt};
use std::{
    any::Any,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{
    async_maybe,
    fs::remove_matching,
    github::{latest_github_release, GitHubLspBinaryVersion},
    ResultExt,
};

fn typescript_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

fn eslint_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct TypeScriptLspAdapter {
    node: Arc<dyn NodeRuntime>,
}

impl TypeScriptLspAdapter {
    const OLD_SERVER_PATH: &'static str = "node_modules/typescript-language-server/lib/cli.js";
    const NEW_SERVER_PATH: &'static str = "node_modules/typescript-language-server/lib/cli.mjs";

    pub fn new(node: Arc<dyn NodeRuntime>) -> Self {
        TypeScriptLspAdapter { node }
    }
}

struct TypeScriptVersions {
    typescript_version: String,
    server_version: String,
}

#[async_trait]
impl LspAdapter for TypeScriptLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("typescript-language-server".into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(TypeScriptVersions {
            typescript_version: self.node.npm_package_latest_version("typescript").await?,
            server_version: self
                .node
                .npm_package_latest_version("typescript-language-server")
                .await?,
        }) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<TypeScriptVersions>().unwrap();
        let server_path = container_dir.join(Self::NEW_SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            self.node
                .npm_install_packages(
                    &container_dir,
                    &[
                        ("typescript", version.typescript_version.as_str()),
                        (
                            "typescript-language-server",
                            version.server_version.as_str(),
                        ),
                    ],
                )
                .await?;
        }

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            env: None,
            arguments: typescript_server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_ts_server_binary(container_dir, &*self.node).await
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        get_cached_ts_server_binary(container_dir, &*self.node).await
    }

    fn code_action_kinds(&self) -> Option<Vec<CodeActionKind>> {
        Some(vec![
            CodeActionKind::QUICKFIX,
            CodeActionKind::REFACTOR,
            CodeActionKind::REFACTOR_EXTRACT,
            CodeActionKind::SOURCE,
        ])
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
            Kind::PROPERTY | Kind::FIELD => grammar.highlight_id_for_name("property"),
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

    fn initialization_options(&self) -> Option<serde_json::Value> {
        Some(json!({
            "provideFormatter": true,
            "tsserver": {
                "path": "node_modules/typescript/lib",
            },
            "preferences": {
                "includeInlayParameterNameHints": "all",
                "includeInlayParameterNameHintsWhenArgumentMatchesName": true,
                "includeInlayFunctionParameterTypeHints": true,
                "includeInlayVariableTypeHints": true,
                "includeInlayVariableTypeHintsWhenTypeMatchesName": true,
                "includeInlayPropertyDeclarationTypeHints": true,
                "includeInlayFunctionLikeReturnTypeHints": true,
                "includeInlayEnumMemberValueHints": true,
            }
        }))
    }

    fn language_ids(&self) -> HashMap<String, String> {
        HashMap::from_iter([
            ("TypeScript".into(), "typescript".into()),
            ("JavaScript".into(), "javascript".into()),
            ("TSX".into(), "typescriptreact".into()),
        ])
    }
}

async fn get_cached_ts_server_binary(
    container_dir: PathBuf,
    node: &dyn NodeRuntime,
) -> Option<LanguageServerBinary> {
    async_maybe!({
        let old_server_path = container_dir.join(TypeScriptLspAdapter::OLD_SERVER_PATH);
        let new_server_path = container_dir.join(TypeScriptLspAdapter::NEW_SERVER_PATH);
        if new_server_path.exists() {
            Ok(LanguageServerBinary {
                path: node.binary_path().await?,
                env: None,
                arguments: typescript_server_binary_arguments(&new_server_path),
            })
        } else if old_server_path.exists() {
            Ok(LanguageServerBinary {
                path: node.binary_path().await?,
                env: None,
                arguments: typescript_server_binary_arguments(&old_server_path),
            })
        } else {
            Err(anyhow!(
                "missing executable in directory {:?}",
                container_dir
            ))
        }
    })
    .await
    .log_err()
}

pub struct EsLintLspAdapter {
    node: Arc<dyn NodeRuntime>,
}

impl EsLintLspAdapter {
    const SERVER_PATH: &'static str = "vscode-eslint/server/out/eslintServer.js";
    const SERVER_NAME: &'static str = "eslint";

    pub fn new(node: Arc<dyn NodeRuntime>) -> Self {
        EsLintLspAdapter { node }
    }
}

#[async_trait]
impl LspAdapter for EsLintLspAdapter {
    fn workspace_configuration(&self, workspace_root: &Path, cx: &mut AppContext) -> Value {
        let eslint_user_settings = ProjectSettings::get_global(cx)
            .lsp
            .get(Self::SERVER_NAME)
            .and_then(|s| s.settings.clone())
            .unwrap_or_default();

        let mut code_action_on_save = json!({
            // We enable this, but without also configuring `code_actions_on_format`
            // in the Zed configuration, it doesn't have an effect.
            "enable": true,
        });

        if let Some(code_action_settings) = eslint_user_settings
            .get("codeActionOnSave")
            .and_then(|settings| settings.as_object())
        {
            if let Some(enable) = code_action_settings.get("enable") {
                code_action_on_save["enable"] = enable.clone();
            }
            if let Some(mode) = code_action_settings.get("mode") {
                code_action_on_save["mode"] = mode.clone();
            }
            if let Some(rules) = code_action_settings.get("rules") {
                code_action_on_save["rules"] = rules.clone();
            }
        }

        json!({
            "": {
                "validate": "on",
                "rulesCustomizations": [],
                "run": "onType",
                "nodePath": null,
                "workingDirectory": {"mode": "auto"},
                "workspaceFolder": {
                    "uri": workspace_root,
                    "name": workspace_root.file_name()
                        .unwrap_or_else(|| workspace_root.as_os_str()),
                },
                "problems": {},
                "codeActionOnSave": code_action_on_save,
                "experimental": {
                    "useFlatConfig": workspace_root.join("eslint.config.js").is_file(),
                },
            }
        })
    }

    fn name(&self) -> LanguageServerName {
        LanguageServerName(Self::SERVER_NAME.into())
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        // At the time of writing the latest vscode-eslint release was released in 2020 and requires
        // special custom LSP protocol extensions be handled to fully initialize. Download the latest
        // prerelease instead to sidestep this issue
        let release = latest_github_release(
            "microsoft/vscode-eslint",
            false,
            true,
            delegate.http_client(),
        )
        .await?;
        Ok(Box::new(GitHubLspBinaryVersion {
            name: release.tag_name,
            url: release.tarball_url,
        }))
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
        let destination_path = container_dir.join(format!("vscode-eslint-{}", version.name));
        let server_path = destination_path.join(Self::SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            remove_matching(&container_dir, |entry| entry != destination_path).await;

            let mut response = delegate
                .http_client()
                .get(&version.url, Default::default(), true)
                .await
                .map_err(|err| anyhow!("error downloading release: {}", err))?;
            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let archive = Archive::new(decompressed_bytes);
            archive.unpack(&destination_path).await?;

            let mut dir = fs::read_dir(&destination_path).await?;
            let first = dir.next().await.ok_or(anyhow!("missing first file"))??;
            let repo_root = destination_path.join("vscode-eslint");
            fs::rename(first.path(), &repo_root).await?;

            self.node
                .run_npm_subcommand(Some(&repo_root), "install", &[])
                .await?;

            self.node
                .run_npm_subcommand(Some(&repo_root), "run-script", &["compile"])
                .await?;
        }

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            env: None,
            arguments: eslint_server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_eslint_server_binary(container_dir, &*self.node).await
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        get_cached_eslint_server_binary(container_dir, &*self.node).await
    }

    async fn label_for_completion(
        &self,
        _item: &lsp::CompletionItem,
        _language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        None
    }

    fn initialization_options(&self) -> Option<serde_json::Value> {
        None
    }
}

async fn get_cached_eslint_server_binary(
    container_dir: PathBuf,
    node: &dyn NodeRuntime,
) -> Option<LanguageServerBinary> {
    async_maybe!({
        // This is unfortunate but we don't know what the version is to build a path directly
        let mut dir = fs::read_dir(&container_dir).await?;
        let first = dir.next().await.ok_or(anyhow!("missing first file"))??;
        if !first.file_type().await?.is_dir() {
            return Err(anyhow!("First entry is not a directory"));
        }
        let server_path = first.path().join(EsLintLspAdapter::SERVER_PATH);

        Ok(LanguageServerBinary {
            path: node.binary_path().await?,
            env: None,
            arguments: eslint_server_binary_arguments(&server_path),
        })
    })
    .await
    .log_err()
}

#[cfg(test)]
mod tests {
    use gpui::{Context, TestAppContext};
    use text::BufferId;
    use unindent::Unindent;

    #[gpui::test]
    async fn test_outline(cx: &mut TestAppContext) {
        let language = crate::language("typescript", tree_sitter_typescript::language_typescript());

        let text = r#"
            function a() {
              // local variables are omitted
              let a1 = 1;
              // all functions are included
              async function a2() {}
            }
            // top-level variables are included
            let b: C
            function getB() {}
            // exported variables are included
            export const d = e;
        "#
        .unindent();

        let buffer = cx.new_model(|cx| {
            language::Buffer::new(0, BufferId::new(cx.entity_id().as_u64()).unwrap(), text)
                .with_language(language, cx)
        });
        let outline = buffer.update(cx, |buffer, _| buffer.snapshot().outline(None).unwrap());
        assert_eq!(
            outline
                .items
                .iter()
                .map(|item| (item.text.as_str(), item.depth))
                .collect::<Vec<_>>(),
            &[
                ("function a()", 0),
                ("async function a2()", 1),
                ("let b", 0),
                ("function getB()", 0),
                ("const d", 0),
            ]
        );
    }
}
