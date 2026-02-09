mod prompts;

use anyhow::{Result, anyhow};
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
use strum::{EnumIter, IntoEnumIterator as _};
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
                .map(|prompt_store| cx.new(|_cx| prompt_store))
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

impl PromptMetadata {
    fn builtin(builtin: BuiltInPrompt) -> Self {
        Self {
            id: PromptId::BuiltIn(builtin),
            title: Some(builtin.title().into()),
            default: false,
            saved_at: DateTime::default(),
        }
    }
}

/// Built-in prompts that have default content and can be customized by users.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum BuiltInPrompt {
    CommitMessage,
}

impl BuiltInPrompt {
    pub fn title(&self) -> &'static str {
        match self {
            Self::CommitMessage => "Commit message",
        }
    }

    /// Returns the default content for this built-in prompt.
    pub fn default_content(&self) -> &'static str {
        match self {
            Self::CommitMessage => include_str!("../../git_ui/src/commit_message_prompt.txt"),
        }
    }
}

impl std::fmt::Display for BuiltInPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommitMessage => write!(f, "Commit message"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum PromptId {
    User { uuid: UserPromptId },
    BuiltIn(BuiltInPrompt),
}

impl PromptId {
    pub fn new() -> PromptId {
        UserPromptId::new().into()
    }

    pub fn as_user(&self) -> Option<UserPromptId> {
        match self {
            Self::User { uuid } => Some(*uuid),
            Self::BuiltIn { .. } => None,
        }
    }

    pub fn as_built_in(&self) -> Option<BuiltInPrompt> {
        match self {
            Self::User { .. } => None,
            Self::BuiltIn(builtin) => Some(*builtin),
        }
    }

    pub fn is_built_in(&self) -> bool {
        matches!(self, Self::BuiltIn { .. })
    }

    pub fn can_edit(&self) -> bool {
        match self {
            Self::User { .. } => true,
            Self::BuiltIn(builtin) => match builtin {
                BuiltInPrompt::CommitMessage => true,
            },
        }
    }
}

impl From<BuiltInPrompt> for PromptId {
    fn from(builtin: BuiltInPrompt) -> Self {
        PromptId::BuiltIn(builtin)
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
            PromptId::BuiltIn(builtin) => write!(f, "{}", builtin),
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
            // Fail-open: skip records that can't be decoded (e.g. from a different branch)
            // rather than failing the entire prompt store initialization.
            let Ok((prompt_id, metadata)) = result else {
                log::warn!(
                    "Skipping unreadable prompt record in database: {:?}",
                    result.err()
                );
                continue;
            };
            cache.metadata.push(metadata.clone());
            cache.metadata_by_id.insert(prompt_id, metadata);
        }

        // Insert all the built-in prompts that were not customized by the user
        for builtin in BuiltInPrompt::iter() {
            let builtin_id = PromptId::BuiltIn(builtin);
            if !cache.metadata_by_id.contains_key(&builtin_id) {
                let metadata = PromptMetadata::builtin(builtin);
                cache.metadata.push(metadata.clone());
                cache.metadata_by_id.insert(builtin_id, metadata);
            }
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
            let mut prompt: String = match bodies.get(&txn, &id)? {
                Some(body) => body.into(),
                None => {
                    if let Some(built_in) = id.as_built_in() {
                        built_in.default_content().into()
                    } else {
                        anyhow::bail!("prompt not found")
                    }
                }
            };
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

            if let PromptId::User { uuid } = id {
                let prompt_id_v1 = PromptIdV1::from(uuid);

                if let Some(metadata_v1_db) = db_connection
                    .open_database::<SerdeBincode<PromptIdV1>, SerdeBincode<()>>(
                        &txn,
                        Some("metadata"),
                    )?
                {
                    metadata_v1_db.delete(&mut txn, &prompt_id_v1)?;
                }

                if let Some(bodies_v1_db) = db_connection
                    .open_database::<SerdeBincode<PromptIdV1>, SerdeBincode<()>>(
                        &txn,
                        Some("bodies"),
                    )?
                {
                    bodies_v1_db.delete(&mut txn, &prompt_id_v1)?;
                }
            }

            txn.commit()?;
            anyhow::Ok(())
        });

        cx.spawn(async move |this, cx| {
            task.await?;
            this.update(cx, |_, cx| cx.emit(PromptsUpdatedEvent)).ok();
            anyhow::Ok(())
        })
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

        let body = body.to_string();
        let is_default_content = id
            .as_built_in()
            .is_some_and(|builtin| body.trim() == builtin.default_content().trim());

        let metadata = if let Some(builtin) = id.as_built_in() {
            PromptMetadata::builtin(builtin)
        } else {
            PromptMetadata {
                id,
                title,
                default,
                saved_at: Utc::now(),
            }
        };

        self.metadata_cache.write().insert(metadata.clone());

        let db_connection = self.env.clone();
        let bodies = self.bodies;
        let metadata_db = self.metadata;

        let task = cx.background_spawn(async move {
            let mut txn = db_connection.write_txn()?;

            if is_default_content {
                metadata_db.delete(&mut txn, &id)?;
                bodies.delete(&mut txn, &id)?;
            } else {
                metadata_db.put(&mut txn, &id, &metadata)?;
                bodies.put(&mut txn, &id, &body)?;
            }

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

/// Deprecated: Legacy V1 prompt ID format, used only for migrating data from old databases. Use `PromptId` instead.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
struct PromptIdV1(Uuid);

impl From<UserPromptId> for PromptIdV1 {
    fn from(id: UserPromptId) -> Self {
        PromptIdV1(id.0)
    }
}

/// Deprecated: Legacy V1 prompt metadata format, used only for migrating data from old databases. Use `PromptMetadata` instead.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct PromptMetadataV1 {
    id: PromptIdV1,
    title: Option<SharedString>,
    default: bool,
    saved_at: DateTime<Utc>,
}

/// Wraps a shared future to a prompt store so it can be assigned as a context global.
pub struct GlobalPromptStore(Shared<Task<Result<Entity<PromptStore>, Arc<anyhow::Error>>>>);

impl Global for GlobalPromptStore {}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_built_in_prompt_load_save(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("prompts-db");

        let store = cx.update(|cx| PromptStore::new(db_path, cx)).await.unwrap();
        let store = cx.new(|_cx| store);

        let commit_message_id = PromptId::BuiltIn(BuiltInPrompt::CommitMessage);

        let loaded_content = store
            .update(cx, |store, cx| store.load(commit_message_id, cx))
            .await
            .unwrap();

        let mut expected_content = BuiltInPrompt::CommitMessage.default_content().to_string();
        LineEnding::normalize(&mut expected_content);
        assert_eq!(
            loaded_content.trim(),
            expected_content.trim(),
            "Loading a built-in prompt not in DB should return default content"
        );

        let metadata = store.read_with(cx, |store, _| store.metadata(commit_message_id));
        assert!(
            metadata.is_some(),
            "Built-in prompt should always have metadata"
        );
        assert!(
            store.read_with(cx, |store, _| {
                store
                    .metadata_cache
                    .read()
                    .metadata_by_id
                    .contains_key(&commit_message_id)
            }),
            "Built-in prompt should always be in cache"
        );

        let custom_content = "Custom commit message prompt";
        store
            .update(cx, |store, cx| {
                store.save(
                    commit_message_id,
                    Some("Commit message".into()),
                    false,
                    Rope::from(custom_content),
                    cx,
                )
            })
            .await
            .unwrap();

        let loaded_custom = store
            .update(cx, |store, cx| store.load(commit_message_id, cx))
            .await
            .unwrap();
        assert_eq!(
            loaded_custom.trim(),
            custom_content.trim(),
            "Custom content should be loaded after saving"
        );

        assert!(
            store
                .read_with(cx, |store, _| store.metadata(commit_message_id))
                .is_some(),
            "Built-in prompt should have metadata after customization"
        );

        store
            .update(cx, |store, cx| {
                store.save(
                    commit_message_id,
                    Some("Commit message".into()),
                    false,
                    Rope::from(BuiltInPrompt::CommitMessage.default_content()),
                    cx,
                )
            })
            .await
            .unwrap();

        let metadata_after_reset =
            store.read_with(cx, |store, _| store.metadata(commit_message_id));
        assert!(
            metadata_after_reset.is_some(),
            "Built-in prompt should still have metadata after reset"
        );
        assert_eq!(
            metadata_after_reset
                .as_ref()
                .and_then(|m| m.title.as_ref().map(|t| t.as_ref())),
            Some("Commit message"),
            "Built-in prompt should have default title after reset"
        );

        let loaded_after_reset = store
            .update(cx, |store, cx| store.load(commit_message_id, cx))
            .await
            .unwrap();
        let mut expected_content_after_reset =
            BuiltInPrompt::CommitMessage.default_content().to_string();
        LineEnding::normalize(&mut expected_content_after_reset);
        assert_eq!(
            loaded_after_reset.trim(),
            expected_content_after_reset.trim(),
            "Content should be back to default after saving default content"
        );
    }

    /// Test that the prompt store initializes successfully even when the database
    /// contains records with incompatible/undecodable PromptId keys (e.g., from
    /// a different branch that used a different serialization format).
    ///
    /// This is a regression test for the "fail-open" behavior: we should skip
    /// bad records rather than failing the entire store initialization.
    #[gpui::test]
    async fn test_prompt_store_handles_incompatible_db_records(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("prompts-db-with-bad-records");
        std::fs::create_dir_all(&db_path).unwrap();

        // First, create the DB and write an incompatible record directly.
        // We simulate a record written by a different branch that used
        // `{"kind":"CommitMessage"}` instead of `{"kind":"BuiltIn", ...}`.
        {
            let db_env = unsafe {
                heed::EnvOpenOptions::new()
                    .map_size(1024 * 1024 * 1024)
                    .max_dbs(4)
                    .open(&db_path)
                    .unwrap()
            };

            let mut txn = db_env.write_txn().unwrap();
            // Create the metadata.v2 database with raw bytes so we can write
            // an incompatible key format.
            let metadata_db: Database<heed::types::Bytes, heed::types::Bytes> = db_env
                .create_database(&mut txn, Some("metadata.v2"))
                .unwrap();

            // Write an incompatible PromptId key: `{"kind":"CommitMessage"}`
            // This is the old/branch format that current code can't decode.
            let bad_key = br#"{"kind":"CommitMessage"}"#;
            let dummy_metadata = br#"{"id":{"kind":"CommitMessage"},"title":"Bad Record","default":false,"saved_at":"2024-01-01T00:00:00Z"}"#;
            metadata_db.put(&mut txn, bad_key, dummy_metadata).unwrap();

            // Also write a valid record to ensure we can still read good data.
            let good_key = br#"{"kind":"User","uuid":"550e8400-e29b-41d4-a716-446655440000"}"#;
            let good_metadata = br#"{"id":{"kind":"User","uuid":"550e8400-e29b-41d4-a716-446655440000"},"title":"Good Record","default":false,"saved_at":"2024-01-01T00:00:00Z"}"#;
            metadata_db.put(&mut txn, good_key, good_metadata).unwrap();

            txn.commit().unwrap();
        }

        // Now try to create a PromptStore from this DB.
        // With fail-open behavior, this should succeed and skip the bad record.
        // Without fail-open, this would return an error.
        let store_result = cx.update(|cx| PromptStore::new(db_path, cx)).await;

        assert!(
            store_result.is_ok(),
            "PromptStore should initialize successfully even with incompatible DB records. \
             Got error: {:?}",
            store_result.err()
        );

        let store = cx.new(|_cx| store_result.unwrap());

        // Verify the good record was loaded.
        let good_id = PromptId::User {
            uuid: UserPromptId("550e8400-e29b-41d4-a716-446655440000".parse().unwrap()),
        };
        let metadata = store.read_with(cx, |store, _| store.metadata(good_id));
        assert!(
            metadata.is_some(),
            "Valid records should still be loaded after skipping bad ones"
        );
        assert_eq!(
            metadata
                .as_ref()
                .and_then(|m| m.title.as_ref().map(|t| t.as_ref())),
            Some("Good Record"),
            "Valid record should have correct title"
        );
    }

    #[gpui::test]
    async fn test_deleted_prompt_does_not_reappear_after_migration(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("prompts-db-v1-migration");
        std::fs::create_dir_all(&db_path).unwrap();

        let prompt_uuid: Uuid = "550e8400-e29b-41d4-a716-446655440001".parse().unwrap();
        let prompt_id_v1 = PromptIdV1(prompt_uuid);
        let prompt_id_v2 = PromptId::User {
            uuid: UserPromptId(prompt_uuid),
        };

        // Create V1 database with a prompt
        {
            let db_env = unsafe {
                heed::EnvOpenOptions::new()
                    .map_size(1024 * 1024 * 1024)
                    .max_dbs(4)
                    .open(&db_path)
                    .unwrap()
            };

            let mut txn = db_env.write_txn().unwrap();

            let metadata_v1_db: Database<SerdeBincode<PromptIdV1>, SerdeBincode<PromptMetadataV1>> =
                db_env.create_database(&mut txn, Some("metadata")).unwrap();

            let bodies_v1_db: Database<SerdeBincode<PromptIdV1>, SerdeBincode<String>> =
                db_env.create_database(&mut txn, Some("bodies")).unwrap();

            let metadata_v1 = PromptMetadataV1 {
                id: prompt_id_v1.clone(),
                title: Some("V1 Prompt".into()),
                default: false,
                saved_at: Utc::now(),
            };

            metadata_v1_db
                .put(&mut txn, &prompt_id_v1, &metadata_v1)
                .unwrap();
            bodies_v1_db
                .put(&mut txn, &prompt_id_v1, &"V1 prompt body".to_string())
                .unwrap();

            txn.commit().unwrap();
        }

        // Migrate V1 to V2 by creating PromptStore
        let store = cx
            .update(|cx| PromptStore::new(db_path.clone(), cx))
            .await
            .unwrap();
        let store = cx.new(|_cx| store);

        // Verify the prompt was migrated
        let metadata = store.read_with(cx, |store, _| store.metadata(prompt_id_v2));
        assert!(metadata.is_some(), "V1 prompt should be migrated to V2");
        assert_eq!(
            metadata
                .as_ref()
                .and_then(|m| m.title.as_ref().map(|t| t.as_ref())),
            Some("V1 Prompt"),
            "Migrated prompt should have correct title"
        );

        // Delete the prompt
        store
            .update(cx, |store, cx| store.delete(prompt_id_v2, cx))
            .await
            .unwrap();

        // Verify prompt is deleted
        let metadata_after_delete = store.read_with(cx, |store, _| store.metadata(prompt_id_v2));
        assert!(
            metadata_after_delete.is_none(),
            "Prompt should be deleted from V2"
        );

        drop(store);

        // "Restart" by creating a new PromptStore from the same path
        let store_after_restart = cx.update(|cx| PromptStore::new(db_path, cx)).await.unwrap();
        let store_after_restart = cx.new(|_cx| store_after_restart);

        // Test the prompt does not reappear
        let metadata_after_restart =
            store_after_restart.read_with(cx, |store, _| store.metadata(prompt_id_v2));
        assert!(
            metadata_after_restart.is_none(),
            "Deleted prompt should NOT reappear after restart/migration"
        );
    }
}
