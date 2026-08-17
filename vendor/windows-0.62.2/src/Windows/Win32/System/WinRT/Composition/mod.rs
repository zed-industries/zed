windows_core::imp::define_interface!(ICompositionCapabilitiesInteropFactory, ICompositionCapabilitiesInteropFactory_Vtbl, 0x2c9db356_e70d_4642_8298_bc4aa5b4865c);
windows_core::imp::interface_hierarchy!(ICompositionCapabilitiesInteropFactory, windows_core::IUnknown, windows_core::IInspectable);
impl ICompositionCapabilitiesInteropFactory {
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn GetForWindow(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<super::super::super::super::UI::Composition::CompositionCapabilities> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), hwnd, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionCapabilitiesInteropFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetForWindow: usize,
}
#[cfg(feature = "UI_Composition")]
pub trait ICompositionCapabilitiesInteropFactory_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<super::super::super::super::UI::Composition::CompositionCapabilities>;
}
#[cfg(feature = "UI_Composition")]
impl ICompositionCapabilitiesInteropFactory_Vtbl {
    pub const fn new<Identity: ICompositionCapabilitiesInteropFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: ICompositionCapabilitiesInteropFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICompositionCapabilitiesInteropFactory_Impl::GetForWindow(this, core::mem::transmute_copy(&hwnd)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICompositionCapabilitiesInteropFactory, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionCapabilitiesInteropFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for ICompositionCapabilitiesInteropFactory {}
windows_core::imp::define_interface!(ICompositionDrawingSurfaceInterop, ICompositionDrawingSurfaceInterop_Vtbl, 0xfd04e6e3_fe0c_4c3c_ab19_a07601a576ee);
windows_core::imp::interface_hierarchy!(ICompositionDrawingSurfaceInterop, windows_core::IUnknown);
impl ICompositionDrawingSurfaceInterop {
    pub unsafe fn BeginDraw<T>(&self, updaterect: Option<*const super::super::super::Foundation::RECT>, updateoffset: *mut super::super::super::Foundation::POINT) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).BeginDraw)(windows_core::Interface::as_raw(self), updaterect.unwrap_or(core::mem::zeroed()) as _, &T::IID, &mut result__, updateoffset as _).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn EndDraw(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndDraw)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Resize(&self, sizepixels: super::super::super::Foundation::SIZE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Resize)(windows_core::Interface::as_raw(self), core::mem::transmute(sizepixels)).ok() }
    }
    pub unsafe fn Scroll(&self, scrollrect: Option<*const super::super::super::Foundation::RECT>, cliprect: Option<*const super::super::super::Foundation::RECT>, offsetx: i32, offsety: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Scroll)(windows_core::Interface::as_raw(self), scrollrect.unwrap_or(core::mem::zeroed()) as _, cliprect.unwrap_or(core::mem::zeroed()) as _, offsetx, offsety).ok() }
    }
    pub unsafe fn ResumeDraw(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResumeDraw)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SuspendDraw(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SuspendDraw)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDrawingSurfaceInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginDraw: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::RECT, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut super::super::super::Foundation::POINT) -> windows_core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::SIZE) -> windows_core::HRESULT,
    pub Scroll: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::RECT, *const super::super::super::Foundation::RECT, i32, i32) -> windows_core::HRESULT,
    pub ResumeDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICompositionDrawingSurfaceInterop_Impl: windows_core::IUnknownImpl {
    fn BeginDraw(&self, updaterect: *const super::super::super::Foundation::RECT, iid: *const windows_core::GUID, updateobject: *mut *mut core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> windows_core::Result<()>;
    fn EndDraw(&self) -> windows_core::Result<()>;
    fn Resize(&self, sizepixels: &super::super::super::Foundation::SIZE) -> windows_core::Result<()>;
    fn Scroll(&self, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> windows_core::Result<()>;
    fn ResumeDraw(&self) -> windows_core::Result<()>;
    fn SuspendDraw(&self) -> windows_core::Result<()>;
}
impl ICompositionDrawingSurfaceInterop_Vtbl {
    pub const fn new<Identity: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginDraw<Identity: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updaterect: *const super::super::super::Foundation::RECT, iid: *const windows_core::GUID, updateobject: *mut *mut core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositionDrawingSurfaceInterop_Impl::BeginDraw(this, core::mem::transmute_copy(&updaterect), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&updateobject), core::mem::transmute_copy(&updateoffset)).into()
            }
        }
        unsafe extern "system" fn EndDraw<Identity: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositionDrawingSurfaceInterop_Impl::EndDraw(this).into()
            }
        }
        unsafe extern "system" fn Resize<Identity: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sizepixels: super::super::super::Foundation::SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositionDrawingSurfaceInterop_Impl::Resize(this, core::mem::transmute(&sizepixels)).into()
            }
        }
        unsafe extern "system" fn Scroll<Identity: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositionDrawingSurfaceInterop_Impl::Scroll(this, core::mem::transmute_copy(&scrollrect), core::mem::transmute_copy(&cliprect), core::mem::transmute_copy(&offsetx), core::mem::transmute_copy(&offsety)).into()
            }
        }
        unsafe extern "system" fn ResumeDraw<Identity: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositionDrawingSurfaceInterop_Impl::ResumeDraw(this).into()
            }
        }
        unsafe extern "system" fn SuspendDraw<Identity: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositionDrawingSurfaceInterop_Impl::SuspendDraw(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginDraw: BeginDraw::<Identity, OFFSET>,
            EndDraw: EndDraw::<Identity, OFFSET>,
            Resize: Resize::<Identity, OFFSET>,
            Scroll: Scroll::<Identity, OFFSET>,
            ResumeDraw: ResumeDraw::<Identity, OFFSET>,
            SuspendDraw: SuspendDraw::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionDrawingSurfaceInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICompositionDrawingSurfaceInterop {}
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
        unsafe { (windows_core::Interface::vtable(self).CopySurface)(windows_core::Interface::as_raw(self), destinationresource.param().abi(), destinationoffsetx, destinationoffsety, sourcerectangle.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDrawingSurfaceInterop2_Vtbl {
    pub base__: ICompositionDrawingSurfaceInterop_Vtbl,
    pub CopySurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *const super::super::super::Foundation::RECT) -> windows_core::HRESULT,
}
pub trait ICompositionDrawingSurfaceInterop2_Impl: ICompositionDrawingSurfaceInterop_Impl {
    fn CopySurface(&self, destinationresource: windows_core::Ref<windows_core::IUnknown>, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> windows_core::Result<()>;
}
impl ICompositionDrawingSurfaceInterop2_Vtbl {
    pub const fn new<Identity: ICompositionDrawingSurfaceInterop2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CopySurface<Identity: ICompositionDrawingSurfaceInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationresource: *mut core::ffi::c_void, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositionDrawingSurfaceInterop2_Impl::CopySurface(this, core::mem::transmute_copy(&destinationresource), core::mem::transmute_copy(&destinationoffsetx), core::mem::transmute_copy(&destinationoffsety), core::mem::transmute_copy(&sourcerectangle)).into()
            }
        }
        Self { base__: ICompositionDrawingSurfaceInterop_Vtbl::new::<Identity, OFFSET>(), CopySurface: CopySurface::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionDrawingSurfaceInterop2 as windows_core::Interface>::IID || iid == &<ICompositionDrawingSurfaceInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICompositionDrawingSurfaceInterop2 {}
windows_core::imp::define_interface!(ICompositionGraphicsDeviceInterop, ICompositionGraphicsDeviceInterop_Vtbl, 0xa116ff71_f8bf_4c8a_9c98_70779a32a9c8);
windows_core::imp::interface_hierarchy!(ICompositionGraphicsDeviceInterop, windows_core::IUnknown);
impl ICompositionGraphicsDeviceInterop {
    pub unsafe fn GetRenderingDevice(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRenderingDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetRenderingDevice<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRenderingDevice)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionGraphicsDeviceInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRenderingDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRenderingDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICompositionGraphicsDeviceInterop_Impl: windows_core::IUnknownImpl {
    fn GetRenderingDevice(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SetRenderingDevice(&self, value: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl ICompositionGraphicsDeviceInterop_Vtbl {
    pub const fn new<Identity: ICompositionGraphicsDeviceInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRenderingDevice<Identity: ICompositionGraphicsDeviceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICompositionGraphicsDeviceInterop_Impl::GetRenderingDevice(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRenderingDevice<Identity: ICompositionGraphicsDeviceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositionGraphicsDeviceInterop_Impl::SetRenderingDevice(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRenderingDevice: GetRenderingDevice::<Identity, OFFSET>,
            SetRenderingDevice: SetRenderingDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionGraphicsDeviceInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICompositionGraphicsDeviceInterop {}
windows_core::imp::define_interface!(ICompositionTextureInterop, ICompositionTextureInterop_Vtbl, 0xd528a265_f0a5_422f_a39d_ef62d7cd1cc4);
windows_core::imp::interface_hierarchy!(ICompositionTextureInterop, windows_core::IUnknown);
impl ICompositionTextureInterop {
    pub unsafe fn GetAvailableFence(&self, fencevalue: *mut u64, iid: *const windows_core::GUID, availablefence: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAvailableFence)(windows_core::Interface::as_raw(self), fencevalue as _, iid, availablefence as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionTextureInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAvailableFence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICompositionTextureInterop_Impl: windows_core::IUnknownImpl {
    fn GetAvailableFence(&self, fencevalue: *mut u64, iid: *const windows_core::GUID, availablefence: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ICompositionTextureInterop_Vtbl {
    pub const fn new<Identity: ICompositionTextureInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAvailableFence<Identity: ICompositionTextureInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fencevalue: *mut u64, iid: *const windows_core::GUID, availablefence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositionTextureInterop_Impl::GetAvailableFence(this, core::mem::transmute_copy(&fencevalue), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&availablefence)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetAvailableFence: GetAvailableFence::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionTextureInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICompositionTextureInterop {}
windows_core::imp::define_interface!(ICompositorDesktopInterop, ICompositorDesktopInterop_Vtbl, 0x29e691fa_4567_4dca_b319_d0f207eb6807);
windows_core::imp::interface_hierarchy!(ICompositorDesktopInterop, windows_core::IUnknown);
impl ICompositorDesktopInterop {
    #[cfg(feature = "UI_Composition_Desktop")]
    pub unsafe fn CreateDesktopWindowTarget(&self, hwndtarget: super::super::super::Foundation::HWND, istopmost: bool) -> windows_core::Result<super::super::super::super::UI::Composition::Desktop::DesktopWindowTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDesktopWindowTarget)(windows_core::Interface::as_raw(self), hwndtarget, istopmost.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnsureOnThread(&self, threadid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnsureOnThread)(windows_core::Interface::as_raw(self), threadid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositorDesktopInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "UI_Composition_Desktop")]
    pub CreateDesktopWindowTarget: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition_Desktop"))]
    CreateDesktopWindowTarget: usize,
    pub EnsureOnThread: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "UI_Composition_Desktop")]
pub trait ICompositorDesktopInterop_Impl: windows_core::IUnknownImpl {
    fn CreateDesktopWindowTarget(&self, hwndtarget: super::super::super::Foundation::HWND, istopmost: windows_core::BOOL) -> windows_core::Result<super::super::super::super::UI::Composition::Desktop::DesktopWindowTarget>;
    fn EnsureOnThread(&self, threadid: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "UI_Composition_Desktop")]
impl ICompositorDesktopInterop_Vtbl {
    pub const fn new<Identity: ICompositorDesktopInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDesktopWindowTarget<Identity: ICompositorDesktopInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndtarget: super::super::super::Foundation::HWND, istopmost: windows_core::BOOL, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICompositorDesktopInterop_Impl::CreateDesktopWindowTarget(this, core::mem::transmute_copy(&hwndtarget), core::mem::transmute_copy(&istopmost)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnsureOnThread<Identity: ICompositorDesktopInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositorDesktopInterop_Impl::EnsureOnThread(this, core::mem::transmute_copy(&threadid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDesktopWindowTarget: CreateDesktopWindowTarget::<Identity, OFFSET>,
            EnsureOnThread: EnsureOnThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositorDesktopInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "UI_Composition_Desktop")]
impl windows_core::RuntimeName for ICompositorDesktopInterop {}
windows_core::imp::define_interface!(ICompositorInterop, ICompositorInterop_Vtbl, 0x25297d5c_3ad4_4c9c_b5cf_e36a38512330);
windows_core::imp::interface_hierarchy!(ICompositorInterop, windows_core::IUnknown);
impl ICompositorInterop {
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateCompositionSurfaceForHandle(&self, swapchain: super::super::super::Foundation::HANDLE) -> windows_core::Result<super::super::super::super::UI::Composition::ICompositionSurface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCompositionSurfaceForHandle)(windows_core::Interface::as_raw(self), swapchain, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateCompositionSurfaceForSwapChain<P0>(&self, swapchain: P0) -> windows_core::Result<super::super::super::super::UI::Composition::ICompositionSurface>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCompositionSurfaceForSwapChain)(windows_core::Interface::as_raw(self), swapchain.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateGraphicsDevice<P0>(&self, renderingdevice: P0) -> windows_core::Result<super::super::super::super::UI::Composition::CompositionGraphicsDevice>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGraphicsDevice)(windows_core::Interface::as_raw(self), renderingdevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "UI_Composition")]
pub trait ICompositorInterop_Impl: windows_core::IUnknownImpl {
    fn CreateCompositionSurfaceForHandle(&self, swapchain: super::super::super::Foundation::HANDLE) -> windows_core::Result<super::super::super::super::UI::Composition::ICompositionSurface>;
    fn CreateCompositionSurfaceForSwapChain(&self, swapchain: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<super::super::super::super::UI::Composition::ICompositionSurface>;
    fn CreateGraphicsDevice(&self, renderingdevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<super::super::super::super::UI::Composition::CompositionGraphicsDevice>;
}
#[cfg(feature = "UI_Composition")]
impl ICompositorInterop_Vtbl {
    pub const fn new<Identity: ICompositorInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateCompositionSurfaceForHandle<Identity: ICompositorInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, swapchain: super::super::super::Foundation::HANDLE, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICompositorInterop_Impl::CreateCompositionSurfaceForHandle(this, core::mem::transmute_copy(&swapchain)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCompositionSurfaceForSwapChain<Identity: ICompositorInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, swapchain: *mut core::ffi::c_void, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICompositorInterop_Impl::CreateCompositionSurfaceForSwapChain(this, core::mem::transmute_copy(&swapchain)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGraphicsDevice<Identity: ICompositorInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingdevice: *mut core::ffi::c_void, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICompositorInterop_Impl::CreateGraphicsDevice(this, core::mem::transmute_copy(&renderingdevice)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateCompositionSurfaceForHandle: CreateCompositionSurfaceForHandle::<Identity, OFFSET>,
            CreateCompositionSurfaceForSwapChain: CreateCompositionSurfaceForSwapChain::<Identity, OFFSET>,
            CreateGraphicsDevice: CreateGraphicsDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositorInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for ICompositorInterop {}
windows_core::imp::define_interface!(ICompositorInterop2, ICompositorInterop2_Vtbl, 0xd3eef34c_0667_4afc_8d13_867607b0fe91);
windows_core::imp::interface_hierarchy!(ICompositorInterop2, windows_core::IUnknown);
impl ICompositorInterop2 {
    pub unsafe fn CheckCompositionTextureSupport<P0>(&self, renderingdevice: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckCompositionTextureSupport)(windows_core::Interface::as_raw(self), renderingdevice.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateCompositionTexture<P0>(&self, d3dtexture: P0) -> windows_core::Result<super::super::super::super::UI::Composition::CompositionTexture>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCompositionTexture)(windows_core::Interface::as_raw(self), d3dtexture.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositorInterop2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CheckCompositionTextureSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub CreateCompositionTexture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateCompositionTexture: usize,
}
#[cfg(feature = "UI_Composition")]
pub trait ICompositorInterop2_Impl: windows_core::IUnknownImpl {
    fn CheckCompositionTextureSupport(&self, renderingdevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<windows_core::BOOL>;
    fn CreateCompositionTexture(&self, d3dtexture: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<super::super::super::super::UI::Composition::CompositionTexture>;
}
#[cfg(feature = "UI_Composition")]
impl ICompositorInterop2_Vtbl {
    pub const fn new<Identity: ICompositorInterop2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckCompositionTextureSupport<Identity: ICompositorInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingdevice: *mut core::ffi::c_void, supportscompositiontextures: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICompositorInterop2_Impl::CheckCompositionTextureSupport(this, core::mem::transmute_copy(&renderingdevice)) {
                    Ok(ok__) => {
                        supportscompositiontextures.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCompositionTexture<Identity: ICompositorInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, d3dtexture: *mut core::ffi::c_void, compositiontexture: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICompositorInterop2_Impl::CreateCompositionTexture(this, core::mem::transmute_copy(&d3dtexture)) {
                    Ok(ok__) => {
                        compositiontexture.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CheckCompositionTextureSupport: CheckCompositionTextureSupport::<Identity, OFFSET>,
            CreateCompositionTexture: CreateCompositionTexture::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositorInterop2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for ICompositorInterop2 {}
windows_core::imp::define_interface!(IDesktopWindowTargetInterop, IDesktopWindowTargetInterop_Vtbl, 0x35dbf59e_e3f9_45b0_81e7_fe75f4145dc9);
windows_core::imp::interface_hierarchy!(IDesktopWindowTargetInterop, windows_core::IUnknown);
impl IDesktopWindowTargetInterop {
    pub unsafe fn Hwnd(&self) -> windows_core::Result<super::super::super::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Hwnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowTargetInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Hwnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT,
}
pub trait IDesktopWindowTargetInterop_Impl: windows_core::IUnknownImpl {
    fn Hwnd(&self) -> windows_core::Result<super::super::super::Foundation::HWND>;
}
impl IDesktopWindowTargetInterop_Vtbl {
    pub const fn new<Identity: IDesktopWindowTargetInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Hwnd<Identity: IDesktopWindowTargetInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDesktopWindowTargetInterop_Impl::Hwnd(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Hwnd: Hwnd::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDesktopWindowTargetInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDesktopWindowTargetInterop {}
windows_core::imp::define_interface!(IVisualInteractionSourceInterop, IVisualInteractionSourceInterop_Vtbl, 0x11f62cd1_2f9d_42d3_b05f_d6790d9e9f8e);
windows_core::imp::interface_hierarchy!(IVisualInteractionSourceInterop, windows_core::IUnknown);
impl IVisualInteractionSourceInterop {
    #[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TryRedirectForManipulation(&self, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TryRedirectForManipulation)(windows_core::Interface::as_raw(self), pointerinfo).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TryRedirectForManipulation: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging")))]
    TryRedirectForManipulation: usize,
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IVisualInteractionSourceInterop_Impl: windows_core::IUnknownImpl {
    fn TryRedirectForManipulation(&self, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl IVisualInteractionSourceInterop_Vtbl {
    pub const fn new<Identity: IVisualInteractionSourceInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TryRedirectForManipulation<Identity: IVisualInteractionSourceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualInteractionSourceInterop_Impl::TryRedirectForManipulation(this, core::mem::transmute_copy(&pointerinfo)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TryRedirectForManipulation: TryRedirectForManipulation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualInteractionSourceInterop as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IVisualInteractionSourceInterop {}
