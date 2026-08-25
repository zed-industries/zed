//! The hidden `<textarea>` that connects browser IMEs to GPUI.
//!
//! IMEs (software keyboards, composition engines) decide what backspace,
//! autocorrect, and suggestions mean by inspecting the focused editable
//! element's value and selection. This module owns that element and keeps a
//! window of the document's text mirrored into it, so IME edits arrive as
//! interpretable events instead of operations against an empty field.
//!
//! The element's value and selection are only ever written by [`sync`],
//! reached through [`ImeMirror::schedule_sync`]: every write is observed by
//! the IME and makes the browser restart the IME's input connection, so
//! writes must be coalesced to at most one per browser event-loop turn,
//! landing only after the current gesture's events have all dispatched.
//! Keeping the element and its write path private to this module makes that
//! discipline a compile-time guarantee rather than a convention.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use wasm_bindgen::JsCast;

use crate::window::WebWindowInner;

/// UTF-16 code units of document text mirrored on each side of the
/// selection.
///
/// There is no demand-driven protocol to size this against: an IME's
/// context requests (e.g. Android's `getTextBeforeCursor`) are answered by
/// the browser from the element's current state, invisibly to the page, so
/// the window must be provisioned ahead of time. The lower bound is what
/// IMEs actually read — sentence-scale context, on the order of a hundred
/// units. The upper bound is that the window size is a per-keystroke cost,
/// not a one-time cost: every imported edit diffs the element's full value
/// against the stored mirror, and every rebuild writes the full window into
/// the element and re-snapshots it across the browser–IME boundary, all on
/// the main thread between frames. An IME that would read further than the
/// window simply sees text truncated at the window's edge — the same thing
/// it sees near the start of any short field — so oversizing buys nothing.
const CONTEXT_CHARS: usize = 512;

/// The element is left alone until the selection gets this close to the
/// mirrored window's edge (unless it desynchronizes outright). Must exceed
/// the span an IME plausibly reads or edits around the caret within a
/// single gesture (a long word plus autocorrect lookback); beyond that,
/// recentering lazily is strictly better, because every recenter is an
/// element write and therefore an IME restart.
const MIN_EDGE_CHARS: usize = 64;

/// The hidden `<textarea>` IMEs edit, plus the bookkeeping that relates it
/// to the document.
///
/// The element and every value/selection write on it are private to this
/// module; other code interacts through read accessors, focus and
/// read-only control, and [`ImeMirror::schedule_sync`].
pub(crate) struct ImeMirror {
    element: web_sys::HtmlTextAreaElement,
    /// The mirror text most recently synced to (or observed in) the hidden
    /// element. `input` events diff the element's new value against this to
    /// recover what edit the IME performed.
    text: RefCell<String>,
    /// The element's selection (in element-local UTF-16 offsets) as of the
    /// last sync or imported edit. Gives IME edits their position relative
    /// to the caret; deliberately element-local, never document
    /// coordinates, which go stale in a collaborative document.
    selection: Cell<(u32, u32)>,
    /// Document offset where the mirror window starts — as a *hint only*.
    /// It is never trusted for edits: every use first re-verifies the
    /// stored window text against the document at this alignment, so a
    /// stale hint costs a window rebuild instead of a misplaced edit.
    window_hint: Cell<usize>,
    /// Whether a coalesced sync is already scheduled for the next task.
    /// Multiple sync requests within one gesture must collapse into a
    /// single element write after the gesture: keyboards sample the field
    /// between writes, and a mid-gesture barrage desynchronizes their word
    /// model.
    sync_scheduled: Cell<bool>,
}

