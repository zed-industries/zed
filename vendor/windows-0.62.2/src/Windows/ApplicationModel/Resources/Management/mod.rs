windows_core::imp::define_interface!(IIndexedResourceCandidate, IIndexedResourceCandidate_Vtbl, 0x0e619ef3_faec_4414_a9d7_54acd5953f29);
impl windows_core::RuntimeType for IIndexedResourceCandidate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndexedResourceCandidate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IndexedResourceType) -> windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Metadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Qualifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ValueAsString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetQualifierValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIndexedResourceQualifier, IIndexedResourceQualifier_Vtbl, 0xdae3bb9b_d304_497f_a168_a340042c8adb);
impl windows_core::RuntimeType for IIndexedResourceQualifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndexedResourceQualifier_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub QualifierName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QualifierValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceIndexer, IResourceIndexer_Vtbl, 0x2d4cf9a5_e32f_4ab2_8748_96350a016da3);
impl windows_core::RuntimeType for IResourceIndexer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceIndexer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IndexFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IndexFileContentsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceIndexerFactory, IResourceIndexerFactory_Vtbl, 0xb8de3f09_31cd_4d97_bd30_8d39f742bc61);
impl windows_core::RuntimeType for IResourceIndexerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceIndexerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateResourceIndexer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceIndexerFactory2, IResourceIndexerFactory2_Vtbl, 0x6040f18d_d5e5_4b60_9201_cd279cbcfed9);
impl windows_core::RuntimeType for IResourceIndexerFactory2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceIndexerFactory2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateResourceIndexerWithExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexedResourceCandidate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(IndexedResourceCandidate, windows_core::IUnknown, windows_core::IInspectable);
impl IndexedResourceCandidate {
    pub fn Type(&self) -> windows_core::Result<IndexedResourceType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Metadata(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Metadata)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Qualifiers(&self) -> windows_core::Result<windows_collections::IVectorView<IndexedResourceQualifier>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Qualifiers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ValueAsString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValueAsString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetQualifierValue(&self, qualifiername: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetQualifierValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(qualifiername), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for IndexedResourceCandidate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIndexedResourceCandidate>();
}
unsafe impl windows_core::Interface for IndexedResourceCandidate {
    type Vtable = <IIndexedResourceCandidate as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IIndexedResourceCandidate as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for IndexedResourceCandidate {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IndexedResourceCandidate";
}
unsafe impl Send for IndexedResourceCandidate {}
unsafe impl Sync for IndexedResourceCandidate {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexedResourceQualifier(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(IndexedResourceQualifier, windows_core::IUnknown, windows_core::IInspectable);
impl IndexedResourceQualifier {
    pub fn QualifierName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QualifierName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn QualifierValue(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QualifierValue)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for IndexedResourceQualifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIndexedResourceQualifier>();
}
unsafe impl windows_core::Interface for IndexedResourceQualifier {
    type Vtable = <IIndexedResourceQualifier as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IIndexedResourceQualifier as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for IndexedResourceQualifier {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IndexedResourceQualifier";
}
unsafe impl Send for IndexedResourceQualifier {}
unsafe impl Sync for IndexedResourceQualifier {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IndexedResourceType(pub i32);
impl IndexedResourceType {
    pub const String: Self = Self(0i32);
    pub const Path: Self = Self(1i32);
    pub const EmbeddedData: Self = Self(2i32);
}
impl windows_core::TypeKind for IndexedResourceType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for IndexedResourceType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Resources.Management.IndexedResourceType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceIndexer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ResourceIndexer, windows_core::IUnknown, windows_core::IInspectable);
impl ResourceIndexer {
    pub fn IndexFilePath<P0>(&self, filepath: P0) -> windows_core::Result<IndexedResourceCandidate>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexFilePath)(windows_core::Interface::as_raw(this), filepath.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexFileContentsAsync<P0>(&self, file: P0) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<IndexedResourceCandidate>>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexFileContentsAsync)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateResourceIndexer<P0>(projectroot: P0) -> windows_core::Result<ResourceIndexer>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        Self::IResourceIndexerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateResourceIndexer)(windows_core::Interface::as_raw(this), projectroot.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateResourceIndexerWithExtension<P0, P1>(projectroot: P0, extensiondllpath: P1) -> windows_core::Result<ResourceIndexer>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        Self::IResourceIndexerFactory2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateResourceIndexerWithExtension)(windows_core::Interface::as_raw(this), projectroot.param().abi(), extensiondllpath.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IResourceIndexerFactory<R, F: FnOnce(&IResourceIndexerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ResourceIndexer, IResourceIndexerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IResourceIndexerFactory2<R, F: FnOnce(&IResourceIndexerFactory2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ResourceIndexer, IResourceIndexerFactory2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ResourceIndexer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IResourceIndexer>();
}
unsafe impl windows_core::Interface for ResourceIndexer {
    type Vtable = <IResourceIndexer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IResourceIndexer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ResourceIndexer {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.ResourceIndexer";
}
unsafe impl Send for ResourceIndexer {}
unsafe impl Sync for ResourceIndexer {}
