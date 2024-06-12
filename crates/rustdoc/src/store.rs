use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use collections::HashMap;
use fuzzy::StringMatchCandidate;
use gpui::{AppContext, BackgroundExecutor, Global, ReadGlobal, Task, UpdateGlobal};
use parking_lot::RwLock;

use crate::crawler::{RustdocCrawler, RustdocProvider};
use crate::RustdocItem;

struct GlobalRustdocStore(Arc<RustdocStore>);

impl Global for GlobalRustdocStore {}

pub struct RustdocStore {
    executor: BackgroundExecutor,
    docs: Arc<RwLock<HashMap<(String, RustdocItem), String>>>,
}

impl RustdocStore {
    pub fn global(cx: &AppContext) -> Arc<Self> {
        GlobalRustdocStore::global(cx).0.clone()
    }

    pub fn init_global(cx: &mut AppContext) {
        GlobalRustdocStore::set_global(
            cx,
            GlobalRustdocStore(Arc::new(Self::new(cx.background_executor().clone()))),
        );
    }

    pub fn new(executor: BackgroundExecutor) -> Self {
        Self {
            executor,
            docs: Arc::new(RwLock::new(HashMap::default())),
        }
    }

    pub fn load(&self, crate_name: String, item_path: Option<String>) -> Task<Result<String>> {
        let item_docs = self
            .docs
            .read()
            .iter()
            .find_map(|((item_crate_name, item), item_docs)| {
                if item_crate_name == &crate_name && item_path == Some(item.display()) {
                    Some(item_docs.clone())
                } else {
                    None
                }
            });

        Task::ready(item_docs.ok_or_else(|| anyhow!("no docs found")))
    }

    pub fn index(
        &self,
        crate_name: String,
        provider: Box<dyn RustdocProvider + Send + Sync + 'static>,
    ) -> Task<Result<()>> {
        let docs = self.docs.clone();
        self.executor.spawn(async move {
            let crawler = RustdocCrawler::new(provider);

            println!("Indexing {crate_name}");

            let Some(crate_docs) = crawler.crawl(crate_name.clone()).await? else {
                return Ok(());
            };

            let mut lock = docs.write();

            for (item, item_docs) in crate_docs.items {
                lock.insert((crate_name.clone(), item), item_docs);
            }

            Ok(())
        })
    }

    pub fn search(&self, query: String) -> Task<Vec<(String, RustdocItem)>> {
        let executor = self.executor.clone();
        let docs = self.docs.read().clone();
        self.executor.spawn(async move {
            if query.is_empty() {
                return Vec::new();
            }

            let items = docs.keys().collect::<Vec<_>>();

            let candidates = items
                .iter()
                .enumerate()
                .map(|(ix, (crate_name, item))| {
                    StringMatchCandidate::new(ix, format!("{crate_name}::{}", item.display()))
                })
                .collect::<Vec<_>>();

            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                false,
                100,
                &AtomicBool::default(),
                executor,
            )
            .await;

            matches
                .into_iter()
                .map(|mat| items[mat.candidate_id].clone())
                .collect()
        })
    }
}
