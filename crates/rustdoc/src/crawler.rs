use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use collections::{HashSet, VecDeque};
use fs::Fs;
use futures::AsyncReadExt;
use html_to_markdown::convert_rustdoc_to_markdown;
use html_to_markdown::structure::rustdoc::{RustdocItem, RustdocItemKind};
use http::{AsyncBody, HttpClient, HttpClientWithUrl};

#[derive(Debug, Clone, Copy)]
pub enum RustdocSource {
    /// The docs were sourced from local `cargo doc` output.
    Local,
    /// The docs were sourced from `docs.rs`.
    DocsDotRs,
}

#[async_trait]
pub trait RustdocProvider {
    async fn fetch_page(
        &self,
        crate_name: &str,
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
        crate_name: &str,
        item: Option<&RustdocItem>,
    ) -> Result<Option<String>> {
        let mut local_cargo_doc_path = self.cargo_workspace_root.join("target/doc");
        local_cargo_doc_path.push(&crate_name);
        if let Some(item) = item {
            if !item.path.is_empty() {
                local_cargo_doc_path.push(item.path.join("/"));
            }
        }
        local_cargo_doc_path.push("index.html");

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
        crate_name: &str,
        item: Option<&RustdocItem>,
    ) -> Result<Option<String>> {
        let version = "latest";
        let path = format!(
            "{crate_name}/{version}/{crate_name}{item_path}",
            item_path = item
                .map(|item| format!("/{}", item.url_path()))
                .unwrap_or_default()
        );

        println!("Fetching {}", &format!("https://docs.rs/{path}"));

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

pub struct RustdocItemWithHistory {
    pub item: RustdocItem,
    #[cfg(debug_assertions)]
    pub history: Vec<String>,
}

pub struct RustdocCrawler {
    provider: Box<dyn RustdocProvider + Send + Sync + 'static>,
}

impl RustdocCrawler {
    pub fn new(provider: Box<dyn RustdocProvider + Send + Sync + 'static>) -> Self {
        Self { provider }
    }

    pub async fn crawl(&self, crate_name: String) -> Result<Option<String>> {
        let Some(crate_index_content) = self.provider.fetch_page(&crate_name, None).await? else {
            return Ok(None);
        };

        let (_markdown, items) = convert_rustdoc_to_markdown(crate_index_content.as_bytes())?;

        let mut seen_items = HashSet::default();
        let mut items_to_visit: VecDeque<RustdocItemWithHistory> =
            VecDeque::from_iter(items.into_iter().map(|item| RustdocItemWithHistory {
                item,
                #[cfg(debug_assertions)]
                history: Vec::new(),
            }));

        while let Some(item_with_history) = items_to_visit.pop_front() {
            let item = &item_with_history.item;
            println!("Visiting {:?} {:?} {}", &item.kind, &item.path, &item.name);

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

            let (_markdown, mut items) = convert_rustdoc_to_markdown(result.as_bytes())?;

            seen_items.insert(item.clone());

            for child in &mut items {
                child.path.extend(item.path.clone());
                match item.kind {
                    RustdocItemKind::Mod => {
                        child.path.push(item.name.clone());
                    }
                    _ => {}
                }
            }

            let unseen_items = items
                .into_iter()
                .map(|item| RustdocItemWithHistory {
                    #[cfg(debug_assertions)]
                    history: {
                        let mut history = item_with_history.history.clone();
                        history.push(item.url_path());
                        history
                    },
                    item,
                })
                .filter(|item| !seen_items.contains(&item.item));

            items_to_visit.extend(unseen_items);
        }

        Ok(Some(String::new()))
    }
}
