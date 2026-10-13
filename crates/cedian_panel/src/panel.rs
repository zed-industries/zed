//! The cedian dock panel (S9a T2/T4, S9 U3): one prompt box, the streamed OMP
//! reply rendered from the headless `cedian_agent::Thread`, and agent-edit
//! import.
//!
//! The panel starts OMP when it opens on a folder ([`crate::omp_link`]): its
//! own thread, the user's `cedian.toml`, the spawn profile. Events cross into
//! GPUI over a channel; edit-class tool events drive [`crate::import`]. When
//! OMP dies the panel says why and offers Restart; the IDE keeps running.
//!
//! OMP's dialogs (approvals, `confirm`, `input`, `editor`, `ask`) are answered
//! here (U4, §63): several can be open at once, keyed by request id. A turn
//! can be stopped, and images pasted into the composer go with the prompt.
//!
//! Every imported agent transaction feeds the task's [`TaskReview`] (U5,
//! §18, ADR-0006). A file OMP is about to write that is not open is opened
//! first, so its write is a reviewable transaction too. The Review Changes
//! view lists each file's hunks with Accept and Reject, Accept all, and
//! Revert turn.

use crate::browser::BrowserHost;
use crate::context;
use crate::dialogs::OpenDialog;
use crate::import::{self, ImportOutcome, Mark};
use crate::omp_link::{
    AnswerError, LaunchSpec, LinkEvent, OmpLink, Prompt, ReviewReader, WorktreeAsk,
};
use crate::omp_settings::OmpSettings;
use crate::review::{ReviewError, TaskReview};
use crate::worker_trees::{self, WorkerTreeRow};
use crate::workflow_view::WorkflowView;
use cedian_agent::{Thread, ThreadEvent};
use cedian_agent_ui::{SubagentRow, SubagentTree, ToolCard};
use cedian_omp::{RouterEvent, SubagentStatus, TextBefore, UserAnswer};
use cedian_review::{AttachedFinding, FindingSeverity, HunkKey, HunkStatus};
use cedian_worker::{Brief, Registry, WorkerScope, WorkerStatus};
use cedian_workflow::{
    CurrentState, Evidence, EvidenceKind, Gate, GateResult, Provenance, WorkflowChannel,
    WorkflowStatus,
};
use cedian_workspace::{capture_ambient, render_snapshot};
use collections::{HashMap, HashSet, IndexMap};
use editor::display_map::{BlockPlacement, BlockProperties, BlockStyle, CustomBlockId};
use editor::{Editor, RowHighlightOptions, actions::Paste};
use futures::{StreamExt, channel::mpsc};
use gpui::{
    Action, AnyElement, App, AsyncWindowContext, ClipboardEntry, Context, ElementId, Entity,
    EntityId, EventEmitter, FocusHandle, Focusable, Pixels, Render, Subscription, Task, WeakEntity,
    Window, actions, px,
};
use language::{Buffer, BufferEvent};
use omp_rpc::{ExtensionUiRequest, ImageContent};

/// The gate every panel browser capture is evidence for.
pub const BROWSER_GATE: &str = "browser";
use project::Project;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use ui::{Button, Label, prelude::*};
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

actions!(
    cedian_panel,
    [
        /// Toggle focus on the cedian panel.
        ToggleFocus,
        /// Revert the latest agent turn (ADR-0056).
        RevertTurn,
        /// Ask OMP to edit the selection (ADR-0056).
        OpenInlineEdit,
    ]
);

const PANEL_KEY: &str = "CedianPanel";
/// OMP tools that write the filesystem directly.
const EDIT_TOOLS: &[&str] = &["edit", "write", "ast_edit"];
/// The worker worktrees are classified and merged against what is checked out
/// (ADR-0033); the registry stores relative branches, not a rebase target.
const WORKER_BASE: &str = "HEAD";

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<CedianPanel>(window, cx);
        });
        workspace.register_action(|workspace, _: &RevertTurn, _, cx| {
            // Deferred: the revert reads the workspace to redraw finding blocks.
            if let Some(panel) = workspace.panel::<CedianPanel>(cx) {
                cx.defer(move |cx| panel.update(cx, |panel, cx| panel.revert_latest_turn(cx)));
            }
        });
        workspace.register_action(|workspace, _: &OpenInlineEdit, window, cx| {
            let panel = workspace.panel::<CedianPanel>(cx);
            let editor = workspace.active_item_as::<Editor>(cx);
            if let (Some(panel), Some(editor)) = (panel, editor) {
                panel.update(cx, |panel, cx| panel.open_instruction(editor, window, cx));
            }
        });
    })
    .detach();
}

/// An inline edit asked for in the editor (ADR-0056): the selection, and
/// what to do with it.
#[derive(Clone, Debug)]
pub struct InlineEdit {
    /// The file as OMP names it (worktree relative).
    pub path: String,
    pub buffer: Entity<Buffer>,
    /// The selection, anchored so it follows later edits.
    pub range: std::ops::Range<text::Anchor>,
    pub instruction: String,
    /// The selection's end, kept before text inserted there: lines added
    /// right after the selection are outside it.
    end_before: text::Anchor,
    /// The selection's start, kept after text inserted there: lines added
    /// right before the selection are outside it.
    start_after: text::Anchor,
}

impl InlineEdit {
    pub fn new(
        path: String,
        buffer: Entity<Buffer>,
        selection: std::ops::Range<language::Point>,
        cx: &App,
    ) -> Self {
        let (range, end_before, start_after) = buffer.read_with(cx, |b, _| {
            (
                b.anchor_before(selection.start)..b.anchor_after(selection.end),
                b.anchor_before(selection.end),
                b.anchor_after(selection.start),
            )
        });
        Self {
            path,
            buffer,
            range,
            instruction: String::new(),
            end_before,
            start_after,
        }
    }

    /// 1-based first and last line of the selection.
    pub fn lines(&self, cx: &App) -> (u32, u32) {
        use text::ToPoint as _;
        let buffer = self.buffer.read(cx);
        let start = self.range.start.to_point(buffer);
        let mut end = self.range.end.to_point(buffer);
        if end.column == 0 && end.row > start.row {
            end.row -= 1;
        }
        (start.row + 1, end.row + 1)
    }

    /// The prompt OMP gets: cedian's own words, short, naming the selection.
    pub fn message(&self, cx: &App) -> String {
        let (first, last) = self.lines(cx);
        let selected: String = self
            .buffer
            .read(cx)
            .text_for_range(self.range.clone())
            .collect();
        format!(
            "Inline edit of {} lines {first}-{last}. Change only the selected text, with your edit tool.\nSelected:\n{}\nInstruction: {}",
            self.path,
            selected.trim_end_matches('\n'),
            self.instruction
        )
    }

    /// `hunk` only adds lines right before or after the selection, which
    /// the selection's outward-biased ends would otherwise take in.
    fn added_outside(&self, hunk: &crate::review::ReviewHunk, cx: &App) -> bool {
        use text::ToOffset as _;
        let buffer = self.buffer.read(cx);
        let start = self.range.start.to_offset(buffer);
        let start_after = self.start_after.to_offset(buffer);
        let end_before = self.end_before.to_offset(buffer);
        hunk.old.is_empty()
            && end_before > start
            && (hunk.new.start >= end_before || hunk.new.end <= start_after)
    }

    /// How the review names the turn.
    pub fn label(&self, cx: &App) -> String {
        let (first, last) = self.lines(cx);
        format!("{}:{first}-{last} {}", self.path, self.instruction)
    }
}

/// A follow-up the panel queued, until OMP runs it: each is its own turn.
struct QueuedFollowUp {
    message: String,
    /// An inline edit's follow-up: its turn is an edit turn.
    edit: Option<InlineEdit>,
    /// The text OMP's queue lists it under, once listed: a slash command
    /// or template shows expanded.
    shown: Option<String>,
    /// Steer now was clicked and OMP has not answered: leaving the queue
    /// for the steering one is not a drain.
    promoting: bool,
}

/// One edit-class tool call in flight: the open buffers marked before its
/// write, and the files it names that are not open, with their disk text
/// at the tool's start (`None`: absent then). Those are opened only at the
/// tool's end, so the write is a hunk against that text however the open
/// and the write race (a new file is one all-added hunk).
struct CallMarks {
    marks: Vec<(Entity<Buffer>, Mark)>,
    unopened: Vec<(PathBuf, Option<String>)>,
    /// Disk reads of unopened files still in flight.
    reading: usize,
    /// `ToolEnd` came (with its `is_error`) before every read finished.
    ended: Option<bool>,
    /// Each file's text before the call as OMP's result reports it, by
    /// absolute path.
    before: Vec<(PathBuf, TextBefore)>,
    /// A `bash` call (ADR-0047): the files not open that changed while it
    /// ran, `(absolute, as the review names it)`.
    bash: Option<Vec<(PathBuf, PathBuf)>>,
    /// Marked buffers another mutating call also marked while one of the
    /// two was a `bash` call, with that call's id: which call wrote what is
    /// unknown, so neither imports them.
    overlapped: Vec<(Entity<Buffer>, String)>,
}

/// The tool whose writes cedian learns only from the files it changed.
const BASH_TOOL: &str = "bash";
/// OMP's browser tool: the page at its end is gate evidence (ADR-0055).
const BROWSER_TOOL: &str = "browser";

/// cedian's instruction block in an editor (ADR-0056 decision 2): the
/// selection it edits and the line the person types the instruction in.
struct InstructionBlock {
    host: WeakEntity<Editor>,
    block: CustomBlockId,
    input: Entity<Editor>,
    edit: InlineEdit,
    /// The message sent as a follow-up: the block stays until OMP queues it.
    pending: Option<String>,
}

/// One finding's block in one editor (ADR-0055 decision 4): never a
/// diagnostic, which would reach OMP's prompt through the U6 context.
#[derive(Clone)]
struct FindingBlock {
    editor: WeakEntity<Editor>,
    block: CustomBlockId,
    finding: String,
    rows: std::ops::Range<u32>,
}

/// The row highlight of a hunk an open finding is on.
enum FindingRows {}

/// How long the reviewer waits for the app to read the task's review.
const REVIEW_READ_TIMEOUT: Duration = Duration::from_secs(30);

/// How long a dialog waits on the person before cedian closes it (§63,
/// ADR-0013).
pub const DIALOG_TIMEOUT: Duration = Duration::from_secs(5 * 60);

/// The running turn, as the panel shows it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Turn {
    Idle,
    /// Sent; OMP has not started it yet.
    Queued,
    Streaming,
    /// Stop was pressed; OMP has not settled yet.
    Stopping,
    /// The turn failed and was aborted; OMP has not settled yet.
    Failed(String),
}

impl Turn {
    fn label(&self) -> String {
        match self {
            Turn::Idle => "idle".to_string(),
            Turn::Queued => "queued".to_string(),
            Turn::Streaming => "streaming".to_string(),
            Turn::Stopping => "stopping".to_string(),
            Turn::Failed(reason) => format!("failed: {reason}"),
        }
    }
}

/// Where the panel's OMP stands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Connection {
    /// No folder open yet, so no OMP.
    NotStarted,
    Starting,
    /// Retry asked OMP's thread to check the session for another driver
    /// again; `Ready` or `Taken` follows.
    Checking,
    Ready {
        session_id: String,
        resumed: bool,
        policy_note: Option<String>,
    },
    /// OMP failed to start or died. Prompts are refused until Restart.
    Stopped(String),
    /// Another process drives the session (ADR-0040 decision 5). Prompts
    /// are refused until Retry finds it free or a new session starts.
    Taken {
        session_id: String,
        reason: String,
    },
}

/// The app's task: its review, its workflow evidence and its correction rows.
pub const TASK_ID: &str = "panel";

pub struct CedianPanel {
    focus_handle: FocusHandle,
    project: Entity<Project>,
    input: Entity<Editor>,
    thread: Thread,
    /// OMP's subagents, each under the `task` call that started it.
    subagents: SubagentTree,
    /// Each running subagent's Steer text box, by id.
    steer_boxes: HashMap<String, Entity<Editor>>,
    /// What the last Steer or Cancel on a subagent row came to, by id.
    subagent_notes: HashMap<String, String>,
    link: Option<OmpLink>,
    connection: Connection,
    calls: HashMap<String, CallMarks>,
    review: TaskReview,
    /// The reviewed buffers whose edits rebuild the review (so render only
    /// shows what is there).
    watched: HashSet<EntityId>,
    buffer_subscriptions: Vec<Subscription>,
    show_review: bool,
    /// Accept all is waiting on whether to include the STALE hunks.
    confirm_accept_all: bool,
    turn: Turn,
    /// The last thing the person should know that is not on a dialog: a
    /// refused prompt, a failed turn, a dialog cedian closed.
    notice: Option<String>,
    /// Compaction, retry and session-setting notes, oldest first, shown in
    /// the thread (ADR-0057 decision 3).
    session_notes: Vec<String>,
    /// OMP waits to retry a failed request: Stop retry is offered.
    retrying: bool,
    /// The model and thinking-level picker (ADR-0057 decision 3).
    picker: ModelPicker,
    onboarding: Option<Onboarding>,
    /// OMP's notices, oldest first, until dismissed (ADR-0057 decision 3),
    /// each with the id its Dismiss button names.
    toasts: Vec<(u64, cedian_omp::Toast)>,
    next_toast: u64,
    /// The OMP the panel runs is not the pinned one (ADR-0057 decision 4).
    omp_warning: Option<String>,
    /// The benchmark's timing rows and frame durations (ADR-0058).
    instruments: crate::instruments::Instruments,
    /// The `policy = "omp"` badge (ADR-0035 decision 4), shown while the
    /// launch runs under it.
    omp_policy: Option<String>,
    /// The correction ledger's last failed write: one line, however many
    /// rows failed, so a ledger that keeps failing does not grow the notice.
    ledger_error: Option<String>,
    /// OMP's dialogs waiting on the person, by request id, oldest first.
    dialogs: IndexMap<String, OpenDialog>,
    /// Images pasted into the composer, sent with the next prompt.
    images: Vec<ImageContent>,
    /// The OMP settings page, shown instead of the thread when open.
    settings: Option<Entity<OmpSettings>>,
    show_settings: bool,
    /// The workspace whose active editor gives the selection OMP sees.
    workspace: Option<WeakEntity<Workspace>>,
    _events: Option<Task<()>>,
    /// The workspace's Chromium, shared with OMP (ADR-0049); closed with
    /// the panel.
    browser: Option<Arc<BrowserHost>>,
    _browser_events: Option<Task<()>>,
    _browser_quit: Option<Subscription>,
    /// Evidence from the panel's browser captures, each bound to its frame.
    browser_evidence: Vec<Evidence>,
    /// Answers OMP's `cedian://` reads.
    _context: Option<Task<()>>,
    /// The workspace's state dir (ADR-0044), once known.
    state_dir: Option<PathBuf>,
    /// The task's workflow as last read, when one was started.
    workflow: Option<WorkflowView>,
    /// The channel OMP's workflow tools call, once OMP was started.
    workflow_channel: Option<Arc<WorkflowChannel>>,
    _workflow_load: Option<Task<()>>,
    /// The workspace scans the workflow view has started.
    workflow_reads: usize,
    /// The §54 escalation the blocked workflow waits on the person for.
    escalation: Option<String>,
    _worktree_changes: Option<Subscription>,
    /// Open buffers with unsaved edits, which evidence binds to; shared
    /// with the running launch's workflow channel.
    unsaved: crate::omp_link::Unsaved,
    _unsaved_watch: Vec<Subscription>,
    /// The reviewer's reader of the task's review: it crosses to this
    /// thread and back (ADR-0055).
    review_reader: ReviewReader,
    _review_reads: Task<()>,
    /// The reviewer's findings as last read from the state dir.
    findings: Vec<AttachedFinding>,
    /// The finding whose Dismiss is open, with its reason box.
    dismissing: Option<(String, Entity<Editor>)>,
    /// The runtime's router once OMP runs: the models that implemented.
    router: Option<Arc<std::sync::OnceLock<Arc<cedian_omp::EventRouter>>>>,
    _review_run: Option<Task<()>>,
    /// The settings OMP was launched with: the person's reviews use them too.
    launch_settings: Option<cedian_shell::Settings>,
    /// New editors get the open findings' blocks at once.
    _workspace_items: Option<gpui::Subscription>,
    /// The blocks open findings put under their hunk in open editors.
    finding_blocks: Vec<FindingBlock>,
    instruction: Option<InstructionBlock>,
    /// Follow-ups sent from the panel, oldest first, until OMP runs them.
    follow_ups: Vec<QueuedFollowUp>,
    /// The inline edit the review's current turn is running, with its turn.
    edit_turn: Option<(u32, InlineEdit)>,
    /// The cedian worktrees git sees, with their registry rows; `None` until
    /// the first read, so the surface changes nothing before it.
    worker_trees: Option<Vec<WorkerTreeRow>>,
    /// The worker whose Remove is waiting on a second click (§43).
    confirm_remove: Option<String>,
    /// A worktree request waiting on the person: (request, id, goal).
    worktree_ask: Option<(u64, String, String)>,
    /// The running launch's worktree gate, which answers its requests.
    worktree_gate: Option<Arc<WorktreeAsk>>,
    /// The lease on the pending worktree request (ADR-0013).
    _worktree_expiry: Option<Task<()>>,
}

/// The model and thinking-level picker: OMP's choices once read, and what
/// the session runs.
#[derive(Debug, Clone, Default)]
pub struct ModelPicker {
    pub open: bool,
    pub models: Vec<omp_rpc::ModelInfo>,
    pub levels: Vec<omp_rpc::ThinkingLevel>,
    /// `provider/id`.
    pub model: Option<String>,
    pub thinking: Option<omp_rpc::ThinkingLevel>,
    /// OMP's refusal of the last read or change.
    pub error: Option<String>,
}

/// Onboarding, while OMP's session has no model: OMP's login providers
/// and the login the person runs (ADR-0057 decision 3).
#[derive(Debug, Clone, Default)]
pub struct Onboarding {
    pub providers: Vec<omp_rpc::LoginProvider>,
    /// The provider whose login runs now.
    pub running: Option<String>,
    /// OMP's sign-in page for the running login, kept as a link.
    pub url: Option<omp_rpc::OpenUrlUiRequest>,
    /// The dialogs OMP opened during the running login (its code prompt).
    pub dialogs: Vec<String>,
    /// A login finished and the picker is being re-read: when OMP still has
    /// no model, the providers are read again.
    pub logged_in: bool,
    /// OMP's refusal of the provider list or of the last login.
    pub error: Option<String>,
}

/// Whether the system browser may open a login's page: `https`, or
/// `http` to this machine (OMP's `launchUrl` loopback redirect). Any other
/// scheme is refused, named.
fn login_link(link: &str) -> Result<String, String> {
    let parsed = url::Url::parse(link).map_err(|e| format!("not a URL ({e}): {link}"))?;
    let local = matches!(parsed.host_str(), Some("localhost" | "127.0.0.1" | "[::1]"));
    match parsed.scheme() {
        "https" => Ok(link.to_string()),
        "http" if local => Ok(link.to_string()),
        scheme => Err(format!("refused to open a {scheme}: page: {link}")),
    }
}

/// How long typing in a file the shown evidence covers pauses before the
/// workflow view re-reads the workspace.
const UNSAVED_DEBOUNCE: Duration = Duration::from_millis(300);

/// The toasts kept: older ones fall off.
const MAX_TOASTS: usize = 8;
/// The session notes kept: older ones fall off.
const MAX_SESSION_NOTES: usize = 32;

/// How long a quitting app waits for Chromium to close itself (flushing its
/// cookies) before killing it. The quit hook blocks the main thread.
const BROWSER_QUIT_LIMIT: Duration = Duration::from_secs(2);

