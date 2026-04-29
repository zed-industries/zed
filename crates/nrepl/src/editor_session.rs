//! Per-editor nREPL session: drives eval / load-file / interrupt and
//! owns the inline result blocks for one editor.
//!
//! Mirrors the `EditorBlock` lifecycle from `crates/repl/src/session.rs`
//! but trimmed down to what nREPL needs:
//!
//! - There is no kernel to start; the workspace's [`NreplConnection`]
//!   already owns the live TCP socket. Each eval just calls
//!   [`NreplClient::send`] and pumps the resulting stream into an
//!   [`OutputView`].
//! - Replies are byte-string streams (`:value`/`:out`/`:err`/`:ex`),
//!   terminated by a status containing `"done"`. We don't deal with
//!   Jupyter-style MIME bundles.
//! - Each editor owns at most one in-flight eval id at a time as far as
//!   `nrepl::Interrupt` is concerned. The user can submit several evals
//!   in flight (each gets its own block); Interrupt cancels the most
//!   recent one — that matches what CIDER does and avoids piling up
//!   per-block "interrupt this one" buttons in the MVP.
//!
//! The actual top-level functions wired to actions live near the
//! bottom: [`eval_form_at_cursor`], [`eval_selection`], [`load_file`],
//! [`interrupt`], [`switch_namespace`], [`clear_outputs`].

use std::{ops::Range, sync::Arc};

use anyhow::{Context as _, Result, anyhow};
use collections::{HashMap, HashSet};
use editor::{
    Anchor, AnchorRangeExt as _, Editor, Inlay, MultiBuffer, ToPoint as _,
    display_map::{
        BlockContext, BlockId, BlockPlacement, BlockProperties, BlockStyle, CustomBlockId,
        RenderBlock,
    },
};
use futures::StreamExt as _;
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, EntityId, Global, SharedString, Subscription,
    Task, WeakEntity, Window, prelude::*,
};
use language::Point;
use multi_buffer::{MultiBufferOffset, ToOffset};
use project::InlayId;
use theme::ActiveTheme;
use ui::{IconButton, IconButtonShape, IconName, IconSize, Tooltip, prelude::*};
use util::ResultExt as _;
use workspace::{Toast, Workspace, notifications::NotificationId};

use crate::client::{NreplClient, RequestStream};
use crate::form_at_cursor::{TopLevelForm, parse_namespace, top_level_form_at_offset};
use crate::nrepl_settings::NreplSettings;
use crate::nrepl_store::{ConnectionState, NreplStore};
use crate::output_view::{
    OutputChunk, OutputStatus, OutputView, OutputViewFinishedEmpty, OutputViewFinishedSmall,
    fail_in_flight,
};
use crate::{Value, dict};

/// Marker type for nREPL gutter highlights. Using a dedicated marker
/// (rather than borrowing `crates/repl/`'s) keeps the two crates'
/// gutter-highlight maps disjoint, so disabling one doesn't disturb the
/// other.
enum NreplExecutedRange {}

/// Default namespace when the buffer has no `(ns ...)` form. nREPL
/// servers also default to `user`, so this matches the server's behavior.
const DEFAULT_NAMESPACE: &str = "user";

/// Hard cap on the size of a `load-file` payload. The protocol has no
/// limit, but pushing tens of MB of source through a REPL is almost
/// always a mistake — and our 16 MiB frame cap on the read side would
/// trip first anyway.
const MAX_LOAD_FILE_BYTES: usize = 4 * 1024 * 1024;

// =====================================================================
// Module-private editor-session registry.
//
// Kept here (rather than on `NreplStore`) so the connection lifecycle
// in `nrepl_store.rs` stays decoupled from the per-editor block
// bookkeeping that's specific to PR4.
// =====================================================================

struct GlobalEditorSessions(Entity<EditorSessions>);
impl Global for GlobalEditorSessions {}

struct EditorSessions {
    sessions: HashMap<EntityId, Entity<NreplEditorSession>>,
}

impl EditorSessions {
    fn init(cx: &mut App) {
        let entity = cx.new(|_| Self {
            sessions: HashMap::default(),
        });
        cx.set_global(GlobalEditorSessions(entity));
    }

    fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalEditorSessions>().0.clone()
    }
}

pub fn init(cx: &mut App) {
    EditorSessions::init(cx);
}

