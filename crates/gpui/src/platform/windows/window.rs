#![deny(unsafe_op_in_unsafe_fn)]

use std::{
    any::Any,
    cell::{Cell, RefCell},
    ffi::c_void,
    iter::once,
    num::NonZeroIsize,
    path::PathBuf,
    rc::{Rc, Weak},
    str::FromStr,
    sync::{Arc, Once},
};

use ::util::ResultExt;
use blade_graphics as gpu;
use futures::channel::oneshot::{self, Receiver};
use itertools::Itertools;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use smallvec::SmallVec;
use std::result::Result;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        System::{Com::*, Ole::*, SystemServices::*},
        UI::{
            Controls::*,
            HiDpi::*,
            Input::{Ime::*, KeyboardAndMouse::*},
            Shell::*,
            WindowsAndMessaging::*,
        },
    },
};

use crate::platform::blade::BladeRenderer;
use crate::*;

pub(crate) struct WindowsWindowInner {
    hwnd: HWND,
    origin: Cell<Point<GlobalPixels>>,
    size: Cell<Size<GlobalPixels>>,
    mouse_position: Cell<Point<Pixels>>,
    input_handler: Cell<Option<PlatformInputHandler>>,
    renderer: RefCell<BladeRenderer>,
    callbacks: RefCell<Callbacks>,
    platform_inner: Rc<WindowsPlatformInner>,
    handle: AnyWindowHandle,
    scale_factor: f32,
}

impl WindowsWindowInner {
    fn new(
        hwnd: HWND,
        cs: &CREATESTRUCTW,
        platform_inner: Rc<WindowsPlatformInner>,
        handle: AnyWindowHandle,
    ) -> Self {
        let origin = Cell::new(Point::new((cs.x as f64).into(), (cs.y as f64).into()));
        let size = Cell::new(Size {
            width: (cs.cx as f64).into(),
            height: (cs.cy as f64).into(),
        });
        let mouse_position = Cell::new(Point::default());
        let input_handler = Cell::new(None);
        struct RawWindow {
            hwnd: *mut c_void,
        }
        unsafe impl blade_rwh::HasRawWindowHandle for RawWindow {
            fn raw_window_handle(&self) -> blade_rwh::RawWindowHandle {
                let mut handle = blade_rwh::Win32WindowHandle::empty();
                handle.hwnd = self.hwnd;
                handle.into()
            }
        }
        unsafe impl blade_rwh::HasRawDisplayHandle for RawWindow {
            fn raw_display_handle(&self) -> blade_rwh::RawDisplayHandle {
                blade_rwh::WindowsDisplayHandle::empty().into()
            }
        }
        let raw = RawWindow { hwnd: hwnd.0 as _ };
        let gpu = Arc::new(
            unsafe {
                gpu::Context::init_windowed(
                    &raw,
                    gpu::ContextDesc {
                        validation: false,
                        capture: false,
                        overlay: false,
                    },
                )
            }
            .unwrap(),
        );
        let extent = gpu::Extent {
            width: 1,
            height: 1,
            depth: 1,
        };
        let renderer = RefCell::new(BladeRenderer::new(gpu, extent));
        let callbacks = RefCell::new(Callbacks::default());
        Self {
            hwnd,
            origin,
            size,
            mouse_position,
            input_handler,
            renderer,
            callbacks,
            platform_inner,
            handle,
            scale_factor: 1.0,
        }
    }

    fn is_maximized(&self) -> bool {
        let mut placement = WINDOWPLACEMENT::default();
        placement.length = std::mem::size_of::<WINDOWPLACEMENT>() as u32;
        if unsafe { GetWindowPlacement(self.hwnd, &mut placement) }.is_ok() {
            return placement.showCmd == SW_SHOWMAXIMIZED.0 as u32;
        }
        return false;
    }

    fn get_titlebar_rect(&self) -> anyhow::Result<RECT> {
        let top_and_bottom_borders = 2;
        let theme = unsafe { OpenThemeData(self.hwnd, w!("WINDOW")) };
        let title_bar_size = unsafe {
            GetThemePartSize(
                theme,
                HDC::default(),
                WP_CAPTION.0,
                CS_ACTIVE.0,
                None,
                TS_TRUE,
            )
        }?;
        unsafe { CloseThemeData(theme) }?;

        let mut height =
            (title_bar_size.cy as f32 * self.scale_factor).round() as i32 + top_and_bottom_borders;

        if self.is_maximized() {
            let dpi = unsafe { GetDpiForWindow(self.hwnd) };
            height += unsafe { (GetSystemMetricsForDpi(SM_CXPADDEDBORDER, dpi) * 2) as i32 };
        }

        let mut rect = RECT::default();
        unsafe { GetClientRect(self.hwnd, &mut rect) }?;
        rect.bottom = rect.top + height;
        Ok(rect)
    }

    fn is_virtual_key_pressed(&self, vkey: VIRTUAL_KEY) -> bool {
        unsafe { GetKeyState(vkey.0 as i32) < 0 }
    }

    fn current_modifiers(&self) -> Modifiers {
        Modifiers {
            control: self.is_virtual_key_pressed(VK_CONTROL),
            alt: self.is_virtual_key_pressed(VK_MENU),
            shift: self.is_virtual_key_pressed(VK_SHIFT),
            command: self.is_virtual_key_pressed(VK_LWIN) || self.is_virtual_key_pressed(VK_RWIN),
            function: false,
        }
    }

    /// mark window client rect to be re-drawn
    /// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidaterect
    pub(crate) fn invalidate_client_area(&self) {
        unsafe { InvalidateRect(self.hwnd, None, FALSE) };
    }

