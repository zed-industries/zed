#[cfg(feature = "Devices_WiFiDirect_Services")]
pub mod Services;
windows_core::imp::define_interface!(IWiFiDirectAdvertisement, IWiFiDirectAdvertisement_Vtbl, 0xab511a2d_2a06_49a1_a584_61435c7905a6);
impl windows_core::RuntimeType for IWiFiDirectAdvertisement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectAdvertisement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub InformationElements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InformationElements: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetInformationElements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetInformationElements: usize,
    pub ListenStateDiscoverability: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WiFiDirectAdvertisementListenStateDiscoverability) -> windows_core::HRESULT,
    pub SetListenStateDiscoverability: unsafe extern "system" fn(*mut core::ffi::c_void, WiFiDirectAdvertisementListenStateDiscoverability) -> windows_core::HRESULT,
    pub IsAutonomousGroupOwnerEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsAutonomousGroupOwnerEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub LegacySettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWiFiDirectAdvertisement2, IWiFiDirectAdvertisement2_Vtbl, 0xb759aa46_d816_491b_917a_b40d7dc403a2);
impl windows_core::RuntimeType for IWiFiDirectAdvertisement2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectAdvertisement2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedConfigurationMethods: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedConfigurationMethods: usize,
}
windows_core::imp::define_interface!(IWiFiDirectAdvertisementPublisher, IWiFiDirectAdvertisementPublisher_Vtbl, 0xb35a2d1a_9b1f_45d9_925a_694d66df68ef);
impl windows_core::RuntimeType for IWiFiDirectAdvertisementPublisher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectAdvertisementPublisher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Advertisement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WiFiDirectAdvertisementPublisherStatus) -> windows_core::HRESULT,
    pub StatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWiFiDirectAdvertisementPublisherStatusChangedEventArgs, IWiFiDirectAdvertisementPublisherStatusChangedEventArgs_Vtbl, 0xaafde53c_5481_46e6_90dd_32116518f192);
impl windows_core::RuntimeType for IWiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectAdvertisementPublisherStatusChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WiFiDirectAdvertisementPublisherStatus) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WiFiDirectError) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWiFiDirectConnectionListener, IWiFiDirectConnectionListener_Vtbl, 0x699c1b0d_8d13_4ee9_b9ec_9c72f8251f7d);
impl windows_core::RuntimeType for IWiFiDirectConnectionListener {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectConnectionListener_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ConnectionRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveConnectionRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWiFiDirectConnectionParameters, IWiFiDirectConnectionParameters_Vtbl, 0xb2e55405_5702_4b16_a02c_bbcd21ef6098);
impl windows_core::RuntimeType for IWiFiDirectConnectionParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectConnectionParameters_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GroupOwnerIntent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetGroupOwnerIntent: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWiFiDirectConnectionParameters2, IWiFiDirectConnectionParameters2_Vtbl, 0xab3b0fbe_aa82_44b4_88c8_e3056b89801d);
impl windows_core::RuntimeType for IWiFiDirectConnectionParameters2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectConnectionParameters2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PreferenceOrderedConfigurationMethods: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PreferenceOrderedConfigurationMethods: usize,
    pub PreferredPairingProcedure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WiFiDirectPairingProcedure) -> windows_core::HRESULT,
    pub SetPreferredPairingProcedure: unsafe extern "system" fn(*mut core::ffi::c_void, WiFiDirectPairingProcedure) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWiFiDirectConnectionParametersStatics, IWiFiDirectConnectionParametersStatics_Vtbl, 0x598af493_7642_456f_b9d8_e8a9eb1f401a);
