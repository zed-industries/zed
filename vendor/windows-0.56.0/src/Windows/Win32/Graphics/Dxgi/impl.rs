pub trait IDXGIAdapter_Impl: Sized + IDXGIObject_Impl {
    fn EnumOutputs(&self, output: u32) -> windows_core::Result<IDXGIOutput>;
    fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> windows_core::Result<()>;
    fn CheckInterfaceSupport(&self, interfacename: *const windows_core::GUID) -> windows_core::Result<i64>;
}
impl windows_core::RuntimeName for IDXGIAdapter {}
impl IDXGIAdapter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter_Impl, const OFFSET: isize>() -> IDXGIAdapter_Vtbl {
        unsafe extern "system" fn EnumOutputs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, output: u32, ppoutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIAdapter_Impl::EnumOutputs(this, core::mem::transmute_copy(&output)) {
                Ok(ok__) => {
                    core::ptr::write(ppoutput, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIAdapter_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn CheckInterfaceSupport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacename: *const windows_core::GUID, pumdversion: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIAdapter_Impl::CheckInterfaceSupport(this, core::mem::transmute_copy(&interfacename)) {
                Ok(ok__) => {
                    core::ptr::write(pumdversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumOutputs: EnumOutputs::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            CheckInterfaceSupport: CheckInterfaceSupport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
pub trait IDXGIAdapter1_Impl: Sized + IDXGIAdapter_Impl {
    fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDXGIAdapter1 {}
impl IDXGIAdapter1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter1_Impl, const OFFSET: isize>() -> IDXGIAdapter1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC1) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIAdapter1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc)).into()
        }
        Self { base__: IDXGIAdapter_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIAdapter as windows_core::Interface>::IID
    }
}
pub trait IDXGIAdapter2_Impl: Sized + IDXGIAdapter1_Impl {
    fn GetDesc2(&self, pdesc: *mut DXGI_ADAPTER_DESC2) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDXGIAdapter2 {}
impl IDXGIAdapter2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter2_Impl, const OFFSET: isize>() -> IDXGIAdapter2_Vtbl {
        unsafe extern "system" fn GetDesc2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC2) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIAdapter2_Impl::GetDesc2(this, core::mem::transmute_copy(&pdesc)).into()
        }
        Self { base__: IDXGIAdapter1_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc2: GetDesc2::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>() -> IDXGIAdapter3_Vtbl {
        unsafe extern "system" fn RegisterHardwareContentProtectionTeardownStatusEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIAdapter3_Impl::RegisterHardwareContentProtectionTeardownStatusEvent(this, core::mem::transmute_copy(&hevent)) {
                Ok(ok__) => {
                    core::ptr::write(pdwcookie, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterHardwareContentProtectionTeardownStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIAdapter3_Impl::UnregisterHardwareContentProtectionTeardownStatus(this, core::mem::transmute_copy(&dwcookie))
        }
        unsafe extern "system" fn QueryVideoMemoryInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIAdapter3_Impl::QueryVideoMemoryInfo(this, core::mem::transmute_copy(&nodeindex), core::mem::transmute_copy(&memorysegmentgroup), core::mem::transmute_copy(&pvideomemoryinfo)).into()
        }
        unsafe extern "system" fn SetVideoMemoryReservation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIAdapter3_Impl::SetVideoMemoryReservation(this, core::mem::transmute_copy(&nodeindex), core::mem::transmute_copy(&memorysegmentgroup), core::mem::transmute_copy(&reservation)).into()
        }
        unsafe extern "system" fn RegisterVideoMemoryBudgetChangeNotificationEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIAdapter3_Impl::RegisterVideoMemoryBudgetChangeNotificationEvent(this, core::mem::transmute_copy(&hevent)) {
                Ok(ok__) => {
                    core::ptr::write(pdwcookie, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterVideoMemoryBudgetChangeNotification<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIAdapter3_Impl::UnregisterVideoMemoryBudgetChangeNotification(this, core::mem::transmute_copy(&dwcookie))
        }
        Self {
            base__: IDXGIAdapter2_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegisterHardwareContentProtectionTeardownStatusEvent: RegisterHardwareContentProtectionTeardownStatusEvent::<Identity, Impl, OFFSET>,
            UnregisterHardwareContentProtectionTeardownStatus: UnregisterHardwareContentProtectionTeardownStatus::<Identity, Impl, OFFSET>,
            QueryVideoMemoryInfo: QueryVideoMemoryInfo::<Identity, Impl, OFFSET>,
            SetVideoMemoryReservation: SetVideoMemoryReservation::<Identity, Impl, OFFSET>,
            RegisterVideoMemoryBudgetChangeNotificationEvent: RegisterVideoMemoryBudgetChangeNotificationEvent::<Identity, Impl, OFFSET>,
            UnregisterVideoMemoryBudgetChangeNotification: UnregisterVideoMemoryBudgetChangeNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter3 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIAdapter as windows_core::Interface>::IID || iid == &<IDXGIAdapter1 as windows_core::Interface>::IID || iid == &<IDXGIAdapter2 as windows_core::Interface>::IID
    }
}
pub trait IDXGIAdapter4_Impl: Sized + IDXGIAdapter3_Impl {
    fn GetDesc3(&self, pdesc: *mut DXGI_ADAPTER_DESC3) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDXGIAdapter4 {}
impl IDXGIAdapter4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter4_Impl, const OFFSET: isize>() -> IDXGIAdapter4_Vtbl {
        unsafe extern "system" fn GetDesc3<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC3) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIAdapter4_Impl::GetDesc3(this, core::mem::transmute_copy(&pdesc)).into()
        }
        Self { base__: IDXGIAdapter3_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc3: GetDesc3::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDebug_Impl, const OFFSET: isize>() -> IDXGIDebug_Vtbl {
        unsafe extern "system" fn ReportLiveObjects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, apiid: windows_core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDebug_Impl::ReportLiveObjects(this, core::mem::transmute(&apiid), core::mem::transmute_copy(&flags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ReportLiveObjects: ReportLiveObjects::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDebug1_Impl, const OFFSET: isize>() -> IDXGIDebug1_Vtbl {
        unsafe extern "system" fn EnableLeakTrackingForThread<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDebug1_Impl::EnableLeakTrackingForThread(this)
        }
        unsafe extern "system" fn DisableLeakTrackingForThread<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDebug1_Impl::DisableLeakTrackingForThread(this)
        }
        unsafe extern "system" fn IsLeakTrackingEnabledForThread<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDebug1_Impl::IsLeakTrackingEnabledForThread(this)
        }
        Self {
            base__: IDXGIDebug_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnableLeakTrackingForThread: EnableLeakTrackingForThread::<Identity, Impl, OFFSET>,
            DisableLeakTrackingForThread: DisableLeakTrackingForThread::<Identity, Impl, OFFSET>,
            IsLeakTrackingEnabledForThread: IsLeakTrackingEnabledForThread::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDebug1 as windows_core::Interface>::IID || iid == &<IDXGIDebug as windows_core::Interface>::IID
    }
}
pub trait IDXGIDecodeSwapChain_Impl: Sized {
    fn PresentBuffer(&self, buffertopresent: u32, syncinterval: u32, flags: u32) -> windows_core::HRESULT;
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>() -> IDXGIDecodeSwapChain_Vtbl {
        unsafe extern "system" fn PresentBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffertopresent: u32, syncinterval: u32, flags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDecodeSwapChain_Impl::PresentBuffer(this, core::mem::transmute_copy(&buffertopresent), core::mem::transmute_copy(&syncinterval), core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn SetSourceRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDecodeSwapChain_Impl::SetSourceRect(this, core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn SetTargetRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDecodeSwapChain_Impl::SetTargetRect(this, core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn SetDestSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDecodeSwapChain_Impl::SetDestSize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn GetSourceRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIDecodeSwapChain_Impl::GetSourceRect(this) {
                Ok(ok__) => {
                    core::ptr::write(prect, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIDecodeSwapChain_Impl::GetTargetRect(this) {
                Ok(ok__) => {
                    core::ptr::write(prect, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDecodeSwapChain_Impl::GetDestSize(this, core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
        }
        unsafe extern "system" fn SetColorSpace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDecodeSwapChain_Impl::SetColorSpace(this, core::mem::transmute_copy(&colorspace)).into()
        }
        unsafe extern "system" fn GetColorSpace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDecodeSwapChain_Impl::GetColorSpace(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PresentBuffer: PresentBuffer::<Identity, Impl, OFFSET>,
            SetSourceRect: SetSourceRect::<Identity, Impl, OFFSET>,
            SetTargetRect: SetTargetRect::<Identity, Impl, OFFSET>,
            SetDestSize: SetDestSize::<Identity, Impl, OFFSET>,
            GetSourceRect: GetSourceRect::<Identity, Impl, OFFSET>,
            GetTargetRect: GetTargetRect::<Identity, Impl, OFFSET>,
            GetDestSize: GetDestSize::<Identity, Impl, OFFSET>,
            SetColorSpace: SetColorSpace::<Identity, Impl, OFFSET>,
            GetColorSpace: GetColorSpace::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: isize>() -> IDXGIDevice_Vtbl {
        unsafe extern "system" fn GetAdapter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, padapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIDevice_Impl::GetAdapter(this) {
                Ok(ok__) => {
                    core::ptr::write(padapter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: DXGI_USAGE, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDevice_Impl::CreateSurface(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&numsurfaces), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&psharedresource), core::mem::transmute_copy(&ppsurface)).into()
        }
        unsafe extern "system" fn QueryResourceResidency<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDevice_Impl::QueryResourceResidency(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&presidencystatus), core::mem::transmute_copy(&numresources)).into()
        }
        unsafe extern "system" fn SetGPUThreadPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDevice_Impl::SetGPUThreadPriority(this, core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn GetGPUThreadPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIDevice_Impl::GetGPUThreadPriority(this) {
                Ok(ok__) => {
                    core::ptr::write(ppriority, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAdapter: GetAdapter::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            QueryResourceResidency: QueryResourceResidency::<Identity, Impl, OFFSET>,
            SetGPUThreadPriority: SetGPUThreadPriority::<Identity, Impl, OFFSET>,
            GetGPUThreadPriority: GetGPUThreadPriority::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice1_Impl, const OFFSET: isize>() -> IDXGIDevice1_Vtbl {
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlatency: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDevice1_Impl::SetMaximumFrameLatency(this, core::mem::transmute_copy(&maxlatency)).into()
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxlatency: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIDevice1_Impl::GetMaximumFrameLatency(this) {
                Ok(ok__) => {
                    core::ptr::write(pmaxlatency, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIDevice_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, Impl, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice2_Impl, const OFFSET: isize>() -> IDXGIDevice2_Vtbl {
        unsafe extern "system" fn OfferResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDevice2_Impl::OfferResources(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn ReclaimResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, pdiscarded: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDevice2_Impl::ReclaimResources(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&pdiscarded)).into()
        }
        unsafe extern "system" fn EnqueueSetEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDevice2_Impl::EnqueueSetEvent(this, core::mem::transmute_copy(&hevent)).into()
        }
        Self {
            base__: IDXGIDevice1_Vtbl::new::<Identity, Impl, OFFSET>(),
            OfferResources: OfferResources::<Identity, Impl, OFFSET>,
            ReclaimResources: ReclaimResources::<Identity, Impl, OFFSET>,
            EnqueueSetEvent: EnqueueSetEvent::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice3_Impl, const OFFSET: isize>() -> IDXGIDevice3_Vtbl {
        unsafe extern "system" fn Trim<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDevice3_Impl::Trim(this)
        }
        Self { base__: IDXGIDevice2_Vtbl::new::<Identity, Impl, OFFSET>(), Trim: Trim::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice3 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDevice as windows_core::Interface>::IID || iid == &<IDXGIDevice1 as windows_core::Interface>::IID || iid == &<IDXGIDevice2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIDevice4_Impl: Sized + IDXGIDevice3_Impl {
    fn OfferResources1(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> windows_core::Result<()>;
    fn ReclaimResources1(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIDevice4 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIDevice4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice4_Impl, const OFFSET: isize>() -> IDXGIDevice4_Vtbl {
        unsafe extern "system" fn OfferResources1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDevice4_Impl::OfferResources1(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&priority), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn ReclaimResources1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDevice4_Impl::ReclaimResources1(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&presults)).into()
        }
        Self {
            base__: IDXGIDevice3_Vtbl::new::<Identity, Impl, OFFSET>(),
            OfferResources1: OfferResources1::<Identity, Impl, OFFSET>,
            ReclaimResources1: ReclaimResources1::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDeviceSubObject_Impl, const OFFSET: isize>() -> IDXGIDeviceSubObject_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDeviceSubObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDeviceSubObject_Impl::GetDevice(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppdevice)).into()
        }
        Self { base__: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(), GetDevice: GetDevice::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDisplayControl_Impl, const OFFSET: isize>() -> IDXGIDisplayControl_Vtbl {
        unsafe extern "system" fn IsStereoEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDisplayControl_Impl::IsStereoEnabled(this)
        }
        unsafe extern "system" fn SetStereoEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIDisplayControl_Impl::SetStereoEnabled(this, core::mem::transmute_copy(&enabled))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsStereoEnabled: IsStereoEnabled::<Identity, Impl, OFFSET>,
            SetStereoEnabled: SetStereoEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDisplayControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory_Impl: Sized + IDXGIObject_Impl {
    fn EnumAdapters(&self, adapter: u32) -> windows_core::Result<IDXGIAdapter>;
    fn MakeWindowAssociation(&self, windowhandle: super::super::Foundation::HWND, flags: u32) -> windows_core::Result<()>;
    fn GetWindowAssociation(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn CreateSwapChain(&self, pdevice: Option<&windows_core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut Option<IDXGISwapChain>) -> windows_core::HRESULT;
    fn CreateSoftwareAdapter(&self, module: super::super::Foundation::HMODULE) -> windows_core::Result<IDXGIAdapter>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: isize>() -> IDXGIFactory_Vtbl {
        unsafe extern "system" fn EnumAdapters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, ppadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIFactory_Impl::EnumAdapters(this, core::mem::transmute_copy(&adapter)) {
                Ok(ok__) => {
                    core::ptr::write(ppadapter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeWindowAssociation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowhandle: super::super::Foundation::HWND, flags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIFactory_Impl::MakeWindowAssociation(this, core::mem::transmute_copy(&windowhandle), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetWindowAssociation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwindowhandle: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIFactory_Impl::GetWindowAssociation(this) {
                Ok(ok__) => {
                    core::ptr::write(pwindowhandle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSwapChain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIFactory_Impl::CreateSwapChain(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppswapchain))
        }
        unsafe extern "system" fn CreateSoftwareAdapter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, module: super::super::Foundation::HMODULE, ppadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIFactory_Impl::CreateSoftwareAdapter(this, core::mem::transmute_copy(&module)) {
                Ok(ok__) => {
                    core::ptr::write(ppadapter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumAdapters: EnumAdapters::<Identity, Impl, OFFSET>,
            MakeWindowAssociation: MakeWindowAssociation::<Identity, Impl, OFFSET>,
            GetWindowAssociation: GetWindowAssociation::<Identity, Impl, OFFSET>,
            CreateSwapChain: CreateSwapChain::<Identity, Impl, OFFSET>,
            CreateSoftwareAdapter: CreateSoftwareAdapter::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory1_Impl, const OFFSET: isize>() -> IDXGIFactory1_Vtbl {
        unsafe extern "system" fn EnumAdapters1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, ppadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIFactory1_Impl::EnumAdapters1(this, core::mem::transmute_copy(&adapter)) {
                Ok(ok__) => {
                    core::ptr::write(ppadapter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIFactory1_Impl::IsCurrent(this)
        }
        Self {
            base__: IDXGIFactory_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumAdapters1: EnumAdapters1::<Identity, Impl, OFFSET>,
            IsCurrent: IsCurrent::<Identity, Impl, OFFSET>,
        }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>() -> IDXGIFactory2_Vtbl {
        unsafe extern "system" fn IsWindowedStereoEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIFactory2_Impl::IsWindowedStereoEnabled(this)
        }
        unsafe extern "system" fn CreateSwapChainForHwnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIFactory2_Impl::CreateSwapChainForHwnd(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pfullscreendesc), windows_core::from_raw_borrowed(&prestricttooutput)) {
                Ok(ok__) => {
                    core::ptr::write(ppswapchain, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSwapChainForCoreWindow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pwindow: *mut core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIFactory2_Impl::CreateSwapChainForCoreWindow(this, windows_core::from_raw_borrowed(&pdevice), windows_core::from_raw_borrowed(&pwindow), core::mem::transmute_copy(&pdesc), windows_core::from_raw_borrowed(&prestricttooutput)) {
                Ok(ok__) => {
                    core::ptr::write(ppswapchain, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSharedResourceAdapterLuid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresource: super::super::Foundation::HANDLE, pluid: *mut super::super::Foundation::LUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIFactory2_Impl::GetSharedResourceAdapterLuid(this, core::mem::transmute_copy(&hresource)) {
                Ok(ok__) => {
                    core::ptr::write(pluid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterStereoStatusWindow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIFactory2_Impl::RegisterStereoStatusWindow(this, core::mem::transmute_copy(&windowhandle), core::mem::transmute_copy(&wmsg)) {
                Ok(ok__) => {
                    core::ptr::write(pdwcookie, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterStereoStatusEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIFactory2_Impl::RegisterStereoStatusEvent(this, core::mem::transmute_copy(&hevent)) {
                Ok(ok__) => {
                    core::ptr::write(pdwcookie, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterStereoStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIFactory2_Impl::UnregisterStereoStatus(this, core::mem::transmute_copy(&dwcookie))
        }
        unsafe extern "system" fn RegisterOcclusionStatusWindow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIFactory2_Impl::RegisterOcclusionStatusWindow(this, core::mem::transmute_copy(&windowhandle), core::mem::transmute_copy(&wmsg)) {
                Ok(ok__) => {
                    core::ptr::write(pdwcookie, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterOcclusionStatusEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIFactory2_Impl::RegisterOcclusionStatusEvent(this, core::mem::transmute_copy(&hevent)) {
                Ok(ok__) => {
                    core::ptr::write(pdwcookie, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterOcclusionStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIFactory2_Impl::UnregisterOcclusionStatus(this, core::mem::transmute_copy(&dwcookie))
        }
        unsafe extern "system" fn CreateSwapChainForComposition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIFactory2_Impl::CreateSwapChainForComposition(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&pdesc), windows_core::from_raw_borrowed(&prestricttooutput)) {
                Ok(ok__) => {
                    core::ptr::write(ppswapchain, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIFactory1_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsWindowedStereoEnabled: IsWindowedStereoEnabled::<Identity, Impl, OFFSET>,
            CreateSwapChainForHwnd: CreateSwapChainForHwnd::<Identity, Impl, OFFSET>,
            CreateSwapChainForCoreWindow: CreateSwapChainForCoreWindow::<Identity, Impl, OFFSET>,
            GetSharedResourceAdapterLuid: GetSharedResourceAdapterLuid::<Identity, Impl, OFFSET>,
            RegisterStereoStatusWindow: RegisterStereoStatusWindow::<Identity, Impl, OFFSET>,
            RegisterStereoStatusEvent: RegisterStereoStatusEvent::<Identity, Impl, OFFSET>,
            UnregisterStereoStatus: UnregisterStereoStatus::<Identity, Impl, OFFSET>,
            RegisterOcclusionStatusWindow: RegisterOcclusionStatusWindow::<Identity, Impl, OFFSET>,
            RegisterOcclusionStatusEvent: RegisterOcclusionStatusEvent::<Identity, Impl, OFFSET>,
            UnregisterOcclusionStatus: UnregisterOcclusionStatus::<Identity, Impl, OFFSET>,
            CreateSwapChainForComposition: CreateSwapChainForComposition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory3_Impl: Sized + IDXGIFactory2_Impl {
    fn GetCreationFlags(&self) -> u32;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory3 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory3_Impl, const OFFSET: isize>() -> IDXGIFactory3_Vtbl {
        unsafe extern "system" fn GetCreationFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIFactory3_Impl::GetCreationFlags(this)
        }
        Self { base__: IDXGIFactory2_Vtbl::new::<Identity, Impl, OFFSET>(), GetCreationFlags: GetCreationFlags::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory4_Impl, const OFFSET: isize>() -> IDXGIFactory4_Vtbl {
        unsafe extern "system" fn EnumAdapterByLuid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapterluid: super::super::Foundation::LUID, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIFactory4_Impl::EnumAdapterByLuid(this, core::mem::transmute(&adapterluid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
        }
        unsafe extern "system" fn EnumWarpAdapter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIFactory4_Impl::EnumWarpAdapter(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
        }
        Self {
            base__: IDXGIFactory3_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumAdapterByLuid: EnumAdapterByLuid::<Identity, Impl, OFFSET>,
            EnumWarpAdapter: EnumWarpAdapter::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory5_Impl, const OFFSET: isize>() -> IDXGIFactory5_Vtbl {
        unsafe extern "system" fn CheckFeatureSupport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: DXGI_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIFactory5_Impl::CheckFeatureSupport(this, core::mem::transmute_copy(&feature), core::mem::transmute_copy(&pfeaturesupportdata), core::mem::transmute_copy(&featuresupportdatasize)).into()
        }
        Self { base__: IDXGIFactory4_Vtbl::new::<Identity, Impl, OFFSET>(), CheckFeatureSupport: CheckFeatureSupport::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory6_Impl, const OFFSET: isize>() -> IDXGIFactory6_Vtbl {
        unsafe extern "system" fn EnumAdapterByGpuPreference<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIFactory6_Impl::EnumAdapterByGpuPreference(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&gpupreference), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
        }
        Self { base__: IDXGIFactory5_Vtbl::new::<Identity, Impl, OFFSET>(), EnumAdapterByGpuPreference: EnumAdapterByGpuPreference::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory7_Impl, const OFFSET: isize>() -> IDXGIFactory7_Vtbl {
        unsafe extern "system" fn RegisterAdaptersChangedEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIFactory7_Impl::RegisterAdaptersChangedEvent(this, core::mem::transmute_copy(&hevent)) {
                Ok(ok__) => {
                    core::ptr::write(pdwcookie, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterAdaptersChangedEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIFactory7_Impl::UnregisterAdaptersChangedEvent(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: IDXGIFactory6_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegisterAdaptersChangedEvent: RegisterAdaptersChangedEvent::<Identity, Impl, OFFSET>,
            UnregisterAdaptersChangedEvent: UnregisterAdaptersChangedEvent::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactoryMedia_Impl, const OFFSET: isize>() -> IDXGIFactoryMedia_Vtbl {
        unsafe extern "system" fn CreateSwapChainForCompositionSurfaceHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactoryMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIFactoryMedia_Impl::CreateSwapChainForCompositionSurfaceHandle(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&hsurface), core::mem::transmute_copy(&pdesc), windows_core::from_raw_borrowed(&prestricttooutput)) {
                Ok(ok__) => {
                    core::ptr::write(ppswapchain, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecodeSwapChainForCompositionSurfaceHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactoryMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: *mut core::ffi::c_void, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIFactoryMedia_Impl::CreateDecodeSwapChainForCompositionSurfaceHandle(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&hsurface), core::mem::transmute_copy(&pdesc), windows_core::from_raw_borrowed(&pyuvdecodebuffers), windows_core::from_raw_borrowed(&prestricttooutput)) {
                Ok(ok__) => {
                    core::ptr::write(ppswapchain, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSwapChainForCompositionSurfaceHandle: CreateSwapChainForCompositionSurfaceHandle::<Identity, Impl, OFFSET>,
            CreateDecodeSwapChainForCompositionSurfaceHandle: CreateDecodeSwapChainForCompositionSurfaceHandle::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>() -> IDXGIInfoQueue_Vtbl {
        unsafe extern "system" fn SetMessageCountLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, messagecountlimit: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::SetMessageCountLimit(this, core::mem::transmute(&producer), core::mem::transmute_copy(&messagecountlimit)).into()
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::ClearStoredMessages(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::GetMessage(this, core::mem::transmute(&producer), core::mem::transmute_copy(&messageindex), core::mem::transmute_copy(&pmessage), core::mem::transmute_copy(&pmessagebytelength)).into()
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::GetNumStoredMessagesAllowedByRetrievalFilters(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::GetNumStoredMessages(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::GetNumMessagesDiscardedByMessageCountLimit(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::GetMessageCountLimit(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::GetNumMessagesAllowedByStorageFilter(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::GetNumMessagesDeniedByStorageFilter(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::AddStorageFilterEntries(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::GetStorageFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::ClearStorageFilter(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::PushEmptyStorageFilter(this, core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushDenyAllStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::PushDenyAllStorageFilter(this, core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::PushCopyOfStorageFilter(this, core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::PushStorageFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::PopStorageFilter(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::GetStorageFilterStackSize(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::AddRetrievalFilterEntries(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::GetRetrievalFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::ClearRetrievalFilter(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::PushEmptyRetrievalFilter(this, core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushDenyAllRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::PushDenyAllRetrievalFilter(this, core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::PushCopyOfRetrievalFilter(this, core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::PushRetrievalFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::PopRetrievalFilter(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::GetRetrievalFilterStackSize(this, core::mem::transmute(&producer))
        }
        unsafe extern "system" fn AddMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: windows_core::PCSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::AddMessage(this, core::mem::transmute(&producer), core::mem::transmute_copy(&category), core::mem::transmute_copy(&severity), core::mem::transmute_copy(&id), core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: windows_core::PCSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::AddApplicationMessage(this, core::mem::transmute_copy(&severity), core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::SetBreakOnCategory(this, core::mem::transmute(&producer), core::mem::transmute_copy(&category), core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::SetBreakOnSeverity(this, core::mem::transmute(&producer), core::mem::transmute_copy(&severity), core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, id: i32, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::SetBreakOnID(this, core::mem::transmute(&producer), core::mem::transmute_copy(&id), core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::GetBreakOnCategory(this, core::mem::transmute(&producer), core::mem::transmute_copy(&category))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::GetBreakOnSeverity(this, core::mem::transmute(&producer), core::mem::transmute_copy(&severity))
        }
        unsafe extern "system" fn GetBreakOnID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, id: i32) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::GetBreakOnID(this, core::mem::transmute(&producer), core::mem::transmute_copy(&id))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, bmute: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::SetMuteDebugOutput(this, core::mem::transmute(&producer), core::mem::transmute_copy(&bmute))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIInfoQueue_Impl::GetMuteDebugOutput(this, core::mem::transmute(&producer))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMessageCountLimit: SetMessageCountLimit::<Identity, Impl, OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilters: GetNumStoredMessagesAllowedByRetrievalFilters::<Identity, Impl, OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Identity, Impl, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Identity, Impl, OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Identity, Impl, OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Identity, Impl, OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Identity, Impl, OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Identity, Impl, OFFSET>,
            GetStorageFilter: GetStorageFilter::<Identity, Impl, OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Identity, Impl, OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Identity, Impl, OFFSET>,
            PushDenyAllStorageFilter: PushDenyAllStorageFilter::<Identity, Impl, OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Identity, Impl, OFFSET>,
            PushStorageFilter: PushStorageFilter::<Identity, Impl, OFFSET>,
            PopStorageFilter: PopStorageFilter::<Identity, Impl, OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Identity, Impl, OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Identity, Impl, OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Identity, Impl, OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Identity, Impl, OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Identity, Impl, OFFSET>,
            PushDenyAllRetrievalFilter: PushDenyAllRetrievalFilter::<Identity, Impl, OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Identity, Impl, OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Identity, Impl, OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Identity, Impl, OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Identity, Impl, OFFSET>,
            AddMessage: AddMessage::<Identity, Impl, OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Identity, Impl, OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Identity, Impl, OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Identity, Impl, OFFSET>,
            SetBreakOnID: SetBreakOnID::<Identity, Impl, OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Identity, Impl, OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Identity, Impl, OFFSET>,
            GetBreakOnID: GetBreakOnID::<Identity, Impl, OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Identity, Impl, OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIKeyedMutex_Impl, const OFFSET: isize>() -> IDXGIKeyedMutex_Vtbl {
        unsafe extern "system" fn AcquireSync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIKeyedMutex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: u64, dwmilliseconds: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIKeyedMutex_Impl::AcquireSync(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn ReleaseSync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIKeyedMutex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIKeyedMutex_Impl::ReleaseSync(this, core::mem::transmute_copy(&key)).into()
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            AcquireSync: AcquireSync::<Identity, Impl, OFFSET>,
            ReleaseSync: ReleaseSync::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIObject_Impl, const OFFSET: isize>() -> IDXGIObject_Vtbl {
        unsafe extern "system" fn SetPrivateData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIObject_Impl::SetPrivateData(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, punknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIObject_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&name), windows_core::from_raw_borrowed(&punknown)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIObject_Impl::GetPrivateData(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetParent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIObject_Impl::GetParent(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppparent)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            GetParent: GetParent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput_Impl: Sized + IDXGIObject_Impl {
    fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> windows_core::Result<()>;
    fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> windows_core::Result<()>;
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>() -> IDXGIOutput_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutput_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetDisplayModeList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutput_Impl::GetDisplayModeList(this, core::mem::transmute_copy(&enumformat), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pnummodes), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn FindClosestMatchingMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutput_Impl::FindClosestMatchingMode(this, core::mem::transmute_copy(&pmodetomatch), core::mem::transmute_copy(&pclosestmatch), windows_core::from_raw_borrowed(&pconcerneddevice)).into()
        }
        unsafe extern "system" fn WaitForVBlank<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutput_Impl::WaitForVBlank(this).into()
        }
        unsafe extern "system" fn TakeOwnership<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, exclusive: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutput_Impl::TakeOwnership(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&exclusive)).into()
        }
        unsafe extern "system" fn ReleaseOwnership<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutput_Impl::ReleaseOwnership(this)
        }
        unsafe extern "system" fn GetGammaControlCapabilities<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutput_Impl::GetGammaControlCapabilities(this, core::mem::transmute_copy(&pgammacaps)).into()
        }
        unsafe extern "system" fn SetGammaControl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parray: *const Common::DXGI_GAMMA_CONTROL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutput_Impl::SetGammaControl(this, core::mem::transmute_copy(&parray)).into()
        }
        unsafe extern "system" fn GetGammaControl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parray: *mut Common::DXGI_GAMMA_CONTROL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutput_Impl::GetGammaControl(this, core::mem::transmute_copy(&parray)).into()
        }
        unsafe extern "system" fn SetDisplaySurface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscanoutsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutput_Impl::SetDisplaySurface(this, windows_core::from_raw_borrowed(&pscanoutsurface)).into()
        }
        unsafe extern "system" fn GetDisplaySurfaceData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutput_Impl::GetDisplaySurfaceData(this, windows_core::from_raw_borrowed(&pdestination)).into()
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutput_Impl::GetFrameStatistics(this, core::mem::transmute_copy(&pstats)).into()
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetDisplayModeList: GetDisplayModeList::<Identity, Impl, OFFSET>,
            FindClosestMatchingMode: FindClosestMatchingMode::<Identity, Impl, OFFSET>,
            WaitForVBlank: WaitForVBlank::<Identity, Impl, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, Impl, OFFSET>,
            ReleaseOwnership: ReleaseOwnership::<Identity, Impl, OFFSET>,
            GetGammaControlCapabilities: GetGammaControlCapabilities::<Identity, Impl, OFFSET>,
            SetGammaControl: SetGammaControl::<Identity, Impl, OFFSET>,
            GetGammaControl: GetGammaControl::<Identity, Impl, OFFSET>,
            SetDisplaySurface: SetDisplaySurface::<Identity, Impl, OFFSET>,
            GetDisplaySurfaceData: GetDisplaySurfaceData::<Identity, Impl, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput1_Impl: Sized + IDXGIOutput_Impl {
    fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> windows_core::Result<()>;
    fn FindClosestMatchingMode1(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetDisplaySurfaceData1(&self, pdestination: Option<&IDXGIResource>) -> windows_core::Result<()>;
    fn DuplicateOutput(&self, pdevice: Option<&windows_core::IUnknown>) -> windows_core::Result<IDXGIOutputDuplication>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGIOutput1 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput1_Impl, const OFFSET: isize>() -> IDXGIOutput1_Vtbl {
        unsafe extern "system" fn GetDisplayModeList1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutput1_Impl::GetDisplayModeList1(this, core::mem::transmute_copy(&enumformat), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pnummodes), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn FindClosestMatchingMode1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutput1_Impl::FindClosestMatchingMode1(this, core::mem::transmute_copy(&pmodetomatch), core::mem::transmute_copy(&pclosestmatch), windows_core::from_raw_borrowed(&pconcerneddevice)).into()
        }
        unsafe extern "system" fn GetDisplaySurfaceData1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutput1_Impl::GetDisplaySurfaceData1(this, windows_core::from_raw_borrowed(&pdestination)).into()
        }
        unsafe extern "system" fn DuplicateOutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, ppoutputduplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIOutput1_Impl::DuplicateOutput(this, windows_core::from_raw_borrowed(&pdevice)) {
                Ok(ok__) => {
                    core::ptr::write(ppoutputduplication, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIOutput_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDisplayModeList1: GetDisplayModeList1::<Identity, Impl, OFFSET>,
            FindClosestMatchingMode1: FindClosestMatchingMode1::<Identity, Impl, OFFSET>,
            GetDisplaySurfaceData1: GetDisplaySurfaceData1::<Identity, Impl, OFFSET>,
            DuplicateOutput: DuplicateOutput::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput2_Impl, const OFFSET: isize>() -> IDXGIOutput2_Vtbl {
        unsafe extern "system" fn SupportsOverlays<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutput2_Impl::SupportsOverlays(this)
        }
        Self { base__: IDXGIOutput1_Vtbl::new::<Identity, Impl, OFFSET>(), SupportsOverlays: SupportsOverlays::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput3_Impl, const OFFSET: isize>() -> IDXGIOutput3_Vtbl {
        unsafe extern "system" fn CheckOverlaySupport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumformat: Common::DXGI_FORMAT, pconcerneddevice: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIOutput3_Impl::CheckOverlaySupport(this, core::mem::transmute_copy(&enumformat), windows_core::from_raw_borrowed(&pconcerneddevice)) {
                Ok(ok__) => {
                    core::ptr::write(pflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IDXGIOutput2_Vtbl::new::<Identity, Impl, OFFSET>(), CheckOverlaySupport: CheckOverlaySupport::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput4_Impl, const OFFSET: isize>() -> IDXGIOutput4_Vtbl {
        unsafe extern "system" fn CheckOverlayColorSpaceSupport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIOutput4_Impl::CheckOverlayColorSpaceSupport(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&colorspace), windows_core::from_raw_borrowed(&pconcerneddevice)) {
                Ok(ok__) => {
                    core::ptr::write(pflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIOutput3_Vtbl::new::<Identity, Impl, OFFSET>(),
            CheckOverlayColorSpaceSupport: CheckOverlayColorSpaceSupport::<Identity, Impl, OFFSET>,
        }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput5_Impl, const OFFSET: isize>() -> IDXGIOutput5_Vtbl {
        unsafe extern "system" fn DuplicateOutput1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT, ppoutputduplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIOutput5_Impl::DuplicateOutput1(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&supportedformatscount), core::mem::transmute_copy(&psupportedformats)) {
                Ok(ok__) => {
                    core::ptr::write(ppoutputduplication, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IDXGIOutput4_Vtbl::new::<Identity, Impl, OFFSET>(), DuplicateOutput1: DuplicateOutput1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput5 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIOutput1 as windows_core::Interface>::IID || iid == &<IDXGIOutput2 as windows_core::Interface>::IID || iid == &<IDXGIOutput3 as windows_core::Interface>::IID || iid == &<IDXGIOutput4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput6_Impl: Sized + IDXGIOutput5_Impl {
    fn GetDesc1(&self, pdesc: *mut DXGI_OUTPUT_DESC1) -> windows_core::Result<()>;
    fn CheckHardwareCompositionSupport(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGIOutput6 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput6_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput6_Impl, const OFFSET: isize>() -> IDXGIOutput6_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC1) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutput6_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn CheckHardwareCompositionSupport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIOutput6_Impl::CheckHardwareCompositionSupport(this) {
                Ok(ok__) => {
                    core::ptr::write(pflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIOutput5_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDesc1: GetDesc1::<Identity, Impl, OFFSET>,
            CheckHardwareCompositionSupport: CheckHardwareCompositionSupport::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>() -> IDXGIOutputDuplication_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_OUTDUPL_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutputDuplication_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        unsafe extern "system" fn AcquireNextFrame<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutputDuplication_Impl::AcquireNextFrame(this, core::mem::transmute_copy(&timeoutinmilliseconds), core::mem::transmute_copy(&pframeinfo), core::mem::transmute_copy(&ppdesktopresource)).into()
        }
        unsafe extern "system" fn GetFrameDirtyRects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutputDuplication_Impl::GetFrameDirtyRects(this, core::mem::transmute_copy(&dirtyrectsbuffersize), core::mem::transmute_copy(&pdirtyrectsbuffer), core::mem::transmute_copy(&pdirtyrectsbuffersizerequired)).into()
        }
        unsafe extern "system" fn GetFrameMoveRects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutputDuplication_Impl::GetFrameMoveRects(this, core::mem::transmute_copy(&moverectsbuffersize), core::mem::transmute_copy(&pmoverectbuffer), core::mem::transmute_copy(&pmoverectsbuffersizerequired)).into()
        }
        unsafe extern "system" fn GetFramePointerShape<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointershapebuffersize: u32, ppointershapebuffer: *mut core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutputDuplication_Impl::GetFramePointerShape(this, core::mem::transmute_copy(&pointershapebuffersize), core::mem::transmute_copy(&ppointershapebuffer), core::mem::transmute_copy(&ppointershapebuffersizerequired), core::mem::transmute_copy(&ppointershapeinfo)).into()
        }
        unsafe extern "system" fn MapDesktopSurface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIOutputDuplication_Impl::MapDesktopSurface(this) {
                Ok(ok__) => {
                    core::ptr::write(plockedrect, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnMapDesktopSurface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutputDuplication_Impl::UnMapDesktopSurface(this).into()
        }
        unsafe extern "system" fn ReleaseFrame<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIOutputDuplication_Impl::ReleaseFrame(this).into()
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            AcquireNextFrame: AcquireNextFrame::<Identity, Impl, OFFSET>,
            GetFrameDirtyRects: GetFrameDirtyRects::<Identity, Impl, OFFSET>,
            GetFrameMoveRects: GetFrameMoveRects::<Identity, Impl, OFFSET>,
            GetFramePointerShape: GetFramePointerShape::<Identity, Impl, OFFSET>,
            MapDesktopSurface: MapDesktopSurface::<Identity, Impl, OFFSET>,
            UnMapDesktopSurface: UnMapDesktopSurface::<Identity, Impl, OFFSET>,
            ReleaseFrame: ReleaseFrame::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutputDuplication as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
pub trait IDXGIResource_Impl: Sized + IDXGIDeviceSubObject_Impl {
    fn GetSharedHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn GetUsage(&self) -> windows_core::Result<DXGI_USAGE>;
    fn SetEvictionPriority(&self, evictionpriority: u32) -> windows_core::Result<()>;
    fn GetEvictionPriority(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IDXGIResource {}
impl IDXGIResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIResource_Impl, const OFFSET: isize>() -> IDXGIResource_Vtbl {
        unsafe extern "system" fn GetSharedHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIResource_Impl::GetSharedHandle(this) {
                Ok(ok__) => {
                    core::ptr::write(psharedhandle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusage: *mut DXGI_USAGE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIResource_Impl::GetUsage(this) {
                Ok(ok__) => {
                    core::ptr::write(pusage, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEvictionPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, evictionpriority: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGIResource_Impl::SetEvictionPriority(this, core::mem::transmute_copy(&evictionpriority)).into()
        }
        unsafe extern "system" fn GetEvictionPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevictionpriority: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIResource_Impl::GetEvictionPriority(this) {
                Ok(ok__) => {
                    core::ptr::write(pevictionpriority, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSharedHandle: GetSharedHandle::<Identity, Impl, OFFSET>,
            GetUsage: GetUsage::<Identity, Impl, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, Impl, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIResource1_Impl, const OFFSET: isize>() -> IDXGIResource1_Vtbl {
        unsafe extern "system" fn CreateSubresourceSurface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIResource1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIResource1_Impl::CreateSubresourceSurface(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(ppsurface, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSharedHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGIResource1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: windows_core::PCWSTR, phandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGIResource1_Impl::CreateSharedHandle(this, core::mem::transmute_copy(&pattributes), core::mem::transmute_copy(&dwaccess), core::mem::transmute(&lpname)) {
                Ok(ok__) => {
                    core::ptr::write(phandle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateSubresourceSurface: CreateSubresourceSurface::<Identity, Impl, OFFSET>,
            CreateSharedHandle: CreateSharedHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIResource1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGIResource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISurface_Impl: Sized + IDXGIDeviceSubObject_Impl {
    fn GetDesc(&self, pdesc: *mut DXGI_SURFACE_DESC) -> windows_core::Result<()>;
    fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> windows_core::Result<()>;
    fn Unmap(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISurface {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISurface_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface_Impl, const OFFSET: isize>() -> IDXGISurface_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SURFACE_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISurface_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn Map<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISurface_Impl::Map(this, core::mem::transmute_copy(&plockedrect), core::mem::transmute_copy(&mapflags)).into()
        }
        unsafe extern "system" fn Unmap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISurface_Impl::Unmap(this).into()
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface1_Impl, const OFFSET: isize>() -> IDXGISurface1_Vtbl {
        unsafe extern "system" fn GetDC<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discard: super::super::Foundation::BOOL, phdc: *mut super::Gdi::HDC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGISurface1_Impl::GetDC(this, core::mem::transmute_copy(&discard)) {
                Ok(ok__) => {
                    core::ptr::write(phdc, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISurface1_Impl::ReleaseDC(this, core::mem::transmute_copy(&pdirtyrect)).into()
        }
        Self {
            base__: IDXGISurface_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
        }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface2_Impl, const OFFSET: isize>() -> IDXGISurface2_Vtbl {
        unsafe extern "system" fn GetResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppparentresource: *mut *mut core::ffi::c_void, psubresourceindex: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISurface2_Impl::GetResource(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppparentresource), core::mem::transmute_copy(&psubresourceindex)).into()
        }
        Self { base__: IDXGISurface1_Vtbl::new::<Identity, Impl, OFFSET>(), GetResource: GetResource::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISurface2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISurface as windows_core::Interface>::IID || iid == &<IDXGISurface1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISwapChain_Impl: Sized + IDXGIDeviceSubObject_Impl {
    fn Present(&self, syncinterval: u32, flags: u32) -> windows_core::HRESULT;
    fn GetBuffer(&self, buffer: u32, riid: *const windows_core::GUID, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetFullscreenState(&self, fullscreen: super::super::Foundation::BOOL, ptarget: Option<&IDXGIOutput>) -> windows_core::Result<()>;
    fn GetFullscreenState(&self, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut Option<IDXGIOutput>) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> windows_core::Result<()>;
    fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> windows_core::Result<()>;
    fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> windows_core::Result<()>;
    fn GetContainingOutput(&self) -> windows_core::Result<IDXGIOutput>;
    fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::Result<()>;
    fn GetLastPresentCount(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISwapChain {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISwapChain_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>() -> IDXGISwapChain_Vtbl {
        unsafe extern "system" fn Present<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncinterval: u32, flags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain_Impl::Present(this, core::mem::transmute_copy(&syncinterval), core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn GetBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: u32, riid: *const windows_core::GUID, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain_Impl::GetBuffer(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppsurface)).into()
        }
        unsafe extern "system" fn SetFullscreenState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fullscreen: super::super::Foundation::BOOL, ptarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain_Impl::SetFullscreenState(this, core::mem::transmute_copy(&fullscreen), windows_core::from_raw_borrowed(&ptarget)).into()
        }
        unsafe extern "system" fn GetFullscreenState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain_Impl::GetFullscreenState(this, core::mem::transmute_copy(&pfullscreen), core::mem::transmute_copy(&pptarget)).into()
        }
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn ResizeBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain_Impl::ResizeBuffers(this, core::mem::transmute_copy(&buffercount), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&newformat), core::mem::transmute_copy(&swapchainflags)).into()
        }
        unsafe extern "system" fn ResizeTarget<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain_Impl::ResizeTarget(this, core::mem::transmute_copy(&pnewtargetparameters)).into()
        }
        unsafe extern "system" fn GetContainingOutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGISwapChain_Impl::GetContainingOutput(this) {
                Ok(ok__) => {
                    core::ptr::write(ppoutput, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain_Impl::GetFrameStatistics(this, core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn GetLastPresentCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastpresentcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGISwapChain_Impl::GetLastPresentCount(this) {
                Ok(ok__) => {
                    core::ptr::write(plastpresentcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            Present: Present::<Identity, Impl, OFFSET>,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            SetFullscreenState: SetFullscreenState::<Identity, Impl, OFFSET>,
            GetFullscreenState: GetFullscreenState::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            ResizeBuffers: ResizeBuffers::<Identity, Impl, OFFSET>,
            ResizeTarget: ResizeTarget::<Identity, Impl, OFFSET>,
            GetContainingOutput: GetContainingOutput::<Identity, Impl, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, Impl, OFFSET>,
            GetLastPresentCount: GetLastPresentCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISwapChain1_Impl: Sized + IDXGISwapChain_Impl {
    fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> windows_core::Result<()>;
    fn GetFullscreenDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> windows_core::Result<()>;
    fn GetHwnd(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn GetCoreWindow(&self, refiid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> windows_core::HRESULT;
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>() -> IDXGISwapChain1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetFullscreenDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain1_Impl::GetFullscreenDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetHwnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGISwapChain1_Impl::GetHwnd(this) {
                Ok(ok__) => {
                    core::ptr::write(phwnd, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCoreWindow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refiid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain1_Impl::GetCoreWindow(this, core::mem::transmute_copy(&refiid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn Present1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain1_Impl::Present1(this, core::mem::transmute_copy(&syncinterval), core::mem::transmute_copy(&presentflags), core::mem::transmute_copy(&ppresentparameters))
        }
        unsafe extern "system" fn IsTemporaryMonoSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain1_Impl::IsTemporaryMonoSupported(this)
        }
        unsafe extern "system" fn GetRestrictToOutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprestricttooutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGISwapChain1_Impl::GetRestrictToOutput(this) {
                Ok(ok__) => {
                    core::ptr::write(pprestricttooutput, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *const DXGI_RGBA) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain1_Impl::SetBackgroundColor(this, core::mem::transmute_copy(&pcolor)).into()
        }
        unsafe extern "system" fn GetBackgroundColor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut DXGI_RGBA) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGISwapChain1_Impl::GetBackgroundColor(this) {
                Ok(ok__) => {
                    core::ptr::write(pcolor, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotation: Common::DXGI_MODE_ROTATION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain1_Impl::SetRotation(this, core::mem::transmute_copy(&rotation)).into()
        }
        unsafe extern "system" fn GetRotation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, protation: *mut Common::DXGI_MODE_ROTATION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGISwapChain1_Impl::GetRotation(this) {
                Ok(ok__) => {
                    core::ptr::write(protation, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGISwapChain_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDesc1: GetDesc1::<Identity, Impl, OFFSET>,
            GetFullscreenDesc: GetFullscreenDesc::<Identity, Impl, OFFSET>,
            GetHwnd: GetHwnd::<Identity, Impl, OFFSET>,
            GetCoreWindow: GetCoreWindow::<Identity, Impl, OFFSET>,
            Present1: Present1::<Identity, Impl, OFFSET>,
            IsTemporaryMonoSupported: IsTemporaryMonoSupported::<Identity, Impl, OFFSET>,
            GetRestrictToOutput: GetRestrictToOutput::<Identity, Impl, OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Identity, Impl, OFFSET>,
            GetBackgroundColor: GetBackgroundColor::<Identity, Impl, OFFSET>,
            SetRotation: SetRotation::<Identity, Impl, OFFSET>,
            GetRotation: GetRotation::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>() -> IDXGISwapChain2_Vtbl {
        unsafe extern "system" fn SetSourceSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain2_Impl::SetSourceSize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn GetSourceSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain2_Impl::GetSourceSize(this, core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlatency: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain2_Impl::SetMaximumFrameLatency(this, core::mem::transmute_copy(&maxlatency)).into()
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxlatency: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGISwapChain2_Impl::GetMaximumFrameLatency(this) {
                Ok(ok__) => {
                    core::ptr::write(pmaxlatency, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameLatencyWaitableObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain2_Impl::GetFrameLatencyWaitableObject(this)
        }
        unsafe extern "system" fn SetMatrixTransform<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *const DXGI_MATRIX_3X2_F) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain2_Impl::SetMatrixTransform(this, core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn GetMatrixTransform<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *mut DXGI_MATRIX_3X2_F) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain2_Impl::GetMatrixTransform(this, core::mem::transmute_copy(&pmatrix)).into()
        }
        Self {
            base__: IDXGISwapChain1_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetSourceSize: SetSourceSize::<Identity, Impl, OFFSET>,
            GetSourceSize: GetSourceSize::<Identity, Impl, OFFSET>,
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, Impl, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, Impl, OFFSET>,
            GetFrameLatencyWaitableObject: GetFrameLatencyWaitableObject::<Identity, Impl, OFFSET>,
            SetMatrixTransform: SetMatrixTransform::<Identity, Impl, OFFSET>,
            GetMatrixTransform: GetMatrixTransform::<Identity, Impl, OFFSET>,
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
    fn ResizeBuffers1(&self, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const Option<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISwapChain3 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISwapChain3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain3_Impl, const OFFSET: isize>() -> IDXGISwapChain3_Vtbl {
        unsafe extern "system" fn GetCurrentBackBufferIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain3_Impl::GetCurrentBackBufferIndex(this)
        }
        unsafe extern "system" fn CheckColorSpaceSupport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pcolorspacesupport: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXGISwapChain3_Impl::CheckColorSpaceSupport(this, core::mem::transmute_copy(&colorspace)) {
                Ok(ok__) => {
                    core::ptr::write(pcolorspacesupport, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorSpace1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain3_Impl::SetColorSpace1(this, core::mem::transmute_copy(&colorspace)).into()
        }
        unsafe extern "system" fn ResizeBuffers1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain3_Impl::ResizeBuffers1(this, core::mem::transmute_copy(&buffercount), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&swapchainflags), core::mem::transmute_copy(&pcreationnodemask), core::mem::transmute_copy(&pppresentqueue)).into()
        }
        Self {
            base__: IDXGISwapChain2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCurrentBackBufferIndex: GetCurrentBackBufferIndex::<Identity, Impl, OFFSET>,
            CheckColorSpaceSupport: CheckColorSpaceSupport::<Identity, Impl, OFFSET>,
            SetColorSpace1: SetColorSpace1::<Identity, Impl, OFFSET>,
            ResizeBuffers1: ResizeBuffers1::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain4_Impl, const OFFSET: isize>() -> IDXGISwapChain4_Vtbl {
        unsafe extern "system" fn SetHDRMetaData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChain4_Impl::SetHDRMetaData(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&size), core::mem::transmute_copy(&pmetadata)).into()
        }
        Self { base__: IDXGISwapChain3_Vtbl::new::<Identity, Impl, OFFSET>(), SetHDRMetaData: SetHDRMetaData::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChainMedia_Impl, const OFFSET: isize>() -> IDXGISwapChainMedia_Vtbl {
        unsafe extern "system" fn GetFrameStatisticsMedia<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChainMedia_Impl::GetFrameStatisticsMedia(this, core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn SetPresentDuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChainMedia_Impl::SetPresentDuration(this, core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn CheckPresentDurationSupport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGISwapChainMedia_Impl::CheckPresentDurationSupport(this, core::mem::transmute_copy(&desiredpresentduration), core::mem::transmute_copy(&pclosestsmallerpresentduration), core::mem::transmute_copy(&pclosestlargerpresentduration)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFrameStatisticsMedia: GetFrameStatisticsMedia::<Identity, Impl, OFFSET>,
            SetPresentDuration: SetPresentDuration::<Identity, Impl, OFFSET>,
            CheckPresentDurationSupport: CheckPresentDurationSupport::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGraphicsAnalysis_Impl, const OFFSET: isize>() -> IDXGraphicsAnalysis_Vtbl {
        unsafe extern "system" fn BeginCapture<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGraphicsAnalysis_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGraphicsAnalysis_Impl::BeginCapture(this)
        }
        unsafe extern "system" fn EndCapture<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXGraphicsAnalysis_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXGraphicsAnalysis_Impl::EndCapture(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginCapture: BeginCapture::<Identity, Impl, OFFSET>,
            EndCapture: EndCapture::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGraphicsAnalysis as windows_core::Interface>::IID
    }
}
