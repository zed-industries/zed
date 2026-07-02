//! The hidden UIKit text-input view that bridges the software keyboard and
//! IME composition to gpui's `PlatformInputHandler`.
//!
//! `GPUIView` itself must not conform to the text-input protocols: UIKit
//! summons the software keyboard for any first responder conforming to
//! `UIKeyInput`, and the keyboard should only appear while gpui focuses an
//! editable. So text input lives on this separate zero-size subview, and the
//! window makes it first responder exactly while the focused gpui element
//! accepts text input (see `update_text_input_responder` in `window.rs`).
//!
//! All positions and ranges exchanged with UIKit are UTF-16 code-unit
//! offsets, matching `PlatformInputHandler`'s convention, so the bridge does
//! no unit conversion.

use crate::{
    CGPoint, CGRect, CGSize, id, nil, ns_string,
    window::{
        WINDOW_STATE_IVAR, dispatch_input, get_window_state, string_from_ns_string,
        with_input_handler,
    },
};
use gpui::{
    Bounds, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, Pixels, PlatformInput,
    PlatformInputHandler, point, px,
};
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{BOOL, Class, NO, Object, Protocol, Sel, YES},
    sel, sel_impl,
};
use std::{ffi::c_void, ops::Range, sync::Once};

// `UITextInputTraits` enum values. The system keyboard's autocorrection,
// autocapitalization, and smart substitutions would rewrite text behind the
// focused gpui element's back (and make injected test input nondeterministic),
// so they are all switched off.
const UI_TEXT_AUTOCAPITALIZATION_TYPE_NONE: i64 = 0;
const UI_TEXT_AUTOCORRECTION_TYPE_NO: i64 = 1;
const UI_TEXT_SMART_QUOTES_TYPE_NO: i64 = 1;
const UI_TEXT_SMART_DASHES_TYPE_NO: i64 = 1;
const UI_TEXT_SMART_INSERT_DELETE_TYPE_NO: i64 = 1;

// `UITextLayoutDirection` values.
const UI_TEXT_LAYOUT_DIRECTION_RIGHT: i64 = 2;
const UI_TEXT_LAYOUT_DIRECTION_LEFT: i64 = 3;
const UI_TEXT_LAYOUT_DIRECTION_UP: i64 = 4;
const UI_TEXT_LAYOUT_DIRECTION_DOWN: i64 = 5;

// `NSComparisonResult` values.
const NS_ORDERED_ASCENDING: i64 = -1;
const NS_ORDERED_SAME: i64 = 0;
const NS_ORDERED_DESCENDING: i64 = 1;

const NS_WRITING_DIRECTION_NATURAL: i64 = -1;
const NS_WRITING_DIRECTION_LEFT_TO_RIGHT: i64 = 0;

const NS_NOT_FOUND: u64 = i64::MAX as u64;

const TEXT_OFFSET_IVAR: &str = "textOffset";
const RANGE_START_IVAR: &str = "rangeStart";
const RANGE_END_IVAR: &str = "rangeEnd";
const INPUT_DELEGATE_IVAR: &str = "inputDelegate";
const TOKENIZER_IVAR: &str = "tokenizer";
const SELECTION_RECT_IVAR: &str = "selectionRect";
const CONTAINS_START_IVAR: &str = "containsStart";
const CONTAINS_END_IVAR: &str = "containsEnd";

#[repr(C)]
#[derive(Clone, Copy)]
struct NSRange {
    location: u64,
    length: u64,
}

unsafe impl objc::Encode for NSRange {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("{_NSRange=QQ}") }
    }
}

