// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Procedure declarations, constant definitions, and macros for the NLS component.
use shared::basetsd::UINT32;
use shared::minwindef::{
    BOOL, BYTE, DWORD, HRGN, INT, LPARAM, LPCVOID, LPVOID, LRESULT, UINT, WPARAM
};
use shared::windef::{HBITMAP, HWND, POINT, PSIZE, RECT};
use um::uxtheme::MARGINS;
use um::winnt::{HANDLE, HRESULT, ULONGLONG};
pub const DWM_BB_ENABLE: DWORD = 0x00000001;
pub const DWM_BB_BLURREGION: DWORD = 0x00000002;
pub const DWM_BB_TRANSITIONONMAXIMIZED: DWORD = 0x00000004;
STRUCT!{#[repr(packed)] struct DWM_BLURBEHIND {
    dwFlags: DWORD,
    fEnable: BOOL,
    hRgnBlur: HRGN,
    fTransitionOnMaximized: BOOL,
}}
ENUM!{enum DWMWINDOWATTRIBUTE {
    DWMWA_NCRENDERING_ENABLED = 1,
    DWMWA_NCRENDERING_POLICY = 2,
    DWMWA_TRANSITIONS_FORCEDISABLED = 3,
    DWMWA_ALLOW_NCPAINT = 4,
    DWMWA_CAPTION_BUTTON_BOUNDS = 5,
    DWMWA_NONCLIENT_RTL_LAYOUT = 6,
    DWMWA_FORCE_ICONIC_REPRESENTATION = 7,
    DWMWA_FLIP3D_POLICY = 8,
    DWMWA_EXTENDED_FRAME_BOUNDS = 9,
    DWMWA_HAS_ICONIC_BITMAP = 10,
    DWMWA_DISALLOW_PEEK = 11,
    DWMWA_EXCLUDED_FROM_PEEK = 12,
    DWMWA_CLOAK = 13,
    DWMWA_CLOAKED = 14,
    DWMWA_FREEZE_REPRESENTATION = 15,
    DWMWA_LAST = 16,
}}
ENUM!{enum DWMNCRENDERINGPOLICY {
    DWMNCRP_USEWINDOWSTYLE = 0,
    DWMNCRP_DISABLED = 1,
    DWMNCRP_ENABLED = 2,
    DWMNCRP_LAST = 3,
}}
ENUM!{enum DWMFLIP3DWINDOWPOLICY {
    DWMFLIP3D_DEFAULT = 0,
    DWMFLIP3D_EXCLUDEBELOW = 1,
    DWMFLIP3D_EXCLUDEABOVE = 2,
    DWMFLIP3D_LAST = 3,
}}
pub const DWM_CLOAKED_APP: u32 = 0x00000001;
pub const DWM_CLOAKED_SHELL: u32 = 0x00000002;
pub const DWM_CLOAKED_INHERITED: u32 = 0x00000004;
pub type HTHUMBNAIL = HANDLE;
pub type PHTHUMBNAIL = *mut HTHUMBNAIL;
pub const DWM_TNP_RECTDESTINATION: DWORD = 0x00000001;
pub const DWM_TNP_RECTSOURCE: DWORD = 0x00000002;
pub const DWM_TNP_OPACITY: DWORD = 0x00000004;
pub const DWM_TNP_VISIBLE: DWORD = 0x00000008;
pub const DWM_TNP_SOURCECLIENTAREAONLY: DWORD = 0x00000010;
STRUCT!{#[repr(packed)] struct DWM_THUMBNAIL_PROPERTIES {
    dwFlags: DWORD,
    rcDestination: RECT,
    rcSource: RECT,
    opacity: BYTE,
    fVisible: BOOL,
    fSourceClientAreaOnly: BOOL,
}}
pub type PDWM_THUMBNAIL_PROPERTIES = *mut DWM_THUMBNAIL_PROPERTIES;
pub type DWM_FRAME_COUNT = ULONGLONG;
pub type QPC_TIME = ULONGLONG;
STRUCT!{#[repr(packed)] struct UNSIGNED_RATIO {
    uiNumerator: UINT32,
    uiDenominator: UINT32,
}}
STRUCT!{#[repr(packed)] struct DWM_TIMING_INFO {
    cbSize: UINT32,
    rateRefresh: UNSIGNED_RATIO,
    qpcRefreshPeriod: QPC_TIME,
    rateCompose: UNSIGNED_RATIO,
    qpcVBlank: QPC_TIME,
    cRefresh: DWM_FRAME_COUNT,
    cDXRefresh: UINT,
    qpcCompose: QPC_TIME,
    cFrame: DWM_FRAME_COUNT,
    cDXPresent: UINT,
    cRefreshFrame: DWM_FRAME_COUNT,
    cFrameSubmitted: DWM_FRAME_COUNT,
    cDXPresentSubmitted: UINT,
    cFrameConfirmed: DWM_FRAME_COUNT,
    cDXPresentConfirmed: UINT,
    cRefreshConfirmed: DWM_FRAME_COUNT,
    cDXRefreshConfirmed: UINT,
    cFramesLate: DWM_FRAME_COUNT,
    cFramesOutstanding: UINT,
    cFrameDisplayed: DWM_FRAME_COUNT,
    qpcFrameDisplayed: QPC_TIME,
    cRefreshFrameDisplayed: DWM_FRAME_COUNT,
    cFrameComplete: DWM_FRAME_COUNT,
    qpcFrameComplete: QPC_TIME,
    cFramePending: DWM_FRAME_COUNT,
    qpcFramePending: QPC_TIME,
    cFramesDisplayed: DWM_FRAME_COUNT,
    cFramesComplete: DWM_FRAME_COUNT,
    cFramesPending: DWM_FRAME_COUNT,
    cFramesAvailable: DWM_FRAME_COUNT,
    cFramesDropped: DWM_FRAME_COUNT,
    cFramesMissed: DWM_FRAME_COUNT,
    cRefreshNextDisplayed: DWM_FRAME_COUNT,
    cRefreshNextPresented: DWM_FRAME_COUNT,
    cRefreshesDisplayed: DWM_FRAME_COUNT,
    cRefreshesPresented: DWM_FRAME_COUNT,
    cRefreshStarted: DWM_FRAME_COUNT,
    cPixelsReceived: ULONGLONG,
    cPixelsDrawn: ULONGLONG,
    cBuffersEmpty: DWM_FRAME_COUNT,
}}
ENUM!{enum DWM_SOURCE_FRAME_SAMPLING {
    DWM_SOURCE_FRAME_SAMPLING_POINT = 0,
    DWM_SOURCE_FRAME_SAMPLING_COVERAGE = 1,
    DWM_SOURCE_FRAME_SAMPLING_LAST = 2,
}}
// pub const c_DwmMaxQueuedBuffers: UINT = 8;
// pub const c_DwmMaxMonitors: UINT = 16;
// pub const c_DwmMaxAdapters: UINT = 16;
STRUCT!{#[repr(packed)] struct DWM_PRESENT_PARAMETERS {
    cbSize: UINT32,
    fQueue: BOOL,
    cRefreshStart: DWM_FRAME_COUNT,
    cBuffer: UINT,
    fUseSourceRate: BOOL,
    rateSource: UNSIGNED_RATIO,
    cRefreshesPerFrame: UINT,
    eSampling: DWM_SOURCE_FRAME_SAMPLING,
}}
// pub const DWM_FRAME_DURATION_DEFAULT: i32 = -1;
extern "system" {
    pub fn DwmDefWindowProc(
        hWnd: HWND,
        msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
        plResult: *mut LRESULT,
    ) -> BOOL;
    pub fn DwmEnableBlurBehindWindow(
        hWnd: HWND,
        pBlurBehind: *const DWM_BLURBEHIND,
    ) -> HRESULT;
}
pub const DWM_EC_DISABLECOMPOSITION: UINT = 0;
pub const DWM_EC_ENABLECOMPOSITION: UINT = 1;
extern "system" {
    pub fn DwmEnableComposition(
        uCompositionAction: UINT,
    ) -> HRESULT;
    pub fn DwmEnableMMCSS(
        fEnableMMCSS: BOOL,
    ) -> HRESULT;
    pub fn DwmExtendFrameIntoClientArea(
        hWnd: HWND,
        pMarInset: *const MARGINS,
    ) -> HRESULT;
    pub fn DwmGetColorizationColor(
        pcrColorization: *mut DWORD,
        pfOpaqueBlend: *mut BOOL,
    ) -> HRESULT;
    pub fn DwmGetCompositionTimingInfo(
        hWnd: HWND,
        pTimingInfo: *mut DWM_TIMING_INFO,
    ) -> HRESULT;
    pub fn DwmGetWindowAttribute(
        hWnd: HWND,
        dwAttribute: DWORD,
        pvAttribute: LPVOID,
        cbAttribute: DWORD,
    ) -> HRESULT;
    pub fn DwmIsCompositionEnabled(
        pfEnabled: *mut BOOL,
    ) -> HRESULT;
    pub fn DwmModifyPreviousDxFrameDuration(
        hwnd: HWND,
        cRefreshes: INT,
        fRelative: BOOL,
    ) -> HRESULT;
    pub fn DwmQueryThumbnailSourceSize(
        hThumbnail: HTHUMBNAIL,
        pSize: PSIZE,
    ) -> HRESULT;
    pub fn DwmRegisterThumbnail(
        hwndDestination: HWND,
        hwndSource: HWND,
        phThumbnailId: PHTHUMBNAIL,
    ) -> HRESULT;
    pub fn DwmSetDxFrameDuration(
        hwnd: HWND,
        cRefreshes: INT,
    ) -> HRESULT;
    pub fn DwmSetPresentParameters(
        hwnd: HWND,
        pPresentParams: *mut DWM_PRESENT_PARAMETERS,
    ) -> HRESULT;
    pub fn DwmSetWindowAttribute(
        hWnd: HWND,
        dwAttribute: DWORD,
        pvAttribute: LPCVOID,
        cbAttribute: DWORD,
    ) -> HRESULT;
    pub fn DwmUnregisterThumbnail(
        hThumbnailId: HTHUMBNAIL,
    ) -> HRESULT;
    pub fn DwmUpdateThumbnailProperties(
        hThumbnailId: HTHUMBNAIL,
        ptnProperties: *const DWM_THUMBNAIL_PROPERTIES,
    ) -> HRESULT;
}
pub const DWM_SIT_DISPLAYFRAME: DWORD = 0x00000001;
extern "system" {
    pub fn DwmSetIconicThumbnail(
        hwnd: HWND,
        hbmp: HBITMAP,
        dwSITFlags: DWORD,
    ) -> HRESULT;
    pub fn DwmSetIconicLivePreviewBitmap(
        hwnd: HWND,
        hbmp: HBITMAP,
        pptClient: *mut POINT,
        dwSITFlags: DWORD,
    ) -> HRESULT;
    pub fn DwmInvalidateIconicBitmaps(
        hwnd: HWND,
    ) -> HRESULT;
    // pub fn DwmAttachMilContent(hwnd: HWND) -> HRESULT;
    // pub fn DwmDetachMilContent(hwnd: HWND) -> HRESULT;
    pub fn DwmFlush() -> HRESULT;
    // pub fn DwmGetGraphicsStreamTransformHint();
    // pub fn DwmGetGraphicsStreamClient();
    pub fn DwmGetTransportAttributes(
        pfIsRemoting: *mut BOOL,
        pfIsConnected: *mut BOOL,
        pDwGeneration: *mut DWORD,
    ) -> HRESULT;
}
ENUM!{enum DWMTRANSITION_OWNEDWINDOW_TARGET {
    DWMTRANSITION_OWNEDWINDOW_NULL = -1i32 as u32,
    DWMTRANSITION_OWNEDWINDOW_REPOSITION = 0,
}}
extern "system" {
    pub fn DwmTransitionOwnedWindow(
        hwnd: HWND,
        target: DWMTRANSITION_OWNEDWINDOW_TARGET,
    ) -> HRESULT;
}
ENUM!{enum GESTURE_TYPE {
    GT_PEN_TAP = 0,
    GT_PEN_DOUBLETAP = 1,
    GT_PEN_RIGHTTAP = 2,
    GT_PEN_PRESSANDHOLD = 3,
    GT_PEN_PRESSANDHOLDABORT = 4,
    GT_TOUCH_TAP = 5,
    GT_TOUCH_DOUBLETAP = 6,
    GT_TOUCH_RIGHTTAP = 7,
    GT_TOUCH_PRESSANDHOLD = 8,
    GT_TOUCH_PRESSANDHOLDABORT = 9,
    GT_TOUCH_PRESSANDTAP = 10,
}}
extern "system" {
    pub fn DwmRenderGesture(
        gt: GESTURE_TYPE,
        cContacts: UINT,
        pdwPointerID: *const DWORD,
        pPoints: *const POINT,
    ) -> HRESULT;
    pub fn DwmTetherContact(
        dwPointerID: DWORD,
        fEnable: BOOL,
        ptTether: POINT,
    ) -> HRESULT;
}
ENUM!{enum DWM_SHOWCONTACT {
    DWMSC_DOWN = 0x00000001,
    DWMSC_UP = 0x00000002,
    DWMSC_DRAG = 0x00000004,
    DWMSC_HOLD = 0x00000008,
    DWMSC_PENBARREL = 0x00000010,
    DWMSC_NONE = 0x00000000,
    DWMSC_ALL = 0xFFFFFFFF,
}}
extern "system" {
    pub fn DwmShowContact(
        dwPointerID: DWORD,
        eShowContact: DWM_SHOWCONTACT,
    ) -> HRESULT;
}
