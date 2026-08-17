#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn CreateDirect3D11DeviceFromDXGIDevice<P0>(dxgidevice: P0) -> windows_core::Result<windows_core::IInspectable>
where
    P0: windows_core::Param<super::super::super::Graphics::Dxgi::IDXGIDevice>,
{
    windows_targets::link!("d3d11.dll" "system" fn CreateDirect3D11DeviceFromDXGIDevice(dxgidevice : * mut core::ffi::c_void, graphicsdevice : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateDirect3D11DeviceFromDXGIDevice(dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn CreateDirect3D11SurfaceFromDXGISurface<P0>(dgxisurface: P0) -> windows_core::Result<windows_core::IInspectable>
where
    P0: windows_core::Param<super::super::super::Graphics::Dxgi::IDXGISurface>,
{
    windows_targets::link!("d3d11.dll" "system" fn CreateDirect3D11SurfaceFromDXGISurface(dgxisurface : * mut core::ffi::c_void, graphicssurface : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateDirect3D11SurfaceFromDXGISurface(dgxisurface.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
windows_core::imp::define_interface!(IDirect3DDxgiInterfaceAccess, IDirect3DDxgiInterfaceAccess_Vtbl, 0xa9b3d012_3df2_4ee3_b8d1_8695f457d3c1);
impl core::ops::Deref for IDirect3DDxgiInterfaceAccess {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DDxgiInterfaceAccess, windows_core::IUnknown);
impl IDirect3DDxgiInterfaceAccess {
    pub unsafe fn GetInterface<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetInterface)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDirect3DDxgiInterfaceAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
