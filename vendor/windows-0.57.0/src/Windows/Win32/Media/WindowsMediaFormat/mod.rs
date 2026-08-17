#[inline]
pub unsafe fn WMCreateBackupRestorer<P0>(pcallback: P0) -> windows_core::Result<IWMLicenseBackup>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("wmvcore.dll" "system" fn WMCreateBackupRestorer(pcallback : * mut core::ffi::c_void, ppbackup : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WMCreateBackupRestorer(pcallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WMCreateEditor() -> windows_core::Result<IWMMetadataEditor> {
    windows_targets::link!("wmvcore.dll" "system" fn WMCreateEditor(ppeditor : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WMCreateEditor(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WMCreateIndexer() -> windows_core::Result<IWMIndexer> {
    windows_targets::link!("wmvcore.dll" "system" fn WMCreateIndexer(ppindexer : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WMCreateIndexer(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WMCreateProfileManager() -> windows_core::Result<IWMProfileManager> {
    windows_targets::link!("wmvcore.dll" "system" fn WMCreateProfileManager(ppprofilemanager : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WMCreateProfileManager(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WMCreateReader<P0>(punkcert: P0, dwrights: u32) -> windows_core::Result<IWMReader>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("wmvcore.dll" "system" fn WMCreateReader(punkcert : * mut core::ffi::c_void, dwrights : u32, ppreader : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WMCreateReader(punkcert.param().abi(), dwrights, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WMCreateSyncReader<P0>(punkcert: P0, dwrights: u32) -> windows_core::Result<IWMSyncReader>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("wmvcore.dll" "system" fn WMCreateSyncReader(punkcert : * mut core::ffi::c_void, dwrights : u32, ppsyncreader : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WMCreateSyncReader(punkcert.param().abi(), dwrights, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WMCreateWriter<P0>(punkcert: P0) -> windows_core::Result<IWMWriter>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("wmvcore.dll" "system" fn WMCreateWriter(punkcert : * mut core::ffi::c_void, ppwriter : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WMCreateWriter(punkcert.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WMCreateWriterFileSink() -> windows_core::Result<IWMWriterFileSink> {
    windows_targets::link!("wmvcore.dll" "system" fn WMCreateWriterFileSink(ppsink : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WMCreateWriterFileSink(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WMCreateWriterNetworkSink() -> windows_core::Result<IWMWriterNetworkSink> {
    windows_targets::link!("wmvcore.dll" "system" fn WMCreateWriterNetworkSink(ppsink : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WMCreateWriterNetworkSink(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WMCreateWriterPushSink() -> windows_core::Result<IWMWriterPushSink> {
    windows_targets::link!("wmvcore.dll" "system" fn WMCreateWriterPushSink(ppsink : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WMCreateWriterPushSink(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WMIsContentProtected<P0>(pwszfilename: P0, pfisprotected: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wmvcore.dll" "system" fn WMIsContentProtected(pwszfilename : windows_core::PCWSTR, pfisprotected : *mut super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    WMIsContentProtected(pwszfilename.param().abi(), pfisprotected).ok()
}
windows_core::imp::define_interface!(INSNetSourceCreator, INSNetSourceCreator_Vtbl, 0x0c0e4080_9081_11d2_beec_0060082f2054);
impl core::ops::Deref for INSNetSourceCreator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INSNetSourceCreator, windows_core::IUnknown);
impl INSNetSourceCreator {
    pub unsafe fn Initialize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateNetSource<P0, P1, P2, P3>(&self, pszstreamname: P0, pmonitor: P1, pdata: *const u8, pusercontext: P2, pcallback: P3, qwcontext: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CreateNetSource)(windows_core::Interface::as_raw(self), pszstreamname.param().abi(), pmonitor.param().abi(), pdata, pusercontext.param().abi(), pcallback.param().abi(), qwcontext).ok()
    }
    pub unsafe fn GetNetSourceProperties<P0>(&self, pszstreamname: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetSourceProperties)(windows_core::Interface::as_raw(self), pszstreamname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNetSourceSharedNamespace(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetSourceSharedNamespace)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNetSourceAdminInterface<P0>(&self, pszstreamname: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetSourceAdminInterface)(windows_core::Interface::as_raw(self), pszstreamname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNumProtocolsSupported(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNumProtocolsSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: windows_core::PWSTR, pcchprotocolname: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProtocolName)(windows_core::Interface::as_raw(self), dwprotocolnum, core::mem::transmute(pwszprotocolname), pcchprotocolname).ok()
    }
    pub unsafe fn Shutdown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct INSNetSourceCreator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNetSource: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *const u8, *mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub GetNetSourceProperties: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNetSourceSharedNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNetSourceAdminInterface: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetNumProtocolsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetProtocolName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u16) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INSSBuffer, INSSBuffer_Vtbl, 0xe1cd3524_03d7_11d2_9eed_006097d2d7cf);
impl core::ops::Deref for INSSBuffer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INSSBuffer, windows_core::IUnknown);
impl INSSBuffer {
    pub unsafe fn GetLength(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLength(&self, dwlength: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLength)(windows_core::Interface::as_raw(self), dwlength).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetBuffer(&self) -> windows_core::Result<*mut u8> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBufferAndLength)(windows_core::Interface::as_raw(self), ppdwbuffer, pdwlength).ok()
    }
}
#[repr(C)]
pub struct INSSBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8) -> windows_core::HRESULT,
    pub GetBufferAndLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INSSBuffer2, INSSBuffer2_Vtbl, 0x4f528693_1035_43fe_b428_757561ad3a68);
impl core::ops::Deref for INSSBuffer2 {
    type Target = INSSBuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INSSBuffer2, windows_core::IUnknown, INSSBuffer);
impl INSSBuffer2 {
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> windows_core::Result<u8> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSampleProperties)(windows_core::Interface::as_raw(self), cbproperties, &mut result__).map(|| result__)
    }
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSampleProperties)(windows_core::Interface::as_raw(self), cbproperties, pbproperties).ok()
    }
}
#[repr(C)]
pub struct INSSBuffer2_Vtbl {
    pub base__: INSSBuffer_Vtbl,
    pub GetSampleProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
    pub SetSampleProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INSSBuffer3, INSSBuffer3_Vtbl, 0xc87ceaaf_75be_4bc4_84eb_ac2798507672);
impl core::ops::Deref for INSSBuffer3 {
    type Target = INSSBuffer2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INSSBuffer3, windows_core::IUnknown, INSSBuffer, INSSBuffer2);
impl INSSBuffer3 {
    pub unsafe fn SetProperty(&self, guidbufferproperty: windows_core::GUID, pvbufferproperty: *const core::ffi::c_void, dwbufferpropertysize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), core::mem::transmute(guidbufferproperty), pvbufferproperty, dwbufferpropertysize).ok()
    }
    pub unsafe fn GetProperty(&self, guidbufferproperty: windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), core::mem::transmute(guidbufferproperty), pvbufferproperty, pdwbufferpropertysize).ok()
    }
}
#[repr(C)]
pub struct INSSBuffer3_Vtbl {
    pub base__: INSSBuffer2_Vtbl,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INSSBuffer4, INSSBuffer4_Vtbl, 0xb6b8fd5a_32e2_49d4_a910_c26cc85465ed);
impl core::ops::Deref for INSSBuffer4 {
    type Target = INSSBuffer3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INSSBuffer4, windows_core::IUnknown, INSSBuffer, INSSBuffer2, INSSBuffer3);
impl INSSBuffer4 {
    pub unsafe fn GetPropertyCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPropertyByIndex(&self, dwbufferpropertyindex: u32, pguidbufferproperty: *mut windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropertyByIndex)(windows_core::Interface::as_raw(self), dwbufferpropertyindex, pguidbufferproperty, pvbufferproperty, pdwbufferpropertysize).ok()
    }
}
#[repr(C)]
pub struct INSSBuffer4_Vtbl {
    pub base__: INSSBuffer3_Vtbl,
    pub GetPropertyCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPropertyByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMAddressAccess, IWMAddressAccess_Vtbl, 0xbb3c6389_1633_4e92_af14_9f3173ba39d0);
impl core::ops::Deref for IWMAddressAccess {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMAddressAccess, windows_core::IUnknown);
impl IWMAddressAccess {
    pub unsafe fn GetAccessEntryCount(&self, aetype: WM_AETYPE) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAccessEntryCount)(windows_core::Interface::as_raw(self), aetype, &mut result__).map(|| result__)
    }
    pub unsafe fn GetAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> windows_core::Result<WM_ADDRESS_ACCESSENTRY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAccessEntry)(windows_core::Interface::as_raw(self), aetype, dwentrynum, &mut result__).map(|| result__)
    }
    pub unsafe fn AddAccessEntry(&self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddAccessEntry)(windows_core::Interface::as_raw(self), aetype, paddraccessentry).ok()
    }
    pub unsafe fn RemoveAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAccessEntry)(windows_core::Interface::as_raw(self), aetype, dwentrynum).ok()
    }
}
#[repr(C)]
pub struct IWMAddressAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAccessEntryCount: unsafe extern "system" fn(*mut core::ffi::c_void, WM_AETYPE, *mut u32) -> windows_core::HRESULT,
    pub GetAccessEntry: unsafe extern "system" fn(*mut core::ffi::c_void, WM_AETYPE, u32, *mut WM_ADDRESS_ACCESSENTRY) -> windows_core::HRESULT,
    pub AddAccessEntry: unsafe extern "system" fn(*mut core::ffi::c_void, WM_AETYPE, *const WM_ADDRESS_ACCESSENTRY) -> windows_core::HRESULT,
    pub RemoveAccessEntry: unsafe extern "system" fn(*mut core::ffi::c_void, WM_AETYPE, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMAddressAccess2, IWMAddressAccess2_Vtbl, 0x65a83fc2_3e98_4d4d_81b5_2a742886b33d);
impl core::ops::Deref for IWMAddressAccess2 {
    type Target = IWMAddressAccess;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMAddressAccess2, windows_core::IUnknown, IWMAddressAccess);
impl IWMAddressAccess2 {
    pub unsafe fn GetAccessEntryEx(&self, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut windows_core::BSTR, pbstrmask: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAccessEntryEx)(windows_core::Interface::as_raw(self), aetype, dwentrynum, core::mem::transmute(pbstraddress), core::mem::transmute(pbstrmask)).ok()
    }
    pub unsafe fn AddAccessEntryEx<P0, P1>(&self, aetype: WM_AETYPE, bstraddress: P0, bstrmask: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddAccessEntryEx)(windows_core::Interface::as_raw(self), aetype, bstraddress.param().abi(), bstrmask.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWMAddressAccess2_Vtbl {
    pub base__: IWMAddressAccess_Vtbl,
    pub GetAccessEntryEx: unsafe extern "system" fn(*mut core::ffi::c_void, WM_AETYPE, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AddAccessEntryEx: unsafe extern "system" fn(*mut core::ffi::c_void, WM_AETYPE, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMAuthorizer, IWMAuthorizer_Vtbl, 0xd9b67d36_a9ad_4eb4_baef_db284ef5504c);
impl core::ops::Deref for IWMAuthorizer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMAuthorizer, windows_core::IUnknown);
impl IWMAuthorizer {
    pub unsafe fn GetCertCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCertCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCert(&self, dwindex: u32) -> windows_core::Result<*mut u8> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCert)(windows_core::Interface::as_raw(self), dwindex, &mut result__).map(|| result__)
    }
    pub unsafe fn GetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8) -> windows_core::Result<*mut u8> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSharedData)(windows_core::Interface::as_raw(self), dwcertindex, pbshareddata, pbcert, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWMAuthorizer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCertCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCert: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u8) -> windows_core::HRESULT,
    pub GetSharedData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *const u8, *mut *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMBackupRestoreProps, IWMBackupRestoreProps_Vtbl, 0x3c8e0da6_996f_4ff3_a1af_4838f9377e2e);
impl core::ops::Deref for IWMBackupRestoreProps {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMBackupRestoreProps, windows_core::IUnknown);
impl IWMBackupRestoreProps {
    pub unsafe fn GetPropCount(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPropByIndex(&self, windex: u16, pwszname: windows_core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropByIndex)(windows_core::Interface::as_raw(self), windex, core::mem::transmute(pwszname), pcchnamelen, ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn GetPropByName<P0>(&self, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetPropByName)(windows_core::Interface::as_raw(self), pszname.param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetProp<P0>(&self, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetProp)(windows_core::Interface::as_raw(self), pszname.param().abi(), r#type, core::mem::transmute(pvalue.as_ptr()), pvalue.len().try_into().unwrap()).ok()
    }
    pub unsafe fn RemoveProp<P0>(&self, pcwszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveProp)(windows_core::Interface::as_raw(self), pcwszname.param().abi()).ok()
    }
    pub unsafe fn RemoveAllProps(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAllProps)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMBackupRestoreProps_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPropCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetPropByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u16, windows_core::PWSTR, *mut u16, *mut WMT_ATTR_DATATYPE, *mut u8, *mut u16) -> windows_core::HRESULT,
    pub GetPropByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut u8, *mut u16) -> windows_core::HRESULT,
    pub SetProp: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const u8, u16) -> windows_core::HRESULT,
    pub RemoveProp: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RemoveAllProps: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMBandwidthSharing, IWMBandwidthSharing_Vtbl, 0xad694af1_f8d9_42f8_bc47_70311b0c4f9e);
impl core::ops::Deref for IWMBandwidthSharing {
    type Target = IWMStreamList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMBandwidthSharing, windows_core::IUnknown, IWMStreamList);
impl IWMBandwidthSharing {
    pub unsafe fn GetType(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetType(&self, guidtype: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), guidtype).ok()
    }
    pub unsafe fn GetBandwidth(&self, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBandwidth)(windows_core::Interface::as_raw(self), pdwbitrate, pmsbufferwindow).ok()
    }
    pub unsafe fn SetBandwidth(&self, dwbitrate: u32, msbufferwindow: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBandwidth)(windows_core::Interface::as_raw(self), dwbitrate, msbufferwindow).ok()
    }
}
#[repr(C)]
pub struct IWMBandwidthSharing_Vtbl {
    pub base__: IWMStreamList_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetBandwidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetBandwidth: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMClientConnections, IWMClientConnections_Vtbl, 0x73c66010_a299_41df_b1f0_ccf03b09c1c6);
impl core::ops::Deref for IWMClientConnections {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMClientConnections, windows_core::IUnknown);
impl IWMClientConnections {
    pub unsafe fn GetClientCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClientCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetClientProperties(&self, dwclientnum: u32) -> windows_core::Result<WM_CLIENT_PROPERTIES> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClientProperties)(windows_core::Interface::as_raw(self), dwclientnum, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWMClientConnections_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClientCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetClientProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut WM_CLIENT_PROPERTIES) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMClientConnections2, IWMClientConnections2_Vtbl, 0x4091571e_4701_4593_bb3d_d5f5f0c74246);
impl core::ops::Deref for IWMClientConnections2 {
    type Target = IWMClientConnections;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMClientConnections2, windows_core::IUnknown, IWMClientConnections);
impl IWMClientConnections2 {
    pub unsafe fn GetClientInfo(&self, dwclientnum: u32, pwsznetworkaddress: windows_core::PWSTR, pcchnetworkaddress: *mut u32, pwszport: windows_core::PWSTR, pcchport: *mut u32, pwszdnsname: windows_core::PWSTR, pcchdnsname: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClientInfo)(windows_core::Interface::as_raw(self), dwclientnum, core::mem::transmute(pwsznetworkaddress), pcchnetworkaddress, core::mem::transmute(pwszport), pcchport, core::mem::transmute(pwszdnsname), pcchdnsname).ok()
    }
}
#[repr(C)]
pub struct IWMClientConnections2_Vtbl {
    pub base__: IWMClientConnections_Vtbl,
    pub GetClientInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32, windows_core::PWSTR, *mut u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMCodecInfo, IWMCodecInfo_Vtbl, 0xa970f41e_34de_4a98_b3ba_e4b3ca7528f0);
impl core::ops::Deref for IWMCodecInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMCodecInfo, windows_core::IUnknown);
impl IWMCodecInfo {
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const windows_core::GUID) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCodecInfoCount)(windows_core::Interface::as_raw(self), guidtype, &mut result__).map(|| result__)
    }
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const windows_core::GUID, dwcodecindex: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCodecFormatCount)(windows_core::Interface::as_raw(self), guidtype, dwcodecindex, &mut result__).map(|| result__)
    }
    pub unsafe fn GetCodecFormat(&self, guidtype: *const windows_core::GUID, dwcodecindex: u32, dwformatindex: u32) -> windows_core::Result<IWMStreamConfig> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCodecFormat)(windows_core::Interface::as_raw(self), guidtype, dwcodecindex, dwformatindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWMCodecInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCodecInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub GetCodecFormatCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut u32) -> windows_core::HRESULT,
    pub GetCodecFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMCodecInfo2, IWMCodecInfo2_Vtbl, 0xaa65e273_b686_4056_91ec_dd768d4df710);
impl core::ops::Deref for IWMCodecInfo2 {
    type Target = IWMCodecInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMCodecInfo2, windows_core::IUnknown, IWMCodecInfo);
impl IWMCodecInfo2 {
    pub unsafe fn GetCodecName(&self, guidtype: *const windows_core::GUID, dwcodecindex: u32, wszname: windows_core::PWSTR, pcchname: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCodecName)(windows_core::Interface::as_raw(self), guidtype, dwcodecindex, core::mem::transmute(wszname), pcchname).ok()
    }
    pub unsafe fn GetCodecFormatDesc(&self, guidtype: *const windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut Option<IWMStreamConfig>, wszdesc: windows_core::PWSTR, pcchdesc: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCodecFormatDesc)(windows_core::Interface::as_raw(self), guidtype, dwcodecindex, dwformatindex, core::mem::transmute(ppistreamconfig), core::mem::transmute(wszdesc), pcchdesc).ok()
    }
}
#[repr(C)]
pub struct IWMCodecInfo2_Vtbl {
    pub base__: IWMCodecInfo_Vtbl,
    pub GetCodecName: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetCodecFormatDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32, *mut *mut core::ffi::c_void, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMCodecInfo3, IWMCodecInfo3_Vtbl, 0x7e51f487_4d93_4f98_8ab4_27d0565adc51);
impl core::ops::Deref for IWMCodecInfo3 {
    type Target = IWMCodecInfo2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMCodecInfo3, windows_core::IUnknown, IWMCodecInfo, IWMCodecInfo2);
impl IWMCodecInfo3 {
    pub unsafe fn GetCodecFormatProp<P0>(&self, guidtype: *const windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetCodecFormatProp)(windows_core::Interface::as_raw(self), guidtype, dwcodecindex, dwformatindex, pszname.param().abi(), ptype, pvalue, pdwsize).ok()
    }
    pub unsafe fn GetCodecProp<P0>(&self, guidtype: *const windows_core::GUID, dwcodecindex: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetCodecProp)(windows_core::Interface::as_raw(self), guidtype, dwcodecindex, pszname.param().abi(), ptype, pvalue, pdwsize).ok()
    }
    pub unsafe fn SetCodecEnumerationSetting<P0>(&self, guidtype: *const windows_core::GUID, dwcodecindex: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetCodecEnumerationSetting)(windows_core::Interface::as_raw(self), guidtype, dwcodecindex, pszname.param().abi(), r#type, core::mem::transmute(pvalue.as_ptr()), pvalue.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetCodecEnumerationSetting<P0>(&self, guidtype: *const windows_core::GUID, dwcodecindex: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetCodecEnumerationSetting)(windows_core::Interface::as_raw(self), guidtype, dwcodecindex, pszname.param().abi(), ptype, pvalue, pdwsize).ok()
    }
}
#[repr(C)]
pub struct IWMCodecInfo3_Vtbl {
    pub base__: IWMCodecInfo2_Vtbl,
    pub GetCodecFormatProp: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetCodecProp: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SetCodecEnumerationSetting: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const u8, u32) -> windows_core::HRESULT,
    pub GetCodecEnumerationSetting: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMCredentialCallback, IWMCredentialCallback_Vtbl, 0x342e0eb7_e651_450c_975b_2ace2c90c48e);
impl core::ops::Deref for IWMCredentialCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMCredentialCallback, windows_core::IUnknown);
impl IWMCredentialCallback {
    pub unsafe fn AcquireCredentials<P0, P1>(&self, pwszrealm: P0, pwszsite: P1, pwszuser: &mut [u16], pwszpassword: &mut [u16], hrstatus: windows_core::HRESULT, pdwflags: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AcquireCredentials)(windows_core::Interface::as_raw(self), pwszrealm.param().abi(), pwszsite.param().abi(), core::mem::transmute(pwszuser.as_ptr()), pwszuser.len().try_into().unwrap(), core::mem::transmute(pwszpassword.as_ptr()), pwszpassword.len().try_into().unwrap(), hrstatus, pdwflags).ok()
    }
}
#[repr(C)]
pub struct IWMCredentialCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AcquireCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PWSTR, u32, windows_core::PWSTR, u32, windows_core::HRESULT, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDRMEditor, IWMDRMEditor_Vtbl, 0xff130ebc_a6c3_42a6_b401_c3382c3e08b3);
impl core::ops::Deref for IWMDRMEditor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDRMEditor, windows_core::IUnknown);
impl IWMDRMEditor {
    pub unsafe fn GetDRMProperty<P0>(&self, pwstrname: P0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetDRMProperty)(windows_core::Interface::as_raw(self), pwstrname.param().abi(), pdwtype, pvalue, pcblength).ok()
    }
}
#[repr(C)]
pub struct IWMDRMEditor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDRMProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut u8, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDRMMessageParser, IWMDRMMessageParser_Vtbl, 0xa73a0072_25a0_4c99_b4a5_ede8101a6c39);
impl core::ops::Deref for IWMDRMMessageParser {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDRMMessageParser, windows_core::IUnknown);
impl IWMDRMMessageParser {
    pub unsafe fn ParseRegistrationReqMsg(&self, pbregistrationreqmsg: &[u8], ppdevicecert: *mut Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ParseRegistrationReqMsg)(windows_core::Interface::as_raw(self), core::mem::transmute(pbregistrationreqmsg.as_ptr()), pbregistrationreqmsg.len().try_into().unwrap(), core::mem::transmute(ppdevicecert), pdeviceserialnumber).ok()
    }
    pub unsafe fn ParseLicenseRequestMsg(&self, pblicenserequestmsg: &[u8], ppdevicecert: *mut Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ParseLicenseRequestMsg)(windows_core::Interface::as_raw(self), core::mem::transmute(pblicenserequestmsg.as_ptr()), pblicenserequestmsg.len().try_into().unwrap(), core::mem::transmute(ppdevicecert), pdeviceserialnumber, core::mem::transmute(pbstraction)).ok()
    }
}
#[repr(C)]
pub struct IWMDRMMessageParser_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ParseRegistrationReqMsg: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut *mut core::ffi::c_void, *mut DRM_VAL16) -> windows_core::HRESULT,
    pub ParseLicenseRequestMsg: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut *mut core::ffi::c_void, *mut DRM_VAL16, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDRMReader, IWMDRMReader_Vtbl, 0xd2827540_3ee7_432c_b14c_dc17f085d3b3);
impl core::ops::Deref for IWMDRMReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDRMReader, windows_core::IUnknown);
impl IWMDRMReader {
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AcquireLicense)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CancelLicenseAcquisition(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelLicenseAcquisition)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Individualize(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Individualize)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CancelIndividualization(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelIndividualization)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MonitorLicenseAcquisition(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MonitorLicenseAcquisition)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelMonitorLicenseAcquisition)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetDRMProperty<P0>(&self, pwstrname: P0, dwtype: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDRMProperty)(windows_core::Interface::as_raw(self), pwstrname.param().abi(), dwtype, core::mem::transmute(pvalue.as_ptr()), pvalue.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDRMProperty<P0>(&self, pwstrname: P0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetDRMProperty)(windows_core::Interface::as_raw(self), pwstrname.param().abi(), pdwtype, pvalue, pcblength).ok()
    }
}
#[repr(C)]
pub struct IWMDRMReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AcquireLicense: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CancelLicenseAcquisition: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Individualize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CancelIndividualization: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MonitorLicenseAcquisition: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelMonitorLicenseAcquisition: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDRMProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const u8, u16) -> windows_core::HRESULT,
    pub GetDRMProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut u8, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDRMReader2, IWMDRMReader2_Vtbl, 0xbefe7a75_9f1d_4075_b9d9_a3c37bda49a0);
impl core::ops::Deref for IWMDRMReader2 {
    type Target = IWMDRMReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDRMReader2, windows_core::IUnknown, IWMDRMReader);
impl IWMDRMReader2 {
    pub unsafe fn SetEvaluateOutputLevelLicenses<P0>(&self, fevaluate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEvaluateOutputLevelLicenses)(windows_core::Interface::as_raw(self), fevaluate.param().abi()).ok()
    }
    pub unsafe fn GetPlayOutputLevels(&self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPlayOutputLevels)(windows_core::Interface::as_raw(self), pplayopl, pcblength, pdwminappcompliancelevel).ok()
    }
    pub unsafe fn GetCopyOutputLevels(&self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCopyOutputLevels)(windows_core::Interface::as_raw(self), pcopyopl, pcblength, pdwminappcompliancelevel).ok()
    }
    pub unsafe fn TryNextLicense(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TryNextLicense)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMDRMReader2_Vtbl {
    pub base__: IWMDRMReader_Vtbl,
    pub SetEvaluateOutputLevelLicenses: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetPlayOutputLevels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DRM_PLAY_OPL, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetCopyOutputLevels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DRM_COPY_OPL, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub TryNextLicense: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDRMReader3, IWMDRMReader3_Vtbl, 0xe08672de_f1e7_4ff4_a0a3_fc4b08e4caf8);
impl core::ops::Deref for IWMDRMReader3 {
    type Target = IWMDRMReader2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDRMReader3, windows_core::IUnknown, IWMDRMReader, IWMDRMReader2);
impl IWMDRMReader3 {
    pub unsafe fn GetInclusionList(&self, ppguids: *mut *mut windows_core::GUID, pcguids: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInclusionList)(windows_core::Interface::as_raw(self), ppguids, pcguids).ok()
    }
}
#[repr(C)]
pub struct IWMDRMReader3_Vtbl {
    pub base__: IWMDRMReader2_Vtbl,
    pub GetInclusionList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDRMTranscryptionManager, IWMDRMTranscryptionManager_Vtbl, 0xb1a887b2_a4f0_407a_b02e_efbd23bbecdf);
impl core::ops::Deref for IWMDRMTranscryptionManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDRMTranscryptionManager, windows_core::IUnknown);
impl IWMDRMTranscryptionManager {
    pub unsafe fn CreateTranscryptor(&self) -> windows_core::Result<IWMDRMTranscryptor> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTranscryptor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWMDRMTranscryptionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTranscryptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDRMTranscryptor, IWMDRMTranscryptor_Vtbl, 0x69059850_6e6f_4bb2_806f_71863ddfc471);
impl core::ops::Deref for IWMDRMTranscryptor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDRMTranscryptor, windows_core::IUnknown);
impl IWMDRMTranscryptor {
    pub unsafe fn Initialize<P0, P1>(&self, bstrfilename: P0, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut Option<INSSBuffer>, pcallback: P1, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IWMStatusCallback>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), bstrfilename.param().abi(), pblicenserequestmsg, cblicenserequestmsg, core::mem::transmute(pplicenseresponsemsg), pcallback.param().abi(), pvcontext).ok()
    }
    pub unsafe fn Seek(&self, hnstime: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), hnstime).ok()
    }
    pub unsafe fn Read(&self, pbdata: *const u8, pcbdata: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pbdata, pcbdata).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMDRMTranscryptor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut u8, u32, *mut *mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *const u32) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDRMTranscryptor2, IWMDRMTranscryptor2_Vtbl, 0xe0da439f_d331_496a_bece_18e5bac5dd23);
