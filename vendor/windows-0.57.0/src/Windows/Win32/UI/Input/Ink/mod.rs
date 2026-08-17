windows_core::imp::define_interface!(IInkCommitRequestHandler, IInkCommitRequestHandler_Vtbl, 0xfabea3fc_b108_45b6_a9fc_8d08fa9f85cf);
impl core::ops::Deref for IInkCommitRequestHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInkCommitRequestHandler, windows_core::IUnknown);
impl IInkCommitRequestHandler {
    pub unsafe fn OnCommitRequested(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCommitRequested)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IInkCommitRequestHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCommitRequested: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkD2DRenderer, IInkD2DRenderer_Vtbl, 0x407fb1de_f85a_4150_97cf_b7fb274fb4f8);
impl core::ops::Deref for IInkD2DRenderer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInkD2DRenderer, windows_core::IUnknown);
impl IInkD2DRenderer {
    pub unsafe fn Draw<P0, P1, P2>(&self, pd2d1devicecontext: P0, pinkstrokeiterable: P1, fhighcontrast: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), pd2d1devicecontext.param().abi(), pinkstrokeiterable.param().abi(), fhighcontrast.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IInkD2DRenderer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkD2DRenderer2, IInkD2DRenderer2_Vtbl, 0x0a95dcd9_4578_4b71_b20b_bf664d4bfeee);
impl core::ops::Deref for IInkD2DRenderer2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInkD2DRenderer2, windows_core::IUnknown);
impl IInkD2DRenderer2 {
    pub unsafe fn Draw<P0, P1>(&self, pd2d1devicecontext: P0, pinkstrokeiterable: P1, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), pd2d1devicecontext.param().abi(), pinkstrokeiterable.param().abi(), highcontrastadjustment).ok()
    }
}
#[repr(C)]
pub struct IInkD2DRenderer2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, INK_HIGH_CONTRAST_ADJUSTMENT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkDesktopHost, IInkDesktopHost_Vtbl, 0x4ce7d875_a981_4140_a1ff_ad93258e8d59);
impl core::ops::Deref for IInkDesktopHost {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInkDesktopHost, windows_core::IUnknown);
impl IInkDesktopHost {
    pub unsafe fn QueueWorkItem<P0>(&self, workitem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IInkHostWorkItem>,
    {
        (windows_core::Interface::vtable(self).QueueWorkItem)(windows_core::Interface::as_raw(self), workitem.param().abi()).ok()
    }
    pub unsafe fn CreateInkPresenter<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateInkPresenter)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateAndInitializeInkPresenter<P0, T>(&self, rootvisual: P0, width: f32, height: f32) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateAndInitializeInkPresenter)(windows_core::Interface::as_raw(self), rootvisual.param().abi(), width, height, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IInkDesktopHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueueWorkItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInkPresenter: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAndInitializeInkPresenter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32, f32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkHostWorkItem, IInkHostWorkItem_Vtbl, 0xccda0a9a_1b78_4632_bb96_97800662e26c);
impl core::ops::Deref for IInkHostWorkItem {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInkHostWorkItem, windows_core::IUnknown);
impl IInkHostWorkItem {
    pub unsafe fn Invoke(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IInkHostWorkItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkPresenterDesktop, IInkPresenterDesktop_Vtbl, 0x73f3c0d9_2e8b_48f3_895e_20cbd27b723b);
impl core::ops::Deref for IInkPresenterDesktop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInkPresenterDesktop, windows_core::IUnknown);
impl IInkPresenterDesktop {
    pub unsafe fn SetRootVisual<P0, P1>(&self, rootvisual: P0, device: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetRootVisual)(windows_core::Interface::as_raw(self), rootvisual.param().abi(), device.param().abi()).ok()
    }
    pub unsafe fn SetCommitRequestHandler<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IInkCommitRequestHandler>,
    {
        (windows_core::Interface::vtable(self).SetCommitRequestHandler)(windows_core::Interface::as_raw(self), handler.param().abi()).ok()
    }
    pub unsafe fn GetSize(&self, width: *mut f32, height: *mut f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), width, height).ok()
    }
    pub unsafe fn SetSize(&self, width: f32, height: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), width, height).ok()
    }
    pub unsafe fn OnHighContrastChanged(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnHighContrastChanged)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IInkPresenterDesktop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetRootVisual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCommitRequestHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32) -> windows_core::HRESULT,
    pub OnHighContrastChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const USE_ORIGINAL_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(2i32);
pub const USE_SYSTEM_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(1i32);
pub const USE_SYSTEM_COLORS_WHEN_NECESSARY: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct INK_HIGH_CONTRAST_ADJUSTMENT(pub i32);
impl windows_core::TypeKind for INK_HIGH_CONTRAST_ADJUSTMENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for INK_HIGH_CONTRAST_ADJUSTMENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("INK_HIGH_CONTRAST_ADJUSTMENT").field(&self.0).finish()
    }
}
pub const InkD2DRenderer: windows_core::GUID = windows_core::GUID::from_u128(0x4044e60c_7b01_4671_a97c_04e0210a07a5);
pub const InkDesktopHost: windows_core::GUID = windows_core::GUID::from_u128(0x062584a6_f830_4bdc_a4d2_0a10ab062b1d);
#[cfg(feature = "implement")]
core::include!("impl.rs");
