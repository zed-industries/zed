#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionDiscovery_Impl: Sized {
    fn GetInstanceCollection(&self, pszcategory: &windows_core::PCWSTR, pszsubcategory: &windows_core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL) -> windows_core::Result<IFunctionInstanceCollection>;
    fn GetInstance(&self, pszfunctioninstanceidentity: &windows_core::PCWSTR) -> windows_core::Result<IFunctionInstance>;
    fn CreateInstanceCollectionQuery(&self, pszcategory: &windows_core::PCWSTR, pszsubcategory: &windows_core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, pifunctiondiscoverynotification: Option<&IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut u64) -> windows_core::Result<IFunctionInstanceCollectionQuery>;
    fn CreateInstanceQuery(&self, pszfunctioninstanceidentity: &windows_core::PCWSTR, pifunctiondiscoverynotification: Option<&IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut u64) -> windows_core::Result<IFunctionInstanceQuery>;
    fn AddInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: &windows_core::PCWSTR, pszsubcategory: &windows_core::PCWSTR, pszcategoryidentity: &windows_core::PCWSTR) -> windows_core::Result<IFunctionInstance>;
    fn RemoveInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: &windows_core::PCWSTR, pszsubcategory: &windows_core::PCWSTR, pszcategoryidentity: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFunctionDiscovery {}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionDiscovery_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFunctionDiscovery_Vtbl
    where
        Identity: IFunctionDiscovery_Impl,
    {
        unsafe extern "system" fn GetInstanceCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcategory: windows_core::PCWSTR, pszsubcategory: windows_core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, ppifunctioninstancecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscovery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionDiscovery_Impl::GetInstanceCollection(this, core::mem::transmute(&pszcategory), core::mem::transmute(&pszsubcategory), core::mem::transmute_copy(&fincludeallsubcategories)) {
                Ok(ok__) => {
                    ppifunctioninstancecollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfunctioninstanceidentity: windows_core::PCWSTR, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscovery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionDiscovery_Impl::GetInstance(this, core::mem::transmute(&pszfunctioninstanceidentity)) {
                Ok(ok__) => {
                    ppifunctioninstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceCollectionQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcategory: windows_core::PCWSTR, pszsubcategory: windows_core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, pifunctiondiscoverynotification: *mut core::ffi::c_void, pfdqcquerycontext: *mut u64, ppifunctioninstancecollectionquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscovery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionDiscovery_Impl::CreateInstanceCollectionQuery(this, core::mem::transmute(&pszcategory), core::mem::transmute(&pszsubcategory), core::mem::transmute_copy(&fincludeallsubcategories), windows_core::from_raw_borrowed(&pifunctiondiscoverynotification), core::mem::transmute_copy(&pfdqcquerycontext)) {
                Ok(ok__) => {
                    ppifunctioninstancecollectionquery.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfunctioninstanceidentity: windows_core::PCWSTR, pifunctiondiscoverynotification: *mut core::ffi::c_void, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscovery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionDiscovery_Impl::CreateInstanceQuery(this, core::mem::transmute(&pszfunctioninstanceidentity), windows_core::from_raw_borrowed(&pifunctiondiscoverynotification), core::mem::transmute_copy(&pfdqcquerycontext)) {
                Ok(ok__) => {
                    ppifunctioninstancequery.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: windows_core::PCWSTR, pszsubcategory: windows_core::PCWSTR, pszcategoryidentity: windows_core::PCWSTR, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscovery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionDiscovery_Impl::AddInstance(this, core::mem::transmute_copy(&enumsystemvisibility), core::mem::transmute(&pszcategory), core::mem::transmute(&pszsubcategory), core::mem::transmute(&pszcategoryidentity)) {
                Ok(ok__) => {
                    ppifunctioninstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: windows_core::PCWSTR, pszsubcategory: windows_core::PCWSTR, pszcategoryidentity: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscovery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFunctionDiscovery_Impl::RemoveInstance(this, core::mem::transmute_copy(&enumsystemvisibility), core::mem::transmute(&pszcategory), core::mem::transmute(&pszsubcategory), core::mem::transmute(&pszcategoryidentity)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInstanceCollection: GetInstanceCollection::<Identity, OFFSET>,
            GetInstance: GetInstance::<Identity, OFFSET>,
            CreateInstanceCollectionQuery: CreateInstanceCollectionQuery::<Identity, OFFSET>,
            CreateInstanceQuery: CreateInstanceQuery::<Identity, OFFSET>,
            AddInstance: AddInstance::<Identity, OFFSET>,
            RemoveInstance: RemoveInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionDiscovery as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionDiscoveryNotification_Impl: Sized {
    fn OnUpdate(&self, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: Option<&IFunctionInstance>) -> windows_core::Result<()>;
    fn OnError(&self, hr: windows_core::HRESULT, fdqcquerycontext: u64, pszprovider: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnEvent(&self, dweventid: u32, fdqcquerycontext: u64, pszprovider: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFunctionDiscoveryNotification {}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionDiscoveryNotification_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFunctionDiscoveryNotification_Vtbl
    where
        Identity: IFunctionDiscoveryNotification_Impl,
    {
        unsafe extern "system" fn OnUpdate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryNotification_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFunctionDiscoveryNotification_Impl::OnUpdate(this, core::mem::transmute_copy(&enumqueryupdateaction), core::mem::transmute_copy(&fdqcquerycontext), windows_core::from_raw_borrowed(&pifunctioninstance)).into()
        }
        unsafe extern "system" fn OnError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, fdqcquerycontext: u64, pszprovider: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryNotification_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFunctionDiscoveryNotification_Impl::OnError(this, core::mem::transmute_copy(&hr), core::mem::transmute_copy(&fdqcquerycontext), core::mem::transmute(&pszprovider)).into()
        }
        unsafe extern "system" fn OnEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweventid: u32, fdqcquerycontext: u64, pszprovider: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryNotification_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFunctionDiscoveryNotification_Impl::OnEvent(this, core::mem::transmute_copy(&dweventid), core::mem::transmute_copy(&fdqcquerycontext), core::mem::transmute(&pszprovider)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnUpdate: OnUpdate::<Identity, OFFSET>,
            OnError: OnError::<Identity, OFFSET>,
            OnEvent: OnEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionDiscoveryNotification as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionDiscoveryProvider_Impl: Sized {
    fn Initialize(&self, pifunctiondiscoveryproviderfactory: Option<&IFunctionDiscoveryProviderFactory>, pifunctiondiscoverynotification: Option<&IFunctionDiscoveryNotification>, lciduserdefault: u32) -> windows_core::Result<u32>;
    fn Query(&self, pifunctiondiscoveryproviderquery: Option<&IFunctionDiscoveryProviderQuery>) -> windows_core::Result<IFunctionInstanceCollection>;
    fn EndQuery(&self) -> windows_core::Result<()>;
    fn InstancePropertyStoreValidateAccess(&self, pifunctioninstance: Option<&IFunctionInstance>, iproviderinstancecontext: isize, dwstgaccess: u32) -> windows_core::Result<()>;
    fn InstancePropertyStoreOpen(&self, pifunctioninstance: Option<&IFunctionInstance>, iproviderinstancecontext: isize, dwstgaccess: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn InstancePropertyStoreFlush(&self, pifunctioninstance: Option<&IFunctionInstance>, iproviderinstancecontext: isize) -> windows_core::Result<()>;
    fn InstanceQueryService(&self, pifunctioninstance: Option<&IFunctionInstance>, iproviderinstancecontext: isize, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn InstanceReleased(&self, pifunctioninstance: Option<&IFunctionInstance>, iproviderinstancecontext: isize) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IFunctionDiscoveryProvider {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionDiscoveryProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFunctionDiscoveryProvider_Vtbl
    where
        Identity: IFunctionDiscoveryProvider_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifunctiondiscoveryproviderfactory: *mut core::ffi::c_void, pifunctiondiscoverynotification: *mut core::ffi::c_void, lciduserdefault: u32, pdwstgaccesscapabilities: *mut u32) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionDiscoveryProvider_Impl::Initialize(this, windows_core::from_raw_borrowed(&pifunctiondiscoveryproviderfactory), windows_core::from_raw_borrowed(&pifunctiondiscoverynotification), core::mem::transmute_copy(&lciduserdefault)) {
                Ok(ok__) => {
                    pdwstgaccesscapabilities.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifunctiondiscoveryproviderquery: *mut core::ffi::c_void, ppifunctioninstancecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionDiscoveryProvider_Impl::Query(this, windows_core::from_raw_borrowed(&pifunctiondiscoveryproviderquery)) {
                Ok(ok__) => {
                    ppifunctioninstancecollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFunctionDiscoveryProvider_Impl::EndQuery(this).into()
        }
        unsafe extern "system" fn InstancePropertyStoreValidateAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifunctioninstance: *mut core::ffi::c_void, iproviderinstancecontext: isize, dwstgaccess: u32) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFunctionDiscoveryProvider_Impl::InstancePropertyStoreValidateAccess(this, windows_core::from_raw_borrowed(&pifunctioninstance), core::mem::transmute_copy(&iproviderinstancecontext), core::mem::transmute_copy(&dwstgaccess)).into()
        }
        unsafe extern "system" fn InstancePropertyStoreOpen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifunctioninstance: *mut core::ffi::c_void, iproviderinstancecontext: isize, dwstgaccess: u32, ppipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionDiscoveryProvider_Impl::InstancePropertyStoreOpen(this, windows_core::from_raw_borrowed(&pifunctioninstance), core::mem::transmute_copy(&iproviderinstancecontext), core::mem::transmute_copy(&dwstgaccess)) {
                Ok(ok__) => {
                    ppipropertystore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancePropertyStoreFlush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifunctioninstance: *mut core::ffi::c_void, iproviderinstancecontext: isize) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFunctionDiscoveryProvider_Impl::InstancePropertyStoreFlush(this, windows_core::from_raw_borrowed(&pifunctioninstance), core::mem::transmute_copy(&iproviderinstancecontext)).into()
        }
        unsafe extern "system" fn InstanceQueryService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifunctioninstance: *mut core::ffi::c_void, iproviderinstancecontext: isize, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppiunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionDiscoveryProvider_Impl::InstanceQueryService(this, windows_core::from_raw_borrowed(&pifunctioninstance), core::mem::transmute_copy(&iproviderinstancecontext), core::mem::transmute_copy(&guidservice), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppiunknown.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceReleased<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifunctioninstance: *mut core::ffi::c_void, iproviderinstancecontext: isize) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFunctionDiscoveryProvider_Impl::InstanceReleased(this, windows_core::from_raw_borrowed(&pifunctioninstance), core::mem::transmute_copy(&iproviderinstancecontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            EndQuery: EndQuery::<Identity, OFFSET>,
            InstancePropertyStoreValidateAccess: InstancePropertyStoreValidateAccess::<Identity, OFFSET>,
            InstancePropertyStoreOpen: InstancePropertyStoreOpen::<Identity, OFFSET>,
            InstancePropertyStoreFlush: InstancePropertyStoreFlush::<Identity, OFFSET>,
            InstanceQueryService: InstanceQueryService::<Identity, OFFSET>,
            InstanceReleased: InstanceReleased::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionDiscoveryProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionDiscoveryProviderFactory_Impl: Sized {
    fn CreatePropertyStore(&self) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn CreateInstance(&self, pszsubcategory: &windows_core::PCWSTR, pszproviderinstanceidentity: &windows_core::PCWSTR, iproviderinstancecontext: isize, pipropertystore: Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>, pifunctiondiscoveryprovider: Option<&IFunctionDiscoveryProvider>) -> windows_core::Result<IFunctionInstance>;
    fn CreateFunctionInstanceCollection(&self) -> windows_core::Result<IFunctionInstanceCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IFunctionDiscoveryProviderFactory {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionDiscoveryProviderFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFunctionDiscoveryProviderFactory_Vtbl
    where
        Identity: IFunctionDiscoveryProviderFactory_Impl,
    {
        unsafe extern "system" fn CreatePropertyStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryProviderFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionDiscoveryProviderFactory_Impl::CreatePropertyStore(this) {
                Ok(ok__) => {
                    ppipropertystore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubcategory: windows_core::PCWSTR, pszproviderinstanceidentity: windows_core::PCWSTR, iproviderinstancecontext: isize, pipropertystore: *mut core::ffi::c_void, pifunctiondiscoveryprovider: *mut core::ffi::c_void, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryProviderFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionDiscoveryProviderFactory_Impl::CreateInstance(this, core::mem::transmute(&pszsubcategory), core::mem::transmute(&pszproviderinstanceidentity), core::mem::transmute_copy(&iproviderinstancecontext), windows_core::from_raw_borrowed(&pipropertystore), windows_core::from_raw_borrowed(&pifunctiondiscoveryprovider)) {
                Ok(ok__) => {
                    ppifunctioninstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFunctionInstanceCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppifunctioninstancecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryProviderFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionDiscoveryProviderFactory_Impl::CreateFunctionInstanceCollection(this) {
                Ok(ok__) => {
                    ppifunctioninstancecollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePropertyStore: CreatePropertyStore::<Identity, OFFSET>,
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            CreateFunctionInstanceCollection: CreateFunctionInstanceCollection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionDiscoveryProviderFactory as windows_core::Interface>::IID
    }
}
pub trait IFunctionDiscoveryProviderQuery_Impl: Sized {
    fn IsInstanceQuery(&self, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> windows_core::Result<()>;
    fn IsSubcategoryQuery(&self, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> windows_core::Result<()>;
    fn GetQueryConstraints(&self) -> windows_core::Result<IProviderQueryConstraintCollection>;
    fn GetPropertyConstraints(&self) -> windows_core::Result<IProviderPropertyConstraintCollection>;
}
impl windows_core::RuntimeName for IFunctionDiscoveryProviderQuery {}
impl IFunctionDiscoveryProviderQuery_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFunctionDiscoveryProviderQuery_Vtbl
    where
        Identity: IFunctionDiscoveryProviderQuery_Impl,
    {
        unsafe extern "system" fn IsInstanceQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryProviderQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFunctionDiscoveryProviderQuery_Impl::IsInstanceQuery(this, core::mem::transmute_copy(&pisinstancequery), core::mem::transmute_copy(&ppszconstraintvalue)).into()
        }
        unsafe extern "system" fn IsSubcategoryQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryProviderQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFunctionDiscoveryProviderQuery_Impl::IsSubcategoryQuery(this, core::mem::transmute_copy(&pissubcategoryquery), core::mem::transmute_copy(&ppszconstraintvalue)).into()
        }
        unsafe extern "system" fn GetQueryConstraints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiproviderqueryconstraints: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryProviderQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionDiscoveryProviderQuery_Impl::GetQueryConstraints(this) {
                Ok(ok__) => {
                    ppiproviderqueryconstraints.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyConstraints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiproviderpropertyconstraints: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryProviderQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionDiscoveryProviderQuery_Impl::GetPropertyConstraints(this) {
                Ok(ok__) => {
                    ppiproviderpropertyconstraints.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsInstanceQuery: IsInstanceQuery::<Identity, OFFSET>,
            IsSubcategoryQuery: IsSubcategoryQuery::<Identity, OFFSET>,
            GetQueryConstraints: GetQueryConstraints::<Identity, OFFSET>,
            GetPropertyConstraints: GetPropertyConstraints::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionDiscoveryProviderQuery as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionDiscoveryServiceProvider_Impl: Sized {
    fn Initialize(&self, pifunctioninstance: Option<&IFunctionInstance>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFunctionDiscoveryServiceProvider {}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionDiscoveryServiceProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFunctionDiscoveryServiceProvider_Vtbl
    where
        Identity: IFunctionDiscoveryServiceProvider_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifunctioninstance: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionDiscoveryServiceProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFunctionDiscoveryServiceProvider_Impl::Initialize(this, windows_core::from_raw_borrowed(&pifunctioninstance), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionDiscoveryServiceProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionInstance_Impl: Sized + super::super::System::Com::IServiceProvider_Impl {
    fn GetID(&self) -> windows_core::Result<*mut u16>;
    fn GetProviderInstanceID(&self) -> windows_core::Result<*mut u16>;
    fn OpenPropertyStore(&self, dwstgaccess: super::super::System::Com::STGM) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn GetCategory(&self, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IFunctionInstance {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionInstance_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFunctionInstance_Vtbl
    where
        Identity: IFunctionInstance_Impl,
    {
        unsafe extern "system" fn GetID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcomemidentity: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: IFunctionInstance_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionInstance_Impl::GetID(this) {
                Ok(ok__) => {
                    ppszcomemidentity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderInstanceID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcomemproviderinstanceidentity: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: IFunctionInstance_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionInstance_Impl::GetProviderInstanceID(this) {
                Ok(ok__) => {
                    ppszcomemproviderinstanceidentity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenPropertyStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstgaccess: super::super::System::Com::STGM, ppipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionInstance_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionInstance_Impl::OpenPropertyStore(this, core::mem::transmute_copy(&dwstgaccess)) {
                Ok(ok__) => {
                    ppipropertystore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: IFunctionInstance_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFunctionInstance_Impl::GetCategory(this, core::mem::transmute_copy(&ppszcomemcategory), core::mem::transmute_copy(&ppszcomemsubcategory)).into()
        }
        Self {
            base__: super::super::System::Com::IServiceProvider_Vtbl::new::<Identity, OFFSET>(),
            GetID: GetID::<Identity, OFFSET>,
            GetProviderInstanceID: GetProviderInstanceID::<Identity, OFFSET>,
            OpenPropertyStore: OpenPropertyStore::<Identity, OFFSET>,
            GetCategory: GetCategory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionInstance as windows_core::Interface>::IID || iid == &<super::super::System::Com::IServiceProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionInstanceCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Get(&self, pszinstanceidentity: &windows_core::PCWSTR, pdwindex: *mut u32) -> windows_core::Result<IFunctionInstance>;
    fn Item(&self, dwindex: u32) -> windows_core::Result<IFunctionInstance>;
    fn Add(&self, pifunctioninstance: Option<&IFunctionInstance>) -> windows_core::Result<()>;
    fn Remove(&self, dwindex: u32) -> windows_core::Result<IFunctionInstance>;
    fn Delete(&self, dwindex: u32) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFunctionInstanceCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionInstanceCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFunctionInstanceCollection_Vtbl
    where
        Identity: IFunctionInstanceCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IFunctionInstanceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionInstanceCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    pdwcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszinstanceidentity: windows_core::PCWSTR, pdwindex: *mut u32, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionInstanceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionInstanceCollection_Impl::Get(this, core::mem::transmute(&pszinstanceidentity), core::mem::transmute_copy(&pdwindex)) {
                Ok(ok__) => {
                    ppifunctioninstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionInstanceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionInstanceCollection_Impl::Item(this, core::mem::transmute_copy(&dwindex)) {
                Ok(ok__) => {
                    ppifunctioninstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifunctioninstance: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionInstanceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFunctionInstanceCollection_Impl::Add(this, windows_core::from_raw_borrowed(&pifunctioninstance)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionInstanceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionInstanceCollection_Impl::Remove(this, core::mem::transmute_copy(&dwindex)) {
                Ok(ok__) => {
                    ppifunctioninstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT
        where
            Identity: IFunctionInstanceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFunctionInstanceCollection_Impl::Delete(this, core::mem::transmute_copy(&dwindex)).into()
        }
        unsafe extern "system" fn DeleteAll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionInstanceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFunctionInstanceCollection_Impl::DeleteAll(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionInstanceCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IFunctionInstanceCollectionQuery_Impl: Sized {
    fn AddQueryConstraint(&self, pszconstraintname: &windows_core::PCWSTR, pszconstraintvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddPropertyConstraint(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const windows_core::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> windows_core::Result<()>;
    fn Execute(&self) -> windows_core::Result<IFunctionInstanceCollection>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IFunctionInstanceCollectionQuery {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IFunctionInstanceCollectionQuery_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFunctionInstanceCollectionQuery_Vtbl
    where
        Identity: IFunctionInstanceCollectionQuery_Impl,
    {
        unsafe extern "system" fn AddQueryConstraint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszconstraintname: windows_core::PCWSTR, pszconstraintvalue: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IFunctionInstanceCollectionQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFunctionInstanceCollectionQuery_Impl::AddQueryConstraint(this, core::mem::transmute(&pszconstraintname), core::mem::transmute(&pszconstraintvalue)).into()
        }
        unsafe extern "system" fn AddPropertyConstraint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, enumpropertyconstraint: PropertyConstraint) -> windows_core::HRESULT
        where
            Identity: IFunctionInstanceCollectionQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFunctionInstanceCollectionQuery_Impl::AddPropertyConstraint(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&enumpropertyconstraint)).into()
        }
        unsafe extern "system" fn Execute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppifunctioninstancecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionInstanceCollectionQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionInstanceCollectionQuery_Impl::Execute(this) {
                Ok(ok__) => {
                    ppifunctioninstancecollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddQueryConstraint: AddQueryConstraint::<Identity, OFFSET>,
            AddPropertyConstraint: AddPropertyConstraint::<Identity, OFFSET>,
            Execute: Execute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionInstanceCollectionQuery as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionInstanceQuery_Impl: Sized {
    fn Execute(&self) -> windows_core::Result<IFunctionInstance>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFunctionInstanceQuery {}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionInstanceQuery_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFunctionInstanceQuery_Vtbl
    where
        Identity: IFunctionInstanceQuery_Impl,
    {
        unsafe extern "system" fn Execute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFunctionInstanceQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFunctionInstanceQuery_Impl::Execute(this) {
                Ok(ok__) => {
                    ppifunctioninstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Execute: Execute::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionInstanceQuery as windows_core::Interface>::IID
    }
}
pub trait IPNPXAssociation_Impl: Sized {
    fn Associate(&self, pszsubcategory: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Unassociate(&self, pszsubcategory: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Delete(&self, pszsubcategory: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPNPXAssociation {}
impl IPNPXAssociation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPNPXAssociation_Vtbl
    where
        Identity: IPNPXAssociation_Impl,
    {
        unsafe extern "system" fn Associate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubcategory: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPNPXAssociation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPNPXAssociation_Impl::Associate(this, core::mem::transmute(&pszsubcategory)).into()
        }
        unsafe extern "system" fn Unassociate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubcategory: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPNPXAssociation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPNPXAssociation_Impl::Unassociate(this, core::mem::transmute(&pszsubcategory)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubcategory: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPNPXAssociation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPNPXAssociation_Impl::Delete(this, core::mem::transmute(&pszsubcategory)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Associate: Associate::<Identity, OFFSET>,
            Unassociate: Unassociate::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPNPXAssociation as windows_core::Interface>::IID
    }
}
pub trait IPNPXDeviceAssociation_Impl: Sized {
    fn Associate(&self, pszsubcategory: &windows_core::PCWSTR, pifunctiondiscoverynotification: Option<&IFunctionDiscoveryNotification>) -> windows_core::Result<()>;
    fn Unassociate(&self, pszsubcategory: &windows_core::PCWSTR, pifunctiondiscoverynotification: Option<&IFunctionDiscoveryNotification>) -> windows_core::Result<()>;
    fn Delete(&self, pszsubcategory: &windows_core::PCWSTR, pifunctiondiscoverynotification: Option<&IFunctionDiscoveryNotification>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPNPXDeviceAssociation {}
impl IPNPXDeviceAssociation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPNPXDeviceAssociation_Vtbl
    where
        Identity: IPNPXDeviceAssociation_Impl,
    {
        unsafe extern "system" fn Associate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubcategory: windows_core::PCWSTR, pifunctiondiscoverynotification: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPNPXDeviceAssociation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPNPXDeviceAssociation_Impl::Associate(this, core::mem::transmute(&pszsubcategory), windows_core::from_raw_borrowed(&pifunctiondiscoverynotification)).into()
        }
        unsafe extern "system" fn Unassociate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubcategory: windows_core::PCWSTR, pifunctiondiscoverynotification: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPNPXDeviceAssociation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPNPXDeviceAssociation_Impl::Unassociate(this, core::mem::transmute(&pszsubcategory), windows_core::from_raw_borrowed(&pifunctiondiscoverynotification)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubcategory: windows_core::PCWSTR, pifunctiondiscoverynotification: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPNPXDeviceAssociation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPNPXDeviceAssociation_Impl::Delete(this, core::mem::transmute(&pszsubcategory), windows_core::from_raw_borrowed(&pifunctiondiscoverynotification)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Associate: Associate::<Identity, OFFSET>,
            Unassociate: Unassociate::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPNPXDeviceAssociation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPropertyStoreCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Get(&self, pszinstanceidentity: &windows_core::PCWSTR, pdwindex: *mut u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Item(&self, dwindex: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Add(&self, pipropertystore: Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
    fn Remove(&self, dwindex: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Delete(&self, dwindex: u32) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IPropertyStoreCollection {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPropertyStoreCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPropertyStoreCollection_Vtbl
    where
        Identity: IPropertyStoreCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPropertyStoreCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyStoreCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    pdwcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszinstanceidentity: windows_core::PCWSTR, pdwindex: *mut u32, ppipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPropertyStoreCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyStoreCollection_Impl::Get(this, core::mem::transmute(&pszinstanceidentity), core::mem::transmute_copy(&pdwindex)) {
                Ok(ok__) => {
                    ppipropertystore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPropertyStoreCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyStoreCollection_Impl::Item(this, core::mem::transmute_copy(&dwindex)) {
                Ok(ok__) => {
                    ppipropertystore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipropertystore: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPropertyStoreCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyStoreCollection_Impl::Add(this, windows_core::from_raw_borrowed(&pipropertystore)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPropertyStoreCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyStoreCollection_Impl::Remove(this, core::mem::transmute_copy(&dwindex)) {
                Ok(ok__) => {
                    pipropertystore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT
        where
            Identity: IPropertyStoreCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyStoreCollection_Impl::Delete(this, core::mem::transmute_copy(&dwindex)).into()
        }
        unsafe extern "system" fn DeleteAll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPropertyStoreCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyStoreCollection_Impl::DeleteAll(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyStoreCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IProviderProperties_Impl: Sized {
    fn GetCount(&self, pifunctioninstance: Option<&IFunctionInstance>, iproviderinstancecontext: isize) -> windows_core::Result<u32>;
    fn GetAt(&self, pifunctioninstance: Option<&IFunctionInstance>, iproviderinstancecontext: isize, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()>;
    fn GetValue(&self, pifunctioninstance: Option<&IFunctionInstance>, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<windows_core::PROPVARIANT>;
    fn SetValue(&self, pifunctioninstance: Option<&IFunctionInstance>, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IProviderProperties {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IProviderProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProviderProperties_Vtbl
    where
        Identity: IProviderProperties_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifunctioninstance: *mut core::ffi::c_void, iproviderinstancecontext: isize, pdwcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IProviderProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProviderProperties_Impl::GetCount(this, windows_core::from_raw_borrowed(&pifunctioninstance), core::mem::transmute_copy(&iproviderinstancecontext)) {
                Ok(ok__) => {
                    pdwcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifunctioninstance: *mut core::ffi::c_void, iproviderinstancecontext: isize, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT
        where
            Identity: IProviderProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProviderProperties_Impl::GetAt(this, windows_core::from_raw_borrowed(&pifunctioninstance), core::mem::transmute_copy(&iproviderinstancecontext), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pkey)).into()
        }
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifunctioninstance: *mut core::ffi::c_void, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IProviderProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProviderProperties_Impl::GetValue(this, windows_core::from_raw_borrowed(&pifunctioninstance), core::mem::transmute_copy(&iproviderinstancecontext), core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    ppropvar.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifunctioninstance: *mut core::ffi::c_void, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IProviderProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProviderProperties_Impl::SetValue(this, windows_core::from_raw_borrowed(&pifunctioninstance), core::mem::transmute_copy(&iproviderinstancecontext), core::mem::transmute_copy(&key), core::mem::transmute_copy(&ppropvar)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProviderProperties as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IProviderPropertyConstraintCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Get(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut windows_core::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> windows_core::Result<()>;
    fn Item(&self, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut windows_core::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> windows_core::Result<()>;
    fn Next(&self, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut windows_core::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IProviderPropertyConstraintCollection {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IProviderPropertyConstraintCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProviderPropertyConstraintCollection_Vtbl
    where
        Identity: IProviderPropertyConstraintCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IProviderPropertyConstraintCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProviderPropertyConstraintCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    pdwcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, pdwpropertyconstraint: *mut u32) -> windows_core::HRESULT
        where
            Identity: IProviderPropertyConstraintCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProviderPropertyConstraintCollection_Impl::Get(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&ppropvar), core::mem::transmute_copy(&pdwpropertyconstraint)).into()
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, pdwpropertyconstraint: *mut u32) -> windows_core::HRESULT
        where
            Identity: IProviderPropertyConstraintCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProviderPropertyConstraintCollection_Impl::Item(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pkey), core::mem::transmute_copy(&ppropvar), core::mem::transmute_copy(&pdwpropertyconstraint)).into()
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, pdwpropertyconstraint: *mut u32) -> windows_core::HRESULT
        where
            Identity: IProviderPropertyConstraintCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProviderPropertyConstraintCollection_Impl::Next(this, core::mem::transmute_copy(&pkey), core::mem::transmute_copy(&ppropvar), core::mem::transmute_copy(&pdwpropertyconstraint)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IProviderPropertyConstraintCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProviderPropertyConstraintCollection_Impl::Skip(this).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IProviderPropertyConstraintCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProviderPropertyConstraintCollection_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProviderPropertyConstraintCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProviderPublishing_Impl: Sized {
    fn CreateInstance(&self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: &windows_core::PCWSTR, pszproviderinstanceidentity: &windows_core::PCWSTR) -> windows_core::Result<IFunctionInstance>;
    fn RemoveInstance(&self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: &windows_core::PCWSTR, pszproviderinstanceidentity: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IProviderPublishing {}
#[cfg(feature = "Win32_System_Com")]
impl IProviderPublishing_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProviderPublishing_Vtbl
    where
        Identity: IProviderPublishing_Impl,
    {
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: windows_core::PCWSTR, pszproviderinstanceidentity: windows_core::PCWSTR, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IProviderPublishing_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProviderPublishing_Impl::CreateInstance(this, core::mem::transmute_copy(&enumvisibilityflags), core::mem::transmute(&pszsubcategory), core::mem::transmute(&pszproviderinstanceidentity)) {
                Ok(ok__) => {
                    ppifunctioninstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: windows_core::PCWSTR, pszproviderinstanceidentity: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IProviderPublishing_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProviderPublishing_Impl::RemoveInstance(this, core::mem::transmute_copy(&enumvisibilityflags), core::mem::transmute(&pszsubcategory), core::mem::transmute(&pszproviderinstanceidentity)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            RemoveInstance: RemoveInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProviderPublishing as windows_core::Interface>::IID
    }
}
pub trait IProviderQueryConstraintCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Get(&self, pszconstraintname: &windows_core::PCWSTR) -> windows_core::Result<*mut u16>;
    fn Item(&self, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> windows_core::Result<()>;
    fn Next(&self, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> windows_core::Result<()>;
    fn Skip(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IProviderQueryConstraintCollection {}
impl IProviderQueryConstraintCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProviderQueryConstraintCollection_Vtbl
    where
        Identity: IProviderQueryConstraintCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IProviderQueryConstraintCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProviderQueryConstraintCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    pdwcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszconstraintname: windows_core::PCWSTR, ppszconstraintvalue: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: IProviderQueryConstraintCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProviderQueryConstraintCollection_Impl::Get(this, core::mem::transmute(&pszconstraintname)) {
                Ok(ok__) => {
                    ppszconstraintvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: IProviderQueryConstraintCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProviderQueryConstraintCollection_Impl::Item(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&ppszconstraintname), core::mem::transmute_copy(&ppszconstraintvalue)).into()
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: IProviderQueryConstraintCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProviderQueryConstraintCollection_Impl::Next(this, core::mem::transmute_copy(&ppszconstraintname), core::mem::transmute_copy(&ppszconstraintvalue)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IProviderQueryConstraintCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProviderQueryConstraintCollection_Impl::Skip(this).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IProviderQueryConstraintCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProviderQueryConstraintCollection_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProviderQueryConstraintCollection as windows_core::Interface>::IID
    }
}
