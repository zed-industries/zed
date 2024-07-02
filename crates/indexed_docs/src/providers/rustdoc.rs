mod item;
mod to_markdown;

pub use item::*;
pub use to_markdown::convert_rustdoc_to_markdown;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use fs::Fs;
use futures::AsyncReadExt;
use http::{AsyncBody, HttpClient, HttpClientWithUrl};

use crate::indexer::IndexedDocsProvider;
use crate::PackageName;

#[derive(Debug, Clone, Copy)]
pub enum RustdocSource {
    /// The docs were sourced from Zed's rustdoc index.
    Index,
    /// The docs were sourced from local `cargo doc` output.
    Local,
    /// The docs were sourced from `docs.rs`.
    DocsDotRs,
}

pub struct LocalProvider {
    fs: Arc<dyn Fs>,
    cargo_workspace_root: PathBuf,
}

impl LocalProvider {
    pub fn new(fs: Arc<dyn Fs>, cargo_workspace_root: PathBuf) -> Self {
        Self {
            fs,
            cargo_workspace_root,
        }
    }
}

#[async_trait]
impl IndexedDocsProvider for LocalProvider {
    async fn fetch_page(
        &self,
        crate_name: &PackageName,
        item: Option<&RustdocItem>,
    ) -> Result<Option<String>> {
        let mut local_cargo_doc_path = self.cargo_workspace_root.join("target/doc");
        local_cargo_doc_path.push(crate_name.as_ref());
        if let Some(item) = item {
            local_cargo_doc_path.push(item.url_path());
        } else {
            local_cargo_doc_path.push("index.html");
        }

        let Ok(contents) = self.fs.load(&local_cargo_doc_path).await else {
            return Ok(None);
        };

        Ok(Some(contents))
    }
}

pub struct DocsDotRsProvider {
    http_client: Arc<HttpClientWithUrl>,
}

impl DocsDotRsProvider {
    pub fn new(http_client: Arc<HttpClientWithUrl>) -> Self {
        Self { http_client }
    }
}

#[async_trait]
impl IndexedDocsProvider for DocsDotRsProvider {
    async fn fetch_page(
        &self,
        crate_name: &PackageName,
        item: Option<&RustdocItem>,
    ) -> Result<Option<String>> {
        let version = "latest";
        let path = format!(
            "{crate_name}/{version}/{crate_name}{item_path}",
            item_path = item
                .map(|item| format!("/{}", item.url_path()))
                .unwrap_or_default()
        );

        let mut response = self
            .http_client
            .get(
                &format!("https://docs.rs/{path}"),
                AsyncBody::default(),
                true,
            )
            .await?;

        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .context("error reading docs.rs response body")?;

        if response.status().is_client_error() {
            let text = String::from_utf8_lossy(body.as_slice());
            bail!(
                "status error {}, response: {text:?}",
                response.status().as_u16()
            );
        }

        Ok(Some(String::from_utf8(body)?))
    }
}
