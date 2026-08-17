windows_core::imp::define_interface!(IBluetoothLEAdvertisement, IBluetoothLEAdvertisement_Vtbl, 0x066fb2b7_33d1_4e7d_8367_cf81d0f79653);
impl windows_core::RuntimeType for IBluetoothLEAdvertisement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetLocalName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ServiceUuids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServiceUuids: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ManufacturerData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ManufacturerData: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DataSections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DataSections: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetManufacturerDataByCompanyId: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetManufacturerDataByCompanyId: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSectionsByType: unsafe extern "system" fn(*mut core::ffi::c_void, u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSectionsByType: usize,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementBytePattern, IBluetoothLEAdvertisementBytePattern_Vtbl, 0xfbfad7f2_b9c5_4a08_bc51_502f8ef68a79);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementBytePattern {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementBytePattern_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DataType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetDataType: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub Offset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetOffset: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementBytePatternFactory, IBluetoothLEAdvertisementBytePatternFactory_Vtbl, 0xc2e24d73_fd5c_4ec3_be2a_9ca6fa11b7bd);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementBytePatternFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementBytePatternFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u8, i16, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementDataSection, IBluetoothLEAdvertisementDataSection_Vtbl, 0xd7213314_3a43_40f9_b6f0_92bfefc34ae3);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementDataSection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementDataSection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DataType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetDataType: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementDataSectionFactory, IBluetoothLEAdvertisementDataSectionFactory_Vtbl, 0xe7a40942_a845_4045_bf7e_3e9971db8a6b);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementDataSectionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementDataSectionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u8, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementDataTypesStatics, IBluetoothLEAdvertisementDataTypesStatics_Vtbl, 0x3bb6472f_0606_434b_a76e_74159f0684d3);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementDataTypesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementDataTypesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub IncompleteService16BitUuids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub CompleteService16BitUuids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub IncompleteService32BitUuids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub CompleteService32BitUuids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub IncompleteService128BitUuids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub CompleteService128BitUuids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub ShortenedLocalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub CompleteLocalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub TxPowerLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub PeripheralConnectionIntervalRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub ServiceSolicitation16BitUuids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub ServiceSolicitation32BitUuids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub ServiceSolicitation128BitUuids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub ServiceData16BitUuids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub ServiceData32BitUuids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub ServiceData128BitUuids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub PublicTargetAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub RandomTargetAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Appearance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub AdvertisingInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub ManufacturerSpecificData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementFilter, IBluetoothLEAdvertisementFilter_Vtbl, 0x131eb0d3_d04e_47b1_837e_49405bf6f80f);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementFilter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Advertisement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAdvertisement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BytePatterns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BytePatterns: usize,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementPublisher, IBluetoothLEAdvertisementPublisher_Vtbl, 0xcde820f9_d9fa_43d6_a264_ddd8b7da8b78);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementPublisher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BluetoothLEAdvertisementPublisherStatus) -> windows_core::HRESULT,
    pub Advertisement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementPublisher2, IBluetoothLEAdvertisementPublisher2_Vtbl, 0xfbdb545e_56f1_510f_a434_217fbd9e7bd2);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementPublisher2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisher2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PreferredTransmitPowerLevelInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPreferredTransmitPowerLevelInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UseExtendedAdvertisement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetUseExtendedAdvertisement: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsAnonymous: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsAnonymous: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IncludeTransmitPowerLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIncludeTransmitPowerLevel: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementPublisherFactory, IBluetoothLEAdvertisementPublisherFactory_Vtbl, 0x5c5f065e_b863_4981_a1af_1c544d8b0c0d);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementPublisherFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisherFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementPublisherStatusChangedEventArgs, IBluetoothLEAdvertisementPublisherStatusChangedEventArgs_Vtbl, 0x09c2bd9f_2dff_4b23_86ee_0d14fb94aeae);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BluetoothLEAdvertisementPublisherStatus) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BluetoothError) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2, IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_Vtbl, 0x8f62790e_dc88_5c8b_b34e_10b321850f88);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SelectedTransmitPowerLevelInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementReceivedEventArgs, IBluetoothLEAdvertisementReceivedEventArgs_Vtbl, 0x27987ddf_e596_41be_8d43_9e6731d4a913);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RawSignalStrengthInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub BluetoothAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub AdvertisementType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BluetoothLEAdvertisementType) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Advertisement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementReceivedEventArgs2, IBluetoothLEAdvertisementReceivedEventArgs2_Vtbl, 0x12d9c87b_0399_5f0e_a348_53b02b6b162e);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementReceivedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BluetoothAddressType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BluetoothAddressType) -> windows_core::HRESULT,
    pub TransmitPowerLevelInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsAnonymous: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsConnectable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsScannable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDirected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsScanResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementWatcher, IBluetoothLEAdvertisementWatcher_Vtbl, 0xa6ac336f_f3d3_4297_8d6c_c81ea6623f40);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementWatcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MinSamplingInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub MaxSamplingInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub MinOutOfRangeTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub MaxOutOfRangeTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BluetoothLEAdvertisementWatcherStatus) -> windows_core::HRESULT,
    pub ScanningMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BluetoothLEScanningMode) -> windows_core::HRESULT,
    pub SetScanningMode: unsafe extern "system" fn(*mut core::ffi::c_void, BluetoothLEScanningMode) -> windows_core::HRESULT,
    pub SignalStrengthFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSignalStrengthFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AdvertisementFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAdvertisementFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Received: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Stopped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStopped: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementWatcher2, IBluetoothLEAdvertisementWatcher2_Vtbl, 0x01bf26bc_b164_5805_90a3_e8a7997ff225);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementWatcher2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementWatcher2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllowExtendedAdvertisements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowExtendedAdvertisements: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementWatcherFactory, IBluetoothLEAdvertisementWatcherFactory_Vtbl, 0x9aaf2d56_39ac_453e_b32a_85c657e017f1);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementWatcherFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementWatcherFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementWatcherStoppedEventArgs, IBluetoothLEAdvertisementWatcherStoppedEventArgs_Vtbl, 0xdd40f84d_e7b9_43e3_9c04_0685d085fd8c);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementWatcherStoppedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementWatcherStoppedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BluetoothError) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEManufacturerData, IBluetoothLEManufacturerData_Vtbl, 0x912dba18_6963_4533_b061_4694dafb34e5);
