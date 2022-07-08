use super::installation::latest_github_release;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use client::http::HttpClient;
use futures::StreamExt;
pub use language::*;
use lazy_static::lazy_static;
use regex::Regex;
use smol::{fs, process};
use std::{any::Any, ops::Range, path::PathBuf, str, sync::Arc};
use util::ResultExt;

#[derive(Copy, Clone)]
pub struct GoLspAdapter;

lazy_static! {
    static ref GOPLS_VERSION_REGEX: Regex = Regex::new(r"\d+\.\d+\.\d+").unwrap();
}

#[async_trait]
impl super::LspAdapterTrait for GoLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("gopls".into())
    }

    async fn server_args(&self) -> Vec<String> {
        vec!["-mode=stdio".into()]
    }

    async fn fetch_latest_server_version(
        &self,
        http: Arc<dyn HttpClient>,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release = latest_github_release("golang/tools", http).await?;
        let version: Option<String> = release.name.strip_prefix("gopls/v").map(str::to_string);
        if version.is_none() {
            log::warn!(
                "couldn't infer gopls version from github release name '{}'",
                release.name
            );
        }
        Ok(Box::new(version) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        _: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> Result<PathBuf> {
        let version = version.downcast::<Option<String>>().unwrap();
        let this = *self;

        if let Some(version) = *version {
            let binary_path = container_dir.join(&format!("gopls_{version}"));
            if let Ok(metadata) = fs::metadata(&binary_path).await {
                if metadata.is_file() {
                    if let Some(mut entries) = fs::read_dir(&container_dir).await.log_err() {
                        while let Some(entry) = entries.next().await {
                            if let Some(entry) = entry.log_err() {
                                let entry_path = entry.path();
                                if entry_path.as_path() != binary_path
                                    && entry.file_name() != "gobin"
                                {
                                    fs::remove_file(&entry_path).await.log_err();
                                }
                            }
                        }
                    }

                    return Ok(binary_path.to_path_buf());
                }
            }
        } else if let Some(path) = this.cached_server_binary(container_dir.clone()).await {
            return Ok(path.to_path_buf());
        }

        let gobin_dir = container_dir.join("gobin");
        fs::create_dir_all(&gobin_dir).await?;
        let install_output = process::Command::new("go")
            .env("GO111MODULE", "on")
            .env("GOBIN", &gobin_dir)
            .args(["install", "golang.org/x/tools/gopls@latest"])
            .output()
            .await?;
        if !install_output.status.success() {
            Err(anyhow!("failed to install gopls. Is go installed?"))?;
        }

        let installed_binary_path = gobin_dir.join("gopls");
        let version_output = process::Command::new(&installed_binary_path)
            .arg("version")
            .output()
            .await
            .map_err(|e| anyhow!("failed to run installed gopls binary {:?}", e))?;
        let version_stdout = str::from_utf8(&version_output.stdout)
            .map_err(|_| anyhow!("gopls version produced invalid utf8"))?;
        let version = GOPLS_VERSION_REGEX
            .find(version_stdout)
            .ok_or_else(|| anyhow!("failed to parse gopls version output"))?
            .as_str();
        let binary_path = container_dir.join(&format!("gopls_{version}"));
        fs::rename(&installed_binary_path, &binary_path).await?;

        Ok(binary_path.to_path_buf())
    }

    async fn cached_server_binary(&self, container_dir: PathBuf) -> Option<PathBuf> {
        (|| async move {
            let mut last_binary_path = None;
            let mut entries = fs::read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                let entry = entry?;
                if entry.file_type().await?.is_file()
                    && entry
                        .file_name()
                        .to_str()
                        .map_or(false, |name| name.starts_with("gopls_"))
                {
                    last_binary_path = Some(entry.path());
                }
            }

            if let Some(path) = last_binary_path {
                Ok(path.to_path_buf())
            } else {
                Err(anyhow!("no cached binary"))
            }
        })()
        .await
        .log_err()
    }

    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Language,
    ) -> Option<CodeLabel> {
        let label = &completion.label;

        // Gopls returns nested fields and methods as completions.
        // To syntax highlight these, combine their final component
        // with their detail.
        let name_offset = label.rfind(".").unwrap_or(0);

        match completion.kind.zip(completion.detail.as_ref()) {
            Some((lsp::CompletionItemKind::MODULE, detail)) => {
                let text = format!("{label} {detail}");
                let source = Rope::from(format!("import {text}").as_str());
                let runs = language.highlight_text(&source, 7..7 + text.len());
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..label.len(),
                });
            }
            Some((
                lsp::CompletionItemKind::CONSTANT | lsp::CompletionItemKind::VARIABLE,
                detail,
            )) => {
                let text = format!("{label} {detail}");
                let source =
                    Rope::from(format!("var {} {}", &text[name_offset..], detail).as_str());
                let runs = adjust_runs(
                    name_offset,
                    language.highlight_text(&source, 4..4 + text.len()),
                );
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..label.len(),
                });
            }
            Some((lsp::CompletionItemKind::STRUCT, _)) => {
                let text = format!("{label} struct {{}}");
                let source = Rope::from(format!("type {}", &text[name_offset..]).as_str());
                let runs = adjust_runs(
                    name_offset,
                    language.highlight_text(&source, 5..5 + text.len()),
                );
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..label.len(),
                });
            }
            Some((lsp::CompletionItemKind::INTERFACE, _)) => {
                let text = format!("{label} interface {{}}");
                let source = Rope::from(format!("type {}", &text[name_offset..]).as_str());
                let runs = adjust_runs(
                    name_offset,
                    language.highlight_text(&source, 5..5 + text.len()),
                );
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..label.len(),
                });
            }
            Some((lsp::CompletionItemKind::FIELD, detail)) => {
                let text = format!("{label} {detail}");
                let source =
                    Rope::from(format!("type T struct {{ {} }}", &text[name_offset..]).as_str());
                let runs = adjust_runs(
                    name_offset,
                    language.highlight_text(&source, 16..16 + text.len()),
                );
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..label.len(),
                });
            }
            Some((lsp::CompletionItemKind::FUNCTION | lsp::CompletionItemKind::METHOD, detail)) => {
                if let Some(signature) = detail.strip_prefix("func") {
                    let text = format!("{label}{signature}");
                    let source = Rope::from(format!("func {} {{}}", &text[name_offset..]).as_str());
                    let runs = adjust_runs(
                        name_offset,
                        language.highlight_text(&source, 5..5 + text.len()),
                    );
                    return Some(CodeLabel {
                        filter_range: 0..label.len(),
                        text,
                        runs,
                    });
                }
            }
            _ => {}
        }
        None
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        kind: lsp::SymbolKind,
        language: &Language,
    ) -> Option<CodeLabel> {
        let (text, filter_range, display_range) = match kind {
            lsp::SymbolKind::METHOD | lsp::SymbolKind::FUNCTION => {
                let text = format!("func {} () {{}}", name);
                let filter_range = 5..5 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::STRUCT => {
                let text = format!("type {} struct {{}}", name);
                let filter_range = 5..5 + name.len();
                let display_range = 0..text.len();
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::INTERFACE => {
                let text = format!("type {} interface {{}}", name);
                let filter_range = 5..5 + name.len();
                let display_range = 0..text.len();
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::CLASS => {
                let text = format!("type {} T", name);
                let filter_range = 5..5 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::CONSTANT => {
                let text = format!("const {} = nil", name);
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::VARIABLE => {
                let text = format!("var {} = nil", name);
                let filter_range = 4..4 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::MODULE => {
                let text = format!("package {}", name);
                let filter_range = 8..8 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            _ => return None,
        };

        Some(CodeLabel {
            runs: language.highlight_text(&text.as_str().into(), display_range.clone()),
            text: text[display_range].to_string(),
            filter_range,
        })
    }
}

fn adjust_runs(
    delta: usize,
    mut runs: Vec<(Range<usize>, HighlightId)>,
) -> Vec<(Range<usize>, HighlightId)> {
    for (range, _) in &mut runs {
        range.start += delta;
        range.end += delta;
    }
    runs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::languages::language;
    use gpui::color::Color;
    use theme::SyntaxTheme;

    #[test]
    fn test_go_label_for_completion() {
        let language = language(
            "go",
            tree_sitter_go::language(),
            Some(smol::block_on(LspAdapter::new(GoLspAdapter))),
        );

        let theme = SyntaxTheme::new(vec![
            ("type".into(), Color::green().into()),
            ("keyword".into(), Color::blue().into()),
            ("function".into(), Color::red().into()),
            ("number".into(), Color::yellow().into()),
            ("property".into(), Color::white().into()),
        ]);
        language.set_theme(theme.into());

        let grammar = language.grammar().unwrap();
        let highlight_function = grammar.highlight_id_for_name("function").unwrap();
        let highlight_type = grammar.highlight_id_for_name("type").unwrap();
        let highlight_keyword = grammar.highlight_id_for_name("keyword").unwrap();
        let highlight_number = grammar.highlight_id_for_name("number").unwrap();
        let highlight_field = grammar.highlight_id_for_name("property").unwrap();

        assert_eq!(
            smol::block_on(language.label_for_completion(&lsp::CompletionItem {
                kind: Some(lsp::CompletionItemKind::FUNCTION),
                label: "Hello".to_string(),
                detail: Some("func(a B) c.D".to_string()),
                ..Default::default()
            })),
            Some(CodeLabel {
                text: "Hello(a B) c.D".to_string(),
                filter_range: 0..5,
                runs: vec![
                    (0..5, highlight_function),
                    (8..9, highlight_type),
                    (13..14, highlight_type),
                ],
            })
        );

        // Nested methods
        assert_eq!(
            smol::block_on(language.label_for_completion(&lsp::CompletionItem {
                kind: Some(lsp::CompletionItemKind::METHOD),
                label: "one.two.Three".to_string(),
                detail: Some("func() [3]interface{}".to_string()),
                ..Default::default()
            })),
            Some(CodeLabel {
                text: "one.two.Three() [3]interface{}".to_string(),
                filter_range: 0..13,
                runs: vec![
                    (8..13, highlight_function),
                    (17..18, highlight_number),
                    (19..28, highlight_keyword),
                ],
            })
        );

        // Nested fields
        assert_eq!(
            smol::block_on(language.label_for_completion(&lsp::CompletionItem {
                kind: Some(lsp::CompletionItemKind::FIELD),
                label: "two.Three".to_string(),
                detail: Some("a.Bcd".to_string()),
                ..Default::default()
            })),
            Some(CodeLabel {
                text: "two.Three a.Bcd".to_string(),
                filter_range: 0..9,
                runs: vec![(4..9, highlight_field), (12..15, highlight_type)],
            })
        );
    }
}
