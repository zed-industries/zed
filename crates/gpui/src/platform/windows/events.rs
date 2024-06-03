use std::rc::Rc;

use ::util::ResultExt;
use anyhow::Context;
use windows::Win32::{
    Foundation::*,
    Graphics::Gdi::*,
    System::SystemServices::*,
    UI::{
        HiDpi::*,
        Input::{Ime::*, KeyboardAndMouse::*},
        WindowsAndMessaging::*,
    },
};

use crate::*;

pub(crate) const CURSOR_STYLE_CHANGED: u32 = WM_USER + 1;
pub(crate) const CLOSE_ONE_WINDOW: u32 = WM_USER + 2;

const SIZE_MOVE_LOOP_TIMER_ID: usize = 1;

pub(crate) fn handle_msg(
    handle: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> LRESULT {
    let handled = match msg {
        WM_ACTIVATE => handle_activate_msg(handle, wparam, state_ptr),
        WM_CREATE => handle_create_msg(handle, state_ptr),
        WM_MOVE => handle_move_msg(handle, lparam, state_ptr),
        WM_SIZE => handle_size_msg(lparam, state_ptr),
        WM_ENTERSIZEMOVE | WM_ENTERMENULOOP => handle_size_move_loop(handle),
        WM_EXITSIZEMOVE | WM_EXITMENULOOP => handle_size_move_loop_exit(handle),
        WM_TIMER => handle_timer_msg(handle, wparam, state_ptr),
        WM_NCCALCSIZE => handle_calc_client_size(handle, wparam, lparam, state_ptr),
        WM_DPICHANGED => handle_dpi_changed_msg(handle, wparam, lparam, state_ptr),
        WM_DISPLAYCHANGE => handle_display_change_msg(handle, state_ptr),
        WM_NCHITTEST => handle_hit_test_msg(handle, msg, wparam, lparam, state_ptr),
        WM_PAINT => handle_paint_msg(handle, state_ptr),
        WM_CLOSE => handle_close_msg(state_ptr),
        WM_DESTROY => handle_destroy_msg(handle, state_ptr),
        WM_MOUSEMOVE => handle_mouse_move_msg(lparam, wparam, state_ptr),
        WM_NCMOUSEMOVE => handle_nc_mouse_move_msg(handle, lparam, state_ptr),
        WM_NCLBUTTONDOWN => {
            handle_nc_mouse_down_msg(handle, MouseButton::Left, wparam, lparam, state_ptr)
        }
        WM_NCRBUTTONDOWN => {
            handle_nc_mouse_down_msg(handle, MouseButton::Right, wparam, lparam, state_ptr)
        }
        WM_NCMBUTTONDOWN => {
            handle_nc_mouse_down_msg(handle, MouseButton::Middle, wparam, lparam, state_ptr)
        }
        WM_NCLBUTTONUP => {
            handle_nc_mouse_up_msg(handle, MouseButton::Left, wparam, lparam, state_ptr)
        }
        WM_NCRBUTTONUP => {
            handle_nc_mouse_up_msg(handle, MouseButton::Right, wparam, lparam, state_ptr)
        }
        WM_NCMBUTTONUP => {
            handle_nc_mouse_up_msg(handle, MouseButton::Middle, wparam, lparam, state_ptr)
        }
        WM_LBUTTONDOWN => handle_mouse_down_msg(MouseButton::Left, lparam, state_ptr),
        WM_RBUTTONDOWN => handle_mouse_down_msg(MouseButton::Right, lparam, state_ptr),
        WM_MBUTTONDOWN => handle_mouse_down_msg(MouseButton::Middle, lparam, state_ptr),
        WM_XBUTTONDOWN => handle_xbutton_msg(wparam, lparam, handle_mouse_down_msg, state_ptr),
        WM_LBUTTONUP => handle_mouse_up_msg(MouseButton::Left, lparam, state_ptr),
        WM_RBUTTONUP => handle_mouse_up_msg(MouseButton::Right, lparam, state_ptr),
        WM_MBUTTONUP => handle_mouse_up_msg(MouseButton::Middle, lparam, state_ptr),
        WM_XBUTTONUP => handle_xbutton_msg(wparam, lparam, handle_mouse_up_msg, state_ptr),
        WM_MOUSEWHEEL => handle_mouse_wheel_msg(handle, wparam, lparam, state_ptr),
        WM_MOUSEHWHEEL => handle_mouse_horizontal_wheel_msg(handle, wparam, lparam, state_ptr),
        WM_SYSKEYDOWN => handle_syskeydown_msg(wparam, lparam, state_ptr),
        WM_SYSKEYUP => handle_syskeyup_msg(wparam, state_ptr),
        WM_KEYDOWN => handle_keydown_msg(wparam, lparam, state_ptr),
        WM_KEYUP => handle_keyup_msg(wparam, state_ptr),
        WM_CHAR => handle_char_msg(wparam, lparam, state_ptr),
        WM_IME_STARTCOMPOSITION => handle_ime_position(handle, state_ptr),
        WM_IME_COMPOSITION => handle_ime_composition(handle, lparam, state_ptr),
        WM_SETCURSOR => handle_set_cursor(lparam, state_ptr),
        WM_SETTINGCHANGE => handle_system_settings_changed(state_ptr),
        CURSOR_STYLE_CHANGED => handle_cursor_changed(lparam, state_ptr),
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
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    let x = lparam.signed_loword() as i32;
    let y = lparam.signed_hiword() as i32;
    let mut lock = state_ptr.state.borrow_mut();
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
        state_ptr.state.borrow_mut().callbacks.moved = Some(callback);
    }
    Some(0)
}

fn handle_size_msg(lparam: LPARAM, state_ptr: Rc<WindowsWindowStatePtr>) -> Option<isize> {
    let width = lparam.loword().max(1) as i32;
    let height = lparam.hiword().max(1) as i32;
    let new_physical_size = size(width.into(), height.into());
    let mut lock = state_ptr.state.borrow_mut();
    let scale_factor = lock.scale_factor;
    lock.physical_size = new_physical_size;
    lock.renderer.update_drawable_size(Size {
        width: width as f64,
        height: height as f64,
    });
    if let Some(mut callback) = lock.callbacks.resize.take() {
        drop(lock);
        let logical_size = logical_size(new_physical_size, scale_factor);
        callback(logical_size, scale_factor);
        state_ptr.state.borrow_mut().callbacks.resize = Some(callback);
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
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    if wparam.0 == SIZE_MOVE_LOOP_TIMER_ID {
        handle_paint_msg(handle, state_ptr)
    } else {
        None
    }
}

fn handle_paint_msg(handle: HWND, state_ptr: Rc<WindowsWindowStatePtr>) -> Option<isize> {
    let mut lock = state_ptr.state.borrow_mut();
    if let Some(mut request_frame) = lock.callbacks.request_frame.take() {
        drop(lock);
        request_frame();
        state_ptr.state.borrow_mut().callbacks.request_frame = Some(request_frame);
    }
    unsafe { ValidateRect(handle, None).ok().log_err() };
    Some(0)
}

fn handle_close_msg(state_ptr: Rc<WindowsWindowStatePtr>) -> Option<isize> {
    let mut lock = state_ptr.state.borrow_mut();
    if let Some(mut callback) = lock.callbacks.should_close.take() {
        drop(lock);
        let should_close = callback();
        state_ptr.state.borrow_mut().callbacks.should_close = Some(callback);
        if should_close {
            None
        } else {
            Some(0)
        }
    } else {
        None
    }
}

fn handle_destroy_msg(handle: HWND, state_ptr: Rc<WindowsWindowStatePtr>) -> Option<isize> {
    let callback = {
        let mut lock = state_ptr.state.borrow_mut();
        lock.callbacks.close.take()
    };
    if let Some(callback) = callback {
        callback();
    }
    unsafe {
        PostMessageW(None, CLOSE_ONE_WINDOW, None, LPARAM(handle.0)).log_err();
    }
    Some(0)
}

fn handle_mouse_move_msg(
    lparam: LPARAM,
    wparam: WPARAM,
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    let mut lock = state_ptr.state.borrow_mut();
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
        state_ptr.state.borrow_mut().callbacks.input = Some(callback);
        return result;
    }
    Some(1)
}

fn handle_syskeydown_msg(
    wparam: WPARAM,
    lparam: LPARAM,
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    // we need to call `DefWindowProcW`, or we will lose the system-wide `Alt+F4`, `Alt+{other keys}`
    // shortcuts.
    let Some(keystroke) = parse_syskeydown_msg_keystroke(wparam) else {
        return None;
    };
    let mut lock = state_ptr.state.borrow_mut();
    let Some(mut func) = lock.callbacks.input.take() else {
        return None;
    };
    drop(lock);
    let event = KeyDownEvent {
        keystroke,
        is_held: lparam.0 & (0x1 << 30) > 0,
    };
    let result = if func(PlatformInput::KeyDown(event)).default_prevented {
        Some(0)
    } else {
        None
    };
    state_ptr.state.borrow_mut().callbacks.input = Some(func);

    result
}

fn handle_syskeyup_msg(wparam: WPARAM, state_ptr: Rc<WindowsWindowStatePtr>) -> Option<isize> {
    // we need to call `DefWindowProcW`, or we will lose the system-wide `Alt+F4`, `Alt+{other keys}`
    // shortcuts.
    let Some(keystroke) = parse_syskeydown_msg_keystroke(wparam) else {
        return None;
    };
    let mut lock = state_ptr.state.borrow_mut();
    let Some(mut func) = lock.callbacks.input.take() else {
        return None;
    };
    drop(lock);
    let event = KeyUpEvent { keystroke };
    let result = if func(PlatformInput::KeyUp(event)).default_prevented {
        Some(0)
    } else {
        Some(1)
    };
    state_ptr.state.borrow_mut().callbacks.input = Some(func);

    result
}

fn handle_keydown_msg(
    wparam: WPARAM,
    lparam: LPARAM,
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    let Some(keystroke_or_modifier) = parse_keydown_msg_keystroke(wparam) else {
        return Some(1);
    };
    let mut lock = state_ptr.state.borrow_mut();
    let Some(mut func) = lock.callbacks.input.take() else {
        return Some(1);
    };
    drop(lock);

    let event = match keystroke_or_modifier {
        KeystrokeOrModifier::Keystroke(keystroke) => PlatformInput::KeyDown(KeyDownEvent {
            keystroke,
            is_held: lparam.0 & (0x1 << 30) > 0,
        }),
        KeystrokeOrModifier::Modifier(modifiers) => {
            PlatformInput::ModifiersChanged(ModifiersChangedEvent { modifiers })
        }
    };

    let result = if func(event).default_prevented {
        Some(0)
    } else {
        Some(1)
    };
    state_ptr.state.borrow_mut().callbacks.input = Some(func);

    result
}

fn handle_keyup_msg(wparam: WPARAM, state_ptr: Rc<WindowsWindowStatePtr>) -> Option<isize> {
    let Some(keystroke_or_modifier) = parse_keydown_msg_keystroke(wparam) else {
        return Some(1);
    };
    let mut lock = state_ptr.state.borrow_mut();
    let Some(mut func) = lock.callbacks.input.take() else {
        return Some(1);
    };
    drop(lock);

    let event = match keystroke_or_modifier {
        KeystrokeOrModifier::Keystroke(keystroke) => PlatformInput::KeyUp(KeyUpEvent { keystroke }),
        KeystrokeOrModifier::Modifier(modifiers) => {
            PlatformInput::ModifiersChanged(ModifiersChangedEvent { modifiers })
        }
    };

    let result = if func(event).default_prevented {
        Some(0)
    } else {
        Some(1)
    };
    state_ptr.state.borrow_mut().callbacks.input = Some(func);

    result
}

fn handle_char_msg(
    wparam: WPARAM,
    lparam: LPARAM,
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    let Some(keystroke) = parse_char_msg_keystroke(wparam) else {
        return Some(1);
    };
    let mut lock = state_ptr.state.borrow_mut();
    let Some(mut func) = lock.callbacks.input.take() else {
        return Some(1);
    };
    drop(lock);
    let ime_key = keystroke.ime_key.clone();
    let event = KeyDownEvent {
        keystroke,
        is_held: lparam.0 & (0x1 << 30) > 0,
    };

    let dispatch_event_result = func(PlatformInput::KeyDown(event));
    let mut lock = state_ptr.state.borrow_mut();
    lock.callbacks.input = Some(func);
    if dispatch_event_result.default_prevented || !dispatch_event_result.propagate {
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
    state_ptr.state.borrow_mut().input_handler = Some(input_handler);

    Some(0)
}

fn handle_mouse_down_msg(
    button: MouseButton,
    lparam: LPARAM,
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    let mut lock = state_ptr.state.borrow_mut();
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
        state_ptr.state.borrow_mut().callbacks.input = Some(callback);

        result
    } else {
        Some(1)
    }
}

fn handle_mouse_up_msg(
    button: MouseButton,
    lparam: LPARAM,
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    let mut lock = state_ptr.state.borrow_mut();
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
        state_ptr.state.borrow_mut().callbacks.input = Some(callback);

        result
    } else {
        Some(1)
    }
}

