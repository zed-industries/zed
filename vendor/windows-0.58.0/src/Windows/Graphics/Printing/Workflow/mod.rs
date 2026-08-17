windows_core::imp::define_interface!(IPrintWorkflowBackgroundSession, IPrintWorkflowBackgroundSession_Vtbl, 0x5b7913ba_0c5e_528a_7458_86a46cbddc45);
impl windows_core::RuntimeType for IPrintWorkflowBackgroundSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowBackgroundSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetupRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSetupRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Submitted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSubmitted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintWorkflowSessionStatus) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowBackgroundSetupRequestedEventArgs, IPrintWorkflowBackgroundSetupRequestedEventArgs_Vtbl, 0x43e97342_1750_59c9_61fb_383748a20362);
impl windows_core::RuntimeType for IPrintWorkflowBackgroundSetupRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowBackgroundSetupRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub GetUserPrintTicketAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    GetUserPrintTicketAsync: usize,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRequiresUI: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowConfiguration, IPrintWorkflowConfiguration_Vtbl, 0xd0aac4ed_fd4b_5df5_4bb6_8d0d159ebe3f);
impl windows_core::RuntimeType for IPrintWorkflowConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SourceAppDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub JobTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowConfiguration2, IPrintWorkflowConfiguration2_Vtbl, 0xde350a50_a6d4_5be2_8b9a_09d3d39ea780);
impl windows_core::RuntimeType for IPrintWorkflowConfiguration2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowConfiguration2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AbortPrintFlow: unsafe extern "system" fn(*mut core::ffi::c_void, PrintWorkflowJobAbortReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowForegroundSession, IPrintWorkflowForegroundSession_Vtbl, 0xc79b63d0_f8ec_4ceb_953a_c8876157dd33);
impl windows_core::RuntimeType for IPrintWorkflowForegroundSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowForegroundSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetupRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSetupRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub XpsDataAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveXpsDataAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintWorkflowSessionStatus) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowForegroundSetupRequestedEventArgs, IPrintWorkflowForegroundSetupRequestedEventArgs_Vtbl, 0xbbe38247_9c1b_4dd3_9b2b_c80468d941b3);
impl windows_core::RuntimeType for IPrintWorkflowForegroundSetupRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowForegroundSetupRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub GetUserPrintTicketAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    GetUserPrintTicketAsync: usize,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowJobActivatedEventArgs, IPrintWorkflowJobActivatedEventArgs_Vtbl, 0xd4bd5e6d_034e_5e00_a616_f961a033dcc8);
impl windows_core::RuntimeType for IPrintWorkflowJobActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowJobActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowJobBackgroundSession, IPrintWorkflowJobBackgroundSession_Vtbl, 0xc5ec6ad8_20c9_5d51_8507_2734b46f96c5);
impl windows_core::RuntimeType for IPrintWorkflowJobBackgroundSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowJobBackgroundSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintWorkflowSessionStatus) -> windows_core::HRESULT,
    pub JobStarting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveJobStarting: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PdlModificationRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePdlModificationRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowJobBackgroundSession2, IPrintWorkflowJobBackgroundSession2_Vtbl, 0x592aadaf_ef26_5a55_ad21_5f63ffcf8366);
impl windows_core::RuntimeType for IPrintWorkflowJobBackgroundSession2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowJobBackgroundSession2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub JobIssueDetected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveJobIssueDetected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowJobIssueDetectedEventArgs, IPrintWorkflowJobIssueDetectedEventArgs_Vtbl, 0xde58a46e_e41e_550a_a9fb_4b1f93fb9d98);
impl windows_core::RuntimeType for IPrintWorkflowJobIssueDetectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowJobIssueDetectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub JobIssueKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintWorkflowJobIssueKind) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub SkipSystemErrorToast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetSkipSystemErrorToast: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PrinterJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UILauncher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowJobNotificationEventArgs, IPrintWorkflowJobNotificationEventArgs_Vtbl, 0x0ae16fba_5398_5eba_b472_978650186a9a);
impl windows_core::RuntimeType for IPrintWorkflowJobNotificationEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowJobNotificationEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrinterJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowJobStartingEventArgs, IPrintWorkflowJobStartingEventArgs_Vtbl, 0xe3d99ba8_31ad_5e09_b0d7_601b97f161ad);
impl windows_core::RuntimeType for IPrintWorkflowJobStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowJobStartingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
    pub SetSkipSystemRendering: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowJobStartingEventArgs2, IPrintWorkflowJobStartingEventArgs2_Vtbl, 0x7deded67_d3dc_5b23_8690_4ebfc0f0914a);
impl windows_core::RuntimeType for IPrintWorkflowJobStartingEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowJobStartingEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsIppCompressionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DisableIppCompressionForJob: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SkipSystemFaxUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetSkipSystemFaxUI: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowJobTriggerDetails, IPrintWorkflowJobTriggerDetails_Vtbl, 0xff296129_60e2_51db_ba8c_e2ccddb516b9);
impl windows_core::RuntimeType for IPrintWorkflowJobTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowJobTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrintWorkflowJobSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowJobUISession, IPrintWorkflowJobUISession_Vtbl, 0x00c8736b_7637_5687_a302_0f664d2aac65);
impl windows_core::RuntimeType for IPrintWorkflowJobUISession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowJobUISession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintWorkflowSessionStatus) -> windows_core::HRESULT,
    pub PdlDataAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePdlDataAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub JobNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveJobNotification: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowJobUISession2, IPrintWorkflowJobUISession2_Vtbl, 0xa8529368_9174_5c78_9fdb_894a82e92ada);
