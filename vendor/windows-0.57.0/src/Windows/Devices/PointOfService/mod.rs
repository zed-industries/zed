#[cfg(feature = "Devices_PointOfService_Provider")]
pub mod Provider;
windows_core::imp::define_interface!(IBarcodeScanner, IBarcodeScanner_Vtbl, 0xbea33e06_b264_4f03_a9c1_45b20f01134f);
impl windows_core::RuntimeType for IBarcodeScanner {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScanner_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClaimScannerAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CheckHealthAsync: unsafe extern "system" fn(*mut core::ffi::c_void, UnifiedPosHealthCheckLevel, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedSymbologiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedSymbologiesAsync: usize,
    pub IsSymbologySupportedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub RetrieveStatisticsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    RetrieveStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedProfiles: usize,
    pub IsProfileSupported: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub StatusUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStatusUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScanner2, IBarcodeScanner2_Vtbl, 0x89215167_8cee_436d_89ab_8dfb43bb4286);
impl windows_core::RuntimeType for IBarcodeScanner2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScanner2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VideoDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerCapabilities, IBarcodeScannerCapabilities_Vtbl, 0xc60691e4_f2c8_4420_a307_b12ef6622857);
impl windows_core::RuntimeType for IBarcodeScannerCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PowerReportingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UnifiedPosPowerReportingType) -> windows_core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsImagePreviewSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerCapabilities1, IBarcodeScannerCapabilities1_Vtbl, 0x8e5ab3e9_0e2c_472f_a1cc_ee8054b6a684);
impl windows_core::RuntimeType for IBarcodeScannerCapabilities1 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerCapabilities1_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSoftwareTriggerSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerCapabilities2, IBarcodeScannerCapabilities2_Vtbl, 0xf211cfec_e1a1_4ea8_9abc_92b1596270ab);
impl windows_core::RuntimeType for IBarcodeScannerCapabilities2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerCapabilities2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsVideoPreviewSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerDataReceivedEventArgs, IBarcodeScannerDataReceivedEventArgs_Vtbl, 0x4234a7e2_ed97_467d_ad2b_01e44313a929);
impl windows_core::RuntimeType for IBarcodeScannerDataReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerDataReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Report: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerErrorOccurredEventArgs, IBarcodeScannerErrorOccurredEventArgs_Vtbl, 0x2cd2602f_cf3a_4002_a75a_c5ec468f0a20);
impl windows_core::RuntimeType for IBarcodeScannerErrorOccurredEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerErrorOccurredEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PartialInputData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsRetriable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ErrorData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerImagePreviewReceivedEventArgs, IBarcodeScannerImagePreviewReceivedEventArgs_Vtbl, 0xf3b7de85_6e8b_434e_9f58_06ef26bc4baf);
impl windows_core::RuntimeType for IBarcodeScannerImagePreviewReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerImagePreviewReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Preview: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Preview: usize,
}
windows_core::imp::define_interface!(IBarcodeScannerReport, IBarcodeScannerReport_Vtbl, 0x5ce4d8b0_a489_4b96_86c4_f0bf8a37753d);
impl windows_core::RuntimeType for IBarcodeScannerReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ScanDataType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ScanData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ScanData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ScanDataLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ScanDataLabel: usize,
}
windows_core::imp::define_interface!(IBarcodeScannerReportFactory, IBarcodeScannerReportFactory_Vtbl, 0xa2547326_2013_457c_8963_49c15dca78ce);
impl windows_core::RuntimeType for IBarcodeScannerReportFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerReportFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateInstance: usize,
}
windows_core::imp::define_interface!(IBarcodeScannerStatics, IBarcodeScannerStatics_Vtbl, 0x5d115f6f_da49_41e8_8c8c_f0cb62a9c4fc);
impl windows_core::RuntimeType for IBarcodeScannerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerStatics2, IBarcodeScannerStatics2_Vtbl, 0xb8652473_a36f_4007_b1d0_279ebe92a656);
impl windows_core::RuntimeType for IBarcodeScannerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(*mut core::ffi::c_void, PosConnectionTypes, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerStatusUpdatedEventArgs, IBarcodeScannerStatusUpdatedEventArgs_Vtbl, 0x355d8586_9c43_462b_a91a_816dc97f452c);
impl windows_core::RuntimeType for IBarcodeScannerStatusUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerStatusUpdatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BarcodeScannerStatus) -> windows_core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeSymbologiesStatics, IBarcodeSymbologiesStatics_Vtbl, 0xca8549bb_06d2_43f4_a44b_c620679fd8d0);
impl windows_core::RuntimeType for IBarcodeSymbologiesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeSymbologiesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Unknown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Ean8: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Ean8Add2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Ean8Add5: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Eanv: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EanvAdd2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EanvAdd5: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Ean13: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Ean13Add2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Ean13Add5: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Isbn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsbnAdd5: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Ismn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsmnAdd2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsmnAdd5: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Issn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IssnAdd2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IssnAdd5: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Ean99: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Ean99Add2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Ean99Add5: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Upca: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UpcaAdd2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UpcaAdd5: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Upce: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UpceAdd2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UpceAdd5: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UpcCoupon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TfStd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TfDis: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TfInt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TfInd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TfMat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TfIata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Gs1DatabarType1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Gs1DatabarType2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Gs1DatabarType3: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Code39: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Code39Ex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Trioptic39: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Code32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Pzn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Code93: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Code93Ex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Code128: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Gs1128: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Gs1128Coupon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UccEan128: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Sisac: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Isbt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Codabar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Code11: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Msi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Plessey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Telepen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Code16k: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CodablockA: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CodablockF: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Codablock128: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Code49: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Aztec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DataCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DataMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub HanXin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Maxicode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MicroPdf417: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MicroQr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Pdf417: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Qr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MsTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Ccab: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Ccc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Tlc39: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AusPost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CanPost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ChinaPost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DutchKix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub InfoMail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ItalianPost25: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ItalianPost39: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub JapanPost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub KoreanPost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SwedenPost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UkPost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UsIntelligent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UsIntelligentPkg: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UsPlanet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UsPostNet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Us4StateFics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub OcrA: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub OcrB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Micr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ExtendedBase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeSymbologiesStatics2, IBarcodeSymbologiesStatics2_Vtbl, 0x8b7518f4_99d0_40bf_9424_b91d6dd4c6e0);
impl windows_core::RuntimeType for IBarcodeSymbologiesStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeSymbologiesStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Gs1DWCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeSymbologyAttributes, IBarcodeSymbologyAttributes_Vtbl, 0x66413a78_ab7a_4ada_8ece_936014b2ead7);
impl windows_core::RuntimeType for IBarcodeSymbologyAttributes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeSymbologyAttributes_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsCheckDigitValidationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCheckDigitValidationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsCheckDigitValidationSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCheckDigitTransmissionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCheckDigitTransmissionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsCheckDigitTransmissionSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DecodeLength1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDecodeLength1: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DecodeLength2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDecodeLength2: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DecodeLengthKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BarcodeSymbologyDecodeLengthKind) -> windows_core::HRESULT,
    pub SetDecodeLengthKind: unsafe extern "system" fn(*mut core::ffi::c_void, BarcodeSymbologyDecodeLengthKind) -> windows_core::HRESULT,
    pub IsDecodeLengthSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICashDrawer, ICashDrawer_Vtbl, 0x9f88f5c8_de54_4aee_a890_920bcbfe30fc);
impl windows_core::RuntimeType for ICashDrawer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICashDrawer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsDrawerOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DrawerEventSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClaimDrawerAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CheckHealthAsync: unsafe extern "system" fn(*mut core::ffi::c_void, UnifiedPosHealthCheckLevel, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStatisticsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStatisticsAsync: usize,
    pub StatusUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStatusUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICashDrawerCapabilities, ICashDrawerCapabilities_Vtbl, 0x0bc6de0b_e8e7_4b1f_b1d1_3e501ad08247);
impl windows_core::RuntimeType for ICashDrawerCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICashDrawerCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PowerReportingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UnifiedPosPowerReportingType) -> windows_core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsStatusReportingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsStatusMultiDrawerDetectSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDrawerOpenSensorAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICashDrawerCloseAlarm, ICashDrawerCloseAlarm_Vtbl, 0x6bf88cc7_6f63_430e_ab3b_95d75ffbe87f);
impl windows_core::RuntimeType for ICashDrawerCloseAlarm {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICashDrawerCloseAlarm_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetAlarmTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub AlarmTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetBeepFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub BeepFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBeepDuration: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub BeepDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetBeepDelay: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub BeepDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub AlarmTimeoutExpired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAlarmTimeoutExpired: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub StartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICashDrawerEventSource, ICashDrawerEventSource_Vtbl, 0xe006e46c_f2f9_442f_8dd6_06c10a4227ba);
impl windows_core::RuntimeType for ICashDrawerEventSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICashDrawerEventSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DrawerClosed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDrawerClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DrawerOpened: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDrawerOpened: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICashDrawerEventSourceEventArgs, ICashDrawerEventSourceEventArgs_Vtbl, 0x69cb3bc1_147f_421c_9c23_090123bb786c);
impl core::ops::Deref for ICashDrawerEventSourceEventArgs {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICashDrawerEventSourceEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ICashDrawerEventSourceEventArgs {
    pub fn CashDrawer(&self) -> windows_core::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CashDrawer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ICashDrawerEventSourceEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICashDrawerEventSourceEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CashDrawer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICashDrawerStatics, ICashDrawerStatics_Vtbl, 0xdfa0955a_d437_4fff_b547_dda969a4f883);
impl windows_core::RuntimeType for ICashDrawerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICashDrawerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICashDrawerStatics2, ICashDrawerStatics2_Vtbl, 0x3e818121_8c42_40e8_9c0e_40297048104c);
impl windows_core::RuntimeType for ICashDrawerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICashDrawerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(*mut core::ffi::c_void, PosConnectionTypes, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICashDrawerStatus, ICashDrawerStatus_Vtbl, 0x6bbd78bf_dca1_4e06_99eb_5af6a5aec108);
impl windows_core::RuntimeType for ICashDrawerStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICashDrawerStatus_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StatusKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CashDrawerStatusKind) -> windows_core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICashDrawerStatusUpdatedEventArgs, ICashDrawerStatusUpdatedEventArgs_Vtbl, 0x30aae98a_0d70_459c_9553_87e124c52488);
impl windows_core::RuntimeType for ICashDrawerStatusUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICashDrawerStatusUpdatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClaimedBarcodeScanner, IClaimedBarcodeScanner_Vtbl, 0x4a63b49c_8fa4_4332_bb26_945d11d81e0f);
impl windows_core::RuntimeType for IClaimedBarcodeScanner {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedBarcodeScanner_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsDisabledOnDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsDisabledOnDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsDecodeDataEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsDecodeDataEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub EnableAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RetainDevice: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetActiveSymbologiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetActiveSymbologiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateStatisticsAsync: usize,
    pub SetActiveProfileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub TriggerPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTriggerPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub TriggerReleased: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTriggerReleased: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ReleaseDeviceRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ImagePreviewReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveImagePreviewReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ErrorOccurred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveErrorOccurred: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClaimedBarcodeScanner1, IClaimedBarcodeScanner1_Vtbl, 0xf61aad0c_8551_42b4_998c_970c20210a22);
impl windows_core::RuntimeType for IClaimedBarcodeScanner1 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedBarcodeScanner1_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartSoftwareTriggerAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopSoftwareTriggerAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClaimedBarcodeScanner2, IClaimedBarcodeScanner2_Vtbl, 0xe3b59e8c_2d8b_4f70_8af3_3448bedd5fe2);
impl windows_core::RuntimeType for IClaimedBarcodeScanner2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedBarcodeScanner2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetSymbologyAttributesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSymbologyAttributesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClaimedBarcodeScanner3, IClaimedBarcodeScanner3_Vtbl, 0xe6ceb430_712e_45fc_8b86_cd55f5aef79d);
impl windows_core::RuntimeType for IClaimedBarcodeScanner3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedBarcodeScanner3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShowVideoPreviewAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HideVideoPreview: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIsVideoPreviewShownOnEnable: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsVideoPreviewShownOnEnable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClaimedBarcodeScanner4, IClaimedBarcodeScanner4_Vtbl, 0x5d501f97_376a_41a8_a230_2f37c1949dde);
impl windows_core::RuntimeType for IClaimedBarcodeScanner4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedBarcodeScanner4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClaimedBarcodeScannerClosedEventArgs, IClaimedBarcodeScannerClosedEventArgs_Vtbl, 0xcf7d5489_a22c_4c65_a901_88d77d833954);
impl windows_core::RuntimeType for IClaimedBarcodeScannerClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedBarcodeScannerClosedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IClaimedCashDrawer, IClaimedCashDrawer_Vtbl, 0xca3f99af_abb8_42c1_8a84_5c66512f5a75);
impl windows_core::RuntimeType for IClaimedCashDrawer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedCashDrawer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDrawerOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CloseAlarm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenDrawerAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RetainDeviceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateStatisticsAsync: usize,
    pub ReleaseDeviceRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClaimedCashDrawer2, IClaimedCashDrawer2_Vtbl, 0x9cbab5a2_de42_4d5b_b0c1_9b57a2ba89c3);
impl windows_core::RuntimeType for IClaimedCashDrawer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedCashDrawer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClaimedCashDrawerClosedEventArgs, IClaimedCashDrawerClosedEventArgs_Vtbl, 0xcc573f33_3f34_4c5c_baae_deadf16cd7fa);
impl windows_core::RuntimeType for IClaimedCashDrawerClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedCashDrawerClosedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IClaimedJournalPrinter, IClaimedJournalPrinter_Vtbl, 0x67ea0630_517d_487f_9fdf_d2e0a0a264a5);
impl windows_core::RuntimeType for IClaimedJournalPrinter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedJournalPrinter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClaimedLineDisplay, IClaimedLineDisplay_Vtbl, 0x120ac970_9a75_4acf_aae7_09972bcf8794);
impl windows_core::RuntimeType for IClaimedLineDisplay {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedLineDisplay_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PhysicalDeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PhysicalDeviceDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DeviceControlDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DeviceControlVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DeviceServiceVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DefaultWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RetainDevice: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseDeviceRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClaimedLineDisplay2, IClaimedLineDisplay2_Vtbl, 0xa31c75ed_41f5_4e76_a074_795e47a46e97);
impl windows_core::RuntimeType for IClaimedLineDisplay2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedLineDisplay2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStatisticsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStatisticsAsync: usize,
    pub CheckHealthAsync: unsafe extern "system" fn(*mut core::ffi::c_void, UnifiedPosHealthCheckLevel, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CheckPowerStatusAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StatusUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStatusUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedScreenSizesInCharacters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedScreenSizesInCharacters: usize,
    pub MaxBitmapSizeInPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCharacterSets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCharacterSets: usize,
    pub CustomGlyphs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryUpdateAttributesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrySetDescriptorAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, LineDisplayDescriptorState, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryClearDescriptorsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryCreateWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect, super::super::Foundation::Size, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub TryStoreStorageFileBitmapAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    TryStoreStorageFileBitmapAsync: usize,
    #[cfg(feature = "Storage")]
    pub TryStoreStorageFileBitmapWithAlignmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, LineDisplayHorizontalAlignment, LineDisplayVerticalAlignment, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    TryStoreStorageFileBitmapWithAlignmentAsync: usize,
    #[cfg(feature = "Storage")]
    pub TryStoreStorageFileBitmapWithAlignmentAndWidthAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, LineDisplayHorizontalAlignment, LineDisplayVerticalAlignment, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    TryStoreStorageFileBitmapWithAlignmentAndWidthAsync: usize,
}
windows_core::imp::define_interface!(IClaimedLineDisplay3, IClaimedLineDisplay3_Vtbl, 0x642ecd92_e9d4_4ecc_af75_329c274cd18f);
impl windows_core::RuntimeType for IClaimedLineDisplay3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedLineDisplay3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClaimedLineDisplayClosedEventArgs, IClaimedLineDisplayClosedEventArgs_Vtbl, 0xf915f364_d3d5_4f10_b511_90939edfacd8);
impl windows_core::RuntimeType for IClaimedLineDisplayClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedLineDisplayClosedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IClaimedLineDisplayStatics, IClaimedLineDisplayStatics_Vtbl, 0x78ca98fb_8b6b_4973_86f0_3e570c351825);
impl windows_core::RuntimeType for IClaimedLineDisplayStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedLineDisplayStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(*mut core::ffi::c_void, PosConnectionTypes, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClaimedMagneticStripeReader, IClaimedMagneticStripeReader_Vtbl, 0x475ca8f3_9417_48bc_b9d7_4163a7844c02);
impl windows_core::RuntimeType for IClaimedMagneticStripeReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedMagneticStripeReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsDisabledOnDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsDisabledOnDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsDecodeDataEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsDecodeDataEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDeviceAuthenticated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDataEncryptionAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DataEncryptionAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTracksToRead: unsafe extern "system" fn(*mut core::ffi::c_void, MagneticStripeReaderTrackIds) -> windows_core::HRESULT,
    pub TracksToRead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MagneticStripeReaderTrackIds) -> windows_core::HRESULT,
    pub SetIsTransmitSentinelsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsTransmitSentinelsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub EnableAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RetainDevice: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetErrorReportingType: unsafe extern "system" fn(*mut core::ffi::c_void, MagneticStripeReaderErrorReportingType) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub RetrieveDeviceAuthenticationDataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RetrieveDeviceAuthenticationDataAsync: usize,
    pub AuthenticateDeviceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeAuthenticateDeviceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateKeyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateStatisticsAsync: usize,
    pub BankCardDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveBankCardDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AamvaCardDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAamvaCardDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub VendorSpecificDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveVendorSpecificDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ReleaseDeviceRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ErrorOccurred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveErrorOccurred: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClaimedMagneticStripeReader2, IClaimedMagneticStripeReader2_Vtbl, 0x236fafdf_e2dc_4d7d_9c78_060df2bf2928);
impl windows_core::RuntimeType for IClaimedMagneticStripeReader2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedMagneticStripeReader2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClaimedMagneticStripeReaderClosedEventArgs, IClaimedMagneticStripeReaderClosedEventArgs_Vtbl, 0x14ada93a_adcd_4c80_acda_c3eaed2647e1);
impl windows_core::RuntimeType for IClaimedMagneticStripeReaderClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedMagneticStripeReaderClosedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IClaimedPosPrinter, IClaimedPosPrinter_Vtbl, 0x6d64ce0c_e03e_4b14_a38e_c28c34b86353);
impl windows_core::RuntimeType for IClaimedPosPrinter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedPosPrinter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCharacterSet: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CharacterSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsCoverOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCharacterSetMappingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsCharacterSetMappingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetMapMode: unsafe extern "system" fn(*mut core::ffi::c_void, PosPrinterMapMode) -> windows_core::HRESULT,
    pub MapMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PosPrinterMapMode) -> windows_core::HRESULT,
    pub Receipt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Slip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RetainDeviceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateStatisticsAsync: usize,
    pub ReleaseDeviceRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClaimedPosPrinter2, IClaimedPosPrinter2_Vtbl, 0x5bf7a3d5_5198_437a_82df_589993fa77e1);