fn editor_session(
    editor: &WeakEntity<Editor>,
    create: bool,
    cx: &mut App,
) -> Option<Entity<NreplEditorSession>> {
    let editor_entity = editor.upgrade()?;
    let entity_id = editor.entity_id();
    let sessions = EditorSessions::global(cx);
    if let Some(existing) = sessions.read(cx).sessions.get(&entity_id).cloned() {
        return Some(existing);
    }
    if !create {
        return None;
    }
    let workspace = editor_entity.read(cx).workspace()?.downgrade();
    let session = cx.new(|cx| NreplEditorSession::new(editor.clone(), workspace, cx));
    sessions.update(cx, |sessions, _cx| {
        sessions.sessions.insert(entity_id, session.clone());
    });
    Some(session)
}

/// Drops the per-editor session for `editor`. Safe to call when no
/// session exists. Intended for the editor's drop hook so we don't leak
/// blocks/inlays bookkeeping on closed editors.
pub fn forget_editor(editor: &WeakEntity<Editor>, cx: &mut App) {
    let entity_id = editor.entity_id();
    EditorSessions::global(cx).update(cx, |sessions, _cx| {
        sessions.sessions.remove(&entity_id);
    });
}

// =====================================================================
// NreplEditorSession
// =====================================================================

pub struct NreplEditorSession {
    editor: WeakEntity<Editor>,
    workspace: WeakEntity<Workspace>,
    /// Cached `(ns ...)` form. Refreshed on session creation, on every
    /// eval, and explicitly by [`switch_namespace`]. We don't subscribe
    /// to buffer-saved events because re-parsing on each eval is cheap
    /// (one tree-sitter walk over top-level children) and avoids
    /// surprises when the user edits the `(ns ...)` form without saving.
    namespace: Option<String>,
    blocks: HashMap<String, EditorBlock>,
    /// `request_id -> (inlay_id, original_code_range, original_code_len)`.
    /// Mirrors the same shape as `crates/repl/`'s `result_inlays` so the
    /// invalidation logic in `on_buffer_event` stays straightforward.
    result_inlays: HashMap<String, (InlayId, Range<Anchor>, usize)>,
    next_inlay_id: usize,
    /// Most recent in-flight request id, used by `Interrupt`. We track
    /// only the latest because Interrupt's UX is "stop what I'm running
    /// now"; for finer-grained control the user clicks the per-block
    /// close button.
    last_in_flight: Option<String>,
    _subscriptions: Vec<Subscription>,
}

struct EditorBlock {
    code_range: Range<Anchor>,
    invalidation_anchor: Anchor,
    block_id: CustomBlockId,
    /// Held so the rendered output entity stays alive for as long as
    /// the block is in the editor. The block's `RenderBlock` closure
    /// also holds a clone, but that clone disappears when the block is
    /// removed; this one keeps the entity around long enough for our
    /// own bookkeeping (e.g. close-button-driven removal) to run.
    _output_view: Entity<OutputView>,
    /// Keeping the stream-pump task here means dropping the block (via
    /// invalidation, on_close, or `clear_outputs`) cancels the in-flight
    /// reply forwarding too — so we don't keep updating an entity whose
    /// block is already gone from the editor.
    _stream_task: Task<()>,
}

type CloseBlockFn =
    Arc<dyn for<'a> Fn(CustomBlockId, &'a mut Window, &mut App) + Send + Sync + 'static>;

impl NreplEditorSession {
    fn new(
        editor: WeakEntity<Editor>,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        let buffer_subscription = match editor.upgrade() {
            Some(editor_entity) => {
                let buffer = editor_entity.read(cx).buffer().clone();
                cx.subscribe(&buffer, Self::on_buffer_event)
            }
            None => Subscription::new(|| {}),
        };

        let mut session = Self {
            editor,
            workspace,
            namespace: None,
            blocks: HashMap::default(),
            result_inlays: HashMap::default(),
            next_inlay_id: 0,
            last_in_flight: None,
            _subscriptions: vec![buffer_subscription],
        };
        session.refresh_namespace(cx);
        session
    }

    pub fn namespace(&self) -> &str {
        self.namespace.as_deref().unwrap_or(DEFAULT_NAMESPACE)
    }

