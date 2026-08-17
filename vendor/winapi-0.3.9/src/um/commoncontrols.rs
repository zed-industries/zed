// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_int, c_void};
use shared::guiddef::{REFCLSID, REFIID};
use shared::minwindef::{BOOL, DWORD, LRESULT, UINT};
use shared::windef::{COLORREF, HBITMAP, HICON, HWND, POINT, RECT};
use um::commctrl::{IMAGEINFO, IMAGELISTDRAWPARAMS};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::HRESULT;
extern "system" {
    pub fn ImageList_CoCreateInstance(
        rclsid: REFCLSID,
        punkOuter: *const IUnknown,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
}
pub const ILIF_ALPHA: DWORD = 0x00000001;
pub const ILIF_LOWQUALITY: DWORD = 0x00000001;
pub const ILDRF_IMAGELOWQUALITY: LRESULT = 0x00000001;
pub const ILDRF_OVERLAYLOWQUALITY: LRESULT = 0x00000010;
RIDL!{#[uuid(0x46eb5926, 0x582e, 0x4017, 0x9f, 0xdf, 0xe8, 0x99, 0x8d, 0xaa, 0x09, 0x50)]
interface IImageList(IImageListVtbl): IUnknown(IUnknownVtbl) {
    fn Add(
        hbmImage: HBITMAP,
        hbmMask: HBITMAP,
        pi: *mut c_int,
    ) -> HRESULT,
    fn ReplaceIcon(
        hicon: HICON,
        pi: *mut c_int,
    ) -> HRESULT,
    fn SetOverlayImage(
        iImage: c_int,
        iOverlay: c_int,
    ) -> HRESULT,
    fn Replace(
        hbmImage: HBITMAP,
        hbmMask: HBITMAP,
    ) -> HRESULT,
    fn AddMasked(
        hbmImage: HBITMAP,
        crMask: COLORREF,
        pi: *mut c_int,
    ) -> HRESULT,
    fn Draw(
        pimldp: *mut IMAGELISTDRAWPARAMS,
    ) -> HRESULT,
    fn Remove(
        i: c_int,
    ) -> HRESULT,
    fn GetIcon(
        i: c_int,
        flags: UINT,
        picon: *mut HICON,
    ) -> HRESULT,
    fn GetImageInfo(
        i: c_int,
        pImageInfo: *mut IMAGEINFO,
    ) -> HRESULT,
    fn Copy(
        iDst: c_int,
        punkSrc: *mut IUnknown,
        iSrc: c_int,
        uFlags: UINT,
    ) -> HRESULT,
    fn Merge(
        i1: c_int,
        punk2: *mut IUnknown,
        i2: c_int,
        dx: c_int,
        dy: c_int,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn Clone(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetImageRect(
        i: c_int,
        prc: *mut RECT,
    ) -> HRESULT,
    fn GetIconSize(
        cx: *mut c_int,
        cy: *mut c_int,
    ) -> HRESULT,
    fn SetIconSize(
        cx: c_int,
        cy: c_int,
    ) -> HRESULT,
    fn GetImageCount(
        pi: *mut c_int,
    ) -> HRESULT,
    fn SetImageCount(
        uNewCount: UINT,
    ) -> HRESULT,
    fn SetBkColor(
        clrBk: COLORREF,
        pclr: *mut COLORREF,
    ) -> HRESULT,
    fn GetBkColor(
        pclr: *mut COLORREF,
    ) -> HRESULT,
    fn BeginDrag(
        iTrack: c_int,
        dxHotspot: c_int,
        dyHotspot: c_int,
    ) -> HRESULT,
    fn EndDrag() -> HRESULT,
    fn DragEnter(
        hwndLock: HWND,
        x: c_int,
        y: c_int,
    ) -> HRESULT,
    fn DragLeave(
        hwndLock: HWND,
    ) -> HRESULT,
    fn DragMove(
        x: c_int,
        y: c_int,
    ) -> HRESULT,
    fn SetDragCursorImage(
        punk: *mut IUnknown,
        iDrag: c_int,
        dxHotspot: c_int,
        dyHotspot: c_int,
    ) -> HRESULT,
    fn DragShowNolock(
        fShow: BOOL,
    ) -> HRESULT,
    fn GetDragImage(
        ppt: *mut POINT,
        pptHotspot: *mut POINT,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetItemFlags(
        dwFlags: *mut DWORD,
    ) -> HRESULT,
    fn GetOverlayImage(
        iOverlay: c_int,
        piIndex: *mut c_int,
    ) -> HRESULT,
}}
pub const ILR_DEFAULT: DWORD = 0x0000;
pub const ILR_HORIZONTAL_LEFT: DWORD = 0x0000;
pub const ILR_HORIZONTAL_CENTER: DWORD = 0x0001;
pub const ILR_HORIZONTAL_RIGHT: DWORD = 0x0002;
pub const ILR_VERTICAL_TOP: DWORD = 0x0000;
pub const ILR_VERTICAL_CENTER: DWORD = 0x0010;
pub const ILR_VERTICAL_BOTTOM: DWORD = 0x0020;
pub const ILR_SCALE_CLIP: DWORD = 0x0000;
pub const ILR_SCALE_ASPECTRATIO: DWORD = 0x0100;
pub const ILGOS_ALWAYS: DWORD = 0x00000000;
pub const ILGOS_FROMSTANDBY: DWORD = 0x00000001;
pub const ILFIP_ALWAYS: DWORD = 0x00000000;
pub const ILFIP_FROMSTANDBY: DWORD = 0x00000001;
pub const ILDI_PURGE: DWORD = 0x00000001;
pub const ILDI_STANDBY: DWORD = 0x00000002;
pub const ILDI_RESETACCESS: DWORD = 0x00000004;
pub const ILDI_QUERYACCESS: DWORD = 0x00000008;
STRUCT!{struct IMAGELISTSTATS {
    cbSize: DWORD,
    cAlloc: c_int,
    cUsed: c_int,
    cStandby: c_int,
}}
RIDL!{#[uuid(0x192b9d83, 0x58fc, 0x457b, 0x90, 0xa0, 0x2b, 0x82, 0xa8, 0xb5, 0xda, 0xe1)]
interface IImageList2(IImageList2Vtbl): IImageList(IImageListVtbl) {
    fn Resize(
        cxNewIconSize: c_int,
        cyNewIconSize: c_int,
    ) -> HRESULT,
    fn GetOriginalSize(
        iImage: c_int,
        dwFlags: DWORD,
        pcx: *mut c_int,
        pcy: *mut c_int,
    ) -> HRESULT,
    fn SetOriginalSize(
        iImage: c_int,
        cx: c_int,
        cy: c_int,
    ) -> HRESULT,
    fn SetCallback(
        punk: *mut IUnknown,
    ) -> HRESULT,
    fn GetCallback(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn ForceImagePresent(
        iImage: c_int,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn DiscardImages(
        iFirstImage: c_int,
        iLastImage: c_int,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn PreloadImages(
        pimldp: *mut IMAGELISTDRAWPARAMS,
    ) -> HRESULT,
    fn GetStatistics(
        pils: *mut IMAGELISTSTATS,
    ) -> HRESULT,
    fn Initialize(
        cx: c_int,
        cy: c_int,
        flags: UINT,
        cInitial: c_int,
        cGrow: c_int,
    ) -> HRESULT,
    fn Replace2(
        i: c_int,
        hbmImage: HBITMAP,
        hbmMask: HBITMAP,
        punk: *mut IUnknown,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn ReplaceFromImageList(
        i: c_int,
        pil: *mut IImageList,
        iSrc: c_int,
        punk: *mut IUnknown,
        dwFlags: DWORD,
    ) -> HRESULT,
}}
