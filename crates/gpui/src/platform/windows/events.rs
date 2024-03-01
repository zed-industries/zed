use std::rc::Rc;

use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        Globalization::{WideCharToMultiByte, CP_UTF8},
        Graphics::Gdi::HBRUSH,
        System::SystemServices::{MK_CONTROL, MK_SHIFT},
        UI::{
            Input::KeyboardAndMouse::{
                VIRTUAL_KEY, VK_BACK, VK_CONTROL, VK_DELETE, VK_DOWN, VK_END, VK_ESCAPE, VK_F1,
                VK_F10, VK_F11, VK_F12, VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9,
                VK_HOME, VK_LEFT, VK_LWIN, VK_MENU, VK_NEXT, VK_PRIOR, VK_RETURN, VK_RIGHT,
                VK_RWIN, VK_SHIFT, VK_UP,
            },
            WindowsAndMessaging::{
                CreateWindowExW, DefWindowProcW, RegisterClassExW, CS_DBLCLKS, CS_HREDRAW,
                CS_VREDRAW, HCURSOR, HICON, HMENU, WINDOW_EX_STYLE, WINDOW_STYLE, WM_KEYDOWN,
                WM_LBUTTONDBLCLK, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDBLCLK, WM_MBUTTONDOWN,
                WM_MBUTTONUP, WM_RBUTTONDBLCLK, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_XBUTTONDBLCLK,
                WM_XBUTTONDOWN, WM_XBUTTONUP, WNDCLASSEXW, XBUTTON1,
            },
        },
    },
};

use crate::{
    get_module_handle, get_windowdata, hiword, loword, KeyDownEvent, KeyUpEvent, Keystroke,
    Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, PlatformInput,
    Point, ScrollDelta, ScrollWheelEvent, TouchPhase, WindowsPlatformInner, MOUSE_MOVE_BUTTONS,
    MOUSE_MOVE_LBUTTON, MOUSE_MOVE_MBUTTON, MOUSE_MOVE_RBUTTON, MOUSE_MOVE_XBUTTON1,
    MOUSE_MOVE_XBUTTON2,
};

struct DispatchWindowData(Rc<WindowsPlatformInner>);

pub struct WindowsWinodwDataWrapper<T: WindowsWindowBase + Sized>(pub Rc<T>);

pub trait WindowsWindowBase
where
    Self: Sized,
{
    unsafe fn handle_message(&self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT;

    extern "system" fn event_runner(
        handle: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            let ptr = get_windowdata(handle) as *const WindowsWinodwDataWrapper<Self>;
            if ptr.is_null() {
                return DefWindowProcW(handle, message, wparam, lparam);
            }
            let this = &*ptr;
            this.0.handle_message(message, wparam, lparam)
        }
    }

    fn create(
        window_class_name: PCWSTR,
        style: WINDOW_STYLE,
        exstyle: WINDOW_EX_STYLE,
        x: Option<i32>,
        y: Option<i32>,
        width: Option<i32>,
        height: Option<i32>,
        menu_handle: Option<HMENU>,
        title: Option<PCWSTR>,
    ) -> HWND {
        let window_class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW | CS_DBLCLKS,
            // lpfnWndProc: runner,
            lpfnWndProc: Some(Self::event_runner),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: get_module_handle().into(),
            hIcon: HICON::default(),
            hCursor: HCURSOR::default(),
            hbrBackground: HBRUSH::default(),
            lpszMenuName: PCWSTR::null(),
            lpszClassName: window_class_name,
            hIconSm: HICON(0),
        };
        // we register windows class multuple times, so it will give an error
        let _ = unsafe { RegisterClassExW(&window_class) };

        let handle = unsafe {
            CreateWindowExW(
                exstyle,
                window_class_name,
                title.unwrap_or(PCWSTR::null()),
                style,
                x.unwrap_or(0),
                y.unwrap_or(0),
                width.unwrap_or(0),
                height.unwrap_or(0),
                HWND::default(),
                menu_handle.unwrap_or(HMENU::default()),
                get_module_handle(),
                None,
            )
        };
        if handle == HWND::default() {
            panic!("Window create error: {}", std::io::Error::last_os_error());
        }

        handle
    }
}

