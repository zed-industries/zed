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
    sync::{Arc, OnceLock, Weak},
};

use collections::HashMap;
use gpui::{AppContext, Context as _, Model, Subscription};
use language::{
    language_settings::AllLanguageSettings, Attach, CachedLspAdapter, LanguageName,
    LanguageRegistry, LspAdapterDelegate,
};
use lsp::LanguageServerName;
use settings::{Settings as _, SettingsLocation, WorktreeId};

use crate::{LanguageServerId, ProjectPath};

use super::{AdapterWrapper, ProjectTree, ProjectTreeEvent};

#[derive(Default)]
struct ServersForWorktree {
    roots: BTreeMap<Arc<Path>, BTreeMap<LanguageServerName, Arc<InnerTreeNode>>>,
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

struct LaunchDisposition<'a> {
    server_name: &'a LanguageServerName,
    attach: Attach,
    path: ProjectPath,
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
        Some(*this.id.get_or_init(|| {
            init(LaunchDisposition {
                server_name: &this.name,
                attach: this.attach,
                path: this.path.clone(),
            })
        }))
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
}

impl InnerTreeNode {
    fn new(name: LanguageServerName, attach: Attach, path: ProjectPath) -> Self {
        InnerTreeNode {
            id: OnceLock::new(),
            name,
            attach,
            path,
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
        let adapters = self.adapters_for_language(&path, language_name, cx);
        #[allow(clippy::mutable_key_type)]
        let roots = self.project_tree.update(cx, |this, cx| {
            this.root_for_path(path, adapters, delegate, cx)
        });
        roots.into_iter().map(|(adapter, root_path)| {
            let attach = self.attach_kind(&adapter);
            let inner_node = self
                .instances
                .entry(root_path.worktree_id)
                .or_default()
                .roots
                .entry(root_path.path.clone())
                .or_default()
                .entry(adapter.0.name.clone())
                .or_insert_with(|| {
                    Arc::new(InnerTreeNode::new(adapter.0.name(), attach, root_path))
                });
            Arc::downgrade(inner_node).into()
        })
    }

    fn adapters_for_language(
        &self,
        path: &ProjectPath,
        language_name: &LanguageName,
        cx: &AppContext,
    ) -> Vec<Arc<CachedLspAdapter>> {
        let available_lsp_adapters = self.languages.lsp_adapters(&language_name);
        let settings_location = SettingsLocation {
            worktree_id: path.worktree_id,
            path: &path.path,
        };
        let settings = AllLanguageSettings::get(Some(settings_location), cx).language(
            Some(settings_location),
            Some(language_name),
            cx,
        );
        let available_language_servers = available_lsp_adapters
            .iter()
            .map(|lsp_adapter| lsp_adapter.name.clone())
            .collect::<Vec<_>>();

        let desired_language_servers =
            settings.customized_language_servers(&available_language_servers);
        let adapters = desired_language_servers
            .into_iter()
            .filter_map(|desired_adapter| {
                if let Some(adapter) = available_lsp_adapters
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
                }
            })
            .collect::<Vec<_>>();
        adapters
    }
    pub(crate) fn on_settings_changed(
        &mut self,
        get_delegate: &mut dyn FnMut(WorktreeId, &mut AppContext) -> Arc<dyn LspAdapterDelegate>,
        mut on_language_server_added: &mut dyn FnMut(LaunchDisposition),
        mut on_language_server_removed: &mut dyn FnMut(LanguageServerId),
        cx: &mut AppContext,
    ) {
        // Settings are checked at query time. Thus, to avoid messing with inference of applicable settings, we're just going to clear ourselves and let the next query repopulate.
        // We're going to optimistically re-run the queries and re-assign the same language server id when a language server still exists at a given tree node.
        let old_instances = std::mem::take(&mut self.instances);

        let mut referenced_instances = BTreeSet::new();
        // Re-map the old tree onto a new one. In the process we'll get a list of servers we have to shut down.
        let mut all_instances = BTreeSet::new();
        for (worktree_id, servers) in old_instances {
            let delegate = get_delegate(worktree_id, cx);

            // Record all initialized node ids.
            all_instances.extend(servers.roots.values().flat_map(|servers_at_node| {
                servers_at_node
                    .values()
                    .filter_map(|server_node| server_node.id.get())
            }));
            while let Some((path, servers)) = servers.roots.pop_first() {
                for (name, nodes) in servers {
                    self.get(
                        ProjectPath {
                            path: path.clone(),
                            worktree_id,
                        },
                        delegate.clone(),
                        cx,
                    );
                }
            }
        }
    }
}
