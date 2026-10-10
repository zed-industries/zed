//! The app's line to OMP (S9 U3, U4): one OMP process per workspace, started
//! the way the CLI starts it (`cedian_shell::launch`, the user's
//! `cedian.toml`, the spawn profile) on its own thread, so a dying OMP never
//! takes the IDE with it. Sessions live where OMP's CLI keeps them for the
//! project, so a session started in either opens in the other (ADR-0040
//! decision 5) and a restart's `open_session` adopts it; cedian's overlay
//! and audit log live in the workspace's state dir (ADR-0044).
//!
//! The link is cedian's gate for the app (§64): every tool execution and
//! every dialog outcome is a row in `audit.jsonl`. A person's answer is sent
//! and recorded in one step; a dialog still open when OMP dies, withdraws it
//! or is stopped is recorded as `abstain` (§63 lease, ADR-0013).

use cedian_omp::{
    DialogRecord, EventRouter, NewSession, OmpBinary, OmpError, OmpRuntime, RouterEvent,
    RuntimeConfig, RuntimeControl, Sessions, SpawnPolicy, UserAnswer,
};
use cedian_shell::audit::AuditLog;
use cedian_shell::{Policy, RunKind};
use collections::HashMap;
use futures::channel::mpsc::UnboundedSender;
use omp_rpc::{
    AbortRetryCommand, CompactCommand, CycleModelCommand, CycleThinkingLevelCommand,
    ExtensionUiRequest, ExtensionUiResponse, GetAvailableModelsCommand,
    GetAvailableThinkingLevelsCommand, GetLoginProvidersCommand, GetStateCommand, HostTool,
    HostUri, ImageContent, InterruptMode, LoginCommand, LoginProvider, ModelInfo,
    PromoteQueuedMessageCommand, QueueMode, RpcAgentEvent, SetAutoCompactionCommand,
    SetAutoRetryCommand, SetFollowUpModeCommand, SetInterruptModeCommand, SetModelCommand,
    SetSteeringModeCommand, SetThinkingLevelCommand, ThinkingLevel,
};
use parking_lot::Mutex;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, OnceLock, mpsc};
use std::time::{Duration, Instant};

/// What the OMP thread tells the panel.
#[derive(Debug, Clone)]
pub enum LinkEvent {
    /// OMP is up and its session is open.
    Ready {
        session_id: String,
        /// The session's file in OMP's store, where the CLI finds it too.
        session_file: Option<String>,
        resumed: bool,
        /// Set under `policy = "omp"` (ADR-0035): OMP's config decides.
        policy_note: Option<String>,
        /// The session's model (`provider/id`) and thinking level, read once
        /// it opened, for the closed picker.
        model: Option<String>,
        thinking: Option<omp_rpc::ThinkingLevel>,
    },
    /// The OMP the link runs is not the pinned one (ADR-0057 decision 4).
    OmpWarning(String),
    /// The `policy = "omp"` badge, with what OMP's config says (ADR-0035
    /// decision 4).
    OmpPolicy(String),
    /// One of OMP's events, `Disconnected` included.
    Event(RouterEvent),
    /// OMP could not start, or its session could not open.
    Failed(String),
    /// An audit row could not be written (ADR-0035: an unaudited turn fails).
    AuditFailed(String),
    /// Another process drives the session, or whether one does could not be
    /// checked: the app sends it no prompt (ADR-0040 decision 5).
    Taken { session_id: String, reason: String },
    /// The prompt was stopped before OMP got it.
    PromptCancelled,
    /// OMP got the prompt at `sent`; its `prompt_result` came at `returned`.
    PromptTimed { sent: Instant, returned: Instant },
    /// OMP ran the prompt, it was stopped, and the call ended in an error
    /// instead of a result.
    PromptStopped(String),
    /// OMP did not run the prompt: it refused it, or the call failed.
    PromptFailed(String),
    /// Stop's abort did not reach OMP.
    AbortFailed(String),
    /// OMP took a mid-turn steer or follow-up with this text.
    Queued(String),
    /// A mid-turn steer or follow-up with this text did not reach OMP's
    /// queue, and why.
    QueueRefused { text: String, reason: String },
    /// Stop took these queued messages back out of OMP's queue before the
    /// abort, oldest first, for the composer.
    Restored(Vec<String>),
    /// Stop could not take some queued messages back: what to tell the
    /// person.
    TakeBackNotice(String),
    /// The model picker's choices and OMP's current model and level.
    Picker(Result<PickerState, String>),
    /// OMP's answer to a model or thinking-level change.
    PickerChanged(Result<PickerChange, String>),
    /// OMP's login providers, for onboarding.
    LoginProviders(Result<Vec<LoginProvider>, String>),
    /// OMP's answer to a login with `provider`.
    LoggedIn {
        provider: String,
        result: Result<(), String>,
    },
    /// OMP's answer to a session setting: what it now is, for the thread.
    Setting(Result<String, String>),
    /// OMP's answer to promoting the queued follow-up `text` to a steer.
    Promoted {
        text: String,
        result: Result<bool, String>,
    },
    /// OMP's answer to a Steer on subagent `id`.
    SubagentSteered {
        id: String,
        result: Result<(), String>,
    },
    /// OMP's answer to a Cancel on subagent `id`: whether it was running.
    SubagentCancelled {
        id: String,
        result: Result<bool, String>,
    },
}

const NOT_RUNNING: &str = "OMP is not running";

/// What OMP offers and runs now, for the model and thinking-level picker.
#[derive(Debug, Clone, PartialEq)]
pub struct PickerState {
    pub models: Vec<ModelInfo>,
    pub levels: Vec<ThinkingLevel>,
    /// `provider/id`.
    pub model: Option<String>,
    pub thinking: Option<ThinkingLevel>,
}

/// What a change made current; `None` where it changed nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PickerChange {
    pub model: Option<String>,
    pub thinking: Option<ThinkingLevel>,
}

/// A setting of the live session the panel changes (ADR-0057 decision 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionSetting {
    SteeringMode(QueueMode),
    FollowUpMode(QueueMode),
    InterruptMode(InterruptMode),
    AutoCompaction(bool),
    AutoRetry(bool),
    Compact,
    AbortRetry,
}

impl SessionSetting {
    fn apply(self, control: &RuntimeControl) -> Result<String, OmpError> {
        let on = |b: bool| if b { "on" } else { "off" };
        Ok(match self {
            Self::SteeringMode(mode) => {
                control.call(&SetSteeringModeCommand { mode })?;
                format!("steering mode: {}", mode.as_str())
            }
            Self::FollowUpMode(mode) => {
                control.call(&SetFollowUpModeCommand { mode })?;
                format!("follow-up mode: {}", mode.as_str())
            }
            Self::InterruptMode(mode) => {
                control.call(&SetInterruptModeCommand { mode })?;
                format!("interrupt mode: {}", mode.as_str())
            }
            Self::AutoCompaction(enabled) => {
                control.call(&SetAutoCompactionCommand { enabled })?;
                format!("auto-compaction {}", on(enabled))
            }
            Self::AutoRetry(enabled) => {
                control.call(&SetAutoRetryCommand { enabled })?;
                format!("auto-retry {}", on(enabled))
            }
            Self::Compact => {
                let result = control.call(&CompactCommand {
                    custom_instructions: None,
                })?;
                format!(
                    "compacted {} tokens: {}",
                    result.tokens_before,
                    result.short_summary.unwrap_or(result.summary)
                )
            }
            Self::AbortRetry => {
                control.call(&AbortRetryCommand {})?;
                "retry stopped".to_string()
            }
        })
    }
}

/// `provider/id`, how the picker names a model.
pub fn model_key(model: &ModelInfo) -> String {
    format!("{}/{}", model.provider, model.id)
}

/// What the panel asks of the OMP thread, in order.
enum Command {
    Prompt(Prompt),
    /// Check the session again for another driver.
    Retry,
    /// Leave the session to its driver and start a fresh one.
    NewSession,
    /// Park the thread until the sender is dropped, so a test can pin a
    /// prompt in the queue across a Restart.
    #[cfg(any(test, feature = "test-support"))]
    Hold(mpsc::Receiver<()>),
}

/// One prompt from the composer.
#[derive(Debug, Clone)]
pub struct Prompt {
    pub text: String,
    pub images: Vec<ImageContent>,
}

/// Everything one launch needs, resolved before any process starts. What
/// needs `omp` itself to run is left to [`LaunchSpec::choose`], off the UI
/// thread.
pub struct LaunchSpec {
    /// `None`: chosen by [`LaunchSpec::choose`] (ADR-0057 decision 4).
    pub binary: Option<PathBuf>,
    pub workdir: PathBuf,
    /// The workspace's state dir: the spawn overlay (`omp/`) and the audit log.
    pub state_dir: PathBuf,
    pub sessions: Sessions,
    pub policy: SpawnPolicy,
    /// `policy = "omp"` (ADR-0035): OMP's config decides, and the panel
    /// shows the badge.
    pub policy_source: Policy,
    /// OMP's state root, where session owner leases live.
    pub omp_state: Option<PathBuf>,
    /// The host URI schemes registered before the session opens.
    pub uris: Vec<HostUri>,
    /// The host tools registered before the session opens (one
    /// `set_host_tools` replaces OMP's whole set, ADR-0004).
    pub tools: Vec<HostTool>,
    /// The runtime's router once OMP runs: the workflow channel binds
    /// reported evidence to the calls in its log (ADR-0031).
    pub router: Arc<OnceLock<Arc<EventRouter>>>,
    /// The workspace's browser once the panel opens it: the workflow
    /// judges captures against its frame now.
    pub browser: Arc<OnceLock<Arc<crate::browser::BrowserHost>>>,
    /// The task's workflow channel, which the tools above call.
    pub workflow: Arc<cedian_workflow::WorkflowChannel>,
    /// The panel's reader of the task's review once it is set: the
    /// reviewer reads its diffs from Zed's buffers (ADR-0055).
    pub review: Arc<OnceLock<ReviewReader>>,
    /// The settings resolved for this launch; every review of it uses them.
    pub settings: cedian_shell::Settings,
    /// The panel's unsaved buffers, which evidence binds to.
    pub unsaved: Unsaved,
}

