windows_core::imp::define_interface!(IPrint3DWorkflow, IPrint3DWorkflow_Vtbl, 0xc56f74bd_3669_4a66_ab42_c8151930cd34);
impl windows_core::RuntimeType for IPrint3DWorkflow {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrint3DWorkflow_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetPrintModelPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPrintReady: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsPrintReady: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PrintRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePrintRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrint3DWorkflow2, IPrint3DWorkflow2_Vtbl, 0xa2a6c54f_8ac1_4918_9741_e34f3004239e);
impl windows_core::RuntimeType for IPrint3DWorkflow2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrint3DWorkflow2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrinterChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePrinterChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrint3DWorkflowPrintRequestedEventArgs, IPrint3DWorkflowPrintRequestedEventArgs_Vtbl, 0x19f8c858_5ac8_4b55_8a5f_e61567dafb4d);
impl windows_core::RuntimeType for IPrint3DWorkflowPrintRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrint3DWorkflowPrintRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Print3DWorkflowStatus) -> windows_core::HRESULT,
    pub SetExtendedStatus: unsafe extern "system" fn(*mut core::ffi::c_void, Print3DWorkflowDetail) -> windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSourceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrint3DWorkflowPrinterChangedEventArgs, IPrint3DWorkflowPrinterChangedEventArgs_Vtbl, 0x45226402_95fc_4847_93b3_134dbf5c60f7);
