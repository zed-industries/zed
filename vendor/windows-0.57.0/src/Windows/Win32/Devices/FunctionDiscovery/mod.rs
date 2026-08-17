windows_core::imp::define_interface!(IFunctionDiscovery, IFunctionDiscovery_Vtbl, 0x4df99b70_e148_4432_b004_4c9eeb535a5e);
impl core::ops::Deref for IFunctionDiscovery {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFunctionDiscovery, windows_core::IUnknown);
impl IFunctionDiscovery {
    pub unsafe fn GetInstanceCollection<P0, P1, P2>(&self, pszcategory: P0, pszsubcategory: P1, fincludeallsubcategories: P2) -> windows_core::Result<IFunctionInstanceCollection>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInstanceCollection)(windows_core::Interface::as_raw(self), pszcategory.param().abi(), pszsubcategory.param().abi(), fincludeallsubcategories.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInstance<P0>(&self, pszfunctioninstanceidentity: P0) -> windows_core::Result<IFunctionInstance>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInstance)(windows_core::Interface::as_raw(self), pszfunctioninstanceidentity.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateInstanceCollectionQuery<P0, P1, P2, P3>(&self, pszcategory: P0, pszsubcategory: P1, fincludeallsubcategories: P2, pifunctiondiscoverynotification: P3, pfdqcquerycontext: *mut u64) -> windows_core::Result<IFunctionInstanceCollectionQuery>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<IFunctionDiscoveryNotification>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInstanceCollectionQuery)(windows_core::Interface::as_raw(self), pszcategory.param().abi(), pszsubcategory.param().abi(), fincludeallsubcategories.param().abi(), pifunctiondiscoverynotification.param().abi(), pfdqcquerycontext, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateInstanceQuery<P0, P1>(&self, pszfunctioninstanceidentity: P0, pifunctiondiscoverynotification: P1, pfdqcquerycontext: *mut u64) -> windows_core::Result<IFunctionInstanceQuery>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IFunctionDiscoveryNotification>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInstanceQuery)(windows_core::Interface::as_raw(self), pszfunctioninstanceidentity.param().abi(), pifunctiondiscoverynotification.param().abi(), pfdqcquerycontext, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddInstance<P0, P1, P2>(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: P0, pszsubcategory: P1, pszcategoryidentity: P2) -> windows_core::Result<IFunctionInstance>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddInstance)(windows_core::Interface::as_raw(self), enumsystemvisibility, pszcategory.param().abi(), pszsubcategory.param().abi(), pszcategoryidentity.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveInstance<P0, P1, P2>(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: P0, pszsubcategory: P1, pszcategoryidentity: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveInstance)(windows_core::Interface::as_raw(self), enumsystemvisibility, pszcategory.param().abi(), pszsubcategory.param().abi(), pszcategoryidentity.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IFunctionDiscovery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInstanceCollection: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInstance: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInstance: usize,
    pub CreateInstanceCollectionQuery: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, super::super::Foundation::BOOL, *mut core::ffi::c_void, *mut u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstanceQuery: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddInstance: unsafe extern "system" fn(*mut core::ffi::c_void, SystemVisibilityFlags, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddInstance: usize,
    pub RemoveInstance: unsafe extern "system" fn(*mut core::ffi::c_void, SystemVisibilityFlags, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFunctionDiscoveryNotification, IFunctionDiscoveryNotification_Vtbl, 0x5f6c1ba8_5330_422e_a368_572b244d3f87);
impl core::ops::Deref for IFunctionDiscoveryNotification {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFunctionDiscoveryNotification, windows_core::IUnknown);
impl IFunctionDiscoveryNotification {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnUpdate<P0>(&self, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFunctionInstance>,
    {
        (windows_core::Interface::vtable(self).OnUpdate)(windows_core::Interface::as_raw(self), enumqueryupdateaction, fdqcquerycontext, pifunctioninstance.param().abi()).ok()
    }
    pub unsafe fn OnError<P0>(&self, hr: windows_core::HRESULT, fdqcquerycontext: u64, pszprovider: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), hr, fdqcquerycontext, pszprovider.param().abi()).ok()
    }
    pub unsafe fn OnEvent<P0>(&self, dweventid: u32, fdqcquerycontext: u64, pszprovider: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnEvent)(windows_core::Interface::as_raw(self), dweventid, fdqcquerycontext, pszprovider.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IFunctionDiscoveryNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, QueryUpdateAction, u64, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnUpdate: usize,
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u64, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u64, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFunctionDiscoveryProvider, IFunctionDiscoveryProvider_Vtbl, 0xdcde394f_1478_4813_a402_f6fb10657222);
impl core::ops::Deref for IFunctionDiscoveryProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFunctionDiscoveryProvider, windows_core::IUnknown);
impl IFunctionDiscoveryProvider {
    pub unsafe fn Initialize<P0, P1>(&self, pifunctiondiscoveryproviderfactory: P0, pifunctiondiscoverynotification: P1, lciduserdefault: u32) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IFunctionDiscoveryProviderFactory>,
        P1: windows_core::Param<IFunctionDiscoveryNotification>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pifunctiondiscoveryproviderfactory.param().abi(), pifunctiondiscoverynotification.param().abi(), lciduserdefault, &mut result__).map(|| result__)
    }
    pub unsafe fn Query<P0>(&self, pifunctiondiscoveryproviderquery: P0) -> windows_core::Result<IFunctionInstanceCollection>
    where
        P0: windows_core::Param<IFunctionDiscoveryProviderQuery>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), pifunctiondiscoveryproviderquery.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndQuery(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndQuery)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancePropertyStoreValidateAccess<P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize, dwstgaccess: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFunctionInstance>,
    {
        (windows_core::Interface::vtable(self).InstancePropertyStoreValidateAccess)(windows_core::Interface::as_raw(self), pifunctioninstance.param().abi(), iproviderinstancecontext, dwstgaccess).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn InstancePropertyStoreOpen<P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize, dwstgaccess: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>
    where
        P0: windows_core::Param<IFunctionInstance>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InstancePropertyStoreOpen)(windows_core::Interface::as_raw(self), pifunctioninstance.param().abi(), iproviderinstancecontext, dwstgaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancePropertyStoreFlush<P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFunctionInstance>,
    {
        (windows_core::Interface::vtable(self).InstancePropertyStoreFlush)(windows_core::Interface::as_raw(self), pifunctioninstance.param().abi(), iproviderinstancecontext).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstanceQueryService<P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<IFunctionInstance>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InstanceQueryService)(windows_core::Interface::as_raw(self), pifunctioninstance.param().abi(), iproviderinstancecontext, guidservice, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstanceReleased<P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFunctionInstance>,
    {
        (windows_core::Interface::vtable(self).InstanceReleased)(windows_core::Interface::as_raw(self), pifunctioninstance.param().abi(), iproviderinstancecontext).ok()
    }
}
#[repr(C)]
pub struct IFunctionDiscoveryProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndQuery: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InstancePropertyStoreValidateAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, isize, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstancePropertyStoreValidateAccess: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub InstancePropertyStoreOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, isize, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    InstancePropertyStoreOpen: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstancePropertyStoreFlush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, isize) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstancePropertyStoreFlush: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstanceQueryService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, isize, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstanceQueryService: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstanceReleased: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, isize) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstanceReleased: usize,
}
windows_core::imp::define_interface!(IFunctionDiscoveryProviderFactory, IFunctionDiscoveryProviderFactory_Vtbl, 0x86443ff0_1ad5_4e68_a45a_40c2c329de3b);
impl core::ops::Deref for IFunctionDiscoveryProviderFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFunctionDiscoveryProviderFactory, windows_core::IUnknown);
impl IFunctionDiscoveryProviderFactory {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CreatePropertyStore(&self) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePropertyStore)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn CreateInstance<P0, P1, P2, P3>(&self, pszsubcategory: P0, pszproviderinstanceidentity: P1, iproviderinstancecontext: isize, pipropertystore: P2, pifunctiondiscoveryprovider: P3) -> windows_core::Result<IFunctionInstance>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
        P3: windows_core::Param<IFunctionDiscoveryProvider>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), pszsubcategory.param().abi(), pszproviderinstanceidentity.param().abi(), iproviderinstancecontext, pipropertystore.param().abi(), pifunctiondiscoveryprovider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFunctionInstanceCollection(&self) -> windows_core::Result<IFunctionInstanceCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFunctionInstanceCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IFunctionDiscoveryProviderFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CreatePropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CreatePropertyStore: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, isize, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    CreateInstance: usize,
    pub CreateFunctionInstanceCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFunctionDiscoveryProviderQuery, IFunctionDiscoveryProviderQuery_Vtbl, 0x6876ea98_baec_46db_bc20_75a76e267a3a);
