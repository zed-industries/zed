use crate::{btree::KvStore, Executor, RepoId, Revision, RevisionId};
use collections::HashMap;
use futures::{channel::mpsc, StreamExt};
use parking_lot::Mutex;
use std::sync::Arc;
use util::ResultExt;

#[derive(Clone, Debug)]
pub struct RevisionCache {
    repo_id: RepoId,
    revisions: Arc<Mutex<HashMap<RevisionId, Revision>>>,
    revisions_to_save: mpsc::UnboundedSender<(RevisionId, Revision)>,
}

impl RevisionCache {
    pub fn new<E: Executor>(repo_id: RepoId, executor: &E, kv: Arc<dyn KvStore>) -> Self {
        let (revisions_to_save_tx, mut revisions_to_save_rx) =
            mpsc::unbounded::<(RevisionId, Revision)>();
        executor.spawn(async move {
            while let Some((revision_id, revision)) = revisions_to_save_rx.next().await {
                if !Revision::exists(repo_id, &revision_id, &*kv).await {
                    revision.save(repo_id, &revision_id, &*kv).await.log_err();
                }
            }
        });

        Self {
            repo_id,
            // Always consider the empty revision as cached.
            revisions: Arc::new(Mutex::new(HashMap::from_iter([(
                RevisionId::default(),
                Revision::default(),
            )]))),
            revisions_to_save: revisions_to_save_tx,
        }
    }

    pub fn get(&self, revision_id: &RevisionId) -> Option<Revision> {
        self.revisions.lock().get(revision_id).cloned()
    }

    pub async fn load(&self, revision_id: &RevisionId, kv: &dyn KvStore) -> Option<Revision> {
        if let Some(revision) = self.get(revision_id) {
            Some(revision)
        } else if let Some(revision) = Revision::load(self.repo_id, revision_id, kv).await.ok() {
            Some(
                self.revisions
                    .lock()
                    .entry(revision_id.clone())
                    .or_insert(revision)
                    .clone(),
            )
        } else {
            None
        }
    }

    pub fn save(&self, revision_id: &RevisionId, revision: Revision) {
        self.revisions
            .lock()
            .entry(revision_id.clone())
            .or_insert_with(|| {
                let _ = self
                    .revisions_to_save
                    .unbounded_send((revision_id.clone(), revision.clone()));
                revision
            });
    }
}
