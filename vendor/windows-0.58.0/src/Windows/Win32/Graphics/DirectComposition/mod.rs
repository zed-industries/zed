#[inline]
pub unsafe fn DCompositionAttachMouseDragToHwnd<P0, P1, P2>(visual: P0, hwnd: P1, enable: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<IDCompositionVisual>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dcomp.dll" "system" fn DCompositionAttachMouseDragToHwnd(visual : * mut core::ffi::c_void, hwnd : super::super::Foundation:: HWND, enable : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    DCompositionAttachMouseDragToHwnd(visual.param().abi(), hwnd.param().abi(), enable.param().abi()).ok()
}
#[inline]
pub unsafe fn DCompositionAttachMouseWheelToHwnd<P0, P1, P2>(visual: P0, hwnd: P1, enable: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<IDCompositionVisual>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dcomp.dll" "system" fn DCompositionAttachMouseWheelToHwnd(visual : * mut core::ffi::c_void, hwnd : super::super::Foundation:: HWND, enable : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    DCompositionAttachMouseWheelToHwnd(visual.param().abi(), hwnd.param().abi(), enable.param().abi()).ok()
}
#[inline]
pub unsafe fn DCompositionBoostCompositorClock<P0>(enable: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dcomp.dll" "system" fn DCompositionBoostCompositorClock(enable : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    DCompositionBoostCompositorClock(enable.param().abi()).ok()
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn DCompositionCreateDevice<P0, T>(dxgidevice: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::Dxgi::IDXGIDevice>,
    T: windows_core::Interface,
{
    windows_targets::link!("dcomp.dll" "system" fn DCompositionCreateDevice(dxgidevice : * mut core::ffi::c_void, iid : *const windows_core::GUID, dcompositiondevice : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    DCompositionCreateDevice(dxgidevice.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn DCompositionCreateDevice2<P0, T>(renderingdevice: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_targets::link!("dcomp.dll" "system" fn DCompositionCreateDevice2(renderingdevice : * mut core::ffi::c_void, iid : *const windows_core::GUID, dcompositiondevice : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    DCompositionCreateDevice2(renderingdevice.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn DCompositionCreateDevice3<P0, T>(renderingdevice: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_targets::link!("dcomp.dll" "system" fn DCompositionCreateDevice3(renderingdevice : * mut core::ffi::c_void, iid : *const windows_core::GUID, dcompositiondevice : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    DCompositionCreateDevice3(renderingdevice.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn DCompositionCreateSurfaceHandle(desiredaccess: u32, securityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("dcomp.dll" "system" fn DCompositionCreateSurfaceHandle(desiredaccess : u32, securityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, surfacehandle : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DCompositionCreateSurfaceHandle(desiredaccess, core::mem::transmute(securityattributes.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DCompositionGetFrameId(frameidtype: COMPOSITION_FRAME_ID_TYPE) -> windows_core::Result<u64> {
    windows_targets::link!("dcomp.dll" "system" fn DCompositionGetFrameId(frameidtype : COMPOSITION_FRAME_ID_TYPE, frameid : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DCompositionGetFrameId(frameidtype, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DCompositionGetStatistics(frameid: u64, framestats: *mut COMPOSITION_FRAME_STATS, targetidcount: u32, targetids: Option<*mut COMPOSITION_TARGET_ID>, actualtargetidcount: Option<*mut u32>) -> windows_core::Result<()> {
    windows_targets::link!("dcomp.dll" "system" fn DCompositionGetStatistics(frameid : u64, framestats : *mut COMPOSITION_FRAME_STATS, targetidcount : u32, targetids : *mut COMPOSITION_TARGET_ID, actualtargetidcount : *mut u32) -> windows_core::HRESULT);
    DCompositionGetStatistics(frameid, framestats, targetidcount, core::mem::transmute(targetids.unwrap_or(std::ptr::null_mut())), core::mem::transmute(actualtargetidcount.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn DCompositionGetTargetStatistics(frameid: u64, targetid: *const COMPOSITION_TARGET_ID, targetstats: *mut COMPOSITION_TARGET_STATS) -> windows_core::Result<()> {
    windows_targets::link!("dcomp.dll" "system" fn DCompositionGetTargetStatistics(frameid : u64, targetid : *const COMPOSITION_TARGET_ID, targetstats : *mut COMPOSITION_TARGET_STATS) -> windows_core::HRESULT);
    DCompositionGetTargetStatistics(frameid, targetid, targetstats).ok()
}
#[inline]
pub unsafe fn DCompositionWaitForCompositorClock(handles: Option<&[super::super::Foundation::HANDLE]>, timeoutinms: u32) -> u32 {
    windows_targets::link!("dcomp.dll" "system" fn DCompositionWaitForCompositorClock(count : u32, handles : *const super::super::Foundation:: HANDLE, timeoutinms : u32) -> u32);
    DCompositionWaitForCompositorClock(handles.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(handles.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), timeoutinms)
}
windows_core::imp::define_interface!(IDCompositionAffineTransform2DEffect, IDCompositionAffineTransform2DEffect_Vtbl, 0x0b74b9e8_cdd6_492f_bbbc_5ed32157026d);
impl core::ops::Deref for IDCompositionAffineTransform2DEffect {
    type Target = IDCompositionFilterEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionAffineTransform2DEffect, windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl IDCompositionAffineTransform2DEffect {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetInterpolationMode(&self, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInterpolationMode)(windows_core::Interface::as_raw(self), interpolationmode).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBorderMode(&self, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBorderMode)(windows_core::Interface::as_raw(self), bordermode).ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransformMatrix(&self, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTransformMatrix)(windows_core::Interface::as_raw(self), transformmatrix).ok()
    }
    pub unsafe fn SetTransformMatrixElement<P0>(&self, row: i32, column: i32, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetTransformMatrixElement)(windows_core::Interface::as_raw(self), row, column, animation.param().abi()).ok()
    }
    pub unsafe fn SetTransformMatrixElement2(&self, row: i32, column: i32, value: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTransformMatrixElement2)(windows_core::Interface::as_raw(self), row, column, value).ok()
    }
    pub unsafe fn SetSharpness<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetSharpness)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetSharpness2(&self, sharpness: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSharpness2)(windows_core::Interface::as_raw(self), sharpness).ok()
    }
}
#[repr(C)]
pub struct IDCompositionAffineTransform2DEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetInterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetInterpolationMode: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBorderMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::Direct2D::Common::D2D1_BORDER_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBorderMode: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransformMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransformMatrix: usize,
    pub SetTransformMatrixElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTransformMatrixElement2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, f32) -> windows_core::HRESULT,
    pub SetSharpness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSharpness2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionAnimation, IDCompositionAnimation_Vtbl, 0xcbfd91d9_51b2_45e4_b3de_d19ccfb863c5);
impl core::ops::Deref for IDCompositionAnimation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionAnimation, windows_core::IUnknown);
impl IDCompositionAnimation {
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetAbsoluteBeginTime(&self, begintime: i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAbsoluteBeginTime)(windows_core::Interface::as_raw(self), begintime).ok()
    }
    pub unsafe fn AddCubic(&self, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddCubic)(windows_core::Interface::as_raw(self), beginoffset, constantcoefficient, linearcoefficient, quadraticcoefficient, cubiccoefficient).ok()
    }
    pub unsafe fn AddSinusoidal(&self, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddSinusoidal)(windows_core::Interface::as_raw(self), beginoffset, bias, amplitude, frequency, phase).ok()
    }
    pub unsafe fn AddRepeat(&self, beginoffset: f64, durationtorepeat: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddRepeat)(windows_core::Interface::as_raw(self), beginoffset, durationtorepeat).ok()
    }
    pub unsafe fn End(&self, endoffset: f64, endvalue: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self), endoffset, endvalue).ok()
    }
}
#[repr(C)]
pub struct IDCompositionAnimation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAbsoluteBeginTime: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub AddCubic: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f32, f32, f32, f32) -> windows_core::HRESULT,
    pub AddSinusoidal: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f32, f32, f32, f32) -> windows_core::HRESULT,
    pub AddRepeat: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionArithmeticCompositeEffect, IDCompositionArithmeticCompositeEffect_Vtbl, 0x3b67dfa8_e3dd_4e61_b640_46c2f3d739dc);
impl core::ops::Deref for IDCompositionArithmeticCompositeEffect {
    type Target = IDCompositionFilterEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionArithmeticCompositeEffect, windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl IDCompositionArithmeticCompositeEffect {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetCoefficients(&self, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCoefficients)(windows_core::Interface::as_raw(self), coefficients).ok()
    }
    pub unsafe fn SetClampOutput<P0>(&self, clampoutput: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetClampOutput)(windows_core::Interface::as_raw(self), clampoutput.param().abi()).ok()
    }
    pub unsafe fn SetCoefficient1<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetCoefficient1)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetCoefficient12(&self, coeffcient1: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCoefficient12)(windows_core::Interface::as_raw(self), coeffcient1).ok()
    }
    pub unsafe fn SetCoefficient2<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetCoefficient2)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetCoefficient22(&self, coefficient2: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCoefficient22)(windows_core::Interface::as_raw(self), coefficient2).ok()
    }
    pub unsafe fn SetCoefficient3<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetCoefficient3)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetCoefficient32(&self, coefficient3: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCoefficient32)(windows_core::Interface::as_raw(self), coefficient3).ok()
    }
    pub unsafe fn SetCoefficient4<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetCoefficient4)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetCoefficient42(&self, coefficient4: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCoefficient42)(windows_core::Interface::as_raw(self), coefficient4).ok()
    }
}
#[repr(C)]
pub struct IDCompositionArithmeticCompositeEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetCoefficients: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Direct2D::Common::D2D_VECTOR_4F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetCoefficients: usize,
    pub SetClampOutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetCoefficient1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCoefficient12: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCoefficient2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCoefficient22: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCoefficient3: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCoefficient32: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCoefficient4: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCoefficient42: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionBlendEffect, IDCompositionBlendEffect_Vtbl, 0x33ecdc0a_578a_4a11_9c14_0cb90517f9c5);
