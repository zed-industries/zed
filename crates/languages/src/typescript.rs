use anyhow::{Context as _, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use collections::HashMap;
use gpui::AsyncApp;
use http_client::github::{AssetKind, GitHubLspBinaryVersion, build_asset_url};
use language::{LanguageToolchainStore, LspAdapter, LspAdapterDelegate};
use lsp::{CodeActionKind, LanguageServerBinary, LanguageServerName};
use node_runtime::NodeRuntime;
use project::ContextProviderWithTasks;
use project::{Fs, lsp_store::language_server_settings};
use serde_json::{Value, json};
use smol::{fs, io::BufReader, stream::StreamExt};
use std::{
    any::Any,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use task::{TaskTemplate, TaskTemplates, VariableName};
use util::archive::extract_zip;
use util::{ResultExt, fs::remove_matching, maybe};

pub(super) fn typescript_task_context() -> ContextProviderWithTasks {
    ContextProviderWithTasks::new(TaskTemplates(vec![
        TaskTemplate {
            label: "jest file test".to_owned(),
            command: "npx jest".to_owned(),
            args: vec![VariableName::File.template_value()],
            ..TaskTemplate::default()
        },
        TaskTemplate {
            label: "jest test $ZED_SYMBOL".to_owned(),
            command: "npx jest".to_owned(),
            args: vec![
                "--testNamePattern".into(),
                format!("\"{}\"", VariableName::Symbol.template_value()),
                VariableName::File.template_value(),
            ],
            tags: vec!["ts-test".into(), "js-test".into(), "tsx-test".into()],
            ..TaskTemplate::default()
        },
        TaskTemplate {
            label: "execute selection $ZED_SELECTED_TEXT".to_owned(),
            command: "node".to_owned(),
            args: vec![
                "-e".into(),
                format!("\"{}\"", VariableName::SelectedText.template_value()),
            ],
            ..TaskTemplate::default()
        },
    ]))
}

fn typescript_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

fn eslint_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![
        "--max-old-space-size=8192".into(),
        server_path.into(),
        "--stdio".into(),
    ]
}

pub struct TypeScriptLspAdapter {
    node: NodeRuntime,
}

impl TypeScriptLspAdapter {
    const OLD_SERVER_PATH: &'static str = "node_modules/typescript-language-server/lib/cli.js";
    const NEW_SERVER_PATH: &'static str = "node_modules/typescript-language-server/lib/cli.mjs";
    const SERVER_NAME: LanguageServerName =
        LanguageServerName::new_static("typescript-language-server");
    const PACKAGE_NAME: &str = "typescript";
    pub fn new(node: NodeRuntime) -> Self {
        TypeScriptLspAdapter { node }
    }
    async fn tsdk_path(fs: &dyn Fs, adapter: &Arc<dyn LspAdapterDelegate>) -> Option<&'static str> {
        let is_yarn = adapter
            .read_text_file(PathBuf::from(".yarn/sdks/typescript/lib/typescript.js"))
            .await
            .is_ok();

        let tsdk_path = if is_yarn {
            ".yarn/sdks/typescript/lib"
        } else {
            "node_modules/typescript/lib"
        };

        if fs
            .is_dir(&adapter.worktree_root_path().join(tsdk_path))
            .await
        {
            Some(tsdk_path)
        } else {
            None
        }
    }
}

struct TypeScriptVersions {
    typescript_version: String,
    server_version: String,
}

