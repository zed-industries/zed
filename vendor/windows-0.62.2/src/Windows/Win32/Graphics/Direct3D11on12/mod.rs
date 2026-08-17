#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
#[inline]
pub unsafe fn D3D11On12CreateDevice<P0>(pdevice: P0, flags: u32, pfeaturelevels: Option<&[super::Direct3D::D3D_FEATURE_LEVEL]>, ppcommandqueues: Option<&[Option<windows_core::IUnknown>]>, nodemask: u32, ppdevice: Option<*mut Option<super::Direct3D11::ID3D11Device>>, ppimmediatecontext: Option<*mut Option<super::Direct3D11::ID3D11DeviceContext>>, pchosenfeaturelevel: Option<*mut super::Direct3D::D3D_FEATURE_LEVEL>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("d3d11.dll" "system" fn D3D11On12CreateDevice(pdevice : * mut core::ffi::c_void, flags : u32, pfeaturelevels : *const super::Direct3D:: D3D_FEATURE_LEVEL, featurelevels : u32, ppcommandqueues : *const * mut core::ffi::c_void, numqueues : u32, nodemask : u32, ppdevice : *mut * mut core::ffi::c_void, ppimmediatecontext : *mut * mut core::ffi::c_void, pchosenfeaturelevel : *mut super::Direct3D:: D3D_FEATURE_LEVEL) -> windows_core::HRESULT);
    unsafe {
        D3D11On12CreateDevice(
            pdevice.param().abi(),
            flags,
            core::mem::transmute(pfeaturelevels.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pfeaturelevels.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(ppcommandqueues.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            ppcommandqueues.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            nodemask,
            ppdevice.unwrap_or(core::mem::zeroed()) as _,
            ppimmediatecontext.unwrap_or(core::mem::zeroed()) as _,
            pchosenfeaturelevel.unwrap_or(core::mem::zeroed()) as _,
        )
        .ok()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_RESOURCE_FLAGS {
    pub BindFlags: u32,
    pub MiscFlags: u32,
    pub CPUAccessFlags: u32,
    pub StructureByteStride: u32,
}
windows_core::imp::define_interface!(ID3D11On12Device, ID3D11On12Device_Vtbl, 0x85611e73_70a9_490e_9614_a9e302777904);
windows_core::imp::interface_hierarchy!(ID3D11On12Device, windows_core::IUnknown);
impl ID3D11On12Device {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateWrappedResource<P0, T>(&self, presource12: P0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateWrappedResource)(windows_core::Interface::as_raw(self), presource12.param().abi(), pflags11, instate, outstate, &T::IID, result__ as *mut _ as *mut _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: &[Option<super::Direct3D11::ID3D11Resource>]) {
        unsafe { (windows_core::Interface::vtable(self).ReleaseWrappedResources)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), ppresources.len().try_into().unwrap()) }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: &[Option<super::Direct3D11::ID3D11Resource>]) {
        unsafe { (windows_core::Interface::vtable(self).AcquireWrappedResources)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), ppresources.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11On12Device_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateWrappedResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_RESOURCE_FLAGS, super::Direct3D12::D3D12_RESOURCE_STATES, super::Direct3D12::D3D12_RESOURCE_STATES, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateWrappedResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub ReleaseWrappedResources: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))]
    ReleaseWrappedResources: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub AcquireWrappedResources: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))]
    AcquireWrappedResources: usize,
}
unsafe impl Send for ID3D11On12Device {}
unsafe impl Sync for ID3D11On12Device {}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D11On12Device_Impl: windows_core::IUnknownImpl {
    fn CreateWrappedResource(&self, presource12: windows_core::Ref<windows_core::IUnknown>, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const windows_core::GUID, ppresource11: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ReleaseWrappedResources(&self, ppresources: *const Option<super::Direct3D11::ID3D11Resource>, numresources: u32);
    fn AcquireWrappedResources(&self, ppresources: *const Option<super::Direct3D11::ID3D11Resource>, numresources: u32);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D11On12Device_Vtbl {
    pub const fn new<Identity: ID3D11On12Device_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateWrappedResource<Identity: ID3D11On12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource12: *mut core::ffi::c_void, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const windows_core::GUID, ppresource11: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11On12Device_Impl::CreateWrappedResource(this, core::mem::transmute_copy(&presource12), core::mem::transmute_copy(&pflags11), core::mem::transmute_copy(&instate), core::mem::transmute_copy(&outstate), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppresource11)).into()
            }
        }
        unsafe extern "system" fn ReleaseWrappedResources<Identity: ID3D11On12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, numresources: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11On12Device_Impl::ReleaseWrappedResources(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&numresources))
            }
        }
        unsafe extern "system" fn AcquireWrappedResources<Identity: ID3D11On12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, numresources: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11On12Device_Impl::AcquireWrappedResources(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&numresources))
            }
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
impl windows_core::RuntimeName for ID3D11On12Device {}
windows_core::imp::define_interface!(ID3D11On12Device1, ID3D11On12Device1_Vtbl, 0xbdb64df4_ea2f_4c70_b861_aaab1258bb5d);
impl core::ops::Deref for ID3D11On12Device1 {
    type Target = ID3D11On12Device;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11On12Device1, windows_core::IUnknown, ID3D11On12Device);
