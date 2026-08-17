windows_core::imp::define_interface!(IBarcodeScannerDisableScannerRequest, IBarcodeScannerDisableScannerRequest_Vtbl, 0x88ecf7c0_37b9_4275_8e77_c8e52ae5a9c8);
impl windows_core::RuntimeType for IBarcodeScannerDisableScannerRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerDisableScannerRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerDisableScannerRequest2, IBarcodeScannerDisableScannerRequest2_Vtbl, 0xccdfe625_65c3_4ccc_b457_f39c7a9ea60d);
impl windows_core::RuntimeType for IBarcodeScannerDisableScannerRequest2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerDisableScannerRequest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerDisableScannerRequestEventArgs, IBarcodeScannerDisableScannerRequestEventArgs_Vtbl, 0x7006e142_e802_46f5_b604_352a15ce9232);
impl windows_core::RuntimeType for IBarcodeScannerDisableScannerRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerDisableScannerRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerEnableScannerRequest, IBarcodeScannerEnableScannerRequest_Vtbl, 0xc0b3e9ba_816a_452b_bd77_b7e453ec446d);
impl windows_core::RuntimeType for IBarcodeScannerEnableScannerRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerEnableScannerRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerEnableScannerRequest2, IBarcodeScannerEnableScannerRequest2_Vtbl, 0x71a4f2a8_9905_41ac_9121_b645916a84a1);
impl windows_core::RuntimeType for IBarcodeScannerEnableScannerRequest2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerEnableScannerRequest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerEnableScannerRequestEventArgs, IBarcodeScannerEnableScannerRequestEventArgs_Vtbl, 0x956c9419_7b4e_4451_8c41_8e10cfbc5b41);
impl windows_core::RuntimeType for IBarcodeScannerEnableScannerRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerEnableScannerRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerFrameReader, IBarcodeScannerFrameReader_Vtbl, 0xdbc72b07_64c3_482b_93c8_65fb33c22208);
impl windows_core::RuntimeType for IBarcodeScannerFrameReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerFrameReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryAcquireLatestFrameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FrameArrived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFrameArrived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerFrameReaderFrameArrivedEventArgs, IBarcodeScannerFrameReaderFrameArrivedEventArgs_Vtbl, 0xb0bbd604_54fd_436d_8629_712e787223dd);
impl windows_core::RuntimeType for IBarcodeScannerFrameReaderFrameArrivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerFrameReaderFrameArrivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerGetSymbologyAttributesRequest, IBarcodeScannerGetSymbologyAttributesRequest_Vtbl, 0x9774c46a_58e4_4c5f_b8e9_e41467632700);
impl windows_core::RuntimeType for IBarcodeScannerGetSymbologyAttributesRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Symbology: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerGetSymbologyAttributesRequest2, IBarcodeScannerGetSymbologyAttributesRequest2_Vtbl, 0x6a6a2b13_75a8_49fb_b852_bfb93d760af7);
impl windows_core::RuntimeType for IBarcodeScannerGetSymbologyAttributesRequest2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerGetSymbologyAttributesRequestEventArgs, IBarcodeScannerGetSymbologyAttributesRequestEventArgs_Vtbl, 0x7f89de3e_fb5d_493c_b402_356b24d574a6);
impl windows_core::RuntimeType for IBarcodeScannerGetSymbologyAttributesRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerGetSymbologyAttributesRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerHideVideoPreviewRequest, IBarcodeScannerHideVideoPreviewRequest_Vtbl, 0xfa4ebe7f_6670_40e1_b90b_bb10d8d425fa);
impl windows_core::RuntimeType for IBarcodeScannerHideVideoPreviewRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerHideVideoPreviewRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerHideVideoPreviewRequest2, IBarcodeScannerHideVideoPreviewRequest2_Vtbl, 0x7e28435d_9814_431d_a2f2_d6248c5ad4b5);
impl windows_core::RuntimeType for IBarcodeScannerHideVideoPreviewRequest2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerHideVideoPreviewRequest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerHideVideoPreviewRequestEventArgs, IBarcodeScannerHideVideoPreviewRequestEventArgs_Vtbl, 0x16a281fc_d6be_4bc7_9df1_33741f3eadea);
impl windows_core::RuntimeType for IBarcodeScannerHideVideoPreviewRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerHideVideoPreviewRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerProviderConnection, IBarcodeScannerProviderConnection_Vtbl, 0xb44acbed_0b3a_4fa3_86c5_491ea30780eb);
impl windows_core::RuntimeType for IBarcodeScannerProviderConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerProviderConnection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedSymbologies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedSymbologies: usize,
    pub CompanyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCompanyName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportScannedDataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportTriggerStateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, BarcodeScannerTriggerState, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportErrorAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportErrorAsyncWithScanReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableScannerRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEnableScannerRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DisableScannerRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDisableScannerRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SetActiveSymbologiesRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSetActiveSymbologiesRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub StartSoftwareTriggerRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStartSoftwareTriggerRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub StopSoftwareTriggerRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStopSoftwareTriggerRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub GetBarcodeSymbologyAttributesRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveGetBarcodeSymbologyAttributesRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SetBarcodeSymbologyAttributesRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSetBarcodeSymbologyAttributesRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub HideVideoPreviewRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveHideVideoPreviewRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerProviderConnection2, IBarcodeScannerProviderConnection2_Vtbl, 0xbe9b53cd_1134_418c_a06b_04423a73f3d7);
