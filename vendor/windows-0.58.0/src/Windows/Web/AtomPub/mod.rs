windows_core::imp::define_interface!(IAtomPubClient, IAtomPubClient_Vtbl, 0x35392c38_cded_4d4c_9637_05f15c1c9406);
impl windows_core::RuntimeType for IAtomPubClient {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAtomPubClient_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Web_Syndication")]
    pub RetrieveServiceDocumentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))]
    RetrieveServiceDocumentAsync: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub RetrieveMediaResourceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "Web_Syndication")))]
    RetrieveMediaResourceAsync: usize,
    #[cfg(feature = "Web_Syndication")]
    pub RetrieveResourceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))]
    RetrieveResourceAsync: usize,
    #[cfg(feature = "Web_Syndication")]
    pub CreateResourceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))]
    CreateResourceAsync: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub CreateMediaResourceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "Web_Syndication")))]
    CreateMediaResourceAsync: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub UpdateMediaResourceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "Web_Syndication")))]
    UpdateMediaResourceAsync: usize,
    #[cfg(feature = "Web_Syndication")]
    pub UpdateResourceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))]
    UpdateResourceAsync: usize,
    #[cfg(feature = "Web_Syndication")]
    pub UpdateResourceItemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))]
    UpdateResourceItemAsync: usize,
    #[cfg(feature = "Web_Syndication")]
    pub DeleteResourceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))]
    DeleteResourceAsync: usize,
    #[cfg(feature = "Web_Syndication")]
    pub DeleteResourceItemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))]
    DeleteResourceItemAsync: usize,
    pub CancelAsyncOperations: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAtomPubClientFactory, IAtomPubClientFactory_Vtbl, 0x49d55012_57cb_4bde_ab9f_2610b172777b);
