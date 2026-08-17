#[inline]
pub unsafe fn CoCreateActivity<P0>(piunknown: P0, riid: *const windows_core::GUID, ppobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("comsvcs.dll" "system" fn CoCreateActivity(piunknown : * mut core::ffi::c_void, riid : *const windows_core::GUID, ppobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    CoCreateActivity(piunknown.param().abi(), riid, ppobj).ok()
}
#[inline]
pub unsafe fn CoEnterServiceDomain<P0>(pconfigobject: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("comsvcs.dll" "system" fn CoEnterServiceDomain(pconfigobject : * mut core::ffi::c_void) -> windows_core::HRESULT);
    CoEnterServiceDomain(pconfigobject.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CoGetDefaultContext(apttype: super::Com::APTTYPE, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoGetDefaultContext(apttype : super::Com:: APTTYPE, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    CoGetDefaultContext(apttype, riid, ppv).ok()
}
#[inline]
pub unsafe fn CoLeaveServiceDomain<P0>(punkstatus: P0)
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("comsvcs.dll" "system" fn CoLeaveServiceDomain(punkstatus : * mut core::ffi::c_void));
    CoLeaveServiceDomain(punkstatus.param().abi())
}
#[inline]
pub unsafe fn GetDispenserManager() -> windows_core::Result<IDispenserManager> {
    windows_targets::link!("mtxdm.dll" "cdecl" fn GetDispenserManager(param0 : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetDispenserManager(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn GetManagedExtensions(dwexts: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("comsvcs.dll" "system" fn GetManagedExtensions(dwexts : *mut u32) -> windows_core::HRESULT);
    GetManagedExtensions(dwexts).ok()
}
#[inline]
pub unsafe fn MTSCreateActivity(riid: *const windows_core::GUID, ppobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("comsvcs.dll" "system" fn MTSCreateActivity(riid : *const windows_core::GUID, ppobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    MTSCreateActivity(riid, ppobj).ok()
}
#[inline]
pub unsafe fn RecycleSurrogate(lreasoncode: i32) -> windows_core::Result<()> {
    windows_targets::link!("comsvcs.dll" "cdecl" fn RecycleSurrogate(lreasoncode : i32) -> windows_core::HRESULT);
    RecycleSurrogate(lreasoncode).ok()
}
#[inline]
pub unsafe fn SafeRef<P0>(rid: *const windows_core::GUID, punk: P0) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("comsvcs.dll" "cdecl" fn SafeRef(rid : *const windows_core::GUID, punk : * mut core::ffi::c_void) -> *mut core::ffi::c_void);
    SafeRef(rid, punk.param().abi())
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ContextInfo, ContextInfo_Vtbl, 0x19a5a02c_0ac8_11d2_b286_00c04f8ef934);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ContextInfo {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ContextInfo, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ContextInfo {
    pub unsafe fn IsInTransaction(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsInTransaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTransaction(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTransactionId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransactionId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetActivityId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActivityId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetContextId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContextId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ContextInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub IsInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTransactionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetContextId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ContextInfo2, ContextInfo2_Vtbl, 0xc99d6e75_2375_11d4_8331_00c04f605588);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ContextInfo2 {
    type Target = ContextInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ContextInfo2, windows_core::IUnknown, super::Com::IDispatch, ContextInfo);
#[cfg(feature = "Win32_System_Com")]
impl ContextInfo2 {
    pub unsafe fn GetPartitionId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPartitionId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetApplicationId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetApplicationId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetApplicationInstanceId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetApplicationInstanceId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ContextInfo2_Vtbl {
    pub base__: ContextInfo_Vtbl,
    pub GetPartitionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetApplicationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetApplicationInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAppDomainHelper, IAppDomainHelper_Vtbl, 0xc7b67079_8255_42c6_9ec0_6994a3548780);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAppDomainHelper {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAppDomainHelper, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAppDomainHelper {
    pub unsafe fn Initialize<P0>(&self, punkad: P0, __midl__iappdomainhelper0000: isize, ppool: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), punkad.param().abi(), __midl__iappdomainhelper0000, ppool).ok()
    }
    pub unsafe fn DoCallback<P0>(&self, punkad: P0, __midl__iappdomainhelper0001: isize, ppool: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).DoCallback)(windows_core::Interface::as_raw(self), punkad.param().abi(), __midl__iappdomainhelper0001, ppool).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAppDomainHelper_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, isize, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DoCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, isize, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAssemblyLocator, IAssemblyLocator_Vtbl, 0x391ffbb9_a8ee_432a_abc8_baa238dab90f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAssemblyLocator {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAssemblyLocator, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAssemblyLocator {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetModules<P0, P1, P2>(&self, applicationdir: P0, applicationname: P1, assemblyname: P2) -> windows_core::Result<*mut super::Com::SAFEARRAY>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetModules)(windows_core::Interface::as_raw(self), applicationdir.param().abi(), applicationname.param().abi(), assemblyname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAssemblyLocator_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetModules: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetModules: usize,
}
windows_core::imp::define_interface!(IAsyncErrorNotify, IAsyncErrorNotify_Vtbl, 0xfe6777fb_a674_4177_8f32_6d707e113484);
impl core::ops::Deref for IAsyncErrorNotify {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAsyncErrorNotify, windows_core::IUnknown);
impl IAsyncErrorNotify {
    pub unsafe fn OnError(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), hr).ok()
    }
}
#[repr(C)]
pub struct IAsyncErrorNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICOMAdminCatalog, ICOMAdminCatalog_Vtbl, 0xdd662187_dfc2_11d1_a2cf_00805fc79235);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICOMAdminCatalog {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICOMAdminCatalog, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICOMAdminCatalog {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCollection<P0>(&self, bstrcollname: P0) -> windows_core::Result<super::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCollection)(windows_core::Interface::as_raw(self), bstrcollname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Connect<P0>(&self, bstrcatalogservername: P0) -> windows_core::Result<super::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), bstrcatalogservername.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MajorVersion(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MajorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MinorVersion(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MinorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCollectionByQuery<P0>(&self, bstrcollname: P0, ppsavarquery: *const *const super::Com::SAFEARRAY) -> windows_core::Result<super::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCollectionByQuery)(windows_core::Interface::as_raw(self), bstrcollname.param().abi(), ppsavarquery, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ImportComponent<P0, P1>(&self, bstrapplidorname: P0, bstrclsidorprogid: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ImportComponent)(windows_core::Interface::as_raw(self), bstrapplidorname.param().abi(), bstrclsidorprogid.param().abi()).ok()
    }
    pub unsafe fn InstallComponent<P0, P1, P2, P3>(&self, bstrapplidorname: P0, bstrdll: P1, bstrtlb: P2, bstrpsdll: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InstallComponent)(windows_core::Interface::as_raw(self), bstrapplidorname.param().abi(), bstrdll.param().abi(), bstrtlb.param().abi(), bstrpsdll.param().abi()).ok()
    }
    pub unsafe fn ShutdownApplication<P0>(&self, bstrapplidorname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ShutdownApplication)(windows_core::Interface::as_raw(self), bstrapplidorname.param().abi()).ok()
    }
    pub unsafe fn ExportApplication<P0, P1>(&self, bstrapplidorname: P0, bstrapplicationfile: P1, loptions: COMAdminApplicationExportOptions) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ExportApplication)(windows_core::Interface::as_raw(self), bstrapplidorname.param().abi(), bstrapplicationfile.param().abi(), loptions).ok()
    }
    pub unsafe fn InstallApplication<P0, P1, P2, P3, P4>(&self, bstrapplicationfile: P0, bstrdestinationdirectory: P1, loptions: COMAdminApplicationInstallOptions, bstruserid: P2, bstrpassword: P3, bstrrsn: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InstallApplication)(windows_core::Interface::as_raw(self), bstrapplicationfile.param().abi(), bstrdestinationdirectory.param().abi(), loptions, bstruserid.param().abi(), bstrpassword.param().abi(), bstrrsn.param().abi()).ok()
    }
    pub unsafe fn StopRouter(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopRouter)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RefreshRouter(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RefreshRouter)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartRouter(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StartRouter)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reserved1(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reserved1)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reserved2(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reserved2)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstallMultipleComponents<P0>(&self, bstrapplidorname: P0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InstallMultipleComponents)(windows_core::Interface::as_raw(self), bstrapplidorname.param().abi(), ppsavarfilenames, ppsavarclsids).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMultipleComponentsInfo<P0>(&self, bstrapplidorname: P0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetMultipleComponentsInfo)(windows_core::Interface::as_raw(self), bstrapplidorname.param().abi(), ppsavarfilenames, ppsavarclsids, ppsavarclassnames, ppsavarfileflags, ppsavarcomponentflags).ok()
    }
    pub unsafe fn RefreshComponents(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RefreshComponents)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackupREGDB<P0>(&self, bstrbackupfilepath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).BackupREGDB)(windows_core::Interface::as_raw(self), bstrbackupfilepath.param().abi()).ok()
    }
    pub unsafe fn RestoreREGDB<P0>(&self, bstrbackupfilepath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RestoreREGDB)(windows_core::Interface::as_raw(self), bstrbackupfilepath.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryApplicationFile<P0>(&self, bstrapplicationfile: P0, pbstrapplicationname: *mut windows_core::BSTR, pbstrapplicationdescription: *mut windows_core::BSTR, pbhasusers: *mut super::super::Foundation::VARIANT_BOOL, pbisproxy: *mut super::super::Foundation::VARIANT_BOOL, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).QueryApplicationFile)(windows_core::Interface::as_raw(self), bstrapplicationfile.param().abi(), core::mem::transmute(pbstrapplicationname), core::mem::transmute(pbstrapplicationdescription), pbhasusers, pbisproxy, ppsavarfilenames).ok()
    }
    pub unsafe fn StartApplication<P0>(&self, bstrapplidorname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).StartApplication)(windows_core::Interface::as_raw(self), bstrapplidorname.param().abi()).ok()
    }
    pub unsafe fn ServiceCheck(&self, lservice: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceCheck)(windows_core::Interface::as_raw(self), lservice, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstallMultipleEventClasses<P0>(&self, bstrapplidorname: P0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InstallMultipleEventClasses)(windows_core::Interface::as_raw(self), bstrapplidorname.param().abi(), ppsavarfilenames, ppsavarclsids).ok()
    }
    pub unsafe fn InstallEventClass<P0, P1, P2, P3>(&self, bstrapplidorname: P0, bstrdll: P1, bstrtlb: P2, bstrpsdll: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InstallEventClass)(windows_core::Interface::as_raw(self), bstrapplidorname.param().abi(), bstrdll.param().abi(), bstrtlb.param().abi(), bstrpsdll.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEventClassesForIID<P0>(&self, bstriid: P0, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetEventClassesForIID)(windows_core::Interface::as_raw(self), bstriid.param().abi(), ppsavarclsids, ppsavarprogids, ppsavardescriptions).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICOMAdminCatalog_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCollection: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Connect: usize,
    pub MajorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCollectionByQuery: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const *const super::Com::SAFEARRAY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCollectionByQuery: usize,
    pub ImportComponent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InstallComponent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ShutdownApplication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ExportApplication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, COMAdminApplicationExportOptions) -> windows_core::HRESULT,
    pub InstallApplication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, COMAdminApplicationInstallOptions, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StopRouter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RefreshRouter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartRouter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reserved1: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reserved2: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InstallMultipleComponents: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const *const super::Com::SAFEARRAY, *const *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstallMultipleComponents: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMultipleComponentsInfo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const *const super::Com::SAFEARRAY, *mut *mut super::Com::SAFEARRAY, *mut *mut super::Com::SAFEARRAY, *mut *mut super::Com::SAFEARRAY, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMultipleComponentsInfo: usize,
    pub RefreshComponents: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackupREGDB: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RestoreREGDB: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryApplicationFile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL, *mut super::super::Foundation::VARIANT_BOOL, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryApplicationFile: usize,
    pub StartApplication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ServiceCheck: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InstallMultipleEventClasses: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const *const super::Com::SAFEARRAY, *const *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstallMultipleEventClasses: usize,
    pub InstallEventClass: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEventClassesForIID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut super::Com::SAFEARRAY, *mut *mut super::Com::SAFEARRAY, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEventClassesForIID: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICOMAdminCatalog2, ICOMAdminCatalog2_Vtbl, 0x790c6e0b_9194_4cc9_9426_a48a63185696);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICOMAdminCatalog2 {
    type Target = ICOMAdminCatalog;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICOMAdminCatalog2, windows_core::IUnknown, super::Com::IDispatch, ICOMAdminCatalog);
#[cfg(feature = "Win32_System_Com")]
impl ICOMAdminCatalog2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCollectionByQuery2<P0>(&self, bstrcollectionname: P0, pvarquerystrings: *const windows_core::VARIANT) -> windows_core::Result<super::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCollectionByQuery2)(windows_core::Interface::as_raw(self), bstrcollectionname.param().abi(), core::mem::transmute(pvarquerystrings), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetApplicationInstanceIDFromProcessID(&self, lprocessid: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetApplicationInstanceIDFromProcessID)(windows_core::Interface::as_raw(self), lprocessid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ShutdownApplicationInstances(&self, pvarapplicationinstanceid: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShutdownApplicationInstances)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarapplicationinstanceid)).ok()
    }
    pub unsafe fn PauseApplicationInstances(&self, pvarapplicationinstanceid: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PauseApplicationInstances)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarapplicationinstanceid)).ok()
    }
    pub unsafe fn ResumeApplicationInstances(&self, pvarapplicationinstanceid: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResumeApplicationInstances)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarapplicationinstanceid)).ok()
    }
    pub unsafe fn RecycleApplicationInstances(&self, pvarapplicationinstanceid: *const windows_core::VARIANT, lreasoncode: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RecycleApplicationInstances)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarapplicationinstanceid), lreasoncode).ok()
    }
    pub unsafe fn AreApplicationInstancesPaused(&self, pvarapplicationinstanceid: *const windows_core::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AreApplicationInstancesPaused)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarapplicationinstanceid), &mut result__).map(|| result__)
    }
    pub unsafe fn DumpApplicationInstance<P0, P1>(&self, bstrapplicationinstanceid: P0, bstrdirectory: P1, lmaximages: i32) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DumpApplicationInstance)(windows_core::Interface::as_raw(self), bstrapplicationinstanceid.param().abi(), bstrdirectory.param().abi(), lmaximages, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsApplicationInstanceDumpSupported(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsApplicationInstanceDumpSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CreateServiceForApplication<P0, P1, P2, P3, P4, P5, P6, P7>(&self, bstrapplicationidorname: P0, bstrservicename: P1, bstrstarttype: P2, bstrerrorcontrol: P3, bstrdependencies: P4, bstrrunas: P5, bstrpassword: P6, bdesktopok: P7) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
        P5: windows_core::Param<windows_core::BSTR>,
        P6: windows_core::Param<windows_core::BSTR>,
        P7: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).CreateServiceForApplication)(windows_core::Interface::as_raw(self), bstrapplicationidorname.param().abi(), bstrservicename.param().abi(), bstrstarttype.param().abi(), bstrerrorcontrol.param().abi(), bstrdependencies.param().abi(), bstrrunas.param().abi(), bstrpassword.param().abi(), bdesktopok.param().abi()).ok()
    }
    pub unsafe fn DeleteServiceForApplication<P0>(&self, bstrapplicationidorname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteServiceForApplication)(windows_core::Interface::as_raw(self), bstrapplicationidorname.param().abi()).ok()
    }
    pub unsafe fn GetPartitionID<P0>(&self, bstrapplicationidorname: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPartitionID)(windows_core::Interface::as_raw(self), bstrapplicationidorname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPartitionName<P0>(&self, bstrapplicationidorname: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPartitionName)(windows_core::Interface::as_raw(self), bstrapplicationidorname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCurrentPartition<P0>(&self, bstrpartitionidorname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCurrentPartition)(windows_core::Interface::as_raw(self), bstrpartitionidorname.param().abi()).ok()
    }
    pub unsafe fn CurrentPartitionID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentPartitionID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentPartitionName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentPartitionName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GlobalPartitionID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GlobalPartitionID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FlushPartitionCache(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FlushPartitionCache)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CopyApplications<P0, P1>(&self, bstrsourcepartitionidorname: P0, pvarapplicationid: *const windows_core::VARIANT, bstrdestinationpartitionidorname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).CopyApplications)(windows_core::Interface::as_raw(self), bstrsourcepartitionidorname.param().abi(), core::mem::transmute(pvarapplicationid), bstrdestinationpartitionidorname.param().abi()).ok()
    }
    pub unsafe fn CopyComponents<P0, P1>(&self, bstrsourceapplicationidorname: P0, pvarclsidorprogid: *const windows_core::VARIANT, bstrdestinationapplicationidorname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).CopyComponents)(windows_core::Interface::as_raw(self), bstrsourceapplicationidorname.param().abi(), core::mem::transmute(pvarclsidorprogid), bstrdestinationapplicationidorname.param().abi()).ok()
    }
    pub unsafe fn MoveComponents<P0, P1>(&self, bstrsourceapplicationidorname: P0, pvarclsidorprogid: *const windows_core::VARIANT, bstrdestinationapplicationidorname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).MoveComponents)(windows_core::Interface::as_raw(self), bstrsourceapplicationidorname.param().abi(), core::mem::transmute(pvarclsidorprogid), bstrdestinationapplicationidorname.param().abi()).ok()
    }
    pub unsafe fn AliasComponent<P0, P1, P2, P3, P4>(&self, bstrsrcapplicationidorname: P0, bstrclsidorprogid: P1, bstrdestapplicationidorname: P2, bstrnewprogid: P3, bstrnewclsid: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AliasComponent)(windows_core::Interface::as_raw(self), bstrsrcapplicationidorname.param().abi(), bstrclsidorprogid.param().abi(), bstrdestapplicationidorname.param().abi(), bstrnewprogid.param().abi(), bstrnewclsid.param().abi()).ok()
    }
    pub unsafe fn IsSafeToDelete<P0>(&self, bstrdllname: P0) -> windows_core::Result<COMAdminInUse>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSafeToDelete)(windows_core::Interface::as_raw(self), bstrdllname.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn ImportUnconfiguredComponents<P0>(&self, bstrapplicationidorname: P0, pvarclsidorprogid: *const windows_core::VARIANT, pvarcomponenttype: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ImportUnconfiguredComponents)(windows_core::Interface::as_raw(self), bstrapplicationidorname.param().abi(), core::mem::transmute(pvarclsidorprogid), core::mem::transmute(pvarcomponenttype)).ok()
    }
    pub unsafe fn PromoteUnconfiguredComponents<P0>(&self, bstrapplicationidorname: P0, pvarclsidorprogid: *const windows_core::VARIANT, pvarcomponenttype: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).PromoteUnconfiguredComponents)(windows_core::Interface::as_raw(self), bstrapplicationidorname.param().abi(), core::mem::transmute(pvarclsidorprogid), core::mem::transmute(pvarcomponenttype)).ok()
    }
    pub unsafe fn ImportComponents<P0>(&self, bstrapplicationidorname: P0, pvarclsidorprogid: *const windows_core::VARIANT, pvarcomponenttype: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ImportComponents)(windows_core::Interface::as_raw(self), bstrapplicationidorname.param().abi(), core::mem::transmute(pvarclsidorprogid), core::mem::transmute(pvarcomponenttype)).ok()
    }
    pub unsafe fn Is64BitCatalogServer(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Is64BitCatalogServer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ExportPartition<P0, P1>(&self, bstrpartitionidorname: P0, bstrpartitionfilename: P1, loptions: COMAdminApplicationExportOptions) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ExportPartition)(windows_core::Interface::as_raw(self), bstrpartitionidorname.param().abi(), bstrpartitionfilename.param().abi(), loptions).ok()
    }
    pub unsafe fn InstallPartition<P0, P1, P2, P3, P4>(&self, bstrfilename: P0, bstrdestdirectory: P1, loptions: COMAdminApplicationInstallOptions, bstruserid: P2, bstrpassword: P3, bstrrsn: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InstallPartition)(windows_core::Interface::as_raw(self), bstrfilename.param().abi(), bstrdestdirectory.param().abi(), loptions, bstruserid.param().abi(), bstrpassword.param().abi(), bstrrsn.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryApplicationFile2<P0>(&self, bstrapplicationfile: P0) -> windows_core::Result<super::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryApplicationFile2)(windows_core::Interface::as_raw(self), bstrapplicationfile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetComponentVersionCount<P0>(&self, bstrclsidorprogid: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetComponentVersionCount)(windows_core::Interface::as_raw(self), bstrclsidorprogid.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICOMAdminCatalog2_Vtbl {
    pub base__: ICOMAdminCatalog_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCollectionByQuery2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCollectionByQuery2: usize,
    pub GetApplicationInstanceIDFromProcessID: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ShutdownApplicationInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PauseApplicationInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ResumeApplicationInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub RecycleApplicationInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, i32) -> windows_core::HRESULT,
    pub AreApplicationInstancesPaused: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DumpApplicationInstance: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsApplicationInstanceDumpSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CreateServiceForApplication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DeleteServiceForApplication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPartitionID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPartitionName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCurrentPartition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentPartitionID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentPartitionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GlobalPartitionID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FlushPartitionCache: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyApplications: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CopyComponents: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MoveComponents: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AliasComponent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsSafeToDelete: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut COMAdminInUse) -> windows_core::HRESULT,
    pub ImportUnconfiguredComponents: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PromoteUnconfiguredComponents: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ImportComponents: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Is64BitCatalogServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ExportPartition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, COMAdminApplicationExportOptions) -> windows_core::HRESULT,
    pub InstallPartition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, COMAdminApplicationInstallOptions, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryApplicationFile2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryApplicationFile2: usize,
    pub GetComponentVersionCount: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICOMLBArguments, ICOMLBArguments_Vtbl, 0x3a0f150f_8ee5_4b94_b40e_aef2f9e42ed2);