impl ID3D11On12Device1 {
    pub unsafe fn GetD3D12Device<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetD3D12Device)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11On12Device1_Vtbl {
    pub base__: ID3D11On12Device_Vtbl,
    pub GetD3D12Device: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D11On12Device1 {}
unsafe impl Sync for ID3D11On12Device1 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D11On12Device1_Impl: ID3D11On12Device_Impl {
    fn GetD3D12Device(&self, riid: *const windows_core::GUID, ppvdevice: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D11On12Device1_Vtbl {
    pub const fn new<Identity: ID3D11On12Device1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetD3D12Device<Identity: ID3D11On12Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11On12Device1_Impl::GetD3D12Device(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvdevice)).into()
            }
        }
        Self { base__: ID3D11On12Device_Vtbl::new::<Identity, OFFSET>(), GetD3D12Device: GetD3D12Device::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11On12Device1 as windows_core::Interface>::IID || iid == &<ID3D11On12Device as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl windows_core::RuntimeName for ID3D11On12Device1 {}
windows_core::imp::define_interface!(ID3D11On12Device2, ID3D11On12Device2_Vtbl, 0xdc90f331_4740_43fa_866e_67f12cb58223);
impl core::ops::Deref for ID3D11On12Device2 {
    type Target = ID3D11On12Device1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11On12Device2, windows_core::IUnknown, ID3D11On12Device, ID3D11On12Device1);
impl ID3D11On12Device2 {
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn UnwrapUnderlyingResource<P0, P1, T>(&self, presource11: P0, pcommandqueue: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::Direct3D11::ID3D11Resource>,
        P1: windows_core::Param<super::Direct3D12::ID3D12CommandQueue>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).UnwrapUnderlyingResource)(windows_core::Interface::as_raw(self), presource11.param().abi(), pcommandqueue.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn ReturnUnderlyingResource<P0>(&self, presource11: P0, numsync: u32, psignalvalues: *const u64, ppfences: *const Option<super::Direct3D12::ID3D12Fence>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Direct3D11::ID3D11Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReturnUnderlyingResource)(windows_core::Interface::as_raw(self), presource11.param().abi(), numsync, psignalvalues, core::mem::transmute(ppfences)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11On12Device2_Vtbl {
    pub base__: ID3D11On12Device1_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    pub UnwrapUnderlyingResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12")))]
    UnwrapUnderlyingResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    pub ReturnUnderlyingResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u64, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12")))]
    ReturnUnderlyingResource: usize,
}
unsafe impl Send for ID3D11On12Device2 {}
unsafe impl Sync for ID3D11On12Device2 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D11On12Device2_Impl: ID3D11On12Device1_Impl {
    fn UnwrapUnderlyingResource(&self, presource11: windows_core::Ref<super::Direct3D11::ID3D11Resource>, pcommandqueue: windows_core::Ref<super::Direct3D12::ID3D12CommandQueue>, riid: *const windows_core::GUID, ppvresource12: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ReturnUnderlyingResource(&self, presource11: windows_core::Ref<super::Direct3D11::ID3D11Resource>, numsync: u32, psignalvalues: *const u64, ppfences: *const Option<super::Direct3D12::ID3D12Fence>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D11On12Device2_Vtbl {
    pub const fn new<Identity: ID3D11On12Device2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UnwrapUnderlyingResource<Identity: ID3D11On12Device2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource11: *mut core::ffi::c_void, pcommandqueue: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvresource12: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11On12Device2_Impl::UnwrapUnderlyingResource(this, core::mem::transmute_copy(&presource11), core::mem::transmute_copy(&pcommandqueue), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvresource12)).into()
            }
        }
        unsafe extern "system" fn ReturnUnderlyingResource<Identity: ID3D11On12Device2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource11: *mut core::ffi::c_void, numsync: u32, psignalvalues: *const u64, ppfences: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11On12Device2_Impl::ReturnUnderlyingResource(this, core::mem::transmute_copy(&presource11), core::mem::transmute_copy(&numsync), core::mem::transmute_copy(&psignalvalues), core::mem::transmute_copy(&ppfences)).into()
            }
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
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl windows_core::RuntimeName for ID3D11On12Device2 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
pub type PFN_D3D11ON12_CREATE_DEVICE = Option<unsafe extern "system" fn(param0: windows_core::Ref<windows_core::IUnknown>, param1: u32, param2: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, param4: *const Option<windows_core::IUnknown>, numqueues: u32, param6: u32, param7: windows_core::OutRef<super::Direct3D11::ID3D11Device>, param8: windows_core::OutRef<super::Direct3D11::ID3D11DeviceContext>, param9: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> windows_core::HRESULT>;
