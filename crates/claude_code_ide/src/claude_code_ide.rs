//! IDE-side implementation of the Claude Code editor integration protocol.
//!
//! This lets the Claude Code CLI connect to Zed the same way it connects to the
//! official VS Code and JetBrains extensions: Zed runs a localhost WebSocket
//! server speaking a WebSocket variant of MCP, advertises it through a lock file
//! under `~/.claude/ide/`, and points the CLI at it via the `CLAUDE_CODE_SSE_PORT`
//! and `ENABLE_IDE_INTEGRATION` environment variables in its integrated terminal.
//!
//! [`init`] wires one [`ClaudeCodeIdeServer`] per [`Workspace`]: it binds a
//! loopback port, writes the lock file, and serves connections until the window
//! closes, at which point the lock file is removed.

mod lockfile;
mod open_diff;
mod server;
mod tools;

use std::{cell::Cell, cell::RefCell, rc::Rc};

use anyhow::{Context as _, Result};
use collections::HashMap;
use gpui::{
    AnyWindowHandle, App, AppContext as _, AsyncApp, Context, Entity, EntityId, Task, WeakEntity,
};
use util::ResultExt as _;
use workspace::Workspace;

pub use lockfile::{IDE_NAME, generate_auth_token};
pub use server::{Dispatcher, ProtocolError, ToolDescriptor, bind, serve_connection};
pub use tools::WorkspaceDispatcher;

/// Registers a Claude Code IDE server for every workspace window.
///
/// Call once during app startup. Each created [`Workspace`] gets its own server
/// entity, kept alive in `servers` for the window's lifetime and dropped (which
/// removes its lock file) when the workspace is released.
pub fn init(cx: &mut App) {
    let servers: Rc<RefCell<HashMap<EntityId, Entity<ClaudeCodeIdeServer>>>> = Rc::default();
    cx.observe_new({
        let servers = servers.clone();
        move |_workspace: &mut Workspace, window, cx: &mut Context<Workspace>| {
            let workspace_id = cx.entity_id();
            let workspace_handle = cx.entity().downgrade();
            let window_handle = window.map(|window| window.window_handle());
            let server =
                cx.new(|cx| ClaudeCodeIdeServer::new(workspace_handle, window_handle, cx));
            servers.borrow_mut().insert(workspace_id, server);

            cx.on_release({
                let servers = servers.clone();
                move |_workspace, _cx| {
                    servers.borrow_mut().remove(&workspace_id);
                }
            })
            .detach();
        }
    })
    .detach();

    // Lock files are removed when a window closes (see `Drop`), but a hard quit
    // skips destructors, so clean them up explicitly on app exit too.
    cx.on_app_quit({
        let servers = servers.clone();
        move |cx| {
            for server in servers.borrow().values() {
                server.update(cx, |server, _| server.remove_lockfile());
            }
            async move {}
        }
    })
    .detach();
}

/// One running WebSocket server, bound to a single workspace window.
struct ClaudeCodeIdeServer {
    /// Set once the listener is bound; read by `Drop` to remove the lock file.
    /// Shared with the accept-loop task, which is the writer.
    port: Rc<Cell<Option<u16>>>,
    /// The bind + accept loop. Dropping it (when the workspace closes) cancels
    /// the loop, stopping the server.
    _server_task: Task<()>,
}

impl ClaudeCodeIdeServer {
    fn new(
        workspace: WeakEntity<Workspace>,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Self>,
    ) -> Self {
        let port = Rc::new(Cell::new(None));
        let server_task = cx.spawn({
            let port = port.clone();
            async move |_this, cx| {
                if let Err(error) = Self::run(workspace, window, port, cx).await {
                    log::error!("Claude Code IDE server stopped: {error:#}");
                }
            }
        });
        Self { port, _server_task: server_task }
    }

    async fn run(
        workspace: WeakEntity<Workspace>,
        window: Option<AnyWindowHandle>,
        port_cell: Rc<Cell<Option<u16>>>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let (listener, port) = bind().await?;
        let auth_token = generate_auth_token();

        let workspace_folders = workspace
            .update(cx, |workspace, cx| {
                workspace
                    .project()
                    .read(cx)
                    .visible_worktrees(cx)
                    .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
                    .collect::<Vec<_>>()
            })
            .context("reading workspace folders")?;

        lockfile::create(port, &auth_token, &workspace_folders)?;
        port_cell.set(Some(port));

        // Publish the port to the project so newly opened terminals advertise it
        // to the Claude CLI via `CLAUDE_CODE_SSE_PORT`.
        workspace
            .update(cx, |workspace, cx| {
                workspace
                    .project()
                    .update(cx, |project, _| project.set_claude_code_ide_port(Some(port)));
            })
            .context("publishing IDE port to project")?;

        log::info!("Claude Code IDE server listening on 127.0.0.1:{port}");

        // Each accepted connection is served on the foreground executor so its
        // tool handlers can touch workspace entities; the async I/O still yields,
        // so it never blocks the UI.
        while let Ok((stream, _addr)) = listener.accept().await {
            let dispatcher = WorkspaceDispatcher::new(workspace.clone(), window, cx.clone());
            let auth_token = auth_token.clone();
            cx.spawn(async move |_cx| {
                serve_connection(stream, auth_token, dispatcher).await.log_err();
            })
            .detach();
        }

        Ok(())
    }

    /// Removes this server's lock file, if one has been written. Idempotent.
    fn remove_lockfile(&self) {
        if let Some(port) = self.port.get() {
            lockfile::remove(port).log_err();
        }
    }
}

impl Drop for ClaudeCodeIdeServer {
    fn drop(&mut self) {
        self.remove_lockfile();
    }
}
