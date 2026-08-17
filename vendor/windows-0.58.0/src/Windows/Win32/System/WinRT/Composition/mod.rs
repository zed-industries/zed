windows_core::imp::define_interface!(ICompositionCapabilitiesInteropFactory, ICompositionCapabilitiesInteropFactory_Vtbl, 0x2c9db356_e70d_4642_8298_bc4aa5b4865c);
impl core::ops::Deref for ICompositionCapabilitiesInteropFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICompositionCapabilitiesInteropFactory, windows_core::IUnknown, windows_core::IInspectable);
impl ICompositionCapabilitiesInteropFactory {
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn GetForWindow<P0>(&self, hwnd: P0) -> windows_core::Result<super::super::super::super::UI::Composition::CompositionCapabilities>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), hwnd.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICompositionCapabilitiesInteropFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetForWindow: usize,
}
windows_core::imp::define_interface!(ICompositionDrawingSurfaceInterop, ICompositionDrawingSurfaceInterop_Vtbl, 0xfd04e6e3_fe0c_4c3c_ab19_a07601a576ee);
impl core::ops::Deref for ICompositionDrawingSurfaceInterop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICompositionDrawingSurfaceInterop, windows_core::IUnknown);
impl ICompositionDrawingSurfaceInterop {
    pub unsafe fn BeginDraw<T>(&self, updaterect: Option<*const super::super::super::Foundation::RECT>, updateoffset: *mut super::super::super::Foundation::POINT) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).BeginDraw)(windows_core::Interface::as_raw(self), core::mem::transmute(updaterect.unwrap_or(std::ptr::null())), &T::IID, &mut result__, updateoffset).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndDraw(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndDraw)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resize(&self, sizepixels: super::super::super::Foundation::SIZE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resize)(windows_core::Interface::as_raw(self), core::mem::transmute(sizepixels)).ok()
    }
    pub unsafe fn Scroll(&self, scrollrect: Option<*const super::super::super::Foundation::RECT>, cliprect: Option<*const super::super::super::Foundation::RECT>, offsetx: i32, offsety: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Scroll)(windows_core::Interface::as_raw(self), core::mem::transmute(scrollrect.unwrap_or(std::ptr::null())), core::mem::transmute(cliprect.unwrap_or(std::ptr::null())), offsetx, offsety).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResumeDraw)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SuspendDraw)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ICompositionDrawingSurfaceInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginDraw: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::RECT, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut super::super::super::Foundation::POINT) -> windows_core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::SIZE) -> windows_core::HRESULT,
    pub Scroll: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::RECT, *const super::super::super::Foundation::RECT, i32, i32) -> windows_core::HRESULT,
    pub ResumeDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompositionDrawingSurfaceInterop2, ICompositionDrawingSurfaceInterop2_Vtbl, 0x41e64aae_98c0_4239_8e95_a330dd6aa18b);
impl core::ops::Deref for ICompositionDrawingSurfaceInterop2 {
    type Target = ICompositionDrawingSurfaceInterop;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICompositionDrawingSurfaceInterop2, windows_core::IUnknown, ICompositionDrawingSurfaceInterop);
impl ICompositionDrawingSurfaceInterop2 {
    pub unsafe fn CopySurface<P0>(&self, destinationresource: P0, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: Option<*const super::super::super::Foundation::RECT>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CopySurface)(windows_core::Interface::as_raw(self), destinationresource.param().abi(), destinationoffsetx, destinationoffsety, core::mem::transmute(sourcerectangle.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct ICompositionDrawingSurfaceInterop2_Vtbl {
    pub base__: ICompositionDrawingSurfaceInterop_Vtbl,
    pub CopySurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *const super::super::super::Foundation::RECT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompositionGraphicsDeviceInterop, ICompositionGraphicsDeviceInterop_Vtbl, 0xa116ff71_f8bf_4c8a_9c98_70779a32a9c8);
impl core::ops::Deref for ICompositionGraphicsDeviceInterop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICompositionGraphicsDeviceInterop, windows_core::IUnknown);
impl ICompositionGraphicsDeviceInterop {
    pub unsafe fn GetRenderingDevice(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRenderingDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRenderingDevice<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetRenderingDevice)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICompositionGraphicsDeviceInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRenderingDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRenderingDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompositionTextureInterop, ICompositionTextureInterop_Vtbl, 0xd528a265_f0a5_422f_a39d_ef62d7cd1cc4);
impl core::ops::Deref for ICompositionTextureInterop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICompositionTextureInterop, windows_core::IUnknown);
impl ICompositionTextureInterop {
    pub unsafe fn GetAvailableFence(&self, fencevalue: *mut u64, iid: *const windows_core::GUID, availablefence: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAvailableFence)(windows_core::Interface::as_raw(self), fencevalue, iid, availablefence).ok()
    }
}
#[repr(C)]
pub struct ICompositionTextureInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAvailableFence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompositorDesktopInterop, ICompositorDesktopInterop_Vtbl, 0x29e691fa_4567_4dca_b319_d0f207eb6807);
impl core::ops::Deref for ICompositorDesktopInterop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICompositorDesktopInterop, windows_core::IUnknown);
impl ICompositorDesktopInterop {
    #[cfg(feature = "UI_Composition_Desktop")]
    pub unsafe fn CreateDesktopWindowTarget<P0, P1>(&self, hwndtarget: P0, istopmost: P1) -> windows_core::Result<super::super::super::super::UI::Composition::Desktop::DesktopWindowTarget>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDesktopWindowTarget)(windows_core::Interface::as_raw(self), hwndtarget.param().abi(), istopmost.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnsureOnThread(&self, threadid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnsureOnThread)(windows_core::Interface::as_raw(self), threadid).ok()
    }
}
#[repr(C)]
pub struct ICompositorDesktopInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "UI_Composition_Desktop")]
    pub CreateDesktopWindowTarget: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, super::super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition_Desktop"))]
    CreateDesktopWindowTarget: usize,
    pub EnsureOnThread: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompositorInterop, ICompositorInterop_Vtbl, 0x25297d5c_3ad4_4c9c_b5cf_e36a38512330);
