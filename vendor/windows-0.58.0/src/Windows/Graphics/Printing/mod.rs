#[cfg(feature = "Graphics_Printing_OptionDetails")]
pub mod OptionDetails;
#[cfg(feature = "Graphics_Printing_PrintSupport")]
pub mod PrintSupport;
#[cfg(feature = "Graphics_Printing_PrintTicket")]
pub mod PrintTicket;
#[cfg(feature = "Graphics_Printing_Workflow")]
pub mod Workflow;
windows_core::imp::define_interface!(IPrintDocumentSource, IPrintDocumentSource_Vtbl, 0xdedc0c30_f1eb_47df_aae6_ed5427511f01);
impl core::ops::Deref for IPrintDocumentSource {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintDocumentSource, windows_core::IUnknown, windows_core::IInspectable);
impl IPrintDocumentSource {}
impl windows_core::RuntimeType for IPrintDocumentSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintDocumentSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPrintManager, IPrintManager_Vtbl, 0xff2a9694_8c99_44fd_ae4a_19d9aa9a0f0a);
impl windows_core::RuntimeType for IPrintManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrintTaskRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePrintTaskRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintManagerStatic, IPrintManagerStatic_Vtbl, 0x58185dcd_e634_4654_84f0_e0152a8217ac);
impl windows_core::RuntimeType for IPrintManagerStatic {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintManagerStatic_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowPrintUIAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintManagerStatic2, IPrintManagerStatic2_Vtbl, 0x35a99955_e6ab_4139_9abd_b86a729b3598);
impl windows_core::RuntimeType for IPrintManagerStatic2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintManagerStatic2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintPageInfo, IPrintPageInfo_Vtbl, 0xdd4be9c9_a6a1_4ada_930e_da872a4f23d3);
impl windows_core::RuntimeType for IPrintPageInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintPageInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetMediaSize: unsafe extern "system" fn(*mut core::ffi::c_void, PrintMediaSize) -> windows_core::HRESULT,
    pub MediaSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintMediaSize) -> windows_core::HRESULT,
    pub SetPageSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Size) -> windows_core::HRESULT,
    pub PageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub SetDpiX: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DpiX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDpiY: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DpiY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, PrintOrientation) -> windows_core::HRESULT,
    pub Orientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintOrientation) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintPageRange, IPrintPageRange_Vtbl, 0xf8a06c54_6e7c_51c5_57fd_0660c2d71513);
impl windows_core::RuntimeType for IPrintPageRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintPageRange_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FirstPageNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastPageNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintPageRangeFactory, IPrintPageRangeFactory_Vtbl, 0x408fd45f_e047_5f85_7129_fb085a4fad14);
impl windows_core::RuntimeType for IPrintPageRangeFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintPageRangeFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithSinglePage: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintPageRangeOptions, IPrintPageRangeOptions_Vtbl, 0xce6db728_1357_46b2_a923_79f995f448fc);
impl windows_core::RuntimeType for IPrintPageRangeOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintPageRangeOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetAllowAllPages: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AllowAllPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowCurrentPage: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AllowCurrentPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowCustomSetOfPages: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AllowCustomSetOfPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTask, IPrintTask_Vtbl, 0x61d80247_6cf6_4fad_84e2_a5e82e2d4ceb);
impl windows_core::RuntimeType for IPrintTask {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTask_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    Properties: usize,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Options: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Previewing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePreviewing: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Submitting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSubmitting: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Progressing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveProgressing: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTask2, IPrintTask2_Vtbl, 0x36234877_3e53_4d9d_8f5e_316ac8dedae1);
impl windows_core::RuntimeType for IPrintTask2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTask2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetIsPreviewEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsPreviewEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTaskCompletedEventArgs, IPrintTaskCompletedEventArgs_Vtbl, 0x5bcd34af_24e9_4c10_8d07_14c346ba3fce);
impl windows_core::RuntimeType for IPrintTaskCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTaskCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Completion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintTaskCompletion) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTaskOptions, IPrintTaskOptions_Vtbl, 0x5a0a66bb_d289_41bb_96dd_57e28338ae3f);
impl windows_core::RuntimeType for IPrintTaskOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTaskOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetBordering: unsafe extern "system" fn(*mut core::ffi::c_void, PrintBordering) -> windows_core::HRESULT,
    pub Bordering: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintBordering) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetPagePrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetPagePrintTicket: usize,
}
windows_core::imp::define_interface!(IPrintTaskOptions2, IPrintTaskOptions2_Vtbl, 0xeb9b1606_9a36_4b59_8617_b217849262e1);
impl windows_core::RuntimeType for IPrintTaskOptions2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTaskOptions2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PageRangeOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CustomPageRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CustomPageRanges: usize,
}
windows_core::imp::define_interface!(IPrintTaskOptionsCore, IPrintTaskOptionsCore_Vtbl, 0x1bdbb474_4ed1_41eb_be3c_72d18ed67337);
impl core::ops::Deref for IPrintTaskOptionsCore {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintTaskOptionsCore, windows_core::IUnknown, windows_core::IInspectable);
impl IPrintTaskOptionsCore {
    pub fn GetPageDescription(&self, jobpagenumber: u32) -> windows_core::Result<PrintPageDescription> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPageDescription)(windows_core::Interface::as_raw(this), jobpagenumber, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IPrintTaskOptionsCore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTaskOptionsCore_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetPageDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut PrintPageDescription) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTaskOptionsCoreProperties, IPrintTaskOptionsCoreProperties_Vtbl, 0xc1b71832_9e93_4e55_814b_3326a59efce1);
