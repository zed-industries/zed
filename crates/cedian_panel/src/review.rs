//! Review Changes over Zed buffers (cedian ARCHITECTURE §15–§18, ADR-0006,
//! S9 U5): one [`TaskReview`] per task, one [`FileReview`] per buffer the
//! task's agent wrote.
//!
//! The baseline of a file is the buffer's `clock::Global` and text just
//! before the task's first agent edit to it (ADR-0006: per task, never git
//! HEAD). Hunks are the buffer's own `edits_since(baseline)`, widened to
//! whole lines, so they are the CRDT's truth, not a second diff engine. A
//! hunk is `Stale` when it intersects a user edit made after the agent last
//! touched the buffer; the user's edits are kept as anchors so a later agent
//! or cedian edit never demotes a stale hunk back to pending.
//!
//! Accept marks. Reject is one `Buffer::edit` putting the baseline lines
//! back, in its own finalized transaction, so native undo restores the
//! agent's text. Reject refuses a stale hunk and a buffer that moved since
//! the hunks were built. Accept all skips stale hunks (decision 7).

use cedian_review::{HunkKey, HunkStatus, StatusRecord};
use cedian_shell::corrections::{CorrectionKind, Event as Correction};
use cedian_worker::WorkerScope;
use collections::HashMap;
use gpui::{App, Entity};
use language::Buffer;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use text::{Anchor, ToOffset, TransactionId};

/// Review failures (caller-visible).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewError {
    NoFile {
        path: PathBuf,
    },
    /// The hunk the user acted on is no longer there, or not as they saw it.
    Moved {
        path: PathBuf,
    },
    /// Resolving from this status is not allowed (a stale hunk is never
    /// rejected; an interrupted one is never resolved).
    BadTransition {
        status: HunkStatus,
    },
    /// The buffer changed since the hunks were built: rebuild and look again.
    Outdated {
        path: PathBuf,
    },
    NoTurn {
        turn: u32,
    },
    /// The turn is still streaming; revert it once it settles.
    TurnInFlight {
        turn: u32,
    },
    /// A tool call is writing the file: edits made now are not classified
    /// yet, so nothing may act on its hunks until the call ends.
    Importing {
        path: PathBuf,
    },
}

impl std::fmt::Display for ReviewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFile { path } => write!(f, "{} is not under review", path.display()),
            Self::Moved { path } => {
                write!(f, "the change in {} moved; review again", path.display())
            }
            Self::BadTransition {
                status: HunkStatus::Stale,
            } => write!(
                f,
                "hunk is STALE (changed after the agent edit): compare and restore manually"
            ),
            Self::BadTransition { status } => {
                write!(f, "hunk transition not allowed from {status:?}")
            }
            Self::Outdated { path } => write!(
                f,
                "{} changed since the review was built; look again",
                path.display()
            ),
            Self::NoTurn { turn } => write!(f, "no turn {turn}"),
            Self::TurnInFlight { turn } => {
                write!(f, "turn {turn} is still running; revert it once it settles")
            }
            Self::Importing { path } => write!(
                f,
                "OMP is writing {}; try again when the call ends",
                path.display()
            ),
        }
    }
}

impl std::error::Error for ReviewError {}

/// Something the owner records outside the model (a corrections row).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewEvent {
    HunkRejected {
        path: PathBuf,
        key: HunkKey,
        turn: u32,
        tool_call_id: String,
    },
    /// Emitted once per hunk identity, however often the review rebuilds.
    UserEditedAgentHunk {
        path: PathBuf,
        key: HunkKey,
        turn: u32,
    },
    TurnReverted {
        turn: u32,
        reverted: usize,
        /// Each STALE change kept, as `<path> line <n>`.
        stale: Vec<String>,
        /// Each OutOfScope change kept, as `<path> line <n>` — a person's
        /// decision, so a revert never puts it back (ADR-0033 decision 3).
        out_of_scope: Vec<String>,
        /// Changes a later turn wrote over; they stay (revert that turn first).
        overwritten: usize,
        /// Changes the person accepted; they stay.
        accepted: usize,
    },
}

impl std::fmt::Display for ReviewEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TurnReverted {
                turn,
                reverted: 0,
                stale,
                out_of_scope,
                overwritten: 0,
                accepted: 0,
            } if stale.is_empty() && out_of_scope.is_empty() => {
                write!(f, "turn {turn}: nothing to do")
            }
            Self::TurnReverted {
                turn,
                reverted,
                stale,
                out_of_scope,
                overwritten,
                accepted,
            } => {
                write!(
                    f,
                    "turn {turn} reverted: {reverted} hunk(s) put back, {} STALE kept, \
                     {overwritten} changed again by a later turn, kept, {accepted} accepted, kept",
                    stale.len()
                )?;
                if !stale.is_empty() {
                    write!(
                        f,
                        "; kept as STALE, edited after the agent wrote it (an undo then \
                         redo counts as an edit): {}",
                        stale.join(", ")
                    )?;
                }
                if !out_of_scope.is_empty() {
                    write!(
                        f,
                        "; kept out-of-scope, outside the running workers' briefs (needs a \
                         decision): {}",
                        out_of_scope.join(", ")
                    )?;
                }
                Ok(())
            }
            other => write!(f, "{other:?}"),
        }
    }
}

impl ReviewEvent {
    /// The ADR-0032 ledger row for this event. A hunk's key is its baseline
    /// range; its agent text is the excerpt.
    pub fn correction(&self) -> (CorrectionKind, Correction) {
        let hunk = |path: &Path, key: &HunkKey, turn: u32| Correction {
            turn: Some(turn),
            path: Some(path.display().to_string()),
            hunk_key: Some(format!("{}+{}", key.before_start, key.before_count)),
            excerpt: Some(key.after_text.clone()),
            ..Correction::default()
        };
        match self {
            Self::HunkRejected {
                path,
                key,
                turn,
                tool_call_id,
            } => (
                CorrectionKind::HunkRejected,
                Correction {
                    tool_call_id: Some(tool_call_id.clone()).filter(|id| !id.is_empty()),
                    ..hunk(path, key, *turn)
                },
            ),
            Self::UserEditedAgentHunk { path, key, turn } => {
                (CorrectionKind::UserEditedAgentHunk, hunk(path, key, *turn))
            }
            Self::TurnReverted { turn, .. } => (
                CorrectionKind::TurnReverted,
                Correction {
                    turn: Some(*turn),
                    ..Correction::default()
                },
            ),
        }
    }
}

/// One agent transaction the task made to a buffer.
#[derive(Debug, Clone)]
pub struct AgentTxn {
    pub tool_call_id: String,
    pub turn: u32,
    pub transaction: TransactionId,
    /// For a revert's restore transaction: the part of it that put this
    /// call's text back (the whole transaction is cedian's, so its own
    /// edited ranges would attribute nothing).
    pub restored: Option<Range<Anchor>>,
    /// The models that had answered in OMP when the call was imported
    /// (ADR-0039: none of them may review it); empty when unknown.
    pub models: Vec<String>,
}

/// One reviewable change: `old` in baseline offsets, `new` in current ones,
/// both whole lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewHunk {
    pub old: Range<usize>,
    pub new: Range<usize>,
    /// 0-based rows in the current buffer.
    pub rows: Range<u32>,
    pub old_text: String,
    pub new_text: String,
    pub status: HunkStatus,
    /// The agent calls whose edits this hunk covers, oldest first.
    pub tool_call_ids: Vec<String>,
    /// The hunk's identity across rebuilds (baseline rows + text after).
    pub key: HunkKey,
}

fn key_of(baseline: &text::BufferSnapshot, old: &Range<usize>, new_text: &str) -> HunkKey {
    let start = baseline.offset_to_point(old.start).row as usize;
    let end = baseline.offset_to_point(old.end).row as usize;
    HunkKey {
        before_start: start,
        before_count: end.saturating_sub(start),
        after_text: new_text.trim_end_matches('\n').to_string(),
    }
}

/// What a turn did, so it can be reverted (§18 "Revert turn").
#[derive(Debug, Clone)]
pub enum TurnKind {
    Prompt,
    /// An inline edit from the editor (ADR-0056), named by its label.
    Edit {
        label: String,
    },
    /// The revert of turn `of`: one transaction per buffer it touched.
    Revert {
        of: u32,
        transactions: Vec<(Entity<Buffer>, TransactionId)>,
    },
}

#[derive(Default)]
struct TurnRevert {
    edits: Vec<(Range<usize>, String)>,
    /// The first line (1-based) of each STALE change kept.
    stale: Vec<u32>,
    /// The first line (1-based) of each OutOfScope change kept (ADR-0033
    /// decision 3): a person decides it, never an automatic revert.
    out_of_scope: Vec<u32>,
    overwritten: usize,
    accepted: usize,
}

/// A range the user edited, kept as anchors so a later agent or cedian edit
/// never demotes a stale hunk back to pending.
struct UserEdit {
    range: Range<Anchor>,
    /// The buffer version the fold saw, so an import can tell a fold made
    /// after its call's mark from an older one.
    folded_at: clock::Global,
}

/// Per-buffer review state.
pub struct FileReview {
    buffer: Entity<Buffer>,
    baseline: text::BufferSnapshot,
    agent_txns: Vec<AgentTxn>,
    /// Every edit up to here is accounted for: the agent's, cedian's, or
    /// folded into `user_edits`.
    last_seen: clock::Global,
    user_edits: Vec<UserEdit>,
    /// The tool calls writing this file (each between
    /// [`TaskReview::observe`] and its outcome): edits landing now are an
    /// import, or are judged by one, so a display rebuild classifies none
    /// of them.
    importing: collections::HashSet<String>,
    /// The `bash` calls among `importing`. A command names no file, so
    /// it marks every open buffer; it does not refuse review actions: one
    /// that lands after an Accept or Reject meets a buffer the review just
    /// changed and imports as STALE, never over it (ADR-0006).
    bash_calls: collections::HashSet<String>,
    /// Set when OMP wrote the disk while the buffer had unsaved edits, so
    /// nothing was imported (decision 5).
    stale_import: Option<String>,
    /// Accepted hunks, each with the agent transactions the accept covered;
    /// a new agent transaction producing the same hunk is a new change to
    /// review. `None` for a resolution restored from the persisted records.
    /// A rejected hunk has no resolution: its text is back to the baseline,
    /// and if it reappears (undo of the reject, an agent retry) it is a
    /// live hunk again.
    resolved: HashMap<HunkKey, Option<Vec<TransactionId>>>,
    /// Hunks a reject put back. One that reappears with the same text is
    /// the agent's text returning (an undo of the reject), not a user edit.
    rejected: collections::HashSet<HunkKey>,
    reported_stale: collections::HashSet<HunkKey>,
    hunks: Vec<ReviewHunk>,
    built_at: clock::Global,
    /// The running workers' briefs this file was imported under (ADR-0063
    /// decision 1); `None` means no running worker, so no scope is checked.
    scope: Option<Arc<WorkerScope>>,
    /// The buffer at each turn's first agent edit to it, so a turn reverts
    /// to its own start, not to the task baseline.
    turn_starts: Vec<(u32, text::BufferSnapshot)>,
}

