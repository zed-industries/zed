windows_core::imp::define_interface!(IBluetoothLEAdvertisementPublisherTriggerDetails, IBluetoothLEAdvertisementPublisherTriggerDetails_Vtbl, 0x610eca86_3480_41c9_a918_7ddadf207e00);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementPublisherTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Advertisement::BluetoothLEAdvertisementPublisherStatus) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))]
    Status: usize,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BluetoothError) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementPublisherTriggerDetails2, IBluetoothLEAdvertisementPublisherTriggerDetails2_Vtbl, 0xd4a3d025_c601_42d6_9829_4ccb3f5cd77f);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementPublisherTriggerDetails2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SelectedTransmitPowerLevelInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementWatcherTriggerDetails, IBluetoothLEAdvertisementWatcherTriggerDetails_Vtbl, 0xa7db5ad7_2257_4e69_9784_fee645c1dce0);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementWatcherTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementWatcherTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BluetoothError) -> windows_core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections"))]
    pub Advertisements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections")))]
    Advertisements: usize,
    pub SignalStrengthFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattCharacteristicNotificationTriggerDetails, IGattCharacteristicNotificationTriggerDetails_Vtbl, 0x9ba03b18_0fec_436a_93b1_f46c697532a2);
impl windows_core::RuntimeType for IGattCharacteristicNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattCharacteristicNotificationTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Characteristic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Characteristic: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
}
windows_core::imp::define_interface!(IGattCharacteristicNotificationTriggerDetails2, IGattCharacteristicNotificationTriggerDetails2_Vtbl, 0x727a50dc_949d_454a_b192_983467e3d50f);
impl windows_core::RuntimeType for IGattCharacteristicNotificationTriggerDetails2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattCharacteristicNotificationTriggerDetails2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BluetoothError) -> windows_core::HRESULT,
    pub EventTriggeringMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BluetoothEventTriggeringMode) -> windows_core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections"))]
    pub ValueChangedEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections")))]
    ValueChangedEvents: usize,
}
windows_core::imp::define_interface!(IGattServiceProviderConnection, IGattServiceProviderConnection_Vtbl, 0x7fa1b9b9_2f13_40b5_9582_8eb78e98ef13);
impl windows_core::RuntimeType for IGattServiceProviderConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattServiceProviderConnection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TriggerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Service: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Service: usize,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattServiceProviderConnectionStatics, IGattServiceProviderConnectionStatics_Vtbl, 0x3d509f4b_0b0e_4466_b8cd_6ebdda1fa17d);
impl windows_core::RuntimeType for IGattServiceProviderConnectionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattServiceProviderConnectionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AllServices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllServices: usize,
}
windows_core::imp::define_interface!(IGattServiceProviderTriggerDetails, IGattServiceProviderTriggerDetails_Vtbl, 0xae8c0625_05ff_4afb_b16a_de95f3cf0158);
impl windows_core::RuntimeType for IGattServiceProviderTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattServiceProviderTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRfcommConnectionTriggerDetails, IRfcommConnectionTriggerDetails_Vtbl, 0xf922734d_2e3c_4efc_ab59_fc5cf96f97e3);
impl windows_core::RuntimeType for IRfcommConnectionTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRfcommConnectionTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_Sockets")]
    pub Socket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    Socket: usize,
    pub Incoming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub RemoteDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRfcommInboundConnectionInformation, IRfcommInboundConnectionInformation_Vtbl, 0x6d3e75a8_5429_4059_92e3_1e8b65528707);
impl windows_core::RuntimeType for IRfcommInboundConnectionInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRfcommInboundConnectionInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SdpRecord: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SdpRecord: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSdpRecord: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSdpRecord: usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub LocalServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    LocalServiceId: usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub SetLocalServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    SetLocalServiceId: usize,
    pub ServiceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BluetoothServiceCapabilities) -> windows_core::HRESULT,
    pub SetServiceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, super::BluetoothServiceCapabilities) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRfcommOutboundConnectionInformation, IRfcommOutboundConnectionInformation_Vtbl, 0xb091227b_f434_4cb0_99b1_4ab8cedaedd7);
