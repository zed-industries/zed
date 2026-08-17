// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit::{ActionHandler, ActivationHandler};
use once_cell::sync::Lazy;
use std::{
    cell::RefCell,
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};
use windows as Windows;
use windows::{
    Win32::{
        Foundation::*,
        Graphics::Gdi::ValidateRect,
        System::{Com::*, LibraryLoader::GetModuleHandleW},
        UI::{Accessibility::*, WindowsAndMessaging::*},
    },
    core::*,
};

use crate::window_handle::WindowHandle;

use super::{
    Adapter,
    context::{ActionHandlerNoMut, ActionHandlerWrapper},
};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

static WINDOW_CLASS_ATOM: Lazy<u16> = Lazy::new(|| {
    let class_name = w!("AccessKitTest");

    let wc = WNDCLASSW {
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW) }.unwrap(),
        hInstance: unsafe { GetModuleHandleW(None) }.unwrap().into(),
        lpszClassName: class_name,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc),
        ..Default::default()
    };

    let atom = unsafe { RegisterClassW(&wc) };
    if atom == 0 {
        panic!("{}", Error::from_thread());
    }
    atom
});

struct WindowState {
    activation_handler: RefCell<Box<dyn ActivationHandler>>,
    adapter: RefCell<Adapter>,
}

unsafe fn get_window_state(window: HWND) -> *const WindowState {
    unsafe { GetWindowLongPtrW(window, GWLP_USERDATA) as _ }
}

fn update_window_focus_state(window: HWND, is_focused: bool) {
    let state = unsafe { &*get_window_state(window) };
    let mut adapter = state.adapter.borrow_mut();
    if let Some(events) = adapter.update_window_focus_state(is_focused) {
        drop(adapter);
        events.raise();
    }
}

struct WindowCreateParams {
    activation_handler: Box<dyn ActivationHandler>,
    action_handler: Arc<dyn ActionHandlerNoMut + Send + Sync>,
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message {
        WM_NCCREATE => {
            let create_struct: &CREATESTRUCTW = unsafe { &mut *(lparam.0 as *mut _) };
            let create_params: Box<WindowCreateParams> =
                unsafe { Box::from_raw(create_struct.lpCreateParams as _) };
            let WindowCreateParams {
                activation_handler,
                action_handler,
            } = *create_params;
            let adapter = Adapter::with_wrapped_action_handler(window, false, action_handler);
            let state = Box::new(WindowState {
                activation_handler: RefCell::new(activation_handler),
                adapter: RefCell::new(adapter),
            });
            unsafe { SetWindowLongPtrW(window, GWLP_USERDATA, Box::into_raw(state) as _) };
            unsafe { DefWindowProcW(window, message, wparam, lparam) }
        }
        WM_PAINT => {
            unsafe { ValidateRect(Some(window), None) }.unwrap();
            LRESULT(0)
        }
        WM_DESTROY => {
            let ptr = unsafe { SetWindowLongPtrW(window, GWLP_USERDATA, 0) };
            if ptr != 0 {
                drop(unsafe { Box::<WindowState>::from_raw(ptr as _) });
            }
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        WM_GETOBJECT => {
            let state_ptr = unsafe { get_window_state(window) };
            if state_ptr.is_null() {
                // We need to be prepared to gracefully handle WM_GETOBJECT
                // while the window is being destroyed; this can happen if
                // the thread is using a COM STA.
                return unsafe { DefWindowProcW(window, message, wparam, lparam) };
            }
            let state = unsafe { &*state_ptr };
            let mut adapter = state.adapter.borrow_mut();
            let mut activation_handler = state.activation_handler.borrow_mut();
            let result = adapter.handle_wm_getobject(wparam, lparam, &mut **activation_handler);
            drop(activation_handler);
            drop(adapter);
            result.map_or_else(
                || unsafe { DefWindowProcW(window, message, wparam, lparam) },
                |result| result.into(),
            )
        }
        WM_SETFOCUS | WM_EXITMENULOOP | WM_EXITSIZEMOVE => {
            update_window_focus_state(window, true);
            LRESULT(0)
        }
        WM_KILLFOCUS | WM_ENTERMENULOOP | WM_ENTERSIZEMOVE => {
            update_window_focus_state(window, false);
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(window, message, wparam, lparam) },
    }
}

fn create_window(
    title: &str,
    activation_handler: impl 'static + ActivationHandler,
    action_handler: impl 'static + ActionHandler + Send,
) -> Result<HWND> {
    let create_params = Box::new(WindowCreateParams {
        activation_handler: Box::new(activation_handler),
        action_handler: Arc::new(ActionHandlerWrapper::new(action_handler)),
    });
    let module = HINSTANCE::from(unsafe { GetModuleHandleW(None)? });

    let window = unsafe {
        CreateWindowExW(
            Default::default(),
            PCWSTR(*WINDOW_CLASS_ATOM as usize as _),
            &HSTRING::from(title),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            Some(module),
            Some(Box::into_raw(create_params) as _),
        )?
    };
    if window.is_invalid() {
        return Err(Error::from_thread());
    }

    Ok(window)
}

pub(crate) struct Scope {
    pub(crate) uia: IUIAutomation,
    pub(crate) window: WindowHandle,
}

impl Scope {
    pub(crate) fn show_and_focus_window(&self) {
        let _ = unsafe { ShowWindow(self.window.0, SW_SHOW) };
        let _ = unsafe { SetForegroundWindow(self.window.0) };
    }
}

// It's not safe to run these UI-related tests concurrently.
// We need a non-poisoning mutex here because the subclassing adapter's
// double-instantiation test intentionally panics.
pub(crate) static MUTEX: parking_lot::Mutex<()> = parking_lot::const_mutex(());

pub(crate) fn scope<F>(
    window_title: &str,
    activation_handler: impl 'static + ActivationHandler + Send,
    action_handler: impl 'static + ActionHandler + Send,
    f: F,
) -> Result<()>
where
    F: FnOnce(&Scope) -> Result<()>,
{
    let _lock_guard = MUTEX.lock();

    let window_mutex: Mutex<Option<WindowHandle>> = Mutex::new(None);
    let window_cv = Condvar::new();

    thread::scope(|thread_scope| {
        thread_scope.spawn(|| {
            // We explicitly don't want to initialize COM on the provider thread,
            // because we want to make sure that the provider side of UIA works
            // even if COM is never initialized on the provider thread
            // (as is the case if the window is never shown), or if COM is
            // initialized after the window is shown (as is the case,
            // at least on some Windows 10 machines, due to IME support).

            let window = create_window(window_title, activation_handler, action_handler).unwrap();

            {
                let mut state = window_mutex.lock().unwrap();
                *state = Some(window.into());
                window_cv.notify_one();
            }

            let mut message = MSG::default();
            while unsafe { GetMessageW(&mut message, None, 0, 0) }.into() {
                let _ = unsafe { TranslateMessage(&message) };
                unsafe { DispatchMessageW(&message) };
            }
        });

        let window = {
            let state = window_mutex.lock().unwrap();
            let mut state = if state.is_none() {
                window_cv.wait(state).unwrap()
            } else {
                state
            };
            state.take().unwrap()
        };

        let _window_guard = scopeguard::guard((), |_| {
            unsafe { PostMessageW(Some(window.0), WM_CLOSE, WPARAM(0), LPARAM(0)) }.unwrap()
        });

        // We must initialize COM before creating the UIA client. The MTA option
        // is cleaner by far, especially when we want to wait for a UIA event
        // handler to be called, and there's no reason not to use it here.
        // Note that we don't initialize COM this way on the provider thread,
        // as explained above. It's also important that we let the provider
        // thread do its forced initialization of UIA, in an environment
        // where COM has not been initialized, before we create the UIA client,
        // which also triggers UIA initialization, in a thread where COM
        // _has_ been initialized. This way, we ensure that the provider side
        // of UIA works even if it is set up in an environment where COM
        // has not been initialized, and that this sequence of events
        // doesn't prevent the UIA client from working.
        unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }.unwrap();
        let _com_guard = scopeguard::guard((), |_| unsafe { CoUninitialize() });

        let uia: IUIAutomation =
            unsafe { CoCreateInstance(&CUIAutomation8, None, CLSCTX_INPROC_SERVER) }?;

        let s = Scope { uia, window };
        f(&s)
    })
}

