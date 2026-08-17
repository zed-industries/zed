windows_core::imp::define_interface!(IPrintSupportExtensionSession, IPrintSupportExtensionSession_Vtbl, 0xeea45f1a_f4c6_54b3_a0b8_a559839aa4c3);
impl windows_core::RuntimeType for IPrintSupportExtensionSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintSupportExtensionSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
    pub PrintTicketValidationRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePrintTicketValidationRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PrintDeviceCapabilitiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePrintDeviceCapabilitiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportExtensionSession2, IPrintSupportExtensionSession2_Vtbl, 0x10fa8c11_6de8_5765_8fcf_e716e0f27ed1);
impl windows_core::RuntimeType for IPrintSupportExtensionSession2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintSupportExtensionSession2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrinterSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePrinterSelected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportExtensionTriggerDetails, IPrintSupportExtensionTriggerDetails_Vtbl, 0xae083711_9b09_55d1_a0ae_2a14c5f83d6a);
impl windows_core::RuntimeType for IPrintSupportExtensionTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintSupportExtensionTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportPrintDeviceCapabilitiesChangedEventArgs, IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Vtbl, 0x15969bf0_9028_5722_8a37_7d7c34b41dd6);
impl windows_core::RuntimeType for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetCurrentPrintDeviceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetCurrentPrintDeviceCapabilities: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub UpdatePrintDeviceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    UpdatePrintDeviceCapabilities: usize,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2, IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_Vtbl, 0x469df9e7_fd07_5eeb_a07d_9fcc67f089ba);
impl windows_core::RuntimeType for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetSupportedPdlPassthroughContentTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetSupportedPdlPassthroughContentTypes: usize,
    pub ResourceLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetCurrentPrintDeviceResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetCurrentPrintDeviceResources: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub UpdatePrintDeviceResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    UpdatePrintDeviceResources: usize,
    pub SetPrintDeviceCapabilitiesUpdatePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportPrintDeviceCapabilitiesUpdatePolicy, IPrintSupportPrintDeviceCapabilitiesUpdatePolicy_Vtbl, 0x5f5fc025_8c35_5529_8038_8cdc3634bbcd);
impl windows_core::RuntimeType for IPrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintSupportPrintDeviceCapabilitiesUpdatePolicy_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics, IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_Vtbl, 0x3d9e1a70_7c39_551f_aa1f_f8ca35b3119e);
impl windows_core::RuntimeType for IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreatePeriodicRefresh: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePrintJobRefresh: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportPrintTicketElement, IPrintSupportPrintTicketElement_Vtbl, 0x4b2a4489_730d_5be7_80e6_8332941abf13);
impl windows_core::RuntimeType for IPrintSupportPrintTicketElement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintSupportPrintTicketElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LocalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetLocalName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub NamespaceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetNamespaceUri: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportPrintTicketValidationRequestedEventArgs, IPrintSupportPrintTicketValidationRequestedEventArgs_Vtbl, 0x338e4e69_db55_55c7_8338_ef64680a8f90);
impl windows_core::RuntimeType for IPrintSupportPrintTicketValidationRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintSupportPrintTicketValidationRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub PrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    PrintTicket: usize,
    pub SetPrintTicketValidationStatus: unsafe extern "system" fn(*mut core::ffi::c_void, WorkflowPrintTicketValidationStatus) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportPrinterSelectedEventArgs, IPrintSupportPrinterSelectedEventArgs_Vtbl, 0x7b1cb7d9_a8a4_5c09_adb2_66165f817977);
impl windows_core::RuntimeType for IPrintSupportPrinterSelectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintSupportPrinterSelectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub SourceAppInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    SourceAppInfo: usize,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub PrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    PrintTicket: usize,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub SetPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    SetPrintTicket: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAdditionalFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAdditionalFeatures: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAdditionalParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAdditionalParameters: usize,
    pub AllowedAdditionalFeaturesAndParametersCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Shell")]
    pub SetAdaptiveCard: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Shell"))]
    SetAdaptiveCard: usize,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportSessionInfo, IPrintSupportSessionInfo_Vtbl, 0x852149af_777d_53e9_9ee9_45d3f4b5be9c);