impl CedianPanel {
    pub fn load(
        workspace: WeakEntity<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<anyhow::Result<Entity<Self>>> {
        cx.spawn(async move |cx| {
            workspace.update_in(cx, |workspace, window, cx| {
                let project = workspace.project().clone();
                let handle = cx.entity().downgrade();
                cx.new(|cx| {
                    let mut panel = Self::new(project, window, cx);
                    panel.set_workspace(handle, cx);
                    panel
                })
            })
        })
    }

    pub fn new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Ask OMP…", window, cx);
            editor
        });
        let (reads, mut read_rx) =
            mpsc::unbounded::<std::sync::mpsc::Sender<cedian_shell::review_agent::ReviewTask>>();
        let review_reader: ReviewReader = Arc::new(move || {
            let (reply, answer) = std::sync::mpsc::channel();
            reads
                .unbounded_send(reply)
                .map_err(|_| "the cedian panel is closed".to_string())?;
            answer
                .recv_timeout(REVIEW_READ_TIMEOUT)
                .map_err(|_| "no answer from the app for the task's review".to_string())
        });
        let _review_reads = cx.spawn(async move |this, cx| {
            while let Some(reply) = read_rx.next().await {
                let Ok(task) = this.update(cx, |this, cx| {
                    this.review.rebuild(cx);
                    this.after_review_change(cx);
                    this.review.review_task(cx)
                }) else {
                    break;
                };
                let _ = reply.send(task);
            }
        });
        let mut this = Self {
            focus_handle: cx.focus_handle(),
            project,
            input,
            thread: Thread::new(),
            subagents: SubagentTree::default(),
            steer_boxes: HashMap::default(),
            subagent_notes: HashMap::default(),
            link: None,
            connection: Connection::NotStarted,
            omp_warning: None,
            instruments: Default::default(),
            omp_policy: None,
            toasts: Vec::new(),
            next_toast: 0,
            picker: ModelPicker::default(),
            onboarding: None,
            session_notes: Vec::new(),
            retrying: false,
            calls: HashMap::default(),
            review: TaskReview::new(TASK_ID),
            watched: HashSet::default(),
            buffer_subscriptions: Vec::new(),
            show_review: false,
            confirm_accept_all: false,
            turn: Turn::Idle,
            notice: None,
            ledger_error: None,
            dialogs: IndexMap::default(),
            images: Vec::new(),
            settings: None,
            show_settings: false,
            state_dir: None,
            workflow: None,
            _workflow_load: None,
            workflow_reads: 0,
            workflow_channel: None,
            escalation: None,
            workspace: None,
            _events: None,
            browser: None,
            _browser_events: None,
            _browser_quit: None,
            browser_evidence: Vec::new(),
            _context: None,
            _worktree_changes: None,
            unsaved: Default::default(),
            _unsaved_watch: Vec::new(),
            review_reader,
            _review_reads,
            findings: Vec::new(),
            dismissing: None,
            router: None,
            _review_run: None,
            launch_settings: None,
            _workspace_items: None,
            finding_blocks: Vec::new(),
            follow_ups: Vec::new(),
            instruction: None,
            edit_turn: None,
            worker_trees: None,
            confirm_remove: None,
            worktree_ask: None,
            worktree_gate: None,
            _worktree_expiry: None,
        };
        this._worktree_changes = Some(cx.subscribe(&this.project, |this, project, event, cx| {
            if let project::Event::WorktreeUpdatedEntries(worktree, changes) = event {
                this.bash_changed(&project, *worktree, changes, cx);
            }
        }));
        let buffer_store = this.project.read(cx).buffer_store().clone();
        this._unsaved_watch
            .push(cx.subscribe(&buffer_store, |this, _, event, cx| {
                if let project::buffer_store::BufferStoreEvent::BufferAdded(buffer) = event {
                    this.watch_unsaved(buffer.clone(), cx);
                }
            }));
        let open: Vec<Entity<Buffer>> = buffer_store.read(cx).buffers().collect();
        for buffer in open {
            this.watch_unsaved(buffer, cx);
        }
        if this.workspace_root(cx).is_some() {
            this.start(window, cx);
            this.refresh_worker_trees(cx);
        }
        this
    }

    fn watch_unsaved(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        self.note_unsaved(&buffer, cx);
        self._unsaved_watch
            .push(cx.subscribe(&buffer, |this, buffer, event, cx| {
                if matches!(
                    event,
                    BufferEvent::Edited { .. }
                        | BufferEvent::DirtyChanged
                        | BufferEvent::Saved
                        | BufferEvent::Reloaded
                        | BufferEvent::FileHandleChanged
                ) && let Some(path) = this.note_unsaved(&buffer, cx)
                    && this
                        .workflow
                        .as_ref()
                        .is_some_and(|w| w.covers.covers(&path))
                {
                    this.load_workflow(UNSAVED_DEBOUNCE, cx);
                }
            }));
    }

    /// Record `buffer`'s version while it has unsaved edits (ADR-0057
    /// decision 7); a saved buffer is the disk again. Its workspace path,
    /// when that changed.
    fn note_unsaved(&mut self, buffer: &Entity<Buffer>, cx: &App) -> Option<String> {
        let (Some(root), Some(file)) = (
            self.workspace_root(cx),
            project::File::from_dyn(buffer.read(cx).file()),
        ) else {
            return None;
        };
        // A symlinked root or a buffer opened through its target: both
        // sides resolved, the key is the path the disk scan uses.
        let canonical = |path: PathBuf| path.canonicalize().unwrap_or(path);
        let abs = canonical(language::LocalFile::abs_path(file, cx));
        let rel = abs.strip_prefix(canonical(root)).ok()?;
        let rel = rel.to_string_lossy().into_owned();
        let buffer = buffer.read(cx);
        let mut unsaved = self.unsaved.lock();
        let changed = if buffer.is_dirty() {
            let version = format!("unsaved {:?}", buffer.version());
            unsaved.insert(rel.clone(), version.clone()) != Some(version)
        } else {
            unsaved.remove(&rel).is_some()
        };
        changed.then_some(rel)
    }

    /// The workspace whose active editor and selection OMP is told about.
    pub fn set_workspace(&mut self, workspace: WeakEntity<Workspace>, cx: &mut Context<Self>) {
        self._workspace_items = workspace.upgrade().map(|w| {
            cx.subscribe(&w, |this, _, event: &workspace::Event, cx| {
                if matches!(event, workspace::Event::ItemAdded { .. }) {
                    this.sync_finding_blocks(cx);
                }
            })
        });
        self.workspace = Some(workspace);
    }

    pub fn workflow_reads(&self) -> usize {
        self.workflow_reads
    }

    pub fn session_notes(&self) -> &[String] {
        &self.session_notes
    }

    pub fn retrying(&self) -> bool {
        self.retrying
    }

    /// The follow-ups the panel queued that are still waiting, oldest first.
    pub fn queued_follow_ups(&self) -> Vec<&str> {
        self.follow_ups.iter().map(|q| q.message.as_str()).collect()
    }

    /// Change a setting of the live session.
    pub fn set_session(
        &mut self,
        setting: crate::omp_link::SessionSetting,
        cx: &mut Context<Self>,
    ) {
        match &self.link {
            Some(link) => link.set(setting),
            None => self.notice = Some("OMP is not running".to_string()),
        }
        cx.notify();
    }

    /// "Steer now" on queued follow-up `index`: OMP moves it to the steering
    /// queue, so it joins the running turn and has no review turn of its
    /// own. An inline edit keeps its own turn, so it is not offered.
    pub fn promote_follow_up(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.link.is_none() {
            return;
        }
        if let Some(text) = self.begin_promote(index)
            && let Some(link) = &self.link
        {
            link.promote(text);
        }
        cx.notify();
    }

    fn push_note(&mut self, note: String) {
        self.session_notes.push(note);
        let excess = self.session_notes.len().saturating_sub(MAX_SESSION_NOTES);
        self.session_notes.drain(..excess);
    }

    /// Mark follow-up `index` as asked to steer; its text, to send.
    fn begin_promote(&mut self, index: usize) -> Option<String> {
        let queued = self
            .follow_ups
            .get_mut(index)
            .filter(|q| q.edit.is_none() && !q.promoting)?;
        queued.promoting = true;
        Some(queued.message.clone())
    }

    pub fn picker(&self) -> &ModelPicker {
        &self.picker
    }

    pub fn onboarding(&self) -> Option<&Onboarding> {
        self.onboarding.as_ref()
    }

    /// Open or close the picker; opening reads OMP's choices.
    pub fn toggle_picker(&mut self, cx: &mut Context<Self>) {
        self.picker.open = !self.picker.open;
        if self.picker.open
            && let Some(link) = &self.link
        {
            link.refresh_picker();
        }
        cx.notify();
    }

    pub fn toasts(&self) -> Vec<cedian_omp::Toast> {
        self.toasts.iter().map(|(_, toast)| toast.clone()).collect()
    }

    #[cfg(test)]
    fn toast_handles(&self) -> Vec<u64> {
        self.toasts.iter().map(|(id, _)| *id).collect()
    }

    /// The Dismiss button of toast `id`.
    fn dismiss_toast(&mut self, id: u64) {
        self.toasts.retain(|(shown, _)| *shown != id);
    }

    pub fn omp_warning(&self) -> Option<&str> {
        self.omp_warning.as_deref()
    }

    pub fn omp_policy_badge(&self) -> Option<&str> {
        self.omp_policy.as_deref()
    }

    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    pub fn turn(&self) -> &Turn {
        &self.turn
    }

    pub fn notice(&self) -> Option<&str> {
        self.notice.as_deref()
    }

    /// Put `text` in the prompt box, as typing would.
    pub fn set_prompt(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.input
            .update(cx, |editor, cx| editor.set_text(text, window, cx));
    }

    /// Send the prompt box, as the Send button does.
    pub fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.send(window, cx);
    }

    /// The open dialogs' request ids, oldest first.
    pub fn dialog_ids(&self) -> Vec<String> {
        self.dialogs.keys().cloned().collect()
    }

    /// The thread as the panel renders it: messages, then tool cards, each
    /// followed by the subagents it started.
    pub fn transcript(&self) -> Vec<String> {
        let (messages, cards) = cedian_agent_ui::render_thread(self.thread.events());
        let mut lines: Vec<String> = messages
            .into_iter()
            .map(|m| format!("{:?}: {}", m.role, m.text))
            .collect();
        for entry in self.cards_with_subagents(cards) {
            match entry {
                Entry::Card(card) => lines.push(card_line(&card)),
                Entry::Subagent(row) => lines.push(subagent_line(row)),
            }
        }
        lines
    }

    pub fn subagents(&self) -> &SubagentTree {
        &self.subagents
    }

    pub fn subagent_note(&self, id: &str) -> Option<&str> {
        self.subagent_notes.get(id).map(String::as_str)
    }

    pub fn subagent_steer_box(&self, id: &str) -> Option<&Entity<Editor>> {
        self.steer_boxes.get(id)
    }

    /// The Steer button on subagent `id`'s row.
    pub fn steer_subagent(&mut self, id: &str, window: &mut Window, cx: &mut Context<Self>) {
        let Some(input) = self.steer_boxes.get(id) else {
            return;
        };
        let text = input.read(cx).text(cx);
        if text.trim().is_empty() {
            return;
        }
        let Some(link) = &self.link else {
            self.subagent_notes
                .insert(id.to_string(), "OMP is not running".to_string());
            cx.notify();
            return;
        };
        link.steer_subagent(id.to_string(), text);
        input.update(cx, |editor, cx| editor.clear(window, cx));
        self.subagent_notes
            .insert(id.to_string(), "steering…".to_string());
        cx.notify();
    }

    /// The Cancel button on subagent `id`'s row.
    pub fn cancel_subagent(&mut self, id: &str, cx: &mut Context<Self>) {
        let note = match &self.link {
            Some(link) => {
                link.cancel_subagent(id.to_string());
                "cancelling…"
            }
            None => {
                let root = self.workspace_root(cx);
                let id = id.to_string();
                let task = cx.background_spawn(async move {
                    let root = root.ok_or_else(|| "no folder is open".to_string())?;
                    crate::omp_link::cancel_without_omp(&root, &id)
                });
                cx.spawn(async move |this, cx| {
                    if let Err(e) = task.await {
                        this.update(cx, |this, cx| {
                            this.audit_failed(e, cx);
                            cx.notify();
                        })
                        .ok();
                    }
                })
                .detach();
                "OMP is not running"
            }
        };
        self.subagent_notes.insert(id.to_string(), note.to_string());
        cx.notify();
    }

    /// Cards in order, each followed by its subagents; subagents whose card
    /// is not in the thread come last.
    fn cards_with_subagents(&self, cards: Vec<ToolCard>) -> Vec<Entry<'_>> {
        let mut out = Vec::new();
        for card in &cards {
            out.push(Entry::Card(card.clone()));
            out.extend(
                self.subagents
                    .under(&card.call_id)
                    .into_iter()
                    .map(Entry::Subagent),
            );
        }
        out.extend(
            self.subagents
                .rows()
                .iter()
                .filter(|row| {
                    !row.parent_tool_call_id
                        .as_ref()
                        .is_some_and(|parent| cards.iter().any(|c| &c.call_id == parent))
                })
                .map(Entry::Subagent),
        );
        out
    }

    pub fn dialog(&self, id: &str) -> Option<&OpenDialog> {
        self.dialogs.get(id)
    }

    pub fn focus_composer(&self, window: &mut Window, cx: &mut App) {
        window.focus(&self.input.focus_handle(cx), cx);
    }

    /// The composer's text.
    pub fn prompt_text(&self, cx: &App) -> String {
        self.input.read(cx).text(cx)
    }

    /// Images attached to the next prompt.
    pub fn attached_images(&self) -> &[ImageContent] {
        &self.images
    }

    /// OMP's process id while it runs.
    pub fn omp_pid(&self) -> Option<u32> {
        self.link.as_ref().and_then(OmpLink::pid)
    }

    /// Park OMP's thread until the sender is dropped.
    #[cfg(any(test, feature = "test-support"))]
    pub fn hold_omp(&self) -> std::sync::mpsc::Sender<()> {
        self.link.as_ref().expect("OMP runs").hold().unwrap()
    }

    /// Whether the link holds the current prompt in its queue, not yet sent.
    #[cfg(any(test, feature = "test-support"))]
    pub fn prompt_queued(&self) -> bool {
        self.link.as_ref().is_some_and(OmpLink::prompt_queued)
    }

    /// Whether a prompt the panel sent is still in the link or in OMP.
    #[cfg(any(test, feature = "test-support"))]
    pub fn prompt_open(&self) -> bool {
        self.link.as_ref().is_some_and(OmpLink::prompt_open)
    }

    /// Whether OMP runs a run no prompt of the panel's started.
    #[cfg(any(test, feature = "test-support"))]
    pub fn omp_runs_unprompted(&self) -> bool {
        self.link.as_ref().is_some_and(OmpLink::runs_unprompted)
    }

    /// The channel OMP's workflow tools call.
    #[cfg(any(test, feature = "test-support"))]
    pub fn workflow_channel(&self) -> Option<Arc<WorkflowChannel>> {
        self.workflow_channel.clone()
    }

    /// Handle `event` as if OMP's router sent it.
    #[cfg(any(test, feature = "test-support"))]
    pub fn router_event(&mut self, event: RouterEvent, cx: &mut Context<Self>) {
        self.on_event(event, cx);
    }

    /// Make the link's audit rows fail from now on.
    #[cfg(any(test, feature = "test-support"))]
    pub fn break_audit(&self) {
        if let Some(link) = &self.link {
            link.break_audit();
        }
    }

    /// The task's review: every agent transaction imported so far.
    pub fn review(&self) -> &TaskReview {
        &self.review
    }

    /// Show or hide the Review Changes view.
    pub fn toggle_review(&mut self, cx: &mut Context<Self>) {
        self.show_review = !self.show_review;
        self.review.rebuild(cx);
        self.after_review_change(cx);
    }

    /// The Accept button of one hunk: the one the person saw, by identity;
    /// a hunk that moved since is refused with a notice.
    pub fn accept_hunk(&mut self, path: &std::path::Path, key: &HunkKey, cx: &mut Context<Self>) {
        let result = self.review.accept(path, key, cx);
        self.report_review(result, cx);
    }

    /// The Reject button of one hunk: the baseline lines come back as one
    /// transaction (one native undo restores the agent's text).
    pub fn reject_hunk(&mut self, path: &std::path::Path, key: &HunkKey, cx: &mut Context<Self>) {
        let result = self.review.reject(path, key, cx).map(|_| ());
        self.report_review(result, cx);
    }

    /// Rebuild the review on every edit to a reviewed buffer.
    fn watch_reviewed_buffers(&mut self, cx: &mut Context<Self>) {
        let buffers: Vec<Entity<Buffer>> = self
            .review
            .files()
            .iter()
            .map(|f| f.buffer().clone())
            .filter(|b| !self.watched.contains(&b.entity_id()))
            .collect();
        for buffer in buffers {
            self.watched.insert(buffer.entity_id());
            self.buffer_subscriptions
                .push(cx.subscribe(&buffer, |this, _, event, cx| {
                    if matches!(event, BufferEvent::Edited { .. }) {
                        this.review.rebuild(cx);
                        this.after_review_change(cx);
                    }
                }));
        }
    }

    /// The Accept all button. With STALE hunks in the set it asks first
    /// whether to include them (owner ruling 2026-10-08); the answer comes
    /// through [`Self::accept_all_answered`].
    pub fn accept_all(&mut self, cx: &mut Context<Self>) {
        if self.review.stale_count() > 0 {
            self.confirm_accept_all = true;
            cx.notify();
            return;
        }
        self.accept_all_answered(false, cx);
    }

    /// Yes: the STALE hunks too. No: only the others. OutOfScope hunks are
    /// never taken and the notice says how many were skipped (ADR-0033
    /// decision 3).
    pub fn accept_all_answered(&mut self, include_stale: bool, cx: &mut Context<Self>) {
        self.confirm_accept_all = false;
        let skipped = self.review.out_of_scope_count();
        let result = self.review.accept_all(include_stale, cx).map(|accepted| {
            self.notice = Some(match skipped {
                0 => format!("accepted {} hunk(s)", accepted.len()),
                n => format!(
                    "accepted {} hunk(s); {n} out-of-scope hunk(s) skipped, each needs a decision",
                    accepted.len()
                ),
            });
        });
        self.report_review(result, cx);
    }

    /// The Revert turn button: reject every pending hunk the turn made.
    pub fn revert_turn(&mut self, turn: u32, cx: &mut Context<Self>) {
        let result = self.review.revert_turn(turn, cx).map(|event| {
            self.notice = Some(event.to_string());
        });
        self.report_review(result, cx);
    }

    /// The Revert turn button's text: an inline edit is named.
    pub fn revert_turn_label(&self, turn: u32) -> String {
        match self.review.turn_label(turn) {
            Some(label) => format!("Revert turn {turn}: inline edit {label}"),
            None => format!("Revert turn {turn}"),
        }
    }

    /// Revert turn from the editor (ADR-0056 decision 7): the latest turn;
    /// when that is a revert, it is undone.
    pub fn revert_latest_turn(&mut self, cx: &mut Context<Self>) {
        match self.review.current_turn() {
            0 => self.notice = Some("no turn to revert".to_string()),
            turn => self.revert_turn(turn, cx),
        }
        cx.notify();
    }

    /// Each open finding as a block under its hunk, with the hunk's rows
    /// highlighted, in every open editor of its file. A finding whose hunk
    /// changed (stale) or that was dismissed has none.
    fn sync_finding_blocks(&mut self, cx: &mut Context<Self>) {
        let editors: Vec<Entity<Editor>> = self
            .workspace
            .as_ref()
            .and_then(|w| w.upgrade())
            .map(|w| w.read(cx).items_of_type::<Editor>(cx).collect())
            .unwrap_or_default();
        let mut wanted: Vec<(
            Entity<Editor>,
            Entity<Buffer>,
            AttachedFinding,
            std::ops::Range<u32>,
        )> = Vec::new();
        for file in self.review.files() {
            let Some(key) = crate::review::review_key(file.buffer().read(cx)) else {
                continue;
            };
            let open: Vec<&Entity<Editor>> = editors
                .iter()
                .filter(|e| {
                    e.read(cx).buffer().read(cx).as_singleton().as_ref() == Some(file.buffer())
                })
                .collect();
            for hunk in file.hunks() {
                for finding in self.findings.iter().filter(|f| {
                    f.dismissed.is_none()
                        && f.finding.path == key
                        && f.hunk_text == hunk.key.after_text
                }) {
                    for editor in &open {
                        wanted.push((
                            (*editor).clone(),
                            file.buffer().clone(),
                            finding.clone(),
                            hunk.rows.clone(),
                        ));
                    }
                }
            }
        }
        let same = wanted.len() == self.finding_blocks.len()
            && wanted
                .iter()
                .zip(&self.finding_blocks)
                .all(|((e, _, f, r), b)| {
                    b.editor.entity_id() == e.entity_id() && b.finding == f.id && b.rows == *r
                });
        if same {
            return;
        }
        for block in std::mem::take(&mut self.finding_blocks) {
            if let Some(editor) = block.editor.upgrade() {
                editor.update(cx, |editor, cx| {
                    editor.remove_blocks([block.block].into_iter().collect(), None, cx);
                    editor.clear_row_highlights::<FindingRows>();
                    cx.notify();
                });
            }
        }
        for (editor, file_buffer, finding, rows) in wanted {
            let block = editor.update(cx, |editor, cx| {
                let snapshot = editor.buffer().read(cx).snapshot(cx);
                let last = rows.end.saturating_sub(1).max(rows.start);
                let start = snapshot.anchor_before(language::Point::new(rows.start, 0));
                let column = file_buffer.read(cx).line_len(last);
                let end = snapshot.anchor_after(language::Point::new(last, column));
                editor.highlight_rows::<FindingRows>(
                    start..end,
                    |cx| cx.theme().status().error_background,
                    RowHighlightOptions {
                        include_gutter: true,
                        ..Default::default()
                    },
                    cx,
                );
                let selector = format!("cedian-finding-block-{}", finding.id);
                let text = format!(
                    "{} {:?}: {}",
                    finding.id, finding.finding.severity, finding.finding.message
                );
                editor.insert_blocks(
                    [BlockProperties {
                        placement: BlockPlacement::Below(end),
                        height: Some(1),
                        style: BlockStyle::Sticky,
                        render: Arc::new(move |_| {
                            let selector = selector.clone();
                            div()
                                .debug_selector(move || selector)
                                .child(
                                    Label::new(text.clone())
                                        .size(LabelSize::Small)
                                        .color(Color::Error),
                                )
                                .into_any_element()
                        }),
                        priority: 0,
                    }],
                    None,
                    cx,
                )[0]
            });
            self.finding_blocks.push(FindingBlock {
                editor: editor.downgrade(),
                block,
                finding: finding.id.clone(),
                rows,
            });
        }
    }

    /// The findings shown as blocks now: (finding id, the hunk's rows).
    pub fn finding_blocks(&self) -> Vec<(String, std::ops::Range<u32>)> {
        self.finding_blocks
            .iter()
            .map(|b| (b.finding.clone(), b.rows.clone()))
            .collect()
    }

    /// A `cedian://` read as OMP makes it.
    #[cfg(any(test, feature = "test-support"))]
    pub fn read_context(
        &self,
        url: &str,
        cx: &mut App,
    ) -> Task<Result<omp_rpc::HostUriRead, String>> {
        context::answer(url, self.workspace.as_ref(), &self.project, cx)
    }

    /// The open Dismiss reason box.
    #[cfg(any(test, feature = "test-support"))]
    pub fn dismiss_input(&self) -> Option<Entity<Editor>> {
        self.dismissing.as_ref().map(|(_, input)| input.clone())
    }

    /// The reviewer's findings as last read.
    pub fn findings(&self) -> &[AttachedFinding] {
        &self.findings
    }

    /// Read the findings the reviewer reported into the state dir.
    fn load_findings(&mut self, cx: &mut Context<Self>) {
        let loaded = self
            .state_dir(cx)
            .and_then(|dir| cedian_shell::review_findings::load(&dir));
        match loaded {
            Ok(store) => self.findings = store.findings,
            Err(e) => self.notice = Some(format!("review findings: {e}")),
        }
        self.sync_finding_blocks(cx);
        cx.notify();
    }

    /// The models that answered in the OMP running now.
    fn answered_models(&self) -> Vec<String> {
        self.router
            .as_ref()
            .and_then(|cell| cell.get())
            .map(|router| router.answered_models())
            .unwrap_or_default()
    }

    /// A person asks for a review of the task now (ADR-0055): the same
    /// reviewer OMP's `cedian_review_request` runs, its evidence not
    /// attributed to any call.
    pub fn request_review(&mut self, focus: String, cx: &mut Context<Self>) {
        if self._review_run.is_some() {
            self.notice = Some("a review is already running".to_string());
            cx.notify();
            return;
        }
        let (Some(root), Some(channel), Some(settings)) = (
            self.workspace_root(cx),
            self.workflow_channel.clone(),
            self.launch_settings.clone(),
        ) else {
            self.notice = Some("start OMP before asking for a review".to_string());
            cx.notify();
            return;
        };
        let state_dir = match self.state_dir(cx) {
            Ok(dir) => dir,
            Err(e) => {
                self.notice = Some(e);
                cx.notify();
                return;
            }
        };
        let read = Arc::clone(&self.review_reader);
        let models = self.answered_models();
        // Its own thread: the review blocks for minutes, and its reads of
        // the task cross back to this thread.
        let (done, run) = futures::channel::oneshot::channel();
        std::thread::spawn(move || {
            let review = || {
                let task = read()?;
                let diffs: cedian_shell::review_findings::TaskDiffs =
                    Arc::new(move || read().map(|task| task.diffs));
                cedian_shell::review_agent::run_review(
                    &crate::omp_link::review_place(&root, &state_dir),
                    &settings,
                    &focus,
                    task,
                    diffs,
                    cedian_shell::review_agent::Requester {
                        implementer_models: models,
                        tool_call_id: None,
                        channel: Some(&channel),
                    },
                )
            };
            let _ = done.send(review());
        });
        self.notice = Some("review running…".to_string());
        self._review_run = Some(cx.spawn(async move |this, cx| {
            let reply = run
                .await
                .unwrap_or_else(|_| Err("the review thread ended".to_string()));
            this.update(cx, |this, cx| {
                this._review_run = None;
                this.notice = Some(match reply {
                    Ok(reply) => reply,
                    Err(e) => format!("review failed: {e}"),
                });
                this.load_findings(cx);
                this.refresh_workflow(cx);
            })
            .ok();
        }));
        cx.notify();
    }

    /// The Dismiss button of a finding: a reason box opens.
    pub fn start_dismiss(&mut self, id: &str, window: &mut Window, cx: &mut Context<Self>) {
        let input = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Why dismiss this finding?", window, cx);
            editor
        });
        self.dismissing = Some((id.to_string(), input));
        cx.notify();
    }

    /// Dismiss the open finding with the reason typed; an empty reason is
    /// refused (ADR-0041). Audited, and a `finding_dismissed` row.
    pub fn confirm_dismiss(&mut self, cx: &mut Context<Self>) {
        let Some((id, input)) = self.dismissing.clone() else {
            return;
        };
        let reason = input.read(cx).text(cx);
        match self.dismiss_finding(&id, &reason, cx) {
            Ok(done) => {
                self.dismissing = None;
                self.notice = Some(done);
            }
            Err(e) => self.notice = Some(e),
        }
        self.load_findings(cx);
        self.refresh_workflow(cx);
    }

    fn dismiss_finding(
        &mut self,
        id: &str,
        reason: &str,
        cx: &mut Context<Self>,
    ) -> Result<String, String> {
        let dir = self.state_dir(cx)?;
        let review = &self.review;
        cedian_shell::review_findings::dismiss(&dir, review.task_id(), id, reason, |path| {
            Ok(review.last_turn_touching(path, cx))
        })
    }

    fn report_review(&mut self, result: Result<(), ReviewError>, cx: &mut Context<Self>) {
        if let Err(e) = result {
            self.notice = Some(e.to_string());
        }
        self.after_review_change(cx);
    }

    fn after_review_change(&mut self, cx: &mut Context<Self>) {
        for event in self.review.drain_events() {
            let (kind, row) = event.correction();
            let task = self.review.task_id().to_string();
            let recorded = self
                .state_dir(cx)
                .and_then(|dir| cedian_shell::corrections::record(&dir, &task, kind, row));
            if let Err(e) = recorded {
                self.ledger_error =
                    Some(format!("the correction ledger did not record {event}: {e}"));
            }
        }
        if self.review.stale_count() == 0 {
            self.confirm_accept_all = false;
        }
        self.sync_finding_blocks(cx);
        cx.notify();
    }

    /// Where this workspace's ledgers live (ADR-0044).
    fn state_dir(&mut self, cx: &App) -> Result<PathBuf, String> {
        if let Some(dir) = &self.state_dir {
            return Ok(dir.clone());
        }
        let root = self.workspace_root(cx).ok_or("no folder is open")?;
        let dir = cedian_shell::state::dir(&root)?;
        self.state_dir = Some(dir.clone());
        Ok(dir)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_state_dir(&mut self, dir: PathBuf) {
        self.state_dir = Some(dir);
    }

    fn workspace_root(&self, cx: &App) -> Option<PathBuf> {
        let worktree = self.project.read(cx).visible_worktrees(cx).next()?;
        Some(worktree.read(cx).abs_path().to_path_buf())
    }

    /// Show or hide the OMP settings page; it is built on first open.
    pub fn toggle_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.show_settings = !self.show_settings;
        if self.show_settings && self.settings.is_none() {
            if let Some(root) = self.workspace_root(cx) {
                let project = self.project.clone();
                self.settings = Some(cx.new(|cx| OmpSettings::new(project, root, window, cx)));
            }
        }
        cx.notify();
    }

    pub fn omp_settings(&self) -> Option<&Entity<OmpSettings>> {
        self.settings.as_ref()
    }

    /// Start (or restart) OMP for the open folder.
    fn start(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let previous = self.link.take();
        self.notice = None;
        self.forget_old_omp(window, cx);
        self.dialogs.clear();
        self.worktree_ask = None;
        self._worktree_expiry = None;
        self.release_in_flight(cx);
        self.set_turn(Turn::Idle);
        let Some(root) = self.workspace_root(cx) else {
            self.connection = Connection::Stopped("open a folder first".to_string());
            return;
        };
        let mut spec = match LaunchSpec::resolve(&root) {
            Ok(spec) => spec,
            Err(e) => {
                self.connection = Connection::Stopped(e);
                cx.notify();
                return;
            }
        };
        self.omp_warning = None;
        self.omp_policy = (spec.policy_source == cedian_shell::Policy::Omp).then(|| {
            "◆ OMP policy — approvals and computer from your OMP config (reading it…)".to_string()
        });
        spec.unsaved.lock().extend(
            self.unsaved
                .lock()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self.unsaved = Arc::clone(&spec.unsaved);
        self.state_dir = Some(spec.state_dir.clone());
        self.refresh_workflow(cx);
        if self.browser.is_none() {
            self.open_browser_host(&spec.state_dir, cx);
        }
        spec.policy.browser_cdp_url = self.browser.as_ref().map(|b| b.url());
        if let Some(browser) = &self.browser {
            spec.browser.set(Arc::clone(browser)).ok();
        }
        self.workflow_channel = Some(Arc::clone(&spec.workflow));
        spec.review.set(Arc::clone(&self.review_reader)).ok();
        self.router = Some(Arc::clone(&spec.router));
        self.launch_settings = Some(spec.settings.clone());
        self.load_findings(cx);
        let (read_tx, read_rx) = mpsc::unbounded::<context::Read>();
        spec.uris.push(context::scheme(read_tx));
        let this = cx.entity().downgrade();
        self._context = Some(context::serve(
            read_rx,
            move |url, cx| {
                this.update(cx, |this, cx| {
                    context::answer(url, this.workspace.as_ref(), &this.project, cx)
                })
                .ok()
            },
            cx,
        ));
        let (event_tx, mut event_rx) = mpsc::unbounded::<LinkEvent>();
        self.worktree_gate = spec.worktree_ask.clone();
        if let Some(ask) = &self.worktree_gate {
            ask.wire(event_tx.clone());
        }
        self.instruments.spawning();
        self.link = Some(OmpLink::start(spec, event_tx, previous));
        self.connection = Connection::Starting;
        self._events = Some(cx.spawn_in(window, async move |this, cx| {
            while let Some(event) = event_rx.next().await {
                if this
                    .update_in(cx, |this, window, cx| this.on_link_event(event, window, cx))
                    .is_err()
                {
                    break;
                }
            }
        }));
        cx.notify();
    }

    /// Listen on the workspace's browser endpoint; Chromium starts on the
    /// first connection.
    fn open_browser_host(&mut self, state_dir: &Path, cx: &mut Context<Self>) {
        let (tx, mut rx) = mpsc::unbounded::<()>();
        match BrowserHost::open(
            state_dir.join("browser-profile"),
            crate::browser::executable(),
            move || {
                let _ = tx.unbounded_send(());
            },
        ) {
            Ok(host) => {
                self.browser = Some(Arc::new(host));
                self._browser_quit = Some(cx.on_app_quit(|this: &mut Self, _| {
                    if let Some(host) = &this.browser {
                        host.close_now(BROWSER_QUIT_LIMIT);
                    }
                    async {}
                }));
            }
            Err(e) => {
                self.notice = Some(e);
                return;
            }
        }
        self._browser_events = Some(cx.spawn(async move |this, cx| {
            while rx.next().await.is_some() {
                if this.update(cx, |_, cx| cx.notify()).is_err() {
                    break;
                }
            }
        }));
    }

    pub fn browser(&self) -> Option<&Arc<BrowserHost>> {
        self.browser.as_ref()
    }

    /// The "Open browser" button: start Chromium now.
    pub fn open_browser(&mut self, cx: &mut Context<Self>) {
        let Some(host) = self.browser.clone() else {
            return;
        };
        cx.background_spawn(async move {
            let _ = host.start();
        })
        .detach();
    }

    /// The browser holds OMP's traffic on the person's input only while a
    /// turn runs.
    fn set_turn(&mut self, turn: Turn) {
        let running = matches!(turn, Turn::Queued | Turn::Streaming | Turn::Stopping);
        self.turn = turn;
        if let Some(host) = &self.browser {
            host.set_turn(running);
        }
    }

    /// The person lets the agent use the browser again.
    pub fn resume_browser(&mut self, cx: &mut Context<Self>) {
        if let Some(host) = &self.browser {
            host.resume();
        }
        cx.notify();
    }

    /// The state gates are judged against: the shared browser's current
    /// frame sequence, so a capture from an earlier frame reads `stale-frame`.
    pub fn current_state(&self) -> CurrentState {
        let mut state = CurrentState::default();
        state.frame_seq = self
            .browser
            .as_ref()
            .map(|b| b.state())
            .filter(|s| s.running)
            .map(|s| s.seq);
        state
    }

    /// `gate` judged on the panel's browser evidence, now.
    pub fn evaluate_gate(&self, gate: &Gate) -> GateResult {
        gate.evaluate(&self.browser_evidence, &self.current_state())
    }

    /// Capture the shared page: screenshot, DOM, console and network.
    pub fn capture_browser(&mut self, cx: &mut Context<Self>) {
        let Some(host) = self.browser.clone() else {
            return;
        };
        let task = cx.background_spawn(async move { host.capture() });
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(capture) => {
                        let code = this.current_state().bind(&[]);
                        this.browser_evidence.push(
                            capture
                                .evidence(
                                    &format!("browser-frame-{}", capture.seq),
                                    &[BROWSER_GATE],
                                    capture.outcome(),
                                )
                                .with_code_state(code),
                        );
                    }
                    Err(e) => this.notice = Some(format!("browser capture failed: {e}")),
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// OMP's `browser` call ended: when the running workflow has a gate
    /// that takes browser evidence (the user's floor asks for one,
    /// ADR-0055), the page now is captured and stored as that call's
    /// evidence, bound to its frame. Otherwise nothing is captured.
    fn observe_browser_call(&mut self, call: String, is_error: bool, cx: &mut Context<Self>) {
        let (Some(host), Some(channel)) = (self.browser.clone(), self.workflow_channel.clone())
        else {
            return;
        };
        let ended_at = host.state().seq;
        let task = cx.background_spawn(async move {
            let gates = channel.gates_taking(EvidenceKind::Browser)?;
            if gates.is_empty() {
                return Ok(None);
            }
            let gates: Vec<&str> = gates.iter().map(String::as_str).collect();
            let mut item =
                crate::browser::call_evidence(host.capture(), ended_at, is_error, &gates)?;
            item.provenance = Provenance::Attributed {
                task_id: TASK_ID.to_string(),
                tool_call_id: call.clone(),
            };
            item.summary = format!("after browser call {call}: {}", item.summary);
            channel.observe(item)
        });
        cx.spawn(async move |this, cx| {
            if let Err(e) = task.await {
                this.update(cx, |this, cx| {
                    this.notice = Some(format!("the browser call's capture failed: {e}"));
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
    }

    /// The shared browser: its latest capture, with the frame it was taken
    /// at, and the console and network lines it carries.
    fn render_browser(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let state = self.browser.as_ref()?.state();
        if !state.running && state.latest.is_none() && state.error.is_none() {
            return None;
        }
        const LINES: usize = 20;
        let tail = |lines: &[String]| -> Vec<AnyElement> {
            lines[lines.len().saturating_sub(LINES)..]
                .iter()
                .map(|l| {
                    Label::new(l.clone())
                        .size(LabelSize::XSmall)
                        .color(Color::Muted)
                        .into_any_element()
                })
                .collect()
        };
        let capture = state.latest.as_ref().map(|c| {
            v_flex()
                .debug_selector(|| "cedian-browser-capture".to_string())
                .gap_1()
                .child(
                    gpui::img(Arc::new(gpui::Image::from_bytes(
                        gpui::ImageFormat::Png,
                        c.png.to_vec(),
                    )))
                    .w_full()
                    .h(px(180.))
                    .object_fit(gpui::ObjectFit::Contain),
                )
                .child(
                    Label::new(format!("capture at frame {} · {}", c.seq, c.url))
                        .size(LabelSize::Small),
                )
                .child(Label::new("Console").size(LabelSize::Small))
                .children(tail(&c.console))
                .child(Label::new("Network").size(LabelSize::Small))
                .children(tail(&c.network))
        });
        Some(
            v_flex()
                .debug_selector(|| "cedian-browser".to_string())
                .gap_1()
                .child(
                    h_flex()
                        .gap_2()
                        .child(
                            Label::new(if state.running {
                                format!("Browser · frame {} · {}", state.seq, state.url)
                            } else {
                                "Browser closed".to_string()
                            })
                            .size(LabelSize::Small),
                        )
                        .when(state.running, |row| {
                            row.child(
                                Button::new("cedian-browser-capture", "Capture").on_click(
                                    cx.listener(|this, _, _, cx| this.capture_browser(cx)),
                                ),
                            )
                        }),
                )
                .when(state.preempted, |col| {
                    col.child(
                        h_flex()
                            .debug_selector(|| "cedian-browser-held".to_string())
                            .gap_2()
                            .child(
                                Label::new(
                                    "You are using the browser: the agent's next browser call waits for you",
                                )
                                .size(LabelSize::Small)
                                .color(Color::Warning),
                            )
                            .child(
                                Button::new("cedian-browser-resume", "Let the agent continue")
                                    .on_click(
                                        cx.listener(|this, _, _, cx| this.resume_browser(cx)),
                                    ),
                            ),
                    )
                })
                .when_some(state.error, |col, e| {
                    col.child(Label::new(e).size(LabelSize::Small).color(Color::Error))
                })
                .children(capture)
                .into_any_element(),
        )
    }

    /// The worker surface's repo and state dir, when the panel has both.
    fn worker_paths(&mut self, cx: &App) -> Option<(PathBuf, PathBuf)> {
        Some((self.workspace_root(cx)?, self.state_dir(cx).ok()?))
    }

    /// The turn's scope: the union of the running workers' briefs at this
    /// import (ADR-0063 decision 1). No running worker, or a registry or
    /// brief that will not read, is no scope — nothing is classified and
    /// nothing changes.
    fn turn_scope(&mut self, cx: &App) -> Option<Arc<WorkerScope>> {
        let (_, state) = self.worker_paths(cx)?;
        let (registry, _) = Registry::open(&state).ok()?;
        let mut briefs = Vec::new();
        for head in registry.all().filter(|h| h.status == WorkerStatus::Running) {
            let path = state.join("briefs").join(format!("{}.1.json", head.id));
            briefs.push((head.id.clone(), Brief::from_file(&path).ok()?));
        }
        if briefs.is_empty() {
            return None;
        }
        WorkerScope::new(briefs.iter().map(|(id, b)| (id.as_str(), b)))
            .ok()
            .map(Arc::new)
    }

    /// One audit row for one out-of-scope file (ADR-0063 decision 4), through
    /// the link that owns the log. In scope, no link, or a row that will not
    /// write changes nothing here.
    fn audit_import(
        &self,
        tool_call_id: &str,
        buffer: &Entity<Buffer>,
        scope: Option<&WorkerScope>,
        cx: &App,
    ) {
        let Some(scope) = scope else { return };
        let Some(path) = crate::review::scope_path(&buffer.read(cx)) else {
            return;
        };
        if scope.covers(&path) {
            return;
        }
        let Some(link) = &self.link else { return };
        let shown = crate::review::review_key(&buffer.read(cx)).unwrap_or(path);
        let workers: Vec<String> = scope.workers().map(str::to_string).collect();
        link.out_of_scope(tool_call_id.to_string(), shown, workers);
    }

    /// Read the worktrees again, off the UI thread (it shells out to `git`):
    /// the first read is what builds the surface.
    fn refresh_worker_trees(&mut self, cx: &mut Context<Self>) {
        let Some((root, dir)) = self.worker_paths(cx) else {
            return;
        };
        let task = cx.background_spawn(async move { worker_trees::read(&root, &dir, WORKER_BASE) });
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(rows) => this.worker_trees = Some(rows),
                    Err(e) => {
                        this.notice = Some(format!("the worker worktrees could not be read: {e}"))
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Run a worker-tree action off the UI thread, show what it came to in
    /// `notice`, and re-read the list.
    fn worker_action(
        &mut self,
        cx: &mut Context<Self>,
        action: impl FnOnce(PathBuf, PathBuf) -> Result<String, String> + Send + 'static,
    ) {
        let Some((root, dir)) = self.worker_paths(cx) else {
            self.notice = Some("no folder is open".to_string());
            cx.notify();
            return;
        };
        let task = cx.background_spawn(async move { action(root, dir) });
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                this.notice = Some(match result {
                    Ok(said) => said,
                    Err(e) => e,
                });
                this.refresh_worker_trees(cx);
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// The cedian worktrees as last read; `None` before the first read.
    pub fn worker_trees(&self) -> Option<&[WorkerTreeRow]> {
        self.worker_trees.as_deref()
    }

    /// The worker whose Remove is waiting on a second click.
    pub fn confirm_remove(&self) -> Option<&str> {
        self.confirm_remove.as_deref()
    }

    /// Trial-merge a worker's branch into the base in a throwaway clone and
    /// say what it would come to, without touching the repo.
    pub fn preview_worker(&mut self, id: &str, cx: &mut Context<Self>) {
        let id = id.to_string();
        self.worker_action(cx, move |repo, state| {
            worker_trees::preview(&repo, &state, &id, WORKER_BASE)
        });
    }

    /// Merge a worker's branch back into the checked-out base, on one click;
    /// a STALE refusal is the notice, and nothing else changes. Refused
    /// while an out-of-scope change is unresolved (ADR-0033 decision 3):
    /// a person must accept or reject it first.
    pub fn merge_worker(&mut self, id: &str, cx: &mut Context<Self>) {
        let unresolved = self.review.out_of_scope_count();
        if unresolved > 0 {
            self.notice = Some(format!(
                "merge refused: {unresolved} out-of-scope change(s) need a decision \
                 (accept each in Review Changes; Accept all skips them)"
            ));
            cx.notify();
            return;
        }
        let (id, said) = (id.to_string(), id.to_string());
        self.worker_action(cx, move |repo, state| {
            worker_trees::merge_back(&repo, &state, &id, WORKER_BASE)
                .map(|()| format!("merged {said}"))
        });
    }

    /// Ask before removing: `worktree remove` throws away a dirty tree, so
    /// the first click only arms the row's confirm.
    pub fn begin_remove_worker(&mut self, id: &str, cx: &mut Context<Self>) {
        self.confirm_remove = Some(id.to_string());
        cx.notify();
    }

    pub fn cancel_remove_worker(&mut self, cx: &mut Context<Self>) {
        self.confirm_remove = None;
        cx.notify();
    }

    pub fn confirm_remove_worker(&mut self, id: &str, cx: &mut Context<Self>) {
        self.confirm_remove = None;
        let (id, said) = (id.to_string(), id.to_string());
        self.worker_action(cx, move |repo, state| {
            worker_trees::remove(&repo, &state, &id).map(|()| format!("removed {said}"))
        });
    }

    /// Clean up this worker's tree; the click is its explicit per-tree
    /// decision (§43), which is what a `Wip` tree needs. Exactly this tree
    /// goes, never its neighbours.
    pub fn cleanup_worker(&mut self, id: &str, cx: &mut Context<Self>) {
        let id = id.to_string();
        self.worker_action(cx, move |repo, state| {
            worker_trees::cleanup(&repo, &state, &id, WORKER_BASE)
        });
    }

    /// The cedian worktrees on disk, each with its class and registry row;
    /// `None` until the first read, so nothing changes before it.
    fn render_worker_trees(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        // Nothing to show until a worker exists: an empty section would be a
        // permanent line on every panel that never uses worktrees.
        let rows = self.worker_trees.as_ref().filter(|rows| !rows.is_empty())?;
        Some(
            v_flex()
                .debug_selector(|| "cedian-workers".to_string())
                .gap_1()
                .child(
                    h_flex()
                        .gap_2()
                        .child(
                            Label::new(format!("Workers · {}", rows.len())).size(LabelSize::Small),
                        )
                        .child(
                            div()
                                .debug_selector(|| "cedian-workers-refresh".to_string())
                                .child(Button::new("cedian-workers-refresh", "Refresh").on_click(
                                    cx.listener(|this, _, _, cx| this.refresh_worker_trees(cx)),
                                )),
                        ),
                )
                .children(rows.iter().map(|row| self.render_worker_tree(row, cx)))
                .into_any_element(),
        )
    }

    /// One worker worktree: its class, the registry row when there is one, the
    /// path on disk, and Preview / Merge / Remove / Cleanup.
    fn render_worker_tree(&self, row: &WorkerTreeRow, cx: &mut Context<Self>) -> AnyElement {
        let id = row.state.id.clone();
        let selector = format!("cedian-worker-{id}");
        type OnClick = fn(&mut CedianPanel, &str, &mut Context<CedianPanel>);
        let button = |label: &'static str, suffix: &str, on: OnClick, cx: &mut Context<Self>| {
            let selector = format!("cedian-worker-{id}-{suffix}");
            let element = ElementId::Name(selector.clone().into());
            let id = id.clone();
            div().debug_selector(move || selector).child(
                Button::new(element, label)
                    .on_click(cx.listener(move |this, _, _, cx| on(this, &id, cx))),
            )
        };
        let mut line = format!("{id} · {}", row.state.class);
        match &row.head {
            Some(head) => line.push_str(&format!(" · {:?} · {}", head.status, head.task_title)),
            None => line.push_str(" · no registry row"),
        }
        let controls = if self.confirm_remove.as_deref() == Some(id.as_str()) {
            let confirm = format!("cedian-worker-{id}-remove-confirm");
            let cancel = format!("cedian-worker-{id}-remove-cancel");
            let confirm_selector = confirm.clone();
            let cancel_selector = cancel.clone();
            let remove = id.clone();
            h_flex()
                .gap_2()
                .child(
                    Label::new("Remove this worktree and its branch?")
                        .size(LabelSize::Small)
                        .color(Color::Warning),
                )
                .child(div().debug_selector(move || confirm_selector).child(
                    Button::new(ElementId::Name(confirm.into()), "Remove").on_click(
                        cx.listener(move |this, _, _, cx| this.confirm_remove_worker(&remove, cx)),
                    ),
                ))
                .child(
                    div().debug_selector(move || cancel_selector).child(
                        Button::new(ElementId::Name(cancel.into()), "Keep")
                            .on_click(cx.listener(|this, _, _, cx| this.cancel_remove_worker(cx))),
                    ),
                )
        } else {
            h_flex()
                .gap_2()
                .child(button("Preview", "preview", Self::preview_worker, cx))
                .child(button("Merge back", "merge", Self::merge_worker, cx))
                .child(button("Remove", "remove", Self::begin_remove_worker, cx))
                .child(button("Cleanup", "cleanup", Self::cleanup_worker, cx))
        };
        let stuck = format!("cedian-worker-{id}-stuck");
        let path = row.state.path.display().to_string();
        v_flex()
            .debug_selector(move || selector)
            .gap_1()
            .child(Label::new(line).size(LabelSize::Small))
            .when(row.stuck, |col| {
                col.child(
                    div().debug_selector(move || stuck).child(
                        Label::new("stuck: no progress inside its timebox")
                            .size(LabelSize::Small)
                            .color(Color::Warning),
                    ),
                )
            })
            .child(Label::new(path).size(LabelSize::Small).color(Color::Muted))
            .child(controls)
            .into_any_element()
    }

    /// The prompt as OMP gets it: the bounded snapshot of what the person
    /// sees (§39), then their text.
    fn with_context(&self, text: &str, cx: &App) -> String {
        let snapshot = context::host(self.workspace.as_ref(), &self.project, cx)
            .map(|host| render_snapshot(&capture_ambient(&host)))
            .unwrap_or_default();
        if snapshot.is_empty() {
            text.to_string()
        } else {
            format!("{snapshot}\n{text}")
        }
    }

    /// The Retry button of a taken session.
    pub fn retry_session(&mut self, cx: &mut Context<Self>) {
        match self.link.as_ref().map(OmpLink::retry) {
            Some(Ok(())) => self.connection = Connection::Checking,
            Some(Err(e)) => self.notice = Some(e),
            None => {}
        }
        cx.notify();
    }

    /// The "Start a new session" button of a taken session.
    pub fn start_new_session(&mut self, cx: &mut Context<Self>) {
        if let Some(Err(e)) = self.link.as_ref().map(OmpLink::new_session) {
            self.notice = Some(e);
        }
        cx.notify();
    }

    /// The old OMP's subagents and queue die with it: their rows go, and
    /// what was queued goes back (`take_back`).
    fn forget_old_omp(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.forget_subagents();
        let queued = self.queued();
        if !queued.is_empty() {
            self.thread.apply(&RouterEvent::Queue {
                steering: Vec::new(),
                follow_up: Vec::new(),
            });
            self.take_back(queued, window, cx);
        }
        self.follow_ups.clear();
        self.retrying = false;
        self.picker = ModelPicker::default();
        self.onboarding = None;
        self.toasts.clear();
        self.session_notes.clear();
        self.edit_turn = None;
        self.unblock_instruction();
    }

    /// No answer to the block's follow-up is coming: Enter sends it again.
    fn unblock_instruction(&mut self) {
        if let Some(block) = &mut self.instruction {
            block.pending = None;
        }
    }

    /// Queued messages that did not run: a chat message goes back to the
    /// composer; an inline edit is named in the notice with its
    /// instruction, since resent from the composer it would be a plain turn.
    fn take_back(&mut self, texts: Vec<String>, window: &mut Window, cx: &mut Context<Self>) {
        let mut composer = Vec::new();
        let mut edits = Vec::new();
        for text in texts {
            let found = self.follow_ups.iter().rposition(|q| q.message == text);
            match found.map(|i| self.follow_ups.remove(i)) {
                Some(QueuedFollowUp {
                    edit: Some(edit), ..
                }) => edits.push(format!(
                    "{} (instruction: {})",
                    edit.label(cx),
                    edit.instruction
                )),
                _ => composer.push(text),
            }
        }
        if !composer.is_empty() {
            self.restore_to_composer(composer, window, cx);
        }
        if !edits.is_empty() {
            self.add_notice(format!(
                "the queued inline edit did not run: {}; select the lines and \
                 press ctrl-enter to send it again",
                edits.join(", ")
            ));
        }
    }

    /// `said` joins the notice shown now.
    fn add_notice(&mut self, said: String) {
        self.notice = Some(match self.notice.take() {
            Some(notice) => format!("{notice}; {said}"),
            None => said,
        });
    }

    fn forget_subagents(&mut self) {
        self.subagents = SubagentTree::default();
        self.steer_boxes.clear();
        self.subagent_notes.clear();
    }

    /// What OMP last said it has queued, oldest steer first.
    fn queued(&self) -> Vec<String> {
        self.thread
            .events()
            .iter()
            .rev()
            .find_map(|event| match event {
                ThreadEvent::Queue {
                    steering,
                    follow_up,
                } => Some(steering.iter().chain(follow_up).cloned().collect()),
                _ => None,
            })
            .unwrap_or_default()
    }

    /// Put `texts` back in the composer, ahead of what is typed there now.
    fn restore_to_composer(
        &mut self,
        mut texts: Vec<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let typed = self.input.read(cx).text(cx);
        if !typed.is_empty() {
            texts.push(typed);
        }
        let text = texts.join("\n");
        self.input
            .update(cx, |editor, cx| editor.set_text(text, window, cx));
    }

    /// The Restart button: a fresh OMP on the same session.
    pub fn restart(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.start(window, cx);
    }

    fn on_link_event(&mut self, event: LinkEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            LinkEvent::Ready {
                session_id,
                resumed,
                policy_note,
                model,
                thinking,
                ..
            } => {
                self.forget_subagents();
                self.onboarding = None;
                if model.is_some() {
                    self.picker.model = model;
                } else if let Some(link) = &self.link {
                    self.onboarding = Some(Onboarding::default());
                    link.login_providers();
                }
                if thinking.is_some() {
                    self.picker.thinking = thinking;
                }
                self.connection = Connection::Ready {
                    session_id,
                    resumed,
                    policy_note,
                };
                self.instruments.ready();
            }
            LinkEvent::OmpWarning(warning) => self.omp_warning = Some(warning),
            LinkEvent::OmpPolicy(badge) => self.omp_policy = Some(badge),
            LinkEvent::Failed(reason) => self.stop(reason, cx),
            LinkEvent::Taken { session_id, reason } => {
                if self.turn != Turn::Idle {
                    self.thread.withdraw_user();
                    self.end_turn(None, cx);
                }
                self.connection = Connection::Taken { session_id, reason };
            }
            LinkEvent::PromptTimed { sent, returned } => {
                self.instruments.prompt_returned(sent, returned);
            }
            LinkEvent::PromptCancelled => {
                self.instruments.abandoned();
                self.thread.withdraw_user();
                self.end_turn(None, cx);
            }
            LinkEvent::PromptStopped(e) => {
                self.end_turn(Some(format!("turn stopped: {e}")), cx);
            }
            LinkEvent::PromptFailed(e) => {
                self.instruments.abandoned();
                self.end_turn(Some(format!("OMP did not run the prompt: {e}")), cx);
            }
            LinkEvent::AbortFailed(e) => {
                if self.turn != Turn::Idle {
                    self.set_turn(Turn::Failed(format!("stop: {e}")));
                }
            }
            LinkEvent::AuditFailed(e) => self.audit_failed(e, cx),
            LinkEvent::Queued(text) => {
                if self.input.read(cx).text(cx) == text {
                    self.input.update(cx, |editor, cx| editor.clear(window, cx));
                }
                if self
                    .instruction
                    .as_ref()
                    .is_some_and(|b| b.pending.as_ref() == Some(&text))
                {
                    self.dismiss_instruction(window, cx);
                }
            }
            LinkEvent::QueueRefused { text, reason } => self.queue_refused(text, reason, cx),
            LinkEvent::Restored(texts) => self.take_back(texts, window, cx),
            LinkEvent::TakeBackNotice(notice) => self.notice = Some(notice),
            LinkEvent::SubagentSteered { id, result } => {
                let note = match result {
                    Ok(()) => "steered".to_string(),
                    Err(e) => format!("steer refused: {e}"),
                };
                self.subagent_notes.insert(id, note);
            }
            LinkEvent::SubagentCancelled { id, result } => {
                let note = match result {
                    Ok(true) => "cancelled".to_string(),
                    Ok(false) => "already ended".to_string(),
                    Err(e) => format!("cancel failed: {e}"),
                };
                self.subagent_notes.insert(id, note);
            }
            LinkEvent::WorktreeAsked { request, id, goal } => {
                self.worktree_ask = Some((request, id, goal));
                // The same 5-minute lease the dialogs get (ADR-0013).
                self._worktree_expiry = Some(cx.spawn(async move |this, cx| {
                    cx.background_executor().timer(DIALOG_TIMEOUT).await;
                    this.update(cx, |this, cx| this.expire_worktree(cx)).ok();
                }));
                cx.notify();
            }
            LinkEvent::Event(RouterEvent::UiRequest(ExtensionUiRequest::Cancel(cancel))) => {
                self.dialogs.shift_remove(&cancel.target_id);
                if let Some(onboarding) = &mut self.onboarding
                    && onboarding.dialogs.contains(&cancel.target_id)
                {
                    onboarding.running = None;
                    onboarding.url = None;
                    onboarding.dialogs.clear();
                }
            }
            LinkEvent::Event(RouterEvent::UiRequest(ExtensionUiRequest::OpenUrl(request))) => {
                self.open_url(request, cx);
            }
            LinkEvent::Event(RouterEvent::UiRequest(request)) => {
                let id = cedian_omp::dialog::dialog(&request).map(|(id, _)| id.to_string());
                if let Some(id) = id
                    && self.link.as_ref().is_some_and(|link| link.is_open(&id))
                    && let Some(mut dialog) = OpenDialog::new(request, window, cx)
                {
                    let expiring = id.clone();
                    dialog.expiry = Some(cx.spawn(async move |this, cx| {
                        cx.background_executor().timer(DIALOG_TIMEOUT).await;
                        this.update(cx, |this, cx| this.expire(&expiring, cx)).ok();
                    }));
                    if let Some(onboarding) = &mut self.onboarding
                        && onboarding.running.is_some()
                    {
                        onboarding.dialogs.push(id.clone());
                    }
                    self.dialogs.insert(id, dialog);
                }
            }
            LinkEvent::Event(RouterEvent::Disconnected) => {
                self.thread.apply(&RouterEvent::Disconnected);
                self.stop("OMP stopped: its process exited".to_string(), cx);
            }
            LinkEvent::Event(RouterEvent::Unknown { frame_type })
                if frame_type == "config_update" =>
            {
                if let Some(settings) = &self.settings {
                    settings.update(cx, |settings, cx| settings.reload(cx));
                }
            }
            LinkEvent::Setting(result) => self.push_note(match result {
                Ok(note) => note,
                Err(e) => format!("OMP refused the setting: {e}"),
            }),
            LinkEvent::Promoted { text, result } => {
                let asked = self
                    .follow_ups
                    .iter()
                    .position(|q| q.promoting && q.message == text);
                match (result, asked) {
                    (Ok(true), asked) => {
                        if let Some(i) = asked {
                            self.follow_ups.remove(i);
                        }
                        self.push_note(format!("steering now: {text}"));
                    }
                    (refused, Some(i)) => {
                        self.follow_ups[i].promoting = false;
                        let why = refused.err().unwrap_or_else(|| "not queued".to_string());
                        self.add_notice(format!("OMP did not steer with {text:?} now: {why}"));
                    }
                    (Ok(false), None) => self.add_notice(format!(
                        "OMP ran {text:?} as its own turn before it could steer"
                    )),
                    (Err(why), None) => {
                        self.add_notice(format!("OMP did not steer with {text:?}: {why}"))
                    }
                }
            }
            LinkEvent::Event(RouterEvent::RunNote { text, retrying }) => {
                if let Some(retrying) = retrying {
                    self.retrying = retrying;
                }
                self.push_note(text);
            }
            LinkEvent::LoginProviders(result) => {
                if let Some(onboarding) = &mut self.onboarding {
                    match result {
                        Ok(providers) => onboarding.providers = providers,
                        Err(e) => onboarding.error = Some(e),
                    }
                }
            }
            LinkEvent::LoggedIn { provider, result } => {
                // A cancelled login's late result leaves a later one running.
                if let Some(onboarding) = &mut self.onboarding {
                    if onboarding.running.as_deref() == Some(provider.as_str()) {
                        onboarding.running = None;
                        onboarding.url = None;
                        onboarding.dialogs.clear();
                    }
                    match result {
                        Ok(()) => {
                            if let Some(link) = &self.link {
                                link.refresh_picker();
                                onboarding.logged_in = true;
                            }
                        }
                        Err(e) => onboarding.error = Some(format!("login with {provider}: {e}")),
                    }
                }
            }
            LinkEvent::Picker(Ok(state)) => {
                if state.model.is_some() {
                    self.onboarding = None;
                } else if let Some(onboarding) = &mut self.onboarding
                    && std::mem::take(&mut onboarding.logged_in)
                    && let Some(link) = &self.link
                {
                    link.login_providers();
                }
                self.picker.models = state.models;
                self.picker.levels = state.levels;
                self.picker.model = state.model;
                self.picker.thinking = state.thinking;
                self.picker.error = None;
            }
            LinkEvent::PickerChanged(Ok(change)) => {
                if change.model.is_some() {
                    self.picker.model = change.model;
                }
                if change.thinking.is_some() {
                    self.picker.thinking = change.thinking;
                }
                self.picker.error = None;
            }
            LinkEvent::Picker(Err(e)) | LinkEvent::PickerChanged(Err(e)) => {
                self.picker.error = Some(e);
            }
            LinkEvent::Event(RouterEvent::ModelChanged) => {
                if let Some(link) = &self.link {
                    link.refresh_picker();
                }
            }
            LinkEvent::Event(RouterEvent::ThinkingLevel(level)) => self.picker.thinking = level,
            LinkEvent::Event(RouterEvent::Toast(toast)) => self.push_toast(toast),
            LinkEvent::Event(event) => {
                let settled = matches!(event, RouterEvent::Settled);
                if matches!(event, RouterEvent::AgentStart) {
                    self.instruments.stream_started(window);
                }
                self.on_event(event, cx);
                if settled {
                    self.instruments.settled();
                    self.instruments.stream_ended(window);
                }
                return;
            }
        }
        cx.notify();
    }

    /// The prompt call ended without OMP settling: idle again, a call whose
    /// ToolEnd never came is released, and a failed turn keeps its reason
    /// over `notice`.
    fn end_turn(&mut self, notice: Option<String>, cx: &mut Context<Self>) {
        self.retrying = false;
        self.review.end_turn();
        self.release_in_flight(cx);
        if let Turn::Failed(reason) = std::mem::replace(&mut self.turn, Turn::Idle) {
            self.notice = Some(format!("turn failed: {reason}"));
        } else if notice.is_some() {
            self.notice = notice;
        }
        self.warn_outside_selection(cx);
    }

    /// A follow-up with `text` did not reach OMP's queue: it is forgotten,
    /// and an inline edit's instruction stays in its block (or the notice
    /// when the block is gone).
    fn queue_refused(&mut self, text: String, reason: String, cx: &mut Context<Self>) {
        let found = self
            .follow_ups
            .iter()
            .rposition(|q| q.shown.is_none() && q.message == text);
        let edit = found.and_then(|i| self.follow_ups.remove(i).edit);
        let block = self
            .instruction
            .as_mut()
            .filter(|b| b.pending.as_ref() == Some(&text))
            .map(|block| block.pending = None);
        self.notice = Some(match (edit, block) {
            (Some(_), Some(())) => {
                format!("OMP did not queue the inline edit: {reason}; it stays in its block")
            }
            (Some(edit), None) => format!(
                "OMP did not queue the inline edit {}: {reason}; instruction: {}",
                edit.label(cx),
                edit.instruction
            ),
            (None, _) => format!("OMP did not queue it: {reason}"),
        });
    }

    /// Release every call still waiting for its ToolEnd: it is not coming.
    fn release_in_flight(&mut self, cx: &mut Context<Self>) {
        let ids: Vec<String> = self
            .calls
            .iter()
            .filter(|(_, c)| c.ended.is_none())
            .map(|(id, _)| id.clone())
            .collect();
        for id in ids {
            let call = self.calls.remove(&id).expect("listed above");
            self.release(&id, call, cx);
        }
    }

    fn stop(&mut self, reason: String, cx: &mut Context<Self>) {
        self.link = None;
        self.omp_policy = None;
        self.omp_warning = None;
        self.retrying = false;
        self.dialogs.clear();
        self.subagents.apply(&RouterEvent::Disconnected);
        for (id, call) in std::mem::take(&mut self.calls) {
            self.release(&id, call, cx);
        }
        self.review.end_turn();
        self.warn_outside_selection(cx);
        self.end_workflow_turn(cx);
        self.set_turn(Turn::Idle);
        self.unblock_instruction();
        self.connection = Connection::Stopped(reason);
    }

    fn send(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let started = Instant::now();
        let text = self.input.read(cx).text(cx);
        if text.trim().is_empty() {
            return;
        }
        match self.turn {
            Turn::Idle => {}
            Turn::Streaming => return self.queue(false, cx),
            _ => {
                self.notice = Some("the turn is starting or stopping; wait for it".to_string());
                cx.notify();
                return;
            }
        }
        let prompt = Prompt {
            text: self.with_context(&text, cx),
            images: self.images.clone(),
        };
        if let Err(e) = self.send_prompt(prompt, window, cx) {
            self.notice = Some(e);
            cx.notify();
            return;
        }
        self.instruments.sent(started);
        self.input.update(cx, |editor, cx| editor.clear(window, cx));
        self.images.clear();
        self.thread.push_user(&text);
        self.review.begin_turn();
        self.notice = None;
        self.set_turn(Turn::Queued);
        cx.notify();
    }

    fn send_prompt(
        &mut self,
        prompt: Prompt,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<(), String> {
        if self.connection == Connection::NotStarted {
            self.start(window, cx);
        }
        match (&self.connection, &self.link) {
            (Connection::Stopped(reason), _) => Err(format!("{reason}; restart OMP")),
            (Connection::Taken { reason, .. }, _) => {
                Err(format!("{reason}; start a new session or retry"))
            }
            (_, Some(link)) => link.send(prompt),
            (_, None) => Err("OMP is not running; restart it".to_string()),
        }
    }

    /// An inline edit from the editor (ADR-0056 decision 3): a turn in the
    /// live session, or a follow-up with its chip while a turn runs.
    pub fn inline_edit(
        &mut self,
        edit: InlineEdit,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let message = edit.message(cx);
        match self.turn {
            Turn::Idle => {
                let started = Instant::now();
                let prompt = Prompt {
                    text: message.clone(),
                    images: Vec::new(),
                };
                if let Err(e) = self.send_prompt(prompt, window, cx) {
                    self.notice = Some(e);
                    cx.notify();
                    return false;
                }
                self.instruments.sent(started);
                self.thread.push_user(&message);
                self.begin_edit_turn(edit, cx);
                self.notice = None;
                self.set_turn(Turn::Queued);
            }
            Turn::Streaming => {
                let Some(link) = &self.link else {
                    self.notice = Some("OMP is not running; restart it".to_string());
                    cx.notify();
                    return false;
                };
                link.queue(message.clone(), false);
                self.follow_ups.push(QueuedFollowUp {
                    message,
                    edit: Some(edit),
                    shown: None,
                    promoting: false,
                });
            }
            _ => {
                self.notice = Some("the turn is starting or stopping; wait for it".to_string());
                cx.notify();
                return false;
            }
        }
        cx.notify();
        true
    }

    /// ctrl-enter in an editor: the instruction block above the selection
    /// (the cursor's line when nothing is selected), focused.
    pub fn open_instruction(
        &mut self,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(open) = &self.instruction
            && (open.pending.is_some() || !open.input.read(cx).text(cx).trim().is_empty())
        {
            let input = open.input.clone();
            self.notice = Some(format!(
                "an inline edit of {} is open: send it or press Escape first",
                open.edit.path
            ));
            input.focus_handle(cx).focus(window, cx);
            cx.notify();
            return;
        }
        self.close_instruction(cx);
        let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton() else {
            self.notice = Some("inline edit works in a single file's editor".to_string());
            cx.notify();
            return;
        };
        let Some(path) = buffer
            .read(cx)
            .file()
            .map(|f| f.path().as_unix_str().to_string())
        else {
            self.notice = Some("inline edit needs a file in the project".to_string());
            cx.notify();
            return;
        };
        let selected = editor.update(cx, |editor, cx| {
            let snapshot = editor.display_snapshot(cx);
            editor
                .selections
                .newest::<language::Point>(&snapshot)
                .range()
        });
        let (start, end) = if selected.is_empty() {
            let row = selected.start.row;
            let max = buffer.read(cx).max_point();
            let end = if row < max.row {
                language::Point::new(row + 1, 0)
            } else {
                max
            };
            (language::Point::new(row, 0), end)
        } else {
            (selected.start, selected.end)
        };
        let input = cx.new(|cx| {
            let mut input = Editor::single_line(window, cx);
            input.set_placeholder_text("Tell OMP how to edit the selection", window, cx);
            input
        });
        let panel = cx.entity().downgrade();
        let child = input.clone();
        let block = editor.update(cx, |editor, cx| {
            let anchor = editor.buffer().read(cx).snapshot(cx).anchor_before(start);
            editor.insert_blocks(
                [BlockProperties {
                    placement: BlockPlacement::Above(anchor),
                    height: Some(2),
                    style: BlockStyle::Sticky,
                    render: Arc::new(move |_| {
                        let (submit, cancel) = (panel.clone(), panel.clone());
                        div()
                            .debug_selector(|| "cedian-inline-edit".to_string())
                            .w_full()
                            .capture_action(move |_: &editor::actions::Newline, window, cx| {
                                cx.stop_propagation();
                                submit
                                    .update(cx, |p, cx| p.submit_instruction(window, cx))
                                    .ok();
                            })
                            .capture_action(move |_: &editor::actions::Cancel, window, cx| {
                                cx.stop_propagation();
                                cancel
                                    .update(cx, |p, cx| p.dismiss_instruction(window, cx))
                                    .ok();
                            })
                            .child(child.clone())
                            .into_any_element()
                    }),
                    priority: 0,
                }],
                None,
                cx,
            )[0]
        });
        input.focus_handle(cx).focus(window, cx);
        self.instruction = Some(InstructionBlock {
            host: editor.downgrade(),
            block,
            input,
            edit: InlineEdit::new(path, buffer, start..end, cx),
            pending: None,
        });
        cx.notify();
    }

    /// Enter in the instruction block: the inline edit goes to OMP; the
    /// block stays, with its text, when it could not be sent.
    fn submit_instruction(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(block) = &self.instruction else {
            return;
        };
        let instruction = block.input.read(cx).text(cx);
        if instruction.trim().is_empty() || block.pending.is_some() {
            return;
        }
        let edit = InlineEdit {
            instruction,
            ..block.edit.clone()
        };
        let queued = (self.turn == Turn::Streaming).then(|| edit.message(cx));
        if !self.inline_edit(edit, window, cx) {
            return;
        }
        match (queued, &mut self.instruction) {
            (Some(message), Some(block)) => block.pending = Some(message),
            _ => self.dismiss_instruction(window, cx),
        }
    }

    /// The block goes and the editor it was in has the focus again.
    fn dismiss_instruction(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let host = self.instruction.as_ref().and_then(|b| b.host.upgrade());
        self.close_instruction(cx);
        if let Some(host) = host {
            host.focus_handle(cx).focus(window, cx);
        }
    }

    /// Escape in the instruction block, or a new one opened: it goes.
    fn close_instruction(&mut self, cx: &mut Context<Self>) {
        let Some(block) = self.instruction.take() else {
            return;
        };
        if let Some(host) = block.host.upgrade() {
            host.update(cx, |editor, cx| {
                editor.remove_blocks([block.block].into_iter().collect(), None, cx);
            });
        }
        cx.notify();
    }

    /// The instruction block is open.
    pub fn instruction_open(&self) -> bool {
        self.instruction.is_some()
    }

    fn begin_edit_turn(&mut self, edit: InlineEdit, cx: &App) {
        let turn = self.review.begin_edit_turn(edit.label(cx));
        self.edit_turn = Some((turn, edit));
    }

    /// The inline edit's turn ended: each of its hunks outside the
    /// selection is named, never rejected (ADR-0056 decision 6).
    fn warn_outside_selection(&mut self, cx: &mut Context<Self>) {
        let Some((turn, edit)) = self.edit_turn.take() else {
            return;
        };
        let (first, last) = edit.lines(cx);
        let mut outside = Vec::new();
        for file in self.review.files() {
            let calls: Vec<&String> = file
                .agent_txns()
                .iter()
                .filter(|t| t.turn == turn && t.restored.is_none())
                .map(|t| &t.tool_call_id)
                .collect();
            for hunk in file.hunks() {
                if !hunk.tool_call_ids.iter().any(|id| calls.contains(&id)) {
                    continue;
                }
                let (start, end) = (hunk.rows.start + 1, hunk.rows.end.max(hunk.rows.start + 1));
                let same = file.buffer() == &edit.buffer;
                if !same || start < first || end > last || edit.added_outside(hunk, cx) {
                    let path = file.buffer().read(cx).file().map_or_else(
                        || self.review.path(file, cx).display().to_string(),
                        |f| f.path().as_unix_str().to_string(),
                    );
                    outside.push(format!("{path}:{start}-{end}"));
                }
            }
        }
        if outside.is_empty() {
            return;
        }
        self.add_notice(format!(
            "the inline edit changed outside its selection: {}; review it in Review Changes",
            outside.join(", ")
        ));
    }

    /// OMP's queue changed: each follow-up of the panel's that left it
    /// started running, so its own turn begins in the review; several
    /// leaving at once (OMP's `followUpMode = all`) run as one turn. While
    /// Stop takes the queue back, OMP lists it empty before it answers
    /// `abort_and_restore_queue`: those go through `take_back`, not here.
    fn observe_follow_ups(
        &mut self,
        steering: &[String],
        follow_up: &[String],
        cx: &mut Context<Self>,
    ) {
        if self.turn == Turn::Stopping {
            return;
        }
        let mut listed: Vec<&String> = follow_up.iter().collect();
        let mut kept = vec![false; self.follow_ups.len()];
        // OMP runs the oldest first: of identical texts, the newest stay.
        for (i, queued) in self.follow_ups.iter_mut().enumerate().rev() {
            let text = queued.shown.as_ref().unwrap_or(&queued.message);
            if let Some(j) = listed.iter().rposition(|m| *m == text) {
                queued.shown = Some(listed.remove(j).clone());
                kept[i] = true;
            }
        }
        // Listed under a text no follow-up has: a slash command or template
        // OMP expanded, matched oldest to oldest (as Stop's take-back does).
        for (i, queued) in self.follow_ups.iter_mut().enumerate() {
            if !kept[i] && queued.shown.is_none() && !listed.is_empty() {
                queued.shown = Some(listed.remove(0).clone());
                kept[i] = true;
            }
        }
        // Steer now that OMP did: it left for the steering queue and joins
        // the running turn.
        let mut steering: Vec<&String> = steering.iter().collect();
        let mut drained = Vec::new();
        for i in (0..self.follow_ups.len()).rev() {
            if kept[i] || self.follow_ups[i].shown.is_none() {
                continue;
            }
            let queued = self.follow_ups.remove(i);
            let text = queued.shown.as_ref().unwrap_or(&queued.message);
            match steering.iter().position(|m| *m == text) {
                Some(j) if queued.promoting => {
                    steering.remove(j);
                }
                _ => drained.push(queued),
            }
        }
        if drained.is_empty() {
            return;
        }
        self.warn_outside_selection(cx);
        self.review.end_turn();
        if drained.len() > 1 {
            let turn = self.review.begin_turn();
            self.add_notice(format!(
                "OMP ran {} queued messages as one turn (turn {turn})",
                drained.len()
            ));
        } else if let Some(edit) = drained.pop().and_then(|q| q.edit) {
            self.begin_edit_turn(edit, cx);
        } else {
            self.review.begin_turn();
        }
    }

    /// The Steer button: `text` goes into the running turn.
    pub fn steer(&mut self, cx: &mut Context<Self>) {
        if self.turn == Turn::Streaming {
            self.queue(true, cx);
        }
    }

    /// Steer (`steer`) or queue after the turn (`follow_up`, ADR-0050
    /// decision 3). The chip comes from OMP's own `queue_update`.
    fn queue(&mut self, steer: bool, cx: &mut Context<Self>) {
        let text = self.input.read(cx).text(cx);
        if text.trim().is_empty() {
            return;
        }
        let Some(link) = &self.link else {
            self.notice = Some("OMP is not running; restart it".to_string());
            cx.notify();
            return;
        };
        if !steer {
            self.follow_ups.push(QueuedFollowUp {
                message: text.clone(),
                edit: None,
                shown: None,
                promoting: false,
            });
        }
        link.queue(text, steer);
        self.notice = None;
        cx.notify();
    }

    fn on_event(&mut self, event: RouterEvent, cx: &mut Context<Self>) {
        match &event {
            RouterEvent::ToolStart {
                tool_call_id,
                tool_name,
                paths,
                ..
            } if EDIT_TOOLS.contains(&tool_name.as_str()) => {
                self.mark_before_write(tool_call_id.clone(), paths, cx);
            }
            RouterEvent::ToolStart {
                tool_call_id,
                tool_name,
                ..
            } if tool_name == BASH_TOOL => self.mark_for_bash(tool_call_id.clone(), cx),
            RouterEvent::ToolEnd {
                tool_call_id,
                tool_name,
                is_error,
                before,
                ..
            } => {
                if tool_name == BROWSER_TOOL {
                    self.observe_browser_call(tool_call_id.clone(), *is_error, cx);
                }
                let root = self.workspace_root(cx);
                if let Some(call) = self.calls.get_mut(tool_call_id) {
                    call.ended = Some(*is_error);
                    call.before = before
                        .iter()
                        .filter_map(|(path, text)| {
                            let path = Path::new(path);
                            let path = if path.is_absolute() {
                                path.to_path_buf()
                            } else {
                                root.as_ref()?.join(path)
                            };
                            Some((path, text.clone()))
                        })
                        .collect();
                    self.import_when_ready(tool_call_id.clone(), cx);
                }
            }
            // OMP started a run of its own (a queued steer it drains): it
            // shows as running so Stop can stop it, also while an earlier
            // Stop still waits for OMP to settle.
            RouterEvent::AgentStart
                if matches!(self.turn, Turn::Queued | Turn::Idle)
                    || (self.turn == Turn::Stopping
                        && self.link.as_ref().is_some_and(OmpLink::runs_own)) =>
            {
                self.set_turn(Turn::Streaming)
            }
            RouterEvent::Queue {
                steering,
                follow_up,
            } => self.observe_follow_ups(steering, follow_up, cx),
            RouterEvent::Settled => {
                self.retrying = false;
                // OMP holds no follow-up once settled.
                self.follow_ups.retain(|q| q.shown.is_none());
                self.review.end_turn();
                if let Turn::Failed(reason) = std::mem::replace(&mut self.turn, Turn::Idle) {
                    self.notice = Some(format!("turn failed: {reason}"));
                }
                self.warn_outside_selection(cx);
                self.end_workflow_turn(cx);
                self.refresh_workflow(cx);
            }
            _ => {}
        }
        self.subagents.apply(&event);
        // A subagent ending is when a worker it started usually finishes: the
        // row's class and status show up without the person clicking Refresh.
        if matches!(event, RouterEvent::SubagentLifecycle { .. }) {
            self.refresh_worker_trees(cx);
        }
        self.thread.apply(&event);
        cx.notify();
    }

    /// Turn boundary (ADR-0036): a claim refused in the turn that just
    /// ended blocks the workflow, and the panel says which gates are unmet.
    fn end_workflow_turn(&mut self, cx: &mut Context<Self>) {
        if self.state_dir.is_none() && self.workspace_root(cx).is_none() {
            return;
        }
        let turn = Some(self.review.current_turn());
        let said = match self
            .state_dir(cx)
            .and_then(|dir| cedian_shell::workflow_host::end_turn(&dir, TASK_ID, turn))
        {
            Ok(None) => return,
            Ok(Some((status, missing))) => format!(
                "workflow {}: the agent claimed done with required gates unmet: {}",
                if status == cedian_workflow::WorkflowStatus::Failed {
                    "failed"
                } else {
                    "blocked"
                },
                missing.join("; ")
            ),
            Err(e) => format!("the workflow could not end its turn: {e}"),
        };
        self.notice = Some(match self.notice.take() {
            Some(notice) => format!("{notice}; {said}"),
            None => said,
        });
    }

    /// The task's workflow as the panel shows it.
    pub fn workflow(&self) -> Option<&WorkflowView> {
        self.workflow.as_ref()
    }

    /// Read the workflow again, off the UI thread: its gates are evaluated
    /// against the workspace hashed now (row H).
    fn refresh_workflow(&mut self, cx: &mut Context<Self>) {
        self.load_workflow(Duration::ZERO, cx);
    }

    /// Re-read the workflow after `delay`. A later load supersedes this
    /// one: its wait ends and a workspace scan already running stops.
    fn load_workflow(&mut self, delay: Duration, cx: &mut Context<Self>) {
        self._workflow_load = Some(cx.spawn(async move |this, cx| {
            if !delay.is_zero() {
                cx.background_executor().timer(delay).await;
            }
            let stop = Arc::new(AtomicBool::new(false));
            let _stop_on_drop = util::defer({
                let stop = stop.clone();
                move || stop.store(true, Ordering::Relaxed)
            });
            let Ok(Some(read)) = this.update(cx, |this, cx| this.read_workflow(stop, cx)) else {
                return;
            };
            let view = read.await;
            this.update(cx, |this, cx| {
                match view {
                    Ok(Some(view)) => this.workflow = view,
                    Ok(None) => {}
                    Err(e) => this.notice = Some(format!("the workflow could not be read: {e}")),
                }
                cx.notify();
            })
            .ok();
        }));
    }

    /// `Ok(None)`: keep the view (finished and shown, or the scan stopped).
    fn read_workflow(
        &mut self,
        stop: Arc<AtomicBool>,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<Option<Option<WorkflowView>>, String>>> {
        let (Ok(dir), Some(root)) = (self.state_dir(cx), self.workspace_root(cx)) else {
            return None;
        };
        let live = |status| matches!(status, WorkflowStatus::Running | WorkflowStatus::Blocked);
        // A finished workflow's gates no longer change: once shown, it is
        // not hashed again on every turn.
        let shown_finished = self.workflow.as_ref().is_some_and(|v| !live(v.status));
        let frame_seq = self.current_state().frame_seq;
        let unsaved = self.unsaved.lock().clone();
        self.workflow_reads += 1;
        Some(cx.background_spawn(async move {
            if !cedian_shell::workflow_store::exists(&dir) {
                return Ok(Some(None));
            }
            let state = cedian_shell::workflow_store::load(&dir)?;
            if shown_finished && !live(state.status) {
                return Ok(None);
            }
            let Some(mut current) = cedian_shell::workflow_host::current_state_unless(&root, &stop)
            else {
                return Ok(None);
            };
            current.frame_seq = frame_seq;
            current.overlay(&unsaved);
            Ok::<_, String>(Some(Some(WorkflowView::new(&state, &current))))
        }))
    }

    /// §54: during a workflow, a question nobody answered blocks its
    /// current phase and waits for the person's Resume.
    fn escalate(&mut self, why: &str, cx: &mut Context<Self>) {
        if self.state_dir.is_none() && self.workspace_root(cx).is_none() {
            return;
        }
        let turn = Some(self.review.current_turn());
        match self
            .state_dir(cx)
            .and_then(|dir| cedian_shell::workflow_host::escalate(&dir, TASK_ID, turn, why))
        {
            Ok(true) => {
                self.escalation = Some(format!(
                    "{why}: the workflow is blocked at its current phase until you resume it"
                ));
                self.refresh_workflow(cx);
            }
            Ok(false) => {}
            Err(e) => self.escalation = Some(format!("{why}; the workflow could not block: {e}")),
        }
    }

    /// The Resume button: the person's answer to a blocked workflow (§54).
    pub fn resume_workflow(&mut self, cx: &mut Context<Self>) {
        let resumed = self
            .state_dir(cx)
            .and_then(|dir| cedian_shell::workflow_host::resume(&dir));
        match resumed {
            Ok(()) => self.escalation = None,
            Err(e) => self.notice = Some(format!("the workflow did not resume: {e}")),
        }
        self.refresh_workflow(cx);
        cx.notify();
    }

    /// Compaction and retry notes, Stop retry, Steer now on each queued
    /// follow-up, and the session's queue, compaction and retry settings
    /// while the picker is open.
    fn render_session(&self, cx: &mut Context<Self>) -> AnyElement {
        use crate::omp_link::SessionSetting as S;
        use omp_rpc::{InterruptMode, QueueMode};
        let setting = |id: &'static str, label: &'static str, setting: S| {
            div().debug_selector(move || id.to_string()).child(
                Button::new(id, label)
                    .on_click(cx.listener(move |this, _, _, cx| this.set_session(setting, cx))),
            )
        };
        let notes = self.session_notes.iter().enumerate().map(|(i, note)| {
            div()
                .debug_selector(move || format!("cedian-session-note-{i}"))
                .child(
                    Label::new(note.clone())
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
        });
        let chips = self
            .follow_ups
            .iter()
            .enumerate()
            .filter(|(_, q)| q.edit.is_none())
            .map(|(i, q)| {
                h_flex()
                    .gap_1()
                    .child(Label::new(format!("queued: {}", q.message)).size(LabelSize::Small))
                    .child(
                        div()
                            .debug_selector(move || format!("cedian-promote-{i}"))
                            .child(Button::new(("cedian-promote", i), "Steer now").on_click(
                                cx.listener(move |this, _, _, cx| this.promote_follow_up(i, cx)),
                            )),
                    )
            });
        v_flex()
            .gap_1()
            .children(notes)
            .children(chips)
            .when(self.retrying, |el| {
                el.child(setting("cedian-abort-retry", "Stop retry", S::AbortRetry))
            })
            .when(self.picker.open, |el| {
                el.child(
                    h_flex()
                        .flex_wrap()
                        .gap_1()
                        .child(setting(
                            "cedian-steering-all",
                            "Steer: all",
                            S::SteeringMode(QueueMode::All),
                        ))
                        .child(setting(
                            "cedian-steering-one",
                            "Steer: one at a time",
                            S::SteeringMode(QueueMode::OneAtATime),
                        ))
                        .child(setting(
                            "cedian-follow-up-all",
                            "Follow-up: all",
                            S::FollowUpMode(QueueMode::All),
                        ))
                        .child(setting(
                            "cedian-follow-up-one",
                            "Follow-up: one at a time",
                            S::FollowUpMode(QueueMode::OneAtATime),
                        ))
                        .child(setting(
                            "cedian-interrupt-immediate",
                            "Interrupt: immediate",
                            S::InterruptMode(InterruptMode::Immediate),
                        ))
                        .child(setting(
                            "cedian-interrupt-wait",
                            "Interrupt: wait",
                            S::InterruptMode(InterruptMode::Wait),
                        ))
                        .child(setting("cedian-compact", "Compact now", S::Compact))
                        .child(setting(
                            "cedian-auto-compaction-off",
                            "Auto-compact off",
                            S::AutoCompaction(false),
                        ))
                        .child(setting(
                            "cedian-auto-compaction-on",
                            "Auto-compact on",
                            S::AutoCompaction(true),
                        ))
                        .child(setting(
                            "cedian-auto-retry-off",
                            "Auto-retry off",
                            S::AutoRetry(false),
                        ))
                        .child(setting(
                            "cedian-auto-retry-on",
                            "Auto-retry on",
                            S::AutoRetry(true),
                        )),
                )
            })
            .into_any_element()
    }

    fn render_picker(&self, cx: &mut Context<Self>) -> AnyElement {
        let picker = &self.picker;
        let level_name = |l: Option<omp_rpc::ThinkingLevel>| l.map_or("?", |l| l.as_str());
        let summary = format!(
            "Model: {} · Thinking: {}",
            picker.model.as_deref().unwrap_or("?"),
            level_name(picker.thinking)
        );
        let button = |id: String, label: String| {
            div()
                .debug_selector({
                    let id = id.clone();
                    move || id
                })
                .child(Button::new(SharedString::from(id), label))
        };
        let mut panel = v_flex().gap_1().child(
            div()
                .debug_selector(|| "cedian-picker-toggle".to_string())
                .child(
                    Button::new("cedian-picker-toggle", summary)
                        .on_click(cx.listener(|this, _, _, cx| this.toggle_picker(cx))),
                ),
        );
        if !picker.open {
            return panel.into_any_element();
        }
        let models = picker.models.iter().map(|model| {
            let key = crate::omp_link::model_key(model);
            let (provider, id) = (model.provider.clone(), model.id.clone());
            button(format!("cedian-model-{key}"), key).on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    if let Some(link) = &this.link {
                        link.set_model(provider.clone(), id.clone());
                    }
                    cx.notify();
                }),
            )
        });
        let levels = picker.levels.iter().map(|level| {
            let level = *level;
            button(
                format!("cedian-thinking-{}", level.as_str()),
                level.as_str().to_string(),
            )
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    if let Some(link) = &this.link {
                        link.set_thinking_level(level);
                    }
                    cx.notify();
                }),
            )
        });
        panel = panel
            .child(h_flex().flex_wrap().gap_1().children(models).child(
                button("cedian-cycle-model".into(), "Next model".into()).on_mouse_down(
                    gpui::MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        if let Some(link) = &this.link {
                            link.cycle_model();
                        }
                        cx.notify();
                    }),
                ),
            ))
            .child(h_flex().flex_wrap().gap_1().children(levels).child(
                button("cedian-cycle-thinking".into(), "Next level".into()).on_mouse_down(
                    gpui::MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        if let Some(link) = &this.link {
                            link.cycle_thinking_level();
                        }
                        cx.notify();
                    }),
                ),
            ))
            .when_some(picker.error.clone(), |panel, error| {
                panel.child(Label::new(error).size(LabelSize::Small).color(Color::Error))
            });
        panel.into_any_element()
    }

    /// OMP's sign-in page for a login the person started opens in the
    /// system browser, never the agent's Chromium (ADR-0049): there the
    /// agent could read the page and its tokens. Any other `open_url` is
    /// only shown, so OMP or an extension cannot open pages unasked.
    fn open_url(&mut self, request: omp_rpc::OpenUrlUiRequest, cx: &mut Context<Self>) {
        match &mut self.onboarding {
            Some(onboarding) if onboarding.running.is_some() => match login_link(&request.url) {
                Ok(link) => {
                    cx.open_url(&link);
                    onboarding.url = Some(request);
                }
                Err(e) => self.push_toast(cedian_omp::Toast {
                    level: cedian_omp::ToastLevel::Warning,
                    text: format!("OMP's login: {e}"),
                }),
            },
            _ => self.push_toast(cedian_omp::Toast {
                level: cedian_omp::ToastLevel::Info,
                text: format!("OMP asks to open {}", request.url),
            }),
        }
    }

    fn push_toast(&mut self, toast: cedian_omp::Toast) {
        self.toasts.push((self.next_toast, toast));
        self.next_toast += 1;
        let excess = self.toasts.len().saturating_sub(MAX_TOASTS);
        self.toasts.drain(..excess);
    }

    fn start_login(&mut self, provider: String, cx: &mut Context<Self>) {
        let (Some(link), Some(onboarding)) = (&self.link, &mut self.onboarding) else {
            return;
        };
        if onboarding.running.is_some() {
            return;
        }
        onboarding.running = Some(provider.clone());
        onboarding.error = None;
        link.login(provider);
        cx.notify();
    }

    /// Cancel on a running login: OMP has no call to stop one, so its
    /// open prompts are dismissed (OMP's login then fails on its own) and
    /// the view leaves "signing in…" now.
    fn cancel_login(&mut self, cx: &mut Context<Self>) {
        let Some(onboarding) = &mut self.onboarding else {
            return;
        };
        onboarding.running = None;
        onboarding.url = None;
        for id in std::mem::take(&mut onboarding.dialogs) {
            if self.dialogs.contains_key(&id) {
                self.answer(&id, UserAnswer::Dismiss, cx);
            }
        }
        cx.notify();
    }

    /// No model yet: OMP's providers, each with Log in, and the running
    /// login's sign-in link.
    fn render_onboarding(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let onboarding = self.onboarding.as_ref()?;
        let running = onboarding.running.clone();
        let providers = onboarding.providers.iter().map(|provider| {
            let selector = format!("cedian-login-{}", provider.id);
            let label = if running.as_deref() == Some(provider.id.as_str()) {
                format!("{}: signing in…", provider.name)
            } else if provider.authenticated {
                format!("{} (signed in)", provider.name)
            } else {
                format!("Log in with {}", provider.name)
            };
            let id = provider.id.clone();
            div()
                .debug_selector({
                    let selector = selector.clone();
                    move || selector
                })
                .child(
                    Button::new(SharedString::from(selector), label)
                        .disabled(!provider.available || running.is_some())
                        .on_click(
                            cx.listener(move |this, _, _, cx| this.start_login(id.clone(), cx)),
                        ),
                )
        });
        Some(
            v_flex()
                .debug_selector(|| "cedian-onboarding".to_string())
                .gap_1()
                .child(Label::new(
                    "OMP has no model to use yet: log in to a provider.",
                ))
                .child(h_flex().flex_wrap().gap_1().children(providers))
                .when(running.is_some(), |panel| {
                    panel.child(
                        div()
                            .debug_selector(|| "cedian-login-cancel".to_string())
                            .child(
                                Button::new("cedian-login-cancel", "Cancel")
                                    .on_click(cx.listener(|this, _, _, cx| this.cancel_login(cx))),
                            ),
                    )
                })
                .when_some(onboarding.url.clone(), |panel, url| {
                    // The shown link is the one opened: `launchUrl` is OMP's
                    // truncation-safe copy of `url`.
                    let Ok(link) = login_link(url.launch_url.as_deref().unwrap_or(&url.url)) else {
                        return panel;
                    };
                    let shown = link.clone();
                    panel.child(
                        v_flex()
                            .debug_selector(|| "cedian-login-url".to_string())
                            .when_some(url.instructions, |panel, text| {
                                panel.child(Label::new(text).size(LabelSize::Small))
                            })
                            .child(
                                Button::new("cedian-login-url", shown)
                                    .on_click(move |_, _, cx| cx.open_url(&link)),
                            ),
                    )
                })
                .when_some(onboarding.error.clone(), |panel, error| {
                    panel.child(Label::new(error).size(LabelSize::Small).color(Color::Error))
                })
                .into_any_element(),
        )
    }

    /// OMP's notices, one row each with Dismiss.
    fn render_toasts(&self, cx: &mut Context<Self>) -> Vec<AnyElement> {
        self.toasts
            .iter()
            .enumerate()
            .map(|(i, (id, toast))| {
                let id = *id;
                let color = match toast.level {
                    cedian_omp::ToastLevel::Info => Color::Muted,
                    cedian_omp::ToastLevel::Warning => Color::Warning,
                    cedian_omp::ToastLevel::Error => Color::Error,
                };
                h_flex()
                    .debug_selector(move || format!("cedian-toast-{i}"))
                    .gap_2()
                    .child(
                        Label::new(toast.text.clone())
                            .size(LabelSize::Small)
                            .color(color),
                    )
                    .child(
                        div()
                            .debug_selector(move || format!("cedian-toast-{i}-dismiss"))
                            .child(
                                Button::new(("cedian-toast-dismiss", i), "Dismiss").on_click(
                                    cx.listener(move |this, _, _, cx| {
                                        this.dismiss_toast(id);
                                        cx.notify();
                                    }),
                                ),
                            ),
                    )
                    .into_any_element()
            })
            .collect()
    }

    fn render_workflow(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let view = self.workflow.as_ref()?;
        let line = |text: String, color: Color| {
            Label::new(text)
                .size(LabelSize::Small)
                .color(color)
                .into_any_element()
        };
        let phases = view
            .phases
            .iter()
            .map(|(id, mark)| format!("{mark} {id}"))
            .collect::<Vec<_>>()
            .join("  ");
        let gates = view.gates.iter().map(|g| {
            let id = g.id.clone();
            div()
                .debug_selector(move || format!("cedian-workflow-gate-{id}"))
                .child(line(
                    format!(
                        "{}{} {:?}: {}",
                        g.id,
                        if g.required { " (required)" } else { "" },
                        g.status,
                        g.reason
                    ),
                    if g.status == cedian_workflow::GateStatus::Passed {
                        Color::Default
                    } else {
                        Color::Warning
                    },
                ))
                .into_any_element()
        });
        let blocked = view.status == cedian_workflow::WorkflowStatus::Blocked;
        Some(
            v_flex()
                .debug_selector(|| "cedian-workflow".to_string())
                .gap_1()
                .child(
                    h_flex()
                        .gap_2()
                        .child(
                            Label::new(format!("Workflow · {} · {:?}", view.title, view.status))
                                .size(LabelSize::Small),
                        )
                        .when(blocked, |row| {
                            row.child(
                                div()
                                    .debug_selector(|| "cedian-workflow-resume".to_string())
                                    .child(
                                        Button::new("cedian-workflow-resume", "Resume").on_click(
                                            cx.listener(|this, _, _, cx| this.resume_workflow(cx)),
                                        ),
                                    ),
                            )
                        }),
                )
                .when_some(self.escalation.clone().filter(|_| blocked), |col, why| {
                    col.child(
                        div()
                            .debug_selector(|| "cedian-workflow-escalation".to_string())
                            .child(line(why, Color::Warning)),
                    )
                })
                .child(line(phases, Color::Muted))
                .children(gates)
                .children(view.evidence.iter().map(|e| line(e.clone(), Color::Muted)))
                .children(
                    view.claims
                        .iter()
                        .flat_map(|c| c.lines())
                        .map(|l| line(l.to_string(), Color::Muted)),
                )
                .into_any_element(),
        )
    }

    /// The Stop button: close the dialogs OMP waits on and abort the
    /// running turn. The turn goes idle when OMP reports the session settled.
    pub fn stop_turn(&mut self, cx: &mut Context<Self>) {
        if !matches!(self.turn, Turn::Queued | Turn::Streaming) {
            return;
        }
        self.set_turn(Turn::Stopping);
        let closed = self.link.as_ref().map(OmpLink::close_dialogs);
        self.dialogs.clear();
        match closed {
            Some(Err(e)) => self.audit_failed(e, cx),
            _ => self.cancel(),
        }
        cx.notify();
    }

    /// ADR-0035: an unaudited turn fails. The first failure aborts it; later
    /// ones in the same turn change nothing.
    fn audit_failed(&mut self, e: String, cx: &mut Context<Self>) {
        let reason = format!("audit: {e}");
        match self.turn {
            Turn::Failed(_) => {}
            Turn::Idle => self.notice = Some(reason),
            Turn::Queued | Turn::Streaming | Turn::Stopping => {
                self.set_turn(Turn::Failed(reason));
                self.cancel();
            }
        }
        cx.notify();
    }

    fn cancel(&self) {
        if let Some(link) = &self.link {
            link.cancel();
        }
    }

    /// Answer dialog `id` as its buttons do. An answer OMP did not get
    /// leaves the dialog open with the reason on it.
    pub fn answer(&mut self, id: &str, answer: UserAnswer, cx: &mut Context<Self>) {
        let sent = match &self.link {
            Some(link) => link.answer(id, answer),
            None => Err(AnswerError::NotSent("OMP is not running".to_string())),
        };
        match sent {
            Ok(()) => {
                self.dialogs.shift_remove(id);
            }
            Err(AnswerError::NotSent(e)) => {
                if let Some(dialog) = self.dialogs.get_mut(id) {
                    dialog.error = Some(e);
                }
            }
            Err(AnswerError::Unaudited(e)) => {
                self.dialogs.shift_remove(id);
                self.audit_failed(e, cx);
            }
        }
        cx.notify();
    }

    /// The dialog's lease ran out: OMP is told it timed out, and the panel
    /// says so where the dialog was.
    fn expire(&mut self, id: &str, cx: &mut Context<Self>) {
        let Some(dialog) = self.dialogs.shift_remove(id) else {
            return;
        };
        let closed = match &self.link {
            Some(link) => link.expire(id),
            None => Ok(false),
        };
        match closed {
            Ok(true) => {
                let why = format!(
                    "\"{}\" got no answer in {} minutes; cedian dismissed it",
                    dialog.title().lines().next().unwrap_or_default(),
                    DIALOG_TIMEOUT.as_secs() / 60
                );
                self.escalate(&why, cx);
                self.notice = Some(why);
            }
            Ok(false) => {}
            Err(e) => self.audit_failed(e, cx),
        }
        cx.notify();
    }

    /// The person's Allow/Deny on a worktree request (ADR-0012): audit the
    /// answer, resolve the blocked host-tool thread, and clear the bar. A
    /// late answer — the lease already ran out — only says so.
    pub fn answer_worktree(&mut self, allow: bool, cx: &mut Context<Self>) {
        let Some((request, id, goal)) = self.worktree_ask.take() else {
            self.notice = Some("that worktree request was already answered".to_string());
            cx.notify();
            return;
        };
        if let Some(link) = &self.link {
            link.worktree_decision(&id, &goal, allow, false);
        }
        if let Some(gate) = &self.worktree_gate {
            gate.answer(request, allow);
        }
        self._worktree_expiry = None;
        cx.notify();
    }

    /// The worktree request's lease ran out: cedian denies it, resolves the
    /// blocked host-tool thread, and says nobody answered (§63, ADR-0013).
    fn expire_worktree(&mut self, cx: &mut Context<Self>) {
        let Some((request, id, goal)) = self.worktree_ask.take() else {
            return;
        };
        if let Some(link) = &self.link {
            link.worktree_decision(&id, &goal, false, true);
        }
        if let Some(gate) = &self.worktree_gate {
            gate.answer(request, false);
        }
        self._worktree_expiry = None;
        self.notice = Some(format!(
            "the worktree request for {id} got no answer in {} minutes; cedian denied it",
            DIALOG_TIMEOUT.as_secs() / 60
        ));
        cx.notify();
    }

    fn submit_dialog(&mut self, id: &str, cx: &mut Context<Self>) {
        let Some(dialog) = self.dialogs.get_mut(id) else {
            return;
        };
        match dialog.submission(cx) {
            Ok(answer) => self.answer(id, answer, cx),
            Err(e) => {
                dialog.error = Some(e);
                cx.notify();
            }
        }
    }

    /// Paste into the composer: clipboard images attach to the next
    /// prompt and any text goes on to the editor. Elsewhere in the panel
    /// (a dialog's text box) paste is the editor's alone.
    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        cx.propagate();
        if !self.input.focus_handle(cx).is_focused(window) {
            return;
        }
        let Some(item) = cx.read_from_clipboard() else {
            return;
        };
        let has_text = item.text().is_some();
        let images: Vec<_> = item
            .into_entries()
            .filter_map(|entry| match entry {
                ClipboardEntry::Image(image) => Some(cedian_omp::image_content(
                    image.format.mime_type(),
                    &image.bytes,
                )),
                _ => None,
            })
            .collect();
        if images.is_empty() {
            return;
        }
        if !has_text {
            cx.stop_propagation();
        }
        self.images.extend(images);
        cx.notify();
    }

    fn render_subagent(&self, row: &SubagentRow, cx: &mut Context<Self>) -> AnyElement {
        let id = row.id.clone();
        let selector = format!("cedian-subagent-{id}");
        type OnClick = fn(&mut CedianPanel, &str, &mut Window, &mut Context<CedianPanel>);
        let button = |label: &'static str, suffix: &str, on: OnClick, cx: &mut Context<Self>| {
            let selector = format!("cedian-subagent-{id}-{suffix}");
            let element = ElementId::Name(selector.clone().into());
            let id = id.clone();
            div().debug_selector(move || selector).child(
                Button::new(element, label)
                    .on_click(cx.listener(move |this, _, window, cx| on(this, &id, window, cx))),
            )
        };
        let controls = (row.status == SubagentStatus::Running)
            .then(|| self.steer_boxes.get(&row.id).cloned())
            .flatten()
            .map(|input| {
                h_flex()
                    .gap_2()
                    .child(div().flex_1().child(input))
                    .child(button("Steer", "steer", Self::steer_subagent, cx))
                    .child(button(
                        "Cancel",
                        "cancel",
                        |this, id, _, cx| this.cancel_subagent(id, cx),
                        cx,
                    ))
            });
        v_flex()
            .debug_selector(move || selector)
            .pl_4()
            .child(Label::new(subagent_line(row)).size(LabelSize::Small))
            .children(controls)
            .children(self.subagent_notes.get(&row.id).map(|note| {
                Label::new(note.clone())
                    .size(LabelSize::Small)
                    .color(Color::Warning)
            }))
            .into_any_element()
    }

    /// The Allow/Deny bar for a worktree request waiting on the person
    /// (ADR-0012). No modal primitive, so it sits with the other confirms.
    fn render_worktree_ask(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let (_, id, goal) = self.worktree_ask.as_ref()?;
        let button = |label: &str, selector: &'static str, allow: bool| {
            div().debug_selector(move || selector.to_string()).child(
                Button::new(ElementId::Name(selector.into()), label)
                    .on_click(cx.listener(move |this, _, _, cx| this.answer_worktree(allow, cx))),
            )
        };
        Some(
            v_flex()
                .debug_selector(|| "cedian-worktree-ask".to_string())
                .gap_1()
                .child(
                    Label::new(format!("OMP asks for a worktree \"{id}\": {goal}"))
                        .size(LabelSize::Small),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .child(button("Allow", "cedian-worktree-ask-allow", true))
                        .child(button("Deny", "cedian-worktree-ask-deny", false)),
                )
                .into_any_element(),
        )
    }

    fn render_dialog(&self, id: &str, dialog: &OpenDialog, cx: &mut Context<Self>) -> AnyElement {
        let button = |label: &str, answer: Option<UserAnswer>| {
            let id = id.to_string();
            let element = ElementId::Name(format!("cedian-dialog-{id}-{label}").into());
            div()
                .debug_selector(|| format!("cedian-dialog-{id}-{label}"))
                .child(
                    Button::new(element, label.to_string()).on_click(cx.listener(
                        move |this, _, _, cx| match answer.clone() {
                            Some(answer) => this.answer(&id, answer, cx),
                            None => this.submit_dialog(&id, cx),
                        },
                    )),
                )
        };
        let mut body = v_flex()
            .debug_selector(|| format!("cedian-dialog-{id}"))
            .p_2()
            .gap_1()
            .border_1()
            .border_color(cx.theme().colors().border)
            .child(Label::new(dialog.title().to_string()));
        let mut buttons = h_flex().gap_2().flex_wrap();
        match dialog.request() {
            ExtensionUiRequest::Select(r) => {
                for option in &r.options {
                    buttons =
                        buttons.child(button(option, Some(UserAnswer::Choice(option.clone()))));
                }
            }
            ExtensionUiRequest::Confirm(r) => {
                body = body.child(Label::new(r.message.clone()).color(Color::Muted));
                buttons = buttons
                    .child(button("Yes", Some(UserAnswer::Confirm(true))))
                    .child(button("No", Some(UserAnswer::Confirm(false))));
            }
            _ => {}
        }
        if let Some(ask) = dialog.ask() {
            for (n, question) in ask.questions().iter().enumerate() {
                let mut options = h_flex().gap_1().flex_wrap();
                for option in &question.options {
                    let (id, qid, label) = (id.to_string(), question.id.clone(), option.clone());
                    let selector = format!("cedian-ask-{id}-{qid}-{label}");
                    let element = ElementId::Name(selector.clone().into());
                    options = options.child(
                        div().debug_selector(|| selector).child(
                            Button::new(element, option.clone())
                                .toggle_state(question.selected.contains(option))
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    if let Some(dialog) = this.dialogs.get_mut(&id) {
                                        dialog.toggle_option(&qid, &label);
                                        dialog.error = None;
                                    }
                                    cx.notify();
                                })),
                        ),
                    );
                }
                body = body
                    .child(Label::new(question.question.clone()))
                    .child(options)
                    .when_some(dialog.custom_box(n).cloned(), |body, editor| {
                        body.child(editor)
                    });
            }
        }
        if let Some(text) = dialog.text_box() {
            body = body.child(text.clone());
        }
        if dialog.text_box().is_some() || dialog.ask().is_some() {
            buttons = buttons.child(button("Submit", None));
        }
        buttons = buttons.child(button("Dismiss", Some(UserAnswer::Dismiss)));
        body.child(buttons)
            .when_some(dialog.error.clone(), |body, error| {
                body.child(Label::new(error).color(Color::Error))
            })
            .into_any_element()
    }

    /// An edit-class tool starts: mark the open buffers of the files it
    /// names, and open the named files that are not open yet so their write
    /// is imported too. A file the call does not name is never an outcome:
    /// the person's own save of it during the call is not the agent's.
    /// ADR-0047: a `bash` call names no files, so every open buffer of the
    /// workspace is marked; the ones it changes become its hunks.
    fn mark_for_bash(&mut self, tool_call_id: String, cx: &mut Context<Self>) {
        let Some(root) = self.workspace_root(cx) else {
            return;
        };
        let open: Vec<PathBuf> = self
            .project
            .read(cx)
            .opened_buffers(cx)
            .into_iter()
            .filter_map(|b| b.read(cx).file()?.as_local().map(|f| f.abs_path(cx)))
            .filter(|path| path.starts_with(&root))
            .collect();
        self.mark_files(tool_call_id, open, true, cx);
    }

    /// Files the worktree saw change: a running `bash` call's, when not open.
    fn bash_changed(
        &mut self,
        project: &Entity<Project>,
        worktree: worktree::WorktreeId,
        changes: &worktree::UpdatedEntriesSet,
        cx: &mut Context<Self>,
    ) {
        if !self
            .calls
            .values()
            .any(|c| c.bash.is_some() && c.ended.is_none())
        {
            return;
        }
        let Some(worktree) = project.read(cx).worktree_for_id(worktree, cx) else {
            return;
        };
        let worktree = worktree.read(cx);
        // The initial scan's `Loaded` is no change; a directory or an
        // ignored path is no file to review. A removed file has no entry.
        let changed: Vec<(PathBuf, PathBuf)> = changes
            .iter()
            .filter(|(rel, _, change)| {
                *change != worktree::PathChange::Loaded
                    && worktree
                        .entry_for_path(rel)
                        .is_none_or(|e| e.is_file() && !e.is_ignored)
            })
            .map(|(rel, _, _)| (worktree.absolutize(rel), worktree.full_path(rel)))
            .collect();
        for call in self.calls.values_mut() {
            if let (Some(seen), None) = (call.bash.as_mut(), call.ended) {
                for path in &changed {
                    if !seen.contains(path) {
                        seen.push(path.clone());
                    }
                }
            }
        }
    }

    fn mark_before_write(
        &mut self,
        tool_call_id: String,
        paths: &[String],
        cx: &mut Context<Self>,
    ) {
        let root = self.workspace_root(cx);
        let named: Vec<PathBuf> = paths
            .iter()
            .filter_map(|p| {
                let path = std::path::Path::new(p);
                if path.is_absolute() {
                    Some(path.to_path_buf())
                } else {
                    root.as_ref().map(|r| r.join(path))
                }
            })
            .collect();
        self.mark_files(tool_call_id, named, false, cx);
    }

    fn mark_files(
        &mut self,
        tool_call_id: String,
        named: Vec<PathBuf>,
        bash: bool,
        cx: &mut Context<Self>,
    ) {
        let buffers: Vec<(Entity<Buffer>, PathBuf)> = self
            .project
            .read(cx)
            .opened_buffers(cx)
            .into_iter()
            .filter_map(|b| {
                let path = b.read(cx).file().and_then(|f| f.as_local())?.abs_path(cx);
                named.contains(&path).then_some((b, path))
            })
            .collect();
        let to_open: Vec<PathBuf> = named
            .iter()
            .filter(|p| !buffers.iter().any(|(_, o)| o == *p))
            .cloned()
            .collect();
        // A buffer dirty before the call differs from its disk: its disk
        // text at the start is read, so a disk the call leaves alone is no
        // outcome (a clean buffer's text is its disk text).
        let mut dirty: Vec<(usize, PathBuf)> = Vec::new();
        let marks: Vec<(Entity<Buffer>, Mark)> = buffers
            .into_iter()
            .enumerate()
            .map(|(i, (b, path))| {
                if bash {
                    self.review.observe_bash(&b, &tool_call_id, cx);
                } else {
                    self.review.observe(&b, &tool_call_id, cx);
                }
                let mark = b.update(cx, |b, cx| import::begin(b, cx));
                if b.read(cx).is_dirty() {
                    dirty.push((i, path));
                }
                (b, mark)
            })
            .collect();
        self.calls.insert(
            tool_call_id.clone(),
            CallMarks {
                marks,
                unopened: Vec::new(),
                reading: to_open.len() + dirty.len(),
                ended: None,
                before: Vec::new(),
                bash: bash.then(Vec::new),
                overlapped: Vec::new(),
            },
        );
        let fs = self.project.read(cx).fs().clone();
        for (i, path) in dirty {
            let fs = fs.clone();
            let id = tool_call_id.clone();
            cx.spawn(async move |this, cx| {
                let before = fs.load(&path).await.ok();
                this.update(cx, |this, cx| {
                    let Some(call) = this.calls.get_mut(&id) else {
                        return;
                    };
                    if let Some(mut before) = before {
                        text::LineEnding::normalize(&mut before);
                        call.marks[i].1.set_disk_at_start(before);
                    }
                    call.reading -= 1;
                    this.import_when_ready(id, cx);
                })
                .ok();
            })
            .detach();
        }
        for path in to_open {
            let fs = fs.clone();
            let id = tool_call_id.clone();
            cx.spawn(async move |this, cx| {
                let before = fs.load(&path).await.ok();
                this.update(cx, |this, cx| {
                    let Some(call) = this.calls.get_mut(&id) else {
                        return;
                    };
                    call.unopened.push((path, before));
                    call.reading -= 1;
                    this.import_when_ready(id, cx);
                })
                .ok();
            })
            .detach();
        }
    }

    /// Import a call's files once its `ToolEnd` came and every disk read
    /// finished: open the files that were not open, then import each file
    /// as one transaction.
    fn import_when_ready(&mut self, tool_call_id: String, cx: &mut Context<Self>) {
        let ready = self
            .calls
            .get(&tool_call_id)
            .is_some_and(|c| c.reading == 0 && c.ended.is_some());
        if !ready {
            return;
        }
        let mut call = self.calls.remove(&tool_call_id).expect("checked above");
        // A failed edit wrote nothing; a failed command may have written.
        if call.ended == Some(true) && call.bash.is_none() {
            return self.release(&tool_call_id, call, cx);
        }
        // The running workers' briefs at this import (ADR-0063 decision 1):
        // read once, applied to every file the call wrote.
        let scope = self.turn_scope(cx);
        // ADR-0055 decision 6: a buffer two calls wrote while one was a
        // `bash` call is imported by neither.
        for (other_id, other) in &mut self.calls {
            if call.bash.is_none() && other.bash.is_none() {
                continue;
            }
            for (buffer, _) in &call.marks {
                if other.marks.iter().any(|(b, _)| b == buffer) {
                    call.overlapped.push((buffer.clone(), other_id.clone()));
                    other
                        .overlapped
                        .push((buffer.clone(), tool_call_id.clone()));
                }
            }
        }
        let (overlapped, marks): (Vec<_>, Vec<_>) = std::mem::take(&mut call.marks)
            .into_iter()
            .partition(|(b, _)| call.overlapped.iter().any(|(o, _)| o == b));
        call.marks = marks;
        if let Some(changed) = &call.bash {
            for (abs, shown) in changed {
                let open = call.marks.iter().chain(&overlapped).any(|(b, _)| {
                    b.read(cx)
                        .file()
                        .and_then(|f| f.as_local())
                        .is_some_and(|f| f.abs_path(cx) == *abs)
                });
                if !open {
                    self.review.could_not_review(
                        shown.clone(),
                        format!(
                            "changed while bash call {tool_call_id} ran (it or another process \
                             wrote it); it was not open, so cedian has no text from before the \
                             call to review it against"
                        ),
                    );
                }
            }
        }
        let fs = self.project.read(cx).fs().clone();
        for (buffer, mark) in overlapped {
            let others: Vec<&str> = call
                .overlapped
                .iter()
                .filter(|(b, _)| *b == buffer)
                .map(|(_, id)| id.as_str())
                .collect();
            let reason = format!(
                "changed while calls {tool_call_id} and {} both ran, so which call wrote what is \
                 unknown; nothing was imported",
                others.join(", ")
            );
            let file = buffer.read(cx).file().cloned();
            let fs = fs.clone();
            let id = tool_call_id.clone();
            cx.spawn(async move |this, cx| {
                let abs = cx.update(|cx| {
                    file.as_ref()
                        .and_then(|f| f.as_local())
                        .map(|f| f.abs_path(cx))
                });
                let disk = match abs {
                    Some(abs) => fs.load(&abs).await.ok(),
                    None => None,
                };
                this.update(cx, |this, cx| {
                    let mut start = mark.disk_at_start();
                    text::LineEnding::normalize(&mut start);
                    if disk.is_none_or(|mut d| {
                        text::LineEnding::normalize(&mut d);
                        d != start
                    }) {
                        let path = file.map(|f| f.full_path(cx)).unwrap_or_default();
                        this.review.could_not_review(path, reason);
                        this.review.import_unattributed(&buffer, &id, cx);
                    } else {
                        this.review.import_done(&buffer, &id, cx);
                    }
                    this.after_review_change(cx);
                })
                .ok();
            })
            .detach();
        }
        let project = self.project.clone();
        cx.spawn(async move |this, cx| {
            // Files the person opened and edited meanwhile: their buffer
            // stays theirs, so a write to it imports nothing.
            let mut held: Vec<(Entity<Buffer>, String)> = Vec::new();
            // Opened files whose before-text is cedian's own read, which may
            // have run after OMP's write: why OMP gave none.
            // Open buffers too: a dirty one's read, or a clean one Zed
            // reloaded before the mark. Each is listed if it imports
            // nothing, never silently Unchanged.
            let mut unsure: Vec<(Entity<Buffer>, String)> = Vec::new();
            let omp_before =
                |path: &Path| call.before.iter().find(|(p, _)| p == path).map(|(_, t)| t);
            let id = tool_call_id.as_str();
            let lost_read = |why: &str| {
                format!(
                    "OMP's call {id} may have written it before cedian read it, and no text \
                     from before the call is left ({why}); nothing was imported"
                )
            };
            let moved = format!(
                "OMP's call {id} moved it, and OMP's text from before the call is the move's \
                 source, so cedian has no baseline for this path; nothing was imported"
            );
            let mut marks = Vec::new();
            for (buffer, mut mark) in call.marks {
                let path = buffer.read_with(cx, |b, cx| {
                    b.file().and_then(|f| f.as_local()).map(|f| f.abs_path(cx))
                });
                let why = match path.as_deref().and_then(omp_before) {
                    Some(TextBefore::Moved) => {
                        this.update(cx, |this, cx| {
                            let path = buffer.read(cx).file().map(|f| f.full_path(cx));
                            this.review
                                .could_not_review(path.unwrap_or_default(), moved.clone());
                            this.review.import_unattributed(&buffer, id, cx);
                        })
                        .ok();
                        continue;
                    }
                    Some(TextBefore::Text(text)) => {
                        let mut text = text.clone();
                        text::LineEnding::normalize(&mut text);
                        let start = mark.start().text();
                        if mark.read_disk() {
                            mark.correct_disk_at_start(&text);
                            None
                        } else {
                            (text != start).then(|| {
                                format!(
                                    "OMP's call {id} wrote it before cedian marked it (Zed had \
                                     already reloaded it); nothing was imported"
                                )
                            })
                        }
                    }
                    _ if call.bash.is_some() => None,
                    Some(TextBefore::Pruned) => Some(lost_read("OMP dropped it past 32 KiB")),
                    _ => Some(lost_read("OMP's result reports none")),
                };
                if let Some(mut why) = why {
                    if mark.read_disk() {
                        why.push_str(
                            ". It was open with your unsaved edits, which are kept: the file \
                             on disk may have changed under them, and saving them overwrites it",
                        );
                    }
                    unsure.push((buffer.clone(), why));
                }
                marks.push((buffer, mark));
            }
            for (path, read) in call.unopened {
                let (before, why) = match omp_before(&path) {
                    Some(TextBefore::Moved) => {
                        this.update(cx, |this, cx| {
                            let shown = this.shown_path(&path, cx);
                            this.review.could_not_review(shown, moved.clone());
                        })
                        .ok();
                        continue;
                    }
                    Some(TextBefore::Text(text)) => {
                        let mut text = text.clone();
                        text::LineEnding::normalize(&mut text);
                        (Some(text), None)
                    }
                    Some(TextBefore::NewFile) => (Some(String::new()), None),
                    Some(TextBefore::Pruned) => {
                        (read, Some(lost_read("OMP dropped it past 32 KiB")))
                    }
                    None => (read, Some(lost_read("OMP's result reports none"))),
                };
                let opened = project
                    .update(cx, |p, cx| p.open_local_buffer(path.clone(), cx))
                    .await;
                match opened {
                    Ok(buffer) => {
                        if let Some(why) = why {
                            unsure.push((buffer.clone(), why));
                        }
                        let before = before.unwrap_or_default();
                        let mark = buffer.update(cx, |b, cx| {
                            import::begin_from(b, &before, cx).unwrap_or_else(|| {
                                held.push((cx.entity(), before.clone()));
                                import::begin(b, cx)
                            })
                        });
                        marks.push((buffer, mark));
                    }
                    Err(e) => {
                        this.update(cx, |this, _| {
                            this.review.could_not_review(path, e.to_string());
                        })
                        .ok();
                    }
                }
            }
            for (buffer, mark) in marks {
                let outcome = import::finish(buffer.clone(), mark.clone(), cx).await;
                this.update(cx, |this, cx| {
                    match outcome {
                        Ok(ImportOutcome::Imported(transaction)) => {
                            let models = this.answered_models();
                            this.audit_import(&tool_call_id, &buffer, scope.as_deref(), cx);
                            this.review.agent_edited(
                                &buffer,
                                &tool_call_id,
                                transaction,
                                mark.start().clone(),
                                models,
                                scope.clone(),
                                cx,
                            )
                        }
                        Ok(ImportOutcome::Stale) => {
                            this.review.import_refused(&buffer, &tool_call_id, cx)
                        }
                        Ok(ImportOutcome::Unchanged) => {
                            let written = held
                                .iter()
                                .find(|(b, _)| *b == buffer)
                                .is_some_and(|(_, before)| buffer.read(cx).text() != *before);
                            let path = || {
                                buffer
                                    .read(cx)
                                    .file()
                                    .map(|f| f.full_path(cx))
                                    .unwrap_or_default()
                            };
                            if written {
                                this.review.could_not_review(
                                    path(),
                                    format!(
                                        "it was open with your own edits when OMP wrote it \
                                         (call {tool_call_id}); nothing was imported"
                                    ),
                                );
                            } else if let Some((_, why)) = unsure.iter().find(|(b, _)| *b == buffer)
                            {
                                this.review.could_not_review(path(), why.clone());
                            }
                            this.review.import_done(&buffer, &tool_call_id, cx);
                        }
                        Err(e) => {
                            let path = buffer.read(cx).file().map(|f| f.full_path(cx));
                            this.review
                                .could_not_review(path.unwrap_or_default(), e.to_string());
                            this.review.import_done(&buffer, &tool_call_id, cx);
                        }
                    }
                    this.still_writing(&buffer, cx);
                    this.watch_reviewed_buffers(cx);
                    this.after_review_change(cx);
                })
                .ok();
            }
            this.update(cx, |this, cx| this.after_review_change(cx))
                .ok();
        })
        .detach();
    }

    /// `abs` as the review shows paths: under its worktree's name.
    fn shown_path(&self, abs: &Path, cx: &App) -> PathBuf {
        match self.project.read(cx).find_worktree(abs, cx) {
            Some((worktree, rel)) => worktree.read(cx).full_path(&rel),
            None => abs.to_path_buf(),
        }
    }

    /// An outcome may have just created the file's review; the calls still
    /// in flight that marked the buffer are writing it, and the review
    /// only learns of a call at its observe, so they are told again.
    fn still_writing(&mut self, buffer: &Entity<Buffer>, cx: &mut Context<Self>) {
        let ids: Vec<(String, bool)> = self
            .calls
            .iter()
            .filter(|(_, c)| c.marks.iter().any(|(b, _)| b == buffer))
            .map(|(id, c)| (id.clone(), c.bash.is_some()))
            .collect();
        for (id, bash) in ids {
            if bash {
                self.review.observe_bash(buffer, &id, cx);
            } else {
                self.review.observe(buffer, &id, cx);
            }
        }
    }

    /// A call that imports nothing: its marked buffers are the user's again.
    fn release(&mut self, tool_call_id: &str, call: CallMarks, cx: &mut Context<Self>) {
        for (buffer, _) in call.marks {
            self.review.import_done(&buffer, tool_call_id, cx);
        }
        self.after_review_change(cx);
    }

    /// One finding on its hunk: severity, message, and Dismiss with a
    /// reason, or the reason it was dismissed for.
    fn render_finding(&self, finding: &AttachedFinding, cx: &mut Context<Self>) -> AnyElement {
        let id = finding.id.clone();
        let selector = format!("cedian-finding-{id}");
        let severity = match finding.finding.severity {
            FindingSeverity::Blocker => ("blocker", Color::Error),
            FindingSeverity::Suggestion => ("suggestion", Color::Warning),
            FindingSeverity::Info => ("info", Color::Muted),
        };
        let mut row = v_flex().debug_selector(move || selector).child(
            Label::new(format!("{id} {}: {}", severity.0, finding.finding.message))
                .size(LabelSize::Small)
                .color(severity.1),
        );
        if let Some(reason) = &finding.dismissed {
            return row
                .child(
                    Label::new(format!("dismissed: {reason}"))
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element();
        }
        match &self.dismissing {
            Some((open, input)) if *open == id => {
                let confirm = format!("cedian-dismiss-confirm-{id}");
                row =
                    row.child(
                        h_flex()
                            .gap_1()
                            .child(div().flex_1().child(input.clone()))
                            .child(div().debug_selector(|| confirm.clone()).child(
                                Button::new(ElementId::Name(confirm.into()), "Dismiss").on_click(
                                    cx.listener(|this, _, _, cx| this.confirm_dismiss(cx)),
                                ),
                            )),
                    );
            }
            _ => {
                let open = format!("cedian-dismiss-{id}");
                row = row.child(div().debug_selector(|| open.clone()).child(
                    Button::new(ElementId::Name(open.into()), "Dismiss…").on_click(
                        cx.listener(move |this, _, window, cx| this.start_dismiss(&id, window, cx)),
                    ),
                ));
            }
        }
        row.into_any_element()
    }

    fn render_review(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let turn = self.review.current_turn();
        let mut body = v_flex()
            .id("cedian-review")
            .debug_selector(|| "cedian-review".to_string())
            .flex_1()
            .overflow_y_scroll()
            .gap_2()
            .child(
                h_flex()
                    .gap_2()
                    .child(Label::new("Review changes"))
                    .child(
                        div()
                            .debug_selector(|| "cedian-review-request".to_string())
                            .child(
                                Button::new("cedian-review-request", "Ask for review")
                                    .disabled(self._review_run.is_some())
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.request_review(String::new(), cx)
                                    })),
                            ),
                    )
                    .child(
                        div()
                            .debug_selector(|| "cedian-accept-all".to_string())
                            .child(
                                Button::new("cedian-accept-all", "Accept all")
                                    .on_click(cx.listener(|this, _, _, cx| this.accept_all(cx))),
                            ),
                    )
                    .when(turn > 0 && self.turn == Turn::Idle, |row| {
                        let label = self.revert_turn_label(turn);
                        let drawn = format!("cedian-revert-turn-text:{label}");
                        row.child(
                            div()
                                .debug_selector(|| "cedian-revert-turn".to_string())
                                .child(div().debug_selector(move || drawn).child(
                                    Button::new("cedian-revert-turn", label).on_click(cx.listener(
                                        move |this, _, _, cx| this.revert_turn(turn, cx),
                                    )),
                                )),
                        )
                    }),
            );
        if self.confirm_accept_all {
            let stale = self.review.stale_count();
            body = body.child(
                h_flex()
                    .gap_2()
                    .debug_selector(|| "cedian-accept-all-confirm".to_string())
                    .child(
                        Label::new(format!(
                            "{stale} STALE hunk(s) were edited after the agent. Accept them too?"
                        ))
                        .size(LabelSize::Small)
                        .color(Color::Warning),
                    )
                    .child(
                        div()
                            .debug_selector(|| "cedian-accept-all-yes".to_string())
                            .child(Button::new("cedian-accept-all-yes", "Yes").on_click(
                                cx.listener(|this, _, _, cx| this.accept_all_answered(true, cx)),
                            )),
                    )
                    .child(
                        div()
                            .debug_selector(|| "cedian-accept-all-no".to_string())
                            .child(Button::new("cedian-accept-all-no", "No").on_click(
                                cx.listener(|this, _, _, cx| this.accept_all_answered(false, cx)),
                            )),
                    ),
            );
        }
        if self.review.is_empty() {
            return body
                .child(Label::new("No agent edits yet").color(Color::Muted))
                .into_any_element();
        }
        let files: Vec<_> = self
            .review
            .files()
            .iter()
            .map(|file| {
                (
                    self.review.path(file, cx),
                    crate::review::review_key(file.buffer().read(cx)),
                    file.stale_import().map(str::to_string),
                    file.hunks().to_vec(),
                )
            })
            .collect();
        let on_hunks: std::collections::HashSet<&str> = self
            .findings
            .iter()
            .filter(|f| {
                files.iter().any(|(_, key, _, hunks)| {
                    Some(&f.finding.path) == key.as_ref()
                        && hunks.iter().any(|h| f.hunk_text == h.key.after_text)
                })
            })
            .map(|f| f.id.as_str())
            .collect();
        let orphans: Vec<AttachedFinding> = self
            .findings
            .iter()
            .filter(|f| f.dismissed.is_none() && !on_hunks.contains(f.id.as_str()))
            .cloned()
            .collect();
        if !orphans.is_empty() {
            let mut section = v_flex().gap_1().child(
                Label::new("Findings whose change is no longer in the review")
                    .size(LabelSize::Small)
                    .color(Color::Warning),
            );
            for finding in &orphans {
                section = section.child(self.render_finding(finding, cx));
            }
            body = body.child(section);
        }
        for (path, reason) in self.review.unreviewable().to_vec() {
            let selector = format!("cedian-unreviewed-{}", path.display());
            body = body.child(
                v_flex()
                    .gap_1()
                    .debug_selector(move || selector)
                    .child(Label::new(path.display().to_string()).size(LabelSize::Small))
                    .child(
                        Label::new(format!(
                            "OMP wrote this file; it could not be reviewed: {reason}"
                        ))
                        .size(LabelSize::Small)
                        .color(Color::Warning),
                    ),
            );
        }
        for (path, key, stale_import, hunks) in files {
            let name = path.display().to_string();
            let mut section = v_flex()
                .gap_1()
                .child(Label::new(name.clone()).size(LabelSize::Small));
            if let Some(reason) = stale_import {
                section = section.child(
                    Label::new(format!("STALE: {reason}"))
                        .size(LabelSize::Small)
                        .color(Color::Warning),
                );
            }
            for (index, hunk) in hunks.into_iter().enumerate() {
                let selector = |action: &str| format!("cedian-{action}-{name}-{index}");
                let head = format!(
                    "lines {}-{} · {:?} · {}",
                    hunk.rows.start + 1,
                    hunk.rows.end,
                    hunk.status,
                    hunk.tool_call_ids.join(", ")
                );
                let mut card = v_flex()
                    .p_1()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .child(Label::new(head).size(LabelSize::Small))
                    .children(hunk.old_text.lines().map(|l| {
                        Label::new(format!("- {l}"))
                            .size(LabelSize::Small)
                            .color(Color::Deleted)
                            .into_any_element()
                    }))
                    .children(hunk.new_text.lines().map(|l| {
                        Label::new(format!("+ {l}"))
                            .size(LabelSize::Small)
                            .color(Color::Created)
                            .into_any_element()
                    }));
                let accept = matches!(
                    hunk.status,
                    HunkStatus::Pending
                        | HunkStatus::Unattributed
                        | HunkStatus::Stale
                        | HunkStatus::OutOfScope
                );
                // Reject refuses OutOfScope like Stale, so its button is absent
                // for both (ADR-0033 decision 3).
                let reject = matches!(hunk.status, HunkStatus::Pending | HunkStatus::Unattributed);
                let mut buttons = h_flex().gap_1();
                if accept {
                    let (p, k, s) = (path.clone(), hunk.key.clone(), selector("accept"));
                    buttons = buttons.child(div().debug_selector(|| s.clone()).child(
                        Button::new(ElementId::Name(s.clone().into()), "Accept").on_click(
                            cx.listener(move |this, _, _, cx| this.accept_hunk(&p, &k, cx)),
                        ),
                    ));
                }
                if reject {
                    let (p, k, s) = (path.clone(), hunk.key.clone(), selector("reject"));
                    buttons = buttons.child(div().debug_selector(|| s.clone()).child(
                        Button::new(ElementId::Name(s.clone().into()), "Reject").on_click(
                            cx.listener(move |this, _, _, cx| this.reject_hunk(&p, &k, cx)),
                        ),
                    ));
                }
                card = card.child(buttons);
                for finding in self.findings.iter().filter(|f| {
                    Some(&f.finding.path) == key.as_ref() && f.hunk_text == hunk.key.after_text
                }) {
                    card = card.child(self.render_finding(finding, cx));
                }
                section = section.child(card);
            }
            body = body.child(section);
        }
        body.into_any_element()
    }
}

