windows_core::imp::define_interface!(IPrintTicketCapabilities, IPrintTicketCapabilities_Vtbl, 0x8c45508b_bbdc_4256_a142_2fd615ecb416);
impl windows_core::RuntimeType for IPrintTicketCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTicketCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub DocumentBindingFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentCollateFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentDuplexFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentHolePunchFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentInputBinFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentNUpFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentStapleFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub JobPasscodeFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageBorderlessFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageMediaSizeFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageMediaTypeFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageOrientationFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageOutputColorFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageOutputQualityFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageResolutionFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFeature: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParameterDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTicketFeature, IPrintTicketFeature_Vtbl, 0xe7607d6a_59f5_4103_8858_b97710963d39);
impl windows_core::RuntimeType for IPrintTicketFeature {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTicketFeature_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetOption: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Options: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Options: usize,
    pub GetSelectedOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSelectedOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintTicketFeatureSelectionType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTicketOption, IPrintTicketOption_Vtbl, 0xb086cf90_b367_4e4b_bd48_9c78a0bb31ce);
impl windows_core::RuntimeType for IPrintTicketOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTicketOption_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetPropertyNode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetPropertyNode: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetScoredPropertyNode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetScoredPropertyNode: usize,
    pub GetPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetScoredPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTicketParameterDefinition, IPrintTicketParameterDefinition_Vtbl, 0xd6bab4e4_2962_4c01_b7f3_9a9294eb8335);
impl windows_core::RuntimeType for IPrintTicketParameterDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTicketParameterDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub DataType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintTicketParameterDataType) -> windows_core::HRESULT,
    pub UnitType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RangeMin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RangeMax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTicketParameterInitializer, IPrintTicketParameterInitializer_Vtbl, 0x5e3335bb_a0a5_48b1_9d5c_07116ddc597a);
impl windows_core::RuntimeType for IPrintTicketParameterInitializer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTicketParameterInitializer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTicketValue, IPrintTicketValue_Vtbl, 0x66b30a32_244d_4e22_a98b_bb3cf1f2dd91);
impl windows_core::RuntimeType for IPrintTicketValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrintTicketValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintTicketValueType) -> windows_core::HRESULT,
    pub GetValueAsInteger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetValueAsString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWorkflowPrintTicket, IWorkflowPrintTicket_Vtbl, 0x41d52285_35e8_448e_a8c5_e4b6a2cf826c);