impl core::ops::Deref for IDCompositionBlendEffect {
    type Target = IDCompositionFilterEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionBlendEffect, windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl IDCompositionBlendEffect {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetMode(&self, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMode)(windows_core::Interface::as_raw(self), mode).ok()
    }
}
#[repr(C)]
pub struct IDCompositionBlendEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::Direct2D::Common::D2D1_BLEND_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMode: usize,
}
windows_core::imp::define_interface!(IDCompositionBrightnessEffect, IDCompositionBrightnessEffect_Vtbl, 0x6027496e_cb3a_49ab_934f_d798da4f7da6);
impl core::ops::Deref for IDCompositionBrightnessEffect {
    type Target = IDCompositionFilterEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionBrightnessEffect, windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl IDCompositionBrightnessEffect {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetWhitePoint(&self, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWhitePoint)(windows_core::Interface::as_raw(self), whitepoint).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBlackPoint(&self, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBlackPoint)(windows_core::Interface::as_raw(self), blackpoint).ok()
    }
    pub unsafe fn SetWhitePointX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetWhitePointX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetWhitePointX2(&self, whitepointx: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWhitePointX2)(windows_core::Interface::as_raw(self), whitepointx).ok()
    }
    pub unsafe fn SetWhitePointY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetWhitePointY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetWhitePointY2(&self, whitepointy: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWhitePointY2)(windows_core::Interface::as_raw(self), whitepointy).ok()
    }
    pub unsafe fn SetBlackPointX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetBlackPointX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetBlackPointX2(&self, blackpointx: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBlackPointX2)(windows_core::Interface::as_raw(self), blackpointx).ok()
    }
    pub unsafe fn SetBlackPointY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetBlackPointY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetBlackPointY2(&self, blackpointy: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBlackPointY2)(windows_core::Interface::as_raw(self), blackpointy).ok()
    }
}
#[repr(C)]
pub struct IDCompositionBrightnessEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetWhitePoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetWhitePoint: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBlackPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBlackPoint: usize,
    pub SetWhitePointX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWhitePointX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetWhitePointY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWhitePointY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetBlackPointX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBlackPointX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetBlackPointY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBlackPointY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionClip, IDCompositionClip_Vtbl, 0x64ac3703_9d3f_45ec_a109_7cac0e7a13a7);
impl core::ops::Deref for IDCompositionClip {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionClip, windows_core::IUnknown);
impl IDCompositionClip {}
#[repr(C)]
pub struct IDCompositionClip_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
windows_core::imp::define_interface!(IDCompositionColorMatrixEffect, IDCompositionColorMatrixEffect_Vtbl, 0xc1170a22_3ce2_4966_90d4_55408bfc84c4);
impl core::ops::Deref for IDCompositionColorMatrixEffect {
    type Target = IDCompositionFilterEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionColorMatrixEffect, windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl IDCompositionColorMatrixEffect {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetMatrix(&self, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMatrix)(windows_core::Interface::as_raw(self), matrix).ok()
    }
    pub unsafe fn SetMatrixElement<P0>(&self, row: i32, column: i32, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetMatrixElement)(windows_core::Interface::as_raw(self), row, column, animation.param().abi()).ok()
    }
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMatrixElement2)(windows_core::Interface::as_raw(self), row, column, value).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetAlphaMode(&self, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAlphaMode)(windows_core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn SetClampOutput<P0>(&self, clamp: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetClampOutput)(windows_core::Interface::as_raw(self), clamp.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDCompositionColorMatrixEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, f32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetAlphaMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetAlphaMode: usize,
    pub SetClampOutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionCompositeEffect, IDCompositionCompositeEffect_Vtbl, 0x576616c0_a231_494d_a38d_00fd5ec4db46);
impl core::ops::Deref for IDCompositionCompositeEffect {
    type Target = IDCompositionFilterEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionCompositeEffect, windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl IDCompositionCompositeEffect {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetMode(&self, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMode)(windows_core::Interface::as_raw(self), mode).ok()
    }
}
#[repr(C)]
pub struct IDCompositionCompositeEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMode: usize,
}
windows_core::imp::define_interface!(IDCompositionDelegatedInkTrail, IDCompositionDelegatedInkTrail_Vtbl, 0xc2448e9b_547d_4057_8cf5_8144ede1c2da);
impl core::ops::Deref for IDCompositionDelegatedInkTrail {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionDelegatedInkTrail, windows_core::IUnknown);
impl IDCompositionDelegatedInkTrail {
    pub unsafe fn AddTrailPoints(&self, inkpoints: &[DCompositionInkTrailPoint]) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddTrailPoints)(windows_core::Interface::as_raw(self), core::mem::transmute(inkpoints.as_ptr()), inkpoints.len().try_into().unwrap(), &mut result__).map(|| result__)
    }
    pub unsafe fn AddTrailPointsWithPrediction(&self, inkpoints: &[DCompositionInkTrailPoint], predictedinkpoints: &[DCompositionInkTrailPoint]) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddTrailPointsWithPrediction)(windows_core::Interface::as_raw(self), core::mem::transmute(inkpoints.as_ptr()), inkpoints.len().try_into().unwrap(), core::mem::transmute(predictedinkpoints.as_ptr()), predictedinkpoints.len().try_into().unwrap(), &mut result__).map(|| result__)
    }
    pub unsafe fn RemoveTrailPoints(&self, generationid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveTrailPoints)(windows_core::Interface::as_raw(self), generationid).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn StartNewTrail(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StartNewTrail)(windows_core::Interface::as_raw(self), color).ok()
    }
}
#[repr(C)]
pub struct IDCompositionDelegatedInkTrail_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddTrailPoints: unsafe extern "system" fn(*mut core::ffi::c_void, *const DCompositionInkTrailPoint, u32, *mut u32) -> windows_core::HRESULT,
    pub AddTrailPointsWithPrediction: unsafe extern "system" fn(*mut core::ffi::c_void, *const DCompositionInkTrailPoint, u32, *const DCompositionInkTrailPoint, u32, *mut u32) -> windows_core::HRESULT,
    pub RemoveTrailPoints: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub StartNewTrail: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Direct2D::Common::D2D1_COLOR_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    StartNewTrail: usize,
}
windows_core::imp::define_interface!(IDCompositionDesktopDevice, IDCompositionDesktopDevice_Vtbl, 0x5f4633fe_1e08_4cb8_8c75_ce24333f5602);
impl core::ops::Deref for IDCompositionDesktopDevice {
    type Target = IDCompositionDevice2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionDesktopDevice, windows_core::IUnknown, IDCompositionDevice2);
