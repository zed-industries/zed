pub trait IDXGIAdapter_Impl: Sized + IDXGIObject_Impl {
    fn EnumOutputs(&self, output: u32) -> windows_core::Result<IDXGIOutput>;
    fn GetDesc(&self) -> windows_core::Result<DXGI_ADAPTER_DESC>;
    fn CheckInterfaceSupport(&self, interfacename: *const windows_core::GUID) -> windows_core::Result<i64>;
}
impl windows_core::RuntimeName for IDXGIAdapter {}
impl IDXGIAdapter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIAdapter_Vtbl
    where
        Identity: IDXGIAdapter_Impl,
    {
        unsafe extern "system" fn EnumOutputs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, output: u32, ppoutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIAdapter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIAdapter_Impl::EnumOutputs(this, core::mem::transmute_copy(&output)) {
                Ok(ok__) => {
                    ppoutput.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC) -> windows_core::HRESULT
        where
            Identity: IDXGIAdapter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIAdapter_Impl::GetDesc(this) {
                Ok(ok__) => {
                    pdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckInterfaceSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacename: *const windows_core::GUID, pumdversion: *mut i64) -> windows_core::HRESULT
        where
            Identity: IDXGIAdapter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIAdapter_Impl::CheckInterfaceSupport(this, core::mem::transmute_copy(&interfacename)) {
                Ok(ok__) => {
                    pumdversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            EnumOutputs: EnumOutputs::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            CheckInterfaceSupport: CheckInterfaceSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
pub trait IDXGIAdapter1_Impl: Sized + IDXGIAdapter_Impl {
    fn GetDesc1(&self) -> windows_core::Result<DXGI_ADAPTER_DESC1>;
}
impl windows_core::RuntimeName for IDXGIAdapter1 {}
impl IDXGIAdapter1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIAdapter1_Vtbl
    where
        Identity: IDXGIAdapter1_Impl,
    {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC1) -> windows_core::HRESULT
        where
            Identity: IDXGIAdapter1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIAdapter1_Impl::GetDesc1(this) {
                Ok(ok__) => {
                    pdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IDXGIAdapter_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIAdapter as windows_core::Interface>::IID
    }
}
pub trait IDXGIAdapter2_Impl: Sized + IDXGIAdapter1_Impl {
    fn GetDesc2(&self) -> windows_core::Result<DXGI_ADAPTER_DESC2>;
}
impl windows_core::RuntimeName for IDXGIAdapter2 {}
impl IDXGIAdapter2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIAdapter2_Vtbl
    where
        Identity: IDXGIAdapter2_Impl,
    {
        unsafe extern "system" fn GetDesc2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC2) -> windows_core::HRESULT
        where
            Identity: IDXGIAdapter2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIAdapter2_Impl::GetDesc2(this) {
                Ok(ok__) => {
                    pdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IDXGIAdapter1_Vtbl::new::<Identity, OFFSET>(), GetDesc2: GetDesc2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIAdapter as windows_core::Interface>::IID || iid == &<IDXGIAdapter1 as windows_core::Interface>::IID
    }
}
pub trait IDXGIAdapter3_Impl: Sized + IDXGIAdapter2_Impl {
    fn RegisterHardwareContentProtectionTeardownStatusEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32);
    fn QueryVideoMemoryInfo(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> windows_core::Result<()>;
    fn SetVideoMemoryReservation(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> windows_core::Result<()>;
    fn RegisterVideoMemoryBudgetChangeNotificationEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32);
}
impl windows_core::RuntimeName for IDXGIAdapter3 {}
impl IDXGIAdapter3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIAdapter3_Vtbl
    where
        Identity: IDXGIAdapter3_Impl,
    {
        unsafe extern "system" fn RegisterHardwareContentProtectionTeardownStatusEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGIAdapter3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIAdapter3_Impl::RegisterHardwareContentProtectionTeardownStatusEvent(this, core::mem::transmute_copy(&hevent)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterHardwareContentProtectionTeardownStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32)
        where
            Identity: IDXGIAdapter3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIAdapter3_Impl::UnregisterHardwareContentProtectionTeardownStatus(this, core::mem::transmute_copy(&dwcookie))
        }
        unsafe extern "system" fn QueryVideoMemoryInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> windows_core::HRESULT
        where
            Identity: IDXGIAdapter3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIAdapter3_Impl::QueryVideoMemoryInfo(this, core::mem::transmute_copy(&nodeindex), core::mem::transmute_copy(&memorysegmentgroup), core::mem::transmute_copy(&pvideomemoryinfo)).into()
        }
        unsafe extern "system" fn SetVideoMemoryReservation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> windows_core::HRESULT
        where
            Identity: IDXGIAdapter3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIAdapter3_Impl::SetVideoMemoryReservation(this, core::mem::transmute_copy(&nodeindex), core::mem::transmute_copy(&memorysegmentgroup), core::mem::transmute_copy(&reservation)).into()
        }
        unsafe extern "system" fn RegisterVideoMemoryBudgetChangeNotificationEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGIAdapter3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIAdapter3_Impl::RegisterVideoMemoryBudgetChangeNotificationEvent(this, core::mem::transmute_copy(&hevent)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterVideoMemoryBudgetChangeNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32)
        where
            Identity: IDXGIAdapter3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIAdapter3_Impl::UnregisterVideoMemoryBudgetChangeNotification(this, core::mem::transmute_copy(&dwcookie))
        }
        Self {
            base__: IDXGIAdapter2_Vtbl::new::<Identity, OFFSET>(),
            RegisterHardwareContentProtectionTeardownStatusEvent: RegisterHardwareContentProtectionTeardownStatusEvent::<Identity, OFFSET>,
            UnregisterHardwareContentProtectionTeardownStatus: UnregisterHardwareContentProtectionTeardownStatus::<Identity, OFFSET>,
            QueryVideoMemoryInfo: QueryVideoMemoryInfo::<Identity, OFFSET>,
            SetVideoMemoryReservation: SetVideoMemoryReservation::<Identity, OFFSET>,
            RegisterVideoMemoryBudgetChangeNotificationEvent: RegisterVideoMemoryBudgetChangeNotificationEvent::<Identity, OFFSET>,
            UnregisterVideoMemoryBudgetChangeNotification: UnregisterVideoMemoryBudgetChangeNotification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter3 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIAdapter as windows_core::Interface>::IID || iid == &<IDXGIAdapter1 as windows_core::Interface>::IID || iid == &<IDXGIAdapter2 as windows_core::Interface>::IID
    }
}
pub trait IDXGIAdapter4_Impl: Sized + IDXGIAdapter3_Impl {
    fn GetDesc3(&self) -> windows_core::Result<DXGI_ADAPTER_DESC3>;
}
impl windows_core::RuntimeName for IDXGIAdapter4 {}
impl IDXGIAdapter4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIAdapter4_Vtbl
    where
        Identity: IDXGIAdapter4_Impl,
    {
        unsafe extern "system" fn GetDesc3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC3) -> windows_core::HRESULT
        where
            Identity: IDXGIAdapter4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIAdapter4_Impl::GetDesc3(this) {
                Ok(ok__) => {
                    pdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IDXGIAdapter3_Vtbl::new::<Identity, OFFSET>(), GetDesc3: GetDesc3::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter4 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIAdapter as windows_core::Interface>::IID || iid == &<IDXGIAdapter1 as windows_core::Interface>::IID || iid == &<IDXGIAdapter2 as windows_core::Interface>::IID || iid == &<IDXGIAdapter3 as windows_core::Interface>::IID
    }
}
pub trait IDXGIDebug_Impl: Sized {
    fn ReportLiveObjects(&self, apiid: &windows_core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDXGIDebug {}
impl IDXGIDebug_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIDebug_Vtbl
    where
        Identity: IDXGIDebug_Impl,
    {
        unsafe extern "system" fn ReportLiveObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, apiid: windows_core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> windows_core::HRESULT
        where
            Identity: IDXGIDebug_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDebug_Impl::ReportLiveObjects(this, core::mem::transmute(&apiid), core::mem::transmute_copy(&flags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ReportLiveObjects: ReportLiveObjects::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDebug as windows_core::Interface>::IID
    }
}
pub trait IDXGIDebug1_Impl: Sized + IDXGIDebug_Impl {
    fn EnableLeakTrackingForThread(&self);
    fn DisableLeakTrackingForThread(&self);
    fn IsLeakTrackingEnabledForThread(&self) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for IDXGIDebug1 {}
impl IDXGIDebug1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIDebug1_Vtbl
    where
        Identity: IDXGIDebug1_Impl,
    {
        unsafe extern "system" fn EnableLeakTrackingForThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IDXGIDebug1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDebug1_Impl::EnableLeakTrackingForThread(this)
        }
        unsafe extern "system" fn DisableLeakTrackingForThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IDXGIDebug1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDebug1_Impl::DisableLeakTrackingForThread(this)
        }
        unsafe extern "system" fn IsLeakTrackingEnabledForThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDXGIDebug1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDebug1_Impl::IsLeakTrackingEnabledForThread(this)
        }
        Self {
            base__: IDXGIDebug_Vtbl::new::<Identity, OFFSET>(),
            EnableLeakTrackingForThread: EnableLeakTrackingForThread::<Identity, OFFSET>,
            DisableLeakTrackingForThread: DisableLeakTrackingForThread::<Identity, OFFSET>,
            IsLeakTrackingEnabledForThread: IsLeakTrackingEnabledForThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDebug1 as windows_core::Interface>::IID || iid == &<IDXGIDebug as windows_core::Interface>::IID
    }
}
pub trait IDXGIDecodeSwapChain_Impl: Sized {
    fn PresentBuffer(&self, buffertopresent: u32, syncinterval: u32, flags: DXGI_PRESENT) -> windows_core::HRESULT;
    fn SetSourceRect(&self, prect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn SetTargetRect(&self, prect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn SetDestSize(&self, width: u32, height: u32) -> windows_core::Result<()>;
    fn GetSourceRect(&self) -> windows_core::Result<super::super::Foundation::RECT>;
    fn GetTargetRect(&self) -> windows_core::Result<super::super::Foundation::RECT>;
    fn GetDestSize(&self, pwidth: *mut u32, pheight: *mut u32) -> windows_core::Result<()>;
    fn SetColorSpace(&self, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> windows_core::Result<()>;
    fn GetColorSpace(&self) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS;
}
impl windows_core::RuntimeName for IDXGIDecodeSwapChain {}
impl IDXGIDecodeSwapChain_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIDecodeSwapChain_Vtbl
    where
        Identity: IDXGIDecodeSwapChain_Impl,
    {
        unsafe extern "system" fn PresentBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffertopresent: u32, syncinterval: u32, flags: DXGI_PRESENT) -> windows_core::HRESULT
        where
            Identity: IDXGIDecodeSwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDecodeSwapChain_Impl::PresentBuffer(this, core::mem::transmute_copy(&buffertopresent), core::mem::transmute_copy(&syncinterval), core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn SetSourceRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDXGIDecodeSwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDecodeSwapChain_Impl::SetSourceRect(this, core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn SetTargetRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDXGIDecodeSwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDecodeSwapChain_Impl::SetTargetRect(this, core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn SetDestSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT
        where
            Identity: IDXGIDecodeSwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDecodeSwapChain_Impl::SetDestSize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn GetSourceRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDXGIDecodeSwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIDecodeSwapChain_Impl::GetSourceRect(this) {
                Ok(ok__) => {
                    prect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDXGIDecodeSwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIDecodeSwapChain_Impl::GetTargetRect(this) {
                Ok(ok__) => {
                    prect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGIDecodeSwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDecodeSwapChain_Impl::GetDestSize(this, core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
        }
        unsafe extern "system" fn SetColorSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> windows_core::HRESULT
        where
            Identity: IDXGIDecodeSwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDecodeSwapChain_Impl::SetColorSpace(this, core::mem::transmute_copy(&colorspace)).into()
        }
        unsafe extern "system" fn GetColorSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS
        where
            Identity: IDXGIDecodeSwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDecodeSwapChain_Impl::GetColorSpace(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PresentBuffer: PresentBuffer::<Identity, OFFSET>,
            SetSourceRect: SetSourceRect::<Identity, OFFSET>,
            SetTargetRect: SetTargetRect::<Identity, OFFSET>,
            SetDestSize: SetDestSize::<Identity, OFFSET>,
            GetSourceRect: GetSourceRect::<Identity, OFFSET>,
            GetTargetRect: GetTargetRect::<Identity, OFFSET>,
            GetDestSize: GetDestSize::<Identity, OFFSET>,
            SetColorSpace: SetColorSpace::<Identity, OFFSET>,
            GetColorSpace: GetColorSpace::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDecodeSwapChain as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIDevice_Impl: Sized + IDXGIObject_Impl {
    fn GetAdapter(&self) -> windows_core::Result<IDXGIAdapter>;
    fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: DXGI_USAGE, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut Option<IDXGISurface>) -> windows_core::Result<()>;
    fn QueryResourceResidency(&self, ppresources: *const Option<windows_core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> windows_core::Result<()>;
    fn SetGPUThreadPriority(&self, priority: i32) -> windows_core::Result<()>;
    fn GetGPUThreadPriority(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIDevice {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIDevice_Vtbl
    where
        Identity: IDXGIDevice_Impl,
    {
        unsafe extern "system" fn GetAdapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, padapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIDevice_Impl::GetAdapter(this) {
                Ok(ok__) => {
                    padapter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: DXGI_USAGE, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDevice_Impl::CreateSurface(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&numsurfaces), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&psharedresource), core::mem::transmute_copy(&ppsurface)).into()
        }
        unsafe extern "system" fn QueryResourceResidency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> windows_core::HRESULT
        where
            Identity: IDXGIDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDevice_Impl::QueryResourceResidency(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&presidencystatus), core::mem::transmute_copy(&numresources)).into()
        }
        unsafe extern "system" fn SetGPUThreadPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: i32) -> windows_core::HRESULT
        where
            Identity: IDXGIDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDevice_Impl::SetGPUThreadPriority(this, core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn GetGPUThreadPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDXGIDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIDevice_Impl::GetGPUThreadPriority(this) {
                Ok(ok__) => {
                    ppriority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            GetAdapter: GetAdapter::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            QueryResourceResidency: QueryResourceResidency::<Identity, OFFSET>,
            SetGPUThreadPriority: SetGPUThreadPriority::<Identity, OFFSET>,
            GetGPUThreadPriority: GetGPUThreadPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIDevice1_Impl: Sized + IDXGIDevice_Impl {
    fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()>;
    fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIDevice1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIDevice1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIDevice1_Vtbl
    where
        Identity: IDXGIDevice1_Impl,
    {
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlatency: u32) -> windows_core::HRESULT
        where
            Identity: IDXGIDevice1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDevice1_Impl::SetMaximumFrameLatency(this, core::mem::transmute_copy(&maxlatency)).into()
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxlatency: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGIDevice1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIDevice1_Impl::GetMaximumFrameLatency(this) {
                Ok(ok__) => {
                    pmaxlatency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIDevice_Vtbl::new::<Identity, OFFSET>(),
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDevice as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIDevice2_Impl: Sized + IDXGIDevice1_Impl {
    fn OfferResources(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> windows_core::Result<()>;
    fn ReclaimResources(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, pdiscarded: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn EnqueueSetEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIDevice2 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIDevice2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIDevice2_Vtbl
    where
        Identity: IDXGIDevice2_Impl,
    {
        unsafe extern "system" fn OfferResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> windows_core::HRESULT
        where
            Identity: IDXGIDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDevice2_Impl::OfferResources(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn ReclaimResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, pdiscarded: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDXGIDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDevice2_Impl::ReclaimResources(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&pdiscarded)).into()
        }
        unsafe extern "system" fn EnqueueSetEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDXGIDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDevice2_Impl::EnqueueSetEvent(this, core::mem::transmute_copy(&hevent)).into()
        }
        Self {
            base__: IDXGIDevice1_Vtbl::new::<Identity, OFFSET>(),
            OfferResources: OfferResources::<Identity, OFFSET>,
            ReclaimResources: ReclaimResources::<Identity, OFFSET>,
            EnqueueSetEvent: EnqueueSetEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDevice as windows_core::Interface>::IID || iid == &<IDXGIDevice1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIDevice3_Impl: Sized + IDXGIDevice2_Impl {
    fn Trim(&self);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIDevice3 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIDevice3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIDevice3_Vtbl
    where
        Identity: IDXGIDevice3_Impl,
    {
        unsafe extern "system" fn Trim<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IDXGIDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDevice3_Impl::Trim(this)
        }
        Self { base__: IDXGIDevice2_Vtbl::new::<Identity, OFFSET>(), Trim: Trim::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice3 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDevice as windows_core::Interface>::IID || iid == &<IDXGIDevice1 as windows_core::Interface>::IID || iid == &<IDXGIDevice2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIDevice4_Impl: Sized + IDXGIDevice3_Impl {
    fn OfferResources1(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: &DXGI_OFFER_RESOURCE_FLAGS) -> windows_core::Result<()>;
    fn ReclaimResources1(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIDevice4 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIDevice4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIDevice4_Vtbl
    where
        Identity: IDXGIDevice4_Impl,
    {
        unsafe extern "system" fn OfferResources1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> windows_core::HRESULT
        where
            Identity: IDXGIDevice4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDevice4_Impl::OfferResources1(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&priority), core::mem::transmute(&flags)).into()
        }
        unsafe extern "system" fn ReclaimResources1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> windows_core::HRESULT
        where
            Identity: IDXGIDevice4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDevice4_Impl::ReclaimResources1(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&presults)).into()
        }
        Self {
            base__: IDXGIDevice3_Vtbl::new::<Identity, OFFSET>(),
            OfferResources1: OfferResources1::<Identity, OFFSET>,
            ReclaimResources1: ReclaimResources1::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice4 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDevice as windows_core::Interface>::IID || iid == &<IDXGIDevice1 as windows_core::Interface>::IID || iid == &<IDXGIDevice2 as windows_core::Interface>::IID || iid == &<IDXGIDevice3 as windows_core::Interface>::IID
    }
}
pub trait IDXGIDeviceSubObject_Impl: Sized + IDXGIObject_Impl {
    fn GetDevice(&self, riid: *const windows_core::GUID, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDXGIDeviceSubObject {}
impl IDXGIDeviceSubObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIDeviceSubObject_Vtbl
    where
        Identity: IDXGIDeviceSubObject_Impl,
    {
        unsafe extern "system" fn GetDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIDeviceSubObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDeviceSubObject_Impl::GetDevice(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppdevice)).into()
        }
        Self { base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(), GetDevice: GetDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
pub trait IDXGIDisplayControl_Impl: Sized {
    fn IsStereoEnabled(&self) -> super::super::Foundation::BOOL;
    fn SetStereoEnabled(&self, enabled: super::super::Foundation::BOOL);
}
impl windows_core::RuntimeName for IDXGIDisplayControl {}
impl IDXGIDisplayControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIDisplayControl_Vtbl
    where
        Identity: IDXGIDisplayControl_Impl,
    {
        unsafe extern "system" fn IsStereoEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDXGIDisplayControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDisplayControl_Impl::IsStereoEnabled(this)
        }
        unsafe extern "system" fn SetStereoEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::BOOL)
        where
            Identity: IDXGIDisplayControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIDisplayControl_Impl::SetStereoEnabled(this, core::mem::transmute_copy(&enabled))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsStereoEnabled: IsStereoEnabled::<Identity, OFFSET>,
            SetStereoEnabled: SetStereoEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDisplayControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory_Impl: Sized + IDXGIObject_Impl {
    fn EnumAdapters(&self, adapter: u32) -> windows_core::Result<IDXGIAdapter>;
    fn MakeWindowAssociation(&self, windowhandle: super::super::Foundation::HWND, flags: DXGI_MWA_FLAGS) -> windows_core::Result<()>;
    fn GetWindowAssociation(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn CreateSwapChain(&self, pdevice: Option<&windows_core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut Option<IDXGISwapChain>) -> windows_core::HRESULT;
    fn CreateSoftwareAdapter(&self, module: super::super::Foundation::HMODULE) -> windows_core::Result<IDXGIAdapter>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIFactory_Vtbl
    where
        Identity: IDXGIFactory_Impl,
    {
        unsafe extern "system" fn EnumAdapters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, ppadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIFactory_Impl::EnumAdapters(this, core::mem::transmute_copy(&adapter)) {
                Ok(ok__) => {
                    ppadapter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeWindowAssociation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowhandle: super::super::Foundation::HWND, flags: DXGI_MWA_FLAGS) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIFactory_Impl::MakeWindowAssociation(this, core::mem::transmute_copy(&windowhandle), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetWindowAssociation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwindowhandle: *mut super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIFactory_Impl::GetWindowAssociation(this) {
                Ok(ok__) => {
                    pwindowhandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSwapChain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIFactory_Impl::CreateSwapChain(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppswapchain))
        }
        unsafe extern "system" fn CreateSoftwareAdapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, module: super::super::Foundation::HMODULE, ppadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIFactory_Impl::CreateSoftwareAdapter(this, core::mem::transmute_copy(&module)) {
                Ok(ok__) => {
                    ppadapter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            EnumAdapters: EnumAdapters::<Identity, OFFSET>,
            MakeWindowAssociation: MakeWindowAssociation::<Identity, OFFSET>,
            GetWindowAssociation: GetWindowAssociation::<Identity, OFFSET>,
            CreateSwapChain: CreateSwapChain::<Identity, OFFSET>,
            CreateSoftwareAdapter: CreateSoftwareAdapter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory1_Impl: Sized + IDXGIFactory_Impl {
    fn EnumAdapters1(&self, adapter: u32) -> windows_core::Result<IDXGIAdapter1>;
    fn IsCurrent(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIFactory1_Vtbl
    where
        Identity: IDXGIFactory1_Impl,
    {
        unsafe extern "system" fn EnumAdapters1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, ppadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIFactory1_Impl::EnumAdapters1(this, core::mem::transmute_copy(&adapter)) {
                Ok(ok__) => {
                    ppadapter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDXGIFactory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIFactory1_Impl::IsCurrent(this)
        }
        Self { base__: IDXGIFactory_Vtbl::new::<Identity, OFFSET>(), EnumAdapters1: EnumAdapters1::<Identity, OFFSET>, IsCurrent: IsCurrent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory2_Impl: Sized + IDXGIFactory1_Impl {
    fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL;
    fn CreateSwapChainForHwnd(&self, pdevice: Option<&windows_core::IUnknown>, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: Option<&IDXGIOutput>) -> windows_core::Result<IDXGISwapChain1>;
    fn CreateSwapChainForCoreWindow(&self, pdevice: Option<&windows_core::IUnknown>, pwindow: Option<&windows_core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: Option<&IDXGIOutput>) -> windows_core::Result<IDXGISwapChain1>;
    fn GetSharedResourceAdapterLuid(&self, hresource: super::super::Foundation::HANDLE) -> windows_core::Result<super::super::Foundation::LUID>;
    fn RegisterStereoStatusWindow(&self, windowhandle: super::super::Foundation::HWND, wmsg: u32) -> windows_core::Result<u32>;
    fn RegisterStereoStatusEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterStereoStatus(&self, dwcookie: u32);
    fn RegisterOcclusionStatusWindow(&self, windowhandle: super::super::Foundation::HWND, wmsg: u32) -> windows_core::Result<u32>;
    fn RegisterOcclusionStatusEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterOcclusionStatus(&self, dwcookie: u32);
    fn CreateSwapChainForComposition(&self, pdevice: Option<&windows_core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: Option<&IDXGIOutput>) -> windows_core::Result<IDXGISwapChain1>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory2 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIFactory2_Vtbl
    where
        Identity: IDXGIFactory2_Impl,
    {
        unsafe extern "system" fn IsWindowedStereoEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDXGIFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIFactory2_Impl::IsWindowedStereoEnabled(this)
        }
        unsafe extern "system" fn CreateSwapChainForHwnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIFactory2_Impl::CreateSwapChainForHwnd(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pfullscreendesc), windows_core::from_raw_borrowed(&prestricttooutput)) {
                Ok(ok__) => {
                    ppswapchain.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSwapChainForCoreWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pwindow: *mut core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIFactory2_Impl::CreateSwapChainForCoreWindow(this, windows_core::from_raw_borrowed(&pdevice), windows_core::from_raw_borrowed(&pwindow), core::mem::transmute_copy(&pdesc), windows_core::from_raw_borrowed(&prestricttooutput)) {
                Ok(ok__) => {
                    ppswapchain.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSharedResourceAdapterLuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresource: super::super::Foundation::HANDLE, pluid: *mut super::super::Foundation::LUID) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIFactory2_Impl::GetSharedResourceAdapterLuid(this, core::mem::transmute_copy(&hresource)) {
                Ok(ok__) => {
                    pluid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterStereoStatusWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIFactory2_Impl::RegisterStereoStatusWindow(this, core::mem::transmute_copy(&windowhandle), core::mem::transmute_copy(&wmsg)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterStereoStatusEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIFactory2_Impl::RegisterStereoStatusEvent(this, core::mem::transmute_copy(&hevent)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterStereoStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32)
        where
            Identity: IDXGIFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIFactory2_Impl::UnregisterStereoStatus(this, core::mem::transmute_copy(&dwcookie))
        }
        unsafe extern "system" fn RegisterOcclusionStatusWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIFactory2_Impl::RegisterOcclusionStatusWindow(this, core::mem::transmute_copy(&windowhandle), core::mem::transmute_copy(&wmsg)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterOcclusionStatusEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIFactory2_Impl::RegisterOcclusionStatusEvent(this, core::mem::transmute_copy(&hevent)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterOcclusionStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32)
        where
            Identity: IDXGIFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIFactory2_Impl::UnregisterOcclusionStatus(this, core::mem::transmute_copy(&dwcookie))
        }
        unsafe extern "system" fn CreateSwapChainForComposition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIFactory2_Impl::CreateSwapChainForComposition(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&pdesc), windows_core::from_raw_borrowed(&prestricttooutput)) {
                Ok(ok__) => {
                    ppswapchain.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIFactory1_Vtbl::new::<Identity, OFFSET>(),
            IsWindowedStereoEnabled: IsWindowedStereoEnabled::<Identity, OFFSET>,
            CreateSwapChainForHwnd: CreateSwapChainForHwnd::<Identity, OFFSET>,
            CreateSwapChainForCoreWindow: CreateSwapChainForCoreWindow::<Identity, OFFSET>,
            GetSharedResourceAdapterLuid: GetSharedResourceAdapterLuid::<Identity, OFFSET>,
            RegisterStereoStatusWindow: RegisterStereoStatusWindow::<Identity, OFFSET>,
            RegisterStereoStatusEvent: RegisterStereoStatusEvent::<Identity, OFFSET>,
            UnregisterStereoStatus: UnregisterStereoStatus::<Identity, OFFSET>,
            RegisterOcclusionStatusWindow: RegisterOcclusionStatusWindow::<Identity, OFFSET>,
            RegisterOcclusionStatusEvent: RegisterOcclusionStatusEvent::<Identity, OFFSET>,
            UnregisterOcclusionStatus: UnregisterOcclusionStatus::<Identity, OFFSET>,
            CreateSwapChainForComposition: CreateSwapChainForComposition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory3_Impl: Sized + IDXGIFactory2_Impl {
    fn GetCreationFlags(&self) -> DXGI_CREATE_FACTORY_FLAGS;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory3 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIFactory3_Vtbl
    where
        Identity: IDXGIFactory3_Impl,
    {
        unsafe extern "system" fn GetCreationFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DXGI_CREATE_FACTORY_FLAGS
        where
            Identity: IDXGIFactory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIFactory3_Impl::GetCreationFlags(this)
        }
        Self { base__: IDXGIFactory2_Vtbl::new::<Identity, OFFSET>(), GetCreationFlags: GetCreationFlags::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory3 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIFactory2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory4_Impl: Sized + IDXGIFactory3_Impl {
    fn EnumAdapterByLuid(&self, adapterluid: &super::super::Foundation::LUID, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EnumWarpAdapter(&self, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory4 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIFactory4_Vtbl
    where
        Identity: IDXGIFactory4_Impl,
    {
        unsafe extern "system" fn EnumAdapterByLuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapterluid: super::super::Foundation::LUID, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIFactory4_Impl::EnumAdapterByLuid(this, core::mem::transmute(&adapterluid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
        }
        unsafe extern "system" fn EnumWarpAdapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIFactory4_Impl::EnumWarpAdapter(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
        }
        Self {
            base__: IDXGIFactory3_Vtbl::new::<Identity, OFFSET>(),
            EnumAdapterByLuid: EnumAdapterByLuid::<Identity, OFFSET>,
            EnumWarpAdapter: EnumWarpAdapter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory4 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIFactory2 as windows_core::Interface>::IID || iid == &<IDXGIFactory3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory5_Impl: Sized + IDXGIFactory4_Impl {
    fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory5 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIFactory5_Vtbl
    where
        Identity: IDXGIFactory5_Impl,
    {
        unsafe extern "system" fn CheckFeatureSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: DXGI_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIFactory5_Impl::CheckFeatureSupport(this, core::mem::transmute_copy(&feature), core::mem::transmute_copy(&pfeaturesupportdata), core::mem::transmute_copy(&featuresupportdatasize)).into()
        }
        Self { base__: IDXGIFactory4_Vtbl::new::<Identity, OFFSET>(), CheckFeatureSupport: CheckFeatureSupport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory5 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIFactory2 as windows_core::Interface>::IID || iid == &<IDXGIFactory3 as windows_core::Interface>::IID || iid == &<IDXGIFactory4 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory6_Impl: Sized + IDXGIFactory5_Impl {
    fn EnumAdapterByGpuPreference(&self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory6 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory6_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIFactory6_Vtbl
    where
        Identity: IDXGIFactory6_Impl,
    {
        unsafe extern "system" fn EnumAdapterByGpuPreference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIFactory6_Impl::EnumAdapterByGpuPreference(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&gpupreference), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
        }
        Self { base__: IDXGIFactory5_Vtbl::new::<Identity, OFFSET>(), EnumAdapterByGpuPreference: EnumAdapterByGpuPreference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory6 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIFactory2 as windows_core::Interface>::IID || iid == &<IDXGIFactory3 as windows_core::Interface>::IID || iid == &<IDXGIFactory4 as windows_core::Interface>::IID || iid == &<IDXGIFactory5 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory7_Impl: Sized + IDXGIFactory6_Impl {
    fn RegisterAdaptersChangedEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterAdaptersChangedEvent(&self, dwcookie: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory7 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory7_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIFactory7_Vtbl
    where
        Identity: IDXGIFactory7_Impl,
    {
        unsafe extern "system" fn RegisterAdaptersChangedEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIFactory7_Impl::RegisterAdaptersChangedEvent(this, core::mem::transmute_copy(&hevent)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterAdaptersChangedEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT
        where
            Identity: IDXGIFactory7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIFactory7_Impl::UnregisterAdaptersChangedEvent(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: IDXGIFactory6_Vtbl::new::<Identity, OFFSET>(),
            RegisterAdaptersChangedEvent: RegisterAdaptersChangedEvent::<Identity, OFFSET>,
            UnregisterAdaptersChangedEvent: UnregisterAdaptersChangedEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory7 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIFactory2 as windows_core::Interface>::IID || iid == &<IDXGIFactory3 as windows_core::Interface>::IID || iid == &<IDXGIFactory4 as windows_core::Interface>::IID || iid == &<IDXGIFactory5 as windows_core::Interface>::IID || iid == &<IDXGIFactory6 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactoryMedia_Impl: Sized {
    fn CreateSwapChainForCompositionSurfaceHandle(&self, pdevice: Option<&windows_core::IUnknown>, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: Option<&IDXGIOutput>) -> windows_core::Result<IDXGISwapChain1>;
    fn CreateDecodeSwapChainForCompositionSurfaceHandle(&self, pdevice: Option<&windows_core::IUnknown>, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: Option<&IDXGIResource>, prestricttooutput: Option<&IDXGIOutput>) -> windows_core::Result<IDXGIDecodeSwapChain>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactoryMedia {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactoryMedia_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIFactoryMedia_Vtbl
    where
        Identity: IDXGIFactoryMedia_Impl,
    {
        unsafe extern "system" fn CreateSwapChainForCompositionSurfaceHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIFactoryMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIFactoryMedia_Impl::CreateSwapChainForCompositionSurfaceHandle(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&hsurface), core::mem::transmute_copy(&pdesc), windows_core::from_raw_borrowed(&prestricttooutput)) {
                Ok(ok__) => {
                    ppswapchain.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecodeSwapChainForCompositionSurfaceHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: *mut core::ffi::c_void, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIFactoryMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIFactoryMedia_Impl::CreateDecodeSwapChainForCompositionSurfaceHandle(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&hsurface), core::mem::transmute_copy(&pdesc), windows_core::from_raw_borrowed(&pyuvdecodebuffers), windows_core::from_raw_borrowed(&prestricttooutput)) {
                Ok(ok__) => {
                    ppswapchain.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSwapChainForCompositionSurfaceHandle: CreateSwapChainForCompositionSurfaceHandle::<Identity, OFFSET>,
            CreateDecodeSwapChainForCompositionSurfaceHandle: CreateDecodeSwapChainForCompositionSurfaceHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactoryMedia as windows_core::Interface>::IID
    }
}
pub trait IDXGIInfoQueue_Impl: Sized {
    fn SetMessageCountLimit(&self, producer: &windows_core::GUID, messagecountlimit: u64) -> windows_core::Result<()>;
    fn ClearStoredMessages(&self, producer: &windows_core::GUID);
    fn GetMessage(&self, producer: &windows_core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> windows_core::Result<()>;
    fn GetNumStoredMessagesAllowedByRetrievalFilters(&self, producer: &windows_core::GUID) -> u64;
    fn GetNumStoredMessages(&self, producer: &windows_core::GUID) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(&self, producer: &windows_core::GUID) -> u64;
    fn GetMessageCountLimit(&self, producer: &windows_core::GUID) -> u64;
    fn GetNumMessagesAllowedByStorageFilter(&self, producer: &windows_core::GUID) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(&self, producer: &windows_core::GUID) -> u64;
    fn AddStorageFilterEntries(&self, producer: &windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn GetStorageFilter(&self, producer: &windows_core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::Result<()>;
    fn ClearStorageFilter(&self, producer: &windows_core::GUID);
    fn PushEmptyStorageFilter(&self, producer: &windows_core::GUID) -> windows_core::Result<()>;
    fn PushDenyAllStorageFilter(&self, producer: &windows_core::GUID) -> windows_core::Result<()>;
    fn PushCopyOfStorageFilter(&self, producer: &windows_core::GUID) -> windows_core::Result<()>;
    fn PushStorageFilter(&self, producer: &windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn PopStorageFilter(&self, producer: &windows_core::GUID);
    fn GetStorageFilterStackSize(&self, producer: &windows_core::GUID) -> u32;
    fn AddRetrievalFilterEntries(&self, producer: &windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn GetRetrievalFilter(&self, producer: &windows_core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::Result<()>;
    fn ClearRetrievalFilter(&self, producer: &windows_core::GUID);
    fn PushEmptyRetrievalFilter(&self, producer: &windows_core::GUID) -> windows_core::Result<()>;
    fn PushDenyAllRetrievalFilter(&self, producer: &windows_core::GUID) -> windows_core::Result<()>;
    fn PushCopyOfRetrievalFilter(&self, producer: &windows_core::GUID) -> windows_core::Result<()>;
    fn PushRetrievalFilter(&self, producer: &windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn PopRetrievalFilter(&self, producer: &windows_core::GUID);
    fn GetRetrievalFilterStackSize(&self, producer: &windows_core::GUID) -> u32;
    fn AddMessage(&self, producer: &windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn AddApplicationMessage(&self, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn SetBreakOnCategory(&self, producer: &windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetBreakOnSeverity(&self, producer: &windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetBreakOnID(&self, producer: &windows_core::GUID, id: i32, benable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetBreakOnCategory(&self, producer: &windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL;
    fn GetBreakOnSeverity(&self, producer: &windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL;
    fn GetBreakOnID(&self, producer: &windows_core::GUID, id: i32) -> super::super::Foundation::BOOL;
    fn SetMuteDebugOutput(&self, producer: &windows_core::GUID, bmute: super::super::Foundation::BOOL);
    fn GetMuteDebugOutput(&self, producer: &windows_core::GUID) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for IDXGIInfoQueue {}
impl IDXGIInfoQueue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIInfoQueue_Vtbl
    where
        Identity: IDXGIInfoQueue_Impl,
    {
        unsafe extern "system" fn SetMessageCountLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, messagecountlimit: u64) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::SetMessageCountLimit(this, core::mem::transmute(&producer), core::mem::transmute_copy(&messagecountlimit)).into()
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID)
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::ClearStoredMessages(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::GetMessage(this, core::mem::transmute(&producer), core::mem::transmute_copy(&messageindex), core::mem::transmute_copy(&pmessage), core::mem::transmute_copy(&pmessagebytelength)).into()
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::GetNumStoredMessagesAllowedByRetrievalFilters(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::GetNumStoredMessages(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::GetNumMessagesDiscardedByMessageCountLimit(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::GetMessageCountLimit(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::GetNumMessagesAllowedByStorageFilter(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::GetNumMessagesDeniedByStorageFilter(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::AddStorageFilterEntries(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetStorageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::GetStorageFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID)
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::ClearStorageFilter(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::PushEmptyStorageFilter(this, core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushDenyAllStorageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::PushDenyAllStorageFilter(this, core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::PushCopyOfStorageFilter(this, core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushStorageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::PushStorageFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopStorageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID)
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::PopStorageFilter(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u32
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::GetStorageFilterStackSize(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::AddRetrievalFilterEntries(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::GetRetrievalFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID)
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::ClearRetrievalFilter(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::PushEmptyRetrievalFilter(this, core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushDenyAllRetrievalFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::PushDenyAllRetrievalFilter(this, core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::PushCopyOfRetrievalFilter(this, core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::PushRetrievalFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID)
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::PopRetrievalFilter(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u32
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::GetRetrievalFilterStackSize(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn AddMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: windows_core::PCSTR) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::AddMessage(this, core::mem::transmute(&producer), core::mem::transmute_copy(&category), core::mem::transmute_copy(&severity), core::mem::transmute_copy(&id), core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: windows_core::PCSTR) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::AddApplicationMessage(this, core::mem::transmute_copy(&severity), core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::SetBreakOnCategory(this, core::mem::transmute(&producer), core::mem::transmute_copy(&category), core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::SetBreakOnSeverity(this, core::mem::transmute(&producer), core::mem::transmute_copy(&severity), core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, id: i32, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::SetBreakOnID(this, core::mem::transmute(&producer), core::mem::transmute_copy(&id), core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::GetBreakOnCategory(this, core::mem::transmute(&producer), core::mem::transmute_copy(&category))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::GetBreakOnSeverity(this, core::mem::transmute(&producer), core::mem::transmute_copy(&severity))
        }
        unsafe extern "system" fn GetBreakOnID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, id: i32) -> super::super::Foundation::BOOL
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::GetBreakOnID(this, core::mem::transmute(&producer), core::mem::transmute_copy(&id))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, bmute: super::super::Foundation::BOOL)
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::SetMuteDebugOutput(this, core::mem::transmute(&producer), core::mem::transmute_copy(&bmute))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> super::super::Foundation::BOOL
        where
            Identity: IDXGIInfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIInfoQueue_Impl::GetMuteDebugOutput(this, core::mem::transmute(&producer))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMessageCountLimit: SetMessageCountLimit::<Identity, OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Identity, OFFSET>,
            GetMessage: GetMessage::<Identity, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilters: GetNumStoredMessagesAllowedByRetrievalFilters::<Identity, OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Identity, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Identity, OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Identity, OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Identity, OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Identity, OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Identity, OFFSET>,
            GetStorageFilter: GetStorageFilter::<Identity, OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Identity, OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Identity, OFFSET>,
            PushDenyAllStorageFilter: PushDenyAllStorageFilter::<Identity, OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Identity, OFFSET>,
            PushStorageFilter: PushStorageFilter::<Identity, OFFSET>,
            PopStorageFilter: PopStorageFilter::<Identity, OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Identity, OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Identity, OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Identity, OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Identity, OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Identity, OFFSET>,
            PushDenyAllRetrievalFilter: PushDenyAllRetrievalFilter::<Identity, OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Identity, OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Identity, OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Identity, OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Identity, OFFSET>,
            AddMessage: AddMessage::<Identity, OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Identity, OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Identity, OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Identity, OFFSET>,
            SetBreakOnID: SetBreakOnID::<Identity, OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Identity, OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Identity, OFFSET>,
            GetBreakOnID: GetBreakOnID::<Identity, OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Identity, OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIInfoQueue as windows_core::Interface>::IID
    }
}
pub trait IDXGIKeyedMutex_Impl: Sized + IDXGIDeviceSubObject_Impl {
    fn AcquireSync(&self, key: u64, dwmilliseconds: u32) -> windows_core::Result<()>;
    fn ReleaseSync(&self, key: u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDXGIKeyedMutex {}
impl IDXGIKeyedMutex_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIKeyedMutex_Vtbl
    where
        Identity: IDXGIKeyedMutex_Impl,
    {
        unsafe extern "system" fn AcquireSync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: u64, dwmilliseconds: u32) -> windows_core::HRESULT
        where
            Identity: IDXGIKeyedMutex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIKeyedMutex_Impl::AcquireSync(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn ReleaseSync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: u64) -> windows_core::HRESULT
        where
            Identity: IDXGIKeyedMutex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIKeyedMutex_Impl::ReleaseSync(this, core::mem::transmute_copy(&key)).into()
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, OFFSET>(),
            AcquireSync: AcquireSync::<Identity, OFFSET>,
            ReleaseSync: ReleaseSync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIKeyedMutex as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID
    }
}
pub trait IDXGIObject_Impl: Sized {
    fn SetPrivateData(&self, name: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, name: *const windows_core::GUID, punknown: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetPrivateData(&self, name: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetParent(&self, riid: *const windows_core::GUID, ppparent: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDXGIObject {}
impl IDXGIObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIObject_Vtbl
    where
        Identity: IDXGIObject_Impl,
    {
        unsafe extern "system" fn SetPrivateData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIObject_Impl::SetPrivateData(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, punknown: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIObject_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&name), windows_core::from_raw_borrowed(&punknown)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIObject_Impl::GetPrivateData(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetParent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIObject_Impl::GetParent(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppparent)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            GetParent: GetParent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput_Impl: Sized + IDXGIObject_Impl {
    fn GetDesc(&self) -> windows_core::Result<DXGI_OUTPUT_DESC>;
    fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: DXGI_ENUM_MODES, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> windows_core::Result<()>;
    fn FindClosestMatchingMode(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn WaitForVBlank(&self) -> windows_core::Result<()>;
    fn TakeOwnership(&self, pdevice: Option<&windows_core::IUnknown>, exclusive: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ReleaseOwnership(&self);
    fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> windows_core::Result<()>;
    fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> windows_core::Result<()>;
    fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> windows_core::Result<()>;
    fn SetDisplaySurface(&self, pscanoutsurface: Option<&IDXGISurface>) -> windows_core::Result<()>;
    fn GetDisplaySurfaceData(&self, pdestination: Option<&IDXGISurface>) -> windows_core::Result<()>;
    fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGIOutput {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIOutput_Vtbl
    where
        Identity: IDXGIOutput_Impl,
    {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIOutput_Impl::GetDesc(this) {
                Ok(ok__) => {
                    pdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayModeList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: DXGI_ENUM_MODES, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutput_Impl::GetDisplayModeList(this, core::mem::transmute_copy(&enumformat), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pnummodes), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn FindClosestMatchingMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutput_Impl::FindClosestMatchingMode(this, core::mem::transmute_copy(&pmodetomatch), core::mem::transmute_copy(&pclosestmatch), windows_core::from_raw_borrowed(&pconcerneddevice)).into()
        }
        unsafe extern "system" fn WaitForVBlank<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutput_Impl::WaitForVBlank(this).into()
        }
        unsafe extern "system" fn TakeOwnership<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, exclusive: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutput_Impl::TakeOwnership(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&exclusive)).into()
        }
        unsafe extern "system" fn ReleaseOwnership<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IDXGIOutput_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutput_Impl::ReleaseOwnership(this)
        }
        unsafe extern "system" fn GetGammaControlCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutput_Impl::GetGammaControlCapabilities(this, core::mem::transmute_copy(&pgammacaps)).into()
        }
        unsafe extern "system" fn SetGammaControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parray: *const Common::DXGI_GAMMA_CONTROL) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutput_Impl::SetGammaControl(this, core::mem::transmute_copy(&parray)).into()
        }
        unsafe extern "system" fn GetGammaControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parray: *mut Common::DXGI_GAMMA_CONTROL) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutput_Impl::GetGammaControl(this, core::mem::transmute_copy(&parray)).into()
        }
        unsafe extern "system" fn SetDisplaySurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscanoutsurface: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutput_Impl::SetDisplaySurface(this, windows_core::from_raw_borrowed(&pscanoutsurface)).into()
        }
        unsafe extern "system" fn GetDisplaySurfaceData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestination: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutput_Impl::GetDisplaySurfaceData(this, windows_core::from_raw_borrowed(&pdestination)).into()
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutput_Impl::GetFrameStatistics(this, core::mem::transmute_copy(&pstats)).into()
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetDisplayModeList: GetDisplayModeList::<Identity, OFFSET>,
            FindClosestMatchingMode: FindClosestMatchingMode::<Identity, OFFSET>,
            WaitForVBlank: WaitForVBlank::<Identity, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, OFFSET>,
            ReleaseOwnership: ReleaseOwnership::<Identity, OFFSET>,
            GetGammaControlCapabilities: GetGammaControlCapabilities::<Identity, OFFSET>,
            SetGammaControl: SetGammaControl::<Identity, OFFSET>,
            GetGammaControl: GetGammaControl::<Identity, OFFSET>,
            SetDisplaySurface: SetDisplaySurface::<Identity, OFFSET>,
            GetDisplaySurfaceData: GetDisplaySurfaceData::<Identity, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput1_Impl: Sized + IDXGIOutput_Impl {
    fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: DXGI_ENUM_MODES, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> windows_core::Result<()>;
    fn FindClosestMatchingMode1(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetDisplaySurfaceData1(&self, pdestination: Option<&IDXGIResource>) -> windows_core::Result<()>;
    fn DuplicateOutput(&self, pdevice: Option<&windows_core::IUnknown>) -> windows_core::Result<IDXGIOutputDuplication>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGIOutput1 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIOutput1_Vtbl
    where
        Identity: IDXGIOutput1_Impl,
    {
        unsafe extern "system" fn GetDisplayModeList1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: DXGI_ENUM_MODES, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutput1_Impl::GetDisplayModeList1(this, core::mem::transmute_copy(&enumformat), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pnummodes), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn FindClosestMatchingMode1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutput1_Impl::FindClosestMatchingMode1(this, core::mem::transmute_copy(&pmodetomatch), core::mem::transmute_copy(&pclosestmatch), windows_core::from_raw_borrowed(&pconcerneddevice)).into()
        }
        unsafe extern "system" fn GetDisplaySurfaceData1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestination: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutput1_Impl::GetDisplaySurfaceData1(this, windows_core::from_raw_borrowed(&pdestination)).into()
        }
        unsafe extern "system" fn DuplicateOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, ppoutputduplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIOutput1_Impl::DuplicateOutput(this, windows_core::from_raw_borrowed(&pdevice)) {
                Ok(ok__) => {
                    ppoutputduplication.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIOutput_Vtbl::new::<Identity, OFFSET>(),
            GetDisplayModeList1: GetDisplayModeList1::<Identity, OFFSET>,
            FindClosestMatchingMode1: FindClosestMatchingMode1::<Identity, OFFSET>,
            GetDisplaySurfaceData1: GetDisplaySurfaceData1::<Identity, OFFSET>,
            DuplicateOutput: DuplicateOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput2_Impl: Sized + IDXGIOutput1_Impl {
    fn SupportsOverlays(&self) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGIOutput2 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIOutput2_Vtbl
    where
        Identity: IDXGIOutput2_Impl,
    {
        unsafe extern "system" fn SupportsOverlays<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDXGIOutput2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutput2_Impl::SupportsOverlays(this)
        }
        Self { base__: IDXGIOutput1_Vtbl::new::<Identity, OFFSET>(), SupportsOverlays: SupportsOverlays::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIOutput1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput3_Impl: Sized + IDXGIOutput2_Impl {
    fn CheckOverlaySupport(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: Option<&windows_core::IUnknown>) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGIOutput3 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIOutput3_Vtbl
    where
        Identity: IDXGIOutput3_Impl,
    {
        unsafe extern "system" fn CheckOverlaySupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumformat: Common::DXGI_FORMAT, pconcerneddevice: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIOutput3_Impl::CheckOverlaySupport(this, core::mem::transmute_copy(&enumformat), windows_core::from_raw_borrowed(&pconcerneddevice)) {
                Ok(ok__) => {
                    pflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IDXGIOutput2_Vtbl::new::<Identity, OFFSET>(), CheckOverlaySupport: CheckOverlaySupport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput3 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIOutput1 as windows_core::Interface>::IID || iid == &<IDXGIOutput2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput4_Impl: Sized + IDXGIOutput3_Impl {
    fn CheckOverlayColorSpaceSupport(&self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: Option<&windows_core::IUnknown>) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGIOutput4 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIOutput4_Vtbl
    where
        Identity: IDXGIOutput4_Impl,
    {
        unsafe extern "system" fn CheckOverlayColorSpaceSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIOutput4_Impl::CheckOverlayColorSpaceSupport(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&colorspace), windows_core::from_raw_borrowed(&pconcerneddevice)) {
                Ok(ok__) => {
                    pflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IDXGIOutput3_Vtbl::new::<Identity, OFFSET>(), CheckOverlayColorSpaceSupport: CheckOverlayColorSpaceSupport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput4 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIOutput1 as windows_core::Interface>::IID || iid == &<IDXGIOutput2 as windows_core::Interface>::IID || iid == &<IDXGIOutput3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput5_Impl: Sized + IDXGIOutput4_Impl {
    fn DuplicateOutput1(&self, pdevice: Option<&windows_core::IUnknown>, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT) -> windows_core::Result<IDXGIOutputDuplication>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGIOutput5 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIOutput5_Vtbl
    where
        Identity: IDXGIOutput5_Impl,
    {
        unsafe extern "system" fn DuplicateOutput1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT, ppoutputduplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIOutput5_Impl::DuplicateOutput1(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&supportedformatscount), core::mem::transmute_copy(&psupportedformats)) {
                Ok(ok__) => {
                    ppoutputduplication.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IDXGIOutput4_Vtbl::new::<Identity, OFFSET>(), DuplicateOutput1: DuplicateOutput1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput5 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIOutput1 as windows_core::Interface>::IID || iid == &<IDXGIOutput2 as windows_core::Interface>::IID || iid == &<IDXGIOutput3 as windows_core::Interface>::IID || iid == &<IDXGIOutput4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput6_Impl: Sized + IDXGIOutput5_Impl {
    fn GetDesc1(&self) -> windows_core::Result<DXGI_OUTPUT_DESC1>;
    fn CheckHardwareCompositionSupport(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGIOutput6 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput6_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIOutput6_Vtbl
    where
        Identity: IDXGIOutput6_Impl,
    {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC1) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIOutput6_Impl::GetDesc1(this) {
                Ok(ok__) => {
                    pdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHardwareCompositionSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGIOutput6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIOutput6_Impl::CheckHardwareCompositionSupport(this) {
                Ok(ok__) => {
                    pflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIOutput5_Vtbl::new::<Identity, OFFSET>(),
            GetDesc1: GetDesc1::<Identity, OFFSET>,
            CheckHardwareCompositionSupport: CheckHardwareCompositionSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput6 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIOutput1 as windows_core::Interface>::IID || iid == &<IDXGIOutput2 as windows_core::Interface>::IID || iid == &<IDXGIOutput3 as windows_core::Interface>::IID || iid == &<IDXGIOutput4 as windows_core::Interface>::IID || iid == &<IDXGIOutput5 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIOutputDuplication_Impl: Sized + IDXGIObject_Impl {
    fn GetDesc(&self, pdesc: *mut DXGI_OUTDUPL_DESC);
    fn AcquireNextFrame(&self, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut Option<IDXGIResource>) -> windows_core::Result<()>;
    fn GetFrameDirtyRects(&self, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> windows_core::Result<()>;
    fn GetFrameMoveRects(&self, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> windows_core::Result<()>;
    fn GetFramePointerShape(&self, pointershapebuffersize: u32, ppointershapebuffer: *mut core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> windows_core::Result<()>;
    fn MapDesktopSurface(&self) -> windows_core::Result<DXGI_MAPPED_RECT>;
    fn UnMapDesktopSurface(&self) -> windows_core::Result<()>;
    fn ReleaseFrame(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIOutputDuplication {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIOutputDuplication_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIOutputDuplication_Vtbl
    where
        Identity: IDXGIOutputDuplication_Impl,
    {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_OUTDUPL_DESC)
        where
            Identity: IDXGIOutputDuplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutputDuplication_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        unsafe extern "system" fn AcquireNextFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIOutputDuplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutputDuplication_Impl::AcquireNextFrame(this, core::mem::transmute_copy(&timeoutinmilliseconds), core::mem::transmute_copy(&pframeinfo), core::mem::transmute_copy(&ppdesktopresource)).into()
        }
        unsafe extern "system" fn GetFrameDirtyRects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGIOutputDuplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutputDuplication_Impl::GetFrameDirtyRects(this, core::mem::transmute_copy(&dirtyrectsbuffersize), core::mem::transmute_copy(&pdirtyrectsbuffer), core::mem::transmute_copy(&pdirtyrectsbuffersizerequired)).into()
        }
        unsafe extern "system" fn GetFrameMoveRects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGIOutputDuplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutputDuplication_Impl::GetFrameMoveRects(this, core::mem::transmute_copy(&moverectsbuffersize), core::mem::transmute_copy(&pmoverectbuffer), core::mem::transmute_copy(&pmoverectsbuffersizerequired)).into()
        }
        unsafe extern "system" fn GetFramePointerShape<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointershapebuffersize: u32, ppointershapebuffer: *mut core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> windows_core::HRESULT
        where
            Identity: IDXGIOutputDuplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutputDuplication_Impl::GetFramePointerShape(this, core::mem::transmute_copy(&pointershapebuffersize), core::mem::transmute_copy(&ppointershapebuffer), core::mem::transmute_copy(&ppointershapebuffersizerequired), core::mem::transmute_copy(&ppointershapeinfo)).into()
        }
        unsafe extern "system" fn MapDesktopSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT) -> windows_core::HRESULT
        where
            Identity: IDXGIOutputDuplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIOutputDuplication_Impl::MapDesktopSurface(this) {
                Ok(ok__) => {
                    plockedrect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnMapDesktopSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIOutputDuplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutputDuplication_Impl::UnMapDesktopSurface(this).into()
        }
        unsafe extern "system" fn ReleaseFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIOutputDuplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIOutputDuplication_Impl::ReleaseFrame(this).into()
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            AcquireNextFrame: AcquireNextFrame::<Identity, OFFSET>,
            GetFrameDirtyRects: GetFrameDirtyRects::<Identity, OFFSET>,
            GetFrameMoveRects: GetFrameMoveRects::<Identity, OFFSET>,
            GetFramePointerShape: GetFramePointerShape::<Identity, OFFSET>,
            MapDesktopSurface: MapDesktopSurface::<Identity, OFFSET>,
            UnMapDesktopSurface: UnMapDesktopSurface::<Identity, OFFSET>,
            ReleaseFrame: ReleaseFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutputDuplication as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
pub trait IDXGIResource_Impl: Sized + IDXGIDeviceSubObject_Impl {
    fn GetSharedHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn GetUsage(&self) -> windows_core::Result<DXGI_USAGE>;
    fn SetEvictionPriority(&self, evictionpriority: DXGI_RESOURCE_PRIORITY) -> windows_core::Result<()>;
    fn GetEvictionPriority(&self) -> windows_core::Result<DXGI_RESOURCE_PRIORITY>;
}
impl windows_core::RuntimeName for IDXGIResource {}
impl IDXGIResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIResource_Vtbl
    where
        Identity: IDXGIResource_Impl,
    {
        unsafe extern "system" fn GetSharedHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDXGIResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIResource_Impl::GetSharedHandle(this) {
                Ok(ok__) => {
                    psharedhandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusage: *mut DXGI_USAGE) -> windows_core::HRESULT
        where
            Identity: IDXGIResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIResource_Impl::GetUsage(this) {
                Ok(ok__) => {
                    pusage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEvictionPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, evictionpriority: DXGI_RESOURCE_PRIORITY) -> windows_core::HRESULT
        where
            Identity: IDXGIResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGIResource_Impl::SetEvictionPriority(this, core::mem::transmute_copy(&evictionpriority)).into()
        }
        unsafe extern "system" fn GetEvictionPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevictionpriority: *mut DXGI_RESOURCE_PRIORITY) -> windows_core::HRESULT
        where
            Identity: IDXGIResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIResource_Impl::GetEvictionPriority(this) {
                Ok(ok__) => {
                    pevictionpriority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, OFFSET>(),
            GetSharedHandle: GetSharedHandle::<Identity, OFFSET>,
            GetUsage: GetUsage::<Identity, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIResource as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Security")]
pub trait IDXGIResource1_Impl: Sized + IDXGIResource_Impl {
    fn CreateSubresourceSurface(&self, index: u32) -> windows_core::Result<IDXGISurface2>;
    fn CreateSharedHandle(&self, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Security")]
impl windows_core::RuntimeName for IDXGIResource1 {}
#[cfg(feature = "Win32_Security")]
impl IDXGIResource1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGIResource1_Vtbl
    where
        Identity: IDXGIResource1_Impl,
    {
        unsafe extern "system" fn CreateSubresourceSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGIResource1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIResource1_Impl::CreateSubresourceSurface(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    ppsurface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSharedHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: windows_core::PCWSTR, phandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDXGIResource1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGIResource1_Impl::CreateSharedHandle(this, core::mem::transmute_copy(&pattributes), core::mem::transmute_copy(&dwaccess), core::mem::transmute(&lpname)) {
                Ok(ok__) => {
                    phandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIResource_Vtbl::new::<Identity, OFFSET>(),
            CreateSubresourceSurface: CreateSubresourceSurface::<Identity, OFFSET>,
            CreateSharedHandle: CreateSharedHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIResource1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGIResource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISurface_Impl: Sized + IDXGIDeviceSubObject_Impl {
    fn GetDesc(&self) -> windows_core::Result<DXGI_SURFACE_DESC>;
    fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: DXGI_MAP_FLAGS) -> windows_core::Result<()>;
    fn Unmap(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISurface {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISurface_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGISurface_Vtbl
    where
        Identity: IDXGISurface_Impl,
    {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SURFACE_DESC) -> windows_core::HRESULT
        where
            Identity: IDXGISurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGISurface_Impl::GetDesc(this) {
                Ok(ok__) => {
                    pdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Map<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: DXGI_MAP_FLAGS) -> windows_core::HRESULT
        where
            Identity: IDXGISurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISurface_Impl::Map(this, core::mem::transmute_copy(&plockedrect), core::mem::transmute_copy(&mapflags)).into()
        }
        unsafe extern "system" fn Unmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGISurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISurface_Impl::Unmap(this).into()
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISurface as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGISurface1_Impl: Sized + IDXGISurface_Impl {
    fn GetDC(&self, discard: super::super::Foundation::BOOL) -> windows_core::Result<super::Gdi::HDC>;
    fn ReleaseDC(&self, pdirtyrect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGISurface1 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGISurface1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGISurface1_Vtbl
    where
        Identity: IDXGISurface1_Impl,
    {
        unsafe extern "system" fn GetDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, discard: super::super::Foundation::BOOL, phdc: *mut super::Gdi::HDC) -> windows_core::HRESULT
        where
            Identity: IDXGISurface1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGISurface1_Impl::GetDC(this, core::mem::transmute_copy(&discard)) {
                Ok(ok__) => {
                    phdc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDXGISurface1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISurface1_Impl::ReleaseDC(this, core::mem::transmute_copy(&pdirtyrect)).into()
        }
        Self { base__: IDXGISurface_Vtbl::new::<Identity, OFFSET>(), GetDC: GetDC::<Identity, OFFSET>, ReleaseDC: ReleaseDC::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISurface1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISurface as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGISurface2_Impl: Sized + IDXGISurface1_Impl {
    fn GetResource(&self, riid: *const windows_core::GUID, ppparentresource: *mut *mut core::ffi::c_void, psubresourceindex: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGISurface2 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGISurface2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGISurface2_Vtbl
    where
        Identity: IDXGISurface2_Impl,
    {
        unsafe extern "system" fn GetResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppparentresource: *mut *mut core::ffi::c_void, psubresourceindex: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGISurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISurface2_Impl::GetResource(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppparentresource), core::mem::transmute_copy(&psubresourceindex)).into()
        }
        Self { base__: IDXGISurface1_Vtbl::new::<Identity, OFFSET>(), GetResource: GetResource::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISurface2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISurface as windows_core::Interface>::IID || iid == &<IDXGISurface1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISwapChain_Impl: Sized + IDXGIDeviceSubObject_Impl {
    fn Present(&self, syncinterval: u32, flags: DXGI_PRESENT) -> windows_core::HRESULT;
    fn GetBuffer(&self, buffer: u32, riid: *const windows_core::GUID, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetFullscreenState(&self, fullscreen: super::super::Foundation::BOOL, ptarget: Option<&IDXGIOutput>) -> windows_core::Result<()>;
    fn GetFullscreenState(&self, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut Option<IDXGIOutput>) -> windows_core::Result<()>;
    fn GetDesc(&self) -> windows_core::Result<DXGI_SWAP_CHAIN_DESC>;
    fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: &DXGI_SWAP_CHAIN_FLAG) -> windows_core::Result<()>;
    fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> windows_core::Result<()>;
    fn GetContainingOutput(&self) -> windows_core::Result<IDXGIOutput>;
    fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::Result<()>;
    fn GetLastPresentCount(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISwapChain {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISwapChain_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGISwapChain_Vtbl
    where
        Identity: IDXGISwapChain_Impl,
    {
        unsafe extern "system" fn Present<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncinterval: u32, flags: DXGI_PRESENT) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain_Impl::Present(this, core::mem::transmute_copy(&syncinterval), core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn GetBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: u32, riid: *const windows_core::GUID, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain_Impl::GetBuffer(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppsurface)).into()
        }
        unsafe extern "system" fn SetFullscreenState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fullscreen: super::super::Foundation::BOOL, ptarget: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain_Impl::SetFullscreenState(this, core::mem::transmute_copy(&fullscreen), windows_core::from_raw_borrowed(&ptarget)).into()
        }
        unsafe extern "system" fn GetFullscreenState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain_Impl::GetFullscreenState(this, core::mem::transmute_copy(&pfullscreen), core::mem::transmute_copy(&pptarget)).into()
        }
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGISwapChain_Impl::GetDesc(this) {
                Ok(ok__) => {
                    pdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResizeBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain_Impl::ResizeBuffers(this, core::mem::transmute_copy(&buffercount), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&newformat), core::mem::transmute(&swapchainflags)).into()
        }
        unsafe extern "system" fn ResizeTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain_Impl::ResizeTarget(this, core::mem::transmute_copy(&pnewtargetparameters)).into()
        }
        unsafe extern "system" fn GetContainingOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGISwapChain_Impl::GetContainingOutput(this) {
                Ok(ok__) => {
                    ppoutput.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain_Impl::GetFrameStatistics(this, core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn GetLastPresentCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastpresentcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGISwapChain_Impl::GetLastPresentCount(this) {
                Ok(ok__) => {
                    plastpresentcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, OFFSET>(),
            Present: Present::<Identity, OFFSET>,
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            SetFullscreenState: SetFullscreenState::<Identity, OFFSET>,
            GetFullscreenState: GetFullscreenState::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            ResizeBuffers: ResizeBuffers::<Identity, OFFSET>,
            ResizeTarget: ResizeTarget::<Identity, OFFSET>,
            GetContainingOutput: GetContainingOutput::<Identity, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, OFFSET>,
            GetLastPresentCount: GetLastPresentCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISwapChain1_Impl: Sized + IDXGISwapChain_Impl {
    fn GetDesc1(&self) -> windows_core::Result<DXGI_SWAP_CHAIN_DESC1>;
    fn GetFullscreenDesc(&self) -> windows_core::Result<DXGI_SWAP_CHAIN_FULLSCREEN_DESC>;
    fn GetHwnd(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn GetCoreWindow(&self, refiid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Present1(&self, syncinterval: u32, presentflags: DXGI_PRESENT, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> windows_core::HRESULT;
    fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL;
    fn GetRestrictToOutput(&self) -> windows_core::Result<IDXGIOutput>;
    fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> windows_core::Result<()>;
    fn GetBackgroundColor(&self) -> windows_core::Result<DXGI_RGBA>;
    fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> windows_core::Result<()>;
    fn GetRotation(&self) -> windows_core::Result<Common::DXGI_MODE_ROTATION>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISwapChain1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISwapChain1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGISwapChain1_Vtbl
    where
        Identity: IDXGISwapChain1_Impl,
    {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGISwapChain1_Impl::GetDesc1(this) {
                Ok(ok__) => {
                    pdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullscreenDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGISwapChain1_Impl::GetFullscreenDesc(this) {
                Ok(ok__) => {
                    pdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHwnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGISwapChain1_Impl::GetHwnd(this) {
                Ok(ok__) => {
                    phwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCoreWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, refiid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain1_Impl::GetCoreWindow(this, core::mem::transmute_copy(&refiid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn Present1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncinterval: u32, presentflags: DXGI_PRESENT, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain1_Impl::Present1(this, core::mem::transmute_copy(&syncinterval), core::mem::transmute_copy(&presentflags), core::mem::transmute_copy(&ppresentparameters))
        }
        unsafe extern "system" fn IsTemporaryMonoSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDXGISwapChain1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain1_Impl::IsTemporaryMonoSupported(this)
        }
        unsafe extern "system" fn GetRestrictToOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprestricttooutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGISwapChain1_Impl::GetRestrictToOutput(this) {
                Ok(ok__) => {
                    pprestricttooutput.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *const DXGI_RGBA) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain1_Impl::SetBackgroundColor(this, core::mem::transmute_copy(&pcolor)).into()
        }
        unsafe extern "system" fn GetBackgroundColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut DXGI_RGBA) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGISwapChain1_Impl::GetBackgroundColor(this) {
                Ok(ok__) => {
                    pcolor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotation: Common::DXGI_MODE_ROTATION) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain1_Impl::SetRotation(this, core::mem::transmute_copy(&rotation)).into()
        }
        unsafe extern "system" fn GetRotation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, protation: *mut Common::DXGI_MODE_ROTATION) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGISwapChain1_Impl::GetRotation(this) {
                Ok(ok__) => {
                    protation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGISwapChain_Vtbl::new::<Identity, OFFSET>(),
            GetDesc1: GetDesc1::<Identity, OFFSET>,
            GetFullscreenDesc: GetFullscreenDesc::<Identity, OFFSET>,
            GetHwnd: GetHwnd::<Identity, OFFSET>,
            GetCoreWindow: GetCoreWindow::<Identity, OFFSET>,
            Present1: Present1::<Identity, OFFSET>,
            IsTemporaryMonoSupported: IsTemporaryMonoSupported::<Identity, OFFSET>,
            GetRestrictToOutput: GetRestrictToOutput::<Identity, OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Identity, OFFSET>,
            GetBackgroundColor: GetBackgroundColor::<Identity, OFFSET>,
            SetRotation: SetRotation::<Identity, OFFSET>,
            GetRotation: GetRotation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISwapChain as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISwapChain2_Impl: Sized + IDXGISwapChain1_Impl {
    fn SetSourceSize(&self, width: u32, height: u32) -> windows_core::Result<()>;
    fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> windows_core::Result<()>;
    fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()>;
    fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32>;
    fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE;
    fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> windows_core::Result<()>;
    fn GetMatrixTransform(&self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISwapChain2 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISwapChain2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGISwapChain2_Vtbl
    where
        Identity: IDXGISwapChain2_Impl,
    {
        unsafe extern "system" fn SetSourceSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain2_Impl::SetSourceSize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn GetSourceSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain2_Impl::GetSourceSize(this, core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlatency: u32) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain2_Impl::SetMaximumFrameLatency(this, core::mem::transmute_copy(&maxlatency)).into()
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxlatency: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGISwapChain2_Impl::GetMaximumFrameLatency(this) {
                Ok(ok__) => {
                    pmaxlatency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameLatencyWaitableObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE
        where
            Identity: IDXGISwapChain2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain2_Impl::GetFrameLatencyWaitableObject(this)
        }
        unsafe extern "system" fn SetMatrixTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *const DXGI_MATRIX_3X2_F) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain2_Impl::SetMatrixTransform(this, core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn GetMatrixTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *mut DXGI_MATRIX_3X2_F) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain2_Impl::GetMatrixTransform(this, core::mem::transmute_copy(&pmatrix)).into()
        }
        Self {
            base__: IDXGISwapChain1_Vtbl::new::<Identity, OFFSET>(),
            SetSourceSize: SetSourceSize::<Identity, OFFSET>,
            GetSourceSize: GetSourceSize::<Identity, OFFSET>,
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, OFFSET>,
            GetFrameLatencyWaitableObject: GetFrameLatencyWaitableObject::<Identity, OFFSET>,
            SetMatrixTransform: SetMatrixTransform::<Identity, OFFSET>,
            GetMatrixTransform: GetMatrixTransform::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISwapChain as windows_core::Interface>::IID || iid == &<IDXGISwapChain1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISwapChain3_Impl: Sized + IDXGISwapChain2_Impl {
    fn GetCurrentBackBufferIndex(&self) -> u32;
    fn CheckColorSpaceSupport(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<u32>;
    fn SetColorSpace1(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<()>;
    fn ResizeBuffers1(&self, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: &DXGI_SWAP_CHAIN_FLAG, pcreationnodemask: *const u32, pppresentqueue: *const Option<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISwapChain3 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISwapChain3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGISwapChain3_Vtbl
    where
        Identity: IDXGISwapChain3_Impl,
    {
        unsafe extern "system" fn GetCurrentBackBufferIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDXGISwapChain3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain3_Impl::GetCurrentBackBufferIndex(this)
        }
        unsafe extern "system" fn CheckColorSpaceSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pcolorspacesupport: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDXGISwapChain3_Impl::CheckColorSpaceSupport(this, core::mem::transmute_copy(&colorspace)) {
                Ok(ok__) => {
                    pcolorspacesupport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorSpace1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain3_Impl::SetColorSpace1(this, core::mem::transmute_copy(&colorspace)).into()
        }
        unsafe extern "system" fn ResizeBuffers1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain3_Impl::ResizeBuffers1(this, core::mem::transmute_copy(&buffercount), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute(&swapchainflags), core::mem::transmute_copy(&pcreationnodemask), core::mem::transmute_copy(&pppresentqueue)).into()
        }
        Self {
            base__: IDXGISwapChain2_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentBackBufferIndex: GetCurrentBackBufferIndex::<Identity, OFFSET>,
            CheckColorSpaceSupport: CheckColorSpaceSupport::<Identity, OFFSET>,
            SetColorSpace1: SetColorSpace1::<Identity, OFFSET>,
            ResizeBuffers1: ResizeBuffers1::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain3 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISwapChain as windows_core::Interface>::IID || iid == &<IDXGISwapChain1 as windows_core::Interface>::IID || iid == &<IDXGISwapChain2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISwapChain4_Impl: Sized + IDXGISwapChain3_Impl {
    fn SetHDRMetaData(&self, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISwapChain4 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISwapChain4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGISwapChain4_Vtbl
    where
        Identity: IDXGISwapChain4_Impl,
    {
        unsafe extern "system" fn SetHDRMetaData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChain4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChain4_Impl::SetHDRMetaData(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&size), core::mem::transmute_copy(&pmetadata)).into()
        }
        Self { base__: IDXGISwapChain3_Vtbl::new::<Identity, OFFSET>(), SetHDRMetaData: SetHDRMetaData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain4 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISwapChain as windows_core::Interface>::IID || iid == &<IDXGISwapChain1 as windows_core::Interface>::IID || iid == &<IDXGISwapChain2 as windows_core::Interface>::IID || iid == &<IDXGISwapChain3 as windows_core::Interface>::IID
    }
}
pub trait IDXGISwapChainMedia_Impl: Sized {
    fn GetFrameStatisticsMedia(&self, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> windows_core::Result<()>;
    fn SetPresentDuration(&self, duration: u32) -> windows_core::Result<()>;
    fn CheckPresentDurationSupport(&self, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDXGISwapChainMedia {}
impl IDXGISwapChainMedia_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGISwapChainMedia_Vtbl
    where
        Identity: IDXGISwapChainMedia_Impl,
    {
        unsafe extern "system" fn GetFrameStatisticsMedia<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChainMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChainMedia_Impl::GetFrameStatisticsMedia(this, core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn SetPresentDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: u32) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChainMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChainMedia_Impl::SetPresentDuration(this, core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn CheckPresentDurationSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDXGISwapChainMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGISwapChainMedia_Impl::CheckPresentDurationSupport(this, core::mem::transmute_copy(&desiredpresentduration), core::mem::transmute_copy(&pclosestsmallerpresentduration), core::mem::transmute_copy(&pclosestlargerpresentduration)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFrameStatisticsMedia: GetFrameStatisticsMedia::<Identity, OFFSET>,
            SetPresentDuration: SetPresentDuration::<Identity, OFFSET>,
            CheckPresentDurationSupport: CheckPresentDurationSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChainMedia as windows_core::Interface>::IID
    }
}
pub trait IDXGraphicsAnalysis_Impl: Sized {
    fn BeginCapture(&self);
    fn EndCapture(&self);
}
impl windows_core::RuntimeName for IDXGraphicsAnalysis {}
impl IDXGraphicsAnalysis_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDXGraphicsAnalysis_Vtbl
    where
        Identity: IDXGraphicsAnalysis_Impl,
    {
        unsafe extern "system" fn BeginCapture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IDXGraphicsAnalysis_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGraphicsAnalysis_Impl::BeginCapture(this)
        }
        unsafe extern "system" fn EndCapture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IDXGraphicsAnalysis_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDXGraphicsAnalysis_Impl::EndCapture(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginCapture: BeginCapture::<Identity, OFFSET>,
            EndCapture: EndCapture::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGraphicsAnalysis as windows_core::Interface>::IID
    }
}
