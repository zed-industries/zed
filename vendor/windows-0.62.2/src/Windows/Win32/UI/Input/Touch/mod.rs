#[inline]
pub unsafe fn CloseGestureInfoHandle(hgestureinfo: HGESTUREINFO) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn CloseGestureInfoHandle(hgestureinfo : HGESTUREINFO) -> windows_core::BOOL);
    unsafe { CloseGestureInfoHandle(hgestureinfo).ok() }
}
#[inline]
pub unsafe fn CloseTouchInputHandle(htouchinput: HTOUCHINPUT) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn CloseTouchInputHandle(htouchinput : HTOUCHINPUT) -> windows_core::BOOL);
    unsafe { CloseTouchInputHandle(htouchinput).ok() }
}
#[inline]
pub unsafe fn GetGestureConfig(hwnd: super::super::super::Foundation::HWND, dwreserved: u32, dwflags: u32, pcids: *const u32, pgestureconfig: *mut GESTURECONFIG, cbsize: u32) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn GetGestureConfig(hwnd : super::super::super::Foundation:: HWND, dwreserved : u32, dwflags : u32, pcids : *const u32, pgestureconfig : *mut GESTURECONFIG, cbsize : u32) -> windows_core::BOOL);
    unsafe { GetGestureConfig(hwnd, dwreserved, dwflags, pcids, pgestureconfig as _, cbsize).ok() }
}
#[inline]
pub unsafe fn GetGestureExtraArgs(hgestureinfo: HGESTUREINFO, pextraargs: &mut [u8]) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn GetGestureExtraArgs(hgestureinfo : HGESTUREINFO, cbextraargs : u32, pextraargs : *mut u8) -> windows_core::BOOL);
    unsafe { GetGestureExtraArgs(hgestureinfo, pextraargs.len().try_into().unwrap(), core::mem::transmute(pextraargs.as_ptr())).ok() }
}
#[inline]
pub unsafe fn GetGestureInfo(hgestureinfo: HGESTUREINFO, pgestureinfo: *mut GESTUREINFO) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn GetGestureInfo(hgestureinfo : HGESTUREINFO, pgestureinfo : *mut GESTUREINFO) -> windows_core::BOOL);
    unsafe { GetGestureInfo(hgestureinfo, pgestureinfo as _).ok() }
}
#[inline]
pub unsafe fn GetTouchInputInfo(htouchinput: HTOUCHINPUT, pinputs: &mut [TOUCHINPUT], cbsize: i32) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn GetTouchInputInfo(htouchinput : HTOUCHINPUT, cinputs : u32, pinputs : *mut TOUCHINPUT, cbsize : i32) -> windows_core::BOOL);
    unsafe { GetTouchInputInfo(htouchinput, pinputs.len().try_into().unwrap(), core::mem::transmute(pinputs.as_ptr()), cbsize).ok() }
}
#[inline]
pub unsafe fn IsTouchWindow(hwnd: super::super::super::Foundation::HWND, pulflags: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsTouchWindow(hwnd : super::super::super::Foundation:: HWND, pulflags : *mut u32) -> windows_core::BOOL);
    unsafe { IsTouchWindow(hwnd, pulflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RegisterTouchWindow(hwnd: super::super::super::Foundation::HWND, ulflags: REGISTER_TOUCH_WINDOW_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn RegisterTouchWindow(hwnd : super::super::super::Foundation:: HWND, ulflags : REGISTER_TOUCH_WINDOW_FLAGS) -> windows_core::BOOL);
    unsafe { RegisterTouchWindow(hwnd, ulflags).ok() }
}
#[inline]
pub unsafe fn SetGestureConfig(hwnd: super::super::super::Foundation::HWND, dwreserved: u32, pgestureconfig: &[GESTURECONFIG], cbsize: u32) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn SetGestureConfig(hwnd : super::super::super::Foundation:: HWND, dwreserved : u32, cids : u32, pgestureconfig : *const GESTURECONFIG, cbsize : u32) -> windows_core::BOOL);
    unsafe { SetGestureConfig(hwnd, dwreserved, pgestureconfig.len().try_into().unwrap(), core::mem::transmute(pgestureconfig.as_ptr()), cbsize).ok() }
}
#[inline]
pub unsafe fn UnregisterTouchWindow(hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn UnregisterTouchWindow(hwnd : super::super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { UnregisterTouchWindow(hwnd).ok() }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GESTURECONFIG {
    pub dwID: GESTURECONFIG_ID,
    pub dwWant: u32,
    pub dwBlock: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GESTURECONFIG_ID(pub u32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GESTURENOTIFYSTRUCT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub hwndTarget: super::super::super::Foundation::HWND,
    pub ptsLocation: super::super::super::Foundation::POINTS,
    pub dwInstanceID: u32,
}
pub const GID_BEGIN: GESTURECONFIG_ID = GESTURECONFIG_ID(1u32);
pub const GID_END: GESTURECONFIG_ID = GESTURECONFIG_ID(2u32);
pub const GID_PAN: GESTURECONFIG_ID = GESTURECONFIG_ID(4u32);
pub const GID_PRESSANDTAP: GESTURECONFIG_ID = GESTURECONFIG_ID(7u32);
pub const GID_ROLLOVER: GESTURECONFIG_ID = GESTURECONFIG_ID(7u32);
pub const GID_ROTATE: GESTURECONFIG_ID = GESTURECONFIG_ID(5u32);
pub const GID_TWOFINGERTAP: GESTURECONFIG_ID = GESTURECONFIG_ID(6u32);
pub const GID_ZOOM: GESTURECONFIG_ID = GESTURECONFIG_ID(3u32);
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
            windows_core::link!("user32.dll" "system" fn CloseGestureInfoHandle(hgestureinfo : *mut core::ffi::c_void) -> i32);
            unsafe {
                CloseGestureInfoHandle(self.0);
            }
        }
    }
}
impl Default for HGESTUREINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
            windows_core::link!("user32.dll" "system" fn CloseTouchInputHandle(htouchinput : *mut core::ffi::c_void) -> i32);
            unsafe {
                CloseTouchInputHandle(self.0);
            }
        }
    }
}
impl Default for HTOUCHINPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(IInertiaProcessor, IInertiaProcessor_Vtbl, 0x18b00c6d_c5ee_41b1_90a9_9d4a929095ad);
windows_core::imp::interface_hierarchy!(IInertiaProcessor, windows_core::IUnknown);
impl IInertiaProcessor {
    pub unsafe fn InitialOriginX(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitialOriginX)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInitialOriginX(&self, x: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInitialOriginX)(windows_core::Interface::as_raw(self), x).ok() }
    }
    pub unsafe fn InitialOriginY(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitialOriginY)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInitialOriginY(&self, y: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInitialOriginY)(windows_core::Interface::as_raw(self), y).ok() }
    }
    pub unsafe fn InitialVelocityX(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitialVelocityX)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInitialVelocityX(&self, x: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInitialVelocityX)(windows_core::Interface::as_raw(self), x).ok() }
    }
    pub unsafe fn InitialVelocityY(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitialVelocityY)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInitialVelocityY(&self, y: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInitialVelocityY)(windows_core::Interface::as_raw(self), y).ok() }
    }
    pub unsafe fn InitialAngularVelocity(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitialAngularVelocity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInitialAngularVelocity(&self, velocity: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInitialAngularVelocity)(windows_core::Interface::as_raw(self), velocity).ok() }
    }
    pub unsafe fn InitialExpansionVelocity(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitialExpansionVelocity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInitialExpansionVelocity(&self, velocity: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInitialExpansionVelocity)(windows_core::Interface::as_raw(self), velocity).ok() }
    }
    pub unsafe fn InitialRadius(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitialRadius)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInitialRadius(&self, radius: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInitialRadius)(windows_core::Interface::as_raw(self), radius).ok() }
    }
    pub unsafe fn BoundaryLeft(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BoundaryLeft)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBoundaryLeft(&self, left: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBoundaryLeft)(windows_core::Interface::as_raw(self), left).ok() }
    }
    pub unsafe fn BoundaryTop(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BoundaryTop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBoundaryTop(&self, top: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBoundaryTop)(windows_core::Interface::as_raw(self), top).ok() }
    }
    pub unsafe fn BoundaryRight(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BoundaryRight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBoundaryRight(&self, right: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBoundaryRight)(windows_core::Interface::as_raw(self), right).ok() }
    }
    pub unsafe fn BoundaryBottom(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BoundaryBottom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBoundaryBottom(&self, bottom: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBoundaryBottom)(windows_core::Interface::as_raw(self), bottom).ok() }
    }
    pub unsafe fn ElasticMarginLeft(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElasticMarginLeft)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetElasticMarginLeft(&self, left: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetElasticMarginLeft)(windows_core::Interface::as_raw(self), left).ok() }
    }
    pub unsafe fn ElasticMarginTop(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElasticMarginTop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetElasticMarginTop(&self, top: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetElasticMarginTop)(windows_core::Interface::as_raw(self), top).ok() }
    }
    pub unsafe fn ElasticMarginRight(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElasticMarginRight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetElasticMarginRight(&self, right: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetElasticMarginRight)(windows_core::Interface::as_raw(self), right).ok() }
    }
    pub unsafe fn ElasticMarginBottom(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElasticMarginBottom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetElasticMarginBottom(&self, bottom: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetElasticMarginBottom)(windows_core::Interface::as_raw(self), bottom).ok() }
    }
    pub unsafe fn DesiredDisplacement(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DesiredDisplacement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDesiredDisplacement(&self, displacement: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDesiredDisplacement)(windows_core::Interface::as_raw(self), displacement).ok() }
    }
    pub unsafe fn DesiredRotation(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DesiredRotation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDesiredRotation(&self, rotation: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDesiredRotation)(windows_core::Interface::as_raw(self), rotation).ok() }
    }
    pub unsafe fn DesiredExpansion(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DesiredExpansion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDesiredExpansion(&self, expansion: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDesiredExpansion)(windows_core::Interface::as_raw(self), expansion).ok() }
    }
    pub unsafe fn DesiredDeceleration(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DesiredDeceleration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDesiredDeceleration(&self, deceleration: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDesiredDeceleration)(windows_core::Interface::as_raw(self), deceleration).ok() }
    }
    pub unsafe fn DesiredAngularDeceleration(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DesiredAngularDeceleration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDesiredAngularDeceleration(&self, deceleration: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDesiredAngularDeceleration)(windows_core::Interface::as_raw(self), deceleration).ok() }
    }
    pub unsafe fn DesiredExpansionDeceleration(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DesiredExpansionDeceleration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDesiredExpansionDeceleration(&self, deceleration: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDesiredExpansionDeceleration)(windows_core::Interface::as_raw(self), deceleration).ok() }
    }
    pub unsafe fn InitialTimestamp(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitialTimestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInitialTimestamp(&self, timestamp: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInitialTimestamp)(windows_core::Interface::as_raw(self), timestamp).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Process(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Process)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProcessTime(&self, timestamp: u32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProcessTime)(windows_core::Interface::as_raw(self), timestamp, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Complete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Complete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CompleteTime(&self, timestamp: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CompleteTime)(windows_core::Interface::as_raw(self), timestamp).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub Process: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub ProcessTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CompleteTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IInertiaProcessor_Impl: windows_core::IUnknownImpl {
    fn InitialOriginX(&self) -> windows_core::Result<f32>;
    fn SetInitialOriginX(&self, x: f32) -> windows_core::Result<()>;
    fn InitialOriginY(&self) -> windows_core::Result<f32>;
    fn SetInitialOriginY(&self, y: f32) -> windows_core::Result<()>;
    fn InitialVelocityX(&self) -> windows_core::Result<f32>;
    fn SetInitialVelocityX(&self, x: f32) -> windows_core::Result<()>;
    fn InitialVelocityY(&self) -> windows_core::Result<f32>;
    fn SetInitialVelocityY(&self, y: f32) -> windows_core::Result<()>;
    fn InitialAngularVelocity(&self) -> windows_core::Result<f32>;
    fn SetInitialAngularVelocity(&self, velocity: f32) -> windows_core::Result<()>;
    fn InitialExpansionVelocity(&self) -> windows_core::Result<f32>;
    fn SetInitialExpansionVelocity(&self, velocity: f32) -> windows_core::Result<()>;
    fn InitialRadius(&self) -> windows_core::Result<f32>;
    fn SetInitialRadius(&self, radius: f32) -> windows_core::Result<()>;
    fn BoundaryLeft(&self) -> windows_core::Result<f32>;
    fn SetBoundaryLeft(&self, left: f32) -> windows_core::Result<()>;
    fn BoundaryTop(&self) -> windows_core::Result<f32>;
    fn SetBoundaryTop(&self, top: f32) -> windows_core::Result<()>;
    fn BoundaryRight(&self) -> windows_core::Result<f32>;
    fn SetBoundaryRight(&self, right: f32) -> windows_core::Result<()>;
    fn BoundaryBottom(&self) -> windows_core::Result<f32>;
    fn SetBoundaryBottom(&self, bottom: f32) -> windows_core::Result<()>;
    fn ElasticMarginLeft(&self) -> windows_core::Result<f32>;
    fn SetElasticMarginLeft(&self, left: f32) -> windows_core::Result<()>;
    fn ElasticMarginTop(&self) -> windows_core::Result<f32>;
    fn SetElasticMarginTop(&self, top: f32) -> windows_core::Result<()>;
    fn ElasticMarginRight(&self) -> windows_core::Result<f32>;
    fn SetElasticMarginRight(&self, right: f32) -> windows_core::Result<()>;
    fn ElasticMarginBottom(&self) -> windows_core::Result<f32>;
    fn SetElasticMarginBottom(&self, bottom: f32) -> windows_core::Result<()>;
    fn DesiredDisplacement(&self) -> windows_core::Result<f32>;
    fn SetDesiredDisplacement(&self, displacement: f32) -> windows_core::Result<()>;
    fn DesiredRotation(&self) -> windows_core::Result<f32>;
    fn SetDesiredRotation(&self, rotation: f32) -> windows_core::Result<()>;
    fn DesiredExpansion(&self) -> windows_core::Result<f32>;
    fn SetDesiredExpansion(&self, expansion: f32) -> windows_core::Result<()>;
    fn DesiredDeceleration(&self) -> windows_core::Result<f32>;
    fn SetDesiredDeceleration(&self, deceleration: f32) -> windows_core::Result<()>;
    fn DesiredAngularDeceleration(&self) -> windows_core::Result<f32>;
    fn SetDesiredAngularDeceleration(&self, deceleration: f32) -> windows_core::Result<()>;
    fn DesiredExpansionDeceleration(&self) -> windows_core::Result<f32>;
    fn SetDesiredExpansionDeceleration(&self, deceleration: f32) -> windows_core::Result<()>;
    fn InitialTimestamp(&self) -> windows_core::Result<u32>;
    fn SetInitialTimestamp(&self, timestamp: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Process(&self) -> windows_core::Result<windows_core::BOOL>;
    fn ProcessTime(&self, timestamp: u32) -> windows_core::Result<windows_core::BOOL>;
    fn Complete(&self) -> windows_core::Result<()>;
    fn CompleteTime(&self, timestamp: u32) -> windows_core::Result<()>;
}
impl IInertiaProcessor_Vtbl {
    pub const fn new<Identity: IInertiaProcessor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitialOriginX<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::InitialOriginX(this) {
                    Ok(ok__) => {
                        x.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInitialOriginX<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetInitialOriginX(this, core::mem::transmute_copy(&x)).into()
            }
        }
        unsafe extern "system" fn InitialOriginY<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, y: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::InitialOriginY(this) {
                    Ok(ok__) => {
                        y.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInitialOriginY<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, y: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetInitialOriginY(this, core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn InitialVelocityX<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::InitialVelocityX(this) {
                    Ok(ok__) => {
                        x.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInitialVelocityX<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetInitialVelocityX(this, core::mem::transmute_copy(&x)).into()
            }
        }
        unsafe extern "system" fn InitialVelocityY<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, y: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::InitialVelocityY(this) {
                    Ok(ok__) => {
                        y.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInitialVelocityY<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, y: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetInitialVelocityY(this, core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn InitialAngularVelocity<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::InitialAngularVelocity(this) {
                    Ok(ok__) => {
                        velocity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInitialAngularVelocity<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetInitialAngularVelocity(this, core::mem::transmute_copy(&velocity)).into()
            }
        }
        unsafe extern "system" fn InitialExpansionVelocity<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::InitialExpansionVelocity(this) {
                    Ok(ok__) => {
                        velocity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInitialExpansionVelocity<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetInitialExpansionVelocity(this, core::mem::transmute_copy(&velocity)).into()
            }
        }
        unsafe extern "system" fn InitialRadius<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::InitialRadius(this) {
                    Ok(ok__) => {
                        radius.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInitialRadius<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetInitialRadius(this, core::mem::transmute_copy(&radius)).into()
            }
        }
        unsafe extern "system" fn BoundaryLeft<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::BoundaryLeft(this) {
                    Ok(ok__) => {
                        left.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBoundaryLeft<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetBoundaryLeft(this, core::mem::transmute_copy(&left)).into()
            }
        }
        unsafe extern "system" fn BoundaryTop<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::BoundaryTop(this) {
                    Ok(ok__) => {
                        top.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBoundaryTop<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetBoundaryTop(this, core::mem::transmute_copy(&top)).into()
            }
        }
        unsafe extern "system" fn BoundaryRight<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, right: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::BoundaryRight(this) {
                    Ok(ok__) => {
                        right.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBoundaryRight<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, right: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetBoundaryRight(this, core::mem::transmute_copy(&right)).into()
            }
        }
        unsafe extern "system" fn BoundaryBottom<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bottom: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::BoundaryBottom(this) {
                    Ok(ok__) => {
                        bottom.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBoundaryBottom<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bottom: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetBoundaryBottom(this, core::mem::transmute_copy(&bottom)).into()
            }
        }
        unsafe extern "system" fn ElasticMarginLeft<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::ElasticMarginLeft(this) {
                    Ok(ok__) => {
                        left.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetElasticMarginLeft<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetElasticMarginLeft(this, core::mem::transmute_copy(&left)).into()
            }
        }
        unsafe extern "system" fn ElasticMarginTop<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::ElasticMarginTop(this) {
                    Ok(ok__) => {
                        top.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetElasticMarginTop<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetElasticMarginTop(this, core::mem::transmute_copy(&top)).into()
            }
        }
        unsafe extern "system" fn ElasticMarginRight<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, right: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::ElasticMarginRight(this) {
                    Ok(ok__) => {
                        right.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetElasticMarginRight<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, right: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetElasticMarginRight(this, core::mem::transmute_copy(&right)).into()
            }
        }
        unsafe extern "system" fn ElasticMarginBottom<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bottom: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::ElasticMarginBottom(this) {
                    Ok(ok__) => {
                        bottom.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetElasticMarginBottom<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bottom: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetElasticMarginBottom(this, core::mem::transmute_copy(&bottom)).into()
            }
        }
        unsafe extern "system" fn DesiredDisplacement<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displacement: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::DesiredDisplacement(this) {
                    Ok(ok__) => {
                        displacement.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDesiredDisplacement<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displacement: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetDesiredDisplacement(this, core::mem::transmute_copy(&displacement)).into()
            }
        }
        unsafe extern "system" fn DesiredRotation<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotation: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::DesiredRotation(this) {
                    Ok(ok__) => {
                        rotation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDesiredRotation<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotation: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetDesiredRotation(this, core::mem::transmute_copy(&rotation)).into()
            }
        }
        unsafe extern "system" fn DesiredExpansion<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expansion: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::DesiredExpansion(this) {
                    Ok(ok__) => {
                        expansion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDesiredExpansion<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expansion: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetDesiredExpansion(this, core::mem::transmute_copy(&expansion)).into()
            }
        }
        unsafe extern "system" fn DesiredDeceleration<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::DesiredDeceleration(this) {
                    Ok(ok__) => {
                        deceleration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDesiredDeceleration<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetDesiredDeceleration(this, core::mem::transmute_copy(&deceleration)).into()
            }
        }
        unsafe extern "system" fn DesiredAngularDeceleration<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::DesiredAngularDeceleration(this) {
                    Ok(ok__) => {
                        deceleration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDesiredAngularDeceleration<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetDesiredAngularDeceleration(this, core::mem::transmute_copy(&deceleration)).into()
            }
        }
        unsafe extern "system" fn DesiredExpansionDeceleration<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::DesiredExpansionDeceleration(this) {
                    Ok(ok__) => {
                        deceleration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDesiredExpansionDeceleration<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetDesiredExpansionDeceleration(this, core::mem::transmute_copy(&deceleration)).into()
            }
        }
        unsafe extern "system" fn InitialTimestamp<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::InitialTimestamp(this) {
                    Ok(ok__) => {
                        timestamp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInitialTimestamp<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::SetInitialTimestamp(this, core::mem::transmute_copy(&timestamp)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Process<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, completed: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::Process(this) {
                    Ok(ok__) => {
                        completed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProcessTime<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u32, completed: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::ProcessTime(this, core::mem::transmute_copy(&timestamp)) {
                    Ok(ok__) => {
                        completed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Complete<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::Complete(this).into()
            }
        }
        unsafe extern "system" fn CompleteTime<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::CompleteTime(this, core::mem::transmute_copy(&timestamp)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitialOriginX: InitialOriginX::<Identity, OFFSET>,
            SetInitialOriginX: SetInitialOriginX::<Identity, OFFSET>,
            InitialOriginY: InitialOriginY::<Identity, OFFSET>,
            SetInitialOriginY: SetInitialOriginY::<Identity, OFFSET>,
            InitialVelocityX: InitialVelocityX::<Identity, OFFSET>,
            SetInitialVelocityX: SetInitialVelocityX::<Identity, OFFSET>,
            InitialVelocityY: InitialVelocityY::<Identity, OFFSET>,
            SetInitialVelocityY: SetInitialVelocityY::<Identity, OFFSET>,
            InitialAngularVelocity: InitialAngularVelocity::<Identity, OFFSET>,
            SetInitialAngularVelocity: SetInitialAngularVelocity::<Identity, OFFSET>,
            InitialExpansionVelocity: InitialExpansionVelocity::<Identity, OFFSET>,
            SetInitialExpansionVelocity: SetInitialExpansionVelocity::<Identity, OFFSET>,
            InitialRadius: InitialRadius::<Identity, OFFSET>,
            SetInitialRadius: SetInitialRadius::<Identity, OFFSET>,
            BoundaryLeft: BoundaryLeft::<Identity, OFFSET>,
            SetBoundaryLeft: SetBoundaryLeft::<Identity, OFFSET>,
            BoundaryTop: BoundaryTop::<Identity, OFFSET>,
            SetBoundaryTop: SetBoundaryTop::<Identity, OFFSET>,
            BoundaryRight: BoundaryRight::<Identity, OFFSET>,
            SetBoundaryRight: SetBoundaryRight::<Identity, OFFSET>,
            BoundaryBottom: BoundaryBottom::<Identity, OFFSET>,
            SetBoundaryBottom: SetBoundaryBottom::<Identity, OFFSET>,
            ElasticMarginLeft: ElasticMarginLeft::<Identity, OFFSET>,
            SetElasticMarginLeft: SetElasticMarginLeft::<Identity, OFFSET>,
            ElasticMarginTop: ElasticMarginTop::<Identity, OFFSET>,
            SetElasticMarginTop: SetElasticMarginTop::<Identity, OFFSET>,
            ElasticMarginRight: ElasticMarginRight::<Identity, OFFSET>,
            SetElasticMarginRight: SetElasticMarginRight::<Identity, OFFSET>,
            ElasticMarginBottom: ElasticMarginBottom::<Identity, OFFSET>,
            SetElasticMarginBottom: SetElasticMarginBottom::<Identity, OFFSET>,
            DesiredDisplacement: DesiredDisplacement::<Identity, OFFSET>,
            SetDesiredDisplacement: SetDesiredDisplacement::<Identity, OFFSET>,
            DesiredRotation: DesiredRotation::<Identity, OFFSET>,
            SetDesiredRotation: SetDesiredRotation::<Identity, OFFSET>,
            DesiredExpansion: DesiredExpansion::<Identity, OFFSET>,
            SetDesiredExpansion: SetDesiredExpansion::<Identity, OFFSET>,
            DesiredDeceleration: DesiredDeceleration::<Identity, OFFSET>,
            SetDesiredDeceleration: SetDesiredDeceleration::<Identity, OFFSET>,
            DesiredAngularDeceleration: DesiredAngularDeceleration::<Identity, OFFSET>,
            SetDesiredAngularDeceleration: SetDesiredAngularDeceleration::<Identity, OFFSET>,
            DesiredExpansionDeceleration: DesiredExpansionDeceleration::<Identity, OFFSET>,
            SetDesiredExpansionDeceleration: SetDesiredExpansionDeceleration::<Identity, OFFSET>,
            InitialTimestamp: InitialTimestamp::<Identity, OFFSET>,
            SetInitialTimestamp: SetInitialTimestamp::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Process: Process::<Identity, OFFSET>,
            ProcessTime: ProcessTime::<Identity, OFFSET>,
            Complete: Complete::<Identity, OFFSET>,
            CompleteTime: CompleteTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInertiaProcessor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInertiaProcessor {}
windows_core::imp::define_interface!(IManipulationProcessor, IManipulationProcessor_Vtbl, 0xa22ac519_8300_48a0_bef4_f1be8737dba4);
windows_core::imp::interface_hierarchy!(IManipulationProcessor, windows_core::IUnknown);
impl IManipulationProcessor {
    pub unsafe fn SupportedManipulations(&self) -> windows_core::Result<MANIPULATION_PROCESSOR_MANIPULATIONS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedManipulations)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSupportedManipulations(&self, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSupportedManipulations)(windows_core::Interface::as_raw(self), manipulations).ok() }
    }
    pub unsafe fn PivotPointX(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PivotPointX)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPivotPointX(&self, pivotpointx: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPivotPointX)(windows_core::Interface::as_raw(self), pivotpointx).ok() }
    }
    pub unsafe fn PivotPointY(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PivotPointY)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPivotPointY(&self, pivotpointy: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPivotPointY)(windows_core::Interface::as_raw(self), pivotpointy).ok() }
    }
    pub unsafe fn PivotRadius(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PivotRadius)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPivotRadius(&self, pivotradius: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPivotRadius)(windows_core::Interface::as_raw(self), pivotradius).ok() }
    }
    pub unsafe fn CompleteManipulation(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CompleteManipulation)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ProcessDown(&self, manipulatorid: u32, x: f32, y: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProcessDown)(windows_core::Interface::as_raw(self), manipulatorid, x, y).ok() }
    }
    pub unsafe fn ProcessMove(&self, manipulatorid: u32, x: f32, y: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProcessMove)(windows_core::Interface::as_raw(self), manipulatorid, x, y).ok() }
    }
    pub unsafe fn ProcessUp(&self, manipulatorid: u32, x: f32, y: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProcessUp)(windows_core::Interface::as_raw(self), manipulatorid, x, y).ok() }
    }
    pub unsafe fn ProcessDownWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProcessDownWithTime)(windows_core::Interface::as_raw(self), manipulatorid, x, y, timestamp).ok() }
    }
    pub unsafe fn ProcessMoveWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProcessMoveWithTime)(windows_core::Interface::as_raw(self), manipulatorid, x, y, timestamp).ok() }
    }
    pub unsafe fn ProcessUpWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProcessUpWithTime)(windows_core::Interface::as_raw(self), manipulatorid, x, y, timestamp).ok() }
    }
    pub unsafe fn GetVelocityX(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVelocityX)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVelocityY(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVelocityY)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetExpansionVelocity(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExpansionVelocity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAngularVelocity(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAngularVelocity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MinimumScaleRotateRadius(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinimumScaleRotateRadius)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMinimumScaleRotateRadius(&self, minradius: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMinimumScaleRotateRadius)(windows_core::Interface::as_raw(self), minradius).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IManipulationProcessor_Impl: windows_core::IUnknownImpl {
    fn SupportedManipulations(&self) -> windows_core::Result<MANIPULATION_PROCESSOR_MANIPULATIONS>;
    fn SetSupportedManipulations(&self, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> windows_core::Result<()>;
    fn PivotPointX(&self) -> windows_core::Result<f32>;
    fn SetPivotPointX(&self, pivotpointx: f32) -> windows_core::Result<()>;
    fn PivotPointY(&self) -> windows_core::Result<f32>;
    fn SetPivotPointY(&self, pivotpointy: f32) -> windows_core::Result<()>;
    fn PivotRadius(&self) -> windows_core::Result<f32>;
    fn SetPivotRadius(&self, pivotradius: f32) -> windows_core::Result<()>;
    fn CompleteManipulation(&self) -> windows_core::Result<()>;
    fn ProcessDown(&self, manipulatorid: u32, x: f32, y: f32) -> windows_core::Result<()>;
    fn ProcessMove(&self, manipulatorid: u32, x: f32, y: f32) -> windows_core::Result<()>;
    fn ProcessUp(&self, manipulatorid: u32, x: f32, y: f32) -> windows_core::Result<()>;
    fn ProcessDownWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::Result<()>;
    fn ProcessMoveWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::Result<()>;
    fn ProcessUpWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::Result<()>;
    fn GetVelocityX(&self) -> windows_core::Result<f32>;
    fn GetVelocityY(&self) -> windows_core::Result<f32>;
    fn GetExpansionVelocity(&self) -> windows_core::Result<f32>;
    fn GetAngularVelocity(&self) -> windows_core::Result<f32>;
    fn MinimumScaleRotateRadius(&self) -> windows_core::Result<f32>;
    fn SetMinimumScaleRotateRadius(&self, minradius: f32) -> windows_core::Result<()>;
}
impl IManipulationProcessor_Vtbl {
    pub const fn new<Identity: IManipulationProcessor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SupportedManipulations<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulations: *mut MANIPULATION_PROCESSOR_MANIPULATIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::SupportedManipulations(this) {
                    Ok(ok__) => {
                        manipulations.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSupportedManipulations<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::SetSupportedManipulations(this, core::mem::transmute_copy(&manipulations)).into()
            }
        }
        unsafe extern "system" fn PivotPointX<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotpointx: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::PivotPointX(this) {
                    Ok(ok__) => {
                        pivotpointx.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPivotPointX<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotpointx: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::SetPivotPointX(this, core::mem::transmute_copy(&pivotpointx)).into()
            }
        }
        unsafe extern "system" fn PivotPointY<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotpointy: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::PivotPointY(this) {
                    Ok(ok__) => {
                        pivotpointy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPivotPointY<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotpointy: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::SetPivotPointY(this, core::mem::transmute_copy(&pivotpointy)).into()
            }
        }
        unsafe extern "system" fn PivotRadius<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotradius: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::PivotRadius(this) {
                    Ok(ok__) => {
                        pivotradius.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPivotRadius<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotradius: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::SetPivotRadius(this, core::mem::transmute_copy(&pivotradius)).into()
            }
        }
        unsafe extern "system" fn CompleteManipulation<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::CompleteManipulation(this).into()
            }
        }
        unsafe extern "system" fn ProcessDown<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::ProcessDown(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn ProcessMove<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::ProcessMove(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn ProcessUp<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::ProcessUp(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn ProcessDownWithTime<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::ProcessDownWithTime(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&timestamp)).into()
            }
        }
        unsafe extern "system" fn ProcessMoveWithTime<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::ProcessMoveWithTime(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&timestamp)).into()
            }
        }
        unsafe extern "system" fn ProcessUpWithTime<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::ProcessUpWithTime(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&timestamp)).into()
            }
        }
        unsafe extern "system" fn GetVelocityX<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocityx: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::GetVelocityX(this) {
                    Ok(ok__) => {
                        velocityx.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVelocityY<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocityy: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::GetVelocityY(this) {
                    Ok(ok__) => {
                        velocityy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExpansionVelocity<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expansionvelocity: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::GetExpansionVelocity(this) {
                    Ok(ok__) => {
                        expansionvelocity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAngularVelocity<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, angularvelocity: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::GetAngularVelocity(this) {
                    Ok(ok__) => {
                        angularvelocity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MinimumScaleRotateRadius<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minradius: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::MinimumScaleRotateRadius(this) {
                    Ok(ok__) => {
                        minradius.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinimumScaleRotateRadius<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minradius: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::SetMinimumScaleRotateRadius(this, core::mem::transmute_copy(&minradius)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SupportedManipulations: SupportedManipulations::<Identity, OFFSET>,
            SetSupportedManipulations: SetSupportedManipulations::<Identity, OFFSET>,
            PivotPointX: PivotPointX::<Identity, OFFSET>,
            SetPivotPointX: SetPivotPointX::<Identity, OFFSET>,
            PivotPointY: PivotPointY::<Identity, OFFSET>,
            SetPivotPointY: SetPivotPointY::<Identity, OFFSET>,
            PivotRadius: PivotRadius::<Identity, OFFSET>,
            SetPivotRadius: SetPivotRadius::<Identity, OFFSET>,
            CompleteManipulation: CompleteManipulation::<Identity, OFFSET>,
            ProcessDown: ProcessDown::<Identity, OFFSET>,
            ProcessMove: ProcessMove::<Identity, OFFSET>,
            ProcessUp: ProcessUp::<Identity, OFFSET>,
            ProcessDownWithTime: ProcessDownWithTime::<Identity, OFFSET>,
            ProcessMoveWithTime: ProcessMoveWithTime::<Identity, OFFSET>,
            ProcessUpWithTime: ProcessUpWithTime::<Identity, OFFSET>,
            GetVelocityX: GetVelocityX::<Identity, OFFSET>,
            GetVelocityY: GetVelocityY::<Identity, OFFSET>,
            GetExpansionVelocity: GetExpansionVelocity::<Identity, OFFSET>,
            GetAngularVelocity: GetAngularVelocity::<Identity, OFFSET>,
            MinimumScaleRotateRadius: MinimumScaleRotateRadius::<Identity, OFFSET>,
            SetMinimumScaleRotateRadius: SetMinimumScaleRotateRadius::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManipulationProcessor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IManipulationProcessor {}
pub const InertiaProcessor: windows_core::GUID = windows_core::GUID::from_u128(0xabb27087_4ce0_4e58_a0cb_e24df96814be);
pub const MANIPULATION_ALL: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(15i32);
pub const MANIPULATION_NONE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MANIPULATION_PROCESSOR_MANIPULATIONS(pub i32);
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
pub const MANIPULATION_ROTATE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(8i32);
pub const MANIPULATION_SCALE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(4i32);
pub const MANIPULATION_TRANSLATE_X: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(1i32);
pub const MANIPULATION_TRANSLATE_Y: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(2i32);
pub const ManipulationProcessor: windows_core::GUID = windows_core::GUID::from_u128(0x597d4fb0_47fd_4aff_89b9_c6cfae8cf08e);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REGISTER_TOUCH_WINDOW_FLAGS(pub u32);
pub const TOUCHEVENTF_DOWN: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOUCHEVENTF_FLAGS(pub u32);
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
pub const TOUCHEVENTF_INRANGE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(8u32);
pub const TOUCHEVENTF_MOVE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(1u32);
pub const TOUCHEVENTF_NOCOALESCE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(32u32);
pub const TOUCHEVENTF_PALM: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(128u32);
pub const TOUCHEVENTF_PEN: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(64u32);
pub const TOUCHEVENTF_PRIMARY: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(16u32);
pub const TOUCHEVENTF_UP: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const TOUCHINPUTMASKF_CONTACTAREA: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(4u32);
pub const TOUCHINPUTMASKF_EXTRAINFO: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOUCHINPUTMASKF_MASK(pub u32);
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
pub const TOUCHINPUTMASKF_TIMEFROMSYSTEM: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(1u32);
pub const TWF_FINETOUCH: REGISTER_TOUCH_WINDOW_FLAGS = REGISTER_TOUCH_WINDOW_FLAGS(1u32);
pub const TWF_WANTPALM: REGISTER_TOUCH_WINDOW_FLAGS = REGISTER_TOUCH_WINDOW_FLAGS(2u32);
windows_core::imp::define_interface!(_IManipulationEvents, _IManipulationEvents_Vtbl, 0x4f62c8da_9c53_4b22_93df_927a862bbb03);
windows_core::imp::interface_hierarchy!(_IManipulationEvents, windows_core::IUnknown);
impl _IManipulationEvents {
    pub unsafe fn ManipulationStarted(&self, x: f32, y: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ManipulationStarted)(windows_core::Interface::as_raw(self), x, y).ok() }
    }
    pub unsafe fn ManipulationDelta(&self, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ManipulationDelta)(windows_core::Interface::as_raw(self), x, y, translationdeltax, translationdeltay, scaledelta, expansiondelta, rotationdelta, cumulativetranslationx, cumulativetranslationy, cumulativescale, cumulativeexpansion, cumulativerotation).ok() }
    }
    pub unsafe fn ManipulationCompleted(&self, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ManipulationCompleted)(windows_core::Interface::as_raw(self), x, y, cumulativetranslationx, cumulativetranslationy, cumulativescale, cumulativeexpansion, cumulativerotation).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _IManipulationEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ManipulationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32) -> windows_core::HRESULT,
    pub ManipulationDelta: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) -> windows_core::HRESULT,
    pub ManipulationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32, f32, f32, f32) -> windows_core::HRESULT,
}
pub trait _IManipulationEvents_Impl: windows_core::IUnknownImpl {
    fn ManipulationStarted(&self, x: f32, y: f32) -> windows_core::Result<()>;
    fn ManipulationDelta(&self, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::Result<()>;
    fn ManipulationCompleted(&self, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::Result<()>;
}
impl _IManipulationEvents_Vtbl {
    pub const fn new<Identity: _IManipulationEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ManipulationStarted<Identity: _IManipulationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _IManipulationEvents_Impl::ManipulationStarted(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn ManipulationDelta<Identity: _IManipulationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _IManipulationEvents_Impl::ManipulationDelta(
                    this,
                    core::mem::transmute_copy(&x),
                    core::mem::transmute_copy(&y),
                    core::mem::transmute_copy(&translationdeltax),
                    core::mem::transmute_copy(&translationdeltay),
                    core::mem::transmute_copy(&scaledelta),
                    core::mem::transmute_copy(&expansiondelta),
                    core::mem::transmute_copy(&rotationdelta),
                    core::mem::transmute_copy(&cumulativetranslationx),
                    core::mem::transmute_copy(&cumulativetranslationy),
                    core::mem::transmute_copy(&cumulativescale),
                    core::mem::transmute_copy(&cumulativeexpansion),
                    core::mem::transmute_copy(&cumulativerotation),
                )
                .into()
            }
        }
        unsafe extern "system" fn ManipulationCompleted<Identity: _IManipulationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _IManipulationEvents_Impl::ManipulationCompleted(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&cumulativetranslationx), core::mem::transmute_copy(&cumulativetranslationy), core::mem::transmute_copy(&cumulativescale), core::mem::transmute_copy(&cumulativeexpansion), core::mem::transmute_copy(&cumulativerotation)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ManipulationStarted: ManipulationStarted::<Identity, OFFSET>,
            ManipulationDelta: ManipulationDelta::<Identity, OFFSET>,
            ManipulationCompleted: ManipulationCompleted::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_IManipulationEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for _IManipulationEvents {}
