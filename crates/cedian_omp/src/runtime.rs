//! `OmpRuntime`: owned handle to one bundled OMP sidecar process.
//!
//! Plan §5 target shape (adapted — blocking client behind a thread bridge):
//!
//! ```rust,ignore
//! struct OmpRuntime {
//!     client: omp_rpc::Client, // blocking; lives on a dedicated I/O thread
//!     child: Child,
//!     state: OmpRuntimeState,
//! }
//! ```
//!
//! Phase 1 is headless by design: spawn → prompt/abort → open/new session →
//! set_model → images → ask-dialog opt-in → host tools/URIs. Every method is
//! blocking and MUST be called off the UI thread; async/GPUI bridging lands
//! with the panel in Phase 2.

use crate::{
    EventRouter, OmpError, SessionBinding, Sessions, SpawnPolicy, SpawnProfile, resolve_on_path,
};
use omp_rpc::wire::{
    CancelSubagentCommand, FollowUpCommand, SetSubagentSubscriptionCommand, SteerSubagentCommand,
    SubagentSubscriptionLevel,
};
use omp_rpc::{
    AbortAndRestoreQueueCommand, AbortCommand, Client, ClientOptions, Event, ExtensionUiResponse,
    GetStateCommand, HostTool, HostUri, ImageContent, NewSessionCommand, OpenSessionCommand,
    OpenSessionResult, PromptCommand, PromptTurn, RpcInbound, RpcNotification, SessionState,
    SetAskDialogCommand, SetModelCommand, SteerCommand, SwitchSessionCommand,
};
use std::{
    path::PathBuf,
    process::Command as Process,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::JoinHandle,
    time::Duration,
};

/// How to find the `omp` binary. Bundled path in product; PATH fallback in dev.
#[derive(Debug, Clone)]
pub enum OmpBinary {
    /// Exact bundled path (`cedian.app/Contents/Resources/omp`).
    Bundled(PathBuf),
    /// Resolve via `PATH` (dev only — never in product).
    Path(String),
}

/// One-shot and unattended turns (CLI `prompt`, reviewers). A wall clock
/// bounds a hung OMP. The interactive app uses [`APP_PROMPT_TIMEOUT`]:
/// B6 was cut mid-task at 600 s under this budget.
pub const HEADLESS_PROMPT_TIMEOUT: Duration = Duration::from_secs(600);

/// Interactive app turns. The person can Stop; a short wall clock would
/// cut a live mid-task turn (ROADMAP follow-up). One day is far above any
/// expected turn and stays inside `Instant` addition.
pub const APP_PROMPT_TIMEOUT: Duration = Duration::from_secs(24 * 60 * 60);

/// Static config for one runtime.
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// OMP binary location.
    pub binary: OmpBinary,
    /// cedian's directory for this runtime (the overlay; with
    /// [`Sessions::InSessionDir`] also `--session-dir`).
    pub session_dir: PathBuf,
    pub sessions: Sessions,
    /// Workspace root (cwd for the sidecar).
    pub cwd: PathBuf,
    /// Opt in to the `ask` tool dialog (off by default upstream — without it,
    /// approval-needing tools fail closed; plan §10 + spike row).
    pub ask_dialog: bool,
    /// Prompt round-trip deadline ([`APP_PROMPT_TIMEOUT`] in the app,
    /// [`HEADLESS_PROMPT_TIMEOUT`] for CLI and reviewers).
    pub prompt_timeout: Duration,
    /// Approval mode + overlay policy (ADR-0020). Every spawn goes through it.
    pub policy: SpawnPolicy,
}

/// Lifecycle of the sidecar process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeState {
    /// Spawned, handshake done, ready for prompts.
    Ready,
    /// A turn is streaming.
    Streaming,
    /// OMP disconnected / stdout closed — user-visible, restart required.
    Disconnected,
}

/// Owned OMP sidecar: blocking client + pump thread + shared router.
///
/// The router is shared (`Arc`): the pump thread dispatches notification frames
/// into it, panel/task subscribers (Phase 2) read from it. Blocking
/// `prompt_and_wait` runs inline on the caller thread — callers MUST be off the
/// UI thread.
pub struct OmpRuntime {
    client: Option<Arc<Client>>,
    router: Arc<EventRouter>,
    state: RuntimeState,
    session: Option<SessionBinding>,
    stop: Arc<AtomicBool>,
    pump: Option<JoinHandle<()>>,
    config: RuntimeConfig,
    headless: Arc<HeadlessUi>,
}

