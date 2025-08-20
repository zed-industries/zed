//! This module defines an LSP Tree.
//!
//! An LSP Tree is responsible for determining which language servers apply to a given project path.
//!
//! ## RPC
//! LSP Tree is transparent to RPC peers; when clients ask host to spawn a new language server, the host will perform LSP Tree lookup for provided path; it may decide
//! to reuse existing language server.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::{Arc, Weak},
};

use collections::IndexMap;
use gpui::{App, Entity};
use language::{
    CachedLspAdapter, LanguageName, LanguageRegistry, ManifestDelegate, ManifestName, Toolchain,
    language_settings::AllLanguageSettings,
};
use lsp::LanguageServerName;
use settings::{Settings, SettingsLocation, WorktreeId};
use std::sync::OnceLock;

use crate::{
    LanguageServerId, ProjectPath, project_settings::LspSettings,
    toolchain_store::LocalToolchainStore,
};

use super::ManifestTree;

#[derive(Clone, Debug, Default)]
pub(crate) struct ServersForWorktree {
    pub(crate) roots: BTreeMap<
        Arc<Path>,
        BTreeMap<LanguageServerName, (Arc<InnerTreeNode>, BTreeSet<LanguageName>)>,
    >,
}

pub struct LanguageServerTree {
    manifest_tree: Entity<ManifestTree>,
    pub(crate) instances: BTreeMap<WorktreeId, ServersForWorktree>,
    languages: Arc<LanguageRegistry>,
    toolchains: Entity<LocalToolchainStore>,
}

/// A node in language server tree represents either:
/// - A language server that has already been initialized/updated for a given project
/// - A soon-to-be-initialized language server.
#[derive(Clone)]
pub struct LanguageServerTreeNode(Weak<InnerTreeNode>);

/// Describes a request to launch a language server.
#[derive(Clone, Debug)]
pub(crate) struct LaunchDisposition {
    pub(crate) server_name: LanguageServerName,
    /// Path to the root directory of a subproject.
    pub(crate) path: ProjectPath,
    pub(crate) settings: Arc<LspSettings>,
    pub(crate) toolchain: Option<Toolchain>,
}

impl LanguageServerTreeNode {
    /// Returns a language server ID for this node if there is one.
    /// Returns None if this node has not been initialized yet or it is no longer in the tree.
    pub(crate) fn server_id(&self) -> Option<LanguageServerId> {
        self.0.upgrade()?.id.get().copied()
    }

    /// Returns a language server ID for this node if it has already been initialized; otherwise runs the provided closure to initialize the language server node in a tree.
    /// May return None if the node no longer belongs to the server tree it was created in.
    pub(crate) fn server_id_or_init(
        &self,
        init: impl FnOnce(&Arc<LaunchDisposition>) -> LanguageServerId,
    ) -> Option<LanguageServerId> {
        let this = self.0.upgrade()?;
        Some(*this.id.get_or_init(|| init(&this.disposition)))
    }

    /// Returns a language server name as the language server adapter would return.
    pub fn name(&self) -> Option<LanguageServerName> {
        self.0
            .upgrade()
            .map(|node| node.disposition.server_name.clone())
    }
}

impl From<Weak<InnerTreeNode>> for LanguageServerTreeNode {
    fn from(weak: Weak<InnerTreeNode>) -> Self {
        LanguageServerTreeNode(weak)
    }
}

#[derive(Debug)]
pub struct InnerTreeNode {
    id: OnceLock<LanguageServerId>,
    disposition: Arc<LaunchDisposition>,
}

impl InnerTreeNode {
    fn new(
        server_name: LanguageServerName,
        path: ProjectPath,
        settings: LspSettings,
        toolchain: Option<Toolchain>,
    ) -> Self {
        InnerTreeNode {
            id: Default::default(),
            disposition: Arc::new(LaunchDisposition {
                server_name,
                path,
                settings: settings.into(),
                toolchain,
            }),
        }
    }
}

impl LanguageServerTree {
    pub(crate) fn new(
        manifest_tree: Entity<ManifestTree>,
        languages: Arc<LanguageRegistry>,
        toolchains: Entity<LocalToolchainStore>,
    ) -> Self {
        Self {
            manifest_tree,
            instances: Default::default(),
            languages,
            toolchains,
        }
    }