impl core::ops::Deref for IWMDRMTranscryptor2 {
    type Target = IWMDRMTranscryptor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDRMTranscryptor2, windows_core::IUnknown, IWMDRMTranscryptor);
impl IWMDRMTranscryptor2 {
    pub unsafe fn SeekEx<P0>(&self, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SeekEx)(windows_core::Interface::as_raw(self), cnsstarttime, cnsduration, flrate, fincludefileheader.param().abi()).ok()
    }
    pub unsafe fn ZeroAdjustTimestamps<P0>(&self, fenable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).ZeroAdjustTimestamps)(windows_core::Interface::as_raw(self), fenable.param().abi()).ok()
    }
    pub unsafe fn GetSeekStartTime(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSeekStartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDuration(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWMDRMTranscryptor2_Vtbl {
    pub base__: IWMDRMTranscryptor_Vtbl,
    pub SeekEx: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, f32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ZeroAdjustTimestamps: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetSeekStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDRMWriter, IWMDRMWriter_Vtbl, 0xd6ea5dd0_12a0_43f4_90ab_a3fd451e6a07);
impl core::ops::Deref for IWMDRMWriter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDRMWriter, windows_core::IUnknown);
impl IWMDRMWriter {
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: windows_core::PWSTR, pcwchlength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GenerateKeySeed)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszkeyseed), pcwchlength).ok()
    }
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: windows_core::PWSTR, pcwchlength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GenerateKeyID)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszkeyid), pcwchlength).ok()
    }
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: windows_core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: windows_core::PWSTR, pcwchpubkeylength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GenerateSigningKeyPair)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszprivkey), pcwchprivkeylength, core::mem::transmute(pwszpubkey), pcwchpubkeylength).ok()
    }
    pub unsafe fn SetDRMAttribute<P0>(&self, wstreamnum: u16, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDRMAttribute)(windows_core::Interface::as_raw(self), wstreamnum, pszname.param().abi(), r#type, core::mem::transmute(pvalue.as_ptr()), pvalue.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct IWMDRMWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GenerateKeySeed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GenerateKeyID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GenerateSigningKeyPair: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub SetDRMAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, u16, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const u8, u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDRMWriter2, IWMDRMWriter2_Vtbl, 0x38ee7a94_40e2_4e10_aa3f_33fd3210ed5b);
impl core::ops::Deref for IWMDRMWriter2 {
    type Target = IWMDRMWriter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDRMWriter2, windows_core::IUnknown, IWMDRMWriter);
impl IWMDRMWriter2 {
    pub unsafe fn SetWMDRMNetEncryption<P0>(&self, fsamplesencrypted: P0, pbkeyid: *const u8, cbkeyid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetWMDRMNetEncryption)(windows_core::Interface::as_raw(self), fsamplesencrypted.param().abi(), pbkeyid, cbkeyid).ok()
    }
}
#[repr(C)]
pub struct IWMDRMWriter2_Vtbl {
    pub base__: IWMDRMWriter_Vtbl,
    pub SetWMDRMNetEncryption: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *const u8, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDRMWriter3, IWMDRMWriter3_Vtbl, 0xa7184082_a4aa_4dde_ac9c_e75dbd1117ce);
impl core::ops::Deref for IWMDRMWriter3 {
    type Target = IWMDRMWriter2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDRMWriter3, windows_core::IUnknown, IWMDRMWriter, IWMDRMWriter2);
impl IWMDRMWriter3 {
    pub unsafe fn SetProtectStreamSamples(&self, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProtectStreamSamples)(windows_core::Interface::as_raw(self), pimportinitstruct).ok()
    }
}
#[repr(C)]
pub struct IWMDRMWriter3_Vtbl {
    pub base__: IWMDRMWriter2_Vtbl,
    pub SetProtectStreamSamples: unsafe extern "system" fn(*mut core::ffi::c_void, *const WMDRM_IMPORT_INIT_STRUCT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDeviceRegistration, IWMDeviceRegistration_Vtbl, 0xf6211f03_8d21_4e94_93e6_8510805f2d99);
impl core::ops::Deref for IWMDeviceRegistration {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDeviceRegistration, windows_core::IUnknown);
impl IWMDeviceRegistration {
    pub unsafe fn RegisterDevice(&self, dwregistertype: u32, pbcertificate: &[u8], serialnumber: DRM_VAL16) -> windows_core::Result<IWMRegisteredDevice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterDevice)(windows_core::Interface::as_raw(self), dwregistertype, core::mem::transmute(pbcertificate.as_ptr()), pbcertificate.len().try_into().unwrap(), core::mem::transmute(serialnumber), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UnregisterDevice(&self, dwregistertype: u32, pbcertificate: &[u8], serialnumber: DRM_VAL16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterDevice)(windows_core::Interface::as_raw(self), dwregistertype, core::mem::transmute(pbcertificate.as_ptr()), pbcertificate.len().try_into().unwrap(), core::mem::transmute(serialnumber)).ok()
    }
    pub unsafe fn GetRegistrationStats(&self, dwregistertype: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRegistrationStats)(windows_core::Interface::as_raw(self), dwregistertype, &mut result__).map(|| result__)
    }
    pub unsafe fn GetFirstRegisteredDevice(&self, dwregistertype: u32) -> windows_core::Result<IWMRegisteredDevice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFirstRegisteredDevice)(windows_core::Interface::as_raw(self), dwregistertype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNextRegisteredDevice(&self) -> windows_core::Result<IWMRegisteredDevice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNextRegisteredDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRegisteredDeviceByID(&self, dwregistertype: u32, pbcertificate: &[u8], serialnumber: DRM_VAL16) -> windows_core::Result<IWMRegisteredDevice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRegisteredDeviceByID)(windows_core::Interface::as_raw(self), dwregistertype, core::mem::transmute(pbcertificate.as_ptr()), pbcertificate.len().try_into().unwrap(), core::mem::transmute(serialnumber), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWMDeviceRegistration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterDevice: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, DRM_VAL16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterDevice: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, DRM_VAL16) -> windows_core::HRESULT,
    pub GetRegistrationStats: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetFirstRegisteredDevice: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNextRegisteredDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRegisteredDeviceByID: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, DRM_VAL16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMGetSecureChannel, IWMGetSecureChannel_Vtbl, 0x94bc0598_c3d2_11d3_bedf_00c04f612986);
impl core::ops::Deref for IWMGetSecureChannel {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMGetSecureChannel, windows_core::IUnknown);
impl IWMGetSecureChannel {
    pub unsafe fn GetPeerSecureChannelInterface(&self) -> windows_core::Result<IWMSecureChannel> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPeerSecureChannelInterface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWMGetSecureChannel_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPeerSecureChannelInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMHeaderInfo, IWMHeaderInfo_Vtbl, 0x96406bda_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMHeaderInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMHeaderInfo, windows_core::IUnknown);
impl IWMHeaderInfo {
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAttributeCount)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
    }
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: windows_core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAttributeByIndex)(windows_core::Interface::as_raw(self), windex, pwstreamnum, core::mem::transmute(pwszname), pcchnamelen, ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn GetAttributeByName<P0>(&self, pwstreamnum: *mut u16, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetAttributeByName)(windows_core::Interface::as_raw(self), pwstreamnum, pszname.param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetAttribute<P0>(&self, wstreamnum: u16, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAttribute)(windows_core::Interface::as_raw(self), wstreamnum, pszname.param().abi(), r#type, core::mem::transmute(pvalue.as_ptr()), pvalue.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetMarkerCount(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMarkerCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: windows_core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMarker)(windows_core::Interface::as_raw(self), windex, core::mem::transmute(pwszmarkername), pcchmarkernamelen, pcnsmarkertime).ok()
    }
    pub unsafe fn AddMarker<P0>(&self, pwszmarkername: P0, cnsmarkertime: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddMarker)(windows_core::Interface::as_raw(self), pwszmarkername.param().abi(), cnsmarkertime).ok()
    }
    pub unsafe fn RemoveMarker(&self, windex: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveMarker)(windows_core::Interface::as_raw(self), windex).ok()
    }
    pub unsafe fn GetScriptCount(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetScriptCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: windows_core::PWSTR, pcchtypelen: *mut u16, pwszcommand: windows_core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetScript)(windows_core::Interface::as_raw(self), windex, core::mem::transmute(pwsztype), pcchtypelen, core::mem::transmute(pwszcommand), pcchcommandlen, pcnsscripttime).ok()
    }
    pub unsafe fn AddScript<P0, P1>(&self, pwsztype: P0, pwszcommand: P1, cnsscripttime: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddScript)(windows_core::Interface::as_raw(self), pwsztype.param().abi(), pwszcommand.param().abi(), cnsscripttime).ok()
    }
    pub unsafe fn RemoveScript(&self, windex: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveScript)(windows_core::Interface::as_raw(self), windex).ok()
    }
}
#[repr(C)]
pub struct IWMHeaderInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAttributeCount: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut u16) -> windows_core::HRESULT,
    pub GetAttributeByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut u16, windows_core::PWSTR, *mut u16, *mut WMT_ATTR_DATATYPE, *mut u8, *mut u16) -> windows_core::HRESULT,
    pub GetAttributeByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut u8, *mut u16) -> windows_core::HRESULT,
    pub SetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, u16, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const u8, u16) -> windows_core::HRESULT,
    pub GetMarkerCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetMarker: unsafe extern "system" fn(*mut core::ffi::c_void, u16, windows_core::PWSTR, *mut u16, *mut u64) -> windows_core::HRESULT,
    pub AddMarker: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u64) -> windows_core::HRESULT,
    pub RemoveMarker: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub GetScriptCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetScript: unsafe extern "system" fn(*mut core::ffi::c_void, u16, windows_core::PWSTR, *mut u16, windows_core::PWSTR, *mut u16, *mut u64) -> windows_core::HRESULT,
    pub AddScript: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u64) -> windows_core::HRESULT,
    pub RemoveScript: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMHeaderInfo2, IWMHeaderInfo2_Vtbl, 0x15cf9781_454e_482e_b393_85fae487a810);
impl core::ops::Deref for IWMHeaderInfo2 {
    type Target = IWMHeaderInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMHeaderInfo2, windows_core::IUnknown, IWMHeaderInfo);
impl IWMHeaderInfo2 {
    pub unsafe fn GetCodecInfoCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCodecInfoCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCodecInfo(&self, windex: u32, pcchname: *mut u16, pwszname: windows_core::PWSTR, pcchdescription: *mut u16, pwszdescription: windows_core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCodecInfo)(windows_core::Interface::as_raw(self), windex, pcchname, core::mem::transmute(pwszname), pcchdescription, core::mem::transmute(pwszdescription), pcodectype, pcbcodecinfo, pbcodecinfo).ok()
    }
}
#[repr(C)]
pub struct IWMHeaderInfo2_Vtbl {
    pub base__: IWMHeaderInfo_Vtbl,
    pub GetCodecInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCodecInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, windows_core::PWSTR, *mut u16, windows_core::PWSTR, *mut WMT_CODEC_INFO_TYPE, *mut u16, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMHeaderInfo3, IWMHeaderInfo3_Vtbl, 0x15cc68e3_27cc_4ecd_b222_3f5d02d80bd5);
impl core::ops::Deref for IWMHeaderInfo3 {
    type Target = IWMHeaderInfo2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMHeaderInfo3, windows_core::IUnknown, IWMHeaderInfo, IWMHeaderInfo2);
impl IWMHeaderInfo3 {
    pub unsafe fn GetAttributeCountEx(&self, wstreamnum: u16) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAttributeCountEx)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
    }
    pub unsafe fn GetAttributeIndices<P0>(&self, wstreamnum: u16, pwszname: P0, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetAttributeIndices)(windows_core::Interface::as_raw(self), wstreamnum, pwszname.param().abi(), pwlangindex, pwindices, pwcount).ok()
    }
    pub unsafe fn GetAttributeByIndexEx(&self, wstreamnum: u16, windex: u16, pwszname: windows_core::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAttributeByIndexEx)(windows_core::Interface::as_raw(self), wstreamnum, windex, core::mem::transmute(pwszname), pwnamelen, ptype, pwlangindex, pvalue, pdwdatalength).ok()
    }
    pub unsafe fn ModifyAttribute(&self, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ModifyAttribute)(windows_core::Interface::as_raw(self), wstreamnum, windex, r#type, wlangindex, core::mem::transmute(pvalue.as_ptr()), pvalue.len().try_into().unwrap()).ok()
    }
    pub unsafe fn AddAttribute<P0>(&self, wstreamnum: u16, pszname: P0, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddAttribute)(windows_core::Interface::as_raw(self), wstreamnum, pszname.param().abi(), pwindex, r#type, wlangindex, core::mem::transmute(pvalue.as_ptr()), pvalue.len().try_into().unwrap()).ok()
    }
    pub unsafe fn DeleteAttribute(&self, wstreamnum: u16, windex: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteAttribute)(windows_core::Interface::as_raw(self), wstreamnum, windex).ok()
    }
    pub unsafe fn AddCodecInfo<P0, P1>(&self, pwszname: P0, pwszdescription: P1, codectype: WMT_CODEC_INFO_TYPE, pbcodecinfo: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddCodecInfo)(windows_core::Interface::as_raw(self), pwszname.param().abi(), pwszdescription.param().abi(), codectype, pbcodecinfo.len().try_into().unwrap(), core::mem::transmute(pbcodecinfo.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IWMHeaderInfo3_Vtbl {
    pub base__: IWMHeaderInfo2_Vtbl,
    pub GetAttributeCountEx: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut u16) -> windows_core::HRESULT,
    pub GetAttributeIndices: unsafe extern "system" fn(*mut core::ffi::c_void, u16, windows_core::PCWSTR, *const u16, *mut u16, *mut u16) -> windows_core::HRESULT,
    pub GetAttributeByIndexEx: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, windows_core::PWSTR, *mut u16, *mut WMT_ATTR_DATATYPE, *mut u16, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub ModifyAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, WMT_ATTR_DATATYPE, u16, *const u8, u32) -> windows_core::HRESULT,
    pub AddAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, u16, windows_core::PCWSTR, *mut u16, WMT_ATTR_DATATYPE, u16, *const u8, u32) -> windows_core::HRESULT,
    pub DeleteAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16) -> windows_core::HRESULT,
    pub AddCodecInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, WMT_CODEC_INFO_TYPE, u16, *const u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMIStreamProps, IWMIStreamProps_Vtbl, 0x6816dad3_2b4b_4c8e_8149_874c3483a753);
impl core::ops::Deref for IWMIStreamProps {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMIStreamProps, windows_core::IUnknown);
impl IWMIStreamProps {
    pub unsafe fn GetProperty<P0>(&self, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), pszname.param().abi(), ptype, pvalue, pdwsize).ok()
    }
}
#[repr(C)]
pub struct IWMIStreamProps_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMImageInfo, IWMImageInfo_Vtbl, 0x9f0aa3b6_7267_4d89_88f2_ba915aa5c4c6);
impl core::ops::Deref for IWMImageInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMImageInfo, windows_core::IUnknown);
impl IWMImageInfo {
    pub unsafe fn GetImageCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetImageCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetImage(&self, windex: u32, pcchmimetype: *mut u16, pwszmimetype: windows_core::PWSTR, pcchdescription: *mut u16, pwszdescription: windows_core::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetImage)(windows_core::Interface::as_raw(self), windex, pcchmimetype, core::mem::transmute(pwszmimetype), pcchdescription, core::mem::transmute(pwszdescription), pimagetype, pcbimagedata, pbimagedata).ok()
    }
}
#[repr(C)]
pub struct IWMImageInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetImageCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetImage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, windows_core::PWSTR, *mut u16, windows_core::PWSTR, *mut u16, *mut u32, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMIndexer, IWMIndexer_Vtbl, 0x6d7cdc71_9888_11d3_8edc_00c04f6109cf);
impl core::ops::Deref for IWMIndexer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMIndexer, windows_core::IUnknown);
impl IWMIndexer {
    pub unsafe fn StartIndexing<P0, P1>(&self, pwszurl: P0, pcallback: P1, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWMStatusCallback>,
    {
        (windows_core::Interface::vtable(self).StartIndexing)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), pcallback.param().abi(), pvcontext).ok()
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMIndexer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartIndexing: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMIndexer2, IWMIndexer2_Vtbl, 0xb70f1e42_6255_4df0_a6b9_02b212d9e2bb);
impl core::ops::Deref for IWMIndexer2 {
    type Target = IWMIndexer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMIndexer2, windows_core::IUnknown, IWMIndexer);
impl IWMIndexer2 {
    pub unsafe fn Configure(&self, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const core::ffi::c_void, pvindextype: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Configure)(windows_core::Interface::as_raw(self), wstreamnum, nindexertype, pvinterval, pvindextype).ok()
    }
}
#[repr(C)]
pub struct IWMIndexer2_Vtbl {
    pub base__: IWMIndexer_Vtbl,
    pub Configure: unsafe extern "system" fn(*mut core::ffi::c_void, u16, WMT_INDEXER_TYPE, *const core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMInputMediaProps, IWMInputMediaProps_Vtbl, 0x96406bd5_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMInputMediaProps {
    type Target = IWMMediaProps;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMInputMediaProps, windows_core::IUnknown, IWMMediaProps);
impl IWMInputMediaProps {
    pub unsafe fn GetConnectionName(&self, pwszname: windows_core::PWSTR, pcchname: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetConnectionName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname), pcchname).ok()
    }
    pub unsafe fn GetGroupName(&self, pwszname: windows_core::PWSTR, pcchname: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGroupName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname), pcchname).ok()
    }
}
#[repr(C)]
pub struct IWMInputMediaProps_Vtbl {
    pub base__: IWMMediaProps_Vtbl,
    pub GetConnectionName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u16) -> windows_core::HRESULT,
    pub GetGroupName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMLanguageList, IWMLanguageList_Vtbl, 0xdf683f00_2d49_4d8e_92b7_fb19f6a0dc57);
impl core::ops::Deref for IWMLanguageList {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMLanguageList, windows_core::IUnknown);
impl IWMLanguageList {
    pub unsafe fn GetLanguageCount(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLanguageCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLanguageDetails(&self, windex: u16, pwszlanguagestring: windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLanguageDetails)(windows_core::Interface::as_raw(self), windex, core::mem::transmute(pwszlanguagestring), pcchlanguagestringlength).ok()
    }
    pub unsafe fn AddLanguageByRFC1766String<P0>(&self, pwszlanguagestring: P0) -> windows_core::Result<u16>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddLanguageByRFC1766String)(windows_core::Interface::as_raw(self), pwszlanguagestring.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWMLanguageList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLanguageCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetLanguageDetails: unsafe extern "system" fn(*mut core::ffi::c_void, u16, windows_core::PWSTR, *mut u16) -> windows_core::HRESULT,
    pub AddLanguageByRFC1766String: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMLicenseBackup, IWMLicenseBackup_Vtbl, 0x05e5ac9f_3fb6_4508_bb43_a4067ba1ebe8);
impl core::ops::Deref for IWMLicenseBackup {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMLicenseBackup, windows_core::IUnknown);
impl IWMLicenseBackup {
    pub unsafe fn BackupLicenses<P0>(&self, dwflags: u32, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMStatusCallback>,
    {
        (windows_core::Interface::vtable(self).BackupLicenses)(windows_core::Interface::as_raw(self), dwflags, pcallback.param().abi()).ok()
    }
    pub unsafe fn CancelLicenseBackup(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelLicenseBackup)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMLicenseBackup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BackupLicenses: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelLicenseBackup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMLicenseRestore, IWMLicenseRestore_Vtbl, 0xc70b6334_a22e_4efb_a245_15e65a004a13);
impl core::ops::Deref for IWMLicenseRestore {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMLicenseRestore, windows_core::IUnknown);
impl IWMLicenseRestore {
    pub unsafe fn RestoreLicenses<P0>(&self, dwflags: u32, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMStatusCallback>,
    {
        (windows_core::Interface::vtable(self).RestoreLicenses)(windows_core::Interface::as_raw(self), dwflags, pcallback.param().abi()).ok()
    }
    pub unsafe fn CancelLicenseRestore(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelLicenseRestore)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMLicenseRestore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RestoreLicenses: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelLicenseRestore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMLicenseRevocationAgent, IWMLicenseRevocationAgent_Vtbl, 0x6967f2c9_4e26_4b57_8894_799880f7ac7b);
impl core::ops::Deref for IWMLicenseRevocationAgent {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMLicenseRevocationAgent, windows_core::IUnknown);
impl IWMLicenseRevocationAgent {
    pub unsafe fn GetLRBChallenge(&self, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLRBChallenge)(windows_core::Interface::as_raw(self), pmachineid, dwmachineidlength, pchallenge, dwchallengelength, pchallengeoutput, pdwchallengeoutputlength).ok()
    }
    pub unsafe fn ProcessLRB(&self, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProcessLRB)(windows_core::Interface::as_raw(self), psignedlrb, dwsignedlrblength, psignedack, pdwsignedacklength).ok()
    }
}
#[repr(C)]
pub struct IWMLicenseRevocationAgent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLRBChallenge: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *const u8, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub ProcessLRB: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMMediaProps, IWMMediaProps_Vtbl, 0x96406bce_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMMediaProps {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMMediaProps, windows_core::IUnknown);
impl IWMMediaProps {
    pub unsafe fn GetType(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMediaType)(windows_core::Interface::as_raw(self), ptype, pcbtype).ok()
    }
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMediaType)(windows_core::Interface::as_raw(self), ptype).ok()
    }
}
#[repr(C)]
pub struct IWMMediaProps_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WM_MEDIA_TYPE, *mut u32) -> windows_core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *const WM_MEDIA_TYPE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMMetadataEditor, IWMMetadataEditor_Vtbl, 0x96406bd9_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMMetadataEditor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMMetadataEditor, windows_core::IUnknown);
impl IWMMetadataEditor {
    pub unsafe fn Open<P0>(&self, pwszfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pwszfilename.param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Flush(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMMetadataEditor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMMetadataEditor2, IWMMetadataEditor2_Vtbl, 0x203cffe3_2e18_4fdf_b59d_6e71530534cf);
impl core::ops::Deref for IWMMetadataEditor2 {
    type Target = IWMMetadataEditor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMMetadataEditor2, windows_core::IUnknown, IWMMetadataEditor);
impl IWMMetadataEditor2 {
    pub unsafe fn OpenEx<P0>(&self, pwszfilename: P0, dwdesiredaccess: u32, dwsharemode: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OpenEx)(windows_core::Interface::as_raw(self), pwszfilename.param().abi(), dwdesiredaccess, dwsharemode).ok()
    }
}
#[repr(C)]
pub struct IWMMetadataEditor2_Vtbl {
    pub base__: IWMMetadataEditor_Vtbl,
    pub OpenEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMMutualExclusion, IWMMutualExclusion_Vtbl, 0x96406bde_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMMutualExclusion {
    type Target = IWMStreamList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMMutualExclusion, windows_core::IUnknown, IWMStreamList);
impl IWMMutualExclusion {
    pub unsafe fn GetType(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetType(&self, guidtype: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), guidtype).ok()
    }
}
#[repr(C)]
pub struct IWMMutualExclusion_Vtbl {
    pub base__: IWMStreamList_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMMutualExclusion2, IWMMutualExclusion2_Vtbl, 0x0302b57d_89d1_4ba2_85c9_166f2c53eb91);
impl core::ops::Deref for IWMMutualExclusion2 {
    type Target = IWMMutualExclusion;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMMutualExclusion2, windows_core::IUnknown, IWMStreamList, IWMMutualExclusion);
impl IWMMutualExclusion2 {
    pub unsafe fn GetName(&self, pwszname: windows_core::PWSTR, pcchname: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname), pcchname).ok()
    }
    pub unsafe fn SetName<P0>(&self, pwszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), pwszname.param().abi()).ok()
    }
    pub unsafe fn GetRecordCount(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecordCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AddRecord(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddRecord)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemoveRecord(&self, wrecordnumber: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveRecord)(windows_core::Interface::as_raw(self), wrecordnumber).ok()
    }
    pub unsafe fn GetRecordName(&self, wrecordnumber: u16, pwszrecordname: windows_core::PWSTR, pcchrecordname: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRecordName)(windows_core::Interface::as_raw(self), wrecordnumber, core::mem::transmute(pwszrecordname), pcchrecordname).ok()
    }
    pub unsafe fn SetRecordName<P0>(&self, wrecordnumber: u16, pwszrecordname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRecordName)(windows_core::Interface::as_raw(self), wrecordnumber, pwszrecordname.param().abi()).ok()
    }
    pub unsafe fn GetStreamsForRecord(&self, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStreamsForRecord)(windows_core::Interface::as_raw(self), wrecordnumber, pwstreamnumarray, pcstreams).ok()
    }
    pub unsafe fn AddStreamForRecord(&self, wrecordnumber: u16, wstreamnumber: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddStreamForRecord)(windows_core::Interface::as_raw(self), wrecordnumber, wstreamnumber).ok()
    }
    pub unsafe fn RemoveStreamForRecord(&self, wrecordnumber: u16, wstreamnumber: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveStreamForRecord)(windows_core::Interface::as_raw(self), wrecordnumber, wstreamnumber).ok()
    }
}
#[repr(C)]
pub struct IWMMutualExclusion2_Vtbl {
    pub base__: IWMMutualExclusion_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u16) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetRecordCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub AddRecord: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveRecord: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub GetRecordName: unsafe extern "system" fn(*mut core::ffi::c_void, u16, windows_core::PWSTR, *mut u16) -> windows_core::HRESULT,
    pub SetRecordName: unsafe extern "system" fn(*mut core::ffi::c_void, u16, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetStreamsForRecord: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut u16, *mut u16) -> windows_core::HRESULT,
    pub AddStreamForRecord: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16) -> windows_core::HRESULT,
    pub RemoveStreamForRecord: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMOutputMediaProps, IWMOutputMediaProps_Vtbl, 0x96406bd7_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMOutputMediaProps {
    type Target = IWMMediaProps;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMOutputMediaProps, windows_core::IUnknown, IWMMediaProps);
impl IWMOutputMediaProps {
    pub unsafe fn GetStreamGroupName(&self, pwszname: windows_core::PWSTR, pcchname: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStreamGroupName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname), pcchname).ok()
    }
    pub unsafe fn GetConnectionName(&self, pwszname: windows_core::PWSTR, pcchname: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetConnectionName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname), pcchname).ok()
    }
}
#[repr(C)]
pub struct IWMOutputMediaProps_Vtbl {
    pub base__: IWMMediaProps_Vtbl,
    pub GetStreamGroupName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u16) -> windows_core::HRESULT,
    pub GetConnectionName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMPacketSize, IWMPacketSize_Vtbl, 0xcdfb97ab_188f_40b3_b643_5b7903975c59);
impl core::ops::Deref for IWMPacketSize {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMPacketSize, windows_core::IUnknown);
impl IWMPacketSize {
    pub unsafe fn GetMaxPacketSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxPacketSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxPacketSize(&self, dwmaxpacketsize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxPacketSize)(windows_core::Interface::as_raw(self), dwmaxpacketsize).ok()
    }
}
#[repr(C)]
pub struct IWMPacketSize_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMaxPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMPacketSize2, IWMPacketSize2_Vtbl, 0x8bfc2b9e_b646_4233_a877_1c6a079669dc);
impl core::ops::Deref for IWMPacketSize2 {
    type Target = IWMPacketSize;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMPacketSize2, windows_core::IUnknown, IWMPacketSize);