/// Workspace-relative path → version of each open buffer with unsaved
/// edits, kept by the panel on the app thread and read wherever evidence
/// binds (ADR-0057 decision 7).
pub type Unsaved = Arc<Mutex<std::collections::BTreeMap<String, String>>>;

/// Reads the task's review on the app thread. Never call it there.
pub type ReviewReader =
    Arc<dyn Fn() -> Result<cedian_shell::review_agent::ReviewTask, String> + Send + Sync>;

impl LaunchSpec {
    /// Resolve settings, policy and paths for `workdir`. The app registers
    /// the workflow channel under either policy (it writes no workspace
    /// file, ADR-0055), and `cedian_worktree_request` only when project
    /// writes are allowed (it writes `.worktrees/` and a branch with no
    /// dialog); the panel adds its `cedian://` scheme.
    pub fn resolve(workdir: &Path) -> Result<Self, String> {
        let settings = cedian_shell::resolve_settings(workdir).map_err(|e| e.to_string())?;
        let choice = settings.policy_for(workdir, RunKind::Interactive);
        let state_dir = cedian_shell::state::dir(workdir)?;
        let router: Arc<OnceLock<Arc<EventRouter>>> = Arc::default();
        let log = Arc::clone(&router);
        let browser: Arc<OnceLock<Arc<crate::browser::BrowserHost>>> = Arc::default();
        let frame = Arc::clone(&browser);
        let unsaved = Unsaved::default();
        let channel = cedian_shell::workflow_host::channel(
            crate::panel::TASK_ID,
            workdir,
            &state_dir,
            &settings,
            move |tool, needle| {
                let calls = log.get()?.finished_tool_calls();
                cedian_shell::workflow_host::bound_call(&calls, tool, needle)
            },
            {
                let unsaved = Arc::clone(&unsaved);
                move |state| {
                    state.frame_seq = frame
                        .get()
                        .map(|b| b.state())
                        .filter(|s| s.running)
                        .map(|s| s.seq);
                    state.overlay(unsaved.lock().iter());
                }
            },
        );
        let review: Arc<OnceLock<ReviewReader>> = Arc::default();
        let reader = Arc::clone(&review);
        let read_task: ReviewReader =
            Arc::new(move || (reader.get().ok_or("the task's review is not open")?)());
        let read = Arc::clone(&read_task);
        let diffs: cedian_shell::review_findings::TaskDiffs =
            Arc::new(move || read().map(|task| task.diffs));
        let (blockers_dir, blocker_diffs) = (state_dir.clone(), Arc::clone(&diffs));
        channel.set_blockers(move || {
            cedian_shell::review_findings::open_blockers(&blockers_dir, || blocker_diffs())
        });
        let mut tools = channel.host_tools();
        let models = Arc::clone(&router);
        tools.push(cedian_shell::review_agent::review_request_tool(
            review_place(workdir, &state_dir),
            settings.clone(),
            read_task,
            diffs,
            Arc::new(move || {
                models
                    .get()
                    .map(|router| router.answered_models())
                    .unwrap_or_default()
            }),
            Arc::clone(&channel),
        ));
        let mut names = vec![
            cedian_workflow::WORKFLOW_UPDATE_TOOL,
            cedian_workflow::COMPLETE_TOOL,
            cedian_shell::review_agent::REVIEW_REQUEST_TOOL,
        ];
        if cedian_shell::launch::registers_worktree_request(&settings) {
            tools.push(cedian_worker::worktree_request_tool(
                workdir.to_path_buf(),
                state_dir.clone(),
            ));
            names.push(cedian_worker::WORKTREE_REQUEST_TOOL);
        }
        let policy = cedian_shell::launch::spawn_policy(&settings, choice.policy, &names);
        Ok(Self {
            binary: None,
            workdir: workdir.to_path_buf(),
            state_dir,
            sessions: Sessions::OmpDefault,
            policy,
            policy_source: choice.policy,
            omp_state: cedian_omp::driver::state_root(),
            uris: Vec::new(),
            tools,
            router,
            browser,
            workflow: channel,
            review,
            settings,
            unsaved,
        })
    }

    /// What needs `omp` to run, so never on the UI thread: the binary
    /// (ADR-0057 decision 4), the ADR-0041 pin of OMP's own allows, and
    /// under `policy = "omp"` the badge (ADR-0035 decision 4). Fills in
    /// `binary` and `policy`; returns the off-pin warning and the badge.
    pub fn choose(&mut self) -> Result<Chosen, String> {
        let (binary, warning) = self.choose_binary()?;
        self.read_allows(&binary)?;
        let badge = (self.policy_source == Policy::Omp)
            .then(|| cedian_shell::launch::omp_policy_badge(&binary, &self.workdir));
        Ok(Chosen {
            binary,
            warning,
            badge,
        })
    }

    /// The `omp` to run and its pin warning, without reading OMP's config.
    pub fn choose_binary(&mut self) -> Result<(PathBuf, Option<String>), String> {
        let warning = match &self.binary {
            Some(_) => None,
            None => {
                let omp = cedian_shell::launch::omp_binary()?;
                self.binary = Some(omp.binary);
                omp.warning
            }
        };
        Ok((self.binary.clone().ok_or("no omp binary")?, warning))
    }

    /// Under cedian's policy, the tools OMP's config allows, which the
    /// overlay pins to a prompt.
    pub fn read_allows(&mut self, binary: &Path) -> Result<(), String> {
        if self.policy_source == Policy::Cedian {
            self.policy.config_allows = cedian_shell::launch::config_allows(binary, &self.workdir)?;
        }
        Ok(())
    }
}

/// What [`LaunchSpec::choose`] found.
pub struct Chosen {
    pub binary: PathBuf,
    pub warning: Option<String>,
    pub badge: Option<String>,
}

/// Where the app's reviewer runs: its run dir and the role it reads sit
/// in the workspace's state dir, outside what the implementer writes.
pub fn review_place(workdir: &Path, state_dir: &Path) -> cedian_shell::review_agent::ReviewPlace {
    cedian_shell::review_agent::ReviewPlace {
        workdir: workdir.to_path_buf(),
        state_dir: state_dir.to_path_buf(),
        session_dir: state_dir.to_path_buf(),
    }
}

/// A running OMP. Dropping it closes the dialogs OMP still waits on (a
/// cancel reply and an `abstain` row each), aborts a running turn so the
/// old process ends promptly, and shuts OMP down (no `Disconnected` follows).
pub struct OmpLink {
    commands: mpsc::Sender<Command>,
    pid: Arc<AtomicU32>,
    gate: Arc<Gate>,
    thread: Option<std::thread::JoinHandle<()>>,
}

/// A dropped link's OMP thread and process, which the next link waits on.
struct Previous {
    thread: std::thread::JoinHandle<()>,
    pid: Option<u32>,
}

/// How long a restart waits for the previous OMP to let go of the session
/// before it kills it, in milliseconds.
static PREVIOUS_EXIT_MS: AtomicU64 = AtomicU64::new(10_000);

/// How long the killed previous OMP's thread may take to wind down.
const KILLED_EXIT: Duration = Duration::from_secs(3);

/// Shorten how long a restart waits before killing the previous OMP.
#[cfg(any(test, feature = "test-support"))]
pub fn set_previous_exit(limit: Duration) {
    PREVIOUS_EXIT_MS.store(limit.as_millis() as u64, Ordering::Relaxed);
}

/// A Cancel on a subagent with no OMP to take it is still the person's
/// cancel: its audit row (ADR-0050 decision 4), in `workdir`'s log.
pub fn cancel_without_omp(workdir: &Path, id: &str) -> Result<(), String> {
    let settings = cedian_shell::resolve_settings(workdir).map_err(|e| e.to_string())?;
    let choice = settings.policy_for(workdir, RunKind::Interactive);
    let policy = cedian_shell::launch::spawn_policy(&settings, choice.policy, &[]);
    let mut audit = AuditLog::open(&cedian_shell::state::dir(workdir)?, policy.approvals)?;
    audit.subagent_cancel(id, &Err("OMP is not running".to_string()))
}

/// Why [`OmpLink::answer`] failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnswerError {
    /// OMP did not get the answer; the dialog stays open.
    NotSent(String),
    /// OMP got the answer, so the dialog is closed, but its audit row could
    /// not be written: the turn must fail (ADR-0035).
    Unaudited(String),
}

