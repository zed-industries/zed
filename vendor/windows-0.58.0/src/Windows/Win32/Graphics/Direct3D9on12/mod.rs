#[cfg(feature = "Win32_Graphics_Direct3D9")]
#[inline]
pub unsafe fn Direct3DCreate9On12(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> Option<super::Direct3D9::IDirect3D9> {
    windows_targets::link!("d3d9.dll" "system" fn Direct3DCreate9On12(sdkversion : u32, poverridelist : *mut D3D9ON12_ARGS, numoverrideentries : u32) -> Option < super::Direct3D9:: IDirect3D9 >);
    Direct3DCreate9On12(sdkversion, poverridelist, numoverrideentries)
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
#[inline]
pub unsafe fn Direct3DCreate9On12Ex(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut Option<super::Direct3D9::IDirect3D9Ex>) -> windows_core::Result<()> {
    windows_targets::link!("d3d9.dll" "system" fn Direct3DCreate9On12Ex(sdkversion : u32, poverridelist : *mut D3D9ON12_ARGS, numoverrideentries : u32, ppoutputinterface : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    Direct3DCreate9On12Ex(sdkversion, poverridelist, numoverrideentries, core::mem::transmute(ppoutputinterface)).ok()
}
windows_core::imp::define_interface!(IDirect3DDevice9On12, IDirect3DDevice9On12_Vtbl, 0xe7fda234_b589_4049_940d_8878977531c8);
impl core::ops::Deref for IDirect3DDevice9On12 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DDevice9On12, windows_core::IUnknown);
impl IDirect3DDevice9On12 {
    pub unsafe fn GetD3D12Device(&self, riid: *const windows_core::GUID, ppvdevice: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetD3D12Device)(windows_core::Interface::as_raw(self), riid, ppvdevice).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub unsafe fn UnwrapUnderlyingResource<P0, P1>(&self, presource: P0, pcommandqueue: P1, riid: *const windows_core::GUID, ppvresource12: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Direct3D9::IDirect3DResource9>,
        P1: windows_core::Param<super::Direct3D12::ID3D12CommandQueue>,
    {
        (windows_core::Interface::vtable(self).UnwrapUnderlyingResource)(windows_core::Interface::as_raw(self), presource.param().abi(), pcommandqueue.param().abi(), riid, ppvresource12).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub unsafe fn ReturnUnderlyingResource<P0>(&self, presource: P0, numsync: u32, psignalvalues: *mut u64, ppfences: *mut Option<super::Direct3D12::ID3D12Fence>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Direct3D9::IDirect3DResource9>,
    {
        (windows_core::Interface::vtable(self).ReturnUnderlyingResource)(windows_core::Interface::as_raw(self), presource.param().abi(), numsync, psignalvalues, core::mem::transmute(ppfences)).ok()
    }
}
#[repr(C)]
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
pub const MAX_D3D9ON12_QUEUES: u32 = 2u32;
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct D3D9ON12_ARGS {
    pub Enable9On12: super::super::Foundation::BOOL,
    pub pD3D12Device: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub ppD3D12Queues: [core::mem::ManuallyDrop<Option<windows_core::IUnknown>>; 2usize],
    pub NumQueues: u32,
    pub NodeMask: u32,
}
impl Clone for D3D9ON12_ARGS {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D9ON12_ARGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D9ON12_ARGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub type PFN_Direct3DCreate9On12 = Option<unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> Option<super::Direct3D9::IDirect3D9>>;
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub type PFN_Direct3DCreate9On12Ex = Option<unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut Option<super::Direct3D9::IDirect3D9Ex>) -> windows_core::HRESULT>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
