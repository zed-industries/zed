use anyhow::{Context as _, Result, ensure};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use collections::HashMap;
use db::kvp::KeyValueStore;
use gpui::{App, AppContext as _, Context, Entity, Global, Task};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, io::Read as _, path::PathBuf};
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

const MAX_SNAPSHOT_BYTES: usize = 2 * 1024 * 1024;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StoredText {
    uncompressed_bytes: usize,
    zstd_base64: String,
}

impl StoredText {
    fn encode(text: &str) -> Result<Self> {
        ensure!(
            text.len() <= MAX_SNAPSHOT_BYTES,
            "Reviewed text exceeds the 2 MiB delta limit"
        );
        let compressed = zstd::encode_all(text.as_bytes(), 3)
            .context("Unable to compress the Viewed snapshot")?;
        Ok(Self {
            uncompressed_bytes: text.len(),
            zstd_base64: BASE64.encode(compressed),
        })
    }

    fn decode(&self) -> Result<String> {
        ensure!(
            self.uncompressed_bytes <= MAX_SNAPSHOT_BYTES,
            "Viewed snapshot exceeds the 2 MiB delta limit"
        );
        ensure!(
            self.zstd_base64.len() <= 4 * 1024 * 1024,
            "Viewed snapshot payload is too large"
        );
        let compressed = BASE64
            .decode(&self.zstd_base64)
            .context("Viewed snapshot is not valid base64")?;
        let decoder = zstd::Decoder::new(compressed.as_slice())
            .context("Unable to decompress the Viewed snapshot")?;
        let mut decoded = Vec::with_capacity(self.uncompressed_bytes);
        decoder
            .take((MAX_SNAPSHOT_BYTES + 1) as u64)
            .read_to_end(&mut decoded)
            .context("Unable to decompress the Viewed snapshot")?;
        ensure!(
            decoded.len() == self.uncompressed_bytes,
            "Viewed snapshot length does not match its metadata"
        );
        ensure!(
            decoded.len() <= MAX_SNAPSHOT_BYTES,
            "Viewed snapshot exceeds the 2 MiB delta limit"
        );
        String::from_utf8(decoded).context("Viewed snapshot is not valid UTF-8")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ApprovedComparison {
    base: Option<StoredText>,
    current: Option<StoredText>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum ApprovedSnapshot {
    Captured { comparison: ApprovedComparison },
    TooLarge,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SnapshotAvailability {
    Available {
        base: Option<String>,
        current: Option<String>,
    },
    TooLarge,
    Legacy,
}

impl ApprovedSnapshot {
    fn capture(base: Option<&str>, current: Option<&str>) -> Result<Self> {
        if base.is_some_and(|text| text.len() > MAX_SNAPSHOT_BYTES)
            || current.is_some_and(|text| text.len() > MAX_SNAPSHOT_BYTES)
        {
            return Ok(Self::TooLarge);
        }
        Ok(Self::Captured {
            comparison: ApprovedComparison {
                base: base.map(StoredText::encode).transpose()?,
                current: current.map(StoredText::encode).transpose()?,
            },
        })
    }

    fn decode(&self) -> Result<SnapshotAvailability> {
        match self {
            Self::Captured { comparison } => Ok(SnapshotAvailability::Available {
                base: comparison
                    .base
                    .as_ref()
                    .map(StoredText::decode)
                    .transpose()?,
                current: comparison
                    .current
                    .as_ref()
                    .map(StoredText::decode)
                    .transpose()?,
            }),
            Self::TooLarge => Ok(SnapshotAvailability::TooLarge),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(from = "SavedFingerprint")]
pub(crate) struct Fingerprint {
    value: String,
    details: Option<ComparisonDetails>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum SavedFingerprint {
    Legacy(String),
    Detailed {
        value: String,
        details: Option<ComparisonDetails>,
    },
}

impl From<SavedFingerprint> for Fingerprint {
    fn from(saved: SavedFingerprint) -> Self {
        match saved {
            SavedFingerprint::Legacy(value) => Self {
                value,
                details: None,
            },
            SavedFingerprint::Detailed { value, details } => Self { value, details },
        }
    }
}

impl PartialEq for Fingerprint {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for Fingerprint {}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ComparisonDetails {
    base_content: String,
    current_content: String,
    base_exists: bool,
    current_exists: bool,
    base_mode: u32,
    current_mode: u32,
    renamed_from: Option<String>,
}

impl Fingerprint {
    pub fn new(
        path: &str,
        base: Option<&[u8]>,
        current: Option<&[u8]>,
        base_mode: u32,
        current_mode: u32,
    ) -> Self {
        Self {
            value: digest(&[
                b"merge_base_comparison_v1",
                path.as_bytes(),
                &[base.is_some() as u8, current.is_some() as u8],
                base.unwrap_or_default(),
                current.unwrap_or_default(),
                &base_mode.to_le_bytes(),
                &current_mode.to_le_bytes(),
            ]),
            details: Some(ComparisonDetails {
                base_content: digest(&[base.unwrap_or_default()]),
                current_content: digest(&[current.unwrap_or_default()]),
                base_exists: base.is_some(),
                current_exists: current.is_some(),
                base_mode,
                current_mode,
                renamed_from: None,
            }),
        }
    }
    pub fn with_rename(mut self, source: Option<&str>) -> Self {
        if let Some(source) = source {
            self.value = digest(&[self.value.as_bytes(), b"rename", source.as_bytes()]);
            if let Some(details) = &mut self.details {
                details.renamed_from = Some(source.to_owned());
            }
        }
        self
    }

    fn changes_from(&self, approved: &Self) -> Vec<&'static str> {
        if self == approved {
            return Vec::new();
        }
        let Some((current, previous)) = self.details.as_ref().zip(approved.details.as_ref()) else {
            return vec!["Comparison changed"];
        };
        let mut reasons = Vec::new();
        if current.base_content != previous.base_content {
            reasons.push("Base changed");
        }
        if current.current_content != previous.current_content {
            reasons.push("Content changed");
        }
        if current.base_exists != previous.base_exists
            || current.current_exists != previous.current_exists
        {
            reasons.push("File added/deleted");
        }
        if current.base_mode != previous.base_mode || current.current_mode != previous.current_mode
        {
            reasons.push("Mode changed");
        }
        if current.renamed_from != previous.renamed_from {
            reasons.push("Rename changed");
        }
        if reasons.is_empty() {
            reasons.push("Comparison changed");
        }
        reasons
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ReviewRecords {
    schema_version: u32,
    viewed: BTreeMap<String, Approval>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(from = "SavedApproval")]
struct Approval {
    fingerprint: Fingerprint,
    snapshot: Option<ApprovedSnapshot>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum SavedApproval {
    Current {
        fingerprint: Fingerprint,
        snapshot: Option<ApprovedSnapshot>,
    },
    Fingerprint(Fingerprint),
}

impl From<SavedApproval> for Approval {
    fn from(saved: SavedApproval) -> Self {
        match saved {
            SavedApproval::Current {
                fingerprint,
                snapshot,
            } => Self {
                fingerprint,
                snapshot,
            },
            SavedApproval::Fingerprint(fingerprint) => Self {
                fingerprint,
                snapshot: None,
            },
        }
    }
}

impl Default for ReviewRecords {
    fn default() -> Self {
        Self {
            schema_version: 3,
            viewed: BTreeMap::new(),
        }
    }
}

impl ReviewRecords {
    fn restore(value: Option<&str>) -> Result<Self> {
        let mut records = value
            .map(serde_json::from_str::<Self>)
            .transpose()
            .context("Unable to read saved Branch Review progress")?
            .unwrap_or_default();
        ensure!(
            matches!(records.schema_version, 1..=3),
            "Unsupported Branch Review state version"
        );
        records.schema_version = 3;
        for approval in records.viewed.values() {
            if let Some(snapshot) = &approval.snapshot {
                snapshot
                    .decode()
                    .context("Unable to read a saved Viewed snapshot")?;
            }
        }
        Ok(records)
    }

    fn is_viewed(&self, path: &str, fingerprint: &Fingerprint) -> bool {
        self.viewed
            .get(path)
            .is_some_and(|approval| approval.fingerprint == *fingerprint)
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

    pub fn change_reasons(&self, path: &str, fingerprint: &Fingerprint) -> Vec<&'static str> {
        if self.error.is_some() {
            return Vec::new();
        }
        self.records
            .viewed
            .get(path)
            .map(|approved| fingerprint.changes_from(&approved.fingerprint))
            .unwrap_or_default()
    }

    pub fn approved_snapshot(&self, path: &str) -> Result<Option<SnapshotAvailability>> {
        if self.error.is_some() {
            return Ok(None);
        }
        self.records
            .viewed
            .get(path)
            .map(|approval| match &approval.snapshot {
                Some(snapshot) => snapshot.decode(),
                None => Ok(SnapshotAvailability::Legacy),
            })
            .transpose()
    }

    pub fn enrich_approval(
        &mut self,
        path: &str,
        fingerprint: &Fingerprint,
        base: Option<&str>,
        current: Option<&str>,
        cx: &mut Context<Self>,
    ) {
        if self.error.is_none()
            && self.records.viewed.get(path).is_some_and(|saved| {
                saved.fingerprint == *fingerprint
                    && (saved.fingerprint.details.is_none() || saved.snapshot.is_none())
            })
        {
            match ApprovedSnapshot::capture(base, current) {
                Ok(snapshot) => {
                    self.records.viewed.insert(
                        path.to_owned(),
                        Approval {
                            fingerprint: fingerprint.clone(),
                            snapshot: Some(snapshot),
                        },
                    );
                    self.persist(cx);
                }
                Err(error) => {
                    self.error = Some(format!("Progress was not saved: {error:#}"));
                    cx.notify();
                }
            }
        }
    }

    pub fn set_viewed(
        &mut self,
        path: String,
        fingerprint: Option<Fingerprint>,
        comparison: Option<(Option<&str>, Option<&str>)>,
        cx: &mut Context<Self>,
    ) {
        if self.error.is_some() {
            return;
        }
        match fingerprint {
            Some(fingerprint) => {
                let snapshot = match comparison
                    .map(|(base, current)| ApprovedSnapshot::capture(base, current))
                    .transpose()
                {
                    Ok(snapshot) => snapshot,
                    Err(error) => {
                        self.error = Some(format!("Progress was not saved: {error:#}"));
                        cx.notify();
                        return;
                    }
                };
                self.records.viewed.insert(
                    path,
                    Approval {
                        fingerprint,
                        snapshot,
                    },
                );
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

    #[test]
    fn legacy_approvals_keep_identity_and_explain_only_known_changes() {
        let approved = fingerprint("a", Some("base"), Some("approved"));
        let legacy =
            serde_json::json!({"schema_version":1,"viewed":{"a":approved.value}}).to_string();
        let records = ReviewRecords::restore(Some(&legacy)).unwrap();
        assert!(records.is_viewed("a", &approved));
        assert_eq!(records.schema_version, 3);
        let revised = fingerprint("a", Some("base"), Some("revised"));
        assert_eq!(
            revised.changes_from(&records.viewed["a"].fingerprint),
            ["Comparison changed"]
        );
        let round_trip =
            ReviewRecords::restore(Some(&serde_json::to_string(&records).unwrap())).unwrap();
        assert!(round_trip.is_viewed("a", &approved));
    }

    #[test]
    fn reasons_track_each_comparison_component_and_exact_undo() {
        let approved = fingerprint("a", Some("base"), Some("approved"));
        assert!(approved.changes_from(&approved).is_empty());
        assert_eq!(
            fingerprint("a", Some("base"), Some("revised")).changes_from(&approved),
            ["Content changed"]
        );
        assert_eq!(
            fingerprint("a", Some("new base"), Some("approved")).changes_from(&approved),
            ["Base changed"]
        );
        let changed = Fingerprint::new("a", None, None, 0, 0).with_rename(Some("old-a"));
        assert_eq!(
            changed.changes_from(&approved),
            [
                "Base changed",
                "Content changed",
                "File added/deleted",
                "Mode changed",
                "Rename changed"
            ]
        );
        assert_eq!(
            Fingerprint::new("a", Some(b"base"), Some(b"approved"), 0o100644, 0o100755)
                .changes_from(&approved),
            ["Mode changed"]
        );
    }

    #[test]
    fn approved_comparison_round_trips_absent_sides_and_rejects_corruption() {
        let snapshot = ApprovedSnapshot::capture(Some("base\r\n"), None).unwrap();
        assert_eq!(
            snapshot.decode().unwrap(),
            SnapshotAvailability::Available {
                base: Some("base\r\n".into()),
                current: None,
            }
        );
        let value = serde_json::to_string(&snapshot).unwrap();
        let restored: ApprovedSnapshot = serde_json::from_str(&value).unwrap();
        assert_eq!(restored.decode().unwrap(), snapshot.decode().unwrap());

        let mut corrupt: serde_json::Value = serde_json::from_str(&value).unwrap();
        corrupt["comparison"]["base"]["zstd_base64"] = "not base64".into();
        let corrupt: ApprovedSnapshot = serde_json::from_value(corrupt).unwrap();
        assert!(corrupt.decode().is_err());

        let mut records = ReviewRecords::default();
        records.viewed.insert(
            "a".into(),
            Approval {
                fingerprint: fingerprint("a", Some("base"), None),
                snapshot: Some(corrupt),
            },
        );
        assert!(ReviewRecords::restore(Some(&serde_json::to_string(&records).unwrap())).is_err());
    }

    #[test]
    fn oversized_approved_comparison_keeps_viewed_without_storing_source() {
        let text = "x".repeat(MAX_SNAPSHOT_BYTES + 1);
        assert!(matches!(
            ApprovedSnapshot::capture(Some(&text), None).unwrap(),
            ApprovedSnapshot::TooLarge
        ));
    }

    #[gpui::test]
    async fn matching_legacy_approval_is_enriched_and_manual_uncheck_remains_explicit(
        cx: &mut gpui::TestAppContext,
    ) {
        let database = KeyValueStore::open_test_db("review_metadata_migration").await;
        let approved = fingerprint("a", Some("base"), Some("approved"));
        database
            .write_kvp(
                "review".into(),
                serde_json::json!({"schema_version":1,"viewed":{"a":approved.value}}).to_string(),
            )
            .await
            .unwrap();
        let state = cx.new(|_| ReviewState::load("review".into(), database.clone()));
        state.update(cx, |state, cx| {
            state.enrich_approval("a", &approved, Some("base"), Some("approved"), cx)
        });
        cx.run_until_parked();
        let restored = ReviewState::load("review".into(), database.clone());
        assert!(restored.is_viewed("a", &approved));
        assert_eq!(
            restored.change_reasons("a", &fingerprint("a", Some("base"), Some("revised"))),
            ["Content changed"]
        );
        assert_eq!(
            restored.approved_snapshot("a").unwrap(),
            Some(SnapshotAvailability::Available {
                base: Some("base".into()),
                current: Some("approved".into()),
            })
        );
        state.update(cx, |state, cx| state.set_viewed("a".into(), None, None, cx));
        cx.run_until_parked();
        let restored = ReviewState::load("review".into(), database);
        assert!(!restored.is_viewed("a", &approved));
        assert!(restored.change_reasons("a", &approved).is_empty());
    }

    fn fingerprint(path: &str, base: Option<&str>, current: Option<&str>) -> Fingerprint {
        Fingerprint::new(
            path,
            base.map(str::as_bytes),
            current.map(str::as_bytes),
            0o100644,
            0o100644,
        )
    }

    fn approval(fingerprint: Fingerprint) -> Approval {
        Approval {
            fingerprint,
            snapshot: None,
        }
    }

    #[test]
    fn selective_invalidation_and_undo() {
        let mut records = ReviewRecords::default();
        let a = fingerprint("a", Some("base a"), Some("reviewed a"));
        let b = fingerprint("b", Some("base b"), Some("reviewed b"));
        records.viewed.insert("a".into(), approval(a.clone()));
        records.viewed.insert("b".into(), approval(b.clone()));
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
        records.viewed.insert("a".into(), approval(a.clone()));
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
        assert!(ReviewRecords::restore(Some(r#"{"schema_version":99,"viewed":{}}"#)).is_err());
    }
    #[gpui::test]
    async fn ordered_writes_failure_and_retry(cx: &mut gpui::TestAppContext) {
        let database = KeyValueStore::open_test_db("branch_review_ordering").await;
        let state = cx.new(|_| ReviewState::load("review".into(), database.clone()));
        let a = fingerprint("a", Some("old"), Some("new"));
        state.update(cx, |state, cx| {
            state.set_viewed("a".into(), Some(a.clone()), None, cx);
            state.set_viewed("b".into(), Some(a.clone()), None, cx);
            state.set_viewed("b".into(), None, None, cx);
        });
        cx.run_until_parked();
        let restored = ReviewState::load("review".into(), database.clone());
        assert!(restored.is_viewed("a", &a));
        assert!(!restored.is_viewed("b", &a));
        database.write(|connection| connection.exec(
            "CREATE TRIGGER fail_review_write BEFORE INSERT ON kv_store BEGIN SELECT RAISE(ABORT, 'injected storage failure'); END"
        )?()).await.unwrap();
        state.update(cx, |state, cx| {
            state.set_viewed("b".into(), Some(a.clone()), None, cx)
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