pub fn parse_system_key(
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    modifiers: &mut Modifiers,
) -> Option<PlatformInput> {
    let keydown = message == WM_KEYDOWN;
    let mut logical_key = None;
    match VIRTUAL_KEY(wparam.0 as _) {
        VK_BACK => logical_key = Some("backspace".to_string()),
        VK_ESCAPE => logical_key = Some("escape".to_string()),
        VK_RETURN => logical_key = Some("enter".to_string()),
        VK_UP => logical_key = Some("up".to_string()),
        VK_DOWN => logical_key = Some("down".to_string()),
        VK_LEFT => logical_key = Some("left".to_string()),
        VK_RIGHT => logical_key = Some("right".to_string()),
        VK_PRIOR => logical_key = Some("pageup".to_string()),
        VK_NEXT => logical_key = Some("pagedown".to_string()),
        VK_HOME => logical_key = Some("home".to_string()),
        VK_END => logical_key = Some("end".to_string()),
        VK_DELETE => logical_key = Some("delete".to_string()),
        VK_F1 => logical_key = Some("f1".to_string()),
        VK_F2 => logical_key = Some("f2".to_string()),
        VK_F3 => logical_key = Some("f3".to_string()),
        VK_F4 => logical_key = Some("f4".to_string()),
        VK_F5 => logical_key = Some("f5".to_string()),
        VK_F6 => logical_key = Some("f6".to_string()),
        VK_F7 => logical_key = Some("f7".to_string()),
        VK_F8 => logical_key = Some("f8".to_string()),
        VK_F9 => logical_key = Some("f9".to_string()),
        VK_F10 => logical_key = Some("f10".to_string()),
        VK_F11 => logical_key = Some("f11".to_string()),
        VK_F12 => logical_key = Some("f12".to_string()),
        // modifiers
        VK_CONTROL => modifiers.control = keydown,
        VK_MENU => modifiers.alt = keydown,
        VK_SHIFT => modifiers.shift = keydown,
        VK_LWIN | VK_RWIN => modifiers.command = keydown,
        _ => {}
    }

    if let Some(key) = logical_key {
        if keydown {
            Some(PlatformInput::KeyDown(KeyDownEvent {
                keystroke: Keystroke {
                    key,
                    ime_key: None,
                    modifiers: modifiers.clone(),
                },
                is_held: lparam.0 & (0x1 << 30) > 0,
            }))
        } else {
            Some(PlatformInput::KeyUp(KeyUpEvent {
                keystroke: Keystroke {
                    key,
                    ime_key: None,
                    modifiers: modifiers.clone(),
                },
            }))
        }
    } else {
        None
    }
}

pub fn parse_keyboard_input(
    wparam: WPARAM,
    lparam: LPARAM,
    modifiers: &Modifiers,
) -> Option<PlatformInput> {
    if wparam.0 == 8 || wparam.0 == 27 || wparam.0 == 13 || (lparam.0 >> 24) & 1 == 1 {
        // backspace escape enter ctrl
        // these keys are handled by Zed
        return None;
    }
    let src = [wparam.0 as u16];
    let Ok(first_char) = char::decode_utf16(src)
        .map(|r| r.map_err(|e| e.unpaired_surrogate()))
        .collect::<Vec<_>>()[0]
    else {
        return None;
    };
    println!("{} => {:?}", wparam.0, first_char);
    if first_char.is_control() {
        return None;
    }
    Some(PlatformInput::KeyDown(KeyDownEvent {
        keystroke: Keystroke {
            key: first_char.to_string(),
            ime_key: None,
            modifiers: modifiers.clone(),
        },
        is_held: lparam.0 & (0x1 << 30) > 0,
    }))
}