    fn handle_msg(&self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        log::debug!("msg: {msg}, wparam: {}, lparam: {}", wparam.0, lparam.0);
        match msg {
            WM_ACTIVATE => self.handle_activate_msg(msg, wparam, lparam),
            WM_CREATE => self.handle_create_msg(lparam),
            WM_MOVE => self.handle_move_msg(lparam),
            WM_SIZE => self.handle_size_msg(lparam),
            WM_NCCALCSIZE => self.handle_calc_client_size(msg, wparam, lparam),
            WM_DPICHANGED => self.handle_dpi_changed_msg(msg, wparam, lparam),
            WM_NCHITTEST => self.handle_hit_test_msg(msg, wparam, lparam),
            WM_PAINT => self.handle_paint_msg(),
            WM_CLOSE => self.handle_close_msg(msg, wparam, lparam),
            WM_DESTROY => self.handle_destroy_msg(),
            WM_MOUSEMOVE => self.handle_mouse_move_msg(lparam, wparam),
            WM_NCMOUSEMOVE => self.handle_nc_mouse_move_msg(msg, wparam, lparam),
            WM_NCLBUTTONDOWN => {
                self.handle_nc_mouse_down_msg(MouseButton::Left, msg, wparam, lparam)
            }
            WM_NCRBUTTONDOWN => {
                self.handle_nc_mouse_down_msg(MouseButton::Right, msg, wparam, lparam)
            }
            WM_NCMBUTTONDOWN => {
                self.handle_nc_mouse_down_msg(MouseButton::Middle, msg, wparam, lparam)
            }
            WM_NCLBUTTONUP => self.handle_nc_mouse_up_msg(MouseButton::Left, msg, wparam, lparam),
            WM_NCRBUTTONUP => self.handle_nc_mouse_up_msg(MouseButton::Right, msg, wparam, lparam),
            WM_NCMBUTTONUP => self.handle_nc_mouse_up_msg(MouseButton::Middle, msg, wparam, lparam),
            WM_LBUTTONDOWN => self.handle_mouse_down_msg(MouseButton::Left, lparam),
            WM_RBUTTONDOWN => self.handle_mouse_down_msg(MouseButton::Right, lparam),
            WM_MBUTTONDOWN => self.handle_mouse_down_msg(MouseButton::Middle, lparam),
            WM_XBUTTONDOWN => {
                let nav_dir = match wparam.hiword() {
                    XBUTTON1 => Some(NavigationDirection::Forward),
                    XBUTTON2 => Some(NavigationDirection::Back),
                    _ => None,
                };

                if let Some(nav_dir) = nav_dir {
                    self.handle_mouse_down_msg(MouseButton::Navigate(nav_dir), lparam)
                } else {
                    LRESULT(1)
                }
            }
            WM_LBUTTONUP => self.handle_mouse_up_msg(MouseButton::Left, lparam),
            WM_RBUTTONUP => self.handle_mouse_up_msg(MouseButton::Right, lparam),
            WM_MBUTTONUP => self.handle_mouse_up_msg(MouseButton::Middle, lparam),
            WM_XBUTTONUP => {
                let nav_dir = match wparam.hiword() {
                    XBUTTON1 => Some(NavigationDirection::Back),
                    XBUTTON2 => Some(NavigationDirection::Forward),
                    _ => None,
                };

                if let Some(nav_dir) = nav_dir {
                    self.handle_mouse_up_msg(MouseButton::Navigate(nav_dir), lparam)
                } else {
                    LRESULT(1)
                }
            }
            WM_MOUSEWHEEL => self.handle_mouse_wheel_msg(wparam, lparam),
            WM_MOUSEHWHEEL => self.handle_mouse_horizontal_wheel_msg(wparam, lparam),
            WM_SYSKEYDOWN => self.handle_syskeydown_msg(msg, wparam, lparam),
            WM_SYSKEYUP => self.handle_syskeyup_msg(msg, wparam, lparam),
            WM_KEYDOWN => self.handle_keydown_msg(msg, wparam, lparam),
            WM_KEYUP => self.handle_keyup_msg(msg, wparam),
            WM_CHAR => self.handle_char_msg(msg, wparam, lparam),
            WM_IME_STARTCOMPOSITION => self.handle_ime_position(),
            WM_IME_COMPOSITION => self.handle_ime_composition(msg, wparam, lparam),
            WM_IME_CHAR => self.handle_ime_char(wparam),
            _ => unsafe { DefWindowProcW(self.hwnd, msg, wparam, lparam) },
        }
    }

    fn handle_move_msg(&self, lparam: LPARAM) -> LRESULT {
        let x = lparam.signed_loword() as f64;
        let y = lparam.signed_hiword() as f64;
        self.origin.set(Point::new(x.into(), y.into()));
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.moved.as_mut() {
            callback()
        }
        LRESULT(0)
    }

    fn handle_size_msg(&self, lparam: LPARAM) -> LRESULT {
        let width = lparam.loword().max(1) as f64;
        let height = lparam.hiword().max(1) as f64;
        self.renderer
            .borrow_mut()
            .update_drawable_size(Size { width, height });
        let width = width.into();
        let height = height.into();
        self.size.set(Size { width, height });
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.resize.as_mut() {
            callback(
                Size {
                    width: Pixels(width.0),
                    height: Pixels(height.0),
                },
                1.0,
            );
        }
        self.invalidate_client_area();
        LRESULT(0)
    }

    fn handle_paint_msg(&self) -> LRESULT {
        let mut paint_struct = PAINTSTRUCT::default();
        let _hdc = unsafe { BeginPaint(self.hwnd, &mut paint_struct) };
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(request_frame) = callbacks.request_frame.as_mut() {
            request_frame();
        }
        unsafe { EndPaint(self.hwnd, &paint_struct) };
        LRESULT(0)
    }

    fn handle_close_msg(&self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.should_close.as_mut() {
            if callback() {
                return LRESULT(0);
            }
        }
        drop(callbacks);
        unsafe { DefWindowProcW(self.hwnd, msg, wparam, lparam) }
    }

    fn handle_destroy_msg(&self) -> LRESULT {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.close.take() {
            callback()
        }
        let index = self
            .platform_inner
            .raw_window_handles
            .read()
            .iter()
            .position(|handle| *handle == self.hwnd)
            .unwrap();
        self.platform_inner.raw_window_handles.write().remove(index);
        if self.platform_inner.raw_window_handles.read().is_empty() {
            self.platform_inner
                .foreground_executor
                .spawn(async {
                    unsafe { PostQuitMessage(0) };
                })
                .detach();
        }
        LRESULT(1)
    }

    fn handle_mouse_move_msg(&self, lparam: LPARAM, wparam: WPARAM) -> LRESULT {
        let x = Pixels::from(lparam.signed_loword() as f32);
        let y = Pixels::from(lparam.signed_hiword() as f32);
        self.mouse_position.set(Point { x, y });
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.input.as_mut() {
            let pressed_button = match MODIFIERKEYS_FLAGS(wparam.loword() as u32) {
                flags if flags.contains(MK_LBUTTON) => Some(MouseButton::Left),
                flags if flags.contains(MK_RBUTTON) => Some(MouseButton::Right),
                flags if flags.contains(MK_MBUTTON) => Some(MouseButton::Middle),
                flags if flags.contains(MK_XBUTTON1) => {
                    Some(MouseButton::Navigate(NavigationDirection::Back))
                }
                flags if flags.contains(MK_XBUTTON2) => {
                    Some(MouseButton::Navigate(NavigationDirection::Forward))
                }
                _ => None,
            };
            let event = MouseMoveEvent {
                position: Point { x, y },
                pressed_button,
                modifiers: self.current_modifiers(),
            };
            if callback(PlatformInput::MouseMove(event)).default_prevented {
                return LRESULT(0);
            }
        }
        LRESULT(1)
    }

