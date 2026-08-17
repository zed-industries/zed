// Based on the create_window sample in windows-samples-rs.

use accesskit::{
    Action, ActionHandler, ActionRequest, ActivationHandler, Live, Node, NodeId, Rect, Role, Tree,
    TreeId, TreeUpdate,
};
use accesskit_windows::Adapter;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use windows::{
    Win32::{
        Foundation::*,
        Graphics::Gdi::ValidateRect,
        System::LibraryLoader::GetModuleHandleW,
        UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
    },
    core::*,
};

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

const WINDOW_TITLE: &str = "Hello world";

const WINDOW_ID: NodeId = NodeId(0);
const BUTTON_1_ID: NodeId = NodeId(1);
const BUTTON_2_ID: NodeId = NodeId(2);
const ANNOUNCEMENT_ID: NodeId = NodeId(3);
const INITIAL_FOCUS: NodeId = BUTTON_1_ID;

const BUTTON_1_RECT: Rect = Rect {
    x0: 20.0,
    y0: 20.0,
    x1: 100.0,
    y1: 60.0,
};

const BUTTON_2_RECT: Rect = Rect {
    x0: 20.0,
    y0: 60.0,
    x1: 100.0,
    y1: 100.0,
};

const SET_FOCUS_MSG: u32 = WM_USER;
const CLICK_MSG: u32 = WM_USER + 1;

fn build_button(id: NodeId, label: &str) -> Node {
    let rect = match id {
        BUTTON_1_ID => BUTTON_1_RECT,
        BUTTON_2_ID => BUTTON_2_RECT,
        _ => unreachable!(),
    };

    let mut node = Node::new(Role::Button);
    node.set_bounds(rect);
    node.set_label(label);
    node.add_action(Action::Focus);
    node.add_action(Action::Click);
    node
}

fn build_announcement(text: &str) -> Node {
    let mut node = Node::new(Role::Label);
    node.set_value(text);
    node.set_live(Live::Polite);
    node
}

struct InnerWindowState {
    focus: NodeId,
    announcement: Option<String>,
}

impl InnerWindowState {
    fn build_root(&mut self) -> Node {
        let mut node = Node::new(Role::Window);
        node.set_children(vec![BUTTON_1_ID, BUTTON_2_ID]);
        if self.announcement.is_some() {
            node.push_child(ANNOUNCEMENT_ID);
        }
        node.set_language("en");
        node
    }
}

impl ActivationHandler for InnerWindowState {
    fn request_initial_tree(&mut self) -> Option<TreeUpdate> {
        println!("Initial tree requested");
        let root = self.build_root();
        let button_1 = build_button(BUTTON_1_ID, "Button 1");
        let button_2 = build_button(BUTTON_2_ID, "Button 2");
        let tree = Tree::new(WINDOW_ID);

        let mut result = TreeUpdate {
            nodes: vec![
                (WINDOW_ID, root),
                (BUTTON_1_ID, button_1),
                (BUTTON_2_ID, button_2),
            ],
            tree: Some(tree),
            tree_id: TreeId::ROOT,
            focus: self.focus,
        };
        if let Some(announcement) = &self.announcement {
            result
                .nodes
                .push((ANNOUNCEMENT_ID, build_announcement(announcement)));
        }
        Some(result)
    }
}

struct WindowState {
    adapter: RefCell<Adapter>,
    inner_state: RefCell<InnerWindowState>,
}

impl WindowState {
    fn set_focus(&self, focus: NodeId) {
        self.inner_state.borrow_mut().focus = focus;
        let mut adapter = self.adapter.borrow_mut();
        if let Some(events) = adapter.update_if_active(|| TreeUpdate {
            nodes: vec![],
            tree: None,
            tree_id: TreeId::ROOT,
            focus,
        }) {
            drop(adapter);
            events.raise();
        }
    }