/// `deny_ui_requests` state shared with the pump thread.
#[derive(Default)]
struct HeadlessUi {
    enabled: AtomicBool,
    /// Labels of the dialogs answered fail-closed, oldest first.
    refused: parking_lot::Mutex<Vec<crate::DialogRecord>>,
}

impl OmpRuntime {
    /// Spawn the sidecar through the spawn profile (argv + overlay + scrubbed
    /// env, ADR-0020), wait `ready`, negotiate v2 (inside vendored client),
    /// apply `ask_dialog`, start the pump thread. Crash recovery respawns with
    /// the same config. A profile error means no process is started.
    pub fn spawn(config: RuntimeConfig) -> Result<Self, OmpError> {
        let binary_path = match &config.binary {
            OmpBinary::Bundled(path) => path.clone(),
            OmpBinary::Path(name) => resolve_on_path(name, std::env::var("PATH").ok().as_deref())?,
        };
        let plan = SpawnProfile {
            binary_path,
            session_dir: config.session_dir.clone(),
            sessions: config.sessions,
            cwd: config.cwd.clone(),
            policy: config.policy.clone(),
        }
        .prepare()?;
        let mut process = Process::new(&plan.argv[0]);
        process.args(&plan.argv[1..]).env_clear().envs(plan.env);
        let options = ClientOptions {
            default_timeout: Duration::from_secs(30),
            ..Default::default()
        };
        let (client, events) = Client::spawn(process, options).map_err(|e| match e {
            omp_rpc::client::Error::Io(io) => OmpError::Spawn(io.to_string()),
            other => OmpError::Handshake(other.to_string()),
        })?;
        let client = Arc::new(client);
        let router = Arc::new(EventRouter::new());
        let stop = Arc::new(AtomicBool::new(false));

        // Pump: notification frames → router classification. Correlated
        // responses are consumed inside the vendored client, never here.
        let pump_router = Arc::clone(&router);
        let pump_stop = Arc::clone(&stop);
        let headless = Arc::new(HeadlessUi::default());
        let pump_headless = Arc::clone(&headless);
        // Weak: `shutdown` needs the only strong ref to take the client.
        let pump_client = Arc::downgrade(&client);
        let pump = std::thread::spawn(move || {
            for event in events {
                if pump_stop.load(Ordering::Relaxed) {
                    break;
                }
                if let Event::Notification(frame) = event {
                    if let RpcNotification::ExtensionUiRequest(request) = &frame {
                        if pump_headless.enabled.load(Ordering::Relaxed) {
                            if let (Some((reply, record)), Some(client)) =
                                (crate::headless_answer(request), pump_client.upgrade())
                            {
                                let _ = client.send(&RpcInbound::ExtensionUiResponse(reply));
                                pump_headless.refused.lock().push(record);
                            }
                        }
                    }
                    pump_router.dispatch_notification(&frame);
                }
            }
            // The channel closes when OMP's stdout does. Unless `shutdown`
            // asked for it, OMP died: say so (U3 crash isolation).
            if !pump_stop.load(Ordering::Relaxed) {
                pump_router.dispatch_disconnected();
            }
        });

        let runtime = Self {
            client: Some(client),
            router,
            state: RuntimeState::Ready,
            session: None,
            stop,
            pump: Some(pump),
            config,
            headless,
        };
        if runtime.config.ask_dialog {
            runtime
                .client()
                .call(&SetAskDialogCommand { enabled: true })
                .map_err(OmpError::from)?;
        }
        Ok(runtime)
    }

    /// No UI will ever answer: from now on every OMP dialog that expects a
    /// reply is answered fail-closed by the pump (approvals denied, confirms
    /// declined, the rest dismissed) instead of stalling the turn until
    /// `prompt_timeout`. Headless callers only — the app answers with real UI.
    pub fn deny_ui_requests(&self) {
        self.headless.enabled.store(true, Ordering::Relaxed);
    }

    /// Drain the dialogs answered by [`Self::deny_ui_requests`].
    pub fn take_refused_ui_requests(&self) -> Vec<crate::DialogRecord> {
        std::mem::take(&mut *self.headless.refused.lock())
    }

