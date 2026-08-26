mod prompts;
pub mod rules_to_skills_migration;

use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use collections::HashMap;
use futures::FutureExt as _;
use futures::future::Shared;

use gpui::{App, AppContext, Entity, Global, ReadGlobal, SharedString, Task};
use heed::{
    Database, RoTxn,
    types::{SerdeBincode, SerdeJson, Str},
};
use parking_lot::RwLock;
pub use prompts::*;

use serde::{Deserialize, Serialize};
use std::{future::Future, path::PathBuf, sync::Arc};
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
    async fn test_built_in_prompt_load(cx: &mut TestAppContext) {
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

        assert!(
            store.read_with(cx, |store, _| {
                store
                    .all_prompt_metadata()
                    .iter()
                    .any(|metadata| metadata.id == commit_message_id)
            }),
            "Built-in prompt should always be in cache"
        );
    }
}
