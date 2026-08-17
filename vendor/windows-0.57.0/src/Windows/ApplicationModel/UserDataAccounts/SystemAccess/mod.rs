windows_core::imp::define_interface!(IDeviceAccountConfiguration, IDeviceAccountConfiguration_Vtbl, 0xad0123a3_fbdc_4d1b_be43_5a27ea4a1b63);
impl windows_core::RuntimeType for IDeviceAccountConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceAccountConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AccountName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetAccountName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DeviceAccountTypeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDeviceAccountTypeId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ServerType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeviceAccountServerType) -> windows_core::HRESULT,
    pub SetServerType: unsafe extern "system" fn(*mut core::ffi::c_void, DeviceAccountServerType) -> windows_core::HRESULT,
    pub EmailAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetEmailAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDomain: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EmailSyncEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetEmailSyncEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ContactsSyncEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetContactsSyncEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CalendarSyncEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCalendarSyncEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IncomingServerAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetIncomingServerAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IncomingServerPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetIncomingServerPort: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IncomingServerRequiresSsl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIncomingServerRequiresSsl: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IncomingServerUsername: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetIncomingServerUsername: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub OutgoingServerAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetOutgoingServerAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub OutgoingServerPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetOutgoingServerPort: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub OutgoingServerRequiresSsl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetOutgoingServerRequiresSsl: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub OutgoingServerUsername: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetOutgoingServerUsername: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceAccountConfiguration2, IDeviceAccountConfiguration2_Vtbl, 0xf2b2e5a6_728d_4a4a_8945_2bf8580136de);
impl windows_core::RuntimeType for IDeviceAccountConfiguration2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceAccountConfiguration2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub IncomingServerCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    IncomingServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetIncomingServerCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetIncomingServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub OutgoingServerCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    OutgoingServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetOutgoingServerCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetOutgoingServerCredential: usize,
    pub OAuthRefreshToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetOAuthRefreshToken: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsExternallyManaged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsExternallyManaged: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AccountIconId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeviceAccountIconId) -> windows_core::HRESULT,
    pub SetAccountIconId: unsafe extern "system" fn(*mut core::ffi::c_void, DeviceAccountIconId) -> windows_core::HRESULT,
    pub AuthenticationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeviceAccountAuthenticationType) -> windows_core::HRESULT,
    pub SetAuthenticationType: unsafe extern "system" fn(*mut core::ffi::c_void, DeviceAccountAuthenticationType) -> windows_core::HRESULT,
    pub IsSsoAuthenticationSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SsoAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSsoAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AlwaysDownloadFullMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAlwaysDownloadFullMessage: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub DoesPolicyAllowMailSync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SyncScheduleKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeviceAccountSyncScheduleKind) -> windows_core::HRESULT,
    pub SetSyncScheduleKind: unsafe extern "system" fn(*mut core::ffi::c_void, DeviceAccountSyncScheduleKind) -> windows_core::HRESULT,
    pub MailAgeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeviceAccountMailAgeFilter) -> windows_core::HRESULT,
    pub SetMailAgeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DeviceAccountMailAgeFilter) -> windows_core::HRESULT,
    pub IsClientAuthenticationCertificateRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsClientAuthenticationCertificateRequired: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AutoSelectAuthenticationCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutoSelectAuthenticationCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AuthenticationCertificateId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetAuthenticationCertificateId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CardDavSyncScheduleKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeviceAccountSyncScheduleKind) -> windows_core::HRESULT,
    pub SetCardDavSyncScheduleKind: unsafe extern "system" fn(*mut core::ffi::c_void, DeviceAccountSyncScheduleKind) -> windows_core::HRESULT,
    pub CalDavSyncScheduleKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeviceAccountSyncScheduleKind) -> windows_core::HRESULT,
    pub SetCalDavSyncScheduleKind: unsafe extern "system" fn(*mut core::ffi::c_void, DeviceAccountSyncScheduleKind) -> windows_core::HRESULT,
    pub CardDavServerUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCardDavServerUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CardDavRequiresSsl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCardDavRequiresSsl: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CalDavServerUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCalDavServerUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CalDavRequiresSsl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCalDavRequiresSsl: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub WasModifiedByUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetWasModifiedByUser: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub WasIncomingServerCertificateHashConfirmed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetWasIncomingServerCertificateHashConfirmed: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IncomingServerCertificateHash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetIncomingServerCertificateHash: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsOutgoingServerAuthenticationRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsOutgoingServerAuthenticationRequired: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsOutgoingServerAuthenticationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsOutgoingServerAuthenticationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub WasOutgoingServerCertificateHashConfirmed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetWasOutgoingServerCertificateHashConfirmed: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub OutgoingServerCertificateHash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetOutgoingServerCertificateHash: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsSyncScheduleManagedBySystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsSyncScheduleManagedBySystem: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataAccountSystemAccessManagerStatics, IUserDataAccountSystemAccessManagerStatics_Vtbl, 0x9d6b11b9_cbe5_45f5_822b_c267b81dbdb6);