impl ImeMirror {
    pub(crate) fn new(
        document: &web_sys::Document,
        body: &web_sys::HtmlElement,
    ) -> anyhow::Result<Self> {
        // A textarea rather than an input: single-line inputs silently strip
        // newlines from assigned values, which would make the mirror text
        // disagree with what was written into it.
        let element: web_sys::HtmlTextAreaElement = document
            .create_element("textarea")
            .map_err(|e| anyhow::anyhow!("Failed to create textarea element: {e:?}"))?
            .dyn_into()
            .map_err(|e| anyhow::anyhow!("Created element is not a textarea: {e:?}"))?;
        let style = element.style();
        style.set_property("position", "fixed").ok();
        style.set_property("top", "0").ok();
        style.set_property("left", "0").ok();
        style.set_property("width", "1px").ok();
        style.set_property("height", "1px").ok();
        style.set_property("opacity", "0").ok();
        // Android Chrome zooms the visual viewport onto a focused text input
        // whose font is smaller than 16px; with page zoom disabled the user
        // can never zoom back out, so keep the hidden IME input at 16px.
        style.set_property("font-size", "16px").ok();
        // The element is an IME conduit, not a form field: browser-side text
        // assistance would mutate it behind the app's back.
        element.set_spellcheck(false);
        element.set_attribute("autocomplete", "off").ok();
        element.set_attribute("autocapitalize", "off").ok();
        element.set_attribute("autocorrect", "off").ok();
        body.append_child(&element)
            .map_err(|e| anyhow::anyhow!("Failed to append input to body: {e:?}"))?;
        element.focus().ok();

        Ok(Self {
            element,
            text: RefCell::new(String::new()),
            selection: Cell::new((0, 0)),
            window_hint: Cell::new(0),
            sync_scheduled: Cell::new(false),
        })
    }

    pub(crate) fn event_target(&self) -> &web_sys::EventTarget {
        self.element.as_ref()
    }

    pub(crate) fn focus(&self) {
        self.element.focus().ok();
    }

    pub(crate) fn blur(&self) {
        self.element.blur().ok();
    }

    pub(crate) fn read_only(&self) -> bool {
        self.element.read_only()
    }

    pub(crate) fn set_read_only(&self, read_only: bool) {
        self.element.set_read_only(read_only);
    }

    pub(crate) fn remove(&self) {
        let element: &web_sys::Element = self.element.as_ref();
        element.remove();
    }

    pub(crate) fn value(&self) -> String {
        self.element.value()
    }

    pub(crate) fn selection_start(&self) -> Option<u32> {
        self.element.selection_start().ok().flatten()
    }

    pub(crate) fn stored_text(&self) -> String {
        self.text.borrow().clone()
    }

    pub(crate) fn stored_selection(&self) -> (u32, u32) {
        self.selection.get()
    }

    /// Adopts the element's current value and selection as the mirror
    /// baseline without writing to the element. Used when the browser
    /// itself applied an edit (an imported IME edit, a composition commit):
    /// the element is already what the IME expects, and echoing a write
    /// back would restart the IME mid-gesture.
    pub(crate) fn adopt_element_state(&self) {
        *self.text.borrow_mut() = self.element.value();
        let selection_start = self.selection_start().unwrap_or(0);
        let selection_end = self
            .element
            .selection_end()
            .ok()
            .flatten()
            .unwrap_or(selection_start);
        self.selection.set((selection_start, selection_end));
    }

