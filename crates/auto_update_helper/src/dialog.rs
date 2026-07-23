use std::{cell::RefCell, sync::mpsc::Receiver};

use anyhow::{Context as _, Result};
use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::Gdi::{
            BeginPaint, CLEARTYPE_QUALITY, CLIP_DEFAULT_PRECIS, CreateFontW, DEFAULT_CHARSET,
            DeleteObject, EndPaint, FW_NORMAL, LOGFONTW, OUT_TT_ONLY_PRECIS, PAINTSTRUCT,
            ReleaseDC, SelectObject, TextOutW,
        },
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            Controls::{PBM_SETRANGE, PBM_SETSTEP, PBM_STEPIT, PROGRESS_CLASS},
            WindowsAndMessaging::{
                CREATESTRUCTW, CS_HREDRAW, CS_VREDRAW, CreateWindowExW, DefWindowProcW,
                GWLP_USERDATA, GetDesktopWindow, GetWindowLongPtrW, GetWindowRect, HICON,
                IMAGE_ICON, LR_DEFAULTSIZE, LR_SHARED, LoadImageW, PostQuitMessage, RegisterClassW,
                SPI_GETICONTITLELOGFONT, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, SendMessageW,
                SetWindowLongPtrW, SystemParametersInfoW, WINDOW_EX_STYLE, WM_CLOSE, WM_CREATE,
                WM_DESTROY, WM_NCCREATE, WM_NCDESTROY, WM_PAINT, WNDCLASSW, WS_CAPTION, WS_CHILD,
                WS_EX_TOPMOST, WS_POPUP, WS_VISIBLE,
            },
        },
    },
    core::HSTRING,
};

use crate::{
    updater::JOBS,
    windows_impl::{WM_JOB_UPDATED, WM_TERMINATE, show_error},
};

#[repr(C)]
#[derive(Debug)]
struct DialogInfo {
    rx: Receiver<Result<()>>,
    progress_bar: isize,
}

pub(crate) fn create_dialog_window(receiver: Receiver<Result<()>>) -> Result<HWND> {
    unsafe {
        let class_name = windows::core::w!("Zed-Auto-Updater-Dialog-Class");
        let module = GetModuleHandleW(None).context("unable to get module handle")?;
        let handle = LoadImageW(
            Some(module.into()),
            windows::core::PCWSTR(1 as _),
            IMAGE_ICON,
            0,
            0,
            LR_DEFAULTSIZE | LR_SHARED,
        )
        .context("unable to load icon file")?;
        let wc = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            lpszClassName: class_name,
            style: CS_HREDRAW | CS_VREDRAW,
            hIcon: HICON(handle.0),
            ..Default::default()
        };
        RegisterClassW(&wc);
        let mut rect = RECT::default();
        GetWindowRect(GetDesktopWindow(), &mut rect)
            .context("unable to get desktop window rect")?;
        let width = 400;
        let height = 150;
        let info = Box::new(RefCell::new(DialogInfo {
            rx: receiver,
            progress_bar: 0,
        }));

        let hwnd = CreateWindowExW(
            WS_EX_TOPMOST,
            class_name,
            windows::core::w!("Zed"),
            WS_VISIBLE | WS_POPUP | WS_CAPTION,
            rect.right / 2 - width / 2,
            rect.bottom / 2 - height / 2,
            width,
            height,
            None,
            None,
            None,
            Some(Box::into_raw(info) as _),
        )
        .context("unable to create dialog window")?;
        Ok(hwnd)
    }
}

macro_rules! return_if_failed {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => {
                return LRESULT(e.code().0 as _);
            }
        }
    };
}

