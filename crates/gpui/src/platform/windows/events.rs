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
pub(crate) const WM_GPUI_KEYBOARD_LAYOUT_CHANGED: u32 = WM_USER + 6;
pub(crate) const WM_GPUI_GPU_DEVICE_LOST: u32 = WM_USER + 7;
pub(crate) const WM_GPUI_KEYDOWN: u32 = WM_USER + 8;

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
            // eagerly activate the window, so calls to `active_window` will work correctly
            WM_MOUSEACTIVATE => {
                unsafe { SetActiveWindow(handle).ok() };
                None
            }
            WM_ACTIVATE => self.handle_activate_msg(wparam),
            WM_CREATE => self.handle_create_msg(handle),
            WM_MOVE => self.handle_move_msg(handle, lparam),
            WM_SIZE => self.handle_size_msg(wparam, lparam),
            WM_GETMINMAXINFO => self.handle_get_min_max_info_msg(lparam),
            WM_ENTERSIZEMOVE | WM_ENTERMENULOOP => self.handle_size_move_loop(handle),
            WM_EXITSIZEMOVE | WM_EXITMENULOOP => self.handle_size_move_loop_exit(handle),
            WM_TIMER => self.handle_timer_msg(handle, wparam),
            WM_NCCALCSIZE => self.handle_calc_client_size(handle, wparam, lparam),
            WM_DPICHANGED => self.handle_dpi_changed_msg(handle, wparam, lparam),
            WM_DISPLAYCHANGE => self.handle_display_change_msg(handle),
            WM_NCHITTEST => self.handle_hit_test_msg(handle, lparam),
            WM_PAINT => self.handle_paint_msg(handle),
            WM_CLOSE => self.handle_close_msg(),
            WM_DESTROY => self.handle_destroy_msg(handle),
            WM_MOUSEMOVE => self.handle_mouse_move_msg(handle, lparam, wparam),
            WM_MOUSELEAVE | WM_NCMOUSELEAVE => self.handle_mouse_leave_msg(),
            WM_NCMOUSEMOVE => self.handle_nc_mouse_move_msg(handle, lparam),
            // Treat double click as a second single click, since we track the double clicks ourselves.
            // If you don't interact with any elements, this will fall through to the windows default
            // behavior of toggling whether the window is maximized.
            WM_NCLBUTTONDBLCLK | WM_NCLBUTTONDOWN => {
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
            WM_SYSKEYUP => self.handle_syskeyup_msg(wparam, lparam),
            WM_KEYUP => self.handle_keyup_msg(wparam, lparam),
            WM_GPUI_KEYDOWN => self.handle_keydown_msg(wparam, lparam),
            WM_CHAR => self.handle_char_msg(wparam),
            WM_IME_STARTCOMPOSITION => self.handle_ime_position(handle),
            WM_IME_COMPOSITION => self.handle_ime_composition(handle, lparam),
            WM_SETCURSOR => self.handle_set_cursor(handle, lparam),
            WM_SETTINGCHANGE => self.handle_system_settings_changed(handle, wparam, lparam),
            WM_INPUTLANGCHANGE => self.handle_input_language_changed(),
            WM_SHOWWINDOW => self.handle_window_visibility_changed(handle, wparam),
            WM_GPUI_CURSOR_STYLE_CHANGED => self.handle_cursor_changed(lparam),
            WM_GPUI_FORCE_UPDATE_WINDOW => self.draw_window(handle, true),
            WM_GPUI_GPU_DEVICE_LOST => self.handle_device_lost(lparam),
            _ => None,
        };
        if let Some(n) = handled {
            LRESULT(n)
        } else {
            unsafe { DefWindowProcW(handle, msg, wparam, lparam) }
        }
    }

    fn handle_move_msg(&self, handle: HWND, lparam: LPARAM) -> Option<isize> {
        let origin = logical_point(
            lparam.signed_loword() as f32,
            lparam.signed_hiword() as f32,
            self.state.scale_factor.get(),
        );
        self.state.origin.set(origin);
        let size = self.state.logical_size.get();
        let center_x = origin.x.0 + size.width.0 / 2.;
        let center_y = origin.y.0 + size.height.0 / 2.;
        let monitor_bounds = self.state.display.get().bounds();
        if center_x < monitor_bounds.left().0
            || center_x > monitor_bounds.right().0
            || center_y < monitor_bounds.top().0
            || center_y > monitor_bounds.bottom().0
        {
            // center of the window may have moved to another monitor
            let monitor = unsafe { MonitorFromWindow(handle, MONITOR_DEFAULTTONULL) };
            // minimize the window can trigger this event too, in this case,
            // monitor is invalid, we do nothing.
            if !monitor.is_invalid() && self.state.display.get().handle != monitor {
                // we will get the same monitor if we only have one
                self.state
                    .display
                    .set(WindowsDisplay::new_with_handle(monitor).log_err()?);
            }
        }
        if let Some(mut callback) = self.state.callbacks.moved.take() {
            callback();
            self.state.callbacks.moved.set(Some(callback));
        }
        Some(0)
    }

    fn handle_get_min_max_info_msg(&self, lparam: LPARAM) -> Option<isize> {
        let min_size = self.state.min_size?;
        let scale_factor = self.state.scale_factor.get();
        let boarder_offset = &self.state.border_offset;

        unsafe {
            let minmax_info = &mut *(lparam.0 as *mut MINMAXINFO);
            minmax_info.ptMinTrackSize.x =
                min_size.width.scale(scale_factor).0 as i32 + boarder_offset.width_offset.get();
            minmax_info.ptMinTrackSize.y =
                min_size.height.scale(scale_factor).0 as i32 + boarder_offset.height_offset.get();
        }
        Some(0)
    }

    fn handle_size_msg(&self, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        // Don't resize the renderer when the window is minimized, but record that it was minimized so
        // that on restore the swap chain can be recreated via `update_drawable_size_even_if_unchanged`.
        if wparam.0 == SIZE_MINIMIZED as usize {
            self.state
                .restore_from_minimized
                .set(self.state.callbacks.request_frame.take());
            return Some(0);
        }

        let width = lparam.loword().max(1) as i32;
        let height = lparam.hiword().max(1) as i32;
        let new_size = size(DevicePixels(width), DevicePixels(height));

        let scale_factor = self.state.scale_factor.get();
        let mut should_resize_renderer = false;
        if let Some(restore_from_minimized) = self.state.restore_from_minimized.take() {
            self.state
                .callbacks
                .request_frame
                .set(Some(restore_from_minimized));
        } else {
            should_resize_renderer = true;
        }

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

        self.state.logical_size.set(new_logical_size);
        if should_resize_renderer
            && let Err(e) = self.state.renderer.borrow_mut().resize(device_size)
        {
            log::error!("Failed to resize renderer, invalidating devices: {}", e);
            self.state
                .invalidate_devices
                .store(true, std::sync::atomic::Ordering::Release);
        }
        if let Some(mut callback) = self.state.callbacks.resize.take() {
            callback(new_logical_size, scale_factor);
            self.state.callbacks.resize.set(Some(callback));
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
            let mut runnables = self.main_receiver.clone().try_iter();
            while let Some(Ok(runnable)) = runnables.next() {
                runnable.run_and_profile();
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
        let mut callback = self.state.callbacks.should_close.take()?;
        let should_close = callback();
        self.state.callbacks.should_close.set(Some(callback));
        if should_close { None } else { Some(0) }
    }

    fn handle_destroy_msg(&self, handle: HWND) -> Option<isize> {
        let callback = { self.state.callbacks.close.take() };
        // Re-enable parent window if this was a modal dialog
        if let Some(parent_hwnd) = self.parent_hwnd {
            unsafe {
                let _ = EnableWindow(parent_hwnd, true);
                let _ = SetForegroundWindow(parent_hwnd);
            }
        }

        if let Some(callback) = callback {
            callback();
        }
        unsafe {
            PostMessageW(
                Some(self.platform_window_handle),
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

        let Some(mut func) = self.state.callbacks.input.take() else {
            return Some(1);
        };
        let scale_factor = self.state.scale_factor.get();

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
        self.state.callbacks.input.set(Some(func));

        if handled { Some(0) } else { Some(1) }
    }

    fn handle_mouse_leave_msg(&self) -> Option<isize> {
        self.state.hovered.set(false);
        if let Some(mut callback) = self.state.callbacks.hovered_status_change.take() {
            callback(false);
            self.state
                .callbacks
                .hovered_status_change
                .set(Some(callback));
        }

        Some(0)
    }

    fn handle_syskeyup_msg(&self, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        let input = handle_key_event(wparam, lparam, &self.state, |keystroke, _| {
            PlatformInput::KeyUp(KeyUpEvent { keystroke })
        })?;
        let mut func = self.state.callbacks.input.take()?;

        func(input);
        self.state.callbacks.input.set(Some(func));

        // Always return 0 to indicate that the message was handled, so we could properly handle `ModifiersChanged` event.
        Some(0)
    }

    // It's a known bug that you can't trigger `ctrl-shift-0`. See:
    // https://superuser.com/questions/1455762/ctrl-shift-number-key-combination-has-stopped-working-for-a-few-numbers
    fn handle_keydown_msg(&self, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        let Some(input) = handle_key_event(
            wparam,
            lparam,
            &self.state,
            |keystroke, prefer_character_input| {
                PlatformInput::KeyDown(KeyDownEvent {
                    keystroke,
                    is_held: lparam.0 & (0x1 << 30) > 0,
                    prefer_character_input,
                })
            },
        ) else {
            return Some(1);
        };

        let Some(mut func) = self.state.callbacks.input.take() else {
            return Some(1);
        };

        let handled = !func(input).propagate;

        self.state.callbacks.input.set(Some(func));

        if handled { Some(0) } else { Some(1) }
    }

    fn handle_keyup_msg(&self, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        let Some(input) = handle_key_event(wparam, lparam, &self.state, |keystroke, _| {
            PlatformInput::KeyUp(KeyUpEvent { keystroke })
        }) else {
            return Some(1);
        };

        let Some(mut func) = self.state.callbacks.input.take() else {
            return Some(1);
        };

        let handled = !func(input).propagate;
        self.state.callbacks.input.set(Some(func));

        if handled { Some(0) } else { Some(1) }
    }

    fn handle_char_msg(&self, wparam: WPARAM) -> Option<isize> {
        let input = self.parse_char_message(wparam)?;
        self.with_input_handler(|input_handler| {
            input_handler.replace_text_in_range(None, &input);
        });

        Some(0)
    }

    fn handle_mouse_down_msg(
        &self,
        handle: HWND,
        button: MouseButton,
        lparam: LPARAM,
    ) -> Option<isize> {
        unsafe { SetCapture(handle) };

        let Some(mut func) = self.state.callbacks.input.take() else {
            return Some(1);
        };
        let x = lparam.signed_loword();
        let y = lparam.signed_hiword();
        let physical_point = point(DevicePixels(x as i32), DevicePixels(y as i32));
        let click_count = self.state.click_state.update(button, physical_point);
        let scale_factor = self.state.scale_factor.get();

        let input = PlatformInput::MouseDown(MouseDownEvent {
            button,
            position: logical_point(x as f32, y as f32, scale_factor),
            modifiers: current_modifiers(),
            click_count,
            first_mouse: false,
        });
        let handled = !func(input).propagate;
        self.state.callbacks.input.set(Some(func));

        if handled { Some(0) } else { Some(1) }
    }

    fn handle_mouse_up_msg(
        &self,
        _handle: HWND,
        button: MouseButton,
        lparam: LPARAM,
    ) -> Option<isize> {
        unsafe { ReleaseCapture().log_err() };

        let Some(mut func) = self.state.callbacks.input.take() else {
            return Some(1);
        };
        let x = lparam.signed_loword() as f32;
        let y = lparam.signed_hiword() as f32;
        let click_count = self.state.click_state.current_count.get();
        let scale_factor = self.state.scale_factor.get();

        let input = PlatformInput::MouseUp(MouseUpEvent {
            button,
            position: logical_point(x, y, scale_factor),
            modifiers: current_modifiers(),
            click_count,
        });
        let handled = !func(input).propagate;
        self.state.callbacks.input.set(Some(func));

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

        let Some(mut func) = self.state.callbacks.input.take() else {
            return Some(1);
        };
        let scale_factor = self.state.scale_factor.get();
        let wheel_scroll_amount = match modifiers.shift {
            true => self
                .system_settings()
                .mouse_wheel_settings
                .wheel_scroll_chars
                .get(),
            false => self
                .system_settings()
                .mouse_wheel_settings
                .wheel_scroll_lines
                .get(),
        };

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
        self.state.callbacks.input.set(Some(func));

        if handled { Some(0) } else { Some(1) }
    }

    fn handle_mouse_horizontal_wheel_msg(
        &self,
        handle: HWND,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<isize> {
        let Some(mut func) = self.state.callbacks.input.take() else {
            return Some(1);
        };
        let scale_factor = self.state.scale_factor.get();
        let wheel_scroll_chars = self
            .system_settings()
            .mouse_wheel_settings
            .wheel_scroll_chars
            .get();

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
        self.state.callbacks.input.set(Some(func));

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
        if let Some(caret_position) = self.retrieve_caret_position() {
            self.update_ime_position(handle, caret_position);
        }
        Some(0)
    }

    pub(crate) fn update_ime_position(&self, handle: HWND, caret_position: POINT) {
        unsafe {
            let ctx = ImmGetContext(handle);
            if ctx.is_invalid() {
                return;
            }

            let config = COMPOSITIONFORM {
                dwStyle: CFS_POINT,
                ptCurrentPos: caret_position,
                ..Default::default()
            };
            ImmSetCompositionWindow(ctx, &config).ok().log_err();
            let config = CANDIDATEFORM {
                dwStyle: CFS_CANDIDATEPOS,
                ptCurrentPos: caret_position,
                ..Default::default()
            };
            ImmSetCandidateWindow(ctx, &config).ok().log_err();
            ImmReleaseContext(handle, ctx).ok().log_err();
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
        if !self.hide_title_bar || self.state.is_fullscreen() || wparam.0 == 0 {
            return None;
        }

        let is_maximized = self.state.is_maximized();
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
        if is_maximized
            && let Some(taskbar_position) = self.system_settings().auto_hide_taskbar_position.get()
        {
            // For the auto-hide taskbar, adjust in by 1 pixel on taskbar edge,
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

        Some(0)
    }

    fn handle_activate_msg(self: &Rc<Self>, wparam: WPARAM) -> Option<isize> {
        let activated = wparam.loword() > 0;
        let this = self.clone();
        self.executor
            .spawn(async move {
                if let Some(mut func) = this.state.callbacks.active_status_change.take() {
                    func(activated);
                    this.state.callbacks.active_status_change.set(Some(func));
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

        let is_maximized = self.state.is_maximized();
        let new_scale_factor = new_dpi / USER_DEFAULT_SCREEN_DPI as f32;
        self.state.scale_factor.set(new_scale_factor);
        self.state.border_offset.update(handle).log_err();

        if is_maximized {
            // Get the monitor and its work area at the new DPI
            let monitor = unsafe { MonitorFromWindow(handle, MONITOR_DEFAULTTONEAREST) };
            let mut monitor_info: MONITORINFO = unsafe { std::mem::zeroed() };
            monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
            if unsafe { GetMonitorInfoW(monitor, &mut monitor_info) }.as_bool() {
                let work_area = monitor_info.rcWork;
                let width = work_area.right - work_area.left;
                let height = work_area.bottom - work_area.top;

                // Update the window size to match the new monitor work area
                // This will trigger WM_SIZE which will handle the size change
                unsafe {
                    SetWindowPos(
                        handle,
                        None,
                        work_area.left,
                        work_area.top,
                        width,
                        height,
                        SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED,
                    )
                    .context("unable to set maximized window position after dpi has changed")
                    .log_err();
                }

                // SetWindowPos may not send WM_SIZE for maximized windows in some cases,
                // so we manually update the size to ensure proper rendering
                let device_size = size(DevicePixels(width), DevicePixels(height));
                self.handle_size_change(device_size, new_scale_factor, true);
            }
        } else {
            // For non-maximized windows, use the suggested RECT from the system
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
        let previous_monitor = self.state.display.get();
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
        let new_display = WindowsDisplay::new_with_handle(new_monitor).log_err()?;
        self.state.display.set(new_display);
        Some(0)
    }

    fn handle_hit_test_msg(&self, handle: HWND, lparam: LPARAM) -> Option<isize> {
        if !self.is_movable || self.state.is_fullscreen() {
            return None;
        }

        let callback = self.state.callbacks.hit_test_window_control.take();
        let drag_area = if let Some(mut callback) = callback {
            let area = callback();
            self.state
                .callbacks
                .hit_test_window_control
                .set(Some(callback));
            if let Some(area) = area {
                match area {
                    WindowControlArea::Drag => Some(HTCAPTION as _),
                    WindowControlArea::Close => return Some(HTCLOSE as _),
                    WindowControlArea::Max => return Some(HTMAXBUTTON as _),
                    WindowControlArea::Min => return Some(HTMINBUTTON as _),
                }
            } else {
                None
            }
        } else {
            None
        };

        if !self.hide_title_bar {
            // If the OS draws the title bar, we don't need to handle hit test messages.
            return drag_area;
        }

        let dpi = unsafe { GetDpiForWindow(handle) };
        // We do not use the OS title bar, so the default `DefWindowProcW` will only register a 1px edge for resizes
        // We need to calculate the frame thickness ourselves and do the hit test manually.
        let frame_y = get_frame_thicknessx(dpi);
        let frame_x = get_frame_thicknessy(dpi);
        let mut cursor_point = POINT {
            x: lparam.signed_loword().into(),
            y: lparam.signed_hiword().into(),
        };

        unsafe { ScreenToClient(handle, &mut cursor_point).ok().log_err() };
        if !self.state.is_maximized() && 0 <= cursor_point.y && cursor_point.y <= frame_y {
            // x-axis actually goes from -frame_x to 0
            return Some(if cursor_point.x <= 0 {
                HTTOPLEFT
            } else {
                let mut rect = Default::default();
                unsafe { GetWindowRect(handle, &mut rect) }.log_err();
                // right and bottom bounds of RECT are exclusive, thus `-1`
                let right = rect.right - rect.left - 1;
                // the bounds include the padding frames, so accomodate for both of them
                if right - 2 * frame_x <= cursor_point.x {
                    HTTOPRIGHT
                } else {
                    HTTOP
                }
            } as _);
        }

        drag_area
    }

    fn handle_nc_mouse_move_msg(&self, handle: HWND, lparam: LPARAM) -> Option<isize> {
        self.start_tracking_mouse(handle, TME_LEAVE | TME_NONCLIENT);

        let mut func = self.state.callbacks.input.take()?;
        let scale_factor = self.state.scale_factor.get();

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
        self.state.callbacks.input.set(Some(func));

        if handled { Some(0) } else { None }
    }

    fn handle_nc_mouse_down_msg(
        &self,
        handle: HWND,
        button: MouseButton,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<isize> {
        if let Some(mut func) = self.state.callbacks.input.take() {
            let scale_factor = self.state.scale_factor.get();
            let mut cursor_point = POINT {
                x: lparam.signed_loword().into(),
                y: lparam.signed_hiword().into(),
            };
            unsafe { ScreenToClient(handle, &mut cursor_point).ok().log_err() };
            let physical_point = point(DevicePixels(cursor_point.x), DevicePixels(cursor_point.y));
            let click_count = self.state.click_state.update(button, physical_point);

            let input = PlatformInput::MouseDown(MouseDownEvent {
                button,
                position: logical_point(cursor_point.x as f32, cursor_point.y as f32, scale_factor),
                modifiers: current_modifiers(),
                click_count,
                first_mouse: false,
            });
            let result = func(input);
            let handled = !result.propagate || result.default_prevented;
            self.state.callbacks.input.set(Some(func));

            if handled {
                return Some(0);
            }
        } else {
        };

        // Since these are handled in handle_nc_mouse_up_msg we must prevent the default window proc
        if button == MouseButton::Left {
            match wparam.0 as u32 {
                HTMINBUTTON => self.state.nc_button_pressed.set(Some(HTMINBUTTON)),
                HTMAXBUTTON => self.state.nc_button_pressed.set(Some(HTMAXBUTTON)),
                HTCLOSE => self.state.nc_button_pressed.set(Some(HTCLOSE)),
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
        if let Some(mut func) = self.state.callbacks.input.take() {
            let scale_factor = self.state.scale_factor.get();

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
            self.state.callbacks.input.set(Some(func));

            if handled {
                return Some(0);
            }
        } else {
        }

        let last_pressed = self.state.nc_button_pressed.take();
        if button == MouseButton::Left
            && let Some(last_pressed) = last_pressed
        {
            let handled = match (wparam.0 as u32, last_pressed) {
                (HTMINBUTTON, HTMINBUTTON) => {
                    unsafe { ShowWindowAsync(handle, SW_MINIMIZE).ok().log_err() };
                    true
                }
                (HTMAXBUTTON, HTMAXBUTTON) => {
                    if self.state.is_maximized() {
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
        let had_cursor = self.state.current_cursor.get().is_some();

        self.state.current_cursor.set(if lparam.0 == 0 {
            None
        } else {
            Some(HCURSOR(lparam.0 as _))
        });

        if had_cursor != self.state.current_cursor.get().is_some() {
            unsafe { SetCursor(self.state.current_cursor.get()) };
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
            SetCursor(self.state.current_cursor.get());
        };
        Some(0)
    }

    fn handle_system_settings_changed(
        &self,
        handle: HWND,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<isize> {
        if wparam.0 != 0 {
            let display = self.state.display.get();
            self.state.click_state.system_update(wparam.0);
            self.state.border_offset.update(handle).log_err();
            // system settings may emit a window message which wants to take the refcell self.state, so drop it

            self.system_settings().update(display, wparam.0);
        } else {
            self.handle_system_theme_changed(handle, lparam)?;
        };
        // Force to trigger WM_NCCALCSIZE event to ensure that we handle auto hide
        // taskbar correctly.
        notify_frame_changed(handle);

        Some(0)
    }

    fn handle_system_theme_changed(&self, handle: HWND, lparam: LPARAM) -> Option<isize> {
        // lParam is a pointer to a string that indicates the area containing the system parameter
        // that was changed.
        let parameter = PCWSTR::from_raw(lparam.0 as _);
        if unsafe { !parameter.is_null() && !parameter.is_empty() }
            && let Some(parameter_string) = unsafe { parameter.to_string() }.log_err()
        {
            log::info!("System settings changed: {}", parameter_string);
            if parameter_string.as_str() == "ImmersiveColorSet" {
                let new_appearance = system_appearance()
                    .context("unable to get system appearance when handling ImmersiveColorSet")
                    .log_err()?;

                if new_appearance != self.state.appearance.get() {
                    self.state.appearance.set(new_appearance);
                    let mut callback = self.state.callbacks.appearance_changed.take()?;

                    callback();
                    self.state.callbacks.appearance_changed.set(Some(callback));
                    configure_dwm_dark_mode(handle, new_appearance);
                }
            }
        }
        Some(0)
    }

    fn handle_input_language_changed(&self) -> Option<isize> {
        unsafe {
            PostMessageW(
                Some(self.platform_window_handle),
                WM_GPUI_KEYBOARD_LAYOUT_CHANGED,
                WPARAM(self.validation_number),
                LPARAM(0),
            )
            .log_err();
        }
        Some(0)
    }

    fn handle_window_visibility_changed(&self, handle: HWND, wparam: WPARAM) -> Option<isize> {
        if wparam.0 == 1 {
            self.draw_window(handle, false);
        }
        None
    }

    fn handle_device_lost(&self, lparam: LPARAM) -> Option<isize> {
        let devices = lparam.0 as *const DirectXDevices;
        let devices = unsafe { &*devices };
        if let Err(err) = self
            .state
            .renderer
            .borrow_mut()
            .handle_device_lost(&devices)
        {
            panic!("Device lost: {err}");
        }
        Some(0)
    }

    #[inline]
    fn draw_window(&self, handle: HWND, force_render: bool) -> Option<isize> {
        let mut request_frame = self.state.callbacks.request_frame.take()?;

        // we are instructing gpui to force render a frame, this will
        // re-populate all the gpu textures for us so we can resume drawing in
        // case we disabled drawing earlier due to a device loss
        self.state.renderer.borrow_mut().mark_drawable();
        request_frame(RequestFrameOptions {
            require_presentation: false,
            force_render,
        });

        self.state.callbacks.request_frame.set(Some(request_frame));
        unsafe { ValidateRect(Some(handle), None).ok().log_err() };

        Some(0)
    }

    #[inline]
    fn parse_char_message(&self, wparam: WPARAM) -> Option<String> {
        let code_point = wparam.loword();

        // https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-3/#G2630
        match code_point {
            0xD800..=0xDBFF => {
                // High surrogate, wait for low surrogate
                self.state.pending_surrogate.set(Some(code_point));
                None
            }
            0xDC00..=0xDFFF => {
                if let Some(high_surrogate) = self.state.pending_surrogate.take() {
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
                self.state.pending_surrogate.set(None);
                char::from_u32(code_point as u32)
                    .filter(|c| !c.is_control())
                    .map(|c| c.to_string())
            }
        }
    }

    fn start_tracking_mouse(&self, handle: HWND, flags: TRACKMOUSEEVENT_FLAGS) {
        if !self.state.hovered.get() {
            self.state.hovered.set(true);
            unsafe {
                TrackMouseEvent(&mut TRACKMOUSEEVENT {
                    cbSize: std::mem::size_of::<TRACKMOUSEEVENT>() as u32,
                    dwFlags: flags,
                    hwndTrack: handle,
                    dwHoverTime: HOVER_DEFAULT,
                })
                .log_err()
            };
            if let Some(mut callback) = self.state.callbacks.hovered_status_change.take() {
                callback(true);
                self.state
                    .callbacks
                    .hovered_status_change
                    .set(Some(callback));
            }
        }
    }

    fn with_input_handler<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut PlatformInputHandler) -> R,
    {
        let mut input_handler = self.state.input_handler.take()?;
        let result = f(&mut input_handler);
        self.state.input_handler.set(Some(input_handler));
        Some(result)
    }

    fn with_input_handler_and_scale_factor<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut PlatformInputHandler, f32) -> Option<R>,
    {
        let mut input_handler = self.state.input_handler.take()?;
        let scale_factor = self.state.scale_factor.get();

        let result = f(&mut input_handler, scale_factor);
        self.state.input_handler.set(Some(input_handler));
        result
    }
}

fn handle_key_event<F>(
    wparam: WPARAM,
    lparam: LPARAM,
    state: &WindowsWindowState,
    f: F,
) -> Option<PlatformInput>
where
    F: FnOnce(Keystroke, bool) -> PlatformInput,
{
    let virtual_key = VIRTUAL_KEY(wparam.loword());
    let modifiers = current_modifiers();

    match virtual_key {
        VK_SHIFT | VK_CONTROL | VK_MENU | VK_LMENU | VK_RMENU | VK_LWIN | VK_RWIN => {
            if state
                .last_reported_modifiers
                .get()
                .is_some_and(|prev_modifiers| prev_modifiers == modifiers)
            {
                return None;
            }
            state.last_reported_modifiers.set(Some(modifiers));
            Some(PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                modifiers,
                capslock: current_capslock(),
            }))
        }
        VK_PACKET => None,
        VK_CAPITAL => {
            let capslock = current_capslock();
            if state
                .last_reported_capslock
                .get()
                .is_some_and(|prev_capslock| prev_capslock == capslock)
            {
                return None;
            }
            state.last_reported_capslock.set(Some(capslock));
            Some(PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                modifiers,
                capslock,
            }))
        }
        vkey => {
            let keystroke = parse_normal_key(vkey, lparam, modifiers)?;
            Some(f(keystroke.0, keystroke.1))
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
) -> Option<(Keystroke, bool)> {
    let (key_char, prefer_character_input) = process_key(vkey, lparam.hiword());

    let key = parse_immutable(vkey).or_else(|| {
        let scan_code = lparam.hiword() & 0xFF;
        get_keystroke_key(vkey, scan_code as u32, &mut modifiers)
    })?;

    Some((
        Keystroke {
            modifiers,
            key,
            key_char,
        },
        prefer_character_input,
    ))
}

fn process_key(vkey: VIRTUAL_KEY, scan_code: u16) -> (Option<String>, bool) {
    let mut keyboard_state = [0u8; 256];
    unsafe {
        if GetKeyboardState(&mut keyboard_state).is_err() {
            return (None, false);
        }
    }

    let mut buffer_c = [0u16; 8];
    let result_c = unsafe {
        ToUnicode(
            vkey.0 as u32,
            scan_code as u32,
            Some(&keyboard_state),
            &mut buffer_c,
            0x4,
        )
    };

    if result_c == 0 {
        return (None, false);
    }

    let c = &buffer_c[..result_c.unsigned_abs() as usize];
    let key_char = String::from_utf16(c)
        .ok()
        .filter(|s| !s.is_empty() && !s.chars().next().unwrap().is_control());

    if result_c < 0 {
        return (key_char, true);
    }

    if key_char.is_none() {
        return (None, false);
    }

    // Workaround for some bug that makes the compiler think keyboard_state is still zeroed out
    let keyboard_state = std::hint::black_box(keyboard_state);
    let ctrl_down = (keyboard_state[VK_CONTROL.0 as usize] & 0x80) != 0;
    let alt_down = (keyboard_state[VK_MENU.0 as usize] & 0x80) != 0;
    let win_down = (keyboard_state[VK_LWIN.0 as usize] & 0x80) != 0
        || (keyboard_state[VK_RWIN.0 as usize] & 0x80) != 0;

    let has_modifiers = ctrl_down || alt_down || win_down;
    if !has_modifiers {
        return (key_char, false);
    }

    let mut state_no_modifiers = keyboard_state;
    state_no_modifiers[VK_CONTROL.0 as usize] = 0;
    state_no_modifiers[VK_LCONTROL.0 as usize] = 0;
    state_no_modifiers[VK_RCONTROL.0 as usize] = 0;
    state_no_modifiers[VK_MENU.0 as usize] = 0;
    state_no_modifiers[VK_LMENU.0 as usize] = 0;
    state_no_modifiers[VK_RMENU.0 as usize] = 0;
    state_no_modifiers[VK_LWIN.0 as usize] = 0;
    state_no_modifiers[VK_RWIN.0 as usize] = 0;

    let mut buffer_c_no_modifiers = [0u16; 8];
    let result_c_no_modifiers = unsafe {
        ToUnicode(
            vkey.0 as u32,
            scan_code as u32,
            Some(&state_no_modifiers),
            &mut buffer_c_no_modifiers,
            0x4,
        )
    };

    let c_no_modifiers = &buffer_c_no_modifiers[..result_c_no_modifiers.unsigned_abs() as usize];
    (
        key_char,
        result_c != result_c_no_modifiers || c != c_no_modifiers,
    )
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
    Capslock { on }
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
    let frame_thickness = get_frame_thicknessx(dpi);
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
fn get_frame_thicknessx(dpi: u32) -> i32 {
    let resize_frame_thickness = unsafe { GetSystemMetricsForDpi(SM_CXSIZEFRAME, dpi) };
    let padding_thickness = unsafe { GetSystemMetricsForDpi(SM_CXPADDEDBORDER, dpi) };
    resize_frame_thickness + padding_thickness
}

fn get_frame_thicknessy(dpi: u32) -> i32 {
    let resize_frame_thickness = unsafe { GetSystemMetricsForDpi(SM_CYSIZEFRAME, dpi) };
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
