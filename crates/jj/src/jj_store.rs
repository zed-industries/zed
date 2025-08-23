use std::collections::HashMap;
use std::sync::Arc;

use gpui::{App, Context, Entity, EventEmitter, Global, Subscription, prelude::*};
use project::worktree_store::{WorktreeStore, WorktreeStoreEvent};
use worktree::{Worktree, WorktreeId};

use crate::{JujutsuRepository, RealJujutsuRepository};

/// Note: We won't ultimately be storing the jj store in a global, we're just doing this for exploration purposes.
struct GlobalJujutsuStore(Entity<JujutsuStore>);

impl Global for GlobalJujutsuStore {}

pub struct JujutsuStore {
    active_repository: Option<WorktreeId>,
    repositories: HashMap<WorktreeId, Arc<dyn JujutsuRepository>>,
    _subscriptions: Vec<Subscription>,
}

pub enum JujutsuStoreEvent {
    ActiveRepositoryChanged(Option<WorktreeId>),
    RepositoryAdded(WorktreeId),
    RepositoryRemoved(WorktreeId),
}

impl EventEmitter<JujutsuStoreEvent> for JujutsuStore {}

impl JujutsuStore {
    pub fn init_global(cx: &mut App, worktree_store: Entity<WorktreeStore>) {
        let jj_store = cx.new(|cx| JujutsuStore::new(worktree_store, cx));
        cx.set_global(GlobalJujutsuStore(jj_store));
    }

    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalJujutsuStore>()
            .map(|global| global.0.clone())
    }

    pub fn new(worktree_store: Entity<WorktreeStore>, cx: &mut Context<Self>) -> Self {
        let _subscriptions = vec![cx.subscribe(&worktree_store, Self::on_worktree_store_event)];

        let mut store = JujutsuStore {
            active_repository: None,
            repositories: HashMap::default(),
            _subscriptions,
        };

        let existing_worktrees: Vec<_> = worktree_store.read(cx).worktrees().collect();

        for worktree in existing_worktrees {
            store.scan_worktree_for_jj_repo(&worktree, cx);
        }

        store
    }

    fn on_worktree_store_event(
        &mut self,
        _worktree_store: Entity<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            WorktreeStoreEvent::WorktreeAdded(worktree) => {
                self.scan_worktree_for_jj_repo(worktree, cx);
            }
            WorktreeStoreEvent::WorktreeRemoved(_, worktree_id) => {
                if self.repositories.remove(worktree_id).is_some() {
                    cx.emit(JujutsuStoreEvent::RepositoryRemoved(*worktree_id));

                    if self.active_repository == Some(*worktree_id) {
                        self.active_repository = None;
                        cx.emit(JujutsuStoreEvent::ActiveRepositoryChanged(None));
                    }
                }
            }
            _ => {}
        }
    }

    fn scan_worktree_for_jj_repo(&mut self, worktree: &Entity<Worktree>, cx: &mut Context<Self>) {
        let worktree = worktree.read(cx);
        let worktree_id = worktree.id();
        let root_path = worktree.abs_path();

        match RealJujutsuRepository::new(&root_path) {
            Ok(repository) => {
                let repository = Arc::new(repository);

                self.repositories.insert(worktree_id, repository);
                cx.emit(JujutsuStoreEvent::RepositoryAdded(worktree_id));

                if self.active_repository.is_none() {
                    self.active_repository = Some(worktree_id);
                    cx.emit(JujutsuStoreEvent::ActiveRepositoryChanged(Some(
                        worktree_id,
                    )));
                }
            }
            Err(_) => {}
        }
    }

    pub fn repository_for_worktree(
        &self,
        worktree_id: WorktreeId,
    ) -> Option<&Arc<dyn JujutsuRepository>> {
        self.repositories.get(&worktree_id)
    }

    pub fn active_repository(&self) -> Option<&Arc<dyn JujutsuRepository>> {
        self.active_repository
            .and_then(|id| self.repositories.get(&id))
    }

    pub fn repository(&self) -> Option<&Arc<dyn JujutsuRepository>> {
        self.active_repository()
    }
}
