// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! USER procedure declarations, constant definitions and macros
use ctypes::{c_int, c_long, c_short, c_uint};
use shared::basetsd::{
    DWORD_PTR, INT32, INT_PTR, PDWORD_PTR, UINT16, UINT32, UINT64, UINT_PTR, ULONG_PTR,
};
#[cfg(target_pointer_width = "64")]
use shared::basetsd::LONG_PTR;
use shared::guiddef::{GUID, LPCGUID};
use shared::minwindef::{
    ATOM, BOOL, BYTE, DWORD, HINSTANCE, HIWORD, HKL, HMODULE, HRGN, HWINSTA, INT, LOWORD, LPARAM,
    LPBYTE, LPDWORD, LPINT, LPVOID, LPWORD, LRESULT, PBYTE, PUINT, PULONG, TRUE, UCHAR, UINT,
    ULONG, USHORT, WORD, WPARAM,
};
use shared::windef::{
    COLORREF, DPI_AWARENESS, DPI_AWARENESS_CONTEXT, DPI_HOSTING_BEHAVIOR, HACCEL, HBITMAP, HBRUSH,
    HCURSOR, HDC, HDESK, HHOOK, HICON, HMENU, HMONITOR, HWINEVENTHOOK, HWND, LPCRECT, LPPOINT,
    LPRECT, POINT, RECT, SIZE,
};
use um::minwinbase::LPSECURITY_ATTRIBUTES;
use um::wingdi::{
    BLENDFUNCTION, DEVMODEA, DEVMODEW, LOGFONTA, LOGFONTW, PDISPLAY_DEVICEA, PDISPLAY_DEVICEW
};
use um::winnt::{
    ACCESS_MASK, BOOLEAN, CHAR, HANDLE, LONG, LPCSTR, LPCWSTR, LPSTR, LPWSTR, LUID,
    PSECURITY_DESCRIPTOR, PSECURITY_INFORMATION, PVOID, SHORT, VOID, WCHAR,
};
use vc::limits::UINT_MAX;
use vc::vadefs::va_list;
pub type HDWP = HANDLE;
pub type MENUTEMPLATEA = VOID;
pub type MENUTEMPLATEW = VOID;
pub type LPMENUTEMPLATEA = PVOID;
pub type LPMENUTEMPLATEW = PVOID;
FN!{stdcall WNDPROC(
    HWND,
    UINT,
    WPARAM,
    LPARAM,
) -> LRESULT}
FN!{stdcall DLGPROC(
    HWND,
    UINT,
    WPARAM,
    LPARAM,
) -> INT_PTR}
FN!{stdcall TIMERPROC(
    HWND,
    UINT,
    UINT_PTR,
    DWORD,
) -> ()}
FN!{stdcall GRAYSTRINGPROC(
    HDC,
    LPARAM,
    c_int,
) -> BOOL}
FN!{stdcall WNDENUMPROC(
    HWND,
    LPARAM,
) -> BOOL}
FN!{stdcall HOOKPROC(
    code: c_int,
    wParam: WPARAM,
    lParam: LPARAM,
) -> LRESULT}
FN!{stdcall SENDASYNCPROC(
    HWND,
    UINT,
    ULONG_PTR,
    LRESULT,
) -> ()}
FN!{stdcall PROPENUMPROCA(
    HWND,
    LPCSTR,
    HANDLE,
) -> BOOL}
FN!{stdcall PROPENUMPROCW(
    HWND,
    LPCWSTR,
    HANDLE,
) -> BOOL}
FN!{stdcall PROPENUMPROCEXA(
    HWND,
    LPSTR,
    HANDLE,
    ULONG_PTR,
) -> BOOL}
FN!{stdcall PROPENUMPROCEXW(
    HWND,
    LPWSTR,
    HANDLE,
    ULONG_PTR,
) -> BOOL}
FN!{stdcall EDITWORDBREAKPROCA(
    lpch: LPSTR,
    ichCurrent: c_int,
    cch: c_int,
    code: c_int,
) -> c_int}
FN!{stdcall EDITWORDBREAKPROCW(
    lpch: LPWSTR,
    ichCurrent: c_int,
    cch: c_int,
    code: c_int,
) -> c_int}
FN!{stdcall DRAWSTATEPROC(
    hdc: HDC,
    lData: LPARAM,
    wData: WPARAM,
    cx: c_int,
    cy: c_int,
) -> BOOL}
FN!{stdcall NAMEENUMPROCA(
    LPSTR,
    LPARAM,
) -> BOOL}
FN!{stdcall NAMEENUMPROCW(
    LPWSTR,
    LPARAM,
) -> BOOL}
pub type WINSTAENUMPROCA = NAMEENUMPROCA;
pub type DESKTOPENUMPROCA = NAMEENUMPROCA;
pub type WINSTAENUMPROCW = NAMEENUMPROCW;
pub type DESKTOPENUMPROCW = NAMEENUMPROCW;
#[inline]
pub fn IS_INTRESOURCE(r: ULONG_PTR) -> bool {
    (r >> 16) == 0
}
#[inline]
pub fn MAKEINTRESOURCEA(i: WORD) -> LPSTR {
    i as ULONG_PTR as LPSTR
}
#[inline]
pub fn MAKEINTRESOURCEW(i: WORD) -> LPWSTR {
    i as ULONG_PTR as LPWSTR
}
pub const RT_CURSOR: LPWSTR = MAKEINTRESOURCE!(1);
pub const RT_BITMAP: LPWSTR = MAKEINTRESOURCE!(2);
pub const RT_ICON: LPWSTR = MAKEINTRESOURCE!(3);
pub const RT_MENU: LPWSTR = MAKEINTRESOURCE!(4);
pub const RT_DIALOG: LPWSTR = MAKEINTRESOURCE!(5);
pub const RT_STRING: LPWSTR = MAKEINTRESOURCE!(6);
pub const RT_FONTDIR: LPWSTR = MAKEINTRESOURCE!(7);
pub const RT_FONT: LPWSTR = MAKEINTRESOURCE!(8);
pub const RT_ACCELERATOR: LPWSTR = MAKEINTRESOURCE!(9);
pub const RT_RCDATA: LPWSTR = MAKEINTRESOURCE!(10);
pub const RT_MESSAGETABLE: LPWSTR = MAKEINTRESOURCE!(11);
pub const DIFFERENCE: WORD = 11;
pub const RT_GROUP_CURSOR: LPWSTR = MAKEINTRESOURCE!(1 + DIFFERENCE);
pub const RT_GROUP_ICON: LPWSTR = MAKEINTRESOURCE!(3 + DIFFERENCE);
pub const RT_VERSION: LPWSTR = MAKEINTRESOURCE!(16);
pub const RT_DLGINCLUDE: LPWSTR = MAKEINTRESOURCE!(17);
pub const RT_PLUGPLAY: LPWSTR = MAKEINTRESOURCE!(19);
pub const RT_VXD: LPWSTR = MAKEINTRESOURCE!(20);
pub const RT_ANICURSOR: LPWSTR = MAKEINTRESOURCE!(21);
pub const RT_ANIICON: LPWSTR = MAKEINTRESOURCE!(22);
pub const RT_HTML: LPWSTR = MAKEINTRESOURCE!(23);
pub const RT_MANIFEST: LPWSTR = MAKEINTRESOURCE!(24);
pub const CREATEPROCESS_MANIFEST_RESOURCE_ID: LPWSTR = MAKEINTRESOURCE!(1);
pub const ISOLATIONAWARE_MANIFEST_RESOURCE_ID: LPWSTR = MAKEINTRESOURCE!(2);
pub const ISOLATIONAWARE_NOSTATICIMPORT_MANIFEST_RESOURCE_ID: LPWSTR
    = MAKEINTRESOURCE!(3);
pub const MINIMUM_RESERVED_MANIFEST_RESOURCE_ID: LPWSTR = MAKEINTRESOURCE!(1);
pub const MAXIMUM_RESERVED_MANIFEST_RESOURCE_ID: LPWSTR = MAKEINTRESOURCE!(16);
extern "system" {
    pub fn wvsprintfA(
        _: LPSTR,
        _: LPCSTR,
        arglist: va_list,
    ) -> c_int;
    pub fn wvsprintfW(
        _: LPWSTR,
        _: LPCWSTR,
        arglist: va_list,
    ) -> c_int;
}
extern "C" {
    pub fn wsprintfA(
        _: LPSTR,
        _: LPCSTR,
        ...
    ) -> c_int;
    pub fn wsprintfW(
        _: LPWSTR,
        _: LPCWSTR,
        ...
    ) -> c_int;
}
pub const SETWALLPAPER_DEFAULT: LPWSTR = -1isize as LPWSTR;
pub const SB_HORZ: UINT = 0;
pub const SB_VERT: UINT = 1;
pub const SB_CTL: UINT = 2;
pub const SB_BOTH: UINT = 3;
pub const SB_LINEUP: LPARAM = 0;
pub const SB_LINELEFT: LPARAM = 0;
pub const SB_LINEDOWN: LPARAM = 1;
pub const SB_LINERIGHT: LPARAM = 1;
pub const SB_PAGEUP: LPARAM = 2;
pub const SB_PAGELEFT: LPARAM = 2;
pub const SB_PAGEDOWN: LPARAM = 3;
pub const SB_PAGERIGHT: LPARAM = 3;
pub const SB_THUMBPOSITION: LPARAM = 4;
pub const SB_THUMBTRACK: LPARAM = 5;
pub const SB_TOP: LPARAM = 6;
pub const SB_LEFT: LPARAM = 6;
pub const SB_BOTTOM: LPARAM = 7;
pub const SB_RIGHT: LPARAM = 7;
pub const SB_ENDSCROLL: LPARAM = 8;
pub const SW_HIDE: c_int = 0;
pub const SW_SHOWNORMAL: c_int = 1;
pub const SW_NORMAL: c_int = 1;
pub const SW_SHOWMINIMIZED: c_int = 2;
pub const SW_SHOWMAXIMIZED: c_int = 3;
pub const SW_MAXIMIZE: c_int = 3;
pub const SW_SHOWNOACTIVATE: c_int = 4;
pub const SW_SHOW: c_int = 5;
pub const SW_MINIMIZE: c_int = 6;
pub const SW_SHOWMINNOACTIVE: c_int = 7;
pub const SW_SHOWNA: c_int = 8;
pub const SW_RESTORE: c_int = 9;
pub const SW_SHOWDEFAULT: c_int = 10;
pub const SW_FORCEMINIMIZE: c_int = 11;
pub const SW_MAX: c_int = 11;
pub const HIDE_WINDOW: c_int = 0;
pub const SHOW_OPENWINDOW: c_int = 1;
pub const SHOW_ICONWINDOW: c_int = 2;
pub const SHOW_FULLSCREEN: c_int = 3;
pub const SHOW_OPENNOACTIVATE: c_int = 4;
pub const SW_PARENTCLOSING: LPARAM = 1;
pub const SW_OTHERZOOM: LPARAM = 2;
pub const SW_PARENTOPENING: LPARAM = 3;
pub const SW_OTHERUNZOOM: LPARAM = 4;
pub const AW_HOR_POSITIVE: DWORD = 0x00000001;
pub const AW_HOR_NEGATIVE: DWORD = 0x00000002;
pub const AW_VER_POSITIVE: DWORD = 0x00000004;
pub const AW_VER_NEGATIVE: DWORD = 0x00000008;
pub const AW_CENTER: DWORD = 0x00000010;
pub const AW_HIDE: DWORD = 0x00010000;
pub const AW_ACTIVATE: DWORD = 0x00020000;
pub const AW_SLIDE: DWORD = 0x00040000;
pub const AW_BLEND: DWORD = 0x00080000;
pub const KF_EXTENDED: WORD = 0x0100;
pub const KF_DLGMODE: WORD = 0x0800;
pub const KF_MENUMODE: WORD = 0x1000;
pub const KF_ALTDOWN: WORD = 0x2000;
pub const KF_REPEAT: WORD = 0x4000;
pub const KF_UP: WORD = 0x8000;
pub const VK_LBUTTON: c_int = 0x01;
pub const VK_RBUTTON: c_int = 0x02;
pub const VK_CANCEL: c_int = 0x03;
pub const VK_MBUTTON: c_int = 0x04;
pub const VK_XBUTTON1: c_int = 0x05;
pub const VK_XBUTTON2: c_int = 0x06;
pub const VK_BACK: c_int = 0x08;
pub const VK_TAB: c_int = 0x09;
pub const VK_CLEAR: c_int = 0x0C;
pub const VK_RETURN: c_int = 0x0D;
pub const VK_SHIFT: c_int = 0x10;
pub const VK_CONTROL: c_int = 0x11;
pub const VK_MENU: c_int = 0x12;
pub const VK_PAUSE: c_int = 0x13;
pub const VK_CAPITAL: c_int = 0x14;
pub const VK_KANA: c_int = 0x15;
pub const VK_HANGEUL: c_int = 0x15;
pub const VK_HANGUL: c_int = 0x15;
pub const VK_JUNJA: c_int = 0x17;
pub const VK_FINAL: c_int = 0x18;
pub const VK_HANJA: c_int = 0x19;
pub const VK_KANJI: c_int = 0x19;
pub const VK_ESCAPE: c_int = 0x1B;
pub const VK_CONVERT: c_int = 0x1C;
pub const VK_NONCONVERT: c_int = 0x1D;
pub const VK_ACCEPT: c_int = 0x1E;
pub const VK_MODECHANGE: c_int = 0x1F;
pub const VK_SPACE: c_int = 0x20;
pub const VK_PRIOR: c_int = 0x21;
pub const VK_NEXT: c_int = 0x22;
pub const VK_END: c_int = 0x23;
pub const VK_HOME: c_int = 0x24;
pub const VK_LEFT: c_int = 0x25;
pub const VK_UP: c_int = 0x26;
pub const VK_RIGHT: c_int = 0x27;
pub const VK_DOWN: c_int = 0x28;
pub const VK_SELECT: c_int = 0x29;
pub const VK_PRINT: c_int = 0x2A;
pub const VK_EXECUTE: c_int = 0x2B;
pub const VK_SNAPSHOT: c_int = 0x2C;
pub const VK_INSERT: c_int = 0x2D;
pub const VK_DELETE: c_int = 0x2E;
pub const VK_HELP: c_int = 0x2F;
pub const VK_LWIN: c_int = 0x5B;
pub const VK_RWIN: c_int = 0x5C;
pub const VK_APPS: c_int = 0x5D;
pub const VK_SLEEP: c_int = 0x5F;
pub const VK_NUMPAD0: c_int = 0x60;
pub const VK_NUMPAD1: c_int = 0x61;
pub const VK_NUMPAD2: c_int = 0x62;
pub const VK_NUMPAD3: c_int = 0x63;
pub const VK_NUMPAD4: c_int = 0x64;
pub const VK_NUMPAD5: c_int = 0x65;
pub const VK_NUMPAD6: c_int = 0x66;
pub const VK_NUMPAD7: c_int = 0x67;
pub const VK_NUMPAD8: c_int = 0x68;
pub const VK_NUMPAD9: c_int = 0x69;
pub const VK_MULTIPLY: c_int = 0x6A;
pub const VK_ADD: c_int = 0x6B;
pub const VK_SEPARATOR: c_int = 0x6C;
pub const VK_SUBTRACT: c_int = 0x6D;
pub const VK_DECIMAL: c_int = 0x6E;
pub const VK_DIVIDE: c_int = 0x6F;
pub const VK_F1: c_int = 0x70;
pub const VK_F2: c_int = 0x71;
pub const VK_F3: c_int = 0x72;
pub const VK_F4: c_int = 0x73;
pub const VK_F5: c_int = 0x74;
pub const VK_F6: c_int = 0x75;
pub const VK_F7: c_int = 0x76;
pub const VK_F8: c_int = 0x77;
pub const VK_F9: c_int = 0x78;
pub const VK_F10: c_int = 0x79;
pub const VK_F11: c_int = 0x7A;
pub const VK_F12: c_int = 0x7B;
pub const VK_F13: c_int = 0x7C;
pub const VK_F14: c_int = 0x7D;
pub const VK_F15: c_int = 0x7E;
pub const VK_F16: c_int = 0x7F;
pub const VK_F17: c_int = 0x80;
pub const VK_F18: c_int = 0x81;
pub const VK_F19: c_int = 0x82;
pub const VK_F20: c_int = 0x83;
pub const VK_F21: c_int = 0x84;
pub const VK_F22: c_int = 0x85;
pub const VK_F23: c_int = 0x86;
pub const VK_F24: c_int = 0x87;
pub const VK_NAVIGATION_VIEW: c_int = 0x88;
pub const VK_NAVIGATION_MENU: c_int = 0x89;
pub const VK_NAVIGATION_UP: c_int = 0x8A;
pub const VK_NAVIGATION_DOWN: c_int = 0x8B;
pub const VK_NAVIGATION_LEFT: c_int = 0x8C;
pub const VK_NAVIGATION_RIGHT: c_int = 0x8D;
pub const VK_NAVIGATION_ACCEPT: c_int = 0x8E;
pub const VK_NAVIGATION_CANCEL: c_int = 0x8F;
pub const VK_NUMLOCK: c_int = 0x90;
pub const VK_SCROLL: c_int = 0x91;
pub const VK_OEM_NEC_EQUAL: c_int = 0x92;
pub const VK_OEM_FJ_JISHO: c_int = 0x92;
pub const VK_OEM_FJ_MASSHOU: c_int = 0x93;
pub const VK_OEM_FJ_TOUROKU: c_int = 0x94;
pub const VK_OEM_FJ_LOYA: c_int = 0x95;
pub const VK_OEM_FJ_ROYA: c_int = 0x96;
pub const VK_LSHIFT: c_int = 0xA0;
pub const VK_RSHIFT: c_int = 0xA1;
pub const VK_LCONTROL: c_int = 0xA2;
pub const VK_RCONTROL: c_int = 0xA3;
pub const VK_LMENU: c_int = 0xA4;
pub const VK_RMENU: c_int = 0xA5;
pub const VK_BROWSER_BACK: c_int = 0xA6;
pub const VK_BROWSER_FORWARD: c_int = 0xA7;
pub const VK_BROWSER_REFRESH: c_int = 0xA8;
pub const VK_BROWSER_STOP: c_int = 0xA9;
pub const VK_BROWSER_SEARCH: c_int = 0xAA;
pub const VK_BROWSER_FAVORITES: c_int = 0xAB;
pub const VK_BROWSER_HOME: c_int = 0xAC;
pub const VK_VOLUME_MUTE: c_int = 0xAD;
pub const VK_VOLUME_DOWN: c_int = 0xAE;
pub const VK_VOLUME_UP: c_int = 0xAF;
pub const VK_MEDIA_NEXT_TRACK: c_int = 0xB0;
pub const VK_MEDIA_PREV_TRACK: c_int = 0xB1;
pub const VK_MEDIA_STOP: c_int = 0xB2;
pub const VK_MEDIA_PLAY_PAUSE: c_int = 0xB3;
pub const VK_LAUNCH_MAIL: c_int = 0xB4;
pub const VK_LAUNCH_MEDIA_SELECT: c_int = 0xB5;
pub const VK_LAUNCH_APP1: c_int = 0xB6;
pub const VK_LAUNCH_APP2: c_int = 0xB7;
pub const VK_OEM_1: c_int = 0xBA;
pub const VK_OEM_PLUS: c_int = 0xBB;
pub const VK_OEM_COMMA: c_int = 0xBC;
pub const VK_OEM_MINUS: c_int = 0xBD;
pub const VK_OEM_PERIOD: c_int = 0xBE;
pub const VK_OEM_2: c_int = 0xBF;
pub const VK_OEM_3: c_int = 0xC0;
pub const VK_GAMEPAD_A: c_int = 0xC3;
pub const VK_GAMEPAD_B: c_int = 0xC4;
pub const VK_GAMEPAD_X: c_int = 0xC5;
pub const VK_GAMEPAD_Y: c_int = 0xC6;
pub const VK_GAMEPAD_RIGHT_SHOULDER: c_int = 0xC7;
pub const VK_GAMEPAD_LEFT_SHOULDER: c_int = 0xC8;
pub const VK_GAMEPAD_LEFT_TRIGGER: c_int = 0xC9;
pub const VK_GAMEPAD_RIGHT_TRIGGER: c_int = 0xCA;
pub const VK_GAMEPAD_DPAD_UP: c_int = 0xCB;
pub const VK_GAMEPAD_DPAD_DOWN: c_int = 0xCC;
pub const VK_GAMEPAD_DPAD_LEFT: c_int = 0xCD;
pub const VK_GAMEPAD_DPAD_RIGHT: c_int = 0xCE;
pub const VK_GAMEPAD_MENU: c_int = 0xCF;
pub const VK_GAMEPAD_VIEW: c_int = 0xD0;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON: c_int = 0xD1;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: c_int = 0xD2;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_UP: c_int = 0xD3;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_DOWN: c_int = 0xD4;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT: c_int = 0xD5;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_LEFT: c_int = 0xD6;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_UP: c_int = 0xD7;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN: c_int = 0xD8;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: c_int = 0xD9;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT: c_int = 0xDA;
pub const VK_OEM_4: c_int = 0xDB;
pub const VK_OEM_5: c_int = 0xDC;
pub const VK_OEM_6: c_int = 0xDD;
pub const VK_OEM_7: c_int = 0xDE;
pub const VK_OEM_8: c_int = 0xDF;
pub const VK_OEM_AX: c_int = 0xE1;
pub const VK_OEM_102: c_int = 0xE2;
pub const VK_ICO_HELP: c_int = 0xE3;
pub const VK_ICO_00: c_int = 0xE4;
pub const VK_PROCESSKEY: c_int = 0xE5;
pub const VK_ICO_CLEAR: c_int = 0xE6;
pub const VK_PACKET: c_int = 0xE7;
pub const VK_OEM_RESET: c_int = 0xE9;
pub const VK_OEM_JUMP: c_int = 0xEA;
pub const VK_OEM_PA1: c_int = 0xEB;
pub const VK_OEM_PA2: c_int = 0xEC;
pub const VK_OEM_PA3: c_int = 0xED;
pub const VK_OEM_WSCTRL: c_int = 0xEE;
pub const VK_OEM_CUSEL: c_int = 0xEF;
pub const VK_OEM_ATTN: c_int = 0xF0;
pub const VK_OEM_FINISH: c_int = 0xF1;
pub const VK_OEM_COPY: c_int = 0xF2;
pub const VK_OEM_AUTO: c_int = 0xF3;
pub const VK_OEM_ENLW: c_int = 0xF4;
pub const VK_OEM_BACKTAB: c_int = 0xF5;
pub const VK_ATTN: c_int = 0xF6;
pub const VK_CRSEL: c_int = 0xF7;
pub const VK_EXSEL: c_int = 0xF8;
pub const VK_EREOF: c_int = 0xF9;
pub const VK_PLAY: c_int = 0xFA;
pub const VK_ZOOM: c_int = 0xFB;
pub const VK_NONAME: c_int = 0xFC;
pub const VK_PA1: c_int = 0xFD;
pub const VK_OEM_CLEAR: c_int = 0xFE;
pub const WH_MIN: c_int = -1;
pub const WH_MSGFILTER: c_int = -1;
pub const WH_JOURNALRECORD: c_int = 0;
pub const WH_JOURNALPLAYBACK: c_int = 1;
pub const WH_KEYBOARD: c_int = 2;
pub const WH_GETMESSAGE: c_int = 3;
pub const WH_CALLWNDPROC: c_int = 4;
pub const WH_CBT: c_int = 5;
pub const WH_SYSMSGFILTER: c_int = 6;
pub const WH_MOUSE: c_int = 7;
pub const WH_HARDWARE: c_int = 8;
pub const WH_DEBUG: c_int = 9;
pub const WH_SHELL: c_int = 10;
pub const WH_FOREGROUNDIDLE: c_int = 11;
pub const WH_CALLWNDPROCRET: c_int = 12;
pub const WH_KEYBOARD_LL: c_int = 13;
pub const WH_MOUSE_LL: c_int = 14;
pub const WH_MAX: c_int = 14;
pub const WH_MINHOOK: c_int = WH_MIN;
pub const WH_MAXHOOK: c_int = WH_MAX;
pub const HC_ACTION: c_int = 0;
pub const HC_GETNEXT: c_int = 1;
pub const HC_SKIP: c_int = 2;
pub const HC_NOREMOVE: c_int = 3;
pub const HC_NOREM: c_int = HC_NOREMOVE;
pub const HC_SYSMODALON: c_int = 4;
pub const HC_SYSMODALOFF: c_int = 5;
pub const HCBT_MOVESIZE: c_int = 0;
pub const HCBT_MINMAX: c_int = 1;
pub const HCBT_QS: c_int = 2;
pub const HCBT_CREATEWND: c_int = 3;
pub const HCBT_DESTROYWND: c_int = 4;
pub const HCBT_ACTIVATE: c_int = 5;
pub const HCBT_CLICKSKIPPED: c_int = 6;
pub const HCBT_KEYSKIPPED: c_int = 7;
pub const HCBT_SYSCOMMAND: c_int = 8;
pub const HCBT_SETFOCUS: c_int = 9;
STRUCT!{struct CBT_CREATEWNDA {
    lpcs: *mut CREATESTRUCTA,
    hwndInsertAfter: HWND,
}}
pub type LPCBT_CREATEWNDA = *mut CBT_CREATEWNDA;
STRUCT!{struct CBT_CREATEWNDW {
    lpcs: *mut CREATESTRUCTW,
    hwndInsertAfter: HWND,
}}
pub type LPCBT_CREATEWNDW = *mut CBT_CREATEWNDW;
STRUCT!{struct CBTACTIVATESTRUCT {
    fMouse: BOOL,
    hWndActive: HWND,
}}
pub type LPCBTACTIVATESTRUCT = *mut CBTACTIVATESTRUCT;
STRUCT!{struct WTSSESSION_NOTIFICATION {
    cbSize: DWORD,
    dwSessionId: DWORD,
}}
pub type PWTSSESSION_NOTIFICATION = *mut WTSSESSION_NOTIFICATION;
pub const WTS_CONSOLE_CONNECT: WPARAM = 0x1;
pub const WTS_CONSOLE_DISCONNECT: WPARAM = 0x2;
pub const WTS_REMOTE_CONNECT: WPARAM = 0x3;
pub const WTS_REMOTE_DISCONNECT: WPARAM = 0x4;
pub const WTS_SESSION_LOGON: WPARAM = 0x5;
pub const WTS_SESSION_LOGOFF: WPARAM = 0x6;
pub const WTS_SESSION_LOCK: WPARAM = 0x7;
pub const WTS_SESSION_UNLOCK: WPARAM = 0x8;
pub const WTS_SESSION_REMOTE_CONTROL: WPARAM = 0x9;
pub const WTS_SESSION_CREATE: WPARAM = 0xa;
pub const WTS_SESSION_TERMINATE: WPARAM = 0xb;
pub const MSGF_DIALOGBOX: c_int = 0;
pub const MSGF_MESSAGEBOX: c_int = 1;
pub const MSGF_MENU: c_int = 2;
pub const MSGF_SCROLLBAR: c_int = 5;
pub const MSGF_NEXTWINDOW: c_int = 6;
pub const MSGF_MAX: c_int = 8;
pub const MSGF_USER: c_int = 4096;
pub const HSHELL_WINDOWCREATED: c_int = 1;
pub const HSHELL_WINDOWDESTROYED: c_int = 2;
pub const HSHELL_ACTIVATESHELLWINDOW: c_int = 3;
pub const HSHELL_WINDOWACTIVATED: c_int = 4;
pub const HSHELL_GETMINRECT: c_int = 5;
pub const HSHELL_REDRAW: c_int = 6;
pub const HSHELL_TASKMAN: c_int = 7;
pub const HSHELL_LANGUAGE: c_int = 8;
pub const HSHELL_SYSMENU: c_int = 9;
pub const HSHELL_ENDTASK: c_int = 10;
pub const HSHELL_ACCESSIBILITYSTATE: c_int = 11;
pub const HSHELL_APPCOMMAND: c_int = 12;
pub const HSHELL_WINDOWREPLACED: c_int = 13;
pub const HSHELL_WINDOWREPLACING: c_int = 14;
pub const HSHELL_MONITORCHANGED: c_int = 16;
pub const HSHELL_HIGHBIT: c_int = 0x8000;
pub const HSHELL_FLASH: c_int = HSHELL_REDRAW | HSHELL_HIGHBIT;
pub const HSHELL_RUDEAPPACTIVATED: c_int = HSHELL_WINDOWACTIVATED | HSHELL_HIGHBIT;
pub const APPCOMMAND_BROWSER_BACKWARD: c_short = 1;
pub const APPCOMMAND_BROWSER_FORWARD: c_short = 2;
pub const APPCOMMAND_BROWSER_REFRESH: c_short = 3;
pub const APPCOMMAND_BROWSER_STOP: c_short = 4;
pub const APPCOMMAND_BROWSER_SEARCH: c_short = 5;
pub const APPCOMMAND_BROWSER_FAVORITES: c_short = 6;
pub const APPCOMMAND_BROWSER_HOME: c_short = 7;
pub const APPCOMMAND_VOLUME_MUTE: c_short = 8;
pub const APPCOMMAND_VOLUME_DOWN: c_short = 9;
pub const APPCOMMAND_VOLUME_UP: c_short = 10;
pub const APPCOMMAND_MEDIA_NEXTTRACK: c_short = 11;
pub const APPCOMMAND_MEDIA_PREVIOUSTRACK: c_short = 12;
pub const APPCOMMAND_MEDIA_STOP: c_short = 13;
pub const APPCOMMAND_MEDIA_PLAY_PAUSE: c_short = 14;
pub const APPCOMMAND_LAUNCH_MAIL: c_short = 15;
pub const APPCOMMAND_LAUNCH_MEDIA_SELECT: c_short = 16;
pub const APPCOMMAND_LAUNCH_APP1: c_short = 17;
pub const APPCOMMAND_LAUNCH_APP2: c_short = 18;
pub const APPCOMMAND_BASS_DOWN: c_short = 19;
pub const APPCOMMAND_BASS_BOOST: c_short = 20;
pub const APPCOMMAND_BASS_UP: c_short = 21;
pub const APPCOMMAND_TREBLE_DOWN: c_short = 22;
pub const APPCOMMAND_TREBLE_UP: c_short = 23;
pub const APPCOMMAND_MICROPHONE_VOLUME_MUTE: c_short = 24;
pub const APPCOMMAND_MICROPHONE_VOLUME_DOWN: c_short = 25;
pub const APPCOMMAND_MICROPHONE_VOLUME_UP: c_short = 26;
pub const APPCOMMAND_HELP: c_short = 27;
pub const APPCOMMAND_FIND: c_short = 28;
pub const APPCOMMAND_NEW: c_short = 29;
pub const APPCOMMAND_OPEN: c_short = 30;
pub const APPCOMMAND_CLOSE: c_short = 31;
pub const APPCOMMAND_SAVE: c_short = 32;
pub const APPCOMMAND_PRINT: c_short = 33;
pub const APPCOMMAND_UNDO: c_short = 34;
pub const APPCOMMAND_REDO: c_short = 35;
pub const APPCOMMAND_COPY: c_short = 36;
pub const APPCOMMAND_CUT: c_short = 37;
pub const APPCOMMAND_PASTE: c_short = 38;
pub const APPCOMMAND_REPLY_TO_MAIL: c_short = 39;
pub const APPCOMMAND_FORWARD_MAIL: c_short = 40;
pub const APPCOMMAND_SEND_MAIL: c_short = 41;
pub const APPCOMMAND_SPELL_CHECK: c_short = 42;
pub const APPCOMMAND_DICTATE_OR_COMMAND_CONTROL_TOGGLE: c_short = 43;
pub const APPCOMMAND_MIC_ON_OFF_TOGGLE: c_short = 44;
pub const APPCOMMAND_CORRECTION_LIST: c_short = 45;
pub const APPCOMMAND_MEDIA_PLAY: c_short = 46;
pub const APPCOMMAND_MEDIA_PAUSE: c_short = 47;
pub const APPCOMMAND_MEDIA_RECORD: c_short = 48;
pub const APPCOMMAND_MEDIA_FAST_FORWARD: c_short = 49;
pub const APPCOMMAND_MEDIA_REWIND: c_short = 50;
pub const APPCOMMAND_MEDIA_CHANNEL_UP: c_short = 51;
pub const APPCOMMAND_MEDIA_CHANNEL_DOWN: c_short = 52;
pub const APPCOMMAND_DELETE: c_short = 53;
pub const APPCOMMAND_DWM_FLIP3D: c_short = 54;
pub const FAPPCOMMAND_MOUSE: WORD = 0x8000;
pub const FAPPCOMMAND_KEY: WORD = 0;
pub const FAPPCOMMAND_OEM: WORD = 0x1000;
pub const FAPPCOMMAND_MASK: WORD = 0xF000;
#[inline]
pub fn GET_APPCOMMAND_LPARAM(lParam: LPARAM) -> c_short {
    (HIWORD(lParam as DWORD) & !FAPPCOMMAND_MASK) as c_short
}
#[inline]
pub fn GET_DEVICE_LPARAM(lParam: LPARAM) -> WORD {
    HIWORD(lParam as DWORD) & FAPPCOMMAND_MASK
}
pub use self::GET_DEVICE_LPARAM as GET_MOUSEORKEY_LPARAM;
pub use shared::minwindef::LOWORD as GET_FLAGS_LPARAM;
pub use self::GET_FLAGS_LPARAM as GET_KEYSTATE_LPARAM;
STRUCT!{struct SHELLHOOKINFO {
    hwnd: HWND,
    rc: RECT,
}}
pub type LPSHELLHOOKINFO = *mut SHELLHOOKINFO;
STRUCT!{struct EVENTMSG {
    message: UINT,
    paramL: UINT,
    paramH: UINT,
    time: DWORD,
    hwnd: HWND,
}}
pub type PEVENTMSGMSG = *mut EVENTMSG;
pub type NPEVENTMSGMSG = *mut EVENTMSG;
pub type LPEVENTMSGMSG = *mut EVENTMSG;
pub type PEVENTMSG = *mut EVENTMSG;
pub type NPEVENTMSG = *mut EVENTMSG;
pub type LPEVENTMSG = *mut EVENTMSG;
STRUCT!{struct CWPSTRUCT {
    lParam: LPARAM,
    wParam: WPARAM,
    message: UINT,
    hwnd: HWND,
}}
pub type PCWPSTRUCT = *mut CWPSTRUCT;
pub type NPCWPSTRUCT = *mut CWPSTRUCT;
pub type LPCWPSTRUCT = *mut CWPSTRUCT;
STRUCT!{struct CWPRETSTRUCT {
    lResult: LRESULT,
    lParam: LPARAM,
    wParam: WPARAM,
    message: UINT,
    hwnd: HWND,
}}
pub type PCWPRETSTRUCT = *mut CWPRETSTRUCT;
pub type NPCWPRETSTRUCT = *mut CWPRETSTRUCT;
pub type LPCWPRETSTRUCT = *mut CWPRETSTRUCT;
pub const LLKHF_EXTENDED: DWORD = (KF_EXTENDED >> 8) as u32;
pub const LLKHF_INJECTED: DWORD = 0x00000010;
pub const LLKHF_ALTDOWN: DWORD = (KF_ALTDOWN >> 8) as u32;
pub const LLKHF_UP: DWORD = (KF_UP >> 8) as u32;
pub const LLKHF_LOWER_IL_INJECTED: DWORD = 0x00000002;
pub const LLMHF_INJECTED: DWORD = 0x00000001;
pub const LLMHF_LOWER_IL_INJECTED: DWORD = 0x00000002;
STRUCT!{struct KBDLLHOOKSTRUCT {
    vkCode: DWORD,
    scanCode: DWORD,
    flags: DWORD,
    time: DWORD,
    dwExtraInfo: ULONG_PTR,
}}
pub type LPKBDLLHOOKSTRUCT = *mut KBDLLHOOKSTRUCT;
pub type PKBDLLHOOKSTRUCT = *mut KBDLLHOOKSTRUCT;
STRUCT!{struct MSLLHOOKSTRUCT {
    pt: POINT,
    mouseData: DWORD,
    flags: DWORD,
    time: DWORD,
    dwExtraInfo: ULONG_PTR,
}}
pub type LPMSLLHOOKSTRUCT = *mut MSLLHOOKSTRUCT;
pub type PMSLLHOOKSTRUCT = *mut MSLLHOOKSTRUCT;
STRUCT!{struct DEBUGHOOKINFO {
    idThread: DWORD,
    idThreadInstaller: DWORD,
    lParam: LPARAM,
    wParam: WPARAM,
    code: c_int,
}}
pub type PDEBUGHOOKINFO = *mut DEBUGHOOKINFO;
pub type NPDEBUGHOOKINFO = *mut DEBUGHOOKINFO;
pub type LPDEBUGHOOKINFO = *mut DEBUGHOOKINFO;
STRUCT!{struct MOUSEHOOKSTRUCT {
    pt: POINT,
    hwnd: HWND,
    wHitTestCode: UINT,
    dwExtraInfo: ULONG_PTR,
}}
pub type LPMOUSEHOOKSTRUCT = *mut MOUSEHOOKSTRUCT;
pub type PMOUSEHOOKSTRUCT = *mut MOUSEHOOKSTRUCT;
STRUCT!{struct MOUSEHOOKSTRUCTEX {
    parent: MOUSEHOOKSTRUCT,
    mouseData: DWORD,
}}
pub type LPMOUSEHOOKSTRUCTEX = *mut MOUSEHOOKSTRUCTEX;
pub type PMOUSEHOOKSTRUCTEX = *mut MOUSEHOOKSTRUCTEX;
STRUCT!{struct HARDWAREHOOKSTRUCT {
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
}}
pub type LPHARDWAREHOOKSTRUCT = *mut HARDWAREHOOKSTRUCT;
pub type PHARDWAREHOOKSTRUCT = *mut HARDWAREHOOKSTRUCT;
pub const HKL_PREV: HKL = 0 as HKL;
pub const HKL_NEXT: HKL = 1 as HKL;
pub const KLF_ACTIVATE: UINT = 0x00000001;
pub const KLF_SUBSTITUTE_OK: UINT = 0x00000002;
pub const KLF_REORDER: UINT = 0x00000008;
pub const KLF_REPLACELANG: UINT = 0x00000010;
pub const KLF_NOTELLSHELL: UINT = 0x00000080;
pub const KLF_SETFORPROCESS: UINT = 0x00000100;
pub const KLF_SHIFTLOCK: UINT = 0x00010000;
pub const KLF_RESET: UINT = 0x40000000;
pub const INPUTLANGCHANGE_SYSCHARSET: WPARAM = 0x0001;
pub const INPUTLANGCHANGE_FORWARD: WPARAM = 0x0002;
pub const INPUTLANGCHANGE_BACKWARD: WPARAM = 0x0004;
pub const KL_NAMELENGTH: usize = 9;
extern "system" {
    pub fn LoadKeyboardLayoutA(
        pwszKLID: LPCSTR,
        Flags: DWORD,
    ) -> HKL;
    pub fn LoadKeyboardLayoutW(
        pwszKLID: LPCWSTR,
        Flags: DWORD,
    ) -> HKL;
    pub fn ActivateKeyboardLayout(
        hkl: HKL,
        Flags: UINT,
    ) -> HKL;
    pub fn ToUnicodeEx(
        wVirtKey: UINT,
        wScanCode: UINT,
        lpKeyState: *const BYTE,
        pwszBuff: LPWSTR,
        cchBuff: c_int,
        wFlags: UINT,
        dwhkl: HKL,
    ) -> c_int;
    pub fn UnloadKeyboardLayout(
        hkl: HKL,
    ) -> BOOL;
    pub fn GetKeyboardLayoutNameA(
        pwszKLID: LPSTR,
    ) -> BOOL;
    pub fn GetKeyboardLayoutNameW(
        pwszKLID: LPWSTR,
    ) -> BOOL;
    pub fn GetKeyboardLayoutList(
        nBuff: c_int,
        lpList: *mut HKL,
    ) -> c_int;
    pub fn GetKeyboardLayout(
        idThread: DWORD,
    ) -> HKL;
}
STRUCT!{struct MOUSEMOVEPOINT {
    x: c_int,
    y: c_int,
    time: DWORD,
    dwExtraInfo: ULONG_PTR,
}}
pub type PMOUSEMOVEPOINT = *mut MOUSEMOVEPOINT;
pub type LPMOUSEMOVEPOINT = *mut MOUSEMOVEPOINT;
pub const GMMP_USE_DISPLAY_POINTS: DWORD = 1;
pub const GMMP_USE_HIGH_RESOLUTION_POINTS: DWORD = 2;
extern "system" {
    pub fn GetMouseMovePointsEx(
        cbSize: UINT,
        lppt: LPMOUSEMOVEPOINT,
        lpptBuf: LPMOUSEMOVEPOINT,
        nBufPoints: c_int,
        resolution: DWORD,
    ) -> c_int;
}
pub const DESKTOP_READOBJECTS: DWORD = 0x0001;
pub const DESKTOP_CREATEWINDOW: DWORD = 0x0002;
pub const DESKTOP_CREATEMENU: DWORD = 0x0004;
pub const DESKTOP_HOOKCONTROL: DWORD = 0x0008;
pub const DESKTOP_JOURNALRECORD: DWORD = 0x0010;
pub const DESKTOP_JOURNALPLAYBACK: DWORD = 0x0020;
pub const DESKTOP_ENUMERATE: DWORD = 0x0040;
pub const DESKTOP_WRITEOBJECTS: DWORD = 0x0080;
pub const DESKTOP_SWITCHDESKTOP: DWORD = 0x0100;
pub const DF_ALLOWOTHERACCOUNTHOOK: DWORD = 0x0001;
extern "system" {
    pub fn CreateDesktopA(
        lpszDesktop: LPCSTR,
        lpszDevice: LPCSTR,
        pDevmode: *mut DEVMODEA,
        dwFlags: DWORD,
        dwDesiredAccess: ACCESS_MASK,
        lpsa: LPSECURITY_ATTRIBUTES,
    ) -> HDESK;
    pub fn CreateDesktopW(
        lpszDesktop: LPCWSTR,
        lpszDevice: LPCWSTR,
        pDevmode: *mut DEVMODEW,
        dwFlags: DWORD,
        dwDesiredAccess: ACCESS_MASK,
        lpsa: LPSECURITY_ATTRIBUTES,
    ) -> HDESK;
    pub fn CreateDesktopExA(
        lpszDesktop: LPCSTR,
        lpszDevice: LPCSTR,
        pDevmode: *mut DEVMODEA,
        dwFlags: DWORD,
        dwDesiredAccess: ACCESS_MASK,
        lpsa: LPSECURITY_ATTRIBUTES,
        ulHeapSize: ULONG,
        pvoid: PVOID,
    ) -> HDESK;
    pub fn CreateDesktopExW(
        lpszDesktop: LPCWSTR,
        lpszDevice: LPCWSTR,
        pDevmode: *mut DEVMODEW,
        dwFlags: DWORD,
        dwDesiredAccess: ACCESS_MASK,
        lpsa: LPSECURITY_ATTRIBUTES,
        ulHeapSize: ULONG,
        pvoid: PVOID,
    ) -> HDESK;
    pub fn OpenDesktopA(
        lpszDesktop: LPCSTR,
        dwFlags: DWORD,
        fInherit: BOOL,
        dwDesiredAccess: ACCESS_MASK,
    ) -> HDESK;
    pub fn OpenDesktopW(
        lpszDesktop: LPCWSTR,
        dwFlags: DWORD,
        fInherit: BOOL,
        dwDesiredAccess: ACCESS_MASK,
    ) -> HDESK;
    pub fn OpenInputDesktop(
        dwFlags: DWORD,
        fInherit: BOOL,
        dwDesiredAccess: ACCESS_MASK,
    ) -> HDESK;
    pub fn EnumDesktopsA(
        hwinsta: HWINSTA,
        lpEnumFunc: DESKTOPENUMPROCA,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn EnumDesktopsW(
        hwinsta: HWINSTA,
        lpEnumFunc: DESKTOPENUMPROCW,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn EnumDesktopWindows(
        hDesktop: HDESK,
        lpfn: WNDENUMPROC,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn SwitchDesktop(
        hDesktop: HDESK,
    ) -> BOOL;
    pub fn SetThreadDesktop(
        hDesktop: HDESK,
    ) -> BOOL;
    pub fn CloseDesktop(
        hDesktop: HDESK,
    ) -> BOOL;
    pub fn GetThreadDesktop(
        dwThreadId: DWORD,
    ) -> HDESK;
}
pub const WINSTA_ENUMDESKTOPS: DWORD = 0x0001;
pub const WINSTA_READATTRIBUTES: DWORD = 0x0002;
pub const WINSTA_ACCESSCLIPBOARD: DWORD = 0x0004;
pub const WINSTA_CREATEDESKTOP: DWORD = 0x0008;
pub const WINSTA_WRITEATTRIBUTES: DWORD = 0x0010;
pub const WINSTA_ACCESSGLOBALATOMS: DWORD = 0x0020;
pub const WINSTA_EXITWINDOWS: DWORD = 0x0040;
pub const WINSTA_ENUMERATE: DWORD = 0x0100;
pub const WINSTA_READSCREEN: DWORD = 0x0200;
pub const WINSTA_ALL_ACCESS: DWORD = WINSTA_ENUMDESKTOPS | WINSTA_READATTRIBUTES
    | WINSTA_ACCESSCLIPBOARD | WINSTA_CREATEDESKTOP | WINSTA_WRITEATTRIBUTES
    | WINSTA_ACCESSGLOBALATOMS | WINSTA_EXITWINDOWS | WINSTA_ENUMERATE | WINSTA_READSCREEN;
pub const CWF_CREATE_ONLY: DWORD = 0x00000001;
pub const WSF_VISIBLE: DWORD = 0x0001;
extern "system" {
    pub fn CreateWindowStationA(
        lpwinsta: LPCSTR,
        dwFlags: DWORD,
        dwDesiredAccess: ACCESS_MASK,
        lpsa: LPSECURITY_ATTRIBUTES,
    ) -> HWINSTA;
    pub fn CreateWindowStationW(
        lpwinsta: LPCWSTR,
        dwFlags: DWORD,
        dwDesiredAccess: ACCESS_MASK,
        lpsa: LPSECURITY_ATTRIBUTES,
    ) -> HWINSTA;
    pub fn OpenWindowStationA(
        lpszWinSta: LPCSTR,
        fInherit: BOOL,
        dwDesiredAccess: ACCESS_MASK,
    ) -> HWINSTA;
    pub fn OpenWindowStationW(
        lpszWinSta: LPCWSTR,
        fInherit: BOOL,
        dwDesiredAccess: ACCESS_MASK,
    ) -> HWINSTA;
    pub fn EnumWindowStationsA(
        lpEnumFunc: WINSTAENUMPROCA,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn EnumWindowStationsW(
        lpEnumFunc: WINSTAENUMPROCW,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn CloseWindowStation(
        hWinSta: HWINSTA,
    ) -> BOOL;
    pub fn SetProcessWindowStation(
        hWinSta: HWINSTA,
    ) -> BOOL;
    pub fn GetProcessWindowStation() -> HWINSTA;
    pub fn SetUserObjectSecurity(
        hObj: HANDLE,
        pSIRequested: PSECURITY_INFORMATION,
        pSID: PSECURITY_DESCRIPTOR,
    ) -> BOOL;
    pub fn GetUserObjectSecurity(
        hObj: HANDLE,
        pSIRequested: PSECURITY_INFORMATION,
        pSID: PSECURITY_DESCRIPTOR,
        nLength: DWORD,
        lpnLengthNeeded: LPDWORD,
    ) -> BOOL;
}
pub const UOI_FLAGS: DWORD = 1;
pub const UOI_NAME: DWORD = 2;
pub const UOI_TYPE: DWORD = 3;
pub const UOI_USER_SID: DWORD = 4;
pub const UOI_HEAPSIZE: DWORD = 5;
pub const UOI_IO: DWORD = 6;
pub const UOI_TIMERPROC_EXCEPTION_SUPPRESSION: DWORD = 7;
STRUCT!{struct USEROBJECTFLAGS {
    fInherit: BOOL,
    fReserved: BOOL,
    dwFlags: DWORD,
}}
pub type PUSEROBJECTFLAGS = *mut USEROBJECTFLAGS;
extern "system" {
    pub fn GetUserObjectInformationA(
        hObj: HANDLE,
        nIndex: c_int,
        pvInfo: PVOID,
        nLength: DWORD,
        lpnLengthNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetUserObjectInformationW(
        hObj: HANDLE,
        nIndex: c_int,
        pvInfo: PVOID,
        nLength: DWORD,
        lpnLengthNeeded: LPDWORD,
    ) -> BOOL;
    pub fn SetUserObjectInformationA(
        hObj: HANDLE,
        nIndex: c_int,
        pvInfo: PVOID,
        nLength: DWORD,
    ) -> BOOL;
    pub fn SetUserObjectInformationW(
        hObj: HANDLE,
        nIndex: c_int,
        pvInfo: PVOID,
        nLength: DWORD,
    ) -> BOOL;
}
STRUCT!{struct WNDCLASSEXA {
    cbSize: UINT,
    style: UINT,
    lpfnWndProc: WNDPROC,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCSTR,
    lpszClassName: LPCSTR,
    hIconSm: HICON,
}}
pub type PWNDCLASSEXA = *mut WNDCLASSEXA;
pub type NPWNDCLASSEXA = *mut WNDCLASSEXA;
pub type LPWNDCLASSEXA = *mut WNDCLASSEXA;
STRUCT!{struct WNDCLASSEXW {
    cbSize: UINT,
    style: UINT,
    lpfnWndProc: WNDPROC,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCWSTR,
    lpszClassName: LPCWSTR,
    hIconSm: HICON,
}}
pub type PWNDCLASSEXW = *mut WNDCLASSEXW;
pub type NPWNDCLASSEXW = *mut WNDCLASSEXW;
pub type LPWNDCLASSEXW = *mut WNDCLASSEXW;
STRUCT!{struct WNDCLASSA {
    style: UINT,
    lpfnWndProc: WNDPROC,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCSTR,
    lpszClassName: LPCSTR,
}}
pub type PWNDCLASSA = *mut WNDCLASSA;
pub type NPWNDCLASSA = *mut WNDCLASSA;
pub type LPWNDCLASSA = *mut WNDCLASSA;
STRUCT!{struct WNDCLASSW {
    style: UINT,
    lpfnWndProc: WNDPROC,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCWSTR,
    lpszClassName: LPCWSTR,
}}
pub type PWNDCLASSW = *mut WNDCLASSW;
pub type NPWNDCLASSW = *mut WNDCLASSW;
pub type LPWNDCLASSW = *mut WNDCLASSW;
extern "system" {
    pub fn IsHungAppWindow(
        hwnd: HWND,
    ) -> BOOL;
    pub fn DisableProcessWindowsGhosting();
}
STRUCT!{struct MSG {
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
}}
pub type PMSG = *mut MSG;
pub type NPMSG = *mut MSG;
pub type LPMSG = *mut MSG;
//POINTSTOPOINT
//POINTTOPOINTS
//MAKEWPARAM
//MAKELPARAM
//MAKELRESULT
pub const GWL_WNDPROC: c_int = -4;
pub const GWL_HINSTANCE: c_int = -6;
pub const GWL_HWNDPARENT: c_int = -8;
pub const GWL_STYLE: c_int = -16;
pub const GWL_EXSTYLE: c_int = -20;
pub const GWL_USERDATA: c_int = -21;
pub const GWL_ID: c_int = -12;
pub const GWLP_WNDPROC: c_int = -4;
pub const GWLP_HINSTANCE: c_int = -6;
pub const GWLP_HWNDPARENT: c_int = -8;
pub const GWLP_USERDATA: c_int = -21;
pub const GWLP_ID: c_int = -12;
pub const GCL_MENUNAME: c_int = -8;
pub const GCL_HBRBACKGROUND: c_int = -10;
pub const GCL_HCURSOR: c_int = -12;
pub const GCL_HICON: c_int = -14;
pub const GCL_HMODULE: c_int = -16;
pub const GCL_CBWNDEXTRA: c_int = -18;
pub const GCL_CBCLSEXTRA: c_int = -20;
pub const GCL_WNDPROC: c_int = -24;
pub const GCL_STYLE: c_int = -26;
pub const GCW_ATOM: c_int = -32;
pub const GCL_HICONSM: c_int = -34;
pub const GCLP_MENUNAME: c_int = -8;
pub const GCLP_HBRBACKGROUND: c_int = -10;
pub const GCLP_HCURSOR: c_int = -12;
pub const GCLP_HICON: c_int = -14;
pub const GCLP_HMODULE: c_int = -16;
pub const GCLP_WNDPROC: c_int = -24;
pub const GCLP_HICONSM: c_int = -34;
pub const WM_NULL: UINT = 0x0000;
pub const WM_CREATE: UINT = 0x0001;
pub const WM_DESTROY: UINT = 0x0002;
pub const WM_MOVE: UINT = 0x0003;
pub const WM_SIZE: UINT = 0x0005;
pub const WM_ACTIVATE: UINT = 0x0006;
pub const WA_INACTIVE: WORD = 0;
pub const WA_ACTIVE: WORD = 1;
pub const WA_CLICKACTIVE: WORD = 2;
pub const WM_SETFOCUS: UINT = 0x0007;
pub const WM_KILLFOCUS: UINT = 0x0008;
pub const WM_ENABLE: UINT = 0x000A;
pub const WM_SETREDRAW: UINT = 0x000B;
pub const WM_SETTEXT: UINT = 0x000C;
pub const WM_GETTEXT: UINT = 0x000D;
pub const WM_GETTEXTLENGTH: UINT = 0x000E;
pub const WM_PAINT: UINT = 0x000F;
pub const WM_CLOSE: UINT = 0x0010;
pub const WM_QUERYENDSESSION: UINT = 0x0011;
pub const WM_QUERYOPEN: UINT = 0x0013;
pub const WM_ENDSESSION: UINT = 0x0016;
pub const WM_QUIT: UINT = 0x0012;
pub const WM_ERASEBKGND: UINT = 0x0014;
pub const WM_SYSCOLORCHANGE: UINT = 0x0015;
pub const WM_SHOWWINDOW: UINT = 0x0018;
pub const WM_WININICHANGE: UINT = 0x001A;
pub const WM_SETTINGCHANGE: UINT = WM_WININICHANGE;
pub const WM_DEVMODECHANGE: UINT = 0x001B;
pub const WM_ACTIVATEAPP: UINT = 0x001C;
pub const WM_FONTCHANGE: UINT = 0x001D;
pub const WM_TIMECHANGE: UINT = 0x001E;
pub const WM_CANCELMODE: UINT = 0x001F;
pub const WM_SETCURSOR: UINT = 0x0020;
pub const WM_MOUSEACTIVATE: UINT = 0x0021;
pub const WM_CHILDACTIVATE: UINT = 0x0022;
pub const WM_QUEUESYNC: UINT = 0x0023;
pub const WM_GETMINMAXINFO: UINT = 0x0024;
STRUCT!{struct MINMAXINFO {
    ptReserved: POINT,
    ptMaxSize: POINT,
    ptMaxPosition: POINT,
    ptMinTrackSize: POINT,
    ptMaxTrackSize: POINT,
}}
pub type PMINMAXINFO = *mut MINMAXINFO;
pub type LPMINMAXINFO = *mut MINMAXINFO;
pub const WM_PAINTICON: UINT = 0x0026;
pub const WM_ICONERASEBKGND: UINT = 0x0027;
pub const WM_NEXTDLGCTL: UINT = 0x0028;
pub const WM_SPOOLERSTATUS: UINT = 0x002A;
pub const WM_DRAWITEM: UINT = 0x002B;
pub const WM_MEASUREITEM: UINT = 0x002C;
pub const WM_DELETEITEM: UINT = 0x002D;
pub const WM_VKEYTOITEM: UINT = 0x002E;
pub const WM_CHARTOITEM: UINT = 0x002F;
pub const WM_SETFONT: UINT = 0x0030;
pub const WM_GETFONT: UINT = 0x0031;
pub const WM_SETHOTKEY: UINT = 0x0032;
pub const WM_GETHOTKEY: UINT = 0x0033;
pub const WM_QUERYDRAGICON: UINT = 0x0037;
pub const WM_COMPAREITEM: UINT = 0x0039;
pub const WM_GETOBJECT: UINT = 0x003D;
pub const WM_COMPACTING: UINT = 0x0041;
pub const WM_COMMNOTIFY: UINT = 0x0044;
pub const WM_WINDOWPOSCHANGING: UINT = 0x0046;
pub const WM_WINDOWPOSCHANGED: UINT = 0x0047;
pub const WM_POWER: UINT = 0x0048;
pub const PWR_OK: WPARAM = 1;
pub const PWR_FAIL: WPARAM = -1isize as usize;
pub const PWR_SUSPENDREQUEST: WPARAM = 1;
pub const PWR_SUSPENDRESUME: WPARAM = 2;
pub const PWR_CRITICALRESUME: WPARAM = 3;
pub const WM_COPYDATA: UINT = 0x004A;
pub const WM_CANCELJOURNAL: UINT = 0x004B;
STRUCT!{struct COPYDATASTRUCT {
    dwData: ULONG_PTR,
    cbData: DWORD,
    lpData: PVOID,
}}
pub type PCOPYDATASTRUCT = *mut COPYDATASTRUCT;
STRUCT!{struct MDINEXTMENU {
    hmenuIn: HMENU,
    hmenuNext: HMENU,
    hwndNext: HWND,
}}
pub type PMDINEXTMENU = *mut MDINEXTMENU;
pub type LPMDINEXTMENU = *mut MDINEXTMENU;
pub const WM_NOTIFY: UINT = 0x004E;
pub const WM_INPUTLANGCHANGEREQUEST: UINT = 0x0050;
pub const WM_INPUTLANGCHANGE: UINT = 0x0051;
pub const WM_TCARD: UINT = 0x0052;
pub const WM_HELP: UINT = 0x0053;
pub const WM_USERCHANGED: UINT = 0x0054;
pub const WM_NOTIFYFORMAT: UINT = 0x0055;
pub const NFR_ANSI: LRESULT = 1;
pub const NFR_UNICODE: LRESULT = 2;
pub const NF_QUERY: LPARAM = 3;
pub const NF_REQUERY: LPARAM = 4;
pub const WM_CONTEXTMENU: UINT = 0x007B;
pub const WM_STYLECHANGING: UINT = 0x007C;
pub const WM_STYLECHANGED: UINT = 0x007D;
pub const WM_DISPLAYCHANGE: UINT = 0x007E;
pub const WM_GETICON: UINT = 0x007F;
pub const WM_SETICON: UINT = 0x0080;
pub const WM_NCCREATE: UINT = 0x0081;
pub const WM_NCDESTROY: UINT = 0x0082;
pub const WM_NCCALCSIZE: UINT = 0x0083;
pub const WM_NCHITTEST: UINT = 0x0084;
pub const WM_NCPAINT: UINT = 0x0085;
pub const WM_NCACTIVATE: UINT = 0x0086;
pub const WM_GETDLGCODE: UINT = 0x0087;
pub const WM_SYNCPAINT: UINT = 0x0088;
pub const WM_NCMOUSEMOVE: UINT = 0x00A0;
pub const WM_NCLBUTTONDOWN: UINT = 0x00A1;
pub const WM_NCLBUTTONUP: UINT = 0x00A2;
pub const WM_NCLBUTTONDBLCLK: UINT = 0x00A3;
pub const WM_NCRBUTTONDOWN: UINT = 0x00A4;
pub const WM_NCRBUTTONUP: UINT = 0x00A5;
pub const WM_NCRBUTTONDBLCLK: UINT = 0x00A6;
pub const WM_NCMBUTTONDOWN: UINT = 0x00A7;
pub const WM_NCMBUTTONUP: UINT = 0x00A8;
pub const WM_NCMBUTTONDBLCLK: UINT = 0x00A9;
pub const WM_NCXBUTTONDOWN: UINT = 0x00AB;
pub const WM_NCXBUTTONUP: UINT = 0x00AC;
pub const WM_NCXBUTTONDBLCLK: UINT = 0x00AD;
pub const WM_INPUT_DEVICE_CHANGE: UINT = 0x00FE;
pub const WM_INPUT: UINT = 0x00FF;
pub const WM_KEYFIRST: UINT = 0x0100;
pub const WM_KEYDOWN: UINT = 0x0100;
pub const WM_KEYUP: UINT = 0x0101;
pub const WM_CHAR: UINT = 0x0102;
pub const WM_DEADCHAR: UINT = 0x0103;
pub const WM_SYSKEYDOWN: UINT = 0x0104;
pub const WM_SYSKEYUP: UINT = 0x0105;
pub const WM_SYSCHAR: UINT = 0x0106;
pub const WM_SYSDEADCHAR: UINT = 0x0107;
pub const WM_UNICHAR: UINT = 0x0109;
pub const WM_KEYLAST: UINT = 0x0109;
pub const UNICODE_NOCHAR: WPARAM = 0xFFFF;
pub const WM_IME_STARTCOMPOSITION: UINT = 0x010D;
pub const WM_IME_ENDCOMPOSITION: UINT = 0x010E;
pub const WM_IME_COMPOSITION: UINT = 0x010F;
pub const WM_IME_KEYLAST: UINT = 0x010F;
pub const WM_INITDIALOG: UINT = 0x0110;
pub const WM_COMMAND: UINT = 0x0111;
pub const WM_SYSCOMMAND: UINT = 0x0112;
pub const WM_TIMER: UINT = 0x0113;
pub const WM_HSCROLL: UINT = 0x0114;
pub const WM_VSCROLL: UINT = 0x0115;
pub const WM_INITMENU: UINT = 0x0116;
pub const WM_INITMENUPOPUP: UINT = 0x0117;
pub const WM_GESTURE: UINT = 0x0119;
pub const WM_GESTURENOTIFY: UINT = 0x011A;
pub const WM_MENUSELECT: UINT = 0x011F;
pub const WM_MENUCHAR: UINT = 0x0120;
pub const WM_ENTERIDLE: UINT = 0x0121;
pub const WM_MENURBUTTONUP: UINT = 0x0122;
pub const WM_MENUDRAG: UINT = 0x0123;
pub const WM_MENUGETOBJECT: UINT = 0x0124;
pub const WM_UNINITMENUPOPUP: UINT = 0x0125;
pub const WM_MENUCOMMAND: UINT = 0x0126;
pub const WM_CHANGEUISTATE: UINT = 0x0127;
pub const WM_UPDATEUISTATE: UINT = 0x0128;
pub const WM_QUERYUISTATE: UINT = 0x0129;
pub const UIS_SET: WORD = 1;
pub const UIS_CLEAR: WORD = 2;
pub const UIS_INITIALIZE: WORD = 3;
pub const UISF_HIDEFOCUS: WORD = 0x1;
pub const UISF_HIDEACCEL: WORD = 0x2;
pub const UISF_ACTIVE: WORD = 0x4;
pub const WM_CTLCOLORMSGBOX: UINT = 0x0132;
pub const WM_CTLCOLOREDIT: UINT = 0x0133;
pub const WM_CTLCOLORLISTBOX: UINT = 0x0134;
pub const WM_CTLCOLORBTN: UINT = 0x0135;
pub const WM_CTLCOLORDLG: UINT = 0x0136;
pub const WM_CTLCOLORSCROLLBAR: UINT = 0x0137;
pub const WM_CTLCOLORSTATIC: UINT = 0x0138;
pub const MN_GETHMENU: UINT = 0x01E1;
pub const WM_MOUSEFIRST: UINT = 0x0200;
pub const WM_MOUSEMOVE: UINT = 0x0200;
pub const WM_LBUTTONDOWN: UINT = 0x0201;
pub const WM_LBUTTONUP: UINT = 0x0202;
pub const WM_LBUTTONDBLCLK: UINT = 0x0203;
pub const WM_RBUTTONDOWN: UINT = 0x0204;
pub const WM_RBUTTONUP: UINT = 0x0205;
pub const WM_RBUTTONDBLCLK: UINT = 0x0206;
pub const WM_MBUTTONDOWN: UINT = 0x0207;
pub const WM_MBUTTONUP: UINT = 0x0208;
pub const WM_MBUTTONDBLCLK: UINT = 0x0209;
pub const WM_MOUSEWHEEL: UINT = 0x020A;
pub const WM_XBUTTONDOWN: UINT = 0x020B;
pub const WM_XBUTTONUP: UINT = 0x020C;
pub const WM_XBUTTONDBLCLK: UINT = 0x020D;
pub const WM_MOUSEHWHEEL: UINT = 0x020E;
pub const WM_MOUSELAST: UINT = 0x020E;
pub const WHEEL_DELTA: c_short = 120;
#[inline]
pub fn GET_WHEEL_DELTA_WPARAM(wParam: WPARAM) -> c_short {
    HIWORD(wParam as DWORD) as c_short
}
pub const WHEEL_PAGESCROLL: UINT = UINT_MAX;
#[inline]
pub fn GET_KEYSTATE_WPARAM(wParam: WPARAM) -> WORD {
    LOWORD(wParam as DWORD)
}
#[inline]
pub fn GET_NCHITTEST_WPARAM(wParam: WPARAM) -> c_short {
    LOWORD(wParam as DWORD) as c_short
}
#[inline]
pub fn GET_XBUTTON_WPARAM(wParam: WPARAM) -> WORD {
    HIWORD(wParam as DWORD)
}
pub const XBUTTON1: WORD = 0x0001;
pub const XBUTTON2: WORD = 0x0002;
pub const WM_PARENTNOTIFY: UINT = 0x0210;
pub const WM_ENTERMENULOOP: UINT = 0x0211;
pub const WM_EXITMENULOOP: UINT = 0x0212;
pub const WM_NEXTMENU: UINT = 0x0213;
pub const WM_SIZING: UINT = 0x0214;
pub const WM_CAPTURECHANGED: UINT = 0x0215;
pub const WM_MOVING: UINT = 0x0216;
pub const WM_POWERBROADCAST: UINT = 0x0218;
pub const PBT_APMQUERYSUSPEND: WPARAM = 0x0000;
pub const PBT_APMQUERYSTANDBY: WPARAM = 0x0001;
pub const PBT_APMQUERYSUSPENDFAILED: WPARAM = 0x0002;
pub const PBT_APMQUERYSTANDBYFAILED: WPARAM = 0x0003;
pub const PBT_APMSUSPEND: WPARAM = 0x0004;
pub const PBT_APMSTANDBY: WPARAM = 0x0005;
pub const PBT_APMRESUMECRITICAL: WPARAM = 0x0006;
pub const PBT_APMRESUMESUSPEND: WPARAM = 0x0007;
pub const PBT_APMRESUMESTANDBY: WPARAM = 0x0008;
pub const PBTF_APMRESUMEFROMFAILURE: LPARAM = 0x00000001;
pub const PBT_APMBATTERYLOW: WPARAM = 0x0009;
pub const PBT_APMPOWERSTATUSCHANGE: WPARAM = 0x000A;
pub const PBT_APMOEMEVENT: WPARAM = 0x000B;
pub const PBT_APMRESUMEAUTOMATIC: WPARAM = 0x0012;
pub const PBT_POWERSETTINGCHANGE: WPARAM = 0x8013;
STRUCT!{struct POWERBROADCAST_SETTING {
    PowerSetting: GUID,
    DataLength: DWORD,
    Data: [UCHAR; 1],
}}
pub type PPOWERBROADCAST_SETTING = *mut POWERBROADCAST_SETTING;
pub const WM_DEVICECHANGE: UINT = 0x0219;
pub const WM_MDICREATE: UINT = 0x0220;
pub const WM_MDIDESTROY: UINT = 0x0221;
pub const WM_MDIACTIVATE: UINT = 0x0222;
pub const WM_MDIRESTORE: UINT = 0x0223;
pub const WM_MDINEXT: UINT = 0x0224;
pub const WM_MDIMAXIMIZE: UINT = 0x0225;
pub const WM_MDITILE: UINT = 0x0226;
pub const WM_MDICASCADE: UINT = 0x0227;
pub const WM_MDIICONARRANGE: UINT = 0x0228;
pub const WM_MDIGETACTIVE: UINT = 0x0229;
pub const WM_MDISETMENU: UINT = 0x0230;
pub const WM_ENTERSIZEMOVE: UINT = 0x0231;
pub const WM_EXITSIZEMOVE: UINT = 0x0232;
pub const WM_DROPFILES: UINT = 0x0233;
pub const WM_MDIREFRESHMENU: UINT = 0x0234;
pub const WM_POINTERDEVICECHANGE: UINT = 0x238;
pub const WM_POINTERDEVICEINRANGE: UINT = 0x239;
pub const WM_POINTERDEVICEOUTOFRANGE: UINT = 0x23A;
pub const WM_TOUCH: UINT = 0x0240;
pub const WM_NCPOINTERUPDATE: UINT = 0x0241;
pub const WM_NCPOINTERDOWN: UINT = 0x0242;
pub const WM_NCPOINTERUP: UINT = 0x0243;
pub const WM_POINTERUPDATE: UINT = 0x0245;
pub const WM_POINTERDOWN: UINT = 0x0246;
pub const WM_POINTERUP: UINT = 0x0247;
pub const WM_POINTERENTER: UINT = 0x0249;
pub const WM_POINTERLEAVE: UINT = 0x024A;
pub const WM_POINTERACTIVATE: UINT = 0x024B;
pub const WM_POINTERCAPTURECHANGED: UINT = 0x024C;
pub const WM_TOUCHHITTESTING: UINT = 0x024D;
pub const WM_POINTERWHEEL: UINT = 0x024E;
pub const WM_POINTERHWHEEL: UINT = 0x024F;
pub const DM_POINTERHITTEST: UINT = 0x0250;
pub const WM_POINTERROUTEDTO: UINT = 0x0251;
pub const WM_POINTERROUTEDAWAY: UINT = 0x0252;
pub const WM_POINTERROUTEDRELEASED: UINT = 0x0253;
pub const WM_IME_SETCONTEXT: UINT = 0x0281;
pub const WM_IME_NOTIFY: UINT = 0x0282;
pub const WM_IME_CONTROL: UINT = 0x0283;
pub const WM_IME_COMPOSITIONFULL: UINT = 0x0284;
pub const WM_IME_SELECT: UINT = 0x0285;
pub const WM_IME_CHAR: UINT = 0x0286;
pub const WM_IME_REQUEST: UINT = 0x0288;
pub const WM_IME_KEYDOWN: UINT = 0x0290;
pub const WM_IME_KEYUP: UINT = 0x0291;
pub const WM_MOUSEHOVER: UINT = 0x02A1;
pub const WM_MOUSELEAVE: UINT = 0x02A3;
pub const WM_NCMOUSEHOVER: UINT = 0x02A0;
pub const WM_NCMOUSELEAVE: UINT = 0x02A2;
pub const WM_WTSSESSION_CHANGE: UINT = 0x02B1;
pub const WM_TABLET_FIRST: UINT = 0x02c0;
pub const WM_TABLET_LAST: UINT = 0x02df;
pub const WM_DPICHANGED: UINT = 0x02E0;
pub const WM_DPICHANGED_BEFOREPARENT: UINT = 0x02E2;
pub const WM_DPICHANGED_AFTERPARENT: UINT = 0x02E3;
pub const WM_GETDPISCALEDSIZE: UINT = 0x02E4;
pub const WM_CUT: UINT = 0x0300;
pub const WM_COPY: UINT = 0x0301;
pub const WM_PASTE: UINT = 0x0302;
pub const WM_CLEAR: UINT = 0x0303;
pub const WM_UNDO: UINT = 0x0304;
pub const WM_RENDERFORMAT: UINT = 0x0305;
pub const WM_RENDERALLFORMATS: UINT = 0x0306;
pub const WM_DESTROYCLIPBOARD: UINT = 0x0307;
pub const WM_DRAWCLIPBOARD: UINT = 0x0308;
pub const WM_PAINTCLIPBOARD: UINT = 0x0309;
pub const WM_VSCROLLCLIPBOARD: UINT = 0x030A;
pub const WM_SIZECLIPBOARD: UINT = 0x030B;
pub const WM_ASKCBFORMATNAME: UINT = 0x030C;
pub const WM_CHANGECBCHAIN: UINT = 0x030D;
pub const WM_HSCROLLCLIPBOARD: UINT = 0x030E;
pub const WM_QUERYNEWPALETTE: UINT = 0x030F;
pub const WM_PALETTEISCHANGING: UINT = 0x0310;
pub const WM_PALETTECHANGED: UINT = 0x0311;
pub const WM_HOTKEY: UINT = 0x0312;
pub const WM_PRINT: UINT = 0x0317;
pub const WM_PRINTCLIENT: UINT = 0x0318;
pub const WM_APPCOMMAND: UINT = 0x0319;
pub const WM_THEMECHANGED: UINT = 0x031A;
pub const WM_CLIPBOARDUPDATE: UINT = 0x031D;
pub const WM_DWMCOMPOSITIONCHANGED: UINT = 0x031E;
pub const WM_DWMNCRENDERINGCHANGED: UINT = 0x031F;
pub const WM_DWMCOLORIZATIONCOLORCHANGED: UINT = 0x0320;
pub const WM_DWMWINDOWMAXIMIZEDCHANGE: UINT = 0x0321;
pub const WM_DWMSENDICONICTHUMBNAIL: UINT = 0x0323;
pub const WM_DWMSENDICONICLIVEPREVIEWBITMAP: UINT = 0x0326;
pub const WM_GETTITLEBARINFOEX: UINT = 0x033F;
pub const WM_HANDHELDFIRST: UINT = 0x0358;
pub const WM_HANDHELDLAST: UINT = 0x035F;
pub const WM_AFXFIRST: UINT = 0x0360;
pub const WM_AFXLAST: UINT = 0x037F;
pub const WM_PENWINFIRST: UINT = 0x0380;
pub const WM_PENWINLAST: UINT = 0x038F;
pub const WM_APP: UINT = 0x8000;
pub const WM_USER: UINT = 0x0400;
pub const WMSZ_LEFT: UINT = 1;
pub const WMSZ_RIGHT: UINT = 2;
pub const WMSZ_TOP: UINT = 3;
pub const WMSZ_TOPLEFT: UINT = 4;
pub const WMSZ_TOPRIGHT: UINT = 5;
pub const WMSZ_BOTTOM: UINT = 6;
pub const WMSZ_BOTTOMLEFT: UINT = 7;
pub const WMSZ_BOTTOMRIGHT: UINT = 8;
pub const HTERROR: LRESULT = -2;
pub const HTTRANSPARENT: LRESULT = -1;
pub const HTNOWHERE: LRESULT = 0;
pub const HTCLIENT: LRESULT = 1;
pub const HTCAPTION: LRESULT = 2;
pub const HTSYSMENU: LRESULT = 3;
pub const HTGROWBOX: LRESULT = 4;
pub const HTSIZE: LRESULT = HTGROWBOX;
pub const HTMENU: LRESULT = 5;
pub const HTHSCROLL: LRESULT = 6;
pub const HTVSCROLL: LRESULT = 7;
pub const HTMINBUTTON: LRESULT = 8;
pub const HTMAXBUTTON: LRESULT = 9;
pub const HTLEFT: LRESULT = 10;
pub const HTRIGHT: LRESULT = 11;
pub const HTTOP: LRESULT = 12;
pub const HTTOPLEFT: LRESULT = 13;
pub const HTTOPRIGHT: LRESULT = 14;
pub const HTBOTTOM: LRESULT = 15;
pub const HTBOTTOMLEFT: LRESULT = 16;
pub const HTBOTTOMRIGHT: LRESULT = 17;
pub const HTBORDER: LRESULT = 18;
pub const HTREDUCE: LRESULT = HTMINBUTTON;
pub const HTZOOM: LRESULT = HTMAXBUTTON;
pub const HTSIZEFIRST: LRESULT = HTLEFT;
pub const HTSIZELAST: LRESULT = HTBOTTOMRIGHT;
pub const HTOBJECT: LRESULT = 19;
pub const HTCLOSE: LRESULT = 20;
pub const HTHELP: LRESULT = 21;
pub const SMTO_NORMAL: UINT = 0x0000;
pub const SMTO_BLOCK: UINT = 0x0001;
pub const SMTO_ABORTIFHUNG: UINT = 0x0002;
pub const SMTO_NOTIMEOUTIFNOTHUNG: UINT = 0x0008;
pub const SMTO_ERRORONEXIT: UINT = 0x0020;
pub const MA_ACTIVATE: UINT = 1;
pub const MA_ACTIVATEANDEAT: UINT = 2;
pub const MA_NOACTIVATE: UINT = 3;
pub const MA_NOACTIVATEANDEAT: UINT = 4;
pub const ICON_SMALL: UINT = 0;
pub const ICON_BIG: UINT = 1;
pub const ICON_SMALL2: UINT = 2;
extern "system" {
    pub fn RegisterWindowMessageA(
        lpString: LPCSTR,
    ) -> UINT;
    pub fn RegisterWindowMessageW(
        lpString: LPCWSTR,
    ) -> UINT;
}
pub const SIZE_RESTORED: WPARAM = 0;
pub const SIZE_MINIMIZED: WPARAM = 1;
pub const SIZE_MAXIMIZED: WPARAM = 2;
pub const SIZE_MAXSHOW: WPARAM = 3;
pub const SIZE_MAXHIDE: WPARAM = 4;
pub const SIZENORMAL: WPARAM = SIZE_RESTORED;
pub const SIZEICONIC: WPARAM = SIZE_MINIMIZED;
pub const SIZEFULLSCREEN: WPARAM = SIZE_MAXIMIZED;
pub const SIZEZOOMSHOW: WPARAM = SIZE_MAXSHOW;
pub const SIZEZOOMHIDE: WPARAM = SIZE_MAXHIDE;
STRUCT!{struct WINDOWPOS {
    hwnd: HWND,
    hwndInsertAfter: HWND,
    x: c_int,
    y: c_int,
    cx: c_int,
    cy: c_int,
    flags: UINT,
}}
pub type LPWINDOWPOS = *mut WINDOWPOS;
pub type PWINDOWPOS = *mut WINDOWPOS;
STRUCT!{struct NCCALCSIZE_PARAMS {
    rgrc: [RECT; 3],
    lppos: PWINDOWPOS,
}}
pub type LPNCCALCSIZE_PARAMS = *mut NCCALCSIZE_PARAMS;
pub const WVR_ALIGNTOP: LRESULT = 0x0010;
pub const WVR_ALIGNLEFT: LRESULT = 0x0020;
pub const WVR_ALIGNBOTTOM: LRESULT = 0x0040;
pub const WVR_ALIGNRIGHT: LRESULT = 0x0080;
pub const WVR_HREDRAW: LRESULT = 0x0100;
pub const WVR_VREDRAW: LRESULT = 0x0200;
pub const WVR_REDRAW: LRESULT = WVR_HREDRAW | WVR_VREDRAW;
pub const WVR_VALIDRECTS: LRESULT = 0x0400;
pub const MK_LBUTTON: WPARAM = 0x0001;
pub const MK_RBUTTON: WPARAM = 0x0002;
pub const MK_SHIFT: WPARAM = 0x0004;
pub const MK_CONTROL: WPARAM = 0x0008;
pub const MK_MBUTTON: WPARAM = 0x0010;
pub const MK_XBUTTON1: WPARAM = 0x0020;
pub const MK_XBUTTON2: WPARAM = 0x0040;
pub const TME_HOVER: DWORD = 0x00000001;
pub const TME_LEAVE: DWORD = 0x00000002;
pub const TME_NONCLIENT: DWORD = 0x00000010;
pub const TME_QUERY: DWORD = 0x40000000;
pub const TME_CANCEL: DWORD = 0x80000000;
pub const HOVER_DEFAULT: DWORD = 0xFFFFFFFF;
STRUCT!{struct TRACKMOUSEEVENT {
    cbSize: DWORD,
    dwFlags: DWORD,
    hwndTrack: HWND,
    dwHoverTime: DWORD,
}}
pub type LPTRACKMOUSEEVENT = *mut TRACKMOUSEEVENT;
extern "system" {
    pub fn TrackMouseEvent(
        lpEventTrack: LPTRACKMOUSEEVENT,
    ) -> BOOL;
}
pub const WS_OVERLAPPED: DWORD = 0x00000000;
pub const WS_POPUP: DWORD = 0x80000000;
pub const WS_CHILD: DWORD = 0x40000000;
pub const WS_MINIMIZE: DWORD = 0x20000000;
pub const WS_VISIBLE: DWORD = 0x10000000;
pub const WS_DISABLED: DWORD = 0x08000000;
pub const WS_CLIPSIBLINGS: DWORD = 0x04000000;
pub const WS_CLIPCHILDREN: DWORD = 0x02000000;
pub const WS_MAXIMIZE: DWORD = 0x01000000;
pub const WS_CAPTION: DWORD = 0x00C00000;
pub const WS_BORDER: DWORD = 0x00800000;
pub const WS_DLGFRAME: DWORD = 0x00400000;
pub const WS_VSCROLL: DWORD = 0x00200000;
pub const WS_HSCROLL: DWORD = 0x00100000;
pub const WS_SYSMENU: DWORD = 0x00080000;
pub const WS_THICKFRAME: DWORD = 0x00040000;
pub const WS_GROUP: DWORD = 0x00020000;
pub const WS_TABSTOP: DWORD = 0x00010000;
pub const WS_MINIMIZEBOX: DWORD = 0x00020000;
pub const WS_MAXIMIZEBOX: DWORD = 0x00010000;
pub const WS_TILED: DWORD = WS_OVERLAPPED;
pub const WS_ICONIC: DWORD = WS_MINIMIZE;
pub const WS_SIZEBOX: DWORD = WS_THICKFRAME;
pub const WS_TILEDWINDOW: DWORD = WS_OVERLAPPEDWINDOW;
pub const WS_OVERLAPPEDWINDOW: DWORD = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME
    | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
pub const WS_POPUPWINDOW: DWORD = WS_POPUP | WS_BORDER | WS_SYSMENU;
pub const WS_CHILDWINDOW: DWORD = WS_CHILD;
pub const WS_EX_DLGMODALFRAME: DWORD = 0x00000001;
pub const WS_EX_NOPARENTNOTIFY: DWORD = 0x00000004;
pub const WS_EX_TOPMOST: DWORD = 0x00000008;
pub const WS_EX_ACCEPTFILES: DWORD = 0x00000010;
pub const WS_EX_TRANSPARENT: DWORD = 0x00000020;
pub const WS_EX_MDICHILD: DWORD = 0x00000040;
pub const WS_EX_TOOLWINDOW: DWORD = 0x00000080;
pub const WS_EX_WINDOWEDGE: DWORD = 0x00000100;
pub const WS_EX_CLIENTEDGE: DWORD = 0x00000200;
pub const WS_EX_CONTEXTHELP: DWORD = 0x00000400;
pub const WS_EX_RIGHT: DWORD = 0x00001000;
pub const WS_EX_LEFT: DWORD = 0x00000000;
pub const WS_EX_RTLREADING: DWORD = 0x00002000;
pub const WS_EX_LTRREADING: DWORD = 0x00000000;
pub const WS_EX_LEFTSCROLLBAR: DWORD = 0x00004000;
pub const WS_EX_RIGHTSCROLLBAR: DWORD = 0x00000000;
pub const WS_EX_CONTROLPARENT: DWORD = 0x00010000;
pub const WS_EX_STATICEDGE: DWORD = 0x00020000;
pub const WS_EX_APPWINDOW: DWORD = 0x00040000;
pub const WS_EX_OVERLAPPEDWINDOW: DWORD = WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE;
pub const WS_EX_PALETTEWINDOW: DWORD = WS_EX_WINDOWEDGE | WS_EX_TOOLWINDOW | WS_EX_TOPMOST;
pub const WS_EX_LAYERED: DWORD = 0x00080000;
pub const WS_EX_NOINHERITLAYOUT: DWORD = 0x00100000;
pub const WS_EX_NOREDIRECTIONBITMAP: DWORD = 0x00200000;
pub const WS_EX_LAYOUTRTL: DWORD = 0x00400000;
pub const WS_EX_COMPOSITED: DWORD = 0x02000000;
pub const WS_EX_NOACTIVATE: DWORD = 0x08000000;
pub const CS_VREDRAW: UINT = 0x0001;
pub const CS_HREDRAW: UINT = 0x0002;
pub const CS_DBLCLKS: UINT = 0x0008;
pub const CS_OWNDC: UINT = 0x0020;
pub const CS_CLASSDC: UINT = 0x0040;
pub const CS_PARENTDC: UINT = 0x0080;
pub const CS_NOCLOSE: UINT = 0x0200;
pub const CS_SAVEBITS: UINT = 0x0800;
pub const CS_BYTEALIGNCLIENT: UINT = 0x1000;
pub const CS_BYTEALIGNWINDOW: UINT = 0x2000;
pub const CS_GLOBALCLASS: UINT = 0x4000;
pub const CS_IME: UINT = 0x00010000;
pub const CS_DROPSHADOW: UINT = 0x00020000;
pub const PRF_CHECKVISIBLE: UINT = 0x00000001;
pub const PRF_NONCLIENT: UINT = 0x00000002;
pub const PRF_CLIENT: UINT = 0x00000004;
pub const PRF_ERASEBKGND: UINT = 0x00000008;
pub const PRF_CHILDREN: UINT = 0x00000010;
pub const PRF_OWNED: UINT = 0x00000020;
pub const BDR_RAISEDOUTER: UINT = 0x0001;
pub const BDR_SUNKENOUTER: UINT = 0x0002;
pub const BDR_RAISEDINNER: UINT = 0x0004;
pub const BDR_SUNKENINNER: UINT = 0x0008;
pub const BDR_OUTER: UINT = BDR_RAISEDOUTER | BDR_SUNKENOUTER;
pub const BDR_INNER: UINT = BDR_RAISEDINNER | BDR_SUNKENINNER;
pub const BDR_RAISED: UINT = BDR_RAISEDOUTER | BDR_RAISEDINNER;
pub const BDR_SUNKEN: UINT = BDR_SUNKENOUTER | BDR_SUNKENINNER;
pub const EDGE_RAISED: UINT = BDR_RAISEDOUTER | BDR_RAISEDINNER;
pub const EDGE_SUNKEN: UINT = BDR_SUNKENOUTER | BDR_SUNKENINNER;
pub const EDGE_ETCHED: UINT = BDR_SUNKENOUTER | BDR_RAISEDINNER;
pub const EDGE_BUMP: UINT = BDR_RAISEDOUTER | BDR_SUNKENINNER;
pub const BF_LEFT: UINT = 0x0001;
pub const BF_TOP: UINT = 0x0002;
pub const BF_RIGHT: UINT = 0x0004;
pub const BF_BOTTOM: UINT = 0x0008;
pub const BF_TOPLEFT: UINT = BF_TOP | BF_LEFT;
pub const BF_TOPRIGHT: UINT = BF_TOP | BF_RIGHT;
pub const BF_BOTTOMLEFT: UINT = BF_BOTTOM | BF_LEFT;
pub const BF_BOTTOMRIGHT: UINT = BF_BOTTOM | BF_RIGHT;
pub const BF_RECT: UINT = BF_LEFT | BF_TOP | BF_RIGHT | BF_BOTTOM;
pub const BF_DIAGONAL: UINT = 0x0010;
pub const BF_DIAGONAL_ENDTOPRIGHT: UINT = BF_DIAGONAL | BF_TOP | BF_RIGHT;
pub const BF_DIAGONAL_ENDTOPLEFT: UINT = BF_DIAGONAL | BF_TOP | BF_LEFT;
pub const BF_DIAGONAL_ENDBOTTOMLEFT: UINT = BF_DIAGONAL | BF_BOTTOM | BF_LEFT;
pub const BF_DIAGONAL_ENDBOTTOMRIGHT: UINT = BF_DIAGONAL | BF_BOTTOM | BF_RIGHT;
pub const BF_MIDDLE: UINT = 0x0800;
pub const BF_SOFT: UINT = 0x1000;
pub const BF_ADJUST: UINT = 0x2000;
pub const BF_FLAT: UINT = 0x4000;
pub const BF_MONO: UINT = 0x8000;
extern "system" {
    pub fn DrawEdge(
        hdc: HDC,
        qrc: LPRECT,
        edge: UINT,
        grfFlags: UINT,
    ) -> BOOL;
}
pub const DFC_CAPTION: UINT = 1;
pub const DFC_MENU: UINT = 2;
pub const DFC_SCROLL: UINT = 3;
pub const DFC_BUTTON: UINT = 4;
pub const DFC_POPUPMENU: UINT = 5;
pub const DFCS_CAPTIONCLOSE: UINT = 0x0000;
pub const DFCS_CAPTIONMIN: UINT = 0x0001;
pub const DFCS_CAPTIONMAX: UINT = 0x0002;
pub const DFCS_CAPTIONRESTORE: UINT = 0x0003;
pub const DFCS_CAPTIONHELP: UINT = 0x0004;
pub const DFCS_MENUARROW: UINT = 0x0000;
pub const DFCS_MENUCHECK: UINT = 0x0001;
pub const DFCS_MENUBULLET: UINT = 0x0002;
pub const DFCS_MENUARROWRIGHT: UINT = 0x0004;
pub const DFCS_SCROLLUP: UINT = 0x0000;
pub const DFCS_SCROLLDOWN: UINT = 0x0001;
pub const DFCS_SCROLLLEFT: UINT = 0x0002;
pub const DFCS_SCROLLRIGHT: UINT = 0x0003;
pub const DFCS_SCROLLCOMBOBOX: UINT = 0x0005;
pub const DFCS_SCROLLSIZEGRIP: UINT = 0x0008;
pub const DFCS_SCROLLSIZEGRIPRIGHT: UINT = 0x0010;
pub const DFCS_BUTTONCHECK: UINT = 0x0000;
pub const DFCS_BUTTONRADIOIMAGE: UINT = 0x0001;
pub const DFCS_BUTTONRADIOMASK: UINT = 0x0002;
pub const DFCS_BUTTONRADIO: UINT = 0x0004;
pub const DFCS_BUTTON3STATE: UINT = 0x0008;
pub const DFCS_BUTTONPUSH: UINT = 0x0010;
pub const DFCS_INACTIVE: UINT = 0x0100;
pub const DFCS_PUSHED: UINT = 0x0200;
pub const DFCS_CHECKED: UINT = 0x0400;
pub const DFCS_TRANSPARENT: UINT = 0x0800;
pub const DFCS_HOT: UINT = 0x1000;
pub const DFCS_ADJUSTRECT: UINT = 0x2000;
pub const DFCS_FLAT: UINT = 0x4000;
pub const DFCS_MONO: UINT = 0x8000;
extern "system" {
    pub fn DrawFrameControl(
        hdc: HDC,
        lprc: LPRECT,
        uType: UINT,
        uState: UINT,
    ) -> BOOL;
}
pub const DC_ACTIVE: UINT = 0x0001;
pub const DC_SMALLCAP: UINT = 0x0002;
pub const DC_ICON: UINT = 0x0004;
pub const DC_TEXT: UINT = 0x0008;
pub const DC_INBUTTON: UINT = 0x0010;
pub const DC_GRADIENT: UINT = 0x0020;
pub const DC_BUTTONS: UINT = 0x1000;
extern "system" {
    pub fn DrawCaption(
        hwnd: HWND,
        hdc: HDC,
        lprect: *const RECT,
        flags: UINT,
    ) -> BOOL;
}
pub const IDANI_OPEN: c_int = 1;
pub const IDANI_CAPTION: c_int = 3;
extern "system" {
    pub fn DrawAnimatedRects(
        hwnd: HWND,
        idAni: c_int,
        lprcFrom: *const RECT,
        lprcTo: *const RECT,
    ) -> BOOL;
}
pub const CF_TEXT: UINT = 1;
pub const CF_BITMAP: UINT = 2;
pub const CF_METAFILEPICT: UINT = 3;
pub const CF_SYLK: UINT = 4;
pub const CF_DIF: UINT = 5;
pub const CF_TIFF: UINT = 6;
pub const CF_OEMTEXT: UINT = 7;
pub const CF_DIB: UINT = 8;
pub const CF_PALETTE: UINT = 9;
pub const CF_PENDATA: UINT = 10;
pub const CF_RIFF: UINT = 11;
pub const CF_WAVE: UINT = 12;
pub const CF_UNICODETEXT: UINT = 13;
pub const CF_ENHMETAFILE: UINT = 14;
pub const CF_HDROP: UINT = 15;
pub const CF_LOCALE: UINT = 16;
pub const CF_DIBV5: UINT = 17;
pub const CF_MAX: UINT = 18;
pub const CF_OWNERDISPLAY: UINT = 0x0080;
pub const CF_DSPTEXT: UINT = 0x0081;
pub const CF_DSPBITMAP: UINT = 0x0082;
pub const CF_DSPMETAFILEPICT: UINT = 0x0083;
pub const CF_DSPENHMETAFILE: UINT = 0x008E;
pub const CF_PRIVATEFIRST: UINT = 0x0200;
pub const CF_PRIVATELAST: UINT = 0x02FF;
pub const CF_GDIOBJFIRST: UINT = 0x0300;
pub const CF_GDIOBJLAST: UINT = 0x03FF;
pub const FVIRTKEY: BYTE = TRUE as u8;
pub const FNOINVERT: BYTE = 0x02;
pub const FSHIFT: BYTE = 0x04;
pub const FCONTROL: BYTE = 0x08;
pub const FALT: BYTE = 0x10;
STRUCT!{struct ACCEL {
    fVirt: BYTE,
    key: WORD,
    cmd: WORD,
}}
pub type LPACCEL = *mut ACCEL;
STRUCT!{struct PAINTSTRUCT {
    hdc: HDC,
    fErase: BOOL,
    rcPaint: RECT,
    fRestore: BOOL,
    fIncUpdate: BOOL,
    rgbReserved: [BYTE; 32],
}}
pub type PPAINTSTRUCT = *mut PAINTSTRUCT;
pub type NPPAINTSTRUCT = *mut PAINTSTRUCT;
pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;
STRUCT!{struct CREATESTRUCTA {
    lpCreateParams: LPVOID,
    hInstance: HINSTANCE,
    hMenu: HMENU,
    hwndParent: HWND,
    cy: c_int,
    cx: c_int,
    y: c_int,
    x: c_int,
    style: LONG,
    lpszName: LPCSTR,
    lpszClass: LPCSTR,
    dwExStyle: DWORD,
}}
pub type LPCREATESTRUCTA = *mut CREATESTRUCTA;
STRUCT!{struct CREATESTRUCTW {
    lpCreateParams: LPVOID,
    hInstance: HINSTANCE,
    hMenu: HMENU,
    hwndParent: HWND,
    cy: c_int,
    cx: c_int,
    y: c_int,
    x: c_int,
    style: LONG,
    lpszName: LPCWSTR,
    lpszClass: LPCWSTR,
    dwExStyle: DWORD,
}}
pub type LPCREATESTRUCTW = *mut CREATESTRUCTW;
STRUCT!{struct WINDOWPLACEMENT {
    length: UINT,
    flags: UINT,
    showCmd: UINT,
    ptMinPosition: POINT,
    ptMaxPosition: POINT,
    rcNormalPosition: RECT,
}}
pub type PWINDOWPLACEMENT = *mut WINDOWPLACEMENT;
pub type LPWINDOWPLACEMENT = *mut WINDOWPLACEMENT;
pub const WPF_SETMINPOSITION: UINT = 0x0001;
pub const WPF_RESTORETOMAXIMIZED: UINT = 0x0002;
pub const WPF_ASYNCWINDOWPLACEMENT: UINT = 0x0004;
STRUCT!{struct NMHDR {
    hwndFrom: HWND,
    idFrom: UINT_PTR,
    code: UINT,
}}
pub type LPNMHDR = *mut NMHDR;
STRUCT!{struct STYLESTRUCT {
    styleOld: DWORD,
    styleNew: DWORD,
}}
pub type LPSTYLESTRUCT = *mut STYLESTRUCT;
pub const ODT_MENU: UINT = 1;
pub const ODT_LISTBOX: UINT = 2;
pub const ODT_COMBOBOX: UINT = 3;
pub const ODT_BUTTON: UINT = 4;
pub const ODT_STATIC: UINT = 5;
pub const ODA_DRAWENTIRE: UINT = 0x0001;
pub const ODA_SELECT: UINT = 0x0002;
pub const ODA_FOCUS: UINT = 0x0004;
pub const ODS_SELECTED: UINT = 0x0001;
pub const ODS_GRAYED: UINT = 0x0002;
pub const ODS_DISABLED: UINT = 0x0004;
pub const ODS_CHECKED: UINT = 0x0008;
pub const ODS_FOCUS: UINT = 0x0010;
pub const ODS_DEFAULT: UINT = 0x0020;
pub const ODS_COMBOBOXEDIT: UINT = 0x1000;
pub const ODS_HOTLIGHT: UINT = 0x0040;
pub const ODS_INACTIVE: UINT = 0x0080;
pub const ODS_NOACCEL: UINT = 0x0100;
pub const ODS_NOFOCUSRECT: UINT = 0x0200;
STRUCT!{struct MEASUREITEMSTRUCT {
    CtlType: UINT,
    CtlID: UINT,
    itemID: UINT,
    itemWidth: UINT,
    itemHeight: UINT,
    itemData: ULONG_PTR,
}}
pub type PMEASUREITEMSTRUCT = *mut MEASUREITEMSTRUCT;
pub type LPMEASUREITEMSTRUCT = *mut MEASUREITEMSTRUCT;
STRUCT!{struct DRAWITEMSTRUCT {
    CtlType: UINT,
    CtlID: UINT,
    itemID: UINT,
    itemAction: UINT,
    itemState: UINT,
    hwndItem: HWND,
    hDC: HDC,
    rcItem: RECT,
    itemData: ULONG_PTR,
}}
pub type PDRAWITEMSTRUCT = *mut DRAWITEMSTRUCT;
pub type LPDRAWITEMSTRUCT = *mut DRAWITEMSTRUCT;
STRUCT!{struct DELETEITEMSTRUCT {
    CtlType: UINT,
    CtlID: UINT,
    itemID: UINT,
    hwndItem: HWND,
    itemData: ULONG_PTR,
}}
pub type PDELETEITEMSTRUCT = *mut DELETEITEMSTRUCT;
pub type LPDELETEITEMSTRUCT = *mut DELETEITEMSTRUCT;
STRUCT!{struct COMPAREITEMSTRUCT {
    CtlType: UINT,
    CtlID: UINT,
    hwndItem: HWND,
    itemID1: UINT,
    itemData1: ULONG_PTR,
    itemID2: UINT,
    itemData2: ULONG_PTR,
    dwLocaleId: DWORD,
}}
pub type PCOMPAREITEMSTRUCT = *mut COMPAREITEMSTRUCT;
pub type LPCOMPAREITEMSTRUCT = *mut COMPAREITEMSTRUCT;
extern "system" {
    pub fn GetMessageA(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
    ) -> BOOL;
    pub fn GetMessageW(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
    ) -> BOOL;
    pub fn TranslateMessage(
        lpmsg: *const MSG,
    ) -> BOOL;
    pub fn DispatchMessageA(
        lpmsg: *const MSG,
    ) -> LRESULT;
    pub fn DispatchMessageW(
        lpmsg: *const MSG,
    ) -> LRESULT;
    pub fn SetMessageQueue(
        cMessagesMax: c_int,
    ) -> BOOL;
    pub fn PeekMessageA(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
        wRemoveMsg: UINT,
    ) -> BOOL;
    pub fn PeekMessageW(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
        wRemoveMsg: UINT,
    ) -> BOOL;
}
pub const PM_NOREMOVE: UINT = 0x0000;
pub const PM_REMOVE: UINT = 0x0001;
pub const PM_NOYIELD: UINT = 0x0002;
pub const PM_QS_INPUT: UINT = QS_INPUT << 16;
pub const PM_QS_POSTMESSAGE: UINT = (QS_POSTMESSAGE | QS_HOTKEY | QS_TIMER) << 16;
pub const PM_QS_PAINT: UINT = QS_PAINT << 16;
pub const PM_QS_SENDMESSAGE: UINT = QS_SENDMESSAGE << 16;
extern "system" {
    pub fn RegisterHotKey(
        hwnd: HWND,
        id: c_int,
        fsModifiers: UINT,
        vk: UINT,
    ) -> BOOL;
    pub fn UnregisterHotKey(
        hWnd: HWND,
        id: c_int,
    ) -> BOOL;
}
pub const MOD_ALT: LPARAM = 0x0001;
pub const MOD_CONTROL: LPARAM = 0x0002;
pub const MOD_SHIFT: LPARAM = 0x0004;
pub const MOD_WIN: LPARAM = 0x0008;
pub const MOD_NOREPEAT: LPARAM = 0x4000;
pub const IDHOT_SNAPWINDOW: WPARAM = -1isize as usize;
pub const IDHOT_SNAPDESKTOP: WPARAM = -2isize as usize;
pub const ENDSESSION_CLOSEAPP: UINT = 0x00000001;
pub const ENDSESSION_CRITICAL: UINT = 0x40000000;
pub const ENDSESSION_LOGOFF: UINT = 0x80000000;
pub const EWX_LOGOFF: UINT = 0x00000000;
pub const EWX_SHUTDOWN: UINT = 0x00000001;
pub const EWX_REBOOT: UINT = 0x00000002;
pub const EWX_FORCE: UINT = 0x00000004;
pub const EWX_POWEROFF: UINT = 0x00000008;
pub const EWX_FORCEIFHUNG: UINT = 0x00000010;
pub const EWX_QUICKRESOLVE: UINT = 0x00000020;
pub const EWX_RESTARTAPPS: UINT = 0x00000040;
pub const EWX_HYBRID_SHUTDOWN: UINT = 0x00400000;
pub const EWX_BOOTOPTIONS: UINT = 0x01000000;
// ExitWindows
extern "system" {
    pub fn ExitWindowsEx(
        uFlags: UINT,
        dwReason: DWORD,
    ) -> BOOL;
    pub fn SwapMouseButton(
        fSwap: BOOL,
    ) -> BOOL;
    pub fn GetMessagePos() -> DWORD;
    pub fn GetMessageTime() -> LONG;
    pub fn GetMessageExtraInfo() -> LPARAM;
    pub fn GetUnpredictedMessagePos() -> DWORD;
    pub fn IsWow64Message() -> BOOL;
    pub fn SetMessageExtraInfo(
        lParam: LPARAM,
    ) -> LPARAM;
    pub fn SendMessageA(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
    pub fn SendMessageW(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
    pub fn SendMessageTimeoutA(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
        fuFlags: UINT,
        uTimeout: UINT,
        lpdwResult: PDWORD_PTR,
    ) -> LRESULT;
    pub fn SendMessageTimeoutW(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
        fuFlags: UINT,
        uTimeout: UINT,
        lpdwResult: PDWORD_PTR,
    ) -> LRESULT;
    pub fn SendNotifyMessageA(
        hWnd: HWND,
        msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn SendNotifyMessageW(
        hWnd: HWND,
        msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn SendMessageCallbackA(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
        lpResultCallBack: SENDASYNCPROC,
        dwData: ULONG_PTR,
    ) -> BOOL;
    pub fn SendMessageCallbackW(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
        lpResultCallBack: SENDASYNCPROC,
        dwData: ULONG_PTR,
    ) -> BOOL;
}
STRUCT!{struct BSMINFO {
    cbSize: UINT,
    hdesk: HDESK,
    hwnd: HWND,
    luid: LUID,
}}
pub type PBSMINFO = *mut BSMINFO;
extern "system" {
    pub fn BroadcastSystemMessageExA(
        flags: DWORD,
        lpInfo: LPDWORD,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
        pbsmInfo: PBSMINFO,
    ) -> c_long;
    pub fn BroadcastSystemMessageExW(
        flags: DWORD,
        lpInfo: LPDWORD,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
        pbsmInfo: PBSMINFO,
    ) -> c_long;
    pub fn BroadcastSystemMessageA(
        flags: DWORD,
        lpInfo: LPDWORD,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LONG;
    pub fn BroadcastSystemMessageW(
        flags: DWORD,
        lpInfo: LPDWORD,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LONG;
}
pub const BSM_ALLCOMPONENTS: DWORD = 0x00000000;
pub const BSM_VXDS: DWORD = 0x00000001;
pub const BSM_NETDRIVER: DWORD = 0x00000002;
pub const BSM_INSTALLABLEDRIVERS: DWORD = 0x00000004;
pub const BSM_APPLICATIONS: DWORD = 0x00000008;
pub const BSM_ALLDESKTOPS: DWORD = 0x00000010;
pub const BSF_QUERY: DWORD = 0x00000001;
pub const BSF_IGNORECURRENTTASK: DWORD = 0x00000002;
pub const BSF_FLUSHDISK: DWORD = 0x00000004;
pub const BSF_NOHANG: DWORD = 0x00000008;
pub const BSF_POSTMESSAGE: DWORD = 0x00000010;
pub const BSF_FORCEIFHUNG: DWORD = 0x00000020;
pub const BSF_NOTIMEOUTIFNOTHUNG: DWORD = 0x00000040;
pub const BSF_ALLOWSFW: DWORD = 0x00000080;
pub const BSF_SENDNOTIFYMESSAGE: DWORD = 0x00000100;
pub const BSF_RETURNHDESK: DWORD = 0x00000200;
pub const BSF_LUID: DWORD = 0x00000400;
pub const BROADCAST_QUERY_DENY: DWORD = 0x424D5144;
pub type HDEVNOTIFY = PVOID;
pub type PHDEVNOTIFY = *mut HDEVNOTIFY;
pub const DEVICE_NOTIFY_WINDOW_HANDLE: DWORD = 0x00000000;
pub const DEVICE_NOTIFY_SERVICE_HANDLE: DWORD = 0x00000001;
pub const DEVICE_NOTIFY_ALL_INTERFACE_CLASSES: DWORD = 0x00000004;
extern "system" {
    pub fn RegisterDeviceNotificationA(
        hRecipient: HANDLE,
        notificationFilter: LPVOID,
        flags: DWORD,
    ) -> HDEVNOTIFY;
    pub fn RegisterDeviceNotificationW(
        hRecipient: HANDLE,
        notificationFilter: LPVOID,
        flags: DWORD,
    ) -> HDEVNOTIFY;
    pub fn UnregisterDeviceNotification(
        Handle: HDEVNOTIFY,
    ) -> BOOL;
}
pub type HPOWERNOTIFY = PVOID;
pub type PHPOWERNOTIFY = *mut HPOWERNOTIFY;
extern "system" {
    pub fn RegisterPowerSettingNotification(
        hRecipient: HANDLE,
        PowerSettingGuid: LPCGUID,
        Flags: DWORD,
    ) -> HPOWERNOTIFY;
    pub fn UnregisterPowerSettingNotification(
        Handle: HPOWERNOTIFY,
    ) -> BOOL;
    pub fn RegisterSuspendResumeNotification(
        hRecipient: HANDLE,
        Flags: DWORD,
    ) -> HPOWERNOTIFY;
    pub fn UnregisterSuspendResumeNotification(
        Handle: HPOWERNOTIFY,
    ) -> BOOL;
    pub fn PostMessageA(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn PostMessageW(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn PostThreadMessageA(
        idThread: DWORD,
        msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn PostThreadMessageW(
        idThread: DWORD,
        msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> BOOL;
}
// PostAppMessageA
// PostAppMessageW
pub const HWND_BROADCAST: HWND = 0xffff as HWND;
pub const HWND_MESSAGE: HWND = -3isize as HWND;
extern "system" {
    pub fn AttachThreadInput(
        idAttach: DWORD,
        idAttachTo: DWORD,
        fAttach: BOOL,
    ) -> BOOL;
    pub fn ReplyMessage(
        lResult: LRESULT,
    ) -> BOOL;
    pub fn WaitMessage() -> BOOL;
    pub fn WaitForInputIdle(
        hProcess: HANDLE,
        dwMilliseconds: DWORD,
    ) -> DWORD;
    pub fn DefWindowProcA(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
    pub fn DefWindowProcW(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
    pub fn PostQuitMessage(
        nExitCode: c_int,
    );
    pub fn CallWindowProcA(
        lpPrevWndFunc: WNDPROC,
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
    pub fn CallWindowProcW(
        lpPrevWndFunc: WNDPROC,
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
    pub fn InSendMessage() -> BOOL;
    pub fn InSendMessageEx(
        lpReserved: LPVOID,
    ) -> DWORD;
}
pub const ISMEX_NOSEND: DWORD = 0x00000000;
pub const ISMEX_SEND: DWORD = 0x00000001;
pub const ISMEX_NOTIFY: DWORD = 0x00000002;
pub const ISMEX_CALLBACK: DWORD = 0x00000004;
pub const ISMEX_REPLIED: DWORD = 0x00000008;
extern "system" {
    pub fn GetDoubleClickTime() -> UINT;
    pub fn SetDoubleClickTime(
        uInterval: UINT,
    ) -> BOOL;
    pub fn RegisterClassA(
        lpWndClass: *const WNDCLASSA,
    ) -> ATOM;
    pub fn RegisterClassW(
        lpWndClass: *const WNDCLASSW,
    ) -> ATOM;
    pub fn UnregisterClassA(
        lpClassName: LPCSTR,
        hInstance: HINSTANCE,
    ) -> BOOL;
    pub fn UnregisterClassW(
        lpClassName: LPCWSTR,
        hInstance: HINSTANCE,
    ) -> BOOL;
    pub fn GetClassInfoA(
        hInstance: HINSTANCE,
        lpClassName: LPCSTR,
        lpWndClass: LPWNDCLASSA,
    ) -> BOOL;
    pub fn GetClassInfoW(
        hInstance: HINSTANCE,
        lpClassName: LPCWSTR,
        lpWndClass: LPWNDCLASSW,
    ) -> BOOL;
    pub fn RegisterClassExA(
        lpWndClass: *const WNDCLASSEXA,
    ) -> ATOM;
    pub fn RegisterClassExW(
        lpWndClass: *const WNDCLASSEXW,
    ) -> ATOM;
    pub fn GetClassInfoExA(
        hinst: HINSTANCE,
        lpszClass: LPCSTR,
        lpwcx: LPWNDCLASSEXA,
    ) -> BOOL;
    pub fn GetClassInfoExW(
        hinst: HINSTANCE,
        lpszClass: LPCWSTR,
        lpwcx: LPWNDCLASSEXW,
    ) -> BOOL;
}
pub const CW_USEDEFAULT: c_int = 0x80000000;
pub const HWND_DESKTOP: HWND = 0 as HWND;
FN!{stdcall PREGISTERCLASSNAMEW(
    LPCWSTR,
) -> BOOLEAN}
extern "system" {
    pub fn CreateWindowExA(
        dwExStyle: DWORD,
        lpClassName: LPCSTR,
        lpWindowName: LPCSTR,
        dwStyle: DWORD,
        x: c_int,
        y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;
    pub fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        x: c_int,
        y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;
}
// CreateWindowA
// CreateWindowW
extern "system" {
    pub fn IsWindow(
        hWnd: HWND,
    ) -> BOOL;
    pub fn IsMenu(
        hMenu: HMENU,
    ) -> BOOL;
    pub fn IsChild(
        hWndParent: HWND,
        hWnd: HWND,
    ) -> BOOL;
    pub fn DestroyWindow(
        hWnd: HWND,
    ) -> BOOL;
    pub fn ShowWindow(
        hWnd: HWND,
        nCmdShow: c_int,
    ) -> BOOL;
    pub fn AnimateWindow(
        hWnd: HWND,
        dwTime: DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn UpdateLayeredWindow(
        hWnd: HWND,
        hdcDst: HDC,
        pptDst: *mut POINT,
        psize: *mut SIZE,
        hdcSrc: HDC,
        pptSrc: *mut POINT,
        crKey: COLORREF,
        pblend: *mut BLENDFUNCTION,
        dwFlags: DWORD,
    ) -> BOOL;
}
STRUCT!{struct UPDATELAYEREDWINDOWINFO {
    cbSize: DWORD,
    hdcDst: HDC,
    pptDst: *const POINT,
    psize: *const SIZE,
    hdcSrc: HDC,
    pptSrc: *const POINT,
    crKey: COLORREF,
    pblend: *const BLENDFUNCTION,
    dwFlags: DWORD,
    prcDirty: *const RECT,
}}
pub type PUPDATELAYEREDWINDOWINFO = *mut UPDATELAYEREDWINDOWINFO;
extern "system" {
    pub fn UpdateLayeredWindowIndirect(
        hWnd: HWND,
        pULWInfo: *mut UPDATELAYEREDWINDOWINFO,
    ) -> BOOL;
    pub fn GetLayeredWindowAttributes(
        hwnd: HWND,
        pcrKey: *mut COLORREF,
        pbAlpha: *mut BYTE,
        pdwFlags: *mut DWORD,
    ) -> BOOL;
}
pub const PW_CLIENTONLY: DWORD = 0x00000001;
pub const PW_RENDERFULLCONTENT: DWORD = 0x00000002;
extern "system" {
    pub fn PrintWindow(
        hwnd: HWND,
        hdcBlt: HDC,
        nFlags: UINT,
    ) -> BOOL;
    pub fn SetLayeredWindowAttributes(
        hwnd: HWND,
        crKey: COLORREF,
        bAlpha: BYTE,
        dwFlags: DWORD,
    ) -> BOOL;
}
pub const LWA_COLORKEY: DWORD = 0x00000001;
pub const LWA_ALPHA: DWORD = 0x00000002;
pub const ULW_COLORKEY: DWORD = 0x00000001;
pub const ULW_ALPHA: DWORD = 0x00000002;
pub const ULW_OPAQUE: DWORD = 0x00000004;
pub const ULW_EX_NORESIZE: DWORD = 0x00000008;
extern "system" {
    pub fn ShowWindowAsync(
        hWnd: HWND,
        nCmdShow: c_int,
    ) -> BOOL;
    pub fn FlashWindow(
        hwnd: HWND,
        bInvert: BOOL,
    ) -> BOOL;
}
STRUCT!{struct FLASHWINFO {
    cbSize: UINT,
    hwnd: HWND,
    dwFlags: DWORD,
    uCount: UINT,
    dwTimeout: DWORD,
}}
pub type PFLASHWINFO = *mut FLASHWINFO;
extern "system" {
    pub fn FlashWindowEx(
        pfwi: PFLASHWINFO,
    ) -> BOOL;
}
pub const FLASHW_STOP: DWORD = 0;
pub const FLASHW_CAPTION: DWORD = 0x00000001;
pub const FLASHW_TRAY: DWORD = 0x00000002;
pub const FLASHW_ALL: DWORD = FLASHW_CAPTION | FLASHW_TRAY;
pub const FLASHW_TIMER: DWORD = 0x00000004;
pub const FLASHW_TIMERNOFG: DWORD = 0x0000000C;
extern "system" {
    pub fn ShowOwnedPopups(
        hWnd: HWND,
        fShow: BOOL,
    ) -> BOOL;
    pub fn OpenIcon(
        hWnd: HWND,
    ) -> BOOL;
    pub fn CloseWindow(
        hWnd: HWND,
    ) -> BOOL;
    pub fn MoveWindow(
        hWnd: HWND,
        X: c_int,
        Y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        bRepaint: BOOL,
    ) -> BOOL;
    pub fn SetWindowPos(
        hWnd: HWND,
        hWndInsertAfter: HWND,
        X: c_int,
        Y: c_int,
        cx: c_int,
        cy: c_int,
        uFlags: UINT,
    ) -> BOOL;
    pub fn GetWindowPlacement(
        hWnd: HWND,
        lpwndpl: *mut WINDOWPLACEMENT,
    ) -> BOOL;
    pub fn SetWindowPlacement(
        hWnd: HWND,
        lpwndpl: *const WINDOWPLACEMENT,
    ) -> BOOL;
}
pub const WDA_NONE: DWORD = 0x00000000;
pub const WDA_MONITOR: DWORD = 0x00000001;
extern "system" {
    pub fn GetWindowDisplayAffinity(
        hWnd: HWND,
        pdwAffinity: *mut DWORD,
    ) -> BOOL;
    pub fn SetWindowDisplayAffinity(
        hWnd: HWND,
        dwAffinity: DWORD,
    ) -> BOOL;
    pub fn BeginDeferWindowPos(
        nNumWindows: c_int,
    ) -> HDWP;
    pub fn DeferWindowPos(
        hWinPosInfo: HDWP,
        hWnd: HWND,
        hWndInserAfter: HWND,
        x: c_int,
        y: c_int,
        cx: c_int,
        cy: c_int,
        uFlags: UINT,
    ) -> HDWP;
    pub fn EndDeferWindowPos(
        hWinPosInfo: HDWP,
    ) -> BOOL;
    pub fn IsWindowVisible(
        hWnd: HWND,
    ) -> BOOL;
    pub fn IsIconic(
        hWnd: HWND,
    ) -> BOOL;
    pub fn AnyPopup() -> BOOL;
    pub fn BringWindowToTop(
        hWnd: HWND,
    ) -> BOOL;
    pub fn IsZoomed(
        hwnd: HWND,
    ) -> BOOL;
}
pub const SWP_NOSIZE: UINT = 0x0001;
pub const SWP_NOMOVE: UINT = 0x0002;
pub const SWP_NOZORDER: UINT = 0x0004;
pub const SWP_NOREDRAW: UINT = 0x0008;
pub const SWP_NOACTIVATE: UINT = 0x0010;
pub const SWP_FRAMECHANGED: UINT = 0x0020;
pub const SWP_SHOWWINDOW: UINT = 0x0040;
pub const SWP_HIDEWINDOW: UINT = 0x0080;
pub const SWP_NOCOPYBITS: UINT = 0x0100;
pub const SWP_NOOWNERZORDER: UINT = 0x0200;
pub const SWP_NOSENDCHANGING: UINT = 0x0400;
pub const SWP_DRAWFRAME: UINT = SWP_FRAMECHANGED;
pub const SWP_NOREPOSITION: UINT = SWP_NOOWNERZORDER;
pub const SWP_DEFERERASE: UINT = 0x2000;
pub const SWP_ASYNCWINDOWPOS: UINT = 0x4000;
pub const HWND_TOP: HWND = 0 as HWND;
pub const HWND_BOTTOM: HWND = 1 as HWND;
pub const HWND_TOPMOST: HWND = -1isize as HWND;
pub const HWND_NOTOPMOST: HWND = -2isize as HWND;
// FIXME packed(2)
STRUCT!{#[repr(packed)] struct DLGTEMPLATE {
    style: DWORD,
    dwExtendedStyle: DWORD,
    cdit: WORD,
    x: c_short,
    y: c_short,
    cx: c_short,
    cy: c_short,
}}
pub type LPDLGTEMPLATEA = *mut DLGTEMPLATE;
pub type LPDLGTEMPLATEW = *mut DLGTEMPLATE;
pub type LPCDLGTEMPLATEA = *const DLGTEMPLATE;
pub type LPCDLGTEMPLATEW = *const DLGTEMPLATE;
// FIXME packed(2)
STRUCT!{#[repr(packed)] struct DLGITEMTEMPLATE {
    style: DWORD,
    dwExtendedStyle: DWORD,
    x: c_short,
    y: c_short,
    cx: c_short,
    cy: c_short,
    id: WORD,
}}
pub type PDLGITEMTEMPLATEA = *mut DLGITEMTEMPLATE;
pub type PDLGITEMTEMPLATEW = *mut DLGITEMTEMPLATE;
pub type LPDLGITEMTEMPLATEA = *mut DLGITEMTEMPLATE;
pub type LPDLGITEMTEMPLATEW = *mut DLGITEMTEMPLATE;
extern "system" {
    pub fn CreateDialogParamA(
        hInstance: HINSTANCE,
        lpTemplateName: LPCSTR,
        hWndParent: HWND,
        lpDialogFunc: DLGPROC,
        dwInitParam: LPARAM,
    ) -> HWND;
    pub fn CreateDialogParamW(
        hInstance: HINSTANCE,
        lpTemplateName: LPCWSTR,
        hWndParent: HWND,
        lpDialogFunc: DLGPROC,
        dwInitParam: LPARAM,
    ) -> HWND;
    pub fn CreateDialogIndirectParamA(
        hInstance: HINSTANCE,
        lpTemplate: LPCDLGTEMPLATEA,
        hWndParent: HWND,
        lpDialogFunc: DLGPROC,
        dwInitParam: LPARAM,
    ) -> HWND;
    pub fn CreateDialogIndirectParamW(
        hInstance: HINSTANCE,
        lpTemplate: LPCDLGTEMPLATEW,
        hWndParent: HWND,
        lpDialogFunc: DLGPROC,
        dwInitParam: LPARAM,
    ) -> HWND;
}
// CreateDialogA
// CreateDialogW
// CreateDialogIndirectA
// CreateDialogIndirectW
extern "system" {
    pub fn DialogBoxParamA(
        hInstance: HINSTANCE,
        lpTemplateName: LPCSTR,
        hWndParent: HWND,
        lpDialogFunc: DLGPROC,
        dwInitParam: LPARAM,
    ) -> INT_PTR;
    pub fn DialogBoxParamW(
        hInstance: HINSTANCE,
        lpTemplateName: LPCWSTR,
        hWndParent: HWND,
        lpDialogFunc: DLGPROC,
        dwInitParam: LPARAM,
    ) -> INT_PTR;
    pub fn DialogBoxIndirectParamA(
        hInstance: HINSTANCE,
        hDialogTemplate: LPCDLGTEMPLATEA,
        hWndParent: HWND,
        lpDialogFunc: DLGPROC,
        dwInitParam: LPARAM,
    ) -> INT_PTR;
    pub fn DialogBoxIndirectParamW(
        hInstance: HINSTANCE,
        hDialogTemplate: LPCDLGTEMPLATEW,
        hWndParent: HWND,
        lpDialogFunc: DLGPROC,
        dwInitParam: LPARAM,
    ) -> INT_PTR;
}
// DialogBoxA
// DialogBoxW
// DialogBoxIndirectA
// DialogBoxIndirectW
extern "system" {
    pub fn EndDialog(
        hDlg: HWND,
        nResult: INT_PTR,
    ) -> BOOL;
    pub fn GetDlgItem(
        hDlg: HWND,
        nIDDlgItem: c_int,
    ) -> HWND;
    pub fn SetDlgItemInt(
        hDlg: HWND,
        nIDDlgItem: c_int,
        uValue: UINT,
        bSigned: BOOL,
    ) -> BOOL;
    pub fn GetDlgItemInt(
        hDlg: HWND,
        nIDDlgItem: c_int,
        lpTranslated: *mut BOOL,
        bSigned: BOOL,
    ) -> UINT;
    pub fn SetDlgItemTextA(
        hDlg: HWND,
        nIDDlgItem: c_int,
        lpString: LPCSTR,
    ) -> BOOL;
    pub fn SetDlgItemTextW(
        hDlg: HWND,
        nIDDlgItem: c_int,
        lpString: LPCWSTR,
    ) -> BOOL;
    pub fn GetDlgItemTextA(
        hDlg: HWND,
        nIDDlgItem: c_int,
        lpString: LPSTR,
        nMaxCount: c_int,
    ) -> UINT;
    pub fn GetDlgItemTextW(
        hDlg: HWND,
        nIDDlgItem: c_int,
        lpString: LPWSTR,
        nMaxCount: c_int,
    ) -> UINT;
    pub fn CheckDlgButton(
        hDlg: HWND,
        nIDButton: c_int,
        uCheck: UINT,
    ) -> BOOL;
    pub fn CheckRadioButton(
        hDlg: HWND,
        nIDFirstButton: c_int,
        nIDLasatButton: c_int,
        nIDCheckButton: c_int,
    ) -> BOOL;
    pub fn IsDlgButtonChecked(
        hDlg: HWND,
        nIDButton: c_int,
    ) -> UINT;
    pub fn SendDlgItemMessageA(
        hDlg: HWND,
        nIDDlgItem: c_int,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
    pub fn SendDlgItemMessageW(
        hDlg: HWND,
        nIDDlgItem: c_int,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
    pub fn GetNextDlgGroupItem(
        hDlg: HWND,
        hCtl: HWND,
        bPrevious: BOOL,
    ) -> HWND;
    pub fn GetNextDlgTabItem(
        hDlg: HWND,
        hCtl: HWND,
        bPrevious: BOOL,
    ) -> HWND;
    pub fn GetDlgCtrlID(
        hwnd: HWND,
    ) -> c_int;
    pub fn GetDialogBaseUnits() -> LONG;
    pub fn DefDlgProcA(
        hDlg: HWND,
        msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
    pub fn DefDlgProcW(
        hDlg: HWND,
        msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
}
ENUM!{enum DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    DCDC_DEFAULT = 0x0000,
    DCDC_DISABLE_FONT_UPDATE = 0x0001,
    DCDC_DISABLE_RELAYOUT = 0x0002,
}}
extern "system" {
    pub fn SetDialogControlDpiChangeBehavior(
        hwnd: HWND,
        mask: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS,
        values: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS,
    ) -> BOOL;
    pub fn GetDialogControlDpiChangeBehavior(
        hwnd: HWND,
    ) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS;
}
ENUM!{enum DIALOG_DPI_CHANGE_BEHAVIORS {
    DDC_DEFAULT = 0x0000,
    DDC_DISABLE_ALL = 0x0001,
    DDC_DISABLE_RESIZE = 0x0002,
    DDC_DISABLE_CONTROL_RELAYOUT = 0x0004,
}}
extern "system" {
    pub fn SetDialogDpiChangeBehavior(
        hDlg: HWND,
        mask: DIALOG_DPI_CHANGE_BEHAVIORS,
        values: DIALOG_DPI_CHANGE_BEHAVIORS,
    ) -> BOOL;
    pub fn GetDialogDpiChangeBehavior(
        hDlg: HWND,
    ) -> DIALOG_DPI_CHANGE_BEHAVIORS;
    pub fn CallMsgFilterA(
        lpMsg: LPMSG,
        nCode: c_int,
    ) -> BOOL;
    pub fn CallMsgFilterW(
        lpMsg: LPMSG,
        nCode: c_int,
    ) -> BOOL;
    pub fn OpenClipboard(
        hWnd: HWND,
    ) -> BOOL;
    pub fn CloseClipboard() -> BOOL;
    pub fn GetClipboardSequenceNumber() -> DWORD;
    pub fn GetClipboardOwner() -> HWND;
    pub fn SetClipboardViewer(
        hWndNewViewer: HWND,
    ) -> HWND;
    pub fn GetClipboardViewer() -> HWND;
    pub fn ChangeClipboardChain(
        hwndRemove: HWND,
        hwndNewNext: HWND,
    ) -> BOOL;
    pub fn SetClipboardData(
        uFormat: UINT,
        hMem: HANDLE,
    ) -> HANDLE;
    pub fn GetClipboardData(
        uFormat: UINT,
    ) -> HANDLE;
    pub fn RegisterClipboardFormatA(
        lpszFormat: LPCSTR,
    ) -> UINT;
    pub fn RegisterClipboardFormatW(
        lpszFormat: LPCWSTR,
    ) -> UINT;
    pub fn CountClipboardFormats() -> c_int;
    pub fn EnumClipboardFormats(
        format: UINT,
    ) -> UINT;
    pub fn GetClipboardFormatNameA(
        format: UINT,
        lpszFormatName: LPSTR,
        cchMaxCount: c_int,
    ) -> c_int;
    pub fn GetClipboardFormatNameW(
        format: UINT,
        lpszFormatName: LPWSTR,
        cchMaxCount: c_int,
    ) -> c_int;
    pub fn EmptyClipboard() -> BOOL;
    pub fn IsClipboardFormatAvailable(
        format: UINT,
    ) -> BOOL;
    pub fn GetPriorityClipboardFormat(
        paFormatPriorityList: *mut UINT,
        cFormats: c_int,
    ) -> c_int;
    pub fn GetOpenClipboardWindow() -> HWND;
    pub fn AddClipboardFormatListener(
        hWnd: HWND,
    ) -> BOOL;
    pub fn RemoveClipboardFormatListener(
        hWnd: HWND,
    ) -> BOOL;
    pub fn GetUpdatedClipboardFormats(
        lpuiFormats: PUINT,
        cFormats: UINT,
        pcFormatsOUT: PUINT,
    ) -> BOOL;
    pub fn CharToOemA(
        pSrc: LPCSTR,
        pDst: LPSTR,
    ) -> BOOL;
    pub fn CharToOemW(
        pSrc: LPCWSTR,
        pDst: LPSTR,
    ) -> BOOL;
    pub fn OemToCharA(
        pSrc: LPCSTR,
        pDst: LPSTR,
    ) -> BOOL;
    pub fn OemToCharW(
        pSrc: LPCSTR,
        pDst: LPWSTR,
    ) -> BOOL;
    pub fn CharToOemBuffA(
        lpszSrc: LPCSTR,
        lpszDst: LPSTR,
        cchDstLength: DWORD,
    ) -> BOOL;
    pub fn CharToOemBuffW(
        lpszSrc: LPCWSTR,
        lpszDst: LPSTR,
        cchDstLength: DWORD,
    ) -> BOOL;
    pub fn OemToCharBuffA(
        lpszSrc: LPCSTR,
        lpszDst: LPSTR,
        cchDstLength: DWORD,
    ) -> BOOL;
    pub fn OemToCharBuffW(
        lpszSrc: LPCSTR,
        lpszDst: LPWSTR,
        cchDstLength: DWORD,
    ) -> BOOL;
    pub fn CharUpperA(
        lpsz: LPSTR,
    ) -> LPSTR;
    pub fn CharUpperW(
        lpsz: LPWSTR,
    ) -> LPWSTR;
    pub fn CharUpperBuffA(
        lpsz: LPSTR,
        cchLength: DWORD,
    ) -> DWORD;
    pub fn CharUpperBuffW(
        lpsz: LPWSTR,
        cchLength: DWORD,
    ) -> DWORD;
    pub fn CharLowerA(
        lpsz: LPSTR,
    ) -> LPSTR;
    pub fn CharLowerW(
        lpsz: LPWSTR,
    ) -> LPWSTR;
    pub fn CharLowerBuffA(
        lpsz: LPSTR,
        cchLength: DWORD,
    ) -> DWORD;
    pub fn CharLowerBuffW(
        lpsz: LPWSTR,
        cchLength: DWORD,
    ) -> DWORD;
    pub fn CharNextA(
        lpsz: LPCSTR,
    ) -> LPSTR;
    pub fn CharNextW(
        lpsz: LPCWSTR,
    ) -> LPWSTR;
    pub fn CharPrevA(
        lpszStart: LPCSTR,
        lpszCurrent: LPCSTR,
    ) -> LPSTR;
    pub fn CharPrevW(
        lpszStart: LPCWSTR,
        lpszCurrent: LPCWSTR,
    ) -> LPWSTR;
    pub fn CharNextExA(
        codePage: WORD,
        lpCurrentChar: LPSTR,
        dwFlags: DWORD,
    ) -> LPSTR;
    pub fn CharPrevExA(
        codePage: WORD,
        lpStart: LPCSTR,
        lpCurrentChar: LPCSTR,
        dwFlags: DWORD,
    ) -> LPSTR;
}
// AnsiToOem
// OemToAnsi
// AnsiToOemBuff
// OemToAnsiBuff
// AnsiUpper
// AnsiUpperBuff
// AnsiLower
// AnsiLowerBuff
// AnsiNext
// AnsiPrev
extern "system" {
    pub fn IsCharAlphaA(
        ch: CHAR,
    ) -> BOOL;
    pub fn IsCharAlphaW(
        ch: WCHAR,
    ) -> BOOL;
    pub fn IsCharAlphaNumericA(
        ch: CHAR,
    ) -> BOOL;
    pub fn IsCharAlphaNumericW(
        ch: WCHAR,
    ) -> BOOL;
    pub fn IsCharUpperA(
        ch: CHAR,
    ) -> BOOL;
    pub fn IsCharUpperW(
        ch: WCHAR,
    ) -> BOOL;
    pub fn IsCharLowerA(
        ch: CHAR,
    ) -> BOOL;
    pub fn IsCharLowerW(
        ch: WCHAR,
    ) -> BOOL;
    pub fn SetFocus(
        hWnd: HWND,
    ) -> HWND;
    pub fn GetActiveWindow() -> HWND;
    pub fn GetFocus() -> HWND;
    pub fn GetKBCodePage() -> UINT;
    pub fn GetKeyState(
        nVirtKey: c_int,
    ) -> SHORT;
    pub fn GetAsyncKeyState(
        vKey: c_int,
    ) -> SHORT;
    pub fn GetKeyboardState(
        lpKeyState: PBYTE,
    ) -> BOOL;
    pub fn SetKeyboardState(
        lpKeyState: LPBYTE,
    ) -> BOOL;
    pub fn GetKeyNameTextA(
        lparam: LONG,
        lpString: LPSTR,
        cchSize: c_int,
    ) -> c_int;
    pub fn GetKeyNameTextW(
        lParam: LONG,
        lpString: LPWSTR,
        cchSize: c_int,
    ) -> c_int;
    pub fn GetKeyboardType(
        nTypeFlag: c_int,
    ) -> c_int;
    pub fn ToAscii(
        uVirtKey: UINT,
        uScanCode: UINT,
        lpKeyState: *const BYTE,
        lpChar: LPWORD,
        uFlags: UINT,
    ) -> c_int;
    pub fn ToAsciiEx(
        uVirtKey: UINT,
        uScanCode: UINT,
        lpKeyState: *const BYTE,
        lpChar: LPWORD,
        uFlags: UINT,
        dwhkl: HKL,
    ) -> c_int;
    pub fn ToUnicode(
        wVirtKey: UINT,
        wScanCode: UINT,
        lpKeyState: *const BYTE,
        lwszBuff: LPWSTR,
        cchBuff: c_int,
        wFlags: UINT,
    ) -> c_int;
    pub fn OemKeyScan(
        wOemChar: WORD,
    ) -> DWORD;
    pub fn VkKeyScanA(
        ch: CHAR,
    ) -> SHORT;
    pub fn VkKeyScanW(
        ch: WCHAR,
    ) -> SHORT;
    pub fn VkKeyScanExA(
        ch: CHAR,
        dwhkl: HKL,
    ) -> SHORT;
    pub fn VkKeyScanExW(
        ch: WCHAR,
        dwhkl: HKL,
    ) -> SHORT;
}
pub const KEYEVENTF_EXTENDEDKEY: DWORD = 0x0001;
pub const KEYEVENTF_KEYUP: DWORD = 0x0002;
pub const KEYEVENTF_UNICODE: DWORD = 0x0004;
pub const KEYEVENTF_SCANCODE: DWORD = 0x0008;
extern "system" {
    pub fn keybd_event(
        bVk: BYTE,
        bScan: BYTE,
        dwFlags: DWORD,
        dwExtraInfo: ULONG_PTR,
    );
}
pub const MOUSEEVENTF_MOVE: DWORD = 0x0001;
pub const MOUSEEVENTF_LEFTDOWN: DWORD = 0x0002;
pub const MOUSEEVENTF_LEFTUP: DWORD = 0x0004;
pub const MOUSEEVENTF_RIGHTDOWN: DWORD = 0x0008;
pub const MOUSEEVENTF_RIGHTUP: DWORD = 0x0010;
pub const MOUSEEVENTF_MIDDLEDOWN: DWORD = 0x0020;
pub const MOUSEEVENTF_MIDDLEUP: DWORD = 0x0040;
pub const MOUSEEVENTF_XDOWN: DWORD = 0x0080;
pub const MOUSEEVENTF_XUP: DWORD = 0x0100;
pub const MOUSEEVENTF_WHEEL: DWORD = 0x0800;
pub const MOUSEEVENTF_HWHEEL: DWORD = 0x01000;
pub const MOUSEEVENTF_MOVE_NOCOALESCE: DWORD = 0x2000;
pub const MOUSEEVENTF_VIRTUALDESK: DWORD = 0x4000;
pub const MOUSEEVENTF_ABSOLUTE: DWORD = 0x8000;
extern "system" {
    pub fn mouse_event(
        dwFlags: DWORD,
        dx: DWORD,
        dy: DWORD,
        dwData: DWORD,
        dwExtraInfo: ULONG_PTR,
    );
}
STRUCT!{struct MOUSEINPUT {
    dx: LONG,
    dy: LONG,
    mouseData: DWORD,
    dwFlags: DWORD,
    time: DWORD,
    dwExtraInfo: ULONG_PTR,
}}
pub type PMOUSEINPUT = *mut MOUSEINPUT;
pub type LPMOUSEINPUT = *mut MOUSEINPUT;
STRUCT!{struct KEYBDINPUT {
    wVk: WORD,
    wScan: WORD,
    dwFlags: DWORD,
    time: DWORD,
    dwExtraInfo: ULONG_PTR,
}}
pub type PKEYBDINPUT = *mut KEYBDINPUT;
pub type LPKEYBDINPUT = *mut KEYBDINPUT;
STRUCT!{struct HARDWAREINPUT {
    uMsg: DWORD,
    wParamL: WORD,
    wParamH: WORD,
}}
pub type PHARDWAREINPUT = *mut HARDWAREINPUT;
pub type LPHARDWAREINPUT= *mut HARDWAREINPUT;
pub const INPUT_MOUSE: DWORD = 0;
pub const INPUT_KEYBOARD: DWORD = 1;
pub const INPUT_HARDWARE: DWORD = 2;
UNION!{union INPUT_u {
    [u32; 6] [u64; 4],
    mi mi_mut: MOUSEINPUT,
    ki ki_mut: KEYBDINPUT,
    hi hi_mut: HARDWAREINPUT,
}}
STRUCT!{struct INPUT {
    type_: DWORD,
    u: INPUT_u,
}}
pub type PINPUT = *mut INPUT;
pub type LPINPUT = *mut INPUT;
extern "system" {
    pub fn SendInput(
        cInputs: UINT,
        pInputs: LPINPUT,
        cbSize: c_int,
    ) -> UINT;
}
DECLARE_HANDLE!{HTOUCHINPUT, HTOUCHINPUT__}
STRUCT!{struct TOUCHINPUT {
    x: LONG,
    y: LONG,
    hSource: HANDLE,
    dwID: DWORD,
    dwFlags: DWORD,
    dwMask: DWORD,
    dwTime: DWORD,
    dwExtraInfo: ULONG_PTR,
    cxContact: DWORD,
    cyContact: DWORD,
}}
pub type PTOUCHINPUT = *mut TOUCHINPUT;
pub type PCTOUCHINPUT = *const TOUCHINPUT;
// TOUCH_COORD_TO_PIXEL
pub const TOUCHEVENTF_MOVE: DWORD = 0x0001;
pub const TOUCHEVENTF_DOWN: DWORD = 0x0002;
pub const TOUCHEVENTF_UP: DWORD = 0x0004;
pub const TOUCHEVENTF_INRANGE: DWORD = 0x0008;
pub const TOUCHEVENTF_PRIMARY: DWORD = 0x0010;
pub const TOUCHEVENTF_NOCOALESCE: DWORD = 0x0020;
pub const TOUCHEVENTF_PEN: DWORD = 0x0040;
pub const TOUCHEVENTF_PALM: DWORD = 0x0080;
pub const TOUCHINPUTMASKF_TIMEFROMSYSTEM: DWORD = 0x0001;
pub const TOUCHINPUTMASKF_EXTRAINFO: DWORD = 0x0002;
pub const TOUCHINPUTMASKF_CONTACTAREA: DWORD = 0x0004;
extern "system" {
    pub fn GetTouchInputInfo(
        hTouchInput: HTOUCHINPUT,
        cInputs: c_uint,
        pInputs: PTOUCHINPUT,
        cbSize: c_int,
    ) -> BOOL;
    pub fn CloseTouchInputHandle(
        hTouchInput: HTOUCHINPUT,
    ) -> BOOL;
}
pub const TWF_FINETOUCH: DWORD = 0x00000001;
pub const TWF_WANTPALM: DWORD = 0x00000002;
extern "system" {
    pub fn RegisterTouchWindow(
        hWnd: HWND,
        flags: ULONG,
    ) -> BOOL;
    pub fn UnregisterTouchWindow(
        hwnd: HWND,
    ) -> BOOL;
    pub fn IsTouchWindow(
        hwnd: HWND,
        pulFlags: PULONG,
    ) -> BOOL;
}
ENUM!{enum POINTER_INPUT_TYPE {
    PT_POINTER = 0x00000001,
    PT_TOUCH = 0x00000002,
    PT_PEN = 0x00000003,
    PT_MOUSE = 0x00000004,
    PT_TOUCHPAD = 0x00000005,
}}
ENUM!{enum POINTER_FLAGS {
    POINTER_FLAG_NONE = 0x00000000,
    POINTER_FLAG_NEW = 0x00000001,
    POINTER_FLAG_INRANGE = 0x00000002,
    POINTER_FLAG_INCONTACT = 0x00000004,
    POINTER_FLAG_FIRSTBUTTON = 0x00000010,
    POINTER_FLAG_SECONDBUTTON = 0x00000020,
    POINTER_FLAG_THIRDBUTTON = 0x00000040,
    POINTER_FLAG_FOURTHBUTTON = 0x00000080,
    POINTER_FLAG_FIFTHBUTTON = 0x00000100,
    POINTER_FLAG_PRIMARY = 0x00002000,
    POINTER_FLAG_CONFIDENCE = 0x00004000,
    POINTER_FLAG_CANCELED = 0x00008000,
    POINTER_FLAG_DOWN = 0x00010000,
    POINTER_FLAG_UPDATE = 0x00020000,
    POINTER_FLAG_UP = 0x00040000,
    POINTER_FLAG_WHEEL = 0x00080000,
    POINTER_FLAG_HWHEEL = 0x00100000,
    POINTER_FLAG_CAPTURECHANGED = 0x00200000,
    POINTER_FLAG_HASTRANSFORM = 0x00400000,
}}
pub const POINTER_MOD_SHIFT: DWORD = 0x0004;
pub const POINTER_MOD_CTRL: DWORD = 0x0008;
ENUM!{enum POINTER_BUTTON_CHANGE_TYPE {
    POINTER_CHANGE_NONE,
    POINTER_CHANGE_FIRSTBUTTON_DOWN,
    POINTER_CHANGE_FIRSTBUTTON_UP,
    POINTER_CHANGE_SECONDBUTTON_DOWN,
    POINTER_CHANGE_SECONDBUTTON_UP,
    POINTER_CHANGE_THIRDBUTTON_DOWN,
    POINTER_CHANGE_THIRDBUTTON_UP,
    POINTER_CHANGE_FOURTHBUTTON_DOWN,
    POINTER_CHANGE_FOURTHBUTTON_UP,
    POINTER_CHANGE_FIFTHBUTTON_DOWN,
    POINTER_CHANGE_FIFTHBUTTON_UP,
}}
STRUCT!{struct POINTER_INFO {
    pointerType: POINTER_INPUT_TYPE,
    pointerId: UINT32,
    frameId: UINT32,
    pointerFlags: POINTER_FLAGS,
    sourceDevice: HANDLE,
    hwndTarget: HWND,
    ptPixelLocation: POINT,
    ptHimetricLocation: POINT,
    ptPixelLocationRaw: POINT,
    ptHimetricLocationRaw: POINT,
    dwTime: DWORD,
    historyCount: UINT32,
    InputData: INT32,
    dwKeyStates: DWORD,
    PerformanceCount: UINT64,
    ButtonChangeType: POINTER_BUTTON_CHANGE_TYPE,
}}
ENUM!{enum TOUCH_FLAGS {
    TOUCH_FLAG_NONE = 0x00000000,
}}
ENUM!{enum TOUCH_MASK {
    TOUCH_MASK_NONE = 0x00000000,
    TOUCH_MASK_CONTACTAREA = 0x00000001,
    TOUCH_MASK_ORIENTATION = 0x00000002,
    TOUCH_MASK_PRESSURE = 0x00000004,
}}
STRUCT!{struct POINTER_TOUCH_INFO {
    pointerInfo: POINTER_INFO,
    touchFlags: TOUCH_FLAGS,
    touchMask: TOUCH_MASK,
    rcContact: RECT,
    rcContactRaw: RECT,
    orientation: UINT32,
    pressure: UINT32,
}}
ENUM!{enum PEN_FLAGS {
    PEN_FLAG_NONE = 0x00000000,
    PEN_FLAG_BARREL = 0x00000001,
    PEN_FLAG_INVERTED = 0x00000002,
    PEN_FLAG_ERASER = 0x00000004,
}}
ENUM!{enum PEN_MASK {
    PEN_MASK_NONE = 0x00000000,
    PEN_MASK_PRESSURE = 0x00000001,
    PEN_MASK_ROTATION = 0x00000002,
    PEN_MASK_TILT_X = 0x00000004,
    PEN_MASK_TILT_Y = 0x00000008,
}}
STRUCT!{struct POINTER_PEN_INFO {
    pointerInfo: POINTER_INFO,
    penFlags: PEN_FLAGS,
    penMask: PEN_MASK,
    pressure: UINT32,
    rotation: UINT32,
    tiltX: INT32,
    tiltY: INT32,
}}
pub const POINTER_MESSAGE_FLAG_NEW: DWORD = 0x00000001;
pub const POINTER_MESSAGE_FLAG_INRANGE: DWORD = 0x00000002;
pub const POINTER_MESSAGE_FLAG_INCONTACT: DWORD = 0x00000004;
pub const POINTER_MESSAGE_FLAG_FIRSTBUTTON: DWORD = 0x00000010;
pub const POINTER_MESSAGE_FLAG_SECONDBUTTON: DWORD = 0x00000020;
pub const POINTER_MESSAGE_FLAG_THIRDBUTTON: DWORD = 0x00000040;
pub const POINTER_MESSAGE_FLAG_FOURTHBUTTON: DWORD = 0x00000080;
pub const POINTER_MESSAGE_FLAG_FIFTHBUTTON: DWORD = 0x00000100;
pub const POINTER_MESSAGE_FLAG_PRIMARY: DWORD = 0x00002000;
pub const POINTER_MESSAGE_FLAG_CONFIDENCE: DWORD = 0x00004000;
pub const POINTER_MESSAGE_FLAG_CANCELED: DWORD = 0x00008000;
pub const PA_ACTIVATE: UINT = MA_ACTIVATE;
pub const PA_NOACTIVATE: UINT = MA_NOACTIVATE;
pub const MAX_TOUCH_COUNT: UINT32 = 256;
pub const TOUCH_FEEDBACK_DEFAULT: DWORD = 0x1;
pub const TOUCH_FEEDBACK_INDIRECT: DWORD = 0x2;
pub const TOUCH_FEEDBACK_NONE: DWORD = 0x3;
ENUM!{enum POINTER_FEEDBACK_MODE {
    POINTER_FEEDBACK_DEFAULT = 1,
    POINTER_FEEDBACK_INDIRECT = 2,
    POINTER_FEEDBACK_NONE = 3,
}}
extern "system" {
    pub fn InitializeTouchInjection(
        maxCount: UINT32,
        dwMode: DWORD,
    ) -> BOOL;
    pub fn InjectTouchInput(
        count: UINT32,
        contacts: *const POINTER_TOUCH_INFO,
    ) -> BOOL;
}
STRUCT!{struct USAGE_PROPERTIES {
    level: USHORT,
    page: USHORT,
    usage: USHORT,
    logicalMinimum: INT32,
    logicalMaximum: INT32,
    unit: USHORT,
    exponent: USHORT,
    count: BYTE,
    physicalMinimum: INT32,
    physicalMaximum: INT32,
}}
pub type PUSAGE_PROPERTIES = *mut USAGE_PROPERTIES;
UNION!{union POINTER_TYPE_INFO_u {
    [u64; 17] [u64; 18],
    touchInfo touchInfo_mut: POINTER_TOUCH_INFO,
    penInfo penInfo_mut: POINTER_PEN_INFO,
}}
STRUCT!{struct POINTER_TYPE_INFO {
    type_: POINTER_INPUT_TYPE,
    u: POINTER_TYPE_INFO_u,
}}
pub type PPOINTER_TYPE_INFO = *mut POINTER_TYPE_INFO;
STRUCT!{struct INPUT_INJECTION_VALUE {
    page: USHORT,
    usage: USHORT,
    value: INT32,
    index: USHORT,
}}
pub type PINPUT_INJECTION_VALUE = *mut INPUT_INJECTION_VALUE;
extern "system" {
    pub fn GetPointerType(
        pointerId: UINT32,
        pointerType: *mut POINTER_INPUT_TYPE,
    ) -> BOOL;
    pub fn GetPointerCursorId(
        pointerId: UINT32,
        cursorId: *mut UINT32,
    ) -> BOOL;
    pub fn GetPointerInfo(
        pointerId: UINT32,
        pointerInfo: *mut POINTER_INFO,
    ) -> BOOL;
    pub fn GetPointerInfoHistory(
        pointerId: UINT32,
        entriesCount: *mut UINT32,
        pointerInfo: *mut POINTER_INFO,
    ) -> BOOL;
    pub fn GetPointerFrameInfo(
        pointerId: UINT32,
        pointerCount: *mut UINT32,
        pointerInfo: *mut POINTER_INFO,
    ) -> BOOL;
    pub fn GetPointerFrameInfoHistory(
        pointerId: UINT32,
        entriesCount: *mut UINT32,
        pointerCount: *mut UINT32,
        pointerInfo: *mut POINTER_INFO,
    ) -> BOOL;
    pub fn GetPointerTouchInfo(
        pointerId: UINT32,
        touchInfo: *mut POINTER_TOUCH_INFO,
    ) -> BOOL;
    pub fn GetPointerTouchInfoHistory(
        pointerId: UINT32,
        entriesCount: *mut UINT32,
        touchInfo: *mut POINTER_TOUCH_INFO,
    ) -> BOOL;
    pub fn GetPointerFrameTouchInfo(
        pointerId: UINT32,
        pointerCount: *mut UINT32,
        touchInfo: *mut POINTER_TOUCH_INFO,
    ) -> BOOL;
    pub fn GetPointerFrameTouchInfoHistory(
        pointerId: UINT32,
        entriesCount: *mut UINT32,
        pointerCount: *mut UINT32,
        touchInfo: *mut POINTER_TOUCH_INFO,
    ) -> BOOL;
    pub fn GetPointerPenInfo(
        pointerId: UINT32,
        penInfo: *mut POINTER_PEN_INFO,
    ) -> BOOL;
    pub fn GetPointerPenInfoHistory(
        pointerId: UINT32,
        entriesCount: *mut UINT32,
        penInfo: *mut POINTER_PEN_INFO,
    ) -> BOOL;
    pub fn GetPointerFramePenInfo(
        pointerId: UINT32,
        pointerCount: *mut UINT32,
        penInfo: *mut POINTER_PEN_INFO,
    ) -> BOOL;
    pub fn GetPointerFramePenInfoHistory(
        pointerId: UINT32,
        entriesCount: *mut UINT32,
        pointerCount: *mut UINT32,
        penInfo: *mut POINTER_PEN_INFO,
    ) -> BOOL;
    pub fn SkipPointerFrameMessages(
        pointerId: UINT32,
    ) -> BOOL;
    pub fn RegisterPointerInputTarget(
        hwnd: HWND,
        pointerType: POINTER_INPUT_TYPE,
    ) -> BOOL;
    pub fn UnregisterPointerInputTarget(
        hwnd: HWND,
        pointerType: POINTER_INPUT_TYPE,
    ) -> BOOL;
    pub fn RegisterPointerInputTargetEx(
        hwnd: HWND,
        pointerType: POINTER_INPUT_TYPE,
        fObserve: BOOL,
    ) -> BOOL;
    pub fn UnregisterPointerInputTargetEx(
        hwnd: HWND,
        pointerType: POINTER_INPUT_TYPE,
    ) -> BOOL;
}
DECLARE_HANDLE!{HSYNTHETICPOINTERDEVICE, HSYNTHETICPOINTERDEVICE__}
extern "system" {
    pub fn CreateSyntheticPointerDevice(
        pointerType: POINTER_INPUT_TYPE,
        maxCount: ULONG,
        mode: POINTER_FEEDBACK_MODE,
    ) -> HSYNTHETICPOINTERDEVICE;
    pub fn InjectSyntheticPointerInput(
        device: HSYNTHETICPOINTERDEVICE,
        pointerInfo: *const POINTER_TYPE_INFO,
        count: UINT32,
    ) -> BOOL;
    pub fn DestroySyntheticPointerDevice(
        device: HSYNTHETICPOINTERDEVICE,
    );
}
extern "system" {
    pub fn EnableMouseInPointer(
        fEnable: BOOL,
    ) -> BOOL;
    pub fn IsMouseInPointerEnabled() -> BOOL;
}
pub const TOUCH_HIT_TESTING_DEFAULT: ULONG = 0x0;
pub const TOUCH_HIT_TESTING_CLIENT: ULONG = 0x1;
pub const TOUCH_HIT_TESTING_NONE: ULONG = 0x2;
extern "system" {
    pub fn RegisterTouchHitTestingWindow(
        hwnd: HWND,
        value: ULONG,
    ) -> BOOL;
}
STRUCT!{struct TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    score: UINT16,
    adjustedPoint: POINT,
}}
pub type PTOUCH_HIT_TESTING_PROXIMITY_EVALUATION = *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION;
STRUCT!{struct TOUCH_HIT_TESTING_INPUT {
    pointerId: UINT32,
    point: POINT,
    boundingBox: RECT,
    nonOccludedBoundingBox: RECT,
    orientation: UINT32,
}}
pub type PTOUCH_HIT_TESTING_INPUT = *mut TOUCH_HIT_TESTING_INPUT;
pub const TOUCH_HIT_TESTING_PROXIMITY_CLOSEST: UINT16 = 0x0;
pub const TOUCH_HIT_TESTING_PROXIMITY_FARTHEST: UINT16 = 0xFFF;
extern "system" {
    pub fn EvaluateProximityToRect(
        controlBoundingBox: *const RECT,
        pHitTestingInput: *const TOUCH_HIT_TESTING_INPUT,
        pProximityEval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION,
    ) -> BOOL;
    pub fn EvaluateProximityToPolygon(
        numVertices: UINT32,
        controlPolygon: *const POINT,
        pHitTestingInput: *const TOUCH_HIT_TESTING_INPUT,
        pProximityEval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION,
    ) -> BOOL;
    pub fn PackTouchHitTestingProximityEvaluation(
        pHitTestingInput: *const TOUCH_HIT_TESTING_INPUT,
        pProximityEval: *const TOUCH_HIT_TESTING_PROXIMITY_EVALUATION,
    ) -> LRESULT;
}
ENUM!{enum FEEDBACK_TYPE {
    FEEDBACK_TOUCH_CONTACTVISUALIZATION = 1,
    FEEDBACK_PEN_BARRELVISUALIZATION = 2,
    FEEDBACK_PEN_TAP = 3,
    FEEDBACK_PEN_DOUBLETAP = 4,
    FEEDBACK_PEN_PRESSANDHOLD = 5,
    FEEDBACK_PEN_RIGHTTAP = 6,
    FEEDBACK_TOUCH_TAP = 7,
    FEEDBACK_TOUCH_DOUBLETAP = 8,
    FEEDBACK_TOUCH_PRESSANDHOLD = 9,
    FEEDBACK_TOUCH_RIGHTTAP = 10,
    FEEDBACK_GESTURE_PRESSANDTAP = 11,
    FEEDBACK_MAX = 0xFFFFFFFF,
}}
pub const GWFS_INCLUDE_ANCESTORS: DWORD = 0x00000001;
extern "system" {
    pub fn GetWindowFeedbackSetting(
        hwnd: HWND,
        feedback: FEEDBACK_TYPE,
        dwFlags: DWORD,
        pSize: *mut UINT32,
        config: *mut VOID,
    ) -> BOOL;
    pub fn SetWindowFeedbackSetting(
        hwnd: HWND,
        feedback: FEEDBACK_TYPE,
        dwFlags: DWORD,
        size: UINT32,
        configuration: *const VOID,
    ) -> BOOL;
}
STRUCT!{struct INPUT_TRANSFORM {
    m: [[f32; 4]; 4],
}}
extern "system" {
    pub fn GetPointerInputTransform(
        pointerId: UINT32,
        historyCount: UINT32,
        inputTransform: *mut INPUT_TRANSFORM,
    ) -> BOOL;
}
STRUCT!{struct LASTINPUTINFO {
    cbSize: UINT,
    dwTime: DWORD,
}}
pub type PLASTINPUTINFO = *mut LASTINPUTINFO;
extern "system" {
    pub fn GetLastInputInfo(
        plii: PLASTINPUTINFO,
    ) -> BOOL;
    pub fn MapVirtualKeyA(
        nCode: UINT,
        uMapType: UINT,
    ) -> UINT;
    pub fn MapVirtualKeyW(
        nCode: UINT,
        uMapType: UINT,
    ) -> UINT;
    pub fn MapVirtualKeyExA(
        nCode: UINT,
        uMapType: UINT,
        dwhkl: HKL,
    ) -> UINT;
    pub fn MapVirtualKeyExW(
        nCode: UINT,
        uMapType: UINT,
        dwhkl: HKL,
    ) -> UINT;
}
pub const MAPVK_VK_TO_VSC: UINT = 0;
pub const MAPVK_VSC_TO_VK: UINT = 1;
pub const MAPVK_VK_TO_CHAR: UINT = 2;
pub const MAPVK_VSC_TO_VK_EX: UINT = 3;
pub const MAPVK_VK_TO_VSC_EX: UINT = 4;
extern "system" {
    pub fn GetInputState() -> BOOL;
    pub fn GetQueueStatus(
        flags: UINT,
    ) -> DWORD;
    pub fn GetCapture() -> HWND;
    pub fn SetCapture(
        hWnd: HWND,
    ) -> HWND;
    pub fn ReleaseCapture() -> BOOL;
    pub fn MsgWaitForMultipleObjects(
        nCount: DWORD,
        pHandles: *const HANDLE,
        fWaitAll: BOOL,
        dwMilliseconds: DWORD,
        dwWakeMask: DWORD,
    ) -> DWORD;
    pub fn MsgWaitForMultipleObjectsEx(
        nCount: DWORD,
        pHandles: *const HANDLE,
        dwMilliseconds: DWORD,
        dwWakeMask: DWORD,
        dwFlags: DWORD,
    ) -> DWORD;
}
pub const MWMO_WAITALL: UINT = 0x0001;
pub const MWMO_ALERTABLE: UINT = 0x0002;
pub const MWMO_INPUTAVAILABLE: UINT = 0x0004;
pub const QS_KEY: UINT = 0x0001;
pub const QS_MOUSEMOVE: UINT = 0x0002;
pub const QS_MOUSEBUTTON: UINT = 0x0004;
pub const QS_POSTMESSAGE: UINT = 0x0008;
pub const QS_TIMER: UINT = 0x0010;
pub const QS_PAINT: UINT = 0x0020;
pub const QS_SENDMESSAGE: UINT = 0x0040;
pub const QS_HOTKEY: UINT = 0x0080;
pub const QS_ALLPOSTMESSAGE: UINT = 0x0100;
pub const QS_RAWINPUT: UINT = 0x0400;
pub const QS_TOUCH: UINT = 0x0800;
pub const QS_POINTER: UINT = 0x1000;
pub const QS_MOUSE: UINT = QS_MOUSEMOVE | QS_MOUSEBUTTON;
pub const QS_INPUT: UINT = QS_MOUSE | QS_KEY | QS_RAWINPUT | QS_TOUCH | QS_POINTER;
pub const QS_ALLEVENTS: UINT = QS_INPUT | QS_POSTMESSAGE | QS_TIMER | QS_PAINT | QS_HOTKEY;
pub const QS_ALLINPUT: UINT = QS_INPUT | QS_POSTMESSAGE | QS_TIMER | QS_PAINT | QS_HOTKEY
    | QS_SENDMESSAGE;
pub const USER_TIMER_MAXIMUM: UINT = 0x7FFFFFFF;
pub const USER_TIMER_MINIMUM: UINT = 0x0000000A;
extern "system" {
    pub fn SetTimer(
        hWnd: HWND,
        nIDEvent: UINT_PTR,
        uElapse: UINT,
        lpTimerFunc: TIMERPROC,
    ) -> UINT_PTR;
}
pub const TIMERV_DEFAULT_COALESCING: ULONG = 0;
pub const TIMERV_NO_COALESCING: ULONG = 0xFFFFFFFF;
pub const TIMERV_COALESCING_MIN: ULONG = 1;
pub const TIMERV_COALESCING_MAX: ULONG = 0x7FFFFFF5;
extern "system" {
    pub fn SetCoalescableTimer(
        hWnd: HWND,
        nIDEvent: UINT_PTR,
        uElapse: UINT,
        lpTimerFunc: TIMERPROC,
        uToleranceDelay: ULONG,
    ) -> UINT_PTR;
    pub fn KillTimer(
        hWnd: HWND,
        uIDEvent: UINT_PTR,
    ) -> BOOL;
    pub fn IsWindowUnicode(
        hWnd: HWND,
    ) -> BOOL;
    pub fn EnableWindow(
        hWnd: HWND,
        bEnable: BOOL,
    ) -> BOOL;
    pub fn IsWindowEnabled(
        hWnd: HWND,
    ) -> BOOL;
    pub fn LoadAcceleratorsA(
        hInstance: HINSTANCE,
        lpTableName: LPCSTR,
    ) -> HACCEL;
    pub fn LoadAcceleratorsW(
        hInstance: HINSTANCE,
        lpTableName: LPCWSTR,
    ) -> HACCEL;
    pub fn CreateAcceleratorTableA(
        paccel: LPACCEL,
        cAccel: c_int,
    ) -> HACCEL;
    pub fn CreateAcceleratorTableW(
        paccel: LPACCEL,
        cAccel: c_int,
    ) -> HACCEL;
    pub fn DestroyAcceleratorTable(
        hAccel: HACCEL,
    ) -> BOOL;
    pub fn CopyAcceleratorTableA(
        hAccelSrc: HACCEL,
        lpAccelDst: LPACCEL,
        cAccelEntries: c_int,
    ) -> c_int;
    pub fn CopyAcceleratorTableW(
        hAccelSrc: HACCEL,
        lpAccelDst: LPACCEL,
        cAccelEntries: c_int,
    ) -> c_int;
    pub fn TranslateAcceleratorA(
        hWnd: HWND,
        hAccTable: HACCEL,
        lpMsg: LPMSG,
    ) -> c_int;
    pub fn TranslateAcceleratorW(
        hWnd: HWND,
        hAccTable: HACCEL,
        lpMsg: LPMSG,
    ) -> c_int;
}
pub const SM_CXSCREEN: c_int = 0;
pub const SM_CYSCREEN: c_int = 1;
pub const SM_CXVSCROLL: c_int = 2;
pub const SM_CYHSCROLL: c_int = 3;
pub const SM_CYCAPTION: c_int = 4;
pub const SM_CXBORDER: c_int = 5;
pub const SM_CYBORDER: c_int = 6;
pub const SM_CXDLGFRAME: c_int = 7;
pub const SM_CYDLGFRAME: c_int = 8;
pub const SM_CYVTHUMB: c_int = 9;
pub const SM_CXHTHUMB: c_int = 10;
pub const SM_CXICON: c_int = 11;
pub const SM_CYICON: c_int = 12;
pub const SM_CXCURSOR: c_int = 13;
pub const SM_CYCURSOR: c_int = 14;
pub const SM_CYMENU: c_int = 15;
pub const SM_CXFULLSCREEN: c_int = 16;
pub const SM_CYFULLSCREEN: c_int = 17;
pub const SM_CYKANJIWINDOW: c_int = 18;
pub const SM_MOUSEPRESENT: c_int = 19;
pub const SM_CYVSCROLL: c_int = 20;
pub const SM_CXHSCROLL: c_int = 21;
pub const SM_DEBUG: c_int = 22;
pub const SM_SWAPBUTTON: c_int = 23;
pub const SM_RESERVED1: c_int = 24;
pub const SM_RESERVED2: c_int = 25;
pub const SM_RESERVED3: c_int = 26;
pub const SM_RESERVED4: c_int = 27;
pub const SM_CXMIN: c_int = 28;
pub const SM_CYMIN: c_int = 29;
pub const SM_CXSIZE: c_int = 30;
pub const SM_CYSIZE: c_int = 31;
pub const SM_CXFRAME: c_int = 32;
pub const SM_CYFRAME: c_int = 33;
pub const SM_CXMINTRACK: c_int = 34;
pub const SM_CYMINTRACK: c_int = 35;
pub const SM_CXDOUBLECLK: c_int = 36;
pub const SM_CYDOUBLECLK: c_int = 37;
pub const SM_CXICONSPACING: c_int = 38;
pub const SM_CYICONSPACING: c_int = 39;
pub const SM_MENUDROPALIGNMENT: c_int = 40;
pub const SM_PENWINDOWS: c_int = 41;
pub const SM_DBCSENABLED: c_int = 42;
pub const SM_CMOUSEBUTTONS: c_int = 43;
pub const SM_CXFIXEDFRAME: c_int = SM_CXDLGFRAME;
pub const SM_CYFIXEDFRAME: c_int = SM_CYDLGFRAME;
pub const SM_CXSIZEFRAME: c_int = SM_CXFRAME;
pub const SM_CYSIZEFRAME: c_int = SM_CYFRAME;
pub const SM_SECURE: c_int = 44;
pub const SM_CXEDGE: c_int = 45;
pub const SM_CYEDGE: c_int = 46;
pub const SM_CXMINSPACING: c_int = 47;
pub const SM_CYMINSPACING: c_int = 48;
pub const SM_CXSMICON: c_int = 49;
pub const SM_CYSMICON: c_int = 50;
pub const SM_CYSMCAPTION: c_int = 51;
pub const SM_CXSMSIZE: c_int = 52;
pub const SM_CYSMSIZE: c_int = 53;
pub const SM_CXMENUSIZE: c_int = 54;
pub const SM_CYMENUSIZE: c_int = 55;
pub const SM_ARRANGE: c_int = 56;
pub const SM_CXMINIMIZED: c_int = 57;
pub const SM_CYMINIMIZED: c_int = 58;
pub const SM_CXMAXTRACK: c_int = 59;
pub const SM_CYMAXTRACK: c_int = 60;
pub const SM_CXMAXIMIZED: c_int = 61;
pub const SM_CYMAXIMIZED: c_int = 62;
pub const SM_NETWORK: c_int = 63;
pub const SM_CLEANBOOT: c_int = 67;
pub const SM_CXDRAG: c_int = 68;
pub const SM_CYDRAG: c_int = 69;
pub const SM_SHOWSOUNDS: c_int = 70;
pub const SM_CXMENUCHECK: c_int = 71;
pub const SM_CYMENUCHECK: c_int = 72;
pub const SM_SLOWMACHINE: c_int = 73;
pub const SM_MIDEASTENABLED: c_int = 74;
pub const SM_MOUSEWHEELPRESENT: c_int = 75;
pub const SM_XVIRTUALSCREEN: c_int = 76;
pub const SM_YVIRTUALSCREEN: c_int = 77;
pub const SM_CXVIRTUALSCREEN: c_int = 78;
pub const SM_CYVIRTUALSCREEN: c_int = 79;
pub const SM_CMONITORS: c_int = 80;
pub const SM_SAMEDISPLAYFORMAT: c_int = 81;
pub const SM_IMMENABLED: c_int = 82;
pub const SM_CXFOCUSBORDER: c_int = 83;
pub const SM_CYFOCUSBORDER: c_int = 84;
pub const SM_TABLETPC: c_int = 86;
pub const SM_MEDIACENTER: c_int = 87;
pub const SM_STARTER: c_int = 88;
pub const SM_SERVERR2: c_int = 89;
pub const SM_MOUSEHORIZONTALWHEELPRESENT: c_int = 91;
pub const SM_CXPADDEDBORDER: c_int = 92;
pub const SM_DIGITIZER: c_int = 94;
pub const SM_MAXIMUMTOUCHES: c_int = 95;
pub const SM_CMETRICS: c_int = 97;
pub const SM_REMOTESESSION: c_int = 0x1000;
pub const SM_SHUTTINGDOWN: c_int = 0x2000;
pub const SM_REMOTECONTROL: c_int = 0x2001;
pub const SM_CARETBLINKINGENABLED: c_int = 0x2002;
pub const SM_CONVERTIBLESLATEMODE: c_int = 0x2003;
pub const SM_SYSTEMDOCKED: c_int = 0x2004;
extern "system" {
    pub fn GetSystemMetrics(
        nIndex: c_int,
    ) -> c_int;
    pub fn GetSystemMetricsForDpi(
        nIndex: c_int,
        dpi: UINT,
    ) -> c_int;
    pub fn LoadMenuA(
        hInstance: HINSTANCE,
        lpMenuName: LPCSTR,
    ) -> HMENU;
    pub fn LoadMenuW(
        hInstance: HINSTANCE,
        lpMenuName: LPCWSTR,
    ) -> HMENU;
    pub fn LoadMenuIndirectA(
        lpMenuTemplate: *const MENUTEMPLATEA,
    ) -> HMENU;
    pub fn LoadMenuIndirectW(
        lpMenuTemplate: *const MENUTEMPLATEW,
    ) -> HMENU;
    pub fn GetMenu(
        hWnd: HWND,
    ) -> HMENU;
    pub fn SetMenu(
        hWnd: HWND,
        hMenu: HMENU,
    ) -> BOOL;
    pub fn ChangeMenuA(
        hMenu: HMENU,
        cmd: UINT,
        lpszNewItem: LPCSTR,
        cmdInsert: UINT,
        flags: UINT,
    ) -> BOOL;
    pub fn ChangeMenuW(
        hMenu: HMENU,
        cmd: UINT,
        lpszNewItem: LPCWSTR,
        cmdInsert: UINT,
        flags: UINT,
    ) -> BOOL;
    pub fn HiliteMenuItem(
        hWnd: HWND,
        hMenu: HMENU,
        uIDHiliteItem: UINT,
        uHilite: UINT,
    ) -> BOOL;
    pub fn GetMenuStringA(
        hMenu: HMENU,
        uIDItem: UINT,
        lpString: LPSTR,
        cchMax: c_int,
        flags: UINT,
    ) -> c_int;
    pub fn GetMenuStringW(
        hMenu: HMENU,
        uIDItem: UINT,
        lpString: LPWSTR,
        cchMax: c_int,
        flags: UINT,
    ) -> c_int;
    pub fn GetMenuState(
        hMenu: HMENU,
        uId: UINT,
        uFlags: UINT,
    ) -> UINT;
    pub fn DrawMenuBar(
        hwnd: HWND,
    ) -> BOOL;
}
pub const PMB_ACTIVE: DWORD = 0x00000001;
extern "system" {
    pub fn GetSystemMenu(
        hWnd: HWND,
        bRevert: BOOL,
    ) -> HMENU;
    pub fn CreateMenu() -> HMENU;
    pub fn CreatePopupMenu() ->HMENU;
    pub fn DestroyMenu(
        hMenu: HMENU,
    ) -> BOOL;
    pub fn CheckMenuItem(
        hMenu: HMENU,
        uIDCheckItem: UINT,
        uCheck: UINT,
    ) -> DWORD;
    pub fn EnableMenuItem(
        hMenu: HMENU,
        uIDEnableItem: UINT,
        uEnable: UINT,
    ) -> BOOL;
    pub fn GetSubMenu(
        hMenu: HMENU,
        nPos: c_int,
    ) -> HMENU;
    pub fn GetMenuItemID(
        hMenu: HMENU,
        nPos: c_int,
    ) -> UINT;
    pub fn GetMenuItemCount(
        hMenu: HMENU,
    ) -> c_int;
    pub fn InsertMenuA(
        hMenu: HMENU,
        uPosition: UINT,
        uFlags: UINT,
        uIDNewItem: UINT_PTR,
        lpNewItem: LPCSTR,
    ) -> BOOL;
    pub fn InsertMenuW(
        hMenu: HMENU,
        uPosition: UINT,
        uFlags: UINT,
        uIDNewItem: UINT_PTR,
        lpNewItem: LPCWSTR,
    ) -> BOOL;
    pub fn AppendMenuA(
        hMenu: HMENU,
        uFlags: UINT,
        uIDNewItem: UINT_PTR,
        lpNewItem: LPCSTR,
    ) -> BOOL;
    pub fn AppendMenuW(
        hMenu: HMENU,
        uFlags: UINT,
        uIDNewItem: UINT_PTR,
        lpNewItem: LPCWSTR,
    ) -> BOOL;
    pub fn ModifyMenuA(
        hMnu: HMENU,
        uPosition: UINT,
        uFlags: UINT,
        uIDNewItem: UINT_PTR,
        lpNewItem: LPCSTR,
    ) -> BOOL;
    pub fn ModifyMenuW(
        hMnu: HMENU,
        uPosition: UINT,
        uFlags: UINT,
        uIDNewItem: UINT_PTR,
        lpNewItem: LPCWSTR,
    ) -> BOOL;
    pub fn RemoveMenu(
        hMenu: HMENU,
        uPosition: UINT,
        uFlags: UINT,
    ) -> BOOL;
    pub fn DeleteMenu(
        hMenu: HMENU,
        uPosition: UINT,
        uFlags: UINT,
    ) -> BOOL;
    pub fn SetMenuItemBitmaps(
        hMenu: HMENU,
        uPosition: UINT,
        uFlags: UINT,
        hBitmapUnchecked: HBITMAP,
        hBitmapChecked: HBITMAP,
    ) -> BOOL;
    pub fn GetMenuCheckMarkDimensions() -> LONG;
    pub fn TrackPopupMenu(
        hMenu: HMENU,
        uFlags: UINT,
        x: c_int,
        y: c_int,
        nReserved: c_int,
        hWnd: HWND,
        prcRect: *const RECT,
    ) -> BOOL;
}
pub const MNC_IGNORE: DWORD = 0;
pub const MNC_CLOSE: DWORD = 1;
pub const MNC_EXECUTE: DWORD = 2;
pub const MNC_SELECT: DWORD = 3;
STRUCT!{struct TPMPARAMS {
    cbSize: UINT,
    rcExclude: RECT,
}}
pub type LPTPMPARAMS = *mut TPMPARAMS;
extern "system" {
    pub fn TrackPopupMenuEx(
        hMenu: HMENU,
        uFlags: UINT,
        x: INT,
        y: INT,
        hwnd: HWND,
        lptpm: LPTPMPARAMS,
    ) -> BOOL;
    pub fn CalculatePopupWindowPosition(
        anchorPoint: *const POINT,
        windowSize: *const SIZE,
        flags: UINT,
        excludeRect: *mut RECT,
        popupWindowPosition: *mut RECT,
    ) -> BOOL;
}
pub const MNS_NOCHECK: DWORD = 0x80000000;
pub const MNS_MODELESS: DWORD = 0x40000000;
pub const MNS_DRAGDROP: DWORD = 0x20000000;
pub const MNS_AUTODISMISS: DWORD = 0x10000000;
pub const MNS_NOTIFYBYPOS: DWORD = 0x08000000;
pub const MNS_CHECKORBMP: DWORD = 0x04000000;
pub const MIM_MAXHEIGHT: DWORD = 0x00000001;
pub const MIM_BACKGROUND: DWORD = 0x00000002;
pub const MIM_HELPID: DWORD = 0x00000004;
pub const MIM_MENUDATA: DWORD = 0x00000008;
pub const MIM_STYLE: DWORD = 0x00000010;
pub const MIM_APPLYTOSUBMENUS: DWORD = 0x80000000;
STRUCT!{struct MENUINFO {
    cbSize: DWORD,
    fMask: DWORD,
    dwStyle: DWORD,
    cyMax: UINT,
    hbrBack: HBRUSH,
    dwContextHelpID: DWORD,
    dwMenuData: ULONG_PTR,
}}
pub type LPMENUINFO = *mut MENUINFO;
pub type LPCMENUINFO = *const MENUINFO;
extern "system" {
    pub fn GetMenuInfo(
        hMenu: HMENU,
        lpcmi: LPMENUINFO,
    ) -> BOOL;
    pub fn SetMenuInfo(
        hMenu: HMENU,
        lpcmi: LPCMENUINFO,
    ) -> BOOL;
    pub fn EndMenu(
        hMenu: HMENU,
        uFlags: UINT,
        uIDNewItem: UINT_PTR,
        lpNewItem: LPCSTR,
    ) -> BOOL;
}
pub const MND_CONTINUE: DWORD = 0;
pub const MND_ENDMENU: DWORD = 1;
STRUCT!{struct MENUGETOBJECTINFO {
    dwFlags: DWORD,
    uPos: UINT,
    hmenu: HMENU,
    riid: PVOID,
    pvObj: PVOID,
}}
pub type PMENUGETOBJECTINFO = *mut MENUGETOBJECTINFO;
pub const MNGOF_TOPGAP: DWORD = 0x00000001;
pub const MNGOF_BOTTOMGAP: DWORD = 0x00000002;
pub const MNGO_NOINTERFACE: DWORD = 0x00000000;
pub const MNGO_NOERROR: DWORD = 0x00000001;
pub const MIIM_STATE: DWORD = 0x00000001;
pub const MIIM_ID: DWORD = 0x00000002;
pub const MIIM_SUBMENU: DWORD = 0x00000004;
pub const MIIM_CHECKMARKS: DWORD = 0x00000008;
pub const MIIM_TYPE: DWORD = 0x00000010;
pub const MIIM_DATA: DWORD = 0x00000020;
pub const MIIM_STRING: DWORD = 0x00000040;
pub const MIIM_BITMAP: DWORD = 0x00000080;
pub const MIIM_FTYPE: DWORD = 0x00000100;
pub const HBMMENU_CALLBACK: HBITMAP = -1isize as HBITMAP;
pub const HBMMENU_SYSTEM: HBITMAP = 1 as HBITMAP;
pub const HBMMENU_MBAR_RESTORE: HBITMAP = 2 as HBITMAP;
pub const HBMMENU_MBAR_MINIMIZE: HBITMAP = 3 as HBITMAP;
pub const HBMMENU_MBAR_CLOSE: HBITMAP = 5 as HBITMAP;
pub const HBMMENU_MBAR_CLOSE_D: HBITMAP = 6 as HBITMAP;
pub const HBMMENU_MBAR_MINIMIZE_D: HBITMAP = 7 as HBITMAP;
pub const HBMMENU_POPUP_CLOSE: HBITMAP = 8 as HBITMAP;
pub const HBMMENU_POPUP_RESTORE: HBITMAP = 9 as HBITMAP;
pub const HBMMENU_POPUP_MAXIMIZE: HBITMAP = 10 as HBITMAP;
pub const HBMMENU_POPUP_MINIMIZE: HBITMAP = 11 as HBITMAP;
STRUCT!{struct MENUITEMINFOA {
    cbSize: UINT,
    fMask: UINT,
    fType: UINT,
    fState: UINT,
    wID: UINT,
    hSubMenu: HMENU,
    hbmpChecked: HBITMAP,
    hbmpUnchecked: HBITMAP,
    dwItemData: ULONG_PTR,
    dwTypeData: LPSTR,
    cch: UINT,
    hbmpItem: HBITMAP,
}}
pub type LPMENUITEMINFOA = *mut MENUITEMINFOA;
pub type LPCMENUITEMINFOA = *const MENUITEMINFOA;
STRUCT!{struct MENUITEMINFOW {
    cbSize: UINT,
    fMask: UINT,
    fType: UINT,
    fState: UINT,
    wID: UINT,
    hSubMenu: HMENU,
    hbmpChecked: HBITMAP,
    hbmpUnchecked: HBITMAP,
    dwItemData: ULONG_PTR,
    dwTypeData: LPWSTR,
    cch: UINT,
    hbmpItem: HBITMAP,
}}
pub type LPMENUITEMINFOW = *mut MENUITEMINFOW;
pub type LPCMENUITEMINFOW = *const MENUITEMINFOW;
extern "system" {
    pub fn InsertMenuItemA(
        hmenu: HMENU,
        item: UINT,
        fByPosition: BOOL,
        lpmi: LPCMENUITEMINFOA,
    ) -> BOOL;
    pub fn InsertMenuItemW(
        hmenu: HMENU,
        item: UINT,
        fByPosition: BOOL,
        lpmi: LPCMENUITEMINFOW,
    ) -> BOOL;
    pub fn GetMenuItemInfoA(
        hMenu: HMENU,
        uItem: UINT,
        fByPosition: BOOL,
        lpmii: LPMENUITEMINFOA,
    ) -> BOOL;
    pub fn GetMenuItemInfoW(
        hMenu: HMENU,
        uItem: UINT,
        fByPosition: BOOL,
        lpmii: LPMENUITEMINFOW,
    ) -> BOOL;
    pub fn SetMenuItemInfoA(
        hmenu: HMENU,
        item: UINT,
        fByPositon: BOOL,
        lpmii: LPCMENUITEMINFOA,
    ) -> BOOL;
    pub fn SetMenuItemInfoW(
        hmenu: HMENU,
        item: UINT,
        fByPositon: BOOL,
        lpmii: LPCMENUITEMINFOW,
    ) -> BOOL;
}
pub const GMDI_USEDISABLED: DWORD = 0x0001;
pub const GMDI_GOINTOPOPUPS: DWORD = 0x0002;
extern "system" {
    pub fn GetMenuDefaultItem(
        hMenu: HMENU,
        fByPos: UINT,
        gmdiFlags: UINT,
    ) -> UINT;
    pub fn SetMenuDefaultItem(
        hMenu: HMENU,
        uItem: UINT,
        fByPos: UINT,
    ) -> BOOL;
    pub fn GetMenuItemRect(
        hWnd: HWND,
        hMenu: HMENU,
        uItem: UINT,
        lprcItem: LPRECT,
    ) -> BOOL;
    pub fn MenuItemFromPoint(
        hWnd: HWND,
        hMenu: HMENU,
        ptScreen: POINT,
    ) -> c_int;
}
pub const TPM_LEFTBUTTON: UINT = 0x0000;
pub const TPM_RIGHTBUTTON: UINT = 0x0002;
pub const TPM_LEFTALIGN: UINT = 0x0000;
pub const TPM_CENTERALIGN: UINT = 0x0004;
pub const TPM_RIGHTALIGN: UINT = 0x0008;
pub const TPM_TOPALIGN: UINT = 0x0000;
pub const TPM_VCENTERALIGN: UINT = 0x0010;
pub const TPM_BOTTOMALIGN: UINT = 0x0020;
pub const TPM_HORIZONTAL: UINT = 0x0000;
pub const TPM_VERTICAL: UINT = 0x0040;
pub const TPM_NONOTIFY: UINT = 0x0080;
pub const TPM_RETURNCMD: UINT = 0x0100;
pub const TPM_RECURSE: UINT = 0x0001;
pub const TPM_HORPOSANIMATION: UINT = 0x0400;
pub const TPM_HORNEGANIMATION: UINT = 0x0800;
pub const TPM_VERPOSANIMATION: UINT = 0x1000;
pub const TPM_VERNEGANIMATION: UINT = 0x2000;
pub const TPM_NOANIMATION: UINT = 0x4000;
pub const TPM_LAYOUTRTL: UINT = 0x8000;
pub const TPM_WORKAREA: UINT = 0x10000;
STRUCT!{struct DROPSTRUCT {
    hwndSource: HWND,
    hwndSink: HWND,
    wFmt: DWORD,
    dwData: ULONG_PTR,
    ptDrop: POINT,
    dwControlData: DWORD,
}}
pub type PDROPSTRUCT = *mut DROPSTRUCT;
pub type LPDROPSTRUCT = *mut DROPSTRUCT;
pub const DOF_EXECUTABLE: DWORD = 0x8001;
pub const DOF_DOCUMENT: DWORD = 0x8002;
pub const DOF_DIRECTORY: DWORD = 0x8003;
pub const DOF_MULTIPLE: DWORD = 0x8004;
pub const DOF_PROGMAN: DWORD = 0x0001;
pub const DOF_SHELLDATA: DWORD = 0x0002;
pub const DO_DROPFILE: DWORD = 0x454C4946;
pub const DO_PRINTFILE: DWORD = 0x544E5250;
extern "system" {
    pub fn DragObject(
        hwndParent: HWND,
        hwndFrom: HWND,
        fmt: UINT,
        data: ULONG_PTR,
        hcur: HCURSOR,
    ) -> DWORD;
    pub fn DragDetect(
        hwnd: HWND,
        pt: POINT,
    ) -> BOOL;
    pub fn DrawIcon(
        hDC: HDC,
        x: c_int,
        y: c_int,
        hIcon: HICON,
    ) -> BOOL;
}
pub const DT_TOP: UINT = 0x00000000;
pub const DT_LEFT: UINT = 0x00000000;
pub const DT_CENTER: UINT = 0x00000001;
pub const DT_RIGHT: UINT = 0x00000002;
pub const DT_VCENTER: UINT = 0x00000004;
pub const DT_BOTTOM: UINT = 0x00000008;
pub const DT_WORDBREAK: UINT = 0x00000010;
pub const DT_SINGLELINE: UINT = 0x00000020;
pub const DT_EXPANDTABS: UINT = 0x00000040;
pub const DT_TABSTOP: UINT = 0x00000080;
pub const DT_NOCLIP: UINT = 0x00000100;
pub const DT_EXTERNALLEADING: UINT = 0x00000200;
pub const DT_CALCRECT: UINT = 0x00000400;
pub const DT_NOPREFIX: UINT = 0x00000800;
pub const DT_INTERNAL: UINT = 0x00001000;
pub const DT_EDITCONTROL: UINT = 0x00002000;
pub const DT_PATH_ELLIPSIS: UINT = 0x00004000;
pub const DT_END_ELLIPSIS: UINT = 0x00008000;
pub const DT_MODIFYSTRING: UINT = 0x00010000;
pub const DT_RTLREADING: UINT = 0x00020000;
pub const DT_WORD_ELLIPSIS: UINT = 0x00040000;
pub const DT_NOFULLWIDTHCHARBREAK: UINT = 0x00080000;
pub const DT_HIDEPREFIX: UINT = 0x00100000;
pub const DT_PREFIXONLY: UINT = 0x00200000;
STRUCT!{struct DRAWTEXTPARAMS {
    cbSize: UINT,
    iTabLength: c_int,
    iLeftMargin: c_int,
    iRightMargin: c_int,
    uiLengthDrawn: UINT,
}}
pub type LPDRAWTEXTPARAMS = *mut DRAWTEXTPARAMS;
extern "system" {
    pub fn DrawTextA(
        hdc: HDC,
        lpchText: LPCSTR,
        cchText: c_int,
        lprc: LPRECT,
        format: UINT,
    ) -> c_int;
    pub fn DrawTextW(
        hdc: HDC,
        lpchText: LPCWSTR,
        cchText: c_int,
        lprc: LPRECT,
        format: UINT,
    ) -> c_int;
    pub fn DrawTextExA(
        hdc: HDC,
        lpchText: LPCSTR,
        cchText: c_int,
        lprc: LPRECT,
        format: UINT,
        lpdtp: LPDRAWTEXTPARAMS,
    ) -> c_int;
    pub fn DrawTextExW(
        hdc: HDC,
        lpchText: LPCWSTR,
        cchText: c_int,
        lprc: LPRECT,
        format: UINT,
        lpdtp: LPDRAWTEXTPARAMS,
    ) -> c_int;
    pub fn GrayStringA(
        hDC: HDC,
        hBrush: HBRUSH,
        lpOutputFunc: GRAYSTRINGPROC,
        lpData: LPARAM,
        nCount: c_int,
        X: c_int,
        Y: c_int,
        nWidth: c_int,
        nHeight: c_int,
    ) -> BOOL;
    pub fn GrayStringW(
        hDC: HDC,
        hBrush: HBRUSH,
        lpOutputFunc: GRAYSTRINGPROC,
        lpData: LPARAM,
        nCount: c_int,
        X: c_int,
        Y: c_int,
        nWidth: c_int,
        nHeight: c_int,
    ) -> BOOL;
}
pub const DST_COMPLEX: UINT = 0x0000;
pub const DST_TEXT: UINT = 0x0001;
pub const DST_PREFIXTEXT: UINT = 0x0002;
pub const DST_ICON: UINT = 0x0003;
pub const DST_BITMAP: UINT = 0x0004;
pub const DSS_NORMAL: UINT = 0x0000;
pub const DSS_UNION: UINT = 0x0010;
pub const DSS_DISABLED: UINT = 0x0020;
pub const DSS_MONO: UINT = 0x0080;
pub const DSS_HIDEPREFIX: UINT = 0x0200;
pub const DSS_PREFIXONLY: UINT = 0x0400;
pub const DSS_RIGHT: UINT = 0x8000;
extern "system" {
    pub fn DrawStateA(
        hdc: HDC,
        hbrFore: HBRUSH,
        qfnCallBack: DRAWSTATEPROC,
        lData: LPARAM,
        wData: WPARAM,
        x: c_int,
        y: c_int,
        cx: c_int,
        cy: c_int,
        uFlags: UINT,
    ) -> BOOL;
    pub fn DrawStateW(
        hdc: HDC,
        hbrFore: HBRUSH,
        qfnCallBack: DRAWSTATEPROC,
        lData: LPARAM,
        wData: WPARAM,
        x: c_int,
        y: c_int,
        cx: c_int,
        cy: c_int,
        uFlags: UINT,
    ) -> BOOL;
    pub fn TabbedTextOutA(
        hdc: HDC,
        x: c_int,
        y: c_int,
        lpString: LPCSTR,
        chCount: c_int,
        nTabPositions: c_int,
        lpnTabStopPositions: *const INT,
        nTabOrigin: c_int,
    ) -> LONG;
    pub fn TabbedTextOutW(
        hdc: HDC,
        x: c_int,
        y: c_int,
        lpString: LPCWSTR,
        chCount: c_int,
        nTabPositions: c_int,
        lpnTabStopPositions: *const INT,
        nTabOrigin: c_int,
    ) -> LONG;
    pub fn GetTabbedTextExtentA(
        hdc: HDC,
        lpString: LPCSTR,
        chCount: c_int,
        nTabPositions: c_int,
        lpnTabStopPositions: *const INT,
    ) -> DWORD;
    pub fn GetTabbedTextExtentW(
        hdc: HDC,
        lpString: LPCWSTR,
        chCount: c_int,
        nTabPositions: c_int,
        lpnTabStopPositions: *const INT,
    ) -> DWORD;
    pub fn UpdateWindow(
        hWnd: HWND,
    ) -> BOOL;
    pub fn SetActiveWindow(
        hWnd: HWND,
    ) -> HWND;
    pub fn GetForegroundWindow() -> HWND;
    pub fn PaintDesktop(
        hdc: HDC,
    ) -> BOOL;
    pub fn SwitchToThisWindow(
        hwnd: HWND,
        fUnknown: BOOL,
    );
    pub fn SetForegroundWindow(
        hWnd: HWND,
    ) -> BOOL;
    pub fn AllowSetForegroundWindow(
        dwProcessId: DWORD,
    ) -> BOOL;
}
pub const ASFW_ANY: DWORD = -1i32 as u32;
extern "system" {
    pub fn LockSetForegroundWindow(
        uLockCode: UINT,
    ) -> BOOL;
}
pub const LSFW_LOCK: UINT = 1;
pub const LSFW_UNLOCK: UINT = 2;
extern "system" {
    pub fn WindowFromDC(
        hDC: HDC,
    ) -> HWND;
    pub fn GetDC(
        hWnd: HWND,
    ) -> HDC;
    pub fn GetDCEx(
        hWnd: HWND,
        hrgnClip: HRGN,
        flags: DWORD,
    ) -> HDC;
}
pub const DCX_WINDOW: DWORD = 0x00000001;
pub const DCX_CACHE: DWORD = 0x00000002;
pub const DCX_NORESETATTRS: DWORD = 0x00000004;
pub const DCX_CLIPCHILDREN: DWORD = 0x00000008;
pub const DCX_CLIPSIBLINGS: DWORD = 0x00000010;
pub const DCX_PARENTCLIP: DWORD = 0x00000020;
pub const DCX_EXCLUDERGN: DWORD = 0x00000040;
pub const DCX_INTERSECTRGN: DWORD = 0x00000080;
pub const DCX_EXCLUDEUPDATE: DWORD = 0x00000100;
pub const DCX_INTERSECTUPDATE: DWORD = 0x00000200;
pub const DCX_LOCKWINDOWUPDATE: DWORD = 0x00000400;
pub const DCX_VALIDATE: DWORD = 0x00200000;
extern "system" {
    pub fn GetWindowDC(
        hWnd: HWND,
    ) -> HDC;
    pub fn ReleaseDC(
        hWnd: HWND,
        hDC: HDC,
    ) -> c_int;
    pub fn BeginPaint(
        hwnd: HWND,
        lpPaint: LPPAINTSTRUCT,
    ) -> HDC;
    pub fn EndPaint(
        hWnd: HWND,
        lpPaint: *const PAINTSTRUCT,
    ) -> BOOL;
    pub fn GetUpdateRect(
        hWnd: HWND,
        lpRect: LPRECT,
        bErase: BOOL,
    ) -> BOOL;
    pub fn GetUpdateRgn(
        hWnd: HWND,
        hRgn: HRGN,
        bErase: BOOL,
    ) -> c_int;
    pub fn SetWindowRgn(
        hWnd: HWND,
        hRgn: HRGN,
        bRedraw: BOOL,
    ) -> c_int;
    pub fn GetWindowRgn(
        hWnd: HWND,
        hRgn: HRGN,
    ) -> c_int;
    pub fn GetWindowRgnBox(
        hWnd: HWND,
        lprc: LPRECT,
    ) -> c_int;
    pub fn ExcludeUpdateRgn(
        hDC: HDC,
        hWnd: HWND,
    ) -> c_int;
    pub fn InvalidateRect(
        hWnd: HWND,
        lpRect: *const RECT,
        bErase: BOOL,
    ) -> BOOL;
    pub fn ValidateRect(
        hWnd: HWND,
        lpRect: *const RECT,
    ) -> BOOL;
    pub fn InvalidateRgn(
        hWnd: HWND,
        hRgn: HRGN,
        bErase: BOOL,
    ) -> BOOL;
    pub fn ValidateRgn(
        hWnd: HWND,
        hRgn: HRGN,
    ) -> BOOL;
    pub fn RedrawWindow(
        hwnd: HWND,
        lprcUpdate: *const RECT,
        hrgnUpdate: HRGN,
        flags: UINT,
    ) -> BOOL;
}
pub const RDW_INVALIDATE: UINT = 0x0001;
pub const RDW_INTERNALPAINT: UINT = 0x0002;
pub const RDW_ERASE: UINT = 0x0004;
pub const RDW_VALIDATE: UINT = 0x0008;
pub const RDW_NOINTERNALPAINT: UINT = 0x0010;
pub const RDW_NOERASE: UINT = 0x0020;
pub const RDW_NOCHILDREN: UINT = 0x0040;
pub const RDW_ALLCHILDREN: UINT = 0x0080;
pub const RDW_UPDATENOW: UINT = 0x0100;
pub const RDW_ERASENOW: UINT = 0x0200;
pub const RDW_FRAME: UINT = 0x0400;
pub const RDW_NOFRAME: UINT = 0x0800;
extern "system" {
    pub fn LockWindowUpdate(
        hWndLock: HWND,
    ) -> BOOL;
    pub fn ScrollWindow(
        hWnd: HWND,
        xAmount: c_int,
        yAmount: c_int,
        lpRect: *const RECT,
        lpClipRect: *const RECT,
    ) -> BOOL;
    pub fn ScrollDC(
        hDC: HDC,
        dx: c_int,
        dy: c_int,
        lprcScroll: *const RECT,
        lprcClip: *const RECT,
        hrgnUpdate: HRGN,
        lprcUpdate: LPRECT,
    ) -> BOOL;
    pub fn ScrollWindowEx(
        hWnd: HWND,
        dx: c_int,
        dy: c_int,
        prcScroll: *const RECT,
        prcClip: *const RECT,
        hrgnUpdate: HRGN,
        prcUpdate: LPRECT,
        flags: UINT,
    ) -> c_int;
}
pub const SW_SCROLLCHILDREN: UINT = 0x0001;
pub const SW_INVALIDATE: UINT = 0x0002;
pub const SW_ERASE: UINT = 0x0004;
pub const SW_SMOOTHSCROLL: UINT = 0x0010;
extern "system" {
    pub fn SetScrollPos(
        hWnd: HWND,
        nBar: c_int,
        nPos: c_int,
        bRedraw: BOOL,
    ) -> c_int;
    pub fn GetScrollPos(
        hWnd: HWND,
        nBar: c_int,
    ) -> c_int;
    pub fn SetScrollRange(
        hWnd: HWND,
        nBar: c_int,
        nMinPos: c_int,
        nMaxPos: c_int,
        bRedraw: BOOL,
    ) -> BOOL;
    pub fn GetScrollRange(
        hWnd: HWND,
        nBar: c_int,
        lpMinPos: LPINT,
        lpMaxPos: LPINT,
    ) -> BOOL;
    pub fn ShowScrollBar(
        hWnd: HWND,
        wBar: c_int,
        bShow: BOOL,
    ) -> BOOL;
    pub fn EnableScrollBar(
        hWnd: HWND,
        wSBflags: UINT,
        wArrows: UINT,
    ) -> BOOL;
}
pub const ESB_ENABLE_BOTH: UINT = 0x0000;
pub const ESB_DISABLE_BOTH: UINT = 0x0003;
pub const ESB_DISABLE_LEFT: UINT = 0x0001;
pub const ESB_DISABLE_RIGHT: UINT = 0x0002;
pub const ESB_DISABLE_UP: UINT = 0x0001;
pub const ESB_DISABLE_DOWN: UINT = 0x0002;
pub const ESB_DISABLE_LTUP: UINT = ESB_DISABLE_LEFT;
pub const ESB_DISABLE_RTDN: UINT = ESB_DISABLE_RIGHT;
extern "system" {
    pub fn SetPropA(
        hWnd: HWND,
        lpString: LPCSTR,
        hData: HANDLE,
    ) -> BOOL;
    pub fn SetPropW(
        hWnd: HWND,
        lpString: LPCWSTR,
        hData: HANDLE,
    ) -> BOOL;
    pub fn GetPropA(
        hwnd: HWND,
        lpString: LPCSTR,
    ) -> HANDLE;
    pub fn GetPropW(
        hwnd: HWND,
        lpString: LPCWSTR,
    ) -> HANDLE;
    pub fn RemovePropA(
        hWnd: HWND,
        lpStr: LPCSTR,
    ) -> HANDLE;
    pub fn RemovePropW(
        hWnd: HWND,
        lpStr: LPCWSTR,
    ) -> HANDLE;
    pub fn EnumPropsExA(
        hWnd: HWND,
        lpEnumFunc: PROPENUMPROCA,
        lParam: LPARAM,
    ) -> c_int;
    pub fn EnumPropsExW(
        hWnd: HWND,
        lpEnumFunc: PROPENUMPROCW,
        lParam: LPARAM,
    ) -> c_int;
    pub fn EnumPropsA(
        hWnd: HWND,
        lpEnumFunc: PROPENUMPROCA,
    ) -> c_int;
    pub fn EnumPropsW(
        hWnd: HWND,
        lpEnumFunc: PROPENUMPROCW,
    ) -> c_int;
    pub fn SetWindowTextA(
        hWnd: HWND,
        lpString: LPCSTR,
    ) -> BOOL;
    pub fn SetWindowTextW(
        hWnd: HWND,
        lpString: LPCWSTR,
    ) -> BOOL;
    pub fn GetWindowTextA(
        hWnd: HWND,
        lpString: LPSTR,
        nMaxCount: c_int,
    ) -> c_int;
    pub fn GetWindowTextW(
        hWnd: HWND,
        lpString: LPWSTR,
        nMaxCount: c_int,
    ) -> c_int;
    pub fn GetWindowTextLengthA(
        hWnd: HWND,
    ) -> c_int;
    pub fn GetWindowTextLengthW(
        hWnd: HWND,
    ) -> c_int;
    pub fn GetClientRect(
        hWnd: HWND,
        lpRect: LPRECT,
    ) -> BOOL;
    pub fn GetWindowRect(
        hWnd: HWND,
        lpRect: LPRECT,
    ) -> BOOL;
    pub fn AdjustWindowRect(
        lpRect: LPRECT,
        dwStyle: DWORD,
        bMenu: BOOL,
    ) -> BOOL;
    pub fn AdjustWindowRectEx(
        lpRect: LPRECT,
        dwStyle: DWORD,
        bMenu: BOOL,
        dwExStyle: DWORD,
    ) -> BOOL;
    pub fn AdjustWindowRectExForDpi(
        lpRect: LPRECT,
        dwStyle: DWORD,
        bMenu: BOOL,
        dwExStyle: DWORD,
        dpi: UINT,
    ) -> BOOL;
}
pub const HELPINFO_WINDOW: UINT = 0x0001;
pub const HELPINFO_MENUITEM: UINT = 0x0002;
STRUCT!{struct HELPINFO {
    cbSize: UINT,
    iContextType: c_int,
    iCtrlId: c_int,
    hItemHandle: HANDLE,
    dwContextId: DWORD,
    MousePos: POINT,
}}
pub type LPHELPINFO = *mut HELPINFO;
extern "system" {
    pub fn SetWindowContextHelpId(
        _: HWND,
        _: DWORD,
    ) -> BOOL;
    pub fn GetWindowContextHelpId(
        _: HWND,
    ) -> DWORD;
    pub fn SetMenuContextHelpId(
        _: HMENU,
        _: DWORD,
    ) -> BOOL;
    pub fn GetMenuContextHelpId(
        _: HMENU,
    ) -> DWORD;
}
pub const MB_OK: UINT = 0x00000000;
pub const MB_OKCANCEL: UINT = 0x00000001;
pub const MB_ABORTRETRYIGNORE: UINT = 0x00000002;
pub const MB_YESNOCANCEL: UINT = 0x00000003;
pub const MB_YESNO: UINT = 0x00000004;
pub const MB_RETRYCANCEL: UINT = 0x00000005;
pub const MB_CANCELTRYCONTINUE: UINT = 0x00000006;
pub const MB_ICONHAND: UINT = 0x00000010;
pub const MB_ICONQUESTION: UINT = 0x00000020;
pub const MB_ICONEXCLAMATION: UINT = 0x00000030;
pub const MB_ICONASTERISK: UINT = 0x00000040;
pub const MB_USERICON: UINT = 0x00000080;
pub const MB_ICONWARNING: UINT = MB_ICONEXCLAMATION;
pub const MB_ICONERROR: UINT = MB_ICONHAND;
pub const MB_ICONINFORMATION: UINT = MB_ICONASTERISK;
pub const MB_ICONSTOP: UINT = MB_ICONHAND;
pub const MB_DEFBUTTON1: UINT = 0x00000000;
pub const MB_DEFBUTTON2: UINT = 0x00000100;
pub const MB_DEFBUTTON3: UINT = 0x00000200;
pub const MB_DEFBUTTON4: UINT = 0x00000300;
pub const MB_APPLMODAL: UINT = 0x00000000;
pub const MB_SYSTEMMODAL: UINT = 0x00001000;
pub const MB_TASKMODAL: UINT = 0x00002000;
pub const MB_HELP: UINT = 0x00004000;
pub const MB_NOFOCUS: UINT = 0x00008000;
pub const MB_SETFOREGROUND: UINT = 0x00010000;
pub const MB_DEFAULT_DESKTOP_ONLY: UINT = 0x00020000;
pub const MB_TOPMOST: UINT = 0x00040000;
pub const MB_RIGHT: UINT = 0x00080000;
pub const MB_RTLREADING: UINT = 0x00100000;
pub const MB_SERVICE_NOTIFICATION: UINT = 0x00200000;
pub const MB_SERVICE_NOTIFICATION_NT3X: UINT = 0x00040000;
pub const MB_TYPEMASK: UINT = 0x0000000F;
pub const MB_ICONMASK: UINT = 0x000000F0;
pub const MB_DEFMASK: UINT = 0x00000F00;
pub const MB_MODEMASK: UINT = 0x00003000;
pub const MB_MISCMASK: UINT = 0x0000C000;
extern "system" {
    pub fn MessageBoxA(
        hWnd: HWND,
        lpText: LPCSTR,
        lpCaption: LPCSTR,
        uType: UINT,
    ) -> c_int;
    pub fn MessageBoxW(
        hWnd: HWND,
        lpText: LPCWSTR,
        lpCaption: LPCWSTR,
        uType: UINT,
    ) -> c_int;
    pub fn MessageBoxExA(
        hWnd: HWND,
        lpText: LPCSTR,
        lpCaption: LPCSTR,
        uType: UINT,
        wLanguageId: WORD,
    ) -> c_int;
    pub fn MessageBoxExW(
        hWnd: HWND,
        lpText: LPCWSTR,
        lpCaption: LPCWSTR,
        uType: UINT,
        wLanguageId: WORD,
    ) -> c_int;
}
FN!{stdcall MSGBOXCALLBACK(
    LPHELPINFO,
) -> ()}
STRUCT!{struct MSGBOXPARAMSA {
    cbSize: UINT,
    hwndOwner: HWND,
    hInstance: HINSTANCE,
    lpszText: LPCSTR,
    lpszCaption: LPCSTR,
    dwStyle: DWORD,
    lpszIcon: LPCSTR,
    dwContextHelpId: DWORD_PTR,
    lpfnMsgBoxCallback: MSGBOXCALLBACK,
    dwLanguageId: DWORD,
}}
pub type PMSGBOXPARAMSA = *mut MSGBOXPARAMSA;
pub type LPMSGBOXPARAMSA = *mut MSGBOXPARAMSA;
STRUCT!{struct MSGBOXPARAMSW {
    cbSize: UINT,
    hwndOwner: HWND,
    hInstance: HINSTANCE,
    lpszText: LPCWSTR,
    lpszCaption: LPCWSTR,
    dwStyle: DWORD,
    lpszIcon: LPCWSTR,
    dwContextHelpId: DWORD_PTR,
    lpfnMsgBoxCallback: MSGBOXCALLBACK,
    dwLanguageId: DWORD,
}}
pub type PMSGBOXPARAMSW = *mut MSGBOXPARAMSW;
pub type LPMSGBOXPARAMSW = *mut MSGBOXPARAMSW;
extern "system" {
    pub fn MessageBoxIndirectA(
        lpmbp: *const MSGBOXPARAMSA,
    ) -> c_int;
    pub fn MessageBoxIndirectW(
        lpmbp: *const MSGBOXPARAMSW,
    ) -> c_int;
    pub fn MessageBeep(
        uType: UINT,
    ) -> BOOL;
    pub fn ShowCursor(
        bShow: BOOL,
    ) -> c_int;
    pub fn SetCursorPos(
        X: c_int,
        Y: c_int,
    ) -> BOOL;
    pub fn SetPhysicalCursorPos(
        X: c_int,
        Y: c_int,
    ) -> BOOL;
    pub fn SetCursor(
        hCursor: HCURSOR,
    ) -> HCURSOR;
    pub fn GetCursorPos(
        lpPoint: LPPOINT,
    ) -> BOOL;
    pub fn GetPhysicalCursorPos(
        lpPoint: LPPOINT,
    ) -> BOOL;
    pub fn GetClipCursor(
        lpRect: LPRECT,
    ) -> BOOL;
    pub fn GetCursor() -> HCURSOR;
    pub fn CreateCaret(
        hWnd: HWND,
        hBitmap: HBITMAP,
        nWidth: c_int,
        nHeight: c_int,
    ) -> BOOL;
    pub fn GetCaretBlinkTime() -> UINT;
    pub fn SetCaretBlinkTime(
        uMSeconds: UINT,
    ) -> BOOL;
    pub fn DestroyCaret() -> BOOL;
    pub fn HideCaret(
        hWnd: HWND,
    ) -> BOOL;
    pub fn ShowCaret(
        hWnd: HWND,
    ) -> BOOL;
    pub fn SetCaretPos(
        X: c_int,
        Y: c_int,
    ) -> BOOL;
    pub fn GetCaretPos(
        lpPoint: LPPOINT,
    ) -> BOOL;
    pub fn ClientToScreen(
        hWnd: HWND,
        lpPoint: LPPOINT,
    ) -> BOOL;
    pub fn ScreenToClient(
        hWnd: HWND,
        lpPoint: LPPOINT,
    ) -> BOOL;
    pub fn LogicalToPhysicalPoint(
        hWnd: HWND,
        lpPoint: LPPOINT,
    ) -> BOOL;
    pub fn PhysicalToLogicalPoint(
        hWnd: HWND,
        lpPoint: LPPOINT,
    ) -> BOOL;
    pub fn LogicalToPhysicalPointForPerMonitorDPI(
        hWnd: HWND,
        lpPoint: LPPOINT,
    ) -> BOOL;
    pub fn PhysicalToLogicalPointForPerMonitorDPI(
        hWnd: HWND,
        lpPoint: LPPOINT,
    ) -> BOOL;
    pub fn MapWindowPoints(
        hWndFrom: HWND,
        hWndTo: HWND,
        lpPoints: LPPOINT,
        cPoints: UINT,
    ) -> c_int;
    pub fn WindowFromPoint(
        Point: POINT,
    ) -> HWND;
    pub fn WindowFromPhysicalPoint(
        Point: POINT,
    ) -> HWND;
    pub fn ChildWindowFromPoint(
        hWndParent: HWND,
        point: POINT,
    ) -> HWND;
    pub fn ClipCursor(
        lpRect: *const RECT,
    ) -> BOOL;
}
pub const CWP_ALL: UINT = 0x0000;
pub const CWP_SKIPINVISIBLE: UINT = 0x0001;
pub const CWP_SKIPDISABLED: UINT = 0x0002;
pub const CWP_SKIPTRANSPARENT: UINT = 0x0004;
extern "system" {
    pub fn ChildWindowFromPointEx(
        hwnd: HWND,
        pt: POINT,
        flags: UINT,
    ) -> HWND;
}
pub const CTLCOLOR_MSGBOX: c_int = 0;
pub const CTLCOLOR_EDIT: c_int = 1;
pub const CTLCOLOR_LISTBOX: c_int = 2;
pub const CTLCOLOR_BTN: c_int = 3;
pub const CTLCOLOR_DLG: c_int = 4;
pub const CTLCOLOR_SCROLLBAR: c_int = 5;
pub const CTLCOLOR_STATIC: c_int = 6;
pub const CTLCOLOR_MAX: c_int = 7;
pub const COLOR_SCROLLBAR: c_int = 0;
pub const COLOR_BACKGROUND: c_int = 1;
pub const COLOR_ACTIVECAPTION: c_int = 2;
pub const COLOR_INACTIVECAPTION: c_int = 3;
pub const COLOR_MENU: c_int = 4;
pub const COLOR_WINDOW: c_int = 5;
pub const COLOR_WINDOWFRAME: c_int = 6;
pub const COLOR_MENUTEXT: c_int = 7;
pub const COLOR_WINDOWTEXT: c_int = 8;
pub const COLOR_CAPTIONTEXT: c_int = 9;
pub const COLOR_ACTIVEBORDER: c_int = 10;
pub const COLOR_INACTIVEBORDER: c_int = 11;
pub const COLOR_APPWORKSPACE: c_int = 12;
pub const COLOR_HIGHLIGHT: c_int = 13;
pub const COLOR_HIGHLIGHTTEXT: c_int = 14;
pub const COLOR_BTNFACE: c_int = 15;
pub const COLOR_BTNSHADOW: c_int = 16;
pub const COLOR_GRAYTEXT: c_int = 17;
pub const COLOR_BTNTEXT: c_int = 18;
pub const COLOR_INACTIVECAPTIONTEXT: c_int = 19;
pub const COLOR_BTNHIGHLIGHT: c_int = 20;
pub const COLOR_3DDKSHADOW: c_int = 21;
pub const COLOR_3DLIGHT: c_int = 22;
pub const COLOR_INFOTEXT: c_int = 23;
pub const COLOR_INFOBK: c_int = 24;
pub const COLOR_HOTLIGHT: c_int = 26;
pub const COLOR_GRADIENTACTIVECAPTION: c_int = 27;
pub const COLOR_GRADIENTINACTIVECAPTION: c_int = 28;
pub const COLOR_MENUHILIGHT: c_int = 29;
pub const COLOR_MENUBAR: c_int = 30;
pub const COLOR_DESKTOP: c_int = COLOR_BACKGROUND;
pub const COLOR_3DFACE: c_int = COLOR_BTNFACE;
pub const COLOR_3DSHADOW: c_int = COLOR_BTNSHADOW;
pub const COLOR_3DHIGHLIGHT: c_int = COLOR_BTNHIGHLIGHT;
pub const COLOR_3DHILIGHT: c_int = COLOR_BTNHIGHLIGHT;
pub const COLOR_BTNHILIGHT: c_int = COLOR_BTNHIGHLIGHT;
extern "system" {
    pub fn GetSysColor(
        nIndex: c_int,
    ) -> DWORD;
    pub fn GetSysColorBrush(
        nIndex: c_int,
    ) -> HBRUSH;
    pub fn SetSysColors(
        cElements: c_int,
        lpaElements: *const INT,
        lpaRgbValues: *const COLORREF,
    ) -> BOOL;
    pub fn DrawFocusRect(
        hDC: HDC,
        lprc: *const RECT,
    ) -> BOOL;
    pub fn FillRect(
        hDC: HDC,
        lprc: *const RECT,
        hbr: HBRUSH,
    ) -> c_int;
    pub fn FrameRect(
        hDC: HDC,
        lprc: *const RECT,
        hbr: HBRUSH,
    ) -> c_int;
    pub fn InvertRect(
        hDC: HDC,
        lprc: *const RECT,
    ) -> BOOL;
    pub fn SetRect(
        lprc: LPRECT,
        xLeft: c_int,
        yTop: c_int,
        xRight: c_int,
        yBottom: c_int,
    ) -> BOOL;
    pub fn SetRectEmpty(
        lprc: LPRECT,
    ) -> BOOL;
    pub fn CopyRect(
        lprcDst: LPRECT,
        lprcSrc: *const RECT,
    ) -> BOOL;
    pub fn InflateRect(
        lprc: LPRECT,
        dx: c_int,
        dy: c_int,
    ) -> BOOL;
    pub fn IntersectRect(
        lprcDst: LPRECT,
        lprcSrc1: *const RECT,
        lprcSrc2: *const RECT,
    ) -> BOOL;
    pub fn UnionRect(
        lprcDst: LPRECT,
        lprcSrc1: *const RECT,
        lprcSrc2: *const RECT,
    ) -> BOOL;
    pub fn SubtractRect(
        lprcDst: LPRECT,
        lprcSrc1: *const RECT,
        lprcSrc2: *const RECT,
    ) -> BOOL;
    pub fn OffsetRect(
        lprc: LPRECT,
        dx: c_int,
        dy: c_int,
    ) -> BOOL;
    pub fn IsRectEmpty(
        lprc: *const RECT,
    ) -> BOOL;
    pub fn EqualRect(
        lprc1: *const RECT,
        lprc2: *const RECT,
    ) -> BOOL;
    pub fn PtInRect(
        lprc: *const RECT,
        pt: POINT,
    ) -> BOOL;
    pub fn GetWindowWord(
        hWnd: HWND,
        nIndex: c_int,
    ) -> WORD;
    pub fn SetWindowWord(
        hwnd: HWND,
        nIndex: c_int,
        wNewWord: WORD,
    ) -> WORD;
    pub fn GetWindowLongA(
        hWnd: HWND,
        nIndex: c_int,
    ) -> LONG;
    pub fn GetWindowLongW(
        hWnd: HWND,
        nIndex: c_int,
    ) -> LONG;
    pub fn SetWindowLongA(
        hWnd: HWND,
        nIndex: c_int,
        dwNewLong: LONG,
    ) -> LONG;
    pub fn SetWindowLongW(
        hWnd: HWND,
        nIndex: c_int,
        dwNewLong: LONG,
    ) -> LONG;
    #[cfg(target_pointer_width = "64")]
    pub fn GetWindowLongPtrA(
        hWnd: HWND,
        nIndex: c_int,
    ) -> LONG_PTR;
    #[cfg(target_pointer_width = "64")]
    pub fn GetWindowLongPtrW(
        hWnd: HWND,
        nIndex: c_int,
    ) -> LONG_PTR;
    #[cfg(target_pointer_width = "64")]
    pub fn SetWindowLongPtrA(
        hWnd: HWND,
        nIndex: c_int,
        dwNewLong: LONG_PTR,
    ) -> LONG_PTR;
    #[cfg(target_pointer_width = "64")]
    pub fn SetWindowLongPtrW(
        hWnd: HWND,
        nIndex: c_int,
        dwNewLong: LONG_PTR,
    ) -> LONG_PTR;
}
#[cfg(target_pointer_width = "32")]
pub use self::GetWindowLongA as GetWindowLongPtrA;
#[cfg(target_pointer_width = "32")]
pub use self::GetWindowLongW as GetWindowLongPtrW;
#[cfg(target_pointer_width = "32")]
pub use self::SetWindowLongA as SetWindowLongPtrA;
#[cfg(target_pointer_width = "32")]
pub use self::SetWindowLongW as SetWindowLongPtrW;
extern "system" {
    pub fn GetClassWord(
        hWnd: HWND,
        nIndex: c_int,
    ) -> WORD;
    pub fn SetClassWord(
        hWnd: HWND,
        nIndex: c_int,
        wNewWord: WORD,
    ) -> WORD;
    pub fn GetClassLongA(
        hWnd: HWND,
        nIndex: c_int,
    ) -> DWORD;
    pub fn GetClassLongW(
        hWnd: HWND,
        nIndex: c_int,
    ) -> DWORD;
    pub fn SetClassLongA(
        hWnd: HWND,
        nIndex: c_int,
        dwNewLong: LONG,
    ) -> DWORD;
    pub fn SetClassLongW(
        hWnd: HWND,
        nIndex: c_int,
        dwNewLong: LONG,
    ) -> DWORD;
    #[cfg(target_pointer_width = "64")]
    pub fn GetClassLongPtrA(
        hWnd: HWND,
        nIndex: c_int,
    ) -> ULONG_PTR;
    #[cfg(target_pointer_width = "64")]
    pub fn GetClassLongPtrW(
        hWnd: HWND,
        nIndex: c_int,
    ) -> ULONG_PTR;
    #[cfg(target_pointer_width = "64")]
    pub fn SetClassLongPtrA(
        hWnd: HWND,
        nIndex: c_int,
        dwNewLong: LONG_PTR,
    ) -> ULONG_PTR;
    #[cfg(target_pointer_width = "64")]
    pub fn SetClassLongPtrW(
        hWnd: HWND,
        nIndex: c_int,
        dwNewLong: LONG_PTR,
    ) -> ULONG_PTR;
}
#[cfg(target_pointer_width = "32")]
pub use self::GetClassLongA as GetClassLongPtrA;
#[cfg(target_pointer_width = "32")]
pub use self::GetClassLongW as GetClassLongPtrW;
#[cfg(target_pointer_width = "32")]
pub use self::SetClassLongA as SetClassLongPtrA;
#[cfg(target_pointer_width = "32")]
pub use self::SetClassLongW as SetClassLongPtrW;
extern "system" {
    pub fn GetProcessDefaultLayout(
        pdwDefaultLayout: *mut DWORD,
    ) -> BOOL;
    pub fn SetProcessDefaultLayout(
        dwDefaultLayout: DWORD,
    ) -> BOOL;
    pub fn GetDesktopWindow() -> HWND;
    pub fn GetParent(
        hWnd: HWND,
    ) -> HWND;
    pub fn SetParent(
        hWndChild: HWND,
        hWndNewParent: HWND,
    ) -> HWND;
    pub fn EnumChildWindows(
        hWndParent: HWND,
        lpEnumFunc: WNDENUMPROC,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn FindWindowA(
        lpClassName: LPCSTR,
        lpWindowName: LPCSTR,
    ) -> HWND;
    pub fn FindWindowW(
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
    ) -> HWND;
    pub fn FindWindowExA(
        hWndParent: HWND,
        hWndChildAfter: HWND,
        lpszClass: LPCSTR,
        lpszWindow: LPCSTR,
    ) -> HWND;
    pub fn FindWindowExW(
        hWndParent: HWND,
        hWndChildAfter: HWND,
        lpszClass: LPCWSTR,
        lpszWindow: LPCWSTR,
    ) -> HWND;
    pub fn GetShellWindow() -> HWND;
    pub fn RegisterShellHookWindow(
        hwnd: HWND,
    ) -> BOOL;
    pub fn DeregisterShellHookWindow(
        hwnd: HWND,
    ) -> BOOL;
    pub fn EnumWindows(
        lpEnumFunc: WNDENUMPROC,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn EnumThreadWindows(
        dwThreadId: DWORD,
        lpfn: WNDENUMPROC,
        lParam: LPARAM,
    ) -> BOOL;
}
// EnumTaskWindows
extern "system" {
    pub fn GetClassNameA(
        hWnd: HWND,
        lpClassName: LPCSTR,
        nMaxCount: c_int,
    ) -> c_int;
    pub fn GetClassNameW(
        hWnd: HWND,
        lpClassName: LPCWSTR,
        nMaxCount: c_int,
    ) -> c_int;
    pub fn GetTopWindow(
        hWnd: HWND,
    ) -> HWND;
}
// GetNextWindow
// GetSysModalWindow
// SetSysModalWindow
extern "system" {
    pub fn GetWindowThreadProcessId(
        hWnd: HWND,
        lpdwProcessId: LPDWORD,
    ) -> DWORD;
    pub fn IsGUIThread(
        bConvert: BOOL,
    ) -> BOOL;
    pub fn GetLastActivePopup(
        hWnd: HWND,
    ) -> HWND;
}
pub const GW_HWNDFIRST: UINT = 0;
pub const GW_HWNDLAST: UINT = 1;
pub const GW_HWNDNEXT: UINT = 2;
pub const GW_HWNDPREV: UINT = 3;
pub const GW_OWNER: UINT = 4;
pub const GW_CHILD: UINT = 5;
pub const GW_ENABLEDPOPUP: UINT = 6;
pub const GW_MAX: UINT = 6;
extern "system" {
    pub fn GetWindow(
        hWnd: HWND,
        uCmd: UINT,
    ) -> HWND;
    pub fn SetWindowsHookA(
        nFilterType: c_int,
        pfnFilterProc: HOOKPROC,
    ) -> HHOOK;
    pub fn SetWindowsHookW(
        nFilterType: c_int,
        pfnFilterProc: HOOKPROC,
    ) -> HHOOK;
    pub fn UnhookWindowsHook(
        nFilterType: c_int,
        pfnFilterProc: HOOKPROC,
    ) -> BOOL;
    pub fn SetWindowsHookExA(
        idHook: c_int,
        lpfn: HOOKPROC,
        hmod: HINSTANCE,
        dwThreadId: DWORD,
    ) -> HHOOK;
    pub fn SetWindowsHookExW(
        idHook: c_int,
        lpfn: HOOKPROC,
        hmod: HINSTANCE,
        dwThreadId: DWORD,
    ) -> HHOOK;
    pub fn UnhookWindowsHookEx(
        hhk: HHOOK,
    ) -> BOOL;
    pub fn CallNextHookEx(
        hhk: HHOOK,
        nCode: c_int,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
}
// DefHookProc
pub const MF_INSERT: UINT = 0x00000000;
pub const MF_CHANGE: UINT = 0x00000080;
pub const MF_APPEND: UINT = 0x00000100;
pub const MF_DELETE: UINT = 0x00000200;
pub const MF_REMOVE: UINT = 0x00001000;
pub const MF_BYCOMMAND: UINT = 0x00000000;
pub const MF_BYPOSITION: UINT = 0x00000400;
pub const MF_SEPARATOR: UINT = 0x00000800;
pub const MF_ENABLED: UINT = 0x00000000;
pub const MF_GRAYED: UINT = 0x00000001;
pub const MF_DISABLED: UINT = 0x00000002;
pub const MF_UNCHECKED: UINT = 0x00000000;
pub const MF_CHECKED: UINT = 0x00000008;
pub const MF_USECHECKBITMAPS: UINT = 0x00000200;
pub const MF_STRING: UINT = 0x00000000;
pub const MF_BITMAP: UINT = 0x00000004;
pub const MF_OWNERDRAW: UINT = 0x00000100;
pub const MF_POPUP: UINT = 0x00000010;
pub const MF_MENUBARBREAK: UINT = 0x00000020;
pub const MF_MENUBREAK: UINT = 0x00000040;
pub const MF_UNHILITE: UINT = 0x00000000;
pub const MF_HILITE: UINT = 0x00000080;
pub const MF_DEFAULT: UINT = 0x00001000;
pub const MF_SYSMENU: UINT = 0x00002000;
pub const MF_HELP: UINT = 0x00004000;
pub const MF_RIGHTJUSTIFY: UINT = 0x00004000;
pub const MF_MOUSESELECT: UINT = 0x00008000;
pub const MF_END: UINT = 0x00000080;
pub const MFT_STRING: UINT = MF_STRING;
pub const MFT_BITMAP: UINT = MF_BITMAP;
pub const MFT_MENUBARBREAK: UINT = MF_MENUBARBREAK;
pub const MFT_MENUBREAK: UINT = MF_MENUBREAK;
pub const MFT_OWNERDRAW: UINT = MF_OWNERDRAW;
pub const MFT_RADIOCHECK: UINT = 0x00000200;
pub const MFT_SEPARATOR: UINT = MF_SEPARATOR;
pub const MFT_RIGHTORDER: UINT = 0x00002000;
pub const MFT_RIGHTJUSTIFY: UINT = MF_RIGHTJUSTIFY;
pub const MFS_GRAYED: UINT = 0x00000003;
pub const MFS_DISABLED: UINT = MFS_GRAYED;
pub const MFS_CHECKED: UINT = MF_CHECKED;
pub const MFS_HILITE: UINT = MF_HILITE;
pub const MFS_ENABLED: UINT = MF_ENABLED;
pub const MFS_UNCHECKED: UINT = MF_UNCHECKED;
pub const MFS_UNHILITE: UINT = MF_UNHILITE;
pub const MFS_DEFAULT: UINT = MF_DEFAULT;
extern "system" {
    pub fn CheckMenuRadioItem(
        hMenu: HMENU,
        first: UINT,
        last: UINT,
        check: UINT,
        flags: UINT,
    ) -> BOOL;
}
//10225
pub const SC_SIZE: WPARAM = 0xF000;
pub const SC_MOVE: WPARAM = 0xF010;
pub const SC_MINIMIZE: WPARAM = 0xF020;
pub const SC_MAXIMIZE: WPARAM = 0xF030;
pub const SC_NEXTWINDOW: WPARAM = 0xF040;
pub const SC_PREVWINDOW: WPARAM = 0xF050;
pub const SC_CLOSE: WPARAM = 0xF060;
pub const SC_VSCROLL: WPARAM = 0xF070;
pub const SC_HSCROLL: WPARAM = 0xF080;
pub const SC_MOUSEMENU: WPARAM = 0xF090;
pub const SC_KEYMENU: WPARAM = 0xF100;
pub const SC_ARRANGE: WPARAM = 0xF110;
pub const SC_RESTORE: WPARAM = 0xF120;
pub const SC_TASKLIST: WPARAM = 0xF130;
pub const SC_SCREENSAVE: WPARAM = 0xF140;
pub const SC_HOTKEY: WPARAM = 0xF150;
pub const SC_DEFAULT: WPARAM = 0xF160;
pub const SC_MONITORPOWER: WPARAM = 0xF170;
pub const SC_CONTEXTHELP: WPARAM = 0xF180;
pub const SC_SEPARATOR: WPARAM = 0xF00F;
//10269
extern "system" {
    pub fn LoadBitmapA(
        hInstance: HINSTANCE,
        lpBitmapName: LPCSTR,
    ) -> HBITMAP;
    pub fn LoadBitmapW(
        hInstance: HINSTANCE,
        lpBitmapName: LPCWSTR,
    ) -> HBITMAP;
    pub fn LoadCursorA(
        hInstance: HINSTANCE,
        lpCursorName: LPCSTR,
    ) -> HCURSOR;
    pub fn LoadCursorW(
        hInstance: HINSTANCE,
        lpCursorName: LPCWSTR,
    ) -> HCURSOR;
    pub fn LoadCursorFromFileA(
        lpFileName: LPCSTR,
    ) -> HCURSOR;
    pub fn LoadCursorFromFileW(
        lpFileName: LPCWSTR,
    ) -> HCURSOR;
    pub fn CreateCursor(
        hInst: HINSTANCE,
        xHotSpot: c_int,
        yHotSpot: c_int,
        nWidth: c_int,
        nHeight: c_int,
        pvAndPlane: *const VOID,
        pvXORPlane: *const VOID,
    ) -> HCURSOR;
    pub fn DestroyCursor(
        hCursor: HCURSOR,
    ) -> BOOL;
}
//10355
pub const IDC_ARROW: LPCWSTR = 32512 as LPCWSTR;
pub const IDC_IBEAM: LPCWSTR = 32513 as LPCWSTR;
pub const IDC_WAIT: LPCWSTR = 32514 as LPCWSTR;
pub const IDC_CROSS: LPCWSTR = 32515 as LPCWSTR;
pub const IDC_UPARROW: LPCWSTR = 32516 as LPCWSTR;
pub const IDC_SIZE: LPCWSTR = 32640 as LPCWSTR;
pub const IDC_ICON: LPCWSTR = 32641 as LPCWSTR;
pub const IDC_SIZENWSE: LPCWSTR = 32642 as LPCWSTR;
pub const IDC_SIZENESW: LPCWSTR = 32643 as LPCWSTR;
pub const IDC_SIZEWE: LPCWSTR = 32644 as LPCWSTR;
pub const IDC_SIZENS: LPCWSTR = 32645 as LPCWSTR;
pub const IDC_SIZEALL: LPCWSTR = 32646 as LPCWSTR;
pub const IDC_NO: LPCWSTR = 32648 as LPCWSTR;
pub const IDC_HAND: LPCWSTR = 32649 as LPCWSTR;
pub const IDC_APPSTARTING: LPCWSTR = 32650 as LPCWSTR;
pub const IDC_HELP: LPCWSTR = 32651 as LPCWSTR;
extern "system" {
    pub fn SetSystemCursor(
        hcur: HCURSOR,
        id: DWORD,
    ) -> BOOL;
}
STRUCT!{struct ICONINFO {
    fIcon: BOOL,
    xHotspot: DWORD,
    yHotspot: DWORD,
    hbmMask: HBITMAP,
    hbmColor: HBITMAP,
}}
pub type PICONINFO = *mut ICONINFO;
extern "system" {
    pub fn LoadIconA(
        hInstance: HINSTANCE,
        lpIconName: LPCSTR,
    ) -> HICON;
    pub fn LoadIconW(
        hInstance: HINSTANCE,
        lpIconName: LPCWSTR,
    ) -> HICON;
}
//10449
extern "system" {
    pub fn CreateIcon(
        hInstance: HINSTANCE,
        nWidth: c_int,
        nHeight: c_int,
        cPlanes: BYTE,
        cBitsPixel: BYTE,
        lpbANDbits: *const BYTE,
        lpbXORbits: *const BYTE,
    ) -> HICON;
    pub fn DestroyIcon(
        hIcon: HICON,
    ) -> BOOL;
    pub fn LookupIconIdFromDirectory(
        presbits: PBYTE,
        fIcon: BOOL,
    ) -> c_int;
    pub fn LookupIconIdFromDirectoryEx(
        presbits: PBYTE,
        fIcon: BOOL,
        cxDesired: c_int,
        cyDesired: c_int,
        Flags: UINT,
    ) -> c_int;
    pub fn CreateIconFromResource(
        presbits: PBYTE,
        dwResSize: DWORD,
        fIcon: BOOL,
        dwVer: DWORD,
    ) -> HICON;
    pub fn CreateIconFromResourceEx(
        presbits: PBYTE,
        dwResSize: DWORD,
        fIcon: BOOL,
        dwVer: DWORD,
        cxDesired: c_int,
        cyDesired: c_int,
        Flags: UINT,
    ) -> HICON;
}
//10524
pub const IMAGE_BITMAP: UINT = 0;
pub const IMAGE_ICON: UINT = 1;
pub const IMAGE_CURSOR: UINT = 2;
pub const IMAGE_ENHMETAFILE: UINT = 3;
pub const LR_DEFAULTCOLOR: UINT = 0x00000000;
pub const LR_MONOCHROME: UINT = 0x00000001;
pub const LR_COLOR: UINT = 0x00000002;
pub const LR_COPYRETURNORG: UINT = 0x00000004;
pub const LR_COPYDELETEORG: UINT = 0x00000008;
pub const LR_LOADFROMFILE: UINT = 0x00000010;
pub const LR_LOADTRANSPARENT: UINT = 0x00000020;
pub const LR_DEFAULTSIZE: UINT = 0x00000040;
pub const LR_VGACOLOR: UINT = 0x00000080;
pub const LR_LOADMAP3DCOLORS: UINT = 0x00001000;
pub const LR_CREATEDIBSECTION: UINT = 0x00002000;
pub const LR_COPYFROMRESOURCE: UINT = 0x00004000;
pub const LR_SHARED: UINT = 0x00008000;
extern "system" {
    pub fn LoadImageA(
        hInst: HINSTANCE,
        name: LPCSTR,
        type_: UINT,
        cx: c_int,
        cy: c_int,
        fuLoad: UINT,
    ) -> HANDLE;
    pub fn LoadImageW(
        hInst: HINSTANCE,
        name: LPCWSTR,
        type_: UINT,
        cx: c_int,
        cy: c_int,
        fuLoad: UINT,
    ) -> HANDLE;
    pub fn CopyImage(
        h: HANDLE,
        type_: UINT,
        cx: c_int,
        cy: c_int,
        flags: UINT,
    ) -> HANDLE;
}
//10592
extern "system" {
    pub fn DrawIconEx(
        hdc: HDC,
        xLeft: c_int,
        yTop: c_int,
        hIcon: HICON,
        cxWidth: c_int,
        cyWidth: c_int,
        istepIfAniCur: UINT,
        hbrFlickerFreeDraw: HBRUSH,
        diFlags: UINT,
    ) -> BOOL;
    pub fn CreateIconIndirect(
        piconinfo: PICONINFO,
    ) -> HICON;
    pub fn CopyIcon(
        hIcon: HICON,
    ) -> HICON;
    pub fn GetIconInfo(
        hIcon: HICON,
        piconinfo: PICONINFO,
    ) -> BOOL;
}
//10781
pub const IDI_APPLICATION: LPCWSTR = 32512 as LPCWSTR;
pub const IDI_HAND: LPCWSTR = 32513 as LPCWSTR;
pub const IDI_QUESTION: LPCWSTR = 32514 as LPCWSTR;
pub const IDI_EXCLAMATION: LPCWSTR = 32515 as LPCWSTR;
pub const IDI_ASTERISK: LPCWSTR = 32516 as LPCWSTR;
pub const IDI_WINLOGO: LPCWSTR = 32517 as LPCWSTR;
pub const IDI_SHIELD: LPCWSTR = 32518 as LPCWSTR;
pub const IDI_WARNING: LPCWSTR = IDI_EXCLAMATION;
pub const IDI_ERROR: LPCWSTR = IDI_HAND;
pub const IDI_INFORMATION: LPCWSTR = IDI_ASTERISK;
//10853
pub const IDOK: c_int = 1;
pub const IDCANCEL: c_int = 2;
pub const IDABORT: c_int = 3;
pub const IDRETRY: c_int = 4;
pub const IDIGNORE: c_int = 5;
pub const IDYES: c_int = 6;
pub const IDNO: c_int = 7;
pub const IDCLOSE: c_int = 8;
pub const IDHELP: c_int = 9;
pub const IDTRYAGAIN: c_int = 10;
pub const IDCONTINUE: c_int = 11;
pub const IDTIMEOUT: c_int = 32000;
pub const ES_LEFT: DWORD = 0x0000;
pub const ES_CENTER: DWORD = 0x0001;
pub const ES_RIGHT: DWORD = 0x0002;
pub const ES_MULTILINE: DWORD = 0x0004;
pub const ES_UPPERCASE: DWORD = 0x0008;
pub const ES_LOWERCASE: DWORD = 0x0010;
pub const ES_PASSWORD: DWORD = 0x0020;
pub const ES_AUTOVSCROLL: DWORD = 0x0040;
pub const ES_AUTOHSCROLL: DWORD = 0x0080;
pub const ES_NOHIDESEL: DWORD = 0x0100;
pub const ES_OEMCONVERT: DWORD = 0x0400;
pub const ES_READONLY: DWORD = 0x0800;
pub const ES_WANTRETURN: DWORD = 0x1000;
pub const ES_NUMBER: DWORD = 0x2000;
pub const EN_SETFOCUS: WORD = 0x0100;
pub const EN_KILLFOCUS: WORD = 0x0200;
pub const EN_CHANGE: WORD = 0x0300;
pub const EN_UPDATE: WORD = 0x0400;
pub const EN_ERRSPACE: WORD = 0x0500;
pub const EN_MAXTEXT: WORD = 0x0501;
pub const EN_HSCROLL: WORD = 0x0601;
pub const EN_VSCROLL: WORD = 0x0602;
pub const EN_ALIGN_LTR_EC: WORD = 0x0700;
pub const EN_ALIGN_RTL_EC: WORD = 0x0701;
pub const EC_LEFTMARGIN: WORD = 0x0001;
pub const EC_RIGHTMARGIN: WORD = 0x0002;
pub const EC_USEFONTINFO: WORD = 0xffff;
pub const EMSIS_COMPOSITIONSTRING: WORD = 0x0001;
pub const EIMES_GETCOMPSTRATONCE: WORD = 0x0001;
pub const EIMES_CANCELCOMPSTRINFOCUS: WORD = 0x0002;
pub const EIMES_COMPLETECOMPSTRKILLFOCUS: WORD = 0x0004;
pub const EM_GETSEL: WORD = 0x00B0;
pub const EM_SETSEL: WORD = 0x00B1;
pub const EM_GETRECT: WORD = 0x00B2;
pub const EM_SETRECT: WORD = 0x00B3;
pub const EM_SETRECTNP: WORD = 0x00B4;
pub const EM_SCROLL: WORD = 0x00B5;
pub const EM_LINESCROLL: WORD = 0x00B6;
pub const EM_SCROLLCARET: WORD = 0x00B7;
pub const EM_GETMODIFY: WORD = 0x00B8;
pub const EM_SETMODIFY: WORD = 0x00B9;
pub const EM_GETLINECOUNT: WORD = 0x00BA;
pub const EM_LINEINDEX: WORD = 0x00BB;
pub const EM_SETHANDLE: WORD = 0x00BC;
pub const EM_GETHANDLE: WORD = 0x00BD;
pub const EM_GETTHUMB: WORD = 0x00BE;
pub const EM_LINELENGTH: WORD = 0x00C1;
pub const EM_REPLACESEL: WORD = 0x00C2;
pub const EM_GETLINE: WORD = 0x00C4;
pub const EM_LIMITTEXT: WORD = 0x00C5;
pub const EM_CANUNDO: WORD = 0x00C6;
pub const EM_UNDO: WORD = 0x00C7;
pub const EM_FMTLINES: WORD = 0x00C8;
pub const EM_LINEFROMCHAR: WORD = 0x00C9;
pub const EM_SETTABSTOPS: WORD = 0x00CB;
pub const EM_SETPASSWORDCHAR: WORD = 0x00CC;
pub const EM_EMPTYUNDOBUFFER: WORD = 0x00CD;
pub const EM_GETFIRSTVISIBLELINE: WORD = 0x00CE;
pub const EM_SETREADONLY: WORD = 0x00CF;
pub const EM_SETWORDBREAKPROC: WORD = 0x00D0;
pub const EM_GETWORDBREAKPROC: WORD = 0x00D1;
pub const EM_GETPASSWORDCHAR: WORD = 0x00D2;
pub const EM_SETMARGINS: WORD = 0x00D3;
pub const EM_GETMARGINS: WORD = 0x00D4;
pub const EM_SETLIMITTEXT: WORD = EM_LIMITTEXT;
pub const EM_GETLIMITTEXT: WORD = 0x00D5;
pub const EM_POSFROMCHAR: WORD = 0x00D6;
pub const EM_CHARFROMPOS: WORD = 0x00D7;
pub const EM_SETIMESTATUS: WORD = 0x00D8;
pub const EM_GETIMESTATUS: WORD = 0x00D9;
pub const WB_LEFT: WORD = 0;
pub const WB_RIGHT: WORD = 1;
pub const WB_ISDELIMITER: WORD = 2;
pub const BN_CLICKED: WORD = 0;
pub const BN_PAINT: WORD = 1;
pub const BN_HILITE: WORD = 2;
pub const BN_UNHILITE: WORD = 3;
pub const BN_DISABLE: WORD = 4;
pub const BN_DOUBLECLICKED: WORD = 5;
pub const BN_PUSHED: WORD = BN_HILITE;
pub const BN_UNPUSHED: WORD = BN_UNHILITE;
pub const BN_DBLCLK: WORD = BN_DOUBLECLICKED;
pub const BN_SETFOCUS: WORD = 6;
pub const BN_KILLFOCUS: WORD = 7;
pub const BS_PUSHBUTTON: DWORD = 0x00000000;
pub const BS_DEFPUSHBUTTON: DWORD = 0x00000001;
pub const BS_CHECKBOX: DWORD = 0x00000002;
pub const BS_AUTOCHECKBOX: DWORD = 0x00000003;
pub const BS_RADIOBUTTON: DWORD = 0x00000004;
pub const BS_3STATE: DWORD = 0x00000005;
pub const BS_AUTO3STATE: DWORD = 0x00000006;
pub const BS_GROUPBOX: DWORD = 0x00000007;
pub const BS_USERBUTTON: DWORD = 0x00000008;
pub const BS_AUTORADIOBUTTON: DWORD = 0x00000009;
pub const BS_PUSHBOX: DWORD = 0x0000000A;
pub const BS_OWNERDRAW: DWORD = 0x0000000B;
pub const BS_TYPEMASK: DWORD = 0x0000000F;
pub const BS_LEFTTEXT: DWORD = 0x00000020;
pub const BS_TEXT: DWORD = 0x00000000;
pub const BS_ICON: DWORD = 0x00000040;
pub const BS_BITMAP: DWORD = 0x00000080;
pub const BS_LEFT: DWORD = 0x00000100;
pub const BS_RIGHT: DWORD = 0x00000200;
pub const BS_CENTER: DWORD = 0x00000300;
pub const BS_TOP: DWORD = 0x00000400;
pub const BS_BOTTOM: DWORD = 0x00000800;
pub const BS_VCENTER: DWORD = 0x00000C00;
pub const BS_PUSHLIKE: DWORD = 0x00001000;
pub const BS_MULTILINE: DWORD = 0x00002000;
pub const BS_NOTIFY: DWORD = 0x00004000;
pub const BS_FLAT: DWORD = 0x00008000;
pub const BS_RIGHTBUTTON: DWORD = BS_LEFTTEXT;
pub const BM_GETCHECK: UINT = 0x00F0;
pub const BM_SETCHECK: UINT = 0x00F1;
pub const BM_GETSTATE: UINT = 0x00F2;
pub const BM_SETSTATE: UINT = 0x00F3;
pub const BM_SETSTYLE: UINT = 0x00F4;
pub const BM_CLICK: UINT = 0x00F5;
pub const BM_GETIMAGE: UINT = 0x00F6;
pub const BM_SETIMAGE: UINT = 0x00F7;
pub const BM_SETDONTCLICK: UINT = 0x00F8;
pub const BST_UNCHECKED: WPARAM = 0x0000;
pub const BST_CHECKED: WPARAM = 0x0001;
pub const BST_INDETERMINATE: WPARAM = 0x0002;
pub const BST_PUSHED: LRESULT = 0x0004;
pub const BST_FOCUS: LRESULT = 0x0008;
pub const SS_LEFT: DWORD = 0x00000000;
pub const SS_CENTER: DWORD = 0x00000001;
pub const SS_RIGHT: DWORD = 0x00000002;
pub const SS_ICON: DWORD = 0x00000003;
pub const SS_BLACKRECT: DWORD = 0x00000004;
pub const SS_GRAYRECT: DWORD = 0x00000005;
pub const SS_WHITERECT: DWORD = 0x00000006;
pub const SS_BLACKFRAME: DWORD = 0x00000007;
pub const SS_GRAYFRAME: DWORD = 0x00000008;
pub const SS_WHITEFRAME: DWORD = 0x00000009;
pub const SS_USERITEM: DWORD = 0x0000000A;
pub const SS_SIMPLE: DWORD = 0x0000000B;
pub const SS_LEFTNOWORDWRAP: DWORD = 0x0000000C;
pub const SS_OWNERDRAW: DWORD = 0x0000000D;
pub const SS_BITMAP: DWORD = 0x0000000E;
pub const SS_ENHMETAFILE: DWORD = 0x0000000F;
pub const SS_ETCHEDHORZ: DWORD = 0x00000010;
pub const SS_ETCHEDVERT: DWORD = 0x00000011;
pub const SS_ETCHEDFRAME: DWORD = 0x00000012;
pub const SS_TYPEMASK: DWORD = 0x0000001F;
pub const SS_REALSIZECONTROL: DWORD = 0x00000040;
pub const SS_NOPREFIX: DWORD = 0x00000080;
pub const SS_NOTIFY: DWORD = 0x00000100;
pub const SS_CENTERIMAGE: DWORD = 0x00000200;
pub const SS_RIGHTJUST: DWORD = 0x00000400;
pub const SS_REALSIZEIMAGE: DWORD = 0x00000800;
pub const SS_SUNKEN: DWORD = 0x00001000;
pub const SS_EDITCONTROL: DWORD = 0x00002000;
pub const SS_ENDELLIPSIS: DWORD = 0x00004000;
pub const SS_PATHELLIPSIS: DWORD = 0x00008000;
pub const SS_WORDELLIPSIS: DWORD = 0x0000C000;
pub const SS_ELLIPSISMASK: DWORD = 0x0000C000;
pub const STM_SETICON: UINT = 0x0170;
pub const STM_GETICON: UINT = 0x0171;
pub const STM_SETIMAGE: UINT = 0x0172;
pub const STM_GETIMAGE: UINT = 0x0173;
pub const STN_CLICKED: WORD = 0;
pub const STN_DBLCLK: WORD = 1;
pub const STN_ENABLE: WORD = 2;
pub const STN_DISABLE: WORD = 3;
pub const STM_MSGMAX: WORD = 0x0174;
//11194
extern "system" {
    pub fn IsDialogMessageA(
        hDlg: HWND,
        lpMsg: LPMSG,
    ) -> BOOL;
    pub fn IsDialogMessageW(
        hDlg: HWND,
        lpMsg: LPMSG,
    ) -> BOOL;
    pub fn MapDialogRect(
        hDlg: HWND,
        lpRect: LPRECT,
    ) -> BOOL;
    pub fn DlgDirListA(
        hDlg: HWND,
        lpPathSpec: LPSTR,
        nIDListBox: c_int,
        nIDStaticPath: c_int,
        uFileType: UINT,
    ) -> c_int;
    pub fn DlgDirListW(
        hDlg: HWND,
        lpPathSpec: LPWSTR,
        nIDListBox: c_int,
        nIDStaticPath: c_int,
        uFileType: UINT,
    ) -> c_int;
}
//11265
extern "system" {
    pub fn DlgDirSelectExA(
        hwndDlg: HWND,
        lpString: LPSTR,
        chCount: c_int,
        idListBox: c_int,
    ) -> BOOL;
    pub fn DlgDirSelectExW(
        hwndDlg: HWND,
        lpString: LPWSTR,
        chCount: c_int,
        idListBox: c_int,
    ) -> BOOL;
    pub fn DlgDirListComboBoxA(
        hDlg: HWND,
        lpPathSpec: LPSTR,
        nIDComboBox: c_int,
        nIDStaticPath: c_int,
        uFiletype: UINT,
    ) -> c_int;
    pub fn DlgDirListComboBoxW(
        hDlg: HWND,
        lpPathSpec: LPWSTR,
        nIDComboBox: c_int,
        nIDStaticPath: c_int,
        uFiletype: UINT,
    ) -> c_int;
    pub fn DlgDirSelectComboBoxExA(
        hwndDlg: HWND,
        lpString: LPSTR,
        cchOut: c_int,
        idComboBox: c_int,
    ) -> BOOL;
    pub fn DlgDirSelectComboBoxExW(
        hwndDlg: HWND,
        lpString: LPWSTR,
        cchOut: c_int,
        idComboBox: c_int,
    ) -> BOOL;
}
pub const DS_ABSALIGN: DWORD = 0x01;
pub const DS_SYSMODAL: DWORD = 0x02;
pub const DS_LOCALEDIT: DWORD = 0x20;
pub const DS_SETFONT: DWORD = 0x40;
pub const DS_MODALFRAME: DWORD = 0x80;
pub const DS_NOIDLEMSG: DWORD = 0x100;
pub const DS_SETFOREGROUND: DWORD = 0x200;
pub const DS_3DLOOK: DWORD = 0x0004;
pub const DS_FIXEDSYS: DWORD = 0x0008;
pub const DS_NOFAILCREATE: DWORD = 0x0010;
pub const DS_CONTROL: DWORD = 0x0400;
pub const DS_CENTER: DWORD = 0x0800;
pub const DS_CENTERMOUSE: DWORD = 0x1000;
pub const DS_CONTEXTHELP: DWORD = 0x2000;
pub const DS_SHELLFONT: DWORD = DS_SETFONT | DS_FIXEDSYS;
pub const DS_USEPIXELS: DWORD = 0x8000;
pub const DM_GETDEFID: UINT = WM_USER + 0;
pub const DM_SETDEFID: UINT = WM_USER + 1;
pub const DM_REPOSITION: UINT = WM_USER + 2;
pub const DC_HASDEFID: WORD = 0x534B;
pub const DLGC_WANTARROWS: LRESULT = 0x0001;
pub const DLGC_WANTTAB: LRESULT = 0x0002;
pub const DLGC_WANTALLKEYS: LRESULT = 0x0004;
pub const DLGC_WANTMESSAGE: LRESULT = 0x0004;
pub const DLGC_HASSETSEL: LRESULT = 0x0008;
pub const DLGC_DEFPUSHBUTTON: LRESULT = 0x0010;
pub const DLGC_UNDEFPUSHBUTTON: LRESULT = 0x0020;
pub const DLGC_RADIOBUTTON: LRESULT = 0x0040;
pub const DLGC_WANTCHARS: LRESULT = 0x0080;
pub const DLGC_STATIC: LRESULT = 0x0100;
pub const DLGC_BUTTON: LRESULT = 0x2000;
pub const LB_OKAY: LRESULT = 0;
pub const LB_ERR: LRESULT = -1;
pub const LB_ERRSPACE: LRESULT = -2;
pub const LBN_ERRSPACE: WORD = -2i16 as u16;
pub const LBN_SELCHANGE: WORD = 1;
pub const LBN_DBLCLK: WORD = 2;
pub const LBN_SELCANCEL: WORD = 3;
pub const LBN_SETFOCUS: WORD = 4;
pub const LBN_KILLFOCUS: WORD = 5;
pub const LB_ADDSTRING: UINT = 0x0180;
pub const LB_INSERTSTRING: UINT = 0x0181;
pub const LB_DELETESTRING: UINT = 0x0182;
pub const LB_SELITEMRANGEEX: UINT = 0x0183;
pub const LB_RESETCONTENT: UINT = 0x0184;
pub const LB_SETSEL: UINT = 0x0185;
pub const LB_SETCURSEL: UINT = 0x0186;
pub const LB_GETSEL: UINT = 0x0187;
pub const LB_GETCURSEL: UINT = 0x0188;
pub const LB_GETTEXT: UINT = 0x0189;
pub const LB_GETTEXTLEN: UINT = 0x018A;
pub const LB_GETCOUNT: UINT = 0x018B;
pub const LB_SELECTSTRING: UINT = 0x018C;
pub const LB_DIR: UINT = 0x018D;
pub const LB_GETTOPINDEX: UINT = 0x018E;
pub const LB_FINDSTRING: UINT = 0x018F;
pub const LB_GETSELCOUNT: UINT = 0x0190;
pub const LB_GETSELITEMS: UINT = 0x0191;
pub const LB_SETTABSTOPS: UINT = 0x0192;
pub const LB_GETHORIZONTALEXTENT: UINT = 0x0193;
pub const LB_SETHORIZONTALEXTENT: UINT = 0x0194;
pub const LB_SETCOLUMNWIDTH: UINT = 0x0195;
pub const LB_ADDFILE: UINT = 0x0196;
pub const LB_SETTOPINDEX: UINT = 0x0197;
pub const LB_GETITEMRECT: UINT = 0x0198;
pub const LB_GETITEMDATA: UINT = 0x0199;
pub const LB_SETITEMDATA: UINT = 0x019A;
pub const LB_SELITEMRANGE: UINT = 0x019B;
pub const LB_SETANCHORINDEX: UINT = 0x019C;
pub const LB_GETANCHORINDEX: UINT = 0x019D;
pub const LB_SETCARETINDEX: UINT = 0x019E;
pub const LB_GETCARETINDEX: UINT = 0x019F;
pub const LB_SETITEMHEIGHT: UINT = 0x01A0;
pub const LB_GETITEMHEIGHT: UINT = 0x01A1;
pub const LB_FINDSTRINGEXACT: UINT = 0x01A2;
pub const LB_SETLOCALE: UINT = 0x01A5;
pub const LB_GETLOCALE: UINT = 0x01A6;
pub const LB_SETCOUNT: UINT = 0x01A7;
pub const LB_INITSTORAGE: UINT = 0x01A8;
pub const LB_ITEMFROMPOINT: UINT = 0x01A9;
pub const LB_MULTIPLEADDSTRING: UINT = 0x01B1;
pub const LB_GETLISTBOXINFO: UINT = 0x01B2;
pub const LB_MSGMAX: UINT = 0x01B3;
pub const LBS_NOTIFY: DWORD = 0x0001;
pub const LBS_SORT: DWORD = 0x0002;
pub const LBS_NOREDRAW: DWORD = 0x0004;
pub const LBS_MULTIPLESEL: DWORD = 0x0008;
pub const LBS_OWNERDRAWFIXED: DWORD = 0x0010;
pub const LBS_OWNERDRAWVARIABLE: DWORD = 0x0020;
pub const LBS_HASSTRINGS: DWORD = 0x0040;
pub const LBS_USETABSTOPS: DWORD = 0x0080;
pub const LBS_NOINTEGRALHEIGHT: DWORD = 0x0100;
pub const LBS_MULTICOLUMN: DWORD = 0x0200;
pub const LBS_WANTKEYBOARDINPUT: DWORD = 0x0400;
pub const LBS_EXTENDEDSEL: DWORD = 0x0800;
pub const LBS_DISABLENOSCROLL: DWORD = 0x1000;
pub const LBS_NODATA: DWORD = 0x2000;
pub const LBS_NOSEL: DWORD = 0x4000;
pub const LBS_COMBOBOX: DWORD = 0x8000;
pub const LBS_STANDARD: DWORD = LBS_NOTIFY | LBS_SORT | WS_VSCROLL | WS_BORDER;
pub const CB_OKAY: LRESULT = 0;
pub const CB_ERR: LRESULT = -1;
pub const CB_ERRSPACE: LRESULT = -2;
pub const CBN_ERRSPACE: WORD = -1i16 as u16;
pub const CBN_SELCHANGE: WORD = 1;
pub const CBN_DBLCLK: WORD = 2;
pub const CBN_SETFOCUS: WORD = 3;
pub const CBN_KILLFOCUS: WORD = 4;
pub const CBN_EDITCHANGE: WORD = 5;
pub const CBN_EDITUPDATE: WORD = 6;
pub const CBN_DROPDOWN: WORD = 7;
pub const CBN_CLOSEUP: WORD = 8;
pub const CBN_SELENDOK: WORD = 9;
pub const CBN_SELENDCANCEL: WORD = 10;
pub const CBS_SIMPLE: DWORD = 0x0001;
pub const CBS_DROPDOWN: DWORD = 0x0002;
pub const CBS_DROPDOWNLIST: DWORD = 0x0003;
pub const CBS_OWNERDRAWFIXED: DWORD = 0x0010;
pub const CBS_OWNERDRAWVARIABLE: DWORD = 0x0020;
pub const CBS_AUTOHSCROLL: DWORD = 0x0040;
pub const CBS_OEMCONVERT: DWORD = 0x0080;
pub const CBS_SORT: DWORD = 0x0100;
pub const CBS_HASSTRINGS: DWORD = 0x0200;
pub const CBS_NOINTEGRALHEIGHT: DWORD = 0x0400;
pub const CBS_DISABLENOSCROLL: DWORD = 0x0800;
pub const CBS_UPPERCASE: DWORD = 0x2000;
pub const CBS_LOWERCASE: DWORD = 0x4000;
//11571
pub const CB_GETEDITSEL: UINT = 0x0140;
pub const CB_LIMITTEXT: UINT = 0x0141;
pub const CB_SETEDITSEL: UINT = 0x0142;
pub const CB_ADDSTRING: UINT = 0x0143;
pub const CB_DELETESTRING: UINT = 0x0144;
pub const CB_DIR: UINT = 0x0145;
pub const CB_GETCOUNT: UINT = 0x0146;
pub const CB_GETCURSEL: UINT = 0x0147;
pub const CB_GETLBTEXT: UINT = 0x0148;
pub const CB_GETLBTEXTLEN: UINT = 0x0149;
pub const CB_INSERTSTRING: UINT = 0x014A;
pub const CB_RESETCONTENT: UINT = 0x014B;
pub const CB_FINDSTRING: UINT = 0x014C;
pub const CB_SELECTSTRING: UINT = 0x014D;
pub const CB_SETCURSEL: UINT = 0x014E;
pub const CB_SHOWDROPDOWN: UINT = 0x014F;
pub const CB_GETITEMDATA: UINT = 0x0150;
pub const CB_SETITEMDATA: UINT = 0x0151;
pub const CB_GETDROPPEDCONTROLRECT: UINT = 0x0152;
pub const CB_SETITEMHEIGHT: UINT = 0x0153;
pub const CB_GETITEMHEIGHT: UINT = 0x0154;
pub const CB_SETEXTENDEDUI: UINT = 0x0155;
pub const CB_GETEXTENDEDUI: UINT = 0x0156;
pub const CB_GETDROPPEDSTATE: UINT = 0x0157;
pub const CB_FINDSTRINGEXACT: UINT = 0x0158;
pub const CB_SETLOCALE: UINT = 0x0159;
pub const CB_GETLOCALE: UINT = 0x015A;
pub const CB_GETTOPINDEX: UINT = 0x015b;
pub const CB_SETTOPINDEX: UINT = 0x015c;
pub const CB_GETHORIZONTALEXTENT: UINT = 0x015d;
pub const CB_SETHORIZONTALEXTENT: UINT = 0x015e;
pub const CB_GETDROPPEDWIDTH: UINT = 0x015f;
pub const CB_SETDROPPEDWIDTH: UINT = 0x0160;
pub const CB_INITSTORAGE: UINT = 0x0161;
pub const CB_MULTIPLEADDSTRING: UINT = 0x0163;
pub const CB_GETCOMBOBOXINFO: UINT = 0x0164;
pub const CB_MSGMAX: UINT = 0x0165;
pub const SBS_HORZ: DWORD = 0x0000;
pub const SBS_VERT: DWORD = 0x0001;
pub const SBS_TOPALIGN: DWORD = 0x0002;
pub const SBS_LEFTALIGN: DWORD = 0x0002;
pub const SBS_BOTTOMALIGN: DWORD = 0x0004;
pub const SBS_RIGHTALIGN: DWORD = 0x0004;
pub const SBS_SIZEBOXTOPLEFTALIGN: DWORD = 0x0002;
pub const SBS_SIZEBOXBOTTOMRIGHTALIGN: DWORD = 0x0004;
pub const SBS_SIZEBOX: DWORD = 0x0008;
pub const SBS_SIZEGRIP: DWORD = 0x0010;
pub const SBM_SETPOS: UINT = 0x00E0;
pub const SBM_GETPOS: UINT = 0x00E1;
pub const SBM_SETRANGE: UINT = 0x00E2;
pub const SBM_SETRANGEREDRAW: UINT = 0x00E6;
pub const SBM_GETRANGE: UINT = 0x00E3;
pub const SBM_ENABLE_ARROWS: UINT = 0x00E4;
pub const SBM_SETSCROLLINFO: UINT = 0x00E9;
pub const SBM_GETSCROLLINFO: UINT = 0x00EA;
pub const SBM_GETSCROLLBARINFO: UINT = 0x00EB;
pub const SIF_RANGE: UINT = 0x0001;
pub const SIF_PAGE: UINT = 0x0002;
pub const SIF_POS: UINT = 0x0004;
pub const SIF_DISABLENOSCROLL: UINT = 0x0008;
pub const SIF_TRACKPOS: UINT = 0x0010;
pub const SIF_ALL: UINT = SIF_RANGE | SIF_PAGE | SIF_POS | SIF_TRACKPOS;
STRUCT!{struct SCROLLINFO {
    cbSize: UINT,
    fMask: UINT,
    nMin: c_int,
    nMax: c_int,
    nPage: UINT,
    nPos: c_int,
    nTrackPos: c_int,
}}
pub type LPSCROLLINFO = *mut SCROLLINFO;
pub type LPCSCROLLINFO = *const SCROLLINFO;
extern "system" {
    pub fn SetScrollInfo(
        hwnd: HWND,
        nBar: c_int,
        lpsi: *const SCROLLINFO,
        redraw: BOOL,
    ) -> c_int;
    pub fn GetScrollInfo(
        hwnd: HWND,
        nBar: c_int,
        lpsi: *mut SCROLLINFO,
    ) -> BOOL;
}
pub const MDITILE_VERTICAL: UINT = 0x0000;
pub const MDITILE_HORIZONTAL: UINT = 0x0001;
pub const MDITILE_SKIPDISABLED: UINT = 0x0002;
pub const MDITILE_ZORDER: UINT = 0x0004;
//11776
extern "system" {
    pub fn DefFrameProcA(
        hwnd: HWND,
        hwndMDIClient: HWND,
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
    pub fn DefFrameProcW(
        hwnd: HWND,
        hwndMDIClient: HWND,
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
    pub fn DefMDIChildProcA(
        hwnd: HWND,
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
    pub fn DefMDIChildProcW(
        hwnd: HWND,
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
    pub fn ArrangeIconicWindows(
        hWnd: HWND,
    ) -> UINT;
    pub fn CreateMDIWindowA(
        lpClassName: LPCSTR,
        lpWindowName: LPCSTR,
        dwStyle: DWORD,
        X: c_int,
        Y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hInstance: HINSTANCE,
        lParam: LPARAM,
    ) -> HWND;
    pub fn CreateMDIWindowW(
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        X: c_int,
        Y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hInstance: HINSTANCE,
        lParam: LPARAM,
    ) -> HWND;
    pub fn CascadeWindows(
        hwndParent: HWND,
        wHow: UINT,
        lpRect: *const RECT,
        cKids: UINT,
        lpKids: *const HWND,
    ) -> WORD;
}
//12016
extern "system" {
    pub fn WinHelpA(
        hWndMain: HWND,
        lpszHelp: LPCSTR,
        uCommand: UINT,
        dwData: ULONG_PTR,
    ) -> BOOL;
    pub fn WinHelpW(
        hWndMain: HWND,
        lpszHelp: LPCWSTR,
        uCommand: UINT,
        dwData: ULONG_PTR,
    ) -> BOOL;
}
//12083
pub const SPI_GETBEEP: UINT = 0x0001;
pub const SPI_SETBEEP: UINT = 0x0002;
pub const SPI_GETMOUSE: UINT = 0x0003;
pub const SPI_SETMOUSE: UINT = 0x0004;
pub const SPI_GETBORDER: UINT = 0x0005;
pub const SPI_SETBORDER: UINT = 0x0006;
pub const SPI_GETKEYBOARDSPEED: UINT = 0x000A;
pub const SPI_SETKEYBOARDSPEED: UINT = 0x000B;
pub const SPI_LANGDRIVER: UINT = 0x000C;
pub const SPI_ICONHORIZONTALSPACING: UINT = 0x000D;
pub const SPI_GETSCREENSAVETIMEOUT: UINT = 0x000E;
pub const SPI_SETSCREENSAVETIMEOUT: UINT = 0x000F;
pub const SPI_GETSCREENSAVEACTIVE: UINT = 0x0010;
pub const SPI_SETSCREENSAVEACTIVE: UINT = 0x0011;
pub const SPI_GETGRIDGRANULARITY: UINT = 0x0012;
pub const SPI_SETGRIDGRANULARITY: UINT = 0x0013;
pub const SPI_SETDESKWALLPAPER: UINT = 0x0014;
pub const SPI_SETDESKPATTERN: UINT = 0x0015;
pub const SPI_GETKEYBOARDDELAY: UINT = 0x0016;
pub const SPI_SETKEYBOARDDELAY: UINT = 0x0017;
pub const SPI_ICONVERTICALSPACING: UINT = 0x0018;
pub const SPI_GETICONTITLEWRAP: UINT = 0x0019;
pub const SPI_SETICONTITLEWRAP: UINT = 0x001A;
pub const SPI_GETMENUDROPALIGNMENT: UINT = 0x001B;
pub const SPI_SETMENUDROPALIGNMENT: UINT = 0x001C;
pub const SPI_SETDOUBLECLKWIDTH: UINT = 0x001D;
pub const SPI_SETDOUBLECLKHEIGHT: UINT = 0x001E;
pub const SPI_GETICONTITLELOGFONT: UINT = 0x001F;
pub const SPI_SETDOUBLECLICKTIME: UINT = 0x0020;
pub const SPI_SETMOUSEBUTTONSWAP: UINT = 0x0021;
pub const SPI_SETICONTITLELOGFONT: UINT = 0x0022;
pub const SPI_GETFASTTASKSWITCH: UINT = 0x0023;
pub const SPI_SETFASTTASKSWITCH: UINT = 0x0024;
pub const SPI_SETDRAGFULLWINDOWS: UINT = 0x0025;
pub const SPI_GETDRAGFULLWINDOWS: UINT = 0x0026;
pub const SPI_GETNONCLIENTMETRICS: UINT = 0x0029;
pub const SPI_SETNONCLIENTMETRICS: UINT = 0x002A;
pub const SPI_GETMINIMIZEDMETRICS: UINT = 0x002B;
pub const SPI_SETMINIMIZEDMETRICS: UINT = 0x002C;
pub const SPI_GETICONMETRICS: UINT = 0x002D;
pub const SPI_SETICONMETRICS: UINT = 0x002E;
pub const SPI_SETWORKAREA: UINT = 0x002F;
pub const SPI_GETWORKAREA: UINT = 0x0030;
pub const SPI_SETPENWINDOWS: UINT = 0x0031;
pub const SPI_GETHIGHCONTRAST: UINT = 0x0042;
pub const SPI_SETHIGHCONTRAST: UINT = 0x0043;
pub const SPI_GETKEYBOARDPREF: UINT = 0x0044;
pub const SPI_SETKEYBOARDPREF: UINT = 0x0045;
pub const SPI_GETSCREENREADER: UINT = 0x0046;
pub const SPI_SETSCREENREADER: UINT = 0x0047;
pub const SPI_GETANIMATION: UINT = 0x0048;
pub const SPI_SETANIMATION: UINT = 0x0049;
pub const SPI_GETFONTSMOOTHING: UINT = 0x004A;
pub const SPI_SETFONTSMOOTHING: UINT = 0x004B;
pub const SPI_SETDRAGWIDTH: UINT = 0x004C;
pub const SPI_SETDRAGHEIGHT: UINT = 0x004D;
pub const SPI_SETHANDHELD: UINT = 0x004E;
pub const SPI_GETLOWPOWERTIMEOUT: UINT = 0x004F;
pub const SPI_GETPOWEROFFTIMEOUT: UINT = 0x0050;
pub const SPI_SETLOWPOWERTIMEOUT: UINT = 0x0051;
pub const SPI_SETPOWEROFFTIMEOUT: UINT = 0x0052;
pub const SPI_GETLOWPOWERACTIVE: UINT = 0x0053;
pub const SPI_GETPOWEROFFACTIVE: UINT = 0x0054;
pub const SPI_SETLOWPOWERACTIVE: UINT = 0x0055;
pub const SPI_SETPOWEROFFACTIVE: UINT = 0x0056;
pub const SPI_SETCURSORS: UINT = 0x0057;
pub const SPI_SETICONS: UINT = 0x0058;
pub const SPI_GETDEFAULTINPUTLANG: UINT = 0x0059;
pub const SPI_SETDEFAULTINPUTLANG: UINT = 0x005A;
pub const SPI_SETLANGTOGGLE: UINT = 0x005B;
pub const SPI_GETWINDOWSEXTENSION: UINT = 0x005C;
pub const SPI_SETMOUSETRAILS: UINT = 0x005D;
pub const SPI_GETMOUSETRAILS: UINT = 0x005E;
pub const SPI_SETSCREENSAVERRUNNING: UINT = 0x0061;
pub const SPI_SCREENSAVERRUNNING: UINT = SPI_SETSCREENSAVERRUNNING;
pub const SPI_GETFILTERKEYS: UINT = 0x0032;
pub const SPI_SETFILTERKEYS: UINT = 0x0033;
pub const SPI_GETTOGGLEKEYS: UINT = 0x0034;
pub const SPI_SETTOGGLEKEYS: UINT = 0x0035;
pub const SPI_GETMOUSEKEYS: UINT = 0x0036;
pub const SPI_SETMOUSEKEYS: UINT = 0x0037;
pub const SPI_GETSHOWSOUNDS: UINT = 0x0038;
pub const SPI_SETSHOWSOUNDS: UINT = 0x0039;
pub const SPI_GETSTICKYKEYS: UINT = 0x003A;
pub const SPI_SETSTICKYKEYS: UINT = 0x003B;
pub const SPI_GETACCESSTIMEOUT: UINT = 0x003C;
pub const SPI_SETACCESSTIMEOUT: UINT = 0x003D;
pub const SPI_GETSERIALKEYS: UINT = 0x003E;
pub const SPI_SETSERIALKEYS: UINT = 0x003F;
pub const SPI_GETSOUNDSENTRY: UINT = 0x0040;
pub const SPI_SETSOUNDSENTRY: UINT = 0x0041;
pub const SPI_GETSNAPTODEFBUTTON: UINT = 0x005F;
pub const SPI_SETSNAPTODEFBUTTON: UINT = 0x0060;
pub const SPI_GETMOUSEHOVERWIDTH: UINT = 0x0062;
pub const SPI_SETMOUSEHOVERWIDTH: UINT = 0x0063;
pub const SPI_GETMOUSEHOVERHEIGHT: UINT = 0x0064;
pub const SPI_SETMOUSEHOVERHEIGHT: UINT = 0x0065;
pub const SPI_GETMOUSEHOVERTIME: UINT = 0x0066;
pub const SPI_SETMOUSEHOVERTIME: UINT = 0x0067;
pub const SPI_GETWHEELSCROLLLINES: UINT = 0x0068;
pub const SPI_SETWHEELSCROLLLINES: UINT = 0x0069;
pub const SPI_GETMENUSHOWDELAY: UINT = 0x006A;
pub const SPI_SETMENUSHOWDELAY: UINT = 0x006B;
pub const SPI_GETWHEELSCROLLCHARS: UINT = 0x006C;
pub const SPI_SETWHEELSCROLLCHARS: UINT = 0x006D;
pub const SPI_GETSHOWIMEUI: UINT = 0x006E;
pub const SPI_SETSHOWIMEUI: UINT = 0x006F;
pub const SPI_GETMOUSESPEED: UINT = 0x0070;
pub const SPI_SETMOUSESPEED: UINT = 0x0071;
pub const SPI_GETSCREENSAVERRUNNING: UINT = 0x0072;
pub const SPI_GETDESKWALLPAPER: UINT = 0x0073;
pub const SPI_GETAUDIODESCRIPTION: UINT = 0x0074;
pub const SPI_SETAUDIODESCRIPTION: UINT = 0x0075;
pub const SPI_GETSCREENSAVESECURE: UINT = 0x0076;
pub const SPI_SETSCREENSAVESECURE: UINT = 0x0077;
pub const SPI_GETHUNGAPPTIMEOUT: UINT = 0x0078;
pub const SPI_SETHUNGAPPTIMEOUT: UINT = 0x0079;
pub const SPI_GETWAITTOKILLTIMEOUT: UINT = 0x007A;
pub const SPI_SETWAITTOKILLTIMEOUT: UINT = 0x007B;
pub const SPI_GETWAITTOKILLSERVICETIMEOUT: UINT = 0x007C;
pub const SPI_SETWAITTOKILLSERVICETIMEOUT: UINT = 0x007D;
pub const SPI_GETMOUSEDOCKTHRESHOLD: UINT = 0x007E;
pub const SPI_SETMOUSEDOCKTHRESHOLD: UINT = 0x007F;
pub const SPI_GETPENDOCKTHRESHOLD: UINT = 0x0080;
pub const SPI_SETPENDOCKTHRESHOLD: UINT = 0x0081;
pub const SPI_GETWINARRANGING: UINT = 0x0082;
pub const SPI_SETWINARRANGING: UINT = 0x0083;
pub const SPI_GETMOUSEDRAGOUTTHRESHOLD: UINT = 0x0084;
pub const SPI_SETMOUSEDRAGOUTTHRESHOLD: UINT = 0x0085;
pub const SPI_GETPENDRAGOUTTHRESHOLD: UINT = 0x0086;
pub const SPI_SETPENDRAGOUTTHRESHOLD: UINT = 0x0087;
pub const SPI_GETMOUSESIDEMOVETHRESHOLD: UINT = 0x0088;
pub const SPI_SETMOUSESIDEMOVETHRESHOLD: UINT = 0x0089;
pub const SPI_GETPENSIDEMOVETHRESHOLD: UINT = 0x008A;
pub const SPI_SETPENSIDEMOVETHRESHOLD: UINT = 0x008B;
pub const SPI_GETDRAGFROMMAXIMIZE: UINT = 0x008C;
pub const SPI_SETDRAGFROMMAXIMIZE: UINT = 0x008D;
pub const SPI_GETSNAPSIZING: UINT = 0x008E;
pub const SPI_SETSNAPSIZING: UINT = 0x008F;
pub const SPI_GETDOCKMOVING: UINT = 0x0090;
pub const SPI_SETDOCKMOVING: UINT = 0x0091;
pub const SPI_GETACTIVEWINDOWTRACKING: UINT = 0x1000;
pub const SPI_SETACTIVEWINDOWTRACKING: UINT = 0x1001;
pub const SPI_GETMENUANIMATION: UINT = 0x1002;
pub const SPI_SETMENUANIMATION: UINT = 0x1003;
pub const SPI_GETCOMBOBOXANIMATION: UINT = 0x1004;
pub const SPI_SETCOMBOBOXANIMATION: UINT = 0x1005;
pub const SPI_GETLISTBOXSMOOTHSCROLLING: UINT = 0x1006;
pub const SPI_SETLISTBOXSMOOTHSCROLLING: UINT = 0x1007;
pub const SPI_GETGRADIENTCAPTIONS: UINT = 0x1008;
pub const SPI_SETGRADIENTCAPTIONS: UINT = 0x1009;
pub const SPI_GETKEYBOARDCUES: UINT = 0x100A;
pub const SPI_SETKEYBOARDCUES: UINT = 0x100B;
pub const SPI_GETMENUUNDERLINES: UINT = SPI_GETKEYBOARDCUES;
pub const SPI_SETMENUUNDERLINES: UINT = SPI_SETKEYBOARDCUES;
pub const SPI_GETACTIVEWNDTRKZORDER: UINT = 0x100C;
pub const SPI_SETACTIVEWNDTRKZORDER: UINT = 0x100D;
pub const SPI_GETHOTTRACKING: UINT = 0x100E;
pub const SPI_SETHOTTRACKING: UINT = 0x100F;
pub const SPI_GETMENUFADE: UINT = 0x1012;
pub const SPI_SETMENUFADE: UINT = 0x1013;
pub const SPI_GETSELECTIONFADE: UINT = 0x1014;
pub const SPI_SETSELECTIONFADE: UINT = 0x1015;
pub const SPI_GETTOOLTIPANIMATION: UINT = 0x1016;
pub const SPI_SETTOOLTIPANIMATION: UINT = 0x1017;
pub const SPI_GETTOOLTIPFADE: UINT = 0x1018;
pub const SPI_SETTOOLTIPFADE: UINT = 0x1019;
pub const SPI_GETCURSORSHADOW: UINT = 0x101A;
pub const SPI_SETCURSORSHADOW: UINT = 0x101B;
pub const SPI_GETMOUSESONAR: UINT = 0x101C;
pub const SPI_SETMOUSESONAR: UINT = 0x101D;
pub const SPI_GETMOUSECLICKLOCK: UINT = 0x101E;
pub const SPI_SETMOUSECLICKLOCK: UINT = 0x101F;
pub const SPI_GETMOUSEVANISH: UINT = 0x1020;
pub const SPI_SETMOUSEVANISH: UINT = 0x1021;
pub const SPI_GETFLATMENU: UINT = 0x1022;
pub const SPI_SETFLATMENU: UINT = 0x1023;
pub const SPI_GETDROPSHADOW: UINT = 0x1024;
pub const SPI_SETDROPSHADOW: UINT = 0x1025;
pub const SPI_GETBLOCKSENDINPUTRESETS: UINT = 0x1026;
pub const SPI_SETBLOCKSENDINPUTRESETS: UINT = 0x1027;
pub const SPI_GETUIEFFECTS: UINT = 0x103E;
pub const SPI_SETUIEFFECTS: UINT = 0x103F;
pub const SPI_GETDISABLEOVERLAPPEDCONTENT: UINT = 0x1040;
pub const SPI_SETDISABLEOVERLAPPEDCONTENT: UINT = 0x1041;
pub const SPI_GETCLIENTAREAANIMATION: UINT = 0x1042;
pub const SPI_SETCLIENTAREAANIMATION: UINT = 0x1043;
pub const SPI_GETCLEARTYPE: UINT = 0x1048;
pub const SPI_SETCLEARTYPE: UINT = 0x1049;
pub const SPI_GETSPEECHRECOGNITION: UINT = 0x104A;
pub const SPI_SETSPEECHRECOGNITION: UINT = 0x104B;
pub const SPI_GETFOREGROUNDLOCKTIMEOUT: UINT = 0x2000;
pub const SPI_SETFOREGROUNDLOCKTIMEOUT: UINT = 0x2001;
pub const SPI_GETACTIVEWNDTRKTIMEOUT: UINT = 0x2002;
pub const SPI_SETACTIVEWNDTRKTIMEOUT: UINT = 0x2003;
pub const SPI_GETFOREGROUNDFLASHCOUNT: UINT = 0x2004;
pub const SPI_SETFOREGROUNDFLASHCOUNT: UINT = 0x2005;
pub const SPI_GETCARETWIDTH: UINT = 0x2006;
pub const SPI_SETCARETWIDTH: UINT = 0x2007;
pub const SPI_GETMOUSECLICKLOCKTIME: UINT = 0x2008;
pub const SPI_SETMOUSECLICKLOCKTIME: UINT = 0x2009;
pub const SPI_GETFONTSMOOTHINGTYPE: UINT = 0x200A;
pub const SPI_SETFONTSMOOTHINGTYPE: UINT = 0x200B;
pub const FE_FONTSMOOTHINGSTANDARD: UINT = 0x0001;
pub const FE_FONTSMOOTHINGCLEARTYPE: UINT = 0x0002;
pub const SPI_GETFONTSMOOTHINGCONTRAST: UINT = 0x200C;
pub const SPI_SETFONTSMOOTHINGCONTRAST: UINT = 0x200D;
pub const SPI_GETFOCUSBORDERWIDTH: UINT = 0x200E;
pub const SPI_SETFOCUSBORDERWIDTH: UINT = 0x200F;
pub const SPI_GETFOCUSBORDERHEIGHT: UINT = 0x2010;
pub const SPI_SETFOCUSBORDERHEIGHT: UINT = 0x2011;
pub const SPI_GETFONTSMOOTHINGORIENTATION: UINT = 0x2012;
pub const SPI_SETFONTSMOOTHINGORIENTATION: UINT = 0x2013;
pub const FE_FONTSMOOTHINGORIENTATIONBGR: UINT = 0x0000;
pub const FE_FONTSMOOTHINGORIENTATIONRGB: UINT = 0x0001;
pub const SPI_GETMINIMUMHITRADIUS: UINT = 0x2014;
pub const SPI_SETMINIMUMHITRADIUS: UINT = 0x2015;
pub const SPI_GETMESSAGEDURATION: UINT = 0x2016;
pub const SPI_SETMESSAGEDURATION: UINT = 0x2017;
//12472
pub const SPIF_UPDATEINIFILE: UINT = 0x0001;
pub const SPIF_SENDWININICHANGE: UINT = 0x0002;
pub const SPIF_SENDCHANGE: UINT = SPIF_SENDWININICHANGE;
//12484
STRUCT!{struct NONCLIENTMETRICSA {
    cbSize: UINT,
    iBorderWidth: c_int,
    iScrollWidth: c_int,
    iScrollHeight: c_int,
    iCaptionWidth: c_int,
    iCaptionHeight: c_int,
    lfCaptionFont: LOGFONTA,
    iSmCaptionWidth: c_int,
    iSmCaptionHeight: c_int,
    lfSmCaptionFont: LOGFONTA,
    iMenuWidth: c_int,
    iMenuHeight: c_int,
    lfMenuFont: LOGFONTA,
    lfStatusFont: LOGFONTA,
    lfMessageFont: LOGFONTA,
    iPaddedBorderWidth: c_int,
}}
pub type LPNONCLIENTMETRICSA = *mut NONCLIENTMETRICSA;
STRUCT!{struct NONCLIENTMETRICSW {
    cbSize: UINT,
    iBorderWidth: c_int,
    iScrollWidth: c_int,
    iScrollHeight: c_int,
    iCaptionWidth: c_int,
    iCaptionHeight: c_int,
    lfCaptionFont: LOGFONTW,
    iSmCaptionWidth: c_int,
    iSmCaptionHeight: c_int,
    lfSmCaptionFont: LOGFONTW,
    iMenuWidth: c_int,
    iMenuHeight: c_int,
    lfMenuFont: LOGFONTW,
    lfStatusFont: LOGFONTW,
    lfMessageFont: LOGFONTW,
    iPaddedBorderWidth: c_int,
}}
pub type LPNONCLIENTMETRICSW = *mut NONCLIENTMETRICSW;
//12598
STRUCT!{struct ANIMATIONINFO {
    cbSize: UINT,
    iMinAnimate: c_int,
}}
pub type LPANIMATIONINFO = *mut ANIMATIONINFO;
//12638
STRUCT!{struct HIGHCONTRASTA {
    cbSize: UINT,
    dwFlags: DWORD,
    lpszDefaultScheme: LPSTR,
}}
pub type LPHIGHCONTRASTA = *mut HIGHCONTRASTA;
STRUCT!{struct HIGHCONTRASTW {
    cbSize: UINT,
    dwFlags: DWORD,
    lpszDefaultScheme: LPWSTR,
}}
pub type LPHIGHCONTRASTW = *mut HIGHCONTRASTW;
pub const HCF_HIGHCONTRASTON: DWORD = 0x00000001;
pub const HCF_AVAILABLE: DWORD = 0x00000002;
pub const HCF_HOTKEYACTIVE: DWORD = 0x00000004;
pub const HCF_CONFIRMHOTKEY: DWORD = 0x00000008;
pub const HCF_HOTKEYSOUND: DWORD = 0x00000010;
pub const HCF_INDICATOR: DWORD = 0x00000020;
pub const HCF_HOTKEYAVAILABLE: DWORD = 0x00000040;
pub const HCF_LOGONDESKTOP: DWORD = 0x00000100;
pub const HCF_DEFAULTDESKTOP: DWORD = 0x00000200;
pub const CDS_UPDATEREGISTRY: DWORD = 0x00000001;
pub const CDS_TEST: DWORD = 0x00000002;
pub const CDS_FULLSCREEN: DWORD = 0x00000004;
pub const CDS_GLOBAL: DWORD = 0x00000008;
pub const CDS_SET_PRIMARY: DWORD = 0x00000010;
pub const CDS_VIDEOPARAMETERS: DWORD = 0x00000020;
pub const CDS_ENABLE_UNSAFE_MODES: DWORD = 0x00000100;
pub const CDS_DISABLE_UNSAFE_MODES: DWORD = 0x00000200;
pub const CDS_RESET: DWORD = 0x40000000;
pub const CDS_RESET_EX: DWORD = 0x20000000;
pub const CDS_NORESET: DWORD = 0x10000000;
pub const DISP_CHANGE_SUCCESSFUL: LONG = 0;
pub const DISP_CHANGE_RESTART: LONG = 1;
pub const DISP_CHANGE_FAILED: LONG = -1;
pub const DISP_CHANGE_BADMODE: LONG = -2;
pub const DISP_CHANGE_NOTUPDATED: LONG = -3;
pub const DISP_CHANGE_BADFLAGS: LONG = -4;
pub const DISP_CHANGE_BADPARAM: LONG = -5;
pub const DISP_CHANGE_BADDUALVIEW: LONG = -6;
extern "system" {
    pub fn ChangeDisplaySettingsA(
        lpDevMode: *mut DEVMODEA,
        dwFlags: DWORD,
    ) -> LONG;
    pub fn ChangeDisplaySettingsW(
        lpDevMode: *mut DEVMODEW,
        dwFlags: DWORD,
    ) -> LONG;
    pub fn ChangeDisplaySettingsExA(
        lpszDeviceName: LPCSTR,
        lpDevMode: *mut DEVMODEA,
        hwnd: HWND,
        dwFlags: DWORD,
        lParam: LPVOID,
    ) -> LONG;
    pub fn ChangeDisplaySettingsExW(
        lpszDeviceName: LPCWSTR,
        lpDevMode: *mut DEVMODEW,
        hwnd: HWND,
        dwFlags: DWORD,
        lParam: LPVOID,
    ) -> LONG;
}
pub const ENUM_CURRENT_SETTINGS: DWORD = 0xFFFFFFFF;
pub const ENUM_REGISTRY_SETTINGS: DWORD = 0xFFFFFFFE;
extern "system" {
    pub fn EnumDisplaySettingsA(
        lpszDeviceName: LPCSTR,
        iModeNum: DWORD,
        lpDevMode: *mut DEVMODEA,
    ) -> BOOL;
    pub fn EnumDisplaySettingsW(
        lpszDeviceName: LPCWSTR,
        iModeNum: DWORD,
        lpDevMode: *mut DEVMODEW,
    ) -> BOOL;
    pub fn EnumDisplaySettingsExA(
        lpszDeviceName: LPCSTR,
        iModeNum: DWORD,
        lpDevMode: *mut DEVMODEA,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumDisplaySettingsExW(
        lpszDeviceName: LPCWSTR,
        iModeNum: DWORD,
        lpDevMode: *mut DEVMODEW,
        dwFlags: DWORD,
    ) -> BOOL;
}
pub const EDS_RAWMODE: DWORD = 0x00000002;
pub const EDS_ROTATEDMODE: DWORD = 0x00000004;
extern "system" {
    pub fn EnumDisplayDevicesA(
        lpDevice: LPCSTR,
        iDevNum: DWORD,
        lpDisplayDevice: PDISPLAY_DEVICEA,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumDisplayDevicesW(
        lpDevice: LPCWSTR,
        iDevNum: DWORD,
        lpDisplayDevice: PDISPLAY_DEVICEW,
        dwFlags: DWORD,
    ) -> BOOL;
}
pub const EDD_GET_DEVICE_INTERFACE_NAME: DWORD = 0x00000001;
extern "system" {
    pub fn SystemParametersInfoA(
        uiAction: UINT,
        uiParam: UINT,
        pvParam: PVOID,
        fWinIni: UINT,
    ) -> BOOL;
    pub fn SystemParametersInfoW(
        uiAction: UINT,
        uiParam: UINT,
        pvParam: PVOID,
        fWinIni: UINT,
    ) -> BOOL;
    pub fn SystemParametersInfoForDpi(
        uiAction: UINT,
        uiParam: UINT,
        pvParam: PVOID,
        fWinIni: UINT,
        dpi: UINT,
    ) -> BOOL;
}
//13191
extern "system" {
    pub fn SetLastErrorEx(
        dwErrCode: DWORD,
        dwType: DWORD,
    );
    pub fn InternalGetWindowText(
        hWnd: HWND,
        pString: LPWSTR,
        cchMaxCount: c_int,
    ) -> c_int;
    pub fn EndTask(
        hWnd: HWND,
        fShutDown: BOOL,
        fForce: BOOL,
    ) -> BOOL;
    pub fn CancelShutdown() -> BOOL;
}
pub const MONITOR_DEFAULTTONULL: DWORD = 0x00000000;
pub const MONITOR_DEFAULTTOPRIMARY: DWORD = 0x00000001;
pub const MONITOR_DEFAULTTONEAREST: DWORD = 0x00000002;
extern "system" {
    pub fn MonitorFromPoint(
        pt: POINT,
        dwFlags: DWORD,
    ) -> HMONITOR;
    pub fn MonitorFromRect(
        lprc: LPCRECT,
        dwFlags: DWORD,
    ) -> HMONITOR;
    pub fn MonitorFromWindow(
        hwnd: HWND,
        dwFlags: DWORD,
    ) -> HMONITOR;
}
pub const MONITORINFOF_PRIMARY: DWORD = 1;
pub const CCHDEVICENAME: usize = 32;
STRUCT!{struct MONITORINFO {
    cbSize: DWORD,
    rcMonitor: RECT,
    rcWork: RECT,
    dwFlags: DWORD,
}}
pub type LPMONITORINFO = *mut MONITORINFO;
STRUCT!{struct MONITORINFOEXA {
    cbSize: DWORD,
    rcMonitor: RECT,
    rcWork: RECT,
    dwFlags: DWORD,
    szDevice: [CHAR; CCHDEVICENAME],
}}
pub type LPMONITORINFOEXA = *mut MONITORINFOEXA;
STRUCT!{struct MONITORINFOEXW {
    cbSize: DWORD,
    rcMonitor: RECT,
    rcWork: RECT,
    dwFlags: DWORD,
    szDevice: [WCHAR; CCHDEVICENAME],
}}
pub type LPMONITORINFOEXW = *mut MONITORINFOEXW;
extern "system" {
    pub fn GetMonitorInfoA(
        hMonitor: HMONITOR,
        lpmi: LPMONITORINFO,
    ) -> BOOL;
    pub fn GetMonitorInfoW(
        hMonitor: HMONITOR,
        lpmi: LPMONITORINFO,
    ) -> BOOL;
}
FN!{stdcall MONITORENUMPROC(
    HMONITOR,
    HDC,
    LPRECT,
    LPARAM,
) -> BOOL}
extern "system" {
    pub fn EnumDisplayMonitors(
        hdc: HDC,
        lprcClip: LPCRECT,
        lpfnEnum: MONITORENUMPROC,
        dwData: LPARAM,
    ) -> BOOL;
    pub fn NotifyWinEvent(
        event: DWORD,
        hwnd: HWND,
        idObject: LONG,
        idChild: LONG,
    );
}
FN!{stdcall WINEVENTPROC(
    HWINEVENTHOOK,
    DWORD,
    HWND,
    LONG,
    LONG,
    DWORD,
    DWORD,
) -> ()}
extern "system" {
    pub fn SetWinEventHook(
        eventMin: DWORD,
        eventMax: DWORD,
        hmodWinEventProc: HMODULE,
        pfnWinEventProc: WINEVENTPROC,
        idProcess: DWORD,
        idThread: DWORD,
        dwFlags: DWORD,
    ) -> HWINEVENTHOOK;
    pub fn IsWinEventHookInstalled(
        event: DWORD,
    ) -> BOOL;
}
pub const WINEVENT_OUTOFCONTEXT: UINT = 0x0000;
pub const WINEVENT_SKIPOWNTHREAD: UINT = 0x0001;
pub const WINEVENT_SKIPOWNPROCESS: UINT = 0x0002;
pub const WINEVENT_INCONTEXT: UINT = 0x0004;
extern "system" {
    pub fn UnhookWinEvent(
        hWinEventHook: HWINEVENTHOOK,
    ) -> BOOL;
}
pub const CHILDID_SELF: LONG = 0;
pub const INDEXID_OBJECT: LONG = 0;
pub const INDEXID_CONTAINER: LONG = 0;
pub const OBJID_WINDOW: LONG = 0x0000;
pub const OBJID_SYSMENU: LONG = 0xFFFFFFFF;
pub const OBJID_TITLEBAR: LONG = 0xFFFFFFFE;
pub const OBJID_MENU: LONG = 0xFFFFFFFD;
pub const OBJID_CLIENT: LONG = 0xFFFFFFFC;
pub const OBJID_VSCROLL: LONG = 0xFFFFFFFB;
pub const OBJID_HSCROLL: LONG = 0xFFFFFFFA;
pub const OBJID_SIZEGRIP: LONG = 0xFFFFFFF9;
pub const OBJID_CARET: LONG = 0xFFFFFFF8;
pub const OBJID_CURSOR: LONG = 0xFFFFFFF7;
pub const OBJID_ALERT: LONG = 0xFFFFFFF6;
pub const OBJID_SOUND: LONG = 0xFFFFFFF5;
pub const OBJID_QUERYCLASSNAMEIDX: LONG = 0xFFFFFFF4;
pub const OBJID_NATIVEOM: LONG = 0xFFFFFFF0;
pub const EVENT_MIN: UINT = 0x0001;
pub const EVENT_MAX: UINT = 0x7FFFFFFF;
pub const EVENT_SYSTEM_SOUND: UINT = 0x0001;
pub const EVENT_SYSTEM_ALERT: UINT = 0x0002;
pub const EVENT_SYSTEM_FOREGROUND: UINT = 0x0003;
pub const EVENT_SYSTEM_MENUSTART: UINT = 0x0004;
pub const EVENT_SYSTEM_MENUEND: UINT = 0x0005;
pub const EVENT_SYSTEM_MENUPOPUPSTART: UINT = 0x0006;
pub const EVENT_SYSTEM_MENUPOPUPEND: UINT = 0x0007;
pub const EVENT_SYSTEM_CAPTURESTART: UINT = 0x0008;
pub const EVENT_SYSTEM_CAPTUREEND: UINT = 0x0009;
pub const EVENT_SYSTEM_MOVESIZESTART: UINT = 0x000A;
pub const EVENT_SYSTEM_MOVESIZEEND: UINT = 0x000B;
pub const EVENT_SYSTEM_CONTEXTHELPSTART: UINT = 0x000C;
pub const EVENT_SYSTEM_CONTEXTHELPEND: UINT = 0x000D;
pub const EVENT_SYSTEM_DRAGDROPSTART: UINT = 0x000E;
pub const EVENT_SYSTEM_DRAGDROPEND: UINT = 0x000F;
pub const EVENT_SYSTEM_DIALOGSTART: UINT = 0x0010;
pub const EVENT_SYSTEM_DIALOGEND: UINT = 0x0011;
pub const EVENT_SYSTEM_SCROLLINGSTART: UINT = 0x0012;
pub const EVENT_SYSTEM_SCROLLINGEND: UINT = 0x0013;
pub const EVENT_SYSTEM_SWITCHSTART: UINT = 0x0014;
pub const EVENT_SYSTEM_SWITCHEND: UINT = 0x0015;
pub const EVENT_SYSTEM_MINIMIZESTART: UINT = 0x0016;
pub const EVENT_SYSTEM_MINIMIZEEND: UINT = 0x0017;
pub const EVENT_SYSTEM_DESKTOPSWITCH: UINT = 0x0020;
pub const EVENT_SYSTEM_SWITCHER_APPGRABBED: UINT = 0x0024;
pub const EVENT_SYSTEM_SWITCHER_APPOVERTARGET: UINT = 0x0025;
pub const EVENT_SYSTEM_SWITCHER_APPDROPPED: UINT = 0x0026;
pub const EVENT_SYSTEM_SWITCHER_CANCELLED: UINT = 0x0027;
pub const EVENT_SYSTEM_IME_KEY_NOTIFICATION: UINT = 0x0029;
pub const EVENT_SYSTEM_END: UINT = 0x00FF;
pub const EVENT_OEM_DEFINED_START: UINT = 0x0101;
pub const EVENT_OEM_DEFINED_END: UINT = 0x01FF;
pub const EVENT_UIA_EVENTID_START: UINT = 0x4E00;
pub const EVENT_UIA_EVENTID_END: UINT = 0x4EFF;
pub const EVENT_UIA_PROPID_START: UINT = 0x7500;
pub const EVENT_UIA_PROPID_END: UINT = 0x75FF;
pub const EVENT_CONSOLE_CARET: UINT = 0x4001;
pub const EVENT_CONSOLE_UPDATE_REGION: UINT = 0x4002;
pub const EVENT_CONSOLE_UPDATE_SIMPLE: UINT = 0x4003;
pub const EVENT_CONSOLE_UPDATE_SCROLL: UINT = 0x4004;
pub const EVENT_CONSOLE_LAYOUT: UINT = 0x4005;
pub const EVENT_CONSOLE_START_APPLICATION: UINT = 0x4006;
pub const EVENT_CONSOLE_END_APPLICATION: UINT = 0x4007;
#[cfg(target_pointer_width = "64")]
pub const CONSOLE_APPLICATION_16BIT: LONG = 0x0000;
#[cfg(target_pointer_width = "32")]
pub const CONSOLE_APPLICATION_16BIT: LONG = 0x0001;
pub const CONSOLE_CARET_SELECTION: LONG = 0x0001;
pub const CONSOLE_CARET_VISIBLE: LONG = 0x0002;
pub const EVENT_CONSOLE_END: UINT = 0x40FF;
pub const EVENT_OBJECT_CREATE: UINT = 0x8000;
pub const EVENT_OBJECT_DESTROY: UINT = 0x8001;
pub const EVENT_OBJECT_SHOW: UINT = 0x8002;
pub const EVENT_OBJECT_HIDE: UINT = 0x8003;
pub const EVENT_OBJECT_REORDER: UINT = 0x8004;
pub const EVENT_OBJECT_FOCUS: UINT = 0x8005;
pub const EVENT_OBJECT_SELECTION: UINT = 0x8006;
pub const EVENT_OBJECT_SELECTIONADD: UINT = 0x8007;
pub const EVENT_OBJECT_SELECTIONREMOVE: UINT = 0x8008;
pub const EVENT_OBJECT_SELECTIONWITHIN: UINT = 0x8009;
pub const EVENT_OBJECT_STATECHANGE: UINT = 0x800A;
pub const EVENT_OBJECT_LOCATIONCHANGE: UINT = 0x800B;
pub const EVENT_OBJECT_NAMECHANGE: UINT = 0x800C;
pub const EVENT_OBJECT_DESCRIPTIONCHANGE: UINT = 0x800D;
pub const EVENT_OBJECT_VALUECHANGE: UINT = 0x800E;
pub const EVENT_OBJECT_PARENTCHANGE: UINT = 0x800F;
pub const EVENT_OBJECT_HELPCHANGE: UINT = 0x8010;
pub const EVENT_OBJECT_DEFACTIONCHANGE: UINT = 0x8011;
pub const EVENT_OBJECT_ACCELERATORCHANGE: UINT = 0x8012;
pub const EVENT_OBJECT_INVOKED: UINT = 0x8013;
pub const EVENT_OBJECT_TEXTSELECTIONCHANGED: UINT = 0x8014;
pub const EVENT_OBJECT_CONTENTSCROLLED: UINT = 0x8015;
pub const EVENT_SYSTEM_ARRANGMENTPREVIEW: UINT = 0x8016;
pub const EVENT_OBJECT_CLOAKED: UINT = 0x8017;
pub const EVENT_OBJECT_UNCLOAKED: UINT = 0x8018;
pub const EVENT_OBJECT_LIVEREGIONCHANGED: UINT = 0x8019;
pub const EVENT_OBJECT_HOSTEDOBJECTSINVALIDATED: UINT = 0x8020;
pub const EVENT_OBJECT_DRAGSTART: UINT = 0x8021;
pub const EVENT_OBJECT_DRAGCANCEL: UINT = 0x8022;
pub const EVENT_OBJECT_DRAGCOMPLETE: UINT = 0x8023;
pub const EVENT_OBJECT_DRAGENTER: UINT = 0x8024;
pub const EVENT_OBJECT_DRAGLEAVE: UINT = 0x8025;
pub const EVENT_OBJECT_DRAGDROPPED: UINT = 0x8026;
pub const EVENT_OBJECT_IME_SHOW: UINT = 0x8027;
pub const EVENT_OBJECT_IME_HIDE: UINT = 0x8028;
pub const EVENT_OBJECT_IME_CHANGE: UINT = 0x8029;
pub const EVENT_OBJECT_TEXTEDIT_CONVERSIONTARGETCHANGED: UINT = 0x8030;
pub const EVENT_OBJECT_END: UINT = 0x80FF;
pub const EVENT_AIA_START: UINT = 0xA000;
pub const EVENT_AIA_END: UINT = 0xAFFF;
pub const ALERT_SYSTEM_INFORMATIONAL: LONG = 1;
pub const ALERT_SYSTEM_WARNING: LONG = 2;
pub const ALERT_SYSTEM_ERROR: LONG = 3;
pub const ALERT_SYSTEM_QUERY: LONG = 4;
pub const ALERT_SYSTEM_CRITICAL: LONG = 5;
pub const CALERT_SYSTEM: LONG = 6;
STRUCT!{struct GUITHREADINFO {
    cbSize: DWORD,
    flags: DWORD,
    hwndActive: HWND,
    hwndFocus: HWND,
    hwndCapture: HWND,
    hwndMenuOwner: HWND,
    hwndMoveSize: HWND,
    hwndCaret: HWND,
    rcCaret: RECT,
}}
pub type PGUITHREADINFO = *mut GUITHREADINFO;
pub type LPGUITHREADINFO = *mut GUITHREADINFO;
pub const GUI_CARETBLINKING: DWORD = 0x00000001;
pub const GUI_INMOVESIZE: DWORD = 0x00000002;
pub const GUI_INMENUMODE: DWORD = 0x00000004;
pub const GUI_SYSTEMMENUMODE: DWORD = 0x00000008;
pub const GUI_POPUPMENUMODE: DWORD = 0x00000010;
#[cfg(target_arch = "x86_64")]
pub const GUI_16BITTASK: DWORD = 0x00000000;
#[cfg(target_arch = "x86")]
pub const GUI_16BITTASK: DWORD = 0x00000020;
extern "system" {
    pub fn GetGUIThreadInfo(
        idThread: DWORD,
        pgui: PGUITHREADINFO,
    ) -> BOOL;
    pub fn BlockInput(
        fBlockIt: BOOL,
    ) -> BOOL;
}
pub const USER_DEFAULT_SCREEN_DPI: LONG = 96;
extern "system" {
    pub fn SetProcessDPIAware() -> BOOL;
    pub fn IsProcessDPIAware() -> BOOL;
    pub fn SetThreadDpiAwarenessContext(
        dpiContext: DPI_AWARENESS_CONTEXT,
    ) -> DPI_AWARENESS_CONTEXT;
    pub fn GetThreadDpiAwarenessContext() -> DPI_AWARENESS_CONTEXT;
    pub fn GetWindowDpiAwarenessContext(
        hwnd: HWND,
    ) -> DPI_AWARENESS_CONTEXT;
    pub fn GetAwarenessFromDpiAwarenessContext(
        value: DPI_AWARENESS_CONTEXT,
    ) -> DPI_AWARENESS;
    pub fn GetDpiFromDpiAwarenessContext(
        value: DPI_AWARENESS_CONTEXT,
    ) -> UINT;
    pub fn AreDpiAwarenessContextsEqual(
        dpiContextA: DPI_AWARENESS_CONTEXT,
        dpiContextB: DPI_AWARENESS_CONTEXT,
    ) -> BOOL;
    pub fn IsValidDpiAwarenessContext(
        value: DPI_AWARENESS_CONTEXT,
    ) -> BOOL;
    pub fn GetDpiForWindow(
        hwnd: HWND,
    ) -> UINT;
    pub fn GetDpiForSystem() -> UINT;
    pub fn GetSystemDpiForProcess(
        hProcess: HANDLE,
    ) -> UINT;
    pub fn EnableNonClientDpiScaling(
        hwnd: HWND,
    ) -> BOOL;
    pub fn SetProcessDpiAwarenessContext(
        value: DPI_AWARENESS_CONTEXT,
    ) -> BOOL;
    pub fn SetThreadDpiHostingBehavior(
        value: DPI_HOSTING_BEHAVIOR,
    ) -> DPI_HOSTING_BEHAVIOR;
    pub fn GetThreadDpiHostingBehavior() -> DPI_HOSTING_BEHAVIOR;
    pub fn GetWindowDpiHostingBehavior(
        hwnd: HWND,
    ) -> DPI_HOSTING_BEHAVIOR;
    pub fn GetWindowModuleFileNameA(
        hWnd: HWND,
        lpszFileName: LPCSTR,
        cchFileNameMax: UINT,
    ) -> UINT;
    pub fn GetWindowModuleFileNameW(
        hWnd: HWND,
        lpszFileName: LPWSTR,
        cchFileNameMax: UINT,
    ) -> UINT;
}
pub const STATE_SYSTEM_UNAVAILABLE: DWORD = 0x00000001;
pub const STATE_SYSTEM_SELECTED: DWORD = 0x00000002;
pub const STATE_SYSTEM_FOCUSED: DWORD = 0x00000004;
pub const STATE_SYSTEM_PRESSED: DWORD = 0x00000008;
pub const STATE_SYSTEM_CHECKED: DWORD = 0x00000010;
pub const STATE_SYSTEM_MIXED: DWORD = 0x00000020;
pub const STATE_SYSTEM_INDETERMINATE: DWORD = STATE_SYSTEM_MIXED;
pub const STATE_SYSTEM_READONLY: DWORD = 0x00000040;
pub const STATE_SYSTEM_HOTTRACKED: DWORD = 0x00000080;
pub const STATE_SYSTEM_DEFAULT: DWORD = 0x00000100;
pub const STATE_SYSTEM_EXPANDED: DWORD = 0x00000200;
pub const STATE_SYSTEM_COLLAPSED: DWORD = 0x00000400;
pub const STATE_SYSTEM_BUSY: DWORD = 0x00000800;
pub const STATE_SYSTEM_FLOATING: DWORD = 0x00001000;
pub const STATE_SYSTEM_MARQUEED: DWORD = 0x00002000;
pub const STATE_SYSTEM_ANIMATED: DWORD = 0x00004000;
pub const STATE_SYSTEM_INVISIBLE: DWORD = 0x00008000;
pub const STATE_SYSTEM_OFFSCREEN: DWORD = 0x00010000;
pub const STATE_SYSTEM_SIZEABLE: DWORD = 0x00020000;
pub const STATE_SYSTEM_MOVEABLE: DWORD = 0x00040000;
pub const STATE_SYSTEM_SELFVOICING: DWORD = 0x00080000;
pub const STATE_SYSTEM_FOCUSABLE: DWORD = 0x00100000;
pub const STATE_SYSTEM_SELECTABLE: DWORD = 0x00200000;
pub const STATE_SYSTEM_LINKED: DWORD = 0x00400000;
pub const STATE_SYSTEM_TRAVERSED: DWORD = 0x00800000;
pub const STATE_SYSTEM_MULTISELECTABLE: DWORD = 0x01000000;
pub const STATE_SYSTEM_EXTSELECTABLE: DWORD = 0x02000000;
pub const STATE_SYSTEM_ALERT_LOW: DWORD = 0x04000000;
pub const STATE_SYSTEM_ALERT_MEDIUM: DWORD = 0x08000000;
pub const STATE_SYSTEM_ALERT_HIGH: DWORD = 0x10000000;
pub const STATE_SYSTEM_PROTECTED: DWORD = 0x20000000;
pub const STATE_SYSTEM_VALID: DWORD = 0x3fffffff;
pub const CCHILDREN_TITLEBAR: usize = 5;
pub const CCHILDREN_SCROLLBAR: usize = 5;
STRUCT!{struct CURSORINFO {
    cbSize: DWORD,
    flags: DWORD,
    hCursor: HCURSOR,
    ptScreenPos: POINT,
}}
pub type PCURSORINFO = *mut CURSORINFO;
pub type LPCURSORINFO = *mut CURSORINFO;
pub const CURSOR_SHOWING: DWORD = 0x00000001;
pub const CURSOR_SUPPRESSED: DWORD = 0x00000002;
extern "system" {
    pub fn GetCursorInfo(
        pci: PCURSORINFO,
    ) -> BOOL;
}
STRUCT!{struct WINDOWINFO {
    cbSize: DWORD,
    rcWindow: RECT,
    rcClient: RECT,
    dwStyle: DWORD,
    dwExStyle: DWORD,
    dwWindowStatus: DWORD,
    cxWindowBorders: UINT,
    cyWindowBorders: UINT,
    atomWindowType: ATOM,
    wCreatorVersion: WORD,
}}
pub type PWINDOWINFO = *mut WINDOWINFO;
pub type LPWINDOWINFO = *mut WINDOWINFO;
pub const WS_ACTIVECAPTION: DWORD = 0x0001;
extern "system" {
    pub fn GetWindowInfo(
        hwnd: HWND,
        pwi: PWINDOWINFO,
    ) -> BOOL;
}
STRUCT!{struct TITLEBARINFO {
    cbSize: DWORD,
    rcTitleBar: RECT,
    rgstate: [DWORD; CCHILDREN_TITLEBAR + 1],
}}
pub type PTITLEBARINFO = *mut TITLEBARINFO;
pub type LPTITLEBARINFO = *mut TITLEBARINFO;
extern "system" {
    pub fn GetTitleBarInfo(
        hwnd: HWND,
        pti: PTITLEBARINFO,
    ) -> BOOL;
}
STRUCT!{struct TITLEBARINFOEX {
    cbSize: DWORD,
    rcTitleBar: RECT,
    rgstate: [DWORD; CCHILDREN_TITLEBAR + 1],
    rgrect: [RECT; CCHILDREN_TITLEBAR + 1],
}}
pub type PTITLEBARINFOEX = *mut TITLEBARINFOEX;
pub type LPTITLEBARINFOEX = *mut TITLEBARINFOEX;
STRUCT!{struct MENUBARINFO {
    cbSize: DWORD,
    rcBar: RECT,
    hMenu: HMENU,
    hwndMenu: HWND,
    BitFields: BOOL,
}}
BITFIELD!{MENUBARINFO BitFields: BOOL [
    fBarFocused set_fBarFocused[0..1],
    fFocused set_fFocused[1..2],
]}
pub type PMENUBARINFO = *mut MENUBARINFO;
pub type LPMENUBARINFO = *mut MENUBARINFO;
extern "system" {
    pub fn GetMenuBarInfo(
        hwnd: HWND,
        idObject: LONG,
        idItem: LONG,
        pmbi: PMENUBARINFO,
    ) -> BOOL;
}
STRUCT!{struct SCROLLBARINFO {
    cbSize: DWORD,
    rcScrollBar: RECT,
    dxyLineButton: c_int,
    xyThumbTop: c_int,
    xyThumbBottom: c_int,
    reserved: c_int,
    rgstate: [DWORD; CCHILDREN_SCROLLBAR + 1],
}}
pub type PSCROLLBARINFO = *mut SCROLLBARINFO;
pub type LPSCROLLBARINFO = *mut SCROLLBARINFO;
extern "system" {
    pub fn GetScrollBarInfo(
        hwnd: HWND,
        idObject: LONG,
        psbi: PSCROLLBARINFO,
    ) -> BOOL;
}
STRUCT!{struct COMBOBOXINFO {
    cbSize: DWORD,
    rcItem: RECT,
    rcButton: RECT,
    stateButton: DWORD,
    hwndCombo: HWND,
    hwndItem: HWND,
    hwndList: HWND,
}}
pub type PCOMBOBOXINFO = *mut COMBOBOXINFO;
pub type LPCOMBOBOXINFO = *mut COMBOBOXINFO;
extern "system" {
    pub fn GetComboBoxInfo(
        hwndCombo: HWND,
        pcbi: PCOMBOBOXINFO,
    ) -> BOOL;
}
pub const GA_PARENT: UINT = 1;
pub const GA_ROOT: UINT = 2;
pub const GA_ROOTOWNER: UINT = 3;
extern "system" {
    pub fn GetAncestor(
        hwnd: HWND,
        gaFlags: UINT,
    ) -> HWND;
    pub fn RealChildWindowFromPoint(
        hwndParent: HWND,
        ptParentClientCoords: POINT,
    ) -> HWND;
    pub fn RealGetWindowClassA(
        hwnd: HWND,
        ptszClassName: LPSTR,
        cchClassNameMax: UINT,
    ) -> UINT;
    pub fn RealGetWindowClassW(
        hwnd: HWND,
        ptszClassName: LPWSTR,
        cchClassNameMax: UINT,
    ) -> UINT;
}
STRUCT!{struct ALTTABINFO {
    cbSize: DWORD,
    cItems: c_int,
    cColumns: c_int,
    cRows: c_int,
    iColFocus: c_int,
    iRowFocus: c_int,
    cxItem: c_int,
    cyItem: c_int,
    ptStart: POINT,
}}
pub type PALTTABINFO = *mut ALTTABINFO;
pub type LPALTTABINFO = *mut ALTTABINFO;
extern "system" {
    pub fn GetAltTabInfoA(
        hwnd: HWND,
        iItem: c_int,
        pati: PALTTABINFO,
        pszItemText: LPSTR,
        cchItemText: UINT,
    ) -> BOOL;
    pub fn GetAltTabInfoW(
        hwnd: HWND,
        iItem: c_int,
        pati: PALTTABINFO,
        pszItemText: LPWSTR,
        cchItemText: UINT,
    ) -> BOOL;
    pub fn GetListBoxInfo(
        hwnd: HWND,
    ) -> DWORD;
    pub fn LockWorkStation() -> BOOL;
    pub fn UserHandleGrantAccess(
        hUserHandle: HANDLE,
        hJob: HANDLE,
        bGrant: BOOL,
    ) -> BOOL;
}
DECLARE_HANDLE!{HRAWINPUT, HRAWINPUT__}
#[inline]
pub fn GET_RAWINPUT_CODE_WPARAM(wParam: WPARAM) -> WPARAM { wParam & 0xff }
pub const RIM_INPUT: WPARAM = 0;
pub const RIM_INPUTSINK: WPARAM = 1;
STRUCT!{struct RAWINPUTHEADER {
    dwType: DWORD,
    dwSize: DWORD,
    hDevice: HANDLE,
    wParam: WPARAM,
}}
pub type PRAWINPUTHEADER = *mut RAWINPUTHEADER;
pub type LPRAWINPUTHEADER = *mut RAWINPUTHEADER;
pub const RIM_TYPEMOUSE: DWORD = 0;
pub const RIM_TYPEKEYBOARD: DWORD = 1;
pub const RIM_TYPEHID: DWORD = 2;
STRUCT!{struct RAWMOUSE {
    usFlags: USHORT,
    memory_padding: USHORT, // 16bit Padding for 32bit align in following union
    usButtonFlags: USHORT,
    usButtonData: USHORT,
    ulRawButtons: ULONG,
    lLastX: LONG,
    lLastY: LONG,
    ulExtraInformation: ULONG,
}}
pub type PRAWMOUSE = *mut RAWMOUSE;
pub type LPRAWMOUSE = *mut RAWMOUSE;
pub const RI_MOUSE_LEFT_BUTTON_DOWN: USHORT = 0x0001;
pub const RI_MOUSE_LEFT_BUTTON_UP: USHORT = 0x0002;
pub const RI_MOUSE_RIGHT_BUTTON_DOWN: USHORT = 0x0004;
pub const RI_MOUSE_RIGHT_BUTTON_UP: USHORT = 0x0008;
pub const RI_MOUSE_MIDDLE_BUTTON_DOWN: USHORT = 0x0010;
pub const RI_MOUSE_MIDDLE_BUTTON_UP: USHORT = 0x0020;
pub const RI_MOUSE_BUTTON_1_DOWN: USHORT = RI_MOUSE_LEFT_BUTTON_DOWN;
pub const RI_MOUSE_BUTTON_1_UP: USHORT = RI_MOUSE_LEFT_BUTTON_UP;
pub const RI_MOUSE_BUTTON_2_DOWN: USHORT = RI_MOUSE_RIGHT_BUTTON_DOWN;
pub const RI_MOUSE_BUTTON_2_UP: USHORT = RI_MOUSE_RIGHT_BUTTON_UP;
pub const RI_MOUSE_BUTTON_3_DOWN: USHORT = RI_MOUSE_MIDDLE_BUTTON_DOWN;
pub const RI_MOUSE_BUTTON_3_UP: USHORT = RI_MOUSE_MIDDLE_BUTTON_UP;
pub const RI_MOUSE_BUTTON_4_DOWN: USHORT = 0x0040;
pub const RI_MOUSE_BUTTON_4_UP: USHORT = 0x0080;
pub const RI_MOUSE_BUTTON_5_DOWN: USHORT = 0x0100;
pub const RI_MOUSE_BUTTON_5_UP: USHORT = 0x0200;
pub const RI_MOUSE_WHEEL: USHORT = 0x0400;
pub const MOUSE_MOVE_RELATIVE: USHORT = 0;
pub const MOUSE_MOVE_ABSOLUTE: USHORT = 1;
pub const MOUSE_VIRTUAL_DESKTOP: USHORT = 0x02;
pub const MOUSE_ATTRIBUTES_CHANGED: USHORT = 0x04;
pub const MOUSE_MOVE_NOCOALESCE: USHORT = 0x08;
STRUCT!{struct RAWKEYBOARD {
    MakeCode: USHORT,
    Flags: USHORT,
    Reserved: USHORT,
    VKey: USHORT,
    Message: UINT,
    ExtraInformation: ULONG,
}}
pub type PRAWKEYBOARD = *mut RAWKEYBOARD;
pub type LPRAWKEYBOARD = *mut RAWKEYBOARD;
pub const KEYBOARD_OVERRUN_MAKE_CODE: DWORD = 0xFF;
pub const RI_KEY_MAKE: DWORD = 0;
pub const RI_KEY_BREAK: DWORD = 1;
pub const RI_KEY_E0: DWORD = 2;
pub const RI_KEY_E1: DWORD = 4;
pub const RI_KEY_TERMSRV_SET_LED: DWORD = 8;
pub const RI_KEY_TERMSRV_SHADOW: DWORD = 0x10;
STRUCT!{struct RAWHID {
    dwSizeHid: DWORD,
    dwCount: DWORD,
    bRawData: [BYTE; 1],
}}
pub type PRAWHID = *mut RAWHID;
pub type LPRAWHID = *mut RAWHID;
UNION!{union RAWINPUT_data {
    [u32; 6],
    mouse mouse_mut: RAWMOUSE,
    keyboard keyboard_mut: RAWKEYBOARD,
    hid hid_mut: RAWHID,
}}
STRUCT!{struct RAWINPUT {
    header: RAWINPUTHEADER,
    data: RAWINPUT_data,
}}
pub type PRAWINPUT = *mut RAWINPUT;
pub type LPRAWINPUT = *mut RAWINPUT;
pub const RID_INPUT: DWORD = 0x10000003;
pub const RID_HEADER: DWORD = 0x10000005;
extern "system" {
    pub fn GetRawInputData(
        hRawInput: HRAWINPUT,
        uiCommand: UINT,
        pData: LPVOID,
        pcbSize: PUINT,
        cbSizeHeader: UINT,
    ) -> UINT;
}
pub const RIDI_PREPARSEDDATA: DWORD = 0x20000005;
pub const RIDI_DEVICENAME: DWORD = 0x20000007;
pub const RIDI_DEVICEINFO: DWORD = 0x2000000b;
STRUCT!{struct RID_DEVICE_INFO_MOUSE {
    dwId: DWORD,
    dwNumberOfButtons: DWORD,
    dwSampleRate: DWORD,
    fHasHorizontalWheel: BOOL,
}}
pub type PRID_DEVICE_INFO_MOUSE = *mut RID_DEVICE_INFO_MOUSE;
STRUCT!{struct RID_DEVICE_INFO_KEYBOARD {
    dwType: DWORD,
    dwSubType: DWORD,
    dwKeyboardMode: DWORD,
    dwNumberOfFunctionKeys: DWORD,
    dwNumberOfIndicators: DWORD,
    dwNumberOfKeysTotal: DWORD,
}}
pub type PRID_DEVICE_INFO_KEYBOARD = *mut RID_DEVICE_INFO_KEYBOARD;
STRUCT!{struct RID_DEVICE_INFO_HID {
    dwVendorId: DWORD,
    dwProductId: DWORD,
    dwVersionNumber: DWORD,
    usUsagePage: USHORT,
    usUsage: USHORT,
}}
pub type PRID_DEVICE_INFO_HID = *mut RID_DEVICE_INFO_HID;
UNION!{union RID_DEVICE_INFO_u {
    [u32; 6],
    mouse mouse_mut: RID_DEVICE_INFO_MOUSE,
    keyboard keyboard_mut: RID_DEVICE_INFO_KEYBOARD,
    hid hid_mut: RID_DEVICE_INFO_HID,
}}
STRUCT!{struct RID_DEVICE_INFO {
    cbSize: DWORD,
    dwType: DWORD,
    u: RID_DEVICE_INFO_u,
}}
pub type PRID_DEVICE_INFO = *mut RID_DEVICE_INFO;
pub type LPRID_DEVICE_INFO = *mut RID_DEVICE_INFO;
extern "system" {
    pub fn GetRawInputDeviceInfoA(
        hDevice: HANDLE,
        uiCommand: UINT,
        pData: LPVOID,
        pcbSize: PUINT,
    ) -> UINT;
    pub fn GetRawInputDeviceInfoW(
        hDevice: HANDLE,
        uiCommand: UINT,
        pData: LPVOID,
        pcbSize: PUINT,
    ) -> UINT;
    pub fn GetRawInputBuffer(
        pData: PRAWINPUT,
        pcbSize: PUINT,
        cbSizeHeader: UINT,
    ) -> UINT;
}
STRUCT!{struct RAWINPUTDEVICE {
    usUsagePage: USHORT,
    usUsage: USHORT,
    dwFlags: DWORD,
    hwndTarget: HWND,
}}
pub type PRAWINPUTDEVICE = *mut RAWINPUTDEVICE;
pub type LPRAWINPUTDEVICE = *mut RAWINPUTDEVICE;
pub type PCRAWINPUTDEVICE = *const RAWINPUTDEVICE;
pub const RIDEV_REMOVE: DWORD = 0x00000001;
pub const RIDEV_EXCLUDE: DWORD = 0x00000010;
pub const RIDEV_PAGEONLY: DWORD = 0x00000020;
pub const RIDEV_NOLEGACY: DWORD = 0x00000030;
pub const RIDEV_INPUTSINK: DWORD = 0x00000100;
pub const RIDEV_CAPTUREMOUSE: DWORD = 0x00000200;
pub const RIDEV_NOHOTKEYS: DWORD = 0x00000200;
pub const RIDEV_APPKEYS: DWORD = 0x00000400;
pub const RIDEV_EXINPUTSINK: DWORD = 0x00001000;
pub const RIDEV_DEVNOTIFY: DWORD = 0x00002000;
pub const RIDEV_EXMODEMASK: DWORD = 0x000000F0;
pub const GIDC_ARRIVAL: DWORD = 1;
pub const GIDC_REMOVAL: DWORD = 2;
extern "system" {
    pub fn RegisterRawInputDevices(
        pRawInputDevices: PCRAWINPUTDEVICE,
        uiNumDevices: UINT,
        cbSize: UINT,
    ) -> BOOL;
    pub fn GetRegisteredRawInputDevices(
        pRawInputDevices: PRAWINPUTDEVICE,
        puiNumDevices: PUINT,
        cbSize: UINT,
    ) -> UINT;
}
STRUCT!{struct RAWINPUTDEVICELIST {
    hDevice: HANDLE,
    dwType: DWORD,
}}
pub type PRAWINPUTDEVICELIST = *mut RAWINPUTDEVICELIST;
extern "system" {
    pub fn GetRawInputDeviceList(
        pRawInputDeviceList: PRAWINPUTDEVICELIST,
        puiNumDevices: PUINT,
        cbSize: UINT,
    ) -> UINT;
    pub fn DefRawInputProc(
        paRawInput: *mut PRAWINPUT,
        nInput: INT,
        cbSizeHeader: UINT,
    ) -> LRESULT;
    pub fn ChangeWindowMessageFilter(
        message: UINT,
        dwFlag: DWORD,
    ) -> BOOL;
}
//15165
STRUCT!{struct CHANGEFILTERSTRUCT {
    cbSize: DWORD,
    ExtStatus: DWORD,
}}
extern "system" {
    pub fn ChangeWindowMessageFilterEx(
        hwnd: HWND,
        message: UINT,
        action: DWORD,
        pChangeFilterStruct: PCHANGEFILTERSTRUCT,
    ) -> BOOL;
}
pub type PCHANGEFILTERSTRUCT = *mut CHANGEFILTERSTRUCT;
//15427
pub const NID_INTEGRATED_TOUCH: UINT = 0x00000001;
pub const NID_EXTERNAL_TOUCH: UINT = 0x00000002;
pub const NID_INTEGRATED_PEN: UINT = 0x00000004;
pub const NID_EXTERNAL_PEN: UINT = 0x00000008;
pub const NID_MULTI_INPUT: UINT = 0x00000040;
pub const NID_READY: UINT = 0x00000080;
pub const MAX_STR_BLOCKREASON: usize = 256;
extern "system" {
    pub fn ShutdownBlockReasonCreate(
        hWnd: HWND,
        pwszReason: LPCWSTR,
    ) -> BOOL;
    pub fn ShutdownBlockReasonQuery(
        hWnd: HWND,
        pwszBuff: LPWSTR,
        pcchBuff: *mut DWORD,
    ) -> BOOL;
    pub fn ShutdownBlockReasonDestroy(
        hWnd: HWND,
    ) -> BOOL;
}
//15615
extern "system" {
    pub fn IsImmersiveProcess(
        hProcess: HANDLE,
    ) -> BOOL;
}