impl windows_core::RuntimeType for IUserDataAccountSystemAccessManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccountSystemAccessManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AddAndShowDeviceAccountsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddAndShowDeviceAccountsAsync: usize,
}
windows_core::imp::define_interface!(IUserDataAccountSystemAccessManagerStatics2, IUserDataAccountSystemAccessManagerStatics2_Vtbl, 0x943f854d_4b4e_439f_83d3_979b27c05ac7);
impl windows_core::RuntimeType for IUserDataAccountSystemAccessManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccountSystemAccessManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SuppressLocalAccountWithAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDeviceAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteDeviceAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceAccountConfigurationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DeviceAccountConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceAccountConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl DeviceAccountConfiguration {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DeviceAccountConfiguration, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AccountName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAccountName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAccountName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn DeviceAccountTypeId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceAccountTypeId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDeviceAccountTypeId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDeviceAccountTypeId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ServerType(&self) -> windows_core::Result<DeviceAccountServerType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetServerType(&self, value: DeviceAccountServerType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetServerType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EmailAddress(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetEmailAddress(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEmailAddress)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Domain(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Domain)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDomain(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDomain)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn EmailSyncEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailSyncEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEmailSyncEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEmailSyncEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ContactsSyncEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactsSyncEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetContactsSyncEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContactsSyncEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CalendarSyncEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CalendarSyncEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCalendarSyncEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCalendarSyncEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncomingServerAddress(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncomingServerAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIncomingServerAddress(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIncomingServerAddress)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IncomingServerPort(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncomingServerPort)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIncomingServerPort(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIncomingServerPort)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncomingServerRequiresSsl(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncomingServerRequiresSsl)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIncomingServerRequiresSsl(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIncomingServerRequiresSsl)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncomingServerUsername(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncomingServerUsername)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIncomingServerUsername(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIncomingServerUsername)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn OutgoingServerAddress(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingServerAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutgoingServerAddress(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingServerAddress)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn OutgoingServerPort(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingServerPort)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOutgoingServerPort(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingServerPort)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingServerRequiresSsl(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingServerRequiresSsl)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOutgoingServerRequiresSsl(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingServerRequiresSsl)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingServerUsername(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingServerUsername)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutgoingServerUsername(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingServerUsername)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn IncomingServerCredential(&self) -> windows_core::Result<super::super::super::Security::Credentials::PasswordCredential> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncomingServerCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetIncomingServerCredential<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIncomingServerCredential)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn OutgoingServerCredential(&self) -> windows_core::Result<super::super::super::Security::Credentials::PasswordCredential> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingServerCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetOutgoingServerCredential<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingServerCredential)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OAuthRefreshToken(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OAuthRefreshToken)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOAuthRefreshToken(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOAuthRefreshToken)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsExternallyManaged(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsExternallyManaged)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsExternallyManaged(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsExternallyManaged)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccountIconId(&self) -> windows_core::Result<DeviceAccountIconId> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountIconId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAccountIconId(&self, value: DeviceAccountIconId) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccountIconId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AuthenticationType(&self) -> windows_core::Result<DeviceAccountAuthenticationType> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AuthenticationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAuthenticationType(&self, value: DeviceAccountAuthenticationType) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAuthenticationType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsSsoAuthenticationSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSsoAuthenticationSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SsoAccountId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SsoAccountId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSsoAccountId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSsoAccountId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn AlwaysDownloadFullMessage(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlwaysDownloadFullMessage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAlwaysDownloadFullMessage(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAlwaysDownloadFullMessage)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DoesPolicyAllowMailSync(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DoesPolicyAllowMailSync)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SyncScheduleKind(&self) -> windows_core::Result<DeviceAccountSyncScheduleKind> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SyncScheduleKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSyncScheduleKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MailAgeFilter(&self) -> windows_core::Result<DeviceAccountMailAgeFilter> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MailAgeFilter)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMailAgeFilter(&self, value: DeviceAccountMailAgeFilter) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMailAgeFilter)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsClientAuthenticationCertificateRequired(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsClientAuthenticationCertificateRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsClientAuthenticationCertificateRequired(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsClientAuthenticationCertificateRequired)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutoSelectAuthenticationCertificate(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoSelectAuthenticationCertificate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoSelectAuthenticationCertificate(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAutoSelectAuthenticationCertificate)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AuthenticationCertificateId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AuthenticationCertificateId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAuthenticationCertificateId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAuthenticationCertificateId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CardDavSyncScheduleKind(&self) -> windows_core::Result<DeviceAccountSyncScheduleKind> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CardDavSyncScheduleKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCardDavSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCardDavSyncScheduleKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CalDavSyncScheduleKind(&self) -> windows_core::Result<DeviceAccountSyncScheduleKind> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CalDavSyncScheduleKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCalDavSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCalDavSyncScheduleKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CardDavServerUrl(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CardDavServerUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCardDavServerUrl<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCardDavServerUrl)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CardDavRequiresSsl(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CardDavRequiresSsl)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCardDavRequiresSsl(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCardDavRequiresSsl)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CalDavServerUrl(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CalDavServerUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCalDavServerUrl<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCalDavServerUrl)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CalDavRequiresSsl(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CalDavRequiresSsl)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCalDavRequiresSsl(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCalDavRequiresSsl)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WasModifiedByUser(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WasModifiedByUser)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWasModifiedByUser(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetWasModifiedByUser)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WasIncomingServerCertificateHashConfirmed(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WasIncomingServerCertificateHashConfirmed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWasIncomingServerCertificateHashConfirmed(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetWasIncomingServerCertificateHashConfirmed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncomingServerCertificateHash(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncomingServerCertificateHash)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIncomingServerCertificateHash(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIncomingServerCertificateHash)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsOutgoingServerAuthenticationRequired(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOutgoingServerAuthenticationRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsOutgoingServerAuthenticationRequired(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsOutgoingServerAuthenticationRequired)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsOutgoingServerAuthenticationEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOutgoingServerAuthenticationEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsOutgoingServerAuthenticationEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsOutgoingServerAuthenticationEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WasOutgoingServerCertificateHashConfirmed(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WasOutgoingServerCertificateHashConfirmed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWasOutgoingServerCertificateHashConfirmed(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetWasOutgoingServerCertificateHashConfirmed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingServerCertificateHash(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingServerCertificateHash)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutgoingServerCertificateHash(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingServerCertificateHash)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsSyncScheduleManagedBySystem(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSyncScheduleManagedBySystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsSyncScheduleManagedBySystem(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsSyncScheduleManagedBySystem)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for DeviceAccountConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceAccountConfiguration>();
}
unsafe impl windows_core::Interface for DeviceAccountConfiguration {
    type Vtable = IDeviceAccountConfiguration_Vtbl;
    const IID: windows_core::GUID = <IDeviceAccountConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceAccountConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration";
}
unsafe impl Send for DeviceAccountConfiguration {}
unsafe impl Sync for DeviceAccountConfiguration {}
pub struct UserDataAccountSystemAccessManager;
impl UserDataAccountSystemAccessManager {
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddAndShowDeviceAccountsAsync<P0>(accounts: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<DeviceAccountConfiguration>>,
    {
        Self::IUserDataAccountSystemAccessManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddAndShowDeviceAccountsAsync)(windows_core::Interface::as_raw(this), accounts.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SuppressLocalAccountWithAccountAsync(userdataaccountid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IUserDataAccountSystemAccessManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuppressLocalAccountWithAccountAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(userdataaccountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateDeviceAccountAsync<P0>(account: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<DeviceAccountConfiguration>,
    {
        Self::IUserDataAccountSystemAccessManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDeviceAccountAsync)(windows_core::Interface::as_raw(this), account.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DeleteDeviceAccountAsync(accountid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IUserDataAccountSystemAccessManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteDeviceAccountAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(accountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceAccountConfigurationAsync(accountid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<DeviceAccountConfiguration>> {
        Self::IUserDataAccountSystemAccessManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceAccountConfigurationAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(accountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUserDataAccountSystemAccessManagerStatics<R, F: FnOnce(&IUserDataAccountSystemAccessManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserDataAccountSystemAccessManager, IUserDataAccountSystemAccessManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IUserDataAccountSystemAccessManagerStatics2<R, F: FnOnce(&IUserDataAccountSystemAccessManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserDataAccountSystemAccessManager, IUserDataAccountSystemAccessManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for UserDataAccountSystemAccessManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.SystemAccess.UserDataAccountSystemAccessManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeviceAccountAuthenticationType(pub i32);
impl DeviceAccountAuthenticationType {
    pub const Basic: Self = Self(0i32);
    pub const OAuth: Self = Self(1i32);
    pub const SingleSignOn: Self = Self(2i32);
}
impl windows_core::TypeKind for DeviceAccountAuthenticationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeviceAccountAuthenticationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeviceAccountAuthenticationType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeviceAccountAuthenticationType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountAuthenticationType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeviceAccountIconId(pub i32);
impl DeviceAccountIconId {
    pub const Exchange: Self = Self(0i32);
    pub const Msa: Self = Self(1i32);
    pub const Outlook: Self = Self(2i32);
    pub const Generic: Self = Self(3i32);
}
impl windows_core::TypeKind for DeviceAccountIconId {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeviceAccountIconId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeviceAccountIconId").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeviceAccountIconId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountIconId;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeviceAccountMailAgeFilter(pub i32);
impl DeviceAccountMailAgeFilter {
    pub const All: Self = Self(0i32);
    pub const Last1Day: Self = Self(1i32);
    pub const Last3Days: Self = Self(2i32);
    pub const Last7Days: Self = Self(3i32);
    pub const Last14Days: Self = Self(4i32);
    pub const Last30Days: Self = Self(5i32);
    pub const Last90Days: Self = Self(6i32);
}
impl windows_core::TypeKind for DeviceAccountMailAgeFilter {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeviceAccountMailAgeFilter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeviceAccountMailAgeFilter").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeviceAccountMailAgeFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountMailAgeFilter;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeviceAccountServerType(pub i32);
impl DeviceAccountServerType {
    pub const Exchange: Self = Self(0i32);
    pub const Pop: Self = Self(1i32);
    pub const Imap: Self = Self(2i32);
}
impl windows_core::TypeKind for DeviceAccountServerType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeviceAccountServerType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeviceAccountServerType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeviceAccountServerType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountServerType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeviceAccountSyncScheduleKind(pub i32);
impl DeviceAccountSyncScheduleKind {
    pub const Manual: Self = Self(0i32);
    pub const Every15Minutes: Self = Self(1i32);
    pub const Every30Minutes: Self = Self(2i32);
    pub const Every60Minutes: Self = Self(3i32);
    pub const Every2Hours: Self = Self(4i32);
    pub const Daily: Self = Self(5i32);
    pub const AsItemsArrive: Self = Self(6i32);
}
impl windows_core::TypeKind for DeviceAccountSyncScheduleKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeviceAccountSyncScheduleKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeviceAccountSyncScheduleKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeviceAccountSyncScheduleKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountSyncScheduleKind;i4)");
}