impl IDCompositionDesktopDevice {
    pub unsafe fn CreateTargetForHwnd<P0, P1>(&self, hwnd: P0, topmost: P1) -> windows_core::Result<IDCompositionTarget>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTargetForHwnd)(windows_core::Interface::as_raw(self), hwnd.param().abi(), topmost.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSurfaceFromHandle<P0>(&self, handle: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSurfaceFromHandle)(windows_core::Interface::as_raw(self), handle.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSurfaceFromHwnd<P0>(&self, hwnd: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSurfaceFromHwnd)(windows_core::Interface::as_raw(self), hwnd.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDCompositionDesktopDevice_Vtbl {
    pub base__: IDCompositionDevice2_Vtbl,
    pub CreateTargetForHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSurfaceFromHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSurfaceFromHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionDevice, IDCompositionDevice_Vtbl, 0xc37ea93a_e7aa_450d_b16f_9746cb0407f3);
impl core::ops::Deref for IDCompositionDevice {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionDevice, windows_core::IUnknown);
impl IDCompositionDevice {
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WaitForCommitCompletion)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFrameStatistics)(windows_core::Interface::as_raw(self), statistics).ok()
    }
    pub unsafe fn CreateTargetForHwnd<P0, P1>(&self, hwnd: P0, topmost: P1) -> windows_core::Result<IDCompositionTarget>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTargetForHwnd)(windows_core::Interface::as_raw(self), hwnd.param().abi(), topmost.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateVisual(&self) -> windows_core::Result<IDCompositionVisual> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateVisual)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionSurface> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSurface)(windows_core::Interface::as_raw(self), width, height, pixelformat, alphamode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionVirtualSurface> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateVirtualSurface)(windows_core::Interface::as_raw(self), initialwidth, initialheight, pixelformat, alphamode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSurfaceFromHandle<P0>(&self, handle: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSurfaceFromHandle)(windows_core::Interface::as_raw(self), handle.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSurfaceFromHwnd<P0>(&self, hwnd: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSurfaceFromHwnd)(windows_core::Interface::as_raw(self), hwnd.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateTranslateTransform(&self) -> windows_core::Result<IDCompositionTranslateTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTranslateTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateScaleTransform(&self) -> windows_core::Result<IDCompositionScaleTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateScaleTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRotateTransform(&self) -> windows_core::Result<IDCompositionRotateTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRotateTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSkewTransform(&self) -> windows_core::Result<IDCompositionSkewTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSkewTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateMatrixTransform(&self) -> windows_core::Result<IDCompositionMatrixTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateMatrixTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateTransformGroup(&self, transforms: &[Option<IDCompositionTransform>]) -> windows_core::Result<IDCompositionTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTransformGroup)(windows_core::Interface::as_raw(self), core::mem::transmute(transforms.as_ptr()), transforms.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateTranslateTransform3D(&self) -> windows_core::Result<IDCompositionTranslateTransform3D> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTranslateTransform3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateScaleTransform3D(&self) -> windows_core::Result<IDCompositionScaleTransform3D> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateScaleTransform3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRotateTransform3D(&self) -> windows_core::Result<IDCompositionRotateTransform3D> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRotateTransform3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateMatrixTransform3D(&self) -> windows_core::Result<IDCompositionMatrixTransform3D> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateMatrixTransform3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[Option<IDCompositionTransform3D>]) -> windows_core::Result<IDCompositionTransform3D> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTransform3DGroup)(windows_core::Interface::as_raw(self), core::mem::transmute(transforms3d.as_ptr()), transforms3d.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateEffectGroup(&self) -> windows_core::Result<IDCompositionEffectGroup> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEffectGroup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRectangleClip(&self) -> windows_core::Result<IDCompositionRectangleClip> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRectangleClip)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateAnimation(&self) -> windows_core::Result<IDCompositionAnimation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAnimation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CheckDeviceState(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckDeviceState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDCompositionDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForCommitCompletion: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetFrameStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DCOMPOSITION_FRAME_STATISTICS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetFrameStatistics: usize,
    pub CreateTargetForHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateVisual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::Dxgi::Common::DXGI_FORMAT, super::Dxgi::Common::DXGI_ALPHA_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::Dxgi::Common::DXGI_FORMAT, super::Dxgi::Common::DXGI_ALPHA_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
    pub CreateSurfaceFromHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSurfaceFromHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTranslateTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateScaleTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRotateTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSkewTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTransformGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTranslateTransform3D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateScaleTransform3D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRotateTransform3D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateMatrixTransform3D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTransform3DGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEffectGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRectangleClip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAnimation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CheckDeviceState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionDevice2, IDCompositionDevice2_Vtbl, 0x75f6468d_1b8e_447c_9bc6_75fea80b5b25);
impl core::ops::Deref for IDCompositionDevice2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionDevice2, windows_core::IUnknown);
impl IDCompositionDevice2 {
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WaitForCommitCompletion)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFrameStatistics)(windows_core::Interface::as_raw(self), statistics).ok()
    }
    pub unsafe fn CreateVisual(&self) -> windows_core::Result<IDCompositionVisual2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateVisual)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSurfaceFactory<P0>(&self, renderingdevice: P0) -> windows_core::Result<IDCompositionSurfaceFactory>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSurfaceFactory)(windows_core::Interface::as_raw(self), renderingdevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionSurface> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSurface)(windows_core::Interface::as_raw(self), width, height, pixelformat, alphamode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionVirtualSurface> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateVirtualSurface)(windows_core::Interface::as_raw(self), initialwidth, initialheight, pixelformat, alphamode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateTranslateTransform(&self) -> windows_core::Result<IDCompositionTranslateTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTranslateTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateScaleTransform(&self) -> windows_core::Result<IDCompositionScaleTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateScaleTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRotateTransform(&self) -> windows_core::Result<IDCompositionRotateTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRotateTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSkewTransform(&self) -> windows_core::Result<IDCompositionSkewTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSkewTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateMatrixTransform(&self) -> windows_core::Result<IDCompositionMatrixTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateMatrixTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateTransformGroup(&self, transforms: &[Option<IDCompositionTransform>]) -> windows_core::Result<IDCompositionTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTransformGroup)(windows_core::Interface::as_raw(self), core::mem::transmute(transforms.as_ptr()), transforms.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateTranslateTransform3D(&self) -> windows_core::Result<IDCompositionTranslateTransform3D> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTranslateTransform3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateScaleTransform3D(&self) -> windows_core::Result<IDCompositionScaleTransform3D> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateScaleTransform3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRotateTransform3D(&self) -> windows_core::Result<IDCompositionRotateTransform3D> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRotateTransform3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateMatrixTransform3D(&self) -> windows_core::Result<IDCompositionMatrixTransform3D> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateMatrixTransform3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[Option<IDCompositionTransform3D>]) -> windows_core::Result<IDCompositionTransform3D> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTransform3DGroup)(windows_core::Interface::as_raw(self), core::mem::transmute(transforms3d.as_ptr()), transforms3d.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateEffectGroup(&self) -> windows_core::Result<IDCompositionEffectGroup> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEffectGroup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRectangleClip(&self) -> windows_core::Result<IDCompositionRectangleClip> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRectangleClip)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateAnimation(&self) -> windows_core::Result<IDCompositionAnimation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAnimation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDCompositionDevice2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForCommitCompletion: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetFrameStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DCOMPOSITION_FRAME_STATISTICS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetFrameStatistics: usize,
    pub CreateVisual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSurfaceFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::Dxgi::Common::DXGI_FORMAT, super::Dxgi::Common::DXGI_ALPHA_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::Dxgi::Common::DXGI_FORMAT, super::Dxgi::Common::DXGI_ALPHA_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
    pub CreateTranslateTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateScaleTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRotateTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSkewTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTransformGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTranslateTransform3D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateScaleTransform3D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRotateTransform3D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateMatrixTransform3D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTransform3DGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEffectGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRectangleClip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAnimation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionDevice3, IDCompositionDevice3_Vtbl, 0x0987cb06_f916_48bf_8d35_ce7641781bd9);
impl core::ops::Deref for IDCompositionDevice3 {
    type Target = IDCompositionDevice2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionDevice3, windows_core::IUnknown, IDCompositionDevice2);
