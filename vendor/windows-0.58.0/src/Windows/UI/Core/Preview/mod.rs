windows_core::imp::define_interface!(ICoreAppWindowPreview, ICoreAppWindowPreview_Vtbl, 0xa4f6e665_365e_5fde_87a5_9543c3a15aa8);
impl windows_core::RuntimeType for ICoreAppWindowPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreAppWindowPreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ICoreAppWindowPreviewStatics, ICoreAppWindowPreviewStatics_Vtbl, 0x33ac21be_423b_5db6_8a8e_4dc87353b75b);
impl windows_core::RuntimeType for ICoreAppWindowPreviewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreAppWindowPreviewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_WindowManagement")]
    pub GetIdFromWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    GetIdFromWindow: usize,
}
windows_core::imp::define_interface!(ISystemNavigationCloseRequestedPreviewEventArgs, ISystemNavigationCloseRequestedPreviewEventArgs_Vtbl, 0x83d00de1_cbe5_4f31_8414_361da046518f);
impl windows_core::RuntimeType for ISystemNavigationCloseRequestedPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemNavigationCloseRequestedPreviewEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemNavigationManagerPreview, ISystemNavigationManagerPreview_Vtbl, 0xec5f0488_6425_4777_a536_cb5634427f0d);
impl windows_core::RuntimeType for ISystemNavigationManagerPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemNavigationManagerPreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CloseRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCloseRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemNavigationManagerPreviewStatics, ISystemNavigationManagerPreviewStatics_Vtbl, 0x0e971360_df74_4bce_84cb_bd1181ac0a71);
impl windows_core::RuntimeType for ISystemNavigationManagerPreviewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemNavigationManagerPreviewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreAppWindowPreview(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreAppWindowPreview, windows_core::IUnknown, windows_core::IInspectable);
impl CoreAppWindowPreview {
    #[cfg(feature = "UI_WindowManagement")]
    pub fn GetIdFromWindow<P0>(window: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::super::WindowManagement::AppWindow>,
    {
        Self::ICoreAppWindowPreviewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIdFromWindow)(windows_core::Interface::as_raw(this), window.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreAppWindowPreviewStatics<R, F: FnOnce(&ICoreAppWindowPreviewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreAppWindowPreview, ICoreAppWindowPreviewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CoreAppWindowPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreAppWindowPreview>();
}
unsafe impl windows_core::Interface for CoreAppWindowPreview {
    type Vtable = ICoreAppWindowPreview_Vtbl;
    const IID: windows_core::GUID = <ICoreAppWindowPreview as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreAppWindowPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.CoreAppWindowPreview";
}
unsafe impl Send for CoreAppWindowPreview {}
unsafe impl Sync for CoreAppWindowPreview {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SystemNavigationCloseRequestedPreviewEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemNavigationCloseRequestedPreviewEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SystemNavigationCloseRequestedPreviewEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SystemNavigationCloseRequestedPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemNavigationCloseRequestedPreviewEventArgs>();
}
unsafe impl windows_core::Interface for SystemNavigationCloseRequestedPreviewEventArgs {
    type Vtable = ISystemNavigationCloseRequestedPreviewEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISystemNavigationCloseRequestedPreviewEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemNavigationCloseRequestedPreviewEventArgs {
    const NAME: &'static str = "Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs";
}
unsafe impl Send for SystemNavigationCloseRequestedPreviewEventArgs {}
unsafe impl Sync for SystemNavigationCloseRequestedPreviewEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SystemNavigationManagerPreview(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemNavigationManagerPreview, windows_core::IUnknown, windows_core::IInspectable);
impl SystemNavigationManagerPreview {
    pub fn CloseRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::EventHandler<SystemNavigationCloseRequestedPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloseRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCloseRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCloseRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<SystemNavigationManagerPreview> {
        Self::ISystemNavigationManagerPreviewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISystemNavigationManagerPreviewStatics<R, F: FnOnce(&ISystemNavigationManagerPreviewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SystemNavigationManagerPreview, ISystemNavigationManagerPreviewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SystemNavigationManagerPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemNavigationManagerPreview>();
}
unsafe impl windows_core::Interface for SystemNavigationManagerPreview {
    type Vtable = ISystemNavigationManagerPreview_Vtbl;
    const IID: windows_core::GUID = <ISystemNavigationManagerPreview as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemNavigationManagerPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.SystemNavigationManagerPreview";
}
unsafe impl Send for SystemNavigationManagerPreview {}
unsafe impl Sync for SystemNavigationManagerPreview {}
