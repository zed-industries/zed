windows_core::imp::define_interface!(IWiFiDirectService, IWiFiDirectService_Vtbl, 0x50aabbb8_5f71_45ec_84f1_a1e4fc7879a3);
impl windows_core::RuntimeType for IWiFiDirectService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectService_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub RemoteServiceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RemoteServiceInfo: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedConfigurationMethods: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedConfigurationMethods: usize,
    pub PreferGroupOwnerMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetPreferGroupOwnerMode: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SessionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SessionInfo: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSessionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSessionInfo: usize,
    pub ServiceError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WiFiDirectServiceError) -> windows_core::HRESULT,
    pub SessionDeferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSessionDeferred: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub GetProvisioningInfoAsync: unsafe extern "system" fn(*mut core::ffi::c_void, WiFiDirectServiceConfigurationMethod, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectAsyncWithPin: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWiFiDirectServiceAdvertiser, IWiFiDirectServiceAdvertiser_Vtbl, 0xa4aa1ee1_9d8f_4f4f_93ee_7ddea2e37f46);
impl windows_core::RuntimeType for IWiFiDirectServiceAdvertiser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectServiceAdvertiser_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ServiceNamePrefixes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServiceNamePrefixes: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ServiceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ServiceInfo: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetServiceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetServiceInfo: usize,
    pub AutoAcceptSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutoAcceptSession: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PreferGroupOwnerMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetPreferGroupOwnerMode: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PreferredConfigurationMethods: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PreferredConfigurationMethods: usize,
    pub ServiceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WiFiDirectServiceStatus) -> windows_core::HRESULT,
    pub SetServiceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, WiFiDirectServiceStatus) -> windows_core::HRESULT,
    pub CustomServiceStatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCustomServiceStatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub DeferredSessionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DeferredSessionInfo: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetDeferredSessionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetDeferredSessionInfo: usize,
    pub AdvertisementStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WiFiDirectServiceAdvertisementStatus) -> windows_core::HRESULT,
    pub ServiceError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WiFiDirectServiceError) -> windows_core::HRESULT,
    pub SessionRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSessionRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AutoAcceptSessionConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAutoAcceptSessionConnected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AdvertisementStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAdvertisementStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub ConnectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    ConnectAsync: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub ConnectAsyncWithPin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    ConnectAsyncWithPin: usize,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWiFiDirectServiceAdvertiserFactory, IWiFiDirectServiceAdvertiserFactory_Vtbl, 0x3106ac0d_b446_4f13_9f9a_8ae925feba2b);
impl windows_core::RuntimeType for IWiFiDirectServiceAdvertiserFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectServiceAdvertiserFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWiFiDirectServiceAdvertiser: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs, IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_Vtbl, 0xdcd9e01e_83df_43e5_8f43_cbe8479e84eb);
impl windows_core::RuntimeType for IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SessionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SessionInfo: usize,
}
windows_core::imp::define_interface!(IWiFiDirectServiceProvisioningInfo, IWiFiDirectServiceProvisioningInfo_Vtbl, 0x8bdb7cfe_97d9_45a2_8e99_db50910fb6a6);
impl windows_core::RuntimeType for IWiFiDirectServiceProvisioningInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectServiceProvisioningInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SelectedConfigurationMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WiFiDirectServiceConfigurationMethod) -> windows_core::HRESULT,
    pub IsGroupFormationNeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWiFiDirectServiceRemotePortAddedEventArgs, IWiFiDirectServiceRemotePortAddedEventArgs_Vtbl, 0xd4cebac1_3fd3_4f0e_b7bd_782906f44411);
impl windows_core::RuntimeType for IWiFiDirectServiceRemotePortAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectServiceRemotePortAddedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub EndpointPairs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))]
    EndpointPairs: usize,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WiFiDirectServiceIPProtocol) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWiFiDirectServiceSession, IWiFiDirectServiceSession_Vtbl, 0x81142163_e426_47cb_8640_e1b3588bf26f);
