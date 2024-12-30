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
    sync::{Arc, OnceLock},
};

use collections::HashMap;
use gpui::{AppContext, Context as _, Model};
use language::{Attach, LanguageName, LanguageRegistry};
use lsp::LanguageServerName;

use crate::{LanguageServerId, ProjectPath};

use super::{AdapterWrapper, ProjectTree};

pub struct LanguageServerTree {
    /// Language servers for which we can just update workspaceFolders when we detect a new project root
    project_tree: Model<ProjectTree>,
    languages: Arc<LanguageRegistry>,
    instances: HashMap<ProjectPath, BTreeMap<LanguageServerName, LanguageServerTreeNode>>,
    attach_kind_cache: HashMap<LanguageServerName, Attach>,
}

/// A node in language server tree represents either:
/// - A language server that has already been initialized/updated for a given project
/// - A soon-to-be-initialized language server.
#[derive(Clone)]
pub(crate) struct LanguageServerTreeNode(Arc<InnerTreeNode>);

impl LanguageServerTreeNode {
    fn new(name: LanguageServerName, attach: Attach, path: ProjectPath) -> Self {
        Self(Arc::new(InnerTreeNode {
            id: Default::default(),
            name,
            attach,
            path,
        }))
    }
    /// Returns a language server ID for this node if there is one; if a language server has not been started yet
    /// for this path, returns None.
    pub(crate) fn server_id(&self) -> Option<LanguageServerId> {
        self.0.id.get().copied()
    }
    pub(crate) fn server_id_or_init(
        &self,
        init: impl FnOnce(&LanguageServerName, Attach, ProjectPath) -> LanguageServerId,
    ) -> LanguageServerId {
        *self
            .0
            .id
            .get_or_init(|| init(&self.0.name, self.0.attach, self.0.path.clone()))
    }
}

struct InnerTreeNode {
    id: OnceLock<LanguageServerId>,
    name: LanguageServerName,
    attach: Attach,
    path: ProjectPath,
}

impl LanguageServerTree {
    pub(crate) fn new(
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
    fn attach_kind(&mut self, adapter: &AdapterWrapper) -> Attach {
        *self
            .attach_kind_cache
            .entry(adapter.0.name.clone())
            .or_insert_with(|| adapter.0.attach_kind())
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

        roots.into_iter().map(|(adapter, root_path)| {
            let attach = self.attach_kind(&adapter);
            self.instances
                .entry(root_path.clone())
                .or_default()
                .entry(adapter.0.name.clone())
                .or_insert_with(|| LanguageServerTreeNode::new(adapter.0.name(), attach, root_path))
                .clone()
        })
    }
}
