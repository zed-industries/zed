windows_core::imp::define_interface!(IPackageExtension, IPackageExtension_Vtbl, 0xda70c957_7ead_5c3b_9cf0_cc43faf474b4);
impl windows_core::RuntimeType for IPackageExtension {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageExtension_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetExtensionProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetExtensionProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetExtensionPropertiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetExtensionPropertiesAsync: usize,
    pub GetPublicPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub GetPublicFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    GetPublicFolder: usize,
    #[cfg(feature = "Storage")]
    pub GetPublicFolderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    GetPublicFolderAsync: usize,
}
windows_core::imp::define_interface!(IPackageExtensionCatalog, IPackageExtensionCatalog_Vtbl, 0x0879dfe6_ac30_58b2_97f9_480b07e75bfa);
impl windows_core::RuntimeType for IPackageExtensionCatalog {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageExtensionCatalog_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAll: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    pub RequestRemovePackageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PackageInstalled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePackageInstalled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PackageUpdating: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePackageUpdating: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PackageUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePackageUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PackageUninstalling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePackageUninstalling: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PackageStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePackageStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageExtensionCatalogStatics, IPackageExtensionCatalogStatics_Vtbl, 0x9588ece4_3183_5684_9e5f_27759733ddfe);
impl windows_core::RuntimeType for IPackageExtensionCatalogStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageExtensionCatalogStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageExtensionPackageInstalledEventArgs, IPackageExtensionPackageInstalledEventArgs_Vtbl, 0x3c9b0067_083c_5fe3_bdfb_9feb156b4118);
impl windows_core::RuntimeType for IPackageExtensionPackageInstalledEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageExtensionPackageInstalledEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PackageExtensionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Extensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Extensions: usize,
}
windows_core::imp::define_interface!(IPackageExtensionPackageStatusChangedEventArgs, IPackageExtensionPackageStatusChangedEventArgs_Vtbl, 0xb8fee20a_680d_5942_876c_5de12df1083c);
impl windows_core::RuntimeType for IPackageExtensionPackageStatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageExtensionPackageStatusChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PackageExtensionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageExtensionPackageUninstallingEventArgs, IPackageExtensionPackageUninstallingEventArgs_Vtbl, 0x3b8e9cb7_c539_554d_bb33_a84c0bfa3f50);
impl windows_core::RuntimeType for IPackageExtensionPackageUninstallingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageExtensionPackageUninstallingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PackageExtensionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageExtensionPackageUpdatedEventArgs, IPackageExtensionPackageUpdatedEventArgs_Vtbl, 0xfdc31add_16a7_509d_8bc4_fde22e856d2d);
impl windows_core::RuntimeType for IPackageExtensionPackageUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageExtensionPackageUpdatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PackageExtensionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Extensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Extensions: usize,
}
windows_core::imp::define_interface!(IPackageExtensionPackageUpdatingEventArgs, IPackageExtensionPackageUpdatingEventArgs_Vtbl, 0x27ae2ce1_a1d3_532e_8e7e_8b43782fce09);
impl windows_core::RuntimeType for IPackageExtensionPackageUpdatingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageExtensionPackageUpdatingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PackageExtensionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PackageExtension(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageExtension, windows_core::IUnknown, windows_core::IInspectable);
impl PackageExtension {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Package(&self) -> windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Package)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetExtensionProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetExtensionProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetExtensionPropertiesAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IPropertySet>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetExtensionPropertiesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPublicPath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPublicPath)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn GetPublicFolder(&self) -> windows_core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPublicFolder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn GetPublicFolderAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPublicFolderAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PackageExtension {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageExtension>();
}
unsafe impl windows_core::Interface for PackageExtension {
    type Vtable = IPackageExtension_Vtbl;
    const IID: windows_core::GUID = <IPackageExtension as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageExtension {
    const NAME: &'static str = "Windows.ApplicationModel.PackageExtensions.PackageExtension";
}
unsafe impl Send for PackageExtension {}
unsafe impl Sync for PackageExtension {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PackageExtensionCatalog(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageExtensionCatalog, windows_core::IUnknown, windows_core::IInspectable);
impl PackageExtensionCatalog {
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAll(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PackageExtension>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAll)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PackageExtension>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestRemovePackageAsync(&self, packagefullname: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestRemovePackageAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefullname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PackageInstalled<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PackageExtensionCatalog, PackageExtensionPackageInstalledEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageInstalled)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePackageInstalled(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePackageInstalled)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PackageUpdating<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PackageExtensionCatalog, PackageExtensionPackageUpdatingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageUpdating)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePackageUpdating(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePackageUpdating)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PackageUpdated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PackageExtensionCatalog, PackageExtensionPackageUpdatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageUpdated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePackageUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePackageUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PackageUninstalling<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PackageExtensionCatalog, PackageExtensionPackageUninstallingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageUninstalling)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePackageUninstalling(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePackageUninstalling)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PackageStatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PackageExtensionCatalog, PackageExtensionPackageStatusChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageStatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePackageStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePackageStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Open(packageextensionname: &windows_core::HSTRING) -> windows_core::Result<PackageExtensionCatalog> {
        Self::IPackageExtensionCatalogStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Open)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packageextensionname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPackageExtensionCatalogStatics<R, F: FnOnce(&IPackageExtensionCatalogStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PackageExtensionCatalog, IPackageExtensionCatalogStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PackageExtensionCatalog {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageExtensionCatalog>();
}
unsafe impl windows_core::Interface for PackageExtensionCatalog {
    type Vtable = IPackageExtensionCatalog_Vtbl;
    const IID: windows_core::GUID = <IPackageExtensionCatalog as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageExtensionCatalog {
    const NAME: &'static str = "Windows.ApplicationModel.PackageExtensions.PackageExtensionCatalog";
}
unsafe impl Send for PackageExtensionCatalog {}
unsafe impl Sync for PackageExtensionCatalog {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PackageExtensionPackageInstalledEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageExtensionPackageInstalledEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PackageExtensionPackageInstalledEventArgs {
    pub fn PackageExtensionName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageExtensionName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Package(&self) -> windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Package)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Extensions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PackageExtension>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Extensions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PackageExtensionPackageInstalledEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageExtensionPackageInstalledEventArgs>();
}
unsafe impl windows_core::Interface for PackageExtensionPackageInstalledEventArgs {
    type Vtable = IPackageExtensionPackageInstalledEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPackageExtensionPackageInstalledEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageExtensionPackageInstalledEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageInstalledEventArgs";
}
unsafe impl Send for PackageExtensionPackageInstalledEventArgs {}
unsafe impl Sync for PackageExtensionPackageInstalledEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PackageExtensionPackageStatusChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageExtensionPackageStatusChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PackageExtensionPackageStatusChangedEventArgs {
    pub fn PackageExtensionName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageExtensionName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Package(&self) -> windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Package)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PackageExtensionPackageStatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageExtensionPackageStatusChangedEventArgs>();
}
unsafe impl windows_core::Interface for PackageExtensionPackageStatusChangedEventArgs {
    type Vtable = IPackageExtensionPackageStatusChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPackageExtensionPackageStatusChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageExtensionPackageStatusChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageStatusChangedEventArgs";
}
unsafe impl Send for PackageExtensionPackageStatusChangedEventArgs {}
unsafe impl Sync for PackageExtensionPackageStatusChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PackageExtensionPackageUninstallingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageExtensionPackageUninstallingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PackageExtensionPackageUninstallingEventArgs {
    pub fn PackageExtensionName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageExtensionName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Package(&self) -> windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Package)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PackageExtensionPackageUninstallingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageExtensionPackageUninstallingEventArgs>();
}
unsafe impl windows_core::Interface for PackageExtensionPackageUninstallingEventArgs {
    type Vtable = IPackageExtensionPackageUninstallingEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPackageExtensionPackageUninstallingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageExtensionPackageUninstallingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUninstallingEventArgs";
}
unsafe impl Send for PackageExtensionPackageUninstallingEventArgs {}
unsafe impl Sync for PackageExtensionPackageUninstallingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PackageExtensionPackageUpdatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageExtensionPackageUpdatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PackageExtensionPackageUpdatedEventArgs {
    pub fn PackageExtensionName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageExtensionName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Package(&self) -> windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Package)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Extensions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PackageExtension>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Extensions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PackageExtensionPackageUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageExtensionPackageUpdatedEventArgs>();
}
unsafe impl windows_core::Interface for PackageExtensionPackageUpdatedEventArgs {
    type Vtable = IPackageExtensionPackageUpdatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPackageExtensionPackageUpdatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageExtensionPackageUpdatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUpdatedEventArgs";
}
unsafe impl Send for PackageExtensionPackageUpdatedEventArgs {}
unsafe impl Sync for PackageExtensionPackageUpdatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PackageExtensionPackageUpdatingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageExtensionPackageUpdatingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PackageExtensionPackageUpdatingEventArgs {
    pub fn PackageExtensionName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageExtensionName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Package(&self) -> windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Package)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PackageExtensionPackageUpdatingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageExtensionPackageUpdatingEventArgs>();
}
unsafe impl windows_core::Interface for PackageExtensionPackageUpdatingEventArgs {
    type Vtable = IPackageExtensionPackageUpdatingEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPackageExtensionPackageUpdatingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageExtensionPackageUpdatingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUpdatingEventArgs";
}
unsafe impl Send for PackageExtensionPackageUpdatingEventArgs {}
unsafe impl Sync for PackageExtensionPackageUpdatingEventArgs {}
