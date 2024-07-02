use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use collections::{HashSet, VecDeque};

use crate::{
    convert_rustdoc_to_markdown, IndexedDocsDatabase, PackageName, RustdocItem, RustdocItemKind,
};

#[async_trait]
pub trait IndexedDocsProvider {
    async fn fetch_page(
        &self,
        package: &PackageName,
        item: Option<&RustdocItem>,
    ) -> Result<Option<String>>;
}

#[derive(Debug)]
struct RustdocItemWithHistory {
    pub item: RustdocItem,
    #[cfg(debug_assertions)]
    pub history: Vec<String>,
}

pub(crate) struct DocsIndexer {
    database: Arc<IndexedDocsDatabase>,
    provider: Box<dyn IndexedDocsProvider + Send + Sync + 'static>,
}

impl DocsIndexer {
    pub fn new(
        database: Arc<IndexedDocsDatabase>,
        provider: Box<dyn IndexedDocsProvider + Send + Sync + 'static>,
    ) -> Self {
        Self { database, provider }
    }

    /// Indexes the package with the given name.
    pub async fn index(&self, package: PackageName) -> Result<()> {
        let Some(package_root_content) = self.provider.fetch_page(&package, None).await? else {
            return Ok(());
        };

        let (crate_root_markdown, items) =
            convert_rustdoc_to_markdown(package_root_content.as_bytes())?;

        self.database
            .insert(package.clone(), None, crate_root_markdown)
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
                .fetch_page(&package, Some(&item))
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
                .insert(package.clone(), Some(item), markdown)
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