    /// A cheap, cloneable handle for `steer`/`abort` from another thread while
    /// this runtime is blocked inside [`Self::prompt`] (`cedian shell`, P4).
    pub fn control(&self) -> RuntimeControl {
        RuntimeControl {
            client: Arc::clone(self.client()),
        }
    }

    /// Borrow the client (shutdown takes it; all ops require it present).
    fn client(&self) -> &Arc<Client> {
        self.client.as_ref().expect("client present until shutdown")
    }

    /// OMP's process id, while it runs.
    pub fn pid(&self) -> Option<u32> {
        self.client().pid()
    }

    /// Current lifecycle state.
    pub fn state(&self) -> RuntimeState {
        self.state
    }

    /// Shared router for panel/task subscribers (Phase 2).
    /// The profile this child was spawned under (ADR-0035 badge and audit).
    pub fn approvals(&self) -> crate::Approvals {
        self.config.policy.approvals
    }

    pub fn router(&self) -> Arc<EventRouter> {
        Arc::clone(&self.router)
    }

    /// Full router log for crash reconciliation (§74 R3).
    pub fn event_log(&self) -> Vec<crate::LogEntry> {
        self.router.log_snapshot()
    }

    /// Send a prompt; block until its `prompt_result` (or timeout). Images are
    /// base64 payloads (`ImageContent`), same shape the spike proved.
    pub fn prompt(
        &mut self,
        message: &str,
        images: Vec<ImageContent>,
    ) -> Result<PromptTurn, OmpError> {
        self.state = RuntimeState::Streaming;
        let cmd = PromptCommand {
            message: message.to_string(),
            images: if images.is_empty() {
                None
            } else {
                Some(images)
            },
            streaming_behavior: None,
        };
        let turn = self
            .client()
            .prompt_and_wait(&cmd, self.config.prompt_timeout)
            .map_err(|e| OmpError::from(e).with_timeout(self.config.prompt_timeout))?;
        self.state = RuntimeState::Ready;
        Ok(turn)
    }

    /// Abort the running turn.
    pub fn abort(&mut self) -> Result<(), OmpError> {
        self.client()
            .call(&AbortCommand {})
            .map_err(OmpError::from)?;
        self.state = RuntimeState::Ready;
        Ok(())
    }

    /// Adopt (or start) the newest session in `session_dir`; records the
    /// workspace↔session binding. Call before first prompt when restoring
    /// (plan §75: directory-adopt, not file-restore).
    pub fn open_session(&mut self, task: &str) -> Result<OpenSessionResult, OmpError> {
        let session_dir = self.sessions_dir()?;
        let result: OpenSessionResult = self
            .client()
            .call(&OpenSessionCommand {
                session_dir: session_dir.to_string_lossy().into_owned(),
                provider: None,
                model_id: None,
            })
            .map_err(OmpError::from)?;
        self.session = Some(SessionBinding::new(
            self.config.cwd.clone(),
            session_dir,
            result.session_id.clone(),
            task.to_string(),
        ));
        Ok(result)
    }

    /// The directory OMP keeps this child's sessions in. Under
    /// [`Sessions::OmpDefault`] OMP chose it: the folder of its current
    /// session file, as `get_state` reports before any prompt.
    fn sessions_dir(&self) -> Result<PathBuf, OmpError> {
        match self.config.sessions {
            Sessions::InSessionDir => Ok(self.config.session_dir.clone()),
            Sessions::OmpDefault => self
                .get_state()?
                .session_file
                .as_deref()
                .and_then(|file| std::path::Path::new(file).parent())
                .map(PathBuf::from)
                .ok_or_else(|| OmpError::Spawn("OMP reported no session file".to_string())),
        }
    }

    /// Start a fresh session, optionally under a parent.
    pub fn new_session(
        &mut self,
        parent: Option<String>,
        task: &str,
    ) -> Result<NewSession, OmpError> {
        let result = self
            .client()
            .call(&NewSessionCommand {
                parent_session: parent,
            })
            .map_err(OmpError::from)?;
        if result.cancelled {
            return Ok(NewSession::Declined);
        }
        let state: SessionState = match self.client().call(&GetStateCommand {}) {
            Ok(state) => state,
            Err(e) => {
                self.session = None;
                return Ok(NewSession::Unknown(OmpError::from(e)));
            }
        };
        self.session = Some(SessionBinding::new(
            self.config.cwd.clone(),
            self.config.session_dir.clone(),
            state.session_id.clone(),
            task.to_string(),
        ));
        Ok(NewSession::Started(state))
    }

