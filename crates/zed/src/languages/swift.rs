use anyhow::{anyhow, Result};
use async_trait::async_trait;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use std::{any::Any, ffi::OsString, path::PathBuf};

pub struct SourcekitLspAdapter;

#[async_trait]
impl LspAdapter for SourcekitLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("swift-sourcekit-lsp".into())
    }

    fn short_name(&self) -> &'static str {
        "sourcekit-lsp"
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
            "swift toolchain must be installed and available in your $PATH"
        ))
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_sourcekit_lsp_binary(vec![])
    }

    async fn installation_test_binary(&self, _: PathBuf) -> Option<LanguageServerBinary> {
        get_cached_sourcekit_lsp_binary(vec!["--help".into()])
    }

    fn can_be_reinstalled(&self) -> bool {
        false
    }
}

fn get_cached_sourcekit_lsp_binary(arguments: Vec<OsString>) -> Option<LanguageServerBinary> {
    Some(LanguageServerBinary {
        path: "/usr/bin/sourcekit-lsp".into(),
        arguments,
    })
}