    fn parse_syskeydown_msg_keystroke(&self, wparam: WPARAM) -> Option<Keystroke> {
        let modifiers = self.current_modifiers();
        if !modifiers.alt {
            // on Windows, F10 can trigger this event, not just the alt key
            // and we just don't care about F10
            return None;
        }

        let vk_code = wparam.loword();
        let basic_key = basic_vkcode_to_string(vk_code, modifiers);
        if basic_key.is_some() {
            return basic_key;
        }

        let key = match VIRTUAL_KEY(vk_code) {
            VK_BACK => Some("backspace"),
            VK_RETURN => Some("enter"),
            VK_TAB => Some("tab"),
            VK_UP => Some("up"),
            VK_DOWN => Some("down"),
            VK_RIGHT => Some("right"),
            VK_LEFT => Some("left"),
            VK_HOME => Some("home"),
            VK_END => Some("end"),
            VK_PRIOR => Some("pageup"),
            VK_NEXT => Some("pagedown"),
            VK_ESCAPE => Some("escape"),
            VK_INSERT => Some("insert"),
            _ => None,
        };

        if let Some(key) = key {
            Some(Keystroke {
                modifiers,
                key: key.to_string(),
                ime_key: None,
            })
        } else {
            None
        }
    }

    fn parse_keydown_msg_keystroke(&self, wparam: WPARAM) -> Option<Keystroke> {
        let vk_code = wparam.loword();

        let modifiers = self.current_modifiers();
        if modifiers.control || modifiers.alt {
            let basic_key = basic_vkcode_to_string(vk_code, modifiers);
            if basic_key.is_some() {
                return basic_key;
            }
        }

        if vk_code >= VK_F1.0 && vk_code <= VK_F24.0 {
            let offset = vk_code - VK_F1.0;
            return Some(Keystroke {
                modifiers,
                key: format!("f{}", offset + 1),
                ime_key: None,
            });
        }

        let key = match VIRTUAL_KEY(vk_code) {
            VK_BACK => Some("backspace"),
            VK_RETURN => Some("enter"),
            VK_TAB => Some("tab"),
            VK_UP => Some("up"),
            VK_DOWN => Some("down"),
            VK_RIGHT => Some("right"),
            VK_LEFT => Some("left"),
            VK_HOME => Some("home"),
            VK_END => Some("end"),
            VK_PRIOR => Some("pageup"),
            VK_NEXT => Some("pagedown"),
            VK_ESCAPE => Some("escape"),
            VK_INSERT => Some("insert"),
            _ => None,
        };

        if let Some(key) = key {
            Some(Keystroke {
                modifiers,
                key: key.to_string(),
                ime_key: None,
            })
        } else {
            None
        }
    }

    fn parse_char_msg_keystroke(&self, wparam: WPARAM) -> Option<Keystroke> {
        let src = [wparam.0 as u16];
        let Ok(first_char) = char::decode_utf16(src).collect::<Vec<_>>()[0] else {
            return None;
        };
        if first_char.is_control() {
            None
        } else {
            Some(Keystroke {
                modifiers: self.current_modifiers(),
                key: first_char.to_lowercase().to_string(),
                ime_key: Some(first_char.to_string()),
            })
        }
    }

    fn handle_syskeydown_msg(&self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        // we need to call `DefWindowProcW`, or we will lose the system-wide `Alt+F4`, `Alt+{other keys}`
        // shortcuts.
        let Some(keystroke) = self.parse_syskeydown_msg_keystroke(wparam) else {
            return unsafe { DefWindowProcW(self.hwnd, message, wparam, lparam) };
        };
        let Some(ref mut func) = self.callbacks.borrow_mut().input else {
            return unsafe { DefWindowProcW(self.hwnd, message, wparam, lparam) };
        };
        let event = KeyDownEvent {
            keystroke,
            is_held: lparam.0 & (0x1 << 30) > 0,
        };
        if func(PlatformInput::KeyDown(event)).default_prevented {
            self.invalidate_client_area();
            return LRESULT(0);
        }
        unsafe { DefWindowProcW(self.hwnd, message, wparam, lparam) }
    }

    fn handle_syskeyup_msg(&self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        // we need to call `DefWindowProcW`, or we will lose the system-wide `Alt+F4`, `Alt+{other keys}`
        // shortcuts.
        let Some(keystroke) = self.parse_syskeydown_msg_keystroke(wparam) else {
            return unsafe { DefWindowProcW(self.hwnd, message, wparam, lparam) };
        };
        let Some(ref mut func) = self.callbacks.borrow_mut().input else {
            return unsafe { DefWindowProcW(self.hwnd, message, wparam, lparam) };
        };
        let event = KeyUpEvent { keystroke };
        if func(PlatformInput::KeyUp(event)).default_prevented {
            self.invalidate_client_area();
            return LRESULT(0);
        }
        unsafe { DefWindowProcW(self.hwnd, message, wparam, lparam) }
    }

    fn handle_keydown_msg(&self, _msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        let Some(keystroke) = self.parse_keydown_msg_keystroke(wparam) else {
            return LRESULT(1);
        };
        let Some(ref mut func) = self.callbacks.borrow_mut().input else {
            return LRESULT(1);
        };
        let event = KeyDownEvent {
            keystroke,
            is_held: lparam.0 & (0x1 << 30) > 0,
        };
        if func(PlatformInput::KeyDown(event)).default_prevented {
            self.invalidate_client_area();
            return LRESULT(0);
        }
        LRESULT(1)
    }

    fn handle_keyup_msg(&self, _msg: u32, wparam: WPARAM) -> LRESULT {
        let Some(keystroke) = self.parse_keydown_msg_keystroke(wparam) else {
            return LRESULT(1);
        };
        let Some(ref mut func) = self.callbacks.borrow_mut().input else {
            return LRESULT(1);
        };
        let event = KeyUpEvent { keystroke };
        if func(PlatformInput::KeyUp(event)).default_prevented {
            self.invalidate_client_area();
            return LRESULT(0);
        }
        LRESULT(1)
    }

