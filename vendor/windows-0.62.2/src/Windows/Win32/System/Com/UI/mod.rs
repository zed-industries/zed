windows_core::imp::define_interface!(IDummyHICONIncluder, IDummyHICONIncluder_Vtbl, 0x947990de_cc28_11d2_a0f7_00805f858fb1);
windows_core::imp::interface_hierarchy!(IDummyHICONIncluder, windows_core::IUnknown);
impl IDummyHICONIncluder {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn Dummy(&self, h1: super::super::super::UI::WindowsAndMessaging::HICON, h2: super::super::super::Graphics::Gdi::HDC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Dummy)(windows_core::Interface::as_raw(self), h1, h2).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDummyHICONIncluder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub Dummy: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::UI::WindowsAndMessaging::HICON, super::super::super::Graphics::Gdi::HDC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    Dummy: usize,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDummyHICONIncluder_Impl: windows_core::IUnknownImpl {
    fn Dummy(&self, h1: super::super::super::UI::WindowsAndMessaging::HICON, h2: super::super::super::Graphics::Gdi::HDC) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDummyHICONIncluder_Vtbl {
    pub const fn new<Identity: IDummyHICONIncluder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Dummy<Identity: IDummyHICONIncluder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, h1: super::super::super::UI::WindowsAndMessaging::HICON, h2: super::super::super::Graphics::Gdi::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDummyHICONIncluder_Impl::Dummy(this, core::mem::transmute_copy(&h1), core::mem::transmute_copy(&h2)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Dummy: Dummy::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDummyHICONIncluder as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IDummyHICONIncluder {}
windows_core::imp::define_interface!(IThumbnailExtractor, IThumbnailExtractor_Vtbl, 0x969dc708_5c76_11d1_8d86_0000f804b057);
windows_core::imp::interface_hierarchy!(IThumbnailExtractor, windows_core::IUnknown);
impl IThumbnailExtractor {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn ExtractThumbnail<P0>(&self, pstg: P0, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::StructuredStorage::IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExtractThumbnail)(windows_core::Interface::as_raw(self), pstg.param().abi(), ullength, ulheight, puloutputlength as _, puloutputheight as _, phoutputbitmap as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn OnFileUpdated<P0>(&self, pstg: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::StructuredStorage::IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnFileUpdated)(windows_core::Interface::as_raw(self), pstg.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IThumbnailExtractor_Impl: windows_core::IUnknownImpl {
    fn ExtractThumbnail(&self, pstg: windows_core::Ref<super::StructuredStorage::IStorage>, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> windows_core::Result<()>;
    fn OnFileUpdated(&self, pstg: windows_core::Ref<super::StructuredStorage::IStorage>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IThumbnailExtractor_Vtbl {
    pub const fn new<Identity: IThumbnailExtractor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ExtractThumbnail<Identity: IThumbnailExtractor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstg: *mut core::ffi::c_void, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IThumbnailExtractor_Impl::ExtractThumbnail(this, core::mem::transmute_copy(&pstg), core::mem::transmute_copy(&ullength), core::mem::transmute_copy(&ulheight), core::mem::transmute_copy(&puloutputlength), core::mem::transmute_copy(&puloutputheight), core::mem::transmute_copy(&phoutputbitmap)).into()
            }
        }
        unsafe extern "system" fn OnFileUpdated<Identity: IThumbnailExtractor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IThumbnailExtractor_Impl::OnFileUpdated(this, core::mem::transmute_copy(&pstg)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ExtractThumbnail: ExtractThumbnail::<Identity, OFFSET>,
            OnFileUpdated: OnFileUpdated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IThumbnailExtractor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IThumbnailExtractor {}