impl windows_core::RuntimeType for IWorkflowPrintTicket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWorkflowPrintTicket_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentBindingFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentCollateFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentDuplexFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentHolePunchFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentInputBinFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentNUpFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentStapleFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub JobPasscodeFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageBorderlessFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageMediaSizeFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageMediaTypeFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageOrientationFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageOutputColorFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageOutputQualityFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageResolutionFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFeature: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyXmlChangedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ValidateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParameterInitializer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetParameterInitializerAsInteger: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetParameterInitializerAsString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MergeAndValidateTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWorkflowPrintTicketValidationResult, IWorkflowPrintTicketValidationResult_Vtbl, 0x0ad1f392_da7b_4a36_bf36_6a99a62e2059);
impl windows_core::RuntimeType for IWorkflowPrintTicketValidationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWorkflowPrintTicketValidationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Validated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintTicketCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTicketCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTicketCapabilities {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn XmlNamespace(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XmlNamespace)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> windows_core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XmlNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentBindingFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentBindingFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentCollateFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentCollateFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentDuplexFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentDuplexFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentHolePunchFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentHolePunchFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentInputBinFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentInputBinFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentNUpFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentNUpFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentStapleFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentStapleFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn JobPasscodeFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JobPasscodeFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PageBorderlessFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageBorderlessFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PageMediaSizeFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageMediaSizeFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PageMediaTypeFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageMediaTypeFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PageOrientationFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageOrientationFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PageOutputColorFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageOutputColorFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PageOutputQualityFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageOutputQualityFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PageResolutionFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageResolutionFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFeature(&self, name: &windows_core::HSTRING, xmlnamespace: &windows_core::HSTRING) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFeature)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(xmlnamespace), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetParameterDefinition(&self, name: &windows_core::HSTRING, xmlnamespace: &windows_core::HSTRING) -> windows_core::Result<PrintTicketParameterDefinition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetParameterDefinition)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(xmlnamespace), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintTicketCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTicketCapabilities>();
}
unsafe impl windows_core::Interface for PrintTicketCapabilities {
    type Vtable = IPrintTicketCapabilities_Vtbl;
    const IID: windows_core::GUID = <IPrintTicketCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTicketCapabilities {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketCapabilities";
}
unsafe impl Send for PrintTicketCapabilities {}
unsafe impl Sync for PrintTicketCapabilities {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintTicketFeature(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTicketFeature, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTicketFeature {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn XmlNamespace(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XmlNamespace)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> windows_core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XmlNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetOption(&self, name: &windows_core::HSTRING, xmlnamespace: &windows_core::HSTRING) -> windows_core::Result<PrintTicketOption> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOption)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(xmlnamespace), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Options(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<PrintTicketOption>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Options)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSelectedOption(&self) -> windows_core::Result<PrintTicketOption> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSelectedOption)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSelectedOption<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PrintTicketOption>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSelectedOption)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SelectionType(&self) -> windows_core::Result<PrintTicketFeatureSelectionType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectionType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PrintTicketFeature {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTicketFeature>();
}
unsafe impl windows_core::Interface for PrintTicketFeature {
    type Vtable = IPrintTicketFeature_Vtbl;
    const IID: windows_core::GUID = <IPrintTicketFeature as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTicketFeature {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketFeature";
}
unsafe impl Send for PrintTicketFeature {}
unsafe impl Sync for PrintTicketFeature {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintTicketOption(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTicketOption, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTicketOption {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn XmlNamespace(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XmlNamespace)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> windows_core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XmlNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetPropertyNode(&self, name: &windows_core::HSTRING, xmlnamespace: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPropertyNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(xmlnamespace), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetScoredPropertyNode(&self, name: &windows_core::HSTRING, xmlnamespace: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetScoredPropertyNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(xmlnamespace), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPropertyValue(&self, name: &windows_core::HSTRING, xmlnamespace: &windows_core::HSTRING) -> windows_core::Result<PrintTicketValue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPropertyValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(xmlnamespace), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetScoredPropertyValue(&self, name: &windows_core::HSTRING, xmlnamespace: &windows_core::HSTRING) -> windows_core::Result<PrintTicketValue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetScoredPropertyValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(xmlnamespace), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintTicketOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTicketOption>();
}
unsafe impl windows_core::Interface for PrintTicketOption {
    type Vtable = IPrintTicketOption_Vtbl;
    const IID: windows_core::GUID = <IPrintTicketOption as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTicketOption {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketOption";
}
unsafe impl Send for PrintTicketOption {}
unsafe impl Sync for PrintTicketOption {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintTicketParameterDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTicketParameterDefinition, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTicketParameterDefinition {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn XmlNamespace(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XmlNamespace)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> windows_core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XmlNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DataType(&self) -> windows_core::Result<PrintTicketParameterDataType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UnitType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnitType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RangeMin(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RangeMin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RangeMax(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RangeMax)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PrintTicketParameterDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTicketParameterDefinition>();
}
unsafe impl windows_core::Interface for PrintTicketParameterDefinition {
    type Vtable = IPrintTicketParameterDefinition_Vtbl;
    const IID: windows_core::GUID = <IPrintTicketParameterDefinition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTicketParameterDefinition {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketParameterDefinition";
}
unsafe impl Send for PrintTicketParameterDefinition {}
unsafe impl Sync for PrintTicketParameterDefinition {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintTicketParameterInitializer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTicketParameterInitializer, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTicketParameterInitializer {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn XmlNamespace(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XmlNamespace)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> windows_core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XmlNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PrintTicketValue>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Value(&self) -> windows_core::Result<PrintTicketValue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintTicketParameterInitializer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTicketParameterInitializer>();
}
unsafe impl windows_core::Interface for PrintTicketParameterInitializer {
    type Vtable = IPrintTicketParameterInitializer_Vtbl;
    const IID: windows_core::GUID = <IPrintTicketParameterInitializer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTicketParameterInitializer {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketParameterInitializer";
}
unsafe impl Send for PrintTicketParameterInitializer {}
unsafe impl Sync for PrintTicketParameterInitializer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrintTicketValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTicketValue, windows_core::IUnknown, windows_core::IInspectable);
impl PrintTicketValue {
    pub fn Type(&self) -> windows_core::Result<PrintTicketValueType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetValueAsInteger(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetValueAsInteger)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetValueAsString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetValueAsString)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintTicketValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTicketValue>();
}
unsafe impl windows_core::Interface for PrintTicketValue {
    type Vtable = IPrintTicketValue_Vtbl;
    const IID: windows_core::GUID = <IPrintTicketValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTicketValue {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketValue";
}
unsafe impl Send for PrintTicketValue {}
unsafe impl Sync for PrintTicketValue {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WorkflowPrintTicket(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WorkflowPrintTicket, windows_core::IUnknown, windows_core::IInspectable);
impl WorkflowPrintTicket {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn XmlNamespace(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XmlNamespace)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> windows_core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XmlNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCapabilities(&self) -> windows_core::Result<PrintTicketCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCapabilities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentBindingFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentBindingFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentCollateFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentCollateFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentDuplexFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentDuplexFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentHolePunchFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentHolePunchFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentInputBinFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentInputBinFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentNUpFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentNUpFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentStapleFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentStapleFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn JobPasscodeFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JobPasscodeFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PageBorderlessFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageBorderlessFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PageMediaSizeFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageMediaSizeFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PageMediaTypeFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageMediaTypeFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PageOrientationFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageOrientationFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PageOutputColorFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageOutputColorFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PageOutputQualityFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageOutputQualityFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PageResolutionFeature(&self) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageResolutionFeature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFeature(&self, name: &windows_core::HSTRING, xmlnamespace: &windows_core::HSTRING) -> windows_core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFeature)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(xmlnamespace), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NotifyXmlChangedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NotifyXmlChangedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ValidateAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<WorkflowPrintTicketValidationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValidateAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetParameterInitializer(&self, name: &windows_core::HSTRING, xmlnamespace: &windows_core::HSTRING) -> windows_core::Result<PrintTicketParameterInitializer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetParameterInitializer)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(xmlnamespace), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetParameterInitializerAsInteger(&self, name: &windows_core::HSTRING, xmlnamespace: &windows_core::HSTRING, integervalue: i32) -> windows_core::Result<PrintTicketParameterInitializer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetParameterInitializerAsInteger)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(xmlnamespace), integervalue, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetParameterInitializerAsString(&self, name: &windows_core::HSTRING, xmlnamespace: &windows_core::HSTRING, stringvalue: &windows_core::HSTRING) -> windows_core::Result<PrintTicketParameterInitializer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetParameterInitializerAsString)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(xmlnamespace), core::mem::transmute_copy(stringvalue), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MergeAndValidateTicket<P0>(&self, deltashematicket: P0) -> windows_core::Result<WorkflowPrintTicket>
    where
        P0: windows_core::Param<WorkflowPrintTicket>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MergeAndValidateTicket)(windows_core::Interface::as_raw(this), deltashematicket.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WorkflowPrintTicket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWorkflowPrintTicket>();
}
unsafe impl windows_core::Interface for WorkflowPrintTicket {
    type Vtable = IWorkflowPrintTicket_Vtbl;
    const IID: windows_core::GUID = <IWorkflowPrintTicket as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WorkflowPrintTicket {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicket";
}
unsafe impl Send for WorkflowPrintTicket {}
unsafe impl Sync for WorkflowPrintTicket {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WorkflowPrintTicketValidationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WorkflowPrintTicketValidationResult, windows_core::IUnknown, windows_core::IInspectable);
impl WorkflowPrintTicketValidationResult {
    pub fn Validated(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Validated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
impl windows_core::RuntimeType for WorkflowPrintTicketValidationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWorkflowPrintTicketValidationResult>();
}
unsafe impl windows_core::Interface for WorkflowPrintTicketValidationResult {
    type Vtable = IWorkflowPrintTicketValidationResult_Vtbl;
    const IID: windows_core::GUID = <IWorkflowPrintTicketValidationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WorkflowPrintTicketValidationResult {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicketValidationResult";
}
unsafe impl Send for WorkflowPrintTicketValidationResult {}
unsafe impl Sync for WorkflowPrintTicketValidationResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintTicketFeatureSelectionType(pub i32);
impl PrintTicketFeatureSelectionType {
    pub const PickOne: Self = Self(0i32);
    pub const PickMany: Self = Self(1i32);
}
impl windows_core::TypeKind for PrintTicketFeatureSelectionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintTicketFeatureSelectionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintTicketFeatureSelectionType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintTicketFeatureSelectionType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintTicket.PrintTicketFeatureSelectionType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintTicketParameterDataType(pub i32);
impl PrintTicketParameterDataType {
    pub const Integer: Self = Self(0i32);
    pub const NumericString: Self = Self(1i32);
    pub const String: Self = Self(2i32);
}
impl windows_core::TypeKind for PrintTicketParameterDataType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintTicketParameterDataType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintTicketParameterDataType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintTicketParameterDataType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintTicket.PrintTicketParameterDataType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintTicketValueType(pub i32);
impl PrintTicketValueType {
    pub const Integer: Self = Self(0i32);
    pub const String: Self = Self(1i32);
    pub const Unknown: Self = Self(2i32);
}
impl windows_core::TypeKind for PrintTicketValueType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintTicketValueType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintTicketValueType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PrintTicketValueType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintTicket.PrintTicketValueType;i4)");
}
