#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::Win32::Graphics::Dxgi;

windows_core::imp::define_interface!(
    ISwapChainPanelNative,
    ISwapChainPanelNative_Vtbl,
    0x63aad0b8_7c24_40ff_85a8_640d944cc325
);
impl core::ops::Deref for ISwapChainPanelNative {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISwapChainPanelNative, windows_core::IUnknown);
impl ISwapChainPanelNative {
    pub unsafe fn SetSwapChain<P0>(&self, swap_chain: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Dxgi::IDXGISwapChain1>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetSwapChain)(
                windows_core::Interface::as_raw(self),
                swap_chain.param().abi(),
            )
        }
        .ok()
    }
}
#[repr(C)]
pub struct ISwapChainPanelNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSwapChain: unsafe extern "system" fn(
        swap_chain_panel_native: *mut core::ffi::c_void,
        swap_chain: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
