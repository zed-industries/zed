// Copyright 2023 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use objc2::{
    encode::{Encode, EncodeArguments, EncodeReturn, Encoding},
    ffi::class_addMethod,
    msg_send,
    runtime::{AnyClass, AnyObject, Bool, MethodImplementation, Sel},
    sel, Message,
};
use objc2_app_kit::NSWindow;
use std::{ffi::CString, ptr::null_mut};

extern "C" fn focus_forwarder(this: &NSWindow, _cmd: Sel) -> *mut AnyObject {
    unsafe {
        this.contentView().map_or_else(null_mut, |view| {
            msg_send![&*view, accessibilityFocusedUIElement]
        })
    }
}

/// Modifies the specified class, which must be a subclass of `NSWindow`,
/// to include an `accessibilityFocusedUIElement` method that calls
/// the corresponding method on the window's content view. This is needed
/// for windowing libraries such as SDL that place the keyboard focus
/// directly on the window rather than the content view.
///
/// # Safety
///
/// This function is declared unsafe because the caller must ensure that the
/// code for this crate is never unloaded from the application process,
/// since it's not possible to reverse this operation. It's safest
/// if this crate is statically linked into the application's main executable.
/// Also, this function assumes that the specified class is a subclass
/// of `NSWindow`.
pub unsafe fn add_focus_forwarder_to_window_class(class_name: &str) {
    let class = AnyClass::get(class_name).unwrap();
    unsafe {
        add_method(
            class as *const AnyClass as *mut AnyClass,
            sel!(accessibilityFocusedUIElement),
            focus_forwarder as unsafe extern "C" fn(_, _) -> _,
        )
    };
}

// The rest of this file is copied from objc2 with only minor adaptations,
// to allow a method to be added to an existing class.

unsafe fn add_method<T, F>(class: *mut AnyClass, sel: Sel, func: F)
where
    T: Message + ?Sized,
    F: MethodImplementation<Callee = T>,
{
    let encs = F::Arguments::ENCODINGS;
    let sel_args = count_args(sel);
    assert_eq!(
        sel_args,
        encs.len(),
        "Selector {:?} accepts {} arguments, but function accepts {}",
        sel,
        sel_args,
        encs.len(),
    );

    let types = method_type_encoding(&F::Return::ENCODING_RETURN, encs);
    let success = Bool::from_raw(unsafe {
        class_addMethod(
            class as *mut _,
            sel.as_ptr(),
            Some(func.__imp()),
            types.as_ptr(),
        )
    });
    assert!(success.as_bool(), "Failed to add method {sel:?}");
}

fn count_args(sel: Sel) -> usize {
    sel.name().chars().filter(|&c| c == ':').count()
}

fn method_type_encoding(ret: &Encoding, args: &[Encoding]) -> CString {
    // First two arguments are always self and the selector
    let mut types = format!("{}{}{}", ret, <*mut AnyObject>::ENCODING, Sel::ENCODING);
    for enc in args {
        use core::fmt::Write;
        write!(&mut types, "{enc}").unwrap();
    }
    CString::new(types).unwrap()
}