impl core::ops::Deref for ICOMLBArguments {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICOMLBArguments, windows_core::IUnknown);
impl ICOMLBArguments {
    pub unsafe fn GetCLSID(&self, pclsid: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCLSID)(windows_core::Interface::as_raw(self), pclsid).ok()
    }
    pub unsafe fn SetCLSID(&self, pclsid: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCLSID)(windows_core::Interface::as_raw(self), pclsid).ok()
    }
    pub unsafe fn GetMachineName(&self, szservername: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMachineName)(windows_core::Interface::as_raw(self), szservername.len().try_into().unwrap(), core::mem::transmute(szservername.as_ptr())).ok()
    }
    pub unsafe fn SetMachineName(&self, szservername: &[u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMachineName)(windows_core::Interface::as_raw(self), szservername.len().try_into().unwrap(), core::mem::transmute(szservername.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct ICOMLBArguments_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetMachineName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetMachineName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICatalogCollection, ICatalogCollection_Vtbl, 0x6eb22872_8a19_11d0_81b6_00a0c9231c29);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICatalogCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICatalogCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICatalogCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Remove(&self, lindex: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), lindex).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Populate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Populate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SaveChanges(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SaveChanges)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCollection<P0, P1>(&self, bstrcollname: P0, varobjectkey: P1) -> windows_core::Result<super::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCollection)(windows_core::Interface::as_raw(self), bstrcollname.param().abi(), varobjectkey.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RemoveEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemoveEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUtilInterface(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUtilInterface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DataStoreMajorVersion(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataStoreMajorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DataStoreMinorVersion(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataStoreMinorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PopulateByKey(&self, psakeys: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PopulateByKey)(windows_core::Interface::as_raw(self), psakeys).ok()
    }
    pub unsafe fn PopulateByQuery<P0>(&self, bstrquerystring: P0, lquerytype: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).PopulateByQuery)(windows_core::Interface::as_raw(self), bstrquerystring.param().abi(), lquerytype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICatalogCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Populate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveChanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCollection: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCollection: usize,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RemoveEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUtilInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUtilInterface: usize,
    pub DataStoreMajorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DataStoreMinorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PopulateByKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PopulateByKey: usize,
    pub PopulateByQuery: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICatalogObject, ICatalogObject_Vtbl, 0x6eb22871_8a19_11d0_81b6_00a0c9231c29);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICatalogObject {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICatalogObject, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICatalogObject {
    pub unsafe fn get_Value<P0>(&self, bstrpropname: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), bstrpropname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_Value<P0, P1>(&self, bstrpropname: P0, val: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).put_Value)(windows_core::Interface::as_raw(self), bstrpropname.param().abi(), val.param().abi()).ok()
    }
    pub unsafe fn Key(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Key)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsPropertyReadOnly<P0>(&self, bstrpropname: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsPropertyReadOnly)(windows_core::Interface::as_raw(self), bstrpropname.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Valid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Valid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsPropertyWriteOnly<P0>(&self, bstrpropname: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsPropertyWriteOnly)(windows_core::Interface::as_raw(self), bstrpropname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICatalogObject_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub IsPropertyReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Valid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsPropertyWriteOnly: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICheckSxsConfig, ICheckSxsConfig_Vtbl, 0x0ff5a96f_11fc_47d1_baa6_25dd347e7242);
impl core::ops::Deref for ICheckSxsConfig {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICheckSxsConfig, windows_core::IUnknown);
impl ICheckSxsConfig {
    pub unsafe fn IsSameSxsConfig<P0, P1, P2>(&self, wszsxsname: P0, wszsxsdirectory: P1, wszsxsappname: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).IsSameSxsConfig)(windows_core::Interface::as_raw(self), wszsxsname.param().abi(), wszsxsdirectory.param().abi(), wszsxsappname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICheckSxsConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsSameSxsConfig: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComActivityEvents, IComActivityEvents_Vtbl, 0x683130b0_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for IComActivityEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComActivityEvents, windows_core::IUnknown);
impl IComActivityEvents {
    pub unsafe fn OnActivityCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnActivityCreate)(windows_core::Interface::as_raw(self), pinfo, guidactivity).ok()
    }
    pub unsafe fn OnActivityDestroy(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnActivityDestroy)(windows_core::Interface::as_raw(self), pinfo, guidactivity).ok()
    }
    pub unsafe fn OnActivityEnter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidentered: *const windows_core::GUID, dwthread: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnActivityEnter)(windows_core::Interface::as_raw(self), pinfo, guidcurrent, guidentered, dwthread).ok()
    }
    pub unsafe fn OnActivityTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidentered: *const windows_core::GUID, dwthread: u32, dwtimeout: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnActivityTimeout)(windows_core::Interface::as_raw(self), pinfo, guidcurrent, guidentered, dwthread, dwtimeout).ok()
    }
    pub unsafe fn OnActivityReenter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, dwthread: u32, dwcalldepth: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnActivityReenter)(windows_core::Interface::as_raw(self), pinfo, guidcurrent, dwthread, dwcalldepth).ok()
    }
    pub unsafe fn OnActivityLeave(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidleft: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnActivityLeave)(windows_core::Interface::as_raw(self), pinfo, guidcurrent, guidleft).ok()
    }
    pub unsafe fn OnActivityLeaveSame(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, dwcalldepth: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnActivityLeaveSame)(windows_core::Interface::as_raw(self), pinfo, guidcurrent, dwcalldepth).ok()
    }
}
#[repr(C)]
pub struct IComActivityEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnActivityCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnActivityDestroy: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnActivityEnter: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub OnActivityTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, u32, u32) -> windows_core::HRESULT,
    pub OnActivityReenter: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, u32, u32) -> windows_core::HRESULT,
    pub OnActivityLeave: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnActivityLeaveSame: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComApp2Events, IComApp2Events_Vtbl, 0x1290bc1a_b219_418d_b078_5934ded08242);
impl core::ops::Deref for IComApp2Events {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComApp2Events, windows_core::IUnknown);
impl IComApp2Events {
    pub unsafe fn OnAppActivation2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID, guidprocess: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnAppActivation2)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp), core::mem::transmute(guidprocess)).ok()
    }
    pub unsafe fn OnAppShutdown2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnAppShutdown2)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp)).ok()
    }
    pub unsafe fn OnAppForceShutdown2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnAppForceShutdown2)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp)).ok()
    }
    pub unsafe fn OnAppPaused2<P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID, bpaused: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnAppPaused2)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp), bpaused.param().abi()).ok()
    }
    pub unsafe fn OnAppRecycle2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID, guidprocess: windows_core::GUID, lreason: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnAppRecycle2)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp), core::mem::transmute(guidprocess), lreason).ok()
    }
}
#[repr(C)]
pub struct IComApp2Events_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnAppActivation2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, windows_core::GUID) -> windows_core::HRESULT,
    pub OnAppShutdown2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnAppForceShutdown2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnAppPaused2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnAppRecycle2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, windows_core::GUID, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComAppEvents, IComAppEvents_Vtbl, 0x683130a6_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for IComAppEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComAppEvents, windows_core::IUnknown);
impl IComAppEvents {
    pub unsafe fn OnAppActivation(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnAppActivation)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp)).ok()
    }
    pub unsafe fn OnAppShutdown(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnAppShutdown)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp)).ok()
    }
    pub unsafe fn OnAppForceShutdown(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnAppForceShutdown)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp)).ok()
    }
}
#[repr(C)]
pub struct IComAppEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnAppActivation: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnAppShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnAppForceShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComCRMEvents, IComCRMEvents_Vtbl, 0x683130b5_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for IComCRMEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComCRMEvents, windows_core::IUnknown);
impl IComCRMEvents {
    pub unsafe fn OnCRMRecoveryStart(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCRMRecoveryStart)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp)).ok()
    }
    pub unsafe fn OnCRMRecoveryDone(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCRMRecoveryDone)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp)).ok()
    }
    pub unsafe fn OnCRMCheckpoint(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCRMCheckpoint)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp)).ok()
    }
    pub unsafe fn OnCRMBegin(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, guidactivity: windows_core::GUID, guidtx: windows_core::GUID, szprogidcompensator: &[u16; 64], szdescription: &[u16; 64]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCRMBegin)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid), core::mem::transmute(guidactivity), core::mem::transmute(guidtx), core::mem::transmute(szprogidcompensator.as_ptr()), core::mem::transmute(szdescription.as_ptr())).ok()
    }
    pub unsafe fn OnCRMPrepare(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCRMPrepare)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid)).ok()
    }
    pub unsafe fn OnCRMCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCRMCommit)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid)).ok()
    }
    pub unsafe fn OnCRMAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCRMAbort)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid)).ok()
    }
    pub unsafe fn OnCRMIndoubt(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCRMIndoubt)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid)).ok()
    }
    pub unsafe fn OnCRMDone(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCRMDone)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid)).ok()
    }
    pub unsafe fn OnCRMRelease(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCRMRelease)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid)).ok()
    }
    pub unsafe fn OnCRMAnalyze(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCRMAnalyze)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid), dwcrmrecordtype, dwrecordsize).ok()
    }
    pub unsafe fn OnCRMWrite<P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, fvariants: P0, dwrecordsize: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnCRMWrite)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid), fvariants.param().abi(), dwrecordsize).ok()
    }
    pub unsafe fn OnCRMForget(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCRMForget)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid)).ok()
    }
    pub unsafe fn OnCRMForce(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCRMForce)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid)).ok()
    }
    pub unsafe fn OnCRMDeliver<P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, fvariants: P0, dwrecordsize: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnCRMDeliver)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid), fvariants.param().abi(), dwrecordsize).ok()
    }
}
#[repr(C)]
pub struct IComCRMEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCRMRecoveryStart: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMRecoveryDone: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMCheckpoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMBegin: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, windows_core::GUID, windows_core::GUID, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnCRMPrepare: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMCommit: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMAbort: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMIndoubt: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMDone: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMRelease: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMAnalyze: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, u32, u32) -> windows_core::HRESULT,
    pub OnCRMWrite: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, super::super::Foundation::BOOL, u32) -> windows_core::HRESULT,
    pub OnCRMForget: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMForce: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMDeliver: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, super::super::Foundation::BOOL, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComExceptionEvents, IComExceptionEvents_Vtbl, 0x683130b3_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for IComExceptionEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComExceptionEvents, windows_core::IUnknown);
impl IComExceptionEvents {
    pub unsafe fn OnExceptionUser<P0>(&self, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnExceptionUser)(windows_core::Interface::as_raw(self), pinfo, code, address, pszstacktrace.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IComExceptionEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnExceptionUser: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u32, u64, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComIdentityEvents, IComIdentityEvents_Vtbl, 0x683130b1_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for IComIdentityEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComIdentityEvents, windows_core::IUnknown);
impl IComIdentityEvents {
    pub unsafe fn OnIISRequestInfo<P0, P1, P2>(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: P0, pszserverip: P1, pszurl: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnIISRequestInfo)(windows_core::Interface::as_raw(self), pinfo, objid, pszclientip.param().abi(), pszserverip.param().abi(), pszurl.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IComIdentityEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnIISRequestInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComInstance2Events, IComInstance2Events_Vtbl, 0x20e3bf07_b506_4ad5_a50c_d2ca5b9c158e);
impl core::ops::Deref for IComInstance2Events {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComInstance2Events, windows_core::IUnknown);
impl IComInstance2Events {
    pub unsafe fn OnObjectCreate2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, clsid: *const windows_core::GUID, tsid: *const windows_core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjectCreate2)(windows_core::Interface::as_raw(self), pinfo, guidactivity, clsid, tsid, ctxtid, objectid, guidpartition).ok()
    }
    pub unsafe fn OnObjectDestroy2(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjectDestroy2)(windows_core::Interface::as_raw(self), pinfo, ctxtid).ok()
    }
}
#[repr(C)]
pub struct IComInstance2Events_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnObjectCreate2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, u64, u64, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnObjectDestroy2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComInstanceEvents, IComInstanceEvents_Vtbl, 0x683130a7_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for IComInstanceEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComInstanceEvents, windows_core::IUnknown);
impl IComInstanceEvents {
    pub unsafe fn OnObjectCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, clsid: *const windows_core::GUID, tsid: *const windows_core::GUID, ctxtid: u64, objectid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjectCreate)(windows_core::Interface::as_raw(self), pinfo, guidactivity, clsid, tsid, ctxtid, objectid).ok()
    }
    pub unsafe fn OnObjectDestroy(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjectDestroy)(windows_core::Interface::as_raw(self), pinfo, ctxtid).ok()
    }
}
#[repr(C)]
pub struct IComInstanceEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnObjectCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, u64, u64) -> windows_core::HRESULT,
    pub OnObjectDestroy: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComLTxEvents, IComLTxEvents_Vtbl, 0x605cf82c_578e_4298_975d_82babcd9e053);
impl core::ops::Deref for IComLTxEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComLTxEvents, windows_core::IUnknown);
impl IComLTxEvents {
    pub unsafe fn OnLtxTransactionStart<P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID, tsid: windows_core::GUID, froot: P0, nisolationlevel: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnLtxTransactionStart)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidltx), core::mem::transmute(tsid), froot.param().abi(), nisolationlevel).ok()
    }
    pub unsafe fn OnLtxTransactionPrepare<P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID, fvote: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnLtxTransactionPrepare)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidltx), fvote.param().abi()).ok()
    }
    pub unsafe fn OnLtxTransactionAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnLtxTransactionAbort)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidltx)).ok()
    }
    pub unsafe fn OnLtxTransactionCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnLtxTransactionCommit)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidltx)).ok()
    }
    pub unsafe fn OnLtxTransactionPromote(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID, txnid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnLtxTransactionPromote)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidltx), core::mem::transmute(txnid)).ok()
    }
}
#[repr(C)]
pub struct IComLTxEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnLtxTransactionStart: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, windows_core::GUID, super::super::Foundation::BOOL, i32) -> windows_core::HRESULT,
    pub OnLtxTransactionPrepare: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnLtxTransactionAbort: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnLtxTransactionCommit: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnLtxTransactionPromote: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComMethod2Events, IComMethod2Events_Vtbl, 0xfb388aaa_567d_4024_af8e_6e93ee748573);
impl core::ops::Deref for IComMethod2Events {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComMethod2Events, windows_core::IUnknown);
impl IComMethod2Events {
    pub unsafe fn OnMethodCall2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnMethodCall2)(windows_core::Interface::as_raw(self), pinfo, oid, guidcid, guidrid, dwthread, imeth).ok()
    }
    pub unsafe fn OnMethodReturn2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32, hresult: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnMethodReturn2)(windows_core::Interface::as_raw(self), pinfo, oid, guidcid, guidrid, dwthread, imeth, hresult).ok()
    }
    pub unsafe fn OnMethodException2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnMethodException2)(windows_core::Interface::as_raw(self), pinfo, oid, guidcid, guidrid, dwthread, imeth).ok()
    }
}
#[repr(C)]
pub struct IComMethod2Events_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnMethodCall2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, *const windows_core::GUID, *const windows_core::GUID, u32, u32) -> windows_core::HRESULT,
    pub OnMethodReturn2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, *const windows_core::GUID, *const windows_core::GUID, u32, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnMethodException2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, *const windows_core::GUID, *const windows_core::GUID, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComMethodEvents, IComMethodEvents_Vtbl, 0x683130a9_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for IComMethodEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComMethodEvents, windows_core::IUnknown);
impl IComMethodEvents {
    pub unsafe fn OnMethodCall(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnMethodCall)(windows_core::Interface::as_raw(self), pinfo, oid, guidcid, guidrid, imeth).ok()
    }
    pub unsafe fn OnMethodReturn(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32, hresult: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnMethodReturn)(windows_core::Interface::as_raw(self), pinfo, oid, guidcid, guidrid, imeth, hresult).ok()
    }
    pub unsafe fn OnMethodException(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnMethodException)(windows_core::Interface::as_raw(self), pinfo, oid, guidcid, guidrid, imeth).ok()
    }
}
#[repr(C)]
pub struct IComMethodEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnMethodCall: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, *const windows_core::GUID, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub OnMethodReturn: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, *const windows_core::GUID, *const windows_core::GUID, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnMethodException: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, *const windows_core::GUID, *const windows_core::GUID, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComMtaThreadPoolKnobs, IComMtaThreadPoolKnobs_Vtbl, 0xf9a76d2e_76a5_43eb_a0c4_49bec8e48480);
impl core::ops::Deref for IComMtaThreadPoolKnobs {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComMtaThreadPoolKnobs, windows_core::IUnknown);
impl IComMtaThreadPoolKnobs {
    pub unsafe fn MTASetMaxThreadCount(&self, dwmaxthreads: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MTASetMaxThreadCount)(windows_core::Interface::as_raw(self), dwmaxthreads).ok()
    }
    pub unsafe fn MTAGetMaxThreadCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MTAGetMaxThreadCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MTASetThrottleValue(&self, dwthrottle: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MTASetThrottleValue)(windows_core::Interface::as_raw(self), dwthrottle).ok()
    }
    pub unsafe fn MTAGetThrottleValue(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MTAGetThrottleValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IComMtaThreadPoolKnobs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MTASetMaxThreadCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MTAGetMaxThreadCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MTASetThrottleValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MTAGetThrottleValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComObjectConstruction2Events, IComObjectConstruction2Events_Vtbl, 0x4b5a7827_8df2_45c0_8f6f_57ea1f856a9f);
impl core::ops::Deref for IComObjectConstruction2Events {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComObjectConstruction2Events, windows_core::IUnknown);
impl IComObjectConstruction2Events {
    pub unsafe fn OnObjectConstruct2<P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, sconstructstring: P0, oid: u64, guidpartition: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnObjectConstruct2)(windows_core::Interface::as_raw(self), pinfo, guidobject, sconstructstring.param().abi(), oid, guidpartition).ok()
    }
}
#[repr(C)]
pub struct IComObjectConstruction2Events_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnObjectConstruct2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, windows_core::PCWSTR, u64, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComObjectConstructionEvents, IComObjectConstructionEvents_Vtbl, 0x683130af_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for IComObjectConstructionEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComObjectConstructionEvents, windows_core::IUnknown);
impl IComObjectConstructionEvents {
    pub unsafe fn OnObjectConstruct<P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, sconstructstring: P0, oid: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnObjectConstruct)(windows_core::Interface::as_raw(self), pinfo, guidobject, sconstructstring.param().abi(), oid).ok()
    }
}
#[repr(C)]
pub struct IComObjectConstructionEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnObjectConstruct: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, windows_core::PCWSTR, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComObjectEvents, IComObjectEvents_Vtbl, 0x683130aa_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for IComObjectEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComObjectEvents, windows_core::IUnknown);
impl IComObjectEvents {
    pub unsafe fn OnObjectActivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjectActivate)(windows_core::Interface::as_raw(self), pinfo, ctxtid, objectid).ok()
    }
    pub unsafe fn OnObjectDeactivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjectDeactivate)(windows_core::Interface::as_raw(self), pinfo, ctxtid, objectid).ok()
    }
    pub unsafe fn OnDisableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnDisableCommit)(windows_core::Interface::as_raw(self), pinfo, ctxtid).ok()
    }
    pub unsafe fn OnEnableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnEnableCommit)(windows_core::Interface::as_raw(self), pinfo, ctxtid).ok()
    }
    pub unsafe fn OnSetComplete(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnSetComplete)(windows_core::Interface::as_raw(self), pinfo, ctxtid).ok()
    }
    pub unsafe fn OnSetAbort(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnSetAbort)(windows_core::Interface::as_raw(self), pinfo, ctxtid).ok()
    }
}
#[repr(C)]
pub struct IComObjectEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnObjectActivate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64) -> windows_core::HRESULT,
    pub OnObjectDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64) -> windows_core::HRESULT,
    pub OnDisableCommit: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64) -> windows_core::HRESULT,
    pub OnEnableCommit: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64) -> windows_core::HRESULT,
    pub OnSetComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64) -> windows_core::HRESULT,
    pub OnSetAbort: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComObjectPool2Events, IComObjectPool2Events_Vtbl, 0x65bf6534_85ea_4f64_8cf4_3d974b2ab1cf);
impl core::ops::Deref for IComObjectPool2Events {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComObjectPool2Events, windows_core::IUnknown);
impl IComObjectPool2Events {
    pub unsafe fn OnObjPoolPutObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjPoolPutObject2)(windows_core::Interface::as_raw(self), pinfo, guidobject, nreason, dwavailable, oid).ok()
    }
    pub unsafe fn OnObjPoolGetObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, dwavailable: u32, oid: u64, guidpartition: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjPoolGetObject2)(windows_core::Interface::as_raw(self), pinfo, guidactivity, guidobject, dwavailable, oid, guidpartition).ok()
    }
    pub unsafe fn OnObjPoolRecycleToTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjPoolRecycleToTx2)(windows_core::Interface::as_raw(self), pinfo, guidactivity, guidobject, guidtx, objid).ok()
    }
    pub unsafe fn OnObjPoolGetFromTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64, guidpartition: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjPoolGetFromTx2)(windows_core::Interface::as_raw(self), pinfo, guidactivity, guidobject, guidtx, objid, guidpartition).ok()
    }
}
#[repr(C)]
pub struct IComObjectPool2Events_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnObjPoolPutObject2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, i32, u32, u64) -> windows_core::HRESULT,
    pub OnObjPoolGetObject2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, u32, u64, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnObjPoolRecycleToTx2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, u64) -> windows_core::HRESULT,
    pub OnObjPoolGetFromTx2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, u64, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComObjectPoolEvents, IComObjectPoolEvents_Vtbl, 0x683130ad_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for IComObjectPoolEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComObjectPoolEvents, windows_core::IUnknown);
impl IComObjectPoolEvents {
    pub unsafe fn OnObjPoolPutObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjPoolPutObject)(windows_core::Interface::as_raw(self), pinfo, guidobject, nreason, dwavailable, oid).ok()
    }
    pub unsafe fn OnObjPoolGetObject(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, dwavailable: u32, oid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjPoolGetObject)(windows_core::Interface::as_raw(self), pinfo, guidactivity, guidobject, dwavailable, oid).ok()
    }
    pub unsafe fn OnObjPoolRecycleToTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjPoolRecycleToTx)(windows_core::Interface::as_raw(self), pinfo, guidactivity, guidobject, guidtx, objid).ok()
    }
    pub unsafe fn OnObjPoolGetFromTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjPoolGetFromTx)(windows_core::Interface::as_raw(self), pinfo, guidactivity, guidobject, guidtx, objid).ok()
    }
}
#[repr(C)]
pub struct IComObjectPoolEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnObjPoolPutObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, i32, u32, u64) -> windows_core::HRESULT,
    pub OnObjPoolGetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, u32, u64) -> windows_core::HRESULT,
    pub OnObjPoolRecycleToTx: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, u64) -> windows_core::HRESULT,
    pub OnObjPoolGetFromTx: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComObjectPoolEvents2, IComObjectPoolEvents2_Vtbl, 0x683130ae_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for IComObjectPoolEvents2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComObjectPoolEvents2, windows_core::IUnknown);