    /// Move OMP to the session in `file` and rebind to it. An error means
    /// OMP refused and kept the current session.
    pub fn switch_session(
        &mut self,
        file: &std::path::Path,
        task: &str,
    ) -> Result<NewSession, OmpError> {
        let result = self
            .client()
            .call(&SwitchSessionCommand {
                session_path: file.to_string_lossy().into_owned(),
                provider: None,
                model_id: None,
            })
            .map_err(OmpError::from)?;
        if result.cancelled {
            return Ok(NewSession::Declined);
        }
        let state = match self.get_state() {
            Ok(state) => state,
            Err(e) => {
                self.session = None;
                return Ok(NewSession::Unknown(e));
            }
        };
        self.session = Some(SessionBinding::new(
            self.config.cwd.clone(),
            file.parent().map_or_else(PathBuf::new, PathBuf::from),
            state.session_id.clone(),
            task.to_string(),
        ));
        Ok(NewSession::Started(state))
    }

    /// Switch the session model (`provider/id` pair, both required upstream).
    pub fn set_model(&self, provider: &str, model_id: &str) -> Result<(), OmpError> {
        self.client()
            .call(&SetModelCommand {
                provider: provider.to_string(),
                model_id: model_id.to_string(),
            })
            .map_err(OmpError::from)?;
        Ok(())
    }

    /// Current session snapshot (model, streaming, queue, todos, usage).
    pub fn get_state(&self) -> Result<SessionState, OmpError> {
        self.client()
            .call(&GetStateCommand {})
            .map_err(OmpError::from)
    }

    /// Replace the host-owned tool set (whole-set replace semantics upstream).
    pub fn set_host_tools(&self, tools: Vec<HostTool>) -> Result<Vec<String>, OmpError> {
        self.client()
            .set_custom_tools(tools)
            .map_err(OmpError::from)
    }

    /// Replace the host-owned URI scheme set.
    pub fn set_host_uris(&self, uris: Vec<HostUri>) -> Result<Vec<String>, OmpError> {
        self.client().set_host_uris(uris).map_err(OmpError::from)
    }

    /// Have OMP send `subagent_lifecycle` and `subagent_progress` (ADR-0050:
    /// level `progress`, never every subagent token).
    pub fn subscribe_subagents(&self) -> Result<(), OmpError> {
        self.client()
            .call(&SetSubagentSubscriptionCommand {
                level: SubagentSubscriptionLevel::Progress,
            })
            .map(|_| ())
            .map_err(OmpError::from)
    }

    /// Active workspace↔session binding, if adopted.
    pub fn session(&self) -> Option<&SessionBinding> {
        self.session.as_ref()
    }

    /// Shut down: close the client first (stdin EOF + SIGTERM→1s→SIGKILL
    /// group teardown, which closes the event channel), then join the pump.
    /// Order matters — joining the pump first deadlocks: the channel only
    /// closes after the client does.
    pub fn shutdown(mut self) -> Result<(), OmpError> {
        self.stop.store(true, Ordering::Relaxed);
        let client = self.client.take().expect("shutdown consumes the client");
        let client = take_unique(client, CONTROL_RELEASE)
            .map_err(|_| OmpError::Transport("a control call was still in flight".to_string()))?;
        let result = client.close().map_err(OmpError::from);
        if let Some(pump) = self.pump.take() {
            let _ = pump.join();
        }
        result
    }
}

/// How long shutdown waits for a [`RuntimeControl`] clone to drop: an abort
/// thread may hold one for a moment after OMP has already answered.
const CONTROL_RELEASE: Duration = Duration::from_secs(2);

fn take_unique<T>(arc: Arc<T>, limit: Duration) -> Result<T, Arc<T>> {
    let deadline = std::time::Instant::now() + limit;
    while Arc::strong_count(&arc) > 1 && std::time::Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(10));
    }
    Arc::try_unwrap(arc)
}

