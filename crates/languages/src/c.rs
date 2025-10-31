use anyhow::{Context as _, Result, bail};
use async_trait::async_trait;
use futures::StreamExt;
use gpui::{App, AsyncApp};
use http_client::github::{AssetKind, GitHubLspBinaryVersion, latest_github_release};
use http_client::github_download::{GithubBinaryMetadata, download_server_binary};
pub use language::*;
use lsp::{InitializeParams, LanguageServerBinary, LanguageServerName};
use project::lsp_store::clangd_ext;
use serde_json::json;
use smol::fs;
use std::{env::consts, path::PathBuf, sync::Arc};
use util::{ResultExt, fs::remove_matching, maybe, merge_json_value_into};

pub struct CLspAdapter;

impl CLspAdapter {
    const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("clangd");
}

impl LspInstaller for CLspAdapter {
    type BinaryVersion = GitHubLspBinaryVersion;

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
        pre_release: bool,
        _: &mut AsyncApp,
    ) -> Result<GitHubLspBinaryVersion> {
        let release =
            latest_github_release("clangd/clangd", true, pre_release, delegate.http_client())
                .await?;
        let os_suffix = match consts::OS {
            "macos" => "mac",
            "linux" => "linux",
            "windows" => "windows",
            other => bail!("Running on unsupported os: {other}"),
        };
        let asset_name = format!("clangd-{}-{}.zip", os_suffix, release.tag_name);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .with_context(|| format!("no asset found matching {asset_name:?}"))?;
        let version = GitHubLspBinaryVersion {
            name: release.tag_name,
            url: asset.browser_download_url.clone(),
            digest: asset.digest.clone(),
        };
        Ok(version)
    }

    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        _: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        let path = delegate.which(Self::SERVER_NAME.as_ref()).await?;
        Some(LanguageServerBinary {
            path,
            arguments: Vec::new(),
            env: None,
        })
    }

    async fn fetch_server_binary(
        &self,
        version: GitHubLspBinaryVersion,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let GitHubLspBinaryVersion {
            name,
            url,
            digest: expected_digest,
        } = version;
        let version_dir = container_dir.join(format!("clangd_{name}"));
        let binary_path = version_dir.join("bin/clangd");

        let binary = LanguageServerBinary {
            path: binary_path.clone(),
            env: None,
            arguments: Default::default(),
        };

        let metadata_path = version_dir.join("metadata");
        let metadata = GithubBinaryMetadata::read_from_file(&metadata_path)
            .await
            .ok();
        if let Some(metadata) = metadata {
            let validity_check = async || {
                delegate
                    .try_exec(LanguageServerBinary {
                        path: binary_path.clone(),
                        arguments: vec!["--version".into()],
                        env: None,
                    })
                    .await
                    .inspect_err(|err| {
                        log::warn!("Unable to run {binary_path:?} asset, redownloading: {err}",)
                    })
            };
            if let (Some(actual_digest), Some(expected_digest)) =
                (&metadata.digest, &expected_digest)
            {
                if actual_digest == expected_digest {
                    if validity_check().await.is_ok() {
                        return Ok(binary);
                    }
                } else {
                    log::info!(
                        "SHA-256 mismatch for {binary_path:?} asset, downloading new asset. Expected: {expected_digest}, Got: {actual_digest}"
                    );
                }
            } else if validity_check().await.is_ok() {
                return Ok(binary);
            }
        }
        download_server_binary(
            &*delegate.http_client(),
            &url,
            expected_digest.as_deref(),
            &container_dir,
            AssetKind::Zip,
        )
        .await?;
        remove_matching(&container_dir, |entry| entry != version_dir).await;
        GithubBinaryMetadata::write_to_file(
            &GithubBinaryMetadata {
                metadata_version: 1,
                digest: expected_digest,
            },
            &metadata_path,
        )
        .await?;

        Ok(binary)
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir).await
    }
}

