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
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::{Arc, Weak},
};

use collections::{HashMap, IndexMap};
use gpui::{App, AppContext as _, Entity, Subscription};
use itertools::Itertools;
use language::{
    language_settings::AllLanguageSettings, Attach, LanguageName, LanguageRegistry,
    LspAdapterDelegate,
};
use lsp::LanguageServerName;
use settings::{Settings, SettingsLocation, WorktreeId};
use std::sync::OnceLock;

use crate::{project_settings::LspSettings, LanguageServerId, ProjectPath};

use super::{AdapterWrapper, ProjectTree, ProjectTreeEvent};

#[derive(Debug, Default)]
struct ServersForWorktree {
    roots: BTreeMap<
        Arc<Path>,
        BTreeMap<LanguageServerName, (Arc<InnerTreeNode>, BTreeSet<LanguageName>)>,
    >,
}

pub struct LanguageServerTree {
    project_tree: Entity<ProjectTree>,
    instances: BTreeMap<WorktreeId, ServersForWorktree>,
    attach_kind_cache: HashMap<LanguageServerName, Attach>,
    languages: Arc<LanguageRegistry>,
    _subscriptions: Subscription,
}

/// A node in language server tree represents either:
/// - A language server that has already been initialized/updated for a given project
/// - A soon-to-be-initialized language server.
#[derive(Clone)]
pub(crate) struct LanguageServerTreeNode(Weak<InnerTreeNode>);

/// Describes a request to launch a language server.
#[derive(Debug)]
pub(crate) struct LaunchDisposition<'a> {
    pub(crate) server_name: &'a LanguageServerName,
    pub(crate) attach: Attach,
    pub(crate) path: ProjectPath,
    pub(crate) settings: Arc<LspSettings>,
}

impl<'a> From<&'a InnerTreeNode> for LaunchDisposition<'a> {
    fn from(value: &'a InnerTreeNode) -> Self {
        LaunchDisposition {
            server_name: &value.name,
            attach: value.attach,
            path: value.path.clone(),
            settings: value.settings.clone(),
        }
    }
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
        init: impl FnOnce(LaunchDisposition) -> LanguageServerId,
    ) -> Option<LanguageServerId> {
        let this = self.0.upgrade()?;
        Some(
            *this
                .id
                .get_or_init(|| init(LaunchDisposition::from(&*this))),
        )
    }
}

impl From<Weak<InnerTreeNode>> for LanguageServerTreeNode {
    fn from(weak: Weak<InnerTreeNode>) -> Self {
        LanguageServerTreeNode(weak)
    }
}

#[derive(Debug)]
struct InnerTreeNode {
    id: OnceLock<LanguageServerId>,
    name: LanguageServerName,
    attach: Attach,
    path: ProjectPath,
    settings: Arc<LspSettings>,
}

impl InnerTreeNode {
    fn new(
        name: LanguageServerName,
        attach: Attach,
        path: ProjectPath,
        settings: impl Into<Arc<LspSettings>>,
    ) -> Self {
        InnerTreeNode {
            id: Default::default(),
            name,
            attach,
            path,
            settings: settings.into(),
        }
    }
}

