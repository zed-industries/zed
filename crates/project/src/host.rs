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
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, EntityId, Global, Subscription, WeakEntity,
};
use language::LanguageRegistry;
use node_runtime::NodeRuntime;
use remote::RemoteClient;
use rpc::proto;
use snippet_provider::SnippetProvider;
use util::paths::PathStyle;

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
    toolchain_store::{EmptyToolchainStore, ToolchainStore},
    worktree_store::{WorktreeIdCounter, WorktreeStore},
};

/// Identifies a "machine" for purposes of `Host` deduplication. Two
/// `Project`s with the same `HostKey` share a single `Entity<Host>`.
///
/// - `Local(fs_identity)`: keyed on the `Arc<dyn Fs>` pointer. In
///   production there's a single `RealFs` shared across the app, so
///   all `Project::local` calls dedupe. In tests each invocation
///   typically constructs a fresh `FakeFs`, which gives each test
///   project its own `Host` (correct — distinct test fixtures
///   shouldn't conflate machine state). Tests that want to exercise
///   actual sharing pass the same `Arc<dyn Fs>` to multiple
///   `Project::local` calls (which collab's `TestClient` already
///   does).
/// - `Remote(EntityId)`: keyed on the `RemoteClient`'s entity id.
///   Two `Project::remote` calls referencing the same `RemoteClient`
///   share a `Host`; calls referencing different `RemoteClient`s do
///   not.
/// - `Collab(remote_id)`: kept for completeness; collab joins do not
///   dedupe (see `Host::collab`).
#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum HostKey {
    Local(usize),
    Remote(EntityId),
    Collab(u64),
}

impl HostKey {
    /// Computes the `Local` key from an `Arc<dyn Fs>` by its address.
    pub fn local_for_fs(fs: &Arc<dyn Fs>) -> Self {
        Self::Local(Arc::as_ptr(fs) as *const () as usize)
    }
}

/// Global registry that maps a `HostKey` to a weak handle on the live
/// `Host` for that machine. Entries become invalid when the last
/// strong reference to the `Host` drops; the registry prunes stale
/// entries lazily on lookup.
#[derive(Default)]
pub struct HostRegistry {
    hosts: HashMap<HostKey, WeakEntity<Host>>,
}

impl Global for HostRegistry {}

/// Initializes the global `HostRegistry`. Called from `Project::init`.
pub fn init(cx: &mut App) {
    if !cx.has_global::<HostRegistry>() {
        cx.set_global(HostRegistry::default());
    }
}

impl HostRegistry {
    /// Resolves an existing `Host` for `key`, or builds a new one via
    /// `build` and registers it. The build closure runs only when no
    /// live `Host` exists for the key, so its side effects (scanner
    /// startup, file watchers, etc.) happen exactly once per machine.
    ///
    /// Currently unused at call sites — kept as the Phase 2 entry point
    /// for actual `Host` deduplication. See the long comment in
    /// `Host::local` for why dedup is gated behind making the
    /// underlying stores multi-tenant-tolerant first.
    #[allow(dead_code)]
    fn get_or_build(
        cx: &mut App,
        key: HostKey,
        build: impl FnOnce(&mut App) -> Entity<Host>,
    ) -> Entity<Host> {
        if let Some(existing) = cx
            .try_global::<HostRegistry>()
            .and_then(|registry| registry.hosts.get(&key).cloned())
            .and_then(|weak| weak.upgrade())
        {
            return existing;
        }
        let host = build(cx);
        if !cx.has_global::<HostRegistry>() {
            cx.set_global(HostRegistry::default());
        }
        cx.global_mut::<HostRegistry>()
            .hosts
            .insert(key, host.downgrade());
        host
    }

