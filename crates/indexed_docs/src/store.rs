use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use derive_more::{Deref, Display};
use futures::FutureExt;
use futures::future::{self, BoxFuture, Shared};
use fuzzy::StringMatchCandidate;
use gpui::{App, BackgroundExecutor, Task};
use heed::Database;
use heed::types::SerdeBincode;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use util::ResultExt;

use crate::IndexedDocsRegistry;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Deref, Display)]
pub struct ProviderId(pub Arc<str>);

/// The name of a package.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Deref, Display)]
pub struct PackageName(Arc<str>);

impl From<&str> for PackageName {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

#[async_trait]
pub trait IndexedDocsProvider {
    /// Returns the ID of this provider.
    fn id(&self) -> ProviderId;

    /// Returns the path to the database for this provider.
    fn database_path(&self) -> PathBuf;

    /// Returns a list of packages as suggestions to be included in the search
    /// results.
    ///
    /// This can be used to provide completions for known packages (e.g., from the
    /// local project or a registry) before a package has been indexed.
    async fn suggest_packages(&self) -> Result<Vec<PackageName>>;

    /// Indexes the package with the given name.
    async fn index(&self, package: PackageName, database: Arc<IndexedDocsDatabase>) -> Result<()>;
}

/// A store for indexed docs.
pub struct IndexedDocsStore {
    executor: BackgroundExecutor,
    database_future:
        Shared<BoxFuture<'static, Result<Arc<IndexedDocsDatabase>, Arc<anyhow::Error>>>>,
    provider: Box<dyn IndexedDocsProvider + Send + Sync + 'static>,
    indexing_tasks_by_package:
        RwLock<HashMap<PackageName, Shared<Task<Result<(), Arc<anyhow::Error>>>>>>,
    latest_errors_by_package: RwLock<HashMap<PackageName, Arc<str>>>,
}

impl IndexedDocsStore {
    pub fn try_global(provider: ProviderId, cx: &App) -> Result<Arc<Self>> {
        let registry = IndexedDocsRegistry::global(cx);
        registry
            .get_provider_store(provider.clone())
            .ok_or_else(|| anyhow!("no indexed docs store found for {provider}"))
    }

    pub fn new(
        provider: Box<dyn IndexedDocsProvider + Send + Sync + 'static>,
        executor: BackgroundExecutor,
    ) -> Self {
        let database_future = executor
            .spawn({
                let executor = executor.clone();
                let database_path = provider.database_path();
                async move { IndexedDocsDatabase::new(database_path, executor) }
            })
            .then(|result| future::ready(result.map(Arc::new).map_err(Arc::new)))
            .boxed()
            .shared();

        Self {
            executor,
            database_future,
            provider,
            indexing_tasks_by_package: RwLock::new(HashMap::default()),
            latest_errors_by_package: RwLock::new(HashMap::default()),
        }
    }

    pub fn latest_error_for_package(&self, package: &PackageName) -> Option<Arc<str>> {
        self.latest_errors_by_package.read().get(package).cloned()
    }

    /// Returns whether the package with the given name is currently being indexed.
    pub fn is_indexing(&self, package: &PackageName) -> bool {
        self.indexing_tasks_by_package.read().contains_key(package)
    }

    pub async fn load(&self, key: String) -> Result<MarkdownDocs> {
        self.database_future
            .clone()
            .await
            .map_err(|err| anyhow!(err))?
            .load(key)
            .await
    }

    pub async fn load_many_by_prefix(&self, prefix: String) -> Result<Vec<(String, MarkdownDocs)>> {
        self.database_future
            .clone()
            .await
            .map_err(|err| anyhow!(err))?
            .load_many_by_prefix(prefix)
            .await
    }

    /// Returns whether any entries exist with the given prefix.
    pub async fn any_with_prefix(&self, prefix: String) -> Result<bool> {
        self.database_future
            .clone()
            .await
            .map_err(|err| anyhow!(err))?
            .any_with_prefix(prefix)
            .await
    }

    pub fn suggest_packages(self: Arc<Self>) -> Task<Result<Vec<PackageName>>> {
        let this = self.clone();
        self.executor
            .spawn(async move { this.provider.suggest_packages().await })
    }