impl IWMPacketSize2 {
    pub unsafe fn GetMinPacketSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMinPacketSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMinPacketSize(&self, dwminpacketsize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinPacketSize)(windows_core::Interface::as_raw(self), dwminpacketsize).ok()
    }
}
#[repr(C)]
pub struct IWMPacketSize2_Vtbl {
    pub base__: IWMPacketSize_Vtbl,
    pub GetMinPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMinPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMPlayerHook, IWMPlayerHook_Vtbl, 0xe5b7ca9a_0f1c_4f66_9002_74ec50d8b304);
impl core::ops::Deref for IWMPlayerHook {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMPlayerHook, windows_core::IUnknown);
impl IWMPlayerHook {
    pub unsafe fn PreDecode(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PreDecode)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMPlayerHook_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PreDecode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMPlayerTimestampHook, IWMPlayerTimestampHook_Vtbl, 0x28580dda_d98e_48d0_b7ae_69e473a02825);
impl core::ops::Deref for IWMPlayerTimestampHook {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMPlayerTimestampHook, windows_core::IUnknown);
impl IWMPlayerTimestampHook {
    pub unsafe fn MapTimestamp(&self, rtin: i64) -> windows_core::Result<i64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MapTimestamp)(windows_core::Interface::as_raw(self), rtin, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWMPlayerTimestampHook_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MapTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMProfile, IWMProfile_Vtbl, 0x96406bdb_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMProfile {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMProfile, windows_core::IUnknown);
impl IWMProfile {
    pub unsafe fn GetVersion(&self) -> windows_core::Result<WMT_VERSION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetName(&self, pwszname: windows_core::PWSTR, pcchname: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname), pcchname).ok()
    }
    pub unsafe fn SetName<P0>(&self, pwszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), pwszname.param().abi()).ok()
    }
    pub unsafe fn GetDescription(&self, pwszdescription: windows_core::PWSTR, pcchdescription: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszdescription), pcchdescription).ok()
    }
    pub unsafe fn SetDescription<P0>(&self, pwszdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), pwszdescription.param().abi()).ok()
    }
    pub unsafe fn GetStreamCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStreamCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> windows_core::Result<IWMStreamConfig> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), dwstreamindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> windows_core::Result<IWMStreamConfig> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStreamByNumber)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveStream<P0>(&self, pconfig: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMStreamConfig>,
    {
        (windows_core::Interface::vtable(self).RemoveStream)(windows_core::Interface::as_raw(self), pconfig.param().abi()).ok()
    }
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveStreamByNumber)(windows_core::Interface::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn AddStream<P0>(&self, pconfig: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMStreamConfig>,
    {
        (windows_core::Interface::vtable(self).AddStream)(windows_core::Interface::as_raw(self), pconfig.param().abi()).ok()
    }
    pub unsafe fn ReconfigStream<P0>(&self, pconfig: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMStreamConfig>,
    {
        (windows_core::Interface::vtable(self).ReconfigStream)(windows_core::Interface::as_raw(self), pconfig.param().abi()).ok()
    }
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const windows_core::GUID) -> windows_core::Result<IWMStreamConfig> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateNewStream)(windows_core::Interface::as_raw(self), guidstreamtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMutualExclusionCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMutualExclusionCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> windows_core::Result<IWMMutualExclusion> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMutualExclusion)(windows_core::Interface::as_raw(self), dwmeindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveMutualExclusion<P0>(&self, pme: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMMutualExclusion>,
    {
        (windows_core::Interface::vtable(self).RemoveMutualExclusion)(windows_core::Interface::as_raw(self), pme.param().abi()).ok()
    }
    pub unsafe fn AddMutualExclusion<P0>(&self, pme: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMMutualExclusion>,
    {
        (windows_core::Interface::vtable(self).AddMutualExclusion)(windows_core::Interface::as_raw(self), pme.param().abi()).ok()
    }
    pub unsafe fn CreateNewMutualExclusion(&self) -> windows_core::Result<IWMMutualExclusion> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateNewMutualExclusion)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWMProfile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMT_VERSION) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetStreamCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStreamByNumber: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveStreamByNumber: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub AddStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReconfigStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNewStream: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMutualExclusionCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMutualExclusion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveMutualExclusion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddMutualExclusion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNewMutualExclusion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMProfile2, IWMProfile2_Vtbl, 0x07e72d33_d94e_4be7_8843_60ae5ff7e5f5);
impl core::ops::Deref for IWMProfile2 {
    type Target = IWMProfile;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMProfile2, windows_core::IUnknown, IWMProfile);
impl IWMProfile2 {
    pub unsafe fn GetProfileID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProfileID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWMProfile2_Vtbl {
    pub base__: IWMProfile_Vtbl,
    pub GetProfileID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMProfile3, IWMProfile3_Vtbl, 0x00ef96cc_a461_4546_8bcd_c9a28f0e06f5);
impl core::ops::Deref for IWMProfile3 {
    type Target = IWMProfile2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMProfile3, windows_core::IUnknown, IWMProfile, IWMProfile2);
impl IWMProfile3 {
    pub unsafe fn GetStorageFormat(&self) -> windows_core::Result<WMT_STORAGE_FORMAT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStorageFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStorageFormat(&self, nstorageformat: WMT_STORAGE_FORMAT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStorageFormat)(windows_core::Interface::as_raw(self), nstorageformat).ok()
    }
    pub unsafe fn GetBandwidthSharingCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBandwidthSharingCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetBandwidthSharing(&self, dwbsindex: u32) -> windows_core::Result<IWMBandwidthSharing> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBandwidthSharing)(windows_core::Interface::as_raw(self), dwbsindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveBandwidthSharing<P0>(&self, pbs: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMBandwidthSharing>,
    {
        (windows_core::Interface::vtable(self).RemoveBandwidthSharing)(windows_core::Interface::as_raw(self), pbs.param().abi()).ok()
    }
    pub unsafe fn AddBandwidthSharing<P0>(&self, pbs: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMBandwidthSharing>,
    {
        (windows_core::Interface::vtable(self).AddBandwidthSharing)(windows_core::Interface::as_raw(self), pbs.param().abi()).ok()
    }
    pub unsafe fn CreateNewBandwidthSharing(&self) -> windows_core::Result<IWMBandwidthSharing> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateNewBandwidthSharing)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetStreamPrioritization(&self) -> windows_core::Result<IWMStreamPrioritization> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStreamPrioritization)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetStreamPrioritization<P0>(&self, psp: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMStreamPrioritization>,
    {
        (windows_core::Interface::vtable(self).SetStreamPrioritization)(windows_core::Interface::as_raw(self), psp.param().abi()).ok()
    }
    pub unsafe fn RemoveStreamPrioritization(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveStreamPrioritization)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateNewStreamPrioritization(&self) -> windows_core::Result<IWMStreamPrioritization> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateNewStreamPrioritization)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetExpectedPacketCount(&self, msduration: u64) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetExpectedPacketCount)(windows_core::Interface::as_raw(self), msduration, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWMProfile3_Vtbl {
    pub base__: IWMProfile2_Vtbl,
    pub GetStorageFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMT_STORAGE_FORMAT) -> windows_core::HRESULT,
    pub SetStorageFormat: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_STORAGE_FORMAT) -> windows_core::HRESULT,
    pub GetBandwidthSharingCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetBandwidthSharing: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveBandwidthSharing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddBandwidthSharing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNewBandwidthSharing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStreamPrioritization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStreamPrioritization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveStreamPrioritization: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNewStreamPrioritization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetExpectedPacketCount: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMProfileManager, IWMProfileManager_Vtbl, 0xd16679f2_6ca0_472d_8d31_2f5d55aee155);
impl core::ops::Deref for IWMProfileManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMProfileManager, windows_core::IUnknown);
impl IWMProfileManager {
    pub unsafe fn CreateEmptyProfile(&self, dwversion: WMT_VERSION) -> windows_core::Result<IWMProfile> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEmptyProfile)(windows_core::Interface::as_raw(self), dwversion, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LoadProfileByID(&self, guidprofile: *const windows_core::GUID) -> windows_core::Result<IWMProfile> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoadProfileByID)(windows_core::Interface::as_raw(self), guidprofile, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LoadProfileByData<P0>(&self, pwszprofile: P0) -> windows_core::Result<IWMProfile>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoadProfileByData)(windows_core::Interface::as_raw(self), pwszprofile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SaveProfile<P0, P1>(&self, piwmprofile: P0, pwszprofile: P1, pdwlength: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMProfile>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SaveProfile)(windows_core::Interface::as_raw(self), piwmprofile.param().abi(), pwszprofile.param().abi(), pdwlength).ok()
    }
    pub unsafe fn GetSystemProfileCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSystemProfileCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LoadSystemProfile(&self, dwprofileindex: u32) -> windows_core::Result<IWMProfile> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoadSystemProfile)(windows_core::Interface::as_raw(self), dwprofileindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWMProfileManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateEmptyProfile: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_VERSION, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadProfileByID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadProfileByData: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetSystemProfileCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub LoadSystemProfile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMProfileManager2, IWMProfileManager2_Vtbl, 0x7a924e51_73c1_494d_8019_23d37ed9b89a);
impl core::ops::Deref for IWMProfileManager2 {
    type Target = IWMProfileManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMProfileManager2, windows_core::IUnknown, IWMProfileManager);
impl IWMProfileManager2 {
    pub unsafe fn GetSystemProfileVersion(&self, pdwversion: *mut WMT_VERSION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSystemProfileVersion)(windows_core::Interface::as_raw(self), pdwversion).ok()
    }
    pub unsafe fn SetSystemProfileVersion(&self, dwversion: WMT_VERSION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSystemProfileVersion)(windows_core::Interface::as_raw(self), dwversion).ok()
    }
}
#[repr(C)]
pub struct IWMProfileManager2_Vtbl {
    pub base__: IWMProfileManager_Vtbl,
    pub GetSystemProfileVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMT_VERSION) -> windows_core::HRESULT,
    pub SetSystemProfileVersion: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_VERSION) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMProfileManagerLanguage, IWMProfileManagerLanguage_Vtbl, 0xba4dcc78_7ee0_4ab8_b27a_dbce8bc51454);
impl core::ops::Deref for IWMProfileManagerLanguage {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMProfileManagerLanguage, windows_core::IUnknown);
impl IWMProfileManagerLanguage {
    pub unsafe fn GetUserLanguageID(&self, wlangid: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUserLanguageID)(windows_core::Interface::as_raw(self), wlangid).ok()
    }
    pub unsafe fn SetUserLanguageID(&self, wlangid: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUserLanguageID)(windows_core::Interface::as_raw(self), wlangid).ok()
    }
}
#[repr(C)]
pub struct IWMProfileManagerLanguage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUserLanguageID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SetUserLanguageID: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMPropertyVault, IWMPropertyVault_Vtbl, 0x72995a79_5090_42a4_9c8c_d9d0b6d34be5);
impl core::ops::Deref for IWMPropertyVault {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMPropertyVault, windows_core::IUnknown);
impl IWMPropertyVault {
    pub unsafe fn GetPropertyCount(&self, pdwcount: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropertyCount)(windows_core::Interface::as_raw(self), pdwcount).ok()
    }
    pub unsafe fn GetPropertyByName<P0>(&self, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetPropertyByName)(windows_core::Interface::as_raw(self), pszname.param().abi(), ptype, pvalue, pdwsize).ok()
    }
    pub unsafe fn SetProperty<P0>(&self, pszname: P0, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), pszname.param().abi(), ptype, pvalue, dwsize).ok()
    }
    pub unsafe fn GetPropertyByIndex(&self, dwindex: u32, pszname: windows_core::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropertyByIndex)(windows_core::Interface::as_raw(self), dwindex, core::mem::transmute(pszname), pdwnamelen, ptype, pvalue, pdwsize).ok()
    }
    pub unsafe fn CopyPropertiesFrom<P0>(&self, piwmpropertyvault: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMPropertyVault>,
    {
        (windows_core::Interface::vtable(self).CopyPropertiesFrom)(windows_core::Interface::as_raw(self), piwmpropertyvault.param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMPropertyVault_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPropertyCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32) -> windows_core::HRESULT,
    pub GetPropertyByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const u8, u32) -> windows_core::HRESULT,
    pub GetPropertyByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32, *mut WMT_ATTR_DATATYPE, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub CopyPropertiesFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMProximityDetection, IWMProximityDetection_Vtbl, 0x6a9fd8ee_b651_4bf0_b849_7d4ece79a2b1);
impl core::ops::Deref for IWMProximityDetection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMProximityDetection, windows_core::IUnknown);
impl IWMProximityDetection {
    pub unsafe fn StartDetection<P0>(&self, pbregistrationmsg: &[u8], pblocaladdress: &[u8], dwextraportsallowed: u32, ppregistrationresponsemsg: *mut Option<INSSBuffer>, pcallback: P0, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMStatusCallback>,
    {
        (windows_core::Interface::vtable(self).StartDetection)(windows_core::Interface::as_raw(self), core::mem::transmute(pbregistrationmsg.as_ptr()), pbregistrationmsg.len().try_into().unwrap(), core::mem::transmute(pblocaladdress.as_ptr()), pblocaladdress.len().try_into().unwrap(), dwextraportsallowed, core::mem::transmute(ppregistrationresponsemsg), pcallback.param().abi(), pvcontext).ok()
    }
}
#[repr(C)]
pub struct IWMProximityDetection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartDetection: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *const u8, u32, u32, *mut *mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMReader, IWMReader_Vtbl, 0x96406bd6_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReader, windows_core::IUnknown);
impl IWMReader {
    pub unsafe fn Open<P0, P1>(&self, pwszurl: P0, pcallback: P1, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWMReaderCallback>,
    {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), pcallback.param().abi(), pvcontext).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetOutputCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutputCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> windows_core::Result<IWMOutputMediaProps> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutputProps)(windows_core::Interface::as_raw(self), dwoutputnum, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOutputProps<P0>(&self, dwoutputnum: u32, poutput: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMOutputMediaProps>,
    {
        (windows_core::Interface::vtable(self).SetOutputProps)(windows_core::Interface::as_raw(self), dwoutputnum, poutput.param().abi()).ok()
    }
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnumber: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutputFormatCount)(windows_core::Interface::as_raw(self), dwoutputnumber, &mut result__).map(|| result__)
    }
    pub unsafe fn GetOutputFormat(&self, dwoutputnumber: u32, dwformatnumber: u32) -> windows_core::Result<IWMOutputMediaProps> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutputFormat)(windows_core::Interface::as_raw(self), dwoutputnumber, dwformatnumber, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Start(&self, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), cnsstart, cnsduration, frate, pvcontext).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutputCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOutputProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOutputProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutputFormatCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetOutputFormat: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, f32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMReaderAccelerator, IWMReaderAccelerator_Vtbl, 0xbddc4d08_944d_4d52_a612_46c3fda07dd4);
impl core::ops::Deref for IWMReaderAccelerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderAccelerator, windows_core::IUnknown);
impl IWMReaderAccelerator {
    pub unsafe fn GetCodecInterface(&self, dwoutputnum: u32, riid: *const windows_core::GUID, ppvcodecinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCodecInterface)(windows_core::Interface::as_raw(self), dwoutputnum, riid, ppvcodecinterface).ok()
    }
    pub unsafe fn Notify(&self, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), dwoutputnum, psubtype).ok()
    }
}
#[repr(C)]
pub struct IWMReaderAccelerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCodecInterface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const WM_MEDIA_TYPE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMReaderAdvanced, IWMReaderAdvanced_Vtbl, 0x96406bea_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMReaderAdvanced {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderAdvanced, windows_core::IUnknown);
impl IWMReaderAdvanced {
    pub unsafe fn SetUserProvidedClock<P0>(&self, fuserclock: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetUserProvidedClock)(windows_core::Interface::as_raw(self), fuserclock.param().abi()).ok()
    }
    pub unsafe fn GetUserProvidedClock(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUserProvidedClock)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeliverTime)(windows_core::Interface::as_raw(self), cnstime).ok()
    }
    pub unsafe fn SetManualStreamSelection<P0>(&self, fselection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetManualStreamSelection)(windows_core::Interface::as_raw(self), fselection.param().abi()).ok()
    }
    pub unsafe fn GetManualStreamSelection(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetManualStreamSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStreamsSelected)(windows_core::Interface::as_raw(self), cstreamcount, pwstreamnumbers, pselections).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> windows_core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStreamSelected)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
    }
    pub unsafe fn SetReceiveSelectionCallbacks<P0>(&self, fgetcallbacks: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetReceiveSelectionCallbacks)(windows_core::Interface::as_raw(self), fgetcallbacks.param().abi()).ok()
    }
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReceiveSelectionCallbacks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetReceiveStreamSamples<P0>(&self, wstreamnum: u16, freceivestreamsamples: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetReceiveStreamSamples)(windows_core::Interface::as_raw(self), wstreamnum, freceivestreamsamples.param().abi()).ok()
    }
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReceiveStreamSamples)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllocateForOutput<P0>(&self, dwoutputnum: u32, fallocate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllocateForOutput)(windows_core::Interface::as_raw(self), dwoutputnum, fallocate.param().abi()).ok()
    }
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllocateForOutput)(windows_core::Interface::as_raw(self), dwoutputnum, &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllocateForStream<P0>(&self, wstreamnum: u16, fallocate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllocateForStream)(windows_core::Interface::as_raw(self), wstreamnum, fallocate.param().abi()).ok()
    }
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllocateForStream)(windows_core::Interface::as_raw(self), dwsreamnum, &mut result__).map(|| result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatistics)(windows_core::Interface::as_raw(self), pstatistics).ok()
    }
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetClientInfo)(windows_core::Interface::as_raw(self), pclientinfo).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxOutputSampleSize)(windows_core::Interface::as_raw(self), dwoutput, &mut result__).map(|| result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxStreamSampleSize)(windows_core::Interface::as_raw(self), wstream, &mut result__).map(|| result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyLateDelivery)(windows_core::Interface::as_raw(self), cnslateness).ok()
    }
}
#[repr(C)]
pub struct IWMReaderAdvanced_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetUserProvidedClock: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetUserProvidedClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub DeliverTime: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub SetManualStreamSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetManualStreamSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetStreamsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *const u16, *const WMT_STREAM_SELECTION) -> windows_core::HRESULT,
    pub GetStreamSelected: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut WMT_STREAM_SELECTION) -> windows_core::HRESULT,
    pub SetReceiveSelectionCallbacks: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetReceiveSelectionCallbacks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetReceiveStreamSamples: unsafe extern "system" fn(*mut core::ffi::c_void, u16, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetReceiveStreamSamples: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetAllocateForOutput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetAllocateForOutput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetAllocateForStream: unsafe extern "system" fn(*mut core::ffi::c_void, u16, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetAllocateForStream: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WM_READER_STATISTICS) -> windows_core::HRESULT,
    pub SetClientInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const WM_READER_CLIENTINFO) -> windows_core::HRESULT,
    pub GetMaxOutputSampleSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetMaxStreamSampleSize: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut u32) -> windows_core::HRESULT,
    pub NotifyLateDelivery: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMReaderAdvanced2, IWMReaderAdvanced2_Vtbl, 0xae14a945_b90c_4d0d_9127_80d665f7d73e);
impl core::ops::Deref for IWMReaderAdvanced2 {
    type Target = IWMReaderAdvanced;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderAdvanced2, windows_core::IUnknown, IWMReaderAdvanced);
impl IWMReaderAdvanced2 {
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPlayMode)(windows_core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> windows_core::Result<WMT_PLAY_MODE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPlayMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBufferProgress)(windows_core::Interface::as_raw(self), pdwpercent, pcnsbuffering).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDownloadProgress)(windows_core::Interface::as_raw(self), pdwpercent, pqwbytesdownloaded, pcnsdownload).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSaveAsProgress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SaveFileAs<P0>(&self, pwszfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SaveFileAs)(windows_core::Interface::as_raw(self), pwszfilename.param().abi()).ok()
    }
    pub unsafe fn GetProtocolName(&self, pwszprotocol: windows_core::PWSTR, pcchprotocol: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProtocolName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszprotocol), pcchprotocol).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StartAtMarker)(windows_core::Interface::as_raw(self), wmarkerindex, cnsduration, frate, pvcontext).ok()
    }
    pub unsafe fn GetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetOutputSetting)(windows_core::Interface::as_raw(self), dwoutputnum, pszname.param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetOutputSetting)(windows_core::Interface::as_raw(self), dwoutputnum, pszname.param().abi(), r#type, core::mem::transmute(pvalue.as_ptr()), pvalue.len().try_into().unwrap()).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Preroll)(windows_core::Interface::as_raw(self), cnsstart, cnsduration, frate).ok()
    }
    pub unsafe fn SetLogClientID<P0>(&self, flogclientid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLogClientID)(windows_core::Interface::as_raw(self), flogclientid.param().abi()).ok()
    }
    pub unsafe fn GetLogClientID(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLogClientID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StopBuffering(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopBuffering)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<P0, P1>(&self, pstream: P0, pcallback: P1, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<IWMReaderCallback>,
    {
        (windows_core::Interface::vtable(self).OpenStream)(windows_core::Interface::as_raw(self), pstream.param().abi(), pcallback.param().abi(), pvcontext).ok()
    }
}
#[repr(C)]
pub struct IWMReaderAdvanced2_Vtbl {
    pub base__: IWMReaderAdvanced_Vtbl,
    pub SetPlayMode: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_PLAY_MODE) -> windows_core::HRESULT,
    pub GetPlayMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMT_PLAY_MODE) -> windows_core::HRESULT,
    pub GetBufferProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u64) -> windows_core::HRESULT,
    pub GetDownloadProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u64, *mut u64) -> windows_core::HRESULT,
    pub GetSaveAsProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SaveFileAs: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetProtocolName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub StartAtMarker: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u64, f32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutputSetting: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut u8, *mut u16) -> windows_core::HRESULT,
    pub SetOutputSetting: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const u8, u16) -> windows_core::HRESULT,
    pub Preroll: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, f32) -> windows_core::HRESULT,
    pub SetLogClientID: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetLogClientID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub StopBuffering: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenStream: usize,
}
windows_core::imp::define_interface!(IWMReaderAdvanced3, IWMReaderAdvanced3_Vtbl, 0x5dc0674b_f04b_4a4e_9f2a_b1afde2c8100);
impl core::ops::Deref for IWMReaderAdvanced3 {
    type Target = IWMReaderAdvanced2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderAdvanced3, windows_core::IUnknown, IWMReaderAdvanced, IWMReaderAdvanced2);
impl IWMReaderAdvanced3 {
    pub unsafe fn StopNetStreaming(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopNetStreaming)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const core::ffi::c_void, pvduration: *const core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StartAtPosition)(windows_core::Interface::as_raw(self), wstreamnum, pvoffsetstart, pvduration, dwoffsetformat, frate, pvcontext).ok()
    }
}
#[repr(C)]
pub struct IWMReaderAdvanced3_Vtbl {
    pub base__: IWMReaderAdvanced2_Vtbl,
    pub StopNetStreaming: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *const core::ffi::c_void, *const core::ffi::c_void, WMT_OFFSET_FORMAT, f32, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMReaderAdvanced4, IWMReaderAdvanced4_Vtbl, 0x945a76a2_12ae_4d48_bd3c_cd1d90399b85);
impl core::ops::Deref for IWMReaderAdvanced4 {
    type Target = IWMReaderAdvanced3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderAdvanced4, windows_core::IUnknown, IWMReaderAdvanced, IWMReaderAdvanced2, IWMReaderAdvanced3);
impl IWMReaderAdvanced4 {
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLanguageCount)(windows_core::Interface::as_raw(self), dwoutputnum, &mut result__).map(|| result__)
    }
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLanguage)(windows_core::Interface::as_raw(self), dwoutputnum, wlanguage, core::mem::transmute(pwszlanguagestring), pcchlanguagestringlength).ok()
    }
    pub unsafe fn GetMaxSpeedFactor(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxSpeedFactor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsUsingFastCache(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsUsingFastCache)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AddLogParam<P0, P1, P2>(&self, wsznamespace: P0, wszname: P1, wszvalue: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddLogParam)(windows_core::Interface::as_raw(self), wsznamespace.param().abi(), wszname.param().abi(), wszvalue.param().abi()).ok()
    }
    pub unsafe fn SendLogParams(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendLogParams)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CanSaveFileAs(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanSaveFileAs)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CancelSaveFileAs(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelSaveFileAs)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetURL(&self, pwszurl: windows_core::PWSTR, pcchurl: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetURL)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszurl), pcchurl).ok()
    }
}
#[repr(C)]
pub struct IWMReaderAdvanced4_Vtbl {
    pub base__: IWMReaderAdvanced3_Vtbl,
    pub GetLanguageCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16) -> windows_core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u16, windows_core::PWSTR, *mut u16) -> windows_core::HRESULT,
    pub GetMaxSpeedFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub IsUsingFastCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub AddLogParam: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SendLogParams: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanSaveFileAs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CancelSaveFileAs: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetURL: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMReaderAdvanced5, IWMReaderAdvanced5_Vtbl, 0x24c44db0_55d1_49ae_a5cc_f13815e36363);
impl core::ops::Deref for IWMReaderAdvanced5 {
    type Target = IWMReaderAdvanced4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderAdvanced5, windows_core::IUnknown, IWMReaderAdvanced, IWMReaderAdvanced2, IWMReaderAdvanced3, IWMReaderAdvanced4);
