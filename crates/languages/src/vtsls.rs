use anyhow::{anyhow, Result};
use async_trait::async_trait;
use collections::HashMap;
use gpui::AsyncAppContext;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::{CodeActionKind, LanguageServerBinary};
use node_runtime::NodeRuntime;
use project::project_settings::{BinarySettings, ProjectSettings};
use serde_json::{json, Value};
use settings::Settings;
use std::{
    any::Any,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{maybe, ResultExt};

fn typescript_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct VtslsLspAdapter {
    node: Arc<dyn NodeRuntime>,
}

impl VtslsLspAdapter {
    const SERVER_PATH: &'static str = "node_modules/@vtsls/language-server/bin/vtsls.js";

    pub fn new(node: Arc<dyn NodeRuntime>) -> Self {
        VtslsLspAdapter { node }
    }
    async fn tsdk_path(adapter: &Arc<dyn LspAdapterDelegate>) -> &'static str {
        let is_yarn = adapter
            .read_text_file(PathBuf::from(".yarn/sdks/typescript/lib/typescript.js"))
            .await
            .is_ok();

        if is_yarn {
            ".yarn/sdks/typescript/lib"
        } else {
            "node_modules/typescript/lib"
        }
    }
}

struct TypeScriptVersions {
    typescript_version: String,
    server_version: String,
}

const SERVER_NAME: &'static str = "vtsls";
#[async_trait(?Send)]
impl LspAdapter for VtslsLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName(SERVER_NAME.into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(TypeScriptVersions {
            typescript_version: self.node.npm_package_latest_version("typescript").await?,
            server_version: self
                .node
                .npm_package_latest_version("@vtsls/language-server")
                .await?,
        }) as Box<_>)
    }

    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        cx: &AsyncAppContext,
    ) -> Option<LanguageServerBinary> {
        let configured_binary = cx.update(|cx| {
            ProjectSettings::get_global(cx)
                .lsp
                .get(SERVER_NAME)
                .and_then(|s| s.binary.clone())
        });

        match configured_binary {
            Ok(Some(BinarySettings {
                path: Some(path),
                arguments,
                ..
            })) => Some(LanguageServerBinary {
                path: path.into(),
                arguments: arguments
                    .unwrap_or_default()
                    .iter()
                    .map(|arg| arg.into())
                    .collect(),
                env: None,
            }),
            Ok(Some(BinarySettings {
                path_lookup: Some(false),
                ..
            })) => None,
            _ => {
                let env = delegate.shell_env().await;
                let path = delegate.which(SERVER_NAME.as_ref()).await?;
                Some(LanguageServerBinary {
                    path: path.clone(),
                    arguments: typescript_server_binary_arguments(&path),
                    env: Some(env),
                })
            }
        }
    }

    async fn fetch_server_binary(
        &self,
        latest_version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let latest_version = latest_version.downcast::<TypeScriptVersions>().unwrap();
        let server_path = container_dir.join(Self::SERVER_PATH);
        let package_name = "typescript";

        let should_install_language_server = self
            .node
            .should_install_npm_package(
                package_name,
                &server_path,
                &container_dir,
                latest_version.typescript_version.as_str(),
            )
            .await;

        if should_install_language_server {
            self.node
                .npm_install_packages(
                    &container_dir,
                    &[
                        (package_name, latest_version.typescript_version.as_str()),
                        (
                            "@vtsls/language-server",
                            latest_version.server_version.as_str(),
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
            Kind::CLASS | Kind::INTERFACE | Kind::ENUM => grammar.highlight_id_for_name("type"),
            Kind::CONSTRUCTOR => grammar.highlight_id_for_name("type"),
            Kind::CONSTANT => grammar.highlight_id_for_name("constant"),
            Kind::FUNCTION | Kind::METHOD => grammar.highlight_id_for_name("function"),
            Kind::PROPERTY | Kind::FIELD => grammar.highlight_id_for_name("property"),
            Kind::VARIABLE => grammar.highlight_id_for_name("variable"),
            _ => None,
        }?;

        let one_line = |s: &str| s.replace("    ", "").replace('\n', " ");

        let text = if let Some(description) = item
            .label_details
            .as_ref()
            .and_then(|label_details| label_details.description.as_ref())
        {
            format!("{} {}", item.label, one_line(description))
        } else if let Some(detail) = &item.detail {
            format!("{} {}", item.label, one_line(detail))
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
        adapter: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        let tsdk_path = Self::tsdk_path(&adapter).await;
        Ok(Some(json!({
            "typescript": {
                "tsdk": tsdk_path,
                "suggest": {
                    "completeFunctionCalls": true
                },
                "inlayHints": {
                    "parameterNames": {
                        "enabled": "all",
                        "suppressWhenArgumentMatchesName": false,
                    },
                    "parameterTypes": {
                        "enabled": true
                    },
                    "variableTypes": {
                        "enabled": true,
                        "suppressWhenTypeMatchesName": false,
                    },
                    "propertyDeclarationTypes": {
                        "enabled": true,
                    },
                    "functionLikeReturnTypes": {
                        "enabled": true,
                    },
                    "enumMemberValues": {
                        "enabled": true,
                    }
                }
            },
            "javascript": {
                "suggest": {
                    "completeFunctionCalls": true
                }
            },
            "vtsls": {
                "experimental": {
                    "completion": {
                        "enableServerSideFuzzyMatch": true,
                        "entriesLimit": 5000,
                    }
                },
               "autoUseWorkspaceTsdk": true
            }
        })))
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        adapter: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<Value> {
        let override_options = cx.update(|cx| {
            ProjectSettings::get_global(cx)
                .lsp
                .get(SERVER_NAME)
                .and_then(|s| s.initialization_options.clone())
        })?;
        if let Some(options) = override_options {
            return Ok(options);
        }
        self.initialization_options(adapter)
            .await
            .map(|o| o.unwrap())
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
    maybe!(async {
        let server_path = container_dir.join(VtslsLspAdapter::SERVER_PATH);
        if server_path.exists() {
            Ok(LanguageServerBinary {
                path: node.binary_path().await?,
                env: None,
                arguments: typescript_server_binary_arguments(&server_path),
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
