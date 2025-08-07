use std::rc::Rc;

use ::util::ResultExt;
use anyhow::Context as _;
use windows::{
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        System::SystemServices::*,
        UI::{
            Controls::*,
            HiDpi::*,
            Input::{Ime::*, KeyboardAndMouse::*},
            WindowsAndMessaging::*,
        },
    },
    core::PCWSTR,
};

use crate::*;

pub(crate) const WM_GPUI_CURSOR_STYLE_CHANGED: u32 = WM_USER + 1;
pub(crate) const WM_GPUI_CLOSE_ONE_WINDOW: u32 = WM_USER + 2;
pub(crate) const WM_GPUI_TASK_DISPATCHED_ON_MAIN_THREAD: u32 = WM_USER + 3;
pub(crate) const WM_GPUI_DOCK_MENU_ACTION: u32 = WM_USER + 4;
pub(crate) const WM_GPUI_FORCE_UPDATE_WINDOW: u32 = WM_USER + 5;

const SIZE_MOVE_LOOP_TIMER_ID: usize = 1;
const AUTO_HIDE_TASKBAR_THICKNESS_PX: i32 = 1;

impl WindowsWindowInner {
    pub(crate) fn handle_msg(
        self: &Rc<Self>,
        handle: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        let handled = match msg {
            WM_ACTIVATE => self.handle_activate_msg(wparam),
            WM_CREATE => self.handle_create_msg(handle),
            WM_DEVICECHANGE => self.handle_device_change_msg(handle, wparam),
            WM_MOVE => self.handle_move_msg(handle, lparam),
            WM_SIZE => self.handle_size_msg(wparam, lparam),
            WM_GETMINMAXINFO => self.handle_get_min_max_info_msg(lparam),
            WM_ENTERSIZEMOVE | WM_ENTERMENULOOP => self.handle_size_move_loop(handle),
            WM_EXITSIZEMOVE | WM_EXITMENULOOP => self.handle_size_move_loop_exit(handle),
            WM_TIMER => self.handle_timer_msg(handle, wparam),
            WM_NCCALCSIZE => self.handle_calc_client_size(handle, wparam, lparam),
            WM_DPICHANGED => self.handle_dpi_changed_msg(handle, wparam, lparam),
            WM_DISPLAYCHANGE => self.handle_display_change_msg(handle),
            WM_NCHITTEST => self.handle_hit_test_msg(handle, msg, wparam, lparam),
            WM_PAINT => self.handle_paint_msg(handle),
            WM_CLOSE => self.handle_close_msg(),
            WM_DESTROY => self.handle_destroy_msg(handle),
            WM_MOUSEMOVE => self.handle_mouse_move_msg(handle, lparam, wparam),
            WM_MOUSELEAVE | WM_NCMOUSELEAVE => self.handle_mouse_leave_msg(),
            WM_NCMOUSEMOVE => self.handle_nc_mouse_move_msg(handle, lparam),
            WM_NCLBUTTONDOWN => {
                self.handle_nc_mouse_down_msg(handle, MouseButton::Left, wparam, lparam)
            }
            WM_NCRBUTTONDOWN => {
                self.handle_nc_mouse_down_msg(handle, MouseButton::Right, wparam, lparam)
            }
            WM_NCMBUTTONDOWN => {
                self.handle_nc_mouse_down_msg(handle, MouseButton::Middle, wparam, lparam)
            }
            WM_NCLBUTTONUP => {
                self.handle_nc_mouse_up_msg(handle, MouseButton::Left, wparam, lparam)
            }
            WM_NCRBUTTONUP => {
                self.handle_nc_mouse_up_msg(handle, MouseButton::Right, wparam, lparam)
            }
            WM_NCMBUTTONUP => {
                self.handle_nc_mouse_up_msg(handle, MouseButton::Middle, wparam, lparam)
            }
            WM_LBUTTONDOWN => self.handle_mouse_down_msg(handle, MouseButton::Left, lparam),
            WM_RBUTTONDOWN => self.handle_mouse_down_msg(handle, MouseButton::Right, lparam),
            WM_MBUTTONDOWN => self.handle_mouse_down_msg(handle, MouseButton::Middle, lparam),
            WM_XBUTTONDOWN => {
                self.handle_xbutton_msg(handle, wparam, lparam, Self::handle_mouse_down_msg)
            }
            WM_LBUTTONUP => self.handle_mouse_up_msg(handle, MouseButton::Left, lparam),
            WM_RBUTTONUP => self.handle_mouse_up_msg(handle, MouseButton::Right, lparam),
            WM_MBUTTONUP => self.handle_mouse_up_msg(handle, MouseButton::Middle, lparam),
            WM_XBUTTONUP => {
                self.handle_xbutton_msg(handle, wparam, lparam, Self::handle_mouse_up_msg)
            }
            WM_MOUSEWHEEL => self.handle_mouse_wheel_msg(handle, wparam, lparam),
            WM_MOUSEHWHEEL => self.handle_mouse_horizontal_wheel_msg(handle, wparam, lparam),
            WM_SYSKEYDOWN => self.handle_syskeydown_msg(handle, wparam, lparam),
            WM_SYSKEYUP => self.handle_syskeyup_msg(handle, wparam, lparam),
            WM_SYSCOMMAND => self.handle_system_command(wparam),
            WM_KEYDOWN => self.handle_keydown_msg(handle, wparam, lparam),
            WM_KEYUP => self.handle_keyup_msg(handle, wparam, lparam),
            WM_CHAR => self.handle_char_msg(wparam),
            WM_DEADCHAR => self.handle_dead_char_msg(wparam),
            WM_IME_STARTCOMPOSITION => self.handle_ime_position(handle),
            WM_IME_COMPOSITION => self.handle_ime_composition(handle, lparam),
            WM_SETCURSOR => self.handle_set_cursor(handle, lparam),
            WM_SETTINGCHANGE => self.handle_system_settings_changed(handle, wparam, lparam),
            WM_INPUTLANGCHANGE => self.handle_input_language_changed(lparam),
            WM_GPUI_CURSOR_STYLE_CHANGED => self.handle_cursor_changed(lparam),
            WM_GPUI_FORCE_UPDATE_WINDOW => self.draw_window(handle, true),
            _ => None,
        };
        if let Some(n) = handled {
            LRESULT(n)
        } else {
            unsafe { DefWindowProcW(handle, msg, wparam, lparam) }
        }
    }