impl core::ops::Deref for IPrintTaskOptionsCoreProperties {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintTaskOptionsCoreProperties, windows_core::IUnknown, windows_core::IInspectable);
impl IPrintTaskOptionsCoreProperties {
    pub fn SetMediaSize(&self, value: PrintMediaSize) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMediaSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MediaSize(&self) -> windows_core::Result<PrintMediaSize> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMediaType(&self, value: PrintMediaType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMediaType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MediaType(&self) -> windows_core::Result<PrintMediaType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOrientation(&self, value: PrintOrientation) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOrientation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Orientation(&self) -> windows_core::Result<PrintOrientation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPrintQuality(&self, value: PrintQuality) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrintQuality)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PrintQuality(&self) -> windows_core::Result<PrintQuality> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintQuality)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetColorMode(&self, value: PrintColorMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetColorMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ColorMode(&self) -> windows_core::Result<PrintColorMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDuplex(&self, value: PrintDuplex) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDuplex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Duplex(&self) -> windows_core::Result<PrintDuplex> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duplex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCollation(&self, value: PrintCollation) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCollation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Collation(&self) -> windows_core::Result<PrintCollation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Collation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStaple(&self, value: PrintStaple) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStaple)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Staple(&self) -> windows_core::Result<PrintStaple> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Staple)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHolePunch(&self, value: PrintHolePunch) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHolePunch)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HolePunch(&self) -> windows_core::Result<PrintHolePunch> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HolePunch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBinding(&self, value: PrintBinding) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBinding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Binding(&self) -> windows_core::Result<PrintBinding> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Binding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinCopies(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinCopies)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxCopies(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxCopies)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNumberOfCopies(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNumberOfCopies)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumberOfCopies(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberOfCopies)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IPrintTaskOptionsCoreProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTaskOptionsCoreProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetMediaSize: unsafe extern "system" fn(*mut core::ffi::c_void, PrintMediaSize) -> windows_core::HRESULT,
    pub MediaSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintMediaSize) -> windows_core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, PrintMediaType) -> windows_core::HRESULT,
    pub MediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintMediaType) -> windows_core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, PrintOrientation) -> windows_core::HRESULT,
    pub Orientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintOrientation) -> windows_core::HRESULT,
    pub SetPrintQuality: unsafe extern "system" fn(*mut core::ffi::c_void, PrintQuality) -> windows_core::HRESULT,
    pub PrintQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintQuality) -> windows_core::HRESULT,
    pub SetColorMode: unsafe extern "system" fn(*mut core::ffi::c_void, PrintColorMode) -> windows_core::HRESULT,
    pub ColorMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintColorMode) -> windows_core::HRESULT,
    pub SetDuplex: unsafe extern "system" fn(*mut core::ffi::c_void, PrintDuplex) -> windows_core::HRESULT,
    pub Duplex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintDuplex) -> windows_core::HRESULT,
    pub SetCollation: unsafe extern "system" fn(*mut core::ffi::c_void, PrintCollation) -> windows_core::HRESULT,
    pub Collation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintCollation) -> windows_core::HRESULT,
    pub SetStaple: unsafe extern "system" fn(*mut core::ffi::c_void, PrintStaple) -> windows_core::HRESULT,
    pub Staple: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintStaple) -> windows_core::HRESULT,
    pub SetHolePunch: unsafe extern "system" fn(*mut core::ffi::c_void, PrintHolePunch) -> windows_core::HRESULT,
    pub HolePunch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintHolePunch) -> windows_core::HRESULT,
    pub SetBinding: unsafe extern "system" fn(*mut core::ffi::c_void, PrintBinding) -> windows_core::HRESULT,
    pub Binding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintBinding) -> windows_core::HRESULT,
    pub MinCopies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxCopies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetNumberOfCopies: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub NumberOfCopies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTaskOptionsCoreUIConfiguration, IPrintTaskOptionsCoreUIConfiguration_Vtbl, 0x62e69e23_9a1e_4336_b74f_3cc7f4cff709);
