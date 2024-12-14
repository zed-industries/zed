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
use language::{LanguageName, LanguageRegistry};
use lsp::{LanguageServerName, Url};
use settings::WorktreeId;

use crate::{LanguageServerId, ProjectPath};

pub trait LspRootFinder {
    fn find_root(&self) -> () {}
}

enum Action {
    PinpointTo,
    ExtendWorkspaceFolders(LanguageServerId, Url),
}

pub type AbsWorkspaceRootPath = Arc<Path>;

pub struct ProjectTree {
    languages: Arc<LanguageRegistry>,
    root_points: HashMap<WorktreeId, BTreeMap<LanguageServerName, BTreeMap<Arc<Path>, IsRoot>>>,
}

type IsRoot = bool;
impl ProjectTree {
    fn new(languages: Arc<LanguageRegistry>) -> Self {
        Self {
            languages,
            root_points: Default::default(),
        }
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
                    // Look at roots detected so far. We should be executing this branch most of the time.
                    // Todo: only search up until worktree root. Or should we? What about anonymous worktrees created for RA when browsing std?
                    // they're rooted at the file being browsed.
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
}