#[async_trait(?Send)]
impl LspAdapter for TypeScriptLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME.clone()
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

    async fn check_if_version_installed(
        &self,
        version: &(dyn 'static + Send + Any),
        container_dir: &PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let version = version.downcast_ref::<TypeScriptVersions>().unwrap();
        let server_path = container_dir.join(Self::NEW_SERVER_PATH);

        let should_install_language_server = self
            .node
            .should_install_npm_package(
                Self::PACKAGE_NAME,
                &server_path,
                &container_dir,
                version.typescript_version.as_str(),
            )
            .await;

        if should_install_language_server {
            None
        } else {
            Some(LanguageServerBinary {
                path: self.node.binary_path().await.ok()?,
                env: None,
                arguments: typescript_server_binary_arguments(&server_path),
            })
        }
    }

    async fn fetch_server_binary(
        &self,
        latest_version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let latest_version = latest_version.downcast::<TypeScriptVersions>().unwrap();
        let server_path = container_dir.join(Self::NEW_SERVER_PATH);

        self.node
            .npm_install_packages(
                &container_dir,
                &[
                    (
                        Self::PACKAGE_NAME,
                        latest_version.typescript_version.as_str(),
                    ),
                    (
                        "typescript-language-server",
                        latest_version.server_version.as_str(),
                    ),
                ],
            )
            .await?;

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
        get_cached_ts_server_binary(container_dir, &self.node).await
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
            Kind::CLASS | Kind::INTERFACE | Kind::ENUM => grammar.highlight_id_for_name("type"),
            Kind::CONSTRUCTOR => grammar.highlight_id_for_name("type"),
            Kind::CONSTANT => grammar.highlight_id_for_name("constant"),
            Kind::FUNCTION | Kind::METHOD => grammar.highlight_id_for_name("function"),
            Kind::PROPERTY | Kind::FIELD => grammar.highlight_id_for_name("property"),
            Kind::VARIABLE => grammar.highlight_id_for_name("variable"),
            _ => None,
        }?;

        let text = if let Some(description) = item
            .label_details
            .as_ref()
            .and_then(|label_details| label_details.description.as_ref())
        {
            format!("{} {}", item.label, description)
        } else if let Some(detail) = &item.detail {
            format!("{} {}", item.label, detail)
        } else {
            item.label.clone()
        };

        Some(language::CodeLabel {
            text,
            runs: vec![(0..len, highlight_id)],
            filter_range: 0..len,
        })
    }

    async fn initialization_options(
        self: Arc<Self>,
        fs: &dyn Fs,
        adapter: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        let tsdk_path = Self::tsdk_path(fs, adapter).await;
        Ok(Some(json!({
            "provideFormatter": true,
            "hostInfo": "zed",
            "tsserver": {
                "path": tsdk_path,
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
        })))
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        _: &dyn Fs,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Arc<dyn LanguageToolchainStore>,
        cx: &mut AsyncApp,
    ) -> Result<Value> {
        let override_options = cx.update(|cx| {
            language_server_settings(delegate.as_ref(), &Self::SERVER_NAME, cx)
                .and_then(|s| s.settings.clone())
        })?;
        if let Some(options) = override_options {
            return Ok(options);
        }
        Ok(json!({
            "completions": {
              "completeFunctionCalls": true
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
    node: &NodeRuntime,
) -> Option<LanguageServerBinary> {
    maybe!(async {
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
            anyhow::bail!("missing executable in directory {container_dir:?}")
        }
    })
    .await
    .log_err()
}

pub struct EsLintLspAdapter {
    node: NodeRuntime,
}

impl EsLintLspAdapter {
    const CURRENT_VERSION: &'static str = "2.4.4";
    const CURRENT_VERSION_TAG_NAME: &'static str = "release/2.4.4";

    #[cfg(not(windows))]
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::TarGz;
    #[cfg(windows)]
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::Zip;

    const SERVER_PATH: &'static str = "vscode-eslint/server/out/eslintServer.js";
    const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("eslint");

    const FLAT_CONFIG_FILE_NAMES: &'static [&'static str] = &[
        "eslint.config.js",
        "eslint.config.mjs",
        "eslint.config.cjs",
        "eslint.config.ts",
        "eslint.config.cts",
        "eslint.config.mts",
    ];

    pub fn new(node: NodeRuntime) -> Self {
        EsLintLspAdapter { node }
    }

    fn build_destination_path(container_dir: &Path) -> PathBuf {
        container_dir.join(format!("vscode-eslint-{}", Self::CURRENT_VERSION))
    }
}

#[async_trait(?Send)]
impl LspAdapter for EsLintLspAdapter {
    fn code_action_kinds(&self) -> Option<Vec<CodeActionKind>> {
        Some(vec![
            CodeActionKind::QUICKFIX,
            CodeActionKind::new("source.fixAll.eslint"),
        ])
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        _: &dyn Fs,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Arc<dyn LanguageToolchainStore>,
        cx: &mut AsyncApp,
    ) -> Result<Value> {
        let workspace_root = delegate.worktree_root_path();

        let eslint_user_settings = cx.update(|cx| {
            language_server_settings(delegate.as_ref(), &Self::SERVER_NAME, cx)
                .and_then(|s| s.settings.clone())
                .unwrap_or_default()
        })?;

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

        let working_directory = eslint_user_settings
            .get("workingDirectory")
            .cloned()
            .unwrap_or_else(|| json!({"mode": "auto"}));

        let problems = eslint_user_settings
            .get("problems")
            .cloned()
            .unwrap_or_else(|| json!({}));

        let rules_customizations = eslint_user_settings
            .get("rulesCustomizations")
            .cloned()
            .unwrap_or_else(|| json!([]));

        let node_path = eslint_user_settings.get("nodePath").unwrap_or(&Value::Null);
        let use_flat_config = Self::FLAT_CONFIG_FILE_NAMES
            .iter()
            .any(|file| workspace_root.join(file).is_file());

        Ok(json!({
            "": {
                "validate": "on",
                "rulesCustomizations": rules_customizations,
                "run": "onType",
                "nodePath": node_path,
                "workingDirectory": working_directory,
                "workspaceFolder": {
                    "uri": workspace_root,
                    "name": workspace_root.file_name()
                        .unwrap_or(workspace_root.as_os_str()),
                },
                "problems": problems,
                "codeActionOnSave": code_action_on_save,
                "codeAction": {
                    "disableRuleComment": {
                        "enable": true,
                        "location": "separateLine",
                    },
                    "showDocumentation": {
                        "enable": true
                    }
                },
                "experimental": {
                    "useFlatConfig": use_flat_config,
                },
            }
        }))
    }

    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME.clone()
    }

    async fn fetch_latest_server_version(
        &self,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let url = build_asset_url(
            "zed-industries/vscode-eslint",
            Self::CURRENT_VERSION_TAG_NAME,
            Self::GITHUB_ASSET_KIND,
        )?;

        Ok(Box::new(GitHubLspBinaryVersion {
            name: Self::CURRENT_VERSION.into(),
            url,
        }))
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
        let destination_path = Self::build_destination_path(&container_dir);
        let server_path = destination_path.join(Self::SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            remove_matching(&container_dir, |entry| entry != destination_path).await;

            let mut response = delegate
                .http_client()
                .get(&version.url, Default::default(), true)
                .await
                .context("downloading release")?;
            match Self::GITHUB_ASSET_KIND {
                AssetKind::TarGz => {
                    let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
                    let archive = Archive::new(decompressed_bytes);
                    archive.unpack(&destination_path).await.with_context(|| {
                        format!("extracting {} to {:?}", version.url, destination_path)
                    })?;
                }
                AssetKind::Gz => {
                    let mut decompressed_bytes =
                        GzipDecoder::new(BufReader::new(response.body_mut()));
                    let mut file =
                        fs::File::create(&destination_path).await.with_context(|| {
                            format!(
                                "creating a file {:?} for a download from {}",
                                destination_path, version.url,
                            )
                        })?;
                    futures::io::copy(&mut decompressed_bytes, &mut file)
                        .await
                        .with_context(|| {
                            format!("extracting {} to {:?}", version.url, destination_path)
                        })?;
                }
                AssetKind::Zip => {
                    extract_zip(&destination_path, BufReader::new(response.body_mut()))
                        .await
                        .with_context(|| {
                            format!("unzipping {} to {:?}", version.url, destination_path)
                        })?;
                }
            }

            let mut dir = fs::read_dir(&destination_path).await?;
            let first = dir.next().await.context("missing first file")??;
            let repo_root = destination_path.join("vscode-eslint");
            fs::rename(first.path(), &repo_root).await?;

            #[cfg(target_os = "windows")]
            {
                handle_symlink(
                    repo_root.join("$shared"),
                    repo_root.join("client").join("src").join("shared"),
                )
                .await?;
                handle_symlink(
                    repo_root.join("$shared"),
                    repo_root.join("server").join("src").join("shared"),
                )
                .await?;
            }

            self.node
                .run_npm_subcommand(&repo_root, "install", &[])
                .await?;

            self.node
                .run_npm_subcommand(&repo_root, "run-script", &["compile"])
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
        let server_path =
            Self::build_destination_path(&container_dir).join(EsLintLspAdapter::SERVER_PATH);
        Some(LanguageServerBinary {
            path: self.node.binary_path().await.ok()?,
            env: None,
            arguments: eslint_server_binary_arguments(&server_path),
        })
    }
}

#[cfg(target_os = "windows")]
async fn handle_symlink(src_dir: PathBuf, dest_dir: PathBuf) -> Result<()> {
    anyhow::ensure!(
        fs::metadata(&src_dir).await.is_ok(),
        "Directory {src_dir:?} is not present"
    );
    if fs::metadata(&dest_dir).await.is_ok() {
        fs::remove_file(&dest_dir).await?;
    }
    fs::create_dir_all(&dest_dir).await?;
    let mut entries = fs::read_dir(&src_dir).await?;
    while let Some(entry) = entries.try_next().await? {
        let entry_path = entry.path();
        let entry_name = entry.file_name();
        let dest_path = dest_dir.join(&entry_name);
        fs::copy(&entry_path, &dest_path).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use gpui::{AppContext as _, TestAppContext};
    use unindent::Unindent;

    #[gpui::test]
    async fn test_outline(cx: &mut TestAppContext) {
        let language = crate::language(
            "typescript",
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        );

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

        let buffer = cx.new(|cx| language::Buffer::local(text, cx).with_language(language, cx));
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