impl windows_core::RuntimeType for IPrintWorkflowJobUISession2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowJobUISession2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VirtualPrinterUIDataAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveVirtualPrinterUIDataAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowObjectModelSourceFileContent, IPrintWorkflowObjectModelSourceFileContent_Vtbl, 0xc36c8a6a_8a2a_419a_b3c3_2090e6bfab2f);
impl windows_core::RuntimeType for IPrintWorkflowObjectModelSourceFileContent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowObjectModelSourceFileContent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPrintWorkflowObjectModelSourceFileContentFactory, IPrintWorkflowObjectModelSourceFileContentFactory_Vtbl, 0x93b1b903_f013_56d6_b708_99ac2ccb12ee);
impl windows_core::RuntimeType for IPrintWorkflowObjectModelSourceFileContentFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowObjectModelSourceFileContentFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateInstance: usize,
}
windows_core::imp::define_interface!(IPrintWorkflowObjectModelTargetPackage, IPrintWorkflowObjectModelTargetPackage_Vtbl, 0x7d96bc74_9b54_4ca1_ad3a_979c3d44ddac);
impl windows_core::RuntimeType for IPrintWorkflowObjectModelTargetPackage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowObjectModelTargetPackage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPrintWorkflowPdlConverter, IPrintWorkflowPdlConverter_Vtbl, 0x40604b62_0ae4_51f1_818f_731dc0b005ab);
impl windows_core::RuntimeType for IPrintWorkflowPdlConverter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowPdlConverter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams"))]
    pub ConvertPdlAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams")))]
    ConvertPdlAsync: usize,
}
windows_core::imp::define_interface!(IPrintWorkflowPdlConverter2, IPrintWorkflowPdlConverter2_Vtbl, 0x854ceec1_7837_5b93_b7af_57a6998c2f71);
impl windows_core::RuntimeType for IPrintWorkflowPdlConverter2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowPdlConverter2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams"))]
    pub ConvertPdlAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, PdlConversionHostBasedProcessingOperations, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams")))]
    ConvertPdlAsync: usize,
}
windows_core::imp::define_interface!(IPrintWorkflowPdlDataAvailableEventArgs, IPrintWorkflowPdlDataAvailableEventArgs_Vtbl, 0xd4ad6b50_1547_5991_a0ef_e2ee20211518);
impl windows_core::RuntimeType for IPrintWorkflowPdlDataAvailableEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowPdlDataAvailableEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrinterJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SourceContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowPdlModificationRequestedEventArgs, IPrintWorkflowPdlModificationRequestedEventArgs_Vtbl, 0x1a339a61_2e13_5edd_a707_ceec61d7333b);
impl windows_core::RuntimeType for IPrintWorkflowPdlModificationRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowPdlModificationRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrinterJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SourceContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UILauncher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateJobOnPrinter: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub CreateJobOnPrinterWithAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Foundation_Collections")))]
    CreateJobOnPrinterWithAttributes: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateJobOnPrinterWithAttributesBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateJobOnPrinterWithAttributesBuffer: usize,
    pub GetPdlConverter: unsafe extern "system" fn(*mut core::ffi::c_void, PrintWorkflowPdlConversionType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowPdlModificationRequestedEventArgs2, IPrintWorkflowPdlModificationRequestedEventArgs2_Vtbl, 0x8d692147_6c62_5e31_a0e7_d49f92c111c0);
impl windows_core::RuntimeType for IPrintWorkflowPdlModificationRequestedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowPdlModificationRequestedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub CreateJobOnPrinterWithAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, PrintWorkflowAttributesMergePolicy, PrintWorkflowAttributesMergePolicy, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Foundation_Collections")))]
    CreateJobOnPrinterWithAttributes: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateJobOnPrinterWithAttributesBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, PrintWorkflowAttributesMergePolicy, PrintWorkflowAttributesMergePolicy, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateJobOnPrinterWithAttributesBuffer: usize,
}
windows_core::imp::define_interface!(IPrintWorkflowPdlSourceContent, IPrintWorkflowPdlSourceContent_Vtbl, 0x92f7fc41_32b8_56ab_845e_b1e68b3aedd5);
impl windows_core::RuntimeType for IPrintWorkflowPdlSourceContent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowPdlSourceContent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetInputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetInputStream: usize,
    #[cfg(feature = "Storage")]
    pub GetContentFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    GetContentFileAsync: usize,
}
windows_core::imp::define_interface!(IPrintWorkflowPdlTargetStream, IPrintWorkflowPdlTargetStream_Vtbl, 0xa742dfe5_1ee3_52a9_9f9f_2e2043180fd1);
impl windows_core::RuntimeType for IPrintWorkflowPdlTargetStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowPdlTargetStream_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetOutputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetOutputStream: usize,
    pub CompleteStreamSubmission: unsafe extern "system" fn(*mut core::ffi::c_void, PrintWorkflowSubmittedStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowPrinterJob, IPrintWorkflowPrinterJob_Vtbl, 0x12009f94_0d14_5443_bc09_250311ce570b);
impl windows_core::RuntimeType for IPrintWorkflowPrinterJob {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowPrinterJob_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub JobId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
    pub GetJobStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintWorkflowPrinterJobStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub GetJobPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    GetJobPrintTicket: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetJobAttributesAsBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetJobAttributesAsBuffer: usize,
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub GetJobAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Foundation_Collections")))]
    GetJobAttributes: usize,
    #[cfg(all(feature = "Devices_Printers", feature = "Storage_Streams"))]
    pub SetJobAttributesFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Storage_Streams")))]
    SetJobAttributesFromBuffer: usize,
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub SetJobAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Foundation_Collections")))]
    SetJobAttributes: usize,
}
windows_core::imp::define_interface!(IPrintWorkflowPrinterJob2, IPrintWorkflowPrinterJob2_Vtbl, 0x747e21d7_69a9_5229_b8f0_874ca1a8871b);
impl windows_core::RuntimeType for IPrintWorkflowPrinterJob2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowPrinterJob2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections", feature = "Graphics_Printing_PrintTicket"))]
    pub ConvertPrintTicketToJobAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Foundation_Collections", feature = "Graphics_Printing_PrintTicket")))]
    ConvertPrintTicketToJobAttributes: usize,
}
windows_core::imp::define_interface!(IPrintWorkflowSourceContent, IPrintWorkflowSourceContent_Vtbl, 0x1a28c641_ceb1_4533_bb73_fbe63eefdb18);
impl windows_core::RuntimeType for IPrintWorkflowSourceContent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowSourceContent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub GetJobPrintTicketAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    GetJobPrintTicketAsync: usize,
    pub GetSourceSpoolDataAsStreamContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSourceSpoolDataAsXpsObjectModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowSpoolStreamContent, IPrintWorkflowSpoolStreamContent_Vtbl, 0x72e55ece_e406_4b74_84e1_3ff3fdcdaf70);
