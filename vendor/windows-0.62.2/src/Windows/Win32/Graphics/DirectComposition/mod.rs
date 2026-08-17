#[inline]
pub unsafe fn DCompositionAttachMouseDragToHwnd<P0>(visual: P0, hwnd: super::super::Foundation::HWND, enable: bool) -> windows_core::Result<()>
where
    P0: windows_core::Param<IDCompositionVisual>,
{
    windows_core::link!("dcomp.dll" "system" fn DCompositionAttachMouseDragToHwnd(visual : * mut core::ffi::c_void, hwnd : super::super::Foundation:: HWND, enable : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { DCompositionAttachMouseDragToHwnd(visual.param().abi(), hwnd, enable.into()).ok() }
}
#[inline]
pub unsafe fn DCompositionAttachMouseWheelToHwnd<P0>(visual: P0, hwnd: super::super::Foundation::HWND, enable: bool) -> windows_core::Result<()>
where
    P0: windows_core::Param<IDCompositionVisual>,
{
    windows_core::link!("dcomp.dll" "system" fn DCompositionAttachMouseWheelToHwnd(visual : * mut core::ffi::c_void, hwnd : super::super::Foundation:: HWND, enable : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { DCompositionAttachMouseWheelToHwnd(visual.param().abi(), hwnd, enable.into()).ok() }
}
#[inline]
pub unsafe fn DCompositionBoostCompositorClock(enable: bool) -> windows_core::Result<()> {
    windows_core::link!("dcomp.dll" "system" fn DCompositionBoostCompositorClock(enable : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { DCompositionBoostCompositorClock(enable.into()).ok() }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn DCompositionCreateDevice<P0, T>(dxgidevice: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::Dxgi::IDXGIDevice>,
    T: windows_core::Interface,
{
    windows_core::link!("dcomp.dll" "system" fn DCompositionCreateDevice(dxgidevice : * mut core::ffi::c_void, iid : *const windows_core::GUID, dcompositiondevice : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { DCompositionCreateDevice(dxgidevice.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn DCompositionCreateDevice2<P0, T>(renderingdevice: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("dcomp.dll" "system" fn DCompositionCreateDevice2(renderingdevice : * mut core::ffi::c_void, iid : *const windows_core::GUID, dcompositiondevice : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { DCompositionCreateDevice2(renderingdevice.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn DCompositionCreateDevice3<P0, T>(renderingdevice: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("dcomp.dll" "system" fn DCompositionCreateDevice3(renderingdevice : * mut core::ffi::c_void, iid : *const windows_core::GUID, dcompositiondevice : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { DCompositionCreateDevice3(renderingdevice.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn DCompositionCreateSurfaceHandle(desiredaccess: u32, securityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("dcomp.dll" "system" fn DCompositionCreateSurfaceHandle(desiredaccess : u32, securityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, surfacehandle : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DCompositionCreateSurfaceHandle(desiredaccess, securityattributes.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DCompositionGetFrameId(frameidtype: COMPOSITION_FRAME_ID_TYPE) -> windows_core::Result<u64> {
    windows_core::link!("dcomp.dll" "system" fn DCompositionGetFrameId(frameidtype : COMPOSITION_FRAME_ID_TYPE, frameid : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DCompositionGetFrameId(frameidtype, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DCompositionGetStatistics(frameid: u64, framestats: *mut COMPOSITION_FRAME_STATS, targetidcount: u32, targetids: Option<*mut COMPOSITION_TARGET_ID>, actualtargetidcount: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("dcomp.dll" "system" fn DCompositionGetStatistics(frameid : u64, framestats : *mut COMPOSITION_FRAME_STATS, targetidcount : u32, targetids : *mut COMPOSITION_TARGET_ID, actualtargetidcount : *mut u32) -> windows_core::HRESULT);
    unsafe { DCompositionGetStatistics(frameid, framestats as _, targetidcount, targetids.unwrap_or(core::mem::zeroed()) as _, actualtargetidcount.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn DCompositionGetTargetStatistics(frameid: u64, targetid: *const COMPOSITION_TARGET_ID) -> windows_core::Result<COMPOSITION_TARGET_STATS> {
    windows_core::link!("dcomp.dll" "system" fn DCompositionGetTargetStatistics(frameid : u64, targetid : *const COMPOSITION_TARGET_ID, targetstats : *mut COMPOSITION_TARGET_STATS) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DCompositionGetTargetStatistics(frameid, targetid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DCompositionWaitForCompositorClock(handles: Option<&[super::super::Foundation::HANDLE]>, timeoutinms: u32) -> u32 {
    windows_core::link!("dcomp.dll" "system" fn DCompositionWaitForCompositorClock(count : u32, handles : *const super::super::Foundation:: HANDLE, timeoutinms : u32) -> u32);
    unsafe { DCompositionWaitForCompositorClock(handles.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(handles.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), timeoutinms) }
}
pub const COMPOSITIONOBJECT_READ: i32 = 1i32;
pub const COMPOSITIONOBJECT_WRITE: i32 = 2i32;
pub const COMPOSITION_FRAME_ID_COMPLETED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(2i32);
pub const COMPOSITION_FRAME_ID_CONFIRMED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(1i32);
pub const COMPOSITION_FRAME_ID_CREATED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMPOSITION_FRAME_ID_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMPOSITION_FRAME_STATS {
    pub startTime: u64,
    pub targetTime: u64,
    pub framePeriod: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMPOSITION_STATS {
    pub presentCount: u32,
    pub refreshCount: u32,
    pub virtualRefreshCount: u32,
    pub time: u64,
}
pub const COMPOSITION_STATS_MAX_TARGETS: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMPOSITION_TARGET_ID {
    pub displayAdapterLuid: super::super::Foundation::LUID,
    pub renderAdapterLuid: super::super::Foundation::LUID,
    pub vidPnSourceId: u32,
    pub vidPnTargetId: u32,
    pub uniqueId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMPOSITION_TARGET_STATS {
    pub outstandingPresents: u32,
    pub presentTime: u64,
    pub vblankDuration: u64,
    pub presentedStats: COMPOSITION_STATS,
    pub completedStats: COMPOSITION_STATS,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DCOMPOSITION_BACKFACE_VISIBILITY(pub i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_HIDDEN: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(1i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_INHERIT: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(-1i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_VISIBLE: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DCOMPOSITION_BITMAP_INTERPOLATION_MODE(pub i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_INHERIT: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(-1i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_LINEAR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(1i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DCOMPOSITION_BORDER_MODE(pub i32);
pub const DCOMPOSITION_BORDER_MODE_HARD: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(1i32);
pub const DCOMPOSITION_BORDER_MODE_INHERIT: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(-1i32);
pub const DCOMPOSITION_BORDER_MODE_SOFT: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DCOMPOSITION_COMPOSITE_MODE(pub i32);
pub const DCOMPOSITION_COMPOSITE_MODE_DESTINATION_INVERT: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(1i32);
pub const DCOMPOSITION_COMPOSITE_MODE_INHERIT: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(-1i32);
pub const DCOMPOSITION_COMPOSITE_MODE_MIN_BLEND: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(2i32);
pub const DCOMPOSITION_COMPOSITE_MODE_SOURCE_OVER: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DCOMPOSITION_DEPTH_MODE(pub i32);
pub const DCOMPOSITION_DEPTH_MODE_INHERIT: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(-1i32);
pub const DCOMPOSITION_DEPTH_MODE_SORTED: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(3i32);
pub const DCOMPOSITION_DEPTH_MODE_SPATIAL: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(1i32);
pub const DCOMPOSITION_DEPTH_MODE_TREE: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(0i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DCOMPOSITION_FRAME_STATISTICS {
    pub lastFrameTime: i64,
    pub currentCompositionRate: super::Dxgi::Common::DXGI_RATIONAL,
    pub currentTime: i64,
    pub timeFrequency: i64,
    pub nextEstimatedFrameTime: i64,
}
pub const DCOMPOSITION_MAX_WAITFORCOMPOSITORCLOCK_OBJECTS: u32 = 32u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DCOMPOSITION_OPACITY_MODE(pub i32);
pub const DCOMPOSITION_OPACITY_MODE_INHERIT: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(-1i32);
pub const DCOMPOSITION_OPACITY_MODE_LAYER: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(0i32);
pub const DCOMPOSITION_OPACITY_MODE_MULTIPLY: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DCompositionInkTrailPoint {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
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
        unsafe { (windows_core::Interface::vtable(self).SetInterpolationMode)(windows_core::Interface::as_raw(self), interpolationmode).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBorderMode(&self, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBorderMode)(windows_core::Interface::as_raw(self), bordermode).ok() }
    }
    pub unsafe fn SetTransformMatrix(&self, transformmatrix: *const windows_numerics::Matrix3x2) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTransformMatrix)(windows_core::Interface::as_raw(self), transformmatrix).ok() }
    }
    pub unsafe fn SetTransformMatrixElement<P2>(&self, row: i32, column: i32, animation: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTransformMatrixElement)(windows_core::Interface::as_raw(self), row, column, animation.param().abi()).ok() }
    }
    pub unsafe fn SetTransformMatrixElement2(&self, row: i32, column: i32, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTransformMatrixElement2)(windows_core::Interface::as_raw(self), row, column, value).ok() }
    }
    pub unsafe fn SetSharpness<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSharpness)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetSharpness2(&self, sharpness: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSharpness2)(windows_core::Interface::as_raw(self), sharpness).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub SetTransformMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Matrix3x2) -> windows_core::HRESULT,
    pub SetTransformMatrixElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTransformMatrixElement2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, f32) -> windows_core::HRESULT,
    pub SetSharpness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSharpness2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionAffineTransform2DEffect_Impl: IDCompositionFilterEffect_Impl {
    fn SetInterpolationMode(&self, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> windows_core::Result<()>;
    fn SetBorderMode(&self, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> windows_core::Result<()>;
    fn SetTransformMatrix(&self, transformmatrix: *const windows_numerics::Matrix3x2) -> windows_core::Result<()>;
    fn SetTransformMatrixElement(&self, row: i32, column: i32, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetTransformMatrixElement2(&self, row: i32, column: i32, value: f32) -> windows_core::Result<()>;
    fn SetSharpness(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetSharpness2(&self, sharpness: f32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionAffineTransform2DEffect_Vtbl {
    pub const fn new<Identity: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetInterpolationMode<Identity: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAffineTransform2DEffect_Impl::SetInterpolationMode(this, core::mem::transmute_copy(&interpolationmode)).into()
            }
        }
        unsafe extern "system" fn SetBorderMode<Identity: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAffineTransform2DEffect_Impl::SetBorderMode(this, core::mem::transmute_copy(&bordermode)).into()
            }
        }
        unsafe extern "system" fn SetTransformMatrix<Identity: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transformmatrix: *const windows_numerics::Matrix3x2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAffineTransform2DEffect_Impl::SetTransformMatrix(this, core::mem::transmute_copy(&transformmatrix)).into()
            }
        }
        unsafe extern "system" fn SetTransformMatrixElement<Identity: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAffineTransform2DEffect_Impl::SetTransformMatrixElement(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column), core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetTransformMatrixElement2<Identity: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAffineTransform2DEffect_Impl::SetTransformMatrixElement2(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetSharpness<Identity: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAffineTransform2DEffect_Impl::SetSharpness(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetSharpness2<Identity: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharpness: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAffineTransform2DEffect_Impl::SetSharpness2(this, core::mem::transmute_copy(&sharpness)).into()
            }
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetInterpolationMode: SetInterpolationMode::<Identity, OFFSET>,
            SetBorderMode: SetBorderMode::<Identity, OFFSET>,
            SetTransformMatrix: SetTransformMatrix::<Identity, OFFSET>,
            SetTransformMatrixElement: SetTransformMatrixElement::<Identity, OFFSET>,
            SetTransformMatrixElement2: SetTransformMatrixElement2::<Identity, OFFSET>,
            SetSharpness: SetSharpness::<Identity, OFFSET>,
            SetSharpness2: SetSharpness2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionAffineTransform2DEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionAffineTransform2DEffect {}
windows_core::imp::define_interface!(IDCompositionAnimation, IDCompositionAnimation_Vtbl, 0xcbfd91d9_51b2_45e4_b3de_d19ccfb863c5);
windows_core::imp::interface_hierarchy!(IDCompositionAnimation, windows_core::IUnknown);
impl IDCompositionAnimation {
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetAbsoluteBeginTime(&self, begintime: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAbsoluteBeginTime)(windows_core::Interface::as_raw(self), begintime).ok() }
    }
    pub unsafe fn AddCubic(&self, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddCubic)(windows_core::Interface::as_raw(self), beginoffset, constantcoefficient, linearcoefficient, quadraticcoefficient, cubiccoefficient).ok() }
    }
    pub unsafe fn AddSinusoidal(&self, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddSinusoidal)(windows_core::Interface::as_raw(self), beginoffset, bias, amplitude, frequency, phase).ok() }
    }
    pub unsafe fn AddRepeat(&self, beginoffset: f64, durationtorepeat: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddRepeat)(windows_core::Interface::as_raw(self), beginoffset, durationtorepeat).ok() }
    }
    pub unsafe fn End(&self, endoffset: f64, endvalue: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self), endoffset, endvalue).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionAnimation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAbsoluteBeginTime: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub AddCubic: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f32, f32, f32, f32) -> windows_core::HRESULT,
    pub AddSinusoidal: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f32, f32, f32, f32) -> windows_core::HRESULT,
    pub AddRepeat: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f32) -> windows_core::HRESULT,
}
pub trait IDCompositionAnimation_Impl: windows_core::IUnknownImpl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn SetAbsoluteBeginTime(&self, begintime: i64) -> windows_core::Result<()>;
    fn AddCubic(&self, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> windows_core::Result<()>;
    fn AddSinusoidal(&self, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> windows_core::Result<()>;
    fn AddRepeat(&self, beginoffset: f64, durationtorepeat: f64) -> windows_core::Result<()>;
    fn End(&self, endoffset: f64, endvalue: f32) -> windows_core::Result<()>;
}
impl IDCompositionAnimation_Vtbl {
    pub const fn new<Identity: IDCompositionAnimation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reset<Identity: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAnimation_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn SetAbsoluteBeginTime<Identity: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, begintime: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAnimation_Impl::SetAbsoluteBeginTime(this, core::mem::transmute_copy(&begintime)).into()
            }
        }
        unsafe extern "system" fn AddCubic<Identity: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAnimation_Impl::AddCubic(this, core::mem::transmute_copy(&beginoffset), core::mem::transmute_copy(&constantcoefficient), core::mem::transmute_copy(&linearcoefficient), core::mem::transmute_copy(&quadraticcoefficient), core::mem::transmute_copy(&cubiccoefficient)).into()
            }
        }
        unsafe extern "system" fn AddSinusoidal<Identity: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAnimation_Impl::AddSinusoidal(this, core::mem::transmute_copy(&beginoffset), core::mem::transmute_copy(&bias), core::mem::transmute_copy(&amplitude), core::mem::transmute_copy(&frequency), core::mem::transmute_copy(&phase)).into()
            }
        }
        unsafe extern "system" fn AddRepeat<Identity: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, beginoffset: f64, durationtorepeat: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAnimation_Impl::AddRepeat(this, core::mem::transmute_copy(&beginoffset), core::mem::transmute_copy(&durationtorepeat)).into()
            }
        }
        unsafe extern "system" fn End<Identity: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endoffset: f64, endvalue: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAnimation_Impl::End(this, core::mem::transmute_copy(&endoffset), core::mem::transmute_copy(&endvalue)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            SetAbsoluteBeginTime: SetAbsoluteBeginTime::<Identity, OFFSET>,
            AddCubic: AddCubic::<Identity, OFFSET>,
            AddSinusoidal: AddSinusoidal::<Identity, OFFSET>,
            AddRepeat: AddRepeat::<Identity, OFFSET>,
            End: End::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionAnimation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionAnimation {}
windows_core::imp::define_interface!(IDCompositionArithmeticCompositeEffect, IDCompositionArithmeticCompositeEffect_Vtbl, 0x3b67dfa8_e3dd_4e61_b640_46c2f3d739dc);
impl core::ops::Deref for IDCompositionArithmeticCompositeEffect {
    type Target = IDCompositionFilterEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionArithmeticCompositeEffect, windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl IDCompositionArithmeticCompositeEffect {
    pub unsafe fn SetCoefficients(&self, coefficients: *const windows_numerics::Vector4) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCoefficients)(windows_core::Interface::as_raw(self), coefficients).ok() }
    }
    pub unsafe fn SetClampOutput(&self, clampoutput: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClampOutput)(windows_core::Interface::as_raw(self), clampoutput.into()).ok() }
    }
    pub unsafe fn SetCoefficient1<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCoefficient1)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetCoefficient12(&self, coeffcient1: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCoefficient12)(windows_core::Interface::as_raw(self), coeffcient1).ok() }
    }
    pub unsafe fn SetCoefficient2<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCoefficient2)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetCoefficient22(&self, coefficient2: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCoefficient22)(windows_core::Interface::as_raw(self), coefficient2).ok() }
    }
    pub unsafe fn SetCoefficient3<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCoefficient3)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetCoefficient32(&self, coefficient3: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCoefficient32)(windows_core::Interface::as_raw(self), coefficient3).ok() }
    }
    pub unsafe fn SetCoefficient4<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCoefficient4)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetCoefficient42(&self, coefficient4: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCoefficient42)(windows_core::Interface::as_raw(self), coefficient4).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionArithmeticCompositeEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetCoefficients: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Vector4) -> windows_core::HRESULT,
    pub SetClampOutput: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetCoefficient1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCoefficient12: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCoefficient2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCoefficient22: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCoefficient3: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCoefficient32: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCoefficient4: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCoefficient42: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
pub trait IDCompositionArithmeticCompositeEffect_Impl: IDCompositionFilterEffect_Impl {
    fn SetCoefficients(&self, coefficients: *const windows_numerics::Vector4) -> windows_core::Result<()>;
    fn SetClampOutput(&self, clampoutput: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetCoefficient1(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCoefficient12(&self, coeffcient1: f32) -> windows_core::Result<()>;
    fn SetCoefficient2(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCoefficient22(&self, coefficient2: f32) -> windows_core::Result<()>;
    fn SetCoefficient3(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCoefficient32(&self, coefficient3: f32) -> windows_core::Result<()>;
    fn SetCoefficient4(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCoefficient42(&self, coefficient4: f32) -> windows_core::Result<()>;
}
impl IDCompositionArithmeticCompositeEffect_Vtbl {
    pub const fn new<Identity: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCoefficients<Identity: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, coefficients: *const windows_numerics::Vector4) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionArithmeticCompositeEffect_Impl::SetCoefficients(this, core::mem::transmute_copy(&coefficients)).into()
            }
        }
        unsafe extern "system" fn SetClampOutput<Identity: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clampoutput: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionArithmeticCompositeEffect_Impl::SetClampOutput(this, core::mem::transmute_copy(&clampoutput)).into()
            }
        }
        unsafe extern "system" fn SetCoefficient1<Identity: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionArithmeticCompositeEffect_Impl::SetCoefficient1(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetCoefficient12<Identity: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, coeffcient1: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionArithmeticCompositeEffect_Impl::SetCoefficient12(this, core::mem::transmute_copy(&coeffcient1)).into()
            }
        }
        unsafe extern "system" fn SetCoefficient2<Identity: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionArithmeticCompositeEffect_Impl::SetCoefficient2(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetCoefficient22<Identity: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, coefficient2: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionArithmeticCompositeEffect_Impl::SetCoefficient22(this, core::mem::transmute_copy(&coefficient2)).into()
            }
        }
        unsafe extern "system" fn SetCoefficient3<Identity: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionArithmeticCompositeEffect_Impl::SetCoefficient3(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetCoefficient32<Identity: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, coefficient3: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionArithmeticCompositeEffect_Impl::SetCoefficient32(this, core::mem::transmute_copy(&coefficient3)).into()
            }
        }
        unsafe extern "system" fn SetCoefficient4<Identity: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionArithmeticCompositeEffect_Impl::SetCoefficient4(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetCoefficient42<Identity: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, coefficient4: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionArithmeticCompositeEffect_Impl::SetCoefficient42(this, core::mem::transmute_copy(&coefficient4)).into()
            }
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetCoefficients: SetCoefficients::<Identity, OFFSET>,
            SetClampOutput: SetClampOutput::<Identity, OFFSET>,
            SetCoefficient1: SetCoefficient1::<Identity, OFFSET>,
            SetCoefficient12: SetCoefficient12::<Identity, OFFSET>,
            SetCoefficient2: SetCoefficient2::<Identity, OFFSET>,
            SetCoefficient22: SetCoefficient22::<Identity, OFFSET>,
            SetCoefficient3: SetCoefficient3::<Identity, OFFSET>,
            SetCoefficient32: SetCoefficient32::<Identity, OFFSET>,
            SetCoefficient4: SetCoefficient4::<Identity, OFFSET>,
            SetCoefficient42: SetCoefficient42::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionArithmeticCompositeEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionArithmeticCompositeEffect {}
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
        unsafe { (windows_core::Interface::vtable(self).SetMode)(windows_core::Interface::as_raw(self), mode).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionBlendEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::Direct2D::Common::D2D1_BLEND_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMode: usize,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionBlendEffect_Impl: IDCompositionFilterEffect_Impl {
    fn SetMode(&self, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionBlendEffect_Vtbl {
    pub const fn new<Identity: IDCompositionBlendEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMode<Identity: IDCompositionBlendEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionBlendEffect_Impl::SetMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        Self { base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(), SetMode: SetMode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionBlendEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionBlendEffect {}
windows_core::imp::define_interface!(IDCompositionBrightnessEffect, IDCompositionBrightnessEffect_Vtbl, 0x6027496e_cb3a_49ab_934f_d798da4f7da6);
impl core::ops::Deref for IDCompositionBrightnessEffect {
    type Target = IDCompositionFilterEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionBrightnessEffect, windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl IDCompositionBrightnessEffect {
    pub unsafe fn SetWhitePoint(&self, whitepoint: *const windows_numerics::Vector2) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWhitePoint)(windows_core::Interface::as_raw(self), whitepoint).ok() }
    }
    pub unsafe fn SetBlackPoint(&self, blackpoint: *const windows_numerics::Vector2) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBlackPoint)(windows_core::Interface::as_raw(self), blackpoint).ok() }
    }
    pub unsafe fn SetWhitePointX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetWhitePointX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetWhitePointX2(&self, whitepointx: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWhitePointX2)(windows_core::Interface::as_raw(self), whitepointx).ok() }
    }
    pub unsafe fn SetWhitePointY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetWhitePointY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetWhitePointY2(&self, whitepointy: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWhitePointY2)(windows_core::Interface::as_raw(self), whitepointy).ok() }
    }
    pub unsafe fn SetBlackPointX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBlackPointX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetBlackPointX2(&self, blackpointx: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBlackPointX2)(windows_core::Interface::as_raw(self), blackpointx).ok() }
    }
    pub unsafe fn SetBlackPointY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBlackPointY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetBlackPointY2(&self, blackpointy: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBlackPointY2)(windows_core::Interface::as_raw(self), blackpointy).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionBrightnessEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetWhitePoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Vector2) -> windows_core::HRESULT,
    pub SetBlackPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Vector2) -> windows_core::HRESULT,
    pub SetWhitePointX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWhitePointX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetWhitePointY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWhitePointY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetBlackPointX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBlackPointX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetBlackPointY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBlackPointY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