/// Shared by the panel (answers), the event thread (tool rows, dialogs
/// opening and closing) and the OMP thread (control, once spawned).
struct Gate {
    /// Taken back before OMP shuts down: a clone left here would keep its
    /// client, and so the process, alive.
    control: Mutex<Option<RuntimeControl>>,
    turn: Mutex<TurnState>,
    state: Mutex<GateState>,
    /// What OMP has queued: what Stop takes back before aborting.
    queue: Mutex<Queue>,
    /// Held by a steer or follow-up from the moment it is checked against
    /// Stop until OMP answered and it is in `queue`, and by Stop while it
    /// takes the queue back and aborts: none lands after Stop's snapshot.
    /// So Stop can wait on one queue call OMP is slow to answer plus, on a
    /// dead link, one timeout for its take-back and one for its abort (the
    /// client's timeout bounds each; Restart frees them); the panel shows
    /// Stopping meanwhile.
    sending: Mutex<()>,
    /// Counts Stops; a queue call made before the latest is refused.
    stops: AtomicU64,
    events: UnboundedSender<LinkEvent>,
}

/// A queued message: its text and whether it steers (else a follow-up).
type Queued = (String, bool);

#[derive(Default, Clone)]
struct Queue {
    /// OMP's latest `queue_update`, steers first, in OMP's own text.
    listed: Vec<Queued>,
    /// What OMP said yes to since it last settled. Its `queue_update` may
    /// not be read yet, and two identical steers are two entries.
    accepted: Vec<Queued>,
}

/// What came of Stop taking OMP's queue back, in the order tried.
#[derive(Debug, Default, PartialEq, Eq)]
struct TakenBack {
    /// OMP gave these back: they go to the composer.
    restored: Vec<String>,
    /// OMP no longer held these (`removed: false`): already taken into a run.
    gone: Vec<String>,
    /// OMP never answered the remove: their text goes back to the
    /// composer too, though OMP may still hold them.
    unanswered: Vec<String>,
    error: Option<String>,
}

/// Try to take back each queued message, steers first. Each accepted
/// message is removed by the text the person typed: OMP 18.6.1 matches that
/// first, then the content its `queue_update` shows, which differs for a
/// template or slash command. Every listed entry no accepted message names
/// by its text is tried too. Each accepted message OMP gave back under a text
/// it does not list has a display twin among those entries; which one is
/// unknown, so that many `removed: false` answers are the twins, not messages
/// taken into a run. After the first error the link is taken as dead: the
/// rest go back to the composer unanswered, without another timeout.
fn take_back(
    queue: &Queue,
    mut remove: impl FnMut(&str, bool) -> Result<bool, String>,
) -> TakenBack {
    let mut out = TakenBack::default();
    for steering in [true, false] {
        let mut listed: Vec<&String> = queue
            .listed
            .iter()
            .filter(|(_, s)| *s == steering)
            .map(|(text, _)| text)
            .collect();
        let mut plan = Vec::new();
        let mut display_only = Vec::new();
        for (text, _) in queue.accepted.iter().filter(|(_, s)| *s == steering) {
            match listed.iter().position(|shown| *shown == text) {
                Some(index) => {
                    listed.remove(index);
                }
                None => display_only.push(plan.len()),
            }
            plan.push(text.clone());
        }
        let mut twins = 0;
        for (index, text) in plan.into_iter().enumerate() {
            if out.try_remove(text, steering, &mut remove) && display_only.contains(&index) {
                twins += 1;
            }
        }
        for text in listed {
            if twins > 0 && out.error.is_none() {
                match remove(text, steering) {
                    Ok(false) => twins -= 1,
                    answer => out.record(text.clone(), answer),
                }
            } else {
                out.try_remove(text.clone(), steering, &mut remove);
            }
        }
    }
    out
}

impl TakenBack {
    /// Whether OMP gave `text` back.
    fn try_remove(
        &mut self,
        text: String,
        steering: bool,
        remove: &mut impl FnMut(&str, bool) -> Result<bool, String>,
    ) -> bool {
        if self.error.is_some() {
            self.unanswered.push(text);
            return false;
        }
        let answer = remove(&text, steering);
        let given_back = answer == Ok(true);
        self.record(text, answer);
        given_back
    }

    fn record(&mut self, text: String, answer: Result<bool, String>) {
        match answer {
            Ok(true) => self.restored.push(text),
            Ok(false) => self.gone.push(text),
            Err(e) => {
                log::warn!("cedian: OMP did not answer taking back a queued message: {e}");
                self.unanswered.push(text);
                self.error = Some(e);
            }
        }
    }

    /// The text to hand back to the composer, oldest first: everything
    /// restored came before the first error, everything unanswered after.
    fn to_composer(&self) -> Vec<String> {
        self.restored
            .iter()
            .chain(&self.unanswered)
            .cloned()
            .collect()
    }

    /// What the person is told about messages Stop could not take back.
    fn notice(&self) -> Option<String> {
        let mut parts = Vec::new();
        if !self.gone.is_empty() {
            parts.push(format!(
                "OMP had already taken these into a run, so they are not given back: {}",
                self.gone.join(" · ")
            ));
        }
        if !self.unanswered.is_empty() {
            parts.push(format!(
                "OMP did not answer taking these back, so it may still run them; the text is back in the composer: {} ({})",
                self.unanswered.join(" · "),
                self.error.as_deref().unwrap_or("no answer")
            ));
        }
        (!parts.is_empty()).then(|| parts.join("; "))
    }
}

/// The one prompt in flight, shared by the panel (send, Stop, Drop), the OMP
/// thread (which sends it) and the event thread (which sees it start), so a
/// Stop or Drop at any point ends it exactly once: before OMP has it, the
/// OMP thread drops it; once OMP started it, one abort goes out.
#[derive(Default)]
struct TurnState {
    phase: Phase,
    /// Stop or an audit failure: the current prompt must not run.
    cancelled: bool,
    /// The link was dropped: no further command runs.
    closed: bool,
    /// OMP started a run no prompt of ours asked for (its own drain of a
    /// queued steer) and has not settled it yet.
    unprompted: bool,
    /// Our prompt's call returned before its `agent_start` was read.
    ours_unread: bool,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    #[default]
    Idle,
    /// Waiting in the queue or in the driver check.
    Queued,
    /// Written to OMP, not started yet.
    Sent,
    Started,
}

impl Gate {
    fn send(&self, reply: ExtensionUiResponse) -> Result<(), String> {
        let control = self.control.lock().clone().ok_or("OMP is not running")?;
        control.respond(reply).map_err(|e| e.to_string())
    }

    /// Cancel the current prompt; with `close`, every later one too. One
    /// abort stops what runs: our started prompt, or a run of OMP's own.
    fn cancel(self: &Arc<Self>, close: bool) {
        self.stops.fetch_add(1, Ordering::SeqCst);
        let mut turn = self.turn.lock();
        let first = !std::mem::replace(&mut turn.cancelled, true);
        turn.closed |= close;
        let ours = first && turn.phase == Phase::Started;
        if std::mem::take(&mut turn.unprompted) || ours {
            self.abort();
        }
    }

    /// The event thread saw a run start or the session settle. The run that
    /// starts once our prompt is sent is that prompt (aborted at once if
    /// cancelled); any other is OMP's own. Both are decided under one lock,
    /// so our run is never taken for OMP's.
    fn observe_run(self: &Arc<Self>, event: &RouterEvent) {
        match event {
            RouterEvent::AgentStart => {
                let mut turn = self.turn.lock();
                let phase = turn.phase;
                match phase {
                    Phase::Sent => {
                        turn.phase = Phase::Started;
                        if turn.cancelled {
                            self.abort();
                        }
                    }
                    _ if std::mem::take(&mut turn.ours_unread) => {}
                    _ => turn.unprompted = true,
                }
            }
            RouterEvent::Settled | RouterEvent::Disconnected => {
                let mut turn = self.turn.lock();
                turn.unprompted = false;
                turn.ours_unread = false;
                drop(turn);
                self.queue.lock().accepted.clear();
            }
            RouterEvent::Queue {
                steering,
                follow_up,
            } => {
                let steering = steering.iter().map(|text| (text.clone(), true));
                let follow_up = follow_up.iter().map(|text| (text.clone(), false));
                self.queue.lock().listed = steering.chain(follow_up).collect();
            }
            _ => {}
        }
    }

    /// Our prompt's call returned; `omp_started` says whether its events
    /// held an `agent_start`. Returns whether it was cancelled and whether
    /// the event thread had seen it start.
    fn prompt_ended(&self, omp_started: bool) -> (bool, bool) {
        let mut turn = self.turn.lock();
        let started = turn.phase == Phase::Started;
        // `prompt_result` can wake this call before the event thread reads
        // the `agent_start` ahead of it: that one is still ours.
        turn.ours_unread = omp_started && turn.phase == Phase::Sent;
        turn.phase = Phase::Idle;
        (turn.cancelled, started)
    }

    fn epoch(&self) -> u64 {
        self.stops.load(Ordering::SeqCst)
    }