impl windows_core::RuntimeType for IPrintSupportSessionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintSupportSessionInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub SourceAppInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    SourceAppInfo: usize,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
}
windows_core::imp::define_interface!(IPrintSupportSettingsActivatedEventArgs, IPrintSupportSettingsActivatedEventArgs_Vtbl, 0x1e1b565e_a013_55ea_9b8c_eea39d9fb6c1);
impl windows_core::RuntimeType for IPrintSupportSettingsActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintSupportSettingsActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportSettingsUISession, IPrintSupportSettingsUISession_Vtbl, 0xc6da2251_83c3_55e4_a0f8_5de8b062adbf);
impl windows_core::RuntimeType for IPrintSupportSettingsUISession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintSupportSettingsUISession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub SessionPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    SessionPrintTicket: usize,
    pub DocumentTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LaunchKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SettingsLaunchKind) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub UpdatePrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    UpdatePrintTicket: usize,
    pub SessionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintSupportExtensionSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportExtensionSession, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportExtensionSession {
    #[cfg(feature = "Devices_Printers")]
    pub fn Printer(&self) -> windows_core::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Printer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PrintTicketValidationRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintTicketValidationRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintTicketValidationRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePrintTicketValidationRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePrintTicketValidationRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PrintDeviceCapabilitiesChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintDeviceCapabilitiesChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintDeviceCapabilitiesChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePrintDeviceCapabilitiesChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePrintDeviceCapabilitiesChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PrinterSelected<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrinterSelectedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IPrintSupportExtensionSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrinterSelected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePrinterSelected(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintSupportExtensionSession2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePrinterSelected)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for PrintSupportExtensionSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportExtensionSession>();
}
unsafe impl windows_core::Interface for PrintSupportExtensionSession {
    type Vtable = IPrintSupportExtensionSession_Vtbl;
    const IID: windows_core::GUID = <IPrintSupportExtensionSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportExtensionSession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession";
}
unsafe impl Send for PrintSupportExtensionSession {}
unsafe impl Sync for PrintSupportExtensionSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintSupportExtensionTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportExtensionTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportExtensionTriggerDetails {
    pub fn Session(&self) -> windows_core::Result<PrintSupportExtensionSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Session)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintSupportExtensionTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportExtensionTriggerDetails>();
}
unsafe impl windows_core::Interface for PrintSupportExtensionTriggerDetails {
    type Vtable = IPrintSupportExtensionTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IPrintSupportExtensionTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportExtensionTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionTriggerDetails";
}
unsafe impl Send for PrintSupportExtensionTriggerDetails {}
unsafe impl Sync for PrintSupportExtensionTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintSupportPrintDeviceCapabilitiesChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportPrintDeviceCapabilitiesChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetCurrentPrintDeviceCapabilities(&self) -> windows_core::Result<super::super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentPrintDeviceCapabilities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn UpdatePrintDeviceCapabilities<P0>(&self, updatedpdc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Data::Xml::Dom::XmlDocument>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdatePrintDeviceCapabilities)(windows_core::Interface::as_raw(this), updatedpdc.param().abi()).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetSupportedPdlPassthroughContentTypes<P0>(&self, supportedpdlcontenttypes: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSupportedPdlPassthroughContentTypes)(windows_core::Interface::as_raw(this), supportedpdlcontenttypes.param().abi()).ok() }
    }
    pub fn ResourceLanguage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetCurrentPrintDeviceResources(&self) -> windows_core::Result<super::super::super::Data::Xml::Dom::XmlDocument> {
        let this = &windows_core::Interface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentPrintDeviceResources)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn UpdatePrintDeviceResources<P0>(&self, updatedpdr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Data::Xml::Dom::XmlDocument>,
    {
        let this = &windows_core::Interface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).UpdatePrintDeviceResources)(windows_core::Interface::as_raw(this), updatedpdr.param().abi()).ok() }
    }
    pub fn SetPrintDeviceCapabilitiesUpdatePolicy<P0>(&self, updatepolicy: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PrintSupportPrintDeviceCapabilitiesUpdatePolicy>,
    {
        let this = &windows_core::Interface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrintDeviceCapabilitiesUpdatePolicy)(windows_core::Interface::as_raw(this), updatepolicy.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportPrintDeviceCapabilitiesChangedEventArgs>();
}
unsafe impl windows_core::Interface for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintSupportPrintDeviceCapabilitiesChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs";
}
unsafe impl Send for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {}
unsafe impl Sync for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintSupportPrintDeviceCapabilitiesUpdatePolicy(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportPrintDeviceCapabilitiesUpdatePolicy, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    pub fn CreatePeriodicRefresh(updateperiod: super::super::super::Foundation::TimeSpan) -> windows_core::Result<PrintSupportPrintDeviceCapabilitiesUpdatePolicy> {
        Self::IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePeriodicRefresh)(windows_core::Interface::as_raw(this), updateperiod, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreatePrintJobRefresh(numberofjobs: u32) -> windows_core::Result<PrintSupportPrintDeviceCapabilitiesUpdatePolicy> {
        Self::IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePrintJobRefresh)(windows_core::Interface::as_raw(this), numberofjobs, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics<R, F: FnOnce(&IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PrintSupportPrintDeviceCapabilitiesUpdatePolicy, IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportPrintDeviceCapabilitiesUpdatePolicy>();
}
unsafe impl windows_core::Interface for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesUpdatePolicy_Vtbl;
    const IID: windows_core::GUID = <IPrintSupportPrintDeviceCapabilitiesUpdatePolicy as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesUpdatePolicy";
}
unsafe impl Send for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {}
unsafe impl Sync for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintSupportPrintTicketElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportPrintTicketElement, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportPrintTicketElement {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PrintSupportPrintTicketElement, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNamespaceUri(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNamespaceUri)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for PrintSupportPrintTicketElement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportPrintTicketElement>();
}
unsafe impl windows_core::Interface for PrintSupportPrintTicketElement {
    type Vtable = IPrintSupportPrintTicketElement_Vtbl;
    const IID: windows_core::GUID = <IPrintSupportPrintTicketElement as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportPrintTicketElement {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketElement";
}
unsafe impl Send for PrintSupportPrintTicketElement {}
unsafe impl Sync for PrintSupportPrintTicketElement {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintSupportPrintTicketValidationRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportPrintTicketValidationRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportPrintTicketValidationRequestedEventArgs {
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn PrintTicket(&self) -> windows_core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintTicket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPrintTicketValidationStatus(&self, status: WorkflowPrintTicketValidationStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrintTicketValidationStatus)(windows_core::Interface::as_raw(this), status).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintSupportPrintTicketValidationRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportPrintTicketValidationRequestedEventArgs>();
}
unsafe impl windows_core::Interface for PrintSupportPrintTicketValidationRequestedEventArgs {
    type Vtable = IPrintSupportPrintTicketValidationRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintSupportPrintTicketValidationRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportPrintTicketValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs";
}
unsafe impl Send for PrintSupportPrintTicketValidationRequestedEventArgs {}
unsafe impl Sync for PrintSupportPrintTicketValidationRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintSupportPrinterSelectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportPrinterSelectedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportPrinterSelectedEventArgs {
    #[cfg(feature = "ApplicationModel")]
    pub fn SourceAppInfo(&self) -> windows_core::Result<super::super::super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceAppInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn PrintTicket(&self) -> windows_core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintTicket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn SetPrintTicket<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::PrintTicket::WorkflowPrintTicket>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrintTicket)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAdditionalFeatures<P0>(&self, features: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<PrintSupportPrintTicketElement>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAdditionalFeatures)(windows_core::Interface::as_raw(this), features.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAdditionalParameters<P0>(&self, parameters: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<PrintSupportPrintTicketElement>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAdditionalParameters)(windows_core::Interface::as_raw(this), parameters.param().abi()).ok() }
    }
    pub fn AllowedAdditionalFeaturesAndParametersCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowedAdditionalFeaturesAndParametersCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Shell")]
    pub fn SetAdaptiveCard<P0>(&self, adaptivecard: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::UI::Shell::IAdaptiveCard>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAdaptiveCard)(windows_core::Interface::as_raw(this), adaptivecard.param().abi()).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintSupportPrinterSelectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportPrinterSelectedEventArgs>();
}
unsafe impl windows_core::Interface for PrintSupportPrinterSelectedEventArgs {
    type Vtable = IPrintSupportPrinterSelectedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintSupportPrinterSelectedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportPrinterSelectedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrinterSelectedEventArgs";
}
unsafe impl Send for PrintSupportPrinterSelectedEventArgs {}
unsafe impl Sync for PrintSupportPrinterSelectedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintSupportSessionInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportSessionInfo, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportSessionInfo {
    #[cfg(feature = "ApplicationModel")]
    pub fn SourceAppInfo(&self) -> windows_core::Result<super::super::super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceAppInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
}
impl windows_core::RuntimeType for PrintSupportSessionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportSessionInfo>();
}
unsafe impl windows_core::Interface for PrintSupportSessionInfo {
    type Vtable = IPrintSupportSessionInfo_Vtbl;
    const IID: windows_core::GUID = <IPrintSupportSessionInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportSessionInfo {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSessionInfo";
}
unsafe impl Send for PrintSupportSessionInfo {}
unsafe impl Sync for PrintSupportSessionInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintSupportSettingsActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportSettingsActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(PrintSupportSettingsActivatedEventArgs, super::super::super::ApplicationModel::Activation::IActivatedEventArgs, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser);
impl PrintSupportSettingsActivatedEventArgs {
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
    pub fn Session(&self) -> windows_core::Result<PrintSupportSettingsUISession> {
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
}
impl windows_core::RuntimeType for PrintSupportSettingsActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportSettingsActivatedEventArgs>();
}
unsafe impl windows_core::Interface for PrintSupportSettingsActivatedEventArgs {
    type Vtable = IPrintSupportSettingsActivatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintSupportSettingsActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs";
}
unsafe impl Send for PrintSupportSettingsActivatedEventArgs {}
unsafe impl Sync for PrintSupportSettingsActivatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintSupportSettingsUISession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportSettingsUISession, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportSettingsUISession {
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn SessionPrintTicket(&self) -> windows_core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionPrintTicket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentTitle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LaunchKind(&self) -> windows_core::Result<SettingsLaunchKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn UpdatePrintTicket<P0>(&self, printticket: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::PrintTicket::WorkflowPrintTicket>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdatePrintTicket)(windows_core::Interface::as_raw(this), printticket.param().abi()).ok() }
    }
    pub fn SessionInfo(&self) -> windows_core::Result<PrintSupportSessionInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintSupportSettingsUISession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportSettingsUISession>();
}
unsafe impl windows_core::Interface for PrintSupportSettingsUISession {
    type Vtable = IPrintSupportSettingsUISession_Vtbl;
    const IID: windows_core::GUID = <IPrintSupportSettingsUISession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportSettingsUISession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsUISession";
}
unsafe impl Send for PrintSupportSettingsUISession {}
unsafe impl Sync for PrintSupportSettingsUISession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SettingsLaunchKind(pub i32);
impl SettingsLaunchKind {
    pub const JobPrintTicket: Self = Self(0i32);
    pub const UserDefaultPrintTicket: Self = Self(1i32);
}
impl windows_core::TypeKind for SettingsLaunchKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SettingsLaunchKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SettingsLaunchKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SettingsLaunchKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintSupport.SettingsLaunchKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WorkflowPrintTicketValidationStatus(pub i32);
impl WorkflowPrintTicketValidationStatus {
    pub const Resolved: Self = Self(0i32);
    pub const Conflicting: Self = Self(1i32);
    pub const Invalid: Self = Self(2i32);
}
impl windows_core::TypeKind for WorkflowPrintTicketValidationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WorkflowPrintTicketValidationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WorkflowPrintTicketValidationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WorkflowPrintTicketValidationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintSupport.WorkflowPrintTicketValidationStatus;i4)");
}
