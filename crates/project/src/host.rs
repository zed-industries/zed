//! Machine-bound, host-level services and stores shared by a `Project`.
//!
//! `Host` is the construction and ownership boundary for everything that
//! depends on the *machine* (filesystem, node runtime, language servers,
//! debug adapters, etc.) rather than on a particular workspace or
//! collaboration session. `Project` holds an `Entity<Host>` and forwards
//! its host-shaped accessors through it.
//!
//! In Phase 1 this is a single-`Project`-per-`Host` setup: each
//! `Project::local`/`remote`/`from_join_project_response` constructs a
//! fresh `Host`. Phase 2 will introduce a host registry so that multiple
//! `Project`s targeting the same machine can share a `Host`, at which
//! point lifecycle (e.g. `cx.on_app_quit` shutdown of remote processes)
//! moves here from `Project`.

use std::{collections::BTreeSet, sync::Arc};

use client::{Client, UserStore};
use collections::HashMap;
use fs::Fs;
use gpui::{App, AppContext as _, Context, Entity};
use language::LanguageRegistry;
use node_runtime::NodeRuntime;
use remote::RemoteClient;
use snippet_provider::SnippetProvider;

use crate::{
    agent_server_store::AgentServerStore,
    bookmark_store::BookmarkStore,
    buffer_store::BufferStore,
    context_server_store::ContextServerStore,
    debugger::{breakpoint_store::BreakpointStore, dap_store::DapStore},
    environment::ProjectEnvironment,
    git_store::GitStore,
    image_store::ImageStore,
    lsp_store::LspStore,
    manifest_tree::ManifestTree,
    prettier_store::PrettierStore,
    project_settings::SettingsObserver,
    task_store::TaskStore,
    toolchain_store::ToolchainStore,
    worktree_store::{WorktreeIdCounter, WorktreeStore},
};

/// Machine-bound dependencies and host-shaped stores shared by a
/// `Project`.
///
/// In Phase 1 each `Project` constructs its own `Host`. Phase 2 will
/// share `Host`s across `Project`s targeting the same machine via a
/// registry; this struct's shape is designed for that future.
pub struct Host {
    // Machine identity / I/O primitives.
    pub fs: Arc<dyn Fs>,
    pub languages: Arc<LanguageRegistry>,
    pub node: Option<NodeRuntime>,
    pub user_store: Entity<UserStore>,
    pub collab_client: Arc<Client>,
    pub remote_client: Option<Entity<RemoteClient>>,
    pub environment: Entity<ProjectEnvironment>,
    pub snippets: Entity<SnippetProvider>,

    // Host-level stores.
    pub worktree_store: Entity<WorktreeStore>,
    pub buffer_store: Entity<BufferStore>,
    pub image_store: Entity<ImageStore>,
    pub lsp_store: Entity<LspStore>,
    pub dap_store: Entity<DapStore>,
    pub breakpoint_store: Entity<BreakpointStore>,
    pub bookmark_store: Entity<BookmarkStore>,
    pub git_store: Entity<GitStore>,
    pub task_store: Entity<TaskStore>,
    pub settings_observer: Entity<SettingsObserver>,
    pub agent_server_store: Entity<AgentServerStore>,
    pub context_server_store: Entity<ContextServerStore>,
    pub toolchain_store: Option<Entity<ToolchainStore>>,
}

