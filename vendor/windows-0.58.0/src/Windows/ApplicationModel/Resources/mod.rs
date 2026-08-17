#[cfg(feature = "ApplicationModel_Resources_Core")]
pub mod Core;
#[cfg(feature = "ApplicationModel_Resources_Management")]
pub mod Management;
windows_core::imp::define_interface!(IResourceLoader, IResourceLoader_Vtbl, 0x08524908_16ef_45ad_a602_293637d7e61a);
impl windows_core::RuntimeType for IResourceLoader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceLoader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceLoader2, IResourceLoader2_Vtbl, 0x10eb6ec6_8138_48c1_bc65_e1f14207367c);
impl windows_core::RuntimeType for IResourceLoader2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceLoader2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetStringForUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceLoaderFactory, IResourceLoaderFactory_Vtbl, 0xc33a3603_69dc_4285_a077_d5c0e47ccbe8);
impl windows_core::RuntimeType for IResourceLoaderFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceLoaderFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateResourceLoaderByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceLoaderStatics, IResourceLoaderStatics_Vtbl, 0xbf777ce1_19c8_49c2_953c_47e9227b334e);
impl windows_core::RuntimeType for IResourceLoaderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceLoaderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetStringForReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceLoaderStatics2, IResourceLoaderStatics2_Vtbl, 0x0cc04141_6466_4989_9494_0b82dfc53f1f);
impl windows_core::RuntimeType for IResourceLoaderStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceLoaderStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetForCurrentViewWithName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetForViewIndependentUse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetForViewIndependentUseWithName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceLoaderStatics3, IResourceLoaderStatics3_Vtbl, 0x64609dfb_64ac_491b_8100_0e558d61c1d0);
impl windows_core::RuntimeType for IResourceLoaderStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceLoaderStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub GetForUIContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetForUIContext: usize,
}
windows_core::imp::define_interface!(IResourceLoaderStatics4, IResourceLoaderStatics4_Vtbl, 0x9fb36c32_6c8c_4316_962e_909539b5c259);
impl windows_core::RuntimeType for IResourceLoaderStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceLoaderStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultPriPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ResourceLoader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ResourceLoader, windows_core::IUnknown, windows_core::IInspectable);
impl ResourceLoader {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ResourceLoader, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetString(&self, resource: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetString)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(resource), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetStringForUri<P0>(&self, uri: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IResourceLoader2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStringForUri)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateResourceLoaderByName(name: &windows_core::HSTRING) -> windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateResourceLoaderByName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetStringForReference<P0>(uri: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IResourceLoaderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStringForReference)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForCurrentView() -> windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForCurrentViewWithName(name: &windows_core::HSTRING) -> windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentViewWithName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForViewIndependentUse() -> windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForViewIndependentUse)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForViewIndependentUseWithName(name: &windows_core::HSTRING) -> windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForViewIndependentUseWithName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI")]
    pub fn GetForUIContext<P0>(context: P0) -> windows_core::Result<ResourceLoader>
    where
        P0: windows_core::Param<super::super::UI::UIContext>,
    {
        Self::IResourceLoaderStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUIContext)(windows_core::Interface::as_raw(this), context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultPriPath(packagefullname: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        Self::IResourceLoaderStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultPriPath)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefullname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IResourceLoaderFactory<R, F: FnOnce(&IResourceLoaderFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ResourceLoader, IResourceLoaderFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IResourceLoaderStatics<R, F: FnOnce(&IResourceLoaderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ResourceLoader, IResourceLoaderStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IResourceLoaderStatics2<R, F: FnOnce(&IResourceLoaderStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ResourceLoader, IResourceLoaderStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IResourceLoaderStatics3<R, F: FnOnce(&IResourceLoaderStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ResourceLoader, IResourceLoaderStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IResourceLoaderStatics4<R, F: FnOnce(&IResourceLoaderStatics4) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ResourceLoader, IResourceLoaderStatics4> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ResourceLoader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IResourceLoader>();
}
unsafe impl windows_core::Interface for ResourceLoader {
    type Vtable = IResourceLoader_Vtbl;
    const IID: windows_core::GUID = <IResourceLoader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ResourceLoader {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.ResourceLoader";
}
unsafe impl Send for ResourceLoader {}
unsafe impl Sync for ResourceLoader {}