impl windows_core::RuntimeType for IWiFiDirectServiceSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectServiceSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WiFiDirectServiceSessionStatus) -> windows_core::HRESULT,
    pub ErrorStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WiFiDirectServiceSessionErrorStatus) -> windows_core::HRESULT,
    pub SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AdvertisementId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ServiceAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SessionAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub GetConnectionEndpointPairs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))]
    GetConnectionEndpointPairs: usize,
    pub SessionStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSessionStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Networking_Sockets")]
    pub AddStreamSocketListenerAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    AddStreamSocketListenerAsync: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub AddDatagramSocketAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    AddDatagramSocketAsync: usize,
    pub RemotePortAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRemotePortAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWiFiDirectServiceSessionDeferredEventArgs, IWiFiDirectServiceSessionDeferredEventArgs_Vtbl, 0x8dfc197f_1201_4f1f_b6f4_5df1b7b9fb2e);
impl windows_core::RuntimeType for IWiFiDirectServiceSessionDeferredEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectServiceSessionDeferredEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub DeferredSessionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DeferredSessionInfo: usize,
}
windows_core::imp::define_interface!(IWiFiDirectServiceSessionRequest, IWiFiDirectServiceSessionRequest_Vtbl, 0xa0e27c8b_50cb_4a58_9bcf_e472b99fba04);
impl windows_core::RuntimeType for IWiFiDirectServiceSessionRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectServiceSessionRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
    pub ProvisioningInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SessionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SessionInfo: usize,
}
windows_core::imp::define_interface!(IWiFiDirectServiceSessionRequestedEventArgs, IWiFiDirectServiceSessionRequestedEventArgs_Vtbl, 0x74bdcc11_53d6_4999_b4f8_6c8ecc1771e7);
impl windows_core::RuntimeType for IWiFiDirectServiceSessionRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectServiceSessionRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetSessionRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWiFiDirectServiceStatics, IWiFiDirectServiceStatics_Vtbl, 0x7db40045_fd74_4688_b725_5dce86acf233);
impl windows_core::RuntimeType for IWiFiDirectServiceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectServiceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetSelector: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetSelectorWithFilter: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetSelectorWithFilter: usize,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectService(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectService, windows_core::IUnknown, windows_core::IInspectable);
impl WiFiDirectService {
    #[cfg(feature = "Storage_Streams")]
    pub fn RemoteServiceInfo(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteServiceInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedConfigurationMethods(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<WiFiDirectServiceConfigurationMethod>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedConfigurationMethods)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreferGroupOwnerMode(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreferGroupOwnerMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPreferGroupOwnerMode(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPreferGroupOwnerMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SessionInfo(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSessionInfo<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSessionInfo)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ServiceError(&self) -> windows_core::Result<WiFiDirectServiceError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SessionDeferred<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<WiFiDirectService, WiFiDirectServiceSessionDeferredEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionDeferred)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSessionDeferred(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSessionDeferred)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetProvisioningInfoAsync(&self, selectedconfigurationmethod: WiFiDirectServiceConfigurationMethod) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceProvisioningInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetProvisioningInfoAsync)(windows_core::Interface::as_raw(this), selectedconfigurationmethod, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectAsyncWithPin(&self, pin: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectAsyncWithPin)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(pin), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSelector(servicename: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        Self::IWiFiDirectServiceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSelector)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(servicename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetSelectorWithFilter<P0>(servicename: &windows_core::HSTRING, serviceinfofilter: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        Self::IWiFiDirectServiceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSelectorWithFilter)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(servicename), serviceinfofilter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectService>> {
        Self::IWiFiDirectServiceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWiFiDirectServiceStatics<R, F: FnOnce(&IWiFiDirectServiceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WiFiDirectService, IWiFiDirectServiceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WiFiDirectService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectService>();
}
unsafe impl windows_core::Interface for WiFiDirectService {
    type Vtable = IWiFiDirectService_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectService as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectService {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectService";
}
unsafe impl Send for WiFiDirectService {}
unsafe impl Sync for WiFiDirectService {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectServiceAdvertiser(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectServiceAdvertiser, windows_core::IUnknown, windows_core::IInspectable);
impl WiFiDirectServiceAdvertiser {
    pub fn ServiceName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServiceNamePrefixes(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceNamePrefixes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ServiceInfo(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetServiceInfo<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetServiceInfo)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AutoAcceptSession(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoAcceptSession)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoAcceptSession(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutoAcceptSession)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PreferGroupOwnerMode(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreferGroupOwnerMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPreferGroupOwnerMode(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPreferGroupOwnerMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PreferredConfigurationMethods(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<WiFiDirectServiceConfigurationMethod>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreferredConfigurationMethods)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ServiceStatus(&self) -> windows_core::Result<WiFiDirectServiceStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetServiceStatus(&self, value: WiFiDirectServiceStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetServiceStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomServiceStatusCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomServiceStatusCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCustomServiceStatusCode(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCustomServiceStatusCode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn DeferredSessionInfo(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeferredSessionInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetDeferredSessionInfo<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDeferredSessionInfo)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AdvertisementStatus(&self) -> windows_core::Result<WiFiDirectServiceAdvertisementStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdvertisementStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ServiceError(&self) -> windows_core::Result<WiFiDirectServiceError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SessionRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, WiFiDirectServiceSessionRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSessionRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSessionRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AutoAcceptSessionConnected<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, WiFiDirectServiceAutoAcceptSessionConnectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoAcceptSessionConnected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAutoAcceptSessionConnected(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAutoAcceptSessionConnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AdvertisementStatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdvertisementStatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAdvertisementStatusChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAdvertisementStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn ConnectAsync<P0>(&self, deviceinfo: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>
    where
        P0: windows_core::Param<super::super::Enumeration::DeviceInformation>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectAsync)(windows_core::Interface::as_raw(this), deviceinfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn ConnectAsyncWithPin<P0>(&self, deviceinfo: P0, pin: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>
    where
        P0: windows_core::Param<super::super::Enumeration::DeviceInformation>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectAsyncWithPin)(windows_core::Interface::as_raw(this), deviceinfo.param().abi(), core::mem::transmute_copy(pin), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateWiFiDirectServiceAdvertiser(servicename: &windows_core::HSTRING) -> windows_core::Result<WiFiDirectServiceAdvertiser> {
        Self::IWiFiDirectServiceAdvertiserFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWiFiDirectServiceAdvertiser)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(servicename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWiFiDirectServiceAdvertiserFactory<R, F: FnOnce(&IWiFiDirectServiceAdvertiserFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WiFiDirectServiceAdvertiser, IWiFiDirectServiceAdvertiserFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WiFiDirectServiceAdvertiser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectServiceAdvertiser>();
}
unsafe impl windows_core::Interface for WiFiDirectServiceAdvertiser {
    type Vtable = IWiFiDirectServiceAdvertiser_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectServiceAdvertiser as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectServiceAdvertiser {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertiser";
}
unsafe impl Send for WiFiDirectServiceAdvertiser {}
unsafe impl Sync for WiFiDirectServiceAdvertiser {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectServiceAutoAcceptSessionConnectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectServiceAutoAcceptSessionConnectedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    pub fn Session(&self) -> windows_core::Result<WiFiDirectServiceSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Session)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SessionInfo(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs>();
}
unsafe impl windows_core::Interface for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    type Vtable = IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAutoAcceptSessionConnectedEventArgs";
}
unsafe impl Send for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {}
unsafe impl Sync for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectServiceProvisioningInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectServiceProvisioningInfo, windows_core::IUnknown, windows_core::IInspectable);
impl WiFiDirectServiceProvisioningInfo {
    pub fn SelectedConfigurationMethod(&self) -> windows_core::Result<WiFiDirectServiceConfigurationMethod> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedConfigurationMethod)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsGroupFormationNeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGroupFormationNeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WiFiDirectServiceProvisioningInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectServiceProvisioningInfo>();
}
unsafe impl windows_core::Interface for WiFiDirectServiceProvisioningInfo {
    type Vtable = IWiFiDirectServiceProvisioningInfo_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectServiceProvisioningInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectServiceProvisioningInfo {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceProvisioningInfo";
}
unsafe impl Send for WiFiDirectServiceProvisioningInfo {}
unsafe impl Sync for WiFiDirectServiceProvisioningInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectServiceRemotePortAddedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectServiceRemotePortAddedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WiFiDirectServiceRemotePortAddedEventArgs {
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub fn EndpointPairs(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Networking::EndpointPair>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndpointPairs)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Protocol(&self) -> windows_core::Result<WiFiDirectServiceIPProtocol> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Protocol)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WiFiDirectServiceRemotePortAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectServiceRemotePortAddedEventArgs>();
}
unsafe impl windows_core::Interface for WiFiDirectServiceRemotePortAddedEventArgs {
    type Vtable = IWiFiDirectServiceRemotePortAddedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectServiceRemotePortAddedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectServiceRemotePortAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceRemotePortAddedEventArgs";
}
unsafe impl Send for WiFiDirectServiceRemotePortAddedEventArgs {}
unsafe impl Sync for WiFiDirectServiceRemotePortAddedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectServiceSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectServiceSession, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WiFiDirectServiceSession, super::super::super::Foundation::IClosable);
impl WiFiDirectServiceSession {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ServiceName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<WiFiDirectServiceSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorStatus(&self) -> windows_core::Result<WiFiDirectServiceSessionErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SessionId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AdvertisementId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdvertisementId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ServiceAddress(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SessionAddress(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub fn GetConnectionEndpointPairs(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Networking::EndpointPair>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConnectionEndpointPairs)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SessionStatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceSession, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionStatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSessionStatusChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSessionStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn AddStreamSocketListenerAsync<P0>(&self, value: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Networking::Sockets::StreamSocketListener>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddStreamSocketListenerAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn AddDatagramSocketAsync<P0>(&self, value: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Networking::Sockets::DatagramSocket>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddDatagramSocketAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemotePortAdded<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceSession, WiFiDirectServiceRemotePortAddedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemotePortAdded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRemotePortAdded(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRemotePortAdded)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for WiFiDirectServiceSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectServiceSession>();
}
unsafe impl windows_core::Interface for WiFiDirectServiceSession {
    type Vtable = IWiFiDirectServiceSession_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectServiceSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectServiceSession {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSession";
}
unsafe impl Send for WiFiDirectServiceSession {}
unsafe impl Sync for WiFiDirectServiceSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectServiceSessionDeferredEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectServiceSessionDeferredEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WiFiDirectServiceSessionDeferredEventArgs {
    #[cfg(feature = "Storage_Streams")]
    pub fn DeferredSessionInfo(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeferredSessionInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WiFiDirectServiceSessionDeferredEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectServiceSessionDeferredEventArgs>();
}
unsafe impl windows_core::Interface for WiFiDirectServiceSessionDeferredEventArgs {
    type Vtable = IWiFiDirectServiceSessionDeferredEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectServiceSessionDeferredEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectServiceSessionDeferredEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionDeferredEventArgs";
}
unsafe impl Send for WiFiDirectServiceSessionDeferredEventArgs {}
unsafe impl Sync for WiFiDirectServiceSessionDeferredEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectServiceSessionRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectServiceSessionRequest, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WiFiDirectServiceSessionRequest, super::super::super::Foundation::IClosable);
impl WiFiDirectServiceSessionRequest {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> windows_core::Result<super::super::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProvisioningInfo(&self) -> windows_core::Result<WiFiDirectServiceProvisioningInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProvisioningInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SessionInfo(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WiFiDirectServiceSessionRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectServiceSessionRequest>();
}
unsafe impl windows_core::Interface for WiFiDirectServiceSessionRequest {
    type Vtable = IWiFiDirectServiceSessionRequest_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectServiceSessionRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectServiceSessionRequest {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequest";
}
unsafe impl Send for WiFiDirectServiceSessionRequest {}
unsafe impl Sync for WiFiDirectServiceSessionRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectServiceSessionRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectServiceSessionRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WiFiDirectServiceSessionRequestedEventArgs {
    pub fn GetSessionRequest(&self) -> windows_core::Result<WiFiDirectServiceSessionRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSessionRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WiFiDirectServiceSessionRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectServiceSessionRequestedEventArgs>();
}
unsafe impl windows_core::Interface for WiFiDirectServiceSessionRequestedEventArgs {
    type Vtable = IWiFiDirectServiceSessionRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectServiceSessionRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectServiceSessionRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequestedEventArgs";
}
unsafe impl Send for WiFiDirectServiceSessionRequestedEventArgs {}
unsafe impl Sync for WiFiDirectServiceSessionRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WiFiDirectServiceAdvertisementStatus(pub i32);
impl WiFiDirectServiceAdvertisementStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Aborted: Self = Self(3i32);
}
impl windows_core::TypeKind for WiFiDirectServiceAdvertisementStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WiFiDirectServiceAdvertisementStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceAdvertisementStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WiFiDirectServiceAdvertisementStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertisementStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WiFiDirectServiceConfigurationMethod(pub i32);
impl WiFiDirectServiceConfigurationMethod {
    pub const Default: Self = Self(0i32);
    pub const PinDisplay: Self = Self(1i32);
    pub const PinEntry: Self = Self(2i32);
}
impl windows_core::TypeKind for WiFiDirectServiceConfigurationMethod {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WiFiDirectServiceConfigurationMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceConfigurationMethod").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WiFiDirectServiceConfigurationMethod {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceConfigurationMethod;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WiFiDirectServiceError(pub i32);
impl WiFiDirectServiceError {
    pub const Success: Self = Self(0i32);
    pub const RadioNotAvailable: Self = Self(1i32);
    pub const ResourceInUse: Self = Self(2i32);
    pub const UnsupportedHardware: Self = Self(3i32);
    pub const NoHardware: Self = Self(4i32);
}
impl windows_core::TypeKind for WiFiDirectServiceError {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WiFiDirectServiceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceError").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WiFiDirectServiceError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceError;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WiFiDirectServiceIPProtocol(pub i32);
impl WiFiDirectServiceIPProtocol {
    pub const Tcp: Self = Self(6i32);
    pub const Udp: Self = Self(17i32);
}
impl windows_core::TypeKind for WiFiDirectServiceIPProtocol {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WiFiDirectServiceIPProtocol {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceIPProtocol").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WiFiDirectServiceIPProtocol {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceIPProtocol;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WiFiDirectServiceSessionErrorStatus(pub i32);
impl WiFiDirectServiceSessionErrorStatus {
    pub const Ok: Self = Self(0i32);
    pub const Disassociated: Self = Self(1i32);
    pub const LocalClose: Self = Self(2i32);
    pub const RemoteClose: Self = Self(3i32);
    pub const SystemFailure: Self = Self(4i32);
    pub const NoResponseFromRemote: Self = Self(5i32);
}
impl windows_core::TypeKind for WiFiDirectServiceSessionErrorStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WiFiDirectServiceSessionErrorStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceSessionErrorStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WiFiDirectServiceSessionErrorStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionErrorStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WiFiDirectServiceSessionStatus(pub i32);
impl WiFiDirectServiceSessionStatus {
    pub const Closed: Self = Self(0i32);
    pub const Initiated: Self = Self(1i32);
    pub const Requested: Self = Self(2i32);
    pub const Open: Self = Self(3i32);
}
impl windows_core::TypeKind for WiFiDirectServiceSessionStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WiFiDirectServiceSessionStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceSessionStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WiFiDirectServiceSessionStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WiFiDirectServiceStatus(pub i32);
impl WiFiDirectServiceStatus {
    pub const Available: Self = Self(0i32);
    pub const Busy: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
}
impl windows_core::TypeKind for WiFiDirectServiceStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WiFiDirectServiceStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WiFiDirectServiceStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceStatus;i4)");
}