impl EventEmitter<PanelEvent> for CedianPanel {}

impl Focusable for CedianPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CedianPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let running = |tree: &SubagentTree, id: &str| {
            tree.rows()
                .iter()
                .any(|row| row.id == id && row.status == SubagentStatus::Running)
        };
        self.steer_boxes
            .retain(|id, _| running(&self.subagents, id));
        for row in self.subagents.rows() {
            if row.status == SubagentStatus::Running && !self.steer_boxes.contains_key(&row.id) {
                let input = cx.new(|cx| {
                    let mut editor = Editor::single_line(window, cx);
                    editor.set_placeholder_text("Steer this subagent…", window, cx);
                    editor
                });
                self.steer_boxes.insert(row.id.clone(), input);
            }
        }
        let (messages, cards) = cedian_agent_ui::render_thread(self.thread.events());
        let mut rows: Vec<AnyElement> = messages
            .into_iter()
            .map(|m| Label::new(format!("{:?}: {}", m.role, m.text)).into_any_element())
            .collect();
        for entry in self.cards_with_subagents(cards) {
            rows.push(match entry {
                Entry::Card(card) => {
                    let selector = format!("cedian-tool-{}", card.call_id);
                    div()
                        .debug_selector(move || selector)
                        .child(Label::new(card_line(&card)).color(Color::Muted))
                        .into_any_element()
                }
                Entry::Subagent(row) => self.render_subagent(row, cx),
            });
        }
        let connection = match &self.connection {
            Connection::NotStarted => "OMP not started".to_string(),
            Connection::Starting => "starting OMP…".to_string(),
            Connection::Checking => {
                "checking whether another process drives the session…".to_string()
            }
            Connection::Ready {
                session_id,
                resumed,
                policy_note,
            } => format!(
                "OMP ready, session {} ({}){}",
                session_id.get(..8).unwrap_or(session_id),
                if *resumed { "resumed" } else { "new" },
                if policy_note.is_some() {
                    " · OMP policy"
                } else {
                    ""
                }
            ),
            Connection::Stopped(reason) => reason.clone(),
            Connection::Taken { session_id, reason } => format!(
                "session {} not opened: {reason}",
                session_id.get(..8).unwrap_or(session_id)
            ),
        };
        let taken = matches!(self.connection, Connection::Taken { .. });
        let stopped = matches!(self.connection, Connection::Stopped(_));
        let dialogs: Vec<AnyElement> = self
            .dialogs
            .iter()
            .map(|(id, dialog)| self.render_dialog(id, dialog, cx))
            .collect();
        let stoppable = matches!(self.turn, Turn::Queued | Turn::Streaming);
        let idle = self.turn == Turn::Idle;
        let images = self.images.len();
        let imported: usize = self
            .review
            .files()
            .iter()
            .map(|f| f.agent_txns().len())
            .sum();
        let open: usize = self
            .review
            .files()
            .iter()
            .flat_map(|f| f.hunks())
            .filter(|h| !matches!(h.status, HunkStatus::Accepted | HunkStatus::Rejected))
            .count();
        let stale_files = self
            .review
            .files()
            .iter()
            .filter(|f| f.stale_import().is_some())
            .count();
        let footer = format!(
            "{} · {} · imported {} agent edit(s){}",
            connection,
            self.turn.label(),
            imported,
            if stale_files > 0 {
                format!(" · {stale_files} STALE file(s)")
            } else {
                String::new()
            }
        );
        let review = self.show_review.then(|| self.render_review(cx));
        let browser = self.render_browser(cx);
        let workflow = self.render_workflow(cx);
        let workers = self.render_worker_trees(cx);
        v_flex()
            .key_context("CedianPanel")
            .track_focus(&self.focus_handle)
            .capture_action(cx.listener(Self::paste))
            .size_full()
            .p_2()
            .gap_2()
            .child(
                h_flex()
                    .justify_end()
                    .gap_2()
                    .child(
                        div()
                            .debug_selector(|| "cedian-review-toggle".to_string())
                            .child(
                                Button::new(
                                    "cedian-review-toggle",
                                    if self.show_review {
                                        "Back to chat".to_string()
                                    } else {
                                        format!("Review changes ({open})")
                                    },
                                )
                                .on_click(cx.listener(|this, _, _, cx| this.toggle_review(cx))),
                            ),
                    )
                    .child(
                        div()
                            .debug_selector(|| "cedian-open-browser".to_string())
                            .child(
                                Button::new("cedian-open-browser", "Open browser")
                                    .disabled(self.browser.is_none())
                                    .on_click(cx.listener(|this, _, _, cx| this.open_browser(cx))),
                            ),
                    )
                    .child(
                        Button::new(
                            "cedian-settings",
                            if self.show_settings {
                                "Back to chat"
                            } else {
                                "OMP settings"
                            },
                        )
                        .on_click(
                            cx.listener(|this, _, window, cx| this.toggle_settings(window, cx)),
                        ),
                    ),
            )
            .when_some(review, |panel, review| panel.child(review))
            .when_some(
                self.settings
                    .clone()
                    .filter(|_| self.show_settings && !self.show_review),
                |panel, settings| panel.child(div().flex_1().child(settings)),
            )
            .when(!self.show_settings && !self.show_review, |panel| {
                panel.child(
                    v_flex()
                        .id("cedian-thread")
                        .flex_1()
                        .overflow_y_scroll()
                        .gap_1()
                        .children(rows),
                )
            })
            .children(workflow)
            .children(browser)
            .children(workers)
            .children(dialogs)
            .children(self.render_worktree_ask(cx))
            .children(self.render_onboarding(cx))
            .child(self.render_picker(cx))
            .child(self.render_session(cx))
            .children(self.render_toasts(cx))
            .when_some(self.omp_policy.clone(), |panel, badge| {
                panel.child(
                    div()
                        .debug_selector(|| "cedian-omp-policy".to_string())
                        .child(
                            Label::new(badge)
                                .size(LabelSize::Small)
                                .color(Color::Warning),
                        ),
                )
            })
            .when_some(self.omp_warning.clone(), |panel, warning| {
                panel.child(
                    div()
                        .debug_selector(|| "cedian-omp-version".to_string())
                        .child(
                            Label::new(warning)
                                .size(LabelSize::Small)
                                .color(Color::Warning),
                        ),
                )
            })
            .when_some(self.ledger_error.clone(), |panel, error| {
                panel.child(
                    div()
                        .debug_selector(|| "cedian-ledger-error".to_string())
                        .child(Label::new(error).size(LabelSize::Small).color(Color::Error)),
                )
            })
            .when_some(self.notice.clone(), |panel, notice| {
                panel.child(
                    div().debug_selector(|| "cedian-notice".to_string()).child(
                        Label::new(notice)
                            .size(LabelSize::Small)
                            .color(Color::Warning),
                    ),
                )
            })
            .when(images > 0, |panel| {
                panel.child(
                    h_flex()
                        .gap_2()
                        .child(
                            Label::new(format!("{images} image(s) attached"))
                                .size(LabelSize::Small),
                        )
                        .child(
                            Button::new("cedian-clear-images", "Remove images").on_click(
                                cx.listener(|this, _, _, cx| {
                                    this.images.clear();
                                    cx.notify();
                                }),
                            ),
                        ),
                )
            })
            .child(
                h_flex()
                    .gap_2()
                    .child(div().flex_1().child(self.input.clone()))
                    .child(
                        Button::new("cedian-send", "Send")
                            .disabled(!idle && self.turn != Turn::Streaming)
                            .on_click(cx.listener(|this, _, window, cx| this.send(window, cx))),
                    )
                    .when(self.turn == Turn::Streaming, |row| {
                        row.child(
                            div().debug_selector(|| "cedian-steer".to_string()).child(
                                Button::new("cedian-steer", "Steer")
                                    .on_click(cx.listener(|this, _, _, cx| this.steer(cx))),
                            ),
                        )
                    })
                    .when(stoppable, |row| {
                        row.child(
                            div().debug_selector(|| "cedian-stop".to_string()).child(
                                Button::new("cedian-stop", "Stop")
                                    .on_click(cx.listener(|this, _, _, cx| this.stop_turn(cx))),
                            ),
                        )
                    }),
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Label::new(footer)
                            .size(LabelSize::Small)
                            .color(if stopped || taken {
                                Color::Error
                            } else {
                                Color::Muted
                            }),
                    )
                    .when(taken, |row| {
                        row.child(
                            div()
                                .debug_selector(|| "cedian-new-session".to_string())
                                .child(
                                    Button::new("cedian-new-session", "Start a new session")
                                        .on_click(
                                            cx.listener(|this, _, _, cx| {
                                                this.start_new_session(cx)
                                            }),
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .debug_selector(|| "cedian-retry".to_string())
                                .child(Button::new("cedian-retry", "Retry").on_click(
                                    cx.listener(|this, _, _, cx| this.retry_session(cx)),
                                )),
                        )
                    })
                    .when(stopped, |row| {
                        row.child(
                            Button::new("cedian-restart", "Restart OMP").on_click(
                                cx.listener(|this, _, window, cx| this.restart(window, cx)),
                            ),
                        )
                    }),
            )
    }
}