impl IDCompositionDevice3 {
    pub unsafe fn CreateGaussianBlurEffect(&self) -> windows_core::Result<IDCompositionGaussianBlurEffect> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGaussianBlurEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBrightnessEffect(&self) -> windows_core::Result<IDCompositionBrightnessEffect> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBrightnessEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateColorMatrixEffect(&self) -> windows_core::Result<IDCompositionColorMatrixEffect> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateColorMatrixEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateShadowEffect(&self) -> windows_core::Result<IDCompositionShadowEffect> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateShadowEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateHueRotationEffect(&self) -> windows_core::Result<IDCompositionHueRotationEffect> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateHueRotationEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSaturationEffect(&self) -> windows_core::Result<IDCompositionSaturationEffect> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSaturationEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateTurbulenceEffect(&self) -> windows_core::Result<IDCompositionTurbulenceEffect> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTurbulenceEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateLinearTransferEffect(&self) -> windows_core::Result<IDCompositionLinearTransferEffect> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLinearTransferEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateTableTransferEffect(&self) -> windows_core::Result<IDCompositionTableTransferEffect> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTableTransferEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateCompositeEffect(&self) -> windows_core::Result<IDCompositionCompositeEffect> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCompositeEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBlendEffect(&self) -> windows_core::Result<IDCompositionBlendEffect> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBlendEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateArithmeticCompositeEffect(&self) -> windows_core::Result<IDCompositionArithmeticCompositeEffect> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateArithmeticCompositeEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateAffineTransform2DEffect(&self) -> windows_core::Result<IDCompositionAffineTransform2DEffect> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAffineTransform2DEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDCompositionDevice3_Vtbl {
    pub base__: IDCompositionDevice2_Vtbl,
    pub CreateGaussianBlurEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBrightnessEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateColorMatrixEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateShadowEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateHueRotationEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSaturationEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTurbulenceEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateLinearTransferEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTableTransferEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCompositeEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBlendEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateArithmeticCompositeEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAffineTransform2DEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionDevice4, IDCompositionDevice4_Vtbl, 0x85fc5cca_2da6_494c_86b6_4a775c049b8a);
impl core::ops::Deref for IDCompositionDevice4 {
    type Target = IDCompositionDevice3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionDevice4, windows_core::IUnknown, IDCompositionDevice2, IDCompositionDevice3);
impl IDCompositionDevice4 {
    pub unsafe fn CheckCompositionTextureSupport<P0>(&self, renderingdevice: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckCompositionTextureSupport)(windows_core::Interface::as_raw(self), renderingdevice.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn CreateCompositionTexture<P0>(&self, d3dtexture: P0) -> windows_core::Result<IDCompositionTexture>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCompositionTexture)(windows_core::Interface::as_raw(self), d3dtexture.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDCompositionDevice4_Vtbl {
    pub base__: IDCompositionDevice3_Vtbl,
    pub CheckCompositionTextureSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CreateCompositionTexture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionDeviceDebug, IDCompositionDeviceDebug_Vtbl, 0xa1a3c64a_224f_4a81_9773_4f03a89d3c6c);
impl core::ops::Deref for IDCompositionDeviceDebug {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionDeviceDebug, windows_core::IUnknown);
impl IDCompositionDeviceDebug {
    pub unsafe fn EnableDebugCounters(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableDebugCounters)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableDebugCounters(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisableDebugCounters)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDCompositionDeviceDebug_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnableDebugCounters: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableDebugCounters: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionEffect, IDCompositionEffect_Vtbl, 0xec81b08f_bfcb_4e8d_b193_a915587999e8);
impl core::ops::Deref for IDCompositionEffect {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionEffect, windows_core::IUnknown);
impl IDCompositionEffect {}
#[repr(C)]
pub struct IDCompositionEffect_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
windows_core::imp::define_interface!(IDCompositionEffectGroup, IDCompositionEffectGroup_Vtbl, 0xa7929a74_e6b2_4bd6_8b95_4040119ca34d);
impl core::ops::Deref for IDCompositionEffectGroup {
    type Target = IDCompositionEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionEffectGroup, windows_core::IUnknown, IDCompositionEffect);
impl IDCompositionEffectGroup {
    pub unsafe fn SetOpacity<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetOpacity)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetOpacity2(&self, opacity: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOpacity2)(windows_core::Interface::as_raw(self), opacity).ok()
    }
    pub unsafe fn SetTransform3D<P0>(&self, transform3d: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionTransform3D>,
    {
        (windows_core::Interface::vtable(self).SetTransform3D)(windows_core::Interface::as_raw(self), transform3d.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDCompositionEffectGroup_Vtbl {
    pub base__: IDCompositionEffect_Vtbl,
    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOpacity2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetTransform3D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionFilterEffect, IDCompositionFilterEffect_Vtbl, 0x30c421d5_8cb2_4e9f_b133_37be270d4ac2);
impl core::ops::Deref for IDCompositionFilterEffect {
    type Target = IDCompositionEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionFilterEffect, windows_core::IUnknown, IDCompositionEffect);
impl IDCompositionFilterEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetInput)(windows_core::Interface::as_raw(self), index, input.param().abi(), flags).ok()
    }
}
#[repr(C)]
pub struct IDCompositionFilterEffect_Vtbl {
    pub base__: IDCompositionEffect_Vtbl,
    pub SetInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionGaussianBlurEffect, IDCompositionGaussianBlurEffect_Vtbl, 0x45d4d0b7_1bd4_454e_8894_2bfa68443033);
impl core::ops::Deref for IDCompositionGaussianBlurEffect {
    type Target = IDCompositionFilterEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionGaussianBlurEffect, windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl IDCompositionGaussianBlurEffect {
    pub unsafe fn SetStandardDeviation<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetStandardDeviation)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetStandardDeviation2(&self, amount: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStandardDeviation2)(windows_core::Interface::as_raw(self), amount).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBorderMode(&self, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBorderMode)(windows_core::Interface::as_raw(self), mode).ok()
    }
}
#[repr(C)]
pub struct IDCompositionGaussianBlurEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetStandardDeviation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStandardDeviation2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBorderMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::Direct2D::Common::D2D1_BORDER_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBorderMode: usize,
}
windows_core::imp::define_interface!(IDCompositionHueRotationEffect, IDCompositionHueRotationEffect_Vtbl, 0x6db9f920_0770_4781_b0c6_381912f9d167);
impl core::ops::Deref for IDCompositionHueRotationEffect {
    type Target = IDCompositionFilterEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionHueRotationEffect, windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl IDCompositionHueRotationEffect {
    pub unsafe fn SetAngle<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetAngle)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetAngle2(&self, amountdegrees: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAngle2)(windows_core::Interface::as_raw(self), amountdegrees).ok()
    }
}
#[repr(C)]
pub struct IDCompositionHueRotationEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetAngle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionInkTrailDevice, IDCompositionInkTrailDevice_Vtbl, 0xdf0c7cec_cdeb_4d4a_b91c_721bf22f4e6c);
impl core::ops::Deref for IDCompositionInkTrailDevice {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionInkTrailDevice, windows_core::IUnknown);
impl IDCompositionInkTrailDevice {
    pub unsafe fn CreateDelegatedInkTrail(&self) -> windows_core::Result<IDCompositionDelegatedInkTrail> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDelegatedInkTrail)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateDelegatedInkTrailForSwapChain<P0>(&self, swapchain: P0) -> windows_core::Result<IDCompositionDelegatedInkTrail>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDelegatedInkTrailForSwapChain)(windows_core::Interface::as_raw(self), swapchain.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDCompositionInkTrailDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateDelegatedInkTrail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDelegatedInkTrailForSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionLinearTransferEffect, IDCompositionLinearTransferEffect_Vtbl, 0x4305ee5b_c4a0_4c88_9385_67124e017683);
impl core::ops::Deref for IDCompositionLinearTransferEffect {
    type Target = IDCompositionFilterEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionLinearTransferEffect, windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl IDCompositionLinearTransferEffect {
    pub unsafe fn SetRedYIntercept<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetRedYIntercept)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetRedYIntercept2(&self, redyintercept: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRedYIntercept2)(windows_core::Interface::as_raw(self), redyintercept).ok()
    }
    pub unsafe fn SetRedSlope<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetRedSlope)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetRedSlope2(&self, redslope: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRedSlope2)(windows_core::Interface::as_raw(self), redslope).ok()
    }
    pub unsafe fn SetRedDisable<P0>(&self, reddisable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetRedDisable)(windows_core::Interface::as_raw(self), reddisable.param().abi()).ok()
    }
    pub unsafe fn SetGreenYIntercept<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetGreenYIntercept)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetGreenYIntercept2(&self, greenyintercept: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGreenYIntercept2)(windows_core::Interface::as_raw(self), greenyintercept).ok()
    }
    pub unsafe fn SetGreenSlope<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetGreenSlope)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetGreenSlope2(&self, greenslope: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGreenSlope2)(windows_core::Interface::as_raw(self), greenslope).ok()
    }
    pub unsafe fn SetGreenDisable<P0>(&self, greendisable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetGreenDisable)(windows_core::Interface::as_raw(self), greendisable.param().abi()).ok()
    }
    pub unsafe fn SetBlueYIntercept<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetBlueYIntercept)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetBlueYIntercept2(&self, blueyintercept: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBlueYIntercept2)(windows_core::Interface::as_raw(self), blueyintercept).ok()
    }
    pub unsafe fn SetBlueSlope<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetBlueSlope)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetBlueSlope2(&self, blueslope: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBlueSlope2)(windows_core::Interface::as_raw(self), blueslope).ok()
    }
    pub unsafe fn SetBlueDisable<P0>(&self, bluedisable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBlueDisable)(windows_core::Interface::as_raw(self), bluedisable.param().abi()).ok()
    }
    pub unsafe fn SetAlphaYIntercept<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetAlphaYIntercept)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetAlphaYIntercept2(&self, alphayintercept: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAlphaYIntercept2)(windows_core::Interface::as_raw(self), alphayintercept).ok()
    }
    pub unsafe fn SetAlphaSlope<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetAlphaSlope)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetAlphaSlope2(&self, alphaslope: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAlphaSlope2)(windows_core::Interface::as_raw(self), alphaslope).ok()
    }
    pub unsafe fn SetAlphaDisable<P0>(&self, alphadisable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAlphaDisable)(windows_core::Interface::as_raw(self), alphadisable.param().abi()).ok()
    }
    pub unsafe fn SetClampOutput<P0>(&self, clampoutput: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetClampOutput)(windows_core::Interface::as_raw(self), clampoutput.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDCompositionLinearTransferEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetRedYIntercept: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRedYIntercept2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetRedSlope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRedSlope2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetRedDisable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetGreenYIntercept: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGreenYIntercept2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetGreenSlope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGreenSlope2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetGreenDisable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetBlueYIntercept: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBlueYIntercept2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetBlueSlope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBlueSlope2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetBlueDisable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetAlphaYIntercept: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAlphaYIntercept2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetAlphaSlope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAlphaSlope2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetAlphaDisable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetClampOutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionMatrixTransform, IDCompositionMatrixTransform_Vtbl, 0x16cdff07_c503_419c_83f2_0965c7af1fa6);
impl core::ops::Deref for IDCompositionMatrixTransform {
    type Target = IDCompositionTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionMatrixTransform, windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D, IDCompositionTransform);
impl IDCompositionMatrixTransform {
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetMatrix(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMatrix)(windows_core::Interface::as_raw(self), matrix).ok()
    }
    pub unsafe fn SetMatrixElement<P0>(&self, row: i32, column: i32, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetMatrixElement)(windows_core::Interface::as_raw(self), row, column, animation.param().abi()).ok()
    }
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMatrixElement2)(windows_core::Interface::as_raw(self), row, column, value).ok()
    }
}
#[repr(C)]
pub struct IDCompositionMatrixTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionMatrixTransform3D, IDCompositionMatrixTransform3D_Vtbl, 0x4b3363f0_643b_41b7_b6e0_ccf22d34467c);
impl core::ops::Deref for IDCompositionMatrixTransform3D {
    type Target = IDCompositionTransform3D;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionMatrixTransform3D, windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D);
