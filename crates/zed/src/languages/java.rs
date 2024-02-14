use anyhow::{anyhow, Result};
use async_trait::async_trait;
pub use language::*;
use log::warn;
use lsp::LanguageServerBinary;
use std::{any::Any, path::PathBuf, str, sync::Arc};
use util::paths::HOME;

pub struct JavaLspAdapter;

#[async_trait]
impl LspAdapter for JavaLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("eclipse.jdt.ls".into())
    }

    fn short_name(&self) -> &'static str {
        "jdtls"
    }

    async fn fetch_latest_server_version(
        &self,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(()))
    }

    async fn fetch_server_binary(
        &self,
        _version: Box<dyn 'static + Send + Any>,
        _container_dir: PathBuf,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        Err(anyhow!("eclipse.jdt.ls must be installed manually"))
    }

    async fn cached_server_binary(
        &self,
        _container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        Some(LanguageServerBinary {
            path: "jdtls".into(),
            arguments: vec![
                "-configuration".into(),
                HOME.join(".cache/jdtls").into(),
                // Should work but... doesn't
                // "-data".into(),
                // ".".into(),
            ],
        })
    }

    fn can_be_reinstalled(&self) -> bool {
        false
    }

    async fn installation_test_binary(
        &self,
        _container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        Some(LanguageServerBinary {
            path: "jdtls".into(),
            arguments: vec!["--help".into()],
        })
    }

    fn disk_based_diagnostic_sources(&self) -> Vec<String> {
        vec!["java".into()]
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
                    let text = format!("{detail} {name}");
                    let source = Rope::from(format!("{} = null;", text).as_str());
                    let runs = language.highlight_text(&source, 0..text.len());

                    return Some(CodeLabel {
                        text,
                        runs,
                        filter_range: detail.len() + 1..detail.len() + 1 + name.len(),
                    });
                }
            }
            Some(lsp::CompletionItemKind::METHOD) => {
                if let Some((name, detail)) = completion.label.split_once(" : ") {
                    let text = format!("{detail} {name}");
                    let source = Rope::from(format!("{} {{}}", text).as_str());
                    let runs = language.highlight_text(&source, 0..text.len());

                    return Some(CodeLabel {
                        text,
                        runs,
                        filter_range: detail.len() + 1..detail.len() + 1 + name.rfind('(')?,
                    });
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