    fn handle_char_msg(&self, _msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        let Some(keystroke) = self.parse_char_msg_keystroke(wparam) else {
            return LRESULT(1);
        };
        let mut callbacks = self.callbacks.borrow_mut();
        let Some(ref mut func) = callbacks.input else {
            return LRESULT(1);
        };
        let ime_key = keystroke.ime_key.clone();
        let event = KeyDownEvent {
            keystroke,
            is_held: lparam.0 & (0x1 << 30) > 0,
        };
        if func(PlatformInput::KeyDown(event)).default_prevented {
            self.invalidate_client_area();
            return LRESULT(0);
        }
        drop(callbacks);
        let Some(ime_char) = ime_key else {
            return LRESULT(1);
        };
        let Some(mut input_handler) = self.input_handler.take() else {
            return LRESULT(1);
        };
        input_handler.replace_text_in_range(None, &ime_char);
        self.input_handler.set(Some(input_handler));
        self.invalidate_client_area();
        LRESULT(0)
    }

    fn handle_mouse_down_msg(&self, button: MouseButton, lparam: LPARAM) -> LRESULT {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.input.as_mut() {
            let x = Pixels::from(lparam.signed_loword() as f32);
            let y = Pixels::from(lparam.signed_hiword() as f32);
            let event = MouseDownEvent {
                button,
                position: Point { x, y },
                modifiers: self.current_modifiers(),
                click_count: 1,
            };
            if callback(PlatformInput::MouseDown(event)).default_prevented {
                return LRESULT(0);
            }
        }
        LRESULT(1)
    }

    fn handle_mouse_up_msg(&self, button: MouseButton, lparam: LPARAM) -> LRESULT {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.input.as_mut() {
            let x = Pixels::from(lparam.signed_loword() as f32);
            let y = Pixels::from(lparam.signed_hiword() as f32);
            let event = MouseUpEvent {
                button,
                position: Point { x, y },
                modifiers: self.current_modifiers(),
                click_count: 1,
            };
            if callback(PlatformInput::MouseUp(event)).default_prevented {
                return LRESULT(0);
            }
        }
        LRESULT(1)
    }

    fn handle_mouse_wheel_msg(&self, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.input.as_mut() {
            let x = Pixels::from(lparam.signed_loword() as f32);
            let y = Pixels::from(lparam.signed_hiword() as f32);
            let wheel_distance = (wparam.signed_hiword() as f32 / WHEEL_DELTA as f32)
                * self.platform_inner.settings.borrow().wheel_scroll_lines as f32;
            let event = crate::ScrollWheelEvent {
                position: Point { x, y },
                delta: ScrollDelta::Lines(Point {
                    x: 0.0,
                    y: wheel_distance,
                }),
                modifiers: self.current_modifiers(),
                touch_phase: TouchPhase::Moved,
            };
            callback(PlatformInput::ScrollWheel(event));
            return LRESULT(0);
        }
        LRESULT(1)
    }

    fn handle_mouse_horizontal_wheel_msg(&self, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.input.as_mut() {
            let x = Pixels::from(lparam.signed_loword() as f32);
            let y = Pixels::from(lparam.signed_hiword() as f32);
            let wheel_distance = (wparam.signed_hiword() as f32 / WHEEL_DELTA as f32)
                * self.platform_inner.settings.borrow().wheel_scroll_chars as f32;
            let event = crate::ScrollWheelEvent {
                position: Point { x, y },
                delta: ScrollDelta::Lines(Point {
                    x: wheel_distance,
                    y: 0.0,
                }),
                modifiers: self.current_modifiers(),
                touch_phase: TouchPhase::Moved,
            };
            if callback(PlatformInput::ScrollWheel(event)).default_prevented {
                return LRESULT(0);
            }
        }
        LRESULT(1)
    }

    fn handle_ime_position(&self) -> LRESULT {
        unsafe {
            let ctx = ImmGetContext(self.hwnd);
            let Some(mut input_handler) = self.input_handler.take() else {
                return LRESULT(1);
            };
            // we are composing, this should never fail
            let caret_range = input_handler.selected_text_range().unwrap();
            let caret_position = input_handler.bounds_for_range(caret_range).unwrap();
            self.input_handler.set(Some(input_handler));
            let config = CANDIDATEFORM {
                dwStyle: CFS_CANDIDATEPOS,
                ptCurrentPos: POINT {
                    x: caret_position.origin.x.0 as i32,
                    y: caret_position.origin.y.0 as i32 + (caret_position.size.height.0 as i32 / 2),
                },
                ..Default::default()
            };
            ImmSetCandidateWindow(ctx, &config as _);
            ImmReleaseContext(self.hwnd, ctx);
            LRESULT(0)
        }
    }

    fn parse_ime_compostion_string(&self) -> Option<(String, usize)> {
        unsafe {
            let ctx = ImmGetContext(self.hwnd);
            let string_len = ImmGetCompositionStringW(ctx, GCS_COMPSTR, None, 0);
            let result = if string_len >= 0 {
                let mut buffer = vec![0u8; string_len as usize + 2];
                // let mut buffer = [0u8; MAX_PATH as _];
                ImmGetCompositionStringW(
                    ctx,
                    GCS_COMPSTR,
                    Some(buffer.as_mut_ptr() as _),
                    string_len as _,
                );
                let wstring = std::slice::from_raw_parts::<u16>(
                    buffer.as_mut_ptr().cast::<u16>(),
                    string_len as usize / 2,
                );
                let string = String::from_utf16_lossy(wstring);
                Some((string, string_len as usize / 2))
            } else {
                None
            };
            ImmReleaseContext(self.hwnd, ctx);
            result
        }
    }

    fn handle_ime_composition(&self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        if lparam.0 as u32 & GCS_COMPSTR.0 > 0 {
            let Some((string, string_len)) = self.parse_ime_compostion_string() else {
                return unsafe { DefWindowProcW(self.hwnd, msg, wparam, lparam) };
            };
            let Some(mut input_handler) = self.input_handler.take() else {
                return unsafe { DefWindowProcW(self.hwnd, msg, wparam, lparam) };
            };
            input_handler.replace_and_mark_text_in_range(
                None,
                string.as_str(),
                Some(0..string_len),
            );
            self.input_handler.set(Some(input_handler));
            unsafe { DefWindowProcW(self.hwnd, msg, wparam, lparam) }
        } else {
            // currently, we don't care other stuff
            unsafe { DefWindowProcW(self.hwnd, msg, wparam, lparam) }
        }
    }

    fn parse_ime_char(&self, wparam: WPARAM) -> Option<String> {
        let src = [wparam.0 as u16];
        let Ok(first_char) = char::decode_utf16(src).collect::<Vec<_>>()[0] else {
            return None;
        };
        Some(first_char.to_string())
    }