    fn handle_move_msg(&self, handle: HWND, lparam: LPARAM) -> Option<isize> {
        let mut lock = self.state.borrow_mut();
        let origin = logical_point(
            lparam.signed_loword() as f32,
            lparam.signed_hiword() as f32,
            lock.scale_factor,
        );
        lock.origin = origin;
        let size = lock.logical_size;
        let center_x = origin.x.0 + size.width.0 / 2.;
        let center_y = origin.y.0 + size.height.0 / 2.;
        let monitor_bounds = lock.display.bounds();
        if center_x < monitor_bounds.left().0
            || center_x > monitor_bounds.right().0
            || center_y < monitor_bounds.top().0
            || center_y > monitor_bounds.bottom().0
        {
            // center of the window may have moved to another monitor
            let monitor = unsafe { MonitorFromWindow(handle, MONITOR_DEFAULTTONULL) };
            // minimize the window can trigger this event too, in this case,
            // monitor is invalid, we do nothing.
            if !monitor.is_invalid() && lock.display.handle != monitor {
                // we will get the same monitor if we only have one
                lock.display = WindowsDisplay::new_with_handle(monitor);
            }
        }
        if let Some(mut callback) = lock.callbacks.moved.take() {
            drop(lock);
            callback();
            self.state.borrow_mut().callbacks.moved = Some(callback);
        }
        Some(0)
    }

    fn handle_get_min_max_info_msg(&self, lparam: LPARAM) -> Option<isize> {
        let lock = self.state.borrow();
        let min_size = lock.min_size?;
        let scale_factor = lock.scale_factor;
        let boarder_offset = lock.border_offset;
        drop(lock);
        unsafe {
            let minmax_info = &mut *(lparam.0 as *mut MINMAXINFO);
            minmax_info.ptMinTrackSize.x =
                min_size.width.scale(scale_factor).0 as i32 + boarder_offset.width_offset;
            minmax_info.ptMinTrackSize.y =
                min_size.height.scale(scale_factor).0 as i32 + boarder_offset.height_offset;
        }
        Some(0)
    }

    fn handle_size_msg(&self, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        let mut lock = self.state.borrow_mut();

        // Don't resize the renderer when the window is minimized, but record that it was minimized so
        // that on restore the swap chain can be recreated via `update_drawable_size_even_if_unchanged`.
        if wparam.0 == SIZE_MINIMIZED as usize {
            lock.restore_from_minimized = lock.callbacks.request_frame.take();
            return Some(0);
        }

        let width = lparam.loword().max(1) as i32;
        let height = lparam.hiword().max(1) as i32;
        let new_size = size(DevicePixels(width), DevicePixels(height));

        let scale_factor = lock.scale_factor;
        let mut should_resize_renderer = false;
        if lock.restore_from_minimized.is_some() {
            lock.callbacks.request_frame = lock.restore_from_minimized.take();
        } else {
            should_resize_renderer = true;
        }
        drop(lock);

        self.handle_size_change(new_size, scale_factor, should_resize_renderer);
        Some(0)
    }

    fn handle_size_change(
        &self,
        device_size: Size<DevicePixels>,
        scale_factor: f32,
        should_resize_renderer: bool,
    ) {
        let new_logical_size = device_size.to_pixels(scale_factor);
        let mut lock = self.state.borrow_mut();
        lock.logical_size = new_logical_size;
        if should_resize_renderer {
            lock.renderer.resize(device_size).log_err();
        }
        if let Some(mut callback) = lock.callbacks.resize.take() {
            drop(lock);
            callback(new_logical_size, scale_factor);
            self.state.borrow_mut().callbacks.resize = Some(callback);
        }
    }

    fn handle_size_move_loop(&self, handle: HWND) -> Option<isize> {
        unsafe {
            let ret = SetTimer(
                Some(handle),
                SIZE_MOVE_LOOP_TIMER_ID,
                USER_TIMER_MINIMUM,
                None,
            );
            if ret == 0 {
                log::error!(
                    "unable to create timer: {}",
                    std::io::Error::last_os_error()
                );
            }
        }
        None
    }

    fn handle_size_move_loop_exit(&self, handle: HWND) -> Option<isize> {
        unsafe {
            KillTimer(Some(handle), SIZE_MOVE_LOOP_TIMER_ID).log_err();
        }
        None
    }

    fn handle_timer_msg(&self, handle: HWND, wparam: WPARAM) -> Option<isize> {
        if wparam.0 == SIZE_MOVE_LOOP_TIMER_ID {
            for runnable in self.main_receiver.drain() {
                runnable.run();
            }
            self.handle_paint_msg(handle)
        } else {
            None
        }
    }

    fn handle_paint_msg(&self, handle: HWND) -> Option<isize> {
        self.draw_window(handle, false)
    }

    fn handle_close_msg(&self) -> Option<isize> {
        let mut callback = self.state.borrow_mut().callbacks.should_close.take()?;
        let should_close = callback();
        self.state.borrow_mut().callbacks.should_close = Some(callback);
        if should_close { None } else { Some(0) }
    }

    fn handle_destroy_msg(&self, handle: HWND) -> Option<isize> {
        let callback = {
            let mut lock = self.state.borrow_mut();
            lock.callbacks.close.take()
        };
        if let Some(callback) = callback {
            callback();
        }
        unsafe {
            PostThreadMessageW(
                self.main_thread_id_win32,
                WM_GPUI_CLOSE_ONE_WINDOW,
                WPARAM(self.validation_number),
                LPARAM(handle.0 as isize),
            )
            .log_err();
        }
        Some(0)
    }

    fn handle_mouse_move_msg(&self, handle: HWND, lparam: LPARAM, wparam: WPARAM) -> Option<isize> {
        self.start_tracking_mouse(handle, TME_LEAVE);

        let mut lock = self.state.borrow_mut();
        let Some(mut func) = lock.callbacks.input.take() else {
            return Some(1);
        };
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
        let input = PlatformInput::MouseMove(MouseMoveEvent {
            position: logical_point(x, y, scale_factor),
            pressed_button,
            modifiers: current_modifiers(),
        });
        let handled = !func(input).propagate;
        self.state.borrow_mut().callbacks.input = Some(func);

        if handled { Some(0) } else { Some(1) }
    }

    fn handle_mouse_leave_msg(&self) -> Option<isize> {
        let mut lock = self.state.borrow_mut();
        lock.hovered = false;
        if let Some(mut callback) = lock.callbacks.hovered_status_change.take() {
            drop(lock);
            callback(false);
            self.state.borrow_mut().callbacks.hovered_status_change = Some(callback);
        }

        Some(0)
    }