impl IComObjectPoolEvents2 {
    pub unsafe fn OnObjPoolCreateObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwobjscreated: u32, oid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjPoolCreateObject)(windows_core::Interface::as_raw(self), pinfo, guidobject, dwobjscreated, oid).ok()
    }
    pub unsafe fn OnObjPoolDestroyObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwobjscreated: u32, oid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjPoolDestroyObject)(windows_core::Interface::as_raw(self), pinfo, guidobject, dwobjscreated, oid).ok()
    }
    pub unsafe fn OnObjPoolCreateDecision(&self, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjPoolCreateDecision)(windows_core::Interface::as_raw(self), pinfo, dwthreadswaiting, dwavail, dwcreated, dwmin, dwmax).ok()
    }
    pub unsafe fn OnObjPoolTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, guidactivity: *const windows_core::GUID, dwtimeout: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjPoolTimeout)(windows_core::Interface::as_raw(self), pinfo, guidobject, guidactivity, dwtimeout).ok()
    }
    pub unsafe fn OnObjPoolCreatePool(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnObjPoolCreatePool)(windows_core::Interface::as_raw(self), pinfo, guidobject, dwmin, dwmax, dwtimeout).ok()
    }
}
#[repr(C)]
pub struct IComObjectPoolEvents2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnObjPoolCreateObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, u32, u64) -> windows_core::HRESULT,
    pub OnObjPoolDestroyObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, u32, u64) -> windows_core::HRESULT,
    pub OnObjPoolCreateDecision: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u32, u32, u32, u32, u32) -> windows_core::HRESULT,
    pub OnObjPoolTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub OnObjPoolCreatePool: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, u32, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComQCEvents, IComQCEvents_Vtbl, 0x683130b2_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for IComQCEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComQCEvents, windows_core::IUnknown);
impl IComQCEvents {
    pub unsafe fn OnQCRecord(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: &[u16; 60], guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, msmqhr: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnQCRecord)(windows_core::Interface::as_raw(self), pinfo, objid, core::mem::transmute(szqueue.as_ptr()), guidmsgid, guidworkflowid, msmqhr).ok()
    }
    pub unsafe fn OnQCQueueOpen(&self, pinfo: *const COMSVCSEVENTINFO, szqueue: &[u16; 60], queueid: u64, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnQCQueueOpen)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(szqueue.as_ptr()), queueid, hr).ok()
    }
    pub unsafe fn OnQCReceive(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnQCReceive)(windows_core::Interface::as_raw(self), pinfo, queueid, guidmsgid, guidworkflowid, hr).ok()
    }
    pub unsafe fn OnQCReceiveFail(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnQCReceiveFail)(windows_core::Interface::as_raw(self), pinfo, queueid, msmqhr).ok()
    }
    pub unsafe fn OnQCMoveToReTryQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, retryindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnQCMoveToReTryQueue)(windows_core::Interface::as_raw(self), pinfo, guidmsgid, guidworkflowid, retryindex).ok()
    }
    pub unsafe fn OnQCMoveToDeadQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnQCMoveToDeadQueue)(windows_core::Interface::as_raw(self), pinfo, guidmsgid, guidworkflowid).ok()
    }
    pub unsafe fn OnQCPlayback(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnQCPlayback)(windows_core::Interface::as_raw(self), pinfo, objid, guidmsgid, guidworkflowid, hr).ok()
    }
}
#[repr(C)]
pub struct IComQCEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnQCRecord: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, windows_core::PCWSTR, *const windows_core::GUID, *const windows_core::GUID, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnQCQueueOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::PCWSTR, u64, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnQCReceive: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, *const windows_core::GUID, *const windows_core::GUID, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnQCReceiveFail: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnQCMoveToReTryQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub OnQCMoveToDeadQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnQCPlayback: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, *const windows_core::GUID, *const windows_core::GUID, windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComResourceEvents, IComResourceEvents_Vtbl, 0x683130ab_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for IComResourceEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComResourceEvents, windows_core::IUnknown);
impl IComResourceEvents {
    pub unsafe fn OnResourceCreate<P0, P1>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: P0, resid: u64, enlisted: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnResourceCreate)(windows_core::Interface::as_raw(self), pinfo, objectid, psztype.param().abi(), resid, enlisted.param().abi()).ok()
    }
    pub unsafe fn OnResourceAllocate<P0, P1>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: P0, resid: u64, enlisted: P1, numrated: u32, rating: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnResourceAllocate)(windows_core::Interface::as_raw(self), pinfo, objectid, psztype.param().abi(), resid, enlisted.param().abi(), numrated, rating).ok()
    }
    pub unsafe fn OnResourceRecycle<P0>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: P0, resid: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnResourceRecycle)(windows_core::Interface::as_raw(self), pinfo, objectid, psztype.param().abi(), resid).ok()
    }
    pub unsafe fn OnResourceDestroy<P0>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: windows_core::HRESULT, psztype: P0, resid: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnResourceDestroy)(windows_core::Interface::as_raw(self), pinfo, objectid, hr, psztype.param().abi(), resid).ok()
    }
    pub unsafe fn OnResourceTrack<P0, P1>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: P0, resid: u64, enlisted: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnResourceTrack)(windows_core::Interface::as_raw(self), pinfo, objectid, psztype.param().abi(), resid, enlisted.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IComResourceEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnResourceCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, windows_core::PCWSTR, u64, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnResourceAllocate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, windows_core::PCWSTR, u64, super::super::Foundation::BOOL, u32, u32) -> windows_core::HRESULT,
    pub OnResourceRecycle: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, windows_core::PCWSTR, u64) -> windows_core::HRESULT,
    pub OnResourceDestroy: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, windows_core::HRESULT, windows_core::PCWSTR, u64) -> windows_core::HRESULT,
    pub OnResourceTrack: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, windows_core::PCWSTR, u64, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComSecurityEvents, IComSecurityEvents_Vtbl, 0x683130ac_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for IComSecurityEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComSecurityEvents, windows_core::IUnknown);
impl IComSecurityEvents {
    pub unsafe fn OnAuthenticate<P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, objectid: u64, guidiid: *const windows_core::GUID, imeth: u32, psidoriginaluser: &[u8], psidcurrentuser: &[u8], bcurrentuserinpersonatinginproc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnAuthenticate)(windows_core::Interface::as_raw(self), pinfo, guidactivity, objectid, guidiid, imeth, psidoriginaluser.len().try_into().unwrap(), core::mem::transmute(psidoriginaluser.as_ptr()), psidcurrentuser.len().try_into().unwrap(), core::mem::transmute(psidcurrentuser.as_ptr()), bcurrentuserinpersonatinginproc.param().abi()).ok()
    }
    pub unsafe fn OnAuthenticateFail<P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, objectid: u64, guidiid: *const windows_core::GUID, imeth: u32, psidoriginaluser: &[u8], psidcurrentuser: &[u8], bcurrentuserinpersonatinginproc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnAuthenticateFail)(windows_core::Interface::as_raw(self), pinfo, guidactivity, objectid, guidiid, imeth, psidoriginaluser.len().try_into().unwrap(), core::mem::transmute(psidoriginaluser.as_ptr()), psidcurrentuser.len().try_into().unwrap(), core::mem::transmute(psidcurrentuser.as_ptr()), bcurrentuserinpersonatinginproc.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IComSecurityEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnAuthenticate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, u64, *const windows_core::GUID, u32, u32, *const u8, u32, *const u8, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnAuthenticateFail: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, u64, *const windows_core::GUID, u32, u32, *const u8, u32, *const u8, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComStaThreadPoolKnobs, IComStaThreadPoolKnobs_Vtbl, 0x324b64fa_33b6_11d2_98b7_00c04f8ee1c4);
impl core::ops::Deref for IComStaThreadPoolKnobs {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComStaThreadPoolKnobs, windows_core::IUnknown);
impl IComStaThreadPoolKnobs {
    pub unsafe fn SetMinThreadCount(&self, minthreads: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinThreadCount)(windows_core::Interface::as_raw(self), minthreads).ok()
    }
    pub unsafe fn GetMinThreadCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMinThreadCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxThreadCount(&self, maxthreads: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxThreadCount)(windows_core::Interface::as_raw(self), maxthreads).ok()
    }
    pub unsafe fn GetMaxThreadCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxThreadCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetActivityPerThread(&self, activitiesperthread: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetActivityPerThread)(windows_core::Interface::as_raw(self), activitiesperthread).ok()
    }
    pub unsafe fn GetActivityPerThread(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActivityPerThread)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetActivityRatio(&self, activityratio: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetActivityRatio)(windows_core::Interface::as_raw(self), activityratio).ok()
    }
    pub unsafe fn GetActivityRatio(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActivityRatio)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetThreadCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetThreadCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetQueueDepth(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetQueueDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQueueDepth(&self, dwqdepth: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQueueDepth)(windows_core::Interface::as_raw(self), dwqdepth).ok()
    }
}
#[repr(C)]
pub struct IComStaThreadPoolKnobs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMinThreadCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMinThreadCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxThreadCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaxThreadCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetActivityPerThread: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetActivityPerThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetActivityRatio: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetActivityRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetThreadCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetQueueDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetQueueDepth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComStaThreadPoolKnobs2, IComStaThreadPoolKnobs2_Vtbl, 0x73707523_ff9a_4974_bf84_2108dc213740);
impl core::ops::Deref for IComStaThreadPoolKnobs2 {
    type Target = IComStaThreadPoolKnobs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComStaThreadPoolKnobs2, windows_core::IUnknown, IComStaThreadPoolKnobs);
impl IComStaThreadPoolKnobs2 {
    pub unsafe fn GetMaxCPULoad(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxCPULoad)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxCPULoad(&self, pdwload: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxCPULoad)(windows_core::Interface::as_raw(self), pdwload).ok()
    }
    pub unsafe fn GetCPUMetricEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCPUMetricEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCPUMetricEnabled<P0>(&self, bmetricenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetCPUMetricEnabled)(windows_core::Interface::as_raw(self), bmetricenabled.param().abi()).ok()
    }
    pub unsafe fn GetCreateThreadsAggressively(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCreateThreadsAggressively)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCreateThreadsAggressively<P0>(&self, bmetricenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetCreateThreadsAggressively)(windows_core::Interface::as_raw(self), bmetricenabled.param().abi()).ok()
    }
    pub unsafe fn GetMaxCSR(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxCSR)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxCSR(&self, dwcsr: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxCSR)(windows_core::Interface::as_raw(self), dwcsr).ok()
    }
    pub unsafe fn GetWaitTimeForThreadCleanup(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWaitTimeForThreadCleanup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetWaitTimeForThreadCleanup(&self, dwthreadcleanupwaittime: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWaitTimeForThreadCleanup)(windows_core::Interface::as_raw(self), dwthreadcleanupwaittime).ok()
    }
}
#[repr(C)]
pub struct IComStaThreadPoolKnobs2_Vtbl {
    pub base__: IComStaThreadPoolKnobs_Vtbl,
    pub GetMaxCPULoad: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxCPULoad: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCPUMetricEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetCPUMetricEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetCreateThreadsAggressively: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetCreateThreadsAggressively: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetMaxCSR: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxCSR: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetWaitTimeForThreadCleanup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetWaitTimeForThreadCleanup: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComThreadEvents, IComThreadEvents_Vtbl, 0x683130a5_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for IComThreadEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComThreadEvents, windows_core::IUnknown);
impl IComThreadEvents {
    pub unsafe fn OnThreadStart(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnThreadStart)(windows_core::Interface::as_raw(self), pinfo, threadid, dwthread, dwtheadcnt).ok()
    }
    pub unsafe fn OnThreadTerminate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnThreadTerminate)(windows_core::Interface::as_raw(self), pinfo, threadid, dwthread, dwtheadcnt).ok()
    }
    pub unsafe fn OnThreadBindToApartment(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnThreadBindToApartment)(windows_core::Interface::as_raw(self), pinfo, threadid, aptid, dwactcnt, dwlowcnt).ok()
    }
    pub unsafe fn OnThreadUnBind(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnThreadUnBind)(windows_core::Interface::as_raw(self), pinfo, threadid, aptid, dwactcnt).ok()
    }
    pub unsafe fn OnThreadWorkEnque(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnThreadWorkEnque)(windows_core::Interface::as_raw(self), pinfo, threadid, msgworkid, queuelen).ok()
    }
    pub unsafe fn OnThreadWorkPrivate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnThreadWorkPrivate)(windows_core::Interface::as_raw(self), pinfo, threadid, msgworkid).ok()
    }
    pub unsafe fn OnThreadWorkPublic(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnThreadWorkPublic)(windows_core::Interface::as_raw(self), pinfo, threadid, msgworkid, queuelen).ok()
    }
    pub unsafe fn OnThreadWorkRedirect(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnThreadWorkRedirect)(windows_core::Interface::as_raw(self), pinfo, threadid, msgworkid, queuelen, threadnum).ok()
    }
    pub unsafe fn OnThreadWorkReject(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnThreadWorkReject)(windows_core::Interface::as_raw(self), pinfo, threadid, msgworkid, queuelen).ok()
    }
    pub unsafe fn OnThreadAssignApartment(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, aptid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnThreadAssignApartment)(windows_core::Interface::as_raw(self), pinfo, guidactivity, aptid).ok()
    }
    pub unsafe fn OnThreadUnassignApartment(&self, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnThreadUnassignApartment)(windows_core::Interface::as_raw(self), pinfo, aptid).ok()
    }
}
#[repr(C)]
pub struct IComThreadEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnThreadStart: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u32, u32) -> windows_core::HRESULT,
    pub OnThreadTerminate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u32, u32) -> windows_core::HRESULT,
    pub OnThreadBindToApartment: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64, u32, u32) -> windows_core::HRESULT,
    pub OnThreadUnBind: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64, u32) -> windows_core::HRESULT,
    pub OnThreadWorkEnque: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64, u32) -> windows_core::HRESULT,
    pub OnThreadWorkPrivate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64) -> windows_core::HRESULT,
    pub OnThreadWorkPublic: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64, u32) -> windows_core::HRESULT,
    pub OnThreadWorkRedirect: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64, u32, u64) -> windows_core::HRESULT,
    pub OnThreadWorkReject: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64, u32) -> windows_core::HRESULT,
    pub OnThreadAssignApartment: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, u64) -> windows_core::HRESULT,
    pub OnThreadUnassignApartment: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComTrackingInfoCollection, IComTrackingInfoCollection_Vtbl, 0xc266c677_c9ad_49ab_9fd9_d9661078588a);
impl core::ops::Deref for IComTrackingInfoCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComTrackingInfoCollection, windows_core::IUnknown);
impl IComTrackingInfoCollection {
    pub unsafe fn Type(&self) -> windows_core::Result<TRACKING_COLL_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Item(&self, ulindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), ulindex, riid, ppv).ok()
    }
}
#[repr(C)]
pub struct IComTrackingInfoCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TRACKING_COLL_TYPE) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComTrackingInfoEvents, IComTrackingInfoEvents_Vtbl, 0x4e6cdcc9_fb25_4fd5_9cc5_c9f4b6559cec);
impl core::ops::Deref for IComTrackingInfoEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComTrackingInfoEvents, windows_core::IUnknown);
impl IComTrackingInfoEvents {
    pub unsafe fn OnNewTrackingInfo<P0>(&self, ptoplevelcollection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OnNewTrackingInfo)(windows_core::Interface::as_raw(self), ptoplevelcollection.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IComTrackingInfoEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnNewTrackingInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComTrackingInfoObject, IComTrackingInfoObject_Vtbl, 0x116e42c5_d8b1_47bf_ab1e_c895ed3e2372);
impl core::ops::Deref for IComTrackingInfoObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComTrackingInfoObject, windows_core::IUnknown);
impl IComTrackingInfoObject {
    pub unsafe fn GetValue<P0>(&self, szpropertyname: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), szpropertyname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IComTrackingInfoObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComTrackingInfoProperties, IComTrackingInfoProperties_Vtbl, 0x789b42be_6f6b_443a_898e_67abf390aa14);
impl core::ops::Deref for IComTrackingInfoProperties {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComTrackingInfoProperties, windows_core::IUnknown);
impl IComTrackingInfoProperties {
    pub unsafe fn PropCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPropName(&self, ulindex: u32) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropName)(windows_core::Interface::as_raw(self), ulindex, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IComTrackingInfoProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PropCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPropName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComTransaction2Events, IComTransaction2Events_Vtbl, 0xa136f62a_2f94_4288_86e0_d8a1fa4c0299);
impl core::ops::Deref for IComTransaction2Events {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComTransaction2Events, windows_core::IUnknown);
impl IComTransaction2Events {
    pub unsafe fn OnTransactionStart2<P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, tsid: *const windows_core::GUID, froot: P0, nisolationlevel: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnTransactionStart2)(windows_core::Interface::as_raw(self), pinfo, guidtx, tsid, froot.param().abi(), nisolationlevel).ok()
    }
    pub unsafe fn OnTransactionPrepare2<P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, fvoteyes: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnTransactionPrepare2)(windows_core::Interface::as_raw(self), pinfo, guidtx, fvoteyes.param().abi()).ok()
    }
    pub unsafe fn OnTransactionAbort2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnTransactionAbort2)(windows_core::Interface::as_raw(self), pinfo, guidtx).ok()
    }
    pub unsafe fn OnTransactionCommit2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnTransactionCommit2)(windows_core::Interface::as_raw(self), pinfo, guidtx).ok()
    }
}
#[repr(C)]
pub struct IComTransaction2Events_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnTransactionStart2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, super::super::Foundation::BOOL, i32) -> windows_core::HRESULT,
    pub OnTransactionPrepare2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnTransactionAbort2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnTransactionCommit2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComTransactionEvents, IComTransactionEvents_Vtbl, 0x683130a8_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for IComTransactionEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComTransactionEvents, windows_core::IUnknown);
impl IComTransactionEvents {
    pub unsafe fn OnTransactionStart<P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, tsid: *const windows_core::GUID, froot: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnTransactionStart)(windows_core::Interface::as_raw(self), pinfo, guidtx, tsid, froot.param().abi()).ok()
    }
    pub unsafe fn OnTransactionPrepare<P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, fvoteyes: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnTransactionPrepare)(windows_core::Interface::as_raw(self), pinfo, guidtx, fvoteyes.param().abi()).ok()
    }
    pub unsafe fn OnTransactionAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnTransactionAbort)(windows_core::Interface::as_raw(self), pinfo, guidtx).ok()
    }
    pub unsafe fn OnTransactionCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnTransactionCommit)(windows_core::Interface::as_raw(self), pinfo, guidtx).ok()
    }
}
#[repr(C)]
pub struct IComTransactionEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnTransactionStart: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnTransactionPrepare: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnTransactionAbort: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnTransactionCommit: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComUserEvent, IComUserEvent_Vtbl, 0x683130a4_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for IComUserEvent {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComUserEvent, windows_core::IUnknown);
impl IComUserEvent {
    pub unsafe fn OnUserEvent(&self, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnUserEvent)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(pvarevent)).ok()
    }
}
#[repr(C)]
pub struct IComUserEvent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnUserEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContextProperties, IContextProperties_Vtbl, 0xd396da85_bf8f_11d1_bbae_00c04fc2fa5f);
impl core::ops::Deref for IContextProperties {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IContextProperties, windows_core::IUnknown);
impl IContextProperties {
    pub unsafe fn Count(&self, plcount: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn GetProperty<P0>(&self, name: P0, pproperty: *mut windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), name.param().abi(), core::mem::transmute(pproperty)).ok()
    }
    pub unsafe fn EnumNames(&self) -> windows_core::Result<IEnumNames> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumNames)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProperty<P0, P1>(&self, name: P0, property: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), name.param().abi(), property.param().abi()).ok()
    }
    pub unsafe fn RemoveProperty<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveProperty)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IContextProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub RemoveProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContextSecurityPerimeter, IContextSecurityPerimeter_Vtbl, 0xa7549a29_a7c4_42e1_8dc1_7e3d748dc24a);
impl core::ops::Deref for IContextSecurityPerimeter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IContextSecurityPerimeter, windows_core::IUnknown);
impl IContextSecurityPerimeter {
    pub unsafe fn GetPerimeterFlag(&self, pflag: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPerimeterFlag)(windows_core::Interface::as_raw(self), pflag).ok()
    }
    pub unsafe fn SetPerimeterFlag<P0>(&self, fflag: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPerimeterFlag)(windows_core::Interface::as_raw(self), fflag.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IContextSecurityPerimeter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPerimeterFlag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetPerimeterFlag: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContextState, IContextState_Vtbl, 0x3c05e54b_a42a_11d2_afc4_00c04f8ee1c4);
impl core::ops::Deref for IContextState {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IContextState, windows_core::IUnknown);
impl IContextState {
    pub unsafe fn SetDeactivateOnReturn<P0>(&self, bdeactivate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDeactivateOnReturn)(windows_core::Interface::as_raw(self), bdeactivate.param().abi()).ok()
    }
    pub unsafe fn GetDeactivateOnReturn(&self, pbdeactivate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDeactivateOnReturn)(windows_core::Interface::as_raw(self), pbdeactivate).ok()
    }
    pub unsafe fn SetMyTransactionVote(&self, txvote: TransactionVote) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMyTransactionVote)(windows_core::Interface::as_raw(self), txvote).ok()
    }
    pub unsafe fn GetMyTransactionVote(&self, ptxvote: *mut TransactionVote) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMyTransactionVote)(windows_core::Interface::as_raw(self), ptxvote).ok()
    }
}
#[repr(C)]
pub struct IContextState_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDeactivateOnReturn: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetDeactivateOnReturn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetMyTransactionVote: unsafe extern "system" fn(*mut core::ffi::c_void, TransactionVote) -> windows_core::HRESULT,
    pub GetMyTransactionVote: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TransactionVote) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateWithLocalTransaction, ICreateWithLocalTransaction_Vtbl, 0x227ac7a8_8423_42ce_b7cf_03061ec9aaa3);
impl core::ops::Deref for ICreateWithLocalTransaction {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICreateWithLocalTransaction, windows_core::IUnknown);
impl ICreateWithLocalTransaction {
    pub unsafe fn CreateInstanceWithSysTx<P0>(&self, ptransaction: P0, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CreateInstanceWithSysTx)(windows_core::Interface::as_raw(self), ptransaction.param().abi(), rclsid, riid, pobject).ok()
    }
}
#[repr(C)]
pub struct ICreateWithLocalTransaction_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstanceWithSysTx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateWithTipTransactionEx, ICreateWithTipTransactionEx_Vtbl, 0x455acf59_5345_11d2_99cf_00c04f797bc9);
impl core::ops::Deref for ICreateWithTipTransactionEx {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICreateWithTipTransactionEx, windows_core::IUnknown);
impl ICreateWithTipTransactionEx {
    pub unsafe fn CreateInstance<P0, T>(&self, bstrtipurl: P0, rclsid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), bstrtipurl.param().abi(), rclsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICreateWithTipTransactionEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateWithTransactionEx, ICreateWithTransactionEx_Vtbl, 0x455acf57_5345_11d2_99cf_00c04f797bc9);
