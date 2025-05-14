//! This module defines a Manifest Tree.
//!
//! A Manifest Tree is responsible for determining where the manifests for subprojects are located in a project.
//! This then is used to provide those locations to language servers & determine locations eligible for toolchain selection.

mod manifest_store;
mod path_trie;
mod server_tree;

use std::{
    borrow::Borrow,
    collections::{BTreeMap, hash_map::Entry},
    ops::ControlFlow,
    sync::Arc,
};

use collections::HashMap;
use gpui::{App, AppContext as _, Context, Entity, EventEmitter, Subscription};
use language::{LspAdapterDelegate, ManifestName, ManifestQuery};
pub use manifest_store::ManifestProviders;
use path_trie::{LabelPresence, RootPathTrie, TriePath};
use settings::{SettingsStore, WorktreeId};
use worktree::{Event as WorktreeEvent, Worktree};

use crate::{
    ProjectPath,
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};

pub(crate) use server_tree::{
    AdapterQuery, LanguageServerTree, LanguageServerTreeNode, LaunchDisposition, ServerTreeRebase,
};

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

pub struct ManifestTree {
    root_points: HashMap<WorktreeId, Entity<WorktreeRoots>>,
    worktree_store: Entity<WorktreeStore>,
    _subscriptions: [Subscription; 2],
}

#[derive(PartialEq)]
pub(crate) enum ManifestTreeEvent {
    WorktreeRemoved(WorktreeId),
    Cleared,
}

impl EventEmitter<ManifestTreeEvent> for ManifestTree {}

impl ManifestTree {
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
                    cx.emit(ManifestTreeEvent::Cleared);
                }),
            ],
            worktree_store,
        })
    }
    fn root_for_path(
        &mut self,
        ProjectPath { worktree_id, path }: ProjectPath,
        manifests: &mut dyn Iterator<Item = ManifestName>,
        delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut App,
    ) -> BTreeMap<ManifestName, ProjectPath> {
        debug_assert_eq!(delegate.worktree_id(), worktree_id);
        let mut roots = BTreeMap::from_iter(
            manifests.map(|manifest| (manifest, (None, LabelPresence::KnownAbsent))),
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

        for (manifest_name, (root_path, presence)) in &mut roots {
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
                let Some(provider) = ManifestProviders::global(cx).get(manifest_name.borrow())
                else {
                    log::warn!("Manifest provider `{}` not found", manifest_name.as_ref());
                    continue;
                };

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
                        *presence = LabelPresence::Present;
                        *root_path = Some(ProjectPath {
                            worktree_id,
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
                cx.emit(ManifestTreeEvent::WorktreeRemoved(*worktree_id));
            }
            _ => {}
        }
    }
}
