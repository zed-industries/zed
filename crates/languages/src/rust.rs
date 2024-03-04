use anyhow::{anyhow, bail, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_trait::async_trait;
use futures::{io::BufReader, StreamExt};
pub use language::*;
use lazy_static::lazy_static;
use lsp::LanguageServerBinary;
use regex::Regex;
use smol::fs::{self, File};
use std::{any::Any, borrow::Cow, env::consts, path::PathBuf, str, sync::Arc};
use util::{
    async_maybe,
    fs::remove_matching,
    github::{latest_github_release, GitHubLspBinaryVersion},
    ResultExt,
};

pub struct RustLspAdapter;

#[async_trait]
impl LspAdapter for RustLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("rust-analyzer".into())
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release = latest_github_release(
            "rust-lang/rust-analyzer",
            true,
            false,
            delegate.http_client(),
        )
        .await?;
        let os = match consts::OS {
            "macos" => "apple-darwin",
            "linux" => "unknown-linux-gnu",
            "windows" => "pc-windows-msvc",
            other => bail!("Running on unsupported os: {other}"),
        };
        let asset_name = format!("rust-analyzer-{}-{os}.gz", consts::ARCH);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?;
        Ok(Box::new(GitHubLspBinaryVersion {
            name: release.tag_name,
            url: asset.browser_download_url.clone(),
        }))
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
        let destination_path = container_dir.join(format!("rust-analyzer-{}", version.name));

        if fs::metadata(&destination_path).await.is_err() {
            let mut response = delegate
                .http_client()
                .get(&version.url, Default::default(), true)
                .await
                .map_err(|err| anyhow!("error downloading release: {}", err))?;
            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let mut file = File::create(&destination_path).await?;
            futures::io::copy(decompressed_bytes, &mut file).await?;
            // todo("windows")
            #[cfg(not(windows))]
            {
                fs::set_permissions(
                    &destination_path,
                    <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
                )
                .await?;
            }

            remove_matching(&container_dir, |entry| entry != destination_path).await;
        }

        Ok(LanguageServerBinary {
            path: destination_path,
            env: None,
            arguments: Default::default(),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir).await
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir)
            .await
            .map(|mut binary| {
                binary.arguments = vec!["--help".into()];
                binary
            })
    }

    fn disk_based_diagnostic_sources(&self) -> Vec<String> {
        vec!["rustc".into()]
    }

    fn disk_based_diagnostics_progress_token(&self) -> Option<String> {
        Some("rust-analyzer/flycheck".into())
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

    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Arc<Language>,
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
                if completion.detail.is_some()
                    && completion.insert_text_format != Some(lsp::InsertTextFormat::SNIPPET) =>
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
                const FUNCTION_PREFIXES: [&'static str; 2] = ["async fn", "fn"];
                let prefix = FUNCTION_PREFIXES
                    .iter()
                    .find_map(|prefix| detail.strip_prefix(*prefix).map(|suffix| (prefix, suffix)));
                // fn keyword should be followed by opening parenthesis.
                if let Some((prefix, suffix)) = prefix {
                    if suffix.starts_with('(') {
                        let text = REGEX.replace(&completion.label, suffix).to_string();
                        let source = Rope::from(format!("{prefix} {} {{}}", text).as_str());
                        let run_start = prefix.len() + 1;
                        let runs =
                            language.highlight_text(&source, run_start..run_start + text.len());
                        return Some(CodeLabel {
                            filter_range: 0..completion.label.find('(').unwrap_or(text.len()),
                            text,
                            runs,
                        });
                    }
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

    async fn label_for_symbol(
        &self,
        name: &str,
        kind: lsp::SymbolKind,
        language: &Arc<Language>,
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

async fn get_cached_server_binary(container_dir: PathBuf) -> Option<LanguageServerBinary> {
    async_maybe!({
        let mut last = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            last = Some(entry?.path());
        }

        anyhow::Ok(LanguageServerBinary {
            path: last.ok_or_else(|| anyhow!("no cached binary"))?,
            env: None,
            arguments: Default::default(),
        })
    })
    .await
    .log_err()
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::*;
    use crate::language;
    use gpui::{Context, Hsla, TestAppContext};
    use language::language_settings::AllLanguageSettings;
    use settings::SettingsStore;
    use text::BufferId;
    use theme::SyntaxTheme;

    #[gpui::test]
    async fn test_process_rust_diagnostics() {
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

    #[gpui::test]
    async fn test_rust_label_for_completion() {
        let adapter = Arc::new(RustLspAdapter);
        let language = language("rust", tree_sitter_rust::language());
        let grammar = language.grammar().unwrap();
        let theme = SyntaxTheme::new_test([
            ("type", Hsla::default()),
            ("keyword", Hsla::default()),
            ("function", Hsla::default()),
            ("property", Hsla::default()),
        ]);

        language.set_theme(&theme);

        let highlight_function = grammar.highlight_id_for_name("function").unwrap();
        let highlight_type = grammar.highlight_id_for_name("type").unwrap();
        let highlight_keyword = grammar.highlight_id_for_name("keyword").unwrap();
        let highlight_field = grammar.highlight_id_for_name("property").unwrap();

        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::FUNCTION),
                        label: "hello(…)".to_string(),
                        detail: Some("fn(&mut Option<T>) -> Vec<T>".to_string()),
                        ..Default::default()
                    },
                    &language
                )
                .await,
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
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::FUNCTION),
                        label: "hello(…)".to_string(),
                        detail: Some("async fn(&mut Option<T>) -> Vec<T>".to_string()),
                        ..Default::default()
                    },
                    &language
                )
                .await,
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
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::FIELD),
                        label: "len".to_string(),
                        detail: Some("usize".to_string()),
                        ..Default::default()
                    },
                    &language
                )
                .await,
            Some(CodeLabel {
                text: "len: usize".to_string(),
                filter_range: 0..3,
                runs: vec![(0..3, highlight_field), (5..10, highlight_type),],
            })
        );

        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::FUNCTION),
                        label: "hello(…)".to_string(),
                        detail: Some("fn(&mut Option<T>) -> Vec<T>".to_string()),
                        ..Default::default()
                    },
                    &language
                )
                .await,
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

    #[gpui::test]
    async fn test_rust_label_for_symbol() {
        let adapter = Arc::new(RustLspAdapter);
        let language = language("rust", tree_sitter_rust::language());
        let grammar = language.grammar().unwrap();
        let theme = SyntaxTheme::new_test([
            ("type", Hsla::default()),
            ("keyword", Hsla::default()),
            ("function", Hsla::default()),
            ("property", Hsla::default()),
        ]);

        language.set_theme(&theme);

        let highlight_function = grammar.highlight_id_for_name("function").unwrap();
        let highlight_type = grammar.highlight_id_for_name("type").unwrap();
        let highlight_keyword = grammar.highlight_id_for_name("keyword").unwrap();

        assert_eq!(
            adapter
                .label_for_symbol("hello", lsp::SymbolKind::FUNCTION, &language)
                .await,
            Some(CodeLabel {
                text: "fn hello".to_string(),
                filter_range: 3..8,
                runs: vec![(0..2, highlight_keyword), (3..8, highlight_function)],
            })
        );

        assert_eq!(
            adapter
                .label_for_symbol("World", lsp::SymbolKind::TYPE_PARAMETER, &language)
                .await,
            Some(CodeLabel {
                text: "type World".to_string(),
                filter_range: 5..10,
                runs: vec![(0..4, highlight_keyword), (5..10, highlight_type)],
            })
        );
    }

    #[gpui::test]
    async fn test_rust_autoindent(cx: &mut TestAppContext) {
        // cx.executor().set_block_on_ticks(usize::MAX..=usize::MAX);
        cx.update(|cx| {
            let test_settings = SettingsStore::test(cx);
            cx.set_global(test_settings);
            language::init(cx);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings::<AllLanguageSettings>(cx, |s| {
                    s.defaults.tab_size = NonZeroU32::new(2);
                });
            });
        });

        let language = crate::language("rust", tree_sitter_rust::language());

        cx.new_model(|cx| {
            let mut buffer = Buffer::new(0, BufferId::new(cx.entity_id().as_u64()).unwrap(), "")
                .with_language(language, cx);

            // indent between braces
            buffer.set_text("fn a() {}", cx);
            let ix = buffer.len() - 1;
            buffer.edit([(ix..ix, "\n\n")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(buffer.text(), "fn a() {\n  \n}");

            // indent between braces, even after empty lines
            buffer.set_text("fn a() {\n\n\n}", cx);
            let ix = buffer.len() - 2;
            buffer.edit([(ix..ix, "\n")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(buffer.text(), "fn a() {\n\n\n  \n}");

            // indent a line that continues a field expression
            buffer.set_text("fn a() {\n  \n}", cx);
            let ix = buffer.len() - 2;
            buffer.edit([(ix..ix, "b\n.c")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(buffer.text(), "fn a() {\n  b\n    .c\n}");

            // indent further lines that continue the field expression, even after empty lines
            let ix = buffer.len() - 2;
            buffer.edit([(ix..ix, "\n\n.d")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(buffer.text(), "fn a() {\n  b\n    .c\n    \n    .d\n}");

            // dedent the line after the field expression
            let ix = buffer.len() - 2;
            buffer.edit([(ix..ix, ";\ne")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(
                buffer.text(),
                "fn a() {\n  b\n    .c\n    \n    .d;\n  e\n}"
            );

            // indent inside a struct within a call
            buffer.set_text("const a: B = c(D {});", cx);
            let ix = buffer.len() - 3;
            buffer.edit([(ix..ix, "\n\n")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(buffer.text(), "const a: B = c(D {\n  \n});");

            // indent further inside a nested call
            let ix = buffer.len() - 4;
            buffer.edit([(ix..ix, "e: f(\n\n)")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(buffer.text(), "const a: B = c(D {\n  e: f(\n    \n  )\n});");

            // keep that indent after an empty line
            let ix = buffer.len() - 8;
            buffer.edit([(ix..ix, "\n")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(
                buffer.text(),
                "const a: B = c(D {\n  e: f(\n    \n    \n  )\n});"
            );

            buffer
        });
    }
}