impl core::ops::Deref for ICreateWithTransactionEx {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICreateWithTransactionEx, windows_core::IUnknown);
impl ICreateWithTransactionEx {
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn CreateInstance<P0, T>(&self, ptransaction: P0, rclsid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::DistributedTransactionCoordinator::ITransaction>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), ptransaction.param().abi(), rclsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICreateWithTransactionEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    CreateInstance: usize,
}
windows_core::imp::define_interface!(ICrmCompensator, ICrmCompensator_Vtbl, 0xbbc01830_8d3b_11d1_82ec_00a0c91eede9);
impl core::ops::Deref for ICrmCompensator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICrmCompensator, windows_core::IUnknown);
impl ICrmCompensator {
    pub unsafe fn SetLogControl<P0>(&self, plogcontrol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICrmLogControl>,
    {
        (windows_core::Interface::vtable(self).SetLogControl)(windows_core::Interface::as_raw(self), plogcontrol.param().abi()).ok()
    }
    pub unsafe fn BeginPrepare(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginPrepare)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrepareRecord(&self, crmlogrec: CrmLogRecordRead) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrepareRecord)(windows_core::Interface::as_raw(self), core::mem::transmute(crmlogrec), &mut result__).map(|| result__)
    }
    pub unsafe fn EndPrepare(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EndPrepare)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn BeginCommit<P0>(&self, frecovery: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).BeginCommit)(windows_core::Interface::as_raw(self), frecovery.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitRecord(&self, crmlogrec: CrmLogRecordRead) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommitRecord)(windows_core::Interface::as_raw(self), core::mem::transmute(crmlogrec), &mut result__).map(|| result__)
    }
    pub unsafe fn EndCommit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndCommit)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginAbort<P0>(&self, frecovery: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).BeginAbort)(windows_core::Interface::as_raw(self), frecovery.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AbortRecord(&self, crmlogrec: CrmLogRecordRead) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AbortRecord)(windows_core::Interface::as_raw(self), core::mem::transmute(crmlogrec), &mut result__).map(|| result__)
    }
    pub unsafe fn EndAbort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndAbort)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ICrmCompensator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetLogControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginPrepare: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PrepareRecord: unsafe extern "system" fn(*mut core::ffi::c_void, CrmLogRecordRead, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrepareRecord: usize,
    pub EndPrepare: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub BeginCommit: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CommitRecord: unsafe extern "system" fn(*mut core::ffi::c_void, CrmLogRecordRead, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommitRecord: usize,
    pub EndCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginAbort: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AbortRecord: unsafe extern "system" fn(*mut core::ffi::c_void, CrmLogRecordRead, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AbortRecord: usize,
    pub EndAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICrmCompensatorVariants, ICrmCompensatorVariants_Vtbl, 0xf0baf8e4_7804_11d1_82e9_00a0c91eede9);
impl core::ops::Deref for ICrmCompensatorVariants {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICrmCompensatorVariants, windows_core::IUnknown);
impl ICrmCompensatorVariants {
    pub unsafe fn SetLogControlVariants<P0>(&self, plogcontrol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICrmLogControl>,
    {
        (windows_core::Interface::vtable(self).SetLogControlVariants)(windows_core::Interface::as_raw(self), plogcontrol.param().abi()).ok()
    }
    pub unsafe fn BeginPrepareVariants(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginPrepareVariants)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PrepareRecordVariants(&self, plogrecord: *const windows_core::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrepareRecordVariants)(windows_core::Interface::as_raw(self), core::mem::transmute(plogrecord), &mut result__).map(|| result__)
    }
    pub unsafe fn EndPrepareVariants(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EndPrepareVariants)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn BeginCommitVariants<P0>(&self, brecovery: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).BeginCommitVariants)(windows_core::Interface::as_raw(self), brecovery.param().abi()).ok()
    }
    pub unsafe fn CommitRecordVariants(&self, plogrecord: *const windows_core::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommitRecordVariants)(windows_core::Interface::as_raw(self), core::mem::transmute(plogrecord), &mut result__).map(|| result__)
    }
    pub unsafe fn EndCommitVariants(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndCommitVariants)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginAbortVariants<P0>(&self, brecovery: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).BeginAbortVariants)(windows_core::Interface::as_raw(self), brecovery.param().abi()).ok()
    }
    pub unsafe fn AbortRecordVariants(&self, plogrecord: *const windows_core::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AbortRecordVariants)(windows_core::Interface::as_raw(self), core::mem::transmute(plogrecord), &mut result__).map(|| result__)
    }
    pub unsafe fn EndAbortVariants(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndAbortVariants)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ICrmCompensatorVariants_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetLogControlVariants: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginPrepareVariants: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrepareRecordVariants: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EndPrepareVariants: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub BeginCommitVariants: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CommitRecordVariants: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EndCommitVariants: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginAbortVariants: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AbortRecordVariants: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EndAbortVariants: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICrmFormatLogRecords, ICrmFormatLogRecords_Vtbl, 0x9c51d821_c98b_11d1_82fb_00a0c91eede9);
impl core::ops::Deref for ICrmFormatLogRecords {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICrmFormatLogRecords, windows_core::IUnknown);
impl ICrmFormatLogRecords {
    pub unsafe fn GetColumnCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColumnCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetColumnHeaders(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColumnHeaders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetColumn(&self, crmlogrec: CrmLogRecordRead) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColumn)(windows_core::Interface::as_raw(self), core::mem::transmute(crmlogrec), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetColumnVariants<P0>(&self, logrecord: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColumnVariants)(windows_core::Interface::as_raw(self), logrecord.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICrmFormatLogRecords_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetColumnHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetColumn: unsafe extern "system" fn(*mut core::ffi::c_void, CrmLogRecordRead, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetColumn: usize,
    pub GetColumnVariants: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICrmLogControl, ICrmLogControl_Vtbl, 0xa0e174b3_d26e_11d2_8f84_00805fc7bcd9);
impl core::ops::Deref for ICrmLogControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICrmLogControl, windows_core::IUnknown);
impl ICrmLogControl {
    pub unsafe fn TransactionUOW(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TransactionUOW)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterCompensator<P0, P1>(&self, lpcwstrprogidcompensator: P0, lpcwstrdescription: P1, lcrmregflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RegisterCompensator)(windows_core::Interface::as_raw(self), lpcwstrprogidcompensator.param().abi(), lpcwstrdescription.param().abi(), lcrmregflags).ok()
    }
    pub unsafe fn WriteLogRecordVariants(&self, plogrecord: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WriteLogRecordVariants)(windows_core::Interface::as_raw(self), core::mem::transmute(plogrecord)).ok()
    }
    pub unsafe fn ForceLog(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ForceLog)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ForgetLogRecord(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ForgetLogRecord)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ForceTransactionToAbort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ForceTransactionToAbort)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteLogRecord(&self, rgblob: &[super::Com::BLOB]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WriteLogRecord)(windows_core::Interface::as_raw(self), core::mem::transmute(rgblob.as_ptr()), rgblob.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct ICrmLogControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TransactionUOW: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RegisterCompensator: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub WriteLogRecordVariants: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ForceLog: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ForgetLogRecord: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ForceTransactionToAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteLogRecord: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::BLOB, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteLogRecord: usize,
}
windows_core::imp::define_interface!(ICrmMonitor, ICrmMonitor_Vtbl, 0x70c8e443_c7ed_11d1_82fb_00a0c91eede9);
impl core::ops::Deref for ICrmMonitor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICrmMonitor, windows_core::IUnknown);
impl ICrmMonitor {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClerks(&self) -> windows_core::Result<ICrmMonitorClerks> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClerks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn HoldClerk<P0>(&self, index: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HoldClerk)(windows_core::Interface::as_raw(self), index.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICrmMonitor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClerks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClerks: usize,
    pub HoldClerk: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICrmMonitorClerks, ICrmMonitorClerks_Vtbl, 0x70c8e442_c7ed_11d1_82fb_00a0c91eede9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICrmMonitorClerks {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICrmMonitorClerks, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICrmMonitorClerks {
    pub unsafe fn Item<P0>(&self, index: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ProgIdCompensator<P0>(&self, index: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProgIdCompensator)(windows_core::Interface::as_raw(self), index.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Description<P0>(&self, index: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), index.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TransactionUOW<P0>(&self, index: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TransactionUOW)(windows_core::Interface::as_raw(self), index.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ActivityId<P0>(&self, index: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActivityId)(windows_core::Interface::as_raw(self), index.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICrmMonitorClerks_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ProgIdCompensator: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub TransactionUOW: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICrmMonitorLogRecords, ICrmMonitorLogRecords_Vtbl, 0x70c8e441_c7ed_11d1_82fb_00a0c91eede9);
impl core::ops::Deref for ICrmMonitorLogRecords {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICrmMonitorLogRecords, windows_core::IUnknown);
impl ICrmMonitorLogRecords {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TransactionState(&self) -> windows_core::Result<CrmTransactionState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TransactionState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StructuredRecords(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StructuredRecords)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetLogRecord(&self, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLogRecord)(windows_core::Interface::as_raw(self), dwindex, pcrmlogrec).ok()
    }
    pub unsafe fn GetLogRecordVariants<P0>(&self, indexnumber: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLogRecordVariants)(windows_core::Interface::as_raw(self), indexnumber.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICrmMonitorLogRecords_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TransactionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CrmTransactionState) -> windows_core::HRESULT,
    pub StructuredRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetLogRecord: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CrmLogRecordRead) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetLogRecord: usize,
    pub GetLogRecordVariants: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDispenserDriver, IDispenserDriver_Vtbl, 0x208b3651_2b48_11cf_be10_00aa00a2fa25);
impl core::ops::Deref for IDispenserDriver {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDispenserDriver, windows_core::IUnknown);
impl IDispenserDriver {
    pub unsafe fn CreateResource(&self, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateResource)(windows_core::Interface::as_raw(self), restypid, presid, psecsfreebeforedestroy).ok()
    }
    pub unsafe fn RateResource<P0>(&self, restypid: usize, resid: usize, frequirestransactionenlistment: P0, prating: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).RateResource)(windows_core::Interface::as_raw(self), restypid, resid, frequirestransactionenlistment.param().abi(), prating).ok()
    }
    pub unsafe fn EnlistResource(&self, resid: usize, transid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnlistResource)(windows_core::Interface::as_raw(self), resid, transid).ok()
    }
    pub unsafe fn ResetResource(&self, resid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetResource)(windows_core::Interface::as_raw(self), resid).ok()
    }
    pub unsafe fn DestroyResource(&self, resid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DestroyResource)(windows_core::Interface::as_raw(self), resid).ok()
    }
    pub unsafe fn DestroyResourceS(&self, resid: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DestroyResourceS)(windows_core::Interface::as_raw(self), resid).ok()
    }
}
#[repr(C)]
pub struct IDispenserDriver_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize, *mut i32) -> windows_core::HRESULT,
    pub RateResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, super::super::Foundation::BOOL, *mut u32) -> windows_core::HRESULT,
    pub EnlistResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
    pub ResetResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub DestroyResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub DestroyResourceS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDispenserManager, IDispenserManager_Vtbl, 0x5cb31e10_2b5f_11cf_be10_00aa00a2fa25);
impl core::ops::Deref for IDispenserManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDispenserManager, windows_core::IUnknown);
impl IDispenserManager {
    pub unsafe fn RegisterDispenser<P0, P1>(&self, __midl__idispensermanager0000: P0, szdispensername: P1) -> windows_core::Result<IHolder>
    where
        P0: windows_core::Param<IDispenserDriver>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterDispenser)(windows_core::Interface::as_raw(self), __midl__idispensermanager0000.param().abi(), szdispensername.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetContext(&self, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), __midl__idispensermanager0002, __midl__idispensermanager0003).ok()
    }
}
#[repr(C)]
pub struct IDispenserManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterDispenser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize, *mut usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumNames, IEnumNames_Vtbl, 0x51372af2_cae7_11cf_be81_00aa00a2fa25);
impl core::ops::Deref for IEnumNames {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumNames, windows_core::IUnknown);
impl IEnumNames {
    pub unsafe fn Next(&self, celt: u32, rgname: *mut windows_core::BSTR, pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgname), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumNames> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumNames_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IEventServerTrace, IEventServerTrace_Vtbl, 0x9a9f12b8_80af_47ab_a579_35ea57725370);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IEventServerTrace {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IEventServerTrace, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IEventServerTrace {
    pub unsafe fn StartTraceGuid<P0, P1>(&self, bstrguidevent: P0, bstrguidfilter: P1, lpidfilter: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).StartTraceGuid)(windows_core::Interface::as_raw(self), bstrguidevent.param().abi(), bstrguidfilter.param().abi(), lpidfilter).ok()
    }
    pub unsafe fn StopTraceGuid<P0, P1>(&self, bstrguidevent: P0, bstrguidfilter: P1, lpidfilter: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).StopTraceGuid)(windows_core::Interface::as_raw(self), bstrguidevent.param().abi(), bstrguidfilter.param().abi(), lpidfilter).ok()
    }
    pub unsafe fn EnumTraceGuid(&self, plcntguids: *mut i32, pbstrguidlist: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumTraceGuid)(windows_core::Interface::as_raw(self), plcntguids, core::mem::transmute(pbstrguidlist)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEventServerTrace_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub StartTraceGuid: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    pub StopTraceGuid: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    pub EnumTraceGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGetAppTrackerData, IGetAppTrackerData_Vtbl, 0x507c3ac8_3e12_4cb0_9366_653d3e050638);
impl core::ops::Deref for IGetAppTrackerData {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetAppTrackerData, windows_core::IUnknown);
impl IGetAppTrackerData {
    pub unsafe fn GetApplicationProcesses(&self, partitionid: *const windows_core::GUID, applicationid: *const windows_core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetApplicationProcesses)(windows_core::Interface::as_raw(self), partitionid, applicationid, flags, numapplicationprocesses, applicationprocesses).ok()
    }
    pub unsafe fn GetApplicationProcessDetails(&self, applicationinstanceid: *const windows_core::GUID, processid: u32, flags: u32, summary: Option<*mut ApplicationProcessSummary>, statistics: Option<*mut ApplicationProcessStatistics>, recycleinfo: Option<*mut ApplicationProcessRecycleInfo>, anycomponentshangmonitored: Option<*mut super::super::Foundation::BOOL>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetApplicationProcessDetails)(windows_core::Interface::as_raw(self), applicationinstanceid, processid, flags, core::mem::transmute(summary.unwrap_or(std::ptr::null_mut())), core::mem::transmute(statistics.unwrap_or(std::ptr::null_mut())), core::mem::transmute(recycleinfo.unwrap_or(std::ptr::null_mut())), core::mem::transmute(anycomponentshangmonitored.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetApplicationsInProcess(&self, applicationinstanceid: *const windows_core::GUID, processid: u32, partitionid: *const windows_core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetApplicationsInProcess)(windows_core::Interface::as_raw(self), applicationinstanceid, processid, partitionid, flags, numapplicationsinprocess, applications).ok()
    }
    pub unsafe fn GetComponentsInProcess(&self, applicationinstanceid: *const windows_core::GUID, processid: u32, partitionid: *const windows_core::GUID, applicationid: *const windows_core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetComponentsInProcess)(windows_core::Interface::as_raw(self), applicationinstanceid, processid, partitionid, applicationid, flags, numcomponentsinprocess, components).ok()
    }
    pub unsafe fn GetComponentDetails(&self, applicationinstanceid: *const windows_core::GUID, processid: u32, clsid: *const windows_core::GUID, flags: u32, summary: Option<*mut ComponentSummary>, statistics: Option<*mut ComponentStatistics>, hangmonitorinfo: Option<*mut ComponentHangMonitorInfo>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetComponentDetails)(windows_core::Interface::as_raw(self), applicationinstanceid, processid, clsid, flags, core::mem::transmute(summary.unwrap_or(std::ptr::null_mut())), core::mem::transmute(statistics.unwrap_or(std::ptr::null_mut())), core::mem::transmute(hangmonitorinfo.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetTrackerDataAsCollectionObject(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTrackerDataAsCollectionObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSuggestedPollingInterval(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSuggestedPollingInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IGetAppTrackerData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetApplicationProcesses: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, *mut u32, *mut *mut ApplicationProcessSummary) -> windows_core::HRESULT,
    pub GetApplicationProcessDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32, *mut ApplicationProcessSummary, *mut ApplicationProcessStatistics, *mut ApplicationProcessRecycleInfo, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetApplicationsInProcess: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID, u32, *mut u32, *mut *mut ApplicationSummary) -> windows_core::HRESULT,
    pub GetComponentsInProcess: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID, *const windows_core::GUID, u32, *mut u32, *mut *mut ComponentSummary) -> windows_core::HRESULT,
    pub GetComponentDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID, u32, *mut ComponentSummary, *mut ComponentStatistics, *mut ComponentHangMonitorInfo) -> windows_core::HRESULT,
    pub GetTrackerDataAsCollectionObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSuggestedPollingInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGetContextProperties, IGetContextProperties_Vtbl, 0x51372af4_cae7_11cf_be81_00aa00a2fa25);
impl core::ops::Deref for IGetContextProperties {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetContextProperties, windows_core::IUnknown);
impl IGetContextProperties {
    pub unsafe fn Count(&self, plcount: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn GetProperty<P0>(&self, name: P0, pproperty: *mut windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), name.param().abi(), core::mem::transmute(pproperty)).ok()
    }
    pub unsafe fn EnumNames(&self) -> windows_core::Result<IEnumNames> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumNames)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IGetContextProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGetSecurityCallContext, IGetSecurityCallContext_Vtbl, 0xcafc823f_b441_11d1_b82b_0000f8757e2a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGetSecurityCallContext {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGetSecurityCallContext, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGetSecurityCallContext {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityCallContext(&self) -> windows_core::Result<ISecurityCallContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSecurityCallContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGetSecurityCallContext_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityCallContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityCallContext: usize,
}
windows_core::imp::define_interface!(IHolder, IHolder_Vtbl, 0xbf6a1850_2b45_11cf_be10_00aa00a2fa25);
impl core::ops::Deref for IHolder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHolder, windows_core::IUnknown);
impl IHolder {
    pub unsafe fn AllocResource(&self, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AllocResource)(windows_core::Interface::as_raw(self), __midl__iholder0000, __midl__iholder0001).ok()
    }
    pub unsafe fn FreeResource(&self, __midl__iholder0002: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FreeResource)(windows_core::Interface::as_raw(self), __midl__iholder0002).ok()
    }
    pub unsafe fn TrackResource(&self, __midl__iholder0003: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TrackResource)(windows_core::Interface::as_raw(self), __midl__iholder0003).ok()
    }
    pub unsafe fn TrackResourceS(&self, __midl__iholder0004: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TrackResourceS)(windows_core::Interface::as_raw(self), __midl__iholder0004).ok()
    }
    pub unsafe fn UntrackResource<P0>(&self, __midl__iholder0005: usize, __midl__iholder0006: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).UntrackResource)(windows_core::Interface::as_raw(self), __midl__iholder0005, __midl__iholder0006.param().abi()).ok()
    }
    pub unsafe fn UntrackResourceS<P0>(&self, __midl__iholder0007: *mut u16, __midl__iholder0008: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).UntrackResourceS)(windows_core::Interface::as_raw(self), __midl__iholder0007, __midl__iholder0008.param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestDestroyResource(&self, __midl__iholder0009: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestDestroyResource)(windows_core::Interface::as_raw(self), __midl__iholder0009).ok()
    }
}
#[repr(C)]
pub struct IHolder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AllocResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize) -> windows_core::HRESULT,
    pub FreeResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub TrackResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub TrackResourceS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub UntrackResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub UntrackResourceS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestDestroyResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILBEvents, ILBEvents_Vtbl, 0x683130b4_2e50_11d2_98a5_00c04f8ee1c4);
impl core::ops::Deref for ILBEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILBEvents, windows_core::IUnknown);
impl ILBEvents {
    pub unsafe fn TargetUp<P0, P1>(&self, bstrservername: P0, bstrclsideng: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).TargetUp)(windows_core::Interface::as_raw(self), bstrservername.param().abi(), bstrclsideng.param().abi()).ok()
    }
    pub unsafe fn TargetDown<P0, P1>(&self, bstrservername: P0, bstrclsideng: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).TargetDown)(windows_core::Interface::as_raw(self), bstrservername.param().abi(), bstrclsideng.param().abi()).ok()
    }
    pub unsafe fn EngineDefined<P0, P1>(&self, bstrpropname: P0, varpropvalue: *const windows_core::VARIANT, bstrclsideng: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).EngineDefined)(windows_core::Interface::as_raw(self), bstrpropname.param().abi(), core::mem::transmute(varpropvalue), bstrclsideng.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ILBEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TargetUp: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TargetDown: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EngineDefined: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMTSActivity, IMTSActivity_Vtbl, 0x51372af0_cae7_11cf_be81_00aa00a2fa25);
