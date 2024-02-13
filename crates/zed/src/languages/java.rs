use anyhow::{anyhow, Result};
use async_trait::async_trait;
pub use language::*;
use lsp::LanguageServerBinary;
use std::{any::Any, path::PathBuf, str};
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
}
