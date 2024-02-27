use std::{any::Any, path::PathBuf};

use async_trait::async_trait;
use language::{LspAdapter, LanguageServerName, LspAdapterDelegate};
use anyhow::{anyhow, Result};
use lsp::LanguageServerBinary;

pub struct MarkdownOxideLanguageServer;

#[async_trait]
impl LspAdapter for MarkdownOxideLanguageServer {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("Markdown Oxide".into())
    }

    fn short_name(&self) -> &'static str {
        "markdown-oxide"
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Any + Send>> {
        Ok(Box::new(()))
    }

    async fn fetch_server_binary(
        &self,
        _version: Box<dyn 'static + Send + Any>,
        _container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        Err(anyhow!(
            "Markdown Oxide must be availiable in $PATH"
        ))
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        Some(LanguageServerBinary {
            path: "markdown-oxide".into(),
            env: None,
            arguments: vec![],
        })
    }

    fn can_be_reinstalled(&self) -> bool {
        false
    }

    async fn installation_test_binary(&self, _: PathBuf) -> Option<LanguageServerBinary> {
        None
    }
}
