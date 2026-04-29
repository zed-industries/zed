//! Inline rendering of nREPL evaluation results.
//!
//! Modeled on `crates/repl/src/outputs.rs`'s `ExecutionView`, but trimmed
//! down to what nREPL actually delivers: streams of `:value`, `:out`,
//! `:err`, and exception messages, terminated by a status containing
//! `"done"`. There is no MIME-rich payload to render — Clojure values come
//! back as `pr-str`-formatted strings (the server-side default) and stdout
//! / stderr arrive as plain text.
//!
//! The view exposes the same two finish-events as the Jupyter side so
//! the per-editor session can collapse a finished block into an inlay:
//!
//! - [`OutputViewFinishedEmpty`] — done, no outputs at all (e.g. a
//!   `(comment ...)` form). The session removes the block entirely.
//! - [`OutputViewFinishedSmall`] — done, the result is small enough to
//!   fit on a single end-of-line inlay; carries the rendered string.
//!
//! Anything larger stays as a below-the-line block.

use gpui::{Context, EventEmitter, Render, prelude::*};
use ui::prelude::*;

/// Maximum length (in characters) of a single-line `:value` that's
/// eligible for the end-of-line inlay collapse. Longer results stay as a
/// full block so they don't shove the gutter around. Mirrors the default
/// of `ReplSettings::inline_output_max_length` — kept as a constant for
/// the MVP rather than a settings knob (see design doc, "Out of scope").
const INLINE_MAX_LENGTH: usize = 50;

/// High-level lifecycle state of an in-flight evaluation.
///
/// Drives the status pill in the rendered block. The transitions are:
///
/// ```text
/// Queued ──► Running ──► Finished
///   │           │            │
///   └───────────┴────────────┴──► Failed { … }   (any time)
///                            └──► Interrupted     (user-triggered)
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputStatus {
    /// Request enqueued on the client but no reply observed yet.
    Queued,
    /// First reply chunk has arrived; still streaming.
    Running,
    /// Server reported `:status "done"` cleanly.
    Finished,
    /// Server reported `:status "interrupted"` (user pressed
    /// `nrepl::Interrupt`) — distinct from `Failed` so the UI can label
    /// it neutrally.
    Interrupted,
    /// Either the connection dropped mid-stream or the server reported
    /// an error status (e.g. `"eval-error"`, `"namespace-not-found"`).
    /// The string is rendered verbatim as a one-line error label.
    Failed(SharedString),
}

impl OutputStatus {
    fn label(&self) -> SharedString {
        match self {
            OutputStatus::Queued => "Queued…".into(),
            OutputStatus::Running => "Running…".into(),
            OutputStatus::Finished => "Done".into(),
            OutputStatus::Interrupted => "Interrupted".into(),
            OutputStatus::Failed(msg) => format!("Failed · {msg}").into(),
        }
    }

    fn color(&self) -> Color {
        match self {
            OutputStatus::Queued => Color::Muted,
            OutputStatus::Running => Color::Info,
            OutputStatus::Finished => Color::Success,
            OutputStatus::Interrupted => Color::Warning,
            OutputStatus::Failed(_) => Color::Error,
        }
    }

    fn is_terminal(&self) -> bool {
        matches!(
            self,
            OutputStatus::Finished | OutputStatus::Interrupted | OutputStatus::Failed(_)
        )
    }
}

/// One displayable chunk of an evaluation reply.
///
/// nREPL streams `:out` / `:err` byte-by-line, so a single eval often
/// produces several `Stdout` / `Stderr` entries. We keep them separate
/// (rather than concatenating into one buffer) so the UI can color each
/// stream independently and the small-inline collapse heuristic only has
/// to look at `Value` entries.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputChunk {
    /// A `:value "..."` entry — i.e. the `pr-str` of an evaluated form.
    Value(String),
    /// A `:out "..."` chunk.
    Stdout(String),
    /// An `:err "..."` chunk.
    Stderr(String),
    /// An `:ex` / `:root-ex` exception summary line. Distinct from
    /// `Stderr` because servers send the human-readable trace via
    /// `:err` and these tend to be a single class name + message.
    Exception(String),
}