impl Panel for CedianPanel {
    fn persistent_name() -> &'static str {
        "cedian"
    }

    fn panel_key() -> &'static str {
        PANEL_KEY
    }

    fn position(&self, _: &Window, _: &App) -> DockPosition {
        DockPosition::Right
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, _: DockPosition, _: &mut Window, _: &mut Context<Self>) {}

    fn default_size(&self, _: &Window, _: &App) -> Pixels {
        px(420.)
    }

    fn icon(&self, _: &Window, _: &App) -> Option<IconName> {
        Some(IconName::ZedAssistant)
    }

    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> {
        Some("cedian")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        20
    }
}

#[cfg(test)]
mod tests {
    //! Live S9a exit check (T2 + T5) without a visible window: the real panel
    //! code, real OMP through the spawn profile, real disk, real Zed buffer
    //! undo. Needs OMP auth:
    //! `cargo test -p cedian_panel -- --ignored --nocapture live_`
    use super::*;
    use cedian_worker::WorkerHead;
    use fs::Fs as _;
    use gpui::{Modifiers, TestAppContext, VisualTestContext};
    use settings::SettingsStore;
    use std::time::{Duration, Instant};

    /// A login ends in the view when OMP cancels its code prompt or the
    /// person presses Cancel; a late result for an earlier login leaves
    /// a later one running.
    #[gpui::test]
    async fn a_login_can_be_cancelled_and_a_stale_result_is_ignored(cx: &mut TestAppContext) {
        init(cx);
        let project = Project::test(fs::FakeFs::new(cx.executor()), [], cx).await;
        let window = cx.add_window(|window, cx| CedianPanel::new(project.clone(), window, cx));
        let running = |provider: &str| Onboarding {
            running: Some(provider.to_string()),
            dialogs: vec!["login-code".to_string()],
            ..Default::default()
        };
        window
            .update(cx, |panel, window, cx| {
                panel.onboarding = Some(running("anthropic"));
                panel.on_link_event(
                    LinkEvent::Event(RouterEvent::UiRequest(ExtensionUiRequest::Cancel(
                        omp_rpc::CancelUiRequest {
                            id: "c1".into(),
                            target_id: "login-code".into(),
                        },
                    ))),
                    window,
                    cx,
                );
                assert_eq!(
                    panel.onboarding.as_ref().unwrap().running,
                    None,
                    "OMP cancelled"
                );

                panel.onboarding = Some(running("anthropic"));
                panel.cancel_login(cx);
                assert_eq!(panel.onboarding.as_ref().unwrap().running, None, "Cancel");

                panel.onboarding = Some(running("openai-codex"));
                panel.on_link_event(
                    LinkEvent::LoggedIn {
                        provider: "anthropic".into(),
                        result: Err("cancelled".into()),
                    },
                    window,
                    cx,
                );
                assert_eq!(
                    panel.onboarding.as_ref().unwrap().running.as_deref(),
                    Some("openai-codex")
                );
            })
            .unwrap();
    }

