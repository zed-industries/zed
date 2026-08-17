windows_core::imp::define_interface!(IHidBooleanControl, IHidBooleanControl_Vtbl, 0x524df48a_3695_408c_bba2_e2eb5abfbc20);
impl windows_core::RuntimeType for IHidBooleanControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHidBooleanControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub UsageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsActive: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ControlDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHidBooleanControlDescription, IHidBooleanControlDescription_Vtbl, 0x6196e543_29d8_4a2a_8683_849e207bbe31);
impl windows_core::RuntimeType for IHidBooleanControlDescription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHidBooleanControlDescription_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReportId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub ReportType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HidReportType) -> windows_core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub UsageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ParentCollections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ParentCollections: usize,
}
windows_core::imp::define_interface!(IHidBooleanControlDescription2, IHidBooleanControlDescription2_Vtbl, 0xc8eed2ea_8a77_4c36_aa00_5ff0449d3e73);
impl windows_core::RuntimeType for IHidBooleanControlDescription2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHidBooleanControlDescription2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsAbsolute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHidCollection, IHidCollection_Vtbl, 0x7189f5a3_32f1_46e3_befd_44d2663b7e6a);
impl windows_core::RuntimeType for IHidCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHidCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HidCollectionType) -> windows_core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UsageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHidDevice, IHidDevice_Vtbl, 0x5f8a14e7_2200_432e_95da_d09b87d574a8);
impl windows_core::RuntimeType for IHidDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHidDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VendorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub ProductId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub UsageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetInputReportAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInputReportByIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFeatureReportAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFeatureReportByIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateOutputReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateOutputReportById: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFeatureReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFeatureReportById: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendOutputReportAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendFeatureReportAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBooleanControlDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, HidReportType, u16, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBooleanControlDescriptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNumericControlDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, HidReportType, u16, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNumericControlDescriptions: usize,
    pub InputReportReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveInputReportReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHidDeviceStatics, IHidDeviceStatics_Vtbl, 0x9e5981e4_9856_418c_9f73_77de0cd85754);
impl windows_core::RuntimeType for IHidDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHidDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeviceSelectorVidPid: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, u16, u16, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Storage::FileAccessMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    FromIdAsync: usize,
}
windows_core::imp::define_interface!(IHidFeatureReport, IHidFeatureReport_Vtbl, 0x841d9b79_5ae5_46e3_82ef_1fec5c8942f4);
impl windows_core::RuntimeType for IHidFeatureReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHidFeatureReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
    pub GetBooleanControl: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBooleanControlByDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNumericControl: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNumericControlByDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHidInputReport, IHidInputReport_Vtbl, 0xc35d0e50_f7e7_4e8d_b23e_cabbe56b90e9);
impl windows_core::RuntimeType for IHidInputReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHidInputReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ActivatedBooleanControls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ActivatedBooleanControls: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TransitionedBooleanControls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TransitionedBooleanControls: usize,
    pub GetBooleanControl: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBooleanControlByDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNumericControl: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNumericControlByDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHidInputReportReceivedEventArgs, IHidInputReportReceivedEventArgs_Vtbl, 0x7059c5cb_59b2_4dc2_985c_0adc6136fa2d);
impl windows_core::RuntimeType for IHidInputReportReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHidInputReportReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Report: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHidNumericControl, IHidNumericControl_Vtbl, 0xe38a12a5_35a7_4b75_89c8_fb1f28b10823);
impl windows_core::RuntimeType for IHidNumericControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHidNumericControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsGrouped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub UsageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ScaledValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub SetScaledValue: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ControlDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHidNumericControlDescription, IHidNumericControlDescription_Vtbl, 0x638d5e86_1d97_4c75_927f_5ff58ba05e32);
impl windows_core::RuntimeType for IHidNumericControlDescription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHidNumericControlDescription_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReportId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub ReportType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HidReportType) -> windows_core::HRESULT,
    pub ReportSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReportCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub UsageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub LogicalMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LogicalMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PhysicalMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PhysicalMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UnitExponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Unit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsAbsolute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasNull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ParentCollections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ParentCollections: usize,
}
windows_core::imp::define_interface!(IHidOutputReport, IHidOutputReport_Vtbl, 0x62cb2544_c896_4463_93c1_df9db053c450);
impl windows_core::RuntimeType for IHidOutputReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHidOutputReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
    pub GetBooleanControl: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBooleanControlByDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNumericControl: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNumericControlByDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HidBooleanControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HidBooleanControl, windows_core::IUnknown, windows_core::IInspectable);