    /// Re-parses the buffer's `(ns ...)` form and caches it. No-op if the
    /// editor has been dropped or backs more than a single buffer (we
    /// don't try to attribute a namespace to multi-buffer views).
    pub fn refresh_namespace(&mut self, cx: &mut Context<Self>) {
        let Some(editor) = self.editor.upgrade() else {
            return;
        };
        let multibuffer = editor.read(cx).buffer().clone();
        let Some(buffer) = multibuffer.read(cx).as_singleton() else {
            return;
        };
        let snapshot = buffer.read(cx).snapshot();
        self.namespace = parse_namespace(&snapshot);
    }

    fn on_buffer_event(
        &mut self,
        buffer: Entity<MultiBuffer>,
        event: &multi_buffer::Event,
        cx: &mut Context<Self>,
    ) {
        let multi_buffer::Event::Edited { .. } = event else {
            return;
        };
        let snapshot = buffer.read(cx).snapshot(cx);

        let mut blocks_to_remove: HashSet<CustomBlockId> = HashSet::default();
        let mut gutter_ranges_to_remove: Vec<Range<Anchor>> = Vec::new();

        self.blocks.retain(|_id, block| {
            if block.invalidation_anchor.is_valid(&snapshot) {
                true
            } else {
                blocks_to_remove.insert(block.block_id);
                gutter_ranges_to_remove.push(block.code_range.clone());
                false
            }
        });

        let mut inlays_to_remove: Vec<InlayId> = Vec::new();
        self.result_inlays
            .retain(|_id, (inlay_id, code_range, original_len)| {
                let start_offset = code_range.start.to_offset(&snapshot);
                let end_offset = code_range.end.to_offset(&snapshot);
                let current_len = end_offset.saturating_sub(start_offset);
                if current_len != *original_len {
                    inlays_to_remove.push(*inlay_id);
                    gutter_ranges_to_remove.push(code_range.clone());
                    false
                } else {
                    true
                }
            });

        if blocks_to_remove.is_empty()
            && inlays_to_remove.is_empty()
            && gutter_ranges_to_remove.is_empty()
        {
            return;
        }

        self.editor
            .update(cx, |editor, cx| {
                if !blocks_to_remove.is_empty() {
                    editor.remove_blocks(blocks_to_remove, None, cx);
                }
                if !inlays_to_remove.is_empty() {
                    editor.splice_inlays(&inlays_to_remove, vec![], cx);
                }
                if !gutter_ranges_to_remove.is_empty() {
                    editor.remove_gutter_highlights::<NreplExecutedRange>(
                        gutter_ranges_to_remove,
                        cx,
                    );
                }
            })
            .ok();
        cx.notify();
    }

    fn dispatch_eval(
        &mut self,
        code: String,
        anchor_range: Range<Anchor>,
        cx: &mut Context<Self>,
    ) -> Result<String> {
        let (client, session_id) = self.connection_handles(cx)?;
        let namespace = self.namespace().to_string();
        let request = dict([
            ("op", Value::str("eval")),
            ("code", Value::str(code)),
            ("ns", Value::str(namespace)),
            ("session", Value::str(session_id)),
        ]);
        let stream = client.send(request)?;
        let request_id = stream.id().to_string();
        self.install_block(request_id.clone(), anchor_range, stream, cx)?;
        Ok(request_id)
    }

    fn dispatch_load_file(
        &mut self,
        file_contents: String,
        file_path: Option<String>,
        file_name: Option<String>,
        anchor_range: Range<Anchor>,
        cx: &mut Context<Self>,
    ) -> Result<String> {
        let (client, session_id) = self.connection_handles(cx)?;
        let mut entries: Vec<(&str, Value)> = vec![
            ("op", Value::str("load-file")),
            ("file", Value::str(file_contents)),
            ("session", Value::str(session_id)),
        ];
        if let Some(p) = file_path {
            entries.push(("file-path", Value::str(p)));
        }
        if let Some(n) = file_name {
            entries.push(("file-name", Value::str(n)));
        }
        let stream = client.send(dict(entries))?;
        let request_id = stream.id().to_string();
        self.install_block(request_id.clone(), anchor_range, stream, cx)?;
        Ok(request_id)
    }

