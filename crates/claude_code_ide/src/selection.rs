//! Editor selection: the MCP tools that read it and the shared state behind them.
//!
//! Tool handlers could read the active editor live, but `selection_changed` needs
//! the same computation on every change anyway, so [`capture`] computes it once and
//! both paths read the cached value.

use gpui::{App, AsyncApp, Entity, WeakEntity};
use language::LocalFile as _;
use std::cell::RefCell;
use std::rc::Rc;
use workspace::Workspace;

use crate::protocol::{Position, SelectionPayload, SelectionRange, tool_error, tool_ok};
use serde_json::{Value, json};

/// State shared between the bridge (which updates it) and the server dispatch
/// loop (which reads it). Single-threaded: only touched on the foreground.
#[derive(Clone)]
pub struct SharedSelection(Rc<RefCell<SelectionState>>);

#[derive(Default)]
struct SelectionState {
    editor: Option<SelectionPayload>,
    /// One region per external source. A `Vec` rather than a map so the aggregate
    /// text keeps a stable order across pushes.
    external: Vec<ExternalRegion>,
    /// The most recently set region, used as the aggregate's top-level `selection`
    /// and `filePath` so the CLI's single-region banner names your latest action
    /// rather than a synthesised range spanning files.
    last_primary: Option<SelectionPayload>,
    workspace: Option<WeakEntity<Workspace>>,
    /// For tools that drive UI (openFile/openDiff).
    window: Option<gpui::AnyWindowHandle>,
}

/// One external source's contribution, plus the process that pushed it.
///
/// `owner_pid` exists so a region can be reaped when its owner dies: a client that
/// is killed never gets to retract, and because keys are per-process nothing would
/// replace it either. `None` when the pid could not be read, and then the region is
/// kept, because an unknown owner is not evidence of a dead one.
struct ExternalRegion {
    source: String,
    payload: SelectionPayload,
    owner_pid: Option<u32>,
}

/// Whether a process still exists. `kill(pid, 0)` performs the permission and
/// existence checks without sending a signal, which is the standard liveness test.
/// Reports `true` on any non-unix target, so a region is never reaped on a platform
/// where we cannot ask.
fn process_is_alive(pid: u32) -> bool {
    #[cfg(unix)]
    {
        // `kill` treats 0 as "every process in my group" and negatives as a group
        // id, so those must never reach it: pid 0 would answer "alive" for the
        // caller's own group, and a pid that casts negative would ask about the
        // wrong thing entirely. Neither is a real owner, so both read as dead.
        let Ok(raw) = i32::try_from(pid) else {
            return false;
        };
        if raw <= 0 {
            return false;
        }
        // SAFETY: `kill` with signal 0 sends nothing; it only performs the
        // existence and permission checks. No memory is touched.
        let result = unsafe { libc::kill(raw as libc::pid_t, 0) };
        if result == 0 {
            return true;
        }
        // EPERM means it exists but belongs to someone else, which still counts as
        // alive. Only ESRCH means "no such process".
        std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH)
    }
    #[cfg(not(unix))]
    {
        let _ = pid;
        true
    }
}

