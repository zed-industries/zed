#[inline]
pub unsafe fn CloseGestureInfoHandle<P0>(hgestureinfo: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HGESTUREINFO>,
{
    windows_targets::link!("user32.dll" "system" fn CloseGestureInfoHandle(hgestureinfo : HGESTUREINFO) -> super::super::super::Foundation:: BOOL);
    CloseGestureInfoHandle(hgestureinfo.param().abi()).ok()
}
#[inline]
pub unsafe fn CloseTouchInputHandle<P0>(htouchinput: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HTOUCHINPUT>,
{
    windows_targets::link!("user32.dll" "system" fn CloseTouchInputHandle(htouchinput : HTOUCHINPUT) -> super::super::super::Foundation:: BOOL);
    CloseTouchInputHandle(htouchinput.param().abi()).ok()
}
#[inline]
pub unsafe fn GetGestureConfig<P0>(hwnd: P0, dwreserved: u32, dwflags: u32, pcids: *const u32, pgestureconfig: *mut GESTURECONFIG, cbsize: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetGestureConfig(hwnd : super::super::super::Foundation:: HWND, dwreserved : u32, dwflags : u32, pcids : *const u32, pgestureconfig : *mut GESTURECONFIG, cbsize : u32) -> super::super::super::Foundation:: BOOL);
    GetGestureConfig(hwnd.param().abi(), dwreserved, dwflags, pcids, pgestureconfig, cbsize).ok()
}
#[inline]
pub unsafe fn GetGestureExtraArgs<P0>(hgestureinfo: P0, pextraargs: &mut [u8]) -> windows_core::Result<()>
where
    P0: windows_core::Param<HGESTUREINFO>,
{
    windows_targets::link!("user32.dll" "system" fn GetGestureExtraArgs(hgestureinfo : HGESTUREINFO, cbextraargs : u32, pextraargs : *mut u8) -> super::super::super::Foundation:: BOOL);
    GetGestureExtraArgs(hgestureinfo.param().abi(), pextraargs.len().try_into().unwrap(), core::mem::transmute(pextraargs.as_ptr())).ok()
}
#[inline]
pub unsafe fn GetGestureInfo<P0>(hgestureinfo: P0, pgestureinfo: *mut GESTUREINFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<HGESTUREINFO>,
{
    windows_targets::link!("user32.dll" "system" fn GetGestureInfo(hgestureinfo : HGESTUREINFO, pgestureinfo : *mut GESTUREINFO) -> super::super::super::Foundation:: BOOL);
    GetGestureInfo(hgestureinfo.param().abi(), pgestureinfo).ok()
}
#[inline]
pub unsafe fn GetTouchInputInfo<P0>(htouchinput: P0, pinputs: &mut [TOUCHINPUT], cbsize: i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<HTOUCHINPUT>,
{
    windows_targets::link!("user32.dll" "system" fn GetTouchInputInfo(htouchinput : HTOUCHINPUT, cinputs : u32, pinputs : *mut TOUCHINPUT, cbsize : i32) -> super::super::super::Foundation:: BOOL);
    GetTouchInputInfo(htouchinput.param().abi(), pinputs.len().try_into().unwrap(), core::mem::transmute(pinputs.as_ptr()), cbsize).ok()
}
#[inline]
pub unsafe fn IsTouchWindow<P0>(hwnd: P0, pulflags: Option<*mut u32>) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn IsTouchWindow(hwnd : super::super::super::Foundation:: HWND, pulflags : *mut u32) -> super::super::super::Foundation:: BOOL);
    IsTouchWindow(hwnd.param().abi(), core::mem::transmute(pulflags.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RegisterTouchWindow<P0>(hwnd: P0, ulflags: REGISTER_TOUCH_WINDOW_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn RegisterTouchWindow(hwnd : super::super::super::Foundation:: HWND, ulflags : REGISTER_TOUCH_WINDOW_FLAGS) -> super::super::super::Foundation:: BOOL);
    RegisterTouchWindow(hwnd.param().abi(), ulflags).ok()
}
#[inline]
pub unsafe fn SetGestureConfig<P0>(hwnd: P0, dwreserved: u32, pgestureconfig: &[GESTURECONFIG], cbsize: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetGestureConfig(hwnd : super::super::super::Foundation:: HWND, dwreserved : u32, cids : u32, pgestureconfig : *const GESTURECONFIG, cbsize : u32) -> super::super::super::Foundation:: BOOL);
    SetGestureConfig(hwnd.param().abi(), dwreserved, pgestureconfig.len().try_into().unwrap(), core::mem::transmute(pgestureconfig.as_ptr()), cbsize).ok()
}
#[inline]
pub unsafe fn UnregisterTouchWindow<P0>(hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn UnregisterTouchWindow(hwnd : super::super::super::Foundation:: HWND) -> super::super::super::Foundation:: BOOL);
    UnregisterTouchWindow(hwnd.param().abi()).ok()
}
windows_core::imp::define_interface!(IInertiaProcessor, IInertiaProcessor_Vtbl, 0x18b00c6d_c5ee_41b1_90a9_9d4a929095ad);
impl core::ops::Deref for IInertiaProcessor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInertiaProcessor, windows_core::IUnknown);
impl IInertiaProcessor {
    pub unsafe fn InitialOriginX(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InitialOriginX)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetInitialOriginX(&self, x: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitialOriginX)(windows_core::Interface::as_raw(self), x).ok()
    }
    pub unsafe fn InitialOriginY(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InitialOriginY)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetInitialOriginY(&self, y: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitialOriginY)(windows_core::Interface::as_raw(self), y).ok()
    }
    pub unsafe fn InitialVelocityX(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InitialVelocityX)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetInitialVelocityX(&self, x: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitialVelocityX)(windows_core::Interface::as_raw(self), x).ok()
    }
    pub unsafe fn InitialVelocityY(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InitialVelocityY)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetInitialVelocityY(&self, y: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitialVelocityY)(windows_core::Interface::as_raw(self), y).ok()
    }
    pub unsafe fn InitialAngularVelocity(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InitialAngularVelocity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetInitialAngularVelocity(&self, velocity: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitialAngularVelocity)(windows_core::Interface::as_raw(self), velocity).ok()
    }
    pub unsafe fn InitialExpansionVelocity(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InitialExpansionVelocity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetInitialExpansionVelocity(&self, velocity: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitialExpansionVelocity)(windows_core::Interface::as_raw(self), velocity).ok()
    }
    pub unsafe fn InitialRadius(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InitialRadius)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetInitialRadius(&self, radius: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitialRadius)(windows_core::Interface::as_raw(self), radius).ok()
    }
    pub unsafe fn BoundaryLeft(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BoundaryLeft)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBoundaryLeft(&self, left: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBoundaryLeft)(windows_core::Interface::as_raw(self), left).ok()
    }
    pub unsafe fn BoundaryTop(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BoundaryTop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBoundaryTop(&self, top: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBoundaryTop)(windows_core::Interface::as_raw(self), top).ok()
    }
    pub unsafe fn BoundaryRight(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BoundaryRight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBoundaryRight(&self, right: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBoundaryRight)(windows_core::Interface::as_raw(self), right).ok()
    }
    pub unsafe fn BoundaryBottom(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BoundaryBottom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBoundaryBottom(&self, bottom: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBoundaryBottom)(windows_core::Interface::as_raw(self), bottom).ok()
    }
    pub unsafe fn ElasticMarginLeft(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ElasticMarginLeft)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetElasticMarginLeft(&self, left: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetElasticMarginLeft)(windows_core::Interface::as_raw(self), left).ok()
    }
    pub unsafe fn ElasticMarginTop(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ElasticMarginTop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetElasticMarginTop(&self, top: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetElasticMarginTop)(windows_core::Interface::as_raw(self), top).ok()
    }
    pub unsafe fn ElasticMarginRight(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ElasticMarginRight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetElasticMarginRight(&self, right: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetElasticMarginRight)(windows_core::Interface::as_raw(self), right).ok()
    }
    pub unsafe fn ElasticMarginBottom(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ElasticMarginBottom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetElasticMarginBottom(&self, bottom: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetElasticMarginBottom)(windows_core::Interface::as_raw(self), bottom).ok()
    }
    pub unsafe fn DesiredDisplacement(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DesiredDisplacement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDesiredDisplacement(&self, displacement: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDesiredDisplacement)(windows_core::Interface::as_raw(self), displacement).ok()
    }
    pub unsafe fn DesiredRotation(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DesiredRotation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDesiredRotation(&self, rotation: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDesiredRotation)(windows_core::Interface::as_raw(self), rotation).ok()
    }
    pub unsafe fn DesiredExpansion(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DesiredExpansion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDesiredExpansion(&self, expansion: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDesiredExpansion)(windows_core::Interface::as_raw(self), expansion).ok()
    }
    pub unsafe fn DesiredDeceleration(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DesiredDeceleration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDesiredDeceleration(&self, deceleration: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDesiredDeceleration)(windows_core::Interface::as_raw(self), deceleration).ok()
    }
    pub unsafe fn DesiredAngularDeceleration(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DesiredAngularDeceleration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDesiredAngularDeceleration(&self, deceleration: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDesiredAngularDeceleration)(windows_core::Interface::as_raw(self), deceleration).ok()
    }
    pub unsafe fn DesiredExpansionDeceleration(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DesiredExpansionDeceleration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDesiredExpansionDeceleration(&self, deceleration: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDesiredExpansionDeceleration)(windows_core::Interface::as_raw(self), deceleration).ok()
    }
    pub unsafe fn InitialTimestamp(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InitialTimestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetInitialTimestamp(&self, timestamp: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitialTimestamp)(windows_core::Interface::as_raw(self), timestamp).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Process(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Process)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ProcessTime(&self, timestamp: u32) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProcessTime)(windows_core::Interface::as_raw(self), timestamp, &mut result__).map(|| result__)
    }
    pub unsafe fn Complete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Complete)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CompleteTime(&self, timestamp: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CompleteTime)(windows_core::Interface::as_raw(self), timestamp).ok()
    }
}
#[repr(C)]
pub struct IInertiaProcessor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitialOriginX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInitialOriginX: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InitialOriginY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInitialOriginY: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InitialVelocityX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInitialVelocityX: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InitialVelocityY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInitialVelocityY: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InitialAngularVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInitialAngularVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InitialExpansionVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInitialExpansionVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InitialRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInitialRadius: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub BoundaryLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetBoundaryLeft: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub BoundaryTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetBoundaryTop: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub BoundaryRight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetBoundaryRight: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub BoundaryBottom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetBoundaryBottom: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub ElasticMarginLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetElasticMarginLeft: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub ElasticMarginTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetElasticMarginTop: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub ElasticMarginRight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetElasticMarginRight: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub ElasticMarginBottom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetElasticMarginBottom: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub DesiredDisplacement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDesiredDisplacement: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub DesiredRotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDesiredRotation: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub DesiredExpansion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDesiredExpansion: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub DesiredDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub DesiredAngularDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDesiredAngularDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub DesiredExpansionDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDesiredExpansionDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InitialTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetInitialTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Process: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ProcessTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CompleteTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationProcessor, IManipulationProcessor_Vtbl, 0xa22ac519_8300_48a0_bef4_f1be8737dba4);
impl core::ops::Deref for IManipulationProcessor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IManipulationProcessor, windows_core::IUnknown);
impl IManipulationProcessor {
    pub unsafe fn SupportedManipulations(&self) -> windows_core::Result<MANIPULATION_PROCESSOR_MANIPULATIONS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedManipulations)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSupportedManipulations(&self, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSupportedManipulations)(windows_core::Interface::as_raw(self), manipulations).ok()
    }
    pub unsafe fn PivotPointX(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PivotPointX)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPivotPointX(&self, pivotpointx: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPivotPointX)(windows_core::Interface::as_raw(self), pivotpointx).ok()
    }
    pub unsafe fn PivotPointY(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PivotPointY)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPivotPointY(&self, pivotpointy: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPivotPointY)(windows_core::Interface::as_raw(self), pivotpointy).ok()
    }
    pub unsafe fn PivotRadius(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PivotRadius)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPivotRadius(&self, pivotradius: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPivotRadius)(windows_core::Interface::as_raw(self), pivotradius).ok()
    }
    pub unsafe fn CompleteManipulation(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CompleteManipulation)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ProcessDown(&self, manipulatorid: u32, x: f32, y: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProcessDown)(windows_core::Interface::as_raw(self), manipulatorid, x, y).ok()
    }
    pub unsafe fn ProcessMove(&self, manipulatorid: u32, x: f32, y: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProcessMove)(windows_core::Interface::as_raw(self), manipulatorid, x, y).ok()
    }
    pub unsafe fn ProcessUp(&self, manipulatorid: u32, x: f32, y: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProcessUp)(windows_core::Interface::as_raw(self), manipulatorid, x, y).ok()
    }
    pub unsafe fn ProcessDownWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProcessDownWithTime)(windows_core::Interface::as_raw(self), manipulatorid, x, y, timestamp).ok()
    }
    pub unsafe fn ProcessMoveWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProcessMoveWithTime)(windows_core::Interface::as_raw(self), manipulatorid, x, y, timestamp).ok()
    }
    pub unsafe fn ProcessUpWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProcessUpWithTime)(windows_core::Interface::as_raw(self), manipulatorid, x, y, timestamp).ok()
    }
    pub unsafe fn GetVelocityX(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVelocityX)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetVelocityY(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVelocityY)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetExpansionVelocity(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetExpansionVelocity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAngularVelocity(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAngularVelocity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MinimumScaleRotateRadius(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MinimumScaleRotateRadius)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMinimumScaleRotateRadius(&self, minradius: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinimumScaleRotateRadius)(windows_core::Interface::as_raw(self), minradius).ok()
    }
}
#[repr(C)]
pub struct IManipulationProcessor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SupportedManipulations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MANIPULATION_PROCESSOR_MANIPULATIONS) -> windows_core::HRESULT,
    pub SetSupportedManipulations: unsafe extern "system" fn(*mut core::ffi::c_void, MANIPULATION_PROCESSOR_MANIPULATIONS) -> windows_core::HRESULT,
    pub PivotPointX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPivotPointX: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub PivotPointY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPivotPointY: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub PivotRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPivotRadius: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub CompleteManipulation: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessDown: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32, f32) -> windows_core::HRESULT,
    pub ProcessMove: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32, f32) -> windows_core::HRESULT,
    pub ProcessUp: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32, f32) -> windows_core::HRESULT,
    pub ProcessDownWithTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32, f32, u32) -> windows_core::HRESULT,
    pub ProcessMoveWithTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32, f32, u32) -> windows_core::HRESULT,
    pub ProcessUpWithTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32, f32, u32) -> windows_core::HRESULT,
    pub GetVelocityX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetVelocityY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetExpansionVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetAngularVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub MinimumScaleRotateRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetMinimumScaleRotateRadius: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(_IManipulationEvents, _IManipulationEvents_Vtbl, 0x4f62c8da_9c53_4b22_93df_927a862bbb03);