impl IWMReaderAdvanced5 {
    pub unsafe fn SetPlayerHook<P0>(&self, dwoutputnum: u32, phook: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMPlayerHook>,
    {
        (windows_core::Interface::vtable(self).SetPlayerHook)(windows_core::Interface::as_raw(self), dwoutputnum, phook.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWMReaderAdvanced5_Vtbl {
    pub base__: IWMReaderAdvanced4_Vtbl,
    pub SetPlayerHook: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMReaderAdvanced6, IWMReaderAdvanced6_Vtbl, 0x18a2e7f8_428f_4acd_8a00_e64639bc93de);
impl core::ops::Deref for IWMReaderAdvanced6 {
    type Target = IWMReaderAdvanced5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderAdvanced6, windows_core::IUnknown, IWMReaderAdvanced, IWMReaderAdvanced2, IWMReaderAdvanced3, IWMReaderAdvanced4, IWMReaderAdvanced5);
impl IWMReaderAdvanced6 {
    pub unsafe fn SetProtectStreamSamples(&self, pbcertificate: &[u8], dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProtectStreamSamples)(windows_core::Interface::as_raw(self), core::mem::transmute(pbcertificate.as_ptr()), pbcertificate.len().try_into().unwrap(), dwcertificatetype, dwflags, pbinitializationvector, pcbinitializationvector).ok()
    }
}
#[repr(C)]
pub struct IWMReaderAdvanced6_Vtbl {
    pub base__: IWMReaderAdvanced5_Vtbl,
    pub SetProtectStreamSamples: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, u32, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMReaderAllocatorEx, IWMReaderAllocatorEx_Vtbl, 0x9f762fa7_a22e_428d_93c9_ac82f3aafe5a);
impl core::ops::Deref for IWMReaderAllocatorEx {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderAllocatorEx, windows_core::IUnknown);
impl IWMReaderAllocatorEx {
    pub unsafe fn AllocateForStreamEx(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AllocateForStreamEx)(windows_core::Interface::as_raw(self), wstreamnum, cbbuffer, core::mem::transmute(ppbuffer), dwflags, cnssampletime, cnssampleduration, pvcontext).ok()
    }
    pub unsafe fn AllocateForOutputEx(&self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AllocateForOutputEx)(windows_core::Interface::as_raw(self), dwoutputnum, cbbuffer, core::mem::transmute(ppbuffer), dwflags, cnssampletime, cnssampleduration, pvcontext).ok()
    }
}
#[repr(C)]
pub struct IWMReaderAllocatorEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AllocateForStreamEx: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u32, *mut *mut core::ffi::c_void, u32, u64, u64, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub AllocateForOutputEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, u32, u64, u64, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMReaderCallback, IWMReaderCallback_Vtbl, 0x96406bd8_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMReaderCallback {
    type Target = IWMStatusCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderCallback, windows_core::IUnknown, IWMStatusCallback);
impl IWMReaderCallback {
    pub unsafe fn OnSample<P0>(&self, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: P0, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INSSBuffer>,
    {
        (windows_core::Interface::vtable(self).OnSample)(windows_core::Interface::as_raw(self), dwoutputnum, cnssampletime, cnssampleduration, dwflags, psample.param().abi(), pvcontext).ok()
    }
}
#[repr(C)]
pub struct IWMReaderCallback_Vtbl {
    pub base__: IWMStatusCallback_Vtbl,
    pub OnSample: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u64, u64, u32, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMReaderCallbackAdvanced, IWMReaderCallbackAdvanced_Vtbl, 0x96406beb_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMReaderCallbackAdvanced {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderCallbackAdvanced, windows_core::IUnknown);
impl IWMReaderCallbackAdvanced {
    pub unsafe fn OnStreamSample<P0>(&self, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: P0, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INSSBuffer>,
    {
        (windows_core::Interface::vtable(self).OnStreamSample)(windows_core::Interface::as_raw(self), wstreamnum, cnssampletime, cnssampleduration, dwflags, psample.param().abi(), pvcontext).ok()
    }
    pub unsafe fn OnTime(&self, cnscurrenttime: u64, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnTime)(windows_core::Interface::as_raw(self), cnscurrenttime, pvcontext).ok()
    }
    pub unsafe fn OnStreamSelection(&self, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnStreamSelection)(windows_core::Interface::as_raw(self), wstreamcount, pstreamnumbers, pselections, pvcontext).ok()
    }
    pub unsafe fn OnOutputPropsChanged(&self, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnOutputPropsChanged)(windows_core::Interface::as_raw(self), dwoutputnum, pmediatype, pvcontext).ok()
    }
    pub unsafe fn AllocateForStream(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut Option<INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AllocateForStream)(windows_core::Interface::as_raw(self), wstreamnum, cbbuffer, core::mem::transmute(ppbuffer), pvcontext).ok()
    }
    pub unsafe fn AllocateForOutput(&self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut Option<INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AllocateForOutput)(windows_core::Interface::as_raw(self), dwoutputnum, cbbuffer, core::mem::transmute(ppbuffer), pvcontext).ok()
    }
}
#[repr(C)]
pub struct IWMReaderCallbackAdvanced_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStreamSample: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u64, u64, u32, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub OnTime: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub OnStreamSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *const u16, *const WMT_STREAM_SELECTION, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub OnOutputPropsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const WM_MEDIA_TYPE, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub AllocateForStream: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u32, *mut *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub AllocateForOutput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMReaderNetworkConfig, IWMReaderNetworkConfig_Vtbl, 0x96406bec_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMReaderNetworkConfig {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderNetworkConfig, windows_core::IUnknown);
impl IWMReaderNetworkConfig {
    pub unsafe fn GetBufferingTime(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBufferingTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBufferingTime(&self, cnsbufferingtime: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBufferingTime)(windows_core::Interface::as_raw(self), cnsbufferingtime).ok()
    }
    pub unsafe fn GetUDPPortRanges(&self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUDPPortRanges)(windows_core::Interface::as_raw(self), prangearray, pcranges).ok()
    }
    pub unsafe fn SetUDPPortRanges(&self, prangearray: &[WM_PORT_NUMBER_RANGE]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUDPPortRanges)(windows_core::Interface::as_raw(self), core::mem::transmute(prangearray.as_ptr()), prangearray.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetProxySettings<P0>(&self, pwszprotocol: P0) -> windows_core::Result<WMT_PROXY_SETTINGS>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProxySettings)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SetProxySettings<P0>(&self, pwszprotocol: P0, proxysetting: WMT_PROXY_SETTINGS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetProxySettings)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), proxysetting).ok()
    }
    pub unsafe fn GetProxyHostName<P0>(&self, pwszprotocol: P0, pwszhostname: windows_core::PWSTR, pcchhostname: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetProxyHostName)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), core::mem::transmute(pwszhostname), pcchhostname).ok()
    }
    pub unsafe fn SetProxyHostName<P0, P1>(&self, pwszprotocol: P0, pwszhostname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetProxyHostName)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), pwszhostname.param().abi()).ok()
    }
    pub unsafe fn GetProxyPort<P0>(&self, pwszprotocol: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProxyPort)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SetProxyPort<P0>(&self, pwszprotocol: P0, dwport: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetProxyPort)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), dwport).ok()
    }
    pub unsafe fn GetProxyExceptionList<P0>(&self, pwszprotocol: P0, pwszexceptionlist: windows_core::PWSTR, pcchexceptionlist: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetProxyExceptionList)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), core::mem::transmute(pwszexceptionlist), pcchexceptionlist).ok()
    }
    pub unsafe fn SetProxyExceptionList<P0, P1>(&self, pwszprotocol: P0, pwszexceptionlist: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetProxyExceptionList)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), pwszexceptionlist.param().abi()).ok()
    }
    pub unsafe fn GetProxyBypassForLocal<P0>(&self, pwszprotocol: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProxyBypassForLocal)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SetProxyBypassForLocal<P0, P1>(&self, pwszprotocol: P0, fbypassforlocal: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetProxyBypassForLocal)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), fbypassforlocal.param().abi()).ok()
    }
    pub unsafe fn GetForceRerunAutoProxyDetection(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetForceRerunAutoProxyDetection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetForceRerunAutoProxyDetection<P0>(&self, fforcererundetection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetForceRerunAutoProxyDetection)(windows_core::Interface::as_raw(self), fforcererundetection.param().abi()).ok()
    }
    pub unsafe fn GetEnableMulticast(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnableMulticast)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnableMulticast<P0>(&self, fenablemulticast: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableMulticast)(windows_core::Interface::as_raw(self), fenablemulticast.param().abi()).ok()
    }
    pub unsafe fn GetEnableHTTP(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnableHTTP)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnableHTTP<P0>(&self, fenablehttp: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableHTTP)(windows_core::Interface::as_raw(self), fenablehttp.param().abi()).ok()
    }
    pub unsafe fn GetEnableUDP(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnableUDP)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnableUDP<P0>(&self, fenableudp: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableUDP)(windows_core::Interface::as_raw(self), fenableudp.param().abi()).ok()
    }
    pub unsafe fn GetEnableTCP(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnableTCP)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnableTCP<P0>(&self, fenabletcp: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableTCP)(windows_core::Interface::as_raw(self), fenabletcp.param().abi()).ok()
    }
    pub unsafe fn ResetProtocolRollover(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetProtocolRollover)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetConnectionBandwidth(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConnectionBandwidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetConnectionBandwidth(&self, dwconnectionbandwidth: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetConnectionBandwidth)(windows_core::Interface::as_raw(self), dwconnectionbandwidth).ok()
    }
    pub unsafe fn GetNumProtocolsSupported(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNumProtocolsSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSupportedProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: windows_core::PWSTR, pcchprotocolname: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSupportedProtocolName)(windows_core::Interface::as_raw(self), dwprotocolnum, core::mem::transmute(pwszprotocolname), pcchprotocolname).ok()
    }
    pub unsafe fn AddLoggingUrl<P0>(&self, pwszurl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddLoggingUrl)(windows_core::Interface::as_raw(self), pwszurl.param().abi()).ok()
    }
    pub unsafe fn GetLoggingUrl(&self, dwindex: u32, pwszurl: windows_core::PWSTR, pcchurl: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLoggingUrl)(windows_core::Interface::as_raw(self), dwindex, core::mem::transmute(pwszurl), pcchurl).ok()
    }
    pub unsafe fn GetLoggingUrlCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLoggingUrlCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ResetLoggingUrlList(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetLoggingUrlList)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMReaderNetworkConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBufferingTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetBufferingTime: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub GetUDPPortRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WM_PORT_NUMBER_RANGE, *mut u32) -> windows_core::HRESULT,
    pub SetUDPPortRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *const WM_PORT_NUMBER_RANGE, u32) -> windows_core::HRESULT,
    pub GetProxySettings: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WMT_PROXY_SETTINGS) -> windows_core::HRESULT,
    pub SetProxySettings: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, WMT_PROXY_SETTINGS) -> windows_core::HRESULT,
    pub GetProxyHostName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub SetProxyHostName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetProxyPort: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub SetProxyPort: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetProxyExceptionList: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub SetProxyExceptionList: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetProxyBypassForLocal: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetProxyBypassForLocal: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetForceRerunAutoProxyDetection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetForceRerunAutoProxyDetection: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetEnableMulticast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnableMulticast: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetEnableHTTP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnableHTTP: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetEnableUDP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnableUDP: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetEnableTCP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnableTCP: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ResetProtocolRollover: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConnectionBandwidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetConnectionBandwidth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetNumProtocolsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSupportedProtocolName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub AddLoggingUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetLoggingUrl: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetLoggingUrlCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ResetLoggingUrlList: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMReaderNetworkConfig2, IWMReaderNetworkConfig2_Vtbl, 0xd979a853_042b_4050_8387_c939db22013f);
impl core::ops::Deref for IWMReaderNetworkConfig2 {
    type Target = IWMReaderNetworkConfig;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderNetworkConfig2, windows_core::IUnknown, IWMReaderNetworkConfig);
impl IWMReaderNetworkConfig2 {
    pub unsafe fn GetEnableContentCaching(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnableContentCaching)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnableContentCaching<P0>(&self, fenablecontentcaching: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableContentCaching)(windows_core::Interface::as_raw(self), fenablecontentcaching.param().abi()).ok()
    }
    pub unsafe fn GetEnableFastCache(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnableFastCache)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnableFastCache<P0>(&self, fenablefastcache: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableFastCache)(windows_core::Interface::as_raw(self), fenablefastcache.param().abi()).ok()
    }
    pub unsafe fn GetAcceleratedStreamingDuration(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAcceleratedStreamingDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAcceleratedStreamingDuration(&self, cnsaccelduration: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAcceleratedStreamingDuration)(windows_core::Interface::as_raw(self), cnsaccelduration).ok()
    }
    pub unsafe fn GetAutoReconnectLimit(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAutoReconnectLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAutoReconnectLimit(&self, dwautoreconnectlimit: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAutoReconnectLimit)(windows_core::Interface::as_raw(self), dwautoreconnectlimit).ok()
    }
    pub unsafe fn GetEnableResends(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnableResends)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnableResends<P0>(&self, fenableresends: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableResends)(windows_core::Interface::as_raw(self), fenableresends.param().abi()).ok()
    }
    pub unsafe fn GetEnableThinning(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnableThinning)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnableThinning<P0>(&self, fenablethinning: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableThinning)(windows_core::Interface::as_raw(self), fenablethinning.param().abi()).ok()
    }
    pub unsafe fn GetMaxNetPacketSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxNetPacketSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWMReaderNetworkConfig2_Vtbl {
    pub base__: IWMReaderNetworkConfig_Vtbl,
    pub GetEnableContentCaching: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnableContentCaching: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetEnableFastCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnableFastCache: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetAcceleratedStreamingDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetAcceleratedStreamingDuration: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub GetAutoReconnectLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAutoReconnectLimit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetEnableResends: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnableResends: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetEnableThinning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnableThinning: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetMaxNetPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMReaderPlaylistBurn, IWMReaderPlaylistBurn_Vtbl, 0xf28c0300_9baa_4477_a846_1744d9cbf533);
impl core::ops::Deref for IWMReaderPlaylistBurn {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderPlaylistBurn, windows_core::IUnknown);
impl IWMReaderPlaylistBurn {
    pub unsafe fn InitPlaylistBurn<P0>(&self, cfiles: u32, ppwszfilenames: *const windows_core::PCWSTR, pcallback: P0, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMStatusCallback>,
    {
        (windows_core::Interface::vtable(self).InitPlaylistBurn)(windows_core::Interface::as_raw(self), cfiles, ppwszfilenames, pcallback.param().abi(), pvcontext).ok()
    }
    pub unsafe fn GetInitResults(&self, cfiles: u32) -> windows_core::Result<windows_core::HRESULT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInitResults)(windows_core::Interface::as_raw(self), cfiles, &mut result__).map(|| result__)
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndPlaylistBurn(&self, hrburnresult: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndPlaylistBurn)(windows_core::Interface::as_raw(self), hrburnresult).ok()
    }
}
#[repr(C)]
pub struct IWMReaderPlaylistBurn_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitPlaylistBurn: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::PCWSTR, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInitResults: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndPlaylistBurn: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMReaderStreamClock, IWMReaderStreamClock_Vtbl, 0x96406bed_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMReaderStreamClock {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderStreamClock, windows_core::IUnknown);
impl IWMReaderStreamClock {
    pub unsafe fn GetTime(&self, pcnsnow: *const u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTime)(windows_core::Interface::as_raw(self), pcnsnow).ok()
    }
    pub unsafe fn SetTimer(&self, cnswhen: u64, pvparam: *const core::ffi::c_void) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetTimer)(windows_core::Interface::as_raw(self), cnswhen, pvparam, &mut result__).map(|| result__)
    }
    pub unsafe fn KillTimer(&self, dwtimerid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KillTimer)(windows_core::Interface::as_raw(self), dwtimerid).ok()
    }
}
#[repr(C)]
pub struct IWMReaderStreamClock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTime: unsafe extern "system" fn(*mut core::ffi::c_void, *const u64) -> windows_core::HRESULT,
    pub SetTimer: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *const core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub KillTimer: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMReaderTimecode, IWMReaderTimecode_Vtbl, 0xf369e2f0_e081_4fe6_8450_b810b2f410d1);
impl core::ops::Deref for IWMReaderTimecode {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderTimecode, windows_core::IUnknown);
impl IWMReaderTimecode {
    pub unsafe fn GetTimecodeRangeCount(&self, wstreamnum: u16) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTimecodeRangeCount)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
    }
    pub unsafe fn GetTimecodeRangeBounds(&self, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTimecodeRangeBounds)(windows_core::Interface::as_raw(self), wstreamnum, wrangenum, pstarttimecode, pendtimecode).ok()
    }
}
#[repr(C)]
pub struct IWMReaderTimecode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTimecodeRangeCount: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut u16) -> windows_core::HRESULT,
    pub GetTimecodeRangeBounds: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMReaderTypeNegotiation, IWMReaderTypeNegotiation_Vtbl, 0xfdbe5592_81a1_41ea_93bd_735cad1adc05);
impl core::ops::Deref for IWMReaderTypeNegotiation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderTypeNegotiation, windows_core::IUnknown);
impl IWMReaderTypeNegotiation {
    pub unsafe fn TryOutputProps<P0>(&self, dwoutputnum: u32, poutput: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMOutputMediaProps>,
    {
        (windows_core::Interface::vtable(self).TryOutputProps)(windows_core::Interface::as_raw(self), dwoutputnum, poutput.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWMReaderTypeNegotiation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TryOutputProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMRegisterCallback, IWMRegisterCallback_Vtbl, 0xcf4b1f99_4de2_4e49_a363_252740d99bc1);
impl core::ops::Deref for IWMRegisterCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMRegisterCallback, windows_core::IUnknown);
impl IWMRegisterCallback {
    pub unsafe fn Advise<P0>(&self, pcallback: P0, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMStatusCallback>,
    {
        (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), pcallback.param().abi(), pvcontext).ok()
    }
    pub unsafe fn Unadvise<P0>(&self, pcallback: P0, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMStatusCallback>,
    {
        (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), pcallback.param().abi(), pvcontext).ok()
    }
}
#[repr(C)]
pub struct IWMRegisterCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMRegisteredDevice, IWMRegisteredDevice_Vtbl, 0xa4503bec_5508_4148_97ac_bfa75760a70d);
impl core::ops::Deref for IWMRegisteredDevice {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMRegisteredDevice, windows_core::IUnknown);
impl IWMRegisteredDevice {
    pub unsafe fn GetDeviceSerialNumber(&self) -> windows_core::Result<DRM_VAL16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceSerialNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDeviceCertificate(&self) -> windows_core::Result<INSSBuffer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceCertificate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDeviceType(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAttributeCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAttributeCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAttributeByIndex(&self, dwindex: u32, pbstrname: *mut windows_core::BSTR, pbstrvalue: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAttributeByIndex)(windows_core::Interface::as_raw(self), dwindex, core::mem::transmute(pbstrname), core::mem::transmute(pbstrvalue)).ok()
    }
    pub unsafe fn GetAttributeByName<P0>(&self, bstrname: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAttributeByName)(windows_core::Interface::as_raw(self), bstrname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAttributeByName<P0, P1>(&self, bstrname: P0, bstrvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetAttributeByName)(windows_core::Interface::as_raw(self), bstrname.param().abi(), bstrvalue.param().abi()).ok()
    }
    pub unsafe fn Approve<P0>(&self, fapprove: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Approve)(windows_core::Interface::as_raw(self), fapprove.param().abi()).ok()
    }
    pub unsafe fn IsValid(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsApproved(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsApproved)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsWmdrmCompliant(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsWmdrmCompliant)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsOpened(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsOpened)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Open(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMRegisteredDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDeviceSerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DRM_VAL16) -> windows_core::HRESULT,
    pub GetDeviceCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAttributeCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAttributeByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetAttributeByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetAttributeByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Approve: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsApproved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsWmdrmCompliant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsOpened: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMSBufferAllocator, IWMSBufferAllocator_Vtbl, 0x61103ca4_2033_11d2_9ef1_006097d2d7cf);
impl core::ops::Deref for IWMSBufferAllocator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMSBufferAllocator, windows_core::IUnknown);
impl IWMSBufferAllocator {
    pub unsafe fn AllocateBuffer(&self, dwmaxbuffersize: u32) -> windows_core::Result<INSSBuffer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllocateBuffer)(windows_core::Interface::as_raw(self), dwmaxbuffersize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AllocatePageSizeBuffer(&self, dwmaxbuffersize: u32) -> windows_core::Result<INSSBuffer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllocatePageSizeBuffer)(windows_core::Interface::as_raw(self), dwmaxbuffersize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWMSBufferAllocator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AllocateBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllocatePageSizeBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMSInternalAdminNetSource, IWMSInternalAdminNetSource_Vtbl, 0x8bb23e5f_d127_4afb_8d02_ae5b66d54c78);
impl core::ops::Deref for IWMSInternalAdminNetSource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMSInternalAdminNetSource, windows_core::IUnknown);
impl IWMSInternalAdminNetSource {
    pub unsafe fn Initialize<P0, P1, P2, P3>(&self, psharednamespace: P0, pnamespacenode: P1, pnetsourcecreator: P2, fembeddedinserver: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<INSNetSourceCreator>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), psharednamespace.param().abi(), pnamespacenode.param().abi(), pnetsourcecreator.param().abi(), fembeddedinserver.param().abi()).ok()
    }
    pub unsafe fn GetNetSourceCreator(&self) -> windows_core::Result<INSNetSourceCreator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetSourceCreator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCredentials<P0, P1, P2, P3, P4>(&self, bstrrealm: P0, bstrname: P1, bstrpassword: P2, fpersist: P3, fconfirmedgood: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
        P4: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetCredentials)(windows_core::Interface::as_raw(self), bstrrealm.param().abi(), bstrname.param().abi(), bstrpassword.param().abi(), fpersist.param().abi(), fconfirmedgood.param().abi()).ok()
    }
    pub unsafe fn GetCredentials<P0>(&self, bstrrealm: P0, pbstrname: *mut windows_core::BSTR, pbstrpassword: *mut windows_core::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetCredentials)(windows_core::Interface::as_raw(self), bstrrealm.param().abi(), core::mem::transmute(pbstrname), core::mem::transmute(pbstrpassword), pfconfirmedgood).ok()
    }
    pub unsafe fn DeleteCredentials<P0>(&self, bstrrealm: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteCredentials)(windows_core::Interface::as_raw(self), bstrrealm.param().abi()).ok()
    }
    pub unsafe fn GetCredentialFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCredentialFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCredentialFlags(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCredentialFlags)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn FindProxyForURL<P0, P1>(&self, bstrprotocol: P0, bstrhost: P1, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut windows_core::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).FindProxyForURL)(windows_core::Interface::as_raw(self), bstrprotocol.param().abi(), bstrhost.param().abi(), pfproxyenabled, core::mem::transmute(pbstrproxyserver), pdwproxyport, pdwproxycontext).ok()
    }
    pub unsafe fn RegisterProxyFailure(&self, hrparam: windows_core::HRESULT, dwproxycontext: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterProxyFailure)(windows_core::Interface::as_raw(self), hrparam, dwproxycontext).ok()
    }
    pub unsafe fn ShutdownProxyContext(&self, dwproxycontext: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShutdownProxyContext)(windows_core::Interface::as_raw(self), dwproxycontext).ok()
    }
    pub unsafe fn IsUsingIE(&self, dwproxycontext: u32) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsUsingIE)(windows_core::Interface::as_raw(self), dwproxycontext, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWMSInternalAdminNetSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetNetSourceCreator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub DeleteCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCredentialFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCredentialFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FindProxyForURL: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::BOOL, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub RegisterProxyFailure: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32) -> windows_core::HRESULT,
    pub ShutdownProxyContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsUsingIE: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMSInternalAdminNetSource2, IWMSInternalAdminNetSource2_Vtbl, 0xe74d58c3_cf77_4b51_af17_744687c43eae);
impl core::ops::Deref for IWMSInternalAdminNetSource2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMSInternalAdminNetSource2, windows_core::IUnknown);
impl IWMSInternalAdminNetSource2 {
    pub unsafe fn SetCredentialsEx<P0, P1, P2, P3, P4, P5, P6>(&self, bstrrealm: P0, bstrurl: P1, fproxy: P2, bstrname: P3, bstrpassword: P4, fpersist: P5, fconfirmedgood: P6) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
        P5: windows_core::Param<super::super::Foundation::BOOL>,
        P6: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetCredentialsEx)(windows_core::Interface::as_raw(self), bstrrealm.param().abi(), bstrurl.param().abi(), fproxy.param().abi(), bstrname.param().abi(), bstrpassword.param().abi(), fpersist.param().abi(), fconfirmedgood.param().abi()).ok()
    }
    pub unsafe fn GetCredentialsEx<P0, P1, P2>(&self, bstrrealm: P0, bstrurl: P1, fproxy: P2, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut windows_core::BSTR, pbstrpassword: *mut windows_core::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetCredentialsEx)(windows_core::Interface::as_raw(self), bstrrealm.param().abi(), bstrurl.param().abi(), fproxy.param().abi(), pdwurlpolicy, core::mem::transmute(pbstrname), core::mem::transmute(pbstrpassword), pfconfirmedgood).ok()
    }
    pub unsafe fn DeleteCredentialsEx<P0, P1, P2>(&self, bstrrealm: P0, bstrurl: P1, fproxy: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).DeleteCredentialsEx)(windows_core::Interface::as_raw(self), bstrrealm.param().abi(), bstrurl.param().abi(), fproxy.param().abi()).ok()
    }
    pub unsafe fn FindProxyForURLEx<P0, P1, P2>(&self, bstrprotocol: P0, bstrhost: P1, bstrurl: P2, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut windows_core::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).FindProxyForURLEx)(windows_core::Interface::as_raw(self), bstrprotocol.param().abi(), bstrhost.param().abi(), bstrurl.param().abi(), pfproxyenabled, core::mem::transmute(pbstrproxyserver), pdwproxyport, pdwproxycontext).ok()
    }
}
#[repr(C)]
pub struct IWMSInternalAdminNetSource2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetCredentialsEx: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::BOOL, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetCredentialsEx: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::BOOL, *mut NETSOURCE_URLCREDPOLICY_SETTINGS, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub DeleteCredentialsEx: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub FindProxyForURLEx: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::BOOL, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMSInternalAdminNetSource3, IWMSInternalAdminNetSource3_Vtbl, 0x6b63d08e_4590_44af_9eb3_57ff1e73bf80);
impl core::ops::Deref for IWMSInternalAdminNetSource3 {
    type Target = IWMSInternalAdminNetSource2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMSInternalAdminNetSource3, windows_core::IUnknown, IWMSInternalAdminNetSource2);
impl IWMSInternalAdminNetSource3 {
    pub unsafe fn GetNetSourceCreator2(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetSourceCreator2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindProxyForURLEx2<P0, P1, P2>(&self, bstrprotocol: P0, bstrhost: P1, bstrurl: P2, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut windows_core::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).FindProxyForURLEx2)(windows_core::Interface::as_raw(self), bstrprotocol.param().abi(), bstrhost.param().abi(), bstrurl.param().abi(), pfproxyenabled, core::mem::transmute(pbstrproxyserver), pdwproxyport, pqwproxycontext).ok()
    }
    pub unsafe fn RegisterProxyFailure2(&self, hrparam: windows_core::HRESULT, qwproxycontext: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterProxyFailure2)(windows_core::Interface::as_raw(self), hrparam, qwproxycontext).ok()
    }
    pub unsafe fn ShutdownProxyContext2(&self, qwproxycontext: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShutdownProxyContext2)(windows_core::Interface::as_raw(self), qwproxycontext).ok()
    }
    pub unsafe fn IsUsingIE2(&self, qwproxycontext: u64) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsUsingIE2)(windows_core::Interface::as_raw(self), qwproxycontext, &mut result__).map(|| result__)
    }
    pub unsafe fn SetCredentialsEx2<P0, P1, P2, P3, P4, P5, P6, P7>(&self, bstrrealm: P0, bstrurl: P1, fproxy: P2, bstrname: P3, bstrpassword: P4, fpersist: P5, fconfirmedgood: P6, fcleartextauthentication: P7) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
        P5: windows_core::Param<super::super::Foundation::BOOL>,
        P6: windows_core::Param<super::super::Foundation::BOOL>,
        P7: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetCredentialsEx2)(windows_core::Interface::as_raw(self), bstrrealm.param().abi(), bstrurl.param().abi(), fproxy.param().abi(), bstrname.param().abi(), bstrpassword.param().abi(), fpersist.param().abi(), fconfirmedgood.param().abi(), fcleartextauthentication.param().abi()).ok()
    }
    pub unsafe fn GetCredentialsEx2<P0, P1, P2, P3>(&self, bstrrealm: P0, bstrurl: P1, fproxy: P2, fcleartextauthentication: P3, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut windows_core::BSTR, pbstrpassword: *mut windows_core::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetCredentialsEx2)(windows_core::Interface::as_raw(self), bstrrealm.param().abi(), bstrurl.param().abi(), fproxy.param().abi(), fcleartextauthentication.param().abi(), pdwurlpolicy, core::mem::transmute(pbstrname), core::mem::transmute(pbstrpassword), pfconfirmedgood).ok()
    }
}
#[repr(C)]
pub struct IWMSInternalAdminNetSource3_Vtbl {
    pub base__: IWMSInternalAdminNetSource2_Vtbl,
    pub GetNetSourceCreator2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindProxyForURLEx2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::BOOL, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut u32, *mut u64) -> windows_core::HRESULT,
    pub RegisterProxyFailure2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u64) -> windows_core::HRESULT,
    pub ShutdownProxyContext2: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub IsUsingIE2: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetCredentialsEx2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::BOOL, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::BOOL, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetCredentialsEx2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *mut NETSOURCE_URLCREDPOLICY_SETTINGS, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMSecureChannel, IWMSecureChannel_Vtbl, 0x2720598a_d0f2_4189_bd10_91c46ef0936f);