impl OutputChunk {
    fn text(&self) -> &str {
        match self {
            OutputChunk::Value(s)
            | OutputChunk::Stdout(s)
            | OutputChunk::Stderr(s)
            | OutputChunk::Exception(s) => s,
        }
    }

    fn color(&self) -> Color {
        match self {
            OutputChunk::Value(_) => Color::Default,
            OutputChunk::Stdout(_) => Color::Muted,
            OutputChunk::Stderr(_) | OutputChunk::Exception(_) => Color::Error,
        }
    }
}

/// Emitted once when the view finishes with no outputs at all.
///
/// The session subscribes to this and replaces the block with a "✓"
/// inlay so the line doesn't keep its big-block footprint forever after
/// a side-effect-only eval (e.g. `(in-ns 'user)`).
pub struct OutputViewFinishedEmpty;

/// Emitted once when the view finishes with a single short `:value` and
/// no other output. Carries the rendered text so the session can drop it
/// straight into an end-of-line inlay.
pub struct OutputViewFinishedSmall(pub String);

/// The render entity attached to an [`EditorBlock`](crate::editor_session)
/// while an eval is running.
///
/// All public mutators (`push_*`, `set_status`) call `cx.notify()` and
/// emit the finished-events at the right transitions, so callers don't
/// need to babysit any of that themselves.
pub struct OutputView {
    chunks: Vec<OutputChunk>,
    status: OutputStatus,
    /// True once we've emitted [`OutputViewFinishedEmpty`] or
    /// [`OutputViewFinishedSmall`]. Guards against double-emit if the
    /// session pumps an extra reply after `done` (some middleware does).
    finished_event_emitted: bool,
}

impl OutputView {
    pub fn new(status: OutputStatus, _cx: &mut Context<Self>) -> Self {
        Self {
            chunks: Vec::new(),
            status,
            finished_event_emitted: false,
        }
    }

    pub fn status(&self) -> &OutputStatus {
        &self.status
    }

    pub fn chunks(&self) -> &[OutputChunk] {
        &self.chunks
    }

    /// Pushes a chunk and flips the status to `Running` if we were still
    /// `Queued`. Does *not* call this with `:status` payloads — those go
    /// through [`Self::set_status`].
    pub fn push_chunk(&mut self, chunk: OutputChunk, cx: &mut Context<Self>) {
        if matches!(self.status, OutputStatus::Queued) {
            self.status = OutputStatus::Running;
        }
        self.chunks.push(chunk);
        cx.notify();
    }

    /// Convenience pushers used by the session when destructuring nREPL
    /// reply dicts. Equivalent to constructing the [`OutputChunk`] and
    /// calling [`Self::push_chunk`], but keep the call sites short.
    pub fn push_value(&mut self, value: impl Into<String>, cx: &mut Context<Self>) {
        self.push_chunk(OutputChunk::Value(value.into()), cx);
    }

    pub fn push_stdout(&mut self, text: impl Into<String>, cx: &mut Context<Self>) {
        self.push_chunk(OutputChunk::Stdout(text.into()), cx);
    }

    pub fn push_stderr(&mut self, text: impl Into<String>, cx: &mut Context<Self>) {
        self.push_chunk(OutputChunk::Stderr(text.into()), cx);
    }

    pub fn push_exception(&mut self, summary: impl Into<String>, cx: &mut Context<Self>) {
        self.push_chunk(OutputChunk::Exception(summary.into()), cx);
    }

