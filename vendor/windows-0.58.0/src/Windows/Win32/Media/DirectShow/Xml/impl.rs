#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
pub trait IXMLGraphBuilder_Impl: Sized {
    fn BuildFromXML(&self, pgraph: Option<&super::IGraphBuilder>, pxml: Option<&super::super::super::Data::Xml::MsXml::IXMLElement>) -> windows_core::Result<()>;
    fn SaveToXML(&self, pgraph: Option<&super::IGraphBuilder>, pbstrxml: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn BuildFromXMLFile(&self, pgraph: Option<&super::IGraphBuilder>, wszfilename: &windows_core::PCWSTR, wszbaseurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXMLGraphBuilder {}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl IXMLGraphBuilder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXMLGraphBuilder_Vtbl
    where
        Identity: IXMLGraphBuilder_Impl,
    {
        unsafe extern "system" fn BuildFromXML<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgraph: *mut core::ffi::c_void, pxml: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXMLGraphBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXMLGraphBuilder_Impl::BuildFromXML(this, windows_core::from_raw_borrowed(&pgraph), windows_core::from_raw_borrowed(&pxml)).into()
        }
        unsafe extern "system" fn SaveToXML<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgraph: *mut core::ffi::c_void, pbstrxml: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IXMLGraphBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXMLGraphBuilder_Impl::SaveToXML(this, windows_core::from_raw_borrowed(&pgraph), core::mem::transmute_copy(&pbstrxml)).into()
        }
        unsafe extern "system" fn BuildFromXMLFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgraph: *mut core::ffi::c_void, wszfilename: windows_core::PCWSTR, wszbaseurl: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXMLGraphBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXMLGraphBuilder_Impl::BuildFromXMLFile(this, windows_core::from_raw_borrowed(&pgraph), core::mem::transmute(&wszfilename), core::mem::transmute(&wszbaseurl)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BuildFromXML: BuildFromXML::<Identity, OFFSET>,
            SaveToXML: SaveToXML::<Identity, OFFSET>,
            BuildFromXMLFile: BuildFromXMLFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLGraphBuilder as windows_core::Interface>::IID
    }
}
