// Tray icon implementation for Windows.
//
// Some code reference from:
// https://github.com/tauri-apps/tray-icon/blob/cb22cd5df6b0938aaeebd6c302ec50bc696d8b1a/src/platform_impl/windows/mod.rs

use crate::*;
use image::EncodableLayout;
use muda::ContextMenu;
use std::{
    mem,
    rc::Rc,
    sync::{
        LazyLock,
        atomic::{AtomicU32, Ordering},
    },
};
use windows::{
    Win32::{
        Foundation::*,
        UI::{Shell::*, WindowsAndMessaging::*},
    },
    core::*,
};

const PLATFORM_TRAY_CLASS_NAME: PCWSTR = w!("GPUI::PlatformTray");
const WM_USER_TRAYICON: u32 = 6002;
const WM_USER_UPDATE_TRAYMENU: u32 = 6003;
const WM_USER_UPDATE_TRAYICON: u32 = 6004;
const WM_USER_UPDATE_TRAYTOOLTIP: u32 = 6005;
static WM_TASKBAR_RESTART: LazyLock<u32> =
    LazyLock::new(|| unsafe { RegisterWindowMessageA(s!("TaskbarCreated")) });
static COUNTER: AtomicU32 = AtomicU32::new(0);

pub(crate) struct WindowsTray {
    tray_id: u32,
    hwnd: HWND,
    menu: Option<muda::Menu>,
    visible: bool,
}

impl WindowsTray {
    pub(crate) fn create(tray: &Tray, menu: Option<muda::Menu>) -> Self {
        let tray_id = COUNTER.fetch_add(1, Ordering::Relaxed);

        let mut this = Self {
            tray_id,
            hwnd: Self::create_tray_window(tray_id),
            menu: None,
            visible: tray.visible,
        };
        this.update(&tray, menu);
        this
    }

    pub(crate) fn update(&mut self, tray: &Tray, menu: Option<muda::Menu>) {
        self.set_visible(tray.visible);
        if !self.visible {
            return;
        }

        let hicon = tray.icon_data.as_ref().map(|image| {
            *Icon::new(image.data.as_bytes(), image.width, image.height).as_raw_handle()
        });

        self.set_menu(menu);
        self.set_icon(hicon);
        self.set_tooltip(tray.tooltip.clone());
    }

    fn create_tray_window(tray_id: u32) -> HWND {
        let traydata = TrayUserData {
            tray_id,
            hwnd: HWND(std::ptr::null_mut()),
            hpopupmenu: None,
            icon: None,
            tooltip: None,
        };

        register_platform_tray_class();
        let result = unsafe {
            CreateWindowExW(
                WS_EX_NOACTIVATE | WS_EX_TRANSPARENT | WS_EX_LAYERED | WS_EX_TOOLWINDOW,
                PLATFORM_TRAY_CLASS_NAME,
                None,
                WS_OVERLAPPED,
                CW_USEDEFAULT,
                0,
                CW_USEDEFAULT,
                0,
                None,
                None,
                None,
                Some(Box::into_raw(Box::new(traydata)) as _),
            )
        };

        let hwnd = result.expect("Failed to create tray window");

        if !register_tray_icon(hwnd, tray_id, None, None) {
            unsafe {
                let _ = DestroyWindow(hwnd);
            };
            return hwnd;
        }

        hwnd
    }

    fn set_visible(&mut self, visible: bool) {
        if self.visible == visible {
            return;
        }

        self.visible = visible;
        if visible {
            self.hwnd = Self::create_tray_window(self.tray_id);
        } else {
            remove_tray_icon(self.hwnd, self.tray_id);
        }
    }

    fn set_icon(&mut self, icon: Option<HICON>) {
        unsafe {
            let mut nid = NOTIFYICONDATAW {
                uFlags: NIF_ICON,
                hWnd: self.hwnd,
                uID: self.tray_id,
                ..std::mem::zeroed()
            };

            if let Some(hicon) = icon {
                nid.hIcon = hicon;
            }

            let _ = Shell_NotifyIconW(NIM_MODIFY, &mut nid as _);
            SendMessageW(
                self.hwnd,
                WM_USER_UPDATE_TRAYICON,
                icon.map(|icon| WPARAM(Box::into_raw(Box::new(Some(icon))) as *mut _ as usize)),
                Some(LPARAM(0)),
            );
        }
    }