fn handle_xbutton_msg(
    wparam: WPARAM,
    lparam: LPARAM,
    handler: impl Fn(MouseButton, LPARAM, Rc<WindowsWindowStatePtr>) -> Option<isize>,
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    let nav_dir = match wparam.hiword() {
        XBUTTON1 => NavigationDirection::Back,
        XBUTTON2 => NavigationDirection::Forward,
        _ => return Some(1),
    };
    handler(MouseButton::Navigate(nav_dir), lparam, state_ptr)
}

fn handle_mouse_wheel_msg(
    handle: HWND,
    wparam: WPARAM,
    lparam: LPARAM,
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    let mut lock = state_ptr.state.borrow_mut();
    if let Some(mut callback) = lock.callbacks.input.take() {
        let scale_factor = lock.scale_factor;
        let wheel_scroll_lines = lock.system_settings.mouse_wheel_settings.wheel_scroll_lines;
        drop(lock);
        let wheel_distance =
            (wparam.signed_hiword() as f32 / WHEEL_DELTA as f32) * wheel_scroll_lines as f32;
        let mut cursor_point = POINT {
            x: lparam.signed_loword().into(),
            y: lparam.signed_hiword().into(),
        };
        unsafe { ScreenToClient(handle, &mut cursor_point).ok().log_err() };
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
        state_ptr.state.borrow_mut().callbacks.input = Some(callback);

        result
    } else {
        Some(1)
    }
}