impl FileReview {
    fn new(buffer: Entity<Buffer>, baseline: text::BufferSnapshot) -> Self {
        Self {
            buffer,
            last_seen: baseline.version().clone(),
            baseline,
            agent_txns: Vec::new(),
            user_edits: Vec::new(),
            importing: collections::HashSet::default(),
            bash_calls: collections::HashSet::default(),
            stale_import: None,
            resolved: HashMap::default(),
            rejected: collections::HashSet::default(),
            reported_stale: collections::HashSet::default(),
            hunks: Vec::new(),
            built_at: clock::Global::new(),
            scope: None,
            turn_starts: Vec::new(),
        }
    }

    pub fn hunks(&self) -> &[ReviewHunk] {
        &self.hunks
    }

    pub fn buffer(&self) -> &Entity<Buffer> {
        &self.buffer
    }

    /// Why the whole file is STALE, when an import was refused.
    pub fn stale_import(&self) -> Option<&str> {
        self.stale_import.as_deref()
    }

    pub fn agent_txns(&self) -> &[AgentTxn] {
        &self.agent_txns
    }

    /// A tool call is writing this file.
    pub fn importing(&self) -> bool {
        !self.importing.is_empty()
    }

    /// A call that names this file is writing it: review actions wait.
    fn writing(&self) -> bool {
        self.importing
            .iter()
            .any(|id| !self.bash_calls.contains(id))
    }

    fn fold_user_edits(&mut self, buffer: &Buffer) {
        self.fold_user_edits_except(buffer, &[]);
    }

    /// Every edit since `last_seen` is the user's, except one lying inside
    /// a range in `agent` (an imported transaction's own edits). An edit
    /// only partly inside stays the user's: unsure means STALE.
    fn fold_user_edits_except(&mut self, buffer: &Buffer, agent: &[Range<usize>]) {
        let snapshot = buffer.text_snapshot();
        for (edit, range) in snapshot.anchored_edits_since::<usize>(&self.last_seen) {
            let inside = agent
                .iter()
                .any(|a| a.start <= edit.new.start && edit.new.end <= a.end);
            if !inside {
                self.user_edits.push(UserEdit {
                    range,
                    folded_at: snapshot.version().clone(),
                });
            }
        }
        self.last_seen = snapshot.version().clone();
    }

    fn rebuild(&mut self, buffer: &Buffer) -> Vec<HunkKey> {
        let snapshot = buffer.text_snapshot();
        // Every hunk of a file outside the running workers' briefs is
        // OutOfScope (ADR-0033 decision 3, ADR-0063 decision 1).
        let out_of_scope = match (&self.scope, scope_path(buffer)) {
            (Some(scope), Some(path)) => !scope.covers(&path),
            _ => false,
        };
        let edits: Vec<text::Edit<usize>> = snapshot
            .edits_since::<usize>(self.baseline.version())
            .collect();
        let user: Vec<Range<usize>> = self
            .user_edits
            .iter()
            .map(|e| e.range.start.to_offset(&snapshot)..e.range.end.to_offset(&snapshot))
            .collect();
        // A call's edits are attributed by row: a pure deletion is an empty
        // range that sits at a line start, which must not credit the call
        // with the line above it.
        let agent: Vec<(&AgentTxn, Vec<Range<u32>>)> = self
            .agent_txns
            .iter()
            .map(|t| {
                let ranges: Vec<Range<usize>> = match &t.restored {
                    Some(r) => vec![r.start.to_offset(&snapshot)..r.end.to_offset(&snapshot)],
                    None => buffer
                        .edited_ranges_for_transaction_id::<usize>(t.transaction)
                        .collect(),
                };
                (t, ranges.iter().map(|r| rows_of(&snapshot, r)).collect())
            })
            .collect();
        let mut newly_stale = Vec::new();
        // The CRDT reports a rejected region as an edit too (its lines were
        // deleted and put back), so equal text is no hunk.
        self.hunks = line_hunks(&self.baseline, &snapshot, &edits)
            .into_iter()
            .filter_map(|(old, new)| {
                let old_text: String = self.baseline.text_for_range(old.clone()).collect();
                let new_text: String = snapshot.text_for_range(new.clone()).collect();
                (old_text != new_text).then_some((old, new, old_text, new_text))
            })
            .map(|(old, new, old_text, new_text)| {
                let rows = snapshot.offset_to_point(new.start).row
                    ..snapshot
                        .offset_to_point(new.end)
                        .row
                        .max(snapshot.offset_to_point(new.start).row + 1);
                let key = key_of(&self.baseline, &old, &new_text);
                let calls: Vec<&AgentTxn> = agent
                    .iter()
                    .filter(|(_, call_rows)| {
                        call_rows
                            .iter()
                            .any(|r| r.start < rows.end && rows.start < r.end)
                    })
                    .map(|(t, _)| *t)
                    .collect();
                let accepted = match self.resolved.get(&key) {
                    Some(None) => true,
                    Some(Some(txns)) => calls.iter().all(|t| txns.contains(&t.transaction)),
                    None => false,
                };
                if !accepted {
                    self.resolved.remove(&key);
                }
                let status = if accepted {
                    HunkStatus::Accepted
                } else if out_of_scope {
                    HunkStatus::OutOfScope
                } else if user.iter().any(|r| touches(&new, r)) {
                    HunkStatus::Stale
                } else if calls.is_empty() {
                    HunkStatus::Unattributed
                } else {
                    HunkStatus::Pending
                };
                if status == HunkStatus::Stale && self.reported_stale.insert(key.clone()) {
                    newly_stale.push(key.clone());
                }
                ReviewHunk {
                    old,
                    new,
                    rows,
                    old_text,
                    new_text,
                    status,
                    tool_call_ids: calls.iter().fold(Vec::new(), |mut ids, t| {
                        if !ids.contains(&t.tool_call_id) {
                            ids.push(t.tool_call_id.clone());
                        }
                        ids
                    }),
                    key,
                }
            })
            .collect();
        let mut revived: Vec<Range<usize>> = Vec::new();
        for hunk in &mut self.hunks {
            if self.rejected.remove(&hunk.key) && hunk.status == HunkStatus::Stale {
                hunk.status = HunkStatus::Pending;
                self.reported_stale.remove(&hunk.key);
                newly_stale.retain(|k| k != &hunk.key);
                revived.push(hunk.new.clone());
            }
        }
        self.user_edits.retain(|e| {
            let r = e.range.start.to_offset(&snapshot)..e.range.end.to_offset(&snapshot);
            !revived.iter().any(|h| touches(h, &r))
        });
        self.built_at = snapshot.version().clone();
        newly_stale
    }

    /// What reverting turn `n` does to this buffer: the turn's changes are
    /// the line hunks between its start snapshot and the next turn's (or
    /// now). Each maps to current offsets through the later edits, unless a
    /// later edit touches it (`overwritten`, or nothing to do when that edit
    /// put the turn's start text back) or a user edit does (`stale`).
    fn turn_revert(&self, n: u32, cx: &App) -> TurnRevert {
        let mut plan = TurnRevert::default();
        let Some(i) = self.turn_starts.iter().position(|(t, _)| *t == n) else {
            return plan;
        };
        let start = &self.turn_starts[i].1;
        let snapshot = self.buffer.read(cx).text_snapshot();
        let end = self
            .turn_starts
            .get(i + 1)
            .map_or_else(|| snapshot.clone(), |(_, s)| s.clone());
        let turn_edits: Vec<text::Edit<usize>> = end.edits_since(start.version()).collect();
        let later_edits: Vec<text::Edit<usize>> = snapshot.edits_since(end.version()).collect();
        // A later change that left the text as it was (a reverted later
        // turn) did not write over anything.
        let later: Vec<(Range<usize>, Range<usize>)> = line_hunks(&end, &snapshot, &later_edits)
            .into_iter()
            .filter(|(old, new)| {
                let before: String = end.text_for_range(old.clone()).collect();
                let after: String = snapshot.text_for_range(new.clone()).collect();
                before != after
            })
            .collect();
        let user: Vec<Range<usize>> = self
            .user_edits
            .iter()
            .map(|e| e.range.start.to_offset(&snapshot)..e.range.end.to_offset(&snapshot))
            .collect();
        let accepted: Vec<Range<usize>> = self
            .hunks
            .iter()
            .filter(|h| h.status == HunkStatus::Accepted)
            .map(|h| h.new.clone())
            .collect();
        // OutOfScope hunks are a person's to decide, so a revert keeps them
        // (ADR-0033 decision 3).
        let out_of_scope: Vec<Range<usize>> = self
            .hunks
            .iter()
            .filter(|h| h.status == HunkStatus::OutOfScope)
            .map(|h| h.new.clone())
            .collect();
        for (old, new) in line_hunks(start, &end, &turn_edits) {
            let old_text: String = start.text_for_range(old.clone()).collect();
            let new_text: String = end.text_for_range(new.clone()).collect();
            if old_text == new_text {
                continue;
            }
            if let Some((later_old, later_new)) = later.iter().find(|(old, _)| touches(&new, old)) {
                let current: String = snapshot.text_for_range(later_new.clone()).collect();
                if *later_old != new || current != old_text {
                    plan.overwritten += 1;
                }
                continue;
            }
            let delta: i64 = later
                .iter()
                .filter(|(old, _)| old.end <= new.start)
                .map(|(old, now)| now.len() as i64 - old.len() as i64)
                .sum();
            let now = (new.start as i64 + delta) as usize..(new.end as i64 + delta) as usize;
            if user.iter().any(|r| touches(&now, r)) {
                plan.stale.push(snapshot.offset_to_point(now.start).row + 1);
                continue;
            }
            if out_of_scope.iter().any(|r| touches(&now, r)) {
                plan.out_of_scope
                    .push(snapshot.offset_to_point(now.start).row + 1);
                continue;
            }
            if accepted.iter().any(|r| touches(&now, r)) {
                plan.accepted += 1;
                continue;
            }
            plan.edits.push((now, old_text));
        }
        plan
    }

    fn hunk_index(&self, key: &HunkKey, path: &std::path::Path) -> Result<usize, ReviewError> {
        self.hunks
            .iter()
            .position(|h| &h.key == key)
            .ok_or_else(|| ReviewError::Moved {
                path: path.to_path_buf(),
            })
    }