    pub fn index(
        self: Arc<Self>,
        package: PackageName,
    ) -> Shared<Task<Result<(), Arc<anyhow::Error>>>> {
        if let Some(existing_task) = self.indexing_tasks_by_package.read().get(&package) {
            return existing_task.clone();
        }

        let indexing_task = self
            .executor
            .spawn({
                let this = self.clone();
                let package = package.clone();
                async move {
                    let _finally = util::defer({
                        let this = this.clone();
                        let package = package.clone();
                        move || {
                            this.indexing_tasks_by_package.write().remove(&package);
                        }
                    });

                    let index_task = {
                        let package = package.clone();
                        async {
                            let database = this
                                .database_future
                                .clone()
                                .await
                                .map_err(|err| anyhow!(err))?;
                            this.provider.index(package, database).await
                        }
                    };

                    let result = index_task.await.map_err(Arc::new);
                    match &result {
                        Ok(_) => {
                            this.latest_errors_by_package.write().remove(&package);
                        }
                        Err(err) => {
                            this.latest_errors_by_package
                                .write()
                                .insert(package, err.to_string().into());
                        }
                    }

                    result
                }
            })
            .shared();

        self.indexing_tasks_by_package
            .write()
            .insert(package, indexing_task.clone());

        indexing_task
    }

    pub fn search(&self, query: String) -> Task<Vec<String>> {
        let executor = self.executor.clone();
        let database_future = self.database_future.clone();
        self.executor.spawn(async move {
            let Some(database) = database_future.await.map_err(|err| anyhow!(err)).log_err() else {
                return Vec::new();
            };

            let Some(items) = database.keys().await.log_err() else {
                return Vec::new();
            };

            let candidates = items
                .iter()
                .enumerate()
                .map(|(ix, item_path)| StringMatchCandidate::new(ix, &item_path))
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

#[derive(Debug, PartialEq, Eq, Clone, Display, Serialize, Deserialize)]
pub struct MarkdownDocs(pub String);

pub struct IndexedDocsDatabase {
    executor: BackgroundExecutor,
    env: heed::Env,
    entries: Database<SerdeBincode<String>, SerdeBincode<MarkdownDocs>>,
}

impl IndexedDocsDatabase {
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

    pub fn load(&self, key: String) -> Task<Result<MarkdownDocs>> {
        let env = self.env.clone();
        let entries = self.entries;

        self.executor.spawn(async move {
            let txn = env.read_txn()?;
            entries
                .get(&txn, &key)?
                .ok_or_else(|| anyhow!("no docs found for {key}"))
        })
    }

    pub fn load_many_by_prefix(&self, prefix: String) -> Task<Result<Vec<(String, MarkdownDocs)>>> {
        let env = self.env.clone();
        let entries = self.entries;

        self.executor.spawn(async move {
            let txn = env.read_txn()?;
            let results = entries
                .iter(&txn)?
                .filter_map(|entry| {
                    let (key, value) = entry.ok()?;
                    if key.starts_with(&prefix) {
                        Some((key, value))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            Ok(results)
        })
    }

    /// Returns whether any entries exist with the given prefix.
    pub fn any_with_prefix(&self, prefix: String) -> Task<Result<bool>> {
        let env = self.env.clone();
        let entries = self.entries;

        self.executor.spawn(async move {
            let txn = env.read_txn()?;
            let any = entries
                .iter(&txn)?
                .any(|entry| entry.map_or(false, |(key, _value)| key.starts_with(&prefix)));
            Ok(any)
        })
    }

    pub fn insert(&self, key: String, docs: String) -> Task<Result<()>> {
        let env = self.env.clone();
        let entries = self.entries;

        self.executor.spawn(async move {
            let mut txn = env.write_txn()?;
            entries.put(&mut txn, &key, &MarkdownDocs(docs))?;
            txn.commit()?;
            Ok(())
        })
    }
}

impl extension::KeyValueStoreDelegate for IndexedDocsDatabase {
    fn insert(&self, key: String, docs: String) -> Task<Result<()>> {
        IndexedDocsDatabase::insert(&self, key, docs)
    }
}
