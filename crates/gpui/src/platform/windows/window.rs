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
    time::{Duration, Instant},
};

use ::util::ResultExt;
use anyhow::Context;
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
    origin: Cell<Point<DevicePixels>>,
    physical_size: Cell<Size<DevicePixels>>,
    scale_factor: Cell<f32>,
    input_handler: Cell<Option<PlatformInputHandler>>,
    renderer: RefCell<BladeRenderer>,
    callbacks: RefCell<Callbacks>,
    platform_inner: Rc<WindowsPlatformInner>,
    pub(crate) handle: AnyWindowHandle,
    hide_title_bar: bool,
    display: RefCell<Rc<WindowsDisplay>>,
    last_ime_input: RefCell<Option<String>>,
    click_state: RefCell<ClickState>,
    fullscreen: Cell<Option<StyleAndBounds>>,
}

impl WindowsWindowInner {
    fn new(
        hwnd: HWND,
        cs: &CREATESTRUCTW,
        platform_inner: Rc<WindowsPlatformInner>,
        handle: AnyWindowHandle,
        hide_title_bar: bool,
        display: Rc<WindowsDisplay>,
    ) -> Self {
        let monitor_dpi = unsafe { GetDpiForWindow(hwnd) } as f32;
        let origin = Cell::new(Point {
            x: DevicePixels(cs.x),
            y: DevicePixels(cs.y),
        });
        let physical_size = Cell::new(Size {
            width: DevicePixels(cs.cx),
            height: DevicePixels(cs.cy),
        });
        let scale_factor = Cell::new(monitor_dpi / USER_DEFAULT_SCREEN_DPI as f32);
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
        let display = RefCell::new(display);
        let last_ime_input = RefCell::new(None);
        let click_state = RefCell::new(ClickState::new());
        let fullscreen = Cell::new(None);
        Self {
            hwnd,
            origin,
            physical_size,
            scale_factor,
            input_handler,
            renderer,
            callbacks,
            platform_inner,
            handle,
            hide_title_bar,
            display,
            last_ime_input,
            click_state,
            fullscreen,
        }
    }

    fn is_maximized(&self) -> bool {
        !self.is_fullscreen() && unsafe { IsZoomed(self.hwnd) }.as_bool()
    }

    fn is_minimized(&self) -> bool {
        unsafe { IsIconic(self.hwnd) }.as_bool()
    }

    fn is_fullscreen(&self) -> bool {
        let fullscreen = self.fullscreen.take();
        let is_fullscreen = fullscreen.is_some();
        self.fullscreen.set(fullscreen);
        is_fullscreen
    }

    async fn toggle_fullscreen(self: Rc<Self>) {
        let StyleAndBounds {
            style,
            x,
            y,
            cx,
            cy,
        } = if let Some(state) = self.fullscreen.take() {
            state
        } else {
            let style = WINDOW_STYLE(unsafe { get_window_long(self.hwnd, GWL_STYLE) } as _);
            let mut rc = RECT::default();
            unsafe { GetWindowRect(self.hwnd, &mut rc) }.log_err();
            self.fullscreen.set(Some(StyleAndBounds {
                style,
                x: rc.left,
                y: rc.top,
                cx: rc.right - rc.left,
                cy: rc.bottom - rc.top,
            }));
            let style = style
                & !(WS_THICKFRAME | WS_SYSMENU | WS_MAXIMIZEBOX | WS_MINIMIZEBOX | WS_CAPTION);
            let bounds = self.display.borrow().clone().bounds();
            StyleAndBounds {
                style,
                x: bounds.left().0,
                y: bounds.top().0,
                cx: bounds.size.width.0,
                cy: bounds.size.height.0,
            }
        };
        unsafe { set_window_long(self.hwnd, GWL_STYLE, style.0 as isize) };
        unsafe {
            SetWindowPos(
                self.hwnd,
                HWND::default(),
                x,
                y,
                cx,
                cy,
                SWP_FRAMECHANGED | SWP_NOACTIVATE | SWP_NOZORDER,
            )
        }
        .log_err();
    }

    pub(crate) fn title_bar_padding(&self) -> Pixels {
        // using USER_DEFAULT_SCREEN_DPI because GPUI handles the scale with the scale factor
        let padding = unsafe { GetSystemMetricsForDpi(SM_CXPADDEDBORDER, USER_DEFAULT_SCREEN_DPI) };
        px(padding as f32)
    }

    pub(crate) fn title_bar_top_offset(&self) -> Pixels {
        if self.is_maximized() {
            self.title_bar_padding() * 2
        } else {
            px(0.)
        }
    }

    pub(crate) fn title_bar_height(&self) -> Pixels {
        // todo(windows) this is hard set to match the ui title bar
        //               in the future the ui title bar component will report the size
        px(32.) + self.title_bar_top_offset()
    }