fn handle_mouse_horizontal_wheel_msg(
    handle: HWND,
    wparam: WPARAM,
    lparam: LPARAM,
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    let mut lock = state_ptr.state.borrow_mut();
    if let Some(mut callback) = lock.callbacks.input.take() {
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
        state_ptr.state.borrow_mut().callbacks.input = Some(callback);

        result
    } else {
        Some(1)
    }
}

fn handle_ime_position(handle: HWND, state_ptr: Rc<WindowsWindowStatePtr>) -> Option<isize> {
    unsafe {
        let mut lock = state_ptr.state.borrow_mut();
        let ctx = ImmGetContext(handle);
        let Some(mut input_handler) = lock.input_handler.take() else {
            return Some(1);
        };
        let scale_factor = lock.scale_factor;
        drop(lock);

        let Some(caret_range) = input_handler.selected_text_range() else {
            state_ptr.state.borrow_mut().input_handler = Some(input_handler);
            return Some(0);
        };
        let caret_position = input_handler.bounds_for_range(caret_range).unwrap();
        state_ptr.state.borrow_mut().input_handler = Some(input_handler);
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
        ImmSetCandidateWindow(ctx, &config as _).ok().log_err();
        ImmReleaseContext(handle, ctx).ok().log_err();
        Some(0)
    }
}