impl windows_core::RuntimeType for IClaimedPosPrinter2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedPosPrinter2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClaimedPosPrinterClosedEventArgs, IClaimedPosPrinterClosedEventArgs_Vtbl, 0xe2b7a27b_4d40_471d_92ed_63375b18c788);
impl windows_core::RuntimeType for IClaimedPosPrinterClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedPosPrinterClosedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IClaimedReceiptPrinter, IClaimedReceiptPrinter_Vtbl, 0x9ad27a74_dd61_4ee2_9837_5b5d72d538b9);
impl windows_core::RuntimeType for IClaimedReceiptPrinter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedReceiptPrinter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SidewaysMaxLines: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SidewaysMaxChars: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub LinesToPaperCut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub PrintArea: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub CreateJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClaimedSlipPrinter, IClaimedSlipPrinter_Vtbl, 0xbd5deff2_af90_4e8a_b77b_e3ae9ca63a7f);
impl windows_core::RuntimeType for IClaimedSlipPrinter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClaimedSlipPrinter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SidewaysMaxLines: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SidewaysMaxChars: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxLines: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub LinesNearEndToEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PrintSide: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PosPrinterPrintSide) -> windows_core::HRESULT,
    pub PageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub PrintArea: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub OpenJaws: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseJaws: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertSlipAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveSlipAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChangePrintSide: unsafe extern "system" fn(*mut core::ffi::c_void, PosPrinterPrintSide) -> windows_core::HRESULT,
    pub CreateJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICommonClaimedPosPrinterStation, ICommonClaimedPosPrinterStation_Vtbl, 0xb7eb66a8_fe8a_4cfb_8b42_e35b280cb27c);
impl core::ops::Deref for ICommonClaimedPosPrinterStation {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICommonClaimedPosPrinterStation, windows_core::IUnknown, windows_core::IInspectable);
impl ICommonClaimedPosPrinterStation {
    pub fn SetCharactersPerLine(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCharactersPerLine)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharactersPerLine(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharactersPerLine)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLineHeight(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLineHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineHeight(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLineSpacing(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLineSpacing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineSpacing(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineSpacing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LineWidth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsLetterQuality(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsLetterQuality)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsLetterQuality(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLetterQuality)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperNearEnd(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperNearEnd)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetColorCartridge)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ColorCartridge(&self) -> windows_core::Result<PosPrinterColorCartridge> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorCartridge)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCoverOpen(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCoverOpen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCartridgeRemoved(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCartridgeRemoved)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCartridgeEmpty(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCartridgeEmpty)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsHeadCleaning(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHeadCleaning)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperEmpty(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperEmpty)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsReadyToPrint(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReadyToPrint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ValidateData(&self, data: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValidateData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ICommonClaimedPosPrinterStation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICommonClaimedPosPrinterStation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetCharactersPerLine: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CharactersPerLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub LineHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub LineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub LineWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetIsLetterQuality: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsLetterQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPaperNearEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetColorCartridge: unsafe extern "system" fn(*mut core::ffi::c_void, PosPrinterColorCartridge) -> windows_core::HRESULT,
    pub ColorCartridge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PosPrinterColorCartridge) -> windows_core::HRESULT,
    pub IsCoverOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCartridgeRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCartridgeEmpty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsHeadCleaning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPaperEmpty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsReadyToPrint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ValidateData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICommonPosPrintStationCapabilities, ICommonPosPrintStationCapabilities_Vtbl, 0xde5b52ca_e02e_40e9_9e5e_1b488e6aacfc);
impl core::ops::Deref for ICommonPosPrintStationCapabilities {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICommonPosPrintStationCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl ICommonPosPrintStationCapabilities {
    pub fn IsPrinterPresent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPrinterPresent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDualColorSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDualColorSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ColorCartridgeCapabilities(&self) -> windows_core::Result<PosPrinterColorCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorCartridgeCapabilities)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CartridgeSensors(&self) -> windows_core::Result<PosPrinterCartridgeSensors> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CartridgeSensors)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsBoldSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBoldSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsItalicSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsItalicSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUnderlineSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDoubleHighPrintSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleHighPrintSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDoubleWidePrintSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleWidePrintSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleHighDoubleWidePrintSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperEmptySensorSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperEmptySensorSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperNearEndSensorSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperNearEndSensorSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharactersPerLine(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedCharactersPerLine)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ICommonPosPrintStationCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICommonPosPrintStationCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPrinterPresent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDualColorSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ColorCartridgeCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PosPrinterColorCapabilities) -> windows_core::HRESULT,
    pub CartridgeSensors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PosPrinterCartridgeSensors) -> windows_core::HRESULT,
    pub IsBoldSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsItalicSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsUnderlineSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDoubleHighPrintSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDoubleWidePrintSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDoubleHighDoubleWidePrintSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPaperEmptySensorSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPaperNearEndSensorSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCharactersPerLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCharactersPerLine: usize,
}
windows_core::imp::define_interface!(ICommonReceiptSlipCapabilities, ICommonReceiptSlipCapabilities_Vtbl, 0x09286b8b_9873_4d05_bfbe_4727a6038f69);
impl core::ops::Deref for ICommonReceiptSlipCapabilities {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICommonReceiptSlipCapabilities, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ICommonReceiptSlipCapabilities, ICommonPosPrintStationCapabilities);
impl ICommonReceiptSlipCapabilities {
    pub fn IsBarcodeSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBarcodeSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsBitmapSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBitmapSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsLeft90RotationSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLeft90RotationSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRight90RotationSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRight90RotationSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Is180RotationSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Is180RotationSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPrintAreaSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPrintAreaSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RuledLineCapabilities(&self) -> windows_core::Result<PosPrinterRuledLineCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RuledLineCapabilities)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBarcodeRotations(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedBarcodeRotations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBitmapRotations(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedBitmapRotations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPrinterPresent(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPrinterPresent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDualColorSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDualColorSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ColorCartridgeCapabilities(&self) -> windows_core::Result<PosPrinterColorCapabilities> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorCartridgeCapabilities)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CartridgeSensors(&self) -> windows_core::Result<PosPrinterCartridgeSensors> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CartridgeSensors)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsBoldSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBoldSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsItalicSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsItalicSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUnderlineSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDoubleHighPrintSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleHighPrintSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDoubleWidePrintSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleWidePrintSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleHighDoubleWidePrintSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperEmptySensorSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperEmptySensorSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperNearEndSensorSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperNearEndSensorSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharactersPerLine(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedCharactersPerLine)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ICommonReceiptSlipCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICommonReceiptSlipCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsBarcodeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsBitmapSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsLeft90RotationSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsRight90RotationSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Is180RotationSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPrintAreaSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub RuledLineCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PosPrinterRuledLineCapabilities) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedBarcodeRotations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedBarcodeRotations: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedBitmapRotations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedBitmapRotations: usize,
}
windows_core::imp::define_interface!(IJournalPrintJob, IJournalPrintJob_Vtbl, 0x9f4f2864_f3f0_55d0_8c39_74cc91783eed);
impl windows_core::RuntimeType for IJournalPrintJob {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IJournalPrintJob_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Print: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FeedPaperByLine: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub FeedPaperByMapModeUnit: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IJournalPrinterCapabilities, IJournalPrinterCapabilities_Vtbl, 0x3b5ccc43_e047_4463_bb58_17b5ba1d8056);
impl windows_core::RuntimeType for IJournalPrinterCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IJournalPrinterCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IJournalPrinterCapabilities2, IJournalPrinterCapabilities2_Vtbl, 0x03b0b645_33b8_533b_baaa_a4389283ab0a);
impl windows_core::RuntimeType for IJournalPrinterCapabilities2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IJournalPrinterCapabilities2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsReverseVideoSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsStrikethroughSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSuperscriptSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSubscriptSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsReversePaperFeedByLineSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsReversePaperFeedByMapModeUnitSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILineDisplay, ILineDisplay_Vtbl, 0x24f5df4e_3c99_44e2_b73f_e51be3637a8c);
impl windows_core::RuntimeType for ILineDisplay {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILineDisplay_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PhysicalDeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PhysicalDeviceDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DeviceControlDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DeviceControlVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DeviceServiceVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ClaimAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILineDisplay2, ILineDisplay2_Vtbl, 0xc296a628_ef44_40f3_bd1c_b04c6a5cdc7d);
impl windows_core::RuntimeType for ILineDisplay2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILineDisplay2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CheckPowerStatusAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILineDisplayAttributes, ILineDisplayAttributes_Vtbl, 0xc17de99c_229a_4c14_a6f1_b4e4b1fead92);
impl windows_core::RuntimeType for ILineDisplayAttributes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILineDisplayAttributes_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPowerNotifyEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsPowerNotifyEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Brightness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBrightness: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BlinkRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetBlinkRate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub ScreenSizeInCharacters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub SetScreenSizeInCharacters: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Size) -> windows_core::HRESULT,
    pub CharacterSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCharacterSet: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsCharacterSetMappingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCharacterSetMappingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CurrentWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCurrentWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILineDisplayCapabilities, ILineDisplayCapabilities_Vtbl, 0x5a15b5d1_8dc5_4b9c_9172_303e47b70c55);
impl windows_core::RuntimeType for ILineDisplayCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILineDisplayCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PowerReportingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UnifiedPosPowerReportingType) -> windows_core::HRESULT,
    pub CanChangeScreenSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanDisplayBitmaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanReadCharacterAtCursor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanMapCharacterSets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanDisplayCustomGlyphs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanReverse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LineDisplayTextAttributeGranularity) -> windows_core::HRESULT,
    pub CanBlink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LineDisplayTextAttributeGranularity) -> windows_core::HRESULT,
    pub CanChangeBlinkRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsBrightnessSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCursorSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsHorizontalMarqueeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsVerticalMarqueeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsInterCharacterWaitSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SupportedDescriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SupportedWindows: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILineDisplayCursor, ILineDisplayCursor_Vtbl, 0xecdffc45_754a_4e3b_ab2b_151181085605);
impl windows_core::RuntimeType for ILineDisplayCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILineDisplayCursor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CanCustomize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsBlinkSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsBlockSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsHalfBlockSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsUnderlineSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsReverseSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsOtherSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryUpdateAttributesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILineDisplayCursorAttributes, ILineDisplayCursorAttributes_Vtbl, 0x4e2d54fe_4ffd_4190_aae1_ce285f20c896);
impl windows_core::RuntimeType for ILineDisplayCursorAttributes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILineDisplayCursorAttributes_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsBlinkEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsBlinkEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CursorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LineDisplayCursorType) -> windows_core::HRESULT,
    pub SetCursorType: unsafe extern "system" fn(*mut core::ffi::c_void, LineDisplayCursorType) -> windows_core::HRESULT,
    pub IsAutoAdvanceEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsAutoAdvanceEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILineDisplayCustomGlyphs, ILineDisplayCustomGlyphs_Vtbl, 0x2257f63c_f263_44f1_a1a0_e750a6a0ec54);
impl windows_core::RuntimeType for ILineDisplayCustomGlyphs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILineDisplayCustomGlyphs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SizeInPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedGlyphCodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedGlyphCodes: usize,
    #[cfg(feature = "Storage_Streams")]
    pub TryRedefineAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TryRedefineAsync: usize,
}
windows_core::imp::define_interface!(ILineDisplayMarquee, ILineDisplayMarquee_Vtbl, 0xa3d33e3e_f46a_4b7a_bc21_53eb3b57f8b4);
impl windows_core::RuntimeType for ILineDisplayMarquee {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILineDisplayMarquee_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LineDisplayMarqueeFormat) -> windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, LineDisplayMarqueeFormat) -> windows_core::HRESULT,
    pub RepeatWaitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetRepeatWaitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub ScrollWaitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetScrollWaitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub TryStartScrollingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, LineDisplayScrollDirection, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryStopScrollingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILineDisplayStatics, ILineDisplayStatics_Vtbl, 0x022dc0b6_11b0_4690_9547_0b39c5af2114);
impl windows_core::RuntimeType for ILineDisplayStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILineDisplayStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(*mut core::ffi::c_void, PosConnectionTypes, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILineDisplayStatics2, ILineDisplayStatics2_Vtbl, 0x600c3f1c_77ab_4968_a7de_c02ff169f2cc);
impl windows_core::RuntimeType for ILineDisplayStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILineDisplayStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StatisticsCategorySelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILineDisplayStatisticsCategorySelector, ILineDisplayStatisticsCategorySelector_Vtbl, 0xb521c46b_9274_4d24_94f3_b6017b832444);
impl windows_core::RuntimeType for ILineDisplayStatisticsCategorySelector {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILineDisplayStatisticsCategorySelector_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub UnifiedPosStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ManufacturerStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILineDisplayStatusUpdatedEventArgs, ILineDisplayStatusUpdatedEventArgs_Vtbl, 0xddd57c1a_86fb_4eba_93d1_6f5eda52b752);
impl windows_core::RuntimeType for ILineDisplayStatusUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILineDisplayStatusUpdatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LineDisplayPowerStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILineDisplayStoredBitmap, ILineDisplayStoredBitmap_Vtbl, 0xf621515b_d81e_43ba_bf1b_bcfa3c785ba0);
impl windows_core::RuntimeType for ILineDisplayStoredBitmap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILineDisplayStoredBitmap_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EscapeSequence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TryDeleteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILineDisplayWindow, ILineDisplayWindow_Vtbl, 0xd21feef4_2364_4be5_bee1_851680af4964);
impl windows_core::RuntimeType for ILineDisplayWindow {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILineDisplayWindow_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SizeInCharacters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub InterCharacterWaitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetInterCharacterWaitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub TryRefreshAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryDisplayTextAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, LineDisplayTextAttribute, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryDisplayTextAtPositionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, LineDisplayTextAttribute, super::super::Foundation::Point, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryDisplayTextNormalAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryScrollTextAsync: unsafe extern "system" fn(*mut core::ffi::c_void, LineDisplayScrollDirection, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryClearTextAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILineDisplayWindow2, ILineDisplayWindow2_Vtbl, 0xa95ce2e6_bdd8_4365_8e11_de94de8dff02);
impl windows_core::RuntimeType for ILineDisplayWindow2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILineDisplayWindow2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Cursor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Marquee: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadCharacterAtCursorAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryDisplayStoredBitmapAtCursorAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub TryDisplayStorageFileBitmapAtCursorAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    TryDisplayStorageFileBitmapAtCursorAsync: usize,
    #[cfg(feature = "Storage")]
    pub TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, LineDisplayHorizontalAlignment, LineDisplayVerticalAlignment, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync: usize,
    #[cfg(feature = "Storage")]
    pub TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, LineDisplayHorizontalAlignment, LineDisplayVerticalAlignment, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync: usize,
    #[cfg(feature = "Storage")]
    pub TryDisplayStorageFileBitmapAtPointAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Point, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    TryDisplayStorageFileBitmapAtPointAsync: usize,
    #[cfg(feature = "Storage")]
    pub TryDisplayStorageFileBitmapAtPointWithWidthAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Point, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    TryDisplayStorageFileBitmapAtPointWithWidthAsync: usize,
}
windows_core::imp::define_interface!(IMagneticStripeReader, IMagneticStripeReader_Vtbl, 0x1a92b015_47c3_468a_9333_0c6517574883);
impl windows_core::RuntimeType for IMagneticStripeReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagneticStripeReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportedCardTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u32) -> windows_core::HRESULT,
    pub DeviceAuthenticationProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MagneticStripeReaderAuthenticationProtocol) -> windows_core::HRESULT,
    pub CheckHealthAsync: unsafe extern "system" fn(*mut core::ffi::c_void, UnifiedPosHealthCheckLevel, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClaimReaderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub RetrieveStatisticsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    RetrieveStatisticsAsync: usize,
    pub GetErrorReportingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MagneticStripeReaderErrorReportingType) -> windows_core::HRESULT,
    pub StatusUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStatusUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagneticStripeReaderAamvaCardDataReceivedEventArgs, IMagneticStripeReaderAamvaCardDataReceivedEventArgs_Vtbl, 0x0a4bbd51_c316_4910_87f3_7a62ba862d31);
impl windows_core::RuntimeType for IMagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagneticStripeReaderAamvaCardDataReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Report: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LicenseNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Restrictions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Class: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Endorsements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub BirthDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FirstName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Surname: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Suffix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Gender: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub HairColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EyeColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Weight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub City: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagneticStripeReaderBankCardDataReceivedEventArgs, IMagneticStripeReaderBankCardDataReceivedEventArgs_Vtbl, 0x2e958823_a31a_4763_882c_23725e39b08e);
impl windows_core::RuntimeType for IMagneticStripeReaderBankCardDataReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagneticStripeReaderBankCardDataReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Report: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AccountNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ServiceCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FirstName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MiddleInitial: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Surname: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Suffix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagneticStripeReaderCapabilities, IMagneticStripeReaderCapabilities_Vtbl, 0x7128809c_c440_44a2_a467_469175d02896);
impl windows_core::RuntimeType for IMagneticStripeReaderCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagneticStripeReaderCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CardAuthentication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SupportedEncryptionAlgorithms: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MagneticStripeReaderAuthenticationLevel) -> windows_core::HRESULT,
    pub IsIsoSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsJisOneSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsJisTwoSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PowerReportingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UnifiedPosPowerReportingType) -> windows_core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsTrackDataMaskingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsTransmitSentinelsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagneticStripeReaderCardTypesStatics, IMagneticStripeReaderCardTypesStatics_Vtbl, 0x528f2c5d_2986_474f_8454_7ccd05928d5f);
impl windows_core::RuntimeType for IMagneticStripeReaderCardTypesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagneticStripeReaderCardTypesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Unknown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Bank: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Aamva: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ExtendedBase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagneticStripeReaderEncryptionAlgorithmsStatics, IMagneticStripeReaderEncryptionAlgorithmsStatics_Vtbl, 0x53b57350_c3db_4754_9c00_41392374a109);
impl windows_core::RuntimeType for IMagneticStripeReaderEncryptionAlgorithmsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagneticStripeReaderEncryptionAlgorithmsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub None: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TripleDesDukpt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ExtendedBase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagneticStripeReaderErrorOccurredEventArgs, IMagneticStripeReaderErrorOccurredEventArgs_Vtbl, 0x1fedf95d_2c84_41ad_b778_f2356a789ab1);
impl windows_core::RuntimeType for IMagneticStripeReaderErrorOccurredEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagneticStripeReaderErrorOccurredEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Track1Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MagneticStripeReaderTrackErrorType) -> windows_core::HRESULT,
    pub Track2Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MagneticStripeReaderTrackErrorType) -> windows_core::HRESULT,
    pub Track3Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MagneticStripeReaderTrackErrorType) -> windows_core::HRESULT,
    pub Track4Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MagneticStripeReaderTrackErrorType) -> windows_core::HRESULT,
    pub ErrorData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PartialInputData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagneticStripeReaderReport, IMagneticStripeReaderReport_Vtbl, 0x6a5b6047_99b0_4188_bef1_eddf79f78fe6);
impl windows_core::RuntimeType for IMagneticStripeReaderReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagneticStripeReaderReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CardType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Track1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Track2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Track3: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Track4: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CardAuthenticationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CardAuthenticationData: usize,
    pub CardAuthenticationDataLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AdditionalSecurityInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AdditionalSecurityInformation: usize,
}
windows_core::imp::define_interface!(IMagneticStripeReaderStatics, IMagneticStripeReaderStatics_Vtbl, 0xc45fab4a_efd7_4760_a5ce_15b0e47e94eb);
impl windows_core::RuntimeType for IMagneticStripeReaderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagneticStripeReaderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagneticStripeReaderStatics2, IMagneticStripeReaderStatics2_Vtbl, 0x8cadc362_d667_48fa_86bc_f5ae1189262b);
impl windows_core::RuntimeType for IMagneticStripeReaderStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagneticStripeReaderStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(*mut core::ffi::c_void, PosConnectionTypes, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagneticStripeReaderStatusUpdatedEventArgs, IMagneticStripeReaderStatusUpdatedEventArgs_Vtbl, 0x09cc6bb0_3262_401d_9e8a_e80d6358906b);
impl windows_core::RuntimeType for IMagneticStripeReaderStatusUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagneticStripeReaderStatusUpdatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MagneticStripeReaderStatus) -> windows_core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagneticStripeReaderTrackData, IMagneticStripeReaderTrackData_Vtbl, 0x104cf671_4a9d_446e_abc5_20402307ba36);
impl windows_core::RuntimeType for IMagneticStripeReaderTrackData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagneticStripeReaderTrackData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DiscretionaryData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DiscretionaryData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub EncryptedData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    EncryptedData: usize,
}
windows_core::imp::define_interface!(IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs, IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_Vtbl, 0xaf0a5514_59cc_4a60_99e8_99a53dace5aa);
impl windows_core::RuntimeType for IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Report: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPosPrinter, IPosPrinter_Vtbl, 0x2a03c10e_9a19_4a01_994f_12dfad6adcbf);
impl windows_core::RuntimeType for IPosPrinter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPosPrinter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCharacterSets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCharacterSets: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedTypeFaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedTypeFaces: usize,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClaimPrinterAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CheckHealthAsync: unsafe extern "system" fn(*mut core::ffi::c_void, UnifiedPosHealthCheckLevel, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStatisticsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStatisticsAsync: usize,
    pub StatusUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStatusUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPosPrinter2, IPosPrinter2_Vtbl, 0x248475e8_8b98_5517_8e48_760e86f68987);