pub trait IDCompositionBrightnessEffect_Impl: IDCompositionFilterEffect_Impl {
    fn SetWhitePoint(&self, whitepoint: *const windows_numerics::Vector2) -> windows_core::Result<()>;
    fn SetBlackPoint(&self, blackpoint: *const windows_numerics::Vector2) -> windows_core::Result<()>;
    fn SetWhitePointX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetWhitePointX2(&self, whitepointx: f32) -> windows_core::Result<()>;
    fn SetWhitePointY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetWhitePointY2(&self, whitepointy: f32) -> windows_core::Result<()>;
    fn SetBlackPointX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBlackPointX2(&self, blackpointx: f32) -> windows_core::Result<()>;
    fn SetBlackPointY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBlackPointY2(&self, blackpointy: f32) -> windows_core::Result<()>;
}
impl IDCompositionBrightnessEffect_Vtbl {
    pub const fn new<Identity: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetWhitePoint<Identity: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, whitepoint: *const windows_numerics::Vector2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionBrightnessEffect_Impl::SetWhitePoint(this, core::mem::transmute_copy(&whitepoint)).into()
            }
        }
        unsafe extern "system" fn SetBlackPoint<Identity: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blackpoint: *const windows_numerics::Vector2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionBrightnessEffect_Impl::SetBlackPoint(this, core::mem::transmute_copy(&blackpoint)).into()
            }
        }
        unsafe extern "system" fn SetWhitePointX<Identity: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionBrightnessEffect_Impl::SetWhitePointX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetWhitePointX2<Identity: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, whitepointx: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionBrightnessEffect_Impl::SetWhitePointX2(this, core::mem::transmute_copy(&whitepointx)).into()
            }
        }
        unsafe extern "system" fn SetWhitePointY<Identity: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionBrightnessEffect_Impl::SetWhitePointY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetWhitePointY2<Identity: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, whitepointy: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionBrightnessEffect_Impl::SetWhitePointY2(this, core::mem::transmute_copy(&whitepointy)).into()
            }
        }
        unsafe extern "system" fn SetBlackPointX<Identity: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionBrightnessEffect_Impl::SetBlackPointX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetBlackPointX2<Identity: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blackpointx: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionBrightnessEffect_Impl::SetBlackPointX2(this, core::mem::transmute_copy(&blackpointx)).into()
            }
        }
        unsafe extern "system" fn SetBlackPointY<Identity: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionBrightnessEffect_Impl::SetBlackPointY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetBlackPointY2<Identity: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blackpointy: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionBrightnessEffect_Impl::SetBlackPointY2(this, core::mem::transmute_copy(&blackpointy)).into()
            }
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetWhitePoint: SetWhitePoint::<Identity, OFFSET>,
            SetBlackPoint: SetBlackPoint::<Identity, OFFSET>,
            SetWhitePointX: SetWhitePointX::<Identity, OFFSET>,
            SetWhitePointX2: SetWhitePointX2::<Identity, OFFSET>,
            SetWhitePointY: SetWhitePointY::<Identity, OFFSET>,
            SetWhitePointY2: SetWhitePointY2::<Identity, OFFSET>,
            SetBlackPointX: SetBlackPointX::<Identity, OFFSET>,
            SetBlackPointX2: SetBlackPointX2::<Identity, OFFSET>,
            SetBlackPointY: SetBlackPointY::<Identity, OFFSET>,
            SetBlackPointY2: SetBlackPointY2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionBrightnessEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionBrightnessEffect {}