impl windows_core::RuntimeType for IAtomPubClientFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAtomPubClientFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub CreateAtomPubClientWithCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateAtomPubClientWithCredentials: usize,
}
windows_core::imp::define_interface!(IResourceCollection, IResourceCollection_Vtbl, 0x7f5fd609_bc88_41d4_88fa_3de6704d428e);
impl windows_core::RuntimeType for IResourceCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Web_Syndication")]
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))]
    Title: usize,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub Categories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web_Syndication")))]
    Categories: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Accepts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Accepts: usize,
}
windows_core::imp::define_interface!(IServiceDocument, IServiceDocument_Vtbl, 0x8b7ec771_2ab3_4dbe_8bcc_778f92b75e51);
impl windows_core::RuntimeType for IServiceDocument {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IServiceDocument_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Workspaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Workspaces: usize,
}
windows_core::imp::define_interface!(IWorkspace, IWorkspace_Vtbl, 0xb41da63b_a4b8_4036_89c5_83c31266ba49);
impl windows_core::RuntimeType for IWorkspace {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWorkspace_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Web_Syndication")]
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))]
    Title: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Collections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Collections: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AtomPubClient(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AtomPubClient, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Web_Syndication")]
windows_core::imp::required_hierarchy!(AtomPubClient, super::Syndication::ISyndicationClient);
impl AtomPubClient {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AtomPubClient, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn RetrieveServiceDocumentAsync<P0>(&self, uri: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<ServiceDocument, super::Syndication::RetrievalProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetrieveServiceDocumentAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub fn RetrieveMediaResourceAsync<P0>(&self, uri: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, super::Syndication::RetrievalProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetrieveMediaResourceAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn RetrieveResourceAsync<P0>(&self, uri: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::RetrievalProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetrieveResourceAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn CreateResourceAsync<P0, P1>(&self, uri: P0, description: &windows_core::HSTRING, item: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::Syndication::SyndicationItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateResourceAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), core::mem::transmute_copy(description), item.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub fn CreateMediaResourceAsync<P0, P1>(&self, uri: P0, mediatype: &windows_core::HSTRING, description: &windows_core::HSTRING, mediastream: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMediaResourceAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), core::mem::transmute_copy(mediatype), core::mem::transmute_copy(description), mediastream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub fn UpdateMediaResourceAsync<P0, P1>(&self, uri: P0, mediatype: &windows_core::HSTRING, mediastream: P1) -> windows_core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateMediaResourceAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), core::mem::transmute_copy(mediatype), mediastream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn UpdateResourceAsync<P0, P1>(&self, uri: P0, item: P1) -> windows_core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::Syndication::SyndicationItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateResourceAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), item.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn UpdateResourceItemAsync<P0>(&self, item: P0) -> windows_core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>
    where
        P0: windows_core::Param<super::Syndication::SyndicationItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateResourceItemAsync)(windows_core::Interface::as_raw(this), item.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn DeleteResourceAsync<P0>(&self, uri: P0) -> windows_core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteResourceAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn DeleteResourceItemAsync<P0>(&self, item: P0) -> windows_core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>
    where
        P0: windows_core::Param<super::Syndication::SyndicationItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteResourceItemAsync)(windows_core::Interface::as_raw(this), item.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CancelAsyncOperations(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CancelAsyncOperations)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateAtomPubClientWithCredentials<P0>(servercredential: P0) -> windows_core::Result<AtomPubClient>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        Self::IAtomPubClientFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAtomPubClientWithCredentials)(windows_core::Interface::as_raw(this), servercredential.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    pub fn ServerCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    pub fn SetServerCredential<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetServerCredential)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    pub fn ProxyCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProxyCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    pub fn SetProxyCredential<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProxyCredential)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn MaxResponseBufferSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxResponseBufferSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetMaxResponseBufferSize(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxResponseBufferSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn Timeout(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetTimeout(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTimeout)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn BypassCacheOnRetrieve(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BypassCacheOnRetrieve)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetBypassCacheOnRetrieve(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBypassCacheOnRetrieve)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetRequestHeader(&self, name: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRequestHeader)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn RetrieveFeedAsync<P0>(&self, uri: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationFeed, super::Syndication::RetrievalProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetrieveFeedAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[doc(hidden)]
    pub fn IAtomPubClientFactory<R, F: FnOnce(&IAtomPubClientFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AtomPubClient, IAtomPubClientFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AtomPubClient {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAtomPubClient>();
}
unsafe impl windows_core::Interface for AtomPubClient {
    type Vtable = IAtomPubClient_Vtbl;
    const IID: windows_core::GUID = <IAtomPubClient as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AtomPubClient {
    const NAME: &'static str = "Windows.Web.AtomPub.AtomPubClient";
}
unsafe impl Send for AtomPubClient {}
unsafe impl Sync for AtomPubClient {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ResourceCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ResourceCollection, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Web_Syndication")]
windows_core::imp::required_hierarchy!(ResourceCollection, super::Syndication::ISyndicationNode);
impl ResourceCollection {
    #[cfg(feature = "Web_Syndication")]
    pub fn Title(&self) -> windows_core::Result<super::Syndication::ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn Categories(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::Syndication::SyndicationCategory>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Categories)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Accepts(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Accepts)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeNamespace(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeNamespace)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeNamespace(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeNamespace)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeValue(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn BaseUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetBaseUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBaseUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn AttributeExtensions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttributeExtensions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn ElementExtensions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ElementExtensions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Web_Syndication"))]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXmlDocument)(windows_core::Interface::as_raw(this), format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ResourceCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IResourceCollection>();
}
unsafe impl windows_core::Interface for ResourceCollection {
    type Vtable = IResourceCollection_Vtbl;
    const IID: windows_core::GUID = <IResourceCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ResourceCollection {
    const NAME: &'static str = "Windows.Web.AtomPub.ResourceCollection";
}
unsafe impl Send for ResourceCollection {}
unsafe impl Sync for ResourceCollection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ServiceDocument(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ServiceDocument, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Web_Syndication")]
windows_core::imp::required_hierarchy!(ServiceDocument, super::Syndication::ISyndicationNode);
impl ServiceDocument {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Workspaces(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<Workspace>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Workspaces)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeNamespace(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeNamespace)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeNamespace(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeNamespace)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeValue(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn BaseUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetBaseUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBaseUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn AttributeExtensions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttributeExtensions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn ElementExtensions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ElementExtensions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Web_Syndication"))]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXmlDocument)(windows_core::Interface::as_raw(this), format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ServiceDocument {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IServiceDocument>();
}
unsafe impl windows_core::Interface for ServiceDocument {
    type Vtable = IServiceDocument_Vtbl;
    const IID: windows_core::GUID = <IServiceDocument as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ServiceDocument {
    const NAME: &'static str = "Windows.Web.AtomPub.ServiceDocument";
}
unsafe impl Send for ServiceDocument {}
unsafe impl Sync for ServiceDocument {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Workspace(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Workspace, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Web_Syndication")]
windows_core::imp::required_hierarchy!(Workspace, super::Syndication::ISyndicationNode);
impl Workspace {
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeNamespace(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeNamespace)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeNamespace(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeNamespace)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeValue(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn BaseUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn SetBaseUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBaseUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn AttributeExtensions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttributeExtensions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn ElementExtensions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ElementExtensions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Web_Syndication"))]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXmlDocument)(windows_core::Interface::as_raw(this), format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Syndication")]
    pub fn Title(&self) -> windows_core::Result<super::Syndication::ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Collections(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ResourceCollection>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Collections)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Workspace {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWorkspace>();
}
unsafe impl windows_core::Interface for Workspace {
    type Vtable = IWorkspace_Vtbl;
    const IID: windows_core::GUID = <IWorkspace as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Workspace {
    const NAME: &'static str = "Windows.Web.AtomPub.Workspace";
}
unsafe impl Send for Workspace {}
unsafe impl Sync for Workspace {}