    /// Let a steer or follow-up made at `epoch` go out, unless Stop came
    /// since. Hold the guard until OMP's answer is in `queue`.
    fn admit(&self, epoch: u64) -> Option<parking_lot::MutexGuard<'_, ()>> {
        let sending = self.sending.lock();
        (self.epoch() == epoch).then_some(sending)
    }

    /// Promote the follow-up `text` clicked at `epoch` through `call`; on
    /// yes, Stop's take-back looks for it in the steering queue.
    fn promote(
        &self,
        epoch: u64,
        text: &str,
        call: impl FnOnce(&str) -> Result<bool, String>,
    ) -> Result<bool, String> {
        let Some(_sending) = self.admit(epoch) else {
            return Err("Stop came first; it was not steered".to_string());
        };
        let result = call(text);
        if result == Ok(true) {
            let mut queue = self.queue.lock();
            if let Some(entry) = queue
                .accepted
                .iter_mut()
                .find(|(t, steer)| t == text && !*steer)
            {
                entry.1 = true;
            }
        }
        result
    }

    /// OMP accepted a steer or follow-up.
    fn accepted(&self, text: &str, steer: bool) {
        self.queue.lock().accepted.push((text.to_string(), steer));
    }

    /// Abort the run. OMP's `abort` keeps queued user messages, and a kept
    /// steer starts a new run right after it, so each queued message is
    /// taken back first and handed to the composer.
    fn abort(self: &Arc<Self>) {
        let Some(control) = self.control.lock().clone() else {
            return;
        };
        let gate = Arc::clone(self);
        std::thread::spawn(move || {
            let _sending = gate.sending.lock();
            let queue = {
                let mut queue = gate.queue.lock();
                let snapshot = queue.clone();
                queue.accepted.clear();
                snapshot
            };
            let taken = take_back(&queue, |text, steering| {
                control
                    .remove_queued(text, steering)
                    .map_err(|e| e.to_string())
            });
            let composer = taken.to_composer();
            if !composer.is_empty() {
                let _ = gate.events.unbounded_send(LinkEvent::Restored(composer));
            }
            if let Some(notice) = taken.notice() {
                let _ = gate
                    .events
                    .unbounded_send(LinkEvent::TakeBackNotice(notice));
            }
            if let Err(e) = control.abort() {
                let _ = gate
                    .events
                    .unbounded_send(LinkEvent::AbortFailed(e.to_string()));
            }
        });
    }
}

#[derive(Default)]
struct GateState {
    audit: Option<AuditLog>,
    /// Dialogs OMP is waiting on, by request id.
    open: HashMap<String, ExtensionUiRequest>,
}

impl GateState {
    fn record(&mut self, record: DialogRecord) -> Result<(), String> {
        match &mut self.audit {
            Some(audit) => audit.dialog(&record),
            None => Err("the audit log is not open".to_string()),
        }
    }

    /// Every open dialog, closed with nobody to answer it.
    fn abstain_all(&mut self) -> Result<(), String> {
        let open: Vec<_> = self.open.drain().map(|(_, request)| request).collect();
        open.iter()
            .filter_map(cedian_omp::abstained)
            .try_for_each(|record| self.record(record))
    }

    fn observe(&mut self, event: &RouterEvent) -> Result<(), String> {
        match event {
            RouterEvent::UiRequest(ExtensionUiRequest::Cancel(cancel)) => {
                match self.open.remove(&cancel.target_id) {
                    Some(request) => {
                        cedian_omp::abstained(&request).map_or(Ok(()), |record| self.record(record))
                    }
                    None => Ok(()),
                }
            }
            RouterEvent::UiRequest(request) => {
                if let Some((id, _)) = cedian_omp::dialog::dialog(request) {
                    self.open.insert(id.to_string(), request.clone());
                }
                Ok(())
            }
            RouterEvent::Disconnected => self.abstain_all(),
            event => match &mut self.audit {
                Some(audit) => audit.record(event),
                None => Ok(()),
            },
        }
    }

    fn answer(
        &mut self,
        request_id: &str,
        answer: UserAnswer,
        send: impl FnOnce(ExtensionUiResponse) -> Result<(), String>,
    ) -> Result<(), AnswerError> {
        let request = self.open.get(request_id).ok_or_else(|| {
            AnswerError::NotSent("OMP no longer waits on that dialog".to_string())
        })?;
        let (reply, record) = cedian_omp::user_answer(request, answer).ok_or_else(|| {
            AnswerError::NotSent("that answer does not fit the dialog".to_string())
        })?;
        send(reply).map_err(AnswerError::NotSent)?;
        self.open.remove(request_id);
        self.record(record).map_err(AnswerError::Unaudited)
    }

    /// Close dialog `request_id` (every open one when `None`) for OMP: a
    /// cancel reply, then its `abstain` row. A reply that fails to go out
    /// still leaves the row: nobody answered. Returns whether any was open.
    fn abandon(
        &mut self,
        request_id: Option<&str>,
        timed_out: bool,
        send: impl Fn(ExtensionUiResponse) -> Result<(), String>,
    ) -> Result<bool, String> {
        let requests: Vec<_> = match request_id {
            Some(id) => self.open.remove(id).into_iter().collect(),
            None => self.open.drain().map(|(_, request)| request).collect(),
        };
        let any = !requests.is_empty();
        let mut first_error = None;
        for request in &requests {
            if let Some((reply, record)) = cedian_omp::abandoned(request, timed_out) {
                if let Err(e) = send(reply) {
                    log::warn!("cedian: OMP did not get the dialog's cancel: {e}");
                }
                if let Err(e) = self.record(record) {
                    first_error.get_or_insert(e);
                }
            }
        }
        first_error.map_or(Ok(any), Err)
    }
}

impl OmpLink {
    /// Start OMP on its own thread; everything it reports goes to `events`.
    /// Prompts sent before it is ready wait in order.
    /// `previous`, the link this one replaces, is dropped here; the new OMP
    /// opens its session only once the old one has ended.
    pub fn start(
        spec: LaunchSpec,
        events: UnboundedSender<LinkEvent>,
        previous: Option<OmpLink>,
    ) -> Self {
        let previous = previous.and_then(|mut link| {
            let pid = link.pid();
            link.thread.take().map(|thread| Previous { thread, pid })
        });
        let (commands, command_rx) = mpsc::channel::<Command>();
        let pid = Arc::new(AtomicU32::new(0));
        let gate = Arc::new(Gate {
            control: Mutex::default(),
            turn: Mutex::default(),
            state: Mutex::default(),
            queue: Mutex::default(),
            sending: Mutex::default(),
            stops: AtomicU64::default(),
            events: events.clone(),
        });
        let thread_pid = Arc::clone(&pid);
        let thread_gate = Arc::clone(&gate);
        let thread = std::thread::Builder::new()
            .name("cedian-omp".to_string())
            .spawn(move || run(spec, command_rx, events, thread_pid, thread_gate, previous))
            .inspect_err(|e| log::error!("cedian: cannot start the OMP thread: {e}"))
            .ok();
        Self {
            commands,
            pid,
            gate,
            thread,
        }
    }

    /// Send a prompt. One runs at a time: the panel sends the next only
    /// once this one has settled, failed, been refused or cancelled.
    pub fn send(&self, prompt: Prompt) -> Result<(), String> {
        {
            let mut turn = self.gate.turn.lock();
            turn.phase = Phase::Queued;
            turn.cancelled = false;
            turn.ours_unread = false;
        }
        self.command(Command::Prompt(prompt))
    }

    /// Stop the current prompt: dropped if OMP does not have it yet,
    /// aborted if it does.
    pub fn cancel(&self) {
        self.gate.cancel(false);
    }

    /// Park the OMP thread before the next command; dropping the sender
    /// lets it go on.
    #[cfg(any(test, feature = "test-support"))]
    pub fn hold(&self) -> Result<mpsc::Sender<()>, String> {
        let (release, held) = mpsc::channel();
        self.command(Command::Hold(held)).map(|()| release)
    }

    /// Whether the current prompt waits in the queue or the driver check.
    #[cfg(any(test, feature = "test-support"))]
    pub fn prompt_queued(&self) -> bool {
        self.gate.turn.lock().phase == Phase::Queued
    }

    /// Whether OMP runs a run of its own (a queued steer it drains).
    pub fn runs_own(&self) -> bool {
        self.gate.turn.lock().unprompted
    }

    /// Whether a prompt of ours is still in the link or in OMP.
    #[cfg(any(test, feature = "test-support"))]
    pub fn prompt_open(&self) -> bool {
        self.gate.turn.lock().phase != Phase::Idle
    }

    /// Whether OMP runs a run of its own, no prompt of ours in flight.
    #[cfg(any(test, feature = "test-support"))]
    pub fn runs_unprompted(&self) -> bool {
        let turn = self.gate.turn.lock();
        turn.phase == Phase::Idle && turn.unprompted
    }

    /// Whether OMP still waits on dialog `request_id`.
    pub fn is_open(&self, request_id: &str) -> bool {
        self.gate.state.lock().open.contains_key(request_id)
    }

    /// Make every later audit row fail, as a full disk would.
    #[cfg(any(test, feature = "test-support"))]
    pub fn break_audit(&self) {
        self.gate.state.lock().audit = None;
    }

    /// Check a taken session again; `Ready` or `Taken` follows.
    pub fn retry(&self) -> Result<(), String> {
        self.command(Command::Retry)
    }

    /// Start a fresh session; `Ready` follows.
    pub fn new_session(&self) -> Result<(), String> {
        self.command(Command::NewSession)
    }

    fn command(&self, command: Command) -> Result<(), String> {
        self.commands
            .send(command)
            .map_err(|_| "OMP is not running".to_string())
    }