impl core::ops::Deref for IPrintTaskOptionsCoreUIConfiguration {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintTaskOptionsCoreUIConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl IPrintTaskOptionsCoreUIConfiguration {
    #[cfg(feature = "Foundation_Collections")]
    pub fn DisplayedOptions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayedOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IPrintTaskOptionsCoreUIConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTaskOptionsCoreUIConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub DisplayedOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DisplayedOptions: usize,
}
windows_core::imp::define_interface!(IPrintTaskProgressingEventArgs, IPrintTaskProgressingEventArgs_Vtbl, 0x810cd3cb_b410_4282_a073_5ac378234174);
impl windows_core::RuntimeType for IPrintTaskProgressingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTaskProgressingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DocumentPageCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTaskRequest, IPrintTaskRequest_Vtbl, 0x6ff61e2e_2722_4240_a67c_f364849a17f3);
impl windows_core::RuntimeType for IPrintTaskRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTaskRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub CreatePrintTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTaskRequestedDeferral, IPrintTaskRequestedDeferral_Vtbl, 0xcfefb3f0_ce3e_42c7_9496_64800c622c44);
impl windows_core::RuntimeType for IPrintTaskRequestedDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTaskRequestedDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTaskRequestedEventArgs, IPrintTaskRequestedEventArgs_Vtbl, 0xd0aff924_a31b_454c_a7b6_5d0cc522fc16);
impl windows_core::RuntimeType for IPrintTaskRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTaskRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTaskSourceRequestedArgs, IPrintTaskSourceRequestedArgs_Vtbl, 0xf9f067be_f456_41f0_9c98_5ce73e851410);
impl windows_core::RuntimeType for IPrintTaskSourceRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTaskSourceRequestedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTaskSourceRequestedDeferral, IPrintTaskSourceRequestedDeferral_Vtbl, 0x4a1560d1_6992_4d9d_8555_4ca4563fb166);
impl windows_core::RuntimeType for IPrintTaskSourceRequestedDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTaskSourceRequestedDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTaskTargetDeviceSupport, IPrintTaskTargetDeviceSupport_Vtbl, 0x295d70c0_c2cb_4b7d_b0ea_93095091a220);
impl windows_core::RuntimeType for IPrintTaskTargetDeviceSupport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTaskTargetDeviceSupport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetIsPrinterTargetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsPrinterTargetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIs3DManufacturingTargetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Is3DManufacturingTargetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStandardPrintTaskOptionsStatic, IStandardPrintTaskOptionsStatic_Vtbl, 0xb4483d26_0dd0_4cd4_baff_930fc7d6a574);
impl windows_core::RuntimeType for IStandardPrintTaskOptionsStatic {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStandardPrintTaskOptionsStatic_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MediaSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Orientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PrintQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ColorMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Duplex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Collation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Staple: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub HolePunch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Binding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Copies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub NUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub InputBin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStandardPrintTaskOptionsStatic2, IStandardPrintTaskOptionsStatic2_Vtbl, 0x3be38bf4_7a44_4269_9a52_81261e289ee9);
impl windows_core::RuntimeType for IStandardPrintTaskOptionsStatic2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStandardPrintTaskOptionsStatic2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Bordering: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStandardPrintTaskOptionsStatic3, IStandardPrintTaskOptionsStatic3_Vtbl, 0xbbf68e86_3858_41b3_a799_55dd9888d475);
impl windows_core::RuntimeType for IStandardPrintTaskOptionsStatic3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStandardPrintTaskOptionsStatic3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CustomPageRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintManager, windows_core::IUnknown, windows_core::IInspectable);
impl PrintManager {
    pub fn PrintTaskRequested<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PrintManager, PrintTaskRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintTaskRequested)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePrintTaskRequested(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePrintTaskRequested)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<PrintManager> {
        Self::IPrintManagerStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShowPrintUIAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IPrintManagerStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowPrintUIAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IPrintManagerStatic2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrintManagerStatic<R, F: FnOnce(&IPrintManagerStatic) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PrintManager, IPrintManagerStatic> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPrintManagerStatic2<R, F: FnOnce(&IPrintManagerStatic2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PrintManager, IPrintManagerStatic2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PrintManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintManager>();
}
unsafe impl windows_core::Interface for PrintManager {
    type Vtable = IPrintManager_Vtbl;
    const IID: windows_core::GUID = <IPrintManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintManager {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintManager";
}
unsafe impl Send for PrintManager {}
unsafe impl Sync for PrintManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintPageInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintPageInfo, windows_core::IUnknown, windows_core::IInspectable);
impl PrintPageInfo {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PrintPageInfo, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetMediaSize(&self, value: PrintMediaSize) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMediaSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MediaSize(&self) -> windows_core::Result<PrintMediaSize> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPageSize(&self, value: super::super::Foundation::Size) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPageSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PageSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDpiX(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDpiX)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DpiX(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DpiX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDpiY(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDpiY)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DpiY(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DpiY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOrientation(&self, value: PrintOrientation) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOrientation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Orientation(&self) -> windows_core::Result<PrintOrientation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PrintPageInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintPageInfo>();
}
unsafe impl windows_core::Interface for PrintPageInfo {
    type Vtable = IPrintPageInfo_Vtbl;
    const IID: windows_core::GUID = <IPrintPageInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintPageInfo {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintPageInfo";
}
unsafe impl Send for PrintPageInfo {}
unsafe impl Sync for PrintPageInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintPageRange(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintPageRange, windows_core::IUnknown, windows_core::IInspectable);
impl PrintPageRange {
    pub fn FirstPageNumber(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstPageNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LastPageNumber(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastPageNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(firstpage: i32, lastpage: i32) -> windows_core::Result<PrintPageRange> {
        Self::IPrintPageRangeFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), firstpage, lastpage, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithSinglePage(page: i32) -> windows_core::Result<PrintPageRange> {
        Self::IPrintPageRangeFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithSinglePage)(windows_core::Interface::as_raw(this), page, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPrintPageRangeFactory<R, F: FnOnce(&IPrintPageRangeFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PrintPageRange, IPrintPageRangeFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PrintPageRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintPageRange>();
}
unsafe impl windows_core::Interface for PrintPageRange {
    type Vtable = IPrintPageRange_Vtbl;
    const IID: windows_core::GUID = <IPrintPageRange as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintPageRange {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintPageRange";
}
unsafe impl Send for PrintPageRange {}
unsafe impl Sync for PrintPageRange {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintPageRangeOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintPageRangeOptions, windows_core::IUnknown, windows_core::IInspectable);
impl PrintPageRangeOptions {
    pub fn SetAllowAllPages(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowAllPages)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowAllPages(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowAllPages)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowCurrentPage(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowCurrentPage)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowCurrentPage(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowCurrentPage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowCustomSetOfPages(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowCustomSetOfPages)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowCustomSetOfPages(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowCustomSetOfPages)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PrintPageRangeOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintPageRangeOptions>();
}
unsafe impl windows_core::Interface for PrintPageRangeOptions {
    type Vtable = IPrintPageRangeOptions_Vtbl;
    const IID: windows_core::GUID = <IPrintPageRangeOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintPageRangeOptions {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintPageRangeOptions";
}
unsafe impl Send for PrintPageRangeOptions {}
unsafe impl Sync for PrintPageRangeOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintTask(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTask, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTask {
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub fn Properties(&self) -> windows_core::Result<super::super::ApplicationModel::DataTransfer::DataPackagePropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Source(&self) -> windows_core::Result<IPrintDocumentSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Options(&self) -> windows_core::Result<PrintTaskOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Options)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Previewing<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PrintTask, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Previewing)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePreviewing(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePreviewing)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn Submitting<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PrintTask, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Submitting)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSubmitting(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSubmitting)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn Progressing<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskProgressingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progressing)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveProgressing(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveProgressing)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn Completed<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCompleted(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCompleted)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn SetIsPreviewEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintTask2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsPreviewEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPreviewEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPrintTask2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPreviewEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsPrinterTargetEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintTaskTargetDeviceSupport>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsPrinterTargetEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPrinterTargetEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPrintTaskTargetDeviceSupport>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPrinterTargetEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIs3DManufacturingTargetEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintTaskTargetDeviceSupport>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIs3DManufacturingTargetEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Is3DManufacturingTargetEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPrintTaskTargetDeviceSupport>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Is3DManufacturingTargetEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PrintTask {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTask>();
}
unsafe impl windows_core::Interface for PrintTask {
    type Vtable = IPrintTask_Vtbl;
    const IID: windows_core::GUID = <IPrintTask as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTask {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTask";
}
unsafe impl Send for PrintTask {}
unsafe impl Sync for PrintTask {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintTaskCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTaskCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTaskCompletedEventArgs {
    pub fn Completion(&self) -> windows_core::Result<PrintTaskCompletion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PrintTaskCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTaskCompletedEventArgs>();
}
unsafe impl windows_core::Interface for PrintTaskCompletedEventArgs {
    type Vtable = IPrintTaskCompletedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintTaskCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTaskCompletedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskCompletedEventArgs";
}
unsafe impl Send for PrintTaskCompletedEventArgs {}
unsafe impl Sync for PrintTaskCompletedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintTaskOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTaskOptions, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PrintTaskOptions, IPrintTaskOptionsCore, IPrintTaskOptionsCoreProperties, IPrintTaskOptionsCoreUIConfiguration);
impl PrintTaskOptions {
    pub fn SetBordering(&self, value: PrintBordering) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBordering)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bordering(&self) -> windows_core::Result<PrintBordering> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bordering)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetPagePrintTicket<P0>(&self, printpageinfo: P0) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStream>
    where
        P0: windows_core::Param<PrintPageInfo>,
    {
        let this = &windows_core::Interface::cast::<IPrintTaskOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPagePrintTicket)(windows_core::Interface::as_raw(this), printpageinfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PageRangeOptions(&self) -> windows_core::Result<PrintPageRangeOptions> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptions2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageRangeOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CustomPageRanges(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<PrintPageRange>> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptions2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomPageRanges)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPageDescription(&self, jobpagenumber: u32) -> windows_core::Result<PrintPageDescription> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPageDescription)(windows_core::Interface::as_raw(this), jobpagenumber, &mut result__).map(|| result__)
        }
    }
    pub fn SetMediaSize(&self, value: PrintMediaSize) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMediaSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MediaSize(&self) -> windows_core::Result<PrintMediaSize> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMediaType(&self, value: PrintMediaType) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMediaType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MediaType(&self) -> windows_core::Result<PrintMediaType> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOrientation(&self, value: PrintOrientation) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOrientation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Orientation(&self) -> windows_core::Result<PrintOrientation> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPrintQuality(&self, value: PrintQuality) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrintQuality)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PrintQuality(&self) -> windows_core::Result<PrintQuality> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintQuality)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetColorMode(&self, value: PrintColorMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetColorMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ColorMode(&self) -> windows_core::Result<PrintColorMode> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDuplex(&self, value: PrintDuplex) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDuplex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Duplex(&self) -> windows_core::Result<PrintDuplex> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duplex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCollation(&self, value: PrintCollation) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCollation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Collation(&self) -> windows_core::Result<PrintCollation> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Collation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStaple(&self, value: PrintStaple) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStaple)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Staple(&self) -> windows_core::Result<PrintStaple> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Staple)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHolePunch(&self, value: PrintHolePunch) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHolePunch)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HolePunch(&self) -> windows_core::Result<PrintHolePunch> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HolePunch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBinding(&self, value: PrintBinding) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBinding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Binding(&self) -> windows_core::Result<PrintBinding> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Binding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinCopies(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinCopies)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxCopies(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxCopies)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNumberOfCopies(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNumberOfCopies)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumberOfCopies(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberOfCopies)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DisplayedOptions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IPrintTaskOptionsCoreUIConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayedOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintTaskOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTaskOptionsCore>();
}
unsafe impl windows_core::Interface for PrintTaskOptions {
    type Vtable = IPrintTaskOptionsCore_Vtbl;
    const IID: windows_core::GUID = <IPrintTaskOptionsCore as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTaskOptions {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskOptions";
}
unsafe impl Send for PrintTaskOptions {}
unsafe impl Sync for PrintTaskOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintTaskProgressingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTaskProgressingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTaskProgressingEventArgs {
    pub fn DocumentPageCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentPageCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PrintTaskProgressingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTaskProgressingEventArgs>();
}
unsafe impl windows_core::Interface for PrintTaskProgressingEventArgs {
    type Vtable = IPrintTaskProgressingEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintTaskProgressingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTaskProgressingEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskProgressingEventArgs";
}
unsafe impl Send for PrintTaskProgressingEventArgs {}
unsafe impl Sync for PrintTaskProgressingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintTaskRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTaskRequest, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTaskRequest {
    pub fn Deadline(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Deadline)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreatePrintTask<P0>(&self, title: &windows_core::HSTRING, handler: P0) -> windows_core::Result<PrintTask>
    where
        P0: windows_core::Param<PrintTaskSourceRequestedHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePrintTask)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(title), handler.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<PrintTaskRequestedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintTaskRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTaskRequest>();
}
unsafe impl windows_core::Interface for PrintTaskRequest {
    type Vtable = IPrintTaskRequest_Vtbl;
    const IID: windows_core::GUID = <IPrintTaskRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTaskRequest {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskRequest";
}
unsafe impl Send for PrintTaskRequest {}
unsafe impl Sync for PrintTaskRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintTaskRequestedDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTaskRequestedDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTaskRequestedDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for PrintTaskRequestedDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTaskRequestedDeferral>();
}
unsafe impl windows_core::Interface for PrintTaskRequestedDeferral {
    type Vtable = IPrintTaskRequestedDeferral_Vtbl;
    const IID: windows_core::GUID = <IPrintTaskRequestedDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTaskRequestedDeferral {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskRequestedDeferral";
}
unsafe impl Send for PrintTaskRequestedDeferral {}
unsafe impl Sync for PrintTaskRequestedDeferral {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintTaskRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTaskRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTaskRequestedEventArgs {
    pub fn Request(&self) -> windows_core::Result<PrintTaskRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintTaskRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTaskRequestedEventArgs>();
}
unsafe impl windows_core::Interface for PrintTaskRequestedEventArgs {
    type Vtable = IPrintTaskRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintTaskRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTaskRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskRequestedEventArgs";
}
unsafe impl Send for PrintTaskRequestedEventArgs {}
unsafe impl Sync for PrintTaskRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintTaskSourceRequestedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTaskSourceRequestedArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTaskSourceRequestedArgs {
    pub fn Deadline(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Deadline)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSource<P0>(&self, source: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrintDocumentSource>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSource)(windows_core::Interface::as_raw(this), source.param().abi()).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<PrintTaskSourceRequestedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintTaskSourceRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTaskSourceRequestedArgs>();
}
unsafe impl windows_core::Interface for PrintTaskSourceRequestedArgs {
    type Vtable = IPrintTaskSourceRequestedArgs_Vtbl;
    const IID: windows_core::GUID = <IPrintTaskSourceRequestedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTaskSourceRequestedArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskSourceRequestedArgs";
}
unsafe impl Send for PrintTaskSourceRequestedArgs {}
unsafe impl Sync for PrintTaskSourceRequestedArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PrintTaskSourceRequestedDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTaskSourceRequestedDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTaskSourceRequestedDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for PrintTaskSourceRequestedDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTaskSourceRequestedDeferral>();
}
unsafe impl windows_core::Interface for PrintTaskSourceRequestedDeferral {
    type Vtable = IPrintTaskSourceRequestedDeferral_Vtbl;
    const IID: windows_core::GUID = <IPrintTaskSourceRequestedDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTaskSourceRequestedDeferral {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskSourceRequestedDeferral";
}
unsafe impl Send for PrintTaskSourceRequestedDeferral {}
unsafe impl Sync for PrintTaskSourceRequestedDeferral {}
pub struct StandardPrintTaskOptions;
impl StandardPrintTaskOptions {
    pub fn MediaSize() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaSize)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MediaType() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Orientation() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orientation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PrintQuality() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintQuality)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ColorMode() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorMode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Duplex() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duplex)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Collation() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Collation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Staple() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Staple)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn HolePunch() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HolePunch)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Binding() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Binding)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Copies() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Copies)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn NUp() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NUp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn InputBin() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputBin)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Bordering() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bordering)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CustomPageRanges() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomPageRanges)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IStandardPrintTaskOptionsStatic<R, F: FnOnce(&IStandardPrintTaskOptionsStatic) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StandardPrintTaskOptions, IStandardPrintTaskOptionsStatic> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStandardPrintTaskOptionsStatic2<R, F: FnOnce(&IStandardPrintTaskOptionsStatic2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StandardPrintTaskOptions, IStandardPrintTaskOptionsStatic2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStandardPrintTaskOptionsStatic3<R, F: FnOnce(&IStandardPrintTaskOptionsStatic3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StandardPrintTaskOptions, IStandardPrintTaskOptionsStatic3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for StandardPrintTaskOptions {
    const NAME: &'static str = "Windows.Graphics.Printing.StandardPrintTaskOptions";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintBinding(pub i32);
impl PrintBinding {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const None: Self = Self(3i32);
    pub const Bale: Self = Self(4i32);
    pub const BindBottom: Self = Self(5i32);
    pub const BindLeft: Self = Self(6i32);
    pub const BindRight: Self = Self(7i32);
    pub const BindTop: Self = Self(8i32);
    pub const Booklet: Self = Self(9i32);
    pub const EdgeStitchBottom: Self = Self(10i32);
    pub const EdgeStitchLeft: Self = Self(11i32);
    pub const EdgeStitchRight: Self = Self(12i32);
    pub const EdgeStitchTop: Self = Self(13i32);
    pub const Fold: Self = Self(14i32);
    pub const JogOffset: Self = Self(15i32);
    pub const Trim: Self = Self(16i32);
}
impl windows_core::TypeKind for PrintBinding {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintBinding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintBinding").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintBinding {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintBinding;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintBordering(pub i32);
impl PrintBordering {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Bordered: Self = Self(3i32);
    pub const Borderless: Self = Self(4i32);
}
impl windows_core::TypeKind for PrintBordering {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintBordering {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintBordering").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintBordering {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintBordering;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintCollation(pub i32);
impl PrintCollation {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Collated: Self = Self(3i32);
    pub const Uncollated: Self = Self(4i32);
}
impl windows_core::TypeKind for PrintCollation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintCollation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintCollation").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintCollation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintCollation;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintColorMode(pub i32);
impl PrintColorMode {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Color: Self = Self(3i32);
    pub const Grayscale: Self = Self(4i32);
    pub const Monochrome: Self = Self(5i32);
    pub const AutoSelect: Self = Self(6i32);
}
impl windows_core::TypeKind for PrintColorMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintColorMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintColorMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintColorMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintColorMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintDuplex(pub i32);
impl PrintDuplex {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const OneSided: Self = Self(3i32);
    pub const TwoSidedShortEdge: Self = Self(4i32);
    pub const TwoSidedLongEdge: Self = Self(5i32);
}
impl windows_core::TypeKind for PrintDuplex {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintDuplex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintDuplex").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintDuplex {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintDuplex;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintHolePunch(pub i32);
impl PrintHolePunch {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const None: Self = Self(3i32);
    pub const LeftEdge: Self = Self(4i32);
    pub const RightEdge: Self = Self(5i32);
    pub const TopEdge: Self = Self(6i32);
    pub const BottomEdge: Self = Self(7i32);
}
impl windows_core::TypeKind for PrintHolePunch {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintHolePunch {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintHolePunch").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintHolePunch {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintHolePunch;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintMediaSize(pub i32);
impl PrintMediaSize {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const BusinessCard: Self = Self(3i32);
    pub const CreditCard: Self = Self(4i32);
    pub const IsoA0: Self = Self(5i32);
    pub const IsoA1: Self = Self(6i32);
    pub const IsoA10: Self = Self(7i32);
    pub const IsoA2: Self = Self(8i32);
    pub const IsoA3: Self = Self(9i32);
    pub const IsoA3Extra: Self = Self(10i32);
    pub const IsoA3Rotated: Self = Self(11i32);
    pub const IsoA4: Self = Self(12i32);
    pub const IsoA4Extra: Self = Self(13i32);
    pub const IsoA4Rotated: Self = Self(14i32);
    pub const IsoA5: Self = Self(15i32);
    pub const IsoA5Extra: Self = Self(16i32);
    pub const IsoA5Rotated: Self = Self(17i32);
    pub const IsoA6: Self = Self(18i32);
    pub const IsoA6Rotated: Self = Self(19i32);
    pub const IsoA7: Self = Self(20i32);
    pub const IsoA8: Self = Self(21i32);
    pub const IsoA9: Self = Self(22i32);
    pub const IsoB0: Self = Self(23i32);
    pub const IsoB1: Self = Self(24i32);
    pub const IsoB10: Self = Self(25i32);
    pub const IsoB2: Self = Self(26i32);
    pub const IsoB3: Self = Self(27i32);
    pub const IsoB4: Self = Self(28i32);
    pub const IsoB4Envelope: Self = Self(29i32);
    pub const IsoB5Envelope: Self = Self(30i32);
    pub const IsoB5Extra: Self = Self(31i32);
    pub const IsoB7: Self = Self(32i32);
    pub const IsoB8: Self = Self(33i32);
    pub const IsoB9: Self = Self(34i32);
    pub const IsoC0: Self = Self(35i32);
    pub const IsoC1: Self = Self(36i32);
    pub const IsoC10: Self = Self(37i32);
    pub const IsoC2: Self = Self(38i32);
    pub const IsoC3: Self = Self(39i32);
    pub const IsoC3Envelope: Self = Self(40i32);
    pub const IsoC4: Self = Self(41i32);
    pub const IsoC4Envelope: Self = Self(42i32);
    pub const IsoC5: Self = Self(43i32);
    pub const IsoC5Envelope: Self = Self(44i32);
    pub const IsoC6: Self = Self(45i32);
    pub const IsoC6C5Envelope: Self = Self(46i32);
    pub const IsoC6Envelope: Self = Self(47i32);
    pub const IsoC7: Self = Self(48i32);
    pub const IsoC8: Self = Self(49i32);
    pub const IsoC9: Self = Self(50i32);
    pub const IsoDLEnvelope: Self = Self(51i32);
    pub const IsoDLEnvelopeRotated: Self = Self(52i32);
    pub const IsoSRA3: Self = Self(53i32);
    pub const Japan2LPhoto: Self = Self(54i32);
    pub const JapanChou3Envelope: Self = Self(55i32);
    pub const JapanChou3EnvelopeRotated: Self = Self(56i32);
    pub const JapanChou4Envelope: Self = Self(57i32);
    pub const JapanChou4EnvelopeRotated: Self = Self(58i32);
    pub const JapanDoubleHagakiPostcard: Self = Self(59i32);
    pub const JapanDoubleHagakiPostcardRotated: Self = Self(60i32);
    pub const JapanHagakiPostcard: Self = Self(61i32);
    pub const JapanHagakiPostcardRotated: Self = Self(62i32);
    pub const JapanKaku2Envelope: Self = Self(63i32);
    pub const JapanKaku2EnvelopeRotated: Self = Self(64i32);
    pub const JapanKaku3Envelope: Self = Self(65i32);
    pub const JapanKaku3EnvelopeRotated: Self = Self(66i32);
    pub const JapanLPhoto: Self = Self(67i32);
    pub const JapanQuadrupleHagakiPostcard: Self = Self(68i32);
    pub const JapanYou1Envelope: Self = Self(69i32);
    pub const JapanYou2Envelope: Self = Self(70i32);
    pub const JapanYou3Envelope: Self = Self(71i32);
    pub const JapanYou4Envelope: Self = Self(72i32);
    pub const JapanYou4EnvelopeRotated: Self = Self(73i32);
    pub const JapanYou6Envelope: Self = Self(74i32);
    pub const JapanYou6EnvelopeRotated: Self = Self(75i32);
    pub const JisB0: Self = Self(76i32);
    pub const JisB1: Self = Self(77i32);
    pub const JisB10: Self = Self(78i32);
    pub const JisB2: Self = Self(79i32);
    pub const JisB3: Self = Self(80i32);
    pub const JisB4: Self = Self(81i32);
    pub const JisB4Rotated: Self = Self(82i32);
    pub const JisB5: Self = Self(83i32);
    pub const JisB5Rotated: Self = Self(84i32);
    pub const JisB6: Self = Self(85i32);
    pub const JisB6Rotated: Self = Self(86i32);
    pub const JisB7: Self = Self(87i32);
    pub const JisB8: Self = Self(88i32);
    pub const JisB9: Self = Self(89i32);
    pub const NorthAmerica10x11: Self = Self(90i32);
    pub const NorthAmerica10x12: Self = Self(91i32);
    pub const NorthAmerica10x14: Self = Self(92i32);
    pub const NorthAmerica11x17: Self = Self(93i32);
    pub const NorthAmerica14x17: Self = Self(94i32);
    pub const NorthAmerica4x6: Self = Self(95i32);
    pub const NorthAmerica4x8: Self = Self(96i32);
    pub const NorthAmerica5x7: Self = Self(97i32);
    pub const NorthAmerica8x10: Self = Self(98i32);
    pub const NorthAmerica9x11: Self = Self(99i32);
    pub const NorthAmericaArchitectureASheet: Self = Self(100i32);
    pub const NorthAmericaArchitectureBSheet: Self = Self(101i32);
    pub const NorthAmericaArchitectureCSheet: Self = Self(102i32);
    pub const NorthAmericaArchitectureDSheet: Self = Self(103i32);
    pub const NorthAmericaArchitectureESheet: Self = Self(104i32);
    pub const NorthAmericaCSheet: Self = Self(105i32);
    pub const NorthAmericaDSheet: Self = Self(106i32);
    pub const NorthAmericaESheet: Self = Self(107i32);
    pub const NorthAmericaExecutive: Self = Self(108i32);
    pub const NorthAmericaGermanLegalFanfold: Self = Self(109i32);
    pub const NorthAmericaGermanStandardFanfold: Self = Self(110i32);
    pub const NorthAmericaLegal: Self = Self(111i32);
    pub const NorthAmericaLegalExtra: Self = Self(112i32);
    pub const NorthAmericaLetter: Self = Self(113i32);
    pub const NorthAmericaLetterExtra: Self = Self(114i32);
    pub const NorthAmericaLetterPlus: Self = Self(115i32);
    pub const NorthAmericaLetterRotated: Self = Self(116i32);
    pub const NorthAmericaMonarchEnvelope: Self = Self(117i32);
    pub const NorthAmericaNote: Self = Self(118i32);
    pub const NorthAmericaNumber10Envelope: Self = Self(119i32);
    pub const NorthAmericaNumber10EnvelopeRotated: Self = Self(120i32);
    pub const NorthAmericaNumber11Envelope: Self = Self(121i32);
    pub const NorthAmericaNumber12Envelope: Self = Self(122i32);
    pub const NorthAmericaNumber14Envelope: Self = Self(123i32);
    pub const NorthAmericaNumber9Envelope: Self = Self(124i32);
    pub const NorthAmericaPersonalEnvelope: Self = Self(125i32);
    pub const NorthAmericaQuarto: Self = Self(126i32);
    pub const NorthAmericaStatement: Self = Self(127i32);
    pub const NorthAmericaSuperA: Self = Self(128i32);
    pub const NorthAmericaSuperB: Self = Self(129i32);
    pub const NorthAmericaTabloid: Self = Self(130i32);
    pub const NorthAmericaTabloidExtra: Self = Self(131i32);
    pub const OtherMetricA3Plus: Self = Self(132i32);
    pub const OtherMetricA4Plus: Self = Self(133i32);
    pub const OtherMetricFolio: Self = Self(134i32);
    pub const OtherMetricInviteEnvelope: Self = Self(135i32);
    pub const OtherMetricItalianEnvelope: Self = Self(136i32);
    pub const Prc10Envelope: Self = Self(137i32);
    pub const Prc10EnvelopeRotated: Self = Self(138i32);
    pub const Prc16K: Self = Self(139i32);
    pub const Prc16KRotated: Self = Self(140i32);
    pub const Prc1Envelope: Self = Self(141i32);
    pub const Prc1EnvelopeRotated: Self = Self(142i32);
    pub const Prc2Envelope: Self = Self(143i32);
    pub const Prc2EnvelopeRotated: Self = Self(144i32);
    pub const Prc32K: Self = Self(145i32);
    pub const Prc32KBig: Self = Self(146i32);
    pub const Prc32KRotated: Self = Self(147i32);
    pub const Prc3Envelope: Self = Self(148i32);
    pub const Prc3EnvelopeRotated: Self = Self(149i32);
    pub const Prc4Envelope: Self = Self(150i32);
    pub const Prc4EnvelopeRotated: Self = Self(151i32);
    pub const Prc5Envelope: Self = Self(152i32);
    pub const Prc5EnvelopeRotated: Self = Self(153i32);
    pub const Prc6Envelope: Self = Self(154i32);
    pub const Prc6EnvelopeRotated: Self = Self(155i32);
    pub const Prc7Envelope: Self = Self(156i32);
    pub const Prc7EnvelopeRotated: Self = Self(157i32);
    pub const Prc8Envelope: Self = Self(158i32);
    pub const Prc8EnvelopeRotated: Self = Self(159i32);
    pub const Prc9Envelope: Self = Self(160i32);
    pub const Prc9EnvelopeRotated: Self = Self(161i32);
    pub const Roll04Inch: Self = Self(162i32);
    pub const Roll06Inch: Self = Self(163i32);
    pub const Roll08Inch: Self = Self(164i32);
    pub const Roll12Inch: Self = Self(165i32);
    pub const Roll15Inch: Self = Self(166i32);
    pub const Roll18Inch: Self = Self(167i32);
    pub const Roll22Inch: Self = Self(168i32);
    pub const Roll24Inch: Self = Self(169i32);
    pub const Roll30Inch: Self = Self(170i32);
    pub const Roll36Inch: Self = Self(171i32);
    pub const Roll54Inch: Self = Self(172i32);
}
impl windows_core::TypeKind for PrintMediaSize {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintMediaSize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintMediaSize").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintMediaSize {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintMediaSize;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintMediaType(pub i32);
impl PrintMediaType {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const AutoSelect: Self = Self(3i32);
    pub const Archival: Self = Self(4i32);
    pub const BackPrintFilm: Self = Self(5i32);
    pub const Bond: Self = Self(6i32);
    pub const CardStock: Self = Self(7i32);
    pub const Continuous: Self = Self(8i32);
    pub const EnvelopePlain: Self = Self(9i32);
    pub const EnvelopeWindow: Self = Self(10i32);
    pub const Fabric: Self = Self(11i32);
    pub const HighResolution: Self = Self(12i32);
    pub const Label: Self = Self(13i32);
    pub const MultiLayerForm: Self = Self(14i32);
    pub const MultiPartForm: Self = Self(15i32);
    pub const Photographic: Self = Self(16i32);
    pub const PhotographicFilm: Self = Self(17i32);
    pub const PhotographicGlossy: Self = Self(18i32);
    pub const PhotographicHighGloss: Self = Self(19i32);
    pub const PhotographicMatte: Self = Self(20i32);
    pub const PhotographicSatin: Self = Self(21i32);
    pub const PhotographicSemiGloss: Self = Self(22i32);
    pub const Plain: Self = Self(23i32);
    pub const Screen: Self = Self(24i32);
    pub const ScreenPaged: Self = Self(25i32);
    pub const Stationery: Self = Self(26i32);
    pub const TabStockFull: Self = Self(27i32);
    pub const TabStockPreCut: Self = Self(28i32);
    pub const Transparency: Self = Self(29i32);
    pub const TShirtTransfer: Self = Self(30i32);
    pub const None: Self = Self(31i32);
}
impl windows_core::TypeKind for PrintMediaType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintMediaType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintMediaType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintMediaType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintMediaType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintOrientation(pub i32);
impl PrintOrientation {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Portrait: Self = Self(3i32);
    pub const PortraitFlipped: Self = Self(4i32);
    pub const Landscape: Self = Self(5i32);
    pub const LandscapeFlipped: Self = Self(6i32);
}
impl windows_core::TypeKind for PrintOrientation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintOrientation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintOrientation").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintOrientation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintOrientation;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintQuality(pub i32);
impl PrintQuality {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Automatic: Self = Self(3i32);
    pub const Draft: Self = Self(4i32);
    pub const Fax: Self = Self(5i32);
    pub const High: Self = Self(6i32);
    pub const Normal: Self = Self(7i32);
    pub const Photographic: Self = Self(8i32);
    pub const Text: Self = Self(9i32);
}
impl windows_core::TypeKind for PrintQuality {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintQuality {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintQuality").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintQuality {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintQuality;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintStaple(pub i32);
impl PrintStaple {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const None: Self = Self(3i32);
    pub const StapleTopLeft: Self = Self(4i32);
    pub const StapleTopRight: Self = Self(5i32);
    pub const StapleBottomLeft: Self = Self(6i32);
    pub const StapleBottomRight: Self = Self(7i32);
    pub const StapleDualLeft: Self = Self(8i32);
    pub const StapleDualRight: Self = Self(9i32);
    pub const StapleDualTop: Self = Self(10i32);
    pub const StapleDualBottom: Self = Self(11i32);
    pub const SaddleStitch: Self = Self(12i32);
}
impl windows_core::TypeKind for PrintStaple {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintStaple {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintStaple").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintStaple {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintStaple;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintTaskCompletion(pub i32);
impl PrintTaskCompletion {
    pub const Abandoned: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
    pub const Submitted: Self = Self(3i32);
}
impl windows_core::TypeKind for PrintTaskCompletion {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintTaskCompletion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintTaskCompletion").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintTaskCompletion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintTaskCompletion;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrintPageDescription {
    pub PageSize: super::super::Foundation::Size,
    pub ImageableRect: super::super::Foundation::Rect,
    pub DpiX: u32,
    pub DpiY: u32,
}
impl windows_core::TypeKind for PrintPageDescription {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PrintPageDescription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.Printing.PrintPageDescription;struct(Windows.Foundation.Size;f4;f4);struct(Windows.Foundation.Rect;f4;f4;f4;f4);u4;u4)");
}
impl Default for PrintPageDescription {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(PrintTaskSourceRequestedHandler, PrintTaskSourceRequestedHandler_Vtbl, 0x6c109fa8_5cb6_4b3a_8663_f39cb02dc9b4);
impl PrintTaskSourceRequestedHandler {
    pub fn new<F: FnMut(Option<&PrintTaskSourceRequestedArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = PrintTaskSourceRequestedHandlerBox::<F> { vtable: &PrintTaskSourceRequestedHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PrintTaskSourceRequestedArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
}
#[repr(C)]
struct PrintTaskSourceRequestedHandlerBox<F: FnMut(Option<&PrintTaskSourceRequestedArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const PrintTaskSourceRequestedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&PrintTaskSourceRequestedArgs>) -> windows_core::Result<()> + Send + 'static> PrintTaskSourceRequestedHandlerBox<F> {
    const VTABLE: PrintTaskSourceRequestedHandler_Vtbl = PrintTaskSourceRequestedHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <PrintTaskSourceRequestedHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&args)).into()
    }
}
impl windows_core::RuntimeType for PrintTaskSourceRequestedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct PrintTaskSourceRequestedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