    fn install_block(
        &mut self,
        request_id: String,
        anchor_range: Range<Anchor>,
        stream: RequestStream,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let editor = self.editor.upgrade().context("editor was dropped")?;

        // Replace any prior block / inlay overlapping the new range so a
        // re-eval visually supersedes the previous result rather than
        // stacking blocks on top of each other.
        self.evict_overlapping(&editor, &anchor_range, cx);

        let output_view = cx.new(|cx| OutputView::new(OutputStatus::Queued, cx));

        let session_view = cx.entity().downgrade();
        let weak_editor = self.editor.clone();
        let request_key = request_id.clone();
        let code_range_for_close = anchor_range.clone();
        let on_close: CloseBlockFn = Arc::new(
            move |closing_block_id: CustomBlockId, _window: &mut Window, cx: &mut App| {
                if let Some(session) = session_view.upgrade() {
                    session.update(cx, |session, cx| {
                        session.blocks.remove(&request_key);
                        cx.notify();
                    });
                }
                if let Some(editor) = weak_editor.upgrade() {
                    editor.update(cx, |editor, cx| {
                        let mut block_ids = HashSet::default();
                        block_ids.insert(closing_block_id);
                        editor.remove_blocks(block_ids, None, cx);
                        editor.remove_gutter_highlights::<NreplExecutedRange>(
                            vec![code_range_for_close.clone()],
                            cx,
                        );
                    });
                }
            },
        );

        let (block_id, invalidation_anchor) = editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().clone();
            let buffer_snapshot = buffer.read(cx).snapshot(cx);
            let end_point = anchor_range.end.to_point(&buffer_snapshot);
            let next_row_start = end_point + Point::new(1, 0);
            // The block lives below the form. If the form is on the very
            // last line we need to insert a newline first; otherwise the
            // block has nothing to anchor against and the placement
            // anchor immediately becomes invalid.
            if next_row_start > buffer_snapshot.max_point() {
                buffer.update(cx, |buffer, cx| {
                    buffer.edit(
                        [(
                            buffer_snapshot.max_point()..buffer_snapshot.max_point(),
                            "\n",
                        )],
                        None,
                        cx,
                    )
                });
            }
            let buffer_snapshot = buffer.read(cx).snapshot(cx);
            let block_placement_anchor = buffer_snapshot.anchor_before(end_point);
            let invalidation_anchor = buffer_snapshot.anchor_before(next_row_start);
            let block = BlockProperties {
                placement: BlockPlacement::Below(block_placement_anchor),
                height: Some(1),
                style: BlockStyle::Sticky,
                render: create_renderer(output_view.clone(), on_close.clone()),
                priority: 0,
            };
            let block_id = editor.insert_blocks([block], None, cx)[0];
            (block_id, invalidation_anchor)
        });

        editor.update(cx, |editor, cx| {
            editor.insert_gutter_highlight::<NreplExecutedRange>(
                anchor_range.clone(),
                |cx| cx.theme().status().success,
                cx,
            );
        });

        // Collapse-on-finish subscriptions. These mirror the Jupyter side:
        // an empty completion drops the block entirely (gutter highlight
        // stays) and a small single-line value collapses to an inlay.
        {
            let key = request_id.clone();
            let sub = cx.subscribe(
                &output_view,
                move |session, _view, _ev: &OutputViewFinishedEmpty, cx| {
                    session.replace_block_with_inlay(&key, "", cx);
                },
            );
            self._subscriptions.push(sub);
        }
        {
            let key = request_id.clone();
            let sub = cx.subscribe(
                &output_view,
                move |session, _view, ev: &OutputViewFinishedSmall, cx| {
                    let text = ev.0.clone();
                    session.replace_block_with_inlay(&key, &text, cx);
                },
            );
            self._subscriptions.push(sub);
        }

        let stream_task = cx.spawn({
            let view = output_view.downgrade();
            let key = request_id.clone();
            async move |this, cx| {
                pump_stream(stream, view, cx).await;
                this.update(cx, |session, _cx| {
                    if session.last_in_flight.as_deref() == Some(key.as_str()) {
                        session.last_in_flight = None;
                    }
                })
                .ok();
            }
        });

        self.last_in_flight = Some(request_id.clone());
        self.blocks.insert(
            request_id,
            EditorBlock {
                code_range: anchor_range,
                invalidation_anchor,
                block_id,
                _output_view: output_view,
                _stream_task: stream_task,
            },
        );
        cx.notify();
        Ok(())
    }

    fn evict_overlapping(
        &mut self,
        editor: &Entity<Editor>,
        anchor_range: &Range<Anchor>,
        cx: &mut Context<Self>,
    ) {
        let buffer = editor.read(cx).buffer().read(cx).snapshot(cx);
        let mut blocks_to_remove: HashSet<CustomBlockId> = HashSet::default();
        let mut inlays_to_remove: Vec<InlayId> = Vec::new();
        let mut gutter_ranges_to_remove: Vec<Range<Anchor>> = Vec::new();

        self.blocks.retain(|_key, block| {
            if anchor_range.overlaps(&block.code_range, &buffer) {
                blocks_to_remove.insert(block.block_id);
                false
            } else {
                true
            }
        });
        self.result_inlays
            .retain(|_key, (inlay_id, inlay_range, _)| {
                if anchor_range.overlaps(inlay_range, &buffer) {
                    inlays_to_remove.push(*inlay_id);
                    gutter_ranges_to_remove.push(inlay_range.clone());
                    false
                } else {
                    true
                }
            });

        if blocks_to_remove.is_empty()
            && inlays_to_remove.is_empty()
            && gutter_ranges_to_remove.is_empty()
        {
            return;
        }

        editor.update(cx, |editor, cx| {
            if !blocks_to_remove.is_empty() {
                editor.remove_blocks(blocks_to_remove, None, cx);
            }
            if !inlays_to_remove.is_empty() {
                editor.splice_inlays(&inlays_to_remove, vec![], cx);
            }
            if !gutter_ranges_to_remove.is_empty() {
                editor.remove_gutter_highlights::<NreplExecutedRange>(gutter_ranges_to_remove, cx);
            }
        });
    }

    fn replace_block_with_inlay(&mut self, request_id: &str, text: &str, cx: &mut Context<Self>) {
        let Some(block) = self.blocks.remove(request_id) else {
            return;
        };
        let Some(editor) = self.editor.upgrade() else {
            return;
        };
        let code_range = block.code_range.clone();

        editor.update(cx, |editor, cx| {
            let mut block_ids = HashSet::default();
            block_ids.insert(block.block_id);
            editor.remove_blocks(block_ids, None, cx);

            // Empty path: nothing to render as an inlay. The gutter
            // highlight stays so the user still sees that the line was
            // evaluated successfully.
            if text.is_empty() {
                return;
            }

            let buffer = editor.buffer().read(cx).snapshot(cx);
            let start_offset = code_range.start.to_offset(&buffer);
            let end_offset = code_range.end.to_offset(&buffer);
            let original_len = end_offset.saturating_sub(start_offset);

            let end_point = code_range.end.to_point(&buffer);
            let inlay_position = buffer.anchor_after(end_point);

            let inlay_id = self.next_inlay_id;
            self.next_inlay_id += 1;

            let inlay = Inlay::repl_result(inlay_id, inlay_position, format!("    {text}"));
            editor.splice_inlays(&[], vec![inlay], cx);
            self.result_inlays.insert(
                request_id.to_string(),
                (
                    InlayId::ReplResult(inlay_id),
                    code_range.clone(),
                    original_len,
                ),
            );
        });
        cx.notify();
    }

    pub fn clear_outputs(&mut self, cx: &mut Context<Self>) {
        let blocks_to_remove: HashSet<CustomBlockId> =
            self.blocks.values().map(|b| b.block_id).collect();
        let inlays_to_remove: Vec<InlayId> =
            self.result_inlays.values().map(|(id, _, _)| *id).collect();

        self.editor
            .update(cx, |editor, cx| {
                editor.remove_blocks(blocks_to_remove, None, cx);
                editor.splice_inlays(&inlays_to_remove, vec![], cx);
                editor.clear_gutter_highlights::<NreplExecutedRange>(cx);
            })
            .ok();
        self.blocks.clear();
        self.result_inlays.clear();
        cx.notify();
    }

    /// Resolves the workspace's nREPL connection into a `(client,
    /// session_id)` pair, returning a meaningful error if not yet
    /// connected. The returned client is a cheap clone — sharing the
    /// same TCP socket and request multiplexer.
    fn connection_handles(&self, cx: &App) -> Result<(NreplClient, String)> {
        let workspace = self.workspace.upgrade().context("workspace dropped")?;
        let workspace_id = workspace.entity_id();
        let store = NreplStore::global(cx);
        let conn = store
            .read(cx)
            .connection_for_workspace(workspace_id)
            .cloned()
            .ok_or_else(|| anyhow!("not connected; run `nrepl::Connect` first"))?;
        match conn.read(cx).state() {
            ConnectionState::Connected {
                client, session, ..
            } => Ok((client.clone(), session.clone())),
            ConnectionState::Resolving | ConnectionState::Connecting { .. } => Err(anyhow!(
                "nREPL connection is still establishing — try again in a moment"
            )),
            ConnectionState::Failed { error } => Err(anyhow!("nREPL connect failed: {error}")),
        }
    }

    pub fn interrupt(&mut self, cx: &mut Context<Self>) -> Result<()> {
        let Some(request_id) = self.last_in_flight.clone() else {
            return Ok(());
        };
        let (client, session_id) = self.connection_handles(cx)?;
        // The interrupt's own reply isn't surfaced — what the user sees
        // is the `:status ["interrupted" "done"]` arriving on the
        // *original* eval's stream, which our pump handles. We still
        // consume the interrupt reply so the request map cleans up.
        let mut stream = client.interrupt(&session_id, &request_id)?;
        cx.background_spawn(async move { while stream.next().await.is_some() {} })
            .detach();
        Ok(())
    }

    pub fn workspace(&self) -> &WeakEntity<Workspace> {
        &self.workspace
    }
}

