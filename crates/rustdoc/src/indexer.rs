use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use collections::{HashSet, VecDeque};
use fs::Fs;
use futures::AsyncReadExt;
use http::{AsyncBody, HttpClient, HttpClientWithUrl};

use crate::{
    convert_rustdoc_to_markdown, CrateName, RustdocDatabase, RustdocItem, RustdocItemKind,
};

#[derive(Debug, Clone, Copy)]
pub enum RustdocSource {
    /// The docs were sourced from Zed's rustdoc index.
    Index,
    /// The docs were sourced from local `cargo doc` output.
    Local,
    /// The docs were sourced from `docs.rs`.
    DocsDotRs,
}

#[async_trait]
pub trait RustdocProvider {
    async fn fetch_page(
        &self,
        crate_name: &CrateName,
        item: Option<&RustdocItem>,
    ) -> Result<Option<String>>;
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
impl RustdocProvider for LocalProvider {
    async fn fetch_page(
        &self,
        crate_name: &CrateName,
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
impl RustdocProvider for DocsDotRsProvider {
    async fn fetch_page(
        &self,
        crate_name: &CrateName,
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

#[derive(Debug)]
struct RustdocItemWithHistory {
    pub item: RustdocItem,
    #[cfg(debug_assertions)]
    pub history: Vec<String>,
}

pub(crate) struct RustdocIndexer {
    database: Arc<RustdocDatabase>,
    provider: Box<dyn RustdocProvider + Send + Sync + 'static>,
}

impl RustdocIndexer {
    pub fn new(
        database: Arc<RustdocDatabase>,
        provider: Box<dyn RustdocProvider + Send + Sync + 'static>,
    ) -> Self {
        Self { database, provider }
    }

    /// Indexes the crate with the given name.
    pub async fn index(&self, crate_name: CrateName) -> Result<()> {
        let Some(crate_root_content) = self.provider.fetch_page(&crate_name, None).await? else {
            return Ok(());
        };

        let (crate_root_markdown, items) =
            convert_rustdoc_to_markdown(crate_root_content.as_bytes())?;

        self.database
            .insert(crate_name.clone(), None, crate_root_markdown)
            .await?;

        let mut seen_items = HashSet::from_iter(items.clone());
        let mut items_to_visit: VecDeque<RustdocItemWithHistory> =
            VecDeque::from_iter(items.into_iter().map(|item| RustdocItemWithHistory {
                item,
                #[cfg(debug_assertions)]
                history: Vec::new(),
            }));

        while let Some(item_with_history) = items_to_visit.pop_front() {
            let item = &item_with_history.item;

            let Some(result) = self
                .provider
                .fetch_page(&crate_name, Some(&item))
                .await
                .with_context(|| {
                    #[cfg(debug_assertions)]
                    {
                        format!(
                            "failed to fetch {item:?}: {history:?}",
                            history = item_with_history.history
                        )
                    }

                    #[cfg(not(debug_assertions))]
                    {
                        format!("failed to fetch {item:?}")
                    }
                })?
            else {
                continue;
            };

            let (markdown, referenced_items) = convert_rustdoc_to_markdown(result.as_bytes())?;

            self.database
                .insert(crate_name.clone(), Some(item), markdown)
                .await?;

            let parent_item = item;
            for mut item in referenced_items {
                if seen_items.contains(&item) {
                    continue;
                }

                seen_items.insert(item.clone());

                item.path.extend(parent_item.path.clone());
                match parent_item.kind {
                    RustdocItemKind::Mod => {
                        item.path.push(parent_item.name.clone());
                    }
                    _ => {}
                }

                items_to_visit.push_back(RustdocItemWithHistory {
                    #[cfg(debug_assertions)]
                    history: {
                        let mut history = item_with_history.history.clone();
                        history.push(item.url_path());
                        history
                    },
                    item,
                });
            }
        }

        Ok(())
    }
}
