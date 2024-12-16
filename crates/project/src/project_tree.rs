//! This module defines a Project Tree.
//!
//! A Project Tree is responsible for determining where the roots of subprojects are located in a project.

mod server_tree;
mod toolchain_tree;

use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::Arc,
};

use collections::HashMap;
use gpui::{AppContext, Context as _, Model, ModelContext, Subscription};
use language::{LanguageName, LanguageRegistry};
use lsp::LanguageServerName;
use settings::WorktreeId;

use crate::{
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
    ProjectPath,
};

type IsRoot = bool;
pub struct ProjectTree {
    languages: Arc<LanguageRegistry>,
    root_points: HashMap<WorktreeId, BTreeMap<LanguageServerName, BTreeMap<Arc<Path>, IsRoot>>>,
    worktree_store: Model<WorktreeStore>,
    _subscriptions: [Subscription; 1],
}

impl ProjectTree {
    fn new(
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
    ) -> Vec<ProjectPath> {
        let mut roots = vec![];
        let adapters = self.languages.lsp_adapters(&language_name);
        let worktree_roots = self.root_points.entry(worktree_id).or_default();

        'adapter: for adapter in adapters {
            if let Some(adapter_roots) = worktree_roots.get(&adapter.name()) {
                for ancestor in path.ancestors().skip(1) {
                    if let Some(is_root) = adapter_roots.get(ancestor) {
                        if *is_root {
                            roots.push((worktree_id, ancestor).into());
                        }
                        continue 'adapter;
                    }
                }
            }
            // Ask adapter what the closest root is.
            let root = adapter.find_closest_project_root(worktree_id, path.clone());
            let is_known_root = root.is_some();
            worktree_roots
                .entry(adapter.name())
                .or_default()
                .entry(root.clone().unwrap_or_else(|| Arc::from(Path::new(""))))
                .or_insert(is_known_root);
            if let Some(root) = root {
                roots.push((worktree_id, root).into());
            }
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