    /// The agent transactions behind a hunk's calls.
    fn txns_of(&self, hunk: &ReviewHunk) -> Vec<TransactionId> {
        self.agent_txns
            .iter()
            .filter(|t| hunk.tool_call_ids.contains(&t.tool_call_id))
            .map(|t| t.transaction)
            .collect()
    }
}

/// The rows a range covers; a range ending at a line start stops before it.
fn rows_of(snapshot: &text::BufferSnapshot, range: &Range<usize>) -> Range<u32> {
    let start = snapshot.offset_to_point(range.start).row;
    let end_point = snapshot.offset_to_point(range.end);
    let end = if range.end > range.start && end_point.column == 0 {
        end_point.row
    } else {
        end_point.row + 1
    };
    start..end.max(start + 1)
}

/// Two ranges touch when they overlap, or one is empty and sits inside or on
/// an edge of the other (an insertion next to a hunk makes it stale: unsure
/// means STALE, never an overwrite).
fn touches(a: &Range<usize>, b: &Range<usize>) -> bool {
    if a.is_empty() || b.is_empty() {
        a.start <= b.end && b.start <= a.end
    } else {
        a.start < b.end && b.start < a.end
    }
}

/// Widen edits to whole lines and merge the ones that then overlap; edits on
/// adjacent lines stay separate hunks, so two calls' lines are never one
/// hunk. Each range is widened to its own snapshot's line boundaries: the
/// text outside every edit is the same in both snapshots, so the same
/// line-boundary lies on both sides unless another edit sits in between, and
/// then the two widened edits overlap and merge.
fn line_hunks(
    old: &text::BufferSnapshot,
    new: &text::BufferSnapshot,
    edits: &[text::Edit<usize>],
) -> Vec<(Range<usize>, Range<usize>)> {
    let at_line_start =
        |s: &text::BufferSnapshot, o: usize| o == 0 || s.chars_at(o - 1).next() == Some('\n');
    let line_start = |s: &text::BufferSnapshot, o: usize| {
        s.point_to_offset(text::Point::new(s.offset_to_point(o).row, 0))
    };
    let line_end = |s: &text::BufferSnapshot, o: usize| {
        let row = s.offset_to_point(o).row;
        if row < s.max_point().row {
            s.point_to_offset(text::Point::new(row + 1, 0))
        } else {
            s.len()
        }
    };
    let mut out: Vec<(Range<usize>, Range<usize>)> = Vec::new();
    for edit in edits {
        let (old_start, new_start) =
            if at_line_start(old, edit.old.start) && at_line_start(new, edit.new.start) {
                (edit.old.start, edit.new.start)
            } else {
                (
                    line_start(old, edit.old.start),
                    line_start(new, edit.new.start),
                )
            };
        let (old_end, new_end) =
            if at_line_start(old, edit.old.end) && at_line_start(new, edit.new.end) {
                (edit.old.end, edit.new.end)
            } else {
                (line_end(old, edit.old.end), line_end(new, edit.new.end))
            };
        let old_range = old_start..old_end;
        let new_range = new_start..new_end;
        match out.last_mut() {
            Some((o, n)) if new_range.start < n.end || old_range.start < o.end => {
                n.end = n.end.max(new_range.end);
                o.end = o.end.max(old_range.end);
            }
            _ => out.push((old_range, new_range)),
        }
    }
    out
}

/// One task's review: the files its agent wrote and their hunks.
pub struct TaskReview {
    task_id: String,
    files: Vec<FileReview>,
    turns: Vec<(u32, TurnKind)>,
    /// A prompt turn is streaming: its revert waits for it to settle.
    turn_open: bool,
    /// Files OMP wrote that could not be opened, with the reason.
    unreviewable: Vec<(PathBuf, String)>,
    events: Vec<ReviewEvent>,
}