impl SharedSelection {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(SelectionState::default())))
    }

    pub fn set_workspace(&self, workspace: WeakEntity<Workspace>) {
        self.0.borrow_mut().workspace = Some(workspace);
    }

    pub fn set_window(&self, window: gpui::AnyWindowHandle) {
        self.0.borrow_mut().window = Some(window);
    }

    pub fn window(&self) -> Option<gpui::AnyWindowHandle> {
        self.0.borrow().window
    }

    /// Record the Zed editor's current selection (or clear it with `None`).
    /// When present it also becomes the primary (most-recent) region.
    pub fn set_editor(&self, payload: Option<SelectionPayload>) {
        let mut state = self.0.borrow_mut();
        if let Some(payload) = &payload {
            state.last_primary = Some(payload.clone());
        }
        state.editor = payload;
    }

    /// Upsert one external source's current selection, keyed by `source`.
    /// Replaces that source's previous entry (re-selecting in the same nvim),
    /// and makes it the primary (most-recent) region.
    pub fn upsert_external(
        &self,
        source: String,
        payload: SelectionPayload,
        owner_pid: Option<u32>,
    ) {
        let mut state = self.0.borrow_mut();
        state.last_primary = Some(payload.clone());
        if let Some(entry) = state.external.iter_mut().find(|entry| entry.source == source) {
            entry.payload = payload;
            // Trust the newest sighting: the same logical source may reconnect from
            // a different process (a restarted editor reusing an explicit key).
            entry.owner_pid = owner_pid;
        } else {
            state.external.push(ExternalRegion {
                source,
                payload,
                owner_pid,
            });
        }
        state.reap_dead_owners();
    }

    /// Drop one source's region, reporting whether there was one to drop.
    ///
    /// `last_primary` is left alone deliberately: the remaining regions still need a
    /// truthful top-level range. A caller that removes the final region should
    /// broadcast a cleared payload, not an aggregate.
    pub fn remove_external(&self, source: &str) -> bool {
        let mut state = self.0.borrow_mut();
        let before = state.external.len();
        state.external.retain(|entry| entry.source != source);
        before != state.external.len()
    }

    /// Drop every external region. The escape hatch for one whose key nothing can
    /// address any more, so neither a retraction nor a replacement can reach it.
    pub fn remove_all_external(&self) -> usize {
        let mut state = self.0.borrow_mut();
        let removed = state.external.len();
        state.external.clear();
        removed
    }

    /// The aggregate across every source, or `None` when nothing is selected. What
    /// `getCurrentSelection` returns and what every `selection_changed` broadcasts.
    ///
    /// Reaps dead owners first. Reaping on read as well as on write matters because
    /// editor selection changes drive most broadcasts and never touch the write path.
    pub fn latest(&self) -> Option<SelectionPayload> {
        let mut state = self.0.borrow_mut();
        state.reap_dead_owners();
        state.aggregate()
    }

    pub fn workspace(&self) -> Option<WeakEntity<Workspace>> {
        self.0.borrow().workspace.clone()
    }
}

impl SelectionState {
    /// Drop regions whose owning process no longer exists. Without this, a client
    /// that was killed leaves a region nothing can retract or replace. An unknown
    /// owner is kept, because it is not evidence of a dead one.
    fn reap_dead_owners(&mut self) {
        self.external.retain(|entry| match entry.owner_pid {
            Some(pid) => {
                let alive = process_is_alive(pid);
                if !alive {
                    log::info!(
                        "claude_code_ide: reaping selection from {} (owner pid {pid} is gone)",
                        entry.source
                    );
                }
                alive
            }
            None => true,
        });
    }

