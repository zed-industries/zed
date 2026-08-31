use anyhow::{Context as _, Result, ensure};
use collections::HashMap;
use db::kvp::KeyValueStore;
use gpui::{App, AppContext as _, Context, Entity, Global, Task};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::PathBuf};
use util::ResultExt as _;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct ReviewScope {
    pub repository: PathBuf,
    pub worktree: PathBuf,
    pub branch: String,
    pub branch_generation: String,
    pub base_ref: String,
}

impl ReviewScope {
    pub fn key(&self) -> Result<String> {
        Ok(format!(
            "branch_review_v1:{}",
            digest(&[&serde_json::to_vec(self)?])
        ))
    }
}

pub(crate) fn digest(parts: &[&[u8]]) -> String {
    let mut hash = Sha256::new();
    for part in parts {
        hash.update((part.len() as u64).to_le_bytes());
        hash.update(part);
    }
    format!("{:x}", hash.finalize())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Fingerprint(String);

impl Fingerprint {
    pub fn new(
        path: &str,
        base: Option<&[u8]>,
        current: Option<&[u8]>,
        base_mode: u32,
        current_mode: u32,
    ) -> Self {
        Self(digest(&[
            b"merge_base_comparison_v1",
            path.as_bytes(),
            &[base.is_some() as u8, current.is_some() as u8],
            base.unwrap_or_default(),
            current.unwrap_or_default(),
            &base_mode.to_le_bytes(),
            &current_mode.to_le_bytes(),
        ]))
    }
    pub fn with_rename(self, source: Option<&str>) -> Self {
        match source {
            Some(source) => Self(digest(&[self.0.as_bytes(), b"rename", source.as_bytes()])),
            None => self,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ReviewRecords {
    schema_version: u32,
    viewed: BTreeMap<String, Fingerprint>,
}

impl Default for ReviewRecords {
    fn default() -> Self {
        Self {
            schema_version: 1,
            viewed: BTreeMap::new(),
        }
    }
}

impl ReviewRecords {
    fn restore(value: Option<&str>) -> Result<Self> {
        let records = value
            .map(serde_json::from_str::<Self>)
            .transpose()
            .context("Unable to read saved Branch Review progress")?
            .unwrap_or_default();
        ensure!(
            records.schema_version == 1,
            "Unsupported Branch Review state version"
        );
        Ok(records)
    }

    fn is_viewed(&self, path: &str, fingerprint: &Fingerprint) -> bool {
        self.viewed.get(path) == Some(fingerprint)
    }
}

#[derive(Default)]
struct ReviewStates(HashMap<String, Entity<ReviewState>>);
impl Global for ReviewStates {}

pub(crate) struct ReviewState {
    key: String,
    database: KeyValueStore,
    records: ReviewRecords,
    pub error: Option<String>,
    pub saving: bool,
    load_failed: bool,
    revision: u64,
    write_task: Option<Task<()>>,
    _quit_subscription: Option<gpui::Subscription>,
}

impl ReviewState {
    pub fn for_scope(scope: &ReviewScope, cx: &mut App) -> Result<Entity<Self>> {
        Self::for_key(scope.key()?, cx)
    }

    pub fn for_key(key: String, cx: &mut App) -> Result<Entity<Self>> {
        if !cx.has_global::<ReviewStates>() {
            cx.set_global(ReviewStates::default());
        }
        if let Some(state) = cx.global::<ReviewStates>().0.get(&key).cloned() {
            if state.read(cx).error.is_some() {
                state.update(cx, |state, cx| state.retry(cx));
            }
            return Ok(state);
        }
        let database = KeyValueStore::global(cx);
        #[cfg(not(any(test, feature = "test-support")))]
        ensure!(
            database.persistent(),
            "Branch Review requires persistent application storage"
        );
        let state = cx.new(|cx: &mut Context<Self>| {
            let mut state = Self::load(key.clone(), database);
            state._quit_subscription = Some(cx.on_app_quit(|state, _| {
                let pending = state.write_task.take();
                async move {
                    if let Some(pending) = pending {
                        pending.await;
                    }
                }
            }));
            state
        });
        cx.global_mut::<ReviewStates>().0.insert(key, state.clone());
        Ok(state)
    }

    fn load(key: String, database: KeyValueStore) -> Self {
        let restored = database
            .read_kvp(&key)
            .and_then(|value| ReviewRecords::restore(value.as_deref()));
        let (records, error) = match restored {
            Ok(records) => (records, None),
            Err(error) => (ReviewRecords::default(), Some(format!("{error:#}"))),
        };
        let load_failed = error.is_some();
        Self {
            key,
            database,
            records,
            error,
            saving: false,
            load_failed,
            revision: 0,
            write_task: None,
            _quit_subscription: None,
        }
    }

    fn retry(&mut self, cx: &mut Context<Self>) {
        if self.load_failed {
            match self
                .database
                .read_kvp(&self.key)
                .and_then(|value| ReviewRecords::restore(value.as_deref()))
            {
                Ok(records) => {
                    self.records = records;
                    self.load_failed = false;
                    self.error = None;
                }
                Err(error) => self.error = Some(format!("{error:#}")),
            }
            cx.notify();
        } else if !self.saving {
            self.persist(cx);
        }
    }

    pub fn is_viewed(&self, path: &str, fingerprint: &Fingerprint) -> bool {
        self.error.is_none() && self.records.is_viewed(path, fingerprint)
    }

    pub fn set_viewed(
        &mut self,
        path: String,
        fingerprint: Option<Fingerprint>,
        cx: &mut Context<Self>,
    ) {
        if self.error.is_some() {
            return;
        }
        match fingerprint {
            Some(fingerprint) => {
                self.records.viewed.insert(path, fingerprint);
            }
            None => {
                self.records.viewed.remove(&path);
            }
        }
        self.persist(cx);
    }

    fn persist(&mut self, cx: &mut Context<Self>) {
        let value = match serde_json::to_string(&self.records) {
            Ok(value) => value,
            Err(error) => {
                self.error = Some(error.to_string());
                cx.notify();
                return;
            }
        };
        self.revision += 1;
        let revision = self.revision;
        let previous_write = self.write_task.take();
        let database = self.database.clone();
        let key = self.key.clone();
        self.saving = true;
        self.write_task = Some(cx.spawn(async move |this, cx| {
            // Keep writes ordered even when several checkboxes are clicked before
            // SQLite finishes. The app-owned entity outlives every review view.
            if let Some(previous_write) = previous_write {
                previous_write.await;
            }
            let result = database.write_kvp(key, value).await;
            this.update(cx, |this, cx| {
                if this.revision == revision {
                    this.saving = false;
                    this.error = result
                        .err()
                        .map(|error| format!("Progress was not saved: {error:#}"));
                    cx.notify();
                }
            })
            .log_err();
        }));
        cx.notify();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fingerprint(path: &str, base: Option<&str>, current: Option<&str>) -> Fingerprint {
        Fingerprint::new(
            path,
            base.map(str::as_bytes),
            current.map(str::as_bytes),
            0o100644,
            0o100644,
        )
    }

    #[test]
    fn selective_invalidation_and_undo() {
        let mut records = ReviewRecords::default();
        let a = fingerprint("a", Some("base a"), Some("reviewed a"));
        let b = fingerprint("b", Some("base b"), Some("reviewed b"));
        records.viewed.insert("a".into(), a.clone());
        records.viewed.insert("b".into(), b.clone());
        assert!(records.is_viewed("a", &a));
        assert!(!records.is_viewed("b", &fingerprint("b", Some("base b"), Some("revised b"))));
        assert!(!records.is_viewed("c", &fingerprint("c", None, Some("new"))));
        assert!(records.is_viewed("b", &b));
        assert!(!records.is_viewed("a", &fingerprint("a", Some("new base"), Some("reviewed a"))));
    }

    #[test]
    fn comparison_identity_includes_absence_paths_and_modes() {
        let original = fingerprint("a", Some(""), Some(""));
        assert_ne!(original, fingerprint("a", None, Some("")));
        assert_ne!(original, fingerprint("a", Some(""), None));
        assert_ne!(original, fingerprint("renamed", Some(""), Some("")));
        assert_ne!(
            original,
            Fingerprint::new("a", Some(b""), Some(b""), 0o100755, 0o100644)
        );
        assert_ne!(
            original,
            Fingerprint::new("a", Some(b""), Some(b""), 0o100644, 0o100755)
        );
        assert_ne!(digest(&[b"ab", b"c"]), digest(&[b"a", b"bc"]));
    }

    #[test]
    fn scopes_isolate_worktrees_branches_reuse_and_base_selection() {
        let scope = ReviewScope {
            repository: "/repo/.git".into(),
            worktree: "/repo".into(),
            branch: "feature".into(),
            branch_generation: "creation".into(),
            base_ref: "main".into(),
        };
        let key = scope.key().unwrap();
        for changed in [
            ReviewScope {
                repository: "/other/.git".into(),
                ..scope.clone()
            },
            ReviewScope {
                worktree: "/worktree".into(),
                ..scope.clone()
            },
            ReviewScope {
                branch: "other".into(),
                ..scope.clone()
            },
            ReviewScope {
                branch_generation: "recreated".into(),
                ..scope.clone()
            },
            ReviewScope {
                base_ref: "release".into(),
                ..scope
            },
        ] {
            assert_ne!(key, changed.key().unwrap());
        }
    }

    #[gpui::test]
    async fn persistence_restores_fingerprints_and_explicit_uncheck() {
        let database = KeyValueStore::open_test_db("branch_review_persistence").await;
        let mut records = ReviewRecords::default();
        let a = fingerprint("a", Some("old"), Some("new"));
        records.viewed.insert("a".into(), a.clone());
        database
            .write_kvp("review".into(), serde_json::to_string(&records).unwrap())
            .await
            .unwrap();
        let restored = ReviewState::load("review".into(), database.clone());
        assert!(restored.is_viewed("a", &a));
        records.viewed.remove("a");
        database
            .write_kvp("review".into(), serde_json::to_string(&records).unwrap())
            .await
            .unwrap();
        assert!(!ReviewState::load("review".into(), database.clone()).is_viewed("a", &a));
        database
            .write_kvp("review".into(), "invalid".into())
            .await
            .unwrap();
        let corrupted = ReviewState::load("review".into(), database);
        assert!(corrupted.error.is_some());
        assert!(!corrupted.is_viewed("a", &a));
        assert!(ReviewRecords::restore(Some(r#"{"schema_version":2,"viewed":{}}"#)).is_err());
    }
    #[gpui::test]
    async fn ordered_writes_failure_and_retry(cx: &mut gpui::TestAppContext) {
        let database = KeyValueStore::open_test_db("branch_review_ordering").await;
        let state = cx.new(|_| ReviewState::load("review".into(), database.clone()));
        let a = fingerprint("a", Some("old"), Some("new"));
        state.update(cx, |state, cx| {
            state.set_viewed("a".into(), Some(a.clone()), cx);
            state.set_viewed("b".into(), Some(a.clone()), cx);
            state.set_viewed("b".into(), None, cx);
        });
        cx.run_until_parked();
        let restored = ReviewState::load("review".into(), database.clone());
        assert!(restored.is_viewed("a", &a));
        assert!(!restored.is_viewed("b", &a));
        database.write(|connection| connection.exec(
            "CREATE TRIGGER fail_review_write BEFORE INSERT ON kv_store BEGIN SELECT RAISE(ABORT, 'injected storage failure'); END"
        )?()).await.unwrap();
        state.update(cx, |state, cx| {
            state.set_viewed("b".into(), Some(a.clone()), cx)
        });
        cx.run_until_parked();
        state.read_with(cx, |state, _| {
            assert!(
                state
                    .error
                    .as_ref()
                    .unwrap()
                    .contains("injected storage failure")
            );
            assert!(!state.is_viewed("a", &a));
            assert!(!state.saving);
        });
        database
            .write(|connection| connection.exec("DROP TRIGGER fail_review_write")?())
            .await
            .unwrap();
        state.update(cx, |state, cx| state.retry(cx));
        cx.run_until_parked();
        let restored = ReviewState::load("review".into(), database);
        assert!(restored.is_viewed("a", &a));
        assert!(restored.is_viewed("b", &a));
    }
}
