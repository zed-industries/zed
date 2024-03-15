use anyhow::{anyhow, bail, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use futures::{io::BufReader, StreamExt};
pub use language::*;
use log::warn;
use lsp::LanguageServerBinary;
use smol::fs::{self};
use std::{any::Any, env::consts, path::PathBuf, sync::Arc};
use util::{
    async_maybe,
    fs::remove_matching,
    github::{latest_github_release, GitHubLspBinaryVersion},
    ResultExt,
};

pub struct JavaLspAdapter;

#[async_trait(?Send)]
impl LspAdapter for JavaLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("eclipse.jdt.ls".into())
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release =
            latest_github_release("ABckh/eclipse.jdt.ls", true, false, delegate.http_client())
                .await?;
        let asset_name = "eclipse.jdt.ls.tar.gz";
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
        let dir_path = container_dir.join("bin");
        let binary_path = dir_path.join("jdtls");

        if fs::metadata(&binary_path).await.is_err() {
            let mut response = delegate
                .http_client()
                .get(&version.url, Default::default(), true)
                .await
                .map_err(|err| anyhow!("error downloading release: {}", err))?;
            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let archive = Archive::new(decompressed_bytes);

            archive.unpack(container_dir.clone()).await?;

            // TODO: windows
            #[cfg(not(windows))]
            fs::set_permissions(
                &binary_path,
                <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
            )
            .await?;

            remove_matching(&container_dir, |entry| entry != dir_path).await;
        }

        let mut config = match consts::OS {
            "macos" => "config_mac".to_string(),
            "linux" => "config_linux".to_string(),
            "windows" => "config_win".to_string(),
            other => bail!("running on unsupported os: {other}"),
        };

        if consts::OS != "windows" && consts::ARCH == "aarch64" {
            config += "_arm";
        }

        Ok(LanguageServerBinary {
            path: binary_path,
            arguments: vec!["-configuration".into(), container_dir.join(config).into()],
            env: None,
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let binary_path = container_dir.join("bin").join("jdtls");
        if binary_path.exists() {
            get_cached_server_binary(container_dir).await
        } else {
            None
        }
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

    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        match completion.kind {
            Some(
                lsp::CompletionItemKind::METHOD
                | lsp::CompletionItemKind::VARIABLE
                | lsp::CompletionItemKind::CONSTANT
                | lsp::CompletionItemKind::FIELD,
            ) => {
                if let Some((name, detail)) = completion.label.split_once(" : ") {
                    let text = format!("{detail} {name}");
                    let source = Rope::from(text.as_str());
                    let runs = language.highlight_text(&source, 0..text.len());

                    return Some(CodeLabel {
                        text: text.clone(),
                        runs,
                        filter_range: detail.len() + 1..text.len(),
                    });
                }
            }
            Some(lsp::CompletionItemKind::ENUM_MEMBER) => {
                if let Some((name, detail)) = completion.label.split_once(" : ") {
                    let property_highlight_id =
                        language.grammar()?.highlight_id_for_name("property")?;
                    let type_highlight_id = language.grammar()?.highlight_id_for_name("type")?;
                    let mut label = CodeLabel::plain(format!("{detail}.{name}"), Some(name));
                    let mut next_start = 0;

                    for identifier in detail.split('.') {
                        label
                            .runs
                            .push((next_start..next_start + identifier.len(), type_highlight_id));

                        next_start += identifier.len() + 1;
                    }

                    label.runs.push((
                        detail.len() + 1..detail.len() + 1 + name.len(),
                        property_highlight_id,
                    ));

                    return Some(label);
                }
            }
            Some(
                lsp::CompletionItemKind::CLASS
                | lsp::CompletionItemKind::INTERFACE
                | lsp::CompletionItemKind::ENUM,
            ) => {
                if let Some((name, detail)) = completion.label.split_once(" - ") {
                    let highlight_id = language.grammar()?.highlight_id_for_name("type")?;
                    let mut label = CodeLabel::plain(format!("{name} (import {detail})"), None);

                    label.runs.push((0..name.len(), highlight_id));

                    return Some(label);
                }
            }
            Some(lsp::CompletionItemKind::KEYWORD) => {
                let highlight_id = language.grammar()?.highlight_id_for_name("keyword")?;
                let mut label = CodeLabel::plain(completion.label.clone(), None);

                label.runs.push((0..label.text.len(), highlight_id));

                return Some(label);
            }
            Some(kind) if kind != lsp::CompletionItemKind::SNIPPET => {
                // Run `RUST_LOG=warn cargo run` to run and show warnings!
                warn!("Unimplemented Java completion: {completion:#?}");
            }
            _ => (),
        }

        None
    }

    fn disk_based_diagnostic_sources(&self) -> Vec<String> {
        vec!["java".into()]
    }
}

async fn get_cached_server_binary(container_dir: PathBuf) -> Option<LanguageServerBinary> {
    async_maybe!({
        let mut last_binary_path = None;
        let mut entries = fs::read_dir(&container_dir).await?;

        while let Some(entry) = entries.next().await {
            let entry = entry?;
            if entry.file_type().await?.is_file()
                && entry
                    .file_name()
                    .to_str()
                    .map_or(false, |name| name == "eclipse.jdt.ls")
            {
                last_binary_path = Some(entry.path());
            }
        }

        if let Some(path) = last_binary_path {
            Ok(LanguageServerBinary {
                path,
                arguments: Vec::new(),
                env: None,
            })
        } else {
            Err(anyhow!("no cached binary"))
        }
    })
    .await
    .log_err()
}
