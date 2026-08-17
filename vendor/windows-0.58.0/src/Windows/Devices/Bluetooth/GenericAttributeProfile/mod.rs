windows_core::imp::define_interface!(IGattCharacteristic, IGattCharacteristic_Vtbl, 0x59cb50c1_5934_4f68_a198_eb864fa44e6b);
impl windows_core::RuntimeType for IGattCharacteristic {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattCharacteristic_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetDescriptors: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetDescriptors: usize,
    pub CharacteristicProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattCharacteristicProperties) -> windows_core::HRESULT,
    pub ProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattProtectionLevel) -> windows_core::HRESULT,
    pub SetProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, GattProtectionLevel) -> windows_core::HRESULT,
    pub UserDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Uuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub AttributeHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PresentationFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PresentationFormats: usize,
    pub ReadValueAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadValueWithCacheModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::BluetoothCacheMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub WriteValueAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    WriteValueAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub WriteValueWithOptionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, GattWriteOption, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    WriteValueWithOptionAsync: usize,
    pub ReadClientCharacteristicConfigurationDescriptorAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteClientCharacteristicConfigurationDescriptorAsync: unsafe extern "system" fn(*mut core::ffi::c_void, GattClientCharacteristicConfigurationDescriptorValue, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ValueChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveValueChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattCharacteristic2, IGattCharacteristic2_Vtbl, 0xae1ab578_ec06_4764_b780_9835a1d35d6e);
impl windows_core::RuntimeType for IGattCharacteristic2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattCharacteristic2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Service: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetAllDescriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetAllDescriptors: usize,
}
windows_core::imp::define_interface!(IGattCharacteristic3, IGattCharacteristic3_Vtbl, 0x3f3c663e_93d4_406b_b817_db81f8ed53b3);
impl windows_core::RuntimeType for IGattCharacteristic3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattCharacteristic3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDescriptorsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDescriptorsWithCacheModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::BluetoothCacheMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDescriptorsForUuidAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDescriptorsForUuidWithCacheModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, super::BluetoothCacheMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub WriteValueWithResultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    WriteValueWithResultAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub WriteValueWithResultAndOptionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, GattWriteOption, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    WriteValueWithResultAndOptionAsync: usize,
    pub WriteClientCharacteristicConfigurationDescriptorWithResultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, GattClientCharacteristicConfigurationDescriptorValue, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattCharacteristicStatics, IGattCharacteristicStatics_Vtbl, 0x59cb50c3_5934_4f68_a198_eb864fa44e6b);
impl windows_core::RuntimeType for IGattCharacteristicStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattCharacteristicStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ConvertShortIdToUuid: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConvertShortIdToUuid: usize,
}
windows_core::imp::define_interface!(IGattCharacteristicUuidsStatics, IGattCharacteristicUuidsStatics_Vtbl, 0x58fa4586_b1de_470c_b7de_0d11ff44f4b7);
impl windows_core::RuntimeType for IGattCharacteristicUuidsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattCharacteristicUuidsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BatteryLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub BloodPressureFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub BloodPressureMeasurement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub BodySensorLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CscFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CscMeasurement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GlucoseFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GlucoseMeasurement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GlucoseMeasurementContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub HeartRateControlPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub HeartRateMeasurement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub IntermediateCuffPressure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub IntermediateTemperature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub MeasurementInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RecordAccessControlPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RscFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RscMeasurement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SCControlPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SensorLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TemperatureMeasurement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TemperatureType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattCharacteristicUuidsStatics2, IGattCharacteristicUuidsStatics2_Vtbl, 0x1855b425_d46e_4a2c_9c3f_ed6dea29e7be);
impl windows_core::RuntimeType for IGattCharacteristicUuidsStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattCharacteristicUuidsStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AlertCategoryId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub AlertCategoryIdBitMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub AlertLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub AlertNotificationControlPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub AlertStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GapAppearance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub BootKeyboardInputReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub BootKeyboardOutputReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub BootMouseInputReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CurrentTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CyclingPowerControlPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CyclingPowerFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CyclingPowerMeasurement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CyclingPowerVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub DateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub DayDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub DayOfWeek: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GapDeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub DstOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ExactTime256: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub FirmwareRevisionString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub HardwareRevisionString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub HidControlPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub HidInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Ieee1107320601RegulatoryCertificationDataList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub LnControlPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub LnFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub LocalTimeInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub LocationAndSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ManufacturerNameString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ModelNumberString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Navigation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub NewAlert: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GapPeripheralPreferredConnectionParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GapPeripheralPrivacyFlag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub PnpId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub PositionQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ProtocolMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GapReconnectionAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ReferenceTimeInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Report: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ReportMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RingerControlPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RingerSetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ScanIntervalWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ScanRefresh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SerialNumberString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GattServiceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SoftwareRevisionString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SupportedNewAlertCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SupportUnreadAlertCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SystemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TimeAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TimeSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TimeUpdateControlPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TimeUpdateState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TimeWithDst: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TimeZone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TxPowerLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub UnreadAlertStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattCharacteristicsResult, IGattCharacteristicsResult_Vtbl, 0x1194945c_b257_4f3e_9db7_f68bc9a9aef2);
impl windows_core::RuntimeType for IGattCharacteristicsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattCharacteristicsResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattCommunicationStatus) -> windows_core::HRESULT,
    pub ProtocolError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
windows_core::imp::define_interface!(IGattClientNotificationResult, IGattClientNotificationResult_Vtbl, 0x506d5599_0112_419a_8e3b_ae21afabd2c2);
impl windows_core::RuntimeType for IGattClientNotificationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattClientNotificationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SubscribedClient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattCommunicationStatus) -> windows_core::HRESULT,
    pub ProtocolError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattClientNotificationResult2, IGattClientNotificationResult2_Vtbl, 0x8faec497_45e0_497e_9582_29a1fe281ad5);
impl windows_core::RuntimeType for IGattClientNotificationResult2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattClientNotificationResult2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BytesSent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattDescriptor, IGattDescriptor_Vtbl, 0x92055f2b_8084_4344_b4c2_284de19a8506);
impl windows_core::RuntimeType for IGattDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattProtectionLevel) -> windows_core::HRESULT,
    pub SetProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, GattProtectionLevel) -> windows_core::HRESULT,
    pub Uuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub AttributeHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub ReadValueAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadValueWithCacheModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::BluetoothCacheMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub WriteValueAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    WriteValueAsync: usize,
}
windows_core::imp::define_interface!(IGattDescriptor2, IGattDescriptor2_Vtbl, 0x8f563d39_d630_406c_ba11_10cdd16b0e5e);
impl windows_core::RuntimeType for IGattDescriptor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattDescriptor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub WriteValueWithResultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    WriteValueWithResultAsync: usize,
}
windows_core::imp::define_interface!(IGattDescriptorStatics, IGattDescriptorStatics_Vtbl, 0x92055f2d_8084_4344_b4c2_284de19a8506);
impl windows_core::RuntimeType for IGattDescriptorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattDescriptorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ConvertShortIdToUuid: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConvertShortIdToUuid: usize,
}
windows_core::imp::define_interface!(IGattDescriptorUuidsStatics, IGattDescriptorUuidsStatics_Vtbl, 0xa6f862ce_9cfc_42f1_9185_ff37b75181d3);
impl windows_core::RuntimeType for IGattDescriptorUuidsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattDescriptorUuidsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CharacteristicAggregateFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CharacteristicExtendedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CharacteristicPresentationFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CharacteristicUserDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ClientCharacteristicConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ServerCharacteristicConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattDescriptorsResult, IGattDescriptorsResult_Vtbl, 0x9bc091f3_95e7_4489_8d25_ff81955a57b9);
impl windows_core::RuntimeType for IGattDescriptorsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattDescriptorsResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattCommunicationStatus) -> windows_core::HRESULT,
    pub ProtocolError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Descriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Descriptors: usize,
}
windows_core::imp::define_interface!(IGattDeviceService, IGattDeviceService_Vtbl, 0xac7b7c05_b33c_47cf_990f_6b8f5577df71);
impl windows_core::RuntimeType for IGattDeviceService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattDeviceService_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetCharacteristics: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetCharacteristics: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetIncludedServices: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetIncludedServices: usize,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Uuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub AttributeHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattDeviceService2, IGattDeviceService2_Vtbl, 0xfc54520b_0b0d_4708_bae0_9ffd9489bc59);
impl windows_core::RuntimeType for IGattDeviceService2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattDeviceService2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Device: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ParentServices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ParentServices: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetAllCharacteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetAllCharacteristics: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetAllIncludedServices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetAllIncludedServices: usize,
}
windows_core::imp::define_interface!(IGattDeviceService3, IGattDeviceService3_Vtbl, 0xb293a950_0c53_437c_a9b3_5c3210c6e569);
impl windows_core::RuntimeType for IGattDeviceService3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattDeviceService3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceAccessInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceAccessInformation: usize,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattSharingMode) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub RequestAccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    RequestAccessAsync: usize,
    pub OpenAsync: unsafe extern "system" fn(*mut core::ffi::c_void, GattSharingMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCharacteristicsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCharacteristicsWithCacheModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::BluetoothCacheMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCharacteristicsForUuidAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCharacteristicsForUuidWithCacheModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, super::BluetoothCacheMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIncludedServicesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIncludedServicesWithCacheModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::BluetoothCacheMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIncludedServicesForUuidAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIncludedServicesForUuidWithCacheModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, super::BluetoothCacheMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattDeviceServiceStatics, IGattDeviceServiceStatics_Vtbl, 0x196d0022_faad_45dc_ae5b_2ac3184e84db);
