use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use collections::HashMap;
use derive_more::{Deref, Display};
use futures::future::{self, BoxFuture, Shared};
use futures::FutureExt;
use fuzzy::StringMatchCandidate;
use gpui::{AppContext, BackgroundExecutor, Global, ReadGlobal, Task, UpdateGlobal};
use heed::types::SerdeBincode;
use heed::Database;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use util::ResultExt;

use crate::indexer::{RustdocIndexer, RustdocProvider};
use crate::{RustdocItem, RustdocItemKind};

/// The name of a crate.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Deref, Display)]
pub struct CrateName(Arc<str>);

impl From<&str> for CrateName {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

struct GlobalRustdocStore(Arc<RustdocStore>);

impl Global for GlobalRustdocStore {}

pub struct RustdocStore {
    executor: BackgroundExecutor,
    database_future: Shared<BoxFuture<'static, Result<Arc<RustdocDatabase>, Arc<anyhow::Error>>>>,
    indexing_tasks_by_crate:
        RwLock<HashMap<CrateName, Shared<Task<Result<(), Arc<anyhow::Error>>>>>>,
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
        let database_future = executor
            .spawn({
                let executor = executor.clone();
                async move {
                    RustdocDatabase::new(
                        paths::support_dir().join("docs/rust/rustdoc-db.0.mdb"),
                        executor,
                    )
                }
            })
            .then(|result| future::ready(result.map(Arc::new).map_err(Arc::new)))
            .boxed()
            .shared();

        Self {
            executor,
            database_future,
            indexing_tasks_by_crate: RwLock::new(HashMap::default()),
        }
    }

    /// Returns whether the crate with the given name is currently being indexed.
    pub fn is_indexing(&self, crate_name: &CrateName) -> bool {
        self.indexing_tasks_by_crate.read().contains_key(crate_name)
    }

    pub async fn load(
        &self,
        crate_name: CrateName,
        item_path: Option<String>,
    ) -> Result<RustdocDatabaseEntry> {
        self.database_future
            .clone()
            .await
            .map_err(|err| anyhow!(err))?
            .load(crate_name, item_path)
            .await
    }

    pub fn index(
        self: Arc<Self>,
        crate_name: CrateName,
        provider: Box<dyn RustdocProvider + Send + Sync + 'static>,
    ) -> Shared<Task<Result<(), Arc<anyhow::Error>>>> {
        if let Some(existing_task) = self.indexing_tasks_by_crate.read().get(&crate_name) {
            return existing_task.clone();
        }

        let indexing_task = self
            .executor
            .spawn({
                let this = self.clone();
                let crate_name = crate_name.clone();
                async move {
                    let _finally = util::defer({
                        let this = this.clone();
                        let crate_name = crate_name.clone();
                        move || {
                            this.indexing_tasks_by_crate.write().remove(&crate_name);
                        }
                    });

                    let index_task = async {
                        let database = this
                            .database_future
                            .clone()
                            .await
                            .map_err(|err| anyhow!(err))?;
                        let indexer = RustdocIndexer::new(database, provider);

                        indexer.index(crate_name.clone()).await
                    };

                    index_task.await.map_err(Arc::new)
                }
            })
            .shared();

        self.indexing_tasks_by_crate
            .write()
            .insert(crate_name, indexing_task.clone());

        indexing_task
    }

    pub fn search(&self, query: String) -> Task<Vec<String>> {
        let executor = self.executor.clone();
        let database_future = self.database_future.clone();
        self.executor.spawn(async move {
            if query.is_empty() {
                return Vec::new();
            }

            let Some(database) = database_future.await.map_err(|err| anyhow!(err)).log_err() else {
                return Vec::new();
            };

            let Some(items) = database.keys().await.log_err() else {
                return Vec::new();
            };

            let candidates = items
                .iter()
                .enumerate()
                .map(|(ix, item_path)| StringMatchCandidate::new(ix, item_path.clone()))
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

#[derive(Serialize, Deserialize)]
pub enum RustdocDatabaseEntry {
    Crate { docs: String },
    Item { kind: RustdocItemKind, docs: String },
}

impl RustdocDatabaseEntry {
    pub fn docs(&self) -> &str {
        match self {
            Self::Crate { docs } | Self::Item { docs, .. } => &docs,
        }
    }
}

pub(crate) struct RustdocDatabase {
    executor: BackgroundExecutor,
    env: heed::Env,
    entries: Database<SerdeBincode<String>, SerdeBincode<RustdocDatabaseEntry>>,
}

impl RustdocDatabase {
    pub fn new(path: PathBuf, executor: BackgroundExecutor) -> Result<Self> {
        std::fs::create_dir_all(&path)?;

        const ONE_GB_IN_BYTES: usize = 1024 * 1024 * 1024;
        let env = unsafe {
            heed::EnvOpenOptions::new()
                .map_size(ONE_GB_IN_BYTES)
                .max_dbs(1)
                .open(path)?
        };

        let mut txn = env.write_txn()?;
        let entries = env.create_database(&mut txn, Some("rustdoc_entries"))?;
        txn.commit()?;

        Ok(Self {
            executor,
            env,
            entries,
        })
    }

    pub fn keys(&self) -> Task<Result<Vec<String>>> {
        let env = self.env.clone();
        let entries = self.entries;

        self.executor.spawn(async move {
            let txn = env.read_txn()?;
            let mut iter = entries.iter(&txn)?;
            let mut keys = Vec::new();
            while let Some((key, _value)) = iter.next().transpose()? {
                keys.push(key);
            }

            Ok(keys)
        })
    }

    pub fn load(
        &self,
        crate_name: CrateName,
        item_path: Option<String>,
    ) -> Task<Result<RustdocDatabaseEntry>> {
        let env = self.env.clone();
        let entries = self.entries;
        let item_path = if let Some(item_path) = item_path {
            format!("{crate_name}::{item_path}")
        } else {
            crate_name.to_string()
        };

        self.executor.spawn(async move {
            let txn = env.read_txn()?;
            entries
                .get(&txn, &item_path)?
                .ok_or_else(|| anyhow!("no docs found for {item_path}"))
        })
    }

    pub fn insert(
        &self,
        crate_name: CrateName,
        item: Option<&RustdocItem>,
        docs: String,
    ) -> Task<Result<()>> {
        let env = self.env.clone();
        let entries = self.entries;
        let (item_path, entry) = if let Some(item) = item {
            (
                format!("{crate_name}::{}", item.display()),
                RustdocDatabaseEntry::Item {
                    kind: item.kind,
                    docs,
                },
            )
        } else {
            (crate_name.to_string(), RustdocDatabaseEntry::Crate { docs })
        };

        self.executor.spawn(async move {
            let mut txn = env.write_txn()?;
            entries.put(&mut txn, &item_path, &entry)?;
            txn.commit()?;
            Ok(())
        })
    }
}