pub(crate) fn text_input_view_class() -> &'static Class {
    static REGISTER: Once = Once::new();
    REGISTER.call_once(|| {
        let mut decl = ClassDecl::new("GPUITextInputView", class!(UIView))
            .expect("GPUITextInputView class is already registered");
        decl.add_ivar::<*mut c_void>(WINDOW_STATE_IVAR);
        decl.add_ivar::<id>(INPUT_DELEGATE_IVAR);
        decl.add_ivar::<id>(TOKENIZER_IVAR);
        decl.add_protocol(
            Protocol::get("UIKeyInput").expect("UIKeyInput protocol is registered by UIKit"),
        );
        decl.add_protocol(
            Protocol::get("UITextInput").expect("UITextInput protocol is registered by UIKit"),
        );
        unsafe {
            decl.add_method(sel!(dealloc), dealloc as extern "C" fn(&Object, Sel));
            decl.add_method(
                sel!(canBecomeFirstResponder),
                can_become_first_responder as extern "C" fn(&Object, Sel) -> BOOL,
            );
            decl.add_method(
                sel!(hasText),
                has_text as extern "C" fn(&Object, Sel) -> BOOL,
            );
            decl.add_method(
                sel!(insertText:),
                insert_text as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(
                sel!(deleteBackward),
                delete_backward as extern "C" fn(&Object, Sel),
            );
            decl.add_method(
                sel!(autocapitalizationType),
                autocapitalization_type as extern "C" fn(&Object, Sel) -> i64,
            );
            decl.add_method(
                sel!(autocorrectionType),
                autocorrection_type as extern "C" fn(&Object, Sel) -> i64,
            );
            decl.add_method(
                sel!(smartQuotesType),
                smart_quotes_type as extern "C" fn(&Object, Sel) -> i64,
            );
            decl.add_method(
                sel!(smartDashesType),
                smart_dashes_type as extern "C" fn(&Object, Sel) -> i64,
            );
            decl.add_method(
                sel!(smartInsertDeleteType),
                smart_insert_delete_type as extern "C" fn(&Object, Sel) -> i64,
            );
            decl.add_method(
                sel!(textInRange:),
                text_in_range as extern "C" fn(&Object, Sel, id) -> id,
            );
            decl.add_method(
                sel!(replaceRange:withText:),
                replace_range_with_text as extern "C" fn(&Object, Sel, id, id),
            );
            decl.add_method(
                sel!(selectedTextRange),
                selected_text_range as extern "C" fn(&Object, Sel) -> id,
            );
            decl.add_method(
                sel!(setSelectedTextRange:),
                set_selected_text_range as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(
                sel!(markedTextRange),
                marked_text_range as extern "C" fn(&Object, Sel) -> id,
            );
            decl.add_method(
                sel!(markedTextStyle),
                marked_text_style as extern "C" fn(&Object, Sel) -> id,
            );
            decl.add_method(
                sel!(setMarkedTextStyle:),
                set_marked_text_style as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(
                sel!(setMarkedText:selectedRange:),
                set_marked_text as extern "C" fn(&Object, Sel, id, NSRange),
            );
            decl.add_method(sel!(unmarkText), unmark_text as extern "C" fn(&Object, Sel));
            decl.add_method(
                sel!(beginningOfDocument),
                beginning_of_document as extern "C" fn(&Object, Sel) -> id,
            );
            decl.add_method(
                sel!(endOfDocument),
                end_of_document as extern "C" fn(&Object, Sel) -> id,
            );
            decl.add_method(
                sel!(textRangeFromPosition:toPosition:),
                text_range_from_positions as extern "C" fn(&Object, Sel, id, id) -> id,
            );
            decl.add_method(
                sel!(positionFromPosition:offset:),
                position_from_position as extern "C" fn(&Object, Sel, id, i64) -> id,
            );
            decl.add_method(
                sel!(positionFromPosition:inDirection:offset:),
                position_from_position_in_direction
                    as extern "C" fn(&Object, Sel, id, i64, i64) -> id,
            );
            decl.add_method(
                sel!(comparePosition:toPosition:),
                compare_positions as extern "C" fn(&Object, Sel, id, id) -> i64,
            );
            decl.add_method(
                sel!(offsetFromPosition:toPosition:),
                offset_from_positions as extern "C" fn(&Object, Sel, id, id) -> i64,
            );
            decl.add_method(
                sel!(inputDelegate),
                input_delegate as extern "C" fn(&Object, Sel) -> id,
            );
            decl.add_method(
                sel!(setInputDelegate:),
                set_input_delegate as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(tokenizer),
                tokenizer as extern "C" fn(&mut Object, Sel) -> id,
            );
            decl.add_method(
                sel!(positionWithinRange:farthestInDirection:),
                position_within_range as extern "C" fn(&Object, Sel, id, i64) -> id,
            );
            decl.add_method(
                sel!(characterRangeByExtendingPosition:inDirection:),
                character_range_by_extending_position as extern "C" fn(&Object, Sel, id, i64) -> id,
            );
            decl.add_method(
                sel!(baseWritingDirectionForPosition:inDirection:),
                base_writing_direction as extern "C" fn(&Object, Sel, id, i64) -> i64,
            );
            decl.add_method(
                sel!(setBaseWritingDirection:forRange:),
                set_base_writing_direction as extern "C" fn(&Object, Sel, i64, id),
            );
            decl.add_method(
                sel!(firstRectForRange:),
                first_rect_for_range as extern "C" fn(&Object, Sel, id) -> CGRect,
            );
            decl.add_method(
                sel!(caretRectForPosition:),
                caret_rect_for_position as extern "C" fn(&Object, Sel, id) -> CGRect,
            );
            decl.add_method(
                sel!(selectionRectsForRange:),
                selection_rects_for_range as extern "C" fn(&Object, Sel, id) -> id,
            );
            decl.add_method(
                sel!(closestPositionToPoint:),
                closest_position_to_point as extern "C" fn(&Object, Sel, CGPoint) -> id,
            );
            decl.add_method(
                sel!(closestPositionToPoint:withinRange:),
                closest_position_to_point_within_range
                    as extern "C" fn(&Object, Sel, CGPoint, id) -> id,
            );
            decl.add_method(
                sel!(characterRangeAtPoint:),
                character_range_at_point as extern "C" fn(&Object, Sel, CGPoint) -> id,
            );
            decl.add_method(
                sel!(textInputView),
                text_input_view_property as extern "C" fn(&Object, Sel) -> id,
            );
            decl.add_method(
                sel!(canPerformAction:withSender:),
                can_perform_action as extern "C" fn(&Object, Sel, Sel, id) -> BOOL,
            );
            decl.add_method(sel!(cut:), perform_cut as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(copy:), perform_copy as extern "C" fn(&Object, Sel, id));
            decl.add_method(
                sel!(paste:),
                perform_paste as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(
                sel!(selectAll:),
                perform_select_all as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(
                sel!(respondsToSelector:),
                responds_to_selector as extern "C" fn(&Object, Sel, Sel) -> BOOL,
            );
        }
        decl.register();
    });
    Class::get("GPUITextInputView").expect("GPUITextInputView was just registered")
}

/// `UITextPosition` subclass carrying a UTF-16 offset into the document.
fn text_position_class() -> &'static Class {
    static REGISTER: Once = Once::new();
    REGISTER.call_once(|| {
        let mut decl = ClassDecl::new("GPUITextPosition", class!(UITextPosition))
            .expect("GPUITextPosition class is already registered");
        decl.add_ivar::<u64>(TEXT_OFFSET_IVAR);
        decl.register();
    });
    Class::get("GPUITextPosition").expect("GPUITextPosition was just registered")
}

/// `UITextRange` subclass carrying UTF-16 offsets. `UITextRange` is abstract:
/// `start`, `end`, and `isEmpty` must be overridden.
fn text_range_class() -> &'static Class {
    static REGISTER: Once = Once::new();
    REGISTER.call_once(|| {
        let mut decl = ClassDecl::new("GPUITextRange", class!(UITextRange))
            .expect("GPUITextRange class is already registered");
        decl.add_ivar::<u64>(RANGE_START_IVAR);
        decl.add_ivar::<u64>(RANGE_END_IVAR);
        unsafe {
            decl.add_method(
                sel!(start),
                range_start as extern "C" fn(&Object, Sel) -> id,
            );
            decl.add_method(sel!(end), range_end as extern "C" fn(&Object, Sel) -> id);
            decl.add_method(
                sel!(isEmpty),
                range_is_empty as extern "C" fn(&Object, Sel) -> BOOL,
            );
        }
        decl.register();
    });
    Class::get("GPUITextRange").expect("GPUITextRange was just registered")
}

extern "C" fn range_start(this: &Object, _: Sel) -> id {
    let start = unsafe { *this.get_ivar::<u64>(RANGE_START_IVAR) };
    unsafe { text_position(start as usize) }
}

extern "C" fn range_end(this: &Object, _: Sel) -> id {
    let end = unsafe { *this.get_ivar::<u64>(RANGE_END_IVAR) };
    unsafe { text_position(end as usize) }
}

extern "C" fn range_is_empty(this: &Object, _: Sel) -> BOOL {
    let (start, end) = unsafe {
        (
            *this.get_ivar::<u64>(RANGE_START_IVAR),
            *this.get_ivar::<u64>(RANGE_END_IVAR),
        )
    };
    if start == end { YES } else { NO }
}

unsafe fn text_position(offset: usize) -> id {
    unsafe {
        let position: id = msg_send![text_position_class(), alloc];
        let position: id = msg_send![position, init];
        (*position).set_ivar::<u64>(TEXT_OFFSET_IVAR, offset as u64);
        msg_send![position, autorelease]
    }
}

/// Reads the offset out of a position UIKit handed back. UIKit only passes
/// positions this bridge created, but a defensive class check avoids reading
/// a garbage ivar if that assumption ever breaks.
unsafe fn position_offset(position: id) -> Option<usize> {
    if position.is_null() {
        return None;
    }
    unsafe {
        let is_text_position: BOOL = msg_send![position, isKindOfClass: text_position_class()];
        (is_text_position == YES).then(|| *(*position).get_ivar::<u64>(TEXT_OFFSET_IVAR) as usize)
    }
}

unsafe fn text_range(range: Range<usize>) -> id {
    unsafe {
        let text_range: id = msg_send![text_range_class(), alloc];
        let text_range: id = msg_send![text_range, init];
        (*text_range).set_ivar::<u64>(RANGE_START_IVAR, range.start as u64);
        (*text_range).set_ivar::<u64>(RANGE_END_IVAR, range.end as u64);
        msg_send![text_range, autorelease]
    }
}

unsafe fn range_offsets(range: id) -> Option<Range<usize>> {
    if range.is_null() {
        return None;
    }
    unsafe {
        let is_text_range: BOOL = msg_send![range, isKindOfClass: text_range_class()];
        (is_text_range == YES).then(|| {
            let start = *(*range).get_ivar::<u64>(RANGE_START_IVAR) as usize;
            let end = *(*range).get_ivar::<u64>(RANGE_END_IVAR) as usize;
            start..end
        })
    }
}

/// `InputHandler` has no document-length query, so the length is recovered
/// from a whole-document `text_for_range`: handlers clamp the range and
/// report the clamped result through `adjusted`. The probe uses `i32::MAX`
/// rather than `usize::MAX` so handlers doing arithmetic on the endpoint
/// can't overflow.
pub(crate) fn document_length_utf16(input_handler: &mut PlatformInputHandler) -> usize {
    let mut adjusted = None;
    let Some(text) = input_handler.text_for_range(0..i32::MAX as usize, &mut adjusted) else {
        return 0;
    };
    adjusted.map_or_else(|| text.encode_utf16().count(), |range| range.end)
}

fn view_document_length_utf16(this: &Object) -> usize {
    with_input_handler(this, document_length_utf16).unwrap_or(0)
}

extern "C" fn dealloc(this: &Object, _: Sel) {
    unsafe {
        let tokenizer: id = *this.get_ivar(TOKENIZER_IVAR);
        if !tokenizer.is_null() {
            let _: () = msg_send![tokenizer, release];
        }
        let _: () = msg_send![super(this, class!(UIView)), dealloc];
    }
}

extern "C" fn can_become_first_responder(_this: &Object, _: Sel) -> BOOL {
    YES
}

extern "C" fn has_text(this: &Object, _: Sel) -> BOOL {
    let has_text = with_input_handler(this, |input_handler| {
        input_handler
            .text_for_range(0..1, &mut None)
            .is_some_and(|text| !text.is_empty())
    })
    .unwrap_or(false);
    has_text as BOOL
}

extern "C" fn insert_text(this: &Object, _: Sel, text: id) {
    let text = unsafe { string_from_ns_string(text) };
    eprintln!("[ti] insertText: {text:?}");
    if text == "\n" {
        // The software keyboard's return key arrives here rather than as a
        // press event, but gpui elements handle enter through key bindings
        // (as they do for the hardware key). Synthesizing the KeyDown lets
        // bindings run; gpui inserts the newline itself if none consume it.
        synthesize_keystroke(this, "enter", Some("\n".to_string()));
    } else {
        with_input_handler(this, |input_handler| {
            input_handler.replace_text_in_range(None, &text)
        });
    }
}

extern "C" fn delete_backward(this: &Object, _: Sel) {
    // The software keyboard's delete key produces no press event, so no
    // KeyDown reaches gpui through the `presses*` path. gpui editors delete
    // via their `backspace` key binding (on macOS the hardware key arrives as
    // a KeyDown the editor handles), so synthesize that KeyDown instead of
    // mutating text through the input handler, which would bypass the focused
    // element's own deletion logic.
    synthesize_keystroke(this, "backspace", None);
}

extern "C" fn autocapitalization_type(_this: &Object, _: Sel) -> i64 {
    UI_TEXT_AUTOCAPITALIZATION_TYPE_NONE
}

extern "C" fn autocorrection_type(_this: &Object, _: Sel) -> i64 {
    UI_TEXT_AUTOCORRECTION_TYPE_NO
}

extern "C" fn smart_quotes_type(_this: &Object, _: Sel) -> i64 {
    UI_TEXT_SMART_QUOTES_TYPE_NO
}

extern "C" fn smart_dashes_type(_this: &Object, _: Sel) -> i64 {
    UI_TEXT_SMART_DASHES_TYPE_NO
}

extern "C" fn smart_insert_delete_type(_this: &Object, _: Sel) -> i64 {
    UI_TEXT_SMART_INSERT_DELETE_TYPE_NO
}

extern "C" fn text_in_range(this: &Object, _: Sel, range: id) -> id {
    let Some(range) = (unsafe { range_offsets(range) }) else {
        return nil;
    };
    with_input_handler(this, |input_handler| {
        input_handler.text_for_range(range, &mut None)
    })
    .flatten()
    .map_or(nil, |text| unsafe { ns_string(&text) })
}

extern "C" fn replace_range_with_text(this: &Object, _: Sel, range: id, text: id) {
    let Some(range) = (unsafe { range_offsets(range) }) else {
        return;
    };
    let text = unsafe { string_from_ns_string(text) };
    eprintln!("[ti] replaceRange:{range:?} withText: {text:?}");
    with_input_handler(this, |input_handler| {
        input_handler.replace_text_in_range(Some(range), &text)
    });
}

extern "C" fn selected_text_range(this: &Object, _: Sel) -> id {
    with_input_handler(this, |input_handler| {
        input_handler.selected_text_range(false)
    })
    .flatten()
    .map_or(nil, |selection| unsafe { text_range(selection.range) })
}

extern "C" fn set_selected_text_range(this: &Object, _: Sel, range: id) {
    eprintln!("[ti] setSelectedTextRange: {:?}", unsafe {
        range_offsets(range)
    });
    let range = if range.is_null() {
        // UIKit passes nil to clear the selection. gpui has no "no selection"
        // state for a focused editable, so collapse to a caret at the
        // selection's end.
        let Some(selection) = with_input_handler(this, |input_handler| {
            input_handler.selected_text_range(false)
        })
        .flatten() else {
            return;
        };
        selection.range.end..selection.range.end
    } else {
        let Some(range) = (unsafe { range_offsets(range) }) else {
            return;
        };
        range
    };
    with_input_handler(this, |input_handler| {
        input_handler.set_selected_text_range(range)
    });
}

extern "C" fn marked_text_range(this: &Object, _: Sel) -> id {
    with_input_handler(this, |input_handler| input_handler.marked_text_range())
        .flatten()
        .map_or(nil, |range| unsafe { text_range(range) })
}

extern "C" fn marked_text_style(_this: &Object, _: Sel) -> id {
    nil
}

extern "C" fn set_marked_text_style(_this: &Object, _: Sel, _style: id) {}

extern "C" fn set_marked_text(this: &Object, _: Sel, text: id, selected_range: NSRange) {
    let text = unsafe { string_from_ns_string(text) };
    eprintln!("[ti] setMarkedText: {text:?}");
    // The selection is relative to the marked text, matching what
    // `replace_and_mark_text_in_range` expects.
    let new_selected_range = if selected_range.location == NS_NOT_FOUND {
        None
    } else {
        let start = selected_range.location as usize;
        Some(start..start + selected_range.length as usize)
    };
    with_input_handler(this, |input_handler| {
        input_handler.replace_and_mark_text_in_range(None, &text, new_selected_range)
    });
}

extern "C" fn unmark_text(this: &Object, _: Sel) {
    with_input_handler(this, |input_handler| input_handler.unmark_text());
}

extern "C" fn beginning_of_document(_this: &Object, _: Sel) -> id {
    unsafe { text_position(0) }
}

extern "C" fn end_of_document(this: &Object, _: Sel) -> id {
    unsafe { text_position(view_document_length_utf16(this)) }
}

extern "C" fn text_range_from_positions(
    _this: &Object,
    _: Sel,
    from_position: id,
    to_position: id,
) -> id {
    let (Some(from), Some(to)) = (unsafe { position_offset(from_position) }, unsafe {
        position_offset(to_position)
    }) else {
        return nil;
    };
    unsafe { text_range(from.min(to)..from.max(to)) }
}

extern "C" fn position_from_position(this: &Object, _: Sel, position: id, offset: i64) -> id {
    let Some(from) = (unsafe { position_offset(position) }) else {
        return nil;
    };
    let new_offset = from as i64 + offset;
    // UIKit probes document edges by stepping past them and expects nil back.
    if new_offset < 0 || new_offset > view_document_length_utf16(this) as i64 {
        return nil;
    }
    unsafe { text_position(new_offset as usize) }
}

extern "C" fn position_from_position_in_direction(
    this: &Object,
    sel: Sel,
    position: id,
    direction: i64,
    offset: i64,
) -> id {
    match direction {
        UI_TEXT_LAYOUT_DIRECTION_RIGHT => position_from_position(this, sel, position, offset),
        UI_TEXT_LAYOUT_DIRECTION_LEFT => position_from_position(this, sel, position, -offset),
        // The document has no layout knowledge here, so vertical movement
        // stays in place rather than guessing a line width.
        UI_TEXT_LAYOUT_DIRECTION_UP | UI_TEXT_LAYOUT_DIRECTION_DOWN => position,
        _ => nil,
    }
}

extern "C" fn compare_positions(_this: &Object, _: Sel, position: id, other: id) -> i64 {
    let (Some(first), Some(second)) = (unsafe { position_offset(position) }, unsafe {
        position_offset(other)
    }) else {
        return NS_ORDERED_SAME;
    };
    match first.cmp(&second) {
        std::cmp::Ordering::Less => NS_ORDERED_ASCENDING,
        std::cmp::Ordering::Equal => NS_ORDERED_SAME,
        std::cmp::Ordering::Greater => NS_ORDERED_DESCENDING,
    }
}

extern "C" fn offset_from_positions(
    _this: &Object,
    _: Sel,
    from_position: id,
    to_position: id,
) -> i64 {
    let (Some(from), Some(to)) = (unsafe { position_offset(from_position) }, unsafe {
        position_offset(to_position)
    }) else {
        return 0;
    };
    to as i64 - from as i64
}

extern "C" fn input_delegate(this: &Object, _: Sel) -> id {
    unsafe { *this.get_ivar(INPUT_DELEGATE_IVAR) }
}

extern "C" fn set_input_delegate(this: &mut Object, _: Sel, delegate: id) {
    // The protocol declares the delegate weak; UIKit installs and removes it
    // around responder changes, so it's stored unretained.
    unsafe { this.set_ivar(INPUT_DELEGATE_IVAR, delegate) };
}

extern "C" fn tokenizer(this: &mut Object, _: Sel) -> id {
    unsafe {
        let existing: id = *this.get_ivar(TOKENIZER_IVAR);
        if !existing.is_null() {
            return existing;
        }
        let tokenizer: id = msg_send![class!(UITextInputStringTokenizer), alloc];
        let tokenizer: id = msg_send![tokenizer, initWithTextInput: this as *mut Object as id];
        this.set_ivar(TOKENIZER_IVAR, tokenizer);
        tokenizer
    }
}

extern "C" fn position_within_range(_this: &Object, _: Sel, range: id, direction: i64) -> id {
    let Some(range) = (unsafe { range_offsets(range) }) else {
        return nil;
    };
    let offset = match direction {
        UI_TEXT_LAYOUT_DIRECTION_LEFT | UI_TEXT_LAYOUT_DIRECTION_UP => range.start,
        UI_TEXT_LAYOUT_DIRECTION_RIGHT | UI_TEXT_LAYOUT_DIRECTION_DOWN => range.end,
        _ => return nil,
    };
    unsafe { text_position(offset) }
}

extern "C" fn character_range_by_extending_position(
    this: &Object,
    _: Sel,
    position: id,
    direction: i64,
) -> id {
    let Some(offset) = (unsafe { position_offset(position) }) else {
        return nil;
    };
    let range = match direction {
        UI_TEXT_LAYOUT_DIRECTION_LEFT | UI_TEXT_LAYOUT_DIRECTION_UP => 0..offset,
        UI_TEXT_LAYOUT_DIRECTION_RIGHT | UI_TEXT_LAYOUT_DIRECTION_DOWN => {
            offset..view_document_length_utf16(this)
        }
        _ => return nil,
    };
    unsafe { text_range(range) }
}

extern "C" fn base_writing_direction(
    _this: &Object,
    _: Sel,
    _position: id,
    _direction: i64,
) -> i64 {
    NS_WRITING_DIRECTION_NATURAL
}

extern "C" fn set_base_writing_direction(_this: &Object, _: Sel, _direction: i64, _range: id) {}

extern "C" fn first_rect_for_range(this: &Object, _: Sel, range: id) -> CGRect {
    let Some(range) = (unsafe { range_offsets(range) }) else {
        return CGRect::default();
    };
    bounds_to_local_rect(
        this,
        with_input_handler(this, |input_handler| input_handler.bounds_for_range(range)).flatten(),
    )
}

extern "C" fn caret_rect_for_position(this: &Object, _: Sel, position: id) -> CGRect {
    let Some(offset) = (unsafe { position_offset(position) }) else {
        return CGRect::default();
    };
    let mut rect = bounds_to_local_rect(
        this,
        with_input_handler(this, |input_handler| {
            input_handler.bounds_for_range(offset..offset)
        })
        .flatten(),
    );
    // An empty range has zero-width bounds, but UIKit expects a caret rect
    // with the caret's drawn width (it hit-tests taps against it to decide
    // when a tap lands on the caret).
    if rect.size.width == 0. {
        rect.size.width = 2.;
    }
    rect
}

/// This view is `textInputView`, so UIKit exchanges all `UITextInput`
/// geometry in its local coordinates, while gpui reports bounds in window
/// coordinates. The view is framed over the focused element within the
/// full-screen `GPUIView` (whose coordinates coincide with the window's, one
/// UIKit point per gpui logical pixel), so converting is a translation by
/// the view's frame origin.
fn view_origin_in_window(this: &Object) -> CGPoint {
    let frame: CGRect = unsafe { msg_send![this, frame] };
    frame.origin
}

fn bounds_to_local_rect(this: &Object, bounds: Option<Bounds<Pixels>>) -> CGRect {
    let Some(bounds) = bounds else {
        return CGRect::default();
    };
    let origin = view_origin_in_window(this);
    CGRect {
        origin: CGPoint {
            x: bounds.origin.x.as_f32() as f64 - origin.x,
            y: bounds.origin.y.as_f32() as f64 - origin.y,
        },
        size: CGSize {
            width: bounds.size.width.as_f32() as f64,
            height: bounds.size.height.as_f32() as f64,
        },
    }
}

extern "C" fn selection_rects_for_range(this: &Object, _: Sel, range: id) -> id {
    let rect = unsafe { range_offsets(range) }.and_then(|range| {
        // An empty (caret) range still gets its zero-width rect: UIKit
        // anchors the edit menu to these rects, and an empty array would
        // leave the menu positioned off a null rectangle (invisible).
        let is_empty = range.is_empty();
        let bounds =
            with_input_handler(this, |input_handler| input_handler.bounds_for_range(range))
                .flatten()?;
        Some((bounds, is_empty))
    });
    let Some((rect, is_empty)) = rect else {
        return unsafe { msg_send![class!(NSArray), array] };
    };
    let rect = bounds_to_local_rect(this, Some(rect));
    unsafe {
        let selection_rect: id = msg_send![text_selection_rect_class(), alloc];
        let selection_rect: id = msg_send![selection_rect, init];
        (*selection_rect).set_ivar::<CGRect>(SELECTION_RECT_IVAR, rect);
        // The field is single-line, so this one rect spans the whole
        // selection and carries both endpoints (where UIKit anchors the
        // selection handles). A caret rect carries neither, so no handles
        // appear on it.
        let contains_endpoints = if is_empty { NO } else { YES };
        (*selection_rect).set_ivar::<BOOL>(CONTAINS_START_IVAR, contains_endpoints);
        (*selection_rect).set_ivar::<BOOL>(CONTAINS_END_IVAR, contains_endpoints);
        let selection_rect: id = msg_send![selection_rect, autorelease];
        msg_send![class!(NSArray), arrayWithObject: selection_rect]
    }
}

/// `UITextSelectionRect` subclass describing one rectangle of the selection
/// highlight. The base class is abstract: every accessor must be overridden.
fn text_selection_rect_class() -> &'static Class {
    static REGISTER: Once = Once::new();
    REGISTER.call_once(|| {
        let mut decl = ClassDecl::new("GPUITextSelectionRect", class!(UITextSelectionRect))
            .expect("GPUITextSelectionRect class is already registered");
        decl.add_ivar::<CGRect>(SELECTION_RECT_IVAR);
        decl.add_ivar::<BOOL>(CONTAINS_START_IVAR);
        decl.add_ivar::<BOOL>(CONTAINS_END_IVAR);
        unsafe {
            decl.add_method(
                sel!(rect),
                selection_rect_rect as extern "C" fn(&Object, Sel) -> CGRect,
            );
            decl.add_method(
                sel!(writingDirection),
                selection_rect_writing_direction as extern "C" fn(&Object, Sel) -> i64,
            );
            decl.add_method(
                sel!(containsStart),
                selection_rect_contains_start as extern "C" fn(&Object, Sel) -> BOOL,
            );
            decl.add_method(
                sel!(containsEnd),
                selection_rect_contains_end as extern "C" fn(&Object, Sel) -> BOOL,
            );
            decl.add_method(
                sel!(isVertical),
                selection_rect_is_vertical as extern "C" fn(&Object, Sel) -> BOOL,
            );
        }
        decl.register();
    });
    Class::get("GPUITextSelectionRect").expect("GPUITextSelectionRect was just registered")
}

extern "C" fn selection_rect_rect(this: &Object, _: Sel) -> CGRect {
    unsafe { *this.get_ivar::<CGRect>(SELECTION_RECT_IVAR) }
}

extern "C" fn selection_rect_writing_direction(_this: &Object, _: Sel) -> i64 {
    NS_WRITING_DIRECTION_LEFT_TO_RIGHT
}

extern "C" fn selection_rect_contains_start(this: &Object, _: Sel) -> BOOL {
    unsafe { *this.get_ivar::<BOOL>(CONTAINS_START_IVAR) }
}

extern "C" fn selection_rect_contains_end(this: &Object, _: Sel) -> BOOL {
    unsafe { *this.get_ivar::<BOOL>(CONTAINS_END_IVAR) }
}

extern "C" fn selection_rect_is_vertical(_this: &Object, _: Sel) -> BOOL {
    NO
}

extern "C" fn closest_position_to_point(this: &Object, _: Sel, position: CGPoint) -> id {
    let index = character_index_for_point(this, position).unwrap_or_else(|| {
        // Past the end of the text (where the handler reports no character),
        // the closest position is the document end.
        view_document_length_utf16(this)
    });
    eprintln!(
        "[ti] closestPositionToPoint: ({}, {}) -> {index}",
        position.x, position.y
    );
    unsafe { text_position(index) }
}

extern "C" fn closest_position_to_point_within_range(
    this: &Object,
    _: Sel,
    position: CGPoint,
    range: id,
) -> id {
    let Some(range) = (unsafe { range_offsets(range) }) else {
        return nil;
    };
    let index = character_index_for_point(this, position)
        .unwrap_or(range.end)
        .clamp(range.start, range.end);
    eprintln!(
        "[ti] closestPositionToPoint:withinRange: ({}, {}) {range:?} -> {index}",
        position.x, position.y
    );
    unsafe { text_position(index) }
}

extern "C" fn character_range_at_point(this: &Object, _: Sel, position: CGPoint) -> id {
    let Some(index) = character_index_for_point(this, position) else {
        return nil;
    };
    // The range must span the character under the point, not collapse to a
    // caret: UIKit reconciles this range with `closestPositionToPoint:` when
    // placing the caret, and an inconsistent answer makes repeated taps at
    // one spot oscillate between offsets (defeating, for example, the
    // tap-on-caret edit-menu gesture). The character's UTF-16 length is
    // recovered from the text itself to keep surrogate pairs intact.
    let character_utf16_length = with_input_handler(this, |input_handler| {
        let text = input_handler.text_for_range(index..index + 2, &mut None)?;
        Some(
            text.chars()
                .next()
                .map_or(0, |character| character.len_utf16()),
        )
    })
    .flatten()
    .unwrap_or(0);
    eprintln!(
        "[ti] characterRangeAtPoint -> {index}..{}",
        index + character_utf16_length
    );
    unsafe { text_range(index..index + character_utf16_length) }
}

fn character_index_for_point(this: &Object, position: CGPoint) -> Option<usize> {
    let origin = view_origin_in_window(this);
    with_input_handler(this, |input_handler| {
        input_handler.character_index_for_point(point(
            px((position.x + origin.x) as f32),
            px((position.y + origin.y) as f32),
        ))
    })
    .flatten()
}

extern "C" fn text_input_view_property(this: &Object, _: Sel) -> id {
    this as *const Object as id
}

fn selected_range(this: &Object) -> Option<Range<usize>> {
    with_input_handler(this, |input_handler| {
        input_handler.selected_text_range(false)
    })
    .flatten()
    .map(|selection| selection.range)
}

fn write_to_pasteboard(text: &str) {
    unsafe {
        let pasteboard: id = msg_send![class!(UIPasteboard), generalPasteboard];
        let _: () = msg_send![pasteboard, setString: ns_string(text)];
    }
}

extern "C" fn responds_to_selector(this: &Object, _: Sel, selector: Sel) -> BOOL {
    let responds: BOOL =
        unsafe { msg_send![super(this, class!(UIView)), respondsToSelector: selector] };
    eprintln!(
        "[ti] respondsToSelector: {} -> {}",
        selector.name(),
        responds == YES
    );
    responds
}

extern "C" fn can_perform_action(this: &Object, _: Sel, action: Sel, sender: id) -> BOOL {
    eprintln!("[ti] canPerformAction: {}", action.name());
    if action == sel!(cut:) || action == sel!(copy:) {
        return selected_range(this).is_some_and(|range| !range.is_empty()) as BOOL;
    }
    if action == sel!(paste:) {
        let has_strings: BOOL = unsafe {
            let pasteboard: id = msg_send![class!(UIPasteboard), generalPasteboard];
            msg_send![pasteboard, hasStrings]
        };
        return has_strings;
    }
    if action == sel!(selectAll:) {
        let length = view_document_length_utf16(this);
        return (length > 0 && selected_range(this) != Some(0..length)) as BOOL;
    }
    unsafe { msg_send![super(this, class!(UIView)), canPerformAction: action withSender: sender] }
}

extern "C" fn perform_copy(this: &Object, _: Sel, _sender: id) {
    let Some(range) = selected_range(this).filter(|range| !range.is_empty()) else {
        return;
    };
    let text = with_input_handler(this, |input_handler| {
        input_handler.text_for_range(range, &mut None)
    })
    .flatten();
    if let Some(text) = text {
        write_to_pasteboard(&text);
    }
}

extern "C" fn perform_cut(this: &Object, _: Sel, _sender: id) {
    let Some(range) = selected_range(this).filter(|range| !range.is_empty()) else {
        return;
    };
    let text = with_input_handler(this, |input_handler| {
        input_handler.text_for_range(range.clone(), &mut None)
    })
    .flatten();
    let Some(text) = text else {
        return;
    };
    write_to_pasteboard(&text);
    with_input_handler(this, |input_handler| {
        input_handler.replace_text_in_range(Some(range), "")
    });
}

extern "C" fn perform_paste(this: &Object, _: Sel, _sender: id) {
    let text = unsafe {
        let pasteboard: id = msg_send![class!(UIPasteboard), generalPasteboard];
        let string: id = msg_send![pasteboard, string];
        if string.is_null() {
            return;
        }
        string_from_ns_string(string)
    };
    let selection = selected_range(this);
    with_input_handler(this, |input_handler| {
        input_handler.replace_text_in_range(selection, &text)
    });
}

extern "C" fn perform_select_all(this: &Object, _: Sel, _sender: id) {
    let length = view_document_length_utf16(this);
    if length == 0 {
        return;
    }
    with_input_handler(this, |input_handler| {
        input_handler.set_selected_text_range(0..length)
    });
}

/// Routes a software-keyboard action through gpui's key-event path. The
/// resulting text mutation happens outside UIKit's own `UIKeyInput` calls, so
/// the input delegate is told the text is changing, keeping the keyboard's
/// autocorrect/composition state in sync.
fn synthesize_keystroke(this: &Object, key: &str, key_char: Option<String>) {
    let delegate: id = unsafe { *this.get_ivar(INPUT_DELEGATE_IVAR) };
    let this_id = this as *const Object as id;
    if !delegate.is_null() {
        let _: () = unsafe { msg_send![delegate, textWillChange: this_id] };
    }
    let window_state = unsafe { get_window_state(this) };
    let keystroke = Keystroke {
        modifiers: Modifiers::default(),
        key: key.to_string(),
        key_char,
    };
    dispatch_input(
        &window_state,
        PlatformInput::KeyDown(KeyDownEvent {
            keystroke: keystroke.clone(),
            is_held: false,
            prefer_character_input: false,
        }),
    );
    dispatch_input(
        &window_state,
        PlatformInput::KeyUp(KeyUpEvent { keystroke }),
    );
    if !delegate.is_null() {
        let _: () = unsafe { msg_send![delegate, textDidChange: this_id] };
    }
}