impl windows_core::RuntimeType for IGattDeviceServiceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattDeviceServiceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelectorFromUuid: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub GetDeviceSelectorFromShortId: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDeviceSelectorFromShortId: usize,
    #[cfg(feature = "deprecated")]
    pub ConvertShortIdToUuid: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConvertShortIdToUuid: usize,
}
windows_core::imp::define_interface!(IGattDeviceServiceStatics2, IGattDeviceServiceStatics2_Vtbl, 0x0604186e_24a6_4b0d_a2f2_30cc01545d25);
impl windows_core::RuntimeType for IGattDeviceServiceStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattDeviceServiceStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdWithSharingModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, GattSharingMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceIdWithCacheMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::BluetoothCacheMode, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceIdAndUuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::GUID, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceIdAndUuidWithCacheMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::GUID, super::BluetoothCacheMode, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattDeviceServicesResult, IGattDeviceServicesResult_Vtbl, 0x171dd3ee_016d_419d_838a_576cf475a3d8);
impl windows_core::RuntimeType for IGattDeviceServicesResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattDeviceServicesResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattCommunicationStatus) -> windows_core::HRESULT,
    pub ProtocolError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Services: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Services: usize,
}
windows_core::imp::define_interface!(IGattLocalCharacteristic, IGattLocalCharacteristic_Vtbl, 0xaede376d_5412_4d74_92a8_8deb8526829c);
impl windows_core::RuntimeType for IGattLocalCharacteristic {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattLocalCharacteristic_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub StaticValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StaticValue: usize,
    pub CharacteristicProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattCharacteristicProperties) -> windows_core::HRESULT,
    pub ReadProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattProtectionLevel) -> windows_core::HRESULT,
    pub WriteProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattProtectionLevel) -> windows_core::HRESULT,
    pub CreateDescriptorAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Descriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Descriptors: usize,
    pub UserDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PresentationFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PresentationFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SubscribedClients: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SubscribedClients: usize,
    pub SubscribedClientsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSubscribedClientsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ReadRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReadRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub WriteRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveWriteRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub NotifyValueAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    NotifyValueAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub NotifyValueForSubscribedClientAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    NotifyValueForSubscribedClientAsync: usize,
}
windows_core::imp::define_interface!(IGattLocalCharacteristicParameters, IGattLocalCharacteristicParameters_Vtbl, 0xfaf73db4_4cff_44c7_8445_040e6ead0063);
impl windows_core::RuntimeType for IGattLocalCharacteristicParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattLocalCharacteristicParameters_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SetStaticValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStaticValue: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StaticValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StaticValue: usize,
    pub SetCharacteristicProperties: unsafe extern "system" fn(*mut core::ffi::c_void, GattCharacteristicProperties) -> windows_core::HRESULT,
    pub CharacteristicProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattCharacteristicProperties) -> windows_core::HRESULT,
    pub SetReadProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, GattProtectionLevel) -> windows_core::HRESULT,
    pub ReadProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattProtectionLevel) -> windows_core::HRESULT,
    pub SetWriteProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, GattProtectionLevel) -> windows_core::HRESULT,
    pub WriteProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattProtectionLevel) -> windows_core::HRESULT,
    pub SetUserDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub UserDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PresentationFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PresentationFormats: usize,
}
windows_core::imp::define_interface!(IGattLocalCharacteristicResult, IGattLocalCharacteristicResult_Vtbl, 0x7975de9b_0170_4397_9666_92f863f12ee6);
impl windows_core::RuntimeType for IGattLocalCharacteristicResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattLocalCharacteristicResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Characteristic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BluetoothError) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattLocalDescriptor, IGattLocalDescriptor_Vtbl, 0xf48ebe06_789d_4a4b_8652_bd017b5d2fc6);
impl windows_core::RuntimeType for IGattLocalDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattLocalDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub StaticValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StaticValue: usize,
    pub ReadProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattProtectionLevel) -> windows_core::HRESULT,
    pub WriteProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattProtectionLevel) -> windows_core::HRESULT,
    pub ReadRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReadRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub WriteRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveWriteRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattLocalDescriptorParameters, IGattLocalDescriptorParameters_Vtbl, 0x5fdede6a_f3c1_4b66_8c4b_e3d2293b40e9);
impl windows_core::RuntimeType for IGattLocalDescriptorParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattLocalDescriptorParameters_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SetStaticValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStaticValue: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StaticValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StaticValue: usize,
    pub SetReadProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, GattProtectionLevel) -> windows_core::HRESULT,
    pub ReadProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattProtectionLevel) -> windows_core::HRESULT,
    pub SetWriteProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, GattProtectionLevel) -> windows_core::HRESULT,
    pub WriteProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattProtectionLevel) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattLocalDescriptorResult, IGattLocalDescriptorResult_Vtbl, 0x375791be_321f_4366_bfc1_3bc6b82c79f8);
impl windows_core::RuntimeType for IGattLocalDescriptorResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattLocalDescriptorResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Descriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BluetoothError) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattLocalService, IGattLocalService_Vtbl, 0xf513e258_f7f7_4902_b803_57fcc7d6fe83);
impl windows_core::RuntimeType for IGattLocalService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattLocalService_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CreateCharacteristicAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
windows_core::imp::define_interface!(IGattPresentationFormat, IGattPresentationFormat_Vtbl, 0x196d0021_faad_45dc_ae5b_2ac3184e84db);
impl windows_core::RuntimeType for IGattPresentationFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattPresentationFormat_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FormatType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Exponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Unit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Namespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattPresentationFormatStatics, IGattPresentationFormatStatics_Vtbl, 0x196d0020_faad_45dc_ae5b_2ac3184e84db);
impl windows_core::RuntimeType for IGattPresentationFormatStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattPresentationFormatStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BluetoothSigAssignedNumbers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattPresentationFormatStatics2, IGattPresentationFormatStatics2_Vtbl, 0xa9c21713_b82f_435e_b634_21fd85a43c07);
impl windows_core::RuntimeType for IGattPresentationFormatStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattPresentationFormatStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromParts: unsafe extern "system" fn(*mut core::ffi::c_void, u8, i32, u16, u8, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattPresentationFormatTypesStatics, IGattPresentationFormatTypesStatics_Vtbl, 0xfaf1ba0a_30ba_409c_bef7_cffb6d03b8fb);
impl windows_core::RuntimeType for IGattPresentationFormatTypesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattPresentationFormatTypesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Boolean: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Bit2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Nibble: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub UInt8: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub UInt12: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub UInt16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub UInt24: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub UInt32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub UInt48: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub UInt64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub UInt128: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SInt8: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SInt12: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SInt16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SInt24: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SInt32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SInt48: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SInt64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SInt128: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Float32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Float64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SFloat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Float: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub DUInt16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Utf8: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Utf16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Struct: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattProtocolErrorStatics, IGattProtocolErrorStatics_Vtbl, 0xca46c5c5_0ecc_4809_bea3_cf79bc991e37);
impl windows_core::RuntimeType for IGattProtocolErrorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattProtocolErrorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InvalidHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub ReadNotPermitted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub WriteNotPermitted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub InvalidPdu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub InsufficientAuthentication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub RequestNotSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub InvalidOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub InsufficientAuthorization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub PrepareQueueFull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub AttributeNotFound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub AttributeNotLong: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub InsufficientEncryptionKeySize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub InvalidAttributeValueLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub UnlikelyError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub InsufficientEncryption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub UnsupportedGroupType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub InsufficientResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattReadClientCharacteristicConfigurationDescriptorResult, IGattReadClientCharacteristicConfigurationDescriptorResult_Vtbl, 0x63a66f09_1aea_4c4c_a50f_97bae474b348);
impl windows_core::RuntimeType for IGattReadClientCharacteristicConfigurationDescriptorResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattReadClientCharacteristicConfigurationDescriptorResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattCommunicationStatus) -> windows_core::HRESULT,
    pub ClientCharacteristicConfigurationDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattClientCharacteristicConfigurationDescriptorValue) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattReadClientCharacteristicConfigurationDescriptorResult2, IGattReadClientCharacteristicConfigurationDescriptorResult2_Vtbl, 0x1bf1a59d_ba4d_4622_8651_f4ee150d0a5d);
impl windows_core::RuntimeType for IGattReadClientCharacteristicConfigurationDescriptorResult2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattReadClientCharacteristicConfigurationDescriptorResult2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProtocolError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattReadRequest, IGattReadRequest_Vtbl, 0xf1dd6535_6acd_42a6_a4bb_d789dae0043e);
impl windows_core::RuntimeType for IGattReadRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattReadRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Offset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattRequestState) -> windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub RespondWithValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RespondWithValue: usize,
    pub RespondWithProtocolError: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattReadRequestedEventArgs, IGattReadRequestedEventArgs_Vtbl, 0x93497243_f39c_484b_8ab6_996ba486cfa3);
impl windows_core::RuntimeType for IGattReadRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattReadRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattReadResult, IGattReadResult_Vtbl, 0x63a66f08_1aea_4c4c_a50f_97bae474b348);
impl windows_core::RuntimeType for IGattReadResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattReadResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattCommunicationStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
}
windows_core::imp::define_interface!(IGattReadResult2, IGattReadResult2_Vtbl, 0xa10f50a0_fb43_48af_baaa_638a5c6329fe);
impl windows_core::RuntimeType for IGattReadResult2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattReadResult2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProtocolError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattReliableWriteTransaction, IGattReliableWriteTransaction_Vtbl, 0x63a66f07_1aea_4c4c_a50f_97bae474b348);
impl windows_core::RuntimeType for IGattReliableWriteTransaction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattReliableWriteTransaction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub WriteValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    WriteValue: usize,
    pub CommitAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattReliableWriteTransaction2, IGattReliableWriteTransaction2_Vtbl, 0x51113987_ef12_462f_9fb2_a1a43a679416);