    /// OMP's process id, once it has started.
    pub fn pid(&self) -> Option<u32> {
        Some(self.pid.load(Ordering::Relaxed)).filter(|pid| *pid != 0)
    }

    /// Steer the running turn (`steer`) or queue a message after it
    /// (`follow_up`), off the UI thread; OMP's `queue_update` shows it.
    pub fn queue(&self, text: String, steer: bool) {
        let gate = Arc::clone(&self.gate);
        let epoch = gate.epoch();
        let refused = |text: &String, reason: String| LinkEvent::QueueRefused {
            text: text.clone(),
            reason,
        };
        let not_running = refused(&text, "OMP is not running".to_string());
        self.off_thread(not_running, move |control| {
            let Some(_sending) = gate.admit(epoch) else {
                return refused(&text, "Stop came first; it stays here".to_string());
            };
            let sent = if steer {
                control.steer(&text)
            } else {
                control.follow_up(&text)
            };
            match sent {
                Ok(()) => {
                    gate.accepted(&text, steer);
                    LinkEvent::Queued(text)
                }
                Err(e) => refused(&text, e.to_string()),
            }
        });
    }

    /// Steer subagent `id` off the UI thread; the answer comes back as
    /// [`LinkEvent::SubagentSteered`].
    pub fn steer_subagent(&self, id: String, message: String) {
        let not_running = LinkEvent::SubagentSteered {
            id: id.clone(),
            result: Err("OMP is not running".to_string()),
        };
        self.off_thread(not_running, move |control| LinkEvent::SubagentSteered {
            result: control
                .steer_subagent(&id, &message)
                .map_err(|e| e.to_string()),
            id,
        });
    }

    /// Cancel subagent `id` off the UI thread. The person's cancel is an
    /// audit row whatever came of it (ADR-0050 decision 4).
    pub fn cancel_subagent(&self, id: String) {
        let gate = Arc::clone(&self.gate);
        let control = gate.control.lock().clone();
        std::thread::spawn(move || {
            let result = match control {
                Some(control) => control.cancel_subagent(&id).map_err(|e| e.to_string()),
                None => Err("OMP is not running".to_string()),
            };
            let recorded = match &mut gate.state.lock().audit {
                Some(audit) => audit.subagent_cancel(&id, &result),
                None => Err("the audit log is not open".to_string()),
            };
            if let Err(e) = recorded {
                let _ = gate.events.unbounded_send(LinkEvent::AuditFailed(e));
            }
            let _ = gate
                .events
                .unbounded_send(LinkEvent::SubagentCancelled { id, result });
        });
    }

    /// Read the picker's state off the UI thread: the session's model and
    /// level, and what OMP offers.
    pub fn refresh_picker(&self) {
        self.off_thread(LinkEvent::Picker(Err(NOT_RUNNING.to_string())), |control| {
            let read = || -> Result<PickerState, OmpError> {
                let state = control.call(&GetStateCommand {})?;
                Ok(PickerState {
                    models: control.call(&GetAvailableModelsCommand {})?,
                    levels: control.call(&GetAvailableThinkingLevelsCommand {})?,
                    model: state.model.as_ref().map(model_key),
                    thinking: state.thinking_level,
                })
            };
            LinkEvent::Picker(read().map_err(|e| e.to_string()))
        });
    }

    /// OMP's login providers, off the UI thread.
    pub fn login_providers(&self) {
        self.off_thread(
            LinkEvent::LoginProviders(Err(NOT_RUNNING.to_string())),
            |control| {
                LinkEvent::LoginProviders(
                    control
                        .call(&GetLoginProvidersCommand {})
                        .map_err(|e| e.to_string()),
                )
            },
        );
    }

    /// Run OMP's login with `provider`. It blocks until the person signs
    /// in (OMP allows 10 minutes); its sign-in page and code prompt come as
    /// `open_url` and `input` UI requests meanwhile.
    pub fn login(&self, provider: String) {
        let refused = LinkEvent::LoggedIn {
            provider: provider.clone(),
            result: Err(NOT_RUNNING.to_string()),
        };
        self.off_thread(refused, move |control| {
            let result = control
                .call(&LoginCommand {
                    provider_id: provider.clone(),
                })
                .map(drop)
                .map_err(|e| e.to_string());
            LinkEvent::LoggedIn { provider, result }
        });
    }

    /// Make `provider/id` the session's model.
    pub fn set_model(&self, provider: String, model_id: String) {
        self.change(move |control| {
            let model = control.call(&SetModelCommand { provider, model_id })?;
            Ok(PickerChange {
                model: Some(model_key(&model)),
                thinking: None,
            })
        });
    }

    /// OMP's next model in its cycle.
    pub fn cycle_model(&self) {
        self.change(|control| {
            let next = control.call(&CycleModelCommand {})?;
            Ok(PickerChange {
                model: next.as_ref().map(|n| model_key(&n.model)),
                thinking: next.and_then(|n| n.thinking_level),
            })
        });
    }

    pub fn set_thinking_level(&self, level: ThinkingLevel) {
        self.change(move |control| {
            control.call(&SetThinkingLevelCommand { level })?;
            Ok(PickerChange {
                model: None,
                thinking: Some(level),
            })
        });
    }

    /// OMP's next thinking level in its cycle.
    pub fn cycle_thinking_level(&self) {
        self.change(|control| {
            let next = control.call(&CycleThinkingLevelCommand {})?;
            Ok(PickerChange {
                model: None,
                thinking: next.and_then(|n| {
                    serde_json::to_value(n.level)
                        .and_then(serde_json::from_value)
                        .ok()
                }),
            })
        });
    }

    /// Change a setting of the live session off the UI thread.
    pub fn set(&self, setting: SessionSetting) {
        self.off_thread(
            LinkEvent::Setting(Err(NOT_RUNNING.to_string())),
            move |control| LinkEvent::Setting(setting.apply(&control).map_err(|e| e.to_string())),
        );
    }

    /// Move the queued follow-up `text` to the steering queue, so it runs
    /// in the current turn. Stop's take-back then looks for it there.
    pub fn promote(&self, text: String) {
        let gate = Arc::clone(&self.gate);
        let epoch = gate.epoch();
        let Some(control) = gate.control.lock().clone() else {
            let _ = gate.events.unbounded_send(LinkEvent::Promoted {
                text,
                result: Err(NOT_RUNNING.to_string()),
            });
            return;
        };
        std::thread::spawn(move || {
            let result = gate.promote(epoch, &text, |text| {
                control
                    .call(&PromoteQueuedMessageCommand {
                        message: text.to_string(),
                    })
                    .map(|r| r.promoted)
                    .map_err(|e| e.to_string())
            });
            let _ = gate
                .events
                .unbounded_send(LinkEvent::Promoted { text, result });
        });
    }

    fn change(
        &self,
        call: impl FnOnce(&RuntimeControl) -> Result<PickerChange, OmpError> + Send + 'static,
    ) {
        self.off_thread(
            LinkEvent::PickerChanged(Err(NOT_RUNNING.to_string())),
            move |control| LinkEvent::PickerChanged(call(&control).map_err(|e| e.to_string())),
        );
    }

    /// Run a blocking control call on its own thread, as Stop's abort runs.
    fn off_thread(
        &self,
        not_running: LinkEvent,
        call: impl FnOnce(RuntimeControl) -> LinkEvent + Send + 'static,
    ) {
        let gate = Arc::clone(&self.gate);
        let Some(control) = gate.control.lock().clone() else {
            let _ = gate.events.unbounded_send(not_running);
            return;
        };
        std::thread::spawn(move || {
            let event = call(control);
            let _ = gate.events.unbounded_send(event);
        });
    }

    /// Mid-turn control (abort, steer), once OMP has started. Its calls
    /// block until OMP answers: run them off the UI thread.
    pub fn control(&self) -> Option<RuntimeControl> {
        self.gate.control.lock().clone()
    }

    /// Send a person's answer to dialog `request_id` and record it.
    pub fn answer(&self, request_id: &str, answer: UserAnswer) -> Result<(), AnswerError> {
        self.gate
            .state
            .lock()
            .answer(request_id, answer, |reply| self.gate.send(reply))
    }

    /// Stop: close every dialog OMP waits on. The caller aborts the turn.
    pub fn close_dialogs(&self) -> Result<(), String> {
        self.gate
            .state
            .lock()
            .abandon(None, false, |reply| self.gate.send(reply))
            .map(|_| ())
    }

    /// The §63 lease on dialog `request_id` ran out: OMP gets a timed-out
    /// cancel. `Ok(false)` when it was already closed.
    pub fn expire(&self, request_id: &str) -> Result<bool, String> {
        self.gate
            .state
            .lock()
            .abandon(Some(request_id), true, |reply| self.gate.send(reply))
    }
}

impl Drop for OmpLink {
    fn drop(&mut self) {
        if let Err(e) = self.close_dialogs() {
            log::error!("cedian: {e}");
        }
        self.gate.cancel(true);
    }
}

