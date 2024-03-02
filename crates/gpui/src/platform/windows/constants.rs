use windows::{
    core::PCWSTR,
    Win32::UI::WindowsAndMessaging::{
        WINDOW_EX_STYLE, WINDOW_STYLE, WM_USER, WS_EX_ACCEPTFILES, WS_EX_LAYERED, WS_EX_NOACTIVATE,
        WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT, WS_OVERLAPPED, WS_OVERLAPPEDWINDOW,
    },
};

// window class
pub const DISPATCH_WINDOW_CLASS: PCWSTR = windows::core::w!("ZedDispatch");
pub const WINDOW_CLASS: PCWSTR = windows::core::w!("ZedWindow");

// window style
pub const DISPATCH_WINDOW_STYLE: WINDOW_STYLE = WS_OVERLAPPED;
pub const DISPATCH_WINDOW_EXSTYLE: WINDOW_EX_STYLE = WINDOW_EX_STYLE(
    WS_EX_NOACTIVATE.0 | WS_EX_TRANSPARENT.0 | WS_EX_LAYERED.0 | WS_EX_TOOLWINDOW.0,
);
pub const WINODW_STYLE: WINDOW_STYLE = WS_OVERLAPPEDWINDOW;
pub const WINODW_EXTRA_EXSTYLE: WINDOW_EX_STYLE = WS_EX_ACCEPTFILES;

// events
// Values in the range 0x0400 (the value of WM_USER) through 0x7FFF are
// available for message identifiers for private window classes.
pub const WINDOW_REFRESH_TIMER: usize = 1;
// the minimum value is 0xA, which means only max to 100fps
pub const WINODW_REFRESH_INTERVAL: u32 = 16;
pub const MAIN_DISPATCH: u32 = WM_USER + 1;
pub const WINDOW_CLOSE: u32 = WM_USER + 2;
pub const WINDOW_OPEN: u32 = WM_USER + 3;
pub const MENU_ACTIONS: u32 = WM_USER + 4;

// mouse buttons
pub const MOUSE_MOVE_BUTTONS: [usize; 5] = [
    MOUSE_MOVE_LBUTTON,
    MOUSE_MOVE_RBUTTON,
    MOUSE_MOVE_MBUTTON,
    MOUSE_MOVE_XBUTTON1,
    MOUSE_MOVE_XBUTTON2,
];
pub const MOUSE_MOVE_LBUTTON: usize = 0x0001;
pub const MOUSE_MOVE_RBUTTON: usize = 0x0002;
pub const MOUSE_MOVE_MBUTTON: usize = 0x0010;
pub const MOUSE_MOVE_XBUTTON1: usize = 0x0020;
pub const MOUSE_MOVE_XBUTTON2: usize = 0x0040;

// text system
pub const STRING_MAX_LENGTH: usize = 128;
pub const CF_UNICODETEXT: u32 = 13;
pub const DRAGDROP_GET_COUNT: u32 = 0xFFFFFFFF;
pub const FILENAME_MAXLENGTH: usize = 256;

// ACCEL structure fVirt
pub const ACCEL_FVIRTKEY: u8 = 0x01;
pub const ACCEL_FSHIFT: u8 = 0x04;
pub const ACCEL_FCONTROL: u8 = 0x08;
pub const ACCEL_FALT: u8 = 0x10;

// clipboard
pub const CLIPBOARD_TEXT_HASH: PCWSTR = windows::core::w!("ZedTextHash");
pub const CLIPBOARD_METADATA: PCWSTR = windows::core::w!("ZedMetadata");