    /// Combine every source into one payload. None for zero regions, the payload
    /// verbatim for one, and for several a `text` concatenating each under a
    /// `# <path>:<start>-<end>` header while the top-level fields stay truthful to
    /// the primary region, so the CLI's single-region banner never lies.
    fn aggregate(&self) -> Option<SelectionPayload> {
        let mut regions: Vec<&SelectionPayload> = Vec::new();
        if let Some(editor) = &self.editor {
            regions.push(editor);
        }
        regions.extend(self.external.iter().map(|entry| &entry.payload));

        match regions.as_slice() {
            [] => None,
            [only] => Some((*only).clone()),
            many => {
                let text = many
                    .iter()
                    .map(|region| {
                        format!(
                            "# {}:{}-{}\n{}",
                            region.file_path,
                            region.selection.start.line + 1,
                            region.selection.end.line + 1,
                            region.text,
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n\n");
                // Top-level fields stay truthful to the primary region; only
                // `text` carries every region. Fall back to the first region if
                // no primary was recorded (should not happen once any source
                // has been set).
                let primary = self.last_primary.clone().unwrap_or_else(|| many[0].clone());
                Some(SelectionPayload {
                    text,
                    file_path: primary.file_path,
                    file_url: primary.file_url,
                    selection: primary.selection,
                })
            }
        }
    }
}

impl Default for SharedSelection {
    fn default() -> Self {
        Self::new()
    }
}

/// The tools advertised in `tools/list`. The names are the wire contract: Claude
/// Code surfaces each one to the model as `mcp__ide__<name>`.
pub fn tools() -> Value {
    let none = json!({ "type": "object", "properties": {} });
    let tool = |name: &str, description: &str, input_schema: Value| {
        json!({ "name": name, "description": description, "inputSchema": input_schema })
    };
    json!([
        tool("getCurrentSelection", "Get the active editor's current text selection.", none.clone()),
        tool("getLatestSelection", "Get the most recent text selection across editors.", none.clone()),
        tool("getWorkspaceFolders", "Get the workspace's root folders.", none.clone()),
        tool("openDiff", "Open a diff for review.",
            json!({
                "type": "object",
                "properties": {
                    "old_file_path": { "type": "string" },
                    "new_file_path": { "type": "string" },
                    "new_file_contents": { "type": "string" },
                    "tab_name": { "type": "string" },
                },
            })),
        tool("closeAllDiffTabs", "Close all diff tabs.", none),
        tool("executeCode", "Execute Python in a Jupyter kernel (requires an open notebook).",
            json!({
                "type": "object",
                "properties": { "code": { "type": "string" } },
                "required": ["code"],
            })),
    ])
}

/// Build the `getCurrentSelection` / `getLatestSelection` tool result from the
/// cached selection. Recomputes live from the active editor first so a tool
/// call reflects the very latest cursor, then falls back to the cache.
pub async fn current_selection_result(
    selection: &SharedSelection,
    cx: &mut AsyncApp,
) -> serde_json::Value {
    // Refresh the editor slot from the live active editor so a tool call
    // reflects the very latest cursor, then return the AGGREGATE across the
    // editor and every external source (so `claude` sees all selected text in
    // all files, not just the editor's).
    if let Some(workspace) = selection.workspace() {
        let live = cx.update(|cx| compute_active_selection(&workspace, cx));
        selection.set_editor(live);
    }

    match selection.latest() {
        Some(payload) => payload_result(&payload),
        None => tool_ok("{}"),
    }
}

fn payload_result(payload: &SelectionPayload) -> serde_json::Value {
    match serde_json::to_string(payload) {
        Ok(text) => tool_ok(text),
        Err(err) => tool_error(format!("failed to encode selection: {err}")),
    }
}

/// Build the `getWorkspaceFolders` tool result from the project's worktrees.
pub async fn workspace_folders_result(
    selection: &SharedSelection,
    cx: &mut AsyncApp,
) -> serde_json::Value {
    let Some(workspace) = selection.workspace() else {
        return tool_error("no workspace bound");
    };
    let folders = cx.update(|cx| compute_workspace_folders(&workspace, cx));
    match folders {
        Some(result) => match serde_json::to_string(&result) {
            Ok(text) => tool_ok(text),
            Err(err) => tool_error(format!("failed to encode folders: {err}")),
        },
        None => tool_ok(r#"{"folders":[],"rootPath":null}"#),
    }
}

/// Read the active editor's newest selection into a payload, or `None` when no
/// editor is active or it has no backing file.
pub fn compute_active_selection(
    workspace: &WeakEntity<Workspace>,
    cx: &mut App,
) -> Option<SelectionPayload> {
    let workspace = workspace.upgrade()?;
    let editor = workspace
        .read(cx)
        .active_item_as::<editor::Editor>(cx)?;
    editor.update(cx, |editor, cx| selection_payload(editor, cx))
}

/// One resolved selection region: its file, range, and selected text.
struct SelectionRegion {
    file_path: String,
    start: language::Point,
    end: language::Point,
    text: String,
}

fn selection_payload(editor: &mut editor::Editor, cx: &mut App) -> Option<SelectionPayload> {
    let display_snapshot = editor.display_snapshot(cx);
    let buffer_snapshot = display_snapshot.buffer_snapshot();
    let multi_buffer = editor.buffer().clone();

    // The newest selection is the PRIMARY region: the top-level `selection` /
    // `filePath` / `fileUrl` stay truthful to it, so the CLI's single-region
    // banner ("N lines from <file>") never lies. Additional regions are carried
    // in `text` only (see below) - the protocol has no `selections[]` array, so
    // multi-selection has nowhere else to go.
    let newest = editor.selections.newest::<language::Point>(&display_snapshot);
    let (primary_buffer, _) = multi_buffer
        .read(cx)
        .point_to_buffer_offset(newest.head(), cx)?;
    let primary_path = project::File::from_dyn(primary_buffer.read(cx).file())
        .map(|file| file.abs_path(cx))?;
    let file_path = primary_path.to_string_lossy().to_string();
    let file_url = format!("file://{file_path}");

    // Resolve every disjoint selection to (path, range, text). A region whose
    // buffer has no backing file (scratch/untitled) can't be attributed, so it
    // is dropped rather than mislabelled.
    let regions: Vec<SelectionRegion> = editor
        .selections
        .all::<language::Point>(&display_snapshot)
        .into_iter()
        .filter_map(|selection| {
            let text = buffer_snapshot
                .text_for_range(selection.start..selection.end)
                .collect::<String>();
            let (buffer, _) = multi_buffer
                .read(cx)
                .point_to_buffer_offset(selection.head(), cx)?;
            let abs_path = project::File::from_dyn(buffer.read(cx).file())
                .map(|file| file.abs_path(cx))?;
            Some(SelectionRegion {
                file_path: abs_path.to_string_lossy().to_string(),
                start: selection.start,
                end: selection.end,
                text,
            })
        })
        .collect();

    // Single region: send its text verbatim (the proven single-select shape, so
    // the banner label from `filePath` is clean). Multiple regions: concatenate
    // all of them into `text` under `# <path>:<startLine>-<endLine>` headers so
    // the model can attribute each block to its file and lines. `text_for_range`
    // over the primary range covers the fallback where no region is file-backed.
    let text = if regions.len() > 1 {
        regions
            .iter()
            .map(|region| {
                format!(
                    "# {}:{}-{}\n{}",
                    region.file_path,
                    region.start.row + 1,
                    region.end.row + 1,
                    region.text,
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    } else {
        buffer_snapshot
            .text_for_range(newest.start..newest.end)
            .collect::<String>()
    };

    Some(SelectionPayload {
        text,
        file_path,
        file_url,
        selection: SelectionRange {
            start: Position {
                line: newest.start.row,
                character: newest.start.column,
            },
            end: Position {
                line: newest.end.row,
                character: newest.end.column,
            },
        },
    })
}

fn compute_workspace_folders(workspace: &WeakEntity<Workspace>, cx: &mut App) -> Option<Value> {
    let workspace = workspace.upgrade()?;
    let project = workspace.read(cx).project().clone();
    let paths: Vec<(String, String)> = project
        .read(cx)
        .visible_worktrees(cx)
        .map(|worktree| {
            let worktree = worktree.read(cx);
            (
                worktree.root_name().as_unix_str().to_string(),
                worktree.abs_path().to_string_lossy().to_string(),
            )
        })
        .collect();
    let root_path = paths.first().map(|(_, path)| path.clone());
    let folders: Vec<Value> = paths
        .iter()
        .map(|(name, path)| json!({ "name": name, "uri": format!("file://{path}"), "path": path }))
        .collect();
    Some(json!({ "folders": folders, "rootPath": root_path }))
}

/// Convenience for the bridge subscription: recompute the editor selection,
/// update the editor slot, and return the AGGREGATE (editor plus every external
/// source) to broadcast. So an editor selection change re-broadcasts every
/// source's current selection together, not the editor's in isolation.
pub fn capture(
    selection: &SharedSelection,
    workspace: &Entity<Workspace>,
    cx: &mut App,
) -> Option<SelectionPayload> {
    let payload = compute_active_selection(&workspace.downgrade(), cx);
    selection.set_editor(payload);
    selection.latest()
}

#[cfg(test)]
mod aggregate_tests {
    use super::*;

    /// Upsert with no known owner. These tests are about aggregation, so they opt
    /// out of liveness: an unknown owner is never reaped.
    fn upsert(selection: &SharedSelection, source: &str, payload: SelectionPayload) {
        selection.upsert_external(source.to_string(), payload, None);
    }

    fn payload(path: &str, start: u32, end: u32, text: &str) -> SelectionPayload {
        SelectionPayload {
            text: text.to_string(),
            file_path: path.to_string(),
            file_url: format!("file://{path}"),
            selection: SelectionRange {
                start: Position {
                    line: start,
                    character: 0,
                },
                end: Position {
                    line: end,
                    character: 0,
                },
            },
        }
    }

    /// Removing one source leaves the others, removing an absent one reports false,
    /// and removing the last empties the aggregate so the caller knows to send a
    /// cleared notification rather than nothing. The sweep drops everything at once.
    #[test]
    fn removal_drops_one_source_or_all_and_can_empty_the_aggregate() {
        let selection = SharedSelection::new();
        upsert(&selection, "pid:1", payload("/a.rs", 0, 0, "alpha"));
        upsert(&selection, "pid:2", payload("/b.rs", 4, 5, "bravo"));

        assert!(selection.remove_external("pid:1"), "removing a present source");
        let text = selection.latest().expect("one source remains").text;
        log::info!("observed remaining text: {text}");
        assert!(text.contains("bravo") && !text.contains("alpha"), "got {text}");
        assert!(!selection.remove_external("pid:1"), "removing an absent source");

        assert!(selection.remove_external("pid:2"), "removes the last source");
        assert!(selection.latest().is_none(), "aggregate must be empty");

        upsert(&selection, "a", payload("/a.rs", 0, 0, "alpha"));
        upsert(&selection, "b", payload("/b.rs", 0, 0, "bravo"));
        assert_eq!(selection.remove_all_external(), 2, "sweep drops both");
        assert!(selection.latest().is_none(), "everything should be gone");
        assert_eq!(selection.remove_all_external(), 0, "a second sweep finds nothing");
    }

    /// A region whose owner has died disappears on the next read, while a live owner
    /// survives and an unknown owner is kept (not knowing who owns a region is not
    /// evidence that it is stale). This is the fix for a client killed without
    /// retracting: its key names a dead process, so nothing would ever replace it.
    #[test]
    fn regions_are_reaped_only_when_their_owner_is_known_dead() {
        let selection = SharedSelection::new();

        // A genuinely dead pid: spawn, reap, reuse its id. More honest than picking a
        // number and hoping. smol's Command because the tree disallows the blocking one.
        let dead_pid = smol::block_on(async {
            let mut child = smol::process::Command::new("true")
                .spawn()
                .expect("spawn a short-lived child");
            let pid = child.id();
            child.status().await.expect("reap the child");
            pid
        });

        let region = |name: &str, text: &str| payload(&format!("/{name}.rs"), 0, 0, text);
        selection.upsert_external("dead".into(), region("gone", "from a dead"), Some(dead_pid));
        selection.upsert_external(
            "live".into(),
            region("live", "from a live"),
            Some(std::process::id()),
        );
        upsert(&selection, "unknown", region("x", "from an unknown"));

        let text = selection.latest().expect("survivors remain").text;
        log::info!("observed aggregate after reaping: {text}");
        assert!(text.contains("from a live"), "live owner survives, got {text}");
        assert!(text.contains("from an unknown"), "unknown owner kept, got {text}");
        assert!(!text.contains("from a dead"), "dead owner reaped, got {text}");
    }

    /// `kill` reads 0 as "my process group" and negatives as a group id, so an owner
    /// pid of 0 must never reach it: it would answer "alive" for our own group and
    /// pin a region forever.
    #[test]
    fn pid_zero_is_never_treated_as_alive() {
        assert!(!process_is_alive(0), "pid 0 must not read as a live owner");
        assert!(process_is_alive(std::process::id()), "our own pid is alive");
    }

    #[test]
    fn no_sources_is_empty() {
        let selection = SharedSelection::new();
        assert!(selection.latest().is_none(), "nothing selected -> None");
    }

    #[test]
    fn single_source_passes_through_verbatim() {
        // One region: the payload is returned unchanged, so the CLI's
        // single-region banner ("N lines from <file>") stays clean.
        let selection = SharedSelection::new();
        upsert(&selection, "terminal:1", payload("/repo/a.rs", 0, 2, "fn a() {}"));
        let got = selection.latest().expect("one source -> Some");
        assert_eq!(got.file_path, "/repo/a.rs");
        assert_eq!(got.text, "fn a() {}");
        assert_eq!(got.selection.start.line, 0);
        assert_eq!(got.selection.end.line, 2);
    }

    #[test]
    fn two_sources_aggregate_with_headers_and_truthful_primary() {
        // Two nvims in two terminals: BOTH selections ride in `text` under
        // `# path:startLine-endLine` headers, and the top-level fields stay
        // truthful to the most-recently-updated (primary) region.
        let selection = SharedSelection::new();
        upsert(&selection, "terminal:1", payload("/repo/a.rs", 0, 0, "alpha"));
        upsert(&selection, "terminal:2", payload("/repo/b.rs", 4, 5, "bravo"));

        let got = selection.latest().expect("two sources -> Some");
        // Primary = the last upsert (b.rs), so the banner names it truthfully.
        assert_eq!(got.file_path, "/repo/b.rs", "primary is the latest source");
        assert_eq!(got.selection.start.line, 4);
        // `text` carries BOTH regions, each under its own header (1-based lines).
        assert!(got.text.contains("# /repo/a.rs:1-1\nalpha"), "text: {}", got.text);
        assert!(got.text.contains("# /repo/b.rs:5-6\nbravo"), "text: {}", got.text);
    }

    #[test]
    fn reselecting_same_source_replaces_not_accumulates() {
        // Re-selecting in the SAME nvim (same source key) replaces its region;
        // the aggregate stays a single region, not two stale copies.
        let selection = SharedSelection::new();
        upsert(&selection, "terminal:1", payload("/repo/a.rs", 0, 0, "first"));
        upsert(&selection, "terminal:1", payload("/repo/a.rs", 9, 9, "second"));

        let got = selection.latest().expect("still Some");
        assert_eq!(got.text, "second", "same source replaces its own region");
        assert_eq!(got.selection.start.line, 9);
    }

    #[test]
    fn editor_and_external_aggregate_together() {
        // The Zed editor selection and an external nvim selection combine: both
        // appear, editor first (it is pushed into the region list first).
        let selection = SharedSelection::new();
        selection.set_editor(Some(payload("/repo/editor.rs", 1, 1, "in-editor")));
        upsert(&selection, "terminal:7", payload("/repo/nvim.rs", 3, 3, "in-nvim"));

        let got = selection.latest().expect("two regions -> Some");
        assert!(got.text.contains("# /repo/editor.rs:2-2\nin-editor"), "text: {}", got.text);
        assert!(got.text.contains("# /repo/nvim.rs:4-4\nin-nvim"), "text: {}", got.text);
        // Primary is the external source (set last).
        assert_eq!(got.file_path, "/repo/nvim.rs");
    }

    #[test]
    fn clearing_editor_leaves_external_sources() {
        // Setting the editor to None must not wipe external nvim selections.
        let selection = SharedSelection::new();
        selection.set_editor(Some(payload("/repo/editor.rs", 0, 0, "e")));
        upsert(&selection, "terminal:1", payload("/repo/n.rs", 0, 0, "n"));
        selection.set_editor(None);

        let got = selection.latest().expect("external survives editor clear");
        assert_eq!(got.file_path, "/repo/n.rs");
        assert_eq!(got.text, "n", "only the external region remains");
    }
}
