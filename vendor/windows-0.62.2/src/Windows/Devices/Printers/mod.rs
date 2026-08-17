#[cfg(feature = "Devices_Printers_Extensions")]
pub mod Extensions;
windows_core::imp::define_interface!(IIppAttributeError, IIppAttributeError_Vtbl, 0x750feda1_9eef_5c39_93e4_46149bbcef27);
impl windows_core::RuntimeType for IIppAttributeError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppAttributeError_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IppAttributeErrorReason) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetUnsupportedValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIppAttributeValue, IIppAttributeValue_Vtbl, 0x99407fed_e2bb_59a3_988b_28a974052a26);
impl windows_core::RuntimeType for IIppAttributeValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppAttributeValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IppAttributeValueKind) -> windows_core::HRESULT,
    pub GetIntegerArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBooleanArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEnumArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetOctetStringArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetOctetStringArray: usize,
    pub GetDateTimeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResolutionArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRangeOfIntegerArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCollectionArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTextWithLanguageArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNameWithLanguageArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTextWithoutLanguageArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNameWithoutLanguageArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetKeywordArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUriArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUriSchemaArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCharsetArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNaturalLanguageArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMimeMediaTypeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIppAttributeValueStatics, IIppAttributeValueStatics_Vtbl, 0x10d43942_dd94_5998_b235_afafb6fa7935);
impl windows_core::RuntimeType for IIppAttributeValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppAttributeValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateUnsupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUnknown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNoValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInteger: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateIntegerArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBoolean: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBooleanArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEnum: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEnumArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateOctetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateOctetString: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateOctetStringArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateOctetStringArray: usize,
    pub CreateDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDateTimeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateResolutionArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRangeOfInteger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRangeOfIntegerArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCollectionArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTextWithLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTextWithLanguageArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNameWithLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNameWithLanguageArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTextWithoutLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTextWithoutLanguageArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNameWithoutLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNameWithoutLanguageArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateKeyword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateKeywordArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUriArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUriSchema: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUriSchemaArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCharset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCharsetArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNaturalLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNaturalLanguageArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateMimeMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateMimeMediaArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIppIntegerRange, IIppIntegerRange_Vtbl, 0x92907346_c3ea_5ed6_bdb1_3752c62c6f7f);
impl windows_core::RuntimeType for IIppIntegerRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppIntegerRange_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIppIntegerRangeFactory, IIppIntegerRangeFactory_Vtbl, 0x75d4ecae_f87e_54ad_b5d0_465204db7553);
impl windows_core::RuntimeType for IIppIntegerRangeFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppIntegerRangeFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIppPrintDevice, IIppPrintDevice_Vtbl, 0xd748ac56_76f3_5dc6_afd4_c2a8686b9359);
impl windows_core::RuntimeType for IIppPrintDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppPrintDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrinterName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrinterUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetPrinterAttributesAsBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetPrinterAttributesAsBuffer: usize,
    pub GetPrinterAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetPrinterAttributesFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPrinterAttributesFromBuffer: usize,
    pub SetPrinterAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIppPrintDevice2, IIppPrintDevice2_Vtbl, 0xf7c844c9_9d21_5c63_ac20_3676915be2d7);
impl windows_core::RuntimeType for IIppPrintDevice2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppPrintDevice2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetMaxSupportedPdfSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetMaxSupportedPdfVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPdlPassthroughSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetPdlPassthroughProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIppPrintDevice3, IIppPrintDevice3_Vtbl, 0xb6258f6d_a46d_5e37_80ce_5f69d5544712);
impl windows_core::RuntimeType for IIppPrintDevice3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppPrintDevice3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsIppFaxOutPrinter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIppPrintDevice4, IIppPrintDevice4_Vtbl, 0x8c48247e_e869_59fb_bc6d_daea0614f93e);
impl windows_core::RuntimeType for IIppPrintDevice4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppPrintDevice4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IppPrintDeviceKind) -> windows_core::HRESULT,
    pub CanModifyUserDefaultPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub UserDefaultPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    UserDefaultPrintTicket: usize,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub SetUserDefaultPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    SetUserDefaultPrintTicket: usize,
    pub RefreshPrintDeviceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMaxSupportedPdlVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIppPrintDevice5, IIppPrintDevice5_Vtbl, 0xea927fca_e073_5db4_9aee_13df714e853a);
impl windows_core::RuntimeType for IIppPrintDevice5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppPrintDevice5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDeviceProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDeviceProperties: usize,
    pub ReplaceDeviceProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIppPrintDeviceStatics, IIppPrintDeviceStatics_Vtbl, 0x7dc19f08_7f20_52ab_94a7_894b83b2a17e);