    /// Updates the status, emitting the appropriate finished-event if
    /// `status` is terminal and we haven't already emitted one.
    pub fn set_status(&mut self, status: OutputStatus, cx: &mut Context<Self>) {
        let was_terminal = self.status.is_terminal();
        self.status = status;
        if !was_terminal && self.status.is_terminal() {
            self.maybe_emit_finished(cx);
        }
        cx.notify();
    }

    fn maybe_emit_finished(&mut self, cx: &mut Context<Self>) {
        if self.finished_event_emitted {
            return;
        }
        // Only attempt the inlay-collapse for clean completions. A
        // failure or interrupt should leave the block in place so the
        // user can read what went wrong.
        if !matches!(self.status, OutputStatus::Finished) {
            return;
        }
        self.finished_event_emitted = true;

        if self.chunks.is_empty() {
            cx.emit(OutputViewFinishedEmpty);
            return;
        }

        if let Some(small) = small_inline_value(&self.chunks) {
            cx.emit(OutputViewFinishedSmall(small));
        }
    }
}

/// Returns a single-line value string suitable for an inline collapse,
/// or `None` if the output is too big / mixed-stream / multi-value.
///
/// Inline collapse rules (kept conservative so we don't truncate
/// surprises):
///
/// 1. Exactly one chunk total.
/// 2. That chunk is a [`OutputChunk::Value`].
/// 3. The value's text is a single line (ignoring a trailing newline).
/// 4. The visible text is no longer than [`INLINE_MAX_LENGTH`] characters.
fn small_inline_value(chunks: &[OutputChunk]) -> Option<String> {
    let [OutputChunk::Value(value)] = chunks else {
        return None;
    };
    let trimmed = value.trim_end_matches('\n');
    if trimmed.contains('\n') {
        return None;
    }
    if trimmed.chars().count() > INLINE_MAX_LENGTH {
        return None;
    }
    Some(trimmed.to_string())
}

impl EventEmitter<OutputViewFinishedEmpty> for OutputView {}
impl EventEmitter<OutputViewFinishedSmall> for OutputView {}

impl Render for OutputView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let status_label = self.status.label();
        let status_color = self.status.color();

        v_flex()
            .gap_1()
            .px_2()
            .py_1()
            .child(
                h_flex().gap_2().items_center().child(
                    Label::new(status_label)
                        .color(status_color)
                        .size(LabelSize::Small),
                ),
            )
            .children(self.chunks.iter().map(render_chunk))
    }
}

fn render_chunk(chunk: &OutputChunk) -> impl IntoElement + use<> {
    let color = chunk.color();
    let raw = chunk.text();

    // Drop trailing whitespace/newlines so each chunk renders without
    // leaving a blank line below it; nREPL servers commonly suffix a
    // newline to `:value` and `:out`.
    let display: SharedString = raw.trim_end().to_string().into();

    div().child(
        Label::new(display)
            .color(color)
            .size(LabelSize::Small)
            .single_line()
            .truncate(),
    )
}