impl core::ops::Deref for IWMSecureChannel {
    type Target = IWMAuthorizer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMSecureChannel, windows_core::IUnknown, IWMAuthorizer);
impl IWMSecureChannel {
    pub unsafe fn WMSC_AddCertificate<P0>(&self, pcert: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMAuthorizer>,
    {
        (windows_core::Interface::vtable(self).WMSC_AddCertificate)(windows_core::Interface::as_raw(self), pcert.param().abi()).ok()
    }
    pub unsafe fn WMSC_AddSignature(&self, pbcertsig: *const u8, cbcertsig: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WMSC_AddSignature)(windows_core::Interface::as_raw(self), pbcertsig, cbcertsig).ok()
    }
    pub unsafe fn WMSC_Connect<P0>(&self, potherside: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMSecureChannel>,
    {
        (windows_core::Interface::vtable(self).WMSC_Connect)(windows_core::Interface::as_raw(self), potherside.param().abi()).ok()
    }
    pub unsafe fn WMSC_IsConnected(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WMSC_IsConnected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn WMSC_Disconnect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WMSC_Disconnect)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WMSC_GetValidCertificate(&self, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WMSC_GetValidCertificate)(windows_core::Interface::as_raw(self), ppbcertificate, pdwsignature).ok()
    }
    pub unsafe fn WMSC_Encrypt(&self, pbdata: *const u8, cbdata: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WMSC_Encrypt)(windows_core::Interface::as_raw(self), pbdata, cbdata).ok()
    }
    pub unsafe fn WMSC_Decrypt(&self, pbdata: *const u8, cbdata: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WMSC_Decrypt)(windows_core::Interface::as_raw(self), pbdata, cbdata).ok()
    }
    pub unsafe fn WMSC_Lock(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WMSC_Lock)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WMSC_Unlock(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WMSC_Unlock)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WMSC_SetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WMSC_SetSharedData)(windows_core::Interface::as_raw(self), dwcertindex, pbshareddata).ok()
    }
}
#[repr(C)]
pub struct IWMSecureChannel_Vtbl {
    pub base__: IWMAuthorizer_Vtbl,
    pub WMSC_AddCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WMSC_AddSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub WMSC_Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WMSC_IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub WMSC_Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WMSC_GetValidCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub WMSC_Encrypt: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub WMSC_Decrypt: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub WMSC_Lock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WMSC_Unlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WMSC_SetSharedData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMStatusCallback, IWMStatusCallback_Vtbl, 0x6d7cdc70_9888_11d3_8edc_00c04f6109cf);
impl core::ops::Deref for IWMStatusCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMStatusCallback, windows_core::IUnknown);
impl IWMStatusCallback {
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: windows_core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnStatus)(windows_core::Interface::as_raw(self), status, hr, dwtype, pvalue, pvcontext).ok()
    }
}
#[repr(C)]
pub struct IWMStatusCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStatus: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_STATUS, windows_core::HRESULT, WMT_ATTR_DATATYPE, *const u8, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMStreamConfig, IWMStreamConfig_Vtbl, 0x96406bdc_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMStreamConfig {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMStreamConfig, windows_core::IUnknown);
impl IWMStreamConfig {
    pub unsafe fn GetStreamType(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStreamType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetStreamNumber(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStreamNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStreamNumber)(windows_core::Interface::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn GetStreamName(&self, pwszstreamname: windows_core::PWSTR, pcchstreamname: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStreamName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszstreamname), pcchstreamname).ok()
    }
    pub unsafe fn SetStreamName<P0>(&self, pwszstreamname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetStreamName)(windows_core::Interface::as_raw(self), pwszstreamname.param().abi()).ok()
    }
    pub unsafe fn GetConnectionName(&self, pwszinputname: windows_core::PWSTR, pcchinputname: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetConnectionName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszinputname), pcchinputname).ok()
    }
    pub unsafe fn SetConnectionName<P0>(&self, pwszinputname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetConnectionName)(windows_core::Interface::as_raw(self), pwszinputname.param().abi()).ok()
    }
    pub unsafe fn GetBitrate(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBitrate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBitrate)(windows_core::Interface::as_raw(self), pdwbitrate).ok()
    }
    pub unsafe fn GetBufferWindow(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBufferWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBufferWindow)(windows_core::Interface::as_raw(self), msbufferwindow).ok()
    }
}
#[repr(C)]
pub struct IWMStreamConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStreamType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetStreamNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SetStreamNumber: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub GetStreamName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u16) -> windows_core::HRESULT,
    pub SetStreamName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetConnectionName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u16) -> windows_core::HRESULT,
    pub SetConnectionName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetBufferWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBufferWindow: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMStreamConfig2, IWMStreamConfig2_Vtbl, 0x7688d8cb_fc0d_43bd_9459_5a8dec200cfa);
impl core::ops::Deref for IWMStreamConfig2 {
    type Target = IWMStreamConfig;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMStreamConfig2, windows_core::IUnknown, IWMStreamConfig);
impl IWMStreamConfig2 {
    pub unsafe fn GetTransportType(&self) -> windows_core::Result<WMT_TRANSPORT_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransportType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTransportType(&self, ntransporttype: WMT_TRANSPORT_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTransportType)(windows_core::Interface::as_raw(self), ntransporttype).ok()
    }
    pub unsafe fn AddDataUnitExtension(&self, guidextensionsystemid: windows_core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddDataUnitExtension)(windows_core::Interface::as_raw(self), core::mem::transmute(guidextensionsystemid), cbextensiondatasize, core::mem::transmute(pbextensionsysteminfo.as_ptr()), pbextensionsysteminfo.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDataUnitExtensionCount(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDataUnitExtensionCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDataUnitExtension(&self, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut windows_core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDataUnitExtension)(windows_core::Interface::as_raw(self), wdataunitextensionnumber, pguidextensionsystemid, pcbextensiondatasize, pbextensionsysteminfo, pcbextensionsysteminfo).ok()
    }
    pub unsafe fn RemoveAllDataUnitExtensions(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAllDataUnitExtensions)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMStreamConfig2_Vtbl {
    pub base__: IWMStreamConfig_Vtbl,
    pub GetTransportType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMT_TRANSPORT_TYPE) -> windows_core::HRESULT,
    pub SetTransportType: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_TRANSPORT_TYPE) -> windows_core::HRESULT,
    pub AddDataUnitExtension: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u16, *const u8, u32) -> windows_core::HRESULT,
    pub GetDataUnitExtensionCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetDataUnitExtension: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut windows_core::GUID, *mut u16, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub RemoveAllDataUnitExtensions: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMStreamConfig3, IWMStreamConfig3_Vtbl, 0xcb164104_3aa9_45a7_9ac9_4daee131d6e1);
impl core::ops::Deref for IWMStreamConfig3 {
    type Target = IWMStreamConfig2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMStreamConfig3, windows_core::IUnknown, IWMStreamConfig, IWMStreamConfig2);
impl IWMStreamConfig3 {
    pub unsafe fn GetLanguage(&self, pwszlanguagestring: windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLanguage)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszlanguagestring), pcchlanguagestringlength).ok()
    }
    pub unsafe fn SetLanguage<P0>(&self, pwszlanguagestring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetLanguage)(windows_core::Interface::as_raw(self), pwszlanguagestring.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWMStreamConfig3_Vtbl {
    pub base__: IWMStreamConfig2_Vtbl,
    pub GetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u16) -> windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMStreamList, IWMStreamList_Vtbl, 0x96406bdd_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMStreamList {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMStreamList, windows_core::IUnknown);
impl IWMStreamList {
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStreams)(windows_core::Interface::as_raw(self), pwstreamnumarray, pcstreams).ok()
    }
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddStream)(windows_core::Interface::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveStream)(windows_core::Interface::as_raw(self), wstreamnum).ok()
    }
}
#[repr(C)]
pub struct IWMStreamList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, *mut u16) -> windows_core::HRESULT,
    pub AddStream: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub RemoveStream: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMStreamPrioritization, IWMStreamPrioritization_Vtbl, 0x8c1c6090_f9a8_4748_8ec3_dd1108ba1e77);
impl core::ops::Deref for IWMStreamPrioritization {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMStreamPrioritization, windows_core::IUnknown);
impl IWMStreamPrioritization {
    pub unsafe fn GetPriorityRecords(&self, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPriorityRecords)(windows_core::Interface::as_raw(self), precordarray, pcrecords).ok()
    }
    pub unsafe fn SetPriorityRecords(&self, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPriorityRecords)(windows_core::Interface::as_raw(self), precordarray, crecords).ok()
    }
}
#[repr(C)]
pub struct IWMStreamPrioritization_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPriorityRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WM_STREAM_PRIORITY_RECORD, *mut u16) -> windows_core::HRESULT,
    pub SetPriorityRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *const WM_STREAM_PRIORITY_RECORD, u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMSyncReader, IWMSyncReader_Vtbl, 0x9397f121_7705_4dc9_b049_98b698188414);
impl core::ops::Deref for IWMSyncReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMSyncReader, windows_core::IUnknown);
impl IWMSyncReader {
    pub unsafe fn Open<P0>(&self, pwszfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pwszfilename.param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetRange(&self, cnsstarttime: u64, cnsduration: i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRange)(windows_core::Interface::as_raw(self), cnsstarttime, cnsduration).ok()
    }
    pub unsafe fn SetRangeByFrame(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRangeByFrame)(windows_core::Interface::as_raw(self), wstreamnum, qwframenumber, cframestoread).ok()
    }
    pub unsafe fn GetNextSample(&self, wstreamnum: u16, ppsample: *mut Option<INSSBuffer>, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNextSample)(windows_core::Interface::as_raw(self), wstreamnum, core::mem::transmute(ppsample), pcnssampletime, pcnsduration, pdwflags, pdwoutputnum, pwstreamnum).ok()
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStreamsSelected)(windows_core::Interface::as_raw(self), cstreamcount, pwstreamnumbers, pselections).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> windows_core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStreamSelected)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
    }
    pub unsafe fn SetReadStreamSamples<P0>(&self, wstreamnum: u16, fcompressed: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetReadStreamSamples)(windows_core::Interface::as_raw(self), wstreamnum, fcompressed.param().abi()).ok()
    }
    pub unsafe fn GetReadStreamSamples(&self, wstreamnum: u16) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReadStreamSamples)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
    }
    pub unsafe fn GetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetOutputSetting)(windows_core::Interface::as_raw(self), dwoutputnum, pszname.param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetOutputSetting)(windows_core::Interface::as_raw(self), dwoutputnum, pszname.param().abi(), r#type, core::mem::transmute(pvalue.as_ptr()), pvalue.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetOutputCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutputCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> windows_core::Result<IWMOutputMediaProps> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutputProps)(windows_core::Interface::as_raw(self), dwoutputnum, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOutputProps<P0>(&self, dwoutputnum: u32, poutput: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMOutputMediaProps>,
    {
        (windows_core::Interface::vtable(self).SetOutputProps)(windows_core::Interface::as_raw(self), dwoutputnum, poutput.param().abi()).ok()
    }
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnum: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutputFormatCount)(windows_core::Interface::as_raw(self), dwoutputnum, &mut result__).map(|| result__)
    }
    pub unsafe fn GetOutputFormat(&self, dwoutputnum: u32, dwformatnum: u32) -> windows_core::Result<IWMOutputMediaProps> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutputFormat)(windows_core::Interface::as_raw(self), dwoutputnum, dwformatnum, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetOutputNumberForStream(&self, wstreamnum: u16) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutputNumberForStream)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
    }
    pub unsafe fn GetStreamNumberForOutput(&self, dwoutputnum: u32) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStreamNumberForOutput)(windows_core::Interface::as_raw(self), dwoutputnum, &mut result__).map(|| result__)
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxOutputSampleSize)(windows_core::Interface::as_raw(self), dwoutput, &mut result__).map(|| result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxStreamSampleSize)(windows_core::Interface::as_raw(self), wstream, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<P0>(&self, pstream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).OpenStream)(windows_core::Interface::as_raw(self), pstream.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWMSyncReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRange: unsafe extern "system" fn(*mut core::ffi::c_void, u64, i64) -> windows_core::HRESULT,
    pub SetRangeByFrame: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u64, i64) -> windows_core::HRESULT,
    pub GetNextSample: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void, *mut u64, *mut u64, *mut u32, *mut u32, *mut u16) -> windows_core::HRESULT,
    pub SetStreamsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *const u16, *const WMT_STREAM_SELECTION) -> windows_core::HRESULT,
    pub GetStreamSelected: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut WMT_STREAM_SELECTION) -> windows_core::HRESULT,
    pub SetReadStreamSamples: unsafe extern "system" fn(*mut core::ffi::c_void, u16, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetReadStreamSamples: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetOutputSetting: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut u8, *mut u16) -> windows_core::HRESULT,
    pub SetOutputSetting: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const u8, u16) -> windows_core::HRESULT,
    pub GetOutputCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOutputProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOutputProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutputFormatCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetOutputFormat: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutputNumberForStream: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut u32) -> windows_core::HRESULT,
    pub GetStreamNumberForOutput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16) -> windows_core::HRESULT,
    pub GetMaxOutputSampleSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetMaxStreamSampleSize: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenStream: usize,
}
windows_core::imp::define_interface!(IWMSyncReader2, IWMSyncReader2_Vtbl, 0xfaed3d21_1b6b_4af7_8cb6_3e189bbc187b);
impl core::ops::Deref for IWMSyncReader2 {
    type Target = IWMSyncReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMSyncReader2, windows_core::IUnknown, IWMSyncReader);
impl IWMSyncReader2 {
    pub unsafe fn SetRangeByTimecode(&self, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRangeByTimecode)(windows_core::Interface::as_raw(self), wstreamnum, pstart, pend).ok()
    }
    pub unsafe fn SetRangeByFrameEx(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetRangeByFrameEx)(windows_core::Interface::as_raw(self), wstreamnum, qwframenumber, cframestoread, &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllocateForOutput<P0>(&self, dwoutputnum: u32, pallocator: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMReaderAllocatorEx>,
    {
        (windows_core::Interface::vtable(self).SetAllocateForOutput)(windows_core::Interface::as_raw(self), dwoutputnum, pallocator.param().abi()).ok()
    }
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> windows_core::Result<IWMReaderAllocatorEx> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllocateForOutput)(windows_core::Interface::as_raw(self), dwoutputnum, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAllocateForStream<P0>(&self, wstreamnum: u16, pallocator: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMReaderAllocatorEx>,
    {
        (windows_core::Interface::vtable(self).SetAllocateForStream)(windows_core::Interface::as_raw(self), wstreamnum, pallocator.param().abi()).ok()
    }
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> windows_core::Result<IWMReaderAllocatorEx> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllocateForStream)(windows_core::Interface::as_raw(self), dwsreamnum, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWMSyncReader2_Vtbl {
    pub base__: IWMSyncReader_Vtbl,
    pub SetRangeByTimecode: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *const WMT_TIMECODE_EXTENSION_DATA, *const WMT_TIMECODE_EXTENSION_DATA) -> windows_core::HRESULT,
    pub SetRangeByFrameEx: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u64, i64, *mut u64) -> windows_core::HRESULT,
    pub SetAllocateForOutput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAllocateForOutput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAllocateForStream: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAllocateForStream: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMVideoMediaProps, IWMVideoMediaProps_Vtbl, 0x96406bcf_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMVideoMediaProps {
    type Target = IWMMediaProps;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMVideoMediaProps, windows_core::IUnknown, IWMMediaProps);
impl IWMVideoMediaProps {
    pub unsafe fn GetMaxKeyFrameSpacing(&self) -> windows_core::Result<i64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxKeyFrameSpacing)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxKeyFrameSpacing(&self, lltime: i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxKeyFrameSpacing)(windows_core::Interface::as_raw(self), lltime).ok()
    }
    pub unsafe fn GetQuality(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetQuality)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQuality(&self, dwquality: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQuality)(windows_core::Interface::as_raw(self), dwquality).ok()
    }
}
#[repr(C)]
pub struct IWMVideoMediaProps_Vtbl {
    pub base__: IWMMediaProps_Vtbl,
    pub GetMaxKeyFrameSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub SetMaxKeyFrameSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GetQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetQuality: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMWatermarkInfo, IWMWatermarkInfo_Vtbl, 0x6f497062_f2e2_4624_8ea7_9dd40d81fc8d);
impl core::ops::Deref for IWMWatermarkInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWatermarkInfo, windows_core::IUnknown);
impl IWMWatermarkInfo {
    pub unsafe fn GetWatermarkEntryCount(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWatermarkEntryCount)(windows_core::Interface::as_raw(self), wmettype, &mut result__).map(|| result__)
    }
    pub unsafe fn GetWatermarkEntry(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWatermarkEntry)(windows_core::Interface::as_raw(self), wmettype, dwentrynum, pentry).ok()
    }
}
#[repr(C)]
pub struct IWMWatermarkInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWatermarkEntryCount: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_WATERMARK_ENTRY_TYPE, *mut u32) -> windows_core::HRESULT,
    pub GetWatermarkEntry: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_WATERMARK_ENTRY_TYPE, u32, *mut WMT_WATERMARK_ENTRY) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMWriter, IWMWriter_Vtbl, 0x96406bd4_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMWriter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriter, windows_core::IUnknown);
impl IWMWriter {
    pub unsafe fn SetProfileByID(&self, guidprofile: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProfileByID)(windows_core::Interface::as_raw(self), guidprofile).ok()
    }
    pub unsafe fn SetProfile<P0>(&self, pprofile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMProfile>,
    {
        (windows_core::Interface::vtable(self).SetProfile)(windows_core::Interface::as_raw(self), pprofile.param().abi()).ok()
    }
    pub unsafe fn SetOutputFilename<P0>(&self, pwszfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetOutputFilename)(windows_core::Interface::as_raw(self), pwszfilename.param().abi()).ok()
    }
    pub unsafe fn GetInputCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInputCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetInputProps(&self, dwinputnum: u32) -> windows_core::Result<IWMInputMediaProps> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInputProps)(windows_core::Interface::as_raw(self), dwinputnum, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetInputProps<P0>(&self, dwinputnum: u32, pinput: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMInputMediaProps>,
    {
        (windows_core::Interface::vtable(self).SetInputProps)(windows_core::Interface::as_raw(self), dwinputnum, pinput.param().abi()).ok()
    }
    pub unsafe fn GetInputFormatCount(&self, dwinputnumber: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInputFormatCount)(windows_core::Interface::as_raw(self), dwinputnumber, &mut result__).map(|| result__)
    }
    pub unsafe fn GetInputFormat(&self, dwinputnumber: u32, dwformatnumber: u32) -> windows_core::Result<IWMInputMediaProps> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInputFormat)(windows_core::Interface::as_raw(self), dwinputnumber, dwformatnumber, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BeginWriting(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginWriting)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndWriting(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndWriting)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AllocateSample(&self, dwsamplesize: u32) -> windows_core::Result<INSSBuffer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllocateSample)(windows_core::Interface::as_raw(self), dwsamplesize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn WriteSample<P0>(&self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INSSBuffer>,
    {
        (windows_core::Interface::vtable(self).WriteSample)(windows_core::Interface::as_raw(self), dwinputnum, cnssampletime, dwflags, psample.param().abi()).ok()
    }
    pub unsafe fn Flush(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetProfileByID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOutputFilename: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetInputCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetInputProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInputProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInputFormatCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetInputFormat: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginWriting: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndWriting: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllocateSample: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteSample: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u64, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMWriterAdvanced, IWMWriterAdvanced_Vtbl, 0x96406be3_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMWriterAdvanced {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterAdvanced, windows_core::IUnknown);
impl IWMWriterAdvanced {
    pub unsafe fn GetSinkCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSinkCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> windows_core::Result<IWMWriterSink> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSink)(windows_core::Interface::as_raw(self), dwsinknum, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddSink<P0>(&self, psink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMWriterSink>,
    {
        (windows_core::Interface::vtable(self).AddSink)(windows_core::Interface::as_raw(self), psink.param().abi()).ok()
    }
    pub unsafe fn RemoveSink<P0>(&self, psink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMWriterSink>,
    {
        (windows_core::Interface::vtable(self).RemoveSink)(windows_core::Interface::as_raw(self), psink.param().abi()).ok()
    }
    pub unsafe fn WriteStreamSample<P0>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INSSBuffer>,
    {
        (windows_core::Interface::vtable(self).WriteStreamSample)(windows_core::Interface::as_raw(self), wstreamnum, cnssampletime, mssamplesendtime, cnssampleduration, dwflags, psample.param().abi()).ok()
    }
    pub unsafe fn SetLiveSource<P0>(&self, fislivesource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLiveSource)(windows_core::Interface::as_raw(self), fislivesource.param().abi()).ok()
    }
    pub unsafe fn IsRealTime(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsRealTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetWriterTime(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWriterTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetStatistics(&self, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatistics)(windows_core::Interface::as_raw(self), wstreamnum, pstats).ok()
    }
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSyncTolerance)(windows_core::Interface::as_raw(self), mswindow).ok()
    }
    pub unsafe fn GetSyncTolerance(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSyncTolerance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWMWriterAdvanced_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSinkCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteStreamSample: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u64, u32, u64, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLiveSource: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsRealTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetWriterTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut WM_WRITER_STATISTICS) -> windows_core::HRESULT,
    pub SetSyncTolerance: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetSyncTolerance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMWriterAdvanced2, IWMWriterAdvanced2_Vtbl, 0x962dc1ec_c046_4db8_9cc7_26ceae500817);
impl core::ops::Deref for IWMWriterAdvanced2 {
    type Target = IWMWriterAdvanced;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterAdvanced2, windows_core::IUnknown, IWMWriterAdvanced);
impl IWMWriterAdvanced2 {
    pub unsafe fn GetInputSetting<P0>(&self, dwinputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetInputSetting)(windows_core::Interface::as_raw(self), dwinputnum, pszname.param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetInputSetting<P0>(&self, dwinputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetInputSetting)(windows_core::Interface::as_raw(self), dwinputnum, pszname.param().abi(), r#type, core::mem::transmute(pvalue.as_ptr()), pvalue.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct IWMWriterAdvanced2_Vtbl {
    pub base__: IWMWriterAdvanced_Vtbl,
    pub GetInputSetting: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut u8, *mut u16) -> windows_core::HRESULT,
    pub SetInputSetting: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const u8, u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMWriterAdvanced3, IWMWriterAdvanced3_Vtbl, 0x2cd6492d_7c37_4e76_9d3b_59261183a22e);
impl core::ops::Deref for IWMWriterAdvanced3 {
    type Target = IWMWriterAdvanced2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterAdvanced3, windows_core::IUnknown, IWMWriterAdvanced, IWMWriterAdvanced2);
impl IWMWriterAdvanced3 {
    pub unsafe fn GetStatisticsEx(&self, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatisticsEx)(windows_core::Interface::as_raw(self), wstreamnum, pstats).ok()
    }
    pub unsafe fn SetNonBlocking(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNonBlocking)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMWriterAdvanced3_Vtbl {
    pub base__: IWMWriterAdvanced2_Vtbl,
    pub GetStatisticsEx: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut WM_WRITER_STATISTICS_EX) -> windows_core::HRESULT,
    pub SetNonBlocking: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMWriterFileSink, IWMWriterFileSink_Vtbl, 0x96406be5_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMWriterFileSink {
    type Target = IWMWriterSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterFileSink, windows_core::IUnknown, IWMWriterSink);
impl IWMWriterFileSink {
    pub unsafe fn Open<P0>(&self, pwszfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pwszfilename.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWMWriterFileSink_Vtbl {
    pub base__: IWMWriterSink_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMWriterFileSink2, IWMWriterFileSink2_Vtbl, 0x14282ba7_4aef_4205_8ce5_c229035a05bc);
impl core::ops::Deref for IWMWriterFileSink2 {
    type Target = IWMWriterFileSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterFileSink2, windows_core::IUnknown, IWMWriterSink, IWMWriterFileSink);
impl IWMWriterFileSink2 {
    pub unsafe fn Start(&self, cnsstarttime: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), cnsstarttime).ok()
    }
    pub unsafe fn Stop(&self, cnsstoptime: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self), cnsstoptime).ok()
    }
    pub unsafe fn IsStopped(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsStopped)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFileDuration(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFileSize(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsClosed(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsClosed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWMWriterFileSink2_Vtbl {
    pub base__: IWMWriterFileSink_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub IsStopped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetFileDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetFileSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsClosed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMWriterFileSink3, IWMWriterFileSink3_Vtbl, 0x3fea4feb_2945_47a7_a1dd_c53a8fc4c45c);
impl core::ops::Deref for IWMWriterFileSink3 {
    type Target = IWMWriterFileSink2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterFileSink3, windows_core::IUnknown, IWMWriterSink, IWMWriterFileSink, IWMWriterFileSink2);
impl IWMWriterFileSink3 {
    pub unsafe fn SetAutoIndexing<P0>(&self, fdoautoindexing: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAutoIndexing)(windows_core::Interface::as_raw(self), fdoautoindexing.param().abi()).ok()
    }
    pub unsafe fn GetAutoIndexing(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAutoIndexing)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetControlStream<P0>(&self, wstreamnumber: u16, fshouldcontrolstartandstop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetControlStream)(windows_core::Interface::as_raw(self), wstreamnumber, fshouldcontrolstartandstop.param().abi()).ok()
    }
    pub unsafe fn GetMode(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn OnDataUnitEx(&self, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnDataUnitEx)(windows_core::Interface::as_raw(self), pfilesinkdataunit).ok()
    }
    pub unsafe fn SetUnbufferedIO<P0, P1>(&self, funbufferedio: P0, frestrictmemusage: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetUnbufferedIO)(windows_core::Interface::as_raw(self), funbufferedio.param().abi(), frestrictmemusage.param().abi()).ok()
    }
    pub unsafe fn GetUnbufferedIO(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUnbufferedIO)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CompleteOperations(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CompleteOperations)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMWriterFileSink3_Vtbl {
    pub base__: IWMWriterFileSink2_Vtbl,
    pub SetAutoIndexing: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetAutoIndexing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetControlStream: unsafe extern "system" fn(*mut core::ffi::c_void, u16, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub OnDataUnitEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const WMT_FILESINK_DATA_UNIT) -> windows_core::HRESULT,
    pub SetUnbufferedIO: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetUnbufferedIO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CompleteOperations: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMWriterNetworkSink, IWMWriterNetworkSink_Vtbl, 0x96406be7_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMWriterNetworkSink {
    type Target = IWMWriterSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterNetworkSink, windows_core::IUnknown, IWMWriterSink);
impl IWMWriterNetworkSink {
    pub unsafe fn SetMaximumClients(&self, dwmaxclients: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaximumClients)(windows_core::Interface::as_raw(self), dwmaxclients).ok()
    }
    pub unsafe fn GetMaximumClients(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaximumClients)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNetworkProtocol(&self, protocol: WMT_NET_PROTOCOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNetworkProtocol)(windows_core::Interface::as_raw(self), protocol).ok()
    }
    pub unsafe fn GetNetworkProtocol(&self) -> windows_core::Result<WMT_NET_PROTOCOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetworkProtocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetHostURL(&self, pwszurl: windows_core::PWSTR, pcchurl: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetHostURL)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszurl), pcchurl).ok()
    }
    pub unsafe fn Open(&self, pdwportnum: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pdwportnum).ok()
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMWriterNetworkSink_Vtbl {
    pub base__: IWMWriterSink_Vtbl,
    pub SetMaximumClients: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaximumClients: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetNetworkProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_NET_PROTOCOL) -> windows_core::HRESULT,
    pub GetNetworkProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMT_NET_PROTOCOL) -> windows_core::HRESULT,
    pub GetHostURL: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMWriterPostView, IWMWriterPostView_Vtbl, 0x81e20ce4_75ef_491a_8004_fc53c45bdc3e);
