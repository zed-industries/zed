use anyhow::Result;
use client::http::HttpClient;
use futures::future::BoxFuture;
pub use language::*;
use std::{any::Any, path::PathBuf, sync::Arc};

pub struct CppLspAdapter;

impl super::LspAdapter for CppLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("clangd".into())
    }

    fn fetch_latest_server_version(
        &self,
        http: Arc<dyn HttpClient>,
    ) -> BoxFuture<'static, Result<Box<dyn 'static + Send + Any>>> {
        super::c::CLspAdapter.fetch_latest_server_version(http)
    }

    fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        http: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> BoxFuture<'static, Result<PathBuf>> {
        super::c::CLspAdapter.fetch_server_binary(version, http, container_dir)
    }

    fn cached_server_binary(&self, container_dir: PathBuf) -> BoxFuture<'static, Option<PathBuf>> {
        super::c::CLspAdapter.cached_server_binary(container_dir)
    }
}
