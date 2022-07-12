use super::installation::{latest_github_release, GitHubLspBinaryVersion};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use client::http::HttpClient;
use futures::StreamExt;
pub use language::*;
use smol::fs::{self, File};
use std::{any::Any, path::PathBuf, sync::Arc};
use util::ResultExt;

pub struct CLspAdapter;

#[async_trait]
impl super::LspAdapter for CLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("clangd".into())
    }

    async fn fetch_latest_server_version(
        &self,
        http: Arc<dyn HttpClient>,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release = latest_github_release("clangd/clangd", http).await?;
        let asset_name = format!("clangd-mac-{}.zip", release.name);
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

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        http: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> Result<PathBuf> {
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
        let zip_path = container_dir.join(format!("clangd_{}.zip", version.name));
        let version_dir = container_dir.join(format!("clangd_{}", version.name));
        let binary_path = version_dir.join("bin/clangd");

        if fs::metadata(&binary_path).await.is_err() {
            let mut response = http
                .get(&version.url, Default::default(), true)
                .await
                .context("error downloading release")?;
            let mut file = File::create(&zip_path).await?;
            if !response.status().is_success() {
                Err(anyhow!(
                    "download failed with status {}",
                    response.status().to_string()
                ))?;
            }
            futures::io::copy(response.body_mut(), &mut file).await?;

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

    async fn cached_server_binary(&self, container_dir: PathBuf) -> Option<PathBuf> {
        (|| async move {
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
        })()
        .await
        .log_err()
    }

    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Language,
    ) -> Option<CodeLabel> {
        let label = completion
            .label
            .strip_prefix("•")
            .unwrap_or(&completion.label)
            .trim();

        match completion.kind {
            Some(lsp::CompletionItemKind::FIELD) if completion.detail.is_some() => {
                let detail = completion.detail.as_ref().unwrap();
                let text = format!("{} {}", detail, label);
                let source = Rope::from(format!("struct S {{ {} }}", text).as_str());
                let runs = language.highlight_text(&source, 11..11 + text.len());
                return Some(CodeLabel {
                    filter_range: detail.len() + 1..text.len(),
                    text,
                    runs,
                });
            }
            Some(lsp::CompletionItemKind::CONSTANT | lsp::CompletionItemKind::VARIABLE)
                if completion.detail.is_some() =>
            {
                let detail = completion.detail.as_ref().unwrap();
                let text = format!("{} {}", detail, label);
                let runs = language.highlight_text(&Rope::from(text.as_str()), 0..text.len());
                return Some(CodeLabel {
                    filter_range: detail.len() + 1..text.len(),
                    text,
                    runs,
                });
            }
            Some(lsp::CompletionItemKind::FUNCTION | lsp::CompletionItemKind::METHOD)
                if completion.detail.is_some() =>
            {
                let detail = completion.detail.as_ref().unwrap();
                let text = format!("{} {}", detail, label);
                let runs = language.highlight_text(&Rope::from(text.as_str()), 0..text.len());
                return Some(CodeLabel {
                    filter_range: detail.len() + 1..text.rfind('(').unwrap_or(text.len()),
                    text,
                    runs,
                });
            }
            Some(kind) => {
                let highlight_name = match kind {
                    lsp::CompletionItemKind::STRUCT
                    | lsp::CompletionItemKind::INTERFACE
                    | lsp::CompletionItemKind::CLASS
                    | lsp::CompletionItemKind::ENUM => Some("type"),
                    lsp::CompletionItemKind::ENUM_MEMBER => Some("variant"),
                    lsp::CompletionItemKind::KEYWORD => Some("keyword"),
                    lsp::CompletionItemKind::VALUE | lsp::CompletionItemKind::CONSTANT => {
                        Some("constant")
                    }
                    _ => None,
                };
                if let Some(highlight_id) = language
                    .grammar()
                    .and_then(|g| g.highlight_id_for_name(highlight_name?))
                {
                    let mut label = CodeLabel::plain(label.to_string(), None);
                    label.runs.push((
                        0..label.text.rfind('(').unwrap_or(label.text.len()),
                        highlight_id,
                    ));
                    return Some(label);
                }
            }
            _ => {}
        }
        Some(CodeLabel::plain(label.to_string(), None))
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        kind: lsp::SymbolKind,
        language: &Language,
    ) -> Option<CodeLabel> {
        let (text, filter_range, display_range) = match kind {
            lsp::SymbolKind::METHOD | lsp::SymbolKind::FUNCTION => {
                let text = format!("void {} () {{}}", name);
                let filter_range = 0..name.len();
                let display_range = 5..5 + name.len();
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
            lsp::SymbolKind::INTERFACE | lsp::SymbolKind::CLASS => {
                let text = format!("class {} {{}}", name);
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::CONSTANT => {
                let text = format!("const int {} = 0;", name);
                let filter_range = 10..10 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::MODULE => {
                let text = format!("namespace {} {{}}", name);
                let filter_range = 10..10 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::TYPE_PARAMETER => {
                let text = format!("typename {} {{}};", name);
                let filter_range = 9..9 + name.len();
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
    use gpui::MutableAppContext;
    use language::{Buffer, IndentSize};
    use std::sync::Arc;

    #[gpui::test]
    fn test_c_autoindent(cx: &mut MutableAppContext) {
        cx.foreground().set_block_on_ticks(usize::MAX..=usize::MAX);
        let language = crate::languages::language("c", tree_sitter_c::language(), None);

        cx.add_model(|cx| {
            let mut buffer = Buffer::new(0, "", cx).with_language(Arc::new(language), cx);
            let size = IndentSize::spaces(2);

            // empty function
            buffer.edit_with_autoindent([(0..0, "int main() {}")], size, cx);

            // indent inside braces
            let ix = buffer.len() - 1;
            buffer.edit_with_autoindent([(ix..ix, "\n\n")], size, cx);
            assert_eq!(buffer.text(), "int main() {\n  \n}");

            // indent body of single-statement if statement
            let ix = buffer.len() - 2;
            buffer.edit_with_autoindent([(ix..ix, "if (a)\nb;")], size, cx);
            assert_eq!(buffer.text(), "int main() {\n  if (a)\n    b;\n}");

            // indent inside field expression
            let ix = buffer.len() - 3;
            buffer.edit_with_autoindent([(ix..ix, "\n.c")], size, cx);
            assert_eq!(buffer.text(), "int main() {\n  if (a)\n    b\n      .c;\n}");

            buffer
        });
    }
}
