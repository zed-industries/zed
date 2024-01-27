use std::{any::Any, path::PathBuf};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;

pub struct OCamlLspAdapter;

#[async_trait]
impl LspAdapter for OCamlLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("ocamllsp".into())
    }

    fn short_name(&self) -> &'static str {
        "ocaml"
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
        Err(anyhow!(
            "ocamllsp (ocaml-language-server) must be installed via opam in your current switch. If it currently is, please reopen Zed from a terminal with the switch containing ocamllsp activated."
        ))
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        Some(LanguageServerBinary {
            path: "ocamllsp".into(),
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
