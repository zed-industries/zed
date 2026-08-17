// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Basic Windows Type Definitions
use ctypes::c_void;
use shared::minwindef::{DWORD, HFILE, WORD};
use um::winnt::{LONG, SHORT};
DECLARE_HANDLE!{HWND, HWND__}
DECLARE_HANDLE!{HHOOK, HHOOK__}
pub type HGDIOBJ = *mut c_void;
DECLARE_HANDLE!{HACCEL, HACCEL__}
DECLARE_HANDLE!{HBITMAP, HBITMAP__}
DECLARE_HANDLE!{HBRUSH, HBRUSH__}
DECLARE_HANDLE!{HCOLORSPACE, HCOLORSPACE__}
DECLARE_HANDLE!{HDC, HDC__}
DECLARE_HANDLE!{HGLRC, HGLRC__}
DECLARE_HANDLE!{HDESK, HDESK__}
DECLARE_HANDLE!{HENHMETAFILE, HENHMETAFILE__}
DECLARE_HANDLE!{HFONT, HFONT__}
DECLARE_HANDLE!{HICON, HICON__}
DECLARE_HANDLE!{HMENU, HMENU__}
DECLARE_HANDLE!{HPALETTE, HPALETTE__}
DECLARE_HANDLE!{HPEN, HPEN__}
DECLARE_HANDLE!{HWINEVENTHOOK, HWINEVENTHOOK__}
DECLARE_HANDLE!{HMONITOR, HMONITOR__}
DECLARE_HANDLE!{HUMPD, HUMPD__}
pub type HCURSOR = HICON;
pub type COLORREF = DWORD;
pub type LPCOLORREF = *mut DWORD;
pub const HFILE_ERROR: HFILE = -1;
STRUCT!{#[debug] struct RECT {
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG,
}}
pub type PRECT = *mut RECT;
pub type NPRECT = *mut RECT;
pub type LPRECT = *mut RECT;
pub type LPCRECT = *const RECT;
STRUCT!{#[debug] struct RECTL {
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG,
}}
pub type PRECTL = *mut RECTL;
pub type LPRECTL = *mut RECTL;
pub type LPCRECTL = *const RECTL;
STRUCT!{struct POINT {
    x: LONG,
    y: LONG,
}}
pub type PPOINT = *mut POINT;
pub type NPPOINT = *mut POINT;
pub type LPPOINT = *mut POINT;
STRUCT!{struct POINTL {
    x: LONG,
    y: LONG,
}}
pub type PPOINTL = *mut POINTL;
STRUCT!{struct SIZE {
    cx: LONG,
    cy: LONG,
}}
pub type PSIZE = *mut SIZE;
pub type LPSIZE = *mut SIZE;
pub type SIZEL = SIZE;
pub type PSIZEL = *mut SIZE;
pub type LPSIZEL = *mut SIZE;
STRUCT!{struct POINTS {
    x: SHORT,
    y: SHORT,
}}
pub type PPOINTS = *mut POINTS;
pub type LPPOINTS = *mut POINTS;
pub const DM_UPDATE: WORD = 1;
pub const DM_COPY: WORD = 2;
pub const DM_PROMPT: WORD = 4;
pub const DM_MODIFY: WORD = 8;
pub const DM_IN_BUFFER: WORD = DM_MODIFY;
pub const DM_IN_PROMPT: WORD = DM_PROMPT;
pub const DM_OUT_BUFFER: WORD = DM_COPY;
pub const DM_OUT_DEFAULT: WORD = DM_UPDATE;
pub const DC_FIELDS: DWORD = 1;
pub const DC_PAPERS: DWORD = 2;
pub const DC_PAPERSIZE: DWORD = 3;
pub const DC_MINEXTENT: DWORD = 4;
pub const DC_MAXEXTENT: DWORD = 5;
pub const DC_BINS: DWORD = 6;
pub const DC_DUPLEX: DWORD = 7;
pub const DC_SIZE: DWORD = 8;
pub const DC_EXTRA: DWORD = 9;
pub const DC_VERSION: DWORD = 10;
pub const DC_DRIVER: DWORD = 11;
pub const DC_BINNAMES: DWORD = 12;
pub const DC_ENUMRESOLUTIONS: DWORD = 13;
pub const DC_FILEDEPENDENCIES: DWORD = 14;
pub const DC_TRUETYPE: DWORD = 15;
pub const DC_PAPERNAMES: DWORD = 16;
pub const DC_ORIENTATION: DWORD = 17;
pub const DC_COPIES: DWORD = 18;
DECLARE_HANDLE!{DPI_AWARENESS_CONTEXT, DPI_AWARENESS_CONTEXT__}
ENUM!{enum DPI_AWARENESS {
    DPI_AWARENESS_INVALID = -1i32 as u32,
    DPI_AWARENESS_UNAWARE = 0,
    DPI_AWARENESS_SYSTEM_AWARE = 1,
    DPI_AWARENESS_PER_MONITOR_AWARE = 2,
}}
pub const DPI_AWARENESS_CONTEXT_UNAWARE: DPI_AWARENESS_CONTEXT = -1isize as DPI_AWARENESS_CONTEXT;
pub const DPI_AWARENESS_CONTEXT_SYSTEM_AWARE: DPI_AWARENESS_CONTEXT
    = -2isize as DPI_AWARENESS_CONTEXT;
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE: DPI_AWARENESS_CONTEXT
    = -3isize as DPI_AWARENESS_CONTEXT;
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT
    = -4isize as DPI_AWARENESS_CONTEXT;
pub const DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED: DPI_AWARENESS_CONTEXT
    = -5isize as DPI_AWARENESS_CONTEXT;
ENUM!{enum DPI_HOSTING_BEHAVIOR {
    DPI_HOSTING_BEHAVIOR_INVALID = -1i32 as u32,
    DPI_HOSTING_BEHAVIOR_DEFAULT = 0,
    DPI_HOSTING_BEHAVIOR_MIXED = 1,
}}