impl windows_core::RuntimeType for IGattReliableWriteTransaction2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattReliableWriteTransaction2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CommitWithResultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattRequestStateChangedEventArgs, IGattRequestStateChangedEventArgs_Vtbl, 0xe834d92c_27be_44b3_9d0d_4fc6e808dd3f);
impl windows_core::RuntimeType for IGattRequestStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattRequestStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattRequestState) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BluetoothError) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattServiceProvider, IGattServiceProvider_Vtbl, 0x7822b3cd_2889_4f86_a051_3f0aed1c2760);
impl windows_core::RuntimeType for IGattServiceProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattServiceProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Service: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AdvertisementStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattServiceProviderAdvertisementStatus) -> windows_core::HRESULT,
    pub AdvertisementStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAdvertisementStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub StartAdvertising: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartAdvertisingWithParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopAdvertising: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattServiceProviderAdvertisementStatusChangedEventArgs, IGattServiceProviderAdvertisementStatusChangedEventArgs_Vtbl, 0x59a5aa65_fa21_4ffc_b155_04d928012686);
impl windows_core::RuntimeType for IGattServiceProviderAdvertisementStatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattServiceProviderAdvertisementStatusChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BluetoothError) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattServiceProviderAdvertisementStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattServiceProviderAdvertisingParameters, IGattServiceProviderAdvertisingParameters_Vtbl, 0xe2ce31ab_6315_4c22_9bd7_781dbc3d8d82);
impl windows_core::RuntimeType for IGattServiceProviderAdvertisingParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattServiceProviderAdvertisingParameters_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetIsConnectable: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsConnectable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsDiscoverable: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsDiscoverable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattServiceProviderAdvertisingParameters2, IGattServiceProviderAdvertisingParameters2_Vtbl, 0xff68468d_ca92_4434_9743_0e90988ad879);
impl windows_core::RuntimeType for IGattServiceProviderAdvertisingParameters2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattServiceProviderAdvertisingParameters2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SetServiceData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetServiceData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ServiceData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ServiceData: usize,
}
windows_core::imp::define_interface!(IGattServiceProviderResult, IGattServiceProviderResult_Vtbl, 0x764696d8_c53e_428c_8a48_67afe02c3ae6);
impl windows_core::RuntimeType for IGattServiceProviderResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattServiceProviderResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BluetoothError) -> windows_core::HRESULT,
    pub ServiceProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattServiceProviderStatics, IGattServiceProviderStatics_Vtbl, 0x31794063_5256_4054_a4f4_7bbe7755a57e);
impl windows_core::RuntimeType for IGattServiceProviderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattServiceProviderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattServiceUuidsStatics, IGattServiceUuidsStatics_Vtbl, 0x6dc57058_9aba_4417_b8f2_dce016d34ee2);
impl windows_core::RuntimeType for IGattServiceUuidsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattServiceUuidsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Battery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub BloodPressure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CyclingSpeedAndCadence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GenericAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GenericAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Glucose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub HealthThermometer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub HeartRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RunningSpeedAndCadence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattServiceUuidsStatics2, IGattServiceUuidsStatics2_Vtbl, 0xd2ae94f5_3d15_4f79_9c0c_eaafa675155c);
impl windows_core::RuntimeType for IGattServiceUuidsStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattServiceUuidsStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AlertNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CurrentTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CyclingPower: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub DeviceInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub HumanInterfaceDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ImmediateAlert: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub LinkLoss: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub LocationAndNavigation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub NextDstChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub PhoneAlertStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ReferenceTimeUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ScanParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TxPower: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattSession, IGattSession_Vtbl, 0xd23b5143_e04e_4c24_999c_9c256f9856b1);
impl windows_core::RuntimeType for IGattSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanMaintainConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetMaintainConnection: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub MaintainConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MaxPduSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SessionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattSessionStatus) -> windows_core::HRESULT,
    pub MaxPduSizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMaxPduSizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SessionStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSessionStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattSessionStatics, IGattSessionStatics_Vtbl, 0x2e65b95c_539f_4db7_82a8_73bdbbf73ebf);
impl windows_core::RuntimeType for IGattSessionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattSessionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromDeviceIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattSessionStatusChangedEventArgs, IGattSessionStatusChangedEventArgs_Vtbl, 0x7605b72e_837f_404c_ab34_3163f39ddf32);
impl windows_core::RuntimeType for IGattSessionStatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattSessionStatusChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BluetoothError) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattSessionStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattSubscribedClient, IGattSubscribedClient_Vtbl, 0x736e9001_15a4_4ec2_9248_e3f20d463be9);
impl windows_core::RuntimeType for IGattSubscribedClient {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattSubscribedClient_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxNotificationSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub MaxNotificationSizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMaxNotificationSizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattValueChangedEventArgs, IGattValueChangedEventArgs_Vtbl, 0xd21bdb54_06e3_4ed8_a263_acfac8ba7313);
impl windows_core::RuntimeType for IGattValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattValueChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CharacteristicValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CharacteristicValue: usize,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattWriteRequest, IGattWriteRequest_Vtbl, 0xaeb6a9ed_de2f_4fc2_a9a8_94ea7844f13d);
impl windows_core::RuntimeType for IGattWriteRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattWriteRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
    pub Offset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Option: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattWriteOption) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattRequestState) -> windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Respond: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RespondWithProtocolError: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattWriteRequestedEventArgs, IGattWriteRequestedEventArgs_Vtbl, 0x2dec8bbe_a73a_471a_94d5_037deadd0806);
impl windows_core::RuntimeType for IGattWriteRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattWriteRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGattWriteResult, IGattWriteResult_Vtbl, 0x4991ddb1_cb2b_44f7_99fc_d29a2871dc9b);
impl windows_core::RuntimeType for IGattWriteResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattWriteResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GattCommunicationStatus) -> windows_core::HRESULT,
    pub ProtocolError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattCharacteristic(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattCharacteristic, windows_core::IUnknown, windows_core::IInspectable);
