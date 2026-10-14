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
use crate::console::{NreplConsole, format_stacktrace, open_console};
use crate::form_at_cursor::{
    TopLevelForm, current_form_at_offset, ns_form_text, parse_namespace, symbol_at_offset,
    top_level_form_at_offset, top_level_form_to_cursor,
};
use crate::navigation::open_location;
use crate::nrepl_settings::NreplSettings;
use crate::nrepl_store::connected_client;
use crate::ops;
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
    ensured_namespaces: HashSet<String>,
    ensured_session_id: Option<String>,
    blocks: HashMap<String, EditorBlock>,
    /// `request_id -> (inlay_id, original_code_range, original_code_len)`.
    /// Mirrors the same shape as `crates/repl/`'s `result_inlays` so the
    /// invalidation logic in `on_buffer_event` stays straightforward.
    result_inlays: HashMap<String, (InlayId, Range<Anchor>, usize)>,
    next_inlay_id: usize,
    /// Request ids in submission order. `Interrupt` targets the newest
    /// request that is still active.
    in_flight: InFlightRequests,
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
    _subscriptions: Vec<Subscription>,
}

#[derive(Default)]
struct InFlightRequests {
    request_ids: Vec<String>,
}

impl InFlightRequests {
    fn insert(&mut self, request_id: String) {
        self.request_ids.push(request_id);
    }

    fn remove(&mut self, request_id: &str) {
        self.request_ids.retain(|candidate| candidate != request_id);
    }

    fn most_recent(&self) -> Option<&str> {
        self.request_ids.last().map(String::as_str)
    }

    fn is_empty(&self) -> bool {
        self.request_ids.is_empty()
    }

    fn clear(&mut self) {
        self.request_ids.clear();
    }
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
            ensured_namespaces: HashSet::default(),
            ensured_session_id: None,
            blocks: HashMap::default(),
            result_inlays: HashMap::default(),
            next_inlay_id: 0,
            in_flight: InFlightRequests::default(),
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

    pub fn ensure_namespace(
        &mut self,
        client: &NreplClient,
        session_id: &str,
        cx: &mut Context<Self>,
    ) {
        if self.ensured_session_id.as_deref() != Some(session_id) {
            self.ensured_namespaces.clear();
            self.ensured_session_id = Some(session_id.to_string());
        }

        let namespace = self.namespace().to_string();
        if namespace == DEFAULT_NAMESPACE || self.ensured_namespaces.contains(&namespace) {
            return;
        }

        let ns_form = self.editor.upgrade().and_then(|editor| {
            let multibuffer = editor.read(cx).buffer().clone();
            let buffer = multibuffer.read(cx).as_singleton()?;
            let snapshot = buffer.read(cx).snapshot();
            ns_form_text(&snapshot)
        });

        let code = match ns_form {
            Some(ns_form) => ns_form,
            None => format!("(clojure.core/in-ns '{namespace})"),
        };

        if let Some(stream) = client
            .send(ops::eval_request(&code, None, session_id))
            .log_err()
        {
            cx.background_spawn(ops::collect(stream)).detach();
            self.ensured_namespaces.insert(namespace);
        }
    }

    pub fn unensure_namespace(&mut self, namespace: &str) {
        self.ensured_namespaces.remove(namespace);
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

        let mut request_ids_to_remove = Vec::new();
        self.blocks.retain(|request_id, block| {
            if block.invalidation_anchor.is_valid(&snapshot) {
                true
            } else {
                request_ids_to_remove.push(request_id.clone());
                blocks_to_remove.insert(block.block_id);
                gutter_ranges_to_remove.push(block.code_range.clone());
                false
            }
        });
        for request_id in request_ids_to_remove {
            self.in_flight.remove(&request_id);
        }

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
            .log_err();
        cx.notify();
    }

