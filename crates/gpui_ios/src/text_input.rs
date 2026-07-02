//! The hidden UIKit text-input view that bridges the software keyboard to
//! gpui's `PlatformInputHandler`.
//!
//! `GPUIView` itself must not conform to the text-input protocols: UIKit
//! summons the software keyboard for any first responder conforming to
//! `UIKeyInput`, and the keyboard should only appear while gpui focuses an
//! editable. So text input lives on this separate zero-size subview, and the
//! window makes it first responder exactly while the focused gpui element
//! accepts text input (see `update_text_input_responder` in `window.rs`).

use crate::{
    id,
    window::{WINDOW_STATE_IVAR, dispatch_input, string_from_ns_string, with_input_handler},
};
use gpui::{KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, PlatformInput};
use objc::{
    class,
    declare::ClassDecl,
    runtime::{BOOL, Class, Object, Protocol, Sel, YES},
    sel, sel_impl,
};
use std::{ffi::c_void, sync::Once};

// `UITextInputTraits` enum values. The system keyboard's autocorrection,
// autocapitalization, and smart substitutions would rewrite text behind the
// focused gpui element's back (and make injected test input nondeterministic),
// so they are all switched off.
const UI_TEXT_AUTOCAPITALIZATION_TYPE_NONE: i64 = 0;
const UI_TEXT_AUTOCORRECTION_TYPE_NO: i64 = 1;
const UI_TEXT_SMART_QUOTES_TYPE_NO: i64 = 1;
const UI_TEXT_SMART_DASHES_TYPE_NO: i64 = 1;
const UI_TEXT_SMART_INSERT_DELETE_TYPE_NO: i64 = 1;

pub(crate) fn text_input_view_class() -> &'static Class {
    static REGISTER: Once = Once::new();
    REGISTER.call_once(|| {
        let mut decl = ClassDecl::new("GPUITextInputView", class!(UIView))
            .expect("GPUITextInputView class is already registered");
        decl.add_ivar::<*mut c_void>(WINDOW_STATE_IVAR);
        decl.add_protocol(
            Protocol::get("UIKeyInput").expect("UIKeyInput protocol is registered by UIKit"),
        );
        unsafe {
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
        }
        decl.register();
    });
    Class::get("GPUITextInputView").expect("GPUITextInputView was just registered")
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

fn synthesize_keystroke(this: &Object, key: &str, key_char: Option<String>) {
    let window_state = unsafe { crate::window::get_window_state(this) };
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
}
