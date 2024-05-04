#![deny(unsafe_op_in_unsafe_fn)]

use std::{
    iter::once,
    num::NonZeroIsize,
    path::PathBuf,
    rc::{Rc, Weak},
    str::FromStr,
    sync::{atomic::AtomicUsize, Arc, Once},
    time::{Duration, Instant},
};

use ::util::ResultExt;
use anyhow::Context;
use blade_graphics as gpu;
use futures::channel::oneshot::{self, Receiver};
use itertools::Itertools;
use parking_lot::{Mutex, RwLock, RwLockUpgradableReadGuard};
use raw_window_handle as rwh;
use smallvec::SmallVec;
use std::result::Result;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        System::{Com::*, LibraryLoader::*, Ole::*, SystemServices::*},
        UI::{
            Controls::*,
            HiDpi::*,
            Input::{Ime::*, KeyboardAndMouse::*},
            Shell::*,
            WindowsAndMessaging::*,
        },
    },
};

use crate::platform::blade::{BladeRenderer, BladeSurfaceConfig};
use crate::*;

pub(crate) struct WindowsWindowState {
    this: std::sync::Weak<RwLock<Self>>,
    hwnd: HWND,
    executor: ForegroundExecutor,
    origin: Point<DevicePixels>,
    physical_size: Size<DevicePixels>,
    scale_factor: f32,
    input_handler: Option<PlatformInputHandler>,
    renderer: BladeRenderer,
    callbacks: Callbacks,
    pub(crate) handle: AnyWindowHandle,
    hide_title_bar: bool,
    display: WindowsDisplay,
    click_state: ClickState,
    fullscreen: Option<StyleAndBounds>,
    cursor: HCURSOR,
    raw_window_handles: Arc<RwLock<SmallVec<[HWND; 4]>>>,
}

unsafe impl Send for WindowsWindowState {}