async fn pump_stream(mut stream: RequestStream, view: WeakEntity<OutputView>, cx: &mut AsyncApp) {
    let mut saw_done = false;
    loop {
        let Some(msg) = stream.next().await else {
            break;
        };

        let value_chunk = msg.get("value").and_then(Value::as_str).map(str::to_string);
        let stdout_chunk = msg.get("out").and_then(Value::as_str).map(str::to_string);
        let stderr_chunk = msg.get("err").and_then(Value::as_str).map(str::to_string);
        let exception_chunk = msg
            .get("ex")
            .and_then(Value::as_str)
            .or_else(|| msg.get("root-ex").and_then(Value::as_str))
            .map(str::to_string);
        let status_items: Vec<String> = msg
            .get("status")
            .and_then(Value::as_list)
            .map(|items| {
                items
                    .iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();

        let is_done = status_items.iter().any(|s| s == "done");

        let update_result = view.update(cx, |view, cx| {
            if let Some(s) = value_chunk {
                view.push_value(s, cx);
            }
            if let Some(s) = stdout_chunk {
                view.push_stdout(s, cx);
            }
            if let Some(s) = stderr_chunk {
                view.push_stderr(s, cx);
            }
            if let Some(s) = exception_chunk {
                view.push_exception(s, cx);
            }
            for status in &status_items {
                match status.as_str() {
                    "done" => {}
                    "interrupted" => {
                        if !matches!(view.status(), OutputStatus::Failed(_)) {
                            view.set_status(OutputStatus::Interrupted, cx);
                        }
                    }
                    "namespace-not-found" => {
                        view.set_status(OutputStatus::Failed("namespace not found".into()), cx);
                    }
                    "eval-error" => {
                        // The actual error text shows up via `:ex` /
                        // `:err`. Pull the most recent exception summary
                        // we've observed, falling back to the bare label.
                        let label: SharedString = view
                            .chunks()
                            .iter()
                            .rev()
                            .find_map(|c| match c {
                                OutputChunk::Exception(s) => {
                                    Some(s.lines().next().unwrap_or("eval-error").to_string())
                                }
                                _ => None,
                            })
                            .unwrap_or_else(|| "eval-error".to_string())
                            .into();
                        view.set_status(OutputStatus::Failed(label), cx);
                    }
                    _ => {}
                }
            }
        });

        if update_result.is_err() {
            // OutputView entity dropped (block closed). No reason to keep
            // pumping, but let the stream drop naturally so the client's
            // pending map entry gets cleared.
            return;
        }

        if is_done {
            saw_done = true;
            break;
        }
    }

    let _ = view.update(cx, |view, cx| {
        if saw_done {
            // Don't trample a Failed/Interrupted status that arrived
            // alongside `done` (some middleware reports both in the same
            // message; status order is otherwise unspecified).
            if !matches!(
                view.status(),
                OutputStatus::Failed(_) | OutputStatus::Interrupted
            ) {
                view.set_status(OutputStatus::Finished, cx);
            }
        } else {
            fail_in_flight(view, "connection closed before reply finished", cx);
        }
    });
}

fn create_renderer(view: Entity<OutputView>, on_close: CloseBlockFn) -> RenderBlock {
    Arc::new(move |cx: &mut BlockContext| {
        let view = view.clone();
        let on_close = on_close.clone();
        let block_id = cx.block_id;

        let close_button = IconButton::new("nrepl-close-output", IconName::Close)
            .icon_size(IconSize::Small)
            .icon_color(Color::Muted)
            .shape(IconButtonShape::Square)
            .tooltip(Tooltip::text("Close output"))
            .on_click(move |_event, window, cx| {
                if let BlockId::Custom(block_id) = block_id {
                    (on_close)(block_id, window, cx);
                }
            });

        h_flex()
            .id(cx.block_id)
            .w_full()
            .border_y_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().background)
            .child(div().pl_2().pr_1().pt_1().child(close_button))
            .child(div().flex_1().overflow_hidden().child(view))
            .into_any_element()
    })
}

// =====================================================================
// Action entry points. The `nrepl` actions in `nrepl_sessions_ui`
// dispatch through these.
//
// All entries follow the same shape: bail if the feature is disabled,
// look up (or lazily create) the per-editor session, snapshot what we
// need from the editor on the foreground, then ask the session to
// dispatch. Errors propagate back to the caller for toast/log handling.
// =====================================================================

pub fn eval_form_at_cursor(
    editor: WeakEntity<Editor>,
    _window: &mut Window,
    cx: &mut App,
) -> Result<()> {
    if !NreplSettings::enabled(cx) {
        return Ok(());
    }
    let session = editor_session(&editor, true, cx)
        .ok_or_else(|| anyhow!("editor has no associated workspace"))?;
    let editor_entity = editor.upgrade().context("editor was dropped")?;

    let (form_text, anchor_range) = editor_entity.update(cx, |editor, cx| -> Result<_> {
        let multibuffer = editor.buffer().clone();
        let buffer = multibuffer
            .read(cx)
            .as_singleton()
            .context("nREPL eval requires a single-buffer editor")?;
        let buffer_snapshot = buffer.read(cx).snapshot();
        let multibuffer_snapshot = multibuffer.read(cx).snapshot(cx);
        let head = editor.selections.newest_anchor().head();
        // Singleton multibuffer => MultiBufferOffset.0 maps 1-to-1 to the
        // underlying buffer's offsets. `top_level_form_at_offset` expects
        // buffer-internal `usize` offsets, so we unwrap the newtype here
        // and re-wrap the resulting range below for the multibuffer side.
        let MultiBufferOffset(offset) = head.to_offset(&multibuffer_snapshot);
        let form: TopLevelForm = top_level_form_at_offset(&buffer_snapshot, offset)
            .context("no top-level form under cursor")?;
        let anchor_range = multibuffer_snapshot.anchor_before(MultiBufferOffset(form.range.start))
            ..multibuffer_snapshot.anchor_after(MultiBufferOffset(form.range.end));
        Ok((form.text, anchor_range))
    })?;

    session.update(cx, |session, cx| {
        session.refresh_namespace(cx);
        session.dispatch_eval(form_text, anchor_range, cx)
    })?;
    Ok(())
}

pub fn eval_selection(
    editor: WeakEntity<Editor>,
    _window: &mut Window,
    cx: &mut App,
) -> Result<()> {
    if !NreplSettings::enabled(cx) {
        return Ok(());
    }
    let session = editor_session(&editor, true, cx)
        .ok_or_else(|| anyhow!("editor has no associated workspace"))?;
    let editor_entity = editor.upgrade().context("editor was dropped")?;

    let prepared = editor_entity.update(cx, |editor, cx| {
        let multibuffer = editor.buffer().clone();
        let display_snapshot = editor.display_snapshot(cx);
        let selection = editor
            .selections
            .newest::<MultiBufferOffset>(&display_snapshot);
        if selection.start == selection.end {
            return None;
        }
        let buffer_snapshot = multibuffer.read(cx).snapshot(cx);
        let text: String = buffer_snapshot
            .text_for_range(selection.start..selection.end)
            .collect();
        if text.trim().is_empty() {
            return None;
        }
        let range = buffer_snapshot.anchor_before(selection.start)
            ..buffer_snapshot.anchor_after(selection.end);
        Some((text, range))
    });

    let Some((text, anchor_range)) = prepared else {
        return Ok(());
    };

    session.update(cx, |session, cx| {
        session.refresh_namespace(cx);
        session.dispatch_eval(text, anchor_range, cx)
    })?;
    Ok(())
}

pub fn load_file(editor: WeakEntity<Editor>, _window: &mut Window, cx: &mut App) -> Result<()> {
    if !NreplSettings::enabled(cx) {
        return Ok(());
    }
    let session = editor_session(&editor, true, cx)
        .ok_or_else(|| anyhow!("editor has no associated workspace"))?;
    let editor_entity = editor.upgrade().context("editor was dropped")?;

    enum Prep {
        Dirty,
        Empty,
        Ready {
            contents: String,
            file_path: Option<String>,
            file_name: Option<String>,
            anchor_range: Range<Anchor>,
        },
    }

    let prep = editor_entity.update(cx, |editor, cx| -> Result<Prep> {
        let multibuffer = editor.buffer().clone();
        let buffer = multibuffer
            .read(cx)
            .as_singleton()
            .context("nREPL load-file requires a single-buffer editor")?;
        let buffer_read = buffer.read(cx);
        if buffer_read.is_dirty() {
            return Ok(Prep::Dirty);
        }
        let snapshot = multibuffer.read(cx).snapshot(cx);
        let len = snapshot.len();
        if len.0 == 0 {
            return Ok(Prep::Empty);
        }
        if len.0 > MAX_LOAD_FILE_BYTES {
            anyhow::bail!(
                "buffer is {} bytes; load-file refuses to send more than {} bytes",
                len.0,
                MAX_LOAD_FILE_BYTES,
            );
        }
        let contents: String = snapshot.text_for_range(MultiBufferOffset(0)..len).collect();
        let file = buffer_read.file();
        let file_path = file.map(|f| f.full_path(cx).to_string_lossy().into_owned());
        let file_name = file.and_then(|f| f.path().file_name().map(|n| n.to_string()));
        let anchor_range = snapshot.anchor_before(MultiBufferOffset(0))..snapshot.anchor_after(len);
        Ok(Prep::Ready {
            contents,
            file_path,
            file_name,
            anchor_range,
        })
    })?;

    match prep {
        Prep::Dirty => {
            toast(
                &session,
                "Save the buffer first; nREPL load-file needs the on-disk version.",
                cx,
            );
            Ok(())
        }
        Prep::Empty => Ok(()),
        Prep::Ready {
            contents,
            file_path,
            file_name,
            anchor_range,
        } => {
            session.update(cx, |session, cx| {
                session.dispatch_load_file(contents, file_path, file_name, anchor_range, cx)
            })?;
            Ok(())
        }
    }
}

pub fn interrupt(editor: WeakEntity<Editor>, cx: &mut App) -> Result<()> {
    if !NreplSettings::enabled(cx) {
        return Ok(());
    }
    let Some(session) = editor_session(&editor, false, cx) else {
        return Ok(());
    };
    session.update(cx, |session, cx| session.interrupt(cx))?;
    Ok(())
}

pub fn switch_namespace(editor: WeakEntity<Editor>, cx: &mut App) -> Result<()> {
    if !NreplSettings::enabled(cx) {
        return Ok(());
    }
    let session = editor_session(&editor, true, cx)
        .ok_or_else(|| anyhow!("editor has no associated workspace"))?;
    session.update(cx, |session, cx| session.refresh_namespace(cx));
    Ok(())
}

pub fn clear_outputs(editor: WeakEntity<Editor>, cx: &mut App) {
    if !NreplSettings::enabled(cx) {
        return;
    }
    let Some(session) = editor_session(&editor, false, cx) else {
        return;
    };
    session.update(cx, |session, cx| session.clear_outputs(cx));
}

/// Routes a one-shot informational toast through the editor's workspace.
/// Used for "save the buffer first"-style messages where we don't want
/// to stop the action with an `Err` (which would log-and-forget).
fn toast(session: &Entity<NreplEditorSession>, message: impl Into<SharedString>, cx: &mut App) {
    let workspace = session.read(cx).workspace().clone();
    let message = message.into().to_string();
    workspace
        .update(cx, |workspace, cx| {
            let id = NotificationId::unique::<NreplEditorToast>();
            workspace.show_toast(Toast::new(id, message).autohide(), cx);
        })
        .log_err();
}

struct NreplEditorToast;
