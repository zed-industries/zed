// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of dcomp.h
use ctypes::{c_float, c_int, c_void};
use shared::d3d9types::D3DMATRIX;
use shared::dcomptypes::{
    DCOMPOSITION_BACKFACE_VISIBILITY, DCOMPOSITION_BITMAP_INTERPOLATION_MODE,
    DCOMPOSITION_BORDER_MODE, DCOMPOSITION_COMPOSITE_MODE, DCOMPOSITION_DEPTH_MODE,
    DCOMPOSITION_FRAME_STATISTICS, DCOMPOSITION_OPACITY_MODE
};
use shared::dxgi::IDXGIDevice;
use shared::dxgi1_2::DXGI_ALPHA_MODE;
use shared::dxgiformat::DXGI_FORMAT;
use shared::guiddef::REFIID;
use shared::minwindef::{BOOL, DWORD, UINT};
use shared::ntdef::{HANDLE, HRESULT};
use shared::windef::{HWND, POINT, RECT};
use um::d2d1::{D2D1_COLOR_F, D2D1_MATRIX_3X2_F};
use um::d2d1_1::{D2D1_COMPOSITE_MODE, D2D1_MATRIX_5X4_F, D2D1_VECTOR_2F, D2D1_VECTOR_4F};
use um::d2d1effects::{
    D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE, D2D1_BLEND_MODE, D2D1_BORDER_MODE,
    D2D1_COLORMATRIX_ALPHA_MODE, D2D1_TURBULENCE_NOISE
};
use um::d2dbasetypes::{D2D_MATRIX_3X2_F, D2D_MATRIX_4X4_F, D2D_RECT_F};
use um::d3dcommon::D3D_FEATURE_LEVEL;
use um::dcompanimation::IDCompositionAnimation;
use um::minwinbase::SECURITY_ATTRIBUTES;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
extern "system" {
    pub fn DCompositionCreateDevice(
        dxgiDevice: *const IDXGIDevice,
        iid: REFIID,
        dcompositionDevice: *mut *mut c_void,
    ) -> HRESULT;
    pub fn DCompositionCreateDevice2(
        renderingDevice: *const IUnknown,
        iid: REFIID,
        dcompositionDevice: *mut *mut c_void,
    ) -> HRESULT;
    pub fn DCompositionCreateDevice3(
        renderingDevice: *const IUnknown,
        iid: REFIID,
        dcompositionDevice: *mut *mut c_void,
    ) -> HRESULT;
    pub fn DCompositionGetFrameStatistics(
        statistics: *const DCOMPOSITION_FRAME_STATISTICS,
        minSafeFeaturelLevel: *const D3D_FEATURE_LEVEL,
        maxHardwareFeaturelLevel: *const D3D_FEATURE_LEVEL,
    ) -> HRESULT;
    pub fn DCompositionCreateSurfaceHandle(
        desiredAccess: DWORD,
        securityAttributes: *const SECURITY_ATTRIBUTES,
        surfaceHandle: *mut HANDLE,
    ) -> HRESULT;
    pub fn DCompositionAttachMouseWheelToHwnd(
        visual: *const IDCompositionVisual,
        hwnd: HWND,
        enable: BOOL,
    ) -> HRESULT;
    pub fn DCompositionAttachMouseDragToHwnd(
        visual: *const IDCompositionVisual,
        hwnd: HWND,
        enable: BOOL,
    ) -> HRESULT;
}
RIDL!{#[uuid(0xc37ea93a, 0xe7aa, 0x450d, 0xb1, 0x6f, 0x97, 0x46, 0xcb, 0x04, 0x07, 0xf3)]
interface IDCompositionDevice(IDCompositionDeviceVtbl): IUnknown(IUnknownVtbl) {
    fn Commit() -> HRESULT,
    fn WaitForCommitCompletion() -> HRESULT,
    fn GetFrameStatistics(
        statistics: *mut DCOMPOSITION_FRAME_STATISTICS,
    ) -> HRESULT,
    fn CreateTargetForHwnd(
        hwnd: HWND,
        topmost: BOOL,
        target: *mut *mut IDCompositionTarget,
    ) -> HRESULT,
    fn CreateVisual(
        visual: *mut *mut IDCompositionVisual,
    ) -> HRESULT,
    fn CreateSurface(
        width: UINT,
        height: UINT,
        pixelFormat: DXGI_FORMAT,
        alphaMode: DXGI_ALPHA_MODE,
        surface: *mut *mut IDCompositionSurface,
    ) -> HRESULT,
    fn CreateVirtualSurface(
        initialWidth: UINT,
        initialHeight: UINT,
        pixelFormat: DXGI_FORMAT,
        alphaMode: DXGI_ALPHA_MODE,
        virtualSurface: *mut *mut IDCompositionVirtualSurface,
    ) -> HRESULT,
    fn CreateSurfaceFromHandle(
        handle: HANDLE,
        mutsurface: *mut *mut IUnknown,
    ) -> HRESULT,
    fn CreateSurfaceFromHwnd(
        hwnd: HWND,
        mutsurface: *mut *mut IUnknown,
    ) -> HRESULT,
    fn CreateTranslateTransform(
        translateTransform: *mut *mut IDCompositionTranslateTransform,
    ) -> HRESULT,
    fn CreateScaleTransform(
        scaleTransform: *mut *mut IDCompositionScaleTransform,
    ) -> HRESULT,
    fn CreateRotateTransform(
        rotateTransform: *mut *mut IDCompositionRotateTransform,
    ) -> HRESULT,
    fn CreateSkewTransform(
        skewTransform: *mut *mut IDCompositionSkewTransform,
    ) -> HRESULT,
    fn CreateMatrixTransform(
        matrixTransform: *mut *mut IDCompositionMatrixTransform,
    ) -> HRESULT,
    fn CreateTransformGroup(
        transforms: *const *const IDCompositionTransform,
        elements: UINT,
        transformGroup: *mut *mut IDCompositionTransform,
    ) -> HRESULT,
    fn CreateTranslateTransform3D(
        translateTransform3D: *mut *mut IDCompositionTranslateTransform3D,
    ) -> HRESULT,
    fn CreateScaleTransform3D(
        scaleTransform3D: *mut *mut IDCompositionScaleTransform3D,
    ) -> HRESULT,
    fn CreateRotateTransform3D(
        rotateTransform3D: *mut *mut IDCompositionRotateTransform3D,
    ) -> HRESULT,
    fn CreateMatrixTransform3D(
        matrixTransform3D: *mut *mut IDCompositionMatrixTransform3D,
    ) -> HRESULT,
    fn CreateTransform3DGroup(
        transforms3D: *const *const IDCompositionTransform3D,
        elements: UINT,
        transform3DGroup: *mut *mut IDCompositionTransform3D,
    ) -> HRESULT,
    fn CreateEffectGroup(
        effectGroup: *mut *mut IDCompositionEffectGroup,
    ) -> HRESULT,
    fn CreateRectangleClip(
        clip: *mut *mut IDCompositionRectangleClip,
    ) -> HRESULT,
    fn CreateAnimation(
        animation: *mut *mut IDCompositionAnimation,
    ) -> HRESULT,
    fn CheckDeviceState(
        pfValid: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xeacdd04c, 0x117e, 0x4e17, 0x88, 0xf4, 0xd1, 0xb1, 0x2b, 0x0e, 0x3d, 0x89)]
interface IDCompositionTarget(IDCompositionTargetVtbl): IUnknown(IUnknownVtbl) {
    fn SetRoot(
        visual: *const IDCompositionVisual,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4d93059d, 0x097b, 0x4651, 0x9a, 0x60, 0xf0, 0xf2, 0x51, 0x16, 0xe2, 0xf3)]
interface IDCompositionVisual(IDCompositionVisualVtbl): IUnknown(IUnknownVtbl) {
    fn SetOffsetX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetOffsetX_1(
        offsetX: c_float,
    ) -> HRESULT,
    fn SetOffsetY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetOffsetY_1(
        offsetY: c_float,
    ) -> HRESULT,
    fn SetTransform_2(
        transform: *const IDCompositionTransform,
    ) -> HRESULT,
    fn SetTransform_1(
        matrix: *const D2D_MATRIX_3X2_F,
    ) -> HRESULT,
    fn SetTransformParent(
        visual: *const IDCompositionVisual,
    ) -> HRESULT,
    fn SetEffect(
        effect: *const IDCompositionEffect,
    ) -> HRESULT,
    fn SetBitmapInterpolationMode(
        interpolationMode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE,
    ) -> HRESULT,
    fn SetBorderMode(
        borderMode: DCOMPOSITION_BORDER_MODE,
    ) -> HRESULT,
    fn SetClip_2(
        clip: *const IDCompositionClip,
    ) -> HRESULT,
    fn SetClip_1(
        rect: *const D2D_RECT_F,
    ) -> HRESULT,
    fn SetContent(
        content: *const IUnknown,
    ) -> HRESULT,
    fn AddVisual(
        visual: *const IDCompositionVisual,
        insertAbove: BOOL,
        referenceVisual: *const IDCompositionVisual,
    ) -> HRESULT,
    fn RemoveVisual(
        visual: *const IDCompositionVisual,
    ) -> HRESULT,
    fn RemoveAllVisuals() -> HRESULT,
    fn SetCompositeMode(
        compositeMode: DCOMPOSITION_COMPOSITE_MODE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xec81b08f, 0xbfcb, 0x4e8d, 0xb1, 0x93, 0xa9, 0x15, 0x58, 0x79, 0x99, 0xe8)]
interface IDCompositionEffect(IDCompositionEffectVtbl): IUnknown(IUnknownVtbl) {}}
RIDL!{#[uuid(0x71185722, 0x246b, 0x41f2, 0xaa, 0xd1, 0x04, 0x43, 0xf7, 0xf4, 0xbf, 0xc2)]
interface IDCompositionTransform3D(IDCompositionTransform3DVtbl):
    IDCompositionEffect(IDCompositionEffectVtbl) {}}
