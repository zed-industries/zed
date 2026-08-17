#[inline]
pub unsafe fn MagGetColorEffect(hwnd: super::super::Foundation::HWND, peffect: *mut MAGCOLOREFFECT) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagGetColorEffect(hwnd : super::super::Foundation:: HWND, peffect : *mut MAGCOLOREFFECT) -> windows_core::BOOL);
    unsafe { MagGetColorEffect(hwnd, peffect as _) }
}
#[inline]
pub unsafe fn MagGetFullscreenColorEffect(peffect: *mut MAGCOLOREFFECT) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagGetFullscreenColorEffect(peffect : *mut MAGCOLOREFFECT) -> windows_core::BOOL);
    unsafe { MagGetFullscreenColorEffect(peffect as _) }
}
#[inline]
pub unsafe fn MagGetFullscreenTransform(pmaglevel: *mut f32, pxoffset: *mut i32, pyoffset: *mut i32) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagGetFullscreenTransform(pmaglevel : *mut f32, pxoffset : *mut i32, pyoffset : *mut i32) -> windows_core::BOOL);
    unsafe { MagGetFullscreenTransform(pmaglevel as _, pxoffset as _, pyoffset as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn MagGetImageScalingCallback(hwnd: super::super::Foundation::HWND) -> MagImageScalingCallback {
    windows_core::link!("magnification.dll" "system" fn MagGetImageScalingCallback(hwnd : super::super::Foundation:: HWND) -> MagImageScalingCallback);
    unsafe { MagGetImageScalingCallback(hwnd) }
}
#[inline]
pub unsafe fn MagGetInputTransform(pfenabled: *mut windows_core::BOOL, prectsource: *mut super::super::Foundation::RECT, prectdest: *mut super::super::Foundation::RECT) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagGetInputTransform(pfenabled : *mut windows_core::BOOL, prectsource : *mut super::super::Foundation:: RECT, prectdest : *mut super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { MagGetInputTransform(pfenabled as _, prectsource as _, prectdest as _) }
}
#[inline]
pub unsafe fn MagGetWindowFilterList(hwnd: super::super::Foundation::HWND, pdwfiltermode: *mut MW_FILTERMODE, count: i32, phwnd: *mut super::super::Foundation::HWND) -> i32 {
    windows_core::link!("magnification.dll" "system" fn MagGetWindowFilterList(hwnd : super::super::Foundation:: HWND, pdwfiltermode : *mut MW_FILTERMODE, count : i32, phwnd : *mut super::super::Foundation:: HWND) -> i32);
    unsafe { MagGetWindowFilterList(hwnd, pdwfiltermode as _, count, phwnd as _) }
}
#[inline]
pub unsafe fn MagGetWindowSource(hwnd: super::super::Foundation::HWND, prect: *mut super::super::Foundation::RECT) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagGetWindowSource(hwnd : super::super::Foundation:: HWND, prect : *mut super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { MagGetWindowSource(hwnd, prect as _) }
}
#[inline]
pub unsafe fn MagGetWindowTransform(hwnd: super::super::Foundation::HWND, ptransform: *mut MAGTRANSFORM) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagGetWindowTransform(hwnd : super::super::Foundation:: HWND, ptransform : *mut MAGTRANSFORM) -> windows_core::BOOL);
    unsafe { MagGetWindowTransform(hwnd, ptransform as _) }
}
#[inline]
pub unsafe fn MagInitialize() -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagInitialize() -> windows_core::BOOL);
    unsafe { MagInitialize() }
}
#[inline]
pub unsafe fn MagSetColorEffect(hwnd: super::super::Foundation::HWND, peffect: *mut MAGCOLOREFFECT) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagSetColorEffect(hwnd : super::super::Foundation:: HWND, peffect : *mut MAGCOLOREFFECT) -> windows_core::BOOL);
    unsafe { MagSetColorEffect(hwnd, peffect as _) }
}
#[inline]
pub unsafe fn MagSetFullscreenColorEffect(peffect: *const MAGCOLOREFFECT) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagSetFullscreenColorEffect(peffect : *const MAGCOLOREFFECT) -> windows_core::BOOL);
    unsafe { MagSetFullscreenColorEffect(peffect) }
}
#[inline]
pub unsafe fn MagSetFullscreenTransform(maglevel: f32, xoffset: i32, yoffset: i32) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagSetFullscreenTransform(maglevel : f32, xoffset : i32, yoffset : i32) -> windows_core::BOOL);
    unsafe { MagSetFullscreenTransform(maglevel, xoffset, yoffset) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn MagSetImageScalingCallback(hwnd: super::super::Foundation::HWND, callback: MagImageScalingCallback) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagSetImageScalingCallback(hwnd : super::super::Foundation:: HWND, callback : MagImageScalingCallback) -> windows_core::BOOL);
    unsafe { MagSetImageScalingCallback(hwnd, callback) }
}
#[inline]
pub unsafe fn MagSetInputTransform(fenabled: bool, prectsource: *const super::super::Foundation::RECT, prectdest: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
    windows_core::link!("magnification.dll" "system" fn MagSetInputTransform(fenabled : windows_core::BOOL, prectsource : *const super::super::Foundation:: RECT, prectdest : *const super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { MagSetInputTransform(fenabled.into(), prectsource, prectdest).ok() }
}
#[inline]
pub unsafe fn MagSetWindowFilterList(hwnd: super::super::Foundation::HWND, dwfiltermode: MW_FILTERMODE, count: i32, phwnd: *mut super::super::Foundation::HWND) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagSetWindowFilterList(hwnd : super::super::Foundation:: HWND, dwfiltermode : MW_FILTERMODE, count : i32, phwnd : *mut super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { MagSetWindowFilterList(hwnd, dwfiltermode, count, phwnd as _) }
}
#[inline]
pub unsafe fn MagSetWindowSource(hwnd: super::super::Foundation::HWND, rect: super::super::Foundation::RECT) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagSetWindowSource(hwnd : super::super::Foundation:: HWND, rect : super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { MagSetWindowSource(hwnd, core::mem::transmute(rect)) }
}
#[inline]
pub unsafe fn MagSetWindowTransform(hwnd: super::super::Foundation::HWND, ptransform: *mut MAGTRANSFORM) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagSetWindowTransform(hwnd : super::super::Foundation:: HWND, ptransform : *mut MAGTRANSFORM) -> windows_core::BOOL);
    unsafe { MagSetWindowTransform(hwnd, ptransform as _) }
}
#[inline]
pub unsafe fn MagShowSystemCursor(fshowcursor: bool) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagShowSystemCursor(fshowcursor : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { MagShowSystemCursor(fshowcursor.into()) }
}
#[inline]
pub unsafe fn MagUninitialize() -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagUninitialize() -> windows_core::BOOL);
    unsafe { MagUninitialize() }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MAGCOLOREFFECT {
    pub transform: [f32; 25],
}
impl Default for MAGCOLOREFFECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MAGIMAGEHEADER {
    pub width: u32,
    pub height: u32,
    pub format: windows_core::GUID,
    pub stride: u32,
    pub offset: u32,
    pub cbSize: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MAGTRANSFORM {
    pub v: [f32; 9],
}
impl Default for MAGTRANSFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MS_CLIPAROUNDCURSOR: i32 = 2i32;
pub const MS_INVERTCOLORS: i32 = 4i32;
pub const MS_SHOWMAGNIFIEDCURSOR: i32 = 1i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MW_FILTERMODE(pub u32);
pub const MW_FILTERMODE_EXCLUDE: MW_FILTERMODE = MW_FILTERMODE(0u32);
pub const MW_FILTERMODE_INCLUDE: MW_FILTERMODE = MW_FILTERMODE(1u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type MagImageScalingCallback = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, srcdata: *mut core::ffi::c_void, srcheader: MAGIMAGEHEADER, destdata: *mut core::ffi::c_void, destheader: MAGIMAGEHEADER, unclipped: super::super::Foundation::RECT, clipped: super::super::Foundation::RECT, dirty: super::super::Graphics::Gdi::HRGN) -> windows_core::BOOL>;
pub const WC_MAGNIFIER: windows_core::PCWSTR = windows_core::w!("Magnifier");
pub const WC_MAGNIFIERA: windows_core::PCSTR = windows_core::s!("Magnifier");
pub const WC_MAGNIFIERW: windows_core::PCWSTR = windows_core::w!("Magnifier");
