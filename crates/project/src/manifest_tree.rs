//! This module defines a Manifest Tree.
//!
//! A Manifest Tree is responsible for determining where the manifests for subprojects are located in a project.
//! This then is used to provide those locations to language servers & determine locations eligible for toolchain selection.

mod manifest_store;
mod path_trie;
mod server_tree;

use std::{borrow::Borrow, collections::hash_map::Entry, ops::ControlFlow, path::Path, sync::Arc};

use collections::HashMap;
use gpui::{App, AppContext as _, Context, Entity, Subscription};
use language::{ManifestDelegate, ManifestName, ManifestQuery};
pub use manifest_store::ManifestProvidersStore;
use path_trie::{LabelPresence, RootPathTrie, TriePath};
use settings::{SettingsStore, WorktreeId};
use worktree::{Event as WorktreeEvent, Snapshot, Worktree};

use crate::{
    ProjectPath,
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};

pub(crate) use server_tree::{LanguageServerTree, LanguageServerTreeNode, LaunchDisposition};

struct WorktreeRoots {
    roots: RootPathTrie<ManifestName>,
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
                            if kind == &worktree::PathChange::Removed {
                                let path = TriePath::from(path.as_ref());
                                this.roots.remove(&path);
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

pub struct ManifestTree {
    root_points: HashMap<WorktreeId, Entity<WorktreeRoots>>,
    worktree_store: Entity<WorktreeStore>,
    _subscriptions: [Subscription; 2],
}

impl ManifestTree {
    pub fn new(worktree_store: Entity<WorktreeStore>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            root_points: Default::default(),
            _subscriptions: [
                cx.subscribe(&worktree_store, Self::on_worktree_store_event),
                cx.observe_global::<SettingsStore>(|this, cx| {
                    for roots in this.root_points.values_mut() {
                        roots.update(cx, |worktree_roots, _| {
                            worktree_roots.roots = RootPathTrie::new();
                        })
                    }
                }),
            ],
            worktree_store,
        })
    }

    pub(crate) fn root_for_path(
        &mut self,
        ProjectPath { worktree_id, path }: &ProjectPath,
        manifest_name: &ManifestName,
        delegate: &Arc<dyn ManifestDelegate>,
        cx: &mut App,
    ) -> Option<ProjectPath> {
        debug_assert_eq!(delegate.worktree_id(), *worktree_id);
        let (mut marked_path, mut current_presence) = (None, LabelPresence::KnownAbsent);
        let worktree_roots = match self.root_points.entry(*worktree_id) {
            Entry::Occupied(occupied_entry) => occupied_entry.get().clone(),
            Entry::Vacant(vacant_entry) => {
                let Some(worktree) = self
                    .worktree_store
                    .read(cx)
                    .worktree_for_id(*worktree_id, cx)
                else {
                    return Default::default();
                };
                let roots = WorktreeRoots::new(self.worktree_store.clone(), worktree, cx);
                vacant_entry.insert(roots).clone()
            }
        };

        let key = TriePath::from(&**path);
        worktree_roots.read_with(cx, |this, _| {
            this.roots.walk(&key, &mut |path, labels| {
                for (label, presence) in labels {
                    if label == manifest_name {
                        if current_presence > *presence {
                            debug_assert!(false, "RootPathTrie precondition violation; while walking the tree label presence is only allowed to increase");
                        }
                        marked_path = Some(ProjectPath {worktree_id: *worktree_id, path: path.clone()});
                        current_presence = *presence;
                    }

                }
                ControlFlow::Continue(())
            });
        });

        if current_presence == LabelPresence::KnownAbsent {
            // Some part of the path is unexplored.
            let depth = marked_path
                .as_ref()
                .map(|root_path| {
                    path.strip_prefix(&root_path.path)
                        .unwrap()
                        .components()
                        .count()
                })
                .unwrap_or_else(|| path.components().count() + 1);

            if depth > 0
                && let Some(provider) =
                    ManifestProvidersStore::global(cx).get(manifest_name.borrow())
            {
                let root = provider.search(ManifestQuery {
                    path: path.clone(),
                    depth,
                    delegate: delegate.clone(),
                });
                match root {
                    Some(known_root) => worktree_roots.update(cx, |this, _| {
                        let root = TriePath::from(&*known_root);
                        this.roots
                            .insert(&root, manifest_name.clone(), LabelPresence::Present);
                        current_presence = LabelPresence::Present;
                        marked_path = Some(ProjectPath {
                            worktree_id: *worktree_id,
                            path: known_root,
                        });
                    }),
                    None => worktree_roots.update(cx, |this, _| {
                        this.roots
                            .insert(&key, manifest_name.clone(), LabelPresence::KnownAbsent);
                    }),
                }
            }
        }
        marked_path.filter(|_| current_presence.eq(&LabelPresence::Present))
    }

    pub(crate) fn root_for_path_or_worktree_root(
        &mut self,
        project_path: &ProjectPath,
        manifest_name: Option<&ManifestName>,
        delegate: &Arc<dyn ManifestDelegate>,
        cx: &mut App,
    ) -> ProjectPath {
        let worktree_id = project_path.worktree_id;
        // Backwards-compat: Fill in any adapters for which we did not detect the root as having the project root at the root of a worktree.
        manifest_name
            .and_then(|manifest_name| self.root_for_path(project_path, manifest_name, delegate, cx))
            .unwrap_or_else(|| ProjectPath {
                worktree_id,
                path: Arc::from(Path::new("")),
            })
    }

    fn on_worktree_store_event(
        &mut self,
        _: Entity<WorktreeStore>,
        evt: &WorktreeStoreEvent,
        _: &mut Context<Self>,
    ) {
        if let WorktreeStoreEvent::WorktreeRemoved(_, worktree_id) = evt {
            self.root_points.remove(worktree_id);
        }
    }
}

pub(crate) struct ManifestQueryDelegate {
    worktree: Snapshot,
}

impl ManifestQueryDelegate {
    pub fn new(worktree: Snapshot) -> Self {
        Self { worktree }
    }
}

impl ManifestDelegate for ManifestQueryDelegate {
    fn exists(&self, path: &Path, is_dir: Option<bool>) -> bool {
        self.worktree.entry_for_path(path).is_some_and(|entry| {
            is_dir.is_none_or(|is_required_to_be_dir| is_required_to_be_dir == entry.is_dir())
        })
    }

    fn worktree_id(&self) -> WorktreeId {
        self.worktree.id()
    }
}