impl windows_core::RuntimeType for IBarcodeScannerProviderConnection2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerProviderConnection2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFrameReaderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub CreateFrameReaderWithFormatAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Graphics::Imaging::BitmapPixelFormat, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CreateFrameReaderWithFormatAsync: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub CreateFrameReaderWithFormatAndSizeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Graphics::Imaging::BitmapPixelFormat, super::super::super::Graphics::Imaging::BitmapSize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CreateFrameReaderWithFormatAndSizeAsync: usize,
}
windows_core::imp::define_interface!(IBarcodeScannerProviderTriggerDetails, IBarcodeScannerProviderTriggerDetails_Vtbl, 0x50856d82_24e3_48ce_99c7_70aac1cbc9f7);
impl windows_core::RuntimeType for IBarcodeScannerProviderTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerProviderTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerSetActiveSymbologiesRequest, IBarcodeScannerSetActiveSymbologiesRequest_Vtbl, 0xdb3f32b9_f7da_41a1_9f79_07bcd95f0bdf);
impl windows_core::RuntimeType for IBarcodeScannerSetActiveSymbologiesRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Symbologies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Symbologies: usize,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerSetActiveSymbologiesRequest2, IBarcodeScannerSetActiveSymbologiesRequest2_Vtbl, 0xf5ff6edf_fa9a_4749_b11b_e8fccb75bc6b);
impl windows_core::RuntimeType for IBarcodeScannerSetActiveSymbologiesRequest2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerSetActiveSymbologiesRequestEventArgs, IBarcodeScannerSetActiveSymbologiesRequestEventArgs_Vtbl, 0x06305afa_7bf6_4d52_801a_330272f60ae1);
impl windows_core::RuntimeType for IBarcodeScannerSetActiveSymbologiesRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerSetActiveSymbologiesRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerSetSymbologyAttributesRequest, IBarcodeScannerSetSymbologyAttributesRequest_Vtbl, 0x32fb814f_a37f_48b0_acea_dce1480f12ae);
impl windows_core::RuntimeType for IBarcodeScannerSetSymbologyAttributesRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Symbology: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerSetSymbologyAttributesRequest2, IBarcodeScannerSetSymbologyAttributesRequest2_Vtbl, 0xdffbbfc1_dba8_4b77_be1e_b56cd72f65b3);
impl windows_core::RuntimeType for IBarcodeScannerSetSymbologyAttributesRequest2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerSetSymbologyAttributesRequestEventArgs, IBarcodeScannerSetSymbologyAttributesRequestEventArgs_Vtbl, 0xb2b89809_9824_47d4_85bd_d0077baa7bd2);
impl windows_core::RuntimeType for IBarcodeScannerSetSymbologyAttributesRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerSetSymbologyAttributesRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerStartSoftwareTriggerRequest, IBarcodeScannerStartSoftwareTriggerRequest_Vtbl, 0xe3fa7b27_ff62_4454_af4a_cb6144a3e3f7);
impl windows_core::RuntimeType for IBarcodeScannerStartSoftwareTriggerRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerStartSoftwareTriggerRequest2, IBarcodeScannerStartSoftwareTriggerRequest2_Vtbl, 0xeb72a25c_6658_4765_a68e_327482653deb);
impl windows_core::RuntimeType for IBarcodeScannerStartSoftwareTriggerRequest2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerStartSoftwareTriggerRequestEventArgs, IBarcodeScannerStartSoftwareTriggerRequestEventArgs_Vtbl, 0x2305d843_c88f_4f3b_8c3b_d3df071051ec);
impl windows_core::RuntimeType for IBarcodeScannerStartSoftwareTriggerRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerStartSoftwareTriggerRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerStopSoftwareTriggerRequest, IBarcodeScannerStopSoftwareTriggerRequest_Vtbl, 0x6f9faf35_e287_4ca8_b70d_5a91d694f668);
impl windows_core::RuntimeType for IBarcodeScannerStopSoftwareTriggerRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerStopSoftwareTriggerRequest2, IBarcodeScannerStopSoftwareTriggerRequest2_Vtbl, 0xcb57c5dd_fe50_49f8_a0b4_bdc230814da2);
impl windows_core::RuntimeType for IBarcodeScannerStopSoftwareTriggerRequest2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerStopSoftwareTriggerRequestEventArgs, IBarcodeScannerStopSoftwareTriggerRequestEventArgs_Vtbl, 0xeac34450_4eb7_481a_9273_147a273b99b8);
impl windows_core::RuntimeType for IBarcodeScannerStopSoftwareTriggerRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerStopSoftwareTriggerRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarcodeScannerVideoFrame, IBarcodeScannerVideoFrame_Vtbl, 0x7e585248_9df7_4121_a175_801d8000112e);
impl windows_core::RuntimeType for IBarcodeScannerVideoFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeScannerVideoFrame_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::Imaging::BitmapPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    Format: usize,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub PixelData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PixelData: usize,
}
windows_core::imp::define_interface!(IBarcodeSymbologyAttributesBuilder, IBarcodeSymbologyAttributesBuilder_Vtbl, 0xc57b0cbf_e4f5_40b9_84cf_e63fbaea42b4);
impl windows_core::RuntimeType for IBarcodeSymbologyAttributesBuilder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarcodeSymbologyAttributesBuilder_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsCheckDigitValidationSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCheckDigitValidationSupported: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsCheckDigitTransmissionSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCheckDigitTransmissionSupported: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsDecodeLengthSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsDecodeLengthSupported: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CreateAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerDisableScannerRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerDisableScannerRequest, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerDisableScannerRequest {
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerDisableScannerRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(windows_core::Interface::as_raw(this), reason, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerDisableScannerRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(windows_core::Interface::as_raw(this), reason, core::mem::transmute_copy(failedreasondescription), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerDisableScannerRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerDisableScannerRequest>();
}
unsafe impl windows_core::Interface for BarcodeScannerDisableScannerRequest {
    type Vtable = IBarcodeScannerDisableScannerRequest_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerDisableScannerRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerDisableScannerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequest";
}
unsafe impl Send for BarcodeScannerDisableScannerRequest {}
unsafe impl Sync for BarcodeScannerDisableScannerRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerDisableScannerRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerDisableScannerRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerDisableScannerRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<BarcodeScannerDisableScannerRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerDisableScannerRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerDisableScannerRequestEventArgs>();
}
unsafe impl windows_core::Interface for BarcodeScannerDisableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerDisableScannerRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerDisableScannerRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerDisableScannerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequestEventArgs";
}
unsafe impl Send for BarcodeScannerDisableScannerRequestEventArgs {}
unsafe impl Sync for BarcodeScannerDisableScannerRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerEnableScannerRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerEnableScannerRequest, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerEnableScannerRequest {
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerEnableScannerRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(windows_core::Interface::as_raw(this), reason, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerEnableScannerRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(windows_core::Interface::as_raw(this), reason, core::mem::transmute_copy(failedreasondescription), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerEnableScannerRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerEnableScannerRequest>();
}
unsafe impl windows_core::Interface for BarcodeScannerEnableScannerRequest {
    type Vtable = IBarcodeScannerEnableScannerRequest_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerEnableScannerRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerEnableScannerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequest";
}
unsafe impl Send for BarcodeScannerEnableScannerRequest {}
unsafe impl Sync for BarcodeScannerEnableScannerRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerEnableScannerRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerEnableScannerRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerEnableScannerRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<BarcodeScannerEnableScannerRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerEnableScannerRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerEnableScannerRequestEventArgs>();
}
unsafe impl windows_core::Interface for BarcodeScannerEnableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerEnableScannerRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerEnableScannerRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerEnableScannerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequestEventArgs";
}
unsafe impl Send for BarcodeScannerEnableScannerRequestEventArgs {}
unsafe impl Sync for BarcodeScannerEnableScannerRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerFrameReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerFrameReader, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BarcodeScannerFrameReader, super::super::super::Foundation::IClosable);
impl BarcodeScannerFrameReader {
    pub fn StartAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StopAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryAcquireLatestFrameAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerVideoFrame>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryAcquireLatestFrameAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Connection(&self) -> windows_core::Result<BarcodeScannerProviderConnection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Connection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FrameArrived<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<BarcodeScannerFrameReader, BarcodeScannerFrameReaderFrameArrivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameArrived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFrameArrived(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFrameArrived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for BarcodeScannerFrameReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerFrameReader>();
}
unsafe impl windows_core::Interface for BarcodeScannerFrameReader {
    type Vtable = IBarcodeScannerFrameReader_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerFrameReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerFrameReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReader";
}
unsafe impl Send for BarcodeScannerFrameReader {}
unsafe impl Sync for BarcodeScannerFrameReader {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerFrameReaderFrameArrivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerFrameReaderFrameArrivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerFrameReaderFrameArrivedEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerFrameReaderFrameArrivedEventArgs>();
}
unsafe impl windows_core::Interface for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    type Vtable = IBarcodeScannerFrameReaderFrameArrivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerFrameReaderFrameArrivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReaderFrameArrivedEventArgs";
}
unsafe impl Send for BarcodeScannerFrameReaderFrameArrivedEventArgs {}
unsafe impl Sync for BarcodeScannerFrameReaderFrameArrivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerGetSymbologyAttributesRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerGetSymbologyAttributesRequest, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerGetSymbologyAttributesRequest {
    pub fn Symbology(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Symbology)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportCompletedAsync<P0>(&self, attributes: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::BarcodeSymbologyAttributes>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), attributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerGetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(windows_core::Interface::as_raw(this), reason, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerGetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(windows_core::Interface::as_raw(this), reason, core::mem::transmute_copy(failedreasondescription), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerGetSymbologyAttributesRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerGetSymbologyAttributesRequest>();
}
unsafe impl windows_core::Interface for BarcodeScannerGetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequest_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerGetSymbologyAttributesRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerGetSymbologyAttributesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequest";
}
unsafe impl Send for BarcodeScannerGetSymbologyAttributesRequest {}
unsafe impl Sync for BarcodeScannerGetSymbologyAttributesRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerGetSymbologyAttributesRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerGetSymbologyAttributesRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<BarcodeScannerGetSymbologyAttributesRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerGetSymbologyAttributesRequestEventArgs>();
}
unsafe impl windows_core::Interface for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerGetSymbologyAttributesRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequestEventArgs";
}
unsafe impl Send for BarcodeScannerGetSymbologyAttributesRequestEventArgs {}
unsafe impl Sync for BarcodeScannerGetSymbologyAttributesRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerHideVideoPreviewRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerHideVideoPreviewRequest, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerHideVideoPreviewRequest {
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerHideVideoPreviewRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(windows_core::Interface::as_raw(this), reason, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerHideVideoPreviewRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(windows_core::Interface::as_raw(this), reason, core::mem::transmute_copy(failedreasondescription), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerHideVideoPreviewRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerHideVideoPreviewRequest>();
}
unsafe impl windows_core::Interface for BarcodeScannerHideVideoPreviewRequest {
    type Vtable = IBarcodeScannerHideVideoPreviewRequest_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerHideVideoPreviewRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerHideVideoPreviewRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequest";
}
unsafe impl Send for BarcodeScannerHideVideoPreviewRequest {}
unsafe impl Sync for BarcodeScannerHideVideoPreviewRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerHideVideoPreviewRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerHideVideoPreviewRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerHideVideoPreviewRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<BarcodeScannerHideVideoPreviewRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerHideVideoPreviewRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerHideVideoPreviewRequestEventArgs>();
}
unsafe impl windows_core::Interface for BarcodeScannerHideVideoPreviewRequestEventArgs {
    type Vtable = IBarcodeScannerHideVideoPreviewRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerHideVideoPreviewRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerHideVideoPreviewRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequestEventArgs";
}
unsafe impl Send for BarcodeScannerHideVideoPreviewRequestEventArgs {}
unsafe impl Sync for BarcodeScannerHideVideoPreviewRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerProviderConnection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerProviderConnection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BarcodeScannerProviderConnection, super::super::super::Foundation::IClosable);
impl BarcodeScannerProviderConnection {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedSymbologies(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedSymbologies)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CompanyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompanyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCompanyName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCompanyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Version(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Version)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetVersion(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVersion)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportScannedDataAsync<P0>(&self, report: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::BarcodeScannerReport>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportScannedDataAsync)(windows_core::Interface::as_raw(this), report.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportTriggerStateAsync(&self, state: BarcodeScannerTriggerState) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportTriggerStateAsync)(windows_core::Interface::as_raw(this), state, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportErrorAsync<P0>(&self, errordata: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::UnifiedPosErrorData>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportErrorAsync)(windows_core::Interface::as_raw(this), errordata.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportErrorAsyncWithScanReport<P0, P1>(&self, errordata: P0, isretriable: bool, scanreport: P1) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::UnifiedPosErrorData>,
        P1: windows_core::Param<super::BarcodeScannerReport>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportErrorAsyncWithScanReport)(windows_core::Interface::as_raw(this), errordata.param().abi(), isretriable, scanreport.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EnableScannerRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerEnableScannerRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnableScannerRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveEnableScannerRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveEnableScannerRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DisableScannerRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerDisableScannerRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisableScannerRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDisableScannerRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDisableScannerRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetActiveSymbologiesRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetActiveSymbologiesRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetActiveSymbologiesRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSetActiveSymbologiesRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSetActiveSymbologiesRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn StartSoftwareTriggerRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStartSoftwareTriggerRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartSoftwareTriggerRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStartSoftwareTriggerRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStartSoftwareTriggerRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn StopSoftwareTriggerRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStopSoftwareTriggerRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopSoftwareTriggerRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStopSoftwareTriggerRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStopSoftwareTriggerRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetBarcodeSymbologyAttributesRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerGetSymbologyAttributesRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBarcodeSymbologyAttributesRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveGetBarcodeSymbologyAttributesRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveGetBarcodeSymbologyAttributesRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetBarcodeSymbologyAttributesRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetSymbologyAttributesRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetBarcodeSymbologyAttributesRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSetBarcodeSymbologyAttributesRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSetBarcodeSymbologyAttributesRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn HideVideoPreviewRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerHideVideoPreviewRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HideVideoPreviewRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHideVideoPreviewRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveHideVideoPreviewRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateFrameReaderAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerProviderConnection2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFrameReaderAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn CreateFrameReaderWithFormatAsync(&self, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerProviderConnection2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFrameReaderWithFormatAsync)(windows_core::Interface::as_raw(this), preferredformat, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn CreateFrameReaderWithFormatAndSizeAsync(&self, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat, preferredsize: super::super::super::Graphics::Imaging::BitmapSize) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerProviderConnection2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFrameReaderWithFormatAndSizeAsync)(windows_core::Interface::as_raw(this), preferredformat, preferredsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for BarcodeScannerProviderConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerProviderConnection>();
}
unsafe impl windows_core::Interface for BarcodeScannerProviderConnection {
    type Vtable = IBarcodeScannerProviderConnection_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerProviderConnection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerProviderConnection {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection";
}
unsafe impl Send for BarcodeScannerProviderConnection {}
unsafe impl Sync for BarcodeScannerProviderConnection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerProviderTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerProviderTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerProviderTriggerDetails {
    pub fn Connection(&self) -> windows_core::Result<BarcodeScannerProviderConnection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Connection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerProviderTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerProviderTriggerDetails>();
}
unsafe impl windows_core::Interface for BarcodeScannerProviderTriggerDetails {
    type Vtable = IBarcodeScannerProviderTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerProviderTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerProviderTriggerDetails {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerProviderTriggerDetails";
}
unsafe impl Send for BarcodeScannerProviderTriggerDetails {}
unsafe impl Sync for BarcodeScannerProviderTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerSetActiveSymbologiesRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerSetActiveSymbologiesRequest, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerSetActiveSymbologiesRequest {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Symbologies(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Symbologies)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerSetActiveSymbologiesRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(windows_core::Interface::as_raw(this), reason, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerSetActiveSymbologiesRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(windows_core::Interface::as_raw(this), reason, core::mem::transmute_copy(failedreasondescription), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerSetActiveSymbologiesRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerSetActiveSymbologiesRequest>();
}
unsafe impl windows_core::Interface for BarcodeScannerSetActiveSymbologiesRequest {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequest_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerSetActiveSymbologiesRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerSetActiveSymbologiesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequest";
}
unsafe impl Send for BarcodeScannerSetActiveSymbologiesRequest {}
unsafe impl Sync for BarcodeScannerSetActiveSymbologiesRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerSetActiveSymbologiesRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerSetActiveSymbologiesRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<BarcodeScannerSetActiveSymbologiesRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerSetActiveSymbologiesRequestEventArgs>();
}
unsafe impl windows_core::Interface for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerSetActiveSymbologiesRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequestEventArgs";
}
unsafe impl Send for BarcodeScannerSetActiveSymbologiesRequestEventArgs {}
unsafe impl Sync for BarcodeScannerSetActiveSymbologiesRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerSetSymbologyAttributesRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerSetSymbologyAttributesRequest, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerSetSymbologyAttributesRequest {
    pub fn Symbology(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Symbology)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<super::BarcodeSymbologyAttributes> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerSetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(windows_core::Interface::as_raw(this), reason, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerSetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(windows_core::Interface::as_raw(this), reason, core::mem::transmute_copy(failedreasondescription), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerSetSymbologyAttributesRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerSetSymbologyAttributesRequest>();
}
unsafe impl windows_core::Interface for BarcodeScannerSetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequest_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerSetSymbologyAttributesRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerSetSymbologyAttributesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequest";
}
unsafe impl Send for BarcodeScannerSetSymbologyAttributesRequest {}
unsafe impl Sync for BarcodeScannerSetSymbologyAttributesRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerSetSymbologyAttributesRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerSetSymbologyAttributesRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<BarcodeScannerSetSymbologyAttributesRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerSetSymbologyAttributesRequestEventArgs>();
}
unsafe impl windows_core::Interface for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerSetSymbologyAttributesRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequestEventArgs";
}
unsafe impl Send for BarcodeScannerSetSymbologyAttributesRequestEventArgs {}
unsafe impl Sync for BarcodeScannerSetSymbologyAttributesRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerStartSoftwareTriggerRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerStartSoftwareTriggerRequest, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerStartSoftwareTriggerRequest {
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerStartSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(windows_core::Interface::as_raw(this), reason, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerStartSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(windows_core::Interface::as_raw(this), reason, core::mem::transmute_copy(failedreasondescription), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerStartSoftwareTriggerRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerStartSoftwareTriggerRequest>();
}
unsafe impl windows_core::Interface for BarcodeScannerStartSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequest_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerStartSoftwareTriggerRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerStartSoftwareTriggerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequest";
}
unsafe impl Send for BarcodeScannerStartSoftwareTriggerRequest {}
unsafe impl Sync for BarcodeScannerStartSoftwareTriggerRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerStartSoftwareTriggerRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerStartSoftwareTriggerRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<BarcodeScannerStartSoftwareTriggerRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerStartSoftwareTriggerRequestEventArgs>();
}
unsafe impl windows_core::Interface for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerStartSoftwareTriggerRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequestEventArgs";
}
unsafe impl Send for BarcodeScannerStartSoftwareTriggerRequestEventArgs {}
unsafe impl Sync for BarcodeScannerStartSoftwareTriggerRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerStopSoftwareTriggerRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerStopSoftwareTriggerRequest, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerStopSoftwareTriggerRequest {
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerStopSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(windows_core::Interface::as_raw(this), reason, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IBarcodeScannerStopSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(windows_core::Interface::as_raw(this), reason, core::mem::transmute_copy(failedreasondescription), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerStopSoftwareTriggerRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerStopSoftwareTriggerRequest>();
}
unsafe impl windows_core::Interface for BarcodeScannerStopSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequest_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerStopSoftwareTriggerRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerStopSoftwareTriggerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequest";
}
unsafe impl Send for BarcodeScannerStopSoftwareTriggerRequest {}
unsafe impl Sync for BarcodeScannerStopSoftwareTriggerRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerStopSoftwareTriggerRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerStopSoftwareTriggerRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<BarcodeScannerStopSoftwareTriggerRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerStopSoftwareTriggerRequestEventArgs>();
}
unsafe impl windows_core::Interface for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerStopSoftwareTriggerRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequestEventArgs";
}
unsafe impl Send for BarcodeScannerStopSoftwareTriggerRequestEventArgs {}
unsafe impl Sync for BarcodeScannerStopSoftwareTriggerRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeScannerVideoFrame(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerVideoFrame, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BarcodeScannerVideoFrame, super::super::super::Foundation::IClosable);
impl BarcodeScannerVideoFrame {
    #[cfg(feature = "Graphics_Imaging")]
    pub fn Format(&self) -> windows_core::Result<super::super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Format)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Height(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn PixelData(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for BarcodeScannerVideoFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerVideoFrame>();
}
unsafe impl windows_core::Interface for BarcodeScannerVideoFrame {
    type Vtable = IBarcodeScannerVideoFrame_Vtbl;
    const IID: windows_core::GUID = <IBarcodeScannerVideoFrame as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerVideoFrame {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerVideoFrame";
}
unsafe impl Send for BarcodeScannerVideoFrame {}
unsafe impl Sync for BarcodeScannerVideoFrame {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarcodeSymbologyAttributesBuilder(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeSymbologyAttributesBuilder, windows_core::IUnknown, windows_core::IInspectable);
impl BarcodeSymbologyAttributesBuilder {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BarcodeSymbologyAttributesBuilder, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsCheckDigitValidationSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCheckDigitValidationSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCheckDigitValidationSupported(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsCheckDigitValidationSupported)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCheckDigitTransmissionSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCheckDigitTransmissionSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCheckDigitTransmissionSupported(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsCheckDigitTransmissionSupported)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecodeLengthSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDecodeLengthSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDecodeLengthSupported(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsDecodeLengthSupported)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateAttributes(&self) -> windows_core::Result<super::BarcodeSymbologyAttributes> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAttributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeSymbologyAttributesBuilder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeSymbologyAttributesBuilder>();
}
unsafe impl windows_core::Interface for BarcodeSymbologyAttributesBuilder {
    type Vtable = IBarcodeSymbologyAttributesBuilder_Vtbl;
    const IID: windows_core::GUID = <IBarcodeSymbologyAttributesBuilder as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeSymbologyAttributesBuilder {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeSymbologyAttributesBuilder";
}
unsafe impl Send for BarcodeSymbologyAttributesBuilder {}
unsafe impl Sync for BarcodeSymbologyAttributesBuilder {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BarcodeScannerTriggerState(pub i32);
impl BarcodeScannerTriggerState {
    pub const Released: Self = Self(0i32);
    pub const Pressed: Self = Self(1i32);
}
impl windows_core::TypeKind for BarcodeScannerTriggerState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BarcodeScannerTriggerState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BarcodeScannerTriggerState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BarcodeScannerTriggerState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.Provider.BarcodeScannerTriggerState;i4)");
}
