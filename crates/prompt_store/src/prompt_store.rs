mod prompts;

use anyhow::{Context as _, Result, anyhow};
use chrono::{DateTime, Utc};
use collections::HashMap;
use futures::FutureExt as _;
use futures::future::Shared;
use fuzzy::StringMatchCandidate;
use gpui::{
    App, AppContext, Context, Entity, EventEmitter, Global, ReadGlobal, SharedString, Task,
};
use heed::{
    Database, RoTxn,
    types::{SerdeBincode, SerdeJson, Str},
};
use parking_lot::RwLock;
pub use prompts::*;
use rope::Rope;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Reverse,
    future::Future,
    path::PathBuf,
    sync::{Arc, atomic::AtomicBool},
};
use text::LineEnding;
use util::ResultExt;
use uuid::Uuid;

/// Init starts loading the PromptStore in the background and assigns
/// a shared future to a global.
pub fn init(cx: &mut App) {
    let db_path = paths::prompts_dir().join("prompts-library-db.0.mdb");
    let prompt_store_task = PromptStore::new(db_path, cx);
    let prompt_store_entity_task = cx
        .spawn(async move |cx| {
            prompt_store_task
                .await
                .and_then(|prompt_store| cx.new(|_cx| prompt_store))
                .map_err(Arc::new)
        })
        .shared();
    cx.set_global(GlobalPromptStore(prompt_store_entity_task))
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
    User { uuid: UserPromptId },
    CommitMessage,
}

impl PromptId {
    pub fn new() -> PromptId {
        UserPromptId::new().into()
    }

    pub fn user_id(&self) -> Option<UserPromptId> {
        match self {
            Self::User { uuid } => Some(*uuid),
            _ => None,
        }
    }

    pub fn is_built_in(&self) -> bool {
        match self {
            Self::User { .. } => false,
            Self::CommitMessage => true,
        }
    }

    pub fn can_edit(&self) -> bool {
        match self {
            Self::User { .. } | Self::CommitMessage => true,
        }
    }

    pub fn default_content(&self) -> Option<&'static str> {
        match self {
            Self::User { .. } => None,
            Self::CommitMessage => Some(include_str!("../../git_ui/src/commit_message_prompt.txt")),
        }
    }
}

impl From<UserPromptId> for PromptId {
    fn from(uuid: UserPromptId) -> Self {
        PromptId::User { uuid }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserPromptId(pub Uuid);

impl UserPromptId {
    pub fn new() -> UserPromptId {
        UserPromptId(Uuid::new_v4())
    }
}

impl From<Uuid> for UserPromptId {
    fn from(uuid: Uuid) -> Self {
        UserPromptId(uuid)
    }
}

impl std::fmt::Display for PromptId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PromptId::User { uuid } => write!(f, "{}", uuid.0),
            PromptId::CommitMessage => write!(f, "Commit message"),
        }
    }
}

pub struct PromptStore {
    env: heed::Env,
    metadata_cache: RwLock<MetadataCache>,
    metadata: Database<SerdeJson<PromptId>, SerdeJson<PromptMetadata>>,
    bodies: Database<SerdeJson<PromptId>, Str>,
}

pub struct PromptsUpdatedEvent;

