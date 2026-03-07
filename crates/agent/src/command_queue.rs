use agent_settings::AgentSettings;
use anyhow::Result;
use gpui::{App, AsyncApp, Global, Task};
use settings::Settings;
use smol::channel;
use std::path::PathBuf;
use std::rc::Rc;

use crate::{TerminalHandle, ThreadEnvironment};

struct GlobalCommandQueue(CommandQueue);

impl Global for GlobalCommandQueue {}

/// A single-worker serialization gate for agent terminal commands.
///
/// Commands are executed one at a time: the next command does not begin until
/// the previous caller drops its [`CommandGuard`]. This replaces
/// fire-and-forget process spawning with a disciplined, sequential model.
///
/// The queue is stored as a GPUI `Global` so it survives terminal UI
/// closure/recycling and lives for the lifetime of the application.
///
/// Implementation: a bounded channel of capacity 1 is pre-filled with a single
/// permit. [`CommandQueue::acquire`] receives the permit (blocking if another
/// caller already holds it) and returns a [`CommandGuard`]. Dropping the guard
/// sends the permit back, unblocking the next waiter.
#[derive(Clone)]
pub struct CommandQueue {
    permit_tx: channel::Sender<()>,
    permit_rx: channel::Receiver<()>,
}

/// RAII guard returned by [`CommandQueue::acquire`].
///
/// While this guard is alive the serialization permit is held, preventing the
/// next queued command from starting. Drop (or explicitly call
/// [`CommandGuard::release`]) when the current command is finished.
pub struct CommandGuard {
    permit_tx: Option<channel::Sender<()>>,
}

impl CommandGuard {
    /// Explicitly release the serialization permit.
    pub fn release(self) {
        drop(self);
    }
}

impl Drop for CommandGuard {
    fn drop(&mut self) {
        if let Some(tx) = self.permit_tx.take() {
            tx.try_send(()).ok();
        }
    }
}

impl CommandQueue {
    /// Initialize the global `CommandQueue`. Must be called once during app
    /// startup. Subsequent calls are no-ops if the global is already set.
    pub fn init(cx: &mut App) {
        if cx.has_global::<GlobalCommandQueue>() {
            return;
        }
        let (permit_tx, permit_rx) = channel::bounded(1);
        // Seed with one permit so the first `acquire` succeeds immediately.
        permit_tx.try_send(()).ok();
        let queue = Self {
            permit_tx,
            permit_rx,
        };
        cx.set_global(GlobalCommandQueue(queue));
    }

    /// Returns a clone of the global `CommandQueue`.
    pub fn global(cx: &App) -> CommandQueue {
        cx.global::<GlobalCommandQueue>().0.clone()
    }

    /// Try to get the global `CommandQueue`, returning `None` if it hasn't
    /// been initialized yet.
    pub fn try_global(cx: &App) -> Option<CommandQueue> {
        cx.try_global::<GlobalCommandQueue>().map(|g| g.0.clone())
    }

    /// Wait for the serialization permit and return a guard.
    ///
    /// The returned [`CommandGuard`] holds the permit. While it is alive no
    /// other call to `acquire` will complete. Drop the guard when the
    /// command's child process has exited (or timed out) to let the next
    /// queued command proceed.
    pub async fn acquire(&self) -> CommandGuard {
        // The channel is FIFO, so callers are served in order.
        self.permit_rx.recv().await.ok();
        CommandGuard {
            permit_tx: Some(self.permit_tx.clone()),
        }
    }

    /// Convenience: acquire the gate, create a terminal, and return both the
    /// terminal handle and the guard.
    ///
    /// The caller is responsible for waiting on the child process and dropping
    /// the guard afterwards (this is intentional — the caller may need to
    /// handle user-cancellation, custom timeouts, etc.).
    pub fn create_terminal_serialized(
        &self,
        environment: &Rc<dyn ThreadEnvironment>,
        command: String,
        cwd: Option<PathBuf>,
        output_byte_limit: Option<u64>,
        cx: &mut AsyncApp,
    ) -> Task<Result<(Rc<dyn TerminalHandle>, CommandGuard)>> {
        let queue = self.clone();
        let environment = environment.clone();

        cx.spawn(async move |cx| {
            let guard = queue.acquire().await;

            let terminal = environment
                .create_terminal(command, cwd, output_byte_limit, cx)
                .await?;

            Ok((terminal, guard))
        })
    }
}

/// Read the `agent.command_timeout` setting.
pub fn configured_command_timeout(cx: &App) -> agent_settings::CommandTimeout {
    let settings = AgentSettings::get_global(cx);
    settings.command_timeout
}