impl IDCompositionMatrixTransform3D {
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetMatrix(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix4x4) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMatrix)(windows_core::Interface::as_raw(self), matrix).ok()
    }
    pub unsafe fn SetMatrixElement<P0>(&self, row: i32, column: i32, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetMatrixElement)(windows_core::Interface::as_raw(self), row, column, animation.param().abi()).ok()
    }
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMatrixElement2)(windows_core::Interface::as_raw(self), row, column, value).ok()
    }
}
#[repr(C)]
pub struct IDCompositionMatrixTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix4x4) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionRectangleClip, IDCompositionRectangleClip_Vtbl, 0x9842ad7d_d9cf_4908_aed7_48b51da5e7c2);
impl core::ops::Deref for IDCompositionRectangleClip {
    type Target = IDCompositionClip;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionRectangleClip, windows_core::IUnknown, IDCompositionClip);
impl IDCompositionRectangleClip {
    pub unsafe fn SetLeft<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetLeft)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetLeft2(&self, left: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLeft2)(windows_core::Interface::as_raw(self), left).ok()
    }
    pub unsafe fn SetTop<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetTop)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetTop2(&self, top: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTop2)(windows_core::Interface::as_raw(self), top).ok()
    }
    pub unsafe fn SetRight<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetRight)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetRight2(&self, right: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRight2)(windows_core::Interface::as_raw(self), right).ok()
    }
    pub unsafe fn SetBottom<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetBottom)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetBottom2(&self, bottom: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBottom2)(windows_core::Interface::as_raw(self), bottom).ok()
    }
    pub unsafe fn SetTopLeftRadiusX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetTopLeftRadiusX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetTopLeftRadiusX2(&self, radius: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTopLeftRadiusX2)(windows_core::Interface::as_raw(self), radius).ok()
    }
    pub unsafe fn SetTopLeftRadiusY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetTopLeftRadiusY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetTopLeftRadiusY2(&self, radius: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTopLeftRadiusY2)(windows_core::Interface::as_raw(self), radius).ok()
    }
    pub unsafe fn SetTopRightRadiusX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetTopRightRadiusX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetTopRightRadiusX2(&self, radius: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTopRightRadiusX2)(windows_core::Interface::as_raw(self), radius).ok()
    }
    pub unsafe fn SetTopRightRadiusY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetTopRightRadiusY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetTopRightRadiusY2(&self, radius: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTopRightRadiusY2)(windows_core::Interface::as_raw(self), radius).ok()
    }
    pub unsafe fn SetBottomLeftRadiusX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetBottomLeftRadiusX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetBottomLeftRadiusX2(&self, radius: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBottomLeftRadiusX2)(windows_core::Interface::as_raw(self), radius).ok()
    }
    pub unsafe fn SetBottomLeftRadiusY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetBottomLeftRadiusY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetBottomLeftRadiusY2(&self, radius: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBottomLeftRadiusY2)(windows_core::Interface::as_raw(self), radius).ok()
    }
    pub unsafe fn SetBottomRightRadiusX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetBottomRightRadiusX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetBottomRightRadiusX2(&self, radius: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBottomRightRadiusX2)(windows_core::Interface::as_raw(self), radius).ok()
    }
    pub unsafe fn SetBottomRightRadiusY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetBottomRightRadiusY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetBottomRightRadiusY2(&self, radius: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBottomRightRadiusY2)(windows_core::Interface::as_raw(self), radius).ok()
    }
}
#[repr(C)]
pub struct IDCompositionRectangleClip_Vtbl {
    pub base__: IDCompositionClip_Vtbl,
    pub SetLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLeft2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTop2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetRight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRight2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetBottom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBottom2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetTopLeftRadiusX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTopLeftRadiusX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetTopLeftRadiusY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTopLeftRadiusY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetTopRightRadiusX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTopRightRadiusX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetTopRightRadiusY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTopRightRadiusY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetBottomLeftRadiusX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBottomLeftRadiusX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetBottomLeftRadiusY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBottomLeftRadiusY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetBottomRightRadiusX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBottomRightRadiusX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetBottomRightRadiusY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBottomRightRadiusY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionRotateTransform, IDCompositionRotateTransform_Vtbl, 0x641ed83c_ae96_46c5_90dc_32774cc5c6d5);
impl core::ops::Deref for IDCompositionRotateTransform {
    type Target = IDCompositionTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionRotateTransform, windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D, IDCompositionTransform);
impl IDCompositionRotateTransform {
    pub unsafe fn SetAngle<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetAngle)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetAngle2(&self, angle: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAngle2)(windows_core::Interface::as_raw(self), angle).ok()
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetCenterX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCenterX2)(windows_core::Interface::as_raw(self), centerx).ok()
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetCenterY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCenterY2)(windows_core::Interface::as_raw(self), centery).ok()
    }
}
#[repr(C)]
pub struct IDCompositionRotateTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetAngle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionRotateTransform3D, IDCompositionRotateTransform3D_Vtbl, 0xd8f5b23f_d429_4a91_b55a_d2f45fd75b18);
impl core::ops::Deref for IDCompositionRotateTransform3D {
    type Target = IDCompositionTransform3D;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionRotateTransform3D, windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D);
impl IDCompositionRotateTransform3D {
    pub unsafe fn SetAngle<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetAngle)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetAngle2(&self, angle: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAngle2)(windows_core::Interface::as_raw(self), angle).ok()
    }
    pub unsafe fn SetAxisX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetAxisX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetAxisX2(&self, axisx: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAxisX2)(windows_core::Interface::as_raw(self), axisx).ok()
    }
    pub unsafe fn SetAxisY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetAxisY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetAxisY2(&self, axisy: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAxisY2)(windows_core::Interface::as_raw(self), axisy).ok()
    }
    pub unsafe fn SetAxisZ<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetAxisZ)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetAxisZ2(&self, axisz: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAxisZ2)(windows_core::Interface::as_raw(self), axisz).ok()
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetCenterX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCenterX2)(windows_core::Interface::as_raw(self), centerx).ok()
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetCenterY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCenterY2)(windows_core::Interface::as_raw(self), centery).ok()
    }
    pub unsafe fn SetCenterZ<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetCenterZ)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetCenterZ2(&self, centerz: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCenterZ2)(windows_core::Interface::as_raw(self), centerz).ok()
    }
}
#[repr(C)]
pub struct IDCompositionRotateTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    pub SetAngle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetAxisX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAxisX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetAxisY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAxisY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetAxisZ: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAxisZ2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCenterZ: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCenterZ2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionSaturationEffect, IDCompositionSaturationEffect_Vtbl, 0xa08debda_3258_4fa4_9f16_9174d3fe93b1);
impl core::ops::Deref for IDCompositionSaturationEffect {
    type Target = IDCompositionFilterEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionSaturationEffect, windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl IDCompositionSaturationEffect {
    pub unsafe fn SetSaturation<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetSaturation)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetSaturation2(&self, ratio: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSaturation2)(windows_core::Interface::as_raw(self), ratio).ok()
    }
}
#[repr(C)]
pub struct IDCompositionSaturationEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetSaturation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSaturation2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionScaleTransform, IDCompositionScaleTransform_Vtbl, 0x71fde914_40ef_45ef_bd51_68b037c339f9);
impl core::ops::Deref for IDCompositionScaleTransform {
    type Target = IDCompositionTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionScaleTransform, windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D, IDCompositionTransform);
impl IDCompositionScaleTransform {
    pub unsafe fn SetScaleX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetScaleX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetScaleX2(&self, scalex: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScaleX2)(windows_core::Interface::as_raw(self), scalex).ok()
    }
    pub unsafe fn SetScaleY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetScaleY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetScaleY2(&self, scaley: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScaleY2)(windows_core::Interface::as_raw(self), scaley).ok()
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetCenterX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCenterX2)(windows_core::Interface::as_raw(self), centerx).ok()
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetCenterY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCenterY2)(windows_core::Interface::as_raw(self), centery).ok()
    }
}
#[repr(C)]
pub struct IDCompositionScaleTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetScaleX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetScaleX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetScaleY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionScaleTransform3D, IDCompositionScaleTransform3D_Vtbl, 0x2a9e9ead_364b_4b15_a7c4_a1997f78b389);
impl core::ops::Deref for IDCompositionScaleTransform3D {
    type Target = IDCompositionTransform3D;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionScaleTransform3D, windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D);
