//! This module defines a Project Tree.
//!
//! A Project Tree is responsible for determining where the roots of subprojects are located in a project.

mod server_tree;
mod toolchain_tree;

use std::{
    collections::{btree_map::Entry as TreeEntry, hash_map::Entry, BTreeMap},
    path::Path,
    sync::Arc,
};

use collections::HashMap;
use gpui::{AppContext, Context as _, Model, ModelContext, Subscription};
use language::{CachedLspAdapter, LanguageName, LanguageRegistry};
use lsp::LanguageServerName;
use settings::WorktreeId;
use worktree::{Event as WorktreeEvent, Worktree};

use crate::{
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
    ProjectPath,
};

pub(crate) use server_tree::LanguageServerTree;
type IsRoot = bool;

struct WorktreeRoots {
    roots: BTreeMap<Arc<Path>, BTreeMap<LanguageServerName, IsRoot>>,
    worktree_store: Model<WorktreeStore>,
    worktree_subscription: Subscription,
}

impl WorktreeRoots {
    fn new(
        worktree_store: Model<WorktreeStore>,
        worktree: Model<Worktree>,
        cx: &mut AppContext,
    ) -> Model<Self> {
        cx.new_model(|cx| Self {
            roots: Default::default(),
            worktree_store,
            worktree_subscription: cx.subscribe(&worktree, |this: &mut Self, _, event, cx| {
                match event {
                    WorktreeEvent::UpdatedEntries(changes) => {
                        for (path, _, kind) in changes.iter() {
                            match kind {
                                worktree::PathChange::Removed => {
                                    this.roots.remove(path);
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
                        this.roots.remove(&entry.path);
                    }
                }
            }),
        })
    }
}

pub struct ProjectTree {
    languages: Arc<LanguageRegistry>,
    root_points: HashMap<WorktreeId, Model<WorktreeRoots>>,
    worktree_store: Model<WorktreeStore>,
    _subscriptions: [Subscription; 1],
}

#[derive(Clone)]
struct AdapterWrapper(Arc<CachedLspAdapter>);
impl PartialEq for AdapterWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.name.eq(&other.0.name)
    }
}

impl PartialOrd for AdapterWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.name.partial_cmp(&other.0.name)
    }
}

impl Ord for AdapterWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.name.cmp(&other.0.name)
    }
}

impl Eq for AdapterWrapper {}

impl ProjectTree {
    pub(crate) fn new(
        languages: Arc<LanguageRegistry>,
        worktree_store: Model<WorktreeStore>,
        cx: &mut AppContext,
    ) -> Model<Self> {
        cx.new_model(|cx| Self {
            languages,
            root_points: Default::default(),
            _subscriptions: [cx.subscribe(&worktree_store, Self::on_worktree_store_event)],
            worktree_store,
        })
    }
    fn root_for_path(
        &mut self,
        ProjectPath { worktree_id, path }: ProjectPath,
        language_name: &LanguageName,
        cx: &mut AppContext,
    ) -> BTreeMap<AdapterWrapper, ProjectPath> {
        let mut roots = BTreeMap::default();
        let adapters = self.languages.lsp_adapters(&language_name);
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

        let mut filled_adapters = vec![false; adapters.len()];
        let mut adapters_with_roots = 0;
        for ancestor in path.ancestors().skip(1) {
            // TODO: scan up until worktree root and no further.
            if adapters_with_roots == adapters.len() {
                // We've found roots for all adapters, no need to continue
                break;
            }
            worktree_roots.update(cx, |this, _| {
                let adapter_roots = this.roots.entry(ancestor.into()).or_default();
                for (ix, adapter) in adapters.iter().enumerate() {
                    let adapter_already_found_root = filled_adapters[ix];
                    if adapter_already_found_root {
                        continue;
                    }

                    match adapter_roots.entry(adapter.name.clone()) {
                        TreeEntry::Vacant(vacant_entry) => {
                            let root = adapter.find_closest_project_root(worktree_id, path.clone());
                            vacant_entry.insert(root.is_some());
                            if let Some(root) = root {
                                roots.insert(
                                    AdapterWrapper(adapter.clone()),
                                    (worktree_id, root).into(),
                                );
                            }
                        }
                        TreeEntry::Occupied(occupied_entry) => {
                            let is_root = *occupied_entry.get();
                            if is_root {
                                roots.insert(
                                    AdapterWrapper(adapter.clone()),
                                    (worktree_id, ancestor).into(),
                                );
                            }

                            continue;
                        }
                    }
                    filled_adapters[ix] = true;
                    adapters_with_roots += 1;
                }
            });
        }

        roots
    }
    fn on_worktree_store_event(
        &mut self,
        _: Model<WorktreeStore>,
        evt: &WorktreeStoreEvent,
        _: &mut ModelContext<Self>,
    ) {
        match evt {
            WorktreeStoreEvent::WorktreeRemoved(_, worktree_id) => {
                self.root_points.remove(&worktree_id);
            }
            _ => {}
        }
    }
}