    /// Async variant for `Host::collab` (which takes `&mut AsyncApp`).
    /// Same Phase 2 status as `get_or_build`.
    #[allow(dead_code)]
    fn get_or_build_async(
        cx: &mut AsyncApp,
        key: HostKey,
        build: impl FnOnce(&mut AsyncApp) -> Entity<Host>,
    ) -> Entity<Host> {
        if let Some(existing) = cx.update(|cx| {
            cx.try_global::<HostRegistry>()
                .and_then(|registry| registry.hosts.get(&key).cloned())
                .and_then(|weak| weak.upgrade())
        }) {
            return existing;
        }
        let host = build(cx);
        cx.update(|cx| {
            if !cx.has_global::<HostRegistry>() {
                cx.set_global(HostRegistry::default());
            }
            cx.global_mut::<HostRegistry>()
                .hosts
                .insert(key, host.downgrade());
        });
        host
    }
}

/// Machine-bound dependencies and host-shaped stores shared by a
/// `Project`.
///
/// In Phase 1 each `Project` constructed its own `Host`. Phase 2 (this
/// commit) introduces `HostRegistry` keyed on `HostKey`, so multiple
/// `Project`s targeting the same machine share an `Entity<Host>` and
/// the host-shaped stores beneath it.
pub struct Host {
    /// `cx.on_release` / `cx.on_app_quit` subscriptions retained for
    /// the lifetime of this `Host`. Used to shut down
    /// `remote_client`-spawned processes when the host is released or
    /// the application quits. Phase 2 makes this the single source of
    /// truth for SSH/WSL process shutdown across shared `Project`s.
    _subscriptions: Vec<Subscription>,
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
        let key = HostKey::local_for_fs(&fs);
        HostRegistry::get_or_build(cx, key, move |cx| {
            Self::build_local(
                client,
                node,
                user_store,
                languages,
                fs,
                env,
                watch_global_configs,
                cx,
            )
        })
    }

    fn build_local(
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

            let context_server_store =
                cx.new(|cx| ContextServerStore::local(worktree_store.clone(), false, cx));

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
                _subscriptions: Vec::new(),
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

    /// Build a `Host` for a remote-server project (ssh / wsl).
    ///
    /// `Project::remote` calls this and then sets up Project-level
    /// subscriptions, the trusted-worktree tracker, and the
    /// `WeakEntity<Project>` back-ref on `ContextServerStore`.
    pub fn remote(
        remote: Entity<RemoteClient>,
        client: Arc<Client>,
        node: NodeRuntime,
        user_store: Entity<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut App,
    ) -> Entity<Self> {
        let key = HostKey::Remote(remote.entity_id());
        HostRegistry::get_or_build(cx, key, move |cx| {
            Self::build_remote(remote, client, node, user_store, languages, fs, cx)
        })
    }

    fn build_remote(
        remote: Entity<RemoteClient>,
        client: Arc<Client>,
        node: NodeRuntime,
        user_store: Entity<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut App,
    ) -> Entity<Self> {
        let (remote_proto, path_style) =
            remote.read_with(cx, |remote, _| (remote.proto_client(), remote.path_style()));

        cx.new(|cx: &mut Context<Self>| {
            let snippets = SnippetProvider::new(fs.clone(), BTreeSet::from_iter([]), cx);
            let worktree_store = cx.new(|cx| {
                WorktreeStore::remote(
                    remote_proto.clone(),
                    rpc::proto::REMOTE_SERVER_PROJECT_ID,
                    path_style,
                    WorktreeIdCounter::get(cx),
                )
            });

            let buffer_store = cx.new(|cx| {
                BufferStore::remote(
                    worktree_store.clone(),
                    remote_proto.clone(),
                    rpc::proto::REMOTE_SERVER_PROJECT_ID,
                    cx,
                )
            });
            let image_store = cx.new(|cx| {
                ImageStore::remote(
                    worktree_store.clone(),
                    remote_proto.clone(),
                    rpc::proto::REMOTE_SERVER_PROJECT_ID,
                    cx,
                )
            });
            let toolchain_store = cx.new(|cx| {
                ToolchainStore::remote(
                    rpc::proto::REMOTE_SERVER_PROJECT_ID,
                    worktree_store.clone(),
                    remote_proto.clone(),
                    cx,
                )
            });

            let context_server_store = cx.new(|cx| {
                ContextServerStore::remote(
                    rpc::proto::REMOTE_SERVER_PROJECT_ID,
                    remote.clone(),
                    worktree_store.clone(),
                    cx,
                )
            });

            let environment = cx.new(|cx| {
                ProjectEnvironment::new(
                    None,
                    worktree_store.downgrade(),
                    Some(remote.downgrade()),
                    false,
                    cx,
                )
            });

            let lsp_store = cx.new(|cx| {
                LspStore::new_remote(
                    buffer_store.clone(),
                    worktree_store.clone(),
                    languages.clone(),
                    remote_proto.clone(),
                    rpc::proto::REMOTE_SERVER_PROJECT_ID,
                    cx,
                )
            });

            let bookmark_store =
                cx.new(|_| BookmarkStore::new(worktree_store.clone(), buffer_store.clone()));

            let breakpoint_store = cx.new(|_| {
                BreakpointStore::remote(
                    rpc::proto::REMOTE_SERVER_PROJECT_ID,
                    remote_proto.clone(),
                    buffer_store.clone(),
                    worktree_store.clone(),
                )
            });

            let dap_store = cx.new(|cx| {
                DapStore::new_remote(
                    rpc::proto::REMOTE_SERVER_PROJECT_ID,
                    remote.clone(),
                    breakpoint_store.clone(),
                    worktree_store.clone(),
                    node.clone(),
                    client.http_client(),
                    fs.clone(),
                    cx,
                )
            });

            let git_store = cx.new(|cx| {
                GitStore::remote(
                    &worktree_store,
                    buffer_store.clone(),
                    remote_proto.clone(),
                    rpc::proto::REMOTE_SERVER_PROJECT_ID,
                    cx,
                )
            });

            let task_store = cx.new(|cx| {
                TaskStore::remote(
                    buffer_store.downgrade(),
                    worktree_store.clone(),
                    toolchain_store.read(cx).as_language_toolchain_store(),
                    remote_proto.clone(),
                    rpc::proto::REMOTE_SERVER_PROJECT_ID,
                    git_store.clone(),
                    cx,
                )
            });

            let settings_observer = cx.new(|cx| {
                SettingsObserver::new_remote(
                    fs.clone(),
                    worktree_store.clone(),
                    task_store.clone(),
                    Some(remote_proto.clone()),
                    false,
                    cx,
                )
            });

            let agent_server_store = cx.new(|_| {
                AgentServerStore::remote(
                    rpc::proto::REMOTE_SERVER_PROJECT_ID,
                    remote.clone(),
                    worktree_store.clone(),
                )
            });

            Self {
                _subscriptions: vec![
                    cx.on_release(Self::release_remote_client),
                    cx.on_app_quit(|host, cx| host.on_app_quit_shutdown(cx)),
                ],
                fs,
                languages,
                node: Some(node),
                user_store,
                collab_client: client,
                remote_client: Some(remote),
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

    /// Build a `Host` for a collab-joined project. The `Host` here
    /// represents the *remote machine* we joined as a participant, even
    /// though this code runs in the local app; almost everything is
    /// remote-state and the local toolchain / node runtime aren't
    /// available.
    ///
    /// `Project::from_join_project_response` calls this with the project
    /// id / role / replica id parsed from the join response, then sets
    /// up Project-level subscriptions and back-references.
    pub fn collab(
        remote_id: u64,
        path_style: PathStyle,
        client: Arc<Client>,
        run_tasks: bool,
        user_store: Entity<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut AsyncApp,
    ) -> Entity<Self> {
        Self::build_collab(
            remote_id, path_style, client, run_tasks, user_store, languages, fs, cx,
        )
    }

    fn build_collab(
        remote_id: u64,
        path_style: PathStyle,
        client: Arc<Client>,
        run_tasks: bool,
        user_store: Entity<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut AsyncApp,
    ) -> Entity<Self> {
        cx.new(|cx: &mut Context<Self>| {
            let snippets = SnippetProvider::new(fs.clone(), BTreeSet::from_iter([]), cx);
            let proto_client: rpc::AnyProtoClient = client.clone().into();
            let worktree_store = cx.new(|cx| {
                WorktreeStore::remote(
                    proto_client.clone(),
                    remote_id,
                    path_style,
                    WorktreeIdCounter::get(cx),
                )
            });
            let buffer_store = cx.new(|cx| {
                BufferStore::remote(worktree_store.clone(), proto_client.clone(), remote_id, cx)
            });
            let image_store = cx.new(|cx| {
                ImageStore::remote(worktree_store.clone(), proto_client.clone(), remote_id, cx)
            });

            let environment = cx.new(|cx| {
                ProjectEnvironment::new(None, worktree_store.downgrade(), None, true, cx)
            });

            let bookmark_store =
                cx.new(|_| BookmarkStore::new(worktree_store.clone(), buffer_store.clone()));

            let breakpoint_store = cx.new(|_| {
                BreakpointStore::remote(
                    remote_id,
                    proto_client.clone(),
                    buffer_store.clone(),
                    worktree_store.clone(),
                )
            });
            let dap_store = cx.new(|cx| {
                DapStore::new_collab(
                    remote_id,
                    proto_client.clone(),
                    breakpoint_store.clone(),
                    worktree_store.clone(),
                    fs.clone(),
                    cx,
                )
            });

            let lsp_store = cx.new(|cx| {
                LspStore::new_remote(
                    buffer_store.clone(),
                    worktree_store.clone(),
                    languages.clone(),
                    proto_client.clone(),
                    remote_id,
                    cx,
                )
            });

            let git_store = cx.new(|cx| {
                GitStore::remote(
                    &worktree_store,
                    buffer_store.clone(),
                    proto_client.clone(),
                    remote_id,
                    cx,
                )
            });

            let task_store = cx.new(|cx| {
                if run_tasks {
                    TaskStore::remote(
                        buffer_store.downgrade(),
                        worktree_store.clone(),
                        Arc::new(EmptyToolchainStore),
                        proto_client.clone(),
                        remote_id,
                        git_store.clone(),
                        cx,
                    )
                } else {
                    TaskStore::Noop
                }
            });

            let settings_observer = cx.new(|cx| {
                SettingsObserver::new_remote(
                    fs.clone(),
                    worktree_store.clone(),
                    task_store.clone(),
                    None,
                    true,
                    cx,
                )
            });

            let agent_server_store = cx.new(|_cx| AgentServerStore::collab());

            // Note: collab uses `ContextServerStore::local` (not
            // `::remote`), matching the pre-Host wiring.
            let context_server_store =
                cx.new(|cx| ContextServerStore::local(worktree_store.clone(), false, cx));

            Self {
                _subscriptions: Vec::new(),
                fs,
                languages,
                node: None,
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
                toolchain_store: None,
            }
        })
    }

    /// Synchronously take and shut down `remote_client`. Used both as
    /// the `on_release` callback (Project drops, Host drops, last
    /// reference gone) and as the body of the `on_app_quit` handler
    /// (whole app shutting down). The shutdown future is dropped on
    /// release; on app-quit the caller awaits it.
    fn release_remote_client(&mut self, cx: &mut App) {
        let Some(remote) = self.remote_client.take() else {
            return;
        };
        let shutdown = remote.update(cx, |client, cx| {
            client.shutdown_processes(
                Some(proto::ShutdownRemoteServer {}),
                cx.background_executor().clone(),
            )
        });
        cx.background_spawn(async move {
            if let Some(shutdown) = shutdown {
                shutdown.await;
            }
        })
        .detach();
    }

    fn on_app_quit_shutdown(&mut self, cx: &mut Context<Self>) -> gpui::Task<()> {
        let shutdown = self.remote_client.take().and_then(|client| {
            client.update(cx, |client, cx| {
                client.shutdown_processes(
                    Some(proto::ShutdownRemoteServer {}),
                    cx.background_executor().clone(),
                )
            })
        });
        cx.background_executor().spawn(async move {
            if let Some(shutdown) = shutdown {
                shutdown.await;
            }
        })
    }
}
