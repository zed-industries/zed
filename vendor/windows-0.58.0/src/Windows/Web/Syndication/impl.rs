#[cfg(feature = "Security_Credentials")]
pub trait ISyndicationClient_Impl: Sized {
    fn ServerCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetServerCredential(&self, value: Option<&super::super::Security::Credentials::PasswordCredential>) -> windows_core::Result<()>;
    fn ProxyCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetProxyCredential(&self, value: Option<&super::super::Security::Credentials::PasswordCredential>) -> windows_core::Result<()>;
    fn MaxResponseBufferSize(&self) -> windows_core::Result<u32>;
    fn SetMaxResponseBufferSize(&self, value: u32) -> windows_core::Result<()>;
    fn Timeout(&self) -> windows_core::Result<u32>;
    fn SetTimeout(&self, value: u32) -> windows_core::Result<()>;
    fn BypassCacheOnRetrieve(&self) -> windows_core::Result<bool>;
    fn SetBypassCacheOnRetrieve(&self, value: bool) -> windows_core::Result<()>;
    fn SetRequestHeader(&self, name: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn RetrieveFeedAsync(&self, uri: Option<&super::super::Foundation::Uri>) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>>;
}
#[cfg(feature = "Security_Credentials")]
impl windows_core::RuntimeName for ISyndicationClient {
    const NAME: &'static str = "Windows.Web.Syndication.ISyndicationClient";
}
#[cfg(feature = "Security_Credentials")]
impl ISyndicationClient_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyndicationClient_Vtbl
    where
        Identity: ISyndicationClient_Impl,
    {
        unsafe extern "system" fn ServerCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyndicationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyndicationClient_Impl::ServerCredential(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyndicationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyndicationClient_Impl::SetServerCredential(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn ProxyCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyndicationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyndicationClient_Impl::ProxyCredential(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyndicationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyndicationClient_Impl::SetProxyCredential(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn MaxResponseBufferSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyndicationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyndicationClient_Impl::MaxResponseBufferSize(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxResponseBufferSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT
        where
            Identity: ISyndicationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyndicationClient_Impl::SetMaxResponseBufferSize(this, value).into()
        }
        unsafe extern "system" fn Timeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyndicationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyndicationClient_Impl::Timeout(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT
        where
            Identity: ISyndicationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyndicationClient_Impl::SetTimeout(this, value).into()
        }
        unsafe extern "system" fn BypassCacheOnRetrieve<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ISyndicationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyndicationClient_Impl::BypassCacheOnRetrieve(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBypassCacheOnRetrieve<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT
        where
            Identity: ISyndicationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyndicationClient_Impl::SetBypassCacheOnRetrieve(this, value).into()
        }
        unsafe extern "system" fn SetRequestHeader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::HSTRING>, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ISyndicationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyndicationClient_Impl::SetRequestHeader(this, core::mem::transmute(&name), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RetrieveFeedAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyndicationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyndicationClient_Impl::RetrieveFeedAsync(this, windows_core::from_raw_borrowed(&uri)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISyndicationClient, OFFSET>(),
            ServerCredential: ServerCredential::<Identity, OFFSET>,
            SetServerCredential: SetServerCredential::<Identity, OFFSET>,
            ProxyCredential: ProxyCredential::<Identity, OFFSET>,
            SetProxyCredential: SetProxyCredential::<Identity, OFFSET>,
            MaxResponseBufferSize: MaxResponseBufferSize::<Identity, OFFSET>,
            SetMaxResponseBufferSize: SetMaxResponseBufferSize::<Identity, OFFSET>,
            Timeout: Timeout::<Identity, OFFSET>,
            SetTimeout: SetTimeout::<Identity, OFFSET>,
            BypassCacheOnRetrieve: BypassCacheOnRetrieve::<Identity, OFFSET>,
            SetBypassCacheOnRetrieve: SetBypassCacheOnRetrieve::<Identity, OFFSET>,
            SetRequestHeader: SetRequestHeader::<Identity, OFFSET>,
            RetrieveFeedAsync: RetrieveFeedAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyndicationClient as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation_Collections"))]
pub trait ISyndicationNode_Impl: Sized {
    fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetNodeName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn NodeNamespace(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetNodeNamespace(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn NodeValue(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetNodeValue(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Language(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn BaseUri(&self) -> windows_core::Result<super::super::Foundation::Uri>;
    fn SetBaseUri(&self, value: Option<&super::super::Foundation::Uri>) -> windows_core::Result<()>;
    fn AttributeExtensions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>>;
    fn ElementExtensions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>>;
    fn GetXmlDocument(&self, format: SyndicationFormat) -> windows_core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation_Collections"))]
impl windows_core::RuntimeName for ISyndicationNode {
    const NAME: &'static str = "Windows.Web.Syndication.ISyndicationNode";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation_Collections"))]
impl ISyndicationNode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyndicationNode_Vtbl
    where
        Identity: ISyndicationNode_Impl,
    {
        unsafe extern "system" fn NodeName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ISyndicationNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyndicationNode_Impl::NodeName(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNodeName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ISyndicationNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyndicationNode_Impl::SetNodeName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn NodeNamespace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ISyndicationNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyndicationNode_Impl::NodeNamespace(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNodeNamespace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ISyndicationNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyndicationNode_Impl::SetNodeNamespace(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn NodeValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ISyndicationNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyndicationNode_Impl::NodeValue(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNodeValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ISyndicationNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyndicationNode_Impl::SetNodeValue(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Language<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ISyndicationNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyndicationNode_Impl::Language(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ISyndicationNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyndicationNode_Impl::SetLanguage(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn BaseUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyndicationNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyndicationNode_Impl::BaseUri(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBaseUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyndicationNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyndicationNode_Impl::SetBaseUri(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn AttributeExtensions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyndicationNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyndicationNode_Impl::AttributeExtensions(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementExtensions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyndicationNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyndicationNode_Impl::ElementExtensions(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXmlDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: SyndicationFormat, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyndicationNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyndicationNode_Impl::GetXmlDocument(this, format) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISyndicationNode, OFFSET>(),
            NodeName: NodeName::<Identity, OFFSET>,
            SetNodeName: SetNodeName::<Identity, OFFSET>,
            NodeNamespace: NodeNamespace::<Identity, OFFSET>,
            SetNodeNamespace: SetNodeNamespace::<Identity, OFFSET>,
            NodeValue: NodeValue::<Identity, OFFSET>,
            SetNodeValue: SetNodeValue::<Identity, OFFSET>,
            Language: Language::<Identity, OFFSET>,
            SetLanguage: SetLanguage::<Identity, OFFSET>,
            BaseUri: BaseUri::<Identity, OFFSET>,
            SetBaseUri: SetBaseUri::<Identity, OFFSET>,
            AttributeExtensions: AttributeExtensions::<Identity, OFFSET>,
            ElementExtensions: ElementExtensions::<Identity, OFFSET>,
            GetXmlDocument: GetXmlDocument::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyndicationNode as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation_Collections"))]
pub trait ISyndicationText_Impl: Sized + ISyndicationNode_Impl {
    fn Text(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Type(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetType(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Xml(&self) -> windows_core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn SetXml(&self, value: Option<&super::super::Data::Xml::Dom::XmlDocument>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation_Collections"))]
impl windows_core::RuntimeName for ISyndicationText {
    const NAME: &'static str = "Windows.Web.Syndication.ISyndicationText";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation_Collections"))]
impl ISyndicationText_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyndicationText_Vtbl
    where
        Identity: ISyndicationText_Impl,
    {
        unsafe extern "system" fn Text<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ISyndicationText_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyndicationText_Impl::Text(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ISyndicationText_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyndicationText_Impl::SetText(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ISyndicationText_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyndicationText_Impl::Type(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ISyndicationText_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyndicationText_Impl::SetType(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Xml<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyndicationText_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyndicationText_Impl::Xml(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXml<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyndicationText_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyndicationText_Impl::SetXml(this, windows_core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISyndicationText, OFFSET>(),
            Text: Text::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            Xml: Xml::<Identity, OFFSET>,
            SetXml: SetXml::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyndicationText as windows_core::Interface>::IID
    }
}