impl core::ops::Deref for IFunctionDiscoveryProviderQuery {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFunctionDiscoveryProviderQuery, windows_core::IUnknown);
impl IFunctionDiscoveryProviderQuery {
    pub unsafe fn IsInstanceQuery(&self, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsInstanceQuery)(windows_core::Interface::as_raw(self), pisinstancequery, ppszconstraintvalue).ok()
    }
    pub unsafe fn IsSubcategoryQuery(&self, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsSubcategoryQuery)(windows_core::Interface::as_raw(self), pissubcategoryquery, ppszconstraintvalue).ok()
    }
    pub unsafe fn GetQueryConstraints(&self) -> windows_core::Result<IProviderQueryConstraintCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetQueryConstraints)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPropertyConstraints(&self) -> windows_core::Result<IProviderPropertyConstraintCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyConstraints)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IFunctionDiscoveryProviderQuery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsInstanceQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL, *mut *mut u16) -> windows_core::HRESULT,
    pub IsSubcategoryQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL, *mut *mut u16) -> windows_core::HRESULT,
    pub GetQueryConstraints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyConstraints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFunctionDiscoveryServiceProvider, IFunctionDiscoveryServiceProvider_Vtbl, 0x4c81ed02_1b04_43f2_a451_69966cbcd1c2);
impl core::ops::Deref for IFunctionDiscoveryServiceProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFunctionDiscoveryServiceProvider, windows_core::IUnknown);
impl IFunctionDiscoveryServiceProvider {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0, T>(&self, pifunctioninstance: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<IFunctionInstance>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pifunctioninstance.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IFunctionDiscoveryServiceProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFunctionInstance, IFunctionInstance_Vtbl, 0x33591c10_0bed_4f02_b0ab_1530d5533ee9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFunctionInstance {
    type Target = super::super::System::Com::IServiceProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFunctionInstance, windows_core::IUnknown, super::super::System::Com::IServiceProvider);
#[cfg(feature = "Win32_System_Com")]
impl IFunctionInstance {
    pub unsafe fn GetID(&self) -> windows_core::Result<*mut u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProviderInstanceID(&self) -> windows_core::Result<*mut u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProviderInstanceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn OpenPropertyStore(&self, dwstgaccess: super::super::System::Com::STGM) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenPropertyStore)(windows_core::Interface::as_raw(self), dwstgaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCategory(&self, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCategory)(windows_core::Interface::as_raw(self), ppszcomemcategory, ppszcomemsubcategory).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFunctionInstance_Vtbl {
    pub base__: super::super::System::Com::IServiceProvider_Vtbl,
    pub GetID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16) -> windows_core::HRESULT,
    pub GetProviderInstanceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub OpenPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Com::STGM, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    OpenPropertyStore: usize,
    pub GetCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16, *mut *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFunctionInstanceCollection, IFunctionInstanceCollection_Vtbl, 0xf0a3d895_855c_42a2_948d_2f97d450ecb1);
impl core::ops::Deref for IFunctionInstanceCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFunctionInstanceCollection, windows_core::IUnknown);
impl IFunctionInstanceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Get<P0>(&self, pszinstanceidentity: P0, pdwindex: *mut u32) -> windows_core::Result<IFunctionInstance>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), pszinstanceidentity.param().abi(), pdwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, dwindex: u32) -> windows_core::Result<IFunctionInstance> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pifunctioninstance: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFunctionInstance>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pifunctioninstance.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Remove(&self, dwindex: u32) -> windows_core::Result<IFunctionInstance> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete(&self, dwindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), dwindex).ok()
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IFunctionInstanceCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Get: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Remove: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFunctionInstanceCollectionQuery, IFunctionInstanceCollectionQuery_Vtbl, 0x57cc6fd2_c09a_4289_bb72_25f04142058e);
impl core::ops::Deref for IFunctionInstanceCollectionQuery {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFunctionInstanceCollectionQuery, windows_core::IUnknown);
impl IFunctionInstanceCollectionQuery {
    pub unsafe fn AddQueryConstraint<P0, P1>(&self, pszconstraintname: P0, pszconstraintvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddQueryConstraint)(windows_core::Interface::as_raw(self), pszconstraintname.param().abi(), pszconstraintvalue.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn AddPropertyConstraint(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const windows_core::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddPropertyConstraint)(windows_core::Interface::as_raw(self), key, core::mem::transmute(pv), enumpropertyconstraint).ok()
    }
    pub unsafe fn Execute(&self) -> windows_core::Result<IFunctionInstanceCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IFunctionInstanceCollectionQuery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddQueryConstraint: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub AddPropertyConstraint: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, PropertyConstraint) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    AddPropertyConstraint: usize,
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFunctionInstanceQuery, IFunctionInstanceQuery_Vtbl, 0x6242bc6b_90ec_4b37_bb46_e229fd84ed95);
impl core::ops::Deref for IFunctionInstanceQuery {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFunctionInstanceQuery, windows_core::IUnknown);
impl IFunctionInstanceQuery {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Execute(&self) -> windows_core::Result<IFunctionInstance> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IFunctionInstanceQuery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Execute: usize,
}
windows_core::imp::define_interface!(IPNPXAssociation, IPNPXAssociation_Vtbl, 0x0bd7e521_4da6_42d5_81ba_1981b6b94075);
impl core::ops::Deref for IPNPXAssociation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPNPXAssociation, windows_core::IUnknown);
impl IPNPXAssociation {
    pub unsafe fn Associate<P0>(&self, pszsubcategory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Associate)(windows_core::Interface::as_raw(self), pszsubcategory.param().abi()).ok()
    }
    pub unsafe fn Unassociate<P0>(&self, pszsubcategory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Unassociate)(windows_core::Interface::as_raw(self), pszsubcategory.param().abi()).ok()
    }
    pub unsafe fn Delete<P0>(&self, pszsubcategory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), pszsubcategory.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IPNPXAssociation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Associate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Unassociate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPNPXDeviceAssociation, IPNPXDeviceAssociation_Vtbl, 0xeed366d0_35b8_4fc5_8d20_7e5bd31f6ded);