    fn handle_ime_char(&self, wparam: WPARAM) -> LRESULT {
        let Some(ime_char) = self.parse_ime_char(wparam) else {
            return LRESULT(1);
        };
        let Some(mut input_handler) = self.input_handler.take() else {
            return LRESULT(1);
        };
        input_handler.replace_text_in_range(None, &ime_char);
        self.input_handler.set(Some(input_handler));
        self.invalidate_client_area();
        LRESULT(0)
    }

    fn handle_drag_drop(&self, input: PlatformInput) {
        let mut callbacks = self.callbacks.borrow_mut();
        let Some(ref mut func) = callbacks.input else {
            return;
        };
        func(input);
    }

    /// SEE: https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize
    fn handle_calc_client_size(&self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        if wparam.0 == 0 {
            return unsafe { DefWindowProcW(self.hwnd, msg, wparam, lparam) };
        }

        let dpi = unsafe { GetDpiForWindow(self.hwnd) };

        let frame_x = unsafe { GetSystemMetricsForDpi(SM_CXFRAME, dpi) };
        let frame_y = unsafe { GetSystemMetricsForDpi(SM_CYFRAME, dpi) };
        let padding = unsafe { GetSystemMetricsForDpi(SM_CXPADDEDBORDER, dpi) };

        // wparam is TRUE so lparam points to an NCCALCSIZE_PARAMS structure
        let mut params = lparam.0 as *mut NCCALCSIZE_PARAMS;
        let mut requested_client_rect = unsafe { &mut ((*params).rgrc) };

        requested_client_rect[0].right -= frame_x + padding;
        requested_client_rect[0].left += frame_x + padding;
        requested_client_rect[0].bottom -= frame_y + padding;

        LRESULT(0)
    }

    fn handle_activate_msg(&self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        if let Some(titlebar_rect) = self.get_titlebar_rect().log_err() {
            unsafe { InvalidateRect(self.hwnd, Some(&titlebar_rect), FALSE) };
        }
        return unsafe { DefWindowProcW(self.hwnd, msg, wparam, lparam) };
    }

    fn handle_create_msg(&self, _lparam: LPARAM) -> LRESULT {
        let mut size_rect = RECT::default();
        unsafe { GetWindowRect(self.hwnd, &mut size_rect).log_err() };
        let width = size_rect.right - size_rect.left;
        let height = size_rect.bottom - size_rect.top;

        self.size.set(Size {
            width: GlobalPixels::from(width as f64),
            height: GlobalPixels::from(height as f64),
        });

        // Inform the application of the frame change to force redrawing with the new
        // client area that is extended into the title bar
        unsafe {
            SetWindowPos(
                self.hwnd,
                HWND::default(),
                size_rect.left,
                size_rect.top,
                width,
                height,
                SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE,
            )
            .log_err()
        };
        LRESULT(0)
    }

    fn handle_dpi_changed_msg(&self, _msg: u32, _wparam: WPARAM, _lparam: LPARAM) -> LRESULT {
        LRESULT(1)
    }

    fn handle_hit_test_msg(&self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        // default handler for resize areas
        let hit = unsafe { DefWindowProcW(self.hwnd, msg, wparam, lparam) };
        if matches!(
            hit.0 as u32,
            HTNOWHERE
                | HTRIGHT
                | HTLEFT
                | HTTOPLEFT
                | HTTOP
                | HTTOPRIGHT
                | HTBOTTOMRIGHT
                | HTBOTTOM
                | HTBOTTOMLEFT
        ) {
            return hit;
        }

        let dpi = unsafe { GetDpiForWindow(self.hwnd) };
        let frame_y = unsafe { GetSystemMetricsForDpi(SM_CYFRAME, dpi) };
        let padding = unsafe { GetSystemMetricsForDpi(SM_CXPADDEDBORDER, dpi) };

        let mut cursor_point = POINT {
            x: lparam.signed_loword().into(),
            y: lparam.signed_hiword().into(),
        };
        unsafe { ScreenToClient(self.hwnd, &mut cursor_point) };
        if cursor_point.y > 0 && cursor_point.y < frame_y + padding {
            return LRESULT(HTTOP as _);
        }

        let titlebar_rect = self.get_titlebar_rect();
        if let Ok(titlebar_rect) = titlebar_rect {
            if cursor_point.y < titlebar_rect.bottom {
                let caption_btn_width = unsafe { GetSystemMetricsForDpi(SM_CXSIZE, dpi) };
                if cursor_point.x >= titlebar_rect.right - caption_btn_width {
                    return LRESULT(HTCLOSE as _);
                } else if cursor_point.x >= titlebar_rect.right - caption_btn_width * 2 {
                    return LRESULT(HTMAXBUTTON as _);
                } else if cursor_point.x >= titlebar_rect.right - caption_btn_width * 3 {
                    return LRESULT(HTMINBUTTON as _);
                }

                return LRESULT(HTCAPTION as _);
            }
        }

        LRESULT(HTCLIENT as _)
    }

    fn handle_nc_mouse_move_msg(&self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        let mut cursor_point = POINT {
            x: lparam.signed_loword().into(),
            y: lparam.signed_hiword().into(),
        };
        unsafe { ScreenToClient(self.hwnd, &mut cursor_point) };
        let x = Pixels::from(cursor_point.x as f32);
        let y = Pixels::from(cursor_point.y as f32);
        self.mouse_position.set(Point { x, y });
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.input.as_mut() {
            let event = MouseMoveEvent {
                position: Point { x, y },
                pressed_button: None,
                modifiers: self.current_modifiers(),
            };
            if callback(PlatformInput::MouseMove(event)).default_prevented {
                return LRESULT(0);
            }
        }
        drop(callbacks);
        unsafe { DefWindowProcW(self.hwnd, msg, wparam, lparam) }
    }

    fn handle_nc_mouse_down_msg(
        &self,
        button: MouseButton,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.input.as_mut() {
            let mut cursor_point = POINT {
                x: lparam.signed_loword().into(),
                y: lparam.signed_hiword().into(),
            };
            unsafe { ScreenToClient(self.hwnd, &mut cursor_point) };
            let x = Pixels::from(cursor_point.x as f32);
            let y = Pixels::from(cursor_point.y as f32);
            let event = MouseDownEvent {
                button: button.clone(),
                position: Point { x, y },
                modifiers: self.current_modifiers(),
                click_count: 1,
            };
            if callback(PlatformInput::MouseDown(event)).default_prevented {
                return LRESULT(0);
            }
        }
        drop(callbacks);

        match wparam.0 as u32 {
            // Since these are handled in handle_nc_mouse_up_msg we must prevent the default window proc
            HTMINBUTTON | HTMAXBUTTON | HTCLOSE => LRESULT(0),
            _ => unsafe { DefWindowProcW(self.hwnd, msg, wparam, lparam) },
        }
    }

