// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_uint;
use shared::minwindef::{BOOL, DWORD, UINT};
use shared::windef::{HWND, POINT, RECT};
pub type LPUINT = *mut c_uint;
STRUCT!{struct COMPOSITIONFORM {
    dwStyle: DWORD,
    ptCurrentPos: POINT,
    rcArea: RECT,
}}
DECLARE_HANDLE!{HIMC, HIMC__}
pub type LPCOMPOSITIONFORM = *mut COMPOSITIONFORM;
extern "system" {
    pub fn ImmGetContext(
        hwnd: HWND,
    ) -> HIMC;
    pub fn ImmGetOpenStatus(
        himc: HIMC,
    ) -> BOOL;
    pub fn ImmSetOpenStatus(
        himc: HIMC,
        fopen: BOOL,
    ) -> BOOL;
    pub fn ImmSetCompositionWindow(
        himc: HIMC,
        lpCompForm: LPCOMPOSITIONFORM,
    ) -> BOOL;
    pub fn ImmReleaseContext(
        hwnd: HWND,
        himc: HIMC,
    ) -> BOOL;
}
pub const CFS_DEFAULT: UINT = 0x0000;
pub const CFS_RECT: UINT = 0x0001;
pub const CFS_POINT: UINT = 0x0002;
pub const CFS_FORCE_POSITION: UINT = 0x0020;
pub const CFS_CANDIDATEPOS: UINT = 0x0040;
pub const CFS_EXCLUDE: UINT = 0x0080;
