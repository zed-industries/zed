use std::{cell::RefCell, sync::mpsc::Receiver};

use anyhow::{Context as _, Result};
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::Gdi::{
            BeginPaint, CreateFontW, DeleteObject, EndPaint, ReleaseDC, SelectObject, TextOutW,
            CLEARTYPE_QUALITY, CLIP_DEFAULT_PRECIS, DEFAULT_CHARSET, FW_NORMAL, LOGFONTW,
            OUT_TT_ONLY_PRECIS, PAINTSTRUCT,
        },
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            Controls::{PBM_SETRANGE, PBM_SETSTEP, PBM_STEPIT, PROGRESS_CLASS},
            WindowsAndMessaging::{
                CreateWindowExW, DefWindowProcW, GetDesktopWindow, GetWindowLongPtrW,
                GetWindowRect, LoadImageW, PostQuitMessage, RegisterClassW, SendMessageW,
                SetWindowLongPtrW, SystemParametersInfoW, CREATESTRUCTW, CS_HREDRAW, CS_VREDRAW,
                GWLP_USERDATA, HICON, IMAGE_ICON, LR_DEFAULTSIZE, LR_SHARED,
                SPI_GETICONTITLELOGFONT, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, WINDOW_EX_STYLE,
                WM_CLOSE, WM_CREATE, WM_DESTROY, WM_NCCREATE, WM_PAINT, WNDCLASSW, WS_CAPTION,
                WS_CHILD, WS_EX_TOPMOST, WS_POPUP, WS_VISIBLE,
            },
        },
    },
};

use crate::{
    updater::JOBS_COUNT,
    windows_impl::{show_error, WM_JOB_UPDATED, WM_TERMINATE},
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
            windows::core::w!("Zed Editor"),
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
        WM_NCCREATE => {
            let create_struct = lparam.0 as *const CREATESTRUCTW;
            let info = (*create_struct).lpCreateParams as *mut RefCell<DialogInfo>;
            let info = Box::from_raw(info);
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(info) as _);
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        WM_CREATE => {
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
                Some(make_lparam!(0, JOBS_COUNT * 10)),
            );
            SendMessageW(progress_bar, PBM_SETSTEP, Some(WPARAM(10)), None);
            with_dialog_data(hwnd, |data| {
                data.borrow_mut().progress_bar = progress_bar.0 as isize
            });
            LRESULT(0)
        }
        WM_PAINT => {
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
            let string = HSTRING::from("Zed Editor is updating...");
            return_if_failed!(TextOutW(hdc, 20, 15, &string).ok());
            return_if_failed!(DeleteObject(temp).ok());

            return_if_failed!(EndPaint(hwnd, &ps).ok());
            ReleaseDC(Some(hwnd), hdc);

            LRESULT(0)
        }
        WM_JOB_UPDATED => with_dialog_data(hwnd, |data| {
            let progress_bar = data.borrow().progress_bar;
            SendMessageW(HWND(progress_bar as _), PBM_STEPIT, None, None)
        }),
        WM_TERMINATE => {
            with_dialog_data(hwnd, |data| {
                if let Ok(result) = data.borrow_mut().rx.recv() {
                    if let Err(e) = result {
                        log::error!("Failed to update Zed: {:?}", e);
                        show_error(format!("Error: {:?}", e));
                    }
                }
            });
            PostQuitMessage(0);
            LRESULT(0)
        }
        WM_CLOSE => LRESULT(0), // Prevent user occasionally closing the window
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn with_dialog_data<F, T>(hwnd: HWND, f: F) -> T
where
    F: FnOnce(&RefCell<DialogInfo>) -> T,
{
    let raw = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut RefCell<DialogInfo> };
    let data = unsafe { Box::from_raw(raw) };
    let result = f(data.as_ref());
    unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(data) as _) };
    result
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
