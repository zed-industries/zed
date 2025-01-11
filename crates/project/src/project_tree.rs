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

use collections::{HashMap, HashSet};
use gpui::{AppContext, Context as _, Model, ModelContext, Subscription};
use language::{CachedLspAdapter, LanguageName, LanguageRegistry, LspAdapterDelegate};
use lsp::LanguageServerName;
use path_trie::{LabelPresence, RootPathTrie, TriePath};
use settings::WorktreeId;
use worktree::{Event as WorktreeEvent, Worktree};

use crate::{
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
    ProjectPath,
};

pub(crate) use server_tree::LanguageServerTree;

struct WorktreeRoots {
    roots: RootPathTrie<LanguageServerName>,
    worktree_store: Model<WorktreeStore>,
    _worktree_subscription: Subscription,
}

impl WorktreeRoots {
    fn new(
        worktree_store: Model<WorktreeStore>,
        worktree: Model<Worktree>,
        cx: &mut AppContext,
    ) -> Model<Self> {
        cx.new_model(|cx| Self {
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
    languages: Arc<LanguageRegistry>,
    root_points: HashMap<WorktreeId, Model<WorktreeRoots>>,
    worktree_store: Model<WorktreeStore>,
    _subscriptions: [Subscription; 1],
}

#[derive(Debug, Clone)]
struct AdapterWrapper(Arc<CachedLspAdapter>);
impl PartialEq for AdapterWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.name.eq(&other.0.name)
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

impl Eq for AdapterWrapper {}

impl Borrow<LanguageServerName> for AdapterWrapper {
    fn borrow(&self) -> &LanguageServerName {
        &self.0.name
    }
}

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
        delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut AppContext,
    ) -> BTreeMap<AdapterWrapper, ProjectPath> {
        debug_assert_eq!(delegate.worktree_id(), worktree_id);
        let adapters = self.languages.lsp_adapters(&language_name);
        let empty_path = ProjectPath {
            worktree_id,
            path: Arc::from("".as_ref()),
        };
        let mut roots = BTreeMap::from_iter(adapters.into_iter().map(|adapter| {
            (
                AdapterWrapper(adapter),
                (empty_path.clone(), LabelPresence::KnownAbsent),
            )
        }));
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

                    debug_assert_ne!(*presence, LabelPresence::Forbidden, "Forbidden labels are not implemented yet");

                    if let Some((marked_path, current_presence)) = roots.get_mut(label) {
                        if *current_presence > *presence {
                            debug_assert!(false, "RootPathTrie precondition violation; while walking the tree label presence is only allowed to increase");
                        }
                        *marked_path = ProjectPath {worktree_id, path: path.clone()};
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
            let depth = path
                .strip_prefix(&root_path.path)
                .unwrap()
                .components()
                .count();

            if depth > 0 {
                dbg!(&path, &root_path.path, depth);
                let root = adapter.0.find_project_root(&path, depth, &delegate);
                match root {
                    Some(known_root) => worktree_roots.update(cx, |this, _| {
                        let root = TriePath::from(&*known_root);
                        this.roots
                            .insert(&root, adapter.0.name(), LabelPresence::Present);
                        *presence = LabelPresence::Present;
                        *root_path = ProjectPath {
                            worktree_id,
                            path: known_root,
                        };
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
                presence.eq(&LabelPresence::Present).then(|| (k, path))
            })
            .collect()
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
