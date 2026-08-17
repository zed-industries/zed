windows_targets::link!("dwmapi.dll" "system" fn DwmAttachMilContent(hwnd : super::super::Foundation:: HWND) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmDefWindowProc(hwnd : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM, plresult : *mut super::super::Foundation:: LRESULT) -> windows_sys::core::BOOL);
windows_targets::link!("dwmapi.dll" "system" fn DwmDetachMilContent(hwnd : super::super::Foundation:: HWND) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("dwmapi.dll" "system" fn DwmEnableBlurBehindWindow(hwnd : super::super::Foundation:: HWND, pblurbehind : *const DWM_BLURBEHIND) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmEnableComposition(ucompositionaction : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmEnableMMCSS(fenablemmcss : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_UI_Controls")]
windows_targets::link!("dwmapi.dll" "system" fn DwmExtendFrameIntoClientArea(hwnd : super::super::Foundation:: HWND, pmarinset : *const super::super::UI::Controls:: MARGINS) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmFlush() -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmGetColorizationColor(pcrcolorization : *mut u32, pfopaqueblend : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmGetCompositionTimingInfo(hwnd : super::super::Foundation:: HWND, ptiminginfo : *mut DWM_TIMING_INFO) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmGetGraphicsStreamClient(uindex : u32, pclientuuid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmGetGraphicsStreamTransformHint(uindex : u32, ptransform : *mut MilMatrix3x2D) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmGetTransportAttributes(pfisremoting : *mut windows_sys::core::BOOL, pfisconnected : *mut windows_sys::core::BOOL, pdwgeneration : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmGetUnmetTabRequirements(appwindow : super::super::Foundation:: HWND, value : *mut DWM_TAB_WINDOW_REQUIREMENTS) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmGetWindowAttribute(hwnd : super::super::Foundation:: HWND, dwattribute : u32, pvattribute : *mut core::ffi::c_void, cbattribute : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmInvalidateIconicBitmaps(hwnd : super::super::Foundation:: HWND) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmIsCompositionEnabled(pfenabled : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmModifyPreviousDxFrameDuration(hwnd : super::super::Foundation:: HWND, crefreshes : i32, frelative : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmQueryThumbnailSourceSize(hthumbnail : isize, psize : *mut super::super::Foundation:: SIZE) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmRegisterThumbnail(hwnddestination : super::super::Foundation:: HWND, hwndsource : super::super::Foundation:: HWND, phthumbnailid : *mut isize) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmRenderGesture(gt : GESTURE_TYPE, ccontacts : u32, pdwpointerid : *const u32, ppoints : *const super::super::Foundation:: POINT) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmSetDxFrameDuration(hwnd : super::super::Foundation:: HWND, crefreshes : i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("dwmapi.dll" "system" fn DwmSetIconicLivePreviewBitmap(hwnd : super::super::Foundation:: HWND, hbmp : super::Gdi:: HBITMAP, pptclient : *const super::super::Foundation:: POINT, dwsitflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("dwmapi.dll" "system" fn DwmSetIconicThumbnail(hwnd : super::super::Foundation:: HWND, hbmp : super::Gdi:: HBITMAP, dwsitflags : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmSetPresentParameters(hwnd : super::super::Foundation:: HWND, ppresentparams : *mut DWM_PRESENT_PARAMETERS) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmSetWindowAttribute(hwnd : super::super::Foundation:: HWND, dwattribute : u32, pvattribute : *const core::ffi::c_void, cbattribute : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmShowContact(dwpointerid : u32, eshowcontact : DWM_SHOWCONTACT) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmTetherContact(dwpointerid : u32, fenable : windows_sys::core::BOOL, pttether : super::super::Foundation:: POINT) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmTransitionOwnedWindow(hwnd : super::super::Foundation:: HWND, target : DWMTRANSITION_OWNEDWINDOW_TARGET) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmUnregisterThumbnail(hthumbnailid : isize) -> windows_sys::core::HRESULT);
windows_targets::link!("dwmapi.dll" "system" fn DwmUpdateThumbnailProperties(hthumbnailid : isize, ptnproperties : *const DWM_THUMBNAIL_PROPERTIES) -> windows_sys::core::HRESULT);
pub type DWMFLIP3DWINDOWPOLICY = i32;
pub const DWMFLIP3D_DEFAULT: DWMFLIP3DWINDOWPOLICY = 0i32;
pub const DWMFLIP3D_EXCLUDEABOVE: DWMFLIP3DWINDOWPOLICY = 2i32;
pub const DWMFLIP3D_EXCLUDEBELOW: DWMFLIP3DWINDOWPOLICY = 1i32;
pub const DWMFLIP3D_LAST: DWMFLIP3DWINDOWPOLICY = 3i32;
pub type DWMNCRENDERINGPOLICY = i32;
pub const DWMNCRP_DISABLED: DWMNCRENDERINGPOLICY = 1i32;
pub const DWMNCRP_ENABLED: DWMNCRENDERINGPOLICY = 2i32;
pub const DWMNCRP_LAST: DWMNCRENDERINGPOLICY = 3i32;
pub const DWMNCRP_USEWINDOWSTYLE: DWMNCRENDERINGPOLICY = 0i32;
pub const DWMSBT_AUTO: DWM_SYSTEMBACKDROP_TYPE = 0i32;
pub const DWMSBT_MAINWINDOW: DWM_SYSTEMBACKDROP_TYPE = 2i32;
pub const DWMSBT_NONE: DWM_SYSTEMBACKDROP_TYPE = 1i32;
pub const DWMSBT_TABBEDWINDOW: DWM_SYSTEMBACKDROP_TYPE = 4i32;
pub const DWMSBT_TRANSIENTWINDOW: DWM_SYSTEMBACKDROP_TYPE = 3i32;
pub const DWMSC_ALL: DWM_SHOWCONTACT = 4294967295u32;
pub const DWMSC_DOWN: DWM_SHOWCONTACT = 1u32;
pub const DWMSC_DRAG: DWM_SHOWCONTACT = 4u32;
pub const DWMSC_HOLD: DWM_SHOWCONTACT = 8u32;
pub const DWMSC_NONE: DWM_SHOWCONTACT = 0u32;
pub const DWMSC_PENBARREL: DWM_SHOWCONTACT = 16u32;
pub const DWMSC_UP: DWM_SHOWCONTACT = 2u32;
pub const DWMTRANSITION_OWNEDWINDOW_NULL: DWMTRANSITION_OWNEDWINDOW_TARGET = -1i32;
pub const DWMTRANSITION_OWNEDWINDOW_REPOSITION: DWMTRANSITION_OWNEDWINDOW_TARGET = 0i32;
pub type DWMTRANSITION_OWNEDWINDOW_TARGET = i32;
pub const DWMTWR_APP_COMPAT: DWM_TAB_WINDOW_REQUIREMENTS = 512i32;
pub const DWMTWR_GROUP_POLICY: DWM_TAB_WINDOW_REQUIREMENTS = 256i32;
pub const DWMTWR_IMPLEMENTED_BY_SYSTEM: DWM_TAB_WINDOW_REQUIREMENTS = 1i32;
pub const DWMTWR_NONE: DWM_TAB_WINDOW_REQUIREMENTS = 0i32;
pub const DWMTWR_TABBING_ENABLED: DWM_TAB_WINDOW_REQUIREMENTS = 64i32;
pub const DWMTWR_USER_POLICY: DWM_TAB_WINDOW_REQUIREMENTS = 128i32;
pub const DWMTWR_WINDOW_DWM_ATTRIBUTES: DWM_TAB_WINDOW_REQUIREMENTS = 16i32;
pub const DWMTWR_WINDOW_MARGINS: DWM_TAB_WINDOW_REQUIREMENTS = 32i32;
pub const DWMTWR_WINDOW_REGION: DWM_TAB_WINDOW_REQUIREMENTS = 8i32;
pub const DWMTWR_WINDOW_RELATIONSHIP: DWM_TAB_WINDOW_REQUIREMENTS = 2i32;
pub const DWMTWR_WINDOW_STYLES: DWM_TAB_WINDOW_REQUIREMENTS = 4i32;
pub const DWMWA_ALLOW_NCPAINT: DWMWINDOWATTRIBUTE = 4i32;
pub const DWMWA_BORDER_COLOR: DWMWINDOWATTRIBUTE = 34i32;
pub const DWMWA_CAPTION_BUTTON_BOUNDS: DWMWINDOWATTRIBUTE = 5i32;
pub const DWMWA_CAPTION_COLOR: DWMWINDOWATTRIBUTE = 35i32;
pub const DWMWA_CLOAK: DWMWINDOWATTRIBUTE = 13i32;
pub const DWMWA_CLOAKED: DWMWINDOWATTRIBUTE = 14i32;
pub const DWMWA_COLOR_DEFAULT: u32 = 4294967295u32;
pub const DWMWA_COLOR_NONE: u32 = 4294967294u32;
pub const DWMWA_DISALLOW_PEEK: DWMWINDOWATTRIBUTE = 11i32;
pub const DWMWA_EXCLUDED_FROM_PEEK: DWMWINDOWATTRIBUTE = 12i32;
pub const DWMWA_EXTENDED_FRAME_BOUNDS: DWMWINDOWATTRIBUTE = 9i32;
pub const DWMWA_FLIP3D_POLICY: DWMWINDOWATTRIBUTE = 8i32;
pub const DWMWA_FORCE_ICONIC_REPRESENTATION: DWMWINDOWATTRIBUTE = 7i32;
pub const DWMWA_FREEZE_REPRESENTATION: DWMWINDOWATTRIBUTE = 15i32;
pub const DWMWA_HAS_ICONIC_BITMAP: DWMWINDOWATTRIBUTE = 10i32;
pub const DWMWA_LAST: DWMWINDOWATTRIBUTE = 39i32;
pub const DWMWA_NCRENDERING_ENABLED: DWMWINDOWATTRIBUTE = 1i32;
pub const DWMWA_NCRENDERING_POLICY: DWMWINDOWATTRIBUTE = 2i32;
pub const DWMWA_NONCLIENT_RTL_LAYOUT: DWMWINDOWATTRIBUTE = 6i32;
pub const DWMWA_PASSIVE_UPDATE_MODE: DWMWINDOWATTRIBUTE = 16i32;
pub const DWMWA_SYSTEMBACKDROP_TYPE: DWMWINDOWATTRIBUTE = 38i32;
pub const DWMWA_TEXT_COLOR: DWMWINDOWATTRIBUTE = 36i32;
pub const DWMWA_TRANSITIONS_FORCEDISABLED: DWMWINDOWATTRIBUTE = 3i32;
pub const DWMWA_USE_HOSTBACKDROPBRUSH: DWMWINDOWATTRIBUTE = 17i32;
pub const DWMWA_USE_IMMERSIVE_DARK_MODE: DWMWINDOWATTRIBUTE = 20i32;
pub const DWMWA_VISIBLE_FRAME_BORDER_THICKNESS: DWMWINDOWATTRIBUTE = 37i32;
pub const DWMWA_WINDOW_CORNER_PREFERENCE: DWMWINDOWATTRIBUTE = 33i32;
pub const DWMWCP_DEFAULT: DWM_WINDOW_CORNER_PREFERENCE = 0i32;
pub const DWMWCP_DONOTROUND: DWM_WINDOW_CORNER_PREFERENCE = 1i32;
pub const DWMWCP_ROUND: DWM_WINDOW_CORNER_PREFERENCE = 2i32;
pub const DWMWCP_ROUNDSMALL: DWM_WINDOW_CORNER_PREFERENCE = 3i32;
pub type DWMWINDOWATTRIBUTE = i32;
pub const DWM_BB_BLURREGION: u32 = 2u32;
pub const DWM_BB_ENABLE: u32 = 1u32;
pub const DWM_BB_TRANSITIONONMAXIMIZED: u32 = 4u32;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct DWM_BLURBEHIND {
    pub dwFlags: u32,
    pub fEnable: windows_sys::core::BOOL,
    pub hRgnBlur: super::Gdi::HRGN,
    pub fTransitionOnMaximized: windows_sys::core::BOOL,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for DWM_BLURBEHIND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DWM_CLOAKED_APP: u32 = 1u32;
pub const DWM_CLOAKED_INHERITED: u32 = 4u32;
pub const DWM_CLOAKED_SHELL: u32 = 2u32;
pub const DWM_EC_DISABLECOMPOSITION: u32 = 0u32;
pub const DWM_EC_ENABLECOMPOSITION: u32 = 1u32;
pub const DWM_FRAME_DURATION_DEFAULT: i32 = -1i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DWM_PRESENT_PARAMETERS {
    pub cbSize: u32,
    pub fQueue: windows_sys::core::BOOL,
    pub cRefreshStart: u64,
    pub cBuffer: u32,
    pub fUseSourceRate: windows_sys::core::BOOL,
    pub rateSource: UNSIGNED_RATIO,
    pub cRefreshesPerFrame: u32,
    pub eSampling: DWM_SOURCE_FRAME_SAMPLING,
}
pub type DWM_SHOWCONTACT = u32;
pub const DWM_SIT_DISPLAYFRAME: u32 = 1u32;
pub type DWM_SOURCE_FRAME_SAMPLING = i32;
pub const DWM_SOURCE_FRAME_SAMPLING_COVERAGE: DWM_SOURCE_FRAME_SAMPLING = 1i32;
pub const DWM_SOURCE_FRAME_SAMPLING_LAST: DWM_SOURCE_FRAME_SAMPLING = 2i32;
pub const DWM_SOURCE_FRAME_SAMPLING_POINT: DWM_SOURCE_FRAME_SAMPLING = 0i32;
pub type DWM_SYSTEMBACKDROP_TYPE = i32;
pub type DWM_TAB_WINDOW_REQUIREMENTS = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DWM_THUMBNAIL_PROPERTIES {
    pub dwFlags: u32,
    pub rcDestination: super::super::Foundation::RECT,
    pub rcSource: super::super::Foundation::RECT,
    pub opacity: u8,
    pub fVisible: windows_sys::core::BOOL,
    pub fSourceClientAreaOnly: windows_sys::core::BOOL,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DWM_TIMING_INFO {
    pub cbSize: u32,
    pub rateRefresh: UNSIGNED_RATIO,
    pub qpcRefreshPeriod: u64,
    pub rateCompose: UNSIGNED_RATIO,
    pub qpcVBlank: u64,
    pub cRefresh: u64,
    pub cDXRefresh: u32,
    pub qpcCompose: u64,
    pub cFrame: u64,
    pub cDXPresent: u32,
    pub cRefreshFrame: u64,
    pub cFrameSubmitted: u64,
    pub cDXPresentSubmitted: u32,
    pub cFrameConfirmed: u64,
    pub cDXPresentConfirmed: u32,
    pub cRefreshConfirmed: u64,
    pub cDXRefreshConfirmed: u32,
    pub cFramesLate: u64,
    pub cFramesOutstanding: u32,
    pub cFrameDisplayed: u64,
    pub qpcFrameDisplayed: u64,
    pub cRefreshFrameDisplayed: u64,
    pub cFrameComplete: u64,
    pub qpcFrameComplete: u64,
    pub cFramePending: u64,
    pub qpcFramePending: u64,
    pub cFramesDisplayed: u64,
    pub cFramesComplete: u64,
    pub cFramesPending: u64,
    pub cFramesAvailable: u64,
    pub cFramesDropped: u64,
    pub cFramesMissed: u64,
    pub cRefreshNextDisplayed: u64,
    pub cRefreshNextPresented: u64,
    pub cRefreshesDisplayed: u64,
    pub cRefreshesPresented: u64,
    pub cRefreshStarted: u64,
    pub cPixelsReceived: u64,
    pub cPixelsDrawn: u64,
    pub cBuffersEmpty: u64,
}
pub const DWM_TNP_OPACITY: u32 = 4u32;
pub const DWM_TNP_RECTDESTINATION: u32 = 1u32;
pub const DWM_TNP_RECTSOURCE: u32 = 2u32;
pub const DWM_TNP_SOURCECLIENTAREAONLY: u32 = 16u32;
pub const DWM_TNP_VISIBLE: u32 = 8u32;
pub type DWM_WINDOW_CORNER_PREFERENCE = i32;
pub type GESTURE_TYPE = i32;
pub const GT_PEN_DOUBLETAP: GESTURE_TYPE = 1i32;
pub const GT_PEN_PRESSANDHOLD: GESTURE_TYPE = 3i32;
pub const GT_PEN_PRESSANDHOLDABORT: GESTURE_TYPE = 4i32;
pub const GT_PEN_RIGHTTAP: GESTURE_TYPE = 2i32;
pub const GT_PEN_TAP: GESTURE_TYPE = 0i32;
pub const GT_TOUCH_DOUBLETAP: GESTURE_TYPE = 6i32;
pub const GT_TOUCH_PRESSANDHOLD: GESTURE_TYPE = 8i32;
pub const GT_TOUCH_PRESSANDHOLDABORT: GESTURE_TYPE = 9i32;
pub const GT_TOUCH_PRESSANDTAP: GESTURE_TYPE = 10i32;
pub const GT_TOUCH_RIGHTTAP: GESTURE_TYPE = 7i32;
pub const GT_TOUCH_TAP: GESTURE_TYPE = 5i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MilMatrix3x2D {
    pub S_11: f64,
    pub S_12: f64,
    pub S_21: f64,
    pub S_22: f64,
    pub DX: f64,
    pub DY: f64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct UNSIGNED_RATIO {
    pub uiNumerator: u32,
    pub uiDenominator: u32,
}
pub const c_DwmMaxAdapters: u32 = 16u32;
pub const c_DwmMaxMonitors: u32 = 16u32;
pub const c_DwmMaxQueuedBuffers: u32 = 8u32;
