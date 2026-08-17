windows_core::imp::define_interface!(IXsltProcessor, IXsltProcessor_Vtbl, 0x7b64703f_550c_48c6_a90f_93a5b964518f);
impl windows_core::RuntimeType for IXsltProcessor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXsltProcessor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub TransformToString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    TransformToString: usize,
}
windows_core::imp::define_interface!(IXsltProcessor2, IXsltProcessor2_Vtbl, 0x8da45c56_97a5_44cb_a8be_27d86280c70a);
impl windows_core::RuntimeType for IXsltProcessor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXsltProcessor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub TransformToDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    TransformToDocument: usize,
}
windows_core::imp::define_interface!(IXsltProcessorFactory, IXsltProcessorFactory_Vtbl, 0x274146c0_9a51_4663_bf30_0ef742146f20);
impl windows_core::RuntimeType for IXsltProcessorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXsltProcessorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateInstance: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XsltProcessor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XsltProcessor, windows_core::IUnknown, windows_core::IInspectable);
impl XsltProcessor {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn TransformToString<P0>(&self, inputnode: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::Dom::IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformToString)(windows_core::Interface::as_raw(this), inputnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn TransformToDocument<P0>(&self, inputnode: P0) -> windows_core::Result<super::Dom::XmlDocument>
    where
        P0: windows_core::Param<super::Dom::IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXsltProcessor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformToDocument)(windows_core::Interface::as_raw(this), inputnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateInstance<P0>(document: P0) -> windows_core::Result<XsltProcessor>
    where
        P0: windows_core::Param<super::Dom::XmlDocument>,
    {
        Self::IXsltProcessorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), document.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IXsltProcessorFactory<R, F: FnOnce(&IXsltProcessorFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<XsltProcessor, IXsltProcessorFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for XsltProcessor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXsltProcessor>();
}
unsafe impl windows_core::Interface for XsltProcessor {
    type Vtable = IXsltProcessor_Vtbl;
    const IID: windows_core::GUID = <IXsltProcessor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XsltProcessor {
    const NAME: &'static str = "Windows.Data.Xml.Xsl.XsltProcessor";
}
unsafe impl Send for XsltProcessor {}
unsafe impl Sync for XsltProcessor {}
