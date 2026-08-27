//! Per-workspace lifecycle for the Claude Code IDE bridge.
//!
//! On workspace open: generate an auth token, start the server, write the lockfile,
//! inject the discovery env vars, and subscribe to editor selection changes. All of
//! it is torn down when the workspace closes.
//!
//! The bridge entity is tracked in a GPUI global keyed by workspace `EntityId` (see
//! [`BridgeRegistry`]) so handlers can find it without forking `Workspace`.

use collections::HashMap;
use editor::{Editor, EditorEvent};
use gpui::{
    Action, App, AppContext as _, Context, Entity, EntityId, Global, Subscription, Task,
    WeakEntity,
};
use std::path::PathBuf;
use std::time::Duration;
use workspace::Workspace;

use crate::lockfile::{self, LockData};
use crate::selection::{self, SharedSelection};
use crate::selection_socket;
use crate::server::{self, BridgeServer};

/// Debounce window for coalescing rapid selection changes before pushing a
/// `selection_changed` notification.
const SELECTION_DEBOUNCE: Duration = Duration::from_millis(50);

/// Editor→Claude-Code notification names (wire contract).
const SELECTION_CHANGED: &str = "selection_changed";
const AT_MENTIONED: &str = "at_mentioned";

gpui::actions!(
    claude_code_ide,
    [
        /// Send the active editor selection to the connected Claude Code session.
        SendSelectionToClaudeCode
    ]
);

/// Maps a workspace to its bridge so handlers can find it without a `Workspace`
/// field.
#[derive(Default)]
struct BridgeRegistry {
    bridges: HashMap<EntityId, WeakEntity<ClaudeCodeIde>>,
}

impl Global for BridgeRegistry {}

fn register_bridge(workspace: EntityId, bridge: &Entity<ClaudeCodeIde>, cx: &mut App) {
    cx.default_global::<BridgeRegistry>()
        .bridges
        .insert(workspace, bridge.downgrade());
}

fn bridge_for(workspace: EntityId, cx: &App) -> Option<Entity<ClaudeCodeIde>> {
    cx.try_global::<BridgeRegistry>()?
        .bridges
        .get(&workspace)
        .and_then(WeakEntity::upgrade)
}

/// Feed in a selection captured outside the Zed editor, keyed by `source`.
///
/// The broadcast is the AGGREGATE across every source, so an editor and two other
/// editors send all three regions together rather than only the newest.
pub(crate) fn push_external_selection(
    workspace_id: EntityId,
    source: String,
    payload: crate::protocol::SelectionPayload,
    owner_pid: Option<u32>,
    cx: &mut App,
) {
    with_bridge(workspace_id, cx, |bridge| {
        bridge.push_external_selection(source, payload, owner_pid)
    });
}

/// Drop one source's selection, for a client whose selection has gone away. Without
/// it the region would ride in every later broadcast for the life of the workspace.
pub(crate) fn clear_external_selection(workspace_id: EntityId, source: String, cx: &mut App) {
    with_bridge(workspace_id, cx, |bridge| {
        bridge.clear_external_selection(&source)
    });
}

/// Drop every source's selection. The escape hatch for a region whose owner died
/// leaving no pid to reap by, so nothing can address its key.
pub(crate) fn clear_all_external_selections(workspace_id: EntityId, cx: &mut App) {
    with_bridge(workspace_id, cx, |bridge| {
        bridge.clear_all_external_selections()
    });
}

fn with_bridge(
    workspace_id: EntityId,
    cx: &mut App,
    update: impl FnOnce(&mut ClaudeCodeIde),
) {
    let Some(bridge) = bridge_for(workspace_id, cx) else {
        log::debug!("claude_code_ide: no live bridge for workspace {workspace_id}");
        return;
    };
    bridge.update(cx, |bridge, _cx| update(bridge));
}

/// Owns the bridge for one workspace. Dropped when the workspace closes, which
/// stops the server (via the contained task handles) and unlinks the lockfile
/// (via the release observer registered in [`start_for_workspace`]).
pub struct ClaudeCodeIde {
    selection: SharedSelection,
    /// `None` until the async server bind completes.
    server: Option<BridgeServer>,
    lock_path: Option<PathBuf>,
    /// Bound path of the selection socket, kept so it can be unlinked on
    /// release. `None` when the socket could not be bound.
    selection_socket_path: Option<PathBuf>,
    /// Accept loop for the selection socket. Dropping it stops accepting.
    selection_socket: Option<Task<()>>,
    /// Subscription to the active editor's events; replaced whenever the active
    /// item changes.
    editor_subscription: Option<Subscription>,
    /// Pending debounce task for the next selection push.
    pending_push: Option<Task<()>>,
    workspace: WeakEntity<Workspace>,
    _startup: Task<()>,
    _workspace_subscription: Subscription,
}