impl IDCompositionScaleTransform3D {
    pub unsafe fn SetScaleX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetScaleX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetScaleX2(&self, scalex: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScaleX2)(windows_core::Interface::as_raw(self), scalex).ok()
    }
    pub unsafe fn SetScaleY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetScaleY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetScaleY2(&self, scaley: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScaleY2)(windows_core::Interface::as_raw(self), scaley).ok()
    }
    pub unsafe fn SetScaleZ<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetScaleZ)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetScaleZ2(&self, scalez: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScaleZ2)(windows_core::Interface::as_raw(self), scalez).ok()
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetCenterX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCenterX2)(windows_core::Interface::as_raw(self), centerx).ok()
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetCenterY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCenterY2)(windows_core::Interface::as_raw(self), centery).ok()
    }
    pub unsafe fn SetCenterZ<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetCenterZ)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetCenterZ2(&self, centerz: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCenterZ2)(windows_core::Interface::as_raw(self), centerz).ok()
    }
}
#[repr(C)]
pub struct IDCompositionScaleTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    pub SetScaleX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetScaleX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetScaleY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetScaleZ: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetScaleZ2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCenterZ: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCenterZ2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionShadowEffect, IDCompositionShadowEffect_Vtbl, 0x4ad18ac0_cfd2_4c2f_bb62_96e54fdb6879);
impl core::ops::Deref for IDCompositionShadowEffect {
    type Target = IDCompositionFilterEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionShadowEffect, windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl IDCompositionShadowEffect {
    pub unsafe fn SetStandardDeviation<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetStandardDeviation)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetStandardDeviation2(&self, amount: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStandardDeviation2)(windows_core::Interface::as_raw(self), amount).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetColor(&self, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColor)(windows_core::Interface::as_raw(self), color).ok()
    }
    pub unsafe fn SetRed<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetRed)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetRed2(&self, amount: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRed2)(windows_core::Interface::as_raw(self), amount).ok()
    }
    pub unsafe fn SetGreen<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetGreen)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetGreen2(&self, amount: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGreen2)(windows_core::Interface::as_raw(self), amount).ok()
    }
    pub unsafe fn SetBlue<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetBlue)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetBlue2(&self, amount: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBlue2)(windows_core::Interface::as_raw(self), amount).ok()
    }
    pub unsafe fn SetAlpha<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetAlpha)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetAlpha2(&self, amount: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAlpha2)(windows_core::Interface::as_raw(self), amount).ok()
    }
}
#[repr(C)]
pub struct IDCompositionShadowEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetStandardDeviation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStandardDeviation2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Direct2D::Common::D2D_VECTOR_4F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetColor: usize,
    pub SetRed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRed2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGreen2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBlue2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetAlpha: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAlpha2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionSkewTransform, IDCompositionSkewTransform_Vtbl, 0xe57aa735_dcdb_4c72_9c61_0591f58889ee);
impl core::ops::Deref for IDCompositionSkewTransform {
    type Target = IDCompositionTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionSkewTransform, windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D, IDCompositionTransform);
impl IDCompositionSkewTransform {
    pub unsafe fn SetAngleX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetAngleX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetAngleX2(&self, anglex: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAngleX2)(windows_core::Interface::as_raw(self), anglex).ok()
    }
    pub unsafe fn SetAngleY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetAngleY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetAngleY2(&self, angley: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAngleY2)(windows_core::Interface::as_raw(self), angley).ok()
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetCenterX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCenterX2)(windows_core::Interface::as_raw(self), centerx).ok()
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetCenterY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCenterY2)(windows_core::Interface::as_raw(self), centery).ok()
    }
}
#[repr(C)]
pub struct IDCompositionSkewTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetAngleX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAngleX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetAngleY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAngleY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionSurface, IDCompositionSurface_Vtbl, 0xbb8a4953_2c99_4f5a_96f5_4819027fa3ac);
impl core::ops::Deref for IDCompositionSurface {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionSurface, windows_core::IUnknown);
impl IDCompositionSurface {
    pub unsafe fn BeginDraw<T>(&self, updaterect: Option<*const super::super::Foundation::RECT>, updateoffset: *mut super::super::Foundation::POINT) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).BeginDraw)(windows_core::Interface::as_raw(self), core::mem::transmute(updaterect.unwrap_or(std::ptr::null())), &T::IID, &mut result__, updateoffset).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndDraw(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndDraw)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SuspendDraw)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResumeDraw)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Scroll(&self, scrollrect: Option<*const super::super::Foundation::RECT>, cliprect: Option<*const super::super::Foundation::RECT>, offsetx: i32, offsety: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Scroll)(windows_core::Interface::as_raw(self), core::mem::transmute(scrollrect.unwrap_or(std::ptr::null())), core::mem::transmute(cliprect.unwrap_or(std::ptr::null())), offsetx, offsety).ok()
    }
}
#[repr(C)]
pub struct IDCompositionSurface_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginDraw: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut super::super::Foundation::POINT) -> windows_core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResumeDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Scroll: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, *const super::super::Foundation::RECT, i32, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionSurfaceFactory, IDCompositionSurfaceFactory_Vtbl, 0xe334bc12_3937_4e02_85eb_fcf4eb30d2c8);
impl core::ops::Deref for IDCompositionSurfaceFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionSurfaceFactory, windows_core::IUnknown);
impl IDCompositionSurfaceFactory {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionSurface> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSurface)(windows_core::Interface::as_raw(self), width, height, pixelformat, alphamode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionVirtualSurface> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateVirtualSurface)(windows_core::Interface::as_raw(self), initialwidth, initialheight, pixelformat, alphamode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDCompositionSurfaceFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::Dxgi::Common::DXGI_FORMAT, super::Dxgi::Common::DXGI_ALPHA_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::Dxgi::Common::DXGI_FORMAT, super::Dxgi::Common::DXGI_ALPHA_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
}
windows_core::imp::define_interface!(IDCompositionTableTransferEffect, IDCompositionTableTransferEffect_Vtbl, 0x9b7e82e2_69c5_4eb4_a5f5_a7033f5132cd);
impl core::ops::Deref for IDCompositionTableTransferEffect {
    type Target = IDCompositionFilterEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionTableTransferEffect, windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl IDCompositionTableTransferEffect {
    pub unsafe fn SetRedTable(&self, tablevalues: &[f32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRedTable)(windows_core::Interface::as_raw(self), core::mem::transmute(tablevalues.as_ptr()), tablevalues.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetGreenTable(&self, tablevalues: &[f32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGreenTable)(windows_core::Interface::as_raw(self), core::mem::transmute(tablevalues.as_ptr()), tablevalues.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetBlueTable(&self, tablevalues: &[f32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBlueTable)(windows_core::Interface::as_raw(self), core::mem::transmute(tablevalues.as_ptr()), tablevalues.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetAlphaTable(&self, tablevalues: &[f32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAlphaTable)(windows_core::Interface::as_raw(self), core::mem::transmute(tablevalues.as_ptr()), tablevalues.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetRedDisable<P0>(&self, reddisable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetRedDisable)(windows_core::Interface::as_raw(self), reddisable.param().abi()).ok()
    }
    pub unsafe fn SetGreenDisable<P0>(&self, greendisable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetGreenDisable)(windows_core::Interface::as_raw(self), greendisable.param().abi()).ok()
    }
    pub unsafe fn SetBlueDisable<P0>(&self, bluedisable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBlueDisable)(windows_core::Interface::as_raw(self), bluedisable.param().abi()).ok()
    }
    pub unsafe fn SetAlphaDisable<P0>(&self, alphadisable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAlphaDisable)(windows_core::Interface::as_raw(self), alphadisable.param().abi()).ok()
    }
    pub unsafe fn SetClampOutput<P0>(&self, clampoutput: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetClampOutput)(windows_core::Interface::as_raw(self), clampoutput.param().abi()).ok()
    }
    pub unsafe fn SetRedTableValue<P0>(&self, index: u32, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetRedTableValue)(windows_core::Interface::as_raw(self), index, animation.param().abi()).ok()
    }
    pub unsafe fn SetRedTableValue2(&self, index: u32, value: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRedTableValue2)(windows_core::Interface::as_raw(self), index, value).ok()
    }
    pub unsafe fn SetGreenTableValue<P0>(&self, index: u32, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetGreenTableValue)(windows_core::Interface::as_raw(self), index, animation.param().abi()).ok()
    }
    pub unsafe fn SetGreenTableValue2(&self, index: u32, value: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGreenTableValue2)(windows_core::Interface::as_raw(self), index, value).ok()
    }
    pub unsafe fn SetBlueTableValue<P0>(&self, index: u32, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetBlueTableValue)(windows_core::Interface::as_raw(self), index, animation.param().abi()).ok()
    }
    pub unsafe fn SetBlueTableValue2(&self, index: u32, value: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBlueTableValue2)(windows_core::Interface::as_raw(self), index, value).ok()
    }
    pub unsafe fn SetAlphaTableValue<P0>(&self, index: u32, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetAlphaTableValue)(windows_core::Interface::as_raw(self), index, animation.param().abi()).ok()
    }
    pub unsafe fn SetAlphaTableValue2(&self, index: u32, value: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAlphaTableValue2)(windows_core::Interface::as_raw(self), index, value).ok()
    }
}
#[repr(C)]
pub struct IDCompositionTableTransferEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetRedTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32) -> windows_core::HRESULT,
    pub SetGreenTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32) -> windows_core::HRESULT,
    pub SetBlueTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32) -> windows_core::HRESULT,
    pub SetAlphaTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32) -> windows_core::HRESULT,
    pub SetRedDisable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetGreenDisable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetBlueDisable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetAlphaDisable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetClampOutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetRedTableValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRedTableValue2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32) -> windows_core::HRESULT,
    pub SetGreenTableValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGreenTableValue2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32) -> windows_core::HRESULT,
    pub SetBlueTableValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBlueTableValue2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32) -> windows_core::HRESULT,
    pub SetAlphaTableValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAlphaTableValue2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionTarget, IDCompositionTarget_Vtbl, 0xeacdd04c_117e_4e17_88f4_d1b12b0e3d89);