    /// Get all initialized language server IDs for a given path.
    pub(crate) fn get<'a>(
        &'a self,
        path: ProjectPath,
        language_name: LanguageName,
        manifest_name: Option<&ManifestName>,
        delegate: &Arc<dyn ManifestDelegate>,
        cx: &mut App,
    ) -> impl Iterator<Item = LanguageServerId> + 'a {
        let manifest_location = self.manifest_location_for_path(&path, manifest_name, delegate, cx);
        let adapters = self.adapters_for_language(&manifest_location, &language_name, cx);
        self.get_with_adapters(manifest_location, adapters)
    }

    /// Get all language server root points for a given path and language; the language servers might already be initialized at a given path.
    pub(crate) fn walk<'a>(
        &'a mut self,
        path: ProjectPath,
        language_name: LanguageName,
        manifest_name: Option<&ManifestName>,
        delegate: &Arc<dyn ManifestDelegate>,
        cx: &'a mut App,
    ) -> impl Iterator<Item = LanguageServerTreeNode> + 'a {
        let manifest_location = self.manifest_location_for_path(&path, manifest_name, delegate, cx);
        let adapters = self.adapters_for_language(&manifest_location, &language_name, cx);
        self.init_with_adapters(manifest_location, language_name, adapters, cx)
    }

    fn init_with_adapters<'a>(
        &'a mut self,
        root_path: ProjectPath,
        language_name: LanguageName,
        adapters: IndexMap<LanguageServerName, (LspSettings, Arc<CachedLspAdapter>)>,
        cx: &'a App,
    ) -> impl Iterator<Item = LanguageServerTreeNode> + 'a {
        adapters.into_iter().map(move |(_, (settings, adapter))| {
            let root_path = root_path.clone();
            let inner_node = self
                .instances
                .entry(root_path.worktree_id)
                .or_default()
                .roots
                .entry(root_path.path.clone())
                .or_default()
                .entry(adapter.name());
            let (node, languages) = inner_node.or_insert_with(|| {
                let toolchain = self.toolchains.read(cx).active_toolchain(
                    root_path.worktree_id,
                    &root_path.path,
                    language_name.clone(),
                );

                (
                    Arc::new(InnerTreeNode::new(
                        adapter.name(),
                        root_path.clone(),
                        settings.clone(),
                        toolchain,
                    )),
                    Default::default(),
                )
            });
            languages.insert(language_name.clone());
            Arc::downgrade(node).into()
        })
    }

    fn get_with_adapters<'a>(
        &'a self,
        root_path: ProjectPath,
        adapters: IndexMap<LanguageServerName, (LspSettings, Arc<CachedLspAdapter>)>,
    ) -> impl Iterator<Item = LanguageServerId> + 'a {
        adapters.into_iter().filter_map(move |(_, (_, adapter))| {
            let root_path = root_path.clone();
            let inner_node = self
                .instances
                .get(&root_path.worktree_id)?
                .roots
                .get(&root_path.path)?
                .get(&adapter.name())?;
            inner_node.0.id.get().copied()
        })
    }

    fn manifest_location_for_path(
        &self,
        path: &ProjectPath,
        manifest_name: Option<&ManifestName>,
        delegate: &Arc<dyn ManifestDelegate>,
        cx: &mut App,
    ) -> ProjectPath {
        // Find out what the root location of our subproject is.
        // That's where we'll look for language settings (that include a set of language servers).
        self.manifest_tree.update(cx, |this, cx| {
            this.root_for_path_or_worktree_root(path, manifest_name, delegate, cx)
        })
    }

    fn adapters_for_language(
        &self,
        manifest_location: &ProjectPath,
        language_name: &LanguageName,
        cx: &App,
    ) -> IndexMap<LanguageServerName, (LspSettings, Arc<CachedLspAdapter>)> {
        let settings_location = SettingsLocation {
            worktree_id: manifest_location.worktree_id,
            path: &manifest_location.path,
        };
        let settings = AllLanguageSettings::get(Some(settings_location), cx).language(
            Some(settings_location),
            Some(language_name),
            cx,
        );
        if !settings.enable_language_server {
            return Default::default();
        }
        let available_lsp_adapters = self.languages.lsp_adapters(language_name);
        let available_language_servers = available_lsp_adapters
            .iter()
            .map(|lsp_adapter| lsp_adapter.name.clone())
            .collect::<Vec<_>>();

        let desired_language_servers =
            settings.customized_language_servers(&available_language_servers);
        let adapters_with_settings = desired_language_servers
            .into_iter()
            .filter_map(|desired_adapter| {
                let adapter = if let Some(adapter) = available_lsp_adapters
                    .iter()
                    .find(|adapter| adapter.name == desired_adapter)
                {
                    Some(adapter.clone())
                } else if let Some(adapter) =
                    self.languages.load_available_lsp_adapter(&desired_adapter)
                {
                    self.languages
                        .register_lsp_adapter(language_name.clone(), adapter.adapter.clone());
                    Some(adapter)
                } else {
                    None
                }?;
                let adapter_settings = crate::lsp_store::language_server_settings_for(
                    settings_location,
                    &adapter.name,
                    cx,
                )
                .cloned()
                .unwrap_or_default();
                Some((adapter.name(), (adapter_settings, adapter)))
            })
            .collect::<IndexMap<_, _>>();
        // After starting all the language servers, reorder them to reflect the desired order
        // based on the settings.
        //
        // This is done, in part, to ensure that language servers loaded at different points
        // (e.g., native vs extension) still end up in the right order at the end, rather than
        // it being based on which language server happened to be loaded in first.
        self.languages.reorder_language_servers(
            language_name,
            adapters_with_settings
                .values()
                .map(|(_, adapter)| adapter.clone())
                .collect(),
        );

        adapters_with_settings
    }

    /// Server Tree is built up incrementally via queries for distinct paths of the worktree.
    /// Results of these queries have to be invalidated when data used to build the tree changes.
    ///
    /// The environment of a server tree is a set of all user settings.
    /// Rebasing a tree means invalidating it and building up a new one while reusing the old tree where applicable.
    /// We want to reuse the old tree in order to preserve as many of the running language servers as possible.
    /// E.g. if the user disables one of their language servers for Python, we don't want to shut down any language servers unaffected by this settings change.
    ///
    /// Thus, [`ServerTreeRebase`] mimics the interface of a [`ServerTree`], except that it tries to find a matching language server in the old tree before handing out an uninitialized node.
    pub(crate) fn rebase(&mut self) -> ServerTreeRebase {
        ServerTreeRebase::new(self)
    }

    /// Remove nodes with a given ID from the tree.
    pub(crate) fn remove_nodes(&mut self, ids: &BTreeSet<LanguageServerId>) {
        for servers in self.instances.values_mut() {
            for nodes in &mut servers.roots.values_mut() {
                nodes.retain(|_, (node, _)| node.id.get().is_none_or(|id| !ids.contains(id)));
            }
        }
    }

    pub(crate) fn register_reused(
        &mut self,
        worktree_id: WorktreeId,
        language_name: LanguageName,
        reused: LanguageServerTreeNode,
    ) {
        let Some(node) = reused.0.upgrade() else {
            return;
        };

        self.instances
            .entry(worktree_id)
            .or_default()
            .roots
            .entry(Arc::from(Path::new("")))
            .or_default()
            .entry(node.disposition.server_name.clone())
            .or_insert_with(|| (node, BTreeSet::new()))
            .1
            .insert(language_name);
    }
}