impl windows_core::RuntimeType for IPrint3DWorkflowPrinterChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrint3DWorkflowPrinterChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NewDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintExtensionContextStatic, IPrintExtensionContextStatic_Vtbl, 0xe70d9fc1_ff79_4aa4_8c9b_0c93aedfde8a);
impl windows_core::RuntimeType for IPrintExtensionContextStatic {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintExtensionContextStatic_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintNotificationEventDetails, IPrintNotificationEventDetails_Vtbl, 0xe00e4c8a_4828_4da1_8bb8_8672df8515e7);
impl windows_core::RuntimeType for IPrintNotificationEventDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintNotificationEventDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrinterName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EventData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetEventData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTaskConfiguration, IPrintTaskConfiguration_Vtbl, 0xe3c22451_3aa4_4885_9240_311f5f8fbe9d);
impl windows_core::RuntimeType for IPrintTaskConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTaskConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrinterExtensionContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSaveRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTaskConfigurationSaveRequest, IPrintTaskConfigurationSaveRequest_Vtbl, 0xeeaf2fcb_621e_4b62_ac77_b281cce08d60);
impl windows_core::RuntimeType for IPrintTaskConfigurationSaveRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTaskConfigurationSaveRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTaskConfigurationSaveRequestedDeferral, IPrintTaskConfigurationSaveRequestedDeferral_Vtbl, 0xe959d568_f729_44a4_871d_bd0628696a33);
impl windows_core::RuntimeType for IPrintTaskConfigurationSaveRequestedDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTaskConfigurationSaveRequestedDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTaskConfigurationSaveRequestedEventArgs, IPrintTaskConfigurationSaveRequestedEventArgs_Vtbl, 0xe06c2879_0d61_4938_91d0_96a45bee8479);
impl windows_core::RuntimeType for IPrintTaskConfigurationSaveRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTaskConfigurationSaveRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Print3DWorkflow(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Print3DWorkflow, windows_core::IUnknown, windows_core::IInspectable);
impl Print3DWorkflow {
    pub fn DeviceID(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceID)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPrintModelPackage(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPrintModelPackage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPrintReady(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPrintReady)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsPrintReady(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsPrintReady)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PrintRequested<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<Print3DWorkflow, Print3DWorkflowPrintRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintRequested)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePrintRequested(&self, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePrintRequested)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn PrinterChanged<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<Print3DWorkflow, Print3DWorkflowPrinterChangedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IPrint3DWorkflow2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrinterChanged)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePrinterChanged(&self, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrint3DWorkflow2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePrinterChanged)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
}
impl windows_core::RuntimeType for Print3DWorkflow {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrint3DWorkflow>();
}
unsafe impl windows_core::Interface for Print3DWorkflow {
    type Vtable = IPrint3DWorkflow_Vtbl;
    const IID: windows_core::GUID = <IPrint3DWorkflow as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Print3DWorkflow {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.Print3DWorkflow";
}
unsafe impl Send for Print3DWorkflow {}
unsafe impl Sync for Print3DWorkflow {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Print3DWorkflowPrintRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Print3DWorkflowPrintRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl Print3DWorkflowPrintRequestedEventArgs {
    pub fn Status(&self) -> windows_core::Result<Print3DWorkflowStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExtendedStatus(&self, value: Print3DWorkflowDetail) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExtendedStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetSource<P0>(&self, source: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSource)(windows_core::Interface::as_raw(this), source.param().abi()).ok() }
    }
    pub fn SetSourceChanged(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSourceChanged)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for Print3DWorkflowPrintRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrint3DWorkflowPrintRequestedEventArgs>();
}
unsafe impl windows_core::Interface for Print3DWorkflowPrintRequestedEventArgs {
    type Vtable = IPrint3DWorkflowPrintRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrint3DWorkflowPrintRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Print3DWorkflowPrintRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.Print3DWorkflowPrintRequestedEventArgs";
}
unsafe impl Send for Print3DWorkflowPrintRequestedEventArgs {}
unsafe impl Sync for Print3DWorkflowPrintRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Print3DWorkflowPrinterChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Print3DWorkflowPrinterChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl Print3DWorkflowPrinterChangedEventArgs {
    pub fn NewDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Print3DWorkflowPrinterChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrint3DWorkflowPrinterChangedEventArgs>();
}
unsafe impl windows_core::Interface for Print3DWorkflowPrinterChangedEventArgs {
    type Vtable = IPrint3DWorkflowPrinterChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrint3DWorkflowPrinterChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Print3DWorkflowPrinterChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.Print3DWorkflowPrinterChangedEventArgs";
}
unsafe impl Send for Print3DWorkflowPrinterChangedEventArgs {}
unsafe impl Sync for Print3DWorkflowPrinterChangedEventArgs {}
pub struct PrintExtensionContext;
impl PrintExtensionContext {
    pub fn FromDeviceId(deviceid: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPrintExtensionContextStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromDeviceId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPrintExtensionContextStatic<R, F: FnOnce(&IPrintExtensionContextStatic) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PrintExtensionContext, IPrintExtensionContextStatic> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PrintExtensionContext {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintExtensionContext";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintNotificationEventDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintNotificationEventDetails, windows_core::IUnknown, windows_core::IInspectable);
impl PrintNotificationEventDetails {
    pub fn PrinterName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrinterName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EventData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EventData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetEventData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEventData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for PrintNotificationEventDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintNotificationEventDetails>();
}
unsafe impl windows_core::Interface for PrintNotificationEventDetails {
    type Vtable = IPrintNotificationEventDetails_Vtbl;
    const IID: windows_core::GUID = <IPrintNotificationEventDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintNotificationEventDetails {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintNotificationEventDetails";
}
unsafe impl Send for PrintNotificationEventDetails {}
unsafe impl Sync for PrintNotificationEventDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintTaskConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTaskConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTaskConfiguration {
    pub fn PrinterExtensionContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrinterExtensionContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SaveRequested<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintTaskConfiguration, PrintTaskConfigurationSaveRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveRequested)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSaveRequested(&self, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSaveRequested)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
}
impl windows_core::RuntimeType for PrintTaskConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTaskConfiguration>();
}
unsafe impl windows_core::Interface for PrintTaskConfiguration {
    type Vtable = IPrintTaskConfiguration_Vtbl;
    const IID: windows_core::GUID = <IPrintTaskConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTaskConfiguration {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintTaskConfiguration";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintTaskConfigurationSaveRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTaskConfigurationSaveRequest, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTaskConfigurationSaveRequest {
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Save<P0>(&self, printerextensioncontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Save)(windows_core::Interface::as_raw(this), printerextensioncontext.param().abi()).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<PrintTaskConfigurationSaveRequestedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Deadline(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Deadline)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PrintTaskConfigurationSaveRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTaskConfigurationSaveRequest>();
}
unsafe impl windows_core::Interface for PrintTaskConfigurationSaveRequest {
    type Vtable = IPrintTaskConfigurationSaveRequest_Vtbl;
    const IID: windows_core::GUID = <IPrintTaskConfigurationSaveRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTaskConfigurationSaveRequest {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequest";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintTaskConfigurationSaveRequestedDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTaskConfigurationSaveRequestedDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTaskConfigurationSaveRequestedDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for PrintTaskConfigurationSaveRequestedDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTaskConfigurationSaveRequestedDeferral>();
}
unsafe impl windows_core::Interface for PrintTaskConfigurationSaveRequestedDeferral {
    type Vtable = IPrintTaskConfigurationSaveRequestedDeferral_Vtbl;
    const IID: windows_core::GUID = <IPrintTaskConfigurationSaveRequestedDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTaskConfigurationSaveRequestedDeferral {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedDeferral";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintTaskConfigurationSaveRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTaskConfigurationSaveRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTaskConfigurationSaveRequestedEventArgs {
    pub fn Request(&self) -> windows_core::Result<PrintTaskConfigurationSaveRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintTaskConfigurationSaveRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTaskConfigurationSaveRequestedEventArgs>();
}
unsafe impl windows_core::Interface for PrintTaskConfigurationSaveRequestedEventArgs {
    type Vtable = IPrintTaskConfigurationSaveRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintTaskConfigurationSaveRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTaskConfigurationSaveRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Print3DWorkflowDetail(pub i32);
impl Print3DWorkflowDetail {
    pub const Unknown: Self = Self(0i32);
    pub const ModelExceedsPrintBed: Self = Self(1i32);
    pub const UploadFailed: Self = Self(2i32);
    pub const InvalidMaterialSelection: Self = Self(3i32);
    pub const InvalidModel: Self = Self(4i32);
    pub const ModelNotManifold: Self = Self(5i32);
    pub const InvalidPrintTicket: Self = Self(6i32);
}
impl windows_core::TypeKind for Print3DWorkflowDetail {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Print3DWorkflowDetail {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Print3DWorkflowDetail").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for Print3DWorkflowDetail {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.Extensions.Print3DWorkflowDetail;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Print3DWorkflowStatus(pub i32);
impl Print3DWorkflowStatus {
    pub const Abandoned: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
    pub const Slicing: Self = Self(3i32);
    pub const Submitted: Self = Self(4i32);
}
impl windows_core::TypeKind for Print3DWorkflowStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Print3DWorkflowStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Print3DWorkflowStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for Print3DWorkflowStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.Extensions.Print3DWorkflowStatus;i4)");
}