impl core::ops::Deref for IPNPXDeviceAssociation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPNPXDeviceAssociation, windows_core::IUnknown);
impl IPNPXDeviceAssociation {
    pub unsafe fn Associate<P0, P1>(&self, pszsubcategory: P0, pifunctiondiscoverynotification: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IFunctionDiscoveryNotification>,
    {
        (windows_core::Interface::vtable(self).Associate)(windows_core::Interface::as_raw(self), pszsubcategory.param().abi(), pifunctiondiscoverynotification.param().abi()).ok()
    }
    pub unsafe fn Unassociate<P0, P1>(&self, pszsubcategory: P0, pifunctiondiscoverynotification: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IFunctionDiscoveryNotification>,
    {
        (windows_core::Interface::vtable(self).Unassociate)(windows_core::Interface::as_raw(self), pszsubcategory.param().abi(), pifunctiondiscoverynotification.param().abi()).ok()
    }
    pub unsafe fn Delete<P0, P1>(&self, pszsubcategory: P0, pifunctiondiscoverynotification: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IFunctionDiscoveryNotification>,
    {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), pszsubcategory.param().abi(), pifunctiondiscoverynotification.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IPNPXDeviceAssociation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Associate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unassociate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyStoreCollection, IPropertyStoreCollection_Vtbl, 0xd14d9c30_12d2_42d8_bce4_c60c2bb226fa);
impl core::ops::Deref for IPropertyStoreCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyStoreCollection, windows_core::IUnknown);
impl IPropertyStoreCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Get<P0>(&self, pszinstanceidentity: P0, pdwindex: *mut u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), pszinstanceidentity.param().abi(), pdwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Item(&self, dwindex: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Add<P0>(&self, pipropertystore: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pipropertystore.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Remove(&self, dwindex: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete(&self, dwindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), dwindex).ok()
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IPropertyStoreCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Get: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Item: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Add: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Remove: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProviderProperties, IProviderProperties_Vtbl, 0xcf986ea6_3b5f_4c5f_b88a_2f8b20ceef17);
impl core::ops::Deref for IProviderProperties {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProviderProperties, windows_core::IUnknown);
impl IProviderProperties {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCount<P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IFunctionInstance>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pifunctioninstance.param().abi(), iproviderinstancecontext, &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetAt<P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFunctionInstance>,
    {
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), pifunctioninstance.param().abi(), iproviderinstancecontext, dwindex, pkey).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue<P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<windows_core::PROPVARIANT>
    where
        P0: windows_core::Param<IFunctionInstance>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), pifunctioninstance.param().abi(), iproviderinstancecontext, key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetValue<P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const windows_core::PROPVARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFunctionInstance>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), pifunctioninstance.param().abi(), iproviderinstancecontext, key, core::mem::transmute(ppropvar)).ok()
    }
}
#[repr(C)]
pub struct IProviderProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, isize, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCount: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, isize, u32, *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetAt: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, isize, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetValue: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, isize, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    SetValue: usize,
}
windows_core::imp::define_interface!(IProviderPropertyConstraintCollection, IProviderPropertyConstraintCollection_Vtbl, 0xf4fae42f_5778_4a13_8540_b5fd8c1398dd);
impl core::ops::Deref for IProviderPropertyConstraintCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProviderPropertyConstraintCollection, windows_core::IUnknown);
impl IProviderPropertyConstraintCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Get(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut windows_core::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), key, core::mem::transmute(ppropvar), pdwpropertyconstraint).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Item(&self, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut windows_core::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), dwindex, pkey, core::mem::transmute(ppropvar), pdwpropertyconstraint).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Next(&self, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut windows_core::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pkey, core::mem::transmute(ppropvar), pdwpropertyconstraint).ok()
    }
    pub unsafe fn Skip(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IProviderPropertyConstraintCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Get: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Item: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProviderPublishing, IProviderPublishing_Vtbl, 0xcd1b9a04_206c_4a05_a0c8_1635a21a2b7c);
