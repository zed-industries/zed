//! This module defines a Project Tree.
//!
//! A Project Tree is responsible for determining where the roots of subprojects are located in a project.

mod path_trie;
mod server_tree;

use std::{
    borrow::Borrow,
    collections::{hash_map::Entry, BTreeMap},
    ops::ControlFlow,
    sync::Arc,
};

use collections::HashMap;
use gpui::{App, AppContext as _, Context, Entity, EventEmitter, Subscription};
use language::{CachedLspAdapter, LspAdapterDelegate};
use lsp::LanguageServerName;
use path_trie::{LabelPresence, RootPathTrie, TriePath};
use settings::{SettingsStore, WorktreeId};
use worktree::{Event as WorktreeEvent, Worktree};

use crate::{
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
    ProjectPath,
};

pub(crate) use server_tree::{AdapterQuery, LanguageServerTree, LaunchDisposition};

struct WorktreeRoots {
    roots: RootPathTrie<LanguageServerName>,
    worktree_store: Entity<WorktreeStore>,
    _worktree_subscription: Subscription,
}

impl WorktreeRoots {
    fn new(
        worktree_store: Entity<WorktreeStore>,
        worktree: Entity<Worktree>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| Self {
            roots: RootPathTrie::new(),
            worktree_store,
            _worktree_subscription: cx.subscribe(&worktree, |this: &mut Self, _, event, cx| {
                match event {
                    WorktreeEvent::UpdatedEntries(changes) => {
                        for (path, _, kind) in changes.iter() {
                            match kind {
                                worktree::PathChange::Removed => {
                                    let path = TriePath::from(path.as_ref());
                                    this.roots.remove(&path);
                                }
                                _ => {}
                            }
                        }
                    }
                    WorktreeEvent::UpdatedGitRepositories(_) => {}
                    WorktreeEvent::DeletedEntry(entry_id) => {
                        let Some(entry) = this.worktree_store.read(cx).entry_for_id(*entry_id, cx)
                        else {
                            return;
                        };
                        let path = TriePath::from(entry.path.as_ref());
                        this.roots.remove(&path);
                    }
                }
            }),
        })
    }
}

pub struct ProjectTree {
    root_points: HashMap<WorktreeId, Entity<WorktreeRoots>>,
    worktree_store: Entity<WorktreeStore>,
    _subscriptions: [Subscription; 2],
}

#[derive(Debug, Clone)]
struct AdapterWrapper(Arc<CachedLspAdapter>);
impl PartialEq for AdapterWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.name.eq(&other.0.name)
    }
}

impl Eq for AdapterWrapper {}

impl std::hash::Hash for AdapterWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.name.hash(state);
    }
}

impl PartialOrd for AdapterWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.name.cmp(&other.0.name))
    }
}

impl Ord for AdapterWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.name.cmp(&other.0.name)
    }
}

impl Borrow<LanguageServerName> for AdapterWrapper {
    fn borrow(&self) -> &LanguageServerName {
        &self.0.name
    }
}

#[derive(PartialEq)]
pub(crate) enum ProjectTreeEvent {
    WorktreeRemoved(WorktreeId),
    Cleared,
}

impl EventEmitter<ProjectTreeEvent> for ProjectTree {}

impl ProjectTree {
    pub(crate) fn new(worktree_store: Entity<WorktreeStore>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            root_points: Default::default(),
            _subscriptions: [
                cx.subscribe(&worktree_store, Self::on_worktree_store_event),
                cx.observe_global::<SettingsStore>(|this, cx| {
                    for (_, roots) in &mut this.root_points {
                        roots.update(cx, |worktree_roots, _| {
                            worktree_roots.roots = RootPathTrie::new();
                        })
                    }
                    cx.emit(ProjectTreeEvent::Cleared);
                }),
            ],
            worktree_store,
        })
    }
    #[allow(clippy::mutable_key_type)]
    fn root_for_path(
        &mut self,
        ProjectPath { worktree_id, path }: ProjectPath,
        adapters: Vec<Arc<CachedLspAdapter>>,
        delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut App,
    ) -> BTreeMap<AdapterWrapper, ProjectPath> {
        debug_assert_eq!(delegate.worktree_id(), worktree_id);
        #[allow(clippy::mutable_key_type)]
        let mut roots = BTreeMap::from_iter(
            adapters
                .into_iter()
                .map(|adapter| (AdapterWrapper(adapter), (None, LabelPresence::KnownAbsent))),
        );
        let worktree_roots = match self.root_points.entry(worktree_id) {
            Entry::Occupied(occupied_entry) => occupied_entry.get().clone(),
            Entry::Vacant(vacant_entry) => {
                let Some(worktree) = self
                    .worktree_store
                    .read(cx)
                    .worktree_for_id(worktree_id, cx)
                else {
                    return Default::default();
                };
                let roots = WorktreeRoots::new(self.worktree_store.clone(), worktree, cx);
                vacant_entry.insert(roots).clone()
            }
        };

        let key = TriePath::from(&*path);
        worktree_roots.update(cx, |this, _| {
            this.roots.walk(&key, &mut |path, labels| {
                for (label, presence) in labels {
                    if let Some((marked_path, current_presence)) = roots.get_mut(label) {
                        if *current_presence > *presence {
                            debug_assert!(false, "RootPathTrie precondition violation; while walking the tree label presence is only allowed to increase");
                        }
                        *marked_path = Some(ProjectPath {worktree_id, path: path.clone()});
                        *current_presence = *presence;
                    }

                }
                ControlFlow::Continue(())
            });
        });
        for (adapter, (root_path, presence)) in &mut roots {
            if *presence == LabelPresence::Present {
                continue;
            }

            let depth = root_path
                .as_ref()
                .map(|root_path| {
                    path.strip_prefix(&root_path.path)
                        .unwrap()
                        .components()
                        .count()
                })
                .unwrap_or_else(|| path.components().count() + 1);

            if depth > 0 {
                let root = adapter.0.find_project_root(&path, depth, &delegate);
                match root {
                    Some(known_root) => worktree_roots.update(cx, |this, _| {
                        let root = TriePath::from(&*known_root);
                        this.roots
                            .insert(&root, adapter.0.name(), LabelPresence::Present);
                        *presence = LabelPresence::Present;
                        *root_path = Some(ProjectPath {
                            worktree_id,
                            path: known_root,
                        });
                    }),
                    None => worktree_roots.update(cx, |this, _| {
                        this.roots
                            .insert(&key, adapter.0.name(), LabelPresence::KnownAbsent);
                    }),
                }
            }
        }

        roots
            .into_iter()
            .filter_map(|(k, (path, presence))| {
                let path = path?;
                presence.eq(&LabelPresence::Present).then(|| (k, path))
            })
            .collect()
    }
    fn on_worktree_store_event(
        &mut self,
        _: Entity<WorktreeStore>,
        evt: &WorktreeStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match evt {
            WorktreeStoreEvent::WorktreeRemoved(_, worktree_id) => {
                self.root_points.remove(&worktree_id);
                cx.emit(ProjectTreeEvent::WorktreeRemoved(*worktree_id));
            }
            _ => {}
        }
    }
}
