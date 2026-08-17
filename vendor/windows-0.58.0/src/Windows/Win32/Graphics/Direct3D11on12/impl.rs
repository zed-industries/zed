#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D11On12Device_Impl: Sized {
    fn CreateWrappedResource(&self, presource12: Option<&windows_core::IUnknown>, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const windows_core::GUID, ppresource11: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ReleaseWrappedResources(&self, ppresources: *const Option<super::Direct3D11::ID3D11Resource>, numresources: u32);
    fn AcquireWrappedResources(&self, ppresources: *const Option<super::Direct3D11::ID3D11Resource>, numresources: u32);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl windows_core::RuntimeName for ID3D11On12Device {}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D11On12Device_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D11On12Device_Vtbl
    where
        Identity: ID3D11On12Device_Impl,
    {
        unsafe extern "system" fn CreateWrappedResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource12: *mut core::ffi::c_void, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const windows_core::GUID, ppresource11: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D11On12Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D11On12Device_Impl::CreateWrappedResource(this, windows_core::from_raw_borrowed(&presource12), core::mem::transmute_copy(&pflags11), core::mem::transmute_copy(&instate), core::mem::transmute_copy(&outstate), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppresource11)).into()
        }
        unsafe extern "system" fn ReleaseWrappedResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, numresources: u32)
        where
            Identity: ID3D11On12Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D11On12Device_Impl::ReleaseWrappedResources(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&numresources))
        }
        unsafe extern "system" fn AcquireWrappedResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, numresources: u32)
        where
            Identity: ID3D11On12Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D11On12Device_Impl::AcquireWrappedResources(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&numresources))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateWrappedResource: CreateWrappedResource::<Identity, OFFSET>,
            ReleaseWrappedResources: ReleaseWrappedResources::<Identity, OFFSET>,
            AcquireWrappedResources: AcquireWrappedResources::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11On12Device as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D11On12Device1_Impl: Sized + ID3D11On12Device_Impl {
    fn GetD3D12Device(&self, riid: *const windows_core::GUID, ppvdevice: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl windows_core::RuntimeName for ID3D11On12Device1 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D11On12Device1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D11On12Device1_Vtbl
    where
        Identity: ID3D11On12Device1_Impl,
    {
        unsafe extern "system" fn GetD3D12Device<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D11On12Device1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D11On12Device1_Impl::GetD3D12Device(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvdevice)).into()
        }
        Self { base__: ID3D11On12Device_Vtbl::new::<Identity, OFFSET>(), GetD3D12Device: GetD3D12Device::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11On12Device1 as windows_core::Interface>::IID || iid == &<ID3D11On12Device as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D11On12Device2_Impl: Sized + ID3D11On12Device1_Impl {
    fn UnwrapUnderlyingResource(&self, presource11: Option<&super::Direct3D11::ID3D11Resource>, pcommandqueue: Option<&super::Direct3D12::ID3D12CommandQueue>, riid: *const windows_core::GUID, ppvresource12: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ReturnUnderlyingResource(&self, presource11: Option<&super::Direct3D11::ID3D11Resource>, numsync: u32, psignalvalues: *const u64, ppfences: *const Option<super::Direct3D12::ID3D12Fence>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl windows_core::RuntimeName for ID3D11On12Device2 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D11On12Device2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D11On12Device2_Vtbl
    where
        Identity: ID3D11On12Device2_Impl,
    {
        unsafe extern "system" fn UnwrapUnderlyingResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource11: *mut core::ffi::c_void, pcommandqueue: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvresource12: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D11On12Device2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D11On12Device2_Impl::UnwrapUnderlyingResource(this, windows_core::from_raw_borrowed(&presource11), windows_core::from_raw_borrowed(&pcommandqueue), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvresource12)).into()
        }
        unsafe extern "system" fn ReturnUnderlyingResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource11: *mut core::ffi::c_void, numsync: u32, psignalvalues: *const u64, ppfences: *const *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D11On12Device2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D11On12Device2_Impl::ReturnUnderlyingResource(this, windows_core::from_raw_borrowed(&presource11), core::mem::transmute_copy(&numsync), core::mem::transmute_copy(&psignalvalues), core::mem::transmute_copy(&ppfences)).into()
        }
        Self {
            base__: ID3D11On12Device1_Vtbl::new::<Identity, OFFSET>(),
            UnwrapUnderlyingResource: UnwrapUnderlyingResource::<Identity, OFFSET>,
            ReturnUnderlyingResource: ReturnUnderlyingResource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11On12Device2 as windows_core::Interface>::IID || iid == &<ID3D11On12Device as windows_core::Interface>::IID || iid == &<ID3D11On12Device1 as windows_core::Interface>::IID
    }
}