    /// Schedules a coalesced sync of the mirror for the next task.
    ///
    /// Event handlers must not write to the mirror element mid-gesture:
    /// every write (value, selection) is observed by the IME, and a
    /// sequence of writes inside one gesture desynchronizes its model of
    /// the field (every native-behaving reference — a plain textarea —
    /// performs at most one such change per gesture). Deferring to a
    /// zero-delay timeout coalesces all sync requests from one gesture into
    /// a single write that lands after the browser has finished processing
    /// the gesture's events.
    ///
    /// `sync` is deliberately nested here so that scheduling is the only
    /// way to reach it: a direct synchronous call would reintroduce the
    /// mid-gesture writes this indirection exists to prevent.
    pub(crate) fn schedule_sync(window: &Rc<WebWindowInner>) {
        if window.ime_mirror.sync_scheduled.replace(true) {
            return;
        }
        let closure = wasm_bindgen::closure::Closure::once_into_js({
            let window = Rc::clone(window);
            move || {
                window.ime_mirror.sync_scheduled.set(false);
                sync(&window);
            }
        });
        window
            .browser_window
            .set_timeout_with_callback(closure.unchecked_ref())
            .ok();

        /// Mirrors the text surrounding the selection into the hidden
        /// element.
        ///
        /// With an empty element, Gboard deletes against its private buffer
        /// (the keypress reaches the page only as an `"Unidentified"`
        /// placeholder) and its suggestion strip has no context. Mirroring
        /// a window of real text makes those operations arrive as
        /// interpretable `beforeinput` events.
        ///
        /// All offsets are UTF-16 code units on both sides: GPUI's
        /// input-handler protocol and JavaScript string indexing agree by
        /// construction.
        ///
        /// Writing to the element is a last resort: any rewrite of its
        /// value or selection makes the browser restart the IME's input
        /// connection, which resets the keyboard's state — fatal in the
        /// middle of a keyboard's multi-step edit sequence (suggestion
        /// picks arrive as delete-then-insert pairs). After an imported
        /// edit, the element already *is* a faithful — if off-center —
        /// window of the document, so this first verifies the element
        /// against the document at its current alignment and skips every
        /// write while that holds. The window is rebuilt only when the app
        /// changed independently (caret moved by tap or keybinding, remote
        /// edit inside the window) or the selection drifted too close to
        /// the window's edge to give the IME context.
        fn sync(window: &WebWindowInner) {
            if window.is_composing.get() {
                return;
            }
            let mirror = &window.ime_mirror;
            let selection = window
                .with_input_handler(|handler| handler.selected_text_range(false))
                .flatten();
            let Some(selection) = selection else {
                if !mirror.text.borrow().is_empty() {
                    mirror.element.set_value("");
                    mirror.text.borrow_mut().clear();
                }
                mirror.selection.set((0, 0));
                return;
            };

            if is_consistent(window, &selection.range, MIN_EDGE_CHARS) {
                return;
            }

            // A caret move within the existing window (a tap into nearby
            // text) must update only the element's selection, like a native
            // tap in a plain textarea. Rewriting the value restarts the IME
            // connection, which desynchronizes the keyboard's word model
            // right when it is about to act on the tapped word.
            if move_selection_within_window(window, &selection.range, MIN_EDGE_CHARS) {
                return;
            }

            let window_range = selection.range.start.saturating_sub(CONTEXT_CHARS)
                ..selection.range.end + CONTEXT_CHARS;
            let mut adjusted = None;
            let text = window
                .with_input_handler(|handler| {
                    handler.text_for_range(window_range.clone(), &mut adjusted)
                })
                .flatten()
                .unwrap_or_default();
            let window_start = adjusted.unwrap_or(window_range).start;

            if *mirror.text.borrow() != text || mirror.element.value() != text {
                mirror.element.set_value(&text);
                *mirror.text.borrow_mut() = text;
            }

            mirror.window_hint.set(window_start);
            let selection_start = selection.range.start.saturating_sub(window_start) as u32;
            let selection_end = selection.range.end.saturating_sub(window_start) as u32;
            if mirror.element.selection_start().ok().flatten() != Some(selection_start)
                || mirror.element.selection_end().ok().flatten() != Some(selection_end)
            {
                mirror
                    .element
                    .set_selection_range(selection_start, selection_end)
                    .ok();
            }
            // Read the selection back rather than trusting the computed
            // values: the browser clamps out-of-bounds positions, and a
            // stored selection the element doesn't actually have would
            // corrupt the next diff.
            let actual_start = mirror.element.selection_start().ok().flatten();
            let actual_end = mirror.element.selection_end().ok().flatten();
            mirror.selection.set((
                actual_start.unwrap_or(selection_start),
                actual_end.unwrap_or(selection_end),
            ));
        }
    }
}