    /// Through a symlinked worktree root, an unsaved buffer is keyed by
    /// the same workspace path the disk scan uses.
    #[gpui::test]
    async fn an_unsaved_buffer_under_a_symlinked_root_keys_its_workspace_path(
        cx: &mut TestAppContext,
    ) {
        init(cx);
        cx.executor().allow_parking();
        let dir = std::env::temp_dir().join(format!("cedian-symroot-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("real")).unwrap();
        std::fs::write(dir.join("real/notes.txt"), "a\n").unwrap();
        std::os::unix::fs::symlink(dir.join("real"), dir.join("link")).unwrap();
        let link = dir.join("link");
        let project =
            Project::test(fs::RealFs::new(None, cx.executor()), [link.as_path()], cx).await;
        let window = cx.add_window(|window, cx| CedianPanel::new(project.clone(), window, cx));
        let buffer = project
            .update(cx, |p, cx| {
                p.open_local_buffer(dir.join("real/notes.txt"), cx)
            })
            .await
            .unwrap();
        buffer.update(cx, |b, cx| b.edit([(0..0, "x")], None, cx));
        cx.run_until_parked();
        let keys: Vec<String> = window
            .update(cx, |panel, _, _| {
                panel.unsaved.lock().keys().cloned().collect()
            })
            .unwrap();
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(keys, ["notes.txt"]);
    }

    #[test]
    fn a_login_opens_only_https_or_a_local_http_page() {
        let open = login_link;
        for refused in [
            "file:///Applications/Calculator.app",
            "x-custom-app://run",
            "javascript:alert(1)",
            "http://example.com/login",
        ] {
            assert!(open(refused).is_err(), "{refused} must be refused");
        }
        assert!(open("file:///etc").unwrap_err().contains("file"));
        for opened in [
            "https://auth.example.com/authorize?x=1",
            "http://localhost:1455/callback",
            "http://127.0.0.1:8080/",
        ] {
            assert_eq!(open(opened).as_deref(), Ok(opened));
        }
    }

    fn init(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });
    }

    /// Pump the test executor while real OS threads (OMP) make progress.
    fn wait_until(
        cx: &mut TestAppContext,
        what: &str,
        mut done: impl FnMut(&mut TestAppContext) -> bool,
    ) {
        let deadline = Instant::now() + Duration::from_secs(300);
        loop {
            cx.run_until_parked();
            if done(cx) {
                return;
            }
            assert!(Instant::now() < deadline, "timed out waiting for {what}");
            std::thread::sleep(Duration::from_millis(200));
        }
    }