fn handle_ime_composition(
    handle: HWND,
    lparam: LPARAM,
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    let mut ime_input = None;
    if lparam.0 as u32 & GCS_COMPSTR.0 > 0 {
        let Some((string, string_len)) = parse_ime_compostion_string(handle) else {
            return None;
        };
        let mut lock = state_ptr.state.borrow_mut();
        let Some(mut input_handler) = lock.input_handler.take() else {
            return None;
        };
        drop(lock);
        input_handler.replace_and_mark_text_in_range(None, string.as_str(), Some(0..string_len));
        state_ptr.state.borrow_mut().input_handler = Some(input_handler);
        ime_input = Some(string);
    }
    if lparam.0 as u32 & GCS_CURSORPOS.0 > 0 {
        let Some(ref comp_string) = ime_input else {
            return None;
        };
        let caret_pos = retrieve_composition_cursor_position(handle);
        let mut lock = state_ptr.state.borrow_mut();
        let Some(mut input_handler) = lock.input_handler.take() else {
            return None;
        };
        drop(lock);
        input_handler.replace_and_mark_text_in_range(None, comp_string, Some(0..caret_pos));
        state_ptr.state.borrow_mut().input_handler = Some(input_handler);
    }
    if lparam.0 as u32 & GCS_RESULTSTR.0 > 0 {
        let Some(comp_result) = parse_ime_compostion_result(handle) else {
            return None;
        };
        let mut lock = state_ptr.state.borrow_mut();
        let Some(mut input_handler) = lock.input_handler.take() else {
            return Some(1);
        };
        drop(lock);
        input_handler.replace_text_in_range(None, &comp_result);
        state_ptr.state.borrow_mut().input_handler = Some(input_handler);
        return Some(0);
    }
    // currently, we don't care other stuff
    None
}