/// Register the bridge to start for every workspace.
pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        start_for_workspace(workspace, window.window_handle(), cx);
    })
    .detach();

    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(
            |workspace, _: &SendSelectionToClaudeCode, _window, cx| {
                let workspace_id = cx.entity_id();
                if let Some(bridge) = bridge_for(workspace_id, cx) {
                    bridge.update(cx, |bridge, cx| bridge.send_at_mention(cx));
                }
                let _ = workspace;
            },
        );
    })
    .detach();
}

/// Start the bridge for a single workspace.
pub fn start_for_workspace(
    workspace: &mut Workspace,
    window: gpui::AnyWindowHandle,
    cx: &mut Context<Workspace>,
) {
    let selection = SharedSelection::new();
    selection.set_workspace(cx.weak_entity());
    selection.set_window(window);

    let auth_token = lockfile::generate_auth_token();
    let workspace_folders = workspace
        .project()
        .read(cx)
        .visible_worktrees(cx)
        .map(|worktree| worktree.read(cx).abs_path().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    let workspace_id = cx.entity_id();

    // Bind the server and, once bound, write the lockfile + inject env vars.
    let startup = cx.spawn({
        let selection = selection.clone();
        async move |workspace_handle, cx| {
            let server = match server::start(auth_token.clone(), selection.clone(), cx).await {
                Ok(server) => server,
                Err(err) => {
                    log::error!("claude_code_ide: failed to start server: {err}");
                    return;
                }
            };
            let port = server.port();

            let lock_data = LockData::new(workspace_folders, auth_token);
            let lock_path = match lockfile::write_lock(port, &lock_data) {
                Ok(path) => Some(path),
                Err(err) => {
                    log::error!("claude_code_ide: failed to write lockfile: {err}");
                    None
                }
            };

            // A selection socket lets programs running in this workspace's
            // terminals (an nvim, a shell script) contribute selections the Zed
            // editor path cannot see. Failing to bind costs only that extra
            // source, so the bridge carries on without it.
            let (socket_path, socket_task) = match selection_socket::start(workspace_id, port, cx) {
                Ok((path, task)) => (Some(path), Some(task)),
                Err(err) => {
                    log::warn!(
                        "claude_code_ide: selection socket unavailable, external \
                         selection sources disabled: {err:#}"
                    );
                    (None, None)
                }
            };

            let _ = workspace_handle.update(cx, |workspace, cx| {
                // Inject discovery env vars so `claude` in this workspace's
                // terminals auto-connects, plus the selection socket path for any
                // relay running there. Only affects terminals spawned after this
                // point, so it has to happen during workspace startup.
                workspace.project().update(cx, |project, _| {
                    project.set_terminal_env_var(
                        "CLAUDE_CODE_SSE_PORT".to_string(),
                        port.to_string(),
                    );
                    project.set_terminal_env_var(
                        "ENABLE_IDE_INTEGRATION".to_string(),
                        "true".to_string(),
                    );
                    if let Some(socket_path) = &socket_path {
                        project.set_terminal_env_var(
                            selection_socket::SELECTION_SOCK_ENV.to_string(),
                            socket_path.to_string_lossy().to_string(),
                        );
                    }
                });
            });

            if let Some(bridge) = cx.update(|cx| bridge_for(workspace_id, cx)) {
                let _ = bridge.update(cx, |bridge, _| {
                    bridge.server = Some(server);
                    bridge.lock_path = lock_path;
                    bridge.selection_socket_path = socket_path;
                    bridge.selection_socket = socket_task;
                });
            }
        }
    });

    let workspace_entity = cx.entity();
    let workspace_subscription = cx.subscribe(
        &workspace_entity,
        move |workspace, _emitter, event, cx| {
            if matches!(event, workspace::Event::ActiveItemChanged) {
                let workspace_id = cx.entity_id();
                let workspace_entity = cx.entity();
                // Defer: this fires while the Workspace is mid-update, so reading
                // it now (on_active_item_changed reads active_item) would double-
                // lease and panic. Run once the current update unwinds.
                cx.defer(move |cx| {
                    if let Some(bridge) = bridge_for(workspace_id, cx) {
                        bridge.update(cx, |bridge, cx| {
                            bridge.on_active_item_changed(&workspace_entity, cx);
                        });
                    }
                });
                let _ = workspace;
            }
        },
    );

    let workspace_handle = cx.weak_entity();
    let bridge = cx.new(|_cx| ClaudeCodeIde {
        selection,
        server: None,
        lock_path: None,
        selection_socket_path: None,
        selection_socket: None,
        editor_subscription: None,
        pending_push: None,
        workspace: workspace_handle,
        _startup: startup,
        _workspace_subscription: workspace_subscription,
    });

    register_bridge(workspace_id, &bridge, cx);

    // Unlink the lockfile and the selection socket when the bridge is released.
    cx.observe_release(&bridge, |_workspace, bridge, _cx| {
        if let Some(path) = bridge.lock_path.take() {
            lockfile::unlink_lock(&path);
        }
        // Drop the accept loop before unlinking so nothing is still listening on
        // a path that no longer exists.
        bridge.selection_socket.take();
        if let Some(path) = bridge.selection_socket_path.take() {
            selection_socket::unlink_socket(&path);
        }
    })
    .detach();

    // Keep the bridge alive for the workspace's life; dropping this strong
    // handle on workspace release fires the release observer above.
    cx.on_release(move |_workspace, _cx| drop(bridge)).detach();

    log::info!("claude_code_ide: started for workspace");
}

impl ClaudeCodeIde {
    /// Re-subscribe to the newly-active editor and push its selection.
    fn on_active_item_changed(&mut self, workspace: &Entity<Workspace>, cx: &mut Context<Self>) {
        let editor = workspace.read(cx).active_item_as::<Editor>(cx);
        let Some(editor) = editor else {
            self.editor_subscription = None;
            return;
        };

        self.editor_subscription =
            Some(cx.subscribe(&editor, |bridge, _editor, event, cx| {
                if let EditorEvent::SelectionsChanged { .. } = event {
                    bridge.schedule_selection_push(cx);
                }
            }));

        // Push the current selection for the freshly-focused editor immediately.
        self.push_selection_now(cx);
    }

    /// Debounce a selection push so a burst of changes coalesces into one
    /// notification.
    fn schedule_selection_push(&mut self, cx: &mut Context<Self>) {
        self.pending_push = Some(cx.spawn(async move |bridge, cx| {
            cx.background_executor().timer(SELECTION_DEBOUNCE).await;
            let _ = bridge.update(cx, |bridge, cx| {
                bridge.push_selection_now(cx);
            });
        }));
    }

    /// Compute the current selection, cache it, and push `selection_changed`.
    fn push_selection_now(&mut self, cx: &mut App) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let payload = selection::capture(&self.selection, &workspace, cx);
        if let (Some(server), Some(payload)) = (self.server.as_ref(), payload) {
            server.notify(SELECTION_CHANGED, payload);
        }
    }

    /// Broadcast what is currently selected across every source.
    ///
    /// When nothing is left the aggregate is `None`, and sending nothing would leave
    /// the client showing the selection just removed, so an explicitly empty payload
    /// goes out instead: `selection: null` with empty text says "nothing selected".
    fn broadcast_selection(&self) {
        let Some(server) = self.server.as_ref() else {
            return;
        };
        match self.selection.latest() {
            Some(aggregate) => server.notify(SELECTION_CHANGED, aggregate),
            None => server.notify(SELECTION_CHANGED, crate::protocol::cleared_selection()),
        }
    }

    /// Store one source's selection, supplied by the caller rather than read from the
    /// active editor, replacing only that source's region.
    fn push_external_selection(
        &mut self,
        source: String,
        payload: crate::protocol::SelectionPayload,
        owner_pid: Option<u32>,
    ) {
        self.selection.upsert_external(source, payload, owner_pid);
        self.broadcast_selection();
    }

    fn clear_external_selection(&mut self, source: &str) {
        if self.selection.remove_external(source) {
            self.broadcast_selection();
        }
    }

    fn clear_all_external_selections(&mut self) {
        let removed = self.selection.remove_all_external();
        if removed > 0 {
            log::info!("claude_code_ide: cleared {removed} external selection region(s)");
            self.broadcast_selection();
        }
    }

    /// Push `at_mentioned` for the current selection (explicit user action).
    fn send_at_mention(&mut self, cx: &mut App) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let Some(payload) = selection::capture(&self.selection, &workspace, cx) else {
            log::info!("claude_code_ide: no selection to send");
            return;
        };
        if let Some(server) = self.server.as_ref() {
            server.notify(
                AT_MENTIONED,
                serde_json::json!({
                    "filePath": payload.file_path,
                    "lineStart": payload.selection.start.line,
                    "lineEnd": payload.selection.end.line,
                }),
            );
        }
    }
}

/// Boxed action, so callers outside this crate can dispatch it.
pub fn send_selection_action() -> Box<dyn Action> {
    SendSelectionToClaudeCode.boxed_clone()
}