    fn handle_syskeydown_msg(&self, handle: HWND, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        let mut lock = self.state.borrow_mut();
        let input = handle_key_event(handle, wparam, lparam, &mut lock, |keystroke| {
            PlatformInput::KeyDown(KeyDownEvent {
                keystroke,
                is_held: lparam.0 & (0x1 << 30) > 0,
            })
        })?;
        let mut func = lock.callbacks.input.take()?;
        drop(lock);

        let handled = !func(input).propagate;

        let mut lock = self.state.borrow_mut();
        lock.callbacks.input = Some(func);

        if handled {
            lock.system_key_handled = true;
            Some(0)
        } else {
            // we need to call `DefWindowProcW`, or we will lose the system-wide `Alt+F4`, `Alt+{other keys}`
            // shortcuts.
            None
        }
    }

    fn handle_syskeyup_msg(&self, handle: HWND, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        let mut lock = self.state.borrow_mut();
        let input = handle_key_event(handle, wparam, lparam, &mut lock, |keystroke| {
            PlatformInput::KeyUp(KeyUpEvent { keystroke })
        })?;
        let mut func = lock.callbacks.input.take()?;
        drop(lock);
        func(input);
        self.state.borrow_mut().callbacks.input = Some(func);

        // Always return 0 to indicate that the message was handled, so we could properly handle `ModifiersChanged` event.
        Some(0)
    }

    // It's a known bug that you can't trigger `ctrl-shift-0`. See:
    // https://superuser.com/questions/1455762/ctrl-shift-number-key-combination-has-stopped-working-for-a-few-numbers
    fn handle_keydown_msg(&self, handle: HWND, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        let mut lock = self.state.borrow_mut();
        let Some(input) = handle_key_event(handle, wparam, lparam, &mut lock, |keystroke| {
            PlatformInput::KeyDown(KeyDownEvent {
                keystroke,
                is_held: lparam.0 & (0x1 << 30) > 0,
            })
        }) else {
            return Some(1);
        };
        drop(lock);

        let is_composing = self
            .with_input_handler(|input_handler| input_handler.marked_text_range())
            .flatten()
            .is_some();
        if is_composing {
            translate_message(handle, wparam, lparam);
            return Some(0);
        }

        let Some(mut func) = self.state.borrow_mut().callbacks.input.take() else {
            return Some(1);
        };

        let handled = !func(input).propagate;

        self.state.borrow_mut().callbacks.input = Some(func);

        if handled {
            Some(0)
        } else {
            translate_message(handle, wparam, lparam);
            Some(1)
        }
    }

    fn handle_keyup_msg(&self, handle: HWND, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        let mut lock = self.state.borrow_mut();
        let Some(input) = handle_key_event(handle, wparam, lparam, &mut lock, |keystroke| {
            PlatformInput::KeyUp(KeyUpEvent { keystroke })
        }) else {
            return Some(1);
        };

        let Some(mut func) = lock.callbacks.input.take() else {
            return Some(1);
        };
        drop(lock);

        let handled = !func(input).propagate;
        self.state.borrow_mut().callbacks.input = Some(func);

        if handled { Some(0) } else { Some(1) }
    }

    fn handle_char_msg(&self, wparam: WPARAM) -> Option<isize> {
        let input = self.parse_char_message(wparam)?;
        self.with_input_handler(|input_handler| {
            input_handler.replace_text_in_range(None, &input);
        });

        Some(0)
    }

    fn handle_dead_char_msg(&self, wparam: WPARAM) -> Option<isize> {
        let ch = char::from_u32(wparam.0 as u32)?.to_string();
        self.with_input_handler(|input_handler| {
            input_handler.replace_and_mark_text_in_range(None, &ch, None);
        });
        None
    }

    fn handle_mouse_down_msg(
        &self,
        handle: HWND,
        button: MouseButton,
        lparam: LPARAM,
    ) -> Option<isize> {
        unsafe { SetCapture(handle) };
        let mut lock = self.state.borrow_mut();
        let Some(mut func) = lock.callbacks.input.take() else {
            return Some(1);
        };
        let x = lparam.signed_loword();
        let y = lparam.signed_hiword();
        let physical_point = point(DevicePixels(x as i32), DevicePixels(y as i32));
        let click_count = lock.click_state.update(button, physical_point);
        let scale_factor = lock.scale_factor;
        drop(lock);

        let input = PlatformInput::MouseDown(MouseDownEvent {
            button,
            position: logical_point(x as f32, y as f32, scale_factor),
            modifiers: current_modifiers(),
            click_count,
            first_mouse: false,
        });
        let handled = !func(input).propagate;
        self.state.borrow_mut().callbacks.input = Some(func);

        if handled { Some(0) } else { Some(1) }
    }

    fn handle_mouse_up_msg(
        &self,
        _handle: HWND,
        button: MouseButton,
        lparam: LPARAM,
    ) -> Option<isize> {
        unsafe { ReleaseCapture().log_err() };
        let mut lock = self.state.borrow_mut();
        let Some(mut func) = lock.callbacks.input.take() else {
            return Some(1);
        };
        let x = lparam.signed_loword() as f32;
        let y = lparam.signed_hiword() as f32;
        let click_count = lock.click_state.current_count;
        let scale_factor = lock.scale_factor;
        drop(lock);

        let input = PlatformInput::MouseUp(MouseUpEvent {
            button,
            position: logical_point(x, y, scale_factor),
            modifiers: current_modifiers(),
            click_count,
        });
        let handled = !func(input).propagate;
        self.state.borrow_mut().callbacks.input = Some(func);

        if handled { Some(0) } else { Some(1) }
    }

    fn handle_xbutton_msg(
        &self,
        handle: HWND,
        wparam: WPARAM,
        lparam: LPARAM,
        handler: impl Fn(&Self, HWND, MouseButton, LPARAM) -> Option<isize>,
    ) -> Option<isize> {
        let nav_dir = match wparam.hiword() {
            XBUTTON1 => NavigationDirection::Back,
            XBUTTON2 => NavigationDirection::Forward,
            _ => return Some(1),
        };
        handler(self, handle, MouseButton::Navigate(nav_dir), lparam)
    }