impl core::ops::Deref for IWMWriterPostView {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterPostView, windows_core::IUnknown);
impl IWMWriterPostView {
    pub unsafe fn SetPostViewCallback<P0>(&self, pcallback: P0, pvcontext: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMWriterPostViewCallback>,
    {
        (windows_core::Interface::vtable(self).SetPostViewCallback)(windows_core::Interface::as_raw(self), pcallback.param().abi(), pvcontext).ok()
    }
    pub unsafe fn SetReceivePostViewSamples<P0>(&self, wstreamnum: u16, freceivepostviewsamples: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetReceivePostViewSamples)(windows_core::Interface::as_raw(self), wstreamnum, freceivepostviewsamples.param().abi()).ok()
    }
    pub unsafe fn GetReceivePostViewSamples(&self, wstreamnum: u16) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReceivePostViewSamples)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
    }
    pub unsafe fn GetPostViewProps(&self, wstreamnumber: u16) -> windows_core::Result<IWMMediaProps> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPostViewProps)(windows_core::Interface::as_raw(self), wstreamnumber, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPostViewProps<P0>(&self, wstreamnumber: u16, poutput: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMMediaProps>,
    {
        (windows_core::Interface::vtable(self).SetPostViewProps)(windows_core::Interface::as_raw(self), wstreamnumber, poutput.param().abi()).ok()
    }
    pub unsafe fn GetPostViewFormatCount(&self, wstreamnumber: u16) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPostViewFormatCount)(windows_core::Interface::as_raw(self), wstreamnumber, &mut result__).map(|| result__)
    }
    pub unsafe fn GetPostViewFormat(&self, wstreamnumber: u16, dwformatnumber: u32) -> windows_core::Result<IWMMediaProps> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPostViewFormat)(windows_core::Interface::as_raw(self), wstreamnumber, dwformatnumber, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAllocateForPostView<P0>(&self, wstreamnumber: u16, fallocate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllocateForPostView)(windows_core::Interface::as_raw(self), wstreamnumber, fallocate.param().abi()).ok()
    }
    pub unsafe fn GetAllocateForPostView(&self, wstreamnumber: u16) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllocateForPostView)(windows_core::Interface::as_raw(self), wstreamnumber, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWMWriterPostView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPostViewCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReceivePostViewSamples: unsafe extern "system" fn(*mut core::ffi::c_void, u16, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetReceivePostViewSamples: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetPostViewProps: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPostViewProps: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPostViewFormatCount: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut u32) -> windows_core::HRESULT,
    pub GetPostViewFormat: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAllocateForPostView: unsafe extern "system" fn(*mut core::ffi::c_void, u16, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetAllocateForPostView: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMWriterPostViewCallback, IWMWriterPostViewCallback_Vtbl, 0xd9d6549d_a193_4f24_b308_03123d9b7f8d);
impl core::ops::Deref for IWMWriterPostViewCallback {
    type Target = IWMStatusCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterPostViewCallback, windows_core::IUnknown, IWMStatusCallback);
impl IWMWriterPostViewCallback {
    pub unsafe fn OnPostViewSample<P0>(&self, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: P0, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INSSBuffer>,
    {
        (windows_core::Interface::vtable(self).OnPostViewSample)(windows_core::Interface::as_raw(self), wstreamnumber, cnssampletime, cnssampleduration, dwflags, psample.param().abi(), pvcontext).ok()
    }
    pub unsafe fn AllocateForPostView(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut Option<INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AllocateForPostView)(windows_core::Interface::as_raw(self), wstreamnum, cbbuffer, core::mem::transmute(ppbuffer), pvcontext).ok()
    }
}
#[repr(C)]
pub struct IWMWriterPostViewCallback_Vtbl {
    pub base__: IWMStatusCallback_Vtbl,
    pub OnPostViewSample: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u64, u64, u32, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub AllocateForPostView: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u32, *mut *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMWriterPreprocess, IWMWriterPreprocess_Vtbl, 0xfc54a285_38c4_45b5_aa23_85b9f7cb424b);
impl core::ops::Deref for IWMWriterPreprocess {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterPreprocess, windows_core::IUnknown);
impl IWMWriterPreprocess {
    pub unsafe fn GetMaxPreprocessingPasses(&self, dwinputnum: u32, dwflags: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxPreprocessingPasses)(windows_core::Interface::as_raw(self), dwinputnum, dwflags, &mut result__).map(|| result__)
    }
    pub unsafe fn SetNumPreprocessingPasses(&self, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNumPreprocessingPasses)(windows_core::Interface::as_raw(self), dwinputnum, dwflags, dwnumpasses).ok()
    }
    pub unsafe fn BeginPreprocessingPass(&self, dwinputnum: u32, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginPreprocessingPass)(windows_core::Interface::as_raw(self), dwinputnum, dwflags).ok()
    }
    pub unsafe fn PreprocessSample<P0>(&self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INSSBuffer>,
    {
        (windows_core::Interface::vtable(self).PreprocessSample)(windows_core::Interface::as_raw(self), dwinputnum, cnssampletime, dwflags, psample.param().abi()).ok()
    }
    pub unsafe fn EndPreprocessingPass(&self, dwinputnum: u32, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndPreprocessingPass)(windows_core::Interface::as_raw(self), dwinputnum, dwflags).ok()
    }
}
#[repr(C)]
pub struct IWMWriterPreprocess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMaxPreprocessingPasses: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub SetNumPreprocessingPasses: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub BeginPreprocessingPass: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub PreprocessSample: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u64, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndPreprocessingPass: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMWriterPushSink, IWMWriterPushSink_Vtbl, 0xdc10e6a5_072c_467d_bf57_6330a9dde12a);
impl core::ops::Deref for IWMWriterPushSink {
    type Target = IWMWriterSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterPushSink, windows_core::IUnknown, IWMWriterSink);