impl windows_core::RuntimeType for IBluetoothLEManufacturerData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEManufacturerData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CompanyId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SetCompanyId: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
}
windows_core::imp::define_interface!(IBluetoothLEManufacturerDataFactory, IBluetoothLEManufacturerDataFactory_Vtbl, 0xc09b39f8_319a_441e_8de5_66a81e877a6c);
impl windows_core::RuntimeType for IBluetoothLEManufacturerDataFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEManufacturerDataFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BluetoothLEAdvertisement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEAdvertisement, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothLEAdvertisement {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAdvertisement, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Flags(&self) -> windows_core::Result<super::super::super::Foundation::IReference<BluetoothLEAdvertisementFlags>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Flags)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFlags<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<BluetoothLEAdvertisementFlags>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFlags)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLocalName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLocalName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServiceUuids(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<windows_core::GUID>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceUuids)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ManufacturerData(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEManufacturerData>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManufacturerData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DataSections(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEAdvertisementDataSection>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataSections)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetManufacturerDataByCompanyId(&self, companyid: u16) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<BluetoothLEManufacturerData>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetManufacturerDataByCompanyId)(windows_core::Interface::as_raw(this), companyid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSectionsByType(&self, r#type: u8) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<BluetoothLEAdvertisementDataSection>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSectionsByType)(windows_core::Interface::as_raw(this), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BluetoothLEAdvertisement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisement>();
}
unsafe impl windows_core::Interface for BluetoothLEAdvertisement {
    type Vtable = IBluetoothLEAdvertisement_Vtbl;
    const IID: windows_core::GUID = <IBluetoothLEAdvertisement as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEAdvertisement {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisement";
}
unsafe impl Send for BluetoothLEAdvertisement {}
unsafe impl Sync for BluetoothLEAdvertisement {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BluetoothLEAdvertisementBytePattern(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEAdvertisementBytePattern, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothLEAdvertisementBytePattern {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAdvertisementBytePattern, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DataType(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDataType(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDataType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Offset(&self) -> windows_core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Offset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOffset(&self, value: i16) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetData)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<P0>(datatype: u8, offset: i16, data: P0) -> windows_core::Result<BluetoothLEAdvertisementBytePattern>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        Self::IBluetoothLEAdvertisementBytePatternFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), datatype, offset, data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothLEAdvertisementBytePatternFactory<R, F: FnOnce(&IBluetoothLEAdvertisementBytePatternFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAdvertisementBytePattern, IBluetoothLEAdvertisementBytePatternFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BluetoothLEAdvertisementBytePattern {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisementBytePattern>();
}
unsafe impl windows_core::Interface for BluetoothLEAdvertisementBytePattern {
    type Vtable = IBluetoothLEAdvertisementBytePattern_Vtbl;
    const IID: windows_core::GUID = <IBluetoothLEAdvertisementBytePattern as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEAdvertisementBytePattern {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern";
}
unsafe impl Send for BluetoothLEAdvertisementBytePattern {}
unsafe impl Sync for BluetoothLEAdvertisementBytePattern {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BluetoothLEAdvertisementDataSection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEAdvertisementDataSection, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothLEAdvertisementDataSection {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAdvertisementDataSection, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DataType(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDataType(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDataType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetData)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<P0>(datatype: u8, data: P0) -> windows_core::Result<BluetoothLEAdvertisementDataSection>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        Self::IBluetoothLEAdvertisementDataSectionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), datatype, data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothLEAdvertisementDataSectionFactory<R, F: FnOnce(&IBluetoothLEAdvertisementDataSectionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAdvertisementDataSection, IBluetoothLEAdvertisementDataSectionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BluetoothLEAdvertisementDataSection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisementDataSection>();
}
unsafe impl windows_core::Interface for BluetoothLEAdvertisementDataSection {
    type Vtable = IBluetoothLEAdvertisementDataSection_Vtbl;
    const IID: windows_core::GUID = <IBluetoothLEAdvertisementDataSection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEAdvertisementDataSection {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection";
}
unsafe impl Send for BluetoothLEAdvertisementDataSection {}
unsafe impl Sync for BluetoothLEAdvertisementDataSection {}
pub struct BluetoothLEAdvertisementDataTypes;
impl BluetoothLEAdvertisementDataTypes {
    pub fn Flags() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Flags)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IncompleteService16BitUuids() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncompleteService16BitUuids)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CompleteService16BitUuids() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompleteService16BitUuids)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IncompleteService32BitUuids() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncompleteService32BitUuids)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CompleteService32BitUuids() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompleteService32BitUuids)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IncompleteService128BitUuids() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncompleteService128BitUuids)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CompleteService128BitUuids() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompleteService128BitUuids)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ShortenedLocalName() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShortenedLocalName)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CompleteLocalName() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompleteLocalName)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TxPowerLevel() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TxPowerLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PeripheralConnectionIntervalRange() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PeripheralConnectionIntervalRange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ServiceSolicitation16BitUuids() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceSolicitation16BitUuids)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ServiceSolicitation32BitUuids() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceSolicitation32BitUuids)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ServiceSolicitation128BitUuids() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceSolicitation128BitUuids)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ServiceData16BitUuids() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceData16BitUuids)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ServiceData32BitUuids() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceData32BitUuids)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ServiceData128BitUuids() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceData128BitUuids)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PublicTargetAddress() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PublicTargetAddress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RandomTargetAddress() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RandomTargetAddress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Appearance() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Appearance)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn AdvertisingInterval() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdvertisingInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ManufacturerSpecificData() -> windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManufacturerSpecificData)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothLEAdvertisementDataTypesStatics<R, F: FnOnce(&IBluetoothLEAdvertisementDataTypesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAdvertisementDataTypes, IBluetoothLEAdvertisementDataTypesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for BluetoothLEAdvertisementDataTypes {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataTypes";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BluetoothLEAdvertisementFilter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEAdvertisementFilter, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothLEAdvertisementFilter {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAdvertisementFilter, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Advertisement(&self) -> windows_core::Result<BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Advertisement)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAdvertisement<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<BluetoothLEAdvertisement>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAdvertisement)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn BytePatterns(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEAdvertisementBytePattern>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BytePatterns)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BluetoothLEAdvertisementFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisementFilter>();
}
unsafe impl windows_core::Interface for BluetoothLEAdvertisementFilter {
    type Vtable = IBluetoothLEAdvertisementFilter_Vtbl;
    const IID: windows_core::GUID = <IBluetoothLEAdvertisementFilter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEAdvertisementFilter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFilter";
}
unsafe impl Send for BluetoothLEAdvertisementFilter {}
unsafe impl Sync for BluetoothLEAdvertisementFilter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BluetoothLEAdvertisementPublisher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEAdvertisementPublisher, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothLEAdvertisementPublisher {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAdvertisementPublisher, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Status(&self) -> windows_core::Result<BluetoothLEAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Advertisement(&self) -> windows_core::Result<BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Advertisement)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub fn StatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementPublisher, BluetoothLEAdvertisementPublisherStatusChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStatusChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PreferredTransmitPowerLevelInDBm(&self) -> windows_core::Result<super::super::super::Foundation::IReference<i16>> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreferredTransmitPowerLevelInDBm)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPreferredTransmitPowerLevelInDBm<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<i16>>,
    {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPreferredTransmitPowerLevelInDBm)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn UseExtendedAdvertisement(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseExtendedAdvertisement)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseExtendedAdvertisement(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseExtendedAdvertisement)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAnonymous(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAnonymous)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAnonymous(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsAnonymous)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeTransmitPowerLevel(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeTransmitPowerLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIncludeTransmitPowerLevel(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIncludeTransmitPowerLevel)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<P0>(advertisement: P0) -> windows_core::Result<BluetoothLEAdvertisementPublisher>
    where
        P0: windows_core::Param<BluetoothLEAdvertisement>,
    {
        Self::IBluetoothLEAdvertisementPublisherFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), advertisement.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothLEAdvertisementPublisherFactory<R, F: FnOnce(&IBluetoothLEAdvertisementPublisherFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAdvertisementPublisher, IBluetoothLEAdvertisementPublisherFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BluetoothLEAdvertisementPublisher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisementPublisher>();
}
unsafe impl windows_core::Interface for BluetoothLEAdvertisementPublisher {
    type Vtable = IBluetoothLEAdvertisementPublisher_Vtbl;
    const IID: windows_core::GUID = <IBluetoothLEAdvertisementPublisher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEAdvertisementPublisher {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher";
}
unsafe impl Send for BluetoothLEAdvertisementPublisher {}
unsafe impl Sync for BluetoothLEAdvertisementPublisher {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BluetoothLEAdvertisementPublisherStatusChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEAdvertisementPublisherStatusChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    pub fn Status(&self) -> windows_core::Result<BluetoothLEAdvertisementPublisherStatus> {
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
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedTransmitPowerLevelInDBm)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisementPublisherStatusChangedEventArgs>();
}
unsafe impl windows_core::Interface for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IBluetoothLEAdvertisementPublisherStatusChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBluetoothLEAdvertisementPublisherStatusChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs";
}
unsafe impl Send for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {}
unsafe impl Sync for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BluetoothLEAdvertisementReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEAdvertisementReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothLEAdvertisementReceivedEventArgs {
    pub fn RawSignalStrengthInDBm(&self) -> windows_core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawSignalStrengthInDBm)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BluetoothAddress(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BluetoothAddress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AdvertisementType(&self) -> windows_core::Result<BluetoothLEAdvertisementType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdvertisementType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Advertisement(&self) -> windows_core::Result<BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Advertisement)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BluetoothAddressType(&self) -> windows_core::Result<super::BluetoothAddressType> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BluetoothAddressType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TransmitPowerLevelInDBm(&self) -> windows_core::Result<super::super::super::Foundation::IReference<i16>> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransmitPowerLevelInDBm)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsAnonymous(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAnonymous)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsConnectable(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsConnectable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsScannable(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsScannable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDirected(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDirected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsScanResponse(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsScanResponse)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for BluetoothLEAdvertisementReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisementReceivedEventArgs>();
}
unsafe impl windows_core::Interface for BluetoothLEAdvertisementReceivedEventArgs {
    type Vtable = IBluetoothLEAdvertisementReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBluetoothLEAdvertisementReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEAdvertisementReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs";
}
unsafe impl Send for BluetoothLEAdvertisementReceivedEventArgs {}
unsafe impl Sync for BluetoothLEAdvertisementReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BluetoothLEAdvertisementWatcher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEAdvertisementWatcher, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothLEAdvertisementWatcher {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAdvertisementWatcher, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MinSamplingInterval(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinSamplingInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxSamplingInterval(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxSamplingInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinOutOfRangeTimeout(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinOutOfRangeTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxOutOfRangeTimeout(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxOutOfRangeTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<BluetoothLEAdvertisementWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ScanningMode(&self) -> windows_core::Result<BluetoothLEScanningMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScanningMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScanningMode(&self, value: BluetoothLEScanningMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetScanningMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignalStrengthFilter(&self) -> windows_core::Result<super::BluetoothSignalStrengthFilter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignalStrengthFilter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSignalStrengthFilter<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::BluetoothSignalStrengthFilter>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSignalStrengthFilter)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AdvertisementFilter(&self) -> windows_core::Result<BluetoothLEAdvertisementFilter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdvertisementFilter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAdvertisementFilter<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<BluetoothLEAdvertisementFilter>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAdvertisementFilter)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Received<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Received)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReceived(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Stopped<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementWatcherStoppedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stopped)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStopped(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStopped)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AllowExtendedAdvertisements(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementWatcher2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowExtendedAdvertisements)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowExtendedAdvertisements(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementWatcher2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowExtendedAdvertisements)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<P0>(advertisementfilter: P0) -> windows_core::Result<BluetoothLEAdvertisementWatcher>
    where
        P0: windows_core::Param<BluetoothLEAdvertisementFilter>,
    {
        Self::IBluetoothLEAdvertisementWatcherFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), advertisementfilter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothLEAdvertisementWatcherFactory<R, F: FnOnce(&IBluetoothLEAdvertisementWatcherFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAdvertisementWatcher, IBluetoothLEAdvertisementWatcherFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BluetoothLEAdvertisementWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisementWatcher>();
}
unsafe impl windows_core::Interface for BluetoothLEAdvertisementWatcher {
    type Vtable = IBluetoothLEAdvertisementWatcher_Vtbl;
    const IID: windows_core::GUID = <IBluetoothLEAdvertisementWatcher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEAdvertisementWatcher {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher";
}
unsafe impl Send for BluetoothLEAdvertisementWatcher {}
unsafe impl Sync for BluetoothLEAdvertisementWatcher {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BluetoothLEAdvertisementWatcherStoppedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEAdvertisementWatcherStoppedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothLEAdvertisementWatcherStoppedEventArgs {
    pub fn Error(&self) -> windows_core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisementWatcherStoppedEventArgs>();
}
unsafe impl windows_core::Interface for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    type Vtable = IBluetoothLEAdvertisementWatcherStoppedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBluetoothLEAdvertisementWatcherStoppedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs";
}
unsafe impl Send for BluetoothLEAdvertisementWatcherStoppedEventArgs {}
unsafe impl Sync for BluetoothLEAdvertisementWatcherStoppedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BluetoothLEManufacturerData(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEManufacturerData, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothLEManufacturerData {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEManufacturerData, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CompanyId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompanyId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCompanyId(&self, value: u16) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCompanyId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetData)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<P0>(companyid: u16, data: P0) -> windows_core::Result<BluetoothLEManufacturerData>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        Self::IBluetoothLEManufacturerDataFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), companyid, data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothLEManufacturerDataFactory<R, F: FnOnce(&IBluetoothLEManufacturerDataFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEManufacturerData, IBluetoothLEManufacturerDataFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BluetoothLEManufacturerData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEManufacturerData>();
}
unsafe impl windows_core::Interface for BluetoothLEManufacturerData {
    type Vtable = IBluetoothLEManufacturerData_Vtbl;
    const IID: windows_core::GUID = <IBluetoothLEManufacturerData as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEManufacturerData {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData";
}
unsafe impl Send for BluetoothLEManufacturerData {}
unsafe impl Sync for BluetoothLEManufacturerData {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BluetoothLEAdvertisementFlags(pub u32);
impl BluetoothLEAdvertisementFlags {
    pub const None: Self = Self(0u32);
    pub const LimitedDiscoverableMode: Self = Self(1u32);
    pub const GeneralDiscoverableMode: Self = Self(2u32);
    pub const ClassicNotSupported: Self = Self(4u32);
    pub const DualModeControllerCapable: Self = Self(8u32);
    pub const DualModeHostCapable: Self = Self(16u32);
}
impl windows_core::TypeKind for BluetoothLEAdvertisementFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BluetoothLEAdvertisementFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementFlags").field(&self.0).finish()
    }
}
impl BluetoothLEAdvertisementFlags {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for BluetoothLEAdvertisementFlags {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for BluetoothLEAdvertisementFlags {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for BluetoothLEAdvertisementFlags {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for BluetoothLEAdvertisementFlags {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for BluetoothLEAdvertisementFlags {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for BluetoothLEAdvertisementFlags {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFlags;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BluetoothLEAdvertisementPublisherStatus(pub i32);
impl BluetoothLEAdvertisementPublisherStatus {
    pub const Created: Self = Self(0i32);
    pub const Waiting: Self = Self(1i32);
    pub const Started: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl windows_core::TypeKind for BluetoothLEAdvertisementPublisherStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BluetoothLEAdvertisementPublisherStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementPublisherStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BluetoothLEAdvertisementPublisherStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BluetoothLEAdvertisementType(pub i32);
impl BluetoothLEAdvertisementType {
    pub const ConnectableUndirected: Self = Self(0i32);
    pub const ConnectableDirected: Self = Self(1i32);
    pub const ScannableUndirected: Self = Self(2i32);
    pub const NonConnectableUndirected: Self = Self(3i32);
    pub const ScanResponse: Self = Self(4i32);
    pub const Extended: Self = Self(5i32);
}
impl windows_core::TypeKind for BluetoothLEAdvertisementType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BluetoothLEAdvertisementType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BluetoothLEAdvertisementType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BluetoothLEAdvertisementWatcherStatus(pub i32);
impl BluetoothLEAdvertisementWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Stopping: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Aborted: Self = Self(4i32);
}
impl windows_core::TypeKind for BluetoothLEAdvertisementWatcherStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BluetoothLEAdvertisementWatcherStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementWatcherStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BluetoothLEAdvertisementWatcherStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BluetoothLEScanningMode(pub i32);
impl BluetoothLEScanningMode {
    pub const Passive: Self = Self(0i32);
    pub const Active: Self = Self(1i32);
    pub const None: Self = Self(2i32);
}
impl windows_core::TypeKind for BluetoothLEScanningMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BluetoothLEScanningMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BluetoothLEScanningMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BluetoothLEScanningMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEScanningMode;i4)");
}