/// Attempts to represent a changed app selection as a pure element
/// selection move within the existing mirror window.
///
/// The stored window-start hint is re-verified textually against the
/// document before use, so a stale hint (remote edit, any drift) fails
/// verification and falls through to a full window rebuild rather than
/// mispositioning the selection.
fn move_selection_within_window(
    window: &WebWindowInner,
    app_selection: &std::ops::Range<usize>,
    min_edge: usize,
) -> bool {
    let mirror = &window.ime_mirror;
    let stored_text = mirror.text.borrow().clone();
    let stored_length = stored_text.encode_utf16().count();
    if stored_length == 0 || mirror.element.value() != stored_text {
        return false;
    }
    let window_start = mirror.window_hint.get();

    // The new selection must sit inside the window with enough context
    // on both sides — except where the window is pinned to a document
    // boundary, where less context is all the context there is. This is
    // the common case: a chat thread's caret usually sits at the end of
    // the document, where the window has no right margin at all.
    let Some(selection_start) = app_selection.start.checked_sub(window_start) else {
        return false;
    };
    let selection_end = selection_start + (app_selection.end - app_selection.start);
    if selection_end > stored_length {
        return false;
    }
    if selection_start < min_edge && window_start != 0 {
        return false;
    }

    // Verify the hint: the stored window text must still equal the
    // document at this alignment. Asking for one unit extra also
    // determines whether the window reaches the document's end, which
    // excuses a missing right margin.
    let mut adjusted = None;
    let document_text = window
        .with_input_handler(|handler| {
            handler.text_for_range(
                window_start..window_start + stored_length + 1,
                &mut adjusted,
            )
        })
        .flatten()
        .unwrap_or_default();
    let document_text_length = document_text.encode_utf16().count();
    let window_at_document_end = document_text_length == stored_length;
    if selection_end + min_edge > stored_length && !window_at_document_end {
        return false;
    }
    if !document_text.starts_with(stored_text.as_str()) || document_text_length > stored_length + 1
    {
        return false;
    }

    mirror
        .element
        .set_selection_range(selection_start as u32, selection_end as u32)
        .ok();
    let actual_start = mirror.element.selection_start().ok().flatten();
    let actual_end = mirror.element.selection_end().ok().flatten();
    if actual_start != Some(selection_start as u32) || actual_end != Some(selection_end as u32) {
        return false;
    }
    mirror
        .selection
        .set((selection_start as u32, selection_end as u32));
    true
}

/// Whether the hidden element, at its current window alignment, is still
/// an accurate mirror of the document around the app selection with
/// enough context on both sides. When this holds, a sync must not touch
/// the element (see [`sync`] on why writes are harmful).
fn is_consistent(
    window: &WebWindowInner,
    app_selection: &std::ops::Range<usize>,
    min_edge: usize,
) -> bool {
    let mirror = &window.ime_mirror;
    let (element_selection_start, element_selection_end) = mirror.selection.get();
    let element_selection_start = element_selection_start as usize;
    let element_selection_end = element_selection_end as usize;
    let stored_text = mirror.text.borrow().clone();
    let stored_length = stored_text.encode_utf16().count();

    if stored_length == 0 {
        return false;
    }
    // The element's real selection must match what we believe it is.
    if mirror.element.selection_start().ok().flatten() != Some(element_selection_start as u32)
        || mirror.element.selection_end().ok().flatten() != Some(element_selection_end as u32)
    {
        return false;
    }
    // Enough context on both sides of the selection, unless the window
    // is pinned to a document boundary (start of window at document
    // offset 0, or window end at document end — approximated by the
    // stored window being shorter than requested on that side).
    let app_window_start = match app_selection.start.checked_sub(element_selection_start) {
        Some(start) => start,
        None => return false,
    };
    let has_left_context = element_selection_start >= min_edge || app_window_start == 0;
    let right_context = stored_length.saturating_sub(element_selection_end);
    if !has_left_context || right_context < min_edge {
        // A short right side is fine when the window genuinely reaches
        // the end of the document; verify by asking for one unit past
        // the stored window.
        let mut adjusted = None;
        let past_end = app_window_start + stored_length;
        let more = window
            .with_input_handler(|handler| {
                handler.text_for_range(past_end..past_end + 1, &mut adjusted)
            })
            .flatten()
            .unwrap_or_default();
        if !has_left_context || !more.is_empty() {
            return false;
        }
    }
    // The stored window must still equal the document at this alignment
    // (a remote edit inside the window invalidates it), and the element
    // must still hold exactly the stored text.
    let mut adjusted = None;
    let document_text = window
        .with_input_handler(|handler| {
            handler.text_for_range(
                app_window_start..app_window_start + stored_length,
                &mut adjusted,
            )
        })
        .flatten()
        .unwrap_or_default();
    if document_text != stored_text {
        return false;
    }
    if mirror.element.value() != stored_text {
        return false;
    }
    // The element selection corresponds to the app selection end too?
    app_selection.end.checked_sub(app_window_start) == Some(element_selection_end)
}