impl WindowsWindowState {
    fn new(
        hwnd: HWND,
        cs: &CREATESTRUCTW,
        executor: ForegroundExecutor,
        handle: AnyWindowHandle,
        hide_title_bar: bool,
        display: WindowsDisplay,
        transparent: bool,
        cursor: HCURSOR,
        raw_window_handles: Arc<RwLock<SmallVec<[HWND; 4]>>>,
    ) -> Arc<RwLock<Self>> {
        let monitor_dpi = unsafe { GetDpiForWindow(hwnd) } as f32;
        let origin = point(cs.x.into(), cs.y.into());
        let physical_size = size(cs.cx.into(), cs.cy.into());
        let scale_factor = monitor_dpi / USER_DEFAULT_SCREEN_DPI as f32;
        let input_handler = None;
        struct RawWindow {
            hwnd: isize,
        }
        impl rwh::HasWindowHandle for RawWindow {
            fn window_handle(&self) -> Result<rwh::WindowHandle<'_>, rwh::HandleError> {
                Ok(unsafe {
                    let hwnd = NonZeroIsize::new_unchecked(self.hwnd);
                    let mut handle = rwh::Win32WindowHandle::new(hwnd);
                    let hinstance = get_window_long(HWND(self.hwnd), GWLP_HINSTANCE);
                    handle.hinstance = NonZeroIsize::new(hinstance);
                    rwh::WindowHandle::borrow_raw(handle.into())
                })
            }
        }
        impl rwh::HasDisplayHandle for RawWindow {
            fn display_handle(&self) -> Result<rwh::DisplayHandle<'_>, rwh::HandleError> {
                let handle = rwh::WindowsDisplayHandle::new();
                Ok(unsafe { rwh::DisplayHandle::borrow_raw(handle.into()) })
            }
        }

        let raw = RawWindow { hwnd: hwnd.0 };
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
        let config = BladeSurfaceConfig {
            size: gpu::Extent::default(),
            transparent,
        };
        let renderer = BladeRenderer::new(gpu, config);
        let callbacks = Callbacks::default();
        let click_state = ClickState::new();
        let fullscreen = None;

        Arc::new_cyclic(|this| {
            RwLock::new(Self {
                this: this.clone(),
                hwnd,
                executor,
                origin,
                physical_size,
                scale_factor,
                input_handler,
                renderer,
                callbacks,
                handle,
                hide_title_bar,
                display,
                click_state,
                fullscreen,
                cursor,
                raw_window_handles,
            })
        })
    }

    fn is_maximized(&self) -> bool {
        !self.is_fullscreen() && unsafe { IsZoomed(self.hwnd) }.as_bool()
    }

    fn is_minimized(&self) -> bool {
        unsafe { IsIconic(self.hwnd) }.as_bool()
    }

    fn is_fullscreen(&self) -> bool {
        // let fullscreen = self.fullscreen.take();
        // let is_fullscreen = fullscreen.is_some();
        // self.fullscreen.insert(fullscreen);
        // is_fullscreen
        self.fullscreen.is_some()
    }

    pub fn bounds(&self) -> Bounds<DevicePixels> {
        Bounds {
            origin: self.origin,
            size: self.physical_size,
        }
    }

    /// get the logical size of the app's drawable area.
    ///
    /// Currently, GPUI uses logical size of the app to handle mouse interactions (such as
    /// whether the mouse collides with other elements of GPUI).
    pub fn content_size(&self) -> Size<Pixels> {
        logical_size(self.physical_size, self.scale_factor)
    }

    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
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
        rect.bottom = rect.top + ((height.0 * self.scale_factor).round() as i32);
        Ok(rect)
    }

    fn handle_msg(
        handle: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> LRESULT {
        let handled = match msg {
            WM_ACTIVATE => Self::handle_activate_msg(handle, wparam, state),
            WM_CREATE => Self::handle_create_msg(handle, state),
            WM_MOVE => Self::handle_move_msg(handle, lparam, state),
            WM_SIZE => Self::handle_size_msg(lparam, state),
            WM_ENTERSIZEMOVE | WM_ENTERMENULOOP => Self::handle_size_move_loop(handle),
            WM_EXITSIZEMOVE | WM_EXITMENULOOP => Self::handle_size_move_loop_exit(handle),
            WM_TIMER => Self::handle_timer_msg(handle, wparam, state),
            WM_NCCALCSIZE => Self::handle_calc_client_size(handle, wparam, lparam, state),
            WM_DPICHANGED => Self::handle_dpi_changed_msg(handle, wparam, lparam, state),
            WM_NCHITTEST => Self::handle_hit_test_msg(handle, msg, wparam, lparam, state),
            WM_PAINT => Self::handle_paint_msg(handle, state),
            WM_CLOSE => Self::handle_close_msg(state),
            WM_DESTROY => Self::handle_destroy_msg(state),
            WM_MOUSEMOVE => Self::handle_mouse_move_msg(lparam, wparam, state),
            WM_NCMOUSEMOVE => Self::handle_nc_mouse_move_msg(handle, lparam, state),
            WM_NCLBUTTONDOWN => {
                Self::handle_nc_mouse_down_msg(handle, MouseButton::Left, wparam, lparam, state)
            }
            WM_NCRBUTTONDOWN => {
                Self::handle_nc_mouse_down_msg(handle, MouseButton::Right, wparam, lparam, state)
            }
            WM_NCMBUTTONDOWN => {
                Self::handle_nc_mouse_down_msg(handle, MouseButton::Middle, wparam, lparam, state)
            }
            WM_NCLBUTTONUP => {
                Self::handle_nc_mouse_up_msg(handle, MouseButton::Left, wparam, lparam, state)
            }
            WM_NCRBUTTONUP => {
                Self::handle_nc_mouse_up_msg(handle, MouseButton::Right, wparam, lparam, state)
            }
            WM_NCMBUTTONUP => {
                Self::handle_nc_mouse_up_msg(handle, MouseButton::Middle, wparam, lparam, state)
            }
            WM_LBUTTONDOWN => Self::handle_mouse_down_msg(MouseButton::Left, lparam, state),
            WM_RBUTTONDOWN => Self::handle_mouse_down_msg(MouseButton::Right, lparam, state),
            WM_MBUTTONDOWN => Self::handle_mouse_down_msg(MouseButton::Middle, lparam, state),
            WM_XBUTTONDOWN => {
                Self::handle_xbutton_msg(wparam, lparam, Self::handle_mouse_down_msg, state)
            }
            WM_LBUTTONUP => Self::handle_mouse_up_msg(MouseButton::Left, lparam, state),
            WM_RBUTTONUP => Self::handle_mouse_up_msg(MouseButton::Right, lparam, state),
            WM_MBUTTONUP => Self::handle_mouse_up_msg(MouseButton::Middle, lparam, state),
            WM_XBUTTONUP => {
                Self::handle_xbutton_msg(wparam, lparam, Self::handle_mouse_up_msg, state)
            }
            WM_MOUSEWHEEL => Self::handle_mouse_wheel_msg(handle, wparam, lparam, state),
            WM_MOUSEHWHEEL => {
                Self::handle_mouse_horizontal_wheel_msg(handle, wparam, lparam, state)
            }
            WM_SYSKEYDOWN => Self::handle_syskeydown_msg(handle, wparam, lparam, state),
            WM_SYSKEYUP => Self::handle_syskeyup_msg(handle, wparam, state),
            WM_KEYDOWN => Self::handle_keydown_msg(handle, wparam, lparam, state),
            WM_KEYUP => Self::handle_keyup_msg(handle, wparam, state),
            WM_CHAR => Self::handle_char_msg(handle, wparam, lparam, state),
            WM_IME_STARTCOMPOSITION => Self::handle_ime_position(handle, state),
            WM_IME_COMPOSITION => Self::handle_ime_composition(handle, lparam, state),
            WM_SETCURSOR => Self::handle_set_cursor(lparam, state),
            1025 => Self::handle_cursor_changed(lparam, state),
            _ => None,
        };
        if let Some(n) = handled {
            LRESULT(n)
        } else {
            unsafe { DefWindowProcW(handle, msg, wparam, lparam) }
        }
    }

    fn handle_move_msg(
        handle: HWND,
        lparam: LPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let x = lparam.signed_loword() as i32;
        let y = lparam.signed_hiword() as i32;
        let mut lock = state.as_ref().write();
        lock.origin = point(x.into(), y.into());
        let size = lock.physical_size;
        let center_x = x + size.width.0 / 2;
        let center_y = y + size.height.0 / 2;
        let monitor_bounds = lock.display.bounds();
        if center_x < monitor_bounds.left().0
            || center_x > monitor_bounds.right().0
            || center_y < monitor_bounds.top().0
            || center_y > monitor_bounds.bottom().0
        {
            // center of the window may have moved to another monitor
            let monitor = unsafe { MonitorFromWindow(handle, MONITOR_DEFAULTTONULL) };
            if !monitor.is_invalid() && lock.display.handle != monitor {
                // we will get the same monitor if we only have one
                lock.display = WindowsDisplay::new_with_handle(monitor);
            }
        }
        if let Some(mut callback) = lock.callbacks.moved.take() {
            drop(lock);
            callback();
            let mut lock = state.write();
            lock.callbacks.moved = Some(callback);
        }
        Some(0)
    }

    fn handle_size_msg(lparam: LPARAM, state: Arc<RwLock<WindowsWindowState>>) -> Option<isize> {
        let width = lparam.loword().max(1) as i32;
        let height = lparam.hiword().max(1) as i32;
        let mut lock = state.as_ref().write();
        let scale_factor = lock.scale_factor;
        let new_physical_size = size(width.into(), height.into());
        lock.physical_size = new_physical_size;
        lock.renderer.update_drawable_size(Size {
            width: width as f64,
            height: height as f64,
        });
        if let Some(mut callback) = lock.callbacks.resize.take() {
            drop(lock);
            let logical_size = logical_size(new_physical_size, scale_factor);
            callback(logical_size, scale_factor);
            let mut lock = state.write();
            lock.callbacks.resize = Some(callback);
        }
        Some(0)
    }

    fn handle_size_move_loop(handle: HWND) -> Option<isize> {
        unsafe {
            let ret = SetTimer(handle, SIZE_MOVE_LOOP_TIMER_ID, USER_TIMER_MINIMUM, None);
            if ret == 0 {
                log::error!(
                    "unable to create timer: {}",
                    std::io::Error::last_os_error()
                );
            }
        }
        None
    }

    fn handle_size_move_loop_exit(handle: HWND) -> Option<isize> {
        unsafe {
            KillTimer(handle, SIZE_MOVE_LOOP_TIMER_ID).log_err();
        }
        None
    }

    fn handle_timer_msg(
        handle: HWND,
        wparam: WPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        if wparam.0 == SIZE_MOVE_LOOP_TIMER_ID {
            // TODO:
            // self.platform_inner.run_foreground_tasks();
            Self::handle_paint_msg(handle, state)
        } else {
            None
        }
    }

    fn handle_paint_msg(handle: HWND, state: Arc<RwLock<WindowsWindowState>>) -> Option<isize> {
        let mut paint_struct = PAINTSTRUCT::default();
        let _hdc = unsafe { BeginPaint(handle, &mut paint_struct) };
        let mut lock = state.as_ref().write();
        if let Some(mut request_frame) = lock.callbacks.request_frame.take() {
            drop(lock);
            request_frame();
            let mut lock = state.write();
            lock.callbacks.request_frame = Some(request_frame);
        }
        unsafe { EndPaint(handle, &paint_struct) };
        Some(0)
    }

    fn handle_close_msg(state: Arc<RwLock<WindowsWindowState>>) -> Option<isize> {
        let mut lock = state.as_ref().write();
        if let Some(mut callback) = lock.callbacks.should_close.take() {
            drop(lock);
            if callback() {
                return Some(0);
            }
            let mut lock = state.write();
            lock.callbacks.should_close = Some(callback);
        }
        None
    }

    fn handle_destroy_msg(state: Arc<RwLock<WindowsWindowState>>) -> Option<isize> {
        let mut lock = state.as_ref().write();
        if let Some(callback) = lock.callbacks.close.take() {
            drop(lock);
            callback();
        }
        let lock = state.as_ref().read();
        let mut handle_vec_lock = lock.raw_window_handles.write();
        let window_handle = lock.hwnd;
        let executor = lock.executor.clone();
        let index = handle_vec_lock
            .iter()
            .position(|handle| *handle == window_handle)
            .unwrap();
        handle_vec_lock.remove(index);
        if handle_vec_lock.is_empty() {
            drop(handle_vec_lock);
            drop(lock);
            executor
                .spawn(async {
                    unsafe { PostQuitMessage(0) };
                })
                .detach();
        }
        Some(1)
    }

    fn handle_mouse_move_msg(
        lparam: LPARAM,
        wparam: WPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let mut lock = state.as_ref().write();
        if let Some(mut callback) = lock.callbacks.input.take() {
            let scale_factor = lock.scale_factor;
            drop(lock);
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
            let event = MouseMoveEvent {
                position: logical_point(x, y, scale_factor),
                pressed_button,
                modifiers: current_modifiers(),
            };
            let result = if callback(PlatformInput::MouseMove(event)).default_prevented {
                Some(0)
            } else {
                Some(1)
            };
            let mut lock = state.write();
            lock.callbacks.input = Some(callback);
            return result;
        }
        Some(1)
    }

    fn handle_syskeydown_msg(
        handle: HWND,
        wparam: WPARAM,
        lparam: LPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        // we need to call `DefWindowProcW`, or we will lose the system-wide `Alt+F4`, `Alt+{other keys}`
        // shortcuts.
        let Some(keystroke) = parse_syskeydown_msg_keystroke(wparam) else {
            return None;
        };
        let mut lock = state.as_ref().write();
        let Some(mut func) = lock.callbacks.input.take() else {
            return None;
        };
        drop(lock);
        let event = KeyDownEvent {
            keystroke,
            is_held: lparam.0 & (0x1 << 30) > 0,
        };
        let result = if func(PlatformInput::KeyDown(event)).default_prevented {
            invalidate_client_area(handle);
            Some(0)
        } else {
            None
        };
        let mut lock = state.write();
        lock.callbacks.input = Some(func);

        result
    }

    fn handle_syskeyup_msg(
        handle: HWND,
        wparam: WPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        // we need to call `DefWindowProcW`, or we will lose the system-wide `Alt+F4`, `Alt+{other keys}`
        // shortcuts.
        let Some(keystroke) = parse_syskeydown_msg_keystroke(wparam) else {
            return None;
        };
        let mut lock = state.as_ref().write();
        let Some(mut func) = lock.callbacks.input.take() else {
            return None;
        };
        drop(lock);
        let event = KeyUpEvent { keystroke };
        let result = if func(PlatformInput::KeyUp(event)).default_prevented {
            invalidate_client_area(handle);
            Some(0)
        } else {
            Some(1)
        };
        let mut lock = state.write();
        lock.callbacks.input = Some(func);

        result
    }

    fn handle_keydown_msg(
        handle: HWND,
        wparam: WPARAM,
        lparam: LPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let Some(keystroke) = parse_keydown_msg_keystroke(wparam) else {
            return Some(1);
        };
        let mut lock = state.as_ref().write();
        let Some(mut func) = lock.callbacks.input.take() else {
            return Some(1);
        };
        drop(lock);
        let event = KeyDownEvent {
            keystroke,
            is_held: lparam.0 & (0x1 << 30) > 0,
        };
        let result = if func(PlatformInput::KeyDown(event)).default_prevented {
            invalidate_client_area(handle);
            Some(0)
        } else {
            Some(1)
        };
        let mut lock = state.write();
        lock.callbacks.input = Some(func);

        result
    }

    fn handle_keyup_msg(
        handle: HWND,
        wparam: WPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let Some(keystroke) = parse_keydown_msg_keystroke(wparam) else {
            return Some(1);
        };
        let mut lock = state.as_ref().write();
        let Some(mut func) = lock.callbacks.input.take() else {
            return Some(1);
        };
        drop(lock);
        let event = KeyUpEvent { keystroke };
        let result = if func(PlatformInput::KeyUp(event)).default_prevented {
            invalidate_client_area(handle);
            Some(0)
        } else {
            Some(1)
        };
        let mut lock = state.write();
        lock.callbacks.input = Some(func);

        result
    }

    fn handle_char_msg(
        handle: HWND,
        wparam: WPARAM,
        lparam: LPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let Some(keystroke) = parse_char_msg_keystroke(wparam) else {
            return Some(1);
        };
        let mut lock = state.as_ref().write();
        let Some(ref mut func) = lock.callbacks.input else {
            return Some(1);
        };
        let ime_key = keystroke.ime_key.clone();
        let event = KeyDownEvent {
            keystroke,
            is_held: lparam.0 & (0x1 << 30) > 0,
        };

        let dispatch_event_result = func(PlatformInput::KeyDown(event));
        if dispatch_event_result.default_prevented || !dispatch_event_result.propagate {
            invalidate_client_area(handle);
            return Some(0);
        }
        let Some(ime_char) = ime_key else {
            return Some(1);
        };
        let Some(mut input_handler) = lock.input_handler.take() else {
            return Some(1);
        };
        drop(lock);
        input_handler.replace_text_in_range(None, &ime_char);
        invalidate_client_area(handle);
        let mut lock = state.write();
        lock.input_handler = Some(input_handler);

        Some(0)
    }

    fn handle_mouse_down_msg(
        button: MouseButton,
        lparam: LPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let mut lock = state.as_ref().write();
        if let Some(mut callback) = lock.callbacks.input.take() {
            let x = lparam.signed_loword() as f32;
            let y = lparam.signed_hiword() as f32;
            let physical_point = point(DevicePixels(x as i32), DevicePixels(y as i32));
            let click_count = lock.click_state.update(button, physical_point);
            let scale_factor = lock.scale_factor;
            drop(lock);

            let event = MouseDownEvent {
                button,
                position: logical_point(x, y, scale_factor),
                modifiers: current_modifiers(),
                click_count,
                first_mouse: false,
            };
            let result = if callback(PlatformInput::MouseDown(event)).default_prevented {
                Some(0)
            } else {
                Some(1)
            };
            let mut lock = state.write();
            lock.callbacks.input = Some(callback);

            result
        } else {
            Some(1)
        }
    }

    fn handle_mouse_up_msg(
        button: MouseButton,
        lparam: LPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let mut lock = state.as_ref().write();
        if let Some(mut callback) = lock.callbacks.input.take() {
            let x = lparam.signed_loword() as f32;
            let y = lparam.signed_hiword() as f32;
            let click_count = lock.click_state.current_count;
            let scale_factor = lock.scale_factor;
            drop(lock);

            let event = MouseUpEvent {
                button,
                position: logical_point(x, y, scale_factor),
                modifiers: current_modifiers(),
                click_count,
            };
            let result = if callback(PlatformInput::MouseUp(event)).default_prevented {
                Some(0)
            } else {
                Some(1)
            };
            let mut lock = state.write();
            lock.callbacks.input = Some(callback);

            result
        } else {
            Some(1)
        }
    }

    fn handle_xbutton_msg(
        wparam: WPARAM,
        lparam: LPARAM,
        handler: impl Fn(MouseButton, LPARAM, Arc<RwLock<WindowsWindowState>>) -> Option<isize>,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let nav_dir = match wparam.hiword() {
            XBUTTON1 => NavigationDirection::Back,
            XBUTTON2 => NavigationDirection::Forward,
            _ => return Some(1),
        };
        handler(MouseButton::Navigate(nav_dir), lparam, state)
    }

    fn handle_mouse_wheel_msg(
        handle: HWND,
        wparam: WPARAM,
        lparam: LPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let mut lock = state.as_ref().write();
        if let Some(mut callback) = lock.callbacks.input.take() {
            let scale_factor = lock.scale_factor;
            drop(lock);
            let wheel_distance = wparam.signed_hiword() as f32 / WHEEL_DELTA as f32;
            // let wheel_distance = (wparam.signed_hiword() as f32 / WHEEL_DELTA as f32)
            // * self.platform_inner.settings.borrow().wheel_scroll_lines as f32;
            let mut cursor_point = POINT {
                x: lparam.signed_loword().into(),
                y: lparam.signed_hiword().into(),
            };
            unsafe { ScreenToClient(handle, &mut cursor_point) };
            let event = ScrollWheelEvent {
                position: logical_point(cursor_point.x as f32, cursor_point.y as f32, scale_factor),
                delta: ScrollDelta::Lines(Point {
                    x: 0.0,
                    y: wheel_distance,
                }),
                modifiers: current_modifiers(),
                touch_phase: TouchPhase::Moved,
            };
            let result = if callback(PlatformInput::ScrollWheel(event)).default_prevented {
                Some(0)
            } else {
                Some(1)
            };
            let mut lock = state.write();
            lock.callbacks.input = Some(callback);

            result
        } else {
            Some(1)
        }
    }

    fn handle_mouse_horizontal_wheel_msg(
        handle: HWND,
        wparam: WPARAM,
        lparam: LPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let mut lock = state.as_ref().write();
        if let Some(mut callback) = lock.callbacks.input.take() {
            let scale_factor = lock.scale_factor;
            drop(lock);
            let wheel_distance = wparam.signed_hiword() as f32 / WHEEL_DELTA as f32;
            // let wheel_distance = (wparam.signed_hiword() as f32 / WHEEL_DELTA as f32)
            //     * self.platform_inner.settings.borrow().wheel_scroll_chars as f32;
            let mut cursor_point = POINT {
                x: lparam.signed_loword().into(),
                y: lparam.signed_hiword().into(),
            };
            unsafe { ScreenToClient(handle, &mut cursor_point) };
            let event = ScrollWheelEvent {
                position: logical_point(cursor_point.x as f32, cursor_point.y as f32, scale_factor),
                delta: ScrollDelta::Lines(Point {
                    x: wheel_distance,
                    y: 0.0,
                }),
                modifiers: current_modifiers(),
                touch_phase: TouchPhase::Moved,
            };
            let result = if callback(PlatformInput::ScrollWheel(event)).default_prevented {
                Some(0)
            } else {
                Some(1)
            };
            let mut lock = state.write();
            lock.callbacks.input = Some(callback);

            result
        } else {
            Some(1)
        }
    }

    fn handle_ime_position(handle: HWND, state: Arc<RwLock<WindowsWindowState>>) -> Option<isize> {
        unsafe {
            let mut lock = state.as_ref().write();
            let ctx = ImmGetContext(handle);
            let Some(mut input_handler) = lock.input_handler.take() else {
                return Some(1);
            };
            let caret_range = input_handler.selected_text_range().unwrap_or_default();
            let caret_position = input_handler.bounds_for_range(caret_range).unwrap();
            lock.input_handler = Some(input_handler);
            let scale_factor = lock.scale_factor;
            drop(lock);
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
            ImmReleaseContext(handle, ctx);
            Some(0)
        }
    }

    fn handle_ime_composition(
        handle: HWND,
        lparam: LPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let mut ime_input = None;
        let mut lock = state.as_ref().write();
        if lparam.0 as u32 & GCS_COMPSTR.0 > 0 {
            let Some((string, string_len)) = parse_ime_compostion_string(handle) else {
                return None;
            };
            let Some(mut input_handler) = lock.input_handler.take() else {
                return None;
            };
            input_handler.replace_and_mark_text_in_range(
                None,
                string.as_str(),
                Some(0..string_len),
            );
            lock.input_handler = Some(input_handler);
            ime_input = Some(string);
        }
        if lparam.0 as u32 & GCS_CURSORPOS.0 > 0 {
            let Some(ref comp_string) = ime_input else {
                return None;
            };
            let caret_pos = retrieve_composition_cursor_position(handle);
            let Some(mut input_handler) = lock.input_handler.take() else {
                return None;
            };
            input_handler.replace_and_mark_text_in_range(None, comp_string, Some(0..caret_pos));
            lock.input_handler = Some(input_handler);
        }
        if lparam.0 as u32 & GCS_RESULTSTR.0 > 0 {
            let Some(comp_result) = parse_ime_compostion_result(handle) else {
                return None;
            };
            let Some(mut input_handler) = lock.input_handler.take() else {
                return Some(1);
            };
            input_handler.replace_text_in_range(None, &comp_result);
            lock.input_handler = Some(input_handler);
            invalidate_client_area(handle);
            return Some(0);
        }
        // currently, we don't care other stuff
        None
    }

    fn handle_drag_drop(&mut self, input: PlatformInput) {
        let Some(ref mut func) = self.callbacks.input else {
            return;
        };
        func(input);
    }

    /// SEE: https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize
    fn handle_calc_client_size(
        handle: HWND,
        wparam: WPARAM,
        lparam: LPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let lock = state.as_ref().upgradable_read();
        if !lock.hide_title_bar || lock.is_fullscreen() {
            return None;
        }
        drop(lock);

        if wparam.0 == 0 {
            return None;
        }

        let dpi = unsafe { GetDpiForWindow(handle) };

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

    fn handle_activate_msg(
        handle: HWND,
        wparam: WPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let activated = wparam.loword() > 0;
        let lock = state.as_ref().read();
        let executor = lock.executor.clone();
        drop(lock);
        executor
            .spawn(async move {
                let mut lock = state.as_ref().write();
                if lock.hide_title_bar {
                    if let Some(titlebar_rect) = lock.get_titlebar_rect().log_err() {
                        unsafe { InvalidateRect(handle, Some(&titlebar_rect), FALSE) };
                    }
                }
                if let Some(mut cb) = lock.callbacks.active_status_change.take() {
                    drop(lock);
                    cb(activated);
                    let mut lock = state.write();
                    lock.callbacks.active_status_change = Some(cb);
                }
            })
            .detach();

        None
    }

    fn handle_create_msg(handle: HWND, state: Arc<RwLock<WindowsWindowState>>) -> Option<isize> {
        let mut size_rect = RECT::default();
        unsafe { GetWindowRect(handle, &mut size_rect).log_err() };

        let width = size_rect.right - size_rect.left;
        let height = size_rect.bottom - size_rect.top;

        let lock = state.as_ref().read();
        if lock.hide_title_bar {
            drop(lock);
            // Inform the application of the frame change to force redrawing with the new
            // client area that is extended into the title bar
            // let executor = lock.executor.clone();
            // executor
            //     .spawn(async move {
            //         unsafe {
            //             SetWindowPos(
            //                 handle,
            //                 None,
            //                 size_rect.left,
            //                 size_rect.top,
            //                 width,
            //                 height,
            //                 SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE,
            //             )
            //             .log_err()
            //         }
            //     })
            //     .detach();
            unsafe {
                SetWindowPos(
                    handle,
                    None,
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

    fn handle_dpi_changed_msg(
        handle: HWND,
        wparam: WPARAM,
        lparam: LPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let new_dpi = wparam.loword() as f32;
        let mut lock = state.as_ref().write();
        lock.scale_factor = new_dpi / USER_DEFAULT_SCREEN_DPI as f32;
        drop(lock);

        let rect = unsafe { &*(lparam.0 as *const RECT) };
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;
        // this will emit `WM_SIZE` and `WM_MOVE` right here
        // even before this function returns
        // the new size is handled in `WM_SIZE`
        unsafe {
            SetWindowPos(
                handle,
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
        invalidate_client_area(handle);

        Some(0)
    }

    fn handle_hit_test_msg(
        handle: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let lock = state.as_ref().read();
        if !lock.hide_title_bar {
            return None;
        }

        // default handler for resize areas
        let hit = unsafe { DefWindowProcW(handle, msg, wparam, lparam) };
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

        if lock.is_fullscreen() {
            return Some(HTCLIENT as _);
        }

        let dpi = unsafe { GetDpiForWindow(handle) };
        let frame_y = unsafe { GetSystemMetricsForDpi(SM_CYFRAME, dpi) };
        let padding = unsafe { GetSystemMetricsForDpi(SM_CXPADDEDBORDER, dpi) };

        let mut cursor_point = POINT {
            x: lparam.signed_loword().into(),
            y: lparam.signed_hiword().into(),
        };
        unsafe { ScreenToClient(handle, &mut cursor_point) };
        if cursor_point.y > 0 && cursor_point.y < frame_y + padding {
            return Some(HTTOP as _);
        }

        let titlebar_rect = lock.get_titlebar_rect();
        if let Ok(titlebar_rect) = titlebar_rect {
            if cursor_point.y < titlebar_rect.bottom {
                let caption_btn_width = (lock.caption_button_width().0 * lock.scale_factor) as i32;
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

    fn handle_nc_mouse_move_msg(
        handle: HWND,
        lparam: LPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let lock = state.as_ref().upgradable_read();
        if !lock.hide_title_bar {
            return None;
        }

        let mut lock = RwLockUpgradableReadGuard::upgrade(lock);
        if let Some(mut callback) = lock.callbacks.input.take() {
            let scale_factor = lock.scale_factor;
            drop(lock);
            let mut cursor_point = POINT {
                x: lparam.signed_loword().into(),
                y: lparam.signed_hiword().into(),
            };
            unsafe { ScreenToClient(handle, &mut cursor_point) };
            let event = MouseMoveEvent {
                position: logical_point(cursor_point.x as f32, cursor_point.y as f32, scale_factor),
                pressed_button: None,
                modifiers: current_modifiers(),
            };
            let result = if callback(PlatformInput::MouseMove(event)).default_prevented {
                Some(0)
            } else {
                Some(1)
            };
            let mut lock = state.write();
            lock.callbacks.input = Some(callback);

            result
        } else {
            None
        }
    }

    fn handle_nc_mouse_down_msg(
        handle: HWND,
        button: MouseButton,
        wparam: WPARAM,
        lparam: LPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let lock = state.as_ref().upgradable_read();
        if !lock.hide_title_bar {
            return None;
        }

        let mut lock = RwLockUpgradableReadGuard::upgrade(lock);
        let result = if let Some(mut callback) = lock.callbacks.input.take() {
            let scale_factor = lock.scale_factor;
            let mut cursor_point = POINT {
                x: lparam.signed_loword().into(),
                y: lparam.signed_hiword().into(),
            };
            unsafe { ScreenToClient(handle, &mut cursor_point) };
            let physical_point = point(DevicePixels(cursor_point.x), DevicePixels(cursor_point.y));
            let click_count = lock.click_state.update(button, physical_point);
            drop(lock);
            let event = MouseDownEvent {
                button,
                position: logical_point(cursor_point.x as f32, cursor_point.y as f32, scale_factor),
                modifiers: current_modifiers(),
                click_count,
                first_mouse: false,
            };
            let result = if callback(PlatformInput::MouseDown(event)).default_prevented {
                Some(0)
            } else {
                None
            };
            let mut lock = state.write();
            lock.callbacks.input = Some(callback);

            result
        } else {
            None
        };

        // Since these are handled in handle_nc_mouse_up_msg we must prevent the default window proc
        result
            .or_else(|| matches!(wparam.0 as u32, HTMINBUTTON | HTMAXBUTTON | HTCLOSE).then_some(0))
    }

    fn handle_nc_mouse_up_msg(
        handle: HWND,
        button: MouseButton,
        wparam: WPARAM,
        lparam: LPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let lock = state.as_ref().upgradable_read();
        if !lock.hide_title_bar {
            return None;
        }

        let scale_factor = lock.scale_factor;
        let mut lock = RwLockUpgradableReadGuard::upgrade(lock);
        if let Some(mut callback) = lock.callbacks.input.take() {
            drop(lock);
            let mut cursor_point = POINT {
                x: lparam.signed_loword().into(),
                y: lparam.signed_hiword().into(),
            };
            unsafe { ScreenToClient(handle, &mut cursor_point) };
            let event = MouseUpEvent {
                button,
                position: logical_point(cursor_point.x as f32, cursor_point.y as f32, scale_factor),
                modifiers: current_modifiers(),
                click_count: 1,
            };
            let result = if callback(PlatformInput::MouseUp(event)).default_prevented {
                Some(0)
            } else {
                None
            };
            let mut lock = state.as_ref().write();
            lock.callbacks.input = Some(callback);
            if result.is_some() {
                return result;
            }
        } else {
            drop(lock);
        }

        let lock = state.as_ref().read();
        if button == MouseButton::Left {
            match wparam.0 as u32 {
                HTMINBUTTON => unsafe {
                    ShowWindowAsync(handle, SW_MINIMIZE);
                },
                HTMAXBUTTON => unsafe {
                    if lock.is_maximized() {
                        ShowWindowAsync(handle, SW_NORMAL);
                    } else {
                        ShowWindowAsync(handle, SW_MAXIMIZE);
                    }
                },
                HTCLOSE => unsafe {
                    PostMessageW(handle, WM_CLOSE, WPARAM::default(), LPARAM::default()).log_err();
                },
                _ => return None,
            };
            return Some(0);
        }

        None
    }

    fn handle_cursor_changed(
        lparam: LPARAM,
        state: Arc<RwLock<WindowsWindowState>>,
    ) -> Option<isize> {
        let mut lock = state.as_ref().write();
        lock.cursor = HCURSOR(lparam.0);
        Some(0)
    }

    fn handle_set_cursor(lparam: LPARAM, state: Arc<RwLock<WindowsWindowState>>) -> Option<isize> {
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

        let lock = state.as_ref().read();
        unsafe { SetCursor(lock.cursor) };
        Some(1)
    }
}

#[derive(Default)]
struct Callbacks {
    request_frame: Option<Box<dyn FnMut()>>,
    input: Option<Box<dyn FnMut(crate::PlatformInput) -> DispatchEventResult>>,
    active_status_change: Option<Box<dyn FnMut(bool)>>,
    resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    moved: Option<Box<dyn FnMut()>>,
    should_close: Option<Box<dyn FnMut() -> bool>>,
    close: Option<Box<dyn FnOnce()>>,
    appearance_changed: Option<Box<dyn FnMut()>>,
}

pub(crate) struct WindowsWindow {
    state: Arc<RwLock<WindowsWindowState>>,
    drag_drop_handler: IDropTarget,
}

struct WindowCreateContext {
    inner: Option<Arc<RwLock<WindowsWindowState>>>,
    handle: AnyWindowHandle,
    hide_title_bar: bool,
    display: WindowsDisplay,
    transparent: bool,
    executor: ForegroundExecutor,
    cursor: HCURSOR,
    raw_window_handles: Arc<RwLock<SmallVec<[HWND; 4]>>>,
}

impl WindowsWindow {
    pub(crate) fn new(
        handle: AnyWindowHandle,
        options: WindowParams,
        raw_window_handles: Arc<RwLock<SmallVec<[HWND; 4]>>>,
        icon: HICON,
        executor: ForegroundExecutor,
        cursor: HCURSOR,
    ) -> Self {
        let classname = register_wnd_class(icon);
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
        let hinstance = get_module_handle();
        let mut context = WindowCreateContext {
            inner: None,
            handle,
            hide_title_bar,
            // todo(windows) move window to target monitor
            // options.display_id
            display: WindowsDisplay::primary_monitor().unwrap(),
            transparent: options.window_background != WindowBackgroundAppearance::Opaque,
            executor,
            cursor,
            raw_window_handles: raw_window_handles.clone(),
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
        let raw_hwnd = context.inner.as_ref().unwrap().as_ref().read().hwnd;
        let drag_drop_handler = {
            let inner = context.inner.as_ref().unwrap();
            let handler = WindowsDragDropHandler(Arc::clone(inner));
            let drag_drop_handler: IDropTarget = handler.into();
            unsafe {
                RegisterDragDrop(raw_hwnd, &drag_drop_handler)
                    .expect("unable to register drag-drop event")
            };
            drag_drop_handler
        };
        let wnd = Self {
            state: context.inner.unwrap(),
            drag_drop_handler,
        };
        raw_window_handles.as_ref().write().push(raw_hwnd);

        unsafe { ShowWindow(raw_hwnd, SW_SHOW) };

        wnd
    }
}

impl rwh::HasWindowHandle for WindowsWindow {
    fn window_handle(&self) -> Result<rwh::WindowHandle<'_>, rwh::HandleError> {
        let raw = rwh::Win32WindowHandle::new(unsafe {
            NonZeroIsize::new_unchecked(self.state.read().hwnd.0)
        })
        .into();
        Ok(unsafe { rwh::WindowHandle::borrow_raw(raw) })
    }
}

// todo(windows)
impl rwh::HasDisplayHandle for WindowsWindow {
    fn display_handle(&self) -> Result<rwh::DisplayHandle<'_>, rwh::HandleError> {
        unimplemented!()
    }
}

impl Drop for WindowsWindow {
    fn drop(&mut self) {
        unsafe {
            let mut lock = self.state.write();
            let _ = RevokeDragDrop(lock.hwnd);
            lock.renderer.destroy();
        }
    }
}

impl PlatformWindow for WindowsWindow {
    fn bounds(&self) -> Bounds<DevicePixels> {
        self.state.as_ref().read().bounds()
    }

    fn is_maximized(&self) -> bool {
        self.state.as_ref().read().is_maximized()
    }

    fn is_minimized(&self) -> bool {
        self.state.as_ref().read().is_minimized()
    }

    /// get the logical size of the app's drawable area.
    ///
    /// Currently, GPUI uses logical size of the app to handle mouse interactions (such as
    /// whether the mouse collides with other elements of GPUI).
    fn content_size(&self) -> Size<Pixels> {
        self.state.as_ref().read().content_size()
    }

    fn scale_factor(&self) -> f32 {
        self.state.as_ref().read().scale_factor()
    }

    // todo(windows)
    fn appearance(&self) -> WindowAppearance {
        WindowAppearance::Dark
    }

    fn display(&self) -> Rc<dyn PlatformDisplay> {
        let display = self.state.as_ref().read().display.clone();
        Rc::new(display)
    }

    fn mouse_position(&self) -> Point<Pixels> {
        let lock = self.state.as_ref().read();
        let handle = lock.hwnd;
        let scale_factor = lock.scale_factor();
        drop(lock);
        let point = unsafe {
            let mut point: POINT = std::mem::zeroed();
            GetCursorPos(&mut point)
                .context("unable to get cursor position")
                .log_err();
            ScreenToClient(handle, &mut point);
            point
        };
        logical_point(point.x as f32, point.y as f32, scale_factor)
    }

    // todo(windows)
    fn modifiers(&self) -> Modifiers {
        Modifiers::none()
    }

    // todo(windows)
    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        let _ = self
            .state
            .as_ref()
            .write()
            .input_handler
            .insert(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.state.as_ref().write().input_handler.take()
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
        let lock = self.state.as_ref().read();
        let handle = lock.hwnd;
        let excutor = lock.executor.clone();
        drop(lock);
        excutor
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
                        crate::PromptLevel::Critical | crate::PromptLevel::Destructive => {
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
        println!("Trigger activate event.");
        let handle = self.state.as_ref().read().hwnd;
        unsafe { SetActiveWindow(handle) };
        unsafe { SetFocus(handle) };
        unsafe { SetForegroundWindow(handle) };
    }

    fn is_active(&self) -> bool {
        self.state.as_ref().read().hwnd == unsafe { GetActiveWindow() }
    }

    // todo(windows)
    fn set_title(&mut self, title: &str) {
        unsafe { SetWindowTextW(self.state.as_ref().read().hwnd, &HSTRING::from(title)) }
            .inspect_err(|e| log::error!("Set title failed: {e}"))
            .ok();
    }

    fn set_app_id(&mut self, _app_id: &str) {}

    fn set_background_appearance(&mut self, background_appearance: WindowBackgroundAppearance) {
        self.inner
            .renderer
            .borrow_mut()
            .update_transparency(background_appearance != WindowBackgroundAppearance::Opaque);
    }

    // todo(windows)
    fn set_edited(&mut self, _edited: bool) {}

    // todo(windows)
    fn show_character_palette(&self) {}

    fn minimize(&self) {
        unsafe { ShowWindowAsync(self.state.as_ref().read().hwnd, SW_MINIMIZE) };
    }

    fn zoom(&self) {
        unsafe { ShowWindowAsync(self.state.as_ref().read().hwnd, SW_MAXIMIZE) };
    }

    fn toggle_fullscreen(&self) {
        let executor = self.state.read().executor.clone();
        let window_state = self.state.clone();
        executor
            .spawn(async move {
                let mut lock = window_state.write();
                let StyleAndBounds {
                    style,
                    x,
                    y,
                    cx,
                    cy,
                } = if let Some(state) = lock.fullscreen.take() {
                    state
                } else {
                    let style = WINDOW_STYLE(unsafe { get_window_long(lock.hwnd, GWL_STYLE) } as _);
                    let mut rc = RECT::default();
                    unsafe { GetWindowRect(lock.hwnd, &mut rc) }.log_err();
                    let _ = lock.fullscreen.insert(StyleAndBounds {
                        style,
                        x: rc.left,
                        y: rc.top,
                        cx: rc.right - rc.left,
                        cy: rc.bottom - rc.top,
                    });
                    let style = style
                        & !(WS_THICKFRAME
                            | WS_SYSMENU
                            | WS_MAXIMIZEBOX
                            | WS_MINIMIZEBOX
                            | WS_CAPTION);
                    let bounds = lock.display.bounds();
                    StyleAndBounds {
                        style,
                        x: bounds.left().0,
                        y: bounds.top().0,
                        cx: bounds.size.width.0,
                        cy: bounds.size.height.0,
                    }
                };
                unsafe { set_window_long(lock.hwnd, GWL_STYLE, style.0 as isize) };
                unsafe {
                    SetWindowPos(
                        lock.hwnd,
                        HWND::default(),
                        x,
                        y,
                        cx,
                        cy,
                        SWP_FRAMECHANGED | SWP_NOACTIVATE | SWP_NOZORDER,
                    )
                }
                .log_err();
            })
            .detach();
    }

    fn is_fullscreen(&self) -> bool {
        self.state.as_ref().read().is_fullscreen()
    }

    // todo(windows)
    fn on_request_frame(&self, callback: Box<dyn FnMut()>) {
        self.state.as_ref().write().callbacks.request_frame = Some(callback);
    }

    // todo(windows)
    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        self.state.as_ref().write().callbacks.input = Some(callback);
    }

    // todo(windows)
    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.state.as_ref().write().callbacks.active_status_change = Some(callback);
    }

    // todo(windows)
    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.state.as_ref().write().callbacks.resize = Some(callback);
    }

    // todo(windows)
    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.state.as_ref().write().callbacks.moved = Some(callback);
    }

    // todo(windows)
    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.state.as_ref().write().callbacks.should_close = Some(callback);
    }

    // todo(windows)
    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.state.as_ref().write().callbacks.close = Some(callback);
    }

    // todo(windows)
    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.inner.as_ref().write().callbacks.appearance_changed = Some(callback);
    }

    // todo(windows)
    fn draw(&self, scene: &Scene) {
        self.state.as_ref().write().renderer.draw(scene)
    }

    // todo(windows)
    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.state.as_ref().read().renderer.sprite_atlas().clone()
    }

    fn get_raw_handle(&self) -> HWND {
        self.state.as_ref().read().hwnd
    }
}

#[implement(IDropTarget)]
struct WindowsDragDropHandler(pub Arc<RwLock<WindowsWindowState>>);

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
            if idata_obj.QueryGetData(&config as _) == S_OK {
                *pdweffect = DROPEFFECT_LINK;
                let Some(mut idata) = idata_obj.GetData(&config as _).log_err() else {
                    return Ok(());
                };
                if idata.u.hGlobal.is_invalid() {
                    return Ok(());
                }
                let hdrop = idata.u.hGlobal.0 as *mut HDROP;
                let mut paths = SmallVec::<[PathBuf; 2]>::new();
                let file_count = DragQueryFileW(*hdrop, DRAGDROP_GET_FILES_COUNT, None);
                for file_index in 0..file_count {
                    let filename_length = DragQueryFileW(*hdrop, file_index, None) as usize;
                    let mut buffer = vec![0u16; filename_length + 1];
                    let ret = DragQueryFileW(*hdrop, file_index, Some(buffer.as_mut_slice()));
                    if ret == 0 {
                        log::error!("unable to read file name");
                        continue;
                    }
                    if let Some(file_name) =
                        String::from_utf16(&buffer[0..filename_length]).log_err()
                    {
                        if let Some(path) = PathBuf::from_str(&file_name).log_err() {
                            paths.push(path);
                        }
                    }
                }
                ReleaseStgMedium(&mut idata);
                let mut cursor_position = POINT { x: pt.x, y: pt.y };
                ScreenToClient(self.0.hwnd, &mut cursor_position);
                let scale_factor = self.0.scale_factor.get();
                let input = PlatformInput::FileDrop(FileDropEvent::Entered {
                    position: logical_point(
                        cursor_position.x as f32,
                        cursor_position.y as f32,
                        scale_factor,
                    ),
                    paths: ExternalPaths(paths),
                });
                self.0.as_ref().write().handle_drag_drop(input);
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
        let mut cursor_position = POINT { x: pt.x, y: pt.y };
        unsafe {
            ScreenToClient(self.0.hwnd, &mut cursor_position);
        }
        let scale_factor = self.0.scale_factor.get();
        let input = PlatformInput::FileDrop(FileDropEvent::Pending {
            position: logical_point(
                cursor_position.x as f32,
                cursor_position.y as f32,
                scale_factor,
            ),
        });
        self.0.as_ref().write().handle_drag_drop(input);

        Ok(())
    }

    fn DragLeave(&self) -> windows::core::Result<()> {
        let input = PlatformInput::FileDrop(FileDropEvent::Exited);
        self.0.as_ref().write().handle_drag_drop(input);

        Ok(())
    }

    fn Drop(
        &self,
        _pdataobj: Option<&IDataObject>,
        _grfkeystate: MODIFIERKEYS_FLAGS,
        pt: &POINTL,
        _pdweffect: *mut DROPEFFECT,
    ) -> windows::core::Result<()> {
        let mut cursor_position = POINT { x: pt.x, y: pt.y };
        unsafe {
            ScreenToClient(self.0.hwnd, &mut cursor_position);
        }
        let scale_factor = self.0.scale_factor.get();
        let input = PlatformInput::FileDrop(FileDropEvent::Submit {
            position: logical_point(
                cursor_position.x as f32,
                cursor_position.y as f32,
                scale_factor,
            ),
        });
        self.0.as_ref().write().handle_drag_drop(input);

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
            hInstance: get_module_handle().into(),
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
        let inner = WindowsWindowState::new(
            hwnd,
            cs,
            ctx.executor.clone(),
            ctx.handle,
            ctx.hide_title_bar,
            ctx.display.clone(),
            ctx.transparent,
            ctx.cursor,
            ctx.raw_window_handles.clone(),
        );
        let weak = Box::new(Arc::downgrade(&inner));
        unsafe { set_window_long(hwnd, GWLP_USERDATA, Box::into_raw(weak) as isize) };
        ctx.inner = Some(inner);
        return LRESULT(1);
    }
    let ptr = unsafe { get_window_long(hwnd, GWLP_USERDATA) }
        as *mut std::sync::Weak<RwLock<WindowsWindowState>>;
    if ptr.is_null() {
        return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
    }
    let inner = unsafe { &*ptr };
    {
        let indent_count = INDENT_COUNT.load(std::sync::atomic::Ordering::Relaxed);
        let mut pre_indent = "".to_owned();
        for _ in 0..indent_count {
            pre_indent.push_str(INDENT_STRING);
        }
        println!("{}Handling MSG: {}", pre_indent, msg);
    }
    INDENT_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let r = if let Some(state) = inner.upgrade() {
        WindowsWindowState::handle_msg(hwnd, msg, wparam, lparam, state)
    } else {
        unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
    };
    INDENT_COUNT.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    {
        let indent_count = INDENT_COUNT.load(std::sync::atomic::Ordering::Relaxed);
        let mut pre_indent = "".to_owned();
        for _ in 0..indent_count {
            pre_indent.push_str(INDENT_STRING);
        }
        println!("{}Finished handling: {}", pre_indent, msg);
    }
    if msg == WM_NCDESTROY {
        unsafe { set_window_long(hwnd, GWLP_USERDATA, 0) };
        unsafe { drop(Box::from_raw(ptr)) };
    }
    r
}

static INDENT_COUNT: AtomicUsize = AtomicUsize::new(0);
const INDENT_STRING: &str = "   ";

pub(crate) fn try_get_window_inner(hwnd: HWND) -> Option<Arc<RwLock<WindowsWindowState>>> {
    if hwnd == HWND(0) {
        return None;
    }

    let ptr = unsafe { get_window_long(hwnd, GWLP_USERDATA) }
        as *mut std::sync::Weak<RwLock<WindowsWindowState>>;
    if !ptr.is_null() {
        let inner = unsafe { &*ptr };
        inner.upgrade()
    } else {
        None
    }
}

fn parse_syskeydown_msg_keystroke(wparam: WPARAM) -> Option<Keystroke> {
    let modifiers = current_modifiers();
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

fn parse_keydown_msg_keystroke(wparam: WPARAM) -> Option<Keystroke> {
    let vk_code = wparam.loword();

    let modifiers = current_modifiers();
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
        VK_DELETE => Some("delete"),
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

fn parse_char_msg_keystroke(wparam: WPARAM) -> Option<Keystroke> {
    let src = [wparam.0 as u16];
    let Ok(first_char) = char::decode_utf16(src).collect::<Vec<_>>()[0] else {
        return None;
    };
    if first_char.is_control() {
        None
    } else {
        let mut modifiers = current_modifiers();
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

/// mark window client rect to be re-drawn
/// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidaterect
pub(crate) fn invalidate_client_area(handle: HWND) {
    unsafe { InvalidateRect(handle, None, FALSE) };
}

fn parse_ime_compostion_string(handle: HWND) -> Option<(String, usize)> {
    unsafe {
        let ctx = ImmGetContext(handle);
        let string_len = ImmGetCompositionStringW(ctx, GCS_COMPSTR, None, 0);
        let result = if string_len >= 0 {
            let mut buffer = vec![0u8; string_len as usize + 2];
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
        ImmReleaseContext(handle, ctx);
        result
    }
}

fn retrieve_composition_cursor_position(handle: HWND) -> usize {
    unsafe {
        let ctx = ImmGetContext(handle);
        let ret = ImmGetCompositionStringW(ctx, GCS_CURSORPOS, None, 0);
        ImmReleaseContext(handle, ctx);
        ret as usize
    }
}

fn parse_ime_compostion_result(handle: HWND) -> Option<String> {
    unsafe {
        let ctx = ImmGetContext(handle);
        let string_len = ImmGetCompositionStringW(ctx, GCS_RESULTSTR, None, 0);
        let result = if string_len >= 0 {
            let mut buffer = vec![0u8; string_len as usize + 2];
            ImmGetCompositionStringW(
                ctx,
                GCS_RESULTSTR,
                Some(buffer.as_mut_ptr() as _),
                string_len as _,
            );
            let wstring = std::slice::from_raw_parts::<u16>(
                buffer.as_mut_ptr().cast::<u16>(),
                string_len as usize / 2,
            );
            let string = String::from_utf16_lossy(wstring);
            Some(string)
        } else {
            None
        };
        ImmReleaseContext(handle, ctx);
        result
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

fn get_module_handle() -> HMODULE {
    unsafe {
        let mut h_module = std::mem::zeroed();
        GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            windows::core::w!("ZedModule"),
            &mut h_module,
        )
        .expect("Unable to get module handle"); // this should never fail

        h_module
    }
}

#[inline]
fn is_virtual_key_pressed(vkey: VIRTUAL_KEY) -> bool {
    unsafe { GetKeyState(vkey.0 as i32) < 0 }
}

#[inline]
fn current_modifiers() -> Modifiers {
    Modifiers {
        control: is_virtual_key_pressed(VK_CONTROL),
        alt: is_virtual_key_pressed(VK_MENU),
        shift: is_virtual_key_pressed(VK_SHIFT),
        platform: is_virtual_key_pressed(VK_LWIN) || is_virtual_key_pressed(VK_RWIN),
        function: false,
    }
}

// https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragqueryfilew
const DRAGDROP_GET_FILES_COUNT: u32 = 0xFFFFFFFF;
// https://learn.microsoft.com/en-us/windows/win32/controls/ttm-setdelaytime?redirectedfrom=MSDN
const DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(500);
// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics
const DOUBLE_CLICK_SPATIAL_TOLERANCE: i32 = 4;
const SIZE_MOVE_LOOP_TIMER_ID: usize = 1;

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
