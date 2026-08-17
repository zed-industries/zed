windows_core::imp::define_interface!(IUsbBulkInEndpointDescriptor, IUsbBulkInEndpointDescriptor_Vtbl, 0x3c6e4846_06cf_42a9_9dc2_971c1b14b6e3);
impl windows_core::RuntimeType for IUsbBulkInEndpointDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbBulkInEndpointDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EndpointNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Pipe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbBulkInPipe, IUsbBulkInPipe_Vtbl, 0xf01d2d3b_4548_4d50_b326_d82cdabe1220);
impl windows_core::RuntimeType for IUsbBulkInPipe {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbBulkInPipe_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxTransferSizeBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EndpointDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearStallAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReadOptions: unsafe extern "system" fn(*mut core::ffi::c_void, UsbReadOptions) -> windows_core::HRESULT,
    pub ReadOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UsbReadOptions) -> windows_core::HRESULT,
    pub FlushBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
}
windows_core::imp::define_interface!(IUsbBulkOutEndpointDescriptor, IUsbBulkOutEndpointDescriptor_Vtbl, 0x2820847a_ffee_4f60_9be1_956cac3ecb65);
impl windows_core::RuntimeType for IUsbBulkOutEndpointDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbBulkOutEndpointDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EndpointNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Pipe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbBulkOutPipe, IUsbBulkOutPipe_Vtbl, 0xa8e9ee6e_0115_45aa_8b21_37b225bccee7);
impl windows_core::RuntimeType for IUsbBulkOutPipe {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbBulkOutPipe_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EndpointDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearStallAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWriteOptions: unsafe extern "system" fn(*mut core::ffi::c_void, UsbWriteOptions) -> windows_core::HRESULT,
    pub WriteOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UsbWriteOptions) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
}
windows_core::imp::define_interface!(IUsbConfiguration, IUsbConfiguration_Vtbl, 0x68177429_36a9_46d7_b873_fc689251ec30);
impl windows_core::RuntimeType for IUsbConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub UsbInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UsbInterfaces: usize,
    pub ConfigurationDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Descriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Descriptors: usize,
}
windows_core::imp::define_interface!(IUsbConfigurationDescriptor, IUsbConfigurationDescriptor_Vtbl, 0xf2176d92_b442_407a_8207_7d646c0385f3);
impl windows_core::RuntimeType for IUsbConfigurationDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbConfigurationDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ConfigurationValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub MaxPowerMilliamps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SelfPowered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub RemoteWakeup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbConfigurationDescriptorStatics, IUsbConfigurationDescriptorStatics_Vtbl, 0x424ced93_e740_40a1_92bd_da120ea04914);
impl windows_core::RuntimeType for IUsbConfigurationDescriptorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbConfigurationDescriptorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbControlRequestType, IUsbControlRequestType_Vtbl, 0x8e9465a6_d73d_46de_94be_aae7f07c0f5c);
impl windows_core::RuntimeType for IUsbControlRequestType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbControlRequestType_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UsbTransferDirection) -> windows_core::HRESULT,
    pub SetDirection: unsafe extern "system" fn(*mut core::ffi::c_void, UsbTransferDirection) -> windows_core::HRESULT,
    pub ControlTransferType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UsbControlTransferType) -> windows_core::HRESULT,
    pub SetControlTransferType: unsafe extern "system" fn(*mut core::ffi::c_void, UsbControlTransferType) -> windows_core::HRESULT,
    pub Recipient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UsbControlRecipient) -> windows_core::HRESULT,
    pub SetRecipient: unsafe extern "system" fn(*mut core::ffi::c_void, UsbControlRecipient) -> windows_core::HRESULT,
    pub AsByte: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetAsByte: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbDescriptor, IUsbDescriptor_Vtbl, 0x0a89f216_5f9d_4874_8904_da9ad3f5528f);
impl windows_core::RuntimeType for IUsbDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub DescriptorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ReadDescriptorBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ReadDescriptorBuffer: usize,
}
windows_core::imp::define_interface!(IUsbDevice, IUsbDevice_Vtbl, 0x5249b992_c456_44d5_ad5e_24f5a089f63b);
impl windows_core::RuntimeType for IUsbDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SendControlOutTransferAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SendControlOutTransferAsync: usize,
    pub SendControlOutTransferAsyncNoBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SendControlInTransferAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SendControlInTransferAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SendControlInTransferAsyncNoBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SendControlInTransferAsyncNoBuffer: usize,
    pub DefaultInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbDeviceClass, IUsbDeviceClass_Vtbl, 0x051942f9_845e_47eb_b12a_38f2f617afe7);
