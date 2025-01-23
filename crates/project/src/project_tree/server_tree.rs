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
use gpui::{AppContext, Context as _, Model, Subscription};
use language::{
    language_settings::AllLanguageSettings, Attach, LanguageName, LanguageRegistry,
    LspAdapterDelegate,
};
use lsp::LanguageServerName;
use once_cell::sync::OnceCell;
use settings::{Settings, SettingsLocation, WorktreeId};
use util::maybe;

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
    project_tree: Model<ProjectTree>,
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
        self.server_id_or_try_init(|disposition| Ok(init(disposition)))
    }
    fn server_id_or_try_init(
        &self,
        init: impl FnOnce(LaunchDisposition) -> Result<LanguageServerId, ()>,
    ) -> Option<LanguageServerId> {
        let this = self.0.upgrade()?;
        this.id
            .get_or_try_init(|| init(LaunchDisposition::from(&*this)))
            .ok()
            .copied()
    }
}

impl From<Weak<InnerTreeNode>> for LanguageServerTreeNode {
    fn from(weak: Weak<InnerTreeNode>) -> Self {
        LanguageServerTreeNode(weak)
    }
}

#[derive(Debug)]
struct InnerTreeNode {
    id: OnceCell<LanguageServerId>,
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

impl LanguageServerTree {
    pub(crate) fn new(
        project_tree: Model<ProjectTree>,
        languages: Arc<LanguageRegistry>,
        cx: &mut AppContext,
    ) -> Model<Self> {
        cx.new_model(|cx| Self {
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
        language_name: &LanguageName,
        delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut AppContext,
    ) -> impl Iterator<Item = LanguageServerTreeNode> + 'a {
        let settings_location = SettingsLocation {
            worktree_id: path.worktree_id,
            path: &path.path,
        };
        let adapters = self.adapters_for_language(settings_location, language_name, cx);
        self.get_with_adapters(path, adapters, delegate, cx)
    }

    fn get_with_adapters<'a>(
        &'a mut self,
        path: ProjectPath,
        adapters: IndexMap<AdapterWrapper, (LspSettings, BTreeSet<LanguageName>)>,
        delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut AppContext,
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

        roots.into_iter().filter_map(move |(adapter, root_path)| {
            let attach = self.attach_kind(&adapter);
            let (settings, new_languages) = adapters.get(&adapter).cloned()?;
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
                        settings,
                    )),
                    Default::default(),
                )
            });
            languages.extend(new_languages);
            Some(Arc::downgrade(&node).into())
        })
    }

    fn adapters_for_language(
        &self,
        settings_location: SettingsLocation,
        language_name: &LanguageName,
        cx: &AppContext,
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
        adapters_with_settings
    }

    pub(crate) fn on_settings_changed(
        &mut self,
        get_delegate: &mut dyn FnMut(
            WorktreeId,
            &mut AppContext,
        ) -> Option<Arc<dyn LspAdapterDelegate>>,
        spawn_language_server: &mut dyn FnMut(
            LaunchDisposition,
            &mut AppContext,
        ) -> LanguageServerId,
        on_language_server_removed: &mut dyn FnMut(LanguageServerId),
        cx: &mut AppContext,
    ) {
        // Settings are checked at query time. Thus, to avoid messing with inference of applicable settings, we're just going to clear ourselves and let the next query repopulate.
        // We're going to optimistically re-run the queries and re-assign the same language server id when a language server still exists at a given tree node.
        let old_instances = std::mem::take(&mut self.instances);
        let old_attach_kinds = std::mem::take(&mut self.attach_kind_cache);

        let mut referenced_instances = BTreeSet::new();
        // Re-map the old tree onto a new one. In the process we'll get a list of servers we have to shut down.
        let mut all_instances = BTreeSet::new();

        for (worktree_id, servers) in &old_instances {
            // Record all initialized node ids.
            all_instances.extend(servers.roots.values().flat_map(|servers_at_node| {
                servers_at_node
                    .values()
                    .filter_map(|(server_node, _)| server_node.id.get().copied())
            }));
            let Some(delegate) = get_delegate(*worktree_id, cx) else {
                // If worktree is no longer around, we're just going to shut down all of the language servers (since they've been added to all_instances).
                continue;
            };

            for (path, servers_for_path) in &servers.roots {
                for (server_name, (_, languages)) in servers_for_path {
                    let settings_location = SettingsLocation {
                        worktree_id: *worktree_id,
                        path: &path,
                    };
                    // Verify which of the previous languages still have this server enabled.

                    let mut adapter_with_settings = IndexMap::default();

                    for language_name in languages {
                        self.adapters_for_language(settings_location, language_name, cx)
                            .into_iter()
                            .for_each(|(lsp_adapter, lsp_settings)| {
                                if &lsp_adapter.0.name() != server_name {
                                    return;
                                }
                                adapter_with_settings
                                    .entry(lsp_adapter)
                                    .and_modify(|x: &mut (_, BTreeSet<LanguageName>)| {
                                        x.1.extend(lsp_settings.1.clone())
                                    })
                                    .or_insert(lsp_settings);
                            });
                    }

                    if adapter_with_settings.is_empty() {
                        // Since all languages that have had this server enabled are now disabled, we can remove the server entirely.
                        continue;
                    };

                    for new_node in self.get_with_adapters(
                        ProjectPath {
                            path: path.clone(),
                            worktree_id: *worktree_id,
                        },
                        adapter_with_settings,
                        delegate.clone(),
                        cx,
                    ) {
                        new_node.server_id_or_try_init(|disposition| {
                            let Some((existing_node, _)) = servers
                                .roots
                                .get(&disposition.path.path)
                                .and_then(|roots| roots.get(disposition.server_name))
                                .filter(|(old_node, _)| {
                                    old_attach_kinds.get(disposition.server_name).map_or(
                                        false,
                                        |old_attach| {
                                            disposition.attach == *old_attach
                                                && disposition.settings == old_node.settings
                                        },
                                    )
                                })
                            else {
                                return Ok(spawn_language_server(disposition, cx));
                            };
                            if let Some(id) = existing_node.id.get().copied() {
                                // If we have a node with ID assigned (and it's parameters match `disposition`), reuse the id.
                                referenced_instances.insert(id);
                                Ok(id)
                            } else {
                                // Otherwise, if we do have a node but it does not have an ID assigned, keep it that way.
                                Err(())
                            }
                        });
                    }
                }
            }
        }
        for server_to_remove in all_instances.difference(&referenced_instances) {
            on_language_server_removed(*server_to_remove);
        }
    }

    /// Updates nodes in language server tree in place, changing the ID of initialized nodes.
    pub(crate) fn restart_language_servers(
        &mut self,
        worktree_id: WorktreeId,
        ids: BTreeSet<LanguageServerId>,
        restart_callback: &mut dyn FnMut(LanguageServerId, LaunchDisposition) -> LanguageServerId,
    ) {
        maybe! {{
                for (_, nodes) in &mut self.instances.get_mut(&worktree_id)?.roots {
                    for (_, (node, _)) in nodes {
                        let Some(old_server_id) = node.id.get().copied() else {
                            continue;
                        };
                        if !ids.contains(&old_server_id) {
                            continue;
                        }

                        let new_id = restart_callback(old_server_id, LaunchDisposition::from(&**node));

                        *node = Arc::new(InnerTreeNode::new(node.name.clone(), node.attach, node.path.clone(), node.settings.clone()));
                        node.id.set(new_id).expect("The id to be unset after clearing the node.");
                    }
            }
            Some(())
        }
        };
    }
}
