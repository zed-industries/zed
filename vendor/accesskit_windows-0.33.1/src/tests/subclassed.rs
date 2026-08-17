// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit::{
    Action, ActionHandler, ActionRequest, ActivationHandler, Node, NodeId, Role, Tree, TreeId,
    TreeUpdate,
};
use once_cell::sync::Lazy;
use windows::{
    Win32::{
        Foundation::*,
        System::LibraryLoader::GetModuleHandleW,
        UI::{Accessibility::*, WindowsAndMessaging::*},
    },
    core::*,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    platform::windows::EventLoopBuilderExtWindows,
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
    window::{Window, WindowId},
};

use super::MUTEX;
use crate::SubclassingAdapter;

const WINDOW_TITLE: &str = "Simple test";

const WINDOW_ID: NodeId = NodeId(0);
const BUTTON_1_ID: NodeId = NodeId(1);
const BUTTON_2_ID: NodeId = NodeId(2);

fn make_button(label: &str) -> Node {
    let mut node = Node::new(Role::Button);
    node.set_label(label);
    node.add_action(Action::Focus);
    node
}

fn get_initial_state() -> TreeUpdate {
    let mut root = Node::new(Role::Window);
    root.set_children(vec![BUTTON_1_ID, BUTTON_2_ID]);
    let button_1 = make_button("Button 1");
    let button_2 = make_button("Button 2");
    TreeUpdate {
        nodes: vec![
            (WINDOW_ID, root),
            (BUTTON_1_ID, button_1),
            (BUTTON_2_ID, button_2),
        ],
        tree: Some(Tree::new(WINDOW_ID)),
        tree_id: TreeId::ROOT,
        focus: BUTTON_1_ID,
    }
}

pub struct NullActionHandler;

impl ActionHandler for NullActionHandler {
    fn do_action(&mut self, _request: ActionRequest) {}
}

struct SimpleActivationHandler;

impl ActivationHandler for SimpleActivationHandler {
    fn request_initial_tree(&mut self) -> Option<TreeUpdate> {
        Some(get_initial_state())
    }
}

// This module uses winit for the purpose of testing with a real third-party
// window implementation that we don't control. However, only one test
// can use winit, because winit only allows an event loop to be created
// once per process. So we end up creating our own window anyway for the
// double-instantiation test.
//
// Also, while these tests don't use the main test harness or show the window,
// they still need to run with the main harness's mutex, to avoid disturbing
// other tests, particularly the focus test.

struct TestApplication;

impl ApplicationHandler<()> for TestApplication {
    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, _: WindowEvent) {}

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title(WINDOW_TITLE)
            .with_visible(false);

        let window = event_loop.create_window(window_attributes).unwrap();
        let hwnd = match window.window_handle().unwrap().as_raw() {
            RawWindowHandle::Win32(handle) => HWND(handle.hwnd.get() as *mut core::ffi::c_void),
            RawWindowHandle::WinRt(_) => unimplemented!(),
            _ => unreachable!(),
        };
        let adapter =
            SubclassingAdapter::new(hwnd, SimpleActivationHandler {}, NullActionHandler {});
        assert!(unsafe { UiaHasServerSideProvider(hwnd) }.as_bool());
        drop(window);
        drop(adapter);
        event_loop.exit();
    }
}

#[test]
fn has_native_uia() {
    let _lock_guard = MUTEX.lock();
    let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();
    let mut state = TestApplication {};
    event_loop.run_app(&mut state).unwrap();
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe { DefWindowProcW(window, message, wparam, lparam) }
}

static WINDOW_CLASS_ATOM: Lazy<u16> = Lazy::new(|| {
    let class_name = w!("AccessKitSubclassTest");

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

fn create_window(title: &str) -> HWND {
    let module = HINSTANCE::from(unsafe { GetModuleHandleW(None).unwrap() });

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
            None,
        )
    }
    .unwrap();
    if window.is_invalid() {
        panic!("{}", Error::from_thread());
    }

    window
}

#[test]
#[should_panic(expected = "already instantiated")]
fn double_instantiate() {
    let _lock_guard = MUTEX.lock();
    let window = create_window(WINDOW_TITLE);
    let _adapter1 =
        SubclassingAdapter::new(window, SimpleActivationHandler {}, NullActionHandler {});
    let _adapter2 =
        SubclassingAdapter::new(window, SimpleActivationHandler {}, NullActionHandler {});
}
