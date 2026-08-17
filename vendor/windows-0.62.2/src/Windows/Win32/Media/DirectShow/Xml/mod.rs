pub const CLSID_XMLGraphBuilder: windows_core::GUID = windows_core::GUID::from_u128(0x1bb05961_5fbf_11d2_a521_44df07c10000);
windows_core::imp::define_interface!(IXMLGraphBuilder, IXMLGraphBuilder_Vtbl, 0x1bb05960_5fbf_11d2_a521_44df07c10000);
windows_core::imp::interface_hierarchy!(IXMLGraphBuilder, windows_core::IUnknown);
impl IXMLGraphBuilder {
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn BuildFromXML<P0, P1>(&self, pgraph: P0, pxml: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IGraphBuilder>,
        P1: windows_core::Param<super::super::super::Data::Xml::MsXml::IXMLElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).BuildFromXML)(windows_core::Interface::as_raw(self), pgraph.param().abi(), pxml.param().abi()).ok() }
    }
    pub unsafe fn SaveToXML<P0>(&self, pgraph: P0, pbstrxml: *mut windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IGraphBuilder>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveToXML)(windows_core::Interface::as_raw(self), pgraph.param().abi(), core::mem::transmute(pbstrxml)).ok() }
    }
    pub unsafe fn BuildFromXMLFile<P0, P1, P2>(&self, pgraph: P0, wszfilename: P1, wszbaseurl: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IGraphBuilder>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).BuildFromXMLFile)(windows_core::Interface::as_raw(self), pgraph.param().abi(), wszfilename.param().abi(), wszbaseurl.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXMLGraphBuilder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub BuildFromXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    BuildFromXML: usize,
    pub SaveToXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BuildFromXMLFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
pub trait IXMLGraphBuilder_Impl: windows_core::IUnknownImpl {
    fn BuildFromXML(&self, pgraph: windows_core::Ref<super::IGraphBuilder>, pxml: windows_core::Ref<super::super::super::Data::Xml::MsXml::IXMLElement>) -> windows_core::Result<()>;
    fn SaveToXML(&self, pgraph: windows_core::Ref<super::IGraphBuilder>, pbstrxml: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn BuildFromXMLFile(&self, pgraph: windows_core::Ref<super::IGraphBuilder>, wszfilename: &windows_core::PCWSTR, wszbaseurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl IXMLGraphBuilder_Vtbl {
    pub const fn new<Identity: IXMLGraphBuilder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BuildFromXML<Identity: IXMLGraphBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgraph: *mut core::ffi::c_void, pxml: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLGraphBuilder_Impl::BuildFromXML(this, core::mem::transmute_copy(&pgraph), core::mem::transmute_copy(&pxml)).into()
            }
        }
        unsafe extern "system" fn SaveToXML<Identity: IXMLGraphBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgraph: *mut core::ffi::c_void, pbstrxml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLGraphBuilder_Impl::SaveToXML(this, core::mem::transmute_copy(&pgraph), core::mem::transmute_copy(&pbstrxml)).into()
            }
        }
        unsafe extern "system" fn BuildFromXMLFile<Identity: IXMLGraphBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgraph: *mut core::ffi::c_void, wszfilename: windows_core::PCWSTR, wszbaseurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLGraphBuilder_Impl::BuildFromXMLFile(this, core::mem::transmute_copy(&pgraph), core::mem::transmute(&wszfilename), core::mem::transmute(&wszbaseurl)).into()
            }
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
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXMLGraphBuilder {}