    fn handle_nc_mouse_up_msg(
        &self,
        button: MouseButton,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.input.as_mut() {
            let mut cursor_point = POINT {
                x: lparam.signed_loword().into(),
                y: lparam.signed_hiword().into(),
            };
            unsafe { ScreenToClient(self.hwnd, &mut cursor_point) };
            let x = Pixels::from(cursor_point.x as f32);
            let y = Pixels::from(cursor_point.y as f32);
            let event = MouseUpEvent {
                button,
                position: Point { x, y },
                modifiers: self.current_modifiers(),
                click_count: 1,
            };
            if callback(PlatformInput::MouseUp(event)).default_prevented {
                return LRESULT(0);
            }
        }
        drop(callbacks);

        if button == MouseButton::Left {
            match wparam.0 as u32 {
                HTMINBUTTON => unsafe {
                    ShowWindowAsync(self.hwnd, SW_MINIMIZE);
                    return LRESULT(0);
                },
                HTMAXBUTTON => unsafe {
                    if self.is_maximized() {
                        ShowWindowAsync(self.hwnd, SW_NORMAL);
                    } else {
                        ShowWindowAsync(self.hwnd, SW_MAXIMIZE);
                    }
                    return LRESULT(0);
                },
                HTCLOSE => unsafe {
                    PostMessageW(self.hwnd, WM_CLOSE, WPARAM::default(), LPARAM::default())
                        .log_err();
                    return LRESULT(0);
                },
                _ => {}
            };
        }

        unsafe { DefWindowProcW(self.hwnd, msg, wparam, lparam) }
    }
}

#[derive(Default)]
struct Callbacks {
    request_frame: Option<Box<dyn FnMut()>>,
    input: Option<Box<dyn FnMut(crate::PlatformInput) -> DispatchEventResult>>,
    active_status_change: Option<Box<dyn FnMut(bool)>>,
    resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    fullscreen: Option<Box<dyn FnMut(bool)>>,
    moved: Option<Box<dyn FnMut()>>,
    should_close: Option<Box<dyn FnMut() -> bool>>,
    close: Option<Box<dyn FnOnce()>>,
    appearance_changed: Option<Box<dyn FnMut()>>,
}

pub(crate) struct WindowsWindow {
    inner: Rc<WindowsWindowInner>,
    drag_drop_handler: IDropTarget,
    display: Rc<WindowsDisplay>,
}

struct WindowCreateContext {
    inner: Option<Rc<WindowsWindowInner>>,
    platform_inner: Rc<WindowsPlatformInner>,
    handle: AnyWindowHandle,
}

impl WindowsWindow {
    pub(crate) fn new(
        platform_inner: Rc<WindowsPlatformInner>,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> Self {
        let classname = register_wnd_class();
        let windowname = HSTRING::from(
            options
                .titlebar
                .as_ref()
                .and_then(|titlebar| titlebar.title.as_ref())
                .map(|title| title.as_ref())
                .unwrap_or(""),
        );
        let dwstyle = WS_THICKFRAME | WS_SYSMENU | WS_MAXIMIZEBOX | WS_MINIMIZEBOX;
        let x = options.bounds.origin.x.0 as i32;
        let y = options.bounds.origin.y.0 as i32;
        let nwidth = options.bounds.size.width.0 as i32;
        let nheight = options.bounds.size.height.0 as i32;
        let hwndparent = HWND::default();
        let hmenu = HMENU::default();
        let hinstance = HINSTANCE::default();
        let mut context = WindowCreateContext {
            inner: None,
            platform_inner: platform_inner.clone(),
            handle,
        };
        let lpparam = Some(&context as *const _ as *const _);
        unsafe {
            CreateWindowExW(
                WS_EX_APPWINDOW,
                classname,
                &windowname,
                dwstyle,
                x,
                y,
                nwidth,
                nheight,
                hwndparent,
                hmenu,
                hinstance,
                lpparam,
            )
        };
        let drag_drop_handler = {
            let inner = context.inner.as_ref().unwrap();
            let handler = WindowsDragDropHandler(Rc::clone(inner));
            let drag_drop_handler: IDropTarget = handler.into();
            unsafe {
                RegisterDragDrop(inner.hwnd, &drag_drop_handler)
                    .expect("unable to register drag-drop event")
            };
            drag_drop_handler
        };
        // todo(windows) move window to target monitor
        // options.display_id
        let wnd = Self {
            inner: context.inner.unwrap(),
            drag_drop_handler,
            display: Rc::new(WindowsDisplay::primary_monitor().unwrap()),
        };
        platform_inner
            .raw_window_handles
            .write()
            .push(wnd.inner.hwnd);

        unsafe { ShowWindow(wnd.inner.hwnd, SW_SHOW) };
        wnd
    }

    fn maximize(&self) {
        unsafe { ShowWindowAsync(self.inner.hwnd, SW_MAXIMIZE) };
    }
}

impl HasWindowHandle for WindowsWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let raw = raw_window_handle::Win32WindowHandle::new(unsafe {
            NonZeroIsize::new_unchecked(self.inner.hwnd.0)
        })
        .into();
        Ok(unsafe { raw_window_handle::WindowHandle::borrow_raw(raw) })
    }
}

// todo(windows)
impl HasDisplayHandle for WindowsWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        unimplemented!()
    }
}

impl Drop for WindowsWindow {
    fn drop(&mut self) {
        unsafe {
            let _ = RevokeDragDrop(self.inner.hwnd);
        }
    }
}

impl PlatformWindow for WindowsWindow {
    fn bounds(&self) -> Bounds<GlobalPixels> {
        Bounds {
            origin: self.inner.origin.get(),
            size: self.inner.size.get(),
        }
    }

    fn is_maximized(&self) -> bool {
        self.inner.is_maximized()
    }

    // todo(windows)
    fn content_size(&self) -> Size<Pixels> {
        let size = self.inner.size.get();
        Size {
            width: size.width.0.into(),
            height: size.height.0.into(),
        }
    }

    // todo(windows)
    fn scale_factor(&self) -> f32 {
        self.inner.scale_factor
    }

    fn titlebar_height(&self) -> Pixels {
        let titlebar_rect = self.inner.get_titlebar_rect().unwrap();
        ((titlebar_rect.bottom - titlebar_rect.top) as f64).into()
    }

    // todo(windows)
    fn appearance(&self) -> WindowAppearance {
        WindowAppearance::Dark
    }

    fn display(&self) -> Rc<dyn PlatformDisplay> {
        self.display.clone()
    }

