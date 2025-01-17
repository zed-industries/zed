use std::{
    rc::{Rc, Weak},
    sync::Once,
};

use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        DefWindowProcW, RegisterClassW, CREATESTRUCTW, CS_HREDRAW, CS_VREDRAW, GWLP_USERDATA,
        WM_NCCREATE, WM_NCDESTROY, WNDCLASSW,
    },
};
use windows_core::PCWSTR;

use crate::{get_module_handle, get_window_long, set_window_long};

// pub(crate) fn init_platform_window() {
//     register_wnd_class();
//     let hwnd =
// }

// fn register_wnd_class() -> PCWSTR {
//     const CLASS_NAME: PCWSTR = windows_core::w!("Zed::PlatformWindow");

//     static ONCE: Once = Once::new();
//     ONCE.call_once(|| {
//         let wc = WNDCLASSW {
//             lpfnWndProc: Some(wnd_proc),
//             lpszClassName: CLASS_NAME,
//             style: CS_HREDRAW | CS_VREDRAW,
//             hInstance: get_module_handle().into(),
//             ..Default::default()
//         };
//         unsafe { RegisterClassW(&wc) };
//     });

//     CLASS_NAME
// }

// unsafe extern "system" fn wnd_proc(
//     hwnd: HWND,
//     msg: u32,
//     wparam: WPARAM,
//     lparam: LPARAM,
// ) -> LRESULT {
//     if msg == WM_NCCREATE {
//         let cs = lparam.0 as *const CREATESTRUCTW;
//         let cs = unsafe { &*cs };
//         let ctx = cs.lpCreateParams as *mut WindowCreateContext;
//         let ctx = unsafe { &mut *ctx };
//         let creation_result = WindowsWindowStatePtr::new(ctx, hwnd, cs);
//         if creation_result.is_err() {
//             ctx.inner = Some(creation_result);
//             return LRESULT(0);
//         }
//         let weak = Box::new(Rc::downgrade(creation_result.as_ref().unwrap()));
//         unsafe { set_window_long(hwnd, GWLP_USERDATA, Box::into_raw(weak) as isize) };
//         ctx.inner = Some(creation_result);
//         return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
//     }
//     let ptr = unsafe { get_window_long(hwnd, GWLP_USERDATA) } as *mut Weak<WindowsWindowStatePtr>;
//     if ptr.is_null() {
//         return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
//     }
//     let inner = unsafe { &*ptr };
//     let r = if let Some(state) = inner.upgrade() {
//         handle_msg(hwnd, msg, wparam, lparam, state)
//     } else {
//         unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
//     };
//     if msg == WM_NCDESTROY {
//         unsafe { set_window_long(hwnd, GWLP_USERDATA, 0) };
//         unsafe { drop(Box::from_raw(ptr)) };
//     }
//     r
// }
