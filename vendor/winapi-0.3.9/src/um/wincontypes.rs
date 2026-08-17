// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This module contains the public data structures, data types, and procedures exported by the NT
//! console subsystem.
use ctypes::c_void;
use shared::minwindef::{BOOL, DWORD, UINT, WORD};
use um::winnt::{CHAR, SHORT, WCHAR};
STRUCT!{struct COORD {
    X: SHORT,
    Y: SHORT,
}}
pub type PCOORD = *mut COORD;
STRUCT!{struct SMALL_RECT {
    Left: SHORT,
    Top: SHORT,
    Right: SHORT,
    Bottom: SHORT,
}}
pub type PSMALL_RECT = *mut SMALL_RECT;
UNION!{union KEY_EVENT_RECORD_uChar {
    [u16; 1],
    UnicodeChar UnicodeChar_mut: WCHAR,
    AsciiChar AsciiChar_mut: CHAR,
}}
STRUCT!{struct KEY_EVENT_RECORD {
    bKeyDown: BOOL,
    wRepeatCount: WORD,
    wVirtualKeyCode: WORD,
    wVirtualScanCode: WORD,
    uChar: KEY_EVENT_RECORD_uChar,
    dwControlKeyState: DWORD,
}}
pub type PKEY_EVENT_RECORD = *mut KEY_EVENT_RECORD;
pub const RIGHT_ALT_PRESSED: DWORD = 0x0001;
pub const LEFT_ALT_PRESSED: DWORD = 0x0002;
pub const RIGHT_CTRL_PRESSED: DWORD = 0x0004;
pub const LEFT_CTRL_PRESSED: DWORD = 0x0008;
pub const SHIFT_PRESSED: DWORD = 0x0010;
pub const NUMLOCK_ON: DWORD = 0x0020;
pub const SCROLLLOCK_ON: DWORD = 0x0040;
pub const CAPSLOCK_ON: DWORD = 0x0080;
pub const ENHANCED_KEY: DWORD = 0x0100;
pub const NLS_DBCSCHAR: DWORD = 0x00010000;
pub const NLS_ALPHANUMERIC: DWORD = 0x00000000;
pub const NLS_KATAKANA: DWORD = 0x00020000;
pub const NLS_HIRAGANA: DWORD = 0x00040000;
pub const NLS_ROMAN: DWORD = 0x00400000;
pub const NLS_IME_CONVERSION: DWORD = 0x00800000;
pub const NLS_IME_DISABLE: DWORD = 0x20000000;
STRUCT!{struct MOUSE_EVENT_RECORD {
    dwMousePosition: COORD,
    dwButtonState: DWORD,
    dwControlKeyState: DWORD,
    dwEventFlags: DWORD,
}}
pub type PMOUSE_EVENT_RECORD = *mut MOUSE_EVENT_RECORD;
pub const FROM_LEFT_1ST_BUTTON_PRESSED: DWORD = 0x0001;
pub const RIGHTMOST_BUTTON_PRESSED: DWORD = 0x0002;
pub const FROM_LEFT_2ND_BUTTON_PRESSED: DWORD = 0x0004;
pub const FROM_LEFT_3RD_BUTTON_PRESSED: DWORD = 0x0008;
pub const FROM_LEFT_4TH_BUTTON_PRESSED: DWORD = 0x0010;
pub const MOUSE_MOVED: DWORD = 0x0001;
pub const DOUBLE_CLICK: DWORD = 0x0002;
pub const MOUSE_WHEELED: DWORD = 0x0004;
pub const MOUSE_HWHEELED: DWORD = 0x0008;
STRUCT!{struct WINDOW_BUFFER_SIZE_RECORD {
    dwSize: COORD,
}}
pub type PWINDOW_BUFFER_SIZE_RECORD = *mut WINDOW_BUFFER_SIZE_RECORD;
STRUCT!{struct MENU_EVENT_RECORD {
    dwCommandId: UINT,
}}
pub type PMENU_EVENT_RECORD = *mut MENU_EVENT_RECORD;
STRUCT!{struct FOCUS_EVENT_RECORD {
    bSetFocus: BOOL,
}}
pub type PFOCUS_EVENT_RECORD = *mut FOCUS_EVENT_RECORD;
UNION!{union INPUT_RECORD_Event {
    [u32; 4],
    KeyEvent KeyEvent_mut: KEY_EVENT_RECORD,
    MouseEvent MouseEvent_mut: MOUSE_EVENT_RECORD,
    WindowBufferSizeEvent WindowBufferSizeEvent_mut: WINDOW_BUFFER_SIZE_RECORD,
    MenuEvent MenuEvent_mut: MENU_EVENT_RECORD,
    FocusEvent FocusEvent_mut: FOCUS_EVENT_RECORD,
}}
STRUCT!{struct INPUT_RECORD {
    EventType: WORD,
    Event: INPUT_RECORD_Event,
}}
pub type PINPUT_RECORD = *mut INPUT_RECORD;
pub const KEY_EVENT: WORD = 0x0001;
pub const MOUSE_EVENT: WORD = 0x0002;
pub const WINDOW_BUFFER_SIZE_EVENT: WORD = 0x0004;
pub const MENU_EVENT: WORD = 0x0008;
pub const FOCUS_EVENT: WORD = 0x0010;
UNION!{union CHAR_INFO_Char {
    [u16; 1],
    UnicodeChar UnicodeChar_mut: WCHAR,
    AsciiChar AsciiChar_mut: CHAR,
}}
STRUCT!{struct CHAR_INFO {
    Char: CHAR_INFO_Char,
    Attributes: WORD,
}}
pub type PCHAR_INFO = *mut CHAR_INFO;
STRUCT!{struct CONSOLE_FONT_INFO {
    nFont: DWORD,
    dwFontSize: COORD,
}}
pub type PCONSOLE_FONT_INFO = *mut CONSOLE_FONT_INFO;
pub type HPCON = *mut c_void;