    fn mouse_position(&self) -> Point<Pixels> {
        self.inner.mouse_position.get()
    }

    // todo(windows)
    fn modifiers(&self) -> Modifiers {
        Modifiers::none()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    // todo(windows)
    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.inner.input_handler.set(Some(input_handler));
    }

    // todo(windows)
    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.inner.input_handler.take()
    }

    fn prompt(
        &self,
        level: PromptLevel,
        msg: &str,
        detail: Option<&str>,
        answers: &[&str],
    ) -> Option<Receiver<usize>> {
        let (done_tx, done_rx) = oneshot::channel();
        let msg = msg.to_string();
        let detail_string = match detail {
            Some(info) => Some(info.to_string()),
            None => None,
        };
        let answers = answers.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let handle = self.inner.hwnd;
        self.inner
            .platform_inner
            .foreground_executor
            .spawn(async move {
                unsafe {
                    let mut config;
                    config = std::mem::zeroed::<TASKDIALOGCONFIG>();
                    config.cbSize = std::mem::size_of::<TASKDIALOGCONFIG>() as _;
                    config.hwndParent = handle;
                    let title;
                    let main_icon;
                    match level {
                        crate::PromptLevel::Info => {
                            title = windows::core::w!("Info");
                            main_icon = TD_INFORMATION_ICON;
                        }
                        crate::PromptLevel::Warning => {
                            title = windows::core::w!("Warning");
                            main_icon = TD_WARNING_ICON;
                        }
                        crate::PromptLevel::Critical => {
                            title = windows::core::w!("Critical");
                            main_icon = TD_ERROR_ICON;
                        }
                    };
                    config.pszWindowTitle = title;
                    config.Anonymous1.pszMainIcon = main_icon;
                    let instruction = msg.encode_utf16().chain(once(0)).collect_vec();
                    config.pszMainInstruction = PCWSTR::from_raw(instruction.as_ptr());
                    let hints_encoded;
                    if let Some(ref hints) = detail_string {
                        hints_encoded = hints.encode_utf16().chain(once(0)).collect_vec();
                        config.pszContent = PCWSTR::from_raw(hints_encoded.as_ptr());
                    };
                    let mut buttons = Vec::new();
                    let mut btn_encoded = Vec::new();
                    for (index, btn_string) in answers.iter().enumerate() {
                        let encoded = btn_string.encode_utf16().chain(once(0)).collect_vec();
                        buttons.push(TASKDIALOG_BUTTON {
                            nButtonID: index as _,
                            pszButtonText: PCWSTR::from_raw(encoded.as_ptr()),
                        });
                        btn_encoded.push(encoded);
                    }
                    config.cButtons = buttons.len() as _;
                    config.pButtons = buttons.as_ptr();

                    config.pfCallback = None;
                    let mut res = std::mem::zeroed();
                    let _ = TaskDialogIndirect(&config, Some(&mut res), None, None)
                        .inspect_err(|e| log::error!("unable to create task dialog: {}", e));

                    let _ = done_tx.send(res as usize);
                }
            })
            .detach();

        Some(done_rx)
    }

    fn activate(&self) {
        unsafe { ShowWindowAsync(self.inner.hwnd, SW_NORMAL) };
    }

    // todo(windows)
    fn set_title(&mut self, title: &str) {
        unsafe { SetWindowTextW(self.inner.hwnd, &HSTRING::from(title)) }
            .inspect_err(|e| log::error!("Set title failed: {e}"))
            .ok();
    }

    // todo(windows)
    fn set_edited(&mut self, _edited: bool) {}

    // todo(windows)
    fn show_character_palette(&self) {}

    fn minimize(&self) {
        unsafe { ShowWindowAsync(self.inner.hwnd, SW_MINIMIZE) };
    }

    fn zoom(&self) {
        unsafe { ShowWindowAsync(self.inner.hwnd, SW_MAXIMIZE) };
    }

    // todo(windows)
    fn toggle_fullscreen(&self) {}

    // todo(windows)
    fn is_fullscreen(&self) -> bool {
        false
    }

    // todo(windows)
    fn on_request_frame(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().request_frame = Some(callback);
    }

    // todo(windows)
    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        self.inner.callbacks.borrow_mut().input = Some(callback);
    }

    // todo(windows)
    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.inner.callbacks.borrow_mut().active_status_change = Some(callback);
    }

    // todo(windows)
    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.inner.callbacks.borrow_mut().resize = Some(callback);
    }

    // todo(windows)
    fn on_fullscreen(&self, callback: Box<dyn FnMut(bool)>) {
        self.inner.callbacks.borrow_mut().fullscreen = Some(callback);
    }

    // todo(windows)
    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().moved = Some(callback);
    }

    // todo(windows)
    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.inner.callbacks.borrow_mut().should_close = Some(callback);
    }

    // todo(windows)
    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.inner.callbacks.borrow_mut().close = Some(callback);
    }

    // todo(windows)
    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().appearance_changed = Some(callback);
    }

    // todo(windows)
    fn is_topmost_for_position(&self, _position: Point<Pixels>) -> bool {
        true
    }

    // todo(windows)
    fn draw(&self, scene: &Scene) {
        self.inner.renderer.borrow_mut().draw(scene)
    }

    // todo(windows)
    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.inner.renderer.borrow().sprite_atlas().clone()
    }
}

#[implement(IDropTarget)]
struct WindowsDragDropHandler(pub Rc<WindowsWindowInner>);

