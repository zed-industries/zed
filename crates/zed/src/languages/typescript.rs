use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::{future::BoxFuture, FutureExt, StreamExt};
use gpui::{AppContext, Task};
use language::{LanguageServerBinary, LanguageServerName, LspAdapter};
use lsp::CodeActionKind;
use node_runtime::NodeRuntime;
use serde_json::{json, Map, Value};
use smol::fs;
use std::{
    any::Any,
    ffi::OsString,
    future,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::http::HttpClient;
use util::ResultExt;

fn typescript_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![
        server_path.into(),
        "--stdio".into(),
        "--tsserver-path".into(),
        "node_modules/typescript/lib".into(),
    ]
}

fn eslint_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct TypeScriptLspAdapter {
    node: Arc<NodeRuntime>,
}

impl TypeScriptLspAdapter {
    const OLD_SERVER_PATH: &'static str = "node_modules/typescript-language-server/lib/cli.js";
    const NEW_SERVER_PATH: &'static str = "node_modules/typescript-language-server/lib/cli.mjs";

    pub fn new(node: Arc<NodeRuntime>) -> Self {
        TypeScriptLspAdapter { node }
    }
}

struct TypeScriptVersions {
    typescript_version: String,
    server_version: String,
}

#[async_trait]
impl LspAdapter for TypeScriptLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("typescript-language-server".into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: Arc<dyn HttpClient>,
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
        versions: Box<dyn 'static + Send + Any>,
        _: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> Result<LanguageServerBinary> {
        dbg!();
        let versions = versions.downcast::<TypeScriptVersions>().unwrap();
        let server_path = container_dir.join(Self::NEW_SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            self.node
                .npm_install_packages(
                    [
                        ("typescript", versions.typescript_version.as_str()),
                        (
                            "typescript-language-server",
                            versions.server_version.as_str(),
                        ),
                    ],
                    &container_dir,
                )
                .await?;
        }

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            arguments: typescript_server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(&self, container_dir: PathBuf) -> Option<LanguageServerBinary> {
        dbg!();
        (|| async move {
            let old_server_path = container_dir.join(Self::OLD_SERVER_PATH);
            let new_server_path = container_dir.join(Self::NEW_SERVER_PATH);
            if new_server_path.exists() {
                Ok(LanguageServerBinary {
                    path: self.node.binary_path().await?,
                    arguments: typescript_server_binary_arguments(&new_server_path),
                })
            } else if old_server_path.exists() {
                Ok(LanguageServerBinary {
                    path: self.node.binary_path().await?,
                    arguments: typescript_server_binary_arguments(&old_server_path),
                })
            } else {
                Err(anyhow!(
                    "missing executable in directory {:?}",
                    container_dir
                ))
            }
        })()
        .await
        .log_err()
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

    async fn initialization_options(&self) -> Option<serde_json::Value> {
        Some(json!({
            "provideFormatter": true
        }))
    }
}

pub struct EsLintLspAdapter {
    node: Arc<NodeRuntime>,
}

impl EsLintLspAdapter {
    const SERVER_PATH: &'static str =
        "node_modules/vscode-langservers-extracted/lib/eslint-language-server/eslintServer.js";

    pub fn new(node: Arc<NodeRuntime>) -> Self {
        EsLintLspAdapter { node }
    }
}

// "workspaceFolder": {
//     "name": "testing_ts",
//     "uri": "file:///Users/julia/Stuff/testing_ts"
// },
// "workingDirectory": "file:///Users/julia/Stuff/testing_ts",
// "nodePath": "/opt/homebrew/opt/node@18/bin/node",
// "experimental": {},

#[async_trait]
impl LspAdapter for EsLintLspAdapter {
    fn workspace_configuration(&self, _: &mut AppContext) -> Option<BoxFuture<'static, Value>> {
        Some(
            future::ready(json!({
                "": {
                      "validate": "on",
                      "packageManager": "npm",
                      "useESLintClass": false,
                      "experimental": {
                        "useFlatConfig": false
                      },
                      "codeActionOnSave": {
                        "mode": "all"
                      },
                      "format": false,
                      "quiet": false,
                      "onIgnoredFiles": "off",
                      "options": {},
                      "rulesCustomizations": [],
                      "run": "onType",
                      "problems": {
                        "shortenToSingleLine": false
                      },
                      "nodePath": null,
                      "workspaceFolder": {
                        "name": "testing_ts",
                        "uri": "file:///Users/julia/Stuff/testing_ts"
                      },
                      "codeAction": {
                        "disableRuleComment": {
                          "enable": true,
                          "location": "separateLine",
                          "commentStyle": "line"
                        },
                        "showDocumentation": {
                          "enable": true
                        }
                      }
                }
            }))
            .boxed(),
        )
    }

    async fn name(&self) -> LanguageServerName {
        LanguageServerName("eslint".into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: Arc<dyn HttpClient>,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(
            self.node
                .npm_package_latest_version("vscode-langservers-extracted")
                .await?,
        ))
    }

    async fn fetch_server_binary(
        &self,
        versions: Box<dyn 'static + Send + Any>,
        _: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> Result<LanguageServerBinary> {
        let version = versions.downcast::<String>().unwrap();
        let server_path = container_dir.join(Self::SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            self.node
                .npm_install_packages(
                    [("vscode-langservers-extracted", version.as_str())],
                    &container_dir,
                )
                .await?;
        }

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            arguments: eslint_server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(&self, container_dir: PathBuf) -> Option<LanguageServerBinary> {
        (|| async move {
            let server_path = container_dir.join(Self::SERVER_PATH);
            if server_path.exists() {
                Ok(LanguageServerBinary {
                    path: self.node.binary_path().await?,
                    arguments: eslint_server_binary_arguments(&server_path),
                })
            } else {
                Err(anyhow!(
                    "missing executable in directory {:?}",
                    container_dir
                ))
            }
        })()
        .await
        .log_err()
    }

    async fn label_for_completion(
        &self,
        item: &lsp::CompletionItem,
        language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        None
    }

    async fn initialization_options(&self) -> Option<serde_json::Value> {
        None
    }
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;
    use unindent::Unindent;

    #[gpui::test]
    async fn test_outline(cx: &mut TestAppContext) {
        let language = crate::languages::language(
            "typescript",
            tree_sitter_typescript::language_typescript(),
            None,
        )
        .await;

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

        let buffer =
            cx.add_model(|cx| language::Buffer::new(0, text, cx).with_language(language, cx));
        let outline = buffer.read_with(cx, |buffer, _| buffer.snapshot().outline(None).unwrap());
        assert_eq!(
            outline
                .items
                .iter()
                .map(|item| (item.text.as_str(), item.depth))
                .collect::<Vec<_>>(),
            &[
                ("function a ( )", 0),
                ("async function a2 ( )", 1),
                ("let b", 0),
                ("function getB ( )", 0),
                ("const d", 0),
            ]
        );
    }
}