impl windows_core::RuntimeType for IIppPrintDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppPrintDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromPrinterName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsIppPrinter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIppResolution, IIppResolution_Vtbl, 0xcb493f86_6bf3_56f5_86ce_263d08aead63);
impl windows_core::RuntimeType for IIppResolution {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppResolution_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Unit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IppResolutionUnit) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIppResolutionFactory, IIppResolutionFactory_Vtbl, 0xe481c2ae_251a_5326_b173_95543ed99a35);
impl windows_core::RuntimeType for IIppResolutionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppResolutionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, IppResolutionUnit, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIppSetAttributesResult, IIppSetAttributesResult_Vtbl, 0x7d1c7f55_aa9d_58a3_90e9_17bdc5281f07);
impl windows_core::RuntimeType for IIppSetAttributesResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppSetAttributesResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AttributeErrors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIppTextWithLanguage, IIppTextWithLanguage_Vtbl, 0x326447a6_5149_5936_90e8_0c736036bf77);
impl windows_core::RuntimeType for IIppTextWithLanguage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppTextWithLanguage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIppTextWithLanguageFactory, IIppTextWithLanguageFactory_Vtbl, 0xca4a1e8d_2968_5775_997c_8a46f1a574ed);
impl windows_core::RuntimeType for IIppTextWithLanguageFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppTextWithLanguageFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPageConfigurationSettings, IPageConfigurationSettings_Vtbl, 0xb6fc1e02_5331_54ff_95a0_1fcb76bb97a9);
impl windows_core::RuntimeType for IPageConfigurationSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageConfigurationSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OrientationSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PageConfigurationSource) -> windows_core::HRESULT,
    pub SetOrientationSource: unsafe extern "system" fn(*mut core::ffi::c_void, PageConfigurationSource) -> windows_core::HRESULT,
    pub SizeSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PageConfigurationSource) -> windows_core::HRESULT,
    pub SetSizeSource: unsafe extern "system" fn(*mut core::ffi::c_void, PageConfigurationSource) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPdlPassthroughProvider, IPdlPassthroughProvider_Vtbl, 0x23c71dd2_6117_553f_9378_180af5849a49);
impl windows_core::RuntimeType for IPdlPassthroughProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdlPassthroughProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SupportedPdlContentTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Printing")]
    pub StartPrintJobWithTaskOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    StartPrintJobWithTaskOptions: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StartPrintJobWithPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StartPrintJobWithPrintTicket: usize,
}
windows_core::imp::define_interface!(IPdlPassthroughTarget, IPdlPassthroughTarget_Vtbl, 0x9840be79_67f8_5385_a5b9_e8c96e0fca76);
impl windows_core::RuntimeType for IPdlPassthroughTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdlPassthroughTarget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrintJobId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetOutputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetOutputStream: usize,
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrint3DDevice, IPrint3DDevice_Vtbl, 0x041c3d19_9713_42a2_9813_7dc3337428d3);
impl windows_core::RuntimeType for IPrint3DDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrintSchema: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrint3DDeviceStatics, IPrint3DDeviceStatics_Vtbl, 0xfde3620a_67cd_41b7_a344_5150a1fd75b5);
impl windows_core::RuntimeType for IPrint3DDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSchema, IPrintSchema_Vtbl, 0xc2b98316_26b8_4bfb_8138_9f962c22a35b);
impl windows_core::RuntimeType for IPrintSchema {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchema_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetDefaultPrintTicketAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetDefaultPrintTicketAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetCapabilitiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetCapabilitiesAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub MergeAndValidateWithDefaultPrintTicketAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    MergeAndValidateWithDefaultPrintTicketAsync: usize,
}
windows_core::imp::define_interface!(IReplaceDevicePropertiesResult, IReplaceDevicePropertiesResult_Vtbl, 0x12feca4b_d973_57e1_826b_f75b9518a9f1);
impl windows_core::RuntimeType for IReplaceDevicePropertiesResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IReplaceDevicePropertiesResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ReplaceDevicePropertiesStatus) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVirtualPrinterInstallationParameters, IVirtualPrinterInstallationParameters_Vtbl, 0xbbc159b3_12f3_584c_8d26_b22c0dc83241);
impl windows_core::RuntimeType for IVirtualPrinterInstallationParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualPrinterInstallationParameters_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrinterName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrinterName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OutputFileExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportedInputFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrintDeviceCapabilitiesPackageRelativeFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrintDeviceCapabilitiesPackageRelativeFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrintDeviceResourcesPackageRelativeFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrintDeviceResourcesPackageRelativeFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PreferredInputFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VirtualPrinterPreferredInputFormat) -> windows_core::HRESULT,
    pub SetPreferredInputFormat: unsafe extern "system" fn(*mut core::ffi::c_void, VirtualPrinterPreferredInputFormat) -> windows_core::HRESULT,
    pub PrinterUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrinterUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EntryPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEntryPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVirtualPrinterInstallationResult, IVirtualPrinterInstallationResult_Vtbl, 0x82defd78_1601_5657_85df_75eb691604bd);