    fn set_tooltip(&mut self, tooltip: Option<SharedString>) {
        unsafe {
            let mut nid = NOTIFYICONDATAW {
                uFlags: NIF_TIP,
                hWnd: self.hwnd,
                uID: self.tray_id,
                ..std::mem::zeroed()
            };
            if let Some(tooltip) = &tooltip {
                let tip = encode_wide(tooltip.as_ref());
                #[allow(clippy::manual_memcpy)]
                for i in 0..tip.len().min(128) {
                    nid.szTip[i] = tip[i];
                }
            }
            let tooltip = tooltip.map(|t| t.as_ref().to_string());

            let _ = Shell_NotifyIconW(NIM_MODIFY, &mut nid as _);
            SendMessageW(
                self.hwnd,
                WM_USER_UPDATE_TRAYTOOLTIP,
                tooltip.map(|t| WPARAM(Box::into_raw(Box::new(Some(t))) as _)),
                Some(LPARAM(0)),
            );
        }
    }

    fn set_menu(&mut self, menu: Option<muda::Menu>) {
        if let Some(menu) = &self.menu {
            unsafe { menu.detach_menu_subclass_from_hwnd(self.hwnd.0 as _) };
        }
        if let Some(menu) = &menu {
            unsafe { menu.attach_menu_subclass_for_hwnd(self.hwnd.0 as _) };
        }

        unsafe {
            // send the new menu to the subclass proc where we will update there
            SendMessageW(
                self.hwnd,
                WM_USER_UPDATE_TRAYMENU,
                menu.as_ref().map(|menu| {
                    WPARAM(Box::into_raw(Box::new(Some(HMENU(menu.hpopupmenu() as _)))) as _)
                }),
                Some(LPARAM(0)),
            );
        }

        self.menu = menu;
    }
}
struct TrayUserData {
    hwnd: HWND,
    tray_id: u32,
    hpopupmenu: Option<HMENU>,
    icon: Option<HICON>,
    tooltip: Option<String>,
}

/// An icon used for the window titlebar, taskbar, etc.
#[derive(Clone)]
struct Icon {
    icon: Rc<HICON>,
}

#[repr(C)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

const PIXEL_SIZE: usize = mem::size_of::<Pixel>();

impl Icon {
    fn new(rgba: &[u8], width: u32, height: u32) -> Self {
        let pixel_count = rgba.len() / PIXEL_SIZE;
        let mut and_mask = Vec::with_capacity(pixel_count);
        let pixels =
            unsafe { std::slice::from_raw_parts_mut(rgba.as_ptr() as *mut Pixel, pixel_count) };
        for pixel in pixels {
            and_mask.push(pixel.a.wrapping_sub(u8::MAX)); // invert alpha channel
        }
        assert_eq!(and_mask.len(), pixel_count);
        let handle = unsafe {
            CreateIcon(
                None,
                width as i32,
                height as i32,
                1,
                (PIXEL_SIZE * 8) as u8,
                and_mask.as_ptr(),
                rgba.as_ptr(),
            )
            .expect("Failed to create tray icon")
        };
        Self {
            icon: Rc::new(handle),
        }
    }

    fn as_raw_handle(&self) -> &HICON {
        self.icon.as_ref()
    }
}

fn register_platform_tray_class() {
    let wc = WNDCLASSW {
        lpfnWndProc: Some(tray_procedure),
        lpszClassName: PCWSTR(PLATFORM_TRAY_CLASS_NAME.as_ptr()),
        ..Default::default()
    };
    unsafe { RegisterClassW(&wc) };
}

