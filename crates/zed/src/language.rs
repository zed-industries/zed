use anyhow::{anyhow, Context, Result};
use async_compression::futures::bufread::GzipDecoder;
use client::http::{self, HttpClient, Method};
use futures::{future::BoxFuture, FutureExt, StreamExt};
use gpui::Task;
pub use language::*;
use lazy_static::lazy_static;
use regex::Regex;
use rust_embed::RustEmbed;
use serde::Deserialize;
use serde_json::json;
use smol::fs::{self, File};
use std::{borrow::Cow, env::consts, path::PathBuf, str, sync::Arc};
use util::{ResultExt, TryFutureExt};

#[derive(RustEmbed)]
#[folder = "languages"]
struct LanguageDir;

struct RustLspAdapter;
struct CLspAdapter;
struct JsonLspAdapter;

#[derive(Deserialize)]
struct GithubRelease {
    name: String,
    assets: Vec<GithubReleaseAsset>,
}

#[derive(Deserialize)]
struct GithubReleaseAsset {
    name: String,
    browser_download_url: http::Url,
}

impl LspAdapter for RustLspAdapter {
    fn name(&self) -> &'static str {
        "rust-analyzer"
    }

    fn fetch_latest_server_version(
        &self,
        http: Arc<dyn HttpClient>,
    ) -> BoxFuture<'static, Result<LspBinaryVersion>> {
        async move {
            let release = http
            .send(
                surf::RequestBuilder::new(
                    Method::Get,
                    http::Url::parse(
                        "https://api.github.com/repos/rust-analyzer/rust-analyzer/releases/latest",
                    )
                    .unwrap(),
                )
                .middleware(surf::middleware::Redirect::default())
                .build(),
            )
            .await
            .map_err(|err| anyhow!("error fetching latest release: {}", err))?
            .body_json::<GithubRelease>()
            .await
            .map_err(|err| anyhow!("error parsing latest release: {}", err))?;
            let asset_name = format!("rust-analyzer-{}-apple-darwin.gz", consts::ARCH);
            let asset = release
                .assets
                .iter()
                .find(|asset| asset.name == asset_name)
                .ok_or_else(|| anyhow!("no release found matching {:?}", asset_name))?;
            Ok(LspBinaryVersion {
                name: release.name,
                url: Some(asset.browser_download_url.clone()),
            })
        }
        .boxed()
    }

    fn fetch_server_binary(
        &self,
        version: LspBinaryVersion,
        http: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> BoxFuture<'static, Result<PathBuf>> {
        async move {
            let destination_path = container_dir.join(format!("rust-analyzer-{}", version.name));

            if fs::metadata(&destination_path).await.is_err() {
                let response = http
                    .send(
                        surf::RequestBuilder::new(Method::Get, version.url.unwrap())
                            .middleware(surf::middleware::Redirect::default())
                            .build(),
                    )
                    .await
                    .map_err(|err| anyhow!("error downloading release: {}", err))?;
                let decompressed_bytes = GzipDecoder::new(response);
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

    fn cached_server_binary(&self, container_dir: PathBuf) -> BoxFuture<'static, Option<PathBuf>> {
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

impl LspAdapter for CLspAdapter {
    fn name(&self) -> &'static str {
        "clangd"
    }

    fn fetch_latest_server_version(
        &self,
        http: Arc<dyn HttpClient>,
    ) -> BoxFuture<'static, Result<LspBinaryVersion>> {
        async move {
            let release = http
                .send(
                    surf::RequestBuilder::new(
                        Method::Get,
                        http::Url::parse(
                            "https://api.github.com/repos/clangd/clangd/releases/latest",
                        )
                        .unwrap(),
                    )
                    .middleware(surf::middleware::Redirect::default())
                    .build(),
                )
                .await
                .map_err(|err| anyhow!("error fetching latest release: {}", err))?
                .body_json::<GithubRelease>()
                .await
                .map_err(|err| anyhow!("error parsing latest release: {}", err))?;
            let asset_name = format!("clangd-mac-{}.zip", release.name);
            let asset = release
                .assets
                .iter()
                .find(|asset| asset.name == asset_name)
                .ok_or_else(|| anyhow!("no release found matching {:?}", asset_name))?;
            Ok(LspBinaryVersion {
                name: release.name,
                url: Some(asset.browser_download_url.clone()),
            })
        }
        .boxed()
    }

    fn fetch_server_binary(
        &self,
        version: LspBinaryVersion,
        http: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> BoxFuture<'static, Result<PathBuf>> {
        async move {
            let zip_path = container_dir.join(format!("clangd_{}.zip", version.name));
            let version_dir = container_dir.join(format!("clangd_{}", version.name));
            let binary_path = version_dir.join("bin/clangd");

            if fs::metadata(&binary_path).await.is_err() {
                let response = http
                    .send(
                        surf::RequestBuilder::new(Method::Get, version.url.unwrap())
                            .middleware(surf::middleware::Redirect::default())
                            .build(),
                    )
                    .await
                    .map_err(|err| anyhow!("error downloading release: {}", err))?;
                let mut file = File::create(&zip_path).await?;
                if !response.status().is_success() {
                    Err(anyhow!(
                        "download failed with status {}",
                        response.status().to_string()
                    ))?;
                }
                futures::io::copy(response, &mut file).await?;

                let unzip_status = smol::process::Command::new("unzip")
                    .current_dir(&container_dir)
                    .arg(&zip_path)
                    .output()
                    .await?
                    .status;
                if !unzip_status.success() {
                    Err(anyhow!("failed to unzip clangd archive"))?;
                }

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
        .boxed()
    }

    fn cached_server_binary(&self, container_dir: PathBuf) -> BoxFuture<'static, Option<PathBuf>> {
        async move {
            let mut last_clangd_dir = None;
            let mut entries = fs::read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                let entry = entry?;
                if entry.file_type().await?.is_dir() {
                    last_clangd_dir = Some(entry.path());
                }
            }
            let clangd_dir = last_clangd_dir.ok_or_else(|| anyhow!("no cached binary"))?;
            let clangd_bin = clangd_dir.join("bin/clangd");
            if clangd_bin.exists() {
                Ok(clangd_bin)
            } else {
                Err(anyhow!(
                    "missing clangd binary in directory {:?}",
                    clangd_dir
                ))
            }
        }
        .log_err()
        .boxed()
    }

    fn process_diagnostics(&self, _: &mut lsp::PublishDiagnosticsParams) {}
}

impl JsonLspAdapter {
    const BIN_PATH: &'static str =
        "node_modules/vscode-json-languageserver/bin/vscode-json-languageserver";
}

impl LspAdapter for JsonLspAdapter {
    fn name(&self) -> &'static str {
        "vscode-json-languageserver"
    }

    fn server_args(&self) -> &[&str] {
        &["--stdio"]
    }

    fn fetch_latest_server_version(
        &self,
        _: Arc<dyn HttpClient>,
    ) -> BoxFuture<'static, Result<LspBinaryVersion>> {
        async move {
            #[derive(Deserialize)]
            struct NpmInfo {
                versions: Vec<String>,
            }

            let output = smol::process::Command::new("npm")
                .args(["info", "vscode-json-languageserver", "--json"])
                .output()
                .await?;
            if !output.status.success() {
                Err(anyhow!("failed to execute npm info"))?;
            }
            let mut info: NpmInfo = serde_json::from_slice(&output.stdout)?;

            Ok(LspBinaryVersion {
                name: info
                    .versions
                    .pop()
                    .ok_or_else(|| anyhow!("no versions found in npm info"))?,
                url: Default::default(),
            })
        }
        .boxed()
    }

    fn fetch_server_binary(
        &self,
        version: LspBinaryVersion,
        _: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> BoxFuture<'static, Result<PathBuf>> {
        async move {
            let version_dir = container_dir.join(&version.name);
            fs::create_dir_all(&version_dir)
                .await
                .context("failed to create version directory")?;
            let binary_path = version_dir.join(Self::BIN_PATH);

            if fs::metadata(&binary_path).await.is_err() {
                let output = smol::process::Command::new("npm")
                    .current_dir(&version_dir)
                    .arg("install")
                    .arg(format!("vscode-json-languageserver@{}", version.name))
                    .output()
                    .await
                    .context("failed to run npm install")?;
                if !output.status.success() {
                    Err(anyhow!("failed to install vscode-json-languageserver"))?;
                }

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
        .boxed()
    }

    fn cached_server_binary(&self, container_dir: PathBuf) -> BoxFuture<'static, Option<PathBuf>> {
        async move {
            let mut last_version_dir = None;
            let mut entries = fs::read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                let entry = entry?;
                if entry.file_type().await?.is_dir() {
                    last_version_dir = Some(entry.path());
                }
            }
            let last_version_dir = last_version_dir.ok_or_else(|| anyhow!("no cached binary"))?;
            let bin_path = last_version_dir.join(Self::BIN_PATH);
            if bin_path.exists() {
                Ok(bin_path)
            } else {
                Err(anyhow!(
                    "missing executable in directory {:?}",
                    last_version_dir
                ))
            }
        }
        .log_err()
        .boxed()
    }

    fn process_diagnostics(&self, _: &mut lsp::PublishDiagnosticsParams) {}

    fn initialization_options(&self) -> Option<serde_json::Value> {
        Some(json!({
            "provideFormatter": true
        }))
    }
}