impl windows_core::RuntimeType for IWiFiDirectConnectionParametersStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectConnectionParametersStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub GetDevicePairingKinds: unsafe extern "system" fn(*mut core::ffi::c_void, WiFiDirectConfigurationMethod, *mut super::Enumeration::DevicePairingKinds) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    GetDevicePairingKinds: usize,
}
windows_core::imp::define_interface!(IWiFiDirectConnectionRequest, IWiFiDirectConnectionRequest_Vtbl, 0x8eb99605_914f_49c3_a614_d18dc5b19b43);
impl windows_core::RuntimeType for IWiFiDirectConnectionRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectConnectionRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
}
windows_core::imp::define_interface!(IWiFiDirectConnectionRequestedEventArgs, IWiFiDirectConnectionRequestedEventArgs_Vtbl, 0xf99d20be_d38d_484f_8215_e7b65abf244c);
impl windows_core::RuntimeType for IWiFiDirectConnectionRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectConnectionRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetConnectionRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWiFiDirectDevice, IWiFiDirectDevice_Vtbl, 0x72deaaa8_72eb_4dae_8a28_8513355d2777);
impl windows_core::RuntimeType for IWiFiDirectDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ConnectionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WiFiDirectConnectionStatus) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ConnectionStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveConnectionStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub GetConnectionEndpointPairs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))]
    GetConnectionEndpointPairs: usize,
}
windows_core::imp::define_interface!(IWiFiDirectDeviceStatics, IWiFiDirectDeviceStatics_Vtbl, 0xe86cb57c_3aac_4851_a792_482aaf931b04);
impl windows_core::RuntimeType for IWiFiDirectDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWiFiDirectDeviceStatics2, IWiFiDirectDeviceStatics2_Vtbl, 0x1a953e49_b103_437e_9226_ab67971342f9);
impl windows_core::RuntimeType for IWiFiDirectDeviceStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectDeviceStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, WiFiDirectDeviceSelectorType, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWiFiDirectInformationElement, IWiFiDirectInformationElement_Vtbl, 0xaffb72d6_76bb_497e_ac8b_dc72838bc309);
impl windows_core::RuntimeType for IWiFiDirectInformationElement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectInformationElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Oui: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Oui: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetOui: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetOui: usize,
    pub OuiType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetOuiType: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetValue: usize,
}
windows_core::imp::define_interface!(IWiFiDirectInformationElementStatics, IWiFiDirectInformationElementStatics_Vtbl, 0xdbd02f16_11a5_4e60_8caa_34772148378a);
impl windows_core::RuntimeType for IWiFiDirectInformationElementStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectInformationElementStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    CreateFromBuffer: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections"))]
    pub CreateFromDeviceInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation_Collections")))]
    CreateFromDeviceInformation: usize,
}
windows_core::imp::define_interface!(IWiFiDirectLegacySettings, IWiFiDirectLegacySettings_Vtbl, 0xa64fdbba_f2fd_4567_a91b_f5c2f5321057);
impl windows_core::RuntimeType for IWiFiDirectLegacySettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWiFiDirectLegacySettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Ssid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSsid: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub Passphrase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Passphrase: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetPassphrase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetPassphrase: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectAdvertisement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectAdvertisement, windows_core::IUnknown, windows_core::IInspectable);
impl WiFiDirectAdvertisement {
    #[cfg(feature = "Foundation_Collections")]
    pub fn InformationElements(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InformationElements)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetInformationElements<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInformationElements)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ListenStateDiscoverability(&self) -> windows_core::Result<WiFiDirectAdvertisementListenStateDiscoverability> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ListenStateDiscoverability)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetListenStateDiscoverability(&self, value: WiFiDirectAdvertisementListenStateDiscoverability) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetListenStateDiscoverability)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAutonomousGroupOwnerEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAutonomousGroupOwnerEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAutonomousGroupOwnerEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsAutonomousGroupOwnerEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LegacySettings(&self) -> windows_core::Result<WiFiDirectLegacySettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LegacySettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedConfigurationMethods(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>> {
        let this = &windows_core::Interface::cast::<IWiFiDirectAdvertisement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedConfigurationMethods)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WiFiDirectAdvertisement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectAdvertisement>();
}
unsafe impl windows_core::Interface for WiFiDirectAdvertisement {
    type Vtable = IWiFiDirectAdvertisement_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectAdvertisement as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectAdvertisement {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisement";
}
unsafe impl Send for WiFiDirectAdvertisement {}
unsafe impl Sync for WiFiDirectAdvertisement {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectAdvertisementPublisher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectAdvertisementPublisher, windows_core::IUnknown, windows_core::IInspectable);
impl WiFiDirectAdvertisementPublisher {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WiFiDirectAdvertisementPublisher, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Advertisement(&self) -> windows_core::Result<WiFiDirectAdvertisement> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Advertisement)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<WiFiDirectAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WiFiDirectAdvertisementPublisher, WiFiDirectAdvertisementPublisherStatusChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for WiFiDirectAdvertisementPublisher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectAdvertisementPublisher>();
}
unsafe impl windows_core::Interface for WiFiDirectAdvertisementPublisher {
    type Vtable = IWiFiDirectAdvertisementPublisher_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectAdvertisementPublisher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectAdvertisementPublisher {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisher";
}
unsafe impl Send for WiFiDirectAdvertisementPublisher {}
unsafe impl Sync for WiFiDirectAdvertisementPublisher {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectAdvertisementPublisherStatusChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectAdvertisementPublisherStatusChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    pub fn Status(&self) -> windows_core::Result<WiFiDirectAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Error(&self) -> windows_core::Result<WiFiDirectError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectAdvertisementPublisherStatusChangedEventArgs>();
}
unsafe impl windows_core::Interface for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IWiFiDirectAdvertisementPublisherStatusChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectAdvertisementPublisherStatusChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisherStatusChangedEventArgs";
}
unsafe impl Send for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {}
unsafe impl Sync for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectConnectionListener(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectConnectionListener, windows_core::IUnknown, windows_core::IInspectable);
impl WiFiDirectConnectionListener {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WiFiDirectConnectionListener, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ConnectionRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WiFiDirectConnectionListener, WiFiDirectConnectionRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveConnectionRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveConnectionRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for WiFiDirectConnectionListener {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectConnectionListener>();
}
unsafe impl windows_core::Interface for WiFiDirectConnectionListener {
    type Vtable = IWiFiDirectConnectionListener_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectConnectionListener as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectConnectionListener {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionListener";
}
unsafe impl Send for WiFiDirectConnectionListener {}
unsafe impl Sync for WiFiDirectConnectionListener {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectConnectionParameters(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectConnectionParameters, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Devices_Enumeration")]
windows_core::imp::required_hierarchy!(WiFiDirectConnectionParameters, super::Enumeration::IDevicePairingSettings);
impl WiFiDirectConnectionParameters {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WiFiDirectConnectionParameters, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GroupOwnerIntent(&self) -> windows_core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GroupOwnerIntent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetGroupOwnerIntent(&self, value: i16) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGroupOwnerIntent)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PreferenceOrderedConfigurationMethods(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>> {
        let this = &windows_core::Interface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreferenceOrderedConfigurationMethods)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreferredPairingProcedure(&self) -> windows_core::Result<WiFiDirectPairingProcedure> {
        let this = &windows_core::Interface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreferredPairingProcedure)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPreferredPairingProcedure(&self, value: WiFiDirectPairingProcedure) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPreferredPairingProcedure)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn GetDevicePairingKinds(configurationmethod: WiFiDirectConfigurationMethod) -> windows_core::Result<super::Enumeration::DevicePairingKinds> {
        Self::IWiFiDirectConnectionParametersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDevicePairingKinds)(windows_core::Interface::as_raw(this), configurationmethod, &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IWiFiDirectConnectionParametersStatics<R, F: FnOnce(&IWiFiDirectConnectionParametersStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WiFiDirectConnectionParameters, IWiFiDirectConnectionParametersStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WiFiDirectConnectionParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectConnectionParameters>();
}
unsafe impl windows_core::Interface for WiFiDirectConnectionParameters {
    type Vtable = IWiFiDirectConnectionParameters_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectConnectionParameters as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectConnectionParameters {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionParameters";
}
unsafe impl Send for WiFiDirectConnectionParameters {}
unsafe impl Sync for WiFiDirectConnectionParameters {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectConnectionRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectConnectionRequest, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WiFiDirectConnectionRequest, super::super::Foundation::IClosable);
impl WiFiDirectConnectionRequest {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> windows_core::Result<super::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WiFiDirectConnectionRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectConnectionRequest>();
}
unsafe impl windows_core::Interface for WiFiDirectConnectionRequest {
    type Vtable = IWiFiDirectConnectionRequest_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectConnectionRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectConnectionRequest {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionRequest";
}
unsafe impl Send for WiFiDirectConnectionRequest {}
unsafe impl Sync for WiFiDirectConnectionRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectConnectionRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectConnectionRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WiFiDirectConnectionRequestedEventArgs {
    pub fn GetConnectionRequest(&self) -> windows_core::Result<WiFiDirectConnectionRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConnectionRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WiFiDirectConnectionRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectConnectionRequestedEventArgs>();
}
unsafe impl windows_core::Interface for WiFiDirectConnectionRequestedEventArgs {
    type Vtable = IWiFiDirectConnectionRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectConnectionRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionRequestedEventArgs";
}
unsafe impl Send for WiFiDirectConnectionRequestedEventArgs {}
unsafe impl Sync for WiFiDirectConnectionRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectDevice, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WiFiDirectDevice, super::super::Foundation::IClosable);
impl WiFiDirectDevice {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ConnectionStatus(&self) -> windows_core::Result<WiFiDirectConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectionStatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WiFiDirectDevice, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionStatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveConnectionStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveConnectionStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub fn GetConnectionEndpointPairs(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Networking::EndpointPair>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConnectionEndpointPairs)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IWiFiDirectDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>> {
        Self::IWiFiDirectDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector2(r#type: WiFiDirectDeviceSelectorType) -> windows_core::Result<windows_core::HSTRING> {
        Self::IWiFiDirectDeviceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync2<P0>(deviceid: &windows_core::HSTRING, connectionparameters: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>>
    where
        P0: windows_core::Param<WiFiDirectConnectionParameters>,
    {
        Self::IWiFiDirectDeviceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), connectionparameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWiFiDirectDeviceStatics<R, F: FnOnce(&IWiFiDirectDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WiFiDirectDevice, IWiFiDirectDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWiFiDirectDeviceStatics2<R, F: FnOnce(&IWiFiDirectDeviceStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WiFiDirectDevice, IWiFiDirectDeviceStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WiFiDirectDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectDevice>();
}
unsafe impl windows_core::Interface for WiFiDirectDevice {
    type Vtable = IWiFiDirectDevice_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectDevice {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectDevice";
}
unsafe impl Send for WiFiDirectDevice {}
unsafe impl Sync for WiFiDirectDevice {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectInformationElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectInformationElement, windows_core::IUnknown, windows_core::IInspectable);
impl WiFiDirectInformationElement {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WiFiDirectInformationElement, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Oui(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Oui)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetOui<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOui)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OuiType(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OuiType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOuiType(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOuiType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Value(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn CreateFromBuffer<P0>(buffer: P0) -> windows_core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::IWiFiDirectInformationElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections"))]
    pub fn CreateFromDeviceInformation<P0>(deviceinformation: P0) -> windows_core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>
    where
        P0: windows_core::Param<super::Enumeration::DeviceInformation>,
    {
        Self::IWiFiDirectInformationElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromDeviceInformation)(windows_core::Interface::as_raw(this), deviceinformation.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWiFiDirectInformationElementStatics<R, F: FnOnce(&IWiFiDirectInformationElementStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WiFiDirectInformationElement, IWiFiDirectInformationElementStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WiFiDirectInformationElement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectInformationElement>();
}
unsafe impl windows_core::Interface for WiFiDirectInformationElement {
    type Vtable = IWiFiDirectInformationElement_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectInformationElement as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectInformationElement {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectInformationElement";
}
unsafe impl Send for WiFiDirectInformationElement {}
unsafe impl Sync for WiFiDirectInformationElement {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiDirectLegacySettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiDirectLegacySettings, windows_core::IUnknown, windows_core::IInspectable);
impl WiFiDirectLegacySettings {
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Ssid(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ssid)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSsid(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSsid)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn Passphrase(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Passphrase)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetPassphrase<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPassphrase)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for WiFiDirectLegacySettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWiFiDirectLegacySettings>();
}
unsafe impl windows_core::Interface for WiFiDirectLegacySettings {
    type Vtable = IWiFiDirectLegacySettings_Vtbl;
    const IID: windows_core::GUID = <IWiFiDirectLegacySettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiDirectLegacySettings {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectLegacySettings";
}
unsafe impl Send for WiFiDirectLegacySettings {}
unsafe impl Sync for WiFiDirectLegacySettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WiFiDirectAdvertisementListenStateDiscoverability(pub i32);
impl WiFiDirectAdvertisementListenStateDiscoverability {
    pub const None: Self = Self(0i32);
    pub const Normal: Self = Self(1i32);
    pub const Intensive: Self = Self(2i32);
}
impl windows_core::TypeKind for WiFiDirectAdvertisementListenStateDiscoverability {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WiFiDirectAdvertisementListenStateDiscoverability {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WiFiDirectAdvertisementListenStateDiscoverability").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WiFiDirectAdvertisementListenStateDiscoverability {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementListenStateDiscoverability;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WiFiDirectAdvertisementPublisherStatus(pub i32);
impl WiFiDirectAdvertisementPublisherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Aborted: Self = Self(3i32);
}
impl windows_core::TypeKind for WiFiDirectAdvertisementPublisherStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WiFiDirectAdvertisementPublisherStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WiFiDirectAdvertisementPublisherStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WiFiDirectAdvertisementPublisherStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisherStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WiFiDirectConfigurationMethod(pub i32);
impl WiFiDirectConfigurationMethod {
    pub const ProvidePin: Self = Self(0i32);
    pub const DisplayPin: Self = Self(1i32);
    pub const PushButton: Self = Self(2i32);
}
impl windows_core::TypeKind for WiFiDirectConfigurationMethod {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WiFiDirectConfigurationMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WiFiDirectConfigurationMethod").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WiFiDirectConfigurationMethod {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectConfigurationMethod;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WiFiDirectConnectionStatus(pub i32);
impl WiFiDirectConnectionStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
}
impl windows_core::TypeKind for WiFiDirectConnectionStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WiFiDirectConnectionStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WiFiDirectConnectionStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WiFiDirectConnectionStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectConnectionStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WiFiDirectDeviceSelectorType(pub i32);
impl WiFiDirectDeviceSelectorType {
    pub const DeviceInterface: Self = Self(0i32);
    pub const AssociationEndpoint: Self = Self(1i32);
}
impl windows_core::TypeKind for WiFiDirectDeviceSelectorType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WiFiDirectDeviceSelectorType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WiFiDirectDeviceSelectorType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WiFiDirectDeviceSelectorType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectDeviceSelectorType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WiFiDirectError(pub i32);
impl WiFiDirectError {
    pub const Success: Self = Self(0i32);
    pub const RadioNotAvailable: Self = Self(1i32);
    pub const ResourceInUse: Self = Self(2i32);
}
impl windows_core::TypeKind for WiFiDirectError {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WiFiDirectError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WiFiDirectError").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WiFiDirectError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectError;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WiFiDirectPairingProcedure(pub i32);
impl WiFiDirectPairingProcedure {
    pub const GroupOwnerNegotiation: Self = Self(0i32);
    pub const Invitation: Self = Self(1i32);
}
impl windows_core::TypeKind for WiFiDirectPairingProcedure {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WiFiDirectPairingProcedure {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WiFiDirectPairingProcedure").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WiFiDirectPairingProcedure {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectPairingProcedure;i4)");
}
