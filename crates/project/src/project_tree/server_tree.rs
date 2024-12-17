//! This module defines an LSP Tree.
//!
//! An LSP Tree is responsible for determining which language servers apply to a given project path.
//!
//! ## RPC
//! LSP Tree is transparent to RPC peers; when clients ask host to spawn a new language server, the host will perform LSP Tree lookup for provided path; it may decide
//! to reuse existing language server. The client maintains it's own LSP Tree that is a subset of host LSP Tree. Done this way, the client does not need to
//! ask about suitable language server for each path it interacts with; it can resolve most of the queries locally.
//! This module defines a Project Tree.

use std::{
    collections::BTreeMap,
    path::Path,
    sync::{Arc, OnceLock},
};

use collections::HashMap;
use gpui::{AppContext, Context as _, Model};
use language::{LanguageName, LanguageRegistry};
use lsp::LanguageServerName;

use crate::{LanguageServerId, ProjectPath};

use super::ProjectTree;

pub type AbsWorkspaceRootPath = Arc<Path>;

pub struct LanguageServerTree {
    /// Language servers for which we can just update workspaceFolders when we detect a new project root
    project_tree: Model<ProjectTree>,
    languages: Arc<LanguageRegistry>,
    instances: HashMap<ProjectPath, BTreeMap<LanguageServerName, LanguageServerTreeNode>>,
    // shared_instances: BTreeMap<WorktreeId, BTreeMap<LanguageServerName, LanguageServerId>>,
    attach_kind_cache: HashMap<LanguageServerName, Attach>,
}

#[derive(Clone, Copy, PartialEq)]
enum Attach {
    /// Create a single language server instance per subproject root.
    InstancePerRoot,
    /// Use one shared language server instance for all subprojects within a project.
    Shared,
}

impl Attach {
    fn root_path(&self, root_subproject_path: ProjectPath) -> ProjectPath {
        match self {
            Attach::InstancePerRoot => root_subproject_path,
            Attach::Shared => ProjectPath {
                worktree_id: root_subproject_path.worktree_id,
                path: Arc::from(Path::new("")),
            },
        }
    }
}

#[derive(Clone)]
pub(crate) struct LanguageServerTreeNode(Arc<InnerTreeNode>);

impl LanguageServerTreeNode {
    fn new(attach: Attach) -> Self {
        Self(Arc::new(InnerTreeNode {
            id: Default::default(),
            attach,
        }))
    }
}
struct InnerTreeNode {
    id: OnceLock<LanguageServerId>,
    attach: Attach,
}

impl LanguageServerTree {
    fn new(
        languages: Arc<LanguageRegistry>,
        project_tree: Model<ProjectTree>,
        cx: &mut AppContext,
    ) -> Model<Self> {
        cx.new_model(|_| Self {
            project_tree,
            languages,
            instances: Default::default(),
            attach_kind_cache: Default::default(),
        })
    }
    fn attach_kind(&mut self, name: LanguageServerName) -> Attach {
        *self
            .attach_kind_cache
            .entry(name)
            .or_insert_with(|| Attach::Shared)
        // todo: query lspadapter for it.
    }

    pub(crate) fn get<'a>(
        &'a mut self,
        path: ProjectPath,
        language: LanguageName,
        cx: &mut AppContext,
    ) -> impl Iterator<Item = LanguageServerTreeNode> + 'a {
        let roots = self
            .project_tree
            .update(cx, |this, cx| this.root_for_path(path, &language, cx));

        roots.into_iter().map(|(adapter_name, root_path)| {
            let attach = self.attach_kind(adapter_name.clone());
            self.instances
                .entry(root_path)
                .or_default()
                .entry(adapter_name)
                .or_insert_with(|| LanguageServerTreeNode::new(attach))
                .clone()
        })
    }
}

#[cfg(test)]
mod tests {}
