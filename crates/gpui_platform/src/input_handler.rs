//! Platform text input handling shared by `gpui` and its platform backends.

use crate::ClipboardItem;
use gpui_types::{Bounds, Pixels, Point, px};
use std::{any::Any, ops::Range};

/// A struct representing a selection in a text buffer, in UTF16 characters.
/// This is different from a range because the head may be before the tail.
#[derive(Debug)]
pub struct UTF16Selection {
    /// The range of text in the document this selection corresponds to
    /// in UTF16 characters.
    pub range: Range<usize>,
    /// Whether the head of this selection is at the start (true), or end (false)
    /// of the range
    pub reversed: bool,
}

/// The gpui-side implementation backing a [`PlatformInputHandler`].
///
/// Platform backends only ever call the methods on [`PlatformInputHandler`];
/// `gpui` provides the implementation, which is where the `Window`/`App`
/// dependencies live.
#[expect(missing_docs)]
pub trait PlatformInputHandlerDelegate: Any {
    fn selected_text_range(&mut self, ignore_disabled_input: bool) -> Option<UTF16Selection>;
    fn marked_text_range(&mut self) -> Option<Range<usize>>;
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        adjusted: &mut Option<Range<usize>>,
    ) -> Option<String>;
    fn replace_text_in_range(&mut self, replacement_range: Option<Range<usize>>, text: &str);
    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
    );
    fn unmark_text(&mut self);
    fn paste(&mut self, item: ClipboardItem);
    fn bounds_for_range(&mut self, range_utf16: Range<usize>) -> Option<Bounds<Pixels>>;
    fn apple_press_and_hold_enabled(&mut self) -> bool;
    fn character_index_for_point(&mut self, point: Point<Pixels>) -> Option<usize>;
    fn set_selected_text_range(&mut self, range_utf16: Range<usize>);
    fn element_bounds(&mut self) -> Option<Bounds<Pixels>>;
    fn text_length_utf16(&mut self) -> Option<usize>;
    fn query_accepts_text_input(&mut self) -> bool;
    fn query_prefers_ime_for_printable_keys(&mut self) -> bool;
    fn text_input_editable_range(&mut self) -> Option<Range<usize>>;
    /// Allows `gpui` to recover its concrete delegate for the methods that
    /// require a `Window` or `App`.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Handles text input from the platform's IME system on behalf of a window.
///
/// This is the vocabulary platform backends speak; the behavior is supplied by
/// a [`PlatformInputHandlerDelegate`].
#[allow(dead_code)]
pub struct PlatformInputHandler {
    delegate: Box<dyn PlatformInputHandlerDelegate>,
}

#[allow(dead_code)]
#[expect(missing_docs)]
impl PlatformInputHandler {
    /// Creates a handler from the delegate that implements it.
    pub fn from_delegate(delegate: Box<dyn PlatformInputHandlerDelegate>) -> Self {
        Self { delegate }
    }

    /// Exposes the delegate as `Any` so `gpui` can downcast to its own
    /// implementation for the `Window`/`App` methods.
    pub fn delegate_as_any_mut(&mut self) -> &mut dyn Any {
        self.delegate.as_any_mut()
    }

    pub fn selected_text_range(&mut self, ignore_disabled_input: bool) -> Option<UTF16Selection> {
        self.delegate.selected_text_range(ignore_disabled_input)
    }

    pub fn marked_text_range(&mut self) -> Option<Range<usize>> {
        self.delegate.marked_text_range()
    }

    pub fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        adjusted: &mut Option<Range<usize>>,
    ) -> Option<String> {
        self.delegate.text_for_range(range_utf16, adjusted)
    }

    pub fn replace_text_in_range(&mut self, replacement_range: Option<Range<usize>>, text: &str) {
        self.delegate.replace_text_in_range(replacement_range, text);
    }

    pub fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
    ) {
        self.delegate
            .replace_and_mark_text_in_range(range_utf16, new_text, new_selected_range);
    }

    pub fn unmark_text(&mut self) {
        self.delegate.unmark_text();
    }

    pub fn paste(&mut self, item: ClipboardItem) {
        self.delegate.paste(item);
    }

    pub fn bounds_for_range(&mut self, range_utf16: Range<usize>) -> Option<Bounds<Pixels>> {
        self.delegate.bounds_for_range(range_utf16)
    }

    pub fn apple_press_and_hold_enabled(&mut self) -> bool {
        self.delegate.apple_press_and_hold_enabled()
    }

    pub fn character_index_for_point(&mut self, point: Point<Pixels>) -> Option<usize> {
        self.delegate.character_index_for_point(point)
    }

    pub fn set_selected_text_range(&mut self, range_utf16: Range<usize>) {
        self.delegate.set_selected_text_range(range_utf16);
    }

    pub fn element_bounds(&mut self) -> Option<Bounds<Pixels>> {
        self.delegate.element_bounds()
    }

    pub fn text_length_utf16(&mut self) -> Option<usize> {
        self.delegate.text_length_utf16()
    }

    pub fn query_accepts_text_input(&mut self) -> bool {
        self.delegate.query_accepts_text_input()
    }

    pub fn query_prefers_ime_for_printable_keys(&mut self) -> bool {
        self.delegate.query_prefers_ime_for_printable_keys()
    }

    pub fn text_input_editable_range(&mut self) -> Option<Range<usize>> {
        self.delegate.text_input_editable_range()
    }

    pub fn compute_ime_candidate_bounds(
        marked_range: Option<Range<usize>>,
        selection: &UTF16Selection,
        mut bounds_for_range: impl FnMut(Range<usize>) -> Option<Bounds<Pixels>>,
    ) -> Option<Bounds<Pixels>> {
        if let Some(marked_range) = marked_range {
            // Default to the start of the marked (composing) range.
            let mut line_start = marked_range.start;

            // Walk backward from the caret looking for a line break. A change in
            // the Y coordinate means we crossed into the previous visual line, so
            // the line start is one position after the break point.
            let caret = selection.range.end;
            if let Some(caret_bounds) = bounds_for_range(caret..caret) {
                for i in (marked_range.start..caret).rev() {
                    if let Some(b) = bounds_for_range(i..i) {
                        if (b.origin.y - caret_bounds.origin.y).abs() > px(0.1) {
                            line_start = i + 1;
                            break;
                        }
                    }
                }
            }
            bounds_for_range(line_start..line_start)
        } else {
            // No active composition — use the selection endpoint.
            let offset = if selection.reversed {
                selection.range.start
            } else {
                selection.range.end
            };
            bounds_for_range(offset..offset)
        }
    }

    pub fn ime_candidate_bounds(&mut self) -> Option<Bounds<Pixels>> {
        let marked_range = self.marked_text_range();
        let selection = self.selected_text_range(true)?;
        Self::compute_ime_candidate_bounds(marked_range, &selection, |range| {
            self.bounds_for_range(range)
        })
    }
}