impl core::ops::Deref for IDCompositionTarget {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionTarget, windows_core::IUnknown);
impl IDCompositionTarget {
    pub unsafe fn SetRoot<P0>(&self, visual: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionVisual>,
    {
        (windows_core::Interface::vtable(self).SetRoot)(windows_core::Interface::as_raw(self), visual.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDCompositionTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionTexture, IDCompositionTexture_Vtbl, 0x929bb1aa_725f_433b_abd7_273075a835f2);
impl core::ops::Deref for IDCompositionTexture {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionTexture, windows_core::IUnknown);
impl IDCompositionTexture {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetSourceRect(&self, sourcerect: *const super::Direct2D::Common::D2D_RECT_U) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSourceRect)(windows_core::Interface::as_raw(self), sourcerect).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColorSpace)(windows_core::Interface::as_raw(self), colorspace).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetAlphaMode(&self, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAlphaMode)(windows_core::Interface::as_raw(self), alphamode).ok()
    }
    pub unsafe fn GetAvailableFence(&self, fencevalue: *mut u64, iid: *const windows_core::GUID, availablefence: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAvailableFence)(windows_core::Interface::as_raw(self), fencevalue, iid, availablefence).ok()
    }
}
#[repr(C)]
pub struct IDCompositionTexture_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Direct2D::Common::D2D_RECT_U) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetSourceRect: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetColorSpace: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetAlphaMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetAlphaMode: usize,
    pub GetAvailableFence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionTransform, IDCompositionTransform_Vtbl, 0xfd55faa7_37e0_4c20_95d2_9be45bc33f55);
impl core::ops::Deref for IDCompositionTransform {
    type Target = IDCompositionTransform3D;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionTransform, windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D);
impl IDCompositionTransform {}
#[repr(C)]
pub struct IDCompositionTransform_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
}
windows_core::imp::define_interface!(IDCompositionTransform3D, IDCompositionTransform3D_Vtbl, 0x71185722_246b_41f2_aad1_0443f7f4bfc2);
impl core::ops::Deref for IDCompositionTransform3D {
    type Target = IDCompositionEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionTransform3D, windows_core::IUnknown, IDCompositionEffect);
impl IDCompositionTransform3D {}
#[repr(C)]
pub struct IDCompositionTransform3D_Vtbl {
    pub base__: IDCompositionEffect_Vtbl,
}
windows_core::imp::define_interface!(IDCompositionTranslateTransform, IDCompositionTranslateTransform_Vtbl, 0x06791122_c6f0_417d_8323_269e987f5954);
impl core::ops::Deref for IDCompositionTranslateTransform {
    type Target = IDCompositionTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionTranslateTransform, windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D, IDCompositionTransform);
impl IDCompositionTranslateTransform {
    pub unsafe fn SetOffsetX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetOffsetX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOffsetX2)(windows_core::Interface::as_raw(self), offsetx).ok()
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetOffsetY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOffsetY2)(windows_core::Interface::as_raw(self), offsety).ok()
    }
}
#[repr(C)]
pub struct IDCompositionTranslateTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetOffsetX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionTranslateTransform3D, IDCompositionTranslateTransform3D_Vtbl, 0x91636d4b_9ba1_4532_aaf7_e3344994d788);
impl core::ops::Deref for IDCompositionTranslateTransform3D {
    type Target = IDCompositionTransform3D;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionTranslateTransform3D, windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D);
impl IDCompositionTranslateTransform3D {
    pub unsafe fn SetOffsetX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetOffsetX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOffsetX2)(windows_core::Interface::as_raw(self), offsetx).ok()
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetOffsetY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOffsetY2)(windows_core::Interface::as_raw(self), offsety).ok()
    }
    pub unsafe fn SetOffsetZ<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetOffsetZ)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetOffsetZ2(&self, offsetz: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOffsetZ2)(windows_core::Interface::as_raw(self), offsetz).ok()
    }
}
#[repr(C)]
pub struct IDCompositionTranslateTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    pub SetOffsetX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetOffsetZ: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOffsetZ2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionTurbulenceEffect, IDCompositionTurbulenceEffect_Vtbl, 0xa6a55bda_c09c_49f3_9193_a41922c89715);
impl core::ops::Deref for IDCompositionTurbulenceEffect {
    type Target = IDCompositionFilterEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionTurbulenceEffect, windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl IDCompositionTurbulenceEffect {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetOffset(&self, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOffset)(windows_core::Interface::as_raw(self), offset).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBaseFrequency(&self, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBaseFrequency)(windows_core::Interface::as_raw(self), frequency).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetSize(&self, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), size).ok()
    }
    pub unsafe fn SetNumOctaves(&self, numoctaves: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNumOctaves)(windows_core::Interface::as_raw(self), numoctaves).ok()
    }
    pub unsafe fn SetSeed(&self, seed: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSeed)(windows_core::Interface::as_raw(self), seed).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetNoise(&self, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNoise)(windows_core::Interface::as_raw(self), noise).ok()
    }
    pub unsafe fn SetStitchable<P0>(&self, stitchable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetStitchable)(windows_core::Interface::as_raw(self), stitchable.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDCompositionTurbulenceEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetOffset: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBaseFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBaseFrequency: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetSize: usize,
    pub SetNumOctaves: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetSeed: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetNoise: unsafe extern "system" fn(*mut core::ffi::c_void, super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetNoise: usize,
    pub SetStitchable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionVirtualSurface, IDCompositionVirtualSurface_Vtbl, 0xae471c51_5f53_4a24_8d3e_d0c39c30b3f0);
impl core::ops::Deref for IDCompositionVirtualSurface {
    type Target = IDCompositionSurface;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionVirtualSurface, windows_core::IUnknown, IDCompositionSurface);
impl IDCompositionVirtualSurface {
    pub unsafe fn Resize(&self, width: u32, height: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resize)(windows_core::Interface::as_raw(self), width, height).ok()
    }
    pub unsafe fn Trim(&self, rectangles: Option<&[super::super::Foundation::RECT]>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Trim)(windows_core::Interface::as_raw(self), core::mem::transmute(rectangles.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), rectangles.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
    }
}
#[repr(C)]
pub struct IDCompositionVirtualSurface_Vtbl {
    pub base__: IDCompositionSurface_Vtbl,
    pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Trim: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionVisual, IDCompositionVisual_Vtbl, 0x4d93059d_097b_4651_9a60_f0f25116e2f3);
impl core::ops::Deref for IDCompositionVisual {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionVisual, windows_core::IUnknown);
impl IDCompositionVisual {
    pub unsafe fn SetOffsetX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetOffsetX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOffsetX2)(windows_core::Interface::as_raw(self), offsetx).ok()
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetOffsetY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOffsetY2)(windows_core::Interface::as_raw(self), offsety).ok()
    }
    pub unsafe fn SetTransform<P0>(&self, transform: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionTransform>,
    {
        (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), transform.param().abi()).ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTransform2)(windows_core::Interface::as_raw(self), matrix).ok()
    }
    pub unsafe fn SetTransformParent<P0>(&self, visual: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionVisual>,
    {
        (windows_core::Interface::vtable(self).SetTransformParent)(windows_core::Interface::as_raw(self), visual.param().abi()).ok()
    }
    pub unsafe fn SetEffect<P0>(&self, effect: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionEffect>,
    {
        (windows_core::Interface::vtable(self).SetEffect)(windows_core::Interface::as_raw(self), effect.param().abi()).ok()
    }
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBitmapInterpolationMode)(windows_core::Interface::as_raw(self), interpolationmode).ok()
    }
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBorderMode)(windows_core::Interface::as_raw(self), bordermode).ok()
    }
    pub unsafe fn SetClip<P0>(&self, clip: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionClip>,
    {
        (windows_core::Interface::vtable(self).SetClip)(windows_core::Interface::as_raw(self), clip.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetClip2)(windows_core::Interface::as_raw(self), rect).ok()
    }
    pub unsafe fn SetContent<P0>(&self, content: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetContent)(windows_core::Interface::as_raw(self), content.param().abi()).ok()
    }
    pub unsafe fn AddVisual<P0, P1, P2>(&self, visual: P0, insertabove: P1, referencevisual: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionVisual>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<IDCompositionVisual>,
    {
        (windows_core::Interface::vtable(self).AddVisual)(windows_core::Interface::as_raw(self), visual.param().abi(), insertabove.param().abi(), referencevisual.param().abi()).ok()
    }
    pub unsafe fn RemoveVisual<P0>(&self, visual: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionVisual>,
    {
        (windows_core::Interface::vtable(self).RemoveVisual)(windows_core::Interface::as_raw(self), visual.param().abi()).ok()
    }
    pub unsafe fn RemoveAllVisuals(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAllVisuals)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCompositeMode)(windows_core::Interface::as_raw(self), compositemode).ok()
    }
}
#[repr(C)]
pub struct IDCompositionVisual_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetOffsetX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform2: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform2: usize,
    pub SetTransformParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBitmapInterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void, DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> windows_core::HRESULT,
    pub SetBorderMode: unsafe extern "system" fn(*mut core::ffi::c_void, DCOMPOSITION_BORDER_MODE) -> windows_core::HRESULT,
    pub SetClip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetClip2: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Direct2D::Common::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetClip2: usize,
    pub SetContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddVisual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveVisual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAllVisuals: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCompositeMode: unsafe extern "system" fn(*mut core::ffi::c_void, DCOMPOSITION_COMPOSITE_MODE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionVisual2, IDCompositionVisual2_Vtbl, 0xe8de1639_4331_4b26_bc5f_6a321d347a85);