pub fn parse_mouse_button(
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    modifiers: &Modifiers,
) -> PlatformInput {
    match msg {
        WM_LBUTTONDOWN => PlatformInput::MouseDown(MouseDownEvent {
            button: MouseButton::Left,
            position: crate::Point {
                x: crate::Pixels(loword!(lparam.0, i16) as _),
                y: crate::Pixels(hiword!(lparam.0, i16) as _),
            },
            modifiers: modifiers.clone(),
            click_count: 1,
        }),
        WM_LBUTTONDBLCLK => PlatformInput::MouseDown(MouseDownEvent {
            button: MouseButton::Left,
            position: crate::Point {
                x: crate::Pixels(loword!(lparam.0, i16) as _),
                y: crate::Pixels(hiword!(lparam.0, i16) as _),
            },
            modifiers: modifiers.clone(),
            click_count: 2,
        }),
        WM_LBUTTONUP => PlatformInput::MouseUp(MouseUpEvent {
            button: MouseButton::Left,
            position: crate::Point {
                x: crate::Pixels(loword!(lparam.0, i16) as _),
                y: crate::Pixels(hiword!(lparam.0, i16) as _),
            },
            modifiers: modifiers.clone(),
            click_count: 1,
        }),
        WM_RBUTTONDOWN => PlatformInput::MouseDown(MouseDownEvent {
            button: MouseButton::Right,
            position: crate::Point {
                x: crate::Pixels(loword!(lparam.0, i16) as _),
                y: crate::Pixels(hiword!(lparam.0, i16) as _),
            },
            modifiers: modifiers.clone(),
            click_count: 1,
        }),
        WM_RBUTTONDBLCLK => PlatformInput::MouseDown(MouseDownEvent {
            button: MouseButton::Right,
            position: crate::Point {
                x: crate::Pixels(loword!(lparam.0, i16) as _),
                y: crate::Pixels(hiword!(lparam.0, i16) as _),
            },
            modifiers: modifiers.clone(),
            click_count: 2,
        }),
        WM_RBUTTONUP => PlatformInput::MouseUp(MouseUpEvent {
            button: MouseButton::Right,
            position: crate::Point {
                x: crate::Pixels(loword!(lparam.0, i16) as _),
                y: crate::Pixels(hiword!(lparam.0, i16) as _),
            },
            modifiers: modifiers.clone(),
            click_count: 1,
        }),
        WM_MBUTTONDOWN => PlatformInput::MouseDown(MouseDownEvent {
            button: MouseButton::Middle,
            position: crate::Point {
                x: crate::Pixels(loword!(lparam.0, i16) as _),
                y: crate::Pixels(hiword!(lparam.0, i16) as _),
            },
            modifiers: modifiers.clone(),
            click_count: 1,
        }),
        WM_MBUTTONDBLCLK => PlatformInput::MouseDown(MouseDownEvent {
            button: MouseButton::Middle,
            position: crate::Point {
                x: crate::Pixels(loword!(lparam.0, i16) as _),
                y: crate::Pixels(hiword!(lparam.0, i16) as _),
            },
            modifiers: modifiers.clone(),
            click_count: 2,
        }),
        WM_MBUTTONUP => PlatformInput::MouseUp(MouseUpEvent {
            button: MouseButton::Middle,
            position: crate::Point {
                x: crate::Pixels(loword!(lparam.0, i16) as _),
                y: crate::Pixels(hiword!(lparam.0, i16) as _),
            },
            modifiers: modifiers.clone(),
            click_count: 1,
        }),
        WM_XBUTTONDOWN => {
            if hiword!(wparam.0, u16) == XBUTTON1 {
                PlatformInput::MouseDown(MouseDownEvent {
                    button: MouseButton::Navigate(crate::NavigationDirection::Forward),
                    position: crate::Point {
                        x: crate::Pixels(loword!(lparam.0, i16) as _),
                        y: crate::Pixels(hiword!(lparam.0, i16) as _),
                    },
                    modifiers: modifiers.clone(),
                    click_count: 1,
                })
            } else {
                PlatformInput::MouseDown(MouseDownEvent {
                    button: MouseButton::Navigate(crate::NavigationDirection::Back),
                    position: crate::Point {
                        x: crate::Pixels(loword!(lparam.0, i16) as _),
                        y: crate::Pixels(hiword!(lparam.0, i16) as _),
                    },
                    modifiers: modifiers.clone(),
                    click_count: 1,
                })
            }
        }
        WM_XBUTTONDBLCLK => {
            if hiword!(wparam.0, u16) == XBUTTON1 {
                PlatformInput::MouseDown(MouseDownEvent {
                    button: MouseButton::Navigate(crate::NavigationDirection::Forward),
                    position: crate::Point {
                        x: crate::Pixels(loword!(lparam.0, i16) as _),
                        y: crate::Pixels(hiword!(lparam.0, i16) as _),
                    },
                    modifiers: modifiers.clone(),
                    click_count: 2,
                })
            } else {
                PlatformInput::MouseDown(MouseDownEvent {
                    button: MouseButton::Navigate(crate::NavigationDirection::Back),
                    position: crate::Point {
                        x: crate::Pixels(loword!(lparam.0, i16) as _),
                        y: crate::Pixels(hiword!(lparam.0, i16) as _),
                    },
                    modifiers: modifiers.clone(),
                    click_count: 2,
                })
            }
        }
        WM_XBUTTONUP => {
            if hiword!(wparam.0, u16) == XBUTTON1 {
                PlatformInput::MouseUp(MouseUpEvent {
                    button: MouseButton::Navigate(crate::NavigationDirection::Forward),
                    position: crate::Point {
                        x: crate::Pixels(loword!(lparam.0, i16) as _),
                        y: crate::Pixels(hiword!(lparam.0, i16) as _),
                    },
                    modifiers: modifiers.clone(),
                    click_count: 1,
                })
            } else {
                PlatformInput::MouseUp(MouseUpEvent {
                    button: MouseButton::Navigate(crate::NavigationDirection::Back),
                    position: crate::Point {
                        x: crate::Pixels(loword!(lparam.0, i16) as _),
                        y: crate::Pixels(hiword!(lparam.0, i16) as _),
                    },
                    modifiers: modifiers.clone(),
                    click_count: 1,
                })
            }
        }
        _ => unreachable!(),
    }
}

