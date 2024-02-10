use anyhow::{anyhow, Result};
use async_trait::async_trait;
pub use language::*;
use lsp::LanguageServerBinary;
use std::{any::Any, path::PathBuf};

pub struct MetalsLspAdapter;

#[async_trait]
impl LspAdapter for MetalsLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("metals".into())
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
            "metals must be installed and available in your $PATH"
        ))
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        Some(LanguageServerBinary {
            path: "metals".into(),
            env: None,
            arguments: vec![],
        })
    }

    fn can_be_reinstalled(&self) -> bool {
        false
    }

    async fn installation_test_binary(
        &self,
        _container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        None
    }
}

#[cfg(test)]
mod tests {}