impl Host {
    /// Build a `Host` for a local project (no remote-server, no collab).
    ///
    /// `Project::local` calls this and then sets up Project-level
    /// subscriptions and back-references (such as the optional
    /// `WeakEntity<Project>` on `ContextServerStore`). All entity
    /// construction (worktree/buffer/lsp/etc. stores and ancillary
    /// services like `ManifestTree` / `PrettierStore`) happens here so
    /// the `Host` is the canonical owner of the per-machine wiring.
    pub fn local(
        client: Arc<Client>,
        node: NodeRuntime,
        user_store: Entity<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        env: Option<HashMap<String, String>>,
        watch_global_configs: bool,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx: &mut Context<Self>| {
            let snippets = SnippetProvider::new(fs.clone(), BTreeSet::from_iter([]), cx);
            let worktree_store =
                cx.new(|cx| WorktreeStore::local(fs.clone(), WorktreeIdCounter::get(cx)));

            // `weak_project` is filled in by `Project::local` after the
            // `Project` entity exists.
            let context_server_store =
                cx.new(|cx| ContextServerStore::local(worktree_store.clone(), None, false, cx));

            let environment = cx.new(|cx| {
                ProjectEnvironment::new(env, worktree_store.downgrade(), None, false, cx)
            });
            let manifest_tree = ManifestTree::new(worktree_store.clone(), cx);
            let toolchain_store = cx.new(|cx| {
                ToolchainStore::local(
                    languages.clone(),
                    worktree_store.clone(),
                    environment.clone(),
                    manifest_tree.clone(),
                    cx,
                )
            });

            let buffer_store = cx.new(|cx| BufferStore::local(worktree_store.clone(), cx));

            let bookmark_store =
                cx.new(|_| BookmarkStore::new(worktree_store.clone(), buffer_store.clone()));

            let breakpoint_store =
                cx.new(|_| BreakpointStore::local(worktree_store.clone(), buffer_store.clone()));

            let dap_store = cx.new(|cx| {
                DapStore::new_local(
                    client.http_client(),
                    node.clone(),
                    fs.clone(),
                    environment.clone(),
                    toolchain_store.read(cx).as_language_toolchain_store(),
                    worktree_store.clone(),
                    breakpoint_store.clone(),
                    false,
                    cx,
                )
            });

            let image_store = cx.new(|cx| ImageStore::local(worktree_store.clone(), cx));

            let prettier_store = cx.new(|cx| {
                PrettierStore::new(
                    node.clone(),
                    fs.clone(),
                    languages.clone(),
                    worktree_store.clone(),
                    cx,
                )
            });

            let git_store = cx.new(|cx| {
                GitStore::local(
                    &worktree_store,
                    buffer_store.clone(),
                    environment.clone(),
                    fs.clone(),
                    cx,
                )
            });

            let task_store = cx.new(|cx| {
                TaskStore::local(
                    buffer_store.downgrade(),
                    worktree_store.clone(),
                    toolchain_store.read(cx).as_language_toolchain_store(),
                    environment.clone(),
                    git_store.clone(),
                    cx,
                )
            });

            let settings_observer = cx.new(|cx| {
                SettingsObserver::new_local(
                    fs.clone(),
                    worktree_store.clone(),
                    task_store.clone(),
                    watch_global_configs,
                    cx,
                )
            });

            let lsp_store = cx.new(|cx| {
                LspStore::new_local(
                    buffer_store.clone(),
                    worktree_store.clone(),
                    prettier_store,
                    toolchain_store
                        .read(cx)
                        .as_local_store()
                        .expect("Toolchain store to be local")
                        .clone(),
                    environment.clone(),
                    manifest_tree,
                    languages.clone(),
                    client.http_client(),
                    fs.clone(),
                    cx,
                )
            });

            let agent_server_store = cx.new(|cx| {
                AgentServerStore::local(
                    node.clone(),
                    fs.clone(),
                    environment.clone(),
                    client.http_client(),
                    cx,
                )
            });

            Self {
                fs,
                languages,
                node: Some(node),
                user_store,
                collab_client: client,
                remote_client: None,
                environment,
                snippets,
                worktree_store,
                buffer_store,
                image_store,
                lsp_store,
                dap_store,
                breakpoint_store,
                bookmark_store,
                git_store,
                task_store,
                settings_observer,
                agent_server_store,
                context_server_store,
                toolchain_store: Some(toolchain_store),
            }
        })
    }
}