    fn press_button(&self, id: NodeId) {
        let mut inner_state = self.inner_state.borrow_mut();
        let text = if id == BUTTON_1_ID {
            "You pressed button 1"
        } else {
            "You pressed button 2"
        };
        inner_state.announcement = Some(text.into());
        let mut adapter = self.adapter.borrow_mut();
        if let Some(events) = adapter.update_if_active(|| {
            let announcement = build_announcement(text);
            let root = inner_state.build_root();
            TreeUpdate {
                nodes: vec![(ANNOUNCEMENT_ID, announcement), (WINDOW_ID, root)],
                tree: None,
                tree_id: TreeId::ROOT,
                focus: inner_state.focus,
            }
        }) {
            drop(adapter);
            drop(inner_state);
            events.raise();
        }
    }
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

struct WindowCreateParams(NodeId);

struct SimpleActionHandler {
    window: HWND,
}

unsafe impl Send for SimpleActionHandler {}
unsafe impl Sync for SimpleActionHandler {}

impl ActionHandler for SimpleActionHandler {
    fn do_action(&mut self, request: ActionRequest) {
        match request.action {
            Action::Focus => {
                unsafe {
                    PostMessageW(
                        Some(self.window),
                        SET_FOCUS_MSG,
                        WPARAM(0),
                        LPARAM(request.target_node.0 as _),
                    )
                }
                .unwrap();
            }
            Action::Click => {
                unsafe {
                    PostMessageW(
                        Some(self.window),
                        CLICK_MSG,
                        WPARAM(0),
                        LPARAM(request.target_node.0 as _),
                    )
                }
                .unwrap();
            }
            _ => (),
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message {
        WM_NCCREATE => {
            let create_struct: &CREATESTRUCTW = unsafe { &mut *(lparam.0 as *mut _) };
            let create_params: Box<WindowCreateParams> =
                unsafe { Box::from_raw(create_struct.lpCreateParams as _) };
            let WindowCreateParams(initial_focus) = *create_params;
            let inner_state = RefCell::new(InnerWindowState {
                focus: initial_focus,
                announcement: None,
            });
            let adapter = Adapter::new(window, false, SimpleActionHandler { window });
            let state = Box::new(WindowState {
                adapter: RefCell::new(adapter),
                inner_state,
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
            let mut inner_state = state.inner_state.borrow_mut();
            let result = adapter.handle_wm_getobject(wparam, lparam, &mut *inner_state);
            drop(inner_state);
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
        WM_KEYDOWN => match VIRTUAL_KEY(wparam.0 as u16) {
            VK_TAB => {
                let state = unsafe { &*get_window_state(window) };
                let old_focus = state.inner_state.borrow().focus;
                let new_focus = if old_focus == BUTTON_1_ID {
                    BUTTON_2_ID
                } else {
                    BUTTON_1_ID
                };
                state.set_focus(new_focus);
                LRESULT(0)
            }
            VK_SPACE => {
                let state = unsafe { &*get_window_state(window) };
                let id = state.inner_state.borrow().focus;
                state.press_button(id);
                LRESULT(0)
            }
            _ => unsafe { DefWindowProcW(window, message, wparam, lparam) },
        },
        SET_FOCUS_MSG => {
            let id = NodeId(lparam.0 as _);
            if id == BUTTON_1_ID || id == BUTTON_2_ID {
                let state = unsafe { &*get_window_state(window) };
                state.set_focus(id);
            }
            LRESULT(0)
        }
        CLICK_MSG => {
            let id = NodeId(lparam.0 as _);
            if id == BUTTON_1_ID || id == BUTTON_2_ID {
                let state = unsafe { &*get_window_state(window) };
                state.press_button(id);
            }
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(window, message, wparam, lparam) },
    }
}

fn create_window(title: &str, initial_focus: NodeId) -> Result<HWND> {
    let create_params = Box::new(WindowCreateParams(initial_focus));
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

fn main() -> Result<()> {
    println!("This example has no visible GUI, and a keyboard interface:");
    println!("- [Tab] switches focus between two logical buttons.");
    println!(
        "- [Space] 'presses' the button, adding static text in a live region announcing that it was pressed."
    );
    println!(
        "Enable Narrator with [Win]+[Ctrl]+[Enter] (or [Win]+[Enter] on older versions of Windows)."
    );

    let window = create_window(WINDOW_TITLE, INITIAL_FOCUS)?;
    let _ = unsafe { ShowWindow(window, SW_SHOW) };

    let mut message = MSG::default();
    while unsafe { GetMessageW(&mut message, None, 0, 0) }.into() {
        let _ = unsafe { TranslateMessage(&message) };
        unsafe { DispatchMessageW(&message) };
    }

    Ok(())
}