    pub(crate) fn caption_button_width(&self) -> Pixels {
        // todo(windows) this is hard set to match the ui title bar
        //               in the future the ui title bar component will report the size
        px(36.)
    }

    fn get_titlebar_rect(&self) -> anyhow::Result<RECT> {
        let height = self.title_bar_height();
        let mut rect = RECT::default();
        unsafe { GetClientRect(self.hwnd, &mut rect) }?;
        rect.bottom = rect.top + ((height.0 * self.scale_factor.get()).round() as i32);
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
        let handled = match msg {
            WM_ACTIVATE => self.handle_activate_msg(wparam),
            WM_CREATE => self.handle_create_msg(lparam),
            WM_MOVE => self.handle_move_msg(lparam),
            WM_SIZE => self.handle_size_msg(lparam),
            WM_NCCALCSIZE => self.handle_calc_client_size(wparam, lparam),
            WM_DPICHANGED => self.handle_dpi_changed_msg(wparam, lparam),
            WM_NCHITTEST => self.handle_hit_test_msg(msg, wparam, lparam),
            WM_PAINT => self.handle_paint_msg(),
            WM_CLOSE => self.handle_close_msg(),
            WM_DESTROY => self.handle_destroy_msg(),
            WM_MOUSEMOVE => self.handle_mouse_move_msg(lparam, wparam),
            WM_NCMOUSEMOVE => self.handle_nc_mouse_move_msg(lparam),
            WM_NCLBUTTONDOWN => self.handle_nc_mouse_down_msg(MouseButton::Left, wparam, lparam),
            WM_NCRBUTTONDOWN => self.handle_nc_mouse_down_msg(MouseButton::Right, wparam, lparam),
            WM_NCMBUTTONDOWN => self.handle_nc_mouse_down_msg(MouseButton::Middle, wparam, lparam),
            WM_NCLBUTTONUP => self.handle_nc_mouse_up_msg(MouseButton::Left, wparam, lparam),
            WM_NCRBUTTONUP => self.handle_nc_mouse_up_msg(MouseButton::Right, wparam, lparam),
            WM_NCMBUTTONUP => self.handle_nc_mouse_up_msg(MouseButton::Middle, wparam, lparam),
            WM_LBUTTONDOWN => self.handle_mouse_down_msg(MouseButton::Left, lparam),
            WM_RBUTTONDOWN => self.handle_mouse_down_msg(MouseButton::Right, lparam),
            WM_MBUTTONDOWN => self.handle_mouse_down_msg(MouseButton::Middle, lparam),
            WM_XBUTTONDOWN => self.handle_xbutton_msg(wparam, lparam, Self::handle_mouse_down_msg),
            WM_LBUTTONUP => self.handle_mouse_up_msg(MouseButton::Left, lparam),
            WM_RBUTTONUP => self.handle_mouse_up_msg(MouseButton::Right, lparam),
            WM_MBUTTONUP => self.handle_mouse_up_msg(MouseButton::Middle, lparam),
            WM_XBUTTONUP => self.handle_xbutton_msg(wparam, lparam, Self::handle_mouse_up_msg),
            WM_MOUSEWHEEL => self.handle_mouse_wheel_msg(wparam, lparam),
            WM_MOUSEHWHEEL => self.handle_mouse_horizontal_wheel_msg(wparam, lparam),
            WM_SYSKEYDOWN => self.handle_syskeydown_msg(wparam, lparam),
            WM_SYSKEYUP => self.handle_syskeyup_msg(wparam),
            WM_KEYDOWN => self.handle_keydown_msg(msg, wparam, lparam),
            WM_KEYUP => self.handle_keyup_msg(msg, wparam),
            WM_CHAR => self.handle_char_msg(msg, wparam, lparam),
            WM_IME_STARTCOMPOSITION => self.handle_ime_position(),
            WM_IME_COMPOSITION => self.handle_ime_composition(lparam),
            WM_IME_CHAR => self.handle_ime_char(wparam),
            WM_SETCURSOR => self.handle_set_cursor(lparam),
            _ => None,
        };
        if let Some(n) = handled {
            LRESULT(n)
        } else {
            unsafe { DefWindowProcW(self.hwnd, msg, wparam, lparam) }
        }
    }

    fn handle_move_msg(&self, lparam: LPARAM) -> Option<isize> {
        let x = lparam.signed_loword() as i32;
        let y = lparam.signed_hiword() as i32;
        self.origin.set(Point {
            x: DevicePixels(x),
            y: DevicePixels(y),
        });
        let size = self.physical_size.get();
        let center_x = x + size.width.0 / 2;
        let center_y = y + size.height.0 / 2;
        let monitor_bounds = self.display.borrow().bounds();
        if center_x < monitor_bounds.left().0
            || center_x > monitor_bounds.right().0
            || center_y < monitor_bounds.top().0
            || center_y > monitor_bounds.bottom().0
        {
            // center of the window may have moved to another monitor
            let monitor = unsafe { MonitorFromWindow(self.hwnd, MONITOR_DEFAULTTONULL) };
            if !monitor.is_invalid() && self.display.borrow().handle != monitor {
                // we will get the same monitor if we only have one
                (*self.display.borrow_mut()) = Rc::new(WindowsDisplay::new_with_handle(monitor));
            }
        }
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.moved.as_mut() {
            callback()
        }
        Some(0)
    }