#[allow(non_snake_case)]
impl IDropTarget_Impl for WindowsDragDropHandler {
    fn DragEnter(
        &self,
        pdataobj: Option<&IDataObject>,
        _grfkeystate: MODIFIERKEYS_FLAGS,
        pt: &POINTL,
        pdweffect: *mut DROPEFFECT,
    ) -> windows::core::Result<()> {
        unsafe {
            let Some(idata_obj) = pdataobj else {
                log::info!("no dragging file or directory detected");
                return Ok(());
            };
            let config = FORMATETC {
                cfFormat: CF_HDROP.0,
                ptd: std::ptr::null_mut() as _,
                dwAspect: DVASPECT_CONTENT.0,
                lindex: -1,
                tymed: TYMED_HGLOBAL.0 as _,
            };
            let mut paths = SmallVec::<[PathBuf; 2]>::new();
            if idata_obj.QueryGetData(&config as _) == S_OK {
                *pdweffect = DROPEFFECT_LINK;
                let Ok(mut idata) = idata_obj.GetData(&config as _) else {
                    return Ok(());
                };
                if idata.u.hGlobal.is_invalid() {
                    return Ok(());
                }
                let hdrop = idata.u.hGlobal.0 as *mut HDROP;
                let file_count = DragQueryFileW(*hdrop, DRAGDROP_GET_FILES_COUNT, None);
                for file_index in 0..file_count {
                    let filename_length = DragQueryFileW(*hdrop, file_index, None) as usize;
                    let mut buffer = vec![0u16; filename_length + 1];
                    let ret = DragQueryFileW(*hdrop, file_index, Some(buffer.as_mut_slice()));
                    if ret == 0 {
                        log::error!("unable to read file name");
                        continue;
                    }
                    if let Ok(file_name) = String::from_utf16(&buffer[0..filename_length]) {
                        if let Ok(path) = PathBuf::from_str(&file_name) {
                            paths.push(path);
                        }
                    }
                }
                ReleaseStgMedium(&mut idata);
                let input = PlatformInput::FileDrop(crate::FileDropEvent::Entered {
                    position: Point {
                        x: Pixels(pt.x as _),
                        y: Pixels(pt.y as _),
                    },
                    paths: crate::ExternalPaths(paths),
                });
                self.0.handle_drag_drop(input);
            } else {
                *pdweffect = DROPEFFECT_NONE;
            }
        }
        Ok(())
    }

    fn DragOver(
        &self,
        _grfkeystate: MODIFIERKEYS_FLAGS,
        pt: &POINTL,
        _pdweffect: *mut DROPEFFECT,
    ) -> windows::core::Result<()> {
        let input = PlatformInput::FileDrop(crate::FileDropEvent::Pending {
            position: Point {
                x: Pixels(pt.x as _),
                y: Pixels(pt.y as _),
            },
        });
        self.0.handle_drag_drop(input);

        Ok(())
    }

    fn DragLeave(&self) -> windows::core::Result<()> {
        let input = PlatformInput::FileDrop(crate::FileDropEvent::Exited);
        self.0.handle_drag_drop(input);

        Ok(())
    }

    fn Drop(
        &self,
        _pdataobj: Option<&IDataObject>,
        _grfkeystate: MODIFIERKEYS_FLAGS,
        pt: &POINTL,
        _pdweffect: *mut DROPEFFECT,
    ) -> windows::core::Result<()> {
        let input = PlatformInput::FileDrop(crate::FileDropEvent::Submit {
            position: Point {
                x: Pixels(pt.x as _),
                y: Pixels(pt.y as _),
            },
        });
        self.0.handle_drag_drop(input);

        Ok(())
    }
}

fn register_wnd_class() -> PCWSTR {
    const CLASS_NAME: PCWSTR = w!("Zed::Window");

    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let wc = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW).ok().unwrap() },
            lpszClassName: PCWSTR(CLASS_NAME.as_ptr()),
            style: CS_HREDRAW | CS_VREDRAW,
            ..Default::default()
        };
        unsafe { RegisterClassW(&wc) };
    });

    CLASS_NAME
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_NCCREATE {
        let cs = lparam.0 as *const CREATESTRUCTW;
        let cs = unsafe { &*cs };
        let ctx = cs.lpCreateParams as *mut WindowCreateContext;
        let ctx = unsafe { &mut *ctx };
        let inner = Rc::new(WindowsWindowInner::new(
            hwnd,
            cs,
            ctx.platform_inner.clone(),
            ctx.handle,
        ));
        let weak = Box::new(Rc::downgrade(&inner));
        unsafe { set_window_long(hwnd, GWLP_USERDATA, Box::into_raw(weak) as isize) };
        ctx.inner = Some(inner);
        return LRESULT(1);
    }
    let ptr = unsafe { get_window_long(hwnd, GWLP_USERDATA) } as *mut Weak<WindowsWindowInner>;
    if ptr.is_null() {
        return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
    }
    let inner = unsafe { &*ptr };
    let r = if let Some(inner) = inner.upgrade() {
        inner.handle_msg(msg, wparam, lparam)
    } else {
        unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
    };
    if msg == WM_NCDESTROY {
        unsafe { set_window_long(hwnd, GWLP_USERDATA, 0) };
        unsafe { std::mem::drop(Box::from_raw(ptr)) };
    }
    r
}

pub(crate) fn try_get_window_inner(hwnd: HWND) -> Option<Rc<WindowsWindowInner>> {
    let ptr = unsafe { get_window_long(hwnd, GWLP_USERDATA) } as *mut Weak<WindowsWindowInner>;
    if !ptr.is_null() {
        let inner = unsafe { &*ptr };
        inner.upgrade()
    } else {
        None
    }
}

fn basic_vkcode_to_string(code: u16, modifiers: Modifiers) -> Option<Keystroke> {
    match code {
        // VK_0 - VK_9
        48..=57 => Some(Keystroke {
            modifiers,
            key: format!("{}", code - VK_0.0),
            ime_key: None,
        }),
        // VK_A - VK_Z
        65..=90 => Some(Keystroke {
            modifiers,
            key: format!("{}", (b'a' + code as u8 - VK_A.0 as u8) as char),
            ime_key: None,
        }),
        // VK_F1 - VK_F24
        112..=135 => Some(Keystroke {
            modifiers,
            key: format!("f{}", code - VK_F1.0 + 1),
            ime_key: None,
        }),
        // OEM3: `/~, OEM_MINUS: -/_, OEM_PLUS: =/+, ...
        _ => {
            if let Some(key) = oemkey_vkcode_to_string(code) {
                Some(Keystroke {
                    modifiers,
                    key,
                    ime_key: None,
                })
            } else {
                None
            }
        }
    }
}

fn oemkey_vkcode_to_string(code: u16) -> Option<String> {
    match code {
        186 => Some(";".to_string()), // VK_OEM_1
        187 => Some("=".to_string()), // VK_OEM_PLUS
        188 => Some(",".to_string()), // VK_OEM_COMMA
        189 => Some("-".to_string()), // VK_OEM_MINUS
        190 => Some(".".to_string()), // VK_OEM_PERIOD
        // https://kbdlayout.info/features/virtualkeys/VK_ABNT_C1
        191 | 193 => Some("/".to_string()), // VK_OEM_2 VK_ABNT_C1
        192 => Some("`".to_string()),       // VK_OEM_3
        219 => Some("[".to_string()),       // VK_OEM_4
        220 => Some("\\".to_string()),      // VK_OEM_5
        221 => Some("]".to_string()),       // VK_OEM_6
        222 => Some("'".to_string()),       // VK_OEM_7
        _ => None,
    }
}

// https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragqueryfilew
const DRAGDROP_GET_FILES_COUNT: u32 = 0xFFFFFFFF;