/// What [`OmpRuntime::new_session`] or [`OmpRuntime::switch_session`] did.
#[derive(Debug)]
pub enum NewSession {
    Started(SessionState),
    /// OMP kept the current session.
    Declined,
    /// OMP switched, but which session it is now on could not be read.
    Unknown(OmpError),
}

/// Mid-turn control over a live session; see [`OmpRuntime::control`].
#[derive(Clone)]
pub struct RuntimeControl {
    client: Arc<Client>,
}

impl RuntimeControl {
    /// Inject a steering message into the running turn.
    pub fn steer(&self, message: &str) -> Result<(), OmpError> {
        self.client
            .call(&SteerCommand {
                message: message.to_string(),
                images: None,
            })
            .map_err(OmpError::from)
    }

    /// Queue a message to run after the current turn.
    pub fn follow_up(&self, message: &str) -> Result<(), OmpError> {
        self.client
            .call(&FollowUpCommand {
                message: message.to_string(),
                images: None,
            })
            .map_err(OmpError::from)
    }

    /// Withdraw OMP's queued user input and abort the run in one command
    /// (ADR-0059 decision 2), so a queued steer cannot start a fresh run after
    /// Stop. The queue comes back oldest first, steering then follow-ups.
    pub fn abort_and_restore_queue(&self) -> Result<omp_rpc::AbortAndRestoreQueueResult, OmpError> {
        self.client
            .call(&AbortAndRestoreQueueCommand {})
            .map_err(OmpError::from)
    }

    /// Steer subagent `id` mid-turn. OMP refuses an id that is not running.
    pub fn steer_subagent(&self, id: &str, message: &str) -> Result<(), OmpError> {
        self.client
            .call(&SteerSubagentCommand {
                subagent_id: id.to_string(),
                message: message.to_string(),
            })
            .map_err(OmpError::from)
    }

    /// Cancel subagent `id`; `false` when it had already ended.
    pub fn cancel_subagent(&self, id: &str) -> Result<bool, OmpError> {
        self.client
            .call(&CancelSubagentCommand {
                subagent_id: id.to_string(),
            })
            .map_err(OmpError::from)
    }

    /// Answer one of OMP's dialogs (`crate::user_answer`).
    pub fn respond(&self, reply: ExtensionUiResponse) -> Result<(), OmpError> {
        self.client
            .send(&RpcInbound::ExtensionUiResponse(reply))
            .map_err(OmpError::from)
    }

    /// Any one RPC command, for the panel's settings of the live session
    /// (model, thinking level, queue modes, compaction, retry).
    pub fn call<C: omp_rpc::Command>(&self, command: &C) -> Result<C::Output, OmpError> {
        self.client.call(command).map_err(OmpError::from)
    }

    /// Abort the running turn; the blocked `prompt` returns.
    pub fn abort(&self) -> Result<(), OmpError> {
        self.client
            .call(&AbortCommand {})
            .map(|_| ())
            .map_err(OmpError::from)
    }
}

/// Build an image payload from raw bytes (base64-encoded on the wire).
pub fn image_content(mime_type: &str, bytes: &[u8]) -> ImageContent {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    ImageContent {
        r#type: None,
        data: Some(STANDARD.encode(bytes)),
        mime_type: Some(mime_type.to_string()),
        detail: None,
        url: None,
        provider_file: Default::default(),
        extra: Default::default(),
    }
}

/// Last assistant text of a prompt turn (thinking excluded by the client).
pub fn last_text(turn: &PromptTurn) -> Option<&str> {
    turn.assistant_text.as_deref()
}

/// Suppress unused-import lint while `RpcNotification` shapes the Phase 2 API.
#[allow(dead_code)]
fn _notification_shape(_: &RpcNotification) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_unique_waits_for_a_clone_dropped_after_shutdown_starts() {
        let shared = Arc::new(7);
        let clone = Arc::clone(&shared);
        let holder = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(300));
            drop(clone);
        });
        assert_eq!(take_unique(shared, CONTROL_RELEASE).ok(), Some(7));
        holder.join().unwrap();
    }

    #[test]
    fn take_unique_gives_up_on_a_clone_that_stays() {
        let shared = Arc::new(7);
        let _clone = Arc::clone(&shared);
        assert!(take_unique(shared, Duration::from_millis(50)).is_err());
    }
}