    fn handle_size_msg(&self, lparam: LPARAM) -> Option<isize> {
        let width = lparam.loword().max(1) as i32;
        let height = lparam.hiword().max(1) as i32;
        let scale_factor = self.scale_factor.get();
        let new_physical_size = Size {
            width: DevicePixels(width),
            height: DevicePixels(height),
        };
        self.physical_size.set(new_physical_size);
        self.renderer.borrow_mut().update_drawable_size(Size {
            width: width as f64,
            height: height as f64,
        });
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.resize.as_mut() {
            let logical_size = logical_size(new_physical_size, scale_factor);
            callback(logical_size, scale_factor);
        }
        self.invalidate_client_area();
        Some(0)
    }

    fn handle_paint_msg(&self) -> Option<isize> {
        let mut paint_struct = PAINTSTRUCT::default();
        let _hdc = unsafe { BeginPaint(self.hwnd, &mut paint_struct) };
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(request_frame) = callbacks.request_frame.as_mut() {
            request_frame();
        }
        unsafe { EndPaint(self.hwnd, &paint_struct) };
        Some(0)
    }

    fn handle_close_msg(&self) -> Option<isize> {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.should_close.as_mut() {
            if callback() {
                return Some(0);
            }
        }
        None
    }

    fn handle_destroy_msg(&self) -> Option<isize> {
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
        Some(1)
    }