impl core::ops::Deref for IMTSActivity {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMTSActivity, windows_core::IUnknown);
impl IMTSActivity {
    pub unsafe fn SynchronousCall<P0>(&self, pcall: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMTSCall>,
    {
        (windows_core::Interface::vtable(self).SynchronousCall)(windows_core::Interface::as_raw(self), pcall.param().abi()).ok()
    }
    pub unsafe fn AsyncCall<P0>(&self, pcall: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMTSCall>,
    {
        (windows_core::Interface::vtable(self).AsyncCall)(windows_core::Interface::as_raw(self), pcall.param().abi()).ok()
    }
    pub unsafe fn Reserved1(&self) {
        (windows_core::Interface::vtable(self).Reserved1)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn BindToCurrentThread(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BindToCurrentThread)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn UnbindFromThread(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnbindFromThread)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IMTSActivity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SynchronousCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AsyncCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reserved1: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub BindToCurrentThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnbindFromThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMTSCall, IMTSCall_Vtbl, 0x51372aef_cae7_11cf_be81_00aa00a2fa25);
impl core::ops::Deref for IMTSCall {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMTSCall, windows_core::IUnknown);
impl IMTSCall {
    pub unsafe fn OnCall(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCall)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IMTSCall_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCall: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMTSLocator, IMTSLocator_Vtbl, 0xd19b8bfd_7f88_11d0_b16e_00aa00ba3258);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMTSLocator {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMTSLocator, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMTSLocator {
    pub unsafe fn GetEventDispatcher(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEventDispatcher)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMTSLocator_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetEventDispatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManagedActivationEvents, IManagedActivationEvents_Vtbl, 0xa5f325af_572f_46da_b8ab_827c3d95d99e);
impl core::ops::Deref for IManagedActivationEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IManagedActivationEvents, windows_core::IUnknown);
impl IManagedActivationEvents {
    pub unsafe fn CreateManagedStub<P0, P1>(&self, pinfo: P0, fdist: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IManagedObjectInfo>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).CreateManagedStub)(windows_core::Interface::as_raw(self), pinfo.param().abi(), fdist.param().abi()).ok()
    }
    pub unsafe fn DestroyManagedStub<P0>(&self, pinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IManagedObjectInfo>,
    {
        (windows_core::Interface::vtable(self).DestroyManagedStub)(windows_core::Interface::as_raw(self), pinfo.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IManagedActivationEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateManagedStub: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub DestroyManagedStub: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManagedObjectInfo, IManagedObjectInfo_Vtbl, 0x1427c51a_4584_49d8_90a0_c50d8086cbe9);
impl core::ops::Deref for IManagedObjectInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IManagedObjectInfo, windows_core::IUnknown);
impl IManagedObjectInfo {
    pub unsafe fn GetIUnknown(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIUnknown)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetIObjectControl(&self) -> windows_core::Result<IObjectControl> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIObjectControl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetInPool<P0, P1>(&self, binpool: P0, ppooledobj: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<IManagedPooledObj>,
    {
        (windows_core::Interface::vtable(self).SetInPool)(windows_core::Interface::as_raw(self), binpool.param().abi(), ppooledobj.param().abi()).ok()
    }
    pub unsafe fn SetWrapperStrength<P0>(&self, bstrong: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetWrapperStrength)(windows_core::Interface::as_raw(self), bstrong.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IManagedObjectInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIUnknown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIObjectControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInPool: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWrapperStrength: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManagedPoolAction, IManagedPoolAction_Vtbl, 0xda91b74e_5388_4783_949d_c1cd5fb00506);
impl core::ops::Deref for IManagedPoolAction {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IManagedPoolAction, windows_core::IUnknown);
impl IManagedPoolAction {
    pub unsafe fn LastRelease(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LastRelease)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IManagedPoolAction_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LastRelease: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManagedPooledObj, IManagedPooledObj_Vtbl, 0xc5da4bea_1b42_4437_8926_b6a38860a770);
impl core::ops::Deref for IManagedPooledObj {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IManagedPooledObj, windows_core::IUnknown);
impl IManagedPooledObj {
    pub unsafe fn SetHeld<P0>(&self, m_bheld: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetHeld)(windows_core::Interface::as_raw(self), m_bheld.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IManagedPooledObj_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetHeld: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMessageMover, IMessageMover_Vtbl, 0x588a085a_b795_11d1_8054_00c04fc340ee);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMessageMover {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMessageMover, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMessageMover {
    pub unsafe fn SourcePath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SourcePath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSourcePath<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSourcePath)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn DestPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DestPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDestPath<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDestPath)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn CommitBatchSize(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommitBatchSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCommitBatchSize(&self, newval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCommitBatchSize)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn MoveMessages(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveMessages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMessageMover_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SourcePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSourcePath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DestPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDestPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CommitBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCommitBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MoveMessages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMtsEventInfo, IMtsEventInfo_Vtbl, 0xd56c3dc1_8482_11d0_b170_00aa00ba3258);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMtsEventInfo {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMtsEventInfo, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMtsEventInfo {
    pub unsafe fn Names(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Names)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EventID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn get_Value<P0>(&self, skey: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), skey.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMtsEventInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Names: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EventID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMtsEvents, IMtsEvents_Vtbl, 0xbacedf4d_74ab_11d0_b162_00aa00ba3258);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMtsEvents {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMtsEvents, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMtsEvents {
    pub unsafe fn PackageName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PackageName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PackageGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PackageGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PostEvent(&self, vevent: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PostEvent)(windows_core::Interface::as_raw(self), core::mem::transmute(vevent)).ok()
    }
    pub unsafe fn FireEvents(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FireEvents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProcessID(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProcessID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMtsEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub PackageName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PackageGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PostEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub FireEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetProcessID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMtsGrp, IMtsGrp_Vtbl, 0x4b2e958c_0393_11d1_b1ab_00aa00ba3258);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMtsGrp {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMtsGrp, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMtsGrp {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Item(&self, lindex: i32) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMtsGrp_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IObjPool, IObjPool_Vtbl, 0x7d8805a0_2ea7_11d1_b1cc_00aa00ba3258);
impl core::ops::Deref for IObjPool {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IObjPool, windows_core::IUnknown);
impl IObjPool {
    pub unsafe fn Reserved1(&self) {
        (windows_core::Interface::vtable(self).Reserved1)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved2(&self) {
        (windows_core::Interface::vtable(self).Reserved2)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved3(&self) {
        (windows_core::Interface::vtable(self).Reserved3)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved4(&self) {
        (windows_core::Interface::vtable(self).Reserved4)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PutEndTx<P0>(&self, pobj: P0)
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).PutEndTx)(windows_core::Interface::as_raw(self), pobj.param().abi())
    }
    pub unsafe fn Reserved5(&self) {
        (windows_core::Interface::vtable(self).Reserved5)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved6(&self) {
        (windows_core::Interface::vtable(self).Reserved6)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IObjPool_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Reserved1: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved2: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved3: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved4: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub PutEndTx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub Reserved5: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved6: unsafe extern "system" fn(*mut core::ffi::c_void),
}
windows_core::imp::define_interface!(IObjectConstruct, IObjectConstruct_Vtbl, 0x41c4f8b3_7439_11d2_98cb_00c04f8ee1c4);
impl core::ops::Deref for IObjectConstruct {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IObjectConstruct, windows_core::IUnknown);
impl IObjectConstruct {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Construct<P0>(&self, pctorobj: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).Construct)(windows_core::Interface::as_raw(self), pctorobj.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IObjectConstruct_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Construct: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Construct: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IObjectConstructString, IObjectConstructString_Vtbl, 0x41c4f8b2_7439_11d2_98cb_00c04f8ee1c4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IObjectConstructString {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IObjectConstructString, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IObjectConstructString {
    pub unsafe fn ConstructString(&self, pval: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConstructString)(windows_core::Interface::as_raw(self), core::mem::transmute(pval)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IObjectConstructString_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ConstructString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IObjectContext, IObjectContext_Vtbl, 0x51372ae0_cae7_11cf_be81_00aa00a2fa25);
impl core::ops::Deref for IObjectContext {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IObjectContext, windows_core::IUnknown);
impl IObjectContext {
    pub unsafe fn CreateInstance(&self, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), rclsid, riid, ppv).ok()
    }
    pub unsafe fn SetComplete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetComplete)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetAbort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAbort)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableCommit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableCommit)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableCommit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisableCommit)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsInTransaction(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsInTransaction)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsSecurityEnabled(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsSecurityEnabled)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsCallerInRole<P0>(&self, bstrrole: P0, pfisinrole: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).IsCallerInRole)(windows_core::Interface::as_raw(self), bstrrole.param().abi(), pfisinrole).ok()
    }
}
#[repr(C)]
pub struct IObjectContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub IsSecurityEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub IsCallerInRole: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IObjectContextActivity, IObjectContextActivity_Vtbl, 0x51372afc_cae7_11cf_be81_00aa00a2fa25);
impl core::ops::Deref for IObjectContextActivity {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IObjectContextActivity, windows_core::IUnknown);
impl IObjectContextActivity {
    pub unsafe fn GetActivityId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetActivityId)(windows_core::Interface::as_raw(self), pguid).ok()
    }
}
#[repr(C)]
pub struct IObjectContextActivity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IObjectContextInfo, IObjectContextInfo_Vtbl, 0x75b52ddb_e8ed_11d1_93ad_00aa00ba3258);
impl core::ops::Deref for IObjectContextInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IObjectContextInfo, windows_core::IUnknown);
impl IObjectContextInfo {
    pub unsafe fn IsInTransaction(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsInTransaction)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetTransaction(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTransactionId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTransactionId)(windows_core::Interface::as_raw(self), pguid).ok()
    }
    pub unsafe fn GetActivityId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetActivityId)(windows_core::Interface::as_raw(self), pguid).ok()
    }
    pub unsafe fn GetContextId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetContextId)(windows_core::Interface::as_raw(self), pguid).ok()
    }
}
#[repr(C)]
pub struct IObjectContextInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub GetTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTransactionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetContextId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IObjectContextInfo2, IObjectContextInfo2_Vtbl, 0x594be71a_4bc4_438b_9197_cfd176248b09);
impl core::ops::Deref for IObjectContextInfo2 {
    type Target = IObjectContextInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IObjectContextInfo2, windows_core::IUnknown, IObjectContextInfo);
impl IObjectContextInfo2 {
    pub unsafe fn GetPartitionId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPartitionId)(windows_core::Interface::as_raw(self), pguid).ok()
    }
    pub unsafe fn GetApplicationId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetApplicationId)(windows_core::Interface::as_raw(self), pguid).ok()
    }
    pub unsafe fn GetApplicationInstanceId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetApplicationInstanceId)(windows_core::Interface::as_raw(self), pguid).ok()
    }
}
#[repr(C)]
pub struct IObjectContextInfo2_Vtbl {
    pub base__: IObjectContextInfo_Vtbl,
    pub GetPartitionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetApplicationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetApplicationInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IObjectContextTip, IObjectContextTip_Vtbl, 0x92fd41ca_bad9_11d2_9a2d_00c04f797bc9);
impl core::ops::Deref for IObjectContextTip {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IObjectContextTip, windows_core::IUnknown);
impl IObjectContextTip {
    pub unsafe fn GetTipUrl(&self, ptipurl: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTipUrl)(windows_core::Interface::as_raw(self), core::mem::transmute(ptipurl)).ok()
    }
}
#[repr(C)]
pub struct IObjectContextTip_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTipUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IObjectControl, IObjectControl_Vtbl, 0x51372aec_cae7_11cf_be81_00aa00a2fa25);
impl core::ops::Deref for IObjectControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IObjectControl, windows_core::IUnknown);
impl IObjectControl {
    pub unsafe fn Activate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Deactivate(&self) {
        (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn CanBePooled(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).CanBePooled)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IObjectControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub CanBePooled: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(IPlaybackControl, IPlaybackControl_Vtbl, 0x51372afd_cae7_11cf_be81_00aa00a2fa25);
impl core::ops::Deref for IPlaybackControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPlaybackControl, windows_core::IUnknown);
impl IPlaybackControl {
    pub unsafe fn FinalClientRetry(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FinalClientRetry)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FinalServerRetry(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FinalServerRetry)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IPlaybackControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FinalClientRetry: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FinalServerRetry: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPoolManager, IPoolManager_Vtbl, 0x0a469861_5a91_43a0_99b6_d5e179bb0631);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPoolManager {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPoolManager, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPoolManager {
    pub unsafe fn ShutdownPool<P0>(&self, clsidorprogid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ShutdownPool)(windows_core::Interface::as_raw(self), clsidorprogid.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPoolManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ShutdownPool: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProcessInitializer, IProcessInitializer_Vtbl, 0x1113f52d_dc7f_4943_aed6_88d04027e32a);
impl core::ops::Deref for IProcessInitializer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProcessInitializer, windows_core::IUnknown);
impl IProcessInitializer {
    pub unsafe fn Startup<P0>(&self, punkprocesscontrol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Startup)(windows_core::Interface::as_raw(self), punkprocesscontrol.param().abi()).ok()
    }
    pub unsafe fn Shutdown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IProcessInitializer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Startup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISecurityCallContext, ISecurityCallContext_Vtbl, 0xcafc823e_b441_11d1_b82b_0000f8757e2a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISecurityCallContext {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISecurityCallContext, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISecurityCallContext {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn get_Item<P0>(&self, name: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsCallerInRole<P0>(&self, bstrrole: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsCallerInRole)(windows_core::Interface::as_raw(self), bstrrole.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn IsSecurityEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSecurityEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsUserInRole<P0>(&self, puser: *const windows_core::VARIANT, bstrrole: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsUserInRole)(windows_core::Interface::as_raw(self), core::mem::transmute(puser), bstrrole.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISecurityCallContext_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsCallerInRole: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsSecurityEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsUserInRole: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISecurityCallersColl, ISecurityCallersColl_Vtbl, 0xcafc823d_b441_11d1_b82b_0000f8757e2a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISecurityCallersColl {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISecurityCallersColl, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISecurityCallersColl {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<ISecurityIdentityColl> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISecurityCallersColl_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISecurityIdentityColl, ISecurityIdentityColl_Vtbl, 0xcafc823c_b441_11d1_b82b_0000f8757e2a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISecurityIdentityColl {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISecurityIdentityColl, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISecurityIdentityColl {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn get_Item<P0>(&self, name: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISecurityIdentityColl_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISecurityProperty, ISecurityProperty_Vtbl, 0x51372aea_cae7_11cf_be81_00aa00a2fa25);
impl core::ops::Deref for ISecurityProperty {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISecurityProperty, windows_core::IUnknown);
impl ISecurityProperty {
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn GetDirectCreatorSID(&self, psid: *mut super::super::Security::PSID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDirectCreatorSID)(windows_core::Interface::as_raw(self), psid).ok()
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn GetOriginalCreatorSID(&self, psid: *mut super::super::Security::PSID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOriginalCreatorSID)(windows_core::Interface::as_raw(self), psid).ok()
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn GetDirectCallerSID(&self, psid: *mut super::super::Security::PSID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDirectCallerSID)(windows_core::Interface::as_raw(self), psid).ok()
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn GetOriginalCallerSID(&self, psid: *mut super::super::Security::PSID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOriginalCallerSID)(windows_core::Interface::as_raw(self), psid).ok()
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn ReleaseSID<P0>(&self, psid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::PSID>,
    {
        (windows_core::Interface::vtable(self).ReleaseSID)(windows_core::Interface::as_raw(self), psid.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISecurityProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Security")]
    pub GetDirectCreatorSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Security::PSID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    GetDirectCreatorSID: usize,
    #[cfg(feature = "Win32_Security")]
    pub GetOriginalCreatorSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Security::PSID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    GetOriginalCreatorSID: usize,
    #[cfg(feature = "Win32_Security")]
    pub GetDirectCallerSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Security::PSID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    GetDirectCallerSID: usize,
    #[cfg(feature = "Win32_Security")]
    pub GetOriginalCallerSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Security::PSID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    GetOriginalCallerSID: usize,
    #[cfg(feature = "Win32_Security")]
    pub ReleaseSID: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Security::PSID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    ReleaseSID: usize,
}
windows_core::imp::define_interface!(ISelectCOMLBServer, ISelectCOMLBServer_Vtbl, 0xdcf443f4_3f8a_4872_b9f0_369a796d12d6);
impl core::ops::Deref for ISelectCOMLBServer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISelectCOMLBServer, windows_core::IUnknown);
impl ISelectCOMLBServer {
    pub unsafe fn Init(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetLBServer<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).GetLBServer)(windows_core::Interface::as_raw(self), punk.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISelectCOMLBServer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLBServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISendMethodEvents, ISendMethodEvents_Vtbl, 0x2732fd59_b2b4_4d44_878c_8b8f09626008);
impl core::ops::Deref for ISendMethodEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISendMethodEvents, windows_core::IUnknown);
impl ISendMethodEvents {
    pub unsafe fn SendMethodCall(&self, pidentity: *const core::ffi::c_void, riid: *const windows_core::GUID, dwmeth: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendMethodCall)(windows_core::Interface::as_raw(self), pidentity, riid, dwmeth).ok()
    }
    pub unsafe fn SendMethodReturn(&self, pidentity: *const core::ffi::c_void, riid: *const windows_core::GUID, dwmeth: u32, hrcall: windows_core::HRESULT, hrserver: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendMethodReturn)(windows_core::Interface::as_raw(self), pidentity, riid, dwmeth, hrcall, hrserver).ok()
    }
}
#[repr(C)]
pub struct ISendMethodEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SendMethodCall: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub SendMethodReturn: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const windows_core::GUID, u32, windows_core::HRESULT, windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServiceActivity, IServiceActivity_Vtbl, 0x67532e0c_9e2f_4450_a354_035633944e17);
impl core::ops::Deref for IServiceActivity {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServiceActivity, windows_core::IUnknown);
impl IServiceActivity {
    pub unsafe fn SynchronousCall<P0>(&self, piservicecall: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IServiceCall>,
    {
        (windows_core::Interface::vtable(self).SynchronousCall)(windows_core::Interface::as_raw(self), piservicecall.param().abi()).ok()
    }
    pub unsafe fn AsynchronousCall<P0>(&self, piservicecall: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IServiceCall>,
    {
        (windows_core::Interface::vtable(self).AsynchronousCall)(windows_core::Interface::as_raw(self), piservicecall.param().abi()).ok()
    }
    pub unsafe fn BindToCurrentThread(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BindToCurrentThread)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn UnbindFromThread(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnbindFromThread)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IServiceActivity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SynchronousCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AsynchronousCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BindToCurrentThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnbindFromThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServiceCall, IServiceCall_Vtbl, 0xbd3e2e12_42dd_40f4_a09a_95a50c58304b);
impl core::ops::Deref for IServiceCall {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServiceCall, windows_core::IUnknown);
impl IServiceCall {
    pub unsafe fn OnCall(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCall)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IServiceCall_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCall: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServiceComTIIntrinsicsConfig, IServiceComTIIntrinsicsConfig_Vtbl, 0x09e6831e_04e1_4ed4_9d0f_e8b168bafeaf);
impl core::ops::Deref for IServiceComTIIntrinsicsConfig {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServiceComTIIntrinsicsConfig, windows_core::IUnknown);
impl IServiceComTIIntrinsicsConfig {
    pub unsafe fn ComTIIntrinsicsConfig(&self, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ComTIIntrinsicsConfig)(windows_core::Interface::as_raw(self), comtiintrinsicsconfig).ok()
    }
}
#[repr(C)]
pub struct IServiceComTIIntrinsicsConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ComTIIntrinsicsConfig: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_COMTIIntrinsicsConfig) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServiceIISIntrinsicsConfig, IServiceIISIntrinsicsConfig_Vtbl, 0x1a0cf920_d452_46f4_bc36_48118d54ea52);
impl core::ops::Deref for IServiceIISIntrinsicsConfig {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServiceIISIntrinsicsConfig, windows_core::IUnknown);
impl IServiceIISIntrinsicsConfig {
    pub unsafe fn IISIntrinsicsConfig(&self, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IISIntrinsicsConfig)(windows_core::Interface::as_raw(self), iisintrinsicsconfig).ok()
    }
}
#[repr(C)]
pub struct IServiceIISIntrinsicsConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IISIntrinsicsConfig: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_IISIntrinsicsConfig) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServiceInheritanceConfig, IServiceInheritanceConfig_Vtbl, 0x92186771_d3b4_4d77_a8ea_ee842d586f35);
impl core::ops::Deref for IServiceInheritanceConfig {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServiceInheritanceConfig, windows_core::IUnknown);
impl IServiceInheritanceConfig {
    pub unsafe fn ContainingContextTreatment(&self, inheritanceconfig: CSC_InheritanceConfig) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ContainingContextTreatment)(windows_core::Interface::as_raw(self), inheritanceconfig).ok()
    }
}
#[repr(C)]
pub struct IServiceInheritanceConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ContainingContextTreatment: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_InheritanceConfig) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServicePartitionConfig, IServicePartitionConfig_Vtbl, 0x80182d03_5ea4_4831_ae97_55beffc2e590);
impl core::ops::Deref for IServicePartitionConfig {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServicePartitionConfig, windows_core::IUnknown);
impl IServicePartitionConfig {
    pub unsafe fn PartitionConfig(&self, partitionconfig: CSC_PartitionConfig) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PartitionConfig)(windows_core::Interface::as_raw(self), partitionconfig).ok()
    }
    pub unsafe fn PartitionID(&self, guidpartitionid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PartitionID)(windows_core::Interface::as_raw(self), guidpartitionid).ok()
    }
}
#[repr(C)]
pub struct IServicePartitionConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PartitionConfig: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_PartitionConfig) -> windows_core::HRESULT,
    pub PartitionID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServicePool, IServicePool_Vtbl, 0xb302df81_ea45_451e_99a2_09f9fd1b1e13);
impl core::ops::Deref for IServicePool {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServicePool, windows_core::IUnknown);
impl IServicePool {
    pub unsafe fn Initialize<P0>(&self, ppoolconfig: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), ppoolconfig.param().abi()).ok()
    }
    pub unsafe fn GetObject(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), riid, ppv).ok()
    }
    pub unsafe fn Shutdown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IServicePool_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServicePoolConfig, IServicePoolConfig_Vtbl, 0xa9690656_5bca_470c_8451_250c1f43a33e);
