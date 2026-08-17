#[cfg(feature = "Win32_Graphics_Direct3D9")]
#[inline]
pub unsafe fn Direct3DCreate9On12(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> Option<super::Direct3D9::IDirect3D9> {
    windows_core::link!("d3d9.dll" "system" fn Direct3DCreate9On12(sdkversion : u32, poverridelist : *mut D3D9ON12_ARGS, numoverrideentries : u32) -> Option < super::Direct3D9:: IDirect3D9 >);
    unsafe { Direct3DCreate9On12(sdkversion, core::mem::transmute(poverridelist), numoverrideentries) }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
#[inline]
pub unsafe fn Direct3DCreate9On12Ex(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut Option<super::Direct3D9::IDirect3D9Ex>) -> windows_core::Result<()> {
    windows_core::link!("d3d9.dll" "system" fn Direct3DCreate9On12Ex(sdkversion : u32, poverridelist : *mut D3D9ON12_ARGS, numoverrideentries : u32, ppoutputinterface : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { Direct3DCreate9On12Ex(sdkversion, core::mem::transmute(poverridelist), numoverrideentries, core::mem::transmute(ppoutputinterface)).ok() }
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct D3D9ON12_ARGS {
    pub Enable9On12: windows_core::BOOL,
    pub pD3D12Device: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub ppD3D12Queues: [core::mem::ManuallyDrop<Option<windows_core::IUnknown>>; 2],
    pub NumQueues: u32,
    pub NodeMask: u32,
}
impl Default for D3D9ON12_ARGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(IDirect3DDevice9On12, IDirect3DDevice9On12_Vtbl, 0xe7fda234_b589_4049_940d_8878977531c8);
windows_core::imp::interface_hierarchy!(IDirect3DDevice9On12, windows_core::IUnknown);
impl IDirect3DDevice9On12 {
    pub unsafe fn GetD3D12Device(&self, riid: *const windows_core::GUID, ppvdevice: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetD3D12Device)(windows_core::Interface::as_raw(self), riid, ppvdevice as _).ok() }
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub unsafe fn UnwrapUnderlyingResource<P0, P1>(&self, presource: P0, pcommandqueue: P1, riid: *const windows_core::GUID, ppvresource12: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Direct3D9::IDirect3DResource9>,
        P1: windows_core::Param<super::Direct3D12::ID3D12CommandQueue>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnwrapUnderlyingResource)(windows_core::Interface::as_raw(self), presource.param().abi(), pcommandqueue.param().abi(), riid, ppvresource12 as _).ok() }
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub unsafe fn ReturnUnderlyingResource<P0>(&self, presource: P0, numsync: u32, psignalvalues: *mut u64, ppfences: *mut Option<super::Direct3D12::ID3D12Fence>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Direct3D9::IDirect3DResource9>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReturnUnderlyingResource)(windows_core::Interface::as_raw(self), presource.param().abi(), numsync, psignalvalues as _, core::mem::transmute(ppfences)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice9On12_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetD3D12Device: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub UnwrapUnderlyingResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9")))]
    UnwrapUnderlyingResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub ReturnUnderlyingResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9")))]
    ReturnUnderlyingResource: usize,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
pub trait IDirect3DDevice9On12_Impl: windows_core::IUnknownImpl {
    fn GetD3D12Device(&self, riid: *const windows_core::GUID, ppvdevice: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn UnwrapUnderlyingResource(&self, presource: windows_core::Ref<super::Direct3D9::IDirect3DResource9>, pcommandqueue: windows_core::Ref<super::Direct3D12::ID3D12CommandQueue>, riid: *const windows_core::GUID, ppvresource12: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ReturnUnderlyingResource(&self, presource: windows_core::Ref<super::Direct3D9::IDirect3DResource9>, numsync: u32, psignalvalues: *mut u64, ppfences: windows_core::OutRef<super::Direct3D12::ID3D12Fence>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
impl IDirect3DDevice9On12_Vtbl {
    pub const fn new<Identity: IDirect3DDevice9On12_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetD3D12Device<Identity: IDirect3DDevice9On12_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9On12_Impl::GetD3D12Device(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvdevice)).into()
            }
        }
        unsafe extern "system" fn UnwrapUnderlyingResource<Identity: IDirect3DDevice9On12_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pcommandqueue: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvresource12: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9On12_Impl::UnwrapUnderlyingResource(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pcommandqueue), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvresource12)).into()
            }
        }
        unsafe extern "system" fn ReturnUnderlyingResource<Identity: IDirect3DDevice9On12_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, numsync: u32, psignalvalues: *mut u64, ppfences: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9On12_Impl::ReturnUnderlyingResource(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&numsync), core::mem::transmute_copy(&psignalvalues), core::mem::transmute_copy(&ppfences)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetD3D12Device: GetD3D12Device::<Identity, OFFSET>,
            UnwrapUnderlyingResource: UnwrapUnderlyingResource::<Identity, OFFSET>,
            ReturnUnderlyingResource: ReturnUnderlyingResource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DDevice9On12 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
impl windows_core::RuntimeName for IDirect3DDevice9On12 {}
pub const MAX_D3D9ON12_QUEUES: u32 = 2u32;
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub type PFN_Direct3DCreate9On12 = Option<unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> Option<super::Direct3D9::IDirect3D9>>;
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub type PFN_Direct3DCreate9On12Ex = Option<unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: windows_core::OutRef<super::Direct3D9::IDirect3D9Ex>) -> windows_core::HRESULT>;