impl windows_core::RuntimeType for IPrintWorkflowSpoolStreamContent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowSpoolStreamContent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetInputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetInputStream: usize,
}
windows_core::imp::define_interface!(IPrintWorkflowStreamTarget, IPrintWorkflowStreamTarget_Vtbl, 0xb23bba84_8565_488b_9839_1c9e7c7aa916);
impl windows_core::RuntimeType for IPrintWorkflowStreamTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowStreamTarget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetOutputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetOutputStream: usize,
}
windows_core::imp::define_interface!(IPrintWorkflowSubmittedEventArgs, IPrintWorkflowSubmittedEventArgs_Vtbl, 0x3add0a41_3794_5569_5c87_40e8ff720f83);
impl windows_core::RuntimeType for IPrintWorkflowSubmittedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowSubmittedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Operation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub GetTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    GetTarget: usize,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowSubmittedOperation, IPrintWorkflowSubmittedOperation_Vtbl, 0x2e4e6216_3be1_5f0f_5c81_a5a2bd4eab0e);
impl windows_core::RuntimeType for IPrintWorkflowSubmittedOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowSubmittedOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void, PrintWorkflowSubmittedStatus) -> windows_core::HRESULT,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub XpsContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowTarget, IPrintWorkflowTarget_Vtbl, 0x29da276c_0a73_5aed_4f3d_970d3251f057);
impl windows_core::RuntimeType for IPrintWorkflowTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowTarget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TargetAsStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TargetAsXpsObjectModelPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowTriggerDetails, IPrintWorkflowTriggerDetails_Vtbl, 0x5739d868_9d86_4052_b0cb_f310becd59bb);
impl windows_core::RuntimeType for IPrintWorkflowTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrintWorkflowSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowUIActivatedEventArgs, IPrintWorkflowUIActivatedEventArgs_Vtbl, 0xbc8a844d_09eb_5746_72a6_8dc8b5edbe9b);
impl windows_core::RuntimeType for IPrintWorkflowUIActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowUIActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrintWorkflowSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowUILauncher, IPrintWorkflowUILauncher_Vtbl, 0x64e9e22f_14cc_5828_96fb_39163fb6c378);
impl windows_core::RuntimeType for IPrintWorkflowUILauncher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowUILauncher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsUILaunchEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub LaunchAndCompleteUIAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowVirtualPrinterDataAvailableEventArgs, IPrintWorkflowVirtualPrinterDataAvailableEventArgs_Vtbl, 0x6b7d5003_14a8_5d52_a428_07330fbab11f);
impl windows_core::RuntimeType for IPrintWorkflowVirtualPrinterDataAvailableEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowVirtualPrinterDataAvailableEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SourceContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UILauncher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub GetJobPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    GetJobPrintTicket: usize,
    pub GetPdlConverter: unsafe extern "system" fn(*mut core::ffi::c_void, PrintWorkflowPdlConversionType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub GetTargetFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    GetTargetFileAsync: usize,
    pub CompleteJob: unsafe extern "system" fn(*mut core::ffi::c_void, PrintWorkflowSubmittedStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowVirtualPrinterSession, IPrintWorkflowVirtualPrinterSession_Vtbl, 0xaa3926f2_8485_5c27_a016_9d39e3ba2614);
impl windows_core::RuntimeType for IPrintWorkflowVirtualPrinterSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowVirtualPrinterSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintWorkflowSessionStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
    pub VirtualPrinterDataAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveVirtualPrinterDataAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowVirtualPrinterTriggerDetails, IPrintWorkflowVirtualPrinterTriggerDetails_Vtbl, 0xff8f2297_727b_53ec_b9e0_f393f72d4e50);
impl windows_core::RuntimeType for IPrintWorkflowVirtualPrinterTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowVirtualPrinterTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VirtualPrinterSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowVirtualPrinterUIEventArgs, IPrintWorkflowVirtualPrinterUIEventArgs_Vtbl, 0x334dbbca_bf10_585f_b7e0_58c4aa43a03f);
impl windows_core::RuntimeType for IPrintWorkflowVirtualPrinterUIEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowVirtualPrinterUIEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
    pub SourceContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub GetJobPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    GetJobPrintTicket: usize,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowXpsDataAvailableEventArgs, IPrintWorkflowXpsDataAvailableEventArgs_Vtbl, 0x4d11c331_54d1_434e_be0e_82c5fa58e5b2);
