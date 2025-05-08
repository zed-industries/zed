mod item;
mod to_markdown;

use cargo_metadata::MetadataCommand;
use futures::future::BoxFuture;
pub use item::*;
use parking_lot::RwLock;
pub use to_markdown::convert_rustdoc_to_markdown;

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use async_trait::async_trait;
use collections::{HashSet, VecDeque};
use fs::Fs;
use futures::{AsyncReadExt, FutureExt};
use http_client::{AsyncBody, HttpClient, HttpClientWithUrl};

use crate::{IndexedDocsDatabase, IndexedDocsProvider, PackageName, ProviderId};

#[derive(Debug)]
struct RustdocItemWithHistory {
    pub item: RustdocItem,
    #[cfg(debug_assertions)]
    pub history: Vec<String>,
}

pub struct LocalRustdocProvider {
    fs: Arc<dyn Fs>,
    cargo_workspace_root: PathBuf,
}

impl LocalRustdocProvider {
    pub fn id() -> ProviderId {
        ProviderId("rustdoc".into())
    }

    pub fn new(fs: Arc<dyn Fs>, cargo_workspace_root: PathBuf) -> Self {
        Self {
            fs,
            cargo_workspace_root,
        }
    }
}

#[async_trait]
impl IndexedDocsProvider for LocalRustdocProvider {
    fn id(&self) -> ProviderId {
        Self::id()
    }

    fn database_path(&self) -> PathBuf {
        paths::data_dir().join("docs/rust/rustdoc-db.1.mdb")
    }

    async fn suggest_packages(&self) -> Result<Vec<PackageName>> {
        static WORKSPACE_CRATES: LazyLock<RwLock<Option<(BTreeSet<PackageName>, Instant)>>> =
            LazyLock::new(|| RwLock::new(None));

        if let Some((crates, fetched_at)) = &*WORKSPACE_CRATES.read() {
            if fetched_at.elapsed() < Duration::from_secs(300) {
                return Ok(crates.iter().cloned().collect());
            }
        }

        let workspace = MetadataCommand::new()
            .manifest_path(self.cargo_workspace_root.join("Cargo.toml"))
            .exec()
            .context("failed to load cargo metadata")?;

        let workspace_crates = workspace
            .packages
            .into_iter()
            .map(|package| PackageName::from(package.name.as_str()))
            .collect::<BTreeSet<_>>();

        *WORKSPACE_CRATES.write() = Some((workspace_crates.clone(), Instant::now()));

        Ok(workspace_crates.into_iter().collect())
    }

    async fn index(&self, package: PackageName, database: Arc<IndexedDocsDatabase>) -> Result<()> {
        index_rustdoc(package, database, {
            move |crate_name, item| {
                let fs = self.fs.clone();
                let cargo_workspace_root = self.cargo_workspace_root.clone();
                let crate_name = crate_name.clone();
                let item = item.cloned();
                async move {
                    let target_doc_path = cargo_workspace_root.join("target/doc");
                    let mut local_cargo_doc_path = target_doc_path.join(crate_name.as_ref().replace('-', "_"));

                    if !fs.is_dir(&local_cargo_doc_path).await {
                        let cargo_doc_exists_at_all = fs.is_dir(&target_doc_path).await;
                        if cargo_doc_exists_at_all {
                            bail!(
                                "no docs directory for '{crate_name}'. if this is a valid crate name, try running `cargo doc`"
                            );
                        } else {
                            bail!("no cargo doc directory. run `cargo doc`");
                        }
                    }

                    if let Some(item) = item {
                        local_cargo_doc_path.push(item.url_path());
                    } else {
                        local_cargo_doc_path.push("index.html");
                    }

                    let Ok(contents) = fs.load(&local_cargo_doc_path).await else {
                        return Ok(None);
                    };

                    Ok(Some(contents))
                }
                .boxed()
            }
        })
        .await
    }
}

pub struct DocsDotRsProvider {
    http_client: Arc<HttpClientWithUrl>,
}

impl DocsDotRsProvider {
    pub fn id() -> ProviderId {
        ProviderId("docs-rs".into())
    }

    pub fn new(http_client: Arc<HttpClientWithUrl>) -> Self {
        Self { http_client }
    }
}

#[async_trait]
impl IndexedDocsProvider for DocsDotRsProvider {
    fn id(&self) -> ProviderId {
        Self::id()
    }

    fn database_path(&self) -> PathBuf {
        paths::data_dir().join("docs/rust/docs-rs-db.1.mdb")
    }

    async fn suggest_packages(&self) -> Result<Vec<PackageName>> {
        static POPULAR_CRATES: LazyLock<Vec<PackageName>> = LazyLock::new(|| {
            include_str!("./rustdoc/popular_crates.txt")
                .lines()
                .filter(|line| !line.starts_with('#'))
                .map(|line| PackageName::from(line.trim()))
                .collect()
        });

        Ok(POPULAR_CRATES.clone())
    }

    async fn index(&self, package: PackageName, database: Arc<IndexedDocsDatabase>) -> Result<()> {
        index_rustdoc(package, database, {
            move |crate_name, item| {
                let http_client = self.http_client.clone();
                let crate_name = crate_name.clone();
                let item = item.cloned();
                async move {
                    let version = "latest";
                    let path = format!(
                        "{crate_name}/{version}/{crate_name}{item_path}",
                        item_path = item
                            .map(|item| format!("/{}", item.url_path()))
                            .unwrap_or_default()
                    );

                    let mut response = http_client
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
                .boxed()
            }
        })
        .await
    }
}

async fn index_rustdoc(
    package: PackageName,
    database: Arc<IndexedDocsDatabase>,
    fetch_page: impl Fn(
        &PackageName,
        Option<&RustdocItem>,
    ) -> BoxFuture<'static, Result<Option<String>>>
    + Send
    + Sync,
) -> Result<()> {
    let Some(package_root_content) = fetch_page(&package, None).await? else {
        return Ok(());
    };

    let (crate_root_markdown, items) =
        convert_rustdoc_to_markdown(package_root_content.as_bytes())?;

    database
        .insert(package.to_string(), crate_root_markdown)
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

        let Some(result) = fetch_page(&package, Some(item)).await.with_context(|| {
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

        database
            .insert(format!("{package}::{}", item.display()), markdown)
            .await?;

        let parent_item = item;
        for mut item in referenced_items {
            if seen_items.contains(&item) {
                continue;
            }

            seen_items.insert(item.clone());

            item.path.extend(parent_item.path.clone());
            if parent_item.kind == RustdocItemKind::Mod {
                item.path.push(parent_item.name.clone());
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