impl core::ops::Deref for IDCompositionVisual2 {
    type Target = IDCompositionVisual;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionVisual2, windows_core::IUnknown, IDCompositionVisual);
impl IDCompositionVisual2 {
    pub unsafe fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOpacityMode)(windows_core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBackFaceVisibility)(windows_core::Interface::as_raw(self), visibility).ok()
    }
}
#[repr(C)]
pub struct IDCompositionVisual2_Vtbl {
    pub base__: IDCompositionVisual_Vtbl,
    pub SetOpacityMode: unsafe extern "system" fn(*mut core::ffi::c_void, DCOMPOSITION_OPACITY_MODE) -> windows_core::HRESULT,
    pub SetBackFaceVisibility: unsafe extern "system" fn(*mut core::ffi::c_void, DCOMPOSITION_BACKFACE_VISIBILITY) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionVisual3, IDCompositionVisual3_Vtbl, 0x2775f462_b6c1_4015_b0be_b3e7d6a4976d);
impl core::ops::Deref for IDCompositionVisual3 {
    type Target = IDCompositionVisualDebug;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionVisual3, windows_core::IUnknown, IDCompositionVisual, IDCompositionVisual2, IDCompositionVisualDebug);
impl IDCompositionVisual3 {
    pub unsafe fn SetDepthMode(&self, mode: DCOMPOSITION_DEPTH_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDepthMode)(windows_core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn SetOffsetZ<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetOffsetZ)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetOffsetZ2(&self, offsetz: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOffsetZ2)(windows_core::Interface::as_raw(self), offsetz).ok()
    }
    pub unsafe fn SetOpacity<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).SetOpacity)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    pub unsafe fn SetOpacity2(&self, opacity: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOpacity2)(windows_core::Interface::as_raw(self), opacity).ok()
    }
    pub unsafe fn SetTransform<P0>(&self, transform: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionTransform3D>,
    {
        (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), transform.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetTransform2(&self, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTransform2)(windows_core::Interface::as_raw(self), matrix).ok()
    }
    pub unsafe fn SetVisible<P0>(&self, visible: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetVisible)(windows_core::Interface::as_raw(self), visible.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDCompositionVisual3_Vtbl {
    pub base__: IDCompositionVisualDebug_Vtbl,
    pub SetDepthMode: unsafe extern "system" fn(*mut core::ffi::c_void, DCOMPOSITION_DEPTH_MODE) -> windows_core::HRESULT,
    pub SetOffsetZ: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOffsetZ2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOpacity2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetTransform2: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetTransform2: usize,
    pub SetVisible: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCompositionVisualDebug, IDCompositionVisualDebug_Vtbl, 0xfed2b808_5eb4_43a0_aea3_35f65280f91b);
impl core::ops::Deref for IDCompositionVisualDebug {
    type Target = IDCompositionVisual2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionVisualDebug, windows_core::IUnknown, IDCompositionVisual, IDCompositionVisual2);
impl IDCompositionVisualDebug {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn EnableHeatMap(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableHeatMap)(windows_core::Interface::as_raw(self), color).ok()
    }
    pub unsafe fn DisableHeatMap(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisableHeatMap)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableRedrawRegions(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableRedrawRegions)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableRedrawRegions(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisableRedrawRegions)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDCompositionVisualDebug_Vtbl {
    pub base__: IDCompositionVisual2_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub EnableHeatMap: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Direct2D::Common::D2D1_COLOR_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    EnableHeatMap: usize,
    pub DisableHeatMap: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableRedrawRegions: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableRedrawRegions: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const COMPOSITIONOBJECT_READ: i32 = 1i32;
pub const COMPOSITIONOBJECT_WRITE: i32 = 2i32;
pub const COMPOSITION_FRAME_ID_COMPLETED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(2i32);
pub const COMPOSITION_FRAME_ID_CONFIRMED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(1i32);
pub const COMPOSITION_FRAME_ID_CREATED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(0i32);
pub const COMPOSITION_STATS_MAX_TARGETS: u32 = 256u32;
pub const DCOMPOSITION_BACKFACE_VISIBILITY_HIDDEN: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(1i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_INHERIT: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(-1i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_VISIBLE: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(0i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_INHERIT: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(-1i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_LINEAR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(1i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(0i32);
pub const DCOMPOSITION_BORDER_MODE_HARD: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(1i32);
pub const DCOMPOSITION_BORDER_MODE_INHERIT: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(-1i32);
pub const DCOMPOSITION_BORDER_MODE_SOFT: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(0i32);
pub const DCOMPOSITION_COMPOSITE_MODE_DESTINATION_INVERT: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(1i32);
pub const DCOMPOSITION_COMPOSITE_MODE_INHERIT: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(-1i32);
pub const DCOMPOSITION_COMPOSITE_MODE_MIN_BLEND: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(2i32);
pub const DCOMPOSITION_COMPOSITE_MODE_SOURCE_OVER: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(0i32);
pub const DCOMPOSITION_DEPTH_MODE_INHERIT: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(-1i32);
pub const DCOMPOSITION_DEPTH_MODE_SORTED: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(3i32);
pub const DCOMPOSITION_DEPTH_MODE_SPATIAL: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(1i32);
pub const DCOMPOSITION_DEPTH_MODE_TREE: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(0i32);
pub const DCOMPOSITION_MAX_WAITFORCOMPOSITORCLOCK_OBJECTS: u32 = 32u32;
pub const DCOMPOSITION_OPACITY_MODE_INHERIT: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(-1i32);
pub const DCOMPOSITION_OPACITY_MODE_LAYER: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(0i32);
pub const DCOMPOSITION_OPACITY_MODE_MULTIPLY: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMPOSITION_FRAME_ID_TYPE(pub i32);
impl windows_core::TypeKind for COMPOSITION_FRAME_ID_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMPOSITION_FRAME_ID_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMPOSITION_FRAME_ID_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DCOMPOSITION_BACKFACE_VISIBILITY(pub i32);
impl windows_core::TypeKind for DCOMPOSITION_BACKFACE_VISIBILITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DCOMPOSITION_BACKFACE_VISIBILITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_BACKFACE_VISIBILITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DCOMPOSITION_BITMAP_INTERPOLATION_MODE(pub i32);
impl windows_core::TypeKind for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_BITMAP_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DCOMPOSITION_BORDER_MODE(pub i32);
impl windows_core::TypeKind for DCOMPOSITION_BORDER_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DCOMPOSITION_BORDER_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_BORDER_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DCOMPOSITION_COMPOSITE_MODE(pub i32);
impl windows_core::TypeKind for DCOMPOSITION_COMPOSITE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DCOMPOSITION_COMPOSITE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_COMPOSITE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DCOMPOSITION_DEPTH_MODE(pub i32);
impl windows_core::TypeKind for DCOMPOSITION_DEPTH_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DCOMPOSITION_DEPTH_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_DEPTH_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DCOMPOSITION_OPACITY_MODE(pub i32);
impl windows_core::TypeKind for DCOMPOSITION_OPACITY_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DCOMPOSITION_OPACITY_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_OPACITY_MODE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COMPOSITION_FRAME_STATS {
    pub startTime: u64,
    pub targetTime: u64,
    pub framePeriod: u64,
}
impl windows_core::TypeKind for COMPOSITION_FRAME_STATS {
    type TypeKind = windows_core::CopyType;
}
impl Default for COMPOSITION_FRAME_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COMPOSITION_STATS {
    pub presentCount: u32,
    pub refreshCount: u32,
    pub virtualRefreshCount: u32,
    pub time: u64,
}
impl windows_core::TypeKind for COMPOSITION_STATS {
    type TypeKind = windows_core::CopyType;
}
impl Default for COMPOSITION_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COMPOSITION_TARGET_ID {
    pub displayAdapterLuid: super::super::Foundation::LUID,
    pub renderAdapterLuid: super::super::Foundation::LUID,
    pub vidPnSourceId: u32,
    pub vidPnTargetId: u32,
    pub uniqueId: u32,
}
impl windows_core::TypeKind for COMPOSITION_TARGET_ID {
    type TypeKind = windows_core::CopyType;
}
impl Default for COMPOSITION_TARGET_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COMPOSITION_TARGET_STATS {
    pub outstandingPresents: u32,
    pub presentTime: u64,
    pub vblankDuration: u64,
    pub presentedStats: COMPOSITION_STATS,
    pub completedStats: COMPOSITION_STATS,
}
impl windows_core::TypeKind for COMPOSITION_TARGET_STATS {
    type TypeKind = windows_core::CopyType;
}
impl Default for COMPOSITION_TARGET_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DCOMPOSITION_FRAME_STATISTICS {
    pub lastFrameTime: i64,
    pub currentCompositionRate: super::Dxgi::Common::DXGI_RATIONAL,
    pub currentTime: i64,
    pub timeFrequency: i64,
    pub nextEstimatedFrameTime: i64,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for DCOMPOSITION_FRAME_STATISTICS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for DCOMPOSITION_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DCompositionInkTrailPoint {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
impl windows_core::TypeKind for DCompositionInkTrailPoint {
    type TypeKind = windows_core::CopyType;
}
impl Default for DCompositionInkTrailPoint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
