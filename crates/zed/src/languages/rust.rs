use super::installation::{latest_github_release, GitHubLspBinaryVersion};
use anyhow::{anyhow, Result};
use async_compression::futures::bufread::GzipDecoder;
use client::http::HttpClient;
use futures::{future::BoxFuture, io::BufReader, FutureExt, StreamExt};
pub use language::*;
use lazy_static::lazy_static;
use regex::Regex;
use smol::fs::{self, File};
use std::{
    any::Any,
    borrow::Cow,
    env::consts,
    path::{Path, PathBuf},
    str,
    sync::Arc,
};
use util::{ResultExt, TryFutureExt};

pub struct RustLspAdapter;

impl LspAdapter for RustLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("rust-analyzer".into())
    }

    fn fetch_latest_server_version(
        &self,
        http: Arc<dyn HttpClient>,
    ) -> BoxFuture<'static, Result<Box<dyn 'static + Send + Any>>> {
        async move {
            let release = latest_github_release("rust-analyzer/rust-analyzer", http).await?;
            let asset_name = format!("rust-analyzer-{}-apple-darwin.gz", consts::ARCH);
            let asset = release
                .assets
                .iter()
                .find(|asset| asset.name == asset_name)
                .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?;
            let version = GitHubLspBinaryVersion {
                name: release.name,
                url: asset.browser_download_url.clone(),
            };
            Ok(Box::new(version) as Box<_>)
        }
        .boxed()
    }

    fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        http: Arc<dyn HttpClient>,
        container_dir: Arc<Path>,
    ) -> BoxFuture<'static, Result<PathBuf>> {
        async move {
            let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
            let destination_path = container_dir.join(format!("rust-analyzer-{}", version.name));

            if fs::metadata(&destination_path).await.is_err() {
                let mut response = http
                    .get(&version.url, Default::default(), true)
                    .await
                    .map_err(|err| anyhow!("error downloading release: {}", err))?;
                let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
                let mut file = File::create(&destination_path).await?;
                futures::io::copy(decompressed_bytes, &mut file).await?;
                fs::set_permissions(
                    &destination_path,
                    <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
                )
                .await?;

                if let Some(mut entries) = fs::read_dir(&container_dir).await.log_err() {
                    while let Some(entry) = entries.next().await {
                        if let Some(entry) = entry.log_err() {
                            let entry_path = entry.path();
                            if entry_path.as_path() != destination_path {
                                fs::remove_file(&entry_path).await.log_err();
                            }
                        }
                    }
                }
            }

            Ok(destination_path)
        }
        .boxed()
    }

    fn cached_server_binary(
        &self,
        container_dir: Arc<Path>,
    ) -> BoxFuture<'static, Option<PathBuf>> {
        async move {
            let mut last = None;
            let mut entries = fs::read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                last = Some(entry?.path());
            }
            last.ok_or_else(|| anyhow!("no cached binary"))
        }
        .log_err()
        .boxed()
    }

    fn disk_based_diagnostic_sources(&self) -> &'static [&'static str] {
        &["rustc"]
    }

    fn disk_based_diagnostics_progress_token(&self) -> Option<&'static str> {
        Some("rustAnalyzer/cargo check")
    }

    fn process_diagnostics(&self, params: &mut lsp::PublishDiagnosticsParams) {
        lazy_static! {
            static ref REGEX: Regex = Regex::new("(?m)`([^`]+)\n`$").unwrap();
        }

        for diagnostic in &mut params.diagnostics {
            for message in diagnostic
                .related_information
                .iter_mut()
                .flatten()
                .map(|info| &mut info.message)
                .chain([&mut diagnostic.message])
            {
                if let Cow::Owned(sanitized) = REGEX.replace_all(message, "`$1`") {
                    *message = sanitized;
                }
            }
        }
    }

    fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Language,
    ) -> Option<CodeLabel> {
        match completion.kind {
            Some(lsp::CompletionItemKind::FIELD) if completion.detail.is_some() => {
                let detail = completion.detail.as_ref().unwrap();
                let name = &completion.label;
                let text = format!("{}: {}", name, detail);
                let source = Rope::from(format!("struct S {{ {} }}", text).as_str());
                let runs = language.highlight_text(&source, 11..11 + text.len());
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..name.len(),
                });
            }
            Some(lsp::CompletionItemKind::CONSTANT | lsp::CompletionItemKind::VARIABLE)
                if completion.detail.is_some() =>
            {
                let detail = completion.detail.as_ref().unwrap();
                let name = &completion.label;
                let text = format!("{}: {}", name, detail);
                let source = Rope::from(format!("let {} = ();", text).as_str());
                let runs = language.highlight_text(&source, 4..4 + text.len());
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..name.len(),
                });
            }
            Some(lsp::CompletionItemKind::FUNCTION | lsp::CompletionItemKind::METHOD)
                if completion.detail.is_some() =>
            {
                lazy_static! {
                    static ref REGEX: Regex = Regex::new("\\(…?\\)").unwrap();
                }

                let detail = completion.detail.as_ref().unwrap();
                if detail.starts_with("fn(") {
                    let text = REGEX.replace(&completion.label, &detail[2..]).to_string();
                    let source = Rope::from(format!("fn {} {{}}", text).as_str());
                    let runs = language.highlight_text(&source, 3..3 + text.len());
                    return Some(CodeLabel {
                        filter_range: 0..completion.label.find('(').unwrap_or(text.len()),
                        text,
                        runs,
                    });
                }
            }
            Some(kind) => {
                let highlight_name = match kind {
                    lsp::CompletionItemKind::STRUCT
                    | lsp::CompletionItemKind::INTERFACE
                    | lsp::CompletionItemKind::ENUM => Some("type"),
                    lsp::CompletionItemKind::ENUM_MEMBER => Some("variant"),
                    lsp::CompletionItemKind::KEYWORD => Some("keyword"),
                    lsp::CompletionItemKind::VALUE | lsp::CompletionItemKind::CONSTANT => {
                        Some("constant")
                    }
                    _ => None,
                };
                let highlight_id = language.grammar()?.highlight_id_for_name(highlight_name?)?;
                let mut label = CodeLabel::plain(completion.label.clone(), None);
                label.runs.push((
                    0..label.text.rfind('(').unwrap_or(label.text.len()),
                    highlight_id,
                ));
                return Some(label);
            }
            _ => {}
        }
        None
    }

    fn label_for_symbol(
        &self,
        name: &str,
        kind: lsp::SymbolKind,
        language: &Language,
    ) -> Option<CodeLabel> {
        let (text, filter_range, display_range) = match kind {
            lsp::SymbolKind::METHOD | lsp::SymbolKind::FUNCTION => {
                let text = format!("fn {} () {{}}", name);
                let filter_range = 3..3 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::STRUCT => {
                let text = format!("struct {} {{}}", name);
                let filter_range = 7..7 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::ENUM => {
                let text = format!("enum {} {{}}", name);
                let filter_range = 5..5 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::INTERFACE => {
                let text = format!("trait {} {{}}", name);
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::CONSTANT => {
                let text = format!("const {}: () = ();", name);
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::MODULE => {
                let text = format!("mod {} {{}}", name);
                let filter_range = 4..4 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::TYPE_PARAMETER => {
                let text = format!("type {} {{}}", name);
                let filter_range = 5..5 + name.len();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::languages::{language, LspAdapter};
    use gpui::color::Color;
    use theme::SyntaxTheme;

    #[test]
    fn test_process_rust_diagnostics() {
        let mut params = lsp::PublishDiagnosticsParams {
            uri: lsp::Url::from_file_path("/a").unwrap(),
            version: None,
            diagnostics: vec![
                // no newlines
                lsp::Diagnostic {
                    message: "use of moved value `a`".to_string(),
                    ..Default::default()
                },
                // newline at the end of a code span
                lsp::Diagnostic {
                    message: "consider importing this struct: `use b::c;\n`".to_string(),
                    ..Default::default()
                },
                // code span starting right after a newline
                lsp::Diagnostic {
                    message: "cannot borrow `self.d` as mutable\n`self` is a `&` reference"
                        .to_string(),
                    ..Default::default()
                },
            ],
        };
        RustLspAdapter.process_diagnostics(&mut params);

        assert_eq!(params.diagnostics[0].message, "use of moved value `a`");

        // remove trailing newline from code span
        assert_eq!(
            params.diagnostics[1].message,
            "consider importing this struct: `use b::c;`"
        );

        // do not remove newline before the start of code span
        assert_eq!(
            params.diagnostics[2].message,
            "cannot borrow `self.d` as mutable\n`self` is a `&` reference"
        );
    }

    #[test]
    fn test_rust_label_for_completion() {
        let language = language(
            "rust",
            tree_sitter_rust::language(),
            Some(Arc::new(RustLspAdapter)),
        );
        let grammar = language.grammar().unwrap();
        let theme = SyntaxTheme::new(vec![
            ("type".into(), Color::green().into()),
            ("keyword".into(), Color::blue().into()),
            ("function".into(), Color::red().into()),
            ("property".into(), Color::white().into()),
        ]);

        language.set_theme(&theme);

        let highlight_function = grammar.highlight_id_for_name("function").unwrap();
        let highlight_type = grammar.highlight_id_for_name("type").unwrap();
        let highlight_keyword = grammar.highlight_id_for_name("keyword").unwrap();
        let highlight_field = grammar.highlight_id_for_name("property").unwrap();

        assert_eq!(
            language.label_for_completion(&lsp::CompletionItem {
                kind: Some(lsp::CompletionItemKind::FUNCTION),
                label: "hello(…)".to_string(),
                detail: Some("fn(&mut Option<T>) -> Vec<T>".to_string()),
                ..Default::default()
            }),
            Some(CodeLabel {
                text: "hello(&mut Option<T>) -> Vec<T>".to_string(),
                filter_range: 0..5,
                runs: vec![
                    (0..5, highlight_function),
                    (7..10, highlight_keyword),
                    (11..17, highlight_type),
                    (18..19, highlight_type),
                    (25..28, highlight_type),
                    (29..30, highlight_type),
                ],
            })
        );

        assert_eq!(
            language.label_for_completion(&lsp::CompletionItem {
                kind: Some(lsp::CompletionItemKind::FIELD),
                label: "len".to_string(),
                detail: Some("usize".to_string()),
                ..Default::default()
            }),
            Some(CodeLabel {
                text: "len: usize".to_string(),
                filter_range: 0..3,
                runs: vec![(0..3, highlight_field), (5..10, highlight_type),],
            })
        );

        assert_eq!(
            language.label_for_completion(&lsp::CompletionItem {
                kind: Some(lsp::CompletionItemKind::FUNCTION),
                label: "hello(…)".to_string(),
                detail: Some("fn(&mut Option<T>) -> Vec<T>".to_string()),
                ..Default::default()
            }),
            Some(CodeLabel {
                text: "hello(&mut Option<T>) -> Vec<T>".to_string(),
                filter_range: 0..5,
                runs: vec![
                    (0..5, highlight_function),
                    (7..10, highlight_keyword),
                    (11..17, highlight_type),
                    (18..19, highlight_type),
                    (25..28, highlight_type),
                    (29..30, highlight_type),
                ],
            })
        );
    }

    #[test]
    fn test_rust_label_for_symbol() {
        let language = language(
            "rust",
            tree_sitter_rust::language(),
            Some(Arc::new(RustLspAdapter)),
        );
        let grammar = language.grammar().unwrap();
        let theme = SyntaxTheme::new(vec![
            ("type".into(), Color::green().into()),
            ("keyword".into(), Color::blue().into()),
            ("function".into(), Color::red().into()),
            ("property".into(), Color::white().into()),
        ]);

        language.set_theme(&theme);

        let highlight_function = grammar.highlight_id_for_name("function").unwrap();
        let highlight_type = grammar.highlight_id_for_name("type").unwrap();
        let highlight_keyword = grammar.highlight_id_for_name("keyword").unwrap();

        assert_eq!(
            language.label_for_symbol("hello", lsp::SymbolKind::FUNCTION),
            Some(CodeLabel {
                text: "fn hello".to_string(),
                filter_range: 3..8,
                runs: vec![(0..2, highlight_keyword), (3..8, highlight_function)],
            })
        );

        assert_eq!(
            language.label_for_symbol("World", lsp::SymbolKind::TYPE_PARAMETER),
            Some(CodeLabel {
                text: "type World".to_string(),
                filter_range: 5..10,
                runs: vec![(0..4, highlight_keyword), (5..10, highlight_type)],
            })
        );
    }
}