impl EventEmitter<PromptsUpdatedEvent> for PromptStore {}

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
    pub fn global(cx: &App) -> impl Future<Output = Result<Entity<Self>>> + use<> {
        let store = GlobalPromptStore::global(cx).0.clone();
        async move { store.await.map_err(|err| anyhow!(err)) }
    }

    pub fn new(db_path: PathBuf, cx: &App) -> Task<Result<Self>> {
        cx.background_spawn(async move {
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

            // Insert default commit message prompt if not present
            if metadata.get(&txn, &PromptId::CommitMessage)?.is_none() {
                metadata.put(
                    &mut txn,
                    &PromptId::CommitMessage,
                    &PromptMetadata {
                        id: PromptId::CommitMessage,
                        title: Some("Git Commit Message".into()),
                        default: false,
                        saved_at: Utc::now(),
                    },
                )?;
            }
            if bodies.get(&txn, &PromptId::CommitMessage)?.is_none() {
                let commit_message_prompt =
                    include_str!("../../git_ui/src/commit_message_prompt.txt");
                bodies.put(&mut txn, &PromptId::CommitMessage, commit_message_prompt)?;
            }

            txn.commit()?;

            Self::upgrade_dbs(&db_env, metadata, bodies).log_err();

            let txn = db_env.read_txn()?;
            let metadata_cache = MetadataCache::from_db(metadata, &txn)?;
            txn.commit()?;

            Ok(PromptStore {
                env: db_env,
                metadata_cache: RwLock::new(metadata_cache),
                metadata,
                bodies,
            })
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
            let prompt_id_v2 = UserPromptId(prompt_id_v1.0).into();
            let Some(body_v1) = bodies_v1.remove(&prompt_id_v1) else {
                continue;
            };

            if metadata_db
                .get(&txn, &prompt_id_v2)?
                .is_none_or(|metadata_v2| metadata_v1.saved_at > metadata_v2.saved_at)
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

    pub fn load(&self, id: PromptId, cx: &App) -> Task<Result<String>> {
        let env = self.env.clone();
        let bodies = self.bodies;
        cx.background_spawn(async move {
            let txn = env.read_txn()?;
            let mut prompt = bodies.get(&txn, &id)?.context("prompt not found")?.into();
            LineEnding::normalize(&mut prompt);
            Ok(prompt)
        })
    }

    pub fn all_prompt_metadata(&self) -> Vec<PromptMetadata> {
        self.metadata_cache.read().metadata.clone()
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

    pub fn delete(&self, id: PromptId, cx: &Context<Self>) -> Task<Result<()>> {
        self.metadata_cache.write().remove(id);

        let db_connection = self.env.clone();
        let bodies = self.bodies;
        let metadata = self.metadata;

        let task = cx.background_spawn(async move {
            let mut txn = db_connection.write_txn()?;

            metadata.delete(&mut txn, &id)?;
            bodies.delete(&mut txn, &id)?;

            txn.commit()?;
            anyhow::Ok(())
        });

        cx.spawn(async move |this, cx| {
            task.await?;
            this.update(cx, |_, cx| cx.emit(PromptsUpdatedEvent)).ok();
            anyhow::Ok(())
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

    pub fn search(
        &self,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        cx: &App,
    ) -> Task<Vec<PromptMetadata>> {
        let cached_metadata = self.metadata_cache.read().metadata.clone();
        let executor = cx.background_executor().clone();
        cx.background_spawn(async move {
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
                    true,
                    100,
                    &cancellation_flag,
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
        cx: &Context<Self>,
    ) -> Task<Result<()>> {
        if !id.can_edit() {
            return Task::ready(Err(anyhow!("this prompt cannot be edited")));
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

        let task = cx.background_spawn(async move {
            let mut txn = db_connection.write_txn()?;

            metadata.put(&mut txn, &id, &prompt_metadata)?;
            bodies.put(&mut txn, &id, &body.to_string())?;

            txn.commit()?;

            anyhow::Ok(())
        });

        cx.spawn(async move |this, cx| {
            task.await?;
            this.update(cx, |_, cx| cx.emit(PromptsUpdatedEvent)).ok();
            anyhow::Ok(())
        })
    }

    pub fn save_metadata(
        &self,
        id: PromptId,
        mut title: Option<SharedString>,
        default: bool,
        cx: &Context<Self>,
    ) -> Task<Result<()>> {
        let mut cache = self.metadata_cache.write();

        if !id.can_edit() {
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

        let task = cx.background_spawn(async move {
            let mut txn = db_connection.write_txn()?;
            metadata.put(&mut txn, &id, &prompt_metadata)?;
            txn.commit()?;

            anyhow::Ok(())
        });

        cx.spawn(async move |this, cx| {
            task.await?;
            this.update(cx, |_, cx| cx.emit(PromptsUpdatedEvent)).ok();
            anyhow::Ok(())
        })
    }
}

/// Wraps a shared future to a prompt store so it can be assigned as a context global.
pub struct GlobalPromptStore(Shared<Task<Result<Entity<PromptStore>, Arc<anyhow::Error>>>>);

impl Global for GlobalPromptStore {}