impl windows_core::RuntimeType for IPosPrinter2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPosPrinter2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedBarcodeSymbologies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedBarcodeSymbologies: usize,
    pub GetFontProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPosPrinterCapabilities, IPosPrinterCapabilities_Vtbl, 0xcde95721_4380_4985_adc5_39db30cd93bc);
impl windows_core::RuntimeType for IPosPrinterCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPosPrinterCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PowerReportingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UnifiedPosPowerReportingType) -> windows_core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DefaultCharacterSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub HasCoverSensor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanMapCharacterSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsTransactionSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Receipt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Slip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPosPrinterCharacterSetIdsStatics, IPosPrinterCharacterSetIdsStatics_Vtbl, 0x5c709eff_709a_4fe7_b215_06a748a38b39);
impl windows_core::RuntimeType for IPosPrinterCharacterSetIdsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPosPrinterCharacterSetIdsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Utf16LE: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Ascii: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Ansi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPosPrinterFontProperty, IPosPrinterFontProperty_Vtbl, 0xa7f4e93a_f8ac_5f04_84d2_29b16d8a633c);
impl windows_core::RuntimeType for IPosPrinterFontProperty {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPosPrinterFontProperty_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TypeFace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsScalableToAnySize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CharacterSizes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CharacterSizes: usize,
}
windows_core::imp::define_interface!(IPosPrinterJob, IPosPrinterJob_Vtbl, 0x9a94005c_0615_4591_a58f_30f87edfe2e4);
impl core::ops::Deref for IPosPrinterJob {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPosPrinterJob, windows_core::IUnknown, windows_core::IInspectable);
impl IPosPrinterJob {
    pub fn Print(&self, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Print)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintLine(&self, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintLine)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintNewline(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintNewline)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ExecuteAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExecuteAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IPosPrinterJob {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPosPrinterJob_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Print: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PrintLine: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PrintNewline: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecuteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPosPrinterPrintOptions, IPosPrinterPrintOptions_Vtbl, 0x0a2e16fd_1d02_5a58_9d59_bfcde76fde86);
impl windows_core::RuntimeType for IPosPrinterPrintOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPosPrinterPrintOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TypeFace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTypeFace: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CharacterHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCharacterHeight: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Bold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetBold: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Italic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetItalic: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Underline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ReverseVideo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetReverseVideo: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Strikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetStrikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Superscript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetSuperscript: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Subscript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetSubscript: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub DoubleWide: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDoubleWide: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub DoubleHigh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDoubleHigh: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Alignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PosPrinterAlignment) -> windows_core::HRESULT,
    pub SetAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, PosPrinterAlignment) -> windows_core::HRESULT,
    pub CharacterSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCharacterSet: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPosPrinterReleaseDeviceRequestedEventArgs, IPosPrinterReleaseDeviceRequestedEventArgs_Vtbl, 0x2bcba359_1cef_40b2_9ecb_f927f856ae3c);
impl windows_core::RuntimeType for IPosPrinterReleaseDeviceRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPosPrinterReleaseDeviceRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPosPrinterStatics, IPosPrinterStatics_Vtbl, 0x8ce0d4ea_132f_4cdf_a64a_2d0d7c96a85b);
impl windows_core::RuntimeType for IPosPrinterStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPosPrinterStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPosPrinterStatics2, IPosPrinterStatics2_Vtbl, 0xeecd2c1c_b0d0_42e7_b137_b89b16244d41);
impl windows_core::RuntimeType for IPosPrinterStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPosPrinterStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(*mut core::ffi::c_void, PosConnectionTypes, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPosPrinterStatus, IPosPrinterStatus_Vtbl, 0xd1f0c730_da40_4328_bf76_5156fa33b747);
impl windows_core::RuntimeType for IPosPrinterStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPosPrinterStatus_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StatusKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PosPrinterStatusKind) -> windows_core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPosPrinterStatusUpdatedEventArgs, IPosPrinterStatusUpdatedEventArgs_Vtbl, 0x2edb87df_13a6_428d_ba81_b0e7c3e5a3cd);
impl windows_core::RuntimeType for IPosPrinterStatusUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPosPrinterStatusUpdatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IReceiptOrSlipJob, IReceiptOrSlipJob_Vtbl, 0x532199be_c8c3_4dc2_89e9_5c4a37b34ddc);
impl core::ops::Deref for IReceiptOrSlipJob {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IReceiptOrSlipJob, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IReceiptOrSlipJob, IPosPrinterJob);
impl IReceiptOrSlipJob {
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBarcodeRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrintRotation)(windows_core::Interface::as_raw(this), value, includebitmaps).ok() }
    }
    pub fn SetPrintArea(&self, value: super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrintArea)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmap<P0>(&self, bitmapnumber: u32, bitmap: P0, alignment: PosPrinterAlignment) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBitmap)(windows_core::Interface::as_raw(this), bitmapnumber, bitmap.param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthStandardAlign<P0>(&self, bitmapnumber: u32, bitmap: P0, alignment: PosPrinterAlignment, width: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBitmapCustomWidthStandardAlign)(windows_core::Interface::as_raw(this), bitmapnumber, bitmap.param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetCustomAlignedBitmap<P0>(&self, bitmapnumber: u32, bitmap: P0, alignmentdistance: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCustomAlignedBitmap)(windows_core::Interface::as_raw(this), bitmapnumber, bitmap.param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthCustomAlign<P0>(&self, bitmapnumber: u32, bitmap: P0, alignmentdistance: u32, width: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBitmapCustomWidthCustomAlign)(windows_core::Interface::as_raw(this), bitmapnumber, bitmap.param().abi(), alignmentdistance, width).ok() }
    }
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintSavedBitmap)(windows_core::Interface::as_raw(this), bitmapnumber).ok() }
    }
    pub fn DrawRuledLine(&self, positionlist: &windows_core::HSTRING, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DrawRuledLine)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(positionlist), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    pub fn PrintBarcode(&self, data: &windows_core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintBarcode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data), symbology, height, width, textposition, alignment).ok() }
    }
    pub fn PrintBarcodeCustomAlign(&self, data: &windows_core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintBarcodeCustomAlign)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmap<P0>(&self, bitmap: P0, alignment: PosPrinterAlignment) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintBitmap)(windows_core::Interface::as_raw(this), bitmap.param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthStandardAlign<P0>(&self, bitmap: P0, alignment: PosPrinterAlignment, width: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintBitmapCustomWidthStandardAlign)(windows_core::Interface::as_raw(this), bitmap.param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintCustomAlignedBitmap<P0>(&self, bitmap: P0, alignmentdistance: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintCustomAlignedBitmap)(windows_core::Interface::as_raw(this), bitmap.param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthCustomAlign<P0>(&self, bitmap: P0, alignmentdistance: u32, width: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintBitmapCustomWidthCustomAlign)(windows_core::Interface::as_raw(this), bitmap.param().abi(), alignmentdistance, width).ok() }
    }
    pub fn Print(&self, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Print)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintLine(&self, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PrintLine)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintNewline(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PrintNewline)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ExecuteAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExecuteAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IReceiptOrSlipJob {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IReceiptOrSlipJob_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetBarcodeRotation: unsafe extern "system" fn(*mut core::ffi::c_void, PosPrinterRotation) -> windows_core::HRESULT,
    pub SetPrintRotation: unsafe extern "system" fn(*mut core::ffi::c_void, PosPrinterRotation, bool) -> windows_core::HRESULT,
    pub SetPrintArea: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, PosPrinterAlignment) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetBitmapCustomWidthStandardAlign: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, PosPrinterAlignment, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetBitmapCustomWidthStandardAlign: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetCustomAlignedBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetCustomAlignedBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetBitmapCustomWidthCustomAlign: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetBitmapCustomWidthCustomAlign: usize,
    pub PrintSavedBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DrawRuledLine: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, PosPrinterLineDirection, u32, PosPrinterLineStyle, u32) -> windows_core::HRESULT,
    pub PrintBarcode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, u32, u32, PosPrinterBarcodeTextPosition, PosPrinterAlignment) -> windows_core::HRESULT,
    pub PrintBarcodeCustomAlign: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, u32, u32, PosPrinterBarcodeTextPosition, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub PrintBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PosPrinterAlignment) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    PrintBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub PrintBitmapCustomWidthStandardAlign: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PosPrinterAlignment, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    PrintBitmapCustomWidthStandardAlign: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub PrintCustomAlignedBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    PrintCustomAlignedBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub PrintBitmapCustomWidthCustomAlign: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    PrintBitmapCustomWidthCustomAlign: usize,
}
windows_core::imp::define_interface!(IReceiptPrintJob, IReceiptPrintJob_Vtbl, 0xaa96066e_acad_4b79_9d0f_c0cfc08dc77b);
impl windows_core::RuntimeType for IReceiptPrintJob {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IReceiptPrintJob_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MarkFeed: unsafe extern "system" fn(*mut core::ffi::c_void, PosPrinterMarkFeedKind) -> windows_core::HRESULT,
    pub CutPaper: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub CutPaperDefault: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IReceiptPrintJob2, IReceiptPrintJob2_Vtbl, 0x0cbc12e3_9e29_5179_bcd8_1811d3b9a10e);
impl windows_core::RuntimeType for IReceiptPrintJob2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IReceiptPrintJob2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StampPaper: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Print: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FeedPaperByLine: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub FeedPaperByMapModeUnit: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IReceiptPrinterCapabilities, IReceiptPrinterCapabilities_Vtbl, 0xb8f0b58f_51a8_43fc_9bd5_8de272a6415b);
impl windows_core::RuntimeType for IReceiptPrinterCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IReceiptPrinterCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CanCutPaper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsStampSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MarkFeedCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PosPrinterMarkFeedCapabilities) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IReceiptPrinterCapabilities2, IReceiptPrinterCapabilities2_Vtbl, 0x20030638_8a2c_55ac_9a7b_7576d8869e99);
impl windows_core::RuntimeType for IReceiptPrinterCapabilities2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IReceiptPrinterCapabilities2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsReverseVideoSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsStrikethroughSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSuperscriptSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSubscriptSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsReversePaperFeedByLineSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsReversePaperFeedByMapModeUnitSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISlipPrintJob, ISlipPrintJob_Vtbl, 0x5d88f95d_6131_5a4b_b7d5_8ef2da7b4165);
impl windows_core::RuntimeType for ISlipPrintJob {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISlipPrintJob_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Print: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FeedPaperByLine: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub FeedPaperByMapModeUnit: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISlipPrinterCapabilities, ISlipPrinterCapabilities_Vtbl, 0x99b16399_488c_4157_8ac2_9f57f708d3db);
impl windows_core::RuntimeType for ISlipPrinterCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISlipPrinterCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsFullLengthSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsBothSidesPrintingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISlipPrinterCapabilities2, ISlipPrinterCapabilities2_Vtbl, 0x6ff89671_2d1a_5000_87c2_b0851bfdf07e);
impl windows_core::RuntimeType for ISlipPrinterCapabilities2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISlipPrinterCapabilities2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsReverseVideoSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsStrikethroughSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSuperscriptSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSubscriptSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsReversePaperFeedByLineSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsReversePaperFeedByMapModeUnitSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUnifiedPosErrorData, IUnifiedPosErrorData_Vtbl, 0x2b998c3a_555c_4889_8ed8_c599bb3a712a);
impl windows_core::RuntimeType for IUnifiedPosErrorData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUnifiedPosErrorData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Severity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UnifiedPosErrorSeverity) -> windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UnifiedPosErrorReason) -> windows_core::HRESULT,
    pub ExtendedReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUnifiedPosErrorDataFactory, IUnifiedPosErrorDataFactory_Vtbl, 0x4b982551_1ffe_451b_a368_63e0ce465f5a);