impl GattCharacteristic {
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetDescriptors(&self, descriptoruuid: windows_core::GUID) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDescriptors)(windows_core::Interface::as_raw(this), descriptoruuid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CharacteristicProperties(&self) -> windows_core::Result<GattCharacteristicProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacteristicProperties)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProtectionLevel(&self) -> windows_core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProtectionLevel(&self, value: GattProtectionLevel) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProtectionLevel)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UserDescription(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserDescription)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Uuid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uuid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AttributeHandle(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttributeHandle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PresentationFormats(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GattPresentationFormat>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PresentationFormats)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReadValueAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadValueAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReadValueWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadValueWithCacheModeAsync)(windows_core::Interface::as_raw(this), cachemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteValueAsync<P0>(&self, value: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteValueAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteValueWithOptionAsync<P0>(&self, value: P0, writeoption: GattWriteOption) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteValueWithOptionAsync)(windows_core::Interface::as_raw(this), value.param().abi(), writeoption, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReadClientCharacteristicConfigurationDescriptorAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattReadClientCharacteristicConfigurationDescriptorResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadClientCharacteristicConfigurationDescriptorAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WriteClientCharacteristicConfigurationDescriptorAsync(&self, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteClientCharacteristicConfigurationDescriptorAsync)(windows_core::Interface::as_raw(this), clientcharacteristicconfigurationdescriptorvalue, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ValueChanged<P0>(&self, valuechangedhandler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GattCharacteristic, GattValueChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValueChanged)(windows_core::Interface::as_raw(this), valuechangedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveValueChanged(&self, valuechangedeventcookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveValueChanged)(windows_core::Interface::as_raw(this), valuechangedeventcookie).ok() }
    }
    pub fn Service(&self) -> windows_core::Result<GattDeviceService> {
        let this = &windows_core::Interface::cast::<IGattCharacteristic2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Service)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetAllDescriptors(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>> {
        let this = &windows_core::Interface::cast::<IGattCharacteristic2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAllDescriptors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDescriptorsAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>> {
        let this = &windows_core::Interface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDescriptorsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDescriptorsWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>> {
        let this = &windows_core::Interface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDescriptorsWithCacheModeAsync)(windows_core::Interface::as_raw(this), cachemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDescriptorsForUuidAsync(&self, descriptoruuid: windows_core::GUID) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>> {
        let this = &windows_core::Interface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDescriptorsForUuidAsync)(windows_core::Interface::as_raw(this), descriptoruuid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDescriptorsForUuidWithCacheModeAsync(&self, descriptoruuid: windows_core::GUID, cachemode: super::BluetoothCacheMode) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>> {
        let this = &windows_core::Interface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDescriptorsForUuidWithCacheModeAsync)(windows_core::Interface::as_raw(this), descriptoruuid, cachemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteValueWithResultAsync<P0>(&self, value: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteValueWithResultAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteValueWithResultAndOptionAsync<P0>(&self, value: P0, writeoption: GattWriteOption) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteValueWithResultAndOptionAsync)(windows_core::Interface::as_raw(this), value.param().abi(), writeoption, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WriteClientCharacteristicConfigurationDescriptorWithResultAsync(&self, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>> {
        let this = &windows_core::Interface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteClientCharacteristicConfigurationDescriptorWithResultAsync)(windows_core::Interface::as_raw(this), clientcharacteristicconfigurationdescriptorvalue, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ConvertShortIdToUuid(shortid: u16) -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConvertShortIdToUuid)(windows_core::Interface::as_raw(this), shortid, &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattCharacteristicStatics<R, F: FnOnce(&IGattCharacteristicStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattCharacteristic, IGattCharacteristicStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GattCharacteristic {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattCharacteristic>();
}
unsafe impl windows_core::Interface for GattCharacteristic {
    type Vtable = IGattCharacteristic_Vtbl;
    const IID: windows_core::GUID = <IGattCharacteristic as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattCharacteristic {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattCharacteristic";
}
unsafe impl Send for GattCharacteristic {}
unsafe impl Sync for GattCharacteristic {}
pub struct GattCharacteristicUuids;
impl GattCharacteristicUuids {
    pub fn BatteryLevel() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BatteryLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BloodPressureFeature() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BloodPressureFeature)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BloodPressureMeasurement() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BloodPressureMeasurement)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BodySensorLocation() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BodySensorLocation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CscFeature() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CscFeature)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CscMeasurement() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CscMeasurement)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GlucoseFeature() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GlucoseFeature)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GlucoseMeasurement() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GlucoseMeasurement)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GlucoseMeasurementContext() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GlucoseMeasurementContext)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn HeartRateControlPoint() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeartRateControlPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn HeartRateMeasurement() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeartRateMeasurement)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IntermediateCuffPressure() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IntermediateCuffPressure)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IntermediateTemperature() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IntermediateTemperature)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MeasurementInterval() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeasurementInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RecordAccessControlPoint() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecordAccessControlPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RscFeature() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RscFeature)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RscMeasurement() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RscMeasurement)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SCControlPoint() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SCControlPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SensorLocation() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SensorLocation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TemperatureMeasurement() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TemperatureMeasurement)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TemperatureType() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TemperatureType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn AlertCategoryId() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlertCategoryId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn AlertCategoryIdBitMask() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlertCategoryIdBitMask)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn AlertLevel() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlertLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn AlertNotificationControlPoint() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlertNotificationControlPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn AlertStatus() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlertStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GapAppearance() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GapAppearance)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BootKeyboardInputReport() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BootKeyboardInputReport)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BootKeyboardOutputReport() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BootKeyboardOutputReport)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BootMouseInputReport() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BootMouseInputReport)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CurrentTime() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CyclingPowerControlPoint() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CyclingPowerControlPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CyclingPowerFeature() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CyclingPowerFeature)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CyclingPowerMeasurement() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CyclingPowerMeasurement)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CyclingPowerVector() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CyclingPowerVector)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DateTime() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DateTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DayDateTime() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DayDateTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DayOfWeek() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DayOfWeek)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GapDeviceName() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GapDeviceName)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DstOffset() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DstOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ExactTime256() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExactTime256)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn FirmwareRevisionString() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirmwareRevisionString)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn HardwareRevisionString() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareRevisionString)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn HidControlPoint() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HidControlPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn HidInformation() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HidInformation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Ieee1107320601RegulatoryCertificationDataList() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ieee1107320601RegulatoryCertificationDataList)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LnControlPoint() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LnControlPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LnFeature() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LnFeature)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LocalTimeInformation() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalTimeInformation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LocationAndSpeed() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocationAndSpeed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ManufacturerNameString() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManufacturerNameString)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ModelNumberString() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModelNumberString)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Navigation() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Navigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn NewAlert() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewAlert)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GapPeripheralPreferredConnectionParameters() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GapPeripheralPreferredConnectionParameters)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GapPeripheralPrivacyFlag() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GapPeripheralPrivacyFlag)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PnpId() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PnpId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PositionQuality() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionQuality)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ProtocolMode() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtocolMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GapReconnectionAddress() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GapReconnectionAddress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ReferenceTimeInformation() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReferenceTimeInformation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Report() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Report)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ReportMap() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportMap)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RingerControlPoint() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RingerControlPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RingerSetting() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RingerSetting)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ScanIntervalWindow() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScanIntervalWindow)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ScanRefresh() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScanRefresh)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SerialNumberString() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SerialNumberString)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GattServiceChanged() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GattServiceChanged)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SoftwareRevisionString() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SoftwareRevisionString)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SupportedNewAlertCategory() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedNewAlertCategory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SupportUnreadAlertCategory() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportUnreadAlertCategory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SystemId() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TimeAccuracy() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeAccuracy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TimeSource() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeSource)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TimeUpdateControlPoint() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeUpdateControlPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TimeUpdateState() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeUpdateState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TimeWithDst() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeWithDst)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TimeZone() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeZone)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TxPowerLevel() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TxPowerLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UnreadAlertStatus() -> windows_core::Result<windows_core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnreadAlertStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattCharacteristicUuidsStatics<R, F: FnOnce(&IGattCharacteristicUuidsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattCharacteristicUuids, IGattCharacteristicUuidsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGattCharacteristicUuidsStatics2<R, F: FnOnce(&IGattCharacteristicUuidsStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattCharacteristicUuids, IGattCharacteristicUuidsStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for GattCharacteristicUuids {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattCharacteristicUuids";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattCharacteristicsResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattCharacteristicsResult, windows_core::IUnknown, windows_core::IInspectable);
impl GattCharacteristicsResult {
    pub fn Status(&self) -> windows_core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProtocolError(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtocolError)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Characteristics(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Characteristics)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GattCharacteristicsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattCharacteristicsResult>();
}
unsafe impl windows_core::Interface for GattCharacteristicsResult {
    type Vtable = IGattCharacteristicsResult_Vtbl;
    const IID: windows_core::GUID = <IGattCharacteristicsResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattCharacteristicsResult";
}
unsafe impl Send for GattCharacteristicsResult {}
unsafe impl Sync for GattCharacteristicsResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattClientNotificationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattClientNotificationResult, windows_core::IUnknown, windows_core::IInspectable);
impl GattClientNotificationResult {
    pub fn SubscribedClient(&self) -> windows_core::Result<GattSubscribedClient> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubscribedClient)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProtocolError(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtocolError)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BytesSent(&self) -> windows_core::Result<u16> {
        let this = &windows_core::Interface::cast::<IGattClientNotificationResult2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BytesSent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GattClientNotificationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattClientNotificationResult>();
}
unsafe impl windows_core::Interface for GattClientNotificationResult {
    type Vtable = IGattClientNotificationResult_Vtbl;
    const IID: windows_core::GUID = <IGattClientNotificationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattClientNotificationResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattClientNotificationResult";
}
unsafe impl Send for GattClientNotificationResult {}
unsafe impl Sync for GattClientNotificationResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattDescriptor, windows_core::IUnknown, windows_core::IInspectable);
impl GattDescriptor {
    pub fn ProtectionLevel(&self) -> windows_core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProtectionLevel(&self, value: GattProtectionLevel) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProtectionLevel)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Uuid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uuid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AttributeHandle(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttributeHandle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadValueAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadValueAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReadValueWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadValueWithCacheModeAsync)(windows_core::Interface::as_raw(this), cachemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteValueAsync<P0>(&self, value: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteValueAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteValueWithResultAsync<P0>(&self, value: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IGattDescriptor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteValueWithResultAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ConvertShortIdToUuid(shortid: u16) -> windows_core::Result<windows_core::GUID> {
        Self::IGattDescriptorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConvertShortIdToUuid)(windows_core::Interface::as_raw(this), shortid, &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattDescriptorStatics<R, F: FnOnce(&IGattDescriptorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattDescriptor, IGattDescriptorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GattDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattDescriptor>();
}
unsafe impl windows_core::Interface for GattDescriptor {
    type Vtable = IGattDescriptor_Vtbl;
    const IID: windows_core::GUID = <IGattDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattDescriptor {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattDescriptor";
}
unsafe impl Send for GattDescriptor {}
unsafe impl Sync for GattDescriptor {}
pub struct GattDescriptorUuids;
impl GattDescriptorUuids {
    pub fn CharacteristicAggregateFormat() -> windows_core::Result<windows_core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacteristicAggregateFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CharacteristicExtendedProperties() -> windows_core::Result<windows_core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacteristicExtendedProperties)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CharacteristicPresentationFormat() -> windows_core::Result<windows_core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacteristicPresentationFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CharacteristicUserDescription() -> windows_core::Result<windows_core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacteristicUserDescription)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ClientCharacteristicConfiguration() -> windows_core::Result<windows_core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClientCharacteristicConfiguration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ServerCharacteristicConfiguration() -> windows_core::Result<windows_core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCharacteristicConfiguration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattDescriptorUuidsStatics<R, F: FnOnce(&IGattDescriptorUuidsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattDescriptorUuids, IGattDescriptorUuidsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for GattDescriptorUuids {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattDescriptorUuids";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattDescriptorsResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattDescriptorsResult, windows_core::IUnknown, windows_core::IInspectable);
impl GattDescriptorsResult {
    pub fn Status(&self) -> windows_core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProtocolError(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtocolError)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Descriptors(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Descriptors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GattDescriptorsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattDescriptorsResult>();
}
unsafe impl windows_core::Interface for GattDescriptorsResult {
    type Vtable = IGattDescriptorsResult_Vtbl;
    const IID: windows_core::GUID = <IGattDescriptorsResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattDescriptorsResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattDescriptorsResult";
}
unsafe impl Send for GattDescriptorsResult {}
unsafe impl Sync for GattDescriptorsResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattDeviceService(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattDeviceService, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(GattDeviceService, super::super::super::Foundation::IClosable);
impl GattDeviceService {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetCharacteristics(&self, characteristicuuid: windows_core::GUID) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCharacteristics)(windows_core::Interface::as_raw(this), characteristicuuid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetIncludedServices(&self, serviceuuid: windows_core::GUID) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIncludedServices)(windows_core::Interface::as_raw(this), serviceuuid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Uuid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uuid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AttributeHandle(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttributeHandle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Device(&self) -> windows_core::Result<super::BluetoothLEDevice> {
        let this = &windows_core::Interface::cast::<IGattDeviceService2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Device)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn ParentServices(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>> {
        let this = &windows_core::Interface::cast::<IGattDeviceService2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentServices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetAllCharacteristics(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>> {
        let this = &windows_core::Interface::cast::<IGattDeviceService2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAllCharacteristics)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetAllIncludedServices(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>> {
        let this = &windows_core::Interface::cast::<IGattDeviceService2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAllIncludedServices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceAccessInformation(&self) -> windows_core::Result<super::super::Enumeration::DeviceAccessInformation> {
        let this = &windows_core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceAccessInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Session(&self) -> windows_core::Result<GattSession> {
        let this = &windows_core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Session)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SharingMode(&self) -> windows_core::Result<GattSharingMode> {
        let this = &windows_core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SharingMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn RequestAccessAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>> {
        let this = &windows_core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OpenAsync(&self, sharingmode: GattSharingMode) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattOpenStatus>> {
        let this = &windows_core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenAsync)(windows_core::Interface::as_raw(this), sharingmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCharacteristicsAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>> {
        let this = &windows_core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCharacteristicsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCharacteristicsWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>> {
        let this = &windows_core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCharacteristicsWithCacheModeAsync)(windows_core::Interface::as_raw(this), cachemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCharacteristicsForUuidAsync(&self, characteristicuuid: windows_core::GUID) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>> {
        let this = &windows_core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCharacteristicsForUuidAsync)(windows_core::Interface::as_raw(this), characteristicuuid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCharacteristicsForUuidWithCacheModeAsync(&self, characteristicuuid: windows_core::GUID, cachemode: super::BluetoothCacheMode) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>> {
        let this = &windows_core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCharacteristicsForUuidWithCacheModeAsync)(windows_core::Interface::as_raw(this), characteristicuuid, cachemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIncludedServicesAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>> {
        let this = &windows_core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIncludedServicesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIncludedServicesWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>> {
        let this = &windows_core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIncludedServicesWithCacheModeAsync)(windows_core::Interface::as_raw(this), cachemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIncludedServicesForUuidAsync(&self, serviceuuid: windows_core::GUID) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>> {
        let this = &windows_core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIncludedServicesForUuidAsync)(windows_core::Interface::as_raw(this), serviceuuid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIncludedServicesForUuidWithCacheModeAsync(&self, serviceuuid: windows_core::GUID, cachemode: super::BluetoothCacheMode) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>> {
        let this = &windows_core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIncludedServicesForUuidWithCacheModeAsync)(windows_core::Interface::as_raw(this), serviceuuid, cachemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceService>> {
        Self::IGattDeviceServiceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorFromUuid(serviceuuid: windows_core::GUID) -> windows_core::Result<windows_core::HSTRING> {
        Self::IGattDeviceServiceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromUuid)(windows_core::Interface::as_raw(this), serviceuuid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn GetDeviceSelectorFromShortId(serviceshortid: u16) -> windows_core::Result<windows_core::HSTRING> {
        Self::IGattDeviceServiceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromShortId)(windows_core::Interface::as_raw(this), serviceshortid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn ConvertShortIdToUuid(shortid: u16) -> windows_core::Result<windows_core::GUID> {
        Self::IGattDeviceServiceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConvertShortIdToUuid)(windows_core::Interface::as_raw(this), shortid, &mut result__).map(|| result__)
        })
    }
    pub fn FromIdWithSharingModeAsync(deviceid: &windows_core::HSTRING, sharingmode: GattSharingMode) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceService>> {
        Self::IGattDeviceServiceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdWithSharingModeAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), sharingmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorForBluetoothDeviceId<P0>(bluetoothdeviceid: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::BluetoothDeviceId>,
    {
        Self::IGattDeviceServiceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorForBluetoothDeviceId)(windows_core::Interface::as_raw(this), bluetoothdeviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorForBluetoothDeviceIdWithCacheMode<P0>(bluetoothdeviceid: P0, cachemode: super::BluetoothCacheMode) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::BluetoothDeviceId>,
    {
        Self::IGattDeviceServiceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorForBluetoothDeviceIdWithCacheMode)(windows_core::Interface::as_raw(this), bluetoothdeviceid.param().abi(), cachemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorForBluetoothDeviceIdAndUuid<P0>(bluetoothdeviceid: P0, serviceuuid: windows_core::GUID) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::BluetoothDeviceId>,
    {
        Self::IGattDeviceServiceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorForBluetoothDeviceIdAndUuid)(windows_core::Interface::as_raw(this), bluetoothdeviceid.param().abi(), serviceuuid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorForBluetoothDeviceIdAndUuidWithCacheMode<P0>(bluetoothdeviceid: P0, serviceuuid: windows_core::GUID, cachemode: super::BluetoothCacheMode) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::BluetoothDeviceId>,
    {
        Self::IGattDeviceServiceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorForBluetoothDeviceIdAndUuidWithCacheMode)(windows_core::Interface::as_raw(this), bluetoothdeviceid.param().abi(), serviceuuid, cachemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGattDeviceServiceStatics<R, F: FnOnce(&IGattDeviceServiceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattDeviceService, IGattDeviceServiceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGattDeviceServiceStatics2<R, F: FnOnce(&IGattDeviceServiceStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattDeviceService, IGattDeviceServiceStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GattDeviceService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattDeviceService>();
}
unsafe impl windows_core::Interface for GattDeviceService {
    type Vtable = IGattDeviceService_Vtbl;
    const IID: windows_core::GUID = <IGattDeviceService as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattDeviceService {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattDeviceService";
}
unsafe impl Send for GattDeviceService {}
unsafe impl Sync for GattDeviceService {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattDeviceServicesResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattDeviceServicesResult, windows_core::IUnknown, windows_core::IInspectable);
impl GattDeviceServicesResult {
    pub fn Status(&self) -> windows_core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProtocolError(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtocolError)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Services(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Services)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GattDeviceServicesResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattDeviceServicesResult>();
}
unsafe impl windows_core::Interface for GattDeviceServicesResult {
    type Vtable = IGattDeviceServicesResult_Vtbl;
    const IID: windows_core::GUID = <IGattDeviceServicesResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattDeviceServicesResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattDeviceServicesResult";
}
unsafe impl Send for GattDeviceServicesResult {}
unsafe impl Sync for GattDeviceServicesResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattLocalCharacteristic(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattLocalCharacteristic, windows_core::IUnknown, windows_core::IInspectable);
impl GattLocalCharacteristic {
    pub fn Uuid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uuid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn StaticValue(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StaticValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CharacteristicProperties(&self) -> windows_core::Result<GattCharacteristicProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacteristicProperties)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadProtectionLevel(&self) -> windows_core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn WriteProtectionLevel(&self) -> windows_core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateDescriptorAsync<P0>(&self, descriptoruuid: windows_core::GUID, parameters: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattLocalDescriptorResult>>
    where
        P0: windows_core::Param<GattLocalDescriptorParameters>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDescriptorAsync)(windows_core::Interface::as_raw(this), descriptoruuid, parameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Descriptors(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GattLocalDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Descriptors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UserDescription(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserDescription)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PresentationFormats(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GattPresentationFormat>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PresentationFormats)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SubscribedClients(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GattSubscribedClient>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubscribedClients)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SubscribedClientsChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubscribedClientsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSubscribedClientsChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSubscribedClientsChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ReadRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, GattReadRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn WriteRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, GattWriteRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveWriteRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveWriteRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn NotifyValueAsync<P0>(&self, value: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GattClientNotificationResult>>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NotifyValueAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn NotifyValueForSubscribedClientAsync<P0, P1>(&self, value: P0, subscribedclient: P1) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattClientNotificationResult>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
        P1: windows_core::Param<GattSubscribedClient>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NotifyValueForSubscribedClientAsync)(windows_core::Interface::as_raw(this), value.param().abi(), subscribedclient.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GattLocalCharacteristic {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattLocalCharacteristic>();
}
unsafe impl windows_core::Interface for GattLocalCharacteristic {
    type Vtable = IGattLocalCharacteristic_Vtbl;
    const IID: windows_core::GUID = <IGattLocalCharacteristic as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattLocalCharacteristic {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalCharacteristic";
}
unsafe impl Send for GattLocalCharacteristic {}
unsafe impl Sync for GattLocalCharacteristic {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattLocalCharacteristicParameters(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattLocalCharacteristicParameters, windows_core::IUnknown, windows_core::IInspectable);
impl GattLocalCharacteristicParameters {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattLocalCharacteristicParameters, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStaticValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStaticValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn StaticValue(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StaticValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCharacteristicProperties(&self, value: GattCharacteristicProperties) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCharacteristicProperties)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharacteristicProperties(&self) -> windows_core::Result<GattCharacteristicProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacteristicProperties)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReadProtectionLevel(&self, value: GattProtectionLevel) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReadProtectionLevel)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReadProtectionLevel(&self) -> windows_core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWriteProtectionLevel(&self, value: GattProtectionLevel) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWriteProtectionLevel)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteProtectionLevel(&self) -> windows_core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUserDescription(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUserDescription)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn UserDescription(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserDescription)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PresentationFormats(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<GattPresentationFormat>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PresentationFormats)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GattLocalCharacteristicParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattLocalCharacteristicParameters>();
}
unsafe impl windows_core::Interface for GattLocalCharacteristicParameters {
    type Vtable = IGattLocalCharacteristicParameters_Vtbl;
    const IID: windows_core::GUID = <IGattLocalCharacteristicParameters as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattLocalCharacteristicParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalCharacteristicParameters";
}
unsafe impl Send for GattLocalCharacteristicParameters {}
unsafe impl Sync for GattLocalCharacteristicParameters {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattLocalCharacteristicResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattLocalCharacteristicResult, windows_core::IUnknown, windows_core::IInspectable);
impl GattLocalCharacteristicResult {
    pub fn Characteristic(&self) -> windows_core::Result<GattLocalCharacteristic> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Characteristic)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Error(&self) -> windows_core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GattLocalCharacteristicResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattLocalCharacteristicResult>();
}
unsafe impl windows_core::Interface for GattLocalCharacteristicResult {
    type Vtable = IGattLocalCharacteristicResult_Vtbl;
    const IID: windows_core::GUID = <IGattLocalCharacteristicResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattLocalCharacteristicResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalCharacteristicResult";
}
unsafe impl Send for GattLocalCharacteristicResult {}
unsafe impl Sync for GattLocalCharacteristicResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattLocalDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattLocalDescriptor, windows_core::IUnknown, windows_core::IInspectable);
impl GattLocalDescriptor {
    pub fn Uuid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uuid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn StaticValue(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StaticValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReadProtectionLevel(&self) -> windows_core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn WriteProtectionLevel(&self) -> windows_core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GattLocalDescriptor, GattReadRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn WriteRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GattLocalDescriptor, GattWriteRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveWriteRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveWriteRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for GattLocalDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattLocalDescriptor>();
}
unsafe impl windows_core::Interface for GattLocalDescriptor {
    type Vtable = IGattLocalDescriptor_Vtbl;
    const IID: windows_core::GUID = <IGattLocalDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattLocalDescriptor {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalDescriptor";
}
unsafe impl Send for GattLocalDescriptor {}
unsafe impl Sync for GattLocalDescriptor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattLocalDescriptorParameters(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattLocalDescriptorParameters, windows_core::IUnknown, windows_core::IInspectable);
impl GattLocalDescriptorParameters {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattLocalDescriptorParameters, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStaticValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStaticValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn StaticValue(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StaticValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetReadProtectionLevel(&self, value: GattProtectionLevel) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReadProtectionLevel)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReadProtectionLevel(&self) -> windows_core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWriteProtectionLevel(&self, value: GattProtectionLevel) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWriteProtectionLevel)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteProtectionLevel(&self) -> windows_core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GattLocalDescriptorParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattLocalDescriptorParameters>();
}
unsafe impl windows_core::Interface for GattLocalDescriptorParameters {
    type Vtable = IGattLocalDescriptorParameters_Vtbl;
    const IID: windows_core::GUID = <IGattLocalDescriptorParameters as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattLocalDescriptorParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalDescriptorParameters";
}
unsafe impl Send for GattLocalDescriptorParameters {}
unsafe impl Sync for GattLocalDescriptorParameters {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattLocalDescriptorResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattLocalDescriptorResult, windows_core::IUnknown, windows_core::IInspectable);
impl GattLocalDescriptorResult {
    pub fn Descriptor(&self) -> windows_core::Result<GattLocalDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Descriptor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Error(&self) -> windows_core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GattLocalDescriptorResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattLocalDescriptorResult>();
}
unsafe impl windows_core::Interface for GattLocalDescriptorResult {
    type Vtable = IGattLocalDescriptorResult_Vtbl;
    const IID: windows_core::GUID = <IGattLocalDescriptorResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattLocalDescriptorResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalDescriptorResult";
}
unsafe impl Send for GattLocalDescriptorResult {}
unsafe impl Sync for GattLocalDescriptorResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattLocalService(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattLocalService, windows_core::IUnknown, windows_core::IInspectable);
impl GattLocalService {
    pub fn Uuid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uuid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateCharacteristicAsync<P0>(&self, characteristicuuid: windows_core::GUID, parameters: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattLocalCharacteristicResult>>
    where
        P0: windows_core::Param<GattLocalCharacteristicParameters>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCharacteristicAsync)(windows_core::Interface::as_raw(this), characteristicuuid, parameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Characteristics(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GattLocalCharacteristic>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Characteristics)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GattLocalService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattLocalService>();
}
unsafe impl windows_core::Interface for GattLocalService {
    type Vtable = IGattLocalService_Vtbl;
    const IID: windows_core::GUID = <IGattLocalService as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattLocalService {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalService";
}
unsafe impl Send for GattLocalService {}
unsafe impl Sync for GattLocalService {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattPresentationFormat(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattPresentationFormat, windows_core::IUnknown, windows_core::IInspectable);
impl GattPresentationFormat {
    pub fn FormatType(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Exponent(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Exponent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Unit(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Namespace(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Namespace)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Description(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BluetoothSigAssignedNumbers() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BluetoothSigAssignedNumbers)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn FromParts(formattype: u8, exponent: i32, unit: u16, namespaceid: u8, description: u16) -> windows_core::Result<GattPresentationFormat> {
        Self::IGattPresentationFormatStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromParts)(windows_core::Interface::as_raw(this), formattype, exponent, unit, namespaceid, description, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGattPresentationFormatStatics<R, F: FnOnce(&IGattPresentationFormatStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattPresentationFormat, IGattPresentationFormatStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGattPresentationFormatStatics2<R, F: FnOnce(&IGattPresentationFormatStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattPresentationFormat, IGattPresentationFormatStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GattPresentationFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattPresentationFormat>();
}
unsafe impl windows_core::Interface for GattPresentationFormat {
    type Vtable = IGattPresentationFormat_Vtbl;
    const IID: windows_core::GUID = <IGattPresentationFormat as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattPresentationFormat {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattPresentationFormat";
}
unsafe impl Send for GattPresentationFormat {}
unsafe impl Sync for GattPresentationFormat {}
pub struct GattPresentationFormatTypes;
impl GattPresentationFormatTypes {
    pub fn Boolean() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Boolean)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Bit2() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bit2)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Nibble() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Nibble)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UInt8() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UInt8)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UInt12() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UInt12)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UInt16() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UInt16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UInt24() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UInt24)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UInt32() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UInt32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UInt48() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UInt48)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UInt64() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UInt64)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UInt128() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UInt128)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SInt8() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SInt8)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SInt12() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SInt12)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SInt16() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SInt16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SInt24() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SInt24)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SInt32() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SInt32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SInt48() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SInt48)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SInt64() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SInt64)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SInt128() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SInt128)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Float32() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Float32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Float64() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Float64)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SFloat() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SFloat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Float() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Float)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DUInt16() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DUInt16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Utf8() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Utf8)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Utf16() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Utf16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Struct() -> windows_core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Struct)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattPresentationFormatTypesStatics<R, F: FnOnce(&IGattPresentationFormatTypesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattPresentationFormatTypes, IGattPresentationFormatTypesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for GattPresentationFormatTypes {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattPresentationFormatTypes";
}
pub struct GattProtocolError;
impl GattProtocolError {
    pub fn InvalidHandle() -> windows_core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvalidHandle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ReadNotPermitted() -> windows_core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadNotPermitted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn WriteNotPermitted() -> windows_core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteNotPermitted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn InvalidPdu() -> windows_core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvalidPdu)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn InsufficientAuthentication() -> windows_core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsufficientAuthentication)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RequestNotSupported() -> windows_core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestNotSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn InvalidOffset() -> windows_core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvalidOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn InsufficientAuthorization() -> windows_core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsufficientAuthorization)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PrepareQueueFull() -> windows_core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrepareQueueFull)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn AttributeNotFound() -> windows_core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttributeNotFound)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn AttributeNotLong() -> windows_core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttributeNotLong)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn InsufficientEncryptionKeySize() -> windows_core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsufficientEncryptionKeySize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn InvalidAttributeValueLength() -> windows_core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvalidAttributeValueLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UnlikelyError() -> windows_core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnlikelyError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn InsufficientEncryption() -> windows_core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsufficientEncryption)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UnsupportedGroupType() -> windows_core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnsupportedGroupType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn InsufficientResources() -> windows_core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsufficientResources)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattProtocolErrorStatics<R, F: FnOnce(&IGattProtocolErrorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattProtocolError, IGattProtocolErrorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for GattProtocolError {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattProtocolError";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattReadClientCharacteristicConfigurationDescriptorResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattReadClientCharacteristicConfigurationDescriptorResult, windows_core::IUnknown, windows_core::IInspectable);
impl GattReadClientCharacteristicConfigurationDescriptorResult {
    pub fn Status(&self) -> windows_core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ClientCharacteristicConfigurationDescriptor(&self) -> windows_core::Result<GattClientCharacteristicConfigurationDescriptorValue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClientCharacteristicConfigurationDescriptor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProtocolError(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = &windows_core::Interface::cast::<IGattReadClientCharacteristicConfigurationDescriptorResult2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtocolError)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GattReadClientCharacteristicConfigurationDescriptorResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattReadClientCharacteristicConfigurationDescriptorResult>();
}
unsafe impl windows_core::Interface for GattReadClientCharacteristicConfigurationDescriptorResult {
    type Vtable = IGattReadClientCharacteristicConfigurationDescriptorResult_Vtbl;
    const IID: windows_core::GUID = <IGattReadClientCharacteristicConfigurationDescriptorResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattReadClientCharacteristicConfigurationDescriptorResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadClientCharacteristicConfigurationDescriptorResult";
}
unsafe impl Send for GattReadClientCharacteristicConfigurationDescriptorResult {}
unsafe impl Sync for GattReadClientCharacteristicConfigurationDescriptorResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattReadRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattReadRequest, windows_core::IUnknown, windows_core::IInspectable);
impl GattReadRequest {
    pub fn Offset(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Offset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Length(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn State(&self) -> windows_core::Result<GattRequestState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GattReadRequest, GattRequestStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStateChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RespondWithValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RespondWithValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RespondWithProtocolError(&self, protocolerror: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RespondWithProtocolError)(windows_core::Interface::as_raw(this), protocolerror).ok() }
    }
}
impl windows_core::RuntimeType for GattReadRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattReadRequest>();
}
unsafe impl windows_core::Interface for GattReadRequest {
    type Vtable = IGattReadRequest_Vtbl;
    const IID: windows_core::GUID = <IGattReadRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattReadRequest {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadRequest";
}
unsafe impl Send for GattReadRequest {}
unsafe impl Sync for GattReadRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattReadRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattReadRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GattReadRequestedEventArgs {
    pub fn Session(&self) -> windows_core::Result<GattSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Session)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetRequestAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattReadRequest>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRequestAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GattReadRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattReadRequestedEventArgs>();
}
unsafe impl windows_core::Interface for GattReadRequestedEventArgs {
    type Vtable = IGattReadRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGattReadRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattReadRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadRequestedEventArgs";
}
unsafe impl Send for GattReadRequestedEventArgs {}
unsafe impl Sync for GattReadRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattReadResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattReadResult, windows_core::IUnknown, windows_core::IInspectable);
impl GattReadResult {
    pub fn Status(&self) -> windows_core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
    pub fn ProtocolError(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = &windows_core::Interface::cast::<IGattReadResult2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtocolError)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GattReadResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattReadResult>();
}
unsafe impl windows_core::Interface for GattReadResult {
    type Vtable = IGattReadResult_Vtbl;
    const IID: windows_core::GUID = <IGattReadResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattReadResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadResult";
}
unsafe impl Send for GattReadResult {}
unsafe impl Sync for GattReadResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattReliableWriteTransaction(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattReliableWriteTransaction, windows_core::IUnknown, windows_core::IInspectable);
impl GattReliableWriteTransaction {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattReliableWriteTransaction, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteValue<P0, P1>(&self, characteristic: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<GattCharacteristic>,
        P1: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteValue)(windows_core::Interface::as_raw(this), characteristic.param().abi(), value.param().abi()).ok() }
    }
    pub fn CommitAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommitAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CommitWithResultAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>> {
        let this = &windows_core::Interface::cast::<IGattReliableWriteTransaction2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommitWithResultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GattReliableWriteTransaction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattReliableWriteTransaction>();
}
unsafe impl windows_core::Interface for GattReliableWriteTransaction {
    type Vtable = IGattReliableWriteTransaction_Vtbl;
    const IID: windows_core::GUID = <IGattReliableWriteTransaction as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattReliableWriteTransaction {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattReliableWriteTransaction";
}
unsafe impl Send for GattReliableWriteTransaction {}
unsafe impl Sync for GattReliableWriteTransaction {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattRequestStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattRequestStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GattRequestStateChangedEventArgs {
    pub fn State(&self) -> windows_core::Result<GattRequestState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Error(&self) -> windows_core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GattRequestStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattRequestStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for GattRequestStateChangedEventArgs {
    type Vtable = IGattRequestStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGattRequestStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattRequestStateChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattRequestStateChangedEventArgs";
}
unsafe impl Send for GattRequestStateChangedEventArgs {}
unsafe impl Sync for GattRequestStateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattServiceProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattServiceProvider, windows_core::IUnknown, windows_core::IInspectable);
impl GattServiceProvider {
    pub fn Service(&self) -> windows_core::Result<GattLocalService> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Service)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AdvertisementStatus(&self) -> windows_core::Result<GattServiceProviderAdvertisementStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdvertisementStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AdvertisementStatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GattServiceProvider, GattServiceProviderAdvertisementStatusChangedEventArgs>>,
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
    pub fn StartAdvertising(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartAdvertising)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StartAdvertisingWithParameters<P0>(&self, parameters: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<GattServiceProviderAdvertisingParameters>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartAdvertisingWithParameters)(windows_core::Interface::as_raw(this), parameters.param().abi()).ok() }
    }
    pub fn StopAdvertising(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StopAdvertising)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateAsync(serviceuuid: windows_core::GUID) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattServiceProviderResult>> {
        Self::IGattServiceProviderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAsync)(windows_core::Interface::as_raw(this), serviceuuid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGattServiceProviderStatics<R, F: FnOnce(&IGattServiceProviderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattServiceProvider, IGattServiceProviderStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GattServiceProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattServiceProvider>();
}
unsafe impl windows_core::Interface for GattServiceProvider {
    type Vtable = IGattServiceProvider_Vtbl;
    const IID: windows_core::GUID = <IGattServiceProvider as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattServiceProvider {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProvider";
}
unsafe impl Send for GattServiceProvider {}
unsafe impl Sync for GattServiceProvider {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattServiceProviderAdvertisementStatusChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattServiceProviderAdvertisementStatusChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GattServiceProviderAdvertisementStatusChangedEventArgs {
    pub fn Error(&self) -> windows_core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<GattServiceProviderAdvertisementStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GattServiceProviderAdvertisementStatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattServiceProviderAdvertisementStatusChangedEventArgs>();
}
unsafe impl windows_core::Interface for GattServiceProviderAdvertisementStatusChangedEventArgs {
    type Vtable = IGattServiceProviderAdvertisementStatusChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGattServiceProviderAdvertisementStatusChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattServiceProviderAdvertisementStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderAdvertisementStatusChangedEventArgs";
}
unsafe impl Send for GattServiceProviderAdvertisementStatusChangedEventArgs {}
unsafe impl Sync for GattServiceProviderAdvertisementStatusChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattServiceProviderAdvertisingParameters(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattServiceProviderAdvertisingParameters, windows_core::IUnknown, windows_core::IInspectable);
impl GattServiceProviderAdvertisingParameters {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattServiceProviderAdvertisingParameters, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetIsConnectable(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsConnectable)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsConnectable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsConnectable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDiscoverable(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsDiscoverable)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDiscoverable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDiscoverable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetServiceData<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IGattServiceProviderAdvertisingParameters2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetServiceData)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ServiceData(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = &windows_core::Interface::cast::<IGattServiceProviderAdvertisingParameters2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GattServiceProviderAdvertisingParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattServiceProviderAdvertisingParameters>();
}
unsafe impl windows_core::Interface for GattServiceProviderAdvertisingParameters {
    type Vtable = IGattServiceProviderAdvertisingParameters_Vtbl;
    const IID: windows_core::GUID = <IGattServiceProviderAdvertisingParameters as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattServiceProviderAdvertisingParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderAdvertisingParameters";
}
unsafe impl Send for GattServiceProviderAdvertisingParameters {}
unsafe impl Sync for GattServiceProviderAdvertisingParameters {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattServiceProviderResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattServiceProviderResult, windows_core::IUnknown, windows_core::IInspectable);
impl GattServiceProviderResult {
    pub fn Error(&self) -> windows_core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ServiceProvider(&self) -> windows_core::Result<GattServiceProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GattServiceProviderResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattServiceProviderResult>();
}
unsafe impl windows_core::Interface for GattServiceProviderResult {
    type Vtable = IGattServiceProviderResult_Vtbl;
    const IID: windows_core::GUID = <IGattServiceProviderResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattServiceProviderResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderResult";
}
unsafe impl Send for GattServiceProviderResult {}
unsafe impl Sync for GattServiceProviderResult {}
pub struct GattServiceUuids;
impl GattServiceUuids {
    pub fn Battery() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Battery)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BloodPressure() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BloodPressure)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CyclingSpeedAndCadence() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CyclingSpeedAndCadence)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GenericAccess() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GenericAccess)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GenericAttribute() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GenericAttribute)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Glucose() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Glucose)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn HealthThermometer() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HealthThermometer)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn HeartRate() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeartRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RunningSpeedAndCadence() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RunningSpeedAndCadence)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn AlertNotification() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlertNotification)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CurrentTime() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CyclingPower() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CyclingPower)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DeviceInformation() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInformation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn HumanInterfaceDevice() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HumanInterfaceDevice)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ImmediateAlert() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImmediateAlert)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LinkLoss() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LinkLoss)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LocationAndNavigation() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocationAndNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn NextDstChange() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextDstChange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PhoneAlertStatus() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneAlertStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ReferenceTimeUpdate() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReferenceTimeUpdate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ScanParameters() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScanParameters)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TxPower() -> windows_core::Result<windows_core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TxPower)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattServiceUuidsStatics<R, F: FnOnce(&IGattServiceUuidsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattServiceUuids, IGattServiceUuidsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGattServiceUuidsStatics2<R, F: FnOnce(&IGattServiceUuidsStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattServiceUuids, IGattServiceUuidsStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for GattServiceUuids {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceUuids";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattSession, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(GattSession, super::super::super::Foundation::IClosable);
impl GattSession {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DeviceId(&self) -> windows_core::Result<super::BluetoothDeviceId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanMaintainConnection(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanMaintainConnection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaintainConnection(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaintainConnection)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaintainConnection(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaintainConnection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxPduSize(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPduSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SessionStatus(&self) -> windows_core::Result<GattSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxPduSizeChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GattSession, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPduSizeChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMaxPduSizeChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMaxPduSizeChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SessionStatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GattSession, GattSessionStatusChangedEventArgs>>,
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
    pub fn FromDeviceIdAsync<P0>(deviceid: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattSession>>
    where
        P0: windows_core::Param<super::BluetoothDeviceId>,
    {
        Self::IGattSessionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromDeviceIdAsync)(windows_core::Interface::as_raw(this), deviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGattSessionStatics<R, F: FnOnce(&IGattSessionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattSession, IGattSessionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GattSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattSession>();
}
unsafe impl windows_core::Interface for GattSession {
    type Vtable = IGattSession_Vtbl;
    const IID: windows_core::GUID = <IGattSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattSession {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattSession";
}
unsafe impl Send for GattSession {}
unsafe impl Sync for GattSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattSessionStatusChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattSessionStatusChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GattSessionStatusChangedEventArgs {
    pub fn Error(&self) -> windows_core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<GattSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GattSessionStatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattSessionStatusChangedEventArgs>();
}
unsafe impl windows_core::Interface for GattSessionStatusChangedEventArgs {
    type Vtable = IGattSessionStatusChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGattSessionStatusChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattSessionStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattSessionStatusChangedEventArgs";
}
unsafe impl Send for GattSessionStatusChangedEventArgs {}
unsafe impl Sync for GattSessionStatusChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattSubscribedClient(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattSubscribedClient, windows_core::IUnknown, windows_core::IInspectable);
impl GattSubscribedClient {
    pub fn Session(&self) -> windows_core::Result<GattSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Session)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxNotificationSize(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxNotificationSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxNotificationSizeChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GattSubscribedClient, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxNotificationSizeChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMaxNotificationSizeChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMaxNotificationSizeChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for GattSubscribedClient {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattSubscribedClient>();
}
unsafe impl windows_core::Interface for GattSubscribedClient {
    type Vtable = IGattSubscribedClient_Vtbl;
    const IID: windows_core::GUID = <IGattSubscribedClient as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattSubscribedClient {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattSubscribedClient";
}
unsafe impl Send for GattSubscribedClient {}
unsafe impl Sync for GattSubscribedClient {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattValueChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattValueChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GattValueChangedEventArgs {
    #[cfg(feature = "Storage_Streams")]
    pub fn CharacteristicValue(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacteristicValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GattValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattValueChangedEventArgs>();
}
unsafe impl windows_core::Interface for GattValueChangedEventArgs {
    type Vtable = IGattValueChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGattValueChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattValueChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattValueChangedEventArgs";
}
unsafe impl Send for GattValueChangedEventArgs {}
unsafe impl Sync for GattValueChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattWriteRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattWriteRequest, windows_core::IUnknown, windows_core::IInspectable);
impl GattWriteRequest {
    #[cfg(feature = "Storage_Streams")]
    pub fn Value(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Offset(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Offset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Option(&self) -> windows_core::Result<GattWriteOption> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Option)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn State(&self) -> windows_core::Result<GattRequestState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GattWriteRequest, GattRequestStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStateChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Respond(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Respond)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RespondWithProtocolError(&self, protocolerror: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RespondWithProtocolError)(windows_core::Interface::as_raw(this), protocolerror).ok() }
    }
}
impl windows_core::RuntimeType for GattWriteRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattWriteRequest>();
}
unsafe impl windows_core::Interface for GattWriteRequest {
    type Vtable = IGattWriteRequest_Vtbl;
    const IID: windows_core::GUID = <IGattWriteRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattWriteRequest {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteRequest";
}
unsafe impl Send for GattWriteRequest {}
unsafe impl Sync for GattWriteRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattWriteRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattWriteRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GattWriteRequestedEventArgs {
    pub fn Session(&self) -> windows_core::Result<GattSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Session)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetRequestAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteRequest>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRequestAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GattWriteRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattWriteRequestedEventArgs>();
}
unsafe impl windows_core::Interface for GattWriteRequestedEventArgs {
    type Vtable = IGattWriteRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGattWriteRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattWriteRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteRequestedEventArgs";
}
unsafe impl Send for GattWriteRequestedEventArgs {}
unsafe impl Sync for GattWriteRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GattWriteResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattWriteResult, windows_core::IUnknown, windows_core::IInspectable);
impl GattWriteResult {
    pub fn Status(&self) -> windows_core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProtocolError(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtocolError)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GattWriteResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattWriteResult>();
}
unsafe impl windows_core::Interface for GattWriteResult {
    type Vtable = IGattWriteResult_Vtbl;
    const IID: windows_core::GUID = <IGattWriteResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattWriteResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteResult";
}
unsafe impl Send for GattWriteResult {}
unsafe impl Sync for GattWriteResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GattCharacteristicProperties(pub u32);
impl GattCharacteristicProperties {
    pub const None: Self = Self(0u32);
    pub const Broadcast: Self = Self(1u32);
    pub const Read: Self = Self(2u32);
    pub const WriteWithoutResponse: Self = Self(4u32);
    pub const Write: Self = Self(8u32);
    pub const Notify: Self = Self(16u32);
    pub const Indicate: Self = Self(32u32);
    pub const AuthenticatedSignedWrites: Self = Self(64u32);
    pub const ExtendedProperties: Self = Self(128u32);
    pub const ReliableWrites: Self = Self(256u32);
    pub const WritableAuxiliaries: Self = Self(512u32);
}
impl windows_core::TypeKind for GattCharacteristicProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GattCharacteristicProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GattCharacteristicProperties").field(&self.0).finish()
    }
}
impl GattCharacteristicProperties {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GattCharacteristicProperties {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GattCharacteristicProperties {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GattCharacteristicProperties {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GattCharacteristicProperties {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GattCharacteristicProperties {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for GattCharacteristicProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattCharacteristicProperties;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GattClientCharacteristicConfigurationDescriptorValue(pub i32);
impl GattClientCharacteristicConfigurationDescriptorValue {
    pub const None: Self = Self(0i32);
    pub const Notify: Self = Self(1i32);
    pub const Indicate: Self = Self(2i32);
}
impl windows_core::TypeKind for GattClientCharacteristicConfigurationDescriptorValue {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GattClientCharacteristicConfigurationDescriptorValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GattClientCharacteristicConfigurationDescriptorValue").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GattClientCharacteristicConfigurationDescriptorValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattClientCharacteristicConfigurationDescriptorValue;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GattCommunicationStatus(pub i32);
impl GattCommunicationStatus {
    pub const Success: Self = Self(0i32);
    pub const Unreachable: Self = Self(1i32);
    pub const ProtocolError: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
}
impl windows_core::TypeKind for GattCommunicationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GattCommunicationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GattCommunicationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GattCommunicationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattCommunicationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GattOpenStatus(pub i32);
impl GattOpenStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const AlreadyOpened: Self = Self(2i32);
    pub const NotFound: Self = Self(3i32);
    pub const SharingViolation: Self = Self(4i32);
    pub const AccessDenied: Self = Self(5i32);
}
impl windows_core::TypeKind for GattOpenStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GattOpenStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GattOpenStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GattOpenStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattOpenStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GattProtectionLevel(pub i32);
impl GattProtectionLevel {
    pub const Plain: Self = Self(0i32);
    pub const AuthenticationRequired: Self = Self(1i32);
    pub const EncryptionRequired: Self = Self(2i32);
    pub const EncryptionAndAuthenticationRequired: Self = Self(3i32);
}
impl windows_core::TypeKind for GattProtectionLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GattProtectionLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GattProtectionLevel").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GattProtectionLevel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattProtectionLevel;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GattRequestState(pub i32);
impl GattRequestState {
    pub const Pending: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl windows_core::TypeKind for GattRequestState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GattRequestState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GattRequestState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GattRequestState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattRequestState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GattServiceProviderAdvertisementStatus(pub i32);
impl GattServiceProviderAdvertisementStatus {
    pub const Created: Self = Self(0i32);
    pub const Stopped: Self = Self(1i32);
    pub const Started: Self = Self(2i32);
    pub const Aborted: Self = Self(3i32);
    pub const StartedWithoutAllAdvertisementData: Self = Self(4i32);
}
impl windows_core::TypeKind for GattServiceProviderAdvertisementStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GattServiceProviderAdvertisementStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GattServiceProviderAdvertisementStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GattServiceProviderAdvertisementStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderAdvertisementStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GattSessionStatus(pub i32);
impl GattSessionStatus {
    pub const Closed: Self = Self(0i32);
    pub const Active: Self = Self(1i32);
}
impl windows_core::TypeKind for GattSessionStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GattSessionStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GattSessionStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GattSessionStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattSessionStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GattSharingMode(pub i32);
impl GattSharingMode {
    pub const Unspecified: Self = Self(0i32);
    pub const Exclusive: Self = Self(1i32);
    pub const SharedReadOnly: Self = Self(2i32);
    pub const SharedReadAndWrite: Self = Self(3i32);
}
impl windows_core::TypeKind for GattSharingMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GattSharingMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GattSharingMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GattSharingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattSharingMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GattWriteOption(pub i32);
impl GattWriteOption {
    pub const WriteWithResponse: Self = Self(0i32);
    pub const WriteWithoutResponse: Self = Self(1i32);
}
impl windows_core::TypeKind for GattWriteOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GattWriteOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GattWriteOption").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GattWriteOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteOption;i4)");
}
