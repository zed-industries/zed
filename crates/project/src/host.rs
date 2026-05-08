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

use std::sync::Arc;

use client::{Client, UserStore};
use fs::Fs;
use gpui::Entity;
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
    project_settings::SettingsObserver,
    task_store::TaskStore,
    toolchain_store::ToolchainStore,
    worktree_store::WorktreeStore,
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