impl windows_core::RuntimeType for IUsbDeviceClass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbDeviceClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ClassCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetClassCode: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub SubclassCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSubclassCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProtocolCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProtocolCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbDeviceClasses, IUsbDeviceClasses_Vtbl, 0x686f955d_9b92_4b30_9781_c22c55ac35cb);
impl windows_core::RuntimeType for IUsbDeviceClasses {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbDeviceClasses_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IUsbDeviceClassesStatics, IUsbDeviceClassesStatics_Vtbl, 0xb20b0527_c580_4599_a165_981b4fd03230);
impl windows_core::RuntimeType for IUsbDeviceClassesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbDeviceClassesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CdcControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Physical: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PersonalHealthcare: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActiveSync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PalmSync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceFirmwareUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Irda: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Measurement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VendorSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbDeviceDescriptor, IUsbDeviceDescriptor_Vtbl, 0x1f48d1f6_ba97_4322_b92c_b5b189216588);
impl windows_core::RuntimeType for IUsbDeviceDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbDeviceDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BcdUsb: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxPacketSize0: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub VendorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ProductId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub BcdDeviceRevision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub NumberOfConfigurations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbDeviceStatics, IUsbDeviceStatics_Vtbl, 0x066b85a2_09b7_4446_8502_6fe6dcaa7309);
impl windows_core::RuntimeType for IUsbDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::GUID, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeviceSelectorGuidOnly: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeviceSelectorVidPidOnly: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeviceClassSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbEndpointDescriptor, IUsbEndpointDescriptor_Vtbl, 0x6b4862d9_8df7_4b40_ac83_578f139f0575);
impl windows_core::RuntimeType for IUsbEndpointDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbEndpointDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EndpointNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UsbTransferDirection) -> windows_core::HRESULT,
    pub EndpointType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UsbEndpointType) -> windows_core::HRESULT,
    pub AsBulkInEndpointDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AsInterruptInEndpointDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AsBulkOutEndpointDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AsInterruptOutEndpointDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbEndpointDescriptorStatics, IUsbEndpointDescriptorStatics_Vtbl, 0xc890b201_9a6a_495e_a82c_295b9e708106);
