use anyhow::{anyhow, Result};
use async_trait::async_trait;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use std::{any::Any, path::PathBuf};

pub struct DartLanguageServer;

#[async_trait]
impl LspAdapter for DartLanguageServer {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("dart".into())
    }

    fn short_name(&self) -> &'static str {
        "dart"
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(()))
    }

    async fn fetch_server_binary(
        &self,
        _: Box<dyn 'static + Send + Any>,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        Err(anyhow!("dart must me installed from dart.dev/get-dart"))
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        Some(LanguageServerBinary {
            path: "dart".into(),
            env: None,
            arguments: vec!["language-server".into(), "--protocol=lsp".into()],
        })
    }

    fn can_be_reinstalled(&self) -> bool {
        false
    }

    async fn installation_test_binary(&self, _: PathBuf) -> Option<LanguageServerBinary> {
        None
    }
}
