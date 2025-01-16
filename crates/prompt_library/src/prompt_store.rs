use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use collections::HashMap;
use futures::future::{self, BoxFuture, Shared};
use futures::FutureExt as _;
use fuzzy::StringMatchCandidate;
use gpui::{AppContext, BackgroundExecutor, Global, ReadGlobal, SharedString, Task};
use heed::{
    types::{SerdeBincode, SerdeJson, Str},
    Database, RoTxn,
};
use parking_lot::RwLock;
use rope::Rope;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Reverse,
    future::Future,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};
use text::LineEnding;
use util::ResultExt;
use uuid::Uuid;

/// Init starts loading the PromptStore in the background and assigns
/// a shared future to a global.
pub fn init(cx: &mut AppContext) {
    let db_path = paths::prompts_dir().join("prompts-library-db.0.mdb");
    let prompt_store_future = PromptStore::new(db_path, cx.background_executor().clone())
        .then(|result| future::ready(result.map(Arc::new).map_err(Arc::new)))
        .boxed()
        .shared();
    cx.set_global(GlobalPromptStore(prompt_store_future))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromptMetadata {
    pub id: PromptId,
    pub title: Option<SharedString>,
    pub default: bool,
    pub saved_at: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum PromptId {
    User { uuid: Uuid },
    EditWorkflow,
}

impl PromptId {
    pub fn new() -> PromptId {
        PromptId::User {
            uuid: Uuid::new_v4(),
        }
    }

    pub fn is_built_in(&self) -> bool {
        !matches!(self, PromptId::User { .. })
    }
}

pub struct PromptStore {
    executor: BackgroundExecutor,
    env: heed::Env,
    metadata_cache: RwLock<MetadataCache>,
    metadata: Database<SerdeJson<PromptId>, SerdeJson<PromptMetadata>>,
    bodies: Database<SerdeJson<PromptId>, Str>,
}

#[derive(Default)]
struct MetadataCache {
    metadata: Vec<PromptMetadata>,
    metadata_by_id: HashMap<PromptId, PromptMetadata>,
}

impl MetadataCache {
    fn from_db(
        db: Database<SerdeJson<PromptId>, SerdeJson<PromptMetadata>>,
        txn: &RoTxn,
    ) -> Result<Self> {
        let mut cache = MetadataCache::default();
        for result in db.iter(txn)? {
            let (prompt_id, metadata) = result?;
            cache.metadata.push(metadata.clone());
            cache.metadata_by_id.insert(prompt_id, metadata);
        }
        cache.sort();
        Ok(cache)
    }

    fn insert(&mut self, metadata: PromptMetadata) {
        self.metadata_by_id.insert(metadata.id, metadata.clone());
        if let Some(old_metadata) = self.metadata.iter_mut().find(|m| m.id == metadata.id) {
            *old_metadata = metadata;
        } else {
            self.metadata.push(metadata);
        }
        self.sort();
    }

    fn remove(&mut self, id: PromptId) {
        self.metadata.retain(|metadata| metadata.id != id);
        self.metadata_by_id.remove(&id);
    }

    fn sort(&mut self) {
        self.metadata.sort_unstable_by(|a, b| {
            a.title
                .cmp(&b.title)
                .then_with(|| b.saved_at.cmp(&a.saved_at))
        });
    }
}

impl PromptStore {
    pub fn global(cx: &AppContext) -> impl Future<Output = Result<Arc<Self>>> {
        let store = GlobalPromptStore::global(cx).0.clone();
        async move { store.await.map_err(|err| anyhow!(err)) }
    }

    pub fn new(db_path: PathBuf, executor: BackgroundExecutor) -> Task<Result<Self>> {
        executor.spawn({
            let executor = executor.clone();
            async move {
                std::fs::create_dir_all(&db_path)?;

                let db_env = unsafe {
                    heed::EnvOpenOptions::new()
                        .map_size(1024 * 1024 * 1024) // 1GB
                        .max_dbs(4) // Metadata and bodies (possibly v1 of both as well)
                        .open(db_path)?
                };

                let mut txn = db_env.write_txn()?;
                let metadata = db_env.create_database(&mut txn, Some("metadata.v2"))?;
                let bodies = db_env.create_database(&mut txn, Some("bodies.v2"))?;

                // Remove edit workflow prompt, as we decided to opt into it using
                // a slash command instead.
                metadata.delete(&mut txn, &PromptId::EditWorkflow).ok();
                bodies.delete(&mut txn, &PromptId::EditWorkflow).ok();

                txn.commit()?;

                Self::upgrade_dbs(&db_env, metadata, bodies).log_err();

                let txn = db_env.read_txn()?;
                let metadata_cache = MetadataCache::from_db(metadata, &txn)?;
                txn.commit()?;

                Ok(PromptStore {
                    executor,
                    env: db_env,
                    metadata_cache: RwLock::new(metadata_cache),
                    metadata,
                    bodies,
                })
            }
        })
    }

    fn upgrade_dbs(
        env: &heed::Env,
        metadata_db: heed::Database<SerdeJson<PromptId>, SerdeJson<PromptMetadata>>,
        bodies_db: heed::Database<SerdeJson<PromptId>, Str>,
    ) -> Result<()> {
        #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
        pub struct PromptIdV1(Uuid);

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct PromptMetadataV1 {
            pub id: PromptIdV1,
            pub title: Option<SharedString>,
            pub default: bool,
            pub saved_at: DateTime<Utc>,
        }

        let mut txn = env.write_txn()?;
        let Some(bodies_v1_db) = env
            .open_database::<SerdeBincode<PromptIdV1>, SerdeBincode<String>>(
                &txn,
                Some("bodies"),
            )?
        else {
            return Ok(());
        };
        let mut bodies_v1 = bodies_v1_db
            .iter(&txn)?
            .collect::<heed::Result<HashMap<_, _>>>()?;

        let Some(metadata_v1_db) = env
            .open_database::<SerdeBincode<PromptIdV1>, SerdeBincode<PromptMetadataV1>>(
                &txn,
                Some("metadata"),
            )?
        else {
            return Ok(());
        };
        let metadata_v1 = metadata_v1_db
            .iter(&txn)?
            .collect::<heed::Result<HashMap<_, _>>>()?;

        for (prompt_id_v1, metadata_v1) in metadata_v1 {
            let prompt_id_v2 = PromptId::User {
                uuid: prompt_id_v1.0,
            };
            let Some(body_v1) = bodies_v1.remove(&prompt_id_v1) else {
                continue;
            };

            if metadata_db
                .get(&txn, &prompt_id_v2)?
                .map_or(true, |metadata_v2| {
                    metadata_v1.saved_at > metadata_v2.saved_at
                })
            {
                metadata_db.put(
                    &mut txn,
                    &prompt_id_v2,
                    &PromptMetadata {
                        id: prompt_id_v2,
                        title: metadata_v1.title.clone(),
                        default: metadata_v1.default,
                        saved_at: metadata_v1.saved_at,
                    },
                )?;
                bodies_db.put(&mut txn, &prompt_id_v2, &body_v1)?;
            }
        }

        txn.commit()?;

        Ok(())
    }

    pub fn load(&self, id: PromptId) -> Task<Result<String>> {
        let env = self.env.clone();
        let bodies = self.bodies;
        self.executor.spawn(async move {
            let txn = env.read_txn()?;
            let mut prompt = bodies
                .get(&txn, &id)?
                .ok_or_else(|| anyhow!("prompt not found"))?
                .into();
            LineEnding::normalize(&mut prompt);
            Ok(prompt)
        })
    }

    pub fn default_prompt_metadata(&self) -> Vec<PromptMetadata> {
        return self
            .metadata_cache
            .read()
            .metadata
            .iter()
            .filter(|metadata| metadata.default)
            .cloned()
            .collect::<Vec<_>>();
    }

    pub fn delete(&self, id: PromptId) -> Task<Result<()>> {
        self.metadata_cache.write().remove(id);

        let db_connection = self.env.clone();
        let bodies = self.bodies;
        let metadata = self.metadata;

        self.executor.spawn(async move {
            let mut txn = db_connection.write_txn()?;

            metadata.delete(&mut txn, &id)?;
            bodies.delete(&mut txn, &id)?;

            txn.commit()?;
            Ok(())
        })
    }

    /// Returns the number of prompts in the store.
    pub fn prompt_count(&self) -> usize {
        self.metadata_cache.read().metadata.len()
    }

    pub fn metadata(&self, id: PromptId) -> Option<PromptMetadata> {
        self.metadata_cache.read().metadata_by_id.get(&id).cloned()
    }

    pub fn first(&self) -> Option<PromptMetadata> {
        self.metadata_cache.read().metadata.first().cloned()
    }

    pub fn id_for_title(&self, title: &str) -> Option<PromptId> {
        let metadata_cache = self.metadata_cache.read();
        let metadata = metadata_cache
            .metadata
            .iter()
            .find(|metadata| metadata.title.as_ref().map(|title| &***title) == Some(title))?;
        Some(metadata.id)
    }

    pub fn search(&self, query: String) -> Task<Vec<PromptMetadata>> {
        let cached_metadata = self.metadata_cache.read().metadata.clone();
        let executor = self.executor.clone();
        self.executor.spawn(async move {
            let mut matches = if query.is_empty() {
                cached_metadata
            } else {
                let candidates = cached_metadata
                    .iter()
                    .enumerate()
                    .filter_map(|(ix, metadata)| {
                        Some(StringMatchCandidate::new(ix, metadata.title.as_ref()?))
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
                    .map(|mat| cached_metadata[mat.candidate_id].clone())
                    .collect()
            };
            matches.sort_by_key(|metadata| Reverse(metadata.default));
            matches
        })
    }

    pub fn save(
        &self,
        id: PromptId,
        title: Option<SharedString>,
        default: bool,
        body: Rope,
    ) -> Task<Result<()>> {
        if id.is_built_in() {
            return Task::ready(Err(anyhow!("built-in prompts cannot be saved")));
        }

        let prompt_metadata = PromptMetadata {
            id,
            title,
            default,
            saved_at: Utc::now(),
        };
        self.metadata_cache.write().insert(prompt_metadata.clone());

        let db_connection = self.env.clone();
        let bodies = self.bodies;
        let metadata = self.metadata;

        self.executor.spawn(async move {
            let mut txn = db_connection.write_txn()?;

            metadata.put(&mut txn, &id, &prompt_metadata)?;
            bodies.put(&mut txn, &id, &body.to_string())?;

            txn.commit()?;

            Ok(())
        })
    }

    pub fn save_metadata(
        &self,
        id: PromptId,
        mut title: Option<SharedString>,
        default: bool,
    ) -> Task<Result<()>> {
        let mut cache = self.metadata_cache.write();

        if id.is_built_in() {
            title = cache
                .metadata_by_id
                .get(&id)
                .and_then(|metadata| metadata.title.clone());
        }

        let prompt_metadata = PromptMetadata {
            id,
            title,
            default,
            saved_at: Utc::now(),
        };

        cache.insert(prompt_metadata.clone());

        let db_connection = self.env.clone();
        let metadata = self.metadata;

        self.executor.spawn(async move {
            let mut txn = db_connection.write_txn()?;
            metadata.put(&mut txn, &id, &prompt_metadata)?;
            txn.commit()?;

            Ok(())
        })
    }
}

/// Wraps a shared future to a prompt store so it can be assigned as a context global.
pub struct GlobalPromptStore(
    Shared<BoxFuture<'static, Result<Arc<PromptStore>, Arc<anyhow::Error>>>>,
);

impl Global for GlobalPromptStore {}