macro_rules! make_lparam {
    ($l:expr, $h:expr) => {
        LPARAM(($l as u32 | ($h as u32) << 16) as isize)
    };
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_NCCREATE => unsafe {
            // Adopt the boxed `DialogInfo` that `CreateWindowExW` passed via `lpCreateParams`,
            // but only once: refuse to overwrite an already-stored pointer (a duplicate or
            // forged `WM_NCCREATE` must not clobber live state), and validate both the creation
            // struct and its payload before storing the pointer we later reclaim in
            // `WM_NCDESTROY`.
            if GetWindowLongPtrW(hwnd, GWLP_USERDATA) == 0 {
                let create_struct = (lparam.0 as *const CREATESTRUCTW).as_ref();
                if let Some(create_struct) = create_struct {
                    let info = create_struct.lpCreateParams as *mut RefCell<DialogInfo>;
                    if !info.is_null() {
                        SetWindowLongPtrW(hwnd, GWLP_USERDATA, info as _);
                    }
                }
            }
            DefWindowProcW(hwnd, msg, wparam, lparam)
        },
        WM_CREATE => unsafe {
            // Create progress bar
            let mut rect = RECT::default();
            return_if_failed!(GetWindowRect(hwnd, &mut rect));
            let progress_bar = return_if_failed!(CreateWindowExW(
                WINDOW_EX_STYLE(0),
                PROGRESS_CLASS,
                None,
                WS_CHILD | WS_VISIBLE,
                20,
                50,
                340,
                35,
                Some(hwnd),
                None,
                None,
                None,
            ));
            SendMessageW(
                progress_bar,
                PBM_SETRANGE,
                None,
                Some(make_lparam!(0, JOBS.len() * 10)),
            );
            SendMessageW(progress_bar, PBM_SETSTEP, Some(WPARAM(10)), None);
            with_dialog_data(hwnd, |data| {
                data.borrow_mut().progress_bar = progress_bar.0 as isize
            });
            LRESULT(0)
        },
        WM_PAINT => unsafe {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);

            let font_name = get_system_ui_font_name();
            let font = CreateFontW(
                24,
                0,
                0,
                0,
                FW_NORMAL.0 as _,
                0,
                0,
                0,
                DEFAULT_CHARSET,
                OUT_TT_ONLY_PRECIS,
                CLIP_DEFAULT_PRECIS,
                CLEARTYPE_QUALITY,
                0,
                &HSTRING::from(font_name),
            );
            let temp = SelectObject(hdc, font.into());
            let string = HSTRING::from("Updating Zed...");
            return_if_failed!(TextOutW(hdc, 20, 15, &string).ok());
            return_if_failed!(DeleteObject(temp).ok());

            return_if_failed!(EndPaint(hwnd, &ps).ok());
            ReleaseDC(Some(hwnd), hdc);

            LRESULT(0)
        },
        WM_JOB_UPDATED => with_dialog_data(hwnd, |data| {
            let progress_bar = data.borrow().progress_bar;
            unsafe { SendMessageW(HWND(progress_bar as _), PBM_STEPIT, None, None) }
        })
        .unwrap_or_else(|| unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }),
        WM_TERMINATE => {
            // Receive the update result while only briefly borrowing the state, then release
            // the borrow before calling `show_error`: its modal message loop can re-enter this
            // window procedure, which must not find the state already borrowed.
            let update_result = with_dialog_data(hwnd, |data| data.borrow_mut().rx.recv());
            if let Some(Ok(Err(e))) = update_result {
                log::error!("Failed to update Zed: {:?}", e);
                show_error(format!("Error: {:?}", e));
            }
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        WM_CLOSE => LRESULT(0), // Prevent user occasionally closing the window
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        WM_NCDESTROY => unsafe {
            // The window is going away and no further messages will reference the state, so
            // reclaim and drop the boxed `DialogInfo` here. This frees it at most once, and only
            // if `WM_NCDESTROY` actually runs: the normal exit path uses `PostQuitMessage` (which
            // unwinds the message loop without destroying the window) and simply leaks the box to
            // process exit, as before.
            let raw = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut RefCell<DialogInfo>;
            if !raw.is_null() {
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
                drop(Box::from_raw(raw));
            }
            DefWindowProcW(hwnd, msg, wparam, lparam)
        },
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

fn with_dialog_data<F, T>(hwnd: HWND, f: F) -> Option<T>
where
    F: FnOnce(&RefCell<DialogInfo>) -> T,
{
    // Borrow the state stored in `GWLP_USERDATA` rather than taking ownership of it. The window
    // procedure can re-enter (for instance through the modal loop that `show_error` runs), and
    // re-`Box::from_raw`-ing the same pointer would create a second owner aliasing a live
    // allocation. The pointer is null before `WM_NCCREATE` stores it and after `WM_NCDESTROY`
    // clears it; `as_ref` turns that null case into `None` instead of dereferencing it.
    let raw = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const RefCell<DialogInfo> };
    let data = unsafe { raw.as_ref() };
    data.map(f)
}

fn get_system_ui_font_name() -> String {
    unsafe {
        let mut info: LOGFONTW = std::mem::zeroed();
        if SystemParametersInfoW(
            SPI_GETICONTITLELOGFONT,
            std::mem::size_of::<LOGFONTW>() as u32,
            Some(&mut info as *mut _ as _),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
        .is_ok()
        {
            let font_name = String::from_utf16_lossy(&info.lfFaceName);
            font_name.trim_matches(char::from(0)).to_owned()
        } else {
            "MS Shell Dlg".to_owned()
        }
    }
}