/// Procedure for handling messages related to the platform tray.
unsafe extern "system" fn tray_procedure(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let userdata_ptr = unsafe { get_window_long(hwnd, GWL_USERDATA) };
    let userdata_ptr = match (userdata_ptr, msg) {
        (0, WM_NCCREATE) => unsafe {
            let createstruct = &mut *(lparam.0 as *mut CREATESTRUCTW);
            let userdata = &mut *(createstruct.lpCreateParams as *mut TrayUserData);
            userdata.hwnd = hwnd;

            set_window_long(hwnd, GWL_USERDATA, createstruct.lpCreateParams as _);
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        },
        // Getting here should quite frankly be impossible,
        // but we'll make window creation fail here just in case.
        (0, WM_CREATE) => return LRESULT(-1),
        (_, WM_CREATE) => unsafe { return DefWindowProcW(hwnd, msg, wparam, lparam) },
        (0, _) => unsafe { return DefWindowProcW(hwnd, msg, wparam, lparam) },
        _ => userdata_ptr as *mut TrayUserData,
    };

    unsafe {
        let userdata = &mut *(userdata_ptr);
        match msg {
            WM_DESTROY => {
                drop(Box::from_raw(userdata_ptr));
                return LRESULT(0);
            }
            WM_USER_UPDATE_TRAYMENU => {
                let hpopupmenu = Box::from_raw(wparam.0 as *mut Option<HMENU>);
                userdata.hpopupmenu = *hpopupmenu;
            }
            WM_USER_UPDATE_TRAYICON => {
                let icon = Box::from_raw(wparam.0 as *mut Option<HICON>);
                userdata.icon = *icon;
            }
            WM_USER_UPDATE_TRAYTOOLTIP => {
                let tooltip = Box::from_raw(wparam.0 as *mut Option<String>);
                userdata.tooltip = *tooltip;
            }
            _ if msg == *WM_TASKBAR_RESTART => {
                remove_tray_icon(userdata.hwnd, userdata.tray_id);
                register_tray_icon(
                    userdata.hwnd,
                    userdata.tray_id,
                    userdata.icon,
                    userdata.tooltip.as_ref(),
                );
            }
            WM_USER_TRAYICON if matches!(lparam.0 as u32, WM_RBUTTONDOWN) => {
                let mut cursor = POINT { x: 0, y: 0 };
                if GetCursorPos(&mut cursor as _).is_err() {
                    return LRESULT(0);
                }

                if let Some(menu) = userdata.hpopupmenu {
                    show_tray_menu(hwnd, menu, cursor.x, cursor.y);
                }
            }

            _ => {}
        }

        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

fn show_tray_menu(hwnd: HWND, menu: HMENU, x: i32, y: i32) {
    unsafe {
        let _ = SetForegroundWindow(hwnd);
        let result = TrackPopupMenu(
            menu,
            TPM_BOTTOMALIGN | TPM_LEFTALIGN,
            x,
            y,
            Some(0),
            hwnd,
            Some(std::ptr::null_mut()),
        );
        if !result.as_bool() {
            log::error!("Failed to show tray menu.");
        }
    }
}

#[inline]
fn register_tray_icon(
    hwnd: HWND,
    tray_id: u32,
    hicon: Option<HICON>,
    tooltip: Option<&String>,
) -> bool {
    let mut h_icon: HICON = HICON(std::ptr::null_mut());
    let mut flags = NIF_MESSAGE;
    let mut sz_tip: [u16; 128] = [0; 128];

    if let Some(hicon) = hicon {
        flags |= NIF_ICON;
        h_icon = hicon;
    }

    if let Some(tooltip) = tooltip {
        flags |= NIF_TIP;
        let tip = encode_wide(tooltip.as_str());
        #[allow(clippy::manual_memcpy)]
        for i in 0..tip.len().min(128) {
            sz_tip[i] = tip[i];
        }
    }

    unsafe {
        let mut nid = NOTIFYICONDATAW {
            uFlags: flags,
            hWnd: hwnd,
            uID: tray_id,
            uCallbackMessage: WM_USER_TRAYICON,
            hIcon: h_icon,
            szTip: sz_tip,
            ..std::mem::zeroed()
        };

        Shell_NotifyIconW(NIM_ADD, &mut nid as _) == TRUE
    }
}

#[inline]
fn remove_tray_icon(hwnd: HWND, id: u32) {
    unsafe {
        let mut nid = NOTIFYICONDATAW {
            uFlags: NIF_ICON,
            hWnd: hwnd,
            uID: id,
            ..std::mem::zeroed()
        };

        if Shell_NotifyIconW(NIM_DELETE, &mut nid as _) == FALSE {
            eprintln!("Error removing system tray icon");
        }
    }
}

#[inline]
fn encode_wide<S: AsRef<std::ffi::OsStr>>(string: S) -> Vec<u16> {
    std::os::windows::prelude::OsStrExt::encode_wide(string.as_ref())
        .chain(std::iter::once(0))
        .collect()
}