fn run(
    spec: LaunchSpec,
    commands: mpsc::Receiver<Command>,
    events: UnboundedSender<LinkEvent>,
    pid: Arc<AtomicU32>,
    gate: Arc<Gate>,
    previous: Option<Previous>,
) {
    let mut spec = spec;
    let chosen = match spec.choose() {
        Ok(chosen) => chosen,
        Err(e) => {
            let _ = events.unbounded_send(LinkEvent::Failed(e));
            return;
        }
    };
    if let Some(warning) = chosen.warning {
        let _ = events.unbounded_send(LinkEvent::OmpWarning(warning));
    }
    if let Some(badge) = &chosen.badge {
        let _ = events.unbounded_send(LinkEvent::OmpPolicy(badge.clone()));
    }
    match AuditLog::open(&spec.state_dir, spec.policy.approvals) {
        Ok(audit) => gate.state.lock().audit = Some(audit),
        Err(e) => {
            let _ = events.unbounded_send(LinkEvent::Failed(format!(
                "OMP not started, nothing could be audited: {e}"
            )));
            return;
        }
    }
    let mut runtime = match OmpRuntime::spawn(RuntimeConfig {
        binary: OmpBinary::Bundled(chosen.binary),
        session_dir: spec.state_dir.join("omp"),
        sessions: spec.sessions,
        cwd: spec.workdir,
        ask_dialog: true,
        prompt_timeout: Duration::from_secs(600),
        policy: spec.policy,
    }) {
        Ok(runtime) => runtime,
        Err(e) => {
            let _ = events.unbounded_send(LinkEvent::Failed(format!("OMP did not start: {e}")));
            return;
        }
    };
    let _ = spec.router.set(runtime.router());
    if !spec.tools.is_empty()
        && let Err(e) = runtime.set_host_tools(spec.tools)
    {
        let _ = events.unbounded_send(LinkEvent::Failed(format!(
            "OMP refused cedian's host tools: {e}"
        )));
        let own = runtime.pid();
        shutdown(runtime, &gate, own);
        return;
    }
    if !spec.uris.is_empty()
        && let Err(e) = runtime.set_host_uris(spec.uris)
    {
        let _ = events.unbounded_send(LinkEvent::Failed(format!(
            "OMP refused cedian:// context: {e}"
        )));
        let own = runtime.pid();
        shutdown(runtime, &gate, own);
        return;
    }
    if let Err(e) = runtime.subscribe_subagents() {
        let _ = events.unbounded_send(LinkEvent::Failed(format!(
            "OMP refused the subagent subscription: {e}"
        )));
        let own = runtime.pid();
        shutdown(runtime, &gate, own);
        return;
    }
    let own = runtime.pid();
    pid.store(own.unwrap_or(0), Ordering::Relaxed);
    *gate.control.lock() = Some(runtime.control());
    let (_sub, router_events) = runtime.router().subscribe();
    let forward = events.clone();
    let forward_gate = Arc::clone(&gate);
    std::thread::spawn(move || {
        for event in router_events {
            forward_gate.observe_run(&event);
            if let Err(e) = forward_gate.state.lock().observe(&event) {
                let _ = forward.unbounded_send(LinkEvent::AuditFailed(e));
            }
            if forward.unbounded_send(LinkEvent::Event(event)).is_err() {
                break;
            }
        }
    });
    if let Some(previous) = previous {
        wait_for(previous);
    }
    let mut session = match runtime.open_session("app") {
        Ok(opened) => Session {
            id: opened.session_id,
            file: opened.session_file,
            resumed: opened.resumed,
        },
        Err(e) => {
            let _ = events.unbounded_send(LinkEvent::Failed(format!(
                "OMP's session did not open: {e}"
            )));
            shutdown(runtime, &gate, own);
            return;
        }
    };
    let state = spec.omp_state;
    let taken = |session: &Session| -> Option<LinkEvent> {
        let files = cedian_omp::driver::session_files(
            state.as_deref(),
            &session.id,
            session.file.as_deref().map(Path::new),
        );
        let reason = match cedian_omp::driver::other_drivers_of(&files, own, session.resumed) {
            Ok(others) if others.is_empty() => return None,
            Ok(others) => format!(
                "another process drives this session (pid {})",
                others
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Err(e) => format!("cannot check whether another process drives this session: {e}"),
        };
        Some(LinkEvent::Taken {
            session_id: session.id.clone(),
            reason,
        })
    };
    let ready = |session: &Session, state: Option<omp_rpc::SessionState>| LinkEvent::Ready {
        session_id: session.id.clone(),
        session_file: session.file.clone(),
        resumed: session.resumed,
        policy_note: chosen.badge.clone(),
        model: state.as_ref().and_then(|s| s.model.as_ref()).map(model_key),
        thinking: state.and_then(|s| s.thinking_level),
    };
    let first = if session.resumed {
        taken(&session)
    } else {
        None
    };
    let _ = events.unbounded_send(
        first.unwrap_or_else(|| ready(&session, runtime.control().call(&GetStateCommand {}).ok())),
    );
    let idle = |turn: &mut TurnState| turn.phase = Phase::Idle;
    for command in commands {
        if gate.turn.lock().closed {
            break;
        }
        match command {
            Command::Prompt(prompt) => {
                if let Some(refusal) = taken(&session) {
                    idle(&mut gate.turn.lock());
                    let _ = events.unbounded_send(refusal);
                    continue;
                }
                {
                    let mut turn = gate.turn.lock();
                    if turn.cancelled || turn.closed {
                        idle(&mut turn);
                        let _ = events.unbounded_send(LinkEvent::PromptCancelled);
                        continue;
                    }
                    turn.phase = Phase::Sent;
                }
                let sent = Instant::now();
                let result = runtime.prompt(&prompt.text, prompt.images);
                let returned = Instant::now();
                let omp_started = result.as_ref().is_ok_and(|turn| {
                    turn.events
                        .iter()
                        .any(|event| matches!(event, RpcAgentEvent::AgentStart(_)))
                });
                let (cancelled, started) = gate.prompt_ended(omp_started);
                if result.is_ok() {
                    let _ = events.unbounded_send(LinkEvent::PromptTimed { sent, returned });
                }
                if let Err(e) = result {
                    let _ = events.unbounded_send(match (cancelled, started) {
                        (true, true) => LinkEvent::PromptStopped(e.to_string()),
                        (true, false) => LinkEvent::PromptCancelled,
                        (false, _) => LinkEvent::PromptFailed(e.to_string()),
                    });
                }
            }
            Command::Retry => {
                let _ = events.unbounded_send(taken(&session).unwrap_or_else(|| {
                    ready(&session, runtime.control().call(&GetStateCommand {}).ok())
                }));
            }
            Command::NewSession => {
                let refused = |reason: String| LinkEvent::Taken {
                    session_id: session.id.clone(),
                    reason,
                };
                let event = match runtime.new_session(None, "app") {
                    Ok(NewSession::Started(state)) => {
                        session = Session {
                            id: state.session_id.clone(),
                            file: state.session_file.clone(),
                            resumed: false,
                        };
                        ready(&session, Some(state))
                    }
                    Ok(NewSession::Unknown(e)) => LinkEvent::Failed(format!(
                        "OMP started a new session but did not say which: {e}"
                    )),
                    Ok(NewSession::Declined) => {
                        refused("OMP declined to start a new session".to_string())
                    }
                    Err(e) => refused(format!("a new session did not start: {e}")),
                };
                let _ = events.unbounded_send(event);
            }
            #[cfg(any(test, feature = "test-support"))]
            Command::Hold(held) => {
                let _ = held.recv();
            }
        }
    }
    shutdown(runtime, &gate, own);
}

/// Shut OMP down; if a control call still holds its client, kill it, so
/// this thread ending means the process no longer holds the session.
fn shutdown(runtime: OmpRuntime, gate: &Gate, own: Option<u32>) {
    gate.control.lock().take();
    if let Err(e) = runtime.shutdown() {
        log::warn!("cedian: OMP did not shut down ({e}); killing it");
        if let Some(pid) = own {
            cedian_omp::driver::kill_group(pid);
        }
    }
}

/// Wait for the previous link's OMP to end, so it no longer holds the
/// session; it is ours, so one that does not end in time is killed with its
/// whole process group, and its thread gets a little longer to wind down.
fn wait_for(previous: Previous) {
    let limit = Duration::from_millis(PREVIOUS_EXIT_MS.load(Ordering::Relaxed));
    if finished_within(&previous.thread, limit) {
        return;
    }
    let Some(pid) = previous.pid else {
        // No pid to kill yet: still wait for the thread rather than open the
        // session under it and meet Taken.
        log::warn!("cedian: the previous OMP did not exit and has no pid yet; waiting");
        if !finished_within(&previous.thread, limit + KILLED_EXIT) {
            log::error!("cedian: the previous OMP's thread did not end");
        }
        return;
    };
    log::warn!("cedian: the previous OMP did not exit; killing it");
    cedian_omp::driver::kill_group(pid);
    if !finished_within(&previous.thread, KILLED_EXIT) {
        log::error!("cedian: the previous OMP's thread did not end after the kill");
    }
}

fn finished_within(thread: &std::thread::JoinHandle<()>, limit: Duration) -> bool {
    let deadline = std::time::Instant::now() + limit;
    while !thread.is_finished() {
        if std::time::Instant::now() >= deadline {
            return false;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    true
}

struct Session {
    id: String,
    file: Option<String>,
    resumed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt as _;
    use omp_rpc::wire::CancelUiResponse;

    fn approval(id: &str) -> RouterEvent {
        RouterEvent::UiRequest(
            ExtensionUiRequest::from_value(serde_json::json!({
                "type": "extension_ui_request", "method": "select", "id": id,
                "title": "Allow tool: bash\nCommand: ls", "options": ["Approve", "Deny"]
            }))
            .unwrap(),
        )
    }

    fn gate(dir: &Path) -> GateState {
        GateState {
            audit: Some(AuditLog::open(dir, SpawnPolicy::default().approvals).unwrap()),
            open: HashMap::default(),
        }
    }

    fn rows(dir: &Path) -> Vec<(String, String)> {
        std::fs::read_to_string(dir.join(cedian_shell::audit::AUDIT_FILE))
            .unwrap_or_default()
            .lines()
            .map(|l| serde_json::from_str::<serde_json::Value>(l).unwrap()["item"].clone())
            .filter(|item| item["kind"] == "gate")
            .map(|item| {
                (
                    item["decision"].as_str().unwrap().to_string(),
                    item["answered_by"].as_str().unwrap().to_string(),
                )
            })
            .collect()
    }

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("cedian-gate-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn abstain() -> (String, String) {
        ("abstain".to_string(), "cedian".to_string())
    }

    #[test]
    fn omp_withdrawing_a_dialog_is_one_abstain_row() {
        let dir = temp("cancel");
        let mut gate = gate(&dir);
        gate.observe(&approval("d1")).unwrap();
        let cancel = RouterEvent::UiRequest(
            ExtensionUiRequest::from_value(serde_json::json!({
                "type": "extension_ui_request", "method": "cancel", "id": "c1", "targetId": "d1"
            }))
            .unwrap(),
        );
        gate.observe(&cancel).unwrap();
        gate.observe(&cancel).unwrap();
        assert!(gate.open.is_empty());
        assert_eq!(rows(&dir), [abstain()]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn omp_dying_with_a_dialog_open_is_one_abstain_row() {
        let dir = temp("disconnect");
        let mut gate = gate(&dir);
        gate.observe(&approval("d1")).unwrap();
        gate.observe(&RouterEvent::Disconnected).unwrap();
        gate.observe(&RouterEvent::Disconnected).unwrap();
        assert!(gate.open.is_empty());
        assert_eq!(rows(&dir), [abstain()]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn an_answer_omp_got_but_the_audit_lost_closes_the_dialog_and_fails_the_turn() {
        let mut gate = GateState::default();
        gate.observe(&approval("d1")).unwrap();
        let sent = std::cell::RefCell::new(Vec::new());
        let result = gate.answer("d1", UserAnswer::Choice("Approve".into()), |reply| {
            sent.borrow_mut().push(reply);
            Ok(())
        });
        assert!(
            matches!(result, Err(AnswerError::Unaudited(_))),
            "{result:?}"
        );
        assert_eq!(sent.borrow().len(), 1, "OMP got the answer");
        assert!(gate.open.is_empty(), "nothing left to answer twice");
    }

    #[test]
    fn an_answer_omp_did_not_get_keeps_the_dialog_open() {
        let dir = temp("unsent");
        let mut gate = gate(&dir);
        gate.observe(&approval("d1")).unwrap();
        let result = gate.answer("d1", UserAnswer::Choice("Approve".into()), |_| {
            Err("pipe closed".to_string())
        });
        assert_eq!(result, Err(AnswerError::NotSent("pipe closed".to_string())));
        assert!(gate.open.contains_key("d1"));
        assert!(rows(&dir).is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_dialog_cedian_closes_gets_a_cancel_and_an_abstain_row() {
        let dir = temp("abandon");
        let mut gate = gate(&dir);
        gate.observe(&approval("d1")).unwrap();
        let sent = std::cell::RefCell::new(Vec::new());
        let send = |reply| {
            sent.borrow_mut().push(reply);
            Ok(())
        };
        assert_eq!(gate.abandon(Some("d1"), true, send), Ok(true));
        assert_eq!(gate.abandon(Some("d1"), true, send), Ok(false));
        assert_eq!(
            *sent.borrow(),
            [ExtensionUiResponse::CancelUiResponse(CancelUiResponse {
                id: "d1".into(),
                cancelled: omp_rpc::wire::LitTrue,
                timed_out: Some(true),
            })]
        );
        assert_eq!(rows(&dir), [abstain()]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn closing_every_dialog_with_the_audit_failing_still_cancels_each() {
        let mut gate = GateState::default();
        gate.observe(&approval("d1")).unwrap();
        gate.observe(&approval("d2")).unwrap();
        let sent = std::cell::RefCell::new(Vec::new());
        let result = gate.abandon(None, false, |reply| {
            sent.borrow_mut().push(reply);
            Ok(())
        });
        assert!(result.is_err(), "{result:?}");
        assert_eq!(sent.borrow().len(), 2, "every dialog got its cancel");
        assert!(gate.open.is_empty());
    }

    #[test]
    fn stop_takes_back_a_steer_whose_queue_update_is_not_read_yet() {
        let gate = Arc::new(bare_gate());
        gate.observe_run(&RouterEvent::Queue {
            steering: Vec::new(),
            follow_up: vec!["two".to_string()],
        });
        gate.accepted("faster", true);
        gate.accepted("two", false);
        let (calls, _) = removes(&gate, |_| Ok(true));
        assert_eq!(
            calls,
            [steer("faster"), ("two".to_string(), false)],
            "OMP said yes to faster: Stop takes it back; two once"
        );
    }

    /// Run Stop's take-back on `gate`'s queue, recording each remove sent.
    fn removes(
        gate: &Gate,
        answer: impl Fn(&str) -> Result<bool, String>,
    ) -> (Vec<Queued>, TakenBack) {
        let queue = gate.queue.lock().clone();
        let mut calls = Vec::new();
        let taken = take_back(&queue, |text, steering| {
            calls.push((text.to_string(), steering));
            answer(text)
        });
        (calls, taken)
    }

    fn bare_gate() -> Gate {
        Gate {
            control: Mutex::default(),
            turn: Mutex::default(),
            state: Mutex::default(),
            queue: Mutex::default(),
            sending: Mutex::default(),
            stops: AtomicU64::default(),
            events: futures::channel::mpsc::unbounded().0,
        }
    }

    fn steer(text: &str) -> (String, bool) {
        (text.to_string(), true)
    }

    #[test]
    fn two_identical_steers_are_both_taken_back() {
        let gate = Arc::new(bare_gate());
        gate.accepted("again", true);
        gate.accepted("again", true);
        let (calls, _) = removes(&gate, |_| Ok(true));
        assert_eq!(calls, [steer("again"), steer("again")]);
    }

    #[test]
    fn stop_takes_back_omps_text_and_each_accepted_message_once() {
        let gate = Arc::new(bare_gate());
        gate.observe_run(&RouterEvent::Queue {
            steering: vec!["/review shown".to_string()],
            follow_up: Vec::new(),
        });
        gate.accepted("/review typed", true);
        gate.accepted("later", false);
        gate.observe_run(&RouterEvent::Queue {
            steering: vec!["/review shown".to_string()],
            follow_up: vec!["later".to_string()],
        });
        // OMP 18.6.1 removeQueuedMessage matches the raw text first, then
        // the content queue_update shows (omp.strings ~593938).
        let (calls, taken) = removes(&gate, |text| Ok(text != "/review shown"));
        assert_eq!(
            calls,
            [
                steer("/review typed"),
                steer("/review shown"),
                ("later".to_string(), false)
            ],
            "the typed text first; the display text after it answers false as its twin"
        );
        assert_eq!(taken.to_composer(), ["/review typed", "later"]);
        assert_eq!(taken.notice(), None, "nothing was taken into a run");
    }

    #[test]
    fn a_listed_message_ahead_of_a_display_text_is_still_taken_back() {
        let gate = Arc::new(bare_gate());
        gate.accepted("/review typed", true);
        gate.observe_run(&RouterEvent::Queue {
            steering: vec!["late".to_string(), "/review shown".to_string()],
            follow_up: Vec::new(),
        });
        let (calls, taken) = removes(&gate, |text| Ok(text != "/review shown"));
        assert_eq!(
            calls,
            [
                steer("/review typed"),
                steer("late"),
                steer("/review shown")
            ]
        );
        assert_eq!(taken.to_composer(), ["/review typed", "late"]);
        assert_eq!(
            taken.notice(),
            None,
            "the display text was the typed message, not one taken into a run"
        );
    }

    #[test]
    fn a_listed_message_nothing_accepted_names_is_taken_back_by_its_text() {
        let gate = Arc::new(bare_gate());
        gate.observe_run(&RouterEvent::Queue {
            steering: vec!["/review shown".to_string()],
            follow_up: Vec::new(),
        });
        gate.accepted("/review typed", true);
        let (calls, taken) = removes(&gate, |text| Ok(text == "/review shown"));
        assert_eq!(calls, [steer("/review typed"), steer("/review shown")]);
        assert_eq!(taken.to_composer(), ["/review shown"]);
    }

    #[test]
    fn a_dead_link_is_asked_once() {
        let gate = Arc::new(bare_gate());
        gate.accepted("a", true);
        gate.accepted("b", true);
        gate.accepted("c", false);
        let (calls, taken) = removes(&gate, |_| Err("timed out".to_string()));
        assert_eq!(calls, [steer("a")], "one timeout, not one per message");
        assert_eq!(taken.to_composer(), ["a", "b", "c"]);
    }

    #[test]
    fn the_composer_gets_the_messages_oldest_first() {
        let gate = Arc::new(bare_gate());
        gate.accepted("a", true);
        gate.accepted("b", true);
        gate.accepted("c", true);
        let (_, taken) = removes(&gate, |text| {
            if text == "b" {
                Err("timed out".to_string())
            } else {
                Ok(true)
            }
        });
        assert_eq!(taken.to_composer(), ["a", "b", "c"]);
    }

    #[test]
    fn a_listed_message_omp_kept_was_delivered_into_the_stopped_turn() {
        let gate = Arc::new(bare_gate());
        gate.observe_run(&RouterEvent::Queue {
            steering: vec!["faster".to_string()],
            follow_up: vec!["two".to_string()],
        });
        let queue = gate.queue.lock().clone();
        let taken = take_back(&queue, |text, _| Ok(text == "two"));
        assert_eq!(
            (taken.restored, taken.gone),
            (vec!["two".to_string()], vec!["faster".to_string()])
        );
    }

    #[test]
    fn an_accepted_message_omp_no_longer_holds_is_named() {
        let gate = Arc::new(bare_gate());
        gate.accepted("kept", true);
        let queue = gate.queue.lock().clone();
        let taken = take_back(&queue, |_, _| Ok(false));
        assert_eq!(
            taken.gone,
            ["kept"],
            "OMP drained it before its queue_update was read"
        );
        assert!(
            taken
                .notice()
                .unwrap()
                .contains("already taken these into a run")
        );
    }

    #[test]
    fn a_remove_that_errors_is_not_taken_back_and_keeps_the_text() {
        let gate = Arc::new(bare_gate());
        gate.accepted("faster", true);
        gate.accepted("two", false);
        let queue = gate.queue.lock().clone();
        let taken = take_back(&queue, |text, _| {
            if text == "two" {
                Err("timed out".to_string())
            } else {
                Ok(true)
            }
        });
        assert_eq!(
            taken.gone,
            Vec::<String>::new(),
            "not delivered: OMP never answered"
        );
        assert_eq!(taken.to_composer(), ["faster", "two"]);
        let notice = taken.notice().unwrap();
        assert!(
            notice.contains("did not answer")
                && notice.contains("two")
                && notice.contains("timed out"),
            "{notice}"
        );
    }

    #[test]
    fn a_lost_agent_start_of_ours_does_not_hide_a_later_own_run() {
        let gate = Arc::new(bare_gate());
        gate.turn.lock().phase = Phase::Sent;
        gate.prompt_ended(true);
        gate.observe_run(&RouterEvent::Settled);
        gate.observe_run(&RouterEvent::AgentStart);
        assert!(
            gate.turn.lock().unprompted,
            "after Settled a run is OMP's own"
        );
    }

    #[test]
    fn our_run_read_after_its_prompt_returned_is_still_ours() {
        let gate = Arc::new(bare_gate());
        gate.turn.lock().phase = Phase::Sent;
        gate.prompt_ended(true);
        gate.observe_run(&RouterEvent::AgentStart);
        assert!(!gate.turn.lock().unprompted, "our own run read late");
        gate.observe_run(&RouterEvent::AgentStart);
        assert!(gate.turn.lock().unprompted, "the next one is OMP's");
    }

    #[test]
    fn a_promote_after_stop_is_not_sent() {
        let gate = Arc::new(bare_gate());
        gate.accepted("later", false);
        let epoch = gate.epoch();
        gate.cancel(false);
        let mut sent = false;
        let result = gate.promote(epoch, "later", |_| {
            sent = true;
            Ok(true)
        });
        assert!(!sent, "Stop came between the click and the call");
        assert!(result.is_err(), "{result:?}");
        let (calls, _) = removes(&gate, |_| Ok(true));
        assert_eq!(calls, [("later".to_string(), false)], "still a follow-up");
    }

    #[test]
    fn a_promoted_follow_up_is_taken_back_as_a_steer_and_a_refused_one_is_not() {
        let gate = Arc::new(bare_gate());
        gate.accepted("later", false);
        gate.accepted("after", false);
        assert_eq!(gate.promote(gate.epoch(), "later", |_| Ok(true)), Ok(true));
        assert_eq!(
            gate.promote(gate.epoch(), "after", |_| Ok(false)),
            Ok(false)
        );
        let (calls, _) = removes(&gate, |_| Ok(true));
        assert_eq!(calls, [steer("later"), ("after".to_string(), false)]);
    }

    #[test]
    fn a_queue_call_after_stop_is_refused() {
        let gate = Arc::new(bare_gate());
        let epoch = gate.epoch();
        gate.cancel(false);
        assert!(gate.admit(epoch).is_none(), "Stop was pressed first");
        assert!(gate.admit(gate.epoch()).is_some());
    }

    #[test]
    fn a_cancel_that_never_reached_omp_is_still_an_audit_row() {
        let dir = temp("cancel-off");
        let (events, mut rx) = futures::channel::mpsc::unbounded();
        let link = OmpLink {
            commands: mpsc::channel().0,
            pid: Arc::default(),
            gate: Arc::new(Gate {
                control: Mutex::default(),
                turn: Mutex::default(),
                state: Mutex::new(gate(&dir)),
                queue: Mutex::default(),
                sending: Mutex::default(),
                stops: AtomicU64::default(),
                events,
            }),
            thread: None,
        };
        link.cancel_subagent("sa-1".to_string());
        let event = futures::executor::block_on(rx.next());
        assert!(
            matches!(
                event,
                Some(LinkEvent::SubagentCancelled { ref id, result: Err(_) }) if id == "sa-1"
            ),
            "{event:?}"
        );
        let rows: Vec<serde_json::Value> =
            std::fs::read_to_string(dir.join(cedian_shell::audit::AUDIT_FILE))
                .unwrap_or_default()
                .lines()
                .map(|l| serde_json::from_str::<serde_json::Value>(l).unwrap()["item"].clone())
                .filter(|item| item["tool"] == "cancel_subagent")
                .collect();
        assert_eq!(rows.len(), 1, "{rows:?}");
        assert_eq!(rows[0]["command"], "sa-1");
        assert_eq!(rows[0]["error"], "OMP is not running");
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn ready(
        events: &mut futures::channel::mpsc::UnboundedReceiver<LinkEvent>,
    ) -> (String, bool, Option<String>) {
        futures::executor::block_on(async {
            while let Some(event) = events.next().await {
                match event {
                    LinkEvent::Ready {
                        session_id,
                        resumed,
                        session_file,
                        ..
                    } => return (session_id, resumed, session_file),
                    LinkEvent::Failed(e) => panic!("{e}"),
                    _ => {}
                }
            }
            panic!("OMP thread ended without a session")
        })
    }

    /// Real OMP keeps the app's session in its own store, where the CLI
    /// finds it (ADR-0040 decision 5), and adopts it on restart. OMP writes a
    /// session only once it has a message, so this sends one short prompt
    /// (one model call). The session folder it made is removed after.
    /// `cargo test -p cedian_panel -- --ignored live_`
    #[test]
    #[ignore]
    fn live_restart_adopts_the_same_omp_session() {
        let root = std::env::temp_dir().join(format!("cedian-u3-live-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("ws")).unwrap();
        let root = root.canonicalize().unwrap();
        let spec = || LaunchSpec {
            binary: Some(cedian_shell::launch::omp_binary().unwrap().binary),
            workdir: root.join("ws"),
            state_dir: root.join("state"),
            sessions: Sessions::OmpDefault,
            policy: SpawnPolicy::default(),
            policy_source: Policy::Cedian,
            omp_state: cedian_omp::driver::state_root(),
            uris: Vec::new(),
            tools: Vec::new(),
            router: Arc::default(),
            browser: Arc::default(),
            workflow: cedian_workflow::WorkflowChannel::new(
                "live",
                Box::new(cedian_shell::workflow_store::DiskWorkflowStore(
                    root.join("state"),
                )),
                |_, _| None,
                cedian_workflow::CurrentState::default,
            ),
            review: Arc::default(),
            settings: cedian_shell::Settings::default(),
            unsaved: Unsaved::default(),
        };
        let (tx, mut rx) = futures::channel::mpsc::unbounded();
        let link = OmpLink::start(spec(), tx, None);
        let (first, resumed, file) = ready(&mut rx);
        assert!(!resumed);
        let file = PathBuf::from(file.expect("OMP names the session file"));
        let agent = cedian_omp::OmpConfig::new(spec().binary.unwrap())
            .dir(&root)
            .unwrap();
        assert!(
            file.starts_with(agent.join("sessions")),
            "in OMP's own store: {}",
            file.display()
        );
        link.send(Prompt {
            text: "Reply with exactly: ok".to_string(),
            images: Vec::new(),
        })
        .unwrap();
        futures::executor::block_on(async {
            while let Some(event) = rx.next().await {
                if matches!(event, LinkEvent::Event(RouterEvent::Settled)) {
                    break;
                }
            }
        });
        let (tx, mut rx) = futures::channel::mpsc::unbounded();
        let _link = OmpLink::start(spec(), tx, Some(link));
        let (again, resumed, _) = ready(&mut rx);
        assert_eq!((again.as_str(), resumed), (first.as_str(), true));
        let _ = std::fs::remove_dir_all(file.parent().unwrap());
        let _ = std::fs::remove_dir_all(&root);
    }
}