impl windows_core::RuntimeType for IUnifiedPosErrorDataFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUnifiedPosErrorDataFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, UnifiedPosErrorSeverity, UnifiedPosErrorReason, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BarcodeScanner(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScanner, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BarcodeScanner, super::super::Foundation::IClosable);
impl BarcodeScanner {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Capabilities(&self) -> windows_core::Result<BarcodeScannerCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Capabilities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClaimScannerAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ClaimedBarcodeScanner>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClaimScannerAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CheckHealthAsync)(windows_core::Interface::as_raw(this), level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedSymbologiesAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSupportedSymbologiesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsSymbologySupportedAsync(&self, barcodesymbology: u32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSymbologySupportedAsync)(windows_core::Interface::as_raw(this), barcodesymbology, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn RetrieveStatisticsAsync<P0>(&self, statisticscategories: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetrieveStatisticsAsync)(windows_core::Interface::as_raw(this), statisticscategories.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedProfiles(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSupportedProfiles)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsProfileSupported(&self, profile: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsProfileSupported)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(profile), &mut result__).map(|| result__)
        }
    }
    pub fn StatusUpdated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<BarcodeScanner, BarcodeScannerStatusUpdatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusUpdated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStatusUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStatusUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn VideoDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IBarcodeScanner2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefaultAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<BarcodeScanner>> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<BarcodeScanner>> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> windows_core::Result<windows_core::HSTRING> {
        Self::IBarcodeScannerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(windows_core::Interface::as_raw(this), connectiontypes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IBarcodeScannerStatics<R, F: FnOnce(&IBarcodeScannerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BarcodeScanner, IBarcodeScannerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBarcodeScannerStatics2<R, F: FnOnce(&IBarcodeScannerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BarcodeScanner, IBarcodeScannerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BarcodeScanner {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScanner>();
}
unsafe impl windows_core::Interface for BarcodeScanner {
    type Vtable = IBarcodeScanner_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScanner as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScanner {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScanner";
}
unsafe impl Send for BarcodeScanner {}
unsafe impl Sync for BarcodeScanner {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BarcodeScannerCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerCapabilities {
    pub fn PowerReportingType(&self) -> windows_core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerReportingType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStatisticsReportingSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStatisticsReportingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStatisticsUpdatingSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStatisticsUpdatingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsImagePreviewSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsImagePreviewSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSoftwareTriggerSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerCapabilities1>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSoftwareTriggerSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsVideoPreviewSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVideoPreviewSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerCapabilities>();
}
unsafe impl windows_core::Interface for BarcodeScannerCapabilities {
    type Vtable = IBarcodeScannerCapabilities_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerCapabilities";
}
unsafe impl Send for BarcodeScannerCapabilities {}
unsafe impl Sync for BarcodeScannerCapabilities {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BarcodeScannerDataReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerDataReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerDataReceivedEventArgs {
    pub fn Report(&self) -> windows_core::Result<BarcodeScannerReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Report)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerDataReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerDataReceivedEventArgs>();
}
unsafe impl windows_core::Interface for BarcodeScannerDataReceivedEventArgs {
    type Vtable = IBarcodeScannerDataReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerDataReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerDataReceivedEventArgs";
}
unsafe impl Send for BarcodeScannerDataReceivedEventArgs {}
unsafe impl Sync for BarcodeScannerDataReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BarcodeScannerErrorOccurredEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerErrorOccurredEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerErrorOccurredEventArgs {
    pub fn PartialInputData(&self) -> windows_core::Result<BarcodeScannerReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PartialInputData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsRetriable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRetriable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorData(&self) -> windows_core::Result<UnifiedPosErrorData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerErrorOccurredEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerErrorOccurredEventArgs>();
}
unsafe impl windows_core::Interface for BarcodeScannerErrorOccurredEventArgs {
    type Vtable = IBarcodeScannerErrorOccurredEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerErrorOccurredEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerErrorOccurredEventArgs";
}
unsafe impl Send for BarcodeScannerErrorOccurredEventArgs {}
unsafe impl Sync for BarcodeScannerErrorOccurredEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BarcodeScannerImagePreviewReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerImagePreviewReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerImagePreviewReceivedEventArgs {
    #[cfg(feature = "Storage_Streams")]
    pub fn Preview(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Preview)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerImagePreviewReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerImagePreviewReceivedEventArgs>();
}
unsafe impl windows_core::Interface for BarcodeScannerImagePreviewReceivedEventArgs {
    type Vtable = IBarcodeScannerImagePreviewReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerImagePreviewReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerImagePreviewReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerImagePreviewReceivedEventArgs";
}
unsafe impl Send for BarcodeScannerImagePreviewReceivedEventArgs {}
unsafe impl Sync for BarcodeScannerImagePreviewReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BarcodeScannerReport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerReport, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerReport {
    pub fn ScanDataType(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScanDataType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ScanData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScanData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ScanDataLabel(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScanDataLabel)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateInstance<P0, P1>(scandatatype: u32, scandata: P0, scandatalabel: P1) -> windows_core::Result<BarcodeScannerReport>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::IBarcodeScannerReportFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), scandatatype, scandata.param().abi(), scandatalabel.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IBarcodeScannerReportFactory<R, F: FnOnce(&IBarcodeScannerReportFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BarcodeScannerReport, IBarcodeScannerReportFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BarcodeScannerReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerReport>();
}
unsafe impl windows_core::Interface for BarcodeScannerReport {
    type Vtable = IBarcodeScannerReport_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerReport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerReport {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerReport";
}
unsafe impl Send for BarcodeScannerReport {}
unsafe impl Sync for BarcodeScannerReport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BarcodeScannerStatusUpdatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerStatusUpdatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerStatusUpdatedEventArgs {
    pub fn Status(&self) -> windows_core::Result<BarcodeScannerStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedStatus(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerStatusUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerStatusUpdatedEventArgs>();
}
unsafe impl windows_core::Interface for BarcodeScannerStatusUpdatedEventArgs {
    type Vtable = IBarcodeScannerStatusUpdatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerStatusUpdatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerStatusUpdatedEventArgs";
}
unsafe impl Send for BarcodeScannerStatusUpdatedEventArgs {}
unsafe impl Sync for BarcodeScannerStatusUpdatedEventArgs {}
pub struct BarcodeSymbologies;
impl BarcodeSymbologies {
    pub fn Unknown() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unknown)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Ean8() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ean8)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Ean8Add2() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ean8Add2)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Ean8Add5() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ean8Add5)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Eanv() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Eanv)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn EanvAdd2() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EanvAdd2)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn EanvAdd5() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EanvAdd5)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Ean13() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ean13)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Ean13Add2() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ean13Add2)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Ean13Add5() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ean13Add5)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Isbn() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Isbn)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IsbnAdd5() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsbnAdd5)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Ismn() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ismn)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IsmnAdd2() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsmnAdd2)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IsmnAdd5() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsmnAdd5)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Issn() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Issn)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IssnAdd2() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IssnAdd2)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IssnAdd5() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IssnAdd5)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Ean99() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ean99)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Ean99Add2() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ean99Add2)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Ean99Add5() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ean99Add5)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Upca() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Upca)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UpcaAdd2() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpcaAdd2)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UpcaAdd5() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpcaAdd5)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Upce() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Upce)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UpceAdd2() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpceAdd2)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UpceAdd5() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpceAdd5)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UpcCoupon() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpcCoupon)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TfStd() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TfStd)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TfDis() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TfDis)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TfInt() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TfInt)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TfInd() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TfInd)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TfMat() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TfMat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TfIata() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TfIata)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Gs1DatabarType1() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gs1DatabarType1)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Gs1DatabarType2() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gs1DatabarType2)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Gs1DatabarType3() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gs1DatabarType3)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Code39() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Code39)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Code39Ex() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Code39Ex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Trioptic39() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Trioptic39)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Code32() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Code32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Pzn() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pzn)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Code93() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Code93)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Code93Ex() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Code93Ex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Code128() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Code128)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Gs1128() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gs1128)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Gs1128Coupon() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gs1128Coupon)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UccEan128() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UccEan128)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Sisac() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Sisac)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Isbt() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Isbt)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Codabar() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Codabar)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Code11() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Code11)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Msi() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Msi)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Plessey() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Plessey)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Telepen() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Telepen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Code16k() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Code16k)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CodablockA() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CodablockA)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CodablockF() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CodablockF)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Codablock128() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Codablock128)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Code49() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Code49)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Aztec() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Aztec)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DataCode() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DataMatrix() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataMatrix)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn HanXin() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HanXin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Maxicode() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Maxicode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MicroPdf417() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MicroPdf417)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MicroQr() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MicroQr)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Pdf417() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pdf417)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Qr() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Qr)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MsTag() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MsTag)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Ccab() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ccab)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Ccc() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ccc)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Tlc39() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tlc39)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn AusPost() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AusPost)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CanPost() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanPost)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ChinaPost() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChinaPost)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DutchKix() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DutchKix)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn InfoMail() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InfoMail)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ItalianPost25() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItalianPost25)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ItalianPost39() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItalianPost39)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn JapanPost() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JapanPost)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn KoreanPost() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KoreanPost)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SwedenPost() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SwedenPost)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UkPost() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UkPost)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UsIntelligent() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsIntelligent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UsIntelligentPkg() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsIntelligentPkg)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UsPlanet() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsPlanet)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UsPostNet() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsPostNet)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Us4StateFics() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Us4StateFics)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn OcrA() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OcrA)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn OcrB() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OcrB)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Micr() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Micr)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ExtendedBase() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedBase)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GetName(scandatatype: u32) -> windows_core::Result<windows_core::HSTRING> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetName)(windows_core::Interface::as_raw(this), scandatatype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Gs1DWCode() -> windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gs1DWCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IBarcodeSymbologiesStatics<R, F: FnOnce(&IBarcodeSymbologiesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BarcodeSymbologies, IBarcodeSymbologiesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBarcodeSymbologiesStatics2<R, F: FnOnce(&IBarcodeSymbologiesStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BarcodeSymbologies, IBarcodeSymbologiesStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for BarcodeSymbologies {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeSymbologies";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BarcodeSymbologyAttributes(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeSymbologyAttributes, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeSymbologyAttributes {
    pub fn IsCheckDigitValidationEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCheckDigitValidationEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCheckDigitValidationEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsCheckDigitValidationEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCheckDigitValidationSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCheckDigitValidationSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCheckDigitTransmissionEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCheckDigitTransmissionEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCheckDigitTransmissionEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsCheckDigitTransmissionEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCheckDigitTransmissionSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCheckDigitTransmissionSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DecodeLength1(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DecodeLength1)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDecodeLength1(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDecodeLength1)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DecodeLength2(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DecodeLength2)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDecodeLength2(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDecodeLength2)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DecodeLengthKind(&self) -> windows_core::Result<BarcodeSymbologyDecodeLengthKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DecodeLengthKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDecodeLengthKind(&self, value: BarcodeSymbologyDecodeLengthKind) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDecodeLengthKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecodeLengthSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDecodeLengthSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for BarcodeSymbologyAttributes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeSymbologyAttributes>();
}
unsafe impl windows_core::Interface for BarcodeSymbologyAttributes {
    type Vtable = IBarcodeSymbologyAttributes_Vtbl;
    const IID: windows_core::GUID = <IBarcodeSymbologyAttributes as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeSymbologyAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeSymbologyAttributes";
}
unsafe impl Send for BarcodeSymbologyAttributes {}
unsafe impl Sync for BarcodeSymbologyAttributes {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CashDrawer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CashDrawer, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CashDrawer, super::super::Foundation::IClosable);
impl CashDrawer {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Capabilities(&self) -> windows_core::Result<CashDrawerCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Capabilities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<CashDrawerStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsDrawerOpen(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDrawerOpen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DrawerEventSource(&self) -> windows_core::Result<CashDrawerEventSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DrawerEventSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClaimDrawerAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ClaimedCashDrawer>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClaimDrawerAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CheckHealthAsync)(windows_core::Interface::as_raw(this), level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStatisticsAsync<P0>(&self, statisticscategories: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatisticsAsync)(windows_core::Interface::as_raw(this), statisticscategories.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StatusUpdated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CashDrawer, CashDrawerStatusUpdatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusUpdated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStatusUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStatusUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefaultAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<CashDrawer>> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<CashDrawer>> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> windows_core::Result<windows_core::HSTRING> {
        Self::ICashDrawerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(windows_core::Interface::as_raw(this), connectiontypes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc(hidden)]
    pub fn ICashDrawerStatics<R, F: FnOnce(&ICashDrawerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CashDrawer, ICashDrawerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICashDrawerStatics2<R, F: FnOnce(&ICashDrawerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CashDrawer, ICashDrawerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CashDrawer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICashDrawer>();
}
unsafe impl windows_core::Interface for CashDrawer {
    type Vtable = ICashDrawer_Vtbl;
    const IID: windows_core::GUID = <ICashDrawer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CashDrawer {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawer";
}
unsafe impl Send for CashDrawer {}
unsafe impl Sync for CashDrawer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CashDrawerCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CashDrawerCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl CashDrawerCapabilities {
    pub fn PowerReportingType(&self) -> windows_core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerReportingType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStatisticsReportingSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStatisticsReportingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStatisticsUpdatingSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStatisticsUpdatingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStatusReportingSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStatusReportingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStatusMultiDrawerDetectSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStatusMultiDrawerDetectSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDrawerOpenSensorAvailable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDrawerOpenSensorAvailable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CashDrawerCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICashDrawerCapabilities>();
}
unsafe impl windows_core::Interface for CashDrawerCapabilities {
    type Vtable = ICashDrawerCapabilities_Vtbl;
    const IID: windows_core::GUID = <ICashDrawerCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CashDrawerCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerCapabilities";
}
unsafe impl Send for CashDrawerCapabilities {}
unsafe impl Sync for CashDrawerCapabilities {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CashDrawerCloseAlarm(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CashDrawerCloseAlarm, windows_core::IUnknown, windows_core::IInspectable);
impl CashDrawerCloseAlarm {
    pub fn SetAlarmTimeout(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAlarmTimeout)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlarmTimeout(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlarmTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBeepFrequency(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBeepFrequency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BeepFrequency(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BeepFrequency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBeepDuration(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBeepDuration)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BeepDuration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BeepDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBeepDelay(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBeepDelay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BeepDelay(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BeepDelay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AlarmTimeoutExpired<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CashDrawerCloseAlarm, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlarmTimeoutExpired)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAlarmTimeoutExpired(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAlarmTimeoutExpired)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn StartAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CashDrawerCloseAlarm {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICashDrawerCloseAlarm>();
}
unsafe impl windows_core::Interface for CashDrawerCloseAlarm {
    type Vtable = ICashDrawerCloseAlarm_Vtbl;
    const IID: windows_core::GUID = <ICashDrawerCloseAlarm as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CashDrawerCloseAlarm {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerCloseAlarm";
}
unsafe impl Send for CashDrawerCloseAlarm {}
unsafe impl Sync for CashDrawerCloseAlarm {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CashDrawerClosedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CashDrawerClosedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CashDrawerClosedEventArgs, ICashDrawerEventSourceEventArgs);
impl CashDrawerClosedEventArgs {
    pub fn CashDrawer(&self) -> windows_core::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CashDrawer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CashDrawerClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICashDrawerEventSourceEventArgs>();
}
unsafe impl windows_core::Interface for CashDrawerClosedEventArgs {
    type Vtable = ICashDrawerEventSourceEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICashDrawerEventSourceEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CashDrawerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerClosedEventArgs";
}
unsafe impl Send for CashDrawerClosedEventArgs {}
unsafe impl Sync for CashDrawerClosedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CashDrawerEventSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CashDrawerEventSource, windows_core::IUnknown, windows_core::IInspectable);
impl CashDrawerEventSource {
    pub fn DrawerClosed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerClosedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DrawerClosed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDrawerClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDrawerClosed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DrawerOpened<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerOpenedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DrawerOpened)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDrawerOpened(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDrawerOpened)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for CashDrawerEventSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICashDrawerEventSource>();
}
unsafe impl windows_core::Interface for CashDrawerEventSource {
    type Vtable = ICashDrawerEventSource_Vtbl;
    const IID: windows_core::GUID = <ICashDrawerEventSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CashDrawerEventSource {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerEventSource";
}
unsafe impl Send for CashDrawerEventSource {}
unsafe impl Sync for CashDrawerEventSource {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CashDrawerOpenedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CashDrawerOpenedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CashDrawerOpenedEventArgs, ICashDrawerEventSourceEventArgs);
impl CashDrawerOpenedEventArgs {
    pub fn CashDrawer(&self) -> windows_core::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CashDrawer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CashDrawerOpenedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICashDrawerEventSourceEventArgs>();
}
unsafe impl windows_core::Interface for CashDrawerOpenedEventArgs {
    type Vtable = ICashDrawerEventSourceEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICashDrawerEventSourceEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CashDrawerOpenedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerOpenedEventArgs";
}
unsafe impl Send for CashDrawerOpenedEventArgs {}
unsafe impl Sync for CashDrawerOpenedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CashDrawerStatus(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CashDrawerStatus, windows_core::IUnknown, windows_core::IInspectable);
impl CashDrawerStatus {
    pub fn StatusKind(&self) -> windows_core::Result<CashDrawerStatusKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedStatus(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CashDrawerStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICashDrawerStatus>();
}
unsafe impl windows_core::Interface for CashDrawerStatus {
    type Vtable = ICashDrawerStatus_Vtbl;
    const IID: windows_core::GUID = <ICashDrawerStatus as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CashDrawerStatus {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerStatus";
}
unsafe impl Send for CashDrawerStatus {}
unsafe impl Sync for CashDrawerStatus {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CashDrawerStatusUpdatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CashDrawerStatusUpdatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CashDrawerStatusUpdatedEventArgs {
    pub fn Status(&self) -> windows_core::Result<CashDrawerStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CashDrawerStatusUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICashDrawerStatusUpdatedEventArgs>();
}
unsafe impl windows_core::Interface for CashDrawerStatusUpdatedEventArgs {
    type Vtable = ICashDrawerStatusUpdatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICashDrawerStatusUpdatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CashDrawerStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerStatusUpdatedEventArgs";
}
unsafe impl Send for CashDrawerStatusUpdatedEventArgs {}
unsafe impl Sync for CashDrawerStatusUpdatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ClaimedBarcodeScanner(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ClaimedBarcodeScanner, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ClaimedBarcodeScanner, super::super::Foundation::IClosable);
impl ClaimedBarcodeScanner {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDisabledOnDataReceived(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsDisabledOnDataReceived)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDisabledOnDataReceived(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDisabledOnDataReceived)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDecodeDataEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsDecodeDataEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecodeDataEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDecodeDataEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EnableAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnableAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisableAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisableAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RetainDevice(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RetainDevice)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetActiveSymbologiesAsync<P0>(&self, symbologies: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<u32>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetActiveSymbologiesAsync)(windows_core::Interface::as_raw(this), symbologies.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetStatisticsAsync<P0>(&self, statisticscategories: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResetStatisticsAsync)(windows_core::Interface::as_raw(this), statisticscategories.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateStatisticsAsync<P0>(&self, statistics: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateStatisticsAsync)(windows_core::Interface::as_raw(this), statistics.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetActiveProfileAsync(&self, profile: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetActiveProfileAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(profile), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DataReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerDataReceivedEventArgs>>,
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
    pub fn TriggerPressed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriggerPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTriggerPressed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTriggerPressed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TriggerReleased<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriggerReleased)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTriggerReleased(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTriggerReleased)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ReleaseDeviceRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReleaseDeviceRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReleaseDeviceRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReleaseDeviceRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ImagePreviewReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerImagePreviewReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImagePreviewReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveImagePreviewReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveImagePreviewReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ErrorOccurred<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerErrorOccurredEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorOccurred)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveErrorOccurred(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveErrorOccurred)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn StartSoftwareTriggerAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IClaimedBarcodeScanner1>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartSoftwareTriggerAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StopSoftwareTriggerAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IClaimedBarcodeScanner1>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopSoftwareTriggerAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSymbologyAttributesAsync(&self, barcodesymbology: u32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<BarcodeSymbologyAttributes>> {
        let this = &windows_core::Interface::cast::<IClaimedBarcodeScanner2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSymbologyAttributesAsync)(windows_core::Interface::as_raw(this), barcodesymbology, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSymbologyAttributesAsync<P0>(&self, barcodesymbology: u32, attributes: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<BarcodeSymbologyAttributes>,
    {
        let this = &windows_core::Interface::cast::<IClaimedBarcodeScanner2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetSymbologyAttributesAsync)(windows_core::Interface::as_raw(this), barcodesymbology, attributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShowVideoPreviewAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowVideoPreviewAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HideVideoPreview(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).HideVideoPreview)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetIsVideoPreviewShownOnEnable(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsVideoPreviewShownOnEnable)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsVideoPreviewShownOnEnable(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVideoPreviewShownOnEnable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Closed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, ClaimedBarcodeScannerClosedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IClaimedBarcodeScanner4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClaimedBarcodeScanner4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ClaimedBarcodeScanner {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IClaimedBarcodeScanner>();
}
unsafe impl windows_core::Interface for ClaimedBarcodeScanner {
    type Vtable = IClaimedBarcodeScanner_Vtbl;
    const IID: windows_core::GUID = <IClaimedBarcodeScanner as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ClaimedBarcodeScanner {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedBarcodeScanner";
}
unsafe impl Send for ClaimedBarcodeScanner {}
unsafe impl Sync for ClaimedBarcodeScanner {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ClaimedBarcodeScannerClosedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ClaimedBarcodeScannerClosedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ClaimedBarcodeScannerClosedEventArgs {}
impl windows_core::RuntimeType for ClaimedBarcodeScannerClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IClaimedBarcodeScannerClosedEventArgs>();
}
unsafe impl windows_core::Interface for ClaimedBarcodeScannerClosedEventArgs {
    type Vtable = IClaimedBarcodeScannerClosedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IClaimedBarcodeScannerClosedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ClaimedBarcodeScannerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedBarcodeScannerClosedEventArgs";
}
unsafe impl Send for ClaimedBarcodeScannerClosedEventArgs {}
unsafe impl Sync for ClaimedBarcodeScannerClosedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ClaimedCashDrawer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ClaimedCashDrawer, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ClaimedCashDrawer, super::super::Foundation::IClosable);
impl ClaimedCashDrawer {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDrawerOpen(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDrawerOpen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CloseAlarm(&self) -> windows_core::Result<CashDrawerCloseAlarm> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloseAlarm)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OpenDrawerAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenDrawerAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EnableAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnableAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisableAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisableAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RetainDeviceAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetainDeviceAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetStatisticsAsync<P0>(&self, statisticscategories: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResetStatisticsAsync)(windows_core::Interface::as_raw(this), statisticscategories.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateStatisticsAsync<P0>(&self, statistics: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateStatisticsAsync)(windows_core::Interface::as_raw(this), statistics.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReleaseDeviceRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReleaseDeviceRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReleaseDeviceRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReleaseDeviceRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Closed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ClaimedCashDrawerClosedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IClaimedCashDrawer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClaimedCashDrawer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ClaimedCashDrawer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IClaimedCashDrawer>();
}
unsafe impl windows_core::Interface for ClaimedCashDrawer {
    type Vtable = IClaimedCashDrawer_Vtbl;
    const IID: windows_core::GUID = <IClaimedCashDrawer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ClaimedCashDrawer {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedCashDrawer";
}
unsafe impl Send for ClaimedCashDrawer {}
unsafe impl Sync for ClaimedCashDrawer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ClaimedCashDrawerClosedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ClaimedCashDrawerClosedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ClaimedCashDrawerClosedEventArgs {}
impl windows_core::RuntimeType for ClaimedCashDrawerClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IClaimedCashDrawerClosedEventArgs>();
}
unsafe impl windows_core::Interface for ClaimedCashDrawerClosedEventArgs {
    type Vtable = IClaimedCashDrawerClosedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IClaimedCashDrawerClosedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ClaimedCashDrawerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedCashDrawerClosedEventArgs";
}
unsafe impl Send for ClaimedCashDrawerClosedEventArgs {}
unsafe impl Sync for ClaimedCashDrawerClosedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ClaimedJournalPrinter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ClaimedJournalPrinter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ClaimedJournalPrinter, ICommonClaimedPosPrinterStation);
impl ClaimedJournalPrinter {
    pub fn CreateJob(&self) -> windows_core::Result<JournalPrintJob> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateJob)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCharactersPerLine(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCharactersPerLine)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharactersPerLine(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharactersPerLine)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLineHeight(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLineHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineHeight(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLineSpacing(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLineSpacing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineSpacing(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineSpacing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LineWidth(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsLetterQuality(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsLetterQuality)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsLetterQuality(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLetterQuality)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperNearEnd(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperNearEnd)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetColorCartridge)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ColorCartridge(&self) -> windows_core::Result<PosPrinterColorCartridge> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorCartridge)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCoverOpen(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCoverOpen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCartridgeRemoved(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCartridgeRemoved)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCartridgeEmpty(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCartridgeEmpty)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsHeadCleaning(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHeadCleaning)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperEmpty(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperEmpty)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsReadyToPrint(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReadyToPrint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ValidateData(&self, data: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValidateData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ClaimedJournalPrinter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IClaimedJournalPrinter>();
}
unsafe impl windows_core::Interface for ClaimedJournalPrinter {
    type Vtable = IClaimedJournalPrinter_Vtbl;
    const IID: windows_core::GUID = <IClaimedJournalPrinter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ClaimedJournalPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedJournalPrinter";
}
unsafe impl Send for ClaimedJournalPrinter {}
unsafe impl Sync for ClaimedJournalPrinter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ClaimedLineDisplay(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ClaimedLineDisplay, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ClaimedLineDisplay, super::super::Foundation::IClosable);
impl ClaimedLineDisplay {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Capabilities(&self) -> windows_core::Result<LineDisplayCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Capabilities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PhysicalDeviceName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhysicalDeviceName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PhysicalDeviceDescription(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhysicalDeviceDescription)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceControlDescription(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceControlDescription)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceControlVersion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceControlVersion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceServiceVersion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceServiceVersion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DefaultWindow(&self) -> windows_core::Result<LineDisplayWindow> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultWindow)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RetainDevice(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RetainDevice)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReleaseDeviceRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReleaseDeviceRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReleaseDeviceRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReleaseDeviceRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStatisticsAsync<P0>(&self, statisticscategories: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatisticsAsync)(windows_core::Interface::as_raw(this), statisticscategories.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CheckHealthAsync)(windows_core::Interface::as_raw(this), level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CheckPowerStatusAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>> {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CheckPowerStatusAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StatusUpdated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, LineDisplayStatusUpdatedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusUpdated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStatusUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveStatusUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedScreenSizesInCharacters(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Size>> {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedScreenSizesInCharacters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxBitmapSizeInPixels(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBitmapSizeInPixels)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharacterSets(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i32>> {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedCharacterSets)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CustomGlyphs(&self) -> windows_core::Result<LineDisplayCustomGlyphs> {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomGlyphs)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAttributes(&self) -> windows_core::Result<LineDisplayAttributes> {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAttributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryUpdateAttributesAsync<P0>(&self, attributes: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<LineDisplayAttributes>,
    {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUpdateAttributesAsync)(windows_core::Interface::as_raw(this), attributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrySetDescriptorAsync(&self, descriptor: u32, descriptorstate: LineDisplayDescriptorState) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetDescriptorAsync)(windows_core::Interface::as_raw(this), descriptor, descriptorstate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryClearDescriptorsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryClearDescriptorsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryCreateWindowAsync(&self, viewport: super::super::Foundation::Rect, windowsize: super::super::Foundation::Size) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LineDisplayWindow>> {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateWindowAsync)(windows_core::Interface::as_raw(this), viewport, windowsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn TryStoreStorageFileBitmapAsync<P0>(&self, bitmap: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>
    where
        P0: windows_core::Param<super::super::Storage::StorageFile>,
    {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryStoreStorageFileBitmapAsync)(windows_core::Interface::as_raw(this), bitmap.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn TryStoreStorageFileBitmapWithAlignmentAsync<P0>(&self, bitmap: P0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>
    where
        P0: windows_core::Param<super::super::Storage::StorageFile>,
    {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryStoreStorageFileBitmapWithAlignmentAsync)(windows_core::Interface::as_raw(this), bitmap.param().abi(), horizontalalignment, verticalalignment, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn TryStoreStorageFileBitmapWithAlignmentAndWidthAsync<P0>(&self, bitmap: P0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>
    where
        P0: windows_core::Param<super::super::Storage::StorageFile>,
    {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryStoreStorageFileBitmapWithAlignmentAndWidthAsync)(windows_core::Interface::as_raw(this), bitmap.param().abi(), horizontalalignment, verticalalignment, widthinpixels, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Closed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ClaimedLineDisplayClosedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClaimedLineDisplay3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> windows_core::Result<windows_core::HSTRING> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(windows_core::Interface::as_raw(this), connectiontypes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IClaimedLineDisplayStatics<R, F: FnOnce(&IClaimedLineDisplayStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ClaimedLineDisplay, IClaimedLineDisplayStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ClaimedLineDisplay {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IClaimedLineDisplay>();
}
unsafe impl windows_core::Interface for ClaimedLineDisplay {
    type Vtable = IClaimedLineDisplay_Vtbl;
    const IID: windows_core::GUID = <IClaimedLineDisplay as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ClaimedLineDisplay {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedLineDisplay";
}
unsafe impl Send for ClaimedLineDisplay {}
unsafe impl Sync for ClaimedLineDisplay {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ClaimedLineDisplayClosedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ClaimedLineDisplayClosedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ClaimedLineDisplayClosedEventArgs {}
impl windows_core::RuntimeType for ClaimedLineDisplayClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IClaimedLineDisplayClosedEventArgs>();
}
unsafe impl windows_core::Interface for ClaimedLineDisplayClosedEventArgs {
    type Vtable = IClaimedLineDisplayClosedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IClaimedLineDisplayClosedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ClaimedLineDisplayClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedLineDisplayClosedEventArgs";
}
unsafe impl Send for ClaimedLineDisplayClosedEventArgs {}
unsafe impl Sync for ClaimedLineDisplayClosedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ClaimedMagneticStripeReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ClaimedMagneticStripeReader, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ClaimedMagneticStripeReader, super::super::Foundation::IClosable);
impl ClaimedMagneticStripeReader {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDisabledOnDataReceived(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsDisabledOnDataReceived)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDisabledOnDataReceived(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDisabledOnDataReceived)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDecodeDataEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsDecodeDataEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecodeDataEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDecodeDataEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDeviceAuthenticated(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDeviceAuthenticated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDataEncryptionAlgorithm(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDataEncryptionAlgorithm)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DataEncryptionAlgorithm(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataEncryptionAlgorithm)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTracksToRead(&self, value: MagneticStripeReaderTrackIds) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTracksToRead)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TracksToRead(&self) -> windows_core::Result<MagneticStripeReaderTrackIds> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TracksToRead)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTransmitSentinelsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsTransmitSentinelsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsTransmitSentinelsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTransmitSentinelsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EnableAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnableAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisableAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisableAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RetainDevice(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RetainDevice)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetErrorReportingType(&self, value: MagneticStripeReaderErrorReportingType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetErrorReportingType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RetrieveDeviceAuthenticationDataAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetrieveDeviceAuthenticationDataAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AuthenticateDeviceAsync(&self, responsetoken: &[u8]) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AuthenticateDeviceAsync)(windows_core::Interface::as_raw(this), responsetoken.len().try_into().unwrap(), responsetoken.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeAuthenticateDeviceAsync(&self, responsetoken: &[u8]) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeAuthenticateDeviceAsync)(windows_core::Interface::as_raw(this), responsetoken.len().try_into().unwrap(), responsetoken.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdateKeyAsync(&self, key: &windows_core::HSTRING, keyname: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateKeyAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), core::mem::transmute_copy(keyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetStatisticsAsync<P0>(&self, statisticscategories: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResetStatisticsAsync)(windows_core::Interface::as_raw(this), statisticscategories.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateStatisticsAsync<P0>(&self, statistics: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateStatisticsAsync)(windows_core::Interface::as_raw(this), statistics.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BankCardDataReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderBankCardDataReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BankCardDataReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveBankCardDataReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveBankCardDataReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AamvaCardDataReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderAamvaCardDataReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AamvaCardDataReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAamvaCardDataReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAamvaCardDataReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn VendorSpecificDataReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VendorSpecificDataReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVendorSpecificDataReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveVendorSpecificDataReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ReleaseDeviceRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<ClaimedMagneticStripeReader>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReleaseDeviceRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReleaseDeviceRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReleaseDeviceRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ErrorOccurred<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderErrorOccurredEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorOccurred)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveErrorOccurred(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveErrorOccurred)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Closed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, ClaimedMagneticStripeReaderClosedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IClaimedMagneticStripeReader2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClaimedMagneticStripeReader2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ClaimedMagneticStripeReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IClaimedMagneticStripeReader>();
}
unsafe impl windows_core::Interface for ClaimedMagneticStripeReader {
    type Vtable = IClaimedMagneticStripeReader_Vtbl;
    const IID: windows_core::GUID = <IClaimedMagneticStripeReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ClaimedMagneticStripeReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedMagneticStripeReader";
}
unsafe impl Send for ClaimedMagneticStripeReader {}
unsafe impl Sync for ClaimedMagneticStripeReader {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ClaimedMagneticStripeReaderClosedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ClaimedMagneticStripeReaderClosedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ClaimedMagneticStripeReaderClosedEventArgs {}
impl windows_core::RuntimeType for ClaimedMagneticStripeReaderClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IClaimedMagneticStripeReaderClosedEventArgs>();
}
unsafe impl windows_core::Interface for ClaimedMagneticStripeReaderClosedEventArgs {
    type Vtable = IClaimedMagneticStripeReaderClosedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IClaimedMagneticStripeReaderClosedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ClaimedMagneticStripeReaderClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedMagneticStripeReaderClosedEventArgs";
}
unsafe impl Send for ClaimedMagneticStripeReaderClosedEventArgs {}
unsafe impl Sync for ClaimedMagneticStripeReaderClosedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ClaimedPosPrinter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ClaimedPosPrinter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ClaimedPosPrinter, super::super::Foundation::IClosable);
impl ClaimedPosPrinter {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCharacterSet(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCharacterSet)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharacterSet(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterSet)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCoverOpen(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCoverOpen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCharacterSetMappingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsCharacterSetMappingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCharacterSetMappingEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCharacterSetMappingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMapMode(&self, value: PosPrinterMapMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMapMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MapMode(&self) -> windows_core::Result<PosPrinterMapMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MapMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Receipt(&self) -> windows_core::Result<ClaimedReceiptPrinter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Receipt)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Slip(&self) -> windows_core::Result<ClaimedSlipPrinter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Slip)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Journal(&self) -> windows_core::Result<ClaimedJournalPrinter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Journal)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EnableAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnableAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisableAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisableAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RetainDeviceAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetainDeviceAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetStatisticsAsync<P0>(&self, statisticscategories: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResetStatisticsAsync)(windows_core::Interface::as_raw(this), statisticscategories.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateStatisticsAsync<P0>(&self, statistics: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateStatisticsAsync)(windows_core::Interface::as_raw(this), statistics.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReleaseDeviceRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, PosPrinterReleaseDeviceRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReleaseDeviceRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReleaseDeviceRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReleaseDeviceRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Closed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, ClaimedPosPrinterClosedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IClaimedPosPrinter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClaimedPosPrinter2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ClaimedPosPrinter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IClaimedPosPrinter>();
}
unsafe impl windows_core::Interface for ClaimedPosPrinter {
    type Vtable = IClaimedPosPrinter_Vtbl;
    const IID: windows_core::GUID = <IClaimedPosPrinter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ClaimedPosPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedPosPrinter";
}
unsafe impl Send for ClaimedPosPrinter {}
unsafe impl Sync for ClaimedPosPrinter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ClaimedPosPrinterClosedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ClaimedPosPrinterClosedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ClaimedPosPrinterClosedEventArgs {}
impl windows_core::RuntimeType for ClaimedPosPrinterClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IClaimedPosPrinterClosedEventArgs>();
}
unsafe impl windows_core::Interface for ClaimedPosPrinterClosedEventArgs {
    type Vtable = IClaimedPosPrinterClosedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IClaimedPosPrinterClosedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ClaimedPosPrinterClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedPosPrinterClosedEventArgs";
}
unsafe impl Send for ClaimedPosPrinterClosedEventArgs {}
unsafe impl Sync for ClaimedPosPrinterClosedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ClaimedReceiptPrinter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ClaimedReceiptPrinter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ClaimedReceiptPrinter, ICommonClaimedPosPrinterStation);
impl ClaimedReceiptPrinter {
    pub fn SidewaysMaxLines(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SidewaysMaxLines)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SidewaysMaxChars(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SidewaysMaxChars)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LinesToPaperCut(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LinesToPaperCut)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PageSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PrintArea(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintArea)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateJob(&self) -> windows_core::Result<ReceiptPrintJob> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateJob)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCharactersPerLine(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCharactersPerLine)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharactersPerLine(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharactersPerLine)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLineHeight(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLineHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineHeight(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLineSpacing(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLineSpacing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineSpacing(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineSpacing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LineWidth(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsLetterQuality(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsLetterQuality)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsLetterQuality(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLetterQuality)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperNearEnd(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperNearEnd)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetColorCartridge)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ColorCartridge(&self) -> windows_core::Result<PosPrinterColorCartridge> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorCartridge)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCoverOpen(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCoverOpen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCartridgeRemoved(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCartridgeRemoved)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCartridgeEmpty(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCartridgeEmpty)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsHeadCleaning(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHeadCleaning)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperEmpty(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperEmpty)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsReadyToPrint(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReadyToPrint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ValidateData(&self, data: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValidateData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ClaimedReceiptPrinter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IClaimedReceiptPrinter>();
}
unsafe impl windows_core::Interface for ClaimedReceiptPrinter {
    type Vtable = IClaimedReceiptPrinter_Vtbl;
    const IID: windows_core::GUID = <IClaimedReceiptPrinter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ClaimedReceiptPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedReceiptPrinter";
}
unsafe impl Send for ClaimedReceiptPrinter {}
unsafe impl Sync for ClaimedReceiptPrinter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ClaimedSlipPrinter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ClaimedSlipPrinter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ClaimedSlipPrinter, ICommonClaimedPosPrinterStation);
impl ClaimedSlipPrinter {
    pub fn SidewaysMaxLines(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SidewaysMaxLines)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SidewaysMaxChars(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SidewaysMaxChars)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxLines(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxLines)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LinesNearEndToEnd(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LinesNearEndToEnd)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PrintSide(&self) -> windows_core::Result<PosPrinterPrintSide> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintSide)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PageSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PrintArea(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintArea)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OpenJaws(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OpenJaws)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CloseJaws(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CloseJaws)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InsertSlipAsync(&self, timeout: super::super::Foundation::TimeSpan) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertSlipAsync)(windows_core::Interface::as_raw(this), timeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveSlipAsync(&self, timeout: super::super::Foundation::TimeSpan) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveSlipAsync)(windows_core::Interface::as_raw(this), timeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChangePrintSide(&self, printside: PosPrinterPrintSide) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ChangePrintSide)(windows_core::Interface::as_raw(this), printside).ok() }
    }
    pub fn CreateJob(&self) -> windows_core::Result<SlipPrintJob> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateJob)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCharactersPerLine(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCharactersPerLine)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharactersPerLine(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharactersPerLine)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLineHeight(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLineHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineHeight(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLineSpacing(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLineSpacing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineSpacing(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineSpacing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LineWidth(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsLetterQuality(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsLetterQuality)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsLetterQuality(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLetterQuality)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperNearEnd(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperNearEnd)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetColorCartridge)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ColorCartridge(&self) -> windows_core::Result<PosPrinterColorCartridge> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorCartridge)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCoverOpen(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCoverOpen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCartridgeRemoved(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCartridgeRemoved)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCartridgeEmpty(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCartridgeEmpty)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsHeadCleaning(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHeadCleaning)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperEmpty(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperEmpty)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsReadyToPrint(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReadyToPrint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ValidateData(&self, data: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValidateData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ClaimedSlipPrinter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IClaimedSlipPrinter>();
}
unsafe impl windows_core::Interface for ClaimedSlipPrinter {
    type Vtable = IClaimedSlipPrinter_Vtbl;
    const IID: windows_core::GUID = <IClaimedSlipPrinter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ClaimedSlipPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedSlipPrinter";
}
unsafe impl Send for ClaimedSlipPrinter {}
unsafe impl Sync for ClaimedSlipPrinter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct JournalPrintJob(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(JournalPrintJob, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(JournalPrintJob, IPosPrinterJob);
impl JournalPrintJob {
    pub fn Print<P0>(&self, data: &windows_core::HSTRING, printoptions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PosPrinterPrintOptions>,
    {
        let this = &windows_core::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Print)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data), printoptions.param().abi()).ok() }
    }
    pub fn FeedPaperByLine(&self, linecount: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).FeedPaperByLine)(windows_core::Interface::as_raw(this), linecount).ok() }
    }
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).FeedPaperByMapModeUnit)(windows_core::Interface::as_raw(this), distance).ok() }
    }
    pub fn Print2(&self, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Print)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintLine(&self, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintLine)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintNewline(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintNewline)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ExecuteAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExecuteAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for JournalPrintJob {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPosPrinterJob>();
}
unsafe impl windows_core::Interface for JournalPrintJob {
    type Vtable = IPosPrinterJob_Vtbl;
    const IID: windows_core::GUID = <IPosPrinterJob as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for JournalPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.JournalPrintJob";
}
unsafe impl Send for JournalPrintJob {}
unsafe impl Sync for JournalPrintJob {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct JournalPrinterCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(JournalPrinterCapabilities, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(JournalPrinterCapabilities, ICommonPosPrintStationCapabilities);
impl JournalPrinterCapabilities {
    pub fn IsPrinterPresent(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPrinterPresent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDualColorSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDualColorSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ColorCartridgeCapabilities(&self) -> windows_core::Result<PosPrinterColorCapabilities> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorCartridgeCapabilities)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CartridgeSensors(&self) -> windows_core::Result<PosPrinterCartridgeSensors> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CartridgeSensors)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsBoldSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBoldSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsItalicSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsItalicSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUnderlineSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDoubleHighPrintSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleHighPrintSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDoubleWidePrintSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleWidePrintSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleHighDoubleWidePrintSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperEmptySensorSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperEmptySensorSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperNearEndSensorSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperNearEndSensorSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharactersPerLine(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedCharactersPerLine)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsReverseVideoSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReverseVideoSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStrikethroughSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStrikethroughSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSuperscriptSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSuperscriptSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSubscriptSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSubscriptSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsReversePaperFeedByLineSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReversePaperFeedByLineSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReversePaperFeedByMapModeUnitSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for JournalPrinterCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IJournalPrinterCapabilities>();
}
unsafe impl windows_core::Interface for JournalPrinterCapabilities {
    type Vtable = IJournalPrinterCapabilities_Vtbl;
    const IID: windows_core::GUID = <IJournalPrinterCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for JournalPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.JournalPrinterCapabilities";
}
unsafe impl Send for JournalPrinterCapabilities {}
unsafe impl Sync for JournalPrinterCapabilities {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LineDisplay(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LineDisplay, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LineDisplay, super::super::Foundation::IClosable);
impl LineDisplay {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Capabilities(&self) -> windows_core::Result<LineDisplayCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Capabilities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PhysicalDeviceName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhysicalDeviceName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PhysicalDeviceDescription(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhysicalDeviceDescription)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceControlDescription(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceControlDescription)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceControlVersion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceControlVersion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceServiceVersion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceServiceVersion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClaimAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClaimAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CheckPowerStatusAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>> {
        let this = &windows_core::Interface::cast::<ILineDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CheckPowerStatusAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LineDisplay>> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<LineDisplay>> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> windows_core::Result<windows_core::HSTRING> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(windows_core::Interface::as_raw(this), connectiontypes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn StatisticsCategorySelector() -> windows_core::Result<LineDisplayStatisticsCategorySelector> {
        Self::ILineDisplayStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatisticsCategorySelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILineDisplayStatics<R, F: FnOnce(&ILineDisplayStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LineDisplay, ILineDisplayStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILineDisplayStatics2<R, F: FnOnce(&ILineDisplayStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LineDisplay, ILineDisplayStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LineDisplay {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILineDisplay>();
}
unsafe impl windows_core::Interface for LineDisplay {
    type Vtable = ILineDisplay_Vtbl;
    const IID: windows_core::GUID = <ILineDisplay as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LineDisplay {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplay";
}
unsafe impl Send for LineDisplay {}
unsafe impl Sync for LineDisplay {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LineDisplayAttributes(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LineDisplayAttributes, windows_core::IUnknown, windows_core::IInspectable);
impl LineDisplayAttributes {
    pub fn IsPowerNotifyEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPowerNotifyEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsPowerNotifyEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsPowerNotifyEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Brightness(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Brightness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBrightness(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBrightness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BlinkRate(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BlinkRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBlinkRate(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBlinkRate)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScreenSizeInCharacters(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScreenSizeInCharacters)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScreenSizeInCharacters(&self, value: super::super::Foundation::Size) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetScreenSizeInCharacters)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharacterSet(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterSet)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCharacterSet(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCharacterSet)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCharacterSetMappingEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCharacterSetMappingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCharacterSetMappingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsCharacterSetMappingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CurrentWindow(&self) -> windows_core::Result<LineDisplayWindow> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentWindow)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCurrentWindow<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<LineDisplayWindow>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCurrentWindow)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for LineDisplayAttributes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILineDisplayAttributes>();
}
unsafe impl windows_core::Interface for LineDisplayAttributes {
    type Vtable = ILineDisplayAttributes_Vtbl;
    const IID: windows_core::GUID = <ILineDisplayAttributes as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LineDisplayAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayAttributes";
}
unsafe impl Send for LineDisplayAttributes {}
unsafe impl Sync for LineDisplayAttributes {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LineDisplayCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LineDisplayCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl LineDisplayCapabilities {
    pub fn IsStatisticsReportingSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStatisticsReportingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStatisticsUpdatingSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStatisticsUpdatingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PowerReportingType(&self) -> windows_core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerReportingType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanChangeScreenSize(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanChangeScreenSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanDisplayBitmaps(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDisplayBitmaps)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanReadCharacterAtCursor(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanReadCharacterAtCursor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanMapCharacterSets(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanMapCharacterSets)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanDisplayCustomGlyphs(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDisplayCustomGlyphs)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanReverse(&self) -> windows_core::Result<LineDisplayTextAttributeGranularity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanReverse)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanBlink(&self) -> windows_core::Result<LineDisplayTextAttributeGranularity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBlink)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanChangeBlinkRate(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanChangeBlinkRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsBrightnessSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBrightnessSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCursorSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCursorSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsHorizontalMarqueeSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHorizontalMarqueeSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsVerticalMarqueeSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVerticalMarqueeSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsInterCharacterWaitSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInterCharacterWaitSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SupportedDescriptors(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedDescriptors)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SupportedWindows(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedWindows)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for LineDisplayCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILineDisplayCapabilities>();
}
unsafe impl windows_core::Interface for LineDisplayCapabilities {
    type Vtable = ILineDisplayCapabilities_Vtbl;
    const IID: windows_core::GUID = <ILineDisplayCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LineDisplayCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCapabilities";
}
unsafe impl Send for LineDisplayCapabilities {}
unsafe impl Sync for LineDisplayCapabilities {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LineDisplayCursor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LineDisplayCursor, windows_core::IUnknown, windows_core::IInspectable);
impl LineDisplayCursor {
    pub fn CanCustomize(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanCustomize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsBlinkSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBlinkSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsBlockSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBlockSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsHalfBlockSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHalfBlockSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUnderlineSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsReverseSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReverseSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsOtherSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOtherSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetAttributes(&self) -> windows_core::Result<LineDisplayCursorAttributes> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAttributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryUpdateAttributesAsync<P0>(&self, attributes: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<LineDisplayCursorAttributes>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUpdateAttributesAsync)(windows_core::Interface::as_raw(this), attributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LineDisplayCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILineDisplayCursor>();
}
unsafe impl windows_core::Interface for LineDisplayCursor {
    type Vtable = ILineDisplayCursor_Vtbl;
    const IID: windows_core::GUID = <ILineDisplayCursor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LineDisplayCursor {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCursor";
}
unsafe impl Send for LineDisplayCursor {}
unsafe impl Sync for LineDisplayCursor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LineDisplayCursorAttributes(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LineDisplayCursorAttributes, windows_core::IUnknown, windows_core::IInspectable);
impl LineDisplayCursorAttributes {
    pub fn IsBlinkEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBlinkEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsBlinkEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsBlinkEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CursorType(&self) -> windows_core::Result<LineDisplayCursorType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CursorType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCursorType(&self, value: LineDisplayCursorType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCursorType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAutoAdvanceEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAutoAdvanceEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAutoAdvanceEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsAutoAdvanceEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPosition(&self, value: super::super::Foundation::Point) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for LineDisplayCursorAttributes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILineDisplayCursorAttributes>();
}
unsafe impl windows_core::Interface for LineDisplayCursorAttributes {
    type Vtable = ILineDisplayCursorAttributes_Vtbl;
    const IID: windows_core::GUID = <ILineDisplayCursorAttributes as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LineDisplayCursorAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCursorAttributes";
}
unsafe impl Send for LineDisplayCursorAttributes {}
unsafe impl Sync for LineDisplayCursorAttributes {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LineDisplayCustomGlyphs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LineDisplayCustomGlyphs, windows_core::IUnknown, windows_core::IInspectable);
impl LineDisplayCustomGlyphs {
    pub fn SizeInPixels(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SizeInPixels)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedGlyphCodes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedGlyphCodes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn TryRedefineAsync<P0>(&self, glyphcode: u32, glyphdata: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryRedefineAsync)(windows_core::Interface::as_raw(this), glyphcode, glyphdata.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LineDisplayCustomGlyphs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILineDisplayCustomGlyphs>();
}
unsafe impl windows_core::Interface for LineDisplayCustomGlyphs {
    type Vtable = ILineDisplayCustomGlyphs_Vtbl;
    const IID: windows_core::GUID = <ILineDisplayCustomGlyphs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LineDisplayCustomGlyphs {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCustomGlyphs";
}
unsafe impl Send for LineDisplayCustomGlyphs {}
unsafe impl Sync for LineDisplayCustomGlyphs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LineDisplayMarquee(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LineDisplayMarquee, windows_core::IUnknown, windows_core::IInspectable);
impl LineDisplayMarquee {
    pub fn Format(&self) -> windows_core::Result<LineDisplayMarqueeFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Format)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFormat(&self, value: LineDisplayMarqueeFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RepeatWaitInterval(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RepeatWaitInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRepeatWaitInterval(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRepeatWaitInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScrollWaitInterval(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScrollWaitInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScrollWaitInterval(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetScrollWaitInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TryStartScrollingAsync(&self, direction: LineDisplayScrollDirection) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryStartScrollingAsync)(windows_core::Interface::as_raw(this), direction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryStopScrollingAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryStopScrollingAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LineDisplayMarquee {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILineDisplayMarquee>();
}
unsafe impl windows_core::Interface for LineDisplayMarquee {
    type Vtable = ILineDisplayMarquee_Vtbl;
    const IID: windows_core::GUID = <ILineDisplayMarquee as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LineDisplayMarquee {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayMarquee";
}
unsafe impl Send for LineDisplayMarquee {}
unsafe impl Sync for LineDisplayMarquee {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LineDisplayStatisticsCategorySelector(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LineDisplayStatisticsCategorySelector, windows_core::IUnknown, windows_core::IInspectable);
impl LineDisplayStatisticsCategorySelector {
    pub fn AllStatistics(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllStatistics)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UnifiedPosStatistics(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnifiedPosStatistics)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ManufacturerStatistics(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManufacturerStatistics)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LineDisplayStatisticsCategorySelector {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILineDisplayStatisticsCategorySelector>();
}
unsafe impl windows_core::Interface for LineDisplayStatisticsCategorySelector {
    type Vtable = ILineDisplayStatisticsCategorySelector_Vtbl;
    const IID: windows_core::GUID = <ILineDisplayStatisticsCategorySelector as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LineDisplayStatisticsCategorySelector {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStatisticsCategorySelector";
}
unsafe impl Send for LineDisplayStatisticsCategorySelector {}
unsafe impl Sync for LineDisplayStatisticsCategorySelector {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LineDisplayStatusUpdatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LineDisplayStatusUpdatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl LineDisplayStatusUpdatedEventArgs {
    pub fn Status(&self) -> windows_core::Result<LineDisplayPowerStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for LineDisplayStatusUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILineDisplayStatusUpdatedEventArgs>();
}
unsafe impl windows_core::Interface for LineDisplayStatusUpdatedEventArgs {
    type Vtable = ILineDisplayStatusUpdatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ILineDisplayStatusUpdatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LineDisplayStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStatusUpdatedEventArgs";
}
unsafe impl Send for LineDisplayStatusUpdatedEventArgs {}
unsafe impl Sync for LineDisplayStatusUpdatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LineDisplayStoredBitmap(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LineDisplayStoredBitmap, windows_core::IUnknown, windows_core::IInspectable);
impl LineDisplayStoredBitmap {
    pub fn EscapeSequence(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EscapeSequence)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryDeleteAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryDeleteAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LineDisplayStoredBitmap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILineDisplayStoredBitmap>();
}
unsafe impl windows_core::Interface for LineDisplayStoredBitmap {
    type Vtable = ILineDisplayStoredBitmap_Vtbl;
    const IID: windows_core::GUID = <ILineDisplayStoredBitmap as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LineDisplayStoredBitmap {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStoredBitmap";
}
unsafe impl Send for LineDisplayStoredBitmap {}
unsafe impl Sync for LineDisplayStoredBitmap {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LineDisplayWindow(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LineDisplayWindow, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LineDisplayWindow, super::super::Foundation::IClosable);
impl LineDisplayWindow {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SizeInCharacters(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SizeInCharacters)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InterCharacterWaitInterval(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InterCharacterWaitInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInterCharacterWaitInterval(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInterCharacterWaitInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TryRefreshAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryRefreshAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryDisplayTextAsync(&self, text: &windows_core::HSTRING, displayattribute: LineDisplayTextAttribute) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryDisplayTextAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), displayattribute, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryDisplayTextAtPositionAsync(&self, text: &windows_core::HSTRING, displayattribute: LineDisplayTextAttribute, startposition: super::super::Foundation::Point) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryDisplayTextAtPositionAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), displayattribute, startposition, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryDisplayTextNormalAsync(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryDisplayTextNormalAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryScrollTextAsync(&self, direction: LineDisplayScrollDirection, numberofcolumnsorrows: u32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryScrollTextAsync)(windows_core::Interface::as_raw(this), direction, numberofcolumnsorrows, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryClearTextAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryClearTextAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Cursor(&self) -> windows_core::Result<LineDisplayCursor> {
        let this = &windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Cursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Marquee(&self) -> windows_core::Result<LineDisplayMarquee> {
        let this = &windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Marquee)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReadCharacterAtCursorAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = &windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadCharacterAtCursorAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryDisplayStoredBitmapAtCursorAsync<P0>(&self, bitmap: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<LineDisplayStoredBitmap>,
    {
        let this = &windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryDisplayStoredBitmapAtCursorAsync)(windows_core::Interface::as_raw(this), bitmap.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn TryDisplayStorageFileBitmapAtCursorAsync<P0>(&self, bitmap: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Storage::StorageFile>,
    {
        let this = &windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryDisplayStorageFileBitmapAtCursorAsync)(windows_core::Interface::as_raw(this), bitmap.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync<P0>(&self, bitmap: P0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Storage::StorageFile>,
    {
        let this = &windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync)(windows_core::Interface::as_raw(this), bitmap.param().abi(), horizontalalignment, verticalalignment, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync<P0>(&self, bitmap: P0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Storage::StorageFile>,
    {
        let this = &windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync)(windows_core::Interface::as_raw(this), bitmap.param().abi(), horizontalalignment, verticalalignment, widthinpixels, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn TryDisplayStorageFileBitmapAtPointAsync<P0>(&self, bitmap: P0, offsetinpixels: super::super::Foundation::Point) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Storage::StorageFile>,
    {
        let this = &windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryDisplayStorageFileBitmapAtPointAsync)(windows_core::Interface::as_raw(this), bitmap.param().abi(), offsetinpixels, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn TryDisplayStorageFileBitmapAtPointWithWidthAsync<P0>(&self, bitmap: P0, offsetinpixels: super::super::Foundation::Point, widthinpixels: i32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Storage::StorageFile>,
    {
        let this = &windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryDisplayStorageFileBitmapAtPointWithWidthAsync)(windows_core::Interface::as_raw(this), bitmap.param().abi(), offsetinpixels, widthinpixels, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LineDisplayWindow {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILineDisplayWindow>();
}
unsafe impl windows_core::Interface for LineDisplayWindow {
    type Vtable = ILineDisplayWindow_Vtbl;
    const IID: windows_core::GUID = <ILineDisplayWindow as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LineDisplayWindow {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayWindow";
}
unsafe impl Send for LineDisplayWindow {}
unsafe impl Sync for LineDisplayWindow {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MagneticStripeReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MagneticStripeReader, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MagneticStripeReader, super::super::Foundation::IClosable);
impl MagneticStripeReader {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Capabilities(&self) -> windows_core::Result<MagneticStripeReaderCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Capabilities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SupportedCardTypes(&self) -> windows_core::Result<windows_core::Array<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).SupportedCardTypes)(windows_core::Interface::as_raw(this), windows_core::Array::<u32>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn DeviceAuthenticationProtocol(&self) -> windows_core::Result<MagneticStripeReaderAuthenticationProtocol> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceAuthenticationProtocol)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CheckHealthAsync)(windows_core::Interface::as_raw(this), level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClaimReaderAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ClaimedMagneticStripeReader>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClaimReaderAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn RetrieveStatisticsAsync<P0>(&self, statisticscategories: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetrieveStatisticsAsync)(windows_core::Interface::as_raw(this), statisticscategories.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetErrorReportingType(&self) -> windows_core::Result<MagneticStripeReaderErrorReportingType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetErrorReportingType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StatusUpdated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MagneticStripeReader, MagneticStripeReaderStatusUpdatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusUpdated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStatusUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStatusUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefaultAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<MagneticStripeReader>> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MagneticStripeReader>> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> windows_core::Result<windows_core::HSTRING> {
        Self::IMagneticStripeReaderStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(windows_core::Interface::as_raw(this), connectiontypes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMagneticStripeReaderStatics<R, F: FnOnce(&IMagneticStripeReaderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MagneticStripeReader, IMagneticStripeReaderStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMagneticStripeReaderStatics2<R, F: FnOnce(&IMagneticStripeReaderStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MagneticStripeReader, IMagneticStripeReaderStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MagneticStripeReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMagneticStripeReader>();
}
unsafe impl windows_core::Interface for MagneticStripeReader {
    type Vtable = IMagneticStripeReader_Vtbl;
    const IID: windows_core::GUID = <IMagneticStripeReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MagneticStripeReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReader";
}
unsafe impl Send for MagneticStripeReader {}
unsafe impl Sync for MagneticStripeReader {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MagneticStripeReaderAamvaCardDataReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MagneticStripeReaderAamvaCardDataReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    pub fn Report(&self) -> windows_core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Report)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LicenseNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LicenseNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExpirationDate(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationDate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Restrictions(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Restrictions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Class(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Class)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Endorsements(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Endorsements)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BirthDate(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BirthDate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Surname(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Surname)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Suffix(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Suffix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Gender(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gender)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HairColor(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HairColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EyeColor(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EyeColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Height(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Weight(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Weight)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Address(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Address)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn City(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).City)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn State(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PostalCode(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PostalCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMagneticStripeReaderAamvaCardDataReceivedEventArgs>();
}
unsafe impl windows_core::Interface for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderAamvaCardDataReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMagneticStripeReaderAamvaCardDataReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderAamvaCardDataReceivedEventArgs";
}
unsafe impl Send for MagneticStripeReaderAamvaCardDataReceivedEventArgs {}
unsafe impl Sync for MagneticStripeReaderAamvaCardDataReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MagneticStripeReaderBankCardDataReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MagneticStripeReaderBankCardDataReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MagneticStripeReaderBankCardDataReceivedEventArgs {
    pub fn Report(&self) -> windows_core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Report)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AccountNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExpirationDate(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationDate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ServiceCode(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MiddleInitial(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MiddleInitial)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Surname(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Surname)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Suffix(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Suffix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MagneticStripeReaderBankCardDataReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMagneticStripeReaderBankCardDataReceivedEventArgs>();
}
unsafe impl windows_core::Interface for MagneticStripeReaderBankCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderBankCardDataReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMagneticStripeReaderBankCardDataReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MagneticStripeReaderBankCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderBankCardDataReceivedEventArgs";
}
unsafe impl Send for MagneticStripeReaderBankCardDataReceivedEventArgs {}
unsafe impl Sync for MagneticStripeReaderBankCardDataReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MagneticStripeReaderCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MagneticStripeReaderCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl MagneticStripeReaderCapabilities {
    pub fn CardAuthentication(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CardAuthentication)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SupportedEncryptionAlgorithms(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedEncryptionAlgorithms)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AuthenticationLevel(&self) -> windows_core::Result<MagneticStripeReaderAuthenticationLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AuthenticationLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsIsoSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsIsoSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsJisOneSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsJisOneSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsJisTwoSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsJisTwoSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PowerReportingType(&self) -> windows_core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerReportingType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStatisticsReportingSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStatisticsReportingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStatisticsUpdatingSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStatisticsUpdatingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsTrackDataMaskingSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTrackDataMaskingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsTransmitSentinelsSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTransmitSentinelsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MagneticStripeReaderCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMagneticStripeReaderCapabilities>();
}
unsafe impl windows_core::Interface for MagneticStripeReaderCapabilities {
    type Vtable = IMagneticStripeReaderCapabilities_Vtbl;
    const IID: windows_core::GUID = <IMagneticStripeReaderCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MagneticStripeReaderCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderCapabilities";
}
unsafe impl Send for MagneticStripeReaderCapabilities {}
unsafe impl Sync for MagneticStripeReaderCapabilities {}
pub struct MagneticStripeReaderCardTypes;
impl MagneticStripeReaderCardTypes {
    pub fn Unknown() -> windows_core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unknown)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Bank() -> windows_core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bank)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Aamva() -> windows_core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Aamva)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ExtendedBase() -> windows_core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedBase)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IMagneticStripeReaderCardTypesStatics<R, F: FnOnce(&IMagneticStripeReaderCardTypesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MagneticStripeReaderCardTypes, IMagneticStripeReaderCardTypesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for MagneticStripeReaderCardTypes {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderCardTypes";
}
pub struct MagneticStripeReaderEncryptionAlgorithms;
impl MagneticStripeReaderEncryptionAlgorithms {
    pub fn None() -> windows_core::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).None)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TripleDesDukpt() -> windows_core::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TripleDesDukpt)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ExtendedBase() -> windows_core::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedBase)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IMagneticStripeReaderEncryptionAlgorithmsStatics<R, F: FnOnce(&IMagneticStripeReaderEncryptionAlgorithmsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MagneticStripeReaderEncryptionAlgorithms, IMagneticStripeReaderEncryptionAlgorithmsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for MagneticStripeReaderEncryptionAlgorithms {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderEncryptionAlgorithms";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MagneticStripeReaderErrorOccurredEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MagneticStripeReaderErrorOccurredEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MagneticStripeReaderErrorOccurredEventArgs {
    pub fn Track1Status(&self) -> windows_core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Track1Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Track2Status(&self) -> windows_core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Track2Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Track3Status(&self) -> windows_core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Track3Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Track4Status(&self) -> windows_core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Track4Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorData(&self) -> windows_core::Result<UnifiedPosErrorData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PartialInputData(&self) -> windows_core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PartialInputData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MagneticStripeReaderErrorOccurredEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMagneticStripeReaderErrorOccurredEventArgs>();
}
unsafe impl windows_core::Interface for MagneticStripeReaderErrorOccurredEventArgs {
    type Vtable = IMagneticStripeReaderErrorOccurredEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMagneticStripeReaderErrorOccurredEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MagneticStripeReaderErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderErrorOccurredEventArgs";
}
unsafe impl Send for MagneticStripeReaderErrorOccurredEventArgs {}
unsafe impl Sync for MagneticStripeReaderErrorOccurredEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MagneticStripeReaderReport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MagneticStripeReaderReport, windows_core::IUnknown, windows_core::IInspectable);
impl MagneticStripeReaderReport {
    pub fn CardType(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CardType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Track1(&self) -> windows_core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Track1)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Track2(&self) -> windows_core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Track2)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Track3(&self) -> windows_core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Track3)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Track4(&self) -> windows_core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Track4)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CardAuthenticationData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CardAuthenticationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CardAuthenticationDataLength(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CardAuthenticationDataLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AdditionalSecurityInformation(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdditionalSecurityInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MagneticStripeReaderReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMagneticStripeReaderReport>();
}
unsafe impl windows_core::Interface for MagneticStripeReaderReport {
    type Vtable = IMagneticStripeReaderReport_Vtbl;
    const IID: windows_core::GUID = <IMagneticStripeReaderReport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MagneticStripeReaderReport {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderReport";
}
unsafe impl Send for MagneticStripeReaderReport {}
unsafe impl Sync for MagneticStripeReaderReport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MagneticStripeReaderStatusUpdatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MagneticStripeReaderStatusUpdatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MagneticStripeReaderStatusUpdatedEventArgs {
    pub fn Status(&self) -> windows_core::Result<MagneticStripeReaderStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedStatus(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MagneticStripeReaderStatusUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMagneticStripeReaderStatusUpdatedEventArgs>();
}
unsafe impl windows_core::Interface for MagneticStripeReaderStatusUpdatedEventArgs {
    type Vtable = IMagneticStripeReaderStatusUpdatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMagneticStripeReaderStatusUpdatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MagneticStripeReaderStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderStatusUpdatedEventArgs";
}
unsafe impl Send for MagneticStripeReaderStatusUpdatedEventArgs {}
unsafe impl Sync for MagneticStripeReaderStatusUpdatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MagneticStripeReaderTrackData(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MagneticStripeReaderTrackData, windows_core::IUnknown, windows_core::IInspectable);
impl MagneticStripeReaderTrackData {
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn DiscretionaryData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DiscretionaryData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn EncryptedData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncryptedData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MagneticStripeReaderTrackData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMagneticStripeReaderTrackData>();
}
unsafe impl windows_core::Interface for MagneticStripeReaderTrackData {
    type Vtable = IMagneticStripeReaderTrackData_Vtbl;
    const IID: windows_core::GUID = <IMagneticStripeReaderTrackData as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MagneticStripeReaderTrackData {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderTrackData";
}
unsafe impl Send for MagneticStripeReaderTrackData {}
unsafe impl Sync for MagneticStripeReaderTrackData {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    pub fn Report(&self) -> windows_core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Report)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs>();
}
unsafe impl windows_core::Interface for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs";
}
unsafe impl Send for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {}
unsafe impl Sync for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PosPrinter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PosPrinter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PosPrinter, super::super::Foundation::IClosable);
impl PosPrinter {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Capabilities(&self) -> windows_core::Result<PosPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Capabilities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharacterSets(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedCharacterSets)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedTypeFaces(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedTypeFaces)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<PosPrinterStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClaimPrinterAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ClaimedPosPrinter>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClaimPrinterAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CheckHealthAsync)(windows_core::Interface::as_raw(this), level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStatisticsAsync<P0>(&self, statisticscategories: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatisticsAsync)(windows_core::Interface::as_raw(this), statisticscategories.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StatusUpdated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PosPrinter, PosPrinterStatusUpdatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusUpdated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStatusUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStatusUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBarcodeSymbologies(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &windows_core::Interface::cast::<IPosPrinter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedBarcodeSymbologies)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFontProperty(&self, typeface: &windows_core::HSTRING) -> windows_core::Result<PosPrinterFontProperty> {
        let this = &windows_core::Interface::cast::<IPosPrinter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFontProperty)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(typeface), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefaultAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<PosPrinter>> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PosPrinter>> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> windows_core::Result<windows_core::HSTRING> {
        Self::IPosPrinterStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(windows_core::Interface::as_raw(this), connectiontypes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPosPrinterStatics<R, F: FnOnce(&IPosPrinterStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PosPrinter, IPosPrinterStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPosPrinterStatics2<R, F: FnOnce(&IPosPrinterStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PosPrinter, IPosPrinterStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PosPrinter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPosPrinter>();
}
unsafe impl windows_core::Interface for PosPrinter {
    type Vtable = IPosPrinter_Vtbl;
    const IID: windows_core::GUID = <IPosPrinter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PosPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinter";
}
unsafe impl Send for PosPrinter {}
unsafe impl Sync for PosPrinter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PosPrinterCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PosPrinterCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl PosPrinterCapabilities {
    pub fn PowerReportingType(&self) -> windows_core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerReportingType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStatisticsReportingSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStatisticsReportingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStatisticsUpdatingSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStatisticsUpdatingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DefaultCharacterSet(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultCharacterSet)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasCoverSensor(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCoverSensor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanMapCharacterSet(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanMapCharacterSet)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsTransactionSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTransactionSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Receipt(&self) -> windows_core::Result<ReceiptPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Receipt)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Slip(&self) -> windows_core::Result<SlipPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Slip)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Journal(&self) -> windows_core::Result<JournalPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Journal)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PosPrinterCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPosPrinterCapabilities>();
}
unsafe impl windows_core::Interface for PosPrinterCapabilities {
    type Vtable = IPosPrinterCapabilities_Vtbl;
    const IID: windows_core::GUID = <IPosPrinterCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PosPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterCapabilities";
}
unsafe impl Send for PosPrinterCapabilities {}
unsafe impl Sync for PosPrinterCapabilities {}
pub struct PosPrinterCharacterSetIds;
impl PosPrinterCharacterSetIds {
    pub fn Utf16LE() -> windows_core::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Utf16LE)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Ascii() -> windows_core::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ascii)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Ansi() -> windows_core::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ansi)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IPosPrinterCharacterSetIdsStatics<R, F: FnOnce(&IPosPrinterCharacterSetIdsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PosPrinterCharacterSetIds, IPosPrinterCharacterSetIdsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PosPrinterCharacterSetIds {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterCharacterSetIds";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PosPrinterFontProperty(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PosPrinterFontProperty, windows_core::IUnknown, windows_core::IInspectable);
impl PosPrinterFontProperty {
    pub fn TypeFace(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TypeFace)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsScalableToAnySize(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsScalableToAnySize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CharacterSizes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<SizeUInt32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterSizes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PosPrinterFontProperty {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPosPrinterFontProperty>();
}
unsafe impl windows_core::Interface for PosPrinterFontProperty {
    type Vtable = IPosPrinterFontProperty_Vtbl;
    const IID: windows_core::GUID = <IPosPrinterFontProperty as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PosPrinterFontProperty {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterFontProperty";
}
unsafe impl Send for PosPrinterFontProperty {}
unsafe impl Sync for PosPrinterFontProperty {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PosPrinterPrintOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PosPrinterPrintOptions, windows_core::IUnknown, windows_core::IInspectable);
impl PosPrinterPrintOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PosPrinterPrintOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TypeFace(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TypeFace)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTypeFace(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTypeFace)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CharacterHeight(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCharacterHeight(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCharacterHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bold(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bold)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBold(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBold)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Italic(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Italic)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetItalic(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetItalic)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Underline(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Underline)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUnderline(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUnderline)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReverseVideo(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReverseVideo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReverseVideo(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReverseVideo)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Strikethrough(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Strikethrough)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStrikethrough(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStrikethrough)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Superscript(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Superscript)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSuperscript(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSuperscript)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Subscript(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subscript)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSubscript(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSubscript)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DoubleWide(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DoubleWide)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDoubleWide(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDoubleWide)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DoubleHigh(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DoubleHigh)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDoubleHigh(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDoubleHigh)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Alignment(&self) -> windows_core::Result<PosPrinterAlignment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Alignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAlignment(&self, value: PosPrinterAlignment) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharacterSet(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterSet)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCharacterSet(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCharacterSet)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for PosPrinterPrintOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPosPrinterPrintOptions>();
}
unsafe impl windows_core::Interface for PosPrinterPrintOptions {
    type Vtable = IPosPrinterPrintOptions_Vtbl;
    const IID: windows_core::GUID = <IPosPrinterPrintOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PosPrinterPrintOptions {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterPrintOptions";
}
unsafe impl Send for PosPrinterPrintOptions {}
unsafe impl Sync for PosPrinterPrintOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PosPrinterReleaseDeviceRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PosPrinterReleaseDeviceRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PosPrinterReleaseDeviceRequestedEventArgs {}
impl windows_core::RuntimeType for PosPrinterReleaseDeviceRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPosPrinterReleaseDeviceRequestedEventArgs>();
}
unsafe impl windows_core::Interface for PosPrinterReleaseDeviceRequestedEventArgs {
    type Vtable = IPosPrinterReleaseDeviceRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPosPrinterReleaseDeviceRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PosPrinterReleaseDeviceRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterReleaseDeviceRequestedEventArgs";
}
unsafe impl Send for PosPrinterReleaseDeviceRequestedEventArgs {}
unsafe impl Sync for PosPrinterReleaseDeviceRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PosPrinterStatus(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PosPrinterStatus, windows_core::IUnknown, windows_core::IInspectable);
impl PosPrinterStatus {
    pub fn StatusKind(&self) -> windows_core::Result<PosPrinterStatusKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedStatus(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PosPrinterStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPosPrinterStatus>();
}
unsafe impl windows_core::Interface for PosPrinterStatus {
    type Vtable = IPosPrinterStatus_Vtbl;
    const IID: windows_core::GUID = <IPosPrinterStatus as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PosPrinterStatus {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterStatus";
}
unsafe impl Send for PosPrinterStatus {}
unsafe impl Sync for PosPrinterStatus {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PosPrinterStatusUpdatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PosPrinterStatusUpdatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PosPrinterStatusUpdatedEventArgs {
    pub fn Status(&self) -> windows_core::Result<PosPrinterStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PosPrinterStatusUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPosPrinterStatusUpdatedEventArgs>();
}
unsafe impl windows_core::Interface for PosPrinterStatusUpdatedEventArgs {
    type Vtable = IPosPrinterStatusUpdatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPosPrinterStatusUpdatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PosPrinterStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterStatusUpdatedEventArgs";
}
unsafe impl Send for PosPrinterStatusUpdatedEventArgs {}
unsafe impl Sync for PosPrinterStatusUpdatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ReceiptPrintJob(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ReceiptPrintJob, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ReceiptPrintJob, IPosPrinterJob, IReceiptOrSlipJob);
impl ReceiptPrintJob {
    pub fn Print(&self, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Print)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintLine(&self, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PrintLine)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintNewline(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PrintNewline)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ExecuteAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExecuteAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBarcodeRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrintRotation)(windows_core::Interface::as_raw(this), value, includebitmaps).ok() }
    }
    pub fn SetPrintArea(&self, value: super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrintArea)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmap<P0>(&self, bitmapnumber: u32, bitmap: P0, alignment: PosPrinterAlignment) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = &windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBitmap)(windows_core::Interface::as_raw(this), bitmapnumber, bitmap.param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthStandardAlign<P0>(&self, bitmapnumber: u32, bitmap: P0, alignment: PosPrinterAlignment, width: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = &windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBitmapCustomWidthStandardAlign)(windows_core::Interface::as_raw(this), bitmapnumber, bitmap.param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetCustomAlignedBitmap<P0>(&self, bitmapnumber: u32, bitmap: P0, alignmentdistance: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = &windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCustomAlignedBitmap)(windows_core::Interface::as_raw(this), bitmapnumber, bitmap.param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthCustomAlign<P0>(&self, bitmapnumber: u32, bitmap: P0, alignmentdistance: u32, width: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = &windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBitmapCustomWidthCustomAlign)(windows_core::Interface::as_raw(this), bitmapnumber, bitmap.param().abi(), alignmentdistance, width).ok() }
    }
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PrintSavedBitmap)(windows_core::Interface::as_raw(this), bitmapnumber).ok() }
    }
    pub fn DrawRuledLine(&self, positionlist: &windows_core::HSTRING, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DrawRuledLine)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(positionlist), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    pub fn PrintBarcode(&self, data: &windows_core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PrintBarcode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data), symbology, height, width, textposition, alignment).ok() }
    }
    pub fn PrintBarcodeCustomAlign(&self, data: &windows_core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PrintBarcodeCustomAlign)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmap<P0>(&self, bitmap: P0, alignment: PosPrinterAlignment) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = &windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PrintBitmap)(windows_core::Interface::as_raw(this), bitmap.param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthStandardAlign<P0>(&self, bitmap: P0, alignment: PosPrinterAlignment, width: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = &windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PrintBitmapCustomWidthStandardAlign)(windows_core::Interface::as_raw(this), bitmap.param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintCustomAlignedBitmap<P0>(&self, bitmap: P0, alignmentdistance: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = &windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PrintCustomAlignedBitmap)(windows_core::Interface::as_raw(this), bitmap.param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthCustomAlign<P0>(&self, bitmap: P0, alignmentdistance: u32, width: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = &windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PrintBitmapCustomWidthCustomAlign)(windows_core::Interface::as_raw(this), bitmap.param().abi(), alignmentdistance, width).ok() }
    }
    pub fn MarkFeed(&self, kind: PosPrinterMarkFeedKind) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).MarkFeed)(windows_core::Interface::as_raw(this), kind).ok() }
    }
    pub fn CutPaper(&self, percentage: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CutPaper)(windows_core::Interface::as_raw(this), percentage).ok() }
    }
    pub fn CutPaperDefault(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CutPaperDefault)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StampPaper(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StampPaper)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Print2<P0>(&self, data: &windows_core::HSTRING, printoptions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PosPrinterPrintOptions>,
    {
        let this = &windows_core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Print)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data), printoptions.param().abi()).ok() }
    }
    pub fn FeedPaperByLine(&self, linecount: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).FeedPaperByLine)(windows_core::Interface::as_raw(this), linecount).ok() }
    }
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).FeedPaperByMapModeUnit)(windows_core::Interface::as_raw(this), distance).ok() }
    }
}
impl windows_core::RuntimeType for ReceiptPrintJob {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IReceiptPrintJob>();
}
unsafe impl windows_core::Interface for ReceiptPrintJob {
    type Vtable = IReceiptPrintJob_Vtbl;
    const IID: windows_core::GUID = <IReceiptPrintJob as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ReceiptPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.ReceiptPrintJob";
}
unsafe impl Send for ReceiptPrintJob {}
unsafe impl Sync for ReceiptPrintJob {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ReceiptPrinterCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ReceiptPrinterCapabilities, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ReceiptPrinterCapabilities, ICommonPosPrintStationCapabilities, ICommonReceiptSlipCapabilities);
impl ReceiptPrinterCapabilities {
    pub fn IsPrinterPresent(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPrinterPresent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDualColorSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDualColorSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ColorCartridgeCapabilities(&self) -> windows_core::Result<PosPrinterColorCapabilities> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorCartridgeCapabilities)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CartridgeSensors(&self) -> windows_core::Result<PosPrinterCartridgeSensors> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CartridgeSensors)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsBoldSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBoldSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsItalicSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsItalicSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUnderlineSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDoubleHighPrintSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleHighPrintSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDoubleWidePrintSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleWidePrintSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleHighDoubleWidePrintSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperEmptySensorSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperEmptySensorSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperNearEndSensorSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperNearEndSensorSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharactersPerLine(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedCharactersPerLine)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsBarcodeSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBarcodeSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsBitmapSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBitmapSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsLeft90RotationSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLeft90RotationSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRight90RotationSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRight90RotationSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Is180RotationSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Is180RotationSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPrintAreaSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPrintAreaSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RuledLineCapabilities(&self) -> windows_core::Result<PosPrinterRuledLineCapabilities> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RuledLineCapabilities)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBarcodeRotations(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedBarcodeRotations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBitmapRotations(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedBitmapRotations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanCutPaper(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanCutPaper)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStampSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStampSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MarkFeedCapabilities(&self) -> windows_core::Result<PosPrinterMarkFeedCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MarkFeedCapabilities)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsReverseVideoSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReverseVideoSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStrikethroughSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStrikethroughSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSuperscriptSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSuperscriptSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSubscriptSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSubscriptSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsReversePaperFeedByLineSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReversePaperFeedByLineSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReversePaperFeedByMapModeUnitSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ReceiptPrinterCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IReceiptPrinterCapabilities>();
}
unsafe impl windows_core::Interface for ReceiptPrinterCapabilities {
    type Vtable = IReceiptPrinterCapabilities_Vtbl;
    const IID: windows_core::GUID = <IReceiptPrinterCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ReceiptPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ReceiptPrinterCapabilities";
}
unsafe impl Send for ReceiptPrinterCapabilities {}
unsafe impl Sync for ReceiptPrinterCapabilities {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SlipPrintJob(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SlipPrintJob, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SlipPrintJob, IPosPrinterJob, IReceiptOrSlipJob);
impl SlipPrintJob {
    pub fn Print(&self, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Print)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintLine(&self, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PrintLine)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintNewline(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PrintNewline)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ExecuteAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExecuteAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBarcodeRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrintRotation)(windows_core::Interface::as_raw(this), value, includebitmaps).ok() }
    }
    pub fn SetPrintArea(&self, value: super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrintArea)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmap<P0>(&self, bitmapnumber: u32, bitmap: P0, alignment: PosPrinterAlignment) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBitmap)(windows_core::Interface::as_raw(this), bitmapnumber, bitmap.param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthStandardAlign<P0>(&self, bitmapnumber: u32, bitmap: P0, alignment: PosPrinterAlignment, width: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBitmapCustomWidthStandardAlign)(windows_core::Interface::as_raw(this), bitmapnumber, bitmap.param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetCustomAlignedBitmap<P0>(&self, bitmapnumber: u32, bitmap: P0, alignmentdistance: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCustomAlignedBitmap)(windows_core::Interface::as_raw(this), bitmapnumber, bitmap.param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthCustomAlign<P0>(&self, bitmapnumber: u32, bitmap: P0, alignmentdistance: u32, width: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBitmapCustomWidthCustomAlign)(windows_core::Interface::as_raw(this), bitmapnumber, bitmap.param().abi(), alignmentdistance, width).ok() }
    }
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintSavedBitmap)(windows_core::Interface::as_raw(this), bitmapnumber).ok() }
    }
    pub fn DrawRuledLine(&self, positionlist: &windows_core::HSTRING, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DrawRuledLine)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(positionlist), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    pub fn PrintBarcode(&self, data: &windows_core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintBarcode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data), symbology, height, width, textposition, alignment).ok() }
    }
    pub fn PrintBarcodeCustomAlign(&self, data: &windows_core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintBarcodeCustomAlign)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmap<P0>(&self, bitmap: P0, alignment: PosPrinterAlignment) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintBitmap)(windows_core::Interface::as_raw(this), bitmap.param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthStandardAlign<P0>(&self, bitmap: P0, alignment: PosPrinterAlignment, width: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintBitmapCustomWidthStandardAlign)(windows_core::Interface::as_raw(this), bitmap.param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintCustomAlignedBitmap<P0>(&self, bitmap: P0, alignmentdistance: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintCustomAlignedBitmap)(windows_core::Interface::as_raw(this), bitmap.param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthCustomAlign<P0>(&self, bitmap: P0, alignmentdistance: u32, width: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::BitmapFrame>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PrintBitmapCustomWidthCustomAlign)(windows_core::Interface::as_raw(this), bitmap.param().abi(), alignmentdistance, width).ok() }
    }
    pub fn Print2<P0>(&self, data: &windows_core::HSTRING, printoptions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PosPrinterPrintOptions>,
    {
        let this = &windows_core::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Print)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data), printoptions.param().abi()).ok() }
    }
    pub fn FeedPaperByLine(&self, linecount: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).FeedPaperByLine)(windows_core::Interface::as_raw(this), linecount).ok() }
    }
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (windows_core::Interface::vtable(this).FeedPaperByMapModeUnit)(windows_core::Interface::as_raw(this), distance).ok() }
    }
}
impl windows_core::RuntimeType for SlipPrintJob {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IReceiptOrSlipJob>();
}
unsafe impl windows_core::Interface for SlipPrintJob {
    type Vtable = IReceiptOrSlipJob_Vtbl;
    const IID: windows_core::GUID = <IReceiptOrSlipJob as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SlipPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.SlipPrintJob";
}
unsafe impl Send for SlipPrintJob {}
unsafe impl Sync for SlipPrintJob {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SlipPrinterCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SlipPrinterCapabilities, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SlipPrinterCapabilities, ICommonPosPrintStationCapabilities, ICommonReceiptSlipCapabilities);
impl SlipPrinterCapabilities {
    pub fn IsPrinterPresent(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPrinterPresent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDualColorSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDualColorSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ColorCartridgeCapabilities(&self) -> windows_core::Result<PosPrinterColorCapabilities> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorCartridgeCapabilities)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CartridgeSensors(&self) -> windows_core::Result<PosPrinterCartridgeSensors> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CartridgeSensors)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsBoldSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBoldSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsItalicSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsItalicSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUnderlineSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDoubleHighPrintSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleHighPrintSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDoubleWidePrintSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleWidePrintSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleHighDoubleWidePrintSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperEmptySensorSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperEmptySensorSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPaperNearEndSensorSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaperNearEndSensorSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharactersPerLine(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedCharactersPerLine)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsBarcodeSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBarcodeSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsBitmapSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBitmapSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsLeft90RotationSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLeft90RotationSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRight90RotationSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRight90RotationSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Is180RotationSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Is180RotationSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPrintAreaSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPrintAreaSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RuledLineCapabilities(&self) -> windows_core::Result<PosPrinterRuledLineCapabilities> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RuledLineCapabilities)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBarcodeRotations(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedBarcodeRotations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBitmapRotations(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedBitmapRotations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsFullLengthSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFullLengthSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsBothSidesPrintingSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBothSidesPrintingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsReverseVideoSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReverseVideoSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStrikethroughSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStrikethroughSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSuperscriptSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSuperscriptSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSubscriptSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSubscriptSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsReversePaperFeedByLineSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReversePaperFeedByLineSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReversePaperFeedByMapModeUnitSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SlipPrinterCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISlipPrinterCapabilities>();
}
unsafe impl windows_core::Interface for SlipPrinterCapabilities {
    type Vtable = ISlipPrinterCapabilities_Vtbl;
    const IID: windows_core::GUID = <ISlipPrinterCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SlipPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.SlipPrinterCapabilities";
}
unsafe impl Send for SlipPrinterCapabilities {}
unsafe impl Sync for SlipPrinterCapabilities {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct UnifiedPosErrorData(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UnifiedPosErrorData, windows_core::IUnknown, windows_core::IInspectable);
impl UnifiedPosErrorData {
    pub fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Severity(&self) -> windows_core::Result<UnifiedPosErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Severity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Reason(&self) -> windows_core::Result<UnifiedPosErrorReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedReason(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedReason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInstance(message: &windows_core::HSTRING, severity: UnifiedPosErrorSeverity, reason: UnifiedPosErrorReason, extendedreason: u32) -> windows_core::Result<UnifiedPosErrorData> {
        Self::IUnifiedPosErrorDataFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(message), severity, reason, extendedreason, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUnifiedPosErrorDataFactory<R, F: FnOnce(&IUnifiedPosErrorDataFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UnifiedPosErrorData, IUnifiedPosErrorDataFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UnifiedPosErrorData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUnifiedPosErrorData>();
}
unsafe impl windows_core::Interface for UnifiedPosErrorData {
    type Vtable = IUnifiedPosErrorData_Vtbl;
    const IID: windows_core::GUID = <IUnifiedPosErrorData as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UnifiedPosErrorData {
    const NAME: &'static str = "Windows.Devices.PointOfService.UnifiedPosErrorData";
}
unsafe impl Send for UnifiedPosErrorData {}
unsafe impl Sync for UnifiedPosErrorData {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BarcodeScannerStatus(pub i32);
impl BarcodeScannerStatus {
    pub const Online: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const Offline: Self = Self(2i32);
    pub const OffOrOffline: Self = Self(3i32);
    pub const Extended: Self = Self(4i32);
}
impl windows_core::TypeKind for BarcodeScannerStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BarcodeScannerStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BarcodeScannerStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BarcodeScannerStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.BarcodeScannerStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BarcodeSymbologyDecodeLengthKind(pub i32);
impl BarcodeSymbologyDecodeLengthKind {
    pub const AnyLength: Self = Self(0i32);
    pub const Discrete: Self = Self(1i32);
    pub const Range: Self = Self(2i32);
}
impl windows_core::TypeKind for BarcodeSymbologyDecodeLengthKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BarcodeSymbologyDecodeLengthKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BarcodeSymbologyDecodeLengthKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BarcodeSymbologyDecodeLengthKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.BarcodeSymbologyDecodeLengthKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CashDrawerStatusKind(pub i32);
impl CashDrawerStatusKind {
    pub const Online: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const Offline: Self = Self(2i32);
    pub const OffOrOffline: Self = Self(3i32);
    pub const Extended: Self = Self(4i32);
}
impl windows_core::TypeKind for CashDrawerStatusKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CashDrawerStatusKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CashDrawerStatusKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CashDrawerStatusKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.CashDrawerStatusKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LineDisplayCursorType(pub i32);
impl LineDisplayCursorType {
    pub const None: Self = Self(0i32);
    pub const Block: Self = Self(1i32);
    pub const HalfBlock: Self = Self(2i32);
    pub const Underline: Self = Self(3i32);
    pub const Reverse: Self = Self(4i32);
    pub const Other: Self = Self(5i32);
}
impl windows_core::TypeKind for LineDisplayCursorType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LineDisplayCursorType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LineDisplayCursorType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LineDisplayCursorType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayCursorType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LineDisplayDescriptorState(pub i32);
impl LineDisplayDescriptorState {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Blink: Self = Self(2i32);
}
impl windows_core::TypeKind for LineDisplayDescriptorState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LineDisplayDescriptorState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LineDisplayDescriptorState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LineDisplayDescriptorState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayDescriptorState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LineDisplayHorizontalAlignment(pub i32);
impl LineDisplayHorizontalAlignment {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl windows_core::TypeKind for LineDisplayHorizontalAlignment {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LineDisplayHorizontalAlignment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LineDisplayHorizontalAlignment").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LineDisplayHorizontalAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayHorizontalAlignment;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LineDisplayMarqueeFormat(pub i32);
impl LineDisplayMarqueeFormat {
    pub const None: Self = Self(0i32);
    pub const Walk: Self = Self(1i32);
    pub const Place: Self = Self(2i32);
}
impl windows_core::TypeKind for LineDisplayMarqueeFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LineDisplayMarqueeFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LineDisplayMarqueeFormat").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LineDisplayMarqueeFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayMarqueeFormat;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LineDisplayPowerStatus(pub i32);
impl LineDisplayPowerStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Online: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const Offline: Self = Self(3i32);
    pub const OffOrOffline: Self = Self(4i32);
}
impl windows_core::TypeKind for LineDisplayPowerStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LineDisplayPowerStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LineDisplayPowerStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LineDisplayPowerStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayPowerStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LineDisplayScrollDirection(pub i32);
impl LineDisplayScrollDirection {
    pub const Up: Self = Self(0i32);
    pub const Down: Self = Self(1i32);
    pub const Left: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
}
impl windows_core::TypeKind for LineDisplayScrollDirection {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LineDisplayScrollDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LineDisplayScrollDirection").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LineDisplayScrollDirection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayScrollDirection;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LineDisplayTextAttribute(pub i32);
impl LineDisplayTextAttribute {
    pub const Normal: Self = Self(0i32);
    pub const Blink: Self = Self(1i32);
    pub const Reverse: Self = Self(2i32);
    pub const ReverseBlink: Self = Self(3i32);
}
impl windows_core::TypeKind for LineDisplayTextAttribute {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LineDisplayTextAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LineDisplayTextAttribute").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LineDisplayTextAttribute {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayTextAttribute;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LineDisplayTextAttributeGranularity(pub i32);
impl LineDisplayTextAttributeGranularity {
    pub const NotSupported: Self = Self(0i32);
    pub const EntireDisplay: Self = Self(1i32);
    pub const PerCharacter: Self = Self(2i32);
}
impl windows_core::TypeKind for LineDisplayTextAttributeGranularity {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LineDisplayTextAttributeGranularity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LineDisplayTextAttributeGranularity").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LineDisplayTextAttributeGranularity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayTextAttributeGranularity;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LineDisplayVerticalAlignment(pub i32);
impl LineDisplayVerticalAlignment {
    pub const Top: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
}
impl windows_core::TypeKind for LineDisplayVerticalAlignment {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LineDisplayVerticalAlignment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LineDisplayVerticalAlignment").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LineDisplayVerticalAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayVerticalAlignment;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MagneticStripeReaderAuthenticationLevel(pub i32);
impl MagneticStripeReaderAuthenticationLevel {
    pub const NotSupported: Self = Self(0i32);
    pub const Optional: Self = Self(1i32);
    pub const Required: Self = Self(2i32);
}
impl windows_core::TypeKind for MagneticStripeReaderAuthenticationLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MagneticStripeReaderAuthenticationLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderAuthenticationLevel").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MagneticStripeReaderAuthenticationLevel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderAuthenticationLevel;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MagneticStripeReaderAuthenticationProtocol(pub i32);
impl MagneticStripeReaderAuthenticationProtocol {
    pub const None: Self = Self(0i32);
    pub const ChallengeResponse: Self = Self(1i32);
}
impl windows_core::TypeKind for MagneticStripeReaderAuthenticationProtocol {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MagneticStripeReaderAuthenticationProtocol {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderAuthenticationProtocol").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MagneticStripeReaderAuthenticationProtocol {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderAuthenticationProtocol;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MagneticStripeReaderErrorReportingType(pub i32);
impl MagneticStripeReaderErrorReportingType {
    pub const CardLevel: Self = Self(0i32);
    pub const TrackLevel: Self = Self(1i32);
}
impl windows_core::TypeKind for MagneticStripeReaderErrorReportingType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MagneticStripeReaderErrorReportingType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderErrorReportingType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MagneticStripeReaderErrorReportingType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderErrorReportingType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MagneticStripeReaderStatus(pub i32);
impl MagneticStripeReaderStatus {
    pub const Unauthenticated: Self = Self(0i32);
    pub const Authenticated: Self = Self(1i32);
    pub const Extended: Self = Self(2i32);
}
impl windows_core::TypeKind for MagneticStripeReaderStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MagneticStripeReaderStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MagneticStripeReaderStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MagneticStripeReaderTrackErrorType(pub i32);
impl MagneticStripeReaderTrackErrorType {
    pub const None: Self = Self(0i32);
    pub const StartSentinelError: Self = Self(1i32);
    pub const EndSentinelError: Self = Self(2i32);
    pub const ParityError: Self = Self(3i32);
    pub const LrcError: Self = Self(4i32);
    pub const Unknown: Self = Self(-1i32);
}
impl windows_core::TypeKind for MagneticStripeReaderTrackErrorType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MagneticStripeReaderTrackErrorType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderTrackErrorType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MagneticStripeReaderTrackErrorType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderTrackErrorType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MagneticStripeReaderTrackIds(pub i32);
impl MagneticStripeReaderTrackIds {
    pub const None: Self = Self(0i32);
    pub const Track1: Self = Self(1i32);
    pub const Track2: Self = Self(2i32);
    pub const Track3: Self = Self(4i32);
    pub const Track4: Self = Self(8i32);
}
impl windows_core::TypeKind for MagneticStripeReaderTrackIds {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MagneticStripeReaderTrackIds {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderTrackIds").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MagneticStripeReaderTrackIds {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderTrackIds;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PosConnectionTypes(pub u32);
impl PosConnectionTypes {
    pub const Local: Self = Self(1u32);
    pub const IP: Self = Self(2u32);
    pub const Bluetooth: Self = Self(4u32);
    pub const All: Self = Self(4294967295u32);
}
impl windows_core::TypeKind for PosConnectionTypes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PosConnectionTypes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PosConnectionTypes").field(&self.0).finish()
    }
}
impl PosConnectionTypes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PosConnectionTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PosConnectionTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PosConnectionTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PosConnectionTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PosConnectionTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for PosConnectionTypes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosConnectionTypes;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PosPrinterAlignment(pub i32);
impl PosPrinterAlignment {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl windows_core::TypeKind for PosPrinterAlignment {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PosPrinterAlignment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PosPrinterAlignment").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PosPrinterAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterAlignment;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PosPrinterBarcodeTextPosition(pub i32);
impl PosPrinterBarcodeTextPosition {
    pub const None: Self = Self(0i32);
    pub const Above: Self = Self(1i32);
    pub const Below: Self = Self(2i32);
}
impl windows_core::TypeKind for PosPrinterBarcodeTextPosition {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PosPrinterBarcodeTextPosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PosPrinterBarcodeTextPosition").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PosPrinterBarcodeTextPosition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterBarcodeTextPosition;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PosPrinterCartridgeSensors(pub u32);
impl PosPrinterCartridgeSensors {
    pub const None: Self = Self(0u32);
    pub const Removed: Self = Self(1u32);
    pub const Empty: Self = Self(2u32);
    pub const HeadCleaning: Self = Self(4u32);
    pub const NearEnd: Self = Self(8u32);
}
impl windows_core::TypeKind for PosPrinterCartridgeSensors {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PosPrinterCartridgeSensors {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PosPrinterCartridgeSensors").field(&self.0).finish()
    }
}
impl PosPrinterCartridgeSensors {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PosPrinterCartridgeSensors {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PosPrinterCartridgeSensors {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PosPrinterCartridgeSensors {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PosPrinterCartridgeSensors {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PosPrinterCartridgeSensors {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for PosPrinterCartridgeSensors {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterCartridgeSensors;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PosPrinterColorCapabilities(pub u32);
impl PosPrinterColorCapabilities {
    pub const None: Self = Self(0u32);
    pub const Primary: Self = Self(1u32);
    pub const Custom1: Self = Self(2u32);
    pub const Custom2: Self = Self(4u32);
    pub const Custom3: Self = Self(8u32);
    pub const Custom4: Self = Self(16u32);
    pub const Custom5: Self = Self(32u32);
    pub const Custom6: Self = Self(64u32);
    pub const Cyan: Self = Self(128u32);
    pub const Magenta: Self = Self(256u32);
    pub const Yellow: Self = Self(512u32);
    pub const Full: Self = Self(1024u32);
}
impl windows_core::TypeKind for PosPrinterColorCapabilities {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PosPrinterColorCapabilities {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PosPrinterColorCapabilities").field(&self.0).finish()
    }
}
impl PosPrinterColorCapabilities {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PosPrinterColorCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PosPrinterColorCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PosPrinterColorCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PosPrinterColorCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PosPrinterColorCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for PosPrinterColorCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterColorCapabilities;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PosPrinterColorCartridge(pub i32);
impl PosPrinterColorCartridge {
    pub const Unknown: Self = Self(0i32);
    pub const Primary: Self = Self(1i32);
    pub const Custom1: Self = Self(2i32);
    pub const Custom2: Self = Self(3i32);
    pub const Custom3: Self = Self(4i32);
    pub const Custom4: Self = Self(5i32);
    pub const Custom5: Self = Self(6i32);
    pub const Custom6: Self = Self(7i32);
    pub const Cyan: Self = Self(8i32);
    pub const Magenta: Self = Self(9i32);
    pub const Yellow: Self = Self(10i32);
}
impl windows_core::TypeKind for PosPrinterColorCartridge {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PosPrinterColorCartridge {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PosPrinterColorCartridge").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PosPrinterColorCartridge {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterColorCartridge;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PosPrinterLineDirection(pub i32);
impl PosPrinterLineDirection {
    pub const Horizontal: Self = Self(0i32);
    pub const Vertical: Self = Self(1i32);
}
impl windows_core::TypeKind for PosPrinterLineDirection {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PosPrinterLineDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PosPrinterLineDirection").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PosPrinterLineDirection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterLineDirection;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PosPrinterLineStyle(pub i32);
impl PosPrinterLineStyle {
    pub const SingleSolid: Self = Self(0i32);
    pub const DoubleSolid: Self = Self(1i32);
    pub const Broken: Self = Self(2i32);
    pub const Chain: Self = Self(3i32);
}
impl windows_core::TypeKind for PosPrinterLineStyle {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PosPrinterLineStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PosPrinterLineStyle").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PosPrinterLineStyle {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterLineStyle;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PosPrinterMapMode(pub i32);
impl PosPrinterMapMode {
    pub const Dots: Self = Self(0i32);
    pub const Twips: Self = Self(1i32);
    pub const English: Self = Self(2i32);
    pub const Metric: Self = Self(3i32);
}
impl windows_core::TypeKind for PosPrinterMapMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PosPrinterMapMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PosPrinterMapMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PosPrinterMapMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMapMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PosPrinterMarkFeedCapabilities(pub u32);
impl PosPrinterMarkFeedCapabilities {
    pub const None: Self = Self(0u32);
    pub const ToTakeUp: Self = Self(1u32);
    pub const ToCutter: Self = Self(2u32);
    pub const ToCurrentTopOfForm: Self = Self(4u32);
    pub const ToNextTopOfForm: Self = Self(8u32);
}
impl windows_core::TypeKind for PosPrinterMarkFeedCapabilities {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PosPrinterMarkFeedCapabilities {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PosPrinterMarkFeedCapabilities").field(&self.0).finish()
    }
}
impl PosPrinterMarkFeedCapabilities {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PosPrinterMarkFeedCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PosPrinterMarkFeedCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for PosPrinterMarkFeedCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMarkFeedCapabilities;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PosPrinterMarkFeedKind(pub i32);
impl PosPrinterMarkFeedKind {
    pub const ToTakeUp: Self = Self(0i32);
    pub const ToCutter: Self = Self(1i32);
    pub const ToCurrentTopOfForm: Self = Self(2i32);
    pub const ToNextTopOfForm: Self = Self(3i32);
}
impl windows_core::TypeKind for PosPrinterMarkFeedKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PosPrinterMarkFeedKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PosPrinterMarkFeedKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PosPrinterMarkFeedKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMarkFeedKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PosPrinterPrintSide(pub i32);
impl PosPrinterPrintSide {
    pub const Unknown: Self = Self(0i32);
    pub const Side1: Self = Self(1i32);
    pub const Side2: Self = Self(2i32);
}
impl windows_core::TypeKind for PosPrinterPrintSide {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PosPrinterPrintSide {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PosPrinterPrintSide").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PosPrinterPrintSide {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterPrintSide;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PosPrinterRotation(pub i32);
impl PosPrinterRotation {
    pub const Normal: Self = Self(0i32);
    pub const Right90: Self = Self(1i32);
    pub const Left90: Self = Self(2i32);
    pub const Rotate180: Self = Self(3i32);
}
impl windows_core::TypeKind for PosPrinterRotation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PosPrinterRotation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PosPrinterRotation").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PosPrinterRotation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterRotation;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PosPrinterRuledLineCapabilities(pub u32);
impl PosPrinterRuledLineCapabilities {
    pub const None: Self = Self(0u32);
    pub const Horizontal: Self = Self(1u32);
    pub const Vertical: Self = Self(2u32);
}
impl windows_core::TypeKind for PosPrinterRuledLineCapabilities {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PosPrinterRuledLineCapabilities {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PosPrinterRuledLineCapabilities").field(&self.0).finish()
    }
}
impl PosPrinterRuledLineCapabilities {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PosPrinterRuledLineCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PosPrinterRuledLineCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for PosPrinterRuledLineCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterRuledLineCapabilities;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PosPrinterStatusKind(pub i32);
impl PosPrinterStatusKind {
    pub const Online: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const Offline: Self = Self(2i32);
    pub const OffOrOffline: Self = Self(3i32);
    pub const Extended: Self = Self(4i32);
}
impl windows_core::TypeKind for PosPrinterStatusKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PosPrinterStatusKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PosPrinterStatusKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PosPrinterStatusKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterStatusKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UnifiedPosErrorReason(pub i32);
impl UnifiedPosErrorReason {
    pub const UnknownErrorReason: Self = Self(0i32);
    pub const NoService: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
    pub const Illegal: Self = Self(3i32);
    pub const NoHardware: Self = Self(4i32);
    pub const Closed: Self = Self(5i32);
    pub const Offline: Self = Self(6i32);
    pub const Failure: Self = Self(7i32);
    pub const Timeout: Self = Self(8i32);
    pub const Busy: Self = Self(9i32);
    pub const Extended: Self = Self(10i32);
}
impl windows_core::TypeKind for UnifiedPosErrorReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UnifiedPosErrorReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UnifiedPosErrorReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UnifiedPosErrorReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosErrorReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UnifiedPosErrorSeverity(pub i32);
impl UnifiedPosErrorSeverity {
    pub const UnknownErrorSeverity: Self = Self(0i32);
    pub const Warning: Self = Self(1i32);
    pub const Recoverable: Self = Self(2i32);
    pub const Unrecoverable: Self = Self(3i32);
    pub const AssistanceRequired: Self = Self(4i32);
    pub const Fatal: Self = Self(5i32);
}
impl windows_core::TypeKind for UnifiedPosErrorSeverity {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UnifiedPosErrorSeverity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UnifiedPosErrorSeverity").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UnifiedPosErrorSeverity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosErrorSeverity;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UnifiedPosHealthCheckLevel(pub i32);
impl UnifiedPosHealthCheckLevel {
    pub const UnknownHealthCheckLevel: Self = Self(0i32);
    pub const POSInternal: Self = Self(1i32);
    pub const External: Self = Self(2i32);
    pub const Interactive: Self = Self(3i32);
}
impl windows_core::TypeKind for UnifiedPosHealthCheckLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UnifiedPosHealthCheckLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UnifiedPosHealthCheckLevel").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UnifiedPosHealthCheckLevel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosHealthCheckLevel;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UnifiedPosPowerReportingType(pub i32);
impl UnifiedPosPowerReportingType {
    pub const UnknownPowerReportingType: Self = Self(0i32);
    pub const Standard: Self = Self(1i32);
    pub const Advanced: Self = Self(2i32);
}
impl windows_core::TypeKind for UnifiedPosPowerReportingType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UnifiedPosPowerReportingType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UnifiedPosPowerReportingType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UnifiedPosPowerReportingType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosPowerReportingType;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SizeUInt32 {
    pub Width: u32,
    pub Height: u32,
}
impl windows_core::TypeKind for SizeUInt32 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SizeUInt32 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.PointOfService.SizeUInt32;u4;u4)");
}
impl Default for SizeUInt32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