/// SEE: https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize
fn handle_calc_client_size(
    handle: HWND,
    wparam: WPARAM,
    lparam: LPARAM,
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    if !state_ptr.hide_title_bar || state_ptr.state.borrow().is_fullscreen() {
        return None;
    }

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
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    let activated = wparam.loword() > 0;
    if state_ptr.hide_title_bar {
        if let Some(titlebar_rect) = state_ptr.state.borrow().get_titlebar_rect().log_err() {
            unsafe {
                InvalidateRect(handle, Some(&titlebar_rect), FALSE)
                    .ok()
                    .log_err()
            };
        }
    }
    let this = state_ptr.clone();
    state_ptr
        .executor
        .spawn(async move {
            let mut lock = this.state.borrow_mut();
            if let Some(mut cb) = lock.callbacks.active_status_change.take() {
                drop(lock);
                cb(activated);
                this.state.borrow_mut().callbacks.active_status_change = Some(cb);
            }
        })
        .detach();

    None
}

fn handle_create_msg(handle: HWND, state_ptr: Rc<WindowsWindowStatePtr>) -> Option<isize> {
    let mut size_rect = RECT::default();
    unsafe { GetWindowRect(handle, &mut size_rect).log_err() };

    let width = size_rect.right - size_rect.left;
    let height = size_rect.bottom - size_rect.top;

    if state_ptr.hide_title_bar {
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
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    let new_dpi = wparam.loword() as f32;
    state_ptr.state.borrow_mut().scale_factor = new_dpi / USER_DEFAULT_SCREEN_DPI as f32;

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
fn handle_display_change_msg(handle: HWND, state_ptr: Rc<WindowsWindowStatePtr>) -> Option<isize> {
    // NOTE:
    // Even the `lParam` holds the resolution of the screen, we just ignore it.
    // Because WM_DPICHANGED, WM_MOVE, WM_SIEZ will come first, window reposition and resize
    // are handled there.
    // So we only care about if monitor is disconnected.
    let previous_monitor = state_ptr.as_ref().state.borrow().display;
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
    state_ptr.as_ref().state.borrow_mut().display = new_display;
    Some(0)
}

fn handle_hit_test_msg(
    handle: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    if !state_ptr.hide_title_bar {
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

    if state_ptr.state.borrow().is_fullscreen() {
        return Some(HTCLIENT as _);
    }

    let dpi = unsafe { GetDpiForWindow(handle) };
    let frame_y = unsafe { GetSystemMetricsForDpi(SM_CYFRAME, dpi) };
    let padding = unsafe { GetSystemMetricsForDpi(SM_CXPADDEDBORDER, dpi) };

    let mut cursor_point = POINT {
        x: lparam.signed_loword().into(),
        y: lparam.signed_hiword().into(),
    };
    unsafe { ScreenToClient(handle, &mut cursor_point).ok().log_err() };
    if cursor_point.y > 0 && cursor_point.y < frame_y + padding {
        return Some(HTTOP as _);
    }

    let titlebar_rect = state_ptr.state.borrow().get_titlebar_rect();
    if let Ok(titlebar_rect) = titlebar_rect {
        if cursor_point.y < titlebar_rect.bottom {
            let caption_btn_width = (state_ptr.state.borrow().caption_button_width().0
                * state_ptr.state.borrow().scale_factor) as i32;
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
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    if !state_ptr.hide_title_bar {
        return None;
    }

    let mut lock = state_ptr.state.borrow_mut();
    if let Some(mut callback) = lock.callbacks.input.take() {
        let scale_factor = lock.scale_factor;
        drop(lock);
        let mut cursor_point = POINT {
            x: lparam.signed_loword().into(),
            y: lparam.signed_hiword().into(),
        };
        unsafe { ScreenToClient(handle, &mut cursor_point).ok().log_err() };
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
        state_ptr.state.borrow_mut().callbacks.input = Some(callback);

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
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    if !state_ptr.hide_title_bar {
        return None;
    }

    let mut lock = state_ptr.state.borrow_mut();
    let result = if let Some(mut callback) = lock.callbacks.input.take() {
        let scale_factor = lock.scale_factor;
        let mut cursor_point = POINT {
            x: lparam.signed_loword().into(),
            y: lparam.signed_hiword().into(),
        };
        unsafe { ScreenToClient(handle, &mut cursor_point).ok().log_err() };
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
        state_ptr.state.borrow_mut().callbacks.input = Some(callback);

        result
    } else {
        None
    };

    // Since these are handled in handle_nc_mouse_up_msg we must prevent the default window proc
    result.or_else(|| matches!(wparam.0 as u32, HTMINBUTTON | HTMAXBUTTON | HTCLOSE).then_some(0))
}

fn handle_nc_mouse_up_msg(
    handle: HWND,
    button: MouseButton,
    wparam: WPARAM,
    lparam: LPARAM,
    state_ptr: Rc<WindowsWindowStatePtr>,
) -> Option<isize> {
    if !state_ptr.hide_title_bar {
        return None;
    }

    let mut lock = state_ptr.state.borrow_mut();
    if let Some(mut callback) = lock.callbacks.input.take() {
        let scale_factor = lock.scale_factor;
        drop(lock);
        let mut cursor_point = POINT {
            x: lparam.signed_loword().into(),
            y: lparam.signed_hiword().into(),
        };
        unsafe { ScreenToClient(handle, &mut cursor_point).ok().log_err() };
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
        state_ptr.state.borrow_mut().callbacks.input = Some(callback);
        if result.is_some() {
            return result;
        }
    } else {
        drop(lock);
    }

    if button == MouseButton::Left {
        match wparam.0 as u32 {
            HTMINBUTTON => unsafe {
                ShowWindowAsync(handle, SW_MINIMIZE).ok().log_err();
            },
            HTMAXBUTTON => unsafe {
                if state_ptr.state.borrow().is_maximized() {
                    ShowWindowAsync(handle, SW_NORMAL).ok().log_err();
                } else {
                    ShowWindowAsync(handle, SW_MAXIMIZE).ok().log_err();
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

fn handle_cursor_changed(lparam: LPARAM, state_ptr: Rc<WindowsWindowStatePtr>) -> Option<isize> {
    state_ptr.state.borrow_mut().current_cursor = HCURSOR(lparam.0);
    Some(0)
}

fn handle_set_cursor(lparam: LPARAM, state_ptr: Rc<WindowsWindowStatePtr>) -> Option<isize> {
    if matches!(
        lparam.loword() as u32,
        HTLEFT | HTRIGHT | HTTOP | HTTOPLEFT | HTTOPRIGHT | HTBOTTOM | HTBOTTOMLEFT | HTBOTTOMRIGHT
    ) {
        return None;
    }
    unsafe { SetCursor(state_ptr.state.borrow().current_cursor) };
    Some(1)
}

fn handle_system_settings_changed(state_ptr: Rc<WindowsWindowStatePtr>) -> Option<isize> {
    let mut lock = state_ptr.state.borrow_mut();
    // mouse wheel
    lock.system_settings.mouse_wheel_settings.update();
    Some(0)
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

enum KeystrokeOrModifier {
    Keystroke(Keystroke),
    Modifier(Modifiers),
}

fn parse_keydown_msg_keystroke(wparam: WPARAM) -> Option<KeystrokeOrModifier> {
    let vk_code = wparam.loword();

    let modifiers = current_modifiers();

    if is_modifier(VIRTUAL_KEY(vk_code)) {
        return Some(KeystrokeOrModifier::Modifier(modifiers));
    }

    if modifiers.control || modifiers.alt {
        let basic_key = basic_vkcode_to_string(vk_code, modifiers);
        if let Some(basic_key) = basic_key {
            return Some(KeystrokeOrModifier::Keystroke(basic_key));
        }
    }

    if vk_code >= VK_F1.0 && vk_code <= VK_F24.0 {
        let offset = vk_code - VK_F1.0;
        return Some(KeystrokeOrModifier::Keystroke(Keystroke {
            modifiers,
            key: format!("f{}", offset + 1),
            ime_key: None,
        }));
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
        Some(KeystrokeOrModifier::Keystroke(Keystroke {
            modifiers,
            key: key.to_string(),
            ime_key: None,
        }))
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
        ImmReleaseContext(handle, ctx).ok().log_err();
        result
    }
}

fn retrieve_composition_cursor_position(handle: HWND) -> usize {
    unsafe {
        let ctx = ImmGetContext(handle);
        let ret = ImmGetCompositionStringW(ctx, GCS_CURSORPOS, None, 0);
        ImmReleaseContext(handle, ctx).ok().log_err();
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
        ImmReleaseContext(handle, ctx).ok().log_err();
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
fn is_virtual_key_pressed(vkey: VIRTUAL_KEY) -> bool {
    unsafe { GetKeyState(vkey.0 as i32) < 0 }
}

fn is_modifier(virtual_key: VIRTUAL_KEY) -> bool {
    matches!(
        virtual_key,
        VK_CONTROL | VK_MENU | VK_SHIFT | VK_LWIN | VK_RWIN
    )
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