impl TaskReview {
    pub fn new(task_id: &str) -> Self {
        Self {
            task_id: task_id.to_string(),
            files: Vec::new(),
            turns: Vec::new(),
            turn_open: false,
            unreviewable: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn unreviewable(&self) -> &[(PathBuf, String)] {
        &self.unreviewable
    }

    /// OMP wrote `path` but it could not be opened for review.
    pub fn could_not_review(&mut self, path: PathBuf, reason: String) {
        match self.unreviewable.iter_mut().find(|(p, _)| *p == path) {
            Some(entry) => entry.1 = reason,
            None => self.unreviewable.push((path, reason)),
        }
    }

    pub fn task_id(&self) -> &str {
        &self.task_id
    }

    pub fn files(&self) -> &[FileReview] {
        &self.files
    }

    pub fn is_empty(&self) -> bool {
        self.unreviewable.is_empty()
            && self
                .files
                .iter()
                .all(|f| f.hunks.is_empty() && f.stale_import.is_none())
    }

    /// Events since the last drain, oldest first.
    pub fn drain_events(&mut self) -> Vec<ReviewEvent> {
        std::mem::take(&mut self.events)
    }

    /// A prompt turn begins; returns its number.
    pub fn begin_turn(&mut self) -> u32 {
        let n = self.turns.last().map_or(1, |(n, _)| n + 1);
        self.turns.push((n, TurnKind::Prompt));
        self.turn_open = true;
        n
    }

    /// An inline-edit turn begins; returns its number.
    pub fn begin_edit_turn(&mut self, label: String) -> u32 {
        let n = self.begin_turn();
        self.turns.last_mut().unwrap().1 = TurnKind::Edit { label };
        n
    }

    /// The label of turn `n` when it is an inline edit.
    pub fn turn_label(&self, n: u32) -> Option<&str> {
        self.turns.iter().find_map(|(t, kind)| match kind {
            TurnKind::Edit { label } if *t == n => Some(label.as_str()),
            _ => None,
        })
    }

    /// The prompt turn settled (its agent is done).
    pub fn end_turn(&mut self) {
        self.turn_open = false;
    }

    pub fn turn_open(&self) -> bool {
        self.turn_open
    }

    pub fn current_turn(&self) -> u32 {
        self.turns.last().map_or(0, |(n, _)| *n)
    }

    fn file_index(&self, buffer: &Entity<Buffer>) -> Option<usize> {
        self.files.iter().position(|f| &f.buffer == buffer)
    }

    /// Account for the user's edits to a tracked buffer up to now. Called
    /// before any agent write can land (the tool's start); until the call's
    /// outcome ([`Self::agent_edited`], [`Self::import_refused`] or
    /// [`Self::import_done`]) a rebuild classifies no edit to the buffer.
    pub fn observe(&mut self, buffer: &Entity<Buffer>, tool_call_id: &str, cx: &App) {
        if let Some(i) = self.file_index(buffer) {
            self.files[i].fold_user_edits(buffer.read(cx));
            self.files[i].importing.insert(tool_call_id.to_string());
        }
    }

    /// [`Self::observe`] for a `bash` call.
    pub fn observe_bash(&mut self, buffer: &Entity<Buffer>, tool_call_id: &str, cx: &App) {
        self.observe(buffer, tool_call_id, cx);
        if let Some(i) = self.file_index(buffer) {
            self.files[i].bash_calls.insert(tool_call_id.to_string());
        }
    }

    /// The call wrote the file but its change is no one's to review (two
    /// calls overlapped, or a move): once no call is writing it, a clean
    /// buffer's change since is Zed's reload of that write, not the
    /// person's edit; a dirty buffer was not reloaded, so its edits are.
    pub fn import_unattributed(&mut self, buffer: &Entity<Buffer>, tool_call_id: &str, cx: &App) {
        if let Some(i) = self.file_index(buffer) {
            let file = &mut self.files[i];
            file.importing.remove(tool_call_id);
            if !file.importing() {
                let b = buffer.read(cx);
                if b.is_dirty() {
                    file.fold_user_edits(b);
                } else {
                    file.last_seen = b.version();
                }
            }
            self.rebuild(cx);
        }
    }

    /// The tool call that [`Self::observe`]d this buffer imported nothing
    /// (or failed, or never ended): once no call is writing the file,
    /// whatever changed since is the user's.
    pub fn import_done(&mut self, buffer: &Entity<Buffer>, tool_call_id: &str, cx: &App) {
        if let Some(i) = self.file_index(buffer) {
            self.files[i].importing.remove(tool_call_id);
            if !self.files[i].importing() {
                self.files[i].fold_user_edits(buffer.read(cx));
            }
            self.rebuild(cx);
        }
    }

    /// An imported agent transaction. `baseline` is the buffer as it was when
    /// the tool started; it becomes the file's baseline on the first edit.
    /// `scope` is the running workers' briefs at this import (ADR-0063
    /// decision 1); `None` means no running worker, so nothing is classified.
    pub fn agent_edited(
        &mut self,
        buffer: &Entity<Buffer>,
        tool_call_id: &str,
        transaction: TransactionId,
        baseline: text::BufferSnapshot,
        models: Vec<String>,
        scope: Option<Arc<WorkerScope>>,
        cx: &App,
    ) {
        let turn = self.current_turn();
        let i = match self.file_index(buffer) {
            Some(i) => i,
            None => {
                self.files
                    .push(FileReview::new(buffer.clone(), baseline.clone()));
                self.files.len() - 1
            }
        };
        let file = &mut self.files[i];
        file.scope = scope;
        if file.agent_txns.is_empty() {
            // The file was only ever refused: the task's first edit to it
            // is this one, and edits before it are never hunks.
            file.last_seen = baseline.version().clone();
            file.baseline = baseline.clone();
            file.user_edits.clear();
        }
        let buffer = buffer.read(cx);
        // Edits since the last accounting outside the transaction are the
        // user's (a transaction autosave wrote, under the watcher's reload
        // of OMP's write).
        let agent: Vec<Range<usize>> = buffer
            .edited_ranges_for_transaction_id::<usize>(transaction)
            .collect();
        file.fold_user_edits_except(buffer, &agent);
        // A fold made after the call's mark that lies wholly inside the
        // transaction is this import landing early (the file's window was
        // closed by another outcome), not the user's: a keystroke in the
        // window is unsaved or saved at the import, and either refuses it.
        let snapshot = buffer.text_snapshot();
        file.user_edits.retain(|e| {
            if !e.folded_at.changed_since(baseline.version()) {
                return true;
            }
            let r = e.range.start.to_offset(&snapshot)..e.range.end.to_offset(&snapshot);
            !agent.iter().any(|a| a.start <= r.start && r.end <= a.end)
        });
        file.importing.remove(tool_call_id);
        if !file.turn_starts.iter().any(|(t, _)| *t == turn) {
            file.turn_starts.push((turn, baseline));
        }
        file.agent_txns.push(AgentTxn {
            tool_call_id: tool_call_id.to_string(),
            turn,
            transaction,
            restored: None,
            models,
        });
        file.last_seen = buffer.version();
        file.stale_import = None;
        let path = path_of(buffer, cx);
        self.unreviewable.retain(|(p, _)| *p != path);
        self.rebuild(cx);
    }

    /// The file changed during the call while the person was editing it
    /// (unsaved edits, or a save during the call): nothing was imported and
    /// the file is STALE as a whole.
    pub fn import_refused(&mut self, buffer: &Entity<Buffer>, tool_call_id: &str, cx: &App) {
        let reason = format!(
            "the file changed during call {tool_call_id} while you were editing it \
             (unsaved edits, or a save during the call); nothing was imported"
        );
        let i = match self.file_index(buffer) {
            Some(i) => i,
            None => {
                let snapshot = buffer.read(cx).text_snapshot();
                self.files.push(FileReview::new(buffer.clone(), snapshot));
                self.files.len() - 1
            }
        };
        let file = &mut self.files[i];
        file.importing.remove(tool_call_id);
        file.fold_user_edits(buffer.read(cx));
        file.stale_import = Some(reason);
    }

    /// Recompute every file's hunks from its buffer.
    pub fn rebuild(&mut self, cx: &App) {
        for file in &mut self.files {
            let buffer = file.buffer.read(cx);
            if !file.importing() {
                file.fold_user_edits(buffer);
            }
            let path = path_of(buffer, cx);
            let turn = file.agent_txns.last().map_or(0, |t| t.turn);
            for key in file.rebuild(buffer) {
                self.events.push(ReviewEvent::UserEditedAgentHunk {
                    path: path.clone(),
                    key,
                    turn,
                });
            }
        }
    }

    pub fn path(&self, file: &FileReview, cx: &App) -> PathBuf {
        path_of(file.buffer.read(cx), cx)
    }

    fn file_by_path(&self, path: &std::path::Path, cx: &App) -> Result<usize, ReviewError> {
        self.files
            .iter()
            .position(|f| path_of(f.buffer.read(cx), cx) == path)
            .ok_or_else(|| ReviewError::NoFile {
                path: path.to_path_buf(),
            })
    }

    /// Accept one hunk: mark only, the code is already in the buffer. An
    /// `OutOfScope` hunk is the person's to resolve, like `Stale`.
    pub fn accept(
        &mut self,
        path: &std::path::Path,
        key: &HunkKey,
        cx: &App,
    ) -> Result<(), ReviewError> {
        let i = self.file_by_path(path, cx)?;
        let file = &mut self.files[i];
        let index = file.hunk_index(key, path)?;
        let hunk = &file.hunks[index];
        match hunk.status {
            HunkStatus::Pending
            | HunkStatus::Unattributed
            | HunkStatus::Stale
            | HunkStatus::OutOfScope => {
                let txns = file.txns_of(hunk);
                file.resolved.insert(hunk.key.clone(), Some(txns));
                file.hunks[index].status = HunkStatus::Accepted;
                Ok(())
            }
            HunkStatus::Interrupted => Err(ReviewError::BadTransition {
                status: hunk.status,
            }),
            HunkStatus::Accepted | HunkStatus::Rejected => Ok(()),
        }
    }

    /// Reject one hunk: put the baseline lines back as one finalized
    /// transaction. Refuses `Stale`, `OutOfScope` and `Interrupted`, and a
    /// buffer that moved since the hunks were built.
    pub fn reject(
        &mut self,
        path: &std::path::Path,
        key: &HunkKey,
        cx: &mut App,
    ) -> Result<TransactionId, ReviewError> {
        let i = self.file_by_path(path, cx)?;
        let file = &mut self.files[i];
        if file.writing() {
            return Err(ReviewError::Importing {
                path: path.to_path_buf(),
            });
        }
        let hunk = file.hunks[file.hunk_index(key, path)?].clone();
        if matches!(
            hunk.status,
            HunkStatus::Stale
                | HunkStatus::OutOfScope
                | HunkStatus::Interrupted
                | HunkStatus::Accepted
                | HunkStatus::Rejected
        ) {
            return Err(ReviewError::BadTransition {
                status: hunk.status,
            });
        }
        if file.buffer.read(cx).version() != file.built_at {
            return Err(ReviewError::Outdated {
                path: path.to_path_buf(),
            });
        }
        let txn = restore(
            &file.buffer,
            vec![(hunk.new.clone(), hunk.old_text.clone())],
            cx,
        );
        file.last_seen = file.buffer.read(cx).version();
        file.rejected.insert(hunk.key.clone());
        let turn = file.agent_txns.last().map_or(0, |t| t.turn);
        self.events.push(ReviewEvent::HunkRejected {
            path: path.to_path_buf(),
            key: hunk.key,
            turn,
            tool_call_id: hunk.tool_call_ids.last().cloned().unwrap_or_default(),
        });
        self.rebuild(cx);
        Ok(txn)
    }

    /// How many hunks are STALE (the user edited them after the agent).
    pub fn stale_count(&self) -> usize {
        self.files
            .iter()
            .flat_map(|f| &f.hunks)
            .filter(|h| h.status == HunkStatus::Stale)
            .count()
    }

    /// How many hunks are `OutOfScope` — a changed file outside the running
    /// workers' briefs (ADR-0033 decision 3). Merge-back is refused while any
    /// is unresolved, and they are never auto-accepted.
    pub fn out_of_scope_count(&self) -> usize {
        self.files
            .iter()
            .flat_map(|f| &f.hunks)
            .filter(|h| h.status == HunkStatus::OutOfScope)
            .count()
    }

    /// Accept every `Pending` hunk, and every `Stale` one too when
    /// `include_stale` (the person was asked; owner ruling 2026-10-08).
    /// Skips `Unattributed`, `Interrupted` (§17 R2) and `OutOfScope`
    /// (never auto-resolved, ADR-0033 decision 3). Refused while a
    /// call is writing a file with a hunk it would take.
    pub fn accept_all(
        &mut self,
        include_stale: bool,
        cx: &App,
    ) -> Result<Vec<(PathBuf, usize)>, ReviewError> {
        // Exhaustive on purpose: a new status must decide here, and every
        // status that is not named is skipped (ADR-0033 decision 3).
        let takes = |status: HunkStatus| match status {
            HunkStatus::Pending => true,
            HunkStatus::Stale => include_stale,
            HunkStatus::OutOfScope
            | HunkStatus::Unattributed
            | HunkStatus::Interrupted
            | HunkStatus::Accepted
            | HunkStatus::Rejected => false,
        };
        self.refuse_importing(cx, |file| file.hunks.iter().any(|h| takes(h.status)))?;
        let mut accepted = Vec::new();
        for file in &mut self.files {
            let path = path_of(file.buffer.read(cx), cx);
            for i in 0..file.hunks.len() {
                if takes(file.hunks[i].status) {
                    let txns = file.txns_of(&file.hunks[i]);
                    file.resolved.insert(file.hunks[i].key.clone(), Some(txns));
                    file.hunks[i].status = HunkStatus::Accepted;
                    accepted.push((path.clone(), i));
                }
            }
        }
        Ok(accepted)
    }

    /// `Importing` for the first file a call is writing among those
    /// `touched` selects.
    fn refuse_importing(
        &self,
        cx: &App,
        touched: impl Fn(&FileReview) -> bool,
    ) -> Result<(), ReviewError> {
        match self.files.iter().find(|f| f.writing() && touched(f)) {
            Some(file) => Err(ReviewError::Importing {
                path: path_of(file.buffer.read(cx), cx),
            }),
            None => Ok(()),
        }
    }

    /// Revert turn `n`: put back, for every change the turn made, the text
    /// the buffer had when the turn first touched it, one transaction per
    /// buffer, so one undo per buffer redoes it. A change the user edited
    /// since (STALE) or a later turn wrote over is kept and counted; an
    /// accepted one stays. Reverting a revert undoes its transactions.
    pub fn revert_turn(&mut self, n: u32, cx: &mut App) -> Result<ReviewEvent, ReviewError> {
        if self.turn_open && self.current_turn() == n {
            return Err(ReviewError::TurnInFlight { turn: n });
        }
        self.refuse_importing(cx, |file| file.turn_starts.iter().any(|(t, _)| *t == n))?;
        let kind = self
            .turns
            .iter()
            .find(|(t, _)| *t == n)
            .map(|(_, k)| k.clone())
            .ok_or(ReviewError::NoTurn { turn: n })?;
        let next = self.turns.last().map_or(1, |(n, _)| n + 1);
        let (reverted, stale, out_of_scope, overwritten, accepted, transactions) = match kind {
            TurnKind::Revert { transactions, .. } => {
                let mut undone = 0;
                for (buffer, txn) in &transactions {
                    if buffer.update(cx, |b, cx| b.undo_transaction(*txn, cx)) {
                        undone += 1;
                    }
                    if let Some(i) = self.file_index(buffer) {
                        self.files[i].last_seen = buffer.read(cx).version();
                    }
                }
                self.rebuild(cx);
                (undone, Vec::new(), Vec::new(), 0, 0, Vec::new())
            }
            TurnKind::Prompt | TurnKind::Edit { .. } => {
                self.rebuild(cx);
                let mut reverted = 0;
                let mut stale = Vec::new();
                let mut out_of_scope = Vec::new();
                let mut overwritten = 0;
                let mut accepted = 0;
                let mut transactions = Vec::new();
                for file in &mut self.files {
                    let plan = file.turn_revert(n, cx);
                    let path = review_key(file.buffer.read(cx)).unwrap_or_default();
                    stale.extend(plan.stale.iter().map(|line| format!("{path} line {line}")));
                    out_of_scope.extend(
                        plan.out_of_scope
                            .iter()
                            .map(|line| format!("{path} line {line}")),
                    );
                    overwritten += plan.overwritten;
                    accepted += plan.accepted;
                    if plan.edits.is_empty() {
                        continue;
                    }
                    reverted += plan.edits.len();
                    // The text put back is the earlier turns' work: credit
                    // each restored range to their calls that had the hunk.
                    let earlier: Vec<(Range<usize>, usize, Vec<(String, u32, Vec<String>)>)> = plan
                        .edits
                        .iter()
                        .map(|(range, text)| {
                            let calls = file
                                .hunks
                                .iter()
                                .filter(|h| touches(&h.new, range))
                                .flat_map(|h| {
                                    file.agent_txns
                                        .iter()
                                        .filter(|t| {
                                            t.turn < n && h.tool_call_ids.contains(&t.tool_call_id)
                                        })
                                        .map(|t| (t.tool_call_id.clone(), t.turn, t.models.clone()))
                                })
                                .collect();
                            (range.clone(), text.len(), calls)
                        })
                        .collect();
                    // The revert closes the reverted turn's window: a later
                    // turn's revert must not include this one's restore.
                    let before = file.buffer.read(cx).text_snapshot();
                    file.turn_starts.push((next, before));
                    let txn = restore(&file.buffer, plan.edits, cx);
                    let snapshot = file.buffer.read(cx).text_snapshot();
                    let mut delta: i64 = 0;
                    for (range, len, calls) in earlier {
                        let start = (range.start as i64 + delta) as usize;
                        let restored = snapshot.anchor_at(start, text::Bias::Right)
                            ..snapshot.anchor_at(start + len, text::Bias::Left);
                        delta += len as i64 - range.len() as i64;
                        for (tool_call_id, turn, models) in calls {
                            file.agent_txns.push(AgentTxn {
                                tool_call_id,
                                turn,
                                transaction: txn,
                                restored: Some(restored.clone()),
                                models,
                            });
                        }
                    }
                    file.last_seen = snapshot.version().clone();
                    transactions.push((file.buffer.clone(), txn));
                }
                self.rebuild(cx);
                (
                    reverted,
                    stale,
                    out_of_scope,
                    overwritten,
                    accepted,
                    transactions,
                )
            }
        };
        self.turns.push((
            next,
            TurnKind::Revert {
                of: n,
                transactions,
            },
        ));
        let event = ReviewEvent::TurnReverted {
            turn: n,
            reverted,
            stale,
            out_of_scope,
            overwritten,
            accepted,
        };
        self.events.push(event.clone());
        Ok(event)
    }

    /// The task as a reviewer reads it (ADR-0055): each reviewed file's
    /// hunks over its buffer text now, keyed `/<worktree-relative path>`
    /// as the reviewer names files, with the baseline text.
    pub fn review_task(&self, cx: &App) -> cedian_shell::review_agent::ReviewTask {
        let mut task = cedian_shell::review_agent::ReviewTask::default();
        for file in &self.files {
            let buffer = file.buffer.read(cx);
            let Some(key) = review_key(buffer) else {
                continue;
            };
            task.diffs.push(cedian_review::FileDiff {
                path: key.clone(),
                hunks: file
                    .hunks
                    .iter()
                    .map(|h| cedian_review::Hunk {
                        before_start: h.key.before_start,
                        before_count: h.key.before_count,
                        after_start: h.rows.start as usize,
                        after_count: h.new_text.lines().count(),
                    })
                    .collect(),
                statuses: file.hunks.iter().map(|h| h.status).collect(),
                snapshot: buffer.text(),
            });
            for hunk in file
                .hunks
                .iter()
                .filter(|h| !matches!(h.status, HunkStatus::Accepted | HunkStatus::Rejected))
            {
                let txns: Vec<&AgentTxn> = file
                    .agent_txns
                    .iter()
                    .filter(|t| hunk.tool_call_ids.contains(&t.tool_call_id))
                    .collect();
                if txns.is_empty() || txns.iter().any(|t| t.models.is_empty()) {
                    task.editors_unknown = true;
                }
                task.editors
                    .extend(txns.iter().flat_map(|t| t.models.iter().cloned()));
            }
            task.baseline
                .insert(PathBuf::from(key), file.baseline.text());
        }
        task.editors.sort();
        task.editors.dedup();
        task
    }

    /// The last turn whose agent wrote the file under review key `key`.
    pub fn last_turn_touching(&self, key: &str, cx: &App) -> Option<u32> {
        self.files
            .iter()
            .find(|f| review_key(f.buffer.read(cx)).as_deref() == Some(key))
            .and_then(|f| f.agent_txns.last().map(|t| t.turn))
    }

    /// Resolutions to persist.
    pub fn status_records(&self, cx: &App) -> Vec<StatusRecord> {
        let mut out: Vec<StatusRecord> = self
            .files
            .iter()
            .flat_map(|f| {
                let path = path_of(f.buffer.read(cx), cx);
                f.resolved.keys().map(move |key| StatusRecord {
                    path: path.clone(),
                    key: key.clone(),
                    status: HunkStatus::Accepted,
                })
            })
            .collect();
        out.sort_by(|a, b| {
            (&a.path, a.key.before_start, &a.key.after_text).cmp(&(
                &b.path,
                b.key.before_start,
                &b.key.after_text,
            ))
        });
        out
    }

    /// Restore persisted resolutions (applied on the next rebuild).
    pub fn restore_statuses(&mut self, records: Vec<StatusRecord>, cx: &App) {
        for record in records {
            if record.status != HunkStatus::Accepted {
                continue;
            }
            if let Ok(i) = self.file_by_path(&record.path, cx) {
                self.files[i].resolved.insert(record.key, None);
            }
        }
        self.rebuild(cx);
    }
}

/// Replace `edits` (current offsets → replacement) as one finalized
/// transaction and return its id.
fn restore(
    buffer: &Entity<Buffer>,
    edits: Vec<(Range<usize>, String)>,
    cx: &mut App,
) -> TransactionId {
    buffer.update(cx, |b, cx| {
        b.finalize_last_transaction();
        b.start_transaction();
        b.edit(edits, None, cx);
        let txn = b
            .end_transaction(cx)
            .expect("the edit opened a transaction");
        b.finalize_last_transaction();
        txn
    })
}

/// A reviewed file as the reviewer and the findings store name it.
pub fn review_key(buffer: &Buffer) -> Option<String> {
    buffer
        .file()
        .map(|f| format!("/{}", f.path().as_unix_str()))
}

/// A reviewed file's path as a brief's globs match it: worktree-relative with
/// `/` separators (ADR-0063 decision 2), or `None` for a buffer with no file.
pub fn scope_path(buffer: &Buffer) -> Option<String> {
    buffer.file().map(|f| f.path().as_unix_str().to_string())
}

fn path_of(buffer: &Buffer, cx: &App) -> PathBuf {
    buffer.file().map(|f| f.full_path(cx)).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import::{self, ImportOutcome};
    use cedian_worker::Brief;
    use fs::{FakeFs, Fs};
    use gpui::TestAppContext;
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;
    use std::path::Path;
    use std::sync::Arc;
    use text::LineEnding;

    const ORIGINAL: &str = "alpha\nbeta\ngamma\n";
    const NOTES: &str = "/ws/notes.txt";

    struct Fixture {
        fs: Arc<FakeFs>,
        project: Entity<Project>,
        buffer: Entity<Buffer>,
        review: TaskReview,
        /// The scope `agent_writes` imports under (ADR-0063); `None` is the
        /// no-running-worker case.
        scope: Option<Arc<WorkerScope>>,
    }

    async fn setup(cx: &mut TestAppContext) -> Fixture {
        setup_with(ORIGINAL, cx).await
    }

    async fn setup_with(original: &str, cx: &mut TestAppContext) -> Fixture {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/ws", json!({"notes.txt": original})).await;
        let project = Project::test(fs.clone(), [Path::new("/ws")], cx).await;
        let buffer = project
            .update(cx, |p, cx| p.open_local_buffer(NOTES, cx))
            .await
            .unwrap();
        let mut review = TaskReview::new("task-1");
        review.begin_turn();
        Fixture {
            fs,
            project,
            buffer,
            review,
            scope: None,
        }
    }

    /// OMP's edit tool writes the disk; the panel's ToolStart/ToolEnd path
    /// imports it as one transaction attributed to `call`.
    async fn agent_writes(f: &mut Fixture, call: &str, text: &str, cx: &mut TestAppContext) {
        agent_writes_as(f, call, text, &["test/model"], cx).await
    }

    /// `agent_writes` while `models` had answered in OMP.
    async fn agent_writes_as(
        f: &mut Fixture,
        call: &str,
        text: &str,
        models: &[&str],
        cx: &mut TestAppContext,
    ) {
        let models: Vec<String> = models.iter().map(|m| m.to_string()).collect();
        cx.update(|cx| f.review.observe(&f.buffer, call, cx));
        let mark = f.buffer.update(cx, |b, cx| import::begin(b, cx));
        let baseline = mark.start().clone();
        f.fs.save(Path::new(NOTES), &text.into(), LineEnding::Unix)
            .await
            .unwrap();
        let outcome = import::finish(f.buffer.clone(), mark, &mut cx.to_async())
            .await
            .unwrap();
        cx.run_until_parked();
        match outcome {
            ImportOutcome::Imported(txn) => cx.update(|cx| {
                let scope = f.scope.clone();
                f.review
                    .agent_edited(&f.buffer, call, txn, baseline, models, scope, cx)
            }),
            ImportOutcome::Stale => cx.update(|cx| f.review.import_refused(&f.buffer, call, cx)),
            ImportOutcome::Unchanged => panic!("the write changed nothing"),
        }
    }

    fn user_types(f: &Fixture, range: Range<usize>, text: &str, cx: &mut TestAppContext) {
        f.buffer.update(cx, |b, cx| {
            b.edit([(range, text)], None, cx);
        });
    }

    fn text(f: &Fixture, cx: &mut TestAppContext) -> String {
        f.buffer.read_with(cx, |b, _| b.text())
    }

    fn key(f: &Fixture, index: usize) -> HunkKey {
        f.review.files()[0].hunks()[index].key.clone()
    }

    fn statuses(f: &Fixture) -> Vec<HunkStatus> {
        f.review.files()[0]
            .hunks()
            .iter()
            .map(|h| h.status)
            .collect()
    }

    /// One running worker's brief, compiled to a scope (ADR-0063).
    fn scope(write: &str) -> Arc<WorkerScope> {
        let brief = Brief::from_json(
            &json!({
                "goal": "g", "scope": {"write": [write]},
                "acceptance": ["a"], "verify": ["v"], "timebox_min": 1,
            })
            .to_string(),
        )
        .unwrap();
        Arc::new(WorkerScope::new([("w1", &brief)]).unwrap())
    }

    /// ADR-0063 decision 1: a file the running worker's brief does not cover
    /// is OutOfScope on every hunk; a covered file is classified as usual.
    #[gpui::test]
    async fn a_file_outside_the_running_workers_scope_is_out_of_scope(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        f.scope = Some(scope("docs/**"));
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        assert_eq!(statuses(&f), vec![HunkStatus::OutOfScope]);
        assert_eq!(f.review.out_of_scope_count(), 1);

        let mut covered = setup(cx).await;
        covered.scope = Some(scope("notes.txt"));
        agent_writes(&mut covered, "c1", "alpha\nBETA\ngamma\n", cx).await;
        assert_eq!(statuses(&covered), vec![HunkStatus::Pending]);
        assert_eq!(covered.review.out_of_scope_count(), 0);
    }

    /// No running worker is no scope: the same edit stays Pending, exactly
    /// as before ADR-0063 (decision 1).
    #[gpui::test]
    async fn with_no_running_worker_nothing_is_out_of_scope(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        assert!(f.scope.is_none());
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        assert_eq!(statuses(&f), vec![HunkStatus::Pending]);
        assert_eq!(f.review.out_of_scope_count(), 0);
    }

    /// An OutOfScope hunk is the person's to resolve like Stale: accept takes
    /// it, reject refuses it (ADR-0033 decision 3).
    #[gpui::test]
    async fn out_of_scope_accepts_but_is_never_rejected(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        f.scope = Some(scope("docs/**"));
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        let path = cx.update(|cx| f.review.path(&f.review.files()[0], cx));
        let k = key(&f, 0);
        let refused = cx.update(|cx| f.review.reject(&path, &k, cx)).unwrap_err();
        assert_eq!(
            refused,
            ReviewError::BadTransition {
                status: HunkStatus::OutOfScope
            }
        );
        cx.update(|cx| f.review.accept(&path, &k, cx)).unwrap();
        assert_eq!(statuses(&f), vec![HunkStatus::Accepted]);
    }

    /// Accept all never takes an OutOfScope hunk, with or without stale
    /// (ADR-0033 decision 3); the count is how many it skipped.
    #[gpui::test]
    async fn accept_all_skips_out_of_scope(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        f.scope = Some(scope("docs/**"));
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        let accepted = cx.update(|cx| f.review.accept_all(true, cx)).unwrap();
        assert!(accepted.is_empty(), "{accepted:?}");
        assert_eq!(statuses(&f), vec![HunkStatus::OutOfScope]);
        assert_eq!(f.review.out_of_scope_count(), 1);
    }

    /// Revert turn keeps an OutOfScope change and says so (ADR-0033
    /// decision 3): it is a person's decision, never reverted.
    #[gpui::test]
    async fn revert_turn_keeps_out_of_scope(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        f.scope = Some(scope("docs/**"));
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        f.review.end_turn();
        let event = cx.update(|cx| f.review.revert_turn(1, cx)).unwrap();
        assert_eq!(
            event,
            ReviewEvent::TurnReverted {
                turn: 1,
                reverted: 0,
                stale: vec![],
                out_of_scope: vec!["/notes.txt line 2".to_string()],
                overwritten: 0,
                accepted: 0,
            }
        );
        assert!(event.to_string().contains("kept out-of-scope"), "{event}");
        assert_eq!(text(&f, cx), "alpha\nBETA\ngamma\n", "the change stays");
        assert_eq!(statuses(&f), vec![HunkStatus::OutOfScope]);
    }

    /// ADR-0039 across an OMP restart: every model that wrote a hunk under
    /// review is an editor, and a hunk whose model is unknown makes the
    /// editors unknown (the review is then inconclusive).
    #[gpui::test]
    async fn every_model_that_wrote_a_reviewed_hunk_is_an_editor(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes_as(&mut f, "c1", "ALPHA\nbeta\ngamma\n", &["a/one"], cx).await;
        agent_writes_as(&mut f, "c2", "ALPHA\nbeta\nGAMMA\n", &["b/two"], cx).await;
        let task = cx.update(|cx| f.review.review_task(cx));
        assert_eq!(task.editors, vec!["a/one".to_string(), "b/two".to_string()]);
        assert!(!task.editors_unknown);
        agent_writes_as(&mut f, "c3", "ALPHA\nBETA\nGAMMA\n", &[], cx).await;
        let task = cx.update(|cx| f.review.review_task(cx));
        assert!(task.editors_unknown, "a hunk with no known model");
    }

    #[gpui::test]
    async fn agent_edit_shows_one_attributed_hunk(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        assert_eq!(f.review.files().len(), 1);
        let hunks = f.review.files()[0].hunks();
        assert_eq!(hunks.len(), 1, "{hunks:?}");
        assert_eq!(hunks[0].status, HunkStatus::Pending);
        assert_eq!(hunks[0].rows, 1..2);
        assert_eq!(
            (hunks[0].old_text.as_str(), hunks[0].new_text.as_str()),
            ("beta\n", "BETA\n")
        );
        assert_eq!(hunks[0].tool_call_ids, vec!["c1".to_string()]);
    }

    /// `Buffer::diff` is word-level: one write can be two edits on one line.
    /// They are one hunk, and reject restores the whole line.
    #[gpui::test]
    async fn two_edits_on_one_line_are_one_hunk(cx: &mut TestAppContext) {
        for (original, written) in [
            ("alpha beta\n", "ALPHA BETAA\n"),
            // A net deletion first, then a second edit on the same line.
            ("alpha beta\n", "A BETAA\n"),
            ("a beta\n", "ALPHA betaY\n"),
        ] {
            let mut f = setup_with(original, cx).await;
            agent_writes(&mut f, "c1", written, cx).await;
            let hunks = f.review.files()[0].hunks().to_vec();
            assert_eq!(hunks.len(), 1, "{original:?} -> {written:?}: {hunks:?}");
            assert_eq!(
                (hunks[0].old_text.as_str(), hunks[0].new_text.as_str()),
                (original, written)
            );
            let path = cx.update(|cx| f.review.path(&f.review.files()[0], cx));
            {
                let k = key(&f, 0);
                cx.update(|cx| f.review.reject(&path, &k, cx))
            }
            .unwrap();
            assert_eq!(text(&f, cx), original, "reject restored {written:?}");
        }
    }

    #[gpui::test]
    async fn reject_restores_baseline_and_logs_correction(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        let path = cx.update(|cx| f.review.path(&f.review.files()[0], cx));
        {
            let k = key(&f, 0);
            cx.update(|cx| f.review.reject(&path, &k, cx))
        }
        .unwrap();
        assert_eq!(text(&f, cx), ORIGINAL, "reject restored the baseline");
        let events = f.review.drain_events();
        assert_eq!(
            events,
            vec![ReviewEvent::HunkRejected {
                path,
                key: HunkKey {
                    before_start: 1,
                    before_count: 1,
                    after_text: "BETA".to_string()
                },
                turn: 1,
                tool_call_id: "c1".to_string(),
            }]
        );
        assert!(
            f.review.files()[0].hunks().is_empty(),
            "no open hunk after reject"
        );
    }

    #[gpui::test]
    async fn reject_is_one_native_undo(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        let path = cx.update(|cx| f.review.path(&f.review.files()[0], cx));
        let txn = {
            let k = key(&f, 0);
            cx.update(|cx| f.review.reject(&path, &k, cx))
        }
        .unwrap();
        f.buffer.update(cx, |b, cx| {
            assert_eq!(b.undo(cx), Some(txn), "one undo is the reject");
            assert_eq!(
                b.text(),
                "alpha\nBETA\ngamma\n",
                "undo brings the agent text back"
            );
        });
        cx.update(|cx| f.review.rebuild(cx));
        assert_eq!(
            statuses(&f),
            vec![HunkStatus::Pending],
            "the agent text is back, so it is a live hunk again"
        );
    }

    /// 4a: the agent retries the same edit after a reject; the new live hunk
    /// is pending, not the old resolution.
    #[gpui::test]
    async fn an_agent_retry_after_a_reject_is_pending(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        let path = cx.update(|cx| f.review.path(&f.review.files()[0], cx));
        {
            let k = key(&f, 0);
            cx.update(|cx| f.review.reject(&path, &k, cx))
        }
        .unwrap();
        f.project
            .update(cx, |p, cx| p.save_buffer(f.buffer.clone(), cx))
            .await
            .unwrap();
        agent_writes(&mut f, "c2", "alpha\nBETA\ngamma\n", cx).await;
        let hunks = f.review.files()[0].hunks();
        assert_eq!(hunks.len(), 1, "{hunks:?}");
        assert_eq!(hunks[0].status, HunkStatus::Pending);
        assert_eq!(hunks[0].tool_call_ids, vec!["c2".to_string()]);
    }

    /// An accepted hunk the agent rewrites with a new transaction is a new
    /// change to review.
    #[gpui::test]
    async fn an_agent_rewrite_of_an_accepted_hunk_is_pending(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        let path = cx.update(|cx| f.review.path(&f.review.files()[0], cx));
        {
            let k = key(&f, 0);
            cx.update(|cx| f.review.accept(&path, &k, cx))
        }
        .unwrap();
        agent_writes(&mut f, "c2", "alpha\nbeta\ngamma\n", cx).await;
        assert!(f.review.files()[0].hunks().is_empty(), "back to baseline");
        agent_writes(&mut f, "c3", "alpha\nBETA\ngamma\n", cx).await;
        assert_eq!(statuses(&f), vec![HunkStatus::Pending]);
        let records = cx.update(|cx| f.review.status_records(cx));
        assert!(records.is_empty(), "{records:?}");
    }

    /// Two turns rewrite the same line: reverting turn 2 gives turn 1's
    /// text, not the baseline; reverting turn 1 under turn 2 is refused.
    #[gpui::test]
    async fn revert_turn_restores_the_turns_own_start(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "alpha\nONE\ngamma\n", cx).await;
        f.review.begin_turn();
        agent_writes(&mut f, "c2", "alpha\nTWO\ngamma\n", cx).await;
        f.review.end_turn();
        let event = cx.update(|cx| f.review.revert_turn(2, cx)).unwrap();
        assert_eq!(
            event,
            ReviewEvent::TurnReverted {
                turn: 2,
                reverted: 1,
                stale: vec![],
                out_of_scope: vec![],
                overwritten: 0,
                accepted: 0,
            }
        );
        assert_eq!(text(&f, cx), "alpha\nONE\ngamma\n");
        let hunks = f.review.files()[0].hunks();
        assert_eq!(hunks.len(), 1, "{hunks:?}");
        assert_eq!(hunks[0].status, HunkStatus::Pending);
        assert_eq!(hunks[0].tool_call_ids, vec!["c1".to_string()]);
    }

    /// U10b (ADR-0056 decision 3): an inline edit is its own kind of turn,
    /// named by its label, and reverts like any turn.
    #[gpui::test]
    async fn an_inline_edit_turn_is_named_and_reverts_like_any_turn(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        f.review.end_turn();
        let n = f
            .review
            .begin_edit_turn("notes.txt:2 make this line uppercase".into());
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        f.review.end_turn();
        assert_eq!(
            f.review.turn_label(n),
            Some("notes.txt:2 make this line uppercase")
        );
        assert_eq!(f.review.turn_label(1), None, "a prompt turn has no label");
        cx.update(|cx| f.review.revert_turn(n, cx)).unwrap();
        assert_eq!(text(&f, cx), ORIGINAL);
    }

    #[gpui::test]
    async fn revert_of_a_turn_a_later_turn_changed_is_refused(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "alpha\nONE\ngamma\n", cx).await;
        f.review.begin_turn();
        agent_writes(&mut f, "c2", "alpha\nTWO\ngamma\n", cx).await;
        f.review.end_turn();
        let event = cx.update(|cx| f.review.revert_turn(1, cx)).unwrap();
        assert_eq!(
            event,
            ReviewEvent::TurnReverted {
                turn: 1,
                reverted: 0,
                stale: vec![],
                out_of_scope: vec![],
                overwritten: 1,
                accepted: 0,
            }
        );
        assert_eq!(text(&f, cx), "alpha\nTWO\ngamma\n", "turn 2's text stays");
        assert!(
            event.to_string().contains("changed again by a later turn"),
            "{event}"
        );
    }