impl core::ops::Deref for ICompositorInterop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICompositorInterop, windows_core::IUnknown);
impl ICompositorInterop {
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateCompositionSurfaceForHandle<P0>(&self, swapchain: P0) -> windows_core::Result<super::super::super::super::UI::Composition::ICompositionSurface>
    where
        P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCompositionSurfaceForHandle)(windows_core::Interface::as_raw(self), swapchain.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateCompositionSurfaceForSwapChain<P0>(&self, swapchain: P0) -> windows_core::Result<super::super::super::super::UI::Composition::ICompositionSurface>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCompositionSurfaceForSwapChain)(windows_core::Interface::as_raw(self), swapchain.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateGraphicsDevice<P0>(&self, renderingdevice: P0) -> windows_core::Result<super::super::super::super::UI::Composition::CompositionGraphicsDevice>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGraphicsDevice)(windows_core::Interface::as_raw(self), renderingdevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICompositorInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateCompositionSurfaceForHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HANDLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateCompositionSurfaceForHandle: usize,
    #[cfg(feature = "UI_Composition")]
    pub CreateCompositionSurfaceForSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateCompositionSurfaceForSwapChain: usize,
    #[cfg(feature = "UI_Composition")]
    pub CreateGraphicsDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateGraphicsDevice: usize,
}
windows_core::imp::define_interface!(ICompositorInterop2, ICompositorInterop2_Vtbl, 0xd3eef34c_0667_4afc_8d13_867607b0fe91);
impl core::ops::Deref for ICompositorInterop2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICompositorInterop2, windows_core::IUnknown);
impl ICompositorInterop2 {
    pub unsafe fn CheckCompositionTextureSupport<P0>(&self, renderingdevice: P0) -> windows_core::Result<super::super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckCompositionTextureSupport)(windows_core::Interface::as_raw(self), renderingdevice.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateCompositionTexture<P0>(&self, d3dtexture: P0) -> windows_core::Result<super::super::super::super::UI::Composition::CompositionTexture>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCompositionTexture)(windows_core::Interface::as_raw(self), d3dtexture.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICompositorInterop2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CheckCompositionTextureSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub CreateCompositionTexture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateCompositionTexture: usize,
}
windows_core::imp::define_interface!(IDesktopWindowTargetInterop, IDesktopWindowTargetInterop_Vtbl, 0x35dbf59e_e3f9_45b0_81e7_fe75f4145dc9);
impl core::ops::Deref for IDesktopWindowTargetInterop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDesktopWindowTargetInterop, windows_core::IUnknown);
impl IDesktopWindowTargetInterop {
    pub unsafe fn Hwnd(&self) -> windows_core::Result<super::super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Hwnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDesktopWindowTargetInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Hwnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVisualInteractionSourceInterop, IVisualInteractionSourceInterop_Vtbl, 0x11f62cd1_2f9d_42d3_b05f_d6790d9e9f8e);
impl core::ops::Deref for IVisualInteractionSourceInterop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVisualInteractionSourceInterop, windows_core::IUnknown);
impl IVisualInteractionSourceInterop {
    #[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TryRedirectForManipulation(&self, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TryRedirectForManipulation)(windows_core::Interface::as_raw(self), pointerinfo).ok()
    }
}
#[repr(C)]
pub struct IVisualInteractionSourceInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TryRedirectForManipulation: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging")))]
    TryRedirectForManipulation: usize,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
