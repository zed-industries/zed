// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This module contains the public data structures, data types, and procedures exported by the NT
//! console subsystem.
use ctypes::c_void;
use shared::minwindef::{BOOL, DWORD, LPDWORD, LPVOID, LPWORD, UINT, ULONG, WORD};
use shared::windef::{COLORREF, HWND};
use um::minwinbase::SECURITY_ATTRIBUTES;
use um::wingdi::LF_FACESIZE;
use um::winnt::{CHAR, HANDLE, LPCSTR, LPCWSTR, LPSTR, LPWSTR, WCHAR};
// Many definitions in wincontypes used to be defined in this file, so reexport them to avoid
// breakage. For clarity they are imported in the order they are defined in that file rather
// than winapi's usual alphabetical ordering, with some newlines and indentation to match their
// grouping in the file.
pub use um::wincontypes::{
    COORD, PCOORD,
    SMALL_RECT, PSMALL_RECT,
    KEY_EVENT_RECORD_uChar, KEY_EVENT_RECORD, PKEY_EVENT_RECORD,
    RIGHT_ALT_PRESSED, LEFT_ALT_PRESSED, RIGHT_CTRL_PRESSED, LEFT_CTRL_PRESSED, SHIFT_PRESSED,
        NUMLOCK_ON, SCROLLLOCK_ON, CAPSLOCK_ON, ENHANCED_KEY, NLS_DBCSCHAR, NLS_ALPHANUMERIC,
        NLS_KATAKANA, NLS_HIRAGANA, NLS_ROMAN, NLS_IME_CONVERSION, NLS_IME_DISABLE,
    MOUSE_EVENT_RECORD, PMOUSE_EVENT_RECORD,
    FROM_LEFT_1ST_BUTTON_PRESSED, RIGHTMOST_BUTTON_PRESSED, FROM_LEFT_2ND_BUTTON_PRESSED,
        FROM_LEFT_3RD_BUTTON_PRESSED, FROM_LEFT_4TH_BUTTON_PRESSED, MOUSE_MOVED, DOUBLE_CLICK,
        MOUSE_WHEELED, MOUSE_HWHEELED,
    WINDOW_BUFFER_SIZE_RECORD, PWINDOW_BUFFER_SIZE_RECORD,
    MENU_EVENT_RECORD, PMENU_EVENT_RECORD,
    FOCUS_EVENT_RECORD, PFOCUS_EVENT_RECORD,
    INPUT_RECORD_Event, INPUT_RECORD, PINPUT_RECORD,
    KEY_EVENT, MOUSE_EVENT, WINDOW_BUFFER_SIZE_EVENT, MENU_EVENT, FOCUS_EVENT,
    CHAR_INFO_Char, CHAR_INFO, PCHAR_INFO,
    CONSOLE_FONT_INFO, PCONSOLE_FONT_INFO
};
pub const FOREGROUND_BLUE: WORD = 0x0001;
pub const FOREGROUND_GREEN: WORD = 0x0002;
pub const FOREGROUND_RED: WORD = 0x0004;
pub const FOREGROUND_INTENSITY: WORD = 0x0008;
pub const BACKGROUND_BLUE: WORD = 0x0010;
pub const BACKGROUND_GREEN: WORD = 0x0020;
pub const BACKGROUND_RED: WORD = 0x0040;
pub const BACKGROUND_INTENSITY: WORD = 0x0080;
pub const COMMON_LVB_LEADING_BYTE: WORD = 0x0100;
pub const COMMON_LVB_TRAILING_BYTE: WORD = 0x0200;
pub const COMMON_LVB_GRID_HORIZONTAL: WORD = 0x0400;
pub const COMMON_LVB_GRID_LVERTICAL: WORD = 0x0800;
pub const COMMON_LVB_GRID_RVERTICAL: WORD = 0x1000;
pub const COMMON_LVB_REVERSE_VIDEO: WORD = 0x4000;
pub const COMMON_LVB_UNDERSCORE: WORD = 0x8000;
pub const COMMON_LVB_SBCSDBCS: WORD = 0x0300;
STRUCT!{struct CONSOLE_SCREEN_BUFFER_INFO {
    dwSize: COORD,
    dwCursorPosition: COORD,
    wAttributes: WORD,
    srWindow: SMALL_RECT,
    dwMaximumWindowSize: COORD,
}}
pub type PCONSOLE_SCREEN_BUFFER_INFO = *mut CONSOLE_SCREEN_BUFFER_INFO;
STRUCT!{struct CONSOLE_SCREEN_BUFFER_INFOEX {
    cbSize: ULONG,
    dwSize: COORD,
    dwCursorPosition: COORD,
    wAttributes: WORD,
    srWindow: SMALL_RECT,
    dwMaximumWindowSize: COORD,
    wPopupAttributes: WORD,
    bFullscreenSupported: BOOL,
    ColorTable: [COLORREF; 16],
}}
pub type PCONSOLE_SCREEN_BUFFER_INFOEX = *mut CONSOLE_SCREEN_BUFFER_INFOEX;
STRUCT!{struct CONSOLE_CURSOR_INFO {
    dwSize: DWORD,
    bVisible: BOOL,
}}
pub type PCONSOLE_CURSOR_INFO = *mut CONSOLE_CURSOR_INFO;
STRUCT!{struct CONSOLE_FONT_INFOEX {
    cbSize: ULONG,
    nFont: DWORD,
    dwFontSize: COORD,
    FontFamily: UINT,
    FontWeight: UINT,
    FaceName: [WCHAR; LF_FACESIZE],
}}
pub type PCONSOLE_FONT_INFOEX = *mut CONSOLE_FONT_INFOEX;
pub const HISTORY_NO_DUP_FLAG: DWORD = 0x1;
STRUCT!{struct CONSOLE_HISTORY_INFO {
    cbSize: UINT,
    HistoryBufferSize: UINT,
    NumberOfHistoryBuffers: UINT,
    dwFlags: DWORD,
}}
pub type PCONSOLE_HISTORY_INFO = *mut CONSOLE_HISTORY_INFO;
STRUCT!{struct CONSOLE_SELECTION_INFO {
    dwFlags: DWORD,
    dwSelectionAnchor: COORD,
    srSelection: SMALL_RECT,
}}
pub type PCONSOLE_SELECTION_INFO = *mut CONSOLE_SELECTION_INFO;
pub const CONSOLE_NO_SELECTION: DWORD = 0x0000;
pub const CONSOLE_SELECTION_IN_PROGRESS: DWORD = 0x0001;
pub const CONSOLE_SELECTION_NOT_EMPTY: DWORD = 0x0002;
pub const CONSOLE_MOUSE_SELECTION: DWORD = 0x0004;
pub const CONSOLE_MOUSE_DOWN: DWORD = 0x0008;
FN!{stdcall PHANDLER_ROUTINE(
    CtrlType: DWORD,
) -> BOOL}
pub const CTRL_C_EVENT: DWORD = 0;
pub const CTRL_BREAK_EVENT: DWORD = 1;
pub const CTRL_CLOSE_EVENT: DWORD = 2;
pub const CTRL_LOGOFF_EVENT: DWORD = 5;
pub const CTRL_SHUTDOWN_EVENT: DWORD = 6;
pub const ENABLE_PROCESSED_INPUT: DWORD = 0x0001;
pub const ENABLE_LINE_INPUT: DWORD = 0x0002;
pub const ENABLE_ECHO_INPUT: DWORD = 0x0004;
pub const ENABLE_WINDOW_INPUT: DWORD = 0x0008;
pub const ENABLE_MOUSE_INPUT: DWORD = 0x0010;
pub const ENABLE_INSERT_MODE: DWORD = 0x0020;
pub const ENABLE_QUICK_EDIT_MODE: DWORD = 0x0040;
pub const ENABLE_EXTENDED_FLAGS: DWORD = 0x0080;
pub const ENABLE_AUTO_POSITION: DWORD = 0x0100;
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: DWORD = 0x0200;
pub const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001;
pub const ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002;
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004;
pub const DISABLE_NEWLINE_AUTO_RETURN: DWORD = 0x0008;
pub const ENABLE_LVB_GRID_WORLDWIDE: DWORD = 0x0010;
extern "system" {
    pub fn PeekConsoleInputW(
        hConsoleInput: HANDLE,
        lpBuffer: PINPUT_RECORD,
        nLength: DWORD,
        lpNumberOfEventsRead: LPDWORD,
    ) -> BOOL;
    pub fn WriteConsoleInputA(
        hConsoleInput: HANDLE,
        lpBuffer: *const INPUT_RECORD,
        nLength: DWORD,
        lpNumberOfEventsWritten: LPDWORD,
    ) -> BOOL;
    pub fn WriteConsoleInputW(
        hConsoleInput: HANDLE,
        lpBuffer: *const INPUT_RECORD,
        nLength: DWORD,
        lpNumberOfEventsWritten: LPDWORD,
    ) -> BOOL;
    pub fn ReadConsoleOutputA(
        hConsoleOutput: HANDLE,
        lpBuffer: PCHAR_INFO,
        dwBufferSize: COORD,
        dwBufferCoord: COORD,
        lpReadRegion: PSMALL_RECT,
    ) -> BOOL;
    pub fn ReadConsoleOutputW(
        hConsoleOutput: HANDLE,
        lpBuffer: PCHAR_INFO,
        dwBufferSize: COORD,
        dwBufferCoord: COORD,
        lpReadRegion: PSMALL_RECT,
    ) -> BOOL;
    pub fn WriteConsoleOutputA(
        hConsoleOutput: HANDLE,
        lpBuffer: *const CHAR_INFO,
        dwBufferSize: COORD,
        dwBufferCoord: COORD,
        lpWriteRegion: PSMALL_RECT,
    ) -> BOOL;
    pub fn WriteConsoleOutputW(
        hConsoleOutput: HANDLE,
        lpBuffer: *const CHAR_INFO,
        dwBufferSize: COORD,
        dwBufferCoord: COORD,
        lpWriteRegion: PSMALL_RECT,
    ) -> BOOL;
    pub fn ReadConsoleOutputCharacterA(
        hConsoleOutput: HANDLE,
        lpCharacter: LPSTR,
        nLength: DWORD,
        dwReadCoord: COORD,
        lpNumberOfCharsRead: LPDWORD,
    ) -> BOOL;
    pub fn ReadConsoleOutputCharacterW(
        hConsoleOutput: HANDLE,
        lpCharacter: LPWSTR,
        nLength: DWORD,
        dwReadCoord: COORD,
        lpNumberOfCharsRead: LPDWORD,
    ) -> BOOL;
    pub fn ReadConsoleOutputAttribute(
        hConsoleOutput: HANDLE,
        lpAttribute: LPWORD,
        nLength: DWORD,
        dwReadCoord: COORD,
        lpNumberOfAttrsRead: LPDWORD,
    ) -> BOOL;
    pub fn WriteConsoleOutputCharacterA(
        hConsoleOutput: HANDLE,
        lpCharacter: LPCSTR,
        nLength: DWORD,
        dwWriteCoord: COORD,
        lpNumberOfCharsWritten: LPDWORD,
    ) -> BOOL;
    pub fn WriteConsoleOutputCharacterW(
        hConsoleOutput: HANDLE,
        lpCharacter: LPCWSTR,
        nLength: DWORD,
        dwWriteCoord: COORD,
        lpNumberOfCharsWritten: LPDWORD,
    ) -> BOOL;
    pub fn WriteConsoleOutputAttribute(
        hConsoleOutput: HANDLE,
        lpAttribute: *const WORD,
        nLength: DWORD,
        dwWriteCoord: COORD,
        lpNumberOfAttrsWritten: LPDWORD,
    ) -> BOOL;
    pub fn FillConsoleOutputCharacterA(
        hConsoleOutput: HANDLE,
        cCharacter: CHAR,
        nLength: DWORD,
        dwWriteCoord: COORD,
        lpNumberOfCharsWritten: LPDWORD,
    ) -> BOOL;
    pub fn FillConsoleOutputCharacterW(
        hConsoleOutput: HANDLE,
        cCharacter: WCHAR,
        nLength: DWORD,
        dwWriteCoord: COORD,
        lpNumberOfCharsWritten: LPDWORD,
    ) -> BOOL;
    pub fn FillConsoleOutputAttribute(
        hConsoleOutput: HANDLE,
        wAttribute: WORD,
        nLength: DWORD,
        dwWriteCoord: COORD,
        lpNumberOfAttrsWritten: LPDWORD,
    ) -> BOOL;
}
pub const CONSOLE_REAL_OUTPUT_HANDLE: *mut c_void = -2isize as *mut c_void;
pub const CONSOLE_REAL_INPUT_HANDLE: *mut c_void = -3isize as *mut c_void;
extern "system" {
    pub fn GetConsoleScreenBufferInfo(
        hConsoleOutput: HANDLE,
        lpConsoleScreenBufferInfo: PCONSOLE_SCREEN_BUFFER_INFO,
    ) -> BOOL;
    pub fn GetConsoleScreenBufferInfoEx(
        hConsoleOutput: HANDLE,
        lpConsoleScreenBufferInfoEx: PCONSOLE_SCREEN_BUFFER_INFOEX,
    ) -> BOOL;
    pub fn SetConsoleScreenBufferInfoEx(
        hConsoleOutput: HANDLE,
        lpConsoleScreenBufferInfoEx: PCONSOLE_SCREEN_BUFFER_INFOEX,
    ) -> BOOL;
    pub fn GetLargestConsoleWindowSize(
        hConsoleOutput: HANDLE,
    ) -> COORD;
    pub fn GetConsoleCursorInfo(
        hConsoleOutput: HANDLE,
        lpConsoleCursorInfo: PCONSOLE_CURSOR_INFO,
    ) -> BOOL;
    pub fn GetCurrentConsoleFont(
        hConsoleOutput: HANDLE,
        bMaximumWindow: BOOL,
        lpConsoleCurrentFont: PCONSOLE_FONT_INFO,
    ) -> BOOL;
    pub fn GetCurrentConsoleFontEx(
        hConsoleOutput: HANDLE,
        bMaximumWindow: BOOL,
        lpConsoleCurrentFontEx: PCONSOLE_FONT_INFOEX,
    ) -> BOOL;
    pub fn SetCurrentConsoleFontEx(
        hConsoleOutput: HANDLE,
        bMaximumWindow: BOOL,
        lpConsoleCurrentFontEx: PCONSOLE_FONT_INFOEX,
    ) -> BOOL;
    pub fn GetConsoleHistoryInfo(
        lpConsoleHistoryInfo: PCONSOLE_HISTORY_INFO,
    ) -> BOOL;
    pub fn SetConsoleHistoryInfo(
        lpConsoleHistoryInfo: PCONSOLE_HISTORY_INFO,
    ) -> BOOL;
    pub fn GetConsoleFontSize(
        hConsoleOutput: HANDLE,
        nFont: DWORD,
    ) -> COORD;
    pub fn GetConsoleSelectionInfo(
        lpConsoleSelectionInfo: PCONSOLE_SELECTION_INFO,
    ) -> BOOL;
    pub fn GetNumberOfConsoleMouseButtons(
        lpNumberOfMouseButtons: LPDWORD,
    ) -> BOOL;
    pub fn SetConsoleActiveScreenBuffer(
        hConsoleOutput: HANDLE,
    ) -> BOOL;
    pub fn FlushConsoleInputBuffer(
        hConsoleInput: HANDLE,
    ) -> BOOL;
    pub fn SetConsoleScreenBufferSize(
        hConsoleOutput: HANDLE,
        dwSize: COORD,
    ) -> BOOL;
    pub fn SetConsoleCursorPosition(
        hConsoleOutput: HANDLE,
        dwCursorPosition: COORD,
    ) -> BOOL;
    pub fn SetConsoleCursorInfo(
        hConsoleOutput: HANDLE,
        lpConsoleCursorInfo: *const CONSOLE_CURSOR_INFO,
    ) -> BOOL;
    pub fn ScrollConsoleScreenBufferA(
        hConsoleOutput: HANDLE,
        lpScrollRectangle: *const SMALL_RECT,
        lpClipRectangle: *const SMALL_RECT,
        dwDestinationOrigin: COORD,
        lpFill: *const CHAR_INFO,
    ) -> BOOL;
    pub fn ScrollConsoleScreenBufferW(
        hConsoleOutput: HANDLE,
        lpScrollRectangle: *const SMALL_RECT,
        lpClipRectangle: *const SMALL_RECT,
        dwDestinationOrigin: COORD,
        lpFill: *const CHAR_INFO,
    ) -> BOOL;
    pub fn SetConsoleWindowInfo(
        hConsoleOutput: HANDLE,
        bAbsolute: BOOL,
        lpConsoleWindow: *const SMALL_RECT,
    ) -> BOOL;
    pub fn SetConsoleTextAttribute(
        hConsoleOutput: HANDLE,
        wAttributes: WORD,
    ) -> BOOL;
    pub fn GenerateConsoleCtrlEvent(
        dwCtrlEvent: DWORD,
        dwProcessGroupId: DWORD,
    ) -> BOOL;
    pub fn FreeConsole() -> BOOL;
    pub fn AttachConsole(
        dwProcessId: DWORD,
    ) -> BOOL;
}
pub const ATTACH_PARENT_PROCESS: DWORD = 0xFFFFFFFF;
extern "system" {
    pub fn GetConsoleTitleA(
        lpConsoleTitle: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetConsoleTitleW(
        lpConsoleTitle: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetConsoleOriginalTitleA(
        lpConsoleTitle: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetConsoleOriginalTitleW(
        lpConsoleTitle: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn SetConsoleTitleA(
        lpConsoleTitle: LPCSTR,
    ) -> BOOL;
    pub fn SetConsoleTitleW(
        lpConsoleTitle: LPCWSTR,
    ) -> BOOL;
}
STRUCT!{struct CONSOLE_READCONSOLE_CONTROL {
    nLength: ULONG,
    nInitialChars: ULONG,
    dwCtrlWakeupMask: ULONG,
    dwControlKeyState: ULONG,
}}
pub type PCONSOLE_READCONSOLE_CONTROL = *mut CONSOLE_READCONSOLE_CONTROL;
pub const CONSOLE_TEXTMODE_BUFFER: DWORD = 1;
extern "system" {
    pub fn CreateConsoleScreenBuffer(
        dwDesiredAccess: DWORD,
        dwShareMode: DWORD,
        lpSecurityAttributes: *const SECURITY_ATTRIBUTES,
        dwFlags: DWORD,
        lpScreenBufferData: LPVOID,
    ) -> HANDLE;
    pub fn SetConsoleCP(
        wCodePageID: UINT,
    ) -> BOOL;
    pub fn SetConsoleOutputCP(
        wCodePageID: UINT,
    ) -> BOOL;
}
pub const CONSOLE_FULLSCREEN: DWORD = 1;
pub const CONSOLE_FULLSCREEN_HARDWARE: DWORD = 2;
extern "system" {
    pub fn GetConsoleDisplayMode(
        lpModeFlags: LPDWORD,
    ) -> BOOL;
}
pub const CONSOLE_FULLSCREEN_MODE: DWORD = 1;
pub const CONSOLE_WINDOWED_MODE: DWORD = 2;
extern "system" {
    pub fn SetConsoleDisplayMode(
        hConsoleOutput: HANDLE,
        dwFlags: DWORD,
        lpNewScreenBufferDimensions: PCOORD,
    ) -> BOOL;
    pub fn GetConsoleWindow() -> HWND;
    pub fn GetConsoleProcessList(
        lpdwProcessList: LPDWORD,
        dwProcessCount: DWORD,
    ) -> DWORD;
    pub fn AddConsoleAliasA(
        Source: LPSTR,
        Target: LPSTR,
        ExeName: LPSTR,
    ) -> BOOL;
    pub fn AddConsoleAliasW(
        Source: LPWSTR,
        Target: LPWSTR,
        ExeName: LPWSTR,
    ) -> BOOL;
    pub fn GetConsoleAliasA(
        Source: LPSTR,
        TargetBuffer: LPSTR,
        TargetBufferLength: DWORD,
        ExeName: LPSTR,
    ) -> DWORD;
    pub fn GetConsoleAliasW(
        Source: LPWSTR,
        TargetBuffer: LPWSTR,
        TargetBufferLength: DWORD,
        ExeName: LPWSTR,
    ) -> DWORD;
    pub fn GetConsoleAliasesLengthA(
        ExeName: LPSTR,
    ) -> DWORD;
    pub fn GetConsoleAliasesLengthW(
        ExeName: LPWSTR,
    ) -> DWORD;
    pub fn GetConsoleAliasExesLengthA() -> DWORD;
    pub fn GetConsoleAliasExesLengthW() -> DWORD;
    pub fn GetConsoleAliasesA(
        AliasBuffer: LPSTR,
        AliasBufferLength: DWORD,
        ExeName: LPSTR,
    ) -> DWORD;
    pub fn GetConsoleAliasesW(
        AliasBuffer: LPWSTR,
        AliasBufferLength: DWORD,
        ExeName: LPWSTR,
    ) -> DWORD;
    pub fn GetConsoleAliasExesA(
        ExeNameBuffer: LPSTR,
        ExeNameBufferLength: DWORD,
    ) -> DWORD;
    pub fn GetConsoleAliasExesW(
        ExeNameBuffer: LPWSTR,
        ExeNameBufferLength: DWORD,
    ) -> DWORD;
}