impl core::ops::Deref for IProviderPublishing {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProviderPublishing, windows_core::IUnknown);
impl IProviderPublishing {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateInstance<P0, P1>(&self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: P0, pszproviderinstanceidentity: P1) -> windows_core::Result<IFunctionInstance>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), enumvisibilityflags, pszsubcategory.param().abi(), pszproviderinstanceidentity.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveInstance<P0, P1>(&self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: P0, pszproviderinstanceidentity: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveInstance)(windows_core::Interface::as_raw(self), enumvisibilityflags, pszsubcategory.param().abi(), pszproviderinstanceidentity.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IProviderPublishing_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, SystemVisibilityFlags, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateInstance: usize,
    pub RemoveInstance: unsafe extern "system" fn(*mut core::ffi::c_void, SystemVisibilityFlags, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProviderQueryConstraintCollection, IProviderQueryConstraintCollection_Vtbl, 0x9c243e11_3261_4bcd_b922_84a873d460ae);
impl core::ops::Deref for IProviderQueryConstraintCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProviderQueryConstraintCollection, windows_core::IUnknown);
impl IProviderQueryConstraintCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Get<P0>(&self, pszconstraintname: P0) -> windows_core::Result<*mut u16>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), pszconstraintname.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Item(&self, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), dwindex, ppszconstraintname, ppszconstraintvalue).ok()
    }
    pub unsafe fn Next(&self, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppszconstraintname, ppszconstraintvalue).ok()
    }
    pub unsafe fn Skip(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IProviderQueryConstraintCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut u16) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u16, *mut *mut u16) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16, *mut *mut u16) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const DEVICEDISPLAY_DISCOVERYMETHOD_AD_PRINTER: windows_core::PCWSTR = windows_core::w!("Published Printer");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_ASP_INFRA: windows_core::PCWSTR = windows_core::w!("AspInfra");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_BLUETOOTH: windows_core::PCWSTR = windows_core::w!("Bluetooth");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_BLUETOOTH_LE: windows_core::PCWSTR = windows_core::w!("Bluetooth Low Energy");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_NETBIOS: windows_core::PCWSTR = windows_core::w!("NetBIOS");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_PNP: windows_core::PCWSTR = windows_core::w!("PnP");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_UPNP: windows_core::PCWSTR = windows_core::w!("UPnP");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_WFD: windows_core::PCWSTR = windows_core::w!("WiFiDirect");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_WSD: windows_core::PCWSTR = windows_core::w!("WSD");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_WUSB: windows_core::PCWSTR = windows_core::w!("WUSB");