impl HidBooleanControl {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsagePage(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsagePage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsageId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsageId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsActive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsActive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsActive(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsActive)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ControlDescription(&self) -> windows_core::Result<HidBooleanControlDescription> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ControlDescription)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HidBooleanControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHidBooleanControl>();
}
unsafe impl windows_core::Interface for HidBooleanControl {
    type Vtable = IHidBooleanControl_Vtbl;
    const IID: windows_core::GUID = <IHidBooleanControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HidBooleanControl {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidBooleanControl";
}
unsafe impl Send for HidBooleanControl {}
unsafe impl Sync for HidBooleanControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HidBooleanControlDescription(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HidBooleanControlDescription, windows_core::IUnknown, windows_core::IInspectable);
impl HidBooleanControlDescription {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportType(&self) -> windows_core::Result<HidReportType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsagePage(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsagePage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsageId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsageId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ParentCollections(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<HidCollection>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentCollections)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsAbsolute(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHidBooleanControlDescription2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAbsolute)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for HidBooleanControlDescription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHidBooleanControlDescription>();
}
unsafe impl windows_core::Interface for HidBooleanControlDescription {
    type Vtable = IHidBooleanControlDescription_Vtbl;
    const IID: windows_core::GUID = <IHidBooleanControlDescription as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HidBooleanControlDescription {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription";
}
unsafe impl Send for HidBooleanControlDescription {}
unsafe impl Sync for HidBooleanControlDescription {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HidCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HidCollection, windows_core::IUnknown, windows_core::IInspectable);
impl HidCollection {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Type(&self) -> windows_core::Result<HidCollectionType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsagePage(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsagePage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsageId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsageId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for HidCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHidCollection>();
}
unsafe impl windows_core::Interface for HidCollection {
    type Vtable = IHidCollection_Vtbl;
    const IID: windows_core::GUID = <IHidCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HidCollection {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidCollection";
}
unsafe impl Send for HidCollection {}
unsafe impl Sync for HidCollection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HidDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HidDevice, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HidDevice, super::super::Foundation::IClosable);
impl HidDevice {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn VendorId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VendorId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProductId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProductId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Version(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Version)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsagePage(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsagePage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsageId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsageId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetInputReportAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<HidInputReport>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInputReportAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetInputReportByIdAsync(&self, reportid: u16) -> windows_core::Result<super::super::Foundation::IAsyncOperation<HidInputReport>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInputReportByIdAsync)(windows_core::Interface::as_raw(this), reportid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFeatureReportAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFeatureReportAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFeatureReportByIdAsync(&self, reportid: u16) -> windows_core::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFeatureReportByIdAsync)(windows_core::Interface::as_raw(this), reportid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateOutputReport(&self) -> windows_core::Result<HidOutputReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateOutputReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateOutputReportById(&self, reportid: u16) -> windows_core::Result<HidOutputReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateOutputReportById)(windows_core::Interface::as_raw(this), reportid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFeatureReport(&self) -> windows_core::Result<HidFeatureReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFeatureReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFeatureReportById(&self, reportid: u16) -> windows_core::Result<HidFeatureReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFeatureReportById)(windows_core::Interface::as_raw(this), reportid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SendOutputReportAsync<P0>(&self, outputreport: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>
    where
        P0: windows_core::Param<HidOutputReport>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendOutputReportAsync)(windows_core::Interface::as_raw(this), outputreport.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SendFeatureReportAsync<P0>(&self, featurereport: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>
    where
        P0: windows_core::Param<HidFeatureReport>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendFeatureReportAsync)(windows_core::Interface::as_raw(this), featurereport.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBooleanControlDescriptions(&self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControlDescription>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBooleanControlDescriptions)(windows_core::Interface::as_raw(this), reporttype, usagepage, usageid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNumericControlDescriptions(&self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<HidNumericControlDescription>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNumericControlDescriptions)(windows_core::Interface::as_raw(this), reporttype, usagepage, usageid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InputReportReceived<P0>(&self, reporthandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<HidDevice, HidInputReportReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputReportReceived)(windows_core::Interface::as_raw(this), reporthandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveInputReportReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveInputReportReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDeviceSelector(usagepage: u16, usageid: u16) -> windows_core::Result<windows_core::HSTRING> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), usagepage, usageid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorVidPid(usagepage: u16, usageid: u16, vendorid: u16, productid: u16) -> windows_core::Result<windows_core::HSTRING> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorVidPid)(windows_core::Interface::as_raw(this), usagepage, usageid, vendorid, productid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage")]
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING, accessmode: super::super::Storage::FileAccessMode) -> windows_core::Result<super::super::Foundation::IAsyncOperation<HidDevice>> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), accessmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IHidDeviceStatics<R, F: FnOnce(&IHidDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HidDevice, IHidDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HidDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHidDevice>();
}
unsafe impl windows_core::Interface for HidDevice {
    type Vtable = IHidDevice_Vtbl;
    const IID: windows_core::GUID = <IHidDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HidDevice {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidDevice";
}
unsafe impl Send for HidDevice {}
unsafe impl Sync for HidDevice {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HidFeatureReport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HidFeatureReport, windows_core::IUnknown, windows_core::IInspectable);
impl HidFeatureReport {
    pub fn Id(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetData)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> windows_core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBooleanControl)(windows_core::Interface::as_raw(this), usagepage, usageid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetBooleanControlByDescription<P0>(&self, controldescription: P0) -> windows_core::Result<HidBooleanControl>
    where
        P0: windows_core::Param<HidBooleanControlDescription>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBooleanControlByDescription)(windows_core::Interface::as_raw(this), controldescription.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> windows_core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNumericControl)(windows_core::Interface::as_raw(this), usagepage, usageid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNumericControlByDescription<P0>(&self, controldescription: P0) -> windows_core::Result<HidNumericControl>
    where
        P0: windows_core::Param<HidNumericControlDescription>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNumericControlByDescription)(windows_core::Interface::as_raw(this), controldescription.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HidFeatureReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHidFeatureReport>();
}
unsafe impl windows_core::Interface for HidFeatureReport {
    type Vtable = IHidFeatureReport_Vtbl;
    const IID: windows_core::GUID = <IHidFeatureReport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HidFeatureReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidFeatureReport";
}
unsafe impl Send for HidFeatureReport {}
unsafe impl Sync for HidFeatureReport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HidInputReport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HidInputReport, windows_core::IUnknown, windows_core::IInspectable);
impl HidInputReport {
    pub fn Id(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ActivatedBooleanControls(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedBooleanControls)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TransitionedBooleanControls(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransitionedBooleanControls)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> windows_core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBooleanControl)(windows_core::Interface::as_raw(this), usagepage, usageid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetBooleanControlByDescription<P0>(&self, controldescription: P0) -> windows_core::Result<HidBooleanControl>
    where
        P0: windows_core::Param<HidBooleanControlDescription>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBooleanControlByDescription)(windows_core::Interface::as_raw(this), controldescription.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> windows_core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNumericControl)(windows_core::Interface::as_raw(this), usagepage, usageid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNumericControlByDescription<P0>(&self, controldescription: P0) -> windows_core::Result<HidNumericControl>
    where
        P0: windows_core::Param<HidNumericControlDescription>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNumericControlByDescription)(windows_core::Interface::as_raw(this), controldescription.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HidInputReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHidInputReport>();
}
unsafe impl windows_core::Interface for HidInputReport {
    type Vtable = IHidInputReport_Vtbl;
    const IID: windows_core::GUID = <IHidInputReport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HidInputReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidInputReport";
}
unsafe impl Send for HidInputReport {}
unsafe impl Sync for HidInputReport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HidInputReportReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HidInputReportReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl HidInputReportReceivedEventArgs {
    pub fn Report(&self) -> windows_core::Result<HidInputReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Report)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HidInputReportReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHidInputReportReceivedEventArgs>();
}
unsafe impl windows_core::Interface for HidInputReportReceivedEventArgs {
    type Vtable = IHidInputReportReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IHidInputReportReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HidInputReportReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs";
}
unsafe impl Send for HidInputReportReceivedEventArgs {}
unsafe impl Sync for HidInputReportReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HidNumericControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HidNumericControl, windows_core::IUnknown, windows_core::IInspectable);
impl HidNumericControl {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsGrouped(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGrouped)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsagePage(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsagePage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsageId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsageId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Value(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetValue(&self, value: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaledValue(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScaledValue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScaledValue(&self, value: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetScaledValue)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ControlDescription(&self) -> windows_core::Result<HidNumericControlDescription> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ControlDescription)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HidNumericControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHidNumericControl>();
}
unsafe impl windows_core::Interface for HidNumericControl {
    type Vtable = IHidNumericControl_Vtbl;
    const IID: windows_core::GUID = <IHidNumericControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HidNumericControl {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidNumericControl";
}
unsafe impl Send for HidNumericControl {}
unsafe impl Sync for HidNumericControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HidNumericControlDescription(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HidNumericControlDescription, windows_core::IUnknown, windows_core::IInspectable);
impl HidNumericControlDescription {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportType(&self) -> windows_core::Result<HidReportType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportSize(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsagePage(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsagePage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsageId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsageId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LogicalMinimum(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LogicalMinimum)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LogicalMaximum(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LogicalMaximum)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PhysicalMinimum(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhysicalMinimum)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PhysicalMaximum(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhysicalMaximum)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UnitExponent(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnitExponent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Unit(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsAbsolute(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAbsolute)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasNull(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasNull)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ParentCollections(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<HidCollection>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentCollections)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HidNumericControlDescription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHidNumericControlDescription>();
}
unsafe impl windows_core::Interface for HidNumericControlDescription {
    type Vtable = IHidNumericControlDescription_Vtbl;
    const IID: windows_core::GUID = <IHidNumericControlDescription as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HidNumericControlDescription {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription";
}
unsafe impl Send for HidNumericControlDescription {}
unsafe impl Sync for HidNumericControlDescription {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HidOutputReport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HidOutputReport, windows_core::IUnknown, windows_core::IInspectable);
impl HidOutputReport {
    pub fn Id(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetData)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> windows_core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBooleanControl)(windows_core::Interface::as_raw(this), usagepage, usageid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetBooleanControlByDescription<P0>(&self, controldescription: P0) -> windows_core::Result<HidBooleanControl>
    where
        P0: windows_core::Param<HidBooleanControlDescription>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBooleanControlByDescription)(windows_core::Interface::as_raw(this), controldescription.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> windows_core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNumericControl)(windows_core::Interface::as_raw(this), usagepage, usageid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNumericControlByDescription<P0>(&self, controldescription: P0) -> windows_core::Result<HidNumericControl>
    where
        P0: windows_core::Param<HidNumericControlDescription>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNumericControlByDescription)(windows_core::Interface::as_raw(this), controldescription.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HidOutputReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHidOutputReport>();
}
unsafe impl windows_core::Interface for HidOutputReport {
    type Vtable = IHidOutputReport_Vtbl;
    const IID: windows_core::GUID = <IHidOutputReport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HidOutputReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidOutputReport";
}
unsafe impl Send for HidOutputReport {}
unsafe impl Sync for HidOutputReport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HidCollectionType(pub i32);
impl HidCollectionType {
    pub const Physical: Self = Self(0i32);
    pub const Application: Self = Self(1i32);
    pub const Logical: Self = Self(2i32);
    pub const Report: Self = Self(3i32);
    pub const NamedArray: Self = Self(4i32);
    pub const UsageSwitch: Self = Self(5i32);
    pub const UsageModifier: Self = Self(6i32);
    pub const Other: Self = Self(7i32);
}
impl windows_core::TypeKind for HidCollectionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HidCollectionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HidCollectionType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HidCollectionType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.HumanInterfaceDevice.HidCollectionType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HidReportType(pub i32);
impl HidReportType {
    pub const Input: Self = Self(0i32);
    pub const Output: Self = Self(1i32);
    pub const Feature: Self = Self(2i32);
}
impl windows_core::TypeKind for HidReportType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HidReportType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HidReportType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HidReportType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.HumanInterfaceDevice.HidReportType;i4)");
}