impl windows_core::RuntimeType for IRfcommOutboundConnectionInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRfcommOutboundConnectionInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub RemoteServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    RemoteServiceId: usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub SetRemoteServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    SetRemoteServiceId: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BluetoothLEAdvertisementPublisherTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEAdvertisementPublisherTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothLEAdvertisementPublisherTriggerDetails {
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn Status(&self) -> windows_core::Result<super::Advertisement::BluetoothLEAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Error(&self) -> windows_core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SelectedTransmitPowerLevelInDBm(&self) -> windows_core::Result<super::super::super::Foundation::IReference<i16>> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisherTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedTransmitPowerLevelInDBm)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BluetoothLEAdvertisementPublisherTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisementPublisherTriggerDetails>();
}
unsafe impl windows_core::Interface for BluetoothLEAdvertisementPublisherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementPublisherTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IBluetoothLEAdvertisementPublisherTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEAdvertisementPublisherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementPublisherTriggerDetails";
}
unsafe impl Send for BluetoothLEAdvertisementPublisherTriggerDetails {}
unsafe impl Sync for BluetoothLEAdvertisementPublisherTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BluetoothLEAdvertisementWatcherTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEAdvertisementWatcherTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothLEAdvertisementWatcherTriggerDetails {
    pub fn Error(&self) -> windows_core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections"))]
    pub fn Advertisements(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::Advertisement::BluetoothLEAdvertisementReceivedEventArgs>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Advertisements)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SignalStrengthFilter(&self) -> windows_core::Result<super::BluetoothSignalStrengthFilter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignalStrengthFilter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BluetoothLEAdvertisementWatcherTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisementWatcherTriggerDetails>();
}
unsafe impl windows_core::Interface for BluetoothLEAdvertisementWatcherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementWatcherTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IBluetoothLEAdvertisementWatcherTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEAdvertisementWatcherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementWatcherTriggerDetails";
}
unsafe impl Send for BluetoothLEAdvertisementWatcherTriggerDetails {}
unsafe impl Sync for BluetoothLEAdvertisementWatcherTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GattCharacteristicNotificationTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattCharacteristicNotificationTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl GattCharacteristicNotificationTriggerDetails {
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Characteristic(&self) -> windows_core::Result<super::GenericAttributeProfile::GattCharacteristic> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Characteristic)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Value(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Error(&self) -> windows_core::Result<super::BluetoothError> {
        let this = &windows_core::Interface::cast::<IGattCharacteristicNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EventTriggeringMode(&self) -> windows_core::Result<BluetoothEventTriggeringMode> {
        let this = &windows_core::Interface::cast::<IGattCharacteristicNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EventTriggeringMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections"))]
    pub fn ValueChangedEvents(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::GenericAttributeProfile::GattValueChangedEventArgs>> {
        let this = &windows_core::Interface::cast::<IGattCharacteristicNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValueChangedEvents)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GattCharacteristicNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattCharacteristicNotificationTriggerDetails>();
}
unsafe impl windows_core::Interface for GattCharacteristicNotificationTriggerDetails {
    type Vtable = IGattCharacteristicNotificationTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IGattCharacteristicNotificationTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattCharacteristicNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.GattCharacteristicNotificationTriggerDetails";
}
unsafe impl Send for GattCharacteristicNotificationTriggerDetails {}
unsafe impl Sync for GattCharacteristicNotificationTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GattServiceProviderConnection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattServiceProviderConnection, windows_core::IUnknown, windows_core::IInspectable);
impl GattServiceProviderConnection {
    pub fn TriggerId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriggerId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Service(&self) -> windows_core::Result<super::GenericAttributeProfile::GattLocalService> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Service)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllServices() -> windows_core::Result<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, GattServiceProviderConnection>> {
        Self::IGattServiceProviderConnectionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllServices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGattServiceProviderConnectionStatics<R, F: FnOnce(&IGattServiceProviderConnectionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattServiceProviderConnection, IGattServiceProviderConnectionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GattServiceProviderConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattServiceProviderConnection>();
}
unsafe impl windows_core::Interface for GattServiceProviderConnection {
    type Vtable = IGattServiceProviderConnection_Vtbl;
    const IID: windows_core::GUID = <IGattServiceProviderConnection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattServiceProviderConnection {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.GattServiceProviderConnection";
}
unsafe impl Send for GattServiceProviderConnection {}
unsafe impl Sync for GattServiceProviderConnection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GattServiceProviderTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattServiceProviderTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl GattServiceProviderTriggerDetails {
    pub fn Connection(&self) -> windows_core::Result<GattServiceProviderConnection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Connection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GattServiceProviderTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattServiceProviderTriggerDetails>();
}
unsafe impl windows_core::Interface for GattServiceProviderTriggerDetails {
    type Vtable = IGattServiceProviderTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IGattServiceProviderTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattServiceProviderTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.GattServiceProviderTriggerDetails";
}
unsafe impl Send for GattServiceProviderTriggerDetails {}
unsafe impl Sync for GattServiceProviderTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RfcommConnectionTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RfcommConnectionTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl RfcommConnectionTriggerDetails {
    #[cfg(feature = "Networking_Sockets")]
    pub fn Socket(&self) -> windows_core::Result<super::super::super::Networking::Sockets::StreamSocket> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Socket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Incoming(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Incoming)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RemoteDevice(&self) -> windows_core::Result<super::BluetoothDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteDevice)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RfcommConnectionTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRfcommConnectionTriggerDetails>();
}
unsafe impl windows_core::Interface for RfcommConnectionTriggerDetails {
    type Vtable = IRfcommConnectionTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IRfcommConnectionTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RfcommConnectionTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.RfcommConnectionTriggerDetails";
}
unsafe impl Send for RfcommConnectionTriggerDetails {}
unsafe impl Sync for RfcommConnectionTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RfcommInboundConnectionInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RfcommInboundConnectionInformation, windows_core::IUnknown, windows_core::IInspectable);
impl RfcommInboundConnectionInformation {
    #[cfg(feature = "Storage_Streams")]
    pub fn SdpRecord(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SdpRecord)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSdpRecord<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSdpRecord)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub fn LocalServiceId(&self) -> windows_core::Result<super::Rfcomm::RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalServiceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub fn SetLocalServiceId<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Rfcomm::RfcommServiceId>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLocalServiceId)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ServiceCapabilities(&self) -> windows_core::Result<super::BluetoothServiceCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceCapabilities)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetServiceCapabilities(&self, value: super::BluetoothServiceCapabilities) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetServiceCapabilities)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for RfcommInboundConnectionInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRfcommInboundConnectionInformation>();
}
unsafe impl windows_core::Interface for RfcommInboundConnectionInformation {
    type Vtable = IRfcommInboundConnectionInformation_Vtbl;
    const IID: windows_core::GUID = <IRfcommInboundConnectionInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RfcommInboundConnectionInformation {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.RfcommInboundConnectionInformation";
}
unsafe impl Send for RfcommInboundConnectionInformation {}
unsafe impl Sync for RfcommInboundConnectionInformation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RfcommOutboundConnectionInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RfcommOutboundConnectionInformation, windows_core::IUnknown, windows_core::IInspectable);
impl RfcommOutboundConnectionInformation {
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub fn RemoteServiceId(&self) -> windows_core::Result<super::Rfcomm::RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteServiceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub fn SetRemoteServiceId<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Rfcomm::RfcommServiceId>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRemoteServiceId)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for RfcommOutboundConnectionInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRfcommOutboundConnectionInformation>();
}
unsafe impl windows_core::Interface for RfcommOutboundConnectionInformation {
    type Vtable = IRfcommOutboundConnectionInformation_Vtbl;
    const IID: windows_core::GUID = <IRfcommOutboundConnectionInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RfcommOutboundConnectionInformation {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.RfcommOutboundConnectionInformation";
}
unsafe impl Send for RfcommOutboundConnectionInformation {}
unsafe impl Sync for RfcommOutboundConnectionInformation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BluetoothEventTriggeringMode(pub i32);
impl BluetoothEventTriggeringMode {
    pub const Serial: Self = Self(0i32);
    pub const Batch: Self = Self(1i32);
    pub const KeepLatest: Self = Self(2i32);
}
impl windows_core::TypeKind for BluetoothEventTriggeringMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BluetoothEventTriggeringMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BluetoothEventTriggeringMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BluetoothEventTriggeringMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Background.BluetoothEventTriggeringMode;i4)");
}