/// Convenience used by the session when the connection drops while a
/// request is in flight: turn the still-open output view into a
/// `Failed` state with a clear message.
pub fn fail_in_flight(
    view: &mut OutputView,
    message: impl Into<SharedString>,
    cx: &mut Context<OutputView>,
) {
    if view.status.is_terminal() {
        return;
    }
    view.set_status(OutputStatus::Failed(message.into()), cx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use std::sync::{Arc, Mutex};

    #[test]
    fn small_inline_value_picks_up_single_short_value() {
        assert_eq!(
            small_inline_value(&[OutputChunk::Value("42".into())]).as_deref(),
            Some("42"),
        );
        // Trailing newlines are tolerated — Clojure's `pr-str` doesn't
        // add one but plenty of middleware does.
        assert_eq!(
            small_inline_value(&[OutputChunk::Value("\"hi\"\n".into())]).as_deref(),
            Some("\"hi\""),
        );
    }

    #[test]
    fn small_inline_value_rejects_multiline_or_mixed() {
        // Multi-line value: stays as a block so the user can read both lines.
        assert_eq!(
            small_inline_value(&[OutputChunk::Value("line1\nline2".into())]),
            None,
        );
        // Stdout-only: not a value, never collapses.
        assert_eq!(
            small_inline_value(&[OutputChunk::Stdout("hi\n".into())]),
            None,
        );
        // Value + extra chunk: don't lose information.
        assert_eq!(
            small_inline_value(&[
                OutputChunk::Stdout("hi\n".into()),
                OutputChunk::Value("nil".into()),
            ]),
            None,
        );
        // Too long to fit on a line.
        let big = "x".repeat(INLINE_MAX_LENGTH + 1);
        assert_eq!(small_inline_value(&[OutputChunk::Value(big)]), None);
    }

    #[gpui::test]
    async fn finished_with_no_chunks_emits_empty(cx: &mut TestAppContext) {
        let view = cx.new(|cx| OutputView::new(OutputStatus::Queued, cx));

        let empty_count = Arc::new(Mutex::new(0u32));
        let small_count = Arc::new(Mutex::new(0u32));

        cx.update(|cx| {
            cx.subscribe(&view, {
                let empty_count = empty_count.clone();
                move |_, _: &OutputViewFinishedEmpty, _| {
                    *empty_count.lock().unwrap() += 1;
                }
            })
            .detach();
            cx.subscribe(&view, {
                let small_count = small_count.clone();
                move |_, _: &OutputViewFinishedSmall, _| {
                    *small_count.lock().unwrap() += 1;
                }
            })
            .detach();
        });

        view.update(cx, |view, cx| {
            view.set_status(OutputStatus::Finished, cx);
        });

        assert_eq!(*empty_count.lock().unwrap(), 1);
        assert_eq!(*small_count.lock().unwrap(), 0);
    }

    #[gpui::test]
    async fn finished_with_short_value_emits_small(cx: &mut TestAppContext) {
        let view = cx.new(|cx| OutputView::new(OutputStatus::Queued, cx));

        let captured = Arc::new(Mutex::new(None::<String>));
        cx.update(|cx| {
            cx.subscribe(&view, {
                let captured = captured.clone();
                move |_, ev: &OutputViewFinishedSmall, _| {
                    *captured.lock().unwrap() = Some(ev.0.clone());
                }
            })
            .detach();
        });

        view.update(cx, |view, cx| {
            view.push_value("42", cx);
            view.set_status(OutputStatus::Finished, cx);
        });

        assert_eq!(captured.lock().unwrap().as_deref(), Some("42"));
    }

    #[gpui::test]
    async fn failed_status_does_not_emit_finished_events(cx: &mut TestAppContext) {
        let view = cx.new(|cx| OutputView::new(OutputStatus::Running, cx));

        let any = Arc::new(Mutex::new(0u32));
        cx.update(|cx| {
            cx.subscribe(&view, {
                let any = any.clone();
                move |_, _: &OutputViewFinishedEmpty, _| {
                    *any.lock().unwrap() += 1;
                }
            })
            .detach();
            cx.subscribe(&view, {
                let any = any.clone();
                move |_, _: &OutputViewFinishedSmall, _| {
                    *any.lock().unwrap() += 1;
                }
            })
            .detach();
        });

        view.update(cx, |view, cx| {
            view.set_status(OutputStatus::Failed("boom".into()), cx);
        });

        assert_eq!(*any.lock().unwrap(), 0);
    }

    #[gpui::test]
    async fn push_chunk_promotes_queued_to_running(cx: &mut TestAppContext) {
        let view = cx.new(|cx| OutputView::new(OutputStatus::Queued, cx));
        view.update(cx, |view, cx| {
            assert_eq!(view.status(), &OutputStatus::Queued);
            view.push_stdout("hello\n", cx);
            assert_eq!(view.status(), &OutputStatus::Running);
        });
    }
}