pub fn parse_mouse_movement(
    wparam: WPARAM,
    lparam: LPARAM,
    modifiers: Modifiers,
) -> (Point<Pixels>, PlatformInput) {
    let new_pos = Point {
        x: Pixels(loword!(lparam.0, i16) as _),
        y: Pixels(hiword!(lparam.0, i16) as _),
    };
    let mut pressed_button = None;
    for button_mask in MOUSE_MOVE_BUTTONS {
        if wparam.0 & button_mask > 0 {
            pressed_button = buttonmask_to_button(button_mask);
            break;
        }
    }
    let input = PlatformInput::MouseMove(MouseMoveEvent {
        position: new_pos.clone(),
        pressed_button,
        modifiers,
    });

    (new_pos, input)
}

pub fn parse_mouse_vwheel(wparam: WPARAM, lparam: LPARAM, modifiers: Modifiers) -> PlatformInput {
    let position = Point {
        x: Pixels(loword!(lparam.0, i16) as _),
        y: Pixels(hiword!(lparam.0, i16) as _),
    };
    let lines = hiword!(wparam.0, i16);
    PlatformInput::ScrollWheel(ScrollWheelEvent {
        position,
        delta: ScrollDelta::Lines(Point {
            x: 0.0,
            y: lines as f32 / 120.0,
        }),
        modifiers,
        touch_phase: TouchPhase::default(),
    })
}

pub fn parse_mouse_hwheel(wparam: WPARAM, lparam: LPARAM, modifiers: Modifiers) -> PlatformInput {
    let position = Point {
        x: Pixels(loword!(lparam.0, i16) as _),
        y: Pixels(hiword!(lparam.0, i16) as _),
    };
    let lines = hiword!(wparam.0, i16);
    PlatformInput::ScrollWheel(ScrollWheelEvent {
        position,
        delta: ScrollDelta::Lines(Point {
            x: lines as f32 / 120.0,
            y: 0.0,
        }),
        modifiers,
        touch_phase: TouchPhase::default(),
    })
}

pub unsafe fn parse_dropfiles(wparam: WPARAM, lparam: LPARAM) -> PlatformInput {
    let hdrop = windows::Win32::UI::Shell::HDROP(wparam.0 as isize);
    // DragQueryFileW(hdrop, ifile, lpszfile);

    PlatformInput::FileDrop(crate::FileDropEvent::Exited)
}

fn buttonmask_to_button(mask: usize) -> Option<MouseButton> {
    match mask {
        MOUSE_MOVE_LBUTTON => Some(MouseButton::Left),
        MOUSE_MOVE_RBUTTON => Some(MouseButton::Right),
        MOUSE_MOVE_MBUTTON => Some(MouseButton::Middle),
        MOUSE_MOVE_XBUTTON1 => Some(MouseButton::Navigate(crate::NavigationDirection::Back)),
        MOUSE_MOVE_XBUTTON2 => Some(MouseButton::Navigate(crate::NavigationDirection::Forward)),
        _ => None,
    }
}