impl windows_core::RuntimeType for IUsbEndpointDescriptorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbEndpointDescriptorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbInterface, IUsbInterface_Vtbl, 0xa0322b95_7f47_48ab_a727_678c25be2112);
impl windows_core::RuntimeType for IUsbInterface {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbInterface_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub BulkInPipes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BulkInPipes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InterruptInPipes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InterruptInPipes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub BulkOutPipes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BulkOutPipes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InterruptOutPipes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InterruptOutPipes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InterfaceSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InterfaceSettings: usize,
    pub InterfaceNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Descriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Descriptors: usize,
}
windows_core::imp::define_interface!(IUsbInterfaceDescriptor, IUsbInterfaceDescriptor_Vtbl, 0x199670c7_b7ee_4f90_8cd5_94a2e257598a);
impl windows_core::RuntimeType for IUsbInterfaceDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbInterfaceDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ClassCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SubclassCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub ProtocolCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub AlternateSettingNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub InterfaceNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbInterfaceDescriptorStatics, IUsbInterfaceDescriptorStatics_Vtbl, 0xe34a9ff5_77d6_48b6_b0be_16c6422316fe);
impl windows_core::RuntimeType for IUsbInterfaceDescriptorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbInterfaceDescriptorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbInterfaceSetting, IUsbInterfaceSetting_Vtbl, 0x1827bba7_8da7_4af7_8f4c_7f3032e781f5);
impl windows_core::RuntimeType for IUsbInterfaceSetting {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbInterfaceSetting_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub BulkInEndpoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BulkInEndpoints: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InterruptInEndpoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InterruptInEndpoints: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub BulkOutEndpoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BulkOutEndpoints: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InterruptOutEndpoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InterruptOutEndpoints: usize,
    pub Selected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SelectSettingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InterfaceDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Descriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Descriptors: usize,
}
windows_core::imp::define_interface!(IUsbInterruptInEndpointDescriptor, IUsbInterruptInEndpointDescriptor_Vtbl, 0xc0528967_c911_4c3a_86b2_419c2da89039);
impl windows_core::RuntimeType for IUsbInterruptInEndpointDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbInterruptInEndpointDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EndpointNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Interval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Pipe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbInterruptInEventArgs, IUsbInterruptInEventArgs_Vtbl, 0xb7b04092_1418_4936_8209_299cf5605583);
impl windows_core::RuntimeType for IUsbInterruptInEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbInterruptInEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub InterruptData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InterruptData: usize,
}
windows_core::imp::define_interface!(IUsbInterruptInPipe, IUsbInterruptInPipe_Vtbl, 0xfa007116_84d7_48c7_8a3f_4c0b235f2ea6);
impl windows_core::RuntimeType for IUsbInterruptInPipe {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbInterruptInPipe_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EndpointDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearStallAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbInterruptOutEndpointDescriptor, IUsbInterruptOutEndpointDescriptor_Vtbl, 0xcc9fed81_10ca_4533_952d_9e278341e80f);
impl windows_core::RuntimeType for IUsbInterruptOutEndpointDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbInterruptOutEndpointDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EndpointNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Interval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Pipe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbInterruptOutPipe, IUsbInterruptOutPipe_Vtbl, 0xe984c8a9_aaf9_49d0_b96c_f661ab4a7f95);
impl windows_core::RuntimeType for IUsbInterruptOutPipe {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbInterruptOutPipe_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EndpointDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearStallAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWriteOptions: unsafe extern "system" fn(*mut core::ffi::c_void, UsbWriteOptions) -> windows_core::HRESULT,
    pub WriteOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UsbWriteOptions) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
}
windows_core::imp::define_interface!(IUsbSetupPacket, IUsbSetupPacket_Vtbl, 0x104ba132_c78f_4c51_b654_e49d02f2cb03);
impl windows_core::RuntimeType for IUsbSetupPacket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbSetupPacket_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRequestType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetRequest: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUsbSetupPacketFactory, IUsbSetupPacketFactory_Vtbl, 0xc9257d50_1b2e_4a41_a2a7_338f0cef3c14);
impl windows_core::RuntimeType for IUsbSetupPacketFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUsbSetupPacketFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateWithEightByteBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateWithEightByteBuffer: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbBulkInEndpointDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbBulkInEndpointDescriptor, windows_core::IUnknown, windows_core::IInspectable);
impl UsbBulkInEndpointDescriptor {
    pub fn MaxPacketSize(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPacketSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EndpointNumber(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndpointNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Pipe(&self) -> windows_core::Result<UsbBulkInPipe> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pipe)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UsbBulkInEndpointDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbBulkInEndpointDescriptor>();
}
unsafe impl windows_core::Interface for UsbBulkInEndpointDescriptor {
    type Vtable = IUsbBulkInEndpointDescriptor_Vtbl;
    const IID: windows_core::GUID = <IUsbBulkInEndpointDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbBulkInEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbBulkInEndpointDescriptor";
}
unsafe impl Send for UsbBulkInEndpointDescriptor {}
unsafe impl Sync for UsbBulkInEndpointDescriptor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbBulkInPipe(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbBulkInPipe, windows_core::IUnknown, windows_core::IInspectable);
impl UsbBulkInPipe {
    pub fn MaxTransferSizeBytes(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxTransferSizeBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EndpointDescriptor(&self) -> windows_core::Result<UsbBulkInEndpointDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndpointDescriptor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClearStallAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClearStallAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetReadOptions(&self, value: UsbReadOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReadOptions)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReadOptions(&self) -> windows_core::Result<UsbReadOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadOptions)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FlushBuffer(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).FlushBuffer)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UsbBulkInPipe {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbBulkInPipe>();
}
unsafe impl windows_core::Interface for UsbBulkInPipe {
    type Vtable = IUsbBulkInPipe_Vtbl;
    const IID: windows_core::GUID = <IUsbBulkInPipe as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbBulkInPipe {
    const NAME: &'static str = "Windows.Devices.Usb.UsbBulkInPipe";
}
unsafe impl Send for UsbBulkInPipe {}
unsafe impl Sync for UsbBulkInPipe {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbBulkOutEndpointDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbBulkOutEndpointDescriptor, windows_core::IUnknown, windows_core::IInspectable);
impl UsbBulkOutEndpointDescriptor {
    pub fn MaxPacketSize(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPacketSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EndpointNumber(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndpointNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Pipe(&self) -> windows_core::Result<UsbBulkOutPipe> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pipe)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UsbBulkOutEndpointDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbBulkOutEndpointDescriptor>();
}
unsafe impl windows_core::Interface for UsbBulkOutEndpointDescriptor {
    type Vtable = IUsbBulkOutEndpointDescriptor_Vtbl;
    const IID: windows_core::GUID = <IUsbBulkOutEndpointDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbBulkOutEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbBulkOutEndpointDescriptor";
}
unsafe impl Send for UsbBulkOutEndpointDescriptor {}
unsafe impl Sync for UsbBulkOutEndpointDescriptor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbBulkOutPipe(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbBulkOutPipe, windows_core::IUnknown, windows_core::IInspectable);
impl UsbBulkOutPipe {
    pub fn EndpointDescriptor(&self) -> windows_core::Result<UsbBulkOutEndpointDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndpointDescriptor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClearStallAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClearStallAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetWriteOptions(&self, value: UsbWriteOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWriteOptions)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteOptions(&self) -> windows_core::Result<UsbWriteOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteOptions)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UsbBulkOutPipe {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbBulkOutPipe>();
}
unsafe impl windows_core::Interface for UsbBulkOutPipe {
    type Vtable = IUsbBulkOutPipe_Vtbl;
    const IID: windows_core::GUID = <IUsbBulkOutPipe as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbBulkOutPipe {
    const NAME: &'static str = "Windows.Devices.Usb.UsbBulkOutPipe";
}
unsafe impl Send for UsbBulkOutPipe {}
unsafe impl Sync for UsbBulkOutPipe {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl UsbConfiguration {
    #[cfg(feature = "Foundation_Collections")]
    pub fn UsbInterfaces(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<UsbInterface>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsbInterfaces)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConfigurationDescriptor(&self) -> windows_core::Result<UsbConfigurationDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConfigurationDescriptor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Descriptors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<UsbDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Descriptors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UsbConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbConfiguration>();
}
unsafe impl windows_core::Interface for UsbConfiguration {
    type Vtable = IUsbConfiguration_Vtbl;
    const IID: windows_core::GUID = <IUsbConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbConfiguration {
    const NAME: &'static str = "Windows.Devices.Usb.UsbConfiguration";
}
unsafe impl Send for UsbConfiguration {}
unsafe impl Sync for UsbConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbConfigurationDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbConfigurationDescriptor, windows_core::IUnknown, windows_core::IInspectable);
impl UsbConfigurationDescriptor {
    pub fn ConfigurationValue(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConfigurationValue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxPowerMilliamps(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPowerMilliamps)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SelfPowered(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelfPowered)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RemoteWakeup(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteWakeup)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TryParse<P0>(descriptor: P0, parsed: &mut Option<UsbConfigurationDescriptor>) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<UsbDescriptor>,
    {
        Self::IUsbConfigurationDescriptorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), descriptor.param().abi(), parsed as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn Parse<P0>(descriptor: P0) -> windows_core::Result<UsbConfigurationDescriptor>
    where
        P0: windows_core::Param<UsbDescriptor>,
    {
        Self::IUsbConfigurationDescriptorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), descriptor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUsbConfigurationDescriptorStatics<R, F: FnOnce(&IUsbConfigurationDescriptorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UsbConfigurationDescriptor, IUsbConfigurationDescriptorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UsbConfigurationDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbConfigurationDescriptor>();
}
unsafe impl windows_core::Interface for UsbConfigurationDescriptor {
    type Vtable = IUsbConfigurationDescriptor_Vtbl;
    const IID: windows_core::GUID = <IUsbConfigurationDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbConfigurationDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbConfigurationDescriptor";
}
unsafe impl Send for UsbConfigurationDescriptor {}
unsafe impl Sync for UsbConfigurationDescriptor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbControlRequestType(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbControlRequestType, windows_core::IUnknown, windows_core::IInspectable);
impl UsbControlRequestType {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UsbControlRequestType, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Direction(&self) -> windows_core::Result<UsbTransferDirection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Direction)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDirection(&self, value: UsbTransferDirection) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDirection)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ControlTransferType(&self) -> windows_core::Result<UsbControlTransferType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ControlTransferType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetControlTransferType(&self, value: UsbControlTransferType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetControlTransferType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Recipient(&self) -> windows_core::Result<UsbControlRecipient> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Recipient)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRecipient(&self, value: UsbControlRecipient) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRecipient)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AsByte(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AsByte)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAsByte(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAsByte)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for UsbControlRequestType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbControlRequestType>();
}
unsafe impl windows_core::Interface for UsbControlRequestType {
    type Vtable = IUsbControlRequestType_Vtbl;
    const IID: windows_core::GUID = <IUsbControlRequestType as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbControlRequestType {
    const NAME: &'static str = "Windows.Devices.Usb.UsbControlRequestType";
}
unsafe impl Send for UsbControlRequestType {}
unsafe impl Sync for UsbControlRequestType {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbDescriptor, windows_core::IUnknown, windows_core::IInspectable);
impl UsbDescriptor {
    pub fn Length(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DescriptorType(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DescriptorType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadDescriptorBuffer<P0>(&self, buffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReadDescriptorBuffer)(windows_core::Interface::as_raw(this), buffer.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for UsbDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbDescriptor>();
}
unsafe impl windows_core::Interface for UsbDescriptor {
    type Vtable = IUsbDescriptor_Vtbl;
    const IID: windows_core::GUID = <IUsbDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbDescriptor";
}
unsafe impl Send for UsbDescriptor {}
unsafe impl Sync for UsbDescriptor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbDevice, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(UsbDevice, super::super::Foundation::IClosable);
impl UsbDevice {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SendControlOutTransferAsync<P0, P1>(&self, setuppacket: P0, buffer: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>
    where
        P0: windows_core::Param<UsbSetupPacket>,
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendControlOutTransferAsync)(windows_core::Interface::as_raw(this), setuppacket.param().abi(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SendControlOutTransferAsyncNoBuffer<P0>(&self, setuppacket: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>
    where
        P0: windows_core::Param<UsbSetupPacket>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendControlOutTransferAsyncNoBuffer)(windows_core::Interface::as_raw(this), setuppacket.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SendControlInTransferAsync<P0, P1>(&self, setuppacket: P0, buffer: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>
    where
        P0: windows_core::Param<UsbSetupPacket>,
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendControlInTransferAsync)(windows_core::Interface::as_raw(this), setuppacket.param().abi(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SendControlInTransferAsyncNoBuffer<P0>(&self, setuppacket: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>
    where
        P0: windows_core::Param<UsbSetupPacket>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendControlInTransferAsyncNoBuffer)(windows_core::Interface::as_raw(this), setuppacket.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DefaultInterface(&self) -> windows_core::Result<UsbInterface> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultInterface)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceDescriptor(&self) -> windows_core::Result<UsbDeviceDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceDescriptor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Configuration(&self) -> windows_core::Result<UsbConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeviceSelector(vendorid: u32, productid: u32, winusbinterfaceclass: windows_core::GUID) -> windows_core::Result<windows_core::HSTRING> {
        Self::IUsbDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), vendorid, productid, winusbinterfaceclass, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorGuidOnly(winusbinterfaceclass: windows_core::GUID) -> windows_core::Result<windows_core::HSTRING> {
        Self::IUsbDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorGuidOnly)(windows_core::Interface::as_raw(this), winusbinterfaceclass, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorVidPidOnly(vendorid: u32, productid: u32) -> windows_core::Result<windows_core::HSTRING> {
        Self::IUsbDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorVidPidOnly)(windows_core::Interface::as_raw(this), vendorid, productid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceClassSelector<P0>(usbclass: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<UsbDeviceClass>,
    {
        Self::IUsbDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceClassSelector)(windows_core::Interface::as_raw(this), usbclass.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UsbDevice>> {
        Self::IUsbDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUsbDeviceStatics<R, F: FnOnce(&IUsbDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UsbDevice, IUsbDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UsbDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbDevice>();
}
unsafe impl windows_core::Interface for UsbDevice {
    type Vtable = IUsbDevice_Vtbl;
    const IID: windows_core::GUID = <IUsbDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbDevice {
    const NAME: &'static str = "Windows.Devices.Usb.UsbDevice";
}
unsafe impl Send for UsbDevice {}
unsafe impl Sync for UsbDevice {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbDeviceClass(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbDeviceClass, windows_core::IUnknown, windows_core::IInspectable);
impl UsbDeviceClass {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UsbDeviceClass, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ClassCode(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClassCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetClassCode(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetClassCode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SubclassCode(&self) -> windows_core::Result<super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubclassCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSubclassCode<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<u8>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSubclassCode)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ProtocolCode(&self) -> windows_core::Result<super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtocolCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProtocolCode<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<u8>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProtocolCode)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for UsbDeviceClass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbDeviceClass>();
}
unsafe impl windows_core::Interface for UsbDeviceClass {
    type Vtable = IUsbDeviceClass_Vtbl;
    const IID: windows_core::GUID = <IUsbDeviceClass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbDeviceClass {
    const NAME: &'static str = "Windows.Devices.Usb.UsbDeviceClass";
}
unsafe impl Send for UsbDeviceClass {}
unsafe impl Sync for UsbDeviceClass {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbDeviceClasses(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbDeviceClasses, windows_core::IUnknown, windows_core::IInspectable);
impl UsbDeviceClasses {
    pub fn CdcControl() -> windows_core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CdcControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Physical() -> windows_core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Physical)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PersonalHealthcare() -> windows_core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PersonalHealthcare)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ActiveSync() -> windows_core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActiveSync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PalmSync() -> windows_core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PalmSync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DeviceFirmwareUpdate() -> windows_core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceFirmwareUpdate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Irda() -> windows_core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Irda)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Measurement() -> windows_core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Measurement)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn VendorSpecific() -> windows_core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VendorSpecific)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUsbDeviceClassesStatics<R, F: FnOnce(&IUsbDeviceClassesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UsbDeviceClasses, IUsbDeviceClassesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UsbDeviceClasses {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbDeviceClasses>();
}
unsafe impl windows_core::Interface for UsbDeviceClasses {
    type Vtable = IUsbDeviceClasses_Vtbl;
    const IID: windows_core::GUID = <IUsbDeviceClasses as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbDeviceClasses {
    const NAME: &'static str = "Windows.Devices.Usb.UsbDeviceClasses";
}
unsafe impl Send for UsbDeviceClasses {}
unsafe impl Sync for UsbDeviceClasses {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbDeviceDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbDeviceDescriptor, windows_core::IUnknown, windows_core::IInspectable);
impl UsbDeviceDescriptor {
    pub fn BcdUsb(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BcdUsb)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxPacketSize0(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPacketSize0)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn VendorId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VendorId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProductId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProductId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BcdDeviceRevision(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BcdDeviceRevision)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NumberOfConfigurations(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberOfConfigurations)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for UsbDeviceDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbDeviceDescriptor>();
}
unsafe impl windows_core::Interface for UsbDeviceDescriptor {
    type Vtable = IUsbDeviceDescriptor_Vtbl;
    const IID: windows_core::GUID = <IUsbDeviceDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbDeviceDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbDeviceDescriptor";
}
unsafe impl Send for UsbDeviceDescriptor {}
unsafe impl Sync for UsbDeviceDescriptor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbEndpointDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbEndpointDescriptor, windows_core::IUnknown, windows_core::IInspectable);
impl UsbEndpointDescriptor {
    pub fn EndpointNumber(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndpointNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Direction(&self) -> windows_core::Result<UsbTransferDirection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Direction)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EndpointType(&self) -> windows_core::Result<UsbEndpointType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndpointType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AsBulkInEndpointDescriptor(&self) -> windows_core::Result<UsbBulkInEndpointDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AsBulkInEndpointDescriptor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AsInterruptInEndpointDescriptor(&self) -> windows_core::Result<UsbInterruptInEndpointDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AsInterruptInEndpointDescriptor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AsBulkOutEndpointDescriptor(&self) -> windows_core::Result<UsbBulkOutEndpointDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AsBulkOutEndpointDescriptor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AsInterruptOutEndpointDescriptor(&self) -> windows_core::Result<UsbInterruptOutEndpointDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AsInterruptOutEndpointDescriptor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryParse<P0>(descriptor: P0, parsed: &mut Option<UsbEndpointDescriptor>) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<UsbDescriptor>,
    {
        Self::IUsbEndpointDescriptorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), descriptor.param().abi(), parsed as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn Parse<P0>(descriptor: P0) -> windows_core::Result<UsbEndpointDescriptor>
    where
        P0: windows_core::Param<UsbDescriptor>,
    {
        Self::IUsbEndpointDescriptorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), descriptor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUsbEndpointDescriptorStatics<R, F: FnOnce(&IUsbEndpointDescriptorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UsbEndpointDescriptor, IUsbEndpointDescriptorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UsbEndpointDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbEndpointDescriptor>();
}
unsafe impl windows_core::Interface for UsbEndpointDescriptor {
    type Vtable = IUsbEndpointDescriptor_Vtbl;
    const IID: windows_core::GUID = <IUsbEndpointDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbEndpointDescriptor";
}
unsafe impl Send for UsbEndpointDescriptor {}
unsafe impl Sync for UsbEndpointDescriptor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbInterface(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbInterface, windows_core::IUnknown, windows_core::IInspectable);
impl UsbInterface {
    #[cfg(feature = "Foundation_Collections")]
    pub fn BulkInPipes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<UsbBulkInPipe>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BulkInPipes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InterruptInPipes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<UsbInterruptInPipe>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InterruptInPipes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn BulkOutPipes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<UsbBulkOutPipe>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BulkOutPipes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InterruptOutPipes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<UsbInterruptOutPipe>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InterruptOutPipes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InterfaceSettings(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<UsbInterfaceSetting>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InterfaceSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InterfaceNumber(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InterfaceNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Descriptors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<UsbDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Descriptors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UsbInterface {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbInterface>();
}
unsafe impl windows_core::Interface for UsbInterface {
    type Vtable = IUsbInterface_Vtbl;
    const IID: windows_core::GUID = <IUsbInterface as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbInterface {
    const NAME: &'static str = "Windows.Devices.Usb.UsbInterface";
}
unsafe impl Send for UsbInterface {}
unsafe impl Sync for UsbInterface {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbInterfaceDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbInterfaceDescriptor, windows_core::IUnknown, windows_core::IInspectable);
impl UsbInterfaceDescriptor {
    pub fn ClassCode(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClassCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SubclassCode(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubclassCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProtocolCode(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtocolCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AlternateSettingNumber(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlternateSettingNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InterfaceNumber(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InterfaceNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TryParse<P0>(descriptor: P0, parsed: &mut Option<UsbInterfaceDescriptor>) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<UsbDescriptor>,
    {
        Self::IUsbInterfaceDescriptorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), descriptor.param().abi(), parsed as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn Parse<P0>(descriptor: P0) -> windows_core::Result<UsbInterfaceDescriptor>
    where
        P0: windows_core::Param<UsbDescriptor>,
    {
        Self::IUsbInterfaceDescriptorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), descriptor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUsbInterfaceDescriptorStatics<R, F: FnOnce(&IUsbInterfaceDescriptorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UsbInterfaceDescriptor, IUsbInterfaceDescriptorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UsbInterfaceDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbInterfaceDescriptor>();
}
unsafe impl windows_core::Interface for UsbInterfaceDescriptor {
    type Vtable = IUsbInterfaceDescriptor_Vtbl;
    const IID: windows_core::GUID = <IUsbInterfaceDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbInterfaceDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbInterfaceDescriptor";
}
unsafe impl Send for UsbInterfaceDescriptor {}
unsafe impl Sync for UsbInterfaceDescriptor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbInterfaceSetting(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbInterfaceSetting, windows_core::IUnknown, windows_core::IInspectable);
impl UsbInterfaceSetting {
    #[cfg(feature = "Foundation_Collections")]
    pub fn BulkInEndpoints(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<UsbBulkInEndpointDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BulkInEndpoints)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InterruptInEndpoints(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<UsbInterruptInEndpointDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InterruptInEndpoints)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn BulkOutEndpoints(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<UsbBulkOutEndpointDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BulkOutEndpoints)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InterruptOutEndpoints(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<UsbInterruptOutEndpointDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InterruptOutEndpoints)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Selected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Selected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SelectSettingAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSettingAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InterfaceDescriptor(&self) -> windows_core::Result<UsbInterfaceDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InterfaceDescriptor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Descriptors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<UsbDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Descriptors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UsbInterfaceSetting {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbInterfaceSetting>();
}
unsafe impl windows_core::Interface for UsbInterfaceSetting {
    type Vtable = IUsbInterfaceSetting_Vtbl;
    const IID: windows_core::GUID = <IUsbInterfaceSetting as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbInterfaceSetting {
    const NAME: &'static str = "Windows.Devices.Usb.UsbInterfaceSetting";
}
unsafe impl Send for UsbInterfaceSetting {}
unsafe impl Sync for UsbInterfaceSetting {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbInterruptInEndpointDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbInterruptInEndpointDescriptor, windows_core::IUnknown, windows_core::IInspectable);
impl UsbInterruptInEndpointDescriptor {
    pub fn MaxPacketSize(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPacketSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EndpointNumber(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndpointNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Interval(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Interval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Pipe(&self) -> windows_core::Result<UsbInterruptInPipe> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pipe)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UsbInterruptInEndpointDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbInterruptInEndpointDescriptor>();
}
unsafe impl windows_core::Interface for UsbInterruptInEndpointDescriptor {
    type Vtable = IUsbInterruptInEndpointDescriptor_Vtbl;
    const IID: windows_core::GUID = <IUsbInterruptInEndpointDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbInterruptInEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbInterruptInEndpointDescriptor";
}
unsafe impl Send for UsbInterruptInEndpointDescriptor {}
unsafe impl Sync for UsbInterruptInEndpointDescriptor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbInterruptInEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbInterruptInEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl UsbInterruptInEventArgs {
    #[cfg(feature = "Storage_Streams")]
    pub fn InterruptData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InterruptData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UsbInterruptInEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbInterruptInEventArgs>();
}
unsafe impl windows_core::Interface for UsbInterruptInEventArgs {
    type Vtable = IUsbInterruptInEventArgs_Vtbl;
    const IID: windows_core::GUID = <IUsbInterruptInEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbInterruptInEventArgs {
    const NAME: &'static str = "Windows.Devices.Usb.UsbInterruptInEventArgs";
}
unsafe impl Send for UsbInterruptInEventArgs {}
unsafe impl Sync for UsbInterruptInEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbInterruptInPipe(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbInterruptInPipe, windows_core::IUnknown, windows_core::IInspectable);
impl UsbInterruptInPipe {
    pub fn EndpointDescriptor(&self) -> windows_core::Result<UsbInterruptInEndpointDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndpointDescriptor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClearStallAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClearStallAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DataReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<UsbInterruptInPipe, UsbInterruptInEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDataReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDataReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for UsbInterruptInPipe {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbInterruptInPipe>();
}
unsafe impl windows_core::Interface for UsbInterruptInPipe {
    type Vtable = IUsbInterruptInPipe_Vtbl;
    const IID: windows_core::GUID = <IUsbInterruptInPipe as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbInterruptInPipe {
    const NAME: &'static str = "Windows.Devices.Usb.UsbInterruptInPipe";
}
unsafe impl Send for UsbInterruptInPipe {}
unsafe impl Sync for UsbInterruptInPipe {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbInterruptOutEndpointDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbInterruptOutEndpointDescriptor, windows_core::IUnknown, windows_core::IInspectable);
impl UsbInterruptOutEndpointDescriptor {
    pub fn MaxPacketSize(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPacketSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EndpointNumber(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndpointNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Interval(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Interval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Pipe(&self) -> windows_core::Result<UsbInterruptOutPipe> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pipe)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UsbInterruptOutEndpointDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbInterruptOutEndpointDescriptor>();
}
unsafe impl windows_core::Interface for UsbInterruptOutEndpointDescriptor {
    type Vtable = IUsbInterruptOutEndpointDescriptor_Vtbl;
    const IID: windows_core::GUID = <IUsbInterruptOutEndpointDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbInterruptOutEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbInterruptOutEndpointDescriptor";
}
unsafe impl Send for UsbInterruptOutEndpointDescriptor {}
unsafe impl Sync for UsbInterruptOutEndpointDescriptor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbInterruptOutPipe(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbInterruptOutPipe, windows_core::IUnknown, windows_core::IInspectable);
impl UsbInterruptOutPipe {
    pub fn EndpointDescriptor(&self) -> windows_core::Result<UsbInterruptOutEndpointDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndpointDescriptor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClearStallAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClearStallAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetWriteOptions(&self, value: UsbWriteOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWriteOptions)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteOptions(&self) -> windows_core::Result<UsbWriteOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteOptions)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UsbInterruptOutPipe {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbInterruptOutPipe>();
}
unsafe impl windows_core::Interface for UsbInterruptOutPipe {
    type Vtable = IUsbInterruptOutPipe_Vtbl;
    const IID: windows_core::GUID = <IUsbInterruptOutPipe as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbInterruptOutPipe {
    const NAME: &'static str = "Windows.Devices.Usb.UsbInterruptOutPipe";
}
unsafe impl Send for UsbInterruptOutPipe {}
unsafe impl Sync for UsbInterruptOutPipe {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UsbSetupPacket(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UsbSetupPacket, windows_core::IUnknown, windows_core::IInspectable);
impl UsbSetupPacket {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UsbSetupPacket, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn RequestType(&self) -> windows_core::Result<UsbControlRequestType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRequestType<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UsbControlRequestType>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRequestType)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Request(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRequest(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRequest)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Value(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetValue(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Index(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Index)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIndex(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Length(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLength)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateWithEightByteBuffer<P0>(eightbytebuffer: P0) -> windows_core::Result<UsbSetupPacket>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::IUsbSetupPacketFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithEightByteBuffer)(windows_core::Interface::as_raw(this), eightbytebuffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUsbSetupPacketFactory<R, F: FnOnce(&IUsbSetupPacketFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UsbSetupPacket, IUsbSetupPacketFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UsbSetupPacket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUsbSetupPacket>();
}
unsafe impl windows_core::Interface for UsbSetupPacket {
    type Vtable = IUsbSetupPacket_Vtbl;
    const IID: windows_core::GUID = <IUsbSetupPacket as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UsbSetupPacket {
    const NAME: &'static str = "Windows.Devices.Usb.UsbSetupPacket";
}
unsafe impl Send for UsbSetupPacket {}
unsafe impl Sync for UsbSetupPacket {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UsbControlRecipient(pub i32);
impl UsbControlRecipient {
    pub const Device: Self = Self(0i32);
    pub const SpecifiedInterface: Self = Self(1i32);
    pub const Endpoint: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
    pub const DefaultInterface: Self = Self(4i32);
}
impl windows_core::TypeKind for UsbControlRecipient {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UsbControlRecipient {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UsbControlRecipient").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UsbControlRecipient {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Usb.UsbControlRecipient;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UsbControlTransferType(pub i32);
impl UsbControlTransferType {
    pub const Standard: Self = Self(0i32);
    pub const Class: Self = Self(1i32);
    pub const Vendor: Self = Self(2i32);
}
impl windows_core::TypeKind for UsbControlTransferType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UsbControlTransferType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UsbControlTransferType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UsbControlTransferType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Usb.UsbControlTransferType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UsbEndpointType(pub i32);
impl UsbEndpointType {
    pub const Control: Self = Self(0i32);
    pub const Isochronous: Self = Self(1i32);
    pub const Bulk: Self = Self(2i32);
    pub const Interrupt: Self = Self(3i32);
}
impl windows_core::TypeKind for UsbEndpointType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UsbEndpointType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UsbEndpointType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UsbEndpointType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Usb.UsbEndpointType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UsbReadOptions(pub u32);
impl UsbReadOptions {
    pub const None: Self = Self(0u32);
    pub const AutoClearStall: Self = Self(1u32);
    pub const OverrideAutomaticBufferManagement: Self = Self(2u32);
    pub const IgnoreShortPacket: Self = Self(4u32);
    pub const AllowPartialReads: Self = Self(8u32);
}
impl windows_core::TypeKind for UsbReadOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UsbReadOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UsbReadOptions").field(&self.0).finish()
    }
}
impl UsbReadOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for UsbReadOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for UsbReadOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for UsbReadOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for UsbReadOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for UsbReadOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for UsbReadOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Usb.UsbReadOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UsbTransferDirection(pub i32);
impl UsbTransferDirection {
    pub const Out: Self = Self(0i32);
    pub const In: Self = Self(1i32);
}
impl windows_core::TypeKind for UsbTransferDirection {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UsbTransferDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UsbTransferDirection").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UsbTransferDirection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Usb.UsbTransferDirection;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UsbWriteOptions(pub u32);
impl UsbWriteOptions {
    pub const None: Self = Self(0u32);
    pub const AutoClearStall: Self = Self(1u32);
    pub const ShortPacketTerminate: Self = Self(2u32);
}
impl windows_core::TypeKind for UsbWriteOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UsbWriteOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UsbWriteOptions").field(&self.0).finish()
    }
}
impl UsbWriteOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for UsbWriteOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for UsbWriteOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for UsbWriteOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for UsbWriteOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for UsbWriteOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for UsbWriteOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Usb.UsbWriteOptions;u4)");
}