#[async_trait(?Send)]
impl super::LspAdapter for CLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
    }

    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        let label_detail = match &completion.label_details {
            Some(label_detail) => match &label_detail.detail {
                Some(detail) => detail.trim(),
                None => "",
            },
            None => "",
        };

        let mut label = completion
            .label
            .strip_prefix('•')
            .unwrap_or(&completion.label)
            .trim()
            .to_owned();

        if !label_detail.is_empty() {
            let should_add_space = match completion.kind {
                Some(lsp::CompletionItemKind::FUNCTION | lsp::CompletionItemKind::METHOD) => false,
                _ => true,
            };

            if should_add_space && !label.ends_with(' ') && !label_detail.starts_with(' ') {
                label.push(' ');
            }
            label.push_str(label_detail);
        }

        match completion.kind {
            Some(lsp::CompletionItemKind::FIELD) if completion.detail.is_some() => {
                let detail = completion.detail.as_ref().unwrap();
                let text = format!("{} {}", detail, label);
                let source = Rope::from_str_small(format!("struct S {{ {} }}", text).as_str());
                let runs = language.highlight_text(&source, 11..11 + text.len());
                let filter_range = completion
                    .filter_text
                    .as_deref()
                    .and_then(|filter_text| {
                        text.find(filter_text)
                            .map(|start| start..start + filter_text.len())
                    })
                    .unwrap_or(detail.len() + 1..text.len());
                return Some(CodeLabel::new(text, filter_range, runs));
            }
            Some(lsp::CompletionItemKind::CONSTANT | lsp::CompletionItemKind::VARIABLE)
                if completion.detail.is_some() =>
            {
                let detail = completion.detail.as_ref().unwrap();
                let text = format!("{} {}", detail, label);
                let runs =
                    language.highlight_text(&Rope::from_str_small(text.as_str()), 0..text.len());
                let filter_range = completion
                    .filter_text
                    .as_deref()
                    .and_then(|filter_text| {
                        text.find(filter_text)
                            .map(|start| start..start + filter_text.len())
                    })
                    .unwrap_or(detail.len() + 1..text.len());
                return Some(CodeLabel::new(text, filter_range, runs));
            }
            Some(lsp::CompletionItemKind::FUNCTION | lsp::CompletionItemKind::METHOD)
                if completion.detail.is_some() =>
            {
                let detail = completion.detail.as_ref().unwrap();
                let text = format!("{} {}", detail, label);
                let runs =
                    language.highlight_text(&Rope::from_str_small(text.as_str()), 0..text.len());
                let filter_range = completion
                    .filter_text
                    .as_deref()
                    .and_then(|filter_text| {
                        text.find(filter_text)
                            .map(|start| start..start + filter_text.len())
                    })
                    .unwrap_or_else(|| {
                        let filter_start = detail.len() + 1;
                        let filter_end = text
                            .rfind('(')
                            .filter(|end| *end > filter_start)
                            .unwrap_or(text.len());
                        filter_start..filter_end
                    });

                return Some(CodeLabel::new(text, filter_range, runs));
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
                    let mut label = CodeLabel::plain(label, completion.filter_text.as_deref());
                    label.runs.push((
                        0..label.text.rfind('(').unwrap_or(label.text.len()),
                        highlight_id,
                    ));
                    return Some(label);
                }
            }
            _ => {}
        }
        Some(CodeLabel::plain(label, completion.filter_text.as_deref()))
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        kind: lsp::SymbolKind,
        language: &Arc<Language>,
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

        Some(CodeLabel::new(
            text[display_range.clone()].to_string(),
            filter_range,
            language.highlight_text(&Rope::from_str_small(text.as_str()), display_range),
        ))
    }

    fn prepare_initialize_params(
        &self,
        mut original: InitializeParams,
        _: &App,
    ) -> Result<InitializeParams> {
        let experimental = json!({
            "textDocument": {
                "completion" : {
                    // enable clangd's dot-to-arrow feature.
                    "editsNearCursor": true
                },
                "inactiveRegionsCapabilities": {
                    "inactiveRegions": true,
                }
            }
        });
        if let Some(ref mut original_experimental) = original.capabilities.experimental {
            merge_json_value_into(experimental, original_experimental);
        } else {
            original.capabilities.experimental = Some(experimental);
        }
        Ok(original)
    }

    fn retain_old_diagnostic(&self, previous_diagnostic: &Diagnostic, _: &App) -> bool {
        clangd_ext::is_inactive_region(previous_diagnostic)
    }

    fn underline_diagnostic(&self, diagnostic: &lsp::Diagnostic) -> bool {
        !clangd_ext::is_lsp_inactive_region(diagnostic)
    }
}

async fn get_cached_server_binary(container_dir: PathBuf) -> Option<LanguageServerBinary> {
    maybe!(async {
        let mut last_clangd_dir = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            let entry = entry?;
            if entry.file_type().await?.is_dir() {
                last_clangd_dir = Some(entry.path());
            }
        }
        let clangd_dir = last_clangd_dir.context("no cached binary")?;
        let clangd_bin = clangd_dir.join("bin/clangd");
        anyhow::ensure!(
            clangd_bin.exists(),
            "missing clangd binary in directory {clangd_dir:?}"
        );
        Ok(LanguageServerBinary {
            path: clangd_bin,
            env: None,
            arguments: Vec::new(),
        })
    })
    .await
    .log_err()
}

#[cfg(test)]
mod tests {
    use gpui::{AppContext as _, BorrowAppContext, TestAppContext};
    use language::{AutoindentMode, Buffer};
    use settings::SettingsStore;
    use std::num::NonZeroU32;

    #[gpui::test]
    async fn test_c_autoindent(cx: &mut TestAppContext) {
        // cx.executor().set_block_on_ticks(usize::MAX..=usize::MAX);
        cx.update(|cx| {
            let test_settings = SettingsStore::test(cx);
            cx.set_global(test_settings);
            language::init(cx);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |s| {
                    s.project.all_languages.defaults.tab_size = NonZeroU32::new(2);
                });
            });
        });
        let language = crate::language("c", tree_sitter_c::LANGUAGE.into());

        cx.new(|cx| {
            let mut buffer = Buffer::local("", cx).with_language(language, cx);

            // empty function
            buffer.edit([(0..0, "int main() {}")], None, cx);

            // indent inside braces
            let ix = buffer.len() - 1;
            buffer.edit([(ix..ix, "\n\n")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(buffer.text(), "int main() {\n  \n}");

            // indent body of single-statement if statement
            let ix = buffer.len() - 2;
            buffer.edit([(ix..ix, "if (a)\nb;")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(buffer.text(), "int main() {\n  if (a)\n    b;\n}");

            // indent inside field expression
            let ix = buffer.len() - 3;
            buffer.edit([(ix..ix, "\n.c")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(buffer.text(), "int main() {\n  if (a)\n    b\n      .c;\n}");

            buffer
        });
    }
}