    fn dispatch_eval(
        &mut self,
        code: String,
        anchor_range: Range<Anchor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<String> {
        let (client, session_id) = self.connection_handles(cx)?;
        self.ensure_namespace(&client, &session_id, cx);
        let namespace = self.namespace().to_string();
        let console = self.workspace.upgrade().map(|workspace| {
            let console = workspace.update(cx, |workspace, cx| {
                open_console(workspace, false, window, cx)
            });
            let prompt = format!("{}=> {}", namespace, code.trim());
            console.update(cx, |console, cx| {
                console.set_namespace(namespace.clone(), cx);
                console.append_line(&prompt, Some(window), cx);
            });
            console.downgrade()
        });

        let request = dict([
            ("op", Value::str("eval")),
            ("code", Value::str(code)),
            ("ns", Value::str(namespace)),
            ("session", Value::str(session_id.clone())),
        ]);
        let stream = client.send(request)?;
        let request_id = stream.id().to_string();
        self.install_block(
            request_id.clone(),
            anchor_range,
            stream,
            (client, session_id),
            console,
            cx,
        )?;
        Ok(request_id)
    }

    fn dispatch_load_file(
        &mut self,
        file_contents: String,
        file_path: Option<String>,
        file_name: Option<String>,
        anchor_range: Range<Anchor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<String> {
        let (client, session_id) = self.connection_handles(cx)?;
        let console = self.workspace.upgrade().map(|workspace| {
            let console = workspace.update(cx, |workspace, cx| {
                open_console(workspace, false, window, cx)
            });
            let name = file_name.as_deref().unwrap_or("buffer");
            let prompt = format!(";; load-file {name}");
            console.update(cx, |console, cx| {
                console.append_line(&prompt, Some(window), cx);
            });
            console.downgrade()
        });

        let mut entries: Vec<(&str, Value)> = vec![
            ("op", Value::str("load-file")),
            ("file", Value::str(file_contents)),
            ("session", Value::str(session_id.clone())),
        ];
        if let Some(p) = file_path {
            entries.push(("file-path", Value::str(p)));
        }
        if let Some(n) = file_name {
            entries.push(("file-name", Value::str(n)));
        }
        let stream = client.send(dict(entries))?;
        let request_id = stream.id().to_string();
        self.install_block(
            request_id.clone(),
            anchor_range,
            stream,
            (client, session_id),
            console,
            cx,
        )?;
        Ok(request_id)
    }

    fn install_block(
        &mut self,
        request_id: String,
        anchor_range: Range<Anchor>,
        stream: RequestStream,
        handles: (NreplClient, String),
        console: Option<WeakEntity<NreplConsole>>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let editor = self.editor.upgrade().context("editor was dropped")?;

        // Replace any prior block / inlay overlapping the new range so a
        // re-eval visually supersedes the previous result rather than
        // stacking blocks on top of each other.
        self.evict_overlapping(&editor, &anchor_range, cx);

        let output_view = cx.new(|cx| OutputView::new(OutputStatus::Queued, cx));
        let workspace = self.workspace.clone();
        output_view.update(cx, |view, _| view.set_workspace(workspace));

        let session_view = cx.entity().downgrade();
        let weak_editor = self.editor.clone();
        let request_key = request_id.clone();
        let code_range_for_close = anchor_range.clone();
        let on_close: CloseBlockFn = Arc::new(
            move |closing_block_id: CustomBlockId, _window: &mut Window, cx: &mut App| {
                if let Some(session) = session_view.upgrade() {
                    session.update(cx, |session, cx| {
                        session.blocks.remove(&request_key);
                        session.remove_in_flight(&request_key);
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
        let mut finish_subscriptions = Vec::with_capacity(2);
        {
            let key = request_id.clone();
            let subscription = cx.subscribe(
                &output_view,
                move |session, _view, _ev: &OutputViewFinishedEmpty, cx| {
                    session.replace_block_with_inlay(&key, "", cx);
                },
            );
            finish_subscriptions.push(subscription);
        }
        {
            let key = request_id.clone();
            let subscription = cx.subscribe(
                &output_view,
                move |session, _view, ev: &OutputViewFinishedSmall, cx| {
                    let text = ev.0.clone();
                    session.replace_block_with_inlay(&key, &text, cx);
                },
            );
            finish_subscriptions.push(subscription);
        }

        let current_namespace = self.namespace().to_string();
        let stream_task = cx.spawn({
            let view = output_view.downgrade();
            let key = request_id.clone();
            async move |this, cx| {
                let saw_namespace_not_found =
                    pump_stream(stream, view, Some(handles), console, cx).await;
                this.update(cx, |session, _cx| {
                    if saw_namespace_not_found {
                        session.unensure_namespace(&current_namespace);
                    }
                    session.remove_in_flight(&key);
                })
                .log_err();
            }
        });

        self.in_flight.insert(request_id.clone());
        self.blocks.insert(
            request_id,
            EditorBlock {
                code_range: anchor_range,
                invalidation_anchor,
                block_id,
                _output_view: output_view,
                _stream_task: stream_task,
                _subscriptions: finish_subscriptions,
            },
        );
        cx.notify();
        Ok(())
    }

    /// Shows informational text (documentation) in a block below
    /// `anchor_range`, replacing any earlier result there.
    fn install_note_block(
        &mut self,
        anchor_range: Range<Anchor>,
        label: SharedString,
        text: String,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let editor = self.editor.upgrade().context("editor was dropped")?;
        self.evict_overlapping(&editor, &anchor_range, cx);
        let output_view = cx.new(|cx| {
            let mut view = OutputView::new(OutputStatus::Note(label), cx);
            view.push_stdout(text, cx);
            view
        });
        let request_key: String = format!("note-{}", output_view.entity_id());
        let session_view = cx.entity().downgrade();
        let weak_editor = self.editor.clone();
        let key = request_key.clone();
        let on_close: CloseBlockFn = Arc::new(move |block_id, _window, cx| {
            if let Some(session) = session_view.upgrade() {
                session.update(cx, |session, _| {
                    session.blocks.remove(&key);
                });
            }
            if let Some(editor) = weak_editor.upgrade() {
                editor.update(cx, |editor, cx| {
                    editor.remove_blocks(HashSet::from_iter([block_id]), None, cx);
                });
            }
        });
        let (block_id, invalidation_anchor) = editor.update(cx, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let end_point = anchor_range.end.to_point(&snapshot);
            let placement = snapshot.anchor_before(end_point);
            let block = BlockProperties {
                placement: BlockPlacement::Below(placement),
                height: Some(1),
                style: BlockStyle::Sticky,
                render: create_renderer(output_view.clone(), on_close),
                priority: 0,
            };
            (editor.insert_blocks([block], None, cx)[0], placement)
        });
        self.blocks.insert(
            request_key,
            EditorBlock {
                code_range: anchor_range,
                invalidation_anchor,
                block_id,
                _output_view: output_view,
                _stream_task: Task::ready(()),
                _subscriptions: Vec::new(),
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

        let mut request_ids_to_remove = Vec::new();
        self.blocks.retain(|request_id, block| {
            if anchor_range.overlaps(&block.code_range, &buffer) {
                request_ids_to_remove.push(request_id.clone());
                blocks_to_remove.insert(block.block_id);
                false
            } else {
                true
            }
        });
        for request_id in request_ids_to_remove {
            self.in_flight.remove(&request_id);
        }
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
        self.remove_in_flight(request_id);
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
            .log_err();
        self.blocks.clear();
        self.result_inlays.clear();
        self.in_flight.clear();
        cx.notify();
    }

    fn remove_in_flight(&mut self, request_id: &str) {
        self.in_flight.remove(request_id);
    }

    /// Resolves the workspace's nREPL connection into a `(client,
    /// session_id)` pair, returning a meaningful error if not yet
    /// connected. The returned client is a cheap clone — sharing the
    /// same TCP socket and request multiplexer.
    fn connection_handles(&self, cx: &App) -> Result<(NreplClient, String)> {
        let workspace = self.workspace.upgrade().context("workspace dropped")?;
        connected_client(workspace.entity_id(), cx)
    }

    pub fn interrupt(&mut self, cx: &mut Context<Self>) -> Result<()> {
        let Some(request_id) = self.in_flight.most_recent().map(str::to_owned) else {
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

async fn pump_stream(
    mut stream: RequestStream,
    view: WeakEntity<OutputView>,
    stacktrace_source: Option<(NreplClient, String)>,
    console: Option<WeakEntity<NreplConsole>>,
    cx: &mut AsyncApp,
) -> bool {
    let mut saw_done = false;
    let mut saw_error = false;
    let mut saw_namespace_not_found = false;
    while let Some(msg) = stream.next().await {
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
        saw_error |= status_items.iter().any(|s| s == "eval-error");
        saw_namespace_not_found |= status_items.iter().any(|s| s == "namespace-not-found");

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

        if let Some(console) = &console {
            let statuses: Vec<&str> = status_items.iter().map(String::as_str).collect();
            console
                .update(cx, |console, cx| {
                    console.handle_reply(&msg, &statuses, None, cx);
                })
                .log_err();
        }

        if update_result.is_err() {
            // OutputView entity dropped (block closed). No reason to keep
            // pumping, but let the stream drop naturally so the client's
            // pending map entry gets cleared.
            return saw_namespace_not_found;
        }

        if is_done {
            saw_done = true;
            break;
        }
    }

    // With cider-nrepl, replace the bare exception line with a navigable,
    // project-first stacktrace.
    if saw_error && let Some((client, session)) = stacktrace_source {
        let causes = ops::last_stacktrace(&client, &session)
            .await
            .log_err()
            .unwrap_or_default();
        view.update(cx, |view, cx| {
            for cause in &causes {
                view.push_exception(format!("{}: {}", cause.class, cause.message.trim()), cx);
                let project_frames = cause.frames.iter().filter(|frame| frame.project).count();
                for frame in cause
                    .frames
                    .iter()
                    .filter(|frame| project_frames == 0 || frame.project)
                    .take(12)
                {
                    let location = match (&frame.file, frame.line) {
                        (Some(file), Some(line)) => format!(" ({file}:{line})"),
                        _ => String::new(),
                    };
                    view.push_chunk(
                        OutputChunk::Frame {
                            label: format!("{}{location}", frame.name),
                            path: frame
                                .file_url
                                .as_deref()
                                .and_then(ops::local_path_from_file_url),
                            line: frame.line,
                        },
                        cx,
                    );
                }
            }
        })
        .log_err();

        if let Some(console) = &console {
            if !causes.is_empty() {
                let text = format_stacktrace(&causes);
                console
                    .update(cx, |console, cx| {
                        console.append_line(&text, None, cx);
                    })
                    .log_err();
            }
        }
    }

    view.update(cx, |view, cx| {
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
    })
    .log_err();

    saw_namespace_not_found
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

/// Finds a form relative to the cursor with `find`, returning its code
/// and the multibuffer range it was read from.
fn form_near_cursor(
    editor: &Entity<Editor>,
    find: fn(&language::BufferSnapshot, usize) -> Option<TopLevelForm>,
    not_found: &str,
    cx: &mut App,
) -> Result<(String, Range<Anchor>)> {
    editor.update(cx, |editor, cx| -> Result<_> {
        let multibuffer = editor.buffer().clone();
        let buffer = multibuffer
            .read(cx)
            .as_singleton()
            .context("nREPL eval needs a single-buffer view (this looks like a multi-buffer)")?;
        let buffer_snapshot = buffer.read(cx).snapshot();
        let multibuffer_snapshot = multibuffer.read(cx).snapshot(cx);
        let head = editor.selections.newest_anchor().head();
        // Singleton multibuffer => MultiBufferOffset.0 maps 1-to-1 to the
        // underlying buffer's offsets.
        let MultiBufferOffset(offset) = head.to_offset(&multibuffer_snapshot);
        let form = find(&buffer_snapshot, offset).with_context(|| not_found.to_string())?;
        let anchor_range = multibuffer_snapshot.anchor_before(MultiBufferOffset(form.range.start))
            ..multibuffer_snapshot.anchor_after(MultiBufferOffset(form.range.end));
        Ok((form.text, anchor_range))
    })
}

/// Shared shape of the "find a form, evaluate it inline" actions.
fn eval_found_form(
    editor: WeakEntity<Editor>,
    find: fn(&language::BufferSnapshot, usize) -> Option<TopLevelForm>,
    not_found: &str,
    window: &mut Window,
    cx: &mut App,
) -> Result<()> {
    if !NreplSettings::enabled(cx) {
        return Ok(());
    }
    let session = editor_session(&editor, true, cx)
        .ok_or_else(|| anyhow!("editor has no associated workspace"))?;
    let editor_entity = editor.upgrade().context("editor was dropped")?;
    let (form_text, anchor_range) = form_near_cursor(&editor_entity, find, not_found, cx)?;
    session.update(cx, |session, cx| {
        session.refresh_namespace(cx);
        session.dispatch_eval(form_text, anchor_range, window, cx)
    })?;
    Ok(())
}

/// Evaluates the top-level form around the cursor (`cider-eval-defun-at-point`).
pub fn eval_form_at_cursor(
    editor: WeakEntity<Editor>,
    window: &mut Window,
    cx: &mut App,
) -> Result<()> {
    eval_found_form(
        editor,
        top_level_form_at_offset,
        "no top-level form under cursor — place the cursor inside a form, \
         or use `nrepl: eval selection`",
        window,
        cx,
    )
}

/// Evaluates the innermost form at (or right before) the cursor.
pub fn eval_current_form(
    editor: WeakEntity<Editor>,
    window: &mut Window,
    cx: &mut App,
) -> Result<()> {
    eval_found_form(
        editor,
        current_form_at_offset,
        "no form at the cursor",
        window,
        cx,
    )
}

/// Evaluates from the start of the top-level form to the cursor, closing
/// any open brackets.
pub fn eval_to_cursor(editor: WeakEntity<Editor>, window: &mut Window, cx: &mut App) -> Result<()> {
    eval_found_form(
        editor,
        top_level_form_to_cursor,
        "no top-level form under cursor",
        window,
        cx,
    )
}

pub fn eval_selection(editor: WeakEntity<Editor>, window: &mut Window, cx: &mut App) -> Result<()> {
    if !NreplSettings::enabled(cx) {
        return Ok(());
    }
    let session = editor_session(&editor, true, cx)
        .ok_or_else(|| anyhow!("editor has no associated workspace"))?;
    let editor_entity = editor.upgrade().context("editor was dropped")?;

    enum SelectionPrep {
        Empty,
        Whitespace,
        Ready {
            text: String,
            anchor_range: Range<Anchor>,
        },
    }

    let prepared = editor_entity.update(cx, |editor, cx| {
        let multibuffer = editor.buffer().clone();
        let display_snapshot = editor.display_snapshot(cx);
        let selection = editor
            .selections
            .newest::<MultiBufferOffset>(&display_snapshot);
        if selection.start == selection.end {
            return SelectionPrep::Empty;
        }
        let buffer_snapshot = multibuffer.read(cx).snapshot(cx);
        let text: String = buffer_snapshot
            .text_for_range(selection.start..selection.end)
            .collect();
        if text.trim().is_empty() {
            return SelectionPrep::Whitespace;
        }
        let range = buffer_snapshot.anchor_before(selection.start)
            ..buffer_snapshot.anchor_after(selection.end);
        SelectionPrep::Ready {
            text,
            anchor_range: range,
        }
    });

    let (text, anchor_range) = match prepared {
        SelectionPrep::Empty => {
            anyhow::bail!("no selection — select a region first, or use `nrepl: eval`");
        }
        SelectionPrep::Whitespace => {
            anyhow::bail!("selection is empty whitespace");
        }
        SelectionPrep::Ready { text, anchor_range } => (text, anchor_range),
    };

    session.update(cx, |session, cx| {
        session.refresh_namespace(cx);
        session.dispatch_eval(text, anchor_range, window, cx)
    })?;
    Ok(())
}

pub fn load_file(editor: WeakEntity<Editor>, window: &mut Window, cx: &mut App) -> Result<()> {
    load_buffer(editor, false, window, cx)
}

/// Evaluates the whole buffer as it is in the editor, saved or not.
pub fn eval_buffer(editor: WeakEntity<Editor>, window: &mut Window, cx: &mut App) -> Result<()> {
    load_buffer(editor, true, window, cx)
}

fn load_buffer(
    editor: WeakEntity<Editor>,
    allow_dirty: bool,
    window: &mut Window,
    cx: &mut App,
) -> Result<()> {
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
        if buffer_read.is_dirty() && !allow_dirty {
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
                session.dispatch_load_file(contents, file_path, file_name, anchor_range, window, cx)
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
        // No session has been created for this editor yet, so there's
        // definitely nothing in flight. Tell the user rather than
        // silently no-oping — they expected something to happen.
        anyhow::bail!("nothing to interrupt — no eval has run in this editor yet");
    };
    let had_in_flight = session.update(cx, |session, cx| {
        let was_running = !session.in_flight.is_empty();
        session.interrupt(cx).map(|_| was_running)
    })?;
    if !had_in_flight {
        anyhow::bail!("nothing to interrupt — no eval is currently running");
    }
    Ok(())
}

/// Switches the REPL session (and the REPL console prompt) to the
/// current file's namespace.
pub fn switch_namespace(editor: WeakEntity<Editor>, cx: &mut App) -> Result<()> {
    if !NreplSettings::enabled(cx) {
        return Ok(());
    }
    let session = editor_session(&editor, true, cx)
        .ok_or_else(|| anyhow!("editor has no associated workspace"))?;
    let (namespace, handles, workspace) = session.update(cx, |session, cx| {
        session.refresh_namespace(cx);
        (
            session.namespace().to_string(),
            session.connection_handles(cx),
            session.workspace.clone(),
        )
    });
    let (client, session_id) = handles?;
    let code = format!("(clojure.core/in-ns '{namespace})");
    cx.spawn(async move |cx| {
        let result = ops::eval(&client, &session_id, None, &code).await;
        cx.update(|cx| match result {
            Ok(_) => {
                if let Some(console) = workspace
                    .upgrade()
                    .and_then(|workspace| crate::console::existing_console(workspace.read(cx), cx))
                {
                    console.update(cx, |console, cx| {
                        console.set_namespace(namespace.clone(), cx)
                    });
                }
                toast_workspace(&workspace, format!("nREPL: switched to {namespace}"), cx);
            }
            Err(err) => toast_workspace(&workspace, format!("nREPL: {err:#}"), cx),
        });
    })
    .detach();
    Ok(())
}

/// `(require 'ns :reload)` (or `:reload-all`) for the current file.
pub fn require_reload(editor: WeakEntity<Editor>, all: bool, cx: &mut App) -> Result<()> {
    if !NreplSettings::enabled(cx) {
        return Ok(());
    }
    let session = editor_session(&editor, true, cx)
        .ok_or_else(|| anyhow!("editor has no associated workspace"))?;
    let (namespace, handles, workspace) = session.update(cx, |session, cx| {
        session.refresh_namespace(cx);
        (
            session.namespace().to_string(),
            session.connection_handles(cx),
            session.workspace.clone(),
        )
    });
    let (client, session_id) = handles?;
    let flag = if all { ":reload-all" } else { ":reload" };
    let code = format!("(clojure.core/require '{namespace} {flag})");
    cx.spawn(async move |cx| {
        let result = ops::eval(&client, &session_id, None, &code).await;
        cx.update(|cx| {
            let message = match result {
                Ok(outcome) => match outcome.exception {
                    None => format!("nREPL: reloaded {namespace}"),
                    Some(ex) => format!(
                        "nREPL: reloading {namespace} failed: {ex} {}",
                        outcome.err.lines().next().unwrap_or_default()
                    ),
                },
                Err(err) => format!("nREPL: {err:#}"),
            };
            toast_workspace(&workspace, message, cx);
        });
    })
    .detach();
    Ok(())
}

/// Evaluates the top-level form and writes the result into the buffer as
/// a `;; =>` comment after it.
pub fn eval_and_comment(editor: WeakEntity<Editor>, cx: &mut App) -> Result<()> {
    if !NreplSettings::enabled(cx) {
        return Ok(());
    }
    let session = editor_session(&editor, true, cx)
        .ok_or_else(|| anyhow!("editor has no associated workspace"))?;
    let editor_entity = editor.upgrade().context("editor was dropped")?;
    let (code, anchor_range) = form_near_cursor(
        &editor_entity,
        top_level_form_at_offset,
        "no top-level form under cursor",
        cx,
    )?;
    let (namespace, handles, workspace) = session.update(cx, |session, cx| {
        session.refresh_namespace(cx);
        let handles = session.connection_handles(cx);
        if let Ok((client, session_id)) = &handles {
            session.ensure_namespace(client, session_id, cx);
        }
        (
            session.namespace().to_string(),
            handles,
            session.workspace.clone(),
        )
    });
    let (client, session_id) = handles?;
    let weak_session = session.downgrade();
    cx.spawn(async move |cx| {
        let result = ops::eval(&client, &session_id, Some(&namespace), &code).await;
        if let Err(error) = &result {
            let error_string = format!("{error:#}");
            if error_string.contains("namespace") && error_string.contains("not found") {
                weak_session
                    .update(cx, |session, _| {
                        session.unensure_namespace(&namespace);
                    })
                    .log_err();
            }
        }
        cx.update(|cx| match result {
            Ok(outcome) => {
                let comment = result_comment(&outcome);
                editor_entity.update(cx, |editor, cx| {
                    let end = anchor_range.end;
                    editor.buffer().update(cx, |buffer, cx| {
                        buffer.edit([(end..end, comment)], None, cx);
                    });
                });
            }
            Err(error) => toast_workspace(&workspace, format!("nREPL: {error:#}"), cx),
        });
    })
    .detach();
    Ok(())
}

fn result_comment(outcome: &ops::EvalOutcome) -> String {
    let body = match (&outcome.exception, &outcome.value) {
        (Some(ex), _) => {
            let detail = outcome.err.lines().next().unwrap_or_default();
            format!("!! {ex} {detail}").trim_end().to_string()
        }
        (None, Some(value)) => value.trim_end().to_string(),
        (None, None) => "nil".to_string(),
    };
    if body.contains('\n') {
        let mut comment = String::from("\n;; =>");
        for line in body.lines() {
            comment.push_str("\n;; ");
            comment.push_str(line);
        }
        comment
    } else {
        format!(" ;; => {body}")
    }
}

/// Sends the top-level form under the cursor to the REPL console, in the
/// file's namespace, opening the console if needed.
pub fn send_to_repl(editor: WeakEntity<Editor>, window: &mut Window, cx: &mut App) -> Result<()> {
    if !NreplSettings::enabled(cx) {
        return Ok(());
    }
    let session = editor_session(&editor, true, cx)
        .ok_or_else(|| anyhow!("editor has no associated workspace"))?;
    let editor_entity = editor.upgrade().context("editor was dropped")?;
    let selection_text = editor_entity.update(cx, |editor, cx| {
        let display_snapshot = editor.display_snapshot(cx);
        let selection = editor
            .selections
            .newest::<MultiBufferOffset>(&display_snapshot);
        (selection.start != selection.end).then(|| {
            editor
                .buffer()
                .read(cx)
                .snapshot(cx)
                .text_for_range(selection.start..selection.end)
                .collect::<String>()
        })
    });
    let code = match selection_text {
        Some(text) => text,
        None => {
            form_near_cursor(
                &editor_entity,
                top_level_form_at_offset,
                "no top-level form under cursor",
                cx,
            )?
            .0
        }
    };
    let (namespace, workspace) = session.update(cx, |session, cx| {
        session.refresh_namespace(cx);
        if let Ok((client, session_id)) = session.connection_handles(cx) {
            session.ensure_namespace(&client, &session_id, cx);
        }
        (session.namespace().to_string(), session.workspace.clone())
    });
    let workspace = workspace.upgrade().context("workspace dropped")?;
    let console = workspace.update(cx, |workspace, cx| {
        open_console(workspace, false, window, cx)
    });
    console.update(cx, |console, cx| {
        console.set_namespace(namespace, cx);
        console.submit_with_echo(code, window, cx);
    });
    Ok(())
}

/// Looks up the symbol under the cursor and shows its documentation in a
/// block below the line.
pub fn show_documentation(editor: WeakEntity<Editor>, cx: &mut App) -> Result<()> {
    if !NreplSettings::enabled(cx) {
        return Ok(());
    }
    let session = editor_session(&editor, true, cx)
        .ok_or_else(|| anyhow!("editor has no associated workspace"))?;
    let editor_entity = editor.upgrade().context("editor was dropped")?;
    let (symbol, anchor_range) = symbol_near_cursor(&editor_entity, cx)?;
    let (namespace, handles) = session.update(cx, |session, cx| {
        session.refresh_namespace(cx);
        let handles = session.connection_handles(cx);
        if let Ok((client, session_id)) = &handles {
            session.ensure_namespace(client, session_id, cx);
        }
        (session.namespace().to_string(), handles)
    });
    let (client, session_id) = handles?;
    let weak_session = session.downgrade();
    cx.spawn(async move |cx| {
        let result = ops::info(&client, &session_id, &namespace, &symbol).await;
        cx.update(|cx| {
            let Some(session) = weak_session.upgrade() else {
                return;
            };
            let (label, text) = match result {
                Ok(Some(info)) => ("Documentation".into(), info.render()),
                Ok(None) => (
                    "Documentation".into(),
                    format!("No documentation for `{symbol}`"),
                ),
                Err(err) => ("Documentation unavailable".into(), format!("{err:#}")),
            };
            session.update(cx, |session, cx| {
                session
                    .install_note_block(anchor_range, label, text, cx)
                    .log_err();
            });
        });
    })
    .detach();
    Ok(())
}

/// Jumps to the definition of the symbol under the cursor, as reported by
/// the running REPL. Works for anything loaded, including vars defined
/// at the REPL and library code extracted on disk.
pub fn go_to_definition(
    editor: WeakEntity<Editor>,
    window: &mut Window,
    cx: &mut App,
) -> Result<()> {
    if !NreplSettings::enabled(cx) {
        return Ok(());
    }
    let session = editor_session(&editor, true, cx)
        .ok_or_else(|| anyhow!("editor has no associated workspace"))?;
    let editor_entity = editor.upgrade().context("editor was dropped")?;
    let (symbol, _) = symbol_near_cursor(&editor_entity, cx)?;
    let (namespace, handles, workspace) = session.update(cx, |session, cx| {
        session.refresh_namespace(cx);
        let handles = session.connection_handles(cx);
        if let Ok((client, session_id)) = &handles {
            session.ensure_namespace(client, session_id, cx);
        }
        (
            session.namespace().to_string(),
            handles,
            session.workspace.clone(),
        )
    });
    let (client, session_id) = handles?;
    window
        .spawn(cx, async move |cx| {
            let result = ops::info(&client, &session_id, &namespace, &symbol).await;
            cx.update(|window, cx| {
                let message = match result {
                    Ok(Some(info)) => match info.file.as_deref() {
                        Some(file) => match ops::local_path_from_file_url(file) {
                            Some(path) => {
                                open_location(&workspace, path, info.line, info.column, window, cx)
                                    .err()
                                    .map(|err| format!("nREPL: {err:#}"))
                            }
                            None => Some(format!(
                                "nREPL: `{symbol}` is defined inside an archive ({file})"
                            )),
                        },
                        None => Some(format!("nREPL: no source location for `{symbol}`")),
                    },
                    Ok(None) => Some(format!("nREPL: `{symbol}` not found")),
                    Err(err) => Some(format!("nREPL: {err:#}")),
                };
                if let Some(message) = message {
                    toast_workspace(&workspace, message, cx);
                }
            })
            .log_err();
        })
        .detach();
    Ok(())
}

fn symbol_near_cursor(editor: &Entity<Editor>, cx: &mut App) -> Result<(String, Range<Anchor>)> {
    editor.update(cx, |editor, cx| -> Result<_> {
        let multibuffer = editor.buffer().clone();
        let buffer = multibuffer
            .read(cx)
            .as_singleton()
            .context("nREPL needs a single-buffer view")?;
        let buffer_snapshot = buffer.read(cx).snapshot();
        let multibuffer_snapshot = multibuffer.read(cx).snapshot(cx);
        let head = editor.selections.newest_anchor().head();
        let MultiBufferOffset(offset) = head.to_offset(&multibuffer_snapshot);
        let symbol =
            symbol_at_offset(&buffer_snapshot, offset).context("no symbol under the cursor")?;
        Ok((symbol, head..head))
    })
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
    toast_workspace(&workspace, message, cx);
}

/// Same as [`toast`] but takes the workspace handle directly. Used by
/// action wrappers that want to surface a user-facing error before any
/// per-editor session has been resolved (e.g. "no top-level form under
/// cursor", "not connected; run `nrepl::Connect` first").
pub fn toast_workspace(
    workspace: &WeakEntity<Workspace>,
    message: impl Into<SharedString>,
    cx: &mut App,
) {
    let message = message.into().to_string();
    workspace
        .update(cx, |workspace, cx| {
            let id = NotificationId::unique::<NreplEditorToast>();
            workspace.show_toast(Toast::new(id, message).autohide(), cx);
        })
        .log_err();
}

/// Resolves the workspace associated with `editor`, if any. Used by
/// action wrappers to route a user-facing toast for failures that
/// happen before a [`NreplEditorSession`] exists.
pub fn workspace_for_editor(
    editor: &WeakEntity<Editor>,
    cx: &App,
) -> Option<WeakEntity<Workspace>> {
    editor
        .upgrade()?
        .read(cx)
        .workspace()
        .map(|ws| ws.downgrade())
}

struct NreplEditorToast;

#[cfg(test)]
mod tests {
    use super::InFlightRequests;
    use gpui::AppContext as _;

    #[test]
    fn in_flight_requests_fall_back_to_previous_request() {
        let mut requests = InFlightRequests::default();
        requests.insert("first".to_string());
        requests.insert("second".to_string());

        assert_eq!(requests.most_recent(), Some("second"));
        requests.remove("second");
        assert_eq!(requests.most_recent(), Some("first"));
        requests.remove("first");
        assert!(requests.is_empty());
    }

    #[test]
    fn in_flight_requests_remove_non_latest_request() {
        let mut requests = InFlightRequests::default();
        requests.insert("first".to_string());
        requests.insert("second".to_string());

        requests.remove("first");
        assert_eq!(requests.most_recent(), Some("second"));
    }

    #[test]
    fn unensure_namespace_removes_from_set() {
        let mut session = super::NreplEditorSession {
            editor: gpui::WeakEntity::new_invalid(),
            workspace: gpui::WeakEntity::new_invalid(),
            namespace: Some("my.app".to_string()),
            ensured_namespaces: collections::HashSet::default(),
            ensured_session_id: Some("session-1".to_string()),
            blocks: collections::HashMap::default(),
            result_inlays: collections::HashMap::default(),
            next_inlay_id: 0,
            in_flight: InFlightRequests::default(),
            _subscriptions: Vec::new(),
        };

        session.ensured_namespaces.insert("my.app".to_string());
        assert!(session.ensured_namespaces.contains("my.app"));

        session.unensure_namespace("my.app");
        assert!(!session.ensured_namespaces.contains("my.app"));
    }

    #[gpui::test]
    async fn ensure_namespace_sends_fallback_and_tracks_session(cx: &mut gpui::TestAppContext) {
        use futures::AsyncReadExt as _;
        use futures::AsyncWriteExt as _;

        cx.executor().allow_parking();
        let listener = smol::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let received_codes = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let received_codes_clone = received_codes.clone();

        let server = cx.executor().spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let (mut reader, mut writer) = stream.split();
            let mut buffer = Vec::new();
            let mut chunk = [0u8; 4096];
            loop {
                let n = reader.read(&mut chunk).await.unwrap();
                if n == 0 {
                    break;
                }
                buffer.extend_from_slice(&chunk[..n]);
                loop {
                    match crate::bencode::decode_one(&buffer).unwrap() {
                        crate::bencode::DecodeOutcome::Incomplete => break,
                        crate::bencode::DecodeOutcome::Value { value, consumed } => {
                            buffer.drain(..consumed);
                            let id = value
                                .get("id")
                                .and_then(crate::Value::as_str)
                                .unwrap_or("")
                                .to_string();
                            if let Some(code) = value.get("code").and_then(crate::Value::as_str) {
                                received_codes_clone.lock().unwrap().push(code.to_string());
                            }
                            let reply = crate::dict([
                                ("id", crate::Value::str(id)),
                                (
                                    "status",
                                    crate::Value::List(vec![crate::Value::str("done")]),
                                ),
                            ]);
                            writer
                                .write_all(&crate::bencode::encode(&reply))
                                .await
                                .unwrap();
                            writer.flush().await.unwrap();
                        }
                    }
                }
            }
        });

        let client = crate::client::NreplClient::connect(addr, &cx.executor())
            .await
            .unwrap();

        let session_entity = cx.update(|cx| {
            cx.new(|_| super::NreplEditorSession {
                editor: gpui::WeakEntity::new_invalid(),
                workspace: gpui::WeakEntity::new_invalid(),
                namespace: Some("my.app".to_string()),
                ensured_namespaces: collections::HashSet::default(),
                ensured_session_id: None,
                blocks: collections::HashMap::default(),
                result_inlays: collections::HashMap::default(),
                next_inlay_id: 0,
                in_flight: InFlightRequests::default(),
                _subscriptions: Vec::new(),
            })
        });

        // First call with session-1: sends (clojure.core/in-ns 'my.app) and inserts
        cx.update(|cx| {
            session_entity.update(cx, |session, cx| {
                session.ensure_namespace(&client, "session-1", cx);
                assert!(session.ensured_namespaces.contains("my.app"));
                assert_eq!(session.ensured_session_id.as_deref(), Some("session-1"));
            });
        });

        for _ in 0..100 {
            if !received_codes.lock().unwrap().is_empty() {
                break;
            }
            cx.background_executor
                .timer(std::time::Duration::from_millis(10))
                .await;
        }

        assert_eq!(
            received_codes.lock().unwrap().as_slice(),
            &["(clojure.core/in-ns 'my.app)"]
        );

        // Second call with same session-1: already in set, no extra eval
        cx.update(|cx| {
            session_entity.update(cx, |session, cx| {
                session.ensure_namespace(&client, "session-1", cx);
            });
        });
        cx.background_executor
            .timer(std::time::Duration::from_millis(30))
            .await;
        assert_eq!(received_codes.lock().unwrap().len(), 1);

        // Third call with session-2: clears set and sends again for session-2
        cx.update(|cx| {
            session_entity.update(cx, |session, cx| {
                session.ensure_namespace(&client, "session-2", cx);
                assert!(session.ensured_namespaces.contains("my.app"));
                assert_eq!(session.ensured_session_id.as_deref(), Some("session-2"));
            });
        });
        for _ in 0..100 {
            if received_codes.lock().unwrap().len() >= 2 {
                break;
            }
            cx.background_executor
                .timer(std::time::Duration::from_millis(10))
                .await;
        }
        assert_eq!(received_codes.lock().unwrap().len(), 2);

        // Fourth call with namespace "user": does not ensure
        cx.update(|cx| {
            session_entity.update(cx, |session, cx| {
                session.namespace = Some("user".to_string());
                session.ensure_namespace(&client, "session-2", cx);
                assert!(!session.ensured_namespaces.contains("user"));
            });
        });
        cx.background_executor
            .timer(std::time::Duration::from_millis(30))
            .await;
        assert_eq!(received_codes.lock().unwrap().len(), 2);

        drop(client);
        server.await;
    }
}