pub fn build_language_registry(login_shell_env_loaded: Task<()>) -> LanguageRegistry {
    let languages = LanguageRegistry::new(login_shell_env_loaded);
    for (name, grammar, lsp_adapter) in [
        (
            "c",
            tree_sitter_c::language(),
            Some(Arc::new(CLspAdapter) as Arc<dyn LspAdapter>),
        ),
        (
            "json",
            tree_sitter_json::language(),
            Some(Arc::new(JsonLspAdapter)),
        ),
        (
            "markdown",
            tree_sitter_markdown::language(),
            None, //
        ),
        (
            "rust",
            tree_sitter_rust::language(),
            Some(Arc::new(RustLspAdapter)),
        ),
    ] {
        languages.add(Arc::new(language(name, grammar, lsp_adapter)));
    }
    languages
}

fn language(
    name: &str,
    grammar: tree_sitter::Language,
    lsp_adapter: Option<Arc<dyn LspAdapter>>,
) -> Language {
    let config = toml::from_slice(
        &LanguageDir::get(&format!("{}/config.toml", name))
            .unwrap()
            .data,
    )
    .unwrap();
    let mut language = Language::new(config, Some(grammar));
    if let Some(query) = load_query(&format!("{}/highlights.scm", name)) {
        language = language.with_highlights_query(query.as_ref()).unwrap();
    }
    if let Some(query) = load_query(&format!("{}/brackets.scm", name)) {
        language = language.with_brackets_query(query.as_ref()).unwrap();
    }
    if let Some(query) = load_query(&format!("{}/indents.scm", name)) {
        language = language.with_indents_query(query.as_ref()).unwrap();
    }
    if let Some(query) = load_query(&format!("{}/outline.scm", name)) {
        language = language.with_outline_query(query.as_ref()).unwrap();
    }
    if let Some(lsp_adapter) = lsp_adapter {
        language = language.with_lsp_adapter(lsp_adapter)
    }
    language
}

fn load_query(path: &str) -> Option<Cow<'static, str>> {
    LanguageDir::get(path).map(|item| match item.data {
        Cow::Borrowed(s) => Cow::Borrowed(str::from_utf8(s).unwrap()),
        Cow::Owned(s) => Cow::Owned(String::from_utf8(s).unwrap()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::color::Color;
    use language::LspAdapter;
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
