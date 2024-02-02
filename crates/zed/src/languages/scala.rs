use anyhow::{anyhow, Result};
use async_trait::async_trait;
pub use language::*;
use lsp::LanguageServerBinary;
use std::{any::Any, path::PathBuf, str};

pub struct MetalsLspAdapter;

#[async_trait]
impl LspAdapter for MetalsLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("metals".into())
    }

    fn short_name(&self) -> &'static str {
        "metals"
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
        /* TODO: install using coursier, take inspiration in VS Code metals extension
        Ok(LanguageServerBinary {
            path: PathBuf::from("/Users/mph/.nix-profile/bin/cs"),
            arguments: vec![
                "bootstrap".into(),
                "--java-opt".into(),
                "-XX:+UseG1GC".into(),
                "--java-opt".into(),
                "-XX:+UseStringDeduplication".into(),
                "--java-opt".into(),
                "-Xss4m".into(),
                "--java-opt".into(),
                "-Xms100m".into(),
                "org.scalameta:metals_2.13:1.2.0".into(),
                "-o".into(),
                "metals".into(),
                "-f".into(),
            ],
            }) */
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        Some(LanguageServerBinary {
            path: "metals".into(),
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