    fn texts(panel: &CedianPanel) -> String {
        let (messages, cards) = cedian_agent_ui::render_thread(panel.thread.events());
        let mut out: Vec<String> = messages
            .iter()
            .map(|m| format!("{:?}: {}", m.role, m.text))
            .collect();
        out.extend(
            cards
                .iter()
                .map(|c| format!("card {:?} {}", c.status, c.display_line())),
        );
        out.join("\n")
    }

    /// ADR-0035: an audit failure fails the turn once, however many rows
    /// fail after it, and OMP settling brings the panel back to idle with
    /// the reason kept on screen.
    #[gpui::test]
    async fn an_audit_failure_fails_the_turn_once_and_settles_to_idle(cx: &mut TestAppContext) {
        init(cx);
        let project = Project::test(fs::FakeFs::new(cx.executor()), [], cx).await;
        let window = cx.add_window(|window, cx| CedianPanel::new(project.clone(), window, cx));
        window
            .update(cx, |panel, window, cx| {
                panel.turn = Turn::Streaming;
                for row in ["row 1", "row 2"] {
                    panel.on_link_event(LinkEvent::AuditFailed(row.to_string()), window, cx);
                }
                assert_eq!(panel.turn, Turn::Failed("audit: row 1".to_string()));
                panel.on_link_event(LinkEvent::Event(RouterEvent::Settled), window, cx);
                assert_eq!(panel.turn, Turn::Idle);
                assert_eq!(panel.notice(), Some("turn failed: audit: row 1"));
            })
            .unwrap();
    }

    /// A prompt call that ends in an error keeps what the person must know:
    /// a failed turn's reason, and a prompt OMP ran stays in the thread.
    #[gpui::test]
    async fn a_prompt_ending_in_an_error_keeps_the_failure_and_what_ran(cx: &mut TestAppContext) {
        init(cx);
        let project = Project::test(fs::FakeFs::new(cx.executor()), [], cx).await;
        let window = cx.add_window(|window, cx| CedianPanel::new(project.clone(), window, cx));
        window
            .update(cx, |panel, window, cx| {
                let failed = |panel: &mut CedianPanel| {
                    panel.turn = Turn::Failed("audit: row".to_string());
                    panel.notice = None;
                };
                let expected = Some("turn failed: audit: row");

                failed(panel);
                panel.on_link_event(LinkEvent::PromptFailed("pipe".into()), window, cx);
                assert_eq!((&panel.turn, panel.notice()), (&Turn::Idle, expected));

                failed(panel);
                panel.on_link_event(LinkEvent::PromptCancelled, window, cx);
                assert_eq!((&panel.turn, panel.notice()), (&Turn::Idle, expected));

                panel.thread.push_user("it ran");
                panel.turn = Turn::Stopping;
                panel.on_link_event(LinkEvent::PromptStopped("pipe".into()), window, cx);
                assert_eq!(panel.turn, Turn::Idle);
                assert_eq!(panel.notice(), Some("turn stopped: pipe"));
                assert!(texts(panel).contains("it ran"), "{}", texts(panel));
            })
            .unwrap();
    }

    const ORIGINAL: &str = "alpha\nbeta\ngamma\n";

    struct Fixture {
        fs: std::sync::Arc<fs::FakeFs>,
        project: Entity<Project>,
        window: gpui::WindowHandle<CedianPanel>,
    }