    fn handle_mouse_move_msg(&self, lparam: LPARAM, wparam: WPARAM) -> Option<isize> {
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
            let x = lparam.signed_loword() as f32;
            let y = lparam.signed_hiword() as f32;
            let scale_factor = self.scale_factor.get();
            let event = MouseMoveEvent {
                position: logical_point(x, y, scale_factor),
                pressed_button,
                modifiers: self.current_modifiers(),
            };
            if callback(PlatformInput::MouseMove(event)).default_prevented {
                return Some(0);
            }
        }
        Some(1)
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
            let mut modifiers = self.current_modifiers();
            // for characters that use 'shift' to type it is expected that the
            // shift is not reported if the uppercase/lowercase are the same and instead only the key is reported
            if first_char.to_lowercase().to_string() == first_char.to_uppercase().to_string() {
                modifiers.shift = false;
            }
            let key = match first_char {
                ' ' => "space".to_string(),
                first_char => first_char.to_lowercase().to_string(),
            };
            Some(Keystroke {
                modifiers,
                key,
                ime_key: Some(first_char.to_string()),
            })
        }
    }

    fn handle_syskeydown_msg(&self, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        // we need to call `DefWindowProcW`, or we will lose the system-wide `Alt+F4`, `Alt+{other keys}`
        // shortcuts.
        let Some(keystroke) = self.parse_syskeydown_msg_keystroke(wparam) else {
            return None;
        };
        let Some(ref mut func) = self.callbacks.borrow_mut().input else {
            return None;
        };
        let event = KeyDownEvent {
            keystroke,
            is_held: lparam.0 & (0x1 << 30) > 0,
        };
        if func(PlatformInput::KeyDown(event)).default_prevented {
            self.invalidate_client_area();
            return Some(0);
        }
        None
    }

    fn handle_syskeyup_msg(&self, wparam: WPARAM) -> Option<isize> {
        // we need to call `DefWindowProcW`, or we will lose the system-wide `Alt+F4`, `Alt+{other keys}`
        // shortcuts.
        let Some(keystroke) = self.parse_syskeydown_msg_keystroke(wparam) else {
            return None;
        };
        let Some(ref mut func) = self.callbacks.borrow_mut().input else {
            return None;
        };
        let event = KeyUpEvent { keystroke };
        if func(PlatformInput::KeyUp(event)).default_prevented {
            self.invalidate_client_area();
            return Some(0);
        }
        None
    }

    fn handle_keydown_msg(&self, _msg: u32, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        let Some(keystroke) = self.parse_keydown_msg_keystroke(wparam) else {
            return Some(1);
        };
        let Some(ref mut func) = self.callbacks.borrow_mut().input else {
            return Some(1);
        };
        let event = KeyDownEvent {
            keystroke,
            is_held: lparam.0 & (0x1 << 30) > 0,
        };
        if func(PlatformInput::KeyDown(event)).default_prevented {
            self.invalidate_client_area();
            return Some(0);
        }
        Some(1)
    }

    fn handle_keyup_msg(&self, _msg: u32, wparam: WPARAM) -> Option<isize> {
        let Some(keystroke) = self.parse_keydown_msg_keystroke(wparam) else {
            return Some(1);
        };
        let Some(ref mut func) = self.callbacks.borrow_mut().input else {
            return Some(1);
        };
        let event = KeyUpEvent { keystroke };
        if func(PlatformInput::KeyUp(event)).default_prevented {
            self.invalidate_client_area();
            return Some(0);
        }
        Some(1)
    }

    fn handle_char_msg(&self, _msg: u32, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        let Some(keystroke) = self.parse_char_msg_keystroke(wparam) else {
            return Some(1);
        };
        let mut callbacks = self.callbacks.borrow_mut();
        let Some(ref mut func) = callbacks.input else {
            return Some(1);
        };
        let ime_key = keystroke.ime_key.clone();
        let event = KeyDownEvent {
            keystroke,
            is_held: lparam.0 & (0x1 << 30) > 0,
        };

        let dispatch_event_result = func(PlatformInput::KeyDown(event));
        if dispatch_event_result.default_prevented || !dispatch_event_result.propagate {
            self.invalidate_client_area();
            return Some(0);
        }
        drop(callbacks);
        let Some(ime_char) = ime_key else {
            return Some(1);
        };
        let Some(mut input_handler) = self.input_handler.take() else {
            return Some(1);
        };
        input_handler.replace_text_in_range(None, &ime_char);
        self.input_handler.set(Some(input_handler));
        self.invalidate_client_area();
        Some(0)
    }

    fn handle_mouse_down_msg(&self, button: MouseButton, lparam: LPARAM) -> Option<isize> {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.input.as_mut() {
            let x = lparam.signed_loword() as f32;
            let y = lparam.signed_hiword() as f32;
            let physical_point = point(DevicePixels(x as i32), DevicePixels(y as i32));
            let click_count = self.click_state.borrow_mut().update(button, physical_point);
            let scale_factor = self.scale_factor.get();
            let event = MouseDownEvent {
                button,
                position: logical_point(x, y, scale_factor),
                modifiers: self.current_modifiers(),
                click_count,
                first_mouse: false,
            };
            if callback(PlatformInput::MouseDown(event)).default_prevented {
                return Some(0);
            }
        }
        Some(1)
    }

    fn handle_mouse_up_msg(&self, button: MouseButton, lparam: LPARAM) -> Option<isize> {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.input.as_mut() {
            let x = lparam.signed_loword() as f32;
            let y = lparam.signed_hiword() as f32;
            let scale_factor = self.scale_factor.get();
            let event = MouseUpEvent {
                button,
                position: logical_point(x, y, scale_factor),
                modifiers: self.current_modifiers(),
                click_count: 1,
            };
            if callback(PlatformInput::MouseUp(event)).default_prevented {
                return Some(0);
            }
        }
        Some(1)
    }

    fn handle_xbutton_msg(
        &self,
        wparam: WPARAM,
        lparam: LPARAM,
        handler: impl Fn(&Self, MouseButton, LPARAM) -> Option<isize>,
    ) -> Option<isize> {
        let nav_dir = match wparam.hiword() {
            XBUTTON1 => NavigationDirection::Back,
            XBUTTON2 => NavigationDirection::Forward,
            _ => return Some(1),
        };
        handler(self, MouseButton::Navigate(nav_dir), lparam)
    }

    fn handle_mouse_wheel_msg(&self, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.input.as_mut() {
            let wheel_distance = (wparam.signed_hiword() as f32 / WHEEL_DELTA as f32)
                * self.platform_inner.settings.borrow().wheel_scroll_lines as f32;
            let mut cursor_point = POINT {
                x: lparam.signed_loword().into(),
                y: lparam.signed_hiword().into(),
            };
            unsafe { ScreenToClient(self.hwnd, &mut cursor_point) };
            let scale_factor = self.scale_factor.get();
            let event = crate::ScrollWheelEvent {
                position: logical_point(cursor_point.x as f32, cursor_point.y as f32, scale_factor),
                delta: ScrollDelta::Lines(Point {
                    x: 0.0,
                    y: wheel_distance,
                }),
                modifiers: self.current_modifiers(),
                touch_phase: TouchPhase::Moved,
            };
            callback(PlatformInput::ScrollWheel(event));
            return Some(0);
        }
        Some(1)
    }

    fn handle_mouse_horizontal_wheel_msg(&self, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.input.as_mut() {
            let wheel_distance = (wparam.signed_hiword() as f32 / WHEEL_DELTA as f32)
                * self.platform_inner.settings.borrow().wheel_scroll_chars as f32;
            let mut cursor_point = POINT {
                x: lparam.signed_loword().into(),
                y: lparam.signed_hiword().into(),
            };
            unsafe { ScreenToClient(self.hwnd, &mut cursor_point) };
            let scale_factor = self.scale_factor.get();
            let event = crate::ScrollWheelEvent {
                position: logical_point(cursor_point.x as f32, cursor_point.y as f32, scale_factor),
                delta: ScrollDelta::Lines(Point {
                    x: wheel_distance,
                    y: 0.0,
                }),
                modifiers: self.current_modifiers(),
                touch_phase: TouchPhase::Moved,
            };
            if callback(PlatformInput::ScrollWheel(event)).default_prevented {
                return Some(0);
            }
        }
        Some(1)
    }

    fn handle_ime_position(&self) -> Option<isize> {
        unsafe {
            let ctx = ImmGetContext(self.hwnd);
            let Some(mut input_handler) = self.input_handler.take() else {
                return Some(1);
            };
            // we are composing, this should never fail
            let caret_range = input_handler.selected_text_range().unwrap();
            let caret_position = input_handler.bounds_for_range(caret_range).unwrap();
            self.input_handler.set(Some(input_handler));
            let scale_factor = self.scale_factor.get();
            let config = CANDIDATEFORM {
                dwStyle: CFS_CANDIDATEPOS,
                // logical to physical
                ptCurrentPos: POINT {
                    x: (caret_position.origin.x.0 * scale_factor) as i32,
                    y: (caret_position.origin.y.0 * scale_factor) as i32
                        + ((caret_position.size.height.0 * scale_factor) as i32 / 2),
                },
                ..Default::default()
            };
            ImmSetCandidateWindow(ctx, &config as _);
            ImmReleaseContext(self.hwnd, ctx);
            Some(0)
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

    fn retrieve_composition_cursor_position(&self) -> usize {
        unsafe {
            let ctx = ImmGetContext(self.hwnd);
            let ret = ImmGetCompositionStringW(ctx, GCS_CURSORPOS, None, 0);
            ImmReleaseContext(self.hwnd, ctx);
            ret as usize
        }
    }

    fn handle_ime_composition(&self, lparam: LPARAM) -> Option<isize> {
        if lparam.0 as u32 & GCS_COMPSTR.0 > 0 {
            let Some((string, string_len)) = self.parse_ime_compostion_string() else {
                return None;
            };
            let Some(mut input_handler) = self.input_handler.take() else {
                return None;
            };
            input_handler.replace_and_mark_text_in_range(
                None,
                string.as_str(),
                Some(0..string_len),
            );
            self.input_handler.set(Some(input_handler));
            *self.last_ime_input.borrow_mut() = Some(string);
        }
        if lparam.0 as u32 & GCS_CURSORPOS.0 > 0 {
            let Some(ref comp_string) = *self.last_ime_input.borrow() else {
                return None;
            };
            let caret_pos = self.retrieve_composition_cursor_position();
            let Some(mut input_handler) = self.input_handler.take() else {
                return None;
            };
            input_handler.replace_and_mark_text_in_range(None, comp_string, Some(0..caret_pos));
            self.input_handler.set(Some(input_handler));
        }
        // currently, we don't care other stuff
        None
    }

    fn parse_ime_char(&self, wparam: WPARAM) -> Option<String> {
        let src = [wparam.0 as u16];
        let Ok(first_char) = char::decode_utf16(src).collect::<Vec<_>>()[0] else {
            return None;
        };
        Some(first_char.to_string())
    }

    fn handle_ime_char(&self, wparam: WPARAM) -> Option<isize> {
        let Some(ime_char) = self.parse_ime_char(wparam) else {
            return Some(1);
        };
        let Some(mut input_handler) = self.input_handler.take() else {
            return Some(1);
        };
        input_handler.replace_text_in_range(None, &ime_char);
        self.input_handler.set(Some(input_handler));
        *self.last_ime_input.borrow_mut() = None;
        self.invalidate_client_area();
        Some(0)
    }

    fn handle_drag_drop(&self, input: PlatformInput) {
        let mut callbacks = self.callbacks.borrow_mut();
        let Some(ref mut func) = callbacks.input else {
            return;
        };
        func(input);
    }

    /// SEE: https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize
    fn handle_calc_client_size(&self, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        if !self.hide_title_bar || self.is_fullscreen() {
            return None;
        }

        if wparam.0 == 0 {
            return None;
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

        Some(0)
    }

    fn handle_activate_msg(&self, wparam: WPARAM) -> Option<isize> {
        if self.hide_title_bar {
            if let Some(titlebar_rect) = self.get_titlebar_rect().log_err() {
                unsafe { InvalidateRect(self.hwnd, Some(&titlebar_rect), FALSE) };
            }
        }
        let activated = wparam.loword() > 0;
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(mut cb) = callbacks.active_status_change.as_mut() {
            cb(activated);
        }
        None
    }

    fn handle_create_msg(&self, _lparam: LPARAM) -> Option<isize> {
        let mut size_rect = RECT::default();
        unsafe { GetWindowRect(self.hwnd, &mut size_rect).log_err() };

        let width = size_rect.right - size_rect.left;
        let height = size_rect.bottom - size_rect.top;

        self.physical_size.set(Size {
            width: DevicePixels(width),
            height: DevicePixels(height),
        });

        if self.hide_title_bar {
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
        }

        Some(0)
    }

    fn handle_dpi_changed_msg(&self, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        let new_dpi = wparam.loword() as f32;
        let scale_factor = new_dpi / USER_DEFAULT_SCREEN_DPI as f32;
        self.scale_factor.set(scale_factor);
        let rect = unsafe { &*(lparam.0 as *const RECT) };
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;
        // this will emit `WM_SIZE` and `WM_MOVE` right here
        // even before this function returns
        // the new size is handled in `WM_SIZE`
        unsafe {
            SetWindowPos(
                self.hwnd,
                None,
                rect.left,
                rect.top,
                width,
                height,
                SWP_NOZORDER | SWP_NOACTIVATE,
            )
            .context("unable to set window position after dpi has changed")
            .log_err();
        }
        self.invalidate_client_area();
        Some(0)
    }

    fn handle_hit_test_msg(&self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        if !self.hide_title_bar {
            return None;
        }

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
            return Some(hit.0);
        }

        if self.is_fullscreen() {
            return Some(HTCLIENT as _);
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
            return Some(HTTOP as _);
        }

        let titlebar_rect = self.get_titlebar_rect();
        if let Ok(titlebar_rect) = titlebar_rect {
            if cursor_point.y < titlebar_rect.bottom {
                let caption_btn_width =
                    (self.caption_button_width().0 * self.scale_factor.get()) as i32;
                if cursor_point.x >= titlebar_rect.right - caption_btn_width {
                    return Some(HTCLOSE as _);
                } else if cursor_point.x >= titlebar_rect.right - caption_btn_width * 2 {
                    return Some(HTMAXBUTTON as _);
                } else if cursor_point.x >= titlebar_rect.right - caption_btn_width * 3 {
                    return Some(HTMINBUTTON as _);
                }

                return Some(HTCAPTION as _);
            }
        }

        Some(HTCLIENT as _)
    }

    fn handle_nc_mouse_move_msg(&self, lparam: LPARAM) -> Option<isize> {
        if !self.hide_title_bar {
            return None;
        }

        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.input.as_mut() {
            let mut cursor_point = POINT {
                x: lparam.signed_loword().into(),
                y: lparam.signed_hiword().into(),
            };
            unsafe { ScreenToClient(self.hwnd, &mut cursor_point) };
            let scale_factor = self.scale_factor.get();
            let event = MouseMoveEvent {
                position: logical_point(cursor_point.x as f32, cursor_point.y as f32, scale_factor),
                pressed_button: None,
                modifiers: self.current_modifiers(),
            };
            if callback(PlatformInput::MouseMove(event)).default_prevented {
                return Some(0);
            }
        }
        None
    }

    fn handle_nc_mouse_down_msg(
        &self,
        button: MouseButton,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<isize> {
        if !self.hide_title_bar {
            return None;
        }

        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.input.as_mut() {
            let mut cursor_point = POINT {
                x: lparam.signed_loword().into(),
                y: lparam.signed_hiword().into(),
            };
            unsafe { ScreenToClient(self.hwnd, &mut cursor_point) };
            let physical_point = point(DevicePixels(cursor_point.x), DevicePixels(cursor_point.y));
            let click_count = self.click_state.borrow_mut().update(button, physical_point);
            let scale_factor = self.scale_factor.get();
            let event = MouseDownEvent {
                button,
                position: logical_point(cursor_point.x as f32, cursor_point.y as f32, scale_factor),
                modifiers: self.current_modifiers(),
                click_count,
                first_mouse: false,
            };
            if callback(PlatformInput::MouseDown(event)).default_prevented {
                return Some(0);
            }
        }

        // Since these are handled in handle_nc_mouse_up_msg we must prevent the default window proc
        matches!(wparam.0 as u32, HTMINBUTTON | HTMAXBUTTON | HTCLOSE).then_some(0)
    }

    fn handle_nc_mouse_up_msg(
        &self,
        button: MouseButton,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<isize> {
        if !self.hide_title_bar {
            return None;
        }

        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(callback) = callbacks.input.as_mut() {
            let mut cursor_point = POINT {
                x: lparam.signed_loword().into(),
                y: lparam.signed_hiword().into(),
            };
            unsafe { ScreenToClient(self.hwnd, &mut cursor_point) };
            let scale_factor = self.scale_factor.get();
            let event = MouseUpEvent {
                button,
                position: logical_point(cursor_point.x as f32, cursor_point.y as f32, scale_factor),
                modifiers: self.current_modifiers(),
                click_count: 1,
            };
            if callback(PlatformInput::MouseUp(event)).default_prevented {
                return Some(0);
            }
        }
        drop(callbacks);

        if button == MouseButton::Left {
            match wparam.0 as u32 {
                HTMINBUTTON => unsafe {
                    ShowWindowAsync(self.hwnd, SW_MINIMIZE);
                },
                HTMAXBUTTON => unsafe {
                    if self.is_maximized() {
                        ShowWindowAsync(self.hwnd, SW_NORMAL);
                    } else {
                        ShowWindowAsync(self.hwnd, SW_MAXIMIZE);
                    }
                },
                HTCLOSE => unsafe {
                    PostMessageW(self.hwnd, WM_CLOSE, WPARAM::default(), LPARAM::default())
                        .log_err();
                },
                _ => return None,
            };
            return Some(0);
        }

        None
    }

    fn handle_set_cursor(&self, lparam: LPARAM) -> Option<isize> {
        if matches!(
            lparam.loword() as u32,
            HTLEFT
                | HTRIGHT
                | HTTOP
                | HTTOPLEFT
                | HTTOPRIGHT
                | HTBOTTOM
                | HTBOTTOMLEFT
                | HTBOTTOMRIGHT
        ) {
            return None;
        }
        unsafe { SetCursor(self.platform_inner.current_cursor.get()) };
        Some(1)
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
}

struct WindowCreateContext {
    inner: Option<Rc<WindowsWindowInner>>,
    platform_inner: Rc<WindowsPlatformInner>,
    handle: AnyWindowHandle,
    hide_title_bar: bool,
    display: Rc<WindowsDisplay>,
}

impl WindowsWindow {
    pub(crate) fn new(
        platform_inner: Rc<WindowsPlatformInner>,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> Self {
        let classname = register_wnd_class(platform_inner.icon);
        let hide_title_bar = options
            .titlebar
            .as_ref()
            .map(|titlebar| titlebar.appears_transparent)
            .unwrap_or(false);
        let windowname = HSTRING::from(
            options
                .titlebar
                .as_ref()
                .and_then(|titlebar| titlebar.title.as_ref())
                .map(|title| title.as_ref())
                .unwrap_or(""),
        );
        let dwstyle = WS_THICKFRAME | WS_SYSMENU | WS_MAXIMIZEBOX | WS_MINIMIZEBOX;
        let x = options.bounds.origin.x.0;
        let y = options.bounds.origin.y.0;
        let nwidth = options.bounds.size.width.0;
        let nheight = options.bounds.size.height.0;
        let hwndparent = HWND::default();
        let hmenu = HMENU::default();
        let hinstance = HINSTANCE::default();
        let mut context = WindowCreateContext {
            inner: None,
            platform_inner: platform_inner.clone(),
            handle,
            hide_title_bar,
            // todo(windows) move window to target monitor
            // options.display_id
            display: Rc::new(WindowsDisplay::primary_monitor().unwrap()),
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
        let wnd = Self {
            inner: context.inner.unwrap(),
            drag_drop_handler,
        };
        platform_inner
            .raw_window_handles
            .write()
            .push(wnd.inner.hwnd);

        unsafe { ShowWindow(wnd.inner.hwnd, SW_SHOW) };
        wnd
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
    fn bounds(&self) -> Bounds<DevicePixels> {
        Bounds {
            origin: self.inner.origin.get(),
            size: self.inner.physical_size.get(),
        }
    }

    fn is_maximized(&self) -> bool {
        self.inner.is_maximized()
    }

    fn is_minimized(&self) -> bool {
        self.inner.is_minimized()
    }

    /// get the logical size of the app's drawable area.
    ///
    /// Currently, GPUI uses logical size of the app to handle mouse interactions (such as
    /// whether the mouse collides with other elements of GPUI).
    fn content_size(&self) -> Size<Pixels> {
        logical_size(
            self.inner.physical_size.get(),
            self.inner.scale_factor.get(),
        )
    }

    fn scale_factor(&self) -> f32 {
        self.inner.scale_factor.get()
    }

    // todo(windows)
    fn appearance(&self) -> WindowAppearance {
        WindowAppearance::Dark
    }

    fn display(&self) -> Rc<dyn PlatformDisplay> {
        self.inner.display.borrow().clone()
    }

    fn mouse_position(&self) -> Point<Pixels> {
        let point = unsafe {
            let mut point: POINT = std::mem::zeroed();
            GetCursorPos(&mut point)
                .context("unable to get cursor position")
                .log_err();
            ScreenToClient(self.inner.hwnd, &mut point);
            point
        };
        logical_point(
            point.x as f32,
            point.y as f32,
            self.inner.scale_factor.get(),
        )
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
        unsafe { SetActiveWindow(self.inner.hwnd) };
        unsafe { SetFocus(self.inner.hwnd) };
        unsafe { SetForegroundWindow(self.inner.hwnd) };
    }

    fn is_active(&self) -> bool {
        self.inner.hwnd == unsafe { GetActiveWindow() }
    }

    // todo(windows)
    fn set_title(&mut self, title: &str) {
        unsafe { SetWindowTextW(self.inner.hwnd, &HSTRING::from(title)) }
            .inspect_err(|e| log::error!("Set title failed: {e}"))
            .ok();
    }

    fn set_background_appearance(&mut self, _background_appearance: WindowBackgroundAppearance) {
        // todo(windows)
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

    fn toggle_fullscreen(&self) {
        self.inner
            .platform_inner
            .foreground_executor
            .spawn(self.inner.clone().toggle_fullscreen())
            .detach();
    }

    fn is_fullscreen(&self) -> bool {
        self.inner.is_fullscreen()
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

    fn get_raw_handle(&self) -> HWND {
        self.inner.hwnd
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

#[derive(Debug)]
struct ClickState {
    button: MouseButton,
    last_click: Instant,
    last_position: Point<DevicePixels>,
    current_count: usize,
}

impl ClickState {
    pub fn new() -> Self {
        ClickState {
            button: MouseButton::Left,
            last_click: Instant::now(),
            last_position: Point::default(),
            current_count: 0,
        }
    }

    /// update self and return the needed click count
    pub fn update(&mut self, button: MouseButton, new_position: Point<DevicePixels>) -> usize {
        if self.button == button && self.is_double_click(new_position) {
            self.current_count += 1;
        } else {
            self.current_count = 1;
        }
        self.last_click = Instant::now();
        self.last_position = new_position;
        self.button = button;

        self.current_count
    }

    #[inline]
    fn is_double_click(&self, new_position: Point<DevicePixels>) -> bool {
        let diff = self.last_position - new_position;

        self.last_click.elapsed() < DOUBLE_CLICK_INTERVAL
            && diff.x.0.abs() <= DOUBLE_CLICK_SPATIAL_TOLERANCE
            && diff.y.0.abs() <= DOUBLE_CLICK_SPATIAL_TOLERANCE
    }
}

fn register_wnd_class(icon_handle: HICON) -> PCWSTR {
    const CLASS_NAME: PCWSTR = w!("Zed::Window");

    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let wc = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            hIcon: icon_handle,
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
            ctx.hide_title_bar,
            ctx.display.clone(),
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
        unsafe { drop(Box::from_raw(ptr)) };
    }
    r
}

pub(crate) fn try_get_window_inner(hwnd: HWND) -> Option<Rc<WindowsWindowInner>> {
    if hwnd == HWND(0) {
        return None;
    }

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

#[inline]
fn logical_size(physical_size: Size<DevicePixels>, scale_factor: f32) -> Size<Pixels> {
    Size {
        width: px(physical_size.width.0 as f32 / scale_factor),
        height: px(physical_size.height.0 as f32 / scale_factor),
    }
}

#[inline]
fn logical_point(x: f32, y: f32, scale_factor: f32) -> Point<Pixels> {
    Point {
        x: px(x / scale_factor),
        y: px(y / scale_factor),
    }
}

struct StyleAndBounds {
    style: WINDOW_STYLE,
    x: i32,
    y: i32,
    cx: i32,
    cy: i32,
}

// https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragqueryfilew
const DRAGDROP_GET_FILES_COUNT: u32 = 0xFFFFFFFF;
// https://learn.microsoft.com/en-us/windows/win32/controls/ttm-setdelaytime?redirectedfrom=MSDN
const DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(500);
// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics
const DOUBLE_CLICK_SPATIAL_TOLERANCE: i32 = 4;

#[cfg(test)]
mod tests {
    use super::ClickState;
    use crate::{point, DevicePixels, MouseButton};
    use std::time::Duration;

    #[test]
    fn test_double_click_interval() {
        let mut state = ClickState::new();
        assert_eq!(
            state.update(MouseButton::Left, point(DevicePixels(0), DevicePixels(0))),
            1
        );
        assert_eq!(
            state.update(MouseButton::Right, point(DevicePixels(0), DevicePixels(0))),
            1
        );
        assert_eq!(
            state.update(MouseButton::Left, point(DevicePixels(0), DevicePixels(0))),
            1
        );
        assert_eq!(
            state.update(MouseButton::Left, point(DevicePixels(0), DevicePixels(0))),
            2
        );
        state.last_click -= Duration::from_millis(700);
        assert_eq!(
            state.update(MouseButton::Left, point(DevicePixels(0), DevicePixels(0))),
            1
        );
    }

    #[test]
    fn test_double_click_spatial_tolerance() {
        let mut state = ClickState::new();
        assert_eq!(
            state.update(MouseButton::Left, point(DevicePixels(-3), DevicePixels(0))),
            1
        );
        assert_eq!(
            state.update(MouseButton::Left, point(DevicePixels(0), DevicePixels(3))),
            2
        );
        assert_eq!(
            state.update(MouseButton::Right, point(DevicePixels(3), DevicePixels(2))),
            1
        );
        assert_eq!(
            state.update(MouseButton::Right, point(DevicePixels(10), DevicePixels(0))),
            1
        );
    }
}