impl IWMWriterPushSink {
    pub unsafe fn Connect<P0, P1, P2>(&self, pwszurl: P0, pwsztemplateurl: P1, fautodestroy: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), pwsztemplateurl.param().abi(), fautodestroy.param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndSession(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndSession)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMWriterPushSink_Vtbl {
    pub base__: IWMWriterSink_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndSession: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMWriterSink, IWMWriterSink_Vtbl, 0x96406be4_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMWriterSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterSink, windows_core::IUnknown);
impl IWMWriterSink {
    pub unsafe fn OnHeader<P0>(&self, pheader: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INSSBuffer>,
    {
        (windows_core::Interface::vtable(self).OnHeader)(windows_core::Interface::as_raw(self), pheader.param().abi()).ok()
    }
    pub unsafe fn IsRealTime(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsRealTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> windows_core::Result<INSSBuffer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllocateDataUnit)(windows_core::Interface::as_raw(self), cbdataunit, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OnDataUnit<P0>(&self, pdataunit: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INSSBuffer>,
    {
        (windows_core::Interface::vtable(self).OnDataUnit)(windows_core::Interface::as_raw(self), pdataunit.param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnEndWriting)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMWriterSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsRealTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub AllocateDataUnit: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDataUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEndWriting: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const AM_CONFIGASFWRITER_PARAM_AUTOINDEX: _AM_ASFWRITERCONFIG_PARAM = _AM_ASFWRITERCONFIG_PARAM(1i32);
pub const AM_CONFIGASFWRITER_PARAM_DONTCOMPRESS: _AM_ASFWRITERCONFIG_PARAM = _AM_ASFWRITERCONFIG_PARAM(3i32);
pub const AM_CONFIGASFWRITER_PARAM_MULTIPASS: _AM_ASFWRITERCONFIG_PARAM = _AM_ASFWRITERCONFIG_PARAM(2i32);
pub const CLSID_ClientNetManager: windows_core::GUID = windows_core::GUID::from_u128(0xcd12a3ce_9c42_11d2_beed_0060082f2054);
pub const CLSID_WMBandwidthSharing_Exclusive: windows_core::GUID = windows_core::GUID::from_u128(0xaf6060aa_5197_11d2_b6af_00c04fd908e9);
pub const CLSID_WMBandwidthSharing_Partial: windows_core::GUID = windows_core::GUID::from_u128(0xaf6060ab_5197_11d2_b6af_00c04fd908e9);
pub const CLSID_WMMUTEX_Bitrate: windows_core::GUID = windows_core::GUID::from_u128(0xd6e22a01_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Language: windows_core::GUID = windows_core::GUID::from_u128(0xd6e22a00_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Presentation: windows_core::GUID = windows_core::GUID::from_u128(0xd6e22a02_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Unknown: windows_core::GUID = windows_core::GUID::from_u128(0xd6e22a03_35da_11d1_9034_00a0c90349be);
pub const DRM_OPL_TYPES: u32 = 1u32;
pub const NETSOURCE_URLCREDPOLICY_SETTING_ANONYMOUSONLY: NETSOURCE_URLCREDPOLICY_SETTINGS = NETSOURCE_URLCREDPOLICY_SETTINGS(2i32);
pub const NETSOURCE_URLCREDPOLICY_SETTING_MUSTPROMPTUSER: NETSOURCE_URLCREDPOLICY_SETTINGS = NETSOURCE_URLCREDPOLICY_SETTINGS(1i32);
pub const NETSOURCE_URLCREDPOLICY_SETTING_SILENTLOGONOK: NETSOURCE_URLCREDPOLICY_SETTINGS = NETSOURCE_URLCREDPOLICY_SETTINGS(0i32);
pub const WEBSTREAM_SAMPLE_TYPE_FILE: WEBSTREAM_SAMPLE_TYPE = WEBSTREAM_SAMPLE_TYPE(1i32);
pub const WEBSTREAM_SAMPLE_TYPE_RENDER: WEBSTREAM_SAMPLE_TYPE = WEBSTREAM_SAMPLE_TYPE(2i32);
pub const WMDRM_IMPORT_INIT_STRUCT_DEFINED: u32 = 1u32;
pub const WMFORMAT_MPEG2Video: windows_core::GUID = windows_core::GUID::from_u128(0xe06d80e3_db46_11cf_b4d1_00805f6cbbea);
pub const WMFORMAT_Script: windows_core::GUID = windows_core::GUID::from_u128(0x5c8510f2_debe_4ca7_bba5_f07a104f8dff);
pub const WMFORMAT_VideoInfo: windows_core::GUID = windows_core::GUID::from_u128(0x05589f80_c356_11ce_bf01_00aa0055595a);
pub const WMFORMAT_WaveFormatEx: windows_core::GUID = windows_core::GUID::from_u128(0x05589f81_c356_11ce_bf01_00aa0055595a);
pub const WMFORMAT_WebStream: windows_core::GUID = windows_core::GUID::from_u128(0xda1e6b13_8359_4050_b398_388e965bf00c);
pub const WMMEDIASUBTYPE_ACELPnet: windows_core::GUID = windows_core::GUID::from_u128(0x00000130_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_Base: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_DRM: windows_core::GUID = windows_core::GUID::from_u128(0x00000009_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_I420: windows_core::GUID = windows_core::GUID::from_u128(0x30323449_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_IYUV: windows_core::GUID = windows_core::GUID::from_u128(0x56555949_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_M4S2: windows_core::GUID = windows_core::GUID::from_u128(0x3253344d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP3: windows_core::GUID = windows_core::GUID::from_u128(0x00000055_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP43: windows_core::GUID = windows_core::GUID::from_u128(0x3334504d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP4S: windows_core::GUID = windows_core::GUID::from_u128(0x5334504d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MPEG2_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8026_db46_11cf_b4d1_00805f6cbbea);
pub const WMMEDIASUBTYPE_MSS1: windows_core::GUID = windows_core::GUID::from_u128(0x3153534d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MSS2: windows_core::GUID = windows_core::GUID::from_u128(0x3253534d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_P422: windows_core::GUID = windows_core::GUID::from_u128(0x32323450_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_PCM: windows_core::GUID = windows_core::GUID::from_u128(0x00000001_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_RGB1: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb78_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB24: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb7d_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB32: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb7e_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB4: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb79_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB555: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb7c_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB565: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb7b_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB8: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb7a_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_UYVY: windows_core::GUID = windows_core::GUID::from_u128(0x59565955_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_VIDEOIMAGE: windows_core::GUID = windows_core::GUID::from_u128(0x1d4a45f2_e5f6_4b44_8388_f0ae5c0e0c37);
pub const WMMEDIASUBTYPE_WMAudioV2: windows_core::GUID = windows_core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV7: windows_core::GUID = windows_core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV8: windows_core::GUID = windows_core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV9: windows_core::GUID = windows_core::GUID::from_u128(0x00000162_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudio_Lossless: windows_core::GUID = windows_core::GUID::from_u128(0x00000163_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMSP1: windows_core::GUID = windows_core::GUID::from_u128(0x0000000a_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMSP2: windows_core::GUID = windows_core::GUID::from_u128(0x0000000b_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV1: windows_core::GUID = windows_core::GUID::from_u128(0x31564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV2: windows_core::GUID = windows_core::GUID::from_u128(0x32564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV3: windows_core::GUID = windows_core::GUID::from_u128(0x33564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMVA: windows_core::GUID = windows_core::GUID::from_u128(0x41564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMVP: windows_core::GUID = windows_core::GUID::from_u128(0x50564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WVC1: windows_core::GUID = windows_core::GUID::from_u128(0x31435657_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WVP2: windows_core::GUID = windows_core::GUID::from_u128(0x32505657_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WebStream: windows_core::GUID = windows_core::GUID::from_u128(0x776257d4_c627_41cb_8f81_7ac7ff1c40cc);
pub const WMMEDIASUBTYPE_YUY2: windows_core::GUID = windows_core::GUID::from_u128(0x32595559_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YV12: windows_core::GUID = windows_core::GUID::from_u128(0x32315659_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YVU9: windows_core::GUID = windows_core::GUID::from_u128(0x39555659_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YVYU: windows_core::GUID = windows_core::GUID::from_u128(0x55595659_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_Audio: windows_core::GUID = windows_core::GUID::from_u128(0x73647561_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_FileTransfer: windows_core::GUID = windows_core::GUID::from_u128(0xd9e47579_930e_4427_adfc_ad80f290e470);
pub const WMMEDIATYPE_Image: windows_core::GUID = windows_core::GUID::from_u128(0x34a50fd8_8aa5_4386_81fe_a0efe0488e31);
pub const WMMEDIATYPE_Script: windows_core::GUID = windows_core::GUID::from_u128(0x73636d64_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_Text: windows_core::GUID = windows_core::GUID::from_u128(0x9bba1ea7_5ab2_4829_ba57_0940209bcf3e);
pub const WMMEDIATYPE_Video: windows_core::GUID = windows_core::GUID::from_u128(0x73646976_0000_0010_8000_00aa00389b71);
pub const WMSCRIPTTYPE_TwoStrings: windows_core::GUID = windows_core::GUID::from_u128(0x82f38a70_c29f_11d1_97ad_00a0c95ea850);
pub const WMT_ACQUIRE_LICENSE: WMT_STATUS = WMT_STATUS(23i32);
pub const WMT_BACKUPRESTORE_BEGIN: WMT_STATUS = WMT_STATUS(21i32);
pub const WMT_BACKUPRESTORE_CONNECTING: WMT_STATUS = WMT_STATUS(28i32);
pub const WMT_BACKUPRESTORE_DISCONNECTING: WMT_STATUS = WMT_STATUS(29i32);
pub const WMT_BACKUPRESTORE_END: WMT_STATUS = WMT_STATUS(27i32);
pub const WMT_BUFFERING_START: WMT_STATUS = WMT_STATUS(2i32);
pub const WMT_BUFFERING_STOP: WMT_STATUS = WMT_STATUS(3i32);
pub const WMT_CLEANPOINT_ONLY: WMT_STREAM_SELECTION = WMT_STREAM_SELECTION(1i32);
pub const WMT_CLIENT_CONNECT: WMT_STATUS = WMT_STATUS(32i32);
pub const WMT_CLIENT_CONNECT_EX: WMT_STATUS = WMT_STATUS(37i32);
pub const WMT_CLIENT_DISCONNECT: WMT_STATUS = WMT_STATUS(33i32);
pub const WMT_CLIENT_DISCONNECT_EX: WMT_STATUS = WMT_STATUS(38i32);
pub const WMT_CLIENT_PROPERTIES: WMT_STATUS = WMT_STATUS(42i32);
pub const WMT_CLOSED: WMT_STATUS = WMT_STATUS(13i32);
pub const WMT_CODECINFO_AUDIO: WMT_CODEC_INFO_TYPE = WMT_CODEC_INFO_TYPE(0i32);
pub const WMT_CODECINFO_UNKNOWN: WMT_CODEC_INFO_TYPE = WMT_CODEC_INFO_TYPE(-1i32);
pub const WMT_CODECINFO_VIDEO: WMT_CODEC_INFO_TYPE = WMT_CODEC_INFO_TYPE(1i32);
pub const WMT_CONNECTING: WMT_STATUS = WMT_STATUS(8i32);
pub const WMT_CONTENT_ENABLER: WMT_STATUS = WMT_STATUS(51i32);
pub const WMT_CREDENTIAL_CLEAR_TEXT: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(4i32);
pub const WMT_CREDENTIAL_DONT_CACHE: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(2i32);
pub const WMT_CREDENTIAL_ENCRYPT: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(16i32);
pub const WMT_CREDENTIAL_PROXY: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(8i32);
pub const WMT_CREDENTIAL_SAVE: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(1i32);
pub const WMT_DMOCATEGORY_AUDIO_WATERMARK: windows_core::GUID = windows_core::GUID::from_u128(0x65221c5a_fa75_4b39_b50c_06c336b6a3ef);
pub const WMT_DMOCATEGORY_VIDEO_WATERMARK: windows_core::GUID = windows_core::GUID::from_u128(0x187cc922_8efc_4404_9daf_63f4830df1bc);
pub const WMT_DRMLA_TAMPERED: WMT_DRMLA_TRUST = WMT_DRMLA_TRUST(2i32);
pub const WMT_DRMLA_TRUSTED: WMT_DRMLA_TRUST = WMT_DRMLA_TRUST(1i32);
pub const WMT_DRMLA_UNTRUSTED: WMT_DRMLA_TRUST = WMT_DRMLA_TRUST(0i32);
pub const WMT_END_OF_FILE: WMT_STATUS = WMT_STATUS(4i32);
pub const WMT_END_OF_SEGMENT: WMT_STATUS = WMT_STATUS(5i32);
pub const WMT_END_OF_STREAMING: WMT_STATUS = WMT_STATUS(6i32);
pub const WMT_EOF: WMT_STATUS = WMT_STATUS(4i32);
pub const WMT_ERROR: WMT_STATUS = WMT_STATUS(0i32);
pub const WMT_ERROR_WITHURL: WMT_STATUS = WMT_STATUS(30i32);
pub const WMT_FM_FILESINK_DATA_UNITS: WMT_FILESINK_MODE = WMT_FILESINK_MODE(2i32);
pub const WMT_FM_FILESINK_UNBUFFERED: WMT_FILESINK_MODE = WMT_FILESINK_MODE(4i32);
pub const WMT_FM_SINGLE_BUFFERS: WMT_FILESINK_MODE = WMT_FILESINK_MODE(1i32);
pub const WMT_IMAGETYPE_BITMAP: WMT_ATTR_IMAGETYPE = WMT_ATTR_IMAGETYPE(1i32);
pub const WMT_IMAGETYPE_GIF: WMT_ATTR_IMAGETYPE = WMT_ATTR_IMAGETYPE(3i32);
pub const WMT_IMAGETYPE_JPEG: WMT_ATTR_IMAGETYPE = WMT_ATTR_IMAGETYPE(2i32);
pub const WMT_INDEX_PROGRESS: WMT_STATUS = WMT_STATUS(16i32);
pub const WMT_INDIVIDUALIZE: WMT_STATUS = WMT_STATUS(24i32);
pub const WMT_INIT_PLAYLIST_BURN: WMT_STATUS = WMT_STATUS(44i32);
pub const WMT_IT_BITMAP: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(1i32);
pub const WMT_IT_FRAME_NUMBERS: WMT_INDEXER_TYPE = WMT_INDEXER_TYPE(1i32);
pub const WMT_IT_GIF: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(3i32);
pub const WMT_IT_JPEG: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(2i32);
pub const WMT_IT_NEAREST_CLEAN_POINT: WMT_INDEX_TYPE = WMT_INDEX_TYPE(3i32);
pub const WMT_IT_NEAREST_DATA_UNIT: WMT_INDEX_TYPE = WMT_INDEX_TYPE(1i32);
pub const WMT_IT_NEAREST_OBJECT: WMT_INDEX_TYPE = WMT_INDEX_TYPE(2i32);
pub const WMT_IT_NONE: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(0i32);
pub const WMT_IT_PRESENTATION_TIME: WMT_INDEXER_TYPE = WMT_INDEXER_TYPE(0i32);
pub const WMT_IT_TIMECODE: WMT_INDEXER_TYPE = WMT_INDEXER_TYPE(2i32);
pub const WMT_LICENSEURL_SIGNATURE_STATE: WMT_STATUS = WMT_STATUS(43i32);
pub const WMT_LOCATING: WMT_STATUS = WMT_STATUS(7i32);
pub const WMT_MISSING_CODEC: WMT_STATUS = WMT_STATUS(10i32);
pub const WMT_MS_CLASS_MIXED: WMT_MUSICSPEECH_CLASS_MODE = WMT_MUSICSPEECH_CLASS_MODE(2i32);
pub const WMT_MS_CLASS_MUSIC: WMT_MUSICSPEECH_CLASS_MODE = WMT_MUSICSPEECH_CLASS_MODE(0i32);
pub const WMT_MS_CLASS_SPEECH: WMT_MUSICSPEECH_CLASS_MODE = WMT_MUSICSPEECH_CLASS_MODE(1i32);
pub const WMT_NATIVE_OUTPUT_PROPS_CHANGED: WMT_STATUS = WMT_STATUS(34i32);
pub const WMT_NEEDS_INDIVIDUALIZATION: WMT_STATUS = WMT_STATUS(25i32);
pub const WMT_NEW_METADATA: WMT_STATUS = WMT_STATUS(20i32);
pub const WMT_NEW_SOURCEFLAGS: WMT_STATUS = WMT_STATUS(19i32);
pub const WMT_NO_RIGHTS: WMT_STATUS = WMT_STATUS(9i32);
pub const WMT_NO_RIGHTS_EX: WMT_STATUS = WMT_STATUS(26i32);
pub const WMT_OFF: WMT_STREAM_SELECTION = WMT_STREAM_SELECTION(0i32);
pub const WMT_OFFSET_FORMAT_100NS: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(0i32);
pub const WMT_OFFSET_FORMAT_100NS_APPROXIMATE: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(4i32);
pub const WMT_OFFSET_FORMAT_FRAME_NUMBERS: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(1i32);
pub const WMT_OFFSET_FORMAT_PLAYLIST_OFFSET: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(2i32);
pub const WMT_OFFSET_FORMAT_TIMECODE: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(3i32);
pub const WMT_ON: WMT_STREAM_SELECTION = WMT_STREAM_SELECTION(2i32);
pub const WMT_OPENED: WMT_STATUS = WMT_STATUS(1i32);
pub const WMT_PLAY_MODE_AUTOSELECT: WMT_PLAY_MODE = WMT_PLAY_MODE(0i32);
pub const WMT_PLAY_MODE_DOWNLOAD: WMT_PLAY_MODE = WMT_PLAY_MODE(2i32);
pub const WMT_PLAY_MODE_LOCAL: WMT_PLAY_MODE = WMT_PLAY_MODE(1i32);
pub const WMT_PLAY_MODE_STREAMING: WMT_PLAY_MODE = WMT_PLAY_MODE(3i32);
pub const WMT_PREROLL_COMPLETE: WMT_STATUS = WMT_STATUS(41i32);
pub const WMT_PREROLL_READY: WMT_STATUS = WMT_STATUS(40i32);
pub const WMT_PROTOCOL_HTTP: WMT_NET_PROTOCOL = WMT_NET_PROTOCOL(0i32);
pub const WMT_PROXIMITY_COMPLETED: WMT_STATUS = WMT_STATUS(50i32);
pub const WMT_PROXIMITY_RESULT: WMT_STATUS = WMT_STATUS(49i32);
pub const WMT_PROXY_SETTING_AUTO: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(2i32);
pub const WMT_PROXY_SETTING_BROWSER: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(3i32);
pub const WMT_PROXY_SETTING_MANUAL: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(1i32);
pub const WMT_PROXY_SETTING_MAX: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(4i32);
pub const WMT_PROXY_SETTING_NONE: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(0i32);
pub const WMT_RECONNECT_END: WMT_STATUS = WMT_STATUS(36i32);
pub const WMT_RECONNECT_START: WMT_STATUS = WMT_STATUS(35i32);
pub const WMT_RESTRICTED_LICENSE: WMT_STATUS = WMT_STATUS(31i32);
pub const WMT_RIGHT_COLLABORATIVE_PLAY: WMT_RIGHTS = WMT_RIGHTS(256i32);
pub const WMT_RIGHT_COPY: WMT_RIGHTS = WMT_RIGHTS(128i32);
pub const WMT_RIGHT_COPY_TO_CD: WMT_RIGHTS = WMT_RIGHTS(8i32);
pub const WMT_RIGHT_COPY_TO_NON_SDMI_DEVICE: WMT_RIGHTS = WMT_RIGHTS(2i32);
pub const WMT_RIGHT_COPY_TO_SDMI_DEVICE: WMT_RIGHTS = WMT_RIGHTS(16i32);
pub const WMT_RIGHT_ONE_TIME: WMT_RIGHTS = WMT_RIGHTS(32i32);
pub const WMT_RIGHT_PLAYBACK: WMT_RIGHTS = WMT_RIGHTS(1i32);
pub const WMT_RIGHT_SAVE_STREAM_PROTECTED: WMT_RIGHTS = WMT_RIGHTS(64i32);
pub const WMT_RIGHT_SDMI_NOMORECOPIES: WMT_RIGHTS = WMT_RIGHTS(131072i32);
pub const WMT_RIGHT_SDMI_TRIGGER: WMT_RIGHTS = WMT_RIGHTS(65536i32);
pub const WMT_SAVEAS_START: WMT_STATUS = WMT_STATUS(17i32);
pub const WMT_SAVEAS_STOP: WMT_STATUS = WMT_STATUS(18i32);
pub const WMT_SET_FEC_SPAN: WMT_STATUS = WMT_STATUS(39i32);
pub const WMT_SOURCE_SWITCH: WMT_STATUS = WMT_STATUS(22i32);
pub const WMT_STARTED: WMT_STATUS = WMT_STATUS(11i32);
pub const WMT_STOPPED: WMT_STATUS = WMT_STATUS(12i32);
pub const WMT_STRIDING: WMT_STATUS = WMT_STATUS(14i32);
pub const WMT_Storage_Format_MP3: WMT_STORAGE_FORMAT = WMT_STORAGE_FORMAT(0i32);
pub const WMT_Storage_Format_V1: WMT_STORAGE_FORMAT = WMT_STORAGE_FORMAT(1i32);
pub const WMT_TIMECODE_FRAMERATE_24: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(3i32);
pub const WMT_TIMECODE_FRAMERATE_25: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(2i32);
pub const WMT_TIMECODE_FRAMERATE_30: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(0i32);
pub const WMT_TIMECODE_FRAMERATE_30DROP: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(1i32);
pub const WMT_TIMER: WMT_STATUS = WMT_STATUS(15i32);
pub const WMT_TRANSCRYPTOR_CLOSED: WMT_STATUS = WMT_STATUS(48i32);
pub const WMT_TRANSCRYPTOR_INIT: WMT_STATUS = WMT_STATUS(45i32);
pub const WMT_TRANSCRYPTOR_READ: WMT_STATUS = WMT_STATUS(47i32);
pub const WMT_TRANSCRYPTOR_SEEKED: WMT_STATUS = WMT_STATUS(46i32);
pub const WMT_TYPE_BINARY: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(2i32);
pub const WMT_TYPE_BOOL: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(3i32);
pub const WMT_TYPE_DWORD: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(0i32);
pub const WMT_TYPE_GUID: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(6i32);
pub const WMT_TYPE_QWORD: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(4i32);
pub const WMT_TYPE_STRING: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(1i32);
pub const WMT_TYPE_WORD: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(5i32);
pub const WMT_Transport_Type_Reliable: WMT_TRANSPORT_TYPE = WMT_TRANSPORT_TYPE(1i32);
pub const WMT_Transport_Type_Unreliable: WMT_TRANSPORT_TYPE = WMT_TRANSPORT_TYPE(0i32);
pub const WMT_VER_4_0: WMT_VERSION = WMT_VERSION(262144i32);
pub const WMT_VER_7_0: WMT_VERSION = WMT_VERSION(458752i32);
pub const WMT_VER_8_0: WMT_VERSION = WMT_VERSION(524288i32);
pub const WMT_VER_9_0: WMT_VERSION = WMT_VERSION(589824i32);
pub const WMT_VIDEOIMAGE_INTEGER_DENOMINATOR: i32 = 65536i32;
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER: u32 = 491406834u32;
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER_2: u32 = 491406835u32;
pub const WMT_VIDEOIMAGE_SAMPLE_ADV_BLENDING: u32 = 8u32;
pub const WMT_VIDEOIMAGE_SAMPLE_BLENDING: u32 = 4u32;
pub const WMT_VIDEOIMAGE_SAMPLE_INPUT_FRAME: u32 = 1u32;
pub const WMT_VIDEOIMAGE_SAMPLE_MOTION: u32 = 1u32;
pub const WMT_VIDEOIMAGE_SAMPLE_OUTPUT_FRAME: u32 = 2u32;
pub const WMT_VIDEOIMAGE_SAMPLE_ROTATION: u32 = 2u32;
pub const WMT_VIDEOIMAGE_SAMPLE_USES_CURRENT_INPUT_FRAME: u32 = 4u32;
pub const WMT_VIDEOIMAGE_SAMPLE_USES_PREVIOUS_INPUT_FRAME: u32 = 8u32;
pub const WMT_VIDEOIMAGE_TRANSITION_BOW_TIE: u32 = 11u32;
pub const WMT_VIDEOIMAGE_TRANSITION_CIRCLE: u32 = 12u32;
pub const WMT_VIDEOIMAGE_TRANSITION_CROSS_FADE: u32 = 13u32;
pub const WMT_VIDEOIMAGE_TRANSITION_DIAGONAL: u32 = 14u32;
pub const WMT_VIDEOIMAGE_TRANSITION_DIAMOND: u32 = 15u32;
pub const WMT_VIDEOIMAGE_TRANSITION_FADE_TO_COLOR: u32 = 16u32;
pub const WMT_VIDEOIMAGE_TRANSITION_FILLED_V: u32 = 17u32;
pub const WMT_VIDEOIMAGE_TRANSITION_FLIP: u32 = 18u32;
pub const WMT_VIDEOIMAGE_TRANSITION_INSET: u32 = 19u32;
pub const WMT_VIDEOIMAGE_TRANSITION_IRIS: u32 = 20u32;
pub const WMT_VIDEOIMAGE_TRANSITION_PAGE_ROLL: u32 = 21u32;
pub const WMT_VIDEOIMAGE_TRANSITION_RECTANGLE: u32 = 23u32;
pub const WMT_VIDEOIMAGE_TRANSITION_REVEAL: u32 = 24u32;
pub const WMT_VIDEOIMAGE_TRANSITION_SLIDE: u32 = 27u32;
pub const WMT_VIDEOIMAGE_TRANSITION_SPLIT: u32 = 29u32;
pub const WMT_VIDEOIMAGE_TRANSITION_STAR: u32 = 30u32;
pub const WMT_VIDEOIMAGE_TRANSITION_WHEEL: u32 = 31u32;
pub const WMT_WMETYPE_AUDIO: WMT_WATERMARK_ENTRY_TYPE = WMT_WATERMARK_ENTRY_TYPE(1i32);
pub const WMT_WMETYPE_VIDEO: WMT_WATERMARK_ENTRY_TYPE = WMT_WATERMARK_ENTRY_TYPE(2i32);
pub const WM_AETYPE_EXCLUDE: WM_AETYPE = WM_AETYPE(101i32);
pub const WM_AETYPE_INCLUDE: WM_AETYPE = WM_AETYPE(105i32);
pub const WM_CL_INTERLACED420: u32 = 0u32;
pub const WM_CL_PROGRESSIVE420: u32 = 1u32;
pub const WM_CT_BOTTOM_FIELD_FIRST: u32 = 32u32;
pub const WM_CT_INTERLACED: u32 = 128u32;
pub const WM_CT_REPEAT_FIRST_FIELD: u32 = 16u32;
pub const WM_CT_TOP_FIELD_FIRST: u32 = 64u32;
pub const WM_DM_DEINTERLACE_HALFSIZE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(2i32);
pub const WM_DM_DEINTERLACE_HALFSIZEDOUBLERATE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(3i32);
pub const WM_DM_DEINTERLACE_INVERSETELECINE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(4i32);
pub const WM_DM_DEINTERLACE_NORMAL: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(1i32);
pub const WM_DM_DEINTERLACE_VERTICALHALFSIZEDOUBLERATE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(5i32);
pub const WM_DM_IT_DISABLE_COHERENT_MODE: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(0i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_AA_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(6i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_AA_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(1i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BB_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(7i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BB_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(2i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BC_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(8i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BC_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(3i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_CD_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(9i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_CD_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(4i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_DD_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(10i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_DD_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(5i32);
pub const WM_DM_NOTINTERLACED: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(0i32);
pub const WM_MAX_STREAMS: u32 = 63u32;
pub const WM_MAX_VIDEO_STREAMS: u32 = 63u32;
pub const WM_PLAYBACK_DRC_HIGH: WM_PLAYBACK_DRC_LEVEL = WM_PLAYBACK_DRC_LEVEL(0i32);
pub const WM_PLAYBACK_DRC_LOW: WM_PLAYBACK_DRC_LEVEL = WM_PLAYBACK_DRC_LEVEL(2i32);
pub const WM_PLAYBACK_DRC_MEDIUM: WM_PLAYBACK_DRC_LEVEL = WM_PLAYBACK_DRC_LEVEL(1i32);
pub const WM_SFEX_DATALOSS: WM_SFEX_TYPE = WM_SFEX_TYPE(4i32);
pub const WM_SFEX_NOTASYNCPOINT: WM_SFEX_TYPE = WM_SFEX_TYPE(2i32);
pub const WM_SF_CLEANPOINT: WM_SF_TYPE = WM_SF_TYPE(1i32);
pub const WM_SF_DATALOSS: WM_SF_TYPE = WM_SF_TYPE(4i32);
pub const WM_SF_DISCONTINUITY: WM_SF_TYPE = WM_SF_TYPE(2i32);
pub const WM_SampleExtensionGUID_ChromaLocation: windows_core::GUID = windows_core::GUID::from_u128(0x4c5acca0_9276_4b2c_9e4c_a0edefdd217e);
pub const WM_SampleExtensionGUID_ColorSpaceInfo: windows_core::GUID = windows_core::GUID::from_u128(0xf79ada56_30eb_4f2b_9f7a_f24b139a1157);
pub const WM_SampleExtensionGUID_ContentType: windows_core::GUID = windows_core::GUID::from_u128(0xd590dc20_07bc_436c_9cf7_f3bbfbf1a4dc);
pub const WM_SampleExtensionGUID_FileName: windows_core::GUID = windows_core::GUID::from_u128(0xe165ec0e_19ed_45d7_b4a7_25cbd1e28e9b);
pub const WM_SampleExtensionGUID_OutputCleanPoint: windows_core::GUID = windows_core::GUID::from_u128(0xf72a3c6f_6eb4_4ebc_b192_09ad9759e828);
pub const WM_SampleExtensionGUID_PixelAspectRatio: windows_core::GUID = windows_core::GUID::from_u128(0x1b1ee554_f9ea_4bc8_821a_376b74e4c4b8);
pub const WM_SampleExtensionGUID_SampleDuration: windows_core::GUID = windows_core::GUID::from_u128(0xc6bd9450_867f_4907_83a3_c77921b733ad);
pub const WM_SampleExtensionGUID_SampleProtectionSalt: windows_core::GUID = windows_core::GUID::from_u128(0x5403deee_b9ee_438f_aa83_3804997e569d);
pub const WM_SampleExtensionGUID_Timecode: windows_core::GUID = windows_core::GUID::from_u128(0x399595ec_8667_4e2d_8fdb_98814ce76c1e);
pub const WM_SampleExtensionGUID_UserDataInfo: windows_core::GUID = windows_core::GUID::from_u128(0x732bb4fa_78be_4549_99bd_02db1a55b7a8);
pub const WM_SampleExtension_ChromaLocation_Size: u32 = 1u32;
pub const WM_SampleExtension_ColorSpaceInfo_Size: u32 = 3u32;
pub const WM_SampleExtension_ContentType_Size: u32 = 1u32;
pub const WM_SampleExtension_PixelAspectRatio_Size: u32 = 2u32;
pub const WM_SampleExtension_SampleDuration_Size: u32 = 2u32;
pub const WM_SampleExtension_Timecode_Size: u32 = 14u32;
pub const g_dwWMContentAttributes: u32 = 5u32;
pub const g_dwWMNSCAttributes: u32 = 5u32;
pub const g_dwWMSpecialAttributes: u32 = 20u32;
pub const g_wszASFLeakyBucketPairs: windows_core::PCWSTR = windows_core::w!("ASFLeakyBucketPairs");
pub const g_wszAllowInterlacedOutput: windows_core::PCWSTR = windows_core::w!("AllowInterlacedOutput");
pub const g_wszAverageLevel: windows_core::PCWSTR = windows_core::w!("AverageLevel");
pub const g_wszBufferAverage: windows_core::PCWSTR = windows_core::w!("Buffer Average");
pub const g_wszComplexity: windows_core::PCWSTR = windows_core::w!("_COMPLEXITYEX");
pub const g_wszComplexityLive: windows_core::PCWSTR = windows_core::w!("_COMPLEXITYEXLIVE");
pub const g_wszComplexityMax: windows_core::PCWSTR = windows_core::w!("_COMPLEXITYEXMAX");
pub const g_wszComplexityOffline: windows_core::PCWSTR = windows_core::w!("_COMPLEXITYEXOFFLINE");
pub const g_wszDecoderComplexityRequested: windows_core::PCWSTR = windows_core::w!("_DECODERCOMPLEXITYPROFILE");
pub const g_wszDedicatedDeliveryThread: windows_core::PCWSTR = windows_core::w!("DedicatedDeliveryThread");
pub const g_wszDeinterlaceMode: windows_core::PCWSTR = windows_core::w!("DeinterlaceMode");
pub const g_wszDeliverOnReceive: windows_core::PCWSTR = windows_core::w!("DeliverOnReceive");
pub const g_wszDeviceConformanceTemplate: windows_core::PCWSTR = windows_core::w!("DeviceConformanceTemplate");
pub const g_wszDynamicRangeControl: windows_core::PCWSTR = windows_core::w!("DynamicRangeControl");
pub const g_wszEDL: windows_core::PCWSTR = windows_core::w!("_EDL");
pub const g_wszEarlyDataDelivery: windows_core::PCWSTR = windows_core::w!("EarlyDataDelivery");
pub const g_wszEnableDiscreteOutput: windows_core::PCWSTR = windows_core::w!("EnableDiscreteOutput");
pub const g_wszEnableFrameInterpolation: windows_core::PCWSTR = windows_core::w!("EnableFrameInterpolation");
pub const g_wszEnableWMAProSPDIFOutput: windows_core::PCWSTR = windows_core::w!("EnableWMAProSPDIFOutput");
pub const g_wszFailSeekOnError: windows_core::PCWSTR = windows_core::w!("FailSeekOnError");
pub const g_wszFixedFrameRate: windows_core::PCWSTR = windows_core::w!("FixedFrameRate");
pub const g_wszFold6To2Channels3: windows_core::PCWSTR = windows_core::w!("Fold6To2Channels3");
pub const g_wszFoldToChannelsTemplate: windows_core::PCWSTR = windows_core::w!("Fold%luTo%luChannels%lu");
pub const g_wszInitialPatternForInverseTelecine: windows_core::PCWSTR = windows_core::w!("InitialPatternForInverseTelecine");
pub const g_wszInterlacedCoding: windows_core::PCWSTR = windows_core::w!("InterlacedCoding");
pub const g_wszIsVBRSupported: windows_core::PCWSTR = windows_core::w!("_ISVBRSUPPORTED");
pub const g_wszJPEGCompressionQuality: windows_core::PCWSTR = windows_core::w!("JPEGCompressionQuality");
pub const g_wszJustInTimeDecode: windows_core::PCWSTR = windows_core::w!("JustInTimeDecode");
pub const g_wszMixedClassMode: windows_core::PCWSTR = windows_core::w!("MixedClassMode");
pub const g_wszMusicClassMode: windows_core::PCWSTR = windows_core::w!("MusicClassMode");
pub const g_wszMusicSpeechClassMode: windows_core::PCWSTR = windows_core::w!("MusicSpeechClassMode");
pub const g_wszNeedsPreviousSample: windows_core::PCWSTR = windows_core::w!("NeedsPreviousSample");
pub const g_wszNumPasses: windows_core::PCWSTR = windows_core::w!("_PASSESUSED");
pub const g_wszOriginalSourceFormatTag: windows_core::PCWSTR = windows_core::w!("_SOURCEFORMATTAG");
pub const g_wszOriginalWaveFormat: windows_core::PCWSTR = windows_core::w!("_ORIGINALWAVEFORMAT");
pub const g_wszPeakValue: windows_core::PCWSTR = windows_core::w!("PeakValue");
pub const g_wszPermitSeeksBeyondEndOfStream: windows_core::PCWSTR = windows_core::w!("PermitSeeksBeyondEndOfStream");
pub const g_wszReloadIndexOnSeek: windows_core::PCWSTR = windows_core::w!("ReloadIndexOnSeek");
pub const g_wszScrambledAudio: windows_core::PCWSTR = windows_core::w!("ScrambledAudio");
pub const g_wszSingleOutputBuffer: windows_core::PCWSTR = windows_core::w!("SingleOutputBuffer");
pub const g_wszSoftwareScaling: windows_core::PCWSTR = windows_core::w!("SoftwareScaling");
pub const g_wszSourceBufferTime: windows_core::PCWSTR = windows_core::w!("SourceBufferTime");
pub const g_wszSourceMaxBytesAtOnce: windows_core::PCWSTR = windows_core::w!("SourceMaxBytesAtOnce");
pub const g_wszSpeakerConfig: windows_core::PCWSTR = windows_core::w!("SpeakerConfig");
pub const g_wszSpeechCaps: windows_core::PCWSTR = windows_core::w!("SpeechFormatCap");
pub const g_wszSpeechClassMode: windows_core::PCWSTR = windows_core::w!("SpeechClassMode");
pub const g_wszStreamLanguage: windows_core::PCWSTR = windows_core::w!("StreamLanguage");
pub const g_wszStreamNumIndexObjects: windows_core::PCWSTR = windows_core::w!("StreamNumIndexObjects");
pub const g_wszUsePacketAtSeekPoint: windows_core::PCWSTR = windows_core::w!("UsePacketAtSeekPoint");
pub const g_wszVBRBitrateMax: windows_core::PCWSTR = windows_core::w!("_RMAX");
pub const g_wszVBRBufferWindowMax: windows_core::PCWSTR = windows_core::w!("_BMAX");
pub const g_wszVBREnabled: windows_core::PCWSTR = windows_core::w!("_VBRENABLED");
pub const g_wszVBRPeak: windows_core::PCWSTR = windows_core::w!("VBR Peak");
pub const g_wszVBRQuality: windows_core::PCWSTR = windows_core::w!("_VBRQUALITY");
pub const g_wszVideoSampleDurations: windows_core::PCWSTR = windows_core::w!("VideoSampleDurations");
pub const g_wszWMADID: windows_core::PCWSTR = windows_core::w!("WM/ADID");
pub const g_wszWMASFPacketCount: windows_core::PCWSTR = windows_core::w!("WM/ASFPacketCount");
pub const g_wszWMASFSecurityObjectsSize: windows_core::PCWSTR = windows_core::w!("WM/ASFSecurityObjectsSize");
pub const g_wszWMAlbumArtist: windows_core::PCWSTR = windows_core::w!("WM/AlbumArtist");
pub const g_wszWMAlbumArtistSort: windows_core::PCWSTR = windows_core::w!("WM/AlbumArtistSort");
pub const g_wszWMAlbumCoverURL: windows_core::PCWSTR = windows_core::w!("WM/AlbumCoverURL");
pub const g_wszWMAlbumTitle: windows_core::PCWSTR = windows_core::w!("WM/AlbumTitle");
pub const g_wszWMAlbumTitleSort: windows_core::PCWSTR = windows_core::w!("WM/AlbumTitleSort");
pub const g_wszWMAspectRatioX: windows_core::PCWSTR = windows_core::w!("AspectRatioX");
pub const g_wszWMAspectRatioY: windows_core::PCWSTR = windows_core::w!("AspectRatioY");
pub const g_wszWMAudioFileURL: windows_core::PCWSTR = windows_core::w!("WM/AudioFileURL");
pub const g_wszWMAudioSourceURL: windows_core::PCWSTR = windows_core::w!("WM/AudioSourceURL");
pub const g_wszWMAuthor: windows_core::PCWSTR = windows_core::w!("Author");
pub const g_wszWMAuthorSort: windows_core::PCWSTR = windows_core::w!("AuthorSort");
pub const g_wszWMAuthorURL: windows_core::PCWSTR = windows_core::w!("WM/AuthorURL");
pub const g_wszWMBannerImageData: windows_core::PCWSTR = windows_core::w!("BannerImageData");
pub const g_wszWMBannerImageType: windows_core::PCWSTR = windows_core::w!("BannerImageType");
pub const g_wszWMBannerImageURL: windows_core::PCWSTR = windows_core::w!("BannerImageURL");
pub const g_wszWMBeatsPerMinute: windows_core::PCWSTR = windows_core::w!("WM/BeatsPerMinute");
pub const g_wszWMBitrate: windows_core::PCWSTR = windows_core::w!("Bitrate");
pub const g_wszWMBroadcast: windows_core::PCWSTR = windows_core::w!("Broadcast");
pub const g_wszWMCategory: windows_core::PCWSTR = windows_core::w!("WM/Category");
pub const g_wszWMCodec: windows_core::PCWSTR = windows_core::w!("WM/Codec");
pub const g_wszWMComposer: windows_core::PCWSTR = windows_core::w!("WM/Composer");
pub const g_wszWMComposerSort: windows_core::PCWSTR = windows_core::w!("WM/ComposerSort");
pub const g_wszWMConductor: windows_core::PCWSTR = windows_core::w!("WM/Conductor");
pub const g_wszWMContainerFormat: windows_core::PCWSTR = windows_core::w!("WM/ContainerFormat");
pub const g_wszWMContentDistributor: windows_core::PCWSTR = windows_core::w!("WM/ContentDistributor");
pub const g_wszWMContentGroupDescription: windows_core::PCWSTR = windows_core::w!("WM/ContentGroupDescription");
pub const g_wszWMCopyright: windows_core::PCWSTR = windows_core::w!("Copyright");
pub const g_wszWMCopyrightURL: windows_core::PCWSTR = windows_core::w!("CopyrightURL");
pub const g_wszWMCurrentBitrate: windows_core::PCWSTR = windows_core::w!("CurrentBitrate");
pub const g_wszWMDRM: windows_core::PCWSTR = windows_core::w!("WM/DRM");
pub const g_wszWMDRM_ContentID: windows_core::PCWSTR = windows_core::w!("DRM_ContentID");
pub const g_wszWMDRM_Flags: windows_core::PCWSTR = windows_core::w!("DRM_Flags");
pub const g_wszWMDRM_HeaderSignPrivKey: windows_core::PCWSTR = windows_core::w!("DRM_HeaderSignPrivKey");
pub const g_wszWMDRM_IndividualizedVersion: windows_core::PCWSTR = windows_core::w!("DRM_IndividualizedVersion");
pub const g_wszWMDRM_KeyID: windows_core::PCWSTR = windows_core::w!("DRM_KeyID");
pub const g_wszWMDRM_KeySeed: windows_core::PCWSTR = windows_core::w!("DRM_KeySeed");
pub const g_wszWMDRM_LASignatureCert: windows_core::PCWSTR = windows_core::w!("DRM_LASignatureCert");
pub const g_wszWMDRM_LASignatureLicSrvCert: windows_core::PCWSTR = windows_core::w!("DRM_LASignatureLicSrvCert");
pub const g_wszWMDRM_LASignaturePrivKey: windows_core::PCWSTR = windows_core::w!("DRM_LASignaturePrivKey");
pub const g_wszWMDRM_LASignatureRootCert: windows_core::PCWSTR = windows_core::w!("DRM_LASignatureRootCert");
pub const g_wszWMDRM_Level: windows_core::PCWSTR = windows_core::w!("DRM_Level");
pub const g_wszWMDRM_LicenseAcqURL: windows_core::PCWSTR = windows_core::w!("DRM_LicenseAcqURL");
pub const g_wszWMDRM_SourceID: windows_core::PCWSTR = windows_core::w!("DRM_SourceID");
pub const g_wszWMDRM_V1LicenseAcqURL: windows_core::PCWSTR = windows_core::w!("DRM_V1LicenseAcqURL");
pub const g_wszWMDVDID: windows_core::PCWSTR = windows_core::w!("WM/DVDID");
pub const g_wszWMDescription: windows_core::PCWSTR = windows_core::w!("Description");
pub const g_wszWMDirector: windows_core::PCWSTR = windows_core::w!("WM/Director");
pub const g_wszWMDuration: windows_core::PCWSTR = windows_core::w!("Duration");
pub const g_wszWMEncodedBy: windows_core::PCWSTR = windows_core::w!("WM/EncodedBy");
pub const g_wszWMEncodingSettings: windows_core::PCWSTR = windows_core::w!("WM/EncodingSettings");
pub const g_wszWMEncodingTime: windows_core::PCWSTR = windows_core::w!("WM/EncodingTime");
pub const g_wszWMEpisodeNumber: windows_core::PCWSTR = windows_core::w!("WM/EpisodeNumber");
pub const g_wszWMFileSize: windows_core::PCWSTR = windows_core::w!("FileSize");
pub const g_wszWMGenre: windows_core::PCWSTR = windows_core::w!("WM/Genre");
pub const g_wszWMGenreID: windows_core::PCWSTR = windows_core::w!("WM/GenreID");
pub const g_wszWMHasArbitraryDataStream: windows_core::PCWSTR = windows_core::w!("HasArbitraryDataStream");
pub const g_wszWMHasAttachedImages: windows_core::PCWSTR = windows_core::w!("HasAttachedImages");
pub const g_wszWMHasAudio: windows_core::PCWSTR = windows_core::w!("HasAudio");
pub const g_wszWMHasFileTransferStream: windows_core::PCWSTR = windows_core::w!("HasFileTransferStream");
pub const g_wszWMHasImage: windows_core::PCWSTR = windows_core::w!("HasImage");
pub const g_wszWMHasScript: windows_core::PCWSTR = windows_core::w!("HasScript");
pub const g_wszWMHasVideo: windows_core::PCWSTR = windows_core::w!("HasVideo");
pub const g_wszWMISAN: windows_core::PCWSTR = windows_core::w!("WM/ISAN");
pub const g_wszWMISRC: windows_core::PCWSTR = windows_core::w!("WM/ISRC");
pub const g_wszWMInitialKey: windows_core::PCWSTR = windows_core::w!("WM/InitialKey");
pub const g_wszWMIsCompilation: windows_core::PCWSTR = windows_core::w!("WM/IsCompilation");
pub const g_wszWMIsVBR: windows_core::PCWSTR = windows_core::w!("IsVBR");
pub const g_wszWMLanguage: windows_core::PCWSTR = windows_core::w!("WM/Language");
pub const g_wszWMLyrics: windows_core::PCWSTR = windows_core::w!("WM/Lyrics");
pub const g_wszWMLyrics_Synchronised: windows_core::PCWSTR = windows_core::w!("WM/Lyrics_Synchronised");
pub const g_wszWMMCDI: windows_core::PCWSTR = windows_core::w!("WM/MCDI");
pub const g_wszWMMediaClassPrimaryID: windows_core::PCWSTR = windows_core::w!("WM/MediaClassPrimaryID");
pub const g_wszWMMediaClassSecondaryID: windows_core::PCWSTR = windows_core::w!("WM/MediaClassSecondaryID");
pub const g_wszWMMediaCredits: windows_core::PCWSTR = windows_core::w!("WM/MediaCredits");
pub const g_wszWMMediaIsDelay: windows_core::PCWSTR = windows_core::w!("WM/MediaIsDelay");
pub const g_wszWMMediaIsFinale: windows_core::PCWSTR = windows_core::w!("WM/MediaIsFinale");
pub const g_wszWMMediaIsLive: windows_core::PCWSTR = windows_core::w!("WM/MediaIsLive");
pub const g_wszWMMediaIsPremiere: windows_core::PCWSTR = windows_core::w!("WM/MediaIsPremiere");
pub const g_wszWMMediaIsRepeat: windows_core::PCWSTR = windows_core::w!("WM/MediaIsRepeat");
pub const g_wszWMMediaIsSAP: windows_core::PCWSTR = windows_core::w!("WM/MediaIsSAP");
pub const g_wszWMMediaIsStereo: windows_core::PCWSTR = windows_core::w!("WM/MediaIsStereo");
pub const g_wszWMMediaIsSubtitled: windows_core::PCWSTR = windows_core::w!("WM/MediaIsSubtitled");
pub const g_wszWMMediaIsTape: windows_core::PCWSTR = windows_core::w!("WM/MediaIsTape");
pub const g_wszWMMediaNetworkAffiliation: windows_core::PCWSTR = windows_core::w!("WM/MediaNetworkAffiliation");
pub const g_wszWMMediaOriginalBroadcastDateTime: windows_core::PCWSTR = windows_core::w!("WM/MediaOriginalBroadcastDateTime");
pub const g_wszWMMediaOriginalChannel: windows_core::PCWSTR = windows_core::w!("WM/MediaOriginalChannel");
pub const g_wszWMMediaStationCallSign: windows_core::PCWSTR = windows_core::w!("WM/MediaStationCallSign");
pub const g_wszWMMediaStationName: windows_core::PCWSTR = windows_core::w!("WM/MediaStationName");
pub const g_wszWMModifiedBy: windows_core::PCWSTR = windows_core::w!("WM/ModifiedBy");
pub const g_wszWMMood: windows_core::PCWSTR = windows_core::w!("WM/Mood");
pub const g_wszWMNSCAddress: windows_core::PCWSTR = windows_core::w!("NSC_Address");
pub const g_wszWMNSCDescription: windows_core::PCWSTR = windows_core::w!("NSC_Description");
pub const g_wszWMNSCEmail: windows_core::PCWSTR = windows_core::w!("NSC_Email");
pub const g_wszWMNSCName: windows_core::PCWSTR = windows_core::w!("NSC_Name");
pub const g_wszWMNSCPhone: windows_core::PCWSTR = windows_core::w!("NSC_Phone");
pub const g_wszWMNumberOfFrames: windows_core::PCWSTR = windows_core::w!("NumberOfFrames");
pub const g_wszWMOptimalBitrate: windows_core::PCWSTR = windows_core::w!("OptimalBitrate");
pub const g_wszWMOriginalAlbumTitle: windows_core::PCWSTR = windows_core::w!("WM/OriginalAlbumTitle");
pub const g_wszWMOriginalArtist: windows_core::PCWSTR = windows_core::w!("WM/OriginalArtist");
pub const g_wszWMOriginalFilename: windows_core::PCWSTR = windows_core::w!("WM/OriginalFilename");
pub const g_wszWMOriginalLyricist: windows_core::PCWSTR = windows_core::w!("WM/OriginalLyricist");
pub const g_wszWMOriginalReleaseTime: windows_core::PCWSTR = windows_core::w!("WM/OriginalReleaseTime");
pub const g_wszWMOriginalReleaseYear: windows_core::PCWSTR = windows_core::w!("WM/OriginalReleaseYear");
pub const g_wszWMParentalRating: windows_core::PCWSTR = windows_core::w!("WM/ParentalRating");
pub const g_wszWMParentalRatingReason: windows_core::PCWSTR = windows_core::w!("WM/ParentalRatingReason");
pub const g_wszWMPartOfSet: windows_core::PCWSTR = windows_core::w!("WM/PartOfSet");
pub const g_wszWMPeakBitrate: windows_core::PCWSTR = windows_core::w!("WM/PeakBitrate");
pub const g_wszWMPeriod: windows_core::PCWSTR = windows_core::w!("WM/Period");
pub const g_wszWMPicture: windows_core::PCWSTR = windows_core::w!("WM/Picture");
pub const g_wszWMPlaylistDelay: windows_core::PCWSTR = windows_core::w!("WM/PlaylistDelay");
pub const g_wszWMProducer: windows_core::PCWSTR = windows_core::w!("WM/Producer");
pub const g_wszWMPromotionURL: windows_core::PCWSTR = windows_core::w!("WM/PromotionURL");
pub const g_wszWMProtected: windows_core::PCWSTR = windows_core::w!("Is_Protected");
pub const g_wszWMProtectionType: windows_core::PCWSTR = windows_core::w!("WM/ProtectionType");
pub const g_wszWMProvider: windows_core::PCWSTR = windows_core::w!("WM/Provider");
pub const g_wszWMProviderCopyright: windows_core::PCWSTR = windows_core::w!("WM/ProviderCopyright");
pub const g_wszWMProviderRating: windows_core::PCWSTR = windows_core::w!("WM/ProviderRating");
pub const g_wszWMProviderStyle: windows_core::PCWSTR = windows_core::w!("WM/ProviderStyle");
pub const g_wszWMPublisher: windows_core::PCWSTR = windows_core::w!("WM/Publisher");
pub const g_wszWMRadioStationName: windows_core::PCWSTR = windows_core::w!("WM/RadioStationName");
pub const g_wszWMRadioStationOwner: windows_core::PCWSTR = windows_core::w!("WM/RadioStationOwner");
pub const g_wszWMRating: windows_core::PCWSTR = windows_core::w!("Rating");
pub const g_wszWMSeasonNumber: windows_core::PCWSTR = windows_core::w!("WM/SeasonNumber");
pub const g_wszWMSeekable: windows_core::PCWSTR = windows_core::w!("Seekable");
pub const g_wszWMSharedUserRating: windows_core::PCWSTR = windows_core::w!("WM/SharedUserRating");
pub const g_wszWMSignature_Name: windows_core::PCWSTR = windows_core::w!("Signature_Name");
pub const g_wszWMSkipBackward: windows_core::PCWSTR = windows_core::w!("Can_Skip_Backward");
pub const g_wszWMSkipForward: windows_core::PCWSTR = windows_core::w!("Can_Skip_Forward");
pub const g_wszWMStreamTypeInfo: windows_core::PCWSTR = windows_core::w!("WM/StreamTypeInfo");
pub const g_wszWMStridable: windows_core::PCWSTR = windows_core::w!("Stridable");
pub const g_wszWMSubTitle: windows_core::PCWSTR = windows_core::w!("WM/SubTitle");
pub const g_wszWMSubTitleDescription: windows_core::PCWSTR = windows_core::w!("WM/SubTitleDescription");
pub const g_wszWMSubscriptionContentID: windows_core::PCWSTR = windows_core::w!("WM/SubscriptionContentID");
pub const g_wszWMText: windows_core::PCWSTR = windows_core::w!("WM/Text");
pub const g_wszWMTitle: windows_core::PCWSTR = windows_core::w!("Title");
pub const g_wszWMTitleSort: windows_core::PCWSTR = windows_core::w!("TitleSort");
pub const g_wszWMToolName: windows_core::PCWSTR = windows_core::w!("WM/ToolName");
pub const g_wszWMToolVersion: windows_core::PCWSTR = windows_core::w!("WM/ToolVersion");
pub const g_wszWMTrack: windows_core::PCWSTR = windows_core::w!("WM/Track");
pub const g_wszWMTrackNumber: windows_core::PCWSTR = windows_core::w!("WM/TrackNumber");
pub const g_wszWMTrusted: windows_core::PCWSTR = windows_core::w!("Is_Trusted");
pub const g_wszWMUniqueFileIdentifier: windows_core::PCWSTR = windows_core::w!("WM/UniqueFileIdentifier");
pub const g_wszWMUse_Advanced_DRM: windows_core::PCWSTR = windows_core::w!("Use_Advanced_DRM");
pub const g_wszWMUse_DRM: windows_core::PCWSTR = windows_core::w!("Use_DRM");
pub const g_wszWMUserWebURL: windows_core::PCWSTR = windows_core::w!("WM/UserWebURL");
pub const g_wszWMVideoClosedCaptioning: windows_core::PCWSTR = windows_core::w!("WM/VideoClosedCaptioning");
pub const g_wszWMVideoFrameRate: windows_core::PCWSTR = windows_core::w!("WM/VideoFrameRate");
pub const g_wszWMVideoHeight: windows_core::PCWSTR = windows_core::w!("WM/VideoHeight");
pub const g_wszWMVideoWidth: windows_core::PCWSTR = windows_core::w!("WM/VideoWidth");
pub const g_wszWMWMADRCAverageReference: windows_core::PCWSTR = windows_core::w!("WM/WMADRCAverageReference");
pub const g_wszWMWMADRCAverageTarget: windows_core::PCWSTR = windows_core::w!("WM/WMADRCAverageTarget");
pub const g_wszWMWMADRCPeakReference: windows_core::PCWSTR = windows_core::w!("WM/WMADRCPeakReference");
pub const g_wszWMWMADRCPeakTarget: windows_core::PCWSTR = windows_core::w!("WM/WMADRCPeakTarget");
pub const g_wszWMWMCPDistributor: windows_core::PCWSTR = windows_core::w!("WM/WMCPDistributor");
pub const g_wszWMWMCPDistributorID: windows_core::PCWSTR = windows_core::w!("WM/WMCPDistributorID");
pub const g_wszWMWMCollectionGroupID: windows_core::PCWSTR = windows_core::w!("WM/WMCollectionGroupID");
pub const g_wszWMWMCollectionID: windows_core::PCWSTR = windows_core::w!("WM/WMCollectionID");
pub const g_wszWMWMContentID: windows_core::PCWSTR = windows_core::w!("WM/WMContentID");
pub const g_wszWMWMShadowFileSourceDRMType: windows_core::PCWSTR = windows_core::w!("WM/WMShadowFileSourceDRMType");
pub const g_wszWMWMShadowFileSourceFileType: windows_core::PCWSTR = windows_core::w!("WM/WMShadowFileSourceFileType");
pub const g_wszWMWriter: windows_core::PCWSTR = windows_core::w!("WM/Writer");
pub const g_wszWMYear: windows_core::PCWSTR = windows_core::w!("WM/Year");
pub const g_wszWatermarkCLSID: windows_core::PCWSTR = windows_core::w!("WatermarkCLSID");
pub const g_wszWatermarkConfig: windows_core::PCWSTR = windows_core::w!("WatermarkConfig");
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETSOURCE_URLCREDPOLICY_SETTINGS(pub i32);
impl windows_core::TypeKind for NETSOURCE_URLCREDPOLICY_SETTINGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETSOURCE_URLCREDPOLICY_SETTINGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETSOURCE_URLCREDPOLICY_SETTINGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WEBSTREAM_SAMPLE_TYPE(pub i32);
impl windows_core::TypeKind for WEBSTREAM_SAMPLE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WEBSTREAM_SAMPLE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WEBSTREAM_SAMPLE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_ATTR_DATATYPE(pub i32);
impl windows_core::TypeKind for WMT_ATTR_DATATYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_ATTR_DATATYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_ATTR_DATATYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_ATTR_IMAGETYPE(pub i32);
impl windows_core::TypeKind for WMT_ATTR_IMAGETYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_ATTR_IMAGETYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_ATTR_IMAGETYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_CODEC_INFO_TYPE(pub i32);
impl windows_core::TypeKind for WMT_CODEC_INFO_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_CODEC_INFO_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_CODEC_INFO_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_CREDENTIAL_FLAGS(pub i32);
impl windows_core::TypeKind for WMT_CREDENTIAL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_CREDENTIAL_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_CREDENTIAL_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_DRMLA_TRUST(pub i32);
impl windows_core::TypeKind for WMT_DRMLA_TRUST {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_DRMLA_TRUST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_DRMLA_TRUST").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_FILESINK_MODE(pub i32);
impl windows_core::TypeKind for WMT_FILESINK_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_FILESINK_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_FILESINK_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_IMAGE_TYPE(pub i32);
impl windows_core::TypeKind for WMT_IMAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_IMAGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_IMAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_INDEXER_TYPE(pub i32);
impl windows_core::TypeKind for WMT_INDEXER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_INDEXER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_INDEXER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_INDEX_TYPE(pub i32);
impl windows_core::TypeKind for WMT_INDEX_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_INDEX_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_INDEX_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_MUSICSPEECH_CLASS_MODE(pub i32);
impl windows_core::TypeKind for WMT_MUSICSPEECH_CLASS_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_MUSICSPEECH_CLASS_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_MUSICSPEECH_CLASS_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_NET_PROTOCOL(pub i32);
impl windows_core::TypeKind for WMT_NET_PROTOCOL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_NET_PROTOCOL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_NET_PROTOCOL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_OFFSET_FORMAT(pub i32);
impl windows_core::TypeKind for WMT_OFFSET_FORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_OFFSET_FORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_OFFSET_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_PLAY_MODE(pub i32);
impl windows_core::TypeKind for WMT_PLAY_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_PLAY_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_PLAY_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_PROXY_SETTINGS(pub i32);
impl windows_core::TypeKind for WMT_PROXY_SETTINGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_PROXY_SETTINGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_PROXY_SETTINGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_RIGHTS(pub i32);
impl windows_core::TypeKind for WMT_RIGHTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_RIGHTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_RIGHTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_STATUS(pub i32);
impl windows_core::TypeKind for WMT_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_STORAGE_FORMAT(pub i32);
impl windows_core::TypeKind for WMT_STORAGE_FORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_STORAGE_FORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_STORAGE_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_STREAM_SELECTION(pub i32);
impl windows_core::TypeKind for WMT_STREAM_SELECTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_STREAM_SELECTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_STREAM_SELECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_TIMECODE_FRAMERATE(pub i32);
impl windows_core::TypeKind for WMT_TIMECODE_FRAMERATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_TIMECODE_FRAMERATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_TIMECODE_FRAMERATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_TRANSPORT_TYPE(pub i32);
impl windows_core::TypeKind for WMT_TRANSPORT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_TRANSPORT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_TRANSPORT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_VERSION(pub i32);
impl windows_core::TypeKind for WMT_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMT_WATERMARK_ENTRY_TYPE(pub i32);
impl windows_core::TypeKind for WMT_WATERMARK_ENTRY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMT_WATERMARK_ENTRY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMT_WATERMARK_ENTRY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WM_AETYPE(pub i32);
impl windows_core::TypeKind for WM_AETYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WM_AETYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WM_AETYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WM_DM_INTERLACED_TYPE(pub i32);
impl windows_core::TypeKind for WM_DM_INTERLACED_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WM_DM_INTERLACED_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WM_DM_INTERLACED_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WM_DM_IT_FIRST_FRAME_COHERENCY(pub i32);
impl windows_core::TypeKind for WM_DM_IT_FIRST_FRAME_COHERENCY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WM_DM_IT_FIRST_FRAME_COHERENCY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WM_DM_IT_FIRST_FRAME_COHERENCY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WM_PLAYBACK_DRC_LEVEL(pub i32);
impl windows_core::TypeKind for WM_PLAYBACK_DRC_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WM_PLAYBACK_DRC_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WM_PLAYBACK_DRC_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WM_SFEX_TYPE(pub i32);
impl windows_core::TypeKind for WM_SFEX_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WM_SFEX_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WM_SFEX_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WM_SF_TYPE(pub i32);
impl windows_core::TypeKind for WM_SF_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WM_SF_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WM_SF_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _AM_ASFWRITERCONFIG_PARAM(pub i32);
impl windows_core::TypeKind for _AM_ASFWRITERCONFIG_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _AM_ASFWRITERCONFIG_PARAM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_AM_ASFWRITERCONFIG_PARAM").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AM_WMT_EVENT_DATA {
    pub hrStatus: windows_core::HRESULT,
    pub pData: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for AM_WMT_EVENT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for AM_WMT_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRM_COPY_OPL {
    pub wMinimumCopyLevel: u16,
    pub oplIdIncludes: DRM_OPL_OUTPUT_IDS,
    pub oplIdExcludes: DRM_OPL_OUTPUT_IDS,
}
impl windows_core::TypeKind for DRM_COPY_OPL {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRM_COPY_OPL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    pub wCompressedDigitalVideo: u16,
    pub wUncompressedDigitalVideo: u16,
    pub wAnalogVideo: u16,
    pub wCompressedDigitalAudio: u16,
    pub wUncompressedDigitalAudio: u16,
}
impl windows_core::TypeKind for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRM_OPL_OUTPUT_IDS {
    pub cIds: u16,
    pub rgIds: *mut windows_core::GUID,
}
impl windows_core::TypeKind for DRM_OPL_OUTPUT_IDS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRM_OPL_OUTPUT_IDS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRM_OUTPUT_PROTECTION {
    pub guidId: windows_core::GUID,
    pub bConfigData: u8,
}
impl windows_core::TypeKind for DRM_OUTPUT_PROTECTION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRM_OUTPUT_PROTECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRM_PLAY_OPL {
    pub minOPL: DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS,
    pub oplIdReserved: DRM_OPL_OUTPUT_IDS,
    pub vopi: DRM_VIDEO_OUTPUT_PROTECTION_IDS,
}
impl windows_core::TypeKind for DRM_PLAY_OPL {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRM_PLAY_OPL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRM_VAL16 {
    pub val: [u8; 16],
}
impl windows_core::TypeKind for DRM_VAL16 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRM_VAL16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    pub cEntries: u16,
    pub rgVop: *mut DRM_OUTPUT_PROTECTION,
}
impl windows_core::TypeKind for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMDRM_IMPORT_INIT_STRUCT {
    pub dwVersion: u32,
    pub cbEncryptedSessionKeyMessage: u32,
    pub pbEncryptedSessionKeyMessage: *mut u8,
    pub cbEncryptedKeyMessage: u32,
    pub pbEncryptedKeyMessage: *mut u8,
}
impl windows_core::TypeKind for WMDRM_IMPORT_INIT_STRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMDRM_IMPORT_INIT_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMMPEG2VIDEOINFO {
    pub hdr: WMVIDEOINFOHEADER2,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub dwProfile: u32,
    pub dwLevel: u32,
    pub dwFlags: u32,
    pub dwSequenceHeader: [u32; 1],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for WMMPEG2VIDEOINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for WMMPEG2VIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMSCRIPTFORMAT {
    pub scriptType: windows_core::GUID,
}
impl windows_core::TypeKind for WMSCRIPTFORMAT {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMSCRIPTFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct WMT_BUFFER_SEGMENT {
    pub pBuffer: core::mem::ManuallyDrop<Option<INSSBuffer>>,
    pub cbOffset: u32,
    pub cbLength: u32,
}
impl Clone for WMT_BUFFER_SEGMENT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for WMT_BUFFER_SEGMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMT_BUFFER_SEGMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMT_COLORSPACEINFO_EXTENSION_DATA {
    pub ucColorPrimaries: u8,
    pub ucColorTransferChar: u8,
    pub ucColorMatrixCoef: u8,
}
impl windows_core::TypeKind for WMT_COLORSPACEINFO_EXTENSION_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct WMT_FILESINK_DATA_UNIT {
    pub packetHeaderBuffer: WMT_BUFFER_SEGMENT,
    pub cPayloads: u32,
    pub pPayloadHeaderBuffers: *mut WMT_BUFFER_SEGMENT,
    pub cPayloadDataFragments: u32,
    pub pPayloadDataFragments: *mut WMT_PAYLOAD_FRAGMENT,
}
impl Clone for WMT_FILESINK_DATA_UNIT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for WMT_FILESINK_DATA_UNIT {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMT_FILESINK_DATA_UNIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct WMT_PAYLOAD_FRAGMENT {
    pub dwPayloadIndex: u32,
    pub segmentData: WMT_BUFFER_SEGMENT,
}
impl Clone for WMT_PAYLOAD_FRAGMENT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for WMT_PAYLOAD_FRAGMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMT_PAYLOAD_FRAGMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct WMT_TIMECODE_EXTENSION_DATA {
    pub wRange: u16,
    pub dwTimecode: u32,
    pub dwUserbits: u32,
    pub dwAmFlags: u32,
}
impl windows_core::TypeKind for WMT_TIMECODE_EXTENSION_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMT_TIMECODE_EXTENSION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMT_VIDEOIMAGE_SAMPLE {
    pub dwMagic: u32,
    pub cbStruct: u32,
    pub dwControlFlags: u32,
    pub dwInputFlagsCur: u32,
    pub lCurMotionXtoX: i32,
    pub lCurMotionYtoX: i32,
    pub lCurMotionXoffset: i32,
    pub lCurMotionXtoY: i32,
    pub lCurMotionYtoY: i32,
    pub lCurMotionYoffset: i32,
    pub lCurBlendCoef1: i32,
    pub lCurBlendCoef2: i32,
    pub dwInputFlagsPrev: u32,
    pub lPrevMotionXtoX: i32,
    pub lPrevMotionYtoX: i32,
    pub lPrevMotionXoffset: i32,
    pub lPrevMotionXtoY: i32,
    pub lPrevMotionYtoY: i32,
    pub lPrevMotionYoffset: i32,
    pub lPrevBlendCoef1: i32,
    pub lPrevBlendCoef2: i32,
}
impl windows_core::TypeKind for WMT_VIDEOIMAGE_SAMPLE {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMT_VIDEOIMAGE_SAMPLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WMT_VIDEOIMAGE_SAMPLE2 {
    pub dwMagic: u32,
    pub dwStructSize: u32,
    pub dwControlFlags: u32,
    pub dwViewportWidth: u32,
    pub dwViewportHeight: u32,
    pub dwCurrImageWidth: u32,
    pub dwCurrImageHeight: u32,
    pub fCurrRegionX0: f32,
    pub fCurrRegionY0: f32,
    pub fCurrRegionWidth: f32,
    pub fCurrRegionHeight: f32,
    pub fCurrBlendCoef: f32,
    pub dwPrevImageWidth: u32,
    pub dwPrevImageHeight: u32,
    pub fPrevRegionX0: f32,
    pub fPrevRegionY0: f32,
    pub fPrevRegionWidth: f32,
    pub fPrevRegionHeight: f32,
    pub fPrevBlendCoef: f32,
    pub dwEffectType: u32,
    pub dwNumEffectParas: u32,
    pub fEffectPara0: f32,
    pub fEffectPara1: f32,
    pub fEffectPara2: f32,
    pub fEffectPara3: f32,
    pub fEffectPara4: f32,
    pub bKeepPrevImage: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WMT_VIDEOIMAGE_SAMPLE2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMT_VIDEOIMAGE_SAMPLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMT_WATERMARK_ENTRY {
    pub wmetType: WMT_WATERMARK_ENTRY_TYPE,
    pub clsid: windows_core::GUID,
    pub cbDisplayName: u32,
    pub pwszDisplayName: windows_core::PWSTR,
}
impl windows_core::TypeKind for WMT_WATERMARK_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMT_WATERMARK_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMT_WEBSTREAM_FORMAT {
    pub cbSize: u16,
    pub cbSampleHeaderFixedData: u16,
    pub wVersion: u16,
    pub wReserved: u16,
}
impl windows_core::TypeKind for WMT_WEBSTREAM_FORMAT {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMT_WEBSTREAM_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMT_WEBSTREAM_SAMPLE_HEADER {
    pub cbLength: u16,
    pub wPart: u16,
    pub cTotalParts: u16,
    pub wSampleType: u16,
    pub wszURL: [u16; 1],
}
impl windows_core::TypeKind for WMT_WEBSTREAM_SAMPLE_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMVIDEOINFOHEADER {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: super::super::Graphics::Gdi::BITMAPINFOHEADER,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for WMVIDEOINFOHEADER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for WMVIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMVIDEOINFOHEADER2 {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub dwInterlaceFlags: u32,
    pub dwCopyProtectFlags: u32,
    pub dwPictAspectRatioX: u32,
    pub dwPictAspectRatioY: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub bmiHeader: super::super::Graphics::Gdi::BITMAPINFOHEADER,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for WMVIDEOINFOHEADER2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for WMVIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WM_ADDRESS_ACCESSENTRY {
    pub dwIPAddress: u32,
    pub dwMask: u32,
}
impl windows_core::TypeKind for WM_ADDRESS_ACCESSENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for WM_ADDRESS_ACCESSENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WM_CLIENT_PROPERTIES {
    pub dwIPAddress: u32,
    pub dwPort: u32,
}
impl windows_core::TypeKind for WM_CLIENT_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for WM_CLIENT_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WM_CLIENT_PROPERTIES_EX {
    pub cbSize: u32,
    pub pwszIPAddress: windows_core::PCWSTR,
    pub pwszPort: windows_core::PCWSTR,
    pub pwszDNSName: windows_core::PCWSTR,
}
impl windows_core::TypeKind for WM_CLIENT_PROPERTIES_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for WM_CLIENT_PROPERTIES_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WM_LEAKY_BUCKET_PAIR {
    pub dwBitrate: u32,
    pub msBufferWindow: u32,
}
impl windows_core::TypeKind for WM_LEAKY_BUCKET_PAIR {
    type TypeKind = windows_core::CopyType;
}
impl Default for WM_LEAKY_BUCKET_PAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct WM_MEDIA_TYPE {
    pub majortype: windows_core::GUID,
    pub subtype: windows_core::GUID,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub lSampleSize: u32,
    pub formattype: windows_core::GUID,
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
impl Clone for WM_MEDIA_TYPE {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for WM_MEDIA_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl Default for WM_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WM_PICTURE {
    pub pwszMIMEType: windows_core::PWSTR,
    pub bPictureType: u8,
    pub pwszDescription: windows_core::PWSTR,
    pub dwDataLen: u32,
    pub pbData: *mut u8,
}
impl windows_core::TypeKind for WM_PICTURE {
    type TypeKind = windows_core::CopyType;
}
impl Default for WM_PICTURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WM_PORT_NUMBER_RANGE {
    pub wPortBegin: u16,
    pub wPortEnd: u16,
}
impl windows_core::TypeKind for WM_PORT_NUMBER_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for WM_PORT_NUMBER_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WM_READER_CLIENTINFO {
    pub cbSize: u32,
    pub wszLang: windows_core::PWSTR,
    pub wszBrowserUserAgent: windows_core::PWSTR,
    pub wszBrowserWebPage: windows_core::PWSTR,
    pub qwReserved: u64,
    pub pReserved: *mut super::super::Foundation::LPARAM,
    pub wszHostExe: windows_core::PWSTR,
    pub qwHostVersion: u64,
    pub wszPlayerUserAgent: windows_core::PWSTR,
}
impl windows_core::TypeKind for WM_READER_CLIENTINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for WM_READER_CLIENTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WM_READER_STATISTICS {
    pub cbSize: u32,
    pub dwBandwidth: u32,
    pub cPacketsReceived: u32,
    pub cPacketsRecovered: u32,
    pub cPacketsLost: u32,
    pub wQuality: u16,
}
impl windows_core::TypeKind for WM_READER_STATISTICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for WM_READER_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct WM_STREAM_PRIORITY_RECORD {
    pub wStreamNumber: u16,
    pub fMandatory: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WM_STREAM_PRIORITY_RECORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for WM_STREAM_PRIORITY_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WM_STREAM_TYPE_INFO {
    pub guidMajorType: windows_core::GUID,
    pub cbFormat: u32,
}
impl windows_core::TypeKind for WM_STREAM_TYPE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for WM_STREAM_TYPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WM_SYNCHRONISED_LYRICS {
    pub bTimeStampFormat: u8,
    pub bContentType: u8,
    pub pwszContentDescriptor: windows_core::PWSTR,
    pub dwLyricsLen: u32,
    pub pbLyrics: *mut u8,
}
impl windows_core::TypeKind for WM_SYNCHRONISED_LYRICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for WM_SYNCHRONISED_LYRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WM_USER_TEXT {
    pub pwszDescription: windows_core::PWSTR,
    pub pwszText: windows_core::PWSTR,
}
impl windows_core::TypeKind for WM_USER_TEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for WM_USER_TEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WM_USER_WEB_URL {
    pub pwszDescription: windows_core::PWSTR,
    pub pwszURL: windows_core::PWSTR,
}
impl windows_core::TypeKind for WM_USER_WEB_URL {
    type TypeKind = windows_core::CopyType;
}
impl Default for WM_USER_WEB_URL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WM_WRITER_STATISTICS {
    pub qwSampleCount: u64,
    pub qwByteCount: u64,
    pub qwDroppedSampleCount: u64,
    pub qwDroppedByteCount: u64,
    pub dwCurrentBitrate: u32,
    pub dwAverageBitrate: u32,
    pub dwExpectedBitrate: u32,
    pub dwCurrentSampleRate: u32,
    pub dwAverageSampleRate: u32,
    pub dwExpectedSampleRate: u32,
}
impl windows_core::TypeKind for WM_WRITER_STATISTICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for WM_WRITER_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WM_WRITER_STATISTICS_EX {
    pub dwBitratePlusOverhead: u32,
    pub dwCurrentSampleDropRateInQueue: u32,
    pub dwCurrentSampleDropRateInCodec: u32,
    pub dwCurrentSampleDropRateInMultiplexer: u32,
    pub dwTotalSampleDropsInQueue: u32,
    pub dwTotalSampleDropsInCodec: u32,
    pub dwTotalSampleDropsInMultiplexer: u32,
}
impl windows_core::TypeKind for WM_WRITER_STATISTICS_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for WM_WRITER_STATISTICS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