pub const E_FDPAIRING_AUTHFAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8FD00003_u32 as _);
pub const E_FDPAIRING_AUTHNOTALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x8FD00006_u32 as _);
pub const E_FDPAIRING_CONNECTTIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8FD00004_u32 as _);
pub const E_FDPAIRING_HWFAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8FD00002_u32 as _);
pub const E_FDPAIRING_IPBUSDISABLED: windows_core::HRESULT = windows_core::HRESULT(0x8FD00007_u32 as _);
pub const E_FDPAIRING_NOCONNECTION: windows_core::HRESULT = windows_core::HRESULT(0x8FD00001_u32 as _);
pub const E_FDPAIRING_NOPROFILES: windows_core::HRESULT = windows_core::HRESULT(0x8FD00008_u32 as _);
pub const E_FDPAIRING_TOOMANYCONNECTIONS: windows_core::HRESULT = windows_core::HRESULT(0x8FD00005_u32 as _);
pub const FCTN_CATEGORY_BT: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Devices.Bluetooth");
pub const FCTN_CATEGORY_DEVICEDISPLAYOBJECTS: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Base.DeviceDisplayObjects");
pub const FCTN_CATEGORY_DEVICEFUNCTIONENUMERATORS: windows_core::PCWSTR = windows_core::w!("Layered\\Microsoft.Devices.FunctionEnumerators");
pub const FCTN_CATEGORY_DEVICEPAIRING: windows_core::PCWSTR = windows_core::w!("Layered\\Microsoft.Base.DevicePairing");
pub const FCTN_CATEGORY_DEVICES: windows_core::PCWSTR = windows_core::w!("Layered\\Microsoft.Base.Devices");
pub const FCTN_CATEGORY_DEVQUERYOBJECTS: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Base.DevQueryObjects");
pub const FCTN_CATEGORY_NETBIOS: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Networking.Netbios");
pub const FCTN_CATEGORY_NETWORKDEVICES: windows_core::PCWSTR = windows_core::w!("Layered\\Microsoft.Networking.Devices");
pub const FCTN_CATEGORY_PNP: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Base.PnP");
pub const FCTN_CATEGORY_PNPXASSOCIATION: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.PnPX.Association");
pub const FCTN_CATEGORY_PUBLICATION: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Base.Publication");
pub const FCTN_CATEGORY_REGISTRY: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Base.Registry");
pub const FCTN_CATEGORY_SSDP: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Networking.SSDP");
pub const FCTN_CATEGORY_WCN: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Networking.WCN");
pub const FCTN_CATEGORY_WSDISCOVERY: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Networking.WSD");
pub const FCTN_CATEGORY_WUSB: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Devices.WirelessUSB");
pub const FCTN_SUBCAT_DEVICES_WSDPRINTERS: windows_core::PCWSTR = windows_core::w!("WSDPrinters");
pub const FCTN_SUBCAT_NETWORKDEVICES_SSDP: windows_core::PCWSTR = windows_core::w!("SSDP");
pub const FCTN_SUBCAT_NETWORKDEVICES_WSD: windows_core::PCWSTR = windows_core::w!("WSD");
pub const FCTN_SUBCAT_REG_DIRECTED: windows_core::PCWSTR = windows_core::w!("Directed");
pub const FCTN_SUBCAT_REG_PUBLICATION: windows_core::PCWSTR = windows_core::w!("Publication");
pub const FD_CONSTRAINTVALUE_ALL: windows_core::PCWSTR = windows_core::w!("All");
pub const FD_CONSTRAINTVALUE_COMCLSCONTEXT_INPROC_SERVER: windows_core::PCWSTR = windows_core::w!("1");
pub const FD_CONSTRAINTVALUE_COMCLSCONTEXT_LOCAL_SERVER: windows_core::PCWSTR = windows_core::w!("4");
pub const FD_CONSTRAINTVALUE_FALSE: windows_core::PCWSTR = windows_core::w!("FALSE");
pub const FD_CONSTRAINTVALUE_PAIRED: windows_core::PCWSTR = windows_core::w!("Paired");
pub const FD_CONSTRAINTVALUE_RECURSESUBCATEGORY_TRUE: windows_core::PCWSTR = windows_core::w!("TRUE");
pub const FD_CONSTRAINTVALUE_ROUTINGSCOPE_ALL: windows_core::PCWSTR = windows_core::w!("All");
pub const FD_CONSTRAINTVALUE_ROUTINGSCOPE_DIRECT: windows_core::PCWSTR = windows_core::w!("Direct");
pub const FD_CONSTRAINTVALUE_TRUE: windows_core::PCWSTR = windows_core::w!("TRUE");
pub const FD_CONSTRAINTVALUE_UNPAIRED: windows_core::PCWSTR = windows_core::w!("UnPaired");
pub const FD_CONSTRAINTVALUE_VISIBILITY_ALL: windows_core::PCWSTR = windows_core::w!("1");
pub const FD_CONSTRAINTVALUE_VISIBILITY_DEFAULT: windows_core::PCWSTR = windows_core::w!("0");
pub const FD_EVENTID: u32 = 1000u32;
pub const FD_EVENTID_ASYNCTHREADEXIT: u32 = 1001u32;
pub const FD_EVENTID_IPADDRESSCHANGE: u32 = 1003u32;
pub const FD_EVENTID_PRIVATE: u32 = 100u32;
pub const FD_EVENTID_QUERYREFRESH: u32 = 1004u32;
pub const FD_EVENTID_SEARCHCOMPLETE: u32 = 1000u32;
pub const FD_EVENTID_SEARCHSTART: u32 = 1002u32;
pub const FD_LONGHORN: u32 = 1u32;
pub const FD_QUERYCONSTRAINT_COMCLSCONTEXT: windows_core::PCWSTR = windows_core::w!("COMClsContext");
pub const FD_QUERYCONSTRAINT_INQUIRY_TIMEOUT: windows_core::PCWSTR = windows_core::w!("InquiryModeTimeout");
pub const FD_QUERYCONSTRAINT_PAIRING_STATE: windows_core::PCWSTR = windows_core::w!("PairingState");
pub const FD_QUERYCONSTRAINT_PROVIDERINSTANCEID: windows_core::PCWSTR = windows_core::w!("ProviderInstanceID");
pub const FD_QUERYCONSTRAINT_RECURSESUBCATEGORY: windows_core::PCWSTR = windows_core::w!("RecurseSubcategory");
pub const FD_QUERYCONSTRAINT_ROUTINGSCOPE: windows_core::PCWSTR = windows_core::w!("RoutingScope");
pub const FD_QUERYCONSTRAINT_SUBCATEGORY: windows_core::PCWSTR = windows_core::w!("Subcategory");
pub const FD_QUERYCONSTRAINT_VISIBILITY: windows_core::PCWSTR = windows_core::w!("Visibility");
pub const FD_SUBKEY: windows_core::PCWSTR = windows_core::w!("SOFTWARE\\Microsoft\\Function Discovery\\");
pub const FD_Visibility_Default: u32 = 0u32;
pub const FD_Visibility_Hidden: u32 = 1u32;
pub const FMTID_Device: windows_core::GUID = windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57);
pub const FMTID_DeviceInterface: windows_core::GUID = windows_core::GUID::from_u128(0x53808008_07bb_4661_bc3c_b5953e708560);
pub const FMTID_FD: windows_core::GUID = windows_core::GUID::from_u128(0x904b03a2_471d_423c_a584_f3483238a146);
pub const FMTID_PNPX: windows_core::GUID = windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd);
pub const FMTID_PNPXDynamicProperty: windows_core::GUID = windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd);
pub const FMTID_Pairing: windows_core::GUID = windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc);
pub const FMTID_WSD: windows_core::GUID = windows_core::GUID::from_u128(0x92506491_ff95_4724_a05a_5b81885a7c92);
pub const MAX_FDCONSTRAINTNAME_LENGTH: u32 = 100u32;
pub const MAX_FDCONSTRAINTVALUE_LENGTH: u32 = 1000u32;
pub const ONLINE_PROVIDER_DEVICES_QUERYCONSTRAINT_OWNERNAME: windows_core::PCWSTR = windows_core::w!("OwnerName");
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Characteristics: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 29 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_ClassCoInstallers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x713d1703_a2e2_49f5_9214_56472ef3da5c), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_ClassInstaller: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_ClassName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_DefaultService: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 11 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_DevType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 27 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Exclusive: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 28 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_IconPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 12 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_LowerFilters: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 20 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Name: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_NoDisplayClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_NoInstallClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_NoUseClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 10 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_PropPageProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Security: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 25 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_SecuritySDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 26 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_SilentInstall: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_UpperFilters: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 19 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Address: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 51 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_AlwaysShowDeviceAsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 101 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_AssociationArray: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 80 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_BaselineExperienceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 78 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Category: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 90 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_CategoryGroup_Desc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 94 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_CategoryGroup_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 95 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Category_Desc_Plural: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 92 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Category_Desc_Singular: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 91 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Category_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 93 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_DeviceDescription1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 81 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_DeviceDescription2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 82 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_DeviceFunctionSubRank: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 100 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_DiscoveryMethod: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 52 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_ExperienceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 89 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_FriendlyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12288 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 57 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_InstallInProgress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x83da6326_97a6_4088_9453_a1923f573b29), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsAuthenticated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 54 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 55 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsDefaultDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 86 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsDeviceUniquelyIdentifiable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 79 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsEncrypted: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 53 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsLocalMachine: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 70 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsMetadataSearchInProgress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 72 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsNetworkDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 85 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsNotInterestingForDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 74 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsNotWorkingProperly: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 83 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsPaired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 56 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsSharedDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 84 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsShowInDisconnectedState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 68 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Last_Connected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 67 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Last_Seen: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 66 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_LaunchDeviceStageFromExplorer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 77 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_LaunchDeviceStageOnDeviceConnect: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 76 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8192 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_MetadataCabinet: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 87 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_MetadataChecksum: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 73 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_MetadataPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 71 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_ModelName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8194 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_ModelNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8195 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_PrimaryCategory: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 97 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_RequiresPairingElevation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 88 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_RequiresUninstallElevation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 99 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_UnpairUninstall: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 98 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Version: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 65 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterfaceClass_DefaultInterface: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x14c83a99_0b3f_44b7_be4c_a178d3990564), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_ClassGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Enabled: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_FriendlyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_AdditionalSoftwareRequested: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 19 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Address: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 30 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BIOSVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xeaee7f1d_6a33_44d1_9441_5f46def23198), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BaseContainerId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 38 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BusNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 23 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BusRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BusReportedDeviceDesc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BusTypeGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 21 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Capabilities: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 17 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Characteristics: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 29 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Children: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Class: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ClassGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 10 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_CompatibleIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ConfigFlags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 12 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ContainerId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8c7ed206_3f8a_4827_b3ab_ae9e1faefc6c), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DHP_Rebalance_Policy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DevNodeStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DevType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 27 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DeviceDesc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Driver: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 11 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverCoInstallers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 11 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverDesc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverInfPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverInfSection: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverInfSectionExt: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverLogoLevel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 15 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverPropPageProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 10 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverRank: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 14 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_EjectionRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_EnumeratorName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 24 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Exclusive: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 28 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_FriendlyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 14 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_FriendlyNameAttributes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_GenericDriverInstalled: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 18 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_HardwareIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_InstallInProgress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x83da6326_97a6_4088_9453_a1923f573b29), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_InstallState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 36 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_InstanceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 256 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_IsAssociateableByUserAction: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Legacy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80497100_8c73_48b9_aad9_ce387e19c56e), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_LegacyBusType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 22 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_LocationInfo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 15 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_LocationPaths: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 37 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_LowerFilters: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 20 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 13 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ManufacturerAttributes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_MatchingDeviceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ModelId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_NoConnectSound: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 17 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Numa_Node: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PDOName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 16 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Parent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PowerData: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PowerRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PresenceNotForDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ProblemCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_RemovalPolicy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 33 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_RemovalPolicyDefault: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 34 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_RemovalPolicyOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 35 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_RemovalRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Reported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80497100_8c73_48b9_aad9_ce387e19c56e), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ResourcePickerExceptions: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 13 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ResourcePickerTags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 12 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_SafeRemovalRequired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xafd97640_86a3_4210_b67c_289c41aabe55), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_SafeRemovalRequiredOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xafd97640_86a3_4210_b67c_289c41aabe55), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Security: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 25 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_SecuritySDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 26 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Service: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Siblings: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 10 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_SignalStrength: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_TransportRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 11 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_UINumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 18 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_UINumberDescFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 31 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_UpperFilters: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 19 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_BrandingIcon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_DetailedDescription: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_DocumentationLink: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_Model: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_VendorWebSite: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FunctionInstance: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x08c0c253_a154_4746_9005_82de5317148b), pid: 1 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Devinst: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4097 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_DisplayAttribute: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_DriverDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 11 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_DriverProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 10 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_DriverVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Function: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4099 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Image: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4098 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Model: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Name: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_SerialNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_ShellAttributes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4100 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Status: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4096 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 10 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Numa_Proximity_Domain: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 1 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Associated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Category_Desc_NonPlural: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12304 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_CompactSignature: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28674 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_CompatibleTypes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_DeviceCategory: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12292 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_DeviceCategory_Desc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12293 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_DeviceCertHash: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28675 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_DomainName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 20480 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_FirmwareVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12289 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_GlobalIdentity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4096 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4101 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_IPBusEnumerated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28688 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_InstallState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Installable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 1 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_IpAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12297 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ManufacturerUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8193 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_MetadataVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4100 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ModelUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8196 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_NetworkInterfaceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12296 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_NetworkInterfaceLuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12295 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_PhysicalAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12294 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_PresentationUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8198 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_RemoteAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4102 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Removable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28672 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_RootProxy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4103 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Scopes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4098 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_SecureChannel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28673 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_SerialNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12290 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16384 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceControlUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16388 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceDescUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16389 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceEventSubUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16390 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16385 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceTypes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16386 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ShareName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 20482 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Types: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4097 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Upc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8197 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_XAddrs: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4099 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_IsWifiOnlyDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 16 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_ListItemDefault: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_ListItemDescription: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_ListItemIcon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_ListItemText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 1 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SSDP_AltLocationInfo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 24576 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SSDP_DevLifeTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 24577 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SSDP_NetworkInterface: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 24578 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_AssocState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b88_4684_11da_a26a_0002b3988e81), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_AuthType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b82_4684_11da_a26a_0002b3988e81), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_ConfigError: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 10 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_ConfigMethods: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b85_4684_11da_a26a_0002b3988e81), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_ConfigState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 11 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_ConnType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b84_4684_11da_a26a_0002b3988e81), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_DevicePasswordId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 12 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_EncryptType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b83_4684_11da_a26a_0002b3988e81), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_OSVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 13 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_RegistrarType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b8b_4684_11da_a26a_0002b3988e81), pid: 15 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_RequestType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b81_4684_11da_a26a_0002b3988e81), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_RfBand: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b87_4684_11da_a26a_0002b3988e81), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_VendorExtension: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b8a_4684_11da_a26a_0002b3988e81), pid: 14 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_Version: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b80_4684_11da_a26a_0002b3988e81), pid: 1 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Comment: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_DisplayType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_LocalName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Provider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_RemoteName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Scope: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 1 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Type: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Usage: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 4 };
pub const PNPX_DEVICECATEGORY_CAMERA: windows_core::PCWSTR = windows_core::w!("Cameras");
pub const PNPX_DEVICECATEGORY_COMPUTER: windows_core::PCWSTR = windows_core::w!("Computers");
pub const PNPX_DEVICECATEGORY_DISPLAYS: windows_core::PCWSTR = windows_core::w!("Displays");
pub const PNPX_DEVICECATEGORY_FAX: windows_core::PCWSTR = windows_core::w!("FAX");
pub const PNPX_DEVICECATEGORY_GAMING_DEVICE: windows_core::PCWSTR = windows_core::w!("Gaming");
pub const PNPX_DEVICECATEGORY_HOME_AUTOMATION_SYSTEM: windows_core::PCWSTR = windows_core::w!("HomeAutomation");
pub const PNPX_DEVICECATEGORY_HOME_SECURITY_SYSTEM: windows_core::PCWSTR = windows_core::w!("HomeSecurity");
pub const PNPX_DEVICECATEGORY_INPUTDEVICE: windows_core::PCWSTR = windows_core::w!("Input");
pub const PNPX_DEVICECATEGORY_MFP: windows_core::PCWSTR = windows_core::w!("MFP");
pub const PNPX_DEVICECATEGORY_MULTIMEDIA_DEVICE: windows_core::PCWSTR = windows_core::w!("MediaDevices");
pub const PNPX_DEVICECATEGORY_NETWORK_INFRASTRUCTURE: windows_core::PCWSTR = windows_core::w!("NetworkInfrastructure");
pub const PNPX_DEVICECATEGORY_OTHER: windows_core::PCWSTR = windows_core::w!("Other");
pub const PNPX_DEVICECATEGORY_PRINTER: windows_core::PCWSTR = windows_core::w!("Printers");
pub const PNPX_DEVICECATEGORY_SCANNER: windows_core::PCWSTR = windows_core::w!("Scanners");
pub const PNPX_DEVICECATEGORY_STORAGE: windows_core::PCWSTR = windows_core::w!("Storage");
pub const PNPX_DEVICECATEGORY_TELEPHONE: windows_core::PCWSTR = windows_core::w!("Phones");
pub const PNPX_INSTALLSTATE_FAILED: u32 = 3u32;
pub const PNPX_INSTALLSTATE_INSTALLED: u32 = 1u32;
pub const PNPX_INSTALLSTATE_INSTALLING: u32 = 2u32;
pub const PNPX_INSTALLSTATE_NOTINSTALLED: u32 = 0u32;
pub const PNP_CONSTRAINTVALUE_NOTIFICATIONSONLY: windows_core::PCWSTR = windows_core::w!("TRUE");
pub const PNP_CONSTRAINTVALUE_NOTPRESENT: windows_core::PCWSTR = windows_core::w!("TRUE");
pub const PROVIDERDDO_QUERYCONSTRAINT_DEVICEFUNCTIONDISPLAYOBJECTS: windows_core::PCWSTR = windows_core::w!("DeviceFunctionDisplayObjects");
pub const PROVIDERDDO_QUERYCONSTRAINT_DEVICEINTERFACES: windows_core::PCWSTR = windows_core::w!("DeviceInterfaces");
pub const PROVIDERDDO_QUERYCONSTRAINT_ONLYCONNECTEDDEVICES: windows_core::PCWSTR = windows_core::w!("OnlyConnectedDevices");
pub const PROVIDERPNP_QUERYCONSTRAINT_INTERFACECLASS: windows_core::PCWSTR = windows_core::w!("InterfaceClass");
pub const PROVIDERPNP_QUERYCONSTRAINT_NOTIFICATIONSONLY: windows_core::PCWSTR = windows_core::w!("NotifyOnly");
pub const PROVIDERPNP_QUERYCONSTRAINT_NOTPRESENT: windows_core::PCWSTR = windows_core::w!("NotPresent");
pub const PROVIDERSSDP_QUERYCONSTRAINT_CUSTOMXMLPROPERTY: windows_core::PCWSTR = windows_core::w!("CustomXmlProperty");
pub const PROVIDERSSDP_QUERYCONSTRAINT_TYPE: windows_core::PCWSTR = windows_core::w!("Type");
pub const PROVIDERWNET_QUERYCONSTRAINT_PROPERTIES: windows_core::PCWSTR = windows_core::w!("Properties");
pub const PROVIDERWNET_QUERYCONSTRAINT_RESOURCETYPE: windows_core::PCWSTR = windows_core::w!("ResourceType");
pub const PROVIDERWNET_QUERYCONSTRAINT_TYPE: windows_core::PCWSTR = windows_core::w!("Type");
pub const PROVIDERWSD_QUERYCONSTRAINT_DIRECTEDADDRESS: windows_core::PCWSTR = windows_core::w!("RemoteAddress");
pub const PROVIDERWSD_QUERYCONSTRAINT_SCOPE: windows_core::PCWSTR = windows_core::w!("Scope");
pub const PROVIDERWSD_QUERYCONSTRAINT_SECURITY_REQUIREMENTS: windows_core::PCWSTR = windows_core::w!("SecurityRequirements");
pub const PROVIDERWSD_QUERYCONSTRAINT_SSL_CERTHASH_FOR_SERVER_AUTH: windows_core::PCWSTR = windows_core::w!("SSLServerAuthCertHash");
pub const PROVIDERWSD_QUERYCONSTRAINT_SSL_CERT_FOR_CLIENT_AUTH: windows_core::PCWSTR = windows_core::w!("SSLClientAuthCert");
pub const PROVIDERWSD_QUERYCONSTRAINT_TYPE: windows_core::PCWSTR = windows_core::w!("Type");
pub const QCT_LAYERED: QueryCategoryType = QueryCategoryType(1i32);
pub const QCT_PROVIDER: QueryCategoryType = QueryCategoryType(0i32);
pub const QC_CONTAINS: PropertyConstraint = PropertyConstraint(9i32);
pub const QC_DOESNOTEXIST: PropertyConstraint = PropertyConstraint(8i32);
pub const QC_EQUALS: PropertyConstraint = PropertyConstraint(0i32);
pub const QC_EXISTS: PropertyConstraint = PropertyConstraint(7i32);
pub const QC_GREATERTHAN: PropertyConstraint = PropertyConstraint(4i32);
pub const QC_GREATERTHANOREQUAL: PropertyConstraint = PropertyConstraint(5i32);
pub const QC_LESSTHAN: PropertyConstraint = PropertyConstraint(2i32);
pub const QC_LESSTHANOREQUAL: PropertyConstraint = PropertyConstraint(3i32);
pub const QC_NOTEQUAL: PropertyConstraint = PropertyConstraint(1i32);
pub const QC_STARTSWITH: PropertyConstraint = PropertyConstraint(6i32);
pub const QUA_ADD: QueryUpdateAction = QueryUpdateAction(0i32);
pub const QUA_CHANGE: QueryUpdateAction = QueryUpdateAction(2i32);
pub const QUA_REMOVE: QueryUpdateAction = QueryUpdateAction(1i32);
pub const SID_DeviceDisplayStatusManager: windows_core::GUID = windows_core::GUID::from_u128(0xf59aa553_8309_46ca_9736_1ac3c62d6031);
pub const SID_EnumDeviceFunction: windows_core::GUID = windows_core::GUID::from_u128(0x13e0e9e2_c3fa_4e3c_906e_64502fa4dc95);
pub const SID_EnumInterface: windows_core::GUID = windows_core::GUID::from_u128(0x40eab0b9_4d7f_4b53_a334_1581dd9041f4);
pub const SID_FDPairingHandler: windows_core::GUID = windows_core::GUID::from_u128(0x383b69fa_5486_49da_91f5_d63c24c8e9d0);
pub const SID_FunctionDiscoveryProviderRefresh: windows_core::GUID = windows_core::GUID::from_u128(0x2b4cbdc9_31c4_40d4_a62d_772aa174ed52);
pub const SID_PNPXAssociation: windows_core::GUID = windows_core::GUID::from_u128(0xcee8ccc9_4f6b_4469_a235_5a22869eef03);
pub const SID_PNPXPropertyStore: windows_core::GUID = windows_core::GUID::from_u128(0xa86530b1_542f_439f_b71c_b0756b13677a);
pub const SID_PNPXServiceCollection: windows_core::GUID = windows_core::GUID::from_u128(0x439e80ee_a217_4712_9fa6_deabd9c2a727);
pub const SID_PnpProvider: windows_core::GUID = windows_core::GUID::from_u128(0x8101368e_cabb_4426_acff_96c410812000);
pub const SID_UPnPActivator: windows_core::GUID = windows_core::GUID::from_u128(0x0d0d66eb_cf74_4164_b52f_08344672dd46);
pub const SID_UninstallDeviceFunction: windows_core::GUID = windows_core::GUID::from_u128(0xc920566e_5671_4496_8025_bf0b89bd44cd);
pub const SID_UnpairProvider: windows_core::GUID = windows_core::GUID::from_u128(0x89a502fc_857b_4698_a0b7_027192002f9e);
pub const SSDP_CONSTRAINTVALUE_TYPE_ALL: windows_core::PCWSTR = windows_core::w!("ssdp:all");
pub const SSDP_CONSTRAINTVALUE_TYPE_DEVICE_PREFIX: windows_core::PCWSTR = windows_core::w!("urn:schemas-upnp-org:device:");
pub const SSDP_CONSTRAINTVALUE_TYPE_ROOT: windows_core::PCWSTR = windows_core::w!("upnp:rootdevice");
pub const SSDP_CONSTRAINTVALUE_TYPE_SVC_PREFIX: windows_core::PCWSTR = windows_core::w!("urn:schemas-upnp-org:service:");
pub const SVF_SYSTEM: SystemVisibilityFlags = SystemVisibilityFlags(0i32);
pub const SVF_USER: SystemVisibilityFlags = SystemVisibilityFlags(1i32);
pub const WNET_CONSTRAINTVALUE_PROPERTIES_ALL: windows_core::PCWSTR = windows_core::w!("All");
pub const WNET_CONSTRAINTVALUE_PROPERTIES_LIMITED: windows_core::PCWSTR = windows_core::w!("Limited");
pub const WNET_CONSTRAINTVALUE_RESOURCETYPE_DISK: windows_core::PCWSTR = windows_core::w!("Disk");
pub const WNET_CONSTRAINTVALUE_RESOURCETYPE_DISKORPRINTER: windows_core::PCWSTR = windows_core::w!("DiskOrPrinter");
pub const WNET_CONSTRAINTVALUE_RESOURCETYPE_PRINTER: windows_core::PCWSTR = windows_core::w!("Printer");
pub const WNET_CONSTRAINTVALUE_TYPE_ALL: windows_core::PCWSTR = windows_core::w!("All");
pub const WNET_CONSTRAINTVALUE_TYPE_DOMAIN: windows_core::PCWSTR = windows_core::w!("Domain");
pub const WNET_CONSTRAINTVALUE_TYPE_SERVER: windows_core::PCWSTR = windows_core::w!("Server");
pub const WSD_CONSTRAINTVALUE_NO_TRUST_VERIFICATION: windows_core::PCWSTR = windows_core::w!("3");
pub const WSD_CONSTRAINTVALUE_REQUIRE_SECURECHANNEL: windows_core::PCWSTR = windows_core::w!("1");
pub const WSD_CONSTRAINTVALUE_REQUIRE_SECURECHANNEL_AND_COMPACTSIGNATURE: windows_core::PCWSTR = windows_core::w!("2");
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PropertyConstraint(pub i32);
impl windows_core::TypeKind for PropertyConstraint {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PropertyConstraint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PropertyConstraint").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct QueryCategoryType(pub i32);
impl windows_core::TypeKind for QueryCategoryType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for QueryCategoryType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("QueryCategoryType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct QueryUpdateAction(pub i32);
impl windows_core::TypeKind for QueryUpdateAction {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for QueryUpdateAction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("QueryUpdateAction").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SystemVisibilityFlags(pub i32);
impl windows_core::TypeKind for SystemVisibilityFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SystemVisibilityFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SystemVisibilityFlags").field(&self.0).finish()
    }
}
pub const FunctionDiscovery: windows_core::GUID = windows_core::GUID::from_u128(0xc72be2ec_8e90_452c_b29a_ab8ff1c071fc);
pub const FunctionInstanceCollection: windows_core::GUID = windows_core::GUID::from_u128(0xba818ce5_b55f_443f_ad39_2fe89be6191f);
pub const PNPXAssociation: windows_core::GUID = windows_core::GUID::from_u128(0xcee8ccc9_4f6b_4469_a235_5a22869eef03);
pub const PNPXPairingHandler: windows_core::GUID = windows_core::GUID::from_u128(0xb8a27942_ade7_4085_aa6e_4fadc7ada1ef);
pub const PropertyStore: windows_core::GUID = windows_core::GUID::from_u128(0xe4796550_df61_448b_9193_13fc1341b163);
pub const PropertyStoreCollection: windows_core::GUID = windows_core::GUID::from_u128(0xedd36029_d753_4862_aa5b_5bccad2a4d29);
#[cfg(feature = "implement")]
core::include!("impl.rs");
