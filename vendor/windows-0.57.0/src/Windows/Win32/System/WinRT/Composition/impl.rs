#[cfg(feature = "UI_Composition")]
pub trait ICompositionCapabilitiesInteropFactory_Impl: Sized {
    fn GetForWindow(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<super::super::super::super::UI::Composition::CompositionCapabilities>;
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for ICompositionCapabilitiesInteropFactory {}
#[cfg(feature = "UI_Composition")]
impl ICompositionCapabilitiesInteropFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionCapabilitiesInteropFactory_Impl, const OFFSET: isize>() -> ICompositionCapabilitiesInteropFactory_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionCapabilitiesInteropFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICompositionCapabilitiesInteropFactory_Impl::GetForWindow(this, core::mem::transmute_copy(&hwnd)) {
                Ok(ok__) => {
                    core::ptr::write(result, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICompositionCapabilitiesInteropFactory, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionCapabilitiesInteropFactory as windows_core::Interface>::IID
    }
}
pub trait ICompositionDrawingSurfaceInterop_Impl: Sized {
    fn BeginDraw(&self, updaterect: *const super::super::super::Foundation::RECT, iid: *const windows_core::GUID, updateobject: *mut *mut core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> windows_core::Result<()>;
    fn EndDraw(&self) -> windows_core::Result<()>;
    fn Resize(&self, sizepixels: &super::super::super::Foundation::SIZE) -> windows_core::Result<()>;
    fn Scroll(&self, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> windows_core::Result<()>;
    fn ResumeDraw(&self) -> windows_core::Result<()>;
    fn SuspendDraw(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICompositionDrawingSurfaceInterop {}
impl ICompositionDrawingSurfaceInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>() -> ICompositionDrawingSurfaceInterop_Vtbl {
        unsafe extern "system" fn BeginDraw<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updaterect: *const super::super::super::Foundation::RECT, iid: *const windows_core::GUID, updateobject: *mut *mut core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICompositionDrawingSurfaceInterop_Impl::BeginDraw(this, core::mem::transmute_copy(&updaterect), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&updateobject), core::mem::transmute_copy(&updateoffset)).into()
        }
        unsafe extern "system" fn EndDraw<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICompositionDrawingSurfaceInterop_Impl::EndDraw(this).into()
        }
        unsafe extern "system" fn Resize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sizepixels: super::super::super::Foundation::SIZE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICompositionDrawingSurfaceInterop_Impl::Resize(this, core::mem::transmute(&sizepixels)).into()
        }
        unsafe extern "system" fn Scroll<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICompositionDrawingSurfaceInterop_Impl::Scroll(this, core::mem::transmute_copy(&scrollrect), core::mem::transmute_copy(&cliprect), core::mem::transmute_copy(&offsetx), core::mem::transmute_copy(&offsety)).into()
        }
        unsafe extern "system" fn ResumeDraw<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICompositionDrawingSurfaceInterop_Impl::ResumeDraw(this).into()
        }
        unsafe extern "system" fn SuspendDraw<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICompositionDrawingSurfaceInterop_Impl::SuspendDraw(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginDraw: BeginDraw::<Identity, Impl, OFFSET>,
            EndDraw: EndDraw::<Identity, Impl, OFFSET>,
            Resize: Resize::<Identity, Impl, OFFSET>,
            Scroll: Scroll::<Identity, Impl, OFFSET>,
            ResumeDraw: ResumeDraw::<Identity, Impl, OFFSET>,
            SuspendDraw: SuspendDraw::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionDrawingSurfaceInterop as windows_core::Interface>::IID
    }
}
pub trait ICompositionDrawingSurfaceInterop2_Impl: Sized + ICompositionDrawingSurfaceInterop_Impl {
    fn CopySurface(&self, destinationresource: Option<&windows_core::IUnknown>, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICompositionDrawingSurfaceInterop2 {}
impl ICompositionDrawingSurfaceInterop2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop2_Impl, const OFFSET: isize>() -> ICompositionDrawingSurfaceInterop2_Vtbl {
        unsafe extern "system" fn CopySurface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationresource: *mut core::ffi::c_void, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICompositionDrawingSurfaceInterop2_Impl::CopySurface(this, windows_core::from_raw_borrowed(&destinationresource), core::mem::transmute_copy(&destinationoffsetx), core::mem::transmute_copy(&destinationoffsety), core::mem::transmute_copy(&sourcerectangle)).into()
        }
        Self { base__: ICompositionDrawingSurfaceInterop_Vtbl::new::<Identity, Impl, OFFSET>(), CopySurface: CopySurface::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionDrawingSurfaceInterop2 as windows_core::Interface>::IID || iid == &<ICompositionDrawingSurfaceInterop as windows_core::Interface>::IID
    }
}
pub trait ICompositionGraphicsDeviceInterop_Impl: Sized {
    fn GetRenderingDevice(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SetRenderingDevice(&self, value: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICompositionGraphicsDeviceInterop {}
impl ICompositionGraphicsDeviceInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionGraphicsDeviceInterop_Impl, const OFFSET: isize>() -> ICompositionGraphicsDeviceInterop_Vtbl {
        unsafe extern "system" fn GetRenderingDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionGraphicsDeviceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICompositionGraphicsDeviceInterop_Impl::GetRenderingDevice(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderingDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionGraphicsDeviceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICompositionGraphicsDeviceInterop_Impl::SetRenderingDevice(this, windows_core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRenderingDevice: GetRenderingDevice::<Identity, Impl, OFFSET>,
            SetRenderingDevice: SetRenderingDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionGraphicsDeviceInterop as windows_core::Interface>::IID
    }
}
pub trait ICompositionTextureInterop_Impl: Sized {
    fn GetAvailableFence(&self, fencevalue: *mut u64, iid: *const windows_core::GUID, availablefence: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICompositionTextureInterop {}
impl ICompositionTextureInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionTextureInterop_Impl, const OFFSET: isize>() -> ICompositionTextureInterop_Vtbl {
        unsafe extern "system" fn GetAvailableFence<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionTextureInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fencevalue: *mut u64, iid: *const windows_core::GUID, availablefence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICompositionTextureInterop_Impl::GetAvailableFence(this, core::mem::transmute_copy(&fencevalue), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&availablefence)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetAvailableFence: GetAvailableFence::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionTextureInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "UI_Composition_Desktop")]
pub trait ICompositorDesktopInterop_Impl: Sized {
    fn CreateDesktopWindowTarget(&self, hwndtarget: super::super::super::Foundation::HWND, istopmost: super::super::super::Foundation::BOOL) -> windows_core::Result<super::super::super::super::UI::Composition::Desktop::DesktopWindowTarget>;
    fn EnsureOnThread(&self, threadid: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "UI_Composition_Desktop")]
impl windows_core::RuntimeName for ICompositorDesktopInterop {}
#[cfg(feature = "UI_Composition_Desktop")]
impl ICompositorDesktopInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositorDesktopInterop_Impl, const OFFSET: isize>() -> ICompositorDesktopInterop_Vtbl {
        unsafe extern "system" fn CreateDesktopWindowTarget<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositorDesktopInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndtarget: super::super::super::Foundation::HWND, istopmost: super::super::super::Foundation::BOOL, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICompositorDesktopInterop_Impl::CreateDesktopWindowTarget(this, core::mem::transmute_copy(&hwndtarget), core::mem::transmute_copy(&istopmost)) {
                Ok(ok__) => {
                    core::ptr::write(result, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnsureOnThread<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositorDesktopInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICompositorDesktopInterop_Impl::EnsureOnThread(this, core::mem::transmute_copy(&threadid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDesktopWindowTarget: CreateDesktopWindowTarget::<Identity, Impl, OFFSET>,
            EnsureOnThread: EnsureOnThread::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositorDesktopInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "UI_Composition")]
pub trait ICompositorInterop_Impl: Sized {
    fn CreateCompositionSurfaceForHandle(&self, swapchain: super::super::super::Foundation::HANDLE) -> windows_core::Result<super::super::super::super::UI::Composition::ICompositionSurface>;
    fn CreateCompositionSurfaceForSwapChain(&self, swapchain: Option<&windows_core::IUnknown>) -> windows_core::Result<super::super::super::super::UI::Composition::ICompositionSurface>;
    fn CreateGraphicsDevice(&self, renderingdevice: Option<&windows_core::IUnknown>) -> windows_core::Result<super::super::super::super::UI::Composition::CompositionGraphicsDevice>;
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for ICompositorInterop {}
#[cfg(feature = "UI_Composition")]
impl ICompositorInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositorInterop_Impl, const OFFSET: isize>() -> ICompositorInterop_Vtbl {
        unsafe extern "system" fn CreateCompositionSurfaceForHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositorInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, swapchain: super::super::super::Foundation::HANDLE, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICompositorInterop_Impl::CreateCompositionSurfaceForHandle(this, core::mem::transmute_copy(&swapchain)) {
                Ok(ok__) => {
                    core::ptr::write(result, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompositionSurfaceForSwapChain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositorInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, swapchain: *mut core::ffi::c_void, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICompositorInterop_Impl::CreateCompositionSurfaceForSwapChain(this, windows_core::from_raw_borrowed(&swapchain)) {
                Ok(ok__) => {
                    core::ptr::write(result, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGraphicsDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositorInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingdevice: *mut core::ffi::c_void, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICompositorInterop_Impl::CreateGraphicsDevice(this, windows_core::from_raw_borrowed(&renderingdevice)) {
                Ok(ok__) => {
                    core::ptr::write(result, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateCompositionSurfaceForHandle: CreateCompositionSurfaceForHandle::<Identity, Impl, OFFSET>,
            CreateCompositionSurfaceForSwapChain: CreateCompositionSurfaceForSwapChain::<Identity, Impl, OFFSET>,
            CreateGraphicsDevice: CreateGraphicsDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositorInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "UI_Composition")]
pub trait ICompositorInterop2_Impl: Sized {
    fn CheckCompositionTextureSupport(&self, renderingdevice: Option<&windows_core::IUnknown>) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn CreateCompositionTexture(&self, d3dtexture: Option<&windows_core::IUnknown>) -> windows_core::Result<super::super::super::super::UI::Composition::ICompositionTexture>;
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for ICompositorInterop2 {}
#[cfg(feature = "UI_Composition")]
impl ICompositorInterop2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositorInterop2_Impl, const OFFSET: isize>() -> ICompositorInterop2_Vtbl {
        unsafe extern "system" fn CheckCompositionTextureSupport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositorInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingdevice: *mut core::ffi::c_void, supportscompositiontextures: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICompositorInterop2_Impl::CheckCompositionTextureSupport(this, windows_core::from_raw_borrowed(&renderingdevice)) {
                Ok(ok__) => {
                    core::ptr::write(supportscompositiontextures, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompositionTexture<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositorInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, d3dtexture: *mut core::ffi::c_void, compositiontexture: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICompositorInterop2_Impl::CreateCompositionTexture(this, windows_core::from_raw_borrowed(&d3dtexture)) {
                Ok(ok__) => {
                    core::ptr::write(compositiontexture, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CheckCompositionTextureSupport: CheckCompositionTextureSupport::<Identity, Impl, OFFSET>,
            CreateCompositionTexture: CreateCompositionTexture::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositorInterop2 as windows_core::Interface>::IID
    }
}
pub trait IDesktopWindowTargetInterop_Impl: Sized {
    fn Hwnd(&self) -> windows_core::Result<super::super::super::Foundation::HWND>;
}
impl windows_core::RuntimeName for IDesktopWindowTargetInterop {}
impl IDesktopWindowTargetInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDesktopWindowTargetInterop_Impl, const OFFSET: isize>() -> IDesktopWindowTargetInterop_Vtbl {
        unsafe extern "system" fn Hwnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDesktopWindowTargetInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDesktopWindowTargetInterop_Impl::Hwnd(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Hwnd: Hwnd::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDesktopWindowTargetInterop as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IVisualInteractionSourceInterop_Impl: Sized {
    fn TryRedirectForManipulation(&self, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IVisualInteractionSourceInterop {}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl IVisualInteractionSourceInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVisualInteractionSourceInterop_Impl, const OFFSET: isize>() -> IVisualInteractionSourceInterop_Vtbl {
        unsafe extern "system" fn TryRedirectForManipulation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVisualInteractionSourceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVisualInteractionSourceInterop_Impl::TryRedirectForManipulation(this, core::mem::transmute_copy(&pointerinfo)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TryRedirectForManipulation: TryRedirectForManipulation::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualInteractionSourceInterop as windows_core::Interface>::IID
    }
}