impl core::ops::Deref for _IManipulationEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(_IManipulationEvents, windows_core::IUnknown);
impl _IManipulationEvents {
    pub unsafe fn ManipulationStarted(&self, x: f32, y: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ManipulationStarted)(windows_core::Interface::as_raw(self), x, y).ok()
    }
    pub unsafe fn ManipulationDelta(&self, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ManipulationDelta)(windows_core::Interface::as_raw(self), x, y, translationdeltax, translationdeltay, scaledelta, expansiondelta, rotationdelta, cumulativetranslationx, cumulativetranslationy, cumulativescale, cumulativeexpansion, cumulativerotation).ok()
    }
    pub unsafe fn ManipulationCompleted(&self, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ManipulationCompleted)(windows_core::Interface::as_raw(self), x, y, cumulativetranslationx, cumulativetranslationy, cumulativescale, cumulativeexpansion, cumulativerotation).ok()
    }
}
#[repr(C)]
pub struct _IManipulationEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ManipulationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32) -> windows_core::HRESULT,
    pub ManipulationDelta: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) -> windows_core::HRESULT,
    pub ManipulationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32, f32, f32, f32) -> windows_core::HRESULT,
}
pub const GID_BEGIN: GESTURECONFIG_ID = GESTURECONFIG_ID(1u32);
pub const GID_END: GESTURECONFIG_ID = GESTURECONFIG_ID(2u32);
pub const GID_PAN: GESTURECONFIG_ID = GESTURECONFIG_ID(4u32);
pub const GID_PRESSANDTAP: GESTURECONFIG_ID = GESTURECONFIG_ID(7u32);
pub const GID_ROLLOVER: GESTURECONFIG_ID = GESTURECONFIG_ID(7u32);
pub const GID_ROTATE: GESTURECONFIG_ID = GESTURECONFIG_ID(5u32);
pub const GID_TWOFINGERTAP: GESTURECONFIG_ID = GESTURECONFIG_ID(6u32);
pub const GID_ZOOM: GESTURECONFIG_ID = GESTURECONFIG_ID(3u32);
pub const MANIPULATION_ALL: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(15i32);
pub const MANIPULATION_NONE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(0i32);
pub const MANIPULATION_ROTATE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(8i32);
pub const MANIPULATION_SCALE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(4i32);
pub const MANIPULATION_TRANSLATE_X: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(1i32);
pub const MANIPULATION_TRANSLATE_Y: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(2i32);
pub const TOUCHEVENTF_DOWN: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(2u32);
pub const TOUCHEVENTF_INRANGE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(8u32);
pub const TOUCHEVENTF_MOVE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(1u32);
pub const TOUCHEVENTF_NOCOALESCE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(32u32);
pub const TOUCHEVENTF_PALM: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(128u32);
pub const TOUCHEVENTF_PEN: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(64u32);
pub const TOUCHEVENTF_PRIMARY: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(16u32);
pub const TOUCHEVENTF_UP: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(4u32);
pub const TOUCHINPUTMASKF_CONTACTAREA: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(4u32);
pub const TOUCHINPUTMASKF_EXTRAINFO: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(2u32);
pub const TOUCHINPUTMASKF_TIMEFROMSYSTEM: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(1u32);
pub const TWF_FINETOUCH: REGISTER_TOUCH_WINDOW_FLAGS = REGISTER_TOUCH_WINDOW_FLAGS(1u32);
pub const TWF_WANTPALM: REGISTER_TOUCH_WINDOW_FLAGS = REGISTER_TOUCH_WINDOW_FLAGS(2u32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GESTURECONFIG_ID(pub u32);
impl windows_core::TypeKind for GESTURECONFIG_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GESTURECONFIG_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GESTURECONFIG_ID").field(&self.0).finish()
    }
}
impl GESTURECONFIG_ID {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GESTURECONFIG_ID {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GESTURECONFIG_ID {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GESTURECONFIG_ID {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GESTURECONFIG_ID {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GESTURECONFIG_ID {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MANIPULATION_PROCESSOR_MANIPULATIONS(pub i32);
impl windows_core::TypeKind for MANIPULATION_PROCESSOR_MANIPULATIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MANIPULATION_PROCESSOR_MANIPULATIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MANIPULATION_PROCESSOR_MANIPULATIONS").field(&self.0).finish()
    }
}
impl MANIPULATION_PROCESSOR_MANIPULATIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MANIPULATION_PROCESSOR_MANIPULATIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MANIPULATION_PROCESSOR_MANIPULATIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MANIPULATION_PROCESSOR_MANIPULATIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MANIPULATION_PROCESSOR_MANIPULATIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MANIPULATION_PROCESSOR_MANIPULATIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REGISTER_TOUCH_WINDOW_FLAGS(pub u32);
impl windows_core::TypeKind for REGISTER_TOUCH_WINDOW_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REGISTER_TOUCH_WINDOW_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REGISTER_TOUCH_WINDOW_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TOUCHEVENTF_FLAGS(pub u32);
impl windows_core::TypeKind for TOUCHEVENTF_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TOUCHEVENTF_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TOUCHEVENTF_FLAGS").field(&self.0).finish()
    }
}
impl TOUCHEVENTF_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TOUCHEVENTF_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TOUCHEVENTF_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TOUCHEVENTF_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TOUCHEVENTF_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TOUCHEVENTF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TOUCHINPUTMASKF_MASK(pub u32);
impl windows_core::TypeKind for TOUCHINPUTMASKF_MASK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TOUCHINPUTMASKF_MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TOUCHINPUTMASKF_MASK").field(&self.0).finish()
    }
}
impl TOUCHINPUTMASKF_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TOUCHINPUTMASKF_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TOUCHINPUTMASKF_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TOUCHINPUTMASKF_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TOUCHINPUTMASKF_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TOUCHINPUTMASKF_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GESTURECONFIG {
    pub dwID: GESTURECONFIG_ID,
    pub dwWant: u32,
    pub dwBlock: u32,
}
impl windows_core::TypeKind for GESTURECONFIG {
    type TypeKind = windows_core::CopyType;
}
impl Default for GESTURECONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GESTUREINFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwID: u32,
    pub hwndTarget: super::super::super::Foundation::HWND,
    pub ptsLocation: super::super::super::Foundation::POINTS,
    pub dwInstanceID: u32,
    pub dwSequenceID: u32,
    pub ullArguments: u64,
    pub cbExtraArgs: u32,
}
impl windows_core::TypeKind for GESTUREINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for GESTUREINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GESTURENOTIFYSTRUCT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub hwndTarget: super::super::super::Foundation::HWND,
    pub ptsLocation: super::super::super::Foundation::POINTS,
    pub dwInstanceID: u32,
}
impl windows_core::TypeKind for GESTURENOTIFYSTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for GESTURENOTIFYSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HGESTUREINFO(pub *mut core::ffi::c_void);
impl HGESTUREINFO {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HGESTUREINFO {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = CloseGestureInfoHandle(*self);
        }
    }
}
impl Default for HGESTUREINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HGESTUREINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HTOUCHINPUT(pub *mut core::ffi::c_void);
impl HTOUCHINPUT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HTOUCHINPUT {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = CloseTouchInputHandle(*self);
        }
    }
}
impl Default for HTOUCHINPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HTOUCHINPUT {
    type TypeKind = windows_core::CopyType;
}
pub const InertiaProcessor: windows_core::GUID = windows_core::GUID::from_u128(0xabb27087_4ce0_4e58_a0cb_e24df96814be);
pub const ManipulationProcessor: windows_core::GUID = windows_core::GUID::from_u128(0x597d4fb0_47fd_4aff_89b9_c6cfae8cf08e);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOUCHINPUT {
    pub x: i32,
    pub y: i32,
    pub hSource: super::super::super::Foundation::HANDLE,
    pub dwID: u32,
    pub dwFlags: TOUCHEVENTF_FLAGS,
    pub dwMask: TOUCHINPUTMASKF_MASK,
    pub dwTime: u32,
    pub dwExtraInfo: usize,
    pub cxContact: u32,
    pub cyContact: u32,
}
impl windows_core::TypeKind for TOUCHINPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOUCHINPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