/// Determines how the list of adapters to query should be constructed.
pub(crate) enum AdapterQuery<'a> {
    /// Search for roots of all adapters associated with a given language name.
    Language(&'a LanguageName),
    /// Search for roots of adapter with a given name.
    Adapter(&'a LanguageServerName),
}

impl LanguageServerTree {
    pub(crate) fn new(
        project_tree: Entity<ProjectTree>,
        languages: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| Self {
            _subscriptions: cx.subscribe(
                &project_tree,
                |_: &mut Self, _, event, _| {
                    if event == &ProjectTreeEvent::Cleared {}
                },
            ),
            project_tree,
            instances: Default::default(),
            attach_kind_cache: Default::default(),
            languages,
        })
    }
    /// Memoize calls to attach_kind on LspAdapter (which might be a WASM extension, thus ~expensive to call).
    fn attach_kind(&mut self, adapter: &AdapterWrapper) -> Attach {
        *self
            .attach_kind_cache
            .entry(adapter.0.name.clone())
            .or_insert_with(|| adapter.0.attach_kind())
    }

    /// Get all language server root points for a given path and language; the language servers might already be initialized at a given path.
    pub(crate) fn get<'a>(
        &'a mut self,
        path: ProjectPath,
        query: AdapterQuery<'_>,
        delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut App,
    ) -> impl Iterator<Item = LanguageServerTreeNode> + 'a {
        let settings_location = SettingsLocation {
            worktree_id: path.worktree_id,
            path: &path.path,
        };
        let adapters = match query {
            AdapterQuery::Language(language_name) => {
                self.adapters_for_language(settings_location, language_name, cx)
            }
            AdapterQuery::Adapter(language_server_name) => IndexMap::from_iter(
                self.adapter_for_name(language_server_name)
                    .map(|adapter| (adapter, (LspSettings::default(), BTreeSet::new()))),
            ),
        };
        self.get_with_adapters(path, adapters, delegate, cx)
    }

    fn get_with_adapters<'a>(
        &'a mut self,
        path: ProjectPath,
        adapters: IndexMap<AdapterWrapper, (LspSettings, BTreeSet<LanguageName>)>,
        delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut App,
    ) -> impl Iterator<Item = LanguageServerTreeNode> + 'a {
        let worktree_id = path.worktree_id;
        #[allow(clippy::mutable_key_type)]
        let mut roots = self.project_tree.update(cx, |this, cx| {
            this.root_for_path(
                path,
                adapters
                    .iter()
                    .map(|(adapter, _)| adapter.0.clone())
                    .collect(),
                delegate,
                cx,
            )
        });
        let mut root_path = None;
        // Backwards-compat: Fill in any adapters for which we did not detect the root as having the project root at the root of a worktree.
        for (adapter, _) in adapters.iter() {
            roots.entry(adapter.clone()).or_insert_with(|| {
                root_path
                    .get_or_insert_with(|| ProjectPath {
                        worktree_id,
                        path: Arc::from("".as_ref()),
                    })
                    .clone()
            });
        }

        roots
            .into_iter()
            .filter_map(move |(adapter, root_path)| {
                let attach = self.attach_kind(&adapter);
                let (index, _, (settings, new_languages)) = adapters.get_full(&adapter)?;
                let inner_node = self
                    .instances
                    .entry(root_path.worktree_id)
                    .or_default()
                    .roots
                    .entry(root_path.path.clone())
                    .or_default()
                    .entry(adapter.0.name.clone());
                let (node, languages) = inner_node.or_insert_with(move || {
                    (
                        Arc::new(InnerTreeNode::new(
                            adapter.0.name(),
                            attach,
                            root_path,
                            settings.clone(),
                        )),
                        Default::default(),
                    )
                });
                languages.extend(new_languages.iter().cloned());
                Some((index, Arc::downgrade(&node).into()))
            })
            .sorted_by_key(|(index, _)| *index)
            .map(|(_, node)| node)
    }

    fn adapter_for_name(&self, name: &LanguageServerName) -> Option<AdapterWrapper> {
        self.languages.adapter_for_name(name).map(AdapterWrapper)
    }

    fn adapters_for_language(
        &self,
        settings_location: SettingsLocation,
        language_name: &LanguageName,
        cx: &App,
    ) -> IndexMap<AdapterWrapper, (LspSettings, BTreeSet<LanguageName>)> {
        let settings = AllLanguageSettings::get(Some(settings_location), cx).language(
            Some(settings_location),
            Some(language_name),
            cx,
        );
        if !settings.enable_language_server {
            return Default::default();
        }
        let available_lsp_adapters = self.languages.lsp_adapters(&language_name);
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
                Some((
                    AdapterWrapper(adapter),
                    (
                        adapter_settings,
                        BTreeSet::from_iter([language_name.clone()]),
                    ),
                ))
            })
            .collect::<IndexMap<_, _>>();
        // After starting all the language servers, reorder them to reflect the desired order
        // based on the settings.
        //
        // This is done, in part, to ensure that language servers loaded at different points
        // (e.g., native vs extension) still end up in the right order at the end, rather than
        // it being based on which language server happened to be loaded in first.
        self.languages.reorder_language_servers(
            &language_name,
            adapters_with_settings
                .keys()
                .map(|wrapper| wrapper.0.clone())
                .collect(),
        );

        adapters_with_settings
    }

    // Rebasing a tree:
    // - Clears it out
    // - Provides you with the indirect access to the old tree while you're reinitializing a new one (by querying it).
    pub(crate) fn rebase(&mut self) -> ServerTreeRebase<'_> {
        ServerTreeRebase::new(self)
    }

    /// Remove nodes with a given ID from the tree.
    pub(crate) fn remove_nodes(&mut self, ids: &BTreeSet<LanguageServerId>) {
        for (_, servers) in &mut self.instances {
            for (_, nodes) in &mut servers.roots {
                nodes.retain(|_, (node, _)| node.id.get().map_or(true, |id| !ids.contains(&id)));
            }
        }
    }
}

