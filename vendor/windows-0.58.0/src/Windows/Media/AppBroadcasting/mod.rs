windows_core::imp::define_interface!(IAppBroadcastingMonitor, IAppBroadcastingMonitor_Vtbl, 0x00f95a68_8907_48a0_b8ef_24d208137542);
impl windows_core::RuntimeType for IAppBroadcastingMonitor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastingMonitor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsCurrentAppBroadcasting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCurrentAppBroadcastingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveIsCurrentAppBroadcastingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastingStatus, IAppBroadcastingStatus_Vtbl, 0x1225e4df_03a1_42f8_8b80_c9228cd9cf2e);
impl windows_core::RuntimeType for IAppBroadcastingStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastingStatus_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CanStartBroadcast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastingStatusDetails, IAppBroadcastingStatusDetails_Vtbl, 0x069dada4_b573_4e3c_8e19_1bafacd09713);
impl windows_core::RuntimeType for IAppBroadcastingStatusDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastingStatusDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsAnyAppBroadcasting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCaptureResourceUnavailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsGameStreamInProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsGpuConstrained: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsAppInactive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsBlockedForApp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDisabledByUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDisabledBySystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastingUI, IAppBroadcastingUI_Vtbl, 0xe56f9f8f_ee99_4dca_a3c3_70af3db44f5f);
impl windows_core::RuntimeType for IAppBroadcastingUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastingUI_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowBroadcastUI: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastingUIStatics, IAppBroadcastingUIStatics_Vtbl, 0x55a8a79d_23cb_4579_9c34_886fe02c045a);
impl windows_core::RuntimeType for IAppBroadcastingUIStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastingUIStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastingMonitor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastingMonitor, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastingMonitor {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppBroadcastingMonitor, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsCurrentAppBroadcasting(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCurrentAppBroadcasting)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCurrentAppBroadcastingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastingMonitor, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCurrentAppBroadcastingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveIsCurrentAppBroadcastingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveIsCurrentAppBroadcastingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for AppBroadcastingMonitor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastingMonitor>();
}
unsafe impl windows_core::Interface for AppBroadcastingMonitor {
    type Vtable = IAppBroadcastingMonitor_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastingMonitor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastingMonitor {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingMonitor";
}
unsafe impl Send for AppBroadcastingMonitor {}
unsafe impl Sync for AppBroadcastingMonitor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastingStatus(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastingStatus, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastingStatus {
    pub fn CanStartBroadcast(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanStartBroadcast)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Details(&self) -> windows_core::Result<AppBroadcastingStatusDetails> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Details)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastingStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastingStatus>();
}
unsafe impl windows_core::Interface for AppBroadcastingStatus {
    type Vtable = IAppBroadcastingStatus_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastingStatus as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastingStatus {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingStatus";
}
unsafe impl Send for AppBroadcastingStatus {}
unsafe impl Sync for AppBroadcastingStatus {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastingStatusDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastingStatusDetails, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastingStatusDetails {
    pub fn IsAnyAppBroadcasting(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAnyAppBroadcasting)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCaptureResourceUnavailable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCaptureResourceUnavailable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsGameStreamInProgress(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGameStreamInProgress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsGpuConstrained(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGpuConstrained)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsAppInactive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAppInactive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsBlockedForApp(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBlockedForApp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDisabledByUser(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDisabledByUser)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDisabledBySystem(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDisabledBySystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastingStatusDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastingStatusDetails>();
}
unsafe impl windows_core::Interface for AppBroadcastingStatusDetails {
    type Vtable = IAppBroadcastingStatusDetails_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastingStatusDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastingStatusDetails {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingStatusDetails";
}
unsafe impl Send for AppBroadcastingStatusDetails {}
unsafe impl Sync for AppBroadcastingStatusDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastingUI(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastingUI, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastingUI {
    pub fn GetStatus(&self) -> windows_core::Result<AppBroadcastingStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatus)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShowBroadcastUI(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ShowBroadcastUI)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetDefault() -> windows_core::Result<AppBroadcastingUI> {
        Self::IAppBroadcastingUIStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<AppBroadcastingUI>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IAppBroadcastingUIStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAppBroadcastingUIStatics<R, F: FnOnce(&IAppBroadcastingUIStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppBroadcastingUI, IAppBroadcastingUIStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppBroadcastingUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastingUI>();
}
unsafe impl windows_core::Interface for AppBroadcastingUI {
    type Vtable = IAppBroadcastingUI_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastingUI as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastingUI {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingUI";
}
unsafe impl Send for AppBroadcastingUI {}
unsafe impl Sync for AppBroadcastingUI {}