pub(crate) struct ServerTreeRebase {
    old_contents: BTreeMap<WorktreeId, ServersForWorktree>,
    new_tree: LanguageServerTree,
    /// All server IDs seen in the old tree.
    all_server_ids: BTreeMap<LanguageServerId, LanguageServerName>,
    /// Server IDs we've preserved for a new iteration of the tree. `all_server_ids - rebased_server_ids` is the
    /// set of server IDs that can be shut down.
    rebased_server_ids: BTreeSet<LanguageServerId>,
}

impl ServerTreeRebase {
    fn new(old_tree: &LanguageServerTree) -> Self {
        let old_contents = old_tree.instances.clone();
        let all_server_ids = old_contents
            .values()
            .flat_map(|nodes| {
                nodes.roots.values().flat_map(|servers| {
                    servers.values().filter_map(|server| {
                        server
                            .0
                            .id
                            .get()
                            .copied()
                            .map(|id| (id, server.0.disposition.server_name.clone()))
                    })
                })
            })
            .collect();
        let new_tree = LanguageServerTree::new(
            old_tree.manifest_tree.clone(),
            old_tree.languages.clone(),
            old_tree.toolchains.clone(),
        );
        Self {
            old_contents,
            all_server_ids,
            new_tree,
            rebased_server_ids: BTreeSet::new(),
        }
    }

    pub(crate) fn walk<'a>(
        &'a mut self,
        path: ProjectPath,
        language_name: LanguageName,
        manifest_name: Option<&ManifestName>,
        delegate: Arc<dyn ManifestDelegate>,
        cx: &'a mut App,
    ) -> impl Iterator<Item = LanguageServerTreeNode> + 'a {
        let manifest =
            self.new_tree
                .manifest_location_for_path(&path, manifest_name, &delegate, cx);
        let adapters = self
            .new_tree
            .adapters_for_language(&manifest, &language_name, cx);

        self.new_tree
            .init_with_adapters(manifest, language_name, adapters, cx)
            .filter_map(|node| {
                // Inspect result of the query and initialize it ourselves before
                // handing it off to the caller.
                let live_node = node.0.upgrade()?;

                if live_node.id.get().is_some() {
                    return Some(node);
                }

                let disposition = &live_node.disposition;
                let Some((existing_node, _)) = self
                    .old_contents
                    .get(&disposition.path.worktree_id)
                    .and_then(|worktree_nodes| worktree_nodes.roots.get(&disposition.path.path))
                    .and_then(|roots| roots.get(&disposition.server_name))
                    .filter(|(old_node, _)| {
                        (&disposition.toolchain, &disposition.settings)
                            == (
                                &old_node.disposition.toolchain,
                                &old_node.disposition.settings,
                            )
                    })
                else {
                    return Some(node);
                };
                if let Some(existing_id) = existing_node.id.get() {
                    self.rebased_server_ids.insert(*existing_id);
                    live_node.id.set(*existing_id).ok();
                }

                Some(node)
            })
    }

    /// Returns IDs of servers that are no longer referenced (and can be shut down).
    pub(crate) fn finish(
        self,
    ) -> (
        LanguageServerTree,
        BTreeMap<LanguageServerId, LanguageServerName>,
    ) {
        (
            self.new_tree,
            self.all_server_ids
                .into_iter()
                .filter(|(id, _)| !self.rebased_server_ids.contains(id))
                .collect(),
        )
    }

    pub(crate) fn server_tree(&mut self) -> &mut LanguageServerTree {
        &mut self.new_tree
    }
}