/// This must only be used to wrap UIA elements returned by a UIA client
/// that was created in the MTA. Those are safe to send between threads.
struct SendableUiaElement(IUIAutomationElement);
unsafe impl Send for SendableUiaElement {}

pub(crate) struct ReceivedFocusEvent {
    mutex: Mutex<Option<SendableUiaElement>>,
    cv: Condvar,
}

impl ReceivedFocusEvent {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            mutex: Mutex::new(None),
            cv: Condvar::new(),
        })
    }

    pub(crate) fn wait<F>(&self, f: F) -> IUIAutomationElement
    where
        F: Fn(&IUIAutomationElement) -> bool,
    {
        let mut received = self.mutex.lock().unwrap();
        loop {
            if let Some(SendableUiaElement(element)) = received.take() {
                if f(&element) {
                    return element;
                }
            }
            let (lock, result) = self.cv.wait_timeout(received, DEFAULT_TIMEOUT).unwrap();
            assert!(!result.timed_out());
            received = lock;
        }
    }

    fn put(&self, element: IUIAutomationElement) {
        let mut received = self.mutex.lock().unwrap();
        *received = Some(SendableUiaElement(element));
        self.cv.notify_one();
    }
}

#[implement(Windows::Win32::UI::Accessibility::IUIAutomationFocusChangedEventHandler)]
pub(crate) struct FocusEventHandler {
    received: Arc<ReceivedFocusEvent>,
}
// Because we create a UIA client in the COM MTA, this event handler
// _will_ be called from a different thread, and possibly multiple threads
// at once.
static_assertions::assert_impl_all!(FocusEventHandler: Send, Sync);

impl FocusEventHandler {
    #[allow(clippy::new_ret_no_self)] // it does return self, but wrapped
    pub(crate) fn new() -> (
        IUIAutomationFocusChangedEventHandler,
        Arc<ReceivedFocusEvent>,
    ) {
        let received = ReceivedFocusEvent::new();
        (
            Self {
                received: Arc::clone(&received),
            }
            .into(),
            received,
        )
    }
}

#[allow(non_snake_case)]
impl IUIAutomationFocusChangedEventHandler_Impl for FocusEventHandler_Impl {
    fn HandleFocusChangedEvent(&self, sender: Ref<IUIAutomationElement>) -> Result<()> {
        self.received.put(sender.unwrap().clone());
        Ok(())
    }
}

mod simple;
mod subclassed;
