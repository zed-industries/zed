windows_core::imp::define_interface!(ICommunicationBlockingAccessManagerStatics, ICommunicationBlockingAccessManagerStatics_Vtbl, 0x1c969998_9d2a_5db7_edd5_0ce407fc2595);
impl windows_core::RuntimeType for ICommunicationBlockingAccessManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICommunicationBlockingAccessManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsBlockingActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsBlockedNumberAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ShowBlockNumbersUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShowBlockNumbersUI: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ShowUnblockNumbersUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShowUnblockNumbersUI: usize,
    pub ShowBlockedCallsUI: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowBlockedMessagesUI: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICommunicationBlockingAppManagerStatics, ICommunicationBlockingAppManagerStatics_Vtbl, 0x77db58ec_14a6_4baa_942a_6a673d999bf2);
impl windows_core::RuntimeType for ICommunicationBlockingAppManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICommunicationBlockingAppManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsCurrentAppActiveBlockingApp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ShowCommunicationBlockingSettingsUI: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICommunicationBlockingAppManagerStatics2, ICommunicationBlockingAppManagerStatics2_Vtbl, 0x14a68edd_ed88_457a_a364_a3634d6f166d);
impl windows_core::RuntimeType for ICommunicationBlockingAppManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICommunicationBlockingAppManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestSetAsActiveBlockingAppAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub struct CommunicationBlockingAccessManager;
impl CommunicationBlockingAccessManager {
    pub fn IsBlockingActive() -> windows_core::Result<bool> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBlockingActive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IsBlockedNumberAsync(number: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBlockedNumberAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(number), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShowBlockNumbersUI<P0>(phonenumbers: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowBlockNumbersUI)(windows_core::Interface::as_raw(this), phonenumbers.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShowUnblockNumbersUI<P0>(phonenumbers: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowUnblockNumbersUI)(windows_core::Interface::as_raw(this), phonenumbers.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn ShowBlockedCallsUI() -> windows_core::Result<()> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).ShowBlockedCallsUI)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn ShowBlockedMessagesUI() -> windows_core::Result<()> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).ShowBlockedMessagesUI)(windows_core::Interface::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    pub fn ICommunicationBlockingAccessManagerStatics<R, F: FnOnce(&ICommunicationBlockingAccessManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CommunicationBlockingAccessManager, ICommunicationBlockingAccessManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for CommunicationBlockingAccessManager {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAccessManager";
}
pub struct CommunicationBlockingAppManager;
impl CommunicationBlockingAppManager {
    pub fn IsCurrentAppActiveBlockingApp() -> windows_core::Result<bool> {
        Self::ICommunicationBlockingAppManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCurrentAppActiveBlockingApp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ShowCommunicationBlockingSettingsUI() -> windows_core::Result<()> {
        Self::ICommunicationBlockingAppManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).ShowCommunicationBlockingSettingsUI)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn RequestSetAsActiveBlockingAppAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ICommunicationBlockingAppManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestSetAsActiveBlockingAppAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICommunicationBlockingAppManagerStatics<R, F: FnOnce(&ICommunicationBlockingAppManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CommunicationBlockingAppManager, ICommunicationBlockingAppManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICommunicationBlockingAppManagerStatics2<R, F: FnOnce(&ICommunicationBlockingAppManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CommunicationBlockingAppManager, ICommunicationBlockingAppManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for CommunicationBlockingAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAppManager";
}