impl core::ops::Deref for IServicePoolConfig {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServicePoolConfig, windows_core::IUnknown);
impl IServicePoolConfig {
    pub unsafe fn SetMaxPoolSize(&self, dwmaxpool: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxPoolSize)(windows_core::Interface::as_raw(self), dwmaxpool).ok()
    }
    pub unsafe fn MaxPoolSize(&self, pdwmaxpool: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MaxPoolSize)(windows_core::Interface::as_raw(self), pdwmaxpool).ok()
    }
    pub unsafe fn SetMinPoolSize(&self, dwminpool: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinPoolSize)(windows_core::Interface::as_raw(self), dwminpool).ok()
    }
    pub unsafe fn MinPoolSize(&self, pdwminpool: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MinPoolSize)(windows_core::Interface::as_raw(self), pdwminpool).ok()
    }
    pub unsafe fn SetCreationTimeout(&self, dwcreationtimeout: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCreationTimeout)(windows_core::Interface::as_raw(self), dwcreationtimeout).ok()
    }
    pub unsafe fn CreationTimeout(&self, pdwcreationtimeout: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreationTimeout)(windows_core::Interface::as_raw(self), pdwcreationtimeout).ok()
    }
    pub unsafe fn SetTransactionAffinity<P0>(&self, ftxaffinity: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetTransactionAffinity)(windows_core::Interface::as_raw(self), ftxaffinity.param().abi()).ok()
    }
    pub unsafe fn TransactionAffinity(&self, pftxaffinity: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TransactionAffinity)(windows_core::Interface::as_raw(self), pftxaffinity).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetClassFactory<P0>(&self, pfactory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IClassFactory>,
    {
        (windows_core::Interface::vtable(self).SetClassFactory)(windows_core::Interface::as_raw(self), pfactory.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassFactory(&self) -> windows_core::Result<super::Com::IClassFactory> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClassFactory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IServicePoolConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMaxPoolSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MaxPoolSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMinPoolSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MinPoolSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCreationTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CreationTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTransactionAffinity: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub TransactionAffinity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetClassFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetClassFactory: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ClassFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ClassFactory: usize,
}
windows_core::imp::define_interface!(IServiceSxsConfig, IServiceSxsConfig_Vtbl, 0xc7cd7379_f3f2_4634_811b_703281d73e08);
impl core::ops::Deref for IServiceSxsConfig {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServiceSxsConfig, windows_core::IUnknown);
impl IServiceSxsConfig {
    pub unsafe fn SxsConfig(&self, scsconfig: CSC_SxsConfig) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SxsConfig)(windows_core::Interface::as_raw(self), scsconfig).ok()
    }
    pub unsafe fn SxsName<P0>(&self, szsxsname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SxsName)(windows_core::Interface::as_raw(self), szsxsname.param().abi()).ok()
    }
    pub unsafe fn SxsDirectory<P0>(&self, szsxsdirectory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SxsDirectory)(windows_core::Interface::as_raw(self), szsxsdirectory.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IServiceSxsConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SxsConfig: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_SxsConfig) -> windows_core::HRESULT,
    pub SxsName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SxsDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServiceSynchronizationConfig, IServiceSynchronizationConfig_Vtbl, 0xfd880e81_6dce_4c58_af83_a208846c0030);
impl core::ops::Deref for IServiceSynchronizationConfig {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServiceSynchronizationConfig, windows_core::IUnknown);
impl IServiceSynchronizationConfig {
    pub unsafe fn ConfigureSynchronization(&self, synchconfig: CSC_SynchronizationConfig) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConfigureSynchronization)(windows_core::Interface::as_raw(self), synchconfig).ok()
    }
}
#[repr(C)]
pub struct IServiceSynchronizationConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConfigureSynchronization: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_SynchronizationConfig) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServiceSysTxnConfig, IServiceSysTxnConfig_Vtbl, 0x33caf1a1_fcb8_472b_b45e_967448ded6d8);
impl core::ops::Deref for IServiceSysTxnConfig {
    type Target = IServiceTransactionConfig;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServiceSysTxnConfig, windows_core::IUnknown, IServiceTransactionConfigBase, IServiceTransactionConfig);
impl IServiceSysTxnConfig {
    pub unsafe fn ConfigureBYOTSysTxn<P0>(&self, ptxproxy: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITransactionProxy>,
    {
        (windows_core::Interface::vtable(self).ConfigureBYOTSysTxn)(windows_core::Interface::as_raw(self), ptxproxy.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IServiceSysTxnConfig_Vtbl {
    pub base__: IServiceTransactionConfig_Vtbl,
    pub ConfigureBYOTSysTxn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServiceThreadPoolConfig, IServiceThreadPoolConfig_Vtbl, 0x186d89bc_f277_4bcc_80d5_4df7b836ef4a);
impl core::ops::Deref for IServiceThreadPoolConfig {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServiceThreadPoolConfig, windows_core::IUnknown);
impl IServiceThreadPoolConfig {
    pub unsafe fn SelectThreadPool(&self, threadpool: CSC_ThreadPool) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SelectThreadPool)(windows_core::Interface::as_raw(self), threadpool).ok()
    }
    pub unsafe fn SetBindingInfo(&self, binding: CSC_Binding) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBindingInfo)(windows_core::Interface::as_raw(self), binding).ok()
    }
}
#[repr(C)]
pub struct IServiceThreadPoolConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SelectThreadPool: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_ThreadPool) -> windows_core::HRESULT,
    pub SetBindingInfo: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_Binding) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServiceTrackerConfig, IServiceTrackerConfig_Vtbl, 0x6c3a3e1d_0ba6_4036_b76f_d0404db816c9);
impl core::ops::Deref for IServiceTrackerConfig {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServiceTrackerConfig, windows_core::IUnknown);
impl IServiceTrackerConfig {
    pub unsafe fn TrackerConfig<P0, P1>(&self, trackerconfig: CSC_TrackerConfig, sztrackerappname: P0, sztrackerctxname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).TrackerConfig)(windows_core::Interface::as_raw(self), trackerconfig, sztrackerappname.param().abi(), sztrackerctxname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IServiceTrackerConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TrackerConfig: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_TrackerConfig, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServiceTransactionConfig, IServiceTransactionConfig_Vtbl, 0x59f4c2a3_d3d7_4a31_b6e4_6ab3177c50b9);
impl core::ops::Deref for IServiceTransactionConfig {
    type Target = IServiceTransactionConfigBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServiceTransactionConfig, windows_core::IUnknown, IServiceTransactionConfigBase);