RIDL!{#[uuid(0xfd55faa7, 0x37e0, 0x4c20, 0x95, 0xd2, 0x9b, 0xe4, 0x5b, 0xc3, 0x3f, 0x55)]
interface IDCompositionTransform(IDCompositionTransformVtbl):
    IDCompositionTransform3D(IDCompositionTransform3DVtbl) {}}
RIDL!{#[uuid(0x06791122, 0xc6f0, 0x417d, 0x83, 0x23, 0x26, 0x9e, 0x98, 0x7f, 0x59, 0x54)]
interface IDCompositionTranslateTransform(IDCompositionTranslateTransformVtbl):
    IDCompositionTransform(IDCompositionTransformVtbl) {
    fn SetOffsetX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetOffsetX_1(
        offsetX: c_float,
    ) -> HRESULT,
    fn SetOffsetY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetOffsetY_1(
        offsetY: c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x71fde914, 0x40ef, 0x45ef, 0xbd, 0x51, 0x68, 0xb0, 0x37, 0xc3, 0x39, 0xf9)]
interface IDCompositionScaleTransform(IDCompositionScaleTransformVtbl):
    IDCompositionTransform(IDCompositionTransformVtbl) {
    fn SetScaleX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetScaleX_1(
        scaleX: c_float,
    ) -> HRESULT,
    fn SetScaleY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetScaleY_1(
        scaleY: c_float,
    ) -> HRESULT,
    fn SetCenterX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetCenterX_1(
        centerX: c_float,
    ) -> HRESULT,
    fn SetCenterY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetCenterY_1(
        centerY: c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x641ed83c, 0xae96, 0x46c5, 0x90, 0xdc, 0x32, 0x77, 0x4c, 0xc5, 0xc6, 0xd5)]
interface IDCompositionRotateTransform(IDCompositionRotateTransformVtbl):
    IDCompositionTransform(IDCompositionTransformVtbl) {
    fn SetAngle_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetAngle_1(
        angle: c_float,
    ) -> HRESULT,
    fn SetCenterX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetCenterX_1(
        centerX: c_float,
    ) -> HRESULT,
    fn SetCenterY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetCenterY_1(
        centerY: c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe57aa735, 0xdcdb, 0x4c72, 0x9c, 0x61, 0x05, 0x91, 0xf5, 0x88, 0x89, 0xee)]
interface IDCompositionSkewTransform(IDCompositionSkewTransformVtbl):
    IDCompositionTransform(IDCompositionTransformVtbl) {
    fn SetAngleX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetAngleX_1(
        angleX: c_float,
    ) -> HRESULT,
    fn SetAngleY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetAngleY_1(
        angleY: c_float,
    ) -> HRESULT,
    fn SetCenterX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetCenterX_1(
        centerX: c_float,
    ) -> HRESULT,
    fn SetCenterY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetCenterY_1(
        centerY: c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x16cdff07, 0xc503, 0x419c, 0x83, 0xf2, 0x09, 0x65, 0xc7, 0xaf, 0x1f, 0xa6)]
interface IDCompositionMatrixTransform(IDCompositionMatrixTransformVtbl):
    IDCompositionTransform(IDCompositionTransformVtbl) {
    fn SetMatrix(
        matrix: *const D2D_MATRIX_3X2_F,
    ) -> HRESULT,
    fn SetMatrixElement_2(
        row: c_int,
        column: c_int,
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetMatrixElement_1(
        row: c_int,
        column: c_int,
        value: c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa7929a74, 0xe6b2, 0x4bd6, 0x8b, 0x95, 0x40, 0x40, 0x11, 0x9c, 0xa3, 0x4d)]
interface IDCompositionEffectGroup(IDCompositionEffectGroupVtbl):
    IDCompositionEffect(IDCompositionEffectVtbl) {
    fn SetOpacity_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetOpacity_1(
        opacity: c_float,
    ) -> HRESULT,
    fn SetTransform3D(
        transform3D: *const IDCompositionTransform3D,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x91636d4b, 0x9ba1, 0x4532, 0xaa, 0xf7, 0xe3, 0x34, 0x49, 0x94, 0xd7, 0x88)]
interface IDCompositionTranslateTransform3D(IDCompositionTranslateTransform3DVtbl):
    IDCompositionTransform3D(IDCompositionTransform3DVtbl) {
    fn SetOffsetX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetOffsetX_1(
        offsetX: c_float,
    ) -> HRESULT,
    fn SetOffsetY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetOffsetY_1(
        offsetY: c_float,
    ) -> HRESULT,
    fn SetOffsetZ_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetOffsetZ_1(
        offsetZ: c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2a9e9ead, 0x364b, 0x4b15, 0xa7, 0xc4, 0xa1, 0x99, 0x7f, 0x78, 0xb3, 0x89)]
interface IDCompositionScaleTransform3D(IDCompositionScaleTransform3DVtbl):
    IDCompositionTransform3D(IDCompositionTransform3DVtbl) {
    fn SetScaleX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetScaleX_1(
        scaleX: c_float,
    ) -> HRESULT,
    fn SetScaleY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetScaleY_1(
        scaleY: c_float,
    ) -> HRESULT,
    fn SetScaleZ_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetScaleZ_1(
        scaleZ: c_float,
    ) -> HRESULT,
    fn SetCenterX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetCenterX_1(
        centerX: c_float,
    ) -> HRESULT,
    fn SetCenterY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetCenterY_1(
        centerY: c_float,
    ) -> HRESULT,
    fn SetCenterZ_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetCenterZ_1(
        centerZ: c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd8f5b23f, 0xd429, 0x4a91, 0xb5, 0x5a, 0xd2, 0xf4, 0x5f, 0xd7, 0x5b, 0x18)]
interface IDCompositionRotateTransform3D(IDCompositionRotateTransform3DVtbl):
    IDCompositionTransform3D(IDCompositionTransform3DVtbl) {
    fn SetAngle_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetAngle_1(
        angle: c_float,
    ) -> HRESULT,
    fn SetAxisX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetAxisX_1(
        axisX: c_float,
    ) -> HRESULT,
    fn SetAxisY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetAxisY_1(
        axisY: c_float,
    ) -> HRESULT,
    fn SetAxisZ_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetAxisZ_1(
        axisZ: c_float,
    ) -> HRESULT,
    fn SetCenterX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetCenterX_1(
        centerX: c_float,
    ) -> HRESULT,
    fn SetCenterY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetCenterY_1(
        centerY: c_float,
    ) -> HRESULT,
    fn SetCenterZ_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetCenterZ_1(
        centerZ: c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4b3363f0, 0x643b, 0x41b7, 0xb6, 0xe0, 0xcc, 0xf2, 0x2d, 0x34, 0x46, 0x7c)]
interface IDCompositionMatrixTransform3D(IDCompositionMatrixTransform3DVtbl):
    IDCompositionTransform3D(IDCompositionTransform3DVtbl) {
    fn SetMatrix(
        matrix: *const D3DMATRIX,
    ) -> HRESULT,
    fn SetMatrixElement_2(
        row: c_int,
        column: c_int,
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetMatrixElement_1(
        row: c_int,
        column: c_int,
        value: c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x64ac3703, 0x9d3f, 0x45ec, 0xa1, 0x09, 0x7c, 0xac, 0x0e, 0x7a, 0x13, 0xa7)]
interface IDCompositionClip(IDCompositionClipVtbl): IUnknown(IUnknownVtbl) {}}
RIDL!{#[uuid(0x9842ad7d, 0xd9cf, 0x4908, 0xae, 0xd7, 0x48, 0xb5, 0x1d, 0xa5, 0xe7, 0xc2)]
interface IDCompositionRectangleClip(IDCompositionRectangleClipVtbl):
    IDCompositionClip(IDCompositionClipVtbl) {
    fn SetLeft_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetLeft_1(
        left: c_float,
    ) -> HRESULT,
    fn SetTop_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetTop_1(
        top: c_float,
    ) -> HRESULT,
    fn SetRight_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetRight_1(
        right: c_float,
    ) -> HRESULT,
    fn SetBottom_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetBottom_1(
        bottom: c_float,
    ) -> HRESULT,
    fn SetTopLeftRadiusX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetTopLeftRadiusX_1(
        radius: c_float,
    ) -> HRESULT,
    fn SetTopLeftRadiusY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetTopLeftRadiusY_1(
        radius: c_float,
    ) -> HRESULT,
    fn SetTopRightRadiusX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetTopRightRadiusX_1(
        radius: c_float,
    ) -> HRESULT,
    fn SetTopRightRadiusY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetTopRightRadiusY_1(
        radius: c_float,
    ) -> HRESULT,
    fn SetBottomLeftRadiusX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetBottomLeftRadiusX_1(
        radius: c_float,
    ) -> HRESULT,
    fn SetBottomLeftRadiusY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetBottomLeftRadiusY_1(
        radius: c_float,
    ) -> HRESULT,
    fn SetBottomRightRadiusX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetBottomRightRadiusX_1(
        radius: c_float,
    ) -> HRESULT,
    fn SetBottomRightRadiusY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetBottomRightRadiusY_1(
        radius: c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xbb8a4953, 0x2c99, 0x4f5a, 0x96, 0xf5, 0x48, 0x19, 0x02, 0x7f, 0xa3, 0xac)]
interface IDCompositionSurface(IDCompositionSurfaceVtbl): IUnknown(IUnknownVtbl) {
    fn BeginDraw(
        updateRect: *const RECT,
        iid: REFIID,
        updateObject: *mut *mut c_void,
        updateOffset: *mut POINT,
    ) -> HRESULT,
    fn EndDraw() -> HRESULT,
    fn SuspendDraw() -> HRESULT,
    fn ResumeDraw() -> HRESULT,
    fn Scroll(
        scrollRect: *const RECT,
        clipRect: *const RECT,
        offsetX: c_int,
        offsetY: c_int,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xae471c51, 0x5f53, 0x4a24, 0x8d, 0x3e, 0xd0, 0xc3, 0x9c, 0x30, 0xb3, 0xf0)]
interface IDCompositionVirtualSurface(IDCompositionVirtualSurfaceVtbl):
    IDCompositionSurface(IDCompositionSurfaceVtbl) {
    fn Resize(
        width: UINT,
        height: UINT,
    ) -> HRESULT,
    fn Trim(
        rectangles: *const RECT,
        count: UINT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x75f6468d, 0x1b8e, 0x447c, 0x9b, 0xc6, 0x75, 0xfe, 0xa8, 0x0b, 0x5b, 0x25)]
interface IDCompositionDevice2(IDCompositionDevice2Vtbl): IUnknown(IUnknownVtbl) {
    fn Commit() -> HRESULT,
    fn WaitForCommitCompletion() -> HRESULT,
    fn GetFrameStatistics(
        statistics: *mut DCOMPOSITION_FRAME_STATISTICS,
    ) -> HRESULT,
    fn CreateVisual(
        visual: *mut *mut IDCompositionVisual2,
    ) -> HRESULT,
    fn CreateSurfaceFactory(
        renderingDevice: *const IUnknown,
        surfaceFactory: *mut *mut IDCompositionSurfaceFactory,
    ) -> HRESULT,
    fn CreateSurface(
        width: UINT,
        height: UINT,
        pixelFormat: DXGI_FORMAT,
        alphaMode: DXGI_ALPHA_MODE,
        surface: *mut *mut IDCompositionSurface,
    ) -> HRESULT,
    fn CreateVirtualSurface(
        initialWidth: UINT,
        initialHeight: UINT,
        pixelFormat: DXGI_FORMAT,
        alphaMode: DXGI_ALPHA_MODE,
        virtualSurface: *mut *mut IDCompositionVirtualSurface,
    ) -> HRESULT,
    fn CreateTranslateTransform(
        translateTransform: *mut *mut IDCompositionTranslateTransform,
    ) -> HRESULT,
    fn CreateScaleTransform(
        scaleTransform: *mut *mut IDCompositionScaleTransform,
    ) -> HRESULT,
    fn CreateRotateTransform(
        rotateTransform: *mut *mut IDCompositionRotateTransform,
    ) -> HRESULT,
    fn CreateSkewTransform(
        skewTransform: *mut *mut IDCompositionSkewTransform,
    ) -> HRESULT,
    fn CreateMatrixTransform(
        matrixTransform: *mut *mut IDCompositionMatrixTransform,
    ) -> HRESULT,
    fn CreateTransformGroup(
        transforms: *const *const IDCompositionTransform,
        elements: UINT,
        transformGroup: *mut *mut IDCompositionTransform,
    ) -> HRESULT,
    fn CreateTranslateTransform3D(
        translateTransform3D: *mut *mut IDCompositionTranslateTransform3D,
    ) -> HRESULT,
    fn CreateScaleTransform3D(
        scaleTransform3D: *mut *mut IDCompositionScaleTransform3D,
    ) -> HRESULT,
    fn CreateRotateTransform3D(
        rotateTransform3D: *mut *mut IDCompositionRotateTransform3D,
    ) -> HRESULT,
    fn CreateMatrixTransform3D(
        matrixTransform3D: *mut *mut IDCompositionMatrixTransform3D,
    ) -> HRESULT,
    fn CreateTransform3DGroup(
        transforms3D: *const *const IDCompositionTransform3D,
        elements: UINT,
        transform3DGroup: *mut *mut IDCompositionTransform3D,
    ) -> HRESULT,
    fn CreateEffectGroup(
        effectGroup: *mut *mut IDCompositionEffectGroup,
    ) -> HRESULT,
    fn CreateRectangleClip(
        clip: *mut *mut IDCompositionRectangleClip,
    ) -> HRESULT,
    fn CreateAnimation(
        animation: *mut *mut IDCompositionAnimation,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x5f4633fe, 0x1e08, 0x4cb8, 0x8c, 0x75, 0xce, 0x24, 0x33, 0x3f, 0x56, 0x02)]
interface IDCompositionDesktopDevice(IDCompositionDesktopDeviceVtbl):
    IDCompositionDevice2(IDCompositionDevice2Vtbl) {
    fn CreateTargetForHwnd(
        hwnd: HWND,
        topmost: BOOL,
        target: *mut *mut IDCompositionTarget,
    ) -> HRESULT,
    fn CreateSurfaceFromHandle(
        handle: HANDLE,
        surface: *mut *mut IUnknown,
    ) -> HRESULT,
    fn CreateSurfaceFromHwnd(
        hwnd: HWND,
        surface: *mut *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa1a3c64a, 0x224f, 0x4a81, 0x97, 0x73, 0x4f, 0x03, 0xa8, 0x9d, 0x3c, 0x6c)]
interface IDCompositionDeviceDebug(IDCompositionDeviceDebugVtbl): IUnknown(IUnknownVtbl) {
    fn EnableDebugCounters() -> HRESULT,
    fn DisableDebugCounters() -> HRESULT,
}}
RIDL!{#[uuid(0xe334bc12, 0x3937, 0x4e02, 0x85, 0xeb, 0xfc, 0xf4, 0xeb, 0x30, 0xd2, 0xc8)]
interface IDCompositionSurfaceFactory(IDCompositionSurfaceFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn CreateSurface(
        width: UINT,
        height: UINT,
        pixelFormat: DXGI_FORMAT,
        alphaMode: DXGI_ALPHA_MODE,
        surface: *mut *mut IDCompositionSurface,
    ) -> HRESULT,
    fn CreateVirtualSurface(
        initialWidth: UINT,
        initialHeight: UINT,
        pixelFormat: DXGI_FORMAT,
        alphaMode: DXGI_ALPHA_MODE,
        virtualSurface: *mut *mut IDCompositionVirtualSurface,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe8de1639, 0x4331, 0x4b26, 0xbc, 0x5f, 0x6a, 0x32, 0x1d, 0x34, 0x7a, 0x85)]
interface IDCompositionVisual2(IDCompositionVisual2Vtbl):
    IDCompositionVisual(IDCompositionVisualVtbl) {
    fn SetOpacityMode(
        mode: DCOMPOSITION_OPACITY_MODE,
    ) -> HRESULT,
    fn SetBackFaceVisibility(
        visibility: DCOMPOSITION_BACKFACE_VISIBILITY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xfed2b808, 0x5eb4, 0x43a0, 0xae, 0xa3, 0x35, 0xf6, 0x52, 0x80, 0xf9, 0x1b)]
interface IDCompositionVisualDebug(IDCompositionVisualDebugVtbl):
    IDCompositionVisual2(IDCompositionVisual2Vtbl) {
    fn EnableHeatMap(
        color: *const D2D1_COLOR_F,
    ) -> HRESULT,
    fn DisableHeatMap() -> HRESULT,
    fn EnableRedrawRegions() -> HRESULT,
    fn DisableRedrawRegions() -> HRESULT,
}}
RIDL!{#[uuid(0x2775f462, 0xb6c1, 0x4015, 0xb0, 0xbe, 0xb3, 0xe7, 0xd6, 0xa4, 0x97, 0x6d)]
interface IDCompositionVisual3(IDCompositionVisual3Vtbl):
    IDCompositionVisualDebug(IDCompositionVisualDebugVtbl) {
    fn SetDepthMode(
        mode: DCOMPOSITION_DEPTH_MODE,
    ) -> HRESULT,
    fn SetOffsetZ_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetOffsetZ_1(
        offsetZ: c_float,
    ) -> HRESULT,
    fn SetOpacity_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetOpacity_1(
        opacity: c_float,
    ) -> HRESULT,
    fn SetTransform_2(
        transform: *const IDCompositionTransform3D,
    ) -> HRESULT,
    fn SetTransform_1(
        matrix: *const D2D_MATRIX_4X4_F,
    ) -> HRESULT,
    fn SetVisible(
        visible: BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0987cb06, 0xf916, 0x48bf, 0x8d, 0x35, 0xce, 0x76, 0x41, 0x78, 0x1b, 0xd9)]
interface IDCompositionDevice3(IDCompositionDevice3Vtbl):
    IDCompositionDevice2(IDCompositionDevice2Vtbl) {
    fn CreateGaussianBlurEffect(
        gaussianBlurEffect: *mut *mut IDCompositionGaussianBlurEffect,
    ) -> HRESULT,
    fn CreateBrightnessEffect(
        brightnessEffect: *mut *mut IDCompositionBrightnessEffect,
    ) -> HRESULT,
    fn CreateColorMatrixEffect(
        colorMatrixEffect: *mut *mut IDCompositionColorMatrixEffect,
    ) -> HRESULT,
    fn CreateShadowEffect(
        shadowEffect: *mut *mut IDCompositionShadowEffect,
    ) -> HRESULT,
    fn CreateHueRotationEffect(
        hueRotationEffect: *mut *mut IDCompositionHueRotationEffect,
    ) -> HRESULT,
    fn CreateSaturationEffect(
        saturationEffect: *mut *mut IDCompositionSaturationEffect,
    ) -> HRESULT,
    fn CreateTurbulenceEffect(
        turbulenceEffect: *mut *mut IDCompositionTurbulenceEffect,
    ) -> HRESULT,
    fn CreateLinearTransferEffect(
        linearTransferEffect: *mut *mut IDCompositionLinearTransferEffect,
    ) -> HRESULT,
    fn CreateTableTransferEffect(
        tableTransferEffect: *mut *mut IDCompositionTableTransferEffect,
    ) -> HRESULT,
    fn CreateCompositeEffect(
        compositeEffect: *mut *mut IDCompositionCompositeEffect,
    ) -> HRESULT,
    fn CreateBlendEffect(
        blendEffect: *mut *mut IDCompositionBlendEffect,
    ) -> HRESULT,
    fn CreateArithmeticCompositeEffect(
        arithmeticCompositeEffect: *mut *mut IDCompositionArithmeticCompositeEffect,
    ) -> HRESULT,
    fn CreateAffineTransform2DEffect(
        affineTransform2dEffect: *mut *mut IDCompositionAffineTransform2DEffect,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x30c421d5, 0x8cb2, 0x4e9f, 0xb1, 0x33, 0x37, 0xbe, 0x27, 0x0d, 0x4a, 0xc2)]
interface IDCompositionFilterEffect(IDCompositionFilterEffectVtbl):
    IDCompositionEffect(IDCompositionEffectVtbl) {
    fn SetInput(
        index: UINT,
        input: *const IUnknown,
        flags: UINT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x45d4d0b7, 0x1bd4, 0x454e, 0x88, 0x94, 0x2b, 0xfa, 0x68, 0x44, 0x30, 0x33)]
interface IDCompositionGaussianBlurEffect(IDCompositionGaussianBlurEffectVtbl):
    IDCompositionFilterEffect(IDCompositionFilterEffectVtbl) {
    fn SetStandardDeviation_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetStandardDeviation_1(
        amount: c_float,
    ) -> HRESULT,
    fn SetBorderMode(
        mode: D2D1_BORDER_MODE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6027496e, 0xcb3a, 0x49ab, 0x93, 0x4f, 0xd7, 0x98, 0xda, 0x4f, 0x7d, 0xa6)]
interface IDCompositionBrightnessEffect(IDCompositionBrightnessEffectVtbl):
    IDCompositionFilterEffect(IDCompositionFilterEffectVtbl) {
    fn SetWhitePoint(
        whitePoint: *const D2D1_VECTOR_2F,
    ) -> HRESULT,
    fn SetBlackPoint(
        blackPoint: *const D2D1_VECTOR_2F,
    ) -> HRESULT,
    fn SetWhitePointX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetWhitePointX_1(
        whitePointX: c_float,
    ) -> HRESULT,
    fn SetWhitePointY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetWhitePointY_1(
        whitePointY: c_float,
    ) -> HRESULT,
    fn SetBlackPointX_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetBlackPointX_1(
        blackPointX: c_float,
    ) -> HRESULT,
    fn SetBlackPointY_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetBlackPointY_1(
        blackPointY: c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc1170a22, 0x3ce2, 0x4966, 0x90, 0xd4, 0x55, 0x40, 0x8b, 0xfc, 0x84, 0xc4)]
interface IDCompositionColorMatrixEffect(IDCompositionColorMatrixEffectVtbl):
    IDCompositionFilterEffect(IDCompositionFilterEffectVtbl) {
    fn SetMatrix(
        matrix: *const D2D1_MATRIX_5X4_F,
    ) -> HRESULT,
    fn SetMatrixElement_2(
        row: c_int,
        column: c_int,
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetMatrixElement_1(
        row: c_int,
        column: c_int,
        value: c_float,
    ) -> HRESULT,
    fn SetAlphaMode(
        mode: D2D1_COLORMATRIX_ALPHA_MODE,
    ) -> HRESULT,
    fn SetClampOutput(
        clamp: BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4ad18ac0, 0xcfd2, 0x4c2f, 0xbb, 0x62, 0x96, 0xe5, 0x4f, 0xdb, 0x68, 0x79)]
interface IDCompositionShadowEffect(IDCompositionShadowEffectVtbl):
    IDCompositionFilterEffect(IDCompositionFilterEffectVtbl) {
    fn SetStandardDeviation_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetStandardDeviation_1(
        amount: c_float,
    ) -> HRESULT,
    fn SetColor(
        color: *const D2D1_VECTOR_4F,
    ) -> HRESULT,
    fn SetRed_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetRed_1(
        amount: c_float,
    ) -> HRESULT,
    fn SetGreen_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetGreen_1(
        amount: c_float,
    ) -> HRESULT,
    fn SetBlue_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetBlue_1(
        amount: c_float,
    ) -> HRESULT,
    fn SetAlpha_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetAlpha_1(
        amount: c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6db9f920, 0x0770, 0x4781, 0xb0, 0xc6, 0x38, 0x19, 0x12, 0xf9, 0xd1, 0x67)]
interface IDCompositionHueRotationEffect(IDCompositionHueRotationEffectVtbl):
    IDCompositionFilterEffect(IDCompositionFilterEffectVtbl) {
    // Changes the angle of rotation
    fn SetAngle_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetAngle_1(
        amountDegrees: c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa08debda, 0x3258, 0x4fa4, 0x9f, 0x16, 0x91, 0x74, 0xd3, 0xfe, 0x93, 0xb1)]
interface IDCompositionSaturationEffect(IDCompositionSaturationEffectVtbl):
    IDCompositionFilterEffect(IDCompositionFilterEffectVtbl) {
    // Changes the amount of saturation to be applied.
    fn SetSaturation_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetSaturation_1(
        ratio: c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa6a55bda, 0xc09c, 0x49f3, 0x91, 0x93, 0xa4, 0x19, 0x22, 0xc8, 0x97, 0x15)]
interface IDCompositionTurbulenceEffect(IDCompositionTurbulenceEffectVtbl):
    IDCompositionFilterEffect(IDCompositionFilterEffectVtbl) {
    fn SetOffset(
        offset: *const D2D1_VECTOR_2F,
    ) -> HRESULT,
    fn SetBaseFrequency(
        frequency: *const D2D1_VECTOR_2F,
    ) -> HRESULT,
    fn SetSize(
        size: *const D2D1_VECTOR_2F,
    ) -> HRESULT,
    fn SetNumOctaves(
        numOctaves: UINT,
    ) -> HRESULT,
    fn SetSeed(
        seed: UINT,
    ) -> HRESULT,
    fn SetNoise(
        noise: D2D1_TURBULENCE_NOISE,
    ) -> HRESULT,
    fn SetStitchable(
        stitchable: BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4305ee5b, 0xc4a0, 0x4c88, 0x93, 0x85, 0x67, 0x12, 0x4e, 0x01, 0x76, 0x83)]
interface IDCompositionLinearTransferEffect(IDCompositionLinearTransferEffectVtbl):
    IDCompositionFilterEffect(IDCompositionFilterEffectVtbl) {
    fn SetRedYIntercept_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetRedYIntercept_1(
        redYIntercept: c_float,
    ) -> HRESULT,
    fn SetRedSlope_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetRedSlope_1(
        redSlope: c_float,
    ) -> HRESULT,
    fn SetRedDisable(
        redDisable: BOOL,
    ) -> HRESULT,
    fn SetGreenYIntercept_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetGreenYIntercept_1(
        greenYIntercept: c_float,
    ) -> HRESULT,
    fn SetGreenSlope_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetGreenSlope_1(
        greenSlope: c_float,
    ) -> HRESULT,
    fn SetGreenDisable(
        greenDisable: BOOL,
    ) -> HRESULT,
    fn SetBlueYIntercept_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetBlueYIntercept_1(
        blueYIntercept: c_float,
    ) -> HRESULT,
    fn SetBlueSlope_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetBlueSlope_1(
        blueSlope: c_float,
    ) -> HRESULT,
    fn SetBlueDisable(
        blueDisable: BOOL,
    ) -> HRESULT,
    fn SetAlphaYIntercept_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetAlphaYIntercept_1(
        alphaYIntercept: c_float,
    ) -> HRESULT,
    fn SetAlphaSlope_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetAlphaSlope_1(
        alphaSlope: c_float,
    ) -> HRESULT,
    fn SetAlphaDisable(
        alphaDisable: BOOL,
    ) -> HRESULT,
    fn SetClampOutput(
        clampOutput: BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x9b7e82e2, 0x69c5, 0x4eb4, 0xa5, 0xf5, 0xa7, 0x03, 0x3f, 0x51, 0x32, 0xcd)]
interface IDCompositionTableTransferEffect(IDCompositionTableTransferEffectVtbl):
    IDCompositionFilterEffect(IDCompositionFilterEffectVtbl) {
    fn SetRedTable(
        tableValues: *const c_float,
        count: UINT,
    ) -> HRESULT,
    fn SetGreenTable(
        tableValues: *const c_float,
        count: UINT,
    ) -> HRESULT,
    fn SetBlueTable(
        tableValues: *const c_float,
        count: UINT,
    ) -> HRESULT,
    fn SetAlphaTable(
        tableValues: *const c_float,
        count: UINT,
    ) -> HRESULT,
    fn SetRedDisable(
        redDisable: BOOL,
    ) -> HRESULT,
    fn SetGreenDisable(
        greenDisable: BOOL,
    ) -> HRESULT,
    fn SetBlueDisable(
        blueDisable: BOOL,
    ) -> HRESULT,
    fn SetAlphaDisable(
        alphaDisable: BOOL,
    ) -> HRESULT,
    fn SetClampOutput(
        clampOutput: BOOL,
    ) -> HRESULT,
    fn SetRedTableValue_2(
        index: UINT,
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetRedTableValue_1(
        index: UINT,
        value: c_float,
    ) -> HRESULT,
    fn SetGreenTableValue_2(
        index: UINT,
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetGreenTableValue_1(
        index: UINT,
        value: c_float,
    ) -> HRESULT,
    fn SetBlueTableValue_2(
        index: UINT,
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetBlueTableValue_1(
        index: UINT,
        value: c_float,
    ) -> HRESULT,
    fn SetAlphaTableValue_2(
        index: UINT,
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetAlphaTableValue_1(
        index: UINT,
        value: c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x576616c0, 0xa231, 0x494d, 0xa3, 0x8d, 0x00, 0xfd, 0x5e, 0xc4, 0xdb, 0x46)]
interface IDCompositionCompositeEffect(IDCompositionCompositeEffectVtbl):
    IDCompositionFilterEffect(IDCompositionFilterEffectVtbl) {
    fn SetMode(
        mode: D2D1_COMPOSITE_MODE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x33ecdc0a, 0x578a, 0x4a11, 0x9c, 0x14, 0x0c, 0xb9, 0x05, 0x17, 0xf9, 0xc5)]
interface IDCompositionBlendEffect(IDCompositionBlendEffectVtbl):
    IDCompositionFilterEffect(IDCompositionFilterEffectVtbl) {
    fn SetMode(
        mode: D2D1_BLEND_MODE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3b67dfa8, 0xe3dd, 0x4e61, 0xb6, 0x40, 0x46, 0xc2, 0xf3, 0xd7, 0x39, 0xdc)]
interface IDCompositionArithmeticCompositeEffect(IDCompositionArithmeticCompositeEffectVtbl):
    IDCompositionFilterEffect(IDCompositionFilterEffectVtbl) {
    fn SetCoefficients(
        coefficients: *const D2D1_VECTOR_4F,
    ) -> HRESULT,
    fn SetClampOutput(
        clampoutput: BOOL,
    ) -> HRESULT,
    fn SetCoefficient1_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetCoefficient1_1(
        Coeffcient1: c_float,
    ) -> HRESULT,
    fn SetCoefficient2_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetCoefficient2_1(
        Coefficient2: c_float,
    ) -> HRESULT,
    fn SetCoefficient3_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetCoefficient3_1(
        Coefficient3: c_float,
    ) -> HRESULT,
    fn SetCoefficient4_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetCoefficient4_1(
        Coefficient4: c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0b74b9e8, 0xcdd6, 0x492f, 0xbb, 0xbc, 0x5e, 0xd3, 0x21, 0x57, 0x02, 0x6d)]
interface IDCompositionAffineTransform2DEffect(IDCompositionAffineTransform2DEffectVtbl):
    IDCompositionFilterEffect(IDCompositionFilterEffectVtbl) {
    fn SetInterpolationMode(
        interpolationMode: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE,
    ) -> HRESULT,
    fn SetBorderMode(
        borderMode: D2D1_BORDER_MODE,
    ) -> HRESULT,
    fn SetTransformMatrix(
        transformMatrix: *const D2D1_MATRIX_3X2_F,
    ) -> HRESULT,
    fn SetTransformMatrixElement_2(
        row: c_int,
        column: c_int,
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetTransformMatrixElement_1(
        row: c_int,
        column: c_int,
        value: c_float,
    ) -> HRESULT,
    fn SetSharpness_2(
        animation: *const IDCompositionAnimation,
    ) -> HRESULT,
    fn SetSharpness_1(
        sharpness: c_float,
    ) -> HRESULT,
}}