impl windows_core::RuntimeType for IVirtualPrinterInstallationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualPrinterInstallationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VirtualPrinterInstallationStatus) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVirtualPrinterManagerStatics, IVirtualPrinterManagerStatics_Vtbl, 0x141084b6_6702_5b5f_83da_c75891657554);
impl windows_core::RuntimeType for IVirtualPrinterManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualPrinterManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InstallVirtualPrinterAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallVirtualPrinterAsync2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallVirtualPrinterForAllUsersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallVirtualPrinterForAllUsersAsync2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindAllVirtualPrinters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindAllVirtualPrinters2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveVirtualPrinterAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveVirtualPrinterForAllUsersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVirtualPrinterSupportedFormat, IVirtualPrinterSupportedFormat_Vtbl, 0x3801fa17_22b5_5dab_ad38_39e47d6071af);
impl windows_core::RuntimeType for IVirtualPrinterSupportedFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualPrinterSupportedFormat_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxSupportedVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMaxSupportedVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVirtualPrinterSupportedFormatFactory, IVirtualPrinterSupportedFormatFactory_Vtbl, 0x6daaed44_97a6_57f4_be8b_9dbabc587f2d);
impl windows_core::RuntimeType for IVirtualPrinterSupportedFormatFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualPrinterSupportedFormatFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IppAttributeError(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(IppAttributeError, windows_core::IUnknown, windows_core::IInspectable);
impl IppAttributeError {
    pub fn Reason(&self) -> windows_core::Result<IppAttributeErrorReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetUnsupportedValues(&self) -> windows_core::Result<windows_collections::IVectorView<IppAttributeValue>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUnsupportedValues)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IppAttributeError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIppAttributeError>();
}
unsafe impl windows_core::Interface for IppAttributeError {
    type Vtable = <IIppAttributeError as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IIppAttributeError as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for IppAttributeError {
    const NAME: &'static str = "Windows.Devices.Printers.IppAttributeError";
}
unsafe impl Send for IppAttributeError {}
unsafe impl Sync for IppAttributeError {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IppAttributeErrorReason(pub i32);
impl IppAttributeErrorReason {
    pub const RequestEntityTooLarge: Self = Self(0i32);
    pub const AttributeNotSupported: Self = Self(1i32);
    pub const AttributeValuesNotSupported: Self = Self(2i32);
    pub const AttributeNotSettable: Self = Self(3i32);
    pub const ConflictingAttributes: Self = Self(4i32);
}
impl windows_core::TypeKind for IppAttributeErrorReason {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for IppAttributeErrorReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.IppAttributeErrorReason;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IppAttributeValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(IppAttributeValue, windows_core::IUnknown, windows_core::IInspectable);
impl IppAttributeValue {
    pub fn Kind(&self) -> windows_core::Result<IppAttributeValueKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetIntegerArray(&self) -> windows_core::Result<windows_collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIntegerArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetBooleanArray(&self) -> windows_core::Result<windows_collections::IVector<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBooleanArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetEnumArray(&self) -> windows_core::Result<windows_collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetEnumArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOctetStringArray(&self) -> windows_core::Result<windows_collections::IVector<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOctetStringArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDateTimeArray(&self) -> windows_core::Result<windows_collections::IVector<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDateTimeArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetResolutionArray(&self) -> windows_core::Result<windows_collections::IVector<IppResolution>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetResolutionArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetRangeOfIntegerArray(&self) -> windows_core::Result<windows_collections::IVector<IppIntegerRange>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRangeOfIntegerArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCollectionArray(&self) -> windows_core::Result<windows_collections::IVector<windows_collections::IMapView<windows_core::HSTRING, IppAttributeValue>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCollectionArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetTextWithLanguageArray(&self) -> windows_core::Result<windows_collections::IVector<IppTextWithLanguage>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTextWithLanguageArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNameWithLanguageArray(&self) -> windows_core::Result<windows_collections::IVector<IppTextWithLanguage>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNameWithLanguageArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetTextWithoutLanguageArray(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTextWithoutLanguageArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNameWithoutLanguageArray(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNameWithoutLanguageArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetKeywordArray(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetKeywordArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetUriArray(&self) -> windows_core::Result<windows_collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUriArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetUriSchemaArray(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUriSchemaArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCharsetArray(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCharsetArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNaturalLanguageArray(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNaturalLanguageArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetMimeMediaTypeArray(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMimeMediaTypeArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateUnsupported() -> windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUnsupported)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateUnknown() -> windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUnknown)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateNoValue() -> windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNoValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInteger(value: i32) -> windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInteger)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateIntegerArray<P0>(values: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<i32>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateIntegerArray)(windows_core::Interface::as_raw(this), values.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateBoolean(value: bool) -> windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateBoolean)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateBooleanArray<P0>(values: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<bool>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateBooleanArray)(windows_core::Interface::as_raw(this), values.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateEnum(value: i32) -> windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateEnum)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateEnumArray<P0>(values: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<i32>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateEnumArray)(windows_core::Interface::as_raw(this), values.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateOctetString<P0>(value: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateOctetString)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateOctetStringArray<P0>(values: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<super::super::Storage::Streams::IBuffer>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateOctetStringArray)(windows_core::Interface::as_raw(this), values.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateDateTime(value: super::super::Foundation::DateTime) -> windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDateTime)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateDateTimeArray<P0>(values: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<super::super::Foundation::DateTime>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDateTimeArray)(windows_core::Interface::as_raw(this), values.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateResolution<P0>(value: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<IppResolution>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateResolution)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateResolutionArray<P0>(values: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<IppResolution>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateResolutionArray)(windows_core::Interface::as_raw(this), values.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateRangeOfInteger<P0>(value: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<IppIntegerRange>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateRangeOfInteger)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateRangeOfIntegerArray<P0>(values: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<IppIntegerRange>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateRangeOfIntegerArray)(windows_core::Interface::as_raw(this), values.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateCollection<P0>(memberattributes: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, IppAttributeValue>>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCollection)(windows_core::Interface::as_raw(this), memberattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateCollectionArray<P0>(memberattributesarray: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, IppAttributeValue>>>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCollectionArray)(windows_core::Interface::as_raw(this), memberattributesarray.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateTextWithLanguage<P0>(value: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<IppTextWithLanguage>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTextWithLanguage)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateTextWithLanguageArray<P0>(values: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<IppTextWithLanguage>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTextWithLanguageArray)(windows_core::Interface::as_raw(this), values.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateNameWithLanguage<P0>(value: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<IppTextWithLanguage>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNameWithLanguage)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateNameWithLanguageArray<P0>(values: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<IppTextWithLanguage>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNameWithLanguageArray)(windows_core::Interface::as_raw(this), values.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateTextWithoutLanguage(value: &windows_core::HSTRING) -> windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTextWithoutLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateTextWithoutLanguageArray<P0>(values: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTextWithoutLanguageArray)(windows_core::Interface::as_raw(this), values.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateNameWithoutLanguage(value: &windows_core::HSTRING) -> windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNameWithoutLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateNameWithoutLanguageArray<P0>(values: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNameWithoutLanguageArray)(windows_core::Interface::as_raw(this), values.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateKeyword(value: &windows_core::HSTRING) -> windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateKeyword)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateKeywordArray<P0>(values: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateKeywordArray)(windows_core::Interface::as_raw(this), values.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateUri<P0>(value: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUri)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateUriArray<P0>(values: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUriArray)(windows_core::Interface::as_raw(this), values.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateUriSchema(value: &windows_core::HSTRING) -> windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUriSchema)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateUriSchemaArray<P0>(values: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUriSchemaArray)(windows_core::Interface::as_raw(this), values.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateCharset(value: &windows_core::HSTRING) -> windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCharset)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateCharsetArray<P0>(values: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCharsetArray)(windows_core::Interface::as_raw(this), values.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateNaturalLanguage(value: &windows_core::HSTRING) -> windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNaturalLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateNaturalLanguageArray<P0>(values: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNaturalLanguageArray)(windows_core::Interface::as_raw(this), values.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateMimeMedia(value: &windows_core::HSTRING) -> windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMimeMedia)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateMimeMediaArray<P0>(values: P0) -> windows_core::Result<IppAttributeValue>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMimeMediaArray)(windows_core::Interface::as_raw(this), values.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IIppAttributeValueStatics<R, F: FnOnce(&IIppAttributeValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IppAttributeValue, IIppAttributeValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for IppAttributeValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIppAttributeValue>();
}
unsafe impl windows_core::Interface for IppAttributeValue {
    type Vtable = <IIppAttributeValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IIppAttributeValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for IppAttributeValue {
    const NAME: &'static str = "Windows.Devices.Printers.IppAttributeValue";
}
unsafe impl Send for IppAttributeValue {}
unsafe impl Sync for IppAttributeValue {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IppAttributeValueKind(pub i32);
impl IppAttributeValueKind {
    pub const Unsupported: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const NoValue: Self = Self(2i32);
    pub const Integer: Self = Self(3i32);
    pub const Boolean: Self = Self(4i32);
    pub const Enum: Self = Self(5i32);
    pub const OctetString: Self = Self(6i32);
    pub const DateTime: Self = Self(7i32);
    pub const Resolution: Self = Self(8i32);
    pub const RangeOfInteger: Self = Self(9i32);
    pub const Collection: Self = Self(10i32);
    pub const TextWithLanguage: Self = Self(11i32);
    pub const NameWithLanguage: Self = Self(12i32);
    pub const TextWithoutLanguage: Self = Self(13i32);
    pub const NameWithoutLanguage: Self = Self(14i32);
    pub const Keyword: Self = Self(15i32);
    pub const Uri: Self = Self(16i32);
    pub const UriSchema: Self = Self(17i32);
    pub const Charset: Self = Self(18i32);
    pub const NaturalLanguage: Self = Self(19i32);
    pub const MimeMediaType: Self = Self(20i32);
}
impl windows_core::TypeKind for IppAttributeValueKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for IppAttributeValueKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.IppAttributeValueKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IppIntegerRange(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(IppIntegerRange, windows_core::IUnknown, windows_core::IInspectable);
impl IppIntegerRange {
    pub fn Start(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn End(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).End)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInstance(start: i32, end: i32) -> windows_core::Result<IppIntegerRange> {
        Self::IIppIntegerRangeFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), start, end, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IIppIntegerRangeFactory<R, F: FnOnce(&IIppIntegerRangeFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IppIntegerRange, IIppIntegerRangeFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for IppIntegerRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIppIntegerRange>();
}
unsafe impl windows_core::Interface for IppIntegerRange {
    type Vtable = <IIppIntegerRange as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IIppIntegerRange as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for IppIntegerRange {
    const NAME: &'static str = "Windows.Devices.Printers.IppIntegerRange";
}
unsafe impl Send for IppIntegerRange {}
unsafe impl Sync for IppIntegerRange {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IppPrintDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(IppPrintDevice, windows_core::IUnknown, windows_core::IInspectable);
impl IppPrintDevice {
    pub fn PrinterName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrinterName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn PrinterUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrinterUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetPrinterAttributesAsBuffer<P0>(&self, attributenames: P0) -> windows_core::Result<super::super::Storage::Streams::IBuffer>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPrinterAttributesAsBuffer)(windows_core::Interface::as_raw(this), attributenames.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPrinterAttributes<P0>(&self, attributenames: P0) -> windows_core::Result<windows_collections::IMap<windows_core::HSTRING, IppAttributeValue>>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPrinterAttributes)(windows_core::Interface::as_raw(this), attributenames.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPrinterAttributesFromBuffer<P0>(&self, printerattributesbuffer: P0) -> windows_core::Result<IppSetAttributesResult>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetPrinterAttributesFromBuffer)(windows_core::Interface::as_raw(this), printerattributesbuffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPrinterAttributes<P0>(&self, printerattributes: P0) -> windows_core::Result<IppSetAttributesResult>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, IppAttributeValue>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetPrinterAttributes)(windows_core::Interface::as_raw(this), printerattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetMaxSupportedPdfSize(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<IIppPrintDevice2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMaxSupportedPdfSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetMaxSupportedPdfVersion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IIppPrintDevice2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMaxSupportedPdfVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IsPdlPassthroughSupported(&self, pdlcontenttype: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IIppPrintDevice2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPdlPassthroughSupported)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(pdlcontenttype), &mut result__).map(|| result__)
        }
    }
    pub fn GetPdlPassthroughProvider(&self) -> windows_core::Result<PdlPassthroughProvider> {
        let this = &windows_core::Interface::cast::<IIppPrintDevice2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPdlPassthroughProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsIppFaxOutPrinter(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IIppPrintDevice3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsIppFaxOutPrinter)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceKind(&self) -> windows_core::Result<IppPrintDeviceKind> {
        let this = &windows_core::Interface::cast::<IIppPrintDevice4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanModifyUserDefaultPrintTicket(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IIppPrintDevice4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanModifyUserDefaultPrintTicket)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn UserDefaultPrintTicket(&self) -> windows_core::Result<super::super::Graphics::Printing::PrintTicket::WorkflowPrintTicket> {
        let this = &windows_core::Interface::cast::<IIppPrintDevice4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserDefaultPrintTicket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn SetUserDefaultPrintTicket<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Printing::PrintTicket::WorkflowPrintTicket>,
    {
        let this = &windows_core::Interface::cast::<IIppPrintDevice4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUserDefaultPrintTicket)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RefreshPrintDeviceCapabilities(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IIppPrintDevice4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RefreshPrintDeviceCapabilities)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMaxSupportedPdlVersion(&self, pdlcontenttype: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IIppPrintDevice4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMaxSupportedPdlVersion)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(pdlcontenttype), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDeviceProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<IIppPrintDevice5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReplaceDeviceProperties<P0>(&self, deviceproperties: P0) -> windows_core::Result<ReplaceDevicePropertiesResult>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>>,
    {
        let this = &windows_core::Interface::cast::<IIppPrintDevice5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceDeviceProperties)(windows_core::Interface::as_raw(this), deviceproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IIppPrintDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FromId(deviceid: &windows_core::HSTRING) -> windows_core::Result<IppPrintDevice> {
        Self::IIppPrintDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromPrinterName(printername: &windows_core::HSTRING) -> windows_core::Result<IppPrintDevice> {
        Self::IIppPrintDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromPrinterName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(printername), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsIppPrinter(printername: &windows_core::HSTRING) -> windows_core::Result<bool> {
        Self::IIppPrintDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsIppPrinter)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(printername), &mut result__).map(|| result__)
        })
    }
    fn IIppPrintDeviceStatics<R, F: FnOnce(&IIppPrintDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IppPrintDevice, IIppPrintDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for IppPrintDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIppPrintDevice>();
}
unsafe impl windows_core::Interface for IppPrintDevice {
    type Vtable = <IIppPrintDevice as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IIppPrintDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for IppPrintDevice {
    const NAME: &'static str = "Windows.Devices.Printers.IppPrintDevice";
}
unsafe impl Send for IppPrintDevice {}
unsafe impl Sync for IppPrintDevice {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IppPrintDeviceKind(pub i32);
impl IppPrintDeviceKind {
    pub const Printer: Self = Self(0i32);
    pub const FaxOut: Self = Self(1i32);
    pub const VirtualPrinter: Self = Self(2i32);
}
impl windows_core::TypeKind for IppPrintDeviceKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for IppPrintDeviceKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.IppPrintDeviceKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IppResolution(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(IppResolution, windows_core::IUnknown, windows_core::IInspectable);
impl IppResolution {
    pub fn Width(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Height(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Unit(&self) -> windows_core::Result<IppResolutionUnit> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInstance(width: i32, height: i32, unit: IppResolutionUnit) -> windows_core::Result<IppResolution> {
        Self::IIppResolutionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), width, height, unit, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IIppResolutionFactory<R, F: FnOnce(&IIppResolutionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IppResolution, IIppResolutionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for IppResolution {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIppResolution>();
}
unsafe impl windows_core::Interface for IppResolution {
    type Vtable = <IIppResolution as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IIppResolution as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for IppResolution {
    const NAME: &'static str = "Windows.Devices.Printers.IppResolution";
}
unsafe impl Send for IppResolution {}
unsafe impl Sync for IppResolution {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IppResolutionUnit(pub i32);
impl IppResolutionUnit {
    pub const DotsPerInch: Self = Self(0i32);
    pub const DotsPerCentimeter: Self = Self(1i32);
}
impl windows_core::TypeKind for IppResolutionUnit {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for IppResolutionUnit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.IppResolutionUnit;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IppSetAttributesResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(IppSetAttributesResult, windows_core::IUnknown, windows_core::IInspectable);
impl IppSetAttributesResult {
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AttributeErrors(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, IppAttributeError>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttributeErrors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IppSetAttributesResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIppSetAttributesResult>();
}
unsafe impl windows_core::Interface for IppSetAttributesResult {
    type Vtable = <IIppSetAttributesResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IIppSetAttributesResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for IppSetAttributesResult {
    const NAME: &'static str = "Windows.Devices.Printers.IppSetAttributesResult";
}
unsafe impl Send for IppSetAttributesResult {}
unsafe impl Sync for IppSetAttributesResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IppTextWithLanguage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(IppTextWithLanguage, windows_core::IUnknown, windows_core::IInspectable);
impl IppTextWithLanguage {
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateInstance(language: &windows_core::HSTRING, text: &windows_core::HSTRING) -> windows_core::Result<IppTextWithLanguage> {
        Self::IIppTextWithLanguageFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(language), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IIppTextWithLanguageFactory<R, F: FnOnce(&IIppTextWithLanguageFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IppTextWithLanguage, IIppTextWithLanguageFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for IppTextWithLanguage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIppTextWithLanguage>();
}
unsafe impl windows_core::Interface for IppTextWithLanguage {
    type Vtable = <IIppTextWithLanguage as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IIppTextWithLanguage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for IppTextWithLanguage {
    const NAME: &'static str = "Windows.Devices.Printers.IppTextWithLanguage";
}
unsafe impl Send for IppTextWithLanguage {}
unsafe impl Sync for IppTextWithLanguage {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageConfigurationSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PageConfigurationSettings, windows_core::IUnknown, windows_core::IInspectable);
impl PageConfigurationSettings {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PageConfigurationSettings, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn OrientationSource(&self) -> windows_core::Result<PageConfigurationSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OrientationSource)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOrientationSource(&self, value: PageConfigurationSource) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOrientationSource)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SizeSource(&self) -> windows_core::Result<PageConfigurationSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SizeSource)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSizeSource(&self, value: PageConfigurationSource) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSizeSource)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for PageConfigurationSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPageConfigurationSettings>();
}
unsafe impl windows_core::Interface for PageConfigurationSettings {
    type Vtable = <IPageConfigurationSettings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPageConfigurationSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PageConfigurationSettings {
    const NAME: &'static str = "Windows.Devices.Printers.PageConfigurationSettings";
}
unsafe impl Send for PageConfigurationSettings {}
unsafe impl Sync for PageConfigurationSettings {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PageConfigurationSource(pub i32);
impl PageConfigurationSource {
    pub const PrintJobConfiguration: Self = Self(0i32);
    pub const PdlContent: Self = Self(1i32);
}
impl windows_core::TypeKind for PageConfigurationSource {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PageConfigurationSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.PageConfigurationSource;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PdlPassthroughProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PdlPassthroughProvider, windows_core::IUnknown, windows_core::IInspectable);
impl PdlPassthroughProvider {
    pub fn SupportedPdlContentTypes(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedPdlContentTypes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Printing")]
    pub fn StartPrintJobWithTaskOptions<P2, P3>(&self, jobname: &windows_core::HSTRING, pdlcontenttype: &windows_core::HSTRING, taskoptions: P2, pageconfigurationsettings: P3) -> windows_core::Result<PdlPassthroughTarget>
    where
        P2: windows_core::Param<super::super::Graphics::Printing::PrintTaskOptions>,
        P3: windows_core::Param<PageConfigurationSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartPrintJobWithTaskOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(jobname), core::mem::transmute_copy(pdlcontenttype), taskoptions.param().abi(), pageconfigurationsettings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn StartPrintJobWithPrintTicket<P2, P3>(&self, jobname: &windows_core::HSTRING, pdlcontenttype: &windows_core::HSTRING, printticket: P2, pageconfigurationsettings: P3) -> windows_core::Result<PdlPassthroughTarget>
    where
        P2: windows_core::Param<super::super::Storage::Streams::IInputStream>,
        P3: windows_core::Param<PageConfigurationSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartPrintJobWithPrintTicket)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(jobname), core::mem::transmute_copy(pdlcontenttype), printticket.param().abi(), pageconfigurationsettings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PdlPassthroughProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPdlPassthroughProvider>();
}
unsafe impl windows_core::Interface for PdlPassthroughProvider {
    type Vtable = <IPdlPassthroughProvider as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPdlPassthroughProvider as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PdlPassthroughProvider {
    const NAME: &'static str = "Windows.Devices.Printers.PdlPassthroughProvider";
}
unsafe impl Send for PdlPassthroughProvider {}
unsafe impl Sync for PdlPassthroughProvider {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PdlPassthroughTarget(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PdlPassthroughTarget, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PdlPassthroughTarget, super::super::Foundation::IClosable);
impl PdlPassthroughTarget {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PrintJobId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintJobId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStream(&self) -> windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOutputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Submit(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Submit)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for PdlPassthroughTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPdlPassthroughTarget>();
}
unsafe impl windows_core::Interface for PdlPassthroughTarget {
    type Vtable = <IPdlPassthroughTarget as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPdlPassthroughTarget as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PdlPassthroughTarget {
    const NAME: &'static str = "Windows.Devices.Printers.PdlPassthroughTarget";
}
unsafe impl Send for PdlPassthroughTarget {}
unsafe impl Sync for PdlPassthroughTarget {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Print3DDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Print3DDevice, windows_core::IUnknown, windows_core::IInspectable);
impl Print3DDevice {
    pub fn PrintSchema(&self) -> windows_core::Result<PrintSchema> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintSchema)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<Print3DDevice>> {
        Self::IPrint3DDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IPrint3DDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    fn IPrint3DDeviceStatics<R, F: FnOnce(&IPrint3DDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Print3DDevice, IPrint3DDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Print3DDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrint3DDevice>();
}
unsafe impl windows_core::Interface for Print3DDevice {
    type Vtable = <IPrint3DDevice as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrint3DDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Print3DDevice {
    const NAME: &'static str = "Windows.Devices.Printers.Print3DDevice";
}
unsafe impl Send for Print3DDevice {}
unsafe impl Sync for Print3DDevice {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintSchema(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSchema, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSchema {
    #[cfg(feature = "Storage_Streams")]
    pub fn GetDefaultPrintTicketAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultPrintTicketAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetCapabilitiesAsync<P0>(&self, constrainticket: P0) -> windows_core::Result<windows_future::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamWithContentType>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCapabilitiesAsync)(windows_core::Interface::as_raw(this), constrainticket.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn MergeAndValidateWithDefaultPrintTicketAsync<P0>(&self, deltaticket: P0) -> windows_core::Result<windows_future::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamWithContentType>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MergeAndValidateWithDefaultPrintTicketAsync)(windows_core::Interface::as_raw(this), deltaticket.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintSchema {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSchema>();
}
unsafe impl windows_core::Interface for PrintSchema {
    type Vtable = <IPrintSchema as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrintSchema as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSchema {
    const NAME: &'static str = "Windows.Devices.Printers.PrintSchema";
}
unsafe impl Send for PrintSchema {}
unsafe impl Sync for PrintSchema {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplaceDevicePropertiesResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ReplaceDevicePropertiesResult, windows_core::IUnknown, windows_core::IInspectable);
impl ReplaceDevicePropertiesResult {
    pub fn Status(&self) -> windows_core::Result<ReplaceDevicePropertiesStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ReplaceDevicePropertiesResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IReplaceDevicePropertiesResult>();
}
unsafe impl windows_core::Interface for ReplaceDevicePropertiesResult {
    type Vtable = <IReplaceDevicePropertiesResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IReplaceDevicePropertiesResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ReplaceDevicePropertiesResult {
    const NAME: &'static str = "Windows.Devices.Printers.ReplaceDevicePropertiesResult";
}
unsafe impl Send for ReplaceDevicePropertiesResult {}
unsafe impl Sync for ReplaceDevicePropertiesResult {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ReplaceDevicePropertiesStatus(pub i32);
impl ReplaceDevicePropertiesStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const OtherFailure: Self = Self(2i32);
}
impl windows_core::TypeKind for ReplaceDevicePropertiesStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ReplaceDevicePropertiesStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.ReplaceDevicePropertiesStatus;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VirtualPrinterInstallationParameters(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VirtualPrinterInstallationParameters, windows_core::IUnknown, windows_core::IInspectable);
impl VirtualPrinterInstallationParameters {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VirtualPrinterInstallationParameters, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PrinterName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrinterName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetPrinterName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrinterName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn OutputFileExtensions(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputFileExtensions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SupportedInputFormats(&self) -> windows_core::Result<windows_collections::IVector<VirtualPrinterSupportedFormat>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedInputFormats)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PrintDeviceCapabilitiesPackageRelativeFilePath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintDeviceCapabilitiesPackageRelativeFilePath)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetPrintDeviceCapabilitiesPackageRelativeFilePath(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrintDeviceCapabilitiesPackageRelativeFilePath)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn PrintDeviceResourcesPackageRelativeFilePath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintDeviceResourcesPackageRelativeFilePath)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetPrintDeviceResourcesPackageRelativeFilePath(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrintDeviceResourcesPackageRelativeFilePath)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn PreferredInputFormat(&self) -> windows_core::Result<VirtualPrinterPreferredInputFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreferredInputFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPreferredInputFormat(&self, value: VirtualPrinterPreferredInputFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPreferredInputFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PrinterUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrinterUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPrinterUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrinterUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn EntryPoint(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EntryPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetEntryPoint(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEntryPoint)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for VirtualPrinterInstallationParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVirtualPrinterInstallationParameters>();
}
unsafe impl windows_core::Interface for VirtualPrinterInstallationParameters {
    type Vtable = <IVirtualPrinterInstallationParameters as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IVirtualPrinterInstallationParameters as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VirtualPrinterInstallationParameters {
    const NAME: &'static str = "Windows.Devices.Printers.VirtualPrinterInstallationParameters";
}
unsafe impl Send for VirtualPrinterInstallationParameters {}
unsafe impl Sync for VirtualPrinterInstallationParameters {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VirtualPrinterInstallationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VirtualPrinterInstallationResult, windows_core::IUnknown, windows_core::IInspectable);
impl VirtualPrinterInstallationResult {
    pub fn Status(&self) -> windows_core::Result<VirtualPrinterInstallationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VirtualPrinterInstallationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVirtualPrinterInstallationResult>();
}
unsafe impl windows_core::Interface for VirtualPrinterInstallationResult {
    type Vtable = <IVirtualPrinterInstallationResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IVirtualPrinterInstallationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VirtualPrinterInstallationResult {
    const NAME: &'static str = "Windows.Devices.Printers.VirtualPrinterInstallationResult";
}
unsafe impl Send for VirtualPrinterInstallationResult {}
unsafe impl Sync for VirtualPrinterInstallationResult {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VirtualPrinterInstallationStatus(pub i32);
impl VirtualPrinterInstallationStatus {
    pub const InstallationSucceeded: Self = Self(0i32);
    pub const PrinterAlreadyInstalled: Self = Self(1i32);
    pub const PrinterInstallationAccessDenied: Self = Self(2i32);
    pub const PrinterInstallationFailed: Self = Self(3i32);
}
impl windows_core::TypeKind for VirtualPrinterInstallationStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for VirtualPrinterInstallationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.VirtualPrinterInstallationStatus;i4)");
}
pub struct VirtualPrinterManager;
impl VirtualPrinterManager {
    pub fn InstallVirtualPrinterAsync<P0>(parameters: P0) -> windows_core::Result<windows_future::IAsyncOperation<VirtualPrinterInstallationResult>>
    where
        P0: windows_core::Param<VirtualPrinterInstallationParameters>,
    {
        Self::IVirtualPrinterManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallVirtualPrinterAsync)(windows_core::Interface::as_raw(this), parameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn InstallVirtualPrinterAsync2<P0>(parameters: P0, apppackagefamilyname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<VirtualPrinterInstallationResult>>
    where
        P0: windows_core::Param<VirtualPrinterInstallationParameters>,
    {
        Self::IVirtualPrinterManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallVirtualPrinterAsync2)(windows_core::Interface::as_raw(this), parameters.param().abi(), core::mem::transmute_copy(apppackagefamilyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn InstallVirtualPrinterForAllUsersAsync<P0>(parameters: P0) -> windows_core::Result<windows_future::IAsyncOperation<VirtualPrinterInstallationResult>>
    where
        P0: windows_core::Param<VirtualPrinterInstallationParameters>,
    {
        Self::IVirtualPrinterManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallVirtualPrinterForAllUsersAsync)(windows_core::Interface::as_raw(this), parameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn InstallVirtualPrinterForAllUsersAsync2<P0>(parameters: P0, apppackagefamilyname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<VirtualPrinterInstallationResult>>
    where
        P0: windows_core::Param<VirtualPrinterInstallationParameters>,
    {
        Self::IVirtualPrinterManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallVirtualPrinterForAllUsersAsync2)(windows_core::Interface::as_raw(this), parameters.param().abi(), core::mem::transmute_copy(apppackagefamilyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FindAllVirtualPrinters() -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        Self::IVirtualPrinterManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllVirtualPrinters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FindAllVirtualPrinters2(apppackagefamilyname: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        Self::IVirtualPrinterManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllVirtualPrinters2)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(apppackagefamilyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RemoveVirtualPrinterAsync(printername: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        Self::IVirtualPrinterManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveVirtualPrinterAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(printername), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RemoveVirtualPrinterForAllUsersAsync(printername: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        Self::IVirtualPrinterManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveVirtualPrinterForAllUsersAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(printername), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IVirtualPrinterManagerStatics<R, F: FnOnce(&IVirtualPrinterManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VirtualPrinterManager, IVirtualPrinterManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for VirtualPrinterManager {
    const NAME: &'static str = "Windows.Devices.Printers.VirtualPrinterManager";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VirtualPrinterPreferredInputFormat(pub i32);
impl VirtualPrinterPreferredInputFormat {
    pub const OpenXps: Self = Self(0i32);
    pub const PostScript: Self = Self(1i32);
}
impl windows_core::TypeKind for VirtualPrinterPreferredInputFormat {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for VirtualPrinterPreferredInputFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.VirtualPrinterPreferredInputFormat;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VirtualPrinterSupportedFormat(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VirtualPrinterSupportedFormat, windows_core::IUnknown, windows_core::IInspectable);
impl VirtualPrinterSupportedFormat {
    pub fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetContentType(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentType)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn MaxSupportedVersion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxSupportedVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetMaxSupportedVersion(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxSupportedVersion)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateInstance(contenttype: &windows_core::HSTRING, maxsupportedversion: &windows_core::HSTRING) -> windows_core::Result<VirtualPrinterSupportedFormat> {
        Self::IVirtualPrinterSupportedFormatFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(contenttype), core::mem::transmute_copy(maxsupportedversion), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IVirtualPrinterSupportedFormatFactory<R, F: FnOnce(&IVirtualPrinterSupportedFormatFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VirtualPrinterSupportedFormat, IVirtualPrinterSupportedFormatFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VirtualPrinterSupportedFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVirtualPrinterSupportedFormat>();
}
unsafe impl windows_core::Interface for VirtualPrinterSupportedFormat {
    type Vtable = <IVirtualPrinterSupportedFormat as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IVirtualPrinterSupportedFormat as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VirtualPrinterSupportedFormat {
    const NAME: &'static str = "Windows.Devices.Printers.VirtualPrinterSupportedFormat";
}
unsafe impl Send for VirtualPrinterSupportedFormat {}
unsafe impl Sync for VirtualPrinterSupportedFormat {}