impl windows_core::RuntimeType for IPrintWorkflowXpsDataAvailableEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintWorkflowXpsDataAvailableEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Operation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowBackgroundSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowBackgroundSession, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowBackgroundSession {
    pub fn SetupRequested<P0>(&self, setupeventhandler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowBackgroundSetupRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetupRequested)(windows_core::Interface::as_raw(this), setupeventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSetupRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSetupRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Submitted<P0>(&self, submittedeventhandler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowSubmittedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Submitted)(windows_core::Interface::as_raw(this), submittedeventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSubmitted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSubmitted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Status(&self) -> windows_core::Result<PrintWorkflowSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for PrintWorkflowBackgroundSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowBackgroundSession>();
}
unsafe impl windows_core::Interface for PrintWorkflowBackgroundSession {
    type Vtable = IPrintWorkflowBackgroundSession_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowBackgroundSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowBackgroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowBackgroundSession";
}
unsafe impl Send for PrintWorkflowBackgroundSession {}
unsafe impl Sync for PrintWorkflowBackgroundSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowBackgroundSetupRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowBackgroundSetupRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowBackgroundSetupRequestedEventArgs {
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn GetUserPrintTicketAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUserPrintTicketAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Configuration(&self) -> windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRequiresUI(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRequiresUI)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintWorkflowBackgroundSetupRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowBackgroundSetupRequestedEventArgs>();
}
unsafe impl windows_core::Interface for PrintWorkflowBackgroundSetupRequestedEventArgs {
    type Vtable = IPrintWorkflowBackgroundSetupRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowBackgroundSetupRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowBackgroundSetupRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowBackgroundSetupRequestedEventArgs";
}
unsafe impl Send for PrintWorkflowBackgroundSetupRequestedEventArgs {}
unsafe impl Sync for PrintWorkflowBackgroundSetupRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowConfiguration {
    pub fn SourceAppDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceAppDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn JobTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JobTitle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SessionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AbortPrintFlow(&self, reason: PrintWorkflowJobAbortReason) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintWorkflowConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AbortPrintFlow)(windows_core::Interface::as_raw(this), reason).ok() }
    }
}
impl windows_core::RuntimeType for PrintWorkflowConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowConfiguration>();
}
unsafe impl windows_core::Interface for PrintWorkflowConfiguration {
    type Vtable = IPrintWorkflowConfiguration_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowConfiguration {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowConfiguration";
}
unsafe impl Send for PrintWorkflowConfiguration {}
unsafe impl Sync for PrintWorkflowConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowForegroundSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowForegroundSession, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowForegroundSession {
    pub fn SetupRequested<P0>(&self, setupeventhandler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowForegroundSetupRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetupRequested)(windows_core::Interface::as_raw(this), setupeventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSetupRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSetupRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn XpsDataAvailable<P0>(&self, xpsdataavailableeventhandler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowXpsDataAvailableEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XpsDataAvailable)(windows_core::Interface::as_raw(this), xpsdataavailableeventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveXpsDataAvailable(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveXpsDataAvailable)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Status(&self) -> windows_core::Result<PrintWorkflowSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for PrintWorkflowForegroundSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowForegroundSession>();
}
unsafe impl windows_core::Interface for PrintWorkflowForegroundSession {
    type Vtable = IPrintWorkflowForegroundSession_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowForegroundSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowForegroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowForegroundSession";
}
unsafe impl Send for PrintWorkflowForegroundSession {}
unsafe impl Sync for PrintWorkflowForegroundSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowForegroundSetupRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowForegroundSetupRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowForegroundSetupRequestedEventArgs {
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn GetUserPrintTicketAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUserPrintTicketAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Configuration(&self) -> windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for PrintWorkflowForegroundSetupRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowForegroundSetupRequestedEventArgs>();
}
unsafe impl windows_core::Interface for PrintWorkflowForegroundSetupRequestedEventArgs {
    type Vtable = IPrintWorkflowForegroundSetupRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowForegroundSetupRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowForegroundSetupRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowForegroundSetupRequestedEventArgs";
}
unsafe impl Send for PrintWorkflowForegroundSetupRequestedEventArgs {}
unsafe impl Sync for PrintWorkflowForegroundSetupRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowJobActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowJobActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(PrintWorkflowJobActivatedEventArgs, super::super::super::ApplicationModel::Activation::IActivatedEventArgs, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser);
impl PrintWorkflowJobActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> windows_core::Result<super::super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> windows_core::Result<super::super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Session(&self) -> windows_core::Result<PrintWorkflowJobUISession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Session)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintWorkflowJobActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowJobActivatedEventArgs>();
}
unsafe impl windows_core::Interface for PrintWorkflowJobActivatedEventArgs {
    type Vtable = IPrintWorkflowJobActivatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowJobActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowJobActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobActivatedEventArgs";
}
unsafe impl Send for PrintWorkflowJobActivatedEventArgs {}
unsafe impl Sync for PrintWorkflowJobActivatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowJobBackgroundSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowJobBackgroundSession, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowJobBackgroundSession {
    pub fn Status(&self) -> windows_core::Result<PrintWorkflowSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn JobStarting<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowJobStartingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JobStarting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveJobStarting(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveJobStarting)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PdlModificationRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowPdlModificationRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PdlModificationRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePdlModificationRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePdlModificationRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn JobIssueDetected<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowJobIssueDetectedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IPrintWorkflowJobBackgroundSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JobIssueDetected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveJobIssueDetected(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintWorkflowJobBackgroundSession2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveJobIssueDetected)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for PrintWorkflowJobBackgroundSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowJobBackgroundSession>();
}
unsafe impl windows_core::Interface for PrintWorkflowJobBackgroundSession {
    type Vtable = IPrintWorkflowJobBackgroundSession_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowJobBackgroundSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowJobBackgroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobBackgroundSession";
}
unsafe impl Send for PrintWorkflowJobBackgroundSession {}
unsafe impl Sync for PrintWorkflowJobBackgroundSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowJobIssueDetectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowJobIssueDetectedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowJobIssueDetectedEventArgs {
    pub fn JobIssueKind(&self) -> windows_core::Result<PrintWorkflowJobIssueKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JobIssueKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SkipSystemErrorToast(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SkipSystemErrorToast)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSkipSystemErrorToast(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSkipSystemErrorToast)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PrinterJob(&self) -> windows_core::Result<PrintWorkflowPrinterJob> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrinterJob)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Configuration(&self) -> windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UILauncher(&self) -> windows_core::Result<PrintWorkflowUILauncher> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UILauncher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for PrintWorkflowJobIssueDetectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowJobIssueDetectedEventArgs>();
}
unsafe impl windows_core::Interface for PrintWorkflowJobIssueDetectedEventArgs {
    type Vtable = IPrintWorkflowJobIssueDetectedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowJobIssueDetectedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowJobIssueDetectedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobIssueDetectedEventArgs";
}
unsafe impl Send for PrintWorkflowJobIssueDetectedEventArgs {}
unsafe impl Sync for PrintWorkflowJobIssueDetectedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowJobNotificationEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowJobNotificationEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowJobNotificationEventArgs {
    pub fn Configuration(&self) -> windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PrinterJob(&self) -> windows_core::Result<PrintWorkflowPrinterJob> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrinterJob)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for PrintWorkflowJobNotificationEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowJobNotificationEventArgs>();
}
unsafe impl windows_core::Interface for PrintWorkflowJobNotificationEventArgs {
    type Vtable = IPrintWorkflowJobNotificationEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowJobNotificationEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowJobNotificationEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobNotificationEventArgs";
}
unsafe impl Send for PrintWorkflowJobNotificationEventArgs {}
unsafe impl Sync for PrintWorkflowJobNotificationEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowJobStartingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowJobStartingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowJobStartingEventArgs {
    pub fn Configuration(&self) -> windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Printers")]
    pub fn Printer(&self) -> windows_core::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Printer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSkipSystemRendering(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSkipSystemRendering)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsIppCompressionEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPrintWorkflowJobStartingEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsIppCompressionEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisableIppCompressionForJob(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintWorkflowJobStartingEventArgs2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DisableIppCompressionForJob)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SkipSystemFaxUI(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPrintWorkflowJobStartingEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SkipSystemFaxUI)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSkipSystemFaxUI(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintWorkflowJobStartingEventArgs2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSkipSystemFaxUI)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for PrintWorkflowJobStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowJobStartingEventArgs>();
}
unsafe impl windows_core::Interface for PrintWorkflowJobStartingEventArgs {
    type Vtable = IPrintWorkflowJobStartingEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowJobStartingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowJobStartingEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobStartingEventArgs";
}
unsafe impl Send for PrintWorkflowJobStartingEventArgs {}
unsafe impl Sync for PrintWorkflowJobStartingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowJobTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowJobTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowJobTriggerDetails {
    pub fn PrintWorkflowJobSession(&self) -> windows_core::Result<PrintWorkflowJobBackgroundSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintWorkflowJobSession)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintWorkflowJobTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowJobTriggerDetails>();
}
unsafe impl windows_core::Interface for PrintWorkflowJobTriggerDetails {
    type Vtable = IPrintWorkflowJobTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowJobTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowJobTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobTriggerDetails";
}
unsafe impl Send for PrintWorkflowJobTriggerDetails {}
unsafe impl Sync for PrintWorkflowJobTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowJobUISession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowJobUISession, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowJobUISession {
    pub fn Status(&self) -> windows_core::Result<PrintWorkflowSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PdlDataAvailable<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowPdlDataAvailableEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PdlDataAvailable)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePdlDataAvailable(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePdlDataAvailable)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn JobNotification<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowJobNotificationEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JobNotification)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveJobNotification(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveJobNotification)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn VirtualPrinterUIDataAvailable<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowVirtualPrinterUIEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IPrintWorkflowJobUISession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VirtualPrinterUIDataAvailable)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVirtualPrinterUIDataAvailable(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintWorkflowJobUISession2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveVirtualPrinterUIDataAvailable)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for PrintWorkflowJobUISession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowJobUISession>();
}
unsafe impl windows_core::Interface for PrintWorkflowJobUISession {
    type Vtable = IPrintWorkflowJobUISession_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowJobUISession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowJobUISession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobUISession";
}
unsafe impl Send for PrintWorkflowJobUISession {}
unsafe impl Sync for PrintWorkflowJobUISession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowObjectModelSourceFileContent(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowObjectModelSourceFileContent, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowObjectModelSourceFileContent {
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateInstance<P0>(xpsstream: P0) -> windows_core::Result<PrintWorkflowObjectModelSourceFileContent>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IInputStream>,
    {
        Self::IPrintWorkflowObjectModelSourceFileContentFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), xpsstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPrintWorkflowObjectModelSourceFileContentFactory<R, F: FnOnce(&IPrintWorkflowObjectModelSourceFileContentFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PrintWorkflowObjectModelSourceFileContent, IPrintWorkflowObjectModelSourceFileContentFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PrintWorkflowObjectModelSourceFileContent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowObjectModelSourceFileContent>();
}
unsafe impl windows_core::Interface for PrintWorkflowObjectModelSourceFileContent {
    type Vtable = IPrintWorkflowObjectModelSourceFileContent_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowObjectModelSourceFileContent as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowObjectModelSourceFileContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowObjectModelSourceFileContent";
}
unsafe impl Send for PrintWorkflowObjectModelSourceFileContent {}
unsafe impl Sync for PrintWorkflowObjectModelSourceFileContent {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowObjectModelTargetPackage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowObjectModelTargetPackage, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowObjectModelTargetPackage {}
impl windows_core::RuntimeType for PrintWorkflowObjectModelTargetPackage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowObjectModelTargetPackage>();
}
unsafe impl windows_core::Interface for PrintWorkflowObjectModelTargetPackage {
    type Vtable = IPrintWorkflowObjectModelTargetPackage_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowObjectModelTargetPackage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowObjectModelTargetPackage {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowObjectModelTargetPackage";
}
unsafe impl Send for PrintWorkflowObjectModelTargetPackage {}
unsafe impl Sync for PrintWorkflowObjectModelTargetPackage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowPdlConverter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowPdlConverter, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowPdlConverter {
    #[cfg(all(feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams"))]
    pub fn ConvertPdlAsync<P0, P1, P2>(&self, printticket: P0, inputstream: P1, outputstream: P2) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::PrintTicket::WorkflowPrintTicket>,
        P1: windows_core::Param<super::super::super::Storage::Streams::IInputStream>,
        P2: windows_core::Param<super::super::super::Storage::Streams::IOutputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConvertPdlAsync)(windows_core::Interface::as_raw(this), printticket.param().abi(), inputstream.param().abi(), outputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams"))]
    pub fn ConvertPdlAsync2<P0, P1, P2>(&self, printticket: P0, inputstream: P1, outputstream: P2, hostbasedprocessingoperations: PdlConversionHostBasedProcessingOperations) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::PrintTicket::WorkflowPrintTicket>,
        P1: windows_core::Param<super::super::super::Storage::Streams::IInputStream>,
        P2: windows_core::Param<super::super::super::Storage::Streams::IOutputStream>,
    {
        let this = &windows_core::Interface::cast::<IPrintWorkflowPdlConverter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConvertPdlAsync)(windows_core::Interface::as_raw(this), printticket.param().abi(), inputstream.param().abi(), outputstream.param().abi(), hostbasedprocessingoperations, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintWorkflowPdlConverter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowPdlConverter>();
}
unsafe impl windows_core::Interface for PrintWorkflowPdlConverter {
    type Vtable = IPrintWorkflowPdlConverter_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowPdlConverter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowPdlConverter {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlConverter";
}
unsafe impl Send for PrintWorkflowPdlConverter {}
unsafe impl Sync for PrintWorkflowPdlConverter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowPdlDataAvailableEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowPdlDataAvailableEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowPdlDataAvailableEventArgs {
    pub fn Configuration(&self) -> windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PrinterJob(&self) -> windows_core::Result<PrintWorkflowPrinterJob> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrinterJob)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SourceContent(&self) -> windows_core::Result<PrintWorkflowPdlSourceContent> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceContent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for PrintWorkflowPdlDataAvailableEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowPdlDataAvailableEventArgs>();
}
unsafe impl windows_core::Interface for PrintWorkflowPdlDataAvailableEventArgs {
    type Vtable = IPrintWorkflowPdlDataAvailableEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowPdlDataAvailableEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowPdlDataAvailableEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlDataAvailableEventArgs";
}
unsafe impl Send for PrintWorkflowPdlDataAvailableEventArgs {}
unsafe impl Sync for PrintWorkflowPdlDataAvailableEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowPdlModificationRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowPdlModificationRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowPdlModificationRequestedEventArgs {
    pub fn Configuration(&self) -> windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PrinterJob(&self) -> windows_core::Result<PrintWorkflowPrinterJob> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrinterJob)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SourceContent(&self) -> windows_core::Result<PrintWorkflowPdlSourceContent> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceContent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UILauncher(&self) -> windows_core::Result<PrintWorkflowUILauncher> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UILauncher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateJobOnPrinter(&self, targetcontenttype: &windows_core::HSTRING) -> windows_core::Result<PrintWorkflowPdlTargetStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateJobOnPrinter)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(targetcontenttype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub fn CreateJobOnPrinterWithAttributes<P0>(&self, jobattributes: P0, targetcontenttype: &windows_core::HSTRING) -> windows_core::Result<PrintWorkflowPdlTargetStream>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateJobOnPrinterWithAttributes)(windows_core::Interface::as_raw(this), jobattributes.param().abi(), core::mem::transmute_copy(targetcontenttype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateJobOnPrinterWithAttributesBuffer<P0>(&self, jobattributesbuffer: P0, targetcontenttype: &windows_core::HSTRING) -> windows_core::Result<PrintWorkflowPdlTargetStream>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateJobOnPrinterWithAttributesBuffer)(windows_core::Interface::as_raw(this), jobattributesbuffer.param().abi(), core::mem::transmute_copy(targetcontenttype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPdlConverter(&self, conversiontype: PrintWorkflowPdlConversionType) -> windows_core::Result<PrintWorkflowPdlConverter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPdlConverter)(windows_core::Interface::as_raw(this), conversiontype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub fn CreateJobOnPrinterWithAttributes2<P0, P1>(&self, jobattributes: P0, targetcontenttype: &windows_core::HSTRING, operationattributes: P1, jobattributesmergepolicy: PrintWorkflowAttributesMergePolicy, operationattributesmergepolicy: PrintWorkflowAttributesMergePolicy) -> windows_core::Result<PrintWorkflowPdlTargetStream>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>>,
        P1: windows_core::Param<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>>,
    {
        let this = &windows_core::Interface::cast::<IPrintWorkflowPdlModificationRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateJobOnPrinterWithAttributes)(windows_core::Interface::as_raw(this), jobattributes.param().abi(), core::mem::transmute_copy(targetcontenttype), operationattributes.param().abi(), jobattributesmergepolicy, operationattributesmergepolicy, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateJobOnPrinterWithAttributesBuffer2<P0, P1>(&self, jobattributesbuffer: P0, targetcontenttype: &windows_core::HSTRING, operationattributesbuffer: P1, jobattributesmergepolicy: PrintWorkflowAttributesMergePolicy, operationattributesmergepolicy: PrintWorkflowAttributesMergePolicy) -> windows_core::Result<PrintWorkflowPdlTargetStream>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
        P1: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IPrintWorkflowPdlModificationRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateJobOnPrinterWithAttributesBuffer)(windows_core::Interface::as_raw(this), jobattributesbuffer.param().abi(), core::mem::transmute_copy(targetcontenttype), operationattributesbuffer.param().abi(), jobattributesmergepolicy, operationattributesmergepolicy, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintWorkflowPdlModificationRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowPdlModificationRequestedEventArgs>();
}
unsafe impl windows_core::Interface for PrintWorkflowPdlModificationRequestedEventArgs {
    type Vtable = IPrintWorkflowPdlModificationRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowPdlModificationRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowPdlModificationRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlModificationRequestedEventArgs";
}
unsafe impl Send for PrintWorkflowPdlModificationRequestedEventArgs {}
unsafe impl Sync for PrintWorkflowPdlModificationRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowPdlSourceContent(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowPdlSourceContent, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowPdlSourceContent {
    pub fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStream(&self) -> windows_core::Result<super::super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn GetContentFileAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetContentFileAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintWorkflowPdlSourceContent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowPdlSourceContent>();
}
unsafe impl windows_core::Interface for PrintWorkflowPdlSourceContent {
    type Vtable = IPrintWorkflowPdlSourceContent_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowPdlSourceContent as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowPdlSourceContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlSourceContent";
}
unsafe impl Send for PrintWorkflowPdlSourceContent {}
unsafe impl Sync for PrintWorkflowPdlSourceContent {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowPdlTargetStream(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowPdlTargetStream, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowPdlTargetStream {
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStream(&self) -> windows_core::Result<super::super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOutputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CompleteStreamSubmission(&self, status: PrintWorkflowSubmittedStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CompleteStreamSubmission)(windows_core::Interface::as_raw(this), status).ok() }
    }
}
impl windows_core::RuntimeType for PrintWorkflowPdlTargetStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowPdlTargetStream>();
}
unsafe impl windows_core::Interface for PrintWorkflowPdlTargetStream {
    type Vtable = IPrintWorkflowPdlTargetStream_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowPdlTargetStream as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowPdlTargetStream {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlTargetStream";
}
unsafe impl Send for PrintWorkflowPdlTargetStream {}
unsafe impl Sync for PrintWorkflowPdlTargetStream {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowPrinterJob(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowPrinterJob, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowPrinterJob {
    pub fn JobId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JobId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Printers")]
    pub fn Printer(&self) -> windows_core::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Printer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetJobStatus(&self) -> windows_core::Result<PrintWorkflowPrinterJobStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetJobStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn GetJobPrintTicket(&self) -> windows_core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetJobPrintTicket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetJobAttributesAsBuffer<P0>(&self, attributenames: P0) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetJobAttributesAsBuffer)(windows_core::Interface::as_raw(this), attributenames.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub fn GetJobAttributes<P0>(&self, attributenames: P0) -> windows_core::Result<super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetJobAttributes)(windows_core::Interface::as_raw(this), attributenames.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Devices_Printers", feature = "Storage_Streams"))]
    pub fn SetJobAttributesFromBuffer<P0>(&self, jobattributesbuffer: P0) -> windows_core::Result<super::super::super::Devices::Printers::IppSetAttributesResult>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetJobAttributesFromBuffer)(windows_core::Interface::as_raw(this), jobattributesbuffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub fn SetJobAttributes<P0>(&self, jobattributes: P0) -> windows_core::Result<super::super::super::Devices::Printers::IppSetAttributesResult>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetJobAttributes)(windows_core::Interface::as_raw(this), jobattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections", feature = "Graphics_Printing_PrintTicket"))]
    pub fn ConvertPrintTicketToJobAttributes<P0>(&self, printticket: P0, targetpdlformat: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>
    where
        P0: windows_core::Param<super::PrintTicket::WorkflowPrintTicket>,
    {
        let this = &windows_core::Interface::cast::<IPrintWorkflowPrinterJob2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConvertPrintTicketToJobAttributes)(windows_core::Interface::as_raw(this), printticket.param().abi(), core::mem::transmute_copy(targetpdlformat), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintWorkflowPrinterJob {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowPrinterJob>();
}
unsafe impl windows_core::Interface for PrintWorkflowPrinterJob {
    type Vtable = IPrintWorkflowPrinterJob_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowPrinterJob as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowPrinterJob {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPrinterJob";
}
unsafe impl Send for PrintWorkflowPrinterJob {}
unsafe impl Sync for PrintWorkflowPrinterJob {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowSourceContent(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowSourceContent, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowSourceContent {
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn GetJobPrintTicketAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetJobPrintTicketAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSourceSpoolDataAsStreamContent(&self) -> windows_core::Result<PrintWorkflowSpoolStreamContent> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSourceSpoolDataAsStreamContent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSourceSpoolDataAsXpsObjectModel(&self) -> windows_core::Result<PrintWorkflowObjectModelSourceFileContent> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSourceSpoolDataAsXpsObjectModel)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintWorkflowSourceContent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowSourceContent>();
}
unsafe impl windows_core::Interface for PrintWorkflowSourceContent {
    type Vtable = IPrintWorkflowSourceContent_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowSourceContent as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowSourceContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowSourceContent";
}
unsafe impl Send for PrintWorkflowSourceContent {}
unsafe impl Sync for PrintWorkflowSourceContent {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowSpoolStreamContent(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowSpoolStreamContent, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowSpoolStreamContent {
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStream(&self) -> windows_core::Result<super::super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintWorkflowSpoolStreamContent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowSpoolStreamContent>();
}
unsafe impl windows_core::Interface for PrintWorkflowSpoolStreamContent {
    type Vtable = IPrintWorkflowSpoolStreamContent_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowSpoolStreamContent as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowSpoolStreamContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowSpoolStreamContent";
}
unsafe impl Send for PrintWorkflowSpoolStreamContent {}
unsafe impl Sync for PrintWorkflowSpoolStreamContent {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowStreamTarget(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowStreamTarget, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowStreamTarget {
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStream(&self) -> windows_core::Result<super::super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOutputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintWorkflowStreamTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowStreamTarget>();
}
unsafe impl windows_core::Interface for PrintWorkflowStreamTarget {
    type Vtable = IPrintWorkflowStreamTarget_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowStreamTarget as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowStreamTarget {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowStreamTarget";
}
unsafe impl Send for PrintWorkflowStreamTarget {}
unsafe impl Sync for PrintWorkflowStreamTarget {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowSubmittedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowSubmittedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowSubmittedEventArgs {
    pub fn Operation(&self) -> windows_core::Result<PrintWorkflowSubmittedOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Operation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn GetTarget<P0>(&self, jobprintticket: P0) -> windows_core::Result<PrintWorkflowTarget>
    where
        P0: windows_core::Param<super::PrintTicket::WorkflowPrintTicket>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTarget)(windows_core::Interface::as_raw(this), jobprintticket.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for PrintWorkflowSubmittedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowSubmittedEventArgs>();
}
unsafe impl windows_core::Interface for PrintWorkflowSubmittedEventArgs {
    type Vtable = IPrintWorkflowSubmittedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowSubmittedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowSubmittedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedEventArgs";
}
unsafe impl Send for PrintWorkflowSubmittedEventArgs {}
unsafe impl Sync for PrintWorkflowSubmittedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowSubmittedOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowSubmittedOperation, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowSubmittedOperation {
    pub fn Complete(&self, status: PrintWorkflowSubmittedStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this), status).ok() }
    }
    pub fn Configuration(&self) -> windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn XpsContent(&self) -> windows_core::Result<PrintWorkflowSourceContent> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XpsContent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintWorkflowSubmittedOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowSubmittedOperation>();
}
unsafe impl windows_core::Interface for PrintWorkflowSubmittedOperation {
    type Vtable = IPrintWorkflowSubmittedOperation_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowSubmittedOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowSubmittedOperation {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedOperation";
}
unsafe impl Send for PrintWorkflowSubmittedOperation {}
unsafe impl Sync for PrintWorkflowSubmittedOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowTarget(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowTarget, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowTarget {
    pub fn TargetAsStream(&self) -> windows_core::Result<PrintWorkflowStreamTarget> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TargetAsStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TargetAsXpsObjectModelPackage(&self) -> windows_core::Result<PrintWorkflowObjectModelTargetPackage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TargetAsXpsObjectModelPackage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintWorkflowTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowTarget>();
}
unsafe impl windows_core::Interface for PrintWorkflowTarget {
    type Vtable = IPrintWorkflowTarget_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowTarget as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowTarget {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowTarget";
}
unsafe impl Send for PrintWorkflowTarget {}
unsafe impl Sync for PrintWorkflowTarget {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowTriggerDetails {
    pub fn PrintWorkflowSession(&self) -> windows_core::Result<PrintWorkflowBackgroundSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintWorkflowSession)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintWorkflowTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowTriggerDetails>();
}
unsafe impl windows_core::Interface for PrintWorkflowTriggerDetails {
    type Vtable = IPrintWorkflowTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowTriggerDetails";
}
unsafe impl Send for PrintWorkflowTriggerDetails {}
unsafe impl Sync for PrintWorkflowTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowUIActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowUIActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(PrintWorkflowUIActivatedEventArgs, super::super::super::ApplicationModel::Activation::IActivatedEventArgs, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser);
impl PrintWorkflowUIActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> windows_core::Result<super::super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> windows_core::Result<super::super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PrintWorkflowSession(&self) -> windows_core::Result<PrintWorkflowForegroundSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintWorkflowSession)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintWorkflowUIActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowUIActivatedEventArgs>();
}
unsafe impl windows_core::Interface for PrintWorkflowUIActivatedEventArgs {
    type Vtable = IPrintWorkflowUIActivatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowUIActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowUIActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowUIActivatedEventArgs";
}
unsafe impl Send for PrintWorkflowUIActivatedEventArgs {}
unsafe impl Sync for PrintWorkflowUIActivatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowUILauncher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowUILauncher, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowUILauncher {
    pub fn IsUILaunchEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUILaunchEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LaunchAndCompleteUIAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<PrintWorkflowUICompletionStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchAndCompleteUIAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintWorkflowUILauncher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowUILauncher>();
}
unsafe impl windows_core::Interface for PrintWorkflowUILauncher {
    type Vtable = IPrintWorkflowUILauncher_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowUILauncher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowUILauncher {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowUILauncher";
}
unsafe impl Send for PrintWorkflowUILauncher {}
unsafe impl Sync for PrintWorkflowUILauncher {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowVirtualPrinterDataAvailableEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowVirtualPrinterDataAvailableEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowVirtualPrinterDataAvailableEventArgs {
    pub fn Configuration(&self) -> windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SourceContent(&self) -> windows_core::Result<PrintWorkflowPdlSourceContent> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceContent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UILauncher(&self) -> windows_core::Result<PrintWorkflowUILauncher> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UILauncher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn GetJobPrintTicket(&self) -> windows_core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetJobPrintTicket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPdlConverter(&self, conversiontype: PrintWorkflowPdlConversionType) -> windows_core::Result<PrintWorkflowPdlConverter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPdlConverter)(windows_core::Interface::as_raw(this), conversiontype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn GetTargetFileAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTargetFileAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CompleteJob(&self, status: PrintWorkflowSubmittedStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CompleteJob)(windows_core::Interface::as_raw(this), status).ok() }
    }
}
impl windows_core::RuntimeType for PrintWorkflowVirtualPrinterDataAvailableEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowVirtualPrinterDataAvailableEventArgs>();
}
unsafe impl windows_core::Interface for PrintWorkflowVirtualPrinterDataAvailableEventArgs {
    type Vtable = IPrintWorkflowVirtualPrinterDataAvailableEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowVirtualPrinterDataAvailableEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowVirtualPrinterDataAvailableEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowVirtualPrinterDataAvailableEventArgs";
}
unsafe impl Send for PrintWorkflowVirtualPrinterDataAvailableEventArgs {}
unsafe impl Sync for PrintWorkflowVirtualPrinterDataAvailableEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowVirtualPrinterSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowVirtualPrinterSession, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowVirtualPrinterSession {
    pub fn Status(&self) -> windows_core::Result<PrintWorkflowSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Printers")]
    pub fn Printer(&self) -> windows_core::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Printer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VirtualPrinterDataAvailable<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintWorkflowVirtualPrinterSession, PrintWorkflowVirtualPrinterDataAvailableEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VirtualPrinterDataAvailable)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVirtualPrinterDataAvailable(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveVirtualPrinterDataAvailable)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for PrintWorkflowVirtualPrinterSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowVirtualPrinterSession>();
}
unsafe impl windows_core::Interface for PrintWorkflowVirtualPrinterSession {
    type Vtable = IPrintWorkflowVirtualPrinterSession_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowVirtualPrinterSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowVirtualPrinterSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowVirtualPrinterSession";
}
unsafe impl Send for PrintWorkflowVirtualPrinterSession {}
unsafe impl Sync for PrintWorkflowVirtualPrinterSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowVirtualPrinterTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowVirtualPrinterTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowVirtualPrinterTriggerDetails {
    pub fn VirtualPrinterSession(&self) -> windows_core::Result<PrintWorkflowVirtualPrinterSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VirtualPrinterSession)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintWorkflowVirtualPrinterTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowVirtualPrinterTriggerDetails>();
}
unsafe impl windows_core::Interface for PrintWorkflowVirtualPrinterTriggerDetails {
    type Vtable = IPrintWorkflowVirtualPrinterTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowVirtualPrinterTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowVirtualPrinterTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowVirtualPrinterTriggerDetails";
}
unsafe impl Send for PrintWorkflowVirtualPrinterTriggerDetails {}
unsafe impl Sync for PrintWorkflowVirtualPrinterTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowVirtualPrinterUIEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowVirtualPrinterUIEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowVirtualPrinterUIEventArgs {
    pub fn Configuration(&self) -> windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Printers")]
    pub fn Printer(&self) -> windows_core::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Printer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SourceContent(&self) -> windows_core::Result<PrintWorkflowPdlSourceContent> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceContent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn GetJobPrintTicket(&self) -> windows_core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetJobPrintTicket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for PrintWorkflowVirtualPrinterUIEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowVirtualPrinterUIEventArgs>();
}
unsafe impl windows_core::Interface for PrintWorkflowVirtualPrinterUIEventArgs {
    type Vtable = IPrintWorkflowVirtualPrinterUIEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowVirtualPrinterUIEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowVirtualPrinterUIEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowVirtualPrinterUIEventArgs";
}
unsafe impl Send for PrintWorkflowVirtualPrinterUIEventArgs {}
unsafe impl Sync for PrintWorkflowVirtualPrinterUIEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintWorkflowXpsDataAvailableEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintWorkflowXpsDataAvailableEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintWorkflowXpsDataAvailableEventArgs {
    pub fn Operation(&self) -> windows_core::Result<PrintWorkflowSubmittedOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Operation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for PrintWorkflowXpsDataAvailableEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintWorkflowXpsDataAvailableEventArgs>();
}
unsafe impl windows_core::Interface for PrintWorkflowXpsDataAvailableEventArgs {
    type Vtable = IPrintWorkflowXpsDataAvailableEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintWorkflowXpsDataAvailableEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintWorkflowXpsDataAvailableEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowXpsDataAvailableEventArgs";
}
unsafe impl Send for PrintWorkflowXpsDataAvailableEventArgs {}
unsafe impl Sync for PrintWorkflowXpsDataAvailableEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PdlConversionHostBasedProcessingOperations(pub u32);
impl PdlConversionHostBasedProcessingOperations {
    pub const None: Self = Self(0u32);
    pub const PageRotation: Self = Self(1u32);
    pub const PageOrdering: Self = Self(2u32);
    pub const Copies: Self = Self(4u32);
    pub const BlankPageInsertion: Self = Self(8u32);
    pub const All: Self = Self(4294967295u32);
}
impl windows_core::TypeKind for PdlConversionHostBasedProcessingOperations {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PdlConversionHostBasedProcessingOperations {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PdlConversionHostBasedProcessingOperations").field(&self.0).finish()
    }
}
impl PdlConversionHostBasedProcessingOperations {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PdlConversionHostBasedProcessingOperations {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PdlConversionHostBasedProcessingOperations {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PdlConversionHostBasedProcessingOperations {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PdlConversionHostBasedProcessingOperations {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PdlConversionHostBasedProcessingOperations {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for PdlConversionHostBasedProcessingOperations {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PdlConversionHostBasedProcessingOperations;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintWorkflowAttributesMergePolicy(pub i32);
impl PrintWorkflowAttributesMergePolicy {
    pub const MergePreferPrintTicketOnConflict: Self = Self(0i32);
    pub const MergePreferPsaOnConflict: Self = Self(1i32);
    pub const DoNotMergeWithPrintTicket: Self = Self(2i32);
}
impl windows_core::TypeKind for PrintWorkflowAttributesMergePolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintWorkflowAttributesMergePolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintWorkflowAttributesMergePolicy").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintWorkflowAttributesMergePolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowAttributesMergePolicy;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintWorkflowJobAbortReason(pub i32);
impl PrintWorkflowJobAbortReason {
    pub const JobFailed: Self = Self(0i32);
    pub const UserCanceled: Self = Self(1i32);
}
impl windows_core::TypeKind for PrintWorkflowJobAbortReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintWorkflowJobAbortReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintWorkflowJobAbortReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintWorkflowJobAbortReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowJobAbortReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintWorkflowJobIssueKind(pub i32);
impl PrintWorkflowJobIssueKind {
    pub const Other: Self = Self(0i32);
    pub const AttentionRequired: Self = Self(1i32);
    pub const DoorOpen: Self = Self(2i32);
    pub const MarkerSupplyLow: Self = Self(3i32);
    pub const MarkerSupplyEmpty: Self = Self(4i32);
    pub const MediaJam: Self = Self(5i32);
    pub const MediaEmpty: Self = Self(6i32);
    pub const MediaLow: Self = Self(7i32);
    pub const OutputAreaAlmostFull: Self = Self(8i32);
    pub const OutputAreaFull: Self = Self(9i32);
    pub const JobPrintingError: Self = Self(10i32);
}
impl windows_core::TypeKind for PrintWorkflowJobIssueKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintWorkflowJobIssueKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintWorkflowJobIssueKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintWorkflowJobIssueKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowJobIssueKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintWorkflowPdlConversionType(pub i32);
impl PrintWorkflowPdlConversionType {
    pub const XpsToPdf: Self = Self(0i32);
    pub const XpsToPwgr: Self = Self(1i32);
    pub const XpsToPclm: Self = Self(2i32);
    pub const XpsToTiff: Self = Self(3i32);
}
impl windows_core::TypeKind for PrintWorkflowPdlConversionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintWorkflowPdlConversionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintWorkflowPdlConversionType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintWorkflowPdlConversionType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlConversionType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintWorkflowPrinterJobStatus(pub i32);
impl PrintWorkflowPrinterJobStatus {
    pub const Error: Self = Self(0i32);
    pub const Aborted: Self = Self(1i32);
    pub const InProgress: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
}
impl windows_core::TypeKind for PrintWorkflowPrinterJobStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintWorkflowPrinterJobStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintWorkflowPrinterJobStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintWorkflowPrinterJobStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowPrinterJobStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintWorkflowSessionStatus(pub i32);
impl PrintWorkflowSessionStatus {
    pub const Started: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Aborted: Self = Self(2i32);
    pub const Closed: Self = Self(3i32);
    pub const PdlDataAvailableForModification: Self = Self(4i32);
}
impl windows_core::TypeKind for PrintWorkflowSessionStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintWorkflowSessionStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintWorkflowSessionStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintWorkflowSessionStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowSessionStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintWorkflowSubmittedStatus(pub i32);
impl PrintWorkflowSubmittedStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl windows_core::TypeKind for PrintWorkflowSubmittedStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintWorkflowSubmittedStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintWorkflowSubmittedStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintWorkflowSubmittedStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintWorkflowUICompletionStatus(pub i32);
impl PrintWorkflowUICompletionStatus {
    pub const Completed: Self = Self(0i32);
    pub const LaunchFailed: Self = Self(1i32);
    pub const JobFailed: Self = Self(2i32);
    pub const UserCanceled: Self = Self(3i32);
}
impl windows_core::TypeKind for PrintWorkflowUICompletionStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintWorkflowUICompletionStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintWorkflowUICompletionStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintWorkflowUICompletionStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowUICompletionStatus;i4)");
}