    /// A revert closes the reverted turn's window: reverting turn 2 after
    /// turn 1 was reverted does not put turn 1's text back.
    #[gpui::test]
    async fn reverting_a_later_turn_does_not_reapply_an_earlier_reverted_one(
        cx: &mut TestAppContext,
    ) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "ALPHA\nbeta\ngamma\n", cx).await;
        f.review.begin_turn();
        agent_writes(&mut f, "c2", "ALPHA\nbeta\nGAMMA\n", cx).await;
        f.review.end_turn();
        cx.update(|cx| f.review.revert_turn(1, cx)).unwrap();
        assert_eq!(text(&f, cx), "alpha\nbeta\nGAMMA\n");
        let event = cx.update(|cx| f.review.revert_turn(2, cx)).unwrap();
        assert_eq!(
            event,
            ReviewEvent::TurnReverted {
                turn: 2,
                reverted: 1,
                stale: vec![],
                out_of_scope: vec![],
                overwritten: 0,
                accepted: 0,
            }
        );
        assert_eq!(
            text(&f, cx),
            ORIGINAL,
            "line 1 stays reverted, line 3 is back"
        );
    }

    #[gpui::test]
    async fn reverting_a_turn_twice_is_nothing_to_do(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "ALPHA\nbeta\ngamma\n", cx).await;
        f.review.end_turn();
        cx.update(|cx| f.review.revert_turn(1, cx)).unwrap();
        let event = cx.update(|cx| f.review.revert_turn(1, cx)).unwrap();
        assert_eq!(
            event,
            ReviewEvent::TurnReverted {
                turn: 1,
                reverted: 0,
                stale: vec![],
                out_of_scope: vec![],
                overwritten: 0,
                accepted: 0,
            }
        );
        assert_eq!(event.to_string(), "turn 1: nothing to do");
        assert_eq!(text(&f, cx), ORIGINAL);
    }

    /// A file listed as unreviewable is a reviewed file again once a later
    /// call imports it.
    #[gpui::test]
    async fn a_later_import_clears_the_files_unreviewable_entry(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        let path = cx.update(|cx| path_of(f.buffer.read(cx), cx));
        f.review
            .could_not_review(path.clone(), "permission denied".into());
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        assert!(
            f.review.unreviewable().is_empty(),
            "{:?}",
            f.review.unreviewable()
        );
        assert_eq!(statuses(&f), vec![HunkStatus::Pending]);
    }

    /// Reverting a turn keeps an accepted hunk and says so.
    #[gpui::test]
    async fn revert_turn_counts_an_accepted_hunk_it_keeps(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "ALPHA\nbeta\nGAMMA\n", cx).await;
        let path = cx.update(|cx| f.review.path(&f.review.files()[0], cx));
        {
            let k = key(&f, 0);
            cx.update(|cx| f.review.accept(&path, &k, cx)).unwrap();
        }
        f.review.end_turn();
        let event = cx.update(|cx| f.review.revert_turn(1, cx)).unwrap();
        assert_eq!(
            event,
            ReviewEvent::TurnReverted {
                turn: 1,
                reverted: 1,
                stale: vec![],
                out_of_scope: vec![],
                overwritten: 0,
                accepted: 1,
            }
        );
        assert!(event.to_string().contains("1 accepted, kept"), "{event}");
        assert_eq!(text(&f, cx), "ALPHA\nbeta\ngamma\n");
    }

    #[gpui::test]
    async fn user_edit_over_agent_hunk_is_stale(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        user_types(&f, 10..10, " by user", cx);
        cx.update(|cx| f.review.rebuild(cx));
        assert_eq!(statuses(&f), vec![HunkStatus::Stale]);
    }

    #[gpui::test]
    async fn stale_event_once(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        user_types(&f, 10..10, " by user", cx);
        cx.update(|cx| {
            f.review.rebuild(cx);
            f.review.rebuild(cx);
        });
        let path = cx.update(|cx| f.review.path(&f.review.files()[0], cx));
        let stale: Vec<_> = f
            .review
            .drain_events()
            .into_iter()
            .filter(|e| matches!(e, ReviewEvent::UserEditedAgentHunk { .. }))
            .collect();
        assert_eq!(
            stale,
            vec![ReviewEvent::UserEditedAgentHunk {
                path,
                key: HunkKey {
                    before_start: 1,
                    before_count: 1,
                    after_text: "BETA by user".to_string()
                },
                turn: 1,
            }],
            "one event per edited hunk, however often the review rebuilds"
        );
    }

    #[gpui::test]
    async fn reject_of_stale_hunk_refused(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        user_types(&f, 10..10, " by user", cx);
        cx.update(|cx| f.review.rebuild(cx));
        let path = cx.update(|cx| f.review.path(&f.review.files()[0], cx));
        assert_eq!(
            {
                let k = key(&f, 0);
                cx.update(|cx| f.review.reject(&path, &k, cx))
            },
            Err(ReviewError::BadTransition {
                status: HunkStatus::Stale
            })
        );
        assert_eq!(
            text(&f, cx),
            "alpha\nBETA by user\ngamma\n",
            "the user's line survives"
        );
    }

    #[gpui::test]
    async fn accept_stale_keeps_buffer_and_persists(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        user_types(&f, 10..10, " by user", cx);
        cx.update(|cx| f.review.rebuild(cx));
        let path = cx.update(|cx| f.review.path(&f.review.files()[0], cx));
        {
            let k = key(&f, 0);
            cx.update(|cx| f.review.accept(&path, &k, cx))
        }
        .unwrap();
        cx.update(|cx| f.review.rebuild(cx));
        assert_eq!(statuses(&f), vec![HunkStatus::Accepted]);
        assert_eq!(
            text(&f, cx),
            "alpha\nBETA by user\ngamma\n",
            "accept keeps the buffer"
        );
        // A fresh review of the same buffer (a reopened task) gets the
        // resolution back from the persisted records.
        let records = cx.update(|cx| f.review.status_records(cx));
        let mut fresh = TaskReview::new("task-1");
        fresh.begin_turn();
        let txn = f.review.files()[0].agent_txns()[0].transaction;
        let baseline = f.review.files()[0].baseline.clone();
        cx.update(|cx| {
            fresh.agent_edited(
                &f.buffer,
                "c1",
                txn,
                baseline,
                vec!["test/model".to_string()],
                None,
                cx,
            );
            fresh.restore_statuses(records, cx);
        });
        assert_eq!(
            fresh.files()[0].hunks()[0].status,
            HunkStatus::Accepted,
            "accepted after reopen"
        );
    }

    #[gpui::test]
    async fn baseline_excludes_pre_task_user_edits(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        // The user's own change, saved before the turn (Agent Sync, §13).
        user_types(&f, 0..5, "ALPHA", cx);
        f.project
            .update(cx, |p, cx| p.save_buffer(f.buffer.clone(), cx))
            .await
            .unwrap();
        agent_writes(&mut f, "c1", "ALPHA\nBETA\ngamma\n", cx).await;
        let hunks = f.review.files()[0].hunks();
        assert_eq!(hunks.len(), 1, "{hunks:?}");
        assert_eq!(hunks[0].rows, 1..2, "the user's line 1 is not a hunk");
        assert_eq!(hunks[0].status, HunkStatus::Pending);
    }

    #[gpui::test]
    async fn reject_refuses_a_buffer_that_moved(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        let path = cx.update(|cx| f.review.path(&f.review.files()[0], cx));
        user_types(&f, 0..0, "zero\n", cx);
        assert!(matches!(
            {
                let k = key(&f, 0);
                cx.update(|cx| f.review.reject(&path, &k, cx))
            },
            Err(ReviewError::Outdated { .. })
        ));
        assert_eq!(text(&f, cx), "zero\nalpha\nBETA\ngamma\n");
    }

    #[gpui::test]
    async fn accept_all_takes_stale_only_when_asked(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "ALPHA\nbeta\nGAMMA\n", cx).await;
        user_types(&f, 11..11, "!", cx);
        cx.update(|cx| f.review.rebuild(cx));
        assert_eq!(statuses(&f), vec![HunkStatus::Pending, HunkStatus::Stale]);
        assert_eq!(f.review.stale_count(), 1);
        let accepted = cx.update(|cx| f.review.accept_all(false, cx)).unwrap();
        assert_eq!(accepted.len(), 1);
        assert_eq!(statuses(&f), vec![HunkStatus::Accepted, HunkStatus::Stale]);
        let accepted = cx.update(|cx| f.review.accept_all(true, cx)).unwrap();
        assert_eq!(accepted.len(), 1);
        assert_eq!(
            statuses(&f),
            vec![HunkStatus::Accepted, HunkStatus::Accepted]
        );
    }

    /// S0 check 9: reverting a turn puts back what that turn's calls wrote,
    /// keeps a hunk the user edited (STALE), is one native undo per buffer,
    /// and an earlier turn can still be reverted afterwards.
    #[gpui::test]
    async fn revert_turn_skips_stale_and_is_undoable(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "ALPHA\nbeta\ngamma\n", cx).await;
        f.review.begin_turn();
        agent_writes(&mut f, "c2", "ALPHA\nBETA\ngamma\ndelta\n", cx).await;
        user_types(&f, 22..22, " by user", cx);
        f.review.end_turn();
        let event = cx.update(|cx| f.review.revert_turn(2, cx)).unwrap();
        assert_eq!(
            event,
            ReviewEvent::TurnReverted {
                turn: 2,
                reverted: 1,
                stale: vec!["/notes.txt line 4".to_string()],
                out_of_scope: vec![],
                overwritten: 0,
                accepted: 0,
            }
        );
        assert_eq!(
            text(&f, cx),
            "ALPHA\nbeta\ngamma\ndelta by user\n",
            "turn 2's line 2 is back, the user's line 4 stays, turn 1's line 1 stays"
        );
        let said = event.to_string();
        assert!(
            said.contains("/notes.txt line 4") && said.contains("edited after the agent"),
            "the notice names the STALE hunk it kept and why: {said}"
        );
        f.buffer.update(cx, |b, cx| {
            b.undo(cx);
            assert_eq!(
                b.text(),
                "ALPHA\nBETA\ngamma\ndelta by user\n",
                "one undo redoes the revert"
            );
            b.redo(cx);
        });
        cx.update(|cx| f.review.revert_turn(1, cx)).unwrap();
        assert_eq!(text(&f, cx), "alpha\nbeta\ngamma\ndelta by user\n");
    }

    /// ADR-0057 decision 5: the person undoes an agent hunk, then redoes
    /// it, with a rebuild after each as the panel makes on every edit. The
    /// hunk stays STALE, so Revert turn never writes over the person's
    /// text, and its notice names the hunk it kept.
    #[gpui::test]
    async fn undo_then_redo_of_an_agent_hunk_stays_stale(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        f.review.end_turn();
        f.buffer.update(cx, |b, cx| {
            b.undo(cx);
        });
        assert_eq!(
            text(&f, cx),
            ORIGINAL,
            "the person's undo takes the agent's write back"
        );
        cx.update(|cx| f.review.rebuild(cx));
        f.buffer.update(cx, |b, cx| {
            b.redo(cx);
        });
        assert_eq!(text(&f, cx), "alpha\nBETA\ngamma\n");
        cx.update(|cx| f.review.rebuild(cx));
        let event = cx.update(|cx| f.review.revert_turn(1, cx)).unwrap();
        assert_eq!(
            event,
            ReviewEvent::TurnReverted {
                turn: 1,
                reverted: 0,
                stale: vec!["/notes.txt line 2".to_string()],
                out_of_scope: vec![],
                overwritten: 0,
                accepted: 0,
            }
        );
        assert_eq!(text(&f, cx), "alpha\nBETA\ngamma\n", "nothing written over");
        assert!(event.to_string().contains("/notes.txt line 2"));
    }

    #[gpui::test]
    async fn revert_of_a_streaming_turn_is_refused(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        assert_eq!(
            cx.update(|cx| f.review.revert_turn(1, cx)),
            Err(ReviewError::TurnInFlight { turn: 1 })
        );
        assert_eq!(text(&f, cx), "alpha\nBETA\ngamma\n");
        f.review.end_turn();
        cx.update(|cx| f.review.revert_turn(1, cx)).unwrap();
        assert_eq!(text(&f, cx), ORIGINAL);
    }

    /// The file's window closed early (a release) and a display rebuild
    /// folded the import's own edit as the user's before the import was
    /// attributed: the attribution drops that fold, so the hunk is Pending.
    #[gpui::test]
    async fn an_import_landing_after_an_early_close_is_still_the_agents(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c0", "ALPHA\nbeta\ngamma\n", cx).await;
        cx.update(|cx| f.review.observe(&f.buffer, "c1", cx));
        let mark = f.buffer.update(cx, |b, cx| import::begin(b, cx));
        let baseline = mark.start().clone();
        cx.update(|cx| f.review.import_done(&f.buffer, "c1", cx));
        f.fs.save(
            Path::new(NOTES),
            &"ALPHA\nbeta\nGAMMA\n".into(),
            LineEnding::Unix,
        )
        .await
        .unwrap();
        let outcome = import::finish(f.buffer.clone(), mark, &mut cx.to_async())
            .await
            .unwrap();
        let ImportOutcome::Imported(txn) = outcome else {
            panic!("{outcome:?}");
        };
        cx.update(|cx| {
            f.review.rebuild(cx);
            f.review.agent_edited(
                &f.buffer,
                "c1",
                txn,
                baseline,
                vec!["test/model".to_string()],
                None,
                cx,
            );
        });
        assert_eq!(statuses(&f), vec![HunkStatus::Pending, HunkStatus::Pending]);
    }

    /// A keystroke inside the word OMP rewrote, autosaved and kept by OMP's
    /// write (it read the autosaved file), with the watcher ahead: the save
    /// in the window makes the import refuse, so the file is STALE as a
    /// whole, the user's text is kept, and nothing is rejectable over it.
    #[gpui::test]
    async fn a_user_edit_the_write_kept_is_not_the_imports_own(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c0", "alpha\nbeta\nGAMMA\n", cx).await;
        cx.update(|cx| f.review.observe(&f.buffer, "c1", cx));
        let mark = f.buffer.update(cx, |b, cx| import::begin(b, cx));
        user_types(&f, 5..5, "foo", cx);
        cx.update(|cx| f.review.observe(&f.buffer, "c2", cx));
        f.project
            .update(cx, |p, cx| p.save_buffer(f.buffer.clone(), cx))
            .await
            .unwrap();
        f.fs.save(
            Path::new(NOTES),
            &"ALPHAfoo\nbeta\nGAMMA\n".into(),
            LineEnding::Unix,
        )
        .await
        .unwrap();
        cx.run_until_parked();
        assert_eq!(
            text(&f, cx),
            "ALPHAfoo\nbeta\nGAMMA\n",
            "the watcher reloaded"
        );
        let outcome = import::finish(f.buffer.clone(), mark, &mut cx.to_async())
            .await
            .unwrap();
        assert_eq!(outcome, ImportOutcome::Stale);
        cx.update(|cx| {
            f.review.import_refused(&f.buffer, "c1", cx);
            f.review.import_done(&f.buffer, "c2", cx);
        });
        assert!(f.review.files()[0].stale_import().is_some());
        assert_eq!(statuses(&f), vec![HunkStatus::Stale, HunkStatus::Pending]);
        let key = key(&f, 0);
        let path = cx.update(|cx| f.review.path(&f.review.files()[0], cx));
        assert_eq!(
            cx.update(|cx| f.review.reject(&path, &key, cx)),
            Err(ReviewError::BadTransition {
                status: HunkStatus::Stale
            })
        );
        assert_eq!(text(&f, cx), "ALPHAfoo\nbeta\nGAMMA\n");
    }

    /// Autosave plus the watcher racing ahead: the user's saved transaction
    /// during the call sits under the watcher's reload of OMP's write. The
    /// save in the window refuses the import: the file is STALE as a whole
    /// and nothing is rejectable, the agent's line included.
    #[gpui::test]
    async fn a_user_transaction_under_the_watchers_reload_is_stale(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        cx.update(|cx| f.review.observe(&f.buffer, "c1", cx));
        let mark = f.buffer.update(cx, |b, cx| import::begin(b, cx));
        user_types(&f, 0..5, "by user", cx);
        f.project
            .update(cx, |p, cx| p.save_buffer(f.buffer.clone(), cx))
            .await
            .unwrap();
        f.fs.save(
            Path::new(NOTES),
            &"by user\nBETA\ngamma\n".into(),
            LineEnding::Unix,
        )
        .await
        .unwrap();
        cx.run_until_parked();
        assert_eq!(
            text(&f, cx),
            "by user\nBETA\ngamma\n",
            "the watcher reloaded"
        );
        let outcome = import::finish(f.buffer.clone(), mark, &mut cx.to_async())
            .await
            .unwrap();
        assert_eq!(outcome, ImportOutcome::Stale);
        cx.update(|cx| f.review.import_refused(&f.buffer, "c1", cx));
        let file = &f.review.files()[0];
        assert!(file.stale_import().unwrap().contains("c1"));
        assert!(file.hunks().is_empty(), "{:?}", file.hunks());
        assert_eq!(text(&f, cx), "by user\nBETA\ngamma\n");
    }

    #[gpui::test]
    async fn a_clean_import_clears_the_files_stale_import(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        user_types(&f, 0..0, "USER ", cx);
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        assert!(f.review.files()[0].stale_import().is_some());
        user_types(&f, 0..5, "", cx);
        f.project
            .update(cx, |p, cx| p.save_buffer(f.buffer.clone(), cx))
            .await
            .unwrap();
        // The user's edit left the buffer clean again under a changed disk
        // mtime, so Zed reloads it; let that reload and the save settle
        // before the next call, as they would between two OMP turns.
        cx.run_until_parked();
        agent_writes(&mut f, "c2", "alpha\nBETA\ngamma\n", cx).await;
        assert_eq!(f.review.files()[0].stale_import(), None);
        assert_eq!(statuses(&f), vec![HunkStatus::Pending]);
    }

    /// A call that deletes a line is credited with the deletion, not with
    /// a line another call edited. Next to that line, the CRDT reports one
    /// edit for both, and that one hunk is both calls' work.
    #[gpui::test]
    async fn a_deletion_is_credited_to_its_own_call(cx: &mut TestAppContext) {
        let mut f = setup_with("alpha\nbeta\ngamma\ndelta\n", cx).await;
        agent_writes(&mut f, "c1", "alpha\nbeta\ngamma\nDELTA\n", cx).await;
        agent_writes(&mut f, "c2", "alpha\ngamma\nDELTA\n", cx).await;
        let hunks: Vec<(&str, &str, Vec<String>)> = f.review.files()[0]
            .hunks()
            .iter()
            .map(|h| {
                (
                    h.old_text.as_str(),
                    h.new_text.as_str(),
                    h.tool_call_ids.clone(),
                )
            })
            .collect();
        assert_eq!(
            hunks,
            vec![
                ("beta\n", "", vec!["c2".to_string()]),
                ("delta\n", "DELTA\n", vec!["c1".to_string()]),
            ]
        );

        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "alpha\nbeta\nGAMMA\n", cx).await;
        agent_writes(&mut f, "c2", "alpha\nGAMMA\n", cx).await;
        let hunks: Vec<(&str, &str, Vec<String>)> = f.review.files()[0]
            .hunks()
            .iter()
            .map(|h| {
                (
                    h.old_text.as_str(),
                    h.new_text.as_str(),
                    h.tool_call_ids.clone(),
                )
            })
            .collect();
        assert_eq!(
            hunks,
            vec![(
                "beta\ngamma\n",
                "GAMMA\n",
                vec!["c1".to_string(), "c2".to_string()]
            )]
        );
    }

    /// A rejected call's emptied range does not credit it with what a later
    /// call writes on that line.
    #[gpui::test]
    async fn a_rejected_call_is_not_credited_with_a_later_calls_line(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        let path = cx.update(|cx| f.review.path(&f.review.files()[0], cx));
        {
            let k = key(&f, 0);
            cx.update(|cx| f.review.reject(&path, &k, cx)).unwrap();
        }
        f.project
            .update(cx, |p, cx| p.save_buffer(f.buffer.clone(), cx))
            .await
            .unwrap();
        agent_writes(&mut f, "c2", "alpha\nbeta two\ngamma\n", cx).await;
        let hunks = f.review.files()[0].hunks();
        assert_eq!(hunks.len(), 1, "{hunks:?}");
        assert_eq!(hunks[0].tool_call_ids, vec!["c2".to_string()]);
    }

    #[gpui::test]
    async fn dirty_buffer_import_is_stale_for_the_file(cx: &mut TestAppContext) {
        let mut f = setup(cx).await;
        user_types(&f, 0..0, "USER ", cx);
        agent_writes(&mut f, "c1", "alpha\nBETA\ngamma\n", cx).await;
        assert_eq!(
            text(&f, cx),
            "USER alpha\nbeta\ngamma\n",
            "nothing imported"
        );
        let file = &f.review.files()[0];
        assert!(
            file.stale_import().unwrap().contains("c1"),
            "{:?}",
            file.stale_import()
        );
        assert!(file.hunks().is_empty());
        assert!(!f.review.is_empty(), "the file still shows in review");
    }
}
