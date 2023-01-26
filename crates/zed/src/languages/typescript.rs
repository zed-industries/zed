use super::installation::{npm_install_packages, npm_package_latest_version};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use client::http::HttpClient;
use futures::StreamExt;
use language::{LanguageServerName, LspAdapter};
use serde_json::json;
use smol::fs;
use std::{any::Any, path::PathBuf, sync::Arc};
use util::ResultExt;

pub struct TypeScriptLspAdapter;

impl TypeScriptLspAdapter {
    const OLD_BIN_PATH: &'static str = "node_modules/typescript-language-server/lib/cli.js";
    const NEW_BIN_PATH: &'static str = "node_modules/typescript-language-server/lib/cli.mjs";
}

struct Versions {
    typescript_version: String,
    server_version: String,
}

#[async_trait]
impl LspAdapter for TypeScriptLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("typescript-language-server".into())
    }

    async fn server_args(&self) -> Vec<String> {
        ["--stdio", "--tsserver-path", "node_modules/typescript/lib"]
            .into_iter()
            .map(str::to_string)
            .collect()
    }

    async fn fetch_latest_server_version(
        &self,
        _: Arc<dyn HttpClient>,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(Versions {
            typescript_version: npm_package_latest_version("typescript").await?,
            server_version: npm_package_latest_version("typescript-language-server").await?,
        }) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        versions: Box<dyn 'static + Send + Any>,
        _: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> Result<PathBuf> {
        let versions = versions.downcast::<Versions>().unwrap();
        let version_dir = container_dir.join(&format!(
            "typescript-{}:server-{}",
            versions.typescript_version, versions.server_version
        ));
        fs::create_dir_all(&version_dir)
            .await
            .context("failed to create version directory")?;
        let binary_path = version_dir.join(Self::NEW_BIN_PATH);

        if fs::metadata(&binary_path).await.is_err() {
            npm_install_packages(
                [
                    ("typescript", versions.typescript_version.as_str()),
                    (
                        "typescript-language-server",
                        versions.server_version.as_str(),
                    ),
                ],
                &version_dir,
            )
            .await?;

            if let Some(mut entries) = fs::read_dir(&container_dir).await.log_err() {
                while let Some(entry) = entries.next().await {
                    if let Some(entry) = entry.log_err() {
                        let entry_path = entry.path();
                        if entry_path.as_path() != version_dir {
                            fs::remove_dir_all(&entry_path).await.log_err();
                        }
                    }
                }
            }
        }

        Ok(binary_path)
    }

    async fn cached_server_binary(&self, container_dir: PathBuf) -> Option<PathBuf> {
        (|| async move {
            let mut last_version_dir = None;
            let mut entries = fs::read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                let entry = entry?;
                if entry.file_type().await?.is_dir() {
                    last_version_dir = Some(entry.path());
                }
            }
            let last_version_dir = last_version_dir.ok_or_else(|| anyhow!("no cached binary"))?;
            let old_bin_path = last_version_dir.join(Self::OLD_BIN_PATH);
            let new_bin_path = last_version_dir.join(Self::NEW_BIN_PATH);
            if new_bin_path.exists() {
                Ok(new_bin_path)
            } else if old_bin_path.exists() {
                Ok(old_bin_path)
            } else {
                Err(anyhow!(
                    "missing executable in directory {:?}",
                    last_version_dir
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
