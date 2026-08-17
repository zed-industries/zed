// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit::{ActionHandler, ActivationHandler, TreeUpdate};
use std::{
    cell::{Cell, RefCell},
    ffi::c_void,
    mem::transmute,
};
use windows::{
    Win32::{Foundation::*, UI::WindowsAndMessaging::*},
    core::*,
};

use crate::{Adapter, QueuedEvents};

fn win32_error() -> ! {
    panic!("{}", Error::from_thread())
}

// Work around a difference between the SetWindowLongPtrW API definition
// in windows-rs on 32-bit and 64-bit Windows.
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
type LongPtr = isize;
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
type LongPtr = i32;

const PROP_NAME: PCWSTR = w!("AccessKitAdapter");

struct SubclassState {
    adapter: Adapter,
    activation_handler: Box<dyn ActivationHandler>,
}

struct SubclassImpl {
    hwnd: HWND,
    state: RefCell<SubclassState>,
    prev_wnd_proc: WNDPROC,
    window_destroyed: Cell<bool>,
}

extern "system" fn wnd_proc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let handle = unsafe { GetPropW(window, PROP_NAME) };
    let impl_ptr = handle.0 as *const SubclassImpl;
    assert!(!impl_ptr.is_null());
    let r#impl = unsafe { &*impl_ptr };
    match message {
        WM_GETOBJECT => {
            let mut state = r#impl.state.borrow_mut();
            let state_mut = &mut *state;
            if let Some(result) = state_mut.adapter.handle_wm_getobject(
                wparam,
                lparam,
                &mut *state_mut.activation_handler,
            ) {
                drop(state);
                return result.into();
            }
        }
        WM_SETFOCUS | WM_EXITMENULOOP | WM_EXITSIZEMOVE => {
            r#impl.update_window_focus_state(true);
        }
        WM_KILLFOCUS | WM_ENTERMENULOOP | WM_ENTERSIZEMOVE => {
            r#impl.update_window_focus_state(false);
        }
        WM_NCDESTROY => {
            r#impl.window_destroyed.set(true);
        }
        _ => (),
    }
    unsafe { CallWindowProcW(r#impl.prev_wnd_proc, window, message, wparam, lparam) }
}

impl SubclassImpl {
    fn new(
        hwnd: HWND,
        activation_handler: impl 'static + ActivationHandler,
        action_handler: impl 'static + ActionHandler + Send,
    ) -> Box<Self> {
        let adapter = Adapter::new(hwnd, false, action_handler);
        let state = RefCell::new(SubclassState {
            adapter,
            activation_handler: Box::new(activation_handler),
        });
        Box::new(Self {
            hwnd,
            state,
            prev_wnd_proc: None,
            window_destroyed: Cell::new(false),
        })
    }

    fn install(&mut self) {
        if !unsafe { GetPropW(self.hwnd, PROP_NAME) }.0.is_null() {
            panic!(
                "subclassing adapter already instantiated on window {:?}",
                self.hwnd.0
            );
        }
        unsafe {
            SetPropW(
                self.hwnd,
                PROP_NAME,
                Some(HANDLE(self as *const SubclassImpl as _)),
            )
        }
        .unwrap();
        let result =
            unsafe { SetWindowLongPtrW(self.hwnd, GWLP_WNDPROC, wnd_proc as *const c_void as _) };
        if result == 0 {
            win32_error();
        }
        self.prev_wnd_proc = unsafe { transmute::<LongPtr, WNDPROC>(result) };
    }

    fn update_window_focus_state(&self, is_focused: bool) {
        let mut state = self.state.borrow_mut();
        if let Some(events) = state.adapter.update_window_focus_state(is_focused) {
            drop(state);
            events.raise();
        }
    }

    fn uninstall(&self) {
        if self.window_destroyed.get() {
            return;
        }
        let result = unsafe {
            SetWindowLongPtrW(
                self.hwnd,
                GWLP_WNDPROC,
                transmute::<WNDPROC, LongPtr>(self.prev_wnd_proc),
            )
        };
        if result == 0 {
            win32_error();
        }
        unsafe { RemovePropW(self.hwnd, PROP_NAME) }.unwrap();
    }
}

/// Uses [Win32 subclassing] to handle `WM_GETOBJECT` messages on a window
/// that provides no other way of adding custom message handlers.
///
/// [Win32 subclassing]: https://docs.microsoft.com/en-us/windows/win32/controls/subclassing-overview
pub struct SubclassingAdapter(Box<SubclassImpl>);

impl SubclassingAdapter {
    /// Creates a new Windows platform adapter using window subclassing.
    /// This must be done before the window is shown or focused
    /// for the first time.
    ///
    /// This must be called on the thread that owns the window. The activation
    /// handler will always be called on that thread. The action handler
    /// may or may not be called on that thread.
    ///
    /// # Panics
    ///
    /// Panics if the window is already visible.
    pub fn new(
        hwnd: HWND,
        activation_handler: impl 'static + ActivationHandler,
        action_handler: impl 'static + ActionHandler + Send,
    ) -> Self {
        if unsafe { IsWindowVisible(hwnd) }.into() {
            panic!(
                "The AccessKit Windows subclassing adapter must be created before the window is shown (made visible) for the first time."
            );
        }

        let mut r#impl = SubclassImpl::new(hwnd, activation_handler, action_handler);
        r#impl.install();
        Self(r#impl)
    }

    /// If and only if the tree has been initialized, call the provided function
    /// and apply the resulting update. Note: If the caller's implementation of
    /// [`ActivationHandler::request_initial_tree`] initially returned `None`,
    /// the [`TreeUpdate`] returned by the provided function must contain
    /// a full tree.
    ///
    /// If a [`QueuedEvents`] instance is returned, the caller must call
    /// [`QueuedEvents::raise`] on it.
    pub fn update_if_active(
        &mut self,
        update_factory: impl FnOnce() -> TreeUpdate,
    ) -> Option<QueuedEvents> {
        // SAFETY: We use `RefCell::borrow_mut` here, even though
        // `RefCell::get_mut` is allowed (because this method takes
        // a mutable self reference), just in case there's some way
        // this method can be called from within the subclassed window
        // procedure, e.g. via `ActivationHandler`.
        let mut state = self.0.state.borrow_mut();
        state.adapter.update_if_active(update_factory)
    }
}

impl Drop for SubclassingAdapter {
    fn drop(&mut self) {
        self.0.uninstall();
    }
}