    /// A workspace with `notes.txt` open and `other.txt` on disk only; no
    /// OMP (the panel is driven with router events directly).
    async fn fixture(cx: &mut TestAppContext) -> (Fixture, Entity<Buffer>) {
        init(cx);
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/ws",
            serde_json::json!({"notes.txt": ORIGINAL, "other.txt": "one\ntwo\n"}),
        )
        .await;
        let project = Project::test(fs.clone(), [std::path::Path::new("/ws")], cx).await;
        let buffer = project
            .update(cx, |p, cx| p.open_local_buffer("/ws/notes.txt", cx))
            .await
            .unwrap();
        let window = cx.add_window(|window, cx| CedianPanel::new(project.clone(), window, cx));
        window
            .update(cx, |panel, _, _| {
                let state =
                    std::env::temp_dir().join(format!("cedian-panel-state-{}", std::process::id()));
                std::fs::create_dir_all(&state).unwrap();
                panel.set_state_dir(state);
                panel.review.begin_turn();
            })
            .unwrap();
        (
            Fixture {
                fs,
                project,
                window,
            },
            buffer,
        )
    }

    fn tool_start(f: &Fixture, cx: &mut TestAppContext, id: &str, paths: &[&str]) {
        tool_start_named(f, cx, id, "edit", paths);
    }

    fn tool_start_named(
        f: &Fixture,
        cx: &mut TestAppContext,
        id: &str,
        name: &str,
        paths: &[&str],
    ) {
        let event = RouterEvent::ToolStart {
            tool_call_id: id.to_string(),
            tool_name: name.to_string(),
            args_preview: String::new(),
            paths: paths.iter().map(|p| p.to_string()).collect(),
        };
        f.window
            .update(cx, |panel, _, cx| panel.on_event(event, cx))
            .unwrap();
    }

    fn settled(f: &Fixture, cx: &mut TestAppContext) {
        f.window
            .update(cx, |panel, _, cx| panel.on_event(RouterEvent::Settled, cx))
            .unwrap();
        cx.run_until_parked();
    }

    fn tool_end(f: &Fixture, cx: &mut TestAppContext, id: &str) {
        tool_end_with(f, cx, id, Vec::new());
    }

    fn tool_end_with(
        f: &Fixture,
        cx: &mut TestAppContext,
        id: &str,
        before: Vec<(String, cedian_omp::TextBefore)>,
    ) {
        tool_end_named(f, cx, id, "edit", before);
    }

    fn tool_end_named(
        f: &Fixture,
        cx: &mut TestAppContext,
        id: &str,
        name: &str,
        before: Vec<(String, cedian_omp::TextBefore)>,
    ) {
        let event = RouterEvent::ToolEnd {
            tool_call_id: id.to_string(),
            tool_name: name.to_string(),
            result_summary: String::new(),
            is_error: false,
            before,
        };
        f.window
            .update(cx, |panel, _, cx| panel.on_event(event, cx))
            .unwrap();
        cx.run_until_parked();
    }

    async fn omp_writes(f: &Fixture, path: &str, text: &str) {
        f.fs.save(
            std::path::Path::new(path),
            &text.into(),
            text::LineEnding::Unix,
        )
        .await
        .unwrap();
    }

    /// `(path, status, tool calls)` per hunk, files in review order.
    fn hunks(f: &Fixture, cx: &mut TestAppContext) -> Vec<(String, HunkStatus, Vec<String>)> {
        f.window
            .update(cx, |panel, _, cx| {
                panel.review.rebuild(cx);
                panel
                    .review
                    .files()
                    .iter()
                    .flat_map(|file| {
                        let path = panel.review.path(file, cx).display().to_string();
                        file.hunks()
                            .iter()
                            .map(move |h| (path.clone(), h.status, h.tool_call_ids.clone()))
                    })
                    .collect()
            })
            .unwrap()
    }

    /// A running worker and its stored brief, as the panel reads them at
    /// import time (ADR-0063 decision 1): only `write` is in scope.
    fn running_worker(state: &std::path::Path, write: &[&str]) {
        let brief = Brief::from_json(
            &serde_json::json!({
                "goal": "g", "scope": {"write": write},
                "acceptance": ["a"], "verify": ["v"], "timebox_min": 15,
            })
            .to_string(),
        )
        .unwrap();
        brief.store(state, "w1", 1).unwrap();
        let mut registry = Registry::default();
        registry
            .insert(WorkerHead {
                id: "w1".into(),
                branch: "cedian/w1".into(),
                worktree: ".worktrees/w1".into(),
                task_title: "t".into(),
                kind: "task".into(),
                status: WorkerStatus::Running,
                note: String::new(),
                timebox_min: 15.0,
                last_progress: None,
            })
            .unwrap();
        registry.save(state).unwrap();
    }

    /// OMP writes `other.txt` under a running worker whose brief allows only
    /// `notes.txt`.
    async fn out_of_scope_hunk(f: &Fixture, cx: &mut TestAppContext) {
        tool_start(f, cx, "o1", &["other.txt"]);
        cx.run_until_parked();
        omp_writes(f, "/ws/other.txt", "ONE\ntwo\n").await;
        tool_end(f, cx, "o1");
    }

    /// With a running worker covering only `notes.txt`, an edit to another
    /// file is OutOfScope and one to `notes.txt` is Pending (ADR-0063
    /// decision 1).
    #[gpui::test]
    async fn a_covered_file_is_pending_and_another_is_out_of_scope(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        let state = state_dir("scope-import");
        running_worker(&state, &["notes.txt"]);
        f.window
            .update(cx, |panel, _, _| panel.set_state_dir(state.clone()))
            .unwrap();

        agent_hunk(&f, cx, "n1", "alpha\nBETA\ngamma\n").await;
        out_of_scope_hunk(&f, cx).await;

        let got = hunks(&f, cx);
        assert!(
            got.iter()
                .any(|(p, s, _)| p.ends_with("notes.txt") && *s == HunkStatus::Pending),
            "{got:?}"
        );
        assert!(
            got.iter()
                .any(|(p, s, _)| p.ends_with("other.txt") && *s == HunkStatus::OutOfScope),
            "{got:?}"
        );
    }

    /// With no running worker the same edit stays Pending, exactly as before
    /// ADR-0063.
    #[gpui::test]
    async fn with_no_running_worker_the_same_edit_stays_pending(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        let state = state_dir("scope-none");
        f.window
            .update(cx, |panel, _, _| panel.set_state_dir(state.clone()))
            .unwrap();
        out_of_scope_hunk(&f, cx).await;
        let got = hunks(&f, cx);
        assert!(
            got.iter()
                .any(|(p, s, _)| p.ends_with("other.txt") && *s == HunkStatus::Pending),
            "{got:?}"
        );
    }

    /// Accept all never takes an OutOfScope hunk, and says how many it
    /// skipped (ADR-0033 decision 3).
    #[gpui::test]
    async fn accept_all_skips_an_out_of_scope_hunk_and_says_how_many(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        let state = state_dir("scope-accept-all");
        running_worker(&state, &["notes.txt"]);
        f.window
            .update(cx, |panel, _, _| panel.set_state_dir(state.clone()))
            .unwrap();

        agent_hunk(&f, cx, "n1", "alpha\nBETA\ngamma\n").await;
        out_of_scope_hunk(&f, cx).await;

        f.window
            .update(cx, |panel, _, cx| panel.accept_all(cx))
            .unwrap();
        let said = notice(&f, cx).unwrap();
        assert!(said.contains("accepted 1 hunk(s)"), "{said}");
        assert!(said.contains("1 out-of-scope hunk(s) skipped"), "{said}");
        let got = hunks(&f, cx);
        assert!(
            got.iter()
                .any(|(p, s, _)| p.ends_with("notes.txt") && *s == HunkStatus::Accepted),
            "{got:?}"
        );
        assert!(
            got.iter()
                .any(|(p, s, _)| p.ends_with("other.txt") && *s == HunkStatus::OutOfScope),
            "{got:?}"
        );
    }

    /// Merge is refused while an out-of-scope change is unresolved
    /// (ADR-0033 decision 3), and a resolved review lets the action through.
    #[gpui::test]
    async fn merge_is_refused_while_a_change_is_out_of_scope(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        let state = state_dir("scope-merge");
        running_worker(&state, &["notes.txt"]);
        f.window
            .update(cx, |panel, _, _| panel.set_state_dir(state.clone()))
            .unwrap();
        out_of_scope_hunk(&f, cx).await;

        f.window
            .update(cx, |panel, _, cx| {
                panel.notice = None;
                panel.merge_worker("w1", cx);
                let refused = panel.notice.clone().expect("a refusal notice");
                assert!(refused.contains("merge refused"), "{refused}");
                assert!(refused.contains("1 out-of-scope change(s)"), "{refused}");
                // A person resolves it: the merge is no longer refused.
                panel.notice = None;
                let path = panel.review.path(&panel.review.files()[0], cx);
                let key = panel.review.files()[0].hunks()[0].key.clone();
                panel.review.accept(&path, &key, cx).unwrap();
                panel.merge_worker("w1", cx);
                assert!(
                    panel.notice.is_none(),
                    "resolved, so nothing refuses the merge: {:?}",
                    panel.notice
                );
            })
            .unwrap();
    }

    /// Revert turn keeps an OutOfScope change and says so (ADR-0033
    /// decision 3).
    #[gpui::test]
    async fn revert_turn_keeps_an_out_of_scope_change(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        let state = state_dir("scope-revert");
        running_worker(&state, &["notes.txt"]);
        f.window
            .update(cx, |panel, _, _| panel.set_state_dir(state.clone()))
            .unwrap();
        out_of_scope_hunk(&f, cx).await;

        f.window
            .update(cx, |panel, _, cx| {
                panel.review.end_turn();
                panel.notice = None;
                panel.revert_turn(1, cx);
            })
            .unwrap();
        let said = notice(&f, cx).unwrap();
        assert!(said.contains("kept out-of-scope"), "{said}");
        let got = hunks(&f, cx);
        assert!(
            got.iter()
                .any(|(p, s, _)| p.ends_with("other.txt") && *s == HunkStatus::OutOfScope),
            "the change stays: {got:?}"
        );
    }

    #[gpui::test]
    async fn an_omp_write_to_an_open_buffer_is_one_pending_hunk(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
        tool_end(&f, cx, "c1");
        assert_eq!(
            hunks(&f, cx),
            vec![(
                "ws/notes.txt".to_string(),
                HunkStatus::Pending,
                vec!["c1".to_string()]
            )]
        );
        assert_eq!(
            buffer.read_with(cx, |b, _| b.text()),
            "alpha\nBETA\ngamma\n"
        );
    }

    /// Two calls write the same open file in sequence. The panel rebuilds
    /// the review on every buffer edit, so the second import lands in the
    /// review before it is attributed; it is still the agent's, not a user
    /// edit over the first hunk.
    #[gpui::test]
    async fn two_calls_to_one_file_are_both_pending(cx: &mut TestAppContext) {
        for watcher_first in [false, true] {
            let (f, buffer) = fixture(cx).await;
            tool_start(&f, cx, "c1", &["notes.txt"]);
            omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
            tool_end(&f, cx, "c1");
            tool_start(&f, cx, "c2", &["notes.txt"]);
            omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\nGAMMA\n").await;
            if watcher_first {
                cx.run_until_parked();
                assert_eq!(
                    buffer.read_with(cx, |b, _| b.text()),
                    "alpha\nBETA\nGAMMA\n",
                    "the watcher reloaded before the tool ended"
                );
            }
            tool_end(&f, cx, "c2");
            assert_eq!(
                hunks(&f, cx),
                vec![
                    (
                        "ws/notes.txt".to_string(),
                        HunkStatus::Pending,
                        vec!["c1".to_string()]
                    ),
                    (
                        "ws/notes.txt".to_string(),
                        HunkStatus::Pending,
                        vec!["c2".to_string()]
                    ),
                ],
                "watcher first: {watcher_first}"
            );
            let events = f
                .window
                .update(cx, |panel, _, _| panel.review.drain_events())
                .unwrap();
            assert!(events.is_empty(), "no user edit was reported: {events:?}");
        }
    }

    #[gpui::test]
    async fn a_file_omp_writes_that_is_not_open_is_opened_and_reviewed(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["other.txt"]);
        cx.run_until_parked();
        omp_writes(&f, "/ws/other.txt", "ONE\ntwo\n").await;
        tool_end(&f, cx, "c1");
        assert_eq!(
            hunks(&f, cx),
            vec![(
                "ws/other.txt".to_string(),
                HunkStatus::Pending,
                vec!["c1".to_string()]
            )]
        );
        let (old, new) = f
            .window
            .update(cx, |panel, _, _| {
                let h = &panel.review.files()[0].hunks()[0];
                (h.old_text.clone(), h.new_text.clone())
            })
            .unwrap();
        assert_eq!((old.as_str(), new.as_str()), ("one\n", "ONE\n"));
        let open = f.project.read_with(cx, |p, cx| p.opened_buffers(cx).len());
        assert_eq!(open, 2, "other.txt is open now");
    }

    /// The write lands before the file is open: the disk text at the tool's
    /// start is the baseline, so the write is still one pending hunk.
    #[gpui::test]
    async fn a_write_landing_before_the_open_is_still_a_hunk(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["other.txt"]);
        cx.run_until_parked();
        let open = f.project.read_with(cx, |p, cx| p.opened_buffers(cx).len());
        assert_eq!(open, 1, "the file is not opened before the write lands");
        omp_writes(&f, "/ws/other.txt", "ONE\ntwo\n").await;
        tool_end(&f, cx, "c1");
        let got = hunks(&f, cx);
        assert_eq!(
            got,
            vec![(
                "ws/other.txt".to_string(),
                HunkStatus::Pending,
                vec!["c1".to_string()]
            )]
        );
        let (old, new) = f
            .window
            .update(cx, |panel, _, _| {
                let h = &panel.review.files()[0].hunks()[0];
                (h.old_text.clone(), h.new_text.clone())
            })
            .unwrap();
        assert_eq!((old.as_str(), new.as_str()), ("one\n", "ONE\n"));
    }

    /// OMP does not wait for cedian: its write can land before the panel
    /// reads the file's disk text. OMP's own `oldText` is then the
    /// baseline, so the write is still one pending hunk.
    #[gpui::test]
    async fn a_write_that_beat_the_read_imports_from_omps_old_text(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        omp_writes(&f, "/ws/other.txt", "ONE\ntwo\n").await;
        tool_start(&f, cx, "c1", &["other.txt"]);
        cx.run_until_parked();
        let before = cedian_omp::TextBefore::Text("one\ntwo\n".to_string());
        tool_end_with(&f, cx, "c1", vec![("/ws/other.txt".to_string(), before)]);
        assert_eq!(
            hunks(&f, cx),
            vec![(
                "ws/other.txt".to_string(),
                HunkStatus::Pending,
                vec!["c1".to_string()]
            )]
        );
        let (old, new) = f
            .window
            .update(cx, |panel, _, _| {
                let h = &panel.review.files()[0].hunks()[0];
                (h.old_text.clone(), h.new_text.clone())
            })
            .unwrap();
        assert_eq!((old.as_str(), new.as_str()), ("one\n", "ONE\n"));
    }

    /// The same race when OMP pruned its before-text (past 32 KiB) or its
    /// tool reports none (`write`): the file is listed as not reviewable,
    /// never silently Unchanged.
    async fn raced_without_old_text(
        cx: &mut TestAppContext,
        before: Vec<(String, cedian_omp::TextBefore)>,
    ) {
        let (f, _buffer) = fixture(cx).await;
        omp_writes(&f, "/ws/other.txt", "ONE\ntwo\n").await;
        tool_start(&f, cx, "c1", &["other.txt"]);
        cx.run_until_parked();
        tool_end_with(&f, cx, "c1", before);
        assert!(hunks(&f, cx).is_empty());
        let unreviewable = f
            .window
            .update(cx, |panel, _, _| panel.review.unreviewable().to_vec())
            .unwrap();
        assert_eq!(unreviewable.len(), 1, "{unreviewable:?}");
        assert_eq!(unreviewable[0].0, PathBuf::from("ws/other.txt"));
        assert!(unreviewable[0].1.contains("c1"), "{unreviewable:?}");
    }

    #[gpui::test]
    async fn a_write_that_beat_the_read_with_pruned_old_text_is_not_reviewable(
        cx: &mut TestAppContext,
    ) {
        let pruned = cedian_omp::TextBefore::Pruned;
        raced_without_old_text(cx, vec![("/ws/other.txt".to_string(), pruned)]).await;
    }

    #[gpui::test]
    async fn a_write_that_beat_the_read_with_no_old_text_is_not_reviewable(
        cx: &mut TestAppContext,
    ) {
        raced_without_old_text(cx, Vec::new()).await;
    }

    fn unreviewable(f: &Fixture, cx: &mut TestAppContext) -> Vec<(PathBuf, String)> {
        f.window
            .update(cx, |panel, _, _| panel.review.unreviewable().to_vec())
            .unwrap()
    }

    /// ADR-0047: an open buffer a `bash` call changes on disk is that
    /// call's hunk.
    #[gpui::test]
    async fn a_bash_write_to_an_open_buffer_is_that_calls_hunk(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        tool_start_named(&f, cx, "b1", "bash", &[]);
        cx.run_until_parked();
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
        cx.run_until_parked();
        tool_end_named(&f, cx, "b1", "bash", Vec::new());
        assert_eq!(
            hunks(&f, cx),
            vec![(
                "ws/notes.txt".to_string(),
                HunkStatus::Pending,
                vec!["b1".to_string()]
            )]
        );
        assert!(unreviewable(&f, cx).is_empty());
    }

    /// A command that exits non-zero may still have written: its changes
    /// are its hunks too.
    #[gpui::test]
    async fn a_failed_bash_calls_writes_are_still_its_hunks(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        tool_start_named(&f, cx, "b1", "bash", &[]);
        cx.run_until_parked();
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
        cx.run_until_parked();
        let event = RouterEvent::ToolEnd {
            tool_call_id: "b1".to_string(),
            tool_name: "bash".to_string(),
            result_summary: "exit 1".to_string(),
            is_error: true,
            before: Vec::new(),
        };
        f.window
            .update(cx, |panel, _, cx| panel.on_event(event, cx))
            .unwrap();
        cx.run_until_parked();
        assert_eq!(hunks(&f, cx).len(), 1);
    }

    /// A buffer the person had unsaved edits in is never overwritten by a
    /// bash write: it shows STALE with the call, their text kept.
    #[gpui::test]
    async fn a_bash_write_under_unsaved_edits_is_stale(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        buffer.update(cx, |b, cx| b.edit([(0..0, "USER ")], None, cx));
        tool_start_named(&f, cx, "b1", "bash", &[]);
        cx.run_until_parked();
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
        cx.run_until_parked();
        tool_end_named(&f, cx, "b1", "bash", Vec::new());
        let reason = f
            .window
            .update(cx, |panel, _, _| {
                panel.review.files()[0].stale_import().map(str::to_string)
            })
            .unwrap();
        assert!(
            reason.as_ref().is_some_and(|r| r.contains("b1")),
            "{reason:?}"
        );
        assert_eq!(
            buffer.read_with(cx, |b, _| b.text()),
            "USER alpha\nbeta\ngamma\n"
        );
    }

    /// A file not open has no text from before the call: it is listed as
    /// changed by that call, not reviewable as hunks (owner, ADR-0055).
    #[gpui::test]
    async fn a_bash_write_to_a_file_not_open_is_listed(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        tool_start_named(&f, cx, "b1", "bash", &[]);
        cx.run_until_parked();
        omp_writes(&f, "/ws/other.txt", "ONE\ntwo\n").await;
        cx.run_until_parked();
        tool_end_named(&f, cx, "b1", "bash", Vec::new());
        assert!(hunks(&f, cx).is_empty());
        let listed = unreviewable(&f, cx);
        assert_eq!(listed.len(), 1, "{listed:?}");
        assert_eq!(listed[0].0, PathBuf::from("ws/other.txt"));
        assert!(listed[0].1.contains("bash call b1"), "{listed:?}");
    }

    /// Another mutating call writing the same file while a bash call runs:
    /// which call made which change is unknown, so neither imports it and
    /// the file is shown as such (ADR-0055 decision 6).
    #[gpui::test]
    async fn an_edit_overlapping_a_bash_call_imports_nothing_for_the_file(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        tool_start_named(&f, cx, "b1", "bash", &[]);
        tool_start(&f, cx, "e1", &["notes.txt"]);
        cx.run_until_parked();
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
        cx.run_until_parked();
        tool_end(&f, cx, "e1");
        tool_end_named(&f, cx, "b1", "bash", Vec::new());
        assert!(hunks(&f, cx).is_empty(), "{:?}", hunks(&f, cx));
        let listed = unreviewable(&f, cx);
        assert_eq!(listed.len(), 1, "{listed:?}");
        assert!(
            listed[0].1.contains("b1") && listed[0].1.contains("e1"),
            "{listed:?}"
        );
    }

    /// An open buffer with unsaved edits, a write that beat cedian's disk
    /// read, and no text from OMP (`write` reports none): the file is listed
    /// as changed on disk under the person's edits, never Unchanged, and
    /// their text is kept.
    async fn dirty_raced_without_old_text(
        cx: &mut TestAppContext,
        before: Vec<(String, cedian_omp::TextBefore)>,
    ) {
        let (f, buffer) = fixture(cx).await;
        buffer.update(cx, |b, cx| b.edit([(0..0, "USER ")], None, cx));
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
        tool_start(&f, cx, "e1", &["notes.txt"]);
        cx.run_until_parked();
        tool_end_with(&f, cx, "e1", before);
        assert!(hunks(&f, cx).is_empty());
        let listed = unreviewable(&f, cx);
        assert_eq!(listed.len(), 1, "{listed:?}");
        assert!(
            listed[0].1.contains("e1") && listed[0].1.contains("unsaved"),
            "{listed:?}"
        );
        assert_eq!(
            buffer.read_with(cx, |b, _| b.text()),
            "USER alpha\nbeta\ngamma\n"
        );
    }

    #[gpui::test]
    async fn a_raced_write_under_unsaved_edits_with_no_old_text_is_listed(cx: &mut TestAppContext) {
        dirty_raced_without_old_text(cx, Vec::new()).await;
    }

    #[gpui::test]
    async fn a_raced_write_under_unsaved_edits_with_pruned_old_text_is_listed(
        cx: &mut TestAppContext,
    ) {
        let pruned = cedian_omp::TextBefore::Pruned;
        dirty_raced_without_old_text(cx, vec![("/ws/notes.txt".to_string(), pruned)]).await;
    }

    /// Zed's watcher reloaded a clean open buffer before cedian marked it:
    /// OMP's `oldText` shows the mark is after the write, so the file is
    /// never silently Unchanged.
    #[gpui::test]
    async fn a_clean_buffer_reloaded_before_the_mark_is_not_unchanged(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |b, _| b.text()),
            "alpha\nBETA\ngamma\n"
        );
        tool_start(&f, cx, "e1", &["notes.txt"]);
        cx.run_until_parked();
        let before = cedian_omp::TextBefore::Text(ORIGINAL.to_string());
        tool_end_with(&f, cx, "e1", vec![("/ws/notes.txt".to_string(), before)]);
        let got = hunks(&f, cx);
        let listed = unreviewable(&f, cx);
        assert!(got.is_empty(), "{got:?}");
        assert_eq!(listed.len(), 1, "{listed:?}");
        assert!(
            listed[0].1.contains("before cedian marked it"),
            "{listed:?}"
        );
    }

    /// A CRLF `oldText` is the same text as the LF disk text: a call that
    /// left the file alone is no STALE under the person's unsaved edits.
    #[gpui::test]
    async fn a_crlf_old_text_is_normalized(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        buffer.update(cx, |b, cx| b.edit([(0..0, "USER ")], None, cx));
        tool_start(&f, cx, "c1", &["notes.txt"]);
        cx.run_until_parked();
        let before = cedian_omp::TextBefore::Text(ORIGINAL.replace('\n', "\r\n"));
        tool_end_with(&f, cx, "c1", vec![("/ws/notes.txt".to_string(), before)]);
        let stale = f
            .window
            .update(cx, |panel, _, _| {
                panel
                    .review
                    .files()
                    .iter()
                    .find_map(|f| f.stale_import().map(str::to_string))
            })
            .unwrap();
        assert_eq!(stale, None);
        assert!(unreviewable(&f, cx).is_empty());
    }

    /// A move's before-text is its source's: it is never imported from it.
    #[gpui::test]
    async fn a_moved_file_is_not_reviewable(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["other.txt"]);
        cx.run_until_parked();
        omp_writes(&f, "/ws/other.txt", "ONE\ntwo\n").await;
        let moved = cedian_omp::TextBefore::Moved;
        tool_end_with(&f, cx, "c1", vec![("/ws/other.txt".to_string(), moved)]);
        assert!(hunks(&f, cx).is_empty(), "{:?}", hunks(&f, cx));
        let listed = unreviewable(&f, cx);
        assert_eq!(listed.len(), 1, "{listed:?}");
        assert!(listed[0].1.contains("moved"), "{listed:?}");
    }

    async fn agent_hunk(f: &Fixture, cx: &mut TestAppContext, id: &str, text: &str) {
        tool_start(f, cx, id, &["notes.txt"]);
        cx.run_until_parked();
        omp_writes(f, "/ws/notes.txt", text).await;
        tool_end(f, cx, id);
    }

    /// The overlap path imports nothing, and the watcher's reload of the
    /// agents' write is no edit of the person's: an earlier hunk does not
    /// go STALE and no `user_edited_agent_hunk` row is written.
    #[gpui::test]
    async fn an_overlap_does_not_credit_the_agents_write_to_the_person(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        agent_hunk(&f, cx, "e0", "alpha\nBETA\ngamma\n").await;
        f.window
            .update(cx, |panel, _, _| panel.review.drain_events())
            .unwrap();
        tool_start_named(&f, cx, "b1", "bash", &[]);
        tool_start(&f, cx, "e1", &["notes.txt"]);
        cx.run_until_parked();
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA2\ngamma\n").await;
        cx.run_until_parked();
        tool_end(&f, cx, "e1");
        tool_end_named(&f, cx, "b1", "bash", Vec::new());
        let got = hunks(&f, cx);
        assert!(
            got.iter()
                .all(|(_, status, _)| *status != HunkStatus::Stale),
            "{got:?}"
        );
        let events = f
            .window
            .update(cx, |panel, _, _| panel.review.drain_events())
            .unwrap();
        assert!(
            !events
                .iter()
                .any(|e| matches!(e, crate::review::ReviewEvent::UserEditedAgentHunk { .. })),
            "{events:?}"
        );
        assert_eq!(unreviewable(&f, cx).len(), 1);
    }

    /// Only files are listed, never a directory, an ignored path or the
    /// worktree's initial scan; and the call is named as the time, not the
    /// writer.
    #[gpui::test]
    async fn bash_lists_only_files_that_changed_while_it_ran(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        omp_writes(&f, "/ws/.gitignore", "target/\n").await;
        f.fs.create_dir(std::path::Path::new("/ws/target"))
            .await
            .unwrap();
        cx.run_until_parked();
        tool_start_named(&f, cx, "b1", "bash", &[]);
        cx.run_until_parked();
        f.fs.create_dir(std::path::Path::new("/ws/sub"))
            .await
            .unwrap();
        omp_writes(&f, "/ws/sub/new.txt", "n\n").await;
        omp_writes(&f, "/ws/target/out.txt", "o\n").await;
        cx.run_until_parked();
        tool_end_named(&f, cx, "b1", "bash", Vec::new());
        let listed = unreviewable(&f, cx);
        assert_eq!(listed.len(), 1, "{listed:?}");
        assert_eq!(listed[0].0, PathBuf::from("ws/sub/new.txt"));
        assert!(listed[0].1.contains("while bash call b1 ran"), "{listed:?}");
    }

    /// A running bash call does not freeze the review: a hunk can still be
    /// rejected, and Accept all still runs.
    #[gpui::test]
    async fn a_running_bash_call_does_not_block_review_actions(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        agent_hunk(&f, cx, "e0", "alpha\nBETA\ngamma\nDELTA\n").await;
        tool_start_named(&f, cx, "b1", "bash", &[]);
        cx.run_until_parked();
        let refused = f
            .window
            .update(cx, |panel, _, cx| {
                panel.review.rebuild(cx);
                let path = PathBuf::from("ws/notes.txt");
                let key = panel.review.files()[0].hunks()[0].key.clone();
                let reject = panel.review.reject(&path, &key, cx).err();
                let all = panel.review.accept_all(false, cx).err();
                (reject, all)
            })
            .unwrap();
        assert!(refused.0.is_none() && refused.1.is_none(), "{refused:?}");
    }

    /// OMP dies after a refused `cedian_complete` and before the turn
    /// settles: stopping still ends the workflow's turn, so the refusal
    /// blocks it now, in this turn.
    #[gpui::test]
    async fn stopping_ends_the_workflow_turn(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        let dir = std::env::temp_dir().join(format!("cedian-panel-stop-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut state = cedian_workflow::WorkflowState::start(cedian_workflow::TaskProfile::new(
            "t",
            cedian_workflow::TaskKind::BugFix,
        ))
        .unwrap();
        state.last_completion = Some(cedian_workflow::CompletionAttempt {
            claims: Vec::new(),
            accepted: false,
            missing: vec!["verify".to_string()],
            turn_ended: false,
        });
        cedian_shell::workflow_store::save(&dir, &state).unwrap();
        f.window
            .update(cx, |panel, _, cx| {
                panel.set_state_dir(dir.clone());
                panel.stop("OMP exited".to_string(), cx);
            })
            .unwrap();
        let status = cedian_shell::workflow_store::load(&dir).unwrap().status;
        std::fs::remove_dir_all(&dir).ok();
        assert_eq!(status, cedian_workflow::WorkflowStatus::Blocked);
    }

    /// The correction ledger failing again and again says so once.
    #[gpui::test]
    async fn a_failing_ledger_is_one_notice(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        agent_hunk(&f, cx, "e0", "alpha\nBETA\ngamma\nDELTA\n").await;
        let notice = f
            .window
            .update(cx, |panel, _, cx| {
                let file = std::env::temp_dir()
                    .join(format!("cedian-panel-not-a-dir-{}", std::process::id()));
                std::fs::write(&file, "").unwrap();
                panel.set_state_dir(file);
                panel.review.rebuild(cx);
                let path = PathBuf::from("ws/notes.txt");
                for _ in 0..2 {
                    let key = panel.review.files()[0].hunks()[0].key.clone();
                    panel.reject_hunk(&path, &key, cx);
                }
                format!("{:?} {:?}", panel.notice(), panel.ledger_error)
            })
            .unwrap();
        assert_eq!(notice.matches("did not record").count(), 1, "{notice}");
    }

    /// The person opens the file and types between the tool's start and its
    /// end: their buffer is kept as it is, dirty, and the file shows STALE
    /// with the reason.
    #[gpui::test]
    async fn a_file_the_user_opened_and_typed_in_during_the_call_keeps_their_text(
        cx: &mut TestAppContext,
    ) {
        let (f, _notes) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["other.txt"]);
        cx.run_until_parked();
        let other = f
            .project
            .update(cx, |p, cx| p.open_local_buffer("/ws/other.txt", cx))
            .await
            .unwrap();
        other.update(cx, |b, cx| b.edit([(0..0, "USER ")], None, cx));
        omp_writes(&f, "/ws/other.txt", "ONE\ntwo\n").await;
        tool_end(&f, cx, "c1");
        other.read_with(cx, |b, _| {
            assert_eq!(b.text(), "USER one\ntwo\n", "the user's text is kept");
            assert!(b.is_dirty(), "and still unsaved");
        });
        let (stale, unreviewable) = f
            .window
            .update(cx, |panel, _, _| {
                (
                    panel
                        .review
                        .files()
                        .iter()
                        .filter_map(|f| f.stale_import().map(str::to_string))
                        .collect::<Vec<_>>(),
                    panel.review.unreviewable().to_vec(),
                )
            })
            .unwrap();
        assert_eq!(
            stale,
            vec![
                "the file changed during call c1 while you were editing it \
                 (unsaved edits, or a save during the call); nothing was imported"
                    .to_string()
            ],
            "the file shows STALE with the reason"
        );
        assert!(unreviewable.is_empty(), "{unreviewable:?}");
    }

    /// A write that creates a file: one all-added hunk; reject empties it.
    #[gpui::test]
    async fn a_new_file_is_one_added_hunk_and_reject_empties_it(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["fresh.txt"]);
        cx.run_until_parked();
        omp_writes(&f, "/ws/fresh.txt", "hello\nworld\n").await;
        tool_end(&f, cx, "c1");
        let got = hunks(&f, cx);
        assert_eq!(got.len(), 1, "{got:?}");
        assert_eq!(got[0].0, "ws/fresh.txt");
        let (path, key, old, new) = f
            .window
            .update(cx, |panel, _, cx| {
                let file = &panel.review.files()[0];
                let h = &file.hunks()[0];
                (
                    panel.review.path(file, cx),
                    h.key.clone(),
                    h.old_text.clone(),
                    h.new_text.clone(),
                )
            })
            .unwrap();
        assert_eq!((old.as_str(), new.as_str()), ("", "hello\nworld\n"));
        let buffer = f
            .window
            .update(cx, |panel, _, _| panel.review.files()[0].buffer().clone())
            .unwrap();
        f.window
            .update(cx, |panel, _, cx| panel.reject_hunk(&path, &key, cx))
            .unwrap();
        assert_eq!(buffer.read_with(cx, |b, _| b.text()), "");
    }

    /// A file OMP wrote that could not be opened or imported is still shown.
    #[gpui::test]
    async fn a_file_that_could_not_be_reviewed_is_shown(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, _, cx| {
                panel
                    .review
                    .could_not_review(PathBuf::from("/ws/out.txt"), "permission denied".into());
                assert!(!panel.review.is_empty());
                cx.notify();
            })
            .unwrap();
        let mut vcx = VisualTestContext::from_window(f.window.into(), cx);
        click(&mut vcx, "cedian-review-toggle");
        assert!(
            vcx.debug_bounds("cedian-unreviewed-/ws/out.txt").is_some(),
            "the file is on screen with its reason"
        );
    }

    #[gpui::test]
    async fn a_file_that_cannot_be_opened_is_shown_as_unreviewed(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["/elsewhere/out.txt"]);
        cx.run_until_parked();
        tool_end(&f, cx, "c1");
        let shown = f
            .window
            .update(cx, |panel, _, _| panel.review.unreviewable().to_vec())
            .unwrap();
        assert_eq!(shown.len(), 1, "{shown:?}");
        assert_eq!(shown[0].0, PathBuf::from("/elsewhere/out.txt"));
        let empty = f
            .window
            .update(cx, |panel, _, _| panel.review.is_empty())
            .unwrap();
        assert!(!empty, "the file shows in review");
    }

    #[gpui::test]
    async fn a_refused_import_shows_the_file_stale(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        buffer.update(cx, |b, cx| b.edit([(0..0, "USER ")], None, cx));
        tool_start(&f, cx, "c1", &["notes.txt"]);
        cx.run_until_parked();
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
        tool_end(&f, cx, "c1");
        let reason = f
            .window
            .update(cx, |panel, _, _| {
                panel.review.files()[0].stale_import().map(str::to_string)
            })
            .unwrap();
        assert!(
            reason.as_ref().is_some_and(|r| r.contains("c1")),
            "{reason:?}"
        );
        assert_eq!(
            buffer.read_with(cx, |b, _| b.text()),
            "USER alpha\nbeta\ngamma\n"
        );
    }

    fn click(vcx: &mut VisualTestContext, selector: &str) {
        let selector: &'static str = Box::leak(selector.to_string().into_boxed_str());
        let bounds = vcx
            .debug_bounds(selector)
            .unwrap_or_else(|| panic!("{selector} is not on screen"));
        vcx.simulate_click(bounds.center(), Modifiers::none());
        vcx.run_until_parked();
    }

    #[gpui::test]
    async fn reject_and_accept_buttons_resolve_hunks(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "ALPHA\nbeta\nGAMMA\n").await;
        tool_end(&f, cx, "c1");
        let mut vcx = VisualTestContext::from_window(f.window.into(), cx);
        click(&mut vcx, "cedian-review-toggle");
        assert!(
            vcx.debug_bounds("cedian-review").is_some(),
            "the view is open"
        );
        click(&mut vcx, "cedian-reject-ws/notes.txt-0");
        assert_eq!(
            buffer.read_with(&vcx, |b, _| b.text()),
            "alpha\nbeta\nGAMMA\n",
            "reject put the baseline line back"
        );
        click(&mut vcx, "cedian-accept-ws/notes.txt-0");
        let statuses: Vec<HunkStatus> = hunks(&f, &mut vcx).into_iter().map(|h| h.1).collect();
        assert_eq!(statuses, vec![HunkStatus::Accepted]);
        assert!(
            vcx.debug_bounds("cedian-accept-ws/notes.txt-0").is_none(),
            "a resolved hunk has no buttons"
        );
        buffer.update(&mut vcx, |b, cx| {
            b.undo(cx);
            assert_eq!(
                b.text(),
                "ALPHA\nbeta\nGAMMA\n",
                "one undo restores the reject"
            );
        });
    }

    /// The person restores hunk A by hand and then acts on the Reject they
    /// saw on it: the click carries A's identity, so nothing happens to B,
    /// which now sits where A was. (The harness redraws on the edit, so the
    /// stale frame's handler is invoked directly.)
    #[gpui::test]
    async fn a_click_on_a_hunk_that_moved_does_nothing_to_the_next_one(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "ALPHA\nbeta\nGAMMA\n").await;
        tool_end(&f, cx, "c1");
        let mut vcx = VisualTestContext::from_window(f.window.into(), cx);
        click(&mut vcx, "cedian-review-toggle");
        assert!(vcx.debug_bounds("cedian-reject-ws/notes.txt-1").is_some());
        let (path, key_a) = f
            .window
            .update(&mut vcx, |panel, _, cx| {
                let file = &panel.review.files()[0];
                (panel.review.path(file, cx), file.hunks()[0].key.clone())
            })
            .unwrap();
        buffer.update(&mut vcx, |b, cx| b.edit([(0..5, "alpha")], None, cx));
        vcx.run_until_parked();
        assert!(
            vcx.debug_bounds("cedian-reject-ws/notes.txt-1").is_none(),
            "the view shows what is there: one hunk left"
        );
        f.window
            .update(&mut vcx, |panel, _, cx| {
                panel.reject_hunk(&path, &key_a, cx)
            })
            .unwrap();
        assert_eq!(
            buffer.read_with(&vcx, |b, _| b.text()),
            "alpha\nbeta\nGAMMA\n",
            "B is untouched"
        );
        let hunks = hunks(&f, &mut vcx);
        assert_eq!(hunks.len(), 1, "{hunks:?}");
        assert_eq!(hunks[0].1, HunkStatus::Pending);
        let notice = f
            .window
            .update(&mut vcx, |panel, _, _| panel.notice().map(str::to_string))
            .unwrap();
        assert!(
            notice
                .as_deref()
                .is_some_and(|n| n.contains("moved; review again")),
            "{notice:?}"
        );
    }

    /// Accept all with STALE hunks in the set asks first; Yes takes them
    /// too, No takes only the others (owner ruling 2026-10-08).
    async fn accept_all_with_stale(cx: &mut TestAppContext, answer: &str) -> Vec<HunkStatus> {
        let (f, buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "ALPHA\nbeta\nGAMMA\n").await;
        tool_end(&f, cx, "c1");
        buffer.update(cx, |b, cx| b.edit([(11..11, "!")], None, cx));
        let mut vcx = VisualTestContext::from_window(f.window.into(), cx);
        click(&mut vcx, "cedian-review-toggle");
        assert!(vcx.debug_bounds("cedian-accept-all-confirm").is_none());
        click(&mut vcx, "cedian-accept-all");
        assert!(
            vcx.debug_bounds("cedian-accept-all-confirm").is_some(),
            "asked before anything is accepted"
        );
        let before: Vec<HunkStatus> = hunks(&f, &mut vcx).into_iter().map(|h| h.1).collect();
        assert_eq!(before, vec![HunkStatus::Pending, HunkStatus::Stale]);
        click(&mut vcx, answer);
        assert!(vcx.debug_bounds("cedian-accept-all-confirm").is_none());
        hunks(&f, &mut vcx).into_iter().map(|h| h.1).collect()
    }

    #[gpui::test]
    async fn accept_all_yes_takes_the_stale_hunks_too(cx: &mut TestAppContext) {
        assert_eq!(
            accept_all_with_stale(cx, "cedian-accept-all-yes").await,
            vec![HunkStatus::Accepted, HunkStatus::Accepted]
        );
    }

    #[gpui::test]
    async fn accept_all_no_leaves_the_stale_hunks(cx: &mut TestAppContext) {
        assert_eq!(
            accept_all_with_stale(cx, "cedian-accept-all-no").await,
            vec![HunkStatus::Accepted, HunkStatus::Stale]
        );
    }

    /// The worktree tool's gate rows `(decision, answered_by)` in `dir`.
    fn worktree_rows(dir: &Path) -> Vec<(String, String)> {
        std::fs::read_to_string(dir.join(cedian_shell::audit::AUDIT_FILE))
            .unwrap_or_default()
            .lines()
            .map(|l| serde_json::from_str::<serde_json::Value>(l).unwrap()["item"].clone())
            .filter(|item| item["tool"] == "cedian_worktree_request")
            .map(|item| {
                (
                    item["decision"].as_str().unwrap().to_string(),
                    item["answered_by"].as_str().unwrap().to_string(),
                )
            })
            .collect()
    }

    /// A fresh temp dir for the link the panel's worktree gate audits
    /// through.
    fn worktree_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("cedian-panel-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Put a link with an audit log in `dir` and a worktree gate behind the
    /// panel, returning the ask and the events it sends.
    fn panel_worktree(
        cx: &mut TestAppContext,
        f: &Fixture,
        dir: &Path,
    ) -> (Arc<WorktreeAsk>, mpsc::UnboundedReceiver<LinkEvent>) {
        let (tx, rx) = mpsc::unbounded::<LinkEvent>();
        let link = OmpLink::audit_only(
            tx.clone(),
            dir,
            cedian_omp::SpawnPolicy::default().approvals,
        )
        .unwrap();
        let ask = Arc::new(WorktreeAsk::new(Duration::from_secs(30)));
        ask.wire(tx);
        f.window
            .update(cx, |panel, _, _| {
                panel.link = Some(link);
                panel.worktree_gate = Some(Arc::clone(&ask));
            })
            .unwrap();
        (ask, rx)
    }

    /// Hand the panel the `WorktreeAsked` event the blocked ask sent.
    fn feed_worktree_ask(
        f: &Fixture,
        cx: &mut TestAppContext,
        rx: &mut mpsc::UnboundedReceiver<LinkEvent>,
    ) {
        let event = futures::executor::block_on(rx.next()).expect("the ask sends an event");
        f.window
            .update(cx, |panel, window, cx| {
                panel.on_link_event(event, window, cx)
            })
            .unwrap();
    }

    #[gpui::test]
    async fn a_worktree_request_shows_a_bar_and_an_allow_is_audited(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        let dir = worktree_dir("worktree-allow");
        let (ask, mut rx) = panel_worktree(cx, &f, &dir);
        let asking = Arc::clone(&ask);
        let handle = std::thread::spawn(move || asking.ask("w1", "try it"));
        feed_worktree_ask(&f, cx, &mut rx);

        let mut vcx = VisualTestContext::from_window(f.window.into(), cx);
        assert!(vcx.debug_bounds("cedian-worktree-ask").is_some());
        click(&mut vcx, "cedian-worktree-ask-allow");
        assert!(vcx.debug_bounds("cedian-worktree-ask").is_none());
        assert!(handle.join().unwrap(), "the person allowed it");
        assert_eq!(
            worktree_rows(&dir),
            [("allow".to_string(), "user".to_string())]
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[gpui::test]
    async fn a_worktree_request_deny_is_audited_and_resolves_false(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        let dir = worktree_dir("worktree-deny");
        let (ask, mut rx) = panel_worktree(cx, &f, &dir);
        let asking = Arc::clone(&ask);
        let handle = std::thread::spawn(move || asking.ask("w1", "try it"));
        feed_worktree_ask(&f, cx, &mut rx);

        let mut vcx = VisualTestContext::from_window(f.window.into(), cx);
        click(&mut vcx, "cedian-worktree-ask-deny");
        assert!(!handle.join().unwrap(), "the person denied it");
        assert_eq!(
            worktree_rows(&dir),
            [("deny".to_string(), "user".to_string())]
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The lease firing is the panel's `expire_worktree` — driven directly
    /// rather than waiting five minutes.
    #[gpui::test]
    async fn an_expired_worktree_request_is_denied_and_audited(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        let dir = worktree_dir("worktree-expire");
        let (ask, mut rx) = panel_worktree(cx, &f, &dir);
        let asking = Arc::clone(&ask);
        let handle = std::thread::spawn(move || asking.ask("w1", "try it"));
        feed_worktree_ask(&f, cx, &mut rx);

        f.window
            .update(cx, |panel, _, cx| {
                assert!(panel.worktree_ask.is_some());
                panel.expire_worktree(cx);
            })
            .unwrap();
        assert!(!handle.join().unwrap(), "nobody answered");
        assert_eq!(
            worktree_rows(&dir),
            [("abstain".to_string(), "cedian".to_string())]
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[gpui::test]
    async fn a_late_worktree_answer_only_says_so(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        let dir = worktree_dir("worktree-late");
        let (ask, mut rx) = panel_worktree(cx, &f, &dir);
        let asking = Arc::clone(&ask);
        let handle = std::thread::spawn(move || asking.ask("w1", "try it"));
        feed_worktree_ask(&f, cx, &mut rx);
        f.window
            .update(cx, |panel, _, cx| panel.expire_worktree(cx))
            .unwrap();
        handle.join().unwrap();

        f.window
            .update(cx, |panel, _, cx| panel.answer_worktree(true, cx))
            .unwrap();
        assert_eq!(
            notice(&f, cx),
            Some("that worktree request was already answered".to_string())
        );
        assert_eq!(
            worktree_rows(&dir),
            [("abstain".to_string(), "cedian".to_string())]
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The question is about the STALE hunks there are: when the person
    /// accepts the one STALE hunk while it is open, there is nothing to ask.
    #[gpui::test]
    async fn the_accept_all_question_goes_when_no_hunk_is_stale(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "ALPHA\nbeta\nGAMMA\n").await;
        tool_end(&f, cx, "c1");
        buffer.update(cx, |b, cx| b.edit([(11..11, "!")], None, cx));
        let mut vcx = VisualTestContext::from_window(f.window.into(), cx);
        click(&mut vcx, "cedian-review-toggle");
        click(&mut vcx, "cedian-accept-all");
        assert!(vcx.debug_bounds("cedian-accept-all-confirm").is_some());
        click(&mut vcx, "cedian-accept-ws/notes.txt-1");
        assert!(
            vcx.debug_bounds("cedian-accept-all-confirm").is_none(),
            "no STALE hunk is left to ask about"
        );
        let statuses: Vec<HunkStatus> = hunks(&f, &mut vcx).into_iter().map(|h| h.1).collect();
        assert_eq!(statuses, vec![HunkStatus::Pending, HunkStatus::Accepted]);
    }

    #[gpui::test]
    async fn accept_all_without_stale_does_not_ask(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "ALPHA\nbeta\nGAMMA\n").await;
        tool_end(&f, cx, "c1");
        let mut vcx = VisualTestContext::from_window(f.window.into(), cx);
        click(&mut vcx, "cedian-review-toggle");
        click(&mut vcx, "cedian-accept-all");
        assert!(vcx.debug_bounds("cedian-accept-all-confirm").is_none());
        let after: Vec<HunkStatus> = hunks(&f, &mut vcx).into_iter().map(|h| h.1).collect();
        assert_eq!(after, vec![HunkStatus::Accepted, HunkStatus::Accepted]);
    }

    #[gpui::test]
    async fn the_revert_turn_button_puts_the_turn_back(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "ALPHA\nbeta\ngamma\n").await;
        tool_end(&f, cx, "c1");
        let mut vcx = VisualTestContext::from_window(f.window.into(), cx);
        click(&mut vcx, "cedian-review-toggle");
        settled(&f, &mut vcx);
        click(&mut vcx, "cedian-revert-turn");
        assert_eq!(buffer.read_with(&vcx, |b, _| b.text()), ORIGINAL);
        let notice = f
            .window
            .update(&mut vcx, |panel, _, _| panel.notice().map(str::to_string))
            .unwrap();
        assert_eq!(
            notice.as_deref(),
            Some(
                "turn 1 reverted: 1 hunk(s) put back, 0 STALE kept, \
                 0 changed again by a later turn, kept, 0 accepted, kept"
            )
        );
    }

    /// U10c (ADR-0056 decision 3): an inline edit queued during a turn
    /// gets its own turn in the review when OMP takes it from its queue.
    #[gpui::test]
    async fn a_queued_inline_edit_begins_its_turn_when_omp_runs_it(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, _, cx| {
                let edit = edit_of(&buffer, 1..2, "shout", cx);
                let message = edit.message(cx);
                panel.follow_ups.push(QueuedFollowUp {
                    message: message.clone(),
                    edit: Some(edit),
                    shown: None,
                    promoting: false,
                });
                let queue = |follow_up: Vec<String>| RouterEvent::Queue {
                    steering: Vec::new(),
                    follow_up,
                };
                panel.on_event(queue(vec![message]), cx);
                assert_eq!(panel.review.current_turn(), 1, "still queued");
                panel.on_event(queue(Vec::new()), cx);
                assert_eq!(panel.review.current_turn(), 2);
                assert_eq!(panel.review.turn_label(2), Some("notes.txt:2-2 shout"));
                assert!(panel.follow_ups.is_empty());
            })
            .unwrap();
    }

    /// U10e (ADR-0056 decision 6): an inline edit that changed a line
    /// outside its selection is named in a notice, and its hunk is kept.
    #[gpui::test]
    async fn an_inline_edit_outside_its_selection_is_warned_about(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, _, cx| {
                panel.review.end_turn();
                let edit = edit_of(&buffer, 0..1, "shout", cx);
                panel.begin_edit_turn(edit, cx);
            })
            .unwrap();
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "ALPHA\nbeta\nGAMMA\n").await;
        tool_end(&f, cx, "c1");
        settled(&f, cx);
        let notice = f
            .window
            .update(cx, |p, _, _| p.notice().map(str::to_string))
            .unwrap();
        assert_eq!(
            notice.as_deref(),
            Some(
                "the inline edit changed outside its selection: notes.txt:3-3; \
                 review it in Review Changes"
            )
        );
        assert_eq!(
            buffer.read_with(cx, |b, _| b.text()),
            "ALPHA\nbeta\nGAMMA\n"
        );
    }

    fn edit_of(
        buffer: &Entity<Buffer>,
        rows: std::ops::Range<u32>,
        instruction: &str,
        cx: &App,
    ) -> InlineEdit {
        let selection = language::Point::new(rows.start, 0)..language::Point::new(rows.end, 0);
        InlineEdit {
            instruction: instruction.to_string(),
            ..InlineEdit::new("notes.txt".to_string(), buffer.clone(), selection, cx)
        }
    }

    fn queue_of(follow_up: Vec<String>) -> RouterEvent {
        RouterEvent::Queue {
            steering: Vec::new(),
            follow_up,
        }
    }

    fn follow_up(message: &str, edit: Option<InlineEdit>) -> QueuedFollowUp {
        QueuedFollowUp {
            message: message.to_string(),
            edit,
            shown: None,
            promoting: false,
        }
    }

    fn promoted(text: &str, result: Result<bool, String>) -> LinkEvent {
        LinkEvent::Promoted {
            text: text.to_string(),
            result,
        }
    }

    /// Steer now that OMP refuses because it already ran the follow-up: the
    /// follow-up had its own turn when it left the queue, no chip comes
    /// back, and no later queue update "drains" it into a second turn.
    #[gpui::test]
    async fn a_refused_promote_of_a_follow_up_omp_ran_leaves_no_ghost(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, window, cx| {
                panel.set_turn(Turn::Streaming);
                panel.follow_ups.push(follow_up("later", None));
                panel.on_event(queue_of(vec!["later".to_string()]), cx);
                let before = panel.review.current_turn();
                assert_eq!(panel.begin_promote(0).as_deref(), Some("later"));
                panel.on_event(queue_of(Vec::new()), cx);
                assert_eq!(panel.review.current_turn(), before + 1, "it ran: its turn");
                panel.on_link_event(promoted("later", Ok(false)), window, cx);
                assert!(panel.queued_follow_ups().is_empty(), "no ghost chip");
                panel.on_event(queue_of(Vec::new()), cx);
                assert_eq!(panel.review.current_turn(), before + 1, "no second turn");
                assert_eq!(
                    panel.notice(),
                    Some("OMP ran \"later\" as its own turn before it could steer")
                );
            })
            .unwrap();
    }

    /// Steer now that OMP accepts: the follow-up leaves the follow-up queue
    /// for the steering one, so it joins the running turn (no turn of its
    /// own); one OMP refuses while still queued stays a chip.
    #[gpui::test]
    async fn a_promoted_follow_up_joins_the_turn_and_a_refused_one_stays(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, window, cx| {
                panel.set_turn(Turn::Streaming);
                panel.follow_ups.push(follow_up("later", None));
                panel.follow_ups.push(follow_up("after", None));
                panel.on_event(queue_of(vec!["later".to_string(), "after".to_string()]), cx);
                let before = panel.review.current_turn();
                panel.begin_promote(0);
                panel.on_event(
                    RouterEvent::Queue {
                        steering: vec!["later".to_string()],
                        follow_up: vec!["after".to_string()],
                    },
                    cx,
                );
                panel.on_link_event(promoted("later", Ok(true)), window, cx);
                assert_eq!(panel.review.current_turn(), before, "joined the turn");
                assert_eq!(panel.queued_follow_ups(), ["after"]);
                panel.begin_promote(0);
                panel.on_link_event(promoted("after", Ok(false)), window, cx);
                assert_eq!(panel.queued_follow_ups(), ["after"], "still queued");
                panel.on_event(queue_of(vec!["after".to_string()]), cx);
                assert_eq!(panel.review.current_turn(), before);
            })
            .unwrap();
    }

    /// A slash command or template OMP expanded: its queue lists it by the
    /// expanded text in both queues, so Steer now still joins the turn.
    #[gpui::test]
    async fn a_promoted_expanded_follow_up_joins_the_turn(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, window, cx| {
                panel.set_turn(Turn::Streaming);
                panel.follow_ups.push(follow_up("/x", None));
                panel.on_event(queue_of(vec!["expanded x".to_string()]), cx);
                let before = panel.review.current_turn();
                panel.begin_promote(0);
                panel.on_event(
                    RouterEvent::Queue {
                        steering: vec!["expanded x".to_string()],
                        follow_up: Vec::new(),
                    },
                    cx,
                );
                panel.on_link_event(promoted("/x", Ok(true)), window, cx);
                assert_eq!(panel.review.current_turn(), before, "joined the turn");
                assert!(panel.queued_follow_ups().is_empty());
            })
            .unwrap();
    }

    /// A launch that fails clears the `policy = "omp"` badge placeholder.
    #[gpui::test]
    async fn a_failed_launch_leaves_no_policy_badge(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, window, cx| {
                panel.omp_policy = Some("OMP policy (reading it…)".to_string());
                panel.omp_warning = Some("omp 18.7.0, pin 18.6.1".to_string());
                panel.on_link_event(LinkEvent::Failed("no omp".to_string()), window, cx);
                assert_eq!(panel.omp_policy_badge(), None);
                assert_eq!(panel.omp_warning(), None);
            })
            .unwrap();
    }

    fn toast(text: &str) -> LinkEvent {
        LinkEvent::Event(RouterEvent::Toast(cedian_omp::Toast {
            level: cedian_omp::ToastLevel::Info,
            text: text.to_string(),
        }))
    }

    /// Dismiss names its toast, not a place: a toast arriving between the
    /// render and the click (the oldest falls off) does not shift it.
    #[gpui::test]
    async fn dismiss_removes_the_toast_it_was_rendered_with(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, window, cx| {
                for i in 0..MAX_TOASTS {
                    panel.on_link_event(toast(&format!("t{i}")), window, cx);
                }
                let handle = panel.toast_handles()[3];
                panel.on_link_event(toast("late"), window, cx);
                panel.dismiss_toast(handle);
                let texts: Vec<String> = panel.toasts().iter().map(|t| t.text.clone()).collect();
                assert!(!texts.contains(&"t3".to_string()), "{texts:?}");
                assert!(texts.contains(&"t4".to_string()), "{texts:?}");
            })
            .unwrap();
    }

    /// Restart forgets the old OMP's picker, toasts and notes; a settled
    /// turn or a stop leaves no Stop retry; the notes are capped.
    #[gpui::test]
    async fn restart_and_settle_forget_the_old_session_state(cx: &mut TestAppContext) {
        let (f, _buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, window, cx| {
                panel.picker.model = Some("anthropic/claude-x".to_string());
                panel.on_link_event(toast("old"), window, cx);
                for i in 0..100 {
                    panel.on_link_event(
                        LinkEvent::Event(RouterEvent::RunNote {
                            text: format!("note {i}"),
                            retrying: Some(true),
                        }),
                        window,
                        cx,
                    );
                }
                assert!(panel.session_notes().len() <= 32, "capped");
                assert_eq!(panel.session_notes().last().unwrap(), "note 99");
                panel.on_event(RouterEvent::Settled, cx);
                assert!(!panel.retrying(), "no Stop retry once settled");
                panel.retrying = true;
                panel.stop("gone".to_string(), cx);
                assert!(!panel.retrying(), "no Stop retry once stopped");
                panel.forget_old_omp(window, cx);
                assert_eq!(panel.picker().model, None);
                assert!(panel.toasts().is_empty());
                assert!(panel.session_notes().is_empty());
            })
            .unwrap();
    }

    /// ADR-0057 decision 5 in the app: the person undoes the agent's hunk
    /// and redoes it. The panel rebuilds the review on each of those edits,
    /// so the hunk is STALE: Revert turn keeps the person's text and names
    /// the hunk it kept.
    #[gpui::test]
    async fn undo_then_redo_of_an_agent_hunk_is_kept_by_revert_turn(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
        tool_end(&f, cx, "c1");
        settled(&f, cx);
        buffer.update(cx, |b, cx| {
            b.undo(cx);
        });
        cx.run_until_parked();
        assert_eq!(buffer.read_with(cx, |b, _| b.text()), ORIGINAL);
        buffer.update(cx, |b, cx| {
            b.redo(cx);
        });
        cx.run_until_parked();
        f.window
            .update(cx, |panel, _, cx| {
                let turn = panel.review.current_turn();
                panel.revert_turn(turn, cx);
                let notice = panel.notice().unwrap_or_default().to_string();
                assert!(notice.contains("/notes.txt line 2"), "{notice}");
                assert!(notice.contains("edited after the agent"), "{notice}");
            })
            .unwrap();
        assert_eq!(
            buffer.read_with(cx, |b, _| b.text()),
            "alpha\nBETA\ngamma\n",
            "nothing written over the redone text"
        );
    }

    /// U10 fix 1: Restart while an inline edit is queued opens no edit turn
    /// in the next OMP, and names the edit that did not run.
    #[gpui::test]
    async fn restart_with_a_queued_edit_opens_no_phantom_turn(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, window, cx| {
                let edit = edit_of(&buffer, 1..2, "shout", cx);
                let message = edit.message(cx);
                panel.follow_ups.push(follow_up(&message, Some(edit)));
                panel.on_event(queue_of(vec![message]), cx);
                panel.on_link_event(LinkEvent::Event(RouterEvent::Disconnected), window, cx);
                panel.forget_old_omp(window, cx);
                panel.on_event(queue_of(Vec::new()), cx);
                assert_eq!(panel.review.current_turn(), 1, "no phantom turn");
                assert_eq!(panel.review.turn_label(1), None);
                assert_eq!(panel.input.read(cx).text(cx), "", "not in the composer");
                assert_eq!(
                    panel.notice(),
                    Some(
                        "the queued inline edit did not run: notes.txt:2-2 shout \
                         (instruction: shout); select the lines and press \
                         ctrl-enter to send it again"
                    )
                );
            })
            .unwrap();
    }

    /// U10 fix 2: a chat follow-up OMP drains during an inline edit's turn
    /// is a turn of its own: not labelled, not warned about, and reverting
    /// the edit keeps its write.
    #[gpui::test]
    async fn a_chat_follow_up_after_an_inline_edit_is_its_own_turn(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, _, cx| {
                panel.review.end_turn();
                let edit = edit_of(&buffer, 0..1, "shout", cx);
                panel.begin_edit_turn(edit, cx);
                panel.follow_ups.push(follow_up("and gamma too", None));
                panel.on_event(queue_of(vec!["and gamma too".to_string()]), cx);
            })
            .unwrap();
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "ALPHA\nbeta\ngamma\n").await;
        tool_end(&f, cx, "c1");
        f.window
            .update(cx, |panel, _, cx| panel.on_event(queue_of(Vec::new()), cx))
            .unwrap();
        tool_start(&f, cx, "c2", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "ALPHA\nbeta\nGAMMA\n").await;
        tool_end(&f, cx, "c2");
        settled(&f, cx);
        f.window
            .update(cx, |panel, _, cx| {
                assert_eq!(panel.review.current_turn(), 3);
                assert_eq!(panel.review.turn_label(3), None);
                assert_eq!(panel.notice(), None, "the chat's write is not the edit's");
                panel.revert_turn(2, cx);
            })
            .unwrap();
        assert_eq!(
            buffer.read_with(cx, |b, _| b.text()),
            "alpha\nbeta\nGAMMA\n"
        );
    }

    /// U10 fix 2 (`followUpMode = all`): two follow-ups leaving OMP's
    /// queue at once are one turn, and the notice says so.
    #[gpui::test]
    async fn follow_ups_drained_together_are_one_named_turn(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, _, cx| {
                let (a, b) = (
                    edit_of(&buffer, 0..1, "shout", cx),
                    edit_of(&buffer, 2..3, "whisper", cx),
                );
                let (ma, mb) = (a.message(cx), b.message(cx));
                panel.follow_ups.push(follow_up(&ma, Some(a)));
                panel.follow_ups.push(follow_up(&mb, Some(b)));
                panel.on_event(queue_of(vec![ma, mb]), cx);
                panel.on_event(queue_of(Vec::new()), cx);
                assert_eq!(panel.review.current_turn(), 2);
                assert_eq!(panel.review.turn_label(2), None);
                assert_eq!(
                    panel.notice(),
                    Some("OMP ran 2 queued messages as one turn (turn 2)")
                );
            })
            .unwrap();
    }

    /// U10 fix 3: a refused follow-up forgets that follow-up, not another;
    /// of identical texts, one leaving the queue is one turn.
    #[gpui::test]
    async fn a_refused_follow_up_forgets_only_itself(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, window, cx| {
                let edit = edit_of(&buffer, 1..2, "shout", cx);
                let message = edit.message(cx);
                panel.follow_ups.push(follow_up("chat", None));
                panel.follow_ups.push(follow_up(&message, Some(edit)));
                let refused = LinkEvent::QueueRefused {
                    text: "chat".to_string(),
                    reason: "Stop came first; it stays here".to_string(),
                };
                panel.on_link_event(refused, window, cx);
                assert_eq!(panel.follow_ups.len(), 1);
                assert_eq!(panel.follow_ups[0].message, message);
                panel.follow_ups.clear();
                panel.follow_ups.push(follow_up("same", None));
                panel.follow_ups.push(follow_up("same", None));
                let same = || "same".to_string();
                panel.on_event(queue_of(vec![same(), same()]), cx);
                panel.on_event(queue_of(vec![same()]), cx);
                assert_eq!(panel.review.current_turn(), 2, "one drained, one turn");
                assert_eq!(panel.follow_ups.len(), 1);
                panel.on_event(queue_of(Vec::new()), cx);
                assert_eq!(panel.review.current_turn(), 3);
            })
            .unwrap();
    }

    /// U10 fixes 5 and 7: the instruction block keeps its text while OMP
    /// may still refuse it, and a second ctrl-enter does not drop it.
    #[gpui::test]
    async fn the_instruction_block_keeps_its_text(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        bind_default_keymap(cx);
        let (workspace, editor) = open_in_workspace(&f, &buffer, cx);
        let mut vcx = VisualTestContext::from_window(workspace.into(), cx);
        editor.update_in(&mut vcx, |editor, window, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.select_ranges([language::Point::new(1, 0)..language::Point::new(1, 4)])
            });
        });
        vcx.update(|window, _| window.refresh());
        vcx.run_until_parked();
        vcx.simulate_keystrokes("ctrl-enter");
        vcx.run_until_parked();
        vcx.simulate_input("shout");
        editor.update_in(&mut vcx, |editor, window, cx| {
            editor.focus_handle(cx).focus(window, cx)
        });
        vcx.simulate_keystrokes("ctrl-enter");
        vcx.run_until_parked();
        let typed = |vcx: &mut VisualTestContext| {
            f.window
                .read_with(vcx, |p, cx| {
                    p.instruction.as_ref().map(|b| b.input.read(cx).text(cx))
                })
                .unwrap()
        };
        assert_eq!(
            typed(&mut vcx).as_deref(),
            Some("shout"),
            "second ctrl-enter"
        );
        // Sent as a follow-up while a turn runs: OMP refuses it.
        let message = f
            .window
            .update(&mut vcx, |p, _, cx| {
                let block = p.instruction.as_mut().unwrap();
                let edit = InlineEdit {
                    instruction: "shout".to_string(),
                    ..block.edit.clone()
                };
                let message = edit.message(cx);
                block.pending = Some(message.clone());
                p.follow_ups.push(follow_up(&message, Some(edit)));
                message
            })
            .unwrap();
        f.window
            .update(&mut vcx, |p, window, cx| {
                let refused = LinkEvent::QueueRefused {
                    text: message.clone(),
                    reason: "pipe".to_string(),
                };
                p.on_link_event(refused, window, cx);
            })
            .unwrap();
        assert_eq!(
            typed(&mut vcx).as_deref(),
            Some("shout"),
            "refused keeps it"
        );
        assert_eq!(
            notice(&f, &mut vcx).as_deref(),
            Some("OMP did not queue the inline edit: pipe; it stays in its block")
        );
        f.window
            .update(&mut vcx, |p, window, cx| {
                p.instruction.as_mut().unwrap().pending = Some(message.clone());
                p.on_link_event(LinkEvent::Queued(message.clone()), window, cx);
                assert!(!p.instruction_open(), "queued: the block goes");
            })
            .unwrap();
        // U10 fix round 2, M2: Restart, or a refusal of no tracked
        // follow-up, unblocks a pending block and keeps its text.
        editor.update_in(&mut vcx, |editor, window, cx| {
            editor.focus_handle(cx).focus(window, cx)
        });
        vcx.simulate_keystrokes("ctrl-enter");
        vcx.run_until_parked();
        vcx.simulate_input("shout");
        f.window
            .update(&mut vcx, |p, window, cx| {
                p.instruction.as_mut().unwrap().pending = Some(message.clone());
                p.forget_old_omp(window, cx);
                let block = p.instruction.as_ref().unwrap();
                assert_eq!(block.pending, None, "restart");
                assert_eq!(block.input.read(cx).text(cx), "shout");
                p.instruction.as_mut().unwrap().pending = Some(message.clone());
                let refused = LinkEvent::QueueRefused {
                    text: message.clone(),
                    reason: "pipe".to_string(),
                };
                p.on_link_event(refused, window, cx);
                assert_eq!(p.instruction.as_ref().unwrap().pending, None, "refused");
            })
            .unwrap();
    }

    /// U10 fix 6: Stop's take-back puts a chat follow-up in the composer
    /// and names a queued inline edit with its instruction.
    #[gpui::test]
    async fn stop_takes_back_an_inline_edit_to_the_notice(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, window, cx| {
                let edit = edit_of(&buffer, 1..2, "shout", cx);
                let message = edit.message(cx);
                panel.follow_ups.push(follow_up(&message, Some(edit)));
                panel.follow_ups.push(follow_up("chat", None));
                let restored = LinkEvent::Restored(vec![message, "chat".to_string()]);
                panel.on_link_event(restored, window, cx);
                assert_eq!(panel.input.read(cx).text(cx), "chat");
                assert_eq!(
                    panel.notice(),
                    Some(
                        "the queued inline edit did not run: notes.txt:2-2 shout \
                         (instruction: shout); select the lines and press \
                         ctrl-enter to send it again"
                    )
                );
                assert!(panel.follow_ups.is_empty());
            })
            .unwrap();
    }

    /// U10 fix 8: a stopped inline edit is checked when it stops, and lines
    /// added right after the selection are outside it.
    #[gpui::test]
    async fn a_stopped_inline_edit_is_checked_at_once(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, _, cx| {
                panel.review.end_turn();
                let edit = edit_of(&buffer, 0..1, "shout", cx);
                panel.begin_edit_turn(edit, cx);
            })
            .unwrap();
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "alpha\nNEW\nbeta\ngamma\n").await;
        tool_end(&f, cx, "c1");
        f.window
            .update(cx, |panel, window, cx| {
                panel.on_link_event(LinkEvent::PromptStopped("pipe".into()), window, cx)
            })
            .unwrap();
        assert_eq!(
            notice(&f, cx).as_deref(),
            Some(
                "turn stopped: pipe; the inline edit changed outside its selection: \
                 notes.txt:2-2; review it in Review Changes"
            )
        );
    }

    /// U10 fix round 2, M1: OMP lists the queue empty before it answers
    /// `abort_and_restore_queue` (tests/fixtures/turn_lifecycle_1.jsonl:31-33),
    /// so during Stop a vanished follow-up is taken back, not drained: no
    /// turn opens, the chat text goes to the composer and the edit is named.
    #[gpui::test]
    async fn stop_takes_back_queued_follow_ups_in_omps_order(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, window, cx| {
                let edit = edit_of(&buffer, 1..2, "shout", cx);
                let message = edit.message(cx);
                panel.follow_ups.push(follow_up(&message, Some(edit)));
                panel.follow_ups.push(follow_up("chat", None));
                panel.on_event(queue_of(vec![message.clone(), "chat".to_string()]), cx);
                panel.turn = Turn::Stopping;
                panel.on_event(queue_of(vec!["chat".to_string()]), cx);
                panel.on_event(queue_of(Vec::new()), cx);
                let restored = LinkEvent::Restored(vec![message, "chat".to_string()]);
                panel.on_link_event(restored, window, cx);
                assert_eq!(panel.review.current_turn(), 1, "no phantom turn");
                assert_eq!(panel.review.turn_label(1), None);
                assert_eq!(panel.input.read(cx).text(cx), "chat");
                assert_eq!(
                    panel.notice(),
                    Some(
                        "the queued inline edit did not run: notes.txt:2-2 shout \
                         (instruction: shout); select the lines and press \
                         ctrl-enter to send it again"
                    )
                );
                assert!(panel.follow_ups.is_empty());
            })
            .unwrap();
    }

    /// U10 fix round 2, M1: after OMP settles it holds no follow-up, so a
    /// listed one the panel still tracks is forgotten.
    #[gpui::test]
    async fn settling_forgets_listed_follow_ups(cx: &mut TestAppContext) {
        let (f, _) = fixture(cx).await;
        f.window
            .update(cx, |panel, _, cx| {
                panel.follow_ups.push(follow_up("listed", None));
                panel.follow_ups.push(follow_up("sent", None));
                panel.on_event(queue_of(vec!["listed".to_string()]), cx);
                panel.on_event(RouterEvent::Settled, cx);
                assert_eq!(panel.follow_ups.len(), 1);
                assert_eq!(panel.follow_ups[0].message, "sent");
            })
            .unwrap();
    }

    /// U10 fix round 2, L2: a slash command OMP lists under its expanded
    /// text is still a turn of its own when OMP runs it.
    #[gpui::test]
    async fn a_follow_up_listed_under_its_expansion_is_its_own_turn(cx: &mut TestAppContext) {
        let (f, _) = fixture(cx).await;
        f.window
            .update(cx, |panel, _, cx| {
                panel.follow_ups.push(follow_up("/fix", None));
                panel.on_event(queue_of(vec!["Fix the failing test".to_string()]), cx);
                assert_eq!(panel.review.current_turn(), 1, "still queued");
                panel.on_event(queue_of(Vec::new()), cx);
                assert_eq!(panel.review.current_turn(), 2);
                assert!(panel.follow_ups.is_empty());
            })
            .unwrap();
    }

    /// U10 fix round 2, L1: a line inserted right before the selection is
    /// outside it.
    #[gpui::test]
    async fn a_line_added_before_the_selection_is_outside_it(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, _, cx| {
                panel.review.end_turn();
                let edit = edit_of(&buffer, 1..2, "shout", cx);
                panel.begin_edit_turn(edit, cx);
            })
            .unwrap();
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "alpha\nNEW\nbeta\ngamma\n").await;
        tool_end(&f, cx, "c1");
        settled(&f, cx);
        assert_eq!(
            notice(&f, cx).as_deref(),
            Some(
                "the inline edit changed outside its selection: notes.txt:2-2; \
                 review it in Review Changes"
            )
        );
    }

    /// Zed's default keymap for this platform, as the app loads it; the
    /// actions this test binary does not link fail to load and are skipped.
    fn bind_default_keymap(cx: &mut TestAppContext) {
        #[cfg(target_os = "macos")]
        let keymap = include_str!("../../../assets/keymaps/default-macos.json");
        #[cfg(target_os = "windows")]
        let keymap = include_str!("../../../assets/keymaps/default-windows.json");
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        let keymap = include_str!("../../../assets/keymaps/default-linux.json");
        cx.update(|cx| {
            let bindings = match settings::KeymapFile::load(keymap, cx) {
                settings::KeymapFileLoadResult::Success { key_bindings, .. } => key_bindings,
                settings::KeymapFileLoadResult::SomeFailedToLoad { key_bindings, .. } => {
                    key_bindings
                }
                settings::KeymapFileLoadResult::JsonParseFailure { error } => {
                    panic!("the default keymap parses: {error}")
                }
            };
            cx.bind_keys(bindings);
        });
    }

    #[cfg(target_os = "macos")]
    const REVERT_TURN_KEY: &str = "cmd-alt-shift-z";
    #[cfg(not(target_os = "macos"))]
    const REVERT_TURN_KEY: &str = "ctrl-alt-shift-z";

    /// The panel docked in a workspace with notes.txt open and focused in
    /// an editor, as the person works.
    fn open_in_workspace(
        f: &Fixture,
        buffer: &Entity<Buffer>,
        cx: &mut TestAppContext,
    ) -> (
        gpui::WindowHandle<workspace::MultiWorkspace>,
        Entity<Editor>,
    ) {
        cx.update(|cx| super::init(cx));
        let panel = f.window.root(cx).unwrap();
        let window = cx.add_window(|window, cx| {
            workspace::MultiWorkspace::test_new(f.project.clone(), window, cx)
        });
        let editor = window
            .update(cx, |multi, window, cx| {
                multi.workspace().clone().update(cx, |workspace, cx| {
                    workspace.add_panel(panel.clone(), window, cx);
                    let editor = cx.new(|cx| {
                        Editor::for_buffer(buffer.clone(), Some(f.project.clone()), window, cx)
                    });
                    workspace.add_item_to_active_pane(
                        Box::new(editor.clone()),
                        None,
                        true,
                        window,
                        cx,
                    );
                    editor.focus_handle(cx).focus(window, cx);
                    editor
                })
            })
            .unwrap();
        let handle = window
            .read_with(cx, |multi, _| multi.workspace().downgrade())
            .unwrap();
        panel.update(cx, |panel, cx| panel.set_workspace(handle, cx));
        cx.run_until_parked();
        (window, editor)
    }

    /// U10f (ADR-0056 decisions 1-2): ctrl-enter in an editor opens
    /// cedian's instruction block over the selection; Enter sends it through
    /// the inline edit (here OMP is stopped, so the block keeps its text and
    /// says why); Escape closes it and the editor has the focus again.
    #[gpui::test]
    async fn ctrl_enter_opens_the_instruction_block(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        bind_default_keymap(cx);
        let (workspace, editor) = open_in_workspace(&f, &buffer, cx);
        f.window
            .update(cx, |panel, _, _| {
                panel.connection = Connection::Stopped("OMP stopped".to_string());
            })
            .unwrap();
        let mut vcx = VisualTestContext::from_window(workspace.into(), cx);
        editor.update_in(&mut vcx, |editor, window, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.select_ranges([language::Point::new(1, 0)..language::Point::new(1, 4)])
            });
        });
        vcx.update(|window, _| window.refresh());
        vcx.run_until_parked();
        vcx.simulate_keystrokes("ctrl-enter");
        vcx.update(|window, _| window.refresh());
        vcx.run_until_parked();
        assert!(
            vcx.debug_bounds("cedian-inline-edit").is_some(),
            "the instruction block is in the editor"
        );
        vcx.simulate_input("shout");
        vcx.simulate_keystrokes("enter");
        vcx.run_until_parked();
        assert_eq!(
            notice(&f, &mut vcx).as_deref(),
            Some("OMP stopped; restart OMP")
        );
        let open = f
            .window
            .read_with(&vcx, |p, _| p.instruction_open())
            .unwrap();
        assert!(open, "a refused edit keeps its block");
        vcx.simulate_keystrokes("escape");
        vcx.run_until_parked();
        let open = f
            .window
            .read_with(&vcx, |p, _| p.instruction_open())
            .unwrap();
        assert!(!open);
        let focused = vcx.update(|window, cx| editor.focus_handle(cx).is_focused(window));
        assert!(focused, "the editor has the focus again");
        assert_eq!(buffer.read_with(&vcx, |b, _| b.text()), ORIGINAL);
    }

    /// U10a (ADR-0056 decision 7): revert turn from the editor, by its key
    /// and the palette's action: the latest turn goes back, a STALE hunk is
    /// kept, and one undo redoes it.
    #[gpui::test]
    async fn revert_turn_from_the_editor_puts_the_latest_turn_back(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "ALPHA\nbeta\nGAMMA\n").await;
        tool_end(&f, cx, "c1");
        settled(&f, cx);
        buffer.update(cx, |b, cx| b.edit([(11..16, "GAMMA!")], None, cx));
        bind_default_keymap(cx);
        let (workspace, _editor) = open_in_workspace(&f, &buffer, cx);
        let mut vcx = VisualTestContext::from_window(workspace.into(), cx);
        vcx.update(|window, _| window.refresh());
        vcx.run_until_parked();
        vcx.simulate_keystrokes(REVERT_TURN_KEY);
        vcx.run_until_parked();
        assert_eq!(
            buffer.read_with(&vcx, |b, _| b.text()),
            "alpha\nbeta\nGAMMA!\n"
        );
        assert_eq!(
            notice(&f, &mut vcx).as_deref(),
            Some(
                "turn 1 reverted: 1 hunk(s) put back, 1 STALE kept, \
                 0 changed again by a later turn, kept, 0 accepted, kept; kept as STALE, \
                 edited after the agent wrote it (an undo then redo counts as an edit): \
                 /notes.txt line 3"
            )
        );
        buffer.update(&mut vcx, |b, cx| {
            b.undo(cx);
        });
        assert_eq!(
            buffer.read_with(&vcx, |b, _| b.text()),
            "ALPHA\nbeta\nGAMMA!\n"
        );
        let palette = vcx.update(|window, cx| {
            window
                .available_actions(cx)
                .iter()
                .any(|a| a.partial_eq(&RevertTurn))
        });
        assert!(
            palette,
            "the command palette lists Revert Turn in the editor"
        );
    }

    /// A prompt that did not settle (stopped, cancelled, failed, or the
    /// session taken) still ends the review's turn, so Revert is not stuck
    /// on "turn n is still running".
    #[gpui::test]
    async fn a_stopped_prompt_ends_the_reviews_turn(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "ALPHA\nbeta\ngamma\n").await;
        tool_end(&f, cx, "c1");
        f.window
            .update(cx, |panel, window, cx| {
                panel.turn = Turn::Stopping;
                panel.on_link_event(LinkEvent::PromptStopped("pipe".into()), window, cx);
                assert!(!panel.review.turn_open(), "the turn ended with the prompt");
                panel.revert_turn(1, cx);
                assert_eq!(
                    panel.notice().map(str::to_string).as_deref(),
                    Some(
                        "turn 1 reverted: 1 hunk(s) put back, 0 STALE kept, \
                         0 changed again by a later turn, kept, 0 accepted, kept"
                    )
                );
            })
            .unwrap();
        assert_eq!(buffer.read_with(cx, |b, _| b.text()), ORIGINAL);
        let (f, _buffer) = fixture(cx).await;
        f.window
            .update(cx, |panel, window, cx| {
                panel.turn = Turn::Streaming;
                let taken = LinkEvent::Taken {
                    session_id: "s".into(),
                    reason: "another driver".into(),
                };
                panel.on_link_event(taken, window, cx);
                assert!(!panel.review.turn_open(), "a taken session ends the turn");
            })
            .unwrap();
    }

    /// `(path, key)` of the first hunk of the first reviewed file.
    fn first_hunk(f: &Fixture, cx: &mut TestAppContext) -> (PathBuf, HunkKey) {
        f.window
            .update(cx, |panel, _, cx| {
                let file = &panel.review.files()[0];
                (panel.review.path(file, cx), file.hunks()[0].key.clone())
            })
            .unwrap()
    }

    fn notice(f: &Fixture, cx: &mut TestAppContext) -> Option<String> {
        f.window
            .update(cx, |panel, _, _| panel.notice().map(str::to_string))
            .unwrap()
    }

    fn state_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("cedian-panel-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// ADR-0032: each correction the person makes in Review Changes is one
    /// row in the ledger, however often the review rebuilds.
    #[gpui::test]
    async fn review_corrections_are_ledger_rows(cx: &mut TestAppContext) {
        use cedian_shell::corrections::{self, CorrectionKind};
        let (f, buffer) = fixture(cx).await;
        let dir = state_dir("corrections");
        f.window
            .update(cx, |panel, _, _| panel.set_state_dir(dir.clone()))
            .unwrap();
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "ALPHA\nbeta\nGAMMA\n").await;
        tool_end(&f, cx, "c1");
        let mut vcx = VisualTestContext::from_window(f.window.into(), cx);
        click(&mut vcx, "cedian-review-toggle");
        click(&mut vcx, "cedian-reject-ws/notes.txt-0");
        buffer.update(&mut vcx, |b, cx| {
            let at = b.text().find("GAMMA").unwrap();
            b.edit([(at..at + 5, "Gamma by hand")], None, cx);
        });
        for _ in 0..3 {
            f.window
                .update(&mut vcx, |panel, _, cx| panel.toggle_review(cx))
                .unwrap();
            vcx.run_until_parked();
        }
        settled(&f, &mut vcx);
        f.window
            .update(&mut vcx, |panel, _, cx| panel.revert_turn(1, cx))
            .unwrap();
        let rows = corrections::load(&dir).unwrap();
        let kinds: Vec<CorrectionKind> = rows.iter().map(|r| r.kind).collect();
        assert_eq!(
            kinds,
            vec![
                CorrectionKind::HunkRejected,
                CorrectionKind::UserEditedAgentHunk,
                CorrectionKind::TurnReverted
            ]
        );
        let rejected = &rows[0];
        assert_eq!(rejected.path.as_deref(), Some("ws/notes.txt"));
        assert_eq!(rejected.tool_call_id.as_deref(), Some("c1"));
        assert_eq!(rejected.turn, Some(1));
        assert!(rejected.hunk_key.is_some() && rejected.excerpt_hash.is_some());
        assert_eq!(rows[1].path.as_deref(), Some("ws/notes.txt"));
        assert_eq!(rows[2].turn, Some(1));
        assert!(rows.iter().all(|r| r.task == "panel"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// While a later call is writing the file, a keystroke inside a Pending
    /// hunk is not yet classified, so Reject must refuse rather than write
    /// over it (ADR-0006). Once the call ends the hunk is STALE and Reject
    /// is refused as such.
    #[gpui::test]
    async fn reject_is_refused_while_the_file_is_importing(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
        tool_end(&f, cx, "c1");
        assert_eq!(hunks(&f, cx)[0].1, HunkStatus::Pending);
        tool_start(&f, cx, "c2", &["notes.txt"]);
        buffer.update(cx, |b, cx| b.edit([(7..7, "!")], None, cx));
        cx.run_until_parked();
        // The keystroke is not classified yet, so the view still shows the
        // hunk as Pending with its Reject button: that click is the one
        // that must be refused.
        assert_eq!(hunks(&f, cx)[0].1, HunkStatus::Pending);
        let (path, key) = first_hunk(&f, cx);
        f.window
            .update(cx, |panel, _, cx| panel.reject_hunk(&path, &key, cx))
            .unwrap();
        assert_eq!(
            notice(&f, cx).as_deref(),
            Some("OMP is writing ws/notes.txt; try again when the call ends")
        );
        assert_eq!(
            buffer.read_with(cx, |b, _| b.text()),
            "alpha\nB!ETA\ngamma\n",
            "the keystroke is kept"
        );
        let refused = Some("OMP is writing ws/notes.txt; try again when the call ends");
        f.window
            .update(cx, |panel, _, cx| {
                panel.notice = None;
                panel.accept_all(cx);
                assert_eq!(panel.notice(), refused, "Accept all");
                panel.notice = None;
                panel.review.end_turn();
                panel.revert_turn(1, cx);
                assert_eq!(panel.notice(), refused, "Revert turn");
            })
            .unwrap();
        assert_eq!(
            hunks(&f, cx)[0].1,
            HunkStatus::Pending,
            "nothing was accepted"
        );
        tool_end(&f, cx, "c2");
        assert_eq!(hunks(&f, cx)[0].1, HunkStatus::Stale);
        let (path, key) = first_hunk(&f, cx);
        f.window
            .update(cx, |panel, _, cx| panel.reject_hunk(&path, &key, cx))
            .unwrap();
        assert!(
            notice(&f, cx).is_some_and(|n| n.contains("hunk is STALE")),
            "{:?}",
            notice(&f, cx)
        );
        assert_eq!(
            buffer.read_with(cx, |b, _| b.text()),
            "alpha\nB!ETA\ngamma\n"
        );
    }

    /// The session is taken while a call is in flight: its ToolEnd never
    /// comes, so the call is released and the file is the person's again.
    #[gpui::test]
    async fn a_taken_session_releases_the_calls_in_flight(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
        tool_end(&f, cx, "c1");
        tool_start(&f, cx, "c2", &["notes.txt"]);
        f.window
            .update(cx, |panel, window, cx| {
                panel.turn = Turn::Streaming;
                let taken = LinkEvent::Taken {
                    session_id: "s".into(),
                    reason: "another driver".into(),
                };
                panel.on_link_event(taken, window, cx);
                assert!(
                    panel.review.files().iter().all(|f| !f.importing()),
                    "no file is importing"
                );
            })
            .unwrap();
        buffer.update(cx, |b, cx| b.edit([(7..7, "!")], None, cx));
        cx.run_until_parked();
        assert_eq!(hunks(&f, cx)[0].1, HunkStatus::Stale);
    }

    /// Two calls write the same file at once and the short one ends first:
    /// the file is still importing for the long one, so its write is not
    /// a user edit when it lands.
    #[gpui::test]
    async fn overlapping_calls_keep_the_file_importing_until_the_last_ends(
        cx: &mut TestAppContext,
    ) {
        let (f, _buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
        tool_end(&f, cx, "c1");
        tool_start(&f, cx, "c2", &["notes.txt"]);
        tool_start(&f, cx, "c3", &["notes.txt"]);
        tool_end(&f, cx, "c3");
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\nGAMMA\n").await;
        tool_end(&f, cx, "c2");
        assert_eq!(
            hunks(&f, cx),
            vec![
                (
                    "ws/notes.txt".to_string(),
                    HunkStatus::Pending,
                    vec!["c1".to_string()]
                ),
                (
                    "ws/notes.txt".to_string(),
                    HunkStatus::Pending,
                    vec!["c2".to_string()]
                ),
            ]
        );
    }

    /// OMP's process exits mid-turn: the turn is over, so its revert is not
    /// refused as still running.
    #[gpui::test]
    async fn a_disconnect_mid_turn_ends_the_reviews_turn(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
        tool_end(&f, cx, "c1");
        f.window
            .update(cx, |panel, window, cx| {
                panel.turn = Turn::Streaming;
                panel.on_link_event(LinkEvent::Event(RouterEvent::Disconnected), window, cx);
                assert!(!panel.review.turn_open(), "the turn ended with the process");
                panel.revert_turn(1, cx);
                assert_eq!(
                    panel.notice().map(str::to_string).as_deref(),
                    Some(
                        "turn 1 reverted: 1 hunk(s) put back, 0 STALE kept, \
                         0 changed again by a later turn, kept, 0 accepted, kept"
                    )
                );
            })
            .unwrap();
        assert_eq!(buffer.read_with(cx, |b, _| b.text()), ORIGINAL);
    }

    /// Typing in an open file OMP did not write is no outcome: the file is
    /// not in review at all, STALE or otherwise.
    #[gpui::test]
    async fn an_unrelated_dirty_file_is_not_in_review(cx: &mut TestAppContext) {
        let (f, _notes) = fixture(cx).await;
        let other = f
            .project
            .update(cx, |p, cx| p.open_local_buffer("/ws/other.txt", cx))
            .await
            .unwrap();
        tool_start(&f, cx, "c1", &["notes.txt"]);
        other.update(cx, |b, cx| b.edit([(0..0, "USER ")], None, cx));
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
        tool_end(&f, cx, "c1");
        let files: Vec<(String, Option<String>)> = f
            .window
            .update(cx, |panel, _, cx| {
                panel
                    .review
                    .files()
                    .iter()
                    .map(|file| {
                        (
                            panel.review.path(file, cx).display().to_string(),
                            file.stale_import().map(str::to_string),
                        )
                    })
                    .collect()
            })
            .unwrap();
        assert_eq!(files, vec![("ws/notes.txt".to_string(), None)]);
        assert_eq!(hunks(&f, cx)[0].1, HunkStatus::Pending);
    }

    /// The person saves their own edit to an open file while a call writes
    /// another: a file the call does not name is never an outcome, so their
    /// save is not credited to the agent and the other file's hunk stands.
    #[gpui::test]
    async fn a_users_save_of_an_unnamed_file_is_not_imported(cx: &mut TestAppContext) {
        let (f, _notes) = fixture(cx).await;
        let other = f
            .project
            .update(cx, |p, cx| p.open_local_buffer("/ws/other.txt", cx))
            .await
            .unwrap();
        tool_start(&f, cx, "c1", &["notes.txt"]);
        other.update(cx, |b, cx| b.edit([(0..0, "USER ")], None, cx));
        f.project
            .update(cx, |p, cx| p.save_buffer(other.clone(), cx))
            .await
            .unwrap();
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
        tool_end(&f, cx, "c1");
        assert_eq!(
            hunks(&f, cx),
            vec![(
                "ws/notes.txt".to_string(),
                HunkStatus::Pending,
                vec!["c1".to_string()]
            )]
        );
        assert_eq!(other.read_with(cx, |b, _| b.text()), "USER one\ntwo\n");
    }

    /// A file already dirty before the call, named by the call but left as
    /// it was on disk: the disk did not change, so it is no outcome. It is
    /// not listed STALE and the person's text is kept.
    #[gpui::test]
    async fn a_named_file_dirty_before_the_call_whose_disk_is_unchanged_is_no_outcome(
        cx: &mut TestAppContext,
    ) {
        let (f, _notes) = fixture(cx).await;
        let other = f
            .project
            .update(cx, |p, cx| p.open_local_buffer("/ws/other.txt", cx))
            .await
            .unwrap();
        other.update(cx, |b, cx| b.edit([(0..0, "USER ")], None, cx));
        tool_start(&f, cx, "c1", &["notes.txt", "other.txt"]);
        cx.run_until_parked();
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
        tool_end(&f, cx, "c1");
        let files: Vec<(String, Option<String>)> = f
            .window
            .update(cx, |panel, _, cx| {
                panel
                    .review
                    .files()
                    .iter()
                    .map(|file| {
                        (
                            panel.review.path(file, cx).display().to_string(),
                            file.stale_import().map(str::to_string),
                        )
                    })
                    .collect()
            })
            .unwrap();
        assert_eq!(files, vec![("ws/notes.txt".to_string(), None)]);
        other.read_with(cx, |b, _| {
            assert_eq!(b.text(), "USER one\ntwo\n");
            assert!(b.is_dirty());
        });
    }

    /// Two calls start on a file with no review yet and the short one ends
    /// first: its outcome creates the file's review, which must know the
    /// long call is still writing it, so Reject is refused until that one
    /// ends.
    #[gpui::test]
    async fn a_review_created_mid_call_knows_the_call_still_writing_it(cx: &mut TestAppContext) {
        let (f, buffer) = fixture(cx).await;
        tool_start(&f, cx, "c1", &["notes.txt"]);
        tool_start(&f, cx, "c2", &["notes.txt"]);
        omp_writes(&f, "/ws/notes.txt", "alpha\nBETA\ngamma\n").await;
        tool_end(&f, cx, "c2");
        assert_eq!(hunks(&f, cx)[0].1, HunkStatus::Pending);
        let (path, key) = first_hunk(&f, cx);
        f.window
            .update(cx, |panel, _, cx| panel.reject_hunk(&path, &key, cx))
            .unwrap();
        assert_eq!(
            notice(&f, cx).as_deref(),
            Some("OMP is writing ws/notes.txt; try again when the call ends")
        );
        assert_eq!(
            buffer.read_with(cx, |b, _| b.text()),
            "alpha\nBETA\ngamma\n",
            "nothing was put back"
        );
        tool_end(&f, cx, "c1");
        f.window
            .update(cx, |panel, _, cx| {
                panel.notice = None;
                panel.reject_hunk(&path, &key, cx);
                assert_eq!(panel.notice(), None, "the reject went through");
            })
            .unwrap();
        assert_eq!(buffer.read_with(cx, |b, _| b.text()), ORIGINAL);
    }

    #[gpui::test]
    #[ignore]
    async fn live_panel_streams_and_imports_one_undoable_omp_edit(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        init(cx);
        let dir = std::env::temp_dir().join(format!("cedian-s9a-live-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("notes.txt"), "alpha\nbeta\ngamma\n").unwrap();
        let dir = dir.canonicalize().unwrap();

        let fs = fs::RealFs::new(None, cx.executor());
        let project = Project::test(fs, [dir.as_path()], cx).await;
        let buffer = project
            .update(cx, |p, cx| p.open_local_buffer(dir.join("notes.txt"), cx))
            .await
            .unwrap();
        let window = cx.add_window(|window, cx| CedianPanel::new(project.clone(), window, cx));

        let send = |cx: &mut TestAppContext, text: &'static str| {
            window
                .update(cx, |panel, window, cx| {
                    panel
                        .input
                        .update(cx, |editor, cx| editor.set_text(text, window, cx));
                    panel.send(window, cx);
                    assert_eq!(panel.turn, Turn::Queued, "{:?}", panel.notice);
                })
                .unwrap();
        };
        let read = |cx: &mut TestAppContext| {
            window
                .update(cx, |panel, _, _| {
                    (
                        panel.turn.clone(),
                        texts(panel),
                        panel
                            .review
                            .files()
                            .iter()
                            .map(|f| f.agent_txns().len())
                            .sum::<usize>(),
                        panel
                            .review
                            .files()
                            .iter()
                            .filter(|f| f.stale_import().is_some())
                            .count(),
                    )
                })
                .unwrap()
        };

        // T2: a prompt typed into the panel streams a reply into the thread.
        send(
            cx,
            "Reply with exactly this word and nothing else: hello-cedian",
        );
        wait_until(cx, "streamed reply", |cx| {
            let (status, text, _, _) = read(cx);
            status == Turn::Idle && text.contains("hello-cedian")
        });
        eprintln!("--- after T2 ---\n{}", read(cx).1);

        // T5: OMP's own edit tool writes disk → one imported transaction.
        send(
            cx,
            "Use your edit tool (not bash, not a host tool) to change the line `beta` to `BETA` \
             in notes.txt in the current workspace. Then reply with only: done",
        );
        wait_until(cx, "imported agent edit", |cx| {
            let (status, _, imported, _) = read(cx);
            status == Turn::Idle && imported >= 1
        });
        let (_, text, imported, stale) = read(cx);
        eprintln!("--- after T5 ---\n{text}\nimported={imported} stale={stale}");
        assert_eq!(imported, 1, "exactly one agent transaction");
        assert_eq!(stale, 0);
        let call_id = window
            .update(cx, |panel, _, _| {
                panel.review.files()[0].agent_txns()[0].tool_call_id.clone()
            })
            .unwrap();
        assert!(!call_id.is_empty(), "keyed by tool_call_id");

        buffer.update(cx, |b, cx| {
            assert_eq!(b.text(), "alpha\nBETA\ngamma\n", "buffer shows OMP's write");
            assert!(!b.is_dirty());
            b.undo(cx);
            assert_eq!(
                b.text(),
                "alpha\nbeta\ngamma\n",
                "ONE native undo reverts it"
            );
        });
        eprintln!("tool_call_id={call_id}: OMP edit imported and reverted by one undo");
    }
}

/// A tool card, or a subagent shown under the card that started it.
enum Entry<'a> {
    Card(ToolCard),
    Subagent(&'a SubagentRow),
}

fn card_line(card: &ToolCard) -> String {
    format!("[{:?}] {}", card.status, card.display_line())
}

fn subagent_line(row: &SubagentRow) -> String {
    format!("↳ [{:?}] {} — {}", row.status, row.agent, row.description)
}