pub(crate) struct ServerTreeRebase<'a> {
    old_contents: BTreeMap<WorktreeId, ServersForWorktree>,
    new_tree: &'a mut LanguageServerTree,
    /// All server IDs seen in the old tree.
    all_server_ids: BTreeMap<LanguageServerId, LanguageServerName>,
    /// Server IDs we've preserved for a new iteration of the tree. `all_server_ids - rebased_server_ids` is the
    /// set of server IDs that can be shut down.
    rebased_server_ids: BTreeSet<LanguageServerId>,
}

impl<'tree> ServerTreeRebase<'tree> {
    fn new(new_tree: &'tree mut LanguageServerTree) -> Self {
        let old_contents = std::mem::take(&mut new_tree.instances);
        new_tree.attach_kind_cache.clear();
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
                            .map(|id| (id, server.0.name.clone()))
                    })
                })
            })
            .collect();
        Self {
            old_contents,
            new_tree,
            all_server_ids,
            rebased_server_ids: BTreeSet::new(),
        }
    }

    pub(crate) fn get<'a>(
        &'a mut self,
        path: ProjectPath,
        query: AdapterQuery<'_>,
        delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut App,
    ) -> impl Iterator<Item = LanguageServerTreeNode> + 'a {
        let settings_location = SettingsLocation {
            worktree_id: path.worktree_id,
            path: &path.path,
        };
        let adapters = match query {
            AdapterQuery::Language(language_name) => {
                self.new_tree
                    .adapters_for_language(settings_location, language_name, cx)
            }
            AdapterQuery::Adapter(language_server_name) => IndexMap::from_iter(
                self.new_tree
                    .adapter_for_name(language_server_name)
                    .map(|adapter| (adapter, (LspSettings::default(), BTreeSet::new()))),
            ),
        };

        self.new_tree
            .get_with_adapters(path, adapters, delegate, cx)
            .filter_map(|node| {
                // Inspect result of the query and initialize it ourselves before
                // handing it off to the caller.
                let disposition = node.0.upgrade()?;

                if disposition.id.get().is_some() {
                    return Some(node);
                }
                let Some((existing_node, _)) = self
                    .old_contents
                    .get(&disposition.path.worktree_id)
                    .and_then(|worktree_nodes| worktree_nodes.roots.get(&disposition.path.path))
                    .and_then(|roots| roots.get(&disposition.name))
                    .filter(|(old_node, _)| {
                        disposition.attach == old_node.attach
                            && disposition.settings == old_node.settings
                    })
                else {
                    return Some(node);
                };
                if let Some(existing_id) = existing_node.id.get() {
                    self.rebased_server_ids.insert(*existing_id);
                    disposition.id.set(*existing_id).ok();
                }

                Some(node)
            })
    }

    /// Returns IDs of servers that are no longer referenced (and can be shut down).
    pub(crate) fn finish(self) -> BTreeMap<LanguageServerId, LanguageServerName> {
        self.all_server_ids
            .into_iter()
            .filter(|(id, _)| !self.rebased_server_ids.contains(id))
            .collect()
    }
}