windows_core::imp::define_interface!(IDCompositionClip, IDCompositionClip_Vtbl, 0x64ac3703_9d3f_45ec_a109_7cac0e7a13a7);
windows_core::imp::interface_hierarchy!(IDCompositionClip, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionClip_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IDCompositionClip_Impl: windows_core::IUnknownImpl {}
impl IDCompositionClip_Vtbl {
    pub const fn new<Identity: IDCompositionClip_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionClip as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionClip {}
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
        unsafe { (windows_core::Interface::vtable(self).SetMatrix)(windows_core::Interface::as_raw(self), matrix).ok() }
    }
    pub unsafe fn SetMatrixElement<P2>(&self, row: i32, column: i32, animation: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMatrixElement)(windows_core::Interface::as_raw(self), row, column, animation.param().abi()).ok() }
    }
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMatrixElement2)(windows_core::Interface::as_raw(self), row, column, value).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetAlphaMode(&self, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAlphaMode)(windows_core::Interface::as_raw(self), mode).ok() }
    }
    pub unsafe fn SetClampOutput(&self, clamp: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClampOutput)(windows_core::Interface::as_raw(self), clamp.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub SetClampOutput: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionColorMatrixEffect_Impl: IDCompositionFilterEffect_Impl {
    fn SetMatrix(&self, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> windows_core::Result<()>;
    fn SetMatrixElement(&self, row: i32, column: i32, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> windows_core::Result<()>;
    fn SetAlphaMode(&self, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> windows_core::Result<()>;
    fn SetClampOutput(&self, clamp: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionColorMatrixEffect_Vtbl {
    pub const fn new<Identity: IDCompositionColorMatrixEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMatrix<Identity: IDCompositionColorMatrixEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionColorMatrixEffect_Impl::SetMatrix(this, core::mem::transmute_copy(&matrix)).into()
            }
        }
        unsafe extern "system" fn SetMatrixElement<Identity: IDCompositionColorMatrixEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionColorMatrixEffect_Impl::SetMatrixElement(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column), core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetMatrixElement2<Identity: IDCompositionColorMatrixEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionColorMatrixEffect_Impl::SetMatrixElement2(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetAlphaMode<Identity: IDCompositionColorMatrixEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionColorMatrixEffect_Impl::SetAlphaMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn SetClampOutput<Identity: IDCompositionColorMatrixEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clamp: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionColorMatrixEffect_Impl::SetClampOutput(this, core::mem::transmute_copy(&clamp)).into()
            }
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetMatrix: SetMatrix::<Identity, OFFSET>,
            SetMatrixElement: SetMatrixElement::<Identity, OFFSET>,
            SetMatrixElement2: SetMatrixElement2::<Identity, OFFSET>,
            SetAlphaMode: SetAlphaMode::<Identity, OFFSET>,
            SetClampOutput: SetClampOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionColorMatrixEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionColorMatrixEffect {}
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
        unsafe { (windows_core::Interface::vtable(self).SetMode)(windows_core::Interface::as_raw(self), mode).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionCompositeEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMode: usize,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionCompositeEffect_Impl: IDCompositionFilterEffect_Impl {
    fn SetMode(&self, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionCompositeEffect_Vtbl {
    pub const fn new<Identity: IDCompositionCompositeEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMode<Identity: IDCompositionCompositeEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionCompositeEffect_Impl::SetMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        Self { base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(), SetMode: SetMode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionCompositeEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionCompositeEffect {}
windows_core::imp::define_interface!(IDCompositionDelegatedInkTrail, IDCompositionDelegatedInkTrail_Vtbl, 0xc2448e9b_547d_4057_8cf5_8144ede1c2da);
windows_core::imp::interface_hierarchy!(IDCompositionDelegatedInkTrail, windows_core::IUnknown);
impl IDCompositionDelegatedInkTrail {
    pub unsafe fn AddTrailPoints(&self, inkpoints: &[DCompositionInkTrailPoint]) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddTrailPoints)(windows_core::Interface::as_raw(self), core::mem::transmute(inkpoints.as_ptr()), inkpoints.len().try_into().unwrap(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddTrailPointsWithPrediction(&self, inkpoints: &[DCompositionInkTrailPoint], predictedinkpoints: &[DCompositionInkTrailPoint]) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddTrailPointsWithPrediction)(windows_core::Interface::as_raw(self), core::mem::transmute(inkpoints.as_ptr()), inkpoints.len().try_into().unwrap(), core::mem::transmute(predictedinkpoints.as_ptr()), predictedinkpoints.len().try_into().unwrap(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveTrailPoints(&self, generationid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveTrailPoints)(windows_core::Interface::as_raw(self), generationid).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn StartNewTrail(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartNewTrail)(windows_core::Interface::as_raw(self), color).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionDelegatedInkTrail_Impl: windows_core::IUnknownImpl {
    fn AddTrailPoints(&self, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32) -> windows_core::Result<u32>;
    fn AddTrailPointsWithPrediction(&self, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32) -> windows_core::Result<u32>;
    fn RemoveTrailPoints(&self, generationid: u32) -> windows_core::Result<()>;
    fn StartNewTrail(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionDelegatedInkTrail_Vtbl {
    pub const fn new<Identity: IDCompositionDelegatedInkTrail_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddTrailPoints<Identity: IDCompositionDelegatedInkTrail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, generationid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDelegatedInkTrail_Impl::AddTrailPoints(this, core::mem::transmute_copy(&inkpoints), core::mem::transmute_copy(&inkpointscount)) {
                    Ok(ok__) => {
                        generationid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddTrailPointsWithPrediction<Identity: IDCompositionDelegatedInkTrail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32, generationid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDelegatedInkTrail_Impl::AddTrailPointsWithPrediction(this, core::mem::transmute_copy(&inkpoints), core::mem::transmute_copy(&inkpointscount), core::mem::transmute_copy(&predictedinkpoints), core::mem::transmute_copy(&predictedinkpointscount)) {
                    Ok(ok__) => {
                        generationid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveTrailPoints<Identity: IDCompositionDelegatedInkTrail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, generationid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionDelegatedInkTrail_Impl::RemoveTrailPoints(this, core::mem::transmute_copy(&generationid)).into()
            }
        }
        unsafe extern "system" fn StartNewTrail<Identity: IDCompositionDelegatedInkTrail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionDelegatedInkTrail_Impl::StartNewTrail(this, core::mem::transmute_copy(&color)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddTrailPoints: AddTrailPoints::<Identity, OFFSET>,
            AddTrailPointsWithPrediction: AddTrailPointsWithPrediction::<Identity, OFFSET>,
            RemoveTrailPoints: RemoveTrailPoints::<Identity, OFFSET>,
            StartNewTrail: StartNewTrail::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionDelegatedInkTrail as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionDelegatedInkTrail {}
windows_core::imp::define_interface!(IDCompositionDesktopDevice, IDCompositionDesktopDevice_Vtbl, 0x5f4633fe_1e08_4cb8_8c75_ce24333f5602);
impl core::ops::Deref for IDCompositionDesktopDevice {
    type Target = IDCompositionDevice2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionDesktopDevice, windows_core::IUnknown, IDCompositionDevice2);
impl IDCompositionDesktopDevice {
    pub unsafe fn CreateTargetForHwnd(&self, hwnd: super::super::Foundation::HWND, topmost: bool) -> windows_core::Result<IDCompositionTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTargetForHwnd)(windows_core::Interface::as_raw(self), hwnd, topmost.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSurfaceFromHandle(&self, handle: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSurfaceFromHandle)(windows_core::Interface::as_raw(self), handle, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSurfaceFromHwnd(&self, hwnd: super::super::Foundation::HWND) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSurfaceFromHwnd)(windows_core::Interface::as_raw(self), hwnd, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDesktopDevice_Vtbl {
    pub base__: IDCompositionDevice2_Vtbl,
    pub CreateTargetForHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSurfaceFromHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSurfaceFromHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionDesktopDevice_Impl: IDCompositionDevice2_Impl {
    fn CreateTargetForHwnd(&self, hwnd: super::super::Foundation::HWND, topmost: windows_core::BOOL) -> windows_core::Result<IDCompositionTarget>;
    fn CreateSurfaceFromHandle(&self, handle: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::IUnknown>;
    fn CreateSurfaceFromHwnd(&self, hwnd: super::super::Foundation::HWND) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionDesktopDevice_Vtbl {
    pub const fn new<Identity: IDCompositionDesktopDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTargetForHwnd<Identity: IDCompositionDesktopDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: windows_core::BOOL, target: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDesktopDevice_Impl::CreateTargetForHwnd(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&topmost)) {
                    Ok(ok__) => {
                        target.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHandle<Identity: IDCompositionDesktopDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDesktopDevice_Impl::CreateSurfaceFromHandle(this, core::mem::transmute_copy(&handle)) {
                    Ok(ok__) => {
                        surface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHwnd<Identity: IDCompositionDesktopDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDesktopDevice_Impl::CreateSurfaceFromHwnd(this, core::mem::transmute_copy(&hwnd)) {
                    Ok(ok__) => {
                        surface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDCompositionDevice2_Vtbl::new::<Identity, OFFSET>(),
            CreateTargetForHwnd: CreateTargetForHwnd::<Identity, OFFSET>,
            CreateSurfaceFromHandle: CreateSurfaceFromHandle::<Identity, OFFSET>,
            CreateSurfaceFromHwnd: CreateSurfaceFromHwnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionDesktopDevice as windows_core::Interface>::IID || iid == &<IDCompositionDevice2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDCompositionDesktopDevice {}
windows_core::imp::define_interface!(IDCompositionDevice, IDCompositionDevice_Vtbl, 0xc37ea93a_e7aa_450d_b16f_9746cb0407f3);
windows_core::imp::interface_hierarchy!(IDCompositionDevice, windows_core::IUnknown);
impl IDCompositionDevice {
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WaitForCommitCompletion)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self) -> windows_core::Result<DCOMPOSITION_FRAME_STATISTICS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrameStatistics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateTargetForHwnd(&self, hwnd: super::super::Foundation::HWND, topmost: bool) -> windows_core::Result<IDCompositionTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTargetForHwnd)(windows_core::Interface::as_raw(self), hwnd, topmost.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateVisual(&self) -> windows_core::Result<IDCompositionVisual> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVisual)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionSurface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSurface)(windows_core::Interface::as_raw(self), width, height, pixelformat, alphamode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionVirtualSurface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVirtualSurface)(windows_core::Interface::as_raw(self), initialwidth, initialheight, pixelformat, alphamode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSurfaceFromHandle(&self, handle: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSurfaceFromHandle)(windows_core::Interface::as_raw(self), handle, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSurfaceFromHwnd(&self, hwnd: super::super::Foundation::HWND) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSurfaceFromHwnd)(windows_core::Interface::as_raw(self), hwnd, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTranslateTransform(&self) -> windows_core::Result<IDCompositionTranslateTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTranslateTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateScaleTransform(&self) -> windows_core::Result<IDCompositionScaleTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateScaleTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRotateTransform(&self) -> windows_core::Result<IDCompositionRotateTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRotateTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSkewTransform(&self) -> windows_core::Result<IDCompositionSkewTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSkewTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateMatrixTransform(&self) -> windows_core::Result<IDCompositionMatrixTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMatrixTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTransformGroup(&self, transforms: &[Option<IDCompositionTransform>]) -> windows_core::Result<IDCompositionTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTransformGroup)(windows_core::Interface::as_raw(self), core::mem::transmute(transforms.as_ptr()), transforms.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTranslateTransform3D(&self) -> windows_core::Result<IDCompositionTranslateTransform3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTranslateTransform3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateScaleTransform3D(&self) -> windows_core::Result<IDCompositionScaleTransform3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateScaleTransform3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRotateTransform3D(&self) -> windows_core::Result<IDCompositionRotateTransform3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRotateTransform3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateMatrixTransform3D(&self) -> windows_core::Result<IDCompositionMatrixTransform3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMatrixTransform3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[Option<IDCompositionTransform3D>]) -> windows_core::Result<IDCompositionTransform3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTransform3DGroup)(windows_core::Interface::as_raw(self), core::mem::transmute(transforms3d.as_ptr()), transforms3d.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateEffectGroup(&self) -> windows_core::Result<IDCompositionEffectGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEffectGroup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRectangleClip(&self) -> windows_core::Result<IDCompositionRectangleClip> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRectangleClip)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAnimation(&self) -> windows_core::Result<IDCompositionAnimation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAnimation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CheckDeviceState(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckDeviceState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForCommitCompletion: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetFrameStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DCOMPOSITION_FRAME_STATISTICS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetFrameStatistics: usize,
    pub CreateTargetForHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
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
    pub CheckDeviceState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionDevice_Impl: windows_core::IUnknownImpl {
    fn Commit(&self) -> windows_core::Result<()>;
    fn WaitForCommitCompletion(&self) -> windows_core::Result<()>;
    fn GetFrameStatistics(&self) -> windows_core::Result<DCOMPOSITION_FRAME_STATISTICS>;
    fn CreateTargetForHwnd(&self, hwnd: super::super::Foundation::HWND, topmost: windows_core::BOOL) -> windows_core::Result<IDCompositionTarget>;
    fn CreateVisual(&self) -> windows_core::Result<IDCompositionVisual>;
    fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionSurface>;
    fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionVirtualSurface>;
    fn CreateSurfaceFromHandle(&self, handle: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::IUnknown>;
    fn CreateSurfaceFromHwnd(&self, hwnd: super::super::Foundation::HWND) -> windows_core::Result<windows_core::IUnknown>;
    fn CreateTranslateTransform(&self) -> windows_core::Result<IDCompositionTranslateTransform>;
    fn CreateScaleTransform(&self) -> windows_core::Result<IDCompositionScaleTransform>;
    fn CreateRotateTransform(&self) -> windows_core::Result<IDCompositionRotateTransform>;
    fn CreateSkewTransform(&self) -> windows_core::Result<IDCompositionSkewTransform>;
    fn CreateMatrixTransform(&self) -> windows_core::Result<IDCompositionMatrixTransform>;
    fn CreateTransformGroup(&self, transforms: *const Option<IDCompositionTransform>, elements: u32) -> windows_core::Result<IDCompositionTransform>;
    fn CreateTranslateTransform3D(&self) -> windows_core::Result<IDCompositionTranslateTransform3D>;
    fn CreateScaleTransform3D(&self) -> windows_core::Result<IDCompositionScaleTransform3D>;
    fn CreateRotateTransform3D(&self) -> windows_core::Result<IDCompositionRotateTransform3D>;
    fn CreateMatrixTransform3D(&self) -> windows_core::Result<IDCompositionMatrixTransform3D>;
    fn CreateTransform3DGroup(&self, transforms3d: *const Option<IDCompositionTransform3D>, elements: u32) -> windows_core::Result<IDCompositionTransform3D>;
    fn CreateEffectGroup(&self) -> windows_core::Result<IDCompositionEffectGroup>;
    fn CreateRectangleClip(&self) -> windows_core::Result<IDCompositionRectangleClip>;
    fn CreateAnimation(&self) -> windows_core::Result<IDCompositionAnimation>;
    fn CheckDeviceState(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionDevice_Vtbl {
    pub const fn new<Identity: IDCompositionDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Commit<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionDevice_Impl::Commit(this).into()
            }
        }
        unsafe extern "system" fn WaitForCommitCompletion<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionDevice_Impl::WaitForCommitCompletion(this).into()
            }
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::GetFrameStatistics(this) {
                    Ok(ok__) => {
                        statistics.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTargetForHwnd<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: windows_core::BOOL, target: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateTargetForHwnd(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&topmost)) {
                    Ok(ok__) => {
                        target.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateVisual<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateVisual(this) {
                    Ok(ok__) => {
                        visual.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateSurface(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&alphamode)) {
                    Ok(ok__) => {
                        surface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateVirtualSurface(this, core::mem::transmute_copy(&initialwidth), core::mem::transmute_copy(&initialheight), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&alphamode)) {
                    Ok(ok__) => {
                        virtualsurface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHandle<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateSurfaceFromHandle(this, core::mem::transmute_copy(&handle)) {
                    Ok(ok__) => {
                        surface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHwnd<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateSurfaceFromHwnd(this, core::mem::transmute_copy(&hwnd)) {
                    Ok(ok__) => {
                        surface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTranslateTransform<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, translatetransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateTranslateTransform(this) {
                    Ok(ok__) => {
                        translatetransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateScaleTransform<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scaletransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateScaleTransform(this) {
                    Ok(ok__) => {
                        scaletransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRotateTransform<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotatetransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateRotateTransform(this) {
                    Ok(ok__) => {
                        rotatetransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSkewTransform<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, skewtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateSkewTransform(this) {
                    Ok(ok__) => {
                        skewtransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateMatrixTransform(this) {
                    Ok(ok__) => {
                        matrixtransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTransformGroup<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transforms: *const *mut core::ffi::c_void, elements: u32, transformgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateTransformGroup(this, core::mem::transmute_copy(&transforms), core::mem::transmute_copy(&elements)) {
                    Ok(ok__) => {
                        transformgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTranslateTransform3D<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, translatetransform3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateTranslateTransform3D(this) {
                    Ok(ok__) => {
                        translatetransform3d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateScaleTransform3D<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scaletransform3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateScaleTransform3D(this) {
                    Ok(ok__) => {
                        scaletransform3d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRotateTransform3D<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotatetransform3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateRotateTransform3D(this) {
                    Ok(ok__) => {
                        rotatetransform3d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMatrixTransform3D<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateMatrixTransform3D(this) {
                    Ok(ok__) => {
                        matrixtransform3d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTransform3DGroup<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transforms3d: *const *mut core::ffi::c_void, elements: u32, transform3dgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateTransform3DGroup(this, core::mem::transmute_copy(&transforms3d), core::mem::transmute_copy(&elements)) {
                    Ok(ok__) => {
                        transform3dgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEffectGroup<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, effectgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateEffectGroup(this) {
                    Ok(ok__) => {
                        effectgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRectangleClip<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clip: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateRectangleClip(this) {
                    Ok(ok__) => {
                        clip.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAnimation<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CreateAnimation(this) {
                    Ok(ok__) => {
                        animation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckDeviceState<Identity: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfvalid: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice_Impl::CheckDeviceState(this) {
                    Ok(ok__) => {
                        pfvalid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, OFFSET>,
            WaitForCommitCompletion: WaitForCommitCompletion::<Identity, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, OFFSET>,
            CreateTargetForHwnd: CreateTargetForHwnd::<Identity, OFFSET>,
            CreateVisual: CreateVisual::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            CreateVirtualSurface: CreateVirtualSurface::<Identity, OFFSET>,
            CreateSurfaceFromHandle: CreateSurfaceFromHandle::<Identity, OFFSET>,
            CreateSurfaceFromHwnd: CreateSurfaceFromHwnd::<Identity, OFFSET>,
            CreateTranslateTransform: CreateTranslateTransform::<Identity, OFFSET>,
            CreateScaleTransform: CreateScaleTransform::<Identity, OFFSET>,
            CreateRotateTransform: CreateRotateTransform::<Identity, OFFSET>,
            CreateSkewTransform: CreateSkewTransform::<Identity, OFFSET>,
            CreateMatrixTransform: CreateMatrixTransform::<Identity, OFFSET>,
            CreateTransformGroup: CreateTransformGroup::<Identity, OFFSET>,
            CreateTranslateTransform3D: CreateTranslateTransform3D::<Identity, OFFSET>,
            CreateScaleTransform3D: CreateScaleTransform3D::<Identity, OFFSET>,
            CreateRotateTransform3D: CreateRotateTransform3D::<Identity, OFFSET>,
            CreateMatrixTransform3D: CreateMatrixTransform3D::<Identity, OFFSET>,
            CreateTransform3DGroup: CreateTransform3DGroup::<Identity, OFFSET>,
            CreateEffectGroup: CreateEffectGroup::<Identity, OFFSET>,
            CreateRectangleClip: CreateRectangleClip::<Identity, OFFSET>,
            CreateAnimation: CreateAnimation::<Identity, OFFSET>,
            CheckDeviceState: CheckDeviceState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionDevice as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDCompositionDevice {}
windows_core::imp::define_interface!(IDCompositionDevice2, IDCompositionDevice2_Vtbl, 0x75f6468d_1b8e_447c_9bc6_75fea80b5b25);
windows_core::imp::interface_hierarchy!(IDCompositionDevice2, windows_core::IUnknown);
impl IDCompositionDevice2 {
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WaitForCommitCompletion)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self) -> windows_core::Result<DCOMPOSITION_FRAME_STATISTICS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrameStatistics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateVisual(&self) -> windows_core::Result<IDCompositionVisual2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVisual)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSurfaceFactory<P0>(&self, renderingdevice: P0) -> windows_core::Result<IDCompositionSurfaceFactory>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSurfaceFactory)(windows_core::Interface::as_raw(self), renderingdevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionSurface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSurface)(windows_core::Interface::as_raw(self), width, height, pixelformat, alphamode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionVirtualSurface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVirtualSurface)(windows_core::Interface::as_raw(self), initialwidth, initialheight, pixelformat, alphamode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTranslateTransform(&self) -> windows_core::Result<IDCompositionTranslateTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTranslateTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateScaleTransform(&self) -> windows_core::Result<IDCompositionScaleTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateScaleTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRotateTransform(&self) -> windows_core::Result<IDCompositionRotateTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRotateTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSkewTransform(&self) -> windows_core::Result<IDCompositionSkewTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSkewTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateMatrixTransform(&self) -> windows_core::Result<IDCompositionMatrixTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMatrixTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTransformGroup(&self, transforms: &[Option<IDCompositionTransform>]) -> windows_core::Result<IDCompositionTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTransformGroup)(windows_core::Interface::as_raw(self), core::mem::transmute(transforms.as_ptr()), transforms.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTranslateTransform3D(&self) -> windows_core::Result<IDCompositionTranslateTransform3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTranslateTransform3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateScaleTransform3D(&self) -> windows_core::Result<IDCompositionScaleTransform3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateScaleTransform3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRotateTransform3D(&self) -> windows_core::Result<IDCompositionRotateTransform3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRotateTransform3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateMatrixTransform3D(&self) -> windows_core::Result<IDCompositionMatrixTransform3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMatrixTransform3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[Option<IDCompositionTransform3D>]) -> windows_core::Result<IDCompositionTransform3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTransform3DGroup)(windows_core::Interface::as_raw(self), core::mem::transmute(transforms3d.as_ptr()), transforms3d.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateEffectGroup(&self) -> windows_core::Result<IDCompositionEffectGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEffectGroup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRectangleClip(&self) -> windows_core::Result<IDCompositionRectangleClip> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRectangleClip)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAnimation(&self) -> windows_core::Result<IDCompositionAnimation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAnimation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionDevice2_Impl: windows_core::IUnknownImpl {
    fn Commit(&self) -> windows_core::Result<()>;
    fn WaitForCommitCompletion(&self) -> windows_core::Result<()>;
    fn GetFrameStatistics(&self) -> windows_core::Result<DCOMPOSITION_FRAME_STATISTICS>;
    fn CreateVisual(&self) -> windows_core::Result<IDCompositionVisual2>;
    fn CreateSurfaceFactory(&self, renderingdevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<IDCompositionSurfaceFactory>;
    fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionSurface>;
    fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionVirtualSurface>;
    fn CreateTranslateTransform(&self) -> windows_core::Result<IDCompositionTranslateTransform>;
    fn CreateScaleTransform(&self) -> windows_core::Result<IDCompositionScaleTransform>;
    fn CreateRotateTransform(&self) -> windows_core::Result<IDCompositionRotateTransform>;
    fn CreateSkewTransform(&self) -> windows_core::Result<IDCompositionSkewTransform>;
    fn CreateMatrixTransform(&self) -> windows_core::Result<IDCompositionMatrixTransform>;
    fn CreateTransformGroup(&self, transforms: *const Option<IDCompositionTransform>, elements: u32) -> windows_core::Result<IDCompositionTransform>;
    fn CreateTranslateTransform3D(&self) -> windows_core::Result<IDCompositionTranslateTransform3D>;
    fn CreateScaleTransform3D(&self) -> windows_core::Result<IDCompositionScaleTransform3D>;
    fn CreateRotateTransform3D(&self) -> windows_core::Result<IDCompositionRotateTransform3D>;
    fn CreateMatrixTransform3D(&self) -> windows_core::Result<IDCompositionMatrixTransform3D>;
    fn CreateTransform3DGroup(&self, transforms3d: *const Option<IDCompositionTransform3D>, elements: u32) -> windows_core::Result<IDCompositionTransform3D>;
    fn CreateEffectGroup(&self) -> windows_core::Result<IDCompositionEffectGroup>;
    fn CreateRectangleClip(&self) -> windows_core::Result<IDCompositionRectangleClip>;
    fn CreateAnimation(&self) -> windows_core::Result<IDCompositionAnimation>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionDevice2_Vtbl {
    pub const fn new<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Commit<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionDevice2_Impl::Commit(this).into()
            }
        }
        unsafe extern "system" fn WaitForCommitCompletion<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionDevice2_Impl::WaitForCommitCompletion(this).into()
            }
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::GetFrameStatistics(this) {
                    Ok(ok__) => {
                        statistics.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateVisual<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateVisual(this) {
                    Ok(ok__) => {
                        visual.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSurfaceFactory<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingdevice: *mut core::ffi::c_void, surfacefactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateSurfaceFactory(this, core::mem::transmute_copy(&renderingdevice)) {
                    Ok(ok__) => {
                        surfacefactory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateSurface(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&alphamode)) {
                    Ok(ok__) => {
                        surface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateVirtualSurface(this, core::mem::transmute_copy(&initialwidth), core::mem::transmute_copy(&initialheight), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&alphamode)) {
                    Ok(ok__) => {
                        virtualsurface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTranslateTransform<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, translatetransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateTranslateTransform(this) {
                    Ok(ok__) => {
                        translatetransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateScaleTransform<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scaletransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateScaleTransform(this) {
                    Ok(ok__) => {
                        scaletransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRotateTransform<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotatetransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateRotateTransform(this) {
                    Ok(ok__) => {
                        rotatetransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSkewTransform<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, skewtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateSkewTransform(this) {
                    Ok(ok__) => {
                        skewtransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateMatrixTransform(this) {
                    Ok(ok__) => {
                        matrixtransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTransformGroup<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transforms: *const *mut core::ffi::c_void, elements: u32, transformgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateTransformGroup(this, core::mem::transmute_copy(&transforms), core::mem::transmute_copy(&elements)) {
                    Ok(ok__) => {
                        transformgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTranslateTransform3D<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, translatetransform3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateTranslateTransform3D(this) {
                    Ok(ok__) => {
                        translatetransform3d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateScaleTransform3D<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scaletransform3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateScaleTransform3D(this) {
                    Ok(ok__) => {
                        scaletransform3d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRotateTransform3D<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotatetransform3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateRotateTransform3D(this) {
                    Ok(ok__) => {
                        rotatetransform3d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMatrixTransform3D<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateMatrixTransform3D(this) {
                    Ok(ok__) => {
                        matrixtransform3d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTransform3DGroup<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transforms3d: *const *mut core::ffi::c_void, elements: u32, transform3dgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateTransform3DGroup(this, core::mem::transmute_copy(&transforms3d), core::mem::transmute_copy(&elements)) {
                    Ok(ok__) => {
                        transform3dgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEffectGroup<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, effectgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateEffectGroup(this) {
                    Ok(ok__) => {
                        effectgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRectangleClip<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clip: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateRectangleClip(this) {
                    Ok(ok__) => {
                        clip.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAnimation<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateAnimation(this) {
                    Ok(ok__) => {
                        animation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, OFFSET>,
            WaitForCommitCompletion: WaitForCommitCompletion::<Identity, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, OFFSET>,
            CreateVisual: CreateVisual::<Identity, OFFSET>,
            CreateSurfaceFactory: CreateSurfaceFactory::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            CreateVirtualSurface: CreateVirtualSurface::<Identity, OFFSET>,
            CreateTranslateTransform: CreateTranslateTransform::<Identity, OFFSET>,
            CreateScaleTransform: CreateScaleTransform::<Identity, OFFSET>,
            CreateRotateTransform: CreateRotateTransform::<Identity, OFFSET>,
            CreateSkewTransform: CreateSkewTransform::<Identity, OFFSET>,
            CreateMatrixTransform: CreateMatrixTransform::<Identity, OFFSET>,
            CreateTransformGroup: CreateTransformGroup::<Identity, OFFSET>,
            CreateTranslateTransform3D: CreateTranslateTransform3D::<Identity, OFFSET>,
            CreateScaleTransform3D: CreateScaleTransform3D::<Identity, OFFSET>,
            CreateRotateTransform3D: CreateRotateTransform3D::<Identity, OFFSET>,
            CreateMatrixTransform3D: CreateMatrixTransform3D::<Identity, OFFSET>,
            CreateTransform3DGroup: CreateTransform3DGroup::<Identity, OFFSET>,
            CreateEffectGroup: CreateEffectGroup::<Identity, OFFSET>,
            CreateRectangleClip: CreateRectangleClip::<Identity, OFFSET>,
            CreateAnimation: CreateAnimation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionDevice2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDCompositionDevice2 {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGaussianBlurEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBrightnessEffect(&self) -> windows_core::Result<IDCompositionBrightnessEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBrightnessEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateColorMatrixEffect(&self) -> windows_core::Result<IDCompositionColorMatrixEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateColorMatrixEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateShadowEffect(&self) -> windows_core::Result<IDCompositionShadowEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateShadowEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateHueRotationEffect(&self) -> windows_core::Result<IDCompositionHueRotationEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateHueRotationEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSaturationEffect(&self) -> windows_core::Result<IDCompositionSaturationEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSaturationEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTurbulenceEffect(&self) -> windows_core::Result<IDCompositionTurbulenceEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTurbulenceEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateLinearTransferEffect(&self) -> windows_core::Result<IDCompositionLinearTransferEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateLinearTransferEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTableTransferEffect(&self) -> windows_core::Result<IDCompositionTableTransferEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTableTransferEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCompositeEffect(&self) -> windows_core::Result<IDCompositionCompositeEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCompositeEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBlendEffect(&self) -> windows_core::Result<IDCompositionBlendEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlendEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateArithmeticCompositeEffect(&self) -> windows_core::Result<IDCompositionArithmeticCompositeEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateArithmeticCompositeEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAffineTransform2DEffect(&self) -> windows_core::Result<IDCompositionAffineTransform2DEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAffineTransform2DEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionDevice3_Impl: IDCompositionDevice2_Impl {
    fn CreateGaussianBlurEffect(&self) -> windows_core::Result<IDCompositionGaussianBlurEffect>;
    fn CreateBrightnessEffect(&self) -> windows_core::Result<IDCompositionBrightnessEffect>;
    fn CreateColorMatrixEffect(&self) -> windows_core::Result<IDCompositionColorMatrixEffect>;
    fn CreateShadowEffect(&self) -> windows_core::Result<IDCompositionShadowEffect>;
    fn CreateHueRotationEffect(&self) -> windows_core::Result<IDCompositionHueRotationEffect>;
    fn CreateSaturationEffect(&self) -> windows_core::Result<IDCompositionSaturationEffect>;
    fn CreateTurbulenceEffect(&self) -> windows_core::Result<IDCompositionTurbulenceEffect>;
    fn CreateLinearTransferEffect(&self) -> windows_core::Result<IDCompositionLinearTransferEffect>;
    fn CreateTableTransferEffect(&self) -> windows_core::Result<IDCompositionTableTransferEffect>;
    fn CreateCompositeEffect(&self) -> windows_core::Result<IDCompositionCompositeEffect>;
    fn CreateBlendEffect(&self) -> windows_core::Result<IDCompositionBlendEffect>;
    fn CreateArithmeticCompositeEffect(&self) -> windows_core::Result<IDCompositionArithmeticCompositeEffect>;
    fn CreateAffineTransform2DEffect(&self) -> windows_core::Result<IDCompositionAffineTransform2DEffect>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionDevice3_Vtbl {
    pub const fn new<Identity: IDCompositionDevice3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateGaussianBlurEffect<Identity: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gaussianblureffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice3_Impl::CreateGaussianBlurEffect(this) {
                    Ok(ok__) => {
                        gaussianblureffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBrightnessEffect<Identity: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brightnesseffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice3_Impl::CreateBrightnessEffect(this) {
                    Ok(ok__) => {
                        brightnesseffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateColorMatrixEffect<Identity: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colormatrixeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice3_Impl::CreateColorMatrixEffect(this) {
                    Ok(ok__) => {
                        colormatrixeffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateShadowEffect<Identity: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, shadoweffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice3_Impl::CreateShadowEffect(this) {
                    Ok(ok__) => {
                        shadoweffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateHueRotationEffect<Identity: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, huerotationeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice3_Impl::CreateHueRotationEffect(this) {
                    Ok(ok__) => {
                        huerotationeffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSaturationEffect<Identity: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, saturationeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice3_Impl::CreateSaturationEffect(this) {
                    Ok(ok__) => {
                        saturationeffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTurbulenceEffect<Identity: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, turbulenceeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice3_Impl::CreateTurbulenceEffect(this) {
                    Ok(ok__) => {
                        turbulenceeffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateLinearTransferEffect<Identity: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lineartransfereffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice3_Impl::CreateLinearTransferEffect(this) {
                    Ok(ok__) => {
                        lineartransfereffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTableTransferEffect<Identity: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tabletransfereffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice3_Impl::CreateTableTransferEffect(this) {
                    Ok(ok__) => {
                        tabletransfereffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCompositeEffect<Identity: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, compositeeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice3_Impl::CreateCompositeEffect(this) {
                    Ok(ok__) => {
                        compositeeffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBlendEffect<Identity: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blendeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice3_Impl::CreateBlendEffect(this) {
                    Ok(ok__) => {
                        blendeffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateArithmeticCompositeEffect<Identity: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, arithmeticcompositeeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice3_Impl::CreateArithmeticCompositeEffect(this) {
                    Ok(ok__) => {
                        arithmeticcompositeeffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAffineTransform2DEffect<Identity: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, affinetransform2deffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice3_Impl::CreateAffineTransform2DEffect(this) {
                    Ok(ok__) => {
                        affinetransform2deffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDCompositionDevice2_Vtbl::new::<Identity, OFFSET>(),
            CreateGaussianBlurEffect: CreateGaussianBlurEffect::<Identity, OFFSET>,
            CreateBrightnessEffect: CreateBrightnessEffect::<Identity, OFFSET>,
            CreateColorMatrixEffect: CreateColorMatrixEffect::<Identity, OFFSET>,
            CreateShadowEffect: CreateShadowEffect::<Identity, OFFSET>,
            CreateHueRotationEffect: CreateHueRotationEffect::<Identity, OFFSET>,
            CreateSaturationEffect: CreateSaturationEffect::<Identity, OFFSET>,
            CreateTurbulenceEffect: CreateTurbulenceEffect::<Identity, OFFSET>,
            CreateLinearTransferEffect: CreateLinearTransferEffect::<Identity, OFFSET>,
            CreateTableTransferEffect: CreateTableTransferEffect::<Identity, OFFSET>,
            CreateCompositeEffect: CreateCompositeEffect::<Identity, OFFSET>,
            CreateBlendEffect: CreateBlendEffect::<Identity, OFFSET>,
            CreateArithmeticCompositeEffect: CreateArithmeticCompositeEffect::<Identity, OFFSET>,
            CreateAffineTransform2DEffect: CreateAffineTransform2DEffect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionDevice3 as windows_core::Interface>::IID || iid == &<IDCompositionDevice2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDCompositionDevice3 {}
windows_core::imp::define_interface!(IDCompositionDevice4, IDCompositionDevice4_Vtbl, 0x85fc5cca_2da6_494c_86b6_4a775c049b8a);
impl core::ops::Deref for IDCompositionDevice4 {
    type Target = IDCompositionDevice3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionDevice4, windows_core::IUnknown, IDCompositionDevice2, IDCompositionDevice3);
impl IDCompositionDevice4 {
    pub unsafe fn CheckCompositionTextureSupport<P0>(&self, renderingdevice: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckCompositionTextureSupport)(windows_core::Interface::as_raw(self), renderingdevice.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateCompositionTexture<P0>(&self, d3dtexture: P0) -> windows_core::Result<IDCompositionTexture>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCompositionTexture)(windows_core::Interface::as_raw(self), d3dtexture.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice4_Vtbl {
    pub base__: IDCompositionDevice3_Vtbl,
    pub CheckCompositionTextureSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CreateCompositionTexture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionDevice4_Impl: IDCompositionDevice3_Impl {
    fn CheckCompositionTextureSupport(&self, renderingdevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<windows_core::BOOL>;
    fn CreateCompositionTexture(&self, d3dtexture: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<IDCompositionTexture>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionDevice4_Vtbl {
    pub const fn new<Identity: IDCompositionDevice4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckCompositionTextureSupport<Identity: IDCompositionDevice4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingdevice: *mut core::ffi::c_void, supportscompositiontextures: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice4_Impl::CheckCompositionTextureSupport(this, core::mem::transmute_copy(&renderingdevice)) {
                    Ok(ok__) => {
                        supportscompositiontextures.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCompositionTexture<Identity: IDCompositionDevice4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, d3dtexture: *mut core::ffi::c_void, compositiontexture: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice4_Impl::CreateCompositionTexture(this, core::mem::transmute_copy(&d3dtexture)) {
                    Ok(ok__) => {
                        compositiontexture.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDCompositionDevice3_Vtbl::new::<Identity, OFFSET>(),
            CheckCompositionTextureSupport: CheckCompositionTextureSupport::<Identity, OFFSET>,
            CreateCompositionTexture: CreateCompositionTexture::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionDevice4 as windows_core::Interface>::IID || iid == &<IDCompositionDevice2 as windows_core::Interface>::IID || iid == &<IDCompositionDevice3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDCompositionDevice4 {}
windows_core::imp::define_interface!(IDCompositionDeviceDebug, IDCompositionDeviceDebug_Vtbl, 0xa1a3c64a_224f_4a81_9773_4f03a89d3c6c);
windows_core::imp::interface_hierarchy!(IDCompositionDeviceDebug, windows_core::IUnknown);
impl IDCompositionDeviceDebug {
    pub unsafe fn EnableDebugCounters(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableDebugCounters)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DisableDebugCounters(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisableDebugCounters)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDeviceDebug_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnableDebugCounters: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableDebugCounters: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDCompositionDeviceDebug_Impl: windows_core::IUnknownImpl {
    fn EnableDebugCounters(&self) -> windows_core::Result<()>;
    fn DisableDebugCounters(&self) -> windows_core::Result<()>;
}
impl IDCompositionDeviceDebug_Vtbl {
    pub const fn new<Identity: IDCompositionDeviceDebug_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnableDebugCounters<Identity: IDCompositionDeviceDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionDeviceDebug_Impl::EnableDebugCounters(this).into()
            }
        }
        unsafe extern "system" fn DisableDebugCounters<Identity: IDCompositionDeviceDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionDeviceDebug_Impl::DisableDebugCounters(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableDebugCounters: EnableDebugCounters::<Identity, OFFSET>,
            DisableDebugCounters: DisableDebugCounters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionDeviceDebug as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionDeviceDebug {}
windows_core::imp::define_interface!(IDCompositionEffect, IDCompositionEffect_Vtbl, 0xec81b08f_bfcb_4e8d_b193_a915587999e8);
windows_core::imp::interface_hierarchy!(IDCompositionEffect, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionEffect_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IDCompositionEffect_Impl: windows_core::IUnknownImpl {}
impl IDCompositionEffect_Vtbl {
    pub const fn new<Identity: IDCompositionEffect_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionEffect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionEffect {}
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
        unsafe { (windows_core::Interface::vtable(self).SetOpacity)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetOpacity2(&self, opacity: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOpacity2)(windows_core::Interface::as_raw(self), opacity).ok() }
    }
    pub unsafe fn SetTransform3D<P0>(&self, transform3d: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionTransform3D>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTransform3D)(windows_core::Interface::as_raw(self), transform3d.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionEffectGroup_Vtbl {
    pub base__: IDCompositionEffect_Vtbl,
    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOpacity2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetTransform3D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDCompositionEffectGroup_Impl: IDCompositionEffect_Impl {
    fn SetOpacity(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOpacity2(&self, opacity: f32) -> windows_core::Result<()>;
    fn SetTransform3D(&self, transform3d: windows_core::Ref<IDCompositionTransform3D>) -> windows_core::Result<()>;
}
impl IDCompositionEffectGroup_Vtbl {
    pub const fn new<Identity: IDCompositionEffectGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOpacity<Identity: IDCompositionEffectGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionEffectGroup_Impl::SetOpacity(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetOpacity2<Identity: IDCompositionEffectGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionEffectGroup_Impl::SetOpacity2(this, core::mem::transmute_copy(&opacity)).into()
            }
        }
        unsafe extern "system" fn SetTransform3D<Identity: IDCompositionEffectGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform3d: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionEffectGroup_Impl::SetTransform3D(this, core::mem::transmute_copy(&transform3d)).into()
            }
        }
        Self {
            base__: IDCompositionEffect_Vtbl::new::<Identity, OFFSET>(),
            SetOpacity: SetOpacity::<Identity, OFFSET>,
            SetOpacity2: SetOpacity2::<Identity, OFFSET>,
            SetTransform3D: SetTransform3D::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionEffectGroup as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionEffectGroup {}
windows_core::imp::define_interface!(IDCompositionFilterEffect, IDCompositionFilterEffect_Vtbl, 0x30c421d5_8cb2_4e9f_b133_37be270d4ac2);
impl core::ops::Deref for IDCompositionFilterEffect {
    type Target = IDCompositionEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionFilterEffect, windows_core::IUnknown, IDCompositionEffect);
impl IDCompositionFilterEffect {
    pub unsafe fn SetInput<P1>(&self, index: u32, input: P1, flags: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInput)(windows_core::Interface::as_raw(self), index, input.param().abi(), flags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionFilterEffect_Vtbl {
    pub base__: IDCompositionEffect_Vtbl,
    pub SetInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IDCompositionFilterEffect_Impl: IDCompositionEffect_Impl {
    fn SetInput(&self, index: u32, input: windows_core::Ref<windows_core::IUnknown>, flags: u32) -> windows_core::Result<()>;
}
impl IDCompositionFilterEffect_Vtbl {
    pub const fn new<Identity: IDCompositionFilterEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetInput<Identity: IDCompositionFilterEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, input: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionFilterEffect_Impl::SetInput(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&input), core::mem::transmute_copy(&flags)).into()
            }
        }
        Self { base__: IDCompositionEffect_Vtbl::new::<Identity, OFFSET>(), SetInput: SetInput::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionFilterEffect {}
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
        unsafe { (windows_core::Interface::vtable(self).SetStandardDeviation)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetStandardDeviation2(&self, amount: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStandardDeviation2)(windows_core::Interface::as_raw(self), amount).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBorderMode(&self, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBorderMode)(windows_core::Interface::as_raw(self), mode).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionGaussianBlurEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetStandardDeviation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStandardDeviation2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBorderMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::Direct2D::Common::D2D1_BORDER_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBorderMode: usize,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionGaussianBlurEffect_Impl: IDCompositionFilterEffect_Impl {
    fn SetStandardDeviation(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetStandardDeviation2(&self, amount: f32) -> windows_core::Result<()>;
    fn SetBorderMode(&self, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionGaussianBlurEffect_Vtbl {
    pub const fn new<Identity: IDCompositionGaussianBlurEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetStandardDeviation<Identity: IDCompositionGaussianBlurEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionGaussianBlurEffect_Impl::SetStandardDeviation(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetStandardDeviation2<Identity: IDCompositionGaussianBlurEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, amount: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionGaussianBlurEffect_Impl::SetStandardDeviation2(this, core::mem::transmute_copy(&amount)).into()
            }
        }
        unsafe extern "system" fn SetBorderMode<Identity: IDCompositionGaussianBlurEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionGaussianBlurEffect_Impl::SetBorderMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetStandardDeviation: SetStandardDeviation::<Identity, OFFSET>,
            SetStandardDeviation2: SetStandardDeviation2::<Identity, OFFSET>,
            SetBorderMode: SetBorderMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionGaussianBlurEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionGaussianBlurEffect {}
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
        unsafe { (windows_core::Interface::vtable(self).SetAngle)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetAngle2(&self, amountdegrees: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAngle2)(windows_core::Interface::as_raw(self), amountdegrees).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionHueRotationEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetAngle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
pub trait IDCompositionHueRotationEffect_Impl: IDCompositionFilterEffect_Impl {
    fn SetAngle(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAngle2(&self, amountdegrees: f32) -> windows_core::Result<()>;
}
impl IDCompositionHueRotationEffect_Vtbl {
    pub const fn new<Identity: IDCompositionHueRotationEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAngle<Identity: IDCompositionHueRotationEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionHueRotationEffect_Impl::SetAngle(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetAngle2<Identity: IDCompositionHueRotationEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, amountdegrees: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionHueRotationEffect_Impl::SetAngle2(this, core::mem::transmute_copy(&amountdegrees)).into()
            }
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetAngle: SetAngle::<Identity, OFFSET>,
            SetAngle2: SetAngle2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionHueRotationEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionHueRotationEffect {}
windows_core::imp::define_interface!(IDCompositionInkTrailDevice, IDCompositionInkTrailDevice_Vtbl, 0xdf0c7cec_cdeb_4d4a_b91c_721bf22f4e6c);
windows_core::imp::interface_hierarchy!(IDCompositionInkTrailDevice, windows_core::IUnknown);
impl IDCompositionInkTrailDevice {
    pub unsafe fn CreateDelegatedInkTrail(&self) -> windows_core::Result<IDCompositionDelegatedInkTrail> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDelegatedInkTrail)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDelegatedInkTrailForSwapChain<P0>(&self, swapchain: P0) -> windows_core::Result<IDCompositionDelegatedInkTrail>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDelegatedInkTrailForSwapChain)(windows_core::Interface::as_raw(self), swapchain.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionInkTrailDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateDelegatedInkTrail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDelegatedInkTrailForSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDCompositionInkTrailDevice_Impl: windows_core::IUnknownImpl {
    fn CreateDelegatedInkTrail(&self) -> windows_core::Result<IDCompositionDelegatedInkTrail>;
    fn CreateDelegatedInkTrailForSwapChain(&self, swapchain: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<IDCompositionDelegatedInkTrail>;
}
impl IDCompositionInkTrailDevice_Vtbl {
    pub const fn new<Identity: IDCompositionInkTrailDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDelegatedInkTrail<Identity: IDCompositionInkTrailDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inktrail: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionInkTrailDevice_Impl::CreateDelegatedInkTrail(this) {
                    Ok(ok__) => {
                        inktrail.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDelegatedInkTrailForSwapChain<Identity: IDCompositionInkTrailDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, swapchain: *mut core::ffi::c_void, inktrail: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionInkTrailDevice_Impl::CreateDelegatedInkTrailForSwapChain(this, core::mem::transmute_copy(&swapchain)) {
                    Ok(ok__) => {
                        inktrail.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDelegatedInkTrail: CreateDelegatedInkTrail::<Identity, OFFSET>,
            CreateDelegatedInkTrailForSwapChain: CreateDelegatedInkTrailForSwapChain::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionInkTrailDevice as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionInkTrailDevice {}
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
        unsafe { (windows_core::Interface::vtable(self).SetRedYIntercept)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetRedYIntercept2(&self, redyintercept: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRedYIntercept2)(windows_core::Interface::as_raw(self), redyintercept).ok() }
    }
    pub unsafe fn SetRedSlope<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRedSlope)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetRedSlope2(&self, redslope: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRedSlope2)(windows_core::Interface::as_raw(self), redslope).ok() }
    }
    pub unsafe fn SetRedDisable(&self, reddisable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRedDisable)(windows_core::Interface::as_raw(self), reddisable.into()).ok() }
    }
    pub unsafe fn SetGreenYIntercept<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetGreenYIntercept)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetGreenYIntercept2(&self, greenyintercept: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGreenYIntercept2)(windows_core::Interface::as_raw(self), greenyintercept).ok() }
    }
    pub unsafe fn SetGreenSlope<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetGreenSlope)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetGreenSlope2(&self, greenslope: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGreenSlope2)(windows_core::Interface::as_raw(self), greenslope).ok() }
    }
    pub unsafe fn SetGreenDisable(&self, greendisable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGreenDisable)(windows_core::Interface::as_raw(self), greendisable.into()).ok() }
    }
    pub unsafe fn SetBlueYIntercept<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBlueYIntercept)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetBlueYIntercept2(&self, blueyintercept: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBlueYIntercept2)(windows_core::Interface::as_raw(self), blueyintercept).ok() }
    }
    pub unsafe fn SetBlueSlope<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBlueSlope)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetBlueSlope2(&self, blueslope: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBlueSlope2)(windows_core::Interface::as_raw(self), blueslope).ok() }
    }
    pub unsafe fn SetBlueDisable(&self, bluedisable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBlueDisable)(windows_core::Interface::as_raw(self), bluedisable.into()).ok() }
    }
    pub unsafe fn SetAlphaYIntercept<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAlphaYIntercept)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetAlphaYIntercept2(&self, alphayintercept: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAlphaYIntercept2)(windows_core::Interface::as_raw(self), alphayintercept).ok() }
    }
    pub unsafe fn SetAlphaSlope<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAlphaSlope)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetAlphaSlope2(&self, alphaslope: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAlphaSlope2)(windows_core::Interface::as_raw(self), alphaslope).ok() }
    }
    pub unsafe fn SetAlphaDisable(&self, alphadisable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAlphaDisable)(windows_core::Interface::as_raw(self), alphadisable.into()).ok() }
    }
    pub unsafe fn SetClampOutput(&self, clampoutput: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClampOutput)(windows_core::Interface::as_raw(self), clampoutput.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionLinearTransferEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetRedYIntercept: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRedYIntercept2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetRedSlope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRedSlope2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetRedDisable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetGreenYIntercept: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGreenYIntercept2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetGreenSlope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGreenSlope2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetGreenDisable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetBlueYIntercept: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBlueYIntercept2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetBlueSlope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBlueSlope2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetBlueDisable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetAlphaYIntercept: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAlphaYIntercept2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetAlphaSlope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAlphaSlope2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetAlphaDisable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetClampOutput: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IDCompositionLinearTransferEffect_Impl: IDCompositionFilterEffect_Impl {
    fn SetRedYIntercept(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetRedYIntercept2(&self, redyintercept: f32) -> windows_core::Result<()>;
    fn SetRedSlope(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetRedSlope2(&self, redslope: f32) -> windows_core::Result<()>;
    fn SetRedDisable(&self, reddisable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetGreenYIntercept(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetGreenYIntercept2(&self, greenyintercept: f32) -> windows_core::Result<()>;
    fn SetGreenSlope(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetGreenSlope2(&self, greenslope: f32) -> windows_core::Result<()>;
    fn SetGreenDisable(&self, greendisable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetBlueYIntercept(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBlueYIntercept2(&self, blueyintercept: f32) -> windows_core::Result<()>;
    fn SetBlueSlope(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBlueSlope2(&self, blueslope: f32) -> windows_core::Result<()>;
    fn SetBlueDisable(&self, bluedisable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetAlphaYIntercept(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAlphaYIntercept2(&self, alphayintercept: f32) -> windows_core::Result<()>;
    fn SetAlphaSlope(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAlphaSlope2(&self, alphaslope: f32) -> windows_core::Result<()>;
    fn SetAlphaDisable(&self, alphadisable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetClampOutput(&self, clampoutput: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IDCompositionLinearTransferEffect_Vtbl {
    pub const fn new<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRedYIntercept<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetRedYIntercept(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetRedYIntercept2<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, redyintercept: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetRedYIntercept2(this, core::mem::transmute_copy(&redyintercept)).into()
            }
        }
        unsafe extern "system" fn SetRedSlope<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetRedSlope(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetRedSlope2<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, redslope: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetRedSlope2(this, core::mem::transmute_copy(&redslope)).into()
            }
        }
        unsafe extern "system" fn SetRedDisable<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reddisable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetRedDisable(this, core::mem::transmute_copy(&reddisable)).into()
            }
        }
        unsafe extern "system" fn SetGreenYIntercept<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetGreenYIntercept(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetGreenYIntercept2<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, greenyintercept: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetGreenYIntercept2(this, core::mem::transmute_copy(&greenyintercept)).into()
            }
        }
        unsafe extern "system" fn SetGreenSlope<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetGreenSlope(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetGreenSlope2<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, greenslope: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetGreenSlope2(this, core::mem::transmute_copy(&greenslope)).into()
            }
        }
        unsafe extern "system" fn SetGreenDisable<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, greendisable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetGreenDisable(this, core::mem::transmute_copy(&greendisable)).into()
            }
        }
        unsafe extern "system" fn SetBlueYIntercept<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetBlueYIntercept(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetBlueYIntercept2<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blueyintercept: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetBlueYIntercept2(this, core::mem::transmute_copy(&blueyintercept)).into()
            }
        }
        unsafe extern "system" fn SetBlueSlope<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetBlueSlope(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetBlueSlope2<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blueslope: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetBlueSlope2(this, core::mem::transmute_copy(&blueslope)).into()
            }
        }
        unsafe extern "system" fn SetBlueDisable<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bluedisable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetBlueDisable(this, core::mem::transmute_copy(&bluedisable)).into()
            }
        }
        unsafe extern "system" fn SetAlphaYIntercept<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetAlphaYIntercept(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetAlphaYIntercept2<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alphayintercept: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetAlphaYIntercept2(this, core::mem::transmute_copy(&alphayintercept)).into()
            }
        }
        unsafe extern "system" fn SetAlphaSlope<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetAlphaSlope(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetAlphaSlope2<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alphaslope: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetAlphaSlope2(this, core::mem::transmute_copy(&alphaslope)).into()
            }
        }
        unsafe extern "system" fn SetAlphaDisable<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alphadisable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetAlphaDisable(this, core::mem::transmute_copy(&alphadisable)).into()
            }
        }
        unsafe extern "system" fn SetClampOutput<Identity: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clampoutput: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionLinearTransferEffect_Impl::SetClampOutput(this, core::mem::transmute_copy(&clampoutput)).into()
            }
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetRedYIntercept: SetRedYIntercept::<Identity, OFFSET>,
            SetRedYIntercept2: SetRedYIntercept2::<Identity, OFFSET>,
            SetRedSlope: SetRedSlope::<Identity, OFFSET>,
            SetRedSlope2: SetRedSlope2::<Identity, OFFSET>,
            SetRedDisable: SetRedDisable::<Identity, OFFSET>,
            SetGreenYIntercept: SetGreenYIntercept::<Identity, OFFSET>,
            SetGreenYIntercept2: SetGreenYIntercept2::<Identity, OFFSET>,
            SetGreenSlope: SetGreenSlope::<Identity, OFFSET>,
            SetGreenSlope2: SetGreenSlope2::<Identity, OFFSET>,
            SetGreenDisable: SetGreenDisable::<Identity, OFFSET>,
            SetBlueYIntercept: SetBlueYIntercept::<Identity, OFFSET>,
            SetBlueYIntercept2: SetBlueYIntercept2::<Identity, OFFSET>,
            SetBlueSlope: SetBlueSlope::<Identity, OFFSET>,
            SetBlueSlope2: SetBlueSlope2::<Identity, OFFSET>,
            SetBlueDisable: SetBlueDisable::<Identity, OFFSET>,
            SetAlphaYIntercept: SetAlphaYIntercept::<Identity, OFFSET>,
            SetAlphaYIntercept2: SetAlphaYIntercept2::<Identity, OFFSET>,
            SetAlphaSlope: SetAlphaSlope::<Identity, OFFSET>,
            SetAlphaSlope2: SetAlphaSlope2::<Identity, OFFSET>,
            SetAlphaDisable: SetAlphaDisable::<Identity, OFFSET>,
            SetClampOutput: SetClampOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionLinearTransferEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionLinearTransferEffect {}
windows_core::imp::define_interface!(IDCompositionMatrixTransform, IDCompositionMatrixTransform_Vtbl, 0x16cdff07_c503_419c_83f2_0965c7af1fa6);
impl core::ops::Deref for IDCompositionMatrixTransform {
    type Target = IDCompositionTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionMatrixTransform, windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D, IDCompositionTransform);
impl IDCompositionMatrixTransform {
    pub unsafe fn SetMatrix(&self, matrix: *const windows_numerics::Matrix3x2) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMatrix)(windows_core::Interface::as_raw(self), matrix).ok() }
    }
    pub unsafe fn SetMatrixElement<P2>(&self, row: i32, column: i32, animation: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMatrixElement)(windows_core::Interface::as_raw(self), row, column, animation.param().abi()).ok() }
    }
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMatrixElement2)(windows_core::Interface::as_raw(self), row, column, value).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionMatrixTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Matrix3x2) -> windows_core::HRESULT,
    pub SetMatrixElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, f32) -> windows_core::HRESULT,
}
pub trait IDCompositionMatrixTransform_Impl: IDCompositionTransform_Impl {
    fn SetMatrix(&self, matrix: *const windows_numerics::Matrix3x2) -> windows_core::Result<()>;
    fn SetMatrixElement(&self, row: i32, column: i32, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> windows_core::Result<()>;
}
impl IDCompositionMatrixTransform_Vtbl {
    pub const fn new<Identity: IDCompositionMatrixTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMatrix<Identity: IDCompositionMatrixTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const windows_numerics::Matrix3x2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionMatrixTransform_Impl::SetMatrix(this, core::mem::transmute_copy(&matrix)).into()
            }
        }
        unsafe extern "system" fn SetMatrixElement<Identity: IDCompositionMatrixTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionMatrixTransform_Impl::SetMatrixElement(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column), core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetMatrixElement2<Identity: IDCompositionMatrixTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionMatrixTransform_Impl::SetMatrixElement2(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column), core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: IDCompositionTransform_Vtbl::new::<Identity, OFFSET>(),
            SetMatrix: SetMatrix::<Identity, OFFSET>,
            SetMatrixElement: SetMatrixElement::<Identity, OFFSET>,
            SetMatrixElement2: SetMatrixElement2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionMatrixTransform as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionTransform as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionMatrixTransform {}
windows_core::imp::define_interface!(IDCompositionMatrixTransform3D, IDCompositionMatrixTransform3D_Vtbl, 0x4b3363f0_643b_41b7_b6e0_ccf22d34467c);
impl core::ops::Deref for IDCompositionMatrixTransform3D {
    type Target = IDCompositionTransform3D;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionMatrixTransform3D, windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D);
impl IDCompositionMatrixTransform3D {
    pub unsafe fn SetMatrix(&self, matrix: *const windows_numerics::Matrix4x4) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMatrix)(windows_core::Interface::as_raw(self), matrix).ok() }
    }
    pub unsafe fn SetMatrixElement<P2>(&self, row: i32, column: i32, animation: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMatrixElement)(windows_core::Interface::as_raw(self), row, column, animation.param().abi()).ok() }
    }
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMatrixElement2)(windows_core::Interface::as_raw(self), row, column, value).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionMatrixTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    pub SetMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Matrix4x4) -> windows_core::HRESULT,
    pub SetMatrixElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, f32) -> windows_core::HRESULT,
}
pub trait IDCompositionMatrixTransform3D_Impl: IDCompositionTransform3D_Impl {
    fn SetMatrix(&self, matrix: *const windows_numerics::Matrix4x4) -> windows_core::Result<()>;
    fn SetMatrixElement(&self, row: i32, column: i32, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> windows_core::Result<()>;
}
impl IDCompositionMatrixTransform3D_Vtbl {
    pub const fn new<Identity: IDCompositionMatrixTransform3D_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMatrix<Identity: IDCompositionMatrixTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const windows_numerics::Matrix4x4) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionMatrixTransform3D_Impl::SetMatrix(this, core::mem::transmute_copy(&matrix)).into()
            }
        }
        unsafe extern "system" fn SetMatrixElement<Identity: IDCompositionMatrixTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionMatrixTransform3D_Impl::SetMatrixElement(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column), core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetMatrixElement2<Identity: IDCompositionMatrixTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionMatrixTransform3D_Impl::SetMatrixElement2(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column), core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: IDCompositionTransform3D_Vtbl::new::<Identity, OFFSET>(),
            SetMatrix: SetMatrix::<Identity, OFFSET>,
            SetMatrixElement: SetMatrixElement::<Identity, OFFSET>,
            SetMatrixElement2: SetMatrixElement2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionMatrixTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionMatrixTransform3D {}
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
        unsafe { (windows_core::Interface::vtable(self).SetLeft)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetLeft2(&self, left: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLeft2)(windows_core::Interface::as_raw(self), left).ok() }
    }
    pub unsafe fn SetTop<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTop)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetTop2(&self, top: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTop2)(windows_core::Interface::as_raw(self), top).ok() }
    }
    pub unsafe fn SetRight<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRight)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetRight2(&self, right: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRight2)(windows_core::Interface::as_raw(self), right).ok() }
    }
    pub unsafe fn SetBottom<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBottom)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetBottom2(&self, bottom: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBottom2)(windows_core::Interface::as_raw(self), bottom).ok() }
    }
    pub unsafe fn SetTopLeftRadiusX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTopLeftRadiusX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetTopLeftRadiusX2(&self, radius: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTopLeftRadiusX2)(windows_core::Interface::as_raw(self), radius).ok() }
    }
    pub unsafe fn SetTopLeftRadiusY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTopLeftRadiusY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetTopLeftRadiusY2(&self, radius: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTopLeftRadiusY2)(windows_core::Interface::as_raw(self), radius).ok() }
    }
    pub unsafe fn SetTopRightRadiusX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTopRightRadiusX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetTopRightRadiusX2(&self, radius: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTopRightRadiusX2)(windows_core::Interface::as_raw(self), radius).ok() }
    }
    pub unsafe fn SetTopRightRadiusY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTopRightRadiusY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetTopRightRadiusY2(&self, radius: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTopRightRadiusY2)(windows_core::Interface::as_raw(self), radius).ok() }
    }
    pub unsafe fn SetBottomLeftRadiusX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBottomLeftRadiusX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetBottomLeftRadiusX2(&self, radius: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBottomLeftRadiusX2)(windows_core::Interface::as_raw(self), radius).ok() }
    }
    pub unsafe fn SetBottomLeftRadiusY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBottomLeftRadiusY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetBottomLeftRadiusY2(&self, radius: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBottomLeftRadiusY2)(windows_core::Interface::as_raw(self), radius).ok() }
    }
    pub unsafe fn SetBottomRightRadiusX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBottomRightRadiusX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetBottomRightRadiusX2(&self, radius: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBottomRightRadiusX2)(windows_core::Interface::as_raw(self), radius).ok() }
    }
    pub unsafe fn SetBottomRightRadiusY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBottomRightRadiusY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetBottomRightRadiusY2(&self, radius: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBottomRightRadiusY2)(windows_core::Interface::as_raw(self), radius).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IDCompositionRectangleClip_Impl: IDCompositionClip_Impl {
    fn SetLeft(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetLeft2(&self, left: f32) -> windows_core::Result<()>;
    fn SetTop(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetTop2(&self, top: f32) -> windows_core::Result<()>;
    fn SetRight(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetRight2(&self, right: f32) -> windows_core::Result<()>;
    fn SetBottom(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBottom2(&self, bottom: f32) -> windows_core::Result<()>;
    fn SetTopLeftRadiusX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetTopLeftRadiusX2(&self, radius: f32) -> windows_core::Result<()>;
    fn SetTopLeftRadiusY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetTopLeftRadiusY2(&self, radius: f32) -> windows_core::Result<()>;
    fn SetTopRightRadiusX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetTopRightRadiusX2(&self, radius: f32) -> windows_core::Result<()>;
    fn SetTopRightRadiusY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetTopRightRadiusY2(&self, radius: f32) -> windows_core::Result<()>;
    fn SetBottomLeftRadiusX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBottomLeftRadiusX2(&self, radius: f32) -> windows_core::Result<()>;
    fn SetBottomLeftRadiusY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBottomLeftRadiusY2(&self, radius: f32) -> windows_core::Result<()>;
    fn SetBottomRightRadiusX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBottomRightRadiusX2(&self, radius: f32) -> windows_core::Result<()>;
    fn SetBottomRightRadiusY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBottomRightRadiusY2(&self, radius: f32) -> windows_core::Result<()>;
}
impl IDCompositionRectangleClip_Vtbl {
    pub const fn new<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetLeft<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetLeft(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetLeft2<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetLeft2(this, core::mem::transmute_copy(&left)).into()
            }
        }
        unsafe extern "system" fn SetTop<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetTop(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetTop2<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetTop2(this, core::mem::transmute_copy(&top)).into()
            }
        }
        unsafe extern "system" fn SetRight<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetRight(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetRight2<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, right: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetRight2(this, core::mem::transmute_copy(&right)).into()
            }
        }
        unsafe extern "system" fn SetBottom<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetBottom(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetBottom2<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bottom: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetBottom2(this, core::mem::transmute_copy(&bottom)).into()
            }
        }
        unsafe extern "system" fn SetTopLeftRadiusX<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetTopLeftRadiusX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetTopLeftRadiusX2<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetTopLeftRadiusX2(this, core::mem::transmute_copy(&radius)).into()
            }
        }
        unsafe extern "system" fn SetTopLeftRadiusY<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetTopLeftRadiusY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetTopLeftRadiusY2<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetTopLeftRadiusY2(this, core::mem::transmute_copy(&radius)).into()
            }
        }
        unsafe extern "system" fn SetTopRightRadiusX<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetTopRightRadiusX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetTopRightRadiusX2<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetTopRightRadiusX2(this, core::mem::transmute_copy(&radius)).into()
            }
        }
        unsafe extern "system" fn SetTopRightRadiusY<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetTopRightRadiusY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetTopRightRadiusY2<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetTopRightRadiusY2(this, core::mem::transmute_copy(&radius)).into()
            }
        }
        unsafe extern "system" fn SetBottomLeftRadiusX<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetBottomLeftRadiusX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetBottomLeftRadiusX2<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetBottomLeftRadiusX2(this, core::mem::transmute_copy(&radius)).into()
            }
        }
        unsafe extern "system" fn SetBottomLeftRadiusY<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetBottomLeftRadiusY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetBottomLeftRadiusY2<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetBottomLeftRadiusY2(this, core::mem::transmute_copy(&radius)).into()
            }
        }
        unsafe extern "system" fn SetBottomRightRadiusX<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetBottomRightRadiusX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetBottomRightRadiusX2<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetBottomRightRadiusX2(this, core::mem::transmute_copy(&radius)).into()
            }
        }
        unsafe extern "system" fn SetBottomRightRadiusY<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetBottomRightRadiusY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetBottomRightRadiusY2<Identity: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRectangleClip_Impl::SetBottomRightRadiusY2(this, core::mem::transmute_copy(&radius)).into()
            }
        }
        Self {
            base__: IDCompositionClip_Vtbl::new::<Identity, OFFSET>(),
            SetLeft: SetLeft::<Identity, OFFSET>,
            SetLeft2: SetLeft2::<Identity, OFFSET>,
            SetTop: SetTop::<Identity, OFFSET>,
            SetTop2: SetTop2::<Identity, OFFSET>,
            SetRight: SetRight::<Identity, OFFSET>,
            SetRight2: SetRight2::<Identity, OFFSET>,
            SetBottom: SetBottom::<Identity, OFFSET>,
            SetBottom2: SetBottom2::<Identity, OFFSET>,
            SetTopLeftRadiusX: SetTopLeftRadiusX::<Identity, OFFSET>,
            SetTopLeftRadiusX2: SetTopLeftRadiusX2::<Identity, OFFSET>,
            SetTopLeftRadiusY: SetTopLeftRadiusY::<Identity, OFFSET>,
            SetTopLeftRadiusY2: SetTopLeftRadiusY2::<Identity, OFFSET>,
            SetTopRightRadiusX: SetTopRightRadiusX::<Identity, OFFSET>,
            SetTopRightRadiusX2: SetTopRightRadiusX2::<Identity, OFFSET>,
            SetTopRightRadiusY: SetTopRightRadiusY::<Identity, OFFSET>,
            SetTopRightRadiusY2: SetTopRightRadiusY2::<Identity, OFFSET>,
            SetBottomLeftRadiusX: SetBottomLeftRadiusX::<Identity, OFFSET>,
            SetBottomLeftRadiusX2: SetBottomLeftRadiusX2::<Identity, OFFSET>,
            SetBottomLeftRadiusY: SetBottomLeftRadiusY::<Identity, OFFSET>,
            SetBottomLeftRadiusY2: SetBottomLeftRadiusY2::<Identity, OFFSET>,
            SetBottomRightRadiusX: SetBottomRightRadiusX::<Identity, OFFSET>,
            SetBottomRightRadiusX2: SetBottomRightRadiusX2::<Identity, OFFSET>,
            SetBottomRightRadiusY: SetBottomRightRadiusY::<Identity, OFFSET>,
            SetBottomRightRadiusY2: SetBottomRightRadiusY2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionRectangleClip as windows_core::Interface>::IID || iid == &<IDCompositionClip as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionRectangleClip {}
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
        unsafe { (windows_core::Interface::vtable(self).SetAngle)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetAngle2(&self, angle: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAngle2)(windows_core::Interface::as_raw(self), angle).ok() }
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCenterX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCenterX2)(windows_core::Interface::as_raw(self), centerx).ok() }
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCenterY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCenterY2)(windows_core::Interface::as_raw(self), centery).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRotateTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetAngle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
pub trait IDCompositionRotateTransform_Impl: IDCompositionTransform_Impl {
    fn SetAngle(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAngle2(&self, angle: f32) -> windows_core::Result<()>;
    fn SetCenterX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()>;
    fn SetCenterY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()>;
}
impl IDCompositionRotateTransform_Vtbl {
    pub const fn new<Identity: IDCompositionRotateTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAngle<Identity: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform_Impl::SetAngle(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetAngle2<Identity: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, angle: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform_Impl::SetAngle2(this, core::mem::transmute_copy(&angle)).into()
            }
        }
        unsafe extern "system" fn SetCenterX<Identity: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform_Impl::SetCenterX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetCenterX2<Identity: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, centerx: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform_Impl::SetCenterX2(this, core::mem::transmute_copy(&centerx)).into()
            }
        }
        unsafe extern "system" fn SetCenterY<Identity: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform_Impl::SetCenterY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetCenterY2<Identity: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, centery: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform_Impl::SetCenterY2(this, core::mem::transmute_copy(&centery)).into()
            }
        }
        Self {
            base__: IDCompositionTransform_Vtbl::new::<Identity, OFFSET>(),
            SetAngle: SetAngle::<Identity, OFFSET>,
            SetAngle2: SetAngle2::<Identity, OFFSET>,
            SetCenterX: SetCenterX::<Identity, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, OFFSET>,
            SetCenterY: SetCenterY::<Identity, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionRotateTransform as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionTransform as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionRotateTransform {}
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
        unsafe { (windows_core::Interface::vtable(self).SetAngle)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetAngle2(&self, angle: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAngle2)(windows_core::Interface::as_raw(self), angle).ok() }
    }
    pub unsafe fn SetAxisX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAxisX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetAxisX2(&self, axisx: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAxisX2)(windows_core::Interface::as_raw(self), axisx).ok() }
    }
    pub unsafe fn SetAxisY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAxisY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetAxisY2(&self, axisy: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAxisY2)(windows_core::Interface::as_raw(self), axisy).ok() }
    }
    pub unsafe fn SetAxisZ<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAxisZ)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetAxisZ2(&self, axisz: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAxisZ2)(windows_core::Interface::as_raw(self), axisz).ok() }
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCenterX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCenterX2)(windows_core::Interface::as_raw(self), centerx).ok() }
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCenterY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCenterY2)(windows_core::Interface::as_raw(self), centery).ok() }
    }
    pub unsafe fn SetCenterZ<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCenterZ)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetCenterZ2(&self, centerz: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCenterZ2)(windows_core::Interface::as_raw(self), centerz).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IDCompositionRotateTransform3D_Impl: IDCompositionTransform3D_Impl {
    fn SetAngle(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAngle2(&self, angle: f32) -> windows_core::Result<()>;
    fn SetAxisX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAxisX2(&self, axisx: f32) -> windows_core::Result<()>;
    fn SetAxisY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAxisY2(&self, axisy: f32) -> windows_core::Result<()>;
    fn SetAxisZ(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAxisZ2(&self, axisz: f32) -> windows_core::Result<()>;
    fn SetCenterX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()>;
    fn SetCenterY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()>;
    fn SetCenterZ(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterZ2(&self, centerz: f32) -> windows_core::Result<()>;
}
impl IDCompositionRotateTransform3D_Vtbl {
    pub const fn new<Identity: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAngle<Identity: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform3D_Impl::SetAngle(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetAngle2<Identity: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, angle: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform3D_Impl::SetAngle2(this, core::mem::transmute_copy(&angle)).into()
            }
        }
        unsafe extern "system" fn SetAxisX<Identity: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform3D_Impl::SetAxisX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetAxisX2<Identity: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, axisx: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform3D_Impl::SetAxisX2(this, core::mem::transmute_copy(&axisx)).into()
            }
        }
        unsafe extern "system" fn SetAxisY<Identity: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform3D_Impl::SetAxisY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetAxisY2<Identity: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, axisy: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform3D_Impl::SetAxisY2(this, core::mem::transmute_copy(&axisy)).into()
            }
        }
        unsafe extern "system" fn SetAxisZ<Identity: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform3D_Impl::SetAxisZ(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetAxisZ2<Identity: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, axisz: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform3D_Impl::SetAxisZ2(this, core::mem::transmute_copy(&axisz)).into()
            }
        }
        unsafe extern "system" fn SetCenterX<Identity: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform3D_Impl::SetCenterX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetCenterX2<Identity: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, centerx: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform3D_Impl::SetCenterX2(this, core::mem::transmute_copy(&centerx)).into()
            }
        }
        unsafe extern "system" fn SetCenterY<Identity: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform3D_Impl::SetCenterY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetCenterY2<Identity: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, centery: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform3D_Impl::SetCenterY2(this, core::mem::transmute_copy(&centery)).into()
            }
        }
        unsafe extern "system" fn SetCenterZ<Identity: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform3D_Impl::SetCenterZ(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetCenterZ2<Identity: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, centerz: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionRotateTransform3D_Impl::SetCenterZ2(this, core::mem::transmute_copy(&centerz)).into()
            }
        }
        Self {
            base__: IDCompositionTransform3D_Vtbl::new::<Identity, OFFSET>(),
            SetAngle: SetAngle::<Identity, OFFSET>,
            SetAngle2: SetAngle2::<Identity, OFFSET>,
            SetAxisX: SetAxisX::<Identity, OFFSET>,
            SetAxisX2: SetAxisX2::<Identity, OFFSET>,
            SetAxisY: SetAxisY::<Identity, OFFSET>,
            SetAxisY2: SetAxisY2::<Identity, OFFSET>,
            SetAxisZ: SetAxisZ::<Identity, OFFSET>,
            SetAxisZ2: SetAxisZ2::<Identity, OFFSET>,
            SetCenterX: SetCenterX::<Identity, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, OFFSET>,
            SetCenterY: SetCenterY::<Identity, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, OFFSET>,
            SetCenterZ: SetCenterZ::<Identity, OFFSET>,
            SetCenterZ2: SetCenterZ2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionRotateTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionRotateTransform3D {}
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
        unsafe { (windows_core::Interface::vtable(self).SetSaturation)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetSaturation2(&self, ratio: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSaturation2)(windows_core::Interface::as_raw(self), ratio).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSaturationEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetSaturation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSaturation2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
pub trait IDCompositionSaturationEffect_Impl: IDCompositionFilterEffect_Impl {
    fn SetSaturation(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetSaturation2(&self, ratio: f32) -> windows_core::Result<()>;
}
impl IDCompositionSaturationEffect_Vtbl {
    pub const fn new<Identity: IDCompositionSaturationEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSaturation<Identity: IDCompositionSaturationEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionSaturationEffect_Impl::SetSaturation(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetSaturation2<Identity: IDCompositionSaturationEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ratio: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionSaturationEffect_Impl::SetSaturation2(this, core::mem::transmute_copy(&ratio)).into()
            }
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetSaturation: SetSaturation::<Identity, OFFSET>,
            SetSaturation2: SetSaturation2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionSaturationEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionSaturationEffect {}
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
        unsafe { (windows_core::Interface::vtable(self).SetScaleX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetScaleX2(&self, scalex: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScaleX2)(windows_core::Interface::as_raw(self), scalex).ok() }
    }
    pub unsafe fn SetScaleY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetScaleY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetScaleY2(&self, scaley: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScaleY2)(windows_core::Interface::as_raw(self), scaley).ok() }
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCenterX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCenterX2)(windows_core::Interface::as_raw(self), centerx).ok() }
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCenterY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCenterY2)(windows_core::Interface::as_raw(self), centery).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IDCompositionScaleTransform_Impl: IDCompositionTransform_Impl {
    fn SetScaleX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetScaleX2(&self, scalex: f32) -> windows_core::Result<()>;
    fn SetScaleY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetScaleY2(&self, scaley: f32) -> windows_core::Result<()>;
    fn SetCenterX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()>;
    fn SetCenterY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()>;
}
impl IDCompositionScaleTransform_Vtbl {
    pub const fn new<Identity: IDCompositionScaleTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetScaleX<Identity: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform_Impl::SetScaleX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetScaleX2<Identity: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scalex: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform_Impl::SetScaleX2(this, core::mem::transmute_copy(&scalex)).into()
            }
        }
        unsafe extern "system" fn SetScaleY<Identity: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform_Impl::SetScaleY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetScaleY2<Identity: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scaley: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform_Impl::SetScaleY2(this, core::mem::transmute_copy(&scaley)).into()
            }
        }
        unsafe extern "system" fn SetCenterX<Identity: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform_Impl::SetCenterX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetCenterX2<Identity: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, centerx: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform_Impl::SetCenterX2(this, core::mem::transmute_copy(&centerx)).into()
            }
        }
        unsafe extern "system" fn SetCenterY<Identity: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform_Impl::SetCenterY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetCenterY2<Identity: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, centery: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform_Impl::SetCenterY2(this, core::mem::transmute_copy(&centery)).into()
            }
        }
        Self {
            base__: IDCompositionTransform_Vtbl::new::<Identity, OFFSET>(),
            SetScaleX: SetScaleX::<Identity, OFFSET>,
            SetScaleX2: SetScaleX2::<Identity, OFFSET>,
            SetScaleY: SetScaleY::<Identity, OFFSET>,
            SetScaleY2: SetScaleY2::<Identity, OFFSET>,
            SetCenterX: SetCenterX::<Identity, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, OFFSET>,
            SetCenterY: SetCenterY::<Identity, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionScaleTransform as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionTransform as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionScaleTransform {}
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
        unsafe { (windows_core::Interface::vtable(self).SetScaleX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetScaleX2(&self, scalex: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScaleX2)(windows_core::Interface::as_raw(self), scalex).ok() }
    }
    pub unsafe fn SetScaleY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetScaleY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetScaleY2(&self, scaley: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScaleY2)(windows_core::Interface::as_raw(self), scaley).ok() }
    }
    pub unsafe fn SetScaleZ<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetScaleZ)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetScaleZ2(&self, scalez: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScaleZ2)(windows_core::Interface::as_raw(self), scalez).ok() }
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCenterX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCenterX2)(windows_core::Interface::as_raw(self), centerx).ok() }
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCenterY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCenterY2)(windows_core::Interface::as_raw(self), centery).ok() }
    }
    pub unsafe fn SetCenterZ<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCenterZ)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetCenterZ2(&self, centerz: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCenterZ2)(windows_core::Interface::as_raw(self), centerz).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IDCompositionScaleTransform3D_Impl: IDCompositionTransform3D_Impl {
    fn SetScaleX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetScaleX2(&self, scalex: f32) -> windows_core::Result<()>;
    fn SetScaleY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetScaleY2(&self, scaley: f32) -> windows_core::Result<()>;
    fn SetScaleZ(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetScaleZ2(&self, scalez: f32) -> windows_core::Result<()>;
    fn SetCenterX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()>;
    fn SetCenterY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()>;
    fn SetCenterZ(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterZ2(&self, centerz: f32) -> windows_core::Result<()>;
}
impl IDCompositionScaleTransform3D_Vtbl {
    pub const fn new<Identity: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetScaleX<Identity: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform3D_Impl::SetScaleX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetScaleX2<Identity: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scalex: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform3D_Impl::SetScaleX2(this, core::mem::transmute_copy(&scalex)).into()
            }
        }
        unsafe extern "system" fn SetScaleY<Identity: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform3D_Impl::SetScaleY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetScaleY2<Identity: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scaley: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform3D_Impl::SetScaleY2(this, core::mem::transmute_copy(&scaley)).into()
            }
        }
        unsafe extern "system" fn SetScaleZ<Identity: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform3D_Impl::SetScaleZ(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetScaleZ2<Identity: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scalez: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform3D_Impl::SetScaleZ2(this, core::mem::transmute_copy(&scalez)).into()
            }
        }
        unsafe extern "system" fn SetCenterX<Identity: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform3D_Impl::SetCenterX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetCenterX2<Identity: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, centerx: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform3D_Impl::SetCenterX2(this, core::mem::transmute_copy(&centerx)).into()
            }
        }
        unsafe extern "system" fn SetCenterY<Identity: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform3D_Impl::SetCenterY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetCenterY2<Identity: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, centery: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform3D_Impl::SetCenterY2(this, core::mem::transmute_copy(&centery)).into()
            }
        }
        unsafe extern "system" fn SetCenterZ<Identity: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform3D_Impl::SetCenterZ(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetCenterZ2<Identity: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, centerz: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionScaleTransform3D_Impl::SetCenterZ2(this, core::mem::transmute_copy(&centerz)).into()
            }
        }
        Self {
            base__: IDCompositionTransform3D_Vtbl::new::<Identity, OFFSET>(),
            SetScaleX: SetScaleX::<Identity, OFFSET>,
            SetScaleX2: SetScaleX2::<Identity, OFFSET>,
            SetScaleY: SetScaleY::<Identity, OFFSET>,
            SetScaleY2: SetScaleY2::<Identity, OFFSET>,
            SetScaleZ: SetScaleZ::<Identity, OFFSET>,
            SetScaleZ2: SetScaleZ2::<Identity, OFFSET>,
            SetCenterX: SetCenterX::<Identity, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, OFFSET>,
            SetCenterY: SetCenterY::<Identity, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, OFFSET>,
            SetCenterZ: SetCenterZ::<Identity, OFFSET>,
            SetCenterZ2: SetCenterZ2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionScaleTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionScaleTransform3D {}
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
        unsafe { (windows_core::Interface::vtable(self).SetStandardDeviation)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetStandardDeviation2(&self, amount: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStandardDeviation2)(windows_core::Interface::as_raw(self), amount).ok() }
    }
    pub unsafe fn SetColor(&self, color: *const windows_numerics::Vector4) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetColor)(windows_core::Interface::as_raw(self), color).ok() }
    }
    pub unsafe fn SetRed<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRed)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetRed2(&self, amount: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRed2)(windows_core::Interface::as_raw(self), amount).ok() }
    }
    pub unsafe fn SetGreen<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetGreen)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetGreen2(&self, amount: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGreen2)(windows_core::Interface::as_raw(self), amount).ok() }
    }
    pub unsafe fn SetBlue<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBlue)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetBlue2(&self, amount: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBlue2)(windows_core::Interface::as_raw(self), amount).ok() }
    }
    pub unsafe fn SetAlpha<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAlpha)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetAlpha2(&self, amount: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAlpha2)(windows_core::Interface::as_raw(self), amount).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionShadowEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetStandardDeviation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStandardDeviation2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Vector4) -> windows_core::HRESULT,
    pub SetRed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRed2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGreen2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBlue2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetAlpha: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAlpha2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
pub trait IDCompositionShadowEffect_Impl: IDCompositionFilterEffect_Impl {
    fn SetStandardDeviation(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetStandardDeviation2(&self, amount: f32) -> windows_core::Result<()>;
    fn SetColor(&self, color: *const windows_numerics::Vector4) -> windows_core::Result<()>;
    fn SetRed(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetRed2(&self, amount: f32) -> windows_core::Result<()>;
    fn SetGreen(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetGreen2(&self, amount: f32) -> windows_core::Result<()>;
    fn SetBlue(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBlue2(&self, amount: f32) -> windows_core::Result<()>;
    fn SetAlpha(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAlpha2(&self, amount: f32) -> windows_core::Result<()>;
}
impl IDCompositionShadowEffect_Vtbl {
    pub const fn new<Identity: IDCompositionShadowEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetStandardDeviation<Identity: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionShadowEffect_Impl::SetStandardDeviation(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetStandardDeviation2<Identity: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, amount: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionShadowEffect_Impl::SetStandardDeviation2(this, core::mem::transmute_copy(&amount)).into()
            }
        }
        unsafe extern "system" fn SetColor<Identity: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const windows_numerics::Vector4) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionShadowEffect_Impl::SetColor(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn SetRed<Identity: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionShadowEffect_Impl::SetRed(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetRed2<Identity: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, amount: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionShadowEffect_Impl::SetRed2(this, core::mem::transmute_copy(&amount)).into()
            }
        }
        unsafe extern "system" fn SetGreen<Identity: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionShadowEffect_Impl::SetGreen(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetGreen2<Identity: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, amount: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionShadowEffect_Impl::SetGreen2(this, core::mem::transmute_copy(&amount)).into()
            }
        }
        unsafe extern "system" fn SetBlue<Identity: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionShadowEffect_Impl::SetBlue(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetBlue2<Identity: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, amount: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionShadowEffect_Impl::SetBlue2(this, core::mem::transmute_copy(&amount)).into()
            }
        }
        unsafe extern "system" fn SetAlpha<Identity: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionShadowEffect_Impl::SetAlpha(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetAlpha2<Identity: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, amount: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionShadowEffect_Impl::SetAlpha2(this, core::mem::transmute_copy(&amount)).into()
            }
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetStandardDeviation: SetStandardDeviation::<Identity, OFFSET>,
            SetStandardDeviation2: SetStandardDeviation2::<Identity, OFFSET>,
            SetColor: SetColor::<Identity, OFFSET>,
            SetRed: SetRed::<Identity, OFFSET>,
            SetRed2: SetRed2::<Identity, OFFSET>,
            SetGreen: SetGreen::<Identity, OFFSET>,
            SetGreen2: SetGreen2::<Identity, OFFSET>,
            SetBlue: SetBlue::<Identity, OFFSET>,
            SetBlue2: SetBlue2::<Identity, OFFSET>,
            SetAlpha: SetAlpha::<Identity, OFFSET>,
            SetAlpha2: SetAlpha2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionShadowEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionShadowEffect {}
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
        unsafe { (windows_core::Interface::vtable(self).SetAngleX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetAngleX2(&self, anglex: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAngleX2)(windows_core::Interface::as_raw(self), anglex).ok() }
    }
    pub unsafe fn SetAngleY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAngleY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetAngleY2(&self, angley: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAngleY2)(windows_core::Interface::as_raw(self), angley).ok() }
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCenterX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCenterX2)(windows_core::Interface::as_raw(self), centerx).ok() }
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCenterY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCenterY2)(windows_core::Interface::as_raw(self), centery).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IDCompositionSkewTransform_Impl: IDCompositionTransform_Impl {
    fn SetAngleX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAngleX2(&self, anglex: f32) -> windows_core::Result<()>;
    fn SetAngleY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAngleY2(&self, angley: f32) -> windows_core::Result<()>;
    fn SetCenterX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()>;
    fn SetCenterY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()>;
}
impl IDCompositionSkewTransform_Vtbl {
    pub const fn new<Identity: IDCompositionSkewTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAngleX<Identity: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionSkewTransform_Impl::SetAngleX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetAngleX2<Identity: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, anglex: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionSkewTransform_Impl::SetAngleX2(this, core::mem::transmute_copy(&anglex)).into()
            }
        }
        unsafe extern "system" fn SetAngleY<Identity: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionSkewTransform_Impl::SetAngleY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetAngleY2<Identity: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, angley: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionSkewTransform_Impl::SetAngleY2(this, core::mem::transmute_copy(&angley)).into()
            }
        }
        unsafe extern "system" fn SetCenterX<Identity: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionSkewTransform_Impl::SetCenterX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetCenterX2<Identity: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, centerx: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionSkewTransform_Impl::SetCenterX2(this, core::mem::transmute_copy(&centerx)).into()
            }
        }
        unsafe extern "system" fn SetCenterY<Identity: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionSkewTransform_Impl::SetCenterY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetCenterY2<Identity: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, centery: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionSkewTransform_Impl::SetCenterY2(this, core::mem::transmute_copy(&centery)).into()
            }
        }
        Self {
            base__: IDCompositionTransform_Vtbl::new::<Identity, OFFSET>(),
            SetAngleX: SetAngleX::<Identity, OFFSET>,
            SetAngleX2: SetAngleX2::<Identity, OFFSET>,
            SetAngleY: SetAngleY::<Identity, OFFSET>,
            SetAngleY2: SetAngleY2::<Identity, OFFSET>,
            SetCenterX: SetCenterX::<Identity, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, OFFSET>,
            SetCenterY: SetCenterY::<Identity, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionSkewTransform as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionTransform as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionSkewTransform {}
windows_core::imp::define_interface!(IDCompositionSurface, IDCompositionSurface_Vtbl, 0xbb8a4953_2c99_4f5a_96f5_4819027fa3ac);
windows_core::imp::interface_hierarchy!(IDCompositionSurface, windows_core::IUnknown);
impl IDCompositionSurface {
    pub unsafe fn BeginDraw<T>(&self, updaterect: Option<*const super::super::Foundation::RECT>, updateoffset: *mut super::super::Foundation::POINT) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).BeginDraw)(windows_core::Interface::as_raw(self), updaterect.unwrap_or(core::mem::zeroed()) as _, &T::IID, &mut result__, updateoffset as _).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn EndDraw(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndDraw)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SuspendDraw(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SuspendDraw)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ResumeDraw(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResumeDraw)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Scroll(&self, scrollrect: Option<*const super::super::Foundation::RECT>, cliprect: Option<*const super::super::Foundation::RECT>, offsetx: i32, offsety: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Scroll)(windows_core::Interface::as_raw(self), scrollrect.unwrap_or(core::mem::zeroed()) as _, cliprect.unwrap_or(core::mem::zeroed()) as _, offsetx, offsety).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSurface_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginDraw: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut super::super::Foundation::POINT) -> windows_core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResumeDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Scroll: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, *const super::super::Foundation::RECT, i32, i32) -> windows_core::HRESULT,
}
pub trait IDCompositionSurface_Impl: windows_core::IUnknownImpl {
    fn BeginDraw(&self, updaterect: *const super::super::Foundation::RECT, iid: *const windows_core::GUID, updateobject: *mut *mut core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> windows_core::Result<()>;
    fn EndDraw(&self) -> windows_core::Result<()>;
    fn SuspendDraw(&self) -> windows_core::Result<()>;
    fn ResumeDraw(&self) -> windows_core::Result<()>;
    fn Scroll(&self, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> windows_core::Result<()>;
}
impl IDCompositionSurface_Vtbl {
    pub const fn new<Identity: IDCompositionSurface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginDraw<Identity: IDCompositionSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updaterect: *const super::super::Foundation::RECT, iid: *const windows_core::GUID, updateobject: *mut *mut core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionSurface_Impl::BeginDraw(this, core::mem::transmute_copy(&updaterect), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&updateobject), core::mem::transmute_copy(&updateoffset)).into()
            }
        }
        unsafe extern "system" fn EndDraw<Identity: IDCompositionSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionSurface_Impl::EndDraw(this).into()
            }
        }
        unsafe extern "system" fn SuspendDraw<Identity: IDCompositionSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionSurface_Impl::SuspendDraw(this).into()
            }
        }
        unsafe extern "system" fn ResumeDraw<Identity: IDCompositionSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionSurface_Impl::ResumeDraw(this).into()
            }
        }
        unsafe extern "system" fn Scroll<Identity: IDCompositionSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionSurface_Impl::Scroll(this, core::mem::transmute_copy(&scrollrect), core::mem::transmute_copy(&cliprect), core::mem::transmute_copy(&offsetx), core::mem::transmute_copy(&offsety)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginDraw: BeginDraw::<Identity, OFFSET>,
            EndDraw: EndDraw::<Identity, OFFSET>,
            SuspendDraw: SuspendDraw::<Identity, OFFSET>,
            ResumeDraw: ResumeDraw::<Identity, OFFSET>,
            Scroll: Scroll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionSurface as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionSurface {}
windows_core::imp::define_interface!(IDCompositionSurfaceFactory, IDCompositionSurfaceFactory_Vtbl, 0xe334bc12_3937_4e02_85eb_fcf4eb30d2c8);
windows_core::imp::interface_hierarchy!(IDCompositionSurfaceFactory, windows_core::IUnknown);
impl IDCompositionSurfaceFactory {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionSurface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSurface)(windows_core::Interface::as_raw(self), width, height, pixelformat, alphamode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionVirtualSurface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVirtualSurface)(windows_core::Interface::as_raw(self), initialwidth, initialheight, pixelformat, alphamode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionSurfaceFactory_Impl: windows_core::IUnknownImpl {
    fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionSurface>;
    fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionVirtualSurface>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionSurfaceFactory_Vtbl {
    pub const fn new<Identity: IDCompositionSurfaceFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSurface<Identity: IDCompositionSurfaceFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionSurfaceFactory_Impl::CreateSurface(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&alphamode)) {
                    Ok(ok__) => {
                        surface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Identity: IDCompositionSurfaceFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionSurfaceFactory_Impl::CreateVirtualSurface(this, core::mem::transmute_copy(&initialwidth), core::mem::transmute_copy(&initialheight), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&alphamode)) {
                    Ok(ok__) => {
                        virtualsurface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            CreateVirtualSurface: CreateVirtualSurface::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionSurfaceFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDCompositionSurfaceFactory {}
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
        unsafe { (windows_core::Interface::vtable(self).SetRedTable)(windows_core::Interface::as_raw(self), core::mem::transmute(tablevalues.as_ptr()), tablevalues.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn SetGreenTable(&self, tablevalues: &[f32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGreenTable)(windows_core::Interface::as_raw(self), core::mem::transmute(tablevalues.as_ptr()), tablevalues.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn SetBlueTable(&self, tablevalues: &[f32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBlueTable)(windows_core::Interface::as_raw(self), core::mem::transmute(tablevalues.as_ptr()), tablevalues.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn SetAlphaTable(&self, tablevalues: &[f32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAlphaTable)(windows_core::Interface::as_raw(self), core::mem::transmute(tablevalues.as_ptr()), tablevalues.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn SetRedDisable(&self, reddisable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRedDisable)(windows_core::Interface::as_raw(self), reddisable.into()).ok() }
    }
    pub unsafe fn SetGreenDisable(&self, greendisable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGreenDisable)(windows_core::Interface::as_raw(self), greendisable.into()).ok() }
    }
    pub unsafe fn SetBlueDisable(&self, bluedisable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBlueDisable)(windows_core::Interface::as_raw(self), bluedisable.into()).ok() }
    }
    pub unsafe fn SetAlphaDisable(&self, alphadisable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAlphaDisable)(windows_core::Interface::as_raw(self), alphadisable.into()).ok() }
    }
    pub unsafe fn SetClampOutput(&self, clampoutput: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClampOutput)(windows_core::Interface::as_raw(self), clampoutput.into()).ok() }
    }
    pub unsafe fn SetRedTableValue<P1>(&self, index: u32, animation: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRedTableValue)(windows_core::Interface::as_raw(self), index, animation.param().abi()).ok() }
    }
    pub unsafe fn SetRedTableValue2(&self, index: u32, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRedTableValue2)(windows_core::Interface::as_raw(self), index, value).ok() }
    }
    pub unsafe fn SetGreenTableValue<P1>(&self, index: u32, animation: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetGreenTableValue)(windows_core::Interface::as_raw(self), index, animation.param().abi()).ok() }
    }
    pub unsafe fn SetGreenTableValue2(&self, index: u32, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGreenTableValue2)(windows_core::Interface::as_raw(self), index, value).ok() }
    }
    pub unsafe fn SetBlueTableValue<P1>(&self, index: u32, animation: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBlueTableValue)(windows_core::Interface::as_raw(self), index, animation.param().abi()).ok() }
    }
    pub unsafe fn SetBlueTableValue2(&self, index: u32, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBlueTableValue2)(windows_core::Interface::as_raw(self), index, value).ok() }
    }
    pub unsafe fn SetAlphaTableValue<P1>(&self, index: u32, animation: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAlphaTableValue)(windows_core::Interface::as_raw(self), index, animation.param().abi()).ok() }
    }
    pub unsafe fn SetAlphaTableValue2(&self, index: u32, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAlphaTableValue2)(windows_core::Interface::as_raw(self), index, value).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTableTransferEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetRedTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32) -> windows_core::HRESULT,
    pub SetGreenTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32) -> windows_core::HRESULT,
    pub SetBlueTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32) -> windows_core::HRESULT,
    pub SetAlphaTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32) -> windows_core::HRESULT,
    pub SetRedDisable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetGreenDisable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetBlueDisable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetAlphaDisable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetClampOutput: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetRedTableValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRedTableValue2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32) -> windows_core::HRESULT,
    pub SetGreenTableValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGreenTableValue2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32) -> windows_core::HRESULT,
    pub SetBlueTableValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBlueTableValue2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32) -> windows_core::HRESULT,
    pub SetAlphaTableValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAlphaTableValue2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32) -> windows_core::HRESULT,
}
pub trait IDCompositionTableTransferEffect_Impl: IDCompositionFilterEffect_Impl {
    fn SetRedTable(&self, tablevalues: *const f32, count: u32) -> windows_core::Result<()>;
    fn SetGreenTable(&self, tablevalues: *const f32, count: u32) -> windows_core::Result<()>;
    fn SetBlueTable(&self, tablevalues: *const f32, count: u32) -> windows_core::Result<()>;
    fn SetAlphaTable(&self, tablevalues: *const f32, count: u32) -> windows_core::Result<()>;
    fn SetRedDisable(&self, reddisable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetGreenDisable(&self, greendisable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetBlueDisable(&self, bluedisable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetAlphaDisable(&self, alphadisable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetClampOutput(&self, clampoutput: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetRedTableValue(&self, index: u32, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetRedTableValue2(&self, index: u32, value: f32) -> windows_core::Result<()>;
    fn SetGreenTableValue(&self, index: u32, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetGreenTableValue2(&self, index: u32, value: f32) -> windows_core::Result<()>;
    fn SetBlueTableValue(&self, index: u32, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBlueTableValue2(&self, index: u32, value: f32) -> windows_core::Result<()>;
    fn SetAlphaTableValue(&self, index: u32, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAlphaTableValue2(&self, index: u32, value: f32) -> windows_core::Result<()>;
}
impl IDCompositionTableTransferEffect_Vtbl {
    pub const fn new<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRedTable<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tablevalues: *const f32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTableTransferEffect_Impl::SetRedTable(this, core::mem::transmute_copy(&tablevalues), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetGreenTable<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tablevalues: *const f32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTableTransferEffect_Impl::SetGreenTable(this, core::mem::transmute_copy(&tablevalues), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetBlueTable<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tablevalues: *const f32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTableTransferEffect_Impl::SetBlueTable(this, core::mem::transmute_copy(&tablevalues), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetAlphaTable<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tablevalues: *const f32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTableTransferEffect_Impl::SetAlphaTable(this, core::mem::transmute_copy(&tablevalues), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetRedDisable<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reddisable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTableTransferEffect_Impl::SetRedDisable(this, core::mem::transmute_copy(&reddisable)).into()
            }
        }
        unsafe extern "system" fn SetGreenDisable<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, greendisable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTableTransferEffect_Impl::SetGreenDisable(this, core::mem::transmute_copy(&greendisable)).into()
            }
        }
        unsafe extern "system" fn SetBlueDisable<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bluedisable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTableTransferEffect_Impl::SetBlueDisable(this, core::mem::transmute_copy(&bluedisable)).into()
            }
        }
        unsafe extern "system" fn SetAlphaDisable<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alphadisable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTableTransferEffect_Impl::SetAlphaDisable(this, core::mem::transmute_copy(&alphadisable)).into()
            }
        }
        unsafe extern "system" fn SetClampOutput<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clampoutput: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTableTransferEffect_Impl::SetClampOutput(this, core::mem::transmute_copy(&clampoutput)).into()
            }
        }
        unsafe extern "system" fn SetRedTableValue<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTableTransferEffect_Impl::SetRedTableValue(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetRedTableValue2<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTableTransferEffect_Impl::SetRedTableValue2(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetGreenTableValue<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTableTransferEffect_Impl::SetGreenTableValue(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetGreenTableValue2<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTableTransferEffect_Impl::SetGreenTableValue2(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetBlueTableValue<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTableTransferEffect_Impl::SetBlueTableValue(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetBlueTableValue2<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTableTransferEffect_Impl::SetBlueTableValue2(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetAlphaTableValue<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTableTransferEffect_Impl::SetAlphaTableValue(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetAlphaTableValue2<Identity: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTableTransferEffect_Impl::SetAlphaTableValue2(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetRedTable: SetRedTable::<Identity, OFFSET>,
            SetGreenTable: SetGreenTable::<Identity, OFFSET>,
            SetBlueTable: SetBlueTable::<Identity, OFFSET>,
            SetAlphaTable: SetAlphaTable::<Identity, OFFSET>,
            SetRedDisable: SetRedDisable::<Identity, OFFSET>,
            SetGreenDisable: SetGreenDisable::<Identity, OFFSET>,
            SetBlueDisable: SetBlueDisable::<Identity, OFFSET>,
            SetAlphaDisable: SetAlphaDisable::<Identity, OFFSET>,
            SetClampOutput: SetClampOutput::<Identity, OFFSET>,
            SetRedTableValue: SetRedTableValue::<Identity, OFFSET>,
            SetRedTableValue2: SetRedTableValue2::<Identity, OFFSET>,
            SetGreenTableValue: SetGreenTableValue::<Identity, OFFSET>,
            SetGreenTableValue2: SetGreenTableValue2::<Identity, OFFSET>,
            SetBlueTableValue: SetBlueTableValue::<Identity, OFFSET>,
            SetBlueTableValue2: SetBlueTableValue2::<Identity, OFFSET>,
            SetAlphaTableValue: SetAlphaTableValue::<Identity, OFFSET>,
            SetAlphaTableValue2: SetAlphaTableValue2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTableTransferEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionTableTransferEffect {}
windows_core::imp::define_interface!(IDCompositionTarget, IDCompositionTarget_Vtbl, 0xeacdd04c_117e_4e17_88f4_d1b12b0e3d89);
windows_core::imp::interface_hierarchy!(IDCompositionTarget, windows_core::IUnknown);
impl IDCompositionTarget {
    pub unsafe fn SetRoot<P0>(&self, visual: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionVisual>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRoot)(windows_core::Interface::as_raw(self), visual.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDCompositionTarget_Impl: windows_core::IUnknownImpl {
    fn SetRoot(&self, visual: windows_core::Ref<IDCompositionVisual>) -> windows_core::Result<()>;
}
impl IDCompositionTarget_Vtbl {
    pub const fn new<Identity: IDCompositionTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRoot<Identity: IDCompositionTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTarget_Impl::SetRoot(this, core::mem::transmute_copy(&visual)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetRoot: SetRoot::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTarget as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionTarget {}
windows_core::imp::define_interface!(IDCompositionTexture, IDCompositionTexture_Vtbl, 0x929bb1aa_725f_433b_abd7_273075a835f2);
windows_core::imp::interface_hierarchy!(IDCompositionTexture, windows_core::IUnknown);
impl IDCompositionTexture {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetSourceRect(&self, sourcerect: *const super::Direct2D::Common::D2D_RECT_U) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSourceRect)(windows_core::Interface::as_raw(self), sourcerect).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetColorSpace)(windows_core::Interface::as_raw(self), colorspace).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetAlphaMode(&self, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAlphaMode)(windows_core::Interface::as_raw(self), alphamode).ok() }
    }
    pub unsafe fn GetAvailableFence<T>(&self, fencevalue: *mut u64) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetAvailableFence)(windows_core::Interface::as_raw(self), fencevalue as _, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDCompositionTexture_Impl: windows_core::IUnknownImpl {
    fn SetSourceRect(&self, sourcerect: *const super::Direct2D::Common::D2D_RECT_U) -> windows_core::Result<()>;
    fn SetColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<()>;
    fn SetAlphaMode(&self, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<()>;
    fn GetAvailableFence(&self, fencevalue: *mut u64, iid: *const windows_core::GUID, availablefence: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDCompositionTexture_Vtbl {
    pub const fn new<Identity: IDCompositionTexture_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSourceRect<Identity: IDCompositionTexture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcerect: *const super::Direct2D::Common::D2D_RECT_U) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTexture_Impl::SetSourceRect(this, core::mem::transmute_copy(&sourcerect)).into()
            }
        }
        unsafe extern "system" fn SetColorSpace<Identity: IDCompositionTexture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTexture_Impl::SetColorSpace(this, core::mem::transmute_copy(&colorspace)).into()
            }
        }
        unsafe extern "system" fn SetAlphaMode<Identity: IDCompositionTexture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTexture_Impl::SetAlphaMode(this, core::mem::transmute_copy(&alphamode)).into()
            }
        }
        unsafe extern "system" fn GetAvailableFence<Identity: IDCompositionTexture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fencevalue: *mut u64, iid: *const windows_core::GUID, availablefence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTexture_Impl::GetAvailableFence(this, core::mem::transmute_copy(&fencevalue), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&availablefence)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSourceRect: SetSourceRect::<Identity, OFFSET>,
            SetColorSpace: SetColorSpace::<Identity, OFFSET>,
            SetAlphaMode: SetAlphaMode::<Identity, OFFSET>,
            GetAvailableFence: GetAvailableFence::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTexture as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for IDCompositionTexture {}
windows_core::imp::define_interface!(IDCompositionTransform, IDCompositionTransform_Vtbl, 0xfd55faa7_37e0_4c20_95d2_9be45bc33f55);
impl core::ops::Deref for IDCompositionTransform {
    type Target = IDCompositionTransform3D;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionTransform, windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D);
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTransform_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
}
pub trait IDCompositionTransform_Impl: IDCompositionTransform3D_Impl {}
impl IDCompositionTransform_Vtbl {
    pub const fn new<Identity: IDCompositionTransform_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IDCompositionTransform3D_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTransform as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionTransform {}
windows_core::imp::define_interface!(IDCompositionTransform3D, IDCompositionTransform3D_Vtbl, 0x71185722_246b_41f2_aad1_0443f7f4bfc2);
impl core::ops::Deref for IDCompositionTransform3D {
    type Target = IDCompositionEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionTransform3D, windows_core::IUnknown, IDCompositionEffect);
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTransform3D_Vtbl {
    pub base__: IDCompositionEffect_Vtbl,
}
pub trait IDCompositionTransform3D_Impl: IDCompositionEffect_Impl {}
impl IDCompositionTransform3D_Vtbl {
    pub const fn new<Identity: IDCompositionTransform3D_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IDCompositionEffect_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionTransform3D {}
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
        unsafe { (windows_core::Interface::vtable(self).SetOffsetX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOffsetX2)(windows_core::Interface::as_raw(self), offsetx).ok() }
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOffsetY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOffsetY2)(windows_core::Interface::as_raw(self), offsety).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTranslateTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetOffsetX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
pub trait IDCompositionTranslateTransform_Impl: IDCompositionTransform_Impl {
    fn SetOffsetX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOffsetX2(&self, offsetx: f32) -> windows_core::Result<()>;
    fn SetOffsetY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOffsetY2(&self, offsety: f32) -> windows_core::Result<()>;
}
impl IDCompositionTranslateTransform_Vtbl {
    pub const fn new<Identity: IDCompositionTranslateTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOffsetX<Identity: IDCompositionTranslateTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTranslateTransform_Impl::SetOffsetX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetOffsetX2<Identity: IDCompositionTranslateTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsetx: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTranslateTransform_Impl::SetOffsetX2(this, core::mem::transmute_copy(&offsetx)).into()
            }
        }
        unsafe extern "system" fn SetOffsetY<Identity: IDCompositionTranslateTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTranslateTransform_Impl::SetOffsetY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetOffsetY2<Identity: IDCompositionTranslateTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsety: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTranslateTransform_Impl::SetOffsetY2(this, core::mem::transmute_copy(&offsety)).into()
            }
        }
        Self {
            base__: IDCompositionTransform_Vtbl::new::<Identity, OFFSET>(),
            SetOffsetX: SetOffsetX::<Identity, OFFSET>,
            SetOffsetX2: SetOffsetX2::<Identity, OFFSET>,
            SetOffsetY: SetOffsetY::<Identity, OFFSET>,
            SetOffsetY2: SetOffsetY2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTranslateTransform as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionTransform as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionTranslateTransform {}
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
        unsafe { (windows_core::Interface::vtable(self).SetOffsetX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOffsetX2)(windows_core::Interface::as_raw(self), offsetx).ok() }
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOffsetY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOffsetY2)(windows_core::Interface::as_raw(self), offsety).ok() }
    }
    pub unsafe fn SetOffsetZ<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOffsetZ)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetOffsetZ2(&self, offsetz: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOffsetZ2)(windows_core::Interface::as_raw(self), offsetz).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTranslateTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    pub SetOffsetX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetOffsetZ: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOffsetZ2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
pub trait IDCompositionTranslateTransform3D_Impl: IDCompositionTransform3D_Impl {
    fn SetOffsetX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOffsetX2(&self, offsetx: f32) -> windows_core::Result<()>;
    fn SetOffsetY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOffsetY2(&self, offsety: f32) -> windows_core::Result<()>;
    fn SetOffsetZ(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOffsetZ2(&self, offsetz: f32) -> windows_core::Result<()>;
}
impl IDCompositionTranslateTransform3D_Vtbl {
    pub const fn new<Identity: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOffsetX<Identity: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTranslateTransform3D_Impl::SetOffsetX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetOffsetX2<Identity: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsetx: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTranslateTransform3D_Impl::SetOffsetX2(this, core::mem::transmute_copy(&offsetx)).into()
            }
        }
        unsafe extern "system" fn SetOffsetY<Identity: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTranslateTransform3D_Impl::SetOffsetY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetOffsetY2<Identity: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsety: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTranslateTransform3D_Impl::SetOffsetY2(this, core::mem::transmute_copy(&offsety)).into()
            }
        }
        unsafe extern "system" fn SetOffsetZ<Identity: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTranslateTransform3D_Impl::SetOffsetZ(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetOffsetZ2<Identity: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsetz: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTranslateTransform3D_Impl::SetOffsetZ2(this, core::mem::transmute_copy(&offsetz)).into()
            }
        }
        Self {
            base__: IDCompositionTransform3D_Vtbl::new::<Identity, OFFSET>(),
            SetOffsetX: SetOffsetX::<Identity, OFFSET>,
            SetOffsetX2: SetOffsetX2::<Identity, OFFSET>,
            SetOffsetY: SetOffsetY::<Identity, OFFSET>,
            SetOffsetY2: SetOffsetY2::<Identity, OFFSET>,
            SetOffsetZ: SetOffsetZ::<Identity, OFFSET>,
            SetOffsetZ2: SetOffsetZ2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTranslateTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionTranslateTransform3D {}
windows_core::imp::define_interface!(IDCompositionTurbulenceEffect, IDCompositionTurbulenceEffect_Vtbl, 0xa6a55bda_c09c_49f3_9193_a41922c89715);
impl core::ops::Deref for IDCompositionTurbulenceEffect {
    type Target = IDCompositionFilterEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCompositionTurbulenceEffect, windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl IDCompositionTurbulenceEffect {
    pub unsafe fn SetOffset(&self, offset: *const windows_numerics::Vector2) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOffset)(windows_core::Interface::as_raw(self), offset).ok() }
    }
    pub unsafe fn SetBaseFrequency(&self, frequency: *const windows_numerics::Vector2) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBaseFrequency)(windows_core::Interface::as_raw(self), frequency).ok() }
    }
    pub unsafe fn SetSize(&self, size: *const windows_numerics::Vector2) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), size).ok() }
    }
    pub unsafe fn SetNumOctaves(&self, numoctaves: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNumOctaves)(windows_core::Interface::as_raw(self), numoctaves).ok() }
    }
    pub unsafe fn SetSeed(&self, seed: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSeed)(windows_core::Interface::as_raw(self), seed).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetNoise(&self, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNoise)(windows_core::Interface::as_raw(self), noise).ok() }
    }
    pub unsafe fn SetStitchable(&self, stitchable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStitchable)(windows_core::Interface::as_raw(self), stitchable.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTurbulenceEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Vector2) -> windows_core::HRESULT,
    pub SetBaseFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Vector2) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Vector2) -> windows_core::HRESULT,
    pub SetNumOctaves: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetSeed: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetNoise: unsafe extern "system" fn(*mut core::ffi::c_void, super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetNoise: usize,
    pub SetStitchable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionTurbulenceEffect_Impl: IDCompositionFilterEffect_Impl {
    fn SetOffset(&self, offset: *const windows_numerics::Vector2) -> windows_core::Result<()>;
    fn SetBaseFrequency(&self, frequency: *const windows_numerics::Vector2) -> windows_core::Result<()>;
    fn SetSize(&self, size: *const windows_numerics::Vector2) -> windows_core::Result<()>;
    fn SetNumOctaves(&self, numoctaves: u32) -> windows_core::Result<()>;
    fn SetSeed(&self, seed: u32) -> windows_core::Result<()>;
    fn SetNoise(&self, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> windows_core::Result<()>;
    fn SetStitchable(&self, stitchable: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionTurbulenceEffect_Vtbl {
    pub const fn new<Identity: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOffset<Identity: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: *const windows_numerics::Vector2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTurbulenceEffect_Impl::SetOffset(this, core::mem::transmute_copy(&offset)).into()
            }
        }
        unsafe extern "system" fn SetBaseFrequency<Identity: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frequency: *const windows_numerics::Vector2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTurbulenceEffect_Impl::SetBaseFrequency(this, core::mem::transmute_copy(&frequency)).into()
            }
        }
        unsafe extern "system" fn SetSize<Identity: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *const windows_numerics::Vector2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTurbulenceEffect_Impl::SetSize(this, core::mem::transmute_copy(&size)).into()
            }
        }
        unsafe extern "system" fn SetNumOctaves<Identity: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numoctaves: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTurbulenceEffect_Impl::SetNumOctaves(this, core::mem::transmute_copy(&numoctaves)).into()
            }
        }
        unsafe extern "system" fn SetSeed<Identity: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seed: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTurbulenceEffect_Impl::SetSeed(this, core::mem::transmute_copy(&seed)).into()
            }
        }
        unsafe extern "system" fn SetNoise<Identity: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTurbulenceEffect_Impl::SetNoise(this, core::mem::transmute_copy(&noise)).into()
            }
        }
        unsafe extern "system" fn SetStitchable<Identity: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stitchable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTurbulenceEffect_Impl::SetStitchable(this, core::mem::transmute_copy(&stitchable)).into()
            }
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetOffset: SetOffset::<Identity, OFFSET>,
            SetBaseFrequency: SetBaseFrequency::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            SetNumOctaves: SetNumOctaves::<Identity, OFFSET>,
            SetSeed: SetSeed::<Identity, OFFSET>,
            SetNoise: SetNoise::<Identity, OFFSET>,
            SetStitchable: SetStitchable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTurbulenceEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionTurbulenceEffect {}
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
        unsafe { (windows_core::Interface::vtable(self).Resize)(windows_core::Interface::as_raw(self), width, height).ok() }
    }
    pub unsafe fn Trim(&self, rectangles: Option<&[super::super::Foundation::RECT]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Trim)(windows_core::Interface::as_raw(self), core::mem::transmute(rectangles.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), rectangles.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVirtualSurface_Vtbl {
    pub base__: IDCompositionSurface_Vtbl,
    pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Trim: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, u32) -> windows_core::HRESULT,
}
pub trait IDCompositionVirtualSurface_Impl: IDCompositionSurface_Impl {
    fn Resize(&self, width: u32, height: u32) -> windows_core::Result<()>;
    fn Trim(&self, rectangles: *const super::super::Foundation::RECT, count: u32) -> windows_core::Result<()>;
}
impl IDCompositionVirtualSurface_Vtbl {
    pub const fn new<Identity: IDCompositionVirtualSurface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Resize<Identity: IDCompositionVirtualSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVirtualSurface_Impl::Resize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn Trim<Identity: IDCompositionVirtualSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangles: *const super::super::Foundation::RECT, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVirtualSurface_Impl::Trim(this, core::mem::transmute_copy(&rectangles), core::mem::transmute_copy(&count)).into()
            }
        }
        Self { base__: IDCompositionSurface_Vtbl::new::<Identity, OFFSET>(), Resize: Resize::<Identity, OFFSET>, Trim: Trim::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionVirtualSurface as windows_core::Interface>::IID || iid == &<IDCompositionSurface as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionVirtualSurface {}
windows_core::imp::define_interface!(IDCompositionVisual, IDCompositionVisual_Vtbl, 0x4d93059d_097b_4651_9a60_f0f25116e2f3);
windows_core::imp::interface_hierarchy!(IDCompositionVisual, windows_core::IUnknown);
impl IDCompositionVisual {
    pub unsafe fn SetOffsetX<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOffsetX)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOffsetX2)(windows_core::Interface::as_raw(self), offsetx).ok() }
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOffsetY)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOffsetY2)(windows_core::Interface::as_raw(self), offsety).ok() }
    }
    pub unsafe fn SetTransform<P0>(&self, transform: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionTransform>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), transform.param().abi()).ok() }
    }
    pub unsafe fn SetTransform2(&self, matrix: *const windows_numerics::Matrix3x2) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTransform2)(windows_core::Interface::as_raw(self), matrix).ok() }
    }
    pub unsafe fn SetTransformParent<P0>(&self, visual: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionVisual>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTransformParent)(windows_core::Interface::as_raw(self), visual.param().abi()).ok() }
    }
    pub unsafe fn SetEffect<P0>(&self, effect: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionEffect>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEffect)(windows_core::Interface::as_raw(self), effect.param().abi()).ok() }
    }
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBitmapInterpolationMode)(windows_core::Interface::as_raw(self), interpolationmode).ok() }
    }
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBorderMode)(windows_core::Interface::as_raw(self), bordermode).ok() }
    }
    pub unsafe fn SetClip<P0>(&self, clip: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionClip>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetClip)(windows_core::Interface::as_raw(self), clip.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClip2)(windows_core::Interface::as_raw(self), rect).ok() }
    }
    pub unsafe fn SetContent<P0>(&self, content: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContent)(windows_core::Interface::as_raw(self), content.param().abi()).ok() }
    }
    pub unsafe fn AddVisual<P0, P2>(&self, visual: P0, insertabove: bool, referencevisual: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionVisual>,
        P2: windows_core::Param<IDCompositionVisual>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddVisual)(windows_core::Interface::as_raw(self), visual.param().abi(), insertabove.into(), referencevisual.param().abi()).ok() }
    }
    pub unsafe fn RemoveVisual<P0>(&self, visual: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionVisual>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveVisual)(windows_core::Interface::as_raw(self), visual.param().abi()).ok() }
    }
    pub unsafe fn RemoveAllVisuals(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllVisuals)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCompositeMode)(windows_core::Interface::as_raw(self), compositemode).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetOffsetX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTransform2: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Matrix3x2) -> windows_core::HRESULT,
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
    pub AddVisual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveVisual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAllVisuals: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCompositeMode: unsafe extern "system" fn(*mut core::ffi::c_void, DCOMPOSITION_COMPOSITE_MODE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionVisual_Impl: windows_core::IUnknownImpl {
    fn SetOffsetX(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOffsetX2(&self, offsetx: f32) -> windows_core::Result<()>;
    fn SetOffsetY(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOffsetY2(&self, offsety: f32) -> windows_core::Result<()>;
    fn SetTransform(&self, transform: windows_core::Ref<IDCompositionTransform>) -> windows_core::Result<()>;
    fn SetTransform2(&self, matrix: *const windows_numerics::Matrix3x2) -> windows_core::Result<()>;
    fn SetTransformParent(&self, visual: windows_core::Ref<IDCompositionVisual>) -> windows_core::Result<()>;
    fn SetEffect(&self, effect: windows_core::Ref<IDCompositionEffect>) -> windows_core::Result<()>;
    fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> windows_core::Result<()>;
    fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> windows_core::Result<()>;
    fn SetClip(&self, clip: windows_core::Ref<IDCompositionClip>) -> windows_core::Result<()>;
    fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> windows_core::Result<()>;
    fn SetContent(&self, content: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn AddVisual(&self, visual: windows_core::Ref<IDCompositionVisual>, insertabove: windows_core::BOOL, referencevisual: windows_core::Ref<IDCompositionVisual>) -> windows_core::Result<()>;
    fn RemoveVisual(&self, visual: windows_core::Ref<IDCompositionVisual>) -> windows_core::Result<()>;
    fn RemoveAllVisuals(&self) -> windows_core::Result<()>;
    fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionVisual_Vtbl {
    pub const fn new<Identity: IDCompositionVisual_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOffsetX<Identity: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetOffsetX(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetOffsetX2<Identity: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsetx: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetOffsetX2(this, core::mem::transmute_copy(&offsetx)).into()
            }
        }
        unsafe extern "system" fn SetOffsetY<Identity: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetOffsetY(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetOffsetY2<Identity: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsety: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetOffsetY2(this, core::mem::transmute_copy(&offsety)).into()
            }
        }
        unsafe extern "system" fn SetTransform<Identity: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetTransform(this, core::mem::transmute_copy(&transform)).into()
            }
        }
        unsafe extern "system" fn SetTransform2<Identity: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const windows_numerics::Matrix3x2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetTransform2(this, core::mem::transmute_copy(&matrix)).into()
            }
        }
        unsafe extern "system" fn SetTransformParent<Identity: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetTransformParent(this, core::mem::transmute_copy(&visual)).into()
            }
        }
        unsafe extern "system" fn SetEffect<Identity: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, effect: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetEffect(this, core::mem::transmute_copy(&effect)).into()
            }
        }
        unsafe extern "system" fn SetBitmapInterpolationMode<Identity: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetBitmapInterpolationMode(this, core::mem::transmute_copy(&interpolationmode)).into()
            }
        }
        unsafe extern "system" fn SetBorderMode<Identity: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bordermode: DCOMPOSITION_BORDER_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetBorderMode(this, core::mem::transmute_copy(&bordermode)).into()
            }
        }
        unsafe extern "system" fn SetClip<Identity: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clip: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetClip(this, core::mem::transmute_copy(&clip)).into()
            }
        }
        unsafe extern "system" fn SetClip2<Identity: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *const super::Direct2D::Common::D2D_RECT_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetClip2(this, core::mem::transmute_copy(&rect)).into()
            }
        }
        unsafe extern "system" fn SetContent<Identity: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, content: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetContent(this, core::mem::transmute_copy(&content)).into()
            }
        }
        unsafe extern "system" fn AddVisual<Identity: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut core::ffi::c_void, insertabove: windows_core::BOOL, referencevisual: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::AddVisual(this, core::mem::transmute_copy(&visual), core::mem::transmute_copy(&insertabove), core::mem::transmute_copy(&referencevisual)).into()
            }
        }
        unsafe extern "system" fn RemoveVisual<Identity: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::RemoveVisual(this, core::mem::transmute_copy(&visual)).into()
            }
        }
        unsafe extern "system" fn RemoveAllVisuals<Identity: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::RemoveAllVisuals(this).into()
            }
        }
        unsafe extern "system" fn SetCompositeMode<Identity: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetCompositeMode(this, core::mem::transmute_copy(&compositemode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetOffsetX: SetOffsetX::<Identity, OFFSET>,
            SetOffsetX2: SetOffsetX2::<Identity, OFFSET>,
            SetOffsetY: SetOffsetY::<Identity, OFFSET>,
            SetOffsetY2: SetOffsetY2::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            SetTransform2: SetTransform2::<Identity, OFFSET>,
            SetTransformParent: SetTransformParent::<Identity, OFFSET>,
            SetEffect: SetEffect::<Identity, OFFSET>,
            SetBitmapInterpolationMode: SetBitmapInterpolationMode::<Identity, OFFSET>,
            SetBorderMode: SetBorderMode::<Identity, OFFSET>,
            SetClip: SetClip::<Identity, OFFSET>,
            SetClip2: SetClip2::<Identity, OFFSET>,
            SetContent: SetContent::<Identity, OFFSET>,
            AddVisual: AddVisual::<Identity, OFFSET>,
            RemoveVisual: RemoveVisual::<Identity, OFFSET>,
            RemoveAllVisuals: RemoveAllVisuals::<Identity, OFFSET>,
            SetCompositeMode: SetCompositeMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionVisual as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionVisual {}
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
        unsafe { (windows_core::Interface::vtable(self).SetOpacityMode)(windows_core::Interface::as_raw(self), mode).ok() }
    }
    pub unsafe fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBackFaceVisibility)(windows_core::Interface::as_raw(self), visibility).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual2_Vtbl {
    pub base__: IDCompositionVisual_Vtbl,
    pub SetOpacityMode: unsafe extern "system" fn(*mut core::ffi::c_void, DCOMPOSITION_OPACITY_MODE) -> windows_core::HRESULT,
    pub SetBackFaceVisibility: unsafe extern "system" fn(*mut core::ffi::c_void, DCOMPOSITION_BACKFACE_VISIBILITY) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionVisual2_Impl: IDCompositionVisual_Impl {
    fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> windows_core::Result<()>;
    fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionVisual2_Vtbl {
    pub const fn new<Identity: IDCompositionVisual2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOpacityMode<Identity: IDCompositionVisual2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: DCOMPOSITION_OPACITY_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual2_Impl::SetOpacityMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn SetBackFaceVisibility<Identity: IDCompositionVisual2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual2_Impl::SetBackFaceVisibility(this, core::mem::transmute_copy(&visibility)).into()
            }
        }
        Self {
            base__: IDCompositionVisual_Vtbl::new::<Identity, OFFSET>(),
            SetOpacityMode: SetOpacityMode::<Identity, OFFSET>,
            SetBackFaceVisibility: SetBackFaceVisibility::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionVisual2 as windows_core::Interface>::IID || iid == &<IDCompositionVisual as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionVisual2 {}
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
        unsafe { (windows_core::Interface::vtable(self).SetDepthMode)(windows_core::Interface::as_raw(self), mode).ok() }
    }
    pub unsafe fn SetOffsetZ<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOffsetZ)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetOffsetZ2(&self, offsetz: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOffsetZ2)(windows_core::Interface::as_raw(self), offsetz).ok() }
    }
    pub unsafe fn SetOpacity<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOpacity)(windows_core::Interface::as_raw(self), animation.param().abi()).ok() }
    }
    pub unsafe fn SetOpacity2(&self, opacity: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOpacity2)(windows_core::Interface::as_raw(self), opacity).ok() }
    }
    pub unsafe fn SetTransform<P0>(&self, transform: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDCompositionTransform3D>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), transform.param().abi()).ok() }
    }
    pub unsafe fn SetTransform2(&self, matrix: *const windows_numerics::Matrix4x4) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTransform2)(windows_core::Interface::as_raw(self), matrix).ok() }
    }
    pub unsafe fn SetVisible(&self, visible: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVisible)(windows_core::Interface::as_raw(self), visible.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual3_Vtbl {
    pub base__: IDCompositionVisualDebug_Vtbl,
    pub SetDepthMode: unsafe extern "system" fn(*mut core::ffi::c_void, DCOMPOSITION_DEPTH_MODE) -> windows_core::HRESULT,
    pub SetOffsetZ: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOffsetZ2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOpacity2: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTransform2: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Matrix4x4) -> windows_core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionVisual3_Impl: IDCompositionVisualDebug_Impl {
    fn SetDepthMode(&self, mode: DCOMPOSITION_DEPTH_MODE) -> windows_core::Result<()>;
    fn SetOffsetZ(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOffsetZ2(&self, offsetz: f32) -> windows_core::Result<()>;
    fn SetOpacity(&self, animation: windows_core::Ref<IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOpacity2(&self, opacity: f32) -> windows_core::Result<()>;
    fn SetTransform(&self, transform: windows_core::Ref<IDCompositionTransform3D>) -> windows_core::Result<()>;
    fn SetTransform2(&self, matrix: *const windows_numerics::Matrix4x4) -> windows_core::Result<()>;
    fn SetVisible(&self, visible: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionVisual3_Vtbl {
    pub const fn new<Identity: IDCompositionVisual3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDepthMode<Identity: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: DCOMPOSITION_DEPTH_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual3_Impl::SetDepthMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn SetOffsetZ<Identity: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual3_Impl::SetOffsetZ(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetOffsetZ2<Identity: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsetz: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual3_Impl::SetOffsetZ2(this, core::mem::transmute_copy(&offsetz)).into()
            }
        }
        unsafe extern "system" fn SetOpacity<Identity: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual3_Impl::SetOpacity(this, core::mem::transmute_copy(&animation)).into()
            }
        }
        unsafe extern "system" fn SetOpacity2<Identity: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual3_Impl::SetOpacity2(this, core::mem::transmute_copy(&opacity)).into()
            }
        }
        unsafe extern "system" fn SetTransform<Identity: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual3_Impl::SetTransform(this, core::mem::transmute_copy(&transform)).into()
            }
        }
        unsafe extern "system" fn SetTransform2<Identity: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const windows_numerics::Matrix4x4) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual3_Impl::SetTransform2(this, core::mem::transmute_copy(&matrix)).into()
            }
        }
        unsafe extern "system" fn SetVisible<Identity: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual3_Impl::SetVisible(this, core::mem::transmute_copy(&visible)).into()
            }
        }
        Self {
            base__: IDCompositionVisualDebug_Vtbl::new::<Identity, OFFSET>(),
            SetDepthMode: SetDepthMode::<Identity, OFFSET>,
            SetOffsetZ: SetOffsetZ::<Identity, OFFSET>,
            SetOffsetZ2: SetOffsetZ2::<Identity, OFFSET>,
            SetOpacity: SetOpacity::<Identity, OFFSET>,
            SetOpacity2: SetOpacity2::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            SetTransform2: SetTransform2::<Identity, OFFSET>,
            SetVisible: SetVisible::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionVisual3 as windows_core::Interface>::IID || iid == &<IDCompositionVisual as windows_core::Interface>::IID || iid == &<IDCompositionVisual2 as windows_core::Interface>::IID || iid == &<IDCompositionVisualDebug as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionVisual3 {}
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
        unsafe { (windows_core::Interface::vtable(self).EnableHeatMap)(windows_core::Interface::as_raw(self), color).ok() }
    }
    pub unsafe fn DisableHeatMap(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisableHeatMap)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EnableRedrawRegions(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableRedrawRegions)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DisableRedrawRegions(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisableRedrawRegions)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionVisualDebug_Impl: IDCompositionVisual2_Impl {
    fn EnableHeatMap(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> windows_core::Result<()>;
    fn DisableHeatMap(&self) -> windows_core::Result<()>;
    fn EnableRedrawRegions(&self) -> windows_core::Result<()>;
    fn DisableRedrawRegions(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionVisualDebug_Vtbl {
    pub const fn new<Identity: IDCompositionVisualDebug_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnableHeatMap<Identity: IDCompositionVisualDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisualDebug_Impl::EnableHeatMap(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn DisableHeatMap<Identity: IDCompositionVisualDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisualDebug_Impl::DisableHeatMap(this).into()
            }
        }
        unsafe extern "system" fn EnableRedrawRegions<Identity: IDCompositionVisualDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisualDebug_Impl::EnableRedrawRegions(this).into()
            }
        }
        unsafe extern "system" fn DisableRedrawRegions<Identity: IDCompositionVisualDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisualDebug_Impl::DisableRedrawRegions(this).into()
            }
        }
        Self {
            base__: IDCompositionVisual2_Vtbl::new::<Identity, OFFSET>(),
            EnableHeatMap: EnableHeatMap::<Identity, OFFSET>,
            DisableHeatMap: DisableHeatMap::<Identity, OFFSET>,
            EnableRedrawRegions: EnableRedrawRegions::<Identity, OFFSET>,
            DisableRedrawRegions: DisableRedrawRegions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionVisualDebug as windows_core::Interface>::IID || iid == &<IDCompositionVisual as windows_core::Interface>::IID || iid == &<IDCompositionVisual2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionVisualDebug {}
