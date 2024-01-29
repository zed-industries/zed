use std::{any::Any, path::PathBuf};

use anyhow::anyhow;
use async_trait::async_trait;
use futures::StreamExt;

use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;

pub struct ScalaLanguageServer;

#[async_trait]
impl LspAdapter for ScalaLanguageServer {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("metals".into())
    }

    fn short_name(&self) -> &'static str {
        "metals"
    }

    async fn fetch_latest_server_version(
        &self,
        _delegate: &dyn LspAdapterDelegate,
    ) -> anyhow::Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(()))
    }

    async fn fetch_server_binary(
        &self,
        _version: Box<dyn 'static + Send + Any>,
        _container_dir: PathBuf,
        _delegate: &dyn LspAdapterDelegate,
    ) -> anyhow::Result<LanguageServerBinary> {
        Err(anyhow!(
            "metals (Scala Language Server) must be installed on the local machine"
        ))
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let metals_binary = container_dir.join("metals");
        Some(LanguageServerBinary {
            path: metals_binary,
            arguments: vec![],
        })
    }

    async fn installation_test_binary(
        &self,
        _container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        None
    }
}