impl IServiceTransactionConfig {
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn ConfigureBYOT<P0>(&self, pitxbyot: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DistributedTransactionCoordinator::ITransaction>,
    {
        (windows_core::Interface::vtable(self).ConfigureBYOT)(windows_core::Interface::as_raw(self), pitxbyot.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IServiceTransactionConfig_Vtbl {
    pub base__: IServiceTransactionConfigBase_Vtbl,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub ConfigureBYOT: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    ConfigureBYOT: usize,
}
windows_core::imp::define_interface!(IServiceTransactionConfigBase, IServiceTransactionConfigBase_Vtbl, 0x772b3fbe_6ffd_42fb_b5f8_8f9b260f3810);
impl core::ops::Deref for IServiceTransactionConfigBase {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServiceTransactionConfigBase, windows_core::IUnknown);
impl IServiceTransactionConfigBase {
    pub unsafe fn ConfigureTransaction(&self, transactionconfig: CSC_TransactionConfig) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConfigureTransaction)(windows_core::Interface::as_raw(self), transactionconfig).ok()
    }
    pub unsafe fn IsolationLevel(&self, option: COMAdminTxIsolationLevelOptions) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsolationLevel)(windows_core::Interface::as_raw(self), option).ok()
    }
    pub unsafe fn TransactionTimeout(&self, ultimeoutsec: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TransactionTimeout)(windows_core::Interface::as_raw(self), ultimeoutsec).ok()
    }
    pub unsafe fn BringYourOwnTransaction<P0>(&self, sztipurl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).BringYourOwnTransaction)(windows_core::Interface::as_raw(self), sztipurl.param().abi()).ok()
    }
    pub unsafe fn NewTransactionDescription<P0>(&self, sztxdesc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).NewTransactionDescription)(windows_core::Interface::as_raw(self), sztxdesc.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IServiceTransactionConfigBase_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConfigureTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_TransactionConfig) -> windows_core::HRESULT,
    pub IsolationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, COMAdminTxIsolationLevelOptions) -> windows_core::HRESULT,
    pub TransactionTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub BringYourOwnTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub NewTransactionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISharedProperty, ISharedProperty_Vtbl, 0x2a005c01_a5de_11cf_9e66_00aa00a3f464);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISharedProperty {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISharedProperty, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISharedProperty {
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue<P0>(&self, val: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), val.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISharedProperty_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISharedPropertyGroup, ISharedPropertyGroup_Vtbl, 0x2a005c07_a5de_11cf_9e66_00aa00a3f464);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISharedPropertyGroup {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISharedPropertyGroup, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISharedPropertyGroup {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyByPosition(&self, index: i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut Option<ISharedProperty>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreatePropertyByPosition)(windows_core::Interface::as_raw(self), index, fexists, core::mem::transmute(ppprop)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_PropertyByPosition(&self, index: i32) -> windows_core::Result<ISharedProperty> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_PropertyByPosition)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateProperty<P0>(&self, name: P0, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut Option<ISharedProperty>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).CreateProperty)(windows_core::Interface::as_raw(self), name.param().abi(), fexists, core::mem::transmute(ppprop)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Property<P0>(&self, name: P0) -> windows_core::Result<ISharedProperty>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Property)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISharedPropertyGroup_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyByPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyByPosition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_PropertyByPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_PropertyByPosition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateProperty: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Property: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Property: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISharedPropertyGroupManager, ISharedPropertyGroupManager_Vtbl, 0x2a005c0d_a5de_11cf_9e66_00aa00a3f464);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISharedPropertyGroupManager {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISharedPropertyGroupManager, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISharedPropertyGroupManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyGroup<P0>(&self, name: P0, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppgroup: *mut Option<ISharedPropertyGroup>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).CreatePropertyGroup)(windows_core::Interface::as_raw(self), name.param().abi(), dwisomode, dwrelmode, fexists, core::mem::transmute(ppgroup)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Group<P0>(&self, name: P0) -> windows_core::Result<ISharedPropertyGroup>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Group)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISharedPropertyGroupManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32, *mut i32, *mut super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Group: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Group: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemAppEventData, ISystemAppEventData_Vtbl, 0xd6d48a3c_d5c5_49e7_8c74_99e4889ed52f);
impl core::ops::Deref for ISystemAppEventData {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISystemAppEventData, windows_core::IUnknown);
impl ISystemAppEventData {
    pub unsafe fn Startup(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Startup)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnDataChanged<P0>(&self, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: P0, dwreason: u32, u64tracehandle: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).OnDataChanged)(windows_core::Interface::as_raw(self), dwpid, dwmask, dwnumbersinks, bstrdwmethodmask.param().abi(), dwreason, u64tracehandle).ok()
    }
}
#[repr(C)]
pub struct ISystemAppEventData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Startup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDataChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, core::mem::MaybeUninit<windows_core::BSTR>, u32, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IThreadPoolKnobs, IThreadPoolKnobs_Vtbl, 0x51372af7_cae7_11cf_be81_00aa00a2fa25);
impl core::ops::Deref for IThreadPoolKnobs {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IThreadPoolKnobs, windows_core::IUnknown);
impl IThreadPoolKnobs {
    pub unsafe fn GetMaxThreads(&self, plcmaxthreads: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMaxThreads)(windows_core::Interface::as_raw(self), plcmaxthreads).ok()
    }
    pub unsafe fn GetCurrentThreads(&self, plccurrentthreads: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCurrentThreads)(windows_core::Interface::as_raw(self), plccurrentthreads).ok()
    }
    pub unsafe fn SetMaxThreads(&self, lcmaxthreads: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxThreads)(windows_core::Interface::as_raw(self), lcmaxthreads).ok()
    }
    pub unsafe fn GetDeleteDelay(&self, pmsecdeletedelay: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDeleteDelay)(windows_core::Interface::as_raw(self), pmsecdeletedelay).ok()
    }
    pub unsafe fn SetDeleteDelay(&self, msecdeletedelay: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDeleteDelay)(windows_core::Interface::as_raw(self), msecdeletedelay).ok()
    }
    pub unsafe fn GetMaxQueuedRequests(&self, plcmaxqueuedrequests: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMaxQueuedRequests)(windows_core::Interface::as_raw(self), plcmaxqueuedrequests).ok()
    }
    pub unsafe fn GetCurrentQueuedRequests(&self, plccurrentqueuedrequests: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCurrentQueuedRequests)(windows_core::Interface::as_raw(self), plccurrentqueuedrequests).ok()
    }
    pub unsafe fn SetMaxQueuedRequests(&self, lcmaxqueuedrequests: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxQueuedRequests)(windows_core::Interface::as_raw(self), lcmaxqueuedrequests).ok()
    }
    pub unsafe fn SetMinThreads(&self, lcminthreads: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinThreads)(windows_core::Interface::as_raw(self), lcminthreads).ok()
    }
    pub unsafe fn SetQueueDepth(&self, lcqueuedepth: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQueueDepth)(windows_core::Interface::as_raw(self), lcqueuedepth).ok()
    }
}
#[repr(C)]
pub struct IThreadPoolKnobs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMaxThreads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetCurrentThreads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxThreads: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetDeleteDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDeleteDelay: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetMaxQueuedRequests: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetCurrentQueuedRequests: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxQueuedRequests: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetMinThreads: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetQueueDepth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITransactionContext, ITransactionContext_Vtbl, 0x7999fc21_d3c6_11cf_acab_00a024a55aef);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITransactionContext {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITransactionContext, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITransactionContext {
    pub unsafe fn CreateInstance<P0>(&self, pszprogid: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), pszprogid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITransactionContext_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionContextEx, ITransactionContextEx_Vtbl, 0x7999fc22_d3c6_11cf_acab_00a024a55aef);
impl core::ops::Deref for ITransactionContextEx {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionContextEx, windows_core::IUnknown);
impl ITransactionContextEx {
    pub unsafe fn CreateInstance<T>(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), rclsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITransactionContextEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionProperty, ITransactionProperty_Vtbl, 0x788ea814_87b1_11d1_bba6_00c04fc2fa5f);
impl core::ops::Deref for ITransactionProperty {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionProperty, windows_core::IUnknown);
impl ITransactionProperty {
    pub unsafe fn Reserved1(&self) {
        (windows_core::Interface::vtable(self).Reserved1)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved2(&self) {
        (windows_core::Interface::vtable(self).Reserved2)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved3(&self) {
        (windows_core::Interface::vtable(self).Reserved3)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved4(&self) {
        (windows_core::Interface::vtable(self).Reserved4)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved5(&self) {
        (windows_core::Interface::vtable(self).Reserved5)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved6(&self) {
        (windows_core::Interface::vtable(self).Reserved6)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved7(&self) {
        (windows_core::Interface::vtable(self).Reserved7)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved8(&self) {
        (windows_core::Interface::vtable(self).Reserved8)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved9(&self) {
        (windows_core::Interface::vtable(self).Reserved9)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetTransactionResourcePool(&self) -> windows_core::Result<ITransactionResourcePool> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransactionResourcePool)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Reserved10(&self) {
        (windows_core::Interface::vtable(self).Reserved10)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved11(&self) {
        (windows_core::Interface::vtable(self).Reserved11)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved12(&self) {
        (windows_core::Interface::vtable(self).Reserved12)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved13(&self) {
        (windows_core::Interface::vtable(self).Reserved13)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved14(&self) {
        (windows_core::Interface::vtable(self).Reserved14)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved15(&self) {
        (windows_core::Interface::vtable(self).Reserved15)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved16(&self) {
        (windows_core::Interface::vtable(self).Reserved16)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved17(&self) {
        (windows_core::Interface::vtable(self).Reserved17)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct ITransactionProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Reserved1: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved2: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved3: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved4: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved5: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved6: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved7: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved8: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved9: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetTransactionResourcePool: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reserved10: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved11: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved12: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved13: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved14: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved15: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved16: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved17: unsafe extern "system" fn(*mut core::ffi::c_void),
}
windows_core::imp::define_interface!(ITransactionProxy, ITransactionProxy_Vtbl, 0x02558374_df2e_4dae_bd6b_1d5c994f9bdc);
impl core::ops::Deref for ITransactionProxy {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionProxy, windows_core::IUnknown);
impl ITransactionProxy {
    pub unsafe fn Commit(&self, guid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), core::mem::transmute(guid)).ok()
    }
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn Promote(&self) -> windows_core::Result<super::DistributedTransactionCoordinator::ITransaction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Promote)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn CreateVoter<P0>(&self, ptxasync: P0) -> windows_core::Result<super::DistributedTransactionCoordinator::ITransactionVoterBallotAsync2>
    where
        P0: windows_core::Param<super::DistributedTransactionCoordinator::ITransactionVoterNotifyAsync2>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateVoter)(windows_core::Interface::as_raw(self), ptxasync.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetIsolationLevel(&self, __midl__itransactionproxy0000: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIsolationLevel)(windows_core::Interface::as_raw(self), __midl__itransactionproxy0000).ok()
    }
    pub unsafe fn GetIdentifier(&self, pbstridentifier: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIdentifier)(windows_core::Interface::as_raw(self), pbstridentifier).ok()
    }
    pub unsafe fn IsReusable(&self, pfisreusable: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsReusable)(windows_core::Interface::as_raw(self), pfisreusable).ok()
    }
}
#[repr(C)]
pub struct ITransactionProxy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub Promote: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    Promote: usize,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub CreateVoter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    CreateVoter: usize,
    pub GetIsolationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub IsReusable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionResourcePool, ITransactionResourcePool_Vtbl, 0xc5feb7c1_346a_11d1_b1cc_00aa00ba3258);
impl core::ops::Deref for ITransactionResourcePool {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionResourcePool, windows_core::IUnknown);
impl ITransactionResourcePool {
    pub unsafe fn PutResource<P0, P1>(&self, ppool: P0, punk: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjPool>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).PutResource)(windows_core::Interface::as_raw(self), ppool.param().abi(), punk.param().abi()).ok()
    }
    pub unsafe fn GetResource<P0>(&self, ppool: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<IObjPool>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), ppool.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITransactionResourcePool_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PutResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionStatus, ITransactionStatus_Vtbl, 0x61f589e8_3724_4898_a0a4_664ae9e1d1b4);
impl core::ops::Deref for ITransactionStatus {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionStatus, windows_core::IUnknown);
impl ITransactionStatus {
    pub unsafe fn SetTransactionStatus(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTransactionStatus)(windows_core::Interface::as_raw(self), hrstatus).ok()
    }
    pub unsafe fn GetTransactionStatus(&self, phrstatus: *mut windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTransactionStatus)(windows_core::Interface::as_raw(self), phrstatus).ok()
    }
}
#[repr(C)]
pub struct ITransactionStatus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTransactionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetTransactionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITxProxyHolder, ITxProxyHolder_Vtbl, 0x13d86f31_0139_41af_bcad_c7d50435fe9f);
impl core::ops::Deref for ITxProxyHolder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITxProxyHolder, windows_core::IUnknown);
impl ITxProxyHolder {
    pub unsafe fn GetIdentifier(&self, pguidltx: *mut windows_core::GUID) {
        (windows_core::Interface::vtable(self).GetIdentifier)(windows_core::Interface::as_raw(self), pguidltx)
    }
}
#[repr(C)]
pub struct ITxProxyHolder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID),
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ObjectContext, ObjectContext_Vtbl, 0x74c08646_cedb_11cf_8b49_00aa00b8a790);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ObjectContext {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ObjectContext, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ObjectContext {
    pub unsafe fn CreateInstance<P0>(&self, bstrprogid: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), bstrprogid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetComplete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetComplete)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetAbort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAbort)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableCommit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableCommit)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableCommit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisableCommit)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsInTransaction(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsInTransaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsSecurityEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSecurityEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsCallerInRole<P0>(&self, bstrrole: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsCallerInRole)(windows_core::Interface::as_raw(self), bstrrole.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn get_Item<P0>(&self, name: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security(&self) -> windows_core::Result<SecurityProperty> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Security)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ContextInfo(&self) -> windows_core::Result<ContextInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ContextInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ObjectContext_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsSecurityEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsCallerInRole: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Security: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ContextInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ContextInfo: usize,
}
windows_core::imp::define_interface!(ObjectControl, ObjectControl_Vtbl, 0x7dc41850_0c31_11d0_8b79_00aa00b8a790);
impl core::ops::Deref for ObjectControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ObjectControl, windows_core::IUnknown);
impl ObjectControl {
    pub unsafe fn Activate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Deactivate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CanBePooled(&self, pbpoolable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CanBePooled)(windows_core::Interface::as_raw(self), pbpoolable).ok()
    }
}
#[repr(C)]
pub struct ObjectControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanBePooled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(SecurityProperty, SecurityProperty_Vtbl, 0xe74a7215_014d_11d1_a63c_00a0c911b4e0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for SecurityProperty {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(SecurityProperty, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl SecurityProperty {
    pub unsafe fn GetDirectCallerName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDirectCallerName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDirectCreatorName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDirectCreatorName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetOriginalCallerName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOriginalCallerName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetOriginalCreatorName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOriginalCreatorName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct SecurityProperty_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetDirectCallerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDirectCreatorName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetOriginalCallerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetOriginalCreatorName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
pub const APPTYPE_LIBRARY: COMPLUS_APPTYPE = COMPLUS_APPTYPE(0i32);
pub const APPTYPE_SERVER: COMPLUS_APPTYPE = COMPLUS_APPTYPE(1i32);
pub const APPTYPE_SWC: COMPLUS_APPTYPE = COMPLUS_APPTYPE(2i32);
pub const APPTYPE_UNKNOWN: COMPLUS_APPTYPE = COMPLUS_APPTYPE(-1i32);
pub const COMAdmin32BitComponent: COMAdminComponentType = COMAdminComponentType(1i32);
pub const COMAdmin64BitComponent: COMAdminComponentType = COMAdminComponentType(2i32);
pub const COMAdminAccessChecksApplicationComponentLevel: COMAdminAccessChecksLevelOptions = COMAdminAccessChecksLevelOptions(1i32);
pub const COMAdminAccessChecksApplicationLevel: COMAdminAccessChecksLevelOptions = COMAdminAccessChecksLevelOptions(0i32);
pub const COMAdminActivationInproc: COMAdminActivationOptions = COMAdminActivationOptions(0i32);
pub const COMAdminActivationLocal: COMAdminActivationOptions = COMAdminActivationOptions(1i32);
pub const COMAdminAuthenticationCall: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(3i32);
pub const COMAdminAuthenticationCapabilitiesDynamicCloaking: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(64i32);
pub const COMAdminAuthenticationCapabilitiesNone: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(0i32);
pub const COMAdminAuthenticationCapabilitiesSecureReference: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(2i32);
pub const COMAdminAuthenticationCapabilitiesStaticCloaking: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(32i32);
pub const COMAdminAuthenticationConnect: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(2i32);
pub const COMAdminAuthenticationDefault: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(0i32);
pub const COMAdminAuthenticationIntegrity: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(5i32);
pub const COMAdminAuthenticationNone: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(1i32);
pub const COMAdminAuthenticationPacket: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(4i32);
pub const COMAdminAuthenticationPrivacy: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(6i32);
pub const COMAdminCompFlagAlreadyInstalled: COMAdminComponentFlags = COMAdminComponentFlags(16i32);
pub const COMAdminCompFlagCOMPlusPropertiesFound: COMAdminComponentFlags = COMAdminComponentFlags(2i32);
pub const COMAdminCompFlagInterfacesFound: COMAdminComponentFlags = COMAdminComponentFlags(8i32);
pub const COMAdminCompFlagNotInApplication: COMAdminComponentFlags = COMAdminComponentFlags(32i32);
pub const COMAdminCompFlagProxyFound: COMAdminComponentFlags = COMAdminComponentFlags(4i32);
pub const COMAdminCompFlagTypeInfoFound: COMAdminComponentFlags = COMAdminComponentFlags(1i32);
pub const COMAdminErrAlreadyInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368508i32);
pub const COMAdminErrAppDirNotFound: COMAdminErrorCodes = COMAdminErrorCodes(-2146368481i32);
pub const COMAdminErrAppFileReadFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368504i32);
pub const COMAdminErrAppFileVersion: COMAdminErrorCodes = COMAdminErrorCodes(-2146368503i32);
pub const COMAdminErrAppFileWriteFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368505i32);
pub const COMAdminErrAppNotRunning: COMAdminErrorCodes = COMAdminErrorCodes(-2146367478i32);
pub const COMAdminErrApplicationExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368501i32);
pub const COMAdminErrApplidMatchesClsid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368442i32);
pub const COMAdminErrAuthenticationLevel: COMAdminErrorCodes = COMAdminErrorCodes(-2146368493i32);
pub const COMAdminErrBadPath: COMAdminErrorCodes = COMAdminErrorCodes(-2146368502i32);
pub const COMAdminErrBadRegistryLibID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368482i32);
pub const COMAdminErrBadRegistryProgID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368494i32);
pub const COMAdminErrBasePartitionOnly: COMAdminErrorCodes = COMAdminErrorCodes(-2146368432i32);
pub const COMAdminErrCLSIDOrIIDMismatch: COMAdminErrorCodes = COMAdminErrorCodes(-2146368488i32);
pub const COMAdminErrCanNotExportAppProxy: COMAdminErrorCodes = COMAdminErrorCodes(-2146368438i32);
pub const COMAdminErrCanNotExportSystemApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368436i32);
pub const COMAdminErrCanNotStartApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368437i32);
pub const COMAdminErrCanNotSubscribeToComponent: COMAdminErrorCodes = COMAdminErrorCodes(-2146368435i32);
pub const COMAdminErrCannotCopyEventClass: COMAdminErrorCodes = COMAdminErrorCodes(-2146367456i32);
pub const COMAdminErrCantCopyFile: COMAdminErrorCodes = COMAdminErrorCodes(-2146368499i32);
pub const COMAdminErrCantRecycleLibraryApps: COMAdminErrorCodes = COMAdminErrorCodes(-2146367473i32);
pub const COMAdminErrCantRecycleServiceApps: COMAdminErrorCodes = COMAdminErrorCodes(-2146367471i32);
pub const COMAdminErrCatBitnessMismatch: COMAdminErrorCodes = COMAdminErrorCodes(-2146368382i32);
pub const COMAdminErrCatPauseResumeNotSupported: COMAdminErrorCodes = COMAdminErrorCodes(-2146368379i32);
pub const COMAdminErrCatServerFault: COMAdminErrorCodes = COMAdminErrorCodes(-2146368378i32);
pub const COMAdminErrCatUnacceptableBitness: COMAdminErrorCodes = COMAdminErrorCodes(-2146368381i32);
pub const COMAdminErrCatWrongAppBitnessBitness: COMAdminErrorCodes = COMAdminErrorCodes(-2146368380i32);
pub const COMAdminErrCoReqCompInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368459i32);
pub const COMAdminErrCompFileBadTLB: COMAdminErrorCodes = COMAdminErrorCodes(-2146368472i32);
pub const COMAdminErrCompFileClassNotAvail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368473i32);
pub const COMAdminErrCompFileDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146368476i32);
pub const COMAdminErrCompFileGetClassObj: COMAdminErrorCodes = COMAdminErrorCodes(-2146368474i32);
pub const COMAdminErrCompFileLoadDLLFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368475i32);
pub const COMAdminErrCompFileNoRegistrar: COMAdminErrorCodes = COMAdminErrorCodes(-2146368460i32);
pub const COMAdminErrCompFileNotInstallable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368471i32);
pub const COMAdminErrCompMoveBadDest: COMAdminErrorCodes = COMAdminErrorCodes(-2146368466i32);
pub const COMAdminErrCompMoveDest: COMAdminErrorCodes = COMAdminErrorCodes(-2146367459i32);
pub const COMAdminErrCompMoveLocked: COMAdminErrorCodes = COMAdminErrorCodes(-2146368467i32);
pub const COMAdminErrCompMovePrivate: COMAdminErrorCodes = COMAdminErrorCodes(-2146367458i32);
pub const COMAdminErrCompMoveSource: COMAdminErrorCodes = COMAdminErrorCodes(-2146367460i32);
pub const COMAdminErrComponentExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368455i32);
pub const COMAdminErrDllLoadFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368483i32);
pub const COMAdminErrDllRegisterServer: COMAdminErrorCodes = COMAdminErrorCodes(-2146368486i32);
pub const COMAdminErrDuplicatePartitionName: COMAdminErrorCodes = COMAdminErrorCodes(-2146368425i32);
pub const COMAdminErrEventClassCannotBeSubscriber: COMAdminErrorCodes = COMAdminErrorCodes(-2146368434i32);
pub const COMAdminErrImportedComponentsNotAllowed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368421i32);
pub const COMAdminErrInvalidPartition: COMAdminErrorCodes = COMAdminErrorCodes(-2146367477i32);
pub const COMAdminErrInvalidUserids: COMAdminErrorCodes = COMAdminErrorCodes(-2146368496i32);
pub const COMAdminErrKeyMissing: COMAdminErrorCodes = COMAdminErrorCodes(-2146368509i32);
pub const COMAdminErrLibAppProxyIncompatible: COMAdminErrorCodes = COMAdminErrorCodes(-2146368433i32);
pub const COMAdminErrMigSchemaNotFound: COMAdminErrorCodes = COMAdminErrorCodes(-2146368383i32);
pub const COMAdminErrMigVersionNotSupported: COMAdminErrorCodes = COMAdminErrorCodes(-2146368384i32);
pub const COMAdminErrNoRegistryCLSID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368495i32);
pub const COMAdminErrNoServerShare: COMAdminErrorCodes = COMAdminErrorCodes(-2146368485i32);
pub const COMAdminErrNoUser: COMAdminErrorCodes = COMAdminErrorCodes(-2146368497i32);
pub const COMAdminErrNotChangeable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368470i32);
pub const COMAdminErrNotDeletable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368469i32);
pub const COMAdminErrNotInRegistry: COMAdminErrorCodes = COMAdminErrorCodes(-2146368450i32);
pub const COMAdminErrObjectDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146367479i32);
pub const COMAdminErrObjectErrors: COMAdminErrorCodes = COMAdminErrorCodes(-2146368511i32);
pub const COMAdminErrObjectExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368456i32);
pub const COMAdminErrObjectInvalid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368510i32);
pub const COMAdminErrObjectNotPoolable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368449i32);
pub const COMAdminErrObjectParentMissing: COMAdminErrorCodes = COMAdminErrorCodes(-2146367480i32);
pub const COMAdminErrPartitionInUse: COMAdminErrorCodes = COMAdminErrorCodes(-2146368423i32);
pub const COMAdminErrPartitionMsiOnly: COMAdminErrorCodes = COMAdminErrorCodes(-2146367463i32);
pub const COMAdminErrPausedProcessMayNotBeRecycled: COMAdminErrorCodes = COMAdminErrorCodes(-2146367469i32);
pub const COMAdminErrProcessAlreadyRecycled: COMAdminErrorCodes = COMAdminErrorCodes(-2146367470i32);
pub const COMAdminErrPropertyOverflow: COMAdminErrorCodes = COMAdminErrorCodes(-2146368452i32);
pub const COMAdminErrPropertySaveFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368457i32);
pub const COMAdminErrQueuingServiceNotAvailable: COMAdminErrorCodes = COMAdminErrorCodes(-2146367998i32);
pub const COMAdminErrRegFileCorrupt: COMAdminErrorCodes = COMAdminErrorCodes(-2146368453i32);
pub const COMAdminErrRegdbAlreadyRunning: COMAdminErrorCodes = COMAdminErrorCodes(-2146368395i32);
pub const COMAdminErrRegdbNotInitialized: COMAdminErrorCodes = COMAdminErrorCodes(-2146368398i32);
pub const COMAdminErrRegdbNotOpen: COMAdminErrorCodes = COMAdminErrorCodes(-2146368397i32);
pub const COMAdminErrRegdbSystemErr: COMAdminErrorCodes = COMAdminErrorCodes(-2146368396i32);
pub const COMAdminErrRegisterTLB: COMAdminErrorCodes = COMAdminErrorCodes(-2146368464i32);
pub const COMAdminErrRegistrarFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368477i32);
pub const COMAdminErrRemoteInterface: COMAdminErrorCodes = COMAdminErrorCodes(-2146368487i32);
pub const COMAdminErrRequiresDifferentPlatform: COMAdminErrorCodes = COMAdminErrorCodes(-2146368439i32);
pub const COMAdminErrRoleDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146368441i32);
pub const COMAdminErrRoleExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368500i32);
pub const COMAdminErrServiceNotInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368458i32);
pub const COMAdminErrSession: COMAdminErrorCodes = COMAdminErrorCodes(-2146368468i32);
pub const COMAdminErrStartAppDisabled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368431i32);
pub const COMAdminErrStartAppNeedsComponents: COMAdminErrorCodes = COMAdminErrorCodes(-2146368440i32);
pub const COMAdminErrSystemApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368461i32);
pub const COMAdminErrUserPasswdNotValid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368492i32);
pub const COMAdminExportApplicationProxy: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(2i32);
pub const COMAdminExportForceOverwriteOfFiles: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(4i32);
pub const COMAdminExportIn10Format: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(16i32);
pub const COMAdminExportNoUsers: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(0i32);
pub const COMAdminExportUsers: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(1i32);
pub const COMAdminFileFlagAlreadyInstalled: COMAdminFileFlags = COMAdminFileFlags(512i32);
pub const COMAdminFileFlagBadTLB: COMAdminFileFlags = COMAdminFileFlags(1024i32);
pub const COMAdminFileFlagCOM: COMAdminFileFlags = COMAdminFileFlags(2i32);
pub const COMAdminFileFlagClassNotAvailable: COMAdminFileFlags = COMAdminFileFlags(4096i32);
pub const COMAdminFileFlagContainsComp: COMAdminFileFlags = COMAdminFileFlags(8i32);
pub const COMAdminFileFlagContainsPS: COMAdminFileFlags = COMAdminFileFlags(4i32);
pub const COMAdminFileFlagContainsTLB: COMAdminFileFlags = COMAdminFileFlags(16i32);
pub const COMAdminFileFlagDLLRegsvrFailed: COMAdminFileFlags = COMAdminFileFlags(32768i32);
pub const COMAdminFileFlagDoesNotExist: COMAdminFileFlags = COMAdminFileFlags(256i32);
pub const COMAdminFileFlagError: COMAdminFileFlags = COMAdminFileFlags(262144i32);
pub const COMAdminFileFlagGetClassObjFailed: COMAdminFileFlags = COMAdminFileFlags(2048i32);
pub const COMAdminFileFlagLoadable: COMAdminFileFlags = COMAdminFileFlags(1i32);
pub const COMAdminFileFlagNoRegistrar: COMAdminFileFlags = COMAdminFileFlags(16384i32);
pub const COMAdminFileFlagRegTLBFailed: COMAdminFileFlags = COMAdminFileFlags(65536i32);
pub const COMAdminFileFlagRegistrar: COMAdminFileFlags = COMAdminFileFlags(8192i32);
pub const COMAdminFileFlagRegistrarFailed: COMAdminFileFlags = COMAdminFileFlags(131072i32);
pub const COMAdminFileFlagSelfReg: COMAdminFileFlags = COMAdminFileFlags(32i32);
pub const COMAdminFileFlagSelfUnReg: COMAdminFileFlags = COMAdminFileFlags(64i32);
pub const COMAdminFileFlagUnloadableDLL: COMAdminFileFlags = COMAdminFileFlags(128i32);
pub const COMAdminImpersonationAnonymous: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(1i32);
pub const COMAdminImpersonationDelegate: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(4i32);
pub const COMAdminImpersonationIdentify: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(2i32);
pub const COMAdminImpersonationImpersonate: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(3i32);
pub const COMAdminInUseByCatalog: COMAdminInUse = COMAdminInUse(1i32);
pub const COMAdminInUseByRegistryClsid: COMAdminInUse = COMAdminInUse(5i32);
pub const COMAdminInUseByRegistryProxyStub: COMAdminInUse = COMAdminInUse(3i32);
pub const COMAdminInUseByRegistryTypeLib: COMAdminInUse = COMAdminInUse(4i32);
pub const COMAdminInUseByRegistryUnknown: COMAdminInUse = COMAdminInUse(2i32);
pub const COMAdminInstallForceOverwriteOfFiles: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(2i32);
pub const COMAdminInstallNoUsers: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(0i32);
pub const COMAdminInstallUsers: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(1i32);
pub const COMAdminNotInUse: COMAdminInUse = COMAdminInUse(0i32);
pub const COMAdminOSNotInitialized: COMAdminOS = COMAdminOS(0i32);
pub const COMAdminOSUnknown: COMAdminOS = COMAdminOS(6i32);
pub const COMAdminOSWindows2000: COMAdminOS = COMAdminOS(3i32);
pub const COMAdminOSWindows2000AdvancedServer: COMAdminOS = COMAdminOS(4i32);
pub const COMAdminOSWindows2000Unknown: COMAdminOS = COMAdminOS(5i32);
pub const COMAdminOSWindows3_1: COMAdminOS = COMAdminOS(1i32);
pub const COMAdminOSWindows7DatacenterServer: COMAdminOS = COMAdminOS(27i32);
pub const COMAdminOSWindows7EnterpriseServer: COMAdminOS = COMAdminOS(26i32);
pub const COMAdminOSWindows7Personal: COMAdminOS = COMAdminOS(23i32);
pub const COMAdminOSWindows7Professional: COMAdminOS = COMAdminOS(24i32);
pub const COMAdminOSWindows7StandardServer: COMAdminOS = COMAdminOS(25i32);
pub const COMAdminOSWindows7WebServer: COMAdminOS = COMAdminOS(28i32);
pub const COMAdminOSWindows8DatacenterServer: COMAdminOS = COMAdminOS(33i32);
pub const COMAdminOSWindows8EnterpriseServer: COMAdminOS = COMAdminOS(32i32);
pub const COMAdminOSWindows8Personal: COMAdminOS = COMAdminOS(29i32);
pub const COMAdminOSWindows8Professional: COMAdminOS = COMAdminOS(30i32);
pub const COMAdminOSWindows8StandardServer: COMAdminOS = COMAdminOS(31i32);
pub const COMAdminOSWindows8WebServer: COMAdminOS = COMAdminOS(34i32);
pub const COMAdminOSWindows9x: COMAdminOS = COMAdminOS(2i32);
pub const COMAdminOSWindowsBlueDatacenterServer: COMAdminOS = COMAdminOS(39i32);
pub const COMAdminOSWindowsBlueEnterpriseServer: COMAdminOS = COMAdminOS(38i32);
pub const COMAdminOSWindowsBluePersonal: COMAdminOS = COMAdminOS(35i32);
pub const COMAdminOSWindowsBlueProfessional: COMAdminOS = COMAdminOS(36i32);
pub const COMAdminOSWindowsBlueStandardServer: COMAdminOS = COMAdminOS(37i32);
pub const COMAdminOSWindowsBlueWebServer: COMAdminOS = COMAdminOS(40i32);
pub const COMAdminOSWindowsLonghornDatacenterServer: COMAdminOS = COMAdminOS(21i32);
pub const COMAdminOSWindowsLonghornEnterpriseServer: COMAdminOS = COMAdminOS(20i32);
pub const COMAdminOSWindowsLonghornPersonal: COMAdminOS = COMAdminOS(17i32);
pub const COMAdminOSWindowsLonghornProfessional: COMAdminOS = COMAdminOS(18i32);
pub const COMAdminOSWindowsLonghornStandardServer: COMAdminOS = COMAdminOS(19i32);
pub const COMAdminOSWindowsLonghornWebServer: COMAdminOS = COMAdminOS(22i32);
pub const COMAdminOSWindowsNETDatacenterServer: COMAdminOS = COMAdminOS(15i32);
pub const COMAdminOSWindowsNETEnterpriseServer: COMAdminOS = COMAdminOS(14i32);
pub const COMAdminOSWindowsNETStandardServer: COMAdminOS = COMAdminOS(13i32);
pub const COMAdminOSWindowsNETWebServer: COMAdminOS = COMAdminOS(16i32);
pub const COMAdminOSWindowsXPPersonal: COMAdminOS = COMAdminOS(11i32);
pub const COMAdminOSWindowsXPProfessional: COMAdminOS = COMAdminOS(12i32);
pub const COMAdminQCMessageAuthenticateOff: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(1i32);
pub const COMAdminQCMessageAuthenticateOn: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(2i32);
pub const COMAdminQCMessageAuthenticateSecureApps: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(0i32);
pub const COMAdminServiceContinuePending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(4i32);
pub const COMAdminServiceLoadBalanceRouter: COMAdminServiceOptions = COMAdminServiceOptions(1i32);
pub const COMAdminServicePausePending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(5i32);
pub const COMAdminServicePaused: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(6i32);
pub const COMAdminServiceRunning: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(3i32);
pub const COMAdminServiceStartPending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(1i32);
pub const COMAdminServiceStopPending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(2i32);
pub const COMAdminServiceStopped: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(0i32);
pub const COMAdminServiceUnknownState: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(7i32);
pub const COMAdminSynchronizationIgnored: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(0i32);
pub const COMAdminSynchronizationNone: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(1i32);
pub const COMAdminSynchronizationRequired: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(3i32);
pub const COMAdminSynchronizationRequiresNew: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(4i32);
pub const COMAdminSynchronizationSupported: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(2i32);
pub const COMAdminThreadingModelApartment: COMAdminThreadingModels = COMAdminThreadingModels(0i32);
pub const COMAdminThreadingModelBoth: COMAdminThreadingModels = COMAdminThreadingModels(3i32);
pub const COMAdminThreadingModelFree: COMAdminThreadingModels = COMAdminThreadingModels(1i32);
pub const COMAdminThreadingModelMain: COMAdminThreadingModels = COMAdminThreadingModels(2i32);
pub const COMAdminThreadingModelNeutral: COMAdminThreadingModels = COMAdminThreadingModels(4i32);
pub const COMAdminThreadingModelNotSpecified: COMAdminThreadingModels = COMAdminThreadingModels(5i32);
pub const COMAdminTransactionIgnored: COMAdminTransactionOptions = COMAdminTransactionOptions(0i32);
pub const COMAdminTransactionNone: COMAdminTransactionOptions = COMAdminTransactionOptions(1i32);
pub const COMAdminTransactionRequired: COMAdminTransactionOptions = COMAdminTransactionOptions(3i32);
pub const COMAdminTransactionRequiresNew: COMAdminTransactionOptions = COMAdminTransactionOptions(4i32);
pub const COMAdminTransactionSupported: COMAdminTransactionOptions = COMAdminTransactionOptions(2i32);
pub const COMAdminTxIsolationLevelAny: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(0i32);
pub const COMAdminTxIsolationLevelReadCommitted: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(2i32);
pub const COMAdminTxIsolationLevelReadUnCommitted: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(1i32);
pub const COMAdminTxIsolationLevelRepeatableRead: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(3i32);
pub const COMAdminTxIsolationLevelSerializable: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(4i32);
pub const CRMFLAG_FORGETTARGET: CRMFLAGS = CRMFLAGS(1i32);
pub const CRMFLAG_REPLAYINPROGRESS: CRMFLAGS = CRMFLAGS(64i32);
pub const CRMFLAG_WRITTENDURINGABORT: CRMFLAGS = CRMFLAGS(8i32);
pub const CRMFLAG_WRITTENDURINGCOMMIT: CRMFLAGS = CRMFLAGS(4i32);
pub const CRMFLAG_WRITTENDURINGPREPARE: CRMFLAGS = CRMFLAGS(2i32);
pub const CRMFLAG_WRITTENDURINGRECOVERY: CRMFLAGS = CRMFLAGS(16i32);
pub const CRMFLAG_WRITTENDURINGREPLAY: CRMFLAGS = CRMFLAGS(32i32);
pub const CRMREGFLAG_ABORTPHASE: CRMREGFLAGS = CRMREGFLAGS(4i32);
pub const CRMREGFLAG_ALLPHASES: CRMREGFLAGS = CRMREGFLAGS(7i32);
pub const CRMREGFLAG_COMMITPHASE: CRMREGFLAGS = CRMREGFLAGS(2i32);
pub const CRMREGFLAG_FAILIFINDOUBTSREMAIN: CRMREGFLAGS = CRMREGFLAGS(16i32);
pub const CRMREGFLAG_PREPAREPHASE: CRMREGFLAGS = CRMREGFLAGS(1i32);
pub const CRR_ACTIVATION_LIMIT: u32 = 4294967294u32;
pub const CRR_CALL_LIMIT: u32 = 4294967293u32;
pub const CRR_LIFETIME_LIMIT: u32 = 4294967295u32;
pub const CRR_MEMORY_LIMIT: u32 = 4294967292u32;
pub const CRR_NO_REASON_SUPPLIED: u32 = 0u32;
pub const CRR_RECYCLED_FROM_UI: u32 = 4294967291u32;
pub const CSC_BindToPoolThread: CSC_Binding = CSC_Binding(1i32);
pub const CSC_CreateTransactionIfNecessary: CSC_TransactionConfig = CSC_TransactionConfig(2i32);
pub const CSC_DontUseTracker: CSC_TrackerConfig = CSC_TrackerConfig(0i32);
pub const CSC_IfContainerIsSynchronized: CSC_SynchronizationConfig = CSC_SynchronizationConfig(1i32);
pub const CSC_IfContainerIsTransactional: CSC_TransactionConfig = CSC_TransactionConfig(1i32);
pub const CSC_Ignore: CSC_InheritanceConfig = CSC_InheritanceConfig(1i32);
pub const CSC_Inherit: CSC_InheritanceConfig = CSC_InheritanceConfig(0i32);
pub const CSC_InheritCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = CSC_COMTIIntrinsicsConfig(1i32);
pub const CSC_InheritIISIntrinsics: CSC_IISIntrinsicsConfig = CSC_IISIntrinsicsConfig(1i32);
pub const CSC_InheritPartition: CSC_PartitionConfig = CSC_PartitionConfig(1i32);
pub const CSC_InheritSxs: CSC_SxsConfig = CSC_SxsConfig(1i32);
pub const CSC_MTAThreadPool: CSC_ThreadPool = CSC_ThreadPool(3i32);
pub const CSC_NewPartition: CSC_PartitionConfig = CSC_PartitionConfig(2i32);
pub const CSC_NewSxs: CSC_SxsConfig = CSC_SxsConfig(2i32);
pub const CSC_NewSynchronization: CSC_SynchronizationConfig = CSC_SynchronizationConfig(3i32);
pub const CSC_NewSynchronizationIfNecessary: CSC_SynchronizationConfig = CSC_SynchronizationConfig(2i32);
pub const CSC_NewTransaction: CSC_TransactionConfig = CSC_TransactionConfig(3i32);
pub const CSC_NoBinding: CSC_Binding = CSC_Binding(0i32);
pub const CSC_NoCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = CSC_COMTIIntrinsicsConfig(0i32);
pub const CSC_NoIISIntrinsics: CSC_IISIntrinsicsConfig = CSC_IISIntrinsicsConfig(0i32);
pub const CSC_NoPartition: CSC_PartitionConfig = CSC_PartitionConfig(0i32);
pub const CSC_NoSxs: CSC_SxsConfig = CSC_SxsConfig(0i32);
pub const CSC_NoSynchronization: CSC_SynchronizationConfig = CSC_SynchronizationConfig(0i32);
pub const CSC_NoTransaction: CSC_TransactionConfig = CSC_TransactionConfig(0i32);
pub const CSC_STAThreadPool: CSC_ThreadPool = CSC_ThreadPool(2i32);
pub const CSC_ThreadPoolInherit: CSC_ThreadPool = CSC_ThreadPool(1i32);
pub const CSC_ThreadPoolNone: CSC_ThreadPool = CSC_ThreadPool(0i32);
pub const CSC_UseTracker: CSC_TrackerConfig = CSC_TrackerConfig(1i32);
pub const DATA_NOT_AVAILABLE: u32 = 4294967295u32;
pub const DUMPTYPE_FULL: DUMPTYPE = DUMPTYPE(0i32);
pub const DUMPTYPE_MINI: DUMPTYPE = DUMPTYPE(1i32);
pub const DUMPTYPE_NONE: DUMPTYPE = DUMPTYPE(2i32);
pub const GATD_INCLUDE_APPLICATION_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(16i32);
pub const GATD_INCLUDE_CLASS_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(8i32);
pub const GATD_INCLUDE_LIBRARY_APPS: GetAppTrackerDataFlags = GetAppTrackerDataFlags(2i32);
pub const GATD_INCLUDE_PROCESS_EXE_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(1i32);
pub const GATD_INCLUDE_SWC: GetAppTrackerDataFlags = GetAppTrackerDataFlags(4i32);
pub const GUID_STRING_SIZE: u32 = 40u32;
pub const LockMethod: LockModes = LockModes(1i32);
pub const LockSetGet: LockModes = LockModes(0i32);
pub const MTXDM_E_ENLISTRESOURCEFAILED: u32 = 2147803392u32;
pub const Process: ReleaseModes = ReleaseModes(1i32);
pub const Standard: ReleaseModes = ReleaseModes(0i32);
pub const TRACKER_INIT_EVENT: windows_core::PCWSTR = windows_core::w!("Global\\COM+ Tracker Init Event");
pub const TRACKER_STARTSTOP_EVENT: windows_core::PCWSTR = windows_core::w!("Global\\COM+ Tracker Push Event");
pub const TRKCOLL_APPLICATIONS: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(1i32);
pub const TRKCOLL_COMPONENTS: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(2i32);
pub const TRKCOLL_PROCESSES: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(0i32);
pub const TxAbort: TransactionVote = TransactionVote(1i32);
pub const TxCommit: TransactionVote = TransactionVote(0i32);
pub const TxState_Aborted: CrmTransactionState = CrmTransactionState(2i32);
pub const TxState_Active: CrmTransactionState = CrmTransactionState(0i32);
pub const TxState_Committed: CrmTransactionState = CrmTransactionState(1i32);
pub const TxState_Indoubt: CrmTransactionState = CrmTransactionState(3i32);
pub const comQCErrApplicationNotQueued: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599296u32);
pub const comQCErrNoQueueableInterfaces: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599297u32);
pub const comQCErrQueueTransactMismatch: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599299u32);
pub const comQCErrQueuingServiceNotAvailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599298u32);
pub const comqcErrBadMarshaledObject: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599382u32);
pub const comqcErrInvalidMessage: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599376u32);
pub const comqcErrMarshaledObjSameTxn: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599304u32);
pub const comqcErrMsgNotAuthenticated: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599380u32);
pub const comqcErrMsmqConnectorUsed: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599381u32);
pub const comqcErrMsmqServiceUnavailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599379u32);
pub const comqcErrMsmqSidUnavailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599377u32);
pub const comqcErrOutParam: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599301u32);
pub const comqcErrPSLoad: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599303u32);
pub const comqcErrRecorderMarshalled: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599300u32);
pub const comqcErrRecorderNotTrusted: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599302u32);
pub const comqcErrWrongMsgExtension: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599378u32);
pub const mtsErrCtxAborted: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803138u32);
pub const mtsErrCtxAborting: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803139u32);
pub const mtsErrCtxNoContext: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803140u32);
pub const mtsErrCtxNoSecurity: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803149u32);
pub const mtsErrCtxNotRegistered: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803141u32);
pub const mtsErrCtxOldReference: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803143u32);
pub const mtsErrCtxRoleNotFound: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803148u32);
pub const mtsErrCtxSynchTimeout: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803142u32);
pub const mtsErrCtxTMNotAvailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803151u32);
pub const mtsErrCtxWrongThread: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803150u32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AutoSvcs_Error_Constants(pub u32);
impl windows_core::TypeKind for AutoSvcs_Error_Constants {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AutoSvcs_Error_Constants {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AutoSvcs_Error_Constants").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminAccessChecksLevelOptions(pub i32);
impl windows_core::TypeKind for COMAdminAccessChecksLevelOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminAccessChecksLevelOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminAccessChecksLevelOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminActivationOptions(pub i32);
impl windows_core::TypeKind for COMAdminActivationOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminActivationOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminActivationOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminApplicationExportOptions(pub i32);
impl windows_core::TypeKind for COMAdminApplicationExportOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminApplicationExportOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminApplicationExportOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminApplicationInstallOptions(pub i32);
impl windows_core::TypeKind for COMAdminApplicationInstallOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminApplicationInstallOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminApplicationInstallOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminAuthenticationCapabilitiesOptions(pub i32);
impl windows_core::TypeKind for COMAdminAuthenticationCapabilitiesOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminAuthenticationCapabilitiesOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminAuthenticationCapabilitiesOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminAuthenticationLevelOptions(pub i32);
impl windows_core::TypeKind for COMAdminAuthenticationLevelOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminAuthenticationLevelOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminAuthenticationLevelOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminComponentFlags(pub i32);
impl windows_core::TypeKind for COMAdminComponentFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminComponentFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminComponentFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminComponentType(pub i32);
impl windows_core::TypeKind for COMAdminComponentType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminComponentType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminComponentType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminErrorCodes(pub i32);
impl windows_core::TypeKind for COMAdminErrorCodes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminErrorCodes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminErrorCodes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminFileFlags(pub i32);
impl windows_core::TypeKind for COMAdminFileFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminFileFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminFileFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminImpersonationLevelOptions(pub i32);
impl windows_core::TypeKind for COMAdminImpersonationLevelOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminImpersonationLevelOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminImpersonationLevelOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminInUse(pub i32);
impl windows_core::TypeKind for COMAdminInUse {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminInUse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminInUse").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminOS(pub i32);
impl windows_core::TypeKind for COMAdminOS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminOS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminOS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminQCMessageAuthenticateOptions(pub i32);
impl windows_core::TypeKind for COMAdminQCMessageAuthenticateOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminQCMessageAuthenticateOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminQCMessageAuthenticateOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminServiceOptions(pub i32);
impl windows_core::TypeKind for COMAdminServiceOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminServiceOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminServiceOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminServiceStatusOptions(pub i32);
impl windows_core::TypeKind for COMAdminServiceStatusOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminServiceStatusOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminServiceStatusOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminSynchronizationOptions(pub i32);
impl windows_core::TypeKind for COMAdminSynchronizationOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminSynchronizationOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminSynchronizationOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminThreadingModels(pub i32);
impl windows_core::TypeKind for COMAdminThreadingModels {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminThreadingModels {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminThreadingModels").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminTransactionOptions(pub i32);
impl windows_core::TypeKind for COMAdminTransactionOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminTransactionOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminTransactionOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMAdminTxIsolationLevelOptions(pub i32);
impl windows_core::TypeKind for COMAdminTxIsolationLevelOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMAdminTxIsolationLevelOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMAdminTxIsolationLevelOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMPLUS_APPTYPE(pub i32);
impl windows_core::TypeKind for COMPLUS_APPTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMPLUS_APPTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMPLUS_APPTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CRMFLAGS(pub i32);
impl windows_core::TypeKind for CRMFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CRMFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CRMFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CRMREGFLAGS(pub i32);
impl windows_core::TypeKind for CRMREGFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CRMREGFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CRMREGFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CSC_Binding(pub i32);
impl windows_core::TypeKind for CSC_Binding {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CSC_Binding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CSC_Binding").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CSC_COMTIIntrinsicsConfig(pub i32);
impl windows_core::TypeKind for CSC_COMTIIntrinsicsConfig {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CSC_COMTIIntrinsicsConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CSC_COMTIIntrinsicsConfig").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CSC_IISIntrinsicsConfig(pub i32);
impl windows_core::TypeKind for CSC_IISIntrinsicsConfig {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CSC_IISIntrinsicsConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CSC_IISIntrinsicsConfig").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CSC_InheritanceConfig(pub i32);
impl windows_core::TypeKind for CSC_InheritanceConfig {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CSC_InheritanceConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CSC_InheritanceConfig").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CSC_PartitionConfig(pub i32);
impl windows_core::TypeKind for CSC_PartitionConfig {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CSC_PartitionConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CSC_PartitionConfig").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CSC_SxsConfig(pub i32);
impl windows_core::TypeKind for CSC_SxsConfig {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CSC_SxsConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CSC_SxsConfig").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CSC_SynchronizationConfig(pub i32);
impl windows_core::TypeKind for CSC_SynchronizationConfig {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CSC_SynchronizationConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CSC_SynchronizationConfig").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CSC_ThreadPool(pub i32);
impl windows_core::TypeKind for CSC_ThreadPool {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CSC_ThreadPool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CSC_ThreadPool").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CSC_TrackerConfig(pub i32);
impl windows_core::TypeKind for CSC_TrackerConfig {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CSC_TrackerConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CSC_TrackerConfig").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CSC_TransactionConfig(pub i32);
impl windows_core::TypeKind for CSC_TransactionConfig {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CSC_TransactionConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CSC_TransactionConfig").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CrmTransactionState(pub i32);
impl windows_core::TypeKind for CrmTransactionState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CrmTransactionState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CrmTransactionState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DUMPTYPE(pub i32);
impl windows_core::TypeKind for DUMPTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DUMPTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DUMPTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GetAppTrackerDataFlags(pub i32);
impl windows_core::TypeKind for GetAppTrackerDataFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GetAppTrackerDataFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GetAppTrackerDataFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LockModes(pub i32);
impl windows_core::TypeKind for LockModes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LockModes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LockModes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ReleaseModes(pub i32);
impl windows_core::TypeKind for ReleaseModes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ReleaseModes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ReleaseModes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TRACKING_COLL_TYPE(pub i32);
impl windows_core::TypeKind for TRACKING_COLL_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TRACKING_COLL_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TRACKING_COLL_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TransactionVote(pub i32);
impl windows_core::TypeKind for TransactionVote {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TransactionVote {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TransactionVote").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct APPDATA {
    pub m_idApp: u32,
    pub m_szAppGuid: [u16; 40],
    pub m_dwAppProcessId: u32,
    pub m_AppStatistics: APPSTATISTICS,
}
impl windows_core::TypeKind for APPDATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for APPDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct APPSTATISTICS {
    pub m_cTotalCalls: u32,
    pub m_cTotalInstances: u32,
    pub m_cTotalClasses: u32,
    pub m_cCallsPerSecond: u32,
}
impl windows_core::TypeKind for APPSTATISTICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for APPSTATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AppDomainHelper: windows_core::GUID = windows_core::GUID::from_u128(0xef24f689_14f8_4d92_b4af_d7b1f0e70fd4);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApplicationProcessRecycleInfo {
    pub IsRecyclable: super::super::Foundation::BOOL,
    pub IsRecycled: super::super::Foundation::BOOL,
    pub TimeRecycled: super::super::Foundation::FILETIME,
    pub TimeToTerminate: super::super::Foundation::FILETIME,
    pub RecycleReasonCode: i32,
    pub IsPendingRecycle: super::super::Foundation::BOOL,
    pub HasAutomaticLifetimeRecycling: super::super::Foundation::BOOL,
    pub TimeForAutomaticRecycling: super::super::Foundation::FILETIME,
    pub MemoryLimitInKB: u32,
    pub MemoryUsageInKBLastCheck: u32,
    pub ActivationLimit: u32,
    pub NumActivationsLastReported: u32,
    pub CallLimit: u32,
    pub NumCallsLastReported: u32,
}
impl windows_core::TypeKind for ApplicationProcessRecycleInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for ApplicationProcessRecycleInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApplicationProcessStatistics {
    pub NumCallsOutstanding: u32,
    pub NumTrackedComponents: u32,
    pub NumComponentInstances: u32,
    pub AvgCallsPerSecond: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: u32,
    pub Reserved4: u32,
}
impl windows_core::TypeKind for ApplicationProcessStatistics {
    type TypeKind = windows_core::CopyType;
}
impl Default for ApplicationProcessStatistics {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApplicationProcessSummary {
    pub PartitionIdPrimaryApplication: windows_core::GUID,
    pub ApplicationIdPrimaryApplication: windows_core::GUID,
    pub ApplicationInstanceId: windows_core::GUID,
    pub ProcessId: u32,
    pub Type: COMPLUS_APPTYPE,
    pub ProcessExeName: windows_core::PWSTR,
    pub IsService: super::super::Foundation::BOOL,
    pub IsPaused: super::super::Foundation::BOOL,
    pub IsRecycled: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for ApplicationProcessSummary {
    type TypeKind = windows_core::CopyType;
}
impl Default for ApplicationProcessSummary {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApplicationSummary {
    pub ApplicationInstanceId: windows_core::GUID,
    pub PartitionId: windows_core::GUID,
    pub ApplicationId: windows_core::GUID,
    pub Type: COMPLUS_APPTYPE,
    pub ApplicationName: windows_core::PWSTR,
    pub NumTrackedComponents: u32,
    pub NumComponentInstances: u32,
}
impl windows_core::TypeKind for ApplicationSummary {
    type TypeKind = windows_core::CopyType;
}
impl Default for ApplicationSummary {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ByotServerEx: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0aa_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLSIDDATA {
    pub m_clsid: windows_core::GUID,
    pub m_cReferences: u32,
    pub m_cBound: u32,
    pub m_cPooled: u32,
    pub m_cInCall: u32,
    pub m_dwRespTime: u32,
    pub m_cCallsCompleted: u32,
    pub m_cCallsFailed: u32,
}
impl windows_core::TypeKind for CLSIDDATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLSIDDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLSIDDATA2 {
    pub m_clsid: windows_core::GUID,
    pub m_appid: windows_core::GUID,
    pub m_partid: windows_core::GUID,
    pub m_pwszAppName: windows_core::PWSTR,
    pub m_pwszCtxName: windows_core::PWSTR,
    pub m_eAppType: COMPLUS_APPTYPE,
    pub m_cReferences: u32,
    pub m_cBound: u32,
    pub m_cPooled: u32,
    pub m_cInCall: u32,
    pub m_dwRespTime: u32,
    pub m_cCallsCompleted: u32,
    pub m_cCallsFailed: u32,
}
impl windows_core::TypeKind for CLSIDDATA2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLSIDDATA2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COMAdminCatalog: windows_core::GUID = windows_core::GUID::from_u128(0xf618c514_dfb8_11d1_a2cf_00805fc79235);
pub const COMAdminCatalogCollection: windows_core::GUID = windows_core::GUID::from_u128(0xf618c516_dfb8_11d1_a2cf_00805fc79235);
pub const COMAdminCatalogObject: windows_core::GUID = windows_core::GUID::from_u128(0xf618c515_dfb8_11d1_a2cf_00805fc79235);
pub const COMEvents: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0ab_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COMSVCSEVENTINFO {
    pub cbSize: u32,
    pub dwPid: u32,
    pub lTime: i64,
    pub lMicroTime: i32,
    pub perfCount: i64,
    pub guidApp: windows_core::GUID,
    pub sMachineName: windows_core::PWSTR,
}
impl windows_core::TypeKind for COMSVCSEVENTINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for COMSVCSEVENTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRMClerk: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0bd_7f19_11d2_978e_0000f8757e2a);
pub const CRMRecoveryClerk: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0be_7f19_11d2_978e_0000f8757e2a);
pub const CServiceConfig: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0c8_7f19_11d2_978e_0000f8757e2a);
pub const ClrAssemblyLocator: windows_core::GUID = windows_core::GUID::from_u128(0x458aa3b5_265a_4b75_bc05_9bea4630cf18);
pub const CoMTSLocator: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0ac_7f19_11d2_978e_0000f8757e2a);
pub const ComServiceEvents: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0c3_7f19_11d2_978e_0000f8757e2a);
pub const ComSystemAppEventData: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0c6_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComponentHangMonitorInfo {
    pub IsMonitored: super::super::Foundation::BOOL,
    pub TerminateOnHang: super::super::Foundation::BOOL,
    pub AvgCallThresholdInMs: u32,
}
impl windows_core::TypeKind for ComponentHangMonitorInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for ComponentHangMonitorInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComponentStatistics {
    pub NumInstances: u32,
    pub NumBoundReferences: u32,
    pub NumPooledObjects: u32,
    pub NumObjectsInCall: u32,
    pub AvgResponseTimeInMs: u32,
    pub NumCallsCompletedRecent: u32,
    pub NumCallsFailedRecent: u32,
    pub NumCallsCompletedTotal: u32,
    pub NumCallsFailedTotal: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: u32,
    pub Reserved4: u32,
}
impl windows_core::TypeKind for ComponentStatistics {
    type TypeKind = windows_core::CopyType;
}
impl Default for ComponentStatistics {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComponentSummary {
    pub ApplicationInstanceId: windows_core::GUID,
    pub PartitionId: windows_core::GUID,
    pub ApplicationId: windows_core::GUID,
    pub Clsid: windows_core::GUID,
    pub ClassName: windows_core::PWSTR,
    pub ApplicationName: windows_core::PWSTR,
}
impl windows_core::TypeKind for ComponentSummary {
    type TypeKind = windows_core::CopyType;
}
impl Default for ComponentSummary {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CrmLogRecordRead {
    pub dwCrmFlags: u32,
    pub dwSequenceNumber: u32,
    pub blobUserData: super::Com::BLOB,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for CrmLogRecordRead {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for CrmLogRecordRead {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DispenserManager: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0c0_7f19_11d2_978e_0000f8757e2a);
pub const Dummy30040732: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0a9_7f19_11d2_978e_0000f8757e2a);
pub const EventServer: windows_core::GUID = windows_core::GUID::from_u128(0xecabafbc_7f19_11d2_978e_0000f8757e2a);
pub const GetSecurityCallContextAppObject: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0a8_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HANG_INFO {
    pub fAppHangMonitorEnabled: super::super::Foundation::BOOL,
    pub fTerminateOnHang: super::super::Foundation::BOOL,
    pub DumpType: DUMPTYPE,
    pub dwHangTimeout: u32,
    pub dwDumpCount: u32,
    pub dwInfoMsgCount: u32,
}
impl windows_core::TypeKind for HANG_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HANG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LBEvents: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0c1_7f19_11d2_978e_0000f8757e2a);
pub const MessageMover: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0bf_7f19_11d2_978e_0000f8757e2a);
pub const MtsGrp: windows_core::GUID = windows_core::GUID::from_u128(0x4b2e958d_0393_11d1_b1ab_00aa00ba3258);
pub const PoolMgr: windows_core::GUID = windows_core::GUID::from_u128(0xecabafb5_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RECYCLE_INFO {
    pub guidCombaseProcessIdentifier: windows_core::GUID,
    pub ProcessStartTime: i64,
    pub dwRecycleLifetimeLimit: u32,
    pub dwRecycleMemoryLimit: u32,
    pub dwRecycleExpirationTimeout: u32,
}
impl windows_core::TypeKind for RECYCLE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for RECYCLE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SecurityCallContext: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0a7_7f19_11d2_978e_0000f8757e2a);
pub const SecurityCallers: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0a6_7f19_11d2_978e_0000f8757e2a);
pub const SecurityIdentity: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0a5_7f19_11d2_978e_0000f8757e2a);
pub const ServicePool: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0c9_7f19_11d2_978e_0000f8757e2a);
pub const ServicePoolConfig: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0ca_7f19_11d2_978e_0000f8757e2a);
pub const SharedProperty: windows_core::GUID = windows_core::GUID::from_u128(0x2a005c05_a5de_11cf_9e66_00aa00a3f464);
pub const SharedPropertyGroup: windows_core::GUID = windows_core::GUID::from_u128(0x2a005c0b_a5de_11cf_9e66_00aa00a3f464);
pub const SharedPropertyGroupManager: windows_core::GUID = windows_core::GUID::from_u128(0x2a005c11_a5de_11cf_9e66_00aa00a3f464);
pub const TrackerServer: windows_core::GUID = windows_core::GUID::from_u128(0xecabafb9_7f19_11d2_978e_0000f8757e2a);
pub const TransactionContext: windows_core::GUID = windows_core::GUID::from_u128(0x7999fc25_d3c6_11cf_acab_00a024a55aef);
pub const TransactionContextEx: windows_core::GUID = windows_core::GUID::from_u128(0x5cb66670_d3d4_11cf_acab_00a024a55aef);
#[cfg(feature = "implement")]
core::include!("impl.rs");
