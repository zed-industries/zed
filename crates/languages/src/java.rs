use anyhow::{anyhow, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use futures::{io::BufReader, StreamExt};
pub use language::*;
use lazy_static::lazy_static;
use log::warn;
use lsp::{CodeActionKind, LanguageServerBinary};
use regex::Regex;
use smol::fs::{self};
use std::{any::Any, path::PathBuf, sync::Arc};
use util::{
    async_maybe,
    github::{latest_github_release, GitHubLspBinaryVersion},
    ResultExt,
};

pub struct JavaLspAdapter;

#[async_trait]
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
            .ok_or_else(|| anyhow!("no asset found matching {:?} \n", asset_name))?;
        let version = GitHubLspBinaryVersion {
            name: release.tag_name,
            url: asset.browser_download_url.clone(),
        };

        Ok(Box::new(version) as Box<_>)
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

            // todo!("windows")
            #[cfg(not(windows))]
            {
                fs::set_permissions(
                    &binary_path,
                    <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
                )
                .await?;
            }
        }

        Ok(LanguageServerBinary {
            path: binary_path,
            env: None,
            arguments: vec![
                "-configuration".into(),
                container_dir.join("config_mac").into(),
            ],
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

    // TODO: code actions don't work
    fn code_action_kinds(&self) -> Option<Vec<CodeActionKind>> {
        Some(vec![
            CodeActionKind::EMPTY,
            CodeActionKind::QUICKFIX,
            CodeActionKind::REFACTOR,
            CodeActionKind::REFACTOR_EXTRACT,
            CodeActionKind::SOURCE,
        ])
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
        vec!["java".into()]
    }

    // TODO: filter diagnostics to get rid of annoying messages while typing
    fn process_diagnostics(&self, params: &mut lsp::PublishDiagnosticsParams) {
    }

    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        match completion.kind {
            Some(
                lsp::CompletionItemKind::VARIABLE
                | lsp::CompletionItemKind::CONSTANT
                | lsp::CompletionItemKind::FIELD,
            ) => {
                if let Some((name, detail)) = completion.label.split_once(" : ") {
                    let highlight_id = language.grammar()?.highlight_id_for_name("constant")?;
                    let mut label = CodeLabel::plain(format!("{name}: {detail}"), None);

                    label.runs.push((0..name.len(), highlight_id));

                    return Some(label);
                }
            }
            Some(lsp::CompletionItemKind::METHOD) => {
                if let Some((name, detail)) = completion.label.split_once(" : ") {
                    let highlight_id = language
                        .grammar()?
                        .highlight_id_for_name("function.method")?;
                    let mut label = CodeLabel::plain(format!("{name}: {detail}"), None);

                    label.runs.push((0..name.len(), highlight_id));

                    return Some(label);
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
                warn!("Unimplemented completion: {completion:#?}")
            }
            _ => (),
        }

        None
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
