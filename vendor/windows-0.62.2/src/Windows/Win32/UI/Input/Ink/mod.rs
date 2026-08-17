windows_core::imp::define_interface!(IInkCommitRequestHandler, IInkCommitRequestHandler_Vtbl, 0xfabea3fc_b108_45b6_a9fc_8d08fa9f85cf);
windows_core::imp::interface_hierarchy!(IInkCommitRequestHandler, windows_core::IUnknown);
impl IInkCommitRequestHandler {
    pub unsafe fn OnCommitRequested(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCommitRequested)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkCommitRequestHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCommitRequested: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInkCommitRequestHandler_Impl: windows_core::IUnknownImpl {
    fn OnCommitRequested(&self) -> windows_core::Result<()>;
}
impl IInkCommitRequestHandler_Vtbl {
    pub const fn new<Identity: IInkCommitRequestHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnCommitRequested<Identity: IInkCommitRequestHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInkCommitRequestHandler_Impl::OnCommitRequested(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCommitRequested: OnCommitRequested::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkCommitRequestHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInkCommitRequestHandler {}
windows_core::imp::define_interface!(IInkD2DRenderer, IInkD2DRenderer_Vtbl, 0x407fb1de_f85a_4150_97cf_b7fb274fb4f8);
windows_core::imp::interface_hierarchy!(IInkD2DRenderer, windows_core::IUnknown);
impl IInkD2DRenderer {
    pub unsafe fn Draw<P0, P1>(&self, pd2d1devicecontext: P0, pinkstrokeiterable: P1, fhighcontrast: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), pd2d1devicecontext.param().abi(), pinkstrokeiterable.param().abi(), fhighcontrast.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkD2DRenderer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IInkD2DRenderer_Impl: windows_core::IUnknownImpl {
    fn Draw(&self, pd2d1devicecontext: windows_core::Ref<windows_core::IUnknown>, pinkstrokeiterable: windows_core::Ref<windows_core::IUnknown>, fhighcontrast: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IInkD2DRenderer_Vtbl {
    pub const fn new<Identity: IInkD2DRenderer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Draw<Identity: IInkD2DRenderer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pd2d1devicecontext: *mut core::ffi::c_void, pinkstrokeiterable: *mut core::ffi::c_void, fhighcontrast: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInkD2DRenderer_Impl::Draw(this, core::mem::transmute_copy(&pd2d1devicecontext), core::mem::transmute_copy(&pinkstrokeiterable), core::mem::transmute_copy(&fhighcontrast)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Draw: Draw::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkD2DRenderer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInkD2DRenderer {}
windows_core::imp::define_interface!(IInkD2DRenderer2, IInkD2DRenderer2_Vtbl, 0x0a95dcd9_4578_4b71_b20b_bf664d4bfeee);
windows_core::imp::interface_hierarchy!(IInkD2DRenderer2, windows_core::IUnknown);
impl IInkD2DRenderer2 {
    pub unsafe fn Draw<P0, P1>(&self, pd2d1devicecontext: P0, pinkstrokeiterable: P1, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), pd2d1devicecontext.param().abi(), pinkstrokeiterable.param().abi(), highcontrastadjustment).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkD2DRenderer2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, INK_HIGH_CONTRAST_ADJUSTMENT) -> windows_core::HRESULT,
}
pub trait IInkD2DRenderer2_Impl: windows_core::IUnknownImpl {
    fn Draw(&self, pd2d1devicecontext: windows_core::Ref<windows_core::IUnknown>, pinkstrokeiterable: windows_core::Ref<windows_core::IUnknown>, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> windows_core::Result<()>;
}
impl IInkD2DRenderer2_Vtbl {
    pub const fn new<Identity: IInkD2DRenderer2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Draw<Identity: IInkD2DRenderer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pd2d1devicecontext: *mut core::ffi::c_void, pinkstrokeiterable: *mut core::ffi::c_void, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInkD2DRenderer2_Impl::Draw(this, core::mem::transmute_copy(&pd2d1devicecontext), core::mem::transmute_copy(&pinkstrokeiterable), core::mem::transmute_copy(&highcontrastadjustment)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Draw: Draw::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkD2DRenderer2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInkD2DRenderer2 {}
windows_core::imp::define_interface!(IInkDesktopHost, IInkDesktopHost_Vtbl, 0x4ce7d875_a981_4140_a1ff_ad93258e8d59);
windows_core::imp::interface_hierarchy!(IInkDesktopHost, windows_core::IUnknown);
impl IInkDesktopHost {
    pub unsafe fn QueueWorkItem<P0>(&self, workitem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IInkHostWorkItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueueWorkItem)(windows_core::Interface::as_raw(self), workitem.param().abi()).ok() }
    }
    pub unsafe fn CreateInkPresenter<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateInkPresenter)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn CreateAndInitializeInkPresenter<P0, T>(&self, rootvisual: P0, width: f32, height: f32) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateAndInitializeInkPresenter)(windows_core::Interface::as_raw(self), rootvisual.param().abi(), width, height, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDesktopHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueueWorkItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInkPresenter: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAndInitializeInkPresenter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32, f32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInkDesktopHost_Impl: windows_core::IUnknownImpl {
    fn QueueWorkItem(&self, workitem: windows_core::Ref<IInkHostWorkItem>) -> windows_core::Result<()>;
    fn CreateInkPresenter(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateAndInitializeInkPresenter(&self, rootvisual: windows_core::Ref<windows_core::IUnknown>, width: f32, height: f32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IInkDesktopHost_Vtbl {
    pub const fn new<Identity: IInkDesktopHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueueWorkItem<Identity: IInkDesktopHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, workitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInkDesktopHost_Impl::QueueWorkItem(this, core::mem::transmute_copy(&workitem)).into()
            }
        }
        unsafe extern "system" fn CreateInkPresenter<Identity: IInkDesktopHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInkDesktopHost_Impl::CreateInkPresenter(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn CreateAndInitializeInkPresenter<Identity: IInkDesktopHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rootvisual: *mut core::ffi::c_void, width: f32, height: f32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInkDesktopHost_Impl::CreateAndInitializeInkPresenter(this, core::mem::transmute_copy(&rootvisual), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueueWorkItem: QueueWorkItem::<Identity, OFFSET>,
            CreateInkPresenter: CreateInkPresenter::<Identity, OFFSET>,
            CreateAndInitializeInkPresenter: CreateAndInitializeInkPresenter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkDesktopHost as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInkDesktopHost {}
windows_core::imp::define_interface!(IInkHostWorkItem, IInkHostWorkItem_Vtbl, 0xccda0a9a_1b78_4632_bb96_97800662e26c);
windows_core::imp::interface_hierarchy!(IInkHostWorkItem, windows_core::IUnknown);
impl IInkHostWorkItem {
    pub unsafe fn Invoke(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkHostWorkItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInkHostWorkItem_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self) -> windows_core::Result<()>;
}
impl IInkHostWorkItem_Vtbl {
    pub const fn new<Identity: IInkHostWorkItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: IInkHostWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInkHostWorkItem_Impl::Invoke(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkHostWorkItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInkHostWorkItem {}
windows_core::imp::define_interface!(IInkPresenterDesktop, IInkPresenterDesktop_Vtbl, 0x73f3c0d9_2e8b_48f3_895e_20cbd27b723b);
windows_core::imp::interface_hierarchy!(IInkPresenterDesktop, windows_core::IUnknown);
impl IInkPresenterDesktop {
    pub unsafe fn SetRootVisual<P0, P1>(&self, rootvisual: P0, device: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRootVisual)(windows_core::Interface::as_raw(self), rootvisual.param().abi(), device.param().abi()).ok() }
    }
    pub unsafe fn SetCommitRequestHandler<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IInkCommitRequestHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCommitRequestHandler)(windows_core::Interface::as_raw(self), handler.param().abi()).ok() }
    }
    pub unsafe fn GetSize(&self, width: *mut f32, height: *mut f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), width as _, height as _).ok() }
    }
    pub unsafe fn SetSize(&self, width: f32, height: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), width, height).ok() }
    }
    pub unsafe fn OnHighContrastChanged(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnHighContrastChanged)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterDesktop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetRootVisual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCommitRequestHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32) -> windows_core::HRESULT,
    pub OnHighContrastChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInkPresenterDesktop_Impl: windows_core::IUnknownImpl {
    fn SetRootVisual(&self, rootvisual: windows_core::Ref<windows_core::IUnknown>, device: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetCommitRequestHandler(&self, handler: windows_core::Ref<IInkCommitRequestHandler>) -> windows_core::Result<()>;
    fn GetSize(&self, width: *mut f32, height: *mut f32) -> windows_core::Result<()>;
    fn SetSize(&self, width: f32, height: f32) -> windows_core::Result<()>;
    fn OnHighContrastChanged(&self) -> windows_core::Result<()>;
}
impl IInkPresenterDesktop_Vtbl {
    pub const fn new<Identity: IInkPresenterDesktop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRootVisual<Identity: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rootvisual: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInkPresenterDesktop_Impl::SetRootVisual(this, core::mem::transmute_copy(&rootvisual), core::mem::transmute_copy(&device)).into()
            }
        }
        unsafe extern "system" fn SetCommitRequestHandler<Identity: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInkPresenterDesktop_Impl::SetCommitRequestHandler(this, core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn GetSize<Identity: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: *mut f32, height: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInkPresenterDesktop_Impl::GetSize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn SetSize<Identity: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: f32, height: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInkPresenterDesktop_Impl::SetSize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn OnHighContrastChanged<Identity: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInkPresenterDesktop_Impl::OnHighContrastChanged(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetRootVisual: SetRootVisual::<Identity, OFFSET>,
            SetCommitRequestHandler: SetCommitRequestHandler::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            OnHighContrastChanged: OnHighContrastChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkPresenterDesktop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInkPresenterDesktop {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INK_HIGH_CONTRAST_ADJUSTMENT(pub i32);
pub const InkD2DRenderer: windows_core::GUID = windows_core::GUID::from_u128(0x4044e60c_7b01_4671_a97c_04e0210a07a5);
pub const InkDesktopHost: windows_core::GUID = windows_core::GUID::from_u128(0x062584a6_f830_4bdc_a4d2_0a10ab062b1d);
pub const USE_ORIGINAL_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(2i32);
pub const USE_SYSTEM_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(1i32);
pub const USE_SYSTEM_COLORS_WHEN_NECESSARY: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(0i32);