    fn handle_mouse_wheel_msg(
        &self,
        handle: HWND,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<isize> {
        let modifiers = current_modifiers();
        let mut lock = self.state.borrow_mut();
        let Some(mut func) = lock.callbacks.input.take() else {
            return Some(1);
        };
        let scale_factor = lock.scale_factor;
        let wheel_scroll_amount = match modifiers.shift {
            true => lock.system_settings.mouse_wheel_settings.wheel_scroll_chars,
            false => lock.system_settings.mouse_wheel_settings.wheel_scroll_lines,
        };
        drop(lock);

        let wheel_distance =
            (wparam.signed_hiword() as f32 / WHEEL_DELTA as f32) * wheel_scroll_amount as f32;
        let mut cursor_point = POINT {
            x: lparam.signed_loword().into(),
            y: lparam.signed_hiword().into(),
        };
        unsafe { ScreenToClient(handle, &mut cursor_point).ok().log_err() };
        let input = PlatformInput::ScrollWheel(ScrollWheelEvent {
            position: logical_point(cursor_point.x as f32, cursor_point.y as f32, scale_factor),
            delta: ScrollDelta::Lines(match modifiers.shift {
                true => Point {
                    x: wheel_distance,
                    y: 0.0,
                },
                false => Point {
                    y: wheel_distance,
                    x: 0.0,
                },
            }),
            modifiers,
            touch_phase: TouchPhase::Moved,
        });
        let handled = !func(input).propagate;
        self.state.borrow_mut().callbacks.input = Some(func);

        if handled { Some(0) } else { Some(1) }
    }

    fn handle_mouse_horizontal_wheel_msg(
        &self,
        handle: HWND,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<isize> {
        let mut lock = self.state.borrow_mut();
        let Some(mut func) = lock.callbacks.input.take() else {
            return Some(1);
        };
        let scale_factor = lock.scale_factor;
        let wheel_scroll_chars = lock.system_settings.mouse_wheel_settings.wheel_scroll_chars;
        drop(lock);

        let wheel_distance =
            (-wparam.signed_hiword() as f32 / WHEEL_DELTA as f32) * wheel_scroll_chars as f32;
        let mut cursor_point = POINT {
            x: lparam.signed_loword().into(),
            y: lparam.signed_hiword().into(),
        };
        unsafe { ScreenToClient(handle, &mut cursor_point).ok().log_err() };
        let event = PlatformInput::ScrollWheel(ScrollWheelEvent {
            position: logical_point(cursor_point.x as f32, cursor_point.y as f32, scale_factor),
            delta: ScrollDelta::Lines(Point {
                x: wheel_distance,
                y: 0.0,
            }),
            modifiers: current_modifiers(),
            touch_phase: TouchPhase::Moved,
        });
        let handled = !func(event).propagate;
        self.state.borrow_mut().callbacks.input = Some(func);

        if handled { Some(0) } else { Some(1) }
    }

    fn retrieve_caret_position(&self) -> Option<POINT> {
        self.with_input_handler_and_scale_factor(|input_handler, scale_factor| {
            let caret_range = input_handler.selected_text_range(false)?;
            let caret_position = input_handler.bounds_for_range(caret_range.range)?;
            Some(POINT {
                // logical to physical
                x: (caret_position.origin.x.0 * scale_factor) as i32,
                y: (caret_position.origin.y.0 * scale_factor) as i32
                    + ((caret_position.size.height.0 * scale_factor) as i32 / 2),
            })
        })
    }

    fn handle_ime_position(&self, handle: HWND) -> Option<isize> {
        unsafe {
            let ctx = ImmGetContext(handle);

            let Some(caret_position) = self.retrieve_caret_position() else {
                return Some(0);
            };
            {
                let config = COMPOSITIONFORM {
                    dwStyle: CFS_POINT,
                    ptCurrentPos: caret_position,
                    ..Default::default()
                };
                ImmSetCompositionWindow(ctx, &config as _).ok().log_err();
            }
            {
                let config = CANDIDATEFORM {
                    dwStyle: CFS_CANDIDATEPOS,
                    ptCurrentPos: caret_position,
                    ..Default::default()
                };
                ImmSetCandidateWindow(ctx, &config as _).ok().log_err();
            }
            ImmReleaseContext(handle, ctx).ok().log_err();
            Some(0)
        }
    }

    fn handle_ime_composition(&self, handle: HWND, lparam: LPARAM) -> Option<isize> {
        let ctx = unsafe { ImmGetContext(handle) };
        let result = self.handle_ime_composition_inner(ctx, lparam);
        unsafe { ImmReleaseContext(handle, ctx).ok().log_err() };
        result
    }

    fn handle_ime_composition_inner(&self, ctx: HIMC, lparam: LPARAM) -> Option<isize> {
        let lparam = lparam.0 as u32;
        if lparam == 0 {
            // Japanese IME may send this message with lparam = 0, which indicates that
            // there is no composition string.
            self.with_input_handler(|input_handler| {
                input_handler.replace_text_in_range(None, "");
            })?;
            Some(0)
        } else {
            if lparam & GCS_COMPSTR.0 > 0 {
                let comp_string = parse_ime_composition_string(ctx, GCS_COMPSTR)?;
                let caret_pos =
                    (!comp_string.is_empty() && lparam & GCS_CURSORPOS.0 > 0).then(|| {
                        let pos = retrieve_composition_cursor_position(ctx);
                        pos..pos
                    });
                self.with_input_handler(|input_handler| {
                    input_handler.replace_and_mark_text_in_range(None, &comp_string, caret_pos);
                })?;
            }
            if lparam & GCS_RESULTSTR.0 > 0 {
                let comp_result = parse_ime_composition_string(ctx, GCS_RESULTSTR)?;
                self.with_input_handler(|input_handler| {
                    input_handler.replace_text_in_range(None, &comp_result);
                })?;
                return Some(0);
            }

            // currently, we don't care other stuff
            None
        }
    }

    /// SEE: https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize
    fn handle_calc_client_size(
        &self,
        handle: HWND,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<isize> {
        if !self.hide_title_bar || self.state.borrow().is_fullscreen() || wparam.0 == 0 {
            return None;
        }

        let is_maximized = self.state.borrow().is_maximized();
        let insets = get_client_area_insets(handle, is_maximized, self.windows_version);
        // wparam is TRUE so lparam points to an NCCALCSIZE_PARAMS structure
        let mut params = lparam.0 as *mut NCCALCSIZE_PARAMS;
        let mut requested_client_rect = unsafe { &mut ((*params).rgrc) };

        requested_client_rect[0].left += insets.left;
        requested_client_rect[0].top += insets.top;
        requested_client_rect[0].right -= insets.right;
        requested_client_rect[0].bottom -= insets.bottom;

        // Fix auto hide taskbar not showing. This solution is based on the approach
        // used by Chrome. However, it may result in one row of pixels being obscured
        // in our client area. But as Chrome says, "there seems to be no better solution."
        if is_maximized {
            if let Some(ref taskbar_position) = self
                .state
                .borrow()
                .system_settings
                .auto_hide_taskbar_position
            {
                // Fot the auto-hide taskbar, adjust in by 1 pixel on taskbar edge,
                // so the window isn't treated as a "fullscreen app", which would cause
                // the taskbar to disappear.
                match taskbar_position {
                    AutoHideTaskbarPosition::Left => {
                        requested_client_rect[0].left += AUTO_HIDE_TASKBAR_THICKNESS_PX
                    }
                    AutoHideTaskbarPosition::Top => {
                        requested_client_rect[0].top += AUTO_HIDE_TASKBAR_THICKNESS_PX
                    }
                    AutoHideTaskbarPosition::Right => {
                        requested_client_rect[0].right -= AUTO_HIDE_TASKBAR_THICKNESS_PX
                    }
                    AutoHideTaskbarPosition::Bottom => {
                        requested_client_rect[0].bottom -= AUTO_HIDE_TASKBAR_THICKNESS_PX
                    }
                }
            }
        }

        Some(0)
    }

    fn handle_activate_msg(self: &Rc<Self>, wparam: WPARAM) -> Option<isize> {
        let activated = wparam.loword() > 0;
        let this = self.clone();
        self.executor
            .spawn(async move {
                let mut lock = this.state.borrow_mut();
                if let Some(mut func) = lock.callbacks.active_status_change.take() {
                    drop(lock);
                    func(activated);
                    this.state.borrow_mut().callbacks.active_status_change = Some(func);
                }
            })
            .detach();

        None
    }

    fn handle_create_msg(&self, handle: HWND) -> Option<isize> {
        if self.hide_title_bar {
            notify_frame_changed(handle);
            Some(0)
        } else {
            None
        }
    }

    fn handle_dpi_changed_msg(
        &self,
        handle: HWND,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<isize> {
        let new_dpi = wparam.loword() as f32;
        let mut lock = self.state.borrow_mut();
        let is_maximized = lock.is_maximized();
        let new_scale_factor = new_dpi / USER_DEFAULT_SCREEN_DPI as f32;
        lock.scale_factor = new_scale_factor;
        lock.border_offset.update(handle).log_err();
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

        // When maximized, SetWindowPos doesn't send WM_SIZE, so we need to manually
        // update the size and call the resize callback
        if is_maximized {
            let device_size = size(DevicePixels(width), DevicePixels(height));
            self.handle_size_change(device_size, new_scale_factor, true);
        }

        Some(0)
    }

    /// The following conditions will trigger this event:
    /// 1. The monitor on which the window is located goes offline or changes resolution.
    /// 2. Another monitor goes offline, is plugged in, or changes resolution.
    ///
    /// In either case, the window will only receive information from the monitor on which
    /// it is located.
    ///
    /// For example, in the case of condition 2, where the monitor on which the window is
    /// located has actually changed nothing, it will still receive this event.
    fn handle_display_change_msg(&self, handle: HWND) -> Option<isize> {
        // NOTE:
        // Even the `lParam` holds the resolution of the screen, we just ignore it.
        // Because WM_DPICHANGED, WM_MOVE, WM_SIZE will come first, window reposition and resize
        // are handled there.
        // So we only care about if monitor is disconnected.
        let previous_monitor = self.state.borrow().display;
        if WindowsDisplay::is_connected(previous_monitor.handle) {
            // we are fine, other display changed
            return None;
        }
        // display disconnected
        // in this case, the OS will move our window to another monitor, and minimize it.
        // we deminimize the window and query the monitor after moving
        unsafe {
            let _ = ShowWindow(handle, SW_SHOWNORMAL);
        };
        let new_monitor = unsafe { MonitorFromWindow(handle, MONITOR_DEFAULTTONULL) };
        // all monitors disconnected
        if new_monitor.is_invalid() {
            log::error!("No monitor detected!");
            return None;
        }
        let new_display = WindowsDisplay::new_with_handle(new_monitor);
        self.state.borrow_mut().display = new_display;
        Some(0)
    }

    fn handle_hit_test_msg(
        &self,
        handle: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<isize> {
        if !self.is_movable || self.state.borrow().is_fullscreen() {
            return None;
        }

        let mut lock = self.state.borrow_mut();
        if let Some(mut callback) = lock.callbacks.hit_test_window_control.take() {
            drop(lock);
            let area = callback();
            self.state.borrow_mut().callbacks.hit_test_window_control = Some(callback);
            if let Some(area) = area {
                return match area {
                    WindowControlArea::Drag => Some(HTCAPTION as _),
                    WindowControlArea::Close => Some(HTCLOSE as _),
                    WindowControlArea::Max => Some(HTMAXBUTTON as _),
                    WindowControlArea::Min => Some(HTMINBUTTON as _),
                };
            }
        } else {
            drop(lock);
        }

        if !self.hide_title_bar {
            // If the OS draws the title bar, we don't need to handle hit test messages.
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

        if self.state.borrow().is_fullscreen() {
            return Some(HTCLIENT as _);
        }

        let dpi = unsafe { GetDpiForWindow(handle) };
        let frame_y = unsafe { GetSystemMetricsForDpi(SM_CYFRAME, dpi) };

        let mut cursor_point = POINT {
            x: lparam.signed_loword().into(),
            y: lparam.signed_hiword().into(),
        };
        unsafe { ScreenToClient(handle, &mut cursor_point).ok().log_err() };
        if !self.state.borrow().is_maximized() && cursor_point.y >= 0 && cursor_point.y <= frame_y {
            return Some(HTTOP as _);
        }

        Some(HTCLIENT as _)
    }

    fn handle_nc_mouse_move_msg(&self, handle: HWND, lparam: LPARAM) -> Option<isize> {
        self.start_tracking_mouse(handle, TME_LEAVE | TME_NONCLIENT);

        let mut lock = self.state.borrow_mut();
        let mut func = lock.callbacks.input.take()?;
        let scale_factor = lock.scale_factor;
        drop(lock);

        let mut cursor_point = POINT {
            x: lparam.signed_loword().into(),
            y: lparam.signed_hiword().into(),
        };
        unsafe { ScreenToClient(handle, &mut cursor_point).ok().log_err() };
        let input = PlatformInput::MouseMove(MouseMoveEvent {
            position: logical_point(cursor_point.x as f32, cursor_point.y as f32, scale_factor),
            pressed_button: None,
            modifiers: current_modifiers(),
        });
        let handled = !func(input).propagate;
        self.state.borrow_mut().callbacks.input = Some(func);

        if handled { Some(0) } else { None }
    }

    fn handle_nc_mouse_down_msg(
        &self,
        handle: HWND,
        button: MouseButton,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<isize> {
        let mut lock = self.state.borrow_mut();
        if let Some(mut func) = lock.callbacks.input.take() {
            let scale_factor = lock.scale_factor;
            let mut cursor_point = POINT {
                x: lparam.signed_loword().into(),
                y: lparam.signed_hiword().into(),
            };
            unsafe { ScreenToClient(handle, &mut cursor_point).ok().log_err() };
            let physical_point = point(DevicePixels(cursor_point.x), DevicePixels(cursor_point.y));
            let click_count = lock.click_state.update(button, physical_point);
            drop(lock);

            let input = PlatformInput::MouseDown(MouseDownEvent {
                button,
                position: logical_point(cursor_point.x as f32, cursor_point.y as f32, scale_factor),
                modifiers: current_modifiers(),
                click_count,
                first_mouse: false,
            });
            let result = func(input.clone());
            let handled = !result.propagate || result.default_prevented;
            self.state.borrow_mut().callbacks.input = Some(func);

            if handled {
                return Some(0);
            }
        } else {
            drop(lock);
        };

        // Since these are handled in handle_nc_mouse_up_msg we must prevent the default window proc
        if button == MouseButton::Left {
            match wparam.0 as u32 {
                HTMINBUTTON => self.state.borrow_mut().nc_button_pressed = Some(HTMINBUTTON),
                HTMAXBUTTON => self.state.borrow_mut().nc_button_pressed = Some(HTMAXBUTTON),
                HTCLOSE => self.state.borrow_mut().nc_button_pressed = Some(HTCLOSE),
                _ => return None,
            };
            Some(0)
        } else {
            None
        }
    }

    fn handle_nc_mouse_up_msg(
        &self,
        handle: HWND,
        button: MouseButton,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<isize> {
        let mut lock = self.state.borrow_mut();
        if let Some(mut func) = lock.callbacks.input.take() {
            let scale_factor = lock.scale_factor;
            drop(lock);

            let mut cursor_point = POINT {
                x: lparam.signed_loword().into(),
                y: lparam.signed_hiword().into(),
            };
            unsafe { ScreenToClient(handle, &mut cursor_point).ok().log_err() };
            let input = PlatformInput::MouseUp(MouseUpEvent {
                button,
                position: logical_point(cursor_point.x as f32, cursor_point.y as f32, scale_factor),
                modifiers: current_modifiers(),
                click_count: 1,
            });
            let handled = !func(input).propagate;
            self.state.borrow_mut().callbacks.input = Some(func);

            if handled {
                return Some(0);
            }
        } else {
            drop(lock);
        }

        let last_pressed = self.state.borrow_mut().nc_button_pressed.take();
        if button == MouseButton::Left
            && let Some(last_pressed) = last_pressed
        {
            let handled = match (wparam.0 as u32, last_pressed) {
                (HTMINBUTTON, HTMINBUTTON) => {
                    unsafe { ShowWindowAsync(handle, SW_MINIMIZE).ok().log_err() };
                    true
                }
                (HTMAXBUTTON, HTMAXBUTTON) => {
                    if self.state.borrow().is_maximized() {
                        unsafe { ShowWindowAsync(handle, SW_NORMAL).ok().log_err() };
                    } else {
                        unsafe { ShowWindowAsync(handle, SW_MAXIMIZE).ok().log_err() };
                    }
                    true
                }
                (HTCLOSE, HTCLOSE) => {
                    unsafe {
                        PostMessageW(Some(handle), WM_CLOSE, WPARAM::default(), LPARAM::default())
                            .log_err()
                    };
                    true
                }
                _ => false,
            };
            if handled {
                return Some(0);
            }
        }

        None
    }

    fn handle_cursor_changed(&self, lparam: LPARAM) -> Option<isize> {
        let mut state = self.state.borrow_mut();
        let had_cursor = state.current_cursor.is_some();

        state.current_cursor = if lparam.0 == 0 {
            None
        } else {
            Some(HCURSOR(lparam.0 as _))
        };

        if had_cursor != state.current_cursor.is_some() {
            unsafe { SetCursor(state.current_cursor) };
        }

        Some(0)
    }

    fn handle_set_cursor(&self, handle: HWND, lparam: LPARAM) -> Option<isize> {
        if unsafe { !IsWindowEnabled(handle).as_bool() }
            || matches!(
                lparam.loword() as u32,
                HTLEFT
                    | HTRIGHT
                    | HTTOP
                    | HTTOPLEFT
                    | HTTOPRIGHT
                    | HTBOTTOM
                    | HTBOTTOMLEFT
                    | HTBOTTOMRIGHT
            )
        {
            return None;
        }
        unsafe {
            SetCursor(self.state.borrow().current_cursor);
        };
        Some(1)
    }

    fn handle_system_settings_changed(
        &self,
        handle: HWND,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<isize> {
        if wparam.0 != 0 {
            let mut lock = self.state.borrow_mut();
            let display = lock.display;
            lock.system_settings.update(display, wparam.0);
            lock.click_state.system_update(wparam.0);
            lock.border_offset.update(handle).log_err();
        } else {
            self.handle_system_theme_changed(handle, lparam)?;
        };
        // Force to trigger WM_NCCALCSIZE event to ensure that we handle auto hide
        // taskbar correctly.
        notify_frame_changed(handle);

        Some(0)
    }

    fn handle_system_command(&self, wparam: WPARAM) -> Option<isize> {
        if wparam.0 == SC_KEYMENU as usize {
            let mut lock = self.state.borrow_mut();
            if lock.system_key_handled {
                lock.system_key_handled = false;
                return Some(0);
            }
        }
        None
    }

    fn handle_system_theme_changed(&self, handle: HWND, lparam: LPARAM) -> Option<isize> {
        // lParam is a pointer to a string that indicates the area containing the system parameter
        // that was changed.
        let parameter = PCWSTR::from_raw(lparam.0 as _);
        if unsafe { !parameter.is_null() && !parameter.is_empty() } {
            if let Some(parameter_string) = unsafe { parameter.to_string() }.log_err() {
                log::info!("System settings changed: {}", parameter_string);
                match parameter_string.as_str() {
                    "ImmersiveColorSet" => {
                        let new_appearance = system_appearance()
                            .context(
                                "unable to get system appearance when handling ImmersiveColorSet",
                            )
                            .log_err()?;
                        let mut lock = self.state.borrow_mut();
                        if new_appearance != lock.appearance {
                            lock.appearance = new_appearance;
                            let mut callback = lock.callbacks.appearance_changed.take()?;
                            drop(lock);
                            callback();
                            self.state.borrow_mut().callbacks.appearance_changed = Some(callback);
                            configure_dwm_dark_mode(handle, new_appearance);
                        }
                    }
                    _ => {}
                }
            }
        }
        Some(0)
    }

    fn handle_input_language_changed(&self, lparam: LPARAM) -> Option<isize> {
        let thread = self.main_thread_id_win32;
        let validation = self.validation_number;
        unsafe {
            PostThreadMessageW(thread, WM_INPUTLANGCHANGE, WPARAM(validation), lparam).log_err();
        }
        Some(0)
    }

    fn handle_device_change_msg(&self, handle: HWND, wparam: WPARAM) -> Option<isize> {
        if wparam.0 == DBT_DEVNODES_CHANGED as usize {
            // The reason for sending this message is to actually trigger a redraw of the window.
            unsafe {
                PostMessageW(
                    Some(handle),
                    WM_GPUI_FORCE_UPDATE_WINDOW,
                    WPARAM(0),
                    LPARAM(0),
                )
                .log_err();
            }
            // If the GPU device is lost, this redraw will take care of recreating the device context.
            // The WM_GPUI_FORCE_UPDATE_WINDOW message will take care of redrawing the window, after
            // the device context has been recreated.
            self.draw_window(handle, true)
        } else {
            // Other device change messages are not handled.
            None
        }
    }

    #[inline]
    fn draw_window(&self, handle: HWND, force_render: bool) -> Option<isize> {
        let mut request_frame = self.state.borrow_mut().callbacks.request_frame.take()?;
        request_frame(RequestFrameOptions {
            require_presentation: false,
            force_render,
        });
        self.state.borrow_mut().callbacks.request_frame = Some(request_frame);
        unsafe { ValidateRect(Some(handle), None).ok().log_err() };
        Some(0)
    }

    #[inline]
    fn parse_char_message(&self, wparam: WPARAM) -> Option<String> {
        let code_point = wparam.loword();
        let mut lock = self.state.borrow_mut();
        // https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-3/#G2630
        match code_point {
            0xD800..=0xDBFF => {
                // High surrogate, wait for low surrogate
                lock.pending_surrogate = Some(code_point);
                None
            }
            0xDC00..=0xDFFF => {
                if let Some(high_surrogate) = lock.pending_surrogate.take() {
                    // Low surrogate, combine with pending high surrogate
                    String::from_utf16(&[high_surrogate, code_point]).ok()
                } else {
                    // Invalid low surrogate without a preceding high surrogate
                    log::warn!(
                        "Received low surrogate without a preceding high surrogate: {code_point:x}"
                    );
                    None
                }
            }
            _ => {
                lock.pending_surrogate = None;
                char::from_u32(code_point as u32)
                    .filter(|c| !c.is_control())
                    .map(|c| c.to_string())
            }
        }
    }

    fn start_tracking_mouse(&self, handle: HWND, flags: TRACKMOUSEEVENT_FLAGS) {
        let mut lock = self.state.borrow_mut();
        if !lock.hovered {
            lock.hovered = true;
            unsafe {
                TrackMouseEvent(&mut TRACKMOUSEEVENT {
                    cbSize: std::mem::size_of::<TRACKMOUSEEVENT>() as u32,
                    dwFlags: flags,
                    hwndTrack: handle,
                    dwHoverTime: HOVER_DEFAULT,
                })
                .log_err()
            };
            if let Some(mut callback) = lock.callbacks.hovered_status_change.take() {
                drop(lock);
                callback(true);
                self.state.borrow_mut().callbacks.hovered_status_change = Some(callback);
            }
        }
    }

    fn with_input_handler<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut PlatformInputHandler) -> R,
    {
        let mut input_handler = self.state.borrow_mut().input_handler.take()?;
        let result = f(&mut input_handler);
        self.state.borrow_mut().input_handler = Some(input_handler);
        Some(result)
    }

    fn with_input_handler_and_scale_factor<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut PlatformInputHandler, f32) -> Option<R>,
    {
        let mut lock = self.state.borrow_mut();
        let mut input_handler = lock.input_handler.take()?;
        let scale_factor = lock.scale_factor;
        drop(lock);
        let result = f(&mut input_handler, scale_factor);
        self.state.borrow_mut().input_handler = Some(input_handler);
        result
    }
}

#[inline]
fn translate_message(handle: HWND, wparam: WPARAM, lparam: LPARAM) {
    let msg = MSG {
        hwnd: handle,
        message: WM_KEYDOWN,
        wParam: wparam,
        lParam: lparam,
        // It seems like leaving the following two parameters empty doesn't break key events, they still work as expected.
        // But if any bugs pop up after this PR, this is probably the place to look first.
        time: 0,
        pt: POINT::default(),
    };
    unsafe { TranslateMessage(&msg).ok().log_err() };
}

fn handle_key_event<F>(
    handle: HWND,
    wparam: WPARAM,
    lparam: LPARAM,
    state: &mut WindowsWindowState,
    f: F,
) -> Option<PlatformInput>
where
    F: FnOnce(Keystroke) -> PlatformInput,
{
    let virtual_key = VIRTUAL_KEY(wparam.loword());
    let mut modifiers = current_modifiers();

    match virtual_key {
        VK_SHIFT | VK_CONTROL | VK_MENU | VK_LWIN | VK_RWIN => {
            if state
                .last_reported_modifiers
                .is_some_and(|prev_modifiers| prev_modifiers == modifiers)
            {
                return None;
            }
            state.last_reported_modifiers = Some(modifiers);
            Some(PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                modifiers,
                capslock: current_capslock(),
            }))
        }
        VK_PACKET => {
            translate_message(handle, wparam, lparam);
            None
        }
        VK_CAPITAL => {
            let capslock = current_capslock();
            if state
                .last_reported_capslock
                .is_some_and(|prev_capslock| prev_capslock == capslock)
            {
                return None;
            }
            state.last_reported_capslock = Some(capslock);
            Some(PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                modifiers,
                capslock,
            }))
        }
        vkey => {
            let vkey = if vkey == VK_PROCESSKEY {
                VIRTUAL_KEY(unsafe { ImmGetVirtualKey(handle) } as u16)
            } else {
                vkey
            };
            let keystroke = parse_normal_key(vkey, lparam, modifiers)?;
            Some(f(keystroke))
        }
    }
}

fn parse_immutable(vkey: VIRTUAL_KEY) -> Option<String> {
    Some(
        match vkey {
            VK_SPACE => "space",
            VK_BACK => "backspace",
            VK_RETURN => "enter",
            VK_TAB => "tab",
            VK_UP => "up",
            VK_DOWN => "down",
            VK_RIGHT => "right",
            VK_LEFT => "left",
            VK_HOME => "home",
            VK_END => "end",
            VK_PRIOR => "pageup",
            VK_NEXT => "pagedown",
            VK_BROWSER_BACK => "back",
            VK_BROWSER_FORWARD => "forward",
            VK_ESCAPE => "escape",
            VK_INSERT => "insert",
            VK_DELETE => "delete",
            VK_APPS => "menu",
            VK_F1 => "f1",
            VK_F2 => "f2",
            VK_F3 => "f3",
            VK_F4 => "f4",
            VK_F5 => "f5",
            VK_F6 => "f6",
            VK_F7 => "f7",
            VK_F8 => "f8",
            VK_F9 => "f9",
            VK_F10 => "f10",
            VK_F11 => "f11",
            VK_F12 => "f12",
            VK_F13 => "f13",
            VK_F14 => "f14",
            VK_F15 => "f15",
            VK_F16 => "f16",
            VK_F17 => "f17",
            VK_F18 => "f18",
            VK_F19 => "f19",
            VK_F20 => "f20",
            VK_F21 => "f21",
            VK_F22 => "f22",
            VK_F23 => "f23",
            VK_F24 => "f24",
            _ => return None,
        }
        .to_string(),
    )
}

fn parse_normal_key(
    vkey: VIRTUAL_KEY,
    lparam: LPARAM,
    mut modifiers: Modifiers,
) -> Option<Keystroke> {
    let mut key_char = None;
    let key = parse_immutable(vkey).or_else(|| {
        let scan_code = lparam.hiword() & 0xFF;
        key_char = generate_key_char(
            vkey,
            scan_code as u32,
            modifiers.control,
            modifiers.shift,
            modifiers.alt,
        );
        get_keystroke_key(vkey, scan_code as u32, &mut modifiers)
    })?;
    Some(Keystroke {
        modifiers,
        key,
        key_char,
    })
}

fn parse_ime_composition_string(ctx: HIMC, comp_type: IME_COMPOSITION_STRING) -> Option<String> {
    unsafe {
        let string_len = ImmGetCompositionStringW(ctx, comp_type, None, 0);
        if string_len >= 0 {
            let mut buffer = vec![0u8; string_len as usize + 2];
            ImmGetCompositionStringW(
                ctx,
                comp_type,
                Some(buffer.as_mut_ptr() as _),
                string_len as _,
            );
            let wstring = std::slice::from_raw_parts::<u16>(
                buffer.as_mut_ptr().cast::<u16>(),
                string_len as usize / 2,
            );
            Some(String::from_utf16_lossy(wstring))
        } else {
            None
        }
    }
}

#[inline]
fn retrieve_composition_cursor_position(ctx: HIMC) -> usize {
    unsafe { ImmGetCompositionStringW(ctx, GCS_CURSORPOS, None, 0) as usize }
}

#[inline]
fn is_virtual_key_pressed(vkey: VIRTUAL_KEY) -> bool {
    unsafe { GetKeyState(vkey.0 as i32) < 0 }
}

#[inline]
pub(crate) fn current_modifiers() -> Modifiers {
    Modifiers {
        control: is_virtual_key_pressed(VK_CONTROL),
        alt: is_virtual_key_pressed(VK_MENU),
        shift: is_virtual_key_pressed(VK_SHIFT),
        platform: is_virtual_key_pressed(VK_LWIN) || is_virtual_key_pressed(VK_RWIN),
        function: false,
    }
}

#[inline]
pub(crate) fn current_capslock() -> Capslock {
    let on = unsafe { GetKeyState(VK_CAPITAL.0 as i32) & 1 } > 0;
    Capslock { on: on }
}

fn get_client_area_insets(
    handle: HWND,
    is_maximized: bool,
    windows_version: WindowsVersion,
) -> RECT {
    // For maximized windows, Windows outdents the window rect from the screen's client rect
    // by `frame_thickness` on each edge, meaning `insets` must contain `frame_thickness`
    // on all sides (including the top) to avoid the client area extending onto adjacent
    // monitors.
    //
    // For non-maximized windows, things become complicated:
    //
    // - On Windows 10
    // The top inset must be zero, since if there is any nonclient area, Windows will draw
    // a full native titlebar outside the client area. (This doesn't occur in the maximized
    // case.)
    //
    // - On Windows 11
    // The top inset is calculated using an empirical formula that I derived through various
    // tests. Without this, the top 1-2 rows of pixels in our window would be obscured.
    let dpi = unsafe { GetDpiForWindow(handle) };
    let frame_thickness = get_frame_thickness(dpi);
    let top_insets = if is_maximized {
        frame_thickness
    } else {
        match windows_version {
            WindowsVersion::Win10 => 0,
            WindowsVersion::Win11 => (dpi as f32 / USER_DEFAULT_SCREEN_DPI as f32).round() as i32,
        }
    };
    RECT {
        left: frame_thickness,
        top: top_insets,
        right: frame_thickness,
        bottom: frame_thickness,
    }
}

// there is some additional non-visible space when talking about window
// borders on Windows:
// - SM_CXSIZEFRAME: The resize handle.
// - SM_CXPADDEDBORDER: Additional border space that isn't part of the resize handle.
fn get_frame_thickness(dpi: u32) -> i32 {
    let resize_frame_thickness = unsafe { GetSystemMetricsForDpi(SM_CXSIZEFRAME, dpi) };
    let padding_thickness = unsafe { GetSystemMetricsForDpi(SM_CXPADDEDBORDER, dpi) };
    resize_frame_thickness + padding_thickness
}

fn notify_frame_changed(handle: HWND) {
    unsafe {
        SetWindowPos(
            handle,
            None,
            0,
            0,
            0,
            0,
            SWP_FRAMECHANGED
                | SWP_NOACTIVATE
                | SWP_NOCOPYBITS
                | SWP_NOMOVE
                | SWP_NOOWNERZORDER
                | SWP_NOREPOSITION
                | SWP_NOSENDCHANGING
                | SWP_NOSIZE
                | SWP_NOZORDER,
        )
        .log_err();
    }
}
