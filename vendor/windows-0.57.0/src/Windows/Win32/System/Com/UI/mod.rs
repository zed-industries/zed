windows_core::imp::define_interface!(IDummyHICONIncluder, IDummyHICONIncluder_Vtbl, 0x947990de_cc28_11d2_a0f7_00805f858fb1);
impl core::ops::Deref for IDummyHICONIncluder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDummyHICONIncluder, windows_core::IUnknown);
impl IDummyHICONIncluder {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn Dummy<P0, P1>(&self, h1: P0, h2: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::UI::WindowsAndMessaging::HICON>,
        P1: windows_core::Param<super::super::super::Graphics::Gdi::HDC>,
    {
        (windows_core::Interface::vtable(self).Dummy)(windows_core::Interface::as_raw(self), h1.param().abi(), h2.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDummyHICONIncluder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub Dummy: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::UI::WindowsAndMessaging::HICON, super::super::super::Graphics::Gdi::HDC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    Dummy: usize,
}
windows_core::imp::define_interface!(IThumbnailExtractor, IThumbnailExtractor_Vtbl, 0x969dc708_5c76_11d1_8d86_0000f804b057);
impl core::ops::Deref for IThumbnailExtractor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IThumbnailExtractor, windows_core::IUnknown);
impl IThumbnailExtractor {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn ExtractThumbnail<P0>(&self, pstg: P0, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::StructuredStorage::IStorage>,
    {
        (windows_core::Interface::vtable(self).ExtractThumbnail)(windows_core::Interface::as_raw(self), pstg.param().abi(), ullength, ulheight, puloutputlength, puloutputheight, phoutputbitmap).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn OnFileUpdated<P0>(&self, pstg: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::StructuredStorage::IStorage>,
    {
        (windows_core::Interface::vtable(self).OnFileUpdated)(windows_core::Interface::as_raw(self), pstg.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IThumbnailExtractor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub ExtractThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut u32, *mut u32, *mut super::super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    ExtractThumbnail: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub OnFileUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    OnFileUpdated: usize,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
